use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::emit::LocalAllocator;
use crate::identifier::VISITOR_TRAIT;
use crate::identifier::emitted_ident;
use crate::identifier::key as identifier_key;
use crate::identifier::path_key;
use crate::identifier::snake_case;
use crate::identifier::structural_sequence_walker;
use crate::model::VisitMode;
use crate::plan::DeclarationKey;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::plan::SourceDeclarationKind as DeclarationKind;
use crate::semantic::AccessorMode;
use crate::semantic::AtomPlan;
use crate::semantic::BindingPlan;
use crate::semantic::BindingTraversalRecipe;
use crate::semantic::CatalogIdentityPlan;
use crate::semantic::ConstructionFieldKind;
use crate::semantic::ConstructionFieldPlan;
use crate::semantic::ConstructionPlan;
use crate::semantic::DeclarationDeterminativePlan;
use crate::semantic::DeclarationNounPlan;
use crate::semantic::DeclarationTermPlan;
use crate::semantic::DeclarationVerbPlan;
use crate::semantic::FiniteDomainKindPlan;
use crate::semantic::FiniteValuePlan;
use crate::semantic::FormPlan;
use crate::semantic::LexemePlan;
use crate::semantic::SemanticPlan;
use crate::semantic::SignedDecimalPlan;
use crate::semantic::StructuralFieldKindPlan;
use crate::semantic::TerminalPlan;
use crate::semantic::TraversalBranchArmPlan;
use crate::semantic::TraversalCallPlan;
use crate::semantic::TraversalFieldPlan;
use crate::semantic::TraversalValuePlan;
use crate::semantic::UnsignedNumberPlan;
use crate::semantic::ValueKindPlan;
use crate::semantic::VocabPlan;
use crate::semantic::VocabVariantPlan;

pub(crate) fn emit(validated: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let constructions = validated.constructions();
    let categories = category_groups(constructions)
        .into_iter()
        .filter(|(category, _)| !validated.explicit_sum_owns_construction_category(category))
        .collect::<Vec<_>>();
    let mut vocabs = Vec::new();
    let mut lexemes = Vec::new();
    let mut bindings = Vec::new();
    let mut context_identities = Vec::new();
    let mut catalog_identities = Vec::new();
    let mut signed_decimal = None;
    let mut unsigned_numbers = Vec::new();
    let mut declaration_nouns = Vec::new();
    let mut declaration_determinatives = Vec::new();
    let mut declaration_terms = Vec::new();
    let mut declaration_verbs = Vec::new();
    for terminal in validated.terminals() {
        match terminal {
            TerminalPlan::Vocab(row) => vocabs.push(row),
            TerminalPlan::Lexeme(row) => lexemes.push(row),
            TerminalPlan::Binding(row) => {
                if let Some(plan) = row.declaration_verb() {
                    declaration_verbs.push(plan);
                } else {
                    bindings.push(row);
                }
            }
            TerminalPlan::ContextIdentity(row) => context_identities.push(row),
            TerminalPlan::CatalogIdentity(row) => catalog_identities.push(row),
            TerminalPlan::SignedDecimal(row) => signed_decimal = Some(row),
            TerminalPlan::UnsignedNumber(row) => unsigned_numbers.push(row),
            TerminalPlan::DeclarationNoun(row) => declaration_nouns.push(row),
            TerminalPlan::DeclarationDeterminative(row) => declaration_determinatives.push(row),
            TerminalPlan::DeclarationTerm(row) => declaration_terms.push(row),
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
        catalog_identities: &catalog_identities,
        lexemes: &lexemes,
        borrowed_bindings: &borrowed_bindings,
        signed_decimal,
        unsigned_numbers: &unsigned_numbers,
        declaration_nouns: &declaration_nouns,
        declaration_determinatives: &declaration_determinatives,
        declaration_terms: &declaration_terms,
        declaration_verbs: &declaration_verbs,
    };
    let trait_item = emit_trait(validated, &categories, constructions, &terminal_visitors)?;
    let mut items = vec![trait_item];
    for (category, members) in &categories {
        items.push(emit_category_walker(category, members));
    }
    items.extend(emit_structural_walkers(validated)?);
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
    for identity in &catalog_identities {
        items.push(emit_catalog_identity_walker(identity));
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
    for codec in &unsigned_numbers {
        items.push(emit_unsigned_number_walker(codec));
    }
    for codec in declaration_nouns {
        items.push(emit_declaration_noun_value_walker(codec));
        items.push(emit_declaration_noun_walker(codec));
    }
    for codec in declaration_determinatives {
        items.push(emit_declaration_determinative_walker(codec));
    }
    for codec in declaration_terms {
        items.push(emit_declaration_term_walker(codec));
    }
    for codec in declaration_verbs {
        items.push(emit_declaration_verb_value_walker(codec));
        items.push(emit_declaration_verb_walker(codec));
    }
    Ok(items)
}

struct TerminalVisitors<'a> {
    containers: &'a [&'a BindingPlan],
    vocabs: &'a [&'a VocabPlan],
    copy_bindings: &'a [&'a BindingPlan],
    context_identities: &'a [&'a crate::semantic::ContextIdentityPlan],
    catalog_identities: &'a [&'a CatalogIdentityPlan],
    lexemes: &'a [&'a LexemePlan],
    borrowed_bindings: &'a [&'a BindingPlan],
    signed_decimal: Option<&'a SignedDecimalPlan>,
    unsigned_numbers: &'a [&'a UnsignedNumberPlan],
    declaration_nouns: &'a [&'a DeclarationNounPlan],
    declaration_determinatives: &'a [&'a DeclarationDeterminativePlan],
    declaration_terms: &'a [&'a DeclarationTermPlan],
    declaration_verbs: &'a [&'a DeclarationVerbPlan],
}

