use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::AGREEMENT_TYPE;
use crate::identifier::CASE_POSITION_TYPE;
use crate::identifier::DECLARATION_CLASS_TYPE;
use crate::identifier::DECLARATION_LEAF_TYPE;
use crate::identifier::DECLARATION_MATCHER_TYPE;
use crate::identifier::FEATURE_CONSTRAINT_TYPE;
use crate::identifier::LEAF_TYPE;
use crate::identifier::LEXICAL_OWNER_IDENTITY_TYPE;
use crate::identifier::LEXICAL_OWNER_TEMPLATE_TYPE;
use crate::identifier::LEXICAL_OWNER_TYPE;
use crate::identifier::LEXICAL_PROVENANCE_KIND_TYPE;
use crate::identifier::LEXICAL_TERMINAL_TYPE;
use crate::identifier::LEXICAL_TYPE;
use crate::identifier::NUMBER_TYPE;
use crate::identifier::SCAN_POSITION_TYPE;
use crate::identifier::TERMINAL_CLASS_TYPE;
use crate::identifier::emitted_ident;
use crate::identifier::path_key;
use crate::identifier::snake_case;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::semantic::BindingPlan;
use crate::semantic::ContextIdentityPlan;
use crate::semantic::DeclarationNounPlan;
use crate::semantic::LexemePlan;
use crate::semantic::SemanticPlan;
use crate::semantic::SignedDecimalPlan;
use crate::semantic::VocabPlan;

struct RuntimeInventory<'a> {
    vocabs: Vec<&'a VocabPlan>,
    noun_lexeme: Option<&'a LexemePlan>,
    verb_lexeme: Option<&'a LexemePlan>,
    noun_binding: Option<&'a BindingPlan>,
    direct_bindings: Vec<&'a BindingPlan>,
    opaque_bindings: Vec<&'a BindingPlan>,
    context_identities: Vec<&'a ContextIdentityPlan>,
    signed_decimal: Option<&'a SignedDecimalPlan>,
    declaration_noun: Option<&'a DeclarationNounPlan>,
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
            context_identities: plan.runtime_context_identities().collect(),
            signed_decimal: plan.runtime_signed_decimal(),
            declaration_noun: plan.runtime_declaration_noun(),
        }
    }

    fn noun_type(&self) -> Option<syn::Ident> {
        self.declaration_noun
            .map(|codec| codec.codec_ident().clone())
            .or_else(|| {
                self.noun_binding
                    .map(|binding| binding.value_type_name().clone())
                    .or_else(|| self.noun_lexeme.map(|lexeme| lexeme.name_ident().clone()))
            })
    }
}

pub(crate) fn emit(plan: &SemanticPlan) -> Vec<GeneratedItem> {
    let inventory = RuntimeInventory::from_plan(plan);
    let mut items = vec![
        named_type(
            AGREEMENT_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Agreement { Bare, ThirdPersonSingular }
            },
        ),
        named_type(
            NUMBER_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Number { Singular, Plural }
            },
        ),
        named_type(
            FEATURE_CONSTRAINT_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum FeatureConstraint<F> { Exact(F), Any }
            },
        ),
        named_type(
            CASE_POSITION_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum CasePosition { DocumentInitial, Continuation }
            },
        ),
        named_type(
            SCAN_POSITION_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) struct ScanPosition {
                    pub(crate) byte_offset: usize,
                    pub(crate) case: CasePosition,
                }
            },
        ),
        named_type(
            DECLARATION_CLASS_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub struct DeclarationClass {
                    kind: ::macro_ron::v2::DeclarationKind,
                    position: ::macro_ron::v2::GrammarPosition,
                }
            },
        ),
        named_type(
            DECLARATION_MATCHER_TYPE,
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
            DECLARATION_LEAF_TYPE,
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
    items.extend(emit_owner_types(&inventory));
    items.extend(emit_runtime_impls(&inventory));
    if plan.has_open_declarations() {
        items.push(emit_required_declarations(plan));
    }
    let origins = plan.declaration_keys().to_vec();
    for item in &mut items {
        item.origins.clone_from(&origins);
    }
    items
}

