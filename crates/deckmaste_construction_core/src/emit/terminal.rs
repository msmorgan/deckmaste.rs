use proc_macro2::Span;
use quote::quote;

use crate::identifier::emitted_ident;
use crate::identifier::key as identifier_key;
use crate::identifier::lexeme_surface_helper;
use crate::identifier::snake_case;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::plan::TerminalContribution;
use crate::plan::TerminalKind;
use crate::plan::TerminalVariantContribution;
use crate::semantic::SemanticPlan;
use crate::semantic::TerminalPlan;

#[allow(
    clippy::too_many_lines,
    clippy::unnecessary_wraps,
    reason = "terminal kinds stay together in one source-ordered emission pass"
)]
pub(crate) fn emit(
    validated: &SemanticPlan,
) -> syn::Result<(Vec<GeneratedItem>, Vec<TerminalContribution>)> {
    let mut items = Vec::new();
    let mut contributions = Vec::new();

    for terminal in validated.terminals() {
        match terminal {
            TerminalPlan::Vocab(row) => {
                let name = row.name().to_owned();
                let origin = DeclarationKey::new(DeclarationKind::Vocab, &name);
                let variants = row
                    .variants()
                    .iter()
                    .map(|variant| {
                        TerminalVariantContribution::new(
                            identifier_key(variant.name()),
                            Some(variant.word().value()),
                        )
                    })
                    .collect::<Vec<_>>();
                let variant_idents = row
                    .variants()
                    .iter()
                    .map(|variant| {
                        emitted_ident(&identifier_key(variant.name()), variant.name().span())
                    })
                    .collect::<Vec<_>>();
                let ident = emitted_ident(&name, row.name_ident().span());
                let tokens = quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                    pub enum #ident {
                        #(#variant_idents),*
                    }
                };
                items.push(GeneratedItem::new(
                    ItemKey::named_type(&name),
                    tokens,
                    vec![origin.clone()],
                ));
                if row
                    .feature_members(crate::feature::Feature::ModifierLicense)
                    .is_some()
                {
                    items.push(emit_vocab_modifier_license_helper(row, origin.clone())?);
                }
                contributions.push(TerminalContribution::new(
                    origin,
                    TerminalKind::Vocab,
                    &name,
                    Some(format!("render_{}", snake_case(&name))),
                    variants,
                    false,
                ));
            }
            TerminalPlan::Lexeme(row) => {
                let name = row.name().to_owned();
                let verb_provider = row.is_verb_provider();
                let origin = DeclarationKey::new(DeclarationKind::Lexeme, &name);
                let variants = row
                    .variants()
                    .iter()
                    .map(|variant| TerminalVariantContribution::new(identifier_key(variant), None))
                    .collect::<Vec<_>>();
                let variant_idents = row
                    .variants()
                    .iter()
                    .map(|variant| emitted_ident(&identifier_key(variant), variant.span()))
                    .collect::<Vec<_>>();
                let ident = emitted_ident(&name, row.name_ident().span());
                let tokens = if verb_provider {
                    quote! {
                        #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                        pub enum #ident {
                            #(#variant_idents),*
                        }
                    }
                } else {
                    quote! {
                        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                        pub enum #ident {
                            #(#variant_idents),*
                        }
                    }
                };
                items.push(GeneratedItem::new(
                    ItemKey::named_type(&name),
                    tokens,
                    vec![origin.clone()],
                ));
                items.push(emit_lexeme_surface_helper(row, origin.clone())?);
                if row
                    .feature_members(crate::feature::Feature::Compoundability)
                    .is_some()
                {
                    items.push(emit_lexeme_compoundability_helper(row, origin.clone())?);
                }
                if row
                    .feature_members(crate::feature::Feature::ModifierLicense)
                    .is_some()
                {
                    items.push(emit_lexeme_modifier_license_helper(row, origin.clone())?);
                }
                contributions.push(TerminalContribution::new(
                    origin,
                    TerminalKind::Lexeme,
                    &name,
                    None,
                    variants,
                    verb_provider,
                ));
            }
            TerminalPlan::Binding(binding) => {
                let Some(row) = binding.declaration_verb() else {
                    continue;
                };
                debug_assert_eq!(row.source_index(), binding.source_index());
                debug_assert!(matches!(
                    row.feature_axis(),
                    crate::feature::Feature::Agreement | crate::feature::Feature::Participle
                ));
                let origin = row.origin().clone();
                let verb = row.codec_ident();
                let declaration = row.declaration_value_ident();
                let position = crate::emit::grammar_position(row.position());
                let frame_atoms = row
                    .frame_key()
                    .atoms()
                    .iter()
                    .map(|atom| match atom {
                        crate::semantic::VerbFrameAtom::Literal(literal) => {
                            let literal = syn::LitStr::new(literal, Span::call_site());
                            quote! { VerbFrameAtom::Literal(#literal) }
                        }
                        crate::semantic::VerbFrameAtom::Amount => quote! { VerbFrameAtom::Amount },
                        crate::semantic::VerbFrameAtom::ObjectNounPhrase => {
                            quote! { VerbFrameAtom::ObjectNounPhrase }
                        }
                        crate::semantic::VerbFrameAtom::PredicativeComplement => {
                            quote! { VerbFrameAtom::PredicativeComplement }
                        }
                    })
                    .collect::<Vec<_>>();
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.declaration_value_ident().to_string()),
                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct #declaration {
                            id: ::macro_ron::v2::DeclarationIdentity,
                        }
                    },
                    vec![origin.clone()],
                ));
                items.push(GeneratedItem::new(
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty: row.declaration_value_ident().to_string(),
                    },
                    quote! {
                        impl #declaration {
                            pub fn new(
                                environment: &crate::environment::ParserEnvironment,
                                id: ::macro_ron::v2::DeclarationIdentity,
                            ) -> Option<Self> {
                                let recipe = environment.grammar_recipe(&id)?;
                                if recipe.position() != #position {
                                    return None;
                                }
                                let ::macro_ron::v2::GrammarRecipe::Verb { valence } = recipe else {
                                    return None;
                                };
                                let frame = VerbFrameKey::new(&[#(#frame_atoms),*]);
                                if !frame.matches_valence(valence) {
                                    return None;
                                }
                                [
                                    ::macro_ron::v2::SurfaceFeature::Bare,
                                    ::macro_ron::v2::SurfaceFeature::ThirdPersonSingular,
                                ]
                                .into_iter()
                                .all(|feature| environment.surface(&id, feature).is_some())
                                .then_some(Self { id })
                            }

                            pub fn id(&self) -> &::macro_ron::v2::DeclarationIdentity {
                                &self.id
                            }
                        }
                    },
                    vec![origin.clone()],
                ));
                let tokens = if let Some(closed) = row.closed_lexeme() {
                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub enum #verb {
                            Lexeme(#closed),
                            Declaration(#declaration),
                        }
                    }
                } else {
                    quote! { pub type #verb = #declaration; }
                };
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.codec_name()),
                    tokens,
                    vec![origin],
                ));
            }
            TerminalPlan::ContextIdentity(row) => {
                let origin = row.origin().clone();
                let ty = row.ident();
                let variants = row
                    .arms()
                    .iter()
                    .map(crate::semantic::ContextIdentityArmPlan::variant);
                let surface_arms = row.arms().iter().map(|arm| {
                    let variant = arm.variant();
                    let accessor = arm.accessor();
                    quote! { Self::#variant => context.#accessor() }
                });
                let canonical = row.canonical();
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.name()),
                    quote! {
                        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                        pub enum #ty { #(#variants),* }
                    },
                    vec![origin.clone()],
                ));
                items.push(GeneratedItem::new(
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty: row.name().to_owned(),
                    },
                    quote! {
                        impl #ty {
                            pub(crate) fn valid_in(self, context: &ParseContext<'_>) -> bool {
                                self == Self::#canonical
                                    || self.surface(context) != Self::#canonical.surface(context)
                            }

                            pub(crate) fn surface<'a>(
                                self,
                                context: &'a ParseContext<'a>,
                            ) -> &'a str {
                                match self { #(#surface_arms,)* }
                            }
                        }
                    },
                    vec![origin],
                ));
            }
            TerminalPlan::CatalogIdentity(row) => {
                let origin = row.origin().clone();
                let ty = row.ident();
                let provider = row.provider();
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.name()),
                    quote! {
                        /// A canonical identity validated against its generated provider.
                        ///
                        /// Rendering and derived-feature lookup require the same frozen parser
                        /// environment used to construct this value, or an environment containing
                        /// an equivalent row for this provider and canonical identity.
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct #ty {
                            provider: CatalogProvider,
                            canonical_identity: std::sync::Arc<str>,
                        }
                    },
                    vec![origin.clone()],
                ));
                items.push(GeneratedItem::new(
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty: row.name().to_owned(),
                    },
                    quote! {
                        impl #ty {
                            /// Validates and constructs one canonical provider identity.
                            ///
                            /// The returned value remains environment-affine: render it only with
                            /// this environment or one containing an equivalent provider row.
                            pub fn new(
                                environment: &crate::environment::ParserEnvironment,
                                canonical_identity: &str,
                            ) -> Option<Self> {
                                environment
                                    .catalog_identity(CatalogProvider::#provider, canonical_identity)
                                    .map(Self::from_canonical)
                            }

                            pub const fn provider(&self) -> CatalogProvider {
                                self.provider
                            }

                            pub fn canonical_identity(&self) -> &str {
                                &self.canonical_identity
                            }

                            pub(crate) fn from_canonical(
                                canonical_identity: std::sync::Arc<str>,
                            ) -> Self {
                                Self {
                                    provider: CatalogProvider::#provider,
                                    canonical_identity,
                                }
                            }
                        }
                    },
                    vec![origin],
                ));
            }
            TerminalPlan::SignedDecimal(row) => {
                let origin = row.origin().clone();
                let sign = row.sign_type();
                let positive = row.positive_variant();
                let negative = row.negative_variant();
                let codec = row.codec_ident();
                let magnitude = match row.magnitude() {
                    crate::semantic::UnsignedPrimitive::U32 => quote! { u32 },
                    crate::semantic::UnsignedPrimitive::NonZeroU32 => {
                        quote! { ::std::num::NonZeroU32 }
                    }
                };
                items.push(GeneratedItem::new(
                    ItemKey::named_type(sign.to_string()),
                    quote! {
                        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                        pub enum #sign { #positive, #negative }
                    },
                    vec![origin.clone()],
                ));
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.codec_name()),
                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct #codec {
                            pub sign: #sign,
                            pub magnitude: #magnitude,
                        }
                    },
                    vec![origin],
                ));
            }
            TerminalPlan::UnsignedNumber(row) => {
                let origin = row.origin().clone();
                let codec = row.codec_ident();
                let magnitude = match row.magnitude() {
                    crate::semantic::UnsignedPrimitive::U32 => quote! { u32 },
                    crate::semantic::UnsignedPrimitive::NonZeroU32 => {
                        quote! { ::std::num::NonZeroU32 }
                    }
                };
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.codec_name()),
                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct #codec {
                            pub magnitude: #magnitude,
                        }
                    },
                    vec![origin],
                ));
            }
            TerminalPlan::DeclarationNoun(row) => {
                let origin = row.origin().clone();
                let noun = row.codec_ident();
                let declaration = row.declaration_value_ident();
                let closed_variant = row
                    .closed_lexeme()
                    .map(|closed| quote! { Lexeme(#closed), });
                let allowed = row.kinds().iter().map(|kind| match kind {
                    crate::semantic::DeclarationKindFamily::Type => {
                        quote! { ::macro_ron::v2::DeclarationKind::Type }
                    }
                    crate::semantic::DeclarationKindFamily::Subtype => {
                        quote! { ::macro_ron::v2::DeclarationKind::Subtype(_) }
                    }
                    crate::semantic::DeclarationKindFamily::SubtypeFamily(family) => {
                        let family = crate::emit::subtype_category(*family);
                        quote! { ::macro_ron::v2::DeclarationKind::Subtype(#family) }
                    }
                });
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.declaration_value_ident().to_string()),
                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct #declaration {
                            id: ::macro_ron::v2::DeclarationIdentity,
                        }
                    },
                    vec![origin.clone()],
                ));
                items.push(GeneratedItem::new(
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty: row.declaration_value_ident().to_string(),
                    },
                    quote! {
                        impl #declaration {
                            pub fn new(
                                environment: &crate::environment::ParserEnvironment,
                                id: ::macro_ron::v2::DeclarationIdentity,
                            ) -> Option<Self> {
                                let has_surface = environment
                                    .surface(&id, ::macro_ron::v2::SurfaceFeature::Singular)
                                    .or_else(|| environment.surface(
                                        &id,
                                        ::macro_ron::v2::SurfaceFeature::Plural,
                                    ))
                                    .is_some();
                                has_surface.then(|| Self::from_reading(id)).flatten()
                            }

                            pub(crate) fn from_reading(
                                id: ::macro_ron::v2::DeclarationIdentity,
                            ) -> Option<Self> {
                                matches!(id.kind(), #(#allowed)|*)
                                    .then_some(Self { id })
                            }

                            pub fn id(&self) -> &::macro_ron::v2::DeclarationIdentity {
                                &self.id
                            }
                        }
                    },
                    vec![origin.clone()],
                ));
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.codec_name()),
                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub enum #noun {
                            #closed_variant
                            Declaration(#declaration),
                        }
                    },
                    vec![origin],
                ));
            }
            TerminalPlan::DeclarationDeterminative(row) => {
                let origin = row.origin().clone();
                let ty = row.codec_ident();
                let lemma = row.lemma_ident();
                let variants = row.closed().iter().map(|member| member.lemma());
                let allowed = row.kinds().iter().map(|kind| crate::emit::declaration_kind(*kind));
                items.push(GeneratedItem::new(ItemKey::named_type(lemma.to_string()), quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum #lemma { #(#variants),* }
                }, vec![origin.clone()]));
                items.push(GeneratedItem::new(ItemKey::named_type(ty.to_string()), quote! {
                    #[derive(Debug, Clone, PartialEq, Eq)] pub enum #ty { Closed(#lemma), Declared(::macro_ron::v2::DeclarationIdentity) }
                }, vec![origin.clone()]));
                items.push(GeneratedItem::new(ItemKey::Impl {
                    trait_name: None,
                    self_ty: ty.to_string(),
                }, quote! {
                    impl #ty { pub(crate) fn declared(id: ::macro_ron::v2::DeclarationIdentity) -> Option<Self> { matches!(id.kind(), #(#allowed)|*).then_some(Self::Declared(id)) } }
                }, vec![origin]));
            }
            TerminalPlan::DeclarationTerm(row) => {
                let origin = row.origin().clone();
                let term = row.codec_ident();
                let allowed = row
                    .kinds()
                    .iter()
                    .map(|kind| crate::emit::declaration_kind(*kind));
                let position = crate::emit::grammar_position(row.position());
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.codec_name()),
                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct #term {
                            id: ::macro_ron::v2::DeclarationIdentity,
                        }
                    },
                    vec![origin.clone()],
                ));
                items.push(GeneratedItem::new(
                    ItemKey::Impl {
                        trait_name: None,
                        self_ty: row.codec_name().to_owned(),
                    },
                    quote! {
                        impl #term {
                            pub fn new(
                                environment: &crate::environment::ParserEnvironment,
                                id: ::macro_ron::v2::DeclarationIdentity,
                            ) -> Option<Self> {
                                let recipe = environment.grammar_recipe(&id)?;
                                (recipe.position() == #position)
                                    .then_some(())
                                    .and_then(|_| environment.surface(
                                        &id,
                                        ::macro_ron::v2::SurfaceFeature::Fixed,
                                    ))
                                    .and_then(|_| Self::from_reading(id))
                            }

                            pub(crate) fn from_reading(
                                id: ::macro_ron::v2::DeclarationIdentity,
                            ) -> Option<Self> {
                                matches!(id.kind(), #(#allowed)|*)
                                    .then_some(Self { id })
                            }

                            pub fn id(&self) -> &::macro_ron::v2::DeclarationIdentity {
                                &self.id
                            }
                        }
                    },
                    vec![origin],
                ));
            }
        }
    }
    let catalog_identities = validated.runtime_catalog_identities().collect::<Vec<_>>();
    if !catalog_identities.is_empty() {
        let mut providers = std::collections::BTreeMap::new();
        for (_, identity) in &catalog_identities {
            providers
                .entry(identifier_key(identity.provider()))
                .or_insert_with(|| identity.provider().clone());
        }
        let variants = providers.values();
        let names = providers.values().map(|provider| {
            let name = syn::LitStr::new(&provider.to_string(), provider.span());
            quote! { Self::#provider => #name }
        });
        let required = providers.values();
        let origins = catalog_identities
            .iter()
            .map(|(_, identity)| identity.origin().clone())
            .collect::<Vec<_>>();
        items.push(GeneratedItem::new(
            ItemKey::named_type("CatalogProvider"),
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
                pub enum CatalogProvider { #(#variants),* }
            },
            origins.clone(),
        ));
        items.push(GeneratedItem::new(
            ItemKey::Impl {
                trait_name: None,
                self_ty: "CatalogProvider".to_owned(),
            },
            quote! {
                impl CatalogProvider {
                    pub const fn name(self) -> &'static str {
                        match self { #(#names,)* }
                    }
                }
            },
            origins.clone(),
        ));
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Constant,
                name: "REQUIRED_CATALOG_PROVIDERS".to_owned(),
            },
            quote! {
                pub(crate) const REQUIRED_CATALOG_PROVIDERS: &[CatalogProvider] =
                    &[#(CatalogProvider::#required),*];
            },
            origins,
        ));
    }
    Ok((items, contributions))
}