#[allow(
    clippy::unnecessary_wraps,
    reason = "keeps every visitor phase finalizer on one fallible interface"
)]
fn emit_trait(
    plan: &SemanticPlan,
    categories: &[(String, Vec<&ConstructionPlan>)],
    constructions: &[ConstructionPlan],
    terminals: &TerminalVisitors<'_>,
) -> syn::Result<GeneratedItem> {
    let visitor = ident(VISITOR_TRAIT);
    let mut methods = scope_visitor_methods();
    methods.extend(visitor_methods(plan, categories, constructions, terminals));

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
    origins.extend(
        terminals
            .unsigned_numbers
            .iter()
            .map(|codec| codec.origin().clone()),
    );
    if !terminals.catalog_identities.is_empty() {
        methods.push(quote! {
            fn visit_catalog_provider(&mut self, _provider: CatalogProvider) {}
        });
        for identity in terminals.catalog_identities {
            methods.push(noop_method(identity.name(), VisitMode::Borrowed));
        }
    }
    origins.extend(
        terminals
            .declaration_nouns
            .iter()
            .map(|codec| codec.origin().clone()),
    );
    origins.extend(
        terminals
            .declaration_determinatives
            .iter()
            .map(|codec| codec.origin().clone()),
    );
    origins.extend(
        terminals
            .declaration_terms
            .iter()
            .map(|codec| codec.origin().clone()),
    );
    origins.extend(
        terminals
            .declaration_verbs
            .iter()
            .map(|codec| codec.origin().clone()),
    );
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

fn scope_visitor_methods() -> Vec<TokenStream> {
    vec![
        quote! {
            /// Reports one concrete AST construction before its children are visited.
            fn enter_construction(&mut self, _construction: &'static str) {}
        },
        quote! {
            /// Reports the end of the current concrete AST construction.
            fn exit_construction(&mut self) {}
        },
        quote! {
            /// Marks the current construction as the Verb Frame building its Clause.
            fn verb_frame(&mut self, _construction: &'static str) {}
        },
        quote! {
            /// Reports entry across the opaque edge to a role declared by a Verb Frame.
            fn enter_frame_role(&mut self, _role: &'static str) {}
        },
        quote! {
            /// Reports exit across the opaque edge to a role declared by a Verb Frame.
            fn exit_frame_role(&mut self) {}
        },
        quote! {
            /// Reports a named construction role before its value is visited.
            fn enter_role(
                &mut self,
                _role: &'static str,
                _scope_sibling: Option<&'static str>,
                _admissible_sites: Option<&AdmissibleSites>,
            ) {}
        },
        quote! {
            /// Reports the end of the current named construction role.
            fn exit_role(&mut self) {}
        },
        quote! {
            /// Reports a sequence member before its value is visited.
            fn enter_conjunct(&mut self, _role: &'static str, _ordinal: usize) {}
        },
        quote! {
            /// Reports the end of the current sequence member.
            fn exit_conjunct(&mut self) {}
        },
        quote! {
            /// Reports one realized atom for structural span accounting.
            fn scope_leaf(&mut self) {}
        },
        quote! {
            /// Reports one terminal leaf in surface order before its typed callback.
            fn enter_leaf(&mut self, _terminal: &'static str) {}
        },
    ]
}

fn visitor_methods(
    plan: &SemanticPlan,
    categories: &[(String, Vec<&ConstructionPlan>)],
    constructions: &[ConstructionPlan],
    terminals: &TerminalVisitors<'_>,
) -> Vec<TokenStream> {
    let mut methods = Vec::new();
    for (category, _) in categories {
        methods.push(default_method(category, &category_argument(category)));
    }
    for semantic in super::semantic_types(plan) {
        if matches!(
            semantic.kind,
            super::SemanticTypeKind::Product | super::SemanticTypeKind::Sum
        ) && !constructions
            .iter()
            .any(|construction| construction.element_type() == semantic.name)
        {
            methods.push(default_method(semantic.name, &snake_case(semantic.name)));
        }
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
    for codec in terminals.unsigned_numbers {
        methods.push(noop_method(codec.codec_name(), VisitMode::Borrowed));
    }
    for codec in terminals.declaration_nouns {
        methods.push(default_method(codec.codec_name(), codec.codec_name()));
        methods.push(default_method(
            &codec.declaration_value_ident().to_string(),
            &crate::identifier::snake_case(&codec.declaration_value_ident().to_string()),
        ));
    }
    for codec in terminals.declaration_determinatives {
        methods.push(default_method(codec.codec_name(), codec.codec_name()));
    }
    for codec in terminals.declaration_terms {
        methods.push(default_method(codec.codec_name(), codec.codec_name()));
    }
    for codec in terminals.declaration_verbs {
        methods.push(default_method(codec.codec_name(), codec.codec_name()));
        methods.push(default_method(
            &codec.declaration_value_ident().to_string(),
            &crate::identifier::snake_case(&codec.declaration_value_ident().to_string()),
        ));
    }
    if !terminals.declaration_nouns.is_empty()
        || !terminals.declaration_determinatives.is_empty()
        || !terminals.declaration_terms.is_empty()
        || !terminals.declaration_verbs.is_empty()
        || constructions.iter().any(|construction| {
            construction
                .forms()
                .iter()
                .flat_map(FormPlan::atoms)
                .any(|atom| matches!(atom, AtomPlan::OpenDeclaration(_)))
        })
    {
        methods.push(quote! {
            fn visit_declaration(
                &mut self,
                _declaration: &::deckmaste_construction_core::macro_def::DeclarationIdentity,
            ) {}
        });
    }
    if !terminals.declaration_verbs.is_empty() {
        methods.push(quote! {
            fn visit_verb_inventory(
                &mut self,
                _verb: &crate::environment::VerbInventoryRef,
            ) {}
        });
    }
    methods
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

fn emit_declaration_verb_value_walker(codec: &DeclarationVerbPlan) -> GeneratedItem {
    let ty = codec.declaration_value_ident();
    let function = ident(&format!("walk_{}", snake_case(&ty.to_string())));
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, declaration: &#ty) {
                visitor.visit_verb_inventory(declaration.reference());
                if let crate::environment::VerbInventoryRef::Declaration(id) = declaration.reference() {
                    visitor.visit_declaration(id);
                }
            }
        },
        vec![codec.origin().clone()],
    )
}

fn emit_declaration_term_walker(codec: &DeclarationTermPlan) -> GeneratedItem {
    let ty = codec.codec_ident();
    let function = ident(&format!("walk_{}", snake_case(codec.codec_name())));
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, term: &#ty) {
                visitor.visit_declaration(term.id());
            }
        },
        vec![codec.origin().clone()],
    )
}

fn emit_catalog_identity_walker(identity: &CatalogIdentityPlan) -> GeneratedItem {
    let ty = identity.ident();
    let function = ident(&format!("walk_{}", snake_case(identity.name())));
    let callback = ident(&format!("visit_{}", snake_case(identity.name())));
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, identity: &#ty) {
                visitor.visit_catalog_provider(identity.provider());
                visitor.#callback(identity);
            }
        },
        vec![identity.origin().clone()],
    )
}

fn emit_structural_walkers(plan: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let mut rows = Vec::new();
    for product in plan.products() {
        let mut emitted = Vec::new();
        for field in product.fields() {
            if matches!(field.kind(), StructuralFieldKindPlan::Sequence { .. }) {
                emitted.push(emit_sequence_walker(
                    plan,
                    product.name(),
                    field,
                    DeclarationKind::AbstractProduct,
                )?);
            }
        }
        emitted.push(emit_product_walker(plan, product)?);
        rows.push((product.source_index(), emitted));
    }
    for sum in plan.sums() {
        rows.push((sum.source_index(), vec![emit_sum_walker(plan, sum)?]));
    }
    for construction in plan.constructions() {
        let mut emitted = Vec::new();
        for field in construction
            .fields()
            .iter()
            .filter_map(|field| field.structural_plan())
        {
            if matches!(field.kind(), StructuralFieldKindPlan::Sequence { .. }) {
                emitted.push(emit_sequence_walker(
                    plan,
                    construction.element_type(),
                    field,
                    DeclarationKind::Construction,
                )?);
            }
        }
        if !emitted.is_empty() {
            rows.push((construction.source_index(), emitted));
        }
    }
    rows.sort_by_key(|(source_index, _)| *source_index);
    Ok(rows.into_iter().flat_map(|(_, items)| items).collect())
}

fn emit_product_walker(
    plan: &SemanticPlan,
    product: &crate::semantic::ProductPlan,
) -> syn::Result<GeneratedItem> {
    let function_name = crate::identifier::prefixed("walk_", product.name());
    let function = ident(&function_name);
    let ty = ident(product.name());
    let argument = ident(&snake_case(product.name()));
    let calls = product
        .fields()
        .iter()
        .map(|field| walk_structural_field(plan, product.name(), field, &argument))
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, #argument: &#ty) {
                #(#calls)*
            }
        },
        vec![DeclarationKey::new(
            DeclarationKind::AbstractProduct,
            product.name(),
        )],
    ))
}

