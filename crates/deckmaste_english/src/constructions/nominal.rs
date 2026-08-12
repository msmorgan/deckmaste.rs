//! Compiler-derived declarations for the complete English nominal spine.
//!
//! The chart-facing forms, stable construction identities, and dominance
//! graph live here together.  The bind adapters are deliberately typed over
//! the public syntax nodes: a generated construction cannot manufacture a
//! nominal through an unvalidated field mutation.

#![allow(
    dead_code,
    clippy::unnecessary_wraps,
    reason = "declaration adapters share the generated checked-builder Result signature and are reached through generated function pointers and metadata"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::catalog::CatalogAtom;
use crate::catalog::CatalogKind;
use crate::features::Conjunction;
use crate::features::NounCardinality;
use crate::grammar::AdjectiveComparisonState;
use crate::grammar::Features;
use crate::grammar::reduction::nominal_attributive_adjective_is_admitted;
use crate::grammar::reduction::nominal_prepositional_attachment_is_admitted;
use crate::grammar::reduction::reduce_devotion_color_pair;
use crate::grammar::reduction::reduce_devotion_color_single;
use crate::grammar::reduction::reduce_nominal_adjective;
use crate::grammar::reduction::reduce_nominal_combat_step_name;
use crate::grammar::reduction::reduce_nominal_comparison;
use crate::grammar::reduction::reduce_nominal_determiner;
use crate::grammar::reduction::reduce_nominal_devotion;
use crate::grammar::reduction::reduce_nominal_infinitive;
use crate::grammar::reduction::reduce_nominal_keyword_predicated_argument;
use crate::grammar::reduction::reduce_nominal_keyword_symbol_argument;
use crate::grammar::reduction::reduce_nominal_negated_modifier;
use crate::grammar::reduction::reduce_nominal_noun;
use crate::grammar::reduction::reduce_nominal_noun_modifier;
use crate::grammar::reduction::reduce_nominal_postpositive_adjective;
use crate::grammar::reduction::reduce_nominal_postpositive_adjective_asyndetic;
use crate::grammar::reduction::reduce_nominal_postpositive_adjective_conjoined_prepositional;
use crate::grammar::reduction::reduce_nominal_postpositive_adjective_continuation;
use crate::grammar::reduction::reduce_nominal_power_toughness_modifier;
use crate::grammar::reduction::reduce_nominal_prepositional;
use crate::grammar::reduction::reduce_nominal_quantity_complement;
use crate::grammar::reduction::reduce_nominal_quantity_modifier;
use crate::grammar::reduction::reduce_nominal_reduced_recipient_passive;
use crate::grammar::reduction::reduce_nominal_relative;
use crate::grammar::reduction::reduce_nominal_times_clause;
use crate::grammar::reduction::reduce_predicated_argument_extend;
use crate::grammar::reduction::reduce_predicated_argument_single;
use crate::grammar::reduction::reduce_predicated_quality;
use crate::grammar::reduction::reduce_recipient_passive_nominal_adjunct;
use crate::grammar::reduction::reduce_recipient_passive_theme;
use crate::grammar::reduction::reduce_rules_object_followup_prepositional;
use crate::grammar::reduction::reduce_rules_object_followup_relative;
use crate::grammar::reduction::reduce_rules_object_nominal_base;
use crate::syntax::AdjectiveComplement;
use crate::syntax::AdjectivePhrase;
use crate::syntax::AdjectivePhraseCoordination;
use crate::syntax::ComparisonComplement;
use crate::syntax::CoordinatedAdjectivePhrase;
use crate::syntax::CoordinatedModifier;
use crate::syntax::Determiner;
use crate::syntax::DevotionColors;
use crate::syntax::IndependentClause;
use crate::syntax::InfinitiveClause;
use crate::syntax::KeywordArgument;
use crate::syntax::KeywordCost;
use crate::syntax::NominalComplement;
use crate::syntax::NominalModifier;
use crate::syntax::NominalPhrase;
use crate::syntax::NounPhrase;
use crate::syntax::OracleSymbol;
use crate::syntax::Phrase;
use crate::syntax::Polarity;
use crate::syntax::PowerToughness;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PredicateComplement;
use crate::syntax::PredicateElement;
use crate::syntax::PredicatedArgument;
use crate::syntax::PredicatedQuality;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::Quantity;
use crate::syntax::RelativeClause;
use crate::syntax::TransitivePredicate;
use crate::word::Adjective;
use crate::word::BareNominalAdjunct;
use crate::word::ColorWord;
use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::NounInstanceKind;
use crate::word::Tense;
use crate::word::VerbSlot;
use crate::word::Vocab;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesObjectNominal(NominalPhrase);

impl RulesObjectNominal {
    pub(crate) fn from_nominal(nominal: NominalPhrase) -> Self {
        Self(nominal)
    }

    #[must_use]
    pub fn into_nominal(self) -> NominalPhrase {
        self.0
    }

