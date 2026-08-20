use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::emit::LocalAllocator;
use crate::identifier::VISITOR_TRAIT;
use crate::identifier::emitted_ident;
use crate::identifier::key as identifier_key;
use crate::identifier::snake_case;
use crate::model::VisitMode;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::semantic::AccessorMode;
use crate::semantic::AtomPlan;
use crate::semantic::BindingPlan;
use crate::semantic::BindingTraversalRecipe;
use crate::semantic::ConstructionFieldKind;
use crate::semantic::ConstructionFieldPlan;
use crate::semantic::ConstructionPlan;
use crate::semantic::DeclarationNounPlan;
use crate::semantic::LexemePlan;
use crate::semantic::SemanticPlan;
use crate::semantic::SignedDecimalPlan;
use crate::semantic::TerminalPlan;
use crate::semantic::TraversalBranchArmPlan;
use crate::semantic::TraversalCallPlan;
use crate::semantic::TraversalFieldPlan;
use crate::semantic::TraversalValuePlan;
use crate::semantic::VocabPlan;
use crate::semantic::VocabVariantPlan;

pub(crate) fn emit(validated: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let constructions = validated.constructions();
    let categories = category_groups(constructions);
    let mut vocabs = Vec::new();
    let mut lexemes = Vec::new();
    let mut bindings = Vec::new();
    let mut context_identities = Vec::new();
    let mut signed_decimal = None;
    let mut declaration_noun = None;
    for terminal in validated.terminals() {
        match terminal {
            TerminalPlan::Vocab(row) => vocabs.push(row),
            TerminalPlan::Lexeme(row) => lexemes.push(row),
            TerminalPlan::Binding(row) => bindings.push(row),
            TerminalPlan::ContextIdentity(row) => context_identities.push(row),
            TerminalPlan::SignedDecimal(row) => signed_decimal = Some(row),
            TerminalPlan::DeclarationNoun(row) => declaration_noun = Some(row),
        }
    }
    let containers = bindings
        .iter()
        .copied()
        .filter(|binding| {
            matches!(
                binding.traversal().recipe(),
                BindingTraversalRecipe::Branches(_)
            )
        })
        .collect::<Vec<_>>();
    let copy_bindings = bindings
        .iter()
        .copied()
        .filter(|binding| binding.traversal().mode() == VisitMode::Copy)
        .collect::<Vec<_>>();
    let borrowed_bindings = bindings
        .iter()
        .copied()
        .filter(|binding| {
            binding.traversal().mode() == VisitMode::Borrowed
                && !matches!(
                    binding.traversal().recipe(),
                    BindingTraversalRecipe::Branches(_)
                )
        })
        .collect::<Vec<_>>();

    let terminal_visitors = TerminalVisitors {
        containers: &containers,
        vocabs: &vocabs,
        copy_bindings: &copy_bindings,
        context_identities: &context_identities,
        lexemes: &lexemes,
        borrowed_bindings: &borrowed_bindings,
        signed_decimal,
        declaration_noun,
    };
    let trait_item = emit_trait(&categories, constructions, &terminal_visitors)?;
    let mut items = vec![trait_item];
    for (category, members) in &categories {
        items.push(emit_category_walker(category, members));
    }
    for binding in &containers {
        items.push(emit_binding_walker(binding)?);
    }
    for construction in constructions {
        items.push(emit_construction_walker(validated, construction)?);
    }
    for vocab in &vocabs {
        items.push(emit_enum_walker(
            vocab.name_ident(),
            vocab.variants().iter().map(VocabVariantPlan::name),
            DeclarationKind::Vocab,
        ));
    }
    for binding in &copy_bindings {
        items.push(emit_binding_walker(binding)?);
    }
    for identity in &context_identities {
        items.push(emit_enum_walker(
            identity.ident(),
            identity
                .arms()
                .iter()
                .map(crate::semantic::ContextIdentityArmPlan::variant),
            DeclarationKind::Identity,
        ));
    }
    for lexeme in &lexemes {
        items.push(emit_enum_walker(
            lexeme.name_ident(),
            lexeme.variants().iter(),
            DeclarationKind::Lexeme,
        ));
    }
    for binding in &borrowed_bindings {
        items.push(emit_binding_walker(binding)?);
    }
    if let Some(codec) = signed_decimal {
        items.push(emit_signed_decimal_sign_walker(codec));
        items.push(emit_signed_decimal_walker(codec));
    }
    if let Some(codec) = declaration_noun {
        items.push(emit_declaration_noun_value_walker(codec));
        items.push(emit_declaration_noun_walker(codec));
    }
    Ok(items)
}

