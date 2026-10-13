use proc_macro2::TokenStream;
use quote::quote;

use crate::compile;
use crate::parse;
use crate::tests::rejects;

fn grammar(interface: &TokenStream, block: &TokenStream) -> TokenStream {
    quote! { mod g {
        feature Flag { No, Yes } default No;
        category A(#interface);
        construction Atom: A { form [head: lexical(Noun)]; export Flag = Yes; }
        schema Pair { head none; form [left: node, " + ", right: node]; }
        schema End { head none; form [left: node, ", + ", right: node]; }
        schema Continue { head none; form [left: node, ", ", rest: node]; }
        schema Complete { head none; form [left: node, ", ", rest: node]; }
        coordination_forms Standard { binary Pair; serial End, Continue, Complete; }
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
            forms binary, serial; helper Tail;
            on helper { combine Flag = always No; require right.Flag = Yes; }
            on tail { require left.Flag = Yes; }
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
        category A(Flag combine choose(coordinator.Kind, right.Flag, left.Flag));
        construction Atom: A { form ["x"]; export Flag = Yes; }
        schema End { head none; form [left: node, " ", coordinator: lexical(Coordinator), " ", right: node]; }
        schema Continue { head none; form [left: node, ", ", rest: node]; }
        schema Complete { head none; form [left: node, ", ", rest: node]; }
        coordination_forms Standard { serial End, Continue, Complete; }
        coordinable A using Standard { forms serial; helper Tail; }
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
        &quote!(head Flag combine always Yes),
        &quote! {
            forms binary, serial; helper Tail;
            on helper { omit Flag; }
        },
    ));
}

#[test]
fn disjoint_context_overrides_replace_the_category_override() {
    let d = normalized(grammar(
        &quote!(head Flag combine always Yes),
        &quote! {
            forms binary, serial; helper Tail;
            on helper { combine Flag = always No; }
            on binary { combine Flag = always Yes; }
        },
    ));
    for (name, expected) in [
        ("Pair", "Yes"),
        ("End", "No"),
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
        "left, right or coordinator",
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
            quote!(forms binary; on binary { combine Flag = always Yes; combine Flag = always No; }),
            "duplicate combiner override",
        ),
        (
            quote!(forms binary; on binary { combine Flag = always Yes; } on result { combine Flag = always No; }),
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
            quote!(forms serial; helper Tail; on helper { omit Missing; }),
            "cannot omit unknown",
        ),
        (quote!(forms binary, binary;), "duplicate coordination form"),
        (
            quote!(forms binary; combine Flag = always Yes;),
            "belong in an on context",
        ),
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
            coordination_forms P { serial End, Continue, Complete; }
            coordinable A using P { forms serial; helper Tail; }
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
        (quote!(always No), quote!(), quote! { export Flag = No; }),
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
            correlative_helper End, Continue;
            either Either, EitherSerial;
        }
        coordinable A using Correlatives {
            forms either;
            correlative_helper Tail;
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

#[test]
fn feature_defaults_category_overrides_and_context_overrides_have_ordered_precedence() {
    for (annotation, contexts, expected) in [
        (quote!(), quote!(), "No"),
        (quote!(combine always Yes), quote!(), "Yes"),
        (
            quote!(combine always Yes),
            quote!(on binary { combine Flag = always No; }),
            "No",
        ),
    ] {
        let d = normalized(quote! { mod g {
            feature Flag { No, Yes } default No combine always No;
            category A(head Flag #annotation);
            construction Atom: A { head none; form ["x"]; export Flag = Yes; }
            schema Pair { head none; form [left: node, "+", right: node]; }
            coordination_forms P { binary Pair; }
            coordinable A using P { forms binary; #contexts }
        } });
        let pair = d.constructions.iter().find(|c| c.name == "Pair").unwrap();
        let parse::Equation::ExportConstant(f, value) = &pair.equations[0] else {
            panic!("constant combination")
        };
        assert_eq!(
            (f.to_string(), value.to_string()),
            ("Flag".into(), expected.into())
        );
    }
}

#[test]
fn constant_values_can_have_primitive_combiner_names() {
    for value in [quote!(all), quote!(any), quote!(same)] {
        let d = normalized(quote! { mod g {
            feature Mode { all, any, same } combine always #value;
            category A(Mode);
            construction Atom: A { form ["x"]; export Mode = all; }
            schema Pair { head none; form [left: node, "+", right: node]; }
            coordination_forms P { binary Pair; }
            coordinable A using P { forms binary; }
        } });
        let pair = d.constructions.iter().find(|c| c.name == "Pair").unwrap();
        let parse::Equation::ExportConstant(_, actual) = &pair.equations[0] else {
            panic!("always must be a constant")
        };
        assert_eq!(actual.to_string(), value.to_string());
    }
}

#[test]
fn inferred_tables_equal_explicit_pairs_and_other_shapes_need_arguments() {
    for marker in [quote!(Kind), quote!(CoordinationKind)] {
        for (inputs, rows, args, inferred) in [
            (
                quote!(Flag, Flag),
                quote!((Yes, Yes) => Yes, (_, _) => No,),
                quote!(left.Flag, right.Flag),
                true,
            ),
            (
                quote!(#marker, Flag, Flag),
                quote!((Add, Yes, Yes) => Yes, (_, _, _) => No,),
                quote!(coordinator.#marker, left.Flag, right.Flag),
                false,
            ),
        ] {
            let grammar = |combiner| {
                quote! { mod g {
                    feature Flag { No, Yes } combine #combiner;
                    feature #marker { Add, Alternative }
                    table choose(#inputs) -> Flag { #rows }
                    category A(Flag);
                    construction Atom: A { form ["x"]; export Flag = Yes; }
                    schema End { head none; form [left: node, coordinator: lexical(Coordinator), right: node]; }
                    schema Continue { head none; form [left: node, ",", rest: node]; }
                    schema Complete { head none; form [left: node, ",", rest: node]; }
                    coordination_forms P { serial End, Continue, Complete; }
                    coordinable A using P { forms serial; }
                } }
            };
            let explicit = compile(grammar(quote!(choose(#args)))).unwrap().to_string();
            if inferred {
                assert_eq!(
                    compile(grammar(quote!(choose))).unwrap().to_string(),
                    explicit
                );
            } else {
                rejects(
                    grammar(quote!(choose)),
                    "table shape needs explicit combiner arguments",
                );
                let expanded = normalized(grammar(quote!(choose(#args))));
                let complete = expanded
                    .constructions
                    .iter()
                    .find(|c| c.name == "Complete")
                    .unwrap();
                let parse::Equation::ExportTable(_, _, refs) = &complete.equations[0] else {
                    panic!("table export")
                };
                assert_eq!(
                    refs.iter()
                        .map(|r| format!("{}.{}", r.field, r.feature))
                        .collect::<Vec<_>>(),
                    [
                        format!("rest.{marker}"),
                        "left.Flag".into(),
                        "rest.Flag".into()
                    ]
                );
            }
        }
    }
}

#[test]
fn template_requirements_apply_only_to_selected_contexts() {
    let d = normalized(quote! { mod g {
        feature Flag { No, Yes } combine same;
        category A(Flag);
        construction Atom: A { form ["x"]; export Flag = Yes; }
        schema Pair { head none; form [left: node, coordinator: lexical(Coordinator), right: node]; }
        schema End { head none; form [left: node, coordinator: lexical(Coordinator), right: node]; }
        schema Continue { head none; form [left: node, ",", rest: node]; }
        schema Complete { head none; form [left: node, ",", rest: node]; }
        coordination_forms P {
            binary Pair; serial End, Continue, Complete;
            on ordinary_overt { require coordinator.Flag = Yes; }
        }
        coordinable A using P { forms binary, serial; }
    } });
    for name in ["Pair", "End", "Continue", "Complete"] {
        let c = d.constructions.iter().find(|c| c.name == name).unwrap();
        let requirements: Vec<_> = c
            .equations
            .iter()
            .filter_map(|e| match e {
                parse::Equation::Require(r, v) => {
                    Some((r.field.to_string(), r.feature.to_string(), v.to_string()))
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            requirements,
            if ["Pair", "End"].contains(&name) {
                vec![("coordinator".into(), "Flag".into(), "Yes".into())]
            } else {
                vec![]
            }
        );
    }
    assert!(d.categories.iter().any(|c| c.name == "ASeries"));
}

#[test]
fn distinct_member_and_result_categories_emit_the_explicit_contract() {
    let common = quote! {
        feature Flag { No, Yes } combine same;
        category Member(Flag);
        category Result(Flag);
        construction Atom: Member { form ["x"]; export Flag = Yes; }
        schema Pair { head none; form [left: node, "+", right: node]; }
    };
    let generated = quote! { mod g { #common
        coordination_forms P { binary Pair; }
        coordinable Result of Member using P { forms binary; }
    } };
    let explicit = quote! { mod g { #common
        instance Pair: Result {
            bind left, right = Member;
            agree left.Flag = right.Flag;
            export Flag = left.Flag;
        }
    } };
    assert_eq!(
        compile(generated).unwrap().to_string(),
        compile(explicit).unwrap().to_string()
    );
}

#[test]
fn unused_category_annotations_irregular_inference_and_unknown_members_reject() {
    rejects(
        quote! { mod g {
            feature Flag { No, Yes }
            category A(Flag combine same);
            construction Atom: A { form ["x"]; export Flag = Yes; }
        } },
        "combine annotation requires a coordinable block",
    );
    rejects(
        quote! { mod g {
            feature Flag { No, Yes } combine choose;
            table choose(Flag) -> Flag { (Yes) => Yes, (No) => No, }
            category A(Flag);
            construction Atom: A { form ["x"]; export Flag = Yes; }
            schema Pair { head none; form [left: node, "+", right: node]; }
            coordination_forms P { binary Pair; }
            coordinable A using P { forms binary; }
        } },
        "table shape needs explicit combiner arguments",
    );
    rejects(
        quote! { mod g {
            category A();
            schema Pair { head none; form [left: node, "+", right: node]; }
            coordination_forms P { binary Pair; }
            coordinable A of Missing using P { forms binary; }
        } },
        "unknown coordination member category",
    );
}

#[test]
fn a_context_cannot_hide_a_missing_default_or_interface_combiner() {
    rejects(
        grammar(
            &quote!(Flag),
            &quote!(forms binary; on binary { combine Flag = always Yes; }),
        ),
        "requires a combiner",
    );
}

#[test]
fn template_contexts_reject_misspellings_and_combiner_overrides() {
    for (context, error) in [
        (
            quote!(on Missing { require left.Flag = Yes; }),
            "matches no template",
        ),
        (
            quote!(on binary { combine Flag = always Yes; }),
            "template contexts contain requirements",
        ),
    ] {
        rejects(
            quote! { mod g {
                feature Flag { No, Yes } combine same;
                category A(Flag);
                schema Pair { head none; form [left: node, "+", right: node]; }
                coordination_forms P { binary Pair; #context }
                coordinable A using P { forms binary; }
            } },
            error,
        );
    }
}

#[test]
fn tail_overt_and_correlative_contexts_select_exactly_their_roles() {
    for (context, expected) in [
        (
            quote!(tail),
            vec![
                "Continue",
                "Complete",
                "CorrelativeContinue",
                "EitherSerial",
                "NeitherSerial",
            ],
        ),
        (
            quote!(overt),
            vec!["Pair", "End", "CorrelativeEnd", "Both", "Either", "Neither"],
        ),
        (
            quote!(correlative),
            vec![
                "CorrelativeEnd",
                "CorrelativeContinue",
                "Both",
                "Either",
                "EitherSerial",
                "Neither",
                "NeitherSerial",
            ],
        ),
    ] {
        let d = normalized(quote! { mod g {
            feature Flag { No, Yes } combine same;
            category A(Flag);
            construction Atom: A { form ["x"]; export Flag = Yes; }
            schema Pair { head none; form [left: node, "+", right: node]; }
            schema End { head none; form [left: node, ",+", right: node]; }
            schema Continue { head none; form [left: node, ",", rest: node]; }
            schema Complete { head none; form [left: node, ",", rest: node]; }
            schema CorrelativeEnd { head none; form [left: node, ",", right: node]; }
            schema CorrelativeContinue { head none; form [left: node, ",", rest: node]; }
            schema Both { head none; form ["both", left: node, "and", right: node]; }
            schema Either { head none; form ["either", left: node, "or", right: node]; }
            schema EitherSerial { head none; form ["either", left: node, ",", rest: node]; }
            schema Neither { head none; form ["neither", left: node, "nor", right: node]; }
            schema NeitherSerial { head none; form ["neither", left: node, ",", rest: node]; }
            coordination_forms P {
                binary Pair; serial End, Continue, Complete;
                correlative_helper CorrelativeEnd, CorrelativeContinue;
                both Both; either Either, EitherSerial; neither Neither, NeitherSerial;
            }
            coordinable A using P {
                forms binary, serial, both, either, neither;
                on #context { require right.Flag = Yes; }
            }
        } });
        for c in &d.constructions {
            let requirements: Vec<_> = c
                .equations
                .iter()
                .filter_map(|e| match e {
                    parse::Equation::Require(r, value) => Some((
                        r.field.to_string(),
                        r.feature.to_string(),
                        value.to_string(),
                    )),
                    _ => None,
                })
                .collect();
            let name = c.name.to_string();
            let tail = [
                "Continue",
                "Complete",
                "CorrelativeContinue",
                "EitherSerial",
                "NeitherSerial",
            ]
            .contains(&name.as_str());
            assert_eq!(
                requirements,
                if expected.contains(&name.as_str()) {
                    vec![(
                        if tail { "rest" } else { "right" }.into(),
                        "Flag".into(),
                        "Yes".into(),
                    )]
                } else {
                    vec![]
                },
                "{context}: {name}"
            );
        }
    }
}

#[test]
fn block_combiner_diagnostics_point_at_the_off_interface_feature() {
    for feature in ["Missing", "Other"] {
        let source = format!(
            r#"mod g {{
    feature Flag {{ No, Yes }} combine same;
    feature Other {{ No, Yes }}
    category A(Flag);
    schema Pair {{ head none; form [left: node, "+", right: node]; }}
    coordination_forms P {{ binary Pair; }}
    coordinable A using P {{
        forms binary;
        on binary {{ combine {feature} = always Yes; }}
    }}
}}"#
        );
        let error = compile(source.parse().unwrap()).unwrap_err();
        assert_eq!(
            error.to_string(),
            format!("feature {feature} is not on the A interface in this coordinable block")
        );
        assert_eq!(
            (error.span().start().line, error.span().start().column),
            (9, 28)
        );
    }
}

#[test]
fn exact_schema_escape_is_rejected_in_forms_and_contexts() {
    for block in [
        quote!(forms schema(Pair);),
        quote!(forms binary; on schema(Pair) { require left.Flag = Yes; }),
    ] {
        rejects(grammar(&quote!(Flag combine same), &block), "expected");
    }
}

#[test]
fn helper_only_summaries_are_declared_and_have_no_propagation_mode() {
    let d = normalized(quote! { mod g {
        feature Flag { No, Yes } combine same;
        feature Summary { No, Yes }
        category A(head Flag);
        construction Atom: A { head none; form ["x"]; export Flag = Yes; }
        schema End { head none; form [left: node, ",+", right: node]; }
        schema Continue { head none; form [left: node, ",", rest: node]; }
        schema Complete { head none; form [left: node, ",", rest: node]; }
        coordination_forms P { serial End, Continue, Complete; }
        coordinable A using P {
            forms serial;
            on helper { combine Summary = always Yes; }
        }
    } });
    let helper = d.categories.iter().find(|c| c.name == "ASeries").unwrap();
    assert_eq!(
        helper
            .features
            .iter()
            .map(|f| f.name.to_string())
            .collect::<Vec<_>>(),
        ["Flag", "Summary"]
    );
    for feature in &helper.features {
        assert!(feature.propagation.is_none());
    }
    for name in ["End", "Continue"] {
        let c = d.constructions.iter().find(|c| c.name == name).unwrap();
        let summaries: Vec<_> = c
            .equations
            .iter()
            .filter_map(|e| match e {
                parse::Equation::ExportConstant(f, v) => Some((f.to_string(), v.to_string())),
                _ => None,
            })
            .collect();
        assert_eq!(summaries, [("Summary".into(), "Yes".into())]);
    }
    let source = r#"mod g {
    feature Flag { No, Yes } combine same;
    category A(Flag);
    schema End { head none; form [left: node, ",+", right: node]; }
    schema Continue { head none; form [left: node, ",", rest: node]; }
    schema Complete { head none; form [left: node, ",", rest: node]; }
    coordination_forms P { serial End, Continue, Complete; }
    coordinable A using P {
        forms serial;
        on helper { combine Missing = always Yes; }
    }
}"#;
    let error = compile(source.parse().unwrap()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "declaration Missing: unknown feature in admission obligation"
    );
    assert_eq!(
        (error.span().start().line, error.span().start().column),
        (10, 28)
    );
}
