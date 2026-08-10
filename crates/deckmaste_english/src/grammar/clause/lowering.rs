use super::Agreement;
use super::AttachedPredicate;
use super::AttachmentPosition;
use super::Auxiliary;
use super::AuxiliaryInflection;
use super::AuxiliaryInstance;
use super::BareNominalAdjunct;
use super::Clause;
use super::ClauseCoordination;
use super::CoordinatedClauseMember;
use super::CoordinatedIndependentClause;
use super::Coordination;
use super::CoordinationJunction;
use super::CopularComplement;
use super::CopularRemainder;
use super::DeonticPredicate;
use super::GapState;
use super::IndependentClause;
use super::Lowered;
use super::NounPhrase;
use super::Phrase;
use super::Predicate;
use super::PredicateAdjunct;
use super::PredicateExpression;
use super::PredicateForm;
use super::PredicateHead;
use super::RelativeBody;
use super::RelativeClause;
use super::RelativeMarker;
use super::RuleTag;
use super::SimpleClause;
use super::Subject;
use super::VerbDependent;
use super::VerbPhrase;
use super::VerbSlot;
use super::reduction::auxiliary_form;
use super::take;
use crate::constructions::predicate::FinishedPredicate;
use crate::features::Conjunction;
use crate::grammar::reduction::predicate_form;
use crate::syntax::InfinitiveClause;
use crate::syntax::ObjectGapPredicate;

pub(in crate::grammar) fn lower_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::VerbPhraseCoordinatedAdjective => lower_predicate(tag, children),
        RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeSubjectDistributiveEach
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::CopularRemainderCoordinatedAdjective
        | RuleTag::RelativeContractedCopularCoordinatedAdjective => {
            lower_simple_clause(tag, children)
        }
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseCoordinationCopularNounPrepositional
        | RuleTag::ClauseCoordinationCopularNounPrepositionalComma
        | RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic => {
            lower_composed_clause(tag, children)
        }
        _ => None,
    }
}

pub(super) fn lower_predicate(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::VerbPhraseCoordinatedAdjective => lower_predicate_dependent(tag, children),
        _ => None,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "one arm per predicate-dependent rule tag is intentionally verbose"
)]
pub(super) fn lower_predicate_dependent(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let Lowered::VerbPhrase(mut predicate) = take(children, 0)? else {
        return None;
    };
    let dependent = match tag {
        RuleTag::VerbPhraseCoordinatedAdjective => {
            let Lowered::CoordinatedModifier(coordinated) = take(children, 1)? else {
                return None;
            };
            VerbDependent::CoordinatedAdjective(coordinated_modifier_as_adjectives(coordinated)?)
        }
        _ => return None,
    };
    predicate.dependents.push(dependent);
    Some(Lowered::VerbPhrase(predicate))
}

