//! Chart assembly for generated construction groups: category allocation,
//! atom-to-production mapping, and rule registration. Production assemblies
//! activate production coordination by default; test assemblies can still
//! select isolated groups through [`GeneratedActivation::Groups`].

use std::collections::BTreeMap;

use deckmaste_construction_compiler::runtime::AtomData;
use deckmaste_construction_compiler::runtime::ConstructionData;
use deckmaste_construction_compiler::runtime::ElementData;
use deckmaste_construction_compiler::runtime::EvidenceSourceData;
use deckmaste_construction_compiler::runtime::FieldKindData;
use deckmaste_construction_compiler::runtime::GroupData;
use deckmaste_construction_compiler::runtime::PredicateData;

use super::EnglishLexicalSlot;
use super::Expected;
use super::Nonterminal;
use super::rules::GeneratedAuxRuleRef;
use super::rules::GeneratedRuleContext;
use super::rules::GeneratedRuleRef;
use super::rules::RuleBuilder;
use super::rules::RuleImpl;
use crate::construction::ConstructionId;
use crate::construction::ProductionId;
use crate::surface::Punctuation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GeneratedFeatureCombinator {
    CompleteSentence,
    CompleteNounPhraseCoordination,
    SharedDeterminerCoordination,
}

impl GeneratedFeatureCombinator {
    pub(super) fn from_name(name: &str) -> Option<Self> {
        match name {
            "complete_sentence" => Some(Self::CompleteSentence),
            "complete_noun_phrase_coordination" => Some(Self::CompleteNounPhraseCoordination),
            "shared_determiner_coordination" => Some(Self::SharedDeterminerCoordination),
            _ => None,
        }
    }

    pub(super) fn from_construction(construction: &ConstructionData) -> Option<Self> {
        construction
            .feature_combinators
            .iter()
            .find_map(|feature| Self::from_name(feature.combinator))
    }

    const fn admits_shared_preposition(self) -> bool {
        matches!(
            self,
            Self::CompleteNounPhraseCoordination | Self::SharedDeterminerCoordination
        )
    }

    fn base_cost(self) -> super::ParseCost {
        match self {
            Self::CompleteNounPhraseCoordination => super::ParseCost {
                precedence: 1,
                ..super::ParseCost::default()
            },
            Self::CompleteSentence | Self::SharedDeterminerCoordination => {
                super::ParseCost::default()
            }
        }
    }

    const fn first_member_argument(self) -> usize {
        match self {
            Self::CompleteSentence | Self::CompleteNounPhraseCoordination => 0,
            Self::SharedDeterminerCoordination => 1,
        }
    }

    const fn rest_argument(self) -> usize {
        match self {
            Self::CompleteSentence => 0,
            Self::CompleteNounPhraseCoordination => 1,
            Self::SharedDeterminerCoordination => 2,
        }
    }

    const fn complements_argument(self) -> Option<usize> {
        match self {
            Self::CompleteSentence | Self::CompleteNounPhraseCoordination => None,
            Self::SharedDeterminerCoordination => Some(3),
        }
    }

    pub(super) fn argument_field_index(
        self,
        construction: &ConstructionData,
        argument: usize,
    ) -> Option<usize> {
        let feature = construction
            .feature_combinators
            .iter()
            .find(|feature| Self::from_name(feature.combinator) == Some(self))?;
        let name = feature.args.get(argument)?;
        construction
            .fields
            .iter()
            .position(|field| field.name == *name)
    }

    pub(super) fn first_member_field_index(self, construction: &ConstructionData) -> Option<usize> {
        self.argument_field_index(construction, self.first_member_argument())
    }

    pub(super) fn rest_field_index(self, construction: &ConstructionData) -> Option<usize> {
        self.argument_field_index(construction, self.rest_argument())
    }

    pub(super) fn complements_field_index(self, construction: &ConstructionData) -> Option<usize> {
        self.argument_field_index(construction, self.complements_argument()?)
    }

    pub(super) fn sequence_argument_element(
        self,
        construction: &ConstructionData,
        argument: usize,
    ) -> Option<&'static str> {
        let field = construction
            .fields
            .get(self.argument_field_index(construction, argument)?)?;
        let FieldKindData::Sequence { element } = field.kind else {
            return None;
        };
        Some(element)
    }

    pub(super) fn member_element(self, construction: &ConstructionData) -> Option<&'static str> {
        self.sequence_argument_element(construction, self.rest_argument())
    }

    pub(super) fn member_value_field_index(
        self,
        group: &GroupData,
        construction: &ConstructionData,
    ) -> Option<usize> {
        let first = construction
            .fields
            .get(self.first_member_field_index(construction)?)?;
        let FieldKindData::Subtree {
            category: member_category,
            ..
        } = first.kind
        else {
            return None;
        };
        let member_element = self.member_element(construction)?;
        let element = group
            .element_data
            .iter()
            .find(|element| element.name == member_element)?;
        element.fields.iter().position(|field| {
            matches!(
                field.kind,
                FieldKindData::Subtree { category, .. } if category == member_category
            )
        })
    }
}

pub(super) fn coordination_member_role<'a>(
    group: &'a GroupData,
    element: &str,
) -> Option<(GeneratedFeatureCombinator, &'a ConstructionData)> {
    group.constructions.iter().find_map(|construction| {
        let combinator = GeneratedFeatureCombinator::from_construction(construction)?;
        (combinator.member_element(construction)? == element).then_some((combinator, construction))
    })
}

pub(super) fn coordination_delimiter_fields(
    _group: &GroupData,
    element: &ElementData,
) -> Option<(usize, usize)> {
    let codec = |kind| match kind {
        FieldKindData::Scalar { codec }
        | FieldKindData::SurfaceScalar { codec }
        | FieldKindData::Optional {
            inner: &FieldKindData::Scalar { codec } | &FieldKindData::SurfaceScalar { codec },
        } => Some(codec),
        _ => None,
    };
    let comma = element
        .fields
        .iter()
        .position(|field| codec(field.kind) == Some("Comma"))?;
    let conjunction = element.fields.iter().position(|field| {
        matches!(
            codec(field.kind),
            Some("Conjunction" | "NounPhraseConjunction")
        )
    })?;
    Some((comma, conjunction))
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GeneratedActivation {
    /// The production constructicon.
    Production,
    /// Test-only handwritten control with no generated group active.
    #[cfg(test)]
    Inactive,
    /// Test assemblies only: register exactly these groups, in slice order.
    #[cfg(test)]
    Groups(&'static [&'static GroupData]),
}

