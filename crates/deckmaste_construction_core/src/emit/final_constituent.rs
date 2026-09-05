use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::RIGHTMOST_LEAF_CATEGORY_TRAIT;
use crate::identifier::RIGHTMOST_LEAF_IS_FUNCTION;
use crate::identifier::RIGHTMOST_LEAF_TRAIT;
use crate::identifier::emitted_ident;
use crate::plan::DeclarationKey;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::plan::SourceDeclarationKind;
use crate::semantic::AtomPlan;
use crate::semantic::ConstructionFieldKind;
use crate::semantic::ConstructionPlan;
use crate::semantic::FiniteDomainKindPlan;
use crate::semantic::FiniteValuePlan;
use crate::semantic::FormPlan;
use crate::semantic::SemanticPlan;
use crate::semantic::StructuralFieldKindPlan;
use crate::semantic::StructuralFieldPlan;
use crate::semantic::ValueKindPlan;

use super::SemanticTypeKind;

pub(crate) fn emit(plan: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let traversal_trait = ident(RIGHTMOST_LEAF_TRAIT);
    let category_trait = ident(RIGHTMOST_LEAF_CATEGORY_TRAIT);
    let predicate = ident(RIGHTMOST_LEAF_IS_FUNCTION);
    let origins = semantic_origins(plan);
    let mut items = vec![
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Trait,
                name: RIGHTMOST_LEAF_TRAIT.to_owned(),
            },
            quote! {
                pub(crate) trait #traversal_trait {
                    fn rightmost_leaf_is(&self, target: Category) -> bool;
                }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Trait,
                name: RIGHTMOST_LEAF_CATEGORY_TRAIT.to_owned(),
            },
            quote! {
                pub(crate) trait #category_trait {
                    const CATEGORY: Category;
                }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: RIGHTMOST_LEAF_IS_FUNCTION.to_owned(),
            },
            quote! {
                pub(crate) fn #predicate<T: #category_trait>(
                    value: &impl #traversal_trait,
                ) -> bool {
                    value.rightmost_leaf_is(T::CATEGORY)
                }
            },
            origins,
        ),
    ];

    for semantic_type in super::semantic_types(plan) {
        let type_name = semantic_type.name;
        let ty = ident(type_name);
        let category = ident(type_name);
        let origin = semantic_type_origin(plan, semantic_type.kind, type_name)?;
        items.push(GeneratedItem::new(
            ItemKey::Impl {
                trait_name: Some(RIGHTMOST_LEAF_CATEGORY_TRAIT.to_owned()),
                self_ty: type_name.to_owned(),
            },
            quote! {
                impl #category_trait for #ty {
                    const CATEGORY: Category = Category::#category;
                }
            },
            vec![origin.clone()],
        ));
        let body = match semantic_type.kind {
            SemanticTypeKind::Category => emit_category_body(plan, type_name)?,
            SemanticTypeKind::Product => emit_product_body(plan, type_name)?,
            SemanticTypeKind::Sum => emit_sum_body(plan, type_name)?,
        };
        items.push(GeneratedItem::new(
            ItemKey::Impl {
                trait_name: Some(RIGHTMOST_LEAF_TRAIT.to_owned()),
                self_ty: type_name.to_owned(),
            },
            quote! {
                impl #traversal_trait for #ty {
                    fn rightmost_leaf_is(&self, target: Category) -> bool {
                        #body
                    }
                }
            },
            vec![origin],
        ));
    }

    Ok(items)
}