#[allow(clippy::too_many_lines, reason = "lowering has many grammar variants")]
pub(super) fn lower_simple_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::CopularRemainderCoordinatedAdjective => lower_copular_remainder(children),
        RuleTag::RelativeObject => {
            let Lowered::NounPhrase(subject) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 1)? else {
                return None;
            };
            let FinishedPredicate {
                modal, predicate, ..
            } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            let Predicate::Intransitive(predicate) = predicate else {
                return None;
            };
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::Zero,
                gap: GapState::Object,
                body: RelativeBody::ObjectGap {
                    subject: Subject(subject),
                    predicate: ObjectGapPredicate {
                        head: predicate.head,
                        kind: crate::syntax::ObjectGap,
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
            let FinishedPredicate {
                modal, predicate, ..
            } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            let Predicate::Intransitive(predicate) = predicate else {
                return None;
            };
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::Zero,
                gap: GapState::Object,
                body: RelativeBody::ObjectGap {
                    subject: subject_auxiliary.subject,
                    predicate: ObjectGapPredicate {
                        head: predicate.head,
                        kind: crate::syntax::ObjectGap,
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
            let FinishedPredicate {
                modal, predicate, ..
            } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::That,
                gap: GapState::Subject,
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
            let FinishedPredicate {
                modal,
                predicate,
                elided,
            } = finish_predicate(predicate)?;
            // Only VP-ellipsis under the modal ("creature that can't") drops the
            // predicate. A modal with a real, complement-less verb ("damage that
            // would be", "creature that would die") keeps it — nulling those on
            // the loose "intransitive with empty elements" test silently ate the
            // verb (e.g. the "be" of "would be dealt").
            let predicate = match modal {
                Some(modal) => Predicate::Deontic(DeonticPredicate {
                    modal,
                    inner: if elided { None } else { Some(Box::new(predicate)) },
                }),
                None => predicate,
            };
            let body = RelativeBody::SubjectGap(predicate);
            Some(Lowered::RelativeClause(RelativeClause {
                marker,
                gap: GapState::Subject,
                body,
            }))
        }
        RuleTag::RelativeSubjectDistributiveEach => {
            let Lowered::RelativeMarker(marker) = take(children, 0)? else {
                return None;
            };
            // `each` (index 1) is the floating quantifier — discarded here
            // and carried as `PredicateHead::distributive_each` instead.
            let Lowered::VerbPhrase(mut predicate) = take(children, 2)? else {
                return None;
            };
            predicate.distributive_each = true;
            let FinishedPredicate {
                modal,
                predicate,
                elided: _,
            } = finish_predicate(predicate)?;
            // The reduce-time gate requires `Finite(Some(agreement))`, which
            // a modal auxiliary never produces here — a modal reaching this
            // arm would mean the gate was bypassed.
            if modal.is_some() {
                return None;
            }
            Some(Lowered::RelativeClause(RelativeClause {
                marker,
                gap: GapState::Subject,
                body: RelativeBody::SubjectGap(predicate),
            }))
        }
        RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::RelativeContractedCopularCoordinatedAdjective => {
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
                RuleTag::RelativeContractedCopularCoordinatedAdjective => {
                    let Lowered::CoordinatedModifier(coordinated) = take(children, 1)? else {
                        return None;
                    };
                    crate::syntax::CopularComplement::CoordinatedAdjective(
                        coordinated_modifier_as_adjectives(coordinated)?,
                    )
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
                gap: GapState::Subject,
                body: RelativeBody::SubjectGap(Predicate::Copular(
                    crate::syntax::CopularPredicate {
                        negated: false,
                        copula: crate::syntax::Copula {
                            auxiliary: subject_auxiliary.auxiliary,
                            contracted_with_subject: crate::features::Contraction::Contracted,
                        },
                        distributive_each: false,
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

pub(super) fn lower_copular_remainder(children: &mut [Lowered]) -> Option<Lowered> {
    let Lowered::CoordinatedModifier(coordinated) = take(children, 0)? else {
        return None;
    };
    Some(Lowered::CopularRemainder(CopularRemainder {
        negated: false,
        distributive_each: false,
        precomplement_adverbs: Vec::new(),
        complement: CopularComplement::CoordinatedAdjective(coordinated_modifier_as_adjectives(
            coordinated,
        )?),
        adjuncts: Vec::new(),
    }))
}

#[allow(
    clippy::too_many_lines,
    reason = "one arm per composed-clause rule tag is intentionally verbose"
)]
pub(super) fn lower_composed_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::ClauseCoordinationCopularNounPrepositional
        | RuleTag::ClauseCoordinationCopularNounPrepositionalComma
        | RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic => {
            lower_shared_copular_coordination(tag, children)
        }
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic => lower_coordination(tag, children),
        _ => None,
    }
}

pub(super) fn lower_coordination(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
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
            match conjunction {
                Conjunction::And | Conjunction::Or | Conjunction::Then | Conjunction::AndOr => {
                    Some(conjunction)
                }
                Conjunction::Plus => return None,
            }
        }
        None => None,
    };
    let Lowered::SimpleClause(next) = take(children, clause_index)? else {
        return None;
    };
    let junction = CoordinationJunction {
        conjunction,
        comma: crate::features::Comma::from(comma),
    };
    let coordinated = if next.subject.is_some() {
        let next = finish_simple_clause(next)?;
        append_complete_clause(first, junction, next)
    } else {
        let FinishedPredicate {
            modal,
            mut predicate,
            elided,
        } = finish_predicate(next.predicate)?;
        if let Some(modal) = modal {
            predicate = Predicate::Deontic(DeonticPredicate {
                modal,
                inner: if elided { None } else { Some(Box::new(predicate)) },
            });
        } else if let Some(inflection) = finite_inflection_of_clause(&first) {
            apply_finite_inflection(&mut predicate, inflection);
        }
        if starts_new_clause_group(&first, &junction) || !accepts_shared_predicate(&first) {
            append_subjectless_clause(first, &junction, predicate)?
        } else {
            append_shared_predicate(first, junction, predicate)?
        }
    };
    Some(Lowered::Clause(Clause::Independent(coordinated)))
}

fn lower_shared_copular_coordination(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let Lowered::Clause(Clause::Independent(first)) = take(children, 0)? else {
        return None;
    };
    let (conjunction_index, copula_index, noun_phrase_index, preposition_index, comma) = match tag {
        RuleTag::ClauseCoordinationCopularNounPrepositional => (Some(1), 2, 3, 4, false),
        RuleTag::ClauseCoordinationCopularNounPrepositionalComma => (Some(2), 3, 4, 5, true),
        RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic => (None, 2, 3, 4, true),
        _ => return None,
    };
    let conjunction = match conjunction_index {
        Some(index) => {
            let Lowered::Conjunction(Conjunction::And) = take(children, index)? else {
                return None;
            };
            Some(Conjunction::And)
        }
        None => None,
    };
    let Lowered::Auxiliary(auxiliary) = take(children, copula_index)? else {
        return None;
    };
    let copula_agreement = match auxiliary {
        AuxiliaryInstance {
            auxiliary: Auxiliary::Be,
            inflection:
                AuxiliaryInflection::Present { person, number }
                | AuxiliaryInflection::Past { person, number },
            ..
        } => Agreement { person, number },
        _ => return None,
    };
    if let Some(host_inflection) = finite_inflection_of_clause(&first) {
        let host_agreement = match host_inflection {
            FiniteInflection::Present(agreement) | FiniteInflection::Past(agreement) => agreement,
        };
        if host_agreement != copula_agreement {
            return None;
        }
    }
    let Lowered::NounPhrase(complement) = take(children, noun_phrase_index)? else {
        return None;
    };
    let Lowered::PrepositionalPhrase(preposition) = take(children, preposition_index)? else {
        return None;
    };
    if preposition.head().preposition != crate::syntax::Preposition::In {
        return None;
    }
    let predicate = Predicate::Copular(crate::syntax::CopularPredicate {
        copula: crate::syntax::Copula {
            auxiliary,
            contracted_with_subject: crate::features::Contraction::Full,
        },
        negated: false,
        distributive_each: false,
        precomplement_adverbs: Vec::new(),
        complement: CopularComplement::NounPhrase(complement),
        adjuncts: vec![PredicateAdjunct::Prepositional(preposition)],
    });
    let coordinated = append_shared_predicate(
        first,
        CoordinationJunction {
            conjunction,
            comma: crate::features::Comma::from(comma),
        },
        predicate,
    )?;
    Some(Lowered::Clause(Clause::Independent(coordinated)))
}

#[derive(Clone, Copy)]
pub(super) enum FiniteInflection {
    Present(Agreement),
    Past(Agreement),
}

impl FiniteInflection {
    const fn verb_slot(self) -> VerbSlot {
        match self {
            Self::Present(Agreement { person, number }) => VerbSlot::Present { person, number },
            Self::Past(Agreement { person, number }) => VerbSlot::Past { person, number },
        }
    }

    const fn auxiliary_inflection(self) -> AuxiliaryInflection {
        match self {
            Self::Present(Agreement { person, number }) => {
                AuxiliaryInflection::Present { person, number }
            }
            Self::Past(Agreement { person, number }) => {
                AuxiliaryInflection::Past { person, number }
            }
        }
    }
}

pub(super) fn finite_inflection_of_clause(clause: &IndependentClause) -> Option<FiniteInflection> {
    match clause {
        IndependentClause::Transitive(_, predicate) => finite_inflection_of_head(&predicate.head),
        IndependentClause::Intransitive(_, predicate) => finite_inflection_of_head(&predicate.head),
        IndependentClause::Passive(_, predicate) => finite_inflection_of_head(&predicate.head),
        IndependentClause::Copular(_, predicate) => {
            finite_inflection_of_auxiliary(predicate.copula.auxiliary)
        }
        IndependentClause::Proform(_, predicate) => {
            finite_inflection_of_auxiliary(predicate.auxiliary)
        }
        IndependentClause::Deontic(..)
        | IndependentClause::Imperative(_)
        | IndependentClause::Existential(_) => None,
        IndependentClause::Complex(complex) => finite_inflection_of_clause(&complex.matrix),
        IndependentClause::Predicated(_, expression) => finite_inflection_of_expression(expression),
        IndependentClause::Coordinated(coordination) => {
            coordination
                .rest
                .last()
                .and_then(|continuation| match &continuation.member {
                    CoordinatedClauseMember::Independent(clause) => {
                        finite_inflection_of_clause(clause)
                    }
                })
        }
    }
}

fn finite_inflection_of_expression(expression: &PredicateExpression) -> Option<FiniteInflection> {
    match expression {
        PredicateExpression::Simple(predicate) => finite_inflection_of_predicate(predicate),
        PredicateExpression::Coordinated(coordination) => coordination
            .conjuncts()
            .first()
            .and_then(finite_inflection_of_expression),
    }
}

pub(super) fn finite_inflection_of_predicate(predicate: &Predicate) -> Option<FiniteInflection> {
    match predicate {
        Predicate::Transitive(predicate) => finite_inflection_of_head(&predicate.head),
        Predicate::Intransitive(predicate) => finite_inflection_of_head(&predicate.head),
        Predicate::Passive(predicate) => finite_inflection_of_head(&predicate.head),
        Predicate::Copular(predicate) => finite_inflection_of_auxiliary(predicate.copula.auxiliary),
        Predicate::Proform(predicate) => finite_inflection_of_auxiliary(predicate.auxiliary),
        Predicate::Deontic(_) => None,
        Predicate::Attached(predicate) => finite_inflection_of_predicate(&predicate.predicate),
    }
}

pub(super) fn finite_inflection_of_head(head: &PredicateHead) -> Option<FiniteInflection> {
    if let Some(auxiliary) = head.auxiliaries.first().copied() {
        finite_inflection_of_auxiliary(auxiliary)
    } else {
        match head.verb.slot {
            VerbSlot::Present { person, number } => {
                Some(FiniteInflection::Present(Agreement { person, number }))
            }
            VerbSlot::Past { person, number } => {
                Some(FiniteInflection::Past(Agreement { person, number }))
            }
            _ => None,
        }
    }
}

pub(super) const fn finite_inflection_of_auxiliary(
    auxiliary: AuxiliaryInstance,
) -> Option<FiniteInflection> {
    match auxiliary.inflection {
        AuxiliaryInflection::Present { person, number } => {
            Some(FiniteInflection::Present(Agreement { person, number }))
        }
        AuxiliaryInflection::Past { person, number } => {
            Some(FiniteInflection::Past(Agreement { person, number }))
        }
        _ => None,
    }
}

pub(super) fn apply_finite_inflection(predicate: &mut Predicate, inflection: FiniteInflection) {
    match predicate {
        Predicate::Transitive(predicate) => {
            apply_finite_inflection_to_head(&mut predicate.head, inflection);
        }
        Predicate::Intransitive(predicate) => {
            apply_finite_inflection_to_head(&mut predicate.head, inflection);
        }
        Predicate::Passive(predicate) => {
            apply_finite_inflection_to_head(&mut predicate.head, inflection);
        }
        Predicate::Copular(predicate) => {
            apply_finite_inflection_to_auxiliary(&mut predicate.copula.auxiliary, inflection);
        }
        Predicate::Proform(predicate) => {
            apply_finite_inflection_to_auxiliary(&mut predicate.auxiliary, inflection);
        }
        // The modal itself is uninflected and licenses the inner infinitive.
        Predicate::Deontic(_) => {}
        Predicate::Attached(predicate) => {
            apply_finite_inflection(&mut predicate.predicate, inflection);
        }
    }
}

pub(super) fn apply_finite_inflection_to_head(
    head: &mut PredicateHead,
    inflection: FiniteInflection,
) {
    if let Some(auxiliary) = head.auxiliaries.first_mut() {
        apply_finite_inflection_to_auxiliary(auxiliary, inflection);
    } else if head.verb.slot == VerbSlot::Infinitive {
        let vocabulary = crate::word::Vocabulary::new();
        let mut finite = head.verb.clone();
        finite.slot = inflection.verb_slot();
        let source_spelling = vocabulary.render_verb_instance(&head.verb);
        if source_spelling.is_some() && source_spelling == vocabulary.render_verb_instance(&finite)
        {
            head.verb = finite;
        }
    }
}

/// Records agreement only when the finite form is syncretic with the parsed
/// base form. The grammar intentionally admits some subjectless continuations
/// whose apparent predicate head is really a noun or an imperative; changing
/// their surface (`target` -> `targets`, `return` -> `returned`) would turn a
/// representation-only normalization into a parse-selection change.
pub(super) fn apply_finite_inflection_to_auxiliary(
    auxiliary: &mut AuxiliaryInstance,
    inflection: FiniteInflection,
) {
    if auxiliary.inflection == AuxiliaryInflection::Base {
        let vocabulary = crate::word::Vocabulary::new();
        let mut finite = *auxiliary;
        finite.inflection = inflection.auxiliary_inflection();
        let source_spelling = vocabulary.render_auxiliary(*auxiliary);
        if source_spelling.is_some() && source_spelling == vocabulary.render_auxiliary(finite) {
            *auxiliary = finite;
        }
    }
}

/// Adds a subjectless predicate to the rightmost finite clause that supplies
/// its subject. This turns mixed `S₁ P₁ and S₂ P₂ and P₃` into coordination of
/// two complete clauses, with `P₂ and P₃` coordinated under `S₂`.
pub(super) fn append_shared_predicate(
    clause: IndependentClause,
    junction: CoordinationJunction,
    predicate: Predicate,
) -> Option<IndependentClause> {
    match clause {
        IndependentClause::Transitive(subject, first) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                PredicateExpression::Simple(Predicate::Transitive(first)),
                junction,
                PredicateExpression::Simple(predicate),
            )),
        )),
        IndependentClause::Intransitive(subject, first) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                PredicateExpression::Simple(Predicate::Intransitive(first)),
                junction,
                PredicateExpression::Simple(predicate),
            )),
        )),
        IndependentClause::Copular(subject, first) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                PredicateExpression::Simple(Predicate::Copular(first)),
                junction,
                PredicateExpression::Simple(predicate),
            )),
        )),
        IndependentClause::Passive(subject, first) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                PredicateExpression::Simple(Predicate::Passive(first)),
                junction,
                PredicateExpression::Simple(predicate),
            )),
        )),
        IndependentClause::Proform(subject, first) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                PredicateExpression::Simple(Predicate::Proform(first)),
                junction,
                PredicateExpression::Simple(predicate),
            )),
        )),
        IndependentClause::Deontic(subject, modal, inner) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                PredicateExpression::Simple(Predicate::Deontic(DeonticPredicate {
                    modal,
                    inner: inner.map(Box::new),
                })),
                junction,
                PredicateExpression::Simple(predicate),
            )),
        )),
        IndependentClause::Imperative(first) => Some(IndependentClause::Predicated(
            None,
            PredicateExpression::Coordinated(Coordination::new(
                PredicateExpression::Simple(first),
                junction,
                PredicateExpression::Simple(predicate),
            )),
        )),
        IndependentClause::Predicated(subject, expression) => {
            let expression = push_predicate_expression(expression, junction, predicate);
            Some(IndependentClause::Predicated(subject, expression))
        }
        IndependentClause::Complex(mut complex) => {
            let after_matrix: Vec<_> = complex
                .attachments
                .iter()
                .filter(|attachment| attachment.position == AttachmentPosition::AfterMatrix)
                .cloned()
                .collect();
            if !after_matrix.is_empty()
                && let Some((subject, PredicateExpression::Simple(first))) =
                    into_predicate_expression(*complex.matrix.clone())
            {
                let first = Predicate::Attached(AttachedPredicate {
                    predicate: Box::new(first),
                    attachments: after_matrix,
                });
                let predicated = IndependentClause::Predicated(
                    subject,
                    PredicateExpression::Coordinated(Coordination::new(
                        PredicateExpression::Simple(first),
                        junction,
                        PredicateExpression::Simple(predicate),
                    )),
                );
                complex
                    .attachments
                    .retain(|attachment| attachment.position == AttachmentPosition::BeforeMatrix);
                return if complex.attachments.is_empty() {
                    Some(predicated)
                } else {
                    complex.matrix = Box::new(predicated);
                    Some(IndependentClause::Complex(complex))
                };
            }
            let matrix = append_shared_predicate(*complex.matrix, junction, predicate)?;
            complex.matrix = Box::new(matrix);
            Some(IndependentClause::Complex(complex))
        }
        IndependentClause::Coordinated(mut coordinated) => {
            let CoordinatedClauseMember::Independent(last) =
                &mut coordinated.rest.last_mut()?.member;
            if !accepts_shared_predicate(last) {
                return None;
            }
            let replacement = append_shared_predicate((**last).clone(), junction, predicate)?;
            **last = replacement;
            Some(IndependentClause::Coordinated(coordinated))
        }
        IndependentClause::Existential(_) => None,
    }
}

