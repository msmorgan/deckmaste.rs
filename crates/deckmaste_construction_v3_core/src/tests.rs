use quote::quote;

use crate::compile;

fn rejects(input: proc_macro2::TokenStream, expected: &str) {
    let error = compile(input)
        .expect_err("invalid declarations must fail before generated Rust is compiled");
    assert!(error.to_string().contains(expected), "{error}");
}

#[test]
fn custom_feature_defaults_are_explicit_typed_and_cannot_replace_builtins() {
    compile(quote! {
        mod valid {
            feature Class { None, Special } default None;
            category Atom(Class);
            schema Atom {
                form [word: lexical(Noun)];
                export Class = word.Class;
            }
            instance Atom: Atom {}
        }
    })
    .unwrap();
    rejects(
        quote! { mod bad { feature Class { None, Special } default Unknown; } },
        "not a value of feature Class",
    );
    rejects(
        quote! { mod bad { feature number { Singular, Plural } default Singular; } },
        "unique generated identity required",
    );
    for declaration in [
        quote! { mod bad { feature Class { None } default; } },
        quote! { mod bad { feature Class { None } default None } },
        quote! { mod bad { feature Class { None } default None; default None; } },
    ] {
        assert!(compile(declaration).is_err());
    }
}

#[test]
fn structured_frame_signatures_preserve_typed_ir_and_every_generated_projection() {
    for (structured, serialized) in [
        (
            quote!(Predicate(Object(NounPhrase))),
            r#"(kind:"Predicate",items:[Argument((relation:Object,category:"NounPhrase"))])"#,
        ),
        (
            quote!(Predicate(
                Object(NounPhrase),
                Preposition(To),
                Object(NounPhrase)
            )),
            r#"(kind:"Predicate",items:[Argument((relation:Object,category:"NounPhrase")),Marker(vocabulary:"Preposition",member:"To"),Argument((relation:Object,category:"NounPhrase"))])"#,
        ),
        (
            quote!(Auxiliary(Complement(ParticipialPredicate))),
            r#"(kind:"Auxiliary",items:[Argument((relation:Complement,category:"ParticipialPredicate"))])"#,
        ),
        (quote!(Nominal()), r#"(kind:"Nominal",items:[])"#),
        (
            quote!(Nominal(Marked(Preposition, Of, Complement(CostSymbols)))),
            r#"(kind:"Nominal",items:[Marked(vocabulary:"Preposition",member:"Of",slot:(relation:Complement,category:"CostSymbols"))])"#,
        ),
        (
            quote!(Custom(
                Subject(UnboundCategory),
                Optional(Optional(Object("other category"))),
                Optional(Marked(
                    "custom vocabulary",
                    "member",
                    Complement(UnboundCategory)
                )),
                Marker(Object, ReservedMember),
                Literal("legacy signature")
            )),
            r#"(kind:"Custom",items:[Argument((relation:Subject,category:"UnboundCategory")),Optional(Optional(Argument((relation:Object,category:"other category")))),Optional(Marked(vocabulary:"custom vocabulary",member:"member",slot:(relation:Complement,category:"UnboundCategory"))),Marker(vocabulary:"Object",member:"ReservedMember"),Literal("legacy signature")])"#,
        ),
    ] {
        let declaration = |signature: proc_macro2::TokenStream| {
            quote! {
                mod fixture {
                    frame Selected = #signature;
                    feature Verdict { Yes }
                    table selected(frame) -> Verdict { (Selected) => Yes }
                    category Head(frame);
                    construction Head: Head {
                        form [head: lexical(Verb)];
                        export frame = head.frame;
                        require selected(head.frame) = Yes;
                    }
                }
            }
        };
        let old = declaration(quote!(#serialized));
        let new = declaration(structured);
        let old_ir = crate::ir::validate(syn::parse2(old.clone()).unwrap()).unwrap();
        let new_ir = crate::ir::validate(syn::parse2(new.clone()).unwrap()).unwrap();
        assert_eq!(old_ir.frames, new_ir.frames);
        assert_eq!(
            compile(old).unwrap().to_string(),
            compile(new).unwrap().to_string()
        );
    }
}

#[test]
fn structured_frames_reject_malformed_items_and_cross_syntax_duplicates() {
    for signature in [
        quote!(Predicate),
        quote!(Predicate(Object())),
        quote!(Predicate(Object(NounPhrase, Other))),
        quote!(Predicate(Preposition(To, Of))),
        quote!(Predicate(Optional())),
        quote!(Predicate(Optional(Object(NounPhrase), Object(NounPhrase)))),
        quote!(Predicate(Marked(Preposition, Of, Unknown(NounPhrase)))),
        quote!(Predicate(Marked(
            Preposition,
            Of,
            Object(NounPhrase, Other)
        ))),
        quote!(Predicate(Literal(Unquoted))),
    ] {
        let result = std::panic::catch_unwind(|| {
            compile(quote! {
                mod bad { frame Invalid = #signature; }
            })
        });
        assert!(result.expect("frame diagnostics must not panic").is_err());
    }
    rejects(
        quote! {
            mod bad {
                frame Old = r#"(kind:"Predicate",items:[Argument((relation:Object,category:"NounPhrase"))])"#;
                frame New = Predicate(Object(NounPhrase));
            }
        },
        "duplicate frame signature",
    );
}

#[test]
fn rejects_surface_loss_and_duplicate_ownership() {
    rejects(
        quote! { mod bad { category A(); construction Lost: A { form [word: lexical(Noun)]; form []; } } },
        "Lost: parse/materialize/realize",
    );
    rejects(
        quote! { mod bad { category A(); construction Twice: A { form [word: lexical(Noun), word: lexical(Noun)]; } } },
        "Twice: each lexical/structural field",
    );
    rejects(
        quote! { mod bad { category A(); construction Changed: A { form [child: A]; form [child: optional(A)]; } } },
        "same named fields and cardinalities",
    );
    rejects(
        quote! { mod bad { category A(); construction Missing: A {} } },
        "Missing: realization",
    );
}

#[test]
fn rejects_unreachable_and_ill_typed_feature_dependencies() {
    rejects(
        quote! { mod bad { category A(number); construction Missing: A { form [word: lexical(Noun)]; } } },
        "Missing: required Category feature",
    );
    rejects(
        quote! { mod bad { category A(); category B(); construction Hidden: B { form [child: A]; require child.number = Singular; } } },
        "Hidden: admission cannot reach child.number",
    );
    rejects(
        quote! { mod bad { category A(); construction Wrong: A { form [word: lexical(Noun)]; require word.number = First; } } },
        "not a value of feature number",
    );
    rejects(
        quote! { mod bad { category A(); construction Crossed: A { form [word: lexical(Noun)]; agree word.number = word.person; } } },
        "Crossed: agreement requires the same feature domain",
    );
    rejects(
        quote! { mod bad { category A(); construction Optional: A { form [child: optional(A)]; require child.number = Singular; } } },
        "Optional: optional/repeated fields have no single feature value",
    );
    rejects(
        quote! { mod bad { category A(); construction Contradiction: A { form [word: lexical(Noun)]; require word.number = Singular; require word.number = Plural; } } },
        "Contradiction: contradictory admission requirements",
    );
    rejects(
        quote! { mod bad { category A(); construction Unknown: A { form [word: lexical(Noun)]; require other.number = Singular; } } },
        "Unknown: admission cannot reach field other",
    );
}

#[test]
fn constant_exports_are_typed_and_cannot_duplicate_or_escape_the_interface() {
    for (input, expected) in [
        (
            quote! { mod bad { category A(number); construction Wrong: A { form []; export number = First; } } },
            "not a value of feature number",
        ),
        (
            quote! { mod bad { category A(); construction Hidden: A { form []; export number = Singular; } } },
            "each declared Category feature exactly once",
        ),
        (
            quote! { mod bad { category A(number); construction Twice: A { form [head: lexical(Noun)]; export number = head.number; export number = Plural; } } },
            "each declared Category feature exactly once",
        ),
        (
            quote! { mod bad { category A(number); construction Twice: A { form []; export number = Singular; export number = Plural; } } },
            "each declared Category feature exactly once",
        ),
    ] {
        rejects(input, expected);
    }
}

#[test]
fn feature_tables_reject_untyped_ambiguous_and_inaccessible_calls() {
    for (input, expected) in [
        (
            quote! { mod bad { category A(); table join(number) -> person { (Singular) => Singular } construction A: A { form []; } } },
            "not a value of feature person",
        ),
        (
            quote! { mod bad { category A(); table join(number) -> person { (First) => First } construction A: A { form []; } } },
            "not a value of feature number",
        ),
        (
            quote! { mod bad { category A(); table join(number) -> person { () => First } construction A: A { form []; } } },
            "row has wrong arity",
        ),
        (
            quote! { mod bad { category A(); table join(number) -> person { (Singular) => First, (Singular) => Second } construction A: A { form []; } } },
            "duplicate feature table input tuple",
        ),
        (
            quote! { mod bad { category A(number); construction A: A { form [head: lexical(Noun)]; export number = unknown(head.number); } } },
            "unknown feature table",
        ),
        (
            quote! { mod bad { category A(person); table join(number) -> person { (Singular) => Third } construction A: A { form [head: lexical(Noun)]; export person = join(head.person); } } },
            "argument has wrong domain",
        ),
        (
            quote! { mod bad { category A(person); table join(number) -> person { (Singular) => Third } construction A: A { form [head: lexical(Noun)]; export person = join(head.number, head.number); } } },
            "call has wrong arity",
        ),
        (
            quote! { mod bad { category A(person); category B(); table join(number) -> person { (Singular) => Third } construction A: A { form [head: B]; export person = join(head.number); } construction B: B { form []; } } },
            "cannot reach head.number",
        ),
        (
            quote! { mod bad { category A(number); table join(number) -> person { (Singular) => Third } construction A: A { form [head: lexical(Noun)]; export number = join(head.number); } } },
            "same domain",
        ),
    ] {
        rejects(input, expected);
    }
}

#[test]
fn table_requirements_validate_calls_constants_and_policy_references() {
    let table = quote! {
        feature Verdict { Yes, No }
        table selected(number, person) -> Verdict { (Singular, Third) => Yes }
    };
    for (equation, expected) in [
        (
            quote!(require unknown(head.number) = Yes;),
            "unknown feature table",
        ),
        (
            quote!(require selected(head.number) = Yes;),
            "call has wrong arity",
        ),
        (
            quote!(require selected(head.person, head.number) = Yes;),
            "argument has wrong domain",
        ),
        (
            quote!(require selected(head.number, head.person) = Singular;),
            "not a value of feature Verdict",
        ),
        (
            quote!(require selected(missing.number, head.person) = Yes;),
            "cannot reach field missing",
        ),
    ] {
        rejects(
            quote! {
                mod bad { #table category A(); construction A: A {
                    form [head: lexical(Pronoun)]; #equation
                } }
            },
            expected,
        );
    }
    rejects(
        quote! {
            mod bad { #table category A(); category B();
                construction A: A { form [head: B]; require selected(head.number, head.person) = Yes; }
                construction B: B { form []; }
            }
        },
        "cannot reach head.number",
    );
    rejects(
        quote! {
            mod bad { #table category A();
                construction A: A { form [head: optional(A)]; require selected(head.number, head.person) = Yes; }
            }
        },
        "optional/repeated fields have no single feature value",
    );
    rejects(
        quote! {
            mod bad { #table category A();
                policy Selection<Other> { require selected(Other.number, absent.person) = Yes; }
                construction A: A { form [head: lexical(Pronoun)]; use Selection(head); }
            }
        },
        "policy reference must name a caller field",
    );
}

#[test]
fn rejects_zero_consumption_cycles_including_normalized_repetitions() {
    rejects(
        quote! { mod bad { category A(); construction Cycle: A { form [child: A]; } construction Empty: A { form []; } } },
        "packing obligation",
    );
    rejects(
        quote! { mod bad { category A(); category B(); construction Left: A { form [child: B]; } construction Right: B { form [child: optional(A)]; } } },
        "packing obligation",
    );
    rejects(
        quote! { mod bad { category A(); category B(); construction Empty: A { form []; } construction List: B { form [items: repeat(A, "")]; } } },
        "packing obligation",
    );
    compile(quote! { mod safe { category A(); construction Nested: A { form ["(", child: A, ")"]; } construction Empty: A { form []; } } }).unwrap();
}

#[test]
fn declaration_errors_are_compile_errors_not_proc_macro_panics() {
    for input in [
        quote! { mod bad { category A(); construction Broken: A { render ["different"]; } } },
        quote! { mod bad { category A(); construction Broken: A { form [word: lexical(Imagined)]; } } },
        quote! { mod bad { feature number { One } } },
        quote! { mod bad { category A(); frame Wrong = "(unknown: 3)"; } },
        quote! { mod bad { category A(); construction Broken: A { form [word: lexical(Noun, extra)]; } } },
    ] {
        let expansion =
            std::panic::catch_unwind(|| crate::expand(input)).expect("diagnostics must not panic");
        assert!(expansion.to_string().contains("compile_error"));
    }
}

#[test]
fn rejects_duplicate_and_invalid_construction_costs() {
    rejects(
        quote! { mod bad { category A(); construction A: A { cost 1; cost 2; form []; } } },
        "duplicate construction cost",
    );
    for cost in [quote!(-1), quote!(1.5), quote!(18446744073709551616)] {
        assert!(
            compile(
                quote! { mod bad { category A(); construction A: A { cost #cost; form []; } } }
            )
            .is_err()
        );
    }
}

#[test]
fn schemas_reject_ambiguous_instances_and_invalid_field_bindings() {
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node, " and ", right: node]; }
            instance Pair: A { bind left = A; bind right = A; }
            instance Pair: A { bind left = A; bind right = A; }
        } },
        "unique result category",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair: A { }
        } },
        "exactly one category binding",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair: A { bind left = A; bind left = A; }
        } },
        "exactly one category binding",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair: A { bind left = A; bind unknown = A; }
        } },
        "generic node field",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair: A { bind left = A; form [left: A]; }
        } },
        "owned by schema",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            construction Pair: A { form [head: lexical(Noun)]; }
            instance Pair: A { bind left = A; }
        } },
        "conflicts with plain construction",
    );
}

