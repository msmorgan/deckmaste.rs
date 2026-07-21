use super::*;
use crate::syntax::ClauseCoordination;
use crate::syntax::ConditionalClause;
use crate::syntax::ConditionalPosition;
use crate::syntax::CoordinatedClause;
use crate::syntax::InfinitiveMarker;
use crate::syntax::SentenceEnding;
use crate::syntax::VerbDependent;

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
        RuleTag::SimpleClauseSubjectless,
        N::SimpleClause,
        [n(N::VerbPhrase)],
    );
    builder.add(RuleTag::ClauseSimple, N::Clause, [n(N::SimpleClause)]);
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
        RuleTag::ClauseConditionalBefore,
        N::Clause,
        [
            l(L::If),
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ClauseConditionalAfterElliptical,
        N::Clause,
        [n(N::Clause), l(L::If), n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::ClauseConditionalAfter,
        N::Clause,
        [n(N::Clause), l(L::AsLongAs), n(N::Clause)],
    );
    builder.add(
        RuleTag::ClauseConditionalAfterIf,
        N::Clause,
        [n(N::Clause), l(L::If), n(N::Clause)],
    );
    builder.add(
        RuleTag::RelativeObject,
        N::RelativeClause,
        [n(N::SimpleClause)],
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
    shape: u64,
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
        | RuleTag::InfinitiveTo => reduce_predicate(tag, children, shape),
        RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::RelativeObject => reduce_simple_clause(tag, children, shape),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseConditionalBefore
        | RuleTag::ClauseConditionalAfterElliptical
        | RuleTag::ClauseConditionalAfter
        | RuleTag::ClauseConditionalAfterIf
        | RuleTag::SentencePeriod
        | RuleTag::SentenceExclamation
        | RuleTag::SentenceQuestion
        | RuleTag::SentenceNone => reduce_composed_clause(tag, children, shape),
        _ => None,
    }
}