struct TerminalVisitors<'a> {
    containers: &'a [&'a BindingPlan],
    vocabs: &'a [&'a VocabPlan],
    copy_bindings: &'a [&'a BindingPlan],
    context_identities: &'a [&'a crate::semantic::ContextIdentityPlan],
    lexemes: &'a [&'a LexemePlan],
    borrowed_bindings: &'a [&'a BindingPlan],
    signed_decimal: Option<&'a SignedDecimalPlan>,
    declaration_noun: Option<&'a DeclarationNounPlan>,
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "keeps every visitor phase finalizer on one fallible interface"
)]
fn emit_trait(
    categories: &[(String, Vec<&ConstructionPlan>)],
    constructions: &[ConstructionPlan],
    terminals: &TerminalVisitors<'_>,
) -> syn::Result<GeneratedItem> {
    let visitor = ident(VISITOR_TRAIT);
    let mut methods = Vec::new();
    for (category, _) in categories {
        methods.push(default_method(category, &category_argument(category)));
    }
    for binding in terminals.containers {
        methods.push(default_method(binding.name(), &binding_argument(binding)));
    }
    for construction in constructions {
        methods.push(default_method(
            construction.element_type(),
            &snake_case(construction.element_type()),
        ));
    }
    for vocab in terminals.vocabs {
        methods.push(noop_method(vocab.name(), VisitMode::Copy));
    }
    for binding in terminals.copy_bindings {
        methods.push(noop_method(binding.name(), VisitMode::Copy));
    }
    for identity in terminals.context_identities {
        methods.push(noop_method(identity.name(), VisitMode::Copy));
    }
    for lexeme in terminals.lexemes {
        methods.push(noop_method(lexeme.name(), VisitMode::Copy));
    }
    for binding in terminals.borrowed_bindings {
        methods.push(noop_method(binding.name(), VisitMode::Borrowed));
    }
    if let Some(codec) = terminals.signed_decimal {
        methods.push(noop_method(&codec.sign_type().to_string(), VisitMode::Copy));
        methods.push(noop_method(codec.codec_name(), VisitMode::Borrowed));
    }
    if let Some(codec) = terminals.declaration_noun {
        methods.push(default_method(codec.codec_name(), codec.codec_name()));
        methods.push(default_method(
            &codec.declaration_value_ident().to_string(),
            &crate::identifier::snake_case(&codec.declaration_value_ident().to_string()),
        ));
    }
    if terminals.declaration_noun.is_some()
        || constructions.iter().any(|construction| {
            construction
                .atoms()
                .iter()
                .any(|atom| matches!(atom, AtomPlan::OpenDeclaration(_)))
        })
    {
        methods.push(quote! {
            fn visit_declaration(
                &mut self,
                _declaration: &::macro_ron::v2::DeclarationIdentity,
            ) {}
        });
    }

    let mut leaf_origins = Vec::new();
    let mut seen = HashSet::new();
    for binding in terminals
        .containers
        .iter()
        .chain(terminals.copy_bindings)
        .chain(terminals.borrowed_bindings)
    {
        for leaf in binding.traversal().leaf_callbacks() {
            if seen.insert(identifier_key(leaf.name())) {
                let name = leaf.name();
                let ty = leaf.value_type();
                let name_string = identifier_key(name);
                let callback_subject = name_string.strip_prefix("visit_").unwrap_or(&name_string);
                let mut allocator = LocalAllocator::default();
                allocator.reserve("self");
                let argument = allocator.allocate(&format!("_{}", leaf_argument(callback_subject)));
                let signature = match leaf.mode() {
                    VisitMode::Copy => quote! { #argument: #ty },
                    VisitMode::Borrowed => quote! { #argument: &#ty },
                };
                methods.push(quote! { fn #name(&mut self, #signature) {} });
                leaf_origins.push(DeclarationKey::new(binding_kind(binding), binding.name()));
            }
        }
    }
    let mut origins = categories
        .iter()
        .flat_map(|(_, members)| {
            members.iter().map(|construction| {
                DeclarationKey::new(
                    DeclarationKind::Construction,
                    construction.construction_id(),
                )
            })
        })
        .collect::<Vec<_>>();
    origins.extend(
        terminals
            .containers
            .iter()
            .map(|binding| binding_origin(binding, constructions)),
    );
    if let Some(codec) = terminals.signed_decimal {
        origins.push(codec.origin().clone());
    }
    if let Some(codec) = terminals.declaration_noun {
        origins.push(codec.origin().clone());
    }
    origins.extend(constructions.iter().map(|construction| {
        DeclarationKey::new(
            DeclarationKind::Construction,
            construction.construction_id(),
        )
    }));
    origins.extend(
        terminals
            .vocabs
            .iter()
            .map(|vocab| DeclarationKey::new(DeclarationKind::Vocab, vocab.name())),
    );
    origins.extend(
        terminals
            .copy_bindings
            .iter()
            .map(|binding| binding_origin(binding, constructions)),
    );
    origins.extend(
        terminals
            .context_identities
            .iter()
            .map(|identity| identity.origin().clone()),
    );
    origins.extend(
        terminals
            .lexemes
            .iter()
            .map(|lexeme| DeclarationKey::new(DeclarationKind::Lexeme, lexeme.name())),
    );
    origins.extend(
        terminals
            .borrowed_bindings
            .iter()
            .map(|binding| binding_origin(binding, constructions)),
    );
    origins.extend(leaf_origins);
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Trait,
            name: VISITOR_TRAIT.to_owned(),
        },
        quote! { pub trait #visitor { #(#methods)* } },
        origins,
    ))
}

fn emit_declaration_noun_value_walker(codec: &DeclarationNounPlan) -> GeneratedItem {
    let ty = codec.declaration_value_ident();
    let function = ident(&format!("walk_{}", snake_case(&ty.to_string())));
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, declaration: &#ty) {
                visitor.visit_declaration(declaration.id());
            }
        },
        vec![codec.origin().clone()],
    )
}

