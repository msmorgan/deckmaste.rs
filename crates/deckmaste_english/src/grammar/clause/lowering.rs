use super::AbilityObject;
use super::Agreement;
use super::AttachedPredicate;
use super::AttachmentPosition;
use super::Auxiliary;
use super::AuxiliaryInflection;
use super::AuxiliaryInstance;
use super::BareNominalAdjunct;
use super::Clause;
use super::ClauseAttachment;
use super::ClauseAttachmentKind;
use super::ClauseCoordination;
use super::ComplexClause;
use super::CoordinatedClauseMember;
use super::CoordinatedIndependentClause;
use super::CoordinatedPredicateObject;
use super::Coordination;
use super::CoordinationJunction;
use super::CopularComplement;
use super::CopularRemainder;
use super::DeonticPredicate;
use super::DependentAttachment;
use super::DependentClause;
use super::EllipticalClause;
use super::ExceptionConjunct;
use super::ExceptionRider;
use super::IndependentClause;
use super::InfinitiveClause;
use super::InfinitiveMarker;
use super::Lowered;
use super::Modal;
use super::NounInstance;
use super::NounPhrase;
use super::PassivePredicate;
use super::Phrase;
use super::Predicate;
use super::PredicateAdjunct;
use super::PredicateComplement;
use super::PredicateElement;
use super::PredicateExpression;
use super::PredicateForm;
use super::PredicateHead;
use super::PredicateObject;
use super::PredicateObjectCoordination;
use super::PrepositionalRole;
use super::PreverbModifier;
use super::ProPredicate;
use super::Quantity;
use super::RelativeBody;
use super::RelativeClause;
use super::RelativeGap;
use super::RelativeMarker;
use super::RestrictionCoordination;
use super::RestrictionRun;
use super::RuleTag;
use super::Sentence;
use super::SentenceBody;
use super::SimpleClause;
use super::Subject;
use super::SubordinateBody;
use super::VerbAnalysis;
use super::VerbDependent;
use super::VerbInstance;
use super::VerbPhrase;
use super::VerbSlot;
use super::Vocab;
use super::reduction::auxiliary_form;
use super::reduction::predicate_form;
use super::take;
use crate::syntax::ObjectGapPredicate;

pub(in crate::grammar) fn lower_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseAuxiliaryProform
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseExceptBy
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhrasePreverbAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseCoinResult
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseManaAmountCoordination
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::ReducedRecipientPassiveNominalAdjunct
        | RuleTag::VerbPhraseCausative
        | RuleTag::VerbPhraseCoordinatedAdjective
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo => lower_predicate(tag, children),
        RuleTag::ManaAmountSymbol
        | RuleTag::ManaAmountSequence
        | RuleTag::ManaAmountListSingle
        | RuleTag::ManaAmountListComma
        | RuleTag::ManaAmountCoordination
        | RuleTag::ManaAmountCoordinationOxford => lower_mana_amount(tag, children),
        RuleTag::GerundClauseBase => {
            let Lowered::VerbPhrase(predicate) = take(children, 0)? else {
                return None;
            };
            let FinishedPredicate {
                modal, predicate, ..
            } = finish_predicate(predicate)?;
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
        | RuleTag::SimpleClauseSubjectDistributiveEach
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseExistential
        | RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderNegated
        | RuleTag::CopularRemainderDistributiveEach
        | RuleTag::ClauseCopular
        | RuleTag::ClauseContractedCopular
        | RuleTag::RelativeObject
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
        RuleTag::ClauseVariableValueConstraint => lower_variable_value_constraint(children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseAdverbBefore
        | RuleTag::ClauseSentenceAdverbialBefore
        | RuleTag::ClausePrepositionalBefore
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateGerundBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterComma
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::ExceptionRiderSingle
        | RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford
        | RuleTag::ClauseExcepted
        | RuleTag::ClauseRestrictionMember
        | RuleTag::ClauseRestrictionRun
        | RuleTag::Sentence => lower_composed_clause(tag, children),
        _ => None,
    }
}