/// Adds one predicate without flattening a changed connective into the
/// existing run. Predicate expressions are recursive specifically so
/// `(attack or block) and has ...` keeps the `or` constituent as the first
/// member of the outer `and` group. Asyndetic Oxford members remain in the
/// open run until its overt closing connective arrives.
fn push_predicate_expression(
    expression: PredicateExpression,
    junction: CoordinationJunction,
    predicate: Predicate,
) -> PredicateExpression {
    match expression {
        PredicateExpression::Coordinated(mut coordinated)
            if !connective_changes(
                coordinated
                    .junctions()
                    .iter()
                    .rev()
                    .find_map(|junction| junction.conjunction),
                junction.conjunction,
            ) =>
        {
            coordinated.push(junction, PredicateExpression::Simple(predicate));
            PredicateExpression::Coordinated(coordinated)
        }
        expression => PredicateExpression::Coordinated(Coordination::new(
            expression,
            junction,
            PredicateExpression::Simple(predicate),
        )),
    }
}

pub(super) fn into_predicate_expression(
    clause: IndependentClause,
) -> Option<(Option<Subject>, PredicateExpression)> {
    match clause {
        IndependentClause::Transitive(subject, predicate) => Some((
            Some(subject),
            PredicateExpression::Simple(Predicate::Transitive(predicate)),
        )),
        IndependentClause::Intransitive(subject, predicate) => Some((
            Some(subject),
            PredicateExpression::Simple(Predicate::Intransitive(predicate)),
        )),
        IndependentClause::Copular(subject, predicate) => Some((
            Some(subject),
            PredicateExpression::Simple(Predicate::Copular(predicate)),
        )),
        IndependentClause::Passive(subject, predicate) => Some((
            Some(subject),
            PredicateExpression::Simple(Predicate::Passive(predicate)),
        )),
        IndependentClause::Proform(subject, predicate) => Some((
            Some(subject),
            PredicateExpression::Simple(Predicate::Proform(predicate)),
        )),
        IndependentClause::Deontic(subject, modal, inner) => Some((
            Some(subject),
            PredicateExpression::Simple(Predicate::Deontic(DeonticPredicate {
                modal,
                inner: inner.map(Box::new),
            })),
        )),
        IndependentClause::Imperative(predicate) => {
            Some((None, PredicateExpression::Simple(predicate)))
        }
        IndependentClause::Predicated(subject, expression) => Some((subject, expression)),
        _ => None,
    }
}