fn emit_sum_walker(
    plan: &SemanticPlan,
    sum: &crate::semantic::SumPlan,
) -> syn::Result<GeneratedItem> {
    let function_name = crate::identifier::prefixed("walk_", sum.name());
    let function = ident(&function_name);
    let ty = ident(sum.name());
    let argument = ident(&snake_case(sum.name()));
    let arms = sum
        .alternatives()
        .iter()
        .map(|alternative| -> syn::Result<TokenStream> {
            let variant = ident(alternative.name());
            let call = walk_structural_value(plan, alternative.value(), quote! { value })?;
            Ok(quote! { #ty::#variant(value) => { #call } })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let match_argument = if sum.alternatives().is_empty() {
        quote! { *#argument }
    } else {
        quote! { #argument }
    };
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, #argument: &#ty) {
                match #match_argument { #(#arms),* }
            }
        },
        vec![DeclarationKey::new(
            DeclarationKind::AbstractSum,
            sum.name(),
        )],
    ))
}

fn emit_sequence_walker(
    plan: &SemanticPlan,
    owner: &str,
    field: &crate::semantic::StructuralFieldPlan,
    kind: DeclarationKind,
) -> syn::Result<GeneratedItem> {
    let StructuralFieldKindPlan::Sequence { item, .. } = field.kind() else {
        return Err(internal("sequence walker received a non-sequence field"));
    };
    let function_name = structural_sequence_walker(owner, field.name());
    let function = ident(&function_name);
    let item_ty = super::value_kind_type(item);
    let call = walk_structural_value(plan, item, quote! { value })?;
    let role = syn::LitStr::new(field.name(), Span::call_site());
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function<V: Visitor + ?Sized>(visitor: &mut V, values: &[#item_ty]) {
                for (ordinal, value) in values.iter().enumerate() {
                    if ordinal > 0 {
                        visitor.scope_leaf();
                    }
                    visitor.enter_conjunct(#role, ordinal);
                    #call
                    visitor.exit_conjunct();
                }
            }
        },
        vec![DeclarationKey::new(kind, owner)],
    ))
}

