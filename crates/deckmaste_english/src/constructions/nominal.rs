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
use crate::syntax::AdjectiveComplement;
use crate::syntax::AdjectivePhrase;
use crate::syntax::AdjectivePhraseCoordination;
use crate::syntax::ComparisonComplement;
use crate::syntax::CoordinatedAdjectivePhrase;
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
use crate::word::BareNominalAdjunct;
use crate::word::ColorWord;
use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::NounInstanceKind;
use crate::word::VerbSlot;
use crate::word::Vocab;

type RulesObjectNominal = NominalPhrase;
type RulesObjectFollowupNominal = NominalPhrase;
type PredicatedQualityFrom = PredicatedQuality;
type PredicatedQualityBare = PredicatedQuality;
type PredicatedArgumentFrom = PredicatedArgument;
type PredicatedArgumentBare = PredicatedArgument;
type ReducedRecipientPassiveTheme = NounPhrase;
type SymbolSequence = Vec<OracleSymbol>;

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn prepend_modifier(mut nominal: NominalPhrase, modifier: NominalModifier) -> NominalPhrase {
    nominal.modifiers.insert(0, modifier);
    nominal
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

fn split_nominal_adjective(value: &NominalPhrase) -> (AdjectivePhrase, NominalPhrase) {
    let mut nominal = value.clone();
    let NominalModifier::Adjective {
        polarity: Polarity::Positive,
        phrase,
    } = nominal.modifiers.remove(0)
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
    } = nominal.modifiers.remove(0)
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
    Ok(NominalPhrase {
        determiner: None,
        modifiers: vec![NominalModifier::CombatStepName { participants }],
        head,
        complements: Vec::new(),
    })
}

fn split_nominal_combat_step_name(value: &NominalPhrase) -> (NounInstance, NounInstance) {
    let [NominalModifier::CombatStepName { participants }] = value.modifiers.as_slice() else {
        unreachable!("combat-step nominal has exactly its formative modifier")
    };
    (participants.clone(), value.head.clone())
}

