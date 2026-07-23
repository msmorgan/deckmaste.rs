#[allow(
    clippy::wildcard_imports,
    reason = "module uses generated imports and shared grammar aliases"
)]
use super::*;
use crate::syntax::AbilityObject;
use crate::syntax::AttachmentPosition;
use crate::syntax::ClauseAttachment;
use crate::syntax::ClauseAttachmentKind;
use crate::syntax::ClauseCoordination;
use crate::syntax::ComplexClause;
use crate::syntax::CoordinatedClauseMember;
use crate::syntax::CoordinatedIndependentClause;
use crate::syntax::CoordinatedPredicateObject;
use crate::syntax::DependentAttachment;
use crate::syntax::DependentClause;
use crate::syntax::EllipticalClause;
use crate::syntax::IndependentClause;
use crate::syntax::InfinitiveMarker;
use crate::syntax::Modal;
use crate::syntax::ObjectGapPredicate;
use crate::syntax::PassivePredicate;
use crate::syntax::Predicate;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PredicateComplement;
use crate::syntax::PredicateElement;
use crate::syntax::PredicateHead;
use crate::syntax::PredicateObject;
use crate::syntax::PredicateObjectCoordination;
use crate::syntax::ProPredicate;
use crate::syntax::RelativeBody;
use crate::syntax::RelativeMarker;
use crate::syntax::SentenceBody;
use crate::syntax::SentenceEnding;
use crate::syntax::SubordinateBody;

