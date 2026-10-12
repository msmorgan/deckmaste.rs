use proc_macro2::TokenStream;
use quote::quote;

use crate::compile;
use crate::tests::rejects;

fn same_projection(inferred: TokenStream, explicit: TokenStream) {
    assert_eq!(
        compile(inferred).unwrap().to_string(),
        compile(explicit).unwrap().to_string(),
        "authoring must elaborate to the same generated admission and Reading representation"
    );
}

#[test]
fn inherited_builtin_and_lexical_features_match_explicit_exports() {
    same_projection(
        quote! { mod g {
            feature number head;
            feature Class head { Ordinary, Special } default Ordinary;
            category A(number, Class);
            construction A: A { head word; form [word: lexical(Noun)]; }
        } },
        quote! { mod g {
            feature Class { Ordinary, Special } default Ordinary;
            category A(number, Class);
            construction A: A {
                form [word: lexical(Noun)];
                export number = word.number;
                export Class = word.Class;
            }
        } },
    );
}

#[test]
fn local_default_ignores_an_accessible_head_and_headless_default_fills() {
    for head in [quote!(), quote!(head word;)] {
        same_projection(
            quote! { mod g {
                feature Class local { Ordinary, Special } default Ordinary;
                category A(Class);
                construction A: A { #head form [word: lexical(Noun)]; }
            } },
            quote! { mod g {
                feature Class { Ordinary, Special } default Ordinary;
                category A(Class);
                construction A: A {
                    form [word: lexical(Noun)];
                    export Class = Ordinary;
                }
            } },
        );
    }
}

#[test]
fn head_default_fills_only_when_there_is_no_static_source() {
    for head in [quote!(), quote!(head word;)] {
        same_projection(
            quote! { mod g {
                feature Class head { Ordinary, Special } default Ordinary;
                category Narrow(); category A(Class);
                construction Narrow: Narrow { form [word: lexical(Noun)]; }
                construction A: A { #head form [word: Narrow]; }
            } },
            quote! { mod g {
                feature Class { Ordinary, Special } default Ordinary;
                category Narrow(); category A(Class);
                construction Narrow: Narrow { form [word: lexical(Noun)]; }
                construction A: A { form [word: Narrow]; export Class = Ordinary; }
            } },
        );
    }
}

#[test]
fn explicit_exports_override_inheritance_and_both_kinds_of_default() {
    for propagation in [quote!(head), quote!(local)] {
        for source in [quote!(lexical(Noun)), quote!(Narrow)] {
            same_projection(
                quote! { mod g {
                    feature Class #propagation { Ordinary, Special } default Ordinary;
                    category Narrow(); category A(Class);
                    construction Narrow: Narrow { form ["x"]; }
                    construction A: A {
                        head word; form [word: #source]; export Class = Special;
                    }
                } },
                quote! { mod g {
                    feature Class { Ordinary, Special } default Ordinary;
                    category Narrow(); category A(Class);
                    construction Narrow: Narrow { form ["x"]; }
                    construction A: A { form [word: #source]; export Class = Special; }
                } },
            );
        }
    }
}

#[test]
fn policies_and_schema_instances_supply_explicit_overrides_before_fill() {
    same_projection(
        quote! { mod g {
            feature Class head { Ordinary, Special } default Ordinary;
            category A(Class);
            policy Special { export Class = Special; }
            schema Wrapper { head word; form ["(", word: node, ")"]; }
            instance Wrapper: A { bind word = A; use Special; }
            construction Leaf: A { form ["x"]; }
        } },
        quote! { mod g {
            feature Class { Ordinary, Special } default Ordinary;
            category A(Class);
            schema Wrapper { form ["(", word: node, ")"]; }
            instance Wrapper: A { bind word = A; export Class = Special; }
            construction Leaf: A { form ["x"]; export Class = Ordinary; }
        } },
    );
    same_projection(
        quote! { mod g {
            feature Class head { Ordinary, Special } default Ordinary;
            category A(Class); category B(Class);
            schema Wrapper { form ["(", word: node, ")"]; }
            instance Wrapper<Result, Child: word>: [(A, Self), (B, Self)] { head word; }
            construction A: A { form ["a"]; }
            construction B: B { form ["b"]; }
        } },
        quote! { mod g {
            feature Class { Ordinary, Special } default Ordinary;
            category A(Class); category B(Class);
            schema Wrapper { form ["(", word: node, ")"]; }
            instance Wrapper<Result, Child: word>: [(A, Self), (B, Self)] {
                export Class = word.Class;
            }
            construction A: A { form ["a"]; export Class = Ordinary; }
            construction B: B { form ["b"]; export Class = Ordinary; }
        } },
    );
}

#[test]
fn each_form_uses_its_concrete_head_interface_and_keeps_its_index() {
    same_projection(
        quote! { mod g {
            feature Class head { Ordinary, Special } default Ordinary;
            category Wide(Class); category Narrow(); category A(Class);
            construction Wide: Wide { form ["w"]; export Class = Special; }
            construction Narrow: Narrow { form ["n"]; }
            construction A: A {
                head word;
                form [word: Wide];
                form [word: Narrow];
                form ["(", word: Wide, ")"] export Class = Ordinary;
            }
        } },
        quote! { mod g {
            feature Class { Ordinary, Special } default Ordinary;
            category Wide(Class); category Narrow(); category A(Class);
            construction Wide: Wide { form ["w"]; export Class = Special; }
            construction Narrow: Narrow { form ["n"]; }
            construction A: A {
                form [word: Wide] export Class = word.Class;
                form [word: Narrow] export Class = Ordinary;
                form ["(", word: Wide, ")"] export Class = Ordinary;
            }
        } },
    );
}

#[test]
fn missing_source_and_default_name_the_construction_and_feature() {
    for mode in [quote!(head), quote!(local), quote!()] {
        let input = quote! { mod g {
            feature Class #mode { Ordinary, Special }
            category A(Class); category Narrow();
            construction Narrow: Narrow { form ["x"]; }
            construction Missing: A { head word; form [word: Narrow]; }
        } };
        rejects(input.clone(), "declaration Missing");
        rejects(input, "missing export Class");
    }
    rejects(
        quote! { mod g {
            feature Class { Ordinary, Special } default Ordinary;
            category A(Class);
            construction Legacy: A { form ["x"]; }
        } },
        "missing export Class",
    );
}

#[test]
fn invalid_heads_are_rejected_even_when_exports_are_explicit() {
    for (directive, source, message) in [
        (quote!(head absent;), quote!(A), "unknown head field absent"),
        (
            quote!(head word; head word;),
            quote!(A),
            "duplicate construction head",
        ),
        (
            quote!(head word;),
            quote!(optional(A)),
            "head must be a single",
        ),
        (
            quote!(head word;),
            quote!(repeat(A, ",")),
            "head must be a single",
        ),
        (
            quote!(head word;),
            quote!(optional(lexical(Noun), " ")),
            "head must be a single",
        ),
    ] {
        rejects(
            quote! { mod g {
                category A(); construction Leaf: A { form ["x"]; }
                construction Bad: A { #directive form ["(", word: #source, ")"]; }
            } },
            message,
        );
    }
    rejects(
        quote! { mod g {
            category A(); policy Bad { head word; }
            construction A: A { form ["x"]; }
        } },
        "policy admits only feature equations",
    );
    rejects(
        quote! { mod g {
            category A(); schema Bad { head word; form [word: lexical(Noun)]; }
            instance Bad: A { head word; }
        } },
        "duplicate construction head",
    );
}

#[test]
fn explicit_conflicts_and_domain_checks_still_apply() {
    for extra in [
        quote!(export Class = Special;),
        quote!(
            use Special;
        ),
    ] {
        rejects(
            quote! { mod g {
                feature Class head { Ordinary, Special } default Ordinary;
                category A(Class);
                policy Special { export Class = Special; }
                construction Bad: A {
                    head word; form [word: lexical(Noun)] export Class = Ordinary;
                    #extra
                }
            } },
            "each declared Category feature exactly once",
        );
    }
    rejects(
        quote! { mod g {
            feature Class head { Ordinary, Special } default Ordinary;
            category A(Class);
            construction Bad: A {
                head word; form [word: lexical(Noun)]; export Class = word.number;
            }
        } },
        "same domain",
    );
    rejects(
        quote! { mod g {
            feature Class local { Ordinary, Special } default Wrong;
        } },
        "not a value of feature Class",
    );
    rejects(
        quote! { mod g { feature Missing head; } },
        "must name a builtin feature",
    );
    rejects(
        quote! { mod g { feature number head; feature number local; } },
        "unique generated identity",
    );
}

#[test]
fn explicit_table_supplies_a_feature_without_any_head_source_or_default() {
    same_projection(
        quote! { mod g {
            feature Class head { Ordinary, Special }
            category A(Class);
            table classify(number) -> Class { (Singular) => Ordinary, (Plural) => Special }
            construction A: A {
                form [word: lexical(Noun)];
                export Class = classify(word.number);
            }
        } },
        quote! { mod g {
            feature Class { Ordinary, Special }
            category A(Class);
            table classify(number) -> Class { (Singular) => Ordinary, (Plural) => Special }
            construction A: A {
                form [word: lexical(Noun)];
                export Class = classify(word.number);
            }
        } },
    );
}