fn reduce_predicate(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
    shape: u64,
) -> Option<Reduced> {
    match tag {
        RuleTag::Verb => Some(propagate(children.first()?)),
        RuleTag::VerbPhraseBase => {
            let Features::Verb(slot) = children.first()?.features else {
                return None;
            };
            let form = predicate_form(*slot);
            Some((
                Features::VerbPhrase {
                    form,
                    has_direct_object: false,
                },
                MeaningKey::VerbPhrase { form, shape },
            ))
        }
        RuleTag::VerbPhraseAuxiliary => {
            let MeaningKey::Auxiliary(auxiliary) = children.first()?.meaning else {
                return None;
            };
            let Features::VerbPhrase {
                form: child_form,
                has_direct_object,
            } = children.get(1)?.features
            else {
                return None;
            };
            let form = auxiliary_form(*auxiliary, *child_form)?;
            Some((
                Features::VerbPhrase {
                    form,
                    has_direct_object: *has_direct_object,
                },
                MeaningKey::VerbPhrase { form, shape },
            ))
        }
        RuleTag::VerbPhraseDirectObject => {
            let Features::NounPhrase { pronoun_case, .. } = children.get(1)?.features else {
                return None;
            };
            if *pronoun_case == Some(PronounCase::Subject) {
                return None;
            }
            extend_predicate(children.first()?, shape, true)
        }
        RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity => extend_predicate(children.first()?, shape, false),
        RuleTag::InfinitiveTo => {
            let Features::VerbPhrase {
                form: PredicateForm::Infinitive,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            Some((
                Features::InfinitiveClause,
                MeaningKey::InfinitiveClause { shape },
            ))
        }
        _ => None,
    }
}

fn extend_predicate(
    predicate: &Child<'_, EnglishGrammar<'_, '_>>,
    shape: u64,
    direct_object: bool,
) -> Option<Reduced> {
    let Features::VerbPhrase {
        form,
        has_direct_object,
    } = predicate.features
    else {
        return None;
    };
    if direct_object && *has_direct_object {
        return None;
    }
    Some((
        Features::VerbPhrase {
            form: *form,
            has_direct_object: *has_direct_object || direct_object,
        },
        MeaningKey::VerbPhrase { form: *form, shape },
    ))
}

fn reduce_simple_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
    shape: u64,
) -> Option<Reduced> {
    match tag {
        RuleTag::SimpleClauseSubject => {
            let Features::NounPhrase {
                agreement: Some(subject_agreement),
                pronoun_case,
            } = children.first()?.features
            else {
                return None;
            };
            if *pronoun_case == Some(PronounCase::Object) {
                return None;
            }
            let Features::VerbPhrase {
                form: PredicateForm::Finite(predicate_agreement),
                has_direct_object,
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
                *has_direct_object,
                shape,
            ))
        }
        RuleTag::SimpleClauseSubjectless => {
            let Features::VerbPhrase {
                form,
                has_direct_object,
            } = children.first()?.features
            else {
                return None;
            };
            match form {
                PredicateForm::Imperative => Some(simple_clause_reduction(
                    None,
                    false,
                    true,
                    *has_direct_object,
                    shape,
                )),
                PredicateForm::Finite(agreement) => Some(simple_clause_reduction(
                    *agreement,
                    false,
                    false,
                    *has_direct_object,
                    shape,
                )),
                PredicateForm::Infinitive
                | PredicateForm::PresentParticiple
                | PredicateForm::PastParticiple => None,
            }
        }
        RuleTag::ClauseSimple => {
            let Features::SimpleClause {
                agreement,
                standalone,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some((
                Features::Clause {
                    agreement: *agreement,
                    standalone: *standalone,
                },
                MeaningKey::Clause { shape },
            ))
        }
        RuleTag::ClauseElliptical => Some((
            Features::Clause {
                agreement: None,
                standalone: false,
            },
            MeaningKey::Clause { shape },
        )),
        RuleTag::RelativeObject => {
            let Features::SimpleClause {
                has_subject: true,
                standalone: true,
                has_direct_object: false,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some((
                Features::RelativeClause(RelativeGap::Object),
                MeaningKey::RelativeClause {
                    gap: RelativeGap::Object,
                    shape,
                },
            ))
        }
        _ => None,
    }
}

fn simple_clause_reduction(
    agreement: Option<Agreement>,
    has_subject: bool,
    standalone: bool,
    has_direct_object: bool,
    shape: u64,
) -> Reduced {
    (
        Features::SimpleClause {
            agreement,
            has_subject,
            standalone,
            has_direct_object,
        },
        MeaningKey::SimpleClause { shape },
    )
}

fn reduce_composed_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
    shape: u64,
) -> Option<Reduced> {
    match tag {
        RuleTag::ClauseCoordination | RuleTag::ClauseCoordinationComma => {
            let Features::Clause {
                agreement: first_agreement,
                standalone: true,
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
            Some((
                Features::Clause {
                    agreement: *first_agreement,
                    standalone: true,
                },
                MeaningKey::Clause { shape },
            ))
        }
        RuleTag::ClauseConditionalBefore => {
            conditional_reduction(children.get(1)?, children.get(3)?, shape)
        }
        RuleTag::ClauseConditionalAfterElliptical => {
            let consequence = children.first()?;
            let Features::Clause {
                agreement,
                standalone: true,
            } = consequence.features
            else {
                return None;
            };
            Some((
                Features::Clause {
                    agreement: *agreement,
                    standalone: true,
                },
                MeaningKey::Clause { shape },
            ))
        }
        RuleTag::ClauseConditionalAfter | RuleTag::ClauseConditionalAfterIf => {
            conditional_reduction(children.get(2)?, children.first()?, shape)
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
            Some((Features::Sentence, MeaningKey::Sentence { shape }))
        }
        _ => None,
    }
}

fn conditional_reduction(
    condition: &Child<'_, EnglishGrammar<'_, '_>>,
    consequence: &Child<'_, EnglishGrammar<'_, '_>>,
    shape: u64,
) -> Option<Reduced> {
    let Features::Clause {
        standalone: true, ..
    } = condition.features
    else {
        return None;
    };
    let Features::Clause {
        agreement,
        standalone: true,
    } = consequence.features
    else {
        return None;
    };
    Some((
        Features::Clause {
            agreement: *agreement,
            standalone: true,
        },
        MeaningKey::Clause { shape },
    ))
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
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::RelativeObject => lower_simple_clause(tag, children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseConditionalBefore
        | RuleTag::ClauseConditionalAfterElliptical
        | RuleTag::ClauseConditionalAfter
        | RuleTag::ClauseConditionalAfterIf
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
            VerbDependent::DirectObject(noun_phrase)
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
                subject: Some(Subject::NounPhrase(subject)),
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
            Some(Lowered::Clause(Clause::Simple(simple)))
        }
        RuleTag::ClauseElliptical => {
            let Lowered::AdjectivePhrase(adjective) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Clause(Clause::Elliptical(
                Phrase::AdjectivePhrase(Box::new(adjective)),
            )))
        }
        RuleTag::RelativeObject => {
            let Lowered::SimpleClause(simple) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::RelativeClause(RelativeClause {
                gap: RelativeGap::Object,
                clause: Box::new(Clause::Simple(simple)),
            }))
        }
        _ => None,
    }
}

