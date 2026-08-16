use std::collections::HashMap;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::ValidatedDeclarations;
use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::feature::FeatureValue;
use crate::model::ConstructorArgument;
use crate::model::Declaration;
use crate::model::FieldKind;
use crate::model::FormAtom;
use crate::model::TerminalBindingKind;
use crate::model::VerbOperand;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;

pub(crate) fn emit(validated: &ValidatedDeclarations) -> syn::Result<Vec<GeneratedItem>> {
    let constructions = validated
        .raw()
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(construction) => Some(construction),
            _ => None,
        })
        .collect::<Vec<_>>();
    let records = validated.contributions().constructions();
    if constructions.len() != records.len() {
        return Err(internal("validated build inventory is inconsistent"));
    }
    let arms = constructions
        .iter()
        .zip(records)
        .map(|(construction, record)| emit_arm(validated, construction, record.rule_id()))
        .collect::<syn::Result<Vec<_>>>()?;
    let mut origins = constructions
        .iter()
        .map(|construction| {
            DeclarationKey::new(DeclarationKind::Construction, construction.name.to_string())
        })
        .collect::<Vec<_>>();
    origins.extend(
        validated
            .raw()
            .declarations
            .iter()
            .filter_map(|declaration| {
                let Declaration::Root(root) = declaration else { return None };
                root.eoi
                    .then(|| DeclarationKey::new(DeclarationKind::Root, path_name(&root.category)))
            }),
    );
    let tokens = quote! {
        #[expect(
            clippy::too_many_lines,
            reason = "the exhaustive generated-shape construction dispatch is intentionally flat"
        )]
        pub(super) fn build(
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
            name: "build".to_owned(),
        },
        tokens,
        origins,
    )])
}

#[derive(Default)]
struct Lowering {
    patterns: Vec<TokenStream>,
    field_values: HashMap<String, TokenStream>,
    role_features: HashMap<(String, Feature), TokenStream>,
    guards: Vec<TokenStream>,
    dynamic_number: Option<syn::Ident>,
}