    #[must_use]
    pub const fn as_nominal(&self) -> &NominalPhrase {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulesObjectFollowupNominal(NominalPhrase);

impl RulesObjectFollowupNominal {
    pub(crate) fn from_nominal(nominal: NominalPhrase) -> Self {
        Self(nominal)
    }

    #[must_use]
    pub fn into_nominal(self) -> NominalPhrase {
        self.0
    }

    #[must_use]
    pub const fn as_nominal(&self) -> &NominalPhrase {
        &self.0
    }
}

pub type PredicatedQualityFrom = PredicatedQuality;
pub type PredicatedQualityBare = PredicatedQuality;
pub type PredicatedArgumentFrom = PredicatedArgument;
pub type PredicatedArgumentBare = PredicatedArgument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedRecipientPassiveTheme(NounPhrase);

impl ReducedRecipientPassiveTheme {
    pub(crate) fn from_noun_phrase(noun_phrase: NounPhrase) -> Self {
        Self(noun_phrase)
    }

    #[must_use]
    pub fn into_noun_phrase(self) -> NounPhrase {
        self.0
    }

    #[must_use]
    pub const fn as_noun_phrase(&self) -> &NounPhrase {
        &self.0
    }
}

pub type SymbolSequence = Vec<OracleSymbol>;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn disprefer_recipient_passive_theme_infinitive(nominal: &Features) -> Option<Features> {
    matches!(
        nominal,
        Features::Nominal {
            recipient_passive_theme: true,
            ..
        }
    )
    .then(|| nominal.clone())
}

fn disprefer_nearer_relative_host(nominal: &Features) -> Option<Features> {
    matches!(
        nominal,
        Features::Nominal {
            attachment: crate::grammar::NominalAttachmentPhase::Prepositional {
                nearer_relative_host: true,
            },
            ..
        }
    )
    .then(|| nominal.clone())
}

fn mark_generated_cost(feature: &Features) -> Option<Features> {
    Some(feature.clone())
}

fn mark_rules_object_relative_attachment(relative: &Features) -> Option<Features> {
    matches!(
        relative,
        Features::RelativeClause {
            gap: crate::features::GapState::Object,
            object_gap_requires_rules_object: true,
            ..
        }
    )
    .then(|| relative.clone())
}

fn mark_rules_object_followup_relative_attachment(
    base: Option<&Features>,
    followup: Option<&Features>,
    relative: &Features,
) -> Option<Features> {
    (base.is_some() != followup.is_some() && matches!(relative, Features::RelativeClause { .. }))
        .then(|| relative.clone())
}

fn mark_rules_object_followup_prepositional_attachment(
    _nominal: &Features,
    preposition: &Features,
) -> Option<Features> {
    matches!(preposition, Features::PrepositionalPhrase { .. }).then(|| preposition.clone())
}

fn admit_recipient_passive_prefix(predicate: &Features) -> Option<Features> {
    matches!(
        predicate,
        Features::VerbPhrase {
            object,
            frame,
            ..
        } if frame.is_recipient_passive()
            && !matches!(object, crate::grammar::PredicateObjectState::None)
    )
    .then(|| predicate.clone())
}

fn prepend_modifier(mut nominal: NominalPhrase, modifier: NominalModifier) -> NominalPhrase {
    nominal.declaration_modifiers_mut().insert(0, modifier);
    nominal
}

pub(crate) fn project_nominal_power_toughness_remainder(
    value: &NominalPhrase,
) -> Option<(NominalPhrase, PowerToughness)> {
    let mut nominal = value.clone();
    let NominalComplement::PowerToughness(stats) = nominal.declaration_complements_mut().pop()?
    else {
        return None;
    };
    Some((nominal, stats))
}

pub(crate) fn project_nominal_coordinated_modifier_remainder(
    value: &NominalPhrase,
) -> Option<(NominalModifier, NominalPhrase)> {
    if value.determiner().is_some()
        || !value
            .complements()
            .iter()
            .all(|complement| matches!(complement, NominalComplement::Relative(_)))
        || !matches!(
            value.modifiers().first(),
            Some(NominalModifier::Coordinated(_))
        )
    {
        return None;
    }
    let mut nominal = value.clone();
    let modifier = nominal.declaration_modifiers_mut().remove(0);
    Some((modifier, nominal))
}

pub(crate) fn build_nominal_power_toughness_complement(
    mut nominal: NominalPhrase,
    stats: PowerToughness,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !general_attachment_base_is_admitted(&nominal) {
        return Err(violation(
            "nominal_power_toughness_complement",
            "the nominal attachment phase admits a power/toughness complement",
        ));
    }
    nominal
        .declaration_complements_mut()
        .push(NominalComplement::PowerToughness(stats));
    Ok(nominal)
}

pub(crate) fn build_nominal_coordinated_modifier(
    coordinated: CoordinatedModifier,
    nominal: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    let repeats_head = std::iter::once(coordinated.first())
        .chain(
            coordinated
                .rest()
                .iter()
                .map(crate::syntax::ModifierCoordination::modifier),
        )
        .any(|modifier| {
            matches!(modifier, NominalModifier::Noun { noun, .. } if noun == nominal.head())
        });
    let relative_owner = nominal.determiner().is_none()
        && !has_postnominal_comparison(&nominal)
        && !nominal.complements().is_empty()
        && nominal
            .complements()
            .iter()
            .all(|complement| matches!(complement, NominalComplement::Relative(_)));
    if !(prefix_is_outermost(&nominal) || relative_owner) || repeats_head {
        return Err(violation(
            "nominal_coordinated_modifier",
            "the coordinated modifier is outermost and does not repeat the nominal head",
        ));
    }
    Ok(prepend_modifier(
        nominal,
        NominalModifier::Coordinated(coordinated),
    ))
}

fn cardinality_accepts(cardinality: NounCardinality, head: &NounInstance) -> bool {
    match cardinality {
        NounCardinality::SingularCount => matches!(head.kind(), NounInstanceKind::Singular(_)),
        NounCardinality::SingularOrMass => {
            matches!(
                head.kind(),
                NounInstanceKind::Singular(_) | NounInstanceKind::Mass(_)
            )
        }
        NounCardinality::PluralCount => matches!(head.kind(), NounInstanceKind::Plural(_)),
        NounCardinality::Mass => matches!(head.kind(), NounInstanceKind::Mass(_)),
        NounCardinality::PluralOrMass => {
            matches!(
                head.kind(),
                NounInstanceKind::Plural(_) | NounInstanceKind::Mass(_)
            )
        }
        NounCardinality::Unconstrained => true,
    }
}

fn is_combat_step_participants(participants: &NounInstance) -> bool {
    matches!(
        participants.kind(),
        NounInstanceKind::Plural(Noun::Agentive(crate::word::Verb::Word(
            Vocab::Attack | Vocab::Block
        )))
    )
}

fn is_combat_step_head(head: &NounInstance) -> bool {
    matches!(
        head.kind(),
        NounInstanceKind::Singular(Noun::Word(Vocab::Step))
    )
}

fn make_nominal_adjective(
    adjective: AdjectivePhrase,
    nominal: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    Ok(prepend_modifier(
        nominal,
        NominalModifier::Adjective {
            polarity: Polarity::Positive,
            phrase: adjective,
        },
    ))
}

fn attributive_adjective_is_admitted(adjective: &AdjectivePhrase) -> bool {
    nominal_attributive_adjective_is_admitted(
        matches!(adjective.head(), Adjective::CardOrientation(_)),
        adjective.degree().is_some(),
    )
}

fn prepositional_nominal_attachment_is_admitted(preposition: &PrepositionalPhrase) -> bool {
    crate::constructions::prepositional::every_prepositional_member(preposition, |member| {
        let by_gerund = member.preposition() == Preposition::By
            && matches!(
                member.object().kind(),
                crate::syntax::PrepositionalObjectKind::GerundClause(_)
            );
        nominal_prepositional_attachment_is_admitted(!by_gerund)
    })
}

fn split_nominal_adjective(value: &NominalPhrase) -> (AdjectivePhrase, NominalPhrase) {
    let mut nominal = value.clone();
    let NominalModifier::Adjective {
        polarity: Polarity::Positive,
        phrase,
    } = nominal.declaration_modifiers_mut().remove(0)
    else {
        unreachable!("nominal_adjective parts require a positive adjective prefix")
    };
    (phrase, nominal)
}

fn make_nominal_noun_modifier(
    noun: NounInstance,
    nominal: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    Ok(prepend_modifier(
        nominal,
        NominalModifier::Noun {
            polarity: Polarity::Positive,
            noun,
        },
    ))
}

fn split_nominal_noun_modifier(value: &NominalPhrase) -> (NounInstance, NominalPhrase) {
    let mut nominal = value.clone();
    let NominalModifier::Noun {
        polarity: Polarity::Positive,
        noun,
    } = nominal.declaration_modifiers_mut().remove(0)
    else {
        unreachable!("nominal_noun_modifier parts require a positive noun prefix")
    };
    (noun, nominal)
}

fn make_nominal_combat_step_name(
    participants: NounInstance,
    head: NounInstance,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !is_combat_step_participants(&participants) {
        return Err(violation(
            "nominal_combat_step_name",
            "participants are plural attackers or blockers",
        ));
    }
    if !is_combat_step_head(&head) {
        return Err(violation(
            "nominal_combat_step_name",
            "head is singular step",
        ));
    }
    Ok(NominalPhrase::from_projection_parts(
        None,
        vec![NominalModifier::CombatStepName { participants }],
        head,
        Vec::new(),
    ))
}

fn split_nominal_combat_step_name(value: &NominalPhrase) -> (NounInstance, NounInstance) {
    let [NominalModifier::CombatStepName { participants }] = value.modifiers() else {
        unreachable!("combat-step nominal has exactly its formative modifier")
    };
    (participants.clone(), value.head().clone())
}

fn make_nominal_quantity_modifier(
    quantity: Quantity,
    nominal: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !cardinality_accepts(quantity.noun_cardinality(), nominal.head()) {
        return Err(violation(
            "nominal_quantity_modifier",
            "quantity cardinality accepts the nominal head",
        ));
    }
    Ok(prepend_modifier(
        nominal,
        NominalModifier::Quantity(quantity),
    ))
}

fn split_nominal_quantity_modifier(value: &NominalPhrase) -> (Quantity, NominalPhrase) {
    let mut nominal = value.clone();
    let NominalModifier::Quantity(quantity) = nominal.declaration_modifiers_mut().remove(0) else {
        unreachable!("quantity-modified nominal has a quantity prefix")
    };
    (quantity, nominal)
}

fn make_nominal_power_toughness_modifier(
    stats: PowerToughness,
    nominal: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    Ok(prepend_modifier(
        nominal,
        NominalModifier::PowerToughness(stats),
    ))
}

fn split_nominal_power_toughness_modifier(
    value: &NominalPhrase,
) -> (PowerToughness, NominalPhrase) {
    let mut nominal = value.clone();
    let NominalModifier::PowerToughness(stats) = nominal.declaration_modifiers_mut().remove(0)
    else {
        unreachable!("power/toughness-modified nominal has a stats prefix")
    };
    (stats, nominal)
}

macro_rules! nominal_complement_adapter {
    ($make:ident, $parts:ident, $ty:ty, $variant:ident, $id:literal) => {
        fn $make(
            mut nominal: NominalPhrase,
            complement: $ty,
        ) -> Result<NominalPhrase, DeclarationViolation> {
            if !general_attachment_base_is_admitted(&nominal) {
                return Err(violation(
                    $id,
                    "the nominal attachment phase admits a general complement",
                ));
            }
            nominal
                .declaration_complements_mut()
                .push(NominalComplement::$variant(complement));
            Ok(nominal)
        }

        fn $parts(value: &NominalPhrase) -> (NominalPhrase, $ty) {
            let mut nominal = value.clone();
            let Some(NominalComplement::$variant(complement)) =
                nominal.declaration_complements_mut().pop()
            else {
                unreachable!(concat!($id, " parts require their final complement"))
            };
            (nominal, complement)
        }
    };
}

nominal_complement_adapter!(
    make_nominal_prepositional,
    split_nominal_prepositional,
    PrepositionalPhrase,
    Prepositional,
    "nominal_prepositional"
);
nominal_complement_adapter!(
    make_nominal_infinitive,
    split_nominal_infinitive,
    crate::syntax::InfinitiveClause,
    Infinitive,
    "nominal_infinitive"
);
nominal_complement_adapter!(
    make_nominal_quantity_complement,
    split_nominal_quantity_complement,
    Quantity,
    Quantity,
    "nominal_quantity_complement"
);
nominal_complement_adapter!(
    make_nominal_relative,
    split_nominal_relative,
    RelativeClause,
    Relative,
    "nominal_relative"
);
fn reduced_recipient_passive_theme_nominal_is_admitted(nominal: &NominalPhrase) -> bool {
    nominal.determiner().is_none()
        && nominal.modifiers().is_empty()
        && nominal.complements().is_empty()
        && matches!(
            nominal.head().kind(),
            NounInstanceKind::Mass(Noun::Word(Vocab::Damage))
        )
}

fn reduced_recipient_passive_theme_is_admitted(theme: &crate::syntax::PredicateObject) -> bool {
    let crate::syntax::PredicateObject::NounPhrase(noun_phrase) = theme else {
        return false;
    };
    matches!(
        noun_phrase.kind(),
        crate::syntax::NounPhraseKind::Nominal(nominal)
            if reduced_recipient_passive_theme_nominal_is_admitted(nominal)
    )
}

fn reduced_recipient_passive_predicate_is_admitted(predicate: &TransitivePredicate) -> bool {
    predicate.head.auxiliaries.is_empty()
        && predicate.head.preverb_modifiers.is_empty()
        && predicate.head.verb.slot == VerbSlot::PastParticiple
        && predicate
            .head
            .verb
            .verb
            .predicate_frames()
            .iter()
            .any(|frame| frame.is_recipient_passive())
        && reduced_recipient_passive_theme_is_admitted(&predicate.kind.object)
        && predicate
            .kind
            .pre_object_elements
            .iter()
            .chain(&predicate.elements)
            .all(|element| {
                !matches!(
                    element,
                    PredicateElement::Complement(PredicateComplement::IndirectObject(_))
                )
            })
}

fn make_nominal_reduced_recipient_passive(
    mut nominal: NominalPhrase,
    predicate: TransitivePredicate,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !general_attachment_base_is_admitted(&nominal) {
        return Err(violation(
            "nominal_reduced_recipient_passive",
            "the nominal attachment phase admits a reduced relative",
        ));
    }
    if !reduced_recipient_passive_predicate_is_admitted(&predicate) {
        return Err(violation(
            "nominal_reduced_recipient_passive",
            "predicate is a reduced recipient-passive participle with a retained theme",
        ));
    }
    nominal
        .declaration_complements_mut()
        .push(NominalComplement::ReducedRecipientPassive(predicate));
    Ok(nominal)
}

fn split_nominal_reduced_recipient_passive(
    value: &NominalPhrase,
) -> (NominalPhrase, TransitivePredicate) {
    let mut nominal = value.clone();
    let Some(NominalComplement::ReducedRecipientPassive(predicate)) =
        nominal.declaration_complements_mut().pop()
    else {
        unreachable!("reduced-recipient-passive parts require their final complement")
    };
    (nominal, predicate)
}
fn make_nominal_postpositive_adjective(
    mut nominal: NominalPhrase,
    adjective: AdjectivePhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !nominal.complements().is_empty() || has_postnominal_comparison(&nominal) {
        return Err(violation(
            "nominal_postpositive_adjective",
            "the nominal attachment phase is open",
        ));
    }
    nominal
        .declaration_complements_mut()
        .push(NominalComplement::Adjective(adjective));
    Ok(nominal)
}

fn split_nominal_postpositive_adjective(value: &NominalPhrase) -> (NominalPhrase, AdjectivePhrase) {
    let mut nominal = value.clone();
    let Some(NominalComplement::Adjective(adjective)) = nominal.declaration_complements_mut().pop()
    else {
        unreachable!("postpositive-adjective parts require their final complement")
    };
    (nominal, adjective)
}

fn make_nominal_keyword_symbol_argument(
    head: NounInstance,
    symbol: Option<OracleSymbol>,
    sequence: Option<Vec<OracleSymbol>>,
) -> Result<NominalPhrase, DeclarationViolation> {
    if keyword_atom_from_head(&head).is_none() {
        return Err(violation(
            "nominal_keyword_symbol_argument",
            "head is a mass keyword-ability noun",
        ));
    }
    let symbols = match (symbol, sequence) {
        (Some(symbol), None) => vec![symbol],
        (None, Some(symbols)) if !symbols.is_empty() => symbols,
        _ => {
            return Err(violation(
                "nominal_keyword_symbol_argument",
                "exactly one nonempty symbol argument is present",
            ));
        }
    };
    Ok(keyword_nominal(
        head,
        KeywordArgument::Costed(KeywordCost::Symbols(symbols)),
    ))
}

fn split_nominal_keyword_symbol_argument(
    value: &NominalPhrase,
) -> (
    NounInstance,
    Option<OracleSymbol>,
    Option<Vec<OracleSymbol>>,
) {
    let [
        NominalComplement::KeywordArgument(KeywordArgument::Costed(KeywordCost::Symbols(symbols))),
    ] = value.complements()
    else {
        unreachable!("symbol keyword nominal carries one symbol-cost argument")
    };
    if let [symbol] = symbols.as_slice() {
        (value.head().clone(), Some(symbol.clone()), None)
    } else {
        (value.head().clone(), None, Some(symbols.clone()))
    }
}

fn keyword_nominal(head: NounInstance, argument: KeywordArgument) -> NominalPhrase {
    NominalPhrase::from_projection_parts(
        None,
        Vec::new(),
        head,
        vec![NominalComplement::KeywordArgument(argument)],
    )
}

fn keyword_atom_from_head(head: &NounInstance) -> Option<&CatalogAtom> {
    let NounInstanceKind::Mass(Noun::Catalog(atom)) = head.kind() else {
        return None;
    };
    (atom.kind == CatalogKind::KeywordAbility).then_some(atom)
}

fn make_predicated_quality_from(
    preposition: Preposition,
    color: Option<ColorWord>,
    adjective: Option<AdjectivePhrase>,
    noun_phrase: Option<NounPhrase>,
) -> Result<PredicatedQuality, DeclarationViolation> {
    if !matches!(preposition, Preposition::From | Preposition::For) {
        return Err(violation(
            "predicated_quality_from",
            "introduced quality preposition is from or for",
        ));
    }
    let quality = match (color, adjective, noun_phrase) {
        (Some(color), None, None) => Phrase::ColorWord(color),
        (None, Some(adjective), None) => Phrase::AdjectivePhrase(Box::new(adjective)),
        (None, None, Some(noun_phrase)) => Phrase::NounPhrase(Box::new(noun_phrase)),
        _ => {
            return Err(violation(
                "predicated_quality_from",
                "exactly one introduced quality is present",
            ));
        }
    };
    Ok(PredicatedQuality {
        preposition: Some(preposition),
        quality,
    })
}

fn split_predicated_quality_from(
    value: &PredicatedQuality,
) -> (
    Preposition,
    Option<ColorWord>,
    Option<AdjectivePhrase>,
    Option<NounPhrase>,
) {
    let preposition = value
        .preposition
        .expect("introduced predicated quality carries its preposition");
    match &value.quality {
        Phrase::ColorWord(color) => (preposition, Some(*color), None, None),
        Phrase::AdjectivePhrase(adjective) => {
            (preposition, None, Some((**adjective).clone()), None)
        }
        Phrase::NounPhrase(noun_phrase) => (preposition, None, None, Some((**noun_phrase).clone())),
        _ => unreachable!("introduced quality has a declared phrase shape"),
    }
}

fn is_declared_predicated_preposition(preposition: Option<Preposition>) -> bool {
    matches!(preposition, Some(Preposition::From | Preposition::For))
}

/// The ability-line parser can preserve `of` in a comma-split proper-name
/// fragment such as `Trynn, Champion of Freedom`. That is not a declared chart
/// production, but it is an existing source-free AST that the generated
/// predicated-quality inverse must remain total over.  Keep the chart/build
/// door closed to the declared `from`/`for` grammar while admitting this one
/// measured inverse-only spelling.
fn is_linearizable_predicated_preposition(preposition: Option<Preposition>) -> bool {
    matches!(
        preposition,
        Some(Preposition::From | Preposition::For | Preposition::Of)
    )
}

fn make_predicated_quality_bare(
    color: Option<ColorWord>,
    adjective: Option<AdjectivePhrase>,
    noun_phrase: Option<NounPhrase>,
) -> Result<PredicatedQuality, DeclarationViolation> {
    let quality = match (color, adjective, noun_phrase) {
        (Some(color), None, None) => Phrase::ColorWord(color),
        (None, Some(adjective), None) => Phrase::AdjectivePhrase(Box::new(adjective)),
        (None, None, Some(noun_phrase)) => Phrase::NounPhrase(Box::new(noun_phrase)),
        _ => {
            return Err(violation(
                "predicated_quality_bare",
                "exactly one bare quality is present",
            ));
        }
    };
    Ok(PredicatedQuality {
        preposition: None,
        quality,
    })
}

fn split_predicated_quality_bare(
    value: &PredicatedQuality,
) -> (
    Option<ColorWord>,
    Option<AdjectivePhrase>,
    Option<NounPhrase>,
) {
    match &value.quality {
        Phrase::ColorWord(color) => (Some(*color), None, None),
        Phrase::AdjectivePhrase(adjective) => (None, Some((**adjective).clone()), None),
        Phrase::NounPhrase(noun_phrase) => (None, None, Some((**noun_phrase).clone())),
        _ => unreachable!("bare predicated quality has a declared quality shape"),
    }
}

fn make_predicated_argument_from_single(
    quality: PredicatedQuality,
) -> Result<PredicatedArgument, DeclarationViolation> {
    if !is_declared_predicated_preposition(quality.preposition) {
        return Err(violation(
            "predicated_argument_from_single",
            "quality carries an explicit from or for preposition",
        ));
    }
    Ok(PredicatedArgument {
        qualities: vec![quality],
    })
}

fn make_predicated_argument_bare_single(
    quality: PredicatedQuality,
) -> Result<PredicatedArgument, DeclarationViolation> {
    if quality.preposition.is_some() {
        return Err(violation(
            "predicated_argument_bare_single",
            "first quality is bare",
        ));
    }
    Ok(PredicatedArgument {
        qualities: vec![quality],
    })
}

fn split_predicated_argument_single(value: &PredicatedArgument) -> PredicatedQuality {
    value.qualities[0].clone()
}

fn make_predicated_argument_from_extend(
    mut argument: PredicatedArgument,
    conjunction: Conjunction,
    quality: PredicatedQuality,
) -> Result<PredicatedArgument, DeclarationViolation> {
    if conjunction != Conjunction::And {
        return Err(violation(
            "predicated_argument_from_extend",
            "predicated keyword qualities extend only with and",
        ));
    }
    if argument.qualities.is_empty()
        || !argument
            .qualities
            .iter()
            .chain(std::iter::once(&quality))
            .all(|quality| is_declared_predicated_preposition(quality.preposition))
    {
        return Err(violation(
            "predicated_argument_from_extend",
            "every quality carries an explicit from or for preposition",
        ));
    }
    argument.qualities.push(quality);
    Ok(argument)
}

fn make_predicated_argument_bare_extend(
    mut argument: PredicatedArgument,
    conjunction: Conjunction,
    quality: PredicatedQuality,
) -> Result<PredicatedArgument, DeclarationViolation> {
    if conjunction != Conjunction::And {
        return Err(violation(
            "predicated_argument_bare_extend",
            "predicated keyword qualities extend only with and",
        ));
    }
    if argument
        .qualities
        .first()
        .is_none_or(|quality| quality.preposition.is_some())
        || argument
            .qualities
            .iter()
            .skip(1)
            .chain(std::iter::once(&quality))
            .any(|quality| quality.preposition != Some(Preposition::From))
    {
        return Err(violation(
            "predicated_argument_bare_extend",
            "one bare quality is followed only by explicit-from qualities",
        ));
    }
    argument.qualities.push(quality);
    Ok(argument)
}

fn split_predicated_argument_extend(
    value: &PredicatedArgument,
) -> (PredicatedArgument, Conjunction, PredicatedQuality) {
    let mut argument = value.clone();
    let quality = argument
        .qualities
        .pop()
        .expect("extended predicated argument carries a final quality");
    (argument, Conjunction::And, quality)
}

fn make_nominal_keyword_predicated_argument(
    head: NounInstance,
    argument: PredicatedArgument,
) -> Result<NominalPhrase, DeclarationViolation> {
    let Some(atom) = keyword_atom_from_head(&head) else {
        return Err(violation(
            "nominal_keyword_predicated_argument",
            "head is a mass keyword-ability noun",
        ));
    };
    if crate::grammar::keyword_atom_carries_from(atom)
        || argument.qualities.is_empty()
        || !argument
            .qualities
            .iter()
            .all(|quality| quality.preposition == Some(Preposition::From))
    {
        return Err(violation(
            "nominal_keyword_predicated_argument",
            "ordinary keyword head takes nonempty explicit-from qualities",
        ));
    }
    Ok(keyword_nominal(head, KeywordArgument::Predicated(argument)))
}

fn make_nominal_keyword_atom_carried_predicated_argument(
    head: NounInstance,
    argument: PredicatedArgument,
) -> Result<NominalPhrase, DeclarationViolation> {
    let Some(atom) = keyword_atom_from_head(&head) else {
        return Err(violation(
            "nominal_keyword_atom_carried_predicated_argument",
            "head is a mass keyword-ability noun",
        ));
    };
    if !crate::grammar::keyword_atom_carries_from(atom)
        || argument
            .qualities
            .first()
            .is_none_or(|quality| quality.preposition.is_some())
        || argument
            .qualities
            .iter()
            .skip(1)
            .any(|quality| quality.preposition != Some(Preposition::From))
    {
        return Err(violation(
            "nominal_keyword_atom_carried_predicated_argument",
            "from-carrying keyword head takes one bare quality then explicit-from qualities",
        ));
    }
    Ok(keyword_nominal(head, KeywordArgument::Predicated(argument)))
}

fn split_nominal_keyword_predicated_argument(
    value: &NominalPhrase,
) -> (NounInstance, PredicatedArgument) {
    let [NominalComplement::KeywordArgument(KeywordArgument::Predicated(argument))] =
        value.complements()
    else {
        unreachable!("predicated keyword nominal carries one predicated argument")
    };
    (value.head().clone(), argument.clone())
}

fn make_rules_object_nominal_base(
    nominal: NominalPhrase,
) -> Result<RulesObjectNominal, DeclarationViolation> {
    Ok(RulesObjectNominal::from_nominal(nominal))
}

fn split_rules_object_nominal_base(value: &RulesObjectNominal) -> NominalPhrase {
    value.0.clone()
}

fn nominal_has_rules_object_relative_edge(value: &NominalPhrase) -> bool {
    matches!(value.complements().last(), Some(NominalComplement::Relative(relative)) if matches!(
        relative.body(),
        crate::syntax::RelativeBody::ObjectGap { predicate, .. } if matches!(
            predicate.head.verb.verb,
            crate::word::Verb::Word(Vocab::Control | Vocab::Own)
        )
    ))
}

fn make_rules_object_followup_nominal_relative(
    base: Option<RulesObjectNominal>,
    followup: Option<RulesObjectFollowupNominal>,
    relative: RelativeClause,
) -> Result<RulesObjectFollowupNominal, DeclarationViolation> {
    let mut nominal = match (base, followup) {
        (Some(base), None) => base.into_nominal(),
        (None, Some(followup)) => followup.into_nominal(),
        _ => {
            return Err(violation(
                "rules_object_followup_nominal_relative",
                "exactly one rules-object source is present",
            ));
        }
    };
    nominal
        .declaration_complements_mut()
        .push(NominalComplement::Relative(relative));
    Ok(RulesObjectFollowupNominal::from_nominal(nominal))
}

fn split_rules_object_followup_nominal_relative(
    value: &RulesObjectFollowupNominal,
) -> (
    Option<RulesObjectNominal>,
    Option<RulesObjectFollowupNominal>,
    RelativeClause,
) {
    let mut nominal = value.0.clone();
    let Some(NominalComplement::Relative(relative)) = nominal.declaration_complements_mut().pop()
    else {
        unreachable!("rules-object followup ends in a relative")
    };
    if nominal_has_rules_object_relative_edge(&nominal) {
        (
            Some(RulesObjectNominal::from_nominal(nominal)),
            None,
            relative,
        )
    } else {
        (
            None,
            Some(RulesObjectFollowupNominal::from_nominal(nominal)),
            relative,
        )
    }
}

fn make_rules_object_followup_nominal_prepositional(
    nominal: RulesObjectFollowupNominal,
    preposition: PrepositionalPhrase,
) -> Result<RulesObjectFollowupNominal, DeclarationViolation> {
    let mut nominal = nominal.into_nominal();
    nominal
        .declaration_complements_mut()
        .push(NominalComplement::Prepositional(preposition));
    Ok(RulesObjectFollowupNominal::from_nominal(nominal))
}

fn split_rules_object_followup_nominal_prepositional(
    value: &RulesObjectFollowupNominal,
) -> (RulesObjectFollowupNominal, PrepositionalPhrase) {
    let mut nominal = value.0.clone();
    let Some(NominalComplement::Prepositional(preposition)) =
        nominal.declaration_complements_mut().pop()
    else {
        unreachable!("rules-object followup ends in a prepositional phrase")
    };
    (
        RulesObjectFollowupNominal::from_nominal(nominal),
        preposition,
    )
}

fn make_reduced_recipient_passive_theme(
    nominal: NominalPhrase,
) -> Result<ReducedRecipientPassiveTheme, DeclarationViolation> {
    if !reduced_recipient_passive_theme_nominal_is_admitted(&nominal) {
        return Err(violation(
            "reduced_recipient_passive_theme",
            "theme nominal is headed by damage",
        ));
    }
    Ok(ReducedRecipientPassiveTheme::from_noun_phrase(
        NounPhrase::from_nominal_declaration(nominal),
    ))
}

fn split_reduced_recipient_passive_theme(value: &ReducedRecipientPassiveTheme) -> NominalPhrase {
    let crate::syntax::NounPhraseKind::Nominal(nominal) = value.0.kind() else {
        unreachable!("recipient-passive theme is nominal")
    };
    nominal.clone()
}

fn make_reduced_recipient_passive_nominal_adjunct(
    mut predicate: TransitivePredicate,
    noun_phrase: NounPhrase,
) -> Result<TransitivePredicate, DeclarationViolation> {
    if !reduced_recipient_passive_predicate_is_admitted(&predicate) {
        return Err(violation(
            "reduced_recipient_passive_nominal_adjunct",
            "predicate is a reduced recipient-passive participle with a retained theme",
        ));
    }
    let crate::syntax::NounPhraseKind::Nominal(nominal) = noun_phrase.kind() else {
        return Err(violation(
            "reduced_recipient_passive_nominal_adjunct",
            "adjunct is a nominal noun phrase",
        ));
    };
    let adjunct = match nominal.head().noun().bare_nominal_adjunct() {
        Some(BareNominalAdjunct::Temporal) => PredicateAdjunct::Temporal(noun_phrase),
        Some(BareNominalAdjunct::Manner) => PredicateAdjunct::Manner(noun_phrase),
        None => {
            return Err(violation(
                "reduced_recipient_passive_nominal_adjunct",
                "noun licenses a bare nominal adjunct",
            ));
        }
    };
    predicate.elements.push(PredicateElement::Adjunct(adjunct));
    Ok(predicate)
}

fn split_reduced_recipient_passive_nominal_adjunct(
    value: &TransitivePredicate,
) -> (TransitivePredicate, NounPhrase) {
    let mut predicate = value.clone();
    let Some(PredicateElement::Adjunct(
        PredicateAdjunct::Temporal(noun_phrase) | PredicateAdjunct::Manner(noun_phrase),
    )) = predicate.elements.pop()
    else {
        unreachable!("recipient-passive adjunct is the final predicate element")
    };
    (predicate, noun_phrase)
}

fn noun_phrase_conjunction(conjunction: Conjunction) -> Result<Conjunction, DeclarationViolation> {
    matches!(
        conjunction,
        Conjunction::And | Conjunction::Or | Conjunction::AndOr
    )
    .then_some(conjunction)
    .ok_or_else(|| {
        violation(
            "nominal_postpositive_adjective_conjoined",
            "postpositive adjective conjunction is and, or, or and/or",
        )
    })
}

fn append_postpositive(
    mut nominal: NominalPhrase,
    conjunction: Option<Conjunction>,
    adjective: AdjectivePhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    let previous = nominal.declaration_complements_mut().pop().ok_or_else(|| {
        violation(
            "nominal_postpositive_adjective_conjoined",
            "a preceding postpositive adjective exists",
        )
    })?;
    let continuation = AdjectivePhraseCoordination::from_declaration_parts(conjunction, adjective);
    let coordinated = match previous {
        NominalComplement::Adjective(first) => {
            CoordinatedAdjectivePhrase::from_declaration_parts(Box::new(first), vec![continuation])
        }
        NominalComplement::CoordinatedAdjective(coordinated) => {
            let (first, mut rest) = coordinated.into_declaration_parts();
            rest.push(continuation);
            CoordinatedAdjectivePhrase::from_declaration_parts(first, rest)
        }
        _ => {
            return Err(violation(
                "nominal_postpositive_adjective_conjoined",
                "a preceding postpositive adjective exists",
            ));
        }
    };
    nominal
        .declaration_complements_mut()
        .push(NominalComplement::CoordinatedAdjective(coordinated));
    Ok(nominal)
}

fn make_nominal_postpositive_adjective_conjoined_prepositional(
    nominal: NominalPhrase,
    conjunction: Conjunction,
    adjective: AdjectivePhrase,
    preposition: PrepositionalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !matches!(nominal.complements(), [NominalComplement::Adjective(_)]) {
        return Err(violation(
            "nominal_postpositive_adjective_conjoined_prepositional",
            "a binary continuation follows exactly one postpositive adjective",
        ));
    }
    if !adjective.complements().is_empty()
        || !matches!(adjective.head(), Adjective::Participle(Tense::Past, _))
        || preposition.head().preposition != Preposition::By
    {
        return Err(violation(
            "nominal_postpositive_adjective_conjoined_prepositional",
            "the continuation is a bare past participle followed by a by phrase",
        ));
    }
    let conjunction = noun_phrase_conjunction(conjunction)?;
    let adjective = adjective
        .try_attach_declared_prepositional(preposition)
        .ok_or_else(|| {
            violation(
                "nominal_postpositive_adjective_conjoined_prepositional",
                "the checked adjective owner accepts the by phrase",
            )
        })?;
    append_postpositive(nominal, Some(conjunction), adjective)
}

fn split_nominal_postpositive_adjective_conjoined_prepositional(
    value: &NominalPhrase,
) -> (
    NominalPhrase,
    Conjunction,
    AdjectivePhrase,
    PrepositionalPhrase,
) {
    let (nominal, conjunction, adjective) = split_postpositive(value, true);
    let (adjective, preposition) = adjective
        .try_split_declared_prepositional()
        .expect("conjoined participle carries its checked by phrase");
    (
        nominal,
        conjunction.expect("conjoined continuation carries a conjunction"),
        adjective,
        preposition,
    )
}

fn make_nominal_postpositive_adjective_conjoined(
    nominal: NominalPhrase,
    conjunction: Conjunction,
    adjective: AdjectivePhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !matches!(nominal.complements(), [NominalComplement::Adjective(_)]) {
        return Err(violation(
            "nominal_postpositive_adjective_conjoined",
            "a binary continuation follows exactly one postpositive adjective",
        ));
    }
    append_postpositive(
        nominal,
        Some(noun_phrase_conjunction(conjunction)?),
        adjective,
    )
}

fn split_nominal_postpositive_adjective_conjoined(
    value: &NominalPhrase,
) -> (NominalPhrase, Conjunction, AdjectivePhrase) {
    let (nominal, conjunction, adjective) = split_postpositive(value, true);
    (
        nominal,
        conjunction.expect("conjoined continuation carries a conjunction"),
        adjective,
    )
}

fn make_nominal_postpositive_adjective_asyndetic(
    nominal: NominalPhrase,
    adjective: AdjectivePhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    let extends_open_run = match nominal.complements() {
        [NominalComplement::Adjective(_)] => true,
        [NominalComplement::CoordinatedAdjective(coordinated)] => coordinated
            .rest()
            .last()
            .is_some_and(|member| member.conjunction().is_none()),
        _ => false,
    };
    if !extends_open_run {
        return Err(violation(
            "nominal_postpositive_adjective_asyndetic",
            "an asyndetic continuation extends an open postpositive run",
        ));
    }
    append_postpositive(nominal, None, adjective)
}

fn split_nominal_postpositive_adjective_asyndetic(
    value: &NominalPhrase,
) -> (NominalPhrase, AdjectivePhrase) {
    let (nominal, _, adjective) = split_postpositive(value, false);
    (nominal, adjective)
}

fn make_nominal_postpositive_adjective_oxford(
    nominal: NominalPhrase,
    conjunction: Conjunction,
    adjective: AdjectivePhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !matches!(
        nominal.complements(),
        [NominalComplement::CoordinatedAdjective(coordinated)]
            if !coordinated.rest().is_empty()
                && coordinated
                    .rest()
                    .iter()
                    .all(|member| member.conjunction().is_none())
    ) {
        return Err(violation(
            "nominal_postpositive_adjective_oxford",
            "an Oxford final extends a nonempty asyndetic postpositive run",
        ));
    }
    append_postpositive(
        nominal,
        Some(noun_phrase_conjunction(conjunction)?),
        adjective,
    )
}

fn split_nominal_postpositive_adjective_oxford(
    value: &NominalPhrase,
) -> (NominalPhrase, Conjunction, AdjectivePhrase) {
    split_nominal_postpositive_adjective_conjoined(value)
}

fn split_postpositive(
    value: &NominalPhrase,
    conjoined: bool,
) -> (NominalPhrase, Option<Conjunction>, AdjectivePhrase) {
    let mut nominal = value.clone();
    let Some(NominalComplement::CoordinatedAdjective(coordinated)) =
        nominal.declaration_complements_mut().pop()
    else {
        unreachable!("postpositive continuation ends in coordinated adjectives")
    };
    let (first, mut rest) = coordinated.into_declaration_parts();
    let continuation = rest
        .pop()
        .expect("coordinated adjective has a continuation");
    let (conjunction, phrase) = continuation.into_declaration_parts();
    assert_eq!(conjunction.is_some(), conjoined);
    nominal
        .declaration_complements_mut()
        .push(if rest.is_empty() {
            NominalComplement::Adjective(*first)
        } else {
            NominalComplement::CoordinatedAdjective(
                CoordinatedAdjectivePhrase::from_declaration_parts(first, rest),
            )
        });
    (nominal, conjunction, phrase)
}

fn make_nominal_comparison(
    mut nominal: NominalPhrase,
    comparison: ComparisonComplement,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !has_only_prepositional_complements(&nominal) {
        return Err(violation(
            "nominal_comparison",
            "comparison follows only an open or prepositional nominal",
        ));
    }
    let adjective = nominal
        .declaration_modifiers_mut()
        .iter_mut()
        .rev()
        .find_map(|modifier| {
            let NominalModifier::Adjective { phrase, .. } = modifier else {
                return None;
            };
            adjective_is_pending_comparative(phrase).then_some(phrase)
        });
    let Some(adjective) = adjective else {
        return Err(violation(
            "nominal_comparison",
            "a pending adjective accepts the comparison",
        ));
    };
    let attached = adjective
        .clone()
        .try_attach_postnominal_comparison(comparison)
        .ok_or_else(|| {
            violation(
                "nominal_comparison",
                "the checked adjective owner accepts the postnominal comparison",
            )
        })?;
    *adjective = attached;
    Ok(nominal)
}

fn split_nominal_comparison(value: &NominalPhrase) -> (NominalPhrase, ComparisonComplement) {
    let mut nominal = value.clone();
    let adjective = nominal
        .declaration_modifiers_mut()
        .iter_mut()
        .rev()
        .find_map(|modifier| match modifier {
            NominalModifier::Adjective { phrase, .. }
                if adjective_has_declared_postnominal_comparison(phrase) =>
            {
                Some(phrase)
            }
            _ => None,
        })
        .expect("comparison nominal has an adjective modifier");
    let (owner, comparison) = adjective
        .clone()
        .try_split_postnominal_comparison()
        .expect("comparison nominal stores one checked postnominal comparison");
    *adjective = owner;
    (nominal, comparison)
}

fn adjective_is_pending_comparative(phrase: &AdjectivePhrase) -> bool {
    matches!(
        crate::grammar::adjective_comparison_state(phrase.head()),
        AdjectiveComparisonState::Pending(_)
    ) && !phrase.complements().iter().any(|complement| {
        matches!(
            complement,
            AdjectiveComplement::Comparison(_) | AdjectiveComplement::PostnominalComparison(_)
        )
    })
}

fn adjective_has_declared_postnominal_comparison(phrase: &AdjectivePhrase) -> bool {
    matches!(
        crate::grammar::adjective_comparison_state(phrase.head()),
        AdjectiveComparisonState::Pending(_)
    ) && matches!(
        phrase.complements(),
        [AdjectiveComplement::PostnominalComparison(_)]
    )
}

fn make_devotion_color_single(color: ColorWord) -> Result<DevotionColors, DeclarationViolation> {
    Ok(DevotionColors::Color(color))
}

#[allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "generated bind destructurers uniformly receive a shared reference to their target"
)]
fn split_devotion_color_single(value: &DevotionColors) -> ColorWord {
    let DevotionColors::Color(color) = value else {
        unreachable!("single devotion color has one color")
    };
    *color
}