pub(super) fn lower_predicate(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
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
                distributive_each: false,
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
                distributive_each: false,
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
        RuleTag::VerbPhrasePreverbAdverb => {
            // Not a §2.4 dependent site: children are swapped relative to the
            // sibling VerbPhraseAdverb (adverb at 0, VerbPhrase at 1), and the
            // modifier lands on preverb_modifiers rather than becoming a
            // VerbDependent — routing through PredicateAttachment::Adjunct
            // would render post-verbally (`cast next …`), a round-trip
            // failure. The literal matcher already pinned this token to
            // `next`, so the specific Vocab value need not be inspected here.
            let Lowered::Adverb(_next) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(mut predicate) = take(children, 1)? else {
                return None;
            };
            predicate.preverb_modifiers.push(PreverbModifier::Next);
            Some(Lowered::VerbPhrase(predicate))
        }
        RuleTag::VerbPhraseCausative => {
            let Lowered::VerbPhrase(mut predicate) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(complement) = take(children, 1)? else {
                return None;
            };
            predicate
                .dependents
                .push(VerbDependent::Infinitive(InfinitiveClause {
                    negated: false,
                    marker: InfinitiveMarker::Bare,
                    predicate: Box::new(complement),
                }));
            Some(Lowered::VerbPhrase(predicate))
        }
        RuleTag::VerbPhraseExceptBy => {
            // Handled directly rather than through `lower_predicate_dependent`,
            // whose ordinary dependent sits at child 1 and whose PP arm
            // consults the verb frame: here the PP is at child 2 and child 1
            // is the pinned `except` literal.
            let Lowered::VerbPhrase(mut predicate) = take(children, 0)? else {
                return None;
            };
            let Lowered::PrepositionalPhrase(pp) = take(children, 2)? else {
                return None;
            };
            predicate.dependents.push(VerbDependent::Exception(pp));
            Some(Lowered::VerbPhrase(predicate))
        }
        RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseCoinResult
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseManaAmountCoordination
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::ReducedRecipientPassiveNominalAdjunct
        | RuleTag::VerbPhraseCoordinatedAdjective => lower_predicate_dependent(tag, children),
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

#[allow(
    clippy::too_many_lines,
    reason = "one arm per predicate-dependent rule tag is intentionally verbose"
)]
pub(super) fn lower_predicate_dependent(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
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
        RuleTag::ReducedRecipientPassiveNominalAdjunct => {
            let Lowered::NounPhrase(noun_phrase) = take(children, 1)? else {
                return None;
            };
            match nominal_adjunct_kind(&noun_phrase)? {
                BareNominalAdjunct::Temporal => VerbDependent::Temporal(noun_phrase),
                BareNominalAdjunct::Manner => VerbDependent::Manner(noun_phrase),
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
        RuleTag::VerbPhraseCoordinatedAdjective => {
            let Lowered::CoordinatedModifier(coordinated) = take(children, 1)? else {
                return None;
            };
            VerbDependent::CoordinatedAdjective(coordinated_modifier_as_adjectives(coordinated)?)
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
        RuleTag::VerbPhraseCoinResult => {
            let Lowered::CoinResult(side) = take(children, 1)? else {
                return None;
            };
            VerbDependent::CoinResult(side)
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
        RuleTag::VerbPhraseQuotedAbility => {
            let Lowered::Phrase(phrase @ Phrase::QuotedAbility(_)) = take(children, 1)? else {
                return None;
            };
            VerbDependent::PredicateComplement(phrase)
        }
        RuleTag::VerbPhraseQuotedAbilityCoordination => {
            let Lowered::Phrase(Phrase::QuotedAbility(first)) = take(children, 1)? else {
                return None;
            };
            let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                return None;
            };
            let Lowered::Phrase(Phrase::QuotedAbility(next)) = take(children, 3)? else {
                return None;
            };
            VerbDependent::CoordinatedObject(CoordinatedPredicateObject {
                first: Box::new(PredicateObject::QuotedAbility(first)),
                rest: vec![PredicateObjectCoordination {
                    conjunction: Some(conjunction),
                    comma: false,
                    object: PredicateObject::QuotedAbility(next),
                }],
            })
        }
        RuleTag::VerbPhraseAbilityQuotedCoordination => {
            let Lowered::Catalog(atom) = take(children, 1)? else {
                return None;
            };
            let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                return None;
            };
            let Lowered::Phrase(Phrase::QuotedAbility(next)) = take(children, 3)? else {
                return None;
            };
            VerbDependent::CoordinatedObject(CoordinatedPredicateObject {
                first: Box::new(PredicateObject::Ability(AbilityObject {
                    ability: atom,
                    argument: None,
                })),
                rest: vec![PredicateObjectCoordination {
                    conjunction: Some(conjunction),
                    comma: false,
                    object: PredicateObject::QuotedAbility(next),
                }],
            })
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
        RuleTag::VerbPhraseManaAmountCoordination => {
            let Lowered::ManaAmount(PredicateObject::Coordinated(coordinated)) = take(children, 1)?
            else {
                return None;
            };
            VerbDependent::CoordinatedObject(coordinated)
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

/// Lower the six `ManaAmount*` rules. Mirrors the noun-phrase list lowering
/// (`mod.rs`, `NounPhraseListSingle` / `NounPhraseListComma` /
/// `NounPhraseCoordinationOxford`) but stays typed to
/// [`PredicateObject`] so no symbol is ever wrapped as a `NounPhrase`.
pub(super) fn lower_mana_amount(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    fn member(lowered: Lowered) -> Option<PredicateObject> {
        let Lowered::ManaAmount(object) = lowered else {
            return None;
        };
        Some(object)
    }

    #[allow(
        clippy::needless_pass_by_value,
        reason = "mirrors the by-value take() idiom used throughout this module"
    )]
    fn mana_conjunction(lowered: Lowered) -> Option<crate::syntax::PredicateConjunction> {
        let Lowered::Conjunction(conjunction) = lowered else {
            return None;
        };
        match conjunction {
            crate::syntax::PredicateConjunction::And | crate::syntax::PredicateConjunction::Or => {
                Some(conjunction)
            }
            // `then`/`and-or` never join predicate objects.
            crate::syntax::PredicateConjunction::Then
            | crate::syntax::PredicateConjunction::AndOr => None,
        }
    }

    match tag {
        RuleTag::ManaAmountSymbol => {
            let Lowered::OracleSymbol(symbol) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::ManaAmount(PredicateObject::OracleSymbol(symbol)))
        }
        RuleTag::ManaAmountSequence => {
            let Lowered::SymbolSequence(symbols) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::ManaAmount(PredicateObject::SymbolSequence(
                symbols,
            )))
        }
        RuleTag::ManaAmountListSingle => take(children, 0),
        RuleTag::ManaAmountListComma => {
            let first = member(take(children, 0)?)?;
            let next = member(take(children, 2)?)?;
            let coordination = PredicateObjectCoordination {
                conjunction: None,
                comma: true,
                object: next,
            };
            Some(Lowered::ManaAmount(PredicateObject::Coordinated(
                push_mana_coordination(first, coordination),
            )))
        }
        RuleTag::ManaAmountCoordination => {
            let first = member(take(children, 0)?)?;
            let conjunction = mana_conjunction(take(children, 1)?)?;
            let next = member(take(children, 2)?)?;
            let coordination = PredicateObjectCoordination {
                conjunction: Some(conjunction),
                comma: false,
                object: next,
            };
            Some(Lowered::ManaAmount(PredicateObject::Coordinated(
                push_mana_coordination(first, coordination),
            )))
        }
        RuleTag::ManaAmountCoordinationOxford => {
            let first = member(take(children, 0)?)?;
            let conjunction = mana_conjunction(take(children, 2)?)?;
            let next = member(take(children, 3)?)?;
            let coordination = PredicateObjectCoordination {
                conjunction: Some(conjunction),
                comma: true,
                object: next,
            };
            Some(Lowered::ManaAmount(PredicateObject::Coordinated(
                push_mana_coordination(first, coordination),
            )))
        }
        _ => None,
    }
}

/// Mirrors `push_noun_phrase_coordination` (`mod.rs`): fold a new coordination
/// member onto an already-coordinated first object, or start a fresh
/// coordinated run.
pub(super) fn push_mana_coordination(
    first: PredicateObject,
    coordination: PredicateObjectCoordination,
) -> CoordinatedPredicateObject {
    match first {
        PredicateObject::Coordinated(mut coordinated) => {
            coordinated.rest.push(coordination);
            coordinated
        }
        first => CoordinatedPredicateObject {
            first: Box::new(first),
            rest: vec![coordination],
        },
    }
}

#[allow(clippy::too_many_lines, reason = "lowering has many grammar variants")]
pub(super) fn lower_simple_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
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
        RuleTag::SimpleClauseSubjectDistributiveEach => {
            let Lowered::NounPhrase(subject) = take(children, 0)? else {
                return None;
            };
            // `each` (index 1) is the floating quantifier — discarded here
            // and carried as `PredicateHead::distributive_each` instead.
            let Lowered::VerbPhrase(mut predicate) = take(children, 2)? else {
                return None;
            };
            predicate.distributive_each = true;
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
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderNegated
        | RuleTag::CopularRemainderDistributiveEach
        | RuleTag::CopularRemainderCoordinatedAdjective) => lower_copular_remainder(tag, children),
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
                gap: RelativeGap::Object,
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
                gap: RelativeGap::Object,
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
                gap: RelativeGap::Subject,
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
                gap: RelativeGap::Subject,
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
                gap: RelativeGap::Subject,
                body: RelativeBody::SubjectGap(Predicate::Copular(
                    crate::syntax::CopularPredicate {
                        negated: false,
                        copula: crate::syntax::Copula {
                            auxiliary: subject_auxiliary.auxiliary,
                            contracted_with_subject: true,
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

pub(super) fn lower_copular_remainder(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let remainder = match tag {
        RuleTag::CopularRemainderNoun => {
            let Lowered::NounPhrase(complement) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                negated: false,
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::NounPhrase(complement),
                adjuncts: Vec::new(),
            }
        }
        RuleTag::CopularRemainderAdjective => {
            let Lowered::AdjectivePhrase(complement) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                negated: false,
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::Adjective(complement),
                adjuncts: Vec::new(),
            }
        }
        RuleTag::CopularRemainderCoordinatedAdjective => {
            let Lowered::CoordinatedModifier(coordinated) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                negated: false,
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::CoordinatedAdjective(
                    coordinated_modifier_as_adjectives(coordinated)?,
                ),
                adjuncts: Vec::new(),
            }
        }
        RuleTag::CopularRemainderPrepositional => {
            let Lowered::PrepositionalPhrase(complement) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                negated: false,
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::Prepositional(complement),
                adjuncts: Vec::new(),
            }
        }
        RuleTag::CopularRemainderPowerToughness => {
            let Lowered::PowerToughness(power_toughness) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                negated: false,
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::PowerToughness(power_toughness),
                adjuncts: Vec::new(),
            }
        }
        RuleTag::CopularRemainderPrepositionalAdjunct => {
            let Lowered::CopularRemainder(mut remainder) = take(children, 0)? else {
                return None;
            };
            let Lowered::PrepositionalPhrase(adjunct) = take(children, 1)? else {
                return None;
            };
            remainder
                .adjuncts
                .push(PredicateAdjunct::Prepositional(adjunct));
            remainder
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
        RuleTag::CopularRemainderNegated => {
            let Lowered::CopularRemainder(mut remainder) = take(children, 1)? else {
                return None;
            };
            // Double negation is not English and the corpus prints none; one
            // `not` per predication.
            if remainder.negated {
                return None;
            }
            remainder.negated = true;
            remainder
        }
        RuleTag::CopularRemainderDistributiveEach => {
            // `each` (index 0) is the floating quantifier — carried as a flag,
            // its lexical child discarded. The adjective (`equal`) takes the
            // trailing prepositional phrase (`to X`) as its own complement so
            // the standard stays bound to the adjective rather than floating as
            // a clause adjunct.
            let Lowered::AdjectivePhrase(mut adjective) = take(children, 1)? else {
                return None;
            };
            let Lowered::PrepositionalPhrase(standard) = take(children, 2)? else {
                return None;
            };
            adjective
                .complements
                .push(crate::syntax::AdjectiveComplement::Prepositional(standard));
            CopularRemainder {
                negated: false,
                distributive_each: true,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::Adjective(adjective),
                adjuncts: Vec::new(),
            }
        }
        _ => return None,
    };
    Some(Lowered::CopularRemainder(remainder))
}

pub(super) fn lower_copular_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
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
                negated: remainder.negated,
                copula,
                distributive_each: remainder.distributive_each,
                precomplement_adverbs: remainder.precomplement_adverbs,
                complement: remainder.complement,
                adjuncts: remainder.adjuncts,
            },
        ),
    )))
}