#[allow(
    clippy::too_many_lines,
    reason = "grammar table builder is intentionally verbose"
)]
pub(super) fn add_rules(builder: &mut RuleBuilder) {
    use EnglishLexicalSlot as L;
    use Expected::Lexical as l;
    use Expected::Nonterminal as n;
    use Nonterminal as N;

    builder.add(
        RuleTag::FrequencyPhrase,
        N::FrequencyPhrase,
        [l(L::Frequency)],
    );
    builder.add(
        RuleTag::FrequencyPhraseAdverb,
        N::FrequencyPhrase,
        [l(L::Adverb), l(L::Frequency)],
    );

    for slot in crate::word::VERB_SLOTS {
        builder.add(RuleTag::Verb, N::Verb, [l(L::Verb(slot))]);
    }
    builder.add(RuleTag::VerbPhraseBase, N::VerbPhrase, [n(N::Verb)]);
    builder.add(
        RuleTag::VerbPhraseAuxiliary,
        N::VerbPhrase,
        [l(L::Auxiliary), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::VerbPhraseAuxiliaryProform,
        N::VerbPhrase,
        [l(L::Auxiliary)],
    );
    builder.add(
        RuleTag::VerbPhraseDirectObject,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::NounPhrase)],
    );
    builder.add(
        RuleTag::VerbPhraseIndirectObject,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::NounPhrase)],
    );
    builder.add_with_cost(
        RuleTag::VerbPhraseAdjective,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::AdjectivePhrase)],
        ParseCost {
            precedence: 2,
            ..ParseCost::default()
        },
    );
    builder.add_with_cost(
        RuleTag::VerbPhrasePrepositional,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::PrepositionalPhrase)],
        ParseCost {
            precedence: 1,
            ..ParseCost::default()
        },
    );
    builder.add(
        RuleTag::VerbPhraseInfinitive,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::InfinitiveClause)],
    );
    builder.add(
        RuleTag::VerbPhraseAdverb,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::Adverb)],
    );
    for particle in [VerbParticle::In, VerbParticle::Out] {
        builder.add(
            RuleTag::VerbPhraseParticle,
            N::VerbPhrase,
            [n(N::VerbPhrase), l(L::VerbParticle(particle))],
        );
    }
    builder.add(
        RuleTag::VerbPhraseFrequency,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::FrequencyPhrase)],
    );
    builder.add(
        RuleTag::VerbPhraseAbility,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::AbilityItem)],
    );
    builder.add(
        RuleTag::VerbPhraseOracleSymbol,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::OracleSymbol)],
    );
    builder.add(
        RuleTag::VerbPhraseSymbolSequence,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::SymbolSequence)],
    );
    builder.add(
        RuleTag::VerbPhraseOracleSymbolCoordination,
        N::VerbPhrase,
        [
            n(N::VerbPhrase),
            l(L::OracleSymbol),
            l(L::Conjunction),
            l(L::OracleSymbol),
        ],
    );
    builder.add(
        RuleTag::VerbPhrasePowerToughness,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::PowerToughness)],
    );
    builder.add(
        RuleTag::VerbPhraseQuantity,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::Quantity)],
    );

    builder.add(
        RuleTag::VerbPhraseBase,
        N::ObjectGapVerbPhrase,
        [n(N::Verb)],
    );
    builder.add(
        RuleTag::VerbPhraseAuxiliary,
        N::ObjectGapVerbPhrase,
        [l(L::Auxiliary), n(N::ObjectGapVerbPhrase)],
    );
    builder.add(
        RuleTag::VerbPhraseIndirectObject,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::NounPhrase)],
    );
    builder.add_with_cost(
        RuleTag::VerbPhraseAdjective,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::AdjectivePhrase)],
        ParseCost {
            precedence: 2,
            ..ParseCost::default()
        },
    );
    builder.add_with_cost(
        RuleTag::VerbPhrasePrepositional,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::PrepositionalPhrase)],
        ParseCost {
            precedence: 1,
            ..ParseCost::default()
        },
    );
    builder.add(
        RuleTag::VerbPhraseInfinitive,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::InfinitiveClause)],
    );
    builder.add(
        RuleTag::VerbPhraseAdverb,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), l(L::Adverb)],
    );
    for particle in [VerbParticle::In, VerbParticle::Out] {
        builder.add(
            RuleTag::VerbPhraseParticle,
            N::ObjectGapVerbPhrase,
            [n(N::ObjectGapVerbPhrase), l(L::VerbParticle(particle))],
        );
    }
    builder.add(
        RuleTag::VerbPhraseFrequency,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::FrequencyPhrase)],
    );

    builder.add(
        RuleTag::InfinitiveTo,
        N::InfinitiveClause,
        [l(L::To), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::InfinitiveNotTo,
        N::InfinitiveClause,
        [l(L::Not), l(L::To), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::GerundClauseBase,
        N::GerundClause,
        [n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::GerundClauseSubordinateAfter,
        N::GerundClause,
        [n(N::GerundClause), l(L::RatherThan), n(N::GerundClause)],
    );
    builder.add(
        RuleTag::SimpleClauseSubject,
        N::SimpleClause,
        [n(N::NounPhrase), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::SimpleClauseContractedSubject,
        N::SimpleClause,
        [l(L::SubjectAuxiliary), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::SimpleClauseSubjectless,
        N::SimpleClause,
        [n(N::VerbPhrase)],
    );
    builder.add(RuleTag::ClauseSimple, N::Clause, [n(N::SimpleClause)]);
    builder.add(
        RuleTag::ClauseExistential,
        N::Clause,
        [l(L::Existential), n(N::NounPhrase)],
    );
    builder.add_with_cost(
        RuleTag::CopularRemainderNoun,
        N::CopularRemainder,
        [n(N::NounPhrase)],
        ParseCost {
            precedence: 1,
            ..ParseCost::default()
        },
    );
    builder.add(
        RuleTag::CopularRemainderAdjective,
        N::CopularRemainder,
        [n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::CopularRemainderPrepositional,
        N::CopularRemainder,
        [n(N::PrepositionalPhrase)],
    );
    builder.add(
        RuleTag::CopularRemainderAdverb,
        N::CopularRemainder,
        [l(L::Adverb), n(N::CopularRemainder)],
    );
    builder.add(
        RuleTag::ClauseCopular,
        N::Clause,
        [n(N::NounPhrase), l(L::Copula), n(N::CopularRemainder)],
    );
    builder.add(
        RuleTag::ClauseContractedCopular,
        N::Clause,
        [l(L::SubjectAuxiliary), n(N::CopularRemainder)],
    );
    builder.add(
        RuleTag::ClauseElliptical,
        N::Clause,
        [n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::ClauseCoordination,
        N::Clause,
        [n(N::Clause), l(L::Conjunction), n(N::SimpleClause)],
    );
    builder.add(
        RuleTag::ClauseCoordinationComma,
        N::Clause,
        [
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            l(L::Conjunction),
            n(N::SimpleClause),
        ],
    );
    builder.add(
        RuleTag::ClauseCoordinationAsyndetic,
        N::Clause,
        [
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::SimpleClause),
        ],
    );
    builder.add(
        RuleTag::ClauseSubordinateBefore,
        N::Clause,
        [
            l(L::Subordinator),
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ClauseAdverbBefore,
        N::Clause,
        [l(L::Adverb), n(N::Clause)],
    );
    builder.add(
        RuleTag::ClausePrepositionalBefore,
        N::Clause,
        [
            n(N::PrepositionalPhrase),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ClauseSubordinateAfterElliptical,
        N::Clause,
        [n(N::Clause), l(L::Subordinator), n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::ClauseSubordinateAfter,
        N::Clause,
        [n(N::Clause), l(L::Subordinator), n(N::Clause)],
    );
    builder.add(
        RuleTag::ClauseSubordinateAfterComma,
        N::Clause,
        [
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            l(L::Subordinator),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ClauseSubordinateAfterInfinitive,
        N::Clause,
        [n(N::Clause), l(L::RatherThan), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::RelativeObject,
        N::RelativeClause,
        [n(N::NounPhrase), n(N::ObjectGapVerbPhrase)],
    );
    builder.add(
        RuleTag::RelativeObjectContractedSubject,
        N::RelativeClause,
        [l(L::SubjectAuxiliary), n(N::ObjectGapVerbPhrase)],
    );
    builder.add(
        RuleTag::RelativeSubjectContractedAuxiliary,
        N::RelativeClause,
        [l(L::SubjectAuxiliary), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::RelativeSubject,
        N::RelativeClause,
        [l(L::RelativeMarker), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::RelativeContractedCopularNoun,
        N::RelativeClause,
        [l(L::SubjectAuxiliary), n(N::NounPhrase)],
    );
    builder.add(
        RuleTag::RelativeContractedCopularAdjective,
        N::RelativeClause,
        [l(L::SubjectAuxiliary), n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::RelativeContractedCopularPrepositional,
        N::RelativeClause,
        [l(L::SubjectAuxiliary), n(N::PrepositionalPhrase)],
    );

    builder.add(
        RuleTag::SentencePeriod,
        N::Sentence,
        [n(N::Clause), l(L::Punctuation(Punctuation::Period))],
    );
    builder.add(RuleTag::SentenceNone, N::Sentence, [n(N::Clause)]);
}

pub(super) fn reduce_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseAuxiliaryProform
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseOracleSymbolCoordination
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo => reduce_predicate(tag, children),
        RuleTag::GerundClauseBase => {
            let Features::VerbPhrase {
                form: PredicateForm::PresentParticiple,
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            Some(Features::GerundClause)
        }
        RuleTag::GerundClauseSubordinateAfter => {
            if !matches!(children.first()?.features, Features::GerundClause)
                || !matches!(children.get(2)?.features, Features::GerundClause)
            {
                return None;
            }
            Some(Features::GerundClause)
        }
        RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseExistential
        | RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderAdverb
        | RuleTag::ClauseCopular
        | RuleTag::ClauseContractedCopular
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional => reduce_simple_clause(tag, children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseAdverbBefore
        | RuleTag::ClausePrepositionalBefore
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterComma
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::SentencePeriod
        | RuleTag::SentenceNone => reduce_composed_clause(tag, children),
        _ => None,
    }
}

pub(super) fn accepts_predicate_prefix(
    tag: RuleTag,
    completed_children: usize,
    features: &Features,
) -> bool {
    if completed_children != 1 {
        return true;
    }
    let predicate_rule = matches!(
        tag,
        RuleTag::VerbPhraseDirectObject
            | RuleTag::VerbPhraseIndirectObject
            | RuleTag::VerbPhraseAdjective
            | RuleTag::VerbPhrasePrepositional
            | RuleTag::VerbPhraseInfinitive
            | RuleTag::VerbPhraseParticle
            | RuleTag::VerbPhraseAbility
            | RuleTag::VerbPhraseOracleSymbol
            | RuleTag::VerbPhraseSymbolSequence
            | RuleTag::VerbPhraseOracleSymbolCoordination
            | RuleTag::VerbPhrasePowerToughness
            | RuleTag::VerbPhraseQuantity
    );
    if !predicate_rule {
        return true;
    }
    let Features::VerbPhrase {
        object,
        indirect_object,
        phase,
        frame,
        ..
    } = features
    else {
        return false;
    };
    match tag {
        RuleTag::VerbPhraseDirectObject => {
            let nominal_adjunct = frame.licenses_bare_nominal_adjunct(BareNominalAdjunct::Temporal)
                || frame.licenses_bare_nominal_adjunct(BareNominalAdjunct::Manner);
            if *phase == PredicateAttachmentPhase::Object && *object == PredicateObjectState::None {
                frame.direct_object().accepts() || nominal_adjunct
            } else {
                nominal_adjunct
            }
        }
        RuleTag::VerbPhraseIndirectObject => {
            *phase == PredicateAttachmentPhase::Object
                && *object == PredicateObjectState::None
                && !*indirect_object
                && frame.indirect_object().accepts()
        }
        RuleTag::VerbPhraseAdjective => {
            frame.licenses_complement(PredicateComplementKind::Adjective)
        }
        RuleTag::VerbPhraseInfinitive => {
            frame.licenses_complement(PredicateComplementKind::Infinitive)
        }
        RuleTag::VerbPhraseAbility => {
            *object == PredicateObjectState::None
                && frame.licenses_complement(PredicateComplementKind::Ability)
        }
        RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseOracleSymbolCoordination => {
            *object == PredicateObjectState::None
                && frame.licenses_complement(PredicateComplementKind::Scalar)
        }
        RuleTag::VerbPhrasePowerToughness => {
            *object == PredicateObjectState::None
                && frame.licenses_complement(PredicateComplementKind::Statistic)
        }
        RuleTag::VerbPhraseQuantity => {
            matches!(
                object,
                PredicateObjectState::None | PredicateObjectState::Ability
            ) && frame.licenses_complement(PredicateComplementKind::Scalar)
        }
        RuleTag::VerbPhraseParticle => !frame.particles.is_empty(),
        _ => true,
    }
}

pub(super) fn reduction_cost(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> ParseCost {
    let active_temporal_attachment = tag == RuleTag::VerbPhraseDirectObject
        && matches!(
            children.first().map(|child| child.features),
            Some(Features::VerbPhrase { passive: false, .. })
        )
        && matches!(
            children.get(1).map(|child| child.features),
            Some(Features::NounPhrase {
                adjunct: Some(BareNominalAdjunct::Temporal),
                ..
            })
        );
    let precedence =
        u32::from(active_temporal_attachment || tag == RuleTag::ClauseSubordinateAfter);
    ParseCost {
        precedence,
        ..ParseCost::default()
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "grammar reduction rule set is intentionally long"
)]
fn reduce_predicate(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::Verb => Some(propagate(children.first()?)),
        RuleTag::VerbPhraseBase => {
            let Features::Verb { slot, frame } = children.first()?.features else {
                return None;
            };
            let form = predicate_form(*slot);
            Some(Features::VerbPhrase {
                form,
                passive: false,
                object: PredicateObjectState::None,
                indirect_object: false,
                selected_preposition: false,
                phase: PredicateAttachmentPhase::Object,
                frame: *frame,
                bare: true,
            })
        }
        RuleTag::VerbPhraseAuxiliaryProform => {
            let Features::Auxiliary(auxiliary) = children.first()?.features else {
                return None;
            };
            let form = auxiliary_form(*auxiliary, PredicateForm::Infinitive)?;
            Some(Features::VerbPhrase {
                form,
                passive: false,
                object: PredicateObjectState::None,
                indirect_object: false,
                selected_preposition: false,
                phase: PredicateAttachmentPhase::Object,
                frame: crate::word::PROFORM_PREDICATE_FRAMES[0],
                bare: false,
            })
        }
        RuleTag::VerbPhraseAuxiliary => {
            let Features::Auxiliary(auxiliary) = children.first()?.features else {
                return None;
            };
            let Features::VerbPhrase {
                form: child_form,
                passive: child_passive,
                object,
                indirect_object,
                selected_preposition,
                phase,
                frame,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let form = auxiliary_form(*auxiliary, *child_form)?;
            let passive = *child_passive
                || (auxiliary.auxiliary == Auxiliary::Be
                    && *child_form == PredicateForm::PastParticiple);
            if passive && object.has_direct_object() {
                return None;
            }
            Some(Features::VerbPhrase {
                form,
                passive,
                object: *object,
                indirect_object: *indirect_object,
                selected_preposition: *selected_preposition,
                phase: *phase,
                frame: *frame,
                bare: false,
            })
        }
        RuleTag::VerbPhraseDirectObject | RuleTag::VerbPhraseIndirectObject => {
            let Features::NounPhrase {
                pronoun_case,
                adjunct,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if *pronoun_case == Some(PronounCase::Subject) {
                return None;
            }
            let attachment = if tag == RuleTag::VerbPhraseIndirectObject {
                PredicateAttachment::IndirectObject
            } else {
                let Features::VerbPhrase {
                    form,
                    passive,
                    object,
                    phase,
                    frame,
                    ..
                } = children.first()?.features
                else {
                    return None;
                };
                match adjunct.filter(|adjunct| {
                    frame.licenses_bare_nominal_adjunct(*adjunct)
                        && (*form == PredicateForm::PastParticiple
                            || *passive
                            || *phase != PredicateAttachmentPhase::Object
                            || object.has_direct_object()
                            || !frame.direct_object().accepts())
                }) {
                    Some(adjunct) => PredicateAttachment::NominalAdjunct(adjunct),
                    None => PredicateAttachment::DirectObject,
                }
            };
            extend_predicate(children.first()?, attachment)
        }
        RuleTag::VerbPhraseOracleSymbolCoordination => {
            let Features::Conjunction(
                crate::syntax::PredicateConjunction::And | crate::syntax::PredicateConjunction::Or,
            ) = children.get(2)?.features
            else {
                return None;
            };
            extend_predicate(children.first()?, PredicateAttachment::ScalarComplement)
        }
        RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity => {
            let attachment = match tag {
                RuleTag::VerbPhraseAdjective => PredicateAttachment::AdjectiveComplement,
                RuleTag::VerbPhrasePrepositional => {
                    let Features::PrepositionalPhrase { preposition, .. } =
                        children.get(1)?.features
                    else {
                        return None;
                    };
                    PredicateAttachment::Prepositional(*preposition)
                }
                RuleTag::VerbPhraseInfinitive => PredicateAttachment::InfinitiveComplement,
                RuleTag::VerbPhraseAdverb | RuleTag::VerbPhraseFrequency => {
                    PredicateAttachment::Adjunct
                }
                RuleTag::VerbPhraseParticle => {
                    let Features::VerbParticle(particle) = children.get(1)?.features else {
                        return None;
                    };
                    PredicateAttachment::Particle(*particle)
                }
                RuleTag::VerbPhraseAbility => PredicateAttachment::AbilityComplement,
                RuleTag::VerbPhraseOracleSymbol | RuleTag::VerbPhraseSymbolSequence => {
                    PredicateAttachment::ScalarComplement
                }
                RuleTag::VerbPhrasePowerToughness => PredicateAttachment::StatisticComplement,
                RuleTag::VerbPhraseQuantity => PredicateAttachment::ScalarOrAbilityArgument,
                _ => return None,
            };
            extend_predicate(children.first()?, attachment)
        }
        RuleTag::InfinitiveTo | RuleTag::InfinitiveNotTo => {
            let predicate_index = if tag == RuleTag::InfinitiveNotTo { 2 } else { 1 };
            let Features::VerbPhrase {
                form: PredicateForm::Infinitive,
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                ..
            } = children.get(predicate_index)?.features
            else {
                return None;
            };
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            Some(Features::InfinitiveClause)
        }
        _ => None,
    }
}

fn extend_predicate(
    predicate: &Child<'_, EnglishGrammar<'_, '_>>,
    attachment: PredicateAttachment,
) -> Option<Reduced> {
    let Features::VerbPhrase {
        form,
        passive,
        object,
        indirect_object,
        selected_preposition,
        phase,
        frame,
        ..
    } = predicate.features
    else {
        return None;
    };
    let prepositional_role = match attachment {
        PredicateAttachment::Prepositional(preposition) => {
            Some(frame.prepositional_role(preposition)?)
        }
        _ => None,
    };
    let licensed = match attachment {
        PredicateAttachment::Adjunct | PredicateAttachment::Prepositional(_) => true,
        PredicateAttachment::DirectObject => frame.direct_object().accepts(),
        PredicateAttachment::IndirectObject => frame.indirect_object().accepts(),
        PredicateAttachment::NominalAdjunct(adjunct) => {
            frame.licenses_bare_nominal_adjunct(adjunct)
        }
        PredicateAttachment::AdjectiveComplement => {
            frame.licenses_complement(PredicateComplementKind::Adjective)
        }
        PredicateAttachment::InfinitiveComplement => {
            frame.licenses_complement(PredicateComplementKind::Infinitive)
        }
        PredicateAttachment::Particle(particle) => frame.licenses_particle(particle),
        PredicateAttachment::AbilityComplement => {
            frame.licenses_complement(PredicateComplementKind::Ability)
        }
        PredicateAttachment::ScalarComplement | PredicateAttachment::ScalarOrAbilityArgument => {
            frame.licenses_complement(PredicateComplementKind::Scalar)
        }
        PredicateAttachment::StatisticComplement => {
            frame.licenses_complement(PredicateComplementKind::Statistic)
        }
    };
    if !licensed {
        return None;
    }
    if *passive
        && !matches!(
            attachment,
            PredicateAttachment::Adjunct
                | PredicateAttachment::NominalAdjunct(_)
                | PredicateAttachment::AdjectiveComplement
                | PredicateAttachment::Prepositional(_)
                | PredicateAttachment::InfinitiveComplement
                | PredicateAttachment::Particle(_)
        )
    {
        return None;
    }
    let next_phase = match attachment {
        PredicateAttachment::DirectObject
        | PredicateAttachment::IndirectObject
        | PredicateAttachment::AbilityComplement
        | PredicateAttachment::ScalarComplement
        | PredicateAttachment::StatisticComplement
        | PredicateAttachment::ScalarOrAbilityArgument => {
            if *phase != PredicateAttachmentPhase::Object {
                return None;
            }
            PredicateAttachmentPhase::Object
        }
        PredicateAttachment::Prepositional(_) => PredicateAttachmentPhase::PrepositionalTail,
        PredicateAttachment::Adjunct => {
            if *object == PredicateObjectState::None {
                PredicateAttachmentPhase::Object
            } else {
                PredicateAttachmentPhase::Tail
            }
        }
        PredicateAttachment::NominalAdjunct(_)
        | PredicateAttachment::AdjectiveComplement
        | PredicateAttachment::InfinitiveComplement
        | PredicateAttachment::Particle(_) => PredicateAttachmentPhase::Tail,
    };
    let object = match (attachment, *object) {
        (
            PredicateAttachment::Adjunct
            | PredicateAttachment::IndirectObject
            | PredicateAttachment::NominalAdjunct(_)
            | PredicateAttachment::AdjectiveComplement
            | PredicateAttachment::Prepositional(_)
            | PredicateAttachment::InfinitiveComplement
            | PredicateAttachment::Particle(_),
            object,
        ) => object,
        (
            PredicateAttachment::DirectObject
            | PredicateAttachment::ScalarComplement
            | PredicateAttachment::StatisticComplement
            | PredicateAttachment::ScalarOrAbilityArgument,
            PredicateObjectState::None,
        ) => PredicateObjectState::Direct,
        (PredicateAttachment::AbilityComplement, PredicateObjectState::None) => {
            PredicateObjectState::Ability
        }
        (PredicateAttachment::ScalarOrAbilityArgument, PredicateObjectState::Ability) => {
            PredicateObjectState::AbilityWithArgument
        }
        _ => return None,
    };
    let indirect_object = match attachment {
        PredicateAttachment::IndirectObject if !*indirect_object && !object.has_direct_object() => {
            true
        }
        PredicateAttachment::IndirectObject => return None,
        _ => *indirect_object,
    };
    let selected_preposition = match prepositional_role {
        Some(PrepositionalRole::SelectedComplement) if *selected_preposition => return None,
        Some(PrepositionalRole::SelectedComplement) => true,
        Some(PrepositionalRole::Adjunct) | None => *selected_preposition,
    };
    Some(Features::VerbPhrase {
        form: *form,
        passive: *passive,
        object,
        indirect_object,
        selected_preposition,
        phase: next_phase,
        frame: *frame,
        bare: false,
    })
}

#[derive(Debug, Clone, Copy)]
enum PredicateAttachment {
    Adjunct,
    DirectObject,
    IndirectObject,
    NominalAdjunct(BareNominalAdjunct),
    AdjectiveComplement,
    Prepositional(Preposition),
    InfinitiveComplement,
    Particle(VerbParticle),
    AbilityComplement,
    ScalarComplement,
    StatisticComplement,
    ScalarOrAbilityArgument,
}

fn predicate_arguments_complete(
    frame: PredicateFrame,
    passive: bool,
    object: PredicateObjectState,
    indirect_object: bool,
    selected_preposition: bool,
) -> bool {
    frame
        .direct_object()
        .is_satisfied_by(passive || object.has_direct_object())
        && frame.indirect_object().is_satisfied_by(indirect_object)
        && frame
            .selected_preposition()
            .is_satisfied_by(selected_preposition)
}

fn predicate_object_gap_complete(
    frame: PredicateFrame,
    indirect_object: bool,
    selected_preposition: bool,
) -> bool {
    frame.direct_object().accepts()
        && frame.indirect_object().is_satisfied_by(indirect_object)
        && frame
            .selected_preposition()
            .is_satisfied_by(selected_preposition)
}

#[allow(
    clippy::too_many_lines,
    reason = "simple-clause reduction logic is intentionally long"
)]
fn reduce_simple_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::SimpleClauseSubject => {
            let Features::NounPhrase {
                agreement: Some(subject_agreement),
                pronoun_case,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if *pronoun_case == Some(PronounCase::Object) {
                return None;
            }
            let Features::VerbPhrase {
                form: PredicateForm::Finite(predicate_agreement),
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if predicate_agreement.is_some_and(|agreement| agreement != *subject_agreement) {
                return None;
            }
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            Some(simple_clause_reduction(
                Some(*subject_agreement),
                true,
                true,
                object.has_direct_object(),
            ))
        }
        RuleTag::SimpleClauseContractedSubject => {
            let Features::SubjectAuxiliary {
                agreement: subject_agreement,
                auxiliary,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::VerbPhrase {
                form: child_form,
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let PredicateForm::Finite(Some(predicate_agreement)) =
                auxiliary_form(*auxiliary, *child_form)?
            else {
                return None;
            };
            if predicate_agreement != *subject_agreement {
                return None;
            }
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            Some(simple_clause_reduction(
                Some(*subject_agreement),
                true,
                true,
                object.has_direct_object(),
            ))
        }
        RuleTag::SimpleClauseSubjectless => {
            let Features::VerbPhrase {
                form,
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            match form {
                PredicateForm::Imperative => Some(simple_clause_reduction(
                    None,
                    false,
                    true,
                    object.has_direct_object(),
                )),
                PredicateForm::Finite(agreement) => Some(simple_clause_reduction(
                    *agreement,
                    false,
                    false,
                    object.has_direct_object(),
                )),
                PredicateForm::Infinitive => Some(simple_clause_reduction(
                    None,
                    false,
                    false,
                    object.has_direct_object(),
                )),
                PredicateForm::PresentParticiple | PredicateForm::PastParticiple => None,
            }
        }
        RuleTag::ClauseSimple => {
            let Features::SimpleClause {
                agreement,
                has_subject,
                standalone,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Clause {
                agreement: *agreement,
                standalone: *standalone,
                finite: *has_subject,
            })
        }
        RuleTag::ClauseElliptical => Some(Features::Clause {
            agreement: None,
            standalone: false,
            finite: false,
        }),
        RuleTag::ClauseExistential => {
            let Features::Existential {
                number: expected_number,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::NounPhrase {
                agreement: Some(agreement),
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if agreement.number != *expected_number {
                return None;
            }
            Some(Features::Clause {
                agreement: None,
                standalone: true,
                finite: true,
            })
        }
        RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderAdverb => Some(Features::None),
        tag @ (RuleTag::ClauseCopular | RuleTag::ClauseContractedCopular) => {
            reduce_copular_clause(tag, children)
        }
        RuleTag::RelativeObject => {
            let Features::NounPhrase {
                agreement: Some(subject_agreement),
                pronoun_case,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if *pronoun_case == Some(PronounCase::Object) {
                return None;
            }
            let Features::VerbPhrase {
                form: PredicateForm::Finite(predicate_agreement),
                object: PredicateObjectState::None,
                indirect_object,
                selected_preposition,
                frame,
                bare,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if (*bare && frame.is_proform())
                || !predicate_object_gap_complete(*frame, *indirect_object, *selected_preposition)
                || predicate_agreement.is_some_and(|agreement| agreement != *subject_agreement)
            {
                return None;
            }
            Some(Features::RelativeClause {
                gap: RelativeGap::Object,
                antecedent_agreement: None,
            })
        }
        RuleTag::RelativeObjectContractedSubject => {
            let Features::SubjectAuxiliary {
                agreement: subject_agreement,
                auxiliary,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::VerbPhrase {
                form: child_form,
                object: PredicateObjectState::None,
                indirect_object,
                selected_preposition,
                frame,
                bare,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let PredicateForm::Finite(Some(predicate_agreement)) =
                auxiliary_form(*auxiliary, *child_form)?
            else {
                return None;
            };
            if (*bare && frame.is_proform())
                || !predicate_object_gap_complete(*frame, *indirect_object, *selected_preposition)
                || predicate_agreement != *subject_agreement
            {
                return None;
            }
            Some(Features::RelativeClause {
                gap: RelativeGap::Object,
                antecedent_agreement: None,
            })
        }
        RuleTag::RelativeSubjectContractedAuxiliary => {
            let Features::SubjectAuxiliary {
                subject: ContractedSubjectKey::Demonstrative(Demonstrative::That),
                agreement,
                auxiliary,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::VerbPhrase {
                form: child_form, ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let PredicateForm::Finite(Some(predicate_agreement)) =
                auxiliary_form(*auxiliary, *child_form)?
            else {
                return None;
            };
            if predicate_agreement != *agreement {
                return None;
            }
            Some(Features::RelativeClause {
                gap: RelativeGap::Subject,
                antecedent_agreement: Some(predicate_agreement),
            })
        }
        RuleTag::RelativeSubject => {
            let Features::VerbPhrase {
                form: PredicateForm::Finite(antecedent_agreement),
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            Some(Features::RelativeClause {
                gap: RelativeGap::Subject,
                antecedent_agreement: *antecedent_agreement,
            })
        }
        RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional => {
            let Features::SubjectAuxiliary {
                subject: ContractedSubjectKey::Demonstrative(Demonstrative::That),
                agreement,
                auxiliary,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if auxiliary.auxiliary != Auxiliary::Be {
                return None;
            }
            Some(Features::RelativeClause {
                gap: RelativeGap::Subject,
                antecedent_agreement: Some(*agreement),
            })
        }
        _ => None,
    }
}

fn reduce_copular_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let contracted = tag == RuleTag::ClauseContractedCopular;
    let agreement = if contracted {
        let Features::SubjectAuxiliary {
            agreement,
            auxiliary,
            ..
        } = children.first()?.features
        else {
            return None;
        };
        if auxiliary.auxiliary != Auxiliary::Be {
            return None;
        }
        *agreement
    } else {
        let Features::NounPhrase {
            agreement: Some(subject_agreement),
            pronoun_case,
            ..
        } = children.first()?.features
        else {
            return None;
        };
        if *pronoun_case == Some(PronounCase::Object) {
            return None;
        }
        let Features::Copula(copula_agreement) = children.get(1)?.features else {
            return None;
        };
        if *subject_agreement != *copula_agreement {
            return None;
        }
        *subject_agreement
    };
    Some(Features::Clause {
        agreement: Some(agreement),
        standalone: true,
        finite: true,
    })
}

fn simple_clause_reduction(
    agreement: Option<Agreement>,
    has_subject: bool,
    standalone: bool,
    has_direct_object: bool,
) -> Reduced {
    Features::SimpleClause {
        agreement,
        has_subject,
        standalone,
        has_direct_object,
    }
}

fn reduce_composed_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic => {
            let Features::Clause {
                agreement: first_agreement,
                standalone: true,
                finite,
            } = children.first()?.features
            else {
                return None;
            };
            let last = children.last()?;
            let Features::SimpleClause {
                agreement: next_agreement,
                has_subject,
                standalone,
                ..
            } = last.features
            else {
                return None;
            };
            if tag == RuleTag::ClauseCoordinationAsyndetic
                && (first_agreement.is_some()
                    || *finite
                    || *has_subject
                    || next_agreement.is_some()
                    || !*standalone)
            {
                return None;
            }
            if !coordination_agrees(*first_agreement, *next_agreement, *has_subject, *standalone) {
                return None;
            }
            Some(Features::Clause {
                agreement: *first_agreement,
                standalone: true,
                finite: *finite,
            })
        }
        RuleTag::ClauseSubordinateBefore => {
            conditional_reduction(children.get(1)?, children.get(3)?)
        }
        RuleTag::ClauseAdverbBefore => fronted_attachment_reduction(children.get(1)?),
        RuleTag::ClausePrepositionalBefore => {
            let Features::PrepositionalPhrase { .. } = children.first()?.features else {
                return None;
            };
            fronted_attachment_reduction(children.get(2)?)
        }
        RuleTag::ClauseSubordinateAfterElliptical => {
            let consequence = children.first()?;
            let Features::Clause {
                agreement,
                standalone: true,
                finite,
            } = consequence.features
            else {
                return None;
            };
            Some(Features::Clause {
                agreement: *agreement,
                standalone: true,
                finite: *finite,
            })
        }
        RuleTag::ClauseSubordinateAfter => {
            conditional_reduction(children.get(2)?, children.first()?)
        }
        RuleTag::ClauseSubordinateAfterComma => {
            conditional_reduction(children.get(3)?, children.first()?)
        }
        RuleTag::ClauseSubordinateAfterInfinitive => {
            let Features::Clause {
                agreement,
                standalone: true,
                finite,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::VerbPhrase {
                form: PredicateForm::Infinitive,
                ..
            } = children.get(2)?.features
            else {
                return None;
            };
            Some(Features::Clause {
                agreement: *agreement,
                standalone: true,
                finite: *finite,
            })
        }
        RuleTag::SentencePeriod | RuleTag::SentenceNone => {
            let Features::Clause {
                standalone: true, ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Sentence)
        }
        _ => None,
    }
}

fn fronted_attachment_reduction(matrix: &Child<'_, EnglishGrammar<'_, '_>>) -> Option<Reduced> {
    let Features::Clause {
        agreement,
        standalone: true,
        finite,
    } = matrix.features
    else {
        return None;
    };
    Some(Features::Clause {
        agreement: *agreement,
        standalone: true,
        finite: *finite,
    })
}

fn conditional_reduction(
    condition: &Child<'_, EnglishGrammar<'_, '_>>,
    consequence: &Child<'_, EnglishGrammar<'_, '_>>,
) -> Option<Reduced> {
    let Features::Clause {
        standalone: true,
        finite: true,
        ..
    } = condition.features
    else {
        return None;
    };
    let Features::Clause {
        agreement,
        standalone: true,
        finite,
    } = consequence.features
    else {
        return None;
    };
    Some(Features::Clause {
        agreement: *agreement,
        standalone: true,
        finite: *finite,
    })
}

fn coordination_agrees(
    first: Option<Agreement>,
    next: Option<Agreement>,
    next_has_subject: bool,
    next_standalone: bool,
) -> bool {
    next_has_subject
        || matches!((first, next, next_standalone),
            (Some(left), Some(right), false) if left == right
        )
        || matches!((first, next, next_standalone), (None, None, true))
        || matches!((first, next, next_standalone), (Some(_), None, false))
}

const fn predicate_form(slot: VerbSlot) -> PredicateForm {
    match slot {
        VerbSlot::Imperative => PredicateForm::Imperative,
        VerbSlot::Infinitive => PredicateForm::Infinitive,
        VerbSlot::Present { person, number } | VerbSlot::Past { person, number } => {
            PredicateForm::Finite(Some(Agreement { person, number }))
        }
        VerbSlot::PresentParticiple => PredicateForm::PresentParticiple,
        VerbSlot::PastParticiple => PredicateForm::PastParticiple,
    }
}

fn auxiliary_form(auxiliary: AuxiliaryInstance, child: PredicateForm) -> Option<PredicateForm> {
    let accepts_child = match auxiliary.auxiliary {
        Auxiliary::Be => matches!(
            child,
            PredicateForm::PresentParticiple | PredicateForm::PastParticiple
        ),
        Auxiliary::Have => child == PredicateForm::PastParticiple,
        Auxiliary::Do
        | Auxiliary::Can
        | Auxiliary::Could
        | Auxiliary::May
        | Auxiliary::Might
        | Auxiliary::Must
        | Auxiliary::Shall
        | Auxiliary::Should
        | Auxiliary::Will
        | Auxiliary::Would => child == PredicateForm::Infinitive,
    };
    if !accepts_child {
        return None;
    }

    match auxiliary.inflection {
        AuxiliaryInflection::Base => match auxiliary.auxiliary {
            Auxiliary::Be | Auxiliary::Have | Auxiliary::Do => Some(PredicateForm::Infinitive),
            Auxiliary::Can
            | Auxiliary::Could
            | Auxiliary::May
            | Auxiliary::Might
            | Auxiliary::Must
            | Auxiliary::Shall
            | Auxiliary::Should
            | Auxiliary::Will
            | Auxiliary::Would => Some(PredicateForm::Finite(None)),
        },
        AuxiliaryInflection::Present { person, number }
        | AuxiliaryInflection::Past { person, number } => {
            Some(PredicateForm::Finite(Some(Agreement { person, number })))
        }
        AuxiliaryInflection::PresentParticiple => Some(PredicateForm::PresentParticiple),
        AuxiliaryInflection::PastParticiple => Some(PredicateForm::PastParticiple),
    }
}

pub(super) fn lower_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseAuxiliaryProform
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseOracleSymbolCoordination
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo => lower_predicate(tag, children),
        RuleTag::GerundClauseBase => {
            let Lowered::VerbPhrase(predicate) = take(children, 0)? else {
                return None;
            };
            let FinishedPredicate { modal, predicate } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            Some(Lowered::GerundClause(crate::syntax::GerundClause {
                predicate: Box::new(predicate),
                attachments: Vec::new(),
            }))
        }
        RuleTag::GerundClauseSubordinateAfter => {
            let Lowered::GerundClause(mut matrix) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(crate::syntax::Subordinator::RatherThan) = take(children, 1)?
            else {
                return None;
            };
            let Lowered::GerundClause(alternative) = take(children, 2)? else {
                return None;
            };
            matrix.attachments.push(DependentAttachment {
                position: AttachmentPosition::AfterMatrix,
                comma: false,
                clause: DependentClause::Subordinate(
                    crate::syntax::Subordinator::RatherThan,
                    SubordinateBody::Gerund(alternative),
                ),
            });
            Some(Lowered::GerundClause(matrix))
        }
        RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseExistential
        | RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderAdverb
        | RuleTag::ClauseCopular
        | RuleTag::ClauseContractedCopular
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional => lower_simple_clause(tag, children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseAdverbBefore
        | RuleTag::ClausePrepositionalBefore
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterComma
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::SentencePeriod
        | RuleTag::SentenceNone => lower_composed_clause(tag, children),
        _ => None,
    }
}

fn lower_predicate(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::Verb => take(children, 0),
        RuleTag::VerbPhraseBase => {
            let Lowered::Verb(VerbAnalysis { instance, frame }) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::VerbPhrase(VerbPhrase {
                auxiliaries: Vec::new(),
                first_auxiliary_contracted_with_subject: false,
                preverb_modifiers: Vec::new(),
                verb: instance,
                frame,
                dependents: Vec::new(),
            }))
        }
        RuleTag::VerbPhraseAuxiliaryProform => {
            let Lowered::Auxiliary(auxiliary) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::VerbPhrase(VerbPhrase {
                auxiliaries: vec![auxiliary],
                first_auxiliary_contracted_with_subject: false,
                preverb_modifiers: Vec::new(),
                verb: VerbInstance {
                    verb: crate::word::Verb::Word(Vocab::Do),
                    slot: VerbSlot::Infinitive,
                },
                frame: crate::word::PROFORM_PREDICATE_FRAMES[0],
                dependents: Vec::new(),
            }))
        }
        RuleTag::VerbPhraseAuxiliary => {
            let Lowered::Auxiliary(auxiliary) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(mut predicate) = take(children, 1)? else {
                return None;
            };
            predicate.auxiliaries.insert(0, auxiliary);
            Some(Lowered::VerbPhrase(predicate))
        }
        RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseOracleSymbolCoordination
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity => lower_predicate_dependent(tag, children),
        RuleTag::InfinitiveTo | RuleTag::InfinitiveNotTo => {
            let negated = tag == RuleTag::InfinitiveNotTo;
            let predicate_index = if negated { 2 } else { 1 };
            let Lowered::VerbPhrase(predicate) = take(children, predicate_index)? else {
                return None;
            };
            Some(Lowered::InfinitiveClause(InfinitiveClause {
                negated,
                marker: InfinitiveMarker::To,
                predicate: Box::new(predicate),
            }))
        }
        _ => None,
    }
}

fn lower_predicate_dependent(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let Lowered::VerbPhrase(mut predicate) = take(children, 0)? else {
        return None;
    };
    let dependent = match tag {
        RuleTag::VerbPhraseDirectObject => {
            let Lowered::NounPhrase(noun_phrase) = take(children, 1)? else {
                return None;
            };
            match lowered_nominal_adjunct_kind(&predicate, &noun_phrase) {
                Some(BareNominalAdjunct::Temporal) => VerbDependent::Temporal(noun_phrase),
                Some(BareNominalAdjunct::Manner) => VerbDependent::Manner(noun_phrase),
                None => VerbDependent::DirectObject(noun_phrase),
            }
        }
        RuleTag::VerbPhraseIndirectObject => {
            let Lowered::NounPhrase(noun_phrase) = take(children, 1)? else {
                return None;
            };
            VerbDependent::IndirectObject(noun_phrase)
        }
        RuleTag::VerbPhraseAdjective => {
            let Lowered::AdjectivePhrase(adjective) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Adverbial(Phrase::AdjectivePhrase(Box::new(adjective)))
        }
        RuleTag::VerbPhrasePrepositional => {
            let Lowered::PrepositionalPhrase(preposition) = take(children, 1)? else {
                return None;
            };
            match predicate
                .frame
                .prepositional_role(preposition.preposition)?
            {
                PrepositionalRole::SelectedComplement => VerbDependent::PredicateComplement(
                    Phrase::PrepositionalPhrase(Box::new(preposition)),
                ),
                PrepositionalRole::Adjunct => VerbDependent::Prepositional(preposition),
            }
        }
        RuleTag::VerbPhraseInfinitive => {
            let Lowered::InfinitiveClause(infinitive) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Infinitive(infinitive)
        }
        RuleTag::VerbPhraseAdverb => {
            let Lowered::Adverb(adverb) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Adverbial(Phrase::Adverb(adverb))
        }
        RuleTag::VerbPhraseParticle => {
            let Lowered::VerbParticle(particle) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Particle(particle)
        }
        RuleTag::VerbPhraseFrequency => {
            let Lowered::Frequency(frequency) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Frequency(frequency)
        }
        RuleTag::VerbPhraseAbility => {
            let Lowered::Catalog(atom) = take(children, 1)? else {
                return None;
            };
            VerbDependent::PredicateComplement(Phrase::CatalogAtom(atom))
        }
        RuleTag::VerbPhraseOracleSymbol => {
            let Lowered::OracleSymbol(symbol) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Scalar(Phrase::OracleSymbol(symbol))
        }
        RuleTag::VerbPhraseSymbolSequence => {
            let Lowered::SymbolSequence(symbols) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Scalar(Phrase::SymbolSequence(symbols))
        }
        RuleTag::VerbPhraseOracleSymbolCoordination => {
            let Lowered::OracleSymbol(first) = take(children, 1)? else {
                return None;
            };
            let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                return None;
            };
            let Lowered::OracleSymbol(next) = take(children, 3)? else {
                return None;
            };
            VerbDependent::CoordinatedObject(CoordinatedPredicateObject {
                first: Box::new(PredicateObject::OracleSymbol(first)),
                rest: vec![PredicateObjectCoordination {
                    conjunction,
                    comma: false,
                    object: PredicateObject::OracleSymbol(next),
                }],
            })
        }
        RuleTag::VerbPhrasePowerToughness => {
            let Lowered::PowerToughness(power_toughness) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Statistic(Phrase::PowerToughness(power_toughness))
        }
        RuleTag::VerbPhraseQuantity => {
            let Lowered::Quantity(quantity) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Scalar(Phrase::Quantity(quantity))
        }
        _ => return None,
    };
    predicate.dependents.push(dependent);
    Some(Lowered::VerbPhrase(predicate))
}

#[allow(clippy::too_many_lines, reason = "lowering has many grammar variants")]
fn lower_simple_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::SimpleClauseSubject => {
            let Lowered::NounPhrase(subject) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::SimpleClause(SimpleClause {
                subject: Some(Subject(subject)),
                predicate,
            }))
        }
        RuleTag::SimpleClauseContractedSubject => {
            let Lowered::SubjectAuxiliary(subject_auxiliary) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(mut predicate) = take(children, 1)? else {
                return None;
            };
            predicate.auxiliaries.insert(0, subject_auxiliary.auxiliary);
            predicate.first_auxiliary_contracted_with_subject = true;
            Some(Lowered::SimpleClause(SimpleClause {
                subject: Some(subject_auxiliary.subject),
                predicate,
            }))
        }
        RuleTag::SimpleClauseSubjectless => {
            let Lowered::VerbPhrase(predicate) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::SimpleClause(SimpleClause {
                subject: None,
                predicate,
            }))
        }
        RuleTag::ClauseSimple => {
            let Lowered::SimpleClause(simple) = take(children, 0)? else {
                return None;
            };
            let independent = finish_simple_clause(simple)?;
            Some(Lowered::Clause(Clause::Independent(independent)))
        }
        RuleTag::ClauseElliptical => {
            let Lowered::AdjectivePhrase(adjective) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::EllipticalClause(EllipticalClause::Adjective(
                adjective,
            )))
        }
        RuleTag::ClauseExistential => {
            let Lowered::Existential(form) = take(children, 0)? else {
                return None;
            };
            let Lowered::NounPhrase(pivot) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Clause(Clause::Independent(
                IndependentClause::Existential(crate::syntax::ExistentialClause {
                    form,
                    pivot,
                    adjuncts: vec![],
                }),
            )))
        }
        tag @ (RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderAdverb) => lower_copular_remainder(tag, children),
        tag @ (RuleTag::ClauseCopular | RuleTag::ClauseContractedCopular) => {
            lower_copular_clause(tag, children)
        }
        RuleTag::RelativeObject => {
            let Lowered::NounPhrase(subject) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 1)? else {
                return None;
            };
            let FinishedPredicate { modal, predicate } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            let Predicate::Intransitive(predicate) = predicate else {
                return None;
            };
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::Zero,
                gap: RelativeGap::Object,
                body: RelativeBody::ObjectGap {
                    subject: Subject(subject),
                    predicate: ObjectGapPredicate {
                        head: predicate.head,
                        elements: predicate.elements,
                    },
                },
            }))
        }
        RuleTag::RelativeObjectContractedSubject => {
            let Lowered::SubjectAuxiliary(subject_auxiliary) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(mut predicate) = take(children, 1)? else {
                return None;
            };
            predicate.auxiliaries.insert(0, subject_auxiliary.auxiliary);
            predicate.first_auxiliary_contracted_with_subject = true;
            let FinishedPredicate { modal, predicate } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            let Predicate::Intransitive(predicate) = predicate else {
                return None;
            };
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::Zero,
                gap: RelativeGap::Object,
                body: RelativeBody::ObjectGap {
                    subject: subject_auxiliary.subject,
                    predicate: ObjectGapPredicate {
                        head: predicate.head,
                        elements: predicate.elements,
                    },
                },
            }))
        }
        RuleTag::RelativeSubjectContractedAuxiliary => {
            let Lowered::SubjectAuxiliary(subject_auxiliary) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(mut predicate) = take(children, 1)? else {
                return None;
            };
            predicate.auxiliaries.insert(0, subject_auxiliary.auxiliary);
            predicate.first_auxiliary_contracted_with_subject = true;
            let FinishedPredicate { modal, predicate } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::That,
                gap: RelativeGap::Subject,
                body: RelativeBody::SubjectGap(predicate),
            }))
        }
        RuleTag::RelativeSubject => {
            let Lowered::RelativeMarker(marker) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 1)? else {
                return None;
            };
            let FinishedPredicate { modal, predicate } = finish_predicate(predicate)?;
            let is_empty_proform = match &predicate {
                Predicate::Proform(_) => true,
                Predicate::Intransitive(intra) => intra.elements.is_empty(),
                _ => false,
            };
            let body = match modal {
                Some(modal) => RelativeBody::ModalSubjectGap {
                    modal,
                    predicate: if is_empty_proform { None } else { Some(predicate) },
                },
                None => RelativeBody::SubjectGap(predicate),
            };
            Some(Lowered::RelativeClause(RelativeClause {
                marker,
                gap: RelativeGap::Subject,
                body,
            }))
        }
        RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional => {
            let Lowered::SubjectAuxiliary(subject_auxiliary) = take(children, 0)? else {
                return None;
            };
            let complement = match tag {
                RuleTag::RelativeContractedCopularNoun => {
                    let Lowered::NounPhrase(complement) = take(children, 1)? else {
                        return None;
                    };
                    crate::syntax::CopularComplement::NounPhrase(complement)
                }
                RuleTag::RelativeContractedCopularAdjective => {
                    let Lowered::AdjectivePhrase(complement) = take(children, 1)? else {
                        return None;
                    };
                    crate::syntax::CopularComplement::Adjective(complement)
                }
                RuleTag::RelativeContractedCopularPrepositional => {
                    let Lowered::PrepositionalPhrase(complement) = take(children, 1)? else {
                        return None;
                    };
                    crate::syntax::CopularComplement::Prepositional(complement)
                }
                _ => return None,
            };
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::That,
                gap: RelativeGap::Subject,
                body: RelativeBody::SubjectGap(Predicate::Copular(
                    crate::syntax::CopularPredicate {
                        copula: crate::syntax::Copula {
                            auxiliary: subject_auxiliary.auxiliary,
                            contracted_with_subject: true,
                        },
                        precomplement_adverbs: vec![],
                        complement,
                        adjuncts: vec![],
                    },
                )),
            }))
        }
        _ => None,
    }
}

