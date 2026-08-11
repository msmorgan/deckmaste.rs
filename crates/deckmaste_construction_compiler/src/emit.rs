//! Deterministic code emission. Input is [`crate::validate::ValidatedGroup`]
//! by construction — the validator is the only way to obtain one. Iteration
//! is over declaration-order `Vec`s exclusively; determinism is structural,
//! not an afterthought.

use proc_macro2::TokenStream;
use quote::quote;

use crate::model::AstShape;
use crate::model::Constraint;
use crate::model::ConstructionDeclaration;
use crate::model::ElementDeclaration;
use crate::model::FieldBinding;
use crate::model::FieldKind;
use crate::model::GroupDeclaration;
use crate::model::InverseDispatchKind;
use crate::model::LensApplication;
use crate::model::LensEditKind;
use crate::model::LensFieldKind;
use crate::model::Predicate;
use crate::validate::ValidatedGroup;

#[must_use]
pub fn emit_group(validated: &ValidatedGroup<'_>) -> TokenStream {
    let group = validated.group();
    let module = quote::format_ident!("__constructions_{}", group.name.value);
    let serialize_reached = trait_reached_elements(group, |construction| construction.serialize);
    let deserialize_reached =
        trait_reached_elements(group, |construction| construction.deserialize);
    let elements: Vec<TokenStream> = group
        .elements
        .iter()
        .map(|element| {
            let serialize = serialize_reached.contains(&element.name.value);
            let deserialize = deserialize_reached.contains(&element.name.value);
            element_item(group, element, serialize, deserialize)
        })
        .collect();
    let constructions: Vec<TokenStream> = group
        .constructions
        .iter()
        .filter_map(|c| own_construction(group, c))
        .collect();
    let (sum_linearizers, product_linearizers) = structural_linearizers(group);
    let bind_constructions: Vec<TokenStream> = group
        .constructions
        .iter()
        .filter_map(|c| bind_construction(group, c))
        .collect();
    let linearizers: Vec<TokenStream> = group
        .constructions
        .iter()
        .map(|construction| linearizer(group, construction))
        .collect();
    let group_linearizer = inverse_group_linearizer(group);
    let category_linearizers = inverse_category_linearizers(group);
    let target_linearizers = inverse_target_linearizers(group);
    let typed_feature_reducers = typed_feature_reducers(group);
    let erased_element_builders: Vec<TokenStream> = group
        .elements
        .iter()
        .map(|element| erased_element_builders(group, element))
        .collect();
    let erased_field_builders: Vec<TokenStream> = group
        .constructions
        .iter()
        .flat_map(|construction| construction_field_builders(group, construction))
        .collect();
    let erased_construction_builders: Vec<TokenStream> = group
        .constructions
        .iter()
        .map(|construction| erased_construction_builder(group, construction))
        .collect();
    let erased_construction_projectors: Vec<TokenStream> = group
        .constructions
        .iter()
        .filter_map(erased_construction_projector)
        .collect();
    let erased_construction_linearizers: Vec<TokenStream> = group
        .constructions
        .iter()
        .map(|construction| erased_construction_linearizer(group, construction))
        .collect();
    let erased_form_recognizers: Vec<TokenStream> = group
        .constructions
        .iter()
        .flat_map(erased_form_recognizers)
        .collect();
    let deserialize_impls: Vec<TokenStream> = group
        .constructions
        .iter()
        .filter_map(|construction| deserialize_impl(group, construction))
        .collect();
    let declaration = declaration_static(group);
    let reexports = reexports(group);
    let free_witness_types: Vec<TokenStream> = group
        .constructions
        .iter()
        .flat_map(|c| c.witnesses.iter())
        .filter_map(|w| match &w.class {
            crate::model::WitnessClass::Free { ty } => Some(parse_type(&ty.value)),
            _ => None,
        })
        .collect();
    let witness_assertions = if free_witness_types.is_empty() {
        quote! {}
    } else {
        quote! {
            #[allow(dead_code, reason = "the trait bound on assert_payload is the check; nothing calls this")]
            fn __assert_free_witness_payloads() {
                fn assert_payload<T: ::deckmaste_features::SurfaceWitnessPayload>() {}
                #(assert_payload::<#free_witness_types>();)*
            }
        }
    };
    quote! {
        mod #module {
            #![allow(
                clippy::nonminimal_bool,
                reason = "the `if !(…)` wrapper negates arbitrary author predicates \
                          uniformly, including `matches!` and nested `&&`/`||` trees; \
                          a De Morgan pass would add a whole rewriting layer for zero \
                          semantic gain"
            )]
            #![allow(
                clippy::must_use_candidate,
                reason = "accessors are generated per declared field, not chosen"
            )]
            #![allow(
                clippy::missing_errors_doc,
                reason = "try_new's error conditions are the declaration's `require` \
                          clauses, which the author wrote and can read"
            )]
            #![allow(
                clippy::borrowed_box,
                reason = "bound-element destructurers preserve the declaration's exact payload type"
            )]
            #![allow(
                clippy::too_many_lines,
                reason = "a total generated linearizer grows with the number and size of declared forms"
            )]
            #![allow(
                clippy::needless_borrow,
                reason = "uniform generated field access composes direct, optional, and sequence-member references"
            )]
            use super::*;
            #(#elements)*
            #(#sum_linearizers)*
            #(#product_linearizers)*
            #(#constructions)*
            #(#bind_constructions)*
            #(#linearizers)*
            #group_linearizer
            #(#category_linearizers)*
            #(#target_linearizers)*
            #(#typed_feature_reducers)*
            #(#erased_field_builders)*
            #(#erased_element_builders)*
            #(#erased_construction_builders)*
            #(#erased_construction_projectors)*
            #(#erased_construction_linearizers)*
            #(#erased_form_recognizers)*
            #witness_assertions
            #(#deserialize_impls)*
            #declaration
        }
        #(#reexports)*
    }
}

fn structural_linearizers(group: &GroupDeclaration) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let sum_reached = sum_reached_elements(group);
    let sum = group
        .elements
        .iter()
        .filter(|element| sum_reached.contains(&element.name.value))
        .map(|element| sum_element_linearizer(group, element))
        .collect();
    let product_reached = product_reached_elements(group);
    let product = group
        .elements
        .iter()
        .filter(|element| product_reached.contains(&element.name.value))
        .map(|element| product_element_linearizer(group, element))
        .collect();
    (sum, product)
}

fn typed_feature_reducers(group: &GroupDeclaration) -> Vec<TokenStream> {
    let mut outputs = Vec::<(String, &crate::model::Spanned<String>)>::new();
    for construction in &group.constructions {
        for constraint in &construction.constraints {
            let crate::model::Constraint::DeriveFeature {
                target,
                feature_type: Some(feature_type),
                ..
            } = constraint
            else {
                continue;
            };
            let target = target.dotted();
            if !outputs.iter().any(|(candidate, _)| *candidate == target) {
                outputs.push((target, feature_type));
            }
        }
    }
    outputs
        .into_iter()
        .map(|(target, feature_type)| typed_feature_reducer(group, &target, feature_type))
        .collect()
}