pub(super) fn lower_variable_value_constraint(children: &mut [Lowered]) -> Option<Lowered> {
    let Lowered::Quantity(subject_quantity @ Quantity::X) = take(children, 0)? else {
        return None;
    };
    let Lowered::Auxiliary(modal) = take(children, 1)? else {
        return None;
    };
    let Lowered::Auxiliary(be) = take(children, 2)? else {
        return None;
    };
    let Lowered::Number(number) = take(children, 3)? else {
        return None;
    };
    Some(Lowered::Clause(Clause::Independent(
        IndependentClause::Deontic(
            Subject(NounPhrase::Quantity(subject_quantity)),
            Modal { auxiliary: modal },
            Some(Predicate::Copular(crate::syntax::CopularPredicate {
                negated: false,
                copula: crate::syntax::Copula {
                    auxiliary: be,
                    contracted_with_subject: false,
                },
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::NounPhrase(NounPhrase::Quantity(Quantity::Exact(
                    number,
                ))),
                adjuncts: Vec::new(),
            })),
        ),
    )))
}

#[allow(
    clippy::too_many_lines,
    reason = "one arm per composed-clause rule tag is intentionally verbose"
)]
pub(super) fn lower_composed_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
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
        RuleTag::ClauseSubordinateGerundBefore => {
            let Lowered::Subordinator(subordinator) = take(children, 0)? else {
                return None;
            };
            let Lowered::GerundClause(gerund) = take(children, 1)? else {
                return None;
            };
            let Lowered::Clause(consequence) = take(children, 3)? else {
                return None;
            };
            conditional_body(
                subordinator,
                AttachmentPosition::BeforeMatrix,
                true,
                SubordinateBody::Gerund(gerund),
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
        RuleTag::ClauseSentenceAdverbialBefore => {
            let Lowered::Adverb(adverb) = take(children, 0)? else {
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
            let FinishedPredicate {
                modal, predicate, ..
            } = finish_predicate(predicate)?;
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
        RuleTag::ExceptionRiderSingle => {
            let Lowered::Clause(Clause::Independent(first)) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::ExceptionRider(ExceptionRider {
                first: Box::new(first),
                rest: Vec::new(),
            }))
        }
        RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford => {
            let Lowered::ExceptionRider(mut rider) = take(children, 0)? else {
                return None;
            };
            let (conjunction, clause_index, comma) = match tag {
                RuleTag::ExceptionRiderConjoined => (Some(1), 2, false),
                RuleTag::ExceptionRiderComma => (None, 2, true),
                RuleTag::ExceptionRiderOxford => (Some(2), 3, true),
                _ => return None,
            };
            let conjunction = match conjunction {
                Some(index) => {
                    let Lowered::Conjunction(conjunction) = take(children, index)? else {
                        return None;
                    };
                    Some(conjunction)
                }
                None => None,
            };
            let Lowered::Clause(Clause::Independent(clause)) = take(children, clause_index)? else {
                return None;
            };
            rider.rest.push(ExceptionConjunct {
                conjunction,
                comma,
                clause,
            });
            Some(Lowered::ExceptionRider(rider))
        }
        RuleTag::ClauseExcepted => {
            let Lowered::Clause(Clause::Independent(matrix)) = take(children, 0)? else {
                return None;
            };
            let Lowered::ExceptionRider(rider) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::Clause(Clause::Independent(
                with_clause_attachment(
                    matrix,
                    ClauseAttachment {
                        position: AttachmentPosition::AfterMatrix,
                        comma: true,
                        kind: ClauseAttachmentKind::Exception(rider),
                    },
                ),
            )))
        }
        RuleTag::ClauseRestrictionMember => {
            let Lowered::Adverb(only) = take(children, 0)? else {
                return None;
            };
            // A bare `only only` (nesting) is unrepresentable: the member
            // payload is never itself the `only` adverb.
            if only != crate::word::Vocab::Only {
                return None;
            }
            let adjuncts = match (children.len(), take(children, 1)?) {
                (2, Lowered::PrepositionalPhrase(preposition)) => {
                    vec![PredicateAdjunct::Prepositional(preposition)]
                }
                (2, Lowered::Adverb(word)) => {
                    if word.spelling() != "once" {
                        return None;
                    }
                    vec![PredicateAdjunct::Adverb(word)]
                }
                (3, Lowered::Subordinator(subordinator)) => {
                    if subordinator != crate::syntax::Subordinator::If {
                        return None;
                    }
                    let Lowered::Clause(Clause::Independent(condition)) = take(children, 2)? else {
                        return None;
                    };
                    vec![PredicateAdjunct::Dependent(Box::new(
                        DependentClause::Subordinate(
                            crate::syntax::Subordinator::If,
                            SubordinateBody::Finite(Box::new(condition)),
                        ),
                    ))]
                }
                (3, Lowered::Adverb(word)) => {
                    if word.spelling() != "once" {
                        return None;
                    }
                    let Lowered::NounPhrase(temporal) = take(children, 2)? else {
                        return None;
                    };
                    vec![
                        PredicateAdjunct::Adverb(word),
                        PredicateAdjunct::Temporal(temporal),
                    ]
                }
                _ => return None,
            };
            Some(Lowered::RestrictionMember(adjuncts))
        }
        RuleTag::ClauseRestrictionRun => match children.len() {
            2 => {
                let Lowered::Clause(Clause::Independent(matrix)) = take(children, 0)? else {
                    return None;
                };
                let Lowered::RestrictionRun(run) = take(children, 1)? else {
                    return None;
                };
                Some(Lowered::Clause(Clause::Independent(
                    with_clause_attachment(
                        matrix,
                        ClauseAttachment {
                            position: AttachmentPosition::AfterMatrix,
                            comma: false,
                            kind: ClauseAttachmentKind::Restriction(run),
                        },
                    ),
                )))
            }
            3 => match (take(children, 0)?, take(children, 1)?) {
                (Lowered::RestrictionMember(first), Lowered::Conjunction(conjunction)) => {
                    // Restrictions are cumulative [CR#601.3,602.5]: only a
                    // bare `and` joins members. `or`/`then`/`and-or` must keep
                    // failing.
                    if conjunction != crate::syntax::PredicateConjunction::And {
                        return None;
                    }
                    let Lowered::RestrictionMember(next) = take(children, 2)? else {
                        return None;
                    };
                    Some(Lowered::RestrictionRun(RestrictionRun {
                        first,
                        rest: vec![RestrictionCoordination {
                            conjunction: Some(conjunction),
                            comma: false,
                            adjuncts: next,
                        }],
                    }))
                }
                (Lowered::RestrictionMember(first), Lowered::Ignored) => {
                    // The comma-joined two-member base.
                    let Lowered::RestrictionMember(next) = take(children, 2)? else {
                        return None;
                    };
                    Some(Lowered::RestrictionRun(RestrictionRun {
                        first,
                        rest: vec![RestrictionCoordination {
                            conjunction: None,
                            comma: true,
                            adjuncts: next,
                        }],
                    }))
                }
                (Lowered::RestrictionRun(mut run), Lowered::Ignored) => {
                    let Lowered::RestrictionMember(next) = take(children, 2)? else {
                        return None;
                    };
                    run.rest.push(RestrictionCoordination {
                        conjunction: None,
                        comma: true,
                        adjuncts: next,
                    });
                    Some(Lowered::RestrictionRun(run))
                }
                _ => None,
            },
            4 => {
                let Lowered::RestrictionRun(mut run) = take(children, 0)? else {
                    return None;
                };
                let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                    return None;
                };
                if conjunction != crate::syntax::PredicateConjunction::And {
                    return None;
                }
                let Lowered::RestrictionMember(next) = take(children, 3)? else {
                    return None;
                };
                run.rest.push(RestrictionCoordination {
                    conjunction: Some(conjunction),
                    comma: true,
                    adjuncts: next,
                });
                Some(Lowered::RestrictionRun(run))
            }
            _ => None,
        },
        RuleTag::Sentence => {
            let Lowered::Clause(Clause::Independent(clause)) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Sentence(Sentence {
                initial_uppercase: true,
                body: SentenceBody::Independent(clause),
            }))
        }
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
            Some(conjunction)
        }
        None => None,
    };
    let Lowered::SimpleClause(next) = take(children, clause_index)? else {
        return None;
    };
    let junction = CoordinationJunction { conjunction, comma };
    let coordinated = if next.subject.is_some() {
        let next = finish_simple_clause(next)?;
        let continuation = ClauseCoordination {
            conjunction: junction.conjunction,
            comma: junction.comma,
            member: CoordinatedClauseMember::Independent(Box::new(next)),
        };
        match first {
            IndependentClause::Coordinated(mut coordinated) => {
                coordinated.rest.push(continuation);
                IndependentClause::Coordinated(coordinated)
            }
            first => IndependentClause::Coordinated(CoordinatedIndependentClause {
                first: Box::new(first),
                rest: vec![continuation],
            }),
        }
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
        IndependentClause::Predicated(_, expression) => match expression {
            PredicateExpression::Simple(predicate) => finite_inflection_of_predicate(predicate),
            PredicateExpression::Coordinated(coordination) => coordination
                .conjuncts()
                .first()
                .and_then(finite_inflection_of_predicate),
        },
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
                Predicate::Transitive(first),
                junction,
                predicate,
            )),
        )),
        IndependentClause::Intransitive(subject, first) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                Predicate::Intransitive(first),
                junction,
                predicate,
            )),
        )),
        IndependentClause::Copular(subject, first) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                Predicate::Copular(first),
                junction,
                predicate,
            )),
        )),
        IndependentClause::Passive(subject, first) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                Predicate::Passive(first),
                junction,
                predicate,
            )),
        )),
        IndependentClause::Proform(subject, first) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                Predicate::Proform(first),
                junction,
                predicate,
            )),
        )),
        IndependentClause::Deontic(subject, modal, inner) => Some(IndependentClause::Predicated(
            Some(subject),
            PredicateExpression::Coordinated(Coordination::new(
                Predicate::Deontic(DeonticPredicate {
                    modal,
                    inner: inner.map(Box::new),
                }),
                junction,
                predicate,
            )),
        )),
        IndependentClause::Imperative(first) => Some(IndependentClause::Predicated(
            None,
            PredicateExpression::Coordinated(Coordination::new(first, junction, predicate)),
        )),
        IndependentClause::Predicated(subject, expression) => {
            let expression = match expression {
                PredicateExpression::Simple(first) => {
                    PredicateExpression::Coordinated(Coordination::new(first, junction, predicate))
                }
                PredicateExpression::Coordinated(mut coordinated) => {
                    coordinated.push(junction, predicate);
                    PredicateExpression::Coordinated(coordinated)
                }
            };
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
                    PredicateExpression::Coordinated(Coordination::new(first, junction, predicate)),
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
    let continuation = ClauseCoordination {
        conjunction: junction.conjunction,
        comma: junction.comma,
        member: CoordinatedClauseMember::Independent(Box::new(IndependentClause::Imperative(
            predicate,
        ))),
    };
    match clause {
        IndependentClause::Coordinated(mut coordinated) => {
            coordinated.rest.push(continuation);
            Some(IndependentClause::Coordinated(coordinated))
        }
        first => Some(IndependentClause::Coordinated(
            CoordinatedIndependentClause {
                first: Box::new(first),
                rest: vec![continuation],
            },
        )),
    }
}

