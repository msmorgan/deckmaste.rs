use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::BUILD_REJECTION_TYPE;
use crate::identifier::BUILD_VIOLATION_TYPE;
use crate::identifier::CARDINALITY_TYPE;
use crate::identifier::CASE_POSITION_TYPE;
use crate::identifier::CONCORD_CLASS_TYPE;
use crate::identifier::DECLARATION_CLASS_TYPE;
use crate::identifier::DECLARATION_LEAF_TYPE;
use crate::identifier::DECLARATION_MATCHER_TYPE;
use crate::identifier::DETERMINER_NUMBER_TYPE;
use crate::identifier::FEATURE_CONSTRAINT_TYPE;
use crate::identifier::FUSED_HEAD_LICENSE_TYPE;
use crate::identifier::LEAF_TYPE;
use crate::identifier::LEXICAL_OWNER_IDENTITY_TYPE;
use crate::identifier::LEXICAL_OWNER_TEMPLATE_TYPE;
use crate::identifier::LEXICAL_OWNER_TYPE;
use crate::identifier::LEXICAL_PROVENANCE_KIND_TYPE;
use crate::identifier::LEXICAL_TERMINAL_TYPE;
use crate::identifier::LEXICAL_TYPE;
use crate::identifier::NOMINAL_FORM_TYPE;
use crate::identifier::NOMINAL_LICENSE_TYPE;
use crate::identifier::NUMBER_TYPE;
use crate::identifier::ONSET_TYPE;
use crate::identifier::PARTICIPLE_TYPE;
use crate::identifier::POSSESSIVE_ENDING_TYPE;
use crate::identifier::PREFIX_POSITION_TYPE;
use crate::identifier::SCAN_POSITION_TYPE;
use crate::identifier::STRUCTURAL_TRANSITION_TYPE;
use crate::identifier::TERMINAL_CLASS_TYPE;
use crate::identifier::emitted_ident;
use crate::identifier::key as identifier_key;
use crate::identifier::path_key;
use crate::identifier::snake_case;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::semantic::AtomPlan;
use crate::semantic::BindingPlan;
use crate::semantic::CatalogIdentityPlan;
use crate::semantic::ContextIdentityPlan;
use crate::semantic::DeclarationDeterminativePlan;
use crate::semantic::DeclarationNounPlan;
use crate::semantic::DeclarationTermPlan;
use crate::semantic::DeclarationVerbPlan;
use crate::semantic::LexemePlan;
use crate::semantic::RootPlan;
use crate::semantic::SemanticPlan;
use crate::semantic::SignedDecimalPlan;
use crate::semantic::UnsignedNumberPlan;
use crate::semantic::VocabPlan;

struct RuntimeInventory<'a> {
    lexemes: Vec<&'a LexemePlan>,
    vocabs: Vec<&'a VocabPlan>,
    unused_vocab_lexicals: Vec<&'a VocabPlan>,
    noun_lexeme: Option<&'a LexemePlan>,
    noun_lexeme_is_aggregated: bool,
    verb_lexeme: Option<&'a LexemePlan>,
    noun_binding: Option<&'a BindingPlan>,
    direct_bindings: Vec<&'a BindingPlan>,
    opaque_bindings: Vec<&'a BindingPlan>,
    context_identities: Vec<&'a ContextIdentityPlan>,
    catalog_identities: Vec<(usize, &'a CatalogIdentityPlan)>,
    signed_decimal: Option<&'a SignedDecimalPlan>,
    unsigned_numbers: Vec<&'a UnsignedNumberPlan>,
    declaration_determinatives: Vec<(usize, &'a DeclarationDeterminativePlan)>,
    declaration_nouns: Vec<(usize, &'a DeclarationNounPlan)>,
    declaration_terms: Vec<(usize, &'a DeclarationTermPlan)>,
    declaration_verbs: Vec<(usize, &'a DeclarationVerbPlan)>,
}

impl<'a> RuntimeInventory<'a> {
    fn from_plan(plan: &'a SemanticPlan) -> Self {
        let vocabs = plan.runtime_vocabs().collect::<Vec<_>>();
        let unused_vocab_lexicals = vocabs
            .iter()
            .copied()
            .filter(|vocab| {
                !plan.constructions().iter().any(|construction| {
                    construction.forms().iter().any(|form| {
                        form.atoms().iter().any(|atom| {
                            matches!(
                                atom,
                                AtomPlan::Lex { terminal, .. } if terminal == vocab.name()
                            )
                        })
                    })
                })
            })
            .collect();
        Self {
            lexemes: plan
                .terminals()
                .iter()
                .filter_map(|terminal| match terminal {
                    crate::semantic::TerminalPlan::Lexeme(lexeme) => Some(lexeme),
                    _ => None,
                })
                .collect(),
            vocabs,
            unused_vocab_lexicals,
            noun_lexeme: plan.runtime_noun_lexeme(),
            noun_lexeme_is_aggregated: plan.runtime_aggregate_noun().is_some(),
            verb_lexeme: plan.runtime_verb_lexeme(),
            noun_binding: plan.runtime_noun_binding(),
            direct_bindings: plan.runtime_direct_bindings().collect(),
            opaque_bindings: plan.runtime_opaque_bindings().collect(),
            context_identities: plan.runtime_context_identities().collect(),
            catalog_identities: plan.runtime_catalog_identities().collect(),
            signed_decimal: plan.runtime_signed_decimal(),
            unsigned_numbers: plan.runtime_unsigned_numbers().collect(),
            declaration_determinatives: plan.runtime_declaration_determinatives().collect(),
            declaration_nouns: plan.runtime_declaration_nouns().collect(),
            declaration_terms: plan.runtime_declaration_terms().collect(),
            declaration_verbs: plan.runtime_declaration_verbs().collect(),
        }
    }

    fn noun_type(&self) -> Option<syn::Ident> {
        self.noun_binding
            .map(|binding| binding.value_type_name().clone())
            .or_else(|| {
                (!self.noun_lexeme_is_aggregated)
                    .then(|| self.noun_lexeme.map(|lexeme| lexeme.name_ident().clone()))
                    .flatten()
            })
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "runtime emission order mirrors the generated module's public ABI"
)]
pub(crate) fn emit(plan: &SemanticPlan) -> Vec<GeneratedItem> {
    let inventory = RuntimeInventory::from_plan(plan);
    let mut items = vec![
        named_type(
            CONCORD_CLASS_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum ConcordClass {
                    Other,
                    ThirdPersonSingular,
                    OtherOrThirdPersonSingular,
                    PlainOrPreterite,
                }
            },
        ),
        impl_item(
            None,
            CONCORD_CLASS_TYPE,
            quote! {
                impl ConcordClass {
                    const fn compatible_with(self, other: Self) -> bool {
                        matches!(
                            (self, other),
                            (Self::Other, Self::Other)
                                | (Self::ThirdPersonSingular, Self::ThirdPersonSingular)
                                | (Self::OtherOrThirdPersonSingular, _)
                                | (_, Self::OtherOrThirdPersonSingular)
                                | (Self::PlainOrPreterite, _)
                                | (_, Self::PlainOrPreterite)
                        )
                    }

                    const fn matches_required(self, required: Self) -> bool {
                        matches!(
                            (self, required),
                            (Self::Other, Self::Other)
                                | (Self::ThirdPersonSingular, Self::ThirdPersonSingular)
                                | (Self::OtherOrThirdPersonSingular, _)
                                | (Self::PlainOrPreterite, _)
                        )
                    }

                    const fn homogeneous_with(self, other: Self) -> bool {
                        matches!(
                            (self, other),
                            (Self::Other, Self::Other)
                                | (Self::ThirdPersonSingular, Self::ThirdPersonSingular)
                                | (Self::OtherOrThirdPersonSingular, _)
                                | (_, Self::OtherOrThirdPersonSingular)
                                | (Self::PlainOrPreterite, _)
                                | (Self::Other, Self::PlainOrPreterite)
                        )
                    }

                    const fn narrow_homogeneous(self, other: Self) -> Self {
                        match (self, other) {
                            (Self::OtherOrThirdPersonSingular | Self::PlainOrPreterite, value) => value,
                            (value, Self::OtherOrThirdPersonSingular | Self::PlainOrPreterite) => value,
                            _ => self,
                        }
                    }

                }
            },
        ),
        named_type(
            CARDINALITY_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Cardinality { Zero, One, TwoPlus }
            },
        ),
        named_type(
            "BareLocativeComplement",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum BareLocativeComplement { No, Yes }
            },
        ),
        named_type(
            "BareLocativeLicense",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum BareLocativeLicense { QualifiedOnly, BareAllowed }
            },
        ),
        named_type(
            "PrepositionComplementKind",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum PrepositionComplementKind {
                    UnrestrictedComplement,
                    RelationalComplement,
                    SelectionComplement,
                    SourceComplement,
                    InComplement,
                    OnComplement,
                    TemporalComplement,
                }
            },
        ),
        named_type(
            "LocativeTemporalLicense",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum LocativeTemporalLicense {
                    Unlicensed,
                    OfLicensed,
                    OfAndOnLicensed,
                    OfInAndOnLicensed,
                    InLicensed,
                    OnLicensed,
                    InOrOnEdgeLicensed,
                    ObjectAttachmentLicensed,
                    TemporalLicensed,
                    OfAndTemporalLicensed,
                }
            },
        ),
        named_type(
            "Compoundability",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Compoundability { Compoundable, NonCompoundable }
            },
        ),
        named_type(
            "Countability",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Countability { Count, Mass }
            },
        ),
        named_type(
            "HomographLicense",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum HomographLicense { Unlicensed, Licensed }
            },
        ),
        named_type(
            "ModifierLicense",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum ModifierLicense { Unrestricted, LocalDeterminer }
            },
        ),
        named_type(
            "PrepositionAttachment",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum PrepositionAttachment {
                    AdjunctCapable,
                    PostmodifierOnly,
                    SelectedOnly,
                }
            },
        ),
        named_type(
            "Properness",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Properness { Common, Proper }
            },
        ),
        named_type(
            "Relationality",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Relationality {
                    NonRelational,
                    QualifiedRelational,
                    DeterminedRelational,
                    Relational,
                    SaturatedRelational,
                }
            },
        ),
        named_type(
            DETERMINER_NUMBER_TYPE,
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)] pub(crate) enum DeterminerNumber { SingularOnly, PluralOnly, Both } },
        ),
        named_type(
            "Quantification",
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)] pub(crate) enum Quantification { NonDistributive, Distributive } },
        ),
        named_type(
            FUSED_HEAD_LICENSE_TYPE,
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)] pub(crate) enum FusedHeadLicense { NominalOnly, PartitiveOnly, FusedHead, PluralPredeterminer } },
        ),
        named_type(
            "Focus",
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)] pub(crate) enum Focus { Unfocused, Focused } },
        ),
        named_type(
            "BareDurationLicense",
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)] pub(crate) enum BareDurationLicense { BareDurationLicensed, MarkerRequired } },
        ),
        named_type(
            crate::identifier::MANNER_ANAPHOR_CLASS_TYPE,
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)] pub(crate) enum MannerAnaphorClass { OtherNoun, MannerAnaphor } },
        ),
        named_type(
            NOMINAL_FORM_TYPE,
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)] pub(crate) enum NominalForm { BareSingularNoun, ModifiedSingularNoun, SingularCoordination, BarePluralNoun, ModifiedPluralNoun, PluralCoordination, MassNoun } },
        ),
        named_type(
            NOMINAL_LICENSE_TYPE,
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)] pub(crate) enum NominalLicense { AnyNominal, CountNominal, BareSingularNoun, MassOrPluralCount } },
        ),
        named_type(
            NUMBER_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Number { Singular, Plural }
            },
        ),
        named_type(
            ONSET_TYPE,
            quote! {
                pub(crate) use ::deckmaste_construction_core::macro_def::Onset;
            },
        ),
        named_type(
            PARTICIPLE_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Participle { Participle }
            },
        ),
        named_type(
            POSSESSIVE_ENDING_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum PossessiveEnding { EndsInS, Other }
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
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
                pub(crate) enum CasePosition { DocumentInitial, SentenceInitial, Continuation }
            },
        ),
        named_type(
            PREFIX_POSITION_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
                pub(crate) enum PrefixPosition { WordOwnedSpace, SurfaceOwned, None }
            },
        ),
        named_type(
            "LexicalBoundary",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum LexicalBoundary {
                    Separated,
                    Adjacent,
                    LeftAdjacent,
                    BothAdjacent,
                }
            },
        ),
        named_type(
            STRUCTURAL_TRANSITION_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum StructuralTransition { Preserve, SentenceInitial, Continuation }
            },
        ),
        named_type(
            SCAN_POSITION_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
                pub(crate) struct ScanPosition {
                    pub(crate) byte_offset: usize,
                    pub(crate) case: CasePosition,
                    pub(crate) prefix: PrefixPosition,
                }
            },
        ),
        named_type(
            DECLARATION_CLASS_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub struct DeclarationClass {
                    kind: ::deckmaste_construction_core::macro_def::DeclarationKind,
                    position: ::deckmaste_construction_core::macro_def::GrammarPosition,
                }
            },
        ),
        named_type(
            DECLARATION_MATCHER_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) struct DeclarationMatcher {
                    pub(crate) kind: ::deckmaste_construction_core::macro_def::DeclarationKind,
                    pub(crate) name: &'static str,
                    pub(crate) position: ::deckmaste_construction_core::macro_def::GrammarPosition,
                    pub(crate) feature: FeatureConstraint<::deckmaste_construction_core::macro_def::SurfaceFeature>,
                }
            },
        ),
        named_type(
            DECLARATION_LEAF_TYPE,
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub(crate) struct DeclarationLeaf {
                    pub(crate) id: ::deckmaste_construction_core::macro_def::DeclarationIdentity,
                    pub(crate) feature: ::deckmaste_construction_core::macro_def::SurfaceFeature,
                    pub(crate) onset: Onset,
                }
            },
        ),
        named_type(
            "FormLiteralSurface",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub(crate) struct FormLiteralSurface {
                    pub(crate) surface: &'static str,
                    pub(crate) construction: &'static str,
                    pub(crate) form: &'static str,
                    pub(crate) atom_index: usize,
                    pub(crate) homograph_license: HomographLicense,
                }
            },
        ),
        named_type(
            "LexiconSurface",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub(crate) struct LexiconSurface {
                    pub(crate) surface: &'static str,
                    pub(crate) terminal: &'static str,
                    pub(crate) member: &'static str,
                    pub(crate) position: ::deckmaste_construction_core::macro_def::GrammarPosition,
                }
            },
        ),
        named_type(
            "VocabSurface",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub(crate) struct VocabSurface {
                    pub(crate) surface: &'static str,
                    pub(crate) vocabulary: &'static str,
                    pub(crate) member: &'static str,
                    pub(crate) homograph_license: HomographLicense,
                }
            },
        ),
        named_type(
            "VerbTailLiteralSurface",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub(crate) struct VerbTailLiteralSurface {
                    pub(crate) surface: &'static str,
                    pub(crate) codec: &'static str,
                    pub(crate) atom_index: usize,
                }
            },
        ),
    ];

    if !inventory.declaration_verbs.is_empty() {
        items.extend(emit_declaration_verb_frame_types());
    }
    items.extend(emit_lexical_types(&inventory));
    items.extend(emit_owner_types(&inventory));
    items.extend(emit_semantic_runtime_types(plan));
    items.extend(emit_generated_roots(plan));
    items.extend(emit_runtime_impls(&inventory));
    items.push(emit_form_literal_surfaces(plan));
    items.push(emit_lexicon_surfaces(plan));
    items.push(emit_vocab_surfaces(plan));
    items.push(emit_verb_tail_literal_surfaces(plan));
    if plan.has_open_declarations() {
        items.push(emit_required_declarations(plan));
    }
    let origins = plan.declaration_keys().to_vec();
    for item in &mut items {
        item.origins.clone_from(&origins);
    }
    items
}