#[test]
fn policies_remain_checked_feature_equations() {
    rejects(
        quote! { mod bad {
            category A();
            construction Leaf: A { form [head: lexical(Noun)]; use Missing; }
        } },
        "unknown policy",
    );
    rejects(
        quote! { mod bad {
            category A();
            policy Bad { form [head: lexical(Noun)]; }
            construction Leaf: A { form [head: lexical(Noun)]; }
        } },
        "only feature equations",
    );
    rejects(
        quote! { mod bad {
            category A();
            policy Wrong { require head.number = Singular; }
            schema Leaf { form [head: node]; use Wrong; }
            instance Leaf: A { bind head = A; }
        } },
        "cannot reach head.number",
    );
}

#[test]
fn category_rows_require_unambiguous_checked_substitution() {
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member>: [(A)] { bind left = Member; }
        } },
        "arity must match",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Result>: [(A,A)] { bind left = Result; }
        } },
        "duplicate category parameter",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member>: [(A,A),(A,A)] { bind left = Member; }
        } },
        "unique result category",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member>: [] { bind left = Member; }
        } },
        "at least one row",
    );
}

#[test]
fn policy_row_columns_have_disjoint_checked_roles() {
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member>: [(A,A)] { bind left = Member; use Member; }
        } },
        "exactly one category or policy role",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member, Rule>: [(A,A,Missing)] { bind left = Member; use Rule; }
        } },
        "unknown policy",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member, Unused>: [(A,A,A)] { bind left = Member; }
        } },
        "exactly one category or policy role",
    );
}