pub(super) fn starts_new_clause_group(
    clause: &IndependentClause,
    junction: &CoordinationJunction,
) -> bool {
    let IndependentClause::Coordinated(coordinated) = clause else {
        return false;
    };
    coordinated.rest.last().is_some_and(|previous| {
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

pub(super) fn conditional(
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

pub(super) fn conditional_body(
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

pub(super) fn with_clause_attachment(
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

pub(super) struct FinishedPredicate {
    modal: Option<Modal>,
    predicate: Predicate,
    /// True when the verb phrase was an elided proform under a modal, i.e.
    /// VP-ellipsis ("If you can't, …"). The `predicate` field then holds the
    /// synthesized proform as a placeholder; consumers building a modal clause
    /// drop it so the modal renders alone.
    elided: bool,
}

pub(in crate::grammar) fn finish_simple_clause(simple: SimpleClause) -> Option<IndependentClause> {
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
pub(super) fn coordinated_modifier_as_adjectives(
    modifier: crate::syntax::CoordinatedModifier,
) -> Option<crate::syntax::CoordinatedAdjectivePhrase> {
    let first = modifier_as_predicative_adjective(*modifier.first)?;
    let mut rest = Vec::with_capacity(modifier.rest.len());
    for coordination in modifier.rest {
        rest.push(crate::syntax::AdjectivePhraseCoordination {
            conjunction: coordination.conjunction,
            comma: coordination.comma,
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

#[allow(
    clippy::too_many_lines,
    reason = "one arm per verb dependent is intentionally verbose"
)]
pub(super) fn finish_predicate(mut phrase: VerbPhrase) -> Option<FinishedPredicate> {
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
            VerbDependent::Exception(phrase) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Exception(
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
            VerbDependent::CoinResult(side) => {
                target_elements.push(PredicateElement::CoinResult(side));
            }
            VerbDependent::Frequency(frequency) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Frequency(
                    frequency,
                )));
            }
            VerbDependent::CoordinatedObject(coordinated) => {
                attach_object(&mut object, PredicateObject::Coordinated(coordinated))?;
            }
            VerbDependent::CoordinatedAdjective(coordinated) => {
                target_elements.push(PredicateElement::Complement(
                    PredicateComplement::CoordinatedAdjective(coordinated),
                ));
            }
        }
    }
    let head = PredicateHead {
        auxiliaries: phrase.auxiliaries,
        first_auxiliary_contracted_with_subject: phrase.first_auxiliary_contracted_with_subject,
        preverb_modifiers: phrase.preverb_modifiers,
        verb: phrase.verb,
        distributive_each: phrase.distributive_each,
    };
    // A bare proform under a modal (nothing surviving beside the modal) is
    // VP-ellipsis: "If you can't, …". The synthesized `do` pro-verb is a
    // placeholder the surface never spelled, so flag it for the modal-clause
    // builder to drop rather than render.
    let elided = proform
        && modal.is_some()
        && object.is_none()
        && head.auxiliaries.is_empty()
        && head.preverb_modifiers.is_empty()
        && pre_object_elements.is_empty()
        && elements.is_empty();
    let predicate = if passive {
        let retained_object = match object {
            None => None,
            Some(object) => {
                if !phrase.frame.is_recipient_passive() {
                    return None;
                }
                // Surface order is head → retained object → elements. Any
                // element that landed BEFORE the object would be lost by the
                // flattening below, so refuse rather than render a reordered
                // face. No corpus witness exists; if one appears, model the
                // position explicitly.
                if !pre_object_elements.is_empty() {
                    return None;
                }
                Some(object)
            }
        };
        pre_object_elements.append(&mut elements);
        Predicate::Passive(PassivePredicate {
            head,
            kind: crate::syntax::Passive { retained_object },
            elements: pre_object_elements,
        })
    } else if let Some(object) = object {
        Predicate::Transitive(crate::syntax::TransitivePredicate {
            head,
            kind: crate::syntax::Transitive {
                pre_object_elements,
                object,
            },
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
            kind: crate::syntax::Intransitive,
            elements: pre_object_elements,
        })
    };
    Some(FinishedPredicate {
        modal,
        predicate,
        elided,
    })
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

pub(super) const fn proform_inflection(slot: VerbSlot) -> AuxiliaryInflection {
    match slot {
        VerbSlot::Infinitive | VerbSlot::Imperative => AuxiliaryInflection::Base,
        VerbSlot::Present { person, number } => AuxiliaryInflection::Present { person, number },
        VerbSlot::Past { person, number } => AuxiliaryInflection::Past { person, number },
        VerbSlot::PresentParticiple => AuxiliaryInflection::PresentParticiple,
        VerbSlot::PastParticiple => AuxiliaryInflection::PastParticiple,
    }
}

pub(in crate::grammar) fn finish_infinitive(
    clause: InfinitiveClause,
) -> Option<crate::syntax::InfinitiveClause> {
    let FinishedPredicate {
        modal, predicate, ..
    } = finish_predicate(*clause.predicate)?;
    if modal.is_some() {
        return None;
    }
    Some(crate::syntax::InfinitiveClause {
        negated: clause.negated,
        marker: clause.marker,
        predicate: Box::new(predicate),
    })
}

pub(super) fn attach_object(
    slot: &mut Option<PredicateObject>,
    object: PredicateObject,
) -> Option<()> {
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

pub(super) const fn is_modal(auxiliary: Auxiliary) -> bool {
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

pub(super) fn lowered_nominal_adjunct_kind(
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
    match &nominal.head {
        NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun) => {
            noun.bare_nominal_adjunct()
        }
    }
}
