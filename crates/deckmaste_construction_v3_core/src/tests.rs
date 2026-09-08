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