fn make_devotion_color_pair(
    first: ColorWord,
    conjunction: Conjunction,
    second: ColorWord,
) -> Result<DevotionColors, DeclarationViolation> {
    if conjunction != Conjunction::And {
        return Err(violation(
            "devotion_color_pair",
            "devotion colors are joined by and",
        ));
    }
    Ok(DevotionColors::Pair(first, second))
}

#[allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "generated bind destructurers uniformly receive a shared reference to their target"
)]
fn split_devotion_color_pair(value: &DevotionColors) -> (ColorWord, Conjunction, ColorWord) {
    let DevotionColors::Pair(first, second) = value else {
        unreachable!("paired devotion colors have two colors")
    };
    (*first, Conjunction::And, *second)
}

fn make_nominal_devotion(
    head: CatalogAtom,
    colors: DevotionColors,
) -> Result<NominalPhrase, DeclarationViolation> {
    if head.kind != CatalogKind::RulesBundle || head.canonical() != "devotion" {
        return Err(violation(
            "nominal_devotion",
            "head is the rules-defined devotion value",
        ));
    }
    Ok(NominalPhrase::from_projection_parts(
        None,
        Vec::new(),
        NounInstance::unchecked_singular(Noun::Catalog(head)),
        vec![NominalComplement::Devotion(colors)],
    ))
}

fn split_nominal_devotion(value: &NominalPhrase) -> (CatalogAtom, DevotionColors) {
    let Noun::Catalog(head) = value.head().noun() else {
        unreachable!("devotion nominal has its catalog head")
    };
    let [NominalComplement::Devotion(colors)] = value.complements() else {
        unreachable!("devotion nominal has its color complement")
    };
    (head.clone(), *colors)
}

fn make_nominal_times_clause(
    head: NounInstance,
    clause: Box<IndependentClause>,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !matches!(
        head.kind(),
        NounInstanceKind::Plural(Noun::Word(Vocab::Time))
    ) {
        return Err(violation("nominal_times_clause", "head is plural times"));
    }
    Ok(NominalPhrase::from_projection_parts(
        None,
        Vec::new(),
        head,
        vec![NominalComplement::EventClause(clause)],
    ))
}

fn split_nominal_times_clause(value: &NominalPhrase) -> (NounInstance, Box<IndependentClause>) {
    let [NominalComplement::EventClause(clause)] = value.complements() else {
        unreachable!("times nominal carries one event clause")
    };
    (value.head().clone(), clause.clone())
}

fn has_postnominal_comparison(value: &NominalPhrase) -> bool {
    value.modifiers().iter().any(|modifier| {
        matches!(
            modifier,
            NominalModifier::Adjective { phrase, .. }
                if phrase
                    .complements()
                    .iter()
                    .any(|complement| matches!(complement, AdjectiveComplement::PostnominalComparison(_)))
        )
    })
}

fn general_attachment_base_is_admitted(value: &NominalPhrase) -> bool {
    !has_postnominal_comparison(value)
        && !matches!(
            value.complements().last(),
            Some(
                NominalComplement::ReducedRecipientPassive(_)
                    | NominalComplement::Adjective(_)
                    | NominalComplement::CoordinatedAdjective(_)
            )
        )
}

fn prefix_before_final_allows_general_attachment(value: &NominalPhrase) -> bool {
    !has_postnominal_comparison(value)
        && !matches!(
            value
                .complements()
                .get(..value.complements().len().saturating_sub(1))
                .and_then(|prefix| prefix.last()),
            Some(
                NominalComplement::ReducedRecipientPassive(_)
                    | NominalComplement::Adjective(_)
                    | NominalComplement::CoordinatedAdjective(_)
            )
        )
}

fn has_only_prepositional_complements(value: &NominalPhrase) -> bool {
    value
        .complements()
        .iter()
        .all(|complement| matches!(complement, NominalComplement::Prepositional(_)))
}

fn is_devotion_body(value: &NominalPhrase) -> bool {
    matches!(
        (value.head().kind(), value.complements()),
        (
            NounInstanceKind::Singular(Noun::Catalog(atom)),
            [NominalComplement::Devotion(_)]
        ) if atom.kind == CatalogKind::RulesBundle && atom.canonical() == "devotion"
    )
}

fn is_times_clause_body(value: &NominalPhrase) -> bool {
    matches!(
        (value.head().kind(), value.complements()),
        (
            NounInstanceKind::Plural(Noun::Word(Vocab::Time)),
            [NominalComplement::EventClause(_)]
        )
    )
}

fn prefix_is_outermost(value: &NominalPhrase) -> bool {
    value.determiner().is_none()
        && !has_postnominal_comparison(value)
        && (value.complements().is_empty()
            || is_devotion_body(value)
            || is_times_clause_body(value))
}

fn is_nominal_adjective(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers().first(),
            Some(NominalModifier::Adjective {
                polarity: Polarity::Positive,
                phrase,
            }) if attributive_adjective_is_admitted(phrase)
        )
}

fn is_nominal_noun_modifier(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers().first(),
            Some(NominalModifier::Noun {
                polarity: Polarity::Positive,
                ..
            })
        )
}

fn is_nominal_combat_step_name(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers(),
            [NominalModifier::CombatStepName { participants }]
                if is_combat_step_participants(participants)
        )
        && is_combat_step_head(value.head())
}

fn is_nominal_negated_modifier(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers().first(),
            Some(
                NominalModifier::Adjective {
                    polarity: Polarity::Negative,
                    ..
                } | NominalModifier::Noun {
                    polarity: Polarity::Negative,
                    ..
                }
            )
        )
}

fn is_nominal_quantity_modifier(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers().first(),
            Some(NominalModifier::Quantity(quantity))
                if cardinality_accepts(quantity.noun_cardinality(), value.head())
        )
}

fn is_nominal_power_toughness_modifier(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers().first(),
            Some(NominalModifier::PowerToughness(_))
        )
}

fn is_nominal_determiner(value: &NominalPhrase) -> bool {
    value
        .determiner()
        .as_ref()
        .is_some_and(|determiner| cardinality_accepts(determiner.noun_cardinality(), value.head()))
        && !has_postnominal_comparison(value)
        && (value.complements().is_empty()
            || is_devotion_body(value)
            || is_times_clause_body(value))
}

fn final_complement(value: &NominalPhrase) -> Option<&NominalComplement> {
    (!has_postnominal_comparison(value))
        .then(|| value.complements().last())
        .flatten()
}

fn is_nominal_prepositional(value: &NominalPhrase) -> bool {
    prefix_before_final_allows_general_attachment(value)
        && matches!(
            final_complement(value),
            Some(NominalComplement::Prepositional(preposition))
                if prepositional_nominal_attachment_is_admitted(preposition)
        )
}

fn is_nominal_infinitive(value: &NominalPhrase) -> bool {
    prefix_before_final_allows_general_attachment(value)
        && matches!(
            final_complement(value),
            Some(NominalComplement::Infinitive(_))
        )
}

fn is_nominal_quantity_complement(value: &NominalPhrase) -> bool {
    prefix_before_final_allows_general_attachment(value)
        && matches!(
            final_complement(value),
            Some(NominalComplement::Quantity(_))
        )
}

fn keyword_atom(value: &NominalPhrase) -> Option<&CatalogAtom> {
    keyword_atom_from_head(value.head())
}