/// Keeps a subjectless imperative as a complete clause member when there is no
/// finite subject to share, or when a changed overt connective starts a new
/// clause-level group (`C1 and C2, or P3`). The latter preserves ordinary
/// `and`/`or` grouping instead of incorrectly making `P3` a predicate of C2's
/// subject.
pub(super) fn append_subjectless_clause(
    clause: IndependentClause,
    junction: &CoordinationJunction,
    predicate: Predicate,
) -> Option<IndependentClause> {
    if matches!(predicate, Predicate::Deontic(_)) {
        return None;
    }
    Some(append_complete_clause(
        clause,
        junction.clone(),
        IndependentClause::Imperative(predicate),
    ))
}

/// Adds a complete clause while preserving punctuation-signaled grouping.
/// A changed connective after a comma starts a new outer group (`A and B, or
/// C`). Without that comma it joins the rightmost clause (`A, or B and C`),
/// which is also where an immediately following subjectless predicate finds
/// its finite subject.
fn append_complete_clause(
    first: IndependentClause,
    junction: CoordinationJunction,
    next: IndependentClause,
) -> IndependentClause {
    let continuation = ClauseCoordination {
        conjunction: junction.conjunction,
        comma: junction.comma,
        member: CoordinatedClauseMember::Independent(Box::new(next)),
    };
    match first {
        IndependentClause::Coordinated(mut coordinated) => {
            let changed = connective_changes(
                coordinated
                    .rest
                    .iter()
                    .rev()
                    .find_map(|member| member.conjunction),
                junction.conjunction,
            );
            if changed && junction.comma.is_present() {
                return IndependentClause::Coordinated(CoordinatedIndependentClause {
                    first: Box::new(IndependentClause::Coordinated(coordinated)),
                    rest: vec![continuation],
                });
            }
            if changed {
                let CoordinatedClauseMember::Independent(last) = &mut coordinated
                    .rest
                    .last_mut()
                    .expect("a coordinated clause has a continuation")
                    .member;
                let CoordinatedClauseMember::Independent(next) = continuation.member;
                **last = append_complete_clause((**last).clone(), junction, *next);
            } else {
                coordinated.rest.push(continuation);
            }
            IndependentClause::Coordinated(coordinated)
        }
        first => IndependentClause::Coordinated(CoordinatedIndependentClause {
            first: Box::new(first),
            rest: vec![continuation],
        }),
    }
}