#[test]
fn self_rows_alias_only_a_concrete_result_in_category_columns() {
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member>: [(Self,A)] { bind left = Member; }
        } },
        "result category cannot be Self",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member, Rule>: [(A,Self,Self)] { bind left = Member; use Rule; }
        } },
        "requires a category column",
    );
}

#[test]
fn explicit_row_defaults_keep_typed_checked_prefixes() {
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member=Self, Other>: [(A,A,A)] { bind left = Member; use Other; }
        } },
        "only trailing",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member=Self, Rule=Missing>: [(A)] { bind left = Member; use Rule; }
        } },
        "unknown policy",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member=Self, Rule=Self>: [(A)] { bind left = Member; use Rule; }
        } },
        "requires a category column",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema Pair { form [left: node]; }
            instance Pair<Result, Member=Missing>: [(A,A)] { bind left = Member; }
        } },
        "default row category must be declared",
    );
}

#[test]
fn policy_field_arguments_are_bounded_and_checked_at_every_use() {
    rejects(
        quote! { mod bad {
            category A();
            policy Empty<Field, Field> {}
            construction Leaf: A { form [head: lexical(Noun)]; }
        } },
        "duplicate policy field parameter",
    );
    rejects(
        quote! { mod bad {
            category A();
            policy Empty<Field> {}
            construction Leaf: A { form [head: lexical(Noun)]; use Empty; }
        } },
        "arity must match",
    );
    rejects(
        quote! { mod bad {
            category A();
            policy Empty<Field> {}
            construction Leaf: A { form [head: lexical(Noun)]; use Empty(missing); }
        } },
        "argument must name a caller field",
    );
    rejects(
        quote! { mod bad {
            category A();
            policy Free { require missing.number = Singular; }
            construction Leaf: A { form [head: lexical(Noun)]; use Free; }
        } },
        "reference must name a caller field",
    );
    rejects(
        quote! { mod bad {
            category A();
            policy Recursive<Field> { use Recursive(Field); }
            construction Leaf: A { form [head: lexical(Noun)]; }
        } },
        "only feature equations",
    );
    rejects(
        quote! { mod bad {
            category A();
            policy Empty {}
            policy Argument<Field> {}
            schema Leaf { form [head: lexical(Noun)]; }
            instance Leaf<Result, Rule=Empty>: [(A, Argument)] { use Rule(head); }
        } },
        "default field argument arity",
    );
}