fn lower_copular_remainder(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let remainder = match tag {
        RuleTag::CopularRemainderNoun => {
            let Lowered::NounPhrase(complement) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::NounPhrase(complement),
            }
        }
        RuleTag::CopularRemainderAdjective => {
            let Lowered::AdjectivePhrase(complement) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::Adjective(complement),
            }
        }
        RuleTag::CopularRemainderPrepositional => {
            let Lowered::PrepositionalPhrase(complement) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::Prepositional(complement),
            }
        }
        RuleTag::CopularRemainderAdverb => {
            let Lowered::Adverb(adverb) = take(children, 0)? else {
                return None;
            };
            let Lowered::CopularRemainder(mut remainder) = take(children, 1)? else {
                return None;
            };
            remainder.precomplement_adverbs.insert(0, adverb);
            remainder
        }
        _ => return None,
    };
    Some(Lowered::CopularRemainder(remainder))
}

fn lower_copular_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let contracted = tag == RuleTag::ClauseContractedCopular;
    let (subject, copula, remainder_index) = if contracted {
        let Lowered::SubjectAuxiliary(subject_auxiliary) = take(children, 0)? else {
            return None;
        };
        if subject_auxiliary.auxiliary.auxiliary != Auxiliary::Be {
            return None;
        }
        (
            subject_auxiliary.subject,
            crate::syntax::Copula {
                auxiliary: subject_auxiliary.auxiliary,
                contracted_with_subject: true,
            },
            1,
        )
    } else {
        let Lowered::NounPhrase(subject) = take(children, 0)? else {
            return None;
        };
        let Lowered::Auxiliary(auxiliary) = take(children, 1)? else {
            return None;
        };
        (
            Subject(subject),
            crate::syntax::Copula {
                auxiliary,
                contracted_with_subject: false,
            },
            2,
        )
    };
    let Lowered::CopularRemainder(remainder) = take(children, remainder_index)? else {
        return None;
    };
    Some(Lowered::Clause(Clause::Independent(
        IndependentClause::Copular(
            subject,
            crate::syntax::CopularPredicate {
                copula,
                precomplement_adverbs: remainder.precomplement_adverbs,
                complement: remainder.complement,
                adjuncts: Vec::new(),
            },
        ),
    )))
}