fn connective_changes(previous: Option<Conjunction>, next: Option<Conjunction>) -> bool {
    matches!((previous, next), (Some(previous), Some(next)) if previous != next)
}

pub(super) fn starts_new_clause_group(
    clause: &IndependentClause,
    junction: &CoordinationJunction,
) -> bool {
    let IndependentClause::Coordinated(coordinated) = clause else {
        return false;
    };
    junction.comma.is_present()
        && coordinated.rest.last().is_some_and(|previous| {
            matches!(
                (previous.conjunction, junction.conjunction),
                (Some(previous), Some(next)) if previous != next
            )
        })
}

pub(super) fn accepts_shared_predicate(clause: &IndependentClause) -> bool {
    match clause {
        IndependentClause::Transitive(..)
        | IndependentClause::Intransitive(..)
        | IndependentClause::Copular(..)
        | IndependentClause::Passive(..)
        | IndependentClause::Proform(..)
        | IndependentClause::Deontic(..)
        | IndependentClause::Imperative(..)
        | IndependentClause::Predicated(..) => true,
        IndependentClause::Complex(complex) => accepts_shared_predicate(&complex.matrix),
        IndependentClause::Coordinated(coordination) => {
            coordination
                .rest
                .last()
                .is_some_and(|continuation| match &continuation.member {
                    CoordinatedClauseMember::Independent(clause) => {
                        accepts_shared_predicate(clause)
                    }
                })
        }
        IndependentClause::Existential(_) => false,
    }
}

