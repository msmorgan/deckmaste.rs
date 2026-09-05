use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::emitted_ident;
use crate::identifier::feature_helper;
use crate::identifier::key as identifier_key;
use crate::identifier::lexeme_surface_helper;
use crate::identifier::snake_case;
use crate::plan::DeclarationKey;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::plan::SourceDeclarationKind;
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
                let origin = DeclarationKey::new(SourceDeclarationKind::Vocab, &name);
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
                    items.push(emit_vocab_modifier_license_helper(row, origin.clone()));
                }
                if row
                    .feature_members(crate::feature::Feature::DeterminerNumber)
                    .is_some()
                {
                    items.push(emit_vocab_determiner_number_helper(row, origin.clone()));
                }
                if row
                    .feature_members(crate::feature::Feature::NominalLicense)
                    .is_some()
                {
                    items.push(emit_vocab_nominal_license_helper(row, origin.clone()));
                }
                if row
                    .feature_members(crate::feature::Feature::BareDurationLicense)
                    .is_some()
                {
                    items.push(emit_vocab_bare_duration_license_helper(row, origin.clone()));
                }
                if row
                    .feature_members(crate::feature::Feature::BareLocativeComplement)
                    .is_some()
                {
                    items.push(emit_vocab_bare_locative_complement_helper(
                        row,
                        origin.clone(),
                    ));
                }
                if row
                    .feature_members(crate::feature::Feature::PrepositionComplementKind)
                    .is_some()
                {
                    items.push(emit_vocab_preposition_complement_kind_helper(
                        row,
                        origin.clone(),
                    ));
                    items.push(emit_vocab_verb_frame_role_preposition_helper(
                        row,
                        origin.clone(),
                    ));
                }
                if row
                    .feature_members(crate::feature::Feature::PrepositionAttachment)
                    .is_some()
                {
                    items.push(emit_vocab_preposition_attachment_helper(
                        row,
                        origin.clone(),
                    ));
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
                let origin = DeclarationKey::new(SourceDeclarationKind::Lexeme, &name);
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
                    .feature_members(crate::feature::Feature::BareLocativeLicense)
                    .is_some()
                {
                    items.push(emit_lexeme_bare_locative_license_helper(
                        row,
                        origin.clone(),
                    ));
                }
                if row
                    .feature_members(crate::feature::Feature::Compoundability)
                    .is_some()
                {
                    items.push(emit_lexeme_compoundability_helper(row, origin.clone()));
                }
                if row
                    .feature_members(crate::feature::Feature::Countability)
                    .is_some()
                {
                    items.push(emit_lexeme_countability_helper(row, origin.clone()));
                }
                if row
                    .feature_members(crate::feature::Feature::MannerAnaphorClass)
                    .is_some()
                {
                    items.push(emit_lexeme_manner_anaphor_class_helper(row, origin.clone()));
                }
                if row
                    .feature_members(crate::feature::Feature::ModifierLicense)
                    .is_some()
                {
                    items.push(emit_lexeme_modifier_license_helper(row, origin.clone()));
                }
                if row
                    .feature_members(crate::feature::Feature::LocativeTemporalLicense)
                    .is_some()
                {
                    items.push(emit_lexeme_locative_temporal_license_helper(
                        row,
                        origin.clone(),
                    ));
                }
                if row
                    .feature_members(crate::feature::Feature::Properness)
                    .is_some()
                {
                    items.push(emit_lexeme_properness_helper(row, origin.clone()));
                }
                if row
                    .feature_members(crate::feature::Feature::Relationality)
                    .is_some()
                {
                    items.push(emit_lexeme_relationality_helper(row, origin.clone()));
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
                    crate::feature::Feature::ConcordClass | crate::feature::Feature::Participle
                ));
                let origin = row.origin().clone();
                let verb = row.codec_ident();
                let declaration = row.declaration_value_ident();
                let frame_complement_pair = row.frame_key().atoms()
                    == [crate::semantic::VerbFrameAtom::FrameComplementPair];
                let frame_class = match row.frame_key().class() {
                    crate::semantic::VerbFrameClass::Predicate => {
                        quote! { VerbFrameClass::Predicate }
                    }
                    crate::semantic::VerbFrameClass::Auxiliary => {
                        quote! { VerbFrameClass::Auxiliary }
                    }
                    crate::semantic::VerbFrameClass::ProVerb => {
                        quote! { VerbFrameClass::ProVerb }
                    }
                };
                let frame_atoms = row
                    .frame_key()
                    .atoms()
                    .iter()
                    .map(|atom| match atom {
                        crate::semantic::VerbFrameAtom::Literal(literal) => {
                            let literal = syn::LitStr::new(literal, Span::call_site());
                            quote! { VerbFrameAtom::Literal(#literal) }
                        }
                        crate::semantic::VerbFrameAtom::Lex(terminal, variant) => {
                            let terminal = syn::LitStr::new(terminal, Span::call_site());
                            let variant = syn::LitStr::new(variant, Span::call_site());
                            quote! { VerbFrameAtom::Lex(#terminal, #variant) }
                        }
                        crate::semantic::VerbFrameAtom::OptionalLex(terminal, variant) => {
                            let terminal = syn::LitStr::new(terminal, Span::call_site());
                            let variant = syn::LitStr::new(variant, Span::call_site());
                            quote! { VerbFrameAtom::OptionalLex(#terminal, #variant) }
                        }
                        crate::semantic::VerbFrameAtom::MarkedRole(terminal, variant, role) => {
                            let terminal = syn::LitStr::new(terminal, Span::call_site());
                            let variant = syn::LitStr::new(variant, Span::call_site());
                            let role = syn::LitStr::new(role, Span::call_site());
                            quote! { VerbFrameAtom::MarkedRole(#terminal, #variant, #role) }
                        }
                        crate::semantic::VerbFrameAtom::OptionalMarkedRole(
                            terminal,
                            variant,
                            role,
                        ) => {
                            let terminal = syn::LitStr::new(terminal, Span::call_site());
                            let variant = syn::LitStr::new(variant, Span::call_site());
                            let role = syn::LitStr::new(role, Span::call_site());
                            quote! { VerbFrameAtom::OptionalMarkedRole(#terminal, #variant, #role) }
                        }
                        crate::semantic::VerbFrameAtom::Amount => quote! { VerbFrameAtom::Amount },
                        crate::semantic::VerbFrameAtom::ObjectNounPhrase => {
                            quote! { VerbFrameAtom::ObjectNounPhrase }
                        }
                        crate::semantic::VerbFrameAtom::PredicativeComplement => {
                            quote! { VerbFrameAtom::PredicativeComplement }
                        }
                        crate::semantic::VerbFrameAtom::FrameComplementPair => {
                            quote! { VerbFrameAtom::FrameComplementPair }
                        }
                        crate::semantic::VerbFrameAtom::Role(role) => {
                            let role = syn::LitStr::new(role, Span::call_site());
                            quote! { VerbFrameAtom::Role(#role) }
                        }
                        crate::semantic::VerbFrameAtom::OptionalRole(role) => {
                            let role = syn::LitStr::new(role, Span::call_site());
                            quote! { VerbFrameAtom::OptionalRole(#role) }
                        }
                    })
                    .collect::<Vec<_>>();
                let pair_field = frame_complement_pair.then(|| {
                    quote! { frame_complement_pair_role: usize, }
                });
                let pair_parameter = frame_complement_pair.then(|| {
                    quote! { frame_complement_pair_role: usize, }
                });
                let pair_initializer = frame_complement_pair.then(|| {
                    quote! { frame_complement_pair_role, }
                });
                let pair_debug = frame_complement_pair.then(|| {
                    quote! { .field("frame_complement_pair_role", &self.frame_complement_pair_role) }
                });
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.declaration_value_ident().to_string()),
                    quote! {
                        #[derive(Clone, PartialEq, Eq)]
                        pub struct #declaration {
                            reference: Box<crate::environment::VerbInventoryRef>,
                            inflectional_forms: ::deckmaste_construction_core::macro_def::InflectionalFormSet,
                            #pair_field
                        }
                    },
                    vec![origin.clone()],
                ));
                items.push(GeneratedItem::new(
                    ItemKey::Impl {
                        trait_name: Some("Debug".to_owned()),
                        self_ty: row.declaration_value_ident().to_string(),
                    },
                    quote! {
                        impl std::fmt::Debug for #declaration {
                            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                formatter
                                    .debug_struct(stringify!(#declaration))
                                    .field("reference", self.reference())
                                    #pair_debug
                                    .finish()
                            }
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
                                reference: crate::environment::VerbInventoryRef,
                                #pair_parameter
                            ) -> Option<Self> {
                                let frame = VerbFrameKey::with_class(
                                    #frame_class,
                                    &[#(#frame_atoms),*],
                                );
                                if !environment.verb_frame_licenses(&reference, frame) {
                                    return None;
                                }
                                [
                                    ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN,
                                    ::deckmaste_construction_core::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                                ]
                                .into_iter()
                                .all(|feature| environment.verb_inventory_surface(&reference, feature).is_some())
                                .then_some(Self {
                                    reference: Box::new(reference),
                                    inflectional_forms: ::deckmaste_construction_core::macro_def::InflectionalFormSet::EMPTY,
                                    #pair_initializer
                                })
                            }

                            pub(crate) fn from_inflectional_forms(
                                environment: &crate::environment::ParserEnvironment,
                                reference: crate::environment::VerbInventoryRef,
                                inflectional_forms: ::deckmaste_construction_core::macro_def::InflectionalFormSet,
                                #pair_parameter
                            ) -> Option<Self> {
                                let frame = VerbFrameKey::with_class(
                                    #frame_class,
                                    &[#(#frame_atoms),*],
                                );
                                let has_present_form = inflectional_forms.contains(
                                    ::deckmaste_construction_core::macro_def::InflectionalForm::Plain,
                                ) || inflectional_forms.contains(
                                    ::deckmaste_construction_core::macro_def::InflectionalForm::ThirdPersonSingularPresent,
                                );
                                let has_present_paradigm = !has_present_form || [
                                        ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN,
                                        ::deckmaste_construction_core::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                                    ]
                                    .into_iter()
                                    .all(|feature| environment.verb_inventory_surface(&reference, feature).is_some());
                                let surfaces = inflectional_forms
                                    .iter()
                                    .filter(|form| matches!(
                                        form,
                                        ::deckmaste_construction_core::macro_def::InflectionalForm::Plain
                                            | ::deckmaste_construction_core::macro_def::InflectionalForm::ThirdPersonSingularPresent
                                            | ::deckmaste_construction_core::macro_def::InflectionalForm::Preterite
                                    ))
                                    .map(|form| environment.verb_inventory_surface(
                                        &reference,
                                        ::deckmaste_construction_core::macro_def::SurfaceFeature::Inflectional(form),
                                    ))
                                    .collect::<Option<Vec<_>>>();
                                let has_one_homographic_surface = surfaces.is_some_and(|surfaces| {
                                    !surfaces.is_empty()
                                        && surfaces.windows(2).all(|pair| pair[0] == pair[1])
                                });
                                let retained_inflectional_forms = if inflectional_forms.contains(
                                    ::deckmaste_construction_core::macro_def::InflectionalForm::Preterite,
                                ) {
                                    inflectional_forms
                                } else {
                                    ::deckmaste_construction_core::macro_def::InflectionalFormSet::EMPTY
                                };
                                (environment.verb_frame_licenses(&reference, frame)
                                    && has_present_paradigm
                                    && has_one_homographic_surface)
                                .then_some(Self {
                                    reference: Box::new(reference),
                                    inflectional_forms: retained_inflectional_forms,
                                    #pair_initializer
                                })
                            }

                            pub fn reference(&self) -> &crate::environment::VerbInventoryRef {
                                &self.reference
                            }

                            pub(crate) const fn inflectional_forms(
                                &self,
                            ) -> ::deckmaste_construction_core::macro_def::InflectionalFormSet {
                                self.inflectional_forms
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
                    vec![origin.clone()],
                ));
                items.extend(emit_declaration_verb_role_prepositions_helper(
                    validated, row, origin,
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
                    vec![origin.clone()],
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
                        quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::Type }
                    }
                    crate::semantic::DeclarationKindFamily::TurnPart => {
                        quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::TurnPart }
                    }
                    crate::semantic::DeclarationKindFamily::Subtype => {
                        quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::Subtype(_) }
                    }
                    crate::semantic::DeclarationKindFamily::SubtypeFamily(family) => {
                        let family = crate::emit::subtype_category(*family);
                        quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::Subtype(#family) }
                    }
                });
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.declaration_value_ident().to_string()),
                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct #declaration {
                            id: ::deckmaste_construction_core::macro_def::DeclarationIdentity,
                            locative_temporal_license: ::deckmaste_construction_core::macro_def::NounLocativeTemporalLicense,
                            relationality: ::deckmaste_construction_core::macro_def::NounRelationality,
                            number_invariant: bool,
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
                                id: ::deckmaste_construction_core::macro_def::DeclarationIdentity,
                            ) -> Option<Self> {
                                let has_surface = environment
                                    .surface(&id, ::deckmaste_construction_core::macro_def::SurfaceFeature::Singular)
                                    .or_else(|| environment.surface(
                                        &id,
                                        ::deckmaste_construction_core::macro_def::SurfaceFeature::Plural,
                                    ))
                                    .is_some();
                                let (locative_temporal_license, relationality, number_invariant) =
                                    environment.declaration_noun_features(&id)?;
                                has_surface.then(|| Self::from_reading(
                                    id,
                                    locative_temporal_license,
                                    relationality,
                                    number_invariant,
                                )).flatten()
                            }

                            pub(crate) fn from_reading(
                                id: ::deckmaste_construction_core::macro_def::DeclarationIdentity,
                                locative_temporal_license: ::deckmaste_construction_core::macro_def::NounLocativeTemporalLicense,
                                relationality: ::deckmaste_construction_core::macro_def::NounRelationality,
                                number_invariant: bool,
                            ) -> Option<Self> {
                                matches!(id.kind(), #(#allowed)|*)
                                    .then_some(Self {
                                        id,
                                        locative_temporal_license,
                                        relationality,
                                        number_invariant,
                                    })
                            }

                            pub fn id(&self) -> &::deckmaste_construction_core::macro_def::DeclarationIdentity {
                                &self.id
                            }

                            pub(crate) fn locative_temporal_license(
                                &self,
                            ) -> ::deckmaste_construction_core::macro_def::NounLocativeTemporalLicense {
                                self.locative_temporal_license
                            }

                            pub(crate) fn relationality(
                                &self,
                            ) -> ::deckmaste_construction_core::macro_def::NounRelationality {
                                self.relationality
                            }

                            pub(crate) fn number_invariant(&self) -> bool {
                                self.number_invariant
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
                    vec![origin.clone()],
                ));
                if let Some(closed) = row.closed_lexeme() {
                    let closed = validated
                        .lexeme(&closed.to_string())
                        .expect("validated aggregate noun closed branch names a lexeme");
                    items.extend(emit_aggregate_noun_feature_helpers(row, closed));
                }
            }
            TerminalPlan::DeclarationDeterminative(row) => {
                let origin = row.origin().clone();
                let ty = row.codec_ident();
                let lemma = row.lemma_ident();
                let variants = row
                    .closed()
                    .iter()
                    .map(crate::semantic::ClosedDeterminativePlan::lemma);
                let closed_variant = quote! { Closed(#lemma), };
                items.push(GeneratedItem::new(ItemKey::named_type(lemma.to_string()), quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum #lemma { #(#variants),* }
                }, vec![origin.clone()]));
                items.push(GeneratedItem::new(
                    ItemKey::named_type(ty.to_string()),
                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)] pub enum #ty { #closed_variant }
                    },
                    vec![origin.clone()],
                ));
                items.push(emit_declaration_determinative_fused_head_license_helper(
                    row,
                ));
                items.push(emit_declaration_determinative_quantification_helper(row));
                items.push(emit_declaration_determinative_bare_duration_license_helper(
                    row,
                ));
            }
            TerminalPlan::DeclarationTerm(row) => {
                let origin = row.origin().clone();
                let term = row.codec_ident();
                let allowed = row
                    .kinds()
                    .iter()
                    .map(|kind| crate::emit::declaration_kind(*kind));
                let position = crate::emit::grammar_position(row.position());
                let feature = crate::emit::surface_feature(row.feature());
                let parameter_prepositions =
                    fixed_keyword_parameter_preposition_match_arms(validated);
                items.push(GeneratedItem::new(
                    ItemKey::named_type(row.codec_name()),
                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct #term {
                            id: ::deckmaste_construction_core::macro_def::DeclarationIdentity,
                            parameter: Option<(u16, Option<::deckmaste_construction_core::macro_def::FixedKeywordNominalNumber>)>,
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
                                id: ::deckmaste_construction_core::macro_def::DeclarationIdentity,
                            ) -> Option<Self> {
                                let recipe = environment.grammar_recipe(&id)?;
                                let parameter = match recipe {
                                    ::deckmaste_construction_core::macro_def::GrammarRecipe::FixedKeyword { parameter } => parameter.clone(),
                                    _ => None,
                                };
                                (recipe.position() == #position)
                                    .then_some(())
                                    .and_then(|_| environment.surface(
                                        &id,
                                        #feature,
                                    ))
                                    .and_then(|_| {
                                        let parameter = parameter.as_ref().map(|parameter| match parameter {
                                            ::deckmaste_construction_core::macro_def::FixedKeywordParameterGrammar::Quality {
                                                preposition: ::deckmaste_construction_core::macro_def::FixedLexeme(terminal, member),
                                                nominal_number,
                                            } => (
                                                match (terminal.as_str(), member.as_str()) {
                                                    #(#parameter_prepositions)*
                                                    _ => u16::MAX,
                                                },
                                                *nominal_number,
                                            ),
                                        });
                                        Self::from_reading(id, parameter)
                                    })
                            }

                            pub(crate) fn from_reading(
                                id: ::deckmaste_construction_core::macro_def::DeclarationIdentity,
                                parameter: Option<(u16, Option<::deckmaste_construction_core::macro_def::FixedKeywordNominalNumber>)>,
                            ) -> Option<Self> {
                                matches!(id.kind(), #(#allowed)|*)
                                    .then_some(Self { id, parameter })
                            }

                            pub fn id(&self) -> &::deckmaste_construction_core::macro_def::DeclarationIdentity {
                                &self.id
                            }

                            pub(crate) fn parameter(
                                &self,
                            ) -> Option<(u16, Option<::deckmaste_construction_core::macro_def::FixedKeywordNominalNumber>)> {
                                self.parameter
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
    if validated.runtime_declaration_verbs().any(|(_, verb)| {
        verb.frame_key().atoms() == [crate::semantic::VerbFrameAtom::FrameComplementPair]
    }) {
        items.extend(emit_frame_complement_pair_role_resolver(validated));
    }
    Ok((items, contributions))
}

fn emit_declaration_determinative_fused_head_license_helper(
    determinative: &crate::semantic::DeclarationDeterminativePlan,
) -> GeneratedItem {
    let function_name = feature_helper("fused_head_license", determinative.codec_name());
    let function = emitted_ident(&function_name, determinative.codec_ident().span());
    let ty = determinative.codec_ident();
    let lemma = determinative.lemma_ident();
    let members = determinative.closed().iter().map(|member| {
        let member_name = member.lemma();
        let value = match member.fused_head_license() {
            crate::macro_def::DeterminativeFusedHeadLicense::NominalOnly => {
                quote! { FusedHeadLicense::NominalOnly }
            }
            crate::macro_def::DeterminativeFusedHeadLicense::PartitiveOnly => {
                quote! { FusedHeadLicense::PartitiveOnly }
            }
            crate::macro_def::DeterminativeFusedHeadLicense::FusedHead => {
                quote! { FusedHeadLicense::FusedHead }
            }
            crate::macro_def::DeterminativeFusedHeadLicense::PluralPredeterminer => {
                quote! { FusedHeadLicense::PluralPredeterminer }
            }
        };
        quote! { #lemma::#member_name => #value }
    });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function(value: impl ::std::borrow::Borrow<#ty>) -> FusedHeadLicense {
                match ::std::borrow::Borrow::borrow(&value) {
                    #ty::Closed(lemma) => match lemma { #(#members),* },
                }
            }
        },
        vec![determinative.origin().clone()],
    )
}

fn emit_declaration_determinative_quantification_helper(
    determinative: &crate::semantic::DeclarationDeterminativePlan,
) -> GeneratedItem {
    let function_name = feature_helper("quantification", determinative.codec_name());
    let function = emitted_ident(&function_name, determinative.codec_ident().span());
    let ty = determinative.codec_ident();
    let lemma = determinative.lemma_ident();
    let members = determinative.closed().iter().map(|member| {
        let member_name = member.lemma();
        let value = match member.quantification() {
            crate::macro_def::DeterminativeQuantification::NonDistributive => {
                quote! { Quantification::NonDistributive }
            }
            crate::macro_def::DeterminativeQuantification::Distributive => {
                quote! { Quantification::Distributive }
            }
        };
        quote! { #lemma::#member_name => #value }
    });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function(value: impl ::std::borrow::Borrow<#ty>) -> Quantification {
                match ::std::borrow::Borrow::borrow(&value) {
                    #ty::Closed(lemma) => match lemma { #(#members),* },
                }
            }
        },
        vec![determinative.origin().clone()],
    )
}

fn emit_declaration_determinative_bare_duration_license_helper(
    determinative: &crate::semantic::DeclarationDeterminativePlan,
) -> GeneratedItem {
    let function_name = feature_helper("bare_duration_license", determinative.codec_name());
    let function = emitted_ident(&function_name, determinative.codec_ident().span());
    let ty = determinative.codec_ident();
    let lemma = determinative.lemma_ident();
    let members = determinative.closed().iter().map(|member| {
        let member_name = member.lemma();
        let value = match member.bare_duration_license() {
            crate::macro_def::DeterminativeBareDurationLicense::BareDurationLicensed => {
                quote! { BareDurationLicense::BareDurationLicensed }
            }
            crate::macro_def::DeterminativeBareDurationLicense::MarkerRequired => {
                quote! { BareDurationLicense::MarkerRequired }
            }
        };
        quote! { #lemma::#member_name => #value }
    });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function(value: impl ::std::borrow::Borrow<#ty>) -> BareDurationLicense {
                match ::std::borrow::Borrow::borrow(&value) {
                    #ty::Closed(lemma) => match lemma { #(#members),* },
                }
            }
        },
        vec![determinative.origin().clone()],
    )
}

fn emit_vocab_modifier_license_helper(
    vocab: &crate::semantic::VocabPlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("modifier_license", vocab.name());
    let function = emitted_ident(&function_name, vocab.name_ident().span());
    let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
    let members = vocab
        .feature_members(crate::feature::Feature::ModifierLicense)
        .expect("requested sealed modifier-license metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, vocab.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::Unrestricted => {
                    quote! { ModifierLicense::Unrestricted }
                }
                crate::feature::FeatureValue::LocalDeterminer => {
                    quote! { ModifierLicense::LocalDeterminer }
                }
                _ => unreachable!("sealed modifier license has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> ModifierLicense { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_vocab_determiner_number_helper(
    vocab: &crate::semantic::VocabPlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("determiner_number", vocab.name());
    let function = emitted_ident(&function_name, vocab.name_ident().span());
    let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
    let members = vocab
        .feature_members(crate::feature::Feature::DeterminerNumber)
        .expect("requested sealed determiner-number metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, vocab.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::SingularOnly => {
                    quote! { DeterminerNumber::SingularOnly }
                }
                crate::feature::FeatureValue::PluralOnly => {
                    quote! { DeterminerNumber::PluralOnly }
                }
                crate::feature::FeatureValue::Both => quote! { DeterminerNumber::Both },
                _ => unreachable!("sealed determiner number has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> DeterminerNumber { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_vocab_nominal_license_helper(
    vocab: &crate::semantic::VocabPlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("nominal_license", vocab.name());
    let function = emitted_ident(&function_name, vocab.name_ident().span());
    let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
    let members = vocab
        .feature_members(crate::feature::Feature::NominalLicense)
        .expect("requested sealed nominal-license metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, vocab.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::AnyNominal => {
                    quote! { NominalLicense::AnyNominal }
                }
                crate::feature::FeatureValue::CountNominal => {
                    quote! { NominalLicense::CountNominal }
                }
                crate::feature::FeatureValue::LicensedBareSingularNoun => {
                    quote! { NominalLicense::BareSingularNoun }
                }
                crate::feature::FeatureValue::LicensedMassOrPluralCount => {
                    quote! { NominalLicense::MassOrPluralCount }
                }
                _ => unreachable!("sealed nominal license has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> NominalLicense { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_vocab_bare_duration_license_helper(
    vocab: &crate::semantic::VocabPlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("bare_duration_license", vocab.name());
    let function = emitted_ident(&function_name, vocab.name_ident().span());
    let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
    let members = vocab
        .feature_members(crate::feature::Feature::BareDurationLicense)
        .expect("requested sealed bare-duration-license metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, vocab.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::BareDurationLicensed => {
                    quote! { BareDurationLicense::BareDurationLicensed }
                }
                crate::feature::FeatureValue::MarkerRequired => {
                    quote! { BareDurationLicense::MarkerRequired }
                }
                _ => unreachable!("sealed bare-duration license has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> BareDurationLicense { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_vocab_bare_locative_complement_helper(
    vocab: &crate::semantic::VocabPlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("bare_locative_complement", vocab.name());
    let function = emitted_ident(&function_name, vocab.name_ident().span());
    let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
    let members = vocab
        .feature_members(crate::feature::Feature::BareLocativeComplement)
        .expect("requested sealed bare-locative-complement metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, vocab.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::No => quote! { BareLocativeComplement::No },
                crate::feature::FeatureValue::Yes => quote! { BareLocativeComplement::Yes },
                _ => unreachable!("sealed bare locative complement has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> BareLocativeComplement { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_vocab_preposition_attachment_helper(
    vocab: &crate::semantic::VocabPlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("preposition_attachment", vocab.name());
    let function = emitted_ident(&function_name, vocab.name_ident().span());
    let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
    let members = vocab
        .feature_members(crate::feature::Feature::PrepositionAttachment)
        .expect("requested sealed preposition-attachment metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, vocab.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::AdjunctCapable => {
                    quote! { PrepositionAttachment::AdjunctCapable }
                }
                crate::feature::FeatureValue::PostmodifierOnly => {
                    quote! { PrepositionAttachment::PostmodifierOnly }
                }
                crate::feature::FeatureValue::SelectedOnly => {
                    quote! { PrepositionAttachment::SelectedOnly }
                }
                _ => unreachable!("sealed preposition attachment has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> PrepositionAttachment { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_vocab_preposition_complement_kind_helper(
    vocab: &crate::semantic::VocabPlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("preposition_complement_kind", vocab.name());
    let function = emitted_ident(&function_name, vocab.name_ident().span());
    let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
    let members = vocab
        .feature_members(crate::feature::Feature::PrepositionComplementKind)
        .expect("requested sealed locative-temporal-complement metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, vocab.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::UnrestrictedComplement => {
                    quote! { PrepositionComplementKind::UnrestrictedComplement }
                }
                crate::feature::FeatureValue::RelationalComplement => {
                    quote! { PrepositionComplementKind::RelationalComplement }
                }
                crate::feature::FeatureValue::SelectionComplement => {
                    quote! { PrepositionComplementKind::SelectionComplement }
                }
                crate::feature::FeatureValue::SourceComplement => {
                    quote! { PrepositionComplementKind::SourceComplement }
                }
                crate::feature::FeatureValue::InComplement => {
                    quote! { PrepositionComplementKind::InComplement }
                }
                crate::feature::FeatureValue::OnComplement => {
                    quote! { PrepositionComplementKind::OnComplement }
                }
                crate::feature::FeatureValue::TemporalComplement => {
                    quote! { PrepositionComplementKind::TemporalComplement }
                }
                _ => unreachable!("sealed locative-temporal complement has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> PrepositionComplementKind { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_vocab_verb_frame_role_preposition_helper(
    vocab: &crate::semantic::VocabPlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("verb_frame_role_preposition", vocab.name());
    let function = emitted_ident(&function_name, vocab.name_ident().span());
    let ty = emitted_ident(vocab.name(), vocab.name_ident().span());
    let terminal = syn::LitStr::new(vocab.name(), vocab.name_ident().span());
    let members = vocab.variants().iter().map(|variant| {
        let member_name = identifier_key(variant.name());
        let member = emitted_ident(&member_name, variant.name().span());
        let member_name = syn::LitStr::new(&member_name, variant.name().span());
        quote! {
            #ty::#member => VerbFrameRolePreposition::new(#terminal, #member_name)
        }
    });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function(value: #ty) -> VerbFrameRolePreposition {
                match value { #(#members),* }
            }
        },
        vec![origin],
    )
}

fn emit_declaration_verb_role_prepositions_helper(
    plan: &SemanticPlan,
    verb: &crate::semantic::DeclarationVerbPlan,
    origin: DeclarationKey,
) -> [GeneratedItem; 2] {
    let function_name = feature_helper("verb_frame_role_prepositions", verb.codec_name());
    let function = emitted_ident(&function_name, verb.codec_ident().span());
    let constant_name = format!("{function_name}_values").to_ascii_uppercase();
    let constant = emitted_ident(&constant_name, verb.codec_ident().span());
    let ty = verb.codec_ident();
    if verb.frame_key().atoms() == [crate::semantic::VerbFrameAtom::FrameComplementPair] {
        return [
            GeneratedItem::new(
                ItemKey::Named {
                    kind: NamedKind::Constant,
                    name: constant_name,
                },
                quote! { const #constant: &[VerbFrameRolePreposition] = &[]; },
                vec![origin.clone()],
            ),
            GeneratedItem::new(
                ItemKey::Named {
                    kind: NamedKind::Function,
                    name: function_name,
                },
                quote! {
                    fn #function(value: &#ty) -> &'static [VerbFrameRolePreposition] {
                        &FRAME_COMPLEMENT_PAIR_ROLE_PREPOSITIONS[value.frame_complement_pair_role]
                    }
                },
                vec![origin],
            ),
        ];
    }
    let mut seen = std::collections::HashSet::new();
    let prepositions = verb.frame_key().atoms().iter().filter_map(|atom| {
        let (terminal, member) = match atom {
            crate::semantic::VerbFrameAtom::Lex(terminal, member)
            | crate::semantic::VerbFrameAtom::OptionalLex(terminal, member)
            | crate::semantic::VerbFrameAtom::MarkedRole(terminal, member, _)
            | crate::semantic::VerbFrameAtom::OptionalMarkedRole(terminal, member, _) => {
                (terminal, member)
            }
            crate::semantic::VerbFrameAtom::Literal(_)
            | crate::semantic::VerbFrameAtom::Amount
            | crate::semantic::VerbFrameAtom::ObjectNounPhrase
            | crate::semantic::VerbFrameAtom::PredicativeComplement
            | crate::semantic::VerbFrameAtom::FrameComplementPair
            | crate::semantic::VerbFrameAtom::Role(_)
            | crate::semantic::VerbFrameAtom::OptionalRole(_) => return None,
        };
        if !plan.terminal_has_feature(terminal, crate::feature::Feature::PrepositionComplementKind)
            || !seen.insert((terminal.as_str(), member.as_str()))
        {
            return None;
        }
        let terminal = syn::LitStr::new(terminal, Span::call_site());
        let member = syn::LitStr::new(member, Span::call_site());
        Some(quote! { VerbFrameRolePreposition::new(#terminal, #member) })
    });
    [
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Constant,
                name: constant_name,
            },
            quote! {
            const #constant: &[VerbFrameRolePreposition] = &[#(#prepositions),*];
            },
            vec![origin.clone()],
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: function_name,
            },
            quote! {
            fn #function(_value: &#ty) -> &'static [VerbFrameRolePreposition] {
                #constant
            }
            },
            vec![origin],
        ),
    ]
}

/// Every preposition-bearing vocabulary member, paired with its owning
/// vocabulary's declaration key. Position in this sequence is the ordinal a
/// generated preposition carries, so it is the single authority every emitter
/// that indexes prepositions reads.
fn preposition_bearing_members(
    plan: &SemanticPlan,
) -> (Vec<DeclarationKey>, Vec<(syn::LitStr, syn::LitStr)>) {
    let mut origins = Vec::new();
    let mut members = Vec::new();
    for terminal in plan.terminals() {
        let TerminalPlan::Vocab(vocab) = terminal else {
            continue;
        };
        if vocab
            .feature_members(crate::feature::Feature::PrepositionComplementKind)
            .is_none()
        {
            continue;
        }
        origins.push(DeclarationKey::new(
            SourceDeclarationKind::Vocab,
            vocab.name(),
        ));
        let terminal_name = syn::LitStr::new(vocab.name(), vocab.name_ident().span());
        for variant in vocab.variants() {
            let member_name = identifier_key(variant.name());
            let member = syn::LitStr::new(&member_name, variant.name().span());
            members.push((terminal_name.clone(), member));
        }
    }
    (origins, members)
}

fn emit_frame_complement_pair_role_resolver(plan: &SemanticPlan) -> [GeneratedItem; 2] {
    let (origins, members) = preposition_bearing_members(plan);
    let mut arms = Vec::new();
    let mut values = Vec::new();
    for (terminal_name, member) in &members {
        let index = syn::Index::from(values.len());
        arms.push(quote! {
            (#terminal_name, #member) => Some(#index),
        });
        values.push(quote! {
            [VerbFrameRolePreposition::new(#terminal_name, #member)]
        });
    }
    [
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Constant,
                name: "FRAME_COMPLEMENT_PAIR_ROLE_PREPOSITIONS".to_owned(),
            },
            quote! {
                const FRAME_COMPLEMENT_PAIR_ROLE_PREPOSITIONS:
                    &[[VerbFrameRolePreposition; 1]] = &[#(#values),*];
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: "frame_complement_pair_role_from_keys".to_owned(),
            },
            quote! {
                fn frame_complement_pair_role_from_keys(
                    terminal: &str,
                    member: &str,
                ) -> Option<usize> {
                    match (terminal, member) {
                        #(#arms)*
                        _ => None,
                    }
                }
            },
            origins,
        ),
    ]
}

pub(super) fn fixed_keyword_parameter_preposition_match_arms(
    plan: &SemanticPlan,
) -> Vec<TokenStream> {
    let (_, members) = preposition_bearing_members(plan);
    members
        .iter()
        .enumerate()
        .map(|(index, (terminal_name, member))| {
            let index = u16::try_from(index)
                .expect("preposition-bearing vocabulary index must fit the scanner carrier");
            quote! { (#terminal_name, #member) => #index, }
        })
        .collect()
}

fn emit_lexeme_surface_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> syn::Result<GeneratedItem> {
    let function_name = lexeme_surface_helper(lexeme.name());
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let (feature_ty, feature_argument) = match lexeme.morphology().feature() {
        crate::Feature::ConcordClass => (quote! { ConcordClass }, quote! { concord_class }),
        crate::Feature::Number => (quote! { Number }, quote! { number }),
        crate::Feature::Participle => (quote! { Participle }, quote! { participle }),
        crate::Feature::Cardinality
        | crate::Feature::InflectionalForm
        | crate::Feature::BareDurationLicense
        | crate::Feature::BareLocativeComplement
        | crate::Feature::BareLocativeLicense
        | crate::Feature::Compoundability
        | crate::Feature::Countability
        | crate::Feature::HomographLicense
        | crate::Feature::MannerAnaphorClass
        | crate::Feature::ModifierLicense
        | crate::Feature::DeterminerNumber
        | crate::Feature::Quantification
        | crate::Feature::FusedHeadLicense
        | crate::Feature::Focus
        | crate::Feature::PrepositionComplementKind
        | crate::Feature::LocativeTemporalLicense
        | crate::Feature::NominalForm
        | crate::Feature::NominalLicense
        | crate::Feature::Onset
        | crate::Feature::PossessiveEnding
        | crate::Feature::PrepositionAttachment
        | crate::Feature::Properness
        | crate::Feature::Relationality => {
            unreachable!("derived surface features are not morphology axes")
        }
    };
    let arms = lexeme
        .surfaces()
        .iter()
        .map(|row| {
            let member = emitted_ident(row.member(), Span::call_site());
            let feature = match row.feature() {
                crate::macro_def::SurfaceFeature::PLAIN => {
                    quote! { ConcordClass::Other }
                }
                crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => {
                    quote! { ConcordClass::ThirdPersonSingular }
                }
                crate::macro_def::SurfaceFeature::Singular => {
                    quote! { Number::Singular }
                }
                crate::macro_def::SurfaceFeature::Plural => {
                    quote! { Number::Plural }
                }
                crate::macro_def::SurfaceFeature::PAST_PARTICIPLE => {
                    quote! { Participle::Participle }
                }
                crate::macro_def::SurfaceFeature::Inflectional(_) => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "sealed lexeme surface has an unsupported Inflectional Form",
                    ));
                }
                crate::macro_def::SurfaceFeature::Fixed
                | crate::macro_def::SurfaceFeature::BoundSuffix
                | crate::macro_def::SurfaceFeature::BlockLabel => {
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
    let concord_fallback = (lexeme.morphology().feature() == crate::Feature::ConcordClass)
        .then(|| {
            quote! {
                (_, ConcordClass::OtherOrThirdPersonSingular | ConcordClass::PlainOrPreterite) => {
                    unreachable!("a closed verb lexeme cannot carry an underspecified Concord Class")
                }
            }
        });
    Ok(GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! {
            fn #function(lexeme: #ty, #feature_argument: #feature_ty,) -> &'static str {
                match (lexeme, #feature_argument) { #(#arms,)* #concord_fallback }
            }
        },
        vec![origin],
    ))
}

fn emit_lexeme_bare_locative_license_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("bare_locative_license", lexeme.name());
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let members = lexeme
        .feature_members(crate::feature::Feature::BareLocativeLicense)
        .expect("requested sealed bare-locative-license metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, lexeme.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::QualifiedOnly => {
                    quote! { BareLocativeLicense::QualifiedOnly }
                }
                crate::feature::FeatureValue::BareAllowed => {
                    quote! { BareLocativeLicense::BareAllowed }
                }
                _ => unreachable!("sealed bare locative license has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> BareLocativeLicense { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_lexeme_compoundability_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("compoundability", lexeme.name());
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let members = lexeme
        .feature_members(crate::feature::Feature::Compoundability)
        .expect("requested sealed compoundability metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, lexeme.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::Compoundable => {
                    quote! { Compoundability::Compoundable }
                }
                crate::feature::FeatureValue::NonCompoundable => {
                    quote! { Compoundability::NonCompoundable }
                }
                _ => unreachable!("sealed compoundability has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> Compoundability { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_lexeme_countability_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("countability", lexeme.name());
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let members = lexeme
        .feature_members(crate::feature::Feature::Countability)
        .expect("requested sealed countability metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, lexeme.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::Count => quote! { Countability::Count },
                crate::feature::FeatureValue::Mass => quote! { Countability::Mass },
                _ => unreachable!("sealed countability has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> Countability { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_lexeme_manner_anaphor_class_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("manner_anaphor_class", lexeme.name());
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let members = lexeme
        .feature_members(crate::feature::Feature::MannerAnaphorClass)
        .expect("requested sealed manner-anaphor-class metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, lexeme.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::OtherNoun => {
                    quote! { MannerAnaphorClass::OtherNoun }
                }
                crate::feature::FeatureValue::MannerAnaphor => {
                    quote! { MannerAnaphorClass::MannerAnaphor }
                }
                _ => unreachable!("sealed manner-anaphor class has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> MannerAnaphorClass { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_lexeme_properness_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("properness", lexeme.name());
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let members = lexeme
        .feature_members(crate::feature::Feature::Properness)
        .expect("requested sealed properness metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, lexeme.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::Common => quote! { Properness::Common },
                crate::feature::FeatureValue::Proper => quote! { Properness::Proper },
                _ => unreachable!("sealed properness has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> Properness { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_lexeme_relationality_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("relationality", lexeme.name());
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let members = lexeme
        .feature_members(crate::feature::Feature::Relationality)
        .expect("requested sealed relationality metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, lexeme.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::NonRelational => {
                    quote! { Relationality::NonRelational }
                }
                crate::feature::FeatureValue::QualifiedRelational => {
                    quote! { Relationality::QualifiedRelational }
                }
                crate::feature::FeatureValue::DeterminedRelational => {
                    quote! { Relationality::DeterminedRelational }
                }
                crate::feature::FeatureValue::Relational => {
                    quote! { Relationality::Relational }
                }
                _ => unreachable!("sealed relationality has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> Relationality { match value { #(#members),* } } },
        vec![origin],
    )
}

fn emit_lexeme_locative_temporal_license_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("locative_temporal_license", lexeme.name());
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let members = lexeme
        .feature_members(crate::feature::Feature::LocativeTemporalLicense)
        .expect("requested sealed locative-temporal-license metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, lexeme.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::Unlicensed => {
                    quote! { LocativeTemporalLicense::Unlicensed }
                }
                crate::feature::FeatureValue::OfLicensed => {
                    quote! { LocativeTemporalLicense::OfLicensed }
                }
                crate::feature::FeatureValue::OfAndOnLicensed => {
                    quote! { LocativeTemporalLicense::OfAndOnLicensed }
                }
                crate::feature::FeatureValue::OfInAndOnLicensed => {
                    quote! { LocativeTemporalLicense::OfInAndOnLicensed }
                }
                crate::feature::FeatureValue::InLicensed => {
                    quote! { LocativeTemporalLicense::InLicensed }
                }
                crate::feature::FeatureValue::OnLicensed => {
                    quote! { LocativeTemporalLicense::OnLicensed }
                }
                crate::feature::FeatureValue::InOrOnEdgeLicensed => {
                    quote! { LocativeTemporalLicense::InOrOnEdgeLicensed }
                }
                crate::feature::FeatureValue::ObjectAttachmentLicensed => {
                    quote! { LocativeTemporalLicense::ObjectAttachmentLicensed }
                }
                crate::feature::FeatureValue::TemporalLicensed => {
                    quote! { LocativeTemporalLicense::TemporalLicensed }
                }
                crate::feature::FeatureValue::OfAndTemporalLicensed => {
                    quote! { LocativeTemporalLicense::OfAndTemporalLicensed }
                }
                _ => unreachable!("sealed locative-temporal license has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> LocativeTemporalLicense { match value { #(#members),* } } },
        vec![origin],
    )
}

#[allow(
    clippy::too_many_lines,
    reason = "one arm per sealed noun classification keeps the aggregate literal"
)]
fn emit_aggregate_noun_feature_helpers(
    noun: &crate::semantic::DeclarationNounPlan,
    closed: &crate::semantic::LexemePlan,
) -> Vec<GeneratedItem> {
    let ty = noun.codec_ident();
    let origin = noun.origin().clone();
    let closed_ident = closed.name_ident();
    let bare_locative_license_name = feature_helper("bare_locative_license", noun.codec_name());
    let bare_locative_license = emitted_ident(&bare_locative_license_name, ty.span());
    let closed_bare_locative_license = emitted_ident(
        &feature_helper("bare_locative_license", closed.name()),
        closed_ident.span(),
    );
    let compoundability_name = feature_helper("compoundability", noun.codec_name());
    let compoundability = emitted_ident(&compoundability_name, ty.span());
    let closed_compoundability = emitted_ident(
        &feature_helper("compoundability", closed.name()),
        closed_ident.span(),
    );
    let countability_name = feature_helper("countability", noun.codec_name());
    let countability = emitted_ident(&countability_name, ty.span());
    let closed_countability = emitted_ident(
        &feature_helper("countability", closed.name()),
        closed_ident.span(),
    );
    let manner_anaphor_class_name = feature_helper("manner_anaphor_class", noun.codec_name());
    let manner_anaphor_class = emitted_ident(&manner_anaphor_class_name, ty.span());
    let closed_manner_anaphor_class = emitted_ident(
        &feature_helper("manner_anaphor_class", closed.name()),
        closed_ident.span(),
    );
    let properness_name = feature_helper("properness", noun.codec_name());
    let properness = emitted_ident(&properness_name, ty.span());
    let closed_properness = emitted_ident(
        &feature_helper("properness", closed.name()),
        closed_ident.span(),
    );
    let locative_temporal_license_name =
        feature_helper("locative_temporal_license", noun.codec_name());
    let locative_temporal_license = emitted_ident(&locative_temporal_license_name, ty.span());
    let closed_locative_temporal_license = emitted_ident(
        &feature_helper("locative_temporal_license", closed.name()),
        closed_ident.span(),
    );
    let relationality_name = feature_helper("relationality", noun.codec_name());
    let relationality = emitted_ident(&relationality_name, ty.span());
    let closed_relationality = emitted_ident(
        &feature_helper("relationality", closed.name()),
        closed_ident.span(),
    );
    let mut items = Vec::new();
    if closed
        .feature_members(crate::feature::Feature::BareLocativeLicense)
        .is_some()
    {
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: bare_locative_license_name,
            },
            quote! {
                fn #bare_locative_license(
                    value: impl ::std::borrow::Borrow<#ty>,
                ) -> BareLocativeLicense {
                    match ::std::borrow::Borrow::borrow(&value) {
                        #ty::Lexeme(value) => #closed_bare_locative_license(*value),
                        #ty::Declaration(_) => BareLocativeLicense::QualifiedOnly,
                    }
                }
            },
            vec![origin.clone()],
        ));
    }
    if closed
        .feature_members(crate::feature::Feature::Compoundability)
        .is_some()
    {
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: compoundability_name,
            },
            quote! {
                fn #compoundability(value: impl ::std::borrow::Borrow<#ty>) -> Compoundability {
                    match ::std::borrow::Borrow::borrow(&value) {
                        #ty::Lexeme(value) => #closed_compoundability(*value),
                        #ty::Declaration(_) => Compoundability::Compoundable,
                    }
                }
            },
            vec![origin.clone()],
        ));
    }
    if closed
        .feature_members(crate::feature::Feature::Countability)
        .is_some()
    {
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: countability_name,
            },
            quote! {
                fn #countability(value: impl ::std::borrow::Borrow<#ty>) -> Countability {
                    match ::std::borrow::Borrow::borrow(&value) {
                        #ty::Lexeme(value) => #closed_countability(*value),
                        #ty::Declaration(_) => Countability::Count,
                    }
                }
            },
            vec![origin.clone()],
        ));
    }
    if closed
        .feature_members(crate::feature::Feature::MannerAnaphorClass)
        .is_some()
    {
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: manner_anaphor_class_name,
            },
            quote! {
                fn #manner_anaphor_class(
                    value: impl ::std::borrow::Borrow<#ty>,
                ) -> MannerAnaphorClass {
                    match ::std::borrow::Borrow::borrow(&value) {
                        #ty::Lexeme(value) => #closed_manner_anaphor_class(*value),
                        #ty::Declaration(_) => MannerAnaphorClass::OtherNoun,
                    }
                }
            },
            vec![origin.clone()],
        ));
    }
    if closed
        .feature_members(crate::feature::Feature::Properness)
        .is_some()
    {
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: properness_name,
            },
            quote! {
                fn #properness(value: impl ::std::borrow::Borrow<#ty>) -> Properness {
                    match ::std::borrow::Borrow::borrow(&value) {
                        #ty::Lexeme(value) => #closed_properness(*value),
                        #ty::Declaration(value) => match value.id().kind() {
                            ::deckmaste_construction_core::macro_def::DeclarationKind::Subtype(_) => Properness::Proper,
                            _ => Properness::Common,
                        },
                    }
                }
            },
            vec![origin.clone()],
        ));
    }
    if closed
        .feature_members(crate::feature::Feature::LocativeTemporalLicense)
        .is_some()
    {
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: locative_temporal_license_name,
            },
            quote! {
                fn #locative_temporal_license(
                    value: impl ::std::borrow::Borrow<#ty>,
                ) -> LocativeTemporalLicense {
                    match ::std::borrow::Borrow::borrow(&value) {
                        #ty::Lexeme(value) => #closed_locative_temporal_license(*value),
                        #ty::Declaration(value) => match value.locative_temporal_license() {
                            ::deckmaste_construction_core::macro_def::NounLocativeTemporalLicense::Unlicensed => LocativeTemporalLicense::Unlicensed,
                            ::deckmaste_construction_core::macro_def::NounLocativeTemporalLicense::InLicensed => LocativeTemporalLicense::InLicensed,
                            ::deckmaste_construction_core::macro_def::NounLocativeTemporalLicense::OnLicensed => LocativeTemporalLicense::OnLicensed,
                            ::deckmaste_construction_core::macro_def::NounLocativeTemporalLicense::InOrOnEdgeLicensed => LocativeTemporalLicense::InOrOnEdgeLicensed,
                            ::deckmaste_construction_core::macro_def::NounLocativeTemporalLicense::ObjectAttachmentLicensed => LocativeTemporalLicense::ObjectAttachmentLicensed,
                            ::deckmaste_construction_core::macro_def::NounLocativeTemporalLicense::TemporalLicensed => LocativeTemporalLicense::OfAndTemporalLicensed,
                        },
                    }
                }
            },
            vec![origin.clone()],
        ));
    }
    if closed
        .feature_members(crate::feature::Feature::Relationality)
        .is_some()
    {
        items.push(GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: relationality_name,
            },
            quote! {
                fn #relationality(value: impl ::std::borrow::Borrow<#ty>) -> Relationality {
                    match ::std::borrow::Borrow::borrow(&value) {
                        #ty::Lexeme(value) => #closed_relationality(*value),
                        #ty::Declaration(value) => match value.relationality() {
                            ::deckmaste_construction_core::macro_def::NounRelationality::NonRelational => Relationality::NonRelational,
                            ::deckmaste_construction_core::macro_def::NounRelationality::QualifiedRelational => Relationality::QualifiedRelational,
                            ::deckmaste_construction_core::macro_def::NounRelationality::DeterminedRelational => Relationality::DeterminedRelational,
                            ::deckmaste_construction_core::macro_def::NounRelationality::Relational => Relationality::Relational,
                        },
                    }
                }
            },
            vec![origin],
        ));
    }
    items
}

