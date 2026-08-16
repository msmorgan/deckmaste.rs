use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::format_ident;
use quote::quote;

use crate::ValidatedDeclarations;
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
        methods.push(default_method(category, category_argument(category)));
    }
    for binding in containers {
        methods.push(default_method(
            &binding.name.to_string(),
            binding_argument(binding),
        ));
    }
    for construction in constructions {
        methods.push(default_method(
            &construction.element.name.to_string(),
            ident(&snake_case(&construction.element.name.to_string())),
        ));
    }
    for vocab in vocabs {
        methods.push(noop_method(&vocab.name.to_string(), VisitMode::Copy));
    }
    for binding in copy_bindings {
        methods.push(noop_method(&binding.name.to_string(), VisitMode::Copy));
    }
    for lexeme in lexemes {
        methods.push(noop_method(&lexeme.name.to_string(), VisitMode::Copy));
    }
    for binding in borrowed_bindings {
        methods.push(noop_method(&binding.name.to_string(), VisitMode::Borrowed));
    }

    let mut leaf_origins = Vec::new();
    let mut seen = HashSet::new();
    for binding in containers
        .iter()
        .chain(copy_bindings)
        .chain(borrowed_bindings)
    {
        for leaf in &binding.traversal.leaf_callbacks {
            if seen.insert(leaf.name.to_string()) {
                let name = &leaf.name;
                let ty = &leaf.value_type;
                let name_string = name.to_string();
                let callback_subject = name_string.strip_prefix("visit_").unwrap_or(&name_string);
                let argument = ident(&format!("_{}", leaf_argument(callback_subject)));
                let signature = match leaf.mode {
                    VisitMode::Copy => quote! { #argument: #ty },
                    VisitMode::Borrowed => quote! { #argument: &#ty },
                };
                methods.push(quote! { fn #name(&mut self, #signature) {} });
                leaf_origins.push(DeclarationKey::new(
                    binding_kind(binding),
                    binding.name.to_string(),
                ));
            }
        }
    }
    let mut origins = categories
        .iter()
        .flat_map(|(_, members)| {
            members.iter().map(|construction| {
                DeclarationKey::new(DeclarationKind::Construction, construction.name.to_string())
            })
        })
        .collect::<Vec<_>>();
    origins.extend(
        containers
            .iter()
            .map(|binding| binding_origin(binding, constructions)),
    );
    origins.extend(constructions.iter().map(|construction| {
        DeclarationKey::new(DeclarationKind::Construction, construction.name.to_string())
    }));
    origins.extend(
        vocabs
            .iter()
            .map(|vocab| DeclarationKey::new(DeclarationKind::Vocab, vocab.name.to_string())),
    );
    origins.extend(
        copy_bindings
            .iter()
            .map(|binding| binding_origin(binding, constructions)),
    );
    origins.extend(
        lexemes
            .iter()
            .map(|lexeme| DeclarationKey::new(DeclarationKind::Lexeme, lexeme.name.to_string())),
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

#[allow(
    clippy::needless_pass_by_value,
    reason = "the identifier is consumed by token interpolation"
)]
fn default_method(type_name: &str, argument: syn::Ident) -> TokenStream {
    let ty = ident(type_name);
    let method = format_ident!("visit_{}", snake_case(type_name));
    let walker = format_ident!("walk_{}", snake_case(type_name));
    quote! {
        fn #method(&mut self, #argument: &#ty) { #walker(self, #argument); }
    }
}

fn noop_method(type_name: &str, mode: VisitMode) -> TokenStream {
    let ty = ident(type_name);
    let method = format_ident!("visit_{}", snake_case(type_name));
    let argument = ident(&format!("_{}", leaf_argument(type_name)));
    match mode {
        VisitMode::Copy => quote! { fn #method(&mut self, #argument: #ty) {} },
        VisitMode::Borrowed => quote! { fn #method(&mut self, #argument: &#ty) {} },
    }
}

fn emit_category_walker(category: &str, members: &[&crate::Construction]) -> GeneratedItem {
    let ty = ident(category);
    let function = format_ident!("walk_{}", snake_case(category));
    let argument = category_argument(category);
    let arms = members.iter().map(|construction| {
        let variant = ident(&pascal_case(&construction.name.to_string()));
        let payload = ident(&snake_case(&construction.element.name.to_string()));
        let callback = format_ident!(
            "visit_{}",
            snake_case(&construction.element.name.to_string())
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
                DeclarationKey::new(DeclarationKind::Construction, construction.name.to_string())
            })
            .collect(),
    )
}

fn emit_construction_walker(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
) -> syn::Result<GeneratedItem> {
    let ty = &construction.element.name;
    let argument = ident(&snake_case(&ty.to_string()));
    let function = format_ident!("walk_{}", snake_case(&ty.to_string()));
    let private = construction.checked.as_ref().is_some_and(|checked| {
        checked.visibilities.iter().any(|visibility| {
            matches!(
                visibility.visibility,
                crate::NonPublicVisibility::Private(_)
            )
        })
    });
    let destructure = if construction.element.fields.is_empty() {
        quote! { let #ty = #argument; }
    } else if private {
        TokenStream::new()
    } else {
        let fields = construction.element.fields.iter().map(|field| &field.name);
        quote! { let #ty { #(#fields),* } = #argument; }
    };
    let fields = construction
        .element
        .fields
        .iter()
        .map(|field| (field.name.to_string(), field))
        .collect::<HashMap<_, _>>();
    let mut calls = Vec::new();
    for atom in &construction.form.atoms {
        let call = match atom {
            FormAtom::Literal(_) => None,
            FormAtom::Role(role) => {
                let field = fields
                    .get(&role.to_string())
                    .ok_or_else(|| internal("walker category role absent"))?;
                let FieldKind::Category(path) = &field.kind else {
                    return Err(internal("walker bare role is not category"));
                };
                let callback = format_ident!("visit_{}", snake_case(&path_name(path)));
                let value = field_value(construction, role, &argument);
                Some(quote! { visitor.#callback(#value); })
            }
            FormAtom::Lex(role) => {
                let field = fields
                    .get(&role.to_string())
                    .ok_or_else(|| internal("walker lex role absent"))?;
                let terminal = field_terminal(field);
                let walker = format_ident!("walk_{}", snake_case(&terminal));
                let value = field_value(construction, role, &argument);
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
                    .get(&role.to_string())
                    .ok_or_else(|| internal("walker identity role absent"))?;
                let terminal = field_terminal(field);
                let walker = format_ident!("walk_{}", snake_case(&terminal));
                let value = field_value(construction, role, &argument);
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
                    .get(&role.to_string())
                    .ok_or_else(|| internal("walker noun role absent"))?;
                let terminal = field_terminal(field);
                let callback = format_ident!("visit_{}", snake_case(&terminal));
                let value = field_value(construction, role, &argument);
                Some(quote! { visitor.#callback(#value); })
            }
            FormAtom::Verb(crate::VerbOperand::Fixed(path)) => {
                let terminal = path
                    .segments
                    .iter()
                    .rev()
                    .nth(1)
                    .ok_or_else(|| internal("fixed verb path lacks terminal"))?
                    .ident
                    .to_string();
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
            construction.name.to_string(),
        )],
    ))
}

fn emit_enum_walker<'a>(
    ty: &syn::Ident,
    variants: impl Iterator<Item = &'a syn::Ident>,
    kind: DeclarationKind,
) -> GeneratedItem {
    let function = format_ident!("walk_{}", snake_case(&ty.to_string()));
    let argument = leaf_argument(&ty.to_string());
    let argument = ident(&argument);
    let callback = format_ident!("visit_{}", snake_case(&ty.to_string()));
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
        vec![DeclarationKey::new(kind, ty.to_string())],
    )
}

fn emit_binding_walker(binding: &crate::TerminalBinding) -> syn::Result<GeneratedItem> {
    let ty = simple_type_ident(&binding.value_type)?;
    let type_name = ty.to_string();
    let function = format_ident!("walk_{}", snake_case(&type_name));
    let argument = binding_argument(binding);
    let mode = binding
        .traversal
        .callback_mode
        .ok_or_else(|| internal("traversal binding lacks callback mode"))?;
    let signature = match mode {
        VisitMode::Copy => quote! { #argument: #ty },
        VisitMode::Borrowed => quote! { #argument: &#ty },
    };
    let body = if !binding.traversal.branches.is_empty() {
        let branches = binding.traversal.branches.iter().map(|branch| {
            let value = &branch.value;
            let arms = branch.arms.iter().map(|arm| {
                let variant = &arm.variant;
                let binding = &arm.binding;
                let call = emit_call_expr(&arm.call);
                quote! { #variant(#binding) => #call }
            });
            quote! { match #value { #(#arms,)* } }
        });
        quote! { #(#branches)* }
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
        let destructure = if fields.is_empty() {
            TokenStream::new()
        } else {
            quote! { let #ty { #(#fields),* } = #argument; }
        };
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
                    let field = fields[field_index];
                    statements.push(quote! { let _ = #field; });
                    ignored_fields[field_index] = true;
                }
            }
            statements.push(emit_call(call));
        }
        for (field_index, field) in fields.iter().enumerate() {
            if !used_fields[field_index] && !ignored_fields[field_index] {
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
            binding.name.to_string(),
        )],
    ))
}

fn emit_call(call: &TraversalCall) -> TokenStream {
    let call = emit_call_expr(call);
    quote! { #call; }
}

fn emit_call_expr(call: &TraversalCall) -> TokenStream {
    let value = &call.value;
    let argument = match call.mode {
        VisitMode::Copy => quote! { *#value },
        VisitMode::Borrowed => quote! { #value },
    };
    let segments = call.callback.segments.iter().collect::<Vec<_>>();
    if segments.len() == 2 && segments[0].ident == "visitor" {
        let method = &segments[1].ident;
        quote! { visitor.#method(#argument) }
    } else {
        let callback = &call.callback;
        quote! { #callback(visitor, #argument) }
    }
}

fn expr_mentions(expression: &syn::Expr, ident: &syn::Ident) -> bool {
    expression
        .to_token_stream()
        .into_iter()
        .any(|token| matches!(token, proc_macro2::TokenTree::Ident(found) if found == *ident))
}

fn terminal_mode(validated: &ValidatedDeclarations, terminal: &str) -> syn::Result<VisitMode> {
    if validated.raw().declarations.iter().any(
        |declaration| matches!(declaration, Declaration::Vocab(vocab) if vocab.name == terminal),
    ) {
        return Ok(VisitMode::Copy);
    }
    validated
        .raw()
        .declarations
        .iter()
        .find_map(|declaration| match declaration {
            Declaration::Codec(binding) | Declaration::Identity(binding)
                if binding.name == terminal =>
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
    DeclarationKey::new(binding_kind(binding), binding.name.to_string())
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
fn binding_argument(binding: &crate::TerminalBinding) -> syn::Ident {
    binding
        .traversal
        .argument
        .clone()
        .unwrap_or_else(|| ident(&leaf_argument(&binding.name.to_string())))
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
fn category_argument(category: &str) -> syn::Ident {
    let snake = snake_case(category);
    ident(&snake)
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
    path.segments
        .last()
        .map_or_else(String::new, |segment| segment.ident.to_string())
}
fn ident(name: &str) -> syn::Ident {
    syn::Ident::new(name, Span::call_site())
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
    fn full_golden_has_exact_visitor_item_count() {
        let expansion = crate::generate(crate::validate::tests::full_golden_tokens())
            .expect("the full golden declaration expands");
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
        assert_eq!(visitor_items, 38);
    }

    #[test]
    fn full_golden_visitor_has_exact_callback_signatures_and_walker_order() {
        let expansion = crate::generate(crate::validate::tests::full_golden_tokens()).unwrap();
        let trait_item = named(&expansion, "Visitor");
        let syn::Item::Trait(ref visitor) = trait_item else {
            panic!("Visitor is a trait")
        };
        assert_eq!(
            trait_item,
            expected_visitor_item(),
            "literal Visitor method order, signatures, and default bodies",
        );
        let actual = visitor
            .items
            .iter()
            .map(|item| {
                let syn::TraitItem::Fn(method) = item else {
                    panic!("Visitor contains only methods")
                };
                let argument = method
                    .sig
                    .inputs
                    .iter()
                    .nth(1)
                    .expect("one value argument")
                    .to_token_stream()
                    .to_string();
                let mut calls = Calls::default();
                if let Some(default) = &method.default {
                    calls.visit_block(default);
                }
                (method.sig.ident.to_string(), argument, calls.0)
            })
            .collect::<Vec<_>>();
        let expected = [
            ("visit_ability", "ability : & Ability", "walk_ability"),
            ("visit_sentence", "sentence : & Sentence", "walk_sentence"),
            ("visit_clause", "clause : & Clause", "walk_clause"),
            (
                "visit_noun_phrase",
                "noun_phrase : & NounPhrase",
                "walk_noun_phrase",
            ),
            (
                "visit_verb_phrase",
                "verb_phrase : & VerbPhrase",
                "walk_verb_phrase",
            ),
            ("visit_amount", "amount : & Amount", "walk_amount"),
            ("visit_noun", "noun : & Noun", "walk_noun"),
            ("visit_spell", "spell : & Spell", "walk_spell"),
            (
                "visit_triggered",
                "triggered : & Triggered",
                "walk_triggered",
            ),
            (
                "visit_imperative",
                "imperative : & Imperative",
                "walk_imperative",
            ),
            (
                "visit_declarative",
                "declarative : & Declarative",
                "walk_declarative",
            ),
            (
                "visit_with_where",
                "with_where : & WithWhere",
                "walk_with_where",
            ),
            (
                "visit_event_clause",
                "event_clause : & EventClause",
                "walk_event_clause",
            ),
            (
                "visit_where_clause",
                "where_clause : & WhereClause",
                "walk_where_clause",
            ),
            (
                "visit_pronoun_np",
                "pronoun_np : & PronounNp",
                "walk_pronoun_np",
            ),
            ("visit_common", "common : & Common", "walk_common"),
            (
                "visit_demonstrative_np",
                "demonstrative_np : & DemonstrativeNp",
                "walk_demonstrative_np",
            ),
            (
                "visit_target_np",
                "target_np : & TargetNp",
                "walk_target_np",
            ),
            (
                "visit_self_reference_np",
                "self_reference_np : & SelfReferenceNp",
                "walk_self_reference_np",
            ),
            ("visit_count_np", "count_np : & CountNp", "walk_count_np"),
            ("visit_destroy", "destroy : & Destroy", "walk_destroy"),
            ("visit_connive", "connive : & Connive", "walk_connive"),
            (
                "visit_deal_damage",
                "deal_damage : & DealDamage",
                "walk_deal_damage",
            ),
            (
                "visit_gain_life",
                "gain_life : & GainLife",
                "walk_gain_life",
            ),
            (
                "visit_number_amount",
                "number_amount : & NumberAmount",
                "walk_number_amount",
            ),
            (
                "visit_variable_amount",
                "variable_amount : & VariableAmount",
                "walk_variable_amount",
            ),
            ("visit_trigger_word", "_word : TriggerWord", ""),
            ("visit_article", "_article : Article", ""),
            ("visit_demonstrative", "_demonstrative : Demonstrative", ""),
            ("visit_pronoun", "_pronoun : Pronoun", ""),
            ("visit_variable", "_variable : Variable", ""),
            ("visit_sign", "_sign : Sign", ""),
            (
                "visit_self_reference_spelling",
                "_spelling : SelfReferenceSpelling",
                "",
            ),
            ("visit_noun_lexeme", "_noun : NounLexeme", ""),
            ("visit_verb_lexeme", "_verb : VerbLexeme", ""),
            ("visit_signed_number", "_number : & SignedNumber", ""),
            (
                "visit_catalog_identity",
                "_identity : & CatalogIdentity",
                "",
            ),
            ("visit_catalog_spelling", "_spelling : & str", ""),
        ];
        assert_eq!(actual.len(), expected.len());
        for ((name, argument, calls), (expected_name, expected_argument, expected_call)) in
            actual.iter().zip(expected)
        {
            assert_eq!(name, expected_name);
            assert_eq!(argument, expected_argument, "{name} argument");
            if expected_call.is_empty() {
                assert!(calls.is_empty(), "{name} default");
            } else {
                assert_eq!(calls, &[expected_call], "{name} default");
            }
        }

        const WALKERS: &[(&str, &[&str])] = &[
            ("walk_ability", &["visit_spell", "visit_triggered"]),
            (
                "walk_sentence",
                &["visit_imperative", "visit_declarative", "visit_with_where"],
            ),
            ("walk_clause", &["visit_event_clause", "visit_where_clause"]),
            (
                "walk_noun_phrase",
                &[
                    "visit_pronoun_np",
                    "visit_common",
                    "visit_demonstrative_np",
                    "visit_target_np",
                    "visit_self_reference_np",
                    "visit_count_np",
                ],
            ),
            (
                "walk_verb_phrase",
                &[
                    "visit_destroy",
                    "visit_connive",
                    "visit_deal_damage",
                    "visit_gain_life",
                ],
            ),
            (
                "walk_amount",
                &["visit_number_amount", "visit_variable_amount"],
            ),
            ("walk_noun", &["walk_noun_lexeme", "walk_catalog_identity"]),
            ("walk_spell", &["visit_sentence"]),
            (
                "walk_triggered",
                &["walk_trigger_word", "visit_clause", "visit_sentence"],
            ),
            ("walk_imperative", &["visit_verb_phrase"]),
            (
                "walk_declarative",
                &["visit_noun_phrase", "visit_verb_phrase"],
            ),
            ("walk_with_where", &["visit_sentence", "visit_clause"]),
            (
                "walk_event_clause",
                &["visit_noun_phrase", "visit_verb_phrase"],
            ),
            (
                "walk_where_clause",
                &["walk_variable", "walk_verb_lexeme", "visit_noun_phrase"],
            ),
            ("walk_pronoun_np", &["walk_pronoun"]),
            ("walk_common", &["walk_article", "visit_noun"]),
            (
                "walk_demonstrative_np",
                &["walk_demonstrative", "visit_noun"],
            ),
            ("walk_target_np", &["visit_noun"]),
            ("walk_self_reference_np", &["walk_self_reference_spelling"]),
            (
                "walk_count_np",
                &[
                    "visit_noun",
                    "walk_pronoun",
                    "walk_verb_lexeme",
                    "walk_signed_number",
                ],
            ),
            ("walk_destroy", &["walk_verb_lexeme", "visit_noun_phrase"]),
            ("walk_connive", &["walk_verb_lexeme"]),
            (
                "walk_deal_damage",
                &["walk_verb_lexeme", "visit_amount", "visit_noun_phrase"],
            ),
            ("walk_gain_life", &["walk_verb_lexeme", "visit_amount"]),
            ("walk_number_amount", &["walk_signed_number"]),
            ("walk_variable_amount", &["walk_variable"]),
            ("walk_trigger_word", &["visit_trigger_word"]),
            ("walk_article", &["visit_article", "visit_article"]),
            (
                "walk_demonstrative",
                &["visit_demonstrative", "visit_demonstrative"],
            ),
            ("walk_pronoun", &["visit_pronoun", "visit_pronoun"]),
            ("walk_variable", &["visit_variable"]),
            ("walk_sign", &["visit_sign", "visit_sign"]),
            (
                "walk_self_reference_spelling",
                &[
                    "visit_self_reference_spelling",
                    "visit_self_reference_spelling",
                ],
            ),
            ("walk_noun_lexeme", &["visit_noun_lexeme"]),
            (
                "walk_verb_lexeme",
                &[
                    "visit_verb_lexeme",
                    "visit_verb_lexeme",
                    "visit_verb_lexeme",
                    "visit_verb_lexeme",
                    "visit_verb_lexeme",
                    "visit_verb_lexeme",
                ],
            ),
            ("walk_signed_number", &["walk_sign", "visit_signed_number"]),
            (
                "walk_catalog_identity",
                &["visit_catalog_identity", "visit_catalog_spelling"],
            ),
        ];
        let actual_names = expansion
            .items()
            .iter()
            .filter_map(|item| match &item.key {
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name,
                } if name.starts_with("walk_") => Some(name.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            actual_names,
            WALKERS.iter().map(|(name, _)| *name).collect::<Vec<_>>()
        );
        let expected_items = expected_walker_items();
        assert_eq!(actual_names.len(), expected_items.len());
        for (&name, expected) in actual_names.iter().zip(expected_items) {
            assert_eq!(
                named(&expansion, name),
                expected,
                "literal walker signature, pattern, body, and call operands for {name}",
            );
        }
        for &(name, expected_calls) in WALKERS {
            let syn::Item::Fn(item) = named(&expansion, name) else {
                panic!("{name} is a function")
            };
            let mut calls = Calls::default();
            calls.visit_block(&item.block);
            assert_eq!(calls.0, expected_calls, "{name} preorder calls");
        }
    }

    fn expected_visitor_item() -> syn::Item {
        syn::parse2(quote::quote! {
            pub trait Visitor {
                fn visit_ability(&mut self, ability: &Ability) { walk_ability(self, ability); }
                fn visit_sentence(&mut self, sentence: &Sentence) { walk_sentence(self, sentence); }
                fn visit_clause(&mut self, clause: &Clause) { walk_clause(self, clause); }
                fn visit_noun_phrase(&mut self, noun_phrase: &NounPhrase) { walk_noun_phrase(self, noun_phrase); }
                fn visit_verb_phrase(&mut self, verb_phrase: &VerbPhrase) { walk_verb_phrase(self, verb_phrase); }
                fn visit_amount(&mut self, amount: &Amount) { walk_amount(self, amount); }
                fn visit_noun(&mut self, noun: &Noun) { walk_noun(self, noun); }
                fn visit_spell(&mut self, spell: &Spell) { walk_spell(self, spell); }
                fn visit_triggered(&mut self, triggered: &Triggered) { walk_triggered(self, triggered); }
                fn visit_imperative(&mut self, imperative: &Imperative) { walk_imperative(self, imperative); }
                fn visit_declarative(&mut self, declarative: &Declarative) { walk_declarative(self, declarative); }
                fn visit_with_where(&mut self, with_where: &WithWhere) { walk_with_where(self, with_where); }
                fn visit_event_clause(&mut self, event_clause: &EventClause) { walk_event_clause(self, event_clause); }
                fn visit_where_clause(&mut self, where_clause: &WhereClause) { walk_where_clause(self, where_clause); }
                fn visit_pronoun_np(&mut self, pronoun_np: &PronounNp) { walk_pronoun_np(self, pronoun_np); }
                fn visit_common(&mut self, common: &Common) { walk_common(self, common); }
                fn visit_demonstrative_np(&mut self, demonstrative_np: &DemonstrativeNp) { walk_demonstrative_np(self, demonstrative_np); }
                fn visit_target_np(&mut self, target_np: &TargetNp) { walk_target_np(self, target_np); }
                fn visit_self_reference_np(&mut self, self_reference_np: &SelfReferenceNp) { walk_self_reference_np(self, self_reference_np); }
                fn visit_count_np(&mut self, count_np: &CountNp) { walk_count_np(self, count_np); }
                fn visit_destroy(&mut self, destroy: &Destroy) { walk_destroy(self, destroy); }
                fn visit_connive(&mut self, connive: &Connive) { walk_connive(self, connive); }
                fn visit_deal_damage(&mut self, deal_damage: &DealDamage) { walk_deal_damage(self, deal_damage); }
                fn visit_gain_life(&mut self, gain_life: &GainLife) { walk_gain_life(self, gain_life); }
                fn visit_number_amount(&mut self, number_amount: &NumberAmount) { walk_number_amount(self, number_amount); }
                fn visit_variable_amount(&mut self, variable_amount: &VariableAmount) { walk_variable_amount(self, variable_amount); }
                fn visit_trigger_word(&mut self, _word: TriggerWord) {}
                fn visit_article(&mut self, _article: Article) {}
                fn visit_demonstrative(&mut self, _demonstrative: Demonstrative) {}
                fn visit_pronoun(&mut self, _pronoun: Pronoun) {}
                fn visit_variable(&mut self, _variable: Variable) {}
                fn visit_sign(&mut self, _sign: Sign) {}
                fn visit_self_reference_spelling(&mut self, _spelling: SelfReferenceSpelling) {}
                fn visit_noun_lexeme(&mut self, _noun: NounLexeme) {}
                fn visit_verb_lexeme(&mut self, _verb: VerbLexeme) {}
                fn visit_signed_number(&mut self, _number: &SignedNumber) {}
                fn visit_catalog_identity(&mut self, _identity: &CatalogIdentity) {}
                fn visit_catalog_spelling(&mut self, _spelling: &str) {}
            }
        })
        .expect("literal Visitor fixture parses")
    }

    fn expected_walker_items() -> Vec<syn::Item> {
        syn::parse2::<syn::File>(quote::quote! {
            pub fn walk_ability<V: Visitor + ?Sized>(visitor: &mut V, ability: &Ability) { match ability { Ability::Spell(spell) => visitor.visit_spell(spell), Ability::Triggered(triggered) => visitor.visit_triggered(triggered), } }
            pub fn walk_sentence<V: Visitor + ?Sized>(visitor: &mut V, sentence: &Sentence) { match sentence { Sentence::Imperative(imperative) => visitor.visit_imperative(imperative), Sentence::Declarative(declarative) => visitor.visit_declarative(declarative), Sentence::WithWhere(with_where) => visitor.visit_with_where(with_where), } }
            pub fn walk_clause<V: Visitor + ?Sized>(visitor: &mut V, clause: &Clause) { match clause { Clause::Event(event_clause) => visitor.visit_event_clause(event_clause), Clause::Where(where_clause) => visitor.visit_where_clause(where_clause), } }
            pub fn walk_noun_phrase<V: Visitor + ?Sized>(visitor: &mut V, noun_phrase: &NounPhrase) { match noun_phrase { NounPhrase::Pronoun(pronoun_np) => visitor.visit_pronoun_np(pronoun_np), NounPhrase::Common(common) => visitor.visit_common(common), NounPhrase::Demonstrative(demonstrative_np) => { visitor.visit_demonstrative_np(demonstrative_np); }, NounPhrase::Target(target_np) => visitor.visit_target_np(target_np), NounPhrase::SelfReference(self_reference_np) => { visitor.visit_self_reference_np(self_reference_np); }, NounPhrase::Count(count_np) => visitor.visit_count_np(count_np), } }
            pub fn walk_verb_phrase<V: Visitor + ?Sized>(visitor: &mut V, verb_phrase: &VerbPhrase) { match verb_phrase { VerbPhrase::Destroy(destroy) => visitor.visit_destroy(destroy), VerbPhrase::Connive(connive) => visitor.visit_connive(connive), VerbPhrase::DealDamage(deal_damage) => visitor.visit_deal_damage(deal_damage), VerbPhrase::GainLife(gain_life) => visitor.visit_gain_life(gain_life), } }
            pub fn walk_amount<V: Visitor + ?Sized>(visitor: &mut V, amount: &Amount) { match amount { Amount::Number(number_amount) => visitor.visit_number_amount(number_amount), Amount::Variable(variable_amount) => visitor.visit_variable_amount(variable_amount), } }
            pub fn walk_noun<V: Visitor + ?Sized>(visitor: &mut V, noun: &Noun) { match noun { Noun::Lexeme(noun_lexeme) => walk_noun_lexeme(visitor, *noun_lexeme), Noun::Catalog(catalog_identity) => walk_catalog_identity(visitor, catalog_identity), } }
            pub fn walk_spell<V: Visitor + ?Sized>(visitor: &mut V, spell: &Spell) { let Spell { effect } = spell; visitor.visit_sentence(effect); }
            pub fn walk_triggered<V: Visitor + ?Sized>(visitor: &mut V, triggered: &Triggered) { let Triggered { trigger, event, effect } = triggered; walk_trigger_word(visitor, *trigger); visitor.visit_clause(event); visitor.visit_sentence(effect); }
            pub fn walk_imperative<V: Visitor + ?Sized>(visitor: &mut V, imperative: &Imperative) { let Imperative { predicate } = imperative; visitor.visit_verb_phrase(predicate); }
            pub fn walk_declarative<V: Visitor + ?Sized>(visitor: &mut V, declarative: &Declarative) { let Declarative { subject, predicate } = declarative; visitor.visit_noun_phrase(subject); visitor.visit_verb_phrase(predicate); }
            pub fn walk_with_where<V: Visitor + ?Sized>(visitor: &mut V, with_where: &WithWhere) { let WithWhere { body, clause } = with_where; visitor.visit_sentence(body); visitor.visit_clause(clause); }
            pub fn walk_event_clause<V: Visitor + ?Sized>(visitor: &mut V, event_clause: &EventClause) { let EventClause { subject, predicate } = event_clause; visitor.visit_noun_phrase(subject); visitor.visit_verb_phrase(predicate); }
            pub fn walk_where_clause<V: Visitor + ?Sized>(visitor: &mut V, where_clause: &WhereClause) { let WhereClause { variable, value } = where_clause; walk_variable(visitor, *variable); walk_verb_lexeme(visitor, VerbLexeme::Be); visitor.visit_noun_phrase(value); }
            pub fn walk_pronoun_np<V: Visitor + ?Sized>(visitor: &mut V, pronoun_np: &PronounNp) { let PronounNp { word } = pronoun_np; walk_pronoun(visitor, *word); }
            pub fn walk_common<V: Visitor + ?Sized>(visitor: &mut V, common: &Common) { let Common { article, head } = common; walk_article(visitor, *article); visitor.visit_noun(head); }
            pub fn walk_demonstrative_np<V: Visitor + ?Sized>(visitor: &mut V, demonstrative_np: &DemonstrativeNp) { let DemonstrativeNp { word, head } = demonstrative_np; walk_demonstrative(visitor, *word); visitor.visit_noun(head); }
            pub fn walk_target_np<V: Visitor + ?Sized>(visitor: &mut V, target_np: &TargetNp) { let TargetNp { head } = target_np; visitor.visit_noun(head); }
            pub fn walk_self_reference_np<V: Visitor + ?Sized>(visitor: &mut V, self_reference_np: &SelfReferenceNp) { walk_self_reference_spelling(visitor, self_reference_np.spelling()); }
            pub fn walk_count_np<V: Visitor + ?Sized>(visitor: &mut V, count_np: &CountNp) { let CountNp { head, controller, threshold } = count_np; visitor.visit_noun(head); walk_pronoun(visitor, *controller); walk_verb_lexeme(visitor, VerbLexeme::Control); walk_signed_number(visitor, threshold); }
            pub fn walk_destroy<V: Visitor + ?Sized>(visitor: &mut V, destroy: &Destroy) { let Destroy { object } = destroy; walk_verb_lexeme(visitor, VerbLexeme::Destroy); visitor.visit_noun_phrase(object); }
            pub fn walk_connive<V: Visitor + ?Sized>(visitor: &mut V, connive: &Connive) { let Connive = connive; walk_verb_lexeme(visitor, VerbLexeme::Connive); }
            pub fn walk_deal_damage<V: Visitor + ?Sized>(visitor: &mut V, deal_damage: &DealDamage) { let DealDamage { amount, to } = deal_damage; walk_verb_lexeme(visitor, VerbLexeme::Deal); visitor.visit_amount(amount); visitor.visit_noun_phrase(to); }
            pub fn walk_gain_life<V: Visitor + ?Sized>(visitor: &mut V, gain_life: &GainLife) { let GainLife { amount } = gain_life; walk_verb_lexeme(visitor, VerbLexeme::Gain); visitor.visit_amount(amount); }
            pub fn walk_number_amount<V: Visitor + ?Sized>(visitor: &mut V, number_amount: &NumberAmount) { let NumberAmount { number } = number_amount; walk_signed_number(visitor, number); }
            pub fn walk_variable_amount<V: Visitor + ?Sized>(visitor: &mut V, variable_amount: &VariableAmount) { let VariableAmount { variable } = variable_amount; walk_variable(visitor, *variable); }
            pub fn walk_trigger_word<V: Visitor + ?Sized>(visitor: &mut V, word: TriggerWord) { match word { TriggerWord::Whenever => visitor.visit_trigger_word(TriggerWord::Whenever), } }
            pub fn walk_article<V: Visitor + ?Sized>(visitor: &mut V, article: Article) { match article { Article::A => visitor.visit_article(Article::A), Article::An => visitor.visit_article(Article::An), } }
            pub fn walk_demonstrative<V: Visitor + ?Sized>(visitor: &mut V, demonstrative: Demonstrative) { match demonstrative { Demonstrative::That => visitor.visit_demonstrative(Demonstrative::That), Demonstrative::Those => visitor.visit_demonstrative(Demonstrative::Those), } }
            pub fn walk_pronoun<V: Visitor + ?Sized>(visitor: &mut V, pronoun: Pronoun) { match pronoun { Pronoun::It => visitor.visit_pronoun(Pronoun::It), Pronoun::You => visitor.visit_pronoun(Pronoun::You), } }
            pub fn walk_variable<V: Visitor + ?Sized>(visitor: &mut V, variable: Variable) { match variable { Variable::X => visitor.visit_variable(Variable::X), } }
            pub fn walk_sign<V: Visitor + ?Sized>(visitor: &mut V, sign: Sign) { match sign { Sign::Positive => visitor.visit_sign(Sign::Positive), Sign::Negative => visitor.visit_sign(Sign::Negative), } }
            pub fn walk_self_reference_spelling<V: Visitor + ?Sized>(visitor: &mut V, spelling: SelfReferenceSpelling) { match spelling { SelfReferenceSpelling::Full => { visitor.visit_self_reference_spelling(SelfReferenceSpelling::Full); }, SelfReferenceSpelling::Abbreviated => { visitor.visit_self_reference_spelling(SelfReferenceSpelling::Abbreviated); }, } }
            pub fn walk_noun_lexeme<V: Visitor + ?Sized>(visitor: &mut V, noun: NounLexeme) { match noun { NounLexeme::Player => visitor.visit_noun_lexeme(NounLexeme::Player), } }
            pub fn walk_verb_lexeme<V: Visitor + ?Sized>(visitor: &mut V, verb: VerbLexeme) { match verb { VerbLexeme::Destroy => visitor.visit_verb_lexeme(VerbLexeme::Destroy), VerbLexeme::Connive => visitor.visit_verb_lexeme(VerbLexeme::Connive), VerbLexeme::Deal => visitor.visit_verb_lexeme(VerbLexeme::Deal), VerbLexeme::Gain => visitor.visit_verb_lexeme(VerbLexeme::Gain), VerbLexeme::Control => visitor.visit_verb_lexeme(VerbLexeme::Control), VerbLexeme::Be => visitor.visit_verb_lexeme(VerbLexeme::Be), } }
            pub fn walk_signed_number<V: Visitor + ?Sized>(visitor: &mut V, number: &SignedNumber) { let SignedNumber { sign, magnitude } = number; walk_sign(visitor, *sign); let _ = magnitude; visitor.visit_signed_number(number); }
            pub fn walk_catalog_identity<V: Visitor + ?Sized>(visitor: &mut V, identity: &CatalogIdentity) { let CatalogIdentity { kind, spelling } = identity; let _ = kind; visitor.visit_catalog_identity(identity); visitor.visit_catalog_spelling(spelling); }
        }).unwrap().items
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