pub(crate) fn finish_simple_clause(simple: SimpleClause) -> Option<IndependentClause> {
    let imperative = simple.subject.is_none() && simple.predicate.verb.slot == VerbSlot::Imperative;
    let subject = simple.subject;
    let FinishedPredicate {
        modal,
        predicate,
        elided,
    } = finish_predicate(simple.predicate)?;
    match (subject, modal, imperative) {
        (None, None, true) => Some(IndependentClause::Imperative(predicate)),
        (Some(subject), Some(modal), false) => Some(IndependentClause::Deontic(
            subject,
            modal,
            if elided { None } else { Some(predicate) },
        )),
        (Some(subject), None, false) => Some(independent_with_subject(subject, predicate)),
        _ => None,
    }
}

pub(super) fn independent_with_subject(
    subject: Subject,
    predicate: Predicate,
) -> IndependentClause {
    match predicate {
        Predicate::Transitive(predicate) => IndependentClause::Transitive(subject, predicate),
        Predicate::Intransitive(predicate) => IndependentClause::Intransitive(subject, predicate),
        Predicate::Copular(predicate) => IndependentClause::Copular(subject, predicate),
        Predicate::Passive(predicate) => IndependentClause::Passive(subject, predicate),
        Predicate::Proform(predicate) => IndependentClause::Proform(subject, predicate),
        Predicate::Deontic(predicate) => IndependentClause::Deontic(
            subject,
            predicate.modal,
            predicate.inner.map(|inner| *inner),
        ),
        predicate @ Predicate::Attached(_) => {
            IndependentClause::Predicated(Some(subject), PredicateExpression::Simple(predicate))
        }
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "sentence assembly is intentionally verbose"
)]
/// Converts a parsed
/// [`CoordinatedModifier`](crate::syntax::CoordinatedModifier)
/// into a predicative
/// [`CoordinatedAdjectivePhrase`](crate::syntax::CoordinatedAdjectivePhrase),
/// keeping only positive attributive-adjective conjuncts. Returns `None` when
/// any conjunct is a noun, a `non-` negated modifier, a quantity, or a
/// power/toughness — none of those predicate coordinately in a copular
/// position, so rejecting them keeps the attributive-only shapes out of the
/// predicative slot.
pub(in crate::grammar) fn coordinated_modifier_as_adjectives(
    modifier: crate::syntax::CoordinatedModifier,
) -> Option<crate::syntax::CoordinatedAdjectivePhrase> {
    let first = modifier_as_predicative_adjective(*modifier.first)?;
    let mut rest = Vec::with_capacity(modifier.rest.len());
    for coordination in modifier.rest {
        rest.push(crate::syntax::AdjectivePhraseCoordination {
            conjunction: coordination.conjunction,
            phrase: modifier_as_predicative_adjective(coordination.modifier)?,
        });
    }
    Some(crate::syntax::CoordinatedAdjectivePhrase {
        first: Box::new(first),
        rest,
    })
}