fn make_nominal_quantity_modifier(
    quantity: Quantity,
    nominal: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !cardinality_accepts(quantity.noun_cardinality(), &nominal.head) {
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
    let NominalModifier::Quantity(quantity) = nominal.modifiers.remove(0) else {
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
    let NominalModifier::PowerToughness(stats) = nominal.modifiers.remove(0) else {
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
                .complements
                .push(NominalComplement::$variant(complement));
            Ok(nominal)
        }

        fn $parts(value: &NominalPhrase) -> (NominalPhrase, $ty) {
            let mut nominal = value.clone();
            let Some(NominalComplement::$variant(complement)) = nominal.complements.pop() else {
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
        .complements
        .push(NominalComplement::ReducedRecipientPassive(predicate));
    Ok(nominal)
}

fn split_nominal_reduced_recipient_passive(
    value: &NominalPhrase,
) -> (NominalPhrase, TransitivePredicate) {
    let mut nominal = value.clone();
    let Some(NominalComplement::ReducedRecipientPassive(predicate)) = nominal.complements.pop()
    else {
        unreachable!("reduced-recipient-passive parts require their final complement")
    };
    (nominal, predicate)
}
fn make_nominal_postpositive_adjective(
    mut nominal: NominalPhrase,
    adjective: AdjectivePhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !nominal.complements.is_empty() || has_postnominal_comparison(&nominal) {
        return Err(violation(
            "nominal_postpositive_adjective",
            "the nominal attachment phase is open",
        ));
    }
    nominal
        .complements
        .push(NominalComplement::Adjective(adjective));
    Ok(nominal)
}

fn split_nominal_postpositive_adjective(value: &NominalPhrase) -> (NominalPhrase, AdjectivePhrase) {
    let mut nominal = value.clone();
    let Some(NominalComplement::Adjective(adjective)) = nominal.complements.pop() else {
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
    ] = value.complements.as_slice()
    else {
        unreachable!("symbol keyword nominal carries one symbol-cost argument")
    };
    if let [symbol] = symbols.as_slice() {
        (value.head.clone(), Some(symbol.clone()), None)
    } else {
        (value.head.clone(), None, Some(symbols.clone()))
    }
}

fn keyword_nominal(head: NounInstance, argument: KeywordArgument) -> NominalPhrase {
    NominalPhrase {
        determiner: None,
        modifiers: Vec::new(),
        head,
        complements: vec![NominalComplement::KeywordArgument(argument)],
    }
}

fn keyword_atom_from_head(head: &NounInstance) -> Option<&CatalogAtom> {
    let NounInstanceKind::Mass(Noun::Catalog(atom)) = head.kind() else {
        return None;
    };
    (atom.kind == CatalogKind::KeywordAbility).then_some(atom)
}

fn make_predicated_quality_from(
    color: Option<ColorWord>,
    noun_phrase: Option<NounPhrase>,
) -> Result<PredicatedQuality, DeclarationViolation> {
    let quality = match (color, noun_phrase) {
        (Some(color), None) => Phrase::ColorWord(color),
        (None, Some(noun_phrase)) => Phrase::NounPhrase(Box::new(noun_phrase)),
        _ => {
            return Err(violation(
                "predicated_quality_from",
                "exactly one explicit-from quality is present",
            ));
        }
    };
    Ok(PredicatedQuality {
        preposition: Some(Preposition::From),
        quality,
    })
}

fn split_predicated_quality_from(
    value: &PredicatedQuality,
) -> (Option<ColorWord>, Option<NounPhrase>) {
    match &value.quality {
        Phrase::ColorWord(color) => (Some(*color), None),
        Phrase::NounPhrase(noun_phrase) => (None, Some((**noun_phrase).clone())),
        _ => unreachable!("explicit-from quality is a color or noun phrase"),
    }
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
    if quality.preposition != Some(Preposition::From) {
        return Err(violation(
            "predicated_argument_from_single",
            "quality carries the explicit from preposition",
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
            .all(|quality| quality.preposition == Some(Preposition::From))
    {
        return Err(violation(
            "predicated_argument_from_extend",
            "every quality carries the explicit from preposition",
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
        value.complements.as_slice()
    else {
        unreachable!("predicated keyword nominal carries one predicated argument")
    };
    (value.head.clone(), argument.clone())
}

fn make_rules_object_nominal_base(
    nominal: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    Ok(nominal)
}

fn split_rules_object_nominal_base(value: &NominalPhrase) -> NominalPhrase {
    value.clone()
}

fn make_rules_object_followup_nominal_relative(
    base: Option<RulesObjectNominal>,
    followup: Option<RulesObjectFollowupNominal>,
    relative: RelativeClause,
) -> Result<NominalPhrase, DeclarationViolation> {
    let ((Some(mut nominal), None) | (None, Some(mut nominal))) = (base, followup) else {
        return Err(violation(
            "rules_object_followup_nominal_relative",
            "exactly one rules-object source is present",
        ));
    };
    nominal
        .complements
        .push(NominalComplement::Relative(relative));
    Ok(nominal)
}

fn split_rules_object_followup_nominal_relative(
    value: &NominalPhrase,
) -> (
    Option<RulesObjectNominal>,
    Option<RulesObjectFollowupNominal>,
    RelativeClause,
) {
    let mut nominal = value.clone();
    let Some(NominalComplement::Relative(relative)) = nominal.complements.pop() else {
        unreachable!("rules-object followup ends in a relative")
    };
    (None, Some(nominal), relative)
}

nominal_complement_adapter!(
    make_rules_object_followup_nominal_prepositional,
    split_rules_object_followup_nominal_prepositional,
    PrepositionalPhrase,
    Prepositional,
    "rules_object_followup_nominal_prepositional"
);

fn make_reduced_recipient_passive_theme(
    nominal: NominalPhrase,
) -> Result<NounPhrase, DeclarationViolation> {
    if !matches!(nominal.head.noun(), Noun::Word(Vocab::Damage)) {
        return Err(violation(
            "reduced_recipient_passive_theme",
            "theme nominal is headed by damage",
        ));
    }
    Ok(NounPhrase::Nominal(nominal))
}

fn split_reduced_recipient_passive_theme(value: &NounPhrase) -> NominalPhrase {
    let NounPhrase::Nominal(nominal) = value else {
        unreachable!("recipient-passive theme is nominal")
    };
    nominal.clone()
}

fn make_reduced_recipient_passive_nominal_adjunct(
    mut predicate: TransitivePredicate,
    noun_phrase: NounPhrase,
) -> Result<TransitivePredicate, DeclarationViolation> {
    let NounPhrase::Nominal(nominal) = &noun_phrase else {
        return Err(violation(
            "reduced_recipient_passive_nominal_adjunct",
            "adjunct is a nominal noun phrase",
        ));
    };
    let adjunct = match nominal.head.noun().bare_nominal_adjunct() {
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
    let previous = nominal.complements.pop().ok_or_else(|| {
        violation(
            "nominal_postpositive_adjective_conjoined",
            "a preceding postpositive adjective exists",
        )
    })?;
    let coordinated = match previous {
        NominalComplement::Adjective(first) => CoordinatedAdjectivePhrase {
            first: Box::new(first),
            rest: vec![AdjectivePhraseCoordination {
                conjunction,
                phrase: adjective,
            }],
        },
        NominalComplement::CoordinatedAdjective(mut coordinated) => {
            coordinated.rest.push(AdjectivePhraseCoordination {
                conjunction,
                phrase: adjective,
            });
            coordinated
        }
        _ => {
            return Err(violation(
                "nominal_postpositive_adjective_conjoined",
                "a preceding postpositive adjective exists",
            ));
        }
    };
    nominal
        .complements
        .push(NominalComplement::CoordinatedAdjective(coordinated));
    Ok(nominal)
}

fn make_nominal_postpositive_adjective_conjoined_prepositional(
    nominal: NominalPhrase,
    conjunction: Conjunction,
    mut adjective: AdjectivePhrase,
    preposition: PrepositionalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    if !matches!(
        nominal.complements.as_slice(),
        [NominalComplement::Adjective(_)]
    ) {
        return Err(violation(
            "nominal_postpositive_adjective_conjoined_prepositional",
            "a binary continuation follows exactly one postpositive adjective",
        ));
    }
    let conjunction = noun_phrase_conjunction(conjunction)?;
    adjective
        .complements
        .push(AdjectiveComplement::Prepositional(preposition));
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
    let (nominal, conjunction, mut adjective) = split_postpositive(value, true);
    let Some(AdjectiveComplement::Prepositional(preposition)) = adjective.complements.pop() else {
        unreachable!("conjoined participle carries its by phrase")
    };
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
    if !matches!(
        nominal.complements.as_slice(),
        [NominalComplement::Adjective(_)]
    ) {
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
    let extends_open_run = match nominal.complements.as_slice() {
        [NominalComplement::Adjective(_)] => true,
        [NominalComplement::CoordinatedAdjective(CoordinatedAdjectivePhrase { rest, .. })] => rest
            .last()
            .is_some_and(|member| member.conjunction.is_none()),
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
        nominal.complements.as_slice(),
        [NominalComplement::CoordinatedAdjective(CoordinatedAdjectivePhrase {
            rest,
            ..
        })] if !rest.is_empty()
            && rest.iter().all(|member| member.conjunction.is_none())
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
    let Some(NominalComplement::CoordinatedAdjective(mut coordinated)) = nominal.complements.pop()
    else {
        unreachable!("postpositive continuation ends in coordinated adjectives")
    };
    let continuation = coordinated
        .rest
        .pop()
        .expect("coordinated adjective has a continuation");
    assert_eq!(continuation.conjunction.is_some(), conjoined);
    nominal.complements.push(if coordinated.rest.is_empty() {
        NominalComplement::Adjective(*coordinated.first)
    } else {
        NominalComplement::CoordinatedAdjective(coordinated)
    });
    (nominal, continuation.conjunction, continuation.phrase)
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
    let adjective = nominal.modifiers.iter_mut().rev().find_map(|modifier| {
        let NominalModifier::Adjective { phrase, .. } = modifier else {
            return None;
        };
        (!phrase.complements.iter().any(|complement| {
            matches!(
                complement,
                AdjectiveComplement::Comparison(_) | AdjectiveComplement::PostnominalComparison(_)
            )
        }))
        .then_some(phrase)
    });
    let Some(adjective) = adjective else {
        return Err(violation(
            "nominal_comparison",
            "a pending adjective accepts the comparison",
        ));
    };
    adjective
        .complements
        .push(AdjectiveComplement::PostnominalComparison(comparison));
    Ok(nominal)
}

fn split_nominal_comparison(value: &NominalPhrase) -> (NominalPhrase, ComparisonComplement) {
    let mut nominal = value.clone();
    let adjective = nominal
        .modifiers
        .iter_mut()
        .rev()
        .find_map(|modifier| match modifier {
            NominalModifier::Adjective { phrase, .. } => Some(phrase),
            _ => None,
        })
        .expect("comparison nominal has an adjective modifier");
    let Some(AdjectiveComplement::PostnominalComparison(comparison)) = adjective.complements.pop()
    else {
        unreachable!("comparison nominal stores its postnominal comparison")
    };
    (nominal, comparison)
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
    Ok(NominalPhrase {
        determiner: None,
        modifiers: Vec::new(),
        head: NounInstance::Singular(Noun::Catalog(head)),
        complements: vec![NominalComplement::Devotion(colors)],
    })
}

fn split_nominal_devotion(value: &NominalPhrase) -> (CatalogAtom, DevotionColors) {
    let Noun::Catalog(head) = value.head.noun() else {
        unreachable!("devotion nominal has its catalog head")
    };
    let [NominalComplement::Devotion(colors)] = value.complements.as_slice() else {
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
    Ok(NominalPhrase {
        determiner: None,
        modifiers: Vec::new(),
        head,
        complements: vec![NominalComplement::EventClause(clause)],
    })
}

fn split_nominal_times_clause(value: &NominalPhrase) -> (NounInstance, Box<IndependentClause>) {
    let [NominalComplement::EventClause(clause)] = value.complements.as_slice() else {
        unreachable!("times nominal carries one event clause")
    };
    (value.head.clone(), clause.clone())
}

fn has_postnominal_comparison(value: &NominalPhrase) -> bool {
    value.modifiers.iter().any(|modifier| {
        matches!(
            modifier,
            NominalModifier::Adjective { phrase, .. }
                if phrase
                    .complements
                    .iter()
                    .any(|complement| matches!(complement, AdjectiveComplement::PostnominalComparison(_)))
        )
    })
}

fn general_attachment_base_is_admitted(value: &NominalPhrase) -> bool {
    !has_postnominal_comparison(value)
        && !matches!(
            value.complements.last(),
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
                .complements
                .get(..value.complements.len().saturating_sub(1))
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
        .complements
        .iter()
        .all(|complement| matches!(complement, NominalComplement::Prepositional(_)))
}

fn is_devotion_body(value: &NominalPhrase) -> bool {
    matches!(
        (value.head.kind(), value.complements.as_slice()),
        (
            NounInstanceKind::Singular(Noun::Catalog(atom)),
            [NominalComplement::Devotion(_)]
        ) if atom.kind == CatalogKind::RulesBundle && atom.canonical() == "devotion"
    )
}

fn is_times_clause_body(value: &NominalPhrase) -> bool {
    matches!(
        (value.head.kind(), value.complements.as_slice()),
        (
            NounInstanceKind::Plural(Noun::Word(Vocab::Time)),
            [NominalComplement::EventClause(_)]
        )
    )
}

fn prefix_is_outermost(value: &NominalPhrase) -> bool {
    value.determiner.is_none()
        && !has_postnominal_comparison(value)
        && (value.complements.is_empty() || is_devotion_body(value) || is_times_clause_body(value))
}

fn is_nominal_adjective(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers.first(),
            Some(NominalModifier::Adjective {
                polarity: Polarity::Positive,
                ..
            })
        )
}

fn is_nominal_noun_modifier(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers.first(),
            Some(NominalModifier::Noun {
                polarity: Polarity::Positive,
                ..
            })
        )
}

fn is_nominal_combat_step_name(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers.as_slice(),
            [NominalModifier::CombatStepName { participants }]
                if is_combat_step_participants(participants)
        )
        && is_combat_step_head(&value.head)
}

fn is_nominal_negated_modifier(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers.first(),
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
            value.modifiers.first(),
            Some(NominalModifier::Quantity(quantity))
                if cardinality_accepts(quantity.noun_cardinality(), &value.head)
        )
}

fn is_nominal_power_toughness_modifier(value: &NominalPhrase) -> bool {
    prefix_is_outermost(value)
        && matches!(
            value.modifiers.first(),
            Some(NominalModifier::PowerToughness(_))
        )
}

fn is_nominal_determiner(value: &NominalPhrase) -> bool {
    value
        .determiner
        .as_ref()
        .is_some_and(|determiner| cardinality_accepts(determiner.noun_cardinality(), &value.head))
        && !has_postnominal_comparison(value)
        && (value.complements.is_empty() || is_devotion_body(value) || is_times_clause_body(value))
}

fn final_complement(value: &NominalPhrase) -> Option<&NominalComplement> {
    (!has_postnominal_comparison(value))
        .then(|| value.complements.last())
        .flatten()
}

fn is_nominal_prepositional(value: &NominalPhrase) -> bool {
    prefix_before_final_allows_general_attachment(value)
        && matches!(
            final_complement(value),
            Some(NominalComplement::Prepositional(_))
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
    keyword_atom_from_head(&value.head)
}

fn is_nominal_keyword_symbol_single(value: &NominalPhrase) -> bool {
    keyword_atom(value).is_some()
        && value.determiner.is_none()
        && value.modifiers.is_empty()
        && matches!(
            value.complements.as_slice(),
            [NominalComplement::KeywordArgument(KeywordArgument::Costed(
                KeywordCost::Symbols(symbols)
            ))] if symbols.len() == 1
        )
}

fn is_nominal_keyword_symbol_sequence(value: &NominalPhrase) -> bool {
    keyword_atom(value).is_some()
        && value.determiner.is_none()
        && value.modifiers.is_empty()
        && matches!(
            value.complements.as_slice(),
            [NominalComplement::KeywordArgument(KeywordArgument::Costed(
                KeywordCost::Symbols(symbols)
            ))] if symbols.len() >= 2
        )
}

fn is_nominal_keyword_predicated_argument(value: &NominalPhrase) -> bool {
    keyword_atom(value).is_some_and(|atom| !crate::grammar::keyword_atom_carries_from(atom))
        && value.determiner.is_none()
        && value.modifiers.is_empty()
        && matches!(
            value.complements.as_slice(),
            [NominalComplement::KeywordArgument(KeywordArgument::Predicated(argument))]
                if !argument.qualities.is_empty()
                    && argument.qualities.iter().all(|quality| quality.preposition == Some(Preposition::From))
        )
}

fn is_nominal_keyword_atom_carried_predicated_argument(value: &NominalPhrase) -> bool {
    keyword_atom(value).is_some_and(crate::grammar::keyword_atom_carries_from)
        && value.determiner.is_none()
        && value.modifiers.is_empty()
        && matches!(
            value.complements.as_slice(),
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
    matches!(
        value.complements.as_slice(),
        [NominalComplement::Adjective(_)]
    )
}

fn is_nominal_postpositive_adjective_conjoined_prepositional(value: &NominalPhrase) -> bool {
    value.complements.len() == 1
        && final_coordinated_adjective(value).is_some_and(|coordinated| {
            coordinated.rest.len() == 1
                && coordinated.rest.last().is_some_and(|last| {
                    last.conjunction.is_some()
                        && matches!(
                            last.phrase.complements.last(),
                            Some(AdjectiveComplement::Prepositional(_))
                        )
                })
        })
}

fn is_nominal_postpositive_adjective_conjoined(value: &NominalPhrase) -> bool {
    value.complements.len() == 1
        && final_coordinated_adjective(value).is_some_and(|coordinated| {
            coordinated.rest.len() == 1
                && coordinated.rest[0].conjunction.is_some()
                && !matches!(
                    coordinated.rest[0].phrase.complements.last(),
                    Some(AdjectiveComplement::Prepositional(_))
                )
        })
}

fn is_nominal_postpositive_adjective_asyndetic(value: &NominalPhrase) -> bool {
    value.complements.len() == 1
        && final_coordinated_adjective(value)
            .and_then(|coordinated| coordinated.rest.last())
            .is_some_and(|last| last.conjunction.is_none())
}

fn is_nominal_postpositive_adjective_oxford(value: &NominalPhrase) -> bool {
    value.complements.len() == 1
        && final_coordinated_adjective(value).is_some_and(|coordinated| {
            coordinated.rest.len() >= 2
                && coordinated
                    .rest
                    .last()
                    .is_some_and(|last| last.conjunction.is_some())
        })
}

fn is_nominal_comparison(value: &NominalPhrase) -> bool {
    has_postnominal_comparison(value) && has_only_prepositional_complements(value)
}

fn is_nominal_devotion(value: &NominalPhrase) -> bool {
    value.determiner.is_none() && value.modifiers.is_empty() && is_devotion_body(value)
}

fn is_nominal_times_clause(value: &NominalPhrase) -> bool {
    value.determiner.is_none() && value.modifiers.is_empty() && is_times_clause_body(value)
}

deckmaste_constructions_macro::constructions! {
    group nominal;

    lens nominal_phrase bind NominalPhrase {
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
        derive features = nominal();
        form only @ 0 = head;
        selection unique;
    }

    construction nominal_adjective: NominalPhrase {
        bind NominalPhrase via make_nominal_adjective, split_nominal_adjective {
            adjective: hole AdjectivePhrase,
            nominal: hole NominalPhrase,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_adjective) = adjective nominal;
        selection unique;
    }

    construction nominal_noun_modifier: NominalPhrase {
        bind NominalPhrase via make_nominal_noun_modifier, split_nominal_noun_modifier {
            noun: hole NounInstance,
            nominal: hole NominalPhrase,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_noun_modifier) = noun nominal;
        selection unique;
    }

    construction nominal_combat_step_name: NominalPhrase {
        bind NominalPhrase via make_nominal_combat_step_name, split_nominal_combat_step_name {
            participants: identity NounInstance via CombatStepParticipants,
            head: identity NounInstance via CombatStepHead,
        }
        derive features = nominal();
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
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_negated_modifier) = identity(modifier) nominal;
        selection unique;
    }

    construction nominal_quantity_modifier: NominalPhrase {
        bind NominalPhrase via make_nominal_quantity_modifier, split_nominal_quantity_modifier {
            quantity: hole Quantity,
            nominal: hole NominalPhrase,
        }
        derive features = nominal();
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
        derive features = nominal();
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
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_determiner) = determiner nominal;
        dominates nominal_comparison;
        selection unique;
    }

    construction nominal_prepositional: NominalPhrase {
        bind NominalPhrase via make_nominal_prepositional, split_nominal_prepositional {
            nominal: hole NominalPhrase,
            preposition: hole PrepositionalPhrase,
        }
        derive features = nominal();
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
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_infinitive) = nominal infinitive;
        selection unique;
    }

    construction nominal_quantity_complement: NominalPhrase {
        bind NominalPhrase via make_nominal_quantity_complement, split_nominal_quantity_complement {
            nominal: hole NominalPhrase,
            quantity: hole Quantity,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_quantity_complement) = nominal quantity;
        selection unique;
    }

    construction nominal_keyword_symbol_argument: NominalPhrase {
        bind NominalPhrase via make_nominal_keyword_symbol_argument, split_nominal_keyword_symbol_argument {
            head: identity NounInstance via SymbolArgumentKeywordNoun,
            symbol: opt lex OracleSymbol,
            sequence: opt lex SymbolSequence,
        }
        derive features = nominal();
        form symbol @ 0 when symbol.is_some() inverse check(is_nominal_keyword_symbol_single) = identity(head) lex(symbol);
        form sequence @ 1 inverse check(is_nominal_keyword_symbol_sequence) otherwise = identity(head) lex(sequence);
        selection unique;
    }

    construction predicated_quality_from: PredicatedQualityFrom {
        bind PredicatedQuality via make_predicated_quality_from, split_predicated_quality_from {
            color: opt lex ColorWord,
            noun_phrase: opt hole NounPhrase,
        }
        derive features = nominal();
        form color @ 0 when color.is_some() = "from" lex(color);
        form noun_phrase @ 1 otherwise = "from" noun_phrase;
        selection unique;
    }

    construction predicated_argument_from_single: PredicatedArgumentFrom {
        bind PredicatedArgument via make_predicated_argument_from_single, split_predicated_argument_single {
            quality: hole PredicatedQualityFrom,
        }
        derive features = nominal();
        form only @ 0 = quality;
        selection unique;
    }

    construction predicated_argument_from_extend: PredicatedArgumentFrom {
        bind PredicatedArgument via make_predicated_argument_from_extend, split_predicated_argument_extend {
            argument: hole PredicatedArgumentFrom,
            conjunction: lex Conjunction,
            quality: hole PredicatedQualityFrom,
        }
        require conjunction in [And];
        derive features = nominal();
        form only @ 0 = argument lex(conjunction) quality;
        selection unique;
    }

    construction nominal_keyword_predicated_argument: NominalPhrase {
        bind NominalPhrase via make_nominal_keyword_predicated_argument, split_nominal_keyword_predicated_argument {
            head: identity NounInstance via ExplicitPredicatedKeywordNoun,
            argument: hole PredicatedArgumentFrom,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_keyword_predicated_argument) = identity(head) argument;
        selection unique;
    }

    construction predicated_quality_bare: PredicatedQualityBare {
        bind PredicatedQuality via make_predicated_quality_bare, split_predicated_quality_bare {
            color: opt lex ColorWord,
            adjective: opt hole AdjectivePhrase,
            noun_phrase: opt hole NounPhrase,
        }
        derive features = nominal();
        form color @ 0 when color.is_some() = lex(color);
        form adjective @ 1 when all(color.is_none(), adjective.is_some()) = adjective;
        form noun_phrase @ 2 otherwise = noun_phrase;
        selection unique;
    }

    construction predicated_argument_bare_single: PredicatedArgumentBare {
        bind PredicatedArgument via make_predicated_argument_bare_single, split_predicated_argument_single {
            quality: hole PredicatedQualityBare,
        }
        derive features = nominal();
        form only @ 0 = quality;
        selection unique;
    }

    construction predicated_argument_bare_extend: PredicatedArgumentBare {
        bind PredicatedArgument via make_predicated_argument_bare_extend, split_predicated_argument_extend {
            argument: hole PredicatedArgumentBare,
            conjunction: lex Conjunction,
            quality: hole PredicatedQualityFrom,
        }
        require conjunction in [And];
        derive features = nominal();
        form only @ 0 = argument lex(conjunction) quality;
        selection unique;
    }

    construction nominal_keyword_atom_carried_predicated_argument: NominalPhrase {
        bind NominalPhrase via make_nominal_keyword_atom_carried_predicated_argument, split_nominal_keyword_predicated_argument {
            head: identity NounInstance via AtomCarriedPredicatedKeywordNoun,
            argument: hole PredicatedArgumentBare,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_keyword_atom_carried_predicated_argument) = identity(head) argument;
        selection unique;
    }

    construction nominal_relative: NominalPhrase {
        bind NominalPhrase via make_nominal_relative, split_nominal_relative {
            nominal: hole NominalPhrase,
            relative: hole RelativeClause,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_relative) = nominal relative;
        dominates nominal_prepositional;
        selection unique;
    }

    construction rules_object_nominal_base: RulesObjectNominal {
        bind NominalPhrase via make_rules_object_nominal_base, split_rules_object_nominal_base {
            nominal: hole NominalPhrase,
        }
        derive features = nominal();
        form only @ 0 = nominal;
        selection unique;
    }

    construction rules_object_followup_nominal_relative: RulesObjectFollowupNominal {
        bind NominalPhrase via make_rules_object_followup_nominal_relative, split_rules_object_followup_nominal_relative {
            base: opt hole RulesObjectNominal,
            followup: opt hole RulesObjectFollowupNominal,
            relative: hole RelativeClause,
        }
        derive features = nominal();
        form base @ 0 when base.is_some() = base relative;
        form followup @ 1 otherwise = followup relative;
        selection unique;
    }

    construction rules_object_followup_nominal_prepositional: RulesObjectFollowupNominal {
        bind NominalPhrase via make_rules_object_followup_nominal_prepositional, split_rules_object_followup_nominal_prepositional {
            nominal: hole RulesObjectFollowupNominal,
            preposition: hole PrepositionalPhrase,
        }
        derive features = nominal();
        form only @ 0 = nominal preposition;
        selection unique;
    }

    construction nominal_reduced_recipient_passive: NominalPhrase {
        bind NominalPhrase via make_nominal_reduced_recipient_passive, split_nominal_reduced_recipient_passive {
            nominal: hole NominalPhrase,
            predicate: hole TransitivePredicate,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_reduced_recipient_passive) = nominal predicate;
        dominates nominal_noun;
        selection unique;
    }

    construction reduced_recipient_passive_theme: ReducedRecipientPassiveTheme {
        bind NounPhrase via make_reduced_recipient_passive_theme, split_reduced_recipient_passive_theme {
            nominal: hole NominalPhrase,
        }
        derive features = nominal();
        form only @ 0 = nominal;
        selection unique;
    }

    construction reduced_recipient_passive_nominal_adjunct: TransitivePredicate {
        bind TransitivePredicate via make_reduced_recipient_passive_nominal_adjunct, split_reduced_recipient_passive_nominal_adjunct {
            predicate: hole TransitivePredicate,
            noun_phrase: hole NounPhrase,
        }
        derive features = nominal();
        form only @ 0 = predicate noun_phrase;
        selection unique;
    }

    construction nominal_postpositive_adjective: NominalPhrase {
        bind NominalPhrase via make_nominal_postpositive_adjective, split_nominal_postpositive_adjective {
            nominal: hole NominalPhrase,
            adjective: hole AdjectivePhrase,
        }
        derive features = nominal();
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
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_postpositive_adjective_conjoined_prepositional) = nominal lex(conjunction) adjective preposition;
        selection unique;
    }

    construction nominal_postpositive_adjective_conjoined: NominalPhrase {
        bind NominalPhrase via make_nominal_postpositive_adjective_conjoined, split_nominal_postpositive_adjective_conjoined {
            nominal: hole NominalPhrase,
            conjunction: lex Conjunction,
            adjective: hole AdjectivePhrase,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_postpositive_adjective_conjoined) = nominal lex(conjunction) adjective;
        selection unique;
    }

    construction nominal_postpositive_adjective_asyndetic: NominalPhrase {
        bind NominalPhrase via make_nominal_postpositive_adjective_asyndetic, split_nominal_postpositive_adjective_asyndetic {
            nominal: hole NominalPhrase,
            adjective: hole AdjectivePhrase,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_postpositive_adjective_asyndetic) = nominal "," adjective;
        selection unique;
    }

    construction nominal_postpositive_adjective_oxford: NominalPhrase {
        bind NominalPhrase via make_nominal_postpositive_adjective_oxford, split_nominal_postpositive_adjective_oxford {
            nominal: hole NominalPhrase,
            conjunction: lex Conjunction,
            adjective: hole AdjectivePhrase,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_postpositive_adjective_oxford) = nominal "," lex(conjunction) adjective;
        selection unique;
    }

    construction nominal_comparison: NominalPhrase {
        bind NominalPhrase via make_nominal_comparison, split_nominal_comparison {
            nominal: hole NominalPhrase,
            comparison: hole ComparisonComplement,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_comparison) = nominal comparison;
        selection unique;
    }

    construction nominal_devotion: NominalPhrase {
        bind NominalPhrase via make_nominal_devotion, split_nominal_devotion {
            head: identity CatalogAtom via DevotionValue,
            colors: hole DevotionColors,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_devotion) = identity(head) "to" colors;
        selection unique;
    }

    construction devotion_color_single: DevotionColors {
        bind DevotionColors via make_devotion_color_single, split_devotion_color_single {
            color: lex ColorWord,
        }
        derive features = nominal();
        form only @ 0 = lex(color);
        selection unique;
    }

    construction devotion_color_pair: DevotionColors {
        bind DevotionColors via make_devotion_color_pair, split_devotion_color_pair {
            first: lex ColorWord,
            conjunction: lex Conjunction,
            second: lex ColorWord,
        }
        require conjunction in [And];
        derive features = nominal();
        form only @ 0 = lex(first) lex(conjunction) lex(second);
        selection unique;
    }

    construction nominal_times_clause: NominalPhrase {
        bind NominalPhrase via make_nominal_times_clause, split_nominal_times_clause {
            head: identity NounInstance via TimesNoun,
            clause: hole box IndependentClause,
        }
        derive features = nominal();
        form only @ 0 inverse check(is_nominal_times_clause) = identity(head) clause;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&NOMINAL_DECLARATION];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word::Adjective;

    const M01_IDS: [&str; 37] = [
        "nominal_noun",
        "nominal_adjective",
        "nominal_noun_modifier",
        "nominal_combat_step_name",
        "nominal_negated_modifier",
        "nominal_quantity_modifier",
        "nominal_power_toughness_modifier",
        "nominal_determiner",
        "nominal_prepositional",
        "nominal_infinitive",
        "nominal_quantity_complement",
        "nominal_keyword_symbol_argument",
        "predicated_quality_from",
        "predicated_argument_from_single",
        "predicated_argument_from_extend",
        "nominal_keyword_predicated_argument",
        "predicated_quality_bare",
        "predicated_argument_bare_single",
        "predicated_argument_bare_extend",
        "nominal_keyword_atom_carried_predicated_argument",
        "nominal_relative",
        "rules_object_nominal_base",
        "rules_object_followup_nominal_relative",
        "rules_object_followup_nominal_prepositional",
        "nominal_reduced_recipient_passive",
        "reduced_recipient_passive_theme",
        "reduced_recipient_passive_nominal_adjunct",
        "nominal_postpositive_adjective",
        "nominal_postpositive_adjective_conjoined_prepositional",
        "nominal_postpositive_adjective_conjoined",
        "nominal_postpositive_adjective_asyndetic",
        "nominal_postpositive_adjective_oxford",
        "nominal_comparison",
        "nominal_devotion",
        "devotion_color_single",
        "devotion_color_pair",
        "nominal_times_clause",
    ];

    #[test]
    fn declaration_metadata_names_every_m01_builder_and_feature_projection() {
        // Mutations caught: omit or rename an atomic M01 row, retain a row
        // without its checked builder, or bypass declaration-selected feature
        // reduction for one nominal construction.
        assert_eq!(
            NOMINAL_DECLARATION
                .constructions
                .iter()
                .map(|construction| construction.id)
                .collect::<Vec<_>>(),
            M01_IDS,
        );
        assert!(
            NOMINAL_DECLARATION
                .constructions
                .iter()
                .all(|construction| construction.erased_builder.is_some()
                    && construction.bind_path.is_some()
                    && construction.feature_combinators.len() == 1
                    && construction.feature_combinators[0].combinator == "nominal")
        );
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
        build_nominal_noun(NounInstance::Singular(Noun::Word(Vocab::Card)))
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
        AdjectivePhrase {
            degree: None,
            head: Adjective::Color(ColorWord::Red),
            complements: Vec::new(),
        }
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
        let Some(crate::Fragment::Nominal(NounPhrase::Nominal(nominal))) = parsed.into_fragment()
        else {
            panic!("expected a nominal parse for {source:?}")
        };
        nominal
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
        reason = "the ordered table intentionally keeps all 37 atomic M01 projections visible in one drift gate"
    )]
    fn every_m01_checked_builder_round_trips_through_generated_parts() {
        // Mutations caught: omit a checked door or generated destructurer,
        // wire one declaration to the wrong adapter, reorder its fields, or
        // let a checked builder manufacture a value its parts projection does
        // not recognize. The final ordered assertion keeps this behavioral
        // table in lockstep with the complete atomic M01 declaration list.
        let mut exercised = Vec::new();
        macro_rules! records {
            ($id:literal, $value:expr, $rebuilt:expr) => {{
                let value = $value;
                let rebuilt = $rebuilt;
                assert_eq!(rebuilt, value, "{} projection", $id);
                exercised.push($id);
                value
            }};
        }

        let nominal_noun = build_nominal_noun(NounInstance::Singular(Noun::Word(Vocab::Card)))
            .expect("singular card is an admitted nominal head");
        let head = parts_nominal_noun(&nominal_noun).expect("head lens projects its sole field");
        records!(
            "nominal_noun",
            nominal_noun,
            build_nominal_noun(head).unwrap()
        );

        let nominal_adjective = build_nominal_adjective(red_adjective(), card_nominal()).unwrap();
        let (adjective, nominal) = parts_nominal_adjective(&nominal_adjective);
        records!(
            "nominal_adjective",
            nominal_adjective,
            build_nominal_adjective(adjective, nominal).unwrap()
        );

        let nominal_noun_modifier = build_nominal_noun_modifier(
            NounInstance::Singular(Noun::Word(Vocab::Ability)),
            card_nominal(),
        )
        .unwrap();
        let (noun, nominal) = parts_nominal_noun_modifier(&nominal_noun_modifier);
        records!(
            "nominal_noun_modifier",
            nominal_noun_modifier,
            build_nominal_noun_modifier(noun, nominal).unwrap()
        );

        let nominal_combat_step_name = build_nominal_combat_step_name(
            NounInstance::Plural(Noun::Agentive(crate::word::Verb::Word(Vocab::Attack))),
            NounInstance::Singular(Noun::Word(Vocab::Step)),
        )
        .unwrap();
        let (participants, head) = parts_nominal_combat_step_name(&nominal_combat_step_name);
        records!(
            "nominal_combat_step_name",
            nominal_combat_step_name,
            build_nominal_combat_step_name(participants, head).unwrap()
        );

        let modifier = NominalModifier::Noun {
            polarity: Polarity::Negative,
            noun: NounInstance::Singular(Noun::Word(Vocab::Ability)),
        };
        let nominal_negated_modifier =
            build_nominal_negated_modifier(modifier, card_nominal()).unwrap();
        let (modifier, nominal) = parts_nominal_negated_modifier(&nominal_negated_modifier)
            .expect("negative prefix is projected by the modifier lens");
        records!(
            "nominal_negated_modifier",
            nominal_negated_modifier,
            build_nominal_negated_modifier(modifier, nominal).unwrap()
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
            build_nominal_quantity_modifier(quantity, nominal).unwrap()
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
            build_nominal_power_toughness_modifier(stats, nominal).unwrap()
        );

        let nominal_determiner =
            build_nominal_determiner(Determiner::Indefinite, card_nominal()).unwrap();
        let (determiner, nominal) = parts_nominal_determiner(&nominal_determiner)
            .expect("determiner lens projects a determined nominal");
        records!(
            "nominal_determiner",
            nominal_determiner,
            build_nominal_determiner(determiner, nominal).unwrap()
        );

        let preposition = PrepositionalPhrase::simple(
            Preposition::In,
            Phrase::NounPhrase(Box::new(NounPhrase::Nominal(card_nominal()))),
        );
        let nominal_prepositional =
            build_nominal_prepositional(card_nominal(), preposition.clone()).unwrap();
        let (nominal, preposition) = parts_nominal_prepositional(&nominal_prepositional);
        records!(
            "nominal_prepositional",
            nominal_prepositional,
            build_nominal_prepositional(nominal, preposition.clone()).unwrap()
        );

        let IndependentClause::Imperative(predicate) = parsed_independent("Draw a card.") else {
            panic!("imperative fixture has an imperative predicate")
        };
        let infinitive = InfinitiveClause {
            negated: false,
            marker: crate::syntax::InfinitiveMarker::To,
            predicate: Box::new(predicate),
        };
        let nominal_infinitive = build_nominal_infinitive(card_nominal(), infinitive).unwrap();
        let (nominal, infinitive) = parts_nominal_infinitive(&nominal_infinitive);
        records!(
            "nominal_infinitive",
            nominal_infinitive,
            build_nominal_infinitive(nominal, infinitive).unwrap()
        );

        let nominal_quantity_complement =
            build_nominal_quantity_complement(card_nominal(), Quantity::try_x().unwrap()).unwrap();
        let (nominal, quantity) = parts_nominal_quantity_complement(&nominal_quantity_complement);
        records!(
            "nominal_quantity_complement",
            nominal_quantity_complement,
            build_nominal_quantity_complement(nominal, quantity).unwrap()
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
            build_nominal_keyword_symbol_argument(head, symbol, sequence).unwrap()
        );

        let predicated_quality_from =
            build_predicated_quality_from(Some(ColorWord::Red), None).unwrap();
        let (color, noun_phrase) = parts_predicated_quality_from(&predicated_quality_from);
        let predicated_quality_from = records!(
            "predicated_quality_from",
            predicated_quality_from,
            build_predicated_quality_from(color, noun_phrase).unwrap()
        );

        let predicated_argument_from_single =
            build_predicated_argument_from_single(predicated_quality_from.clone()).unwrap();
        let quality = parts_predicated_argument_from_single(&predicated_argument_from_single);
        let predicated_argument_from_single = records!(
            "predicated_argument_from_single",
            predicated_argument_from_single,
            build_predicated_argument_from_single(quality).unwrap()
        );

        let second_from = build_predicated_quality_from(Some(ColorWord::Black), None).unwrap();
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
            build_predicated_argument_from_extend(argument, conjunction, quality).unwrap()
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
            build_nominal_keyword_predicated_argument(head, argument).unwrap()
        );

        let predicated_quality_bare =
            build_predicated_quality_bare(Some(ColorWord::Blue), None, None).unwrap();
        let (color, adjective, noun_phrase) =
            parts_predicated_quality_bare(&predicated_quality_bare);
        let predicated_quality_bare = records!(
            "predicated_quality_bare",
            predicated_quality_bare,
            build_predicated_quality_bare(color, adjective, noun_phrase).unwrap()
        );

        let predicated_argument_bare_single =
            build_predicated_argument_bare_single(predicated_quality_bare).unwrap();
        let quality = parts_predicated_argument_bare_single(&predicated_argument_bare_single);
        let predicated_argument_bare_single = records!(
            "predicated_argument_bare_single",
            predicated_argument_bare_single,
            build_predicated_argument_bare_single(quality).unwrap()
        );

        let final_from = build_predicated_quality_from(Some(ColorWord::Black), None).unwrap();
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
            build_predicated_argument_bare_extend(argument, conjunction, quality).unwrap()
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
            build_nominal_keyword_atom_carried_predicated_argument(head, argument).unwrap()
        );

        let nominal_relative = parsed_nominal("creature you control");
        let (relative_base, relative) = parts_nominal_relative(&nominal_relative);
        records!(
            "nominal_relative",
            nominal_relative,
            build_nominal_relative(relative_base.clone(), relative.clone()).unwrap()
        );

        let rules_object_nominal_base = build_rules_object_nominal_base(card_nominal()).unwrap();
        let nominal = parts_rules_object_nominal_base(&rules_object_nominal_base);
        records!(
            "rules_object_nominal_base",
            rules_object_nominal_base,
            build_rules_object_nominal_base(nominal).unwrap()
        );

        let rules_object_followup_nominal_relative =
            build_rules_object_followup_nominal_relative(Some(relative_base), None, relative)
                .unwrap();
        let (base, followup, relative) =
            parts_rules_object_followup_nominal_relative(&rules_object_followup_nominal_relative);
        let rules_object_followup_nominal_relative = records!(
            "rules_object_followup_nominal_relative",
            rules_object_followup_nominal_relative,
            build_rules_object_followup_nominal_relative(base, followup, relative).unwrap()
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
            build_rules_object_followup_nominal_prepositional(nominal, preposition).unwrap()
        );

        let IndependentClause::Deontic(
            crate::syntax::Subject(NounPhrase::Nominal(reduced_subject)),
            _,
            _,
        ) = parsed_independent("A creature dealt damage this way can't block this turn.")
        else {
            panic!("reduced-passive fixture has a deontic nominal subject")
        };
        let (reduced_base, reduced_predicate) =
            parts_nominal_reduced_recipient_passive(&reduced_subject);
        records!(
            "nominal_reduced_recipient_passive",
            reduced_subject,
            build_nominal_reduced_recipient_passive(reduced_base, reduced_predicate.clone())
                .unwrap()
        );

        let damage = build_nominal_noun(NounInstance::Mass(Noun::Word(Vocab::Damage))).unwrap();
        let reduced_recipient_passive_theme =
            build_reduced_recipient_passive_theme(damage).unwrap();
        let nominal = parts_reduced_recipient_passive_theme(&reduced_recipient_passive_theme);
        records!(
            "reduced_recipient_passive_theme",
            reduced_recipient_passive_theme,
            build_reduced_recipient_passive_theme(nominal).unwrap()
        );

        let (predicate, noun_phrase) =
            parts_reduced_recipient_passive_nominal_adjunct(&reduced_predicate);
        records!(
            "reduced_recipient_passive_nominal_adjunct",
            reduced_predicate,
            build_reduced_recipient_passive_nominal_adjunct(predicate, noun_phrase).unwrap()
        );

        let nominal_postpositive_adjective =
            build_nominal_postpositive_adjective(card_nominal(), red_adjective()).unwrap();
        let (nominal, adjective) =
            parts_nominal_postpositive_adjective(&nominal_postpositive_adjective);
        records!(
            "nominal_postpositive_adjective",
            nominal_postpositive_adjective,
            build_nominal_postpositive_adjective(nominal, adjective).unwrap()
        );

        let blue_adjective = || AdjectivePhrase {
            degree: None,
            head: Adjective::Color(ColorWord::Blue),
            complements: Vec::new(),
        };
        let green_adjective = || AdjectivePhrase {
            degree: None,
            head: Adjective::Color(ColorWord::Green),
            complements: Vec::new(),
        };
        let prepositional_base =
            build_nominal_postpositive_adjective(card_nominal(), red_adjective()).unwrap();
        let nominal_postpositive_adjective_conjoined_prepositional =
            build_nominal_postpositive_adjective_conjoined_prepositional(
                prepositional_base,
                Conjunction::Or,
                blue_adjective(),
                PrepositionalPhrase::simple(
                    Preposition::By,
                    Phrase::NounPhrase(Box::new(NounPhrase::Nominal(card_nominal()))),
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
            .unwrap()
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
                .unwrap()
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
            build_nominal_postpositive_adjective_asyndetic(nominal, adjective).unwrap()
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
            build_nominal_postpositive_adjective_oxford(nominal, conjunction, adjective).unwrap()
        );

        let comparison = ComparisonComplement {
            marker: crate::syntax::ComparisonMarker::Than,
            standard: Box::new(Phrase::NounPhrase(Box::new(NounPhrase::Nominal(
                card_nominal(),
            )))),
        };
        let nominal_comparison = build_nominal_comparison(
            build_nominal_adjective(red_adjective(), card_nominal()).unwrap(),
            comparison,
        )
        .unwrap();
        let (nominal, comparison) = parts_nominal_comparison(&nominal_comparison);
        records!(
            "nominal_comparison",
            nominal_comparison,
            build_nominal_comparison(nominal, comparison).unwrap()
        );

        let mut nominal_devotion = parsed_nominal("your devotion to green");
        nominal_devotion.determiner = None;
        let (head, colors) = parts_nominal_devotion(&nominal_devotion);
        records!(
            "nominal_devotion",
            nominal_devotion,
            build_nominal_devotion(head, colors).unwrap()
        );

        let devotion_color_single = build_devotion_color_single(ColorWord::Green).unwrap();
        let color = parts_devotion_color_single(&devotion_color_single);
        records!(
            "devotion_color_single",
            devotion_color_single,
            build_devotion_color_single(color).unwrap()
        );

        let devotion_color_pair =
            build_devotion_color_pair(ColorWord::White, Conjunction::And, ColorWord::Black)
                .unwrap();
        let (first, conjunction, second) = parts_devotion_color_pair(&devotion_color_pair);
        records!(
            "devotion_color_pair",
            devotion_color_pair,
            build_devotion_color_pair(first, conjunction, second).unwrap()
        );

        let nominal_times_clause = build_nominal_times_clause(
            NounInstance::Plural(Noun::Word(Vocab::Time)),
            Box::new(parsed_independent("You draw a card.")),
        )
        .unwrap();
        let (head, clause) = parts_nominal_times_clause(&nominal_times_clause);
        records!(
            "nominal_times_clause",
            nominal_times_clause,
            build_nominal_times_clause(head, clause).unwrap()
        );

        assert_eq!(exercised, M01_IDS);
    }

    #[test]
    fn generated_inverse_selects_lensed_and_adapted_nominal_projections() {
        // Mutations caught: let a broad vector lens overlap a typed modifier,
        // omit adapted M01 rows from mixed inverse dispatch, or select a form
        // before checking its complete-value recognition predicate.
        let adjective = AdjectivePhrase {
            degree: None,
            head: Adjective::Color(ColorWord::Red),
            complements: Vec::new(),
        };
        let adjective_nominal = build_nominal_adjective(adjective, card_nominal()).unwrap();
        assert_eq!(
            selected_nominal_construction(&adjective_nominal),
            "nominal_adjective"
        );

        let negative = NominalModifier::Noun {
            polarity: Polarity::Negative,
            noun: NounInstance::Singular(Noun::Word(Vocab::Ability)),
        };
        let negative_nominal = build_nominal_negated_modifier(negative, card_nominal()).unwrap();
        assert_eq!(
            selected_nominal_construction(&negative_nominal),
            "nominal_negated_modifier"
        );

        let determined = build_nominal_determiner(Determiner::Indefinite, adjective_nominal)
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
            Determiner::Quantity(three),
            build_nominal_times_clause(
                NounInstance::Plural(Noun::Word(Vocab::Time)),
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
            NounInstance::Singular(Noun::Word(Vocab::Ability)),
            build_nominal_times_clause(
                NounInstance::Plural(Noun::Word(Vocab::Time)),
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
    fn generated_inverse_rejects_invalid_direct_nominal_invariants() {
        // Mutations caught: treat an inverse shape predicate as render-only
        // decoration, or let writable AST fields bypass cardinality, identity,
        // attachment-phase, reduced-passive, and specialized-head gates.
        let plural_cards = NounInstance::Plural(Noun::Word(Vocab::Card));
        assert_nominal_inverse_rejects(&NominalPhrase {
            determiner: Some(Determiner::Each),
            modifiers: Vec::new(),
            head: plural_cards.clone(),
            complements: Vec::new(),
        });

        let one = Quantity::try_exact(crate::syntax::NumberLiteral {
            value: 1,
            numeral: crate::Numeral::Cardinal,
        })
        .unwrap();
        assert_nominal_inverse_rejects(&NominalPhrase {
            determiner: None,
            modifiers: vec![NominalModifier::Quantity(one)],
            head: plural_cards,
            complements: Vec::new(),
        });

        assert_nominal_inverse_rejects(&NominalPhrase {
            determiner: None,
            modifiers: Vec::new(),
            head: NounInstance::Singular(Noun::Word(Vocab::Ability)),
            complements: vec![NominalComplement::KeywordArgument(KeywordArgument::Costed(
                KeywordCost::Symbols(vec![OracleSymbol::new("{2}").unwrap()]),
            ))],
        });

        assert_nominal_inverse_rejects(&NominalPhrase {
            determiner: None,
            modifiers: vec![NominalModifier::CombatStepName {
                participants: NounInstance::Singular(Noun::Word(Vocab::Card)),
            }],
            head: NounInstance::Singular(Noun::Word(Vocab::Step)),
            complements: Vec::new(),
        });

        let mut invalid_phase = parsed_nominal("card in a graveyard");
        invalid_phase
            .complements
            .insert(0, NominalComplement::Adjective(red_adjective()));
        assert_nominal_inverse_rejects(&invalid_phase);

        let mut invalid_devotion = parsed_nominal("your devotion to green");
        invalid_devotion.determiner = None;
        invalid_devotion.head = NounInstance::Singular(Noun::Word(Vocab::Card));
        assert_nominal_inverse_rejects(&invalid_devotion);

        let mut invalid_times = parsed_nominal("the number of times you drew a card");
        let [NominalComplement::Prepositional(of)] = invalid_times.complements.as_mut_slice()
        else {
            panic!("number-of-times fixture keeps its of complement")
        };
        let Phrase::NounPhrase(object) = of.head_mut().object.as_mut() else {
            panic!("the of complement keeps its noun-phrase object")
        };
        let NounPhrase::Nominal(times) = object.as_mut() else {
            panic!("the of object is the times nominal")
        };
        times.head = NounInstance::Plural(Noun::Word(Vocab::Card));
        assert_nominal_inverse_rejects(times);

        let invalid_determined_times = NominalPhrase {
            determiner: Some(Determiner::All),
            modifiers: Vec::new(),
            head: NounInstance::Plural(Noun::Word(Vocab::Card)),
            complements: vec![NominalComplement::EventClause(Box::new(
                parsed_independent("You draw a card."),
            ))],
        };
        assert_nominal_inverse_rejects(&invalid_determined_times);
    }

    #[test]
    fn generated_builders_reject_wrong_specialized_roles_and_punctuation() {
        // Mutations caught: trust typed identity holes without enforcing their
        // provider role, normalize `or` to `and`, or let the binary and Oxford
        // postpositive constructors manufacture the same stored shape.
        assert!(
            build_nominal_combat_step_name(
                NounInstance::Plural(Noun::Word(Vocab::Card)),
                NounInstance::Singular(Noun::Word(Vocab::Step)),
            )
            .is_err()
        );
        assert!(
            build_nominal_keyword_symbol_argument(
                NounInstance::Singular(Noun::Word(Vocab::Ability)),
                Some(OracleSymbol::new("{2}").unwrap()),
                None,
            )
            .is_err()
        );

        let bare = build_predicated_quality_bare(Some(ColorWord::Red), None, None).unwrap();
        assert!(build_predicated_argument_from_single(bare).is_err());
        let explicit = build_predicated_quality_from(Some(ColorWord::Red), None).unwrap();
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
                NounInstance::Plural(Noun::Word(Vocab::Card)),
                Box::new(parsed_independent("You draw a card.")),
            )
            .is_err()
        );
    }
}