fn is_nominal_keyword_symbol_single(value: &NominalPhrase) -> bool {
    keyword_atom(value).is_some()
        && value.determiner().is_none()
        && value.modifiers().is_empty()
        && matches!(
            value.complements(),
            [NominalComplement::KeywordArgument(KeywordArgument::Costed(
                KeywordCost::Symbols(symbols)
            ))] if symbols.len() == 1
        )
}

fn is_nominal_keyword_symbol_sequence(value: &NominalPhrase) -> bool {
    keyword_atom(value).is_some()
        && value.determiner().is_none()
        && value.modifiers().is_empty()
        && matches!(
            value.complements(),
            [NominalComplement::KeywordArgument(KeywordArgument::Costed(
                KeywordCost::Symbols(symbols)
            ))] if symbols.len() >= 2
        )
}

fn is_nominal_keyword_predicated_argument(value: &NominalPhrase) -> bool {
    keyword_atom(value).is_some_and(|atom| !crate::grammar::keyword_atom_carries_from(atom))
        && value.determiner().is_none()
        && value.modifiers().is_empty()
        && matches!(
            value.complements(),
            [NominalComplement::KeywordArgument(KeywordArgument::Predicated(argument))]
                if !argument.qualities.is_empty()
                    && argument.qualities.iter().all(|quality| quality.preposition == Some(Preposition::From))
        )
}

fn is_nominal_keyword_atom_carried_predicated_argument(value: &NominalPhrase) -> bool {
    keyword_atom(value).is_some_and(crate::grammar::keyword_atom_carries_from)
        && value.determiner().is_none()
        && value.modifiers().is_empty()
        && matches!(
            value.complements(),
            [NominalComplement::KeywordArgument(KeywordArgument::Predicated(argument))]
                if !argument.qualities.is_empty()
                    && argument.qualities.first().is_some_and(|quality| quality.preposition.is_none())
                    && argument.qualities.iter().skip(1).all(|quality| quality.preposition == Some(Preposition::From))
        )
}

fn is_nominal_relative(value: &NominalPhrase) -> bool {
    prefix_before_final_allows_general_attachment(value)
        && matches!(
            final_complement(value),
            Some(NominalComplement::Relative(_))
        )
}

fn is_nominal_reduced_recipient_passive(value: &NominalPhrase) -> bool {
    prefix_before_final_allows_general_attachment(value)
        && matches!(
            final_complement(value),
            Some(NominalComplement::ReducedRecipientPassive(predicate))
                if reduced_recipient_passive_predicate_is_admitted(predicate)
        )
}

fn final_coordinated_adjective(value: &NominalPhrase) -> Option<&CoordinatedAdjectivePhrase> {
    let Some(NominalComplement::CoordinatedAdjective(coordinated)) = final_complement(value) else {
        return None;
    };
    Some(coordinated)
}

fn is_nominal_postpositive_adjective(value: &NominalPhrase) -> bool {
    matches!(value.complements(), [NominalComplement::Adjective(_)])
}

fn is_nominal_postpositive_adjective_conjoined_prepositional(value: &NominalPhrase) -> bool {
    value.complements().len() == 1
        && final_coordinated_adjective(value).is_some_and(|coordinated| {
            coordinated.rest().len() == 1
                && coordinated.rest().last().is_some_and(|last| {
                    last.conjunction().is_some()
                        && matches!(
                            (last.phrase().head(), last.phrase().complements()),
                            (
                                Adjective::Participle(Tense::Past, _),
                                [AdjectiveComplement::Prepositional(preposition)]
                            ) if preposition.head().preposition == Preposition::By
                        )
                })
        })
}

fn is_nominal_postpositive_adjective_conjoined(value: &NominalPhrase) -> bool {
    value.complements().len() == 1
        && final_coordinated_adjective(value).is_some_and(|coordinated| {
            coordinated.rest().len() == 1
                && coordinated.rest()[0].conjunction().is_some()
                && !matches!(
                    coordinated.rest()[0].phrase().complements().last(),
                    Some(AdjectiveComplement::Prepositional(_))
                )
        })
}

fn is_nominal_postpositive_adjective_asyndetic(value: &NominalPhrase) -> bool {
    value.complements().len() == 1
        && final_coordinated_adjective(value)
            .and_then(|coordinated| coordinated.rest().last())
            .is_some_and(|last| last.conjunction().is_none())
}

fn is_nominal_postpositive_adjective_oxford(value: &NominalPhrase) -> bool {
    value.complements().len() == 1
        && final_coordinated_adjective(value).is_some_and(|coordinated| {
            coordinated.rest().len() >= 2
                && coordinated
                    .rest()
                    .last()
                    .is_some_and(|last| last.conjunction().is_some())
        })
}

fn is_nominal_comparison(value: &NominalPhrase) -> bool {
    has_only_prepositional_complements(value)
        && value
            .modifiers()
            .iter()
            .filter_map(|modifier| match modifier {
                NominalModifier::Adjective { phrase, .. }
                    if adjective_has_declared_postnominal_comparison(phrase) =>
                {
                    Some(())
                }
                _ => None,
            })
            .count()
            == 1
}

fn is_nominal_devotion(value: &NominalPhrase) -> bool {
    value.determiner().is_none() && value.modifiers().is_empty() && is_devotion_body(value)
}

fn is_nominal_times_clause(value: &NominalPhrase) -> bool {
    value.determiner().is_none() && value.modifiers().is_empty() && is_times_clause_body(value)
}