fn typed_feature_reducer(
    group: &GroupDeclaration,
    target: &str,
    feature_type: &crate::model::Spanned<String>,
) -> TokenStream {
    let typed = group
        .constructions
        .iter()
        .enumerate()
        .filter_map(|(index, construction)| {
            construction.constraints.iter().find_map(|constraint| {
                let crate::model::Constraint::DeriveFeature {
                    target: candidate,
                    feature_type: Some(_),
                    combinator,
                    args,
                } = constraint
                else {
                    return None;
                };
                (candidate.dotted() == target).then_some((index, construction, combinator, args))
            })
        })
        .collect::<Vec<_>>();
    let feature_type = parse_type(&feature_type.value);
    let target = target.replace('.', "_");
    let function = quote::format_ident!("reduce_{}_{}", group.name.value, target);
    let arms = typed
        .into_iter()
        .map(|(index, construction, combinator, args)| {
            let index = proc_macro2::Literal::usize_unsuffixed(index);
            let callback = parse_type(&combinator.value);
            let fields = construction.ast.fields();
            let args = args.iter().map(|arg| {
                let field = arg
                    .segments
                    .first()
                    .expect("validated typed feature arguments are nonempty");
                let field_index = fields
                    .iter()
                    .position(|binding| binding.field.value == field.value)
                    .expect("validated typed feature arguments name construction fields");
                let index = proc_macro2::Literal::usize_unsuffixed(field_index);
                if matches!(
                    fields[field_index].kind,
                    crate::model::FieldKind::Optional { .. }
                ) {
                    quote! { fields.get(#index).copied().flatten() }
                } else {
                    quote! { fields.get(#index).copied().flatten()? }
                }
            });
            quote! { #index => #callback(#(#args),*), }
        });
    quote! {
        pub fn #function(
            construction: usize,
            fields: &[Option<&#feature_type>],
        ) -> Option<#feature_type> {
            match construction {
                #(#arms)*
                _ => None,
            }
        }
    }
}

fn inverse_group_linearizer(group: &GroupDeclaration) -> TokenStream {
    let Some(family) = group.inverse_dispatch_family() else {
        return quote! {};
    };
    let function = quote::format_ident!("linearize_{}_group_with", group.name.value);
    inverse_family_linearizer(group, family, &function)
}

fn inverse_category_linearizers(group: &GroupDeclaration) -> Vec<TokenStream> {
    group
        .inverse_dispatch_families()
        .into_iter()
        .map(|family| {
            let category = crate::model::snake_case(
                &family
                    .category
                    .expect("category families carry their declared category")
                    .value,
            );
            let function = quote::format_ident!("linearize_{}_{}_with", group.name.value, category);
            inverse_family_linearizer(group, family, &function)
        })
        .collect()
}

fn inverse_target_linearizers(group: &GroupDeclaration) -> Vec<TokenStream> {
    group
        .inverse_target_dispatch_families()
        .into_iter()
        .map(|family| {
            let function = quote::format_ident!(
                "{}",
                crate::model::inverse_target_dispatcher_name(
                    &group.name.value,
                    &family.target.value,
                )
            );
            inverse_family_linearizer(group, family, &function)
        })
        .collect()
}

fn inverse_family_linearizer(
    group: &GroupDeclaration,
    family: crate::model::InverseDispatchFamily<'_>,
    function: &proc_macro2::Ident,
) -> TokenStream {
    let target = parse_type(&family.target.value);
    let group_name = group.name.value.as_str();
    let constructions = group
        .constructions
        .iter()
        .filter(|construction| family.contains(construction))
        .collect::<Vec<_>>();
    let selections: Vec<TokenStream> = constructions
        .iter()
        .enumerate()
        .map(|(index, construction)| {
            let id = construction.id.value.as_str();
            let index = proc_macro2::Literal::usize_unsuffixed(index);
            let condition = inverse_selection_condition(family, construction);
            quote! {
                if #condition {
                    if let Some((_, first)) = selected_construction {
                        return Err(
                            ::deckmaste_construction_compiler::runtime::LinearizationError::MultipleMatchingConstructions {
                                group: #group_name,
                                first,
                                second: #id,
                            },
                        );
                    }
                    selected_construction = Some((#index, #id));
                }
            }
        })
        .collect();
    let arms: Vec<TokenStream> = constructions
        .iter()
        .enumerate()
        .map(|(index, construction)| {
            let index = proc_macro2::Literal::usize_unsuffixed(index);
            let linearizer = quote::format_ident!("linearize_{}_with", construction.id.value);
            quote! { #index => #linearizer(value, visitor), }
        })
        .collect();

    quote! {
        pub fn #function<V>(
            value: &#target,
            visitor: &mut V,
        ) -> Result<(), ::deckmaste_construction_compiler::runtime::LinearizationError<V::Error>>
        where
            V: ::deckmaste_construction_compiler::runtime::LinearizationVisitor,
        {
            let mut selected_construction: Option<(usize, &'static str)> = None;
            #(#selections)*
            let Some((selected_construction, _)) = selected_construction else {
                return Err(
                    ::deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                        group: #group_name,
                    },
                );
            };
            match selected_construction {
                #(#arms)*
                _ => unreachable!("generated construction index belongs to the declaration"),
            }
        }
    }
}

fn inverse_selection_condition(
    family: crate::model::InverseDispatchFamily<'_>,
    construction: &ConstructionDeclaration,
) -> TokenStream {
    match family.kind {
        InverseDispatchKind::ValueGuard => {
            let conditions = construction.forms.iter().map(|form| {
                let predicate = parse_type(
                    &form
                        .inverse_guard
                        .as_ref()
                        .or(form.value_guard.as_ref())
                        .expect("inverse-dispatch forms carry recognizers")
                        .value,
                );
                quote! { #predicate(value) }
            });
            conditions.fold(quote! { false }, |condition, next| {
                quote! { #condition || #next }
            })
        }
        InverseDispatchKind::LensParts => inverse_lens_selection_condition(construction),
        InverseDispatchKind::Mixed if construction.lens.is_some() => {
            inverse_lens_selection_condition(construction)
        }
        InverseDispatchKind::Mixed => {
            let conditions = construction.forms.iter().map(|form| {
                let predicate = parse_type(
                    &form
                        .inverse_guard
                        .as_ref()
                        .or(form.value_guard.as_ref())
                        .expect("mixed inverse-dispatch adapters carry recognizers")
                        .value,
                );
                quote! { #predicate(value) }
            });
            conditions.fold(quote! { false }, |condition, next| {
                quote! { #condition || #next }
            })
        }
    }
}

fn inverse_lens_selection_condition(construction: &ConstructionDeclaration) -> TokenStream {
    let parts = quote::format_ident!("parts_{}", construction.id.value);
    let guarded_domain = construction
        .forms
        .iter()
        .map(|form| {
            form.inverse_guard
                .as_ref()
                .or(form.value_guard.as_ref())
                .map(|guard| {
                    let predicate = parse_type(&guard.value);
                    quote! { #predicate(value) }
                })
        })
        .collect::<Option<Vec<_>>>()
        .filter(|_| construction.forms.iter().all(|form| !form.fallback))
        .map_or_else(
            || quote! { true },
            |conditions| {
                conditions
                    .into_iter()
                    .fold(quote! { false }, |condition, next| {
                        quote! { #condition || #next }
                    })
            },
        );
    quote! { #parts(value).is_some() && (#guarded_domain) }
}

fn erased_form_recognizers(construction: &ConstructionDeclaration) -> Vec<TokenStream> {
    let target = match &construction.ast {
        AstShape::Own { name, .. } => parse_type(&name.value),
        AstShape::Bind { path, .. } => parse_type(&path.value),
    };
    construction
        .forms
        .iter()
        .filter_map(|form| {
            let guard = form.value_guard.as_ref()?;
            let predicate = parse_type(&guard.value);
            let function = quote::format_ident!(
                "__erased_recognize_{}_{}",
                construction.id.value,
                form.ordinal.value,
            );
            Some(quote! {
                fn #function(value: &dyn ::std::any::Any) -> bool {
                    value.downcast_ref::<#target>().is_some_and(#predicate)
                }
            })
        })
        .collect()
}

/// BFS over declared structural fields reached from selected own constructions.
/// Membership-only: element **emission** order stays declaration order in
/// `emit_group`, unaffected by `sort_unstable()` below.
fn trait_reached_elements(
    group: &GroupDeclaration,
    selected: impl Fn(&ConstructionDeclaration) -> bool,
) -> Vec<String> {
    let mut reached: Vec<String> = Vec::new();
    let mut queue: Vec<&str> = Vec::new();
    for construction in &group.constructions {
        if !selected(construction) {
            continue;
        }
        for binding in construction.ast.fields() {
            enqueue_trait_elements(&binding.kind, &mut queue);
        }
    }
    while let Some(name) = queue.pop() {
        if reached.iter().any(|r| r == name) {
            continue;
        }
        reached.push(name.to_owned());
        if let Some(element) = group.elements.iter().find(|e| e.name.value == name) {
            for binding in &element.fields {
                enqueue_trait_elements(&binding.kind, &mut queue);
            }
            for variant in &element.variants {
                enqueue_trait_elements(&variant.payload, &mut queue);
            }
        }
    }
    reached.sort_unstable(); // deterministic regardless of discovery order
    reached
}

fn sum_reached_elements(group: &GroupDeclaration) -> Vec<String> {
    let mut reached = Vec::new();
    for construction in &group.constructions {
        for field in construction.ast.fields() {
            collect_sum_references(&field.kind, &mut reached);
        }
    }
    for element in &group.elements {
        for field in &element.fields {
            collect_sum_references(&field.kind, &mut reached);
        }
        for variant in &element.variants {
            collect_sum_references(&variant.payload, &mut reached);
        }
    }
    reached.sort_unstable();
    reached.dedup();
    reached
}

fn product_reached_elements(group: &GroupDeclaration) -> Vec<String> {
    let mut reached = Vec::new();
    for construction in &group.constructions {
        for field in construction.ast.fields() {
            collect_product_references(&field.kind, &mut reached);
        }
    }
    for element in &group.elements {
        for field in &element.fields {
            collect_product_references(&field.kind, &mut reached);
        }
        for variant in &element.variants {
            collect_product_references(&variant.payload, &mut reached);
        }
    }
    reached.sort_unstable();
    reached.dedup();
    reached
}

fn collect_sum_references(kind: &FieldKind, reached: &mut Vec<String>) {
    match kind {
        FieldKind::Sum { element, .. } => reached.push(element.value.clone()),
        FieldKind::Optional { inner } => collect_sum_references(inner, reached),
        FieldKind::TupleProduct { fields } | FieldKind::StructProduct { fields } => {
            for field in fields {
                collect_sum_references(&field.kind, reached);
            }
        }
        FieldKind::Unit
        | FieldKind::Identity { .. }
        | FieldKind::Subtree { .. }
        | FieldKind::TypedSubtree { .. }
        | FieldKind::Scalar { .. }
        | FieldKind::TypedScalar { .. }
        | FieldKind::SurfaceScalar { .. }
        | FieldKind::Sequence { .. }
        | FieldKind::NonEmptySequence { .. }
        | FieldKind::SeparatedNonEmptySequence { .. }
        | FieldKind::Product { .. } => {}
    }
}

fn collect_product_references(kind: &FieldKind, reached: &mut Vec<String>) {
    match kind {
        FieldKind::Product { element, .. } => reached.push(element.value.clone()),
        FieldKind::Optional { inner } => collect_product_references(inner, reached),
        FieldKind::TupleProduct { fields } | FieldKind::StructProduct { fields } => {
            for field in fields {
                collect_product_references(&field.kind, reached);
            }
        }
        FieldKind::Unit
        | FieldKind::Identity { .. }
        | FieldKind::Subtree { .. }
        | FieldKind::TypedSubtree { .. }
        | FieldKind::Scalar { .. }
        | FieldKind::TypedScalar { .. }
        | FieldKind::SurfaceScalar { .. }
        | FieldKind::Sequence { .. }
        | FieldKind::NonEmptySequence { .. }
        | FieldKind::SeparatedNonEmptySequence { .. }
        | FieldKind::Sum { .. } => {}
    }
}

fn enqueue_trait_elements<'g>(kind: &'g FieldKind, queue: &mut Vec<&'g str>) {
    match kind {
        FieldKind::Sequence { element }
        | FieldKind::NonEmptySequence { element }
        | FieldKind::SeparatedNonEmptySequence { element, .. }
        | FieldKind::Sum { element, .. }
        | FieldKind::Product { element, .. } => {
            queue.push(element.value.as_str());
        }
        FieldKind::Optional { inner } => enqueue_trait_elements(inner, queue),
        FieldKind::TupleProduct { fields } | FieldKind::StructProduct { fields } => {
            for field in fields {
                enqueue_trait_elements(&field.kind, queue);
            }
        }
        FieldKind::Unit
        | FieldKind::Identity { .. }
        | FieldKind::Subtree { .. }
        | FieldKind::TypedSubtree { .. }
        | FieldKind::Scalar { .. }
        | FieldKind::TypedScalar { .. }
        | FieldKind::SurfaceScalar { .. } => {}
    }
}

fn element_item(
    group: &GroupDeclaration,
    element: &ElementDeclaration,
    serialize: bool,
    deserialize: bool,
) -> TokenStream {
    let Some(bind_path) = &element.bind_path else {
        return owned_element_struct(group, element, serialize, deserialize);
    };
    let target = parse_type(&bind_path.value);
    if !element.variants.is_empty() {
        return bound_enum_element(group, element, &target);
    }
    if element.fields.is_empty() {
        return quote! {
            const _: fn(&#target) = |_| {};
        };
    }
    let parts = quote::format_ident!("__parts_{}", element.name.value);
    let semantic_fields = element
        .fields
        .iter()
        .filter(|binding| !matches!(binding.kind, FieldKind::SurfaceScalar { .. }))
        .collect::<Vec<_>>();
    let field_names: Vec<proc_macro2::Ident> = semantic_fields
        .iter()
        .map(|binding| quote::format_ident!("{}", binding.field.value))
        .collect();
    let field_types: Vec<TokenStream> = semantic_fields
        .iter()
        .map(|binding| field_type(group, &binding.kind))
        .collect();
    quote! {
        fn #parts(value: &#target) -> (#(&#field_types),*) {
            let #target { #(#field_names),* } = value;
            (#(#field_names),*)
        }
        const _: fn(&#target) -> (#(&#field_types),*) = #parts;
    }
}

fn bound_enum_element(
    group: &GroupDeclaration,
    element: &ElementDeclaration,
    target: &TokenStream,
) -> TokenStream {
    let view = quote::format_ident!(
        "{}VariantRef",
        crate::model::pascal_case(&element.name.value)
    );
    let borrows_payload = element
        .variants
        .iter()
        .any(|variant| !matches!(variant.payload, FieldKind::Unit));
    let parts = quote::format_ident!("parts_{}", element.name.value);
    let view_variants: Vec<TokenStream> = element
        .variants
        .iter()
        .map(|variant| {
            let name = quote::format_ident!("{}", variant.name.value);
            match &variant.payload {
                FieldKind::Unit => quote! { #name, },
                FieldKind::TupleProduct { fields } | FieldKind::StructProduct { fields } => {
                    let fields = fields.iter().map(|field| {
                        let field_name = quote::format_ident!("{}", field.field.value);
                        let field_type = field_type(group, &field.kind);
                        quote! { #field_name: &'a #field_type }
                    });
                    quote! { #name { #(#fields),* }, }
                }
                payload => {
                    let payload = field_type(group, payload);
                    quote! { #name(&'a #payload), }
                }
            }
        })
        .collect();
    let match_arms: Vec<TokenStream> = element
        .variants
        .iter()
        .map(|variant| {
            let name = quote::format_ident!("{}", variant.name.value);
            match &variant.payload {
                FieldKind::Unit => quote! { #target::#name => #view::#name, },
                FieldKind::TupleProduct { fields } => {
                    let fields = fields
                        .iter()
                        .map(|field| quote::format_ident!("{}", field.field.value))
                        .collect::<Vec<_>>();
                    quote! {
                        #target::#name(#(#fields),*) => #view::#name { #(#fields),* },
                    }
                }
                FieldKind::StructProduct { fields } => {
                    let fields = fields
                        .iter()
                        .map(|field| quote::format_ident!("{}", field.field.value))
                        .collect::<Vec<_>>();
                    quote! {
                        #target::#name { #(#fields),* } => #view::#name { #(#fields),* },
                    }
                }
                _ => quote! { #target::#name(payload) => #view::#name(payload), },
            }
        })
        .collect();
    let builders: Vec<TokenStream> = element
        .variants
        .iter()
        .map(|variant| {
            let variant_name = quote::format_ident!("{}", variant.name.value);
            let builder = quote::format_ident!(
                "build_{}_{}",
                element.name.value,
                crate::model::snake_case(&variant.name.value),
            );
            match &variant.payload {
                FieldKind::Unit => quote! {
                    pub fn #builder() -> #target {
                        #target::#variant_name
                    }
                },
                FieldKind::TupleProduct { fields } => {
                    let args = fields.iter().map(|field| {
                        let field_name = quote::format_ident!("{}", field.field.value);
                        let field_type = field_type(group, &field.kind);
                        quote! { #field_name: #field_type }
                    });
                    let fields = fields
                        .iter()
                        .map(|field| quote::format_ident!("{}", field.field.value));
                    quote! {
                        pub fn #builder(#(#args),*) -> #target {
                            #target::#variant_name(#(#fields),*)
                        }
                    }
                }
                FieldKind::StructProduct { fields } => {
                    let args = fields.iter().map(|field| {
                        let field_name = quote::format_ident!("{}", field.field.value);
                        let field_type = field_type(group, &field.kind);
                        quote! { #field_name: #field_type }
                    });
                    let fields = fields
                        .iter()
                        .map(|field| quote::format_ident!("{}", field.field.value));
                    quote! {
                        pub fn #builder(#(#args),*) -> #target {
                            #target::#variant_name { #(#fields),* }
                        }
                    }
                }
                payload => {
                    let payload = field_type(group, payload);
                    quote! {
                        pub fn #builder(payload: #payload) -> #target {
                            #target::#variant_name(payload)
                        }
                    }
                }
            }
        })
        .collect();
    let view_declaration = if borrows_payload {
        quote! {
            pub enum #view<'a> {
                #(#view_variants)*
            }
        }
    } else {
        quote! {
            pub enum #view {
                #(#view_variants)*
            }
        }
    };
    let view_return = if borrows_payload {
        quote! { #view<'_> }
    } else {
        quote! { #view }
    };
    quote! {
        #view_declaration

        pub fn #parts(value: &#target) -> #view_return {
            match value {
                #(#match_arms)*
            }
        }

        #(#builders)*
    }
}

fn owned_element_struct(
    group: &GroupDeclaration,
    element: &ElementDeclaration,
    serialize: bool,
    deserialize: bool,
) -> TokenStream {
    let name = pascal_ident(&element.name.value);
    let fields: Vec<TokenStream> = element
        .fields
        .iter()
        .filter(|binding| !matches!(binding.kind, FieldKind::SurfaceScalar { .. }))
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let ty = field_type(group, &binding.kind);
            quote! { pub #field: #ty, }
        })
        .collect();
    // Reached (transitively, via `seq`) by at least one serde-opt-in own
    // construction: the element needs the corresponding structural trait.
    let serialize_derive = if serialize {
        quote! { #[derive(serde::Serialize)] }
    } else {
        quote! {}
    };
    let deserialize_derive = if deserialize {
        quote! { #[derive(serde::Deserialize)] }
    } else {
        quote! {}
    };
    quote! {
        // Elements carry no declaration invariants, so their fields stay
        // public; the owning construction validates the sequence whole.
        #[derive(Debug, PartialEq, Eq)]
        #serialize_derive
        #deserialize_derive
        pub struct #name {
            #(#fields)*
        }
    }
}

fn erased_field_reads(
    owner: &str,
    fields: &[FieldBinding],
    group: &GroupDeclaration,
) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|binding| {
            let field = if matches!(binding.kind, FieldKind::SurfaceScalar { .. }) {
                quote::format_ident!("_{}", binding.field.value)
            } else {
                quote::format_ident!("{}", binding.field.value)
            };
            let field_name = binding.field.value.as_str();
            let ty = field_type(group, &binding.kind);
            quote! {
                let #field: #ty =
                    ::deckmaste_construction_compiler::runtime::take_erased(
                        &mut values,
                        #owner,
                        #field_name,
                        stringify!(#ty),
                    )?;
            }
        })
        .collect()
}

fn construction_field_builders(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> Vec<TokenStream> {
    let owner = construction.id.value.as_str();
    construction
        .ast
        .fields()
        .iter()
        .filter_map(|binding| {
            let field = binding.field.value.as_str();
            let function = quote::format_ident!(
                "__erased_field_{}_{}",
                construction.id.value,
                binding.field.value,
            );
            match &binding.kind {
                FieldKind::NonEmptySequence { element } => {
                    let member = element_type(group, element);
                    Some(quote! {
                        fn #function(
                            values: Vec<::deckmaste_construction_compiler::runtime::ErasedValue>,
                        ) -> Result<
                            ::deckmaste_construction_compiler::runtime::ErasedValue,
                            ::deckmaste_construction_compiler::runtime::ErasedBuildError,
                        > {
                            let mut members = Vec::<#member>::with_capacity(values.len());
                            for value in values {
                                members.push(*value.downcast::<#member>().map_err(|_| {
                                    ::deckmaste_construction_compiler::runtime::ErasedBuildError::WrongFieldType {
                                        owner: #owner,
                                        field: #field,
                                        expected: stringify!(#member),
                                    }
                                })?);
                            }
                            let sequence = ::deckmaste_construction_compiler::runtime::NonEmpty::try_from(members)
                                .map_err(|_| ::deckmaste_construction_compiler::runtime::ErasedBuildError::EmptySequence {
                                    owner: #owner,
                                    field: #field,
                                })?;
                            Ok(Box::new(sequence))
                        }
                    })
                }
                FieldKind::SeparatedNonEmptySequence { element, separator } => {
                    let member = element_type(group, element);
                    let separator = parse_type(&separator.value);
                    Some(quote! {
                        fn #function(
                            values: Vec<::deckmaste_construction_compiler::runtime::ErasedValue>,
                        ) -> Result<
                            ::deckmaste_construction_compiler::runtime::ErasedValue,
                            ::deckmaste_construction_compiler::runtime::ErasedBuildError,
                        > {
                            let mut values = values.into_iter();
                            let first: #member = ::deckmaste_construction_compiler::runtime::take_erased(
                                &mut values,
                                #owner,
                                #field,
                                stringify!(#member),
                            ).map_err(|error| match error {
                                ::deckmaste_construction_compiler::runtime::ErasedBuildError::MissingField { .. } =>
                                    ::deckmaste_construction_compiler::runtime::ErasedBuildError::EmptySequence {
                                        owner: #owner,
                                        field: #field,
                                    },
                                other => other,
                            })?;
                            let mut rest = Vec::new();
                            while let Some(separator_value) = values.next() {
                                let separator = *separator_value.downcast::<#separator>().map_err(|_| {
                                    ::deckmaste_construction_compiler::runtime::ErasedBuildError::WrongFieldType {
                                        owner: #owner,
                                        field: #field,
                                        expected: stringify!(#separator),
                                    }
                                })?;
                                let value: #member = ::deckmaste_construction_compiler::runtime::take_erased(
                                    &mut values,
                                    #owner,
                                    #field,
                                    stringify!(#member),
                                )?;
                                rest.push(::deckmaste_construction_compiler::runtime::Separated::new(
                                    separator,
                                    value,
                                ));
                            }
                            Ok(Box::new(
                                ::deckmaste_construction_compiler::runtime::SeparatedNonEmpty::new(
                                    first,
                                    rest,
                                ),
                            ))
                        }
                    })
                }
                _ => None,
            }
        })
        .collect()
}

fn erased_element_builders(group: &GroupDeclaration, element: &ElementDeclaration) -> TokenStream {
    let owner = element.name.value.as_str();
    let target = element.bind_path.as_ref().map_or_else(
        || {
            let element_type = pascal_ident(&element.name.value);
            quote! { #element_type }
        },
        |path| parse_type(&path.value),
    );
    let sequence_function = quote::format_ident!("__erased_sequence_{}", owner);
    let sequence_builder = quote! {
        fn #sequence_function(
            values: Vec<::deckmaste_construction_compiler::runtime::ErasedValue>,
        ) -> Result<
            ::deckmaste_construction_compiler::runtime::ErasedValue,
            ::deckmaste_construction_compiler::runtime::ErasedBuildError,
        > {
            let mut result = Vec::<#target>::with_capacity(values.len());
            for value in values {
                result.push(*value.downcast::<#target>().map_err(|_| {
                    ::deckmaste_construction_compiler::runtime::ErasedBuildError::WrongFieldType {
                        owner: #owner,
                        field: "sequence member",
                        expected: stringify!(#target),
                    }
                })?);
            }
            Ok(Box::new(result))
        }
    };
    if element.bind_path.is_some() && element.fields.is_empty() && element.variants.is_empty() {
        return sequence_builder;
    }
    if !element.variants.is_empty() {
        let builders = element.variants.iter().map(|variant| {
            let suffix = crate::model::snake_case(&variant.name.value);
            let function = quote::format_ident!("__erased_build_{}_{}", owner, suffix);
            let variant_name = quote::format_ident!("{}", variant.name.value);
            let (reads, construct) = match &variant.payload {
                FieldKind::Unit => (Vec::new(), quote! { #target::#variant_name }),
                FieldKind::TupleProduct { fields } => {
                    let reads = erased_field_reads(owner, fields, group);
                    let names = fields
                        .iter()
                        .map(|field| quote::format_ident!("{}", field.field.value));
                    (reads, quote! { #target::#variant_name(#(#names),*) })
                }
                FieldKind::StructProduct { fields } => {
                    let reads = erased_field_reads(owner, fields, group);
                    let names = fields
                        .iter()
                        .map(|field| quote::format_ident!("{}", field.field.value));
                    (reads, quote! { #target::#variant_name { #(#names),* } })
                }
                payload => {
                    let payload = field_type(group, payload);
                    let read = quote! {
                        let payload: #payload =
                            ::deckmaste_construction_compiler::runtime::take_erased(
                                &mut values,
                                #owner,
                                "payload",
                                stringify!(#payload),
                            )?;
                    };
                    (vec![read], quote! { #target::#variant_name(payload) })
                }
            };
            quote! {
                fn #function(
                    values: Vec<::deckmaste_construction_compiler::runtime::ErasedValue>,
                ) -> Result<
                    ::deckmaste_construction_compiler::runtime::ErasedValue,
                    ::deckmaste_construction_compiler::runtime::ErasedBuildError,
                > {
                    let mut values = values.into_iter();
                    #(#reads)*
                    if values.next().is_some() {
                        return Err(::deckmaste_construction_compiler::runtime::ErasedBuildError::ExtraFields {
                            owner: #owner,
                        });
                    }
                    Ok(Box::new(#construct))
                }
            }
        });
        return quote! { #(#builders)* #sequence_builder };
    }

    let function = quote::format_ident!("__erased_build_{}", owner);
    let reads = erased_field_reads(owner, &element.fields, group);
    let fields: Vec<proc_macro2::Ident> = element
        .fields
        .iter()
        .filter(|binding| !matches!(binding.kind, FieldKind::SurfaceScalar { .. }))
        .map(|binding| quote::format_ident!("{}", binding.field.value))
        .collect();
    quote! {
        fn #function(
            values: Vec<::deckmaste_construction_compiler::runtime::ErasedValue>,
        ) -> Result<
            ::deckmaste_construction_compiler::runtime::ErasedValue,
            ::deckmaste_construction_compiler::runtime::ErasedBuildError,
        > {
            let mut values = values.into_iter();
            #(#reads)*
            if values.next().is_some() {
                return Err(::deckmaste_construction_compiler::runtime::ErasedBuildError::ExtraFields {
                    owner: #owner,
                });
            }
            Ok(Box::new(#target { #(#fields),* }))
        }
        #sequence_builder
    }
}

fn erased_construction_builder(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> TokenStream {
    let owner = construction.id.value.as_str();
    let partial_function = quote::format_ident!("__erased_partial_build_{}", construction.id.value);
    let function = quote::format_ident!("__erased_build_{}", construction.id.value);
    let fields = construction.ast.fields();
    let reads = erased_field_reads(owner, fields, group);
    let partial_reads = reads.clone();
    let final_reads = reads;
    let field_names: Vec<proc_macro2::Ident> = fields
        .iter()
        .map(|binding| quote::format_ident!("{}", binding.field.value))
        .collect();
    let partial_build = match &construction.ast {
        AstShape::Own { name, .. } => {
            let target = quote::format_ident!("{}", name.value);
            quote! { #target::try_new(#(#field_names),*) }
        }
        AstShape::Bind { path, .. } => {
            let target = parse_type(&path.value);
            let checks: Vec<TokenStream> = construction
                .constraints
                .iter()
                .filter_map(|constraint| match constraint {
                    Constraint::Require(predicate) => {
                        Some(require_check(owner, group, fields, &predicate.value))
                    }
                    Constraint::Recognize(_) | Constraint::DeriveFeature { .. } => None,
                })
                .collect();
            let construct = if let Some(application) = &construction.lens {
                let lens = group
                    .lenses
                    .iter()
                    .find(|lens| lens.name.value == application.owner.value)
                    .expect("validated: EC016 rejects an undeclared lens owner");
                lens_construct(lens, application, &target, owner)
            } else if let Some(adapter) = &construction.bind_adapter {
                let constructor = parse_type(&adapter.constructor.value);
                quote! { #constructor(#(#field_names),*) }
            } else {
                quote! { Ok(#target { #(#field_names),* }) }
            };
            quote! {
                (|| -> Result<
                    #target,
                    ::deckmaste_construction_compiler::runtime::DeclarationViolation,
                > {
                    #(#checks)*
                    #construct
                })()
            }
        }
    };
    let final_build = match &construction.ast {
        AstShape::Own { name, .. } => {
            let target = quote::format_ident!("{}", name.value);
            quote! { #target::try_new(#(#field_names),*) }
        }
        AstShape::Bind { .. } => {
            let build_fn = quote::format_ident!("build_{}", construction.id.value);
            quote! { #build_fn(#(#field_names),*) }
        }
    };
    quote! {
        fn #partial_function(
            values: Vec<::deckmaste_construction_compiler::runtime::ErasedValue>,
        ) -> Result<
            ::deckmaste_construction_compiler::runtime::ErasedValue,
            ::deckmaste_construction_compiler::runtime::ErasedBuildError,
        > {
            let mut values = values.into_iter();
            #(#partial_reads)*
            if values.next().is_some() {
                return Err(::deckmaste_construction_compiler::runtime::ErasedBuildError::ExtraFields {
                    owner: #owner,
                });
            }
            #partial_build
                .map(|value| Box::new(value) as ::deckmaste_construction_compiler::runtime::ErasedValue)
                .map_err(::deckmaste_construction_compiler::runtime::ErasedBuildError::Declaration)
        }

        fn #function(
            values: Vec<::deckmaste_construction_compiler::runtime::ErasedValue>,
        ) -> Result<
            ::deckmaste_construction_compiler::runtime::ErasedValue,
            ::deckmaste_construction_compiler::runtime::ErasedBuildError,
        > {
            let mut values = values.into_iter();
            #(#final_reads)*
            if values.next().is_some() {
                return Err(::deckmaste_construction_compiler::runtime::ErasedBuildError::ExtraFields {
                    owner: #owner,
                });
            }
            #final_build
                .map(|value| Box::new(value) as ::deckmaste_construction_compiler::runtime::ErasedValue)
                .map_err(::deckmaste_construction_compiler::runtime::ErasedBuildError::Declaration)
        }
    }
}

fn erased_construction_projector(construction: &ConstructionDeclaration) -> Option<TokenStream> {
    construction.projection.as_ref()?;
    let owner = construction.id.value.as_str();
    let function = quote::format_ident!("__erased_project_{}", construction.id.value);
    let category = quote::format_ident!("{}", construction.category.value);
    let source = match &construction.ast {
        AstShape::Own { name, .. } => parse_type(&name.value),
        AstShape::Bind { path, .. } => parse_type(&path.value),
    };
    Some(quote! {
        fn #function(
            value: ::deckmaste_construction_compiler::runtime::ErasedValue,
        ) -> Result<
            ::deckmaste_construction_compiler::runtime::ErasedValue,
            ::deckmaste_construction_compiler::runtime::ErasedBuildError,
        > {
            let value = value.downcast::<#source>().map_err(|_| {
                ::deckmaste_construction_compiler::runtime::ErasedBuildError::WrongFieldType {
                    owner: #owner,
                    field: "projection",
                    expected: stringify!(#source),
                }
            })?;
            Ok(Box::new(#category::from(*value)))
        }
    })
}

fn erased_construction_linearizer(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> TokenStream {
    let owner = construction.id.value.as_str();
    let group_name = group.name.value.as_str();
    let function = quote::format_ident!("__erased_linearize_{}", construction.id.value);
    let linearizer = quote::format_ident!("linearize_{}_with", construction.id.value);
    let source = match &construction.ast {
        AstShape::Own { name, .. } => parse_type(&name.value),
        AstShape::Bind { path, .. } => parse_type(&path.value),
    };
    let value_binding = construction.projection_inverse.as_ref().map_or_else(
        || {
            quote! {
                let value = value.as_any().downcast_ref::<#source>().ok_or(
                    ::deckmaste_construction_compiler::runtime::ErasedLinearizationError::WrongValueType {
                        construction: #owner,
                        expected: stringify!(#source),
                    },
                )?;
            }
        },
        |inverse| {
            let inverse = parse_type(&inverse.value);
            let category = parse_type(&construction.category.value);
            quote! {
                let value = if let Some(value) = value.as_any().downcast_ref::<#source>() {
                    value
                } else {
                    let category = value.as_any().downcast_ref::<#category>().ok_or(
                        ::deckmaste_construction_compiler::runtime::ErasedLinearizationError::WrongValueType {
                            construction: #owner,
                            expected: stringify!(#category),
                        },
                    )?;
                    let Some(value) = #inverse(category) else {
                        return Err(
                            ::deckmaste_construction_compiler::runtime::ErasedLinearizationError::Linearization(
                                ::deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                                    group: #group_name,
                                },
                            ),
                        );
                    };
                    value
                };
            }
        },
    );
    let family = group
        .inverse_dispatch_families()
        .into_iter()
        .find(|family| family.contains(construction));
    let recognition = family.map_or_else(
        || quote! {},
        |family| {
            let condition = inverse_selection_condition(family, construction);
            quote! {
                if !(#condition) {
                    return Err(
                        ::deckmaste_construction_compiler::runtime::ErasedLinearizationError::Linearization(
                            ::deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                                group: #group_name,
                            },
                        ),
                    );
                }
            }
        },
    );
    quote! {
        fn #function(
            value: &dyn ::deckmaste_construction_compiler::runtime::ProjectionInput,
            sink: &mut dyn ::deckmaste_construction_compiler::runtime::ProjectionSink,
        ) -> Result<
            (),
            ::deckmaste_construction_compiler::runtime::ErasedLinearizationError,
        > {
            #value_binding
            #recognition
            let mut visitor =
                ::deckmaste_construction_compiler::runtime::ProjectionSinkAdapter::new(sink);
            #linearizer(value, &mut visitor).map_err(
                ::deckmaste_construction_compiler::runtime::ErasedLinearizationError::Linearization,
            )
        }
    }
}

fn own_construction(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> Option<TokenStream> {
    let AstShape::Own { name, fields } = &construction.ast else {
        return None; // bind mode: handled separately by `bind_construction`
    };
    let ty = quote::format_ident!("{}", name.value);
    let id = construction.id.value.as_str();
    let field_decls: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! { #field: #field_ty, }
        })
        .collect();
    let params: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! { #field: #field_ty }
        })
        .collect();
    let checks: Vec<TokenStream> = construction
        .constraints
        .iter()
        .filter_map(|constraint| match constraint {
            Constraint::Require(predicate) => {
                Some(require_check(id, group, fields, &predicate.value))
            }
            Constraint::Recognize(_) | Constraint::DeriveFeature { .. } => None,
        })
        .collect();
    let field_names: Vec<proc_macro2::Ident> = fields
        .iter()
        .map(|b| quote::format_ident!("{}", b.field.value))
        .collect();
    let accessors: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! {
                pub fn #field(&self) -> &#field_ty {
                    &self.#field
                }
            }
        })
        .collect();
    let serialize_derive = if construction.serialize {
        quote! { #[derive(serde::Serialize)] }
    } else {
        quote! {}
    };
    Some(quote! {
        #[derive(Debug, PartialEq, Eq)]
        #serialize_derive
        pub struct #ty {
            #(#field_decls)*
        }
        impl #ty {
            pub fn try_new(#(#params),*) -> Result<Self, ::deckmaste_construction_compiler::runtime::DeclarationViolation> {
                #(#checks)*
                Ok(Self { #(#field_names),* })
            }
            #(#accessors)*
        }
    })
}

fn lens_field_type(kind: &LensFieldKind) -> TokenStream {
    match kind {
        LensFieldKind::Value { value_type } => parse_type(&value_type.value),
        LensFieldKind::Optional { value_type } => {
            let value_type = parse_type(&value_type.value);
            quote! { Option<#value_type> }
        }
        LensFieldKind::Vector { element_type } => {
            let element_type = parse_type(&element_type.value);
            quote! { Vec<#element_type> }
        }
    }
}

fn lens_construct(
    lens: &crate::model::LensDeclaration,
    application: &LensApplication,
    target: &TokenStream,
    construction_id: &str,
) -> TokenStream {
    if let Some(adapter) = &lens.adapter {
        let constructor = parse_type(&adapter.constructor.value);
        let destructurer = parse_type(&adapter.destructurer.value);
        let owner_fields = lens
            .fields
            .iter()
            .map(|field| quote::format_ident!("__lens_{}", field.name.value))
            .collect::<Vec<_>>();
        if let Some(source) = &application.source {
            let source = quote::format_ident!("{}", source.value);
            let edits = application.edits.iter().map(|edit| {
                let target_field =
                    quote::format_ident!("__lens_{}", edit.target.segments[0].value);
                let value = quote::format_ident!("{}", edit.value.value);
                let value = edit.adapter.as_ref().map_or_else(
                    || quote! { #value },
                    |adapter| {
                        let constructor = parse_type(&adapter.constructor.value);
                        quote! { #constructor(&__lens_source_owner, #value)? }
                    },
                );
                match edit.kind {
                    LensEditKind::Focus => {
                        let requirement = format!("{}.is_none()", edit.target.dotted());
                        quote! {
                            if #target_field.is_some() {
                                return Err(::deckmaste_construction_compiler::runtime::DeclarationViolation {
                                    construction: #construction_id,
                                    requirement: #requirement,
                                });
                            }
                            #target_field = Some(#value);
                        }
                    }
                    LensEditKind::Prepend => quote! { #target_field.insert(0, #value); },
                    LensEditKind::Append => quote! { #target_field.push(#value); },
                }
            });
            return quote! {
                let __lens_source_owner = #source.clone();
                let (#(mut #owner_fields),*) = #destructurer(#source);
                #(#edits)*
                Ok(#constructor(#(#owner_fields),*))
            };
        }

        let values = lens.fields.iter().map(|field| {
            let edit = application
                .edits
                .iter()
                .find(|edit| edit.target.segments[0].value == field.name.value);
            match (edit, &field.kind) {
                (Some(edit), LensFieldKind::Value { .. }) => {
                    let value = quote::format_ident!("{}", edit.value.value);
                    quote! { #value }
                }
                (Some(edit), LensFieldKind::Optional { .. }) => {
                    let value = quote::format_ident!("{}", edit.value.value);
                    quote! { Some(#value) }
                }
                (None, LensFieldKind::Optional { .. }) => quote! { None },
                (None, LensFieldKind::Vector { .. }) => quote! { Vec::new() },
                _ => unreachable!("validated: EC017/EC033 admit only rebuildable focus edits"),
            }
        });
        return quote! { Ok(#constructor(#(#values),*)) };
    }

    if let Some(source) = &application.source {
        let source = quote::format_ident!("{}", source.value);
        let edits = application.edits.iter().map(|edit| {
            let target_field = quote::format_ident!("{}", edit.target.segments[0].value);
            let value = quote::format_ident!("{}", edit.value.value);
            let value = edit.adapter.as_ref().map_or_else(
                || quote! { #value },
                |adapter| {
                    let constructor = parse_type(&adapter.constructor.value);
                    quote! { #constructor(&#source, #value)? }
                },
            );
            match edit.kind {
                LensEditKind::Focus => {
                    let requirement = format!("{}.is_none()", edit.target.dotted());
                    quote! {
                        if #source.#target_field.is_some() {
                            return Err(::deckmaste_construction_compiler::runtime::DeclarationViolation {
                                construction: #construction_id,
                                requirement: #requirement,
                            });
                        }
                        #source.#target_field = Some(#value);
                    }
                }
                LensEditKind::Prepend => quote! { #source.#target_field.insert(0, #value); },
                LensEditKind::Append => quote! { #source.#target_field.push(#value); },
            }
        });
        return quote! {
            let mut #source = #source;
            #(#edits)*
            Ok(#source)
        };
    }

    let values = lens.fields.iter().map(|field| {
        let field_name = quote::format_ident!("{}", field.name.value);
        let edit = application
            .edits
            .iter()
            .find(|edit| edit.target.segments[0].value == field.name.value);
        let value = match (edit, &field.kind) {
            (Some(edit), LensFieldKind::Value { .. }) => {
                let value = quote::format_ident!("{}", edit.value.value);
                quote! { #value }
            }
            (Some(edit), LensFieldKind::Optional { .. }) => {
                let value = quote::format_ident!("{}", edit.value.value);
                quote! { Some(#value) }
            }
            (None, LensFieldKind::Optional { .. }) => quote! { None },
            (None, LensFieldKind::Vector { .. }) => quote! { Vec::new() },
            _ => unreachable!("validated: EC017/EC033 admit only rebuildable focus edits"),
        };
        quote! { #field_name: #value }
    });
    quote! { Ok(#target { #(#values),* }) }
}

#[allow(
    clippy::too_many_lines,
    reason = "source-relative and whole-owner lenses share one ordered inverse emitter"
)]
fn lens_destructure(
    lens: &crate::model::LensDeclaration,
    application: &LensApplication,
    field_names: &[proc_macro2::Ident],
) -> TokenStream {
    if let Some(adapter) = &lens.adapter {
        let constructor = parse_type(&adapter.constructor.value);
        let destructurer = parse_type(&adapter.destructurer.value);
        let owner_fields = lens
            .fields
            .iter()
            .map(|field| quote::format_ident!("__lens_{}", field.name.value))
            .collect::<Vec<_>>();
        if let Some(source) = &application.source {
            let source_ident = quote::format_ident!("{}", source.value);
            let extracts = application.edits.iter().map(|edit| {
                let target_field = quote::format_ident!("__lens_{}", edit.target.segments[0].value);
                let raw = quote::format_ident!("__lens_raw_{}", edit.value.value);
                match edit.kind {
                    LensEditKind::Focus => quote! { let #raw = #target_field.take()?; },
                    LensEditKind::Prepend => quote! {
                        if #target_field.is_empty() {
                            return None;
                        }
                        let #raw = #target_field.remove(0);
                    },
                    LensEditKind::Append => quote! { let #raw = #target_field.pop()?; },
                }
            });
            let decodes = application.edits.iter().map(|edit| {
                let value = quote::format_ident!("{}", edit.value.value);
                let raw = quote::format_ident!("__lens_raw_{}", edit.value.value);
                edit.adapter.as_ref().map_or_else(
                    || quote! { let #value = #raw; },
                    |adapter| {
                        let destructurer = parse_type(&adapter.destructurer.value);
                        quote! { let #value = #destructurer(&#source_ident, #raw)?; }
                    },
                )
            });
            return quote! {
                let (#(mut #owner_fields),*) = #destructurer(value.clone());
                #(#extracts)*
                let #source_ident = #constructor(#(#owner_fields),*);
                #(#decodes)*
                Some((#(#field_names),*))
            };
        }

        let default_checks = lens.fields.iter().filter_map(|field| {
            let claimed = application
                .edits
                .iter()
                .any(|edit| edit.target.segments[0].value == field.name.value);
            if claimed {
                return None;
            }
            let field_name = quote::format_ident!("__lens_{}", field.name.value);
            match field.kind {
                LensFieldKind::Optional { .. } => {
                    Some(quote! { if #field_name.is_some() { return None; } })
                }
                LensFieldKind::Vector { .. } => {
                    Some(quote! { if !#field_name.is_empty() { return None; } })
                }
                LensFieldKind::Value { .. } => {
                    unreachable!("validated: EC033 rejects unclaimed required owner fields")
                }
            }
        });
        let extracts = application.edits.iter().map(|edit| {
            let target_field = quote::format_ident!("__lens_{}", edit.target.segments[0].value);
            let value = quote::format_ident!("{}", edit.value.value);
            let owner_field = lens
                .fields
                .iter()
                .find(|field| field.name.value == edit.target.segments[0].value)
                .expect("validated lens target");
            match owner_field.kind {
                LensFieldKind::Value { .. } => quote! { let #value = #target_field.clone(); },
                LensFieldKind::Optional { .. } => {
                    quote! { let #value = #target_field.clone()?; }
                }
                LensFieldKind::Vector { .. } => {
                    unreachable!("validated: no-source vector edits are not focusable")
                }
            }
        });
        return quote! {
            let (#(mut #owner_fields),*) = #destructurer(value.clone());
            #(#default_checks)*
            #(#extracts)*
            Some((#(#field_names),*))
        };
    }

    if let Some(source) = &application.source {
        let source_ident = quote::format_ident!("{}", source.value);
        let extracts = application.edits.iter().map(|edit| {
            let target_field = quote::format_ident!("{}", edit.target.segments[0].value);
            let raw = quote::format_ident!("__lens_raw_{}", edit.value.value);
            match edit.kind {
                LensEditKind::Focus => quote! { let #raw = __owner.#target_field.take()?; },
                LensEditKind::Prepend => quote! {
                    if __owner.#target_field.is_empty() {
                        return None;
                    }
                    let #raw = __owner.#target_field.remove(0);
                },
                LensEditKind::Append => quote! { let #raw = __owner.#target_field.pop()?; },
            }
        });
        let decodes = application.edits.iter().map(|edit| {
            let value = quote::format_ident!("{}", edit.value.value);
            let raw = quote::format_ident!("__lens_raw_{}", edit.value.value);
            edit.adapter.as_ref().map_or_else(
                || quote! { let #value = #raw; },
                |adapter| {
                    let destructurer = parse_type(&adapter.destructurer.value);
                    quote! { let #value = #destructurer(&#source_ident, #raw)?; }
                },
            )
        });
        return quote! {
            let mut __owner = value.clone();
            #(#extracts)*
            let #source_ident = __owner;
            #(#decodes)*
            Some((#(#field_names),*))
        };
    }

    let default_checks = lens.fields.iter().filter_map(|field| {
        let claimed = application
            .edits
            .iter()
            .any(|edit| edit.target.segments[0].value == field.name.value);
        if claimed {
            return None;
        }
        let field_name = quote::format_ident!("{}", field.name.value);
        match field.kind {
            LensFieldKind::Optional { .. } => {
                Some(quote! { if value.#field_name.is_some() { return None; } })
            }
            LensFieldKind::Vector { .. } => {
                Some(quote! { if !value.#field_name.is_empty() { return None; } })
            }
            LensFieldKind::Value { .. } => {
                unreachable!("validated: EC033 rejects unclaimed required owner fields")
            }
        }
    });
    let extracts = application.edits.iter().map(|edit| {
        let target_field = quote::format_ident!("{}", edit.target.segments[0].value);
        let value = quote::format_ident!("{}", edit.value.value);
        let owner_field = lens
            .fields
            .iter()
            .find(|field| field.name.value == edit.target.segments[0].value)
            .expect("validated lens target");
        match owner_field.kind {
            LensFieldKind::Value { .. } => {
                quote! { let #value = value.#target_field.clone(); }
            }
            LensFieldKind::Optional { .. } => {
                quote! { let #value = value.#target_field.clone()?; }
            }
            LensFieldKind::Vector { .. } => {
                unreachable!("validated: no-source vector edits are not focusable")
            }
        }
    });
    quote! {
        #(#default_checks)*
        #(#extracts)*
        Some((#(#field_names),*))
    }
}

fn lens_bind_construction(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
    path: &crate::model::Spanned<String>,
    fields: &[FieldBinding],
    application: &LensApplication,
) -> TokenStream {
    let lens = group
        .lenses
        .iter()
        .find(|lens| lens.name.value == application.owner.value)
        .expect("validated: EC016 rejects an undeclared lens owner");
    let target = parse_type(&path.value);
    let id = construction.id.value.as_str();
    let build_fn = quote::format_ident!("build_{}", construction.id.value);
    let parts_fn = quote::format_ident!("parts_{}", construction.id.value);
    let assert_fn = quote::format_ident!("__assert_lens_owner_{}", construction.id.value);
    let params: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! { #field: #field_ty }
        })
        .collect();
    let arity_allowance = builder_arity_allowance(params.len());
    let field_names: Vec<proc_macro2::Ident> = fields
        .iter()
        .map(|binding| quote::format_ident!("{}", binding.field.value))
        .collect();
    let field_types: Vec<TokenStream> = fields
        .iter()
        .map(|binding| field_type(group, &binding.kind))
        .collect();
    let checks: Vec<TokenStream> = construction
        .constraints
        .iter()
        .filter_map(|constraint| match constraint {
            Constraint::Require(predicate) => {
                Some(require_check(id, group, fields, &predicate.value))
            }
            Constraint::Recognize(_) | Constraint::DeriveFeature { .. } => None,
        })
        .collect();

    let owner_fields: Vec<proc_macro2::Ident> = lens
        .fields
        .iter()
        .map(|field| quote::format_ident!("{}", field.name.value))
        .collect();
    let owner_types: Vec<TokenStream> = lens
        .fields
        .iter()
        .map(|field| lens_field_type(&field.kind))
        .collect();
    let adapter_assert = lens.adapter.is_some();
    let assert_types = owner_fields.iter().zip(&owner_types).map(|(field, ty)| {
        if adapter_assert {
            quote! { let _: &#ty = &#field; }
        } else {
            quote! { let _: &#ty = #field; }
        }
    });

    let assert_destructure = lens.adapter.as_ref().map_or_else(
        || quote! { let #target { #(#owner_fields),* } = value; },
        |adapter| {
            let destructurer = parse_type(&adapter.destructurer.value);
            quote! { let (#(#owner_fields),*) = #destructurer(value.clone()); }
        },
    );

    let construct = checked_inverse_construct(
        construction,
        &target,
        lens_construct(lens, application, &target, id),
    );
    let destructure = lens_destructure(lens, application, &field_names);
    let parts_return = quote! { Option<(#(#field_types),*)> };

    quote! {
        #[allow(dead_code, reason = "the exhaustive record pattern and typed borrows are the lens shape check")]
        fn #assert_fn(value: &#target) {
            #assert_destructure
            #(#assert_types)*
        }

        #arity_allowance
        pub fn #build_fn(#(#params),*) -> Result<#target, ::deckmaste_construction_compiler::runtime::DeclarationViolation> {
            #(#checks)*
            #construct
        }

        pub fn #parts_fn(value: &#target) -> #parts_return {
            #destructure
        }
    }
}

/// Bind mode's checked door onto an unmigrated target type (CF-7): a
/// `require`-enforcing builder plus a full-pattern destructurer, both
/// re-exported. No struct, no `try_new`, no serde — EC005 already bars
/// bind+deserialize at validation time, so there is no door to route through
/// a `Deserialize` impl here.
fn bind_construction(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> Option<TokenStream> {
    let AstShape::Bind { path, fields } = &construction.ast else {
        return None; // own mode: handled separately by `own_construction`
    };
    if let Some(application) = &construction.lens {
        return Some(lens_bind_construction(
            group,
            construction,
            path,
            fields,
            application,
        ));
    }
    let target = parse_type(&path.value);
    let id = construction.id.value.as_str();
    let build_fn = quote::format_ident!("build_{}", construction.id.value);
    let parts_fn = quote::format_ident!("parts_{}", construction.id.value);
    let params: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! { #field: #field_ty }
        })
        .collect();
    let arity_allowance = builder_arity_allowance(params.len());
    let checks: Vec<TokenStream> = construction
        .constraints
        .iter()
        .filter_map(|constraint| match constraint {
            Constraint::Require(predicate) => {
                Some(require_check(id, group, fields, &predicate.value))
            }
            Constraint::Recognize(_) | Constraint::DeriveFeature { .. } => None,
        })
        .collect();
    let field_names: Vec<proc_macro2::Ident> = fields
        .iter()
        .map(|b| quote::format_ident!("{}", b.field.value))
        .collect();
    let field_types: Vec<TokenStream> = fields.iter().map(|b| field_type(group, &b.kind)).collect();
    let construct = construction.bind_adapter.as_ref().map_or_else(
        || quote! { Ok(#target { #(#field_names),* }) },
        |adapter| {
            let constructor = parse_type(&adapter.constructor.value);
            quote! { #constructor(#(#field_names),*) }
        },
    );
    let construct = checked_inverse_construct(construction, &target, construct);
    let destructure = construction.bind_adapter.as_ref().map_or_else(
        || {
            quote! {
                let #target { #(#field_names),* } = value;
                (#(#field_names),*)
            }
        },
        |adapter| {
            let destructurer = parse_type(&adapter.destructurer.value);
            quote! { #destructurer(value) }
        },
    );
    let parts_return = if construction.bind_adapter.is_some() {
        quote! { (#(#field_types),*) }
    } else {
        quote! { (#(&#field_types),*) }
    };
    Some(quote! {
        // Bind mode: the target type stays public and unmigrated; these are
        // the checked door and the drift gate. The struct literal and the
        // full (no `..`) pattern each name every declared field, so a
        // declaration/type mismatch in either direction is a compile error.
        #arity_allowance
        pub fn #build_fn(#(#params),*) -> Result<#target, ::deckmaste_construction_compiler::runtime::DeclarationViolation> {
            #(#checks)*
            #construct
        }
        pub fn #parts_fn(value: &#target) -> #parts_return {
            #destructure
        }
    })
}

fn builder_arity_allowance(param_count: usize) -> Option<TokenStream> {
    (param_count > 7).then(|| {
        quote! {
            #[allow(
                clippy::too_many_arguments,
                reason = "the generated builder preserves the declaration's typed field arity"
            )]
        }
    })
}

/// A whole-value inverse recognizer is part of a checked construction's
/// ingress contract, not merely a renderer-side dispatch hint. When every
/// canonical form names one, reject a built value outside their union before
/// it can escape through the generated door.
fn checked_inverse_construct(
    construction: &ConstructionDeclaration,
    target: &TokenStream,
    construct: TokenStream,
) -> TokenStream {
    let guards = construction
        .forms
        .iter()
        .map(|form| {
            (!form.fallback)
                .then_some(form.inverse_guard.as_ref().or(form.value_guard.as_ref()))
                .flatten()
        })
        .collect::<Option<Vec<_>>>();
    let Some(guards) = guards.filter(|guards| !guards.is_empty()) else {
        return construct;
    };
    let condition = guards
        .into_iter()
        .fold(quote! { false }, |condition, guard| {
            let predicate = parse_type(&guard.value);
            quote! { #condition || #predicate(&value) }
        });
    let id = construction.id.value.as_str();
    quote! {
        let value: #target = { #construct }?;
        if !(#condition) {
            return Err(::deckmaste_construction_compiler::runtime::DeclarationViolation {
                construction: #id,
                requirement: "constructed value matches a declared inverse form",
            });
        }
        Ok(value)
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "one declaration walk emits canonical selection, explicit-form replay, \
              and their shared visitor arms"
)]
fn linearizer(group: &GroupDeclaration, construction: &ConstructionDeclaration) -> TokenStream {
    let id = construction.id.value.as_str();
    let function = quote::format_ident!("linearize_{}_with", construction.id.value);
    let form_function = quote::format_ident!("linearize_{}_form_with", construction.id.value);
    let selected_form_function = quote::format_ident!("selected_{}_form", construction.id.value);
    let declaration = declaration_ident(group);
    let construction_index = group
        .constructions
        .iter()
        .position(|candidate| std::ptr::eq(candidate, construction))
        .expect("the emitted construction belongs to its group");
    let (target, fields) = match &construction.ast {
        AstShape::Own { name, fields } => {
            let target = quote::format_ident!("{}", name.value);
            (quote! { #target }, fields.as_slice())
        }
        AstShape::Bind { path, fields } => (parse_type(&path.value), fields.as_slice()),
    };
    let field_names: Vec<proc_macro2::Ident> = fields
        .iter()
        .map(|binding| quote::format_ident!("{}", binding.field.value))
        .collect();
    let selections: Vec<TokenStream> = construction
        .forms
        .iter()
        .enumerate()
        .filter(|(_, form)| !form.fallback)
        .map(|(form_index, form)| {
            let field_guard = form
                .guard
                .as_ref()
                .map(|guard| predicate_tokens(group, fields, &guard.value));
            let value_guard = form.value_guard.as_ref().map(|guard| {
                let predicate = parse_type(&guard.value);
                quote! { #predicate(value) }
            });
            let condition = match (field_guard, value_guard) {
                (Some(field_guard), Some(value_guard)) => {
                    quote! { (#field_guard) && (#value_guard) }
                }
                (Some(condition), None) | (None, Some(condition)) => condition,
                (None, None) => quote! { true },
            };
            let ordinal = proc_macro2::Literal::u16_unsuffixed(form.ordinal.value);
            quote! {
                if #condition {
                    if let Some((first, _)) = selected_form {
                        return Err(
                            ::deckmaste_construction_compiler::runtime::FormSelectionError::MultipleMatchingForms {
                                construction: #id,
                                first,
                                second: #ordinal,
                            },
                        );
                    }
                    selected_form = Some((#ordinal, #form_index));
                }
            }
        })
        .collect();
    let fallback_selection = construction
        .forms
        .iter()
        .enumerate()
        .find(|(_, form)| form.fallback)
        .map(|(form_index, form)| {
            let ordinal = proc_macro2::Literal::u16_unsuffixed(form.ordinal.value);
            quote! {
                if selected_form.is_none() {
                    selected_form = Some((#ordinal, #form_index));
                }
            }
        });
    let stored_witnesses: Vec<TokenStream> = construction
        .witnesses
        .iter()
        .filter_map(|witness| {
            let crate::model::WitnessClass::Stored { path } = &witness.class else {
                return None;
            };
            Some(linearize_stored_witness(
                group,
                fields,
                witness.name.value.as_str(),
                path,
            ))
        })
        .collect();
    let form_arms: Vec<TokenStream> = construction
        .forms
        .iter()
        .map(|form| {
            let name = form.name.value.as_str();
            let ordinal = proc_macro2::Literal::u16_unsuffixed(form.ordinal.value);
            let atoms: Vec<TokenStream> = form
                .surface
                .iter()
                .map(|atom| linearize_atom(group, fields, atom))
                .collect();
            quote! {
                #ordinal => {
                    visitor
                        .begin_form(#id, #name, #ordinal)
                        .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                    #(#atoms)*
                    #(#stored_witnesses)*
                    visitor
                        .end_form(#id)
                        .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                    Ok(())
                }
            }
        })
        .collect();
    let form_checks: Vec<TokenStream> = construction
        .forms
        .iter()
        .filter_map(|form| {
            let ordinal = proc_macro2::Literal::u16_unsuffixed(form.ordinal.value);
            let field_guard = form
                .guard
                .as_ref()
                .map(|guard| predicate_tokens(group, fields, &guard.value));
            let value_guard = form.value_guard.as_ref().map(|guard| {
                let predicate = parse_type(&guard.value);
                quote! { #predicate(value) }
            });
            let condition = match (field_guard, value_guard) {
                (Some(field_guard), Some(value_guard)) => {
                    quote! { (#field_guard) && (#value_guard) }
                }
                (Some(condition), None) | (None, Some(condition)) => condition,
                (None, None) => return None,
            };
            Some(quote! {
                #ordinal if !(#condition) => {
                    Err(::deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingForm {
                        construction: #id,
                    })
                }
            })
        })
        .collect();
    let prepare_fields = match &construction.ast {
        AstShape::Own { name, .. } => {
            let target = quote::format_ident!("{}", name.value);
            quote! { let #target { #(#field_names),* } = value; }
        }
        AstShape::Bind { .. } if construction.lens.is_some() => {
            let parts = quote::format_ident!("parts_{}", construction.id.value);
            quote! {
                let Some((#(#field_names),*)) = #parts(value) else {
                    return Err(::deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingForm {
                        construction: #id,
                    });
                };
                #(let #field_names = &#field_names;)*
            }
        }
        AstShape::Bind { .. } if construction.bind_adapter.is_some() => {
            let parts = quote::format_ident!("parts_{}", construction.id.value);
            quote! {
                let (#(#field_names),*) = #parts(value);
                #(let #field_names = &#field_names;)*
            }
        }
        AstShape::Bind { path, .. } => {
            let target = parse_type(&path.value);
            quote! { let #target { #(#field_names),* } = value; }
        }
    };
    let selection_bindings: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            if construction.forms.iter().any(|form| {
                form.guard.as_ref().is_some_and(|guard| {
                    predicate_mentions_field(&guard.value, &binding.field.value)
                })
            }) {
                quote! { #field }
            } else {
                quote! { #field: _ }
            }
        })
        .collect();
    let selection_prepare_fields = match &construction.ast {
        AstShape::Own { name, .. } => {
            let target = quote::format_ident!("{}", name.value);
            quote! { let #target { #(#selection_bindings),* } = value; }
        }
        AstShape::Bind { .. } if construction.lens.is_some() => {
            let parts = quote::format_ident!("parts_{}", construction.id.value);
            quote! {
                let Some((#(#field_names),*)) = #parts(value) else {
                    return Err(::deckmaste_construction_compiler::runtime::FormSelectionError::NoMatchingForm {
                        construction: #id,
                    });
                };
                #(let #field_names = &#field_names;)*
            }
        }
        AstShape::Bind { .. } if construction.bind_adapter.is_some() => {
            let any_field_guard = construction.forms.iter().any(|form| form.guard.is_some());
            if any_field_guard {
                let parts = quote::format_ident!("parts_{}", construction.id.value);
                quote! {
                    let (#(#field_names),*) = #parts(value);
                    #(let #field_names = &#field_names;)*
                }
            } else {
                quote! {}
            }
        }
        AstShape::Bind { path, .. } => {
            let target = parse_type(&path.value);
            quote! { let #target { #(#selection_bindings),* } = value; }
        }
    };
    quote! {
        pub fn #selected_form_function(
            value: &#target,
        ) -> Result<
            &'static ::deckmaste_construction_compiler::runtime::FormData,
            ::deckmaste_construction_compiler::runtime::FormSelectionError,
        > {
            #selection_prepare_fields
            let mut selected_form: Option<(u16, usize)> = None;
            #(#selections)*
            #fallback_selection
            let Some((_, selected_form)) = selected_form else {
                return Err(
                    ::deckmaste_construction_compiler::runtime::FormSelectionError::NoMatchingForm {
                        construction: #id,
                    },
                );
            };
            Ok(&#declaration.constructions[#construction_index].forms[selected_form])
        }

        pub fn #function<V>(
            value: &#target,
            visitor: &mut V,
        ) -> Result<(), ::deckmaste_construction_compiler::runtime::LinearizationError<V::Error>>
        where
            V: ::deckmaste_construction_compiler::runtime::LinearizationVisitor,
        {
            let selected_form = #selected_form_function(value).map_err(|error| match error {
                ::deckmaste_construction_compiler::runtime::FormSelectionError::NoMatchingForm {
                    construction,
                } => ::deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingForm {
                    construction,
                },
                ::deckmaste_construction_compiler::runtime::FormSelectionError::MultipleMatchingForms {
                    construction,
                    first,
                    second,
                } => ::deckmaste_construction_compiler::runtime::LinearizationError::MultipleMatchingForms {
                    construction,
                    first,
                    second,
                },
            })?;
            #form_function(value, selected_form.ordinal, visitor)
        }

        pub fn #form_function<V>(
            value: &#target,
            selected_form: u16,
            visitor: &mut V,
        ) -> Result<(), ::deckmaste_construction_compiler::runtime::LinearizationError<V::Error>>
        where
            V: ::deckmaste_construction_compiler::runtime::LinearizationVisitor,
        {
            #prepare_fields
            match selected_form {
                #(#form_checks)*
                #(#form_arms)*
                _ => Err(::deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingForm {
                    construction: #id,
                }),
            }
        }
    }
}

fn predicate_mentions_field(predicate: &Predicate, field: &str) -> bool {
    match predicate {
        Predicate::LenAtLeast { path, .. }
        | Predicate::LenIs { path, .. }
        | Predicate::In { path, .. }
        | Predicate::IsSome { path }
        | Predicate::IsNone { path } => path
            .segments
            .first()
            .is_some_and(|segment| segment.value == field),
        Predicate::All(predicates) | Predicate::Any(predicates) => predicates
            .iter()
            .any(|predicate| predicate_mentions_field(predicate, field)),
    }
}

fn linearize_atom(
    group: &GroupDeclaration,
    fields: &[FieldBinding],
    atom: &crate::model::SurfaceAtom,
) -> TokenStream {
    match atom {
        crate::model::SurfaceAtom::Literal(literal) => {
            let literal = literal.value.as_str();
            quote! {
                visitor
                    .literal(#literal)
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            }
        }
        crate::model::SurfaceAtom::Hole(path)
        | crate::model::SurfaceAtom::Lexeme(path)
        | crate::model::SurfaceAtom::Identity(path) => linearize_path_value(group, fields, path),
    }
}

fn linearize_path_value(
    group: &GroupDeclaration,
    fields: &[FieldBinding],
    path: &crate::model::FieldPath,
) -> TokenStream {
    match path.segments.as_slice() {
        [field] => {
            let binding = fields
                .iter()
                .find(|binding| binding.field.value == field.value)
                .expect("validated: EC010 rejects a form path naming a nonexistent field");
            let accessor = quote::format_ident!("{}", field.value);
            visit_kind(group, &quote! { #accessor }, &field.value, &binding.kind)
        }
        [sequence, selector, element_field] => {
            debug_assert_eq!(selector.value, "last");
            let sequence_ident = quote::format_ident!("{}", sequence.value);
            let element = element_of_sequence_field(group, fields, &sequence.value);
            let binding = element
                .fields
                .iter()
                .find(|binding| binding.field.value == element_field.value)
                .expect("validated: EC010 rejects a form path naming a nonexistent element field");
            let field_ident = quote::format_ident!("{}", element_field.value);
            let label = path.dotted();
            let visit = visit_kind(
                group,
                &quote! { &member.#field_ident },
                &label,
                &binding.kind,
            );
            quote! {
                if let Some(member) = #sequence_ident.last() {
                    #visit
                }
            }
        }
        _ => unreachable!("validated surface paths are direct fields or sequence.last fields"),
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "one closed field-kind dispatch emits every structural visitor protocol"
)]
fn visit_kind(
    group: &GroupDeclaration,
    accessor: &TokenStream,
    label: &str,
    kind: &FieldKind,
) -> TokenStream {
    match kind {
        FieldKind::Unit | FieldKind::TupleProduct { .. } | FieldKind::StructProduct { .. } => {
            unreachable!("variant products are visited by their bound-enum owner")
        }
        FieldKind::Identity {
            value_type,
            provider,
        } => {
            let value_type = value_type.value.as_str();
            let provider = provider.value.as_str();
            quote! {
                visitor
                    .identity(#provider, #value_type, #accessor)
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                visitor
                    .projected_identity(
                        #label,
                        #provider,
                        #value_type,
                        ::deckmaste_construction_compiler::runtime::projection_value(#accessor),
                    )
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            }
        }
        FieldKind::Subtree { category, boxed }
        | FieldKind::TypedSubtree {
            category, boxed, ..
        } => {
            let category = category.value.as_str();
            let value = if *boxed {
                quote! { (#accessor).as_ref() }
            } else {
                quote! { #accessor }
            };
            quote! {
                visitor
                    .subtree(#category, #value)
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                visitor
                    .projected_subtree(
                        #label,
                        #category,
                        ::deckmaste_construction_compiler::runtime::projection_value(#value),
                    )
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            }
        }
        FieldKind::Scalar { codec } | FieldKind::TypedScalar { codec, .. } => {
            let codec = codec.value.as_str();
            quote! {
                visitor
                    .scalar(#codec, #accessor)
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                visitor
                    .projected_scalar(
                        #label,
                        #codec,
                        ::deckmaste_construction_compiler::runtime::projection_value(#accessor),
                    )
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            }
        }
        FieldKind::SurfaceScalar { .. } => {
            unreachable!("surface-only scalars are emitted with sequence position context")
        }
        FieldKind::Sequence { element } | FieldKind::NonEmptySequence { element } => {
            let element_declaration = group
                .elements
                .iter()
                .find(|candidate| candidate.name.value == element.value)
                .expect("validated: EC003 rejects a sequence naming an undeclared element");
            let member = visit_element(
                group,
                element_declaration,
                &quote! { member },
                &quote! { index },
                &quote! { (#accessor).len() },
            );
            quote! {
                visitor
                    .begin_sequence(#label, (#accessor).len())
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                for (index, member) in (#accessor).iter().enumerate() {
                    visitor
                        .sequence_member(#label, index)
                        .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                    #member
                }
                visitor
                    .end_sequence(#label)
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            }
        }
        FieldKind::SeparatedNonEmptySequence { element, separator } => {
            let element_declaration = group
                .elements
                .iter()
                .find(|candidate| candidate.name.value == element.value)
                .expect("validated: EC003 rejects a sequence naming an undeclared element");
            let first = visit_element(
                group,
                element_declaration,
                &quote! { (#accessor).first() },
                &quote! { 0usize },
                &quote! { (#accessor).len() },
            );
            let continuation = visit_element(
                group,
                element_declaration,
                &quote! { continuation.value() },
                &quote! { index },
                &quote! { (#accessor).len() },
            );
            let separator_codec = separator.value.as_str();
            let separator_role = format!("{label}.separator");
            quote! {
                visitor
                    .begin_sequence(#label, (#accessor).len())
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                visitor
                    .sequence_member(#label, 0usize)
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                #first
                for (offset, continuation) in (#accessor).rest().iter().enumerate() {
                    let index = offset + 1;
                    visitor
                        .sequence_member(#label, index)
                        .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                    visitor
                        .scalar(#separator_codec, continuation.separator())
                        .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                    visitor
                        .projected_scalar(
                            #separator_role,
                            #separator_codec,
                            ::deckmaste_construction_compiler::runtime::projection_value(
                                continuation.separator(),
                            ),
                        )
                        .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                    #continuation
                }
                visitor
                    .end_sequence(#label)
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            }
        }
        FieldKind::Sum { element, boxed } => visit_sum(group, accessor, label, element, *boxed),
        FieldKind::Product { element, boxed } => visit_product(accessor, label, element, *boxed),
        FieldKind::Optional { inner } => {
            let inner = visit_kind(group, &quote! { present }, label, inner);
            quote! {
                visitor
                    .optional(#label, (#accessor).is_some())
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                if let Some(present) = (#accessor).as_ref() {
                    #inner
                }
            }
        }
    }
}

fn visit_product(
    accessor: &TokenStream,
    label: &str,
    element_name: &crate::model::Spanned<String>,
    boxed: bool,
) -> TokenStream {
    let function = quote::format_ident!("__linearize_product_{}_with", element_name.value);
    let value = if boxed {
        quote! { (#accessor).as_ref() }
    } else {
        quote! { #accessor }
    };
    quote! {
        #function(#value, #label, visitor)?;
    }
}

fn visit_sum(
    _group: &GroupDeclaration,
    accessor: &TokenStream,
    label: &str,
    element_name: &crate::model::Spanned<String>,
    boxed: bool,
) -> TokenStream {
    let function = quote::format_ident!("__linearize_sum_{}_with", element_name.value);
    let value = if boxed {
        quote! { (#accessor).as_ref() }
    } else {
        quote! { #accessor }
    };
    quote! {
        #function(#value, #label, visitor)?;
    }
}

fn sum_element_linearizer(group: &GroupDeclaration, element: &ElementDeclaration) -> TokenStream {
    let function = quote::format_ident!("__linearize_sum_{}_with", element.name.value);
    let target = parse_type(
        &element
            .bind_path
            .as_ref()
            .expect("validated: sums require a bound enum element")
            .value,
    );
    let element_label = element.name.value.as_str();
    let arms = element.variants.iter().map(|variant| {
        let variant_ident = quote::format_ident!("{}", variant.name.value);
        let variant_label = variant.name.value.as_str();
        let (pattern, payload) = match &variant.payload {
            FieldKind::Unit => (quote! { #target::#variant_ident }, quote! {}),
            FieldKind::TupleProduct { fields } => {
                let names = fields
                    .iter()
                    .map(|field| quote::format_ident!("{}", field.field.value))
                    .collect::<Vec<_>>();
                let visits = fields.iter().zip(&names).map(|(field, name)| {
                    let role = field.field.value.as_str();
                    visit_kind(group, &quote! { #name }, role, &field.kind)
                });
                (
                    quote! { #target::#variant_ident(#(#names),*) },
                    quote! { #(#visits)* },
                )
            }
            FieldKind::StructProduct { fields } => {
                let names = fields
                    .iter()
                    .map(|field| quote::format_ident!("{}", field.field.value))
                    .collect::<Vec<_>>();
                let visits = fields.iter().zip(&names).map(|(field, name)| {
                    let role = field.field.value.as_str();
                    visit_kind(group, &quote! { #name }, role, &field.kind)
                });
                (
                    quote! { #target::#variant_ident { #(#names),* } },
                    quote! { #(#visits)* },
                )
            }
            payload => {
                let visit = visit_kind(group, &quote! { payload }, "payload", payload);
                (
                    quote! { #target::#variant_ident(payload) },
                    quote! { #visit },
                )
            }
        };
        quote! {
            #pattern => {
                visitor
                    .begin_sum_variant(role, #element_label, #variant_label)
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                #payload
                visitor
                    .end_sum_variant(role)
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            }
        }
    });
    quote! {
        fn #function<V>(
            value: &#target,
            role: &'static str,
            visitor: &mut V,
        ) -> Result<(), ::deckmaste_construction_compiler::runtime::LinearizationError<V::Error>>
        where
            V: ::deckmaste_construction_compiler::runtime::LinearizationVisitor,
        {
            visitor
                .bound_value(#element_label, value)
                .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            match value {
                #(#arms),*
            }
            Ok(())
        }
    }
}

fn product_element_linearizer(
    group: &GroupDeclaration,
    element: &ElementDeclaration,
) -> TokenStream {
    let function = quote::format_ident!("__linearize_product_{}_with", element.name.value);
    let target = element.bind_path.as_ref().map_or_else(
        || {
            let owned = pascal_ident(&element.name.value);
            quote! { #owned }
        },
        |path| parse_type(&path.value),
    );
    let element_label = element.name.value.as_str();
    let fields = element
        .fields
        .iter()
        .filter(|field| !matches!(field.kind, FieldKind::SurfaceScalar { .. }))
        .collect::<Vec<_>>();
    let field_names = fields
        .iter()
        .map(|field| quote::format_ident!("__product_{}", field.field.value))
        .collect::<Vec<_>>();
    let prepare = if element.bind_path.is_some() {
        let parts = quote::format_ident!("__parts_{}", element.name.value);
        if let [field_name] = field_names.as_slice() {
            quote! {
                let #field_name = #parts(value);
            }
        } else {
            quote! {
                let (#(#field_names),*) = #parts(value);
            }
        }
    } else {
        let reads = fields.iter().zip(&field_names).map(|(field, name)| {
            let member = quote::format_ident!("{}", field.field.value);
            quote! { let #name = &value.#member; }
        });
        quote! { #(#reads)* }
    };
    let visits = fields.iter().zip(&field_names).map(|(field, name)| {
        let role = field.field.value.as_str();
        visit_kind(group, &quote! { #name }, role, &field.kind)
    });
    quote! {
        fn #function<V>(
            value: &#target,
            role: &'static str,
            visitor: &mut V,
        ) -> Result<(), ::deckmaste_construction_compiler::runtime::LinearizationError<V::Error>>
        where
            V: ::deckmaste_construction_compiler::runtime::LinearizationVisitor,
        {
            visitor
                .bound_value(#element_label, value)
                .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            visitor
                .begin_product(role, #element_label)
                .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            #prepare
            #(#visits)*
            visitor
                .end_product(role)
                .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            Ok(())
        }
    }
}

fn visit_element(
    group: &GroupDeclaration,
    element: &ElementDeclaration,
    accessor: &TokenStream,
    sequence_index: &TokenStream,
    sequence_len: &TokenStream,
) -> TokenStream {
    let element_name = element.name.value.as_str();
    let structural_visit = if element.variants.is_empty() {
        let fields: Vec<TokenStream> = element
            .fields
            .iter()
            .map(|binding| {
                let label = format!("{}.{}", element.name.value, binding.field.value);
                if let FieldKind::SurfaceScalar { codec } = &binding.kind {
                    let codec = codec.value.as_str();
                    quote! {
                        visitor
                            .derived_sequence_scalar(
                                #label,
                                #codec,
                                #sequence_index,
                                #sequence_len,
                            )
                            .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                    }
                } else {
                    let field = quote::format_ident!("{}", binding.field.value);
                    visit_kind(group, &quote! { &#accessor.#field }, &label, &binding.kind)
                }
            })
            .collect();
        quote! { #(#fields)* }
    } else {
        let target = parse_type(
            &element
                .bind_path
                .as_ref()
                .expect("validated: enum variants require a bind target")
                .value,
        );
        let arms: Vec<TokenStream> = element
            .variants
            .iter()
            .map(|variant| {
                let name = quote::format_ident!("{}", variant.name.value);
                let variant_name = variant.name.value.as_str();
                let (pattern, payload) = match &variant.payload {
                    FieldKind::Unit => (quote! { #target::#name }, quote! {}),
                    FieldKind::TupleProduct { fields } => {
                        let names = fields
                            .iter()
                            .map(|field| quote::format_ident!("{}", field.field.value))
                            .collect::<Vec<_>>();
                        let visits = fields.iter().zip(&names).map(|(field, name)| {
                            let label = format!("{}.{}", element.name.value, field.field.value);
                            visit_kind(group, &quote! { #name }, &label, &field.kind)
                        });
                        (
                            quote! { #target::#name(#(#names),*) },
                            quote! { #(#visits)* },
                        )
                    }
                    FieldKind::StructProduct { fields } => {
                        let names = fields
                            .iter()
                            .map(|field| quote::format_ident!("{}", field.field.value))
                            .collect::<Vec<_>>();
                        let visits = fields.iter().zip(&names).map(|(field, name)| {
                            let label = format!("{}.{}", element.name.value, field.field.value);
                            visit_kind(group, &quote! { #name }, &label, &field.kind)
                        });
                        (
                            quote! { #target::#name { #(#names),* } },
                            quote! { #(#visits)* },
                        )
                    }
                    payload => {
                        let label = format!("{}.payload", element.name.value);
                        let visit = visit_kind(group, &quote! { payload }, &label, payload);
                        (
                            quote! { #target::#name(payload) },
                            quote! { #visit },
                        )
                    }
                };
                quote! {
                    #pattern => {
                        visitor
                            .element_variant(#element_name, #variant_name)
                            .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                        #payload
                    }
                }
            })
            .collect();
        quote! {
            match #accessor {
                #(#arms),*
            }
        }
    };
    quote! {
        visitor
            .bound_value(#element_name, #accessor)
            .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
        visitor
            .projected_bound_value(
                #element_name,
                ::deckmaste_construction_compiler::runtime::projection_input(#accessor),
            )
            .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
        #structural_visit
    }
}

fn linearize_stored_witness(
    group: &GroupDeclaration,
    fields: &[FieldBinding],
    name: &str,
    path: &crate::model::FieldPath,
) -> TokenStream {
    let dotted = path.dotted();
    match path.segments.as_slice() {
        [field] => {
            let field = quote::format_ident!("{}", field.value);
            quote! {
                visitor
                    .stored_witness(#name, #dotted, #field)
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                visitor
                    .projected_stored_witness(
                        #name,
                        #dotted,
                        ::deckmaste_construction_compiler::runtime::projection_value(#field),
                    )
                    .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
            }
        }
        [sequence, selector, element_field] => {
            debug_assert_eq!(selector.value, "last");
            let sequence = quote::format_ident!("{}", sequence.value);
            let element = element_of_sequence_field(group, fields, &path.segments[0].value);
            debug_assert!(
                element
                    .fields
                    .iter()
                    .any(|binding| binding.field.value == element_field.value)
            );
            let element_field = quote::format_ident!("{}", element_field.value);
            quote! {
                if let Some(member) = #sequence.last() {
                    visitor
                        .stored_witness(#name, #dotted, &member.#element_field)
                        .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                    visitor
                        .projected_stored_witness(
                            #name,
                            #dotted,
                            ::deckmaste_construction_compiler::runtime::projection_value(
                                &member.#element_field,
                            ),
                        )
                        .map_err(::deckmaste_construction_compiler::runtime::LinearizationError::Visitor)?;
                }
            }
        }
        _ => unreachable!("validated stored paths are direct fields or sequence.last fields"),
    }
}

/// Serde-opt-in own constructions only; `None` for opt-out. The `AstShape::Own`
/// match below also returns `None` for a `Bind`-mode construction, but
/// EC005 (`SerdeRequiresOwn`) already rejects `deserialize: true` on
/// bind mode at validation time — for a `ValidatedGroup`, `construction.ast`
/// is never `Bind` here once `construction.deserialize` is true. That arm is
/// defensive dead code, not a live deferral to a later milestone.
fn deserialize_impl(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> Option<TokenStream> {
    if !construction.deserialize {
        return None;
    }
    let AstShape::Own { name, fields } = &construction.ast else { return None };
    let ty = quote::format_ident!("{}", name.value);
    let raw_fields: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! { #field: #field_ty, }
        })
        .collect();
    let args: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            quote! { raw.#field }
        })
        .collect();
    Some(quote! {
        // Opt-in validating deserialization: a private raw mirror is
        // deserialized structurally, then handed to the same validator as
        // public construction. There is no other deserialize path.
        impl<'de> serde::Deserialize<'de> for #ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                struct Raw {
                    #(#raw_fields)*
                }
                let raw = Raw::deserialize(deserializer)?;
                #ty::try_new(#(#args),*).map_err(|violation| {
                    serde::de::Error::custom(format!(
                        "{}: {}",
                        violation.construction, violation.requirement
                    ))
                })
            }
        }
    })
}

fn field_type(group: &GroupDeclaration, kind: &FieldKind) -> TokenStream {
    match kind {
        FieldKind::Unit => quote! { () },
        FieldKind::TupleProduct { fields } => {
            let fields = fields.iter().map(|field| field_type(group, &field.kind));
            quote! { (#(#fields),*) }
        }
        FieldKind::StructProduct { .. } => {
            unreachable!("a struct-variant product has no standalone Rust type")
        }
        FieldKind::Identity { value_type, .. } => parse_type(&value_type.value),
        FieldKind::Subtree { category, boxed } => {
            let ty = parse_type(&category.value);
            if *boxed {
                quote! { Box<#ty> }
            } else {
                quote! { #ty }
            }
        }
        FieldKind::TypedSubtree {
            value_type, boxed, ..
        } => {
            let ty = parse_type(&value_type.value);
            if *boxed {
                quote! { Box<#ty> }
            } else {
                quote! { #ty }
            }
        }
        FieldKind::Scalar { codec: value_type } | FieldKind::TypedScalar { value_type, .. } => {
            let ty = parse_type(&value_type.value);
            quote! { #ty }
        }
        FieldKind::SurfaceScalar { codec } => {
            let ty = parse_type(&codec.value);
            quote! { #ty }
        }
        FieldKind::Sequence { element } => {
            let declaration = group
                .elements
                .iter()
                .find(|declaration| declaration.name.value == element.value);
            let ty = declaration
                .and_then(|declaration| declaration.bind_path.as_ref())
                .map_or_else(
                    || {
                        // EC003 covers top-level construction fields. Preserve
                        // the prior PascalCase fallback for an undeclared nested
                        // element sequence rather than changing that diagnostic
                        // boundary as a side effect of bound-element resolution.
                        let owned = pascal_ident(&element.value);
                        quote! { #owned }
                    },
                    |path| parse_type(&path.value),
                );
            quote! { Vec<#ty> }
        }
        FieldKind::NonEmptySequence { element } => {
            let ty = element_type(group, element);
            quote! { ::deckmaste_construction_compiler::runtime::NonEmpty<#ty> }
        }
        FieldKind::SeparatedNonEmptySequence { element, separator } => {
            let ty = element_type(group, element);
            let separator = parse_type(&separator.value);
            quote! {
                ::deckmaste_construction_compiler::runtime::SeparatedNonEmpty<#ty, #separator>
            }
        }
        FieldKind::Sum { element, boxed } | FieldKind::Product { element, boxed } => {
            let ty = element_type(group, element);
            if *boxed {
                quote! { Box<#ty> }
            } else {
                ty
            }
        }
        FieldKind::Optional { inner } => {
            let ty = field_type(group, inner);
            quote! { Option<#ty> }
        }
    }
}

fn element_type(group: &GroupDeclaration, element: &crate::model::Spanned<String>) -> TokenStream {
    group
        .elements
        .iter()
        .find(|declaration| declaration.name.value == element.value)
        .and_then(|declaration| declaration.bind_path.as_ref())
        .map_or_else(
            || {
                let owned = pascal_ident(&element.value);
                quote! { #owned }
            },
            |path| parse_type(&path.value),
        )
}

fn require_check(
    id: &str,
    group: &GroupDeclaration,
    fields: &[FieldBinding],
    predicate: &Predicate,
) -> TokenStream {
    let condition = predicate_tokens(group, fields, predicate);
    let requirement = render_predicate(predicate);
    quote! {
        if !(#condition) {
            return Err(::deckmaste_construction_compiler::runtime::DeclarationViolation {
                construction: #id,
                requirement: #requirement,
            });
        }
    }
}

/// EC032 guarantees every path here is either a single segment naming a
/// direct field, or exactly `<sequence>.(last|nonfinal).<element field>` (and
/// only onto a scalar/optional-scalar element field), and EC015 guarantees kind
/// agreement (`In` targets a scalar, `len()` targets a sequence) — so this
/// mapping is total for validated input.
fn predicate_tokens(
    group: &GroupDeclaration,
    fields: &[FieldBinding],
    predicate: &Predicate,
) -> TokenStream {
    match predicate {
        Predicate::LenAtLeast { path, min } => {
            // EC032: `len()` paths stay single-segment.
            let field = path_ident(path);
            // Unsuffixed: the generated comparison is against usize (.len()),
            // and an unsuffixed literal adopts that type with no cast.
            let min = proc_macro2::Literal::u32_unsuffixed(*min);
            quote! { #field.len() >= #min }
        }
        Predicate::LenIs { path, len } => {
            let field = path_ident(path);
            let len = proc_macro2::Literal::u32_unsuffixed(*len);
            quote! { #field.len() == #len }
        }
        Predicate::In { path, .. } | Predicate::IsSome { path } | Predicate::IsNone { path } => {
            match path.segments.as_slice() {
                [only] => {
                    let accessor = quote::format_ident!("{}", only.value);
                    single_field_check(&quote! { #accessor }, fields, &only.value, predicate)
                }
                [seq, selector, elem_field] => {
                    let seq_field = quote::format_ident!("{}", seq.value);
                    let element = element_of_sequence_field(group, fields, &seq.value);
                    let inner = if elem_field.value == "variant" {
                        bound_variant_check(element, predicate, &quote! { member })
                    } else {
                        let field_ident = quote::format_ident!("{}", elem_field.value);
                        single_field_check(
                            &quote! { member.#field_ident },
                            &element.fields,
                            &elem_field.value,
                            predicate,
                        )
                    };
                    match selector.value.as_str() {
                        "first" => quote! { #seq_field.first().is_none_or(|member| #inner) },
                        // Vacuous on an empty sequence; when the element field
                        // itself is optional-scalar, `single_field_check` nests
                        // Task 1's `None | Some(...)` reading INSIDE this
                        // closure, so the two vacuous layers compose correctly.
                        "last" => quote! { #seq_field.last().is_none_or(|member| #inner) },
                        // `take(len - 1)` means empty and singleton sequences
                        // both have an empty quantified prefix, exactly the
                        // declarative all-nonfinal reading.
                        "nonfinal" => quote! {
                            #seq_field
                                .iter()
                                .take(#seq_field.len().saturating_sub(1))
                                .all(|member| #inner)
                        },
                        _ => unreachable!(
                            "validated: EC032 admits only first/last/nonfinal sequence selectors"
                        ),
                    }
                }
                [sum, variant] if variant.value == "variant" => {
                    let accessor = quote::format_ident!("{}", sum.value);
                    let binding = fields
                        .iter()
                        .find(|binding| binding.field.value == sum.value)
                        .expect("validated: EC010 rejects a sum path naming a nonexistent field");
                    let FieldKind::Sum { element, boxed } = &binding.kind else {
                        unreachable!("validated: `.variant` opens only a sum field")
                    };
                    let element = group
                        .elements
                        .iter()
                        .find(|candidate| candidate.name.value == element.value)
                        .expect("validated: EC003 rejects a sum naming an undeclared element");
                    let accessor = if *boxed {
                        quote! { (#accessor).as_ref() }
                    } else {
                        quote! { #accessor }
                    };
                    bound_variant_check(element, predicate, &accessor)
                }
                _ => unreachable!(
                    "validated: EC032 admits only a single-field path or exactly seq.(last|nonfinal).field"
                ),
            }
        }
        Predicate::All(children) => {
            let parts: Vec<TokenStream> = children
                .iter()
                .map(|c| predicate_tokens(group, fields, c))
                .collect();
            quote! { (#(#parts)&&*) }
        }
        Predicate::Any(children) => {
            let parts: Vec<TokenStream> = children
                .iter()
                .map(|c| predicate_tokens(group, fields, c))
                .collect();
            quote! { (#(#parts)||*) }
        }
    }
}

fn bound_variant_check(
    element: &ElementDeclaration,
    predicate: &Predicate,
    accessor: &TokenStream,
) -> TokenStream {
    let Predicate::In { allowed, .. } = predicate else {
        unreachable!("validated: a variant discriminant is constrained only by `in [...]`")
    };
    let target = parse_type(
        &element
            .bind_path
            .as_ref()
            .expect("validated: enum variants require a bind target")
            .value,
    );
    let variants = allowed.iter().map(|variant| {
        let variant_ident = quote::format_ident!("{}", variant);
        let declaration = element
            .variants
            .iter()
            .find(|candidate| candidate.name.value == *variant)
            .expect("the bound enum declaration maps every predicate variant");
        match declaration.payload {
            FieldKind::Unit => quote! { #target::#variant_ident },
            FieldKind::StructProduct { .. } => quote! { #target::#variant_ident { .. } },
            _ => quote! { #target::#variant_ident(..) },
        }
    });
    quote! { matches!(#accessor, #(#variants)|*) }
}

/// The single-field check body shared between a direct field access
/// (`accessor` = the field itself) and the `.last()` closure's per-element
/// access (`accessor` = `member.field`) — today's `In`/`IsSome`/`IsNone`
/// emission, reused verbatim so the `.last()` closure's vacuous-on-empty
/// layer and Task 1's vacuous-on-absent optional-`In` reading nest
/// correctly by construction rather than by a second hand-written copy.
/// `fields`/`field_name` locate the SAME field `accessor` reads: top-level
/// ast fields for a direct path, an element's fields for the `.last` case.
fn single_field_check(
    accessor: &TokenStream,
    fields: &[FieldBinding],
    field_name: &str,
    predicate: &Predicate,
) -> TokenStream {
    match predicate {
        Predicate::In { allowed, .. } => {
            let codec = codec_of(fields, field_name);
            let variants: Vec<TokenStream> = allowed
                .iter()
                .map(|variant| {
                    let v = quote::format_ident!("{}", variant);
                    quote! { #codec::#v }
                })
                .collect();
            if is_optional_scalar(fields, field_name) {
                // Reading B: absence is vacuously admitted — `f in [...]`
                // on an optional field does not by itself demand presence.
                quote! { matches!(#accessor, None | Some(#(#variants)|*)) }
            } else {
                quote! { matches!(#accessor, #(#variants)|*) }
            }
        }
        Predicate::IsSome { .. } => quote! { #accessor.is_some() },
        Predicate::IsNone { .. } => quote! { #accessor.is_none() },
        Predicate::LenAtLeast { .. }
        | Predicate::LenIs { .. }
        | Predicate::All(_)
        | Predicate::Any(_) => {
            unreachable!("single_field_check is only called for In/IsSome/IsNone predicates")
        }
    }
}

/// The element declaration a sequence field selector addresses. `fields`
/// is the field list `seq_field_name` is looked up in (the construction's
/// own ast fields — selectors only ever open off a top-level sequence).
fn element_of_sequence_field<'g>(
    group: &'g GroupDeclaration,
    fields: &[FieldBinding],
    seq_field_name: &str,
) -> &'g ElementDeclaration {
    let binding = fields
        .iter()
        .find(|b| b.field.value == seq_field_name)
        .expect("validated: EC032 admits a sequence selector only when its field resolves");
    let FieldKind::Sequence { element } = &binding.kind else {
        unreachable!("validated: resolve_path only opens sequence selectors on Sequence fields")
    };
    group
        .elements
        .iter()
        .find(|e| e.name.value == element.value)
        .expect("validated: EC003 rejects a sequence naming an undeclared element")
}

fn render_predicate(predicate: &Predicate) -> String {
    match predicate {
        Predicate::LenAtLeast { path, min } => format!("{}.len() >= {min}", path.dotted()),
        Predicate::LenIs { path, len } => format!("{}.len() == {len}", path.dotted()),
        Predicate::In { path, allowed } => format!("{} in [{}]", path.dotted(), allowed.join(", ")),
        Predicate::IsSome { path } => format!("{}.is_some()", path.dotted()),
        Predicate::IsNone { path } => format!("{}.is_none()", path.dotted()),
        Predicate::All(children) => {
            let parts: Vec<String> = children.iter().map(render_predicate).collect();
            format!("all({})", parts.join(", "))
        }
        Predicate::Any(children) => {
            let parts: Vec<String> = children.iter().map(render_predicate).collect();
            format!("any({})", parts.join(", "))
        }
    }
}

/// Only ever called on a path EC032 guarantees is a single segment
/// (`LenAtLeast`/`LenIs` — selector-deep paths are `In`/`IsSome`/`IsNone`
/// only, handled separately in `predicate_tokens`).
fn path_ident(path: &crate::model::FieldPath) -> proc_macro2::Ident {
    quote::format_ident!("{}", path.segments[0].value)
}

fn codec_of(fields: &[FieldBinding], field_name: &str) -> TokenStream {
    let binding = fields
        .iter()
        .find(|b| b.field.value == field_name)
        .expect("validated: EC010 rejects a require path naming a nonexistent field");
    let codec = match &binding.kind {
        FieldKind::Scalar { codec: value_type } | FieldKind::TypedScalar { value_type, .. } => {
            value_type
        }
        FieldKind::Optional { inner } => match &**inner {
            FieldKind::Scalar { codec: value_type } | FieldKind::TypedScalar { value_type, .. } => {
                value_type
            }
            _ => unreachable!(
                "validated: EC015 guarantees an In-predicate path resolves to a scalar kind"
            ),
        },
        FieldKind::Unit
        | FieldKind::TupleProduct { .. }
        | FieldKind::StructProduct { .. }
        | FieldKind::Subtree { .. }
        | FieldKind::TypedSubtree { .. }
        | FieldKind::Identity { .. }
        | FieldKind::Sequence { .. }
        | FieldKind::NonEmptySequence { .. }
        | FieldKind::SeparatedNonEmptySequence { .. }
        | FieldKind::Sum { .. }
        | FieldKind::Product { .. }
        | FieldKind::SurfaceScalar { .. } => {
            unreachable!(
                "validated: EC015 guarantees an In-predicate path resolves to a scalar kind"
            )
        }
    };
    parse_type(&codec.value)
}

/// Whether an `In` predicate's target field (as `codec_of` locates it) is
/// `Optional { Scalar | TypedScalar }` — the case whose `matches!` gains a
/// `None |` arm.
fn is_optional_scalar(fields: &[FieldBinding], field_name: &str) -> bool {
    let binding = fields
        .iter()
        .find(|b| b.field.value == field_name)
        .expect("validated: EC010 rejects a require path naming a nonexistent field");
    matches!(
        &binding.kind,
        FieldKind::Optional { inner }
            if matches!(**inner, FieldKind::Scalar { .. } | FieldKind::TypedScalar { .. })
    )
}

fn parse_type(name: &str) -> TokenStream {
    let ty: syn::Type = syn::parse_str(name)
        .unwrap_or_else(|_| panic!("declared type `{name}` is not a parseable Rust type"));
    quote! { #ty }
}

fn pascal_ident(snake: &str) -> proc_macro2::Ident {
    quote::format_ident!("{}", crate::model::pascal_case(snake))
}

fn declaration_ident(group: &GroupDeclaration) -> proc_macro2::Ident {
    quote::format_ident!("{}_DECLARATION", group.name.value.to_uppercase())
}

fn declaration_static(group: &GroupDeclaration) -> TokenStream {
    let upper = declaration_ident(group);
    let name = group.name.value.as_str();
    let backend = match group.backend {
        crate::model::ConstructionBackend::Chart => quote! {
            ::deckmaste_construction_compiler::runtime::ConstructionBackendData::Chart
        },
        crate::model::ConstructionBackend::Ability => quote! {
            ::deckmaste_construction_compiler::runtime::ConstructionBackendData::Ability
        },
    };
    let elements: Vec<TokenStream> = group
        .elements
        .iter()
        .map(|e| {
            let n = e.name.value.as_str();
            quote! { #n }
        })
        .collect();
    let element_data: Vec<TokenStream> = group.elements.iter().map(element_row).collect();
    let lenses: Vec<TokenStream> = group.lenses.iter().map(lens_row).collect();
    let constructions: Vec<TokenStream> =
        group.constructions.iter().map(construction_row).collect();
    quote! {
        pub static #upper: ::deckmaste_construction_compiler::runtime::GroupData =
            ::deckmaste_construction_compiler::runtime::GroupData {
                name: #name,
                backend: #backend,
                elements: &[#(#elements),*],
                element_data: &[#(#element_data),*],
                lenses: &[#(#lenses),*],
                constructions: &[#(#constructions),*],
            };
    }
}

fn lens_row(lens: &crate::model::LensDeclaration) -> TokenStream {
    let name = lens.name.value.as_str();
    let owner_type = lens.owner_type.value.as_str();
    let fields = lens.fields.iter().map(|field| {
        let name = field.name.value.as_str();
        let kind = match &field.kind {
            LensFieldKind::Value { value_type } => {
                let value_type = value_type.value.as_str();
                quote! { ::deckmaste_construction_compiler::runtime::LensFieldKindData::Value { value_type: #value_type } }
            }
            LensFieldKind::Optional { value_type } => {
                let value_type = value_type.value.as_str();
                quote! { ::deckmaste_construction_compiler::runtime::LensFieldKindData::Optional { value_type: #value_type } }
            }
            LensFieldKind::Vector { element_type } => {
                let element_type = element_type.value.as_str();
                quote! { ::deckmaste_construction_compiler::runtime::LensFieldKindData::Vector { element_type: #element_type } }
            }
        };
        quote! {
            ::deckmaste_construction_compiler::runtime::LensFieldData {
                name: #name,
                kind: #kind,
            }
        }
    });
    quote! {
        ::deckmaste_construction_compiler::runtime::LensData {
            name: #name,
            owner_type: #owner_type,
            fields: &[#(#fields),*],
        }
    }
}

fn element_row(element: &ElementDeclaration) -> TokenStream {
    let name = element.name.value.as_str();
    let bind_path = element.bind_path.as_ref().map_or_else(
        || quote! { None },
        |path| {
            let path = path.value.as_str();
            quote! { Some(#path) }
        },
    );
    let fields: Vec<TokenStream> = element
        .fields
        .iter()
        .map(|field| field_row(field, None))
        .collect();
    let variants: Vec<TokenStream> = element
        .variants
        .iter()
        .map(|variant| {
            let name = variant.name.value.as_str();
            let payload = field_kind_row(&variant.payload, None, "payload");
            quote! {
                ::deckmaste_construction_compiler::runtime::ElementVariantData {
                    name: #name,
                    payload: #payload,
                }
            }
        })
        .collect();
    let erased_builders: Vec<TokenStream> = if element.bind_path.is_some()
        && element.fields.is_empty()
        && element.variants.is_empty()
    {
        Vec::new()
    } else if element.variants.is_empty() {
        let function = quote::format_ident!("__erased_build_{}", element.name.value);
        vec![quote! { #function }]
    } else {
        element
            .variants
            .iter()
            .map(|variant| {
                let suffix = crate::model::snake_case(&variant.name.value);
                let function =
                    quote::format_ident!("__erased_build_{}_{}", element.name.value, suffix);
                quote! { #function }
            })
            .collect()
    };
    let sequence_builder = quote::format_ident!("__erased_sequence_{}", element.name.value);
    quote! {
        ::deckmaste_construction_compiler::runtime::ElementData {
            name: #name,
            bind_path: #bind_path,
            fields: &[#(#fields),*],
            variants: &[#(#variants),*],
            erased_builders: &[#(#erased_builders),*],
            erased_sequence_builder: Some(#sequence_builder),
        }
    }
}

fn construction_witness_rows(construction: &ConstructionDeclaration) -> Vec<TokenStream> {
    construction
        .witnesses
        .iter()
        .map(|witness| {
            let name = witness.name.value.as_str();
            let class = match &witness.class {
                crate::model::WitnessClass::Stored { path } => {
                    let dotted = path.dotted();
                    quote! { ::deckmaste_construction_compiler::runtime::WitnessClassData::Stored { path: #dotted } }
                }
                crate::model::WitnessClass::Derived { combinator, .. } => {
                    let combinator = combinator.value.as_str();
                    quote! { ::deckmaste_construction_compiler::runtime::WitnessClassData::Derived { combinator: #combinator } }
                }
                crate::model::WitnessClass::Free { ty } => {
                    let ty = ty.value.as_str();
                    quote! { ::deckmaste_construction_compiler::runtime::WitnessClassData::Free { ty: #ty } }
                }
            };
            quote! {
                ::deckmaste_construction_compiler::runtime::WitnessData { name: #name, class: #class }
            }
        })
        .collect()
}

fn construction_form_rows(construction: &ConstructionDeclaration) -> Vec<TokenStream> {
    construction
        .forms
        .iter()
        .map(|form| {
            let name = form.name.value.as_str();
            let ordinal = form.ordinal.value;
            let guarded = form.guard.is_some() || form.value_guard.is_some();
            let erased_recognizer = form.value_guard.as_ref().map_or_else(
                || quote! { None },
                |_| {
                    let recognizer = quote::format_ident!(
                        "__erased_recognize_{}_{}",
                        construction.id.value,
                        form.ordinal.value,
                    );
                    quote! { Some(#recognizer) }
                },
            );
            let atoms: Vec<TokenStream> = form
                .surface
                .iter()
                .map(|atom| match atom {
                    crate::model::SurfaceAtom::Literal(s) => {
                        let s = s.value.as_str();
                        quote! { ::deckmaste_construction_compiler::runtime::AtomData::Literal(#s) }
                    }
                    crate::model::SurfaceAtom::Hole(path) => {
                        let dotted = path.dotted();
                        quote! { ::deckmaste_construction_compiler::runtime::AtomData::Hole(#dotted) }
                    }
                    crate::model::SurfaceAtom::Lexeme(path) => {
                        let dotted = path.dotted();
                        quote! { ::deckmaste_construction_compiler::runtime::AtomData::Lexeme(#dotted) }
                    }
                    crate::model::SurfaceAtom::Identity(path) => {
                        let dotted = path.dotted();
                        quote! { ::deckmaste_construction_compiler::runtime::AtomData::Identity(#dotted) }
                    }
                })
                .collect();
            quote! {
                ::deckmaste_construction_compiler::runtime::FormData {
                    name: #name,
                    ordinal: #ordinal,
                    guarded: #guarded,
                    erased_recognizer: #erased_recognizer,
                    atoms: &[#(#atoms),*],
                }
            }
        })
        .collect()
}

fn lens_application_row(application: &LensApplication) -> TokenStream {
    let owner = application.owner.value.as_str();
    let source = application.source.as_ref().map_or_else(
        || quote! { None },
        |source| {
            let source = source.value.as_str();
            quote! { Some(#source) }
        },
    );
    let edits = application.edits.iter().map(|edit| {
        let target = edit.target.dotted();
        let value = edit.value.value.as_str();
        let kind = match edit.kind {
            LensEditKind::Focus => {
                quote! { ::deckmaste_construction_compiler::runtime::LensEditKindData::Focus }
            }
            LensEditKind::Prepend => {
                quote! { ::deckmaste_construction_compiler::runtime::LensEditKindData::Prepend }
            }
            LensEditKind::Append => {
                quote! { ::deckmaste_construction_compiler::runtime::LensEditKindData::Append }
            }
        };
        let adapter = edit.adapter.as_ref().map_or_else(
            || quote! { None },
            |adapter| {
                let constructor = adapter.constructor.value.as_str();
                let destructurer = adapter.destructurer.value.as_str();
                quote! {
                    Some(::deckmaste_construction_compiler::runtime::LensEditAdapterData {
                        constructor: #constructor,
                        destructurer: #destructurer,
                    })
                }
            },
        );
        quote! {
            ::deckmaste_construction_compiler::runtime::LensEditData {
                target: #target,
                value: #value,
                kind: #kind,
                adapter: #adapter,
            }
        }
    });
    quote! {
        Some(::deckmaste_construction_compiler::runtime::LensApplicationData {
            owner: #owner,
            source: #source,
            edits: &[#(#edits),*],
        })
    }
}

fn construction_row(construction: &ConstructionDeclaration) -> TokenStream {
    let id = construction.id.value.as_str();
    let category = construction.category.value.as_str();
    let internal = construction.internal;
    let deserialize = construction.deserialize;
    let selection_unique = matches!(
        construction.selection,
        crate::model::SelectionPromise::Unique
    );
    let (own_type, bind_path) = match &construction.ast {
        AstShape::Own { name, .. } => {
            let n = name.value.as_str();
            (quote! { Some(#n) }, quote! { None })
        }
        AstShape::Bind { path, .. } => {
            let p = path.value.as_str();
            (quote! { None }, quote! { Some(#p) })
        }
    };
    let (projection_variant, erased_projector) = match &construction.projection {
        Some(projection) => {
            let projection = projection.value.as_str();
            let projector = quote::format_ident!("__erased_project_{}", construction.id.value);
            (quote! { Some(#projection) }, quote! { Some(#projector) })
        }
        None => (quote! { None }, quote! { None }),
    };
    let lens = construction
        .lens
        .as_ref()
        .map_or_else(|| quote! { None }, lens_application_row);
    let evidence = construction
        .evidence
        .as_ref()
        .map_or_else(|| quote! { None }, evidence_row);
    // `dominance` holds every edge this construction is named in, winner or
    // loser (see `edges_leaving_the_group_are_not_cycle_checked_here` in
    // validate.rs, whose edge lists exactly that shape). `dominates` means
    // "ids this construction beats" — only edges where it is the winner.
    let dominates: Vec<TokenStream> = construction
        .dominance
        .iter()
        .filter(|edge| edge.winner.value == construction.id.value)
        .map(|edge| {
            let loser = edge.loser.value.as_str();
            quote! { #loser }
        })
        .collect();
    let dominated_by: Vec<TokenStream> = construction
        .dominance
        .iter()
        .filter(|edge| edge.loser.value == construction.id.value)
        .map(|edge| {
            let winner = edge.winner.value.as_str();
            quote! { #winner }
        })
        .collect();
    let fields: Vec<TokenStream> = construction
        .ast
        .fields()
        .iter()
        .map(|field| field_row(field, Some(construction.id.value.as_str())))
        .collect();
    let witnesses = construction_witness_rows(construction);
    let forms = construction_form_rows(construction);
    let feature_combinators: Vec<TokenStream> = construction
        .constraints
        .iter()
        .filter_map(|constraint| {
            let Constraint::DeriveFeature {
                target,
                feature_type: _,
                combinator,
                args,
            } = constraint
            else {
                return None;
            };
            let target = target.dotted();
            let combinator = combinator.value.as_str();
            let args = args.iter().map(crate::model::FieldPath::dotted);
            Some(quote! {
                ::deckmaste_construction_compiler::runtime::FeatureCombinatorData {
                    target: #target,
                    combinator: #combinator,
                    args: &[#(#args),*],
                }
            })
        })
        .collect();
    let requirements: Vec<TokenStream> = construction
        .constraints
        .iter()
        .filter_map(|constraint| {
            let Constraint::Require(requirement) = constraint else {
                return None;
            };
            let description = render_predicate(&requirement.value);
            let predicate = predicate_row(&requirement.value);
            Some(quote! {
                ::deckmaste_construction_compiler::runtime::RequirementData {
                    description: #description,
                    predicate: #predicate,
                }
            })
        })
        .collect();
    let recognition_requirements: Vec<TokenStream> = construction
        .constraints
        .iter()
        .filter_map(|constraint| {
            let Constraint::Recognize(requirement) = constraint else {
                return None;
            };
            let description = render_predicate(&requirement.value);
            let predicate = predicate_row(&requirement.value);
            Some(quote! {
                ::deckmaste_construction_compiler::runtime::RequirementData {
                    description: #description,
                    predicate: #predicate,
                }
            })
        })
        .collect();
    let erased_partial_builder =
        quote::format_ident!("__erased_partial_build_{}", construction.id.value);
    let erased_builder = quote::format_ident!("__erased_build_{}", construction.id.value);
    let erased_linearizer = quote::format_ident!("__erased_linearize_{}", construction.id.value);
    quote! {
        ::deckmaste_construction_compiler::runtime::ConstructionData {
            id: #id,
            category: #category,
            internal: #internal,
            own_type: #own_type,
            bind_path: #bind_path,
            lens: #lens,
            projection_variant: #projection_variant,
            fields: &[#(#fields),*],
            witnesses: &[#(#witnesses),*],
            deserialize: #deserialize,
            selection_unique: #selection_unique,
            dominates: &[#(#dominates),*],
            dominated_by: &[#(#dominated_by),*],
            forms: &[#(#forms),*],
            requirements: &[#(#requirements),*],
            recognition_requirements: &[#(#recognition_requirements),*],
            feature_combinators: &[#(#feature_combinators),*],
            evidence: #evidence,
            erased_partial_builder: Some(#erased_partial_builder),
            erased_builder: Some(#erased_builder),
            erased_projector: #erased_projector,
            erased_linearizer: #erased_linearizer,
        }
    }
}

fn evidence_row(evidence: &crate::model::EvidenceDeclaration) -> TokenStream {
    let label = evidence.label.value.as_str();
    let kind = match evidence.kind {
        crate::model::EvidenceKind::Guard => quote! {
            ::deckmaste_construction_compiler::runtime::EvidenceKindData::Guard
        },
        crate::model::EvidenceKind::Feature => quote! {
            ::deckmaste_construction_compiler::runtime::EvidenceKindData::Feature
        },
        crate::model::EvidenceKind::Role => quote! {
            ::deckmaste_construction_compiler::runtime::EvidenceKindData::Role
        },
    };
    let source = match &evidence.source {
        crate::model::EvidenceSource::Requirement(path) => {
            let path = path.dotted();
            quote! { ::deckmaste_construction_compiler::runtime::EvidenceSourceData::Requirement(#path) }
        }
        crate::model::EvidenceSource::Output(path) => {
            let path = path.dotted();
            quote! { ::deckmaste_construction_compiler::runtime::EvidenceSourceData::Output(#path) }
        }
        crate::model::EvidenceSource::Field(path) => {
            let path = path.dotted();
            quote! { ::deckmaste_construction_compiler::runtime::EvidenceSourceData::Field(#path) }
        }
        crate::model::EvidenceSource::Category => quote! {
            ::deckmaste_construction_compiler::runtime::EvidenceSourceData::Category
        },
    };
    quote! {
        Some(::deckmaste_construction_compiler::runtime::EvidenceData {
            kind: #kind,
            label: #label,
            source: #source,
        })
    }
}

fn predicate_row(predicate: &Predicate) -> TokenStream {
    match predicate {
        Predicate::LenAtLeast { path, min } => {
            let path = path.dotted();
            quote! { ::deckmaste_construction_compiler::runtime::PredicateData::LenAtLeast { path: #path, min: #min } }
        }
        Predicate::LenIs { path, len } => {
            let path = path.dotted();
            quote! { ::deckmaste_construction_compiler::runtime::PredicateData::LenIs { path: #path, len: #len } }
        }
        Predicate::In { path, allowed } => {
            let path = path.dotted();
            quote! { ::deckmaste_construction_compiler::runtime::PredicateData::In { path: #path, allowed: &[#(#allowed),*] } }
        }
        Predicate::IsSome { path } => {
            let path = path.dotted();
            quote! { ::deckmaste_construction_compiler::runtime::PredicateData::IsSome { path: #path } }
        }
        Predicate::IsNone { path } => {
            let path = path.dotted();
            quote! { ::deckmaste_construction_compiler::runtime::PredicateData::IsNone { path: #path } }
        }
        Predicate::All(children) => {
            let children = children.iter().map(predicate_row);
            quote! { ::deckmaste_construction_compiler::runtime::PredicateData::All(&[#(#children),*]) }
        }
        Predicate::Any(children) => {
            let children = children.iter().map(predicate_row);
            quote! { ::deckmaste_construction_compiler::runtime::PredicateData::Any(&[#(#children),*]) }
        }
    }
}

fn field_row(binding: &FieldBinding, owner: Option<&str>) -> TokenStream {
    let name = binding.field.value.as_str();
    let kind = field_kind_row(&binding.kind, owner, name);
    quote! {
        ::deckmaste_construction_compiler::runtime::FieldData { name: #name, kind: #kind }
    }
}

fn field_kind_row(kind: &FieldKind, owner: Option<&str>, field: &str) -> TokenStream {
    match kind {
        FieldKind::Unit => {
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::Unit }
        }
        FieldKind::TupleProduct { fields } => {
            let fields = fields.iter().map(|field| field_row(field, None));
            quote! {
                ::deckmaste_construction_compiler::runtime::FieldKindData::TupleProduct {
                    fields: &[#(#fields),*],
                }
            }
        }
        FieldKind::StructProduct { fields } => {
            let fields = fields.iter().map(|field| field_row(field, None));
            quote! {
                ::deckmaste_construction_compiler::runtime::FieldKindData::StructProduct {
                    fields: &[#(#fields),*],
                }
            }
        }
        FieldKind::Identity {
            value_type,
            provider,
        } => {
            let value_type = value_type.value.as_str();
            let provider = provider.value.as_str();
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::Identity { value_type: #value_type, provider: #provider } }
        }
        FieldKind::Subtree { category, boxed } => {
            let category = category.value.as_str();
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::Subtree { category: #category, boxed: #boxed } }
        }
        FieldKind::TypedSubtree {
            value_type,
            category,
            boxed,
        } => {
            let value_type = value_type.value.as_str();
            let category = category.value.as_str();
            quote! {
                ::deckmaste_construction_compiler::runtime::FieldKindData::TypedSubtree {
                    value_type: #value_type,
                    category: #category,
                    boxed: #boxed,
                }
            }
        }
        FieldKind::Scalar { codec } => {
            let codec = codec.value.as_str();
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::Scalar { codec: #codec } }
        }
        FieldKind::TypedScalar { value_type, codec } => {
            let value_type = value_type.value.as_str();
            let codec = codec.value.as_str();
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::TypedScalar { value_type: #value_type, codec: #codec } }
        }
        FieldKind::SurfaceScalar { codec } => {
            let codec = codec.value.as_str();
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::SurfaceScalar { codec: #codec } }
        }
        FieldKind::Sequence { element } => {
            let element = element.value.as_str();
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::Sequence { element: #element } }
        }
        FieldKind::NonEmptySequence { element } => {
            let element = element.value.as_str();
            let owner = owner.expect("validated nonempty sequences are construction fields");
            let builder = quote::format_ident!("__erased_field_{}_{}", owner, field);
            quote! {
                ::deckmaste_construction_compiler::runtime::FieldKindData::NonEmptySequence {
                    element: #element,
                    erased_builder: #builder,
                }
            }
        }
        FieldKind::SeparatedNonEmptySequence { element, separator } => {
            let element = element.value.as_str();
            let separator = separator.value.as_str();
            let owner = owner.expect("validated separated sequences are construction fields");
            let builder = quote::format_ident!("__erased_field_{}_{}", owner, field);
            quote! {
                ::deckmaste_construction_compiler::runtime::FieldKindData::SeparatedNonEmptySequence {
                    element: #element,
                    separator: #separator,
                    erased_builder: #builder,
                }
            }
        }
        FieldKind::Sum { element, boxed } => {
            let element = element.value.as_str();
            quote! {
                ::deckmaste_construction_compiler::runtime::FieldKindData::Sum {
                    element: #element,
                    boxed: #boxed,
                }
            }
        }
        FieldKind::Product { element, boxed } => {
            let element = element.value.as_str();
            quote! {
                ::deckmaste_construction_compiler::runtime::FieldKindData::Product {
                    element: #element,
                    boxed: #boxed,
                }
            }
        }
        FieldKind::Optional { inner } => {
            let inner = field_kind_row(inner, owner, field);
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::Optional { inner: &#inner } }
        }
    }
}

fn reexports(group: &GroupDeclaration) -> Vec<TokenStream> {
    let module = quote::format_ident!("__constructions_{}", group.name.value);
    // `DeclarationViolation` is not re-exported: it lives once in
    // `runtime.rs` (not minted per group), so there is no per-module item to
    // export — every generated door already names it by absolute path.
    let mut items: Vec<proc_macro2::Ident> = Vec::new();
    for element in &group.elements {
        if element.bind_path.is_none() {
            items.push(pascal_ident(&element.name.value));
        } else if !element.variants.is_empty() {
            items.push(quote::format_ident!(
                "{}VariantRef",
                crate::model::pascal_case(&element.name.value)
            ));
            items.push(quote::format_ident!("parts_{}", element.name.value));
            for variant in &element.variants {
                items.push(quote::format_ident!(
                    "build_{}_{}",
                    element.name.value,
                    crate::model::snake_case(&variant.name.value),
                ));
            }
        }
    }
    for construction in &group.constructions {
        match &construction.ast {
            AstShape::Own { name, .. } => items.push(quote::format_ident!("{}", name.value)),
            AstShape::Bind { .. } => {
                items.push(quote::format_ident!("build_{}", construction.id.value));
                items.push(quote::format_ident!("parts_{}", construction.id.value));
            }
        }
        items.push(quote::format_ident!(
            "linearize_{}_with",
            construction.id.value
        ));
        items.push(quote::format_ident!(
            "linearize_{}_form_with",
            construction.id.value
        ));
        items.push(quote::format_ident!(
            "selected_{}_form",
            construction.id.value
        ));
    }
    if group.inverse_dispatch_family().is_some() {
        items.push(quote::format_ident!(
            "linearize_{}_group_with",
            group.name.value
        ));
    }
    for family in group.inverse_dispatch_families() {
        let category = crate::model::snake_case(
            &family
                .category
                .expect("category inverse families carry a category")
                .value,
        );
        items.push(quote::format_ident!(
            "linearize_{}_{}_with",
            group.name.value,
            category
        ));
    }
    for family in group.inverse_target_dispatch_families() {
        items.push(quote::format_ident!(
            "{}",
            crate::model::inverse_target_dispatcher_name(&group.name.value, &family.target.value,)
        ));
    }
    let mut typed_targets = Vec::new();
    for construction in &group.constructions {
        for constraint in &construction.constraints {
            let crate::model::Constraint::DeriveFeature {
                target,
                feature_type: Some(_),
                ..
            } = constraint
            else {
                continue;
            };
            let target = target.dotted().replace('.', "_");
            if !typed_targets.contains(&target) {
                typed_targets.push(target);
            }
        }
    }
    for target in typed_targets {
        items.push(quote::format_ident!(
            "reduce_{}_{}",
            group.name.value,
            target
        ));
    }
    items.push(declaration_ident(group));
    items
        .iter()
        .map(|item| quote! { pub use #module::#item; })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::validate;

    #[test]
    fn own_mode_emission_parses_and_seals() {
        let group = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&group).expect("fixture validates");
        let tokens = emit_group(&validated);
        let file: syn::File = syn::parse2(tokens).expect("emission is a valid Rust file");
        let rendered = prettyplease::unparse(&file);
        assert!(
            rendered.contains("mod __constructions_"),
            "sealed module present"
        );
        assert!(
            rendered.contains("pub fn try_new("),
            "validated constructor present"
        );
        assert!(
            rendered.contains("DeclarationViolation"),
            "error class present"
        );
        assert!(
            !rendered.contains("pub conjunction"),
            "construction fields stay private"
        );
        assert!(
            !rendered.contains("_mut(&mut self)"),
            "validated values have no mutation bypass"
        );
    }

    #[test]
    fn in_predicate_emits_a_closed_match() {
        let group = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("matches!(conjunction, Conjunction::And | Conjunction::Or)"),
            "closed-domain check, no wildcard: {rendered}"
        );
        assert!(
            rendered.contains("conjunction in [And, Or]"),
            "human-readable requirement string"
        );
    }

    #[test]
    fn evidence_metadata_emits_kind_label_and_declaration_source() {
        let group = crate::parse::parse_group(quote::quote! {
            group evidence_metadata;
            construction guarded: Phrase {
                own Guarded { conjunction: lex Conjunction, }
                require conjunction in [And];
                evidence guard "conjunction gate" from requirement conjunction;
                form only @ 0 = lex(conjunction);
            }
        })
        .expect("evidence fixture parses");
        let validated = validate(&group).expect("evidence fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("evidence: Some("),
            "evidence metadata is present: {rendered}",
        );
        assert!(
            rendered.contains("EvidenceKindData::Guard")
                && rendered.contains("label: \"conjunction gate\"")
                && rendered.contains("EvidenceSourceData::Requirement(")
                && rendered.contains("\"conjunction\","),
            "evidence metadata is declaration-derived: {rendered}",
        );
    }

    #[test]
    fn optional_typed_scalar_in_emits_an_optional_value_pattern() {
        let group = crate::parse::parse_group(quote::quote! {
            group typed_optional;
            construction typed: Phrase {
                own Typed {
                    value: opt lex SemanticValue via SurfaceCodec,
                }
                require value in [Variable];
                form only @ 0 = lex(value);
            }
        })
        .expect("typed optional scalar fixture parses");
        let validated = validate(&group).expect("typed optional scalar fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("matches!(value, None | Some(SemanticValue::Variable))"),
            "optional typed scalar uses its semantic value type inside Some: {rendered}"
        );
    }

    #[test]
    fn last_path_in_emits_is_none_or_with_nested_optional_reading() {
        // `.last()`'s vacuous-on-empty layer must nest OUTSIDE Task 1's
        // vacuous-on-absent optional-`In` reading: `rest.last().is_none_or(
        // |member| matches!(member.comma, None | Some(Comma::Present)))`.
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("comma".to_owned()),
                kind: crate::model::FieldKind::Optional {
                    inner: Box::new(crate::model::FieldKind::Scalar {
                        codec: crate::model::Spanned::call_site("Comma".to_owned()),
                    }),
                },
            }],
        });
        if let crate::model::AstShape::Own { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("rest".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("m".to_owned()),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("rest"),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("rest.last.comma"),
                    allowed: vec!["Present".to_owned()],
                }),
            ));
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        // prettyplease line-wraps the chained call, so match the pieces
        // rather than one contiguous string: `.last()` (vacuous on empty)
        // wrapping a closure whose body is Task 1's `None | Some(...)`
        // optional-`In` reading — the two vacuous layers nesting correctly.
        assert!(
            rendered.contains(".last()"),
            "`.last()` present: {rendered}"
        );
        assert!(
            rendered.contains(".is_none_or(|member| {"),
            "`.is_none_or` closure present: {rendered}"
        );
        assert!(
            rendered.contains("matches!(member.comma, None | Some(Comma::Present))"),
            "nested optional-In reading inside the closure: {rendered}"
        );
        assert!(
            rendered.contains("rest.last.comma in [Present]"),
            "human-readable requirement string"
        );
    }

    #[test]
    fn nonfinal_path_emits_all_but_the_last_member() {
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("comma".to_owned()),
                kind: crate::model::FieldKind::Scalar {
                    codec: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            }],
        });
        if let crate::model::AstShape::Own { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("rest".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("m".to_owned()),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("rest"),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("rest.nonfinal.comma"),
                    allowed: vec!["Present".to_owned()],
                }),
            ));
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains(".take(rest.len().saturating_sub(1))"),
            "nonfinal quantifier excludes exactly the final member: {rendered}",
        );
        assert!(
            rendered.contains(".all(|member| matches!(member.comma, Comma::Present))"),
            "nonfinal quantifier requires every member in the prefix: {rendered}",
        );
        assert!(
            rendered.contains("matches!(member.comma, Comma::Present)"),
            "element predicate is emitted inside the quantifier: {rendered}",
        );
        assert!(
            rendered.contains("rest.nonfinal.comma in [Present]"),
            "human-readable requirement preserves the authored path: {rendered}",
        );
    }

    #[test]
    fn bind_mode_emits_builder_and_destructurer() {
        let group = crate::validate::fixtures::minimal_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("pub fn build_"),
            "bind mode emits a checked builder: {rendered}"
        );
        assert!(
            rendered.contains("pub fn parts_"),
            "bind mode emits a destructurer: {rendered}"
        );
        assert!(
            !rendered.contains("fn try_new("),
            "bind mode still emits no sealed constructor"
        );
        assert!(
            !rendered.contains("impl<'de> serde::Deserialize"),
            "bind mode still has no serde door"
        );
        assert!(
            !rendered.contains("struct NounPhraseCoordination")
                && !rendered.contains("struct MinimalNode"),
            "bind mode emits no owned construction struct: {rendered}"
        );
    }

    #[test]
    fn bind_erased_builders_separate_partial_assembly_from_final_validation() {
        // Chart reduction needs the semantic adapter before all feature gates
        // have run. Every external erased ingress, however, must cross the
        // same checked typed builder (including its inverse recognizer).
        let group = crate::parse::parse_group(quote::quote! {
            group erased_boundary;
            construction wrapped: Node {
                bind Node via make_node, split_node {
                    head: hole Head,
                }
                form only @ 0 inverse check(is_wrapped) = head;
                selection unique;
            }
        })
        .expect("erased-boundary declaration parses");
        let validated = validate(&group).expect("erased-boundary declaration validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("fn __erased_partial_build_wrapped(")
                && rendered.contains("fn __erased_build_wrapped(")
                && rendered.contains("build_wrapped(head)"),
            "partial assembly and final checked ingress are distinct: {rendered}",
        );
        assert!(
            rendered.contains("erased_partial_builder: Some(__erased_partial_build_wrapped)")
                && rendered.contains("erased_builder: Some(__erased_build_wrapped)"),
            "runtime metadata exposes both intentional doors: {rendered}",
        );
    }

    #[test]
    fn typed_feature_callbacks_emit_one_declaration_order_dispatcher() {
        // Mutations caught: route language feature semantics through a
        // handwritten construction-ID table, erase the callback's feature
        // type, or silently ignore its declared field arguments.
        let group = crate::parse::parse_group(quote::quote! {
            group typed_features;
            construction base: Node {
                bind Node via make_base, split_base {
                    head: hole Head,
                }
                derive features: Feature = reduce_base(head);
                form only @ 0 = head;
                selection unique;
            }
            construction extend: Node {
                bind Node via make_extend, split_extend {
                    node: hole Node,
                    tail: hole Tail,
                }
                derive features: Feature = reduce_extend(node, tail);
                form only @ 0 = node tail;
                selection unique;
            }
        })
        .expect("typed feature declarations parse");
        let validated = validate(&group).expect("typed feature declarations validate");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("pub fn reduce_typed_features_features(")
                && rendered.contains("0 => reduce_base(")
                && rendered.contains("reduce_extend(")
                && rendered.contains("fields.get(1).copied().flatten()?"),
            "the compiler emits declaration-order typed callback dispatch: {rendered}",
        );
    }

    #[test]
    fn distinct_typed_output_targets_emit_independent_dispatchers() {
        let group = crate::parse::parse_group(quote::quote! {
            group typed_outputs;
            construction base: Node {
                bind Node via make_base, split_base { head: hole Head, }
                derive features: Feature = reduce_base(head);
                derive precedence: Feature = disprefer_base(head);
                form only @ 0 = head;
                selection unique;
            }
            construction extend: Node {
                bind Node via make_extend, split_extend { tail: hole Tail, }
                derive features: Feature = reduce_extend(tail);
                form only @ 0 = tail;
                selection unique;
            }
        })
        .expect("multiple typed output syntax parses");
        let validated = validate(&group).expect("distinct typed outputs validate independently");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("pub fn reduce_typed_outputs_features(")
                && rendered.contains("pub fn reduce_typed_outputs_precedence(")
                && rendered.contains("0 => disprefer_base("),
            "each declaration output owns its generated typed dispatcher: {rendered}",
        );
    }

    #[test]
    fn one_group_emits_an_inverse_dispatcher_for_each_declared_category() {
        // Mutations caught: infer one inverse target for the whole group, drop
        // a category because it binds the same Rust type as another category,
        // or restore a language-side family selector.
        let group = crate::parse::parse_group(quote::quote! {
            group category_families;
            construction left_one: Left {
                bind Shared via make_left_one, split_left_one { token: lex Token }
                form only @ 0 when check(is_left_one) = lex(token);
                selection unique;
            }
            construction left_two: Left {
                bind Shared via make_left_two, split_left_two { token: lex Token }
                form only @ 0 when check(is_left_two) = lex(token);
                selection unique;
            }
            construction right_one: Right {
                bind Shared via make_right_one, split_right_one { token: lex Token }
                form only @ 0 when check(is_right_one) = lex(token);
                selection unique;
            }
            construction right_two: Right {
                bind Shared via make_right_two, split_right_two { token: lex Token }
                form only @ 0 when check(is_right_two) = lex(token);
                selection unique;
            }
        })
        .expect("multi-category inverse fixture parses");
        let validated = validate(&group).expect("multi-category inverse fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("pub fn linearize_category_families_left_with<V>(")
                && rendered.contains("pub fn linearize_category_families_right_with<V>(")
                && rendered.contains("pub fn linearize_category_families_shared_with<V>("),
            "each category and its shared bound target own emitted inverse families: {rendered}",
        );
    }

    #[test]
    fn singleton_category_emits_its_inverse_dispatcher() {
        let group = crate::parse::parse_group(quote::quote! {
            group singleton_category;
            construction only: Only {
                bind Shared via make_only, split_only { token: lex Token }
                form only @ 0 when check(is_only) = lex(token);
                selection unique;
            }
        })
        .expect("singleton category fixture parses");
        let validated = validate(&group).expect("singleton category fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("pub fn linearize_singleton_category_only_with<V>("),
            "a one-construction category still owns an emitted inverse family: {rendered}",
        );
    }

    #[test]
    fn qualified_singleton_bind_target_emits_its_category_dispatcher() {
        // Mutation caught: derive every inverse dispatcher suffix from the
        // bound Rust type instead of using the declared category identity.
        let group = crate::parse::parse_group(quote::quote! {
            group qualified_singleton;
            construction only: Only {
                bind covered::path::Target via make_only, split_only { token: lex Token }
                form only @ 0 when check(is_only) = lex(token);
                selection unique;
            }
        })
        .expect("qualified singleton target parses");
        let validated = validate(&group).expect("qualified singleton target validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("pub fn linearize_qualified_singleton_only_with<V>(")
                && rendered.contains("value: &covered::path::Target"),
            "a qualified singleton target emits through its category identity: {rendered}",
        );
    }

    #[test]
    fn qualified_multi_category_bind_target_uses_a_path_safe_dispatcher_name() {
        // Mutation caught: feed the full `covered::path::Target` spelling to
        // `snake_case` and `format_ident!`, which panics on the retained `::`.
        let group = crate::parse::parse_group(quote::quote! {
            group qualified_targets;
            construction left: Left {
                bind covered::path::Target via make_left, split_left { token: lex Token }
                form only @ 0 when check(is_left) = lex(token);
                selection unique;
            }
            construction right: Right {
                bind covered::path::Target via make_right, split_right { token: lex Token }
                form only @ 0 when check(is_right) = lex(token);
                selection unique;
            }
        })
        .expect("qualified multi-category target parses");
        let validated = validate(&group).expect("qualified multi-category target validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains(
                "pub fn linearize_qualified_targets_path_636f76657265643a3a706174683a3a546172676574_with",
            ),
            "the full qualified target owns one deterministic Rust identifier: {rendered}",
        );
    }

    #[test]
    fn distinct_qualified_bind_targets_get_distinct_dispatcher_names() {
        // Mutation caught: retain only the terminal type segment when making
        // target dispatcher names, collapsing two `...::Target` families.
        let group = crate::parse::parse_group(quote::quote! {
            group distinct_qualified_targets;
            construction covered_left: CoveredLeft {
                bind covered::path::Target via make_covered_left, split_covered_left { token: lex Token }
                form only @ 0 when check(is_covered_left) = lex(token);
                selection unique;
            }
            construction covered_right: CoveredRight {
                bind covered::path::Target via make_covered_right, split_covered_right { token: lex Token }
                form only @ 0 when check(is_covered_right) = lex(token);
                selection unique;
            }
            construction other_left: OtherLeft {
                bind other::path::Target via make_other_left, split_other_left { token: lex Token }
                form only @ 0 when check(is_other_left) = lex(token);
                selection unique;
            }
            construction other_right: OtherRight {
                bind other::path::Target via make_other_right, split_other_right { token: lex Token }
                form only @ 0 when check(is_other_right) = lex(token);
                selection unique;
            }
        })
        .expect("distinct qualified target families parse");
        let validated = validate(&group).expect("distinct qualified target families validate");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains(
                "linearize_distinct_qualified_targets_path_636f76657265643a3a706174683a3a546172676574_with",
            ) && rendered.contains(
                "linearize_distinct_qualified_targets_path_6f746865723a3a706174683a3a546172676574_with",
            ),
            "distinct full Rust paths must not collapse to one target dispatcher: {rendered}",
        );
    }

    #[test]
    fn bound_enum_element_emits_total_typed_parts_and_builders() {
        let group = crate::parse::parse_group(quote::quote! {
            group enum_mapping;
            element nominal_complement bind NominalComplement {
                variant Adjective: hole AdjectivePhrase,
                variant EventClause: hole box IndependentClause,
            }
        })
        .expect("typed variant fixture parses");
        let validated = validate(&group).expect("typed variant fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("pub enum NominalComplementVariantRef<'a>"),
            "a heterogeneous borrowed view carries every payload type: {rendered}",
        );
        assert!(
            rendered.contains("Adjective(&'a AdjectivePhrase)")
                && rendered.contains("EventClause(&'a Box<IndependentClause>)"),
            "the borrowed view preserves variant order, payload types, and boxing: {rendered}",
        );
        assert!(
            rendered.contains("pub fn parts_nominal_complement(")
                && rendered.contains("NominalComplement::Adjective(payload)")
                && rendered.contains("NominalComplement::EventClause(payload)"),
            "the destructurer exhaustively names every Rust enum case: {rendered}",
        );
        assert!(
            rendered.contains("pub fn build_nominal_complement_adjective(")
                && rendered.contains("NominalComplement::Adjective(payload)"),
            "each variant has a typed builder: {rendered}",
        );
        assert!(
            rendered.contains("pub fn build_nominal_complement_event_clause(")
                && rendered.contains("payload: Box<IndependentClause>"),
            "boxed payload builders preserve the declared Rust type: {rendered}",
        );
        assert!(
            rendered.contains("variants: &[")
                && rendered.contains("ElementVariantData")
                && rendered.contains("name: \"EventClause\""),
            "runtime declaration metadata carries ordered variant rows: {rendered}",
        );
    }

    #[test]
    fn bound_element_surface_scalar_is_consumed_but_not_stored() {
        let group = crate::parse::parse_group(quote::quote! {
            group surface_fields;
            element member bind Member {
                comma: surface lex Comma,
                phrase: hole Phrase,
            }
            construction list: Phrase {
                own List {
                    first: hole box Phrase,
                    rest: seq member,
                }
                require rest.len() >= 1;
                form only @ 0 = first rest;
            }
        })
        .expect("surface-only scalar fixture parses");
        let validated = validate(&group).expect("surface-only scalar fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        let compact = rendered
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>();
        assert!(
            compact.contains("letMember{phrase}=value;"),
            "the exhaustive semantic destructurer omits the surface field: {rendered}",
        );
        assert!(
            compact.contains("let_comma:Comma=")
                && compact.contains("Ok(Box::new(Member{phrase}))"),
            "the erased builder consumes comma but excludes it from the semantic struct: {rendered}",
        );
        assert!(
            compact.contains("derived_sequence_scalar(")
                && compact.contains("\"member.comma\",\"Comma\",index,(rest).len(),"),
            "linearization derives the surface scalar from sequence position: {rendered}",
        );
        assert!(
            compact.contains("FieldKindData::SurfaceScalar{codec:\"Comma\","),
            "runtime metadata retains the surface codec: {rendered}",
        );
    }

    #[test]
    fn own_mode_emits_total_structural_linearizer() {
        let group = crate::parse::parse_group(quote::quote! {
            group linearization;
            element member bind Member {
                comma: lex Comma,
                conjunction: opt lex Conjunction,
                phrase: hole Phrase,
            }
            construction list: Phrase {
                own List {
                    first: hole box Phrase,
                    rest: seq member,
                }
                require rest.len() >= 1;
                witness oxford = stored rest.last.comma;
                form plain @ 0 when rest.last.comma in [Absent] = first rest;
                form oxford @ 1 when rest.last.comma in [Present] = first "," rest;
            }
        })
        .expect("linearization fixture parses");
        let validated = validate(&group).expect("linearization fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("pub fn linearize_list_with<V>(")
                && rendered.contains(
                    "V: ::deckmaste_construction_compiler::runtime::LinearizationVisitor"
                ),
            "own mode exposes the generated visitor entry: {rendered}",
        );
        assert!(
            rendered.contains("MultipleMatchingForms")
                && rendered.contains("NoMatchingForm")
                && rendered.contains("begin_form(\"list\", \"plain\", 0)"),
            "form choice is explicit and total: {rendered}",
        );
        assert!(
            rendered.contains("begin_sequence(\"rest\", (rest).len())")
                && rendered.contains("sequence_member(\"rest\", index)")
                && rendered.contains("end_sequence(\"rest\")"),
            "sequence structure reaches the visitor: {rendered}",
        );
        assert!(
            rendered.contains("optional(\"member.conjunction\", (&member.conjunction).is_some())")
                && rendered.contains("scalar(\"Comma\"")
                && rendered.contains("subtree(\"Phrase\""),
            "element optionals, scalar values, and typed holes are traversed: {rendered}",
        );
        assert!(
            rendered.contains("stored_witness(\"oxford\", \"rest.last.comma\""),
            "the stored form witness is replayed from its typed value: {rendered}",
        );
    }

    #[test]
    fn opt_in_construction_gains_serialize_only_when_requested() {
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.constructions[0].serialize = true;
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains(
                "#[derive(Debug, PartialEq, Eq)]\n    #[derive(serde::Serialize)]\n    pub struct MinimalNode"
            ),
            "opt-in owner derives Serialize: {rendered}",
        );
        assert!(
            !rendered.contains("serde::Deserialize"),
            "Serialize does not implicitly enable Deserialize: {rendered}",
        );

        let plain = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&plain).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            !rendered.contains("serde::Serialize"),
            "opt-out owner gains no Serialize derive: {rendered}",
        );
    }

    #[test]
    fn opt_in_construction_gains_validating_deserialize() {
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.constructions[0].deserialize = true;
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(rendered.contains("impl<'de> serde::Deserialize<'de> for MinimalNode"));
        assert!(rendered.contains("struct Raw"), "private mirror present");
        assert!(
            rendered.contains("MinimalNode::try_new(raw.conjunction)"),
            "routes through the validator"
        );
        // The opt-out sibling property: absence.
        let plain = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&plain).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            !rendered.contains("Deserialize"),
            "opt-out means no impl at all"
        );
    }

    #[test]
    fn elements_reached_by_serde_opt_ins_derive_each_requested_trait() {
        // Synthetic: opt-in construction with a seq field. The derive
        // presence pins traversal independently for each serde capability.
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            variants: vec![],
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("tag".to_owned()),
                kind: crate::model::FieldKind::Subtree {
                    category: crate::model::Spanned::call_site("FixturePhrase".to_owned()),
                    boxed: false,
                },
            }],
        });
        if let crate::model::AstShape::Own { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("members".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("m".to_owned()),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("members"),
            ));
        let mut serializable = group.clone();
        serializable.constructions[0].serialize = true;
        let validated = validate(&serializable).expect("serialize fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains(
                "    #[derive(Debug, PartialEq, Eq)]\n    #[derive(serde::Serialize)]\n    pub struct M"
            ),
            "serialize-reached element derives Serialize: {rendered}"
        );
        assert!(
            !rendered.contains(
                "    #[derive(Debug, PartialEq, Eq)]\n    #[derive(serde::Deserialize)]\n    pub struct M"
            ),
            "Serialize traversal does not enable Deserialize: {rendered}"
        );

        group.constructions[0].deserialize = true;
        let validated = validate(&group).expect("deserialize fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains(
                "    #[derive(Debug, PartialEq, Eq)]\n    #[derive(serde::Deserialize)]\n    pub struct M"
            ),
            "deserialize-reached element derives Deserialize: {rendered}"
        );
    }

    #[test]
    fn declaration_data_table_is_emitted() {
        let group = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(rendered.contains("pub static NOUN_COORDINATION_DECLARATION"));
        assert!(rendered.contains("::deckmaste_construction_compiler::runtime::GroupData"));
    }

    #[test]
    fn field_and_witness_rows_are_emitted() {
        let group = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("fields: &["),
            "declaration data carries field rows: {rendered}"
        );
        assert!(
            rendered.contains("name: \"conjunction\""),
            "conjunction field row present: {rendered}"
        );
        assert!(
            rendered.contains("FieldKindData::Scalar { codec : \"Conjunction\" }")
                || rendered.contains("codec: \"Conjunction\""),
            "conjunction's kind is a Conjunction-codec scalar row: {rendered}"
        );
    }

    #[test]
    fn free_witness_group_emits_payload_assertion() {
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.constructions[0]
            .witnesses
            .push(crate::model::WitnessDeclaration {
                name: crate::model::Spanned::call_site("gap".to_owned()),
                class: crate::model::WitnessClass::Free {
                    ty: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            });
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("fn __assert_free_witness_payloads"),
            "a free witness renders the payload assertion fn: {rendered}"
        );
        assert!(
            rendered.contains("assert_payload :: < Comma > ()")
                || rendered.contains("assert_payload::<Comma>()"),
            "the assertion instantiates for the free witness's type: {rendered}"
        );
    }

    #[test]
    fn no_free_witness_means_no_payload_assertion() {
        let group = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            !rendered.contains("__assert_free_witness_payloads"),
            "no free witness means no payload assertion fn: {rendered}"
        );
    }

    #[test]
    fn dominates_row_excludes_edges_where_this_construction_is_the_loser() {
        // This construction (`noun_phrase_coordination`) is the *loser* of
        // this edge, not the winner — an in-IR shape `validate()` permits
        // for external dominance (see
        // `edges_leaving_the_group_are_not_cycle_checked_here` in
        // validate.rs). Its own `dominates` row must not claim its own id.
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.constructions[0]
            .dominance
            .push(crate::model::DominanceEdge {
                winner: crate::model::Spanned::call_site("noun_phrase_nominal".to_owned()),
                loser: crate::model::Spanned::call_site("noun_phrase_coordination".to_owned()),
            });
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            !rendered.contains("dominates: &[\"noun_phrase_coordination\"]"),
            "construction must not claim to dominate itself via an edge where it is the loser: {rendered}"
        );
    }
}