fn lower_composed_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic => lower_coordination(tag, children),
        RuleTag::ClauseSubordinateBefore => {
            let Lowered::Subordinator(subordinator) = take(children, 0)? else {
                return None;
            };
            let Lowered::Clause(condition) = take(children, 1)? else {
                return None;
            };
            let Lowered::Clause(consequence) = take(children, 3)? else {
                return None;
            };
            conditional(
                subordinator,
                AttachmentPosition::BeforeMatrix,
                true,
                condition,
                consequence,
            )
        }
        RuleTag::ClauseAdverbBefore => {
            let Lowered::Adverb(adverb) = take(children, 0)? else {
                return None;
            };
            let Lowered::Clause(Clause::Independent(matrix)) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Clause(Clause::Independent(
                with_clause_attachment(
                    matrix,
                    ClauseAttachment {
                        position: AttachmentPosition::BeforeMatrix,
                        comma: false,
                        kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(adverb)),
                    },
                ),
            )))
        }
        RuleTag::ClausePrepositionalBefore => {
            let Lowered::PrepositionalPhrase(preposition) = take(children, 0)? else {
                return None;
            };
            let Lowered::Clause(Clause::Independent(matrix)) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::Clause(Clause::Independent(
                with_clause_attachment(
                    matrix,
                    ClauseAttachment {
                        position: AttachmentPosition::BeforeMatrix,
                        comma: true,
                        kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(
                            preposition,
                        )),
                    },
                ),
            )))
        }
        RuleTag::ClauseSubordinateAfterElliptical => {
            let Lowered::Clause(consequence) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(subordinator) = take(children, 1)? else {
                return None;
            };
            let Lowered::AdjectivePhrase(condition) = take(children, 2)? else {
                return None;
            };
            conditional_body(
                subordinator,
                AttachmentPosition::AfterMatrix,
                false,
                SubordinateBody::Elliptical(EllipticalClause::Adjective(condition)),
                consequence,
            )
        }
        RuleTag::ClauseSubordinateAfter | RuleTag::ClauseSubordinateAfterComma => {
            let offset = usize::from(tag == RuleTag::ClauseSubordinateAfterComma);
            let Lowered::Clause(consequence) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(subordinator) = take(children, 1 + offset)? else {
                return None;
            };
            let Lowered::Clause(condition) = take(children, 2 + offset)? else {
                return None;
            };
            conditional(
                subordinator,
                AttachmentPosition::AfterMatrix,
                tag == RuleTag::ClauseSubordinateAfterComma,
                condition,
                consequence,
            )
        }
        RuleTag::ClauseSubordinateAfterInfinitive => {
            let Lowered::Clause(consequence) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(crate::syntax::Subordinator::RatherThan) = take(children, 1)?
            else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 2)? else {
                return None;
            };
            let FinishedPredicate { modal, predicate } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            conditional_body(
                crate::syntax::Subordinator::RatherThan,
                AttachmentPosition::AfterMatrix,
                false,
                SubordinateBody::Infinitive(crate::syntax::InfinitiveClause {
                    negated: false,
                    marker: InfinitiveMarker::Bare,
                    predicate: Box::new(predicate),
                }),
                consequence,
            )
        }
        RuleTag::SentencePeriod | RuleTag::SentenceNone => {
            let Lowered::Clause(Clause::Independent(clause)) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Sentence(Sentence {
                initial_uppercase: true,
                body: SentenceBody::Independent(clause),
                ending: sentence_ending(tag)?,
            }))
        }
        _ => None,
    }
}

