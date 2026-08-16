use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::ValidatedDeclarations;
use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::feature::FeatureValue;
use crate::model::Declaration;
use crate::model::FieldKind;
use crate::model::FormAtom;
use crate::model::RenderBinding;
use crate::model::VisitMode;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;

#[allow(
    clippy::too_many_lines,
    reason = "the phase finalizer preserves the pinned source-order item sequence"
)]
pub(crate) fn emit(validated: &ValidatedDeclarations) -> syn::Result<Vec<GeneratedItem>> {
    let constructions = validated
        .raw()
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(value) => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    let roots = validated
        .raw()
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Root(value) if value.standalone_render => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    let categories = category_groups(validated, &constructions);
    let nested_categories = constructions
        .iter()
        .flat_map(|construction| &construction.element.fields)
        .filter_map(|field| match &field.kind {
            FieldKind::Category(path) => Some(path_name(path)),
            FieldKind::Lex(_) | FieldKind::Identity(_) => None,
        })
        .collect::<HashSet<_>>();
    let root_names = roots
        .iter()
        .map(|root| path_name(&root.category))
        .collect::<HashSet<_>>();
    let context_categories = context_categories(&categories);
    let agreement_parameter_categories = categories
        .iter()
        .filter(|(_, members)| members.iter().any(|construction| {
            validated.feature_equations(&construction.name.to_string()).iter().any(|equation| {
                matches!(equation.target(), FeaturePlace::Construction(Feature::Agreement))
                    && matches!(equation.value(), FeatureExpr::FromRole { role, .. } if !construction.element.fields.iter().any(|field| field.name == *role))
            })
        }))
        .map(|(name, _)| name.clone())
        .collect::<HashSet<_>>();

    let mut items = Vec::new();
    for root in &roots {
        let category = path_name(&root.category);
        let members = categories
            .iter()
            .find(|(name, _)| name == &category)
            .map(|(_, members)| members.as_slice())
            .ok_or_else(|| internal("validated root category is absent"))?;
        let ty = &root.category;
        let punctuation = punctuation(&root.punctuation)?;
        let render_body = if nested_categories.contains(&category) {
            let helper = render_category_name(&category, true);
            quote! { #helper(&mut writer, self, context); }
        } else {
            let arms = render_arms(
                validated,
                members,
                true,
                &categories,
                &root_names,
                &context_categories,
            )?;
            quote! { match self { #(#arms)* } }
        };
        let tokens = quote! {
            impl Render for #ty {
                fn render(&self, context: &ParseContext<'_>) -> String {
                    let mut writer = Writer::new();
                    #render_body
                    writer.punctuation(#punctuation);
                    writer.finish()
                }
            }
        };
        items.push(GeneratedItem::new(
            ItemKey::Impl {
                trait_name: Some("Render".to_owned()),
                self_ty: category.clone(),
            },
            tokens,
            vec![DeclarationKey::new(DeclarationKind::Root, category)],
        ));
    }

    let render_order = render_category_order(&categories, &roots, &nested_categories);
    for category in render_order {
        let members = categories
            .iter()
            .find(|(name, _)| name == &category)
            .map(|(_, members)| members)
            .ok_or_else(|| internal("render category order is inconsistent"))?;
        if root_names.contains(&category) && !nested_categories.contains(&category) {
            continue;
        }
        let helper = render_category_name(&category, root_names.contains(&category));
        let ty = ident(&category);
        let argument = category_argument(&category);
        let takes_agreement = agreement_parameter_categories.contains(&category);
        let takes_context = context_categories.contains(&category);
        let agreement = takes_agreement.then(|| quote! { agreement: Agreement });
        let context = takes_context.then(|| quote! { context: &ParseContext<'_> });
        let separators = signature_tail(&[agreement, context]);
        let arms = render_arms(
            validated,
            members,
            false,
            &categories,
            &root_names,
            &context_categories,
        )?;
        let tokens = quote! {
            fn #helper(writer: &mut Writer, #argument: &#ty #separators) {
                match #argument { #(#arms)* }
            }
        };
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: helper.to_string(),
            },
            tokens,
            members
                .iter()
                .map(|construction| {
                    DeclarationKey::new(
                        DeclarationKind::Construction,
                        construction.name.to_string(),
                    )
                })
                .collect(),
        ));
    }

    for declaration in &validated.raw().declarations {
        let Declaration::Vocab(vocab) = declaration else { continue };
        let function = format_ident!("render_{}", snake_case(&vocab.name.to_string()));
        let ty = &vocab.name;
        let argument = render_vocab_argument(&vocab.name.to_string());
        let arms = vocab.variants.iter().map(|variant| {
            let name = &variant.name;
            let word = &variant.word;
            quote! { #ty::#name => writer.word(#word) }
        });
        let tokens = quote! {
            fn #function(writer: &mut Writer, #argument: #ty) {
                match #argument { #(#arms,)* }
            }
        };
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: function.to_string(),
            },
            tokens,
            vec![DeclarationKey::new(
                DeclarationKind::Vocab,
                vocab.name.to_string(),
            )],
        ));
    }

    for feature in [Feature::Agreement, Feature::Number] {
        for (category, members) in &categories {
            if !feature_is_read(validated, &constructions, category, feature) {
                continue;
            }
            items.push(emit_feature_helper(validated, category, members, feature)?);
        }
    }
    Ok(items)
}

