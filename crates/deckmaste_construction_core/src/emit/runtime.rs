use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::emitted_ident;
use crate::identifier::path_key;
use crate::identifier::snake_case;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::semantic::BindingPlan;
use crate::semantic::LexemePlan;
use crate::semantic::SemanticPlan;
use crate::semantic::VocabPlan;

struct RuntimeInventory<'a> {
    vocabs: Vec<&'a VocabPlan>,
    noun_lexeme: Option<&'a LexemePlan>,
    verb_lexeme: Option<&'a LexemePlan>,
    noun_binding: Option<&'a BindingPlan>,
    direct_bindings: Vec<&'a BindingPlan>,
    opaque_bindings: Vec<&'a BindingPlan>,
}

impl<'a> RuntimeInventory<'a> {
    fn from_plan(plan: &'a SemanticPlan) -> Self {
        Self {
            vocabs: plan.runtime_vocabs().collect(),
            noun_lexeme: plan.runtime_noun_lexeme(),
            verb_lexeme: plan.runtime_verb_lexeme(),
            noun_binding: plan.runtime_noun_binding(),
            direct_bindings: plan.runtime_direct_bindings().collect(),
            opaque_bindings: plan.runtime_opaque_bindings().collect(),
        }
    }

    fn noun_type(&self) -> Option<syn::Ident> {
        self.noun_binding
            .map(|binding| binding.value_type_name().clone())
            .or_else(|| self.noun_lexeme.map(|lexeme| lexeme.name_ident().clone()))
    }
}

pub(crate) fn emit(plan: &SemanticPlan) -> Vec<GeneratedItem> {
    let inventory = RuntimeInventory::from_plan(plan);
    let mut items = vec![
        named_type(
            "Agreement",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Agreement { Bare, ThirdPersonSingular }
            },
        ),
        named_type(
            "Number",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Number { Singular, Plural }
            },
        ),
        named_type(
            "FeatureConstraint",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum FeatureConstraint<F> { Exact(F), Any }
            },
        ),
        named_type(
            "CasePosition",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum CasePosition { DocumentInitial, Continuation }
            },
        ),
        named_type(
            "ScanPosition",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) struct ScanPosition {
                    pub(crate) byte_offset: usize,
                    pub(crate) case: CasePosition,
                }
            },
        ),
        named_type(
            "DeclarationClass",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub struct DeclarationClass {
                    kind: ::macro_ron::v2::DeclarationKind,
                    position: ::macro_ron::v2::GrammarPosition,
                }
            },
        ),
        named_type(
            "DeclarationMatcher",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) struct DeclarationMatcher {
                    pub(crate) kind: ::macro_ron::v2::DeclarationKind,
                    pub(crate) name: &'static str,
                    pub(crate) position: ::macro_ron::v2::GrammarPosition,
                    pub(crate) feature: FeatureConstraint<::macro_ron::v2::SurfaceFeature>,
                }
            },
        ),
        named_type(
            "DeclarationLeaf",
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub(crate) struct DeclarationLeaf {
                    pub(crate) id: ::macro_ron::v2::DeclarationIdentity,
                    pub(crate) feature: ::macro_ron::v2::SurfaceFeature,
                }
            },
        ),
    ];

    items.extend(emit_lexical_types(&inventory));
    items.extend(emit_owner_types());
    items.extend(emit_runtime_impls(&inventory));
    let origins = plan.declaration_keys().to_vec();
    for item in &mut items {
        item.origins.clone_from(&origins);
    }
    items
}

