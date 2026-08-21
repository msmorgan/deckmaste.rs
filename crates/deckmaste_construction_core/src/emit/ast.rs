use std::collections::HashMap;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::emitted_ident;
use crate::identifier::feature_helper;
use crate::identifier::key as identifier_key;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::semantic::AccessorMode;
use crate::semantic::ConstructionFieldKind;
use crate::semantic::ConstructionPlan;
use crate::semantic::PredicateSubjectPlan;
use crate::semantic::SemanticPlan;

pub(crate) fn emit(plan: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let mut categories = Vec::<CategoryItem>::new();
    let mut category_indices = HashMap::<String, usize>::new();
    for construction in plan.constructions() {
        let category = construction.category().to_owned();
        let index = *category_indices.entry(category.clone()).or_insert_with(|| {
            let index = categories.len();
            categories.push(CategoryItem {
                name: category,
                span: construction.category_span(),
                variants: Vec::new(),
                origins: Vec::new(),
            });
            index
        });
        categories[index].variants.push((
            construction.category_variant().to_owned(),
            construction.element_type().to_owned(),
            construction.origin_span(),
        ));
        categories[index].origins.push(DeclarationKey::new(
            DeclarationKind::Construction,
            construction.construction_id(),
        ));
    }

    let mut items = categories
        .into_iter()
        .map(CategoryItem::finish)
        .collect::<Vec<_>>();
    let mut structural_items = plan
        .products()
        .iter()
        .map(|product| {
            let origin = DeclarationKey::new(DeclarationKind::AbstractProduct, product.name());
            let ident = emitted_ident(product.name(), Span::call_site());
            let mut emitted = vec![GeneratedItem::new(
                ItemKey::named_type(product.name()),
                emit_structural_product(product, &ident),
                vec![origin.clone()],
            )];
            if structural_product_requires_constructor(product) {
                emitted.push(GeneratedItem::new(
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty: product.name().to_owned(),
                    },
                    emit_structural_product_impl(product, &ident),
                    vec![origin],
                ));
            }
            (product.source_index(), emitted)
        })
        .chain(plan.sums().iter().map(|sum| {
            let ident = emitted_ident(sum.name(), Span::call_site());
            (
                sum.source_index(),
                vec![GeneratedItem::new(
                    ItemKey::named_type(sum.name()),
                    emit_structural_sum(sum, &ident),
                    vec![DeclarationKey::new(
                        DeclarationKind::AbstractSum,
                        sum.name(),
                    )],
                )],
            )
        }))
        .collect::<Vec<_>>();
    structural_items.sort_by_key(|(source_index, _)| *source_index);
    items.extend(
        structural_items
            .into_iter()
            .flat_map(|(_, emitted)| emitted),
    );
    for construction in plan.constructions() {
        let ident = emitted_ident(construction.element_type(), construction.origin_span());
        let origins = vec![DeclarationKey::new(
            DeclarationKind::Construction,
            construction.construction_id(),
        )];
        let tokens = emit_product(plan, construction, &ident)?;
        items.push(GeneratedItem::new(
            ItemKey::named_type(construction.element_type()),
            tokens,
            origins.clone(),
        ));
        if !construction.fields().is_empty() && construction_requires_constructor(construction) {
            items.push(GeneratedItem::new(
                ItemKey::Impl {
                    trait_name: None,
                    self_ty: construction.element_type().to_owned(),
                },
                emit_invariant_impl(plan, construction, &ident)?,
                origins,
            ));
        }
    }
    Ok(items)
}

fn emit_structural_sum(sum: &crate::semantic::SumPlan, ident: &syn::Ident) -> TokenStream {
    let alternatives = sum.alternatives().iter().map(|alternative| {
        let name = emitted_ident(alternative.name(), Span::call_site());
        let value = super::value_kind_type(alternative.value());
        if alternative.is_recursive() {
            quote! { #name(Box<#value>) }
        } else {
            quote! { #name(#value) }
        }
    });
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum #ident {
            #(#alternatives),*
        }
    }
}

fn emit_structural_product(
    product: &crate::semantic::ProductPlan,
    ident: &syn::Ident,
) -> TokenStream {
    if product.fields().is_empty() {
        return quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #ident;
        };
    }
    let fields = product.fields().iter().map(|field| {
        let name = emitted_ident(field.name(), Span::call_site());
        let visibility = (!structural_field_is_constrained(field)).then(|| quote! { pub });
        let ty = super::structural_field_type(field);
        quote! { #visibility #name: #ty }
    });
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct #ident {
            #(#fields),*
        }
    }
}