#[test]
fn field_policy_expansion_is_identical_to_authored_equations_for_right_and_rest() {
    for name in ["right", "rest"] {
        let other = quote::format_ident!("{name}");
        let header = quote! {
            category Nominal(number);
            table echo(number) -> number { (Singular) => Singular, (Plural) => Plural }
            construction Noun: Nominal {
                form [head: lexical(Noun)];
                export number = head.number;
            }
        };
        let plain = compile(quote! { mod same {
            #header
            construction Pair: Nominal {
                form [left: Nominal, " and ", #other: Nominal];
                agree left.number = #other.number;
                require #other.number = Singular;
                export number = echo(#other.number);
            }
        } })
        .unwrap();
        let reused = compile(quote! { mod same {
            #header
            policy Concord<Other> {
                agree left.number = Other.number;
                require Other.number = Singular;
                export number = echo(Other.number);
            }
            construction Pair: Nominal {
                form [left: Nominal, " and ", #other: Nominal];
                use Concord(#other);
            }
        } })
        .unwrap();
        assert_eq!(plain.to_string(), reused.to_string());
        let two_parameters = compile(quote! { mod same {
            #header
            policy Concord<Other, Source> {
                agree Source.number = Other.number;
                require Other.number = Singular;
                export number = echo(Other.number);
            }
            construction Pair: Nominal {
                form [left: Nominal, " and ", #other: Nominal];
                use Concord(#other, left);
            }
        } })
        .unwrap();
        assert_eq!(plain.to_string(), two_parameters.to_string());
    }
}

#[test]
fn header_category_bindings_emit_exactly_the_existing_explicit_bind_contracts() {
    let shared = quote! {
        category A(number);
        category B(number);
        construction Atom: A {
            form [head: lexical(Noun)]; export number = head.number;
        }
        policy Concord<Left, Right> {
            agree Left.number = Right.number; export number = Left.number;
        }
        schema Pair {
            cost 7;
            boundary Initial;
            form [left: node, " and ", right: node];
        }
        schema Container {
            form ["[", child: optional(node), ":", children: repeat(node, "; "), "]"];
            export number = Singular;
        }
    };
    let explicit = compile(quote! { mod same {
        #shared
        instance Pair<Result, Member=A, Rule=Concord>: [(A), (B, A)] {
            bind left, right = Member;
            use Rule(left, right);
        }
        instance Container<Result, Member>: [(A, A), (B, A)] {
            bind child, children = Member;
        }
    } })
    .unwrap();
    let bound = compile(quote! { mod same {
        #shared
        instance Pair<Result, Member: (left, right)=A, Rule=Concord>: [(A), (B, A)] {
            use Rule(left, right);
        }
        instance Container<Result, Child: child, Children: children>: [(A, A, A), (B, A, A)] {}
    } })
    .unwrap();
    assert_eq!(explicit.to_string(), bound.to_string());
}

#[test]
fn header_category_bindings_reject_invalid_targets_and_policy_roles() {
    rejects(
        quote! { mod bad {
            category A();
            schema One { form [head: node]; }
            instance One<Result, Child: head>: [(A, A)] { bind head = A; }
        } },
        "exactly one category binding",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema One { form [head: node]; }
            instance One<Result, Child: (head, head)>: [(A, A)] {}
        } },
        "exactly one category binding",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema One { form [head: node]; }
            instance One<Result, Child: unknown>: [(A, A)] { bind head = A; }
        } },
        "binding must name a generic node field",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema One { form [head: lexical(Noun)]; }
            instance One<Result, Child: head>: [(A, A)] {}
        } },
        "binding must name a generic node field",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema One { form [head: A]; }
            instance One<Result, Child: head>: [(A, A)] {}
        } },
        "binding must name a generic node field",
    );
    rejects(
        quote! { mod bad {
            category A();
            policy Noop {}
            schema One { form [head: node]; }
            instance One<Result, Rule: head>: [(A, Noop)] { use Rule; }
        } },
        "exactly one category or policy role",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema One { form [head: node]; }
            instance One<Result, Child: ()>: [(A, A)] {}
        } },
        "needs a field target",
    );
    rejects(
        quote! { mod bad {
            category A();
            schema One { form [head: node]; }
            instance One<Result, Child: head>: [(A, Missing)] {}
        } },
        "unknown constituent Category",
    );
}

#[test]
fn frame_segment_relations_validate_fields_order_and_schema_policies() {
    compile(quote! {
        mod valid {
            category Segment();
            category Predicate();
            policy Sharing<Right> { share_segments left, Right; }
            construction Segment: Segment {
                form [object: Predicate, " ", marker: lexical(Preposition), " ", complement: Predicate];
                segment Predicate;
            }
            construction Pair: Segment {
                form [left: Segment, " and ", right: Segment];
                use Sharing(right);
            }
            construction Host: Predicate {
                form [head: lexical(Verb), " ", tail: Segment];
                discharge_segments head, tail;
            }
        }
    }).unwrap();
    for (relation, expected) in [
        (quote!(share_segments right, left;), "distinct and ordered"),
        (quote!(share_segments left, left;), "distinct and ordered"),
        (quote!(share_segments left, absent;), "must name a field"),
        (quote!(discharge_segments left, right;), "lexical Verb head"),
        (
            quote!(share_segments left, right; share_segments left, right;),
            "duplicate segment relation",
        ),
    ] {
        rejects(
            quote! {
                mod invalid {
                    category Node();
                    construction Node: Node { form [left: Node, " ", right: Node]; #relation }
                }
            },
            expected,
        );
    }
}
