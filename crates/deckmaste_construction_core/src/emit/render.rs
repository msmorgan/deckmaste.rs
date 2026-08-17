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
use crate::identifier::category_renderer;
use crate::identifier::emitted_ident;
use crate::identifier::feature_helper;
use crate::identifier::key as identifier_key;
use crate::identifier::pascal_case;
use crate::identifier::path_key;
use crate::identifier::snake_case;
use crate::model::FieldKind;
use crate::model::FormAtom;
use crate::model::RenderBinding;
use crate::model::VisitMode;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::semantic::SemanticPlan;
use crate::semantic::TerminalPlan;

#[allow(
    clippy::too_many_lines,
    reason = "the phase finalizer preserves the pinned source-order item sequence"
)]
pub(crate) fn emit(validated: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let constructions = validated
        .constructions()
        .iter()
        .map(|row| validated.construction_source(row))
        .collect::<syn::Result<Vec<_>>>()?;
    let roots = validated
        .roots()
        .iter()
        .filter(|root| root.is_render_entry())
        .map(|row| validated.root_source(row))
        .collect::<syn::Result<Vec<_>>>()?;
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
    let mut items = Vec::new();
    for root in &roots {
        let category = path_name(&root.category);
        let members = categories
            .iter()
            .find(|(name, _)| name == &category)
            .map(|(_, members)| members.as_slice())
            .ok_or_else(|| internal("validated root category is absent"))?;
        let ty = ident(&category);
        let punctuation = punctuation(&root.punctuation)?;
        let render_body = if nested_categories.contains(&category) {
            let helper = render_category_name(&category, true);
            let capability = validated.category_render_capability(&category);
            if capability.requires_external_agreement() {
                return Err(internal(
                    "validated standalone render root requires external agreement",
                ));
            }
            let context = capability.requires_context().then(|| quote! { context });
            let tail = signature_tail(&[None, context]);
            quote! { #helper(&mut writer, self #tail); }
        } else {
            let allocator = render_allocator(validated, members, true, &root_names, false, true)?;
            let arms = render_arms(
                validated,
                members,
                true,
                &root_names,
                &allocator,
                &quote! { self },
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
        let capability = validated.category_render_capability(&category);
        let takes_agreement = capability.requires_external_agreement();
        let takes_context = capability.requires_context();
        let allocator = render_allocator(
            validated,
            members,
            false,
            &root_names,
            takes_agreement,
            takes_context,
        )?;
        let mut signature_allocator = allocator.clone();
        let argument = signature_allocator.allocate(&category_argument(&category));
        let agreement = takes_agreement.then(|| quote! { agreement: Agreement });
        let context = takes_context.then(|| quote! { context: &ParseContext<'_> });
        let separators = signature_tail(&[agreement, context]);
        let arms = render_arms(
            validated,
            members,
            false,
            &root_names,
            &signature_allocator,
            &quote! { #argument },
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
                        identifier_key(&construction.name),
                    )
                })
                .collect(),
        ));
    }

    for terminal in validated.terminals() {
        let TerminalPlan::Vocab(row) = terminal else {
            continue;
        };
        let vocab = validated.vocab_source(row)?;
        let function = ident(&format!(
            "render_{}",
            snake_case(&identifier_key(&vocab.name))
        ));
        let ty = emitted_ident(&identifier_key(&vocab.name), vocab.name.span());
        let mut allocator = LocalAllocator::default();
        allocator.reserve("writer");
        let argument = allocator.allocate(&render_vocab_argument(&vocab.name));
        let arms = vocab.variants.iter().map(|variant| {
            let name = emitted_ident(&identifier_key(&variant.name), variant.name.span());
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
                identifier_key(&vocab.name),
            )],
        ));
    }

    for feature in [Feature::Agreement, Feature::Number] {
        for (category, members) in &categories {
            if !validated.category_reads_feature(category, feature) {
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

fn render_allocator(
    validated: &SemanticPlan,
    members: &[&crate::Construction],
    root_impl: bool,
    root_names: &HashSet<String>,
    takes_agreement: bool,
    takes_context: bool,
) -> syn::Result<LocalAllocator> {
    let mut allocator = LocalAllocator::default();
    allocator.reserve("writer");
    if root_impl {
        allocator.reserve("self");
    }
    if takes_agreement {
        allocator.reserve("agreement");
    }
    if takes_context {
        allocator.reserve("context");
    }
    for construction in members {
        let fields = construction
            .element
            .fields
            .iter()
            .map(|field| (identifier_key(&field.name), field))
            .collect::<HashMap<_, _>>();
        for atom in &construction.form.atoms {
            match atom {
                FormAtom::Literal(_) => {}
                FormAtom::Role(role) => {
                    let field = fields
                        .get(&identifier_key(role))
                        .ok_or_else(|| internal("resolved role is absent"))?;
                    let FieldKind::Category(path) = &field.kind else {
                        return Err(internal("bare role is not a category"));
                    };
                    let category = path_name(path);
                    allocator.reserve(
                        render_category_name(&category, root_names.contains(&category)).to_string(),
                    );
                    if validated.category_requires_external_agreement(&category)
                        && let Some(equation) = validated
                            .feature_equations(&identifier_key(&construction.name))
                            .iter()
                            .find(|equation| {
                                matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if identifier_key(field) == identifier_key(role))
                            })
                    {
                        reserve_feature_callees(
                            validated,
                            construction,
                            equation.value(),
                            &mut allocator,
                        )?;
                    }
                }
                FormAtom::Lex(role) => {
                    let field = fields
                        .get(&identifier_key(role))
                        .ok_or_else(|| internal("resolved lexical role is absent"))?;
                    let terminal = field_terminal(field);
                    if let Some(vocab) = find_vocab(validated, &terminal)? {
                        allocator.reserve(format!(
                            "render_{}",
                            snake_case(&identifier_key(&vocab.name))
                        ));
                    } else if let RenderBinding::Runtime(path) = find_binding(validated, &terminal)?
                        .render
                        .as_ref()
                        .ok_or_else(|| internal("lex terminal lacks render metadata"))?
                    {
                        reserve_bare_path(&mut allocator, path);
                    }
                }
                FormAtom::Identity(role) => {
                    let field = fields
                        .get(&identifier_key(role))
                        .ok_or_else(|| internal("resolved identity role is absent"))?;
                    if let RenderBinding::Runtime(path) =
                        find_binding(validated, &field_terminal(field))?
                            .render
                            .as_ref()
                            .ok_or_else(|| internal("identity lacks render metadata"))?
                    {
                        reserve_bare_path(&mut allocator, path);
                    }
                }
                FormAtom::Verb(_) => {
                    allocator.reserve("inflect");
                    let equations =
                        validated.feature_equations(&identifier_key(&construction.name));
                    if let Some(equation) = equations.iter().find(|equation| {
                        matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if identifier_key(field) == "verb")
                    }) {
                        reserve_feature_callees(
                            validated,
                            construction,
                            equation.value(),
                            &mut allocator,
                        )?;
                    }
                }
                FormAtom::Noun(role) => {
                    let field = fields
                        .get(&identifier_key(role))
                        .ok_or_else(|| internal("resolved noun role is absent"))?;
                    let binding = find_binding(validated, &field_terminal(field))?;
                    let Some(RenderBinding::Runtime(path)) = &binding.render else {
                        return Err(internal("noun terminal lacks runtime render binding"));
                    };
                    reserve_bare_path(&mut allocator, path);
                    allocator.reserve(feature_helper("number", &path_name(&construction.category)));
                }
            }
        }
    }
    Ok(allocator)
}

fn reserve_bare_path(allocator: &mut LocalAllocator, path: &syn::Path) {
    if path.segments.len() == 1 {
        allocator.reserve_ident(&path.segments[0].ident);
    }
}

fn reserve_feature_callees(
    validated: &SemanticPlan,
    construction: &crate::Construction,
    expression: &FeatureExpr,
    allocator: &mut LocalAllocator,
) -> syn::Result<()> {
    let FeatureExpr::FromRole {
        role,
        feature: source_feature,
    } = expression
    else {
        return Ok(());
    };
    if !construction
        .element
        .fields
        .iter()
        .any(|field| identifier_key(&field.name) == identifier_key(role))
    {
        if let Some(writer) = validated
            .feature_equations(&identifier_key(&construction.name))
            .iter()
            .find(|equation| {
                matches!(equation.target(), FeaturePlace::Role { field, feature } if identifier_key(field) == identifier_key(role) && feature == source_feature)
            })
        {
            reserve_feature_callees(validated, construction, writer.value(), allocator)?;
        }
        return Ok(());
    }
    let field = construction
        .element
        .fields
        .iter()
        .find(|field| identifier_key(&field.name) == identifier_key(role))
        .expect("resolved feature role");
    if matches!(field.kind, FieldKind::Category(_))
        && let Some(writer) = validated
            .feature_equations(&identifier_key(&construction.name))
            .iter()
            .find(|equation| {
                matches!(equation.target(), FeaturePlace::Role { field, feature } if identifier_key(field) == identifier_key(role) && feature == source_feature)
            })
    {
        return reserve_feature_callees(validated, construction, writer.value(), allocator);
    }
    let source = match &field.kind {
        FieldKind::Category(path) => path_name(path),
        FieldKind::Lex(_) => {
            let (_, vocabulary) =
                canonical_lexical_feature_lowering(validated, construction, role, *source_feature)?;
            path_name(vocabulary)
        }
        FieldKind::Identity(_) => {
            return Err(internal(
                "identity roles cannot provide grammatical features",
            ));
        }
    };
    allocator.reserve(format!(
        "{}_for_{}",
        feature_name(*source_feature),
        snake_case(&source)
    ));
    Ok(())
}

struct RenderLocals {
    whole: Option<syn::Ident>,
    fields: HashMap<String, syn::Ident>,
    category: TokenStream,
}

fn render_arms(
    validated: &SemanticPlan,
    members: &[&crate::Construction],
    root_impl: bool,
    root_names: &HashSet<String>,
    allocator: &LocalAllocator,
    category_value: &TokenStream,
) -> syn::Result<Vec<TokenStream>> {
    members
        .iter()
        .map(|construction| {
            let mut allocator = allocator.clone();
            let variant = ident(&pascal_case(&identifier_key(&construction.name)));
            let element = ident(&identifier_key(&construction.element.name));
            let qualifier = if root_impl {
                quote! { Self }
            } else {
                let category = ident(&path_name(&construction.category));
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
            let whole = private.then(|| allocator.allocate_ident(&construction.name));
            let mut field_locals = HashMap::new();
            let pattern = if construction.element.fields.is_empty() {
                quote! { #qualifier::#variant(#element) }
            } else if private {
                let whole = whole
                    .as_ref()
                    .expect("private construction has a whole local");
                quote! { #qualifier::#variant(#whole) }
            } else {
                let fields = construction
                    .element
                    .fields
                    .iter()
                    .map(|field| {
                        let source = &field.name;
                        let local = allocator.allocate_ident(source);
                        field_locals.insert(identifier_key(source), local.clone());
                        if local == *source {
                            quote! { #source }
                        } else {
                            quote! { #source: #local }
                        }
                    })
                    .collect::<Vec<_>>();
                quote! { #qualifier::#variant(#element { #(#fields),* }) }
            };
            let locals = RenderLocals {
                whole,
                fields: field_locals,
                category: category_value.clone(),
            };
            let statements = render_atoms(validated, construction, &locals, root_names, root_impl)?;
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
    validated: &SemanticPlan,
    construction: &crate::Construction,
    locals: &RenderLocals,
    root_names: &HashSet<String>,
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
        .map(|field| (identifier_key(&field.name), field))
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
                    .get(&identifier_key(role))
                    .ok_or_else(|| internal("resolved role is absent"))?;
                let FieldKind::Category(path) = &field.kind else {
                    return Err(internal("bare role is not a category"));
                };
                let category = path_name(path);
                let helper = render_category_name(&category, root_names.contains(&category));
                let value = field_value(construction, role, locals)?;
                let capability = validated.category_render_capability(&category);
                let agreement = if capability.requires_external_agreement() {
                    role_agreement(validated, construction, role, locals)?
                } else {
                    None
                };
                let context = capability.requires_context().then(|| quote! { context });
                let tail = signature_tail(&[agreement, context]);
                Ok(quote! { #helper(#call_writer, #value #tail); })
            }
            FormAtom::Lex(role) => {
                let field = fields
                    .get(&identifier_key(role))
                    .ok_or_else(|| internal("resolved lexical role is absent"))?;
                let terminal = field_terminal(field);
                let value = field_value(construction, role, locals)?;
                if let Some(vocab) = find_vocab(validated, &terminal)? {
                    let function = ident(&format!(
                        "render_{}",
                        snake_case(&identifier_key(&vocab.name))
                    ));
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
                    .get(&identifier_key(role))
                    .ok_or_else(|| internal("resolved identity role is absent"))?;
                let binding = find_binding(validated, &field_terminal(field))?;
                let value = field_value(construction, role, locals)?;
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
                let agreement = verb_agreement(validated, construction, locals)?;
                Ok(quote! { #method_writer.word(inflect(#variant, #agreement)); })
            }
            FormAtom::Noun(role) => {
                let field = fields
                    .get(&identifier_key(role))
                    .ok_or_else(|| internal("resolved noun role is absent"))?;
                let binding = find_binding(validated, &field_terminal(field))?;
                let Some(RenderBinding::Runtime(function)) = &binding.render else {
                    return Err(internal("noun terminal lacks runtime render binding"));
                };
                let category = path_name(&construction.category);
                let number = ident(&feature_helper("number", &category));
                let category_value = &locals.category;
                let value = field_value(construction, role, locals)?;
                Ok(quote! { #function(#call_writer, #value, #number(#category_value)); })
            }
        })
        .collect()
}

fn role_agreement(
    validated: &SemanticPlan,
    construction: &crate::Construction,
    role: &syn::Ident,
    locals: &RenderLocals,
) -> syn::Result<Option<TokenStream>> {
    let equation = validated.feature_equations(&identifier_key(&construction.name)).iter().find(|equation| {
        matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if identifier_key(field) == identifier_key(role))
    });
    equation
        .map(|equation| {
            feature_expr(
                validated,
                construction,
                equation.value(),
                Feature::Agreement,
                locals,
            )
        })
        .transpose()
}

fn verb_agreement(
    validated: &SemanticPlan,
    construction: &crate::Construction,
    locals: &RenderLocals,
) -> syn::Result<TokenStream> {
    let equations = validated.feature_equations(&identifier_key(&construction.name));
    if let Some(equation) = equations.iter().find(|equation| {
        matches!(equation.target(), FeaturePlace::Role { field, feature: Feature::Agreement } if identifier_key(field) == "verb")
    }) {
        return feature_expr(
            validated,
            construction,
            equation.value(),
            Feature::Agreement,
            locals,
        );
    }
    if equations.iter().any(|equation| {
        matches!(equation.target(), FeaturePlace::Construction(Feature::Agreement))
            && matches!(equation.value(), FeatureExpr::FromRole { role, feature: Feature::Agreement } if identifier_key(role) == "verb")
    }) {
        return Ok(quote! { agreement });
    }
    Err(internal("verb atom lacks validated agreement flow"))
}

fn feature_expr(
    validated: &SemanticPlan,
    construction: &crate::Construction,
    expression: &FeatureExpr,
    _feature: Feature,
    locals: &RenderLocals,
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
                .any(|field| identifier_key(&field.name) == identifier_key(role))
            {
                if let Some(writer) = validated
                    .feature_equations(&identifier_key(&construction.name))
                    .iter()
                    .find(|equation| {
                        matches!(
                            equation.target(),
                            FeaturePlace::Role { field, feature }
                                if identifier_key(field) == identifier_key(role) && feature == source_feature
                        )
                    })
                {
                    return feature_expr(
                        validated,
                        construction,
                        writer.value(),
                        *source_feature,
                        locals,
                    );
                }
                return Ok(quote! { agreement });
            }
            let field = construction
                .element
                .fields
                .iter()
                .find(|field| identifier_key(&field.name) == identifier_key(role))
                .expect("resolved role");
            if matches!(field.kind, FieldKind::Category(_))
                && let Some(writer) = validated
                    .feature_equations(&identifier_key(&construction.name))
                    .iter()
                    .find(|equation| {
                        matches!(
                            equation.target(),
                            FeaturePlace::Role { field, feature }
                                if identifier_key(field) == identifier_key(role) && feature == source_feature
                        )
                    })
            {
                return feature_expr(
                    validated,
                    construction,
                    writer.value(),
                    *source_feature,
                    locals,
                );
            }
            let role_value = field_value(construction, role, locals)?;
            let (source, value) = match &field.kind {
                FieldKind::Category(path) => (path_name(path), role_value),
                FieldKind::Lex(_) => {
                    let (writer_role, vocabulary) = canonical_lexical_feature_lowering(
                        validated,
                        construction,
                        role,
                        *source_feature,
                    )?;
                    let writer_value = field_value(construction, writer_role, locals)?;
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
            let function = ident(&feature_helper(feature_name(*source_feature), &source));
            quote! { #function(#value) }
        }
        FeatureExpr::MatchVocab { role, arms } => {
            let field = construction
                .element
                .fields
                .iter()
                .find(|field| identifier_key(&field.name) == identifier_key(role))
                .ok_or_else(|| internal("match feature role is absent"))?;
            let ty = ident(&field_terminal(field));
            let match_arms = arms.iter().map(|(variant, value)| {
                let variant = variant.value();
                let value = feature_value(*value);
                quote! { #ty::#variant => #value }
            });
            let role_value = field_value(construction, role, locals)?;
            let role_value = copy_value(construction, role, role_value);
            quote! { match #role_value { #(#match_arms),* } }
        }
    })
}

fn canonical_lexical_feature_lowering<'a>(
    validated: &'a SemanticPlan,
    construction: &'a crate::Construction,
    role: &syn::Ident,
    feature: Feature,
) -> syn::Result<(&'a syn::Ident, &'a syn::Path)> {
    let writer = validated
        .feature_equations(&identifier_key(&construction.name))
        .iter()
        .find(|equation| {
            matches!(
                equation.target(),
                FeaturePlace::Role {
                    field,
                    feature: writer_feature,
                } if identifier_key(field) == identifier_key(role) && *writer_feature == feature
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
    if identifier_key(writer_role) != identifier_key(role) {
        return Err(internal(&format!(
            "lexical feature read `{role}.{}` has a mismatched local match writer",
            feature_name(feature),
        )));
    }
    let vocabulary = construction
        .element
        .fields
        .iter()
        .find(|field| identifier_key(&field.name) == identifier_key(writer_role))
        .and_then(|field| match &field.kind {
            FieldKind::Lex(path) => Some(path),
            FieldKind::Category(_) | FieldKind::Identity(_) => None,
        })
        .ok_or_else(|| internal("lexical feature writer is not rooted in a vocabulary role"))?;
    Ok((writer_role, vocabulary))
}

fn emit_feature_helper(
    validated: &SemanticPlan,
    category: &str,
    members: &[&crate::Construction],
    feature: Feature,
) -> syn::Result<GeneratedItem> {
    let function = ident(&feature_helper(feature_name(feature), category));
    let ty = ident(category);
    let mut allocator = LocalAllocator::default();
    for construction in members {
        let equation = validated
            .feature_equations(&identifier_key(&construction.name))
            .iter()
            .find(|equation| {
                matches!(equation.target(), FeaturePlace::Construction(found) if *found == feature)
            })
            .ok_or_else(|| internal("feature helper construction lacks equation"))?;
        reserve_feature_callees(validated, construction, equation.value(), &mut allocator)?;
    }
    let argument = allocator.allocate(&category_argument(category));
    let return_ty = match feature {
        Feature::Agreement => quote! { Agreement },
        Feature::Number => quote! { Number },
    };
    let mut entries: Vec<(TokenStream, String, TokenStream)> = Vec::new();
    for construction in members {
        let mut arm_allocator = allocator.clone();
        let equation = validated.feature_equations(&identifier_key(&construction.name)).iter().find(|equation| {
            matches!(equation.target(), FeaturePlace::Construction(found) if *found == feature)
        }).ok_or_else(|| internal("feature helper construction lacks equation"))?;
        let variant = ident(&pascal_case(&identifier_key(&construction.name)));
        let element = ident(&identifier_key(&construction.element.name));
        if let FeatureExpr::MatchVocab { role, arms: values } = equation.value() {
            let field = construction
                .element
                .fields
                .iter()
                .find(|field| identifier_key(&field.name) == identifier_key(role))
                .ok_or_else(|| internal("feature match role absent"))?;
            let field_type = ident(&field_terminal(field));
            let other_fields = construction
                .element
                .fields
                .iter()
                .filter(|candidate| identifier_key(&candidate.name) != identifier_key(role))
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
            let roles = feature_roles(validated, construction, equation.value())?;
            let (pattern, locals) = feature_constant_pattern(
                validated,
                construction,
                &ty,
                &variant,
                &element,
                &roles,
                &mut arm_allocator,
            )?;
            let value = feature_expr(validated, construction, equation.value(), feature, &locals)?;
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
                DeclarationKey::new(
                    DeclarationKind::Construction,
                    identifier_key(&construction.name),
                )
            })
            .collect(),
    ))
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "keeps the feature-pattern lowering interface uniformly fallible"
)]
fn feature_roles(
    validated: &SemanticPlan,
    construction: &crate::Construction,
    expression: &FeatureExpr,
) -> syn::Result<HashSet<String>> {
    fn collect(
        validated: &SemanticPlan,
        construction: &crate::Construction,
        expression: &FeatureExpr,
        roles: &mut HashSet<String>,
        visiting: &mut HashSet<(String, Feature)>,
    ) -> syn::Result<()> {
        match expression {
            FeatureExpr::Constant(_) => Ok(()),
            FeatureExpr::MatchVocab { role, .. } => {
                roles.insert(identifier_key(role));
                Ok(())
            }
            FeatureExpr::FromRole {
                role,
                feature: source_feature,
            } => {
                let key = (identifier_key(role), *source_feature);
                if !visiting.insert(key.clone()) {
                    return Err(internal("sealed feature flow contains a cycle"));
                }
                let field = construction
                    .element
                    .fields
                    .iter()
                    .find(|field| identifier_key(&field.name) == identifier_key(role));
                let writer = validated
                    .feature_equations(&identifier_key(&construction.name))
                    .iter()
                    .find(|equation| {
                        matches!(equation.target(), FeaturePlace::Role { field, feature } if identifier_key(field) == identifier_key(role) && feature == source_feature)
                    });
                let result = if field.is_none()
                    || field.is_some_and(|field| matches!(field.kind, FieldKind::Category(_)))
                        && writer.is_some()
                {
                    if let Some(writer) = writer {
                        collect(validated, construction, writer.value(), roles, visiting)
                    } else {
                        Ok(())
                    }
                } else {
                    roles.insert(identifier_key(role));
                    Ok(())
                };
                visiting.remove(&key);
                result
            }
        }
    }

    let mut roles = HashSet::new();
    collect(
        validated,
        construction,
        expression,
        &mut roles,
        &mut HashSet::new(),
    )?;
    Ok(roles)
}

fn feature_constant_pattern(
    validated: &SemanticPlan,
    construction: &crate::Construction,
    category: &syn::Ident,
    variant: &syn::Ident,
    element: &syn::Ident,
    roles: &HashSet<String>,
    allocator: &mut LocalAllocator,
) -> syn::Result<(TokenStream, RenderLocals)> {
    if construction.element.fields.is_empty() {
        return Ok((
            quote! { #category::#variant(#element) },
            RenderLocals {
                whole: None,
                fields: HashMap::new(),
                category: TokenStream::new(),
            },
        ));
    }
    if construction.checked.as_ref().is_some_and(|checked| {
        checked.visibilities.iter().any(|visibility| {
            matches!(
                visibility.visibility,
                crate::NonPublicVisibility::Private(_)
            )
        })
    }) {
        let whole = (!roles.is_empty()).then(|| allocator.allocate_ident(&construction.name));
        let pattern = whole.as_ref().map_or_else(
            || quote! { #category::#variant(_) },
            |whole| quote! { #category::#variant(#whole) },
        );
        return Ok((
            pattern,
            RenderLocals {
                whole,
                fields: HashMap::new(),
                category: TokenStream::new(),
            },
        ));
    }
    let mut fields = Vec::new();
    let mut locals = HashMap::new();
    for field in &construction.element.fields {
        let name = &field.name;
        if roles.contains(&identifier_key(name)) {
            let local = allocator.allocate_ident(name);
            locals.insert(identifier_key(name), local.clone());
            if local == *name {
                fields.push(quote! { #name });
            } else {
                fields.push(quote! { #name: #local });
            }
            continue;
        }
        let refined = construction
            .requirements
            .iter()
            .any(|requirement| identifier_key(&requirement.role) == identifier_key(name));
        let pattern = if refined {
            quote! { #name: _ }
        } else if let FieldKind::Lex(path) = &field.kind {
            if let Some(vocab) = find_vocab(validated, &path_name(path))? {
                let ty = ident(&identifier_key(&vocab.name));
                let variants = vocab.variants.iter().map(|variant| {
                    let variant = ident(&identifier_key(&variant.name));
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
    Ok((
        quote! { #category::#variant(#element { #(#fields),* }) },
        RenderLocals {
            whole: None,
            fields: locals,
            category: TokenStream::new(),
        },
    ))
}

fn feature_value(value: FeatureValue) -> TokenStream {
    match value {
        FeatureValue::Bare => quote! { Agreement::Bare },
        FeatureValue::ThirdPersonSingular => quote! { Agreement::ThirdPersonSingular },
        FeatureValue::Singular => quote! { Number::Singular },
        FeatureValue::Plural => quote! { Number::Plural },
    }
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
                .find(|field| identifier_key(&field.name) == identifier_key(role))
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
    validated: &SemanticPlan,
    constructions: &[&'a crate::Construction],
) -> Vec<(String, Vec<&'a crate::Construction>)> {
    let mut result: Vec<(String, Vec<&crate::Construction>)> = Vec::new();
    for (construction, record) in constructions.iter().zip(validated.constructions()) {
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

fn find_vocab<'a>(
    validated: &'a SemanticPlan,
    name: &str,
) -> syn::Result<Option<&'a crate::Vocab>> {
    for terminal in validated.terminals() {
        if let TerminalPlan::Vocab(row) = terminal {
            let vocab = validated.vocab_source(row)?;
            if identifier_key(&vocab.name) == name {
                return Ok(Some(vocab));
            }
        }
    }
    Ok(None)
}

fn find_binding<'a>(
    validated: &'a SemanticPlan,
    name: &str,
) -> syn::Result<&'a crate::TerminalBinding> {
    for terminal in validated.terminals() {
        if let TerminalPlan::Binding(row) = terminal {
            let binding = validated.binding_source(row)?;
            if identifier_key(&binding.name) == name {
                return Ok(binding);
            }
        }
    }
    Err(internal("resolved terminal binding is absent"))
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
    locals: &RenderLocals,
) -> syn::Result<TokenStream> {
    if let Some(accessor) = construction.checked.as_ref().and_then(|checked| {
        checked
            .accessors
            .iter()
            .find(|accessor| identifier_key(&accessor.role) == identifier_key(role))
    }) {
        let whole = locals
            .whole
            .as_ref()
            .ok_or_else(|| internal("checked render accessor lacks its allocated whole local"))?;
        let method = &accessor.method;
        Ok(quote! { #whole.#method() })
    } else if has_private_fields(construction) {
        let whole = locals
            .whole
            .as_ref()
            .ok_or_else(|| internal("private render field lacks its allocated whole local"))?;
        Ok(quote! { &#whole.#role })
    } else {
        let local = locals
            .fields
            .get(&identifier_key(role))
            .ok_or_else(|| internal("public render field lacks its allocated local"))?;
        Ok(quote! { #local })
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
            .any(|accessor| identifier_key(&accessor.role) == identifier_key(role))
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
    ident(&category_renderer(category, root))
}

fn category_argument(category: &str) -> String {
    let snake = snake_case(category);
    if snake.ends_with("_phrase") { "phrase".to_owned() } else { snake }
}
fn render_vocab_argument(name: &syn::Ident) -> String {
    let snake = snake_case(&identifier_key(name));
    snake.strip_suffix("_word").unwrap_or(&snake).to_owned()
}

fn feature_name(feature: Feature) -> &'static str {
    match feature {
        Feature::Agreement => "agreement",
        Feature::Number => "number",
    }
}
fn ident(name: &str) -> syn::Ident {
    emitted_ident(name, Span::call_site())
}
fn path_name(path: &syn::Path) -> String {
    path_key(path)
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
    fn raw_fields_and_keyword_checked_constructions_lower_to_valid_render_locals() {
        let expansion = crate::generate(quote::quote! {
            vocab Marker { One = "marker", }
            lexeme Verbs { Act, }

            construction payload: PayloadBox {
                element PayloadNode { r#payload: lex Marker, }
                form payload = lex(r#payload);
            }
            construction writer_field: WriterField {
                element WriterFieldNode { r#writer: lex Marker, }
                form writer_field = lex(r#writer);
            }
            construction where: Keyword {
                element KeywordNode { marker: lex Marker, }
                checked {
                    visibility marker = private;
                    access marker = marker;
                    constructor = KeywordNode::new(marker);
                }
                derive agreement = Values::Bare;
                form where = lex(marker);
            }
            construction root: Root {
                element RootNode {
                    payload: PayloadBox,
                    writer_field: WriterField,
                    keyword: Keyword,
                }
                derive verb.agreement = keyword.agreement;
                form root = payload writer_field keyword verb(Verbs::Act);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("raw fields and a DSL-keyword checked construction generate without panic");

        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for fragment in [
            "PayloadNode { r#payload : payload }",
            "render_marker (writer , * payload)",
            "WriterFieldNode { r#writer : writer_2 }",
            "render_marker (writer , * writer_2)",
            "Keyword :: Where (where_value)",
            "render_marker (writer , where_value . marker ())",
            "map (| where_value |",
        ] {
            assert!(source.contains(fragment), "missing `{fragment}`: {source}");
        }
        syn::parse2::<syn::File>(expansion.tokens()).expect("the complete expansion reparses");
    }

    #[test]
    fn generated_render_locals_are_hygienic_across_abi_and_helper_names() {
        let expansion = crate::generate(quote::quote! {
            vocab Marker { One = "marker", }
            vocab WriterWord { One = "writer", }
            lexeme Verbs { Act, }
            identity SelfRef {
                value_type = SelfRef;
                lexical = Lexical::SelfRef;
                render context_identity { Full => card_name, }
                build { pattern = BuildValue::SelfRef(value); construct = value; }
                traversal {
                    callback = copy;
                    argument = value;
                    variant Full;
                }
            }

            construction child: Child {
                element ChildNode {}
                derive agreement = Values::Bare;
                form child = "child";
            }
            construction contextual: Action {
                element ContextualAction { agreement: lex Marker, }
                derive agreement = verb.agreement;
                form contextual = lex(agreement) verb(Verbs::Act);
            }
            construction wrapper: RenderChild {
                element Wrapper { child: Child, }
                form wrapper = child;
            }
            construction wrapped: AgreementForChild {
                element Wrapped { child: Child, }
                checked {
                    visibility child = private;
                    access child = child;
                    constructor = Wrapped::new(child);
                }
                derive agreement = child.agreement;
                form wrapped = child;
            }
            construction writer: CheckedRender {
                element CheckedRenderNode { marker: lex Marker, }
                checked {
                    visibility marker = private;
                    access marker = marker;
                    constructor = CheckedRenderNode::new(marker);
                }
                form writer = lex(marker);
            }
            construction root: Root {
                element RootNode {
                    writer: lex Marker,
                    context: identity SelfRef,
                    action: Action,
                    wrapper: RenderChild,
                    wrapped: AgreementForChild,
                    word: lex WriterWord,
                }
                derive action.agreement = Values::Bare;
                derive verb.agreement = wrapped.agreement;
                form root = lex(writer) identity(context) action wrapper wrapped lex(word)
                    verb(Verbs::Act);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("adversarial render names remain valid declaration vocabulary");

        let source = expansion
            .items()
            .iter()
            .filter(|item| match &item.key {
                crate::ItemKey::Impl { trait_name, .. } => trait_name.as_deref() == Some("Render"),
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name,
                } => {
                    name.starts_with("render_")
                        || name.starts_with("agreement_for_")
                        || name.starts_with("number_for_")
                }
                crate::ItemKey::Named { .. } => false,
            })
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for fragment in [
            "ContextualAction { agreement : agreement_2 }",
            "render_marker (writer , * agreement_2)",
            "inflect (Verbs :: Act , agreement)",
            "RootNode { writer : writer_2 , context : context_2",
            "render_marker (& mut writer , * writer_2)",
            "writer . identity (context . card_name ())",
            "match * context_2",
            "fn render_writer_word (writer : & mut Writer , writer_2 : WriterWord)",
            "fn render_render_child (writer : & mut Writer , render_child_2 : & RenderChild)",
            "match render_child_2",
            "render_child (writer , child)",
            "fn agreement_for_agreement_for_child (agreement_for_child_2 : & AgreementForChild)",
            "match agreement_for_child_2",
            "agreement_for_child (wrapped . child ())",
            "CheckedRender :: Writer (writer_2)",
            "render_marker (writer , writer_2 . marker ())",
        ] {
            assert!(
                source.contains(fragment),
                "missing hygienic `{fragment}`: {source}"
            );
        }
        for shadowed in [
            "ContextualAction { agreement })",
            "RootNode { writer , context",
            "writer : & mut Writer , writer : WriterWord",
            "writer : & mut Writer , render_child : & RenderChild",
            "fn agreement_for_agreement_for_child (agreement_for_child : & AgreementForChild)",
            "CheckedRender :: Writer (writer)",
        ] {
            assert!(
                !source.contains(shadowed),
                "shadowed binding survived as `{shadowed}`: {source}",
            );
        }
    }

    #[test]
    fn role_derived_noun_render_uses_declared_feature_helper() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::role_derived_noun_tokens()).unwrap(),
        )
        .unwrap();
        let generated = super::emit(validated.semantic()).expect("role-derived noun render lowers");
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
        let generated = super::emit(validated.semantic()).expect("zero-noun feature helpers lower");
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
        let generated = super::emit(validated.semantic()).expect("two noun render atoms lower");
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
        let plan = validated.semantic();
        let construction = plan
            .constructions()
            .first()
            .map(|row| plan.construction_source(row))
            .expect("construction row")
            .expect("sealed construction source");
        let role = syn::parse_quote!(person);
        let (writer_role, vocabulary) = super::canonical_lexical_feature_lowering(
            plan,
            construction,
            &role,
            crate::feature::Feature::Agreement,
        )
        .expect("the exact role.feature writer is retrievable");
        assert_eq!(writer_role, "person");
        assert_eq!(super::path_name(vocabulary), "Person");
        let missing = super::canonical_lexical_feature_lowering(
            plan,
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
