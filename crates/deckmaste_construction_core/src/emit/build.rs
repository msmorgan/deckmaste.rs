use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::emit::LocalAllocator;
use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::feature::FeatureValue;
use crate::identifier::BUILD_FUNCTION;
use crate::identifier::CHECKED_BUILD_FUNCTION;
use crate::identifier::emitted_ident;
use crate::identifier::feature_helper;
use crate::identifier::key as identifier_key;
use crate::identifier::snake_case;
use crate::model::TerminalBindingKind;
use crate::plan::DeclarationKey;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::plan::SourceDeclarationKind;
use crate::semantic::AtomPlan;
use crate::semantic::AtomTerminal;
use crate::semantic::BindingBuildExprPlan;
use crate::semantic::ConcordClassAuthorityPlan;
use crate::semantic::ConstructionPlan;
use crate::semantic::FieldCheckArgumentPlan;
use crate::semantic::FiniteDomainKindPlan;
use crate::semantic::FiniteValuePlan;
use crate::semantic::SemanticPlan;
use crate::semantic::StructuralFieldKindPlan;
use crate::semantic::StructuralFieldPlan;
use crate::semantic::UnsignedNumberKind;
use crate::semantic::ValueKindPlan;

pub(crate) fn emit(plan: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let build_function = ident(BUILD_FUNCTION);
    let checked_build_function = ident(CHECKED_BUILD_FUNCTION);
    let constructions = plan.constructions();
    let lowered = super::rules::lowered_rows(plan)?;
    let arms = lowered
        .iter()
        .map(|row| emit_rule_arm(plan, row))
        .collect::<syn::Result<Vec<_>>>()?;
    let origins = constructions
        .iter()
        .map(|construction| {
            DeclarationKey::new(
                SourceDeclarationKind::Construction,
                construction.construction_id(),
            )
        })
        .collect::<Vec<_>>();
    let tokens = quote! {
        #[expect(
            clippy::too_many_lines,
            reason = "the exhaustive generated-shape construction dispatch is intentionally flat"
        )]
        pub(super) fn #checked_build_function(
            rule: RuleId,
            children: &[BuildValue],
            context: &ParseContext<'_>,
        ) -> Result<Option<BuildValue>, BuildRejection> {
            match rule { #(#arms)* }
        }
    };
    let compatibility = quote! {
        pub(super) fn #build_function(
            rule: RuleId,
            children: &[BuildValue],
            context: &ParseContext<'_>,
        ) -> Option<BuildValue> {
            #checked_build_function(rule, children, context).ok().flatten()
        }
    };
    Ok(vec![
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: CHECKED_BUILD_FUNCTION.to_owned(),
            },
            tokens,
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: BUILD_FUNCTION.to_owned(),
            },
            compatibility,
            origins,
        ),
    ])
}

struct Lowering {
    patterns: Vec<TokenStream>,
    field_values: HashMap<String, TokenStream>,
    vocab_values: HashMap<String, syn::Ident>,
    role_features: HashMap<(String, Feature), LocalFeatureValue>,
    role_following_onsets: BTreeMap<String, syn::Ident>,
    output_following_onset: TokenStream,
    guards: Vec<TokenStream>,
    dynamic_numbers: Vec<syn::Ident>,
    constructor_map_local: Option<syn::Ident>,
    binders: LocalAllocator,
}

impl Default for Lowering {
    fn default() -> Self {
        let mut binders = LocalAllocator::default();
        for name in ["rule", "children", "context"] {
            binders.reserve(name);
        }
        Self {
            patterns: Vec::new(),
            field_values: HashMap::new(),
            vocab_values: HashMap::new(),
            role_features: HashMap::new(),
            role_following_onsets: BTreeMap::new(),
            output_following_onset: quote! { FeatureConstraint::<Onset>::Any },
            guards: Vec::new(),
            dynamic_numbers: Vec::new(),
            constructor_map_local: None,
            binders,
        }
    }
}

#[derive(Debug, Clone)]
enum LocalFeatureValue {
    Known(FeatureValue),
    Bound(syn::Ident),
    Computed(TokenStream),
}

#[derive(Clone)]
enum ResolvedFeatureValue {
    Known(FeatureValue),
    Bound(syn::Ident),
    Computed(TokenStream),
}