fn signature_tail(parts: &[Option<TokenStream>]) -> TokenStream {
    let parts = parts.iter().flatten();
    quote! { #(, #parts)* }
}

fn render_arms(
    validated: &ValidatedDeclarations,
    members: &[&crate::Construction],
    root_impl: bool,
    categories: &[(String, Vec<&crate::Construction>)],
    root_names: &HashSet<String>,
    context_categories: &HashSet<String>,
) -> syn::Result<Vec<TokenStream>> {
    members
        .iter()
        .map(|construction| {
            let variant = ident(&pascal_case(&construction.name.to_string()));
            let element = &construction.element.name;
            let qualifier = if root_impl {
                quote! { Self }
            } else {
                let category = &construction.category;
                quote! { #category }
            };
            let private = construction.checked.as_ref().is_some_and(|checked| {
                checked.visibilities.iter().any(|visibility| {
                    matches!(
                        visibility.visibility,
                        crate::NonPublicVisibility::Private(_)
                    )
                })
            });
            let whole = ident(&construction.name.to_string());
            let pattern = if construction.element.fields.is_empty() {
                quote! { #qualifier::#variant(#element) }
            } else if private {
                quote! { #qualifier::#variant(#whole) }
            } else {
                let fields = construction.element.fields.iter().map(|field| &field.name);
                quote! { #qualifier::#variant(#element { #(#fields),* }) }
            };
            let statements = render_atoms(
                validated,
                construction,
                &whole,
                categories,
                root_names,
                context_categories,
                root_impl,
            )?;
            let category_role_block =
                !root_impl && matches!(construction.form.atoms.as_slice(), [FormAtom::Role(_)]);
            if statements.len() == 1 && !category_role_block {
                let statement = syn::parse2::<syn::Stmt>(
                    statements.into_iter().next().expect("one statement"),
                )?;
                let syn::Stmt::Expr(expression, _) = statement else {
                    return Err(internal(
                        "render atom did not lower to an expression statement",
                    ));
                };
                Ok(quote! { #pattern => #expression, })
            } else {
                Ok(quote! { #pattern => { #(#statements)* } })
            }
        })
        .collect()
}

fn render_atoms(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    whole: &syn::Ident,
    categories: &[(String, Vec<&crate::Construction>)],
    root_names: &HashSet<String>,
    context_categories: &HashSet<String>,
    root_impl: bool,
) -> syn::Result<Vec<TokenStream>> {
    let call_writer = if root_impl {
        quote! { &mut writer }
    } else {
        quote! { writer }
    };
    let method_writer = quote! { writer };
    let fields = construction
        .element
        .fields
        .iter()
        .map(|field| (field.name.to_string(), field))
        .collect::<HashMap<_, _>>();
    construction
        .form
        .atoms
        .iter()
        .map(|atom| match atom {
            FormAtom::Literal(literal) => {
                let value = literal.value();
                if value.chars().count() == 1
                    && value
                        .chars()
                        .all(|character| character.is_ascii_punctuation())
                {
                    let mark = value.chars().next().expect("one punctuation character");
                    Ok(quote! { #method_writer.punctuation(#mark); })
                } else {
                    Ok(quote! { #method_writer.word(#literal); })
                }
            }
            FormAtom::Role(role) => {
                let field = fields
                    .get(&role.to_string())
                    .ok_or_else(|| internal("resolved role is absent"))?;
                let FieldKind::Category(path) = &field.kind else {
                    return Err(internal("bare role is not a category"));
                };
                let category = path_name(path);
                let helper = render_category_name(&category, root_names.contains(&category));
                let value = field_value(construction, role, whole);
                let agreement =
                    if category_takes_agreement_parameter(validated, categories, &category) {
                        role_agreement(validated, construction, role, whole, categories)?
                    } else {
                        None
                    };
                let context = context_categories
                    .contains(&category)
                    .then(|| quote! { context });
                let tail = signature_tail(&[agreement, context]);
                Ok(quote! { #helper(#call_writer, #value #tail); })
            }
            FormAtom::Lex(role) => {
                let field = fields
                    .get(&role.to_string())
                    .ok_or_else(|| internal("resolved lexical role is absent"))?;
                let terminal = field_terminal(field);
                let value = field_value(construction, role, whole);
                if let Some(vocab) = find_vocab(validated, &terminal) {
                    let function = format_ident!("render_{}", snake_case(&vocab.name.to_string()));
                    let value = copy_value(construction, role, value);
                    Ok(quote! { #function(#call_writer, #value); })
                } else {
                    let binding = find_binding(validated, &terminal)?;
                    let Some(RenderBinding::Runtime(function)) = &binding.render else {
                        return Err(internal("lex terminal lacks runtime render binding"));
                    };
                    let value = match binding.traversal.callback_mode {
                        Some(VisitMode::Copy) => copy_value(construction, role, value),
                        Some(VisitMode::Borrowed) => value,
                        None => return Err(internal("lex terminal lacks traversal pass mode")),
                    };
                    Ok(quote! { #function(#call_writer, #value); })
                }
            }
            FormAtom::Identity(role) => {
                let field = fields
                    .get(&role.to_string())
                    .ok_or_else(|| internal("resolved identity role is absent"))?;
                let binding = find_binding(validated, &field_terminal(field))?;
                let value = field_value(construction, role, whole);
                match binding
                    .render
                    .as_ref()
                    .ok_or_else(|| internal("identity lacks render metadata"))?
                {
                    RenderBinding::Runtime(function) => {
                        let value = match binding.traversal.callback_mode {
                            Some(VisitMode::Copy) => copy_value(construction, role, value),
                            Some(VisitMode::Borrowed) => value,
                            None => return Err(internal("identity lacks traversal pass mode")),
                        };
                        Ok(quote! { #function(#call_writer, #value, context); })
                    }
                    RenderBinding::ContextIdentity(arms) => {
                        let ty = simple_type_ident(&binding.value_type)?;
                        let match_arms = arms.iter().map(|arm| {
                            let variant = &arm.variant;
                            let accessor = &arm.accessor;
                            crate::emit::call_match_arm(
                                &quote! { #ty::#variant },
                                &quote! { #method_writer.identity(context.#accessor()) },
                                12,
                            )
                        });
                        let value = match binding.traversal.callback_mode {
                            Some(VisitMode::Copy) => copy_value(construction, role, value),
                            Some(VisitMode::Borrowed) => value,
                            None => return Err(internal("identity lacks traversal pass mode")),
                        };
                        Ok(quote! { match #value { #(#match_arms),* } })
                    }
                }
            }
            FormAtom::Verb(operand) => {
                let variant = match operand {
                    crate::VerbOperand::Fixed(path) => path,
                    crate::VerbOperand::Projected(_) => {
                        return Err(internal("projected verb has no fixed render lexeme"));
                    }
                };
                let agreement = verb_agreement(validated, construction, whole)?;
                Ok(quote! { #method_writer.word(inflect(#variant, #agreement)); })
            }
            FormAtom::Noun(role) => {
                let field = fields
                    .get(&role.to_string())
                    .ok_or_else(|| internal("resolved noun role is absent"))?;
                let binding = find_binding(validated, &field_terminal(field))?;
                let Some(RenderBinding::Runtime(function)) = &binding.render else {
                    return Err(internal("noun terminal lacks runtime render binding"));
                };
                let category = path_name(&construction.category);
                let number = format_ident!("number_for_{}", snake_case(&category));
                let category_value = if root_impl {
                    quote! { self }
                } else {
                    let category_value = category_argument(&category);
                    quote! { #category_value }
                };
                let value = field_value(construction, role, whole);
                Ok(quote! { #function(#call_writer, #value, #number(#category_value)); })
            }
        })
        .collect()
}

fn role_agreement(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    role: &syn::Ident,
    whole: &syn::Ident,
    categories: &[(String, Vec<&crate::Construction>)],
) -> syn::Result<Option<TokenStream>> {
    let equation = validated.feature_equations(&construction.name.to_string()).iter().find(|equation| {
        matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if field == role)
    });
    equation
        .map(|equation| {
            feature_expr(
                validated,
                construction,
                equation.value(),
                Feature::Agreement,
                categories,
                Some(whole),
            )
        })
        .transpose()
}

fn category_takes_agreement_parameter(
    validated: &ValidatedDeclarations,
    categories: &[(String, Vec<&crate::Construction>)],
    category: &str,
) -> bool {
    categories
        .iter()
        .find(|(name, _)| name == category)
        .is_some_and(|(_, members)| {
            members.iter().any(|construction| {
                validated
                    .feature_equations(&construction.name.to_string())
                    .iter()
                    .any(|equation| {
                        matches!(
                            equation.target(),
                            FeaturePlace::Construction(Feature::Agreement)
                        ) && matches!(
                            equation.value(),
                            FeatureExpr::FromRole { role, .. }
                                if !construction
                                    .element
                                    .fields
                                    .iter()
                                    .any(|field| field.name == *role)
                        )
                    })
            })
        })
}

fn verb_agreement(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    whole: &syn::Ident,
) -> syn::Result<TokenStream> {
    let equations = validated.feature_equations(&construction.name.to_string());
    if let Some(equation) = equations.iter().find(|equation| {
        matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if field == "verb")
    }) {
        return feature_expr(
            validated,
            construction,
            equation.value(),
            Feature::Agreement,
            &[],
            Some(whole),
        );
    }
    if equations.iter().any(|equation| {
        matches!(equation.target(), FeaturePlace::Construction(Feature::Agreement))
            && matches!(equation.value(), FeatureExpr::FromRole { role, feature: Feature::Agreement } if role == "verb")
    }) {
        return Ok(quote! { agreement });
    }
    Err(internal("verb atom lacks validated agreement flow"))
}

fn feature_expr(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    expression: &FeatureExpr,
    _feature: Feature,
    _categories: &[(String, Vec<&crate::Construction>)],
    whole: Option<&syn::Ident>,
) -> syn::Result<TokenStream> {
    Ok(match expression {
        FeatureExpr::Constant(value) => feature_value(*value.value()),
        FeatureExpr::FromRole {
            role,
            feature: source_feature,
        } => {
            if !construction
                .element
                .fields
                .iter()
                .any(|field| field.name == *role)
            {
                return Ok(quote! { agreement });
            }
            let field = construction
                .element
                .fields
                .iter()
                .find(|field| field.name == *role)
                .expect("resolved role");
            let role_value = whole.map_or_else(
                || quote! { #role },
                |whole| field_value(construction, role, whole),
            );
            let (source, value) = match &field.kind {
                FieldKind::Category(path) => (path_name(path), role_value),
                FieldKind::Lex(_) => {
                    let (writer_role, vocabulary) = canonical_lexical_feature_lowering(
                        validated,
                        construction,
                        role,
                        *source_feature,
                    )?;
                    let writer_value = whole.map_or_else(
                        || quote! { #writer_role },
                        |whole| field_value(construction, writer_role, whole),
                    );
                    (
                        path_name(vocabulary),
                        copy_value(construction, writer_role, writer_value),
                    )
                }
                FieldKind::Identity(_) => {
                    return Err(internal(
                        "identity roles cannot provide grammatical features",
                    ));
                }
            };
            let function = format_ident!(
                "{}_for_{}",
                feature_name(*source_feature),
                snake_case(&source)
            );
            quote! { #function(#value) }
        }
        FeatureExpr::MatchVocab { role, arms } => {
            let field = construction
                .element
                .fields
                .iter()
                .find(|field| field.name == *role)
                .ok_or_else(|| internal("match feature role is absent"))?;
            let ty = ident(&field_terminal(field));
            let match_arms = arms.iter().map(|(variant, value)| {
                let variant = variant.value();
                let value = feature_value(*value);
                quote! { #ty::#variant => #value }
            });
            let role_value = whole.map_or_else(
                || quote! { #role },
                |whole| field_value(construction, role, whole),
            );
            let role_value = copy_value(construction, role, role_value);
            quote! { match #role_value { #(#match_arms),* } }
        }
    })
}

fn canonical_lexical_feature_lowering<'a>(
    validated: &'a ValidatedDeclarations,
    construction: &'a crate::Construction,
    role: &syn::Ident,
    feature: Feature,
) -> syn::Result<(&'a syn::Ident, &'a syn::Path)> {
    let writer = validated
        .feature_equations(&construction.name.to_string())
        .iter()
        .find(|equation| {
            matches!(
                equation.target(),
                FeaturePlace::Role {
                    field,
                    feature: writer_feature,
                } if field == role && *writer_feature == feature
            )
        })
        .ok_or_else(|| {
            internal(&format!(
                "lexical feature read `{role}.{}` lacks its exact local {} writer",
                feature_name(feature),
                feature_name(feature),
            ))
        })?;
    let FeatureExpr::MatchVocab {
        role: writer_role, ..
    } = writer.value()
    else {
        return Err(internal(&format!(
            "lexical feature read `{role}.{}` requires an exhaustive local match writer",
            feature_name(feature),
        )));
    };
    if writer_role != role {
        return Err(internal(&format!(
            "lexical feature read `{role}.{}` has a mismatched local match writer",
            feature_name(feature),
        )));
    }
    let vocabulary = construction
        .element
        .fields
        .iter()
        .find(|field| field.name == *writer_role)
        .and_then(|field| match &field.kind {
            FieldKind::Lex(path) => Some(path),
            FieldKind::Category(_) | FieldKind::Identity(_) => None,
        })
        .ok_or_else(|| internal("lexical feature writer is not rooted in a vocabulary role"))?;
    Ok((writer_role, vocabulary))
}

fn emit_feature_helper(
    validated: &ValidatedDeclarations,
    category: &str,
    members: &[&crate::Construction],
    feature: Feature,
) -> syn::Result<GeneratedItem> {
    let function = format_ident!("{}_for_{}", feature_name(feature), snake_case(category));
    let ty = ident(category);
    let argument = category_argument(category);
    let return_ty = match feature {
        Feature::Agreement => quote! { Agreement },
        Feature::Number => quote! { Number },
    };
    let mut entries: Vec<(TokenStream, String, TokenStream)> = Vec::new();
    for construction in members {
        let equation = validated.feature_equations(&construction.name.to_string()).iter().find(|equation| {
            matches!(equation.target(), FeaturePlace::Construction(found) if *found == feature)
        }).ok_or_else(|| internal("feature helper construction lacks equation"))?;
        let variant = ident(&pascal_case(&construction.name.to_string()));
        let element = &construction.element.name;
        if let FeatureExpr::MatchVocab { role, arms: values } = equation.value() {
            let field = construction
                .element
                .fields
                .iter()
                .find(|field| field.name == *role)
                .ok_or_else(|| internal("feature match role absent"))?;
            let field_type = ident(&field_terminal(field));
            let other_fields = construction
                .element
                .fields
                .iter()
                .filter(|candidate| candidate.name != *role)
                .map(|candidate| {
                    let name = &candidate.name;
                    quote! { #name: _ }
                })
                .collect::<Vec<_>>();
            for (value_variant, value) in values {
                let value_variant = value_variant.value();
                let value = feature_value(*value);
                let pattern = quote! { #ty::#variant(#element { #role: #field_type::#value_variant, #(#other_fields),* }) };
                entries.push((pattern, value.to_string(), value));
            }
        } else {
            let whole = ident(&construction.name.to_string());
            let pattern = feature_constant_pattern(
                validated,
                construction,
                &ty,
                &variant,
                element,
                equation.value(),
                &whole,
            )?;
            let value = feature_expr(
                validated,
                construction,
                equation.value(),
                feature,
                &[],
                Some(&whole),
            )?;
            entries.push((pattern, value.to_string(), value));
        }
    }
    let mut groups: Vec<(String, TokenStream, Vec<TokenStream>)> = Vec::new();
    for (pattern, key, value) in entries {
        if let Some((_, _, patterns)) = groups.iter_mut().find(|(found, _, _)| found == &key) {
            patterns.push(pattern);
        } else {
            groups.push((key, value, vec![pattern]));
        }
    }
    let arms = groups
        .into_iter()
        .map(|(_, value, patterns)| quote! { #(#patterns)|* => #value });
    let tokens = quote! {
        fn #function(#argument: &#ty) -> #return_ty {
            match #argument { #(#arms,)* }
        }
    };
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        tokens,
        members
            .iter()
            .map(|construction| {
                DeclarationKey::new(DeclarationKind::Construction, construction.name.to_string())
            })
            .collect(),
    ))
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "keeps the feature-pattern lowering interface uniformly fallible"
)]
fn feature_constant_pattern(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    category: &syn::Ident,
    variant: &syn::Ident,
    element: &syn::Ident,
    expression: &FeatureExpr,
    whole: &syn::Ident,
) -> syn::Result<TokenStream> {
    if construction.element.fields.is_empty() {
        return Ok(quote! { #category::#variant(#element) });
    }
    if construction.checked.as_ref().is_some_and(|checked| {
        checked.visibilities.iter().any(|visibility| {
            matches!(
                visibility.visibility,
                crate::NonPublicVisibility::Private(_)
            )
        })
    }) {
        if matches!(expression, FeatureExpr::FromRole { role, .. } if construction.element.fields.iter().any(|field| field.name == *role))
        {
            return Ok(quote! { #category::#variant(#whole) });
        }
        return Ok(quote! { #category::#variant(_) });
    }
    let mut fields = Vec::new();
    for field in &construction.element.fields {
        let name = &field.name;
        if matches!(expression, FeatureExpr::FromRole { role, .. } if role == name) {
            fields.push(quote! { #name });
            continue;
        }
        let refined = construction
            .requirements
            .iter()
            .any(|requirement| requirement.role == *name);
        let pattern = if refined {
            quote! { #name: _ }
        } else if let FieldKind::Lex(path) = &field.kind {
            if let Some(vocab) = find_vocab(validated, &path_name(path)) {
                let ty = &vocab.name;
                let variants = vocab.variants.iter().map(|variant| {
                    let variant = &variant.name;
                    quote! { #ty::#variant }
                });
                quote! { #name: #(#variants)|* }
            } else {
                quote! { #name: _ }
            }
        } else {
            quote! { #name: _ }
        };
        fields.push(pattern);
    }
    Ok(quote! { #category::#variant(#element { #(#fields),* }) })
}

fn feature_value(value: FeatureValue) -> TokenStream {
    match value {
        FeatureValue::Bare => quote! { Agreement::Bare },
        FeatureValue::ThirdPersonSingular => quote! { Agreement::ThirdPersonSingular },
        FeatureValue::Singular => quote! { Number::Singular },
        FeatureValue::Plural => quote! { Number::Plural },
    }
}

fn feature_is_read(
    validated: &ValidatedDeclarations,
    constructions: &[&crate::Construction],
    category: &str,
    feature: Feature,
) -> bool {
    constructions.iter().any(|construction| {
        validated.feature_equations(&construction.name.to_string()).iter().any(|equation| {
            matches!(equation.value(), FeatureExpr::FromRole { role, feature: found } if *found == feature && construction.element.fields.iter().any(|field| field.name == *role && matches!(&field.kind, FieldKind::Category(path) if path_name(path) == category)))
        }) || (feature == Feature::Number && construction.form.atoms.iter().any(|atom| matches!(atom, FormAtom::Noun(_))) && path_name(&construction.category) == category)
    })
}

fn context_categories(categories: &[(String, Vec<&crate::Construction>)]) -> HashSet<String> {
    let mut result = HashSet::new();
    loop {
        let before = result.len();
        for (category, members) in categories {
            if members.iter().any(|construction| {
                construction.form.atoms.iter().any(|atom| match atom {
                    FormAtom::Identity(_) => true,
                    FormAtom::Role(role) => construction.element.fields.iter().find(|field| field.name == *role).is_some_and(|field| matches!(&field.kind, FieldKind::Category(path) if result.contains(&path_name(path)))),
                    _ => false,
                })
            }) {
                result.insert(category.clone());
            }
        }
        if result.len() == before {
            break;
        }
    }
    result
}

fn render_category_order(
    categories: &[(String, Vec<&crate::Construction>)],
    roots: &[&crate::Root],
    nested: &HashSet<String>,
) -> Vec<String> {
    let mut order = Vec::new();
    let mut queued = HashSet::new();
    for root in roots {
        let root_name = path_name(&root.category);
        if nested.contains(&root_name) {
            continue;
        }
        if let Some((_, members)) = categories.iter().find(|(name, _)| name == &root_name) {
            enqueue_role_categories(members, &mut order, &mut queued);
        }
    }
    let mut index = 0;
    while index < order.len() {
        let name = order[index].clone();
        if let Some((_, members)) = categories.iter().find(|(category, _)| category == &name) {
            enqueue_role_categories(members, &mut order, &mut queued);
        }
        index += 1;
    }
    for (name, _) in categories {
        if queued.insert(name.clone()) {
            order.push(name.clone());
        }
    }
    order
}

fn enqueue_role_categories(
    members: &[&crate::Construction],
    order: &mut Vec<String>,
    queued: &mut HashSet<String>,
) {
    for construction in members {
        for atom in &construction.form.atoms {
            let FormAtom::Role(role) = atom else { continue };
            let Some(field) = construction
                .element
                .fields
                .iter()
                .find(|field| field.name == *role)
            else {
                continue;
            };
            let FieldKind::Category(path) = &field.kind else { continue };
            let category = path_name(path);
            if queued.insert(category.clone()) {
                order.push(category);
            }
        }
    }
}

fn category_groups<'a>(
    validated: &ValidatedDeclarations,
    constructions: &[&'a crate::Construction],
) -> Vec<(String, Vec<&'a crate::Construction>)> {
    let mut result: Vec<(String, Vec<&crate::Construction>)> = Vec::new();
    for (construction, record) in constructions
        .iter()
        .zip(validated.contributions().constructions())
    {
        if let Some((_, members)) = result
            .iter_mut()
            .find(|(name, _)| name == record.category())
        {
            members.push(*construction);
        } else {
            result.push((record.category().to_owned(), vec![*construction]));
        }
    }
    result
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

fn field_terminal(field: &crate::Field) -> String {
    match &field.kind {
        FieldKind::Lex(path) | FieldKind::Identity(path) | FieldKind::Category(path) => {
            path_name(path)
        }
    }
}

fn field_value(
    construction: &crate::Construction,
    role: &syn::Ident,
    whole: &syn::Ident,
) -> TokenStream {
    if let Some(accessor) = construction.checked.as_ref().and_then(|checked| {
        checked
            .accessors
            .iter()
            .find(|accessor| accessor.role == *role)
    }) {
        let method = &accessor.method;
        quote! { #whole.#method() }
    } else if has_private_fields(construction) {
        quote! { &#whole.#role }
    } else {
        quote! { #role }
    }
}

fn copy_value(
    construction: &crate::Construction,
    role: &syn::Ident,
    value: TokenStream,
) -> TokenStream {
    if has_accessor(construction, role) {
        value
    } else {
        quote! { *#value }
    }
}

fn has_accessor(construction: &crate::Construction, role: &syn::Ident) -> bool {
    construction.checked.as_ref().is_some_and(|checked| {
        checked
            .accessors
            .iter()
            .any(|accessor| accessor.role == *role)
    })
}

fn has_private_fields(construction: &crate::Construction) -> bool {
    construction.checked.as_ref().is_some_and(|checked| {
        checked.visibilities.iter().any(|visibility| {
            matches!(
                visibility.visibility,
                crate::NonPublicVisibility::Private(_)
            )
        })
    })
}

fn render_category_name(category: &str, root: bool) -> syn::Ident {
    let suffix = if root {
        format!("{}_body", snake_case(category))
    } else {
        snake_case(category)
    };
    format_ident!("render_{suffix}")
}

fn category_argument(category: &str) -> syn::Ident {
    let snake = snake_case(category);
    if snake.ends_with("_phrase") { ident("phrase") } else { ident(&snake) }
}
fn render_vocab_argument(name: &str) -> syn::Ident {
    let snake = snake_case(name);
    ident(snake.strip_suffix("_word").unwrap_or(&snake))
}

fn feature_name(feature: Feature) -> &'static str {
    match feature {
        Feature::Agreement => "agreement",
        Feature::Number => "number",
    }
}
fn ident(name: &str) -> syn::Ident {
    syn::Ident::new(name, Span::call_site())
}
fn path_name(path: &syn::Path) -> String {
    path.segments
        .last()
        .map_or_else(String::new, |segment| segment.ident.to_string())
}
fn simple_type_ident(ty: &syn::Type) -> syn::Result<&syn::Ident> {
    match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| &segment.ident)
            .ok_or_else(|| internal("empty binding type")),
        _ => Err(internal("binding type must be a path")),
    }
}
fn punctuation(value: &syn::LitStr) -> syn::Result<char> {
    let text = value.value();
    let mut chars = text.chars();
    let first = chars
        .next()
        .ok_or_else(|| syn::Error::new(value.span(), "root punctuation cannot be empty"))?;
    if chars.next().is_some() {
        return Err(syn::Error::new(
            value.span(),
            "root punctuation must be one character",
        ));
    }
    Ok(first)
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
    let characters = name.chars().collect::<Vec<_>>();
    let mut result = String::new();
    for (index, character) in characters.iter().copied().enumerate() {
        if character.is_uppercase() {
            let lower = index > 0 && characters[index - 1].is_lowercase();
            let acronym = index > 0
                && characters[index - 1].is_uppercase()
                && characters
                    .get(index + 1)
                    .is_some_and(|next| next.is_lowercase());
            if (lower || acronym) && !result.ends_with('_') {
                result.push('_');
            }
            result.extend(character.to_lowercase());
        } else {
            result.push(character);
        }
    }
    result
}
fn internal(message: &str) -> syn::Error {
    syn::Error::new(Span::call_site(), message)
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::manual_assert,
        clippy::match_wildcard_for_single_variants,
        clippy::too_many_lines,
        reason = "literal full-surface structural oracles retain detailed mismatch output"
    )]
    use quote::ToTokens;

    #[test]
    fn role_derived_noun_render_uses_declared_feature_helper() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::role_derived_noun_tokens()).unwrap(),
        )
        .unwrap();
        let generated = super::emit(&validated).expect("role-derived noun render lowers");
        let source = generated
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            source.contains("number_for_source (source)"),
            "the derived feature helper must read the declared source role: {source}"
        );
        assert!(
            source.contains("render_head (& mut writer , head , number_for_phrase (self))"),
            "noun rendering must consume the construction feature helper: {source}"
        );
    }

    #[test]
    fn zero_noun_vocab_match_emits_exact_category_feature_arms() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(
                crate::test_support::vocab_matched_number_without_noun_tokens(),
            )
            .unwrap(),
        )
        .unwrap();
        let generated = super::emit(&validated).expect("zero-noun feature helpers lower");
        let source = generated
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            source.contains(
                "Source :: Source (SourceNode { count : Count :: One , }) => Number :: Singular"
            ) && source.contains(
                "Source :: Source (SourceNode { count : Count :: Many , }) => Number :: Plural"
            ),
            "the stored vocab drives the exact number helper arms: {source}"
        );
    }

    #[test]
    fn two_noun_render_calls_share_the_declared_category_number() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(
                crate::test_support::vocab_matched_number_with_two_nouns_tokens(),
            )
            .unwrap(),
        )
        .unwrap();
        let generated = super::emit(&validated).expect("two noun render atoms lower");
        let source = generated
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for fragment in [
            "render_head (& mut writer , left , number_for_phrase (self))",
            "render_head (& mut writer , right , number_for_phrase (self))",
        ] {
            assert!(source.contains(fragment), "missing `{fragment}`: {source}");
        }
    }

    #[test]
    fn explicit_lexical_feature_dependencies_drive_render_lowering() {
        let expansion = crate::generate(quote::quote! {
            vocab Person { One = "one", Many = "many", }
            lexeme Verbs { Be, }
            construction only: Root {
                element Only { person: lex Person, }
                require person is One;
                derive person.agreement = match person {
                    One => Values::Bare,
                    Many => Values::ThirdPersonSingular,
                };
                derive agreement = person.agreement;
                derive verb.agreement = person.agreement;
                form only = lex(person) verb(Verbs::Be);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("an explicit lexical provider is valid");
        let implementation = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Impl { self_ty, trait_name } if self_ty == "Root" && trait_name.as_deref() == Some("Render")))
            .expect("root Render impl");
        let source = implementation.tokens.to_string();
        assert!(
            source.contains("agreement_for_person (* person)"),
            "the consuming verb operand uses the canonical helper from the explicit writer: {source}",
        );
    }

    #[test]
    fn canonical_lexical_feature_lowering_requires_the_exact_writer() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Person { One = "one", Many = "many", }
                construction only: Root {
                    element Only { person: lex Person, }
                    derive person.agreement = match person {
                        One => Values::Bare,
                        Many => Values::ThirdPersonSingular,
                    };
                    derive agreement = person.agreement;
                    form only = lex(person);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .unwrap(),
        )
        .expect("the exact lexical writer validates");
        let construction = validated
            .raw()
            .declarations
            .iter()
            .find_map(|declaration| match declaration {
                crate::Declaration::Construction(construction) => Some(construction),
                _ => None,
            })
            .expect("construction");
        let role = syn::parse_quote!(person);
        let (writer_role, vocabulary) = super::canonical_lexical_feature_lowering(
            &validated,
            construction,
            &role,
            crate::feature::Feature::Agreement,
        )
        .expect("the exact role.feature writer is retrievable");
        assert_eq!(writer_role, "person");
        assert_eq!(super::path_name(vocabulary), "Person");
        let missing = super::canonical_lexical_feature_lowering(
            &validated,
            construction,
            &role,
            crate::feature::Feature::Number,
        )
        .expect_err("a different role.feature writer cannot satisfy the read");
        assert!(
            missing.to_string().contains("exact local number writer"),
            "{missing}"
        );
    }

    #[test]
    fn mixed_checked_render_accesses_each_field_explicitly() {
        let expansion = crate::test_support::access_modes_expansion();
        let implementation = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Impl { self_ty, trait_name } if self_ty == "Root" && trait_name.as_deref() == Some("Render")))
            .expect("root Render impl");
        let syn::Item::Impl(item) = parse(implementation) else {
            panic!("root render is an impl");
        };
        let syn::ImplItem::Fn(method) = &item.items[0] else {
            panic!("root render method");
        };
        let body = method.block.to_token_stream().to_string();
        assert!(body.contains("mixed . hidden ()"));
        assert!(body.contains("render_child (\u{26} mut writer , \u{26} mixed . child)"));
    }

    #[test]
    fn checked_feature_reads_use_the_declared_field_accessor() {
        let expansion = crate::generate(quote::quote! {
            construction subject: Subject {
                element SubjectNode {}
                derive agreement = Values::Bare;
                form subject = "subject";
            }
            construction predicate: Predicate {
                element PredicateNode {}
                derive agreement = Values::Bare;
                form predicate = "predicate";
            }
            construction checked_root: Root {
                element CheckedRoot { subject: Subject, predicate: Predicate, }
                checked {
                    visibility subject = private;
                    access subject = subject;
                    visibility predicate = pub(crate);
                    constructor = CheckedRoot::new(subject, predicate);
                }
                derive predicate.agreement = subject.agreement;
                form checked_root = subject predicate;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a checked product may source a feature from a private category field");
        let implementation = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Impl { self_ty, trait_name } if self_ty == "Root" && trait_name.as_deref() == Some("Render")))
            .expect("root Render impl");
        let source = implementation.tokens.to_string();
        assert!(
            !source.contains("agreement_for_subject (subject)"),
            "checked feature read lowered to an unbound bare role: {source}",
        );
        assert!(
            !source.contains("agreement_for_subject"),
            "a category without an agreement parameter must not receive a stray argument: {source}",
        );
        assert!(
            source.contains("render_predicate (& mut writer , & checked_root . predicate)"),
            "checked role rendering must still use its declared field access: {source}",
        );
    }

    #[test]
    fn representative_root_metadata_controls_standalone_rendering() {
        let expansion = crate::test_support::representative_expansion();
        let render_impls = expansion
            .items()
            .iter()
            .filter(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Impl { trait_name, .. }
                        if trait_name.as_deref() == Some("Render")
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(render_impls.len(), 1);
        assert!(matches!(
            &render_impls[0].key,
            crate::ItemKey::Impl { self_ty, .. } if self_ty == "Action"
        ));
        let syn::Item::Impl(item) = parse(render_impls[0]) else {
            panic!("root render item is an impl");
        };
        let syn::ImplItem::Fn(method) = &item.items[0] else {
            panic!("root render impl contains a method");
        };
        assert!(
            method
                .block
                .to_token_stream()
                .to_string()
                .contains("writer . punctuation ('.')")
        );
    }

    #[test]
    fn synthetic_projection_has_exact_render_ownership_and_feature_dispatch() {
        let expansion = crate::test_support::synthetic_projection_expansion();
        let render = expansion
            .items()
            .iter()
            .filter(|item| match &item.key {
                crate::ItemKey::Impl { trait_name, .. } => trait_name.as_deref() == Some("Render"),
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name,
                } => {
                    name.starts_with("render_")
                        || name == "agreement_for_expr"
                        || name == "number_for_expr"
                }
                _ => false,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            render
                .iter()
                .map(|item| match &item.key {
                    crate::ItemKey::Impl { self_ty, .. } => format!("impl Render for {self_ty}"),
                    crate::ItemKey::Named { name, .. } => name.clone(),
                })
                .collect::<Vec<_>>(),
            [
                "impl Render for Document",
                "render_expr",
                "render_predicate",
                "render_tag",
                "render_mode",
                "agreement_for_expr",
                "number_for_expr",
            ],
        );
        assert_eq!(
            render
                .iter()
                .map(|item| {
                    item.origins
                        .iter()
                        .map(crate::DeclarationKey::name)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            [
                vec!["Document"],
                vec!["leaf", "nested"],
                vec!["action", "idle"],
                vec!["solo"],
                vec!["Mode"],
                vec!["leaf", "nested"],
                vec!["leaf", "nested"],
            ],
        );

        let root = parse(render[0]).to_token_stream().to_string();
        for fragment in [
            "Self :: Document (document)",
            "render_expr (& mut writer , & document . subject)",
            "render_predicate (& mut writer , & document . predicate , agreement_for_expr (& document . subject))",
            "Handle :: Primary => writer . identity (context . primary_name ())",
            "Handle :: Alias => writer . identity (context . alias_name ())",
            "render_pair (& mut writer , document . pair ())",
            "writer . punctuation ('!')",
        ] {
            assert!(root.contains(fragment), "root render lacks `{fragment}`");
        }

        let agreement = parse(render[5]).to_token_stream().to_string();
        assert!(agreement.contains("Expr :: Leaf"));
        assert!(
            agreement.contains(
                "mode : Mode :: Solo , resource : _ }) => Agreement :: ThirdPersonSingular"
            )
        );
        assert!(agreement.contains("mode : Mode :: Group , resource : _ }) => Agreement :: Bare"));
        assert!(agreement.contains("Expr :: Nested"));
        assert!(agreement.contains("agreement_for_expr (nested . next ())"));

        let number = parse(render[6]).to_token_stream().to_string();
        assert!(number.contains("mode : Mode :: Solo , resource : _ }) => Number :: Singular"));
        assert!(number.contains("mode : Mode :: Group , resource : _ }) => Number :: Plural"));
        assert!(number.contains("number_for_expr (nested . next ())"));
    }
    fn parse(item: &crate::GeneratedItem) -> syn::Item {
        syn::parse2::<syn::File>(item.tokens.clone())
            .unwrap()
            .items
            .into_iter()
            .next()
            .unwrap()
    }
}