fn emit_vocab_modifier_license_helper(
    vocab: &crate::semantic::VocabPlan,
    origin: DeclarationKey,
) -> syn::Result<GeneratedItem> {
    let function_name = format!("modifier_license_{}", snake_case(vocab.name()));
    let function = emitted_ident(&function_name, vocab.name_ident().span());
    let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
    let members = vocab
        .feature_members(crate::feature::Feature::ModifierLicense)
        .expect("requested sealed modifier-license metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, vocab.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::Unrestricted => quote! { ModifierLicense::Unrestricted },
                crate::feature::FeatureValue::LocalDeterminer => quote! { ModifierLicense::LocalDeterminer },
                _ => unreachable!("sealed modifier license has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> ModifierLicense { match value { #(#members),* } } },
        vec![origin],
    ))
}

fn emit_lexeme_surface_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> syn::Result<GeneratedItem> {
    let function_name = lexeme_surface_helper(lexeme.name());
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let (feature_ty, feature_argument) = match lexeme.morphology().feature() {
        crate::Feature::Agreement => (quote! { Agreement }, quote! { agreement }),
        crate::Feature::Number => (quote! { Number }, quote! { number }),
        crate::Feature::Participle => (quote! { Participle }, quote! { participle }),
        crate::Feature::Cardinality
        | crate::Feature::Compoundability
        | crate::Feature::ModifierLicense
        | crate::Feature::DeterminerNumber
        | crate::Feature::FusedHeadLicense
        | crate::Feature::NominalForm
        | crate::Feature::NominalLicense
        | crate::Feature::Onset
        | crate::Feature::PossessiveEnding => {
            unreachable!("derived surface features are not morphology axes")
        }
    };
    let arms = lexeme
        .surfaces()
        .iter()
        .map(|row| {
            let member = emitted_ident(row.member(), Span::call_site());
            let feature = match row.feature() {
                macro_ron::v2::SurfaceFeature::Bare => quote! { Agreement::Bare },
                macro_ron::v2::SurfaceFeature::ThirdPersonSingular => {
                    quote! { Agreement::ThirdPersonSingular }
                }
                macro_ron::v2::SurfaceFeature::Singular => quote! { Number::Singular },
                macro_ron::v2::SurfaceFeature::Plural => quote! { Number::Plural },
                macro_ron::v2::SurfaceFeature::Participle => {
                    quote! { Participle::Participle }
                }
                macro_ron::v2::SurfaceFeature::Fixed => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "sealed lexeme surface has an unsupported fixed feature",
                    ));
                }
            };
            let surface = syn::LitStr::new(row.surface(), Span::call_site());
            Ok(quote! { (#ty::#member, #feature) => #surface })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function(lexeme: #ty, #feature_argument: #feature_ty,) -> &'static str {
                match (lexeme, #feature_argument) { #(#arms,)* }
            }
        },
        vec![origin],
    ))
}