fn emit_lexeme_modifier_license_helper(
    lexeme: &crate::semantic::LexemePlan,
    origin: DeclarationKey,
) -> GeneratedItem {
    let function_name = feature_helper("modifier_license", lexeme.name());
    let function = emitted_ident(&function_name, lexeme.name_ident().span());
    let ty = emitted_ident(lexeme.name(), lexeme.name_ident().span());
    let members = lexeme
        .feature_members(crate::feature::Feature::ModifierLicense)
        .expect("requested sealed modifier-license metadata")
        .iter()
        .map(|(member, value)| {
            let member = emitted_ident(member, lexeme.name_ident().span());
            let value = match value {
                crate::feature::FeatureValue::Unrestricted => {
                    quote! { ModifierLicense::Unrestricted }
                }
                crate::feature::FeatureValue::LocalDeterminer => {
                    quote! { ModifierLicense::LocalDeterminer }
                }
                _ => unreachable!("sealed modifier license has its closed domain"),
            };
            quote! { #ty::#member => #value }
        });
    GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: function_name,
        },
        quote! { fn #function(value: #ty) -> ModifierLicense { match value { #(#members),* } } },
        vec![origin],
    )
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
                crate::SourceDeclarationKind::Lexeme,
                "VerbLexeme",
            )]
        );
        assert_eq!(
            selected[3].origins,
            [crate::DeclarationKey::new(
                crate::SourceDeclarationKind::Lexeme,
                "NounLexeme",
            )]
        );
        assert_eq!(
            parse_item(selected[1]),
            syn::parse_quote! {
                fn surface_for_verb_lexeme(
                    lexeme: VerbLexeme,
                    concord_class: ConcordClass,
                ) -> &'static str {
                    match (lexeme, concord_class) {
                        (VerbLexeme::InventedLemma, ConcordClass::Other) => "deal",
                        (VerbLexeme::InventedLemma, ConcordClass::ThirdPersonSingular) => "deals",
                        (VerbLexeme::Be, ConcordClass::Other) => "are",
                        (VerbLexeme::Be, ConcordClass::ThirdPersonSingular) => "is",
                        (_, ConcordClass::OtherOrThirdPersonSingular | ConcordClass::PlainOrPreterite) => {
                            unreachable!("a closed verb lexeme cannot carry an underspecified Concord Class")
                        }
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

    #[test]
    fn aggregate_noun_relationality_preserves_core_metadata_and_declaration_provenance() {
        let expansion = crate::generate(quote::quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme CoreNoun using EnglishNoun {
                feature Relationality = NonRelational;
                Player = "player",
                End = "end" { feature Relationality = Relational; },
            }
            codec Noun {
                generate declaration_noun {
                    closed = CoreNoun;
                    position = Noun;
                    kinds = [Type, Subtype, TurnPart];
                    feature = Number;
                }
            }
            construction relational: Phrase {
                element Relational { head: lex Noun, }
                require head.relationality is Relational;
                derive head.number = Values::Singular;
                derive number = head.number;
                form relational = noun(head);
            }
            root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("aggregate noun relationality generates");
        let helper = expansion
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    ItemKey::Named { name, .. } if name == "relationality_for_noun"
                )
            })
            .expect("aggregate relationality helper is emitted");

        assert_eq!(
            parse_item(helper),
            syn::parse_quote! {
                fn relationality_for_noun(
                    value: impl ::std::borrow::Borrow<Noun>
                ) -> Relationality {
                    match ::std::borrow::Borrow::borrow(&value) {
                        Noun::Lexeme(value) => relationality_for_core_noun(*value),
                        Noun::Declaration(value) => match value.relationality() {
                            ::deckmaste_construction_core::macro_def::NounRelationality::NonRelational => Relationality::NonRelational,
                            ::deckmaste_construction_core::macro_def::NounRelationality::QualifiedRelational => Relationality::QualifiedRelational,
                            ::deckmaste_construction_core::macro_def::NounRelationality::DeterminedRelational => Relationality::DeterminedRelational,
                            ::deckmaste_construction_core::macro_def::NounRelationality::Relational => Relationality::Relational,
                        },
                    }
                }
            }
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
