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
use crate::semantic::ConstructorArgumentPlan;
use crate::semantic::SemanticPlan;

pub(crate) fn emit(plan: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let build_function = ident(BUILD_FUNCTION);
    let constructions = plan.constructions();
    let arms = constructions
        .iter()
        .map(|construction| emit_arm(plan, construction))
        .collect::<syn::Result<Vec<_>>>()?;
    let mut origins = constructions
        .iter()
        .map(|construction| {
            DeclarationKey::new(
                DeclarationKind::Construction,
                construction.construction_id(),
            )
        })
        .collect::<Vec<_>>();
    origins.extend(
        plan.roots()
            .iter()
            .filter(|root| root.is_parse_entry())
            .map(|root| DeclarationKey::new(DeclarationKind::Root, root.category())),
    );
    let tokens = quote! {
        #[expect(
            clippy::too_many_lines,
            reason = "the exhaustive generated-shape construction dispatch is intentionally flat"
        )]
        pub(super) fn #build_function(
            rule: RuleId,
            children: &[BuildValue],
            context: &ParseContext<'_>,
        ) -> Option<BuildValue> {
            match rule { #(#arms)* }
        }
    };
    Ok(vec![GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: BUILD_FUNCTION.to_owned(),
        },
        tokens,
        origins,
    )])
}

struct Lowering {
    patterns: Vec<TokenStream>,
    field_values: HashMap<String, TokenStream>,
    vocab_values: HashMap<String, syn::Ident>,
    role_features: HashMap<(String, Feature), LocalFeatureValue>,
    guards: Vec<TokenStream>,
    dynamic_numbers: Vec<syn::Ident>,
    checked_map_local: Option<syn::Ident>,
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
            checked_map_local: None,
            binders,
        }
    }
}

#[derive(Debug, Clone)]
enum LocalFeatureValue {
    Known(FeatureValue),
    Bound(syn::Ident),
}

#[derive(Clone)]
enum ResolvedFeatureValue {
    Known(FeatureValue),
    Bound(syn::Ident),
    Computed(TokenStream),
}

fn emit_arm(plan: &SemanticPlan, row: &ConstructionPlan) -> syn::Result<TokenStream> {
    emit_arm_from_plan(plan, row)
}