impl GeneratedActivation {
    #[allow(
        clippy::unnecessary_wraps,
        reason = "the test-only Inactive variant returns None; production builds see only the Some arm"
    )]
    pub(super) fn groups(self) -> Option<&'static [&'static GroupData]> {
        match self {
            Self::Production => Some(crate::constructions::GROUPS),
            #[cfg(test)]
            Self::Inactive => None,
            #[cfg(test)]
            Self::Groups(groups) => Some(groups),
        }
    }

    pub(crate) const fn is_production(self) -> bool {
        matches!(self, Self::Production)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum GeneratedAssemblyError {
    /// A non-internal category with no explicit English engine mapping.
    UnknownCategory {
        construction: &'static str,
        category: &'static str,
    },
    /// A lexeme codec without a closed engine slot.
    UnknownLexemeCodec {
        construction: &'static str,
        codec: &'static str,
    },
    /// A lexical identity whose Rust value type and provider have no exact
    /// English recognition/lowering adapter.
    UnsupportedIdentityAdapter {
        owner: &'static str,
        field: &'static str,
        value_type: &'static str,
        provider: &'static str,
    },
    /// Optional identities have no present/absent lowering adapter yet.
    UnsupportedOptionalIdentityAdapter {
        owner: &'static str,
        field: &'static str,
        value_type: &'static str,
        provider: &'static str,
    },
    /// A typed scalar whose Rust value type and surface codec have no exact
    /// English lowering adapter.
    UnsupportedTypedScalar {
        owner: &'static str,
        field: &'static str,
        value_type: &'static str,
        codec: &'static str,
    },
    /// Optional typed scalars have no present/absent lowering adapter yet.
    UnsupportedOptionalTypedScalar {
        owner: &'static str,
        field: &'static str,
        value_type: &'static str,
        codec: &'static str,
    },
    /// Multi-segment atom paths have no chart meaning yet.
    UnsupportedAtomPath {
        construction: &'static str,
        path: &'static str,
    },
    /// An atom naming a field this construction does not declare.
    UnknownAtomField {
        construction: &'static str,
        field: &'static str,
    },
    /// A `Hole` atom on a field shape other than `Subtree` or `Sequence`, OR
    /// a `Lexeme` atom on a field
    /// shape other than `Scalar` — including `Optional { Scalar }`, e.g.
    /// `lex(opt field)`, which `validate.rs`'s `resolved_is_scalar` admits
    /// as a legal, renderable EC014 declaration but which has no production
    /// meaning here. Fires for either atom kind, hence the name — the
    /// field's SHAPE, not the atom's declared legality, is what's
    /// unsupported.
    UnsupportedAtomKind {
        construction: &'static str,
        field: &'static str,
    },
    UnknownElement {
        construction: &'static str,
        element: &'static str,
    },
    TooManyOptionalAtoms {
        owner: &'static str,
    },
    /// Registering a sequence over an element with an empty surface would
    /// create a nullable left-recursive extension whose feature length grows
    /// without consuming input.
    NullableElement {
        group: &'static str,
        element: &'static str,
    },
    /// Evidence metadata must name a semantic source whose English feature
    /// adapter is total. Reject unsupported paths while assembling the active
    /// constructicon instead of emitting a label with no concrete value.
    UnsupportedEvidenceSource {
        construction: &'static str,
        source: EvidenceSourceData,
    },
    /// A reserved contextual output has no exact closed adapter for its
    /// declaration group and source category.
    UnsupportedContextAdapter {
        group: &'static str,
        category: &'static str,
        output: &'static str,
    },
    /// A contextual adapter names a field role that none of the declarations
    /// producing that output expose.
    MissingContextFieldRole {
        group: &'static str,
        category: &'static str,
        output: &'static str,
        role: &'static str,
    },
    /// A contextual role exists but its exact subtree/identity type does not
    /// satisfy the adapter contract.
    InvalidContextFieldRole {
        group: &'static str,
        category: &'static str,
        output: &'static str,
        role: &'static str,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextFieldSource {
    Subtree(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ContextFieldSubstitution {
    role: &'static str,
    source: ContextFieldSource,
    replacement: Expected<Nonterminal, EnglishLexicalSlot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ContextAdapter {
    group: &'static str,
    source_category: &'static str,
    target_output: &'static str,
    context: GeneratedRuleContext,
    target_category: Nonterminal,
    substitutions: &'static [ContextFieldSubstitution],
}

const OBJECT_GAP_CONTEXT_FIELDS: &[ContextFieldSubstitution] = &[ContextFieldSubstitution {
    role: "predicate",
    source: ContextFieldSource::Subtree("VerbPhrase"),
    replacement: Expected::Nonterminal(Nonterminal::ObjectGapVerbPhrase),
}];

const OBJECT_GAP_CONTEXT_ADAPTER: ContextAdapter = ContextAdapter {
    group: "predicate",
    source_category: "VerbPhrase",
    target_output: "object_gap",
    context: GeneratedRuleContext::ObjectGap,
    target_category: Nonterminal::ObjectGapVerbPhrase,
    substitutions: OBJECT_GAP_CONTEXT_FIELDS,
};

const REDUCED_PASSIVE_CONTEXT_FIELDS: &[ContextFieldSubstitution] = &[
    ContextFieldSubstitution {
        role: "predicate",
        source: ContextFieldSource::Subtree("VerbPhrase"),
        replacement: Expected::Nonterminal(Nonterminal::ReducedRecipientPassive),
    },
    ContextFieldSubstitution {
        role: "head",
        source: ContextFieldSource::Subtree("Verb"),
        replacement: Expected::Lexical(EnglishLexicalSlot::ReducedRecipientPassiveParticiple),
    },
    ContextFieldSubstitution {
        role: "object",
        source: ContextFieldSource::Subtree("NounPhrase"),
        replacement: Expected::Nonterminal(Nonterminal::ReducedRecipientPassiveTheme),
    },
];

const REDUCED_PASSIVE_CONTEXT_ADAPTER: ContextAdapter = ContextAdapter {
    group: "predicate",
    source_category: "VerbPhrase",
    target_output: "reduced_passive",
    context: GeneratedRuleContext::ReducedRecipientPassive,
    target_category: Nonterminal::ReducedRecipientPassive,
    substitutions: REDUCED_PASSIVE_CONTEXT_FIELDS,
};

const CONTEXT_ADAPTERS: &[ContextAdapter] =
    &[OBJECT_GAP_CONTEXT_ADAPTER, REDUCED_PASSIVE_CONTEXT_ADAPTER];

/// Deterministic internal-category ids: the sorted set of `internal`
/// constructions' category names across the active groups, in order. Shared
/// categories collapse to one id; ids never depend on group or registration
/// order.
pub(super) fn internal_categories(groups: &[&'static GroupData]) -> BTreeMap<&'static str, u16> {
    let mut names = BTreeMap::new();
    for group in groups {
        for construction in group.constructions {
            if construction.internal {
                names.insert(construction.category, 0_u16);
            }
        }
    }
    for (index, (_, id)) in names.iter_mut().enumerate() {
        *id = u16::try_from(index).expect("more than u16::MAX internal categories");
    }
    names
}

/// Non-internal declaration categories map explicitly onto English chart
/// categories. Payload-specific conversion remains a lowering concern.
fn engine_category(name: &str) -> Option<Nonterminal> {
    Some(match name {
        "Verb" => Nonterminal::Verb,
        "VerbPhrase" => Nonterminal::VerbPhrase,
        "Adjective" => Nonterminal::Adjective,
        "Noun" | "NounInstance" => Nonterminal::Noun,
        "NounPhrase" => Nonterminal::NounPhrase,
        "NominalPhrase" => Nonterminal::Nominal,
        "Determiner" => Nonterminal::Determiner,
        "AdjectivePhrase" | "ComparisonAdjectivePhrase" => Nonterminal::AdjectivePhrase,
        "CoordinatedAdjectivePhrase" => Nonterminal::CoordinatedModifier,
        "PrepositionalPhrase" => Nonterminal::PrepositionalPhrase,
        "InfinitiveClause" => Nonterminal::InfinitiveClause,
        "FrequencyPhrase" => Nonterminal::FrequencyPhrase,
        "RelativeClause" => Nonterminal::RelativeClause,
        "TransitivePredicate" => Nonterminal::ReducedRecipientPassive,
        "ReducedRecipientPassiveTheme" => Nonterminal::ReducedRecipientPassiveTheme,
        "Quantity" => Nonterminal::Quantity,
        "DevotionColors" => Nonterminal::DevotionColors,
        "PowerToughness" => Nonterminal::PowerToughness,
        "ManaAmount" => Nonterminal::ManaAmount,
        "ManaAmountList" => Nonterminal::ManaAmountList,
        "CoordinatedManaAmount" => Nonterminal::CoordinatedManaAmount,
        "IndependentClause" | "Clause" => Nonterminal::Clause,
        "ComparisonComplement" => Nonterminal::ComparisonComplement,
        "RulesObjectNominal" => Nonterminal::RulesObjectNominal,
        "RulesObjectFollowupNominal" => Nonterminal::RulesObjectFollowupNominal,
        "PredicatedQualityFrom" => Nonterminal::PredicatedQualityFrom,
        "PredicatedArgumentFrom" => Nonterminal::PredicatedArgumentFrom,
        "PredicatedQualityBare" => Nonterminal::PredicatedQualityBare,
        "PredicatedArgumentBare" | "KeywordArgument" => Nonterminal::PredicatedArgumentBare,
        "Sentence" => Nonterminal::Sentence,
        _ => return None,
    })
}

/// Runs a declaration-emitted typed feature callback without coupling the
/// chart adapter to one nominal declaration address. The outer option says
/// whether this group owns the requested output; the inner option is the
/// callback's semantic admission result.
#[allow(
    clippy::option_option,
    reason = "the outer option is adapter ownership and the inner option is semantic admission"
)]
pub(super) fn typed_feature_projection(
    group: &GroupData,
    construction: usize,
    target: &str,
    fields: &[Option<&super::Features>],
) -> Option<Option<super::Features>> {
    let construction_data = group.constructions.get(construction)?;
    if !construction_data
        .feature_combinators
        .iter()
        .any(|feature| feature.target == target)
    {
        return None;
    }
    match (group.name, target) {
        ("nominal", "features") => Some(crate::constructions::nominal::reduce_nominal_features(
            construction,
            fields,
        )),
        ("nominal", "precedence") => Some(
            crate::constructions::nominal::reduce_nominal_precedence(construction, fields),
        ),
        ("nominal", "prefix_admission") => Some(
            crate::constructions::nominal::reduce_nominal_prefix_admission(construction, fields),
        ),
        ("nominal", "attachment_distance") => Some(
            crate::constructions::nominal::reduce_nominal_attachment_distance(construction, fields),
        ),
        ("nominal", "attachment_extent") => Some(
            crate::constructions::nominal::reduce_nominal_attachment_extent(construction, fields),
        ),
        ("predicate", "features") => Some(
            crate::constructions::predicate::reduce_predicate_features(construction, fields),
        ),
        ("predicate", "object_gap") => {
            Some(crate::constructions::predicate::reduce_predicate_object_gap(construction, fields))
        }
        ("predicate", "reduced_passive") => Some(
            crate::constructions::predicate::reduce_predicate_reduced_passive(construction, fields),
        ),
        ("predicate", "argument_complete") => Some(
            crate::constructions::predicate::reduce_predicate_argument_complete(
                construction,
                fields,
            ),
        ),
        ("predicate", "precedence") => {
            Some(crate::constructions::predicate::reduce_predicate_precedence(construction, fields))
        }
        ("predicate", "prefix_admission") => Some(
            crate::constructions::predicate::reduce_predicate_prefix_admission(
                construction,
                fields,
            ),
        ),
        ("adjective", "features") => Some(
            crate::constructions::adjective::reduce_adjective_features(construction, fields),
        ),
        _ => None,
    }
}

#[cfg(test)]
pub(crate) fn declared_category_nonterminal(name: &str) -> Option<Nonterminal> {
    engine_category(name)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum AuxCategoryKind {
    Element,
    Sequence,
}

type AuxCategories = BTreeMap<(&'static str, &'static str, AuxCategoryKind), u16>;

fn auxiliary_categories(groups: &[&'static GroupData], internal_count: usize) -> AuxCategories {
    let mut result = BTreeMap::new();
    for group in groups {
        for element in group.element_data {
            for kind in [AuxCategoryKind::Element, AuxCategoryKind::Sequence] {
                result.insert((group.name, element.name, kind), 0);
            }
        }
    }
    for (offset, id) in result.values_mut().enumerate() {
        *id = u16::try_from(internal_count + offset).expect("generated categories exceed u16::MAX");
    }
    result
}

/// Resolves a category referenced from a `Hole` atom's `Subtree` field. Such
/// a reference names a category, not a specific construction, so it may
/// legitimately land on any active group's internal category or (once the
/// table grows) an engine nonterminal.
fn category_nonterminal(
    cats: &BTreeMap<&'static str, u16>,
    construction: &'static ConstructionData,
    category: &'static str,
) -> Result<Nonterminal, GeneratedAssemblyError> {
    if let Some(&id) = cats.get(category) {
        return Ok(Nonterminal::Generated(id));
    }
    engine_category(category).ok_or(GeneratedAssemblyError::UnknownCategory {
        construction: construction.id,
        category,
    })
}

/// Resolves a construction's own left-hand-side category. Membership is
/// decided by the construction's OWN `internal` flag, never by whether some
/// other construction happens to declare the same category name as internal
/// — otherwise a non-internal construction sharing a category name with an
/// internal sibling would be silently admitted as `Nonterminal::Generated`
/// instead of surfacing `UnknownCategory`.
fn lhs_category_nonterminal(
    cats: &BTreeMap<&'static str, u16>,
    construction: &'static ConstructionData,
) -> Result<Nonterminal, GeneratedAssemblyError> {
    if construction.internal {
        let id = *cats.get(construction.category).expect(
            "internal_categories collected this construction's category from the same group set",
        );
        return Ok(Nonterminal::Generated(id));
    }
    engine_category(construction.category).ok_or(GeneratedAssemblyError::UnknownCategory {
        construction: construction.id,
        category: construction.category,
    })
}

fn codec_slot(codec: &'static str) -> Option<EnglishLexicalSlot> {
    match codec {
        "Numeral" => Some(EnglishLexicalSlot::QuantityNumber),
        "ComparativeWord" => Some(EnglishLexicalSlot::ComparativeWord),
        "Conjunction" => Some(EnglishLexicalSlot::Conjunction),
        "NounPhraseConjunction" => Some(EnglishLexicalSlot::NounPhraseConjunction),
        "Comma" => Some(EnglishLexicalSlot::Punctuation(Punctuation::Comma)),
        "ColorWord" => Some(EnglishLexicalSlot::ColorWord),
        "Preposition" => Some(EnglishLexicalSlot::Preposition),
        "OracleSymbol" => Some(EnglishLexicalSlot::OracleSymbol),
        "SymbolSequence" => Some(EnglishLexicalSlot::SymbolSequence),
        _ => None,
    }
}

fn typed_scalar_slot(
    owner: &'static str,
    field: &'static str,
    value_type: &'static str,
    codec: &'static str,
    optional: bool,
) -> Result<EnglishLexicalSlot, GeneratedAssemblyError> {
    if optional {
        return Err(GeneratedAssemblyError::UnsupportedOptionalTypedScalar {
            owner,
            field,
            value_type,
            codec,
        });
    }
    match (value_type, codec) {
        ("NumberLiteral" | "QuantityValue", "Numeral") => Ok(EnglishLexicalSlot::QuantityNumber),
        ("PowerToughness", "PowerToughness") => Ok(EnglishLexicalSlot::PowerToughness),
        _ => Err(GeneratedAssemblyError::UnsupportedTypedScalar {
            owner,
            field,
            value_type,
            codec,
        }),
    }
}

fn identity_slot(
    owner: &'static str,
    field: &'static str,
    value_type: &'static str,
    provider: &'static str,
    optional: bool,
) -> Result<EnglishLexicalSlot, GeneratedAssemblyError> {
    if optional {
        return Err(GeneratedAssemblyError::UnsupportedOptionalIdentityAdapter {
            owner,
            field,
            value_type,
            provider,
        });
    }
    match (value_type, provider) {
        ("VerbAnalysis", "LexicalVerb") => Ok(EnglishLexicalSlot::AnyVerb),
        ("Adjective", "Adjective") => Ok(EnglishLexicalSlot::Adjective),
        ("AuxiliaryInstance", "Auxiliary") => Ok(EnglishLexicalSlot::Auxiliary),
        ("Vocab", "Adverb") => Ok(EnglishLexicalSlot::Adverb),
        ("PreverbModifier", "PreverbAdverb") => Ok(EnglishLexicalSlot::PreverbAdverb),
        ("VerbParticle", "VerbParticle") => Ok(EnglishLexicalSlot::AnyVerbParticle),
        ("CoinSide", "CoinResult") => Ok(EnglishLexicalSlot::AnyCoinResult),
        ("FrequencyPhrase", "Frequency") => Ok(EnglishLexicalSlot::Frequency),
        ("CatalogAtom", "AbilityItem") => Ok(EnglishLexicalSlot::AbilityItem),
        ("QuotedAbility", "QuotedAbility") => Ok(EnglishLexicalSlot::QuotedAbility),
        ("OracleSymbol", "OracleSymbol") => Ok(EnglishLexicalSlot::OracleSymbol),
        ("SymbolSequence", "SymbolSequence") => Ok(EnglishLexicalSlot::SymbolSequence),
        ("NounInstance", "KnownNoun") => {
            Ok(EnglishLexicalSlot::Noun(crate::word::NounUsage::Either))
        }
        ("NounInstance", "OpaqueNoun") => Ok(EnglishLexicalSlot::OpaqueNoun),
        ("NounInstance", "CombatStepParticipants") => {
            Ok(EnglishLexicalSlot::CombatStepParticipants)
        }
        ("NounInstance", "CombatStepHead") => Ok(EnglishLexicalSlot::CombatStepHead),
        ("NominalModifier", "NegatedModifier") => Ok(EnglishLexicalSlot::NegatedModifier),
        ("NounInstance", "SymbolArgumentKeywordNoun") => {
            Ok(EnglishLexicalSlot::SymbolArgumentKeywordNoun)
        }
        ("NounInstance", "ExplicitPredicatedKeywordNoun") => {
            Ok(EnglishLexicalSlot::ExplicitPredicatedKeywordNoun)
        }
        ("NounInstance", "AtomCarriedPredicatedKeywordNoun") => {
            Ok(EnglishLexicalSlot::AtomCarriedPredicatedKeywordNoun)
        }
        ("CatalogAtom", "DevotionValue") => Ok(EnglishLexicalSlot::DevotionValue),
        ("NounInstance", "TimesNoun") => Ok(EnglishLexicalSlot::TimesNoun),
        _ => Err(GeneratedAssemblyError::UnsupportedIdentityAdapter {
            owner,
            field,
            value_type,
            provider,
        }),
    }
}

fn validate_field_kind(
    owner: &'static str,
    field: &'static str,
    kind: FieldKindData,
) -> Result<(), GeneratedAssemblyError> {
    match kind {
        FieldKindData::TypedScalar { value_type, codec } => {
            typed_scalar_slot(owner, field, value_type, codec, false).map(drop)
        }
        FieldKindData::Optional {
            inner: &FieldKindData::TypedScalar { value_type, codec },
        } => typed_scalar_slot(owner, field, value_type, codec, true).map(drop),
        FieldKindData::Identity {
            value_type,
            provider,
        } => identity_slot(owner, field, value_type, provider, false).map(drop),
        FieldKindData::Optional {
            inner:
                &FieldKindData::Identity {
                    value_type,
                    provider,
                },
        } => identity_slot(owner, field, value_type, provider, true).map(drop),
        FieldKindData::Subtree { .. }
        | FieldKindData::Scalar { .. }
        | FieldKindData::SurfaceScalar { .. }
        | FieldKindData::Sequence { .. }
        | FieldKindData::Optional { .. } => Ok(()),
    }
}

fn validate_field_adapters(groups: &[&'static GroupData]) -> Result<(), GeneratedAssemblyError> {
    for group in groups {
        for construction in group.constructions {
            for field in construction.fields {
                validate_field_kind(construction.id, field.name, field.kind)?;
            }
        }
        for element in group.element_data {
            for field in element.fields {
                validate_field_kind(element.name, field.name, field.kind)?;
            }
            for variant in element.variants {
                validate_field_kind(element.name, variant.name, variant.payload)?;
            }
        }
    }
    Ok(())
}

fn evidence_requirement_is_supported(construction: &ConstructionData, path: &str) -> bool {
    let conjunction_field = construction.fields.iter().any(|field| {
        field.name == path
            && matches!(
                field.kind,
                FieldKindData::Scalar {
                    codec: "Conjunction" | "NounPhraseConjunction"
                }
            )
    });
    conjunction_field
        && construction.requirements.iter().any(|requirement| {
            matches!(
                requirement.predicate,
                PredicateData::In {
                    path: candidate,
                    ..
                } if candidate == path
            )
        })
}

fn evidence_source_is_supported(
    construction: &ConstructionData,
    source: EvidenceSourceData,
) -> bool {
    match source {
        EvidenceSourceData::Requirement(path) => {
            evidence_requirement_is_supported(construction, path)
        }
        EvidenceSourceData::Output("attachment") => construction.category == "NominalPhrase",
        EvidenceSourceData::Field("predicate.frame") => construction.fields.iter().any(|field| {
            field.name == "predicate"
                && matches!(
                    field.kind,
                    FieldKindData::Subtree {
                        category: "TransitivePredicate",
                        ..
                    }
                )
        }),
        EvidenceSourceData::Category => true,
        EvidenceSourceData::Output(_) | EvidenceSourceData::Field(_) => false,
    }
}

fn validate_evidence_sources(groups: &[&'static GroupData]) -> Result<(), GeneratedAssemblyError> {
    for construction in groups.iter().flat_map(|group| group.constructions.iter()) {
        if let Some(evidence) = construction.evidence
            && !evidence_source_is_supported(construction, evidence.source)
        {
            return Err(GeneratedAssemblyError::UnsupportedEvidenceSource {
                construction: construction.id,
                source: evidence.source,
            });
        }
    }
    Ok(())
}

fn is_reserved_context_output(output: &str) -> bool {
    CONTEXT_ADAPTERS
        .iter()
        .any(|adapter| adapter.target_output == output)
}

fn construction_has_output(construction: &ConstructionData, output: &str) -> bool {
    construction
        .feature_combinators
        .iter()
        .any(|feature| feature.target == output)
}

fn context_field_matches(kind: FieldKindData, source: ContextFieldSource) -> bool {
    matches!(
        (kind, source),
        (
            FieldKindData::Subtree { category, .. },
            ContextFieldSource::Subtree(expected),
        ) if category == expected
    )
}

fn validate_context_adapters(
    groups: &[&'static GroupData],
    adapters: &[ContextAdapter],
) -> Result<(), GeneratedAssemblyError> {
    for group in groups {
        for (construction_index, construction) in group.constructions.iter().enumerate() {
            for output in construction
                .feature_combinators
                .iter()
                .map(|feature| feature.target)
                .filter(|output| is_reserved_context_output(output))
            {
                if !adapters.iter().any(|adapter| {
                    adapter.group == group.name
                        && adapter.source_category == construction.category
                        && adapter.target_output == output
                }) || typed_feature_projection(
                    group,
                    construction_index,
                    output,
                    &vec![None; construction.fields.len()],
                )
                .is_none()
                {
                    return Err(GeneratedAssemblyError::UnsupportedContextAdapter {
                        group: group.name,
                        category: construction.category,
                        output,
                    });
                }
            }
        }
    }

    for adapter in adapters {
        let Some(group) = groups
            .iter()
            .copied()
            .find(|group| group.name == adapter.group)
        else {
            continue;
        };
        let constructions = group
            .constructions
            .iter()
            .filter(|construction| {
                construction.category == adapter.source_category
                    && construction_has_output(construction, adapter.target_output)
            })
            .collect::<Vec<_>>();
        if constructions.is_empty() {
            continue;
        }
        for substitution in adapter.substitutions {
            let mut matched = false;
            for field in constructions
                .iter()
                .flat_map(|construction| construction.fields.iter())
                .filter(|field| field.name == substitution.role)
            {
                matched = true;
                if !context_field_matches(field.kind, substitution.source) {
                    return Err(GeneratedAssemblyError::InvalidContextFieldRole {
                        group: adapter.group,
                        category: adapter.source_category,
                        output: adapter.target_output,
                        role: substitution.role,
                    });
                }
            }
            if !matched {
                return Err(GeneratedAssemblyError::MissingContextFieldRole {
                    group: adapter.group,
                    category: adapter.source_category,
                    output: adapter.target_output,
                    role: substitution.role,
                });
            }
        }
    }
    Ok(())
}

fn contextual_rhs(
    adapter: ContextAdapter,
    construction: &ConstructionData,
    form: &deckmaste_construction_compiler::runtime::FormData,
    present: u64,
    rhs: &[Expected<Nonterminal, EnglishLexicalSlot>],
) -> Vec<Expected<Nonterminal, EnglishLexicalSlot>> {
    let mut rhs = rhs.iter().copied();
    let mut contextual = Vec::with_capacity(rhs.len());
    for (atom_index, atom) in form.atoms.iter().enumerate() {
        let path = match atom {
            AtomData::Hole(path) | AtomData::Lexeme(path) | AtomData::Identity(path) => Some(*path),
            AtomData::Literal(_) => None,
        };
        if path.is_some_and(|path| {
            construction.fields.iter().any(|field| {
                field.name == path
                    && matches!(field.kind, FieldKindData::Sequence { .. })
                    && present & (1_u64 << atom_index) == 0
            })
        }) {
            continue;
        }
        let expected = rhs
            .next()
            .expect("the ordinary RHS was assembled from these same present atoms");
        contextual.push(
            path.and_then(|path| {
                adapter
                    .substitutions
                    .iter()
                    .find(|substitution| substitution.role == path)
                    .map(|substitution| substitution.replacement)
            })
            .unwrap_or(expected),
        );
    }
    debug_assert!(rhs.next().is_none());
    contextual
}

fn atom_expected(
    cats: &BTreeMap<&'static str, u16>,
    aux: &AuxCategories,
    group: &'static GroupData,
    construction: &'static ConstructionData,
    atom: AtomData,
) -> Result<Expected<Nonterminal, EnglishLexicalSlot>, GeneratedAssemblyError> {
    match atom {
        AtomData::Literal(",") => Ok(Expected::Lexical(EnglishLexicalSlot::Punctuation(
            Punctuation::Comma,
        ))),
        AtomData::Literal(".") => Ok(Expected::Lexical(EnglishLexicalSlot::Punctuation(
            Punctuation::Period,
        ))),
        AtomData::Literal(literal) => Ok(Expected::Lexical(EnglishLexicalSlot::GeneratedLiteral(
            literal,
        ))),
        AtomData::Hole(path) | AtomData::Lexeme(path) | AtomData::Identity(path) => {
            if path.contains('.') {
                return Err(GeneratedAssemblyError::UnsupportedAtomPath {
                    construction: construction.id,
                    path,
                });
            }
            let field = construction
                .fields
                .iter()
                .find(|field| field.name == path)
                .ok_or(GeneratedAssemblyError::UnknownAtomField {
                    construction: construction.id,
                    field: path,
                })?;
            match (atom, field.kind) {
                (
                    AtomData::Hole(_),
                    FieldKindData::Subtree { category, .. }
                    | FieldKindData::Optional {
                        inner: &FieldKindData::Subtree { category, .. },
                    },
                ) => Ok(Expected::Nonterminal(category_nonterminal(
                    cats,
                    construction,
                    category,
                )?)),
                (AtomData::Hole(_), FieldKindData::Sequence { element }) => aux
                    .get(&(group.name, element, AuxCategoryKind::Sequence))
                    .copied()
                    .map(Nonterminal::Generated)
                    .map(Expected::Nonterminal)
                    .ok_or(GeneratedAssemblyError::UnknownElement {
                        construction: construction.id,
                        element,
                    }),
                (
                    AtomData::Lexeme(_),
                    FieldKindData::Scalar { codec }
                    | FieldKindData::Optional {
                        inner: &FieldKindData::Scalar { codec },
                    },
                ) => codec_slot(codec).map(Expected::Lexical).ok_or(
                    GeneratedAssemblyError::UnknownLexemeCodec {
                        construction: construction.id,
                        codec,
                    },
                ),
                (AtomData::Lexeme(_), FieldKindData::TypedScalar { value_type, codec }) => {
                    typed_scalar_slot(construction.id, path, value_type, codec, false)
                        .map(Expected::Lexical)
                }
                (
                    AtomData::Lexeme(_),
                    FieldKindData::Optional {
                        inner: &FieldKindData::TypedScalar { value_type, codec },
                    },
                ) => typed_scalar_slot(construction.id, path, value_type, codec, true)
                    .map(Expected::Lexical),
                (
                    AtomData::Identity(_),
                    FieldKindData::Identity {
                        value_type,
                        provider,
                    },
                ) => identity_slot(construction.id, path, value_type, provider, false)
                    .map(Expected::Lexical),
                (
                    AtomData::Identity(_),
                    FieldKindData::Optional {
                        inner:
                            &FieldKindData::Identity {
                                value_type,
                                provider,
                            },
                    },
                ) => identity_slot(construction.id, path, value_type, provider, true)
                    .map(Expected::Lexical),
                _ => Err(GeneratedAssemblyError::UnsupportedAtomKind {
                    construction: construction.id,
                    field: path,
                }),
            }
        }
    }
}

fn field_expected(
    cats: &BTreeMap<&'static str, u16>,
    construction: &'static ConstructionData,
    owner: &'static str,
    field: &'static str,
    kind: FieldKindData,
) -> Result<Expected<Nonterminal, EnglishLexicalSlot>, GeneratedAssemblyError> {
    match kind {
        FieldKindData::Subtree { category, .. } => Ok(Expected::Nonterminal(category_nonterminal(
            cats,
            construction,
            category,
        )?)),
        FieldKindData::Scalar { codec } | FieldKindData::SurfaceScalar { codec } => {
            codec_slot(codec).map(Expected::Lexical).ok_or(
                GeneratedAssemblyError::UnknownLexemeCodec {
                    construction: construction.id,
                    codec,
                },
            )
        }
        FieldKindData::TypedScalar { value_type, codec } => {
            typed_scalar_slot(owner, field, value_type, codec, false).map(Expected::Lexical)
        }
        FieldKindData::Identity {
            value_type,
            provider,
        } => identity_slot(owner, field, value_type, provider, false).map(Expected::Lexical),
        FieldKindData::Optional {
            inner:
                &FieldKindData::Identity {
                    value_type,
                    provider,
                },
        } => identity_slot(owner, field, value_type, provider, true).map(Expected::Lexical),
        FieldKindData::Optional {
            inner: &FieldKindData::TypedScalar { value_type, codec },
        } => typed_scalar_slot(owner, field, value_type, codec, true).map(Expected::Lexical),
        FieldKindData::Optional { inner } => {
            field_expected(cats, construction, owner, field, *inner)
        }
        FieldKindData::Sequence { .. } => Err(GeneratedAssemblyError::UnsupportedAtomKind {
            construction: construction.id,
            field: "nested sequence element",
        }),
    }
}

/// Registers every form of every construction of every active group, in
/// declaration order, with EXPLICIT ordinals from the declaration data.
pub(super) fn register_generated(
    builder: &mut RuleBuilder,
    groups: &[&'static GroupData],
    cats: &BTreeMap<&'static str, u16>,
) -> Result<(), GeneratedAssemblyError> {
    register_generated_with_context_adapters(builder, groups, cats, CONTEXT_ADAPTERS)
}

#[allow(
    clippy::too_many_lines,
    reason = "registration follows the declaration's group, construction, form, and sequence nesting"
)]
fn register_generated_with_context_adapters(
    builder: &mut RuleBuilder,
    groups: &[&'static GroupData],
    cats: &BTreeMap<&'static str, u16>,
    context_adapters: &[ContextAdapter],
) -> Result<(), GeneratedAssemblyError> {
    for group in groups {
        if let Some(element) = group.element_data.iter().find(|element| {
            !element.fields.is_empty()
                && element.variants.is_empty()
                && element.fields.iter().all(|field| {
                    matches!(field.kind, FieldKindData::Optional { .. })
                        || matches!(
                            field.kind,
                            FieldKindData::Scalar { codec: "Comma" }
                                | FieldKindData::SurfaceScalar { codec: "Comma" }
                        )
                })
        }) {
            return Err(GeneratedAssemblyError::NullableElement {
                group: group.name,
                element: element.name,
            });
        }
    }
    validate_field_adapters(groups)?;
    validate_evidence_sources(groups)?;
    validate_context_adapters(groups, context_adapters)?;
    let aux = auxiliary_categories(groups, cats.len());
    if groups.iter().any(|group| {
        group.element_data.iter().any(|element| {
            element.variants.iter().any(|variant| {
                matches!(
                    variant.payload,
                    FieldKindData::Subtree {
                        category: "PowerToughness",
                        ..
                    }
                )
            })
        })
    }) {
        builder.add_generated(
            RuleImpl::GeneratedAux(GeneratedAuxRuleRef::Transparent),
            ProductionId {
                construction: ConstructionId::new("__generated_power_toughness"),
                ordinal: 0,
            },
            Nonterminal::PowerToughness,
            [Expected::Lexical(EnglishLexicalSlot::PowerToughness)],
        );
    }
    for group in groups {
        for (element_index, element) in group.element_data.iter().enumerate() {
            register_element(builder, group, element_index, element, cats, &aux)?;
        }
        for (construction_index, construction) in group.constructions.iter().enumerate() {
            let lhs = lhs_category_nonterminal(cats, construction)?;
            for (form_index, form) in construction.forms.iter().enumerate() {
                let sequence_atoms = form
                    .atoms
                    .iter()
                    .enumerate()
                    .filter_map(|(index, atom)| match atom {
                        AtomData::Hole(path) => construction
                            .fields
                            .iter()
                            .find(|field| field.name == *path)
                            .and_then(|field| {
                                matches!(field.kind, FieldKindData::Sequence { .. })
                                    .then_some(index)
                            }),
                        AtomData::Literal(_) | AtomData::Lexeme(_) | AtomData::Identity(_) => None,
                    })
                    .collect::<Vec<_>>();
                if sequence_atoms.len() > 15
                    || sequence_atoms
                        .iter()
                        .any(|&index| index >= u64::BITS as usize)
                {
                    return Err(GeneratedAssemblyError::TooManyOptionalAtoms {
                        owner: construction.id,
                    });
                }
                for subset in 0..(1_u64 << sequence_atoms.len()) {
                    if sequence_atoms
                        .iter()
                        .enumerate()
                        .any(|(position, &atom_index)| {
                            let Some(AtomData::Hole(path)) = form.atoms.get(atom_index) else {
                                return false;
                            };
                            sequence_is_required_nonempty(construction, path)
                                && subset & (1_u64 << position) == 0
                        })
                    {
                        // Do not register a nullable shape which the
                        // declaration itself rejects. Besides wasting chart
                        // work, a recursive category with an omitted required
                        // tail can seed an unbounded family of empty wrappers.
                        continue;
                    }
                    let mut rhs = Vec::with_capacity(form.atoms.len());
                    let mut present = 0_u64;
                    for (atom_index, atom) in form.atoms.iter().enumerate() {
                        let sequence_position =
                            sequence_atoms.iter().position(|&i| i == atom_index);
                        if let Some(position) = sequence_position {
                            if subset & (1_u64 << position) == 0 {
                                continue;
                            }
                            present |= 1_u64 << atom_index;
                        }
                        rhs.push(atom_expected(cats, &aux, group, construction, *atom)?);
                    }
                    if rhs.is_empty() {
                        continue;
                    }
                    if sequence_atoms
                        .iter()
                        .any(|&index| present & (1_u64 << index) == 0)
                        && matches!(
                            rhs.as_slice(),
                            [Expected::Nonterminal(target)] if *target == lhs
                        )
                    {
                        continue;
                    }
                    builder.add_generated_with_cost(
                        RuleImpl::Generated(GeneratedRuleRef {
                            group,
                            construction: construction_index,
                            form: form_index,
                            sequence_atoms: present,
                            context: GeneratedRuleContext::Value,
                        }),
                        ProductionId {
                            construction: ConstructionId::new(construction.id),
                            ordinal: form.ordinal,
                        },
                        lhs,
                        rhs.clone(),
                        generated_cost(construction),
                    );
                    for adapter in context_adapters.iter().copied().filter(|adapter| {
                        adapter.group == group.name
                            && adapter.source_category == construction.category
                            && construction_has_output(construction, adapter.target_output)
                    }) {
                        let alternate_rhs =
                            contextual_rhs(adapter, construction, form, present, &rhs);
                        builder.add_generated_with_cost(
                            RuleImpl::Generated(GeneratedRuleRef {
                                group,
                                construction: construction_index,
                                form: form_index,
                                sequence_atoms: present,
                                context: adapter.context,
                            }),
                            ProductionId {
                                construction: ConstructionId::new(construction.id),
                                ordinal: form.ordinal,
                            },
                            adapter.target_category,
                            alternate_rhs,
                            generated_cost(construction),
                        );
                    }
                    if GeneratedFeatureCombinator::from_construction(construction)
                        .is_some_and(GeneratedFeatureCombinator::admits_shared_preposition)
                    {
                        let mut prepositional_rhs = Vec::with_capacity(rhs.len() + 1);
                        prepositional_rhs.push(Expected::Lexical(EnglishLexicalSlot::Preposition));
                        prepositional_rhs.extend(rhs.iter().copied());
                        builder.add_generated_with_cost(
                            RuleImpl::Generated(GeneratedRuleRef {
                                group,
                                construction: construction_index,
                                form: form_index,
                                sequence_atoms: present,
                                context: GeneratedRuleContext::SharedPreposition,
                            }),
                            ProductionId {
                                construction: ConstructionId::new(construction.id),
                                ordinal: form.ordinal,
                            },
                            Nonterminal::PrepositionalPhrase,
                            prepositional_rhs,
                            generated_cost(construction),
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn sequence_is_required_nonempty(construction: &ConstructionData, path: &str) -> bool {
    construction
        .requirements
        .iter()
        .chain(construction.recognition_requirements)
        .any(|requirement| predicate_requires_nonempty(requirement.predicate, path))
}

fn predicate_requires_nonempty(predicate: PredicateData, path: &str) -> bool {
    match predicate {
        PredicateData::LenAtLeast {
            path: predicate_path,
            min,
        } => predicate_path == path && min >= 1,
        PredicateData::LenIs {
            path: predicate_path,
            len,
        } => predicate_path == path && len >= 1,
        PredicateData::All(predicates) => predicates
            .iter()
            .any(|predicate| predicate_requires_nonempty(*predicate, path)),
        PredicateData::Any(predicates) => {
            !predicates.is_empty()
                && predicates
                    .iter()
                    .all(|predicate| predicate_requires_nonempty(*predicate, path))
        }
        PredicateData::In { .. } | PredicateData::IsSome { .. } | PredicateData::IsNone { .. } => {
            false
        }
    }
}

fn generated_cost(construction: &ConstructionData) -> super::ParseCost {
    let mut cost = GeneratedFeatureCombinator::from_construction(construction).map_or_else(
        super::ParseCost::default,
        GeneratedFeatureCombinator::base_cost,
    );
    for output in construction.feature_combinators {
        match output.target {
            "base_precedence" => cost.precedence = cost.precedence.saturating_add(1),
            "base_precedence_2" => cost.precedence = cost.precedence.saturating_add(2),
            "base_attachment_count" => {
                cost.attachment_count = cost.attachment_count.saturating_add(1);
            }
            _ => {}
        }
    }
    cost
}

fn rules_object_member_rhs(
    group: &GroupData,
    element: &ElementData,
    present_fields: u64,
    rhs: &[Expected<Nonterminal, EnglishLexicalSlot>],
) -> Option<Vec<Expected<Nonterminal, EnglishLexicalSlot>>> {
    let (combinator, construction) = coordination_member_role(group, element.name)?;
    if combinator != GeneratedFeatureCombinator::CompleteNounPhraseCoordination {
        return None;
    }
    let value_field = combinator.member_value_field_index(group, construction)?;
    if present_fields & (1_u64 << value_field) == 0 {
        return None;
    }
    let rhs_field = (0..=value_field)
        .filter(|index| present_fields & (1_u64 << index) != 0)
        .count()
        .checked_sub(1)?;
    let mut alternate = rhs.to_vec();
    *alternate.get_mut(rhs_field)? = Expected::Nonterminal(Nonterminal::RulesObjectNounPhrase);
    Some(alternate)
}

fn element_field_expected(
    cats: &BTreeMap<&'static str, u16>,
    construction: &'static ConstructionData,
    element: &'static ElementData,
    field: &'static str,
    kind: FieldKindData,
) -> Result<Expected<Nonterminal, EnglishLexicalSlot>, GeneratedAssemblyError> {
    field_expected(cats, construction, element.name, field, kind)
}

fn register_element_variants(
    builder: &mut RuleBuilder,
    group: &'static GroupData,
    element_index: usize,
    element: &'static ElementData,
    cats: &BTreeMap<&'static str, u16>,
    representative: &'static ConstructionData,
    element_nt: Nonterminal,
) -> Result<u16, GeneratedAssemblyError> {
    let mut ordinal = 0_u16;
    for (variant_index, variant) in element.variants.iter().enumerate() {
        builder.add_generated(
            RuleImpl::GeneratedAux(GeneratedAuxRuleRef::ElementVariant {
                group,
                element: element_index,
                variant: variant_index,
            }),
            ProductionId {
                construction: ConstructionId::new(element.name),
                ordinal,
            },
            element_nt,
            [element_field_expected(
                cats,
                representative,
                element,
                variant.name,
                variant.payload,
            )?],
        );
        ordinal = ordinal
            .checked_add(1)
            .expect("element rule ordinal overflow");
    }
    Ok(ordinal)
}

fn register_element(
    builder: &mut RuleBuilder,
    group: &'static GroupData,
    element_index: usize,
    element: &'static ElementData,
    cats: &BTreeMap<&'static str, u16>,
    aux: &AuxCategories,
) -> Result<(), GeneratedAssemblyError> {
    let element_nt =
        Nonterminal::Generated(aux[&(group.name, element.name, AuxCategoryKind::Element)]);
    let sequence_nt =
        Nonterminal::Generated(aux[&(group.name, element.name, AuxCategoryKind::Sequence)]);
    let representative = group
        .constructions
        .first()
        .expect("validated generated groups contain a construction");
    let mut ordinal = 0_u16;

    if element.variants.is_empty() && !element.fields.is_empty() {
        if element.fields.len() > u64::BITS as usize {
            return Err(GeneratedAssemblyError::TooManyOptionalAtoms {
                owner: element.name,
            });
        }
        let optional = element
            .fields
            .iter()
            .enumerate()
            .filter_map(|(index, field)| {
                matches!(field.kind, FieldKindData::Optional { .. })
                    .then_some(index)
                    .or_else(|| {
                        matches!(
                            field.kind,
                            FieldKindData::Scalar { codec: "Comma" }
                                | FieldKindData::SurfaceScalar { codec: "Comma" }
                        )
                        .then_some(index)
                    })
            })
            .collect::<Vec<_>>();
        if optional.len() > 15 {
            return Err(GeneratedAssemblyError::TooManyOptionalAtoms {
                owner: element.name,
            });
        }
        for subset in 0..(1_u64 << optional.len()) {
            let mut rhs = Vec::new();
            let mut present_fields = 0_u64;
            for (field_index, field) in element.fields.iter().enumerate() {
                if let Some(position) = optional.iter().position(|&i| i == field_index)
                    && subset & (1_u64 << position) == 0
                {
                    continue;
                }
                present_fields |= 1_u64 << field_index;
                rhs.push(element_field_expected(
                    cats,
                    representative,
                    element,
                    field.name,
                    field.kind,
                )?);
            }
            if let Some((comma, conjunction)) = coordination_delimiter_fields(group, element)
                && present_fields & (1_u64 << comma | 1_u64 << conjunction) == 0
            {
                // Neither delimiter can participate in an admitted member;
                // omitting this rule avoids a delimiter-free recursive NP
                // sequence in the chart.
                continue;
            }
            let rules_object_rhs = rules_object_member_rhs(group, element, present_fields, &rhs);
            builder.add_generated(
                RuleImpl::GeneratedAux(GeneratedAuxRuleRef::ElementStruct {
                    group,
                    element: element_index,
                    present_fields,
                }),
                ProductionId {
                    construction: ConstructionId::new(element.name),
                    ordinal,
                },
                element_nt,
                rhs,
            );
            ordinal = ordinal
                .checked_add(1)
                .expect("element rule ordinal overflow");
            if let Some(rhs) = rules_object_rhs {
                builder.add_generated(
                    RuleImpl::GeneratedAux(GeneratedAuxRuleRef::ElementStruct {
                        group,
                        element: element_index,
                        present_fields,
                    }),
                    ProductionId {
                        construction: ConstructionId::new(element.name),
                        ordinal,
                    },
                    element_nt,
                    rhs,
                );
                ordinal = ordinal
                    .checked_add(1)
                    .expect("element rule ordinal overflow");
            }
        }
    } else {
        ordinal = register_element_variants(
            builder,
            group,
            element_index,
            element,
            cats,
            representative,
            element_nt,
        )?;
    }

    if ordinal == 0 {
        return Ok(());
    }
    builder.add_generated(
        RuleImpl::GeneratedAux(GeneratedAuxRuleRef::SequenceSeed {
            group,
            element: element_index,
        }),
        ProductionId {
            construction: ConstructionId::new(element.name),
            ordinal,
        },
        sequence_nt,
        [Expected::Nonterminal(element_nt)],
    );
    builder.add_generated(
        RuleImpl::GeneratedAux(GeneratedAuxRuleRef::SequenceExtend {
            group,
            element: element_index,
        }),
        ProductionId {
            construction: ConstructionId::new(element.name),
            ordinal: ordinal
                .checked_add(1)
                .expect("element rule ordinal overflow"),
        },
        sequence_nt,
        [
            Expected::Nonterminal(sequence_nt),
            Expected::Nonterminal(element_nt),
        ],
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use deckmaste_construction_compiler::runtime::AtomData;
    use deckmaste_construction_compiler::runtime::ConstructionData;
    use deckmaste_construction_compiler::runtime::ElementData;
    use deckmaste_construction_compiler::runtime::EvidenceData;
    use deckmaste_construction_compiler::runtime::EvidenceKindData;
    use deckmaste_construction_compiler::runtime::EvidenceSourceData;
    use deckmaste_construction_compiler::runtime::FeatureCombinatorData;
    use deckmaste_construction_compiler::runtime::FieldData;
    use deckmaste_construction_compiler::runtime::FieldKindData;
    use deckmaste_construction_compiler::runtime::FormData;
    use deckmaste_construction_compiler::runtime::GroupData;

    use super::super::EnglishLexicalSlot;
    use super::super::Expected;
    use super::super::ParseCost;
    use super::super::rules::RegistrationOrder;
    use super::super::rules::RuleBuilder;
    use super::GeneratedAssemblyError;
    use super::GeneratedFeatureCombinator;
    use super::OBJECT_GAP_CONTEXT_ADAPTER;
    use super::generated_cost;
    use super::internal_categories;
    use super::register_generated;
    use super::register_generated_with_context_adapters;
    use crate::surface::Punctuation;

    const fn construction(
        id: &'static str,
        category: &'static str,
        internal: bool,
        fields: &'static [FieldData],
        forms: &'static [FormData],
    ) -> ConstructionData {
        ConstructionData {
            id,
            category,
            internal,
            own_type: Some("Synthetic"),
            bind_path: None,
            lens: None,
            projection_variant: None,
            deserialize: false,
            selection_unique: false,
            dominates: &[],
            dominated_by: &[],
            fields,
            witnesses: &[],
            forms,
            requirements: &[],
            recognition_requirements: &[],
            feature_combinators: &[],
            evidence: None,
            erased_partial_builder: None,
            erased_builder: None,
            erased_projector: None,
        }
    }

    const WORD_FIELDS: &[FieldData] = &[FieldData {
        name: "word",
        kind: FieldKindData::Scalar {
            codec: "Conjunction",
        },
    }];
    const WORD_FORM: &[FormData] = &[FormData {
        name: "only",
        ordinal: 0,
        guarded: false,
        erased_recognizer: None,
        atoms: &[AtomData::Lexeme("word")],
    }];

    #[test]
    fn adjective_category_adapters_are_independent_of_construction_ids() {
        const ADJECTIVE_FIELDS: &[FieldData] = &[FieldData {
            name: "identity",
            kind: FieldKindData::Identity {
                value_type: "Adjective",
                provider: "Adjective",
            },
        }];
        const ADJECTIVE_FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Identity("identity")],
        }];
        const PHRASE_FIELDS: &[FieldData] = &[FieldData {
            name: "head",
            kind: FieldKindData::Subtree {
                category: "Adjective",
                boxed: false,
            },
        }];
        const PHRASE_FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Hole("head")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] = &[
            construction(
                "descriptive_word_alias",
                "Adjective",
                false,
                ADJECTIVE_FIELDS,
                ADJECTIVE_FORM,
            ),
            construction(
                "descriptive_phrase_alias",
                "AdjectivePhrase",
                false,
                PHRASE_FIELDS,
                PHRASE_FORM,
            ),
        ];
        const GROUP: GroupData = GroupData {
            name: "adjective_adapter_fixture",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };

        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, &[&GROUP], &cats)
            .expect("category-owned adjective adapters assemble aliases");
        let book = builder.finish(RegistrationOrder::Normal);

        assert_eq!(book.rules[0].lhs, super::super::Nonterminal::Adjective);
        assert_eq!(
            book.rules[0].rhs,
            [Expected::Lexical(EnglishLexicalSlot::Adjective)]
        );
        assert_eq!(
            book.rules[1].lhs,
            super::super::Nonterminal::AdjectivePhrase
        );
        assert_eq!(
            book.rules[1].rhs,
            [Expected::Nonterminal(super::super::Nonterminal::Adjective)]
        );
    }

    #[test]
    fn adjective_attachment_and_degree_use_category_owned_generic_adapters() {
        let declaration = &crate::constructions::adjective::ADJECTIVE_DECLARATION;
        let comparison = &declaration.constructions[7];
        assert_eq!(comparison.id, "adjective_phrase_comparison");
        assert_eq!(
            comparison.fields[0].kind,
            FieldKindData::Subtree {
                category: "AdjectivePhrase",
                boxed: false,
            }
        );
        assert_eq!(
            comparison.fields[1].kind,
            FieldKindData::Subtree {
                category: "ComparisonComplement",
                boxed: false,
            }
        );
        let lens = comparison
            .lens
            .expect("comparison appends through its owner lens");
        assert_eq!(
            (lens.owner, lens.source),
            ("adjective_phrase_complements", Some("owner"))
        );
        assert_eq!(lens.edits[0].target, "complements");
        assert_eq!(
            lens.edits[0].kind,
            deckmaste_construction_compiler::runtime::LensEditKindData::Append
        );

        let degree = &declaration.constructions[8];
        assert_eq!(degree.id, "adjective_phrase_degree_measure");
        assert_eq!(
            degree.fields[0].kind,
            FieldKindData::TypedScalar {
                value_type: "NumberLiteral",
                codec: "Numeral",
            }
        );
        assert_eq!(
            super::typed_scalar_slot(
                degree.id,
                degree.fields[0].name,
                "NumberLiteral",
                "Numeral",
                false,
            ),
            Ok(EnglishLexicalSlot::QuantityNumber)
        );

        let cats = internal_categories(&[declaration]);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, &[declaration], &cats)
            .expect("the generic adjective lens and scalar adapters assemble");
    }

    #[test]
    fn engine_combinator_discovery_preserves_legacy_targets_among_typed_outputs() {
        const FIELDS: &[FieldData] = &[
            FieldData {
                name: "first",
                kind: FieldKindData::Scalar {
                    codec: "Conjunction",
                },
            },
            FieldData {
                name: "rest",
                kind: FieldKindData::Scalar {
                    codec: "Conjunction",
                },
            },
        ];
        const OUTPUTS: &[FeatureCombinatorData] = &[
            FeatureCombinatorData {
                target: "base_precedence",
                combinator: "mark_generated_cost",
                args: &["rest"],
            },
            FeatureCombinatorData {
                target: "first",
                combinator: "complete_noun_phrase_coordination",
                args: &["first", "rest"],
            },
        ];
        let construction = ConstructionData {
            feature_combinators: OUTPUTS,
            ..construction("coordination", "NounPhrase", false, FIELDS, WORD_FORM)
        };

        let combinator = GeneratedFeatureCombinator::from_construction(&construction)
            .expect("the closed engine combinator is independent of its output target");
        assert_eq!(
            combinator,
            GeneratedFeatureCombinator::CompleteNounPhraseCoordination
        );
        assert_eq!(combinator.first_member_field_index(&construction), Some(0));
        assert_eq!(combinator.rest_field_index(&construction), Some(1));
    }

    #[test]
    fn predicate_static_attachment_costs_are_declaration_owned() {
        // Mutation caught: collapse the adjective dispreference from two
        // precedence points to the PP's single point. Equalizing those costs
        // makes adjective-vs-PP selection registration-order-sensitive.
        let construction = |id| {
            crate::constructions::predicate::PREDICATE_DECLARATION
                .constructions
                .iter()
                .find(|construction| construction.id == id)
                .expect("the Task 2 census contains the costed declaration")
        };
        assert_eq!(
            generated_cost(construction("verb_phrase_adjective")).precedence,
            2
        );
        assert_eq!(
            generated_cost(construction("verb_phrase_prepositional")).precedence,
            1
        );
        for id in [
            "verb_phrase_passive_shared_determiner_prepositional",
            "verb_phrase_except_by",
            "verb_phrase_adverb",
            "verb_phrase_preverb_adverb",
            "verb_phrase_particle",
            "verb_phrase_coin_result",
            "frequency_phrase_adverb",
            "frequency_phrase",
        ] {
            assert_eq!(
                generated_cost(construction(id)),
                ParseCost::default(),
                "{id} retains its exact historical zero local cost"
            );
        }
        assert_eq!(
            generated_cost(construction("verb_phrase_frequency")),
            ParseCost {
                attachment_count: 1,
                ..ParseCost::default()
            },
            "frequency declares the attachment cost that preserves its reduced-passive role"
        );
    }

    #[test]
    fn predicate_auxiliary_declares_dominance_over_postverbal_adverb() {
        let auxiliary = crate::constructions::predicate::PREDICATE_DECLARATION
            .constructions
            .iter()
            .find(|construction| construction.id == "verb_phrase_auxiliary")
            .expect("the predicate declaration contains the auxiliary construction");
        assert!(
            auxiliary.dominates.contains(&"verb_phrase_adverb"),
            "the generated declaration itself must encode auxiliary > postverbal adverb"
        );
    }

    #[test]
    fn predicate_value_fields_have_closed_typed_chart_adapters() {
        // Mutations caught: route an ability, quoted ability, symbol, symbol
        // sequence, or P/T value through a string/scalar fallback, or treat a
        // mana-list category as an untyped generated side channel.
        assert_eq!(
            super::identity_slot(
                "verb_phrase_ability",
                "ability",
                "CatalogAtom",
                "AbilityItem",
                false
            ),
            Ok(EnglishLexicalSlot::AbilityItem)
        );
        assert_eq!(
            super::identity_slot(
                "verb_phrase_quoted_ability",
                "ability",
                "QuotedAbility",
                "QuotedAbility",
                false,
            ),
            Ok(EnglishLexicalSlot::QuotedAbility)
        );
        assert_eq!(
            super::identity_slot(
                "verb_phrase_oracle_symbol",
                "symbol",
                "OracleSymbol",
                "OracleSymbol",
                false,
            ),
            Ok(EnglishLexicalSlot::OracleSymbol)
        );
        assert_eq!(
            super::identity_slot(
                "verb_phrase_symbol_sequence",
                "symbols",
                "SymbolSequence",
                "SymbolSequence",
                false,
            ),
            Ok(EnglishLexicalSlot::SymbolSequence)
        );
        assert_eq!(
            super::typed_scalar_slot(
                "verb_phrase_power_toughness",
                "value",
                "PowerToughness",
                "PowerToughness",
                false,
            ),
            Ok(EnglishLexicalSlot::PowerToughness)
        );
        assert_eq!(
            super::engine_category("ManaAmount"),
            Some(super::super::Nonterminal::ManaAmount)
        );
        assert_eq!(
            super::engine_category("ManaAmountList"),
            Some(super::super::Nonterminal::ManaAmountList)
        );
        assert_eq!(
            super::engine_category("CoordinatedManaAmount"),
            Some(super::super::Nonterminal::CoordinatedManaAmount)
        );
    }

    #[test]
    fn predicate_temporal_attachment_cost_is_a_typed_conditional_output() {
        // Mutation caught: omit the handwritten +1 precedence cost when the
        // direct-object surface slot is actually an active temporal adjunct.
        use crate::grammar::Features;
        use crate::grammar::NounPhraseCoordinationState;
        use crate::grammar::PredicateAttachmentPhase;
        use crate::grammar::PredicateForm;
        use crate::grammar::PredicateObjectState;
        use crate::grammar::SetExceptionState;
        use crate::word::BareNominalAdjunct;
        use crate::word::PredicateFrame;

        let construction = crate::constructions::predicate::PREDICATE_DECLARATION
            .constructions
            .iter()
            .position(|construction| construction.id == "verb_phrase_direct_object")
            .expect("the Task 2 census contains the direct-object declaration");
        let active = Features::VerbPhrase {
            form: PredicateForm::Imperative,
            passive: false,
            dependent_count: 1,
            object: PredicateObjectState::Direct,
            indirect_object: false,
            selected_preposition: false,
            phase: PredicateAttachmentPhase::Object,
            frame: PredicateFrame::OPEN,
            bare: true,
            head_is_copular: false,
            object_gap_requires_rules_object: false,
            subjunctive: false,
        };
        let temporal = Features::NounPhrase {
            agreement: None,
            coordination_domain: None,
            pronoun_case: None,
            adjunct: Some(BareNominalAdjunct::Temporal),
            set_exception: SetExceptionState::Ineligible,
            coordination: NounPhraseCoordinationState::None,
            recipient_passive_theme: false,
            rules_object_followup: false,
        };
        assert!(matches!(
            super::typed_feature_projection(
                &crate::constructions::predicate::PREDICATE_DECLARATION,
                construction,
                "precedence",
                &[Some(&active), Some(&temporal)],
            ),
            Some(Some(_))
        ));
        let mut passive = active.clone();
        let Features::VerbPhrase {
            passive: passive_state,
            ..
        } = &mut passive
        else {
            unreachable!()
        };
        *passive_state = true;
        assert_eq!(
            super::typed_feature_projection(
                &crate::constructions::predicate::PREDICATE_DECLARATION,
                construction,
                "precedence",
                &[Some(&passive), Some(&temporal)],
            ),
            Some(None)
        );
    }

    const CONTEXT_OUTPUT: &[FeatureCombinatorData] = &[FeatureCombinatorData {
        target: "object_gap",
        combinator: "synthetic_object_gap",
        args: &["predicate"],
    }];
    const PREDICATE_FIELD: FieldData = FieldData {
        name: "predicate",
        kind: FieldKindData::Subtree {
            category: "VerbPhrase",
            boxed: false,
        },
    };
    const OTHER_PREDICATE_FIELD: FieldData = FieldData {
        name: "other",
        kind: FieldKindData::Subtree {
            category: "VerbPhrase",
            boxed: false,
        },
    };

    fn contextual_construction(
        category: &'static str,
        fields: &'static [FieldData],
        atoms: &'static [AtomData],
    ) -> ConstructionData {
        let forms = Box::leak(
            vec![FormData {
                name: "only",
                ordinal: 0,
                guarded: false,
                erased_recognizer: None,
                atoms,
            }]
            .into_boxed_slice(),
        );
        ConstructionData {
            feature_combinators: CONTEXT_OUTPUT,
            ..construction("contextual", category, false, fields, forms)
        }
    }

    #[allow(
        clippy::large_types_passed_by_value,
        reason = "the synthetic runtime declaration is moved into deliberately leaked test metadata"
    )]
    fn contextual_group(name: &'static str, construction: ConstructionData) -> &'static GroupData {
        let constructions = Box::leak(vec![construction].into_boxed_slice());
        Box::leak(Box::new(GroupData {
            name,
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions,
        }))
    }

    #[test]
    fn contextual_output_is_rejected_on_the_wrong_group() {
        // Mutation caught: trigger a contextual adapter globally from the
        // bare `object_gap` target string, regardless of declaration group.
        let group = contextual_group(
            "not_predicate",
            contextual_construction(
                "VerbPhrase",
                &[PREDICATE_FIELD],
                &[AtomData::Hole("predicate")],
            ),
        );
        let cats = internal_categories(&[group]);
        assert!(register_generated(&mut RuleBuilder::default(), &[group], &cats).is_err());
    }

    #[test]
    fn contextual_output_is_rejected_on_the_wrong_source_category() {
        // Mutation caught: accept the predicate context target on a source
        // category other than the declared VerbPhrase adapter domain.
        let group = contextual_group(
            "predicate",
            contextual_construction(
                "NounPhrase",
                &[PREDICATE_FIELD],
                &[AtomData::Hole("predicate")],
            ),
        );
        let cats = internal_categories(&[group]);
        assert!(register_generated(&mut RuleBuilder::default(), &[group], &cats).is_err());
    }

    #[test]
    fn contextual_output_is_rejected_when_the_adapter_names_the_wrong_output() {
        // Mutation caught: let an adapter silently bind a target output other
        // than the declaration's authored contextual feature.
        let group = contextual_group(
            "predicate",
            contextual_construction(
                "VerbPhrase",
                &[PREDICATE_FIELD],
                &[AtomData::Hole("predicate")],
            ),
        );
        let cats = internal_categories(&[group]);
        let wrong_output = super::ContextAdapter {
            target_output: "not_object_gap",
            ..OBJECT_GAP_CONTEXT_ADAPTER
        };
        assert!(
            register_generated_with_context_adapters(
                &mut RuleBuilder::default(),
                &[group],
                &cats,
                &[wrong_output],
            )
            .is_err()
        );
    }

    #[test]
    fn contextual_output_is_rejected_when_the_declared_role_is_absent() {
        // Mutation caught: replace any VerbPhrase-shaped field even when the
        // declaration did not assign it the contextual `predicate` role.
        const OUTPUT: &[FeatureCombinatorData] = &[FeatureCombinatorData {
            target: "object_gap",
            combinator: "synthetic_object_gap",
            args: &["other"],
        }];
        let mut construction = contextual_construction(
            "VerbPhrase",
            &[OTHER_PREDICATE_FIELD],
            &[AtomData::Hole("other")],
        );
        construction.feature_combinators = OUTPUT;
        let group = contextual_group("predicate", construction);
        let cats = internal_categories(&[group]);
        assert!(register_generated(&mut RuleBuilder::default(), &[group], &cats).is_err());
    }

    #[test]
    fn contextual_output_is_rejected_when_the_role_has_the_wrong_field_category() {
        // Mutation caught: accept the right role name with a source field
        // category that does not satisfy the adapter's typed contract.
        const WRONG_PREDICATE_FIELD: FieldData = FieldData {
            name: "predicate",
            kind: FieldKindData::Subtree {
                category: "NounPhrase",
                boxed: false,
            },
        };
        let group = contextual_group(
            "predicate",
            contextual_construction(
                "VerbPhrase",
                &[WRONG_PREDICATE_FIELD],
                &[AtomData::Hole("predicate")],
            ),
        );
        let cats = internal_categories(&[group]);
        assert!(register_generated(&mut RuleBuilder::default(), &[group], &cats).is_err());
    }

    #[test]
    fn contextual_output_substitutes_only_the_declared_field_role() {
        // Mutation caught: rewrite every recursive VerbPhrase RHS member
        // instead of only the explicitly adapted `predicate` field.
        let group = contextual_group(
            "predicate",
            contextual_construction(
                "VerbPhrase",
                &[PREDICATE_FIELD, OTHER_PREDICATE_FIELD],
                &[AtomData::Hole("predicate"), AtomData::Hole("other")],
            ),
        );
        let cats = internal_categories(&[group]);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, &[group], &cats).expect("the adapter shape is valid");
        let book = builder.finish(RegistrationOrder::Normal);
        let contextual = book
            .rules
            .iter()
            .find(|rule| rule.lhs == super::super::Nonterminal::ObjectGapVerbPhrase)
            .expect("the object-gap output registers one contextual rule");
        assert_eq!(
            contextual.rhs,
            [
                Expected::Nonterminal(super::super::Nonterminal::ObjectGapVerbPhrase),
                Expected::Nonterminal(super::super::Nonterminal::VerbPhrase),
            ]
        );
    }

    #[test]
    fn production_frequency_registers_a_reduced_passive_context_rule() {
        let cats = internal_categories(crate::constructions::GROUPS);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, crate::constructions::GROUPS, &cats)
            .expect("production declarations assemble");
        let book = builder.finish(RegistrationOrder::Normal);
        let rule = book
            .rules
            .iter()
            .find(|rule| {
                rule.production.construction.as_str() == "verb_phrase_frequency"
                    && rule.lhs == super::super::Nonterminal::ReducedRecipientPassive
            })
            .expect("frequency declares a reduced-passive contextual production");
        assert_eq!(
            rule.rhs,
            [
                Expected::Nonterminal(super::super::Nonterminal::ReducedRecipientPassive),
                Expected::Nonterminal(super::super::Nonterminal::FrequencyPhrase),
            ]
        );
    }

    #[test]
    fn unknown_category_is_refused() {
        const CONSTRUCTIONS: &[ConstructionData] = &[construction(
            "np_only",
            "DefinitelyUnknown",
            false,
            WORD_FIELDS,
            WORD_FORM,
        )];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnknownCategory {
                construction: "np_only",
                category: "DefinitelyUnknown",
            }
        );
    }

    #[test]
    fn unsupported_evidence_source_is_refused_before_registration() {
        const CONSTRUCTIONS: &[ConstructionData] = &[ConstructionData {
            evidence: Some(EvidenceData {
                kind: EvidenceKindData::Feature,
                label: "synthetic output",
                source: EvidenceSourceData::Output("unknown.path"),
            }),
            ..construction("np_only", "NounPhrase", false, WORD_FIELDS, WORD_FORM)
        }];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedEvidenceSource {
                construction: "np_only",
                source: EvidenceSourceData::Output("unknown.path"),
            }
        );
        assert!(
            builder.finish(RegistrationOrder::Normal).rules.is_empty(),
            "unsupported evidence must fail before mutating the rule builder",
        );
    }

    #[test]
    fn nullable_element_is_refused_before_rules_are_registered() {
        const NULLABLE_FIELDS: &[FieldData] = &[FieldData {
            name: "maybe_word",
            kind: FieldKindData::Optional {
                inner: &FieldKindData::Scalar {
                    codec: "Conjunction",
                },
            },
        }];
        const ELEMENT_DATA: &[ElementData] = &[ElementData {
            name: "nullable_member",
            bind_path: None,
            fields: NULLABLE_FIELDS,
            variants: &[],
            erased_builders: &[],
            erased_sequence_builder: None,
        }];
        const CONSTRUCTIONS: &[ConstructionData] = &[construction(
            "np_only",
            "NounPhrase",
            false,
            WORD_FIELDS,
            WORD_FORM,
        )];
        const GROUP: GroupData = GroupData {
            name: "nullable",
            elements: &["nullable_member"],
            element_data: ELEMENT_DATA,
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::NullableElement {
                group: "nullable",
                element: "nullable_member",
            },
        );
        assert!(
            builder.finish(RegistrationOrder::Normal).rules.is_empty(),
            "nullable-element rejection must happen before the builder is mutated",
        );
    }

    /// Controller finding: a non-internal construction sharing a category
    /// name with an internal sibling in the same group must still surface
    /// `UnknownCategory` — membership is decided by the construction's own
    /// `internal` flag, never by name presence in the internal-category map.
    #[test]
    fn non_internal_construction_sharing_an_internal_category_name_is_refused() {
        const CONSTRUCTIONS: &[ConstructionData] = &[
            construction("internal_member", "Shared", true, WORD_FIELDS, WORD_FORM),
            construction(
                "non_internal_member",
                "Shared",
                false,
                WORD_FIELDS,
                WORD_FORM,
            ),
        ];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        assert!(
            cats.contains_key("Shared"),
            "the internal member must seed the category map"
        );
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnknownCategory {
                construction: "non_internal_member",
                category: "Shared",
            }
        );
    }

    #[test]
    fn generated_word_literal_is_admitted() {
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Literal("and")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("lit_and", "Internal", true, &[], FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, &[&GROUP], &cats)
            .expect("declaration-owned word literal assembles");
        let rule_book = builder.finish(RegistrationOrder::Normal);
        assert_eq!(
            rule_book.rules[0].rhs,
            vec![Expected::Lexical(EnglishLexicalSlot::GeneratedLiteral(
                "and"
            ))]
        );
    }

    #[test]
    fn comma_literal_is_admitted() {
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Literal(",")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("lit_comma", "Internal", true, &[], FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, &[&GROUP], &cats).expect("comma literal must assemble");
        let rule_book = builder.finish(RegistrationOrder::Normal);
        assert_eq!(
            rule_book.rules[0].rhs,
            vec![Expected::Lexical(EnglishLexicalSlot::Punctuation(
                Punctuation::Comma
            ))]
        );
    }

    #[test]
    fn unknown_lexeme_codec_is_refused() {
        const FIELDS: &[FieldData] = &[FieldData {
            name: "who",
            kind: FieldKindData::Scalar { codec: "Person" },
        }];
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Lexeme("who")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("codec_test", "Internal", true, FIELDS, FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnknownLexemeCodec {
                construction: "codec_test",
                codec: "Person",
            }
        );
    }

    #[test]
    fn mistyped_typed_scalar_pair_is_refused() {
        const FIELDS: &[FieldData] = &[FieldData {
            name: "number",
            kind: FieldKindData::TypedScalar {
                value_type: "DefinitelyNotANumber",
                codec: "Numeral",
            },
        }];
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Lexeme("number")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] = &[construction(
            "mistyped_scalar",
            "Internal",
            true,
            FIELDS,
            FORM,
        )];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedTypedScalar {
                owner: "mistyped_scalar",
                field: "number",
                value_type: "DefinitelyNotANumber",
                codec: "Numeral",
            },
            "a codec alone must not admit a typed scalar that lowering cannot represent",
        );
    }

    #[test]
    fn optional_typed_scalar_is_refused() {
        const FIELDS: &[FieldData] = &[FieldData {
            name: "number",
            kind: FieldKindData::Optional {
                inner: &FieldKindData::TypedScalar {
                    value_type: "QuantityValue",
                    codec: "Numeral",
                },
            },
        }];
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Lexeme("number")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] = &[construction(
            "optional_typed_scalar",
            "Internal",
            true,
            FIELDS,
            FORM,
        )];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedOptionalTypedScalar {
                owner: "optional_typed_scalar",
                field: "number",
                value_type: "QuantityValue",
                codec: "Numeral",
            },
            "optional typed scalars must be rejected until lowering has an adapter",
        );
    }

    #[test]
    fn mistyped_identity_pair_is_refused_before_rules_are_registered() {
        const GOOD_FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Literal("good")],
        }];
        const BAD_FIELDS: &[FieldData] = &[FieldData {
            name: "identity",
            kind: FieldKindData::Identity {
                value_type: "WrongType",
                provider: "KnownNoun",
            },
        }];
        const BAD_FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Identity("identity")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] = &[
            construction("good", "Internal", true, &[], GOOD_FORM),
            construction("mistyped_identity", "Internal", true, BAD_FIELDS, BAD_FORM),
        ];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedIdentityAdapter {
                owner: "mistyped_identity",
                field: "identity",
                value_type: "WrongType",
                provider: "KnownNoun",
            },
            "a provider name alone must not admit an identity that lowering cannot represent",
        );
        assert!(
            builder.finish(RegistrationOrder::Normal).rules.is_empty(),
            "identity rejection must happen before an earlier valid construction mutates the builder",
        );
    }

    #[test]
    fn unknown_identity_provider_has_a_dedicated_pair_diagnostic() {
        const FIELDS: &[FieldData] = &[FieldData {
            name: "identity",
            kind: FieldKindData::Identity {
                value_type: "NounInstance",
                provider: "UnknownNounProvider",
            },
        }];
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Identity("identity")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] = &[construction(
            "unknown_identity_provider",
            "Internal",
            true,
            FIELDS,
            FORM,
        )];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedIdentityAdapter {
                owner: "unknown_identity_provider",
                field: "identity",
                value_type: "NounInstance",
                provider: "UnknownNounProvider",
            },
        );
    }

    #[test]
    fn optional_identity_is_refused_until_lowering_can_build_both_states() {
        const FIELDS: &[FieldData] = &[FieldData {
            name: "identity",
            kind: FieldKindData::Optional {
                inner: &FieldKindData::Identity {
                    value_type: "NounInstance",
                    provider: "KnownNoun",
                },
            },
        }];
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Identity("identity")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] = &[construction(
            "optional_identity",
            "Internal",
            true,
            FIELDS,
            FORM,
        )];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedOptionalIdentityAdapter {
                owner: "optional_identity",
                field: "identity",
                value_type: "NounInstance",
                provider: "KnownNoun",
            },
        );
    }

    #[test]
    fn sequence_hole_registers_element_seed_and_extension_rules() {
        const FIELDS: &[FieldData] = &[FieldData {
            name: "items",
            kind: FieldKindData::Sequence { element: "Foo" },
        }];
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Hole("items")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("seq_test", "Internal", true, FIELDS, FORM)];
        const ELEMENTS: &[ElementData] = &[ElementData {
            name: "Foo",
            bind_path: None,
            fields: &[FieldData {
                name: "phrase",
                kind: FieldKindData::Subtree {
                    category: "NounPhrase",
                    boxed: false,
                },
            }],
            variants: &[],
            erased_builders: &[],
            erased_sequence_builder: None,
        }];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &["Foo"],
            element_data: ELEMENTS,
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, &[&GROUP], &cats)
            .expect("declared sequence elements assemble");
        let book = builder.finish(RegistrationOrder::Normal);
        assert_eq!(book.rules.len(), 4);
        assert_eq!(book.rules[0].lhs, super::super::Nonterminal::Generated(1));
        assert_eq!(
            book.rules[1].rhs,
            vec![Expected::Nonterminal(super::super::Nonterminal::Generated(
                1
            ))]
        );
        assert_eq!(
            book.rules[2].rhs,
            vec![
                Expected::Nonterminal(super::super::Nonterminal::Generated(2)),
                Expected::Nonterminal(super::super::Nonterminal::Generated(1)),
            ]
        );
        assert_eq!(
            book.rules[3].rhs,
            vec![Expected::Nonterminal(super::super::Nonterminal::Generated(
                2
            ))]
        );
    }

    #[test]
    fn dotted_atom_path_is_refused() {
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            erased_recognizer: None,
            atoms: &[AtomData::Hole("members.last")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("dotted", "Internal", true, &[], FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedAtomPath {
                construction: "dotted",
                path: "members.last",
            }
        );
    }

    #[test]
    fn bind_construction_can_register_as_generated() {
        const CONSTRUCTIONS: &[ConstructionData] = &[ConstructionData {
            id: "bound",
            category: "Internal",
            internal: true,
            own_type: None,
            bind_path: Some("x::Y"),
            lens: None,
            projection_variant: None,
            deserialize: false,
            selection_unique: false,
            dominates: &[],
            dominated_by: &[],
            fields: &[],
            witnesses: &[],
            forms: &[],
            requirements: &[],
            recognition_requirements: &[],
            feature_combinators: &[],
            evidence: None,
            erased_partial_builder: None,
            erased_builder: None,
            erased_projector: None,
        }];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, &[&GROUP], &cats)
            .expect("compiler-emitted bind builders are valid generated constructions");
    }

    #[test]
    fn real_coordination_group_assembles_as_generated() {
        let groups = crate::constructions::coordination::GROUPS;
        let cats = internal_categories(groups);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, groups, &cats)
            .expect("every real coordination category and sequence must assemble");
        let book = builder.finish(RegistrationOrder::Normal);
        assert!(
            book.rules.iter().any(|rule| {
                rule.production.construction.as_str() == "noun_phrase_coordination"
            })
        );
        assert!(
            book.rules.iter().any(|rule| {
                rule.production.construction.as_str() == "shared_determiner_nominal"
            })
        );
        assert!(book.rules.iter().all(|rule| {
            rule.production.construction.as_str() != "noun_phrase_coordination"
                || rule.rhs.len() >= 2
        }));
        assert!(book.rules.iter().all(|rule| {
            rule.production.construction.as_str() != "shared_determiner_nominal"
                || rule.rhs.len() >= 3
        }));
    }

    #[test]
    fn nested_predicates_prove_nonempty_sequences_only_when_every_any_arm_does() {
        use deckmaste_construction_compiler::runtime::PredicateData;

        const NONEMPTY: PredicateData = PredicateData::LenAtLeast {
            path: "rest",
            min: 1,
        };
        const OTHER: PredicateData = PredicateData::IsSome { path: "tag" };
        assert!(super::predicate_requires_nonempty(
            PredicateData::All(&[OTHER, NONEMPTY]),
            "rest",
        ));
        assert!(!super::predicate_requires_nonempty(
            PredicateData::Any(&[OTHER, NONEMPTY]),
            "rest",
        ));
        assert!(super::predicate_requires_nonempty(
            PredicateData::Any(&[
                NONEMPTY,
                PredicateData::LenIs {
                    path: "rest",
                    len: 2,
                },
            ]),
            "rest",
        ));
    }
}