fn emit_vocab_surfaces(plan: &SemanticPlan) -> GeneratedItem {
    let rows = plan
        .runtime_vocabs()
        .flat_map(|vocab| {
            let licenses = vocab.feature_members(crate::feature::Feature::HomographLicense);
            vocab.variants().iter().map(move |variant| {
                let surface = variant.word();
                let vocabulary = syn::LitStr::new(vocab.name(), vocab.name_ident().span());
                let member =
                    syn::LitStr::new(&identifier_key(variant.name()), variant.name().span());
                let homograph_license = licenses
                    .and_then(|members| {
                        members
                            .iter()
                            .find(|(name, _)| name == &identifier_key(variant.name()))
                            .map(|(_, value)| value)
                    })
                    .map_or_else(
                        || quote! { HomographLicense::Unlicensed },
                        |value| match value {
                            crate::feature::FeatureValue::HomographUnlicensed => {
                                quote! { HomographLicense::Unlicensed }
                            }
                            crate::feature::FeatureValue::HomographLicensed => {
                                quote! { HomographLicense::Licensed }
                            }
                            _ => unreachable!("sealed homograph license has its closed domain"),
                        },
                    );
                quote! {
                    VocabSurface {
                        surface: #surface,
                        vocabulary: #vocabulary,
                        member: #member,
                        homograph_license: #homograph_license,
                    }
                }
            })
        })
        .collect::<Vec<_>>();

    GeneratedItem::new(
        ItemKey::Named {
            kind: crate::plan::NamedKind::Constant,
            name: "VOCAB_SURFACES".to_owned(),
        },
        quote! {
            pub(crate) const VOCAB_SURFACES: &[VocabSurface] = &[#(#rows),*];
        },
        plan.declaration_keys().to_vec(),
    )
}

fn emit_verb_tail_literal_surfaces(plan: &SemanticPlan) -> GeneratedItem {
    let rows = plan
        .runtime_declaration_verbs()
        .flat_map(|(_, verb)| {
            verb.frame_key()
                .atoms()
                .iter()
                .enumerate()
                .filter_map(move |(atom_index, atom)| {
                    let crate::semantic::VerbFrameAtom::Literal(surface) = atom else {
                        return None;
                    };
                    let surface = syn::LitStr::new(surface, Span::call_site());
                    let codec = syn::LitStr::new(verb.codec_name(), verb.codec_ident().span());
                    Some(quote! {
                        VerbTailLiteralSurface {
                            surface: #surface,
                            codec: #codec,
                            atom_index: #atom_index,
                        }
                    })
                })
        })
        .collect::<Vec<_>>();

    GeneratedItem::new(
        ItemKey::Named {
            kind: crate::plan::NamedKind::Constant,
            name: "VERB_TAIL_LITERAL_SURFACES".to_owned(),
        },
        quote! {
            pub(crate) const VERB_TAIL_LITERAL_SURFACES: &[VerbTailLiteralSurface] = &[#(#rows),*];
        },
        plan.declaration_keys().to_vec(),
    )
}

fn emit_lexicon_surfaces(plan: &SemanticPlan) -> GeneratedItem {
    let rows = [
        (
            plan.runtime_noun_lexeme(),
            crate::macro_def::GrammarPosition::Noun,
        ),
        (
            plan.runtime_verb_lexeme(),
            crate::macro_def::GrammarPosition::Verb,
        ),
    ]
    .into_iter()
    .flat_map(|(lexeme, position)| {
        lexeme.into_iter().flat_map(move |lexeme| {
            lexeme.surfaces().iter().map(move |row| {
                let surface = syn::LitStr::new(row.surface(), Span::call_site());
                let terminal = syn::LitStr::new(lexeme.name(), lexeme.name_ident().span());
                let member = syn::LitStr::new(row.member(), lexeme.name_ident().span());
                let position = crate::emit::grammar_position(position);
                quote! {
                    LexiconSurface {
                        surface: #surface,
                        terminal: #terminal,
                        member: #member,
                        position: #position,
                    }
                }
            })
        })
    })
    .collect::<Vec<_>>();

    GeneratedItem::new(
        ItemKey::Named {
            kind: crate::plan::NamedKind::Constant,
            name: "LEXICON_SURFACES".to_owned(),
        },
        quote! {
            pub(crate) const LEXICON_SURFACES: &[LexiconSurface] = &[#(#rows),*];
        },
        plan.declaration_keys().to_vec(),
    )
}

fn emit_form_literal_surfaces(plan: &SemanticPlan) -> GeneratedItem {
    let rows = plan
        .constructions()
        .iter()
        .flat_map(|construction| {
            construction.forms().iter().flat_map(move |form| {
                form.atoms()
                    .iter()
                    .enumerate()
                    .filter_map(move |(atom_index, atom)| {
                        let surface = match atom {
                            AtomPlan::Literal(surface)
                            | AtomPlan::SentenceInitialLiteral(surface)
                            | AtomPlan::StructuralLiteral(surface) => surface,
                            AtomPlan::Category { .. }
                            | AtomPlan::Lex { .. }
                            | AtomPlan::LexFixed { .. }
                            | AtomPlan::Marked { .. }
                            | AtomPlan::Identity { .. }
                            | AtomPlan::Noun { .. }
                            | AtomPlan::VerbFixed { .. }
                            | AtomPlan::OpenDeclaration(_)
                            | AtomPlan::Bound { .. }
                            | AtomPlan::Circumfix { .. } => return None,
                        };
                        let surface = syn::LitStr::new(surface, Span::call_site());
                        let construction = syn::LitStr::new(
                            construction.construction_id(),
                            construction.origin_span(),
                        );
                        let homograph_license = if form.literal_is_licensed(atom_index) {
                            quote! { HomographLicense::Licensed }
                        } else {
                            quote! { HomographLicense::Unlicensed }
                        };
                        let form = syn::LitStr::new(form.name(), form.origin_span());
                        Some(quote! {
                            FormLiteralSurface {
                                surface: #surface,
                                construction: #construction,
                                form: #form,
                                atom_index: #atom_index,
                                homograph_license: #homograph_license,
                            }
                        })
                    })
            })
        })
        .collect::<Vec<_>>();

    GeneratedItem::new(
        ItemKey::Named {
            kind: crate::plan::NamedKind::Constant,
            name: "FORM_LITERAL_SURFACES".to_owned(),
        },
        quote! {
            pub(crate) const FORM_LITERAL_SURFACES: &[FormLiteralSurface] = &[#(#rows),*];
        },
        plan.declaration_keys().to_vec(),
    )
}

fn emit_declaration_verb_frame_types() -> Vec<GeneratedItem> {
    let mut types = vec![
        named_type(
            "VerbFrameRolePreposition",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                pub(crate) struct VerbFrameRolePreposition {
                    terminal: &'static str,
                    member: &'static str,
                }
            },
        ),
        impl_item(
            None,
            "VerbFrameRolePreposition",
            quote! {
                impl VerbFrameRolePreposition {
                    pub(crate) const fn new(
                        terminal: &'static str,
                        member: &'static str,
                    ) -> Self {
                        Self { terminal, member }
                    }
                }
            },
        ),
        named_type(
            "VerbFrameClass",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                pub(crate) enum VerbFrameClass {
                    Predicate,
                    Auxiliary,
                    ProVerb,
                }
            },
        ),
        named_type(
            "VerbFrameAtom",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                pub(crate) enum VerbFrameAtom {
                    Literal(&'static str),
                    Lex(&'static str, &'static str),
                    OptionalLex(&'static str, &'static str),
                    MarkedRole(&'static str, &'static str, &'static str),
                    OptionalMarkedRole(&'static str, &'static str, &'static str),
                    Amount,
                    ObjectNounPhrase,
                    PredicativeComplement,
                    FrameComplementPair,
                    Role(&'static str),
                    OptionalRole(&'static str),
                }
            },
        ),
        named_type(
            "VerbFrameKey",
            quote! {
                /// Compiler compatibility key for matching a realized Lexical Verb Phrase
                /// against a declared Verb Frame.
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                pub(crate) struct VerbFrameKey {
                    class: VerbFrameClass,
                    atoms: &'static [VerbFrameAtom],
                }
            },
        ),
        impl_item(
            None,
            "VerbFrameKey",
            quote! {
                impl VerbFrameKey {
                    pub(crate) const fn new(atoms: &'static [VerbFrameAtom]) -> Self {
                        Self::with_class(VerbFrameClass::Predicate, atoms)
                    }

                    pub(crate) const fn with_class(
                        class: VerbFrameClass,
                        atoms: &'static [VerbFrameAtom],
                    ) -> Self {
                        Self { class, atoms }
                    }

                    pub(crate) const fn class(self) -> VerbFrameClass {
                        self.class
                    }

                    pub(crate) const fn atoms(self) -> &'static [VerbFrameAtom] {
                        self.atoms
                    }

                    pub(crate) fn matches_frame_set(
                        self,
                        frame_set: &::deckmaste_construction_core::macro_def::VerbFrameSet,
                    ) -> bool {
                        use ::deckmaste_construction_core::macro_def::CustomTailAtom;
                        use ::deckmaste_construction_core::macro_def::VerbFrameSet;

                        if self.class != VerbFrameClass::Predicate {
                            return false;
                        }
                        match frame_set {
                            VerbFrameSet::Intransitive => self.atoms.is_empty(),
                            VerbFrameSet::Transitive => {
                                self.atoms == [VerbFrameAtom::ObjectNounPhrase]
                            }
                            VerbFrameSet::MeasureComplement => {
                                self.atoms == [VerbFrameAtom::Amount]
                            }
                            VerbFrameSet::Custom { frames } => frames.iter().any(|frame| {
                                frame.len() == self.atoms.len()
                                    && frame.iter().zip(self.atoms).all(
                                        |(source, planned)| match (source, planned) {
                                            (
                                                CustomTailAtom::Literal(source),
                                                VerbFrameAtom::Literal(planned),
                                            ) => source == planned,
                                            (
                                                CustomTailAtom::Lex(source_terminal, source_variant),
                                                VerbFrameAtom::Lex(planned_terminal, planned_variant),
                                            ) => {
                                                source_terminal == planned_terminal
                                                    && source_variant == planned_variant
                                            }
                                            (CustomTailAtom::Amount, VerbFrameAtom::Amount)
                                            | (
                                                CustomTailAtom::ObjectNounPhrase,
                                                VerbFrameAtom::ObjectNounPhrase,
                                            )
                                            | (
                                                CustomTailAtom::PredicativeComplement,
                                                VerbFrameAtom::PredicativeComplement,
                                            ) => true,
                                            _ => false,
                                        },
                                    )
                            }),
                        }
                    }
                }
            },
        ),
    ];
    types.extend(emit_verb_frame_role_preemption_types());
    types
}

fn emit_verb_frame_role_preemption_types() -> [GeneratedItem; 2] {
    [
        named_type(
            "VerbFrameRolePreemption",
            quote! {
                pub(crate) struct VerbFrameRolePreemption {
                    roles: &'static [VerbFrameRolePreposition],
                    next: usize,
                }
            },
        ),
        impl_item(
            None,
            "VerbFrameRolePreemption",
            quote! {
                impl VerbFrameRolePreemption {
                    pub(crate) const fn new(
                        roles: &'static [VerbFrameRolePreposition],
                    ) -> Self {
                        Self { roles, next: 0 }
                    }

                    pub(crate) fn is_pending(
                        &self,
                        role: VerbFrameRolePreposition,
                    ) -> bool {
                        self.roles[self.next..].contains(&role)
                    }

                    pub(crate) fn fill(&mut self, role: VerbFrameRolePreposition) {
                        if let Some(offset) = self.roles[self.next..]
                            .iter()
                            .position(|candidate| *candidate == role)
                        {
                            self.next += offset + 1;
                        }
                    }
                }
            },
        ),
    ]
}

fn emit_generated_roots(plan: &SemanticPlan) -> Vec<GeneratedItem> {
    let origins = plan
        .roots()
        .iter()
        .map(|root| {
            crate::plan::DeclarationKey::new(
                crate::plan::SourceDeclarationKind::Root,
                root.category(),
            )
        })
        .collect::<Vec<_>>();
    let mut items = vec![
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Trait,
                name: "GeneratedRoot".to_owned(),
            },
            quote! {
                pub(crate) trait GeneratedRoot:
                    Clone + PartialEq + Eq + std::fmt::Debug
                {
                    const CATEGORY: Category;
                    const NAME: &'static str;
                    const EOI: bool;
                    fn from_build(value: BuildValue) -> Option<Self>;
                }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Trait,
                name: "GeneratedParseRoot".to_owned(),
            },
            quote! {
                pub(crate) trait GeneratedParseRoot: GeneratedRoot {
                    fn visit_scope(&self, visitor: &mut dyn Visitor);

                    fn render_with_claims(
                        &self,
                        context: &ParseContext<'_>,
                        environment: &crate::environment::ParserEnvironment,
                    ) -> (
                        String,
                        Vec<RawRenderedClaim>,
                    );
                }
            },
            origins,
        ),
    ];
    items.extend(plan.roots().iter().map(|root| {
        let category = emitted_ident(root.category(), Span::call_site());
        let name = syn::LitStr::new(root.category(), Span::call_site());
        let eoi = root.is_parse_entry();
        let concord_class = plan
            .category_carries_concord_class(root.category())
            .then(|| quote! { , _ });
        let cardinality = plan
            .category_carries_cardinality(root.category())
            .then(|| quote! { , _ });
        let number = plan
            .category_carries_number(root.category())
            .then(|| quote! { , _ });
        let determiner_number = plan
            .carries_feature(
                root.category(),
                crate::feature::Feature::DeterminerNumber,
            )
            .then(|| quote! { , _ });
        let fused_head_license = plan
            .carries_feature(
                root.category(),
                crate::feature::Feature::FusedHeadLicense,
            )
            .then(|| quote! { , _ });
        let nominal_license = plan
            .carries_feature(root.category(), crate::feature::Feature::NominalLicense)
            .then(|| quote! { , _ });
        let onset = plan
            .category_carries_onset(root.category())
            .then(|| quote! { , _ });
        let possessive_ending = plan
            .category_carries_possessive_ending(root.category())
            .then(|| quote! { , _ });
        let following_onset = super::semantic_types(plan)
            .into_iter()
            .find(|item| item.name == root.category())
            .is_some_and(|item| item.kind != super::SemanticTypeKind::Product)
            .then(|| quote! { , _ });
        GeneratedItem::new(
            ItemKey::Impl {
                trait_name: Some("GeneratedRoot".to_owned()),
                self_ty: root.category().to_owned(),
            },
            quote! {
                impl GeneratedRoot for #category {
                    const CATEGORY: Category = Category::#category;
                    const NAME: &'static str = #name;
                    const EOI: bool = #eoi;

                    fn from_build(value: BuildValue) -> Option<Self> {
                        match value {
                            BuildValue::#category(value #concord_class #cardinality #number #determiner_number #fused_head_license #nominal_license #onset #possessive_ending #following_onset) => Some(value),
                            _ => None,
                        }
                    }
                }
            },
            vec![crate::plan::DeclarationKey::new(
                crate::plan::SourceDeclarationKind::Root,
                root.category(),
            )],
        )
    }));
    let takes_environment = plan.needs_parser_environment();
    items.extend(
        plan.roots()
            .iter()
            .filter(|root| root.is_render_entry())
            .map(|root| emit_generated_parse_root(root, takes_environment)),
    );
    items
}

fn emit_generated_parse_root(root: &RootPlan, takes_environment: bool) -> GeneratedItem {
    let category = emitted_ident(root.category(), Span::call_site());
    let visit = emitted_ident(
        &format!("visit_{}", snake_case(root.category())),
        Span::call_site(),
    );
    let renderer = emitted_ident(
        &format!("render_{}_with_claims", snake_case(root.category())),
        Span::call_site(),
    );
    let environment = takes_environment.then(|| quote! { , environment });
    GeneratedItem::new(
        ItemKey::Impl {
            trait_name: Some("GeneratedParseRoot".to_owned()),
            self_ty: root.category().to_owned(),
        },
        quote! {
            impl GeneratedParseRoot for #category {
                fn visit_scope(&self, visitor: &mut dyn Visitor) {
                    visitor.#visit(self);
                }

                fn render_with_claims(
                    &self,
                    context: &ParseContext<'_>,
                    environment: &crate::environment::ParserEnvironment,
                ) -> (
                    String,
                    Vec<RawRenderedClaim>,
                ) {
                    #renderer(self, context #environment)
                }
            }
        },
        vec![crate::plan::DeclarationKey::new(
            crate::plan::SourceDeclarationKind::Root,
            root.category(),
        )],
    )
}

fn emit_build_rejection_types() -> Vec<GeneratedItem> {
    vec![
        named_type(
            BUILD_VIOLATION_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
                pub enum BuildViolation {
                    Length {
                        minimum: usize,
                        maximum: Option<usize>,
                        actual: usize,
                    },
                    Invariant {
                        identity: &'static str,
                    },
                }
            },
        ),
        impl_item(
            None,
            BUILD_VIOLATION_TYPE,
            quote! {
                impl BuildViolation {
                    pub const fn minimum(&self) -> Option<usize> {
                        match self {
                            Self::Length { minimum, .. } => Some(*minimum),
                            Self::Invariant { .. } => None,
                        }
                    }

                    pub const fn maximum(&self) -> Option<usize> {
                        match self {
                            Self::Length { maximum, .. } => *maximum,
                            Self::Invariant { .. } => None,
                        }
                    }

                    pub const fn actual(&self) -> Option<usize> {
                        match self {
                            Self::Length { actual, .. } => Some(*actual),
                            Self::Invariant { .. } => None,
                        }
                    }

                    pub const fn identity(&self) -> Option<&'static str> {
                        match self {
                            Self::Length { .. } => None,
                            Self::Invariant { identity } => Some(*identity),
                        }
                    }
                }
            },
        ),
        named_type(
            BUILD_REJECTION_TYPE,
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
                pub struct BuildRejection {
                    owner: &'static str,
                    role: &'static str,
                    violation: BuildViolation,
                }
            },
        ),
        impl_item(
            Some(BUILD_REJECTION_TYPE),
            BUILD_REJECTION_TYPE,
            quote! {
                impl BuildRejection {
                    pub(crate) const fn new(
                        owner: &'static str,
                        role: &'static str,
                        violation: BuildViolation,
                    ) -> Self {
                        Self { owner, role, violation }
                    }

                    pub const fn owner(&self) -> &'static str { self.owner }
                    pub const fn role(&self) -> &'static str { self.role }
                    pub const fn violation(&self) -> &BuildViolation { &self.violation }
                }
            },
        ),
        impl_item(
            Some("std::fmt::Display"),
            BUILD_REJECTION_TYPE,
            quote! {
                impl std::fmt::Display for BuildRejection {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self.violation {
                            BuildViolation::Length {
                                minimum,
                                maximum: Some(maximum),
                                actual,
                            } => write!(
                                formatter,
                                "{}.{}: length {} violates {}..={}",
                                self.owner,
                                self.role,
                                actual,
                                minimum,
                                maximum,
                            ),
                            BuildViolation::Length {
                                minimum,
                                maximum: None,
                                actual,
                            } => write!(
                                formatter,
                                "{}.{}: length {} violates minimum {}",
                                self.owner,
                                self.role,
                                actual,
                                minimum,
                            ),
                            BuildViolation::Invariant { identity } => write!(
                                formatter,
                                "{}.{}: invariant `{}` rejected",
                                self.owner,
                                self.role,
                                identity,
                            ),
                        }
                    }
                }
            },
        ),
    ]
}

