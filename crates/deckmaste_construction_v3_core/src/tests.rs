use quote::quote;

use crate::compile;

fn rejects(input: proc_macro2::TokenStream, expected: &str) {
    let error = compile(input)
        .expect_err("invalid declarations must fail before generated Rust is compiled");
    assert!(error.to_string().contains(expected), "{error}");
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