fn emit_lexeme_compoundability_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> syn::Result<GeneratedItem> {
    let function_name = format!("compoundability_{}", snake_case(lexeme.name()));
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let members = lexeme
        .feature_members(crate::feature::Feature::Compoundability)
        .expect("requested sealed compoundability metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, lexeme.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::Compoundable => quote! { Compoundability::Compoundable },
                crate::feature::FeatureValue::NonCompoundable => quote! { Compoundability::NonCompoundable },
                _ => unreachable!("sealed compoundability has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> Compoundability { match value { #(#members),* } } },
        vec![origin],
    ))
}

fn emit_lexeme_modifier_license_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> syn::Result<GeneratedItem> {
    let function_name = format!("modifier_license_{}", snake_case(lexeme.name()));
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let members = lexeme
        .feature_members(crate::feature::Feature::ModifierLicense)
        .expect("requested sealed modifier-license metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, lexeme.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::Unrestricted => quote! { ModifierLicense::Unrestricted },
                crate::feature::FeatureValue::LocalDeterminer => quote! { ModifierLicense::LocalDeterminer },
                _ => unreachable!("sealed modifier license has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> ModifierLicense { match value { #(#members),* } } },
        vec![origin],
    ))
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::items_after_statements,
        clippy::too_many_lines,
        clippy::type_complexity,
        reason = "literal full-surface structural oracles are intentionally table-dense"
    )]
    use syn::Item;
    use syn::parse::Parser;

    use crate::ItemKey;
    use crate::TerminalKind;
    use crate::test_support::representative_expansion;

    #[test]
    fn emits_vocab_and_lexeme_enums_with_resolved_derives_and_word_data() {
        let expansion = representative_expansion();

        assert_enum(&expansion, "Words", &["First", "Second"], false);
        assert_enum(&expansion, "Nouns", &["Person"], false);
        assert_enum(&expansion, "Verbs", &["Act"], true);

        let contributions = expansion.terminal_contributions();
        assert_eq!(contributions.len(), 3);
        assert_eq!(contributions[0].kind(), TerminalKind::Vocab);
        assert_eq!(contributions[0].name(), "Words");
        assert_eq!(contributions[0].render_function(), Some("render_words"));
        assert_eq!(
            contributions[0]
                .variants()
                .iter()
                .map(|variant| (variant.name(), variant.word()))
                .collect::<Vec<_>>(),
            [("First", Some("first")), ("Second", Some("second"))]
        );
        assert_eq!(contributions[1].kind(), TerminalKind::Lexeme);
        assert_eq!(contributions[1].render_function(), None);
        assert!(!contributions[1].is_verb_provider());
        assert!(contributions[2].is_verb_provider());
    }

    #[test]
    fn synthetic_projection_has_exact_terminal_tables_and_capabilities() {
        let expansion = crate::test_support::synthetic_projection_expansion();

        assert_enum(&expansion, "Mode", &["Solo", "Group"], false);
        assert_enum(&expansion, "ObjectStem", &["Widget"], false);
        assert_enum(&expansion, "ActionStem", &["Activate"], true);

        let contributions = expansion.terminal_contributions();
        assert_eq!(contributions.len(), 3);
        assert_eq!(
            contributions
                .iter()
                .map(|terminal| (
                    terminal.name(),
                    terminal.kind(),
                    terminal.render_function(),
                    terminal.is_verb_provider(),
                ))
                .collect::<Vec<_>>(),
            [
                ("Mode", TerminalKind::Vocab, Some("render_mode"), false),
                ("ObjectStem", TerminalKind::Lexeme, None, false),
                ("ActionStem", TerminalKind::Lexeme, None, true),
            ]
        );
        assert_eq!(
            contributions[0]
                .variants()
                .iter()
                .map(|variant| (variant.name(), variant.word()))
                .collect::<Vec<_>>(),
            [("Solo", Some("solo")), ("Group", Some("group"))],
        );

        let emitted = expansion
            .items()
            .iter()
            .filter_map(|item| match &item.key {
                ItemKey::Named {
                    kind: crate::NamedKind::Type,
                    name,
                } if ["Mode", "ObjectStem", "ActionStem"].contains(&name.as_str()) => {
                    Some(name.as_str())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(emitted, ["Mode", "ObjectStem", "ActionStem"]);
    }

    #[test]
    fn generated_morphology_emits_total_surface_helpers() {
        let expansion = crate::test_support::generated_morphology_expansion();
        let selected = expansion
            .items()
            .iter()
            .filter(|item| {
                matches!(
                    &item.key,
                    ItemKey::Named { name, .. }
                        if matches!(
                            name.as_str(),
                            "VerbLexeme"
                                | "surface_for_verb_lexeme"
                                | "NounLexeme"
                                | "surface_for_noun_lexeme"
                        )
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            selected.iter().map(|item| &item.key).collect::<Vec<_>>(),
            [
                &ItemKey::named_type("VerbLexeme"),
                &ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name: "surface_for_verb_lexeme".into(),
                },
                &ItemKey::named_type("NounLexeme"),
                &ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name: "surface_for_noun_lexeme".into(),
                },
            ]
        );
        assert_eq!(
            selected[1].origins,
            [crate::DeclarationKey::new(
                crate::DeclarationKind::Lexeme,
                "VerbLexeme",
            )]
        );
        assert_eq!(
            selected[3].origins,
            [crate::DeclarationKey::new(
                crate::DeclarationKind::Lexeme,
                "NounLexeme",
            )]
        );
        assert_eq!(
            parse_item(selected[1]),
            syn::parse_quote! {
                fn surface_for_verb_lexeme(
                    lexeme: VerbLexeme,
                    agreement: Agreement,
                ) -> &'static str {
                    match (lexeme, agreement) {
                        (VerbLexeme::InventedLemma, Agreement::Bare) => "deal",
                        (VerbLexeme::InventedLemma, Agreement::ThirdPersonSingular) => "deals",
                        (VerbLexeme::Be, Agreement::Bare) => "are",
                        (VerbLexeme::Be, Agreement::ThirdPersonSingular) => "is",
                    }
                }
            }
        );
        assert_eq!(
            parse_item(selected[3]),
            syn::parse_quote! {
                fn surface_for_noun_lexeme(
                    lexeme: NounLexeme,
                    number: Number,
                ) -> &'static str {
                    match (lexeme, number) {
                        (NounLexeme::TwoWords, Number::Singular) => "object",
                        (NounLexeme::TwoWords, Number::Plural) => "objects",
                    }
                }
            }
        );

        let source = expansion
            .items()
            .iter()
            .filter(|item| {
                selected.iter().any(|selected| selected.key == item.key)
                    || matches!(
                        &item.key,
                        ItemKey::Impl { self_ty, trait_name: None } if self_ty == "Sentence"
                    )
            })
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !source.contains("inflect"),
            "legacy inflection call survived: {source}"
        );
        assert!(
            !source.contains("\"invented_lemma\""),
            "member spelling became a lemma: {source}"
        );
        assert!(
            !source.contains("\"two_words\""),
            "member spelling became a lemma: {source}"
        );
    }

    fn parse_item(item: &crate::GeneratedItem) -> syn::Item {
        syn::parse2::<syn::File>(item.tokens.clone())
            .expect("generated item reparses")
            .items
            .into_iter()
            .next()
            .expect("generated item contains one item")
    }
    fn assert_enum(expansion: &crate::Expansion, name: &str, variants: &[&str], ordered: bool) {
        let item = expansion
            .items()
            .iter()
            .find(|item| matches!(&item.key, ItemKey::Named { name: found, .. } if found == name))
            .unwrap_or_else(|| panic!("terminal enum `{name}` exists"));
        let file = syn::parse2::<syn::File>(item.tokens.clone()).expect("terminal item reparses");
        let Item::Enum(item) = &file.items[0] else {
            panic!("terminal output is an enum");
        };
        assert_eq!(
            item.variants
                .iter()
                .map(|variant| variant.ident.to_string())
                .collect::<Vec<_>>(),
            variants
        );
        let derive = item
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("derive"))
            .expect("derive attribute");
        let syn::Meta::List(list) = &derive.meta else {
            panic!("derive is a list");
        };
        let paths = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
            .parse2(list.tokens.clone())
            .expect("derive paths parse");
        let mut expected = vec!["Debug", "Clone", "Copy", "PartialEq", "Eq"];
        if ordered {
            expected.extend(["Ord", "PartialOrd"]);
        }
        assert_eq!(
            paths
                .iter()
                .map(|path| path
                    .segments
                    .last()
                    .expect("derive path has a segment")
                    .ident
                    .to_string())
                .collect::<Vec<_>>(),
            expected
        );
    }
}
