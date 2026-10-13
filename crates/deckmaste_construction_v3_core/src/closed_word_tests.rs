use crate::{compile, parse};
use quote::quote;

fn declaration(body: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! { mod g {
        closed_word To: Subordinator { id "old:to"; lemma "to"; }
        lexical_class Marker: Subordinator | Preposition;
        category A();
        #body
    } }
}

#[test]
fn selectors_keep_field_storage_and_form_indices() {
    let input = declaration(&quote! {
        construction A: A {
            form [marker: lexical_word(To)];
            form [marker: lexical(Preposition)];
        }
    });
    compile(input.clone()).unwrap();
    let parsed: parse::Declaration = syn::parse2(input).unwrap();
    let construction = &parsed.constructions[0];
    assert_eq!(construction.forms.len(), 2);
    for (form, category) in construction
        .forms
        .iter()
        .zip(["Subordinator", "Preposition"])
    {
        let parse::Part::Field(name, kind) = &form[0] else { panic!() };
        assert_eq!(name, "marker");
        assert_eq!(kind, &parse::FieldType::Lexical(category.into()));
    }
    assert_eq!(construction.form_requirements[0].len(), 1);
    assert!(construction.form_requirements[1].is_empty());
}

#[test]
fn policy_word_and_class_requirements_bind_caller_fields() {
    compile(declaration(&quote! {
        policy Licence<x> { require x is To; require x in Marker; }
        construction A: A { form [head: lexical(Subordinator)]; use Licence(head); }
    }))
    .unwrap();
}

#[test]
fn mixed_category_class_preserves_separate_forms() {
    compile(declaration(&quote! {
        construction A: A {
            form [head: lexical(Subordinator)];
            form [head: lexical(Preposition)];
            require head in Marker;
        }
    }))
    .unwrap();
}

#[test]
fn optional_lexical_requirements_use_present_word_checks() {
    compile(declaration(&quote! {
        construction A: A {
            form [head: optional(lexical(Subordinator), " ")];
            require head is To;
            require head in Marker;
        }
    }))
    .unwrap();
}

#[test]
fn undeclared_words_and_classes_are_rejected() {
    for body in [
        quote!(construction A: A { form [head: lexical_word(Missing)]; }),
        quote!(construction A: A { form [head: lexical(Subordinator)]; require head is Missing; }),
        quote!(construction A: A { form [head: lexical(Subordinator)]; require head in Missing; }),
    ] {
        assert!(
            compile(declaration(&body))
                .unwrap_err()
                .to_string()
                .contains("unknown")
        );
    }
}

#[test]
fn lexical_constraints_reject_constituents_and_wrong_categories() {
    for body in [
        quote!(construction A: A { form [head: A]; require head is To; }),
        quote!(construction A: A { form [head: lexical(Noun)]; require head in Marker; }),
        quote!(construction A: A { form [head: lexical(Preposition)]; require head is To; }),
    ] {
        assert!(compile(declaration(&body)).is_err());
    }
}

#[test]
fn declarations_reject_duplicate_ids_unknown_classes_and_open_nouns() {
    for extra in [
        quote!(closed_word Other: Subordinator { id "old:to"; lemma "other"; }),
        quote!(closed_word Other: Subordinator { id "other"; lemma "other"; classes [Missing]; }),
        quote!(closed_word Other: Noun { id "other"; lemma "other"; }),
        quote!(lexical_class Other: Missing;),
    ] {
        assert!(compile(declaration(&quote!(#extra construction A: A { form []; }))).is_err());
    }
}

#[test]
fn unused_registry_members_do_not_allocate_admission_slots() {
    let input = declaration(&quote! {
        closed_word By: Preposition { id "old:by"; lemma "by"; classes [Marker]; }
        construction A: A { form [head: lexical(Subordinator)]; require head in Marker; }
    });
    let ir = crate::ir::validate(syn::parse2(input.clone()).unwrap()).unwrap();
    assert_eq!(
        ir.features
            .iter()
            .filter(|f| f.name.starts_with("__"))
            .map(|f| f.name.as_str())
            .collect::<Vec<_>>(),
        ["__class_Marker"]
    );
    assert_eq!(ir.words.len(), 2);
    compile(input).unwrap();
}
