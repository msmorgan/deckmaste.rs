use super::*;
use crate::syntax::AbilityObject;
use crate::syntax::AttachmentPosition;
use crate::syntax::ClauseCoordination;
use crate::syntax::ComplexClause;
use crate::syntax::CoordinatedClauseMember;
use crate::syntax::CoordinatedIndependentClause;
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
use crate::syntax::RelativeBody;
use crate::syntax::RelativeMarker;
use crate::syntax::SentenceBody;
use crate::syntax::SentenceEnding;
use crate::syntax::SubordinateBody;

pub(super) fn add_rules(builder: &mut RuleBuilder) {
    use EnglishLexicalSlot as L;
    use Expected::Lexical as l;
    use Expected::Nonterminal as n;
    use Nonterminal as N;

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
        RuleTag::VerbPhraseDirectObject,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::NounPhrase)],
    );
    builder.add_with_cost(
        RuleTag::VerbPhraseAdjective,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::AdjectivePhrase)],
        ParseCost {
            precedence: 1,
            ..ParseCost::default()
        },
    );
    builder.add(
        RuleTag::VerbPhrasePrepositional,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::PrepositionalPhrase)],
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
    builder.add_with_cost(
        RuleTag::VerbPhraseAdjective,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::AdjectivePhrase)],
        ParseCost {
            precedence: 1,
            ..ParseCost::default()
        },
    );
    builder.add(
        RuleTag::VerbPhrasePrepositional,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::PrepositionalPhrase)],
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

    builder.add(
        RuleTag::InfinitiveTo,
        N::InfinitiveClause,
        [l(L::To), n(N::VerbPhrase)],
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
    builder.add(
        RuleTag::ClauseCopularNoun,
        N::Clause,
        [n(N::NounPhrase), l(L::Copula), n(N::NounPhrase)],
    );
    builder.add(
        RuleTag::ClauseCopularAdjective,
        N::Clause,
        [n(N::NounPhrase), l(L::Copula), n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::ClauseCopularPrepositional,
        N::Clause,
        [n(N::NounPhrase), l(L::Copula), n(N::PrepositionalPhrase)],
    );
    builder.add(
        RuleTag::ClauseContractedCopularNoun,
        N::Clause,
        [l(L::SubjectAuxiliary), n(N::NounPhrase)],
    );
    builder.add(
        RuleTag::ClauseContractedCopularAdjective,
        N::Clause,
        [l(L::SubjectAuxiliary), n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::ClauseContractedCopularPrepositional,
        N::Clause,
        [l(L::SubjectAuxiliary), n(N::PrepositionalPhrase)],
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

    for (tag, punctuation) in [
        (RuleTag::SentencePeriod, Punctuation::Period),
        (RuleTag::SentenceExclamation, Punctuation::Exclamation),
        (RuleTag::SentenceQuestion, Punctuation::Question),
    ] {
        builder.add(
            tag,
            N::Sentence,
            [n(N::Clause), l(L::Punctuation(punctuation))],
        );
    }
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
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::InfinitiveTo => reduce_predicate(tag, children),
        RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseExistential
        | RuleTag::ClauseCopularNoun
        | RuleTag::ClauseCopularAdjective
        | RuleTag::ClauseCopularPrepositional
        | RuleTag::ClauseContractedCopularNoun
        | RuleTag::ClauseContractedCopularAdjective
        | RuleTag::ClauseContractedCopularPrepositional
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional => reduce_simple_clause(tag, children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::SentencePeriod
        | RuleTag::SentenceExclamation
        | RuleTag::SentenceQuestion
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
    let object_rule = matches!(
        tag,
        RuleTag::VerbPhraseDirectObject
            | RuleTag::VerbPhraseAbility
            | RuleTag::VerbPhraseOracleSymbol
            | RuleTag::VerbPhrasePowerToughness
    );
    let quantity_rule = tag == RuleTag::VerbPhraseQuantity;
    if !object_rule && !quantity_rule {
        return true;
    }
    let Features::VerbPhrase {
        object,
        phase,
        accepts_direct_object,
        ..
    } = features
    else {
        return false;
    };
    if tag == RuleTag::VerbPhraseDirectObject
        && (*phase == PredicateAttachmentPhase::Tail || object.has_direct_object())
    {
        return true;
    }
    if *phase != PredicateAttachmentPhase::Object {
        return false;
    }
    if quantity_rule {
        matches!(
            object,
            PredicateObjectState::None | PredicateObjectState::Ability
        )
    } else {
        *accepts_direct_object && *object == PredicateObjectState::None
    }
}

fn reduce_predicate(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::Verb => Some(propagate(children.first()?)),
        RuleTag::VerbPhraseBase => {
            let Features::Verb {
                slot,
                accepts_direct_object,
            } = children.first()?.features
            else {
                return None;
            };
            let form = predicate_form(*slot);
            Some(Features::VerbPhrase {
                form,
                object: PredicateObjectState::None,
                phase: PredicateAttachmentPhase::Object,
                accepts_direct_object: *accepts_direct_object,
            })
        }
        RuleTag::VerbPhraseAuxiliary => {
            let Features::Auxiliary(auxiliary) = children.first()?.features else {
                return None;
            };
            let Features::VerbPhrase {
                form: child_form,
                object,
                phase,
                accepts_direct_object,
            } = children.get(1)?.features
            else {
                return None;
            };
            let form = auxiliary_form(*auxiliary, *child_form)?;
            Some(Features::VerbPhrase {
                form,
                object: *object,
                phase: *phase,
                accepts_direct_object: *accepts_direct_object,
            })
        }
        RuleTag::VerbPhraseDirectObject => {
            let Features::NounPhrase {
                pronoun_case,
                temporal,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if *pronoun_case == Some(PronounCase::Subject) {
                return None;
            }
            let Features::VerbPhrase { object, phase, .. } = children.first()?.features else {
                return None;
            };
            let attachment = if *temporal
                && (*phase == PredicateAttachmentPhase::Tail || object.has_direct_object())
            {
                ObjectAttachment::None
            } else {
                ObjectAttachment::Direct
            };
            extend_predicate(children.first()?, attachment)
        }
        RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity => {
            let attachment = match tag {
                RuleTag::VerbPhraseAbility => ObjectAttachment::Ability,
                RuleTag::VerbPhrasePrepositional => ObjectAttachment::Prepositional,
                RuleTag::VerbPhraseOracleSymbol | RuleTag::VerbPhrasePowerToughness => {
                    ObjectAttachment::Direct
                }
                RuleTag::VerbPhraseQuantity => ObjectAttachment::DirectOrAbilityArgument,
                _ => ObjectAttachment::None,
            };
            extend_predicate(children.first()?, attachment)
        }
        RuleTag::InfinitiveTo => {
            let Features::VerbPhrase {
                form: PredicateForm::Infinitive,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            Some(Features::InfinitiveClause)
        }
        _ => None,
    }
}

fn extend_predicate(
    predicate: &Child<'_, EnglishGrammar<'_, '_>>,
    attachment: ObjectAttachment,
) -> Option<Reduced> {
    let Features::VerbPhrase {
        form,
        object,
        phase,
        accepts_direct_object,
    } = predicate.features
    else {
        return None;
    };
    let next_phase = match (attachment, *phase) {
        (ObjectAttachment::None, _) => PredicateAttachmentPhase::Tail,
        (ObjectAttachment::Prepositional, _) => PredicateAttachmentPhase::PrepositionalTail,
        (_, PredicateAttachmentPhase::Object) => PredicateAttachmentPhase::Object,
        (_, PredicateAttachmentPhase::Tail | PredicateAttachmentPhase::PrepositionalTail) => {
            return None;
        }
    };
    if !*accepts_direct_object
        && !matches!(
            attachment,
            ObjectAttachment::None | ObjectAttachment::Prepositional
        )
    {
        return None;
    }
    let object = match (attachment, *object) {
        (ObjectAttachment::None | ObjectAttachment::Prepositional, object) => object,
        (ObjectAttachment::Direct, PredicateObjectState::None) => PredicateObjectState::Direct,
        (ObjectAttachment::Ability, PredicateObjectState::None) => PredicateObjectState::Ability,
        (ObjectAttachment::DirectOrAbilityArgument, PredicateObjectState::None) => {
            PredicateObjectState::Direct
        }
        (ObjectAttachment::DirectOrAbilityArgument, PredicateObjectState::Ability) => {
            PredicateObjectState::AbilityWithArgument
        }
        _ => return None,
    };
    Some(Features::VerbPhrase {
        form: *form,
        object,
        phase: next_phase,
        accepts_direct_object: *accepts_direct_object,
    })
}

#[derive(Debug, Clone, Copy)]
enum ObjectAttachment {
    None,
    Prepositional,
    Direct,
    Ability,
    DirectOrAbilityArgument,
}

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
                object,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if predicate_agreement.is_some_and(|agreement| agreement != *subject_agreement) {
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
                object,
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
            Some(simple_clause_reduction(
                Some(*subject_agreement),
                true,
                true,
                object.has_direct_object(),
            ))
        }
        RuleTag::SimpleClauseSubjectless => {
            let Features::VerbPhrase { form, object, .. } = children.first()?.features else {
                return None;
            };
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
                PredicateForm::Infinitive
                | PredicateForm::PresentParticiple
                | PredicateForm::PastParticiple => None,
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
        tag @ (RuleTag::ClauseCopularNoun
        | RuleTag::ClauseCopularAdjective
        | RuleTag::ClauseCopularPrepositional
        | RuleTag::ClauseContractedCopularNoun
        | RuleTag::ClauseContractedCopularAdjective
        | RuleTag::ClauseContractedCopularPrepositional) => reduce_copular_clause(tag, children),
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
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if predicate_agreement.is_some_and(|agreement| agreement != *subject_agreement) {
                return None;
            }
            Some(Features::RelativeClause(RelativeGap::Object))
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
            Some(Features::RelativeClause(RelativeGap::Object))
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
            Some(Features::RelativeClause(RelativeGap::Subject))
        }
        RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional => {
            let Features::SubjectAuxiliary {
                subject: ContractedSubjectKey::Demonstrative(Demonstrative::That),
                auxiliary,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if auxiliary.auxiliary != Auxiliary::Be {
                return None;
            }
            Some(Features::RelativeClause(RelativeGap::Subject))
        }
        _ => None,
    }
}

fn reduce_copular_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let contracted = matches!(
        tag,
        RuleTag::ClauseContractedCopularNoun
            | RuleTag::ClauseContractedCopularAdjective
            | RuleTag::ClauseContractedCopularPrepositional
    );
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
        RuleTag::ClauseCoordination | RuleTag::ClauseCoordinationComma => {
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
        RuleTag::SentencePeriod
        | RuleTag::SentenceExclamation
        | RuleTag::SentenceQuestion
        | RuleTag::SentenceNone => {
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
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::InfinitiveTo => lower_predicate(tag, children),
        RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseExistential
        | RuleTag::ClauseCopularNoun
        | RuleTag::ClauseCopularAdjective
        | RuleTag::ClauseCopularPrepositional
        | RuleTag::ClauseContractedCopularNoun
        | RuleTag::ClauseContractedCopularAdjective
        | RuleTag::ClauseContractedCopularPrepositional
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional => lower_simple_clause(tag, children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::SentencePeriod
        | RuleTag::SentenceExclamation
        | RuleTag::SentenceQuestion
        | RuleTag::SentenceNone => lower_composed_clause(tag, children),
        _ => None,
    }
}

fn lower_predicate(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::Verb => take(children, 0),
        RuleTag::VerbPhraseBase => {
            let Lowered::Verb(verb) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::VerbPhrase(VerbPhrase {
                auxiliaries: Vec::new(),
                first_auxiliary_contracted_with_subject: false,
                preverb_modifiers: Vec::new(),
                verb,
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
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity => lower_predicate_dependent(tag, children),
        RuleTag::InfinitiveTo => {
            let Lowered::VerbPhrase(predicate) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::InfinitiveClause(InfinitiveClause {
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
            if !predicate.dependents.is_empty() && is_temporal_noun_phrase(&noun_phrase) {
                VerbDependent::Temporal(noun_phrase)
            } else {
                VerbDependent::DirectObject(noun_phrase)
            }
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
            VerbDependent::Prepositional(preposition)
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
        tag @ (RuleTag::ClauseCopularNoun
        | RuleTag::ClauseCopularAdjective
        | RuleTag::ClauseCopularPrepositional
        | RuleTag::ClauseContractedCopularNoun
        | RuleTag::ClauseContractedCopularAdjective
        | RuleTag::ClauseContractedCopularPrepositional) => lower_copular_clause(tag, children),
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
                        complement,
                        adjuncts: vec![],
                    },
                )),
            }))
        }
        _ => None,
    }
}

fn lower_copular_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let contracted = matches!(
        tag,
        RuleTag::ClauseContractedCopularNoun
            | RuleTag::ClauseContractedCopularAdjective
            | RuleTag::ClauseContractedCopularPrepositional
    );
    let (subject, copula, complement_index) = if contracted {
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
    let complement = match tag {
        RuleTag::ClauseCopularNoun | RuleTag::ClauseContractedCopularNoun => {
            let Lowered::NounPhrase(complement) = take(children, complement_index)? else {
                return None;
            };
            crate::syntax::CopularComplement::NounPhrase(complement)
        }
        RuleTag::ClauseCopularAdjective | RuleTag::ClauseContractedCopularAdjective => {
            let Lowered::AdjectivePhrase(complement) = take(children, complement_index)? else {
                return None;
            };
            crate::syntax::CopularComplement::Adjective(complement)
        }
        RuleTag::ClauseCopularPrepositional | RuleTag::ClauseContractedCopularPrepositional => {
            let Lowered::PrepositionalPhrase(complement) = take(children, complement_index)? else {
                return None;
            };
            crate::syntax::CopularComplement::Prepositional(complement)
        }
        _ => return None,
    };
    Some(Lowered::Clause(Clause::Independent(
        IndependentClause::Copular(
            subject,
            crate::syntax::CopularPredicate {
                copula,
                complement,
                adjuncts: Vec::new(),
            },
        ),
    )))
}

fn lower_composed_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::ClauseCoordination | RuleTag::ClauseCoordinationComma => {
            lower_coordination(tag, children)
        }
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
        RuleTag::ClauseSubordinateAfter => {
            let Lowered::Clause(consequence) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(subordinator) = take(children, 1)? else {
                return None;
            };
            let Lowered::Clause(condition) = take(children, 2)? else {
                return None;
            };
            conditional(
                subordinator,
                AttachmentPosition::AfterMatrix,
                false,
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
                    marker: InfinitiveMarker::Bare,
                    predicate: Box::new(predicate),
                }),
                consequence,
            )
        }
        RuleTag::SentencePeriod
        | RuleTag::SentenceExclamation
        | RuleTag::SentenceQuestion
        | RuleTag::SentenceNone => {
            let Lowered::Clause(Clause::Independent(clause)) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Sentence(Sentence {
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
        RuleTag::ClauseCoordination => (1, 2, false),
        RuleTag::ClauseCoordinationComma => (2, 3, true),
        _ => return None,
    };
    let Lowered::Conjunction(conjunction) = take(children, conjunction_index)? else {
        return None;
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
        IndependentClause::Complex(ComplexClause {
            matrix: Box::new(matrix),
            attachments: vec![DependentAttachment {
                position,
                comma,
                clause: DependentClause::Subordinate(subordinator, body),
            }],
        }),
    )))
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
        (Some(subject), None, false) => independent_with_subject(subject, predicate),
        _ => None,
    }
}

