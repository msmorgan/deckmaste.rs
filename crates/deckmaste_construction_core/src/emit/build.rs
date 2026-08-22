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
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::semantic::AtomPlan;
use crate::semantic::AtomTerminal;
use crate::semantic::BindingBuildExprPlan;
use crate::semantic::ConstructionPlan;
use crate::semantic::FiniteDomainKindPlan;
use crate::semantic::FiniteValuePlan;
use crate::semantic::SemanticPlan;
use crate::semantic::StructuralFieldKindPlan;
use crate::semantic::StructuralFieldPlan;
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
                DeclarationKind::Construction,
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
            sequence_states,
        } => emit_arm_from_plan(plan, row, &plan.constructions()[*index], sequence_states),
        super::rules::RuleBuildPlan::Product {
            index,
            sequence_states,
        } => emit_product_arm(plan, row, &plan.products()[*index], sequence_states),
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

fn emit_arm_from_plan(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    row: &ConstructionPlan,
    sequence_states: &[super::rules::SequenceOwnerBuildPlan],
) -> syn::Result<TokenStream> {
    let mut lowering = Lowering::default();
    let form_index = rule
        .form_index
        .ok_or_else(|| internal("construction rule has no form index"))?;
    for atom in row.forms()[form_index].atoms() {
        let state = atom_role(atom)
            .and_then(|role| sequence_states.iter().find(|field| field.role == role));
        if let Some(state) = state {
            let field_role =
                atom_role(atom).ok_or_else(|| internal("sequence atom has no role"))?;
            let field = row
                .fields()
                .iter()
                .find(|field| field.name_key() == field_role)
                .and_then(crate::semantic::ConstructionFieldPlan::structural_plan)
                .ok_or_else(|| internal("positional atom has no structural plan"))?;
            lower_sequence_owner_field(
                plan,
                rule,
                row.element_type(),
                field,
                state,
                &mut lowering,
            )?;
        } else {
            lower_atom(plan, row, atom, &mut lowering)?;
        }
    }
    lower_feature_guards(plan, row, &mut lowering)?;
    if let Some(guard) = super::emit_form_guard_expression(row, form_index, |domain, value| {
        let guard_role = domain.role();
        let field_value = lowering
            .field_values
            .get(guard_role)
            .cloned()
            .ok_or_else(|| internal("form guard role has no lowered build value"))?;
        match (domain.kind(), value) {
            (FiniteDomainKindPlan::Vocab { terminal, .. }, FiniteValuePlan::Vocab(variant)) => {
                let terminal = ident(terminal);
                let variant = ident(variant);
                Ok(quote! { matches!(#field_value, #terminal::#variant) })
            }
            (
                FiniteDomainKindPlan::OptionalPresence,
                FiniteValuePlan::OptionalPresence(present),
            ) => Ok(quote! { #field_value.is_some() == #present }),
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

fn emit_product_arm(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    product: &crate::semantic::ProductPlan,
    sequence_states: &[super::rules::SequenceOwnerBuildPlan],
) -> syn::Result<TokenStream> {
    let mut binders = LocalAllocator::default();
    for name in ["rule", "children", "context"] {
        binders.reserve(name);
    }
    let mut patterns = Vec::new();
    let mut values = Vec::new();
    for field in product.fields() {
        let state = sequence_states
            .iter()
            .find(|state| state.role == field.name());
        let (mut field_patterns, mut value) = if let Some(state) = state {
            lower_sequence_owner_value(plan, rule, product.name(), field, state, &mut binders)?
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
    let value = lower_value(plan, alternative.value(), "value", &mut binders)?;
    let payload = if alternative.is_recursive() {
        let expression = value.expression;
        quote! { Box::new(#expression) }
    } else {
        value.expression
    };
    let sum_type = ident(sum.name());
    let variant = ident(alternative.name());
    let patterns = vec![value.pattern];
    let rule_id = ident(&rule.id);
    Ok(quote! {
        RuleId::#rule_id => match children {
            [#(#patterns),*] => Ok(Some(BuildValue::#sum_type(#sum_type::#variant(#payload)))),
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
    let pattern = value.pattern;
    let expression = value.expression;
    Ok(quote! {
        RuleId::#rule_id => match children {
            [#pattern] => Ok(Some(BuildValue::#carrier(Some(#expression)))),
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
    let lowered = lower_sequence_rhs(
        plan,
        &rule.rhs,
        &carrier,
        helper_category,
        &rule.state,
        &mut binders,
    )?;
    let patterns = lowered.patterns.clone();
    let success = match state {
        super::rules::SequenceBuildState::Singleton => {
            let values = exact_sequence_values(lowered, 1, &rule.state)?;
            quote! { Ok(Some(BuildValue::#carrier(vec![#(#values),*]))) }
        }
        super::rules::SequenceBuildState::Exact(length)
        | super::rules::SequenceBuildState::PositionalExactTail(length) => {
            let values = exact_sequence_values(lowered, length, &rule.state)?;
            quote! { Ok(Some(BuildValue::#carrier(vec![#(#values),*]))) }
        }
        super::rules::SequenceBuildState::Last => {
            let values = exact_sequence_values(lowered, 2, &rule.state)?;
            quote! { Ok(Some(BuildValue::#carrier(vec![#(#values),*]))) }
        }
        super::rules::SequenceBuildState::Recursive | super::rules::SequenceBuildState::Middle => {
            let (values, tail) = prefixed_sequence_values(lowered, 1, &rule.state)?;
            let prefix_len = values.len();
            quote! {{
                let mut values = Vec::with_capacity(#prefix_len + #tail.len());
                #(values.push(#values);)*
                values.extend(#tail.iter().cloned());
                Ok(Some(BuildValue::#carrier(values)))
            }}
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
            let agreement = plan
                .category_carries_agreement(name)
                .then(|| quote! { , _ });
            let number = plan.category_carries_number(name).then(|| quote! { , _ });
            Ok(LoweredValue {
                pattern: quote! { BuildValue::#variant(#binding #agreement #number) },
                expression: quote! { #binding.clone() },
            })
        }
        ValueKindPlan::Product(name) | ValueKindPlan::Sum(name) => {
            let variant = ident(name);
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::#variant(#binding) },
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
        AtomTerminal::ContextIdentity(identity) => {
            let leaf = identity.aggregate_ident();
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(Leaf::#leaf(#binding)) },
                expression: quote! { *#binding },
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
        AtomTerminal::DeclarationNoun(_) => {
            let binding = binders.allocate(preferred);
            Ok(LoweredValue {
                pattern: quote! { BuildValue::Leaf(Leaf::Noun { noun: #binding, number: _ }) },
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

fn lower_sequence_owner_field(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    owner: &str,
    field: &StructuralFieldPlan,
    state: &super::rules::SequenceOwnerBuildPlan,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let (patterns, value) =
        lower_sequence_owner_value(plan, rule, owner, field, state, &mut lowering.binders)?;
    lowering.patterns.extend(patterns);
    lowering.field_values.insert(field.name().to_owned(), value);
    Ok(())
}

fn lower_sequence_owner_value(
    plan: &SemanticPlan,
    rule: &super::rules::RuleRowPlan,
    owner: &str,
    field: &StructuralFieldPlan,
    state: &super::rules::SequenceOwnerBuildPlan,
    binders: &mut LocalAllocator,
) -> syn::Result<(Vec<TokenStream>, TokenStream)> {
    let StructuralFieldKindPlan::Sequence { .. } = field.kind() else {
        return Err(internal(
            "sequence owner state does not name a sequence field",
        ));
    };
    let symbols = rule
        .rhs
        .get(state.rhs_start..state.rhs_end)
        .ok_or_else(|| internal("sequence owner RHS range exceeds the lowered rule"))?;
    let carrier = ident(&carrier_variant(owner, field)?);
    let lowered = lower_sequence_rhs(
        plan,
        symbols,
        &carrier,
        state.helper_category.as_deref(),
        &rule.state,
        binders,
    )?;
    let patterns = lowered.patterns.clone();
    let value = match state.state {
        super::rules::SequenceOwnerState::UniformEmpty
        | super::rules::SequenceOwnerState::PositionalEmpty => {
            let values = exact_sequence_values(lowered, 0, &rule.state)?;
            quote! { vec![#(#values),*] }
        }
        super::rules::SequenceOwnerState::UniformNonEmpty => {
            let (values, tail) = prefixed_sequence_values(lowered, 0, &rule.state)?;
            debug_assert!(values.is_empty());
            quote! { #tail.clone() }
        }
        super::rules::SequenceOwnerState::PositionalSingleton => {
            let values = exact_sequence_values(lowered, 1, &rule.state)?;
            quote! { vec![#(#values),*] }
        }
        super::rules::SequenceOwnerState::PositionalPair => {
            let values = exact_sequence_values(lowered, 2, &rule.state)?;
            quote! { vec![#(#values),*] }
        }
        super::rules::SequenceOwnerState::PositionalThreePlus
        | super::rules::SequenceOwnerState::PositionalMinimumPlus(_) => {
            let (values, tail) = prefixed_sequence_values(lowered, 1, &rule.state)?;
            let prefix_len = values.len();
            quote! {{
                let mut values = Vec::with_capacity(#prefix_len + #tail.len());
                #(values.push(#values);)*
                values.extend(#tail.iter().cloned());
                values
            }}
        }
    };
    Ok((patterns, value))
}

struct LoweredSequenceRhs {
    patterns: Vec<TokenStream>,
    values: Vec<TokenStream>,
    tail: Option<syn::Ident>,
}

fn lower_sequence_rhs(
    plan: &SemanticPlan,
    symbols: &[super::rules::RuleSymbolPlan],
    carrier: &syn::Ident,
    helper_category: Option<&str>,
    state: &str,
    binders: &mut LocalAllocator,
) -> syn::Result<LoweredSequenceRhs> {
    let mut patterns = Vec::new();
    let mut values = Vec::new();
    let mut tail = None;
    for (index, symbol) in symbols.iter().enumerate() {
        match symbol {
            super::rules::RuleSymbolPlan::Value(value) => {
                let value = lower_value(plan, value, &format!("item_{index}"), binders)?;
                patterns.push(value.pattern);
                values.push(value.expression);
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
                patterns.push(quote! { BuildValue::#carrier(#binding) });
                tail = Some(binding);
            }
            super::rules::RuleSymbolPlan::Surface(surface) => {
                patterns.push(fixed_surface_pattern(plan, &surface.atom)?);
            }
            super::rules::RuleSymbolPlan::Authored { .. } => {
                return Err(internal(&format!(
                    "{state} sequence RHS contains an authored atom"
                )));
            }
        }
    }
    Ok(LoweredSequenceRhs {
        patterns,
        values,
        tail,
    })
}

fn exact_sequence_values(
    lowered: LoweredSequenceRhs,
    expected: usize,
    state: &str,
) -> syn::Result<Vec<TokenStream>> {
    if lowered.tail.is_some() || lowered.values.len() != expected {
        return Err(internal(&format!(
            "{state} sequence RHS must contain exactly {expected} values and no tail"
        )));
    }
    Ok(lowered.values)
}

fn prefixed_sequence_values(
    lowered: LoweredSequenceRhs,
    expected: usize,
    state: &str,
) -> syn::Result<(Vec<TokenStream>, syn::Ident)> {
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
    Ok((lowered.values, tail))
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
        _ => Err(internal(
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
    match atom {
        AtomPlan::Category { role, .. }
        | AtomPlan::Lex { role, .. }
        | AtomPlan::Identity { role, .. }
        | AtomPlan::Noun { role, .. } => Some(role),
        AtomPlan::Literal(_) | AtomPlan::VerbFixed { .. } | AtomPlan::OpenDeclaration(_) => None,
    }
}

fn lower_atom(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
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
            StructuralFieldKindPlan::Optional(_) => {
                let variant = ident(&carrier_variant(row.element_type(), structural)?);
                let binding = lowering.binders.allocate(role);
                lowering
                    .patterns
                    .push(quote! { BuildValue::#variant(#binding) });
                lowering
                    .field_values
                    .insert(role.to_owned(), quote! { #binding.clone() });
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
                lowering
                    .patterns
                    .push(quote! { BuildValue::#variant(#binding) });
                lowering
                    .field_values
                    .insert(role.to_owned(), quote! { #binding.clone() });
                return Ok(());
            }
            StructuralFieldKindPlan::Required(_) => {}
        }
    }
    match atom {
        AtomPlan::Literal(literal) => {
            let literal = syn::LitStr::new(literal, Span::call_site());
            lowering
                .patterns
                .push(quote! { BuildValue::Leaf(Leaf::Literal(#literal)) });
        }
        AtomPlan::Category { role, category } => {
            lower_category_role(validated, row, role, category, lowering)?;
        }
        AtomPlan::Lex { role, terminal } | AtomPlan::Identity { role, terminal } => {
            lower_terminal_role(validated, row, role, terminal, false, lowering)?;
        }
        AtomPlan::Noun { role, terminal } => {
            lower_terminal_role(validated, row, role, terminal, true, lowering)?;
        }
        AtomPlan::VerbFixed {
            terminal, variant, ..
        } => {
            let agreement = verb_agreement_pattern(validated, row, lowering)?;
            let agreement_field = if agreement.to_string() == "agreement" {
                quote! { agreement }
            } else {
                quote! { agreement: #agreement }
            };
            let terminal = ident(terminal);
            let variant = ident(variant);
            lowering.patterns.push(quote! {
                BuildValue::Leaf(Leaf::Verb { lexeme: #terminal::#variant, #agreement_field })
            });
        }
        AtomPlan::OpenDeclaration(open) => {
            let declaration = lowering.binders.allocate("declaration");
            let surface_feature = lowering.binders.allocate("surface_feature");
            lowering.patterns.push(quote! {
                BuildValue::Leaf(Leaf::Declaration(DeclarationLeaf {
                    id: #declaration,
                    feature: #surface_feature,
                }))
            });
            let kind = crate::emit::declaration_kind(open.kind());
            let name = syn::LitStr::new(open.name(), Span::call_site());
            lowering.guards.push(quote! {
                #declaration.kind() == #kind && #declaration.name() == #name
            });
            lowering.guards.push(quote! {
                matches!(
                    #surface_feature,
                    ::macro_ron::v2::SurfaceFeature::Bare
                        | ::macro_ron::v2::SurfaceFeature::ThirdPersonSingular
                )
            });
            lowering.role_features.insert(
                ("verb".to_owned(), Feature::Agreement),
                LocalFeatureValue::Computed(quote! {
                    match #surface_feature {
                        ::macro_ron::v2::SurfaceFeature::Bare => Agreement::Bare,
                        ::macro_ron::v2::SurfaceFeature::ThirdPersonSingular => {
                            Agreement::ThirdPersonSingular
                        }
                        _ => unreachable!("open verb matcher admitted a non-verb feature"),
                    }
                }),
            );
        }
    }
    Ok(())
}

fn lower_category_role(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    role: &str,
    category_name: &str,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let role = ident(role);
    let role_name = identifier_key(&role);
    let role_binding = lowering.binders.allocate_ident(&role);
    let category = ident(category_name);
    lowering
        .field_values
        .insert(role_name.clone(), quote! { #role_binding.clone() });

    let carries_agreement = validated.category_carries_agreement(category_name);
    let carries_number = validated.category_carries_number(category_name);
    let agreement = carries_agreement
        .then(|| role_agreement_pattern(validated, row, &role, category_name, lowering))
        .transpose()?;
    let number = carries_number.then(|| role_number_pattern(validated, row, &role, lowering));
    match (agreement, number) {
        (Some(agreement), Some(number)) => lowering
            .patterns
            .push(quote! { BuildValue::#category(#role_binding, #agreement, #number) }),
        (Some(pattern), None) => {
            lowering
                .patterns
                .push(quote! { BuildValue::#category(#role_binding, #pattern) });
        }
        (None, Some(number)) => lowering
            .patterns
            .push(quote! { BuildValue::#category(#role_binding, #number) }),
        (None, None) => lowering
            .patterns
            .push(quote! { BuildValue::#category(#role_binding) }),
    }
    Ok(())
}

fn role_agreement_pattern(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    role: &syn::Ident,
    category: &str,
    lowering: &mut Lowering,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: role.clone(),
        feature: Feature::Agreement,
    };
    if let Some(crate::feature::FeatureResolution::Known(value)) =
        validated.feature_resolution(row.construction_id(), &target)
    {
        lowering.role_features.insert(
            (identifier_key(role), Feature::Agreement),
            LocalFeatureValue::Known(value),
        );
        return Ok(feature_value(value));
    }
    if let Some(equation) = equation(validated, row, &target) {
        if matches!(equation.value(), FeatureExpr::MatchVocab { .. }) {
            return Err(internal("category role feature cannot be a vocab match"));
        }
        let stem = snake_case(category).trim_end_matches("_phrase").to_owned();
        let name = lowering.binders.allocate(&format!("{stem}_agreement"));
        lowering.role_features.insert(
            (identifier_key(role), Feature::Agreement),
            LocalFeatureValue::Bound(name.clone()),
        );
        return Ok(quote! { #name });
    }
    if feature_is_read(validated, row, role, Feature::Agreement) {
        let name = lowering
            .binders
            .allocate(&format!("{}_agreement", identifier_key(role)));
        lowering.role_features.insert(
            (identifier_key(role), Feature::Agreement),
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
    role: &syn::Ident,
    lowering: &mut Lowering,
) -> TokenStream {
    if role_number_is_needed_in_build(validated, row, role) {
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

fn lower_terminal_role(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
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
            lowering
                .patterns
                .push(quote! { BuildValue::Leaf(Leaf::#leaf(#binding)) });
            lowering
                .field_values
                .insert(identifier_key(&role), quote! { *#binding });
            lowering
                .vocab_values
                .insert(identifier_key(&role), binding.clone());
            return Ok(());
        }
        AtomTerminal::Binding(binding) => binding,
        AtomTerminal::ContextIdentity(identity) => {
            let variant = identity.aggregate_ident();
            let value = lowering.binders.allocate(&identifier_key(&role));
            lowering
                .patterns
                .push(quote! { BuildValue::Leaf(Leaf::#variant(#value)) });
            lowering
                .field_values
                .insert(identifier_key(&role), quote! { *#value });
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
        AtomTerminal::DeclarationNoun(_) => {
            let value = lowering.binders.allocate(&identifier_key(&role));
            let number = noun_number_pattern(validated, row, &role, lowering)?;
            let number_field = if number.to_string() == "number" {
                quote! { number }
            } else {
                quote! { number: #number }
            };
            lowering.patterns.push(quote! {
                BuildValue::Leaf(Leaf::Noun { noun: #value, #number_field })
            });
            lowering
                .field_values
                .insert(identifier_key(&role), quote! { #value.clone() });
            return Ok(());
        }
    };
    let build = binding
        .build()
        .ok_or_else(|| internal("atom-capable binding has no build metadata"))?;
    let variant = build.variant();
    let names = build.slots();
    let noun_count = row
        .atoms()
        .iter()
        .filter(|atom| matches!(atom, AtomPlan::Noun { .. }))
        .count();
    let preferred_names = if noun && noun_count > 1 {
        names
            .iter()
            .map(|name| {
                if names.len() == 1 {
                    identifier_key(&role)
                } else {
                    format!("{}_{}", identifier_key(&role), identifier_key(name))
                }
            })
            .collect::<Vec<_>>()
    } else if noun || names.len() != 1 {
        names.iter().map(identifier_key).collect()
    } else {
        vec![identifier_key(&role)]
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
        let number = noun_number_pattern(validated, row, &role, lowering)?;
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
        quote! { Leaf::Noun { noun: #noun_value, #number_field } }
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
    lowering.field_values.insert(identifier_key(&role), stored);
    Ok(())
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
    let target = FeaturePlace::Construction(Feature::Number);
    if let Some(crate::feature::FeatureResolution::Known(value)) =
        validated.feature_resolution(row.construction_id(), &target)
    {
        return Ok(feature_value(value));
    }
    let equation = equation(validated, row, &target)
        .ok_or_else(|| internal("noun atom has no construction number"))?;
    match equation.value() {
        FeatureExpr::Constant(value) => Ok(feature_value(*value.value())),
        FeatureExpr::MatchVocab { .. } | FeatureExpr::FromRole { .. } => {
            let preferred = if lowering.dynamic_numbers.is_empty() {
                "number".to_owned()
            } else {
                format!("{}_number", identifier_key(role))
            };
            let number = lowering.binders.allocate(&preferred);
            lowering.dynamic_numbers.push(number.clone());
            Ok(quote! { #number })
        }
    }
}

fn verb_agreement_pattern(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &mut Lowering,
) -> syn::Result<TokenStream> {
    let verb = ident("verb");
    let target = FeaturePlace::Role {
        field: verb,
        feature: Feature::Agreement,
    };
    if let Some(crate::feature::FeatureResolution::Known(value)) =
        validated.feature_resolution(row.construction_id(), &target)
    {
        lowering.role_features.insert(
            ("verb".to_owned(), Feature::Agreement),
            LocalFeatureValue::Known(value),
        );
        return Ok(feature_value(value));
    }
    if matches!(
        equation(validated, row, &target).map(crate::feature::FeatureEquation::value),
        Some(FeatureExpr::MatchVocab { .. })
    ) {
        return Err(internal("verb agreement cannot be a vocab match"));
    }
    let agreement = lowering.binders.allocate("agreement");
    lowering.role_features.insert(
        ("verb".to_owned(), Feature::Agreement),
        LocalFeatureValue::Bound(agreement.clone()),
    );
    Ok(quote! { #agreement })
}

fn lower_feature_guards(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
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
            if row
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
    agreement_override: Option<FeatureValue>,
    number_override: Option<FeatureValue>,
) -> syn::Result<TokenStream> {
    let element = ident(row.element_type());
    let category = ident(row.category());
    let variant = ident(row.category_variant());
    if !row.fields().is_empty() && row.requires_constructor() {
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
            agreement_override,
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
    let category_value = quote! { #category::#variant(#element_value) };
    let carries_agreement = validated.category_carries_agreement(row.category());
    let carries_number = validated.category_carries_number(row.category());
    let wrapped = if carries_agreement && carries_number {
        let agreement = construction_agreement(validated, row, lowering, agreement_override)?;
        let number = construction_number(validated, row, lowering, number_override)?;
        quote! { BuildValue::#category(#category_value, #agreement, #number) }
    } else if carries_agreement {
        let agreement = construction_agreement(validated, row, lowering, agreement_override)?;
        quote! { BuildValue::#category(#category_value, #agreement) }
    } else if carries_number {
        let number = construction_number(validated, row, lowering, number_override)?;
        quote! { BuildValue::#category(#category_value, #number) }
    } else {
        quote! { BuildValue::#category(#category_value) }
    };
    Ok(quote! { Ok(Some(#wrapped)) })
}

fn emit_fallible_element_success(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &mut Lowering,
    result: &TokenStream,
    agreement_override: Option<FeatureValue>,
    number_override: Option<FeatureValue>,
) -> syn::Result<TokenStream> {
    let category = ident(row.category());
    let variant = ident(row.category_variant());
    let mapped = quote! { #category::#variant };
    let carries_agreement = validated.category_carries_agreement(row.category());
    let carries_number = validated.category_carries_number(row.category());
    if !carries_agreement && !carries_number {
        return Ok(quote! {
            #result.map(#mapped).map(BuildValue::#category).map(Some)
        });
    }

    let agreement = carries_agreement
        .then(|| construction_agreement(validated, row, lowering, agreement_override))
        .transpose()?;
    let number = carries_number
        .then(|| construction_number(validated, row, lowering, number_override))
        .transpose()?;
    let argument = lowering.constructor_map_local.clone().unwrap_or_else(|| {
        let argument = lowering.binders.allocate(row.construction_id());
        lowering.constructor_map_local = Some(argument.clone());
        argument
    });
    if let (Some(agreement), Some(number)) = (&agreement, &number) {
        return Ok(quote! {
            #result
                .map(|#argument| { BuildValue::#category(#category::#variant(#argument), #agreement, #number) })
                .map(Some)
        });
    }
    if let Some(number) = number {
        return Ok(quote! {
            #result
                .map(|#argument| { BuildValue::#category(#category::#variant(#argument), #number) })
                .map(Some)
        });
    }
    let output = agreement
        .ok_or_else(|| internal("fallible construction is missing its carried feature output"))?;
    Ok(quote! {
        #result
            .map(|#argument| { BuildValue::#category(#category::#variant(#argument), #output) })
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
    let agreement_arms = equation(
        validated,
        row,
        &FeaturePlace::Construction(Feature::Agreement),
    )
    .and_then(|equation| match equation.value() {
        FeatureExpr::MatchVocab { arms, .. } => Some(arms.as_slice()),
        FeatureExpr::Constant(_) | FeatureExpr::FromRole { .. } => None,
    });
    let driving_arms = number_arms
        .or(agreement_arms)
        .ok_or_else(|| internal("dynamic feature match has no vocab arms"))?;
    let ty = ident(terminal_for_role(row, role)?);
    let mut arms = Vec::new();
    for (variant, _) in driving_arms {
        let variant = variant.value();
        let mut overrides = HashMap::new();
        overrides.insert(identifier_key(role), quote! { #ty::#variant });
        let agreement_value = agreement_arms.and_then(|arms| {
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
            agreement_value,
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
    if validated
        .boxed_fields()
        .contains(&(row.construction_id().to_owned(), identifier_key(role)))
    {
        value = quote! { Box::new(#value) };
    }
    Ok(value)
}

fn construction_agreement(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    lowering: &Lowering,
    agreement_override: Option<FeatureValue>,
) -> syn::Result<TokenStream> {
    if let Some(agreement) = agreement_override {
        return Ok(feature_value(agreement));
    }
    let output = resolve_feature_place(
        validated,
        row,
        lowering,
        &FeaturePlace::Construction(Feature::Agreement),
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
    let equation = equation(validated, row, place)
        .ok_or_else(|| internal("accepted feature source has no symbolic binding"))?;
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
                FeaturePlace::Construction(Feature::Agreement)
                | FeaturePlace::Role {
                    feature: Feature::Agreement,
                    ..
                } => {
                    let helper = ident(&feature_helper("agreement", terminal_for_role(row, role)?));
                    ResolvedFeatureValue::Computed(quote! { #helper(#source) })
                }
                FeaturePlace::Construction(Feature::Number)
                | FeaturePlace::Role {
                    feature: Feature::Number,
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
    [Feature::Number, Feature::Agreement]
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
    })
}

fn role_number_is_needed_in_build(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    role: &syn::Ident,
) -> bool {
    let output_needs_number = row
        .atoms()
        .iter()
        .any(|atom| matches!(atom, AtomPlan::Noun { .. }))
        || validated.category_carries_number(row.category());
    output_needs_number
        && matches!(
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
    row.atoms()
        .iter()
        .find_map(|atom| match atom {
            AtomPlan::Lex {
                role: found,
                terminal,
            }
            | AtomPlan::Identity {
                role: found,
                terminal,
            }
            | AtomPlan::Noun {
                role: found,
                terminal,
            } if found == &identifier_key(role) => Some(terminal.as_str()),
            AtomPlan::Literal(_)
            | AtomPlan::Category { .. }
            | AtomPlan::VerbFixed { .. }
            | AtomPlan::OpenDeclaration(_)
            | AtomPlan::Lex { .. }
            | AtomPlan::Identity { .. }
            | AtomPlan::Noun { .. } => None,
        })
        .ok_or_else(|| internal("resolved terminal role is absent from the semantic plan"))
}
fn feature_value(value: FeatureValue) -> TokenStream {
    match value {
        FeatureValue::Bare => quote! { Agreement::Bare },
        FeatureValue::ThirdPersonSingular => quote! { Agreement::ThirdPersonSingular },
        FeatureValue::Singular => quote! { Number::Singular },
        FeatureValue::Plural => quote! { Number::Plural },
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
                        [
                            BuildValue::Child(child),
                            BuildValue::Leaf(Leaf::Mode(mode))
                        ] => RecursiveNode::try_new(Box::new(child.clone()), *mode)
                            .map(Child::Recursive)
                            .map(BuildValue::Child)
                            .map(Some),
                        _ => Ok(None),
                    }
                },
            ),
            (
                "RootCategoryGuarded",
                syn::parse_quote! {
                    RuleId::RootCategoryGuarded => match children {
                        [
                            BuildValue::Child(child)
                        ] => CategoryGuarded::try_new(child.clone())
                            .map(Root::CategoryGuarded)
                            .map(BuildValue::Root)
                            .map(Some),
                        _ => Ok(None),
                    }
                },
            ),
            (
                "RootVocabGuarded",
                syn::parse_quote! {
                    RuleId::RootVocabGuarded => match children {
                        [
                            BuildValue::Leaf(Leaf::Mode(mode))
                        ] => VocabGuarded::try_new(*mode)
                            .map(Root::VocabGuarded)
                            .map(BuildValue::Root)
                            .map(Some),
                        _ => Ok(None),
                    }
                },
            ),
            (
                "RootDnfGuarded",
                syn::parse_quote! {
                    RuleId::RootDnfGuarded => match children {
                        [
                            BuildValue::Leaf(Leaf::Mode(mode)),
                            BuildValue::Child(child)
                        ] => DnfGuarded::try_new(*mode, child.clone())
                            .map(Root::DnfGuarded)
                            .map(BuildValue::Root)
                            .map(Some),
                        _ => Ok(None),
                    }
                },
            ),
            (
                "RootContextGuarded",
                syn::parse_quote! {
                    RuleId::RootContextGuarded => match children {
                        [
                            BuildValue::Leaf(Leaf::SelfReference(context_2))
                        ] => ContextGuarded::try_new(*context_2, context)
                            .map(Root::ContextGuarded)
                            .map(BuildValue::Root)
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
            for forbidden in [
                "Child :: First",
                "Child :: Second",
                "Mode :: One",
                "Mode :: Two",
                "matches !",
            ] {
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

            mod generated {
                use super::*;
                #(#runtime)*
                #(#ast)*
                #(#build)*
            }

            #[derive(Debug)]
            enum BuildValue {
                Child(generated::Child),
                Root(generated::Root),
                Leaf(Leaf),
            }

            fn main() {
                let context = ParseContext {
                    canonical: SelfReferenceSpelling::Full,
                    marker: "materialization",
                };
                let valid = [
                    BuildValue::Child(generated::Child::First(generated::FirstChild)),
                ];
                let invalid = [
                    BuildValue::Child(generated::Child::Second(generated::SecondChild)),
                ];

                assert!(matches!(
                    generated::build(RuleId::RootCategoryGuarded, &valid, &context),
                    Some(BuildValue::Root(generated::Root::CategoryGuarded(_)))
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
            source.contains("Leaf :: Noun { noun : head , number }"),
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
    fn agreement_match_with_constant_number_uses_vocab_only_dispatch() {
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
                    derive agreement = match count {
                        One => Values::ThirdPersonSingular,
                        Many => Values::Bare,
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
            .expect("agreement-only vocab dispatch lowers")
            .remove(0)
            .tokens
            .to_string();
        assert!(
            source.contains("Leaf :: Noun { noun : head , number : Number :: Singular }")
                && source.contains("match count")
                && source.contains("Count :: One => Ok (Some")
                && source.contains("Agreement :: ThirdPersonSingular")
                && source.contains("Count :: Many => Ok (Some")
                && source.contains("Agreement :: Bare"),
            "agreement matching is vocab-only while noun number stays constant: {source}"
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
            "Leaf :: Noun { noun : left , number }",
            "Leaf :: Noun { noun : right , number : right_number }",
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
                derive agreement = Values::Bare;
                form bare = "bare";
            }
            construction third: Child {
                element ThirdChild {}
                derive agreement = Values::ThirdPersonSingular;
                form third = "third";
            }
            construction parent: Parent {
                element ParentNode { mode: lex Mode, child: Child, }
                require mode is One;
                derive mode.agreement = match mode {
                    One => Values::ThirdPersonSingular,
                    Many => Values::Bare,
                };
                derive child.agreement = mode.agreement;
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
            source.contains("BuildValue :: Child (child , Agreement :: ThirdPersonSingular)"),
            "the required Mode::One arm must constrain the child agreement: {source}"
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
                })))),
                _ => Ok(None),
            }
        };
        assert_eq!(arm, expected);
    }

    #[test]
    fn allocator_reserves_abi_and_feature_locals() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Marker { One = "one", }
                morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
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
                construction agreement: FeatureContainer {
                    element AgreementNode { marker: lex Marker, }
                    derive agreement = verb.agreement;
                    form agreement = lex(marker) verb(Verbs::Act);
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
            "FeatureContainer :: Agreement (AgreementNode { marker : * marker }) , * agreement",
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
                (crate::DeclarationKind::Construction, "leaf"),
                (crate::DeclarationKind::Construction, "nested"),
                (crate::DeclarationKind::Construction, "action"),
                (crate::DeclarationKind::Construction, "idle"),
                (crate::DeclarationKind::Construction, "solo"),
                (crate::DeclarationKind::Construction, "document"),
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
            "Agreement :: ThirdPersonSingular",
            "Number :: Singular",
            "Mode :: Group",
            "Agreement :: Bare",
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
}