fn emit_structural_product_impl(
    product: &crate::semantic::ProductPlan,
    ident: &syn::Ident,
) -> TokenStream {
    let fields = product.fields().iter().map(|field| {
        let name = emitted_ident(field.name(), Span::call_site());
        let ty = super::structural_field_type(field);
        quote! { #name: #ty }
    });
    let initializers = product
        .fields()
        .iter()
        .map(|field| emitted_ident(field.name(), Span::call_site()));
    let guards = product.fields().iter().filter_map(|field| {
        let crate::semantic::StructuralFieldKindPlan::Sequence { bounds, .. } = field.kind() else {
            return None;
        };
        structural_bounds_are_constrained(*bounds).then(|| {
            let name = emitted_ident(field.name(), Span::call_site());
            emit_length_guard(&name, *bounds)
        })
    });
    let accessors = product
        .fields()
        .iter()
        .filter(|field| structural_field_is_constrained(field))
        .map(emit_structural_accessor);
    quote! {
        impl #ident {
            pub fn new(#(#fields),*) -> Option<Self> {
                if #(#guards)&&* {
                    Some(Self { #(#initializers),* })
                } else {
                    None
                }
            }

            #(#accessors)*
        }
    }
}

fn structural_product_requires_constructor(product: &crate::semantic::ProductPlan) -> bool {
    product.fields().iter().any(structural_field_is_constrained)
}

fn structural_field_is_constrained(field: &crate::semantic::StructuralFieldPlan) -> bool {
    matches!(
        field.kind(),
        crate::semantic::StructuralFieldKindPlan::Sequence { bounds, .. }
            if structural_bounds_are_constrained(*bounds)
    )
}

fn structural_bounds_are_constrained(bounds: crate::semantic::LengthBounds) -> bool {
    bounds.min() != 0 || bounds.max().is_some()
}

fn emit_length_guard(name: &syn::Ident, bounds: crate::semantic::LengthBounds) -> TokenStream {
    let min = syn::LitInt::new(&bounds.min().to_string(), Span::call_site());
    let lower = quote! { #name.len() >= #min };
    bounds.max().map_or(lower.clone(), |max| {
        let max = syn::LitInt::new(&max.to_string(), Span::call_site());
        quote! { (#lower) && (#name.len() <= #max) }
    })
}

fn emit_structural_accessor(field: &crate::semantic::StructuralFieldPlan) -> TokenStream {
    let name = emitted_ident(field.name(), Span::call_site());
    let value = super::value_kind_type(field.kind().value());
    match field.kind() {
        crate::semantic::StructuralFieldKindPlan::Required(_) if field.is_recursive() => quote! {
            pub fn #name(&self) -> &#value { self.#name.as_ref() }
        },
        crate::semantic::StructuralFieldKindPlan::Required(_) => quote! {
            pub const fn #name(&self) -> &#value { &self.#name }
        },
        crate::semantic::StructuralFieldKindPlan::Optional(_) if field.is_recursive() => quote! {
            pub fn #name(&self) -> Option<&#value> { self.#name.as_ref().as_ref() }
        },
        crate::semantic::StructuralFieldKindPlan::Optional(_) => quote! {
            pub const fn #name(&self) -> Option<&#value> { self.#name.as_ref() }
        },
        crate::semantic::StructuralFieldKindPlan::Sequence { .. } if field.is_recursive() => {
            quote! {
                pub fn #name(&self) -> &[#value] { self.#name.as_slice() }
            }
        }
        crate::semantic::StructuralFieldKindPlan::Sequence { .. } => quote! {
            pub const fn #name(&self) -> &[#value] { self.#name.as_slice() }
        },
    }
}

struct CategoryItem {
    name: String,
    span: Span,
    variants: Vec<(String, String, Span)>,
    origins: Vec<DeclarationKey>,
}

impl CategoryItem {
    fn finish(self) -> GeneratedItem {
        let ident = emitted_ident(&self.name, self.span);
        let variants = self.variants.into_iter().map(|(variant, element, span)| {
            let variant = emitted_ident(&variant, span);
            let element = emitted_ident(&element, span);
            quote! { #variant(#element) }
        });
        let tokens = quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum #ident {
                #(#variants),*
            }
        };
        GeneratedItem::new(ItemKey::named_type(self.name), tokens, self.origins)
    }
}

fn emit_product(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    ident: &syn::Ident,
) -> syn::Result<TokenStream> {
    if construction.fields().is_empty() {
        return Ok(quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #ident;
        });
    }

    let construction_name = construction.construction_id();
    let fields = construction
        .fields()
        .iter()
        .map(|field| {
            let name = field.name();
            let structural_constraint = field
                .structural_plan()
                .is_some_and(structural_field_is_constrained);
            let visibility =
                (field.accessor_mode().is_none() && !structural_constraint).then(|| quote! { pub });
            Ok(if let Some(structural) = field.structural_plan() {
                let ty = super::structural_field_type(structural);
                quote! { #visibility #name: #ty }
            } else {
                let ty = field.value_type();
                let boxed = plan
                    .boxed_fields()
                    .contains(&(construction_name.to_owned(), field.name_key()));
                if boxed {
                    quote! { #visibility #name: Box<#ty> }
                } else {
                    quote! { #visibility #name: #ty }
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct #ident {
            #(#fields),*
        }
    })
}

fn construction_requires_constructor(construction: &ConstructionPlan) -> bool {
    construction.invariant().requires_constructor()
        || construction.fields().iter().any(|field| {
            field
                .structural_plan()
                .is_some_and(structural_field_is_constrained)
        })
}

fn emit_invariant_impl(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    ident: &syn::Ident,
) -> syn::Result<TokenStream> {
    let mut allocator = super::LocalAllocator::default();
    for (category, feature) in construction.invariant().category_feature_reads() {
        allocator.reserve(feature_helper(feature.key(), category));
    }
    if construction.invariant().requires_context() {
        allocator.reserve("context");
    }
    let locals = construction
        .fields()
        .iter()
        .map(|field| (field.name_key(), allocator.allocate_ident(field.name())))
        .collect::<HashMap<_, _>>();
    let mut parameters = construction
        .fields()
        .iter()
        .map(|field| {
            let name = field_local(&locals, field)?;
            let ty = stored_field_type(plan, construction, field);
            Ok(quote! { #name: #ty })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    if construction.invariant().requires_context() {
        parameters.push(quote! { context: &ParseContext<'_> });
    }

    let mut subject_expressions = HashMap::new();
    for alternative in construction.invariant().alternatives() {
        for atom in alternative.atoms() {
            let subject = atom.subject();
            let key = subject.semantic_key();
            if subject_expressions.contains_key(&key) {
                continue;
            }
            subject_expressions.insert(
                key,
                constructor_subject_expression(plan, construction, subject, &locals)?,
            );
        }
    }
    let predicate = if construction.invariant().requires_constructor() {
        super::emit_invariant_expression(construction.invariant(), &subject_expressions)?
    } else {
        quote! { true }
    };
    let context_guards = construction
        .invariant()
        .context_identity_fields()
        .iter()
        .map(|field| {
            let local = locals
                .get(&identifier_key(field))
                .ok_or_else(|| internal("context identity field has no constructor local"))?;
            Ok(quote! { #local.valid_in(context) })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let length_guards = construction
        .fields()
        .iter()
        .filter_map(|field| {
            let crate::semantic::StructuralFieldKindPlan::Sequence { bounds, .. } =
                field.structural_kind()?
            else {
                return None;
            };
            structural_bounds_are_constrained(*bounds).then(|| {
                let name = field_local(&locals, field)?;
                Ok(emit_length_guard(name, *bounds))
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let initializers = construction
        .fields()
        .iter()
        .map(|field| {
            let name = field.name();
            let local = field_local(&locals, field)?;
            Ok(if name == local {
                quote! { #name }
            } else {
                quote! { #name: #local }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let accessors = construction.fields().iter().filter_map(|field| {
        let structural_constraint = field
            .structural_plan()
            .is_some_and(structural_field_is_constrained);
        if field.accessor_mode().is_none() && !structural_constraint {
            return None;
        }
        if let Some(structural) = field.structural_plan() {
            return Some(emit_structural_accessor(structural));
        }
        let mode = field
            .accessor_mode()
            .expect("non-structural private fields have a sealed accessor mode");
        let name = field.name();
        let ty = field.value_type();
        Some(match mode {
            AccessorMode::Copy => quote! {
                pub const fn #name(&self) -> #ty {
                    self.#name
                }
            },
            AccessorMode::Borrow => quote! {
                pub const fn #name(&self) -> &#ty {
                    &self.#name
                }
            },
        })
    });

    Ok(quote! {
        impl #ident {
            pub fn new(
                #(#parameters),*
            ) -> Option<Self> {
                if (#predicate) #(&& (#context_guards))* #(&& (#length_guards))* {
                    Some(Self { #(#initializers),* })
                } else {
                    None
                }
            }

            #(#accessors)*
        }
    })
}

fn constructor_subject_expression(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    subject: &PredicateSubjectPlan,
    locals: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    match subject {
        PredicateSubjectPlan::CategoryRole { role, category } => {
            let field = construction.field(&identifier_key(role))?;
            if field.kind() != ConstructionFieldKind::Category || field.terminal() != category {
                return Err(internal(
                    "category predicate subject is inconsistent with its field",
                ));
            }
            borrowed_field_expression(plan, construction, field, locals)
        }
        PredicateSubjectPlan::VocabRole { role, terminal } => {
            let field = construction.field(&identifier_key(role))?;
            if field.kind() != ConstructionFieldKind::Lex || field.terminal() != terminal {
                return Err(internal(
                    "vocabulary predicate subject is inconsistent with its field",
                ));
            }
            let name = field_local(locals, field)?;
            Ok(quote! { #name })
        }
        PredicateSubjectPlan::RoleFeature { role, feature } => resolve_constructor_feature(
            plan,
            construction,
            &crate::feature::FeaturePlace::Role {
                field: role.clone(),
                feature: *feature,
            },
            &mut std::collections::HashSet::new(),
            locals,
        ),
        PredicateSubjectPlan::ConstructionFeature(feature) => resolve_constructor_feature(
            plan,
            construction,
            &crate::feature::FeaturePlace::Construction(*feature),
            &mut std::collections::HashSet::new(),
            locals,
        ),
    }
}

fn resolve_constructor_feature(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    place: &crate::feature::FeaturePlace,
    visiting: &mut std::collections::HashSet<crate::feature::FeaturePlace>,
    locals: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    if let Some(crate::feature::FeatureResolution::Known(value)) =
        plan.feature_resolution(construction.construction_id(), place)
    {
        return Ok(feature_value(value));
    }
    if !visiting.insert(place.clone()) {
        return Err(internal(
            "constructor feature expression contains a sealed cycle",
        ));
    }
    let expression = if let Some(equation) = plan
        .feature_equations(construction.construction_id())
        .iter()
        .find(|equation| equation.target() == place)
    {
        lower_constructor_feature_expression(
            plan,
            construction,
            equation.value(),
            visiting,
            locals,
        )?
    } else if let crate::feature::FeaturePlace::Role { field, feature } = place {
        let stored = construction.field(&identifier_key(field))?;
        if stored.kind() != ConstructionFieldKind::Category {
            return Err(internal(
                "constructor feature subject lacks a sealed derivation",
            ));
        }
        let function = emitted_ident(
            &feature_helper(feature.key(), stored.terminal()),
            proc_macro2::Span::call_site(),
        );
        let field = borrowed_field_expression(plan, construction, stored, locals)?;
        quote! { #function(#field) }
    } else {
        return Err(internal(
            "construction feature subject lacks a sealed derivation",
        ));
    };
    visiting.remove(place);
    Ok(expression)
}

fn lower_constructor_feature_expression(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    expression: &crate::feature::FeatureExpr,
    visiting: &mut std::collections::HashSet<crate::feature::FeaturePlace>,
    locals: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    match expression {
        crate::feature::FeatureExpr::Constant(value) => Ok(feature_value(*value.value())),
        crate::feature::FeatureExpr::FromRole { role, feature } => resolve_constructor_feature(
            plan,
            construction,
            &crate::feature::FeaturePlace::Role {
                field: role.clone(),
                feature: *feature,
            },
            visiting,
            locals,
        ),
        crate::feature::FeatureExpr::MatchVocab { role, arms } => {
            let field = construction.field(&identifier_key(role))?;
            if field.kind() != ConstructionFieldKind::Lex {
                return Err(internal(
                    "constructor vocabulary feature source is not lexical",
                ));
            }
            let source = field_local(locals, field)?;
            let terminal = emitted_ident(field.terminal(), source.span());
            let arms = arms.iter().map(|(variant, value)| {
                let variant = variant.value();
                let value = feature_value(*value);
                quote! { #terminal::#variant => #value }
            });
            Ok(quote! { match #source { #(#arms),* } })
        }
    }
}

fn feature_value(value: crate::feature::FeatureValue) -> TokenStream {
    match value {
        crate::feature::FeatureValue::Bare => quote! { Agreement::Bare },
        crate::feature::FeatureValue::ThirdPersonSingular => {
            quote! { Agreement::ThirdPersonSingular }
        }
        crate::feature::FeatureValue::Singular => quote! { Number::Singular },
        crate::feature::FeatureValue::Plural => quote! { Number::Plural },
    }
}

fn borrowed_field_expression(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    field: &crate::semantic::ConstructionFieldPlan,
    locals: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    let name = field_local(locals, field)?;
    if is_boxed(plan, construction, field) {
        Ok(quote! { &*#name })
    } else {
        Ok(quote! { &#name })
    }
}

fn field_local<'a>(
    locals: &'a HashMap<String, syn::Ident>,
    field: &crate::semantic::ConstructionFieldPlan,
) -> syn::Result<&'a syn::Ident> {
    locals
        .get(&field.name_key())
        .ok_or_else(|| internal("stored field has no constructor local"))
}

fn stored_field_type(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    field: &crate::semantic::ConstructionFieldPlan,
) -> TokenStream {
    if let Some(structural) = field.structural_plan() {
        return super::structural_field_type(structural);
    }
    let ty = field.value_type();
    if is_boxed(plan, construction, field) {
        quote! { Box<#ty> }
    } else {
        quote! { #ty }
    }
}

fn is_boxed(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    field: &crate::semantic::ConstructionFieldPlan,
) -> bool {
    plan.boxed_fields()
        .contains(&(construction.construction_id().to_owned(), field.name_key()))
}

fn internal(detail: &str) -> syn::Error {
    syn::Error::new(
        proc_macro2::Span::call_site(),
        format!("internal invariant constructor emitter: {detail}"),
    )
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::items_after_statements,
        clippy::match_wildcard_for_single_variants,
        clippy::too_many_lines,
        clippy::type_complexity,
        reason = "literal full-surface structural oracles are intentionally table-dense"
    )]
    use quote::ToTokens;
    use syn::Fields;
    use syn::ImplItem;
    use syn::Item;
    use syn::Type;
    use syn::Visibility;
    use syn::parse::Parser;

    use crate::DeclarationKind;
    use crate::ItemKey;
    use crate::test_support::representative_expansion;

    #[test]
    fn structural_products_sums_bounds_accessors_and_recursive_edges_are_exact() {
        let plan = structural_semantic_plan();
        let items = super::emit(&plan).expect("structural AST fixture emits");

        let Item::Enum(choice) = parse_named(&items, "Choice") else {
            panic!("Choice is an enum");
        };
        assert_public(&choice.vis, "Choice");
        assert_common_derives(&choice.attrs, "Choice");
        assert_eq!(
            choice
                .variants
                .iter()
                .map(|variant| {
                    let Fields::Unnamed(fields) = &variant.fields else {
                        panic!("structural sum alternatives have tuple payloads");
                    };
                    assert_eq!(fields.unnamed.len(), 1);
                    (
                        variant.ident.to_string(),
                        fields.unnamed[0].ty.to_token_stream().to_string(),
                    )
                })
                .collect::<Vec<_>>(),
            [
                ("Left".to_owned(), "LeftNode".to_owned()),
                ("Right".to_owned(), "RightNode".to_owned()),
            ]
        );

        let Item::Struct(holder) = parse_named(&items, "Holder") else {
            panic!("Holder is a struct");
        };
        assert_public(&holder.vis, "Holder");
        assert_common_derives(&holder.attrs, "Holder");
        let Fields::Named(fields) = holder.fields else {
            panic!("Holder has named fields");
        };
        assert_eq!(
            fields
                .named
                .iter()
                .map(|field| (
                    field.ident.as_ref().unwrap().to_string(),
                    field.ty.to_token_stream().to_string(),
                    field_visibility(&field.vis),
                ))
                .collect::<Vec<_>>(),
            [
                (
                    "maybe".to_owned(),
                    quote::quote!(Option<LeftNode>).to_string(),
                    FieldVisibility::Public,
                ),
                (
                    "items".to_owned(),
                    quote::quote!(Vec<Choice>).to_string(),
                    FieldVisibility::Private,
                ),
            ]
        );
        assert_eq!(
            method_headers(&inherent_impl(&items, "Holder")),
            [
                quote::quote! {
                    pub fn new(maybe: Option<LeftNode>, items: Vec<Choice>) -> Option<Self>
                }
                .to_string(),
                quote::quote! { pub const fn items(&self) -> &[Choice] }.to_string(),
            ]
        );
        let body = method(&inherent_impl(&items, "Holder"), "new")
            .block
            .to_token_stream()
            .to_string();
        assert!(body.contains("items . len () >= 1"), "{body}");
        assert!(body.contains("Some (Self { maybe , items })"), "{body}");

        let Item::Enum(recursive_choice) = parse_named(&items, "RecursiveChoice") else {
            panic!("RecursiveChoice is an enum");
        };
        let Fields::Unnamed(branch) = &recursive_choice.variants[0].fields else {
            panic!("recursive sum alternative has one payload");
        };
        assert_eq!(
            branch.unnamed[0].ty.to_token_stream().to_string(),
            quote::quote!(Box<RecursiveBranch>).to_string(),
            "the sealed recursive sum edge receives an explicit box",
        );
        let Item::Struct(recursive_branch) = parse_named(&items, "RecursiveBranch") else {
            panic!("RecursiveBranch is a struct");
        };
        let Fields::Named(recursive_fields) = recursive_branch.fields else {
            panic!("RecursiveBranch has named fields");
        };
        assert_eq!(
            recursive_fields.named[0].ty.to_token_stream().to_string(),
            {
                let expected: Type = syn::parse_quote!(Box<Vec<RecursiveChoice>>);
                expected.to_token_stream().to_string()
            },
            "the sealed recursive product edge receives an explicit box",
        );
        assert!(
            choice
                .variants
                .iter()
                .all(|variant| { !variant.fields.to_token_stream().to_string().contains("Box") })
        );
    }

    #[test]
    fn invariant_products_emit_exact_constructor_and_accessor_surface() {
        let items = invariant_ast_items();

        let Item::Struct(guarded) = parse_named(&items, "GuardedNode") else {
            panic!("GuardedNode is a struct");
        };
        let Fields::Named(fields) = guarded.fields else {
            panic!("GuardedNode has named fields");
        };
        assert_eq!(
            fields
                .named
                .iter()
                .map(|field| {
                    (
                        field.ident.as_ref().unwrap().to_string(),
                        exact_simple_type(&field.ty),
                        field_visibility(&field.vis),
                    )
                })
                .collect::<Vec<_>>(),
            [
                (
                    "open".to_owned(),
                    "OpenValue".to_owned(),
                    FieldVisibility::Public
                ),
                (
                    "mode".to_owned(),
                    "Mode".to_owned(),
                    FieldVisibility::Private
                ),
                (
                    "child".to_owned(),
                    "Child".to_owned(),
                    FieldVisibility::Private
                ),
            ]
        );

        let implementation = inherent_impl(&items, "GuardedNode");
        assert_eq!(
            method_headers(&implementation),
            [
                quote::quote! {
                    pub fn new(open: OpenValue, mode: Mode, child: Child) -> Option<Self>
                }
                .to_string(),
                quote::quote! { pub const fn mode(&self) -> Mode }.to_string(),
                quote::quote! { pub const fn child(&self) -> &Child }.to_string(),
            ]
        );
        let constructor = method(&implementation, "new");
        let body = constructor.block.to_token_stream().to_string();
        for required in [
            "matches ! (mode , Mode :: One)",
            "matches ! (& child , Child :: First (_))",
            "Agreement :: Bare",
            "matches ! (mode , Mode :: Two)",
            "matches ! (& child , Child :: Second (_))",
            "Agreement :: ThirdPersonSingular",
            "Some (Self { open , mode , child })",
            "None",
        ] {
            assert!(
                body.contains(required),
                "constructor body lacks `{required}`: {body}"
            );
        }

        let Item::Struct(recursive) = parse_named(&items, "RecursiveNode") else {
            panic!("RecursiveNode is a struct");
        };
        let Fields::Named(fields) = recursive.fields else {
            panic!("RecursiveNode has named fields");
        };
        assert_eq!(
            exact_field_type(&fields.named[1].ty),
            ("Child".to_owned(), true)
        );
        assert_eq!(
            method_headers(&inherent_impl(&items, "RecursiveNode")),
            [
                quote::quote! {
                    pub fn new(mode: Mode, child: Box<Child>) -> Option<Self>
                }
                .to_string(),
                quote::quote! { pub const fn mode(&self) -> Mode }.to_string(),
                quote::quote! { pub const fn child(&self) -> &Child }.to_string(),
            ]
        );
    }

    #[test]
    fn invariant_context_identity_is_guarded_and_unit_or_open_products_gain_no_impl() {
        let items = invariant_ast_items();

        let implementation = inherent_impl(&items, "ContextNode");
        assert_eq!(
            method_headers(&implementation),
            [
                quote::quote! {
                    pub fn new(
                        spelling: SelfReferenceSpelling,
                        context: &ParseContext<'_>
                    ) -> Option<Self>
                }
                .to_string(),
                quote::quote! { pub const fn spelling(&self) -> SelfReferenceSpelling }.to_string(),
            ]
        );
        let body = method(&implementation, "new")
            .block
            .to_token_stream()
            .to_string();
        assert!(
            body.contains("spelling . valid_in (context)"),
            "context identity guard: {body}"
        );

        for name in ["UnitNode", "OpenNode"] {
            assert!(
                items.iter().all(|item| {
                    !matches!(
                        &item.key,
                        ItemKey::Impl {
                            trait_name: None,
                            self_ty,
                        } if self_ty == name
                    )
                }),
                "{name} has no invariant implementation"
            );
        }
    }

    #[test]
    fn invariant_expression_rejects_a_missing_typed_subject() {
        let plan = invariant_semantic_plan();
        let guarded = plan
            .constructions()
            .iter()
            .find(|construction| construction.element_type() == "GuardedNode")
            .expect("guarded construction is sealed");
        let error = super::super::emit_invariant_expression(
            guarded.invariant(),
            &std::collections::HashMap::new(),
        )
        .expect_err("missing subject expressions are named errors")
        .to_string();

        assert!(error.contains("invariant expression"), "{error}");
        assert!(error.contains("subject"), "{error}");
    }

    #[test]
    fn invariant_role_feature_seals_its_category_helper_and_dnf_grouping() {
        let plan = invariant_semantic_plan();
        assert!(
            plan.category_reads_feature("FeatureChild", crate::feature::Feature::Agreement,),
            "a role-feature predicate seals the category helper inventory used by its constructor",
        );

        let items = super::emit(&plan).expect("invariant AST fixture emits");
        let guarded = inherent_impl(&items, "GuardedNode");
        let constructor = method(&guarded, "new");
        let Some(syn::Stmt::Expr(syn::Expr::If(condition), None)) = constructor.block.stmts.first()
        else {
            panic!("constructor starts with its invariant condition");
        };
        let syn::Expr::Paren(predicate) = condition.cond.as_ref() else {
            panic!("constructor predicate is explicitly grouped");
        };
        let syn::Expr::Binary(disjunction) = predicate.expr.as_ref() else {
            panic!("two DNF alternatives form a binary expression");
        };
        assert!(matches!(disjunction.op, syn::BinOp::Or(_)));
        for conjunction in [&disjunction.left, &disjunction.right] {
            assert!(
                expression_contains_and(conjunction),
                "each DNF alternative remains a conjunction: {}",
                conjunction.to_token_stream(),
            );
        }
    }

    #[test]
    fn invariant_feature_dependencies_drive_constructor_and_private_field_policy() {
        let plan = invariant_semantic_plan();
        let items = super::emit(&plan).expect("feature dependency fixture emits");

        for (element, fields) in [
            ("ConstructionFeatureNode", &["mode"][..]),
            ("TransitiveRoleFeatureNode", &["child", "mode"][..]),
        ] {
            let Item::Struct(product) = parse_named(&items, element) else {
                panic!("{element} is a struct");
            };
            let Fields::Named(named) = product.fields else {
                panic!("{element} has named fields");
            };
            assert_eq!(
                named
                    .named
                    .iter()
                    .filter(|field| matches!(field.vis, Visibility::Inherited))
                    .map(|field| field.ident.as_ref().unwrap().to_string())
                    .collect::<Vec<_>>(),
                fields,
                "every direct or transitive invariant dependency is private",
            );
            assert_eq!(
                method_headers(&inherent_impl(&items, element))
                    .into_iter()
                    .skip(1)
                    .collect::<Vec<_>>(),
                fields
                    .iter()
                    .map(|field| {
                        let field = syn::Ident::new(field, proc_macro2::Span::call_site());
                        if field == "child" {
                            quote::quote! { pub const fn #field(&self) -> &FeatureChild }
                                .to_string()
                        } else {
                            quote::quote! { pub const fn #field(&self) -> Mode }.to_string()
                        }
                    })
                    .collect::<Vec<_>>(),
                "every private dependency has exactly one sealed accessor",
            );
        }
    }

    #[test]
    fn invariant_constant_feature_emits_constructor_without_accessors() {
        let plan = invariant_semantic_plan();
        let construction = plan
            .constructions()
            .iter()
            .find(|construction| construction.element_type() == "ConstantFeatureNode")
            .expect("constant-feature construction is sealed");
        assert!(construction.invariant().requires_constructor());
        assert_eq!(construction.fields().len(), 1);
        assert!(construction.fields()[0].accessor_mode().is_none());

        let items = super::emit(&plan).expect("constant-feature fixture emits");
        let Item::Struct(product) = parse_named(&items, "ConstantFeatureNode") else {
            panic!("ConstantFeatureNode is a struct");
        };
        let Fields::Named(fields) = product.fields else {
            panic!("ConstantFeatureNode has named fields");
        };
        assert!(matches!(fields.named[0].vis, Visibility::Public(_)));
        assert_eq!(
            method_headers(&inherent_impl(&items, "ConstantFeatureNode")),
            [quote::quote! {
                pub fn new(open: OpenValue) -> Option<Self>
            }
            .to_string()],
            "the non-tautological product has new and no accessor methods",
        );
    }

    #[test]
    fn invariant_category_feature_helper_is_reserved_before_constructor_locals() {
        let items = invariant_ast_items();
        let implementation = inherent_impl(&items, "FeatureHelperCollisionNode");
        let constructor = method(&implementation, "new");
        assert_eq!(
            constructor.sig.to_token_stream().to_string(),
            quote::quote! {
                fn new(
                    child: FeatureChild,
                    agreement_for_feature_child_2: Mode
                ) -> Option<Self>
            }
            .to_string(),
            "the authored field local is suffixed while the generated helper keeps its ABI name",
        );
        let body = constructor.block.to_token_stream().to_string();
        assert!(
            body.contains("agreement_for_feature_child (& child)"),
            "the category helper call remains unshadowed: {body}",
        );
        assert!(
            body.contains("agreement_for_feature_child : agreement_for_feature_child_2"),
            "the suffixed local initializes the unchanged authored field: {body}",
        );
    }

    #[test]
    fn invariant_category_subject_rejects_a_mismatched_sealed_domain() {
        let plan = invariant_semantic_plan();
        let recursive = plan
            .constructions()
            .iter()
            .find(|construction| construction.element_type() == "RecursiveNode")
            .expect("recursive construction is sealed");
        let subject = crate::semantic::PredicateSubjectPlan::CategoryRole {
            role: syn::parse_quote!(child),
            category: "WrongCategory".to_owned(),
        };
        let locals =
            std::collections::HashMap::from([("child".to_owned(), syn::parse_quote!(child))]);

        let error = super::constructor_subject_expression(&plan, recursive, &subject, &locals)
            .expect_err("a sealed category/field mismatch is diagnosed")
            .to_string();

        assert!(error.contains("invariant constructor emitter"), "{error}");
        assert!(error.contains("category predicate subject"), "{error}");
        assert!(error.contains("inconsistent"), "{error}");
    }

    #[test]
    fn invariant_feature_constructors_compile_and_execute_from_actual_ast_items() {
        let plan = invariant_semantic_plan();
        let items = super::emit(&plan).expect("invariant AST fixture emits");
        let generated = items.iter().map(|item| &item.tokens);
        let source = quote::quote! {
            #![allow(dead_code)]

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum Agreement { Bare, ThirdPersonSingular }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum Number { Singular, Plural }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum OpenValue { Open }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum Mode { One, Two }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum SelfReferenceSpelling { Full, Abbreviated }
            struct ParseContext<'a> { canonical: SelfReferenceSpelling, marker: &'a str }

            impl SelfReferenceSpelling {
                fn valid_in(self, context: &ParseContext<'_>) -> bool {
                    self == context.canonical
                }
            }

            #(#generated)*

            fn agreement_for_feature_child(child: &FeatureChild) -> Agreement {
                match child {
                    FeatureChild::FeatureBare(_) => Agreement::Bare,
                    FeatureChild::FeatureThird(_) => Agreement::ThirdPersonSingular,
                }
            }

            fn main() {
                let constructed = ConstructionFeatureNode::new(Mode::One)
                    .expect("construction feature accepts its Bare source");
                assert_eq!(constructed.mode(), Mode::One);
                assert!(ConstructionFeatureNode::new(Mode::Two).is_none());

                let constant = ConstantFeatureNode::new(OpenValue::Open)
                    .expect("a known satisfying feature still emits a constructor");
                assert_eq!(constant.open, OpenValue::Open);

                let transitive = TransitiveRoleFeatureNode::new(
                    FeatureChild::FeatureBare(FeatureBare),
                    Mode::One,
                )
                .expect("transitive role feature accepts its Bare source");
                assert!(matches!(transitive.child(), FeatureChild::FeatureBare(_)));
                assert_eq!(transitive.mode(), Mode::One);
                assert!(TransitiveRoleFeatureNode::new(
                    FeatureChild::FeatureBare(FeatureBare),
                    Mode::Two,
                ).is_none());

                assert!(RoleFeatureGuarded::new(
                    FeatureChild::FeatureBare(FeatureBare),
                ).is_some());
                assert!(RoleFeatureGuarded::new(
                    FeatureChild::FeatureThird(FeatureThird),
                ).is_none());

                let collision = FeatureHelperCollisionNode::new(
                    FeatureChild::FeatureBare(FeatureBare),
                    Mode::Two,
                )
                .expect("the generated helper remains callable beside its namesake field");
                assert!(matches!(collision.child(), FeatureChild::FeatureBare(_)));
                assert_eq!(collision.agreement_for_feature_child, Mode::Two);
                assert!(FeatureHelperCollisionNode::new(
                    FeatureChild::FeatureThird(FeatureThird),
                    Mode::One,
                ).is_none());
            }
        }
        .to_string();

        let directory = tempfile::tempdir().expect("temporary compiler harness directory");
        let source_path = directory.path().join("invariant_ast_harness.rs");
        let binary_path = directory.path().join("invariant_ast_harness");
        std::fs::write(&source_path, &source).expect("write deterministic compiler harness");
        let compiler = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
        let compilation = std::process::Command::new(compiler)
            .arg("--edition=2024")
            .arg(&source_path)
            .arg("-o")
            .arg(&binary_path)
            .output()
            .expect("run active Rust compiler");
        assert!(
            compilation.status.success(),
            "AST harness compilation failed\nstdout:\n{}\nstderr:\n{}\nsource:\n{source}",
            String::from_utf8_lossy(&compilation.stdout),
            String::from_utf8_lossy(&compilation.stderr),
        );

        let execution = std::process::Command::new(&binary_path)
            .output()
            .expect("execute compiled invariant AST harness");
        assert!(
            execution.status.success(),
            "AST harness execution failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&execution.stdout),
            String::from_utf8_lossy(&execution.stderr),
        );
    }

    #[test]
    fn emits_exact_category_variants_products_visibility_and_boxing() {
        let expansion = representative_expansion();
        let items = expansion.items();

        let node = parse_named(items, "Node");
        let Item::Enum(node) = node else {
            panic!("Node is an enum");
        };
        assert_eq!(
            derive_names(&node.attrs),
            ["Debug", "Clone", "PartialEq", "Eq"]
        );
        assert_eq!(
            node.variants
                .iter()
                .map(|variant| {
                    let Fields::Unnamed(fields) = &variant.fields else {
                        panic!("category variants have one unnamed payload");
                    };
                    assert_eq!(fields.unnamed.len(), 1);
                    (variant.ident.to_string(), type_name(&fields.unnamed[0].ty))
                })
                .collect::<Vec<_>>(),
            [
                ("Leaf".into(), "WordLeaf".into()),
                ("Chain".into(), "Chain".into())
            ]
        );
        assert_eq!(
            named(items, "Node")
                .origins
                .iter()
                .map(|origin| (origin.kind(), origin.name()))
                .collect::<Vec<_>>(),
            [
                (DeclarationKind::Construction, "leaf"),
                (DeclarationKind::Construction, "chain"),
            ]
        );

        let chain = parse_named(items, "Chain");
        let Item::Struct(chain) = chain else {
            panic!("Chain is a struct");
        };
        assert_eq!(
            derive_names(&chain.attrs),
            ["Debug", "Clone", "PartialEq", "Eq"]
        );
        let Fields::Named(fields) = chain.fields else {
            panic!("Chain has named fields");
        };
        let fields = fields.named.into_iter().collect::<Vec<_>>();
        assert_eq!(fields.len(), 2);
        assert!(matches!(fields[0].vis, Visibility::Public(_)));
        assert_eq!(boxed_inner_name(&fields[0].ty), Some("Node".to_owned()));
        assert!(matches!(fields[1].vis, Visibility::Public(_)));
        assert_eq!(type_name(&fields[1].ty), "Words");

        let leaf = parse_named(items, "WordLeaf");
        let Item::Struct(leaf) = leaf else {
            panic!("WordLeaf is a struct");
        };
        let Fields::Named(fields) = leaf.fields else {
            panic!("WordLeaf has named fields");
        };
        assert!(matches!(fields.named[0].vis, Visibility::Public(_)));
        assert_eq!(type_name(&fields.named[0].ty), "Words");

        for item in items {
            let file = syn::parse2::<syn::File>(item.tokens.clone()).expect("item reparses");
            assert_eq!(file.items.len(), 1, "{:?}", item.key);
        }
    }

    #[test]
    fn synthetic_projection_has_exact_category_products_visibility_and_boxing() {
        let expansion = crate::test_support::synthetic_projection_expansion();

        const CATEGORIES: &[(&str, &[(&str, &str)])] = &[
            ("Expr", &[("Leaf", "LeafNode"), ("Nested", "NestedNode")]),
            (
                "Predicate",
                &[("Action", "ActionNode"), ("Idle", "IdleNode")],
            ),
            ("Tag", &[("Solo", "SoloTag")]),
            ("Document", &[("Document", "DocumentNode")]),
        ];
        for &(name, variants) in CATEGORIES {
            let Item::Enum(item) = parse_named(expansion.items(), name) else {
                panic!("{name} is a category enum");
            };
            assert_public(&item.vis, name);
            assert_common_derives(&item.attrs, name);
            assert_eq!(
                item.variants
                    .iter()
                    .map(|variant| {
                        let Fields::Unnamed(fields) = &variant.fields else {
                            panic!("{} has one tuple payload", variant.ident);
                        };
                        (
                            variant.ident.to_string(),
                            exact_simple_type(&fields.unnamed[0].ty),
                        )
                    })
                    .collect::<Vec<_>>(),
                variants
                    .iter()
                    .map(|&(variant, ty)| (variant.to_owned(), ty.to_owned()))
                    .collect::<Vec<_>>(),
            );
        }

        const PUB: FieldVisibility = FieldVisibility::Public;
        const PRIVATE: FieldVisibility = FieldVisibility::Private;
        const PRODUCTS: &[(&str, Option<&[(&str, &str, bool, FieldVisibility)]>)] = &[
            (
                "LeafNode",
                Some(&[
                    ("mode", "Mode", false, PUB),
                    ("resource", "Resource", false, PUB),
                ]),
            ),
            (
                "NestedNode",
                Some(&[
                    ("next", "Expr", true, PUB),
                    ("marker", "Marker", false, PUB),
                ]),
            ),
            ("ActionNode", None),
            ("IdleNode", None),
            ("SoloTag", Some(&[("mode", "Mode", false, PRIVATE)])),
            (
                "DocumentNode",
                Some(&[
                    ("subject", "Expr", false, PRIVATE),
                    ("predicate", "Predicate", false, PUB),
                    ("handle", "Handle", false, PUB),
                    ("pair", "Pair", false, PUB),
                ]),
            ),
        ];
        for &(name, expected_fields) in PRODUCTS {
            let Item::Struct(item) = parse_named(expansion.items(), name) else {
                panic!("{name} is a product struct");
            };
            assert_public(&item.vis, name);
            assert_common_derives(&item.attrs, name);
            match expected_fields {
                None => assert!(matches!(item.fields, Fields::Unit)),
                Some(expected) => {
                    let Fields::Named(fields) = item.fields else {
                        panic!("{name} has named fields");
                    };
                    assert_eq!(
                        fields
                            .named
                            .iter()
                            .map(|field| {
                                let (ty, boxed) = exact_field_type(&field.ty);
                                (
                                    field.ident.as_ref().unwrap().to_string(),
                                    ty,
                                    boxed,
                                    field_visibility(&field.vis),
                                )
                            })
                            .collect::<Vec<_>>(),
                        expected
                            .iter()
                            .map(|&(field, ty, boxed, visibility)| {
                                (field.to_owned(), ty.to_owned(), boxed, visibility)
                            })
                            .collect::<Vec<_>>(),
                    );
                }
            }
        }

        assert_eq!(
            expansion
                .items()
                .iter()
                .filter_map(|item| {
                    if let ItemKey::Named {
                        kind: crate::NamedKind::Type,
                        name,
                    } = &item.key
                    {
                        Some(name.as_str())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
            [
                "Expr",
                "Predicate",
                "Tag",
                "Document",
                "LeafNode",
                "NestedNode",
                "ActionNode",
                "IdleNode",
                "SoloTag",
                "DocumentNode",
                "Mode",
                "ObjectStem",
                "ActionStem",
                "Agreement",
                "Number",
                "FeatureConstraint",
                "CasePosition",
                "PrefixPosition",
                "ScanPosition",
                "DeclarationClass",
                "DeclarationMatcher",
                "DeclarationLeaf",
                "Lexical",
                "Leaf",
                "TerminalClass",
                "LexicalTerminal",
                "LexicalProvenanceKind",
                "LexicalOwnerTemplate",
                "LexicalOwnerIdentity",
                "LexicalOwner",
                "BuildValue",
                "NonterminalCategory",
                "Category",
                "Construction",
                "RuleId",
            ],
        );
    }
    fn named<'a>(items: &'a [crate::GeneratedItem], name: &str) -> &'a crate::GeneratedItem {
        items
            .iter()
            .find(|item| matches!(&item.key, ItemKey::Named { name: found, .. } if found == name))
            .unwrap_or_else(|| panic!("generated item `{name}` exists"))
    }

    fn inherent_impl(items: &[crate::GeneratedItem], name: &str) -> syn::ItemImpl {
        let item = items
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty,
                    } if self_ty == name
                )
            })
            .unwrap_or_else(|| panic!("generated inherent impl `{name}` exists"));
        let file = syn::parse2::<syn::File>(item.tokens.clone()).expect("impl item reparses");
        assert_eq!(file.items.len(), 1);
        let Item::Impl(implementation) = file.items.into_iter().next().expect("one impl item")
        else {
            panic!("{name} implementation is an impl item");
        };
        implementation
    }

    fn method_headers(implementation: &syn::ItemImpl) -> Vec<String> {
        implementation
            .items
            .iter()
            .map(|item| {
                let ImplItem::Fn(method) = item else {
                    panic!("invariant impl contains only methods");
                };
                let visibility = &method.vis;
                let signature = &method.sig;
                quote::quote! { #visibility #signature }.to_string()
            })
            .collect()
    }

    fn method<'a>(implementation: &'a syn::ItemImpl, name: &str) -> &'a syn::ImplItemFn {
        implementation
            .items
            .iter()
            .find_map(|item| match item {
                ImplItem::Fn(method) if method.sig.ident == name => Some(method),
                ImplItem::Fn(_) | ImplItem::Const(_) | ImplItem::Type(_) | ImplItem::Macro(_) => {
                    None
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("method `{name}` exists"))
    }

    fn invariant_ast_items() -> Vec<crate::GeneratedItem> {
        let plan = invariant_semantic_plan();
        super::emit(&plan).expect("invariant AST fixture emits")
    }

    fn structural_semantic_plan() -> crate::semantic::SemanticPlan {
        crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                abstract sum Choice { Left: LeftNode, Right: RightNode, }
                abstract product Holder {
                    maybe: opt LeftNode,
                    items: seq Choice terminated by ",",
                }
                require len(Holder.items) >= 1;
                abstract sum RecursiveChoice { Branch: RecursiveBranch, }
                abstract product RecursiveBranch {
                    choices: seq RecursiveChoice terminated by ".",
                }
                require len(RecursiveBranch.choices) = 1;
                construction left: LeftNode {
                    element LeftValue {}
                    form left = "left";
                }
                construction right: RightNode {
                    element RightValue {}
                    form right = "right";
                }
                root LeftNode { punctuation = "."; eoi = true; standalone_render = true; }
                root RightNode { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("structural AST fixture parses"),
        )
        .expect("structural AST fixture validates")
        .into_semantic()
    }

    fn invariant_semantic_plan() -> crate::semantic::SemanticPlan {
        crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab OpenValue { Open = "open", }
                vocab Mode { One = "one", Two = "two", }
                identity SelfReferenceSpelling {
                    generate context {
                        Full => card_name,
                        Abbreviated => abbreviated_card_name,
                        canonical_on_collision = Full;
                    }
                }
                construction first: Child {
                    element FirstChild {}
                    form first = "first";
                }
                construction second: Child {
                    element SecondChild {}
                    form second = "second";
                }
                construction feature_bare: FeatureChild {
                    element FeatureBare {}
                    derive agreement = Values::Bare;
                    form feature_bare = "feature bare";
                }
                construction feature_third: FeatureChild {
                    element FeatureThird {}
                    derive agreement = Values::ThirdPersonSingular;
                    form feature_third = "feature third";
                }
                construction role_feature_guarded: FeatureRoot {
                    element RoleFeatureGuarded { child: FeatureChild, }
                    require child.agreement is Bare;
                    derive agreement = child.agreement;
                    form role_feature_guarded = child;
                }
                construction construction_feature: FeatureRoot {
                    element ConstructionFeatureNode { mode: lex Mode, }
                    require agreement is Bare;
                    derive agreement = mode.agreement;
                    derive mode.agreement = match mode {
                        One => Values::Bare,
                        Two => Values::ThirdPersonSingular,
                    };
                    form construction_feature = lex(mode);
                }
                construction transitive_role_feature: FeatureRoot {
                    element TransitiveRoleFeatureNode {
                        child: FeatureChild,
                        mode: lex Mode,
                    }
                    require child.agreement is Bare;
                    derive agreement = child.agreement;
                    derive child.agreement = mode.agreement;
                    derive mode.agreement = match mode {
                        One => Values::Bare,
                        Two => Values::ThirdPersonSingular,
                    };
                    form transitive_role_feature = child lex(mode);
                }
                construction feature_helper_collision: FeatureRoot {
                    element FeatureHelperCollisionNode {
                        child: FeatureChild,
                        agreement_for_feature_child: lex Mode,
                    }
                    require child.agreement is Bare;
                    derive agreement = child.agreement;
                    form feature_helper_collision = child lex(agreement_for_feature_child);
                }
                construction constant_feature: FeatureRoot {
                    element ConstantFeatureNode { open: lex OpenValue, }
                    require agreement is Bare;
                    derive agreement = Values::Bare;
                    form constant_feature = lex(open);
                }
                construction recursive: Child {
                    element RecursiveNode { mode: lex Mode, child: Child, }
                    require mode is One;
                    require child is First;
                    form recursive = lex(mode) child;
                }
                construction guarded: Root {
                    element GuardedNode {
                        open: lex OpenValue,
                        mode: lex Mode,
                        child: Child,
                    }
                    require any(
                        all(mode is One, child is First, agreement is Bare),
                        all(mode is Two, child is Second, agreement is ThirdPersonSingular)
                    );
                    derive agreement = mode.agreement;
                    derive mode.agreement = match mode {
                        One => Values::Bare,
                        Two => Values::ThirdPersonSingular,
                    };
                    form guarded = lex(open) lex(mode) child;
                }
                construction contextual: Root {
                    element ContextNode { spelling: identity SelfReferenceSpelling, }
                    derive agreement = Values::Bare;
                    form contextual = identity(spelling);
                }
                construction unit: Root {
                    element UnitNode {}
                    derive agreement = Values::Bare;
                    form unit = "unit";
                }
                construction open: Root {
                    element OpenNode { open: lex OpenValue, }
                    derive agreement = Values::Bare;
                    form open = lex(open);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("invariant AST fixture parses"),
        )
        .expect("invariant AST fixture validates")
        .into_semantic()
    }

    fn expression_contains_and(expression: &syn::Expr) -> bool {
        match expression {
            syn::Expr::Binary(binary) => {
                matches!(binary.op, syn::BinOp::And(_))
                    || expression_contains_and(&binary.left)
                    || expression_contains_and(&binary.right)
            }
            syn::Expr::Group(group) => expression_contains_and(&group.expr),
            syn::Expr::Paren(paren) => expression_contains_and(&paren.expr),
            _ => false,
        }
    }

    fn parse_named(items: &[crate::GeneratedItem], name: &str) -> Item {
        let file = syn::parse2::<syn::File>(named(items, name).tokens.clone())
            .expect("generated item parses as a file");
        assert_eq!(file.items.len(), 1);
        file.items.into_iter().next().expect("one item")
    }

    fn derive_names(attrs: &[syn::Attribute]) -> Vec<String> {
        assert_eq!(attrs.len(), 1, "derive is the only item attribute");
        let derive = attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("derive"))
            .expect("derive attribute");
        let syn::Meta::List(list) = &derive.meta else {
            panic!("derive is a list");
        };
        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
            .parse2(list.tokens.clone())
            .expect("derive paths parse")
            .iter()
            .map(|path| {
                assert_eq!(path.segments.len(), 1, "derive path is unqualified");
                path.segments[0].ident.to_string()
            })
            .collect()
    }

    fn assert_common_derives(attrs: &[syn::Attribute], name: &str) {
        assert_eq!(
            derive_names(attrs),
            ["Debug", "Clone", "PartialEq", "Eq"],
            "{name} derives"
        );
    }

    fn assert_public(visibility: &Visibility, name: &str) {
        assert!(
            matches!(visibility, Visibility::Public(_)),
            "{name} is public"
        );
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum FieldVisibility {
        Public,
        Private,
        Crate,
    }

    fn field_visibility(visibility: &Visibility) -> FieldVisibility {
        match visibility {
            Visibility::Public(_) => FieldVisibility::Public,
            Visibility::Inherited => FieldVisibility::Private,
            Visibility::Restricted(restricted)
                if restricted.in_token.is_none()
                    && restricted.path.leading_colon.is_none()
                    && restricted.path.is_ident("crate") =>
            {
                FieldVisibility::Crate
            }
            other => panic!("unexpected field visibility: {other:?}"),
        }
    }

    fn exact_field_type(ty: &Type) -> (String, bool) {
        let Type::Path(path) = ty else {
            panic!("field type is a path: {}", ty.to_token_stream());
        };
        assert!(path.qself.is_none(), "field type has no qualified self");
        if path.path.segments.len() == 1 && path.path.segments[0].ident == "Box" {
            let syn::PathArguments::AngleBracketed(arguments) = &path.path.segments[0].arguments
            else {
                panic!("Box has one type argument");
            };
            assert_eq!(arguments.args.len(), 1, "Box has one type argument");
            let syn::GenericArgument::Type(inner) = &arguments.args[0] else {
                panic!("Box argument is a type");
            };
            return (exact_simple_type(inner), true);
        }
        (exact_simple_type(ty), false)
    }

    fn exact_simple_type(ty: &Type) -> String {
        let Type::Path(path) = ty else {
            panic!("type is a simple path: {}", ty.to_token_stream());
        };
        assert!(path.qself.is_none(), "type has no qualified self");
        assert!(path.path.leading_colon.is_none(), "type is not absolute");
        assert_eq!(path.path.segments.len(), 1, "type path is unqualified");
        let segment = &path.path.segments[0];
        assert!(matches!(segment.arguments, syn::PathArguments::None));
        segment.ident.to_string()
    }

    fn type_name(ty: &Type) -> String {
        let Type::Path(path) = ty else {
            panic!("field type is a path: {}", ty.to_token_stream());
        };
        path.path
            .segments
            .last()
            .expect("type path has a segment")
            .ident
            .to_string()
    }

    fn boxed_inner_name(ty: &Type) -> Option<String> {
        let Type::Path(path) = ty else { return None };
        let segment = path.path.segments.last()?;
        if segment.ident != "Box" {
            return None;
        }
        let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
            return None;
        };
        let syn::GenericArgument::Type(inner) = arguments.args.first()? else {
            return None;
        };
        Some(type_name(inner))
    }
}