deckmaste_constructions_macro::constructions! {
    group nominal;

    lens nominal_phrase bind NominalPhrase via NominalPhrase::from_projection_parts, NominalPhrase::into_projection_parts {
        determiner: opt Determiner,
        modifiers: vec NominalModifier,
        head: value NounInstance,
        complements: vec NominalComplement,
    }

    construction nominal_noun: NominalPhrase {
        bind NominalPhrase {
            head: hole NounInstance,
        }
        lens nominal_phrase {
            focus head with head;
        }
        derive features: Features = reduce_nominal_noun(head);
        form only @ 0 = head;
        selection unique;
    }

    construction nominal_adjective: NominalPhrase {
        bind NominalPhrase via make_nominal_adjective, split_nominal_adjective {
            adjective: hole AdjectivePhrase,
            nominal: hole NominalPhrase,
        }
        derive features: Features = reduce_nominal_adjective(adjective, nominal);
        derive base_precedence: Features = mark_generated_cost(adjective);
        form only @ 0 inverse check(is_nominal_adjective) = adjective nominal;
        selection unique;
    }

    construction nominal_noun_modifier: NominalPhrase {
        bind NominalPhrase via make_nominal_noun_modifier, split_nominal_noun_modifier {
            noun: hole NounInstance,
            nominal: hole NominalPhrase,
        }
        derive features: Features = reduce_nominal_noun_modifier(noun, nominal);
        derive base_precedence: Features = mark_generated_cost(noun);
        form only @ 0 inverse check(is_nominal_noun_modifier) = noun nominal;
        selection unique;
    }

    construction nominal_combat_step_name: NominalPhrase {
        bind NominalPhrase via make_nominal_combat_step_name, split_nominal_combat_step_name {
            participants: identity NounInstance via CombatStepParticipants,
            head: identity NounInstance via CombatStepHead,
        }
        derive features: Features = reduce_nominal_combat_step_name(participants, head);
        form only @ 0 inverse check(is_nominal_combat_step_name) = "declare" identity(participants) identity(head);
        selection unique;
    }

    construction nominal_negated_modifier: NominalPhrase {
        bind NominalPhrase {
            modifier: identity NominalModifier via NegatedModifier,
            nominal: hole NominalPhrase,
        }
        lens nominal_phrase from nominal {
            prepend modifiers with modifier;
        }
        derive features: Features = reduce_nominal_negated_modifier(nominal);
        derive base_precedence: Features = mark_generated_cost(nominal);
        form only @ 0 inverse check(is_nominal_negated_modifier) = identity(modifier) nominal;
        selection unique;
    }

    construction nominal_quantity_modifier: NominalPhrase {
        bind NominalPhrase via make_nominal_quantity_modifier, split_nominal_quantity_modifier {
            quantity: hole Quantity,
            nominal: hole NominalPhrase,
        }
        derive features: Features = reduce_nominal_quantity_modifier(quantity, nominal);
        derive base_precedence: Features = mark_generated_cost(quantity);
        form only @ 0 inverse check(is_nominal_quantity_modifier) = quantity nominal;
        dominates nominal_prepositional;
        dominates nominal_postpositive_adjective;
        dominates nominal_comparison;
        selection unique;
    }

    construction nominal_power_toughness_modifier: NominalPhrase {
        bind NominalPhrase via make_nominal_power_toughness_modifier, split_nominal_power_toughness_modifier {
            stats: hole PowerToughness,
            nominal: hole NominalPhrase,
        }
        derive features: Features = reduce_nominal_power_toughness_modifier(stats, nominal);
        derive base_precedence: Features = mark_generated_cost(stats);
        form only @ 0 inverse check(is_nominal_power_toughness_modifier) = stats nominal;
        selection unique;
    }

    construction nominal_determiner: NominalPhrase {
        bind NominalPhrase {
            determiner: hole Determiner,
            nominal: hole NominalPhrase,
        }
        lens nominal_phrase from nominal {
            focus determiner with determiner;
        }
        derive features: Features = reduce_nominal_determiner(determiner, nominal);
        form only @ 0 inverse check(is_nominal_determiner) = determiner nominal;
        dominates nominal_comparison;
        selection unique;
    }

    construction nominal_prepositional: NominalPhrase {
        bind NominalPhrase via make_nominal_prepositional, split_nominal_prepositional {
            nominal: hole NominalPhrase,
            preposition: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_nominal_prepositional(nominal, preposition);
        evidence feature "nominal attachment phase" from output attachment;
        form only @ 0 inverse check(is_nominal_prepositional) = nominal preposition;
        dominates nominal_infinitive;
        dominates nominal_coordinated_modifier;
        dominates nominal_keyword_predicated_argument;
        dominates nominal_noun;
        selection unique;
    }

    construction nominal_infinitive: NominalPhrase {
        bind NominalPhrase via make_nominal_infinitive, split_nominal_infinitive {
            nominal: hole NominalPhrase,
            infinitive: hole InfinitiveClause,
        }
        derive features: Features = reduce_nominal_infinitive(nominal, infinitive);
        derive precedence: Features = disprefer_recipient_passive_theme_infinitive(nominal);
        form only @ 0 inverse check(is_nominal_infinitive) = nominal infinitive;
        selection unique;
    }

    construction nominal_quantity_complement: NominalPhrase {
        bind NominalPhrase via make_nominal_quantity_complement, split_nominal_quantity_complement {
            nominal: hole NominalPhrase,
            quantity: hole Quantity,
        }
        derive features: Features = reduce_nominal_quantity_complement(nominal, quantity);
        derive base_precedence: Features = mark_generated_cost(quantity);
        form only @ 0 inverse check(is_nominal_quantity_complement) = nominal quantity;
        selection unique;
    }

    construction nominal_keyword_symbol_argument: NominalPhrase {
        bind NominalPhrase via make_nominal_keyword_symbol_argument, split_nominal_keyword_symbol_argument {
            head: identity NounInstance via SymbolArgumentKeywordNoun,
            symbol: opt lex OracleSymbol,
            sequence: opt lex SymbolSequence,
        }
        derive features: Features = reduce_nominal_keyword_symbol_argument(head);
        form symbol @ 0 when symbol.is_some() inverse check(is_nominal_keyword_symbol_single) = identity(head) lex(symbol);
        form sequence @ 1 inverse check(is_nominal_keyword_symbol_sequence) otherwise = identity(head) lex(sequence);
        selection unique;
    }

    construction predicated_quality_from: PredicatedQualityFrom {
        bind PredicatedQuality via make_predicated_quality_from, split_predicated_quality_from {
            preposition: lex Preposition,
            color: opt lex ColorWord,
            adjective: opt hole AdjectivePhrase,
            noun_phrase: opt hole NounPhrase,
        }
        require preposition in [From, For];
        derive features: Features = reduce_predicated_quality();
        form color @ 0 when color.is_some() inverse check(is_linearizable_predicated_quality_introduced) = lex(preposition) lex(color);
        form adjective @ 1 when all(color.is_none(), adjective.is_some()) inverse check(is_linearizable_predicated_quality_introduced) = lex(preposition) adjective;
        form noun_phrase @ 2 inverse check(is_linearizable_predicated_quality_introduced) otherwise = lex(preposition) noun_phrase;
        selection unique;
    }

    construction predicated_argument_from_single: PredicatedArgumentFrom {
        bind PredicatedArgument via make_predicated_argument_from_single, split_predicated_argument_single {
            quality: hole PredicatedQualityFrom,
        }
        derive features: Features = reduce_predicated_argument_single(quality);
        form only @ 0 inverse check(is_predicated_argument_from_single) = quality;
        selection unique;
    }

    construction predicated_argument_from_extend: PredicatedArgumentFrom {
        bind PredicatedArgument via make_predicated_argument_from_extend, split_predicated_argument_extend {
            argument: hole PredicatedArgumentFrom,
            conjunction: lex Conjunction,
            quality: hole PredicatedQualityFrom,
        }
        require conjunction in [And];
        evidence guard "keyword-grant conjunction gate" from requirement conjunction;
        derive features: Features = reduce_predicated_argument_extend(argument, conjunction, quality);
        form only @ 0 inverse check(is_predicated_argument_from_extend) = argument lex(conjunction) quality;
        selection unique;
    }

    construction nominal_keyword_predicated_argument: NominalPhrase {
        bind NominalPhrase via make_nominal_keyword_predicated_argument, split_nominal_keyword_predicated_argument {
            head: identity NounInstance via ExplicitPredicatedKeywordNoun,
            argument: hole PredicatedArgumentFrom,
        }
        derive features: Features = reduce_nominal_keyword_predicated_argument(head, argument);
        form only @ 0 inverse check(is_nominal_keyword_predicated_argument) = identity(head) argument;
        selection unique;
    }

    construction predicated_quality_bare: PredicatedQualityBare {
        bind PredicatedQuality via make_predicated_quality_bare, split_predicated_quality_bare {
            color: opt lex ColorWord,
            adjective: opt hole AdjectivePhrase,
            noun_phrase: opt hole NounPhrase,
        }
        derive features: Features = reduce_predicated_quality();
        form color @ 0 when color.is_some() inverse check(is_declared_predicated_quality_bare) = lex(color);
        form adjective @ 1 when all(color.is_none(), adjective.is_some()) inverse check(is_declared_predicated_quality_bare) = adjective;
        form noun_phrase @ 2 inverse check(is_declared_predicated_quality_bare) otherwise = noun_phrase;
        selection unique;
    }

    construction predicated_argument_bare_single: PredicatedArgumentBare {
        bind PredicatedArgument via make_predicated_argument_bare_single, split_predicated_argument_single {
            quality: hole PredicatedQualityBare,
        }
        derive features: Features = reduce_predicated_argument_single(quality);
        form only @ 0 inverse check(is_predicated_argument_bare_single) = quality;
        selection unique;
    }

    construction predicated_argument_bare_extend: PredicatedArgumentBare {
        bind PredicatedArgument via make_predicated_argument_bare_extend, split_predicated_argument_extend {
            argument: hole PredicatedArgumentBare,
            conjunction: lex Conjunction,
            quality: hole PredicatedQualityFrom,
        }
        require conjunction in [And];
        evidence guard "keyword-grant conjunction gate" from requirement conjunction;
        derive features: Features = reduce_predicated_argument_extend(argument, conjunction, quality);
        form only @ 0 inverse check(is_predicated_argument_bare_extend) = argument lex(conjunction) quality;
        selection unique;
    }

    construction nominal_keyword_atom_carried_predicated_argument: NominalPhrase {
        bind NominalPhrase via make_nominal_keyword_atom_carried_predicated_argument, split_nominal_keyword_predicated_argument {
            head: identity NounInstance via AtomCarriedPredicatedKeywordNoun,
            argument: hole PredicatedArgumentBare,
        }
        derive features: Features = reduce_nominal_keyword_predicated_argument(head, argument);
        form only @ 0 inverse check(is_nominal_keyword_atom_carried_predicated_argument) = identity(head) argument;
        selection unique;
    }

    construction nominal_relative: NominalPhrase {
        bind NominalPhrase via make_nominal_relative, split_nominal_relative {
            nominal: hole NominalPhrase,
            relative: hole RelativeClause,
        }
        derive features: Features = reduce_nominal_relative(nominal, relative);
        derive precedence: Features = disprefer_nearer_relative_host(nominal);
        derive attachment_distance: Features = mark_rules_object_relative_attachment(relative);
        form only @ 0 inverse check(is_nominal_relative) = nominal relative;
        dominates nominal_prepositional;
        selection unique;
    }

    construction rules_object_nominal_base: RulesObjectNominal {
        bind RulesObjectNominal via make_rules_object_nominal_base, split_rules_object_nominal_base {
            nominal: hole NominalPhrase,
        }
        derive features: Features = reduce_rules_object_nominal_base(nominal);
        evidence role "rules-object attachment role" from category;
        form only @ 0 = nominal;
        selection unique;
    }

    construction rules_object_followup_nominal_relative: RulesObjectFollowupNominal {
        bind RulesObjectFollowupNominal via make_rules_object_followup_nominal_relative, split_rules_object_followup_nominal_relative {
            base: opt hole RulesObjectNominal,
            followup: opt hole RulesObjectFollowupNominal,
            relative: hole RelativeClause,
        }
        derive features: Features = reduce_rules_object_followup_relative(base, followup, relative);
        derive attachment_extent: Features = mark_rules_object_followup_relative_attachment(base, followup, relative);
        evidence role "rules-object attachment role" from category;
        form base @ 0 when base.is_some() = base relative;
        form followup @ 1 otherwise = followup relative;
        selection unique;
    }

    construction rules_object_followup_nominal_prepositional: RulesObjectFollowupNominal {
        bind RulesObjectFollowupNominal via make_rules_object_followup_nominal_prepositional, split_rules_object_followup_nominal_prepositional {
            nominal: hole RulesObjectFollowupNominal,
            preposition: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_rules_object_followup_prepositional(nominal, preposition);
        derive attachment_extent: Features = mark_rules_object_followup_prepositional_attachment(nominal, preposition);
        evidence role "rules-object attachment role" from category;
        form only @ 0 = nominal preposition;
        selection unique;
    }

    construction nominal_reduced_recipient_passive: NominalPhrase {
        bind NominalPhrase via make_nominal_reduced_recipient_passive, split_nominal_reduced_recipient_passive {
            nominal: hole NominalPhrase,
            predicate: hole TransitivePredicate,
        }
        derive features: Features = reduce_nominal_reduced_recipient_passive(nominal, predicate);
        evidence guard "reduced-recipient-passive frame" from field predicate.frame;
        form only @ 0 inverse check(is_nominal_reduced_recipient_passive) = nominal predicate;
        dominates nominal_noun;
        selection unique;
    }

    construction reduced_recipient_passive_theme: ReducedRecipientPassiveTheme {
        bind ReducedRecipientPassiveTheme via make_reduced_recipient_passive_theme, split_reduced_recipient_passive_theme {
            nominal: hole NominalPhrase,
        }
        derive features: Features = reduce_recipient_passive_theme(nominal);
        form only @ 0 = nominal;
        selection unique;
    }

    construction reduced_recipient_passive_nominal_adjunct: TransitivePredicate {
        bind TransitivePredicate via make_reduced_recipient_passive_nominal_adjunct, split_reduced_recipient_passive_nominal_adjunct {
            predicate: hole TransitivePredicate,
            noun_phrase: hole NounPhrase,
        }
        derive features: Features = reduce_recipient_passive_nominal_adjunct(predicate, noun_phrase);
        derive prefix_admission: Features = admit_recipient_passive_prefix(predicate);
        derive base_attachment_count: Features = mark_generated_cost(noun_phrase);
        evidence guard "reduced-recipient-passive frame" from field predicate.frame;
        form only @ 0 = predicate noun_phrase;
        selection unique;
    }

    construction nominal_postpositive_adjective: NominalPhrase {
        bind NominalPhrase via make_nominal_postpositive_adjective, split_nominal_postpositive_adjective {
            nominal: hole NominalPhrase,
            adjective: hole AdjectivePhrase,
        }
        derive features: Features = reduce_nominal_postpositive_adjective(nominal, adjective);
        form only @ 0 inverse check(is_nominal_postpositive_adjective) = nominal adjective;
        selection unique;
    }

    construction nominal_postpositive_adjective_conjoined_prepositional: NominalPhrase {
        bind NominalPhrase via make_nominal_postpositive_adjective_conjoined_prepositional, split_nominal_postpositive_adjective_conjoined_prepositional {
            nominal: hole NominalPhrase,
            conjunction: lex Conjunction,
            adjective: hole AdjectivePhrase,
            preposition: hole PrepositionalPhrase,
        }
        derive features: Features = reduce_nominal_postpositive_adjective_conjoined_prepositional(nominal, conjunction, adjective, preposition);
        derive base_attachment_count: Features = mark_generated_cost(preposition);
        form only @ 0 inverse check(is_nominal_postpositive_adjective_conjoined_prepositional) = nominal lex(conjunction) adjective preposition;
        selection unique;
    }

    construction nominal_postpositive_adjective_conjoined: NominalPhrase {
        bind NominalPhrase via make_nominal_postpositive_adjective_conjoined, split_nominal_postpositive_adjective_conjoined {
            nominal: hole NominalPhrase,
            conjunction: lex Conjunction,
            adjective: hole AdjectivePhrase,
        }
        derive features: Features = reduce_nominal_postpositive_adjective_continuation(nominal, conjunction, adjective);
        form only @ 0 inverse check(is_nominal_postpositive_adjective_conjoined) = nominal lex(conjunction) adjective;
        selection unique;
    }

    construction nominal_postpositive_adjective_asyndetic: NominalPhrase {
        bind NominalPhrase via make_nominal_postpositive_adjective_asyndetic, split_nominal_postpositive_adjective_asyndetic {
            nominal: hole NominalPhrase,
            adjective: hole AdjectivePhrase,
        }
        derive features: Features = reduce_nominal_postpositive_adjective_asyndetic(nominal, adjective);
        form only @ 0 inverse check(is_nominal_postpositive_adjective_asyndetic) = nominal "," adjective;
        selection unique;
    }

    construction nominal_postpositive_adjective_oxford: NominalPhrase {
        bind NominalPhrase via make_nominal_postpositive_adjective_oxford, split_nominal_postpositive_adjective_oxford {
            nominal: hole NominalPhrase,
            conjunction: lex Conjunction,
            adjective: hole AdjectivePhrase,
        }
        derive features: Features = reduce_nominal_postpositive_adjective_continuation(nominal, conjunction, adjective);
        form only @ 0 inverse check(is_nominal_postpositive_adjective_oxford) = nominal "," lex(conjunction) adjective;
        selection unique;
    }

    construction nominal_comparison: NominalPhrase {
        bind NominalPhrase via make_nominal_comparison, split_nominal_comparison {
            nominal: hole NominalPhrase,
            comparison: hole ComparisonComplement,
        }
        derive features: Features = reduce_nominal_comparison(nominal, comparison);
        form only @ 0 inverse check(is_nominal_comparison) = nominal comparison;
        selection unique;
    }

    construction nominal_devotion: NominalPhrase {
        bind NominalPhrase via make_nominal_devotion, split_nominal_devotion {
            head: identity CatalogAtom via DevotionValue,
            colors: hole DevotionColors,
        }
        derive features: Features = reduce_nominal_devotion(head, colors);
        form only @ 0 inverse check(is_nominal_devotion) = identity(head) "to" colors;
        selection unique;
    }

    construction devotion_color_single: DevotionColors {
        bind DevotionColors via make_devotion_color_single, split_devotion_color_single {
            color: lex ColorWord,
        }
        derive features: Features = reduce_devotion_color_single(color);
        form only @ 0 inverse check(is_devotion_color_single) = lex(color);
        selection unique;
    }

    construction devotion_color_pair: DevotionColors {
        bind DevotionColors via make_devotion_color_pair, split_devotion_color_pair {
            first: lex ColorWord,
            conjunction: lex Conjunction,
            second: lex ColorWord,
        }
        require conjunction in [And];
        derive features: Features = reduce_devotion_color_pair(first, conjunction, second);
        form only @ 0 inverse check(is_devotion_color_pair) = lex(first) lex(conjunction) lex(second);
        selection unique;
    }

    construction nominal_times_clause: NominalPhrase {
        bind NominalPhrase via make_nominal_times_clause, split_nominal_times_clause {
            head: identity NounInstance via TimesNoun,
            clause: hole box IndependentClause,
        }
        derive features: Features = reduce_nominal_times_clause(head, clause);
        form only @ 0 inverse check(is_nominal_times_clause) = identity(head) clause;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&NOMINAL_DECLARATION];

fn is_linearizable_predicated_quality_introduced(value: &PredicatedQuality) -> bool {
    is_linearizable_predicated_preposition(value.preposition)
        && matches!(
            value.quality,
            Phrase::ColorWord(_) | Phrase::AdjectivePhrase(_) | Phrase::NounPhrase(_)
        )
}

fn is_declared_predicated_quality_bare(value: &PredicatedQuality) -> bool {
    value.preposition.is_none()
        && matches!(
            value.quality,
            Phrase::ColorWord(_) | Phrase::AdjectivePhrase(_) | Phrase::NounPhrase(_)
        )
}

fn is_predicated_argument_from_single(value: &PredicatedArgument) -> bool {
    value.qualities.len() == 1
        && value
            .qualities
            .iter()
            .all(is_linearizable_predicated_quality_introduced)
}

fn is_predicated_argument_from_extend(value: &PredicatedArgument) -> bool {
    value.qualities.len() >= 2
        && value
            .qualities
            .iter()
            .all(is_linearizable_predicated_quality_introduced)
}

fn is_predicated_argument_bare_single(value: &PredicatedArgument) -> bool {
    value.qualities.len() == 1
        && value
            .qualities
            .first()
            .is_some_and(is_declared_predicated_quality_bare)
}

fn is_predicated_argument_bare_extend(value: &PredicatedArgument) -> bool {
    let Some((first, rest)) = value.qualities.split_first() else {
        return false;
    };
    !rest.is_empty()
        && is_declared_predicated_quality_bare(first)
        && rest
            .iter()
            .all(is_linearizable_predicated_quality_introduced)
}

#[allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "generated inverse checks receive borrowed declaration fields"
)]
const fn is_devotion_color_single(value: &DevotionColors) -> bool {
    matches!(value, DevotionColors::Color(_))
}

#[allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "generated inverse checks receive borrowed declaration fields"
)]
const fn is_devotion_color_pair(value: &DevotionColors) -> bool {
    matches!(value, DevotionColors::Pair(..))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word::Adjective;

    #[test]
    fn declaration_metadata_names_every_nominal_builder_and_feature_projection() {
        // Mutations caught: omit or rename an atomic nominal row, retain a row
        // without its checked builder, or fall back to the former shared
        // `nominal` ID dispatcher instead of a declaration-selected callback.
        assert_eq!(NOMINAL_DECLARATION.constructions.len(), 37);
        assert!(
            NOMINAL_DECLARATION
                .constructions
                .iter()
                .all(|construction| {
                    let mut projections = construction
                        .feature_combinators
                        .iter()
                        .filter(|projection| projection.target == "features");
                    let Some(projection) = projections.next() else {
                        return false;
                    };
                    if projections.next().is_some() {
                        return false;
                    }
                    construction.erased_builder.is_some()
                        && construction.bind_path.is_some()
                        && projection.combinator.starts_with("reduce_")
                        && projection.combinator != "nominal"
                        && projection.args.iter().all(|argument| {
                            construction
                                .fields
                                .iter()
                                .any(|field| field.name == *argument)
                        })
                })
        );
    }

    #[test]
    fn declaration_typed_precedence_callbacks_own_nominal_cost_gates() {
        fn nominal(recipient_passive_theme: bool) -> Features {
            crate::grammar::reduction::reduce_nominal_noun(&Features::Noun {
                identity: None,
                coordination_domain: None,
                form: crate::grammar::NounForm::Singular,
                initial_sound: crate::features::Onset::Consonant,
                adjunct: None,
                opaque: false,
                recipient_passive_theme,
            })
            .expect("noun features produce nominal features")
        }

        let infinitive = NOMINAL_DECLARATION
            .constructions
            .iter()
            .position(|construction| construction.id == "nominal_infinitive")
            .expect("infinitive declaration");
        let passive_theme = nominal(true);
        let ordinary = nominal(false);
        assert!(reduce_nominal_precedence(infinitive, &[Some(&passive_theme), None]).is_some());
        assert!(reduce_nominal_precedence(infinitive, &[Some(&ordinary), None]).is_none());

        let relative = NOMINAL_DECLARATION
            .constructions
            .iter()
            .position(|construction| construction.id == "nominal_relative")
            .expect("relative declaration");
        let nearer_host = crate::grammar::reduction::reduce_nominal_prepositional(
            &ordinary,
            &Features::PrepositionalPhrase {
                preposition: Preposition::Of,
                nominal_attachment: true,
                role_members: vec![crate::grammar::PrepositionalRoleMember {
                    preposition: Preposition::Of,
                    nominal_attachment: true,
                }],
                shared_determiner_object: false,
                nearer_relative_host: true,
            },
        )
        .expect("preposition opens a nearer relative host");
        assert!(reduce_nominal_precedence(relative, &[Some(&nearer_host), None]).is_some());
        assert!(reduce_nominal_precedence(relative, &[Some(&ordinary), None]).is_none());
    }

    #[test]
    fn flattened_nominal_owner_uses_task_one_lenses() {
        // Mutations caught: restore hand-written head construction, optional
        // determiner replacement, or ordered modifier insertion instead of
        // rebuilding the same NominalPhrase through typed lens metadata.
        let lens_ids = NOMINAL_DECLARATION
            .constructions
            .iter()
            .filter(|construction| construction.lens.is_some())
            .map(|construction| construction.id)
            .collect::<Vec<_>>();
        assert_eq!(
            lens_ids,
            [
                "nominal_noun",
                "nominal_negated_modifier",
                "nominal_determiner"
            ]
        );
        assert_eq!(NOMINAL_DECLARATION.lenses.len(), 1);
        assert_eq!(NOMINAL_DECLARATION.lenses[0].owner_type, "NominalPhrase");
    }

    #[derive(Default)]
    struct ConstructionRecorder {
        construction: Option<&'static str>,
    }

    impl deckmaste_construction_compiler::runtime::LinearizationVisitor for ConstructionRecorder {
        type Error = std::convert::Infallible;

        fn begin_form(
            &mut self,
            construction: &'static str,
            _form: &'static str,
            _ordinal: u16,
        ) -> Result<(), Self::Error> {
            self.construction = Some(construction);
            Ok(())
        }

        fn literal(&mut self, _literal: &'static str) -> Result<(), Self::Error> {
            Ok(())
        }

        fn subtree<T: std::any::Any>(
            &mut self,
            _category: &'static str,
            _value: &T,
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn scalar<T: std::any::Any>(
            &mut self,
            _codec: &'static str,
            _value: &T,
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn identity<T: std::any::Any>(
            &mut self,
            _provider: &'static str,
            _value_type: &'static str,
            _value: &T,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    fn card_nominal() -> NominalPhrase {
        build_nominal_noun(NounInstance::unchecked_singular(Noun::Word(Vocab::Card)))
            .expect("the generated head lens admits a renderable card noun")
    }

    fn selected_nominal_construction(value: &NominalPhrase) -> &'static str {
        let mut recorder = ConstructionRecorder::default();
        linearize_nominal_group_with(value, &mut recorder)
            .expect("the direct AST has exactly one declaration-selected inverse");
        recorder
            .construction
            .expect("the selected generated form begins before visiting fields")
    }

    fn assert_nominal_inverse_rejects(value: &NominalPhrase) {
        let mut recorder = ConstructionRecorder::default();
        assert!(matches!(
            linearize_nominal_group_with(value, &mut recorder),
            Err(
                deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                    group: "nominal"
                }
            )
        ));
        assert_eq!(
            recorder.construction, None,
            "inverse validation must finish before visitor side effects"
        );
    }

    fn red_adjective() -> AdjectivePhrase {
        crate::adjective::build_adjective_phrase(Adjective::Color(ColorWord::Red))
            .expect("red is a lexical adjective")
    }

    fn parsed_nominal(source: &str) -> NominalPhrase {
        parsed_nominal_with_catalogs(source, &crate::Catalogs::default())
    }

    fn parsed_nominal_with_catalogs(source: &str, catalogs: &crate::Catalogs) -> NominalPhrase {
        let parsed = crate::parse_fragment(
            source,
            catalogs,
            crate::FragmentKind::Nominal,
            "Test Card",
            false,
        );
        let Some(crate::Fragment::Nominal(noun_phrase)) = parsed.into_fragment() else {
            panic!("expected a nominal parse for {source:?}")
        };
        let crate::syntax::NounPhraseKind::Nominal(nominal) = noun_phrase.kind() else {
            panic!("expected a nominal parse for {source:?}")
        };
        nominal.clone()
    }

    fn parsed_independent(source: &str) -> IndependentClause {
        let parsed = crate::parse_fragment(
            source,
            &crate::Catalogs::default(),
            crate::FragmentKind::Sentence,
            "Test Card",
            false,
        );
        let Some(crate::Fragment::Sentence(sentence)) = parsed.into_fragment() else {
            panic!("expected a sentence parse for {source:?}")
        };
        let crate::syntax::SentenceBody::Independent(clause) = sentence.body() else {
            panic!("expected an independent sentence for {source:?}")
        };
        clause.clone()
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "the ordered table intentionally keeps all 37 atomic nominal projections visible in one drift gate"
    )]
    fn every_nominal_checked_builder_round_trips_through_generated_parts() {
        // Mutations caught: omit a checked door, generated destructurer, or
        // per-form inverse; wire one declaration to the wrong adapter; reorder
        // fields; or let a different construction take credit for an emitted
        // surface. The final ordered assertion keeps this behavioral table in
        // lockstep with the complete atomic nominal declaration list.
        let mut exercised = Vec::new();
        let mut exercised_forms = Vec::new();
        let exact_catalogs = crate::Catalogs::default()
            .with_catalog(
                CatalogKind::KeywordAbility,
                ["Ward", "Protection", "Hexproof", "Hexproof from"],
            )
            .with_catalog(CatalogKind::CardType, ["Creature"]);
        macro_rules! exact_form {
            ($id:literal, $value:expr, $ordinal:literal, $linearizer:path) => {{
                let source = crate::renderer::render_nominal_construction_form(
                    &$value,
                    $ordinal,
                    |value, ordinal, visitor| $linearizer(value, ordinal, visitor),
                )
                .unwrap_or_else(|error| {
                    panic!(
                        "{} form {} did not linearize through its emitted entry: {error:?}",
                        $id, $ordinal
                    )
                });
                exercised_forms.push(($id, $ordinal));
                let category = NOMINAL_DECLARATION
                    .constructions
                    .iter()
                    .find(|construction| construction.id == $id)
                    .unwrap_or_else(|| panic!("missing nominal construction {}", $id))
                    .category;
                let orders = crate::grammar::exact::parse_production_as_declared_category_in_both_orders(
                    &source,
                    &exact_catalogs,
                    category,
                    &$value,
                    100_000,
                )
                .unwrap_or_else(|error| {
                    panic!(
                        "{} form {} exact parse failed for {source:?}: {error:?}",
                        $id, $ordinal
                    )
                });
                for parses in orders {
                    let actual_roots = parses
                        .iter()
                        .map(|parse| {
                            (parse.ast().construction, parse.ast().form_ordinal)
                        })
                        .collect::<std::collections::BTreeSet<_>>();
                    let expected_roots = std::collections::BTreeSet::from([($id, $ordinal)]);
                    assert_eq!(
                        actual_roots, expected_roots,
                        "{} form {} rendered {source:?}, but exact parsing attributed it to {parses:#?}",
                        $id, $ordinal,
                    );
                }
            }};
        }
        macro_rules! records {
            ($id:literal, $value:expr, $rebuilt:expr, $linearizer:path) => {{
                let value = $value;
                let rebuilt = $rebuilt;
                assert_eq!(rebuilt, value, "{} projection", $id);
                exact_form!($id, value, 0, $linearizer);
                exercised.push($id);
                value
            }};
        }

        let nominal_noun =
            build_nominal_noun(NounInstance::unchecked_singular(Noun::Word(Vocab::Card)))
                .expect("singular card is an admitted nominal head");
        let head = parts_nominal_noun(&nominal_noun).expect("head lens projects its sole field");
        records!(
            "nominal_noun",
            nominal_noun,
            build_nominal_noun(head).unwrap(),
            linearize_nominal_noun_form_with
        );

        let nominal_adjective = build_nominal_adjective(red_adjective(), card_nominal()).unwrap();
        let (adjective, nominal) = parts_nominal_adjective(&nominal_adjective);
        records!(
            "nominal_adjective",
            nominal_adjective,
            build_nominal_adjective(adjective, nominal).unwrap(),
            linearize_nominal_adjective_form_with
        );

        let nominal_noun_modifier = build_nominal_noun_modifier(
            NounInstance::unchecked_singular(Noun::Word(Vocab::Ability)),
            card_nominal(),
        )
        .unwrap();
        let (noun, nominal) = parts_nominal_noun_modifier(&nominal_noun_modifier);
        records!(
            "nominal_noun_modifier",
            nominal_noun_modifier,
            build_nominal_noun_modifier(noun, nominal).unwrap(),
            linearize_nominal_noun_modifier_form_with
        );

        let nominal_combat_step_name = build_nominal_combat_step_name(
            NounInstance::unchecked_plural(Noun::Agentive(crate::word::Verb::Word(Vocab::Attack))),
            NounInstance::unchecked_singular(Noun::Word(Vocab::Step)),
        )
        .unwrap();
        let (participants, head) = parts_nominal_combat_step_name(&nominal_combat_step_name);
        records!(
            "nominal_combat_step_name",
            nominal_combat_step_name,
            build_nominal_combat_step_name(participants, head).unwrap(),
            linearize_nominal_combat_step_name_form_with
        );

        let modifier = NominalModifier::Noun {
            polarity: Polarity::Negative,
            noun: NounInstance::unchecked_singular(Noun::Word(Vocab::Ability)),
        };
        let nominal_negated_modifier =
            build_nominal_negated_modifier(modifier, card_nominal()).unwrap();
        let (modifier, nominal) = parts_nominal_negated_modifier(&nominal_negated_modifier)
            .expect("negative prefix is projected by the modifier lens");
        records!(
            "nominal_negated_modifier",
            nominal_negated_modifier,
            build_nominal_negated_modifier(modifier, nominal).unwrap(),
            linearize_nominal_negated_modifier_form_with
        );

        let one = Quantity::try_exact(crate::syntax::NumberLiteral {
            value: 1,
            numeral: crate::Numeral::Cardinal,
        })
        .unwrap();
        let nominal_quantity_modifier =
            build_nominal_quantity_modifier(one, card_nominal()).unwrap();
        let (quantity, nominal) = parts_nominal_quantity_modifier(&nominal_quantity_modifier);
        records!(
            "nominal_quantity_modifier",
            nominal_quantity_modifier,
            build_nominal_quantity_modifier(quantity, nominal).unwrap(),
            linearize_nominal_quantity_modifier_form_with
        );

        let stats = PowerToughness {
            power: crate::syntax::SignedScalar {
                sign: crate::syntax::ScalarSign::None,
                value: crate::syntax::ScalarValue::Integer(2),
            },
            toughness: crate::syntax::SignedScalar {
                sign: crate::syntax::ScalarSign::None,
                value: crate::syntax::ScalarValue::Integer(3),
            },
        };
        let nominal_power_toughness_modifier =
            build_nominal_power_toughness_modifier(stats, card_nominal()).unwrap();
        let (stats, nominal) =
            parts_nominal_power_toughness_modifier(&nominal_power_toughness_modifier);
        records!(
            "nominal_power_toughness_modifier",
            nominal_power_toughness_modifier,
            build_nominal_power_toughness_modifier(stats, nominal).unwrap(),
            linearize_nominal_power_toughness_modifier_form_with
        );

        let nominal_determiner =
            build_nominal_determiner(crate::determiner::indefinite(), card_nominal()).unwrap();
        let (determiner, nominal) = parts_nominal_determiner(&nominal_determiner)
            .expect("determiner lens projects a determined nominal");
        records!(
            "nominal_determiner",
            nominal_determiner,
            build_nominal_determiner(determiner, nominal).unwrap(),
            linearize_nominal_determiner_form_with
        );

        let preposition = crate::constructions::prepositional::expect_prepositional_phrase(
            Preposition::In,
            crate::syntax::PrepositionalObjectKind::NounPhrase(Box::new(
                NounPhrase::from_nominal_declaration(card_nominal()),
            )),
        );
        let nominal_prepositional =
            build_nominal_prepositional(card_nominal(), preposition.clone()).unwrap();
        let (nominal, preposition) = parts_nominal_prepositional(&nominal_prepositional);
        records!(
            "nominal_prepositional",
            nominal_prepositional,
            build_nominal_prepositional(nominal, preposition.clone()).unwrap(),
            linearize_nominal_prepositional_form_with
        );

        let IndependentClause::Finite(finite) = parsed_independent("Draw a card.") else {
            panic!("imperative fixture has an imperative predicate")
        };
        assert!(finite.subject().is_none());
        let crate::syntax::PredicateExpression::Simple(mut predicate) = finite.predicate else {
            panic!("imperative fixture has one predicate")
        };
        let crate::syntax::Predicate::Transitive(transitive) = &mut predicate else {
            panic!("draw fixture has a transitive predicate")
        };
        transitive.head.verb.slot = VerbSlot::Infinitive;
        let infinitive = crate::clause::build_infinitive_to(&predicate).unwrap();
        let nominal_infinitive = build_nominal_infinitive(card_nominal(), infinitive).unwrap();
        let (nominal, infinitive) = parts_nominal_infinitive(&nominal_infinitive);
        records!(
            "nominal_infinitive",
            nominal_infinitive,
            build_nominal_infinitive(nominal, infinitive).unwrap(),
            linearize_nominal_infinitive_form_with
        );

        let nominal_quantity_complement =
            build_nominal_quantity_complement(card_nominal(), Quantity::try_x().unwrap()).unwrap();
        let (nominal, quantity) = parts_nominal_quantity_complement(&nominal_quantity_complement);
        records!(
            "nominal_quantity_complement",
            nominal_quantity_complement,
            build_nominal_quantity_complement(nominal, quantity).unwrap(),
            linearize_nominal_quantity_complement_form_with
        );

        let ward_catalogs =
            crate::Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Ward"]);
        let nominal_keyword_symbol_argument =
            parsed_nominal_with_catalogs("ward {2}", &ward_catalogs);
        let (head, symbol, sequence) =
            parts_nominal_keyword_symbol_argument(&nominal_keyword_symbol_argument);
        records!(
            "nominal_keyword_symbol_argument",
            nominal_keyword_symbol_argument,
            build_nominal_keyword_symbol_argument(head, symbol, sequence).unwrap(),
            linearize_nominal_keyword_symbol_argument_form_with
        );
        let nominal_keyword_symbol_sequence =
            parsed_nominal_with_catalogs("ward {2}{U}", &exact_catalogs);
        exact_form!(
            "nominal_keyword_symbol_argument",
            nominal_keyword_symbol_sequence,
            1,
            linearize_nominal_keyword_symbol_argument_form_with
        );

        let predicated_quality_from =
            build_predicated_quality_from(Preposition::From, Some(ColorWord::Red), None, None)
                .unwrap();
        let (predicated_preposition, color, adjective, noun_phrase) =
            parts_predicated_quality_from(&predicated_quality_from);
        let predicated_quality_from = records!(
            "predicated_quality_from",
            predicated_quality_from,
            build_predicated_quality_from(predicated_preposition, color, adjective, noun_phrase)
                .unwrap(),
            linearize_predicated_quality_from_form_with
        );
        let predicated_quality_from_adjective = build_predicated_quality_from(
            Preposition::From,
            None,
            Some(
                crate::adjective::build_adjective_phrase(Adjective::Word(Vocab::Monocolored))
                    .unwrap(),
            ),
            None,
        )
        .unwrap();
        exact_form!(
            "predicated_quality_from",
            predicated_quality_from_adjective,
            1,
            linearize_predicated_quality_from_form_with
        );
        let predicated_quality_from_noun_phrase = build_predicated_quality_from(
            Preposition::From,
            None,
            None,
            Some(NounPhrase::from_nominal_declaration(
                build_nominal_determiner(crate::determiner::indefinite(), card_nominal()).unwrap(),
            )),
        )
        .unwrap();
        exact_form!(
            "predicated_quality_from",
            predicated_quality_from_noun_phrase,
            2,
            linearize_predicated_quality_from_form_with
        );

        let predicated_argument_from_single =
            build_predicated_argument_from_single(predicated_quality_from.clone()).unwrap();
        let quality = parts_predicated_argument_from_single(&predicated_argument_from_single);
        let predicated_argument_from_single = records!(
            "predicated_argument_from_single",
            predicated_argument_from_single,
            build_predicated_argument_from_single(quality).unwrap(),
            linearize_predicated_argument_from_single_form_with
        );

        let second_from =
            build_predicated_quality_from(Preposition::From, Some(ColorWord::Black), None, None)
                .unwrap();
        let predicated_argument_from_extend = build_predicated_argument_from_extend(
            predicated_argument_from_single,
            Conjunction::And,
            second_from,
        )
        .unwrap();
        let (argument, conjunction, quality) =
            parts_predicated_argument_from_extend(&predicated_argument_from_extend);
        let predicated_argument_from_extend = records!(
            "predicated_argument_from_extend",
            predicated_argument_from_extend,
            build_predicated_argument_from_extend(argument, conjunction, quality).unwrap(),
            linearize_predicated_argument_from_extend_form_with
        );

        let protection_catalogs =
            crate::Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Protection"]);
        let protection = parsed_nominal_with_catalogs("protection from red", &protection_catalogs);
        let (head, _) = parts_nominal_keyword_predicated_argument(&protection);
        let nominal_keyword_predicated_argument =
            build_nominal_keyword_predicated_argument(head, predicated_argument_from_extend)
                .unwrap();
        let (head, argument) =
            parts_nominal_keyword_predicated_argument(&nominal_keyword_predicated_argument);
        records!(
            "nominal_keyword_predicated_argument",
            nominal_keyword_predicated_argument,
            build_nominal_keyword_predicated_argument(head, argument).unwrap(),
            linearize_nominal_keyword_predicated_argument_form_with
        );

        let predicated_quality_bare =
            build_predicated_quality_bare(Some(ColorWord::Blue), None, None).unwrap();
        let (color, adjective, noun_phrase) =
            parts_predicated_quality_bare(&predicated_quality_bare);
        let predicated_quality_bare = records!(
            "predicated_quality_bare",
            predicated_quality_bare,
            build_predicated_quality_bare(color, adjective, noun_phrase).unwrap(),
            linearize_predicated_quality_bare_form_with
        );
        let predicated_quality_bare_adjective =
            build_predicated_quality_bare(None, Some(red_adjective()), None).unwrap();
        exact_form!(
            "predicated_quality_bare",
            predicated_quality_bare_adjective,
            1,
            linearize_predicated_quality_bare_form_with
        );
        let predicated_quality_bare_noun_phrase = build_predicated_quality_bare(
            None,
            None,
            Some(NounPhrase::from_nominal_declaration(
                build_nominal_determiner(crate::determiner::indefinite(), card_nominal()).unwrap(),
            )),
        )
        .unwrap();
        exact_form!(
            "predicated_quality_bare",
            predicated_quality_bare_noun_phrase,
            2,
            linearize_predicated_quality_bare_form_with
        );

        let predicated_argument_bare_single =
            build_predicated_argument_bare_single(predicated_quality_bare).unwrap();
        let quality = parts_predicated_argument_bare_single(&predicated_argument_bare_single);
        let predicated_argument_bare_single = records!(
            "predicated_argument_bare_single",
            predicated_argument_bare_single,
            build_predicated_argument_bare_single(quality).unwrap(),
            linearize_predicated_argument_bare_single_form_with
        );

        let final_from =
            build_predicated_quality_from(Preposition::From, Some(ColorWord::Black), None, None)
                .unwrap();
        let predicated_argument_bare_extend = build_predicated_argument_bare_extend(
            predicated_argument_bare_single,
            Conjunction::And,
            final_from,
        )
        .unwrap();
        let (argument, conjunction, quality) =
            parts_predicated_argument_bare_extend(&predicated_argument_bare_extend);
        let predicated_argument_bare_extend = records!(
            "predicated_argument_bare_extend",
            predicated_argument_bare_extend,
            build_predicated_argument_bare_extend(argument, conjunction, quality).unwrap(),
            linearize_predicated_argument_bare_extend_form_with
        );

        let hexproof_catalogs = crate::Catalogs::default()
            .with_catalog(CatalogKind::KeywordAbility, ["Hexproof", "Hexproof from"]);
        let hexproof = parsed_nominal_with_catalogs("hexproof from blue", &hexproof_catalogs);
        let (head, _) = parts_nominal_keyword_atom_carried_predicated_argument(&hexproof);
        let nominal_keyword_atom_carried_predicated_argument =
            build_nominal_keyword_atom_carried_predicated_argument(
                head,
                predicated_argument_bare_extend,
            )
            .unwrap();
        let (head, argument) = parts_nominal_keyword_atom_carried_predicated_argument(
            &nominal_keyword_atom_carried_predicated_argument,
        );
        records!(
            "nominal_keyword_atom_carried_predicated_argument",
            nominal_keyword_atom_carried_predicated_argument,
            build_nominal_keyword_atom_carried_predicated_argument(head, argument).unwrap(),
            linearize_nominal_keyword_atom_carried_predicated_argument_form_with
        );

        let nominal_relative =
            parsed_nominal_with_catalogs("creature you control", &exact_catalogs);
        let (relative_base, relative) = parts_nominal_relative(&nominal_relative);
        let nominal_relative = records!(
            "nominal_relative",
            nominal_relative,
            build_nominal_relative(relative_base.clone(), relative.clone()).unwrap(),
            linearize_nominal_relative_form_with
        );

        let rules_object_nominal_base =
            build_rules_object_nominal_base(nominal_relative.clone()).unwrap();
        let nominal = parts_rules_object_nominal_base(&rules_object_nominal_base);
        let rules_object_nominal_base = records!(
            "rules_object_nominal_base",
            rules_object_nominal_base,
            build_rules_object_nominal_base(nominal).unwrap(),
            linearize_rules_object_nominal_base_form_with
        );

        let ordinary_relative_nominal =
            parsed_nominal_with_catalogs("creature that attacks", &exact_catalogs);
        let (_, ordinary_relative) = parts_nominal_relative(&ordinary_relative_nominal);
        let rules_object_followup_nominal_relative = build_rules_object_followup_nominal_relative(
            Some(rules_object_nominal_base),
            None,
            ordinary_relative.clone(),
        )
        .unwrap();
        let (base, followup, relative) =
            parts_rules_object_followup_nominal_relative(&rules_object_followup_nominal_relative);
        let rules_object_followup_nominal_relative = records!(
            "rules_object_followup_nominal_relative",
            rules_object_followup_nominal_relative,
            build_rules_object_followup_nominal_relative(base, followup, relative).unwrap(),
            linearize_rules_object_followup_nominal_relative_form_with
        );
        let repeated_rules_object_followup = build_rules_object_followup_nominal_relative(
            None,
            Some(rules_object_followup_nominal_relative.clone()),
            ordinary_relative,
        )
        .unwrap();
        exact_form!(
            "rules_object_followup_nominal_relative",
            repeated_rules_object_followup,
            1,
            linearize_rules_object_followup_nominal_relative_form_with
        );

        let rules_object_followup_nominal_prepositional =
            build_rules_object_followup_nominal_prepositional(
                rules_object_followup_nominal_relative,
                preposition,
            )
            .unwrap();
        let (nominal, preposition) = parts_rules_object_followup_nominal_prepositional(
            &rules_object_followup_nominal_prepositional,
        );
        records!(
            "rules_object_followup_nominal_prepositional",
            rules_object_followup_nominal_prepositional,
            build_rules_object_followup_nominal_prepositional(nominal, preposition).unwrap(),
            linearize_rules_object_followup_nominal_prepositional_form_with
        );

        let reduced_subject =
            parsed_nominal_with_catalogs("a creature dealt damage this way", &exact_catalogs);
        let (reduced_base, reduced_predicate) =
            parts_nominal_reduced_recipient_passive(&reduced_subject);
        let (_, reduced_base) = parts_nominal_determiner(&reduced_base)
            .expect("fixture creature carries its indefinite determiner");
        let reduced_subject =
            build_nominal_reduced_recipient_passive(reduced_base, reduced_predicate.clone())
                .expect("bare creature admits the same reduced recipient-passive predicate");
        let (reduced_base, reduced_predicate) =
            parts_nominal_reduced_recipient_passive(&reduced_subject);
        records!(
            "nominal_reduced_recipient_passive",
            reduced_subject,
            build_nominal_reduced_recipient_passive(reduced_base, reduced_predicate.clone())
                .unwrap(),
            linearize_nominal_reduced_recipient_passive_form_with
        );

        let damage =
            build_nominal_noun(NounInstance::unchecked_mass(Noun::Word(Vocab::Damage))).unwrap();
        let reduced_recipient_passive_theme =
            build_reduced_recipient_passive_theme(damage).unwrap();
        let nominal = parts_reduced_recipient_passive_theme(&reduced_recipient_passive_theme);
        records!(
            "reduced_recipient_passive_theme",
            reduced_recipient_passive_theme,
            build_reduced_recipient_passive_theme(nominal).unwrap(),
            linearize_reduced_recipient_passive_theme_form_with
        );

        let (predicate, noun_phrase) =
            parts_reduced_recipient_passive_nominal_adjunct(&reduced_predicate);
        records!(
            "reduced_recipient_passive_nominal_adjunct",
            reduced_predicate,
            build_reduced_recipient_passive_nominal_adjunct(predicate, noun_phrase).unwrap(),
            linearize_reduced_recipient_passive_nominal_adjunct_form_with
        );

        let nominal_postpositive_adjective =
            build_nominal_postpositive_adjective(card_nominal(), red_adjective()).unwrap();
        let (nominal, adjective) =
            parts_nominal_postpositive_adjective(&nominal_postpositive_adjective);
        records!(
            "nominal_postpositive_adjective",
            nominal_postpositive_adjective,
            build_nominal_postpositive_adjective(nominal, adjective).unwrap(),
            linearize_nominal_postpositive_adjective_form_with
        );

        let blue_adjective = || {
            crate::adjective::build_adjective_phrase(Adjective::Color(ColorWord::Blue))
                .expect("blue is a lexical adjective")
        };
        let green_adjective = || {
            crate::adjective::build_adjective_phrase(Adjective::Color(ColorWord::Green))
                .expect("green is a lexical adjective")
        };
        let prepositional_base =
            build_nominal_postpositive_adjective(card_nominal(), red_adjective()).unwrap();
        let nominal_postpositive_adjective_conjoined_prepositional =
            build_nominal_postpositive_adjective_conjoined_prepositional(
                prepositional_base,
                Conjunction::Or,
                participle(crate::word::Tense::Past, Vocab::Block),
                crate::constructions::prepositional::expect_prepositional_phrase(
                    Preposition::By,
                    crate::syntax::PrepositionalObjectKind::NounPhrase(Box::new(
                        NounPhrase::from_nominal_declaration(card_nominal()),
                    )),
                ),
            )
            .unwrap();
        let (nominal, conjunction, adjective, preposition) =
            parts_nominal_postpositive_adjective_conjoined_prepositional(
                &nominal_postpositive_adjective_conjoined_prepositional,
            );
        records!(
            "nominal_postpositive_adjective_conjoined_prepositional",
            nominal_postpositive_adjective_conjoined_prepositional,
            build_nominal_postpositive_adjective_conjoined_prepositional(
                nominal,
                conjunction,
                adjective,
                preposition,
            )
            .unwrap(),
            linearize_nominal_postpositive_adjective_conjoined_prepositional_form_with
        );

        let conjoined_base =
            build_nominal_postpositive_adjective(card_nominal(), red_adjective()).unwrap();
        let nominal_postpositive_adjective_conjoined =
            build_nominal_postpositive_adjective_conjoined(
                conjoined_base,
                Conjunction::AndOr,
                blue_adjective(),
            )
            .unwrap();
        let (nominal, conjunction, adjective) = parts_nominal_postpositive_adjective_conjoined(
            &nominal_postpositive_adjective_conjoined,
        );
        records!(
            "nominal_postpositive_adjective_conjoined",
            nominal_postpositive_adjective_conjoined,
            build_nominal_postpositive_adjective_conjoined(nominal, conjunction, adjective)
                .unwrap(),
            linearize_nominal_postpositive_adjective_conjoined_form_with
        );

        let asyndetic_base =
            build_nominal_postpositive_adjective(card_nominal(), red_adjective()).unwrap();
        let nominal_postpositive_adjective_asyndetic =
            build_nominal_postpositive_adjective_asyndetic(asyndetic_base, blue_adjective())
                .unwrap();
        let (nominal, adjective) = parts_nominal_postpositive_adjective_asyndetic(
            &nominal_postpositive_adjective_asyndetic,
        );
        let nominal_postpositive_adjective_asyndetic = records!(
            "nominal_postpositive_adjective_asyndetic",
            nominal_postpositive_adjective_asyndetic,
            build_nominal_postpositive_adjective_asyndetic(nominal, adjective).unwrap(),
            linearize_nominal_postpositive_adjective_asyndetic_form_with
        );

        let nominal_postpositive_adjective_oxford = build_nominal_postpositive_adjective_oxford(
            nominal_postpositive_adjective_asyndetic,
            Conjunction::And,
            green_adjective(),
        )
        .unwrap();
        let (nominal, conjunction, adjective) =
            parts_nominal_postpositive_adjective_oxford(&nominal_postpositive_adjective_oxford);
        records!(
            "nominal_postpositive_adjective_oxford",
            nominal_postpositive_adjective_oxford,
            build_nominal_postpositive_adjective_oxford(nominal, conjunction, adjective).unwrap(),
            linearize_nominal_postpositive_adjective_oxford_form_with
        );

        let comparison = crate::adjective::build_comparison_than(
            crate::adjective::build_comparison_standard(
                Some(NounPhrase::from_nominal_declaration(card_nominal())),
                None,
                None,
            )
            .unwrap(),
        )
        .unwrap();
        let nominal_comparison = build_nominal_comparison(
            build_nominal_adjective(
                crate::adjective::build_adjective_phrase(Adjective::Word(Vocab::Greater)).unwrap(),
                card_nominal(),
            )
            .unwrap(),
            comparison,
        )
        .unwrap();
        let (nominal, comparison) = parts_nominal_comparison(&nominal_comparison);
        records!(
            "nominal_comparison",
            nominal_comparison,
            build_nominal_comparison(nominal, comparison).unwrap(),
            linearize_nominal_comparison_form_with
        );

        let nominal_devotion = parsed_nominal("your devotion to green");
        let (_, modifiers, head, complements) = nominal_devotion.into_projection_parts();
        let nominal_devotion =
            NominalPhrase::from_projection_parts(None, modifiers, head, complements);
        let (head, colors) = parts_nominal_devotion(&nominal_devotion);
        records!(
            "nominal_devotion",
            nominal_devotion,
            build_nominal_devotion(head, colors).unwrap(),
            linearize_nominal_devotion_form_with
        );

        let devotion_color_single = build_devotion_color_single(ColorWord::Green).unwrap();
        let color = parts_devotion_color_single(&devotion_color_single);
        records!(
            "devotion_color_single",
            devotion_color_single,
            build_devotion_color_single(color).unwrap(),
            linearize_devotion_color_single_form_with
        );

        let devotion_color_pair =
            build_devotion_color_pair(ColorWord::White, Conjunction::And, ColorWord::Black)
                .unwrap();
        let (first, conjunction, second) = parts_devotion_color_pair(&devotion_color_pair);
        records!(
            "devotion_color_pair",
            devotion_color_pair,
            build_devotion_color_pair(first, conjunction, second).unwrap(),
            linearize_devotion_color_pair_form_with
        );

        let nominal_times_clause = build_nominal_times_clause(
            NounInstance::unchecked_plural(Noun::Word(Vocab::Time)),
            Box::new(parsed_independent("You draw a card.")),
        )
        .unwrap();
        let (head, clause) = parts_nominal_times_clause(&nominal_times_clause);
        records!(
            "nominal_times_clause",
            nominal_times_clause,
            build_nominal_times_clause(head, clause).unwrap(),
            linearize_nominal_times_clause_form_with
        );

        assert_eq!(
            exercised,
            NOMINAL_DECLARATION
                .constructions
                .iter()
                .map(|construction| construction.id)
                .collect::<Vec<_>>(),
        );
        assert_eq!(
            exercised_forms,
            NOMINAL_DECLARATION
                .constructions
                .iter()
                .flat_map(|construction| {
                    construction
                        .forms
                        .iter()
                        .map(move |form| (construction.id, form.ordinal))
                })
                .collect::<Vec<_>>(),
            "every declared nominal form has one construction-specific exact law",
        );
    }

    #[test]
    fn generated_inverse_selects_lensed_and_adapted_nominal_projections() {
        // Mutations caught: let a broad vector lens overlap a typed modifier,
        // omit adapted nominal rows from mixed inverse dispatch, or select a form
        // before checking its complete-value recognition predicate.
        let adjective = red_adjective();
        let adjective_nominal = build_nominal_adjective(adjective, card_nominal()).unwrap();
        assert_eq!(
            selected_nominal_construction(&adjective_nominal),
            "nominal_adjective"
        );

        let negative = NominalModifier::Noun {
            polarity: Polarity::Negative,
            noun: NounInstance::unchecked_singular(Noun::Word(Vocab::Ability)),
        };
        let negative_nominal = build_nominal_negated_modifier(negative, card_nominal()).unwrap();
        assert_eq!(
            selected_nominal_construction(&negative_nominal),
            "nominal_negated_modifier"
        );

        let determined =
            build_nominal_determiner(crate::determiner::indefinite(), adjective_nominal)
                .expect("an undetermined generated nominal admits one determiner");
        assert_eq!(
            selected_nominal_construction(&determined),
            "nominal_determiner"
        );

        let three = Quantity::try_exact(crate::syntax::NumberLiteral {
            value: 3,
            numeral: crate::Numeral::Cardinal,
        })
        .unwrap();
        let quantified_times = build_nominal_determiner(
            crate::determiner::quantity(three),
            build_nominal_times_clause(
                NounInstance::unchecked_plural(Noun::Word(Vocab::Time)),
                Box::new(parsed_independent("You draw a card.")),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            selected_nominal_construction(&quantified_times),
            "nominal_determiner",
            "a determiner is structurally outside the times-clause base"
        );

        let modified_times = build_nominal_noun_modifier(
            NounInstance::unchecked_singular(Noun::Word(Vocab::Ability)),
            build_nominal_times_clause(
                NounInstance::unchecked_plural(Noun::Word(Vocab::Time)),
                Box::new(parsed_independent("You draw a card.")),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            selected_nominal_construction(&modified_times),
            "nominal_noun_modifier",
            "a prefix modifier is structurally outside the times-clause base"
        );
    }

    #[test]
    fn checked_prefix_builders_reject_a_determined_residual() {
        // Mutations caught: let either an adapted or lensed prefix builder
        // manufacture a value whose outermost declared inverse is the
        // determiner rather than that prefix construction.
        let determined = || {
            build_nominal_determiner(crate::determiner::indefinite(), card_nominal())
                .expect("control: a bare singular card admits an indefinite determiner")
        };
        let negative = || NominalModifier::Noun {
            polarity: Polarity::Negative,
            noun: NounInstance::unchecked_singular(Noun::Word(Vocab::Ability)),
        };
        let one = || {
            Quantity::try_exact(crate::syntax::NumberLiteral {
                value: 1,
                numeral: crate::Numeral::Cardinal,
            })
            .unwrap()
        };
        let stats = || PowerToughness {
            power: crate::syntax::SignedScalar {
                sign: crate::syntax::ScalarSign::None,
                value: crate::syntax::ScalarValue::Integer(1),
            },
            toughness: crate::syntax::SignedScalar {
                sign: crate::syntax::ScalarSign::None,
                value: crate::syntax::ScalarValue::Integer(1),
            },
        };

        assert!(build_nominal_adjective(red_adjective(), determined()).is_err());
        assert!(
            build_nominal_noun_modifier(
                NounInstance::unchecked_singular(Noun::Word(Vocab::Ability)),
                determined(),
            )
            .is_err()
        );
        assert!(build_nominal_negated_modifier(negative(), determined()).is_err());
        assert!(build_nominal_quantity_modifier(one(), determined()).is_err());
        assert!(build_nominal_power_toughness_modifier(stats(), determined()).is_err());
    }

    #[test]
    fn checked_prefix_builder_rejects_late_attachment_but_legal_order_rebuilds() {
        // Mutations caught: allow a prefix edit after a complement has already
        // become the outer construction, or make the legal prefix-then-PP
        // projection reverse that declared construction order.
        let preposition = || {
            crate::constructions::prepositional::expect_prepositional_phrase(
                Preposition::In,
                crate::syntax::PrepositionalObjectKind::NounPhrase(Box::new(
                    NounPhrase::from_nominal_declaration(card_nominal()),
                )),
            )
        };
        let attached = build_nominal_prepositional(card_nominal(), preposition()).unwrap();
        assert!(
            build_nominal_noun_modifier(
                NounInstance::unchecked_singular(Noun::Word(Vocab::Ability)),
                attached,
            )
            .is_err()
        );

        let prefixed = build_nominal_noun_modifier(
            NounInstance::unchecked_singular(Noun::Word(Vocab::Ability)),
            card_nominal(),
        )
        .unwrap();
        let value = build_nominal_prepositional(prefixed, preposition()).unwrap();
        assert_eq!(
            selected_nominal_construction(&value),
            "nominal_prepositional"
        );
        let (prefix, preposition) = parts_nominal_prepositional(&value);
        assert_eq!(
            selected_nominal_construction(&prefix),
            "nominal_noun_modifier"
        );
        assert_eq!(
            build_nominal_prepositional(prefix, preposition).unwrap(),
            value
        );
    }

    fn than_a_card() -> ComparisonComplement {
        crate::adjective::build_comparison_than(
            crate::adjective::build_comparison_standard(
                Some(NounPhrase::from_nominal_declaration(
                    build_nominal_determiner(crate::determiner::indefinite(), card_nominal())
                        .unwrap(),
                )),
                None,
                None,
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn comparison_builder_rejects_a_noncomparative_adjective() {
        // Mutation caught: identify comparison availability only by the
        // absence of an existing complement, allowing an ordinary adjective
        // such as `red` to take a postnominal `than` standard.
        let nominal = build_nominal_adjective(red_adjective(), card_nominal()).unwrap();
        assert!(build_nominal_comparison(nominal, than_a_card()).is_err());
    }

    #[test]
    fn comparison_builder_rejects_an_already_completed_adjective() {
        // Mutation caught: append a second completion after a comparative
        // adjective already carries its ordinary comparison complement.
        let completed = crate::adjective::build_adjective_phrase_comparison(
            crate::adjective::build_adjective_phrase(Adjective::Word(Vocab::Greater)).unwrap(),
            than_a_card(),
        )
        .unwrap();
        let nominal = build_nominal_adjective(completed, card_nominal()).unwrap();
        assert!(build_nominal_comparison(nominal, than_a_card()).is_err());
    }

    #[test]
    fn pending_comparative_builder_rebuilds_and_renders_its_postnominal_standard() {
        // Mutation caught: reject the actual comparison-pending vocabulary
        // class, lose the comparison during parts projection, or render the
        // stored postnominal completion in prefix position.
        let pending =
            crate::adjective::build_adjective_phrase(Adjective::Word(Vocab::Greater)).unwrap();
        let value = build_nominal_comparison(
            build_nominal_adjective(pending, card_nominal()).unwrap(),
            than_a_card(),
        )
        .unwrap();
        assert_eq!(selected_nominal_construction(&value), "nominal_comparison");
        let (nominal, comparison) = parts_nominal_comparison(&value);
        assert_eq!(
            build_nominal_comparison(nominal, comparison).unwrap(),
            value
        );
        assert_eq!(
            crate::render_fragment(
                &crate::Fragment::Nominal(NounPhrase::from_nominal_declaration(value)),
                "Test Card",
                false,
            )
            .unwrap(),
            "greater card than a card"
        );
    }

    #[test]
    fn attributive_card_orientation_crosses_only_the_partial_chart_door() {
        let orientation = crate::adjective::build_adjective_phrase_face_down().unwrap();
        assert!(build_nominal_adjective(orientation.clone(), card_nominal()).is_err());

        let declaration = NOMINAL_DECLARATION
            .constructions
            .iter()
            .find(|construction| construction.id == "nominal_adjective")
            .expect("the adjective construction is declared");
        let partial = declaration
            .erased_partial_builder
            .expect("chart partial builder exists")(vec![
            Box::new(orientation.clone()),
            Box::new(card_nominal()),
        ])
        .expect("partial chart assembly precedes feature rejection");
        let partial = *partial
            .downcast::<NominalPhrase>()
            .expect("nominal adjective partial value keeps its type");
        assert_nominal_inverse_rejects(&partial);
        assert!(
            declaration.erased_builder.expect("checked builder exists")(vec![
                Box::new(orientation),
                Box::new(card_nominal()),
            ])
            .is_err()
        );

        assert!(build_nominal_adjective(red_adjective(), card_nominal()).is_ok());
    }

    #[test]
    fn measured_adjective_crosses_only_the_partial_chart_door() {
        let measured = crate::adjective::build_adjective_phrase_degree_measure(
            crate::syntax::NumberLiteral {
                value: 2,
                numeral: crate::Numeral::Cardinal,
            },
            Adjective::Word(Vocab::Greater),
        )
        .unwrap();
        assert!(build_nominal_adjective(measured.clone(), card_nominal()).is_err());

        let declaration = NOMINAL_DECLARATION
            .constructions
            .iter()
            .find(|construction| construction.id == "nominal_adjective")
            .expect("the adjective construction is declared");
        let partial = declaration
            .erased_partial_builder
            .expect("chart partial builder exists")(vec![
            Box::new(measured.clone()),
            Box::new(card_nominal()),
        ])
        .expect("partial chart assembly precedes feature rejection");
        let partial = *partial
            .downcast::<NominalPhrase>()
            .expect("measured adjective partial value keeps its type");
        assert_nominal_inverse_rejects(&partial);
        assert!(
            declaration.erased_builder.expect("checked builder exists")(vec![
                Box::new(measured),
                Box::new(card_nominal()),
            ])
            .is_err()
        );

        let pending =
            crate::adjective::build_adjective_phrase(Adjective::Word(Vocab::Greater)).unwrap();
        assert!(build_nominal_adjective(pending, card_nominal()).is_ok());
    }

    #[test]
    fn by_gerund_pp_crosses_only_the_partial_chart_door() {
        let IndependentClause::Finite(finite) = parsed_independent("Sacrifice a card.") else {
            panic!("gerund fixture starts from an imperative predicate");
        };
        assert!(finite.subject().is_none());
        let crate::syntax::PredicateExpression::Simple(mut predicate) = finite.predicate else {
            panic!("gerund fixture starts from one predicate");
        };
        let crate::syntax::Predicate::Transitive(transitive) = &mut predicate else {
            panic!("sacrifice fixture has a transitive predicate")
        };
        transitive.head.verb.slot = VerbSlot::PresentParticiple;
        let gerund = crate::clause::build_gerund_clause_base(&predicate).unwrap();
        let by_gerund = crate::constructions::prepositional::expect_prepositional_phrase(
            Preposition::By,
            crate::syntax::PrepositionalObjectKind::GerundClause(Box::new(gerund)),
        );
        assert!(build_nominal_prepositional(card_nominal(), by_gerund.clone()).is_err());

        let declaration = NOMINAL_DECLARATION
            .constructions
            .iter()
            .find(|construction| construction.id == "nominal_prepositional")
            .expect("the prepositional construction is declared");
        let partial = declaration
            .erased_partial_builder
            .expect("chart partial builder exists")(vec![
            Box::new(card_nominal()),
            Box::new(by_gerund.clone()),
        ])
        .expect("partial chart assembly precedes feature rejection");
        let partial = *partial
            .downcast::<NominalPhrase>()
            .expect("prepositional partial value keeps its type");
        assert_nominal_inverse_rejects(&partial);
        assert!(
            declaration.erased_builder.expect("checked builder exists")(vec![
                Box::new(card_nominal()),
                Box::new(by_gerund),
            ])
            .is_err()
        );

        assert!(
            build_nominal_prepositional(card_nominal(), preposition_with_card(Preposition::By),)
                .is_ok()
        );
    }

    fn participle(tense: crate::word::Tense, verb: Vocab) -> AdjectivePhrase {
        crate::adjective::build_adjective_phrase(Adjective::Participle(
            tense,
            crate::word::Verb::Word(verb),
        ))
        .expect("the participle is a lexical adjective")
    }

    fn preposition_with_card(preposition: Preposition) -> PrepositionalPhrase {
        crate::constructions::prepositional::expect_prepositional_phrase(
            preposition,
            crate::syntax::PrepositionalObjectKind::NounPhrase(Box::new(
                NounPhrase::from_nominal_declaration(card_nominal()),
            )),
        )
    }

    fn reduced_predicate_and_adjunct() -> (TransitivePredicate, NounPhrase) {
        let IndependentClause::Finite(finite) =
            parsed_independent("A creature dealt damage this way can't block this turn.")
        else {
            panic!("reduced-passive fixture has a deontic nominal subject")
        };
        let Some(crate::syntax::Subject(subject)) = finite.subject() else {
            panic!("reduced-passive fixture has a deontic nominal subject")
        };
        let crate::syntax::NounPhraseKind::Nominal(subject) = subject.kind() else {
            panic!("reduced-passive fixture has a deontic nominal subject")
        };
        let (_, predicate) = parts_nominal_reduced_recipient_passive(subject);
        parts_reduced_recipient_passive_nominal_adjunct(&predicate)
    }

    #[test]
    fn rules_object_followup_builder_rejects_a_base_role() {
        // Mutation caught: collapse the base and followup rules-object roles
        // to aliases of NominalPhrase. The typed generated function prevents
        // this call statically; the erased chart door must reject it too.
        let construction = NOMINAL_DECLARATION
            .constructions
            .iter()
            .find(|construction| construction.id == "rules_object_followup_nominal_prepositional")
            .expect("the rules-object followup construction is declared");
        let base = build_rules_object_nominal_base(card_nominal()).unwrap();
        assert!(
            construction.erased_builder.expect("checked builder exists")(vec![
                Box::new(base),
                Box::new(preposition_with_card(Preposition::Of)),
            ])
            .is_err()
        );
    }

    #[test]
    fn reduced_passive_theme_has_a_distinct_typed_role() {
        // Mutation caught: erase the semantic theme category back to a type
        // alias, allowing an ordinary noun phrase through the typed boundary.
        assert_ne!(
            std::any::TypeId::of::<ReducedRecipientPassiveTheme>(),
            std::any::TypeId::of::<NounPhrase>(),
        );
    }

    #[test]
    fn reduced_passive_adjunct_builder_rejects_a_non_recipient_frame() {
        // Mutation caught: validate only the trailing bare adjunct and allow
        // this specialized extension to manufacture a reduced-passive value
        // from an ordinary transitive frame.
        let (mut predicate, adjunct) = reduced_predicate_and_adjunct();
        predicate.head.verb.verb = crate::word::Verb::Word(Vocab::Draw);
        assert!(build_reduced_recipient_passive_nominal_adjunct(predicate, adjunct).is_err());
    }

    #[test]
    fn reduced_passive_nominal_builder_rejects_a_non_theme_object() {
        // Mutation caught: check only that the verb owns some recipient-
        // passive frame while accepting an arbitrary direct object in the
        // retained-theme slot.
        let (mut predicate, _) = reduced_predicate_and_adjunct();
        predicate.kind.object = crate::syntax::PredicateObject::NounPhrase(
            NounPhrase::from_nominal_declaration(card_nominal()),
        );
        assert!(build_nominal_reduced_recipient_passive(card_nominal(), predicate).is_err());
    }

    #[test]
    fn reduced_passive_direct_ast_builds_with_frame_theme_and_adjunct_intact() {
        // Mutation caught: over-tighten either role gate, drop the retained
        // damage theme or trailing adjunct during a split/rebuild, or make the
        // valid direct AST unrenderable through the generated nominal family.
        let (predicate, adjunct) = reduced_predicate_and_adjunct();
        let predicate =
            build_reduced_recipient_passive_nominal_adjunct(predicate, adjunct).unwrap();
        let base =
            build_nominal_determiner(crate::determiner::indefinite(), card_nominal()).unwrap();
        let value = build_nominal_reduced_recipient_passive(base, predicate).unwrap();
        assert_eq!(
            selected_nominal_construction(&value),
            "nominal_reduced_recipient_passive"
        );
        let (base, predicate) = parts_nominal_reduced_recipient_passive(&value);
        assert_eq!(
            build_nominal_reduced_recipient_passive(base, predicate).unwrap(),
            value
        );
        assert_eq!(
            crate::render_fragment(
                &crate::Fragment::Nominal(NounPhrase::from_nominal_declaration(value)),
                "Test Card",
                false,
            )
            .unwrap(),
            "a card dealt damage this way"
        );
    }

    #[test]
    fn conjoined_postpositive_pp_builder_requires_a_past_participle_and_by() {
        // Mutations caught: omit either half of the declared gate,
        // admitting an ordinary adjective before `by` or a past participle
        // followed by an unrelated preposition.
        let base = || {
            build_nominal_postpositive_adjective(
                card_nominal(),
                participle(crate::word::Tense::Present, Vocab::Block),
            )
            .unwrap()
        };
        assert!(
            build_nominal_postpositive_adjective_conjoined_prepositional(
                base(),
                Conjunction::Or,
                red_adjective(),
                preposition_with_card(Preposition::By),
            )
            .is_err()
        );
        assert!(
            build_nominal_postpositive_adjective_conjoined_prepositional(
                base(),
                Conjunction::Or,
                participle(crate::word::Tense::Past, Vocab::Block),
                preposition_with_card(Preposition::In),
            )
            .is_err()
        );
    }

    #[test]
    fn conjoined_postpositive_pp_inverse_rejects_an_unlicensed_direct_ast_before_visiting() {
        // Mutation caught: enforce the participle/`by` contract only at the
        // checked builder while the generated inverse still accepts a raw
        // invariant-breaking value.
        let invalid = NominalPhrase::from_projection_parts(
            None,
            Vec::new(),
            NounInstance::unchecked_singular(Noun::Word(Vocab::Card)),
            vec![NominalComplement::CoordinatedAdjective(
                CoordinatedAdjectivePhrase::from_declaration_parts(
                    Box::new(participle(crate::word::Tense::Present, Vocab::Block)),
                    vec![AdjectivePhraseCoordination::from_declaration_parts(
                        Some(Conjunction::Or),
                        red_adjective()
                            .try_attach_declared_prepositional(preposition_with_card(
                                Preposition::By,
                            ))
                            .unwrap(),
                    )],
                ),
            )],
        );
        assert_nominal_inverse_rejects(&invalid);
    }

    #[test]
    fn licensed_conjoined_past_participle_by_phrase_is_exact_and_rebuildable() {
        // Mutation caught: make the restored gate too broad or too narrow,
        // lose the PP during destructuring, or route the valid stored shape
        // through a different nominal inverse.
        let source = "creature blocking or blocked by this creature";
        let value = parsed_nominal(source);
        assert_eq!(
            selected_nominal_construction(&value),
            "nominal_postpositive_adjective_conjoined_prepositional"
        );
        let (nominal, conjunction, adjective, preposition) =
            parts_nominal_postpositive_adjective_conjoined_prepositional(&value);
        assert_eq!(
            build_nominal_postpositive_adjective_conjoined_prepositional(
                nominal,
                conjunction,
                adjective,
                preposition,
            )
            .unwrap(),
            value
        );
        assert_eq!(
            crate::render_fragment(
                &crate::Fragment::Nominal(NounPhrase::from_nominal_declaration(value)),
                "Test Card",
                false,
            )
            .unwrap(),
            source
        );
    }

    #[test]
    fn determiner_builder_rejects_duplicate_late_and_cardinality_mutations() {
        // Mutations caught: overwrite an existing determiner, attach a
        // determiner after the PP phase, or ignore determiner/head
        // cardinality. The control performs the same two legal operations in
        // their declared order.
        let determined =
            build_nominal_determiner(crate::determiner::indefinite(), card_nominal()).unwrap();
        assert!(build_nominal_determiner(crate::determiner::the(), determined).is_err());

        let attached =
            build_nominal_prepositional(card_nominal(), preposition_with_card(Preposition::In))
                .unwrap();
        assert!(build_nominal_determiner(crate::determiner::the(), attached).is_err());

        let plural =
            build_nominal_noun(NounInstance::unchecked_plural(Noun::Word(Vocab::Card))).unwrap();
        assert!(build_nominal_determiner(crate::determiner::each(), plural).is_err());

        let legal = build_nominal_prepositional(
            build_nominal_determiner(crate::determiner::the(), card_nominal()).unwrap(),
            preposition_with_card(Preposition::In),
        );
        assert!(legal.is_ok());
    }

    #[test]
    fn article_onset_mutation_is_rejected_before_nominal_lowering() {
        // Mutation caught: stop threading the leading modifier/head sound to
        // the indefinite determiner gate, making `a` and `an`
        // interchangeable. Both valid controls use the same lexical heads.
        for source in ["an ability", "a card"] {
            assert!(
                crate::parse_fragment(
                    source,
                    &crate::Catalogs::default(),
                    crate::FragmentKind::Nominal,
                    "Test Card",
                    false,
                )
                .fragment()
                .is_some(),
                "valid article fixture {source:?}",
            );
        }
        for source in ["a ability", "an card"] {
            assert!(
                crate::parse_fragment(
                    source,
                    &crate::Catalogs::default(),
                    crate::FragmentKind::Nominal,
                    "Test Card",
                    false,
                )
                .fragment()
                .is_none(),
                "article-onset mutation {source:?} must be rejected",
            );
        }
    }

    #[test]
    fn keyword_extension_rejects_conjunction_identity_mutation() {
        // Mutation caught: admit `or` where the keyword-grant rule requires
        // independent qualities joined only by `and`, for either explicit-
        // from or atom-carried-first argument families.
        let first_from =
            build_predicated_quality_from(Preposition::From, Some(ColorWord::Red), None, None)
                .unwrap();
        let second_from =
            build_predicated_quality_from(Preposition::From, Some(ColorWord::Blue), None, None)
                .unwrap();
        let explicit = build_predicated_argument_from_single(first_from).unwrap();
        assert!(
            build_predicated_argument_from_extend(explicit, Conjunction::Or, second_from.clone())
                .is_err()
        );

        let first_bare = build_predicated_quality_bare(Some(ColorWord::Red), None, None).unwrap();
        let atom_carried = build_predicated_argument_bare_single(first_bare).unwrap();
        assert!(
            build_predicated_argument_bare_extend(atom_carried, Conjunction::Or, second_from,)
                .is_err()
        );
    }

    #[test]
    fn devotion_pair_rejects_connective_and_field_arity_mutations() {
        // Mutations caught: widen the devotion pair connective beyond `and`,
        // or let erased chart construction call a three-field declaration
        // with a missing color field.
        assert!(
            build_devotion_color_pair(ColorWord::White, Conjunction::Or, ColorWord::Blue).is_err()
        );
        let declaration = NOMINAL_DECLARATION
            .constructions
            .iter()
            .find(|construction| construction.id == "devotion_color_pair")
            .expect("the devotion pair construction is declared");
        assert!(
            declaration.erased_builder.expect("checked builder exists")(vec![
                Box::new(ColorWord::White),
                Box::new(Conjunction::And),
            ])
            .is_err()
        );
    }

    #[test]
    fn generated_inverse_rejects_invalid_direct_nominal_invariants() {
        // Mutations caught: treat an inverse shape predicate as render-only
        // decoration, or let writable AST fields bypass cardinality, identity,
        // attachment-phase, reduced-passive, and specialized-head gates.
        let plural_cards = NounInstance::unchecked_plural(Noun::Word(Vocab::Card));
        assert_nominal_inverse_rejects(&NominalPhrase::from_projection_parts(
            Some(crate::determiner::each()),
            Vec::new(),
            plural_cards.clone(),
            Vec::new(),
        ));

        let one = Quantity::try_exact(crate::syntax::NumberLiteral {
            value: 1,
            numeral: crate::Numeral::Cardinal,
        })
        .unwrap();
        assert_nominal_inverse_rejects(&NominalPhrase::from_projection_parts(
            None,
            vec![NominalModifier::Quantity(one)],
            plural_cards,
            Vec::new(),
        ));

        assert_nominal_inverse_rejects(&NominalPhrase::from_projection_parts(
            None,
            Vec::new(),
            NounInstance::unchecked_singular(Noun::Word(Vocab::Ability)),
            vec![NominalComplement::KeywordArgument(KeywordArgument::Costed(
                KeywordCost::Symbols(vec![OracleSymbol::new("{2}").unwrap()]),
            ))],
        ));

        assert_nominal_inverse_rejects(&NominalPhrase::from_projection_parts(
            None,
            vec![NominalModifier::CombatStepName {
                participants: NounInstance::unchecked_singular(Noun::Word(Vocab::Card)),
            }],
            NounInstance::unchecked_singular(Noun::Word(Vocab::Step)),
            Vec::new(),
        ));

        let mut invalid_phase = parsed_nominal("card in a graveyard");
        invalid_phase
            .declaration_complements_mut()
            .insert(0, NominalComplement::Adjective(red_adjective()));
        assert_nominal_inverse_rejects(&invalid_phase);

        let invalid_devotion = parsed_nominal("your devotion to green");
        let (_, modifiers, _, complements) = invalid_devotion.into_projection_parts();
        let invalid_devotion = NominalPhrase::from_projection_parts(
            None,
            modifiers,
            NounInstance::unchecked_singular(Noun::Word(Vocab::Card)),
            complements,
        );
        assert_nominal_inverse_rejects(&invalid_devotion);

        let mut invalid_times = parsed_nominal("the number of times you drew a card");
        let [NominalComplement::Prepositional(of)] =
            invalid_times.declaration_complements_mut().as_mut_slice()
        else {
            panic!("number-of-times fixture keeps its of complement")
        };
        let crate::syntax::PrepositionalObjectKind::NounPhrase(object) =
            of.head_mut().object.test_kind_mut()
        else {
            panic!("the of complement keeps its noun-phrase object")
        };
        let crate::syntax::NounPhraseKind::Nominal(times) = object.kind() else {
            panic!("the of object is the times nominal")
        };
        let times = NominalPhrase::from_projection_parts(
            times.determiner().cloned(),
            times.modifiers().to_vec(),
            NounInstance::unchecked_plural(Noun::Word(Vocab::Card)),
            times.complements().to_vec(),
        );
        assert_nominal_inverse_rejects(&times);
        **object = NounPhrase::from_nominal_declaration(times);

        let invalid_determined_times = NominalPhrase::from_projection_parts(
            Some(crate::determiner::all()),
            Vec::new(),
            NounInstance::unchecked_plural(Noun::Word(Vocab::Card)),
            vec![NominalComplement::EventClause(Box::new(
                parsed_independent("You draw a card."),
            ))],
        );
        assert_nominal_inverse_rejects(&invalid_determined_times);
    }

    #[test]
    fn generated_builders_reject_wrong_specialized_roles_and_punctuation() {
        // Mutations caught: trust typed identity holes without enforcing their
        // provider role, normalize `or` to `and`, or let the binary and Oxford
        // postpositive constructors manufacture the same stored shape.
        assert!(
            build_nominal_combat_step_name(
                NounInstance::unchecked_plural(Noun::Word(Vocab::Card)),
                NounInstance::unchecked_singular(Noun::Word(Vocab::Step)),
            )
            .is_err()
        );
        assert!(
            build_nominal_keyword_symbol_argument(
                NounInstance::unchecked_singular(Noun::Word(Vocab::Ability)),
                Some(OracleSymbol::new("{2}").unwrap()),
                None,
            )
            .is_err()
        );

        let bare = build_predicated_quality_bare(Some(ColorWord::Red), None, None).unwrap();
        assert!(build_predicated_argument_from_single(bare).is_err());
        let explicit =
            build_predicated_quality_from(Preposition::From, Some(ColorWord::Red), None, None)
                .unwrap();
        assert!(build_predicated_argument_bare_single(explicit).is_err());

        let postpositive_base =
            build_nominal_postpositive_adjective(card_nominal(), red_adjective()).unwrap();
        assert!(
            build_nominal_postpositive_adjective_oxford(
                postpositive_base,
                Conjunction::And,
                red_adjective(),
            )
            .is_err()
        );
        assert!(build_reduced_recipient_passive_theme(card_nominal()).is_err());
        assert!(
            build_nominal_times_clause(
                NounInstance::unchecked_plural(Noun::Word(Vocab::Card)),
                Box::new(parsed_independent("You draw a card.")),
            )
            .is_err()
        );
    }
}