fn lower_coordination(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let Lowered::Clause(Clause::Independent(first)) = take(children, 0)? else {
        return None;
    };
    let (conjunction_index, clause_index, comma) = match tag {
        RuleTag::ClauseCoordination => (Some(1), 2, false),
        RuleTag::ClauseCoordinationComma => (Some(2), 3, true),
        RuleTag::ClauseCoordinationAsyndetic => (None, 2, true),
        _ => return None,
    };
    let conjunction = match conjunction_index {
        Some(index) => {
            let Lowered::Conjunction(conjunction) = take(children, index)? else {
                return None;
            };
            Some(conjunction)
        }
        None => None,
    };
    let Lowered::SimpleClause(next) = take(children, clause_index)? else {
        return None;
    };
    let member = if next.subject.is_some() {
        CoordinatedClauseMember::Independent(Box::new(finish_simple_clause(next)?))
    } else {
        let FinishedPredicate { modal, predicate } = finish_predicate(next.predicate)?;
        if modal.is_some() {
            return None;
        }
        CoordinatedClauseMember::SharedPredicate(predicate)
    };
    let coordination = ClauseCoordination {
        conjunction,
        comma,
        member,
    };
    let coordinated = match first {
        IndependentClause::Coordinated(mut coordinated) => {
            coordinated.rest.push(coordination);
            coordinated
        }
        first => CoordinatedIndependentClause {
            first: Box::new(first),
            rest: vec![coordination],
        },
    };
    Some(Lowered::Clause(Clause::Independent(
        IndependentClause::Coordinated(coordinated),
    )))
}

fn conditional(
    subordinator: crate::syntax::Subordinator,
    position: AttachmentPosition,
    comma: bool,
    condition: Clause,
    consequence: Clause,
) -> Option<Lowered> {
    let Clause::Independent(condition) = condition else {
        return None;
    };
    conditional_body(
        subordinator,
        position,
        comma,
        SubordinateBody::Finite(Box::new(condition)),
        consequence,
    )
}

fn conditional_body(
    subordinator: crate::syntax::Subordinator,
    position: AttachmentPosition,
    comma: bool,
    body: SubordinateBody,
    consequence: Clause,
) -> Option<Lowered> {
    let Clause::Independent(matrix) = consequence else {
        return None;
    };
    Some(Lowered::Clause(Clause::Independent(
        with_clause_attachment(
            matrix,
            ClauseAttachment {
                position,
                comma,
                kind: ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                    subordinator,
                    body,
                )),
            },
        ),
    )))
}

fn with_clause_attachment(
    matrix: IndependentClause,
    attachment: ClauseAttachment,
) -> IndependentClause {
    match matrix {
        IndependentClause::Complex(mut complex) => {
            match attachment.position {
                AttachmentPosition::BeforeMatrix => complex.attachments.insert(0, attachment),
                AttachmentPosition::AfterMatrix => complex.attachments.push(attachment),
            }
            IndependentClause::Complex(complex)
        }
        matrix => IndependentClause::Complex(ComplexClause {
            matrix: Box::new(matrix),
            attachments: vec![attachment],
        }),
    }
}

struct FinishedPredicate {
    modal: Option<Modal>,
    predicate: Predicate,
}

pub(super) fn finish_simple_clause(simple: SimpleClause) -> Option<IndependentClause> {
    let imperative = simple.subject.is_none() && simple.predicate.verb.slot == VerbSlot::Imperative;
    let subject = simple.subject;
    let FinishedPredicate { modal, predicate } = finish_predicate(simple.predicate)?;
    match (subject, modal, imperative) {
        (None, None, true) => Some(IndependentClause::Imperative(predicate)),
        (Some(subject), Some(modal), false) => {
            Some(IndependentClause::Deontic(subject, modal, predicate))
        }
        (Some(subject), None, false) => Some(independent_with_subject(subject, predicate)),
        _ => None,
    }
}

fn independent_with_subject(subject: Subject, predicate: Predicate) -> IndependentClause {
    match predicate {
        Predicate::Transitive(predicate) => IndependentClause::Transitive(subject, predicate),
        Predicate::Intransitive(predicate) => IndependentClause::Intransitive(subject, predicate),
        Predicate::Copular(predicate) => IndependentClause::Copular(subject, predicate),
        Predicate::Passive(predicate) => IndependentClause::Passive(subject, predicate),
        Predicate::Proform(predicate) => IndependentClause::Proform(subject, predicate),
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "sentence assembly is intentionally verbose"
)]
fn finish_predicate(mut phrase: VerbPhrase) -> Option<FinishedPredicate> {
    let proform = phrase.frame.is_proform();
    let modal = phrase
        .auxiliaries
        .first()
        .copied()
        .filter(|auxiliary| is_modal(auxiliary.auxiliary))
        .map(|auxiliary| {
            phrase.auxiliaries.remove(0);
            Modal { auxiliary }
        });
    let passive = phrase.verb.slot == VerbSlot::PastParticiple
        && phrase
            .auxiliaries
            .first()
            .is_some_and(|auxiliary| auxiliary.auxiliary == Auxiliary::Be);
    let mut object = None;
    let mut pre_object_elements = Vec::new();
    let mut elements = Vec::new();
    for dependent in phrase.dependents {
        let target_elements =
            if object.is_none() { &mut pre_object_elements } else { &mut elements };
        match dependent {
            VerbDependent::DirectObject(noun_phrase) => {
                attach_object(&mut object, PredicateObject::NounPhrase(noun_phrase))?;
            }
            VerbDependent::IndirectObject(noun_phrase) => {
                target_elements.push(PredicateElement::Complement(
                    PredicateComplement::IndirectObject(noun_phrase),
                ));
            }
            VerbDependent::PredicateComplement(phrase) => match phrase {
                Phrase::CatalogAtom(atom) => {
                    attach_object(
                        &mut object,
                        PredicateObject::Ability(AbilityObject {
                            ability: atom,
                            argument: None,
                        }),
                    )?;
                }
                Phrase::EmbeddedAbility(ability) => {
                    attach_object(&mut object, PredicateObject::EmbeddedAbility(ability))?;
                }
                Phrase::QuotedAbility(ability) => {
                    attach_object(&mut object, PredicateObject::QuotedAbility(ability))?;
                }
                Phrase::AdjectivePhrase(adjective) => {
                    target_elements.push(PredicateElement::Complement(
                        PredicateComplement::Adjective(*adjective),
                    ));
                }
                Phrase::PrepositionalPhrase(preposition) => {
                    target_elements.push(PredicateElement::Complement(
                        PredicateComplement::Prepositional(*preposition),
                    ));
                }
                _ => return None,
            },
            VerbDependent::Scalar(phrase) => match phrase {
                Phrase::Quantity(quantity) => {
                    attach_object(&mut object, PredicateObject::Quantity(quantity))?;
                }
                Phrase::OracleSymbol(symbol) => {
                    attach_object(&mut object, PredicateObject::OracleSymbol(symbol))?;
                }
                Phrase::SymbolSequence(symbols) => {
                    attach_object(&mut object, PredicateObject::SymbolSequence(symbols))?;
                }
                _ => return None,
            },
            VerbDependent::Statistic(Phrase::PowerToughness(value)) => {
                attach_object(&mut object, PredicateObject::PowerToughness(value))?;
            }
            VerbDependent::Prepositional(phrase) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Prepositional(
                    phrase,
                )));
            }
            VerbDependent::Temporal(phrase) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Temporal(
                    phrase,
                )));
            }
            VerbDependent::Manner(phrase) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Manner(phrase)));
            }
            VerbDependent::Infinitive(clause) => {
                target_elements.push(PredicateElement::Complement(
                    PredicateComplement::Infinitive(finish_infinitive(clause)?),
                ));
            }
            VerbDependent::Subordinate(clause) => {
                let Clause::Dependent(clause) = *clause else {
                    return None;
                };
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Dependent(
                    Box::new(clause),
                )));
            }
            VerbDependent::Adverbial(Phrase::Adverb(adverb)) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Adverb(adverb)));
            }
            VerbDependent::Adverbial(Phrase::AdjectivePhrase(adjective)) => {
                target_elements.push(PredicateElement::Complement(
                    PredicateComplement::Adjective(*adjective),
                ));
            }
            VerbDependent::Statistic(_) | VerbDependent::Adverbial(_) => return None,
            VerbDependent::Particle(particle) => {
                target_elements.push(PredicateElement::Particle(particle));
            }
            VerbDependent::Frequency(frequency) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Frequency(
                    frequency,
                )));
            }
            VerbDependent::CoordinatedObject(coordinated) => {
                attach_object(&mut object, PredicateObject::Coordinated(coordinated))?;
            }
        }
    }
    let head = PredicateHead {
        auxiliaries: phrase.auxiliaries,
        first_auxiliary_contracted_with_subject: phrase.first_auxiliary_contracted_with_subject,
        preverb_modifiers: phrase.preverb_modifiers,
        verb: phrase.verb,
    };
    let predicate = if passive {
        if object.is_some() {
            return None;
        }
        pre_object_elements.append(&mut elements);
        Predicate::Passive(PassivePredicate {
            head,
            elements: pre_object_elements,
        })
    } else if let Some(object) = object {
        Predicate::Transitive(crate::syntax::TransitivePredicate {
            head,
            pre_object_elements,
            object,
            elements,
        })
    } else if head.auxiliaries.is_empty()
        && modal.is_none()
        && head.preverb_modifiers.is_empty()
        && pre_object_elements.is_empty()
        && elements.is_empty()
        && proform
    {
        Predicate::Proform(ProPredicate {
            auxiliary: AuxiliaryInstance {
                auxiliary: Auxiliary::Do,
                inflection: proform_inflection(head.verb.slot),
                contracted_negation: false,
            },
        })
    } else if head.auxiliaries.len() == 1
        && head.preverb_modifiers.is_empty()
        && pre_object_elements.is_empty()
        && elements.is_empty()
        && proform
    {
        Predicate::Proform(ProPredicate {
            auxiliary: head.auxiliaries[0],
        })
    } else {
        pre_object_elements.append(&mut elements);
        Predicate::Intransitive(crate::syntax::IntransitivePredicate {
            head,
            elements: pre_object_elements,
        })
    };
    Some(FinishedPredicate { modal, predicate })
}

const fn proform_inflection(slot: VerbSlot) -> AuxiliaryInflection {
    match slot {
        VerbSlot::Infinitive | VerbSlot::Imperative => AuxiliaryInflection::Base,
        VerbSlot::Present { person, number } => AuxiliaryInflection::Present { person, number },
        VerbSlot::Past { person, number } => AuxiliaryInflection::Past { person, number },
        VerbSlot::PresentParticiple => AuxiliaryInflection::PresentParticiple,
        VerbSlot::PastParticiple => AuxiliaryInflection::PastParticiple,
    }
}

pub(super) fn finish_infinitive(
    clause: InfinitiveClause,
) -> Option<crate::syntax::InfinitiveClause> {
    let FinishedPredicate { modal, predicate } = finish_predicate(*clause.predicate)?;
    if modal.is_some() {
        return None;
    }
    Some(crate::syntax::InfinitiveClause {
        negated: clause.negated,
        marker: clause.marker,
        predicate: Box::new(predicate),
    })
}