fn emit_arm(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    rule_id: &str,
) -> syn::Result<TokenStream> {
    let mut lowering = Lowering::default();
    for atom in &construction.form.atoms {
        lower_atom(validated, construction, atom, &mut lowering)?;
    }
    if let Some(root) = parse_root(validated, &construction.category) {
        let punctuation = &root.punctuation;
        lowering
            .patterns
            .push(quote! { BuildValue::Leaf(Leaf::Literal(#punctuation)) });
        lowering
            .patterns
            .push(quote! { BuildValue::Leaf(Leaf::EndOfInput) });
    }
    lower_feature_guards(validated, construction, &mut lowering)?;
    let success = if let Some(dynamic_role) = dynamic_match_role(validated, construction) {
        emit_dynamic_match(validated, construction, &lowering, &dynamic_role)?
    } else {
        emit_success(validated, construction, &lowering, None)?
    };
    let rule_id = ident(rule_id);
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
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    atom: &FormAtom,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    match atom {
        FormAtom::Literal(literal) => lowering
            .patterns
            .push(quote! { BuildValue::Leaf(Leaf::Literal(#literal)) }),
        FormAtom::Role(role) => lower_category_role(validated, construction, role, lowering)?,
        FormAtom::Lex(role) | FormAtom::Identity(role) => {
            lower_terminal_role(validated, construction, role, false, lowering)?;
        }
        FormAtom::Noun(role) => {
            lower_terminal_role(validated, construction, role, true, lowering)?;
        }
        FormAtom::Verb(VerbOperand::Fixed(path)) => {
            let agreement = verb_agreement_pattern(validated, construction, lowering)?;
            let agreement_field = if agreement.to_string() == "agreement" {
                quote! { agreement }
            } else {
                quote! { agreement: #agreement }
            };
            lowering.patterns.push(quote! {
                BuildValue::Leaf(Leaf::Verb { lexeme: #path, #agreement_field })
            });
        }
        FormAtom::Verb(VerbOperand::Projected(_)) => {
            return Err(internal(
                "projected verb build requires a fixed lexical variant",
            ));
        }
    }
    Ok(())
}

fn lower_category_role(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    role: &syn::Ident,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let field = field(construction, role)?;
    let FieldKind::Category(category) = &field.kind else {
        return Err(internal("validated category role has the wrong field kind"));
    };
    let role_name = role.to_string();
    let refinement = construction
        .requirements
        .iter()
        .find(|requirement| requirement.role == *role);
    let value_pattern = if let Some(requirement) = refinement {
        let variant = &requirement.variant;
        quote! { #category::#variant(#role) }
    } else {
        quote! { #role }
    };
    let stored = if let Some(requirement) = refinement {
        let variant = &requirement.variant;
        quote! { #category::#variant(#role.clone()) }
    } else {
        quote! { #role.clone() }
    };
    lowering.field_values.insert(role_name.clone(), stored);

    if category_has_agreement(validated, category) {
        let pattern = role_agreement_pattern(validated, construction, role, category, lowering)?;
        lowering
            .patterns
            .push(quote! { BuildValue::#category(#value_pattern, #pattern) });
    } else {
        lowering
            .patterns
            .push(quote! { BuildValue::#category(#value_pattern) });
    }
    Ok(())
}

fn role_agreement_pattern(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    role: &syn::Ident,
    category: &syn::Path,
    lowering: &mut Lowering,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: role.clone(),
        feature: Feature::Agreement,
    };
    if let Some(equation) = equation(validated, construction, &target) {
        return match equation.value() {
            FeatureExpr::Constant(value) => Ok(feature_value(*value.value())),
            FeatureExpr::FromRole { .. } => {
                let stem = snake_case(&path_name(category))
                    .trim_end_matches("_phrase")
                    .to_owned();
                let name = ident(&format!("{stem}_agreement"));
                lowering
                    .role_features
                    .insert((role.to_string(), Feature::Agreement), quote! { #name });
                Ok(quote! { #name })
            }
            FeatureExpr::MatchVocab { .. } => {
                Err(internal("category role feature cannot be a vocab match"))
            }
        };
    }
    if feature_is_read(validated, construction, role, Feature::Agreement) {
        let name = ident(&format!("{role}_agreement"));
        lowering
            .role_features
            .insert((role.to_string(), Feature::Agreement), quote! { #name });
        Ok(quote! { #name })
    } else {
        Ok(quote! { _ })
    }
}

fn lower_terminal_role(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    role: &syn::Ident,
    noun: bool,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    let field = field(construction, role)?;
    let terminal = terminal_path(field)?;
    if let Some(vocab) = find_vocab(validated, &path_name(terminal)) {
        let leaf = &vocab.name;
        if let Some(requirement) = construction
            .requirements
            .iter()
            .find(|item| item.role == *role)
        {
            let variant = &requirement.variant;
            lowering
                .patterns
                .push(quote! { BuildValue::Leaf(Leaf::#leaf(#leaf::#variant)) });
            lowering
                .field_values
                .insert(role.to_string(), quote! { #leaf::#variant });
        } else {
            let binding = ident(&vocab_argument(&vocab.name.to_string()));
            lowering
                .patterns
                .push(quote! { BuildValue::Leaf(Leaf::#leaf(#binding)) });
            lowering
                .field_values
                .insert(role.to_string(), quote! { *#binding });
            lowering
                .role_features
                .insert((role.to_string(), Feature::Agreement), quote! { #binding });
        }
        return Ok(());
    }
    let binding = find_binding(validated, &path_name(terminal))?;
    let build = binding
        .build
        .as_ref()
        .ok_or_else(|| internal("atom-capable binding has no build metadata"))?;
    let (variant, names) = binding_pattern(&build.pattern)?;
    let pattern_names = if noun || names.len() != 1 { names.clone() } else { vec![role.clone()] };
    let substitutions = names
        .iter()
        .zip(&pattern_names)
        .map(|(declared, emitted)| (declared.to_string(), quote! { #emitted }))
        .collect::<HashMap<_, _>>();
    let inner = if noun {
        let number = noun_number_pattern(validated, construction, lowering)?;
        let number_field = if number.to_string() == "number" {
            quote! { number }
        } else {
            quote! { number: #number }
        };
        quote! { Leaf::#variant { #(#pattern_names),*, #number_field } }
    } else {
        quote! { Leaf::#variant(#(#pattern_names),*) }
    };
    lowering.patterns.push(quote! { BuildValue::Leaf(#inner) });
    let construct = lower_build_expr(&build.construct, &substitutions)?;
    let stored = if direct_bound_path(&build.construct).is_some() {
        match binding.kind {
            TerminalBindingKind::Codec => quote! { #construct.clone() },
            TerminalBindingKind::Identity => quote! { *#construct },
        }
    } else {
        construct
    };
    lowering.field_values.insert(role.to_string(), stored);
    Ok(())
}

fn lower_build_expr(
    expr: &syn::Expr,
    substitutions: &HashMap<String, TokenStream>,
) -> syn::Result<TokenStream> {
    match expr {
        syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => {
            let name = path.path.segments[0].ident.to_string();
            substitutions
                .get(&name)
                .cloned()
                .ok_or_else(|| internal("validated build expression references an absent binder"))
        }
        syn::Expr::Field(field) => {
            let base = lower_build_expr(&field.base, substitutions)?;
            let member = &field.member;
            Ok(quote! { #base.#member })
        }
        syn::Expr::Call(call) => {
            let syn::Expr::Path(function) = &*call.func else {
                return Err(internal("validated build call target is not a static path"));
            };
            if function.qself.is_some() || function.path.segments.len() <= 1 {
                return Err(internal("validated build call target is not a static path"));
            }
            let arguments = call
                .args
                .iter()
                .map(|argument| lower_build_expr(argument, substitutions))
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! { #function(#(#arguments),*) })
        }
        syn::Expr::Paren(paren) => {
            let inner = lower_build_expr(&paren.expr, substitutions)?;
            Ok(quote! { (#inner) })
        }
        _ => Err(internal(
            "validated build expression is outside the lowerable closed grammar",
        )),
    }
}

fn direct_bound_path(expr: &syn::Expr) -> Option<&syn::Path> {
    match expr {
        syn::Expr::Path(path) => Some(&path.path),
        syn::Expr::Paren(paren) => direct_bound_path(&paren.expr),
        _ => None,
    }
}

fn noun_number_pattern(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    lowering: &mut Lowering,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Construction(Feature::Number);
    let equation = equation(validated, construction, &target)
        .ok_or_else(|| internal("noun atom has no construction number"))?;
    match equation.value() {
        FeatureExpr::Constant(value) => Ok(feature_value(*value.value())),
        FeatureExpr::MatchVocab { .. } => {
            let number = ident("number");
            lowering.dynamic_number = Some(number.clone());
            Ok(quote! { #number })
        }
        FeatureExpr::FromRole { .. } => Err(internal(
            "role-derived noun number is not a closed parser pattern",
        )),
    }
}

fn verb_agreement_pattern(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    lowering: &mut Lowering,
) -> syn::Result<TokenStream> {
    let verb = ident("verb");
    let target = FeaturePlace::Role {
        field: verb,
        feature: Feature::Agreement,
    };
    match equation(validated, construction, &target).map(crate::feature::FeatureEquation::value) {
        Some(FeatureExpr::Constant(value)) => Ok(feature_value(*value.value())),
        Some(FeatureExpr::FromRole { role, feature }) => {
            if let Some(value) = refined_vocab_feature(validated, construction, role, *feature) {
                return Ok(feature_value(value));
            }
            let agreement = ident("agreement");
            lowering.role_features.insert(
                ("verb".to_owned(), Feature::Agreement),
                quote! { #agreement },
            );
            Ok(quote! { #agreement })
        }
        Some(FeatureExpr::MatchVocab { .. }) => {
            Err(internal("verb agreement cannot be a vocab match"))
        }
        None => {
            let agreement = ident("agreement");
            lowering.role_features.insert(
                ("verb".to_owned(), Feature::Agreement),
                quote! { #agreement },
            );
            Ok(quote! { #agreement })
        }
    }
}

fn lower_feature_guards(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    lowering: &mut Lowering,
) -> syn::Result<()> {
    for equation in validated.feature_equations(&construction.name.to_string()) {
        let FeaturePlace::Role { field, feature } = equation.target() else { continue };
        let FeatureExpr::FromRole {
            role,
            feature: source_feature,
        } = equation.value()
        else {
            continue;
        };
        if feature != source_feature
            || field == "verb"
                && refined_vocab_feature(validated, construction, role, *feature).is_some()
        {
            continue;
        }
        let left = lowering
            .role_features
            .get(&(role.to_string(), *feature))
            .ok_or_else(|| internal("feature source was not bound by build pattern"))?;
        let right = lowering
            .role_features
            .get(&(field.to_string(), *feature))
            .ok_or_else(|| internal("feature target was not bound by build pattern"))?;
        lowering.guards.push(quote! { #left == #right });
    }
    Ok(())
}

fn emit_success(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    lowering: &Lowering,
    overrides: Option<&HashMap<String, TokenStream>>,
) -> syn::Result<TokenStream> {
    let element = &construction.element.name;
    let category = &construction.category;
    let variant = ident(&pascal_case(&construction.name.to_string()));
    let element_value = if let Some(checked) = &construction.checked {
        let path = &checked.constructor.path;
        let arguments = checked
            .constructor
            .arguments
            .iter()
            .map(|argument| match argument {
                ConstructorArgument::Role(role) => {
                    stored_value(validated, construction, lowering, role, overrides)
                }
                ConstructorArgument::VecRole { role, .. } => {
                    stored_value(validated, construction, lowering, role, overrides)
                        .map(|value| quote! { vec![#value] })
                }
                ConstructorArgument::Context(_) => Ok(quote! { context }),
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let mapped = quote! { #category::#variant };
        if category_has_agreement(validated, category) {
            let output = construction_agreement(validated, construction, lowering, overrides)?;
            let argument = ident(&snake_case(&construction.name.to_string()));
            return Ok(quote! {
                #path(#(#arguments),*).map(|#argument| { BuildValue::#category(#category::#variant(#argument), #output) })
            });
        }
        return Ok(quote! { #path(#(#arguments),*).map(#mapped).map(BuildValue::#category) });
    } else if construction.element.fields.is_empty() {
        quote! { #element }
    } else {
        let fields = construction
            .element
            .fields
            .iter()
            .map(|field| {
                let name = &field.name;
                let value = stored_value(validated, construction, lowering, name, overrides)?;
                Ok(quote! { #name: #value })
            })
            .collect::<syn::Result<Vec<_>>>()?;
        quote! { #element { #(#fields),* } }
    };
    let category_value = quote! { #category::#variant(#element_value) };
    let wrapped = if category_has_agreement(validated, category) {
        let agreement = construction_agreement(validated, construction, lowering, overrides)?;
        quote! { BuildValue::#category(#category_value, #agreement) }
    } else {
        quote! { BuildValue::#category(#category_value) }
    };
    Ok(quote! { Some(#wrapped) })
}

fn emit_dynamic_match(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    lowering: &Lowering,
    role: &syn::Ident,
) -> syn::Result<TokenStream> {
    let number = lowering
        .dynamic_number
        .as_ref()
        .ok_or_else(|| internal("dynamic number pattern is absent"))?;
    let role_value = lowering
        .role_features
        .get(&(role.to_string(), Feature::Agreement))
        .ok_or_else(|| internal("dynamic vocabulary role pattern is absent"))?;
    let number_equation = equation(
        validated,
        construction,
        &FeaturePlace::Construction(Feature::Number),
    )
    .unwrap();
    let FeatureExpr::MatchVocab {
        arms: number_arms, ..
    } = number_equation.value()
    else {
        unreachable!()
    };
    let agreement_equation = equation(
        validated,
        construction,
        &FeaturePlace::Construction(Feature::Agreement),
    )
    .ok_or_else(|| internal("dynamic noun number requires an agreement equation"))?;
    let FeatureExpr::MatchVocab {
        arms: agreement_arms,
        ..
    } = agreement_equation.value()
    else {
        return Err(internal(
            "dynamic noun number requires a parallel agreement match",
        ));
    };
    let field = field(construction, role)?;
    let terminal = terminal_path(field)?;
    let ty = terminal;
    let mut arms = Vec::new();
    for (variant, number_value) in number_arms {
        let agreement_value = agreement_arms
            .iter()
            .find(|(candidate, _)| candidate.value() == variant.value())
            .map(|(_, value)| *value)
            .ok_or_else(|| internal("dynamic feature maps are inconsistent"))?;
        let variant = variant.value();
        let number_value = feature_value(*number_value);
        let mut overrides = HashMap::new();
        overrides.insert(role.to_string(), quote! { #ty::#variant });
        let success = emit_success_with_agreement(
            validated,
            construction,
            lowering,
            &overrides,
            agreement_value,
        )?;
        arms.push(quote! { (#ty::#variant, #number_value) => #success });
    }
    Ok(quote! { match (#role_value, #number) { #(#arms,)* _ => None, } })
}

fn emit_success_with_agreement(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    lowering: &Lowering,
    overrides: &HashMap<String, TokenStream>,
    agreement: FeatureValue,
) -> syn::Result<TokenStream> {
    let element = &construction.element.name;
    let category = &construction.category;
    let variant = ident(&pascal_case(&construction.name.to_string()));
    let fields = construction
        .element
        .fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let value = stored_value(validated, construction, lowering, name, Some(overrides))?;
            Ok(quote! { #name: #value })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let agreement = feature_value(agreement);
    Ok(
        quote! { Some(BuildValue::#category(#category::#variant(#element { #(#fields),* }), #agreement)) },
    )
}

fn stored_value(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    lowering: &Lowering,
    role: &syn::Ident,
    overrides: Option<&HashMap<String, TokenStream>>,
) -> syn::Result<TokenStream> {
    if let Some(value) = overrides.and_then(|values| values.get(&role.to_string())) {
        return Ok(value.clone());
    }
    let mut value = lowering
        .field_values
        .get(&role.to_string())
        .cloned()
        .ok_or_else(|| internal("stored field has no build value"))?;
    if validated
        .boxed_fields()
        .contains(&(construction.name.to_string(), role.to_string()))
    {
        value = quote! { Box::new(#value) };
    }
    Ok(value)
}

fn construction_agreement(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    lowering: &Lowering,
    _overrides: Option<&HashMap<String, TokenStream>>,
) -> syn::Result<TokenStream> {
    let equation = equation(
        validated,
        construction,
        &FeaturePlace::Construction(Feature::Agreement),
    )
    .ok_or_else(|| internal("agreement-carrying category construction has no agreement"))?;
    match equation.value() {
        FeatureExpr::Constant(value) => Ok(feature_value(*value.value())),
        FeatureExpr::FromRole { role, feature } if role == "verb" => {
            let value = lowering
                .role_features
                .get(&(role.to_string(), *feature))
                .ok_or_else(|| internal("verb agreement was not bound"))?;
            Ok(quote! { *#value })
        }
        FeatureExpr::FromRole { role, feature } => {
            if let Some(value) = refined_vocab_feature(validated, construction, role, *feature) {
                return Ok(feature_value(value));
            }
            if let Some(field) = construction
                .element
                .fields
                .iter()
                .find(|field| field.name == *role)
                && matches!(field.kind, FieldKind::Lex(_))
            {
                let terminal = terminal_path(field)?;
                let helper = format_ident!(
                    "{}_for_{}",
                    feature_name(*feature),
                    snake_case(&path_name(terminal))
                );
                let value = lowering
                    .role_features
                    .get(&(role.to_string(), *feature))
                    .ok_or_else(|| internal("lexical feature source was not bound"))?;
                return Ok(quote! { #helper(*#value) });
            }
            let value = lowering
                .role_features
                .get(&(role.to_string(), *feature))
                .ok_or_else(|| internal("role feature source was not bound"))?;
            Ok(quote! { *#value })
        }
        FeatureExpr::MatchVocab { .. } => Err(internal(
            "dynamic agreement must be lowered with its number match",
        )),
    }
}

fn dynamic_match_role(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
) -> Option<syn::Ident> {
    equation(
        validated,
        construction,
        &FeaturePlace::Construction(Feature::Number),
    )
    .and_then(|equation| match equation.value() {
        FeatureExpr::MatchVocab { role, .. } => Some(role.clone()),
        _ => None,
    })
}

fn refined_vocab_feature(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    role: &syn::Ident,
    feature: Feature,
) -> Option<FeatureValue> {
    let requirement = construction
        .requirements
        .iter()
        .find(|item| item.role == *role)?;
    let target = FeaturePlace::Role {
        field: role.clone(),
        feature,
    };
    let writer = equation(validated, construction, &target)?;
    let FeatureExpr::MatchVocab { arms, .. } = writer.value() else { return None };
    arms.iter()
        .find(|(variant, _)| variant.value() == &requirement.variant)
        .map(|(_, value)| *value)
}

fn equation<'a>(
    validated: &'a ValidatedDeclarations,
    construction: &crate::Construction,
    target: &FeaturePlace,
) -> Option<&'a crate::feature::FeatureEquation> {
    validated
        .feature_equations(&construction.name.to_string())
        .iter()
        .find(|equation| equation.target() == target)
}

fn feature_is_read(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    role: &syn::Ident,
    feature: Feature,
) -> bool {
    validated.feature_equations(&construction.name.to_string()).iter().any(|equation| {
        matches!(equation.value(), FeatureExpr::FromRole { role: source, feature: found } if source == role && *found == feature)
    })
}

fn category_has_agreement(validated: &ValidatedDeclarations, category: &syn::Path) -> bool {
    validated
        .raw()
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(construction)
                if path_name(&construction.category) == path_name(category) =>
            {
                Some(construction)
            }
            _ => None,
        })
        .any(|construction| {
            equation(
                validated,
                construction,
                &FeaturePlace::Construction(Feature::Agreement),
            )
            .is_some()
        })
}

fn binding_pattern(pattern: &syn::Pat) -> syn::Result<(syn::Ident, Vec<syn::Ident>)> {
    let syn::Pat::TupleStruct(tuple) = pattern else {
        return Err(internal("validated binding pattern is not tuple-like"));
    };
    let variant = tuple
        .path
        .segments
        .last()
        .ok_or_else(|| internal("binding pattern path is empty"))?
        .ident
        .clone();
    let names = tuple
        .elems
        .iter()
        .map(|pattern| match pattern {
            syn::Pat::Ident(value) => Ok(value.ident.clone()),
            _ => Err(internal(
                "validated binding pattern slot is not an identifier",
            )),
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok((variant, names))
}

fn find_vocab<'a>(validated: &'a ValidatedDeclarations, name: &str) -> Option<&'a crate::Vocab> {
    validated
        .raw()
        .declarations
        .iter()
        .find_map(|declaration| match declaration {
            Declaration::Vocab(vocab) if vocab.name == name => Some(vocab),
            _ => None,
        })
}
fn find_binding<'a>(
    validated: &'a ValidatedDeclarations,
    name: &str,
) -> syn::Result<&'a crate::TerminalBinding> {
    validated
        .raw()
        .declarations
        .iter()
        .find_map(|declaration| match declaration {
            Declaration::Codec(binding) | Declaration::Identity(binding)
                if binding.name == name =>
            {
                Some(binding)
            }
            _ => None,
        })
        .ok_or_else(|| internal("resolved terminal binding is absent"))
}
fn parse_root<'a>(
    validated: &'a ValidatedDeclarations,
    category: &syn::Path,
) -> Option<&'a crate::Root> {
    validated
        .raw()
        .declarations
        .iter()
        .find_map(|declaration| match declaration {
            Declaration::Root(root)
                if root.eoi && path_name(&root.category) == path_name(category) =>
            {
                Some(root)
            }
            _ => None,
        })
}
fn field<'a>(
    construction: &'a crate::Construction,
    role: &syn::Ident,
) -> syn::Result<&'a crate::Field> {
    construction
        .element
        .fields
        .iter()
        .find(|field| field.name == *role)
        .ok_or_else(|| internal("resolved role field is absent"))
}
fn terminal_path(field: &crate::Field) -> syn::Result<&syn::Path> {
    match &field.kind {
        FieldKind::Lex(path) | FieldKind::Identity(path) => Ok(path),
        FieldKind::Category(_) => Err(internal("terminal role resolves to category")),
    }
}
fn feature_value(value: FeatureValue) -> TokenStream {
    match value {
        FeatureValue::Bare => quote! { Agreement::Bare },
        FeatureValue::ThirdPersonSingular => quote! { Agreement::ThirdPersonSingular },
        FeatureValue::Singular => quote! { NounNumber::Singular },
        FeatureValue::Plural => quote! { NounNumber::Plural },
    }
}
fn feature_name(feature: Feature) -> &'static str {
    match feature {
        Feature::Agreement => "agreement",
        Feature::Number => "number",
    }
}
fn vocab_argument(name: &str) -> String {
    snake_case(name.strip_suffix("Word").unwrap_or(name))
}
fn path_name(path: &syn::Path) -> String {
    path.segments
        .last()
        .map_or_else(String::new, |segment| segment.ident.to_string())
}
fn ident(name: &str) -> syn::Ident {
    format_ident!("{name}")
}
fn pascal_case(name: &str) -> String {
    name.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(chars).collect()
            })
        })
        .collect()
}
fn snake_case(name: &str) -> String {
    let chars = name.chars().collect::<Vec<_>>();
    let mut result = String::new();
    for (index, ch) in chars.iter().copied().enumerate() {
        if ch.is_uppercase() {
            let lower = index > 0 && chars[index - 1].is_lowercase();
            let acronym = index > 0
                && chars[index - 1].is_uppercase()
                && chars.get(index + 1).is_some_and(|next| next.is_lowercase());
            if (lower || acronym) && !result.ends_with('_') {
                result.push('_');
            }
            result.extend(ch.to_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}
fn internal(message: &str) -> syn::Error {
    syn::Error::new(Span::call_site(), message)
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    #[test]
    fn binding_build_construct_consumes_every_declared_pattern_slot() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                codec Pair {
                    atom = lex;
                    value_type = RuntimeType;
                    lexical = Lexical::Pair;
                    render = render_runtime_type;
                    build {
                        pattern = BuildValue::Pair(left, right);
                        construct = RuntimeType::new(left.code, (Factory::wrap(right)));
                    }
                    traversal {
                        part left = scalar(left);
                        part right = scalar(right);
                        visit left;
                        visit right;
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
        let items = super::emit(&validated).expect("validated binding construction lowers");
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
                    BuildValue::Leaf(Leaf::Pair(left, right)),
                    BuildValue::Leaf(Leaf::Literal("!")),
                    BuildValue::Leaf(Leaf::EndOfInput)
                ] => Some(BuildValue::Root(Root::Wrapped(Wrapped {
                    value: RuntimeType::new(left.code, (Factory::wrap(right)))
                }))),
                _ => None,
            }
        };
        assert_eq!(arm, expected);
    }

    #[test]
    fn synthetic_projection_build_owns_every_mechanism_and_fallback() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::synthetic_projection_tokens()).unwrap(),
        )
        .unwrap();
        let actual = super::emit(&validated).unwrap();
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