fn lower_composed_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::ClauseCoordination | RuleTag::ClauseCoordinationComma => {
            lower_coordination(tag, children)
        }
        RuleTag::ClauseConditionalBefore => {
            let Lowered::Subordinator(subordinator) = take(children, 0)? else {
                return None;
            };
            let Lowered::Clause(condition) = take(children, 1)? else {
                return None;
            };
            let Lowered::Clause(consequence) = take(children, 3)? else {
                return None;
            };
            Some(conditional(
                subordinator,
                ConditionalPosition::BeforeConsequence,
                condition,
                consequence,
            ))
        }
        RuleTag::ClauseConditionalAfterElliptical => {
            let Lowered::Clause(consequence) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(subordinator) = take(children, 1)? else {
                return None;
            };
            let Lowered::AdjectivePhrase(adjective) = take(children, 2)? else {
                return None;
            };
            Some(conditional(
                subordinator,
                ConditionalPosition::AfterConsequence,
                Clause::Elliptical(Phrase::AdjectivePhrase(Box::new(adjective))),
                consequence,
            ))
        }
        RuleTag::ClauseConditionalAfter | RuleTag::ClauseConditionalAfterIf => {
            let Lowered::Clause(consequence) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(subordinator) = take(children, 1)? else {
                return None;
            };
            let Lowered::Clause(condition) = take(children, 2)? else {
                return None;
            };
            Some(conditional(
                subordinator,
                ConditionalPosition::AfterConsequence,
                condition,
                consequence,
            ))
        }
        RuleTag::SentencePeriod
        | RuleTag::SentenceExclamation
        | RuleTag::SentenceQuestion
        | RuleTag::SentenceNone => {
            let Lowered::Clause(clause) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Sentence(Sentence {
                clause,
                ending: sentence_ending(tag)?,
            }))
        }
        _ => None,
    }
}