fn emit_rule_arm(plan: &SemanticPlan, row: &super::rules::RuleRowPlan) -> syn::Result<TokenStream> {
    match &row.build {
        super::rules::RuleBuildPlan::Construction {
            index,
            owner_states,
        } => emit_arm_from_plan(plan, row, &plan.constructions()[*index], owner_states),
        super::rules::RuleBuildPlan::Product {
            index,
            owner_states,
        } => emit_product_arm(plan, row, &plan.products()[*index], owner_states),
        super::rules::RuleBuildPlan::Sum {
            sum_index,
            alternative_index,
        } => emit_sum_arm(plan, row, *sum_index, *alternative_index),
        super::rules::RuleBuildPlan::Optional {
            owner,
            field_index,
            present,
        } => emit_optional_arm(plan, row, *owner, *field_index, *present),
        super::rules::RuleBuildPlan::Sequence {
            owner,
            field_index,
            state,
            helper_category,
        } => emit_sequence_arm(
            plan,
            row,
            *owner,
            *field_index,
            *state,
            helper_category.as_deref(),
        ),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one construction arm must keep its lowering, feature guards, and checked-boundary assembly together"
)]
fn emit_arm_from_plan(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    row: &ConstructionPlan,
    owner_states: &[super::rules::OwnerFieldBuildPlan],
) -> syn::Result<TokenStream> {
    let mut lowering = Lowering::default();
    let form_index = rule
        .form_index
        .ok_or_else(|| internal("construction rule has no form index"))?;
    let form = &row.forms()[form_index];
    for atom in form.atoms() {
        let state =
            atom_role(atom).and_then(|role| owner_states.iter().find(|field| field.role == role));
        if let Some(state) = state {
            let field_role =
                atom_role(atom).ok_or_else(|| internal("sequence atom has no role"))?;
            let field = row
                .fields()
                .iter()
                .find(|field| field.name_key() == field_role)
                .and_then(crate::semantic::ConstructionFieldPlan::structural_plan)
                .ok_or_else(|| internal("positional atom has no structural plan"))?;
            if let AtomPlan::Circumfix { prefix, .. } = atom {
                push_fixed_form_literal(&mut lowering, prefix);
            }
            if let AtomPlan::Marked {
                terminal, variant, ..
            } = atom
                && !matches!(
                    state.state,
                    super::rules::OwnerFieldBuildState::ZeroableAbsent
                )
            {
                lowering
                    .patterns
                    .push(fixed_lex_pattern(plan, terminal, variant)?);
            }
            match state.state {
                super::rules::OwnerFieldBuildState::Sequence(_) => {
                    lower_sequence_owner_field(
                        plan,
                        rule,
                        row.element_type(),
                        field,
                        state,
                        &mut lowering,
                    )?;
                }
                super::rules::OwnerFieldBuildState::ZeroableAbsent
                | super::rules::OwnerFieldBuildState::ZeroablePresent => {
                    lower_zeroable_owner_field(plan, rule, field, state, &mut lowering)?;
                }
            }
            if let AtomPlan::Circumfix { suffix, .. } = atom {
                push_fixed_form_literal(&mut lowering, suffix);
            }
        } else {
            lower_atom(plan, row, form, atom, &mut lowering)?;
        }
    }
    lower_feature_guards(plan, row, form, &mut lowering)?;
    lower_following_onset_constraints(form, &mut lowering);
    if let Some(guard) = lower_ordered_role_preemption_guard(plan, row, form, &lowering)? {
        lowering.guards.push(guard);
    }
    for field in row.fields() {
        let Some((function, arguments)) = field.field_check() else {
            continue;
        };
        if arguments.iter().any(|argument| {
            matches!(
                argument,
                FieldCheckArgumentPlan::VerbFrameRolePrepositions { .. }
            )
        }) {
            continue;
        }
        let value = lowering
            .field_values
            .get(&field.name_key())
            .cloned()
            .ok_or_else(|| internal("checked field has no lowered value"))?;
        let arguments = lower_checked_field_arguments(
            plan,
            row,
            &lowering,
            &CheckedFieldArgumentContext {
                owner: &field.name_key(),
                owner_value: &value,
                zeroable_owner: field.is_zeroable(),
                role_preemption: None,
            },
            arguments,
        )?;
        let checked_value = if field.is_zeroable() {
            quote! { (#value).as_ref() }
        } else {
            quote! { &#value }
        };
        if field.is_optional() {
            lowering.guards.push(quote! {
                (#value).as_ref().as_ref().map_or(true, |checked| {
                    #function(checked, #(#arguments),*)
                })
            });
        } else {
            lowering
                .guards
                .push(quote! { #function(#checked_value, #(#arguments),*) });
        }
    }
    if let Some(guard) = super::emit_form_guard_expression(row, form_index, |domain, value| {
        let guard_role = domain.role();
        match (domain.kind(), value) {
            (FiniteDomainKindPlan::Vocab { terminal, .. }, FiniteValuePlan::Vocab(variant)) => {
                let field_value = lowering
                    .field_values
                    .get(guard_role)
                    .cloned()
                    .ok_or_else(|| internal("form guard role has no lowered build value"))?;
                let terminal = ident(terminal);
                let variant = ident(variant);
                Ok(quote! { matches!(#field_value, #terminal::#variant) })
            }
            (
                FiniteDomainKindPlan::OptionalVocab { terminal, .. },
                FiniteValuePlan::OptionalVocab(variant),
            ) => {
                let field_value = lowering
                    .field_values
                    .get(guard_role)
                    .cloned()
                    .ok_or_else(|| internal("form guard role has no lowered build value"))?;
                if let Some(variant) = variant {
                    let terminal = ident(terminal);
                    let variant = ident(variant);
                    Ok(quote! { matches!(#field_value, Some(#terminal::#variant)) })
                } else {
                    Ok(quote! { #field_value.is_none() })
                }
            }
            (
                FiniteDomainKindPlan::OptionalPresence,
                FiniteValuePlan::OptionalPresence(present),
            ) => {
                let field_value = lowering
                    .field_values
                    .get(guard_role)
                    .cloned()
                    .ok_or_else(|| internal("form guard role has no lowered build value"))?;
                Ok(quote! { #field_value.is_some() == #present })
            }
            (FiniteDomainKindPlan::Feature { feature, .. }, FiniteValuePlan::Feature(value)) => {
                let actual = if guard_role.starts_with('@') {
                    resolve_feature_place(
                        plan,
                        row,
                        &lowering,
                        &FeaturePlace::Construction(*feature),
                        &mut HashSet::new(),
                    )?
                } else if let Some(value) = lowering
                    .role_features
                    .get(&(guard_role.to_owned(), *feature))
                    .map(local_feature_value)
                {
                    value
                } else {
                    let field = row.field(guard_role)?;
                    if !plan.terminal_has_feature(field.terminal(), *feature) {
                        return Err(internal("form guard feature role has no lowered value"));
                    }
                    let value = lowering.field_values.get(guard_role).ok_or_else(|| {
                        internal("form guard feature role has no lowered build value")
                    })?;
                    let helper = ident(&feature_helper(feature.key(), field.terminal()));
                    let value = if plan
                        .runtime_declaration_noun_for(field.terminal())
                        .is_some()
                        || plan
                            .runtime_declaration_determinative_for(field.terminal())
                            .is_some()
                    {
                        quote! { &#value }
                    } else {
                        quote! { #value }
                    };
                    ResolvedFeatureValue::Computed(quote! { #helper(#value) })
                };
                let expected = feature_value(*value);
                let actual = resolved_feature_value_tokens(&actual);
                Ok(quote! { #actual == #expected })
            }
            _ => Err(internal("form guard domain and assignment value disagree")),
        }
    })? {
        lowering.guards.push(guard);
    }
    let success = if let Some(dynamic_role) = dynamic_match_role(plan, row) {
        emit_dynamic_match(plan, row, &mut lowering, &dynamic_role)?
    } else {
        emit_success(plan, row, &mut lowering, None, None, None)?
    };
    let rule_id = ident(&rule.id);
    let patterns = &lowering.patterns;
    let guard = if lowering.guards.is_empty() {
        TokenStream::new()
    } else {
        let guards = &lowering.guards;
        quote! { if #(#guards)&&* }
    };
    Ok(quote! {
        RuleId::#rule_id => match children {
            [#(#patterns),*] #guard => #success,
            _ => Ok(None),
        },
    })
}

fn lower_following_onset_constraints(form: &crate::semantic::FormPlan, lowering: &mut Lowering) {
    let mut pending = Vec::new();
    for (role, constraint) in &lowering.role_following_onsets {
        let next_role = form
            .atoms()
            .iter()
            .position(|atom| atom_role(atom) == Some(role.as_str()))
            .and_then(|index| form.atoms().get(index + 1))
            .and_then(atom_role);
        let following_onset = next_role.and_then(|next| {
            lowering
                .role_features
                .get(&(next.to_owned(), Feature::Onset))
        });
        if let Some(following_onset) = following_onset {
            let following_onset =
                resolved_feature_value_tokens(&local_feature_value(following_onset));
            lowering.guards.push(quote! {
                match *#constraint {
                    FeatureConstraint::Any => true,
                    FeatureConstraint::Exact(expected) => expected == #following_onset,
                }
            });
        } else {
            pending.push(constraint.clone());
        }
    }
    let mut merged = quote! { FeatureConstraint::<Onset>::Any };
    for constraint in pending {
        let previous = merged;
        lowering.guards.push(quote! {
            match (#previous, *#constraint) {
                (FeatureConstraint::Any, _) | (_, FeatureConstraint::Any) => true,
                (FeatureConstraint::Exact(left), FeatureConstraint::Exact(right)) => left == right,
            }
        });
        merged = quote! {
            match (#previous, *#constraint) {
                (FeatureConstraint::Any, right) => right,
                (left, FeatureConstraint::Any) => left,
                (FeatureConstraint::Exact(left), FeatureConstraint::Exact(_)) => FeatureConstraint::Exact(left),
            }
        };
    }
    lowering.output_following_onset = merged;
}

struct CheckedFieldArgumentContext<'a> {
    owner: &'a str,
    owner_value: &'a TokenStream,
    zeroable_owner: bool,
    role_preemption: Option<&'a TokenStream>,
}

fn lower_checked_field_arguments(
    plan: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
    context: &CheckedFieldArgumentContext<'_>,
    arguments: &[FieldCheckArgumentPlan],
) -> syn::Result<Vec<TokenStream>> {
    arguments
        .iter()
        .map(|argument| {
            let FieldCheckArgumentPlan::Feature { role, feature } = argument else {
                let FieldCheckArgumentPlan::VerbFrameRolePrepositions { role } = argument else {
                    unreachable!("sealed field-check argument changed variant")
                };
                if let Some(role_preemption) = context.role_preemption {
                    return Ok(role_preemption.clone());
                }
                let value = if role == context.owner {
                    context.owner_value.clone()
                } else {
                    lowering
                        .field_values
                        .get(role)
                        .cloned()
                        .ok_or_else(|| internal("verb-frame projection has no lowered value"))?
                };
                let source = row.field(role)?;
                let helper = ident(&feature_helper(
                    "verb_frame_role_prepositions",
                    source.terminal(),
                ));
                return Ok(quote! { #helper(&#value) });
            };
            if let Some(value) = lowering
                .role_features
                .get(&(role.clone(), *feature))
                .map(local_feature_value)
            {
                let value = resolved_feature_value_tokens(&value);
                return Ok(if context.zeroable_owner && role == context.owner {
                    quote! { Some(#value) }
                } else {
                    value
                });
            }
            if context.zeroable_owner && role == context.owner {
                return Ok(quote! { None });
            }
            let value = if role == context.owner {
                context.owner_value.clone()
            } else {
                lowering
                    .field_values
                    .get(role)
                    .cloned()
                    .ok_or_else(|| internal("checked field feature has no lowered value"))?
            };
            let source = row.field(role)?;
            match source.kind() {
                crate::semantic::ConstructionFieldKind::Category => {
                    let helper = ident(&feature_helper(feature.key(), source.terminal()));
                    Ok(quote! { #helper(&#value) })
                }
                crate::semantic::ConstructionFieldKind::Lex => {
                    if !plan.terminal_has_feature(source.terminal(), *feature) {
                        return Err(internal(
                            "checked lexical field feature has no lowered value",
                        ));
                    }
                    let helper = ident(&feature_helper(feature.key(), source.terminal()));
                    if plan
                        .runtime_declaration_noun_for(source.terminal())
                        .is_some()
                        || plan
                            .runtime_declaration_determinative_for(source.terminal())
                            .is_some()
                    {
                        Ok(quote! { #helper(&#value) })
                    } else {
                        Ok(quote! { #helper(#value) })
                    }
                }
                crate::semantic::ConstructionFieldKind::Identity => Err(internal(
                    "checked field feature source is not a category or lexical value",
                )),
            }
        })
        .collect()
}

fn lower_ordered_role_preemption_guard(
    plan: &SemanticPlan,
    row: &ConstructionPlan,
    form: &crate::semantic::FormPlan,
    lowering: &Lowering,
) -> syn::Result<Option<TokenStream>> {
    let checked_fields = row
        .fields()
        .iter()
        .filter_map(|field| {
            let (function, arguments) = field.field_check()?;
            arguments
                .iter()
                .any(|argument| {
                    matches!(
                        argument,
                        FieldCheckArgumentPlan::VerbFrameRolePrepositions { .. }
                    )
                })
                .then_some((field, function, arguments))
        })
        .collect::<Vec<_>>();
    if checked_fields.is_empty() {
        return Ok(None);
    }

    let mut projection_sources = checked_fields.iter().flat_map(|(_, _, arguments)| {
        arguments.iter().filter_map(|argument| match argument {
            FieldCheckArgumentPlan::VerbFrameRolePrepositions { role } => Some(role.as_str()),
            FieldCheckArgumentPlan::Feature { .. } => None,
        })
    });
    let source_role = projection_sources
        .next()
        .ok_or_else(|| internal("ordered role preemption has no projection source"))?;
    if projection_sources.any(|role| role != source_role) {
        return Err(internal(
            "ordered role preemption has multiple projection sources",
        ));
    }
    let source_value = lowering
        .field_values
        .get(source_role)
        .ok_or_else(|| internal("ordered role preemption source has no lowered value"))?;
    let source = row.field(source_role)?;
    let accessor = ident(&feature_helper(
        "verb_frame_role_prepositions",
        source.terminal(),
    ));
    let role_preemption = quote! { &mut role_preemption };
    let mut operations = Vec::new();
    let mut checked_roles = HashSet::new();

    for atom in form.atoms() {
        match atom.value_atom() {
            AtomPlan::LexFixed {
                terminal, variant, ..
            } => {
                let terminal = syn::LitStr::new(terminal, Span::call_site());
                let variant = syn::LitStr::new(variant, Span::call_site());
                operations.push(quote! {
                    role_preemption.fill(VerbFrameRolePreposition::new(#terminal, #variant));
                });
            }
            AtomPlan::Marked {
                role,
                terminal,
                variant,
                ..
            } => {
                let value = lowering
                    .field_values
                    .get(role)
                    .ok_or_else(|| internal("marked role preemption field has no lowered value"))?;
                let terminal = syn::LitStr::new(terminal, Span::call_site());
                let variant = syn::LitStr::new(variant, Span::call_site());
                let field = row.field(role)?;
                if field.is_optional() || field.is_zeroable() {
                    operations.push(quote! {
                        if (#value).is_some() {
                            role_preemption.fill(VerbFrameRolePreposition::new(#terminal, #variant));
                        }
                    });
                } else {
                    operations.push(quote! {
                        role_preemption.fill(VerbFrameRolePreposition::new(#terminal, #variant));
                    });
                }
                if let Some(operation) =
                    lower_ordered_role_field_check(plan, row, lowering, role, &role_preemption)?
                {
                    operations.push(operation);
                    checked_roles.insert(role.clone());
                }
            }
            AtomPlan::Lex { role, terminal } => {
                let value = lowering.field_values.get(role).ok_or_else(|| {
                    internal("lexical role preemption field has no lowered value")
                })?;
                let field = row.field(role)?;
                if plan.terminal_has_feature(terminal, Feature::PrepositionComplementKind) {
                    let helper = ident(&feature_helper("verb_frame_role_preposition", terminal));
                    if field.is_optional() || field.is_zeroable() {
                        operations.push(quote! {
                            if let Some(value) = (#value).as_ref() {
                                role_preemption.fill(#helper(*value));
                            }
                        });
                    } else {
                        operations.push(quote! {
                            role_preemption.fill(#helper(#value));
                        });
                    }
                }
                if let Some(operation) =
                    lower_ordered_role_field_check(plan, row, lowering, role, &role_preemption)?
                {
                    operations.push(operation);
                    checked_roles.insert(role.clone());
                }
            }
            AtomPlan::Category { role, .. }
            | AtomPlan::Identity { role, .. }
            | AtomPlan::Noun { role, .. } => {
                if let Some(operation) =
                    lower_ordered_role_field_check(plan, row, lowering, role, &role_preemption)?
                {
                    operations.push(operation);
                    checked_roles.insert(role.clone());
                }
            }
            AtomPlan::Literal(_)
            | AtomPlan::SentenceInitialLiteral(_)
            | AtomPlan::StructuralLiteral(_)
            | AtomPlan::VerbFixed { .. }
            | AtomPlan::OpenDeclaration(_)
            | AtomPlan::Bound { .. }
            | AtomPlan::Circumfix { .. } => {}
        }
    }

    if checked_fields
        .iter()
        .any(|(field, _, _)| !checked_roles.contains(&field.name_key()))
    {
        return Err(internal(
            "ordered role preemption field is absent from its form",
        ));
    }

    Ok(Some(quote! {
        {
            let mut role_preemption = VerbFrameRolePreemption::new(
                #accessor(&#source_value)
            );
            let mut accepted = true;
            #(#operations)*
            accepted
        }
    }))
}

fn lower_ordered_role_field_check(
    plan: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
    role: &str,
    role_preemption: &TokenStream,
) -> syn::Result<Option<TokenStream>> {
    let field = row.field(role)?;
    let Some((function, arguments)) = field.field_check() else {
        return Ok(None);
    };
    if !arguments.iter().any(|argument| {
        matches!(
            argument,
            FieldCheckArgumentPlan::VerbFrameRolePrepositions { .. }
        )
    }) {
        return Ok(None);
    }
    let value = lowering
        .field_values
        .get(role)
        .cloned()
        .ok_or_else(|| internal("ordered role preemption field has no lowered value"))?;
    let arguments = lower_checked_field_arguments(
        plan,
        row,
        lowering,
        &CheckedFieldArgumentContext {
            owner: role,
            owner_value: &value,
            zeroable_owner: field.is_zeroable(),
            role_preemption: Some(role_preemption),
        },
        arguments,
    )?;
    let checked_value = if field.is_zeroable() {
        quote! { (#value).as_ref() }
    } else {
        quote! { &#value }
    };
    let check = if field.is_optional() {
        quote! {
            (#value).as_ref().as_ref().map_or(true, |checked| {
                #function(checked, #(#arguments),*)
            })
        }
    } else {
        quote! { #function(#checked_value, #(#arguments),*) }
    };
    Ok(Some(quote! { accepted = accepted && #check; }))
}

fn emit_product_arm(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    product: &crate::semantic::ProductPlan,
    owner_states: &[super::rules::OwnerFieldBuildPlan],
) -> syn::Result<TokenStream> {
    let mut binders = LocalAllocator::default();
    for name in ["rule", "children", "context"] {
        binders.reserve(name);
    }
    let mut patterns = Vec::new();
    let mut values = Vec::new();
    for field in product.fields() {
        let state = owner_states.iter().find(|state| state.role == field.name());
        let (mut field_patterns, mut value) = if let Some(state) = state {
            match state.state {
                super::rules::OwnerFieldBuildState::Sequence(_) => {
                    let (patterns, value, features, guards) = lower_sequence_owner_value(
                        plan,
                        rule,
                        product.name(),
                        field,
                        state,
                        &mut binders,
                    )?;
                    debug_assert!(features.is_empty());
                    debug_assert!(guards.is_empty());
                    (patterns, value)
                }
                super::rules::OwnerFieldBuildState::ZeroableAbsent
                | super::rules::OwnerFieldBuildState::ZeroablePresent => {
                    let mut zeroable = Lowering::default();
                    std::mem::swap(&mut binders, &mut zeroable.binders);
                    let lowered =
                        lower_zeroable_owner_value(plan, rule, field, state, &mut zeroable)?;
                    std::mem::swap(&mut binders, &mut zeroable.binders);
                    lowered
                }
            }
        } else {
            lower_product_field(plan, product.name(), field, &mut binders)?
        };
        patterns.append(&mut field_patterns);
        if field.is_recursive() {
            value = quote! { Box::new(#value) };
        }
        values.push((ident(field.name()), value));
    }
    let product_type = ident(product.name());
    let success = if product.requires_constructor() {
        let arguments = values.iter().map(|(_, value)| value);
        quote! {
            #product_type::try_new(#(#arguments),*)
                .map(BuildValue::#product_type)
                .map(Some)
        }
    } else if values.is_empty() {
        quote! { Ok(Some(BuildValue::#product_type(#product_type))) }
    } else {
        let fields = values.iter().map(|(name, value)| quote! { #name: #value });
        quote! { Ok(Some(BuildValue::#product_type(#product_type { #(#fields),* }))) }
    };
    let rule_id = ident(&rule.id);
    Ok(quote! {
        RuleId::#rule_id => match children {
            [#(#patterns),*] => #success,
            _ => Ok(None),
        },
    })
}

fn emit_sum_arm(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    sum_index: usize,
    alternative_index: usize,
) -> syn::Result<TokenStream> {
    let sum = &plan.sums()[sum_index];
    let alternative = &sum.alternatives()[alternative_index];
    let mut binders = LocalAllocator::default();
    for name in ["rule", "children", "context"] {
        binders.reserve(name);
    }
    let concord_class_carry = plan.sum_carries_concord_class(sum.name());
    let fused_head_license_carry = plan.carries_feature(sum.name(), Feature::FusedHeadLicense);
    let (value, concord_class, fused_head_license) = if concord_class_carry {
        let (value, concord_class, fused_head_license) =
            lower_value_with_concord_class(plan, alternative.value(), "value", &mut binders)?;
        (value, Some(concord_class), fused_head_license)
    } else if fused_head_license_carry {
        let (value, fused_head_license) =
            lower_value_with_fused_head_license(plan, alternative.value(), "value", &mut binders)?;
        (value, None, Some(fused_head_license))
    } else {
        (
            lower_value(plan, alternative.value(), "value", &mut binders)?,
            None,
            None,
        )
    };
    let payload = if alternative.is_recursive() {
        let expression = value.expression;
        quote! { Box::new(#expression) }
    } else {
        value.expression
    };
    let sum_type = ident(sum.name());
    let variant = ident(alternative.name());
    let patterns = vec![value.pattern];
    let concord_class = concord_class.map(|concord_class| quote! { , *#concord_class });
    let fused_head_license =
        fused_head_license.map(|fused_head_license| quote! { , *#fused_head_license });
    let rule_id = ident(&rule.id);
    Ok(quote! {
        RuleId::#rule_id => match children {
            [#(#patterns),*] => Ok(Some(BuildValue::#sum_type(
                #sum_type::#variant(#payload) #concord_class #fused_head_license, FeatureConstraint::Any
            ))),
            _ => Ok(None),
        },
    })
}

fn emit_optional_arm(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    owner: super::rules::StructuralOwner,
    field_index: usize,
    present: bool,
) -> syn::Result<TokenStream> {
    let (owner_name, field) = structural_owner_field(plan, owner, field_index);
    let StructuralFieldKindPlan::Optional(value) = field.kind() else {
        return Err(internal(
            "optional build row does not name an optional field",
        ));
    };
    let carrier = ident(&carrier_variant(owner_name, field)?);
    let rule_id = ident(&rule.id);
    if !present {
        return Ok(quote! {
            RuleId::#rule_id => match children {
                [] => Ok(Some(BuildValue::#carrier(None))),
                _ => Ok(None),
            },
        });
    }
    let mut binders = LocalAllocator::default();
    for name in ["rule", "children", "context"] {
        binders.reserve(name);
    }
    let value = lower_value(plan, value, "item", &mut binders)?;
    let marker = match rule.rhs.as_slice() {
        [
            super::rules::RuleSymbolPlan::MarkedMarker {
                construction_index,
                form_index,
                atom_index,
            },
            super::rules::RuleSymbolPlan::Value(_),
        ] => {
            let AtomPlan::Marked {
                terminal, variant, ..
            } = &plan.constructions()[*construction_index].forms()[*form_index].atoms()
                [*atom_index]
            else {
                return Err(internal(
                    "optional marked-role row does not name a marked atom",
                ));
            };
            Some(fixed_lex_pattern(plan, terminal, variant)?)
        }
        [super::rules::RuleSymbolPlan::Value(_)] => None,
        _ => return Err(internal("optional present row has an invalid RHS")),
    };
    let pattern = value.pattern;
    let patterns = marker.into_iter().chain(std::iter::once(pattern));
    let expression = value.expression;
    Ok(quote! {
        RuleId::#rule_id => match children {
            [#(#patterns),*] => Ok(Some(BuildValue::#carrier(Some(#expression)))),
            _ => Ok(None),
        },
    })
}

fn emit_sequence_arm(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    owner: super::rules::StructuralOwner,
    field_index: usize,
    state: super::rules::SequenceBuildState,
    helper_category: Option<&str>,
) -> syn::Result<TokenStream> {
    let (owner_name, field) = structural_owner_field(plan, owner, field_index);
    let StructuralFieldKindPlan::Sequence { .. } = field.kind() else {
        return Err(internal(
            "sequence build row does not name a sequence field",
        ));
    };
    let carrier = ident(&carrier_variant(owner_name, field)?);
    let rule_id = ident(&rule.id);
    let mut binders = LocalAllocator::default();
    for name in ["rule", "children", "context"] {
        binders.reserve(name);
    }
    let sequence_features = plan.sequence_features(owner_name, field.name());
    let lowered = lower_sequence_rhs(
        plan,
        &rule.rhs,
        &carrier,
        helper_category,
        sequence_features,
        &rule.state,
        &mut binders,
    )?;
    let patterns = lowered.patterns.clone();
    let success = match state {
        super::rules::SequenceBuildState::Singleton => {
            let parts = exact_sequence_parts(lowered, 1, &rule.state)?;
            emit_exact_sequence_success(&carrier, sequence_features, parts)
        }
        super::rules::SequenceBuildState::Exact(length)
        | super::rules::SequenceBuildState::PositionalExactTail(length) => {
            let parts = exact_sequence_parts(lowered, length, &rule.state)?;
            emit_exact_sequence_success(&carrier, sequence_features, parts)
        }
        super::rules::SequenceBuildState::Last => {
            let parts = exact_sequence_parts(lowered, 2, &rule.state)?;
            emit_exact_sequence_success(&carrier, sequence_features, parts)
        }
        super::rules::SequenceBuildState::Recursive | super::rules::SequenceBuildState::Middle => {
            let parts = prefixed_sequence_parts(lowered, 1, &rule.state)?;
            emit_prefixed_sequence_success(&carrier, sequence_features, parts)
        }
    };
    Ok(quote! {
        RuleId::#rule_id => match children {
            [#(#patterns),*] => #success,
            _ => Ok(None),
        },
    })
}

struct LoweredValue {
    pattern: TokenStream,
    expression: TokenStream,
}

#[derive(Default)]
struct SequenceItemFeatures {
    concord_class: Option<syn::Ident>,
    number: Option<syn::Ident>,
    onset: Option<syn::Ident>,
    possessive_ending: Option<syn::Ident>,
}

fn lower_value_with_sequence_features(
    plan: &SemanticPlan,
    value: &ValueKindPlan,
    preferred: &str,
    features: &[Feature],
    binders: &mut LocalAllocator,
) -> syn::Result<(LoweredValue, SequenceItemFeatures)> {
    if features.is_empty() {
        return Ok((
            lower_value(plan, value, preferred, binders)?,
            SequenceItemFeatures::default(),
        ));
    }
    match value {
        ValueKindPlan::Category(name)
            if features
                .iter()
                .all(|feature| category_carries_sequence_feature(plan, name, *feature)) =>
        {
            let variant = ident(name);
            let binding = binders.allocate(preferred);
            let mut item_features = SequenceItemFeatures::default();
            if features.contains(&Feature::ConcordClass) {
                item_features.concord_class =
                    Some(binders.allocate(&format!("{preferred}_concord_class")));
            }
            if features.contains(&Feature::Number) {
                item_features.number = Some(binders.allocate(&format!("{preferred}_number")));
            }
            if features.contains(&Feature::Onset) {
                item_features.onset = Some(binders.allocate(&format!("{preferred}_onset")));
            }
            if features.contains(&Feature::PossessiveEnding) {
                item_features.possessive_ending =
                    Some(binders.allocate(&format!("{preferred}_possessive_ending")));
            }
            let concord_class = plan.category_carries_concord_class(name).then(|| {
                item_features
                    .concord_class
                    .as_ref()
                    .map_or_else(|| quote! { , _ }, |value| quote! { , #value })
            });
            let cardinality = plan
                .category_carries_cardinality(name)
                .then(|| quote! { , _ });
            let number = plan.category_carries_number(name).then(|| {
                item_features
                    .number
                    .as_ref()
                    .map_or_else(|| quote! { , _ }, |value| quote! { , #value })
            });
            let determiner_number = plan
                .carries_feature(name, Feature::DeterminerNumber)
                .then(|| quote! { , _ });
            let fused_head_license = plan
                .carries_feature(name, Feature::FusedHeadLicense)
                .then(|| quote! { , _ });
            let nominal_license = plan
                .carries_feature(name, Feature::NominalLicense)
                .then(|| quote! { , _ });
            let onset = plan.category_carries_onset(name).then(|| {
                item_features
                    .onset
                    .as_ref()
                    .map_or_else(|| quote! { , _ }, |value| quote! { , #value })
            });
            let possessive_ending = plan.category_carries_possessive_ending(name).then(|| {
                item_features
                    .possessive_ending
                    .as_ref()
                    .map_or_else(|| quote! { , _ }, |value| quote! { , #value })
            });
            let following_onset = carries_following_onset(plan, name).then(|| quote! { , _ });
            Ok((
                LoweredValue {
                    pattern: quote! { BuildValue::#variant(#binding #concord_class #cardinality #number #determiner_number #fused_head_license #nominal_license #onset #possessive_ending #following_onset) },
                    expression: quote! { #binding.clone() },
                },
                item_features,
            ))
        }
        ValueKindPlan::Sum(name)
            if features == [Feature::ConcordClass] && plan.sum_carries_concord_class(name) =>
        {
            let variant = ident(name);
            let binding = binders.allocate(preferred);
            let concord_class = binders.allocate(&format!("{preferred}_concord_class"));
            let fused_head_license = plan
                .carries_feature(name, Feature::FusedHeadLicense)
                .then(|| quote! { , _ });
            Ok((
                LoweredValue {
                    pattern: quote! { BuildValue::#variant(#binding, #concord_class #fused_head_license, _) },
                    expression: quote! { #binding.clone() },
                },
                SequenceItemFeatures {
                    concord_class: Some(concord_class),
                    ..SequenceItemFeatures::default()
                },
            ))
        }
        ValueKindPlan::Product(_)
        | ValueKindPlan::Category(_)
        | ValueKindPlan::Sum(_)
        | ValueKindPlan::Lex(_)
        | ValueKindPlan::Identity(_) => Err(internal(
            "sequence-feature value lowering received a value without every feature",
        )),
    }
}

fn category_carries_sequence_feature(plan: &SemanticPlan, name: &str, feature: Feature) -> bool {
    match feature {
        Feature::ConcordClass => plan.category_carries_concord_class(name),
        Feature::Number => plan.category_carries_number(name),
        Feature::Onset => plan.category_carries_onset(name),
        Feature::PossessiveEnding => plan.category_carries_possessive_ending(name),
        _ => false,
    }
}

fn lower_value_with_concord_class(
    plan: &SemanticPlan,
    value: &ValueKindPlan,
    preferred: &str,
    binders: &mut LocalAllocator,
) -> syn::Result<(LoweredValue, syn::Ident, Option<syn::Ident>)> {
    let concord_class = binders.allocate(&format!("{preferred}_concord_class"));
    match value {
        ValueKindPlan::Category(name) if plan.category_carries_concord_class(name) => {
            let variant = ident(name);
            let binding = binders.allocate(preferred);
            let cardinality = plan
                .category_carries_cardinality(name)
                .then(|| quote! { , _ });
            let number = plan.category_carries_number(name).then(|| quote! { , _ });
            let determiner_number = plan
                .carries_feature(name, Feature::DeterminerNumber)
                .then(|| quote! { , _ });
            let carried_fused_head_license = plan
                .carries_feature(name, Feature::FusedHeadLicense)
                .then(|| binders.allocate(&format!("{preferred}_fused_head_license")));
            let fused_head_license = carried_fused_head_license
                .as_ref()
                .map(|value| quote! { , #value });
            let nominal_license = plan
                .carries_feature(name, Feature::NominalLicense)
                .then(|| quote! { , _ });
            let onset = plan.category_carries_onset(name).then(|| quote! { , _ });
            let possessive_ending = plan
                .category_carries_possessive_ending(name)
                .then(|| quote! { , _ });
            let following_onset = carries_following_onset(plan, name).then(|| quote! { , _ });
            Ok((
                LoweredValue {
                    pattern: quote! { BuildValue::#variant(
                        #binding, #concord_class #cardinality #number #determiner_number #fused_head_license #nominal_license #onset #possessive_ending #following_onset
                    ) },
                    expression: quote! { #binding.clone() },
                },
                concord_class,
                carried_fused_head_license,
            ))
        }
        ValueKindPlan::Sum(name) if plan.sum_carries_concord_class(name) => {
            let variant = ident(name);
            let binding = binders.allocate(preferred);
            let carried_fused_head_license = plan
                .carries_feature(name, Feature::FusedHeadLicense)
                .then(|| binders.allocate(&format!("{preferred}_fused_head_license")));
            let fused_head_license = carried_fused_head_license
                .as_ref()
                .map(|value| quote! { , #value });
            Ok((
                LoweredValue {
                    pattern: quote! { BuildValue::#variant(#binding, #concord_class #fused_head_license, _) },
                    expression: quote! { #binding.clone() },
                },
                concord_class,
                carried_fused_head_license,
            ))
        }
        ValueKindPlan::Category(_)
        | ValueKindPlan::Product(_)
        | ValueKindPlan::Sum(_)
        | ValueKindPlan::Lex(_)
        | ValueKindPlan::Identity(_) => Err(internal(
            "concord_class-bearing value lowering received a value without concord_class",
        )),
    }
}

fn lower_value_with_fused_head_license(
    plan: &SemanticPlan,
    value: &ValueKindPlan,
    preferred: &str,
    binders: &mut LocalAllocator,
) -> syn::Result<(LoweredValue, syn::Ident)> {
    let fused_head_license = binders.allocate(&format!("{preferred}_fused_head_license"));
    match value {
        ValueKindPlan::Category(name) if plan.carries_feature(name, Feature::FusedHeadLicense) => {
            let variant = ident(name);
            let binding = binders.allocate(preferred);
            let concord_class = plan
                .category_carries_concord_class(name)
                .then(|| quote! { , _ });
            let cardinality = plan
                .category_carries_cardinality(name)
                .then(|| quote! { , _ });
            let number = plan.category_carries_number(name).then(|| quote! { , _ });
            let determiner_number = plan
                .carries_feature(name, Feature::DeterminerNumber)
                .then(|| quote! { , _ });
            let nominal_license = plan
                .carries_feature(name, Feature::NominalLicense)
                .then(|| quote! { , _ });
            let onset = plan.category_carries_onset(name).then(|| quote! { , _ });
            let possessive_ending = plan
                .category_carries_possessive_ending(name)
                .then(|| quote! { , _ });
            let following_onset = carries_following_onset(plan, name).then(|| quote! { , _ });
            Ok((
                LoweredValue {
                    pattern: quote! { BuildValue::#variant(
                        #binding #concord_class #cardinality #number #determiner_number,
                        #fused_head_license #nominal_license #onset #possessive_ending #following_onset
                    ) },
                    expression: quote! { #binding.clone() },
                },
                fused_head_license,
            ))
        }
        ValueKindPlan::Sum(name) if plan.carries_feature(name, Feature::FusedHeadLicense) => {
            let variant = ident(name);
            let binding = binders.allocate(preferred);
            let concord_class = plan.sum_carries_concord_class(name).then(|| quote! { , _ });
            Ok((
                LoweredValue {
                    pattern: quote! { BuildValue::#variant(#binding #concord_class, #fused_head_license, _) },
                    expression: quote! { #binding.clone() },
                },
                fused_head_license,
            ))
        }
        ValueKindPlan::Category(_)
        | ValueKindPlan::Product(_)
        | ValueKindPlan::Sum(_)
        | ValueKindPlan::Lex(_)
        | ValueKindPlan::Identity(_) => Err(internal(
            "fused-head-bearing value lowering received a value without that feature",
        )),
    }
}

fn lower_value(
    plan: &SemanticPlan,
    value: &ValueKindPlan,
    preferred: &str,
    binders: &mut LocalAllocator,
) -> syn::Result<LoweredValue> {
    match value {
        ValueKindPlan::Category(name) => {
            let variant = ident(name);
            let binding = binders.allocate(preferred);
            let concord_class = plan
                .category_carries_concord_class(name)
                .then(|| quote! { , _ });
            let cardinality = plan
                .category_carries_cardinality(name)
                .then(|| quote! { , _ });
            let number = plan.category_carries_number(name).then(|| quote! { , _ });
            let determiner_number = plan
                .carries_feature(name, Feature::DeterminerNumber)
                .then(|| quote! { , _ });
            let fused_head_license = plan
                .carries_feature(name, Feature::FusedHeadLicense)
                .then(|| quote! { , _ });
            let nominal_license = plan
                .carries_feature(name, Feature::NominalLicense)
                .then(|| quote! { , _ });
            let onset = plan.category_carries_onset(name).then(|| quote! { , _ });
            let possessive_ending = plan
                .category_carries_possessive_ending(name)
                .then(|| quote! { , _ });
            let following_onset = carries_following_onset(plan, name).then(|| quote! { , _ });
            Ok(LoweredValue {
                pattern: quote! { BuildValue::#variant(#binding #concord_class #cardinality #number #determiner_number #fused_head_license #nominal_license #onset #possessive_ending #following_onset) },
                expression: quote! { #binding.clone() },
            })
        }
        ValueKindPlan::Product(name) => {
            let variant = ident(name);
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::#variant(#binding) },
                expression: quote! { #binding.clone() },
            })
        }
        ValueKindPlan::Sum(name) => {
            let variant = ident(name);
            let binding = binders.allocate(preferred);
            let concord_class = plan.sum_carries_concord_class(name).then(|| quote! { , _ });
            let fused_head_license = plan
                .carries_feature(name, Feature::FusedHeadLicense)
                .then(|| quote! { , _ });
            Ok(LoweredValue {
                pattern: quote! { BuildValue::#variant(#binding #concord_class #fused_head_license, _) },
                expression: quote! { #binding.clone() },
            })
        }
        ValueKindPlan::Lex(name) | ValueKindPlan::Identity(name) => {
            lower_terminal_value(plan, name, preferred, binders)
        }
    }
}

fn lower_terminal_value(
    plan: &SemanticPlan,
    terminal_name: &str,
    preferred: &str,
    binders: &mut LocalAllocator,
) -> syn::Result<LoweredValue> {
    match plan.atom_terminal(terminal_name)? {
        AtomTerminal::Vocab(vocab) => {
            let leaf = ident(vocab.name());
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(Leaf::#leaf(#binding)) },
                expression: quote! { *#binding },
            })
        }
        AtomTerminal::Lexeme(_) => {
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(Leaf::Noun { noun: #binding, number: _, onset: _, possessive_ending: _ }) },
                expression: quote! { *#binding },
            })
        }
        AtomTerminal::ContextIdentity(identity) => {
            let leaf = identity.aggregate_ident();
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(Leaf::#leaf(#binding)) },
                expression: quote! { *#binding },
            })
        }
        AtomTerminal::CatalogIdentity { plan, .. } => {
            let ty = plan.ident();
            let provider = plan.provider();
            let identity = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! {
                    BuildValue::Leaf(Leaf::CatalogIdentity {
                        provider: CatalogProvider::#provider,
                        canonical_identity: #identity,
                        onset: _,
                        possessive_ending: _,
                    })
                },
                expression: quote! { #ty::from_canonical(#identity.clone()) },
            })
        }
        AtomTerminal::SignedDecimal(codec) => {
            let leaf = codec.codec_ident();
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(Leaf::#leaf(#binding)) },
                expression: quote! { #binding.clone() },
            })
        }
        AtomTerminal::UnsignedNumber(codec) => {
            let leaf = codec.codec_ident();
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(Leaf::#leaf(#binding)) },
                expression: quote! { #binding.clone() },
            })
        }
        AtomTerminal::DeclarationNoun { plan, .. } => {
            let leaf = plan.codec_ident();
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(Leaf::#leaf { noun: #binding, number: _, onset: _, possessive_ending: _ }) },
                expression: quote! { #binding.clone() },
            })
        }
        AtomTerminal::DeclarationDeterminative { plan, .. } => {
            let leaf = plan.codec_ident();
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(Leaf::#leaf { value: #binding, onset: _, following_onset: _, number_license: _, fused_head_license: _, nominal_license: _ }) },
                expression: quote! { #binding.clone() },
            })
        }
        AtomTerminal::DeclarationTerm {
            terminal_index,
            plan,
        } => {
            let term = plan.codec_ident();
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! {
                    BuildValue::Leaf(Leaf::DeclarationTerm {
                        terminal_index: #terminal_index,
                        id: #binding,
                        onset: _,
                        possessive_ending: _,
                    })
                },
                expression: quote! {
                    #term::from_reading(#binding.clone())
                        .expect("scanned declaration term preserves its codec kind")
                },
            })
        }
        AtomTerminal::DeclarationVerb { plan, .. } => {
            let leaf = plan.codec_ident();
            let binding = binders.allocate(preferred);
            let feature = match plan.feature_axis() {
                Feature::ConcordClass => quote! { concord_class: _, },
                Feature::Participle => quote! {},
                _ => unreachable!("validated declaration verb feature axis is closed"),
            };
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(Leaf::#leaf { verb: #binding, #feature onset: _ }) },
                expression: quote! { #binding.clone() },
            })
        }
        AtomTerminal::Binding(binding) => {
            let build = binding
                .build()
                .ok_or_else(|| internal("atom-capable binding has no build metadata"))?;
            let variant = build.variant();
            let slots = build.slots();
            let pattern_names = slots
                .iter()
                .enumerate()
                .map(|(index, _)| {
                    let name = if slots.len() == 1 {
                        preferred.to_owned()
                    } else {
                        format!("{preferred}_{index}")
                    };
                    binders.allocate(&name)
                })
                .collect::<Vec<_>>();
            let substitutions = slots
                .iter()
                .zip(&pattern_names)
                .map(|(declared, emitted)| (identifier_key(declared), quote! { #emitted }))
                .collect::<HashMap<_, _>>();
            let inner = if build.construct_is_direct_slot() {
                quote! { Leaf::#variant(#(#pattern_names),*) }
            } else {
                quote! { Leaf::#variant(BoundLeaf::#variant(#(#pattern_names),*)) }
            };
            let construct = lower_build_recipe(build.recipe(), &substitutions)?;
            let expression = if build.construct_is_direct_slot() {
                match binding.kind() {
                    TerminalBindingKind::Codec => quote! { #construct.clone() },
                    TerminalBindingKind::Identity => quote! { *#construct },
                }
            } else {
                construct
            };
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(#inner) },
                expression,
            })
        }
    }
}

fn lower_product_field(
    plan: &SemanticPlan,
    owner: &str,
    field: &StructuralFieldPlan,
    binders: &mut LocalAllocator,
) -> syn::Result<(Vec<TokenStream>, TokenStream)> {
    match field.kind() {
        StructuralFieldKindPlan::Required(value) => {
            let value = lower_value(plan, value, field.name(), binders)?;
            Ok((vec![value.pattern], value.expression))
        }
        StructuralFieldKindPlan::Zeroable(_) => Err(internal(
            "zeroable product field reached the unexpanded build path",
        )),
        StructuralFieldKindPlan::Optional(_) | StructuralFieldKindPlan::Sequence { .. } => {
            if matches!(
                field.kind(),
                StructuralFieldKindPlan::Sequence {
                    surface,
                    ..
                } if matches!(
                    surface.separator(),
                    Some(crate::semantic::SeparatorPlan::Positional(_))
                )
            ) {
                return Err(internal(
                    "positional product field reached the unexpanded build path",
                ));
            }
            let carrier = ident(&carrier_variant(owner, field)?);
            let binding = binders.allocate(field.name());
            Ok((
                vec![quote! { BuildValue::#carrier(#binding) }],
                quote! { #binding.clone() },
            ))
        }
    }
}

fn lower_zeroable_owner_field(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    field: &StructuralFieldPlan,
    state: &super::rules::OwnerFieldBuildPlan,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let (patterns, value) = lower_zeroable_owner_value(plan, rule, field, state, lowering)?;
    lowering.patterns.extend(patterns);
    lowering.field_values.insert(field.name().to_owned(), value);
    Ok(())
}

fn lower_zeroable_owner_value(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    field: &StructuralFieldPlan,
    state: &super::rules::OwnerFieldBuildPlan,
    lowering: &mut Lowering,
) -> syn::Result<(Vec<TokenStream>, TokenStream)> {
    let StructuralFieldKindPlan::Zeroable(value) = field.kind() else {
        return Err(internal(
            "zeroable owner state does not name a zeroable field",
        ));
    };
    let symbols = rule
        .rhs
        .get(state.rhs_start..state.rhs_end)
        .ok_or_else(|| internal("zeroable owner RHS range exceeds the lowered rule"))?;
    match state.state {
        super::rules::OwnerFieldBuildState::ZeroableAbsent => {
            if !symbols.is_empty() {
                return Err(internal("zeroable absent owner state has a nonempty RHS"));
            }
            Ok((Vec::new(), quote! { None }))
        }
        super::rules::OwnerFieldBuildState::ZeroablePresent => {
            let [super::rules::RuleSymbolPlan::Value(item)] = symbols else {
                return Err(internal(
                    "zeroable present owner state does not contain exactly one value",
                ));
            };
            if item != value {
                return Err(internal(
                    "zeroable present owner value disagrees with its structural field",
                ));
            }
            let item = if let ValueKindPlan::Category(name) = item {
                let variant = ident(name);
                let binding = lowering.binders.allocate(field.name());
                let concord_class = plan
                    .category_carries_concord_class(name)
                    .then(|| quote! { , _ });
                let cardinality = plan
                    .category_carries_cardinality(name)
                    .then(|| quote! { , _ });
                let number = plan.category_carries_number(name).then(|| quote! { , _ });
                let determiner_number =
                    plan.carries_feature(name, Feature::DeterminerNumber)
                        .then(|| {
                            let feature = lowering
                                .binders
                                .allocate(&format!("{}_determiner_number", field.name()));
                            lowering.role_features.insert(
                                (field.name().to_owned(), Feature::DeterminerNumber),
                                LocalFeatureValue::Bound(feature.clone()),
                            );
                            quote! { , #feature }
                        });
                let fused_head_license = plan
                    .carries_feature(name, Feature::FusedHeadLicense)
                    .then(|| {
                        let feature = lowering
                            .binders
                            .allocate(&format!("{}_fused_head_license", field.name()));
                        lowering.role_features.insert(
                            (field.name().to_owned(), Feature::FusedHeadLicense),
                            LocalFeatureValue::Bound(feature.clone()),
                        );
                        quote! { , #feature }
                    });
                let nominal_license =
                    plan.carries_feature(name, Feature::NominalLicense)
                        .then(|| {
                            let feature = lowering
                                .binders
                                .allocate(&format!("{}_nominal_license", field.name()));
                            lowering.role_features.insert(
                                (field.name().to_owned(), Feature::NominalLicense),
                                LocalFeatureValue::Bound(feature.clone()),
                            );
                            quote! { , #feature }
                        });
                let onset = plan.category_carries_onset(name).then(|| quote! { , _ });
                let possessive_ending = plan
                    .category_carries_possessive_ending(name)
                    .then(|| quote! { , _ });
                let following_onset = carries_following_onset(plan, name).then(|| {
                    let feature = lowering
                        .binders
                        .allocate(&format!("{}_following_onset", field.name()));
                    lowering
                        .role_following_onsets
                        .insert(field.name().to_owned(), feature.clone());
                    quote! { , #feature }
                });
                LoweredValue {
                    pattern: quote! { BuildValue::#variant(#binding #concord_class #cardinality #number #determiner_number #fused_head_license #nominal_license #onset #possessive_ending #following_onset) },
                    expression: quote! { #binding.clone() },
                }
            } else {
                lower_value(plan, item, field.name(), &mut lowering.binders)?
            };
            let pattern = item.pattern;
            let expression = item.expression;
            Ok((vec![pattern], quote! { Some(#expression) }))
        }
        super::rules::OwnerFieldBuildState::Sequence(_) => {
            Err(internal("zeroable owner field has a sequence state"))
        }
    }
}

fn lower_sequence_owner_field(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    owner: &str,
    field: &StructuralFieldPlan,
    state: &super::rules::OwnerFieldBuildPlan,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let (patterns, value, sequence_feature, guards) =
        lower_sequence_owner_value(plan, rule, owner, field, state, &mut lowering.binders)?;
    lowering.patterns.extend(patterns);
    lowering.field_values.insert(field.name().to_owned(), value);
    for (feature, value) in sequence_feature {
        lowering.role_features.insert(
            (field.name().to_owned(), feature),
            LocalFeatureValue::Bound(value),
        );
    }
    lowering.guards.extend(guards);
    Ok(())
}

type LoweredSequenceOwnerValue = (
    Vec<TokenStream>,
    TokenStream,
    Vec<(Feature, syn::Ident)>,
    Vec<TokenStream>,
);

fn lower_sequence_owner_value(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    owner: &str,
    field: &StructuralFieldPlan,
    state: &super::rules::OwnerFieldBuildPlan,
    binders: &mut LocalAllocator,
) -> syn::Result<LoweredSequenceOwnerValue> {
    let StructuralFieldKindPlan::Sequence { .. } = field.kind() else {
        return Err(internal(
            "sequence owner state does not name a sequence field",
        ));
    };
    let super::rules::OwnerFieldBuildState::Sequence(sequence_state) = state.state else {
        return Err(internal("sequence owner field has a non-sequence state"));
    };
    let symbols = rule
        .rhs
        .get(state.rhs_start..state.rhs_end)
        .ok_or_else(|| internal("sequence owner RHS range exceeds the lowered rule"))?;
    let carrier = ident(&carrier_variant(owner, field)?);
    let sequence_features = plan.sequence_features(owner, field.name());
    let lowered = lower_sequence_rhs(
        plan,
        symbols,
        &carrier,
        state.helper_category.as_deref(),
        sequence_features,
        &rule.state,
        binders,
    )?;
    let patterns = lowered.patterns.clone();
    let (value, feature_value, guards) = match sequence_state {
        super::rules::SequenceOwnerState::UniformEmpty
        | super::rules::SequenceOwnerState::PositionalEmpty => {
            let parts = exact_sequence_parts(lowered, 0, &rule.state)?;
            debug_assert!(
                parts.concord_classes.is_empty()
                    && parts.numbers.is_empty()
                    && parts.onsets.is_empty()
                    && parts.possessive_endings.is_empty()
            );
            let values = parts.values;
            (quote! { vec![#(#values),*] }, Vec::new(), Vec::new())
        }
        super::rules::SequenceOwnerState::UniformNonEmpty => {
            let parts = prefixed_sequence_parts(lowered, 0, &rule.state)?;
            debug_assert!(parts.values.is_empty());
            let tail = parts.tail;
            let (feature_value, guards) = sequence_owner_feature_values(
                sequence_features,
                &SequenceFeatureBindings {
                    concord_classes: Vec::new(),
                    tail_concord_class: parts.tail_concord_class,
                    numbers: Vec::new(),
                    tail_number: parts.tail_number,
                    onsets: Vec::new(),
                    tail_onset: parts.tail_onset,
                    possessive_endings: Vec::new(),
                    tail_possessive_ending: parts.tail_possessive_ending,
                },
            )?;
            (quote! { #tail.clone() }, feature_value, guards)
        }
        super::rules::SequenceOwnerState::PositionalSingleton => {
            let parts = exact_sequence_parts(lowered, 1, &rule.state)?;
            let values = parts.values;
            let (feature_value, guards) = sequence_owner_feature_values(
                sequence_features,
                &SequenceFeatureBindings {
                    concord_classes: parts.concord_classes,
                    tail_concord_class: None,
                    numbers: parts.numbers,
                    tail_number: None,
                    onsets: parts.onsets,
                    tail_onset: None,
                    possessive_endings: parts.possessive_endings,
                    tail_possessive_ending: None,
                },
            )?;
            (quote! { vec![#(#values),*] }, feature_value, guards)
        }
        super::rules::SequenceOwnerState::PositionalPair => {
            let parts = exact_sequence_parts(lowered, 2, &rule.state)?;
            let values = parts.values;
            let (feature_value, guards) = sequence_owner_feature_values(
                sequence_features,
                &SequenceFeatureBindings {
                    concord_classes: parts.concord_classes,
                    tail_concord_class: None,
                    numbers: parts.numbers,
                    tail_number: None,
                    onsets: parts.onsets,
                    tail_onset: None,
                    possessive_endings: parts.possessive_endings,
                    tail_possessive_ending: None,
                },
            )?;
            (quote! { vec![#(#values),*] }, feature_value, guards)
        }
        super::rules::SequenceOwnerState::PositionalThreePlus
        | super::rules::SequenceOwnerState::PositionalMinimumPlus(_) => {
            let parts = prefixed_sequence_parts(lowered, 1, &rule.state)?;
            let values = parts.values;
            let tail = parts.tail;
            let prefix_len = values.len();
            let (feature_value, guards) = sequence_owner_feature_values(
                sequence_features,
                &SequenceFeatureBindings {
                    concord_classes: parts.concord_classes,
                    tail_concord_class: parts.tail_concord_class,
                    numbers: parts.numbers,
                    tail_number: parts.tail_number,
                    onsets: parts.onsets,
                    tail_onset: parts.tail_onset,
                    possessive_endings: parts.possessive_endings,
                    tail_possessive_ending: parts.tail_possessive_ending,
                },
            )?;
            (
                quote! {{
                    let mut values = Vec::with_capacity(#prefix_len + #tail.len());
                    #(values.push(#values);)*
                    values.extend(#tail.iter().cloned());
                    values
                }},
                feature_value,
                guards,
            )
        }
    };
    Ok((patterns, value, feature_value, guards))
}

struct LoweredSequenceRhs {
    patterns: Vec<TokenStream>,
    values: Vec<TokenStream>,
    tail: Option<syn::Ident>,
    concord_classes: Vec<syn::Ident>,
    tail_concord_class: Option<syn::Ident>,
    numbers: Vec<syn::Ident>,
    tail_number: Option<syn::Ident>,
    onsets: Vec<syn::Ident>,
    tail_onset: Option<syn::Ident>,
    possessive_endings: Vec<syn::Ident>,
    tail_possessive_ending: Option<syn::Ident>,
}

fn lower_sequence_rhs(
    plan: &SemanticPlan,
    symbols: &[super::rules::RuleSymbolPlan],
    carrier: &syn::Ident,
    helper_category: Option<&str>,
    features: &[Feature],
    state: &str,
    binders: &mut LocalAllocator,
) -> syn::Result<LoweredSequenceRhs> {
    let mut patterns = Vec::new();
    let mut values = Vec::new();
    let mut tail = None;
    let mut concord_classes = Vec::new();
    let mut tail_concord_class = None;
    let mut numbers = Vec::new();
    let mut tail_number = None;
    let mut onsets = Vec::new();
    let mut tail_onset = None;
    let mut possessive_endings = Vec::new();
    let mut tail_possessive_ending = None;
    for (index, symbol) in symbols.iter().enumerate() {
        match symbol {
            super::rules::RuleSymbolPlan::Value(value)
            | super::rules::RuleSymbolPlan::AdjacentValue(value) => {
                let (value, item_features) = lower_value_with_sequence_features(
                    plan,
                    value,
                    &format!("item_{index}"),
                    features,
                    binders,
                )?;
                patterns.push(value.pattern);
                values.push(value.expression);
                concord_classes.extend(item_features.concord_class);
                numbers.extend(item_features.number);
                onsets.extend(item_features.onset);
                possessive_endings.extend(item_features.possessive_ending);
            }
            super::rules::RuleSymbolPlan::Helper(category) => {
                if Some(category.as_str()) != helper_category {
                    return Err(internal(&format!(
                        "{state} sequence RHS names a different helper category"
                    )));
                }
                if tail.is_some() {
                    return Err(internal(&format!(
                        "{state} sequence RHS contains more than one tail"
                    )));
                }
                let binding = binders.allocate("tail");
                let concord_class = features
                    .contains(&Feature::ConcordClass)
                    .then(|| binders.allocate("tail_concord_class"));
                let number = features
                    .contains(&Feature::Number)
                    .then(|| binders.allocate("tail_number"));
                let onset = features
                    .contains(&Feature::Onset)
                    .then(|| binders.allocate("tail_onset"));
                let possessive_ending = features
                    .contains(&Feature::PossessiveEnding)
                    .then(|| binders.allocate("tail_possessive_ending"));
                let feature_patterns = features
                    .iter()
                    .map(|feature| match feature {
                        Feature::ConcordClass => concord_class.as_ref(),
                        Feature::Number => number.as_ref(),
                        Feature::Onset => onset.as_ref(),
                        Feature::PossessiveEnding => possessive_ending.as_ref(),
                        _ => None,
                    })
                    .map(|feature| feature.expect("validated sequence feature has a binding"));
                patterns.push(quote! { BuildValue::#carrier(#binding #(, #feature_patterns)*) });
                tail = Some(binding);
                tail_concord_class = concord_class;
                tail_number = number;
                tail_onset = onset;
                tail_possessive_ending = possessive_ending;
            }
            super::rules::RuleSymbolPlan::Surface(surface) => {
                patterns.push(fixed_surface_pattern(plan, &surface.atom)?);
            }
            super::rules::RuleSymbolPlan::Authored { .. }
            | super::rules::RuleSymbolPlan::MarkedMarker { .. }
            | super::rules::RuleSymbolPlan::BoundAffix { .. }
            | super::rules::RuleSymbolPlan::CircumfixAffix { .. } => {
                return Err(internal(&format!(
                    "{state} sequence RHS contains a construction atom"
                )));
            }
        }
    }
    Ok(LoweredSequenceRhs {
        patterns,
        values,
        tail,
        concord_classes,
        tail_concord_class,
        numbers,
        tail_number,
        onsets,
        tail_onset,
        possessive_endings,
        tail_possessive_ending,
    })
}

struct ExactSequenceParts {
    values: Vec<TokenStream>,
    concord_classes: Vec<syn::Ident>,
    numbers: Vec<syn::Ident>,
    onsets: Vec<syn::Ident>,
    possessive_endings: Vec<syn::Ident>,
}

fn exact_sequence_parts(
    lowered: LoweredSequenceRhs,
    expected: usize,
    state: &str,
) -> syn::Result<ExactSequenceParts> {
    if lowered.tail.is_some() || lowered.values.len() != expected {
        return Err(internal(&format!(
            "{state} sequence RHS must contain exactly {expected} values and no tail"
        )));
    }
    if lowered.tail_concord_class.is_some()
        || lowered.tail_number.is_some()
        || lowered.tail_onset.is_some()
        || lowered.tail_possessive_ending.is_some()
        || (!lowered.concord_classes.is_empty() && lowered.concord_classes.len() != expected)
        || (!lowered.numbers.is_empty() && lowered.numbers.len() != expected)
        || (!lowered.onsets.is_empty() && lowered.onsets.len() != expected)
        || (!lowered.possessive_endings.is_empty() && lowered.possessive_endings.len() != expected)
    {
        return Err(internal(&format!(
            "{state} sequence RHS has inconsistent concord_class bindings"
        )));
    }
    Ok(ExactSequenceParts {
        values: lowered.values,
        concord_classes: lowered.concord_classes,
        numbers: lowered.numbers,
        onsets: lowered.onsets,
        possessive_endings: lowered.possessive_endings,
    })
}

struct PrefixedSequenceParts {
    values: Vec<TokenStream>,
    tail: syn::Ident,
    concord_classes: Vec<syn::Ident>,
    tail_concord_class: Option<syn::Ident>,
    numbers: Vec<syn::Ident>,
    tail_number: Option<syn::Ident>,
    onsets: Vec<syn::Ident>,
    tail_onset: Option<syn::Ident>,
    possessive_endings: Vec<syn::Ident>,
    tail_possessive_ending: Option<syn::Ident>,
}

fn prefixed_sequence_parts(
    lowered: LoweredSequenceRhs,
    expected: usize,
    state: &str,
) -> syn::Result<PrefixedSequenceParts> {
    let Some(tail) = lowered.tail else {
        return Err(internal(&format!(
            "{state} sequence RHS must contain a tail"
        )));
    };
    if lowered.values.len() != expected {
        return Err(internal(&format!(
            "{state} sequence RHS must contain exactly {expected} prefix values"
        )));
    }
    if (!lowered.concord_classes.is_empty() && lowered.concord_classes.len() != expected)
        || (!lowered.numbers.is_empty() && lowered.numbers.len() != expected)
        || (!lowered.onsets.is_empty() && lowered.onsets.len() != expected)
        || (!lowered.possessive_endings.is_empty() && lowered.possessive_endings.len() != expected)
    {
        return Err(internal(&format!(
            "{state} sequence RHS has inconsistent concord_class bindings"
        )));
    }
    Ok(PrefixedSequenceParts {
        values: lowered.values,
        tail,
        concord_classes: lowered.concord_classes,
        tail_concord_class: lowered.tail_concord_class,
        numbers: lowered.numbers,
        tail_number: lowered.tail_number,
        onsets: lowered.onsets,
        tail_onset: lowered.tail_onset,
        possessive_endings: lowered.possessive_endings,
        tail_possessive_ending: lowered.tail_possessive_ending,
    })
}

fn homogeneous_sequence_feature(
    values: Vec<syn::Ident>,
    tail_value: Option<syn::Ident>,
) -> (Option<syn::Ident>, Vec<TokenStream>) {
    let mut all = values;
    all.extend(tail_value);
    let Some(first) = all.first().cloned() else {
        return (None, Vec::new());
    };
    let guards = all
        .iter()
        .skip(1)
        .map(|value| quote! { #first == #value })
        .collect();
    (Some(first), guards)
}

type SequenceOwnerFeatures = Vec<(Feature, syn::Ident)>;

struct SequenceFeatureBindings {
    concord_classes: Vec<syn::Ident>,
    tail_concord_class: Option<syn::Ident>,
    numbers: Vec<syn::Ident>,
    tail_number: Option<syn::Ident>,
    onsets: Vec<syn::Ident>,
    tail_onset: Option<syn::Ident>,
    possessive_endings: Vec<syn::Ident>,
    tail_possessive_ending: Option<syn::Ident>,
}

fn sequence_owner_feature_values(
    features: &[Feature],
    bindings: &SequenceFeatureBindings,
) -> syn::Result<(SequenceOwnerFeatures, Vec<TokenStream>)> {
    let mut values = Vec::new();
    let mut guards = Vec::new();
    for feature in features {
        let (value, feature_guards) = match feature {
            Feature::ConcordClass => {
                let (value, guards) = homogeneous_sequence_feature(
                    bindings.concord_classes.clone(),
                    bindings.tail_concord_class.clone(),
                );
                (value, guards)
            }
            Feature::Number => {
                let (value, guards) = homogeneous_sequence_feature(
                    bindings.numbers.clone(),
                    bindings.tail_number.clone(),
                );
                (value, guards)
            }
            Feature::Onset => (
                bindings
                    .onsets
                    .first()
                    .cloned()
                    .or(bindings.tail_onset.clone()),
                Vec::new(),
            ),
            Feature::PossessiveEnding => (
                bindings
                    .tail_possessive_ending
                    .clone()
                    .or_else(|| bindings.possessive_endings.last().cloned()),
                Vec::new(),
            ),
            _ => return Err(internal("unsupported sequence owner feature")),
        };
        if let Some(value) = value {
            values.push((*feature, value));
        }
        guards.extend(feature_guards);
    }
    Ok((values, guards))
}

fn emit_exact_sequence_success(
    carrier: &syn::Ident,
    features: &[Feature],
    parts: ExactSequenceParts,
) -> TokenStream {
    let values = parts.values;
    let (sequence_features, guards) = sequence_owner_feature_values(
        features,
        &SequenceFeatureBindings {
            concord_classes: parts.concord_classes,
            tail_concord_class: None,
            numbers: parts.numbers,
            tail_number: None,
            onsets: parts.onsets,
            tail_onset: None,
            possessive_endings: parts.possessive_endings,
            tail_possessive_ending: None,
        },
    )
    .expect("validated sequence features are supported");
    if sequence_features.is_empty() {
        quote! { Ok(Some(BuildValue::#carrier(vec![#(#values),*]))) }
    } else {
        let sequence_features = sequence_features.iter().map(|(_, value)| value);
        let predicate = if guards.is_empty() {
            quote! { true }
        } else {
            quote! { #(#guards)&&* }
        };
        quote! {
            if #predicate {
                Ok(Some(BuildValue::#carrier(vec![#(#values),*] #(, *#sequence_features)*)))
            } else {
                Ok(None)
            }
        }
    }
}

fn emit_prefixed_sequence_success(
    carrier: &syn::Ident,
    features: &[Feature],
    parts: PrefixedSequenceParts,
) -> TokenStream {
    let values = parts.values;
    let tail = parts.tail;
    let prefix_len = values.len();
    let (sequence_features, guards) = sequence_owner_feature_values(
        features,
        &SequenceFeatureBindings {
            concord_classes: parts.concord_classes,
            tail_concord_class: parts.tail_concord_class,
            numbers: parts.numbers,
            tail_number: parts.tail_number,
            onsets: parts.onsets,
            tail_onset: parts.tail_onset,
            possessive_endings: parts.possessive_endings,
            tail_possessive_ending: parts.tail_possessive_ending,
        },
    )
    .expect("validated sequence features are supported");
    if sequence_features.is_empty() {
        quote! {{
            let mut values = Vec::with_capacity(#prefix_len + #tail.len());
            #(values.push(#values);)*
            values.extend(#tail.iter().cloned());
            Ok(Some(BuildValue::#carrier(values)))
        }}
    } else {
        let sequence_features = sequence_features.iter().map(|(_, value)| value);
        let predicate = if guards.is_empty() {
            quote! { true }
        } else {
            quote! { #(#guards)&&* }
        };
        quote! {
            if #predicate {
                let mut values = Vec::with_capacity(#prefix_len + #tail.len());
                #(values.push(#values);)*
                values.extend(#tail.iter().cloned());
                Ok(Some(BuildValue::#carrier(values #(, *#sequence_features)*)))
            } else {
                Ok(None)
            }
        }
    }
}

fn fixed_surface_pattern(
    plan: &SemanticPlan,
    surface: &crate::semantic::FixedSurfaceAtomPlan,
) -> syn::Result<TokenStream> {
    match surface {
        crate::semantic::FixedSurfaceAtomPlan::Literal(value) => {
            let value = syn::LitStr::new(value, Span::call_site());
            Ok(quote! { BuildValue::Leaf(Leaf::Literal(#value)) })
        }
        crate::semantic::FixedSurfaceAtomPlan::Lex { terminal, variant } => {
            fixed_lex_pattern(plan, terminal, variant)
        }
    }
}

fn fixed_lex_pattern(
    plan: &SemanticPlan,
    terminal: &str,
    variant: &str,
) -> syn::Result<TokenStream> {
    match plan.atom_terminal(terminal)? {
        AtomTerminal::Vocab(vocab) => {
            let leaf = ident(vocab.name());
            let value = ident(variant);
            Ok(quote! { BuildValue::Leaf(Leaf::#leaf(#leaf::#value)) })
        }
        AtomTerminal::Lexeme(_)
        | AtomTerminal::Binding(_)
        | AtomTerminal::ContextIdentity(_)
        | AtomTerminal::CatalogIdentity { .. }
        | AtomTerminal::SignedDecimal(_)
        | AtomTerminal::UnsignedNumber(_)
        | AtomTerminal::DeclarationNoun { .. }
        | AtomTerminal::DeclarationDeterminative { .. }
        | AtomTerminal::DeclarationTerm { .. }
        | AtomTerminal::DeclarationVerb { .. } => Err(internal(
            "fixed structural lexeme lowering currently requires a vocabulary terminal",
        )),
    }
}

fn structural_owner_field(
    plan: &SemanticPlan,
    owner: super::rules::StructuralOwner,
    field_index: usize,
) -> (&str, &StructuralFieldPlan) {
    match owner {
        super::rules::StructuralOwner::Construction(index) => {
            let construction = &plan.constructions()[index];
            (
                construction.element_type(),
                construction.fields()[field_index]
                    .structural_plan()
                    .expect("helper owner is a structural construction field"),
            )
        }
        super::rules::StructuralOwner::Product(index) => {
            let product = &plan.products()[index];
            (product.name(), &product.fields()[field_index])
        }
    }
}

fn carrier_variant(owner: &str, field: &StructuralFieldPlan) -> syn::Result<String> {
    match field.kind() {
        StructuralFieldKindPlan::Zeroable(_) => {
            Err(internal("zeroable field has no helper carrier"))
        }
        StructuralFieldKindPlan::Optional(_) => Ok(format!(
            "{}{}Optional",
            crate::identifier::pascal_case(owner),
            crate::identifier::pascal_case(field.name()),
        )),
        StructuralFieldKindPlan::Sequence { .. } => field
            .helper_names()
            .map(|names| names.all()[0].to_owned())
            .ok_or_else(|| internal("sequence field has no helper carrier inventory")),
        StructuralFieldKindPlan::Required(_) => {
            Err(internal("required field has no helper carrier"))
        }
    }
}

fn atom_role(atom: &AtomPlan) -> Option<&str> {
    match atom.value_atom() {
        AtomPlan::Category { role, .. }
        | AtomPlan::Lex { role, .. }
        | AtomPlan::Marked { role, .. }
        | AtomPlan::Identity { role, .. }
        | AtomPlan::Noun { role, .. } => Some(role),
        AtomPlan::Literal(_)
        | AtomPlan::SentenceInitialLiteral(_)
        | AtomPlan::StructuralLiteral(_)
        | AtomPlan::LexFixed { .. }
        | AtomPlan::VerbFixed { .. }
        | AtomPlan::OpenDeclaration(_)
        | AtomPlan::Bound { .. }
        | AtomPlan::Circumfix { .. } => None,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "the exhaustive structural and lexical atom lowering matrix is kept together"
)]
fn lower_atom(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    form: &crate::semantic::FormPlan,
    atom: &AtomPlan,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    if let Some(role) = atom_role(atom)
        && let Some(structural) = row
            .fields()
            .iter()
            .find(|field| field.name_key() == role)
            .and_then(crate::semantic::ConstructionFieldPlan::structural_plan)
    {
        match structural.kind() {
            StructuralFieldKindPlan::Zeroable(_) => {
                return Err(internal(
                    "zeroable field reached the unexpanded construction build arm",
                ));
            }
            StructuralFieldKindPlan::Optional(_) => {
                let variant = ident(&carrier_variant(row.element_type(), structural)?);
                let binding = lowering.binders.allocate(role);
                if let AtomPlan::Circumfix { prefix, .. } = atom {
                    push_fixed_form_literal(lowering, prefix);
                }
                lowering
                    .patterns
                    .push(quote! { BuildValue::#variant(#binding) });
                lowering
                    .field_values
                    .insert(role.to_owned(), quote! { #binding.clone() });
                if let AtomPlan::Circumfix { suffix, .. } = atom {
                    push_fixed_form_literal(lowering, suffix);
                }
                return Ok(());
            }
            StructuralFieldKindPlan::Sequence { surface, .. } => {
                if matches!(
                    surface.separator(),
                    Some(crate::semantic::SeparatorPlan::Positional(_))
                ) {
                    return Err(internal(
                        "positional sequence reached the unexpanded construction build arm",
                    ));
                }
                let variant = ident(&carrier_variant(row.element_type(), structural)?);
                let binding = lowering.binders.allocate(role);
                let sequence_features = validated
                    .sequence_features(row.element_type(), structural.name())
                    .iter()
                    .map(|feature| {
                        let feature_binding = lowering
                            .binders
                            .allocate(&format!("{role}_{}", feature.key()));
                        lowering.role_features.insert(
                            (role.to_owned(), *feature),
                            LocalFeatureValue::Bound(feature_binding.clone()),
                        );
                        feature_binding
                    })
                    .collect::<Vec<_>>();
                if let AtomPlan::Circumfix { prefix, .. } = atom {
                    push_fixed_form_literal(lowering, prefix);
                }
                lowering
                    .patterns
                    .push(quote! { BuildValue::#variant(#binding #(, #sequence_features)*) });
                lowering
                    .field_values
                    .insert(role.to_owned(), quote! { #binding.clone() });
                if let AtomPlan::Circumfix { suffix, .. } = atom {
                    push_fixed_form_literal(lowering, suffix);
                }
                return Ok(());
            }
            StructuralFieldKindPlan::Required(_) => {}
        }
    }
    match atom {
        AtomPlan::Bound {
            direction,
            affix,
            value,
        } => lower_bound_atom(validated, row, form, *direction, affix, value, lowering)?,
        AtomPlan::Circumfix {
            prefix,
            value,
            suffix,
        } => {
            push_fixed_form_literal(lowering, prefix);
            lower_atom(validated, row, form, value, lowering)?;
            push_fixed_form_literal(lowering, suffix);
        }
        AtomPlan::Literal(literal)
        | AtomPlan::SentenceInitialLiteral(literal)
        | AtomPlan::StructuralLiteral(literal) => {
            let literal = syn::LitStr::new(literal, Span::call_site());
            lowering
                .patterns
                .push(quote! { BuildValue::Leaf(Leaf::Literal(#literal)) });
        }
        AtomPlan::Category { role, category } => {
            lower_category_role(validated, row, form, role, category, lowering)?;
        }
        AtomPlan::Marked {
            role,
            category,
            terminal,
            variant,
            ..
        } => {
            lowering
                .patterns
                .push(fixed_lex_pattern(validated, terminal, variant)?);
            lower_category_role(validated, row, form, role, category, lowering)?;
        }
        AtomPlan::Lex { role, terminal } | AtomPlan::Identity { role, terminal } => {
            lower_terminal_role(validated, row, form, role, terminal, false, lowering)?;
        }
        AtomPlan::LexFixed {
            terminal, variant, ..
        } => lowering
            .patterns
            .push(fixed_lex_pattern(validated, terminal, variant)?),
        AtomPlan::Noun { role, terminal } => {
            lower_terminal_role(validated, row, form, role, terminal, true, lowering)?;
        }
        AtomPlan::VerbFixed {
            terminal, variant, ..
        } => {
            let concord_class = verb_concord_class_pattern(validated, row, lowering)?;
            let onset =
                verb_onset_pattern(validated, row, lowering, terminal, variant, &concord_class)?;
            let concord_class_field = if concord_class.to_string() == "concord_class" {
                quote! { concord_class }
            } else {
                quote! { concord_class: #concord_class }
            };
            let terminal = ident(terminal);
            let variant = ident(variant);
            lowering.patterns.push(quote! {
                BuildValue::Leaf(Leaf::Verb { lexeme: #terminal::#variant, #concord_class_field, onset: #onset })
            });
        }
        AtomPlan::OpenDeclaration(open) => {
            let declaration = lowering.binders.allocate("declaration");
            let surface_feature = lowering.binders.allocate("surface_feature");
            let onset = lowering.binders.allocate("verb_onset");
            lowering.patterns.push(quote! {
                BuildValue::Leaf(Leaf::Declaration(DeclarationLeaf {
                    id: #declaration,
                    feature: #surface_feature,
                    onset: #onset,
                }))
            });
            let kind = crate::emit::declaration_kind(open.kind());
            let name = syn::LitStr::new(open.name(), Span::call_site());
            lowering.guards.push(quote! {
                #declaration.kind() == #kind && #declaration.name() == #name
            });
            lowering.guards.push(quote! {
                matches!(
                    *#surface_feature,
                    ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN
                        | ::deckmaste_construction_core::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT
                )
            });
            lowering.role_features.insert(
                ("verb".to_owned(), Feature::ConcordClass),
                LocalFeatureValue::Computed(quote! {
                    match #surface_feature {
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN => ConcordClass::Other,
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => {
                            ConcordClass::ThirdPersonSingular
                        }
                        _ => unreachable!("open verb matcher admitted a non-verb feature"),
                    }
                }),
            );
            lowering.role_features.insert(
                ("verb".to_owned(), Feature::Onset),
                LocalFeatureValue::Bound(onset),
            );
        }
    }
    Ok(())
}

fn push_fixed_form_literal(lowering: &mut Lowering, surface: &str) {
    let literal = syn::LitStr::new(surface, Span::call_site());
    lowering
        .patterns
        .push(quote! { BuildValue::Leaf(Leaf::Literal(#literal)) });
}

fn lower_bound_atom(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    form: &crate::semantic::FormPlan,
    direction: crate::semantic::BoundDirectionPlan,
    affix_surface: &str,
    value: &AtomPlan,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let affix = syn::LitStr::new(affix_surface, Span::call_site());
    let push_affix = |lowering: &mut Lowering| {
        lowering
            .patterns
            .push(quote! { BuildValue::Leaf(Leaf::Literal(#affix)) });
    };
    match direction {
        crate::semantic::BoundDirectionPlan::Prefix => {
            push_affix(lowering);
            lower_atom(validated, row, form, value, lowering)?;
            if let Some(role) = atom_role(value)
                && let Some(onset) = crate::macro_def::normalize_surface_onset(affix_surface, None)
            {
                let onset = match onset {
                    crate::macro_def::Onset::Consonant => FeatureValue::Consonant,
                    crate::macro_def::Onset::Vowel => FeatureValue::Vowel,
                };
                lowering.role_features.insert(
                    (role.to_owned(), Feature::Onset),
                    LocalFeatureValue::Known(onset),
                );
            }
        }
        crate::semantic::BoundDirectionPlan::Suffix => {
            lower_atom(validated, row, form, value, lowering)?;
            push_affix(lowering);
        }
    }
    Ok(())
}

fn lower_category_role(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    form: &crate::semantic::FormPlan,
    role: &str,
    category_name: &str,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let role = ident(role);
    let role_name = identifier_key(&role);
    let role_binding = lowering.binders.allocate_ident(&role);
    let category = ident(category_name);
    let following_onset = carries_following_onset(validated, category_name).then(|| {
        let following_onset = lowering
            .binders
            .allocate(&format!("{}_following_onset", identifier_key(&role)));
        lowering
            .role_following_onsets
            .insert(role_name.clone(), following_onset.clone());
        quote! { , #following_onset }
    });
    lowering
        .field_values
        .insert(role_name.clone(), quote! { #role_binding.clone() });

    let carries_concord_class = validated.category_carries_concord_class(category_name);
    let carries_cardinality = validated.category_carries_cardinality(category_name);
    let carries_number = validated.category_carries_number(category_name);
    let carries_determiner_number =
        validated.carries_feature(category_name, Feature::DeterminerNumber);
    let carries_fused_head_license =
        validated.carries_feature(category_name, Feature::FusedHeadLicense);
    let carries_nominal_license = validated.carries_feature(category_name, Feature::NominalLicense);
    let carries_onset = validated.category_carries_onset(category_name);
    let carries_possessive_ending = validated.category_carries_possessive_ending(category_name);
    let concord_class = carries_concord_class
        .then(|| role_concord_class_pattern(validated, row, &role, category_name, lowering))
        .transpose()?;
    let cardinality = carries_cardinality.then(|| {
        let name = lowering
            .binders
            .allocate(&format!("{}_cardinality", identifier_key(&role)));
        lowering.role_features.insert(
            (identifier_key(&role), Feature::Cardinality),
            LocalFeatureValue::Bound(name.clone()),
        );
        name
    });
    let number = carries_number.then(|| role_number_pattern(validated, row, form, &role, lowering));
    let determiner_number = carries_determiner_number.then(|| {
        let name = lowering
            .binders
            .allocate(&format!("{}_determiner_number", identifier_key(&role)));
        lowering.role_features.insert(
            (identifier_key(&role), Feature::DeterminerNumber),
            LocalFeatureValue::Bound(name.clone()),
        );
        name
    });
    let fused_head_license = carries_fused_head_license.then(|| {
        let name = lowering
            .binders
            .allocate(&format!("{}_fused_head_license", identifier_key(&role)));
        lowering.role_features.insert(
            (identifier_key(&role), Feature::FusedHeadLicense),
            LocalFeatureValue::Bound(name.clone()),
        );
        name
    });
    let nominal_license = carries_nominal_license.then(|| {
        let name = lowering
            .binders
            .allocate(&format!("{}_nominal_license", identifier_key(&role)));
        lowering.role_features.insert(
            (identifier_key(&role), Feature::NominalLicense),
            LocalFeatureValue::Bound(name.clone()),
        );
        name
    });
    let onset = carries_onset.then(|| {
        let name = lowering
            .binders
            .allocate(&format!("{}_onset", identifier_key(&role)));
        lowering.role_features.insert(
            (identifier_key(&role), Feature::Onset),
            LocalFeatureValue::Bound(name.clone()),
        );
        name
    });
    let possessive_ending = carries_possessive_ending.then(|| {
        let name = lowering
            .binders
            .allocate(&format!("{}_possessive_ending", identifier_key(&role)));
        lowering.role_features.insert(
            (identifier_key(&role), Feature::PossessiveEnding),
            LocalFeatureValue::Bound(name.clone()),
        );
        name
    });
    let concord_class = concord_class.map(|value| quote! { , #value });
    let cardinality = cardinality.map(|value| quote! { , #value });
    let number = number.map(|value| quote! { , #value });
    let determiner_number = determiner_number.map(|value| quote! { , #value });
    let fused_head_license = fused_head_license.map(|value| quote! { , #value });
    let nominal_license = nominal_license.map(|value| quote! { , #value });
    let onset = onset.map(|value| quote! { , #value });
    let possessive_ending = possessive_ending.map(|value| quote! { , #value });
    lowering
        .patterns
        .push(quote! { BuildValue::#category(#role_binding #concord_class #cardinality #number #determiner_number #fused_head_license #nominal_license #onset #possessive_ending #following_onset) });
    Ok(())
}

fn carries_following_onset(plan: &SemanticPlan, name: &str) -> bool {
    super::semantic_types(plan)
        .into_iter()
        .find(|item| item.name == name)
        .is_some_and(|item| item.kind != super::SemanticTypeKind::Product)
}

fn role_concord_class_pattern(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    role: &syn::Ident,
    category: &str,
    lowering: &mut Lowering,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: role.clone(),
        feature: Feature::ConcordClass,
    };
    if let Some(crate::feature::FeatureResolution::Known(value)) =
        validated.feature_resolution(row.construction_id(), &target)
    {
        lowering.role_features.insert(
            (identifier_key(role), Feature::ConcordClass),
            LocalFeatureValue::Known(value),
        );
        return Ok(feature_value(value));
    }
    if let Some(equation) = equation(validated, row, &target) {
        if matches!(equation.value(), FeatureExpr::MatchVocab { .. }) {
            return Err(internal("category role feature cannot be a vocab match"));
        }
        let stem = snake_case(category).trim_end_matches("_phrase").to_owned();
        let name = lowering.binders.allocate(&format!("{stem}_concord_class"));
        lowering.role_features.insert(
            (identifier_key(role), Feature::ConcordClass),
            LocalFeatureValue::Bound(name.clone()),
        );
        return Ok(quote! { #name });
    }
    if feature_is_read(validated, row, role, Feature::ConcordClass) {
        let name = lowering
            .binders
            .allocate(&format!("{}_concord_class", identifier_key(role)));
        lowering.role_features.insert(
            (identifier_key(role), Feature::ConcordClass),
            LocalFeatureValue::Bound(name.clone()),
        );
        Ok(quote! { #name })
    } else {
        Ok(quote! { _ })
    }
}

fn role_number_pattern(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    form: &crate::semantic::FormPlan,
    role: &syn::Ident,
    lowering: &mut Lowering,
) -> TokenStream {
    let target = FeaturePlace::Role {
        field: role.clone(),
        feature: Feature::Number,
    };
    if let Some(crate::feature::FeatureResolution::Known(value)) =
        validated.feature_resolution(row.construction_id(), &target)
    {
        lowering.role_features.insert(
            (identifier_key(role), Feature::Number),
            LocalFeatureValue::Known(value),
        );
        return feature_value(value);
    }
    if role_number_is_needed_in_build(validated, row, form, role) {
        let name = lowering
            .binders
            .allocate(&format!("{}_number", identifier_key(role)));
        lowering.role_features.insert(
            (identifier_key(role), Feature::Number),
            LocalFeatureValue::Bound(name.clone()),
        );
        quote! { #name }
    } else {
        quote! { _ }
    }
}

fn lower_catalog_identity_role(
    plan: &crate::semantic::CatalogIdentityPlan,
    role: &syn::Ident,
    lowering: &mut Lowering,
) {
    let ty = plan.ident();
    let provider = plan.provider();
    let identity = lowering.binders.allocate(&identifier_key(role));
    let onset = lowering
        .binders
        .allocate(&format!("{}_onset", identifier_key(role)));
    let possessive_ending = lowering
        .binders
        .allocate(&format!("{}_possessive_ending", identifier_key(role)));
    lowering.patterns.push(quote! {
        BuildValue::Leaf(Leaf::CatalogIdentity {
            provider: CatalogProvider::#provider,
            canonical_identity: #identity,
            onset: #onset,
            possessive_ending: #possessive_ending,
        })
    });
    lowering.field_values.insert(
        identifier_key(role),
        quote! { #ty::from_canonical(#identity.clone()) },
    );
    lowering.role_features.insert(
        (identifier_key(role), Feature::Onset),
        LocalFeatureValue::Bound(onset),
    );
    lowering.role_features.insert(
        (identifier_key(role), Feature::PossessiveEnding),
        LocalFeatureValue::Bound(possessive_ending),
    );
}

fn lower_unsigned_number_role(
    codec: &crate::semantic::UnsignedNumberPlan,
    role: &syn::Ident,
    lowering: &mut Lowering,
) {
    let concord_class_provider = (codec.kind() == UnsignedNumberKind::EnglishCardinal)
        .then(|| ident(&feature_helper("concord_class", codec.codec_name())));
    let determiner_number_provider = (codec.kind() == UnsignedNumberKind::EnglishCardinal)
        .then(|| ident(&feature_helper("determiner_number", codec.codec_name())));
    let number_provider = (codec.kind() == UnsignedNumberKind::EnglishCardinal)
        .then(|| ident(&feature_helper("number", codec.codec_name())));
    let cardinality_provider = (codec.kind() == UnsignedNumberKind::EnglishCardinal)
        .then(|| ident(&feature_helper("cardinality", codec.codec_name())));
    for provider in [
        &concord_class_provider,
        &determiner_number_provider,
        &number_provider,
        &cardinality_provider,
    ]
    .into_iter()
    .flatten()
    {
        lowering.binders.reserve(provider.to_string());
    }
    let value = lowering.binders.allocate(&identifier_key(role));
    let variant = codec.codec_ident();
    lowering
        .patterns
        .push(quote! { BuildValue::Leaf(Leaf::#variant(#value)) });
    lowering
        .field_values
        .insert(identifier_key(role), quote! { #value.clone() });
    for (feature, provider) in [
        (Feature::ConcordClass, concord_class_provider),
        (Feature::DeterminerNumber, determiner_number_provider),
        (Feature::Number, number_provider),
        (Feature::Cardinality, cardinality_provider),
    ] {
        if let Some(provider) = provider {
            lowering.role_features.insert(
                (identifier_key(role), feature),
                LocalFeatureValue::Computed(quote! { #provider(#value) }),
            );
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "terminal lowering is deliberately exhaustive over the sealed terminal inventory"
)]
fn lower_terminal_role(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    form: &crate::semantic::FormPlan,
    role: &str,
    terminal_name: &str,
    noun: bool,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let role = ident(role);
    let binding = match validated.atom_terminal(terminal_name)? {
        AtomTerminal::Vocab(vocab) => {
            let leaf = ident(vocab.name());
            let binding = lowering.binders.allocate(&vocab_argument(vocab.name()));
            let onset_arms = vocab.variants().iter().map(|variant| {
                let member = variant.name();
                let onset = super::onset(variant.onset());
                quote! { #leaf::#member => #onset }
            });
            let possessive_ending_arms = vocab.variants().iter().map(|variant| {
                let member = variant.name();
                let ending = possessive_ending_for_surface(&variant.word().value());
                quote! { #leaf::#member => #ending }
            });
            lowering
                .patterns
                .push(quote! { BuildValue::Leaf(Leaf::#leaf(#binding)) });
            lowering
                .field_values
                .insert(identifier_key(&role), quote! { *#binding });
            lowering
                .vocab_values
                .insert(identifier_key(&role), binding.clone());
            lowering.role_features.insert(
                (identifier_key(&role), Feature::Onset),
                LocalFeatureValue::Computed(quote! {
                    match #binding { #(#onset_arms,)* }
                }),
            );
            lowering.role_features.insert(
                (identifier_key(&role), Feature::PossessiveEnding),
                LocalFeatureValue::Computed(quote! {
                    match #binding { #(#possessive_ending_arms,)* }
                }),
            );
            return Ok(());
        }
        AtomTerminal::Lexeme(_) => {
            let value = lowering.binders.allocate(&identifier_key(&role));
            let number = noun_number_pattern(validated, row, &role, lowering)?;
            let onset = lowering
                .binders
                .allocate(&format!("{}_onset", identifier_key(&role)));
            let possessive_ending = lowering
                .binders
                .allocate(&format!("{}_possessive_ending", identifier_key(&role)));
            lowering.role_features.insert(
                (identifier_key(&role), Feature::Onset),
                LocalFeatureValue::Bound(onset.clone()),
            );
            lowering.role_features.insert(
                (identifier_key(&role), Feature::PossessiveEnding),
                LocalFeatureValue::Bound(possessive_ending.clone()),
            );
            let number_field = if number.to_string() == "number" {
                quote! { number }
            } else {
                quote! { number: #number }
            };
            lowering.patterns.push(quote! {
                BuildValue::Leaf(Leaf::Noun {
                    noun: #value,
                    #number_field,
                    onset: #onset,
                    possessive_ending: #possessive_ending,
                })
            });
            lowering
                .field_values
                .insert(identifier_key(&role), quote! { *#value });
            return Ok(());
        }
        AtomTerminal::Binding(binding) => binding,
        AtomTerminal::ContextIdentity(identity) => {
            lower_context_identity_role(identity, &role, lowering);
            return Ok(());
        }
        AtomTerminal::CatalogIdentity { plan, .. } => {
            lower_catalog_identity_role(plan, &role, lowering);
            return Ok(());
        }
        AtomTerminal::SignedDecimal(codec) => {
            let variant = codec.codec_ident();
            let value = lowering.binders.allocate(&identifier_key(&role));
            lowering
                .patterns
                .push(quote! { BuildValue::Leaf(Leaf::#variant(#value)) });
            lowering
                .field_values
                .insert(identifier_key(&role), quote! { #value.clone() });
            return Ok(());
        }
        AtomTerminal::UnsignedNumber(codec) => {
            lower_unsigned_number_role(codec, &role, lowering);
            return Ok(());
        }
        AtomTerminal::DeclarationNoun { plan, .. } => {
            let leaf = plan.codec_ident();
            let value = lowering.binders.allocate(&identifier_key(&role));
            let number = noun_number_pattern(validated, row, &role, lowering)?;
            let onset = lowering
                .binders
                .allocate(&format!("{}_onset", identifier_key(&role)));
            let possessive_ending = lowering
                .binders
                .allocate(&format!("{}_possessive_ending", identifier_key(&role)));
            lowering.role_features.insert(
                (identifier_key(&role), Feature::Onset),
                LocalFeatureValue::Bound(onset.clone()),
            );
            lowering.role_features.insert(
                (identifier_key(&role), Feature::PossessiveEnding),
                LocalFeatureValue::Bound(possessive_ending.clone()),
            );
            let number_field = if number.to_string() == "number" {
                quote! { number }
            } else {
                quote! { number: #number }
            };
            lowering.patterns.push(quote! {
                BuildValue::Leaf(Leaf::#leaf {
                    noun: #value,
                    #number_field,
                    onset: #onset,
                    possessive_ending: #possessive_ending,
                })
            });
            lowering
                .field_values
                .insert(identifier_key(&role), quote! { #value.clone() });
            return Ok(());
        }
        AtomTerminal::DeclarationDeterminative { plan, .. } => {
            let leaf = plan.codec_ident();
            let value = lowering.binders.allocate(&identifier_key(&role));
            let onset = lowering
                .binders
                .allocate(&format!("{}_onset", identifier_key(&role)));
            let following_onset = lowering
                .binders
                .allocate(&format!("{}_following_onset", identifier_key(&role)));
            let number_license = lowering
                .binders
                .allocate(&format!("{}_number_license", identifier_key(&role)));
            let fused_head_license = lowering
                .binders
                .allocate(&format!("{}_fused_head_license", identifier_key(&role)));
            let nominal_license = lowering
                .binders
                .allocate(&format!("{}_nominal_license", identifier_key(&role)));
            lowering.role_features.insert(
                (identifier_key(&role), Feature::Onset),
                LocalFeatureValue::Bound(onset.clone()),
            );
            lowering
                .role_following_onsets
                .insert(identifier_key(&role), following_onset.clone());
            lowering.role_features.insert(
                (identifier_key(&role), Feature::DeterminerNumber),
                LocalFeatureValue::Bound(number_license.clone()),
            );
            lowering.role_features.insert(
                (identifier_key(&role), Feature::FusedHeadLicense),
                LocalFeatureValue::Bound(fused_head_license.clone()),
            );
            lowering.role_features.insert(
                (identifier_key(&role), Feature::NominalLicense),
                LocalFeatureValue::Bound(nominal_license.clone()),
            );
            lowering.patterns.push(quote! { BuildValue::Leaf(Leaf::#leaf { value: #value, onset: #onset, following_onset: #following_onset, number_license: #number_license, fused_head_license: #fused_head_license, nominal_license: #nominal_license }) });
            lowering
                .field_values
                .insert(identifier_key(&role), quote! { #value.clone() });
            return Ok(());
        }
        AtomTerminal::DeclarationTerm {
            terminal_index,
            plan,
        } => {
            let term = plan.codec_ident();
            let value = lowering.binders.allocate(&identifier_key(&role));
            let onset = lowering
                .binders
                .allocate(&format!("{}_onset", identifier_key(&role)));
            let possessive_ending = lowering
                .binders
                .allocate(&format!("{}_possessive_ending", identifier_key(&role)));
            lowering.role_features.insert(
                (identifier_key(&role), Feature::Onset),
                LocalFeatureValue::Bound(onset.clone()),
            );
            lowering.role_features.insert(
                (identifier_key(&role), Feature::PossessiveEnding),
                LocalFeatureValue::Bound(possessive_ending.clone()),
            );
            lowering.patterns.push(quote! {
                BuildValue::Leaf(Leaf::DeclarationTerm {
                    terminal_index: #terminal_index,
                    id: #value,
                    onset: #onset,
                    possessive_ending: #possessive_ending,
                })
            });
            lowering.field_values.insert(
                identifier_key(&role),
                quote! {
                    #term::from_reading(#value.clone())
                        .expect("scanned declaration term preserves its codec kind")
                },
            );
            return Ok(());
        }
        AtomTerminal::DeclarationVerb { plan, .. } => {
            lower_declaration_verb_role(validated, row, &role, plan, lowering)?;
            return Ok(());
        }
    };
    lower_binding_terminal_role(binding, validated, row, form, &role, noun, lowering)
}

fn lower_declaration_verb_role(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    role: &syn::Ident,
    plan: &crate::semantic::DeclarationVerbPlan,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let leaf = plan.codec_ident();
    let value = lowering.binders.allocate(&identifier_key(role));
    let onset = lowering
        .binders
        .allocate(&format!("{}_onset", identifier_key(role)));
    lowering.role_features.insert(
        (identifier_key(role), Feature::Onset),
        LocalFeatureValue::Bound(onset.clone()),
    );
    let feature_field = match plan.feature_axis() {
        Feature::ConcordClass => {
            let concord_class =
                role_concord_class_pattern(validated, row, role, plan.codec_name(), lowering)?;
            if concord_class.to_string() == "concord_class" {
                quote! { concord_class, }
            } else {
                quote! { concord_class: #concord_class, }
            }
        }
        Feature::Participle => quote! {},
        _ => unreachable!("validated declaration verb feature axis is closed"),
    };
    lowering.patterns.push(quote! {
        BuildValue::Leaf(Leaf::#leaf {
            verb: #value,
            #feature_field
            onset: #onset,
        })
    });
    lowering
        .field_values
        .insert(identifier_key(role), quote! { #value.clone() });
    Ok(())
}

fn lower_binding_terminal_role(
    binding: &crate::semantic::BindingPlan,
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    form: &crate::semantic::FormPlan,
    role: &syn::Ident,
    noun: bool,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let build = binding
        .build()
        .ok_or_else(|| internal("atom-capable binding has no build metadata"))?;
    let variant = build.variant();
    let names = build.slots();
    let noun_count = form
        .atoms()
        .iter()
        .filter(|atom| matches!(atom, AtomPlan::Noun { .. }))
        .count();
    let preferred_names = if noun && noun_count > 1 {
        names
            .iter()
            .map(|name| {
                if names.len() == 1 {
                    identifier_key(role)
                } else {
                    format!("{}_{}", identifier_key(role), identifier_key(name))
                }
            })
            .collect::<Vec<_>>()
    } else if noun || names.len() != 1 {
        names.iter().map(identifier_key).collect()
    } else {
        vec![identifier_key(role)]
    };
    let pattern_names = preferred_names
        .iter()
        .map(|name| lowering.binders.allocate(name))
        .collect::<Vec<_>>();
    let substitutions = names
        .iter()
        .zip(&pattern_names)
        .map(|(declared, emitted)| (identifier_key(declared), quote! { #emitted }))
        .collect::<HashMap<_, _>>();
    let inner = if noun {
        let number = noun_number_pattern(validated, row, role, lowering)?;
        let number_field = if number.to_string() == "number" {
            quote! { number }
        } else {
            quote! { number: #number }
        };
        let [noun_value] = pattern_names.as_slice() else {
            return Err(internal(
                "noun terminal binding must expose exactly one build slot",
            ));
        };
        let onset = lowering
            .binders
            .allocate(&format!("{}_onset", identifier_key(role)));
        let possessive_ending = lowering
            .binders
            .allocate(&format!("{}_possessive_ending", identifier_key(role)));
        lowering.role_features.insert(
            (identifier_key(role), Feature::Onset),
            LocalFeatureValue::Bound(onset.clone()),
        );
        lowering.role_features.insert(
            (identifier_key(role), Feature::PossessiveEnding),
            LocalFeatureValue::Bound(possessive_ending.clone()),
        );
        quote! {
            Leaf::Noun {
                noun: #noun_value,
                #number_field,
                onset: #onset,
                possessive_ending: #possessive_ending,
            }
        }
    } else if build.construct_is_direct_slot() {
        quote! { Leaf::#variant(#(#pattern_names),*) }
    } else {
        quote! { Leaf::#variant(BoundLeaf::#variant(#(#pattern_names),*)) }
    };
    lowering.patterns.push(quote! { BuildValue::Leaf(#inner) });
    let construct = lower_build_recipe(build.recipe(), &substitutions)?;
    let stored = if build.construct_is_direct_slot() {
        match binding.kind() {
            TerminalBindingKind::Codec => quote! { #construct.clone() },
            TerminalBindingKind::Identity => quote! { *#construct },
        }
    } else {
        construct
    };
    lowering.field_values.insert(identifier_key(role), stored);
    Ok(())
}

fn context_identity_onset(
    identity: &crate::semantic::ContextIdentityPlan,
    value: &syn::Ident,
) -> TokenStream {
    let identity_type = identity.ident();
    let arms = identity.arms().iter().map(|arm| {
        let member = arm.variant();
        let accessor = ident(&format!("{}_onset", identifier_key(arm.accessor())));
        quote! { #identity_type::#member => context.#accessor() }
    });
    quote! { match #value { #(#arms,)* } }
}

fn context_identity_possessive_ending(
    identity: &crate::semantic::ContextIdentityPlan,
    value: &syn::Ident,
) -> TokenStream {
    let identity_type = identity.ident();
    let arms = identity.arms().iter().map(|arm| {
        let member = arm.variant();
        let accessor = arm.accessor();
        let ending = runtime_possessive_ending(&quote! { context.#accessor() });
        quote! { #identity_type::#member => #ending }
    });
    quote! { match #value { #(#arms,)* } }
}

fn possessive_ending_for_surface(surface: &str) -> TokenStream {
    if surface
        .as_bytes()
        .last()
        .is_some_and(|byte| matches!(byte, b's' | b'S'))
    {
        quote! { PossessiveEnding::EndsInS }
    } else {
        quote! { PossessiveEnding::Other }
    }
}

fn runtime_possessive_ending(surface: &TokenStream) -> TokenStream {
    quote! {
        if (#surface)
            .as_bytes()
            .last()
            .is_some_and(|byte| matches!(byte, b's' | b'S'))
        {
            PossessiveEnding::EndsInS
        } else {
            PossessiveEnding::Other
        }
    }
}

fn lower_context_identity_role(
    identity: &crate::semantic::ContextIdentityPlan,
    role: &syn::Ident,
    lowering: &mut Lowering,
) {
    let variant = identity.aggregate_ident();
    let value = lowering.binders.allocate(&identifier_key(role));
    lowering
        .patterns
        .push(quote! { BuildValue::Leaf(Leaf::#variant(#value)) });
    lowering
        .field_values
        .insert(identifier_key(role), quote! { *#value });
    lowering.role_features.insert(
        (identifier_key(role), Feature::Onset),
        LocalFeatureValue::Computed(context_identity_onset(identity, &value)),
    );
    lowering.role_features.insert(
        (identifier_key(role), Feature::PossessiveEnding),
        LocalFeatureValue::Computed(context_identity_possessive_ending(identity, &value)),
    );
}

fn lower_build_recipe(
    expression: &BindingBuildExprPlan,
    substitutions: &HashMap<String, TokenStream>,
) -> syn::Result<TokenStream> {
    match expression {
        BindingBuildExprPlan::Slot(name) => substitutions
            .get(name)
            .cloned()
            .ok_or_else(|| internal("validated build expression references an absent binder")),
        BindingBuildExprPlan::Field { base, member } => {
            let base = lower_build_recipe(base, substitutions)?;
            Ok(quote! { #base.#member })
        }
        BindingBuildExprPlan::Call {
            function,
            arguments,
        } => {
            let arguments = arguments
                .iter()
                .map(|argument| lower_build_recipe(argument, substitutions))
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! { #function(#(#arguments),*) })
        }
        BindingBuildExprPlan::Parenthesized(inner) => {
            let inner = lower_build_recipe(inner, substitutions)?;
            Ok(quote! { (#inner) })
        }
    }
}

fn noun_number_pattern(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    role: &syn::Ident,
    lowering: &mut Lowering,
) -> syn::Result<TokenStream> {
    let role_target = FeaturePlace::Role {
        field: role.clone(),
        feature: Feature::Number,
    };
    let target = if equation(validated, row, &role_target).is_some() {
        role_target.clone()
    } else {
        FeaturePlace::Construction(Feature::Number)
    };
    if let Some(crate::feature::FeatureResolution::Known(value)) =
        validated.feature_resolution(row.construction_id(), &target)
    {
        lowering.role_features.insert(
            (identifier_key(role), Feature::Number),
            LocalFeatureValue::Known(value),
        );
        return Ok(feature_value(value));
    }
    let equation = equation(validated, row, &target)
        .ok_or_else(|| internal("noun atom has no construction number"))?;
    match equation.value() {
        FeatureExpr::Constant(value) => {
            lowering.role_features.insert(
                (identifier_key(role), Feature::Number),
                LocalFeatureValue::Known(*value.value()),
            );
            Ok(feature_value(*value.value()))
        }
        FeatureExpr::MatchVocab { .. } | FeatureExpr::FromRole { .. } => {
            let preferred = if lowering.dynamic_numbers.is_empty() {
                "number".to_owned()
            } else {
                format!("{}_number", identifier_key(role))
            };
            let number = lowering.binders.allocate(&preferred);
            lowering.role_features.insert(
                (identifier_key(role), Feature::Number),
                LocalFeatureValue::Bound(number.clone()),
            );
            if target != role_target {
                lowering.dynamic_numbers.push(number.clone());
            }
            Ok(quote! { #number })
        }
    }
}

fn verb_concord_class_pattern(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &mut Lowering,
) -> syn::Result<TokenStream> {
    let verb = ident("verb");
    let target = FeaturePlace::Role {
        field: verb,
        feature: Feature::ConcordClass,
    };
    if let Some(crate::feature::FeatureResolution::Known(value)) =
        validated.feature_resolution(row.construction_id(), &target)
    {
        lowering.role_features.insert(
            ("verb".to_owned(), Feature::ConcordClass),
            LocalFeatureValue::Known(value),
        );
        return Ok(feature_value(value));
    }
    if matches!(
        equation(validated, row, &target).map(crate::feature::FeatureEquation::value),
        Some(FeatureExpr::MatchVocab { .. })
    ) {
        return Err(internal("verb concord_class cannot be a vocab match"));
    }
    let concord_class = lowering.binders.allocate("concord_class");
    lowering.role_features.insert(
        ("verb".to_owned(), Feature::ConcordClass),
        LocalFeatureValue::Bound(concord_class.clone()),
    );
    Ok(quote! { #concord_class })
}

fn verb_onset_pattern(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &mut Lowering,
    terminal: &str,
    variant: &str,
    concord_class_pattern: &TokenStream,
) -> syn::Result<TokenStream> {
    let lexeme = validated
        .runtime_verb_lexeme()
        .filter(|lexeme| lexeme.name() == terminal)
        .ok_or_else(|| internal("fixed verb has no sealed lexeme onset provider"))?;
    let rows = lexeme
        .surfaces()
        .iter()
        .filter(|surface| surface.member() == variant)
        .collect::<Vec<_>>();
    let target = FeaturePlace::Role {
        field: ident("verb"),
        feature: Feature::ConcordClass,
    };
    if let Some(crate::feature::FeatureResolution::Known(concord_class)) =
        validated.feature_resolution(row.construction_id(), &target)
    {
        let feature = match concord_class {
            FeatureValue::ConcordOther => crate::macro_def::SurfaceFeature::PLAIN,
            FeatureValue::ThirdPersonSingular => {
                crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT
            }
            FeatureValue::Singular
            | FeatureValue::Plural
            | FeatureValue::Consonant
            | FeatureValue::Vowel
            | FeatureValue::EndsInS
            | FeatureValue::Other
            | FeatureValue::Participle
            | FeatureValue::Zero
            | FeatureValue::One
            | FeatureValue::TwoPlus => {
                return Err(internal("fixed verb resolved a non-ConcordClass feature"));
            }
            _ => return Err(internal("fixed verb resolved a non-ConcordClass feature")),
        };
        let onset = rows
            .iter()
            .find(|surface| surface.feature() == feature)
            .map(|surface| match surface.onset() {
                crate::macro_def::Onset::Consonant => FeatureValue::Consonant,
                crate::macro_def::Onset::Vowel => FeatureValue::Vowel,
            })
            .ok_or_else(|| internal("fixed verb has no exact realized onset row"))?;
        lowering.role_features.insert(
            ("verb".to_owned(), Feature::Onset),
            LocalFeatureValue::Known(onset),
        );
        return Ok(feature_value(onset));
    }

    let onset = lowering.binders.allocate("verb_onset");
    lowering.role_features.insert(
        ("verb".to_owned(), Feature::Onset),
        LocalFeatureValue::Bound(onset.clone()),
    );
    let correlations = rows.iter().map(|surface| {
        let concord_class = match surface.feature() {
            crate::macro_def::SurfaceFeature::PLAIN => {
                quote! { ConcordClass::Other }
            }
            crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => {
                quote! { ConcordClass::ThirdPersonSingular }
            }
            crate::macro_def::SurfaceFeature::Inflectional(_)
            | crate::macro_def::SurfaceFeature::Singular
            | crate::macro_def::SurfaceFeature::Plural
            | crate::macro_def::SurfaceFeature::Fixed
            | crate::macro_def::SurfaceFeature::BlockLabel => {
                unreachable!("validated verb lexeme has ConcordClass rows")
            }
        };
        let expected_onset = match surface.onset() {
            crate::macro_def::Onset::Consonant => quote! { Onset::Consonant },
            crate::macro_def::Onset::Vowel => quote! { Onset::Vowel },
        };
        quote! { (*#concord_class_pattern == #concord_class && *#onset == #expected_onset) }
    });
    lowering.guards.push(quote! {
        #(#correlations)||*
    });
    Ok(quote! { #onset })
}

fn lower_feature_guards(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    form: &crate::semantic::FormPlan,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    for equation in validated.feature_equations(row.construction_id()) {
        if let (
            FeaturePlace::Construction(Feature::Number),
            FeatureExpr::FromRole {
                role,
                feature: Feature::Number,
            },
        ) = (equation.target(), equation.value())
        {
            if form
                .atoms()
                .iter()
                .any(|atom| matches!(atom, AtomPlan::Noun { .. }))
            {
                let source = resolve_feature_place(
                    validated,
                    row,
                    lowering,
                    &FeaturePlace::Role {
                        field: role.clone(),
                        feature: Feature::Number,
                    },
                    &mut HashSet::new(),
                )?;
                for number in &lowering.dynamic_numbers {
                    let source = resolved_feature_value_tokens(&source);
                    lowering.guards.push(quote! { #source == *#number });
                }
            }
            continue;
        }
        let FeaturePlace::Role { field, feature } = equation.target() else { continue };
        let FeatureExpr::FromRole {
            role,
            feature: source_feature,
        } = equation.value()
        else {
            continue;
        };
        if feature != source_feature {
            continue;
        }
        let Some(target_field) = row
            .fields()
            .iter()
            .find(|candidate| candidate.name_key() == identifier_key(field))
        else {
            // Form slots such as `verb` carry a feature equation but have no
            // stored construction field to guard during materialization.
            continue;
        };
        if target_field.is_zeroable() {
            let optional = lowering
                .field_values
                .get(&identifier_key(field))
                .ok_or_else(|| internal("zeroable feature target has no lowered value"))?;
            let source_field = row.field(&identifier_key(role))?;
            let source = if source_field.kind() == crate::semantic::ConstructionFieldKind::Category
            {
                let value = lowering
                    .field_values
                    .get(&identifier_key(role))
                    .ok_or_else(|| internal("zeroable feature source has no lowered value"))?;
                let helper = ident(&feature_helper(feature.key(), source_field.terminal()));
                quote! { #helper(&#value) }
            } else {
                let source = resolve_feature_place(
                    validated,
                    row,
                    lowering,
                    &FeaturePlace::Role {
                        field: role.clone(),
                        feature: *feature,
                    },
                    &mut HashSet::new(),
                )?;
                resolved_feature_value_tokens(&source)
            };
            let helper = ident(&feature_helper(feature.key(), target_field.terminal()));
            let absent = if *feature == Feature::Number {
                quote! { #source == Number::Plural }
            } else {
                quote! { true }
            };
            lowering.guards.push(quote! {
                match (#optional).as_ref() {
                    Some(value) => #helper(value) == #source,
                    None => #absent,
                }
            });
            continue;
        }
        let Some(target) = lowering
            .role_features
            .get(&(identifier_key(field), *feature))
            .map(local_feature_value)
        else {
            continue;
        };
        let source = resolve_feature_place(
            validated,
            row,
            lowering,
            &FeaturePlace::Role {
                field: role.clone(),
                feature: *feature,
            },
            &mut HashSet::new(),
        )?;
        if !same_known_feature(&source, &target) {
            lowering
                .guards
                .push(compare_feature_values(&source, &target));
        }
    }
    Ok(())
}

fn emit_success(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &mut Lowering,
    overrides: Option<&HashMap<String, TokenStream>>,
    concord_class_override: Option<FeatureValue>,
    number_override: Option<FeatureValue>,
) -> syn::Result<TokenStream> {
    let element = ident(row.element_type());
    let category = ident(row.category());
    let variant = ident(row.category_variant());
    if !row.fields().is_empty() && validated.construction_requires_checked_ast(row) {
        let mut arguments = row
            .fields()
            .iter()
            .map(|field| stored_value(validated, row, lowering, field.name(), overrides))
            .collect::<syn::Result<Vec<_>>>()?;
        if row.invariant().requires_context() {
            arguments.push(quote! { context });
        }
        let result = quote! { #element::try_new(#(#arguments),*) };
        return emit_fallible_element_success(
            validated,
            row,
            lowering,
            &result,
            concord_class_override,
            number_override,
        );
    }
    let element_value = if row.fields().is_empty() {
        quote! { #element }
    } else {
        let fields = row
            .fields()
            .iter()
            .map(|field| {
                let name = field.name();
                let value = stored_value(validated, row, lowering, name, overrides)?;
                Ok(quote! { #name: #value })
            })
            .collect::<syn::Result<Vec<_>>>()?;
        quote! { #element { #(#fields),* } }
    };
    if validated.explicit_sum_owns_construction_category(row.category()) {
        return Ok(quote! { Ok(Some(BuildValue::#element(#element_value))) });
    }
    let category_value = quote! { #category::#variant(#element_value) };
    let following_onset = &lowering.output_following_onset;
    let carries_concord_class = validated.category_carries_concord_class(row.category());
    let carries_cardinality = validated.category_carries_cardinality(row.category());
    let carries_number = validated.category_carries_number(row.category());
    let carries_determiner_number =
        validated.carries_feature(row.category(), Feature::DeterminerNumber);
    let carries_fused_head_license =
        validated.carries_feature(row.category(), Feature::FusedHeadLicense);
    let carries_nominal_license =
        validated.carries_feature(row.category(), Feature::NominalLicense);
    let carries_onset = validated.category_carries_onset(row.category());
    let carries_possessive_ending = validated.category_carries_possessive_ending(row.category());
    let concord_class = carries_concord_class
        .then(|| construction_concord_class(validated, row, lowering, concord_class_override))
        .transpose()?
        .map(|value| quote! { , #value });
    let cardinality = carries_cardinality
        .then(|| construction_cardinality(validated, row, lowering))
        .transpose()?
        .map(|value| quote! { , #value });
    let number = carries_number
        .then(|| construction_number(validated, row, lowering, number_override))
        .transpose()?
        .map(|value| quote! { , #value });
    let determiner_number = carries_determiner_number
        .then(|| construction_determiner_number(validated, row, lowering))
        .transpose()?
        .map(|value| quote! { , #value });
    let fused_head_license = carries_fused_head_license
        .then(|| construction_fused_head_license(validated, row, lowering))
        .transpose()?
        .map(|value| quote! { , #value });
    let nominal_license = carries_nominal_license
        .then(|| construction_nominal_license(validated, row, lowering))
        .transpose()?
        .map(|value| quote! { , #value });
    let onset = carries_onset
        .then(|| construction_onset(validated, row, lowering))
        .transpose()?
        .map(|value| quote! { , #value });
    let possessive_ending = carries_possessive_ending
        .then(|| construction_possessive_ending(validated, row, lowering))
        .transpose()?
        .map(|value| quote! { , #value });
    let wrapped = quote! { BuildValue::#category(#category_value #concord_class #cardinality #number #determiner_number #fused_head_license #nominal_license #onset #possessive_ending, #following_onset) };
    Ok(quote! { Ok(Some(#wrapped)) })
}

fn emit_fallible_element_success(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &mut Lowering,
    result: &TokenStream,
    concord_class_override: Option<FeatureValue>,
    number_override: Option<FeatureValue>,
) -> syn::Result<TokenStream> {
    if validated.explicit_sum_owns_construction_category(row.category()) {
        let element = ident(row.element_type());
        return Ok(quote! { #result.map(BuildValue::#element).map(Some) });
    }
    let category = ident(row.category());
    let variant = ident(row.category_variant());
    let following_onset = &lowering.output_following_onset;
    let carries_concord_class = validated.category_carries_concord_class(row.category());
    let carries_cardinality = validated.category_carries_cardinality(row.category());
    let carries_number = validated.category_carries_number(row.category());
    let carries_determiner_number =
        validated.carries_feature(row.category(), Feature::DeterminerNumber);
    let carries_fused_head_license =
        validated.carries_feature(row.category(), Feature::FusedHeadLicense);
    let carries_nominal_license =
        validated.carries_feature(row.category(), Feature::NominalLicense);
    let carries_onset = validated.category_carries_onset(row.category());
    let carries_possessive_ending = validated.category_carries_possessive_ending(row.category());
    let concord_class = carries_concord_class
        .then(|| construction_concord_class(validated, row, lowering, concord_class_override))
        .transpose()?;
    let cardinality = carries_cardinality
        .then(|| construction_cardinality(validated, row, lowering))
        .transpose()?;
    let number = carries_number
        .then(|| construction_number(validated, row, lowering, number_override))
        .transpose()?;
    let determiner_number = carries_determiner_number
        .then(|| construction_determiner_number(validated, row, lowering))
        .transpose()?;
    let fused_head_license = carries_fused_head_license
        .then(|| construction_fused_head_license(validated, row, lowering))
        .transpose()?;
    let nominal_license = carries_nominal_license
        .then(|| construction_nominal_license(validated, row, lowering))
        .transpose()?;
    let onset = carries_onset
        .then(|| construction_onset(validated, row, lowering))
        .transpose()?;
    let possessive_ending = carries_possessive_ending
        .then(|| construction_possessive_ending(validated, row, lowering))
        .transpose()?;
    let argument = lowering.constructor_map_local.clone().unwrap_or_else(|| {
        let argument = lowering.binders.allocate(row.construction_id());
        lowering.constructor_map_local = Some(argument.clone());
        argument
    });
    let concord_class_constraint_role = match validated.construction_concord_class_authority(row) {
        ConcordClassAuthorityPlan::SequenceConstraint { role, .. }
        | ConcordClassAuthorityPlan::ValueConstraint { role, .. } => Some(role),
        ConcordClassAuthorityPlan::Contextual | ConcordClassAuthorityPlan::Exact => None,
    };
    if let Some(role) = concord_class_constraint_role {
        let concord_class = concord_class.as_ref().ok_or_else(|| {
            internal("concord_class-constrained category lacks carried concord_class")
        })?;
        let helper = ident(&feature_helper("concord_class_matches", row.category()));
        let owner = syn::LitStr::new(row.element_type(), row.origin_span());
        let role = syn::LitStr::new(&role, row.origin_span());
        let cardinality = cardinality.map(|value| quote! { , #value });
        let number = number.map(|value| quote! { , #value });
        let determiner_number = determiner_number.map(|value| quote! { , #value });
        let fused_head_license = fused_head_license.map(|value| quote! { , #value });
        let nominal_license = nominal_license.map(|value| quote! { , #value });
        let onset = onset.map(|value| quote! { , #value });
        let possessive_ending = possessive_ending.map(|value| quote! { , #value });
        return Ok(quote! {
            match #result {
                Ok(#argument) => {
                    let value = #category::#variant(#argument);
                    if #helper(&value, #concord_class) {
                        Ok(Some(BuildValue::#category(
                            value,
                            #concord_class
                            #cardinality
                            #number
                            #determiner_number
                            #fused_head_license
                            #nominal_license
                            #onset
                            #possessive_ending
                            , #following_onset
                        )))
                    } else {
                        Err(BuildRejection::new(
                            #owner,
                            #role,
                            BuildViolation::Invariant {
                                identity: "value matches carried concord_class",
                            },
                        ))
                    }
                }
                Err(rejection) => Err(rejection),
            }
        });
    }
    let concord_class = concord_class.map(|value| quote! { , #value });
    let cardinality = cardinality.map(|value| quote! { , #value });
    let number = number.map(|value| quote! { , #value });
    let determiner_number = determiner_number.map(|value| quote! { , #value });
    let fused_head_license = fused_head_license.map(|value| quote! { , #value });
    let nominal_license = nominal_license.map(|value| quote! { , #value });
    let onset = onset.map(|value| quote! { , #value });
    let possessive_ending = possessive_ending.map(|value| quote! { , #value });
    Ok(quote! {
        #result
            .map(|#argument| { BuildValue::#category(#category::#variant(#argument) #concord_class #cardinality #number #determiner_number #fused_head_license #nominal_license #onset #possessive_ending, #following_onset) })
            .map(Some)
    })
}

fn emit_dynamic_match(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &mut Lowering,
    role: &syn::Ident,
) -> syn::Result<TokenStream> {
    let role_value = lowering
        .vocab_values
        .get(&identifier_key(role))
        .map(|value| quote! { #value })
        .or_else(|| lowering.field_values.get(&identifier_key(role)).cloned())
        .ok_or_else(|| internal("dynamic vocabulary role pattern is absent"))?;
    let number_arms = equation(validated, row, &FeaturePlace::Construction(Feature::Number))
        .and_then(|equation| match equation.value() {
            FeatureExpr::MatchVocab { arms, .. } => Some(arms.as_slice()),
            FeatureExpr::Constant(_) | FeatureExpr::FromRole { .. } => None,
        });
    let concord_class_arms = equation(
        validated,
        row,
        &FeaturePlace::Construction(Feature::ConcordClass),
    )
    .and_then(|equation| match equation.value() {
        FeatureExpr::MatchVocab { arms, .. } => Some(arms.as_slice()),
        FeatureExpr::Constant(_) | FeatureExpr::FromRole { .. } => None,
    });
    let driving_arms = number_arms
        .or(concord_class_arms)
        .ok_or_else(|| internal("dynamic feature match has no vocab arms"))?;
    let ty = ident(terminal_for_role(row, role)?);
    let mut arms = Vec::new();
    for (variant, _) in driving_arms {
        let variant = variant.value();
        let mut overrides = HashMap::new();
        overrides.insert(identifier_key(role), quote! { #ty::#variant });
        let concord_class_value = concord_class_arms.and_then(|arms| {
            arms.iter()
                .find(|(candidate, _)| identifier_key(candidate.value()) == identifier_key(variant))
                .map(|(_, value)| *value)
        });
        let number_value = number_arms.and_then(|arms| {
            arms.iter()
                .find(|(candidate, _)| identifier_key(candidate.value()) == identifier_key(variant))
                .map(|(_, value)| *value)
        });
        let success = emit_success(
            validated,
            row,
            lowering,
            Some(&overrides),
            concord_class_value,
            number_value,
        )?;
        let variant_pattern = quote! { #ty::#variant };
        let pattern = if let Some(number_arms) = number_arms
            && !lowering.dynamic_numbers.is_empty()
        {
            let number_value = number_arms
                .iter()
                .find(|(candidate, _)| identifier_key(candidate.value()) == identifier_key(variant))
                .map(|(_, value)| feature_value(*value))
                .ok_or_else(|| internal("dynamic feature maps are inconsistent"))?;
            let number_values = lowering.dynamic_numbers.iter().map(|_| &number_value);
            quote! { (#variant_pattern, #(#number_values),*) }
        } else {
            variant_pattern
        };
        arms.push(quote! { #pattern => #success });
    }
    if number_arms.is_some() && !lowering.dynamic_numbers.is_empty() {
        let numbers = &lowering.dynamic_numbers;
        Ok(quote! { match (#role_value, #(#numbers),*) { #(#arms,)* _ => Ok(None), } })
    } else {
        Ok(quote! { match #role_value { #(#arms,)* _ => Ok(None), } })
    }
}

fn stored_value(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
    role: &syn::Ident,
    overrides: Option<&HashMap<String, TokenStream>>,
) -> syn::Result<TokenStream> {
    if let Some(value) = overrides.and_then(|values| values.get(&identifier_key(role))) {
        return Ok(value.clone());
    }
    let mut value = lowering
        .field_values
        .get(&identifier_key(role))
        .cloned()
        .ok_or_else(|| internal("stored field has no build value"))?;
    let field = row.field(&identifier_key(role))?;
    if field.is_zeroable() {
        let ty = field.value_type();
        value = quote! {
            match #value {
                Some(value) => #ty::Headed(value),
                None => #ty::Zero,
            }
        };
    }
    if validated
        .boxed_fields()
        .contains(&(row.construction_id().to_owned(), identifier_key(role)))
    {
        value = quote! { Box::new(#value) };
    }
    Ok(value)
}

fn construction_concord_class(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
    concord_class_override: Option<FeatureValue>,
) -> syn::Result<TokenStream> {
    if let Some(concord_class) = concord_class_override {
        return Ok(feature_value(concord_class));
    }
    let output = resolve_feature_place(
        validated,
        row,
        lowering,
        &FeaturePlace::Construction(Feature::ConcordClass),
        &mut HashSet::new(),
    )?;
    Ok(resolved_feature_value_tokens(&output))
}

fn construction_number(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
    number_override: Option<FeatureValue>,
) -> syn::Result<TokenStream> {
    if let Some(number) = number_override {
        return Ok(feature_value(number));
    }
    let output = resolve_feature_place(
        validated,
        row,
        lowering,
        &FeaturePlace::Construction(Feature::Number),
        &mut HashSet::new(),
    )?;
    Ok(resolved_feature_value_tokens(&output))
}

fn construction_determiner_number(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
) -> syn::Result<TokenStream> {
    let output = resolve_feature_place(
        validated,
        row,
        lowering,
        &FeaturePlace::Construction(Feature::DeterminerNumber),
        &mut HashSet::new(),
    )?;
    Ok(resolved_feature_value_tokens(&output))
}

fn construction_nominal_license(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
) -> syn::Result<TokenStream> {
    let output = resolve_feature_place(
        validated,
        row,
        lowering,
        &FeaturePlace::Construction(Feature::NominalLicense),
        &mut HashSet::new(),
    )?;
    Ok(resolved_feature_value_tokens(&output))
}

fn construction_fused_head_license(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
) -> syn::Result<TokenStream> {
    let output = resolve_feature_place(
        validated,
        row,
        lowering,
        &FeaturePlace::Construction(Feature::FusedHeadLicense),
        &mut HashSet::new(),
    )?;
    Ok(resolved_feature_value_tokens(&output))
}

fn construction_cardinality(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
) -> syn::Result<TokenStream> {
    let output = resolve_feature_place(
        validated,
        row,
        lowering,
        &FeaturePlace::Construction(Feature::Cardinality),
        &mut HashSet::new(),
    )?;
    Ok(resolved_feature_value_tokens(&output))
}

fn construction_onset(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
) -> syn::Result<TokenStream> {
    let output = resolve_feature_place(
        validated,
        row,
        lowering,
        &FeaturePlace::Construction(Feature::Onset),
        &mut HashSet::new(),
    )?;
    Ok(resolved_feature_value_tokens(&output))
}

fn construction_possessive_ending(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
) -> syn::Result<TokenStream> {
    let output = resolve_feature_place(
        validated,
        row,
        lowering,
        &FeaturePlace::Construction(Feature::PossessiveEnding),
        &mut HashSet::new(),
    )?;
    Ok(resolved_feature_value_tokens(&output))
}

#[expect(
    clippy::too_many_lines,
    reason = "the generated feature-axis lowering matrix stays exhaustive in one dispatcher"
)]
fn resolve_feature_place(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
    place: &FeaturePlace,
    visiting: &mut HashSet<FeaturePlace>,
) -> syn::Result<ResolvedFeatureValue> {
    if let FeaturePlace::Role { field, feature } = place
        && let Some(value) = lowering
            .role_features
            .get(&(identifier_key(field), *feature))
    {
        return Ok(local_feature_value(value));
    }
    if let Some(crate::feature::FeatureResolution::Known(value)) =
        validated.feature_resolution(row.construction_id(), place)
    {
        return Ok(ResolvedFeatureValue::Known(value));
    }
    if !visiting.insert(place.clone()) {
        return Err(internal(
            "validated feature equation graph contains a cycle",
        ));
    }
    let equation = equation(validated, row, place).ok_or_else(|| {
        internal(&format!(
            "accepted feature source has no symbolic binding: construction `{}`, place `{place:?}`",
            row.construction_id(),
        ))
    })?;
    let resolved = match equation.value() {
        FeatureExpr::Constant(value) => ResolvedFeatureValue::Known(*value.value()),
        FeatureExpr::FromRole { role, feature } => resolve_feature_place(
            validated,
            row,
            lowering,
            &FeaturePlace::Role {
                field: role.clone(),
                feature: *feature,
            },
            visiting,
        )?,
        FeatureExpr::MatchVocab { role, arms } => {
            let source = lowering
                .field_values
                .get(&identifier_key(role))
                .ok_or_else(|| internal("vocabulary feature source value was not bound"))?;
            match place {
                FeaturePlace::Construction(Feature::ConcordClass)
                | FeaturePlace::Role {
                    feature: Feature::ConcordClass,
                    ..
                } => {
                    let helper = ident(&feature_helper(
                        "concord_class",
                        terminal_for_role(row, role)?,
                    ));
                    ResolvedFeatureValue::Computed(quote! { #helper(#source) })
                }
                FeaturePlace::Construction(Feature::Cardinality | Feature::Number)
                | FeaturePlace::Role {
                    feature: Feature::Cardinality | Feature::Number,
                    ..
                } => {
                    let ty = ident(terminal_for_role(row, role)?);
                    let arms = arms.iter().map(|(variant, value)| {
                        let variant = variant.value();
                        let value = feature_value(*value);
                        quote! { #ty::#variant => #value }
                    });
                    ResolvedFeatureValue::Computed(quote! { match #source { #(#arms,)* } })
                }
                FeaturePlace::Construction(
                    Feature::BareLocativeLicense
                    | Feature::Compoundability
                    | Feature::Countability
                    | Feature::HomographLicense
                    | Feature::MannerAnaphorClass
                    | Feature::LocativeTemporalLicense
                    | Feature::Properness
                    | Feature::Relationality,
                )
                | FeaturePlace::Role {
                    feature:
                        Feature::BareLocativeLicense
                        | Feature::Compoundability
                        | Feature::Countability
                        | Feature::HomographLicense
                        | Feature::MannerAnaphorClass
                        | Feature::LocativeTemporalLicense
                        | Feature::Properness
                        | Feature::Relationality,
                    ..
                } => {
                    return Err(internal(
                        "lexical classification is closed noun metadata, not an equation value",
                    ));
                }
                FeaturePlace::Construction(
                    Feature::BareLocativeComplement
                    | Feature::PrepositionComplementKind
                    | Feature::ModifierLicense
                    | Feature::PrepositionAttachment,
                )
                | FeaturePlace::Role {
                    feature:
                        Feature::BareLocativeComplement
                        | Feature::PrepositionComplementKind
                        | Feature::ModifierLicense
                        | Feature::PrepositionAttachment,
                    ..
                } => {
                    let ty = ident(terminal_for_role(row, role)?);
                    let arms = arms.iter().map(|(variant, value)| {
                        let variant = variant.value();
                        let value = feature_value(*value);
                        quote! { #ty::#variant => #value }
                    });
                    ResolvedFeatureValue::Computed(quote! { match #source { #(#arms,)* } })
                }
                FeaturePlace::Construction(
                    Feature::DeterminerNumber
                    | Feature::FusedHeadLicense
                    | Feature::NominalForm
                    | Feature::NominalLicense,
                )
                | FeaturePlace::Role {
                    feature:
                        Feature::DeterminerNumber
                        | Feature::FusedHeadLicense
                        | Feature::NominalForm
                        | Feature::NominalLicense,
                    ..
                } => {
                    let ty = ident(terminal_for_role(row, role)?);
                    let arms = arms.iter().map(|(variant, value)| {
                        let variant = variant.value();
                        let value = feature_value(*value);
                        quote! { #ty::#variant => #value }
                    });
                    ResolvedFeatureValue::Computed(quote! { match #source { #(#arms,)* } })
                }
                FeaturePlace::Construction(Feature::Onset)
                | FeaturePlace::Role {
                    feature: Feature::Onset,
                    ..
                } => {
                    let ty = ident(terminal_for_role(row, role)?);
                    let arms = arms.iter().map(|(variant, value)| {
                        let variant = variant.value();
                        let value = feature_value(*value);
                        quote! { #ty::#variant => #value }
                    });
                    ResolvedFeatureValue::Computed(quote! { match #source { #(#arms,)* } })
                }
                FeaturePlace::Construction(Feature::PossessiveEnding)
                | FeaturePlace::Role {
                    feature: Feature::PossessiveEnding,
                    ..
                } => {
                    let ty = ident(terminal_for_role(row, role)?);
                    let arms = arms.iter().map(|(variant, value)| {
                        let variant = variant.value();
                        let value = feature_value(*value);
                        quote! { #ty::#variant => #value }
                    });
                    ResolvedFeatureValue::Computed(quote! { match #source { #(#arms,)* } })
                }
                FeaturePlace::Construction(Feature::Participle)
                | FeaturePlace::Role {
                    feature: Feature::Participle,
                    ..
                } => ResolvedFeatureValue::Known(FeatureValue::Participle),
            }
        }
    };
    visiting.remove(place);
    Ok(resolved)
}

fn local_feature_value(value: &LocalFeatureValue) -> ResolvedFeatureValue {
    match value {
        LocalFeatureValue::Known(value) => ResolvedFeatureValue::Known(*value),
        LocalFeatureValue::Bound(binding) => ResolvedFeatureValue::Bound(binding.clone()),
        LocalFeatureValue::Computed(tokens) => ResolvedFeatureValue::Computed(tokens.clone()),
    }
}

fn resolved_feature_value_tokens(value: &ResolvedFeatureValue) -> TokenStream {
    match value {
        ResolvedFeatureValue::Known(value) => feature_value(*value),
        ResolvedFeatureValue::Bound(binding) => quote! { *#binding },
        ResolvedFeatureValue::Computed(tokens) => tokens.clone(),
    }
}

fn compare_feature_values(
    left: &ResolvedFeatureValue,
    right: &ResolvedFeatureValue,
) -> TokenStream {
    if let (ResolvedFeatureValue::Bound(left), ResolvedFeatureValue::Bound(right)) = (left, right) {
        quote! { #left == #right }
    } else {
        let left = resolved_feature_value_tokens(left);
        let right = resolved_feature_value_tokens(right);
        quote! { #left == #right }
    }
}

fn same_known_feature(left: &ResolvedFeatureValue, right: &ResolvedFeatureValue) -> bool {
    matches!(
        (left, right),
        (ResolvedFeatureValue::Known(left), ResolvedFeatureValue::Known(right)) if left == right
    )
}

fn dynamic_match_role(validated: &SemanticPlan, row: &ConstructionPlan) -> Option<syn::Ident> {
    [Feature::Number, Feature::ConcordClass]
        .into_iter()
        .find_map(|feature| {
            equation(validated, row, &FeaturePlace::Construction(feature)).and_then(|equation| {
                match equation.value() {
                    FeatureExpr::MatchVocab { role, .. } => Some(role.clone()),
                    FeatureExpr::Constant(_) | FeatureExpr::FromRole { .. } => None,
                }
            })
        })
}

fn equation<'a>(
    validated: &'a SemanticPlan,
    row: &ConstructionPlan,
    target: &FeaturePlace,
) -> Option<&'a crate::feature::FeatureEquation> {
    validated
        .feature_equations(row.construction_id())
        .iter()
        .find(|equation| equation.target() == target)
}

fn feature_is_read(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    role: &syn::Ident,
    feature: Feature,
) -> bool {
    validated.feature_equations(row.construction_id()).iter().any(|equation| {
        matches!(equation.value(), FeatureExpr::FromRole { role: source, feature: found } if identifier_key(source) == identifier_key(role) && *found == feature)
    }) || row.fields().iter().any(|field| {
        field.field_check().is_some_and(|(_, arguments)| {
            arguments.iter().any(|argument| {
                matches!(
                    argument,
                    FieldCheckArgumentPlan::Feature { role: source, feature: found }
                        if source == &identifier_key(role) && *found == feature
                )
            })
        })
    })
}

fn role_number_is_needed_in_build(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    _form: &crate::semantic::FormPlan,
    role: &syn::Ident,
) -> bool {
    feature_is_read(validated, row, role, Feature::Number)
        || equation(
            validated,
            row,
            &FeaturePlace::Role {
                field: role.clone(),
                feature: Feature::Number,
            },
        )
        .is_some()
        || matches!(
            equation(
                validated,
                row,
                &FeaturePlace::Construction(Feature::Number),
            )
            .map(crate::feature::FeatureEquation::value),
            Some(FeatureExpr::FromRole {
                role: source,
                feature: Feature::Number,
            }) if identifier_key(source) == identifier_key(role)
        )
}

fn terminal_for_role<'a>(row: &'a ConstructionPlan, role: &syn::Ident) -> syn::Result<&'a str> {
    row.field(&identifier_key(role))
        .map(crate::semantic::ConstructionFieldPlan::terminal)
}
fn feature_value(value: FeatureValue) -> TokenStream {
    match value {
        FeatureValue::ConcordOther => quote! { ConcordClass::Other },
        FeatureValue::ThirdPersonSingular => quote! { ConcordClass::ThirdPersonSingular },
        FeatureValue::QualifiedOnly => quote! { BareLocativeLicense::QualifiedOnly },
        FeatureValue::BareAllowed => quote! { BareLocativeLicense::BareAllowed },
        FeatureValue::Singular => quote! { Number::Singular },
        FeatureValue::Plural => quote! { Number::Plural },
        FeatureValue::Consonant => quote! { Onset::Consonant },
        FeatureValue::Vowel => quote! { Onset::Vowel },
        FeatureValue::EndsInS => quote! { PossessiveEnding::EndsInS },
        FeatureValue::Other => quote! { PossessiveEnding::Other },
        FeatureValue::Participle => quote! { Participle::Participle },
        FeatureValue::Zero => quote! { Cardinality::Zero },
        FeatureValue::One => quote! { Cardinality::One },
        FeatureValue::TwoPlus => quote! { Cardinality::TwoPlus },
        FeatureValue::Compoundable => quote! { Compoundability::Compoundable },
        FeatureValue::NonCompoundable => quote! { Compoundability::NonCompoundable },
        FeatureValue::Count => quote! { Countability::Count },
        FeatureValue::Mass => quote! { Countability::Mass },
        FeatureValue::HomographUnlicensed => quote! { HomographLicense::Unlicensed },
        FeatureValue::HomographLicensed => quote! { HomographLicense::Licensed },
        FeatureValue::OtherNoun => quote! { MannerAnaphorClass::OtherNoun },
        FeatureValue::MannerAnaphor => quote! { MannerAnaphorClass::MannerAnaphor },
        FeatureValue::Unrestricted => quote! { ModifierLicense::Unrestricted },
        FeatureValue::LocalDeterminer => quote! { ModifierLicense::LocalDeterminer },
        FeatureValue::No => quote! { BareLocativeComplement::No },
        FeatureValue::Yes => quote! { BareLocativeComplement::Yes },
        FeatureValue::UnrestrictedComplement => {
            quote! { PrepositionComplementKind::UnrestrictedComplement }
        }
        FeatureValue::RelationalComplement => {
            quote! { PrepositionComplementKind::RelationalComplement }
        }
        FeatureValue::SelectionComplement => {
            quote! { PrepositionComplementKind::SelectionComplement }
        }
        FeatureValue::SourceComplement => quote! { PrepositionComplementKind::SourceComplement },
        FeatureValue::InComplement => quote! { PrepositionComplementKind::InComplement },
        FeatureValue::OnComplement => quote! { PrepositionComplementKind::OnComplement },
        FeatureValue::TemporalComplement => {
            quote! { PrepositionComplementKind::TemporalComplement }
        }
        FeatureValue::Unlicensed => quote! { LocativeTemporalLicense::Unlicensed },
        FeatureValue::OfLicensed => quote! { LocativeTemporalLicense::OfLicensed },
        FeatureValue::OfAndOnLicensed => quote! { LocativeTemporalLicense::OfAndOnLicensed },
        FeatureValue::OfInAndOnLicensed => quote! { LocativeTemporalLicense::OfInAndOnLicensed },
        FeatureValue::InLicensed => quote! { LocativeTemporalLicense::InLicensed },
        FeatureValue::OnLicensed => quote! { LocativeTemporalLicense::OnLicensed },
        FeatureValue::InOrOnEdgeLicensed => {
            quote! { LocativeTemporalLicense::InOrOnEdgeLicensed }
        }
        FeatureValue::ObjectAttachmentLicensed => {
            quote! { LocativeTemporalLicense::ObjectAttachmentLicensed }
        }
        FeatureValue::TemporalLicensed => quote! { LocativeTemporalLicense::TemporalLicensed },
        FeatureValue::OfAndTemporalLicensed => {
            quote! { LocativeTemporalLicense::OfAndTemporalLicensed }
        }
        FeatureValue::AdjunctCapable => quote! { PrepositionAttachment::AdjunctCapable },
        FeatureValue::PostmodifierOnly => quote! { PrepositionAttachment::PostmodifierOnly },
        FeatureValue::SelectedOnly => quote! { PrepositionAttachment::SelectedOnly },
        FeatureValue::SingularOnly => quote! { DeterminerNumber::SingularOnly },
        FeatureValue::PluralOnly => quote! { DeterminerNumber::PluralOnly },
        FeatureValue::Both => quote! { DeterminerNumber::Both },
        FeatureValue::NominalOnly => quote! { FusedHeadLicense::NominalOnly },
        FeatureValue::PartitiveOnly => quote! { FusedHeadLicense::PartitiveOnly },
        FeatureValue::FusedHead => quote! { FusedHeadLicense::FusedHead },
        FeatureValue::PluralPredeterminer => quote! { FusedHeadLicense::PluralPredeterminer },
        FeatureValue::BareSingularNoun => quote! { NominalForm::BareSingularNoun },
        FeatureValue::ModifiedSingularNoun => quote! { NominalForm::ModifiedSingularNoun },
        FeatureValue::SingularCoordination => quote! { NominalForm::SingularCoordination },
        FeatureValue::BarePluralNoun => quote! { NominalForm::BarePluralNoun },
        FeatureValue::ModifiedPluralNoun => quote! { NominalForm::ModifiedPluralNoun },
        FeatureValue::PluralCoordination => quote! { NominalForm::PluralCoordination },
        FeatureValue::MassNoun => quote! { NominalForm::MassNoun },
        FeatureValue::AnyNominal => quote! { NominalLicense::AnyNominal },
        FeatureValue::CountNominal => quote! { NominalLicense::CountNominal },
        FeatureValue::LicensedBareSingularNoun => quote! { NominalLicense::BareSingularNoun },
        FeatureValue::LicensedMassOrPluralCount => quote! { NominalLicense::MassOrPluralCount },
        FeatureValue::Common => quote! { Properness::Common },
        FeatureValue::Proper => quote! { Properness::Proper },
        FeatureValue::NonRelational => quote! { Relationality::NonRelational },
        FeatureValue::QualifiedRelational => quote! { Relationality::QualifiedRelational },
        FeatureValue::DeterminedRelational => quote! { Relationality::DeterminedRelational },
        FeatureValue::Relational => quote! { Relationality::Relational },
        FeatureValue::SaturatedRelational => quote! { Relationality::SaturatedRelational },
    }
}
fn vocab_argument(name: &str) -> String {
    snake_case(name.strip_suffix("Word").unwrap_or(name))
}
fn ident(name: &str) -> syn::Ident {
    emitted_ident(name, Span::call_site())
}
fn internal(message: &str) -> syn::Error {
    syn::Error::new(Span::call_site(), message)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use quote::ToTokens;
    use syn::visit::Visit;

    struct Binders(Vec<String>);

    #[test]
    fn determinative_following_onset_survives_a_wrapper_and_is_discharged_by_onset() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                codec Head {
                    generate declaration_determinative {
                        closed = [
                            Article {
                                number_license = SingularOnly;
                                fused_head_license = NominalOnly;
                                nominal_license = CountNominal;
                                realizations = [
                                    { surface = "an"; following_onset = Vowel; },
                                    { surface = "a"; following_onset = Consonant; },
                                ];
                            },
                            Unconditioned {
                                number_license = SingularOnly;
                                fused_head_license = NominalOnly;
                                nominal_license = CountNominal;
                                realizations = [{ surface = "the"; }];
                            },
                        ];
                    }
                }
                construction wrapper: Wrapper {
                    element WrapperValue { head: lex Head, }
                    derive onset = head.onset;
                    form wrapper = lex(head);
                }
                construction vowel: Following {
                    element VowelFollowing {}
                    derive onset = Values::Vowel;
                    form vowel = "herb";
                }
                construction consonant: Following {
                    element ConsonantFollowing {}
                    derive onset = Values::Consonant;
                    form consonant = "creature";
                }
                construction consumed: Result {
                    element Consumed { determiner: Wrapper, following: Following, }
                    form consumed = determiner following;
                }
                root Result { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("following-onset fixture parses"),
        )
        .expect("following-onset fixture validates")
        .into_semantic();
        let source = super::emit(&plan)
            .expect("following-onset fixture emits")
            .remove(0)
            .tokens
            .to_string();

        assert!(
            source.contains("following_onset : head_following_onset"),
            "the lexical realization condition is bound: {source}",
        );
        assert!(
            source.contains("BuildValue :: Wrapper") && source.contains("head_following_onset"),
            "the wrapper preserves the erased condition: {source}",
        );
        assert!(
            source.contains("FeatureConstraint :: Any => true")
                && source.contains(
                    "FeatureConstraint :: Exact (expected) => expected == * following_onset"
                ),
            "Any accepts either onset while Exact accepts only the matching sealed onset: {source}",
        );
        assert!(
            source.contains("FeatureConstraint :: < Onset > :: Any"),
            "a consumed condition is discharged to Any: {source}",
        );
    }

    #[test]
    fn guarded_form_build_arms_reject_same_shape_values_outside_their_partition() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Word { That = "that", Those = "those", Other = "other", }
                construction demonstrative: NounPhrase {
                    element Demonstrative { word: lex Word, }
                    form that when word is That = lex(word);
                    form those when word is Those = lex(word);
                    form fallback otherwise = lex(word);
                }
                root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("guarded build fixture parses"),
        )
        .expect("guarded build fixture validates")
        .into_semantic();
        let item = super::emit(&plan)
            .expect("guarded build fixture emits")
            .remove(0);

        let that = build_arm(&item, "NounPhraseDemonstrativeThat")
            .to_token_stream()
            .to_string();
        assert!(that.contains("Word :: That"), "{that}");
        assert!(!that.contains("Word :: Those"), "{that}");
        let those = build_arm(&item, "NounPhraseDemonstrativeThose")
            .to_token_stream()
            .to_string();
        assert!(those.contains("Word :: Those"), "{those}");
        assert!(!those.contains("Word :: That"), "{those}");
        let fallback = build_arm(&item, "NounPhraseDemonstrativeFallback")
            .to_token_stream()
            .to_string();
        assert!(
            fallback.contains("Word :: That")
                && fallback.contains("Word :: Those")
                && fallback.contains('!'),
            "{fallback}",
        );
    }

    #[test]
    fn mobile_role_build_initializes_metadata_without_consuming_a_build_value() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
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
            .expect("mobile build fixture parses"),
        )
        .expect("mobile build fixture validates")
        .into_semantic();
        let item = super::emit(&plan)
            .expect("mobile build fixture emits")
            .remove(0);
        let source = build_arm(&item, "RootMobile").to_token_stream().to_string();
        assert!(
            source.contains("MobileHost :: try_new (tail . clone ())"),
            "{source}",
        );
        assert!(!source.contains("AdmissibleSites"), "{source}");
    }

    #[test]
    fn checked_required_category_field_emits_a_build_only_guard() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction determiner: Determinative {
                    element Determiner {}
                    derive fused_head_license = Values::FusedHead;
                    form determiner = "each";
                }
                construction partitive: Root {
                    element Partitive {
                        head: Determinative checked by determinative_is_fused(head.fused_head_license),
                    }
                    form partitive = head;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("checked required category fixture parses"),
        )
        .expect("checked required category fixture validates");
        let source = super::emit(validated.semantic())
            .expect("checked required category fixture emits")
            .remove(0)
            .tokens
            .to_string();

        for required in [
            "determinative_is_fused",
            "head_fused_head_license",
            "& head",
        ] {
            assert!(source.contains(required), "missing `{required}`: {source}");
        }
        assert!(
            !source.contains("Partitive { head : head , fused_head_license"),
            "transient feature must not enter the AST: {source}"
        );
    }

    #[test]
    fn checked_required_lexical_field_emits_a_build_only_guard() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                codec DeterminativeHead {
                    generate declaration_determinative {
                        closed = [
                            Each {
                                number_license = SingularOnly;
                                fused_head_license = FusedHead;
                                nominal_license = CountNominal;
                                realizations = [{ surface = "each"; }];
                            },
                        ];
                    }
                }
                construction partitive: Root {
                    element Partitive {
                        head: lex DeterminativeHead
                            checked by determinative_is_fused(head.fused_head_license),
                    }
                    form partitive = lex(head);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("checked required lexical fixture parses"),
        )
        .expect("checked required lexical fixture validates");
        let source = super::emit(validated.semantic())
            .expect("checked required lexical fixture emits")
            .remove(0)
            .tokens
            .to_string();

        for required in [
            "determinative_is_fused",
            "head_fused_head_license",
            "& head",
        ] {
            assert!(source.contains(required), "missing `{required}`: {source}");
        }
    }

    #[test]
    fn checked_field_reads_declared_verb_frame_role_prepositions() {
        let expansion = crate::generate(quote::quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme CoreVerb using EnglishVerb { Act = "act", }
            vocab Relation {
                Selected = "selected" {
                    feature PrepositionComplementKind = UnrestrictedComplement;
                },
            }
            codec FramedVerb {
                generate declaration_verb {
                    closed = CoreVerb;
                    position = Verb;
                    tail = [ObjectNounPhrase, lex(Relation::Selected), object: Object];
                    feature = ConcordClass;
                }
            }
            construction object: Object {
                element ObjectValue {}
                form object = "object";
            }
            construction framed: Root {
                element Framed {
                    head: lex FramedVerb,
                    direct_object: Object checked by accepts_role_prepositions(
                        head.verb_frame_role_prepositions
                    ),
                    object: Object checked by accepts_role_prepositions(
                        head.verb_frame_role_prepositions
                    ),
                }
                derive head.concord_class = Values::Other;
                form framed = verb(head) direct_object lex(Relation::Selected) object;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("Verb Frame projection fixture generates");
        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        assert!(
            source.contains("VerbFrameRolePreposition :: new (\"Relation\" , \"Selected\")"),
            "the accessor is derived from the declared frame: {source}",
        );
        assert_eq!(
            source
                .matches("verb_frame_role_prepositions_for_framed_verb (& head . clone ())",)
                .count(),
            1,
            "the governed fields share one ordered role state: {source}",
        );
        let direct_object_check = source
            .find("accepts_role_prepositions (& direct_object . clone () , & mut role_preemption)")
            .expect("the object is checked before the selected role");
        let selected_role = source
            .find("role_preemption . fill (VerbFrameRolePreposition :: new (\"Relation\" , \"Selected\"))")
            .expect("the selected role advances the ordered state");
        let complement_check = source
            .find("accepts_role_prepositions (& object . clone () , & mut role_preemption)")
            .expect("the complement is checked after the selected role");
        assert!(direct_object_check < selected_role && selected_role < complement_check);
    }

    #[test]
    fn checked_optional_category_calls_the_guard_only_when_present() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction feature_child: FeatureChild {
                    element FeatureChildValue {}
                    derive number = Values::Plural;
                    form feature_child = "child";
                }
                construction optional: Root {
                    element Optional {
                        feature: FeatureChild,
                        maybe: opt FeatureChild checked by accepts_optional(feature.number),
                    }
                    form optional = feature maybe;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("checked optional category fixture parses"),
        )
        .expect("checked optional category fixture validates");
        let source = super::emit(validated.semantic())
            .expect("checked optional category fixture emits")
            .remove(0)
            .tokens
            .to_string();

        assert!(
            source.contains("map_or (true"),
            "absence is accepted: {source}"
        );
        assert!(
            source.contains("accepts_optional (checked , * feature_number)"),
            "a present value is checked: {source}",
        );
    }

    #[test]
    fn checked_callback_binds_companion_category_feature_without_a_helper() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction determiner: Determinative {
                    element Determiner {}
                    derive fused_head_license = Values::FusedHead;
                    form determiner = "each";
                }
                construction object: Object {
                    element ObjectNode {}
                    derive number = Values::Plural;
                    form object = "creatures";
                }
                construction partitive: Root {
                    element Partitive {
                        head: Determinative checked by accepts_partitive(
                            head.fused_head_license,
                            whole.number,
                        ),
                        whole: Object,
                    }
                    form partitive = head whole;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("callback companion-feature fixture parses"),
        )
        .expect("callback companion-feature fixture validates");
        let source = super::emit(validated.semantic())
            .expect("callback companion-feature fixture emits")
            .remove(0)
            .tokens
            .to_string();

        assert!(
            source.contains("whole_number"),
            "missing direct binding: {source}"
        );
        assert!(
            !source.contains("number_for_object"),
            "must not fall back to a helper: {source}"
        );
    }

    fn shared_rhs_structural_plan() -> crate::semantic::SemanticPlan {
        crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Marker { Alpha = "alpha", Beta = "beta", }
                construction item: Item {
                    element ItemValue { marker: lex Marker, }
                    form item = lex(marker);
                }
                construction uniform: Uniform {
                    element UniformValue {
                        items: seq Item separated by "<S>" terminated by "<T>",
                    }
                    require len(items) >= 1;
                    form uniform = items;
                }
                construction finite: Finite {
                    element FiniteValue {
                        items: seq Item separated by "<S>" terminated by "<T>",
                    }
                    require len(items) >= 2;
                    require len(items) <= 4;
                    form finite = items;
                }
                root Item { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("shared-RHS fixture parses"),
        )
        .expect("shared-RHS fixture validates")
        .into_semantic()
    }

    #[test]
    fn sequence_build_patterns_consume_the_exact_lowered_rule_rhs() {
        let plan = shared_rhs_structural_plan();
        let mut row = super::super::rules::lowered_rows(&plan)
            .expect("shared-RHS rows lower")
            .into_iter()
            .find(|row| row.id == "UniformValueItemsSequenceRecursive")
            .expect("recursive sequence row");

        row.rhs.swap(1, 2);
        let mutated = super::emit_rule_arm(&plan, &row)
            .expect("a reordered sealed RHS still lowers")
            .to_string();
        let separator = mutated
            .find("Literal (\"<S>\")")
            .expect("separator pattern");
        let terminator = mutated
            .find("Literal (\"<T>\")")
            .expect("terminator pattern");
        assert!(
            separator < terminator,
            "build patterns must follow the mutated rule RHS order: {mutated}",
        );

        let mut wrong_category = row.clone();
        let helper = wrong_category
            .rhs
            .iter_mut()
            .find_map(|symbol| match symbol {
                super::super::rules::RuleSymbolPlan::Helper(category) => Some(category),
                _ => None,
            })
            .expect("recursive row has a helper symbol");
        *helper = "WrongSequenceCategory".to_owned();
        let mismatch = super::emit_rule_arm(&plan, &wrong_category)
            .expect_err("a sequence fold cannot consume a different helper category")
            .to_string();
        assert!(
            mismatch.contains("sequence_recursive") && mismatch.contains("helper category"),
            "the shared lowering mismatch must identify the row state: {mismatch}",
        );

        row.rhs.pop();
        let mismatch = super::emit_rule_arm(&plan, &row)
            .expect_err("a recursive fold without its lowered tail must be rejected")
            .to_string();
        assert!(
            mismatch.contains("sequence_recursive") && mismatch.contains("tail"),
            "the shared lowering mismatch must identify the row state: {mismatch}",
        );

        let finite_rows =
            super::super::rules::lowered_rows(&plan).expect("finite shared-RHS rows lower");
        let mut counted = finite_rows
            .iter()
            .find(|row| row.id == "FiniteValueItemsSequenceCount2Continue")
            .expect("counted continuation row")
            .clone();
        let counted_tail = counted
            .rhs
            .iter_mut()
            .find_map(|symbol| match symbol {
                super::super::rules::RuleSymbolPlan::Helper(category) => Some(category),
                _ => None,
            })
            .expect("counted continuation has a tail");
        *counted_tail = "FiniteValueItemsSequenceCount4Category".to_owned();
        let mismatch = super::emit_rule_arm(&plan, &counted)
            .expect_err("a counted fold cannot skip to a different count category")
            .to_string();
        assert!(
            mismatch.contains("sequence_count_2_continue") && mismatch.contains("helper category"),
            "the counted shared lowering mismatch must identify its row: {mismatch}",
        );

        let mut owner = finite_rows
            .iter()
            .find(|row| row.id == "FiniteValueItemsSequenceNonEmpty")
            .expect("finite nonempty owner row")
            .clone();
        let owner_tail = owner
            .rhs
            .iter_mut()
            .find_map(|symbol| match symbol {
                super::super::rules::RuleSymbolPlan::Helper(category) => Some(category),
                _ => None,
            })
            .expect("finite owner has a helper entry");
        *owner_tail = "FiniteValueItemsSequenceCount2Category".to_owned();
        let mismatch = super::emit_rule_arm(&plan, &owner)
            .expect_err("a finite owner cannot skip its first counted category")
            .to_string();
        assert!(
            mismatch.contains("sequence_non_empty") && mismatch.contains("helper category"),
            "the owner shared lowering mismatch must identify its row: {mismatch}",
        );
    }

    #[test]
    fn structural_helper_folds_are_tag_free_bounded_and_source_ordered() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Marker { Alpha = "alpha", Beta = "beta", }
                construction item: Item {
                    element ItemValue { marker: lex Marker, }
                    form item = lex(marker);
                }
                construction uniform: Uniform {
                    element UniformValue {
                        maybe: opt Item,
                        items: seq Item separated by "<S>" terminated by "<T>",
                    }
                    require len(items) >= 2;
                    form uniform = maybe items;
                }
                construction positional: Positional {
                    element PositionalValue {
                        items: seq Item separated by position {
                            pair = "<P>";
                            first = "<F>";
                            middle = "<M>";
                            last = "<L>";
                        } terminated by "<T>",
                    }
                    require len(items) >= 1;
                    form positional = items;
                }
                root Item { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("structural build fixture parses"),
        )
        .expect("structural build fixture validates")
        .into_semantic();
        let item = super::emit(&plan)
            .expect("structural build fixture lowers")
            .remove(0);
        let syn::Item::Fn(function) = syn::parse2(item.tokens).expect("build item parses") else {
            panic!("build is a function")
        };
        let syn::Stmt::Expr(syn::Expr::Match(dispatch), None) = &function.block.stmts[0] else {
            panic!("build is a flat rule dispatch")
        };
        let arms = dispatch
            .arms
            .iter()
            .map(|arm| {
                let id = arm.pat.to_token_stream().to_string();
                (id, arm.to_token_stream().to_string())
            })
            .collect::<Vec<_>>();
        let arm = |name: &str| {
            &arms
                .iter()
                .find(|(id, _)| id == &format!("RuleId :: {name}"))
                .unwrap_or_else(|| panic!("missing structural build arm {name}"))
                .1
        };

        assert!(
            arm("UniformValueMaybeOptionalAbsent")
                .contains("BuildValue :: UniformValueMaybeOptional (None)"),
        );
        assert!(
            arm("UniformValueMaybeOptionalPresent")
                .contains("BuildValue :: UniformValueMaybeOptional (Some (item . clone ()))"),
        );
        assert!(
            !arms
                .iter()
                .any(|(id, _)| id == "RuleId :: UniformValueItemsSequenceEmpty"
                    || id == "RuleId :: UniformValueItemsSequenceSingleton"),
            "the minimum-two sequence must expose neither an empty nor singleton build arm",
        );
        assert!(
            arm("UniformValueItemsSequenceLength2")
                .contains("BuildValue :: UniformValueItemsSequence (vec ! [item_0 . clone () , item_3 . clone ()])"),
        );
        let recursive = arm("UniformValueItemsSequenceRecursive");
        assert!(
            recursive.contains("values . push (item_0 . clone ())"),
            "{recursive}"
        );
        assert!(
            recursive.contains("values . extend (tail . iter () . cloned ())"),
            "{recursive}",
        );
        assert!(
            !recursive.contains("reverse") && !recursive.contains("insert"),
            "the fold is source ordered without later repair: {recursive}",
        );

        let owner = arm("UniformUniform");
        assert!(
            owner.contains("UniformValue :: try_new")
                && owner.contains("BuildValue :: UniformValueItemsSequence (items)"),
            "the public owner validates bounds through its constructor: {owner}",
        );
        let positional = arm("PositionalValueItemsSequenceThreePlus");
        assert!(
            positional.contains("values . push (item_0 . clone ())")
                && positional.contains("values . extend (tail . iter () . cloned ())")
                && positional.contains("PositionalValue :: try_new"),
            "the owner folds first + ordered tail and applies bounds: {positional}",
        );
        for helper in [
            "UniformValueMaybeOptionalAbsent",
            "UniformValueMaybeOptionalPresent",
            "UniformValueItemsSequenceLength2",
            "UniformValueItemsSequenceRecursive",
            "PositionalValueItemsSequenceLast",
            "PositionalValueItemsSequenceMiddle",
        ] {
            let source = arm(helper);
            assert!(
                !source.contains("Form") && !source.contains("Separator"),
                "helper carrier stores neither a form nor separator tag: {source}",
            );
        }
    }

    impl<'ast> Visit<'ast> for Binders {
        fn visit_pat_ident(&mut self, pattern: &'ast syn::PatIdent) {
            self.0.push(pattern.ident.to_string());
            syn::visit::visit_pat_ident(self, pattern);
        }
    }

    #[test]
    fn invariant_arms_extract_broad_values_and_call_generated_constructors_exactly() {
        let plan = invariant_build_plan();
        let items = super::emit(&plan).expect("invariant build fixture emits");

        for (rule, expected) in [
            (
                "ChildRecursive",
                syn::parse_quote! {
                    RuleId::ChildRecursive => match children {
                        [BuildValue::Child(child, child_following_onset), BuildValue::Leaf(Leaf::Mode(mode))]
                            if match *child_following_onset {
                                FeatureConstraint::Any => true,
                                FeatureConstraint::Exact(expected) => expected == match mode {
                                    Mode::One => Onset::Consonant,
                                    Mode::Two => Onset::Consonant,
                                },
                            } => RecursiveNode::try_new(Box::new(child.clone()), *mode)
                                .map(|recursive| { BuildValue::Child(Child::Recursive(recursive), FeatureConstraint::<Onset>::Any) })
                                .map(Some),
                        _ => Ok(None),
                    }
                },
            ),
            (
                "RootCategoryGuarded",
                syn::parse_quote! {
                    RuleId::RootCategoryGuarded => match children {
                        [BuildValue::Child(child, child_following_onset)]
                            if match (FeatureConstraint::<Onset>::Any, *child_following_onset) {
                                (FeatureConstraint::Any, _) | (_, FeatureConstraint::Any) => true,
                                (FeatureConstraint::Exact(left), FeatureConstraint::Exact(right)) => left == right,
                            } => CategoryGuarded::try_new(child.clone())
                                .map(|category_guarded| { BuildValue::Root(Root::CategoryGuarded(category_guarded), match (FeatureConstraint::<Onset>::Any, *child_following_onset) {
                                    (FeatureConstraint::Any, right) => right,
                                    (left, FeatureConstraint::Any) => left,
                                    (FeatureConstraint::Exact(left), FeatureConstraint::Exact(_)) => FeatureConstraint::Exact(left),
                                }) })
                                .map(Some),
                        _ => Ok(None),
                    }
                },
            ),
            (
                "RootVocabGuarded",
                syn::parse_quote! {
                    RuleId::RootVocabGuarded => match children {
                        [BuildValue::Leaf(Leaf::Mode(mode))] => VocabGuarded::try_new(*mode)
                            .map(|vocab_guarded| { BuildValue::Root(Root::VocabGuarded(vocab_guarded), FeatureConstraint::<Onset>::Any) })
                            .map(Some),
                        _ => Ok(None),
                    }
                },
            ),
            (
                "RootDnfGuarded",
                syn::parse_quote! {
                    RuleId::RootDnfGuarded => match children {
                        [BuildValue::Leaf(Leaf::Mode(mode)), BuildValue::Child(child, child_following_onset)]
                            if match (FeatureConstraint::<Onset>::Any, *child_following_onset) {
                                (FeatureConstraint::Any, _) | (_, FeatureConstraint::Any) => true,
                                (FeatureConstraint::Exact(left), FeatureConstraint::Exact(right)) => left == right,
                            } => DnfGuarded::try_new(*mode, child.clone())
                                .map(|dnf_guarded| { BuildValue::Root(Root::DnfGuarded(dnf_guarded), match (FeatureConstraint::<Onset>::Any, *child_following_onset) {
                                    (FeatureConstraint::Any, right) => right,
                                    (left, FeatureConstraint::Any) => left,
                                    (FeatureConstraint::Exact(left), FeatureConstraint::Exact(_)) => FeatureConstraint::Exact(left),
                                }) })
                                .map(Some),
                        _ => Ok(None),
                    }
                },
            ),
            (
                "RootContextGuarded",
                syn::parse_quote! {
                    RuleId::RootContextGuarded => match children {
                        [BuildValue::Leaf(Leaf::SelfReference(context_2))] => ContextGuarded::try_new(*context_2, context)
                            .map(|context_guarded| { BuildValue::Root(Root::ContextGuarded(context_guarded), FeatureConstraint::<Onset>::Any) })
                            .map(Some),
                        _ => Ok(None),
                    }
                },
            ),
        ] {
            assert_eq!(build_arm(&items[0], rule), expected, "{rule} build arm");
        }
    }

    #[test]
    fn invariant_arms_contain_no_authored_predicate_members_or_duplicate_guards() {
        let plan = invariant_build_plan();
        let item = super::emit(&plan)
            .expect("invariant build fixture emits")
            .remove(0);

        for rule in [
            "ChildRecursive",
            "RootCategoryGuarded",
            "RootVocabGuarded",
            "RootDnfGuarded",
        ] {
            let source = build_arm(&item, rule).to_token_stream().to_string();
            for forbidden in ["Child :: First", "Child :: Second", "matches !"] {
                assert!(
                    !source.contains(forbidden),
                    "{rule} duplicates invariant predicate `{forbidden}`: {source}",
                );
            }
            assert!(
                source.contains(":: try_new"),
                "{rule} calls Element::try_new: {source}"
            );
        }
    }

    #[test]
    fn invariant_materialization_executes_actual_ast_and_build_items() {
        let plan = invariant_build_plan();
        let ast = super::super::ast::emit(&plan).expect("invariant AST fixture emits");
        let build = super::emit(&plan).expect("invariant build fixture emits");
        let runtime_items = super::super::runtime::emit(&plan)
            .into_iter()
            .filter(|item| match &item.key {
                crate::ItemKey::Named { name, .. } | crate::ItemKey::Impl { self_ty: name, .. } => {
                    matches!(name.as_str(), "BuildRejection" | "BuildViolation")
                }
            })
            .collect::<Vec<_>>();
        let runtime = runtime_items.iter().map(|item| &item.tokens);
        let ast = ast.iter().map(|item| &item.tokens);
        let build = build.iter().map(|item| &item.tokens);
        let source = quote::quote! {
            #![allow(dead_code)]

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum Mode { One, Two }
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum SelfReferenceSpelling { Full, Abbreviated }
            struct ParseContext<'a> {
                canonical: SelfReferenceSpelling,
                marker: &'a str,
            }

            impl SelfReferenceSpelling {
                fn valid_in(self, context: &ParseContext<'_>) -> bool {
                    self == context.canonical
                }
            }

            #[derive(Debug)]
            enum Leaf {
                Mode(Mode),
                SelfReference(SelfReferenceSpelling),
                Literal(&'static str),
                EndOfInput,
            }

            #[derive(Debug)]
            enum RuleId {
                ChildFirst,
                ChildSecond,
                ChildRecursive,
                RootCategoryGuarded,
                RootVocabGuarded,
                RootDnfGuarded,
                RootContextGuarded,
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum Onset { Consonant, Vowel }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum FeatureConstraint<F> { Exact(F), Any }

            mod generated {
                use super::*;
                #(#runtime)*
                #(#ast)*
                #(#build)*
            }

            #[derive(Debug)]
            enum BuildValue {
                Child(generated::Child, FeatureConstraint<Onset>),
                Root(generated::Root, FeatureConstraint<Onset>),
                Leaf(Leaf),
            }

            fn main() {
                let context = ParseContext {
                    canonical: SelfReferenceSpelling::Full,
                    marker: "materialization",
                };
                let valid = [
                    BuildValue::Child(
                        generated::Child::First(generated::FirstChild),
                        FeatureConstraint::Any,
                    ),
                ];
                let invalid = [
                    BuildValue::Child(
                        generated::Child::Second(generated::SecondChild),
                        FeatureConstraint::Any,
                    ),
                ];

                assert!(matches!(
                    generated::build(RuleId::RootCategoryGuarded, &valid, &context),
                    Some(BuildValue::Root(generated::Root::CategoryGuarded(_), _))
                ));
                assert!(generated::build(
                    RuleId::RootCategoryGuarded,
                    &invalid,
                    &context,
                ).is_none());
            }
        }
        .to_string();

        let directory = tempfile::tempdir().expect("temporary build harness directory");
        let source_path = directory.path().join("invariant_build_harness.rs");
        let binary_path = directory.path().join("invariant_build_harness");
        std::fs::write(&source_path, &source).expect("write deterministic build harness");
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
            "build harness compilation failed\nstdout:\n{}\nstderr:\n{}\nsource:\n{source}",
            String::from_utf8_lossy(&compilation.stdout),
            String::from_utf8_lossy(&compilation.stderr),
        );

        let execution = std::process::Command::new(&binary_path)
            .output()
            .expect("execute compiled invariant build harness");
        assert!(
            execution.status.success(),
            "build harness execution failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&execution.stdout),
            String::from_utf8_lossy(&execution.stderr),
        );
    }

    fn invariant_build_plan() -> crate::semantic::SemanticPlan {
        crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
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
                construction recursive: Child {
                    element RecursiveNode { child: Child, mode: lex Mode, }
                    require any(
                        all(child is First, mode is One),
                        all(child is Second, mode is Two)
                    );
                    form recursive = child lex(mode);
                }
                construction category_guarded: Root {
                    element CategoryGuarded { child: Child, }
                    require child is First;
                    form category_guarded = child;
                }
                construction vocab_guarded: Root {
                    element VocabGuarded { mode: lex Mode, }
                    require mode is One;
                    form vocab_guarded = lex(mode);
                }
                construction dnf_guarded: Root {
                    element DnfGuarded { mode: lex Mode, child: Child, }
                    require any(
                        all(mode is One, child is First),
                        all(mode is Two, child is Second)
                    );
                    form dnf_guarded = lex(mode) child;
                }
                construction context_guarded: Root {
                    element ContextGuarded {
                        context: identity SelfReferenceSpelling,
                    }
                    form context_guarded = identity(context);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("invariant build fixture parses"),
        )
        .expect("invariant build fixture validates")
        .into_semantic()
    }

    fn build_arm(item: &crate::GeneratedItem, rule: &str) -> syn::Arm {
        let syn::Item::Fn(function) = syn::parse2(item.tokens.clone()).unwrap() else {
            panic!("build is a function")
        };
        let syn::Stmt::Expr(syn::Expr::Match(dispatch), None) = &function.block.stmts[0] else {
            panic!("flat match dispatch")
        };
        let mut arm = dispatch
            .arms
            .iter()
            .find(|arm| {
                arm.pat
                    .to_token_stream()
                    .to_string()
                    .contains(&format!("RuleId :: {rule}"))
            })
            .unwrap_or_else(|| panic!("{rule} build arm exists"))
            .clone();
        arm.comma = None;
        arm
    }

    #[test]
    fn role_derived_noun_build_guards_scanner_number_and_keeps_fallback() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::role_derived_noun_tokens()).unwrap(),
        )
        .unwrap();
        let items = super::emit(validated.semantic()).expect("role-derived noun build lowers");
        let source = items[0].tokens.to_string();
        assert!(
            source.contains("Leaf :: Noun { noun : head , number , onset : head_onset , possessive_ending : head_possessive_ending , }"),
            "{source}"
        );
        assert!(
            source.contains("* source_number == * number"),
            "the scanner number must equal the bound parser-domain source number: {source}"
        );
        assert!(
            !source.contains("number_for_source"),
            "build must not call a render-domain number helper: {source}"
        );
        assert!(
            source.contains("head : head . clone ()"),
            "the noun value must be propagated into the construction: {source}"
        );
        assert!(
            source.contains("_ => Ok (None)"),
            "a scanner-number mismatch must retain the ordinary fallback: {source}"
        );
    }

    #[test]
    fn vocab_matched_number_without_noun_matches_only_the_stored_vocab() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(
                crate::test_support::vocab_matched_number_without_noun_tokens(),
            )
            .unwrap(),
        )
        .unwrap();
        let source = super::emit(validated.semantic())
            .expect("zero-noun vocab number match lowers")
            .remove(0)
            .tokens
            .to_string();
        assert!(
            source.contains("Count :: One => Ok (Some")
                && source.contains("Count :: Many => Ok (Some"),
            "the stored vocab alone selects the output features: {source}"
        );
        assert!(
            !source.contains("dynamic number pattern is absent"),
            "{source}"
        );
    }

    #[test]
    fn vocab_matched_number_constrains_every_noun_number() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(
                crate::test_support::vocab_matched_number_with_two_nouns_tokens(),
            )
            .unwrap(),
        )
        .unwrap();
        let source = super::emit(validated.semantic())
            .expect("two-noun vocab number match lowers")
            .remove(0)
            .tokens
            .to_string();
        assert!(source.contains("number , right_number"), "{source}");
        assert!(
            source.contains("Count :: One , Number :: Singular , Number :: Singular")
                && source.contains("Count :: Many , Number :: Plural , Number :: Plural"),
            "every noun scanner number must match the vocab-selected number: {source}"
        );
        assert!(source.contains("_ => Ok (None)"), "{source}");
    }

    #[test]
    fn concord_class_match_with_constant_number_uses_vocab_only_dispatch() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Count { One = "one", Many = "many", }
                codec Head {
                    atom = noun;
                    value_type = Head;
                    lexical = Lexical::Head;
                    render = render_head;
                    build { pattern = BuildValue::Head(head); construct = head; }
                    traversal { callback = borrowed; argument = head; call visitor::visit_head(borrowed(head)); }
                }
                construction only: Root {
                    element Only { count: lex Count, head: lex Head, }
                    derive concord_class = match count {
                        One => Values::ThirdPersonSingular,
                        Many => Values::Other,
                    };
                    derive number = Values::Singular;
                    form only = lex(count) noun(head);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .unwrap(),
        )
        .unwrap();
        let source = super::emit(validated.semantic())
            .expect("concord_class-only vocab dispatch lowers")
            .remove(0)
            .tokens
            .to_string();
        assert!(
            source.contains(
                "Leaf :: Noun { noun : head , number : Number :: Singular , onset : head_onset , possessive_ending : head_possessive_ending , }"
            ) && source.contains("match count")
                && source.contains("Count :: One => Ok (Some")
                && source.contains("ConcordClass :: ThirdPersonSingular")
                && source.contains("Count :: Many => Ok (Some")
                && source.contains("ConcordClass :: Other"),
            "concord_class matching is vocab-only while noun number stays constant: {source}"
        );
        assert!(source.contains("_ => Ok (None)"), "{source}");
    }

    #[test]
    fn role_derived_number_guards_every_noun_scanner_value() {
        let fixture = quote::quote! {
            codec Head {
                atom = noun;
                value_type = Head;
                lexical = Lexical::Head;
                render = render_head;
                build { pattern = BuildValue::Head(head); construct = head; }
                traversal { callback = borrowed; argument = head; call visitor::visit_head(borrowed(head)); }
            }
            construction source: Source {
                element SourceNode {}
                derive number = Values::Singular;
                form source = "source";
            }
            construction pair: Phrase {
                element PairNode { source: Source, left: lex Head, right: lex Head, }
                derive number = source.number;
                form pair = source noun(left) noun(right);
            }
            root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        };
        let expansion =
            crate::generate(fixture.clone()).expect("two FromRole nouns are backend-complete");
        assert!(!expansion.items().is_empty());
        let validated =
            crate::validate_declarations(crate::parse_declarations(fixture).unwrap()).unwrap();
        let source = super::emit(validated.semantic())
            .expect("two FromRole noun guards lower")
            .remove(0)
            .tokens
            .to_string();
        for fragment in [
            "* source_number == * number",
            "* source_number == * right_number",
            "Leaf :: Noun { noun : left , number , onset : left_onset , possessive_ending : left_possessive_ending , }",
            "Leaf :: Noun { noun : right , number : right_number , onset : right_onset , possessive_ending : right_possessive_ending , }",
            "_ => Ok (None)",
        ] {
            assert!(source.contains(fragment), "missing `{fragment}`: {source}");
        }
        assert!(!source.contains("number_for_source"), "{source}");
    }

    #[test]
    fn refined_vocab_writer_constrains_a_category_feature_with_a_literal_pattern() {
        let fixture = quote::quote! {
            vocab Mode { One = "one", Many = "many", }
            construction bare: Child {
                element BareChild {}
                derive concord_class = Values::Other;
                form bare = "bare";
            }
            construction third: Child {
                element ThirdChild {}
                derive concord_class = Values::ThirdPersonSingular;
                form third = "third";
            }
            construction parent: Parent {
                element ParentNode { mode: lex Mode, child: Child, }
                require mode is One;
                derive mode.concord_class = match mode {
                    One => Values::ThirdPersonSingular,
                    Many => Values::Other,
                };
                derive child.concord_class = mode.concord_class;
                form parent = lex(mode) child;
            }
            root Parent { punctuation = "."; eoi = true; standalone_render = true; }
        };
        let validated =
            crate::validate_declarations(crate::parse_declarations(fixture.clone()).unwrap())
                .unwrap();
        crate::generate(fixture).expect("a refined local writer is backend-complete");
        let source = super::emit(validated.semantic())
            .expect("the refined writer lowers to a literal child pattern")
            .remove(0)
            .tokens
            .to_string();
        assert!(
            source.contains("BuildValue :: Child (child , ConcordClass :: ThirdPersonSingular , child_following_onset)"),
            "the required Mode::One arm must constrain the child concord_class: {source}"
        );
        assert!(!source.contains("feature source was not bound"), "{source}");
        assert!(source.contains("_ => Ok (None)"), "{source}");
    }

    #[test]
    fn adversarial_roles_receive_unique_pattern_binders_and_exact_substitutions() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Count { One = "one", Many = "many", }
                codec Head {
                    atom = noun;
                    value_type = Head;
                    lexical = Lexical::Head;
                    render = render_head;
                    build { pattern = BuildValue::Head(head); construct = head; }
                    traversal { callback = borrowed; argument = head; call visitor::visit_head(borrowed(head)); }
                }
                construction collision: Collision {
                    element CollisionNode {
                        count: lex Count,
                        number: lex Head,
                        right_number: lex Head,
                    }
                    derive number = match count {
                        One => Values::Singular,
                        Many => Values::Plural,
                    };
                    form collision = noun(number) noun(right_number) lex(count);
                }
                root Collision { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .unwrap(),
        )
        .unwrap();
        let item = super::emit(validated.semantic())
            .expect("colliding preferred names are allocated safely")
            .remove(0);
        let syn::Item::Fn(function) = syn::parse2(item.tokens.clone()).unwrap() else {
            panic!("build is a function")
        };
        let syn::Stmt::Expr(syn::Expr::Match(dispatch), None) = &function.block.stmts[0] else {
            panic!("flat match dispatch")
        };
        let arm = dispatch
            .arms
            .iter()
            .find(|arm| {
                quote::ToTokens::to_token_stream(&arm.pat)
                    .to_string()
                    .contains("Collision")
            })
            .expect("collision rule arm");
        let syn::Expr::Match(children) = &*arm.body else {
            panic!("rule arm matches children")
        };
        let mut binders = Binders(Vec::new());
        binders.visit_pat(&children.arms[0].pat);
        assert_eq!(
            binders.0.len(),
            binders.0.iter().collect::<HashSet<_>>().len(),
            "every binding in one child pattern must be unique: {:?}",
            binders.0
        );
        let source = item.tokens.to_string();
        assert!(source.contains("number : number . clone ()"), "{source}");
        assert!(
            source.contains("right_number : right_number . clone ()"),
            "{source}"
        );
    }

    #[test]
    fn binding_build_construct_consumes_only_semantic_pattern_slots() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                codec Pair {
                    atom = lex;
                    value_type = Pair;
                    lexical = Lexical::Pair;
                    render = render_runtime_type;
                    build {
                        pattern = BuildValue::Pair(left, right);
                        construct = Pair::new(left.code, (Factory::wrap(right)));
                    }
                    traversal {
                        callback = borrowed;
                        argument = value;
                        call visitor::visit_pair(borrowed(value));
                    }
                }
                construction wrapped: Root {
                    element Wrapped { value: lex Pair, }
                    form wrapped = lex(value);
                }
                root Root { punctuation = "!"; eoi = true; standalone_render = true; }
            })
            .unwrap(),
        )
        .expect("a two-slot closed binding construction validates");
        let items =
            super::emit(validated.semantic()).expect("validated binding construction lowers");
        let syn::Item::Fn(function) = syn::parse2(items[0].tokens.clone()).unwrap() else {
            panic!("build is a function")
        };
        let syn::Stmt::Expr(syn::Expr::Match(dispatch), None) = &function.block.stmts[0] else {
            panic!("flat match dispatch")
        };
        let mut arm = dispatch.arms[0].clone();
        arm.comma = None;
        let expected: syn::Arm = syn::parse_quote! {
            RuleId::RootWrapped => match children {
                [
                    BuildValue::Leaf(Leaf::Pair(BoundLeaf::Pair(left, right)))
                ] => Ok(Some(BuildValue::Root(Root::Wrapped(Wrapped {
                    value: Pair::new(left.code, (Factory::wrap(right)))
                }), FeatureConstraint::<Onset>::Any))),
                _ => Ok(None),
            }
        };
        assert_eq!(arm, expected);
    }

    #[test]
    fn positional_sequence_onset_carries_the_first_member_without_homogeneity_guards() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction vowel: Item {
                    element VowelItem {}
                    derive onset = Values::Vowel;
                    form vowel = "apple";
                }
                construction consonant: Item {
                    element ConsonantItem {}
                    derive onset = Values::Consonant;
                    form consonant = "bear";
                }
                construction coordinated: Root {
                    element Coordinated {
                        members: seq Item separated by position {
                            pair = " and ";
                            first = ", ";
                            middle = ", ";
                            last = ", and ";
                        },
                    }
                    require len(members) >= 2;
                    derive onset = members.onset;
                    form coordinated = members;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("first-member onset fixture parses"),
        )
        .expect("first-member onset fixture validates");
        let source = super::emit(validated.semantic())
            .expect("first-member onset sequence build emits")
            .remove(0)
            .tokens
            .to_string();

        for required in [
            "item_0_onset",
            "tail_onset",
            "BuildValue :: CoordinatedMembersSequence (vec ! [item_0 . clone () , item_2 . clone ()] , * item_0_onset)",
            "BuildValue :: CoordinatedMembersSequence (values , * item_0_onset)",
        ] {
            assert!(source.contains(required), "missing `{required}`: {source}");
        }
        for forbidden in ["item_0_onset == item_2_onset", "item_0_onset == tail_onset"] {
            assert!(
                !source.contains(forbidden),
                "unexpected `{forbidden}`: {source}"
            );
        }
    }

    #[test]
    fn positional_sequence_possessive_ending_carries_the_last_member_without_homogeneity_guards() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                construction ends_in_s: Item {
                    element EndsInSItem {}
                    derive possessive_ending = Values::EndsInS;
                    form ends_in_s = "artifacts";
                }
                construction other: Item {
                    element OtherItem {}
                    derive possessive_ending = Values::Other;
                    form other = "bear";
                }
                construction coordinated: Root {
                    element Coordinated {
                        members: seq Item separated by position {
                            pair = " and ";
                            first = ", ";
                            middle = ", ";
                            last = ", and ";
                        },
                    }
                    require len(members) >= 2;
                    derive possessive_ending = members.possessive_ending;
                    form coordinated = members;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("last-member possessive-ending fixture parses"),
        )
        .expect("last-member possessive-ending fixture validates");
        let source = super::emit(validated.semantic())
            .expect("last-member possessive-ending sequence build emits")
            .remove(0)
            .tokens
            .to_string();

        for required in [
            "item_0_possessive_ending",
            "item_2_possessive_ending",
            "tail_possessive_ending",
            "BuildValue :: CoordinatedMembersSequence (vec ! [item_0 . clone () , item_2 . clone ()] , * item_2_possessive_ending)",
            "BuildValue :: CoordinatedMembersSequence (values , * tail_possessive_ending)",
        ] {
            assert!(source.contains(required), "missing `{required}`: {source}");
        }
        for forbidden in [
            "item_0_possessive_ending == item_2_possessive_ending",
            "item_0_possessive_ending == tail_possessive_ending",
        ] {
            assert!(
                !source.contains(forbidden),
                "unexpected `{forbidden}`: {source}"
            );
        }
    }

    #[test]
    fn allocator_reserves_abi_and_feature_locals() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Marker { One = "one", }
                morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
                lexeme Verbs using EnglishVerb { Act = "act", }
                codec Pair {
                    atom = lex;
                    value_type = Pair;
                    lexical = Lexical::Pair;
                    render = render_pair;
                    build {
                        pattern = BuildValue::Pair(context, children, rule);
                        construct = Pair::new(context, children, rule);
                    }
                    traversal {
                        callback = borrowed;
                        argument = pair;
                        call visitor::visit_pair(borrowed(pair));
                    }
                }
                construction context_container: ContextContainer {
                    element ContextContainerNode { pair: lex Pair, }
                    form context_container = lex(pair);
                }
                construction concord_class: FeatureContainer {
                    element ConcordClassNode { marker: lex Marker, }
                    derive concord_class = verb.concord_class;
                    form concord_class = lex(marker) verb(Verbs::Act);
                }
                construction entry: Entry { element EntryNode {} form entry = "entry"; }
                root Entry { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .unwrap(),
        )
        .expect("the adversarial ABI-local fixture validates");
        let source = super::emit(validated.semantic())
            .expect("all accepted local names lower hygienically")
            .remove(0)
            .tokens
            .to_string();

        for fragment in [
            "Leaf :: Pair (context_2 , children_2 , rule_2)",
            "Pair :: new (context_2 , children_2 , rule_2)",
            "ContextContainer :: ContextContainer (ContextContainerNode { pair : Pair :: new (context_2 , children_2 , rule_2) })",
            "FeatureContainer :: ConcordClass (ConcordClassNode { marker : * marker }) , * concord_class",
        ] {
            assert!(source.contains(fragment), "missing `{fragment}`: {source}");
        }
    }

    #[test]
    fn raw_context_slots_cannot_bypass_the_build_abi_reservation() {
        let expansion = crate::generate(quote::quote! {
            codec Pair {
                atom = lex;
                value_type = Pair;
                lexical = Lexical::Pair;
                render = render_pair;
                build {
                    pattern = BuildValue::Pair(r#context, r#children, r#rule);
                    construct = Pair::new(r#context, r#children, r#rule);
                }
                traversal {
                    callback = borrowed;
                    argument = pair;
                    call visitor::visit_pair(borrowed(pair));
                }
            }
            construction container: Root {
                element RootNode { pair: lex Pair, }
                form container = lex(pair);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a raw terminal slot cannot capture the parser context ABI");
        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for fragment in [
            "Leaf :: Pair (context_2 , children_2 , rule_2)",
            "Pair :: new (context_2 , children_2 , rule_2)",
            "RootNode { pair : Pair :: new (context_2 , children_2 , rule_2) }",
        ] {
            assert!(source.contains(fragment), "missing `{fragment}`: {source}");
        }
    }

    #[test]
    fn synthetic_projection_build_owns_every_mechanism_and_fallback() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::synthetic_projection_tokens()).unwrap(),
        )
        .unwrap();
        let actual = super::emit(validated.semantic()).unwrap();
        assert_eq!(actual.len(), 2);
        assert_eq!(
            actual[0]
                .origins
                .iter()
                .map(|origin| (origin.kind(), origin.name()))
                .collect::<Vec<_>>(),
            [
                (crate::SourceDeclarationKind::Construction, "leaf"),
                (crate::SourceDeclarationKind::Construction, "nested"),
                (crate::SourceDeclarationKind::Construction, "action"),
                (crate::SourceDeclarationKind::Construction, "idle"),
                (crate::SourceDeclarationKind::Construction, "solo"),
                (crate::SourceDeclarationKind::Construction, "document"),
            ],
        );

        let syn::Item::Fn(function) = syn::parse2(actual[0].tokens.clone()).unwrap() else {
            panic!("build is a function");
        };
        assert_eq!(
            function.sig,
            syn::parse_quote!(fn build_checked(rule: RuleId, children: &[BuildValue], context: &ParseContext<'_>,) -> Result<Option<BuildValue>, BuildRejection>)
        );
        let syn::Stmt::Expr(syn::Expr::Match(dispatch), None) = &function.block.stmts[0] else {
            panic!("flat match dispatch")
        };
        assert_eq!(dispatch.arms.len(), 6);
        let arms = dispatch
            .arms
            .iter()
            .map(ToTokens::to_token_stream)
            .map(|tokens| tokens.to_string())
            .collect::<Vec<_>>();
        assert!(arms.iter().all(|arm| arm.contains("_ => Ok (None)")));

        let joined = arms.join("\n");
        for fragment in [
            "RuleId :: ExprLeaf",
            "Mode :: Solo",
            "ConcordClass :: ThirdPersonSingular",
            "Number :: Singular",
            "Mode :: Group",
            "ConcordClass :: Other",
            "Number :: Plural",
            "RuleId :: ExprNested",
            "NestedNode {",
            "Box :: new (next . clone ())",
            "RuleId :: PredicateAction",
            "ActionStem :: Activate",
            "RuleId :: PredicateIdle",
            "RuleId :: TagSolo",
            "RuleId :: DocumentDocument",
            "BuildValue :: Expr (subject",
            "RuntimePair :: new (left , Factory :: wrap (right))",
            "DocumentNode :: try_new",
        ] {
            assert!(
                joined.contains(fragment),
                "build dispatch lacks `{fragment}`"
            );
        }
        let root_arm = arms
            .iter()
            .find(|arm| arm.contains("RuleId :: DocumentDocument"))
            .expect("root-category construction build arm");
        assert!(!root_arm.contains("Leaf :: Literal"), "{root_arm}");
        assert!(!root_arm.contains("Leaf :: EndOfInput"), "{root_arm}");
        assert!(!joined.contains("vec !"), "{joined}");
    }

    #[test]
    fn bound_prefix_onset_comes_from_the_fixed_realized_prefix() {
        let source = quote::quote! {
            vocab Modifier { Artifact = "artifact", }
            construction prefixed_onset: NounPhrase {
                element PrefixedOnset { modifier: lex Modifier, }
                derive onset = modifier.onset;
                form prefixed_onset = prefix("non", lex(modifier));
            }
            construction wrapper: Phrase {
                element Wrapper { head: NounPhrase, }
                derive onset = head.onset;
                form an when head.onset is Vowel = "an" head;
                form a otherwise = "a" head;
            }
            root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        };
        let raw = crate::parse_declarations(source).expect("the fixed-prefix onset fixture parses");
        let validated =
            crate::validate_declarations(raw).expect("the fixed-prefix onset fixture validates");
        crate::emit::ast::emit(validated.semantic()).expect("fixed-prefix AST emits");
        crate::emit::terminal::emit(validated.semantic()).expect("fixed-prefix terminals emit");
        crate::emit::render::emit(validated.semantic()).expect("fixed-prefix render emits");
        crate::emit::visit::emit(validated.semantic()).expect("fixed-prefix visitor emits");
        crate::emit::rules::emit(validated.semantic()).expect("fixed-prefix rules emit");
        crate::emit::build::emit(validated.semantic()).expect("fixed-prefix build emits");
        let expansion = crate::generate_from_semantic(validated.semantic())
            .expect("a fixed bound prefix overrides its value's vowel onset");
        let source = expansion.tokens().to_string();

        assert!(
            source.contains(
                "BuildValue :: NounPhrase (NounPhrase :: PrefixedOnset (PrefixedOnset { modifier : * modifier }) , Onset :: Consonant , FeatureConstraint :: < Onset > :: Any)"
            ),
            "the prefix's consonantal onset overrides `artifact`: {source}"
        );
    }

    #[test]
    fn punctuation_only_bound_prefix_forwards_the_realized_payload_onset() {
        let source = quote::quote! {
            vocab Modifier { Artifact = "artifact", }
            construction prefixed_onset: NounPhrase {
                element PrefixedOnset { modifier: lex Modifier, }
                derive onset = modifier.onset;
                form prefixed_onset = prefix("+", lex(modifier));
            }
            construction wrapper: Phrase {
                element Wrapper { head: NounPhrase, }
                derive onset = head.onset;
                form an when head.onset is Vowel = "an" head;
                form a otherwise = "a" head;
            }
            root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        };
        let raw = crate::parse_declarations(source).expect("punctuation prefix fixture parses");
        let validated =
            crate::validate_declarations(raw).expect("punctuation prefix fixture validates");
        crate::emit::ast::emit(validated.semantic()).expect("punctuation-prefix AST emits");
        crate::emit::terminal::emit(validated.semantic())
            .expect("punctuation-prefix terminals emit");
        crate::emit::render::emit(validated.semantic()).expect("punctuation-prefix render emits");
        crate::emit::visit::emit(validated.semantic()).expect("punctuation-prefix visitor emits");
        crate::emit::rules::emit(validated.semantic()).expect("punctuation-prefix rules emit");
        crate::emit::build::emit(validated.semantic()).expect("punctuation-prefix build emits");
        let expansion = crate::generate_from_semantic(validated.semantic())
            .expect("a punctuation-only prefix forwards its payload onset");
        let source = expansion.tokens().to_string();

        for expected in [
            "Leaf :: Literal (\"+\")",
            "modifier : * modifier",
            "match modifier { Modifier :: Artifact => Onset :: Vowel",
        ] {
            assert!(
                source.contains(expected),
                "the punctuation prefix must preserve `artifact`'s vowel onset; missing `{expected}`: {source}"
            );
        }
    }
}