fn emit_lexical_types(inventory: &RuntimeInventory<'_>) -> Vec<GeneratedItem> {
    let vocab_variants = inventory.vocabs.iter().map(|vocab| vocab.name_ident());
    let vocab_leaf_variants = inventory.vocabs.iter().map(|vocab| {
        let ident = vocab.name_ident();
        quote! { #ident(#ident) }
    });
    let vocab_class_variants = inventory.vocabs.iter().map(|vocab| vocab.name_ident());

    let noun_lexical = inventory
        .noun_type()
        .map(|_| quote! { Noun(FeatureConstraint<Number>), });
    let noun_leaf = inventory.noun_type().map(|noun_type| {
        quote! { Noun { noun: #noun_type, number: Number }, }
    });
    let noun_class = inventory.noun_type().map(|_| quote! { Noun, });

    let verb_lexical = inventory.verb_lexeme.map(|lexeme| {
        let ident = lexeme.name_ident();
        quote! { Verb(#ident), }
    });
    let verb_leaf = inventory.verb_lexeme.map(|lexeme| {
        let ident = lexeme.name_ident();
        quote! { Verb { lexeme: #ident, agreement: Agreement }, }
    });
    let verb_class = inventory.verb_lexeme.map(|_| quote! { VerbLexeme, });

    let direct_lexical_variants = inventory.direct_bindings.iter().map(|binding| {
        let variant = binding_variant(binding);
        quote! { #variant, }
    });
    let direct_leaf_variants = inventory.direct_bindings.iter().map(|binding| {
        let variant = binding_variant(binding);
        let value_type = binding.value_type_name();
        quote! { #variant(#value_type), }
    });
    let direct_class_variants = inventory
        .direct_bindings
        .iter()
        .map(|binding| binding_variant(binding));

    let opaque_lexical_variants = inventory.opaque_bindings.iter().map(|binding| {
        let variant = binding_variant(binding);
        quote! { #variant, }
    });
    let opaque_leaf_variants = inventory.opaque_bindings.iter().map(|binding| {
        let variant = binding_variant(binding);
        quote! { #variant(BoundLeaf), }
    });
    let opaque_class_variants = inventory
        .opaque_bindings
        .iter()
        .map(|binding| binding_variant(binding));

    vec![
        named_type(
            "Lexical",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Lexical {
                    Literal(&'static str),
                    EndOfInput,
                    #(#vocab_variants,)*
                    #noun_lexical
                    #verb_lexical
                    #(#direct_lexical_variants)*
                    #(#opaque_lexical_variants)*
                    Declaration(DeclarationMatcher),
                }
            },
        ),
        named_type(
            "Leaf",
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub(crate) enum Leaf {
                    Literal(&'static str),
                    EndOfInput,
                    #(#vocab_leaf_variants,)*
                    #noun_leaf
                    #verb_leaf
                    #(#direct_leaf_variants)*
                    #(#opaque_leaf_variants)*
                    Declaration(DeclarationLeaf),
                }
            },
        ),
        named_type(
            "TerminalClass",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub enum TerminalClass {
                    EndOfInput,
                    #(#vocab_class_variants,)*
                    #noun_class
                    #verb_class
                    #(#direct_class_variants,)*
                    #(#opaque_class_variants,)*
                    Declaration(DeclarationClass),
                }
            },
        ),
        named_type(
            "LexicalTerminal",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) struct LexicalTerminal {
                    pub(crate) matcher: Lexical,
                    pub(crate) owner: LexicalOwnerTemplate,
                }
            },
        ),
    ]
}

fn emit_owner_types() -> Vec<GeneratedItem> {
    vec![
        named_type(
            "LexicalProvenanceKind",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub enum LexicalProvenanceKind {
                    FormLiteral,
                    Vocab,
                    Lexeme,
                    Codec,
                    Identity,
                    Declaration,
                }
            },
        ),
        named_type(
            "LexicalOwnerTemplate",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum LexicalOwnerTemplate {
                    None,
                    Static {
                        kind: LexicalProvenanceKind,
                        stable_id: &'static str,
                    },
                    Vocab {
                        declaration: &'static str,
                    },
                    Lexeme {
                        declaration: &'static str,
                        member: &'static str,
                    },
                    Declaration {
                        kind: ::macro_ron::v2::DeclarationKind,
                        name: &'static str,
                    },
                }
            },
        ),
        named_type(
            "LexicalOwner",
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
                pub struct LexicalOwner {
                    kind: LexicalProvenanceKind,
                    stable_id: String,
                }
            },
        ),
    ]
}

fn emit_runtime_impls(inventory: &RuntimeInventory<'_>) -> Vec<GeneratedItem> {
    let mut items = emit_class_impls(inventory);
    items.extend(emit_owner_impls(inventory));
    items
}

fn emit_class_impls(inventory: &RuntimeInventory<'_>) -> Vec<GeneratedItem> {
    let vocab_class_arms = inventory.vocabs.iter().map(|vocab| {
        let variant = vocab.name_ident();
        quote! { Lexical::#variant => TerminalClass::#variant, }
    });
    let noun_class_arm = inventory
        .noun_type()
        .map(|_| quote! { Lexical::Noun(_) => TerminalClass::Noun, });
    let verb_class_arm = inventory
        .verb_lexeme
        .map(|_| quote! { Lexical::Verb(_) => TerminalClass::VerbLexeme, });
    let direct_class_arms = inventory.direct_bindings.iter().map(|binding| {
        let variant = binding_variant(binding);
        quote! { Lexical::#variant => TerminalClass::#variant, }
    });
    let opaque_class_arms = inventory.opaque_bindings.iter().map(|binding| {
        let variant = binding_variant(binding);
        quote! { Lexical::#variant => TerminalClass::#variant, }
    });
    let vocab_labels = inventory.vocabs.iter().map(|vocab| {
        let variant = vocab.name_ident();
        let label = syn::LitStr::new(
            &snake_case(vocab.name()).replace('_', " "),
            Span::call_site(),
        );
        quote! { TerminalClass::#variant => #label, }
    });
    let noun_label = inventory
        .noun_type()
        .map(|_| quote! { TerminalClass::Noun => "noun", });
    let verb_label = inventory
        .verb_lexeme
        .map(|_| quote! { TerminalClass::VerbLexeme => "verb lexeme", });
    let direct_labels = inventory.direct_bindings.iter().map(|binding| {
        let variant = binding_variant(binding);
        let label = syn::LitStr::new(
            &snake_case(&variant.to_string()).replace('_', " "),
            Span::call_site(),
        );
        quote! { TerminalClass::#variant => #label, }
    });
    let opaque_labels = inventory.opaque_bindings.iter().map(|binding| {
        let variant = binding_variant(binding);
        let label = syn::LitStr::new(
            &snake_case(&variant.to_string()).replace('_', " "),
            Span::call_site(),
        );
        quote! { TerminalClass::#variant => #label, }
    });
    vec![
        impl_item(
            Some("Lexical"),
            "Lexical",
            quote! {
                impl Lexical {
                    pub(crate) const fn class(self) -> TerminalClass {
                        match self {
                            Lexical::Literal(_) => unreachable!(),
                            Lexical::EndOfInput => TerminalClass::EndOfInput,
                            #(#vocab_class_arms)*
                            #noun_class_arm
                            #verb_class_arm
                            #(#direct_class_arms)*
                            #(#opaque_class_arms)*
                            Lexical::Declaration(matcher) => TerminalClass::Declaration(
                                DeclarationClass {
                                    kind: matcher.kind,
                                    position: matcher.position,
                                }
                            ),
                        }
                    }
                }
            },
        ),
        impl_item(
            Some("LexicalTerminal"),
            "LexicalTerminal",
            quote! {
                impl LexicalTerminal {
                    pub(crate) const fn class(self) -> TerminalClass {
                        self.matcher.class()
                    }
                }
            },
        ),
        impl_item(
            Some("TerminalClass"),
            "TerminalClass",
            quote! {
                impl TerminalClass {
                    pub const fn label(self) -> &'static str {
                        match self {
                            TerminalClass::EndOfInput => "end of input",
                            #(#vocab_labels)*
                            #noun_label
                            #verb_label
                            #(#direct_labels)*
                            #(#opaque_labels)*
                            TerminalClass::Declaration(_) => "open declaration",
                        }
                    }
                }
            },
        ),
        impl_item(
            Some("std::fmt::Display"),
            "TerminalClass",
            quote! {
                impl std::fmt::Display for TerminalClass {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str(self.label())
                    }
                }
            },
        ),
        impl_item(
            Some("DeclarationClass"),
            "DeclarationClass",
            quote! {
                impl DeclarationClass {
                    pub const fn kind(self) -> ::macro_ron::v2::DeclarationKind {
                        self.kind
                    }

                    pub const fn position(self) -> ::macro_ron::v2::GrammarPosition {
                        self.position
                    }
                }
            },
        ),
    ]
}

fn emit_owner_impls(inventory: &RuntimeInventory<'_>) -> Vec<GeneratedItem> {
    let vocab_owner_arms = inventory.vocabs.iter().flat_map(|vocab| {
        let declaration_ident = vocab.name_ident();
        let declaration = syn::LitStr::new(vocab.name(), Span::call_site());
        vocab.variants().iter().map(move |variant| {
            let member_ident = variant.name();
            let stable_id = syn::LitStr::new(
                &format!("vocab:{}/{}", vocab.name(), member_ident),
                Span::call_site(),
            );
            quote! {
                (
                    LexicalOwnerTemplate::Vocab { declaration: #declaration },
                    Leaf::#declaration_ident(#declaration_ident::#member_ident),
                ) => Some(LexicalOwner {
                    kind: LexicalProvenanceKind::Vocab,
                    stable_id: #stable_id.to_owned(),
                }),
            }
        })
    });

    vec![
        impl_item(
            Some("LexicalOwner"),
            "LexicalOwner",
            quote! {
                impl LexicalOwner {
                    pub const fn kind(&self) -> LexicalProvenanceKind {
                        self.kind
                    }

                    pub fn stable_id(&self) -> &str {
                        &self.stable_id
                    }
                }
            },
        ),
        impl_item(
            Some("LexicalOwnerTemplate"),
            "LexicalOwnerTemplate",
            quote! {
                impl LexicalOwnerTemplate {
                    pub(crate) fn instantiate(self, value: &Leaf) -> Option<LexicalOwner> {
                        match (self, value) {
                            (LexicalOwnerTemplate::None, Leaf::EndOfInput) => None,
                            (
                                LexicalOwnerTemplate::Static { kind, stable_id },
                                _,
                            ) => Some(LexicalOwner {
                                kind,
                                stable_id: stable_id.to_owned(),
                            }),
                            #(#vocab_owner_arms)*
                            (
                                LexicalOwnerTemplate::Lexeme {
                                    declaration,
                                    member,
                                },
                                _,
                            ) => Some(LexicalOwner {
                                kind: LexicalProvenanceKind::Lexeme,
                                stable_id: format!("lexeme:{declaration}/{member}"),
                            }),
                            (
                                LexicalOwnerTemplate::Declaration { kind, name },
                                Leaf::Declaration(_),
                            ) => Some(LexicalOwner {
                                kind: LexicalProvenanceKind::Declaration,
                                stable_id: format!("declaration:{kind}/{name}"),
                            }),
                            _ => unreachable!("validated lexical owner template/value mismatch"),
                        }
                    }
                }
            },
        ),
    ]
}

fn binding_variant(binding: &BindingPlan) -> syn::Ident {
    let name = binding
        .lexical_variant()
        .map_or_else(|| binding.name().to_owned(), path_key);
    emitted_ident(&name, binding.origin_span())
}

fn named_type(name: &str, tokens: TokenStream) -> GeneratedItem {
    GeneratedItem::new(ItemKey::named_type(name), tokens, Vec::new())
}

fn impl_item(trait_name: Option<&str>, self_ty: &str, tokens: TokenStream) -> GeneratedItem {
    GeneratedItem::new(
        ItemKey::Impl {
            trait_name: trait_name.map(str::to_owned),
            self_ty: self_ty.to_owned(),
        },
        tokens,
        Vec::new(),
    )
}