fn emit_declaration_noun_walker(codec: &DeclarationNounPlan) -> GeneratedItem {
    let ty = codec.codec_ident();
    let declaration = codec.declaration_value_ident();
    let closed = codec.closed_lexeme();
    let function = ident(&format!("walk_{}", snake_case(codec.codec_name())));
    let closed_walker = ident(&format!("walk_{}", snake_case(&closed.to_string())));
    let declaration_callback = ident(&format!("visit_{}", snake_case(&declaration.to_string())));
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, noun: &#ty) {
                match noun {
                    #ty::Lexeme(lexeme) => #closed_walker(visitor, *lexeme),
                    #ty::Declaration(declaration) => visitor.#declaration_callback(declaration),
                }
            }
        },
        vec![codec.origin().clone()],
    )
}

fn emit_signed_decimal_sign_walker(codec: &SignedDecimalPlan) -> GeneratedItem {
    let sign = codec.sign_type();
    let function = ident(&format!("walk_{}", snake_case(&sign.to_string())));
    let callback = ident(&format!("visit_{}", snake_case(&sign.to_string())));
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, sign: #sign) {
                visitor.#callback(sign);
            }
        },
        vec![codec.origin().clone()],
    )
}

fn emit_signed_decimal_walker(codec: &SignedDecimalPlan) -> GeneratedItem {
    let ty = codec.codec_ident();
    let function = ident(&format!("walk_{}", snake_case(codec.codec_name())));
    let sign_callback = ident(&format!(
        "visit_{}",
        snake_case(&codec.sign_type().to_string())
    ));
    let callback = ident(&format!("visit_{}", snake_case(codec.codec_name())));
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, number: &#ty) {
                visitor.#sign_callback(number.sign);
                visitor.#callback(number);
            }
        },
        vec![codec.origin().clone()],
    )
}