fn emit_category_body(plan: &SemanticPlan, category: &str) -> syn::Result<TokenStream> {
    let arms = plan
        .constructions()
        .iter()
        .filter(|construction| construction.category() == category)
        .map(|construction| {
            let variant = ident(construction.category_variant());
            let body = emit_construction_body(plan, construction, &quote! { value })?;
            Ok(quote! { Self::#variant(value) => { #body } })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! { match self { #(#arms,)* } })
}

fn emit_product_body(plan: &SemanticPlan, product_name: &str) -> syn::Result<TokenStream> {
    if let Some(construction) = plan
        .constructions()
        .iter()
        .find(|construction| construction.element_type() == product_name)
    {
        return emit_construction_body(plan, construction, &quote! { self });
    }
    let product = plan
        .products()
        .iter()
        .find(|product| product.name() == product_name)
        .ok_or_else(|| internal("semantic product has no sealed source"))?;
    let mut expression = quote! { false };
    for field in product.fields() {
        expression = structural_field_expression(field, &quote! { self }, &expression);
    }
    Ok(expression)
}

fn emit_sum_body(plan: &SemanticPlan, sum_name: &str) -> syn::Result<TokenStream> {
    let sum = plan
        .sums()
        .iter()
        .find(|sum| sum.name() == sum_name)
        .ok_or_else(|| internal("semantic sum has no sealed source"))?;
    let arms = sum.alternatives().iter().map(|alternative| {
        let variant = ident(alternative.name());
        let expression = value_expression(alternative.value(), &quote! { value });
        quote! { Self::#variant(value) => #expression }
    });
    Ok(quote! { match self { #(#arms,)* } })
}

fn emit_construction_body(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    value: &TokenStream,
) -> syn::Result<TokenStream> {
    let has_feature_guard = construction.forms().iter().any(|form| {
        form.guard().predicate().is_some_and(|predicate| {
            predicate
                .domains()
                .iter()
                .any(|domain| matches!(domain.kind(), FiniteDomainKindPlan::Feature { .. }))
        })
    });
    if has_feature_guard {
        let form = construction
            .forms()
            .first()
            .ok_or_else(|| internal("construction has no canonical form"))?;
        return emit_form_expression(plan, construction, form, value);
    }

    let forms = construction
        .forms()
        .iter()
        .enumerate()
        .map(|(form_index, form)| {
            let expression = emit_form_expression(plan, construction, form, value)?;
            let guard =
                super::emit_form_guard_expression(construction, form_index, |domain, expected| {
                    guard_expression(construction, value, domain, expected)
                })?;
            Ok((guard, expression))
        })
        .collect::<syn::Result<Vec<_>>>()?;
    if forms.len() == 1 {
        return forms
            .into_iter()
            .next()
            .map(|(_, expression)| expression)
            .ok_or_else(|| internal("construction has no form"));
    }
    let branches = forms.iter().map(|(guard, expression)| {
        let guard = guard.as_ref().expect("multi-form rows are guarded");
        quote! { if #guard { #expression } else }
    });
    Ok(quote! {
        #(#branches)* { unreachable!("sealed form partition is total") }
    })
}

fn emit_form_expression(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    form: &FormPlan,
    value: &TokenStream,
) -> syn::Result<TokenStream> {
    let mut expression = quote! { false };
    for atom in form.atoms() {
        expression = atom_expression(plan, construction, atom.value_atom(), value, &expression)?;
    }
    Ok(expression)
}

fn atom_expression(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    atom: &AtomPlan,
    value: &TokenStream,
    fallback: &TokenStream,
) -> syn::Result<TokenStream> {
    let (AtomPlan::Category { role, .. } | AtomPlan::Marked { role, .. }) = atom else {
        return Ok(match atom {
            AtomPlan::Lex { role, .. }
            | AtomPlan::Identity { role, .. }
            | AtomPlan::Noun { role, .. } => {
                terminal_field_expression(construction.field(role)?, value, fallback)
            }
            AtomPlan::Literal(_)
            | AtomPlan::SentenceInitialLiteral(_)
            | AtomPlan::StructuralLiteral(_)
            | AtomPlan::LexFixed { .. }
            | AtomPlan::VerbFixed { .. }
            | AtomPlan::OpenDeclaration(_) => quote! { false },
            // Bound and Circumfix are transparent delimiters: this fold finds
            // the rightmost non-delimiter constituent. `quoted_ability` and
            // both quote-terminator `checked by` sites depend on that contract.
            AtomPlan::Bound { .. } | AtomPlan::Circumfix { .. } => {
                unreachable!("value_atom removes wrappers")
            }
            AtomPlan::Category { .. } | AtomPlan::Marked { .. } => {
                unreachable!("handled above")
            }
        });
    };
    let field = construction.field(role)?;
    if field.kind() != ConstructionFieldKind::Category {
        return Err(internal("rightmost category atom has a non-category field"));
    }
    if let Some(structural) = field.structural_plan() {
        return Ok(construction_structural_field_expression(
            field, structural, value, fallback,
        ));
    }
    let field_name = field.name();
    let child = if plan
        .boxed_fields()
        .contains(&(construction.construction_id().to_owned(), field.name_key()))
    {
        quote! { #value.#field_name.as_ref() }
    } else {
        quote! { &#value.#field_name }
    };
    Ok(value_expression(
        &ValueKindPlan::Category(field.terminal().to_owned()),
        &child,
    ))
}

fn terminal_field_expression(
    field: &crate::semantic::ConstructionFieldPlan,
    value: &TokenStream,
    fallback: &TokenStream,
) -> TokenStream {
    let Some(structural) = field.structural_plan() else {
        return quote! { false };
    };
    let field_name = field.name();
    match structural.kind() {
        StructuralFieldKindPlan::Required(_) => quote! { false },
        StructuralFieldKindPlan::Zeroable(_) => {
            let ty = field.value_type();
            quote! {
                match &#value.#field_name {
                    #ty::Headed(_) => false,
                    #ty::Zero => #fallback,
                }
            }
        }
        StructuralFieldKindPlan::Optional(_) => {
            quote! { if #value.#field_name.is_some() { false } else { #fallback } }
        }
        StructuralFieldKindPlan::Sequence { .. } => {
            quote! { if #value.#field_name.is_empty() { #fallback } else { false } }
        }
    }
}

fn construction_structural_field_expression(
    field: &crate::semantic::ConstructionFieldPlan,
    structural: &StructuralFieldPlan,
    value: &TokenStream,
    fallback: &TokenStream,
) -> TokenStream {
    let field_name = field.name();
    match structural.kind() {
        StructuralFieldKindPlan::Required(kind) => {
            let child = if structural.is_recursive() {
                quote! { #value.#field_name.as_ref() }
            } else {
                quote! { &#value.#field_name }
            };
            value_expression(kind, &child)
        }
        StructuralFieldKindPlan::Zeroable(kind) => {
            let ty = field.value_type();
            let present = value_expression(kind, &quote! { child });
            quote! {
                match &#value.#field_name {
                    #ty::Headed(child) => #present,
                    #ty::Zero => #fallback,
                }
            }
        }
        StructuralFieldKindPlan::Optional(kind) => {
            let optional = if structural.is_recursive() {
                quote! { #value.#field_name.as_ref().as_ref() }
            } else {
                quote! { #value.#field_name.as_ref() }
            };
            let present = value_expression(kind, &quote! { child });
            quote! {
                match #optional {
                    Some(child) => #present,
                    None => #fallback,
                }
            }
        }
        StructuralFieldKindPlan::Sequence { item, .. } => {
            let sequence = if structural.is_recursive() {
                quote! { #value.#field_name.as_ref() }
            } else {
                quote! { &#value.#field_name }
            };
            let present = value_expression(item, &quote! { child });
            quote! {
                match #sequence.last() {
                    Some(child) => #present,
                    None => #fallback,
                }
            }
        }
    }
}

fn structural_field_expression(
    field: &StructuralFieldPlan,
    value: &TokenStream,
    fallback: &TokenStream,
) -> TokenStream {
    let field_name = ident(field.name());
    match field.kind() {
        StructuralFieldKindPlan::Required(kind) => {
            let child = if field.is_recursive() {
                quote! { #value.#field_name.as_ref() }
            } else {
                quote! { &#value.#field_name }
            };
            value_expression(kind, &child)
        }
        StructuralFieldKindPlan::Zeroable(kind) | StructuralFieldKindPlan::Optional(kind) => {
            let optional = if field.is_recursive() {
                quote! { #value.#field_name.as_ref().as_ref() }
            } else {
                quote! { #value.#field_name.as_ref() }
            };
            let present = value_expression(kind, &quote! { child });
            quote! {
                match #optional {
                    Some(child) => #present,
                    None => #fallback,
                }
            }
        }
        StructuralFieldKindPlan::Sequence { item, .. } => {
            let sequence = if field.is_recursive() {
                quote! { #value.#field_name.as_ref() }
            } else {
                quote! { &#value.#field_name }
            };
            let present = value_expression(item, &quote! { child });
            quote! {
                match #sequence.last() {
                    Some(child) => #present,
                    None => #fallback,
                }
            }
        }
    }
}

fn value_expression(kind: &ValueKindPlan, child: &TokenStream) -> TokenStream {
    match kind {
        ValueKindPlan::Category(name) | ValueKindPlan::Product(name) | ValueKindPlan::Sum(name) => {
            let ty = ident(name);
            let category = ident(name);
            quote! {
                target == Category::#category
                    || <#ty as RightmostLeaf>::rightmost_leaf_is(#child, target)
            }
        }
        ValueKindPlan::Lex(_) | ValueKindPlan::Identity(_) => quote! { false },
    }
}

fn guard_expression(
    construction: &ConstructionPlan,
    value: &TokenStream,
    domain: &crate::semantic::FiniteDomainPlan,
    expected: &FiniteValuePlan,
) -> syn::Result<TokenStream> {
    let field = construction.field(domain.role())?;
    let name = field.name();
    match (domain.kind(), expected) {
        (FiniteDomainKindPlan::Vocab { terminal, .. }, FiniteValuePlan::Vocab(variant)) => {
            let terminal = ident(terminal);
            let variant = ident(variant);
            Ok(quote! { matches!(#value.#name, #terminal::#variant) })
        }
        (
            FiniteDomainKindPlan::OptionalVocab { terminal, .. },
            FiniteValuePlan::OptionalVocab(variant),
        ) => Ok(variant.as_ref().map_or_else(
            || quote! { #value.#name.is_none() },
            |variant| {
                let terminal = ident(terminal);
                let variant = ident(variant);
                quote! { matches!(#value.#name, Some(#terminal::#variant)) }
            },
        )),
        (FiniteDomainKindPlan::OptionalPresence, FiniteValuePlan::OptionalPresence(present)) => {
            Ok(quote! { #value.#name.is_some() == #present })
        }
        (FiniteDomainKindPlan::Feature { .. }, FiniteValuePlan::Feature(_)) => {
            Err(internal("feature guard escaped canonical rightmost form"))
        }
        _ => Err(internal(
            "rightmost form guard domain and assignment value disagree",
        )),
    }
}

fn semantic_origins(plan: &SemanticPlan) -> Vec<DeclarationKey> {
    plan.declaration_keys()
        .iter()
        .filter(|origin| {
            matches!(
                origin.kind(),
                SourceDeclarationKind::Construction
                    | SourceDeclarationKind::AbstractProduct
                    | SourceDeclarationKind::AbstractSum
            )
        })
        .cloned()
        .collect()
}

fn semantic_type_origin(
    plan: &SemanticPlan,
    kind: SemanticTypeKind,
    name: &str,
) -> syn::Result<DeclarationKey> {
    match kind {
        SemanticTypeKind::Category => plan
            .constructions()
            .iter()
            .find(|construction| construction.category() == name)
            .map(|construction| {
                DeclarationKey::new(
                    SourceDeclarationKind::Construction,
                    construction.construction_id(),
                )
            }),
        SemanticTypeKind::Product => plan
            .constructions()
            .iter()
            .find(|construction| construction.element_type() == name)
            .map(|construction| {
                DeclarationKey::new(
                    SourceDeclarationKind::Construction,
                    construction.construction_id(),
                )
            })
            .or_else(|| {
                plan.products()
                    .iter()
                    .find(|product| product.name() == name)
                    .map(|_| DeclarationKey::new(SourceDeclarationKind::AbstractProduct, name))
            }),
        SemanticTypeKind::Sum => plan
            .sums()
            .iter()
            .find(|sum| sum.name() == name)
            .map(|_| DeclarationKey::new(SourceDeclarationKind::AbstractSum, name)),
    }
    .ok_or_else(|| internal("semantic type has no declaration origin"))
}

fn ident(name: &str) -> syn::Ident {
    emitted_ident(name, Span::call_site())
}

fn internal(message: &str) -> syn::Error {
    syn::Error::new(Span::call_site(), message)
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    fn fixture() -> SemanticPlan {
        crate::validate_declarations(
            crate::parse_declarations(quote! {
                vocab Word { Plain = "plain", }

                construction quoted: QuotedBlock {
                    element QuotedValue {}
                    form quoted = "quoted";
                }
                construction plain: Plain {
                    element PlainValue { word: lex Word, }
                    form plain = lex(word);
                }
                abstract sum Ending {
                    Quote: QuotedBlock,
                    Ordinary: Plain,
                }
                abstract product EndingSequence {
                    values: seq Ending separated by " ",
                }
                construction direct: Sentence {
                    element DirectSentence { ending: Ending, }
                    form direct = suffix(ending, "!");
                }
                construction optional: Sentence {
                    element OptionalSentence {
                        prefix: Plain,
                        ending: opt QuotedBlock,
                    }
                    form optional = prefix ending;
                }
                construction sequence: Sentence {
                    element SequenceSentence { endings: EndingSequence, }
                    form sequence = endings;
                }
                construction literal_final: LiteralEnded {
                    element LiteralEndedValue { ending: Ending, }
                    form literal_final = ending "instead";
                }

                root Sentence { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("rightmost-leaf fixture parses"),
        )
        .expect("rightmost-leaf fixture validates")
        .into_semantic()
    }

    fn traversal_implementation(items: &[GeneratedItem], ty: &str) -> String {
        items
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    ItemKey::Impl {
                        trait_name: Some(trait_name),
                        self_ty,
                    } if trait_name == RIGHTMOST_LEAF_TRAIT && self_ty == ty
                )
            })
            .unwrap_or_else(|| panic!("rightmost-leaf implementation for {ty}"))
            .tokens
            .to_string()
    }

    #[test]
    fn constructions_derive_their_rightmost_category_from_their_forms() {
        let items = emit(&fixture()).expect("rightmost-leaf fixture emits");
        let sentence = traversal_implementation(&items, "Sentence");

        assert!(
            sentence.contains("target == Category :: Ending"),
            "the future direct construction contributes without a handwritten walker: {sentence}",
        );
        assert!(
            sentence.contains("target == Category :: QuotedBlock"),
            "the optional final construction checks its present category: {sentence}",
        );
        assert!(
            sentence.contains("Some (child)") && sentence.contains("None =>"),
            "the optional final construction falls back to its prior constituent: {sentence}",
        );
        assert!(
            sentence.contains("target == Category :: EndingSequence"),
            "a structural product remains on the derived rightmost spine: {sentence}",
        );
    }

    #[test]
    fn a_trailing_literal_is_an_opaque_rightmost_leaf() {
        let items = emit(&fixture()).expect("rightmost-leaf fixture emits");
        let literal_ended = traversal_implementation(&items, "LiteralEnded");

        assert!(
            literal_ended.contains("Self :: LiteralFinal (value) => { false }"),
            "a form literal is a leaf rather than a transparent wrapper: {literal_ended}",
        );
    }

    #[test]
    fn sums_products_and_sequences_recurse_through_the_last_present_value() {
        let items = emit(&fixture()).expect("rightmost-leaf fixture emits");
        let sum = traversal_implementation(&items, "Ending");
        let product = traversal_implementation(&items, "EndingSequence");

        assert!(sum.contains("Self :: Quote"), "sum dispatch: {sum}");
        assert!(
            sum.contains("target == Category :: QuotedBlock"),
            "sum target category: {sum}",
        );
        assert!(
            sum.contains("Self :: Ordinary"),
            "every future sum alternative is exhaustive: {sum}",
        );
        assert!(
            product.contains("values . last"),
            "sequences select their rightmost member: {product}",
        );
        assert!(
            product.contains("None => false"),
            "empty sequences fail closed: {product}",
        );
    }

    #[test]
    fn mobile_role_metadata_does_not_become_a_rightmost_leaf() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote! {
                construction child: Child {
                    element ChildNode {}
                    form child = "child";
                }
                construction mobile: Root {
                    element MobileHost { tail: mobile Child, }
                    form mobile = tail;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("mobile rightmost-leaf fixture parses"),
        )
        .expect("mobile rightmost-leaf fixture validates")
        .into_semantic();
        let items = emit(&plan).expect("mobile rightmost-leaf fixture emits");
        let root = traversal_implementation(&items, "Root");
        assert!(root.contains("target == Category :: Child"), "{root}");
        assert!(!root.contains("AdmissibleSites"), "{root}");
    }
}