fn attach_object(slot: &mut Option<PredicateObject>, object: PredicateObject) -> Option<()> {
    if let Some(PredicateObject::Ability(ability)) = slot
        && ability.argument.is_none()
    {
        ability.argument = Some(Box::new(object));
        return Some(());
    }
    if slot.replace(object).is_some() {
        return None;
    }
    Some(())
}

const fn is_modal(auxiliary: Auxiliary) -> bool {
    matches!(
        auxiliary,
        Auxiliary::Can
            | Auxiliary::Could
            | Auxiliary::May
            | Auxiliary::Might
            | Auxiliary::Must
            | Auxiliary::Shall
            | Auxiliary::Should
            | Auxiliary::Will
            | Auxiliary::Would
    )
}

fn lowered_nominal_adjunct_kind(
    predicate: &VerbPhrase,
    phrase: &NounPhrase,
) -> Option<BareNominalAdjunct> {
    let adjunct = nominal_adjunct_kind(phrase)?;
    if !predicate.frame.licenses_bare_nominal_adjunct(adjunct) {
        return None;
    }
    let (form, passive) = lowered_predicate_form(predicate)?;
    let has_direct_object = predicate.dependents.iter().any(|dependent| {
        matches!(
            dependent,
            VerbDependent::DirectObject(_)
                | VerbDependent::PredicateComplement(
                    Phrase::CatalogAtom(_) | Phrase::EmbeddedAbility(_) | Phrase::QuotedAbility(_)
                )
                | VerbDependent::Scalar(_)
                | VerbDependent::Statistic(_)
                | VerbDependent::CoordinatedObject(_)
        )
    });
    let has_tail = predicate.dependents.iter().any(|dependent| {
        !matches!(
            dependent,
            VerbDependent::DirectObject(_)
                | VerbDependent::IndirectObject(_)
                | VerbDependent::PredicateComplement(
                    Phrase::CatalogAtom(_) | Phrase::EmbeddedAbility(_) | Phrase::QuotedAbility(_)
                )
                | VerbDependent::Scalar(_)
                | VerbDependent::Statistic(_)
                | VerbDependent::CoordinatedObject(_)
        )
    });
    (form == PredicateForm::PastParticiple
        || passive
        || has_direct_object
        || has_tail
        || !predicate.frame.direct_object().accepts())
    .then_some(adjunct)
}

fn lowered_predicate_form(predicate: &VerbPhrase) -> Option<(PredicateForm, bool)> {
    let mut form = predicate_form(predicate.verb.slot);
    let mut passive = false;
    for auxiliary in predicate.auxiliaries.iter().rev() {
        passive |= auxiliary.auxiliary == Auxiliary::Be && form == PredicateForm::PastParticiple;
        form = auxiliary_form(*auxiliary, form)?;
    }
    Some((form, passive))
}

fn nominal_adjunct_kind(phrase: &NounPhrase) -> Option<BareNominalAdjunct> {
    let NounPhrase::Nominal(nominal) = phrase else {
        return None;
    };
    match &nominal.head {
        NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun) => {
            noun.bare_nominal_adjunct()
        }
    }
}