fn walk_structural_field(
    plan: &SemanticPlan,
    owner: &str,
    field: &crate::semantic::StructuralFieldPlan,
    whole: &syn::Ident,
) -> syn::Result<TokenStream> {
    let name = ident(field.name());
    match field.kind() {
        StructuralFieldKindPlan::Required(value) => {
            walk_structural_value(plan, value, quote! { &#whole.#name })
        }
        StructuralFieldKindPlan::Zeroable(value) | StructuralFieldKindPlan::Optional(value) => {
            let call = walk_structural_value(plan, value, quote! { value })?;
            Ok(quote! { if let Some(value) = #whole.#name.as_ref() { #call } })
        }
        StructuralFieldKindPlan::Sequence { .. } => {
            let function = ident(&structural_sequence_walker(owner, field.name()));
            Ok(quote! { #function(visitor, &#whole.#name); })
        }
    }
}

fn walk_structural_value(
    plan: &SemanticPlan,
    value: &ValueKindPlan,
    expression: TokenStream,
) -> syn::Result<TokenStream> {
    match value {
        ValueKindPlan::Category(name) | ValueKindPlan::Product(name) | ValueKindPlan::Sum(name) => {
            let callback = ident(&crate::identifier::prefixed("visit_", name));
            Ok(quote! { visitor.#callback(#expression); })
        }
        ValueKindPlan::Lex(name) | ValueKindPlan::Identity(name) => {
            let walker = ident(&crate::identifier::prefixed("walk_", name));
            let enter_leaf = enter_leaf_call(plan, name)?;
            let expression = if terminal_mode(plan, name)? == VisitMode::Copy {
                quote! { *#expression }
            } else {
                expression
            };
            Ok(quote! { visitor.scope_leaf(); #enter_leaf #walker(visitor, #expression); })
        }
    }
}

fn emit_declaration_noun_walker(codec: &DeclarationNounPlan) -> GeneratedItem {
    let ty = codec.codec_ident();
    let declaration = codec.declaration_value_ident();
    let function = ident(&format!("walk_{}", snake_case(codec.codec_name())));
    let closed_arm = codec.closed_lexeme().map(|closed| {
        let closed_walker = ident(&format!("walk_{}", snake_case(&closed.to_string())));
        quote! { #ty::Lexeme(lexeme) => #closed_walker(visitor, *lexeme), }
    });
    let declaration_callback = ident(&format!("visit_{}", snake_case(&declaration.to_string())));
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, noun: &#ty) {
                match noun {
                    #closed_arm
                    #ty::Declaration(declaration) => visitor.#declaration_callback(declaration),
                }
            }
        },
        vec![codec.origin().clone()],
    )
}

fn emit_declaration_determinative_walker(codec: &DeclarationDeterminativePlan) -> GeneratedItem {
    let ty = codec.codec_ident();
    let function = ident(&format!("walk_{}", snake_case(codec.codec_name())));
    let closed_arm = quote! { #ty::Closed(_) => {}, };
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, det: &#ty) {
                match det {
                    #closed_arm
                }
            }
        },
        vec![codec.origin().clone()],
    )
}

fn emit_declaration_verb_walker(codec: &DeclarationVerbPlan) -> GeneratedItem {
    let ty = codec.codec_ident();
    let declaration = codec.declaration_value_ident();
    let function = ident(&format!("walk_{}", snake_case(codec.codec_name())));
    let declaration_callback = ident(&format!("visit_{}", snake_case(&declaration.to_string())));
    let body = if let Some(closed) = codec.closed_lexeme() {
        let closed_walker = ident(&format!("walk_{}", snake_case(&closed.to_string())));
        quote! {
            match verb {
                #ty::Lexeme(lexeme) => #closed_walker(visitor, *lexeme),
                #ty::Declaration(declaration) => visitor.#declaration_callback(declaration),
            }
        }
    } else {
        quote! { visitor.#declaration_callback(verb); }
    };
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, verb: &#ty) {
                #body
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

fn emit_unsigned_number_walker(codec: &UnsignedNumberPlan) -> GeneratedItem {
    let ty = codec.codec_ident();
    let function = ident(&format!("walk_{}", snake_case(codec.codec_name())));
    let callback = ident(&format!("visit_{}", snake_case(codec.codec_name())));
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, number: &#ty) {
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
    let construction_id = syn::LitStr::new(construction.rule_id(), Span::call_site());
    let ty = ident(&type_name);
    let function = ident(&format!("walk_{}", snake_case(&type_name)));
    let mut allocator = LocalAllocator::default();
    allocator.reserve("visitor");
    for atom in construction.forms().iter().flat_map(FormPlan::atoms) {
        let terminal = match atom.value_atom() {
            AtomPlan::Lex { terminal, .. }
            | AtomPlan::LexFixed { terminal, .. }
            | AtomPlan::Marked { terminal, .. }
            | AtomPlan::Identity { terminal, .. }
            | AtomPlan::VerbFixed { terminal, .. } => Some(terminal.clone()),
            AtomPlan::Literal(_)
            | AtomPlan::SentenceInitialLiteral(_)
            | AtomPlan::StructuralLiteral(_)
            | AtomPlan::Category { .. }
            | AtomPlan::Noun { .. }
            | AtomPlan::OpenDeclaration(_) => None,
            AtomPlan::Bound { .. } | AtomPlan::Circumfix { .. } => {
                unreachable!("value_atom removes form wrappers")
            }
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
        if construction.has_mobile_role() {
            quote! { let #ty { #(#fields),*, .. } = #argument; }
        } else {
            quote! { let #ty { #(#fields),* } = #argument; }
        }
    };
    let fields = construction
        .fields()
        .iter()
        .map(|field| (field.name_key(), field))
        .collect::<HashMap<_, _>>();
    let has_feature_guard = construction.forms().iter().any(|form| {
        form.guard().predicate().is_some_and(|predicate| {
            predicate
                .domains()
                .iter()
                .any(|domain| matches!(domain.kind(), FiniteDomainKindPlan::Feature { .. }))
        })
    });
    if has_feature_guard {
        let canonical = construction
            .forms()
            .first()
            .ok_or_else(|| internal("construction has no canonical traversal form"))?;
        let calls = emit_construction_form_walker_calls(
            validated,
            construction,
            canonical,
            &argument,
            &field_locals,
            &fields,
        )?;
        return Ok(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: function.to_string(),
            },
            quote! {
                pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, #argument: &#ty) {
                    visitor.enter_construction(#construction_id);
                    #destructure
                    #(#calls)*
                    visitor.exit_construction();
                }
            },
            vec![DeclarationKey::new(
                DeclarationKind::Construction,
                construction.construction_id(),
            )],
        ));
    }
    let form_bodies = construction
        .forms()
        .iter()
        .enumerate()
        .map(|(form_index, form)| {
            let calls = emit_construction_form_walker_calls(
                validated,
                construction,
                form,
                &argument,
                &field_locals,
                &fields,
            )?;
            let guard =
                super::emit_form_guard_expression(construction, form_index, |domain, value| {
                    let field = construction.field(domain.role())?;
                    let field_value =
                        field_value(construction, domain.role(), &argument, &field_locals)?;
                    match (domain.kind(), value) {
                        (
                            FiniteDomainKindPlan::Vocab { terminal, .. },
                            FiniteValuePlan::Vocab(variant),
                        ) => {
                            let terminal = ident(terminal);
                            let variant = ident(variant);
                            let value = copy_value(field, field_value);
                            Ok(quote! { matches!(#value, #terminal::#variant) })
                        }
                        (
                            FiniteDomainKindPlan::OptionalVocab { terminal, .. },
                            FiniteValuePlan::OptionalVocab(variant),
                        ) => {
                            if let Some(variant) = variant {
                                let terminal = ident(terminal);
                                let variant = ident(variant);
                                let value = copy_value(field, field_value);
                                Ok(quote! { matches!(#value, Some(#terminal::#variant)) })
                            } else {
                                Ok(quote! { #field_value.is_none() })
                            }
                        }
                        (
                            FiniteDomainKindPlan::OptionalPresence,
                            FiniteValuePlan::OptionalPresence(present),
                        ) => Ok(quote! { #field_value.is_some() == #present }),
                        (FiniteDomainKindPlan::Feature { .. }, FiniteValuePlan::Feature(_)) => {
                            unreachable!("feature guards use one validated canonical traversal")
                        }
                        _ => Err(internal("form guard domain and assignment value disagree")),
                    }
                })?;
            Ok(if let Some(guard) = guard {
                quote! { if #guard { #(#calls)* visitor.exit_construction(); return; } }
            } else {
                quote! { #(#calls)* }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let exhaustiveness = (construction.forms().len() > 1).then(|| {
        quote! { unreachable!("sealed guarded forms are exhaustive"); }
    });
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function.to_string(),
        },
        quote! {
            pub fn #function<V: Visitor + ?Sized>(visitor: &mut V, #argument: &#ty) {
                visitor.enter_construction(#construction_id);
                #destructure
                #(#form_bodies)*
                visitor.exit_construction();
                #exhaustiveness
            }
        },
        vec![DeclarationKey::new(
            DeclarationKind::Construction,
            construction.construction_id(),
        )],
    ))
}

fn emit_construction_form_walker_calls(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    form: &FormPlan,
    argument: &syn::Ident,
    field_locals: &HashMap<String, syn::Ident>,
    fields: &HashMap<String, &ConstructionFieldPlan>,
) -> syn::Result<Vec<TokenStream>> {
    let (mut calls, frame_roles) = verb_frame_traversal_prelude(validated, construction, form);
    for (atom_index, atom) in form.atoms().iter().enumerate() {
        let frame_role = is_frame_role(frame_roles.as_ref(), atom_index);
        if let Some(call) = structural_role_walker_call(
            validated,
            construction,
            form,
            (atom_index, atom),
            argument,
            field_locals,
            fields,
        )? {
            calls.push(wrap_frame_role_call(atom, frame_role, call));
            continue;
        }
        let call = match atom.value_atom() {
            AtomPlan::Literal(_)
            | AtomPlan::SentenceInitialLiteral(_)
            | AtomPlan::StructuralLiteral(_) => Some(quote! { visitor.scope_leaf(); }),
            AtomPlan::Category { role, category } => {
                let field = fields
                    .get(role)
                    .ok_or_else(|| internal("walker category role absent"))?;
                if field.kind() != ConstructionFieldKind::Category {
                    return Err(internal("walker bare role is not category"));
                }
                let callback = ident(&format!("visit_{}", snake_case(category)));
                let value = field_value(construction, role, argument, field_locals)?;
                Some(role_visit(
                    construction,
                    form,
                    atom_index,
                    field,
                    argument,
                    &quote! { visitor.#callback(#value); },
                ))
            }
            AtomPlan::Marked { role, .. } => {
                let field = fields
                    .get(role)
                    .ok_or_else(|| internal("walker marked role absent"))?;
                let visit = marked_role_visit(
                    validated,
                    construction,
                    atom.value_atom(),
                    argument,
                    field_locals,
                    fields,
                )?;
                Some(role_visit(
                    construction,
                    form,
                    atom_index,
                    field,
                    argument,
                    &visit,
                ))
            }
            AtomPlan::Lex { role, terminal } => {
                let field = fields
                    .get(role)
                    .ok_or_else(|| internal("walker lex role absent"))?;
                let walker = ident(&format!("walk_{}", snake_case(terminal)));
                let value = field_value(construction, role, argument, field_locals)?;
                let copy = terminal_mode(validated, terminal)? == VisitMode::Copy;
                let enter_leaf = enter_leaf_call(validated, terminal)?;
                let visit = if copy {
                    let value = copy_value(field, value);
                    quote! { visitor.scope_leaf(); #enter_leaf #walker(visitor, #value); }
                } else {
                    quote! { visitor.scope_leaf(); #enter_leaf #walker(visitor, #value); }
                };
                Some(role_visit(
                    construction,
                    form,
                    atom_index,
                    field,
                    argument,
                    &visit,
                ))
            }
            AtomPlan::Identity { role, terminal } => {
                let field = fields
                    .get(role)
                    .ok_or_else(|| internal("walker identity role absent"))?;
                let walker = ident(&format!("walk_{}", snake_case(terminal)));
                let value = field_value(construction, role, argument, field_locals)?;
                let copy = terminal_mode(validated, terminal)? == VisitMode::Copy;
                let enter_leaf = enter_leaf_call(validated, terminal)?;
                let visit = if copy {
                    let value = copy_value(field, value);
                    quote! { visitor.scope_leaf(); #enter_leaf #walker(visitor, #value); }
                } else {
                    quote! { visitor.scope_leaf(); #enter_leaf #walker(visitor, #value); }
                };
                Some(role_visit(
                    construction,
                    form,
                    atom_index,
                    field,
                    argument,
                    &visit,
                ))
            }
            AtomPlan::Noun { role, terminal } => {
                let field = fields
                    .get(role)
                    .ok_or_else(|| internal("walker noun role absent"))?;
                let callback = ident(&format!("visit_{}", snake_case(terminal)));
                let value = field_value(construction, role, argument, field_locals)?;
                let enter_leaf = enter_leaf_call(validated, terminal)?;
                let visit = if terminal_mode(validated, terminal)? == VisitMode::Copy {
                    let value = copy_value(field, value);
                    quote! { visitor.scope_leaf(); #enter_leaf visitor.#callback(#value); }
                } else {
                    quote! { visitor.scope_leaf(); #enter_leaf visitor.#callback(#value); }
                };
                Some(role_visit(
                    construction,
                    form,
                    atom_index,
                    field,
                    argument,
                    &visit,
                ))
            }
            AtomPlan::LexFixed { terminal, path, .. }
            | AtomPlan::VerbFixed { terminal, path, .. } => {
                let walker = ident(&format!("walk_{}", snake_case(terminal)));
                let enter_leaf = enter_leaf_call(validated, terminal)?;
                Some(quote! { visitor.scope_leaf(); #enter_leaf #walker(visitor, #path); })
            }
            AtomPlan::OpenDeclaration(open) => {
                let kind = crate::emit::declaration_kind(open.kind());
                let name = syn::LitStr::new(open.name(), Span::call_site());
                Some(quote! {
                    visitor.scope_leaf();
                    visitor.enter_leaf("open declaration");
                    visitor.visit_declaration(
                        &::deckmaste_construction_core::macro_def::DeclarationIdentity::new(#kind, #name),
                    );
                })
            }
            AtomPlan::Bound { .. } | AtomPlan::Circumfix { .. } => {
                unreachable!("value_atom removes form wrappers")
            }
        };
        calls.extend(call.map(|call| wrap_frame_role_call(atom, frame_role, call)));
    }
    Ok(calls)
}

fn verb_frame_traversal_prelude(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    form: &FormPlan,
) -> (Vec<TokenStream>, Option<HashSet<usize>>) {
    let frame = declaration_verb_frame(validated, form);
    let frame_roles = frame_role_atom_indexes(construction, form, frame);
    let calls = frame_roles
        .as_ref()
        .map(|_| {
            let construction_id = syn::LitStr::new(construction.rule_id(), Span::call_site());
            quote! { visitor.verb_frame(#construction_id); }
        })
        .into_iter()
        .collect();
    (calls, frame_roles)
}

fn is_frame_role(frame_roles: Option<&HashSet<usize>>, atom_index: usize) -> bool {
    frame_roles.is_some_and(|indexes| indexes.contains(&atom_index))
}

fn wrap_frame_role_call(atom: &AtomPlan, frame_role: bool, call: TokenStream) -> TokenStream {
    if !frame_role {
        return call;
    }
    let role = visit_atom_role(atom)
        .expect("a declared Verb Frame role is represented by a construction role");
    let role = syn::LitStr::new(role, Span::call_site());
    quote! {
        visitor.enter_frame_role(#role);
        #call
        visitor.exit_frame_role();
    }
}

fn declaration_verb_frame<'a>(
    validated: &'a SemanticPlan,
    form: &FormPlan,
) -> Option<(usize, &'a crate::semantic::DeclarationVerbPlan)> {
    form.atoms().iter().enumerate().find_map(|(index, atom)| {
        let (AtomPlan::Lex { terminal, .. } | AtomPlan::VerbFixed { terminal, .. }) =
            atom.value_atom()
        else {
            return None;
        };
        validated
            .runtime_declaration_verb_for(terminal)
            .filter(|(_, frame)| frame.frame_key().has_declared_role_boundary())
            .map(|(_, frame)| (index, frame))
    })
}

fn frame_role_atom_indexes(
    construction: &ConstructionPlan,
    form: &FormPlan,
    frame: Option<(usize, &crate::semantic::DeclarationVerbPlan)>,
) -> Option<HashSet<usize>> {
    let (head_index, frame) = frame?;
    let mut indexes = HashSet::new();
    let mut next_form_index = head_index + 1;
    for frame_atom in frame.frame_key().atoms() {
        let (offset, _) = form.atoms()[next_form_index..]
            .iter()
            .enumerate()
            .find(|(_, form_atom)| frame_atom_matches(construction, frame_atom, form_atom))?;
        let form_index = next_form_index + offset;
        if visit_atom_role(&form.atoms()[form_index]).is_some() {
            indexes.insert(form_index);
        }
        next_form_index = form_index + 1;
    }
    Some(indexes)
}

fn frame_atom_matches(
    construction: &ConstructionPlan,
    frame: &crate::semantic::VerbFrameAtom,
    form: &AtomPlan,
) -> bool {
    use crate::semantic::VerbFrameAtom;

    let form = form.value_atom();
    match (frame, form) {
        (VerbFrameAtom::Literal(expected), AtomPlan::Literal(actual)) => expected == actual,
        (
            VerbFrameAtom::Lex(expected_terminal, expected_variant)
            | VerbFrameAtom::OptionalLex(expected_terminal, expected_variant),
            AtomPlan::LexFixed {
                terminal, variant, ..
            },
        ) => expected_terminal == terminal && expected_variant == variant,
        (
            VerbFrameAtom::MarkedRole(expected_terminal, expected_variant, expected_role)
            | VerbFrameAtom::OptionalMarkedRole(expected_terminal, expected_variant, expected_role),
            AtomPlan::Marked {
                role,
                terminal,
                variant,
                ..
            },
        ) => expected_terminal == terminal && expected_variant == variant && expected_role == role,
        (VerbFrameAtom::Role(expected) | VerbFrameAtom::OptionalRole(expected), _) => {
            visit_atom_role(form).is_some_and(|role| role == expected)
        }
        (
            VerbFrameAtom::Amount
            | VerbFrameAtom::ObjectNounPhrase
            | VerbFrameAtom::PredicativeComplement
            | VerbFrameAtom::FrameComplementPair,
            _,
        ) => visit_atom_role(form)
            .and_then(|role| {
                construction
                    .fields()
                    .iter()
                    .find(|field| field.name_key() == role)
            })
            .is_some_and(|field| match frame {
                VerbFrameAtom::Amount => field.terminal() == "Amount",
                VerbFrameAtom::ObjectNounPhrase => field.terminal() == "Object",
                VerbFrameAtom::PredicativeComplement => field.terminal() == "PredicativeComplement",
                VerbFrameAtom::FrameComplementPair => field.terminal() == "FrameComplementPair",
                _ => false,
            }),
        _ => false,
    }
}

fn structural_role_walker_call(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    form: &FormPlan,
    atom: (usize, &AtomPlan),
    argument: &syn::Ident,
    field_locals: &HashMap<String, syn::Ident>,
    fields: &HashMap<String, &ConstructionFieldPlan>,
) -> syn::Result<Option<TokenStream>> {
    let (atom_index, atom) = atom;
    let Some(role) = visit_atom_role(atom) else {
        return Ok(None);
    };
    let Some(field) = fields.get(role) else {
        return Ok(None);
    };
    let Some(structural) = field.structural_plan() else {
        return Ok(None);
    };
    let value = field_value(construction, role, argument, field_locals)?;
    let marker = if let AtomPlan::Marked { terminal, path, .. } = atom {
        let walker = ident(&format!("walk_{}", snake_case(terminal)));
        let enter_leaf = enter_leaf_call(validated, terminal)?;
        Some(quote! { #enter_leaf #walker(visitor, #path); })
    } else {
        None
    };
    let call = match structural.kind() {
        StructuralFieldKindPlan::Required(kind) => {
            let visit = walk_structural_value(validated, kind, value)?;
            role_visit(
                construction,
                form,
                atom_index,
                field,
                argument,
                &quote! { #marker #visit },
            )
        }
        StructuralFieldKindPlan::Zeroable(kind) => {
            let visit = walk_structural_value(validated, kind, quote! { value })?;
            let ty = field.value_type();
            let visit = role_visit(
                construction,
                form,
                atom_index,
                field,
                argument,
                &quote! { #marker #visit },
            );
            quote! { if let #ty::Headed(value) = #value { #visit } }
        }
        StructuralFieldKindPlan::Optional(kind) => {
            let visit = walk_structural_value(validated, kind, quote! { value })?;
            let optional = if structural.is_recursive() {
                quote! { #value.as_ref() }
            } else {
                value
            };
            let visit = role_visit(
                construction,
                form,
                atom_index,
                field,
                argument,
                &quote! { #marker #visit },
            );
            quote! { if let Some(value) = #optional { #visit } }
        }
        StructuralFieldKindPlan::Sequence { .. } => {
            if marker.is_some() {
                return Err(internal("marked roles cannot be sequences"));
            }
            let walker = ident(&structural_sequence_walker(
                construction.element_type(),
                structural.name(),
            ));
            quote! { #walker(visitor, #value); }
        }
    };
    Ok(Some(call))
}

fn marked_role_visit(
    validated: &SemanticPlan,
    construction: &ConstructionPlan,
    atom: &AtomPlan,
    argument: &syn::Ident,
    field_locals: &HashMap<String, syn::Ident>,
    fields: &HashMap<String, &ConstructionFieldPlan>,
) -> syn::Result<TokenStream> {
    let AtomPlan::Marked {
        role,
        category,
        terminal,
        path,
        ..
    } = atom
    else {
        return Err(internal("marked-role walker received another atom"));
    };
    let field = fields
        .get(role)
        .ok_or_else(|| internal("walker marked role absent"))?;
    if field.kind() != ConstructionFieldKind::Category {
        return Err(internal("walker marked role is not category"));
    }
    let walker = ident(&format!("walk_{}", snake_case(terminal)));
    let enter_leaf = enter_leaf_call(validated, terminal)?;
    let callback = ident(&format!("visit_{}", snake_case(category)));
    let value = field_value(construction, role, argument, field_locals)?;
    Ok(quote! {
        visitor.scope_leaf();
        #enter_leaf
        #walker(visitor, #path);
        visitor.#callback(#value);
    })
}

fn role_visit(
    construction: &ConstructionPlan,
    form: &FormPlan,
    atom_index: usize,
    field: &ConstructionFieldPlan,
    argument: &syn::Ident,
    visit: &TokenStream,
) -> TokenStream {
    let role = syn::LitStr::new(&field.name_key(), field.name().span());
    let scope_sibling = if field.is_mobile() {
        field.mobile_scope_sibling().map(str::to_owned).or_else(|| {
            [atom_index.checked_sub(1), atom_index.checked_add(1)]
                .into_iter()
                .flatten()
                .filter_map(|index| form.atoms().get(index))
                .filter_map(visit_atom_role)
                .find(|role| {
                    construction
                        .fields()
                        .iter()
                        .find(|candidate| candidate.name_key() == **role)
                        .and_then(ConstructionFieldPlan::structural_plan)
                        .is_some_and(|structural| {
                            matches!(structural.kind(), StructuralFieldKindPlan::Sequence { .. })
                        })
                })
                .map(str::to_owned)
        })
    } else {
        None
    };
    let scope_sibling = scope_sibling.map_or_else(
        || quote! { None },
        |scope_sibling| {
            let scope_sibling = syn::LitStr::new(&scope_sibling, field.name().span());
            quote! { Some(#scope_sibling) }
        },
    );
    let sites = if field.is_mobile() {
        quote! {
            Some(
                #argument
                    .admissible_sites(#role)
                    .expect("every mobile role has one attachment slot"),
            )
        }
    } else {
        quote! { None }
    };
    quote! {
        visitor.enter_role(#role, #scope_sibling, #sites);
        #visit
        visitor.exit_role();
    }
}

fn visit_atom_role(atom: &AtomPlan) -> Option<&str> {
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
        | AtomPlan::OpenDeclaration(_) => None,
        AtomPlan::Bound { .. } | AtomPlan::Circumfix { .. } => {
            unreachable!("value_atom removes form wrappers")
        }
    }
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
            TerminalPlan::CatalogIdentity(row) if row.name() == terminal => {
                return Ok(VisitMode::Borrowed);
            }
            TerminalPlan::SignedDecimal(row) if row.codec_name() == terminal => {
                return Ok(VisitMode::Borrowed);
            }
            TerminalPlan::UnsignedNumber(row) if row.codec_name() == terminal => {
                return Ok(VisitMode::Borrowed);
            }
            TerminalPlan::DeclarationNoun(row) if row.codec_name() == terminal => {
                return Ok(VisitMode::Borrowed);
            }
            TerminalPlan::DeclarationDeterminative(row) if row.codec_name() == terminal => {
                return Ok(VisitMode::Borrowed);
            }
            TerminalPlan::DeclarationTerm(row) if row.codec_name() == terminal => {
                return Ok(VisitMode::Borrowed);
            }
            TerminalPlan::Vocab(_)
            | TerminalPlan::Lexeme(_)
            | TerminalPlan::Binding(_)
            | TerminalPlan::ContextIdentity(_)
            | TerminalPlan::CatalogIdentity(_)
            | TerminalPlan::SignedDecimal(_)
            | TerminalPlan::UnsignedNumber(_)
            | TerminalPlan::DeclarationNoun(_)
            | TerminalPlan::DeclarationDeterminative(_)
            | TerminalPlan::DeclarationTerm(_) => {}
        }
    }
    Err(internal("terminal traversal mode is absent"))
}

fn enter_leaf_call(plan: &SemanticPlan, terminal: &str) -> syn::Result<TokenStream> {
    let label = terminal_label(plan, terminal)?;
    let label = syn::LitStr::new(&label, Span::call_site());
    Ok(quote! { visitor.enter_leaf(#label); })
}

// Renders the same label the emitted `TerminalClass::label` arms in
// `emit/runtime.rs` render for this terminal. The leaf traversal property
// compares the two renderings, so a divergence fails the coverage gate.
fn terminal_label(plan: &SemanticPlan, terminal: &str) -> syn::Result<String> {
    for planned in plan.terminals() {
        let label = match planned {
            TerminalPlan::Vocab(row) if row.name() == terminal => Some(row.name().to_owned()),
            TerminalPlan::Lexeme(row) if row.name() == terminal => {
                if row.is_verb_provider() {
                    return Ok("verb lexeme".to_owned());
                }
                if row.morphology().recipe() == crate::morphology::MorphologyRecipe::EnglishNoun {
                    return Ok("noun".to_owned());
                }
                Some(row.name().to_owned())
            }
            TerminalPlan::Binding(row) if row.name() == terminal => {
                if row.declaration_verb().is_some() {
                    return Ok("declaration verb".to_owned());
                }
                if plan
                    .runtime_noun_binding()
                    .is_some_and(|noun| noun.name() == terminal)
                {
                    return Ok("noun".to_owned());
                }
                Some(
                    row.lexical_variant()
                        .map_or_else(|| row.name().to_owned(), path_key),
                )
            }
            TerminalPlan::ContextIdentity(row) if row.name() == terminal => {
                Some(row.aggregate_ident().to_string())
            }
            TerminalPlan::CatalogIdentity(row) if row.name() == terminal => {
                return Ok("catalog identity".to_owned());
            }
            TerminalPlan::SignedDecimal(row) if row.codec_name() == terminal => {
                Some(row.codec_name().to_owned())
            }
            TerminalPlan::UnsignedNumber(row) if row.codec_name() == terminal => {
                Some(row.codec_name().to_owned())
            }
            TerminalPlan::DeclarationNoun(row) if row.codec_name() == terminal => {
                return Ok("declaration noun".to_owned());
            }
            TerminalPlan::DeclarationDeterminative(row) if row.codec_name() == terminal => {
                return Ok("declaration determinative".to_owned());
            }
            TerminalPlan::DeclarationTerm(row) if row.codec_name() == terminal => {
                return Ok("declaration term".to_owned());
            }
            TerminalPlan::Vocab(_)
            | TerminalPlan::Lexeme(_)
            | TerminalPlan::Binding(_)
            | TerminalPlan::ContextIdentity(_)
            | TerminalPlan::CatalogIdentity(_)
            | TerminalPlan::SignedDecimal(_)
            | TerminalPlan::UnsignedNumber(_)
            | TerminalPlan::DeclarationNoun(_)
            | TerminalPlan::DeclarationDeterminative(_)
            | TerminalPlan::DeclarationTerm(_) => None,
        };
        if let Some(label) = label {
            return Ok(snake_case(&label).replace('_', " "));
        }
    }
    Err(internal("terminal traversal label is absent"))
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
    fn feature_guarded_walkers_emit_one_canonical_traversal_program() {
        let expansion = crate::generate(quote::quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme NounLexeme using EnglishNoun {
                Player = "player",
                Artifact = "artifact",
            }
            codec Noun {
                generate declaration_noun {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type];
                    feature = Number;
                }
            }
            construction common: Root {
                element Common { head: lex Noun, }
                derive number = Values::Singular;
                derive onset = head.onset;
                form an when head.onset is Vowel = "an" noun(head);
                form a otherwise = "a" noun(head);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("identical article traversal programs validate");
        let walker = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { kind: crate::NamedKind::Function, name } if name == "walk_common"))
            .expect("common walker is generated")
            .tokens
            .to_string();
        assert_eq!(walker.matches("visit_noun").count(), 1, "{walker}");
        assert!(!walker.contains("if true"), "{walker}");
        assert!(!walker.contains("unreachable !"), "{walker}");
    }

    #[test]
    fn guarded_walkers_execute_the_selected_forms_complete_traversal() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab Mode { One = "one", Many = "many", }
                vocab Marker { Alpha = "alpha", Beta = "beta", }
                morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
                lexeme VerbLexeme using EnglishVerb { Act = "act", }
                construction child: Child {
                    element ChildNode {}
                    form child = "child";
                }
                construction guarded_visit: Root {
                    element GuardedVisit { mode: lex Mode, marker: lex Marker, child: Child, }
                    derive verb.concord_class = Values::Other;
                    form one when mode is One = lex(mode) child lex(marker);
                    form many otherwise = lex(marker) open_verb(KeywordAction, "Draw") child lex(mode);
                }
                construction guarded_fixed: FixedRoot {
                    element GuardedFixed { mode: lex Mode, marker: lex Marker, }
                    derive concord_class = verb.concord_class;
                    derive verb.concord_class = Values::Other;
                    form one when mode is One = lex(mode) lex(marker);
                    form many otherwise = lex(marker) verb(VerbLexeme::Act) lex(mode);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("guarded visitor fixture parses"),
        )
        .expect("guarded visitor fixture validates")
        .into_semantic();
        let ast = super::super::ast::emit(&plan).expect("guarded visitor AST emits");
        let visitors = super::emit(&plan).expect("guarded visitors emit");
        let ast = ast.iter().map(|item| &item.tokens);
        let visitors = visitors.iter().map(|item| &item.tokens);
        let source = quote::quote! {
            #![allow(dead_code)]
            extern crate self as deckmaste_construction_core;

            pub mod macro_def {
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub enum DeclarationKind { KeywordAction }

                #[derive(Debug, Clone, PartialEq, Eq)]
                pub struct DeclarationIdentity(DeclarationKind, String);

                impl DeclarationIdentity {
                    pub fn new(kind: DeclarationKind, name: &str) -> Self {
                        Self(kind, name.to_owned())
                    }

                    pub fn name(&self) -> &str { &self.1 }
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum ConcordClass { Other, ThirdPersonSingular }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum Mode { One, Many }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum Marker { Alpha, Beta }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum VerbLexeme { Act }

            struct ParseContext<'a>(std::marker::PhantomData<&'a ()>);

            #(#ast)*
            #(#visitors)*

            #[derive(Default)]
            struct Recorder(Vec<String>);

            impl Visitor for Recorder {
                fn visit_mode(&mut self, value: Mode) {
                    self.0.push(format!("mode:{value:?}"));
                }

                fn visit_marker(&mut self, value: Marker) {
                    self.0.push(format!("marker:{value:?}"));
                }

                fn visit_child(&mut self, _value: &Child) {
                    self.0.push("child".to_owned());
                }

                fn visit_verb_lexeme(&mut self, value: VerbLexeme) {
                    self.0.push(format!("verb:{value:?}"));
                }

                fn visit_declaration(&mut self, value: &macro_def::DeclarationIdentity) {
                    self.0.push(format!("declaration:{}", value.name()));
                }
            }

            fn main() {
                for (value, expected) in [
                    (
                        GuardedVisit {
                            mode: Mode::One,
                            marker: Marker::Alpha,
                            child: Child::Child(ChildNode),
                        },
                        ["mode:One", "child", "marker:Alpha"].as_slice(),
                    ),
                    (
                        GuardedVisit {
                            mode: Mode::Many,
                            marker: Marker::Beta,
                            child: Child::Child(ChildNode),
                        },
                        ["marker:Beta", "declaration:Draw", "child", "mode:Many"].as_slice(),
                    ),
                ] {
                    let mut recorder = Recorder::default();
                    walk_guarded_visit(&mut recorder, &value);
                    assert_eq!(recorder.0, expected);
                }

                for (value, expected) in [
                    (
                        GuardedFixed { mode: Mode::One, marker: Marker::Alpha },
                        ["mode:One", "marker:Alpha"].as_slice(),
                    ),
                    (
                        GuardedFixed { mode: Mode::Many, marker: Marker::Beta },
                        ["marker:Beta", "verb:Act", "mode:Many"].as_slice(),
                    ),
                ] {
                    let mut recorder = Recorder::default();
                    walk_guarded_fixed(&mut recorder, &value);
                    assert_eq!(recorder.0, expected);
                }
            }
        }
        .to_string();

        let directory = tempfile::tempdir().expect("temporary visitor harness directory");
        let source_path = directory.path().join("guarded_visitor_harness.rs");
        let binary_path = directory.path().join("guarded_visitor_harness");
        std::fs::write(&source_path, &source).expect("write deterministic visitor harness");
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
            "visitor harness compilation failed\nstdout:\n{}\nstderr:\n{}\nsource:\n{source}",
            String::from_utf8_lossy(&compilation.stdout),
            String::from_utf8_lossy(&compilation.stderr),
        );
        let execution = std::process::Command::new(&binary_path)
            .output()
            .expect("execute compiled guarded visitor harness");
        assert!(
            execution.status.success(),
            "visitor harness execution failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&execution.stdout),
            String::from_utf8_lossy(&execution.stderr),
        );
    }

    #[test]
    fn structural_optional_sum_and_sequence_walkers_preserve_stored_source_order() {
        let expansion = crate::generate(quote::quote! {
            vocab Word { Alpha = "alpha", Beta = "beta", }
            construction atom: Atom {
                element AtomValue { word: lex Word, }
                form atom = lex(word);
            }
            abstract sum Branch { Atom, }
            abstract product Holder {
                maybe: opt Branch,
                items: seq Branch separated by ", ",
            }
            root Atom { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("structural traversal fixture validates");

        let syn::Item::Trait(visitor) = named(&expansion, "Visitor") else {
            panic!("Visitor is a trait")
        };
        let callbacks = visitor
            .items
            .iter()
            .map(|item| {
                let syn::TraitItem::Fn(method) = item else {
                    panic!("Visitor contains only methods: {item:?}");
                };
                method.sig.ident.to_string()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            callbacks,
            [
                "enter_construction",
                "exit_construction",
                "verb_frame",
                "enter_frame_role",
                "exit_frame_role",
                "enter_role",
                "exit_role",
                "enter_conjunct",
                "exit_conjunct",
                "scope_leaf",
                "enter_leaf",
                "visit_atom",
                "visit_branch",
                "visit_holder",
                "visit_atom_value",
                "visit_word",
            ],
            "the complete Visitor inventory contains semantic callbacks only",
        );

        let syn::Item::Fn(atom_value) = named(&expansion, "walk_atom_value") else {
            panic!("AtomValue walker is a function")
        };
        let atom_value_source = atom_value.block.to_token_stream().to_string();
        assert_eq!(
            atom_value_source.matches("enter_construction").count(),
            1,
            "each concrete construction reports exactly one traversal entry: {atom_value_source}",
        );
        assert!(
            atom_value_source.contains("enter_construction (\"AtomAtom\")"),
            "the traversal entry uses the parser construction identity: {atom_value_source}",
        );

        let walkers = expansion
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
            walkers,
            [
                "walk_atom",
                "walk_branch",
                "walk_holder_items_sequence",
                "walk_holder",
                "walk_atom_value",
                "walk_word",
            ],
            "the complete walker inventory contains no helper-state or surface-policy walker",
        );

        let syn::Item::Fn(holder) = named(&expansion, "walk_holder") else {
            panic!("Holder walker is a function")
        };
        let holder_source = holder.block.to_token_stream().to_string();
        assert!(
            holder_source.contains(
                "if let Some (value) = holder . maybe . as_ref () { visitor . visit_branch (value) ; }"
            ),
            "optional traversal visits only Some: {holder_source}",
        );
        let optional = holder_source
            .find("visit_branch")
            .expect("optional callback");
        let sequence = holder_source
            .find("walk_holder_items_sequence")
            .expect("sequence callback");
        assert!(
            optional < sequence,
            "field order is declaration order: {holder_source}"
        );

        let syn::Item::Fn(sequence) = named(&expansion, "walk_holder_items_sequence") else {
            panic!("sequence walker is a function")
        };
        assert!(
            sequence
                .block
                .to_token_stream()
                .to_string()
                .contains("for (ordinal , value) in values . iter () . enumerate () { if ordinal > 0 { visitor . scope_leaf () ; } visitor . enter_conjunct (\"items\" , ordinal) ; visitor . visit_branch (value) ; visitor . exit_conjunct () ; }"),
            "sequence traversal walks the stored slice with source-order ordinals",
        );

        let syn::Item::Fn(sum) = named(&expansion, "walk_branch") else {
            panic!("sum walker is a function")
        };
        assert!(
            sum.block
                .to_token_stream()
                .to_string()
                .contains("Branch :: Atom (value) => { visitor . visit_atom (value) ; }"),
            "sum traversal delegates directly to its declared payload callback",
        );
    }

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
                "visitor . enter_construction (\"NodeWriter\")",
                "visitor . enter_role (\"plain\" , None , None)",
                "visitor . scope_leaf ()",
                "visitor . enter_leaf (\"plain\")",
                "walk_plain (visitor , * & walk_mode_2 . plain)",
                "visitor . exit_role ()",
                "visitor . enter_role (\"mode\" , None , None)",
                "visitor . scope_leaf ()",
                "visitor . enter_leaf (\"mode\")",
                "walk_mode (visitor , walk_mode_2 . mode ())",
                "visitor . exit_role ()",
                "visitor . enter_role (\"child\" , None , None)",
                "visitor . visit_node (walk_mode_2 . child ())",
                "visitor . exit_role ()",
                "visitor . enter_role (\"spelling\" , None , None)",
                "visitor . scope_leaf ()",
                "visitor . enter_leaf (\"self reference\")",
                "walk_self_reference_spelling (visitor , walk_mode_2 . spelling ())",
                "visitor . exit_role ()",
                "visitor . enter_role (\"visitor\" , None , None)",
                "visitor . scope_leaf ()",
                "visitor . enter_leaf (\"marker\")",
                "walk_marker (visitor , * & walk_mode_2 . visitor)",
                "visitor . exit_role ()",
                "visitor . exit_construction ()",
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
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
            construction visitor_verb: VerbRoot {
                element VisitorVerb {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
                form visitor_verb = verb(VisitorLexeme::Act);
            }
            construction visitor_category: VISITOR {
                element VisitorNode {}
                form visitor_category = "visitor";
            }
            construction marker: MarkerRoot {
                element WalkMarker { marker: lex Marker, }
                form marker = lex(marker);
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
            [
                crate::SourceDeclarationKind::Codec,
                crate::SourceDeclarationKind::Codec
            ],
            "both the codec callback and its leaf callback retain codec provenance",
        );
    }

    #[test]
    fn declaration_determinative_walker_is_closed_without_reentering_itself() {
        let expansion = crate::generate(quote::quote! {
            codec DeterminativeHead {
                generate declaration_determinative {
                    closed = [
                        Each {
                            bare_duration_license = BareDurationLicensed;
                            quantification = Distributive;
                            number_license = SingularOnly;
                            fused_head_license = FusedHead;
                            nominal_license = CountNominal;
                            realizations = [{ surface = "each"; }];
                        },
                    ];
                }
            }
            construction only: Root {
                element Only { head: lex DeterminativeHead, }
                derive number = Values::Singular;
                form only = lex(head);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("declaration determinative visitor fixture is valid");
        let source = expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !source.contains("DeterminativeHead :: Declared"),
            "closed determinatives have no declaration child: {source}",
        );
        assert!(
            !source.contains("visitor . visit_determinative_head (det)"),
            "the walker must not re-enter its own default callback: {source}",
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
                    method
                        .sig
                        .inputs
                        .iter()
                        .nth(1)
                        .map(ToTokens::to_token_stream)
                        .unwrap_or_default()
                        .to_string(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            callbacks,
            [
                (
                    "enter_construction".into(),
                    "_construction : & 'static str".into(),
                ),
                ("exit_construction".into(), String::new()),
                ("verb_frame".into(), "_construction : & 'static str".into(),),
                ("enter_frame_role".into(), "_role : & 'static str".into(),),
                ("exit_frame_role".into(), String::new()),
                ("enter_role".into(), "_role : & 'static str".into()),
                ("exit_role".into(), String::new()),
                ("enter_conjunct".into(), "_role : & 'static str".into()),
                ("exit_conjunct".into(), String::new()),
                ("scope_leaf".into(), String::new()),
                ("enter_leaf".into(), "_terminal : & 'static str".into()),
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