fn emit_arm_from_plan(plan: &SemanticPlan, row: &ConstructionPlan) -> syn::Result<TokenStream> {
    let mut lowering = Lowering::default();
    for atom in row.atoms() {
        lower_atom(plan, row, atom, &mut lowering)?;
    }
    if let Some(root) = plan.parse_root(row.category()) {
        let punctuation = syn::LitStr::new(root.punctuation(), Span::call_site());
        lowering
            .patterns
            .push(quote! { BuildValue::Leaf(Leaf::Literal(#punctuation)) });
        lowering
            .patterns
            .push(quote! { BuildValue::Leaf(Leaf::EndOfInput) });
    }
    lower_feature_guards(plan, row, &mut lowering)?;
    let success = if let Some(dynamic_role) = dynamic_match_role(plan, row) {
        emit_dynamic_match(plan, row, &mut lowering, &dynamic_role)?
    } else {
        emit_success(plan, row, &mut lowering, None, None, None)?
    };
    let rule_id = ident(row.build_arm());
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
            _ => None,
        },
    })
}

fn lower_atom(
    validated: &SemanticPlan,
    row: &ConstructionPlan,
    atom: &AtomPlan,
    lowering: &mut Lowering,
) -> syn::Result<()> {
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
    let refinement = row
        .refinements()
        .iter()
        .find(|requirement| identifier_key(requirement.role()) == role_name);
    let category = ident(category_name);
    let value_pattern = if let Some(requirement) = refinement {
        let variant = ident(&identifier_key(requirement.variant()));
        quote! { #category::#variant(#role_binding) }
    } else {
        quote! { #role_binding }
    };
    let stored = if let Some(requirement) = refinement {
        let variant = ident(&identifier_key(requirement.variant()));
        quote! { #category::#variant(#role_binding.clone()) }
    } else {
        quote! { #role_binding.clone() }
    };
    lowering.field_values.insert(role_name.clone(), stored);

    let carries_agreement = validated.category_carries_agreement(category_name);
    let carries_number = validated.category_carries_number(category_name);
    let agreement = carries_agreement
        .then(|| role_agreement_pattern(validated, row, &role, category_name, lowering))
        .transpose()?;
    let number = carries_number.then(|| role_number_pattern(validated, row, &role, lowering));
    match (agreement, number) {
        (Some(agreement), Some(number)) => lowering
            .patterns
            .push(quote! { BuildValue::#category(#value_pattern, #agreement, #number) }),
        (Some(pattern), None) => {
            lowering
                .patterns
                .push(quote! { BuildValue::#category(#value_pattern, #pattern) });
        }
        (None, Some(number)) => lowering
            .patterns
            .push(quote! { BuildValue::#category(#value_pattern, #number) }),
        (None, None) => lowering
            .patterns
            .push(quote! { BuildValue::#category(#value_pattern) }),
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
            if let Some(requirement) = row
                .refinements()
                .iter()
                .find(|item| identifier_key(item.role()) == identifier_key(&role))
            {
                let variant = ident(&identifier_key(requirement.variant()));
                lowering
                    .patterns
                    .push(quote! { BuildValue::Leaf(Leaf::#leaf(#leaf::#variant)) });
                lowering
                    .field_values
                    .insert(identifier_key(&role), quote! { #leaf::#variant });
            } else {
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
            }
            return Ok(());
        }
        AtomTerminal::Binding(binding) => binding,
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
    let element_value = if let Some(checked) = row.constructor() {
        let path = checked.path();
        let arguments = checked
            .arguments()
            .iter()
            .map(|argument| match argument {
                ConstructorArgumentPlan::Role(role) => {
                    stored_value(validated, row, lowering, role, overrides)
                }
                ConstructorArgumentPlan::VecRole(role) => {
                    stored_value(validated, row, lowering, role, overrides)
                        .map(|value| quote! { vec![#value] })
                }
                ConstructorArgumentPlan::Context => Ok(quote! { context }),
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let mapped = quote! { #category::#variant };
        let carries_agreement = validated.category_carries_agreement(row.category());
        let carries_number = validated.category_carries_number(row.category());
        if carries_agreement || carries_number {
            let agreement = carries_agreement
                .then(|| construction_agreement(validated, row, lowering, agreement_override))
                .transpose()?;
            let number = carries_number
                .then(|| construction_number(validated, row, lowering, number_override))
                .transpose()?;
            let argument = lowering.checked_map_local.clone().unwrap_or_else(|| {
                let argument = lowering.binders.allocate(row.construction_id());
                lowering.checked_map_local = Some(argument.clone());
                argument
            });
            if let (Some(agreement), Some(number)) = (&agreement, &number) {
                return Ok(quote! {
                    #path(#(#arguments),*).map(|#argument| { BuildValue::#category(#category::#variant(#argument), #agreement, #number) })
                });
            }
            if let Some(number) = number {
                return Ok(quote! {
                    #path(#(#arguments),*).map(|#argument| { BuildValue::#category(#category::#variant(#argument), #number) })
                });
            }
            let output = agreement.ok_or_else(|| {
                internal("checked construction is missing its carried feature output")
            })?;
            return Ok(quote! {
                #path(#(#arguments),*).map(|#argument| { BuildValue::#category(#category::#variant(#argument), #output) })
            });
        }
        return Ok(quote! { #path(#(#arguments),*).map(#mapped).map(BuildValue::#category) });
    } else if row.fields().is_empty() {
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
    Ok(quote! { Some(#wrapped) })
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
        Ok(quote! { match (#role_value, #(#numbers),*) { #(#arms,)* _ => None, } })
    } else {
        Ok(quote! { match #role_value { #(#arms,)* _ => None, } })
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

    impl<'ast> Visit<'ast> for Binders {
        fn visit_pat_ident(&mut self, pattern: &'ast syn::PatIdent) {
            self.0.push(pattern.ident.to_string());
            syn::visit::visit_pat_ident(self, pattern);
        }
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
            source.contains("_ => None"),
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
            source.contains("Count :: One => Some") && source.contains("Count :: Many => Some"),
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
        assert!(source.contains("_ => None"), "{source}");
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
                && source.contains("Count :: One => Some")
                && source.contains("Agreement :: ThirdPersonSingular")
                && source.contains("Count :: Many => Some")
                && source.contains("Agreement :: Bare"),
            "agreement matching is vocab-only while noun number stays constant: {source}"
        );
        assert!(source.contains("_ => None"), "{source}");
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
            "_ => None",
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
        assert!(source.contains("_ => None"), "{source}");
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
    fn binding_build_construct_consumes_every_declared_pattern_slot() {
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
                    BuildValue::Leaf(Leaf::Pair(BoundLeaf::Pair(left, right))),
                    BuildValue::Leaf(Leaf::Literal("!")),
                    BuildValue::Leaf(Leaf::EndOfInput)
                ] => Some(BuildValue::Root(Root::Wrapped(Wrapped {
                    value: Pair::new(left.code, (Factory::wrap(right)))
                }))),
                _ => None,
            }
        };
        assert_eq!(arm, expected);
    }

    #[test]
    fn task_11_allocator_reserves_abi_and_checked_feature_locals() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Marker { One = "one", }
                lexeme Verbs { Act, }
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
                construction checked_context: CheckedContext {
                    element CheckedContextNode { pair: lex Pair, }
                    checked {
                        visibility pair = pub(crate);
                        constructor = CheckedContextNode::new(pair, context);
                    }
                    form checked_context = lex(pair);
                }
                construction agreement: FeatureChecked {
                    element AgreementNode { marker: lex Marker, }
                    checked {
                        visibility marker = pub(crate);
                        constructor = AgreementNode::new(marker);
                    }
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
            "CheckedContextNode :: new (Pair :: new (context_2 , children_2 , rule_2) , context)",
            "map (| agreement_2 |",
            "FeatureChecked :: Agreement (agreement_2) , * agreement",
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
            construction checked: Root {
                element RootNode { pair: lex Pair, }
                checked {
                    visibility pair = pub(crate);
                    constructor = RootNode::new(pair, context);
                }
                form checked = lex(pair);
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
            "RootNode :: new (Pair :: new (context_2 , children_2 , rule_2) , context)",
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
        assert_eq!(actual.len(), 1);
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
                (crate::DeclarationKind::Root, "Document"),
            ],
        );

        let syn::Item::Fn(function) = syn::parse2(actual[0].tokens.clone()).unwrap() else {
            panic!("build is a function");
        };
        assert_eq!(
            function.sig,
            syn::parse_quote!(fn build(rule: RuleId, children: &[BuildValue], context: &ParseContext<'_>,) -> Option<BuildValue>)
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
        assert!(arms.iter().all(|arm| arm.contains("_ => None")));

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
            "NestedNode :: checked",
            "Box :: new (next . clone ())",
            "RuleId :: PredicateAction",
            "ActionStem :: Activate",
            "RuleId :: PredicateIdle",
            "RuleId :: TagSolo",
            "RuleId :: DocumentDocument",
            "Expr :: Leaf",
            "RuntimePair :: new (left , Factory :: wrap (right))",
            "DocumentNode :: checked",
            "vec ! [RuntimePair :: new",
            r#"Leaf :: Literal ("!")"#,
            "Leaf :: EndOfInput",
        ] {
            assert!(
                joined.contains(fragment),
                "build dispatch lacks `{fragment}`"
            );
        }
    }
}
