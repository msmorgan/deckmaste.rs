use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::format_ident;
use quote::quote;

use crate::ValidatedDeclarations;
use crate::emit::LocalAllocator;
use crate::identifier::key as identifier_key;
use crate::identifier::path_key;
use crate::model::Declaration;
use crate::model::FieldKind;
use crate::model::FormAtom;
use crate::model::TraversalCall;
use crate::model::VisitMode;
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
            Declaration::Construction(value) => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    let categories = category_groups(validated, &constructions);
    let vocabs = validated
        .raw()
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Vocab(value) => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    let lexemes = validated
        .raw()
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Lexeme(value) => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    let bindings = validated
        .raw()
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Codec(value) | Declaration::Identity(value) => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    let containers = bindings
        .iter()
        .copied()
        .filter(|binding| !binding.traversal.branches.is_empty())
        .collect::<Vec<_>>();
    let copy_bindings = bindings
        .iter()
        .copied()
        .filter(|binding| binding.traversal.callback_mode == Some(VisitMode::Copy))
        .collect::<Vec<_>>();
    let borrowed_bindings = bindings
        .iter()
        .copied()
        .filter(|binding| {
            binding.traversal.callback_mode == Some(VisitMode::Borrowed)
                && binding.traversal.branches.is_empty()
        })
        .collect::<Vec<_>>();

    let trait_item = emit_trait(
        &categories,
        &containers,
        &constructions,
        &vocabs,
        &copy_bindings,
        &lexemes,
        &borrowed_bindings,
    )?;
    let mut items = vec![trait_item];
    for (category, members) in &categories {
        items.push(emit_category_walker(category, members));
    }
    for binding in &containers {
        items.push(emit_binding_walker(binding)?);
    }
    for construction in &constructions {
        items.push(emit_construction_walker(validated, construction)?);
    }
    for vocab in &vocabs {
        items.push(emit_enum_walker(
            &vocab.name,
            vocab.variants.iter().map(|variant| &variant.name),
            DeclarationKind::Vocab,
        ));
    }
    for binding in &copy_bindings {
        items.push(emit_binding_walker(binding)?);
    }
    for lexeme in &lexemes {
        items.push(emit_enum_walker(
            &lexeme.name,
            lexeme.variants.iter(),
            DeclarationKind::Lexeme,
        ));
    }
    for binding in &borrowed_bindings {
        items.push(emit_binding_walker(binding)?);
    }
    Ok(items)
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "keeps every visitor phase finalizer on one fallible interface"
)]
fn emit_trait(
    categories: &[(String, Vec<&crate::Construction>)],
    containers: &[&crate::TerminalBinding],
    constructions: &[&crate::Construction],
    vocabs: &[&crate::Vocab],
    copy_bindings: &[&crate::TerminalBinding],
    lexemes: &[&crate::Lexeme],
    borrowed_bindings: &[&crate::TerminalBinding],
) -> syn::Result<GeneratedItem> {
    let mut methods = Vec::new();
    for (category, _) in categories {
        methods.push(default_method(category, &category_argument(category)));
    }
    for binding in containers {
        methods.push(default_method(
            &identifier_key(&binding.name),
            &binding_argument(binding),
        ));
    }
    for construction in constructions {
        methods.push(default_method(
            &identifier_key(&construction.element.name),
            &snake_case(&identifier_key(&construction.element.name)),
        ));
    }
    for vocab in vocabs {
        methods.push(noop_method(&identifier_key(&vocab.name), VisitMode::Copy));
    }
    for binding in copy_bindings {
        methods.push(noop_method(&identifier_key(&binding.name), VisitMode::Copy));
    }
    for lexeme in lexemes {
        methods.push(noop_method(&identifier_key(&lexeme.name), VisitMode::Copy));
    }
    for binding in borrowed_bindings {
        methods.push(noop_method(
            &identifier_key(&binding.name),
            VisitMode::Borrowed,
        ));
    }

    let mut leaf_origins = Vec::new();
    let mut seen = HashSet::new();
    for binding in containers
        .iter()
        .chain(copy_bindings)
        .chain(borrowed_bindings)
    {
        for leaf in &binding.traversal.leaf_callbacks {
            if seen.insert(identifier_key(&leaf.name)) {
                let name = &leaf.name;
                let ty = &leaf.value_type;
                let name_string = identifier_key(name);
                let callback_subject = name_string.strip_prefix("visit_").unwrap_or(&name_string);
                let mut allocator = LocalAllocator::default();
                allocator.reserve("self");
                let argument = allocator.allocate(&format!("_{}", leaf_argument(callback_subject)));
                let signature = match leaf.mode {
                    VisitMode::Copy => quote! { #argument: #ty },
                    VisitMode::Borrowed => quote! { #argument: &#ty },
                };
                methods.push(quote! { fn #name(&mut self, #signature) {} });
                leaf_origins.push(DeclarationKey::new(
                    binding_kind(binding),
                    identifier_key(&binding.name),
                ));
            }
        }
    }
    let mut origins = categories
        .iter()
        .flat_map(|(_, members)| {
            members.iter().map(|construction| {
                DeclarationKey::new(
                    DeclarationKind::Construction,
                    identifier_key(&construction.name),
                )
            })
        })
        .collect::<Vec<_>>();
    origins.extend(
        containers
            .iter()
            .map(|binding| binding_origin(binding, constructions)),
    );
    origins.extend(constructions.iter().map(|construction| {
        DeclarationKey::new(
            DeclarationKind::Construction,
            identifier_key(&construction.name),
        )
    }));
    origins.extend(
        vocabs
            .iter()
            .map(|vocab| DeclarationKey::new(DeclarationKind::Vocab, identifier_key(&vocab.name))),
    );
    origins.extend(
        copy_bindings
            .iter()
            .map(|binding| binding_origin(binding, constructions)),
    );
    origins.extend(
        lexemes.iter().map(|lexeme| {
            DeclarationKey::new(DeclarationKind::Lexeme, identifier_key(&lexeme.name))
        }),
    );
    origins.extend(
        borrowed_bindings
            .iter()
            .map(|binding| binding_origin(binding, constructions)),
    );
    origins.extend(leaf_origins);
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Trait,
            name: "Visitor".to_owned(),
        },
        quote! { pub trait Visitor { #(#methods)* } },
        origins,
    ))
}

fn default_method(type_name: &str, argument: &str) -> TokenStream {
    let ty = ident(type_name);
    let method = format_ident!("visit_{}", snake_case(type_name));
    let walker = format_ident!("walk_{}", snake_case(type_name));
    let mut allocator = LocalAllocator::default();
    allocator.reserve("self");
    allocator.reserve(walker.to_string());
    let argument = allocator.allocate(argument);
    quote! {
        fn #method(&mut self, #argument: &#ty) { #walker(self, #argument); }
    }
}

fn noop_method(type_name: &str, mode: VisitMode) -> TokenStream {
    let ty = ident(type_name);
    let method = format_ident!("visit_{}", snake_case(type_name));
    let mut allocator = LocalAllocator::default();
    allocator.reserve("self");
    let argument = allocator.allocate(&format!("_{}", leaf_argument(type_name)));
    match mode {
        VisitMode::Copy => quote! { fn #method(&mut self, #argument: #ty) {} },
        VisitMode::Borrowed => quote! { fn #method(&mut self, #argument: &#ty) {} },
    }
}

fn emit_category_walker(category: &str, members: &[&crate::Construction]) -> GeneratedItem {
    let ty = ident(category);
    let function = format_ident!("walk_{}", snake_case(category));
    let mut allocator = LocalAllocator::default();
    allocator.reserve("visitor");
    let argument = allocator.allocate(&category_argument(category));
    let arms = members.iter().map(|construction| {
        let mut arm_allocator = allocator.clone();
        let variant = ident(&pascal_case(&identifier_key(&construction.name)));
        let payload =
            arm_allocator.allocate(&snake_case(&identifier_key(&construction.element.name)));
        let callback = format_ident!(
            "visit_{}",
            snake_case(&identifier_key(&construction.element.name))
        );
        crate::emit::call_match_arm(
            &quote! { #ty::#variant(#payload) },
            &quote! { visitor.#callback(#payload) },
            8,
        )
    });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! { pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, #argument: &#ty) { match #argument { #(#arms,)* } } },
        members
            .iter()
            .map(|construction| {
                DeclarationKey::new(
                    DeclarationKind::Construction,
                    identifier_key(&construction.name),
                )
            })
            .collect(),
    )
}

#[allow(
    clippy::too_many_lines,
    reason = "construction walkers lower the closed form order and every typed access mode together"
)]
fn emit_construction_walker(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
) -> syn::Result<GeneratedItem> {
    let ty = &construction.element.name;
    let function = format_ident!("walk_{}", snake_case(&identifier_key(ty)));
    let mut allocator = LocalAllocator::default();
    allocator.reserve("visitor");
    for atom in &construction.form.atoms {
        let terminal = match atom {
            FormAtom::Lex(role) | FormAtom::Identity(role) => construction
                .element
                .fields
                .iter()
                .find(|field| identifier_key(&field.name) == identifier_key(role))
                .map(field_terminal),
            FormAtom::Verb(crate::VerbOperand::Fixed(path)) => path
                .segments
                .iter()
                .rev()
                .nth(1)
                .map(|segment| identifier_key(&segment.ident)),
            FormAtom::Literal(_)
            | FormAtom::Role(_)
            | FormAtom::Noun(_)
            | FormAtom::Verb(crate::VerbOperand::Projected(_)) => None,
        };
        if let Some(terminal) = terminal {
            allocator.reserve(format!("walk_{}", snake_case(&terminal)));
        }
    }
    let argument = allocator.allocate(&snake_case(&identifier_key(ty)));
    let private = construction.checked.as_ref().is_some_and(|checked| {
        checked.visibilities.iter().any(|visibility| {
            matches!(
                visibility.visibility,
                crate::NonPublicVisibility::Private(_)
            )
        })
    });
    let mut field_locals = HashMap::new();
    let destructure = if construction.element.fields.is_empty() {
        quote! { let #ty = #argument; }
    } else if private {
        TokenStream::new()
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
        quote! { let #ty { #(#fields),* } = #argument; }
    };
    let fields = construction
        .element
        .fields
        .iter()
        .map(|field| (identifier_key(&field.name), field))
        .collect::<HashMap<_, _>>();
    let mut calls = Vec::new();
    for atom in &construction.form.atoms {
        let call = match atom {
            FormAtom::Literal(_) => None,
            FormAtom::Role(role) => {
                let field = fields
                    .get(&identifier_key(role))
                    .ok_or_else(|| internal("walker category role absent"))?;
                let FieldKind::Category(path) = &field.kind else {
                    return Err(internal("walker bare role is not category"));
                };
                let callback = format_ident!("visit_{}", snake_case(&path_name(path)));
                let value = field_value(construction, role, &argument, &field_locals)?;
                Some(quote! { visitor.#callback(#value); })
            }
            FormAtom::Lex(role) => {
                let field = fields
                    .get(&identifier_key(role))
                    .ok_or_else(|| internal("walker lex role absent"))?;
                let terminal = field_terminal(field);
                let walker = format_ident!("walk_{}", snake_case(&terminal));
                let value = field_value(construction, role, &argument, &field_locals)?;
                let copy = terminal_mode(validated, &terminal)? == VisitMode::Copy;
                Some(if copy {
                    if has_accessor(construction, role) {
                        quote! { #walker(visitor, #value); }
                    } else {
                        quote! { #walker(visitor, *#value); }
                    }
                } else {
                    quote! { #walker(visitor, #value); }
                })
            }
            FormAtom::Identity(role) => {
                let field = fields
                    .get(&identifier_key(role))
                    .ok_or_else(|| internal("walker identity role absent"))?;
                let terminal = field_terminal(field);
                let walker = format_ident!("walk_{}", snake_case(&terminal));
                let value = field_value(construction, role, &argument, &field_locals)?;
                let copy = terminal_mode(validated, &terminal)? == VisitMode::Copy;
                Some(if copy {
                    if has_accessor(construction, role) {
                        quote! { #walker(visitor, #value); }
                    } else {
                        quote! { #walker(visitor, *#value); }
                    }
                } else {
                    quote! { #walker(visitor, #value); }
                })
            }
            FormAtom::Noun(role) => {
                let field = fields
                    .get(&identifier_key(role))
                    .ok_or_else(|| internal("walker noun role absent"))?;
                let terminal = field_terminal(field);
                let callback = format_ident!("visit_{}", snake_case(&terminal));
                let value = field_value(construction, role, &argument, &field_locals)?;
                Some(quote! { visitor.#callback(#value); })
            }
            FormAtom::Verb(crate::VerbOperand::Fixed(path)) => {
                let terminal = &path
                    .segments
                    .iter()
                    .rev()
                    .nth(1)
                    .ok_or_else(|| internal("fixed verb path lacks terminal"))?
                    .ident;
                let terminal = identifier_key(terminal);
                let walker = format_ident!("walk_{}", snake_case(&terminal));
                Some(quote! { #walker(visitor, #path); })
            }
            FormAtom::Verb(crate::VerbOperand::Projected(_)) => {
                return Err(internal("projected verb walker is unsupported"));
            }
        };
        if let Some(call) = call {
            calls.push(call);
        }
    }
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! { pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, #argument: &#ty) { #destructure #(#calls)* } },
        vec![DeclarationKey::new(
            DeclarationKind::Construction,
            identifier_key(&construction.name),
        )],
    ))
}

fn emit_enum_walker<'a>(
    ty: &syn::Ident,
    variants: impl Iterator<Item = &'a syn::Ident>,
    kind: DeclarationKind,
) -> GeneratedItem {
    let function = format_ident!("walk_{}", snake_case(&identifier_key(ty)));
    let mut allocator = LocalAllocator::default();
    allocator.reserve("visitor");
    let argument = allocator.allocate(&leaf_argument(&identifier_key(ty)));
    let callback = format_ident!("visit_{}", snake_case(&identifier_key(ty)));
    let arms = variants
        .map(|variant| {
            crate::emit::call_match_arm(
                &quote! { #ty::#variant },
                &quote! { visitor.#callback(#ty::#variant) },
                8,
            )
        })
        .collect::<Vec<_>>();
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! { pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, #argument: #ty) { match #argument { #(#arms,)* } } },
        vec![DeclarationKey::new(kind, identifier_key(ty))],
    )
}

fn emit_binding_walker(binding: &crate::TerminalBinding) -> syn::Result<GeneratedItem> {
    let ty = simple_type_ident(&binding.value_type)?;
    let type_name = identifier_key(ty);
    let function = format_ident!("walk_{}", snake_case(&type_name));
    let source_argument = binding_argument(binding);
    let mut allocator = LocalAllocator::default();
    allocator.reserve("visitor");
    for call in binding.traversal.calls.iter().chain(
        binding
            .traversal
            .branches
            .iter()
            .flat_map(|branch| branch.arms.iter().map(|arm| &arm.call)),
    ) {
        reserve_callback(&mut allocator, call);
    }
    let argument = allocator.allocate(&source_argument);
    let mut bindings = HashMap::from([(source_argument, argument.clone())]);
    let mode = binding
        .traversal
        .callback_mode
        .ok_or_else(|| internal("traversal binding lacks callback mode"))?;
    let signature = match mode {
        VisitMode::Copy => quote! { #argument: #ty },
        VisitMode::Borrowed => quote! { #argument: &#ty },
    };
    let mut field_patterns = Vec::new();
    for field in &binding.traversal.fields {
        let source = &field.name;
        let local = allocator.allocate_ident(source);
        bindings.insert(identifier_key(source), local.clone());
        if local == *source {
            field_patterns.push(quote! { #source });
        } else {
            field_patterns.push(quote! { #source: #local });
        }
    }
    let destructure = if field_patterns.is_empty() {
        TokenStream::new()
    } else {
        quote! { let #ty { #(#field_patterns),* } = #argument; }
    };
    let body = if !binding.traversal.branches.is_empty() {
        let branches = binding
            .traversal
            .branches
            .iter()
            .map(|branch| -> syn::Result<TokenStream> {
                let value = lower_traversal_expr(&branch.value, &bindings)?;
                let arms = branch
                    .arms
                    .iter()
                    .map(|arm| -> syn::Result<TokenStream> {
                        let mut arm_allocator = allocator.clone();
                        let mut arm_bindings = bindings.clone();
                        let variant = &arm.variant;
                        let local = arm_allocator.allocate_ident(&arm.binding);
                        arm_bindings.insert(identifier_key(&arm.binding), local.clone());
                        let call = emit_call_expr(&arm.call, &arm_bindings)?;
                        Ok(quote! { #variant(#local) => #call })
                    })
                    .collect::<syn::Result<Vec<_>>>()?;
                Ok(quote! { match #value { #(#arms,)* } })
            })
            .collect::<syn::Result<Vec<_>>>()?;
        quote! { #destructure #(#branches)* }
    } else if !binding.traversal.variants.is_empty() {
        let callback = format_ident!("visit_{}", snake_case(&type_name));
        let variants = binding.traversal.variants.iter().map(|variant| {
            crate::emit::call_match_arm(
                &quote! { #ty::#variant },
                &quote! { visitor.#callback(#ty::#variant) },
                8,
            )
        });
        quote! { match #argument { #(#variants,)* } }
    } else {
        let fields = binding
            .traversal
            .fields
            .iter()
            .map(|field| &field.name)
            .collect::<Vec<_>>();
        let used_fields = fields
            .iter()
            .map(|field| {
                binding
                    .traversal
                    .calls
                    .iter()
                    .any(|call| expr_mentions(&call.value, field))
            })
            .collect::<Vec<_>>();
        let mut ignored_fields = vec![false; fields.len()];
        let mut statements = Vec::new();
        for (call_index, call) in binding.traversal.calls.iter().enumerate() {
            let next_used_field = binding.traversal.calls[call_index..]
                .iter()
                .find_map(|candidate| {
                    fields
                        .iter()
                        .position(|field| expr_mentions(&candidate.value, field))
                })
                .unwrap_or(fields.len());
            for field_index in 0..next_used_field {
                if !used_fields[field_index] && !ignored_fields[field_index] {
                    let field = bindings
                        .get(&identifier_key(fields[field_index]))
                        .expect("traversal field has an allocated local");
                    statements.push(quote! { let _ = #field; });
                    ignored_fields[field_index] = true;
                }
            }
            statements.push(emit_call(call, &bindings)?);
        }
        for (field_index, field) in fields.iter().enumerate() {
            if !used_fields[field_index] && !ignored_fields[field_index] {
                let field = bindings
                    .get(&identifier_key(field))
                    .expect("traversal field has an allocated local");
                statements.push(quote! { let _ = #field; });
            }
        }
        quote! { #destructure #(#statements)* }
    };
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! { pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, #signature) { #body } },
        vec![DeclarationKey::new(
            binding_kind(binding),
            identifier_key(&binding.name),
        )],
    ))
}

fn reserve_callback(allocator: &mut LocalAllocator, call: &TraversalCall) {
    if call.callback.leading_colon.is_none() && call.callback.segments.len() == 1 {
        allocator.reserve_ident(&call.callback.segments[0].ident);
    }
}

fn emit_call(
    call: &TraversalCall,
    bindings: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    let call = emit_call_expr(call, bindings)?;
    Ok(quote! { #call; })
}

fn emit_call_expr(
    call: &TraversalCall,
    bindings: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    let value = lower_traversal_expr(&call.value, bindings)?;
    let argument = match call.mode {
        VisitMode::Copy => quote! { *#value },
        VisitMode::Borrowed => quote! { #value },
    };
    let segments = call.callback.segments.iter().collect::<Vec<_>>();
    if segments.len() == 2 && identifier_key(&segments[0].ident) == "visitor" {
        let method = &segments[1].ident;
        Ok(quote! { visitor.#method(#argument) })
    } else {
        let callback = &call.callback;
        Ok(quote! { #callback(visitor, #argument) })
    }
}

fn lower_traversal_expr(
    expression: &syn::Expr,
    bindings: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    match expression {
        syn::Expr::Path(path)
            if path.qself.is_none()
                && path.path.leading_colon.is_none()
                && path.path.segments.len() == 1 =>
        {
            let source = &path.path.segments[0].ident;
            let local = bindings.get(&identifier_key(source)).ok_or_else(|| {
                internal("validated traversal expression lacks its allocated root local")
            })?;
            Ok(quote! { #local })
        }
        syn::Expr::Field(field) => {
            let base = lower_traversal_expr(&field.base, bindings)?;
            let member = &field.member;
            Ok(quote! { #base.#member })
        }
        syn::Expr::Paren(paren) => {
            let inner = lower_traversal_expr(&paren.expr, bindings)?;
            Ok(quote! { (#inner) })
        }
        _ => Err(internal(
            "validated traversal expression is outside the closed field-path grammar",
        )),
    }
}

fn expr_mentions(expression: &syn::Expr, ident: &syn::Ident) -> bool {
    expression
        .to_token_stream()
        .into_iter()
        .any(|token| matches!(token, proc_macro2::TokenTree::Ident(found) if identifier_key(&found) == identifier_key(ident)))
}

fn terminal_mode(validated: &ValidatedDeclarations, terminal: &str) -> syn::Result<VisitMode> {
    if validated.raw().declarations.iter().any(
        |declaration| matches!(declaration, Declaration::Vocab(vocab) if identifier_key(&vocab.name) == terminal),
    ) {
        return Ok(VisitMode::Copy);
    }
    validated
        .raw()
        .declarations
        .iter()
        .find_map(|declaration| match declaration {
            Declaration::Codec(binding) | Declaration::Identity(binding)
                if identifier_key(&binding.name) == terminal =>
            {
                binding.traversal.callback_mode
            }
            _ => None,
        })
        .ok_or_else(|| internal("terminal traversal mode is absent"))
}

fn binding_origin(
    binding: &crate::TerminalBinding,
    _constructions: &[&crate::Construction],
) -> DeclarationKey {
    DeclarationKey::new(binding_kind(binding), identifier_key(&binding.name))
}

fn binding_kind(binding: &crate::TerminalBinding) -> DeclarationKind {
    match binding.kind {
        crate::TerminalBindingKind::Codec => DeclarationKind::Codec,
        crate::TerminalBindingKind::Identity => DeclarationKind::Identity,
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

fn field_terminal(field: &crate::Field) -> String {
    match &field.kind {
        FieldKind::Category(path) | FieldKind::Lex(path) | FieldKind::Identity(path) => {
            path_name(path)
        }
    }
}
fn binding_argument(binding: &crate::TerminalBinding) -> String {
    binding.traversal.argument.as_ref().map_or_else(
        || leaf_argument(&identifier_key(&binding.name)),
        identifier_key,
    )
}
fn field_value(
    construction: &crate::Construction,
    role: &syn::Ident,
    whole: &syn::Ident,
    fields: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    if let Some(accessor) = construction.checked.as_ref().and_then(|checked| {
        checked
            .accessors
            .iter()
            .find(|accessor| identifier_key(&accessor.role) == identifier_key(role))
    }) {
        let method = &accessor.method;
        Ok(quote! { #whole.#method() })
    } else if has_private_fields(construction) {
        Ok(quote! { &#whole.#role })
    } else {
        let local = fields
            .get(&identifier_key(role))
            .ok_or_else(|| internal("public walker field lacks its allocated local"))?;
        Ok(quote! { #local })
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
fn category_argument(category: &str) -> String {
    snake_case(category)
}
fn leaf_argument(name: &str) -> String {
    let snake = snake_case(name);
    if snake.ends_with("_word") {
        "word".to_owned()
    } else if snake.ends_with("_spelling") {
        "spelling".to_owned()
    } else if let Some(prefix) = snake.strip_suffix("_lexeme") {
        prefix.to_owned()
    } else if snake.ends_with("_identity") {
        "identity".to_owned()
    } else if snake.ends_with("_number") {
        "number".to_owned()
    } else {
        snake
    }
}
fn simple_type_ident(ty: &syn::Type) -> syn::Result<&syn::Ident> {
    match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| &segment.ident)
            .ok_or_else(|| internal("empty binding type")),
        _ => Err(internal("binding type must be path")),
    }
}
fn path_name(path: &syn::Path) -> String {
    path_key(path)
}
fn ident(name: &str) -> syn::Ident {
    syn::parse_str(name).expect("validated visitor identifier")
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
        clippy::collapsible_if,
        clippy::items_after_statements,
        clippy::too_many_lines,
        reason = "literal full-surface structural oracles are intentionally table-dense"
    )]
    use quote::ToTokens;
    use syn::visit::Visit;

    #[test]
    fn generated_walker_locals_are_hygienic_across_abi_and_helper_names() {
        let expansion = crate::generate(quote::quote! {
            vocab Marker { One = "marker", }
            lexeme VisitorLexeme { Act, }
            codec Token {
                atom = lex;
                value_type = Token;
                lexical = Lexical::Token;
                render = render_token;
                build { pattern = BuildValue::Token(token); construct = token; }
                traversal {
                    callback = borrowed;
                    argument = visitor;
                    call visitor::visit_token(borrowed(visitor));
                }
            }
            codec FieldToken {
                value_type = FieldToken;
                traversal {
                    callback = borrowed;
                    argument = field_token;
                    field visitor: Marker;
                    call walk_marker(copy(visitor));
                }
            }
            codec BranchToken {
                value_type = BranchToken;
                traversal {
                    callback = borrowed;
                    argument = branch_token;
                    variant Marker;
                    match branch_token {
                        BranchToken::Marker(walk_marker: Marker) =>
                            walk_marker(copy(walk_marker)),
                    }
                }
            }
            codec RawBranchToken {
                value_type = RawBranchToken;
                traversal {
                    callback = borrowed;
                    argument = r#payload;
                    variant Marker;
                    match r#payload {
                        RawBranchToken::Marker(r#walk_marker: Marker) =>
                            walk_marker(copy(r#walk_marker)),
                    }
                }
            }

            construction visit_node: Root {
                element VisitNode { visitor: lex Marker, }
                form visit_node = lex(visitor);
            }
            construction visitor_category: Visitor {
                element VisitorNode {}
                form visitor_category = "visitor";
            }
            construction checked_marker: CheckedMarker {
                element WalkMarker { marker: lex Marker, }
                checked {
                    visibility marker = private;
                    access marker = marker;
                    constructor = WalkMarker::new(marker);
                }
                form checked_marker = lex(marker);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("adversarial walker names remain valid declaration vocabulary");

        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for fragment in [
            "let VisitNode { visitor : visitor_2 } = visit_node",
            "walk_marker (visitor , * visitor_2)",
            "fn walk_visitor_lexeme < V : Visitor + ? Sized > (visitor : & mut V , visitor_2 : VisitorLexeme)",
            "fn walk_visitor < V : Visitor + ? Sized > (visitor : & mut V , visitor_2 : & Visitor)",
            "match visitor_2",
            "fn walk_token < V : Visitor + ? Sized > (visitor : & mut V , visitor_2 : & Token)",
            "visitor . visit_token (visitor_2)",
            "fn walk_walk_marker < V : Visitor + ? Sized > (visitor : & mut V , walk_marker_2 : & WalkMarker)",
            "walk_marker (visitor , walk_marker_2 . marker ())",
            "let FieldToken { visitor : visitor_2 } = field_token",
            "BranchToken :: Marker (walk_marker_2) => walk_marker (visitor , * walk_marker_2)",
            "fn walk_raw_branch_token < V : Visitor + ? Sized > (visitor : & mut V , payload : & RawBranchToken)",
            "match payload",
            "RawBranchToken :: Marker (walk_marker_2) => walk_marker (visitor , * walk_marker_2)",
        ] {
            assert!(
                source.contains(fragment),
                "missing hygienic `{fragment}`: {source}"
            );
        }
        for shadowed in [
            "let VisitNode { visitor }",
            "visitor : & mut V , visitor : VisitorLexeme",
            "visitor : & mut V , visitor : & Visitor",
            "visitor : & mut V , visitor : & Token",
            "visitor : & mut V , walk_marker : & WalkMarker",
            "FieldToken { visitor }",
            "BranchToken :: Marker (walk_marker) => walk_marker",
        ] {
            assert!(
                !source.contains(shadowed),
                "shadowed binding survived as `{shadowed}`: {source}",
            );
        }
    }

    #[test]
    fn copy_identity_and_mixed_checked_fields_use_explicit_access_modes() {
        let expansion = crate::test_support::access_modes_expansion();
        let expected: &[(&str, &[&str])] = &[
            ("walk_public_identity", &["walk_flag (visitor , * flag)"]),
            (
                "walk_mixed_access",
                &[
                    "walk_flag (visitor , mixed_access . hidden ())",
                    "visitor . visit_child (\u{26} mixed_access . child)",
                ],
            ),
        ];
        for &(name, expected_calls) in expected {
            let syn::Item::Fn(item) = named(&expansion, name) else {
                panic!("{name} is a function");
            };
            let calls = item
                .block
                .stmts
                .iter()
                .filter_map(|statement| match statement {
                    syn::Stmt::Expr(expression, _) => {
                        Some(expression.to_token_stream().to_string())
                    }
                    syn::Stmt::Local(_) | syn::Stmt::Item(_) | syn::Stmt::Macro(_) => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(calls, expected_calls, "{name} access/pass lowering");
        }
    }

    #[test]
    fn visitor_leaf_callback_provenance_uses_the_declared_binding_kind() {
        let expansion = crate::generate(quote::quote! {
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = runtime;
                    leaf visit_runtime_text: str = borrowed;
                    field text: str;
                    call visitor::visit_runtime_text(borrowed(text));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a traversal-only codec with a leaf callback is valid");
        let visitor = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { kind: crate::NamedKind::Trait, name } if name == "Visitor"))
            .expect("Visitor item");
        let runtime_origins = visitor
            .origins
            .iter()
            .filter(|origin| origin.name() == "Runtime")
            .map(crate::DeclarationKey::kind)
            .collect::<Vec<_>>();
        assert_eq!(
            runtime_origins,
            [crate::DeclarationKind::Codec, crate::DeclarationKind::Codec],
            "both the codec callback and its leaf callback retain codec provenance",
        );
    }

    #[test]
    fn supported_binding_body_modes_emit_typed_walkers() {
        let expansion = crate::generate(quote::quote! {
            codec Mode {
                value_type = Mode;
                traversal { callback = copy; argument = mode; variant One; }
            }
            codec Record {
                value_type = Record;
                traversal {
                    callback = borrowed;
                    argument = record;
                    field mode: Mode;
                    call walk_mode(copy(mode));
                    call visitor::visit_record(borrowed(record));
                }
            }
            codec Sum {
                value_type = Sum;
                traversal {
                    callback = borrowed;
                    argument = sum;
                    variant Record;
                    match sum {
                        Sum::Record(record: Record) => walk_record(borrowed(record)),
                    }
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("all three supported body/pass-mode pairs validate");
        let expected = [
            syn::parse_quote! {
                pub fn walk_mode<V: Visitor + ?Sized>(visitor: &mut V, mode: Mode) {
                    match mode { Mode::One => visitor.visit_mode(Mode::One), }
                }
            },
            syn::parse_quote! {
                pub fn walk_record<V: Visitor + ?Sized>(visitor: &mut V, record: &Record) {
                    let Record { mode } = record;
                    walk_mode(visitor, *mode);
                    visitor.visit_record(record);
                }
            },
            syn::parse_quote! {
                pub fn walk_sum<V: Visitor + ?Sized>(visitor: &mut V, sum: &Sum) {
                    match sum { Sum::Record(record) => walk_record(visitor, record), }
                }
            },
        ];
        for (name, expected) in ["walk_mode", "walk_record", "walk_sum"]
            .into_iter()
            .zip(expected)
        {
            assert_eq!(named(&expansion, name), expected, "{name} body/mode tokens");
        }
    }

    #[test]
    fn representative_walkers_follow_declared_form_order() {
        let expansion = crate::test_support::representative_expansion();
        let cases: &[(&str, &[&str])] = &[
            ("walk_chain", &["visit_node", "walk_words"]),
            ("walk_action_element", &["walk_verbs", "visit_node"]),
        ];
        for &(name, expected) in cases {
            let syn::Item::Fn(item) = named(&expansion, name) else {
                panic!("{name} is a function");
            };
            let mut calls = Calls::default();
            calls.visit_block(&item.block);
            assert_eq!(calls.0, expected, "{name} declared-form preorder");
        }
    }

    #[test]
    fn synthetic_projection_has_exact_visitor_callbacks_and_walker_order() {
        let expansion = crate::test_support::synthetic_projection_expansion();
        let visitor_items = expansion
            .items()
            .iter()
            .filter(|item| match &item.key {
                crate::ItemKey::Named { kind, name } => {
                    (*kind == crate::NamedKind::Trait && name == "Visitor")
                        || (*kind == crate::NamedKind::Function && name.starts_with("walk_"))
                }
                crate::ItemKey::Impl { .. } => false,
            })
            .count();
        assert_eq!(visitor_items, 19);

        let syn::Item::Trait(visitor) = named(&expansion, "Visitor") else {
            panic!("Visitor is a trait")
        };
        let callbacks = visitor
            .items
            .iter()
            .map(|item| {
                let syn::TraitItem::Fn(method) = item else {
                    panic!("Visitor contains only methods")
                };
                (
                    method.sig.ident.to_string(),
                    method.sig.inputs[1].to_token_stream().to_string(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            callbacks,
            [
                ("visit_expr".into(), "expr : & Expr".into()),
                ("visit_predicate".into(), "predicate : & Predicate".into()),
                ("visit_tag".into(), "tag : & Tag".into()),
                ("visit_document".into(), "document : & Document".into()),
                ("visit_resource".into(), "resource : & Resource".into()),
                ("visit_leaf_node".into(), "leaf_node : & LeafNode".into()),
                (
                    "visit_nested_node".into(),
                    "nested_node : & NestedNode".into(),
                ),
                (
                    "visit_action_node".into(),
                    "action_node : & ActionNode".into()
                ),
                ("visit_idle_node".into(), "idle_node : & IdleNode".into()),
                ("visit_solo_tag".into(), "solo_tag : & SoloTag".into()),
                (
                    "visit_document_node".into(),
                    "document_node : & DocumentNode".into(),
                ),
                ("visit_mode".into(), "_mode : Mode".into()),
                ("visit_marker".into(), "_marker : Marker".into()),
                ("visit_handle".into(), "_handle : Handle".into()),
                (
                    "visit_object_stem".into(),
                    "_object_stem : ObjectStem".into(),
                ),
                (
                    "visit_action_stem".into(),
                    "_action_stem : ActionStem".into(),
                ),
                ("visit_pair".into(), "_pair : & Pair".into()),
                ("visit_record".into(), "_record : & Record".into()),
                ("visit_record_label".into(), "_record_label : & str".into(),),
            ],
        );

        let cases: &[(&str, &[&str])] = &[
            ("walk_leaf_node", &["walk_mode", "visit_resource"]),
            ("walk_nested_node", &["visit_expr", "walk_marker"]),
            ("walk_action_node", &["walk_action_stem"]),
            (
                "walk_document_node",
                &["visit_expr", "visit_predicate", "walk_handle", "walk_pair"],
            ),
            ("walk_resource", &["walk_object_stem", "walk_record"]),
            ("walk_pair", &["visit_pair"]),
            ("walk_record", &["visit_record", "visit_record_label"]),
        ];
        for &(name, expected) in cases {
            let syn::Item::Fn(item) = named(&expansion, name) else {
                panic!("{name} is a function");
            };
            let mut calls = Calls::default();
            calls.visit_block(&item.block);
            assert_eq!(calls.0, expected, "{name} callback order");
        }

        assert_eq!(
            named(&expansion, "walk_handle"),
            syn::parse_quote! {
                pub fn walk_handle<V: Visitor + ?Sized>(
                    visitor: &mut V,
                    handle: Handle
                ) {
                    match handle {
                        Handle::Primary => visitor.visit_handle(Handle::Primary),
                        Handle::Alias => visitor.visit_handle(Handle::Alias),
                    }
                }
            },
            "copy identity walker remains a literal independent oracle",
        );
    }
    fn named(expansion: &crate::Expansion, name: &str) -> syn::Item {
        let item = expansion.items().iter().find(|item| matches!(&item.key, crate::ItemKey::Named { name: found, .. } if found == name)).unwrap_or_else(|| panic!("{name} exists"));
        syn::parse2::<syn::File>(item.tokens.clone())
            .unwrap()
            .items
            .into_iter()
            .next()
            .unwrap()
    }

    #[derive(Default)]
    struct Calls(Vec<String>);
    impl<'ast> Visit<'ast> for Calls {
        fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
            if let syn::Expr::Path(path) = &*node.func {
                if let Some(segment) = path.path.segments.last() {
                    let name = segment.ident.to_string();
                    if name.starts_with("visit_") || name.starts_with("walk_") {
                        self.0.push(name);
                    }
                }
            }
            syn::visit::visit_expr_call(self, node);
        }
        fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
            let name = node.method.to_string();
            if name.starts_with("visit_") || name.starts_with("walk_") {
                self.0.push(name);
            }
            syn::visit::visit_expr_method_call(self, node);
        }
    }
}