pub(super) fn modifier_as_predicative_adjective(
    modifier: crate::syntax::NominalModifier,
) -> Option<crate::syntax::AdjectivePhrase> {
    match modifier {
        crate::syntax::NominalModifier::Adjective {
            polarity: crate::syntax::Polarity::Positive,
            phrase,
        } => Some(phrase),
        _ => None,
    }
}

pub(super) fn finish_predicate(phrase: VerbPhrase) -> Option<FinishedPredicate> {
    crate::constructions::predicate::project_public_predicate(phrase).ok()
}
pub(in crate::grammar) fn finish_reduced_recipient_passive(
    phrase: VerbPhrase,
) -> Option<crate::syntax::TransitivePredicate> {
    if !phrase.auxiliaries.is_empty()
        || phrase.verb.slot != VerbSlot::PastParticiple
        || !phrase.frame.is_recipient_passive()
    {
        return None;
    }
    let FinishedPredicate {
        modal: None,
        predicate: Predicate::Transitive(predicate),
        elided: false,
    } = finish_predicate(phrase)?
    else {
        return None;
    };
    Some(predicate)
}

pub(in crate::grammar) fn finish_infinitive(
    clause: InfinitiveClause,
) -> Option<crate::syntax::InfinitiveClause> {
    crate::constructions::nonfinite::is_valid_infinitive(&clause).then_some(clause)
}

pub(crate) fn lowered_nominal_adjunct_kind(
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

pub(super) fn lowered_predicate_form(predicate: &VerbPhrase) -> Option<(PredicateForm, bool)> {
    let mut form = predicate_form(predicate.verb.slot);
    let mut passive = false;
    for auxiliary in predicate.auxiliaries.iter().rev() {
        passive |= auxiliary.auxiliary == Auxiliary::Be && form == PredicateForm::PastParticiple;
        form = auxiliary_form(*auxiliary, form)?;
    }
    Some((form, passive))
}

pub(super) fn nominal_adjunct_kind(phrase: &NounPhrase) -> Option<BareNominalAdjunct> {
    let NounPhrase::Nominal(nominal) = phrase else {
        return None;
    };
    nominal.head().noun().bare_nominal_adjunct()
}