fn default_method(type_name: &str, argument: &str) -> TokenStream {
    let ty = ident(type_name);
    let method = ident(&format!("visit_{}", snake_case(type_name)));
    let walker = ident(&format!("walk_{}", snake_case(type_name)));
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
    let method = ident(&format!("visit_{}", snake_case(type_name)));
    let mut allocator = LocalAllocator::default();
    allocator.reserve("self");
    let argument = allocator.allocate(&format!("_{}", leaf_argument(type_name)));
    match mode {
        VisitMode::Copy => quote! { fn #method(&mut self, #argument: #ty) {} },
        VisitMode::Borrowed => quote! { fn #method(&mut self, #argument: &#ty) {} },
    }
}

fn emit_category_walker(category: &str, members: &[&ConstructionPlan]) -> GeneratedItem {
    let ty = ident(category);
    let function = ident(&format!("walk_{}", snake_case(category)));
    let mut allocator = LocalAllocator::default();
    allocator.reserve("visitor");
    let argument = allocator.allocate(&category_argument(category));
    let arms = members.iter().map(|construction| {
        let mut arm_allocator = allocator.clone();
        let variant = ident(construction.category_variant());
        let payload = arm_allocator.allocate(&snake_case(construction.element_type()));
        let callback = ident(&format!(
            "visit_{}",
            snake_case(construction.element_type())
        ));
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
                    construction.construction_id(),
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
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
) -> syn::Result<GeneratedItem> {
    let type_name = construction.element_type().to_owned();
    let ty = ident(&type_name);
    let function = ident(&format!("walk_{}", snake_case(&type_name)));
    let mut allocator = LocalAllocator::default();
    allocator.reserve("visitor");
    for atom in construction.atoms() {
        let terminal = match atom {
            AtomPlan::Lex { terminal, .. }
            | AtomPlan::Identity { terminal, .. }
            | AtomPlan::VerbFixed { terminal, .. } => Some(terminal.clone()),
            AtomPlan::Literal(_)
            | AtomPlan::Category { .. }
            | AtomPlan::Noun { .. }
            | AtomPlan::OpenDeclaration(_) => None,
        };
        if let Some(terminal) = terminal {
            allocator.reserve(format!("walk_{}", snake_case(&terminal)));
        }
    }
    let argument = allocator.allocate(&snake_case(&type_name));
    let whole = needs_whole_value(construction);
    let mut field_locals = HashMap::new();
    let destructure = if construction.fields().is_empty() {
        quote! { let #ty = #argument; }
    } else if whole {
        TokenStream::new()
    } else {
        let fields = construction
            .fields()
            .iter()
            .map(|field| {
                let source = field.name();
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
        .fields()
        .iter()
        .map(|field| (field.name_key(), field))
        .collect::<HashMap<_, _>>();
    let mut calls = Vec::new();
    for atom in construction.atoms() {
        let call = match atom {
            AtomPlan::Literal(_) => None,
            AtomPlan::Category { role, category } => {
                let field = fields
                    .get(role)
                    .ok_or_else(|| internal("walker category role absent"))?;
                if field.kind() != ConstructionFieldKind::Category {
                    return Err(internal("walker bare role is not category"));
                }
                let callback = ident(&format!("visit_{}", snake_case(category)));
                let value = field_value(construction, role, &argument, &field_locals)?;
                Some(quote! { visitor.#callback(#value); })
            }
            AtomPlan::Lex { role, terminal } => {
                let field = fields
                    .get(role)
                    .ok_or_else(|| internal("walker lex role absent"))?;
                let walker = ident(&format!("walk_{}", snake_case(terminal)));
                let value = field_value(construction, role, &argument, &field_locals)?;
                let copy = terminal_mode(validated, terminal)? == VisitMode::Copy;
                Some(if copy {
                    let value = copy_value(field, value);
                    quote! { #walker(visitor, #value); }
                } else {
                    quote! { #walker(visitor, #value); }
                })
            }
            AtomPlan::Identity { role, terminal } => {
                let field = fields
                    .get(role)
                    .ok_or_else(|| internal("walker identity role absent"))?;
                let walker = ident(&format!("walk_{}", snake_case(terminal)));
                let value = field_value(construction, role, &argument, &field_locals)?;
                let copy = terminal_mode(validated, terminal)? == VisitMode::Copy;
                Some(if copy {
                    let value = copy_value(field, value);
                    quote! { #walker(visitor, #value); }
                } else {
                    quote! { #walker(visitor, #value); }
                })
            }
            AtomPlan::Noun { role, terminal } => {
                fields
                    .get(role)
                    .ok_or_else(|| internal("walker noun role absent"))?;
                let callback = ident(&format!("visit_{}", snake_case(terminal)));
                let value = field_value(construction, role, &argument, &field_locals)?;
                Some(quote! { visitor.#callback(#value); })
            }
            AtomPlan::VerbFixed { terminal, path, .. } => {
                let walker = ident(&format!("walk_{}", snake_case(terminal)));
                Some(quote! { #walker(visitor, #path); })
            }
            AtomPlan::OpenDeclaration(open) => {
                let kind = crate::emit::declaration_kind(open.kind());
                let name = syn::LitStr::new(open.name(), Span::call_site());
                Some(quote! {
                    visitor.visit_declaration(
                        &::macro_ron::v2::DeclarationIdentity::new(#kind, #name),
                    );
                })
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
            construction.construction_id(),
        )],
    ))
}

fn emit_enum_walker<'a>(
    authored_ty: &syn::Ident,
    variants: impl Iterator<Item = &'a syn::Ident>,
    kind: DeclarationKind,
) -> GeneratedItem {
    let type_name = identifier_key(authored_ty);
    let ty = ident(&type_name);
    let function = ident(&format!("walk_{}", snake_case(&type_name)));
    let mut allocator = LocalAllocator::default();
    allocator.reserve("visitor");
    let argument = allocator.allocate(&leaf_argument(&type_name));
    let callback = ident(&format!("visit_{}", snake_case(&type_name)));
    let arms = variants
        .map(|variant| {
            let variant = ident(&identifier_key(variant));
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
        vec![DeclarationKey::new(kind, type_name)],
    )
}

fn emit_binding_walker(binding: &BindingPlan) -> syn::Result<GeneratedItem> {
    let type_name = identifier_key(binding.value_type_name());
    let ty = ident(&type_name);
    let function = ident(&format!("walk_{}", snake_case(&type_name)));
    let source_argument = binding_argument(binding);
    let mut allocator = LocalAllocator::default();
    allocator.reserve("visitor");
    match binding.traversal().recipe() {
        BindingTraversalRecipe::Branches(branches) => {
            for call in branches
                .iter()
                .flat_map(|branch| branch.arms().iter().map(TraversalBranchArmPlan::call))
            {
                reserve_callback(&mut allocator, call);
            }
        }
        BindingTraversalRecipe::Calls(calls) => {
            for call in calls {
                reserve_callback(&mut allocator, call);
            }
        }
        BindingTraversalRecipe::Variants(_) => {}
    }
    let argument = allocator.allocate(&source_argument);
    let mut bindings = HashMap::from([(source_argument, argument.clone())]);
    let mode = binding.traversal().mode();
    let signature = match mode {
        VisitMode::Copy => quote! { #argument: #ty },
        VisitMode::Borrowed => quote! { #argument: &#ty },
    };
    let mut field_patterns = Vec::new();
    for field in binding.traversal().fields() {
        let source = field.name();
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
    let body = match binding.traversal().recipe() {
        BindingTraversalRecipe::Branches(branches) => {
            let branches = branches
                .iter()
                .map(|branch| -> syn::Result<TokenStream> {
                    let value = lower_traversal_expr(branch.value(), &bindings)?;
                    let arms = branch
                        .arms()
                        .iter()
                        .map(|arm| -> syn::Result<TokenStream> {
                            let mut arm_allocator = allocator.clone();
                            let mut arm_bindings = bindings.clone();
                            let variant = arm.variant();
                            let local = arm_allocator.allocate_ident(arm.binding());
                            arm_bindings.insert(identifier_key(arm.binding()), local.clone());
                            let call = emit_call_expr(arm.call(), &arm_bindings)?;
                            Ok(quote! { #variant(#local) => #call })
                        })
                        .collect::<syn::Result<Vec<_>>>()?;
                    Ok(quote! { match #value { #(#arms,)* } })
                })
                .collect::<syn::Result<Vec<_>>>()?;
            quote! { #destructure #(#branches)* }
        }
        BindingTraversalRecipe::Variants(variants) => {
            let callback = ident(&format!("visit_{}", snake_case(&type_name)));
            let variants = variants.iter().map(|variant| {
                crate::emit::call_match_arm(
                    &quote! { #ty::#variant },
                    &quote! { visitor.#callback(#ty::#variant) },
                    8,
                )
            });
            quote! { match #argument { #(#variants,)* } }
        }
        BindingTraversalRecipe::Calls(calls) => {
            let fields = binding
                .traversal()
                .fields()
                .iter()
                .map(TraversalFieldPlan::name)
                .collect::<Vec<_>>();
            let used_fields = fields
                .iter()
                .map(|field| {
                    calls
                        .iter()
                        .any(|call| call.mentions(&identifier_key(field)))
                })
                .collect::<Vec<_>>();
            let mut ignored_fields = vec![false; fields.len()];
            let mut statements = Vec::new();
            for (call_index, call) in calls.iter().enumerate() {
                let next_used_field = calls[call_index..]
                    .iter()
                    .find_map(|candidate| {
                        fields
                            .iter()
                            .position(|field| candidate.mentions(&identifier_key(field)))
                    })
                    .unwrap_or(fields.len());
                for field_index in 0..next_used_field {
                    if !used_fields[field_index] && !ignored_fields[field_index] {
                        let field = bindings
                            .get(&identifier_key(fields[field_index]))
                            .ok_or_else(|| internal("traversal field lacks its allocated local"))?;
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
                        .ok_or_else(|| internal("traversal field lacks its allocated local"))?;
                    statements.push(quote! { let _ = #field; });
                }
            }
            quote! { #destructure #(#statements)* }
        }
    };
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! { pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, #signature) { #body } },
        vec![DeclarationKey::new(binding_kind(binding), binding.name())],
    ))
}

fn reserve_callback(allocator: &mut LocalAllocator, call: &TraversalCallPlan) {
    if call.callback().leading_colon.is_none() && call.callback().segments.len() == 1 {
        allocator.reserve_ident(&call.callback().segments[0].ident);
    }
}

fn emit_call(
    call: &TraversalCallPlan,
    bindings: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    let call = emit_call_expr(call, bindings)?;
    Ok(quote! { #call; })
}

fn emit_call_expr(
    call: &TraversalCallPlan,
    bindings: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    let value = lower_traversal_expr(call.value(), bindings)?;
    let argument = match call.mode() {
        VisitMode::Copy => quote! { *#value },
        VisitMode::Borrowed => quote! { #value },
    };
    let segments = call.callback().segments.iter().collect::<Vec<_>>();
    if segments.len() == 2 && identifier_key(&segments[0].ident) == "visitor" {
        let method = &segments[1].ident;
        Ok(quote! { visitor.#method(#argument) })
    } else {
        let callback = call.callback();
        Ok(quote! { #callback(visitor, #argument) })
    }
}

fn lower_traversal_expr(
    expression: &TraversalValuePlan,
    bindings: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    match expression {
        TraversalValuePlan::Root(source) => {
            let local = bindings.get(source).ok_or_else(|| {
                internal("validated traversal expression lacks its allocated root local")
            })?;
            Ok(quote! { #local })
        }
        TraversalValuePlan::Field { base, member } => {
            let base = lower_traversal_expr(base, bindings)?;
            Ok(quote! { #base.#member })
        }
        TraversalValuePlan::Parenthesized(inner) => {
            let inner = lower_traversal_expr(inner, bindings)?;
            Ok(quote! { (#inner) })
        }
    }
}

fn terminal_mode(validated: &SemanticPlan, terminal: &str) -> syn::Result<VisitMode> {
    for planned in validated.terminals() {
        match planned {
            TerminalPlan::Vocab(row) if row.name() == terminal => {
                return Ok(VisitMode::Copy);
            }
            TerminalPlan::Lexeme(row) if row.name() == terminal => {
                return Ok(VisitMode::Copy);
            }
            TerminalPlan::Binding(row) if row.name() == terminal => {
                return Ok(row.traversal().mode());
            }
            TerminalPlan::ContextIdentity(row) if row.name() == terminal => {
                return Ok(VisitMode::Copy);
            }
            TerminalPlan::SignedDecimal(row) if row.codec_name() == terminal => {
                return Ok(VisitMode::Borrowed);
            }
            TerminalPlan::DeclarationNoun(row) if row.codec_name() == terminal => {
                return Ok(VisitMode::Borrowed);
            }
            TerminalPlan::Vocab(_)
            | TerminalPlan::Lexeme(_)
            | TerminalPlan::Binding(_)
            | TerminalPlan::ContextIdentity(_)
            | TerminalPlan::SignedDecimal(_)
            | TerminalPlan::DeclarationNoun(_) => {}
        }
    }
    Err(internal("terminal traversal mode is absent"))
}

fn binding_origin(binding: &BindingPlan, _constructions: &[ConstructionPlan]) -> DeclarationKey {
    DeclarationKey::new(binding_kind(binding), binding.name())
}

fn binding_kind(binding: &BindingPlan) -> DeclarationKind {
    match binding.kind() {
        crate::TerminalBindingKind::Codec => DeclarationKind::Codec,
        crate::TerminalBindingKind::Identity => DeclarationKind::Identity,
    }
}

fn category_groups(constructions: &[ConstructionPlan]) -> Vec<(String, Vec<&ConstructionPlan>)> {
    let mut result: Vec<(String, Vec<&ConstructionPlan>)> = Vec::new();
    for construction in constructions {
        if let Some((_, members)) = result
            .iter_mut()
            .find(|(name, _)| name == construction.category())
        {
            members.push(construction);
        } else {
            result.push((construction.category().to_owned(), vec![construction]));
        }
    }
    result
}

fn binding_argument(binding: &BindingPlan) -> String {
    binding.traversal().argument().to_owned()
}
fn field_value(
    construction: &ConstructionPlan,
    role: &str,
    whole: &syn::Ident,
    fields: &HashMap<String, syn::Ident>,
) -> syn::Result<TokenStream> {
    let field = construction.field(role)?;
    if field.accessor_mode().is_some() {
        let method = field.name();
        Ok(quote! { #whole.#method() })
    } else if needs_whole_value(construction) {
        let role = field.name();
        Ok(quote! { &#whole.#role })
    } else {
        let local = fields
            .get(role)
            .ok_or_else(|| internal("public walker field lacks its allocated local"))?;
        Ok(quote! { #local })
    }
}

fn copy_value(field: &ConstructionFieldPlan, value: TokenStream) -> TokenStream {
    if field.accessor_mode() == Some(AccessorMode::Copy) {
        value
    } else {
        quote! { *#value }
    }
}

fn needs_whole_value(construction: &ConstructionPlan) -> bool {
    construction
        .fields()
        .iter()
        .any(|field| field.accessor_mode().is_some())
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
fn ident(name: &str) -> syn::Ident {
    emitted_ident(name, Span::call_site())
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
    fn invariant_mixed_fields_visit_through_sealed_access_modes_in_form_order() {
        let plan = crate::test_support::invariant_access_semantic_plan();
        let items = super::emit(&plan).expect("mixed invariant visitors emit");
        let visitor = items
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { kind: crate::NamedKind::Trait, name } if name == "Visitor"))
            .expect("Visitor trait")
            .tokens
            .to_string();
        assert!(
            visitor.contains("fn visit_node (& mut self , node : & Node)"),
            "borrowed category callback signature: {visitor}",
        );
        let walker = items
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Named {
                        kind: crate::NamedKind::Function,
                        name,
                    } if name == "walk_walk_mode"
                )
            })
            .expect("WalkMode walker");
        let syn::Item::Fn(walker) = syn::parse2::<syn::File>(walker.tokens.clone())
            .expect("WalkMode walker parses")
            .items
            .into_iter()
            .next()
            .expect("WalkMode walker item")
        else {
            panic!("WalkMode walker is a function");
        };
        let source = walker.to_token_stream().to_string();
        let calls = walker
            .block
            .stmts
            .iter()
            .filter_map(|statement| match statement {
                syn::Stmt::Expr(expression, _) => Some(expression.to_token_stream().to_string()),
                syn::Stmt::Local(_) | syn::Stmt::Item(_) | syn::Stmt::Macro(_) => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            calls,
            [
                "walk_plain (visitor , * & walk_mode_2 . plain)",
                "walk_mode (visitor , walk_mode_2 . mode ())",
                "visitor . visit_node (walk_mode_2 . child ())",
                "walk_self_reference_spelling (visitor , walk_mode_2 . spelling ())",
                "walk_marker (visitor , * & walk_mode_2 . visitor)",
            ],
            "access levels and callbacks must follow declaration/form order",
        );
        assert!(source.contains("walk_mode_2 : & WalkMode"), "{source}");
        assert!(!source.contains("let WalkMode {"), "{source}");
        assert!(!source.contains("* walk_mode_2 . mode ()"), "{source}");
        assert!(!source.contains("& walk_mode_2 . child ()"), "{source}");
        assert!(!source.contains("walk_mode_2 . mode)"), "{source}");
        assert!(!source.contains("walk_mode_2 . child)"), "{source}");
        assert!(!source.contains("walk_mode_2 . spelling)"), "{source}");
    }

    #[test]
    fn generated_walker_locals_are_hygienic_across_abi_and_helper_names() {
        let expansion = crate::generate(quote::quote! {
            vocab Marker { One = "marker", }
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme VisitorLexeme using EnglishVerb { Act = "act", }
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
            construction visitor_category: VISITOR {
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
            "fn walk_visitor < V : Visitor + ? Sized > (visitor : & mut V , visitor_2 : & VISITOR)",
            "match visitor_2",
            "fn walk_token < V : Visitor + ? Sized > (visitor : & mut V , visitor_2 : & Token)",
            "visitor . visit_token (visitor_2)",
            "fn walk_walk_marker < V : Visitor + ? Sized > (visitor : & mut V , walk_marker_2 : & WalkMarker)",
            "let WalkMarker { marker } = walk_marker_2",
            "walk_marker (visitor , * marker)",
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
    fn temporary_checked_fields_follow_direct_walker_paths() {
        let expansion = crate::test_support::access_modes_expansion();
        let expected: &[(&str, &[&str])] = &[
            ("walk_public_identity", &["walk_flag (visitor , * flag)"]),
            (
                "walk_mixed_access",
                &[
                    "walk_flag (visitor , * hidden)",
                    "visitor . visit_child (child)",
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
    fn generated_morphology_keeps_lexeme_visitor_callbacks() {
        let expansion = crate::test_support::generated_morphology_expansion();
        let syn::Item::Trait(visitor) = named(&expansion, "Visitor") else {
            panic!("Visitor is a trait")
        };
        let callbacks = visitor
            .items
            .iter()
            .filter_map(|item| {
                let syn::TraitItem::Fn(method) = item else {
                    return None;
                };
                matches!(
                    method.sig.ident.to_string().as_str(),
                    "visit_verb_lexeme" | "visit_noun_lexeme"
                )
                .then(|| {
                    (
                        method.sig.ident.to_string(),
                        method.sig.inputs[1].to_token_stream().to_string(),
                    )
                })
            })
            .collect::<Vec<_>>();
        assert_eq!(
            callbacks,
            [
                ("visit_verb_lexeme".into(), "_verb : VerbLexeme".into(),),
                ("visit_noun_lexeme".into(), "_noun : NounLexeme".into(),),
            ],
        );
        assert!(matches!(
            named(&expansion, "walk_verb_lexeme"),
            syn::Item::Fn(_)
        ));
        assert!(matches!(
            named(&expansion, "walk_noun_lexeme"),
            syn::Item::Fn(_)
        ));
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