fn independent_with_subject(subject: Subject, predicate: Predicate) -> Option<IndependentClause> {
    Some(match predicate {
        Predicate::Transitive(predicate) => IndependentClause::Transitive(subject, predicate),
        Predicate::Intransitive(predicate) => IndependentClause::Intransitive(subject, predicate),
        Predicate::Copular(predicate) => IndependentClause::Copular(subject, predicate),
        Predicate::Passive(predicate) => IndependentClause::Passive(subject, predicate),
        Predicate::Proform(predicate) => IndependentClause::Proform(subject, predicate),
    })
}

fn finish_predicate(mut phrase: VerbPhrase) -> Option<FinishedPredicate> {
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
    let mut elements = Vec::new();
    for dependent in phrase.dependents {
        match dependent {
            VerbDependent::DirectObject(noun_phrase)
                if is_temporal_adjunct(&phrase.verb, &noun_phrase) =>
            {
                elements.push(PredicateElement::Adjunct(PredicateAdjunct::Temporal(
                    noun_phrase,
                )));
            }
            VerbDependent::DirectObject(noun_phrase) => {
                attach_object(&mut object, PredicateObject::NounPhrase(noun_phrase))?;
            }
            VerbDependent::IndirectObject(noun_phrase) => {
                elements.push(PredicateElement::Complement(
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
                    elements.push(PredicateElement::Complement(
                        PredicateComplement::Adjective(*adjective),
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
                _ => return None,
            },
            VerbDependent::Statistic(Phrase::PowerToughness(value)) => {
                attach_object(&mut object, PredicateObject::PowerToughness(value))?;
            }
            VerbDependent::Statistic(_) => return None,
            VerbDependent::Prepositional(phrase) => {
                elements.push(PredicateElement::Adjunct(PredicateAdjunct::Prepositional(
                    phrase,
                )));
            }
            VerbDependent::Temporal(phrase) => {
                elements.push(PredicateElement::Adjunct(PredicateAdjunct::Temporal(
                    phrase,
                )));
            }
            VerbDependent::Infinitive(clause) => {
                elements.push(PredicateElement::Complement(
                    PredicateComplement::Infinitive(finish_infinitive(clause)?),
                ));
            }
            VerbDependent::Subordinate(clause) => {
                let Clause::Dependent(clause) = *clause else {
                    return None;
                };
                elements.push(PredicateElement::Adjunct(PredicateAdjunct::Dependent(
                    Box::new(clause),
                )));
            }
            VerbDependent::Adverbial(Phrase::Adverb(adverb)) => {
                elements.push(PredicateElement::Adjunct(PredicateAdjunct::Adverb(adverb)));
            }
            VerbDependent::Adverbial(Phrase::AdjectivePhrase(adjective)) => {
                elements.push(PredicateElement::Complement(
                    PredicateComplement::Adjective(*adjective),
                ));
            }
            VerbDependent::Adverbial(_) => return None,
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
        Predicate::Passive(PassivePredicate { head, elements })
    } else if let Some(object) = object {
        Predicate::Transitive(crate::syntax::TransitivePredicate {
            head,
            object,
            elements,
        })
    } else {
        Predicate::Intransitive(crate::syntax::IntransitivePredicate { head, elements })
    };
    Some(FinishedPredicate { modal, predicate })
}

fn finish_infinitive(clause: InfinitiveClause) -> Option<crate::syntax::InfinitiveClause> {
    let FinishedPredicate { modal, predicate } = finish_predicate(*clause.predicate)?;
    if modal.is_some() {
        return None;
    }
    Some(crate::syntax::InfinitiveClause {
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

fn is_temporal_adjunct(verb: &VerbInstance, phrase: &NounPhrase) -> bool {
    if !matches!(
        verb.verb,
        crate::word::Verb::Word(Vocab::Attack | Vocab::Block)
    ) {
        return false;
    }
    is_temporal_noun_phrase(phrase)
}

fn is_temporal_noun_phrase(phrase: &NounPhrase) -> bool {
    matches!(
        phrase,
        NounPhrase::Nominal(nominal)
            if matches!(
                nominal.head,
                NounInstance::Singular(Noun::Word(Vocab::Combat | Vocab::Turn))
                    | NounInstance::Plural(Noun::Word(Vocab::Combat | Vocab::Turn))
            )
    )
}

const fn sentence_ending(tag: RuleTag) -> Option<SentenceEnding> {
    match tag {
        RuleTag::SentencePeriod => Some(SentenceEnding::Period(1)),
        RuleTag::SentenceExclamation => Some(SentenceEnding::Exclamation(1)),
        RuleTag::SentenceQuestion => Some(SentenceEnding::Question(1)),
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
    use crate::syntax::Demonstrative;
    use crate::syntax::Determiner;
    use crate::syntax::NominalComplement;
    use crate::syntax::NominalModifier;
    use crate::syntax::NounPhrase;
    use crate::syntax::OracleText;
    use crate::syntax::Paragraph;
    use crate::syntax::PredicateObject;
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

    const FIXTURES: [&str; 25] = [
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
        assert!(matches!(
            predicate.elements.as_slice(),
            [
                PredicateElement::Adjunct(PredicateAdjunct::Adverb(Vocab::Only)),
                PredicateElement::Adjunct(PredicateAdjunct::Prepositional(preposition)),
            ] if preposition.preposition == crate::syntax::Preposition::As
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn temporal_noun_phrases_can_follow_predicate_tail_adverbs() {
        for source in [
            "Activate only once each turn.",
            "This ability triggers only once each turn.",
        ] {
            let parsed = parse(source);
            let predicate = match &parsed.sentence().expect("sentence root").body {
                SentenceBody::Independent(IndependentClause::Imperative(
                    Predicate::Intransitive(predicate),
                ))
                | SentenceBody::Independent(IndependentClause::Intransitive(_, predicate)) => {
                    predicate
                }
                clause => panic!("expected an intransitive clause, got {clause:#?}"),
            };
            assert!(matches!(
                predicate.elements.as_slice(),
                [
                    PredicateElement::Adjunct(PredicateAdjunct::Adverb(Vocab::Only)),
                    PredicateElement::Adjunct(PredicateAdjunct::Adverb(once)),
                    PredicateElement::Adjunct(PredicateAdjunct::Temporal(
                        NounPhrase::Nominal(turn),
                    )),
                ] if once.spelling() == "once"
                    && matches!(turn.head, NounInstance::Singular(Noun::Word(Vocab::Turn)))
            ));
            assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        }
    }

    #[test]
    fn plural_temporal_heads_do_not_become_late_objects() {
        let source = "This creature can't attack during extra turns.";
        let parsed = parse(source);
        assert!(matches!(
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
        ));
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
                    [DependentAttachment {
                        position: actual,
                        clause: DependentClause::Subordinate(
                            crate::syntax::Subordinator::As,
                            SubordinateBody::Finite(_),
                        ),
                        ..
                    }] if *actual == position
                )
            ));
        }
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
                [DependentAttachment {
                    position: AttachmentPosition::AfterMatrix,
                    clause: DependentClause::Subordinate(
                        crate::syntax::Subordinator::While,
                        SubordinateBody::Elliptical(EllipticalClause::Adjective(_)),
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
                [DependentAttachment {
                    position: AttachmentPosition::AfterMatrix,
                    clause: DependentClause::Subordinate(
                        crate::syntax::Subordinator::Unless,
                        SubordinateBody::Finite(_),
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
            [DependentAttachment {
                position: AttachmentPosition::AfterMatrix,
                comma: false,
                clause: DependentClause::Subordinate(
                    Subordinator::RatherThan,
                    SubordinateBody::Infinitive(crate::syntax::InfinitiveClause {
                        marker: InfinitiveMarker::Bare,
                        ..
                    }),
                ),
            }]
        ));
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
    fn predicate_prefix_pruning_keeps_possible_temporal_candidates() {
        let open = Features::VerbPhrase {
            form: PredicateForm::Imperative,
            object: PredicateObjectState::None,
            phase: PredicateAttachmentPhase::Object,
            accepts_direct_object: true,
        };
        let occupied = Features::VerbPhrase {
            form: PredicateForm::Imperative,
            object: PredicateObjectState::Direct,
            phase: PredicateAttachmentPhase::Object,
            accepts_direct_object: true,
        };
        let ability = Features::VerbPhrase {
            form: PredicateForm::Imperative,
            object: PredicateObjectState::Ability,
            phase: PredicateAttachmentPhase::Object,
            accepts_direct_object: true,
        };
        let tail = Features::VerbPhrase {
            form: PredicateForm::Imperative,
            object: PredicateObjectState::None,
            phase: PredicateAttachmentPhase::Tail,
            accepts_direct_object: true,
        };
        let prepositional_tail = Features::VerbPhrase {
            form: PredicateForm::Imperative,
            object: PredicateObjectState::None,
            phase: PredicateAttachmentPhase::PrepositionalTail,
            accepts_direct_object: true,
        };

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
        assert!(!accepts_predicate_prefix(
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
                    sentences: vec![sentence.clone()],
                }),
            }],
        }
        .render("Test Card", false)
        .expect("parsed sentence must render")
    }
}