fn lower_coordination(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let Lowered::Clause(first) = take(children, 0)? else {
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
    let coordination = ClauseCoordination {
        conjunction,
        comma,
        clause: Clause::Simple(next),
    };
    let coordinated = match first {
        Clause::Coordinated(mut coordinated) => {
            coordinated.rest.push(coordination);
            coordinated
        }
        first => CoordinatedClause {
            first: Box::new(first),
            rest: vec![coordination],
        },
    };
    Some(Lowered::Clause(Clause::Coordinated(coordinated)))
}

fn conditional(
    subordinator: crate::syntax::Subordinator,
    position: ConditionalPosition,
    condition: Clause,
    consequence: Clause,
) -> Lowered {
    Lowered::Clause(Clause::Conditional(ConditionalClause {
        subordinator,
        position,
        condition: Box::new(condition),
        consequence: Box::new(consequence),
    }))
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
    use crate::syntax::Clause;
    use crate::syntax::Determiner;
    use crate::syntax::NominalComplement;
    use crate::syntax::NominalModifier;
    use crate::syntax::NounPhrase;
    use crate::syntax::OracleText;
    use crate::syntax::Paragraph;
    use crate::syntax::Phrase;
    use crate::syntax::Sentence;
    use crate::syntax::Subject;
    use crate::syntax::VerbDependent;
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

    const FIXTURES: [&str; 11] = [
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
        "Target creature you control fights target creature you don't control.",
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
        let plural = simple(plural_parse.sentence().unwrap());
        assert!(matches!(
            plural.subject,
            Some(Subject::NounPhrase(NounPhrase::Nominal(ref nominal)))
                if matches!(nominal.head, NounInstance::Plural(Noun::Word(Vocab::Spell)))
        ));
        assert_eq!(
            plural.predicate.verb.slot,
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Plural,
            }
        );
        assert!(matches!(
            plural.predicate.verb.verb,
            Verb::Word(Vocab::Cost)
        ));

        let singular_parse = parse("This creature costs {1} less to cast.");
        let singular = simple(singular_parse.sentence().unwrap());
        assert_eq!(
            singular.predicate.verb.slot,
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Singular,
            }
        );
    }

    #[test]
    fn participle_position_distinguishes_modifier_from_passive_predicate() {
        let parsed = parse("Prevented damage is dealt to that creature's controller instead.");
        let clause = simple(parsed.sentence().unwrap());
        let Some(Subject::NounPhrase(NounPhrase::Nominal(subject))) = &clause.subject else {
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
            clause.predicate.auxiliaries.as_slice(),
            [auxiliary]
                if auxiliary.auxiliary == Auxiliary::Be
                    && auxiliary.inflection == (AuxiliaryInflection::Present {
                        person: Person::Third,
                        number: Number::Singular,
                    })
        ));
        assert!(matches!(
            clause.predicate.verb.verb,
            Verb::Word(Vocab::Deal)
        ));
        assert_eq!(clause.predicate.verb.slot, VerbSlot::PastParticiple);
    }

    #[test]
    fn target_and_relative_clauses_keep_their_nominal_roles() {
        let target_parse = parse("Target creature gets +1/+1 until end of turn.");
        let target = simple(target_parse.sentence().unwrap());
        let Some(Subject::NounPhrase(NounPhrase::Nominal(target))) = &target.subject else {
            panic!("expected target nominal subject");
        };
        assert_eq!(target.determiner, Some(Determiner::Target(None)));

        let fight_parse =
            parse("Target creature you control fights target creature you don't control.");
        let fight = simple(fight_parse.sentence().unwrap());
        let Some(Subject::NounPhrase(NounPhrase::Nominal(subject))) = &fight.subject else {
            panic!("expected controlled target subject");
        };
        assert!(matches!(
            subject.complements.as_slice(),
            [NominalComplement::Relative(relative)]
                if relative.gap == crate::syntax::RelativeGap::Object
        ));
        let [VerbDependent::DirectObject(NounPhrase::Nominal(object))] =
            fight.predicate.dependents.as_slice()
        else {
            panic!(
                "expected one direct-object nominal, got {:#?}",
                fight.predicate.dependents
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
        let Clause::Coordinated(coordination) = &sentence.sentence().unwrap().clause else {
            panic!("expected coordinated predicates");
        };
        let Clause::Simple(first) = coordination.first.as_ref() else {
            panic!("expected simple first predicate");
        };
        assert!(first.predicate.dependents.iter().any(|dependent| matches!(
            dependent,
            VerbDependent::Statistic(Phrase::PowerToughness(_))
        )));
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
            .with_catalog(CatalogKind::CardType, ["Creature"])
    }

    fn simple(sentence: &Sentence) -> &crate::syntax::SimpleClause {
        let Clause::Simple(simple) = &sentence.clause else {
            panic!("expected a simple clause, got {:?}", sentence.clause);
        };
        simple
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