fn emit_required_declarations(plan: &SemanticPlan) -> GeneratedItem {
    let requirements = plan
        .required_open_declarations()
        .map(|(construction, open)| {
            let kind = crate::emit::declaration_kind(open.kind());
            let name = syn::LitStr::new(open.name(), Span::call_site());
            let position = crate::emit::grammar_position(open.position());
            let feature = crate::emit::rules::open_verb_feature(plan, construction)?;
            Ok(quote! {
                DeclarationMatcher {
                    kind: #kind,
                    name: #name,
                    position: #position,
                    feature: #feature,
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()
        .expect("validated open declarations have sealed agreement constraints");
    GeneratedItem::new(
        ItemKey::Named {
            kind: crate::plan::NamedKind::Constant,
            name: "REQUIRED_DECLARATIONS".to_owned(),
        },
        quote! {
            pub(crate) const REQUIRED_DECLARATIONS: &[DeclarationMatcher] = &[#(#requirements),*];
        },
        plan.declaration_keys().to_vec(),
    )
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
    let signed_lexical = inventory.signed_decimal.map(|codec| {
        let variant = codec.codec_ident();
        quote! { #variant, }
    });
    let context_lexical_variants = inventory.context_identities.iter().map(|identity| {
        let variant = identity.aggregate_ident();
        quote! { #variant, }
    });
    let context_leaf_variants = inventory.context_identities.iter().map(|identity| {
        let variant = identity.aggregate_ident();
        let ty = identity.ident();
        quote! { #variant(#ty), }
    });
    let context_class_variants = inventory
        .context_identities
        .iter()
        .map(|identity| identity.aggregate_ident());
    let signed_leaf = inventory.signed_decimal.map(|codec| {
        let variant = codec.codec_ident();
        quote! { #variant(#variant), }
    });
    let signed_class = inventory.signed_decimal.map(|codec| {
        let variant = codec.codec_ident();
        quote! { #variant, }
    });

    vec![
        named_type(
            LEXICAL_TYPE,
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
                    #(#context_lexical_variants)*
                    #signed_lexical
                    Declaration(DeclarationMatcher),
                }
            },
        ),
        named_type(
            LEAF_TYPE,
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
                    #(#context_leaf_variants)*
                    #signed_leaf
                    Declaration(DeclarationLeaf),
                }
            },
        ),
        named_type(
            TERMINAL_CLASS_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub enum TerminalClass {
                    EndOfInput,
                    #(#vocab_class_variants,)*
                    #noun_class
                    #verb_class
                    #(#direct_class_variants,)*
                    #(#opaque_class_variants,)*
                    #(#context_class_variants,)*
                    #signed_class
                    Declaration(DeclarationClass),
                }
            },
        ),
        named_type(
            LEXICAL_TERMINAL_TYPE,
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

fn emit_owner_types(inventory: &RuntimeInventory<'_>) -> Vec<GeneratedItem> {
    let declaration_noun = inventory
        .declaration_noun
        .map(|_| quote! { DeclarationNoun, });
    vec![
        named_type(
            LEXICAL_PROVENANCE_KIND_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub enum LexicalProvenanceKind {
                    FormLiteral,
                    Vocab,
                    Lexeme,
                    Codec,
                    Identity,
                }
            },
        ),
        named_type(
            LEXICAL_OWNER_TEMPLATE_TYPE,
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
                    Identity {
                        declaration: &'static str,
                    },
                    #declaration_noun
                    Declaration {
                        kind: ::macro_ron::v2::DeclarationKind,
                        name: &'static str,
                    },
                }
            },
        ),
        named_type(
            LEXICAL_OWNER_IDENTITY_TYPE,
            quote! {
                #[derive(Clone)]
                pub(crate) enum LexicalOwnerIdentity {
                    Static {
                        kind: LexicalProvenanceKind,
                        stable_id: &'static str,
                    },
                    Declaration(std::sync::Arc<(
                        ::macro_ron::v2::DeclarationIdentity,
                        ::macro_ron::v2::SurfaceFeature,
                        std::sync::OnceLock<String>,
                    )>),
                }
            },
        ),
        named_type(
            LEXICAL_OWNER_TYPE,
            quote! {
                pub struct LexicalOwner {
                    identity: LexicalOwnerIdentity,
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

#[allow(
    clippy::too_many_lines,
    reason = "the generated exhaustive class and label projections stay visibly paired"
)]
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
    let signed_class_arm = inventory.signed_decimal.map(|codec| {
        let variant = codec.codec_ident();
        quote! { Lexical::#variant => TerminalClass::#variant, }
    });
    let context_class_arms = inventory.context_identities.iter().map(|identity| {
        let variant = identity.aggregate_ident();
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
    let signed_label = inventory.signed_decimal.map(|codec| {
        let variant = codec.codec_ident();
        let label = syn::LitStr::new(
            &snake_case(codec.codec_name()).replace('_', " "),
            Span::call_site(),
        );
        quote! { TerminalClass::#variant => #label, }
    });
    let context_labels = inventory.context_identities.iter().map(|identity| {
        let variant = identity.aggregate_ident();
        let label = syn::LitStr::new(
            &snake_case(&variant.to_string()).replace('_', " "),
            identity.origin_span(),
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
                            #(#context_class_arms)*
                            #signed_class_arm
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
                            #(#context_labels)*
                            #signed_label
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

#[expect(
    clippy::too_many_lines,
    reason = "the generated owner ABI and its exhaustive value instantiation remain co-located"
)]
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
                ) => Some(LexicalOwner::static_owner(
                    LexicalProvenanceKind::Vocab,
                    #stable_id,
                )),
            }
        })
    });
    let context_owner_arms = inventory.context_identities.iter().flat_map(|identity| {
        let declaration_ident = identity.ident();
        let aggregate_ident = identity.aggregate_ident();
        let declaration = syn::LitStr::new(identity.name(), identity.origin_span());
        identity.arms().iter().map(move |arm| {
            let member = arm.variant();
            let stable_id = syn::LitStr::new(
                &format!("identity:{}/{}", identity.name(), member),
                Span::call_site(),
            );
            quote! {
                (
                    LexicalOwnerTemplate::Identity { declaration: #declaration },
                    Leaf::#aggregate_ident(#declaration_ident::#member),
                ) => Some(LexicalOwner::static_owner(
                    LexicalProvenanceKind::Identity,
                    #stable_id,
                )),
            }
        })
    });
    let verb_lexeme_owner_arms = inventory.verb_lexeme.into_iter().flat_map(|lexeme| {
        let declaration_ident = lexeme.name_ident();
        let declaration = syn::LitStr::new(lexeme.name(), lexeme.name_ident().span());
        lexeme.surfaces().iter().map(move |row| {
            let member_ident = emitted_ident(row.member(), Span::call_site());
            let member = syn::LitStr::new(row.member(), Span::call_site());
            let agreement = match row.feature() {
                macro_ron::v2::SurfaceFeature::Bare => quote! { Agreement::Bare },
                macro_ron::v2::SurfaceFeature::ThirdPersonSingular => {
                    quote! { Agreement::ThirdPersonSingular }
                }
                macro_ron::v2::SurfaceFeature::Singular
                | macro_ron::v2::SurfaceFeature::Plural
                | macro_ron::v2::SurfaceFeature::Fixed => {
                    unreachable!("validated verb lexeme has the Agreement feature axis")
                }
            };
            let stable_id =
                crate::emit::closed_lexeme_owner_id(lexeme.name(), row.member(), row.feature());
            quote! {
                (
                    LexicalOwnerTemplate::Lexeme {
                        declaration: #declaration,
                        member: #member,
                    },
                    Leaf::Verb {
                        lexeme: #declaration_ident::#member_ident,
                        agreement: #agreement,
                    },
                ) => Some(LexicalOwner::static_owner(
                    LexicalProvenanceKind::Lexeme,
                    #stable_id,
                )),
            }
        })
    });
    let declaration_noun_owner = inventory.declaration_noun.map(|codec| {
        let noun = codec.codec_ident();
        let closed = codec.closed_lexeme();
        let closed_owner_arms = inventory
            .noun_lexeme
            .expect("validated declaration noun has its closed lexeme provider")
            .surfaces()
            .iter()
            .map(|row| {
                let member = emitted_ident(row.member(), Span::call_site());
                let number = match row.feature() {
                    macro_ron::v2::SurfaceFeature::Singular => quote! { Number::Singular },
                    macro_ron::v2::SurfaceFeature::Plural => quote! { Number::Plural },
                    macro_ron::v2::SurfaceFeature::Bare
                    | macro_ron::v2::SurfaceFeature::ThirdPersonSingular
                    | macro_ron::v2::SurfaceFeature::Fixed => {
                        unreachable!("validated noun lexeme has the Number feature axis")
                    }
                };
                let stable_id = crate::emit::closed_lexeme_owner_id(
                    &closed.to_string(),
                    row.member(),
                    row.feature(),
                );
                quote! {
                    (
                        LexicalOwnerTemplate::DeclarationNoun,
                        Leaf::Noun {
                            noun: #noun::Lexeme(#closed::#member),
                            number: #number,
                        },
                    ) => Some(LexicalOwner::static_owner(
                        LexicalProvenanceKind::Lexeme,
                        #stable_id,
                    )),
                }
            });
        quote! {
            #(#closed_owner_arms)*
            (
                LexicalOwnerTemplate::DeclarationNoun,
                Leaf::Noun {
                    noun: #noun::Declaration(declaration),
                    number: _,
                },
            ) => Some(LexicalOwner::declaration_owner(
                declaration.id().clone(),
                declaration.feature(),
            )),
        }
    });

    vec![
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: "declaration_lexeme_owner_id".to_owned(),
            },
            quote! {
                pub(crate) fn declaration_lexeme_owner_id(
                    id: &::macro_ron::v2::DeclarationIdentity,
                    feature: ::macro_ron::v2::SurfaceFeature,
                ) -> String {
                    let kind = match id.kind() {
                        ::macro_ron::v2::DeclarationKind::KeywordAction => "keyword_action",
                        ::macro_ron::v2::DeclarationKind::KeywordAbility => "keyword_ability",
                        ::macro_ron::v2::DeclarationKind::Subtype(category) => match category {
                            ::macro_ron::v2::SubtypeCategory::Artifact => "artifact_subtype",
                            ::macro_ron::v2::SubtypeCategory::Battle => "battle_subtype",
                            ::macro_ron::v2::SubtypeCategory::Creature => "creature_subtype",
                            ::macro_ron::v2::SubtypeCategory::Enchantment => "enchantment_subtype",
                            ::macro_ron::v2::SubtypeCategory::Land => "land_subtype",
                            ::macro_ron::v2::SubtypeCategory::Planeswalker => "planeswalker_subtype",
                            ::macro_ron::v2::SubtypeCategory::Spell => "spell_subtype",
                        },
                        ::macro_ron::v2::DeclarationKind::Type => "type",
                        ::macro_ron::v2::DeclarationKind::CounterKind => "counter_kind",
                        ::macro_ron::v2::DeclarationKind::Designation => "designation",
                    };
                    let feature = match feature {
                        ::macro_ron::v2::SurfaceFeature::Bare => "bare",
                        ::macro_ron::v2::SurfaceFeature::ThirdPersonSingular => {
                            "third_person_singular"
                        }
                        ::macro_ron::v2::SurfaceFeature::Singular => "singular",
                        ::macro_ron::v2::SurfaceFeature::Plural => "plural",
                        ::macro_ron::v2::SurfaceFeature::Fixed => "fixed",
                    };
                    LexicalOwner::construct_label(|| {
                        format!("lexeme:{kind}/{}/{feature}", id.name())
                    })
                }
            },
            Vec::new(),
        ),
        impl_item(
            Some("LexicalOwner"),
            "LexicalOwner",
            quote! {
                impl LexicalOwner {
                    #[cfg(test)]
                    fn update_label_constructions(set: Option<usize>, increment: bool) -> usize {
                        std::thread_local! {
                            static COUNT: std::cell::Cell<usize> = const {
                                std::cell::Cell::new(0)
                            };
                        }
                        COUNT.with(|count| {
                            if let Some(value) = set {
                                count.set(value);
                            }
                            if increment {
                                count.set(count.get() + 1);
                            }
                            count.get()
                        })
                    }

                    #[cfg(test)]
                    pub(crate) fn reset_label_constructions() {
                        Self::update_label_constructions(Some(0), false);
                    }

                    #[cfg(test)]
                    pub(crate) fn label_constructions() -> usize {
                        Self::update_label_constructions(None, false)
                    }

                    fn construct_label(build: impl FnOnce() -> String) -> String {
                        #[cfg(test)]
                        Self::update_label_constructions(None, true);
                        build()
                    }

                    pub(crate) fn static_owner(
                        kind: LexicalProvenanceKind,
                        stable_id: &'static str,
                    ) -> Self {
                        Self {
                            identity: LexicalOwnerIdentity::Static { kind, stable_id },
                        }
                    }

                    pub(crate) fn declaration_owner(
                        id: ::macro_ron::v2::DeclarationIdentity,
                        feature: ::macro_ron::v2::SurfaceFeature,
                    ) -> Self {
                        Self {
                            identity: LexicalOwnerIdentity::Declaration(std::sync::Arc::new((
                                id,
                                feature,
                                std::sync::OnceLock::new(),
                            ))),
                        }
                    }

                    pub const fn kind(&self) -> LexicalProvenanceKind {
                        match self.identity {
                            LexicalOwnerIdentity::Static { kind, .. } => kind,
                            LexicalOwnerIdentity::Declaration(_) => LexicalProvenanceKind::Lexeme,
                        }
                    }

                    pub fn stable_id(&self) -> &str {
                        match &self.identity {
                            LexicalOwnerIdentity::Static { stable_id, .. } => stable_id,
                            LexicalOwnerIdentity::Declaration(declaration) => declaration
                                .2
                                .get_or_init(|| declaration_lexeme_owner_id(&declaration.0, declaration.1)),
                        }
                    }

                    pub(crate) fn stable_id_owned(&self) -> String {
                        match &self.identity {
                            LexicalOwnerIdentity::Static { stable_id, .. } => {
                                Self::construct_label(|| stable_id.to_string())
                            }
                            LexicalOwnerIdentity::Declaration(_) => self.stable_id().to_owned(),
                        }
                    }
                }
            },
        ),
        impl_item(
            Some("Debug"),
            "LexicalOwner",
            quote! {
                impl std::fmt::Debug for LexicalOwner {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter
                            .debug_struct("LexicalOwner")
                            .field("kind", &self.kind())
                            .field("stable_id", &self.stable_id())
                            .finish()
                    }
                }
            },
        ),
        impl_item(
            Some("Clone"),
            "LexicalOwner",
            quote! {
                impl Clone for LexicalOwner {
                    fn clone(&self) -> Self {
                        Self {
                            identity: self.identity.clone(),
                        }
                    }
                }
            },
        ),
        impl_item(
            Some("PartialEq"),
            "LexicalOwner",
            quote! {
                impl PartialEq for LexicalOwner {
                    fn eq(&self, other: &Self) -> bool {
                        match (&self.identity, &other.identity) {
                            (
                                LexicalOwnerIdentity::Static {
                                    kind: left_kind,
                                    stable_id: left_id,
                                },
                                LexicalOwnerIdentity::Static {
                                    kind: right_kind,
                                    stable_id: right_id,
                                },
                            ) => left_kind == right_kind && left_id == right_id,
                            (
                                LexicalOwnerIdentity::Declaration(left),
                                LexicalOwnerIdentity::Declaration(right),
                            ) => left.0 == right.0 && left.1 == right.1,
                            _ => false,
                        }
                    }
                }
            },
        ),
        impl_item(
            Some("Eq"),
            "LexicalOwner",
            quote! { impl Eq for LexicalOwner {} },
        ),
        impl_item(
            Some("Ord"),
            "LexicalOwner",
            quote! {
                impl Ord for LexicalOwner {
                    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                        self.kind().cmp(&other.kind()).then_with(|| {
                            match (&self.identity, &other.identity) {
                                (
                                    LexicalOwnerIdentity::Static { stable_id: left, .. },
                                    LexicalOwnerIdentity::Static { stable_id: right, .. },
                                ) => left.cmp(right),
                                (
                                    LexicalOwnerIdentity::Declaration(left),
                                    LexicalOwnerIdentity::Declaration(right),
                                ) => left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)),
                                (LexicalOwnerIdentity::Static { .. }, _) => std::cmp::Ordering::Less,
                                (LexicalOwnerIdentity::Declaration(_), _) => std::cmp::Ordering::Greater,
                            }
                        })
                    }
                }
            },
        ),
        impl_item(
            Some("PartialOrd"),
            "LexicalOwner",
            quote! {
                impl PartialOrd for LexicalOwner {
                    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                        Some(self.cmp(other))
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
                            ) => Some(LexicalOwner::static_owner(kind, stable_id)),
                            #(#vocab_owner_arms)*
                            #(#context_owner_arms)*
                            #(#verb_lexeme_owner_arms)*
                            (
                                LexicalOwnerTemplate::Declaration { kind, name },
                                Leaf::Declaration(declaration),
                            ) if declaration.id.kind() == kind && declaration.id.name() == name => {
                                Some(LexicalOwner::declaration_owner(
                                    declaration.id.clone(),
                                    declaration.feature,
                                ))
                            },
                            #declaration_noun_owner
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