const fn sentence_ending(tag: RuleTag) -> Option<SentenceEnding> {
    match tag {
        RuleTag::SentencePeriod => Some(SentenceEnding::Period),
        RuleTag::SentenceNone => Some(SentenceEnding::None),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CatalogKind;
    use crate::catalog::Catalogs;
    use crate::syntax::Ability;
    use crate::syntax::AbilityKind;
    use crate::syntax::AdjectiveComplement;
    use crate::syntax::Demonstrative;
    use crate::syntax::Determiner;
    use crate::syntax::FrequencyBound;
    use crate::syntax::FrequencyCount;
    use crate::syntax::NominalComplement;
    use crate::syntax::NominalModifier;
    use crate::syntax::NounPhrase;
    use crate::syntax::OracleText;
    use crate::syntax::Paragraph;
    use crate::syntax::PredicateObject;
    use crate::syntax::RelativeBody;
    use crate::syntax::RelativeMarker;
    use crate::syntax::Sentence;
    use crate::syntax::SentenceBody;
    use crate::syntax::Subject;
    use crate::syntax::Subordinator;
    use crate::word::Adjective;
    use crate::word::Auxiliary;
    use crate::word::AuxiliaryInflection;
    use crate::word::Noun;
    use crate::word::NounInstance;
    use crate::word::Number;
    use crate::word::Person;
    use crate::word::Tense;
    use crate::word::Verb;
    use crate::word::VerbSlot;
    use crate::word::Vocab;

    const FIXTURES: [&str; 26] = [
        "Draw a card.",
        "Spells cost {1} less to cast.",
        "This creature costs {1} less to cast.",
        "Creatures you control attack each combat if able.",
        "Prevented damage is dealt to that creature's controller instead.",
        "Target creature gets +1/+1 until end of turn.",
        "Other Goblin creatures you control get +1/+1 and have haste.",
        "Creatures you control gain flying, then draw a card.",
        "If you control a Plains, creatures you control get +1/+1.",
        "Creatures you control get +1/+1 as long as you control a Goblin.",
        "You may exert this creature as it attacks.",
        "As this creature enters, choose a creature type.",
        "This creature attacks while saddled.",
        "Target creature you control fights target creature you don't control.",
        "You gain 2 life.",
        "This creature deals 3 damage to any target.",
        "Activate only as a sorcery.",
        "Activate only once each turn.",
        "This ability triggers only once each turn.",
        "This creature can't attack during extra turns.",
        "You may play that card this turn.",
        "You gain 1 life for each spell you've cast.",
        "It's put into exile.",
        "Counter target spell that's one or more colors.",
        "You may discard a Plains card rather than pay this spell's mana cost.",
        "Gain control of target creature for as long as you control this artifact.",
    ];

    #[test]
    fn clause_fixtures_parse_structurally_and_render_without_source() {
        for source in FIXTURES {
            let parsed = parse(source);
            assert_eq!(
                render_sentence(parsed.sentence().expect("sentence root")),
                source
            );
        }
    }

    #[test]
    fn finite_verbs_agree_with_their_subjects() {
        let plural_parse = parse("Spells cost {1} less to cast.");
        let (plural_subject, plural) = finite(plural_parse.sentence().unwrap());
        assert!(matches!(
            plural_subject,
            Subject(NounPhrase::Nominal(nominal))
                if matches!(nominal.head, NounInstance::Plural(Noun::Word(Vocab::Spell)))
        ));
        assert_eq!(
            plural.verb.slot,
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Plural,
            }
        );
        assert!(matches!(plural.verb.verb, Verb::Word(Vocab::Cost)));

        let singular_parse = parse("This creature costs {1} less to cast.");
        let (_, singular) = finite(singular_parse.sentence().unwrap());
        assert_eq!(
            singular.verb.slot,
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Singular,
            }
        );
    }

    #[test]
    fn exact_quantities_can_measure_mass_nouns() {
        for (source, expected) in [
            ("You gain 2 life.", Vocab::Life),
            ("This creature deals 3 damage to any target.", Vocab::Damage),
        ] {
            let parsed = parse(source);
            let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
                &parsed.sentence().expect("sentence root").body
            else {
                panic!("expected a transitive clause for {source:?}");
            };
            assert!(matches!(
                &predicate.object,
                PredicateObject::NounPhrase(NounPhrase::Nominal(nominal))
                    if matches!(
                        nominal.determiner,
                        Some(Determiner::Quantity(crate::syntax::Quantity::Exact(_)))
                    ) && matches!(
                        nominal.head,
                        NounInstance::Mass(Noun::Word(ref word)) if *word == expected
                    )
            ));
        }
    }

    #[test]
    fn as_fills_the_preposition_slot_after_an_adverb() {
        let source = "Activate only as a sorcery.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Intransitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected an imperative intransitive clause");
        };
        assert!(
            matches!(
                predicate.elements.as_slice(),
                [
                    PredicateElement::Adjunct(PredicateAdjunct::Adverb(Vocab::Only)),
                    PredicateElement::Adjunct(PredicateAdjunct::Prepositional(preposition)),
                ] if preposition.preposition == crate::syntax::Preposition::As
            ),
            "{predicate:#?}"
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn temporal_noun_phrases_can_follow_predicate_tail_adverbs() {
        for source in [
            "Activate only once each turn.",
            "This ability triggers only once each turn.",
            "Do this only once each turn.",
        ] {
            let parsed = parse(source);
            assert_eq!(render_sentence(parsed.sentence().expect(source)), source);
        }
    }

    #[test]
    fn activation_restriction_clauses_parse_and_render_structurally() {
        for source in [
            "Activate only during your turn, before attackers are declared.",
            "Activate only as a sorcery.",
        ] {
            let parsed = parse(source);
            assert_eq!(render_sentence(parsed.sentence().expect(source)), source);
        }
    }

    #[test]
    fn bounded_frequency_phrases_are_predicate_adjuncts() {
        for (source, expected_bound, expected_count) in [
            (
                "You may choose the same mode more than once.",
                FrequencyBound::MoreThan,
                FrequencyCount::Once,
            ),
            (
                "Activate no more than twice each turn.",
                FrequencyBound::NoMoreThan,
                FrequencyCount::Twice,
            ),
            (
                "Activate no more than three times each turn.",
                FrequencyBound::NoMoreThan,
                FrequencyCount::Times(crate::syntax::NumberLiteral {
                    value: 3,
                    numeral: crate::Numeral::Cardinal,
                }),
            ),
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            let elements = match &parsed.sentence().unwrap().body {
                SentenceBody::Independent(IndependentClause::Deontic(
                    _,
                    _,
                    Predicate::Transitive(predicate),
                )) => &predicate.elements,
                SentenceBody::Independent(IndependentClause::Imperative(
                    Predicate::Intransitive(predicate),
                )) => &predicate.elements,
                clause => panic!("expected a bounded-frequency predicate: {clause:#?}"),
            };
            assert!(
                elements.iter().any(|element| matches!(
                    element,
                    PredicateElement::Adjunct(PredicateAdjunct::Frequency(frequency))
                        if frequency.bound == expected_bound && frequency.count == expected_count
                )),
                "{source}: {elements:#?}"
            );
            assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        }
    }

    #[test]
    fn subject_relative_differential_comparisons_are_structural() {
        for (source, expected_adjective) in [
            (
                "Choose target opponent who has at least two more cards in hand than you do.",
                "more",
            ),
            (
                "Choose target opponent who has at least two fewer creature cards in their graveyard than you do.",
                "fewer",
            ),
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
                choose,
            ))) = &parsed.sentence().expect("sentence root").body
            else {
                panic!("expected an imperative choice: {:#?}", parsed.sentence());
            };
            let PredicateObject::NounPhrase(NounPhrase::Nominal(opponent)) = &choose.object else {
                panic!("expected an opponent object: {:#?}", choose.object);
            };
            let [NominalComplement::Relative(relative)] = opponent.complements.as_slice() else {
                panic!("expected one relative clause: {opponent:#?}");
            };
            assert_eq!(relative.marker, RelativeMarker::Who);
            let RelativeBody::SubjectGap(Predicate::Transitive(have)) = &relative.body else {
                panic!("expected a subject-gap transitive relative: {relative:#?}");
            };
            let PredicateObject::NounPhrase(NounPhrase::Nominal(cards)) = &have.object else {
                panic!("expected a compared card count: {:#?}", have.object);
            };
            let adjective = cards
                .modifiers
                .iter()
                .find_map(|modifier| match modifier {
                    NominalModifier::Adjective(adjective)
                        if matches!(
                            adjective.head,
                            Adjective::Word(word) if word.spelling() == expected_adjective
                        ) =>
                    {
                        Some(adjective)
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("missing {expected_adjective:?} modifier: {cards:#?}"));
            let [AdjectiveComplement::PostnominalComparison(comparison)] =
                adjective.complements.as_slice()
            else {
                panic!("expected a comparison complement: {adjective:#?}");
            };
            assert!(matches!(
                comparison.standard.as_ref(),
                crate::syntax::Phrase::Clause(clause)
                    if matches!(
                        clause.as_ref(),
                        Clause::Independent(IndependentClause::Proform(_, _))
                    )
            ));
            assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        }
    }

    #[test]
    fn plural_temporal_heads_do_not_become_late_objects() {
        let source = "This creature can't attack during extra turns.";
        let parsed = parse(source);
        assert!(
            matches!(
                &parsed.sentence().expect("sentence root").body,
                SentenceBody::Independent(IndependentClause::Deontic(
                    _,
                    _,
                    Predicate::Intransitive(predicate),
                )) if matches!(
                    predicate.elements.as_slice(),
                    [PredicateElement::Adjunct(PredicateAdjunct::Prepositional(preposition))]
                        if preposition.preposition == crate::syntax::Preposition::During
                )
            ),
            "{:#?}",
            parsed.sentence()
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn temporal_noun_phrases_can_follow_direct_objects() {
        let source = "You may play that card this turn.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Deontic(
            _,
            _,
            Predicate::Transitive(predicate),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a deontic transitive clause");
        };
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Adjunct(PredicateAdjunct::Temporal(
                NounPhrase::Nominal(turn),
            ))] if matches!(turn.head, NounInstance::Singular(Noun::Word(Vocab::Turn)))
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn selected_preposition_precedes_a_temporal_adjunct() {
        let source = "You may look at the top card of your library any time.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Deontic(
            _,
            _,
            Predicate::Intransitive(predicate),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a deontic intransitive clause: {:#?}",
                parsed.sentence()
            );
        };
        assert!(
            matches!(
                predicate.elements.as_slice(),
                [
                    PredicateElement::Complement(PredicateComplement::Prepositional(_)),
                    PredicateElement::Adjunct(PredicateAdjunct::Temporal(NounPhrase::Nominal(time))),
                ] if matches!(time.head, NounInstance::Singular(Noun::Word(Vocab::Time)))
            ),
            "{predicate:#?}"
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn intransitive_frame_rejects_a_direct_object() {
        let result = parse_nonterminal("Look a card.", &fixture_catalogs(), Nonterminal::Sentence);

        assert!(result.is_err());
    }

    #[test]
    fn ditransitive_frame_builds_an_indirect_object_complement() {
        let source = "Ask a player a number.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative: {:#?}", parsed.sentence());
        };
        assert!(matches!(
            predicate.object,
            PredicateObject::NounPhrase(NounPhrase::Nominal(ref number))
                if matches!(number.head, NounInstance::Singular(Noun::Word(Vocab::Number)))
        ));
        assert!(matches!(
            predicate.pre_object_elements.as_slice(),
            [PredicateElement::Complement(PredicateComplement::IndirectObject(
                NounPhrase::Nominal(player),
            ))] if matches!(player.head, NounInstance::Singular(Noun::Word(Vocab::Player)))
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn as_clauses_keep_their_surface_attachment_position() {
        for (source, position) in [
            (
                "You may exert this creature as it attacks.",
                AttachmentPosition::AfterMatrix,
            ),
            (
                "As this creature enters, choose a creature type.",
                AttachmentPosition::BeforeMatrix,
            ),
        ] {
            let parsed = parse(source);
            assert!(matches!(
                &parsed.sentence().expect("sentence root").body,
                SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                    attachments,
                    ..
                })) if matches!(
                    attachments.as_slice(),
                    [ClauseAttachment {
                        position: actual,
                        kind: ClauseAttachmentKind::Dependent(
                            DependentClause::Subordinate(
                                crate::syntax::Subordinator::As,
                                SubordinateBody::Finite(_),
                            ),
                        ),
                        ..
                    }] if *actual == position
                )
            ));
        }
    }

    #[test]
    fn fronted_cost_phrase_is_an_adjunct_with_an_infinitive_complement() {
        let source = "As an additional cost to cast this spell, sacrifice a creature.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause: {:#?}", parsed.sentence());
        };
        let [
            ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                comma: true,
                kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(preposition)),
            },
        ] = complex.attachments.as_slice()
        else {
            panic!("expected one fronted prepositional adjunct: {complex:#?}");
        };
        assert_eq!(preposition.preposition, crate::syntax::Preposition::As);
        let crate::syntax::Phrase::NounPhrase(noun_phrase) = preposition.object.as_ref() else {
            panic!("expected the adjunct to modify a cost noun phrase: {preposition:#?}");
        };
        let NounPhrase::Nominal(nominal) = noun_phrase.as_ref() else {
            panic!("expected a nominal cost phrase: {noun_phrase:#?}");
        };
        assert!(matches!(
            nominal.head,
            NounInstance::Singular(Noun::Word(Vocab::Cost))
        ));
        assert!(matches!(
            nominal.complements.as_slice(),
            [NominalComplement::Infinitive(
                crate::syntax::InfinitiveClause {
                    marker: InfinitiveMarker::To,
                    ..
                }
            )]
        ));
        assert!(matches!(
            complex.matrix.as_ref(),
            IndependentClause::Imperative(_)
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn this_way_is_a_manner_adjunct_inside_a_condition() {
        let source = "If you search your library this way, shuffle.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause: {:#?}", parsed.sentence());
        };
        let [
            ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                kind:
                    ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                        Subordinator::If,
                        SubordinateBody::Finite(condition),
                    )),
                ..
            },
        ] = complex.attachments.as_slice()
        else {
            panic!("expected a fronted if-clause: {complex:#?}");
        };
        let IndependentClause::Transitive(_, condition) = condition.as_ref() else {
            panic!("expected a transitive search condition: {condition:#?}");
        };
        assert!(matches!(
            condition.elements.as_slice(),
            [PredicateElement::Adjunct(PredicateAdjunct::Manner(
                NounPhrase::Nominal(way),
            ))] if matches!(way.head, NounInstance::Singular(Noun::Word(Vocab::Way)))
        ));
        assert!(matches!(
            complex.matrix.as_ref(),
            IndependentClause::Imperative(_)
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn then_can_modify_a_following_independent_clause() {
        let source = "Then that player shuffles.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a clause with a fronted adjunct: {:#?}",
                parsed.sentence()
            );
        };
        assert!(matches!(
            complex.attachments.as_slice(),
            [ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                comma: false,
                kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Then)),
            }]
        ));
        assert!(matches!(
            complex.matrix.as_ref(),
            IndependentClause::Intransitive(_, _)
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn for_as_long_as_is_one_finite_subordinator() {
        let parsed =
            parse("Gain control of target creature for as long as you control this artifact.");
        assert!(matches!(
            &parsed.sentence().expect("sentence root").body,
            SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                attachments,
                ..
            })) if matches!(
                attachments.as_slice(),
                [ClauseAttachment {
                    position: AttachmentPosition::AfterMatrix,
                    kind: ClauseAttachmentKind::Dependent(
                        DependentClause::Subordinate(
                            crate::syntax::Subordinator::ForAsLongAs,
                            SubordinateBody::Finite(_),
                        ),
                    ),
                    ..
                }]
            )
        ));
    }

    #[test]
    fn while_can_introduce_an_elliptical_postposed_clause() {
        let parsed = parse("This creature attacks while saddled.");
        assert!(matches!(
            &parsed.sentence().expect("sentence root").body,
            SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                attachments,
                ..
            })) if matches!(
                attachments.as_slice(),
                [ClauseAttachment {
                    position: AttachmentPosition::AfterMatrix,
                    kind: ClauseAttachmentKind::Dependent(
                        DependentClause::Subordinate(
                            crate::syntax::Subordinator::While,
                            SubordinateBody::Elliptical(EllipticalClause::Adjective(_)),
                        ),
                    ),
                    ..
                }]
            )
        ));
    }

    #[test]
    fn unless_introduces_a_finite_postposed_clause() {
        let parsed = parse("This land enters tapped unless you control a basic land.");
        assert!(matches!(
            &parsed.sentence().expect("sentence root").body,
            SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                attachments,
                ..
            })) if matches!(
                attachments.as_slice(),
                [ClauseAttachment {
                    position: AttachmentPosition::AfterMatrix,
                    kind: ClauseAttachmentKind::Dependent(
                        DependentClause::Subordinate(
                            crate::syntax::Subordinator::Unless,
                            SubordinateBody::Finite(_),
                        ),
                    ),
                    ..
                }]
            )
        ));
    }

    #[test]
    fn subjectless_imperatives_are_not_finite_subordinate_bodies() {
        let parsed = parse("Target creature gets +1/+1 until end of turn.");
        assert!(!parsed.chart.forest.nodes().any(|node| {
            node.key.symbol == crate::forest::ForestSymbol::Nonterminal(Nonterminal::Clause)
                && (node.key.start, node.key.end) == (5, 8)
                && matches!(
                    node.key.constituent_features(),
                    Some(Features::Clause { finite: true, .. })
                )
        }));
    }

    #[test]
    fn participle_position_distinguishes_modifier_from_passive_predicate() {
        let parsed = parse("Prevented damage is dealt to that creature's controller instead.");
        let SentenceBody::Independent(IndependentClause::Passive(
            Subject(NounPhrase::Nominal(subject)),
            predicate,
        )) = &parsed.sentence().unwrap().body
        else {
            panic!("expected nominal subject");
        };
        assert!(matches!(
            subject.modifiers.as_slice(),
            [NominalModifier::Adjective(adjective)]
                if matches!(
                    adjective.head,
                    Adjective::Participle(Tense::Past, Verb::Word(Vocab::Prevent))
                )
        ));
        assert!(matches!(
            predicate.head.auxiliaries.as_slice(),
            [auxiliary]
                if auxiliary.auxiliary == Auxiliary::Be
                    && auxiliary.inflection == (AuxiliaryInflection::Present {
                        person: Person::Third,
                        number: Number::Singular,
                    })
        ));
        assert!(matches!(predicate.head.verb.verb, Verb::Word(Vocab::Deal)));
        assert_eq!(predicate.head.verb.slot, VerbSlot::PastParticiple);
    }

    #[test]
    fn subject_copula_contractions_are_structural() {
        let catalogs = fixture_catalogs();
        for (source, contracted) in [("you are the monarch", false), ("you're the monarch", true)] {
            let parsed = parse_nonterminal(source, &catalogs, Nonterminal::Clause)
                .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            let Some(Clause::Independent(IndependentClause::Copular(subject, predicate))) =
                parsed.clause()
            else {
                panic!("expected a copular clause for {source:?}");
            };
            assert!(matches!(
                subject,
                Subject(NounPhrase::Pronoun {
                    pronoun: Pronoun::You,
                    case: PronounCase::Subject,
                })
            ));
            assert_eq!(predicate.copula.auxiliary.auxiliary, Auxiliary::Be);
            assert_eq!(
                predicate.copula.auxiliary.inflection,
                AuxiliaryInflection::Present {
                    person: Person::Second,
                    number: Number::Singular,
                }
            );
            assert_eq!(predicate.copula.contracted_with_subject, contracted);
            assert!(matches!(
                predicate.complement,
                crate::syntax::CopularComplement::NounPhrase(_)
            ));
        }
    }

    #[test]
    fn copular_adverbs_precede_the_complement() {
        for (source, contracted) in [("It is still a land.", false), ("It's still a land.", true)] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
            let SentenceBody::Independent(IndependentClause::Copular(subject, predicate)) =
                &parsed.sentence().expect("sentence root").body
            else {
                panic!("expected a copular clause: {:#?}", parsed.sentence());
            };
            assert!(matches!(
                subject,
                Subject(NounPhrase::Pronoun {
                    pronoun: Pronoun::It(crate::word::Gender::Neuter),
                    case: PronounCase::Subject,
                })
            ));
            assert_eq!(predicate.copula.contracted_with_subject, contracted);
            assert_eq!(predicate.precomplement_adverbs.len(), 1);
            assert_eq!(predicate.precomplement_adverbs[0].spelling(), "still");
            assert!(matches!(
                predicate.complement,
                crate::syntax::CopularComplement::NounPhrase(_)
            ));
            assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        }
    }

    #[test]
    fn contracted_subject_auxiliaries_are_structural() {
        let catalogs = fixture_catalogs();

        let parsed =
            parse_nonterminal("each spell you've cast", &catalogs, Nonterminal::NounPhrase)
                .expect("perfect relative clause should parse");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal with a relative clause");
        };
        let [NominalComplement::Relative(relative)] = nominal.complements.as_slice() else {
            panic!("expected a relative-clause complement: {nominal:#?}");
        };
        let RelativeBody::ObjectGap { subject, predicate } = &relative.body else {
            panic!("expected an object-gap relative clause");
        };
        assert!(matches!(
            subject,
            Subject(NounPhrase::Pronoun {
                pronoun: Pronoun::You,
                case: PronounCase::Subject,
            })
        ));
        assert!(predicate.head.first_auxiliary_contracted_with_subject);
        assert!(matches!(
            predicate.head.auxiliaries.as_slice(),
            [auxiliary] if auxiliary.auxiliary == Auxiliary::Have
        ));
        assert_eq!(predicate.head.verb.slot, VerbSlot::PastParticiple);

        let parsed = parse_nonterminal("it's put into exile", &catalogs, Nonterminal::Clause)
            .expect("contracted passive should parse");
        let Some(Clause::Independent(IndependentClause::Passive(subject, predicate))) =
            parsed.clause()
        else {
            panic!(
                "expected a contracted passive clause: {:#?}",
                parsed.clause()
            );
        };
        assert!(matches!(
            subject,
            Subject(NounPhrase::Pronoun {
                pronoun: Pronoun::It(crate::word::Gender::Neuter),
                case: PronounCase::Subject,
            })
        ));
        assert!(predicate.head.first_auxiliary_contracted_with_subject);
        assert!(matches!(
            predicate.head.auxiliaries.as_slice(),
            [auxiliary] if auxiliary.auxiliary == Auxiliary::Be
        ));

        let parsed = parse_nonterminal("that's one or more colors", &catalogs, Nonterminal::Clause)
            .expect("contracted demonstrative copula should parse");
        let Some(Clause::Independent(IndependentClause::Copular(subject, predicate))) =
            parsed.clause()
        else {
            panic!(
                "expected a contracted copular clause: {:#?}",
                parsed.clause()
            );
        };
        assert!(matches!(
            subject,
            Subject(NounPhrase::Demonstrative(Demonstrative::That))
        ));
        assert!(predicate.copula.contracted_with_subject);

        let parsed = parse_nonterminal(
            "a spell that's one or more colors",
            &catalogs,
            Nonterminal::NounPhrase,
        )
        .expect("contracted relative copula should parse");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal with a contracted relative clause");
        };
        assert!(
            matches!(
                nominal.complements.as_slice(),
                [NominalComplement::Relative(RelativeClause {
                    marker: RelativeMarker::That,
                    gap: RelativeGap::Subject,
                    body: RelativeBody::SubjectGap(Predicate::Copular(predicate)),
                })] if predicate.copula.contracted_with_subject
            ),
            "{nominal:#?}"
        );
    }

    #[test]
    fn rather_than_introduces_a_bare_infinitive_clause() {
        let parsed = parse("You may discard a Plains card rather than pay this spell's mana cost.");
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause");
        };
        assert!(matches!(
            complex.attachments.as_slice(),
            [ClauseAttachment {
                position: AttachmentPosition::AfterMatrix,
                comma: false,
                kind: ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                    Subordinator::RatherThan,
                    SubordinateBody::Infinitive(crate::syntax::InfinitiveClause {
                        marker: InfinitiveMarker::Bare,
                        ..
                    }),
                ),),
            }]
        ));
    }

    #[test]
    fn rather_than_can_contrast_gerund_clauses() {
        let source = "You may cast that card by paying life equal to the spell's mana value rather than paying its mana cost.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Deontic(
            _,
            _,
            Predicate::Transitive(cast),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a deontic cast clause: {:#?}", parsed.sentence());
        };
        let Some(PredicateElement::Adjunct(PredicateAdjunct::Prepositional(by))) =
            cast.elements.iter().find(|element| {
                matches!(
                    element,
                    PredicateElement::Adjunct(PredicateAdjunct::Prepositional(preposition))
                        if preposition.preposition == crate::syntax::Preposition::By
                )
            })
        else {
            panic!("expected a by-gerund adjunct: {cast:#?}");
        };
        let crate::syntax::Phrase::Clause(clause) = by.object.as_ref() else {
            panic!("by should take a clause: {by:#?}");
        };
        let Clause::Dependent(DependentClause::Gerund(gerund)) = clause.as_ref() else {
            panic!("by should take a gerund clause: {clause:#?}");
        };
        assert!(matches!(
            gerund.predicate.as_ref(),
            Predicate::Transitive(_)
        ));
        assert!(matches!(
            gerund.attachments.as_slice(),
            [DependentAttachment {
                position: AttachmentPosition::AfterMatrix,
                comma: false,
                clause: DependentClause::Subordinate(
                    Subordinator::RatherThan,
                    SubordinateBody::Gerund(alternative),
                ),
            }] if matches!(alternative.predicate.as_ref(), Predicate::Transitive(_))
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn not_to_negates_an_infinitive_clause() {
        let source = "You may choose not to untap this creature.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Deontic(
            _,
            _,
            Predicate::Intransitive(choose),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a deontic choose clause: {:#?}", parsed.sentence());
        };
        assert!(matches!(
            choose.elements.as_slice(),
            [PredicateElement::Complement(
                PredicateComplement::Infinitive(crate::syntax::InfinitiveClause {
                    negated: true,
                    ..
                })
            )]
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn directional_particle_completes_an_intransitive_predicate() {
        let source = "This creature phases out.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Intransitive(_, predicate)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected an intransitive phase clause: {:#?}",
                parsed.sentence()
            );
        };
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Particle(VerbParticle::Out)]
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn directional_particle_requires_a_licensed_verb_pair() {
        let parsed = parse_nonterminal(
            "This creature transforms out.",
            &fixture_catalogs(),
            Nonterminal::Sentence,
        )
        .expect("opaque-noun fallback should remain available");

        assert_eq!(parsed.opacity_mode(), OpacityMode::OpaqueNouns);
        assert!(!matches!(
            &parsed.sentence().expect("sentence root").body,
            SentenceBody::Independent(IndependentClause::Intransitive(_, predicate))
                if matches!(predicate.elements.as_slice(), [PredicateElement::Particle(_)])
        ));
    }

    #[test]
    fn face_down_is_a_secondary_adjective_predicate() {
        let source = "Turn this creature face down.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected an imperative turn clause: {:#?}",
                parsed.sentence()
            );
        };
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Complement(PredicateComplement::Adjective(
                AdjectivePhrase {
                    head: Adjective::CardOrientation(crate::word::CardOrientation::FaceDown),
                    complements,
                }
            ))] if complements.is_empty()
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn plus_coordinates_additive_noun_phrases() {
        let source = "You gain that much life plus 1.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a transitive gain clause: {:#?}",
                parsed.sentence()
            );
        };
        let PredicateObject::NounPhrase(NounPhrase::Coordinated(object)) = &predicate.object else {
            panic!("expected an additive noun phrase: {:#?}", predicate.object);
        };
        assert!(matches!(
            object.rest.as_slice(),
            [crate::syntax::NounPhraseCoordination {
                conjunction: crate::syntax::NounPhraseConjunction::Plus,
                ..
            }]
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn variable_quantity_has_singular_standalone_agreement() {
        let source = "X is 5 or more.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Copular(
            Subject(NounPhrase::Quantity(Quantity::X)),
            _,
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a copular variable clause: {:#?}",
                parsed.sentence()
            );
        };
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn count_sense_of_power_accepts_an_adjective_and_plural_inflection() {
        let source = "You control three creatures with different powers.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);

        let source = "You gain the difference.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn passive_blocking_treats_this_turn_as_a_temporal_adjunct() {
        let source = "Creatures you control can't be blocked this turn.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Deontic(
            _,
            _,
            Predicate::Passive(predicate),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a deontic passive clause");
        };
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Adjunct(PredicateAdjunct::Temporal(
                NounPhrase::Nominal(turn),
            ))] if matches!(turn.head, NounInstance::Singular(Noun::Word(Vocab::Turn)))
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn direct_objects_cannot_follow_predicate_tail_elements() {
        let result = parse_nonterminal(
            "You draw during your turn a card.",
            &fixture_catalogs(),
            Nonterminal::Sentence,
        );

        assert!(result.is_err());
    }

    #[test]
    fn predicate_prefix_pruning_keeps_possible_nominal_adjuncts() {
        let predicate = |object, phase| Features::VerbPhrase {
            form: PredicateForm::Imperative,
            passive: false,
            object,
            indirect_object: false,
            selected_preposition: false,
            phase,
            frame: PredicateFrame::OPEN,
            bare: object == PredicateObjectState::None,
        };
        let open = predicate(PredicateObjectState::None, PredicateAttachmentPhase::Object);
        let occupied = predicate(
            PredicateObjectState::Direct,
            PredicateAttachmentPhase::Object,
        );
        let ability = predicate(
            PredicateObjectState::Ability,
            PredicateAttachmentPhase::Object,
        );
        let tail = predicate(PredicateObjectState::None, PredicateAttachmentPhase::Tail);
        let prepositional_tail = predicate(
            PredicateObjectState::None,
            PredicateAttachmentPhase::PrepositionalTail,
        );

        assert!(accepts_predicate_prefix(
            RuleTag::VerbPhraseDirectObject,
            1,
            &open,
        ));
        assert!(accepts_predicate_prefix(
            RuleTag::VerbPhraseDirectObject,
            1,
            &occupied,
        ));
        assert!(accepts_predicate_prefix(
            RuleTag::VerbPhraseQuantity,
            1,
            &ability,
        ));
        assert!(accepts_predicate_prefix(
            RuleTag::VerbPhraseDirectObject,
            1,
            &tail,
        ));
        assert!(accepts_predicate_prefix(
            RuleTag::VerbPhraseDirectObject,
            1,
            &prepositional_tail,
        ));
    }

    #[test]
    fn karmic_justice_trigger_event_is_transitive() {
        let catalogs = fixture_catalogs();
        for noun_phrase in [
            "a spell or ability an opponent controls",
            "a noncreature permanent you control",
        ] {
            parse_nonterminal(noun_phrase, &catalogs, Nonterminal::NounPhrase)
                .unwrap_or_else(|error| panic!("failed to parse {noun_phrase:?}: {error:?}"));
        }
        let source =
            "a spell or ability an opponent controls destroys a noncreature permanent you control";
        let parsed = parse_nonterminal(source, &catalogs, Nonterminal::SimpleClause)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
        assert!(matches!(
            finish_simple_clause(parsed.simple_clause().unwrap().clone()),
            Some(IndependentClause::Transitive(_, _))
        ));
    }

    #[test]
    fn target_and_relative_clauses_keep_their_nominal_roles() {
        let target_parse = parse("Target creature gets +1/+1 until end of turn.");
        let (Subject(NounPhrase::Nominal(target)), _) = finite(target_parse.sentence().unwrap())
        else {
            panic!("expected target nominal subject");
        };
        assert_eq!(target.determiner, Some(Determiner::Target(None)));

        let fight_parse =
            parse("Target creature you control fights target creature you don't control.");
        let SentenceBody::Independent(IndependentClause::Transitive(
            Subject(NounPhrase::Nominal(subject)),
            fight,
        )) = &fight_parse.sentence().unwrap().body
        else {
            panic!("expected controlled target subject");
        };
        assert!(matches!(
            subject.complements.as_slice(),
            [NominalComplement::Relative(relative)]
                if relative.gap == crate::syntax::RelativeGap::Object
        ));
        let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &fight.object else {
            panic!(
                "expected one direct-object nominal, got {:#?}",
                fight.object
            );
        };
        assert!(matches!(
            object.complements.as_slice(),
            [NominalComplement::Relative(relative)]
                if relative.gap == crate::syntax::RelativeGap::Object
        ));
    }

    #[test]
    fn goblin_chieftain_stat_change_remains_one_magic_atom() {
        let sentence = parse("Other Goblin creatures you control get +1/+1 and have haste.");
        let SentenceBody::Independent(IndependentClause::Coordinated(coordination)) =
            &sentence.sentence().unwrap().body
        else {
            panic!("expected coordinated predicates");
        };
        let IndependentClause::Transitive(_, first) = coordination.first.as_ref() else {
            panic!("expected simple first predicate");
        };
        assert!(matches!(first.object, PredicateObject::PowerToughness(_)));
    }

    #[test]
    fn contiguous_oracle_symbols_are_one_scalar_object() {
        let source = "Add {C}{C}.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative: {:#?}", parsed.sentence());
        };
        assert!(matches!(
            &predicate.object,
            PredicateObject::SymbolSequence(symbols)
                if symbols.iter().map(crate::syntax::OracleSymbol::as_str).collect::<String>()
                    == "{C}{C}"
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn oracle_symbol_alternatives_are_a_coordinated_object() {
        let source = "Add {R} or {G}.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative: {:#?}", parsed.sentence());
        };
        assert!(matches!(
            &predicate.object,
            PredicateObject::Coordinated(CoordinatedPredicateObject { first, rest })
                if matches!(first.as_ref(), PredicateObject::OracleSymbol(symbol) if symbol.as_str() == "{R}")
                    && matches!(rest.as_slice(), [PredicateObjectCoordination {
                        conjunction: crate::syntax::PredicateConjunction::Or,
                        comma: false,
                        object: PredicateObject::OracleSymbol(symbol),
                    }] if symbol.as_str() == "{G}")
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn passive_temporal_adjunct_is_not_a_direct_object() {
        let source = "No spells were cast last turn.";
        let parsed = parse(source);
        assert!(
            matches!(
                &parsed.sentence().expect("sentence root").body,
                SentenceBody::Independent(IndependentClause::Passive(_, PassivePredicate {
                    elements,
                    ..
                })) if matches!(
                    elements.as_slice(),
                    [PredicateElement::Adjunct(PredicateAdjunct::Temporal(_))]
                )
            ),
            "{:#?}",
            parsed.sentence()
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn modal_subject_relative_keeps_its_passive_temporal_adjunct() {
        let source = "Prevent all combat damage that would be dealt this turn.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(matrix))) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative");
        };
        let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &matrix.object else {
            panic!("expected a nominal object");
        };
        assert!(matrix.elements.is_empty(), "{matrix:#?}");
        assert!(matches!(
            object.complements.as_slice(),
            [NominalComplement::Relative(RelativeClause {
                marker: RelativeMarker::That,
                gap: RelativeGap::Subject,
                body: RelativeBody::ModalSubjectGap {
                    modal: Modal {
                        auxiliary: AuxiliaryInstance {
                            auxiliary: Auxiliary::Would,
                            ..
                        },
                    },
                    predicate: Some(Predicate::Passive(PassivePredicate { elements, .. })),
                },
            })] if matches!(
                elements.as_slice(),
                [PredicateElement::Adjunct(PredicateAdjunct::Temporal(_))]
            )
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn multiple_fronted_clause_attachments_keep_surface_order() {
        let source = "At the beginning of each upkeep, if no spells were cast last turn, transform this creature.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert!(matches!(
            &parsed.sentence().expect("sentence root").body,
            SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                attachments,
                ..
            })) if matches!(
                attachments.as_slice(),
                [
                    ClauseAttachment {
                        kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(_)),
                        ..
                    },
                    ClauseAttachment {
                        kind: ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                            Subordinator::If,
                            SubordinateBody::Finite(_),
                        )),
                        ..
                    },
                ]
            )
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn if_you_dont_it_enters_tapped_parses() {
        let source = "If you don't, it enters tapped.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn enchanted_creature_cant_attack_or_block_parses() {
        let source = "Enchanted creature can't attack or block.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn this_creature_can_block_only_creatures_with_flying_parses() {
        let source = "This creature can block only creatures with flying.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    fn parse(source: &str) -> ParsedNonterminal {
        parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"))
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::KeywordAbility, ["Flying", "Haste"])
            .with_catalog(CatalogKind::CreatureType, ["Goblin"])
            .with_catalog(CatalogKind::LandType, ["Plains"])
            .with_catalog(CatalogKind::CardType, ["Creature", "Land", "Sorcery"])
    }

    fn finite(sentence: &Sentence) -> (&Subject, &crate::syntax::PredicateHead) {
        let SentenceBody::Independent(clause) = &sentence.body else {
            panic!("expected an independent clause, got {:?}", sentence.body);
        };
        match clause {
            IndependentClause::Transitive(subject, predicate) => (subject, &predicate.head),
            IndependentClause::Intransitive(subject, predicate) => (subject, &predicate.head),
            IndependentClause::Passive(subject, predicate) => (subject, &predicate.head),
            other => panic!("expected a finite lexical predicate, got {other:?}"),
        }
    }

    fn render_sentence(sentence: &Sentence) -> String {
        OracleText {
            abilities: vec![Ability {
                ability_word: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    flavor_header: None,
                    sentences: vec![sentence.clone()],
                }),
            }],
        }
        .render("Test Card", false)
        .expect("parsed sentence must render")
    }
}