fn emit_semantic_runtime_types(plan: &SemanticPlan) -> Vec<GeneratedItem> {
    let semantic_types = super::semantic_types(plan);
    let build_variants = semantic_types.iter().map(|item| {
        let name = emitted_ident(item.name, Span::call_site());
        match item.kind {
            super::SemanticTypeKind::Category => {
                let concord_class = plan
                    .category_carries_concord_class(item.name)
                    .then(|| quote! { , ConcordClass });
                let cardinality = plan
                    .category_carries_cardinality(item.name)
                    .then(|| quote! { , Cardinality });
                let number = plan
                    .category_carries_number(item.name)
                    .then(|| quote! { , Number });
                let determiner_number = plan
                    .carries_feature(item.name, crate::feature::Feature::DeterminerNumber)
                    .then(|| quote! { , DeterminerNumber });
                let fused_head_license = plan
                    .carries_feature(item.name, crate::feature::Feature::FusedHeadLicense)
                    .then(|| quote! { , FusedHeadLicense });
                let nominal_license = plan
                    .carries_feature(item.name, crate::feature::Feature::NominalLicense)
                    .then(|| quote! { , NominalLicense });
                let onset = plan
                    .category_carries_onset(item.name)
                    .then(|| quote! { , Onset });
                let possessive_ending = plan
                    .category_carries_possessive_ending(item.name)
                    .then(|| quote! { , PossessiveEnding });
                quote! { #name(#name #concord_class #cardinality #number #determiner_number #fused_head_license #nominal_license #onset #possessive_ending, FeatureConstraint<Onset>) }
            }
            super::SemanticTypeKind::Product => {
                quote! { #name(#name) }
            }
            super::SemanticTypeKind::Sum => {
                let concord_class = plan
                    .sum_carries_concord_class(item.name)
                    .then(|| quote! { , ConcordClass });
                let fused_head_license = plan
                    .carries_feature(item.name, crate::feature::Feature::FusedHeadLicense)
                    .then(|| quote! { , FusedHeadLicense });
                quote! { #name(#name #concord_class #fused_head_license, FeatureConstraint<Onset>) }
            }
        }
    });
    let helper_variants = super::structural_carriers(plan).into_iter().map(|carrier| {
        let name = emitted_ident(&carrier.value_variant(), Span::call_site());
        let ty = super::structural_carrier_type(carrier.field.kind());
        let features = plan
            .sequence_features(carrier.owner, carrier.field.name())
            .iter()
            .map(|feature| super::feature_type(*feature));
        quote! { #name(#ty #(, #features)*) }
    });
    let nonterminal_variants = semantic_types
        .iter()
        .map(|item| emitted_ident(item.name, Span::call_site()));
    let labels = semantic_types.iter().map(|item| {
        let name = emitted_ident(item.name, Span::call_site());
        let label = syn::LitStr::new(&snake_case(item.name).replace('_', " "), Span::call_site());
        quote! { Self::#name => #label }
    });
    let public_category_mappings = semantic_types.iter().map(|item| {
        let name = emitted_ident(item.name, Span::call_site());
        quote! { Category::#name => NonterminalCategory::#name }
    });
    let helper_category_mappings =
        super::structural_helper_categories(plan)
            .into_iter()
            .map(|category| {
                let helper = emitted_ident(&category.name, Span::call_site());
                let owner = emitted_ident(category.diagnostic_owner, Span::call_site());
                quote! { Category::#helper => NonterminalCategory::#owner }
            });

    let mut items = emit_build_rejection_types();
    items.extend([
        named_type(
            "BuildValue",
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub(crate) enum BuildValue {
                    #(#build_variants,)*
                    #(#helper_variants,)*
                    Leaf(Leaf),
                }
            },
        ),
        named_type(
            "NonterminalCategory",
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub enum NonterminalCategory {
                    #(#nonterminal_variants),*
                }
            },
        ),
        impl_item(
            Some("NonterminalCategory"),
            "NonterminalCategory",
            quote! {
                impl NonterminalCategory {
                    pub const fn label(self) -> &'static str {
                        match self { #(#labels,)* }
                    }
                }
            },
        ),
        impl_item(
            Some("std::fmt::Display"),
            "NonterminalCategory",
            quote! {
                impl std::fmt::Display for NonterminalCategory {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str(self.label())
                    }
                }
            },
        ),
        impl_item(
            Some("Category"),
            "Category",
            quote! {
                impl Category {
                    pub(crate) const fn nonterminal_category(self) -> NonterminalCategory {
                        match self {
                            #(#public_category_mappings,)*
                            #(#helper_category_mappings,)*
                        }
                    }
                }
            },
        ),
        impl_item(
            Some("From<Category>"),
            "NonterminalCategory",
            quote! {
                impl From<Category> for NonterminalCategory {
                    fn from(value: Category) -> Self {
                        value.nonterminal_category()
                    }
                }
            },
        ),
    ]);
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
        .expect("validated open declarations have sealed concord_class constraints");
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

fn vocab_lexical_variants(inventory: &RuntimeInventory<'_>) -> Vec<TokenStream> {
    inventory
        .vocabs
        .iter()
        .map(|vocab| {
            let ident = vocab.name_ident();
            if inventory
                .unused_vocab_lexicals
                .iter()
                .any(|unused| unused.name() == vocab.name())
            {
                quote! {
                    #[allow(
                        dead_code,
                        reason = "closed vocabulary remains generated until its first construction"
                    )]
                    #ident
                }
            } else {
                quote! { #ident }
            }
        })
        .collect()
}

fn catalog_lexical_variants(
    inventory: &RuntimeInventory<'_>,
) -> (
    Option<TokenStream>,
    Option<TokenStream>,
    Option<TokenStream>,
) {
    let present = !inventory.catalog_identities.is_empty();
    let lexical = present.then(|| quote! { CatalogIdentity(usize), });
    let leaf = present.then(|| {
        quote! {
            CatalogIdentity {
                provider: CatalogProvider,
                canonical_identity: std::sync::Arc<str>,
                onset: Onset,
                possessive_ending: PossessiveEnding,
            },
        }
    });
    let class = present.then(|| quote! { CatalogIdentity(usize), });
    (lexical, leaf, class)
}

fn signed_lexical_variants(
    inventory: &RuntimeInventory<'_>,
) -> (
    Option<TokenStream>,
    Option<TokenStream>,
    Option<TokenStream>,
) {
    let lexical = inventory.signed_decimal.map(|codec| {
        let variant = codec.codec_ident();
        quote! { #variant, }
    });
    let leaf = inventory.signed_decimal.map(|codec| {
        let variant = codec.codec_ident();
        quote! { #variant(#variant), }
    });
    let class = inventory.signed_decimal.map(|codec| {
        let variant = codec.codec_ident();
        quote! { #variant, }
    });
    (lexical, leaf, class)
}

fn unsigned_lexical_variants(
    inventory: &RuntimeInventory<'_>,
) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
    let lexical = inventory
        .unsigned_numbers
        .iter()
        .map(|codec| {
            let variant = codec.codec_ident();
            quote! { #variant, }
        })
        .collect();
    let leaf = inventory
        .unsigned_numbers
        .iter()
        .map(|codec| {
            let variant = codec.codec_ident();
            quote! { #variant(#variant), }
        })
        .collect();
    let class = inventory
        .unsigned_numbers
        .iter()
        .map(|codec| {
            let variant = codec.codec_ident();
            quote! { #variant, }
        })
        .collect();
    (lexical, leaf, class)
}

fn declaration_verb_lexical_variants(
    inventory: &RuntimeInventory<'_>,
) -> (Option<TokenStream>, Vec<TokenStream>, Option<TokenStream>) {
    let has_concord_class = inventory
        .declaration_verbs
        .iter()
        .any(|(_, codec)| codec.feature_axis() == crate::feature::Feature::ConcordClass);
    let has_participle = inventory
        .declaration_verbs
        .iter()
        .any(|(_, codec)| codec.feature_axis() == crate::feature::Feature::Participle);
    let concord_class = has_concord_class
        .then(|| quote! { DeclarationVerb(usize, FeatureConstraint<ConcordClass>), });
    let participle = has_participle.then(|| quote! { DeclarationParticiple(usize), });
    let lexical =
        (!inventory.declaration_verbs.is_empty()).then(|| quote! { #concord_class #participle });
    let leaf = inventory
        .declaration_verbs
        .iter()
        .map(|(_, codec)| {
            let verb = codec.codec_ident();
            match codec.feature_axis() {
                crate::feature::Feature::ConcordClass => {
                    quote! { #verb { verb: #verb, concord_class: ConcordClass, onset: Onset }, }
                }
                crate::feature::Feature::Participle => {
                    quote! { #verb { verb: #verb, onset: Onset }, }
                }
                _ => unreachable!("validated declaration_verb feature axis is closed"),
            }
        })
        .collect();
    let class =
        (!inventory.declaration_verbs.is_empty()).then(|| quote! { DeclarationVerb(usize), });
    (lexical, leaf, class)
}

fn declaration_noun_lexical_variants(
    inventory: &RuntimeInventory<'_>,
) -> (Option<TokenStream>, Vec<TokenStream>, Option<TokenStream>) {
    let lexical = (!inventory.declaration_nouns.is_empty())
        .then(|| quote! { DeclarationNoun(usize, FeatureConstraint<Number>), });
    let leaf = inventory
        .declaration_nouns
        .iter()
        .map(|(_, codec)| {
            let noun = codec.codec_ident();
            quote! { #noun { noun: #noun, number: Number, onset: Onset, possessive_ending: PossessiveEnding }, }
        })
        .collect();
    let class =
        (!inventory.declaration_nouns.is_empty()).then(|| quote! { DeclarationNoun(usize), });
    (lexical, leaf, class)
}

fn declaration_term_lexical_variants(
    inventory: &RuntimeInventory<'_>,
) -> (
    Option<TokenStream>,
    Option<TokenStream>,
    Option<TokenStream>,
) {
    let present = !inventory.declaration_terms.is_empty();
    (
        present.then(|| quote! { DeclarationTerm(u16), }),
        present.then(|| {
            quote! {
                DeclarationTerm {
                    terminal_index: u16,
                    id: ::deckmaste_construction_core::macro_def::DeclarationIdentity,
                    parameter: Option<(u16, Option<::deckmaste_construction_core::macro_def::FixedKeywordNominalNumber>)>,
                    onset: Onset,
                    possessive_ending: PossessiveEnding,
                },
            }
        }),
        present.then(|| quote! { DeclarationTerm(u16), }),
    )
}

#[expect(
    clippy::too_many_lines,
    reason = "lexical type emission is a single ordered inventory pass"
)]
fn emit_lexical_types(inventory: &RuntimeInventory<'_>) -> Vec<GeneratedItem> {
    let vocab_variants = vocab_lexical_variants(inventory);
    let vocab_leaf_variants = inventory.vocabs.iter().map(|vocab| {
        let ident = vocab.name_ident();
        quote! { #ident(#ident) }
    });
    let vocab_class_variants = inventory.vocabs.iter().map(|vocab| vocab.name_ident());

    let noun_lexical = inventory
        .noun_type()
        .map(|_| quote! { Noun(FeatureConstraint<Number>), });
    let noun_leaf = inventory.noun_type().map(|noun_type| {
        quote! { Noun { noun: #noun_type, number: Number, onset: Onset, possessive_ending: PossessiveEnding }, }
    });
    let noun_class = inventory.noun_type().map(|_| quote! { Noun, });
    let (declaration_noun_lexical, declaration_noun_leaf_variants, declaration_noun_class) =
        declaration_noun_lexical_variants(inventory);
    let (declaration_term_lexical, declaration_term_leaf, declaration_term_class) =
        declaration_term_lexical_variants(inventory);
    let (declaration_verb_lexical, declaration_verb_leaf_variants, declaration_verb_class) =
        declaration_verb_lexical_variants(inventory);

    let verb_lexical = inventory.verb_lexeme.map(|lexeme| {
        let ident = lexeme.name_ident();
        quote! { Verb(#ident, FeatureConstraint<ConcordClass>), }
    });
    let verb_leaf = inventory.verb_lexeme.map(|lexeme| {
        let ident = lexeme.name_ident();
        quote! { Verb { lexeme: #ident, concord_class: ConcordClass, onset: Onset }, }
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
    let (catalog_lexical, catalog_leaf, catalog_class) = catalog_lexical_variants(inventory);
    let (signed_lexical, signed_leaf, signed_class) = signed_lexical_variants(inventory);
    let (unsigned_lexical, unsigned_leaf, unsigned_class) = unsigned_lexical_variants(inventory);
    let declaration_determinative_lexical = (!inventory.declaration_determinatives.is_empty())
        .then(|| quote! { DeclarationDeterminative(usize), });
    let declaration_determinative_leaf = inventory.declaration_determinatives.iter().map(|(_, codec)| {
        let ty = codec.codec_ident();
        quote! { #ty { value: #ty, onset: Onset, following_onset: FeatureConstraint<Onset>, number_license: DeterminerNumber, quantification: Quantification, fused_head_license: FusedHeadLicense, nominal_license: NominalLicense, bare_duration_license: BareDurationLicense }, }
    });
    let declaration_determinative_class = (!inventory.declaration_determinatives.is_empty())
        .then(|| quote! { DeclarationDeterminative(usize), });

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
                    #declaration_determinative_lexical
                    #declaration_noun_lexical
                    #declaration_term_lexical
                    #declaration_verb_lexical
                    #verb_lexical
                    #(#direct_lexical_variants)*
                    #(#opaque_lexical_variants)*
                    #(#context_lexical_variants)*
                    #catalog_lexical
                    #signed_lexical
                    #(#unsigned_lexical)*
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
                    #(#declaration_determinative_leaf)*
                    #(#declaration_noun_leaf_variants)*
                    #declaration_term_leaf
                    #(#declaration_verb_leaf_variants)*
                    #verb_leaf
                    #(#direct_leaf_variants)*
                    #(#opaque_leaf_variants)*
                    #(#context_leaf_variants)*
                    #catalog_leaf
                    #signed_leaf
                    #(#unsigned_leaf)*
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
                    #declaration_determinative_class
                    #declaration_noun_class
                    #declaration_term_class
                    #declaration_verb_class
                    #verb_class
                    #(#direct_class_variants,)*
                    #(#opaque_class_variants,)*
                    #(#context_class_variants,)*
                    #catalog_class
                    #signed_class
                    #(#unsigned_class)*
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
                    pub(crate) right_boundary: LexicalBoundary,
                }
            },
        ),
    ]
}

fn emit_owner_types(inventory: &RuntimeInventory<'_>) -> Vec<GeneratedItem> {
    let noun_lexeme = inventory
        .noun_lexeme
        .filter(|_| !inventory.noun_lexeme_is_aggregated)
        .map(|_| quote! { NounLexeme, });
    let declaration_noun =
        (!inventory.declaration_nouns.is_empty()).then(|| quote! { DeclarationNoun(usize), });
    let declaration_determinative = (!inventory.declaration_determinatives.is_empty())
        .then(|| quote! { DeclarationDeterminative(usize), });
    let declaration_term =
        (!inventory.declaration_terms.is_empty()).then(|| quote! { DeclarationTerm(u16), });
    let declaration_verb =
        (!inventory.declaration_verbs.is_empty()).then(|| quote! { DeclarationVerb(usize), });
    let catalog_identity =
        (!inventory.catalog_identities.is_empty()).then(|| quote! { CatalogIdentity(usize), });
    let catalog_owner_identity = (!inventory.catalog_identities.is_empty()).then(|| {
        quote! {
            Catalog(std::sync::Arc<(
                CatalogProvider,
                std::sync::Arc<str>,
                std::sync::OnceLock<String>,
            )>),
        }
    });
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
                    Structural {
                        stable_id: &'static str,
                        transition: StructuralTransition,
                    },
                    Static {
                        kind: LexicalProvenanceKind,
                        stable_id: &'static str,
                    },
                    TransitionedStatic {
                        kind: LexicalProvenanceKind,
                        stable_id: &'static str,
                        transition: StructuralTransition,
                    },
                    Vocab {
                        declaration: &'static str,
                    },
                    Lexeme {
                        declaration: &'static str,
                        member: &'static str,
                    },
                    #noun_lexeme
                    Identity {
                        declaration: &'static str,
                    },
                    #catalog_identity
                    #declaration_noun
                    #declaration_determinative
                    #declaration_term
                    #declaration_verb
                    Declaration {
                        kind: ::deckmaste_construction_core::macro_def::DeclarationKind,
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
                        ::deckmaste_construction_core::macro_def::DeclarationIdentity,
                        ::deckmaste_construction_core::macro_def::SurfaceFeature,
                        std::sync::OnceLock<String>,
                    )>),
                    #catalog_owner_identity
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
    let declaration_noun_class_arm = (!inventory.declaration_nouns.is_empty()).then(|| {
        quote! { Lexical::DeclarationNoun(terminal_index, _) => TerminalClass::DeclarationNoun(terminal_index), }
    });
    let declaration_determinative_class_arm = (!inventory.declaration_determinatives.is_empty())
        .then(|| {
            quote! {
                Lexical::DeclarationDeterminative(terminal_index) =>
                    TerminalClass::DeclarationDeterminative(terminal_index),
            }
        });
    let declaration_term_class_arm = (!inventory.declaration_terms.is_empty()).then(|| {
        quote! { Lexical::DeclarationTerm(terminal_index) => TerminalClass::DeclarationTerm(terminal_index), }
    });
    let declaration_verb_class_arm = (!inventory.declaration_verbs.is_empty()).then(|| {
        let concord_class = inventory.declaration_verbs.iter().any(|(_, codec)| codec.feature_axis() == crate::feature::Feature::ConcordClass).then(|| quote! {
            Lexical::DeclarationVerb(terminal_index, _) => TerminalClass::DeclarationVerb(terminal_index),
        });
        let participle = inventory.declaration_verbs.iter().any(|(_, codec)| codec.feature_axis() == crate::feature::Feature::Participle).then(|| quote! {
            Lexical::DeclarationParticiple(terminal_index) => TerminalClass::DeclarationVerb(terminal_index),
        });
        quote! { #concord_class #participle }
    });
    let verb_class_arm = inventory
        .verb_lexeme
        .map(|_| quote! { Lexical::Verb(_, _) => TerminalClass::VerbLexeme, });
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
    let unsigned_class_arms = inventory.unsigned_numbers.iter().map(|codec| {
        let variant = codec.codec_ident();
        quote! { Lexical::#variant => TerminalClass::#variant, }
    });
    let context_class_arms = inventory.context_identities.iter().map(|identity| {
        let variant = identity.aggregate_ident();
        quote! { Lexical::#variant => TerminalClass::#variant, }
    });
    let catalog_class_arm = (!inventory.catalog_identities.is_empty()).then(|| {
        quote! {
            Lexical::CatalogIdentity(terminal_index) =>
                TerminalClass::CatalogIdentity(terminal_index),
        }
    });
    let context_identity_arms = inventory.context_identities.iter().map(|identity| {
        let variant = identity.aggregate_ident();
        quote! { Lexical::#variant => true, }
    });
    let catalog_identity_arm = (!inventory.catalog_identities.is_empty())
        .then(|| quote! { Lexical::CatalogIdentity(_) => true, });
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
    let declaration_noun_label = (!inventory.declaration_nouns.is_empty())
        .then(|| quote! { TerminalClass::DeclarationNoun(_) => "declaration noun", });
    let declaration_determinative_label = (!inventory.declaration_determinatives.is_empty()).then(
        || quote! { TerminalClass::DeclarationDeterminative(_) => "declaration determinative", },
    );
    let declaration_term_label = (!inventory.declaration_terms.is_empty())
        .then(|| quote! { TerminalClass::DeclarationTerm(_) => "declaration term", });
    let declaration_verb_label = (!inventory.declaration_verbs.is_empty())
        .then(|| quote! { TerminalClass::DeclarationVerb(_) => "declaration verb", });
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
    let unsigned_labels = inventory.unsigned_numbers.iter().map(|codec| {
        let variant = codec.codec_ident();
        let label = syn::LitStr::new(
            &snake_case(codec.codec_name()).replace('_', " "),
            Span::call_site(),
        );
        quote! { TerminalClass::#variant => #label, }
    });
    let catalog_label = (!inventory.catalog_identities.is_empty())
        .then(|| quote! { TerminalClass::CatalogIdentity(_) => "catalog identity", });
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
            Some("StructuralTransition"),
            "StructuralTransition",
            quote! {
                impl StructuralTransition {
                    pub(crate) const fn case_after(self, current: CasePosition) -> CasePosition {
                        match self {
                            StructuralTransition::Preserve => current,
                            StructuralTransition::SentenceInitial => CasePosition::SentenceInitial,
                            StructuralTransition::Continuation => CasePosition::Continuation,
                        }
                    }

                    pub(crate) const fn position_after(
                        self,
                        current: ScanPosition,
                        byte_offset: usize,
                    ) -> ScanPosition {
                        ScanPosition {
                            byte_offset,
                            case: self.case_after(current.case),
                            prefix: PrefixPosition::SurfaceOwned,
                        }
                    }
                }
            },
        ),
        impl_item(
            Some("Lexical"),
            "Lexical",
            quote! {
                impl Lexical {
                    /// Whether this terminal matches an identity: a spelling
                    /// supplied by the parse context or a catalog, rather than
                    /// a member of a declared type.
                    pub(crate) const fn is_identity(self) -> bool {
                        match self {
                            #(#context_identity_arms)*
                            #catalog_identity_arm
                            _ => false,
                        }
                    }

                    pub(crate) const fn class(self) -> TerminalClass {
                        match self {
                            Lexical::Literal(_) => unreachable!(),
                            Lexical::EndOfInput => TerminalClass::EndOfInput,
                            #(#vocab_class_arms)*
                            #noun_class_arm
                            #declaration_noun_class_arm
                            #declaration_determinative_class_arm
                            #declaration_term_class_arm
                            #declaration_verb_class_arm
                            #verb_class_arm
                            #(#direct_class_arms)*
                            #(#opaque_class_arms)*
                            #(#context_class_arms)*
                            #catalog_class_arm
                            #signed_class_arm
                            #(#unsigned_class_arms)*
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

                    pub(crate) const fn position_before(
                        self,
                        current: ScanPosition,
                    ) -> ScanPosition {
                        match self.right_boundary {
                            LexicalBoundary::LeftAdjacent | LexicalBoundary::BothAdjacent => ScanPosition {
                                prefix: PrefixPosition::SurfaceOwned,
                                ..current
                            },
                            LexicalBoundary::Separated | LexicalBoundary::Adjacent => current,
                        }
                    }

                    pub(crate) const fn suppress_right_boundary(self) -> Self {
                        Self {
                            right_boundary: match self.right_boundary {
                                LexicalBoundary::Separated | LexicalBoundary::Adjacent => {
                                    LexicalBoundary::Adjacent
                                }
                                LexicalBoundary::LeftAdjacent | LexicalBoundary::BothAdjacent => {
                                    LexicalBoundary::BothAdjacent
                                }
                            },
                            ..self
                        }
                    }

                    pub(crate) const fn position_after(
                        self,
                        current: ScanPosition,
                        byte_offset: usize,
                    ) -> ScanPosition {
                        match self.owner {
                            LexicalOwnerTemplate::Structural { transition, .. }
                            | LexicalOwnerTemplate::TransitionedStatic { transition, .. } => {
                                transition.position_after(current, byte_offset)
                            }
                            LexicalOwnerTemplate::None => ScanPosition {
                                byte_offset,
                                case: current.case,
                                prefix: current.prefix,
                            },
                            _ => ScanPosition {
                                byte_offset,
                                case: CasePosition::Continuation,
                                prefix: match self.right_boundary {
                                    LexicalBoundary::Separated => PrefixPosition::WordOwnedSpace,
                                    LexicalBoundary::Adjacent => PrefixPosition::SurfaceOwned,
                                    LexicalBoundary::LeftAdjacent => PrefixPosition::WordOwnedSpace,
                                    LexicalBoundary::BothAdjacent => PrefixPosition::SurfaceOwned,
                                },
                            },
                        }
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
                            #declaration_noun_label
                            #declaration_determinative_label
                            #declaration_term_label
                            #declaration_verb_label
                            #verb_label
                            #(#direct_labels)*
                            #(#opaque_labels)*
                            #(#context_labels)*
                            #catalog_label
                            #signed_label
                            #(#unsigned_labels)*
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
                    pub const fn kind(self) -> ::deckmaste_construction_core::macro_def::DeclarationKind {
                        self.kind
                    }

                    pub const fn position(self) -> ::deckmaste_construction_core::macro_def::GrammarPosition {
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
    let catalog_owner_arms =
        inventory
            .catalog_identities
            .iter()
            .map(|(terminal_index, identity)| {
                let provider = identity.provider();
                quote! {
                    (
                        LexicalOwnerTemplate::CatalogIdentity(#terminal_index),
                        Leaf::CatalogIdentity {
                            provider: CatalogProvider::#provider,
                            canonical_identity,
                            ..
                        },
                    ) => Some(LexicalOwner::catalog_owner(
                        CatalogProvider::#provider,
                        canonical_identity.clone(),
                    )),
                }
            });
    let catalog_owner_constructor = (!inventory.catalog_identities.is_empty()).then(|| {
        quote! {
            pub(crate) fn catalog_owner(
                provider: CatalogProvider,
                canonical_identity: std::sync::Arc<str>,
            ) -> Self {
                Self {
                    identity: LexicalOwnerIdentity::Catalog(std::sync::Arc::new((
                        provider,
                        canonical_identity,
                        std::sync::OnceLock::new(),
                    ))),
                }
            }
        }
    });
    let catalog_kind_arm = (!inventory.catalog_identities.is_empty()).then(|| {
        quote! {
            LexicalOwnerIdentity::Catalog(_) => LexicalProvenanceKind::Identity,
        }
    });
    let catalog_stable_id_arm = (!inventory.catalog_identities.is_empty()).then(|| {
        quote! {
            LexicalOwnerIdentity::Catalog(identity) => identity.2.get_or_init(|| {
                LexicalOwner::construct_label(|| {
                    format!("identity:{}/{}", identity.0.name(), identity.1)
                })
            }),
        }
    });
    let catalog_stable_id_owned_arm = (!inventory.catalog_identities.is_empty()).then(|| {
        quote! {
            LexicalOwnerIdentity::Catalog(_) => self.stable_id().to_owned(),
        }
    });
    let catalog_eq_arm = (!inventory.catalog_identities.is_empty()).then(|| {
        quote! {
            (
                LexicalOwnerIdentity::Catalog(left),
                LexicalOwnerIdentity::Catalog(right),
            ) => left.0 == right.0 && left.1 == right.1,
        }
    });
    let catalog_ord_arms = (!inventory.catalog_identities.is_empty()).then(|| {
        quote! {
            (
                LexicalOwnerIdentity::Catalog(left),
                LexicalOwnerIdentity::Catalog(right),
            ) => left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)),
            (LexicalOwnerIdentity::Catalog(_), _) => std::cmp::Ordering::Greater,
        }
    });
    let declaration_ord_arms = if inventory.catalog_identities.is_empty() {
        quote! {
            (LexicalOwnerIdentity::Declaration(_), _) => std::cmp::Ordering::Greater,
        }
    } else {
        quote! {
            (
                LexicalOwnerIdentity::Declaration(_),
                LexicalOwnerIdentity::Static { .. },
            ) => std::cmp::Ordering::Greater,
            (
                LexicalOwnerIdentity::Declaration(_),
                LexicalOwnerIdentity::Catalog(_),
            ) => std::cmp::Ordering::Less,
        }
    };
    let verb_lexeme_owner_arms = inventory.verb_lexeme.into_iter().flat_map(|lexeme| {
        let declaration_ident = lexeme.name_ident();
        let declaration = syn::LitStr::new(lexeme.name(), lexeme.name_ident().span());
        lexeme.surfaces().iter().map(move |row| {
            let member_ident = emitted_ident(row.member(), Span::call_site());
            let member = syn::LitStr::new(row.member(), Span::call_site());
            let concord_class = match row.feature() {
                crate::macro_def::SurfaceFeature::PLAIN => {
                    quote! { ConcordClass::Other }
                }
                crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => {
                    quote! { ConcordClass::ThirdPersonSingular }
                }
                crate::macro_def::SurfaceFeature::Inflectional(_)
                | crate::macro_def::SurfaceFeature::Singular
                | crate::macro_def::SurfaceFeature::Plural
                | crate::macro_def::SurfaceFeature::Fixed
                | crate::macro_def::SurfaceFeature::BoundSuffix
                | crate::macro_def::SurfaceFeature::BlockLabel => {
                    unreachable!("validated verb lexeme has the ConcordClass feature axis")
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
                        concord_class: #concord_class,
                        ..
                    },
                ) => Some(LexicalOwner::static_owner(
                    LexicalProvenanceKind::Lexeme,
                    #stable_id,
                )),
            }
        })
    });
    let noun_lexeme_owner_arms = inventory
        .noun_lexeme
        .filter(|_| !inventory.noun_lexeme_is_aggregated)
        .into_iter()
        .flat_map(|lexeme| {
            let noun = lexeme.name_ident();
            lexeme.surfaces().iter().map(move |row| {
                let member = emitted_ident(row.member(), Span::call_site());
                let number = match row.feature() {
                    crate::macro_def::SurfaceFeature::Singular => {
                        quote! { Number::Singular }
                    }
                    crate::macro_def::SurfaceFeature::Plural => {
                        quote! { Number::Plural }
                    }
                    crate::macro_def::SurfaceFeature::Inflectional(_)
                    | crate::macro_def::SurfaceFeature::Fixed
                    | crate::macro_def::SurfaceFeature::BoundSuffix
                    | crate::macro_def::SurfaceFeature::BlockLabel => {
                        unreachable!("validated noun lexeme has the Number feature axis")
                    }
                };
                let stable_id =
                    crate::emit::closed_lexeme_owner_id(lexeme.name(), row.member(), row.feature());
                quote! {
                    (
                        LexicalOwnerTemplate::NounLexeme,
                        Leaf::Noun {
                            noun: #noun::#member,
                            number: #number,
                            ..
                        },
                    ) => Some(LexicalOwner::static_owner(
                        LexicalProvenanceKind::Lexeme,
                        #stable_id,
                    )),
                }
            })
        });
    let declaration_noun_owner =
        inventory
            .declaration_nouns
            .iter()
            .map(|(terminal_index, codec)| {
                let noun = codec.codec_ident();
                let closed_owner_arms = if let Some(closed) = codec.closed_lexeme() {
                    inventory
                        .noun_lexeme
                        .expect("validated declaration noun has its closed lexeme provider")
                        .surfaces()
                        .iter()
                        .map(|row| {
                            let member = emitted_ident(row.member(), Span::call_site());
                            let number = match row.feature() {
                                crate::macro_def::SurfaceFeature::Singular => {
                                    quote! { Number::Singular }
                                }
                                crate::macro_def::SurfaceFeature::Plural => quote! { Number::Plural },
                                crate::macro_def::SurfaceFeature::Inflectional(_)
                                | crate::macro_def::SurfaceFeature::Fixed
                                | crate::macro_def::SurfaceFeature::BoundSuffix
                                | crate::macro_def::SurfaceFeature::BlockLabel => {
                                    unreachable!(
                                        "validated noun lexeme has the Number feature axis"
                                    )
                                }
                            };
                            let stable_id = crate::emit::closed_lexeme_owner_id(
                                &closed.to_string(),
                                row.member(),
                                row.feature(),
                            );
                            quote! {
                                (
                                    LexicalOwnerTemplate::DeclarationNoun(#terminal_index),
                                    Leaf::#noun {
                                        noun: #noun::Lexeme(#closed::#member),
                                        number: #number,
                                        ..
                                    },
                                ) => Some(LexicalOwner::static_owner(
                                    LexicalProvenanceKind::Lexeme,
                                    #stable_id,
                                )),
                            }
                        })
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                };
                quote! {
                    #(#closed_owner_arms)*
                    (
                        LexicalOwnerTemplate::DeclarationNoun(#terminal_index),
                        Leaf::#noun {
                            noun: #noun::Declaration(declaration),
                            number,
                            ..
                        },
                    ) => Some(LexicalOwner::declaration_owner(
                        declaration.id().clone(),
                        match number {
                            Number::Singular => ::deckmaste_construction_core::macro_def::SurfaceFeature::Singular,
                            Number::Plural => ::deckmaste_construction_core::macro_def::SurfaceFeature::Plural,
                        },
                    )),
                }
            });
    let declaration_determinative_owner =
        inventory
            .declaration_determinatives
            .iter()
            .map(|(terminal_index, codec)| {
                let ty = codec.codec_ident();
                let lemma = codec.lemma_ident();
                let closed = codec.closed().iter().map(|member| {
                    let member = member.lemma();
                    let stable_id = syn::LitStr::new(
                        &format!("determinative:{}/{member}", codec.codec_name()),
                        Span::call_site(),
                    );
                    quote! {
                        (
                            LexicalOwnerTemplate::DeclarationDeterminative(#terminal_index),
                            Leaf::#ty { value: #ty::Closed(#lemma::#member), .. },
                        ) => Some(LexicalOwner::static_owner(
                            LexicalProvenanceKind::Codec,
                            #stable_id,
                        )),
                    }
                });
                quote! {
                    #(#closed)*
                }
            });
    let declaration_verb_owner =
        inventory
            .declaration_verbs
            .iter()
            .map(|(terminal_index, codec)| {
                let verb = codec.codec_ident();
                let closed_owner_arms = codec.closed_lexeme().map_or_else(Vec::new, |closed| {
                    inventory.lexemes.iter().copied()
                        .find(|lexeme| closed == lexeme.name())
                        .expect("validated declaration verb has its closed lexeme")
                        .surfaces().iter().map(|row| {
                            let member = emitted_ident(row.member(), Span::call_site());
                            let stable_id = crate::emit::closed_lexeme_owner_id(
                                &closed.to_string(), row.member(), row.feature(),
                            );
                            match codec.feature_axis() {
                                crate::feature::Feature::ConcordClass => {
                                    let concord_class = match row.feature() {
                                        crate::macro_def::SurfaceFeature::PLAIN => quote! { ConcordClass::Other },
                                        crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => quote! { ConcordClass::ThirdPersonSingular },
                                        _ => unreachable!("validated ConcordClass declaration verb has ConcordClass rows"),
                                    };
                                    quote! {
                                        (LexicalOwnerTemplate::DeclarationVerb(#terminal_index), Leaf::#verb {
                                            verb: #verb::Lexeme(#closed::#member), concord_class: #concord_class, ..
                                        }) => Some(LexicalOwner::static_owner(LexicalProvenanceKind::Lexeme, #stable_id)),
                                    }
                                }
                                crate::feature::Feature::Participle => {
                                    debug_assert_eq!(row.feature(), crate::macro_def::SurfaceFeature::PAST_PARTICIPLE);
                                    quote! {
                                        (LexicalOwnerTemplate::DeclarationVerb(#terminal_index), Leaf::#verb {
                                            verb: #verb::Lexeme(#closed::#member),
                                            ..
                                        }) => Some(LexicalOwner::static_owner(LexicalProvenanceKind::Lexeme, #stable_id)),
                                    }
                                }
                                _ => unreachable!("validated declaration verb feature axis is closed"),
                            }
                        }).collect::<Vec<_>>()
                });
                let declaration_pattern = if codec.closed_lexeme().is_some() {
                    quote! { #verb::Declaration(declaration) }
                } else {
                    quote! { declaration }
                };
                let closed_union_arm = codec.closed_lexeme().is_some().then(|| quote! {
                    (LexicalOwnerTemplate::DeclarationVerb(#terminal_index), Leaf::#verb {
                        verb: #verb::Lexeme(_),
                        concord_class: ConcordClass::OtherOrThirdPersonSingular,
                        ..
                    }) => unreachable!("a closed verb lexeme cannot carry an underspecified Concord Class"),
                });
                match codec.feature_axis() {
                    crate::feature::Feature::ConcordClass => quote! {
                        #(#closed_owner_arms)*
                        #closed_union_arm
                        (LexicalOwnerTemplate::DeclarationVerb(#terminal_index), Leaf::#verb {
                            verb: #declaration_pattern, concord_class, ..
                        }) => Some(match declaration.reference() {
                            crate::environment::VerbInventoryRef::Core(identity) => LexicalOwner::static_owner(
                                LexicalProvenanceKind::Lexeme,
                                identity.owner_id(),
                            ),
                            crate::environment::VerbInventoryRef::Declaration(id) => {
                                let feature = match concord_class {
                                    ConcordClass::Other => ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN,
                                    ConcordClass::ThirdPersonSingular => ::deckmaste_construction_core::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                                    ConcordClass::OtherOrThirdPersonSingular => ::deckmaste_construction_core::macro_def::SurfaceFeature::Inflectional(
                                        ::deckmaste_construction_core::macro_def::InflectionalForm::Preterite,
                                    ),
                                    ConcordClass::PlainOrPreterite => ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN,
                                };
                                LexicalOwner::declaration_owner(id.clone(), feature)
                            }
                        }),
                    },
                    crate::feature::Feature::Participle => quote! {
                        #(#closed_owner_arms)*
                        (LexicalOwnerTemplate::DeclarationVerb(#terminal_index), Leaf::#verb {
                            verb: #declaration_pattern, ..
                        }) => Some(match declaration.reference() {
                            crate::environment::VerbInventoryRef::Core(identity) => LexicalOwner::static_owner(
                                LexicalProvenanceKind::Lexeme,
                                identity.owner_id(),
                            ),
                            crate::environment::VerbInventoryRef::Declaration(id) => LexicalOwner::declaration_owner(id.clone(), ::deckmaste_construction_core::macro_def::SurfaceFeature::PAST_PARTICIPLE),
                        }),
                    },
                    _ => unreachable!("validated declaration verb feature axis is closed"),
                }
            });
    let declaration_term_owner =
        inventory
            .declaration_terms
            .iter()
            .map(|(terminal_index, codec)| {
                let terminal_index = syn::Index::from(*terminal_index);
                let feature = crate::emit::surface_feature(codec.feature());
                quote! {
                    (
                        LexicalOwnerTemplate::DeclarationTerm(#terminal_index),
                        Leaf::DeclarationTerm {
                            terminal_index: #terminal_index,
                            id,
                            ..
                        },
                    ) => Some(LexicalOwner::declaration_owner(
                        id.clone(),
                        #feature,
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
                    id: &::deckmaste_construction_core::macro_def::DeclarationIdentity,
                    feature: ::deckmaste_construction_core::macro_def::SurfaceFeature,
                ) -> String {
                    let kind = match id.kind() {
                        ::deckmaste_construction_core::macro_def::DeclarationKind::KeywordAction => "keyword_action",
                        ::deckmaste_construction_core::macro_def::DeclarationKind::KeywordAbility => "keyword_ability",
                        ::deckmaste_construction_core::macro_def::DeclarationKind::AbilityWord => "ability_word",
                        ::deckmaste_construction_core::macro_def::DeclarationKind::FlavorWord => "flavor_word",
                        ::deckmaste_construction_core::macro_def::DeclarationKind::Subtype(category) => match category {
                            ::deckmaste_construction_core::macro_def::SubtypeCategory::Artifact => "artifact_subtype",
                            ::deckmaste_construction_core::macro_def::SubtypeCategory::Battle => "battle_subtype",
                            ::deckmaste_construction_core::macro_def::SubtypeCategory::Creature => "creature_subtype",
                            ::deckmaste_construction_core::macro_def::SubtypeCategory::Enchantment => "enchantment_subtype",
                            ::deckmaste_construction_core::macro_def::SubtypeCategory::Land => "land_subtype",
                            ::deckmaste_construction_core::macro_def::SubtypeCategory::Planeswalker => "planeswalker_subtype",
                            ::deckmaste_construction_core::macro_def::SubtypeCategory::Spell => "spell_subtype",
                        },
                        ::deckmaste_construction_core::macro_def::DeclarationKind::Type => "type",
                        ::deckmaste_construction_core::macro_def::DeclarationKind::TurnPart => "turn_part",
                        ::deckmaste_construction_core::macro_def::DeclarationKind::CounterKind => "counter_kind",
                        ::deckmaste_construction_core::macro_def::DeclarationKind::Designation => "designation",
                    };
                    let feature = match feature {
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN => "bare",
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => {
                            "third_person_singular"
                        }
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::Singular => "singular",
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::Plural => "plural",
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::PAST_PARTICIPLE => "participle",
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::Inflectional(
                            ::deckmaste_construction_core::macro_def::InflectionalForm::Preterite,
                        ) => "preterite",
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::Inflectional(
                            ::deckmaste_construction_core::macro_def::InflectionalForm::GerundParticiple,
                        ) => "gerund_participle",
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::Fixed => "fixed",
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::BoundSuffix => "bound_suffix",
                        ::deckmaste_construction_core::macro_def::SurfaceFeature::BlockLabel => "block_label",
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
                        id: ::deckmaste_construction_core::macro_def::DeclarationIdentity,
                        feature: ::deckmaste_construction_core::macro_def::SurfaceFeature,
                    ) -> Self {
                        Self {
                            identity: LexicalOwnerIdentity::Declaration(std::sync::Arc::new((
                                id,
                                feature,
                                std::sync::OnceLock::new(),
                            ))),
                        }
                    }

                    #catalog_owner_constructor

                    pub const fn kind(&self) -> LexicalProvenanceKind {
                        match self.identity {
                            LexicalOwnerIdentity::Static { kind, .. } => kind,
                            LexicalOwnerIdentity::Declaration(_) => LexicalProvenanceKind::Lexeme,
                            #catalog_kind_arm
                        }
                    }

                    pub fn stable_id(&self) -> &str {
                        match &self.identity {
                            LexicalOwnerIdentity::Static { stable_id, .. } => stable_id,
                            LexicalOwnerIdentity::Declaration(declaration) => declaration
                                .2
                                .get_or_init(|| declaration_lexeme_owner_id(&declaration.0, declaration.1)),
                            #catalog_stable_id_arm
                        }
                    }

                    pub(crate) fn stable_id_owned(&self) -> String {
                        match &self.identity {
                            LexicalOwnerIdentity::Static { stable_id, .. } => {
                                Self::construct_label(|| stable_id.to_string())
                            }
                            LexicalOwnerIdentity::Declaration(_) => self.stable_id().to_owned(),
                            #catalog_stable_id_owned_arm
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
                            #catalog_eq_arm
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
                                #declaration_ord_arms
                                #catalog_ord_arms
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
                            (LexicalOwnerTemplate::Structural { stable_id, .. }, _) => {
                                Some(LexicalOwner::static_owner(
                                    LexicalProvenanceKind::FormLiteral,
                                    stable_id,
                                ))
                            },
                            (
                                LexicalOwnerTemplate::Static { kind, stable_id },
                                _,
                            ) => Some(LexicalOwner::static_owner(kind, stable_id)),
                            (
                                LexicalOwnerTemplate::TransitionedStatic {
                                    kind,
                                    stable_id,
                                    ..
                                },
                                _,
                            ) => Some(LexicalOwner::static_owner(kind, stable_id)),
                            #(#vocab_owner_arms)*
                            #(#context_owner_arms)*
                            #(#catalog_owner_arms)*
                            #(#verb_lexeme_owner_arms)*
                            #(#noun_lexeme_owner_arms)*
                            (
                                LexicalOwnerTemplate::Declaration { kind, name },
                                Leaf::Declaration(declaration),
                            ) if declaration.id.kind() == kind && declaration.id.name() == name => {
                                Some(LexicalOwner::declaration_owner(
                                    declaration.id.clone(),
                                    declaration.feature,
                                ))
                            },
                            #(#declaration_noun_owner)*
                            #(#declaration_determinative_owner)*
                            #(#declaration_term_owner)*
                            #(#declaration_verb_owner)*
                            _ => None,
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

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::Fields;
    use syn::Item;

    #[test]
    fn generated_root_metadata_and_parse_capability_are_sealed_and_source_ordered() {
        let plan = structural_semantic_plan();
        let runtime = super::emit(&plan);

        let Item::Trait(generated_root) = parse_named(&runtime, "GeneratedRoot") else {
            panic!("GeneratedRoot is a trait");
        };
        assert_eq!(generated_root.ident, "GeneratedRoot");
        assert!(
            matches!(&generated_root.vis, syn::Visibility::Restricted(_)),
            "GeneratedRoot remains sealed inside the generated crate",
        );
        let contract = generated_root.to_token_stream().to_string();
        for fragment in [
            "const CATEGORY : Category",
            "const NAME : & 'static str",
            "const EOI : bool",
            "fn from_build (value : BuildValue) -> Option < Self >",
        ] {
            assert!(
                contract.contains(fragment),
                "missing `{fragment}`: {contract}"
            );
        }
        assert!(
            !contract.contains("render_with_claims"),
            "universal root metadata must not imply standalone rendering: {contract}",
        );

        let Item::Trait(generated_parse_root) = parse_named(&runtime, "GeneratedParseRoot") else {
            panic!("GeneratedParseRoot is a trait");
        };
        assert_eq!(generated_parse_root.ident, "GeneratedParseRoot");
        assert!(
            matches!(&generated_parse_root.vis, syn::Visibility::Restricted(_)),
            "GeneratedParseRoot remains sealed inside the generated crate",
        );
        let parse_contract = generated_parse_root.to_token_stream().to_string();
        for fragment in [
            "GeneratedParseRoot : GeneratedRoot",
            "fn render_with_claims",
            "Vec < RawRenderedClaim >",
        ] {
            assert!(
                parse_contract.contains(fragment),
                "missing `{fragment}`: {parse_contract}",
            );
        }

        let implementations = runtime
            .iter()
            .filter_map(|item| match &item.key {
                crate::ItemKey::Impl {
                    trait_name: Some(trait_name),
                    self_ty,
                } if trait_name == "GeneratedRoot" => {
                    Some((self_ty.as_str(), item.tokens.to_string()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            implementations
                .iter()
                .map(|(self_ty, _)| *self_ty)
                .collect::<Vec<_>>(),
            ["LeftNode", "RightNode"],
        );
        for (self_ty, eoi) in [("LeftNode", "true"), ("RightNode", "false")] {
            let implementation = implementations
                .iter()
                .find_map(|(actual, tokens)| (*actual == self_ty).then_some(tokens))
                .expect("root implementation exists");
            for fragment in [
                format!("const CATEGORY : Category = Category :: {self_ty}"),
                format!("const NAME : & 'static str = \"{self_ty}\""),
                format!("const EOI : bool = {eoi}"),
                format!("BuildValue :: {self_ty} (value , _) => Some (value)"),
                "_ => None".to_owned(),
            ] {
                assert!(
                    implementation.contains(&fragment),
                    "{self_ty} root metadata lacks `{fragment}`: {implementation}",
                );
            }
            assert!(
                !implementation.contains("render_with_claims"),
                "{self_ty} metadata impl must not imply parse-root capability: {implementation}",
            );
        }

        let parse_implementations = runtime
            .iter()
            .filter_map(|item| match &item.key {
                crate::ItemKey::Impl {
                    trait_name: Some(trait_name),
                    self_ty,
                } if trait_name == "GeneratedParseRoot" => {
                    Some((self_ty.as_str(), item.tokens.to_string()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            parse_implementations
                .iter()
                .map(|(self_ty, _)| *self_ty)
                .collect::<Vec<_>>(),
            ["LeftNode"],
            "only source-ordered standalone roots are selectable parse roots",
        );
        assert!(
            parse_implementations[0]
                .1
                .contains("render_left_node_with_claims (self , context"),
            "the selectable root owns standalone claim rendering",
        );
    }

    #[test]
    fn structural_build_and_diagnostic_inventories_are_generated_from_one_plan() {
        let plan = structural_semantic_plan();
        let runtime = super::emit(&plan);

        let Item::Enum(build_value) = parse_named(&runtime, "BuildValue") else {
            panic!("BuildValue is an enum");
        };
        let variants = build_value
            .variants
            .iter()
            .map(|variant| {
                let Fields::Unnamed(fields) = &variant.fields else {
                    panic!("BuildValue variants are tuple carriers");
                };
                (
                    variant.ident.to_string(),
                    fields
                        .unnamed
                        .iter()
                        .map(|field| field.ty.to_token_stream().to_string())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            variants,
            [
                (
                    "Choice".to_owned(),
                    vec![
                        "Choice".to_owned(),
                        "FeatureConstraint < Onset >".to_owned()
                    ]
                ),
                ("Holder".to_owned(), vec!["Holder".to_owned()]),
                (
                    "RecursiveChoice".to_owned(),
                    vec![
                        "RecursiveChoice".to_owned(),
                        "FeatureConstraint < Onset >".to_owned()
                    ],
                ),
                (
                    "RecursiveBranch".to_owned(),
                    vec!["RecursiveBranch".to_owned()],
                ),
                (
                    "LeftNode".to_owned(),
                    vec![
                        "LeftNode".to_owned(),
                        "FeatureConstraint < Onset >".to_owned()
                    ]
                ),
                (
                    "RightNode".to_owned(),
                    vec![
                        "RightNode".to_owned(),
                        "FeatureConstraint < Onset >".to_owned()
                    ]
                ),
                (
                    "HolderMaybeOptional".to_owned(),
                    vec!["Option < LeftNode >".to_owned()],
                ),
                (
                    "HolderItemsSequence".to_owned(),
                    vec!["Vec < Choice >".to_owned()],
                ),
                (
                    "RecursiveBranchChoicesSequence".to_owned(),
                    vec!["Vec < RecursiveChoice >".to_owned()],
                ),
                (
                    "LeftValueMaybeOptional".to_owned(),
                    vec!["Option < RightNode >".to_owned()],
                ),
                ("Leaf".to_owned(), vec!["Leaf".to_owned()]),
            ],
            "the generated carrier inventory is complete, ordered, and uses raw helper payloads",
        );

        let Item::Enum(nonterminal) = parse_named(&runtime, "NonterminalCategory") else {
            panic!("NonterminalCategory is an enum");
        };
        assert_eq!(
            nonterminal
                .variants
                .iter()
                .map(|variant| variant.ident.to_string())
                .collect::<Vec<_>>(),
            [
                "Choice",
                "Holder",
                "RecursiveChoice",
                "RecursiveBranch",
                "LeftNode",
                "RightNode"
            ]
        );
        assert!(nonterminal.variants.iter().all(|variant| {
            !variant.ident.to_string().contains("Optional")
                && !variant.ident.to_string().contains("Sequence")
        }));

        let category_impl = runtime
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Impl { trait_name: Some(name), self_ty } if name == "Category" && self_ty == "Category"))
            .expect("generated exhaustive Category mapping");
        let conversion = category_impl.tokens.to_string();
        for mapping in [
            "Category :: Choice => NonterminalCategory :: Choice",
            "Category :: Holder => NonterminalCategory :: Holder",
            "Category :: RecursiveChoice => NonterminalCategory :: RecursiveChoice",
            "Category :: RecursiveBranch => NonterminalCategory :: RecursiveBranch",
            "Category :: LeftNode => NonterminalCategory :: LeftNode",
            "Category :: RightNode => NonterminalCategory :: RightNode",
            "Category :: HolderMaybeOptionalCategory => NonterminalCategory :: Holder",
            "Category :: HolderItemsSequenceCategory => NonterminalCategory :: Holder",
            "Category :: HolderItemsSequenceCount2Category => NonterminalCategory :: Holder",
            "Category :: HolderItemsSequenceCount3Category => NonterminalCategory :: Holder",
            "Category :: HolderItemsSequenceCount4Category => NonterminalCategory :: Holder",
            "Category :: RecursiveBranchChoicesSequenceCategory => NonterminalCategory :: RecursiveBranch",
            "Category :: LeftValueMaybeOptionalCategory => NonterminalCategory :: LeftNode",
        ] {
            assert!(
                conversion.contains(mapping),
                "missing `{mapping}`: {conversion}"
            );
        }

        let label_impl = runtime
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Impl { trait_name: Some(name), self_ty } if name == "NonterminalCategory" && self_ty == "NonterminalCategory"))
            .expect("generated diagnostic labels")
            .tokens
            .to_string();
        assert!(
            label_impl.contains("Self :: RecursiveChoice => \"recursive choice\""),
            "labels derive from the same public inventory: {label_impl}",
        );
        assert!(runtime.iter().any(|item| {
            matches!(&item.key, crate::ItemKey::Impl { trait_name: Some(name), self_ty } if name == "From<Category>" && self_ty == "NonterminalCategory")
        }));
    }

    #[test]
    fn vocab_surface_rows_carry_effective_homograph_licenses() {
        let plan = crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                vocab AttributiveAdjective {
                    feature HomographLicense = Unlicensed;
                    Plain = "plain",
                    Target = "target" { feature HomographLicense = Licensed; },
                }
                vocab Preposition { During = "during", }
                construction phrase: Phrase {
                    element PhraseNode {
                        adjective: lex AttributiveAdjective,
                        preposition: lex Preposition,
                    }
                    form phrase = lex(adjective) licensed(" ") lex(preposition);
                }
                root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("homograph metadata fixture parses"),
        )
        .expect("homograph metadata fixture validates")
        .into_semantic();
        let runtime = super::emit(&plan);
        let rows = runtime
            .iter()
            .find(|item| {
                matches!(&item.key, crate::ItemKey::Named { name, .. } if name == "VOCAB_SURFACES")
            })
            .expect("vocab surface rows are emitted")
            .tokens
            .to_string();

        for fragment in [
            "surface : \"plain\" , vocabulary : \"AttributiveAdjective\" , member : \"Plain\" , homograph_license : HomographLicense :: Unlicensed",
            "surface : \"target\" , vocabulary : \"AttributiveAdjective\" , member : \"Target\" , homograph_license : HomographLicense :: Licensed",
            "surface : \"during\" , vocabulary : \"Preposition\" , member : \"During\" , homograph_license : HomographLicense :: Unlicensed",
        ] {
            assert!(rows.contains(fragment), "missing `{fragment}`: {rows}");
        }

        let form_rows = runtime
            .iter()
            .find(|item| {
                matches!(&item.key, crate::ItemKey::Named { name, .. } if name == "FORM_LITERAL_SURFACES")
            })
            .expect("form literal surface rows are emitted")
            .tokens
            .to_string();
        assert!(
            form_rows.contains("surface : \" \" , construction : \"phrase\" , form : \"phrase\" , atom_index : 1usize , homograph_license : HomographLicense :: Licensed"),
            "licensed form atom metadata is missing: {form_rows}",
        );
    }

    fn structural_semantic_plan() -> crate::semantic::SemanticPlan {
        crate::validate_declarations(
            crate::parse_declarations(quote::quote! {
                abstract sum Choice { Left: LeftNode, Right: RightNode, }
                abstract product Holder {
                    maybe: opt LeftNode,
                    items: seq Choice terminated by ",",
                }
                require len(Holder.items) >= 1;
                require len(Holder.items) <= 4;
                abstract sum RecursiveChoice { Branch: RecursiveBranch, }
                abstract product RecursiveBranch {
                    choices: seq RecursiveChoice terminated by ".",
                }
                require len(RecursiveBranch.choices) = 1;
                construction left: LeftNode {
                    element LeftValue { maybe: opt RightNode, }
                    form left = maybe "left";
                }
                construction right: RightNode {
                    element RightValue {}
                    form right = "right";
                }
                root LeftNode { punctuation = "."; eoi = true; standalone_render = true; }
                root RightNode { punctuation = "."; eoi = false; standalone_render = false; }
            })
            .expect("structural runtime fixture parses"),
        )
        .expect("structural runtime fixture validates")
        .into_semantic()
    }

    fn parse_named(items: &[crate::GeneratedItem], name: &str) -> Item {
        let item = items
            .iter()
            .find(|item| matches!(&item.key, crate::ItemKey::Named { name: actual, .. } if actual == name))
            .unwrap_or_else(|| panic!("missing generated item {name}"));
        syn::parse2(item.tokens.clone()).expect("generated tokens parse as one item")
    }
}
