use proc_macro2::TokenStream;
use quote::quote;

use crate::tests::rejects;
use crate::{compile, parse};

fn grammar(interface: &TokenStream, block: &TokenStream) -> TokenStream {
    quote! { mod g {
        feature Flag { No, Yes } default No;
        category A(#interface);
        construction Atom: A { form [head: lexical(Noun)]; export Flag = Yes; }
        schema Pair { head none; form [left: node, " + ", right: node]; }
        schema End { head none; form [left: node, ", + ", right: node]; }
        schema Continue { head none; form [left: node, ", ", rest: node]; }
        schema Complete { head none; form [left: node, ", ", rest: node]; }
        coordination_forms Standard { binary Pair; series End, Continue, Complete; }
        coordinable A using Standard { #block }
    } }
}

fn normalized(input: TokenStream) -> parse::Declaration {
    let d = syn::parse2(input.clone()).unwrap();
    compile(input).unwrap();
    d
}

#[test]
fn combination_is_independent_of_head_inheritance_and_local_defaults() {
    for mode in [quote!(head), quote!(local)] {
        let input = grammar(&quote!(#mode Flag combine same), &quote!(forms binary;));
        let d = normalized(input);
        let pair = d.constructions.iter().find(|c| c.name == "Pair").unwrap();
        assert_eq!(pair.equations.len(), 2);
        let parse::Equation::Agree(left, right) = &pair.equations[0] else {
            panic!("same must check both members")
        };
        assert_eq!(
            (left.field.to_string(), right.field.to_string()),
            ("left".into(), "right".into())
        );
        let atom_export = if mode.to_string() == "head" {
            quote!(export Flag = head.Flag;)
        } else {
            quote!(export Flag = No;)
        };
        let inferred = quote! { mod g {
            feature Flag { No, Yes } default No;
            category A(#mode Flag combine same);
            construction Atom: A { form [head: lexical(Noun)]; }
            schema Pair { head none; form [left: node, " + ", right: node]; }
            coordination_forms P { binary Pair; }
            coordinable A using P { forms binary; }
        } };
        let explicit = quote! { mod g {
            feature Flag { No, Yes } default No;
            category A(Flag);
            construction Atom: A { form [head: lexical(Noun)]; #atom_export }
            schema Pair { head none; form [left: node, " + ", right: node]; }
            instance Pair: A {
                bind left, right = A;
                agree left.Flag = right.Flag;
                export Flag = left.Flag;
            }
        } };
        assert_eq!(
            compile(inferred).unwrap().to_string(),
            compile(explicit).unwrap().to_string()
        );
    }
}

#[test]
fn series_overrides_and_asymmetric_requirements_do_not_restate_regular_features() {
    let d = normalized(grammar(
        &quote!(head Flag combine same),
        &quote! {
            forms binary, serial; series Tail;
            on series { combine Flag = No; require right.Flag = Yes; }
            on Complete { require left.Flag = Yes; }
        },
    ));
    for name in ["End", "Continue"] {
        let c = d.constructions.iter().find(|c| c.name == name).unwrap();
        assert_eq!(c.category.to_string(), "Tail");
        let parse::Equation::ExportConstant(feature, value) = &c.equations[0] else {
            panic!("series override")
        };
        assert_eq!(
            (feature.to_string(), value.to_string()),
            ("Flag".into(), "No".into())
        );
        let parse::Equation::Require(r, value) = &c.equations[1] else {
            panic!("right-only requirement")
        };
        assert_eq!(
            (r.field.to_string(), value.to_string()),
            (
                if name == "End" { "right" } else { "rest" }.into(),
                "Yes".into()
            )
        );
    }
    let complete = d
        .constructions
        .iter()
        .find(|c| c.name == "Complete")
        .unwrap();
    assert_eq!(complete.equations.len(), 3);
    let parse::Equation::Require(r, _) = &complete.equations[2] else {
        panic!("completion requirement")
    };
    assert_eq!(r.field.to_string(), "left");
}

#[test]
fn named_table_argument_order_and_coordinator_summary_survive_serial_elaboration() {
    let d = normalized(quote! { mod g {
        feature Flag { No, Yes }
        feature Kind { Add, Alternative }
        table choose(Kind, Flag, Flag) -> Flag { (Add, Yes, No) => Yes, (_, _, _) => No, }
        category A(Flag combine choose(source.Kind, right.Flag, left.Flag));
        construction Atom: A { form ["x"]; export Flag = Yes; }
        schema End { head none; form [left: node, " ", coordinator: lexical(Coordinator), " ", right: node]; }
        schema Continue { head none; form [left: node, ", ", rest: node]; }
        schema Complete { head none; form [left: node, ", ", rest: node]; }
        coordination_forms Standard { series End, Continue, Complete; }
        coordinable A using Standard { forms serial; series Tail; }
    } });
    let tail = d.categories.iter().find(|c| c.name == "Tail").unwrap();
    assert_eq!(
        tail.features
            .iter()
            .map(|f| f.name.to_string())
            .collect::<Vec<_>>(),
        vec!["Flag", "Kind"]
    );
    let c = d
        .constructions
        .iter()
        .find(|c| c.name == "Complete")
        .unwrap();
    let parse::Equation::ExportTable(_, table, refs) = &c.equations[0] else {
        panic!("named judgment")
    };
    assert_eq!(table.to_string(), "choose");
    assert_eq!(
        refs.iter()
            .map(|r| format!("{}.{}", r.field, r.feature))
            .collect::<Vec<_>>(),
        vec!["rest.Kind", "rest.Flag", "left.Flag"]
    );
}

#[test]
fn series_can_omit_a_completed_only_summary() {
    normalized(grammar(
        &quote!(head Flag combine Yes),
        &quote! {
            forms binary, serial; series Tail;
            on series { omit Flag; }
        },
    ));
}

#[test]
fn exact_template_override_wins_over_group_and_category_override() {
    let d = normalized(grammar(
        &quote!(head Flag combine same),
        &quote! {
            forms binary, serial; series Tail;
            combine Flag = Yes;
            on series { combine Flag = No; }
            on End { combine Flag = Yes; }
        },
    ));
    for (name, expected) in [
        ("Pair", "Yes"),
        ("End", "Yes"),
        ("Continue", "No"),
        ("Complete", "Yes"),
    ] {
        let c = d.constructions.iter().find(|c| c.name == name).unwrap();
        let parse::Equation::ExportConstant(_, value) = &c.equations[0] else {
            panic!("constant override")
        };
        assert_eq!(value.to_string(), expected);
    }
}

#[test]
fn invalid_combination_domains_and_references_reject() {
    rejects(
        grammar(&quote!(Flag), &quote!(forms binary;)),
        "requires a combiner",
    );
    rejects(
        grammar(
            &quote!(Flag combine missing(left.Flag)),
            &quote!(forms binary;),
        ),
        "unknown feature table",
    );
    rejects(
        grammar(&quote!(Flag combine left.Unknown), &quote!(forms binary;)),
        "Unknown",
    );
    rejects(
        grammar(&quote!(Flag combine bogus.Flag), &quote!(forms binary;)),
        "left, right or source",
    );
    rejects(
        quote! { mod g {
            category A(number combine all);
            construction Atom: A { form ["x"]; export number = Singular; }
            schema Pair { form [left: node, "+", right: node]; }
            coordination_forms P { binary Pair; }
            coordinable A using P { forms binary; }
        } },
        "Yes/No feature domain",
    );
}

#[test]
fn duplicate_overrides_unknown_contexts_and_invalid_omission_reject() {
    for (block, error) in [
        (
            quote!(forms binary; combine Flag = Yes; combine Flag = No;),
            "duplicate combiner override",
        ),
        (
            quote!(forms binary; on binary { combine Flag = Yes; } on complete { combine Flag = No; }),
            "same specificity",
        ),
        (
            quote!(forms binary; on Missing { require left.Flag = Yes; }),
            "matches no selected form",
        ),
        (
            quote!(forms binary; omit Flag;),
            "omit applies only to series",
        ),
        (
            quote!(forms serial; series Tail; on series { omit Missing; }),
            "cannot omit unknown",
        ),
        (quote!(forms binary, Pair;), "duplicate coordination form"),
        (quote!(forms serial;), "named series category"),
    ] {
        rejects(grammar(&quote!(Flag combine same), &block), error);
    }
}

#[test]
fn helper_name_collisions_and_duplicate_generated_instances_reject() {
    rejects(
        quote! { mod g {
            category A(); category Tail();
            schema End { form [left: node, "+", right: node]; }
            schema Continue { form [left: node, ",", rest: node]; }
            schema Complete { form [left: node, ",", rest: node]; }
            coordination_forms P { series End, Continue, Complete; }
            coordinable A using P { forms serial; series Tail; }
        } },
        "series category already declared",
    );
    rejects(
        quote! { mod g {
            category A(); construction Atom: A { form ["x"]; }
            schema Pair { form [left: node, "+", right: node]; }
            instance Pair: A { bind left, right = A; }
            coordination_forms P { binary Pair; }
            coordinable A using P { forms binary; }
        } },
        "unique result category",
    );
}

#[test]
fn generated_binary_emits_exactly_the_explicit_contract_for_every_combiner() {
    for (combiner, table, equations) in [
        (
            quote!(same),
            quote!(),
            quote! { agree left.Flag = right.Flag; export Flag = left.Flag; },
        ),
        (
            quote!(all),
            quote! { table truth(Flag, Flag) -> Flag { (Yes, Yes) => Yes, (_, _) => No, } },
            quote! { export Flag = truth(left.Flag, right.Flag); },
        ),
        (
            quote!(any),
            quote! { table truth(Flag, Flag) -> Flag { (No, No) => No, (_, _) => Yes, } },
            quote! { export Flag = truth(left.Flag, right.Flag); },
        ),
        (
            quote!(left.Flag),
            quote!(),
            quote! { export Flag = left.Flag; },
        ),
        (quote!(No), quote!(), quote! { export Flag = No; }),
    ] {
        let inferred = grammar(&quote!(head Flag combine #combiner), &quote!(forms binary;));
        let explicit = quote! { mod g {
            feature Flag { No, Yes } default No;
            #table
            category A(head Flag);
            construction Atom: A { form [head: lexical(Noun)]; export Flag = Yes; }
            schema Pair { head none; form [left: node, " + ", right: node]; }
            instance Pair: A { bind left, right = A; #equations }
        } };
        assert_eq!(
            compile(inferred).unwrap().to_string(),
            compile(explicit).unwrap().to_string()
        );
    }
}

#[test]
fn correlative_helpers_preserve_marker_requirements_and_every_constructor_field() {
    let d = normalized(quote! { mod g {
        feature Flag { No, Yes }
        feature Marker { Alternative, Negative }
        category A(Flag combine same);
        construction Atom: A { form ["x"]; export Flag = Yes; }
        schema End {
            head none;
            form [left: node, ", ", coordinator: lexical(Coordinator), " ", right: node];
            export Marker = coordinator.Marker;
        }
        schema Continue {
            head none;
            form [left: node, ", ", rest: node];
            export Marker = rest.Marker;
        }
        schema Either {
            head none;
            form [marker: lexical(Determinative), " ", left: node, " ", coordinator: lexical(Coordinator), " ", right: node];
            form ["[", marker: lexical(Determinative), " ", left: node, " ", coordinator: lexical(Coordinator), " ", right: node, "]"];
            require coordinator.Marker = Alternative;
        }
        schema EitherSerial {
            head none;
            form [marker: lexical(Determinative), " ", left: node, ", ", rest: node];
            require rest.Marker = Alternative;
        }
        coordination_forms Correlatives {
            correlative_series End, Continue;
            correlative Either, EitherSerial;
        }
        coordinable A using Correlatives {
            forms Either, EitherSerial;
            correlative_series Tail;
        }
    } });
    let either = d.constructions.iter().find(|c| c.name == "Either").unwrap();
    assert_eq!(either.forms.len(), 2);
    for form in &either.forms {
        let fields: Vec<_> = form
            .iter()
            .filter_map(
                |p| if let parse::Part::Field(n, _) = p { Some(n.to_string()) } else { None },
            )
            .collect();
        assert_eq!(fields, ["marker", "left", "coordinator", "right"]);
    }
    let tail = d.categories.iter().find(|c| c.name == "Tail").unwrap();
    assert_eq!(
        tail.features
            .iter()
            .map(|f| f.name.to_string())
            .collect::<Vec<_>>(),
        ["Flag", "Marker"]
    );
    let serial = d
        .constructions
        .iter()
        .find(|c| c.name == "EitherSerial")
        .unwrap();
    let parse::Equation::Require(r, value) = &serial.equations[0] else {
        panic!("schema requirement must survive")
    };
    assert_eq!(
        (
            r.field.to_string(),
            r.feature.to_string(),
            value.to_string()
        ),
        ("rest".into(), "Marker".into(), "Alternative".into())
    );
    assert!(serial.shared);
}
