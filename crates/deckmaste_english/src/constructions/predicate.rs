//! Compiler-derived English predicate declarations.

#![allow(
    dead_code,
    clippy::needless_pass_by_value,
    clippy::unnecessary_wraps,
    reason = "generated adapters own their erased builder and inverse values"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::catalog::CatalogAtom;
use crate::features::Comma;
use crate::features::Conjunction;
use crate::features::Contraction;
use crate::grammar::Features;
use crate::grammar::InfinitiveClause;
use crate::grammar::PredicateAttachment;
use crate::grammar::VerbAnalysis;
use crate::grammar::VerbDependent;
use crate::grammar::VerbPhrase;
use crate::grammar::auxiliary_form;
use crate::grammar::extend_predicate_features;
use crate::grammar::fold_auxiliary_passive;
use crate::grammar::reduction::reduce_verb_phrase_base;
use crate::syntax::AbilityObject;
use crate::syntax::AdjectivePhrase;
use crate::syntax::CoinSide;
use crate::syntax::CoordinatedPredicateObject;
use crate::syntax::DeonticPredicate;
use crate::syntax::FrequencyBound;
use crate::syntax::FrequencyPhrase;
use crate::syntax::Modal;
use crate::syntax::NounPhrase;
use crate::syntax::OracleSymbol;
use crate::syntax::PassivePredicate;
use crate::syntax::Phrase;
use crate::syntax::PowerToughness;
use crate::syntax::Predicate;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PredicateComplement;
use crate::syntax::PredicateElement;
use crate::syntax::PredicateHead;
use crate::syntax::PredicateObject;
use crate::syntax::PredicateObjectCoordination;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PreverbModifier;
use crate::syntax::ProPredicate;
use crate::syntax::Quantity;
use crate::syntax::QuotedAbility;
use crate::syntax::VerbParticle;
use crate::word::Auxiliary;
use crate::word::AuxiliaryInflection;
use crate::word::AuxiliaryInstance;
use crate::word::PredicateFrame;
use crate::word::VerbInstance;
use crate::word::VerbSlot;
use crate::word::Vocab;
use crate::word::Vocabulary;

type Verb = VerbAnalysis;
type SymbolSequence = Vec<OracleSymbol>;
type ManaAmount = PredicateObject;
type ManaAmountList = PredicateObject;
type CoordinatedManaAmount = CoordinatedPredicateObject;

/// A typed choice among the declaration's lexical predicate frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredicateFrameChoice {
    Intransitive,
    Transitive,
    Ditransitive,
    RecipientPassive,
    Causative,
}

/// An opaque, declaration-validated predicate construction in progress.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateBuilder {
    phrase: VerbPhrase,
}

pub(crate) struct FinishedPredicate {
    pub(crate) modal: Option<Modal>,
    pub(crate) predicate: Predicate,
    pub(crate) elided: bool,
}

/// Starts a checked predicate from a typed lexical head and frame choice.
///
/// # Errors
///
/// Returns a declaration violation when the verb is not renderable or does not
/// license exactly one frame of the requested typed kind.
pub fn build_predicate_verb(
    verb: VerbInstance,
    choice: PredicateFrameChoice,
) -> Result<PredicateBuilder, DeclarationViolation> {
    let frame = select_public_frame(&verb, choice)?;
    let phrase = build_verb_phrase_base(VerbAnalysis::new(verb, frame))?;
    Ok(PredicateBuilder { phrase })
}

/// Attaches a noun-phrase direct object through the generated declaration.
///
/// # Errors
///
/// Returns a declaration violation when the selected frame or current
/// attachment phase does not admit a direct object.
pub fn build_predicate_direct_object(
    predicate: PredicateBuilder,
    object: NounPhrase,
) -> Result<PredicateBuilder, DeclarationViolation> {
    build_verb_phrase_direct_object(predicate.phrase, object)
        .map(|phrase| PredicateBuilder { phrase })
}

/// Wraps a predicate with a typed auxiliary through the generated declaration.
///
/// # Errors
///
/// Returns a declaration violation when the auxiliary's form or the resulting
/// voice is incompatible with the predicate's declared state.
pub fn build_predicate_auxiliary(
    auxiliary: AuxiliaryInstance,
    predicate: PredicateBuilder,
) -> Result<PredicateBuilder, DeclarationViolation> {
    build_verb_phrase_auxiliary(auxiliary, predicate.phrase)
        .map(|phrase| PredicateBuilder { phrase })
}

/// Attaches one public predicate element through its declaration-owned row.
///
/// # Errors
///
/// Returns a declaration violation when no generated predicate row admits the
/// typed element in the builder's current state or order.
pub fn build_predicate_element(
    predicate: PredicateBuilder,
    element: PredicateElement,
) -> Result<PredicateBuilder, DeclarationViolation> {
    let phrase = match element {
        PredicateElement::Complement(PredicateComplement::IndirectObject(value)) => {
            build_verb_phrase_indirect_object(predicate.phrase, value)?
        }
        PredicateElement::Complement(PredicateComplement::Adjective(value)) => {
            build_verb_phrase_adjective(predicate.phrase, value)?
        }
        PredicateElement::Complement(PredicateComplement::Prepositional(value))
        | PredicateElement::Adjunct(PredicateAdjunct::Prepositional(value)) => {
            build_verb_phrase_prepositional(predicate.phrase, value)?
        }
        PredicateElement::Complement(PredicateComplement::Infinitive(value)) => {
            let infinitive = InfinitiveClause::from_finished_parts(
                value.negated,
                value.marker,
                inverse_public_predicate(&value.predicate)?,
            );
            build_verb_phrase_infinitive(predicate.phrase, infinitive)?
        }
        PredicateElement::Adjunct(PredicateAdjunct::Adverb(value)) => {
            build_verb_phrase_adverb(predicate.phrase, value)?
        }
        PredicateElement::Adjunct(PredicateAdjunct::Frequency(value)) => {
            build_verb_phrase_frequency(predicate.phrase, value)?
        }
        PredicateElement::Adjunct(PredicateAdjunct::Exception(value)) => {
            build_verb_phrase_except_by(predicate.phrase, value)?
        }
        PredicateElement::Particle(value) => build_verb_phrase_particle(predicate.phrase, value)?,
        PredicateElement::CoinResult(value) => {
            build_verb_phrase_coin_result(predicate.phrase, value)?
        }
        PredicateElement::Complement(PredicateComplement::CoordinatedAdjective(_))
        | PredicateElement::Adjunct(
            PredicateAdjunct::Temporal(_)
            | PredicateAdjunct::Manner(_)
            | PredicateAdjunct::Dependent(_),
        ) => {
            return Err(violation(
                "predicate",
                "the public element has a generated predicate construction",
            ));
        }
    };
    Ok(PredicateBuilder { phrase })
}

/// Finishes a checked builder into the sealed public predicate projection.
///
/// # Errors
///
/// Returns a declaration violation when required arguments are missing or the
/// validated construction cannot project to one sealed public predicate shape.
pub fn finish_predicate(predicate: PredicateBuilder) -> Result<Predicate, DeclarationViolation> {
    ensure_complete(&predicate.phrase)?;
    let FinishedPredicate {
        modal,
        predicate,
        elided,
    } = project_public_predicate(predicate.phrase)?;
    match modal {
        None => Ok(predicate),
        Some(modal) => Ok(Predicate::Deontic(DeonticPredicate {
            modal,
            inner: (!elided).then(|| Box::new(predicate)),
        })),
    }
}

/// Projects immutable checked construction parts from a sealed predicate.
///
/// # Errors
///
/// Returns a declaration violation when the public predicate is outside the
/// generated family or does not satisfy the declaration's completion rules.
pub fn parts_predicate(predicate: &Predicate) -> Result<PredicateBuilder, DeclarationViolation> {
    let phrase = inverse_public_predicate(predicate)?;
    ensure_complete(&phrase)?;
    Ok(PredicateBuilder { phrase })
}

/// Rebuilds a sealed predicate from its immutable checked parts.
///
/// # Errors
///
/// Returns a declaration violation when the parts are incomplete or cannot
/// project to one sealed public predicate shape.
pub fn rebuild_predicate(parts: PredicateBuilder) -> Result<Predicate, DeclarationViolation> {
    finish_predicate(parts)
}

fn select_public_frame(
    verb: &VerbInstance,
    choice: PredicateFrameChoice,
) -> Result<PredicateFrame, DeclarationViolation> {
    let mut candidates =
        verb.verb
            .predicate_frames()
            .iter()
            .copied()
            .filter(|frame| match choice {
                PredicateFrameChoice::Intransitive => {
                    !frame.direct_object().accepts()
                        && !frame.indirect_object().accepts()
                        && !frame.is_recipient_passive()
                        && !frame.causative_complement()
                        && !frame.requires_coin_result()
                }
                PredicateFrameChoice::Transitive => {
                    frame.direct_object().accepts()
                        && !frame.indirect_object().accepts()
                        && !frame.is_recipient_passive()
                        && !frame.causative_complement()
                }
                PredicateFrameChoice::Ditransitive => {
                    frame.indirect_object().accepts() && !frame.is_recipient_passive()
                }
                PredicateFrameChoice::RecipientPassive => frame.is_recipient_passive(),
                PredicateFrameChoice::Causative => frame.causative_complement(),
            });
    let frame = candidates.next().ok_or_else(|| {
        violation(
            "verb_phrase_base",
            "the lexical verb licenses the requested typed predicate frame",
        )
    })?;
    if candidates.next().is_some() {
        return Err(violation(
            "verb_phrase_base",
            "the typed predicate frame choice selects exactly one lexical frame",
        ));
    }
    Ok(frame)
}

fn ensure_complete(phrase: &VerbPhrase) -> Result<(), DeclarationViolation> {
    phrase
        .declaration_core_arguments_complete()
        .then_some(())
        .ok_or_else(|| {
            violation(
                "predicate",
                "the generated predicate satisfies form, voice, valency, and dependent order",
            )
        })
}

#[allow(
    clippy::too_many_lines,
    reason = "one arm per declaration dependent keeps the public projection explicit"
)]
pub(crate) fn project_public_predicate(
    phrase: VerbPhrase,
) -> Result<FinishedPredicate, DeclarationViolation> {
    let (
        mut auxiliaries,
        first_auxiliary_contracted_with_subject,
        preverb_modifiers,
        verb,
        frame,
        dependents,
        distributive_each,
    ) = phrase.into_public_projection_parts();
    let proform = frame.is_proform();
    let modal = auxiliaries
        .first()
        .copied()
        .filter(|auxiliary| is_modal(auxiliary.auxiliary))
        .map(|auxiliary| {
            auxiliaries.remove(0);
            Modal { auxiliary }
        });
    let passive = verb.slot == VerbSlot::PastParticiple
        && auxiliaries
            .first()
            .is_some_and(|auxiliary| auxiliary.auxiliary == Auxiliary::Be);
    let mut object = None;
    let mut pre_object_elements = Vec::new();
    let mut elements = Vec::new();
    for dependent in dependents {
        let target_elements =
            if object.is_none() { &mut pre_object_elements } else { &mut elements };
        match dependent {
            VerbDependent::DirectObject(noun_phrase) => {
                attach_public_object(&mut object, PredicateObject::NounPhrase(noun_phrase))?;
            }
            VerbDependent::IndirectObject(noun_phrase) => {
                target_elements.push(PredicateElement::Complement(
                    PredicateComplement::IndirectObject(noun_phrase),
                ));
            }
            VerbDependent::PredicateComplement(phrase) => match phrase {
                Phrase::CatalogAtom(atom) => {
                    attach_public_object(
                        &mut object,
                        PredicateObject::Ability(AbilityObject {
                            ability: atom,
                            argument: None,
                        }),
                    )?;
                }
                Phrase::EmbeddedAbility(ability) => {
                    attach_public_object(&mut object, PredicateObject::EmbeddedAbility(ability))?;
                }
                Phrase::QuotedAbility(ability) => {
                    attach_public_object(&mut object, PredicateObject::QuotedAbility(ability))?;
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
                _ => return Err(public_projection_violation()),
            },
            VerbDependent::Scalar(phrase) => match phrase {
                Phrase::Quantity(quantity) => {
                    attach_public_object(&mut object, PredicateObject::Quantity(quantity))?;
                }
                Phrase::OracleSymbol(symbol) => {
                    attach_public_object(&mut object, PredicateObject::OracleSymbol(symbol))?;
                }
                Phrase::SymbolSequence(symbols) => {
                    attach_public_object(&mut object, PredicateObject::SymbolSequence(symbols))?;
                }
                _ => return Err(public_projection_violation()),
            },
            VerbDependent::Statistic(Phrase::PowerToughness(value)) => {
                attach_public_object(&mut object, PredicateObject::PowerToughness(value))?;
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
                    PredicateComplement::Infinitive(project_public_infinitive(clause)?),
                ));
            }
            VerbDependent::Subordinate(clause) => {
                let crate::syntax::Clause::Dependent(clause) = *clause else {
                    return Err(public_projection_violation());
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
            VerbDependent::Statistic(_) | VerbDependent::Adverbial(_) => {
                return Err(public_projection_violation());
            }
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
                attach_public_object(&mut object, PredicateObject::Coordinated(coordinated))?;
            }
            VerbDependent::CoordinatedAdjective(coordinated) => {
                target_elements.push(PredicateElement::Complement(
                    PredicateComplement::CoordinatedAdjective(coordinated),
                ));
            }
        }
    }
    let head = PredicateHead {
        auxiliaries,
        first_auxiliary_contracted_with_subject: Contraction::from(
            first_auxiliary_contracted_with_subject,
        ),
        preverb_modifiers,
        verb,
        frame,
        distributive_each,
    };
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
                if !frame.is_recipient_passive() || !pre_object_elements.is_empty() {
                    return Err(public_projection_violation());
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
                contracted_negation: Contraction::Full,
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
    Ok(FinishedPredicate {
        modal,
        predicate,
        elided,
    })
}

fn project_public_infinitive(
    clause: InfinitiveClause,
) -> Result<crate::syntax::InfinitiveClause, DeclarationViolation> {
    let (negated, marker, predicate) = clause.declaration_parts();
    let FinishedPredicate {
        modal, predicate, ..
    } = project_public_predicate(predicate.clone())?;
    if modal.is_some() {
        return Err(public_projection_violation());
    }
    Ok(crate::syntax::InfinitiveClause {
        negated,
        marker,
        predicate: Box::new(predicate),
    })
}

fn attach_public_object(
    slot: &mut Option<PredicateObject>,
    object: PredicateObject,
) -> Result<(), DeclarationViolation> {
    if let Some(PredicateObject::Ability(ability)) = slot
        && ability.argument.is_none()
    {
        ability.argument = Some(Box::new(object));
        return Ok(());
    }
    if slot.replace(object).is_some() {
        return Err(public_projection_violation());
    }
    Ok(())
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

pub(crate) const fn is_modal(auxiliary: Auxiliary) -> bool {
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

fn public_projection_violation() -> DeclarationViolation {
    violation(
        "predicate",
        "the generated predicate projects to one sealed public shape without reordering",
    )
}

pub(crate) fn inverse_public_predicate(
    predicate: &Predicate,
) -> Result<VerbPhrase, DeclarationViolation> {
    let invalid = || {
        violation(
            "predicate",
            "the sealed public predicate has a generated owner",
        )
    };
    match predicate {
        Predicate::Transitive(predicate) => {
            let mut dependents = public_elements(&predicate.pre_object_elements)?;
            dependents.extend(public_object(&predicate.object)?);
            dependents.extend(public_elements(predicate.elements())?);
            Ok(VerbPhrase::from_finished_parts(
                predicate.head(),
                dependents,
            ))
        }
        Predicate::Intransitive(predicate) => Ok(VerbPhrase::from_finished_parts(
            predicate.head(),
            public_elements(predicate.elements())?,
        )),
        Predicate::Passive(predicate) => {
            let mut dependents = Vec::new();
            if let Some(object) = &predicate.retained_object {
                dependents.extend(public_object(object)?);
            }
            dependents.extend(public_elements(predicate.elements())?);
            Ok(VerbPhrase::from_finished_parts(
                predicate.head(),
                dependents,
            ))
        }
        Predicate::Proform(predicate) => Ok(VerbPhrase::declaration_proform(predicate.auxiliary)),
        Predicate::Deontic(predicate) => {
            let inner = predicate.inner.as_deref().map_or_else(
                || Ok(VerbPhrase::declaration_elided_proform()),
                inverse_public_predicate,
            )?;
            Ok(VerbPhrase::declaration_with_auxiliary(
                inner,
                predicate.modal.auxiliary,
            ))
        }
        Predicate::Attached(_) | Predicate::Copular(_) => Err(invalid()),
    }
}

pub(crate) fn inverse_public_object_gap_predicate(
    predicate: &crate::syntax::ObjectGapPredicate,
) -> Result<VerbPhrase, DeclarationViolation> {
    Ok(VerbPhrase::from_finished_parts(
        predicate.head(),
        public_elements(predicate.elements())?,
    ))
}

fn public_elements(
    elements: &[PredicateElement],
) -> Result<Vec<VerbDependent>, DeclarationViolation> {
    elements.iter().map(public_element).collect()
}

fn public_element(element: &PredicateElement) -> Result<VerbDependent, DeclarationViolation> {
    Ok(match element {
        PredicateElement::Complement(PredicateComplement::IndirectObject(value)) => {
            VerbDependent::IndirectObject(value.clone())
        }
        PredicateElement::Complement(PredicateComplement::Adjective(value)) => {
            VerbDependent::Adverbial(Phrase::AdjectivePhrase(Box::new(value.clone())))
        }
        PredicateElement::Complement(PredicateComplement::CoordinatedAdjective(value)) => {
            VerbDependent::CoordinatedAdjective(value.clone())
        }
        PredicateElement::Complement(PredicateComplement::Prepositional(value)) => {
            VerbDependent::PredicateComplement(Phrase::PrepositionalPhrase(Box::new(value.clone())))
        }
        PredicateElement::Complement(PredicateComplement::Infinitive(value)) => {
            VerbDependent::Infinitive(InfinitiveClause::from_finished_parts(
                value.negated,
                value.marker,
                inverse_public_predicate(&value.predicate)?,
            ))
        }
        PredicateElement::Adjunct(PredicateAdjunct::Adverb(value)) => {
            VerbDependent::Adverbial(Phrase::Adverb(*value))
        }
        PredicateElement::Adjunct(PredicateAdjunct::Frequency(value)) => {
            VerbDependent::Frequency(*value)
        }
        PredicateElement::Adjunct(PredicateAdjunct::Temporal(value)) => {
            VerbDependent::Temporal(value.clone())
        }
        PredicateElement::Adjunct(PredicateAdjunct::Manner(value)) => {
            VerbDependent::Manner(value.clone())
        }
        PredicateElement::Adjunct(PredicateAdjunct::Prepositional(value)) => {
            VerbDependent::Prepositional(value.clone())
        }
        PredicateElement::Adjunct(PredicateAdjunct::Exception(value)) => {
            VerbDependent::Exception(value.clone())
        }
        PredicateElement::Adjunct(PredicateAdjunct::Dependent(value)) => {
            VerbDependent::Subordinate(Box::new(crate::syntax::Clause::Dependent(
                (**value).clone(),
            )))
        }
        PredicateElement::Particle(value) => VerbDependent::Particle(*value),
        PredicateElement::CoinResult(value) => VerbDependent::CoinResult(*value),
    })
}

fn public_object(object: &PredicateObject) -> Result<Vec<VerbDependent>, DeclarationViolation> {
    let mut dependents = Vec::new();
    match object {
        PredicateObject::NounPhrase(value) => {
            dependents.push(VerbDependent::DirectObject(value.clone()));
        }
        PredicateObject::Ability(value) => {
            dependents.push(VerbDependent::PredicateComplement(Phrase::CatalogAtom(
                value.ability.clone(),
            )));
            if let Some(argument) = &value.argument {
                dependents.extend(public_object(argument)?);
            }
        }
        PredicateObject::Quantity(value) => {
            dependents.push(VerbDependent::Scalar(Phrase::Quantity(*value)));
        }
        PredicateObject::OracleSymbol(value) => {
            dependents.push(VerbDependent::Scalar(Phrase::OracleSymbol(value.clone())));
        }
        PredicateObject::SymbolSequence(value) => {
            dependents.push(VerbDependent::Scalar(Phrase::SymbolSequence(value.clone())));
        }
        PredicateObject::PowerToughness(value) => {
            dependents.push(VerbDependent::Statistic(Phrase::PowerToughness(*value)));
        }
        PredicateObject::EmbeddedAbility(value) => {
            dependents.push(VerbDependent::PredicateComplement(Phrase::EmbeddedAbility(
                value.clone(),
            )));
        }
        PredicateObject::QuotedAbility(value) => {
            dependents.push(VerbDependent::PredicateComplement(Phrase::QuotedAbility(
                value.clone(),
            )));
        }
        PredicateObject::Coordinated(value) => {
            dependents.push(VerbDependent::CoordinatedObject(value.clone()));
        }
    }
    Ok(dependents)
}

fn violation(construction: &'static str, requirement: &'static str) -> DeclarationViolation {
    DeclarationViolation {
        construction,
        requirement,
    }
}

fn make_verb(head: VerbAnalysis) -> Result<VerbAnalysis, DeclarationViolation> {
    if Vocabulary::new()
        .render_verb_instance(head.instance())
        .is_none()
    {
        return Err(violation(
            "verb",
            "the lexical head has a renderable verb form",
        ));
    }
    Ok(head)
}

fn verb_parts(value: &VerbAnalysis) -> VerbAnalysis {
    value.clone()
}

fn is_verb(value: &VerbAnalysis) -> bool {
    Vocabulary::new()
        .render_verb_instance(value.instance())
        .is_some()
}

fn reduce_verb_features(head: &Features) -> Option<Features> {
    matches!(head, Features::Verb { .. }).then(|| head.clone())
}

fn make_verb_phrase_base(head: VerbAnalysis) -> Result<VerbPhrase, DeclarationViolation> {
    if Vocabulary::new()
        .render_verb_instance(head.instance())
        .is_none()
    {
        return Err(DeclarationViolation {
            construction: "verb_phrase_base",
            requirement: "the lexical head has a renderable verb form",
        });
    }
    Ok(VerbPhrase::from_base(head))
}

fn verb_phrase_base_parts(value: &VerbPhrase) -> VerbAnalysis {
    value.base_head()
}

fn is_verb_phrase_base(value: &VerbPhrase) -> bool {
    value.is_declaration_base()
}

pub(crate) fn reduce_verb_phrase_auxiliary_proform_features(
    auxiliary: &Features,
) -> Option<Features> {
    let Features::Auxiliary(auxiliary) = auxiliary else {
        return None;
    };
    let form = auxiliary_form(*auxiliary, crate::grammar::PredicateForm::Infinitive)?;
    Some(Features::VerbPhrase {
        form,
        passive: false,
        dependent_count: 0,
        object: crate::grammar::PredicateObjectState::None,
        indirect_object: false,
        selected_preposition: false,
        phase: crate::grammar::PredicateAttachmentPhase::Object,
        frame: crate::word::PROFORM_PREDICATE_FRAMES[0],
        bare: false,
        head_is_copular: false,
        object_gap_requires_rules_object: false,
        subjunctive: matches!(
            auxiliary.inflection(),
            crate::word::AuxiliaryInflection::PastSubjunctive
        ),
    })
}

pub(crate) fn reduce_verb_phrase_auxiliary_features(
    auxiliary: &Features,
    predicate: &Features,
) -> Option<Features> {
    let Features::Auxiliary(auxiliary) = auxiliary else {
        return None;
    };
    let Features::VerbPhrase {
        form: child_form,
        passive: child_passive,
        dependent_count,
        object,
        indirect_object,
        selected_preposition,
        phase,
        frame,
        head_is_copular,
        object_gap_requires_rules_object,
        subjunctive: child_subjunctive,
        ..
    } = predicate
    else {
        return None;
    };
    let form = auxiliary_form(*auxiliary, *child_form)?;
    let passive = fold_auxiliary_passive(
        *auxiliary,
        *child_form,
        *child_passive,
        *object,
        *indirect_object,
        *frame,
    )?;
    Some(Features::VerbPhrase {
        form,
        passive,
        dependent_count: *dependent_count,
        object: *object,
        indirect_object: *indirect_object,
        selected_preposition: *selected_preposition,
        phase: *phase,
        frame: *frame,
        bare: false,
        head_is_copular: *head_is_copular,
        object_gap_requires_rules_object: *object_gap_requires_rules_object,
        subjunctive: *child_subjunctive
            || matches!(
                auxiliary.inflection(),
                crate::word::AuxiliaryInflection::PastSubjunctive
            ),
    })
}

fn noun_phrase_is_object_case(noun_phrase: &Features) -> bool {
    matches!(
        noun_phrase,
        Features::NounPhrase { pronoun_case, .. }
            if *pronoun_case != Some(crate::features::PronounCase::Subject)
    )
}

fn noun_phrase_value_is_object_case(noun_phrase: &NounPhrase) -> bool {
    !matches!(
        noun_phrase,
        NounPhrase::Pronoun {
            case: crate::features::PronounCase::Subject,
            ..
        }
    )
}

fn reduce_verb_phrase_direct_object_features(
    predicate: &Features,
    object: &Features,
) -> Option<Features> {
    let Features::NounPhrase {
        pronoun_case,
        adjunct,
        ..
    } = object
    else {
        return None;
    };
    if *pronoun_case == Some(crate::features::PronounCase::Subject) {
        return None;
    }
    let Features::VerbPhrase {
        form,
        passive,
        object,
        phase,
        frame,
        ..
    } = predicate
    else {
        return None;
    };
    let attachment = match adjunct.filter(|adjunct| {
        frame.licenses_bare_nominal_adjunct(*adjunct)
            && (*form == crate::grammar::PredicateForm::PastParticiple
                || *passive
                || *phase != crate::grammar::PredicateAttachmentPhase::Object
                || !matches!(object, crate::grammar::PredicateObjectState::None)
                || !frame.direct_object().accepts())
    }) {
        Some(adjunct) => PredicateAttachment::NominalAdjunct(adjunct),
        None => PredicateAttachment::DirectObject,
    };
    extend_predicate_features(predicate, attachment)
}

fn disprefer_active_temporal_attachment(
    predicate: &Features,
    object: &Features,
) -> Option<Features> {
    matches!(predicate, Features::VerbPhrase { passive: false, .. })
        .then_some(())
        .and_then(|()| {
            matches!(
                object,
                Features::NounPhrase {
                    adjunct: Some(crate::word::BareNominalAdjunct::Temporal),
                    ..
                }
            )
            .then(|| predicate.clone())
        })
}

fn reduce_verb_phrase_indirect_object_features(
    predicate: &Features,
    object: &Features,
) -> Option<Features> {
    noun_phrase_is_object_case(object)
        .then(|| extend_predicate_features(predicate, PredicateAttachment::IndirectObject))?
}

fn reduce_verb_phrase_adjective_features(
    predicate: &Features,
    adjective: &Features,
) -> Option<Features> {
    matches!(adjective, Features::Adjective { .. })
        .then(|| extend_predicate_features(predicate, PredicateAttachment::AdjectiveComplement))?
}

fn reduce_verb_phrase_prepositional_features(
    predicate: &Features,
    preposition: &Features,
) -> Option<Features> {
    let Features::PrepositionalPhrase { preposition, .. } = preposition else {
        return None;
    };
    extend_predicate_features(predicate, PredicateAttachment::Prepositional(*preposition))
}

fn reduce_verb_phrase_infinitive_features(
    predicate: &Features,
    infinitive: &Features,
) -> Option<Features> {
    matches!(infinitive, Features::InfinitiveClause)
        .then(|| extend_predicate_features(predicate, PredicateAttachment::InfinitiveComplement))?
}

fn reduce_object_gap_base(head: &Features) -> Option<Features> {
    let features = reduce_verb_phrase_base(head)?;
    let Features::VerbPhrase { frame, .. } = features else {
        return None;
    };
    frame.direct_object().accepts().then_some(features)
}

fn reduce_object_gap_attachment(
    predicate: &Features,
    dependent: &Features,
    attachment: PredicateAttachment,
) -> Option<Features> {
    let features = match attachment {
        PredicateAttachment::IndirectObject => {
            reduce_verb_phrase_indirect_object_features(predicate, dependent)?
        }
        PredicateAttachment::AdjectiveComplement => {
            reduce_verb_phrase_adjective_features(predicate, dependent)?
        }
        PredicateAttachment::Prepositional(_) => {
            reduce_verb_phrase_prepositional_features(predicate, dependent)?
        }
        PredicateAttachment::InfinitiveComplement => {
            reduce_verb_phrase_infinitive_features(predicate, dependent)?
        }
        _ => return None,
    };
    Some(features)
}

fn reduce_object_gap_auxiliary(auxiliary: &Features, predicate: &Features) -> Option<Features> {
    reduce_verb_phrase_auxiliary_features(auxiliary, predicate)
}

fn reduce_object_gap_indirect(predicate: &Features, object: &Features) -> Option<Features> {
    reduce_object_gap_attachment(predicate, object, PredicateAttachment::IndirectObject)
}

fn reduce_object_gap_adjective(predicate: &Features, adjective: &Features) -> Option<Features> {
    reduce_object_gap_attachment(
        predicate,
        adjective,
        PredicateAttachment::AdjectiveComplement,
    )
}

fn reduce_object_gap_prepositional(
    predicate: &Features,
    preposition: &Features,
) -> Option<Features> {
    reduce_object_gap_attachment(
        predicate,
        preposition,
        PredicateAttachment::Prepositional(crate::syntax::Preposition::By),
    )
}

fn reduce_object_gap_infinitive(predicate: &Features, infinitive: &Features) -> Option<Features> {
    reduce_object_gap_attachment(
        predicate,
        infinitive,
        PredicateAttachment::InfinitiveComplement,
    )
}

fn reduce_reduced_passive_base(head: &Features) -> Option<Features> {
    let features = reduce_verb_phrase_base(head)?;
    matches!(
        features,
        Features::VerbPhrase {
            form: crate::grammar::PredicateForm::PastParticiple,
            frame,
            ..
        } if frame.is_recipient_passive()
    )
    .then_some(features)
}

fn reduce_reduced_passive_direct(predicate: &Features, object: &Features) -> Option<Features> {
    let Features::NounPhrase {
        recipient_passive_theme: true,
        ..
    } = object
    else {
        return None;
    };
    reduce_verb_phrase_direct_object_features(predicate, object)
}

fn reduce_reduced_passive_prepositional(
    predicate: &Features,
    preposition: &Features,
) -> Option<Features> {
    reduce_verb_phrase_prepositional_features(predicate, preposition)
}

fn admit_argument_complete(features: Option<Features>) -> Option<Features> {
    let features = features?;
    crate::grammar::predicate_features_are_argument_complete(&features).then_some(features)
}

fn admit_attachment_prefix(
    predicate: &Features,
    attachments: impl IntoIterator<Item = PredicateAttachment>,
) -> Option<Features> {
    attachments
        .into_iter()
        .any(|attachment| extend_predicate_features(predicate, attachment).is_some())
        .then(|| predicate.clone())
}

fn admit_direct_object_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(
        predicate,
        [
            PredicateAttachment::DirectObject,
            PredicateAttachment::NominalAdjunct(crate::word::BareNominalAdjunct::Temporal),
            PredicateAttachment::NominalAdjunct(crate::word::BareNominalAdjunct::Manner),
        ],
    )
}

fn admit_indirect_object_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(predicate, [PredicateAttachment::IndirectObject])
}

fn admit_adjective_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(predicate, [PredicateAttachment::AdjectiveComplement])
}

fn admit_infinitive_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(predicate, [PredicateAttachment::InfinitiveComplement])
}

fn admit_particle_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(
        predicate,
        [
            PredicateAttachment::Particle(VerbParticle::In),
            PredicateAttachment::Particle(VerbParticle::Out),
        ],
    )
}

fn admit_coin_result_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(
        predicate,
        [PredicateAttachment::CoinResult(CoinSide::Heads)],
    )
}

fn admit_passive_shared_prepositional_prefix(predicate: &Features) -> Option<Features> {
    matches!(predicate, Features::VerbPhrase { passive: true, .. }).then(|| predicate.clone())
}

fn admit_exception_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(predicate, [PredicateAttachment::Exception])
}

fn admit_ability_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(predicate, [PredicateAttachment::AbilityComplement])
}

fn admit_quoted_ability_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(predicate, [PredicateAttachment::QuotedObject])
}

fn admit_scalar_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(predicate, [PredicateAttachment::ScalarComplement])
}

fn admit_power_toughness_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(predicate, [PredicateAttachment::StatisticComplement])
}

fn admit_quantity_prefix(predicate: &Features) -> Option<Features> {
    admit_attachment_prefix(predicate, [PredicateAttachment::ScalarOrAbilityArgument])
}

fn complete_base(head: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_base(head))
}

fn complete_auxiliary(auxiliary: &Features, predicate: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_auxiliary_features(auxiliary, predicate))
}

fn complete_auxiliary_proform(auxiliary: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_auxiliary_proform_features(auxiliary))
}

fn complete_direct_object(predicate: &Features, object: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_direct_object_features(predicate, object))
}

fn complete_indirect_object(predicate: &Features, object: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_indirect_object_features(
        predicate, object,
    ))
}

fn complete_adjective(predicate: &Features, adjective: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_adjective_features(predicate, adjective))
}

fn complete_prepositional(predicate: &Features, preposition: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_prepositional_features(
        predicate,
        preposition,
    ))
}

fn complete_infinitive(predicate: &Features, infinitive: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_infinitive_features(
        predicate, infinitive,
    ))
}

fn reduce_verb_phrase_adverb_features(predicate: &Features, adverb: &Features) -> Option<Features> {
    matches!(adverb, Features::None)
        .then(|| extend_predicate_features(predicate, PredicateAttachment::Adjunct))?
}

fn reduce_verb_phrase_preverb_adverb_features(
    modifier: &Features,
    predicate: &Features,
) -> Option<Features> {
    matches!(modifier, Features::None).then(|| predicate.clone())
}

fn reduce_verb_phrase_particle_features(
    predicate: &Features,
    particle: &Features,
) -> Option<Features> {
    let Features::VerbParticle(particle) = particle else {
        return None;
    };
    extend_predicate_features(predicate, PredicateAttachment::Particle(*particle))
}

fn reduce_verb_phrase_coin_result_features(
    predicate: &Features,
    side: &Features,
) -> Option<Features> {
    let Features::CoinResult(side) = side else {
        return None;
    };
    extend_predicate_features(predicate, PredicateAttachment::CoinResult(*side))
}

fn reduce_verb_phrase_frequency_features(
    predicate: &Features,
    frequency: &Features,
) -> Option<Features> {
    matches!(frequency, Features::None)
        .then(|| extend_predicate_features(predicate, PredicateAttachment::Adjunct))?
}

fn mark_generated_attachment_cost(features: &Features) -> Option<Features> {
    Some(features.clone())
}

fn reduce_passive_shared_prepositional_features(
    predicate: &Features,
    preposition: &Features,
) -> Option<Features> {
    let Features::VerbPhrase { passive: true, .. } = predicate else {
        return None;
    };
    let Features::PrepositionalPhrase {
        preposition,
        shared_determiner_object: true,
        ..
    } = preposition
    else {
        return None;
    };
    extend_predicate_features(predicate, PredicateAttachment::Prepositional(*preposition))
}

fn reduce_exception_features(predicate: &Features, preposition: &Features) -> Option<Features> {
    let Features::PrepositionalPhrase {
        preposition: crate::syntax::Preposition::By,
        ..
    } = preposition
    else {
        return None;
    };
    extend_predicate_features(predicate, PredicateAttachment::Exception)
}

fn reduce_frequency_phrase_features(frequency: &Features) -> Option<Features> {
    matches!(frequency, Features::None).then_some(Features::None)
}

fn reduce_frequency_phrase_adverb_features(frequency: &Features) -> Option<Features> {
    matches!(frequency, Features::None).then_some(Features::None)
}

fn reduce_verb_phrase_ability_features(
    predicate: &Features,
    ability: &Features,
) -> Option<Features> {
    matches!(ability, Features::None)
        .then(|| extend_predicate_features(predicate, PredicateAttachment::AbilityComplement))?
}

fn reduce_verb_phrase_quoted_ability_features(
    predicate: &Features,
    quoted: &Features,
) -> Option<Features> {
    matches!(quoted, Features::None)
        .then(|| extend_predicate_features(predicate, PredicateAttachment::QuotedObject))?
}

fn reduce_verb_phrase_quoted_coordination_features(
    predicate: &Features,
    pair: &Features,
) -> Option<Features> {
    matches!(pair, Features::GeneratedSequence { .. })
        .then(|| extend_predicate_features(predicate, PredicateAttachment::QuotedObject))?
}

fn reduce_verb_phrase_scalar_features(predicate: &Features, scalar: &Features) -> Option<Features> {
    matches!(scalar, Features::None)
        .then(|| extend_predicate_features(predicate, PredicateAttachment::ScalarComplement))?
}

fn reduce_verb_phrase_mana_coordination_features(
    predicate: &Features,
    coordination: &Features,
) -> Option<Features> {
    matches!(coordination, Features::None)
        .then(|| extend_predicate_features(predicate, PredicateAttachment::ScalarComplement))?
}

fn reduce_verb_phrase_power_toughness_features(
    predicate: &Features,
    value: &Features,
) -> Option<Features> {
    matches!(value, Features::PowerToughness { .. })
        .then(|| extend_predicate_features(predicate, PredicateAttachment::StatisticComplement))?
}

fn reduce_verb_phrase_quantity_features(
    predicate: &Features,
    quantity: &Features,
) -> Option<Features> {
    matches!(quantity, Features::Quantity(_)).then(|| {
        extend_predicate_features(predicate, PredicateAttachment::ScalarOrAbilityArgument)
    })?
}

fn complete_adverb(predicate: &Features, adverb: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_adverb_features(predicate, adverb))
}

fn complete_preverb_adverb(modifier: &Features, predicate: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_preverb_adverb_features(
        modifier, predicate,
    ))
}

fn complete_particle(predicate: &Features, particle: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_particle_features(predicate, particle))
}

fn complete_coin_result(predicate: &Features, side: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_coin_result_features(predicate, side))
}

fn complete_frequency(predicate: &Features, frequency: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_frequency_features(predicate, frequency))
}

fn complete_passive_shared_prepositional(
    predicate: &Features,
    preposition: &Features,
) -> Option<Features> {
    admit_argument_complete(reduce_passive_shared_prepositional_features(
        predicate,
        preposition,
    ))
}

fn complete_exception(predicate: &Features, preposition: &Features) -> Option<Features> {
    admit_argument_complete(reduce_exception_features(predicate, preposition))
}

fn complete_ability(predicate: &Features, ability: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_ability_features(predicate, ability))
}

fn complete_quoted_ability(predicate: &Features, quoted: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_quoted_ability_features(
        predicate, quoted,
    ))
}

fn complete_quoted_coordination(predicate: &Features, pair: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_quoted_coordination_features(
        predicate, pair,
    ))
}

fn complete_scalar(predicate: &Features, scalar: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_scalar_features(predicate, scalar))
}

fn complete_mana_coordination(predicate: &Features, coordination: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_mana_coordination_features(
        predicate,
        coordination,
    ))
}

fn complete_power_toughness(predicate: &Features, value: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_power_toughness_features(
        predicate, value,
    ))
}

fn complete_quantity(predicate: &Features, quantity: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_quantity_features(predicate, quantity))
}

fn causative_host_features(host: &Features) -> Option<Features> {
    let Features::VerbPhrase {
        form: crate::grammar::PredicateForm::Infinitive,
        passive: false,
        dependent_count: 1,
        object: crate::grammar::PredicateObjectState::Direct,
        indirect_object,
        selected_preposition,
        phase: crate::grammar::PredicateAttachmentPhase::Object,
        frame,
        ..
    } = host
    else {
        return None;
    };
    (frame.causative_complement()
        && crate::grammar::predicate_features_are_argument_complete(host)
        && frame.indirect_object().is_satisfied_by(*indirect_object)
        && frame
            .selected_preposition()
            .is_satisfied_by(*selected_preposition))
    .then(|| host.clone())
}

fn causative_complement_features(complement: &Features) -> Option<Features> {
    let Features::VerbPhrase {
        form: crate::grammar::PredicateForm::Infinitive,
        frame,
        bare,
        ..
    } = complement
    else {
        return None;
    };
    (!(*bare && frame.is_proform())
        && crate::grammar::predicate_features_are_argument_complete(complement))
    .then(|| complement.clone())
}

pub(crate) fn reduce_verb_phrase_causative_features(
    host: &Features,
    complement: &Features,
) -> Option<Features> {
    let Features::VerbPhrase {
        form,
        dependent_count,
        indirect_object,
        selected_preposition,
        frame,
        head_is_copular,
        object_gap_requires_rules_object,
        ..
    } = causative_host_features(host)?
    else {
        unreachable!("the causative host helper returns verb-phrase features")
    };
    causative_complement_features(complement)?;
    Some(Features::VerbPhrase {
        form,
        passive: false,
        dependent_count: dependent_count.checked_add(1)?,
        object: crate::grammar::PredicateObjectState::Direct,
        indirect_object,
        selected_preposition,
        phase: crate::grammar::PredicateAttachmentPhase::Tail,
        frame,
        bare: false,
        head_is_copular,
        object_gap_requires_rules_object,
        subjunctive: false,
    })
}

fn complete_causative(host: &Features, complement: &Features) -> Option<Features> {
    admit_argument_complete(reduce_verb_phrase_causative_features(host, complement))
}

fn make_verb_phrase_causative(
    host: VerbPhrase,
    complement: VerbPhrase,
) -> Result<VerbPhrase, DeclarationViolation> {
    if !causative_host_has_only_causee(&host) {
        return Err(violation(
            "verb_phrase_causative",
            "the host is a complete causative infinitive with exactly one direct-object causee",
        ));
    }
    let host_features = host.declaration_core_features().ok_or_else(|| {
        violation(
            "verb_phrase_causative",
            "the host is a complete causative infinitive with exactly one direct-object causee",
        )
    })?;
    if causative_host_features(&host_features).is_none() {
        return Err(violation(
            "verb_phrase_causative",
            "the host is a complete causative infinitive with exactly one direct-object causee",
        ));
    }
    let complement_features = complement.declaration_core_features().ok_or_else(|| {
        violation(
            "verb_phrase_causative",
            "the complement is a complete bare-infinitive predicate",
        )
    })?;
    if causative_complement_features(&complement_features).is_none() {
        return Err(violation(
            "verb_phrase_causative",
            "the complement is a complete bare-infinitive predicate",
        ));
    }
    reduce_verb_phrase_causative_features(&host_features, &complement_features).ok_or_else(
        || {
            violation(
                "verb_phrase_causative",
                "the causee precedes the bare-infinitive complement",
            )
        },
    )?;
    let (mut dependents, shell) = host.declaration_into_dependent_projection();
    dependents.push(VerbDependent::Infinitive(
        InfinitiveClause::declaration_bare(complement),
    ));
    Ok(VerbPhrase::declaration_from_dependent_projection(
        shell, dependents,
    ))
}

fn causative_host_has_only_causee(host: &VerbPhrase) -> bool {
    let (dependents, _) = host.clone().declaration_into_dependent_projection();
    matches!(dependents.as_slice(), [VerbDependent::DirectObject(_)])
}

fn verb_phrase_causative_parts(value: &VerbPhrase) -> (VerbPhrase, VerbPhrase) {
    let (host, VerbDependent::Infinitive(infinitive)) = value
        .declaration_last_dependent_parts()
        .expect("the causative inverse has a final infinitive dependent")
    else {
        unreachable!("the causative inverse check pins the dependent variant")
    };
    let complement = infinitive
        .declaration_bare_predicate()
        .expect("the causative inverse check pins the bare infinitive")
        .clone();
    (host, complement)
}

fn is_verb_phrase_causative(value: &VerbPhrase) -> bool {
    let Some((host, VerbDependent::Infinitive(infinitive))) =
        value.declaration_last_dependent_parts()
    else {
        return false;
    };
    if !causative_host_has_only_causee(&host) {
        return false;
    }
    let Some(complement) = infinitive.declaration_bare_predicate() else {
        return false;
    };
    let Some(host_features) = host.declaration_core_features() else {
        return false;
    };
    let Some(complement_features) = complement.declaration_core_features() else {
        return false;
    };
    reduce_verb_phrase_causative_features(&host_features, &complement_features).is_some()
}

fn make_verb_phrase_auxiliary(
    auxiliary: AuxiliaryInstance,
    predicate: VerbPhrase,
) -> Result<VerbPhrase, DeclarationViolation> {
    Ok(VerbPhrase::declaration_with_auxiliary(predicate, auxiliary))
}

fn verb_phrase_auxiliary_parts(value: &VerbPhrase) -> (AuxiliaryInstance, VerbPhrase) {
    value
        .declaration_auxiliary_parts()
        .expect("auxiliary inverse requires a leading auxiliary")
}

fn is_verb_phrase_auxiliary(value: &VerbPhrase) -> bool {
    let Some((_, predicate)) = value.declaration_auxiliary_parts() else {
        return false;
    };
    // A bare temporal/manner nominal is classified against the lexical
    // participle before a perfect auxiliary wraps it. The flattened finished
    // predicate retains both facts, so inverse dispatch must peel that
    // auxiliary before asking the nominal lens to reproduce its classification.
    let nominal_adjunct_reproduces_without_auxiliary = predicate
        .declaration_last_dependent_parts()
        .is_some_and(|(host, dependent)| {
            matches!(
                &dependent,
                VerbDependent::Temporal(_) | VerbDependent::Manner(_)
            ) && detach_nominal_dependent(&host, dependent).is_some()
        });
    let nominal_adjunct_reproduces_with_auxiliary = value
        .declaration_last_dependent_parts()
        .is_some_and(|(host, dependent)| {
            matches!(
                &dependent,
                VerbDependent::Temporal(_) | VerbDependent::Manner(_)
            ) && detach_nominal_dependent(&host, dependent).is_some()
        });
    let reveals_nominal_adjunct =
        nominal_adjunct_reproduces_without_auxiliary && !nominal_adjunct_reproduces_with_auxiliary;
    value.declaration_proform_part().is_none()
        && (value.declaration_last_dependent_parts().is_none()
            || is_verb_phrase_causative(&predicate)
            || reveals_nominal_adjunct)
        && value.declaration_first_preverb_modifier().is_none()
        && value.declaration_core_features().is_some()
}

fn make_verb_phrase_auxiliary_proform(
    auxiliary: AuxiliaryInstance,
) -> Result<VerbPhrase, DeclarationViolation> {
    Ok(VerbPhrase::declaration_proform(auxiliary))
}

fn verb_phrase_auxiliary_proform_parts(value: &VerbPhrase) -> AuxiliaryInstance {
    value
        .declaration_proform_part()
        .expect("proform inverse requires the canonical synthesized do shape")
}

fn is_verb_phrase_auxiliary_proform(value: &VerbPhrase) -> bool {
    value.declaration_proform_part().is_some() && value.declaration_core_features().is_some()
}

macro_rules! dependent_lens {
    ($from:ident, $into:ident, $check:ident, $variant:ident, $ty:ty, $valid:expr) => {
        fn $from(
            mut before: Vec<VerbDependent>,
            dependent: Option<$ty>,
            after: Vec<VerbDependent>,
            shell: VerbPhrase,
        ) -> VerbPhrase {
            if let Some(dependent) = dependent {
                before.push(VerbDependent::$variant(dependent));
            }
            before.extend(after);
            VerbPhrase::declaration_from_dependent_projection(shell, before)
        }

        fn $into(
            value: VerbPhrase,
        ) -> (Vec<VerbDependent>, Option<$ty>, Vec<VerbDependent>, VerbPhrase) {
            let (mut dependents, shell) = value.declaration_into_dependent_projection();
            let Some(index) = dependents
                .iter()
                .rposition(|dependent| matches!(dependent, VerbDependent::$variant(_)))
            else {
                return (dependents, None, Vec::new(), shell);
            };
            let after = dependents.split_off(index + 1);
            let VerbDependent::$variant(dependent) = dependents.remove(index) else {
                unreachable!("the dependent lens index pins its variant")
            };
            (dependents, Some(dependent), after, shell)
        }

        fn $check(value: &VerbPhrase) -> bool {
            matches!(
                value.declaration_last_dependent_parts(),
                Some((_, VerbDependent::$variant(ref dependent)))
                    if ($valid)(dependent)
            ) && value.declaration_core_features().is_some()
        }
    };
}

fn from_dependent_lens_parts(dependents: Vec<VerbDependent>, shell: VerbPhrase) -> VerbPhrase {
    VerbPhrase::declaration_from_dependent_projection(shell, dependents)
}

fn into_dependent_lens_parts(value: VerbPhrase) -> (Vec<VerbDependent>, VerbPhrase) {
    value.declaration_into_dependent_projection()
}

fn attach_nominal_dependent(
    predicate: &VerbPhrase,
    dependent: NounPhrase,
) -> Result<VerbDependent, DeclarationViolation> {
    if !noun_phrase_value_is_object_case(&dependent) {
        return Err(violation(
            "verb_phrase_direct_object",
            "the nominal dependent has object case",
        ));
    }
    let Some(features) = predicate.declaration_core_features() else {
        return Err(violation(
            "verb_phrase_direct_object",
            "the source owner has valid predicate features",
        ));
    };
    let (attachment, dependent) =
        match crate::grammar::lowered_nominal_adjunct_kind(predicate, &dependent) {
            Some(crate::word::BareNominalAdjunct::Temporal) => (
                PredicateAttachment::NominalAdjunct(crate::word::BareNominalAdjunct::Temporal),
                VerbDependent::Temporal(dependent),
            ),
            Some(crate::word::BareNominalAdjunct::Manner) => (
                PredicateAttachment::NominalAdjunct(crate::word::BareNominalAdjunct::Manner),
                VerbDependent::Manner(dependent),
            ),
            None => (
                PredicateAttachment::DirectObject,
                VerbDependent::DirectObject(dependent),
            ),
        };
    extend_predicate_features(&features, attachment)
        .is_some()
        .then_some(dependent)
        .ok_or_else(|| {
            violation(
                "verb_phrase_direct_object",
                "the nominal dependent is licensed at the source owner's attachment phase",
            )
        })
}

fn detach_nominal_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<NounPhrase> {
    let noun_phrase = match &dependent {
        VerbDependent::DirectObject(noun_phrase)
        | VerbDependent::Temporal(noun_phrase)
        | VerbDependent::Manner(noun_phrase) => noun_phrase.clone(),
        _ => return None,
    };
    (attach_nominal_dependent(predicate, noun_phrase.clone()).ok()? == dependent)
        .then_some(noun_phrase)
}

fn from_preverb_lens_parts(modifiers: Vec<PreverbModifier>, shell: VerbPhrase) -> VerbPhrase {
    VerbPhrase::declaration_from_preverb_projection(shell, modifiers)
}

fn into_preverb_lens_parts(value: VerbPhrase) -> (Vec<PreverbModifier>, VerbPhrase) {
    value.declaration_into_preverb_projection()
}

fn licensed_attachment(
    construction: &'static str,
    predicate: &VerbPhrase,
    attachment: PredicateAttachment,
) -> Result<(), DeclarationViolation> {
    let Some(features) = predicate.declaration_core_features() else {
        return Err(violation(
            construction,
            "the source owner has valid predicate features",
        ));
    };
    extend_predicate_features(&features, attachment)
        .is_some()
        .then_some(())
        .ok_or_else(|| {
            violation(
                construction,
                "the dependent is licensed at the source owner's attachment phase",
            )
        })
}

fn attach_adverb_dependent(
    predicate: &VerbPhrase,
    adverb: Vocab,
) -> Result<VerbDependent, DeclarationViolation> {
    licensed_attachment(
        "verb_phrase_adverb",
        predicate,
        PredicateAttachment::Adjunct,
    )?;
    Ok(VerbDependent::Adverbial(Phrase::Adverb(adverb)))
}

fn detach_adverb_dependent(predicate: &VerbPhrase, dependent: VerbDependent) -> Option<Vocab> {
    let VerbDependent::Adverbial(Phrase::Adverb(adverb)) = dependent else {
        return None;
    };
    attach_adverb_dependent(predicate, adverb).ok()?;
    Some(adverb)
}

fn attach_particle_dependent(
    predicate: &VerbPhrase,
    particle: VerbParticle,
) -> Result<VerbDependent, DeclarationViolation> {
    licensed_attachment(
        "verb_phrase_particle",
        predicate,
        PredicateAttachment::Particle(particle),
    )?;
    Ok(VerbDependent::Particle(particle))
}

fn detach_particle_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<VerbParticle> {
    let VerbDependent::Particle(particle) = dependent else {
        return None;
    };
    attach_particle_dependent(predicate, particle).ok()?;
    Some(particle)
}

fn attach_coin_result_dependent(
    predicate: &VerbPhrase,
    side: CoinSide,
) -> Result<VerbDependent, DeclarationViolation> {
    licensed_attachment(
        "verb_phrase_coin_result",
        predicate,
        PredicateAttachment::CoinResult(side),
    )?;
    Ok(VerbDependent::CoinResult(side))
}

fn detach_coin_result_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<CoinSide> {
    let VerbDependent::CoinResult(side) = dependent else {
        return None;
    };
    attach_coin_result_dependent(predicate, side).ok()?;
    Some(side)
}

fn attach_frequency_dependent(
    predicate: &VerbPhrase,
    frequency: FrequencyPhrase,
) -> Result<VerbDependent, DeclarationViolation> {
    licensed_attachment(
        "verb_phrase_frequency",
        predicate,
        PredicateAttachment::Adjunct,
    )?;
    Ok(VerbDependent::Frequency(frequency))
}

fn detach_frequency_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<FrequencyPhrase> {
    let VerbDependent::Frequency(frequency) = dependent else {
        return None;
    };
    attach_frequency_dependent(predicate, frequency).ok()?;
    Some(frequency)
}

fn prepositional_dependent(
    predicate: &VerbPhrase,
    preposition: PrepositionalPhrase,
) -> Option<VerbDependent> {
    match predicate
        .declaration_frame()
        .prepositional_role(preposition.head().preposition)?
    {
        crate::features::ComplementRole::SelectedComplement => Some(
            VerbDependent::PredicateComplement(Phrase::PrepositionalPhrase(Box::new(preposition))),
        ),
        crate::features::ComplementRole::Adjunct => Some(VerbDependent::Prepositional(preposition)),
    }
}

fn has_shared_determiner_object(preposition: &PrepositionalPhrase) -> bool {
    matches!(
        preposition.as_simple().map(|simple| simple.object.as_ref()),
        Some(Phrase::NounPhrase(object))
            if matches!(object.as_ref(), NounPhrase::CoordinatedNominal(_))
    )
}

fn attach_passive_shared_prepositional_dependent(
    predicate: &VerbPhrase,
    preposition: PrepositionalPhrase,
) -> Result<VerbDependent, DeclarationViolation> {
    let Some(Features::VerbPhrase { passive: true, .. }) = predicate.declaration_core_features()
    else {
        return Err(violation(
            "verb_phrase_passive_shared_determiner_prepositional",
            "the source owner has passive voice",
        ));
    };
    if !has_shared_determiner_object(&preposition) {
        return Err(violation(
            "verb_phrase_passive_shared_determiner_prepositional",
            "the prepositional object shares one determiner",
        ));
    }
    licensed_attachment(
        "verb_phrase_passive_shared_determiner_prepositional",
        predicate,
        PredicateAttachment::Prepositional(preposition.head().preposition),
    )?;
    prepositional_dependent(predicate, preposition).ok_or_else(|| {
        violation(
            "verb_phrase_passive_shared_determiner_prepositional",
            "the preposition has a selected or adjunct role",
        )
    })
}

fn detach_passive_shared_prepositional_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<PrepositionalPhrase> {
    let preposition = match dependent {
        VerbDependent::PredicateComplement(Phrase::PrepositionalPhrase(preposition)) => {
            *preposition
        }
        VerbDependent::Prepositional(preposition) => preposition,
        _ => return None,
    };
    attach_passive_shared_prepositional_dependent(predicate, preposition.clone()).ok()?;
    Some(preposition)
}

fn attach_exception_dependent(
    predicate: &VerbPhrase,
    preposition: PrepositionalPhrase,
) -> Result<VerbDependent, DeclarationViolation> {
    if preposition.head().preposition != crate::syntax::Preposition::By {
        return Err(violation(
            "verb_phrase_except_by",
            "the exception uses the selected `by` form",
        ));
    }
    licensed_attachment(
        "verb_phrase_except_by",
        predicate,
        PredicateAttachment::Exception,
    )?;
    Ok(VerbDependent::Exception(preposition))
}

fn detach_exception_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<PrepositionalPhrase> {
    let VerbDependent::Exception(preposition) = dependent else {
        return None;
    };
    attach_exception_dependent(predicate, preposition.clone()).ok()?;
    Some(preposition)
}

fn make_frequency_phrase(
    frequency: FrequencyPhrase,
) -> Result<FrequencyPhrase, DeclarationViolation> {
    Ok(frequency)
}

fn frequency_phrase_parts(frequency: &FrequencyPhrase) -> FrequencyPhrase {
    *frequency
}

fn make_frequency_phrase_adverb(
    frequency: FrequencyPhrase,
) -> Result<FrequencyPhrase, DeclarationViolation> {
    if frequency.bound != FrequencyBound::MoreThan {
        return Err(violation(
            "frequency_phrase_adverb",
            "the child frequency has the `more than` bound",
        ));
    }
    Ok(FrequencyPhrase {
        bound: FrequencyBound::NoMoreThan,
        count: frequency.count,
    })
}

fn frequency_phrase_adverb_parts(frequency: &FrequencyPhrase) -> FrequencyPhrase {
    FrequencyPhrase {
        bound: FrequencyBound::MoreThan,
        count: frequency.count,
    }
}

fn is_frequency_phrase_adverb(frequency: &FrequencyPhrase) -> bool {
    frequency.bound == FrequencyBound::NoMoreThan
}

fn is_frequency_phrase(frequency: &FrequencyPhrase) -> bool {
    frequency.bound == FrequencyBound::MoreThan
}

fn object_conjunction(
    construction: &'static str,
    conjunction: Conjunction,
) -> Result<Conjunction, DeclarationViolation> {
    matches!(conjunction, Conjunction::And | Conjunction::Or)
        .then_some(conjunction)
        .ok_or_else(|| {
            violation(
                construction,
                "the object conjunction is exactly `and` or `or`",
            )
        })
}

fn is_mana_atom(object: &PredicateObject) -> bool {
    match object {
        PredicateObject::OracleSymbol(_) => true,
        PredicateObject::SymbolSequence(symbols) => !symbols.is_empty(),
        _ => false,
    }
}

fn coordinated_mana_is_valid(value: &CoordinatedPredicateObject) -> bool {
    !value.rest.is_empty()
        && is_mana_atom(value.first.as_ref())
        && value.rest.iter().enumerate().all(|(index, member)| {
            is_mana_atom(&member.object)
                && if index + 1 == value.rest.len() {
                    matches!(member.conjunction, Some(Conjunction::And | Conjunction::Or))
                } else {
                    member.conjunction.is_none()
                }
        })
}

pub(crate) fn declaration_coordination_is_mana(value: &CoordinatedPredicateObject) -> bool {
    coordinated_mana_is_valid(value)
}

pub(crate) fn declaration_coordination_is_quoted(value: &CoordinatedPredicateObject) -> bool {
    matches!(
        value.rest.as_slice(),
        [PredicateObjectCoordination {
            conjunction: Some(Conjunction::And | Conjunction::Or),
            object: PredicateObject::QuotedAbility(_),
        }]
    ) && match value.first.as_ref() {
        PredicateObject::QuotedAbility(_) => true,
        PredicateObject::Ability(AbilityObject {
            ability,
            argument: None,
        }) => ability.is_keyword_ability(),
        _ => false,
    }
}

fn attach_ability_dependent(
    predicate: &VerbPhrase,
    ability: CatalogAtom,
) -> Result<VerbDependent, DeclarationViolation> {
    if !ability.is_keyword_ability() {
        return Err(violation(
            "verb_phrase_ability",
            "the ability identity is a keyword-ability catalog atom",
        ));
    }
    licensed_attachment(
        "verb_phrase_ability",
        predicate,
        PredicateAttachment::AbilityComplement,
    )?;
    Ok(VerbDependent::PredicateComplement(Phrase::CatalogAtom(
        ability,
    )))
}

fn detach_ability_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<CatalogAtom> {
    let VerbDependent::PredicateComplement(Phrase::CatalogAtom(ability)) = dependent else {
        return None;
    };
    attach_ability_dependent(predicate, ability.clone()).ok()?;
    Some(ability)
}

fn attach_quoted_ability_dependent(
    predicate: &VerbPhrase,
    quoted: QuotedAbility,
) -> Result<VerbDependent, DeclarationViolation> {
    licensed_attachment(
        "verb_phrase_quoted_ability",
        predicate,
        PredicateAttachment::QuotedObject,
    )?;
    Ok(VerbDependent::PredicateComplement(Phrase::QuotedAbility(
        Box::new(quoted),
    )))
}

fn detach_quoted_ability_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<QuotedAbility> {
    let VerbDependent::PredicateComplement(Phrase::QuotedAbility(quoted)) = dependent else {
        return None;
    };
    attach_quoted_ability_dependent(predicate, (*quoted).clone()).ok()?;
    Some(*quoted)
}

fn attach_quoted_pair_dependent(
    predicate: &VerbPhrase,
    mut pair: Vec<QuotedAbilityPairMember>,
) -> Result<VerbDependent, DeclarationViolation> {
    let [pair] = pair.as_mut_slice() else {
        return Err(violation(
            "verb_phrase_quoted_ability_coordination",
            "the quoted coordination has exactly two members",
        ));
    };
    let conjunction =
        object_conjunction("verb_phrase_quoted_ability_coordination", pair.conjunction)?;
    licensed_attachment(
        "verb_phrase_quoted_ability_coordination",
        predicate,
        PredicateAttachment::QuotedObject,
    )?;
    Ok(VerbDependent::CoordinatedObject(
        CoordinatedPredicateObject {
            first: Box::new(PredicateObject::QuotedAbility(Box::new(pair.first.clone()))),
            rest: vec![PredicateObjectCoordination {
                conjunction: Some(conjunction),
                object: PredicateObject::QuotedAbility(Box::new(pair.next.clone())),
            }],
        },
    ))
}

fn detach_quoted_pair_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<Vec<QuotedAbilityPairMember>> {
    let VerbDependent::CoordinatedObject(CoordinatedPredicateObject { first, rest }) = dependent
    else {
        return None;
    };
    let PredicateObject::QuotedAbility(first) = *first else {
        return None;
    };
    let [
        PredicateObjectCoordination {
            conjunction: Some(conjunction),
            object: PredicateObject::QuotedAbility(next),
        },
    ] = rest.as_slice()
    else {
        return None;
    };
    let pair = vec![QuotedAbilityPairMember {
        first: *first,
        conjunction: *conjunction,
        next: (**next).clone(),
    }];
    object_conjunction("verb_phrase_quoted_ability_coordination", *conjunction).ok()?;
    licensed_attachment(
        "verb_phrase_quoted_ability_coordination",
        predicate,
        PredicateAttachment::QuotedObject,
    )
    .ok()?;
    Some(pair)
}

fn attach_ability_quoted_pair_dependent(
    predicate: &VerbPhrase,
    mut pair: Vec<AbilityQuotedPairMember>,
) -> Result<VerbDependent, DeclarationViolation> {
    let [pair] = pair.as_mut_slice() else {
        return Err(violation(
            "verb_phrase_ability_quoted_coordination",
            "the mixed coordination has exactly one ability then one quote",
        ));
    };
    let conjunction =
        object_conjunction("verb_phrase_ability_quoted_coordination", pair.conjunction)?;
    if !pair.ability.is_keyword_ability() {
        return Err(violation(
            "verb_phrase_ability_quoted_coordination",
            "the first identity is a keyword-ability catalog atom",
        ));
    }
    licensed_attachment(
        "verb_phrase_ability_quoted_coordination",
        predicate,
        PredicateAttachment::QuotedObject,
    )?;
    Ok(VerbDependent::CoordinatedObject(
        CoordinatedPredicateObject {
            first: Box::new(PredicateObject::Ability(AbilityObject {
                ability: pair.ability.clone(),
                argument: None,
            })),
            rest: vec![PredicateObjectCoordination {
                conjunction: Some(conjunction),
                object: PredicateObject::QuotedAbility(Box::new(pair.quoted.clone())),
            }],
        },
    ))
}

fn detach_ability_quoted_pair_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<Vec<AbilityQuotedPairMember>> {
    let VerbDependent::CoordinatedObject(CoordinatedPredicateObject { first, rest }) = dependent
    else {
        return None;
    };
    let PredicateObject::Ability(AbilityObject {
        ability,
        argument: None,
    }) = *first
    else {
        return None;
    };
    if !ability.is_keyword_ability() {
        return None;
    }
    let [
        PredicateObjectCoordination {
            conjunction: Some(conjunction),
            object: PredicateObject::QuotedAbility(quoted),
        },
    ] = rest.as_slice()
    else {
        return None;
    };
    let pair = vec![AbilityQuotedPairMember {
        ability,
        conjunction: *conjunction,
        quoted: (**quoted).clone(),
    }];
    object_conjunction("verb_phrase_ability_quoted_coordination", *conjunction).ok()?;
    licensed_attachment(
        "verb_phrase_ability_quoted_coordination",
        predicate,
        PredicateAttachment::QuotedObject,
    )
    .ok()?;
    Some(pair)
}

fn attach_oracle_symbol_dependent(
    predicate: &VerbPhrase,
    symbol: OracleSymbol,
) -> Result<VerbDependent, DeclarationViolation> {
    licensed_attachment(
        "verb_phrase_oracle_symbol",
        predicate,
        PredicateAttachment::ScalarComplement,
    )?;
    Ok(VerbDependent::Scalar(Phrase::OracleSymbol(symbol)))
}

fn detach_oracle_symbol_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<OracleSymbol> {
    let VerbDependent::Scalar(Phrase::OracleSymbol(symbol)) = dependent else {
        return None;
    };
    attach_oracle_symbol_dependent(predicate, symbol.clone()).ok()?;
    Some(symbol)
}

fn attach_symbol_sequence_dependent(
    predicate: &VerbPhrase,
    symbols: SymbolSequence,
) -> Result<VerbDependent, DeclarationViolation> {
    if symbols.is_empty() {
        return Err(violation(
            "verb_phrase_symbol_sequence",
            "the symbol sequence is nonempty",
        ));
    }
    licensed_attachment(
        "verb_phrase_symbol_sequence",
        predicate,
        PredicateAttachment::ScalarComplement,
    )?;
    Ok(VerbDependent::Scalar(Phrase::SymbolSequence(symbols)))
}

fn detach_symbol_sequence_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<SymbolSequence> {
    let VerbDependent::Scalar(Phrase::SymbolSequence(symbols)) = dependent else {
        return None;
    };
    attach_symbol_sequence_dependent(predicate, symbols.clone()).ok()?;
    Some(symbols)
}

fn attach_mana_coordination_dependent(
    predicate: &VerbPhrase,
    coordination: CoordinatedManaAmount,
) -> Result<VerbDependent, DeclarationViolation> {
    if !coordinated_mana_is_valid(&coordination) {
        return Err(violation(
            "verb_phrase_mana_amount_coordination",
            "the mana coordination has two or more typed symbol members",
        ));
    }
    licensed_attachment(
        "verb_phrase_mana_amount_coordination",
        predicate,
        PredicateAttachment::ScalarComplement,
    )?;
    Ok(VerbDependent::CoordinatedObject(coordination))
}

fn detach_mana_coordination_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<CoordinatedManaAmount> {
    let VerbDependent::CoordinatedObject(coordination) = dependent else {
        return None;
    };
    attach_mana_coordination_dependent(predicate, coordination.clone()).ok()?;
    Some(coordination)
}

fn attach_power_toughness_dependent(
    predicate: &VerbPhrase,
    value: PowerToughness,
) -> Result<VerbDependent, DeclarationViolation> {
    licensed_attachment(
        "verb_phrase_power_toughness",
        predicate,
        PredicateAttachment::StatisticComplement,
    )?;
    Ok(VerbDependent::Statistic(Phrase::PowerToughness(value)))
}

fn detach_power_toughness_dependent(
    predicate: &VerbPhrase,
    dependent: VerbDependent,
) -> Option<PowerToughness> {
    let VerbDependent::Statistic(Phrase::PowerToughness(value)) = dependent else {
        return None;
    };
    attach_power_toughness_dependent(predicate, value).ok()?;
    Some(value)
}

fn attach_quantity_dependent(
    predicate: &VerbPhrase,
    quantity: Quantity,
) -> Result<VerbDependent, DeclarationViolation> {
    licensed_attachment(
        "verb_phrase_quantity",
        predicate,
        PredicateAttachment::ScalarOrAbilityArgument,
    )?;
    Ok(VerbDependent::Scalar(Phrase::Quantity(quantity)))
}

fn detach_quantity_dependent(predicate: &VerbPhrase, dependent: VerbDependent) -> Option<Quantity> {
    let VerbDependent::Scalar(Phrase::Quantity(quantity)) = dependent else {
        return None;
    };
    attach_quantity_dependent(predicate, quantity).ok()?;
    Some(quantity)
}

fn make_mana_amount_symbol(symbol: OracleSymbol) -> Result<ManaAmount, DeclarationViolation> {
    Ok(PredicateObject::OracleSymbol(symbol))
}

fn mana_amount_symbol_parts(value: &ManaAmount) -> OracleSymbol {
    let PredicateObject::OracleSymbol(symbol) = value else {
        unreachable!("mana_amount_symbol dispatcher admits only one symbol")
    };
    symbol.clone()
}

fn is_mana_amount_symbol(value: &ManaAmount) -> bool {
    matches!(value, PredicateObject::OracleSymbol(_))
}

fn make_mana_amount_sequence(symbols: SymbolSequence) -> Result<ManaAmount, DeclarationViolation> {
    if symbols.is_empty() {
        return Err(violation(
            "mana_amount_sequence",
            "the symbol sequence is nonempty",
        ));
    }
    Ok(PredicateObject::SymbolSequence(symbols))
}

fn mana_amount_sequence_parts(value: &ManaAmount) -> SymbolSequence {
    let PredicateObject::SymbolSequence(symbols) = value else {
        unreachable!("mana_amount_sequence dispatcher admits only symbol groups")
    };
    symbols.clone()
}

fn is_mana_amount_sequence(value: &ManaAmount) -> bool {
    matches!(value, PredicateObject::SymbolSequence(symbols) if !symbols.is_empty())
}

fn make_mana_amount_list_single(
    amount: ManaAmount,
) -> Result<ManaAmountList, DeclarationViolation> {
    is_mana_atom(&amount).then_some(amount).ok_or_else(|| {
        violation(
            "mana_amount_list_single",
            "the list has exactly one typed mana amount",
        )
    })
}

fn mana_amount_list_single_parts(value: &ManaAmountList) -> ManaAmount {
    value.clone()
}

fn is_mana_amount_list_single(value: &ManaAmountList) -> bool {
    is_mana_atom(value)
}

fn make_mana_amount_list_comma(
    first: ManaAmount,
    rest: Vec<PredicateObjectCoordination>,
) -> Result<ManaAmountList, DeclarationViolation> {
    if !is_mana_atom(&first)
        || rest.is_empty()
        || rest
            .iter()
            .any(|member| member.conjunction.is_some() || !is_mana_atom(&member.object))
    {
        return Err(violation(
            "mana_amount_list_comma",
            "the open list has at least two comma-separated mana amounts",
        ));
    }
    Ok(PredicateObject::Coordinated(CoordinatedPredicateObject {
        first: Box::new(first),
        rest,
    }))
}

fn mana_amount_list_comma_parts(
    value: &ManaAmountList,
) -> (ManaAmount, Vec<PredicateObjectCoordination>) {
    let PredicateObject::Coordinated(coordination) = value else {
        unreachable!("mana_amount_list_comma dispatcher admits only open lists")
    };
    ((*coordination.first).clone(), coordination.rest.clone())
}

fn is_mana_amount_list_comma(value: &ManaAmountList) -> bool {
    matches!(
        value,
        PredicateObject::Coordinated(CoordinatedPredicateObject { first, rest })
            if is_mana_atom(first)
                && !rest.is_empty()
                && rest.iter().all(|member| member.conjunction.is_none() && is_mana_atom(&member.object))
    )
}

fn make_mana_amount_coordination(
    first: ManaAmount,
    conjunction: Conjunction,
    next: ManaAmount,
) -> Result<CoordinatedManaAmount, DeclarationViolation> {
    if !is_mana_atom(&first) || !is_mana_atom(&next) {
        return Err(violation(
            "mana_amount_coordination",
            "the binary coordination has exactly two mana amounts",
        ));
    }
    Ok(CoordinatedPredicateObject {
        first: Box::new(first),
        rest: vec![PredicateObjectCoordination {
            conjunction: Some(object_conjunction("mana_amount_coordination", conjunction)?),
            object: next,
        }],
    })
}

fn mana_amount_coordination_parts(
    value: &CoordinatedManaAmount,
) -> (ManaAmount, Conjunction, ManaAmount) {
    let [member] = value.rest.as_slice() else {
        unreachable!("mana_amount_coordination dispatcher admits only binary values")
    };
    (
        (*value.first).clone(),
        member
            .conjunction
            .expect("the binary member has a conjunction"),
        member.object.clone(),
    )
}

fn is_mana_amount_coordination(value: &CoordinatedManaAmount) -> bool {
    coordinated_mana_is_valid(value) && value.rest.len() == 1
}

fn make_mana_amount_coordination_oxford(
    list: ManaAmountList,
    conjunction: Conjunction,
    next: ManaAmount,
) -> Result<CoordinatedManaAmount, DeclarationViolation> {
    let PredicateObject::Coordinated(mut coordination) = list else {
        return Err(violation(
            "mana_amount_coordination_oxford",
            "the Oxford prefix contains at least two comma-separated members",
        ));
    };
    if !is_mana_atom(coordination.first.as_ref())
        || coordination.rest.is_empty()
        || coordination
            .rest
            .iter()
            .any(|member| member.conjunction.is_some() || !is_mana_atom(&member.object))
        || !is_mana_atom(&next)
    {
        return Err(violation(
            "mana_amount_coordination_oxford",
            "the Oxford list has a comma prefix and final mana member",
        ));
    }
    coordination.rest.push(PredicateObjectCoordination {
        conjunction: Some(object_conjunction(
            "mana_amount_coordination_oxford",
            conjunction,
        )?),
        object: next,
    });
    Ok(coordination)
}

fn mana_amount_coordination_oxford_parts(
    value: &CoordinatedManaAmount,
) -> (ManaAmountList, Conjunction, ManaAmount) {
    let mut prefix = value.clone();
    let final_member = prefix
        .rest
        .pop()
        .expect("the Oxford dispatcher admits a final member");
    (
        PredicateObject::Coordinated(prefix),
        final_member
            .conjunction
            .expect("the Oxford final member has a conjunction"),
        final_member.object,
    )
}

fn is_mana_amount_coordination_oxford(value: &CoordinatedManaAmount) -> bool {
    coordinated_mana_is_valid(value) && value.rest.len() >= 2
}

fn dependent_matches(value: &VerbPhrase, matches: impl FnOnce(&VerbDependent) -> bool) -> bool {
    value
        .declaration_last_dependent_parts()
        .is_some_and(|(_, dependent)| matches(&dependent))
        && value.declaration_core_features().is_some()
}

fn is_verb_phrase_ability(value: &VerbPhrase) -> bool {
    dependent_matches(value, |dependent| {
        matches!(
            dependent,
            VerbDependent::PredicateComplement(Phrase::CatalogAtom(ability))
                if ability.is_keyword_ability()
        )
    })
}

fn is_verb_phrase_quoted_ability(value: &VerbPhrase) -> bool {
    dependent_matches(value, |dependent| {
        matches!(
            dependent,
            VerbDependent::PredicateComplement(Phrase::QuotedAbility(_))
        )
    })
}

fn is_verb_phrase_quoted_ability_coordination(value: &VerbPhrase) -> bool {
    dependent_matches(value, |dependent| {
        matches!(
            dependent,
            VerbDependent::CoordinatedObject(CoordinatedPredicateObject { first, rest })
                if matches!(first.as_ref(), PredicateObject::QuotedAbility(_))
                    && matches!(rest.as_slice(), [PredicateObjectCoordination {
                        conjunction: Some(Conjunction::And | Conjunction::Or),
                        object: PredicateObject::QuotedAbility(_),
                    }])
        )
    })
}

fn is_verb_phrase_ability_quoted_coordination(value: &VerbPhrase) -> bool {
    dependent_matches(value, |dependent| {
        matches!(
            dependent,
            VerbDependent::CoordinatedObject(CoordinatedPredicateObject { first, rest })
                if matches!(
                    first.as_ref(),
                    PredicateObject::Ability(AbilityObject { ability, argument: None })
                        if ability.is_keyword_ability()
                )
                    && matches!(rest.as_slice(), [PredicateObjectCoordination {
                        conjunction: Some(Conjunction::And | Conjunction::Or),
                        object: PredicateObject::QuotedAbility(_),
                    }])
        )
    })
}

fn is_verb_phrase_oracle_symbol(value: &VerbPhrase) -> bool {
    dependent_matches(value, |dependent| {
        matches!(dependent, VerbDependent::Scalar(Phrase::OracleSymbol(_)))
    })
}

fn is_verb_phrase_symbol_sequence(value: &VerbPhrase) -> bool {
    dependent_matches(value, |dependent| {
        matches!(
            dependent,
            VerbDependent::Scalar(Phrase::SymbolSequence(symbols)) if !symbols.is_empty()
        )
    })
}

fn is_verb_phrase_mana_amount_coordination(value: &VerbPhrase) -> bool {
    dependent_matches(value, |dependent| {
        matches!(
            dependent,
            VerbDependent::CoordinatedObject(coordination) if coordinated_mana_is_valid(coordination)
        )
    })
}

fn is_verb_phrase_power_toughness(value: &VerbPhrase) -> bool {
    dependent_matches(value, |dependent| {
        matches!(
            dependent,
            VerbDependent::Statistic(Phrase::PowerToughness(_))
        )
    })
}

fn is_verb_phrase_quantity(value: &VerbPhrase) -> bool {
    dependent_matches(value, |dependent| {
        matches!(dependent, VerbDependent::Scalar(Phrase::Quantity(_)))
    })
}

fn is_verb_phrase_direct_object(value: &VerbPhrase) -> bool {
    matches!(
        value.declaration_last_dependent_parts(),
        Some((
            _,
            VerbDependent::DirectObject(ref dependent)
                | VerbDependent::Temporal(ref dependent)
                | VerbDependent::Manner(ref dependent)
        ))
            if noun_phrase_value_is_object_case(dependent)
    ) && value.declaration_core_features().is_some()
}
dependent_lens!(
    from_indirect_object_lens_parts,
    into_indirect_object_lens_parts,
    is_verb_phrase_indirect_object,
    IndirectObject,
    NounPhrase,
    noun_phrase_value_is_object_case
);
dependent_lens!(
    from_infinitive_lens_parts,
    into_infinitive_lens_parts,
    is_verb_phrase_infinitive,
    Infinitive,
    InfinitiveClause,
    |infinitive: &InfinitiveClause| infinitive.declaration_bare_predicate().is_none()
);

fn from_adjective_lens_parts(
    mut before: Vec<VerbDependent>,
    adjective: Option<AdjectivePhrase>,
    after: Vec<VerbDependent>,
    shell: VerbPhrase,
) -> VerbPhrase {
    if let Some(adjective) = adjective {
        before.push(VerbDependent::Adverbial(Phrase::AdjectivePhrase(Box::new(
            adjective,
        ))));
    }
    before.extend(after);
    VerbPhrase::declaration_from_dependent_projection(shell, before)
}

fn into_adjective_lens_parts(
    value: VerbPhrase,
) -> (
    Vec<VerbDependent>,
    Option<AdjectivePhrase>,
    Vec<VerbDependent>,
    VerbPhrase,
) {
    let (mut dependents, shell) = value.declaration_into_dependent_projection();
    let Some(index) = dependents.iter().rposition(|dependent| {
        matches!(
            dependent,
            VerbDependent::Adverbial(Phrase::AdjectivePhrase(_))
        )
    }) else {
        return (dependents, None, Vec::new(), shell);
    };
    let after = dependents.split_off(index + 1);
    let VerbDependent::Adverbial(Phrase::AdjectivePhrase(adjective)) = dependents.remove(index)
    else {
        unreachable!("the adjective lens index pins its variant")
    };
    (dependents, Some(*adjective), after, shell)
}

fn is_verb_phrase_adjective(value: &VerbPhrase) -> bool {
    matches!(
        value.declaration_last_dependent_parts(),
        Some((_, VerbDependent::Adverbial(Phrase::AdjectivePhrase(_))))
    ) && value.declaration_core_features().is_some()
}

fn from_prepositional_lens_parts(
    mut before: Vec<VerbDependent>,
    preposition: Option<PrepositionalPhrase>,
    after: Vec<VerbDependent>,
    shell: VerbPhrase,
) -> VerbPhrase {
    if let Some(preposition) = preposition {
        let dependent = match shell
            .declaration_frame()
            .prepositional_role(preposition.head().preposition)
        {
            Some(crate::features::ComplementRole::SelectedComplement) => {
                VerbDependent::PredicateComplement(Phrase::PrepositionalPhrase(Box::new(
                    preposition,
                )))
            }
            Some(crate::features::ComplementRole::Adjunct) => {
                VerbDependent::Prepositional(preposition)
            }
            None => VerbDependent::Prepositional(preposition),
        };
        before.push(dependent);
    }
    before.extend(after);
    VerbPhrase::declaration_from_dependent_projection(shell, before)
}

fn into_prepositional_lens_parts(
    value: VerbPhrase,
) -> (
    Vec<VerbDependent>,
    Option<PrepositionalPhrase>,
    Vec<VerbDependent>,
    VerbPhrase,
) {
    let (mut dependents, shell) = value.declaration_into_dependent_projection();
    let Some(index) = dependents.iter().rposition(|dependent| {
        matches!(
            dependent,
            VerbDependent::PredicateComplement(Phrase::PrepositionalPhrase(_))
                | VerbDependent::Prepositional(_)
        )
    }) else {
        return (dependents, None, Vec::new(), shell);
    };
    let after = dependents.split_off(index + 1);
    let preposition = match dependents.remove(index) {
        VerbDependent::PredicateComplement(Phrase::PrepositionalPhrase(preposition)) => {
            *preposition
        }
        VerbDependent::Prepositional(preposition) => preposition,
        _ => unreachable!("the prepositional lens index pins its variant"),
    };
    (dependents, Some(preposition), after, shell)
}

fn is_verb_phrase_prepositional(value: &VerbPhrase) -> bool {
    matches!(
        value.declaration_last_dependent_parts(),
        Some((
            _,
            VerbDependent::PredicateComplement(Phrase::PrepositionalPhrase(_))
                | VerbDependent::Prepositional(_)
        ))
    ) && !is_passive_shared_prepositional(value)
        && value.declaration_core_features().is_some()
}

fn is_verb_phrase_adverb(value: &VerbPhrase) -> bool {
    matches!(
        value.declaration_last_dependent_parts(),
        Some((_, VerbDependent::Adverbial(Phrase::Adverb(_))))
    ) && value.declaration_core_features().is_some()
}

fn is_verb_phrase_preverb_adverb(value: &VerbPhrase) -> bool {
    value.declaration_first_preverb_modifier() == Some(PreverbModifier::Next)
        && value.declaration_core_features().is_some()
}

fn is_verb_phrase_particle(value: &VerbPhrase) -> bool {
    matches!(
        value.declaration_last_dependent_parts(),
        Some((_, VerbDependent::Particle(_)))
    ) && value.declaration_core_features().is_some()
}

fn is_verb_phrase_coin_result(value: &VerbPhrase) -> bool {
    matches!(
        value.declaration_last_dependent_parts(),
        Some((_, VerbDependent::CoinResult(_)))
    ) && value.declaration_core_features().is_some()
}

fn is_verb_phrase_frequency(value: &VerbPhrase) -> bool {
    matches!(
        value.declaration_last_dependent_parts(),
        Some((_, VerbDependent::Frequency(_)))
    ) && value.declaration_core_features().is_some()
}

fn is_passive_shared_prepositional(value: &VerbPhrase) -> bool {
    let Some((predicate, dependent)) = value.declaration_last_dependent_parts() else {
        return false;
    };
    detach_passive_shared_prepositional_dependent(&predicate, dependent).is_some()
}

fn is_exception(value: &VerbPhrase) -> bool {
    let Some((predicate, dependent)) = value.declaration_last_dependent_parts() else {
        return false;
    };
    detach_exception_dependent(&predicate, dependent).is_some()
}

deckmaste_constructions_macro::constructions! {
    group predicate;

    element quoted_ability_pair_member {
        first: identity QuotedAbility via QuotedAbility,
        conjunction: lex Conjunction,
        next: identity QuotedAbility via QuotedAbility,
    }

    element ability_quoted_pair_member {
        ability: identity CatalogAtom via AbilityItem,
        conjunction: lex Conjunction,
        quoted: identity QuotedAbility via QuotedAbility,
    }

    element mana_amount_list_member bind PredicateObjectCoordination {
        comma: surface lex Comma,
        conjunction: opt lex Conjunction,
        object: hole ManaAmount,
    }

    lens verb_phrase_dependents bind VerbPhrase via from_dependent_lens_parts, into_dependent_lens_parts {
        dependents: vec VerbDependent,
        shell: value VerbPhrase,
    }

    lens verb_phrase_preverbs bind VerbPhrase via from_preverb_lens_parts, into_preverb_lens_parts {
        modifiers: vec PreverbModifier,
        shell: value VerbPhrase,
    }

    lens verb_phrase_indirect_object bind VerbPhrase via from_indirect_object_lens_parts, into_indirect_object_lens_parts {
        before: vec VerbDependent,
        object: opt NounPhrase,
        after: vec VerbDependent,
        shell: value VerbPhrase,
    }

    lens verb_phrase_adjective bind VerbPhrase via from_adjective_lens_parts, into_adjective_lens_parts {
        before: vec VerbDependent,
        adjective: opt AdjectivePhrase,
        after: vec VerbDependent,
        shell: value VerbPhrase,
    }

    lens verb_phrase_prepositional bind VerbPhrase via from_prepositional_lens_parts, into_prepositional_lens_parts {
        before: vec VerbDependent,
        preposition: opt PrepositionalPhrase,
        after: vec VerbDependent,
        shell: value VerbPhrase,
    }

    lens verb_phrase_infinitive bind VerbPhrase via from_infinitive_lens_parts, into_infinitive_lens_parts {
        before: vec VerbDependent,
        infinitive: opt InfinitiveClause,
        after: vec VerbDependent,
        shell: value VerbPhrase,
    }

    construction verb: Verb {
        bind VerbAnalysis via make_verb, verb_parts {
            head: identity VerbAnalysis via LexicalVerb,
        }
        derive features: Features = reduce_verb_features(head);
        form only @ 0 inverse check(is_verb) = identity(head);
        selection unique;
    }

    construction verb_phrase_base: VerbPhrase {
        bind VerbPhrase via make_verb_phrase_base, verb_phrase_base_parts {
            head: hole Verb,
        }
        derive features: Features = reduce_verb_phrase_base(head);
        derive argument_complete: Features = complete_base(head);
        derive object_gap: Features = reduce_object_gap_base(head);
        derive reduced_passive: Features = reduce_reduced_passive_base(head);
        form only @ 0 inverse check(is_verb_phrase_base) = head;
        dominates verb_phrase_auxiliary_proform;
        selection unique;
    }

    construction verb_phrase_auxiliary: VerbPhrase {
        bind VerbPhrase via make_verb_phrase_auxiliary, verb_phrase_auxiliary_parts {
            auxiliary: identity AuxiliaryInstance via Auxiliary,
            predicate: hole VerbPhrase,
        }
        derive features: Features = reduce_verb_phrase_auxiliary_features(auxiliary, predicate);
        derive argument_complete: Features = complete_auxiliary(auxiliary, predicate);
        derive object_gap: Features = reduce_object_gap_auxiliary(auxiliary, predicate);
        evidence feature "predicate voice" from category;
        form only @ 0 inverse check(is_verb_phrase_auxiliary) = identity(auxiliary) predicate;
        dominates verb_phrase_adjective;
        dominates verb_phrase_adverb;
        dominates verb_phrase_ability;
        selection unique;
    }

    construction verb_phrase_auxiliary_proform: VerbPhrase {
        bind VerbPhrase via make_verb_phrase_auxiliary_proform, verb_phrase_auxiliary_proform_parts {
            auxiliary: identity AuxiliaryInstance via Auxiliary,
        }
        derive features: Features = reduce_verb_phrase_auxiliary_proform_features(auxiliary);
        derive argument_complete: Features = complete_auxiliary_proform(auxiliary);
        form only @ 0 inverse check(is_verb_phrase_auxiliary_proform) = identity(auxiliary);
        selection unique;
    }

    construction verb_phrase_direct_object: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            object: hole NounPhrase,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with object via attach_nominal_dependent, detach_nominal_dependent;
        }
        derive prefix_admission: Features = admit_direct_object_prefix(predicate);
        derive features: Features = reduce_verb_phrase_direct_object_features(predicate, object);
        derive argument_complete: Features = complete_direct_object(predicate, object);
        derive reduced_passive: Features = reduce_reduced_passive_direct(predicate, object);
        derive precedence: Features = disprefer_active_temporal_attachment(predicate, object);
        evidence role "predicate object role" from category;
        form only @ 0 inverse check(is_verb_phrase_direct_object) = predicate object;
        selection unique;
    }

    construction verb_phrase_indirect_object: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            object: hole NounPhrase,
        }
        lens verb_phrase_indirect_object from predicate {
            focus object with object;
        }
        derive prefix_admission: Features = admit_indirect_object_prefix(predicate);
        derive features: Features = reduce_verb_phrase_indirect_object_features(predicate, object);
        derive argument_complete: Features = complete_indirect_object(predicate, object);
        derive object_gap: Features = reduce_object_gap_indirect(predicate, object);
        form only @ 0 inverse check(is_verb_phrase_indirect_object) = predicate object;
        selection unique;
    }

    construction verb_phrase_adjective: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            adjective: hole AdjectivePhrase,
        }
        lens verb_phrase_adjective from predicate {
            focus adjective with adjective;
        }
        derive prefix_admission: Features = admit_adjective_prefix(predicate);
        derive features: Features = reduce_verb_phrase_adjective_features(predicate, adjective);
        derive argument_complete: Features = complete_adjective(predicate, adjective);
        derive object_gap: Features = reduce_object_gap_adjective(predicate, adjective);
        derive base_precedence_2: Features = reduce_verb_phrase_adjective_features(predicate, adjective);
        evidence feature "predicate local cost" from category;
        form only @ 0 inverse check(is_verb_phrase_adjective) = predicate adjective;
        selection unique;
    }

    construction verb_phrase_prepositional: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            preposition: hole PrepositionalPhrase,
        }
        lens verb_phrase_prepositional from predicate {
            focus preposition with preposition;
        }
        derive features: Features = reduce_verb_phrase_prepositional_features(predicate, preposition);
        derive argument_complete: Features = complete_prepositional(predicate, preposition);
        derive object_gap: Features = reduce_object_gap_prepositional(predicate, preposition);
        derive reduced_passive: Features = reduce_reduced_passive_prepositional(predicate, preposition);
        derive base_precedence: Features = reduce_verb_phrase_prepositional_features(predicate, preposition);
        form only @ 0 inverse check(is_verb_phrase_prepositional) = predicate preposition;
        selection unique;
    }

    construction verb_phrase_passive_shared_determiner_prepositional: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            preposition: hole PrepositionalPhrase,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with preposition via attach_passive_shared_prepositional_dependent, detach_passive_shared_prepositional_dependent;
        }
        derive prefix_admission: Features = admit_passive_shared_prepositional_prefix(predicate);
        derive features: Features = reduce_passive_shared_prepositional_features(predicate, preposition);
        derive argument_complete: Features = complete_passive_shared_prepositional(predicate, preposition);
        form only @ 0 inverse check(is_passive_shared_prepositional) = predicate preposition;
        selection unique;
    }

    construction verb_phrase_except_by: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            preposition: hole PrepositionalPhrase,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with preposition via attach_exception_dependent, detach_exception_dependent;
        }
        derive prefix_admission: Features = admit_exception_prefix(predicate);
        derive features: Features = reduce_exception_features(predicate, preposition);
        derive argument_complete: Features = complete_exception(predicate, preposition);
        form only @ 0 inverse check(is_exception) = predicate "except" preposition;
        selection unique;
    }

    construction verb_phrase_infinitive: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            infinitive: hole InfinitiveClause,
        }
        lens verb_phrase_infinitive from predicate {
            focus infinitive with infinitive;
        }
        derive prefix_admission: Features = admit_infinitive_prefix(predicate);
        derive features: Features = reduce_verb_phrase_infinitive_features(predicate, infinitive);
        derive argument_complete: Features = complete_infinitive(predicate, infinitive);
        derive object_gap: Features = reduce_object_gap_infinitive(predicate, infinitive);
        form only @ 0 inverse check(is_verb_phrase_infinitive) = predicate infinitive;
        selection unique;
    }

    construction verb_phrase_adverb: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            adverb: identity Vocab via Adverb,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with adverb via attach_adverb_dependent, detach_adverb_dependent;
        }
        derive features: Features = reduce_verb_phrase_adverb_features(predicate, adverb);
        derive argument_complete: Features = complete_adverb(predicate, adverb);
        derive object_gap: Features = reduce_verb_phrase_adverb_features(predicate, adverb);
        derive reduced_passive: Features = reduce_verb_phrase_adverb_features(predicate, adverb);
        evidence feature "predicate attachment phase" from category;
        form only @ 0 inverse check(is_verb_phrase_adverb) = predicate identity(adverb);
        selection unique;
    }

    construction verb_phrase_preverb_adverb: VerbPhrase {
        bind VerbPhrase {
            modifier: identity PreverbModifier via PreverbAdverb,
            predicate: hole VerbPhrase,
        }
        lens verb_phrase_preverbs from predicate {
            prepend modifiers with modifier;
        }
        derive features: Features = reduce_verb_phrase_preverb_adverb_features(modifier, predicate);
        derive argument_complete: Features = complete_preverb_adverb(modifier, predicate);
        form only @ 0 inverse check(is_verb_phrase_preverb_adverb) = identity(modifier) predicate;
        selection unique;
    }

    construction verb_phrase_particle: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            particle: identity VerbParticle via VerbParticle,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with particle via attach_particle_dependent, detach_particle_dependent;
        }
        derive prefix_admission: Features = admit_particle_prefix(predicate);
        derive features: Features = reduce_verb_phrase_particle_features(predicate, particle);
        derive argument_complete: Features = complete_particle(predicate, particle);
        derive object_gap: Features = reduce_verb_phrase_particle_features(predicate, particle);
        form only @ 0 inverse check(is_verb_phrase_particle) = predicate identity(particle);
        selection unique;
    }

    construction verb_phrase_coin_result: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            side: identity CoinSide via CoinResult,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with side via attach_coin_result_dependent, detach_coin_result_dependent;
        }
        derive prefix_admission: Features = admit_coin_result_prefix(predicate);
        derive features: Features = reduce_verb_phrase_coin_result_features(predicate, side);
        derive argument_complete: Features = complete_coin_result(predicate, side);
        form only @ 0 inverse check(is_verb_phrase_coin_result) = predicate identity(side);
        selection unique;
    }

    construction verb_phrase_frequency: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            frequency: hole FrequencyPhrase,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with frequency via attach_frequency_dependent, detach_frequency_dependent;
        }
        derive features: Features = reduce_verb_phrase_frequency_features(predicate, frequency);
        derive argument_complete: Features = complete_frequency(predicate, frequency);
        derive object_gap: Features = reduce_verb_phrase_frequency_features(predicate, frequency);
        derive reduced_passive: Features = reduce_verb_phrase_frequency_features(predicate, frequency);
        derive base_attachment_count: Features = mark_generated_attachment_cost(predicate);
        form only @ 0 inverse check(is_verb_phrase_frequency) = predicate frequency;
        selection unique;
    }

    construction frequency_phrase_adverb: FrequencyPhrase {
        bind FrequencyPhrase via make_frequency_phrase_adverb, frequency_phrase_adverb_parts {
            frequency: hole FrequencyPhrase,
        }
        derive features: Features = reduce_frequency_phrase_adverb_features(frequency);
        form only @ 0 inverse check(is_frequency_phrase_adverb) = "no" frequency;
        selection unique;
    }

    construction frequency_phrase: FrequencyPhrase {
        bind FrequencyPhrase via make_frequency_phrase, frequency_phrase_parts {
            frequency: identity FrequencyPhrase via Frequency,
        }
        derive features: Features = reduce_frequency_phrase_features(frequency);
        form only @ 0 inverse check(is_frequency_phrase) = identity(frequency);
        selection unique;
    }

    construction verb_phrase_ability: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            ability: identity CatalogAtom via AbilityItem,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with ability via attach_ability_dependent, detach_ability_dependent;
        }
        derive prefix_admission: Features = admit_ability_prefix(predicate);
        derive features: Features = reduce_verb_phrase_ability_features(predicate, ability);
        derive argument_complete: Features = complete_ability(predicate, ability);
        form only @ 0 inverse check(is_verb_phrase_ability) = predicate identity(ability);
        selection unique;
    }

    construction verb_phrase_quoted_ability: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            quoted: identity QuotedAbility via QuotedAbility,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with quoted via attach_quoted_ability_dependent, detach_quoted_ability_dependent;
        }
        derive prefix_admission: Features = admit_quoted_ability_prefix(predicate);
        derive features: Features = reduce_verb_phrase_quoted_ability_features(predicate, quoted);
        derive argument_complete: Features = complete_quoted_ability(predicate, quoted);
        form only @ 0 inverse check(is_verb_phrase_quoted_ability) = predicate identity(quoted);
        selection unique;
    }

    construction verb_phrase_quoted_ability_coordination: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            pair: seq quoted_ability_pair_member,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with pair via attach_quoted_pair_dependent, detach_quoted_pair_dependent;
        }
        require pair.len() == 1;
        require pair.last.conjunction in [And, Or];
        derive prefix_admission: Features = admit_quoted_ability_prefix(predicate);
        derive features: Features = reduce_verb_phrase_quoted_coordination_features(predicate, pair);
        derive argument_complete: Features = complete_quoted_coordination(predicate, pair);
        form only @ 0 inverse check(is_verb_phrase_quoted_ability_coordination) = predicate pair;
        selection unique;
    }

    construction verb_phrase_ability_quoted_coordination: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            pair: seq ability_quoted_pair_member,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with pair via attach_ability_quoted_pair_dependent, detach_ability_quoted_pair_dependent;
        }
        require pair.len() == 1;
        require pair.last.conjunction in [And, Or];
        derive prefix_admission: Features = admit_quoted_ability_prefix(predicate);
        derive features: Features = reduce_verb_phrase_quoted_coordination_features(predicate, pair);
        derive argument_complete: Features = complete_quoted_coordination(predicate, pair);
        form only @ 0 inverse check(is_verb_phrase_ability_quoted_coordination) = predicate pair;
        selection unique;
    }

    construction verb_phrase_oracle_symbol: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            symbol: identity OracleSymbol via OracleSymbol,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with symbol via attach_oracle_symbol_dependent, detach_oracle_symbol_dependent;
        }
        derive prefix_admission: Features = admit_scalar_prefix(predicate);
        derive features: Features = reduce_verb_phrase_scalar_features(predicate, symbol);
        derive argument_complete: Features = complete_scalar(predicate, symbol);
        form only @ 0 inverse check(is_verb_phrase_oracle_symbol) = predicate identity(symbol);
        selection unique;
    }

    construction verb_phrase_symbol_sequence: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            symbols: identity SymbolSequence via SymbolSequence,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with symbols via attach_symbol_sequence_dependent, detach_symbol_sequence_dependent;
        }
        derive prefix_admission: Features = admit_scalar_prefix(predicate);
        derive features: Features = reduce_verb_phrase_scalar_features(predicate, symbols);
        derive argument_complete: Features = complete_scalar(predicate, symbols);
        form only @ 0 inverse check(is_verb_phrase_symbol_sequence) = predicate identity(symbols);
        selection unique;
    }

    construction mana_amount_symbol: ManaAmount {
        bind ManaAmount via make_mana_amount_symbol, mana_amount_symbol_parts {
            symbol: identity OracleSymbol via OracleSymbol,
        }
        form only @ 0 inverse check(is_mana_amount_symbol) = identity(symbol);
        selection unique;
    }

    construction mana_amount_sequence: ManaAmount {
        bind ManaAmount via make_mana_amount_sequence, mana_amount_sequence_parts {
            symbols: identity SymbolSequence via SymbolSequence,
        }
        form only @ 0 inverse check(is_mana_amount_sequence) = identity(symbols);
        selection unique;
    }

    construction mana_amount_list_single: ManaAmountList {
        bind ManaAmountList via make_mana_amount_list_single, mana_amount_list_single_parts {
            amount: hole ManaAmount,
        }
        form only @ 0 inverse check(is_mana_amount_list_single) = amount;
        selection unique;
    }

    construction mana_amount_list_comma: ManaAmountList {
        bind ManaAmountList via make_mana_amount_list_comma, mana_amount_list_comma_parts {
            first: hole ManaAmount,
            rest: seq mana_amount_list_member,
        }
        require rest.len() >= 1;
        require rest.nonfinal.conjunction.is_none();
        require rest.last.conjunction.is_none();
        form only @ 0 inverse check(is_mana_amount_list_comma) = first rest;
        selection unique;
    }

    construction mana_amount_coordination: CoordinatedManaAmount {
        bind CoordinatedManaAmount via make_mana_amount_coordination, mana_amount_coordination_parts {
            first: hole ManaAmount,
            conjunction: lex Conjunction,
            next: hole ManaAmount,
        }
        require conjunction in [And, Or];
        form only @ 0 inverse check(is_mana_amount_coordination) = first lex(conjunction) next;
        selection unique;
    }

    construction mana_amount_coordination_oxford: CoordinatedManaAmount {
        bind CoordinatedManaAmount via make_mana_amount_coordination_oxford, mana_amount_coordination_oxford_parts {
            list: hole ManaAmountList,
            conjunction: lex Conjunction,
            next: hole ManaAmount,
        }
        require conjunction in [And, Or];
        form only @ 0 inverse check(is_mana_amount_coordination_oxford) = list "," lex(conjunction) next;
        selection unique;
    }

    construction verb_phrase_mana_amount_coordination: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            coordination: hole CoordinatedManaAmount,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with coordination via attach_mana_coordination_dependent, detach_mana_coordination_dependent;
        }
        derive prefix_admission: Features = admit_scalar_prefix(predicate);
        derive features: Features = reduce_verb_phrase_mana_coordination_features(predicate, coordination);
        derive argument_complete: Features = complete_mana_coordination(predicate, coordination);
        form only @ 0 inverse check(is_verb_phrase_mana_amount_coordination) = predicate coordination;
        selection unique;
    }

    construction verb_phrase_power_toughness: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            stats: lex PowerToughness via PowerToughness,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with stats via attach_power_toughness_dependent, detach_power_toughness_dependent;
        }
        derive prefix_admission: Features = admit_power_toughness_prefix(predicate);
        derive features: Features = reduce_verb_phrase_power_toughness_features(predicate, stats);
        derive argument_complete: Features = complete_power_toughness(predicate, stats);
        form only @ 0 inverse check(is_verb_phrase_power_toughness) = predicate lex(stats);
        selection unique;
    }

    construction verb_phrase_quantity: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            quantity: hole Quantity,
        }
        lens verb_phrase_dependents from predicate {
            append dependents with quantity via attach_quantity_dependent, detach_quantity_dependent;
        }
        derive prefix_admission: Features = admit_quantity_prefix(predicate);
        derive features: Features = reduce_verb_phrase_quantity_features(predicate, quantity);
        derive argument_complete: Features = complete_quantity(predicate, quantity);
        form only @ 0 inverse check(is_verb_phrase_quantity) = predicate quantity;
        selection unique;
    }

    construction verb_phrase_causative: VerbPhrase {
        bind VerbPhrase via make_verb_phrase_causative, verb_phrase_causative_parts {
            host: hole VerbPhrase,
            complement: hole VerbPhrase,
        }
        derive prefix_admission: Features = causative_host_features(host);
        derive features: Features = reduce_verb_phrase_causative_features(host, complement);
        derive argument_complete: Features = complete_causative(host, complement);
        evidence role "causative host-causee-complement order" from category;
        form only @ 0 inverse check(is_verb_phrase_causative) = host complement;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&PREDICATE_DECLARATION];

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;
    use crate::CatalogKind;
    use crate::catalog::CatalogValue;
    use crate::catalog::Catalogs;
    use crate::features::Contraction;
    use crate::features::Number;
    use crate::features::Person;
    use crate::features::VerbSlot;
    use crate::grammar::GeneratedActivation;
    use crate::grammar::Nonterminal;
    use crate::syntax::NumberLiteral;
    use crate::word::Auxiliary;
    use crate::word::AuxiliaryInflection;
    use crate::word::PredicateFrame;
    use crate::word::Verb as LexicalVerb;
    use crate::word::VerbInstance;
    use crate::word::Vocab;

    #[derive(Default)]
    struct PredicateRecorder {
        construction: Option<&'static str>,
        source: Option<String>,
    }

    impl deckmaste_construction_compiler::runtime::LinearizationVisitor for PredicateRecorder {
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

        fn subtree<T: Any>(
            &mut self,
            category: &'static str,
            value: &T,
        ) -> Result<(), Self::Error> {
            if category == "Verb" {
                let head = (value as &dyn Any)
                    .downcast_ref::<VerbAnalysis>()
                    .expect("the Verb subtree retains its typed lexical analysis");
                self.source = Vocabulary::new().render_verb_instance(head.instance());
            }
            Ok(())
        }

        fn scalar<T: Any>(&mut self, _codec: &'static str, _value: &T) -> Result<(), Self::Error> {
            Ok(())
        }

        fn identity<T: Any>(
            &mut self,
            _provider: &'static str,
            _value_type: &'static str,
            _value: &T,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    fn selected_verb_phrase_construction(value: &VerbPhrase) -> &'static str {
        let mut recorder = PredicateRecorder::default();
        linearize_predicate_verb_phrase_with(value, &mut recorder)
            .expect("the verb phrase has exactly one declaration-selected inverse");
        recorder
            .construction
            .expect("the selected generated verb-phrase form begins before its fields")
    }

    fn attack_head() -> VerbAnalysis {
        VerbAnalysis::new(
            VerbInstance {
                verb: LexicalVerb::Word(Vocab::Attack),
                slot: VerbSlot::Imperative,
            },
            PredicateFrame::OPEN,
        )
    }

    fn lexical_head(vocab: Vocab, slot: VerbSlot, frame: usize) -> VerbAnalysis {
        VerbAnalysis::new(
            VerbInstance {
                verb: LexicalVerb::Word(vocab),
                slot,
            },
            vocab.predicate_frames()[frame],
        )
    }

    fn this_card() -> NounPhrase {
        NounPhrase::ThisCard(crate::syntax::ThisCardForm::AbbreviatedName)
    }

    fn play_base() -> VerbPhrase {
        build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Play, VerbSlot::Imperative, 0)).unwrap(),
        )
        .unwrap()
    }

    fn full_auxiliary(auxiliary: Auxiliary, inflection: AuxiliaryInflection) -> AuxiliaryInstance {
        AuxiliaryInstance {
            auxiliary,
            inflection,
            contracted_negation: Contraction::Full,
        }
    }

    fn all_groups_with_predicate() -> &'static [&'static GroupData] {
        crate::constructions::GROUPS
    }

    fn parsed_preposition(source: &str) -> PrepositionalPhrase {
        crate::grammar::parse_nonterminal_with_activation(
            source,
            &crate::grammar::fixture_catalogs(),
            Nonterminal::PrepositionalPhrase,
            GeneratedActivation::Production,
        )
        .unwrap_or_else(|error| panic!("failed to parse PP {source:?}: {error:?}"))
        .prepositional_phrase()
        .expect("the requested root is a prepositional phrase")
        .clone()
    }

    #[test]
    fn lexical_verb_builder_parts_retain_analysis_frame_and_slot() {
        let expected_frame = Vocab::Ask.predicate_frames()[1];
        let expected_instance = VerbInstance {
            verb: LexicalVerb::Word(Vocab::Ask),
            slot: VerbSlot::PastParticiple,
        };
        let analysis = VerbAnalysis::new(expected_instance.clone(), expected_frame);
        let verb = build_verb(analysis.clone()).expect("the exact lexical analysis builds");
        assert_eq!(verb.instance().slot, VerbSlot::PastParticiple);
        let recovered = parts_verb(&verb);
        assert_eq!(
            recovered,
            VerbAnalysis::new(expected_instance, expected_frame),
            "the exact lexical instance, slot, and selected frame survive parts"
        );
        assert_eq!(recovered, analysis);
        assert_eq!(
            build_verb(recovered).expect("the recovered lexical analysis rebuilds"),
            verb
        );
    }

    #[test]
    fn verb_phrase_base_builds_destructures_linearizes_and_reparses() {
        // Mutations caught: bypass lexical-form validation in the checked
        // builder, bypass the generated whole-value inverse check so a
        // non-base predicate linearizes, accept a feature argument whose
        // category is not `Features::Verb`, or break the production generated
        // lowering path.
        assert!(
            crate::constructions::GROUPS
                .iter()
                .any(|group| group.name == "predicate"),
            "the predicate declaration is active in production"
        );
        let head = attack_head();
        let predicate = build_verb_phrase_base(head.clone()).expect("a lexical head builds");
        assert_eq!(parts_verb_phrase_base(&predicate), head);

        let wrong_form = VerbAnalysis::new(
            VerbInstance {
                verb: LexicalVerb::Word(Vocab::Card),
                slot: VerbSlot::Imperative,
            },
            PredicateFrame::OPEN,
        );
        assert!(
            Vocabulary::new()
                .render_verb_instance(wrong_form.instance())
                .is_none(),
            "the negative fixture must lack the declared surface form"
        );
        assert_eq!(
            build_verb_phrase_base(wrong_form),
            Err(DeclarationViolation {
                construction: "verb_phrase_base",
                requirement: "the lexical head has a renderable verb form",
            })
        );

        let verb_features = Features::Verb {
            slot: VerbSlot::Imperative,
            frame: PredicateFrame::OPEN,
            head_is_copular: false,
            object_gap_requires_rules_object: false,
        };
        let base_index = PREDICATE_DECLARATION
            .constructions
            .iter()
            .position(|construction| construction.id == "verb_phrase_base")
            .expect("the census includes the base declaration");
        assert_eq!(
            reduce_predicate_features(base_index, &[Some(&verb_features)]),
            Some(Features::VerbPhrase {
                form: crate::grammar::PredicateForm::Imperative,
                passive: false,
                dependent_count: 0,
                object: crate::grammar::PredicateObjectState::None,
                indirect_object: false,
                selected_preposition: false,
                phase: crate::grammar::PredicateAttachmentPhase::Object,
                frame: PredicateFrame::OPEN,
                bare: true,
                head_is_copular: false,
                object_gap_requires_rules_object: false,
                subjunctive: false,
            })
        );
        assert!(reduce_predicate_argument_complete(base_index, &[Some(&verb_features)]).is_some());
        assert!(reduce_predicate_features(base_index, &[Some(&Features::None)]).is_none());

        let mut recorder = PredicateRecorder::default();
        linearize_predicate_verb_phrase_with(&predicate, &mut recorder)
            .expect("the named VerbPhrase inverse selects the base declaration");
        let source = recorder
            .source
            .expect("the inverse visits the lexical head");
        assert_eq!(source, "attack");

        let mut wrong_shape = predicate.clone();
        wrong_shape.test_add_preverb_modifier(crate::syntax::PreverbModifier::Not);
        let mut rejected = PredicateRecorder::default();
        assert!(matches!(
            linearize_predicate_verb_phrase_with(&wrong_shape, &mut rejected),
            Err(
                deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                    group: "predicate"
                }
            )
        ));
        assert!(rejected.source.is_none());

        let parsed = crate::grammar::parse_nonterminal_with_activation(
            &source,
            &Catalogs::default(),
            Nonterminal::VerbPhrase,
            GeneratedActivation::Groups(GROUPS),
        )
        .expect("the generated base declaration reparses its own inverse");
        assert!(parsed.construction_decisions().iter().any(|decision| {
            decision.selected().as_str() == "verb_phrase_base"
                && decision.owner() == crate::construction::ConstructionOwner::Generated
        }));
    }

    #[test]
    fn core_predicate_slice_has_the_independently_authored_nine_id_census() {
        // Mutations caught by the completed Task 2 matrix: accept a missing or
        // surplus direct/indirect object, accept the wrong lexical frame,
        // form, voice, or agreement, admit an unselected PP, or retain an
        // illegal recipient-passive theme. The first RED deliberately pins
        // the missing generated owners before their builders exist.
        assert_eq!(
            PREDICATE_DECLARATION
                .constructions
                .iter()
                .map(|construction| construction.id)
                .filter(|id| {
                    matches!(
                        *id,
                        "verb"
                            | "verb_phrase_base"
                            | "verb_phrase_auxiliary"
                            | "verb_phrase_auxiliary_proform"
                            | "verb_phrase_direct_object"
                            | "verb_phrase_indirect_object"
                            | "verb_phrase_adjective"
                            | "verb_phrase_prepositional"
                            | "verb_phrase_infinitive"
                    )
                })
                .collect::<Vec<_>>(),
            [
                "verb",
                "verb_phrase_base",
                "verb_phrase_auxiliary",
                "verb_phrase_auxiliary_proform",
                "verb_phrase_direct_object",
                "verb_phrase_indirect_object",
                "verb_phrase_adjective",
                "verb_phrase_prepositional",
                "verb_phrase_infinitive",
            ]
        );
    }

    #[test]
    fn attachment_slice_has_the_independently_authored_nine_id_census() {
        // Mutations caught by the Task 3 matrix: omit or rename any declared
        // attachment owner after production activation.
        let attachment_ids = PREDICATE_DECLARATION
            .constructions
            .iter()
            .map(|construction| construction.id)
            .filter(|id| {
                matches!(
                    *id,
                    "verb_phrase_adverb"
                        | "verb_phrase_preverb_adverb"
                        | "verb_phrase_particle"
                        | "verb_phrase_coin_result"
                        | "verb_phrase_frequency"
                        | "frequency_phrase_adverb"
                        | "frequency_phrase"
                        | "verb_phrase_passive_shared_determiner_prepositional"
                        | "verb_phrase_except_by"
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            attachment_ids,
            [
                "verb_phrase_passive_shared_determiner_prepositional",
                "verb_phrase_except_by",
                "verb_phrase_adverb",
                "verb_phrase_preverb_adverb",
                "verb_phrase_particle",
                "verb_phrase_coin_result",
                "verb_phrase_frequency",
                "frequency_phrase_adverb",
                "frequency_phrase",
            ]
        );
    }

    #[test]
    fn predicate_value_slice_has_the_independently_authored_fifteen_id_census() {
        // Mutations caught by the Task 4 matrix: omit or rename any typed
        // object, symbol, mana-list, power/toughness, or quantity owner after
        // production activation.
        let value_ids = PREDICATE_DECLARATION
            .constructions
            .iter()
            .map(|construction| construction.id)
            .filter(|id| {
                matches!(
                    *id,
                    "verb_phrase_ability"
                        | "verb_phrase_quoted_ability"
                        | "verb_phrase_quoted_ability_coordination"
                        | "verb_phrase_ability_quoted_coordination"
                        | "verb_phrase_oracle_symbol"
                        | "verb_phrase_symbol_sequence"
                        | "mana_amount_symbol"
                        | "mana_amount_sequence"
                        | "mana_amount_list_single"
                        | "mana_amount_list_comma"
                        | "mana_amount_coordination"
                        | "mana_amount_coordination_oxford"
                        | "verb_phrase_mana_amount_coordination"
                        | "verb_phrase_power_toughness"
                        | "verb_phrase_quantity"
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            value_ids,
            [
                "verb_phrase_ability",
                "verb_phrase_quoted_ability",
                "verb_phrase_quoted_ability_coordination",
                "verb_phrase_ability_quoted_coordination",
                "verb_phrase_oracle_symbol",
                "verb_phrase_symbol_sequence",
                "mana_amount_symbol",
                "mana_amount_sequence",
                "mana_amount_list_single",
                "mana_amount_list_comma",
                "mana_amount_coordination",
                "mana_amount_coordination_oxford",
                "verb_phrase_mana_amount_coordination",
                "verb_phrase_power_toughness",
                "verb_phrase_quantity",
            ]
        );
    }

    #[test]
    fn generated_mana_symbol_lowers_through_the_declared_category() {
        // Mutation caught: register the typed field/category in the chart but
        // omit the corresponding generic lowering projection.
        let expected = build_mana_amount_symbol(
            OracleSymbol::new("{C}").expect("a colorless mana symbol is valid"),
        )
        .expect("one symbol is a valid mana amount");
        let parsed = crate::grammar::parse_nonterminal_with_activation(
            "{C}",
            &Catalogs::default(),
            Nonterminal::ManaAmount,
            GeneratedActivation::Groups(all_groups_with_predicate()),
        )
        .expect("the generated mana amount root lowers");
        assert_eq!(
            parsed
                .construction_decisions()
                .last()
                .expect("the root has a construction decision")
                .selected()
                .as_str(),
            "mana_amount_symbol",
        );
        let orders = crate::grammar::exact::parse_groups_as_declared_category_in_both_orders(
            "{C}",
            &Catalogs::default(),
            "ManaAmount",
            &expected,
            10_000,
            all_groups_with_predicate(),
        )
        .expect("the exact generated parse stays within its enumeration budget");
        for parses in orders {
            assert_eq!(
                parses
                    .iter()
                    .map(|parse| parse.ast().construction)
                    .collect::<Vec<_>>(),
                ["mana_amount_symbol"],
            );
        }
    }

    fn test_ability_atom(catalogs: &Catalogs) -> CatalogAtom {
        let CatalogValue::Atom(atom) = catalogs
            .matches("flying", crate::catalog::CatalogSlot::AbilityItem)[0]
            .value
            .clone()
        else {
            panic!("the ability-item match carries a catalog atom")
        };
        atom
    }

    fn test_non_keyword_ability_atoms() -> [CatalogAtom; 2] {
        let catalogs = Catalogs::default()
            .with_catalog(CatalogKind::CardType, ["Creature"])
            .with_catalog(CatalogKind::AbilityWord, ["Landfall"]);
        let CatalogValue::Word(crate::word::WordMatch::Noun(card_type)) = catalogs.matches(
            "creature",
            crate::catalog::CatalogSlot::Noun(crate::word::NounUsage::Count),
        )[0]
        .value
        .clone() else {
            panic!("the card-type fixture carries a noun")
        };
        let crate::word::NounInstanceKind::Singular(crate::word::Noun::Catalog(card_type)) =
            card_type.kind()
        else {
            panic!("the card-type noun carries a catalog atom")
        };
        let CatalogValue::Atom(ability_word) = catalogs
            .matches("Landfall", crate::catalog::CatalogSlot::AbilityWord)[0]
            .value
            .clone()
        else {
            panic!("the ability-word fixture carries a catalog atom")
        };
        [card_type.clone(), ability_word]
    }

    fn test_quoted_ability(source: &str, catalogs: &Catalogs) -> QuotedAbility {
        crate::grammar::ability::parse_quoted_ability_fragment(
            source,
            catalogs,
            &crate::identity::SelfReference::default(),
        )
    }

    fn open_predicate(vocab: Vocab) -> VerbPhrase {
        build_verb_phrase_base(
            build_verb(lexical_head(vocab, VerbSlot::Imperative, 0))
                .expect("the lexical form builds"),
        )
        .expect("the open predicate head builds")
    }

    fn test_symbol(source: &str) -> OracleSymbol {
        OracleSymbol::new(source).unwrap_or_else(|| panic!("invalid test symbol {source:?}"))
    }

    #[derive(Debug)]
    enum PredicateFamilyWitnessValue {
        Verb(VerbAnalysis),
        VerbPhrase(VerbPhrase),
        Frequency(FrequencyPhrase),
        ManaAmount(PredicateObject),
        ManaAmountList(PredicateObject),
        CoordinatedMana(CoordinatedPredicateObject),
    }

    impl PredicateFamilyWitnessValue {
        fn as_any(&self) -> &dyn Any {
            match self {
                Self::Verb(value) => value,
                Self::VerbPhrase(value) => value,
                Self::Frequency(value) => value,
                Self::ManaAmount(value) | Self::ManaAmountList(value) => value,
                Self::CoordinatedMana(value) => value,
            }
        }

        fn render_generated(
            &self,
        ) -> Result<crate::renderer::GeneratedPredicateRender, crate::renderer::RenderError>
        {
            match self {
                Self::Verb(value) => crate::renderer::render_generated_predicate_verb_law(value),
                Self::VerbPhrase(value) => {
                    crate::renderer::render_generated_predicate_verb_phrase_law(value)
                }
                Self::Frequency(value) => {
                    crate::renderer::render_generated_predicate_frequency_phrase_law(value)
                }
                Self::ManaAmount(value) => {
                    crate::renderer::render_generated_predicate_mana_amount_law(value)
                }
                Self::ManaAmountList(value) => {
                    crate::renderer::render_generated_predicate_mana_amount_list_law(value)
                }
                Self::CoordinatedMana(value) => {
                    crate::renderer::render_generated_predicate_coordinated_mana_amount_law(value)
                }
            }
        }
    }

    #[derive(Debug)]
    struct PredicateFamilyWitness {
        id: &'static str,
        category: &'static str,
        surface: &'static str,
        value: PredicateFamilyWitnessValue,
    }

    #[allow(
        clippy::too_many_lines,
        reason = "the explicit table keeps all 34 stable IDs and their typed witnesses reviewable in declaration order"
    )]
    fn all_34_predicate_family_witnesses(catalogs: &Catalogs) -> Vec<PredicateFamilyWitness> {
        let verb = |vocab, slot, frame| {
            build_verb(lexical_head(vocab, slot, frame)).expect("the witness verb builds")
        };
        let base = |vocab, slot, frame| {
            build_verb_phrase_base(verb(vocab, slot, frame)).expect("the witness base builds")
        };
        let object_it = || NounPhrase::Pronoun {
            pronoun: crate::word::Pronoun::It(crate::word::Gender::Neuter),
            case: crate::features::PronounCase::Object,
        };
        let adjective = AdjectivePhrase {
            degree: None,
            head: crate::word::Adjective::Color(crate::word::ColorWord::Red),
            complements: Vec::new(),
        };
        let progressive = build_verb_phrase_auxiliary(
            full_auxiliary(
                Auxiliary::Be,
                AuxiliaryInflection::Present {
                    person: Person::Third,
                    number: Number::Singular,
                },
            ),
            base(Vocab::Attack, VerbSlot::PresentParticiple, 0),
        )
        .unwrap();
        let passive = build_verb_phrase_auxiliary(
            full_auxiliary(
                Auxiliary::Be,
                AuxiliaryInflection::Present {
                    person: Person::Third,
                    number: Number::Singular,
                },
            ),
            base(Vocab::Block, VerbSlot::PastParticiple, 1),
        )
        .unwrap();
        let frequency_more_than_twice = FrequencyPhrase {
            bound: FrequencyBound::MoreThan,
            count: crate::syntax::FrequencyCount::Twice,
        };
        let frequency_no_more_than_once = FrequencyPhrase {
            bound: FrequencyBound::NoMoreThan,
            count: crate::syntax::FrequencyCount::Once,
        };
        let ability = test_ability_atom(catalogs);
        let quoted_tap = test_quoted_ability("{T}: Draw a card.", catalogs);
        let quoted_flying = test_quoted_ability("Flying", catalogs);
        let white = test_symbol("{W}");
        let blue = test_symbol("{U}");
        let black = test_symbol("{B}");
        let white_amount = build_mana_amount_symbol(white.clone()).unwrap();
        let blue_amount = build_mana_amount_symbol(blue.clone()).unwrap();
        let black_amount = build_mana_amount_symbol(black.clone()).unwrap();
        let blue_black = build_mana_amount_sequence(vec![blue.clone(), black.clone()]).unwrap();
        let comma_list = build_mana_amount_list_comma(
            white_amount.clone(),
            vec![PredicateObjectCoordination {
                conjunction: None,
                object: blue_amount.clone(),
            }],
        )
        .unwrap();
        let binary_mana = build_mana_amount_coordination(
            white_amount.clone(),
            Conjunction::Or,
            blue_amount.clone(),
        )
        .unwrap();
        let stats = PowerToughness {
            power: crate::syntax::SignedScalar {
                sign: crate::syntax::ScalarSign::Plus,
                value: crate::syntax::ScalarValue::Integer(1),
            },
            toughness: crate::syntax::SignedScalar {
                sign: crate::syntax::ScalarSign::Plus,
                value: crate::syntax::ScalarValue::Integer(2),
            },
        };
        let quantity = Quantity::try_exact(NumberLiteral {
            value: 2,
            numeral: crate::numeral::Numeral::Cardinal,
        })
        .unwrap();

        vec![
            PredicateFamilyWitness {
                id: "verb",
                category: "Verb",
                surface: "attack",
                value: PredicateFamilyWitnessValue::Verb(verb(
                    Vocab::Attack,
                    VerbSlot::Imperative,
                    0,
                )),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_base",
                category: "VerbPhrase",
                surface: "attack",
                value: PredicateFamilyWitnessValue::VerbPhrase(base(
                    Vocab::Attack,
                    VerbSlot::Imperative,
                    0,
                )),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_auxiliary",
                category: "VerbPhrase",
                surface: "is attacking",
                value: PredicateFamilyWitnessValue::VerbPhrase(progressive),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_auxiliary_proform",
                category: "VerbPhrase",
                surface: "can",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_auxiliary_proform(full_auxiliary(
                        Auxiliary::Can,
                        AuxiliaryInflection::Base,
                    ))
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_direct_object",
                category: "VerbPhrase",
                surface: "have it",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_direct_object(
                        base(Vocab::Have, VerbSlot::Imperative, 0),
                        object_it(),
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_indirect_object",
                category: "VerbPhrase",
                surface: "ask it",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_indirect_object(
                        base(Vocab::Ask, VerbSlot::Imperative, 1),
                        object_it(),
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_adjective",
                category: "VerbPhrase",
                surface: "be red",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_adjective(
                        base(Vocab::Be, VerbSlot::Infinitive, 0),
                        adjective,
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_prepositional",
                category: "VerbPhrase",
                surface: "look at it",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_prepositional(
                        base(Vocab::Look, VerbSlot::Imperative, 1),
                        PrepositionalPhrase::simple(
                            crate::syntax::Preposition::At,
                            Phrase::NounPhrase(Box::new(object_it())),
                        ),
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_passive_shared_determiner_prepositional",
                category: "VerbPhrase",
                surface: "is blocked to target player or planeswalker",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_passive_shared_determiner_prepositional(
                        passive.clone(),
                        parsed_preposition("to target player or planeswalker"),
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_except_by",
                category: "VerbPhrase",
                surface: "is blocked except by creatures",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_except_by(passive, parsed_preposition("by creatures"))
                        .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_infinitive",
                category: "VerbPhrase",
                surface: "begins to attack",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_infinitive(
                        base(
                            Vocab::Begin,
                            VerbSlot::Present {
                                person: Person::Third,
                                number: Number::Singular,
                            },
                            0,
                        ),
                        InfinitiveClause::declaration_to(base(
                            Vocab::Attack,
                            VerbSlot::Infinitive,
                            0,
                        )),
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_adverb",
                category: "VerbPhrase",
                surface: "attack again",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_adverb(
                        base(Vocab::Attack, VerbSlot::Imperative, 0),
                        Vocab::Again,
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_preverb_adverb",
                category: "VerbPhrase",
                surface: "next attack",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_preverb_adverb(
                        PreverbModifier::Next,
                        base(Vocab::Attack, VerbSlot::Imperative, 0),
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_particle",
                category: "VerbPhrase",
                surface: "phase out",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_particle(
                        base(Vocab::Phase, VerbSlot::Imperative, 0),
                        VerbParticle::Out,
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_coin_result",
                category: "VerbPhrase",
                surface: "come up heads",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_coin_result(
                        base(Vocab::Come, VerbSlot::Imperative, 0),
                        CoinSide::Heads,
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_frequency",
                category: "VerbPhrase",
                surface: "attack no more than once",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_frequency(
                        base(Vocab::Attack, VerbSlot::Imperative, 0),
                        frequency_no_more_than_once,
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "frequency_phrase_adverb",
                category: "FrequencyPhrase",
                surface: "no more than twice",
                value: PredicateFamilyWitnessValue::Frequency(
                    build_frequency_phrase_adverb(frequency_more_than_twice).unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "frequency_phrase",
                category: "FrequencyPhrase",
                surface: "more than twice",
                value: PredicateFamilyWitnessValue::Frequency(
                    build_frequency_phrase(frequency_more_than_twice).unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_ability",
                category: "VerbPhrase",
                surface: "gain flying",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_ability(open_predicate(Vocab::Gain), ability.clone())
                        .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_quoted_ability",
                category: "VerbPhrase",
                surface: "gain \"{T}: Draw a card.\"",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_quoted_ability(
                        open_predicate(Vocab::Gain),
                        quoted_tap.clone(),
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_quoted_ability_coordination",
                category: "VerbPhrase",
                surface: "have \"{T}: Draw a card.\" or \"Flying\"",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_quoted_ability_coordination(
                        open_predicate(Vocab::Have),
                        vec![QuotedAbilityPairMember {
                            first: quoted_tap.clone(),
                            conjunction: Conjunction::Or,
                            next: quoted_flying,
                        }],
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_ability_quoted_coordination",
                category: "VerbPhrase",
                surface: "have flying and \"{T}: Draw a card.\"",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_ability_quoted_coordination(
                        open_predicate(Vocab::Have),
                        vec![AbilityQuotedPairMember {
                            ability,
                            conjunction: Conjunction::And,
                            quoted: quoted_tap,
                        }],
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_oracle_symbol",
                category: "VerbPhrase",
                surface: "add {W}",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_oracle_symbol(open_predicate(Vocab::Add), white.clone())
                        .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_symbol_sequence",
                category: "VerbPhrase",
                surface: "add {U}{B}",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_symbol_sequence(
                        open_predicate(Vocab::Add),
                        vec![blue.clone(), black.clone()],
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "mana_amount_symbol",
                category: "ManaAmount",
                surface: "{W}",
                value: PredicateFamilyWitnessValue::ManaAmount(white_amount.clone()),
            },
            PredicateFamilyWitness {
                id: "mana_amount_sequence",
                category: "ManaAmount",
                surface: "{U}{B}",
                value: PredicateFamilyWitnessValue::ManaAmount(blue_black),
            },
            PredicateFamilyWitness {
                id: "mana_amount_list_single",
                category: "ManaAmountList",
                surface: "{W}",
                value: PredicateFamilyWitnessValue::ManaAmountList(
                    build_mana_amount_list_single(white_amount.clone()).unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "mana_amount_list_comma",
                category: "ManaAmountList",
                surface: "{W}, {U}",
                value: PredicateFamilyWitnessValue::ManaAmountList(comma_list.clone()),
            },
            PredicateFamilyWitness {
                id: "mana_amount_coordination",
                category: "CoordinatedManaAmount",
                surface: "{W} or {U}",
                value: PredicateFamilyWitnessValue::CoordinatedMana(binary_mana.clone()),
            },
            PredicateFamilyWitness {
                id: "mana_amount_coordination_oxford",
                category: "CoordinatedManaAmount",
                surface: "{W}, {U}, and {B}",
                value: PredicateFamilyWitnessValue::CoordinatedMana(
                    build_mana_amount_coordination_oxford(
                        comma_list,
                        Conjunction::And,
                        black_amount,
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_mana_amount_coordination",
                category: "VerbPhrase",
                surface: "add {W} or {U}",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_mana_amount_coordination(
                        open_predicate(Vocab::Add),
                        binary_mana,
                    )
                    .unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_power_toughness",
                category: "VerbPhrase",
                surface: "get +1/+2",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_power_toughness(open_predicate(Vocab::Get), stats).unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_quantity",
                category: "VerbPhrase",
                surface: "scry two",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_quantity(open_predicate(Vocab::Scry), quantity).unwrap(),
                ),
            },
            PredicateFamilyWitness {
                id: "verb_phrase_causative",
                category: "VerbPhrase",
                surface: "have it enter",
                value: PredicateFamilyWitnessValue::VerbPhrase(
                    build_verb_phrase_causative(
                        build_verb_phrase_direct_object(
                            base(Vocab::Have, VerbSlot::Infinitive, 1),
                            object_it(),
                        )
                        .unwrap(),
                        base(Vocab::Enter, VerbSlot::Infinitive, 0),
                    )
                    .unwrap(),
                ),
            },
        ]
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "one explicit behavioral law checks all 34 stable declaration rows"
    )]
    fn every_predicate_row_has_a_typed_generated_inverse_and_exact_reparse() {
        // Mutations caught: omit a row from the category visitor, route any
        // identity through a parallel renderer, change exact bytes or form
        // ordinal, lower a row to a sibling typed value, or make registration
        // order choose a different generated construction.
        let catalogs = crate::grammar::fixture_catalogs();
        let witnesses = all_34_predicate_family_witnesses(&catalogs);
        assert_eq!(witnesses.len(), 34);
        assert_eq!(
            witnesses
                .iter()
                .map(|witness| witness.id)
                .collect::<Vec<_>>(),
            PREDICATE_DECLARATION
                .constructions
                .iter()
                .map(|construction| construction.id)
                .collect::<Vec<_>>()
        );

        for witness in witnesses {
            let rendered = witness.value.render_generated().unwrap_or_else(|error| {
                panic!("{} generated inverse failed: {error:?}", witness.id)
            });
            assert_eq!(
                rendered.text, witness.surface,
                "{} inverse bytes",
                witness.id
            );
            assert_eq!(
                (rendered.construction, rendered.form_ordinal),
                (witness.id, 0),
                "{} emitted category dispatcher selection",
                witness.id
            );
            let orders = crate::grammar::exact::parse_groups_as_declared_category_in_both_orders(
                witness.surface,
                &catalogs,
                witness.category,
                witness.value.as_any(),
                100_000,
                all_groups_with_predicate(),
            )
            .unwrap_or_else(|error| panic!("{} exact reparse failed: {error:?}", witness.id));
            for parses in orders {
                assert_eq!(
                    parses
                        .iter()
                        .filter(|parse| parse.ast().construction == witness.id)
                        .map(|parse| (parse.ast().construction, parse.ast().form_ordinal))
                        .collect::<Vec<_>>(),
                    [(witness.id, 0)],
                    "{} exact generated root/form must survive alongside genuine alternatives: {parses:#?}",
                    witness.id,
                );
            }
        }
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "the one ordered matrix audits every Task 4 declaration row"
    )]
    fn predicate_value_builders_and_parts_round_trip_every_declared_row() {
        // Mutations caught: erase a typed identity, flatten a symbol sequence,
        // reorder the mixed pair, discard a mana member/connective, collapse
        // list cardinality, or coerce P/T and quantity into an untyped scalar.
        let catalogs = Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Flying"]);
        let ability = test_ability_atom(&catalogs);
        let quoted_tap = test_quoted_ability("{T}: Draw a card.", &catalogs);
        let quoted_fly = test_quoted_ability("Flying", &catalogs);

        let ability_value = build_verb_phrase_ability(open_predicate(Vocab::Gain), ability.clone())
            .expect("an open frame admits the keyword-ability complement");
        let (predicate, recovered) =
            parts_verb_phrase_ability(&ability_value).expect("ability parts exist");
        assert_eq!(
            build_verb_phrase_ability(predicate, recovered).unwrap(),
            ability_value,
            "verb_phrase_ability"
        );

        let quoted_value =
            build_verb_phrase_quoted_ability(open_predicate(Vocab::Gain), quoted_tap.clone())
                .expect("an open frame admits the quoted-ability complement");
        let (predicate, recovered) =
            parts_verb_phrase_quoted_ability(&quoted_value).expect("quoted ability parts exist");
        assert_eq!(
            build_verb_phrase_quoted_ability(predicate, recovered).unwrap(),
            quoted_value,
            "verb_phrase_quoted_ability"
        );

        let quoted_pair_value = build_verb_phrase_quoted_ability_coordination(
            open_predicate(Vocab::Have),
            vec![QuotedAbilityPairMember {
                first: quoted_tap.clone(),
                conjunction: Conjunction::Or,
                next: quoted_fly.clone(),
            }],
        )
        .expect("two quotes joined by or are admitted");
        let (predicate, recovered) =
            parts_verb_phrase_quoted_ability_coordination(&quoted_pair_value)
                .expect("quoted-pair parts exist");
        assert_eq!(
            build_verb_phrase_quoted_ability_coordination(predicate, recovered).unwrap(),
            quoted_pair_value,
            "verb_phrase_quoted_ability_coordination"
        );

        let mixed_value = build_verb_phrase_ability_quoted_coordination(
            open_predicate(Vocab::Have),
            vec![AbilityQuotedPairMember {
                ability: ability.clone(),
                conjunction: Conjunction::And,
                quoted: quoted_tap.clone(),
            }],
        )
        .expect("ability first and quote second are admitted");
        let (predicate, recovered) = parts_verb_phrase_ability_quoted_coordination(&mixed_value)
            .expect("mixed-pair parts exist");
        assert_eq!(
            build_verb_phrase_ability_quoted_coordination(predicate, recovered).unwrap(),
            mixed_value,
            "verb_phrase_ability_quoted_coordination"
        );

        let white = test_symbol("{W}");
        let blue = test_symbol("{U}");
        let black = test_symbol("{B}");
        let symbol_value =
            build_verb_phrase_oracle_symbol(open_predicate(Vocab::Add), white.clone())
                .expect("one symbol is a scalar complement");
        let (predicate, recovered) =
            parts_verb_phrase_oracle_symbol(&symbol_value).expect("symbol parts exist");
        assert_eq!(
            build_verb_phrase_oracle_symbol(predicate, recovered).unwrap(),
            symbol_value,
            "verb_phrase_oracle_symbol"
        );

        let symbols = vec![blue.clone(), black.clone()];
        let sequence_value = build_verb_phrase_symbol_sequence(open_predicate(Vocab::Add), symbols)
            .expect("a nonempty symbol sequence is a scalar complement");
        let (predicate, recovered) = parts_verb_phrase_symbol_sequence(&sequence_value)
            .expect("symbol-sequence parts exist");
        assert_eq!(
            build_verb_phrase_symbol_sequence(predicate, recovered).unwrap(),
            sequence_value,
            "verb_phrase_symbol_sequence"
        );

        let mana_symbol = build_mana_amount_symbol(white.clone()).unwrap();
        assert_eq!(
            build_mana_amount_symbol(parts_mana_amount_symbol(&mana_symbol)).unwrap(),
            mana_symbol,
            "mana_amount_symbol"
        );
        let mana_sequence = build_mana_amount_sequence(vec![blue.clone(), black.clone()]).unwrap();
        assert_eq!(
            build_mana_amount_sequence(parts_mana_amount_sequence(&mana_sequence)).unwrap(),
            mana_sequence,
            "mana_amount_sequence"
        );
        let list_single = build_mana_amount_list_single(mana_symbol.clone()).unwrap();
        assert_eq!(
            build_mana_amount_list_single(parts_mana_amount_list_single(&list_single)).unwrap(),
            list_single,
            "mana_amount_list_single"
        );
        let list_comma = build_mana_amount_list_comma(
            mana_symbol.clone(),
            vec![PredicateObjectCoordination {
                conjunction: None,
                object: mana_sequence.clone(),
            }],
        )
        .unwrap();
        let (first, rest) = parts_mana_amount_list_comma(&list_comma);
        assert_eq!(
            build_mana_amount_list_comma(first, rest).unwrap(),
            list_comma,
            "mana_amount_list_comma"
        );
        let binary = build_mana_amount_coordination(
            mana_symbol.clone(),
            Conjunction::Or,
            mana_sequence.clone(),
        )
        .unwrap();
        let (first, conjunction, next) = parts_mana_amount_coordination(&binary);
        assert_eq!(
            build_mana_amount_coordination(first, conjunction, next).unwrap(),
            binary,
            "mana_amount_coordination"
        );
        let oxford = build_mana_amount_coordination_oxford(
            list_comma,
            Conjunction::And,
            build_mana_amount_symbol(black.clone()).unwrap(),
        )
        .unwrap();
        let (list, conjunction, next) = parts_mana_amount_coordination_oxford(&oxford);
        assert_eq!(
            build_mana_amount_coordination_oxford(list, conjunction, next).unwrap(),
            oxford,
            "mana_amount_coordination_oxford"
        );
        let coordinated_value =
            build_verb_phrase_mana_amount_coordination(open_predicate(Vocab::Add), binary).unwrap();
        let (predicate, recovered) = parts_verb_phrase_mana_amount_coordination(&coordinated_value)
            .expect("coordinated mana parts exist");
        assert_eq!(
            build_verb_phrase_mana_amount_coordination(predicate, recovered).unwrap(),
            coordinated_value,
            "verb_phrase_mana_amount_coordination"
        );

        let stats = PowerToughness {
            power: crate::syntax::SignedScalar {
                sign: crate::syntax::ScalarSign::Plus,
                value: crate::syntax::ScalarValue::Integer(1),
            },
            toughness: crate::syntax::SignedScalar {
                sign: crate::syntax::ScalarSign::Plus,
                value: crate::syntax::ScalarValue::Integer(2),
            },
        };
        let stats_value =
            build_verb_phrase_power_toughness(open_predicate(Vocab::Get), stats).unwrap();
        let (predicate, recovered) =
            parts_verb_phrase_power_toughness(&stats_value).expect("power/toughness parts exist");
        assert_eq!(
            build_verb_phrase_power_toughness(predicate, recovered).unwrap(),
            stats_value,
            "verb_phrase_power_toughness"
        );

        let quantity = Quantity::try_exact(NumberLiteral {
            value: 2,
            numeral: crate::numeral::Numeral::Cardinal,
        })
        .unwrap();
        let quantity_value =
            build_verb_phrase_quantity(open_predicate(Vocab::Scry), quantity).unwrap();
        let (predicate, recovered) =
            parts_verb_phrase_quantity(&quantity_value).expect("quantity parts exist");
        assert_eq!(
            build_verb_phrase_quantity(predicate, recovered).unwrap(),
            quantity_value,
            "verb_phrase_quantity"
        );
    }

    #[test]
    fn predicate_value_builders_reject_named_structural_mutations() {
        // Named mutations: EMPTY_SEQUENCE, MISSING_PAIR, SURPLUS_PAIR,
        // SWAP_MIXED_ORDER, ONE_MEMBER_COORDINATION, ILLEGAL_CONJUNCTION,
        // MALFORMED_COMMA_PREFIX, and SCALAR_CATEGORY_MISMATCH.
        let catalogs = Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Flying"]);
        let ability = test_ability_atom(&catalogs);
        let quoted = test_quoted_ability("{T}: Draw a card.", &catalogs);
        assert!(
            build_verb_phrase_symbol_sequence(open_predicate(Vocab::Add), Vec::new()).is_err(),
            "EMPTY_SEQUENCE: a verb phrase cannot attach an empty symbol group"
        );
        assert!(
            build_mana_amount_sequence(Vec::new()).is_err(),
            "EMPTY_SEQUENCE: a mana amount cannot erase symbol identity"
        );
        assert!(
            build_verb_phrase_quoted_ability_coordination(open_predicate(Vocab::Have), Vec::new(),)
                .is_err(),
            "MISSING_PAIR"
        );
        assert!(
            build_verb_phrase_ability_quoted_coordination(
                open_predicate(Vocab::Have),
                vec![
                    AbilityQuotedPairMember {
                        ability: ability.clone(),
                        conjunction: Conjunction::And,
                        quoted: quoted.clone(),
                    },
                    AbilityQuotedPairMember {
                        ability: ability.clone(),
                        conjunction: Conjunction::Or,
                        quoted: quoted.clone(),
                    },
                ],
            )
            .is_err(),
            "SURPLUS_PAIR"
        );

        let base = open_predicate(Vocab::Have);
        let (_, shell) = base.clone().declaration_into_dependent_projection();
        let swapped = VerbPhrase::declaration_from_dependent_projection(
            shell,
            vec![VerbDependent::CoordinatedObject(
                CoordinatedPredicateObject {
                    first: Box::new(PredicateObject::QuotedAbility(Box::new(quoted.clone()))),
                    rest: vec![PredicateObjectCoordination {
                        conjunction: Some(Conjunction::And),
                        object: PredicateObject::Ability(AbilityObject {
                            ability: ability.clone(),
                            argument: None,
                        }),
                    }],
                },
            )],
        );
        assert!(
            parts_verb_phrase_ability_quoted_coordination(&swapped).is_none(),
            "SWAP_MIXED_ORDER: the verified surface is ability first, quote second"
        );

        let white = build_mana_amount_symbol(test_symbol("{W}")).unwrap();
        let blue = build_mana_amount_symbol(test_symbol("{U}")).unwrap();
        assert!(
            build_mana_amount_list_comma(white.clone(), Vec::new()).is_err(),
            "ONE_MEMBER_COORDINATION: a comma list needs a second member"
        );
        assert!(
            build_mana_amount_coordination(white.clone(), Conjunction::Then, blue.clone(),)
                .is_err(),
            "ILLEGAL_CONJUNCTION: `then` is not a predicate-object connective"
        );
        assert!(
            build_mana_amount_coordination(white.clone(), Conjunction::AndOr, blue.clone(),)
                .is_err(),
            "ILLEGAL_CONJUNCTION: `and/or` is not admitted by the predicate family"
        );
        assert!(
            build_mana_amount_list_comma(
                white.clone(),
                vec![PredicateObjectCoordination {
                    conjunction: Some(Conjunction::Or),
                    object: blue.clone(),
                }],
            )
            .is_err(),
            "MALFORMED_COMMA_PREFIX: an open prefix cannot contain its final connective"
        );
        assert!(
            build_mana_amount_coordination_oxford(
                build_mana_amount_list_single(white.clone()).unwrap(),
                Conjunction::Or,
                blue.clone(),
            )
            .is_err(),
            "ONE_MEMBER_COORDINATION: an Oxford prefix must already contain two members"
        );
        let quantity = Quantity::try_exact(NumberLiteral {
            value: 2,
            numeral: crate::numeral::Numeral::Cardinal,
        })
        .unwrap();
        assert!(
            build_mana_amount_list_single(PredicateObject::Quantity(quantity)).is_err(),
            "SCALAR_CATEGORY_MISMATCH: quantity is not a mana atom"
        );
        let one_member = CoordinatedPredicateObject {
            first: Box::new(white),
            rest: Vec::new(),
        };
        assert!(
            build_verb_phrase_mana_amount_coordination(open_predicate(Vocab::Add), one_member,)
                .is_err(),
            "ONE_MEMBER_COORDINATION: the verb attachment refuses an empty rest"
        );
    }

    #[test]
    fn predicate_value_ability_builders_reject_non_keyword_atoms() {
        // Mutation caught: forge the broad CatalogAtom identity with a card
        // type or ability word instead of a keyword ability.
        let catalogs = Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Flying"]);
        let quoted = test_quoted_ability("{T}: Draw a card.", &catalogs);
        for wrong in test_non_keyword_ability_atoms() {
            assert!(
                build_verb_phrase_ability(open_predicate(Vocab::Gain), wrong.clone()).is_err(),
                "single-ability ingress admitted {:?}",
                wrong.kind,
            );
            assert!(
                build_verb_phrase_ability_quoted_coordination(
                    open_predicate(Vocab::Have),
                    vec![AbilityQuotedPairMember {
                        ability: wrong.clone(),
                        conjunction: Conjunction::And,
                        quoted: quoted.clone(),
                    }],
                )
                .is_err(),
                "mixed ability+quote ingress admitted {:?}",
                wrong.kind,
            );
        }
    }

    #[test]
    fn predicate_value_ability_parts_reject_non_keyword_atoms() {
        // Mutation caught: let an invalid direct AST bypass the checked
        // builder through a permissive detach/inverse path.
        let catalogs = Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Flying"]);
        let quoted = test_quoted_ability("{T}: Draw a card.", &catalogs);
        for wrong in test_non_keyword_ability_atoms() {
            let base = open_predicate(Vocab::Gain);
            let (_, shell) = base.declaration_into_dependent_projection();
            let invalid_single = VerbPhrase::declaration_from_dependent_projection(
                shell,
                vec![VerbDependent::PredicateComplement(Phrase::CatalogAtom(
                    wrong.clone(),
                ))],
            );
            assert!(
                parts_verb_phrase_ability(&invalid_single).is_none(),
                "single-ability inverse admitted {:?}",
                wrong.kind,
            );

            let base = open_predicate(Vocab::Have);
            let (_, shell) = base.declaration_into_dependent_projection();
            let invalid_mixed = VerbPhrase::declaration_from_dependent_projection(
                shell,
                vec![VerbDependent::CoordinatedObject(
                    CoordinatedPredicateObject {
                        first: Box::new(PredicateObject::Ability(AbilityObject {
                            ability: wrong.clone(),
                            argument: None,
                        })),
                        rest: vec![PredicateObjectCoordination {
                            conjunction: Some(Conjunction::And),
                            object: PredicateObject::QuotedAbility(Box::new(quoted.clone())),
                        }],
                    },
                )],
            );
            assert!(
                parts_verb_phrase_ability_quoted_coordination(&invalid_mixed).is_none(),
                "mixed ability+quote inverse admitted {:?}",
                wrong.kind,
            );
        }
    }

    #[test]
    fn predicate_value_feature_replay_rejects_non_keyword_catalog_atoms() {
        // Mutation caught: classify every Phrase::CatalogAtom as an ability
        // complement while replaying a direct AST's predicate features.
        for wrong in test_non_keyword_ability_atoms() {
            let base = open_predicate(Vocab::Gain);
            let (_, shell) = base.declaration_into_dependent_projection();
            let invalid = VerbPhrase::declaration_from_dependent_projection(
                shell,
                vec![VerbDependent::PredicateComplement(Phrase::CatalogAtom(
                    wrong.clone(),
                ))],
            );
            assert!(
                invalid.declaration_core_features().is_none(),
                "feature replay admitted {:?}",
                wrong.kind,
            );
        }
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "the exact registration matrix deliberately names every Task 4 row"
    )]
    fn predicate_value_rows_lower_exactly_in_normal_and_reversed_registration() {
        // Mutations caught: lose a typed lowering adapter, attribute a surface
        // to the wrong row, flatten a sequence, or let registration position
        // choose between the atomic/list/Oxford shapes.
        let catalogs = Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Flying"]);
        macro_rules! exact_row_impl {
            ($id:literal, $source:literal, $category:literal, $value:expr) => {{
                let orders =
                    crate::grammar::exact::parse_groups_as_declared_category_in_both_orders(
                        $source,
                        &catalogs,
                        $category,
                        $value,
                        100_000,
                        all_groups_with_predicate(),
                    )
                    .unwrap_or_else(|error| {
                        panic!("{} exact parse failed for {:?}: {error:?}", $id, $source)
                    });
                for parses in orders {
                    let actual = parses
                        .iter()
                        .map(|parse| (parse.ast().construction, parse.ast().form_ordinal))
                        .collect::<std::collections::BTreeSet<_>>();
                    assert_eq!(
                        actual,
                        std::collections::BTreeSet::from([($id, 0)]),
                        "{} exact attribution for {:?}: {parses:#?}",
                        $id,
                        $source,
                    );
                }
            }};
        }
        macro_rules! exact_row {
            ($id:literal, $source:literal, "VerbPhrase", $value:expr) => {{
                let value = $value;
                assert_eq!(
                    crate::renderer::render_generated_predicate_verb_phrase(&value).unwrap(),
                    $source,
                    "{} generated inverse",
                    $id,
                );
                exact_row_impl!($id, $source, "VerbPhrase", &value);
            }};
            ($id:literal, $source:literal, "ManaAmount", $value:expr) => {{
                let value = $value;
                assert_eq!(
                    crate::renderer::render_generated_predicate_mana_amount(&value).unwrap(),
                    $source,
                    "{} generated inverse",
                    $id,
                );
                exact_row_impl!($id, $source, "ManaAmount", &value);
            }};
            ($id:literal, $source:literal, "ManaAmountList", $value:expr) => {{
                let value = $value;
                assert_eq!(
                    crate::renderer::render_generated_predicate_mana_amount_list(&value).unwrap(),
                    $source,
                    "{} generated inverse",
                    $id,
                );
                exact_row_impl!($id, $source, "ManaAmountList", &value);
            }};
            ($id:literal, $source:literal, "CoordinatedManaAmount", $value:expr) => {{
                let value = $value;
                assert_eq!(
                    crate::renderer::render_generated_predicate_coordinated_mana_amount(&value)
                        .unwrap(),
                    $source,
                    "{} generated inverse",
                    $id,
                );
                exact_row_impl!($id, $source, "CoordinatedManaAmount", &value);
            }};
        }

        let ability = test_ability_atom(&catalogs);
        let quoted_tap = test_quoted_ability("{T}: Draw a card.", &catalogs);
        let quoted_flying = test_quoted_ability("Flying", &catalogs);
        exact_row!(
            "verb_phrase_ability",
            "gain flying",
            "VerbPhrase",
            build_verb_phrase_ability(open_predicate(Vocab::Gain), ability.clone()).unwrap()
        );
        exact_row!(
            "verb_phrase_quoted_ability",
            "gain \"{T}: Draw a card.\"",
            "VerbPhrase",
            build_verb_phrase_quoted_ability(open_predicate(Vocab::Gain), quoted_tap.clone(),)
                .unwrap()
        );
        exact_row!(
            "verb_phrase_quoted_ability_coordination",
            "have \"{T}: Draw a card.\" or \"Flying\"",
            "VerbPhrase",
            build_verb_phrase_quoted_ability_coordination(
                open_predicate(Vocab::Have),
                vec![QuotedAbilityPairMember {
                    first: quoted_tap.clone(),
                    conjunction: Conjunction::Or,
                    next: quoted_flying,
                }],
            )
            .unwrap()
        );
        exact_row!(
            "verb_phrase_ability_quoted_coordination",
            "have flying and \"{T}: Draw a card.\"",
            "VerbPhrase",
            build_verb_phrase_ability_quoted_coordination(
                open_predicate(Vocab::Have),
                vec![AbilityQuotedPairMember {
                    ability,
                    conjunction: Conjunction::And,
                    quoted: quoted_tap,
                }],
            )
            .unwrap()
        );

        let white = build_mana_amount_symbol(test_symbol("{W}")).unwrap();
        let blue = build_mana_amount_symbol(test_symbol("{U}")).unwrap();
        let black = build_mana_amount_symbol(test_symbol("{B}")).unwrap();
        let blue_black =
            build_mana_amount_sequence(vec![test_symbol("{U}"), test_symbol("{B}")]).unwrap();
        exact_row!(
            "verb_phrase_oracle_symbol",
            "add {W}",
            "VerbPhrase",
            build_verb_phrase_oracle_symbol(open_predicate(Vocab::Add), test_symbol("{W}"))
                .unwrap()
        );
        exact_row!(
            "verb_phrase_symbol_sequence",
            "add {U}{B}",
            "VerbPhrase",
            build_verb_phrase_symbol_sequence(
                open_predicate(Vocab::Add),
                vec![test_symbol("{U}"), test_symbol("{B}")],
            )
            .unwrap()
        );
        exact_row!("mana_amount_symbol", "{W}", "ManaAmount", white.clone());
        exact_row!(
            "mana_amount_sequence",
            "{U}{B}",
            "ManaAmount",
            blue_black.clone()
        );
        exact_row!(
            "mana_amount_list_single",
            "{W}",
            "ManaAmountList",
            build_mana_amount_list_single(white.clone()).unwrap()
        );
        let comma_list = build_mana_amount_list_comma(
            white.clone(),
            vec![PredicateObjectCoordination {
                conjunction: None,
                object: blue.clone(),
            }],
        )
        .unwrap();
        crate::grammar::parse_nonterminal_with_activation(
            "{W}, {U}",
            &catalogs,
            Nonterminal::ManaAmountList,
            GeneratedActivation::Groups(all_groups_with_predicate()),
        )
        .expect("the generated comma-list root parses and lowers");
        exact_row!(
            "mana_amount_list_comma",
            "{W}, {U}",
            "ManaAmountList",
            comma_list.clone()
        );
        let binary =
            build_mana_amount_coordination(white.clone(), Conjunction::Or, blue.clone()).unwrap();
        exact_row!(
            "mana_amount_coordination",
            "{W} or {U}",
            "CoordinatedManaAmount",
            binary.clone()
        );
        exact_row!(
            "mana_amount_coordination_oxford",
            "{W}, {U}, or {B}",
            "CoordinatedManaAmount",
            build_mana_amount_coordination_oxford(comma_list, Conjunction::Or, black,).unwrap()
        );
        exact_row!(
            "verb_phrase_mana_amount_coordination",
            "add {W} or {U}",
            "VerbPhrase",
            build_verb_phrase_mana_amount_coordination(open_predicate(Vocab::Add), binary).unwrap()
        );
        let stats = PowerToughness {
            power: crate::syntax::SignedScalar {
                sign: crate::syntax::ScalarSign::Plus,
                value: crate::syntax::ScalarValue::Integer(1),
            },
            toughness: crate::syntax::SignedScalar {
                sign: crate::syntax::ScalarSign::Plus,
                value: crate::syntax::ScalarValue::Integer(2),
            },
        };
        exact_row!(
            "verb_phrase_power_toughness",
            "get +1/+2",
            "VerbPhrase",
            build_verb_phrase_power_toughness(open_predicate(Vocab::Get), stats).unwrap()
        );
        let quantity = Quantity::try_exact(NumberLiteral {
            value: 2,
            numeral: crate::numeral::Numeral::Cardinal,
        })
        .unwrap();
        exact_row!(
            "verb_phrase_quantity",
            "scry two",
            "VerbPhrase",
            build_verb_phrase_quantity(open_predicate(Vocab::Scry), quantity).unwrap()
        );
    }

    #[test]
    fn predicate_value_surfaces_reject_malformed_list_and_connective_witnesses() {
        // Named surface mutations: DROP_COMMA, INSERT_CONNECTIVE_IN_OPEN_LIST,
        // DROP_OXFORD_COMMA, ONE_MEMBER_COORDINATION, and CHANGE_CONNECTIVE.
        let activation = GeneratedActivation::Groups(all_groups_with_predicate());
        for (source, category, mutation) in [
            ("{W} {U}", Nonterminal::ManaAmountList, "DROP_COMMA"),
            (
                "{W}, or {U}",
                Nonterminal::ManaAmountList,
                "INSERT_CONNECTIVE_IN_OPEN_LIST",
            ),
            (
                "{W}, {U} or {B}",
                Nonterminal::CoordinatedManaAmount,
                "DROP_OXFORD_COMMA",
            ),
            (
                "{W}",
                Nonterminal::CoordinatedManaAmount,
                "ONE_MEMBER_COORDINATION",
            ),
            (
                "{W} then {U}",
                Nonterminal::CoordinatedManaAmount,
                "CHANGE_CONNECTIVE",
            ),
            (
                "{W} and/or {U}",
                Nonterminal::CoordinatedManaAmount,
                "CHANGE_CONNECTIVE",
            ),
        ] {
            assert!(
                crate::grammar::parse_nonterminal_with_activation(
                    source,
                    &Catalogs::default(),
                    category,
                    activation,
                )
                .is_err(),
                "{mutation} unexpectedly admitted {source:?} as {category:?}"
            );
        }
    }

    #[test]
    fn production_predicate_value_anchors_match_selection_in_both_orders() {
        // Mutations caught: change a Task 4 row's local cost or semantic AST,
        // or let a generated result depend on registration order.
        let catalogs = crate::grammar::fixture_catalogs();
        let self_reference = crate::identity::SelfReference::default();
        for source in [
            "This creature has flying.",
            "Enchanted creature gains \"{T}: Draw a card.\"",
            "Enchanted creature has \"When this creature dies, draw a card\" and \"{T}: Draw a card.\"",
            "Enchanted creature has flying and \"{T}: Draw a card.\"",
            "Add {C}.",
            "Add {C}{C}.",
            "Add {R} or {G}.",
            "Add {W}, {B}, or {G}.",
            "Other Goblin creatures you control get +1/+1 and have haste.",
            "Scry 2.",
        ] {
            let production = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
                source,
                &catalogs,
                Nonterminal::Sentence,
                &self_reference,
                GeneratedActivation::Production,
            );
            let generated = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
                source,
                &catalogs,
                Nonterminal::Sentence,
                &self_reference,
                GeneratedActivation::Groups(all_groups_with_predicate()),
            );
            let production_shape = format!(
                "{:#?}",
                production[0]
                    .sentence()
                    .unwrap_or_else(|| panic!("production root is not a sentence for {source:?}"))
            );
            let generated_shape = format!(
                "{:#?}",
                generated[0]
                    .sentence()
                    .unwrap_or_else(|| panic!("generated root is not a sentence for {source:?}"))
            );
            assert_eq!(
                format!("{:#?}", production[1].sentence().unwrap()),
                production_shape,
                "production reversal changed {source:?}"
            );
            assert_eq!(
                format!("{:#?}", generated[1].sentence().unwrap()),
                generated_shape,
                "generated reversal changed {source:?}"
            );
            assert_eq!(
                generated_shape, production_shape,
                "AST mismatch for {source:?}"
            );
            assert_eq!(generated[0].cost(), production[0].cost(), "{source:?}");
            assert_eq!(generated[1].cost(), production[1].cost(), "{source:?}");
        }
    }

    #[test]
    fn attachment_builders_replay_exact_order_and_frequency_composition() {
        // Mutations caught: drop whether an adverb preceded or followed the
        // direct object, append `next` after an existing preverb, decode a
        // sibling dependent kind, or erase the lexical frequency's bound.
        let play = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Play, VerbSlot::Imperative, 0)).unwrap(),
        )
        .unwrap();
        let before_object = build_verb_phrase_direct_object(
            build_verb_phrase_adverb(play.clone(), Vocab::Only).unwrap(),
            this_card(),
        )
        .unwrap();
        assert!(matches!(
            before_object
                .clone()
                .declaration_into_dependent_projection()
                .0
                .as_slice(),
            [
                VerbDependent::Adverbial(Phrase::Adverb(Vocab::Only)),
                VerbDependent::DirectObject(_)
            ]
        ));
        let Some((before_source, before_object_value)) =
            parts_verb_phrase_direct_object(&before_object)
        else {
            panic!("the final object remains the outer construction")
        };
        assert_eq!(before_object_value, this_card());
        assert_eq!(
            build_verb_phrase_direct_object(before_source, before_object_value).unwrap(),
            before_object
        );

        let with_object = build_verb_phrase_direct_object(play.clone(), this_card()).unwrap();
        let after_object = build_verb_phrase_adverb(with_object.clone(), Vocab::Again).unwrap();
        assert!(matches!(
            after_object
                .clone()
                .declaration_into_dependent_projection()
                .0
                .as_slice(),
            [
                VerbDependent::DirectObject(_),
                VerbDependent::Adverbial(Phrase::Adverb(Vocab::Again))
            ]
        ));
        let (after_source, after_adverb) = parts_verb_phrase_adverb(&after_object)
            .expect("the final adverb remains the outer construction");
        assert_eq!(after_source, with_object);
        assert_eq!(after_adverb, Vocab::Again);
        assert_eq!(
            build_verb_phrase_adverb(after_source, after_adverb).unwrap(),
            after_object
        );

        let with_not = from_preverb_lens_parts(vec![PreverbModifier::Not], play.clone());
        let preverb = build_verb_phrase_preverb_adverb(PreverbModifier::Next, with_not.clone())
            .expect("the declared preverb prepends before existing modifiers");
        assert_eq!(
            preverb.clone().declaration_into_preverb_projection().0,
            vec![PreverbModifier::Next, PreverbModifier::Not]
        );
        let (modifier, preverb_source) = parts_verb_phrase_preverb_adverb(&preverb)
            .expect("the first preverb remains the outer construction");
        assert_eq!(modifier, PreverbModifier::Next);
        assert_eq!(preverb_source, with_not);
        assert_eq!(
            build_verb_phrase_preverb_adverb(modifier, preverb_source).unwrap(),
            preverb
        );

        let lexical_frequency = FrequencyPhrase {
            bound: FrequencyBound::MoreThan,
            count: crate::syntax::FrequencyCount::Twice,
        };
        let base_frequency = build_frequency_phrase(lexical_frequency).unwrap();
        assert_eq!(parts_frequency_phrase(&base_frequency), lexical_frequency);
        let limited_frequency = build_frequency_phrase_adverb(base_frequency).unwrap();
        assert_eq!(limited_frequency.bound, FrequencyBound::NoMoreThan);
        let recovered_frequency = parts_frequency_phrase_adverb(&limited_frequency);
        assert_eq!(recovered_frequency, lexical_frequency);
        assert_eq!(
            build_frequency_phrase_adverb(recovered_frequency).unwrap(),
            limited_frequency
        );
    }

    #[test]
    fn frequency_no_more_literal_rejects_already_bounded_child_and_rebuilds_from_parts() {
        let already_bounded = FrequencyPhrase {
            bound: FrequencyBound::NoMoreThan,
            count: crate::syntax::FrequencyCount::Once,
        };
        assert_eq!(
            build_frequency_phrase_adverb(already_bounded),
            Err(DeclarationViolation {
                construction: "frequency_phrase_adverb",
                requirement: "the child frequency has the `more than` bound",
            }),
            "No + NoMoreThan must not collapse onto the same semantic value as No + MoreThan"
        );

        let child = FrequencyPhrase {
            bound: FrequencyBound::MoreThan,
            count: crate::syntax::FrequencyCount::Twice,
        };
        let built = build_frequency_phrase_adverb(child)
            .expect("No + MoreThan is the admitted composition");
        let recovered_child = parts_frequency_phrase_adverb(&built);
        assert_eq!(recovered_child, child);
        assert_eq!(
            build_frequency_phrase_adverb(recovered_child).unwrap(),
            built,
            "an admitted build must survive parts and rebuild exactly"
        );
    }

    #[test]
    fn typed_attachment_builders_round_trip_particle_coin_frequency_shared_and_exception() {
        let phase = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Phase, VerbSlot::Imperative, 0)).unwrap(),
        )
        .unwrap();
        let particle = build_verb_phrase_particle(phase.clone(), VerbParticle::Out).unwrap();
        let (particle_source, recovered_particle) = parts_verb_phrase_particle(&particle)
            .expect("the final particle remains the outer construction");
        assert_eq!(particle_source, phase);
        assert_eq!(recovered_particle, VerbParticle::Out);
        assert_eq!(
            build_verb_phrase_particle(particle_source, recovered_particle).unwrap(),
            particle
        );

        let come = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Come, VerbSlot::Imperative, 0)).unwrap(),
        )
        .unwrap();
        let coin = build_verb_phrase_coin_result(come.clone(), CoinSide::Heads).unwrap();
        let (coin_source, side) = parts_verb_phrase_coin_result(&coin)
            .expect("the final coin result remains the outer construction");
        assert_eq!(coin_source, come);
        assert_eq!(side, CoinSide::Heads);
        assert_eq!(
            build_verb_phrase_coin_result(coin_source, side).unwrap(),
            coin
        );

        let frequency_value = FrequencyPhrase {
            bound: FrequencyBound::NoMoreThan,
            count: crate::syntax::FrequencyCount::Once,
        };
        let frequency = build_verb_phrase_frequency(play_base(), frequency_value).unwrap();
        let (frequency_source, recovered_frequency) = parts_verb_phrase_frequency(&frequency)
            .expect("the final frequency remains the outer construction");
        assert_eq!(recovered_frequency, frequency_value);
        assert_eq!(
            build_verb_phrase_frequency(frequency_source, recovered_frequency).unwrap(),
            frequency
        );
        build_verb_phrase_frequency(
            build_verb_phrase_base(
                build_verb(lexical_head(
                    Vocab::Control,
                    VerbSlot::Present {
                        person: Person::Third,
                        number: Number::Singular,
                    },
                    0,
                ))
                .unwrap(),
            )
            .unwrap(),
            frequency_value,
        )
        .expect("finite object-gap-compatible values share the frequency builder contract");

        let passive = build_verb_phrase_auxiliary(
            full_auxiliary(
                Auxiliary::Be,
                AuxiliaryInflection::Present {
                    person: Person::Third,
                    number: Number::Singular,
                },
            ),
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Block, VerbSlot::PastParticiple, 1)).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        let shared_pp = parsed_preposition("to target player or planeswalker");
        let shared = build_verb_phrase_passive_shared_determiner_prepositional(
            passive.clone(),
            shared_pp.clone(),
        )
        .unwrap();
        let (shared_source, recovered_pp) =
            parts_verb_phrase_passive_shared_determiner_prepositional(&shared)
                .expect("the final shared-determiner PP remains the outer construction");
        assert_eq!(shared_source, passive);
        assert_eq!(recovered_pp, shared_pp);
        assert_eq!(
            build_verb_phrase_passive_shared_determiner_prepositional(shared_source, recovered_pp,)
                .unwrap(),
            shared
        );

        let except_pp = parsed_preposition("by creatures with flying");
        let exception = build_verb_phrase_except_by(passive.clone(), except_pp.clone()).unwrap();
        let (exception_source, recovered_exception) = parts_verb_phrase_except_by(&exception)
            .expect("the exception remains the outer construction");
        assert_eq!(exception_source, passive);
        assert_eq!(recovered_exception, except_pp);
        assert_eq!(
            build_verb_phrase_except_by(exception_source, recovered_exception).unwrap(),
            exception
        );
    }

    #[test]
    fn attachment_builders_reject_named_phase_role_frame_voice_and_composition_mutations() {
        let phase = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Phase, VerbSlot::Imperative, 0)).unwrap(),
        )
        .unwrap();
        let particle = build_verb_phrase_particle(phase.clone(), VerbParticle::Out).unwrap();
        assert!(
            build_verb_phrase_particle(particle.clone(), VerbParticle::Out).is_err(),
            "duplicating a particle must not cross the particle's tail boundary"
        );
        assert!(
            build_verb_phrase_direct_object(particle, this_card()).is_err(),
            "moving an object across a particle tail must fail"
        );
        assert!(
            build_verb_phrase_particle(play_base(), VerbParticle::Out).is_err(),
            "an unlicensed head must not accept a particle"
        );
        assert!(
            build_verb_phrase_coin_result(play_base(), CoinSide::Tails).is_err(),
            "only the pending Come frame accepts a coin result"
        );
        assert!(
            build_verb_phrase_preverb_adverb(PreverbModifier::Not, play_base()).is_err(),
            "the lexical preverb declaration admits only Next"
        );

        let active = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Block, VerbSlot::PastParticiple, 1)).unwrap(),
        )
        .unwrap();
        let shared_pp = parsed_preposition("to target player or planeswalker");
        assert!(
            build_verb_phrase_passive_shared_determiner_prepositional(active, shared_pp.clone(),)
                .is_err(),
            "losing passive voice must reject shared-determiner attachment"
        );
        let passive = build_verb_phrase_auxiliary(
            full_auxiliary(
                Auxiliary::Be,
                AuxiliaryInflection::Present {
                    person: Person::Third,
                    number: Number::Singular,
                },
            ),
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Block, VerbSlot::PastParticiple, 1)).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        assert!(
            build_verb_phrase_passive_shared_determiner_prepositional(
                passive.clone(),
                parsed_preposition("by creatures"),
            )
            .is_err(),
            "an ordinary PP object must not masquerade as shared-determiner syntax"
        );
        let roleless_passive = build_verb_phrase_auxiliary(
            full_auxiliary(
                Auxiliary::Be,
                AuxiliaryInflection::Present {
                    person: Person::Third,
                    number: Number::Singular,
                },
            ),
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Look, VerbSlot::PastParticiple, 0)).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        assert!(
            build_verb_phrase_passive_shared_determiner_prepositional(roleless_passive, shared_pp,)
                .is_err(),
            "a PP without selected or adjunct role must be rejected"
        );

        assert!(
            build_verb_phrase_except_by(passive.clone(), parsed_preposition("during this turn"),)
                .is_err(),
            "the exception construction is By-only"
        );
        assert!(
            build_verb_phrase_except_by(play_base(), parsed_preposition("by creatures")).is_err(),
            "an active predicate must not accept the passive exception"
        );
        let incomplete_recipient_passive = build_verb_phrase_auxiliary(
            full_auxiliary(
                Auxiliary::Be,
                AuxiliaryInflection::Present {
                    person: Person::Third,
                    number: Number::Singular,
                },
            ),
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Deal, VerbSlot::PastParticiple, 1)).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        assert!(
            build_verb_phrase_except_by(
                incomplete_recipient_passive,
                parsed_preposition("by creatures"),
            )
            .is_err(),
            "the exception requires an argument-complete passive host"
        );

        let mut exception_terminal = passive
            .declaration_core_features()
            .expect("the passive fixture has typed predicate features");
        let Features::VerbPhrase { phase, .. } = &mut exception_terminal else {
            unreachable!()
        };
        *phase = crate::grammar::PredicateAttachmentPhase::ExceptionTail;
        assert!(
            reduce_verb_phrase_adverb_features(&exception_terminal, &Features::None).is_none(),
            "no attachment may move beyond the exception terminal"
        );
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "the single mutation matrix keeps the frame and valency cases auditable together"
    )]
    fn core_valency_builders_round_trip_and_reject_the_negative_matrix() {
        // Mutations caught: accept missing/surplus direct or indirect objects,
        // accept a direct object under the wrong lexical frame, admit the
        // wrong auxiliary-selected form or an illegal passive retained theme,
        // accept a subject-case object, or admit an unselected preposition.
        let intransitive = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Attack, VerbSlot::Imperative, 0)).unwrap(),
        )
        .unwrap();
        assert_eq!(
            selected_verb_phrase_construction(&intransitive),
            "verb_phrase_base"
        );
        assert!(intransitive.declaration_core_arguments_complete());

        let required_object = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Have, VerbSlot::Imperative, 0)).unwrap(),
        )
        .unwrap();
        assert!(!required_object.declaration_core_arguments_complete());
        let transitive = build_verb_phrase_direct_object(required_object.clone(), this_card())
            .expect("the required direct object completes the lexical frame");
        assert_eq!(
            selected_verb_phrase_construction(&transitive),
            "verb_phrase_direct_object"
        );
        assert!(transitive.declaration_core_arguments_complete());
        let Some((transitive_base, object)) = parts_verb_phrase_direct_object(&transitive) else {
            panic!("the dependent lens must recover the direct-object focus");
        };
        assert_eq!(transitive_base, required_object);
        assert_eq!(object, this_card());
        assert_eq!(
            build_verb_phrase_direct_object(transitive_base, object).unwrap(),
            transitive
        );
        assert!(build_verb_phrase_direct_object(transitive.clone(), this_card()).is_err());

        let ditransitive_base = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Ask, VerbSlot::Imperative, 1)).unwrap(),
        )
        .unwrap();
        assert!(!ditransitive_base.declaration_core_arguments_complete());
        let with_recipient =
            build_verb_phrase_indirect_object(ditransitive_base.clone(), this_card()).unwrap();
        assert_eq!(
            selected_verb_phrase_construction(&with_recipient),
            "verb_phrase_indirect_object"
        );
        assert!(!with_recipient.declaration_core_arguments_complete());
        assert!(build_verb_phrase_indirect_object(with_recipient.clone(), this_card()).is_err());
        let ditransitive = build_verb_phrase_direct_object(with_recipient, this_card()).unwrap();
        assert!(ditransitive.declaration_core_arguments_complete());

        let forbidden_object = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Attack, VerbSlot::Imperative, 0)).unwrap(),
        )
        .unwrap();
        assert!(build_verb_phrase_direct_object(forbidden_object, this_card()).is_err());
        let subject_pronoun = NounPhrase::Pronoun {
            pronoun: crate::word::Pronoun::They,
            case: crate::features::PronounCase::Subject,
        };
        assert!(build_verb_phrase_direct_object(required_object.clone(), subject_pronoun).is_err());

        let adjective = AdjectivePhrase {
            degree: None,
            head: crate::word::Adjective::Color(crate::word::ColorWord::Red),
            complements: Vec::new(),
        };
        let copular = build_verb_phrase_adjective(
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Be, VerbSlot::Infinitive, 0)).unwrap(),
            )
            .unwrap(),
            adjective,
        )
        .expect("the open copular frame admits an adjective complement");
        assert_eq!(
            selected_verb_phrase_construction(&copular),
            "verb_phrase_adjective"
        );
        assert!(copular.declaration_core_arguments_complete());

        let selected_pp = PrepositionalPhrase::simple(
            crate::syntax::Preposition::At,
            Phrase::NounPhrase(Box::new(this_card())),
        );
        let look = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Look, VerbSlot::Imperative, 1)).unwrap(),
        )
        .unwrap();
        let selected = build_verb_phrase_prepositional(look.clone(), selected_pp).unwrap();
        assert_eq!(
            selected_verb_phrase_construction(&selected),
            "verb_phrase_prepositional"
        );
        assert!(selected.declaration_core_arguments_complete());
        let unselected = PrepositionalPhrase::simple(
            crate::syntax::Preposition::From,
            Phrase::NounPhrase(Box::new(this_card())),
        );
        let pp_forbidden = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Look, VerbSlot::Imperative, 0)).unwrap(),
        )
        .unwrap();
        assert!(build_verb_phrase_prepositional(pp_forbidden, unselected).is_err());

        let infinitive_body = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Attack, VerbSlot::Infinitive, 0)).unwrap(),
        )
        .unwrap();
        let infinitive = InfinitiveClause::declaration_to(infinitive_body);
        let infinitival = build_verb_phrase_infinitive(
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Begin, VerbSlot::Imperative, 0)).unwrap(),
            )
            .unwrap(),
            infinitive,
        )
        .unwrap();
        assert_eq!(
            selected_verb_phrase_construction(&infinitival),
            "verb_phrase_infinitive"
        );
        assert!(infinitival.declaration_core_arguments_complete());

        let progressive = build_verb_phrase_auxiliary(
            full_auxiliary(
                Auxiliary::Be,
                AuxiliaryInflection::Present {
                    person: Person::Third,
                    number: Number::Singular,
                },
            ),
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Attack, VerbSlot::PresentParticiple, 0)).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            selected_verb_phrase_construction(&progressive),
            "verb_phrase_auxiliary"
        );
        assert!(progressive.declaration_core_arguments_complete());
        assert!(
            build_verb_phrase_auxiliary(
                full_auxiliary(Auxiliary::Be, AuxiliaryInflection::Base),
                build_verb_phrase_base(
                    build_verb(lexical_head(Vocab::Attack, VerbSlot::Infinitive, 0)).unwrap(),
                )
                .unwrap(),
            )
            .is_err()
        );

        let passive_with_illegal_theme = build_verb_phrase_direct_object(
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Have, VerbSlot::PastParticiple, 0)).unwrap(),
            )
            .unwrap(),
            this_card(),
        )
        .unwrap();
        assert!(
            build_verb_phrase_auxiliary(
                full_auxiliary(
                    Auxiliary::Be,
                    AuxiliaryInflection::Present {
                        person: Person::Third,
                        number: Number::Singular,
                    },
                ),
                passive_with_illegal_theme,
            )
            .is_err()
        );

        let proform = build_verb_phrase_auxiliary_proform(full_auxiliary(
            Auxiliary::Can,
            AuxiliaryInflection::Base,
        ))
        .unwrap();
        assert_eq!(
            selected_verb_phrase_construction(&proform),
            "verb_phrase_auxiliary_proform"
        );
        assert_eq!(
            parts_verb_phrase_auxiliary_proform(&proform),
            full_auxiliary(Auxiliary::Can, AuxiliaryInflection::Base)
        );
    }

    #[test]
    fn every_valency_destructurer_rebuilds_and_preserves_dependent_order() {
        // Mutations caught: swap a lens prefix/suffix, drop a dependent while
        // projecting the checked owner, or wire a generated parts function to
        // a sibling construction's field order. These assertions add missing
        // inverse coverage; the reviewed implementation already satisfies
        // them, so they are regression evidence rather than a historical RED.
        let auxiliary = full_auxiliary(
            Auxiliary::Be,
            AuxiliaryInflection::Present {
                person: Person::Third,
                number: Number::Singular,
            },
        );
        let progressive_body = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Attack, VerbSlot::PresentParticiple, 0)).unwrap(),
        )
        .unwrap();
        let progressive = build_verb_phrase_auxiliary(auxiliary, progressive_body.clone()).unwrap();
        let (recovered_auxiliary, recovered_body) = parts_verb_phrase_auxiliary(&progressive);
        assert_eq!(recovered_auxiliary, auxiliary);
        assert_eq!(recovered_body, progressive_body);
        assert_eq!(
            build_verb_phrase_auxiliary(recovered_auxiliary, recovered_body).unwrap(),
            progressive
        );

        let ditransitive_base = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Ask, VerbSlot::Imperative, 1)).unwrap(),
        )
        .unwrap();
        let indirect =
            build_verb_phrase_indirect_object(ditransitive_base.clone(), this_card()).unwrap();
        let Some((recovered_base, recovered_indirect)) =
            parts_verb_phrase_indirect_object(&indirect)
        else {
            panic!("the indirect-object lens must expose its focus")
        };
        assert_eq!(recovered_base, ditransitive_base);
        assert_eq!(recovered_indirect, this_card());
        assert_eq!(
            build_verb_phrase_indirect_object(recovered_base, recovered_indirect).unwrap(),
            indirect
        );

        let adjective = AdjectivePhrase {
            degree: None,
            head: crate::word::Adjective::Color(crate::word::ColorWord::Red),
            complements: Vec::new(),
        };
        let copular_base = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Be, VerbSlot::Infinitive, 0)).unwrap(),
        )
        .unwrap();
        let copular = build_verb_phrase_adjective(copular_base.clone(), adjective.clone()).unwrap();
        let Some((recovered_base, recovered_adjective)) = parts_verb_phrase_adjective(&copular)
        else {
            panic!("the adjective lens must expose its focus")
        };
        assert_eq!(recovered_base, copular_base);
        assert_eq!(recovered_adjective, adjective);
        assert_eq!(
            build_verb_phrase_adjective(recovered_base, recovered_adjective).unwrap(),
            copular
        );

        let selected_pp = PrepositionalPhrase::simple(
            crate::syntax::Preposition::At,
            Phrase::NounPhrase(Box::new(this_card())),
        );
        let look = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Look, VerbSlot::Imperative, 1)).unwrap(),
        )
        .unwrap();
        let prepositional =
            build_verb_phrase_prepositional(look.clone(), selected_pp.clone()).unwrap();
        let Some((recovered_base, recovered_preposition)) =
            parts_verb_phrase_prepositional(&prepositional)
        else {
            panic!("the prepositional lens must expose its focus")
        };
        assert_eq!(recovered_base, look);
        assert_eq!(recovered_preposition, selected_pp);
        assert_eq!(
            build_verb_phrase_prepositional(recovered_base, recovered_preposition).unwrap(),
            prepositional
        );

        let infinitive = InfinitiveClause::declaration_to(
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Attack, VerbSlot::Infinitive, 0)).unwrap(),
            )
            .unwrap(),
        );
        let begin = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Begin, VerbSlot::Imperative, 0)).unwrap(),
        )
        .unwrap();
        let infinitival = build_verb_phrase_infinitive(begin.clone(), infinitive.clone()).unwrap();
        let Some((recovered_base, recovered_infinitive)) =
            parts_verb_phrase_infinitive(&infinitival)
        else {
            panic!("the infinitive lens must expose its focus")
        };
        assert_eq!(recovered_base, begin);
        assert_eq!(recovered_infinitive, infinitive);
        assert_eq!(
            build_verb_phrase_infinitive(recovered_base, recovered_infinitive).unwrap(),
            infinitival
        );

        let with_indirect =
            build_verb_phrase_indirect_object(ditransitive_base.clone(), this_card()).unwrap();
        let with_direct =
            build_verb_phrase_direct_object(with_indirect.clone(), this_card()).unwrap();
        let trailing_pp = PrepositionalPhrase::simple(
            crate::syntax::Preposition::During,
            Phrase::NounPhrase(Box::new(this_card())),
        );
        let ordered =
            build_verb_phrase_prepositional(with_direct.clone(), trailing_pp.clone()).unwrap();
        let Some((before_pp, recovered_pp)) = parts_verb_phrase_prepositional(&ordered) else {
            panic!("the last PP must remain the outer focus")
        };
        assert_eq!(before_pp, with_direct);
        assert_eq!(recovered_pp, trailing_pp);
        let Some((before_direct, recovered_direct)) = parts_verb_phrase_direct_object(&before_pp)
        else {
            panic!("the direct object must follow the indirect object")
        };
        assert_eq!(before_direct, with_indirect);
        assert_eq!(recovered_direct, this_card());
        let Some((before_indirect, recovered_indirect)) =
            parts_verb_phrase_indirect_object(&before_direct)
        else {
            panic!("the indirect object must remain nearest the base")
        };
        assert_eq!(before_indirect, ditransitive_base);
        assert_eq!(recovered_indirect, this_card());
        assert_eq!(
            build_verb_phrase_prepositional(
                build_verb_phrase_direct_object(
                    build_verb_phrase_indirect_object(before_indirect, recovered_indirect).unwrap(),
                    recovered_direct,
                )
                .unwrap(),
                recovered_pp,
            )
            .unwrap(),
            ordered
        );
    }

    #[test]
    fn generated_contexts_cover_agreement_object_gap_and_reduced_recipient_passive() {
        // Mutations caught: bypass finite subject agreement, fail to project
        // the declaration family into the object-gap category, retain an
        // arbitrary reduced-recipient-passive theme, or break the generated
        // lowering path.
        let activation = GeneratedActivation::Groups(all_groups_with_predicate());
        assert!(
            crate::grammar::parse_nonterminal_with_activation(
                "it attacks",
                &Catalogs::default(),
                Nonterminal::Clause,
                activation,
            )
            .is_ok()
        );
        assert!(
            crate::grammar::parse_nonterminal_with_activation(
                "they attacks",
                &Catalogs::default(),
                Nonterminal::Clause,
                activation,
            )
            .is_err()
        );

        let object_gap = crate::grammar::parse_nonterminal_with_activation(
            "controls",
            &Catalogs::default(),
            Nonterminal::ObjectGapVerbPhrase,
            activation,
        )
        .expect("the generated verb/base path projects into object-gap parsing");
        assert!(object_gap.construction_decisions().iter().any(|decision| {
            decision.selected().as_str() == "verb_phrase_base"
                && decision.owner() == crate::construction::ConstructionOwner::Generated
        }));

        crate::grammar::parse_nonterminal_with_activation(
            "dealt",
            &Catalogs::default(),
            Nonterminal::ReducedRecipientPassive,
            activation,
        )
        .expect("the generated reduced-recipient-passive base lowers");
        let reduced = crate::grammar::parse_nonterminal_with_activation(
            "dealt damage",
            &Catalogs::default(),
            Nonterminal::ReducedRecipientPassive,
            activation,
        )
        .expect("the generated base/direct-object path retains the licensed damage theme");
        assert!(reduced.construction_decisions().iter().any(|decision| {
            decision.selected().as_str() == "verb_phrase_direct_object"
                && decision.owner() == crate::construction::ConstructionOwner::Generated
        }));
        let reduced_value = reduced
            .verb_phrase()
            .expect("the reduced-passive prefix lowers a VerbPhrase");
        build_verb_phrase_frequency(
            reduced_value.clone(),
            FrequencyPhrase {
                bound: FrequencyBound::NoMoreThan,
                count: crate::syntax::FrequencyCount::Twice,
            },
        )
        .expect("the generated frequency builder admits the reduced-passive value");
        assert!(
            crate::grammar::parse_nonterminal_with_activation(
                "dealt a card",
                &Catalogs::default(),
                Nonterminal::ReducedRecipientPassive,
                activation,
            )
            .is_err()
        );

        let generated_frequency = crate::grammar::parse_nonterminal_with_activation(
            "no more than three times",
            &Catalogs::default(),
            Nonterminal::FrequencyPhrase,
            activation,
        )
        .expect("the generated lexical frequency construction lowers independently");
        assert!(
            generated_frequency
                .construction_decisions()
                .iter()
                .any(|decision| {
                    decision.selected().as_str() == "frequency_phrase"
                        && decision.owner() == crate::construction::ConstructionOwner::Generated
                })
        );

        for (source, construction) in [
            ("controls again", "verb_phrase_adverb"),
            ("phases out", "verb_phrase_particle"),
            ("controls no more than three times", "verb_phrase_frequency"),
        ] {
            let parsed = crate::grammar::parse_nonterminal_with_activation(
                source,
                &Catalogs::default(),
                Nonterminal::ObjectGapVerbPhrase,
                activation,
            )
            .unwrap_or_else(|error| {
                panic!("generated object-gap attachment failed for {source:?}: {error:?}")
            });
            assert!(
                parsed.construction_decisions().iter().any(|decision| {
                    decision.selected().as_str() == construction
                        && decision.owner() == crate::construction::ConstructionOwner::Generated
                }),
                "{source:?} selected decisions: {:#?}",
                parsed.construction_decisions()
            );
        }

        for (source, construction) in [
            ("dealt damage again", "verb_phrase_adverb"),
            (
                "dealt damage no more than three times",
                "verb_phrase_frequency",
            ),
        ] {
            let parsed = crate::grammar::parse_nonterminal_with_activation(
                source,
                &Catalogs::default(),
                Nonterminal::ReducedRecipientPassive,
                activation,
            )
            .unwrap_or_else(|error| {
                panic!("generated reduced-passive attachment failed for {source:?}: {error:?}")
            });
            assert!(
                parsed.construction_decisions().iter().any(|decision| {
                    decision.selected().as_str() == construction
                        && decision.owner() == crate::construction::ConstructionOwner::Generated
                }),
                "{source:?} selected decisions: {:#?}",
                parsed.construction_decisions()
            );
        }
        assert!(
            crate::grammar::parse_nonterminal_with_activation(
                "deal damage again",
                &Catalogs::default(),
                Nonterminal::ReducedRecipientPassive,
                activation,
            )
            .is_err(),
            "the reduced-passive attachment context retains its participial-form gate"
        );
    }

    #[test]
    fn generated_direct_object_preserves_a_later_temporal_dependent() {
        // Mutation caught: project predicate dependents through one occupied
        // optional focus, so attaching `this turn` after `that card` fails
        // during generated lowering instead of preserving both in order.
        let parsed = crate::grammar::parse_nonterminal_with_activation(
            "Play that card this turn.",
            &Catalogs::default(),
            Nonterminal::Sentence,
            GeneratedActivation::Groups(all_groups_with_predicate()),
        )
        .expect("an ordered lens preserves the object before the temporal dependent");
        let sentence = parsed.sentence().expect("the generated parse lowers");
        let crate::syntax::SentenceBody::Independent(crate::syntax::IndependentClause::Imperative(
            crate::syntax::Predicate::Transitive(predicate),
        )) = &sentence.body
        else {
            panic!("expected an imperative transitive predicate: {sentence:#?}")
        };
        assert!(matches!(
            predicate.object,
            crate::syntax::PredicateObject::NounPhrase(_)
        ));
        assert!(matches!(
            predicate.elements.as_slice(),
            [crate::syntax::PredicateElement::Adjunct(
                crate::syntax::PredicateAdjunct::Temporal(_)
            )]
        ));
    }

    #[test]
    fn generated_adjective_pp_selection_matches_in_both_orders() {
        // Mutations caught: charge adjective like PP instead of +2 versus +1,
        // or let that defect make equal-cost alternatives depend on
        // registration order. The conditional active-temporal cost has its
        // own exact projection test in `grammar::generated`.
        let source = "Nissa's power is equal to the number of lands you control.";
        let self_reference = crate::identity::SelfReference::new("Nissa Revane", true);
        let catalogs = crate::grammar::fixture_catalogs();
        let production = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
            source,
            &catalogs,
            Nonterminal::Sentence,
            &self_reference,
            GeneratedActivation::Production,
        );
        let generated = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
            source,
            &catalogs,
            Nonterminal::Sentence,
            &self_reference,
            GeneratedActivation::Groups(all_groups_with_predicate()),
        );
        let production_shape = format!(
            "{:#?}",
            production[0]
                .sentence()
                .expect("the production parse lowers a sentence")
        );
        let generated_shape = format!(
            "{:#?}",
            generated[0]
                .sentence()
                .expect("the generated parse lowers a sentence")
        );
        assert_eq!(
            format!(
                "{:#?}",
                production[1]
                    .sentence()
                    .expect("the reversed production parse lowers a sentence")
            ),
            production_shape,
            "production selection changed under reversal for {source:?}"
        );
        assert_eq!(
            format!(
                "{:#?}",
                generated[1]
                    .sentence()
                    .expect("the reversed generated parse lowers a sentence")
            ),
            generated_shape,
            "generated selection changed under reversal for {source:?}"
        );
        assert_eq!(generated_shape, production_shape, "{source:?}");
        assert_eq!(generated[0].cost(), production[0].cost(), "{source:?}");
        assert_eq!(generated[1].cost(), production[1].cost(), "{source:?}");
    }

    #[test]
    fn predicate_dominance_edges_remove_only_the_named_loser() {
        // Mutations caught: remove or reverse any one of the four declared
        // edges, let dominance leak to an incomparable attachment, choose by
        // registration order, or classify lexical `do` with a proform frame
        // as the synthesized auxiliary-proform shape during checked lowering.
        let catalogs = crate::grammar::fixture_catalogs();
        for (source, winner, loser) in [
            (
                "do again",
                "verb_phrase_base",
                "verb_phrase_auxiliary_proform",
            ),
            (
                "is becoming red",
                "verb_phrase_auxiliary",
                "verb_phrase_adjective",
            ),
            (
                "is attacking again",
                "verb_phrase_auxiliary",
                "verb_phrase_adverb",
            ),
            (
                "has gained flying",
                "verb_phrase_auxiliary",
                "verb_phrase_ability",
            ),
        ] {
            let parses = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
                source,
                &catalogs,
                Nonterminal::VerbPhrase,
                &crate::identity::SelfReference::default(),
                GeneratedActivation::Groups(all_groups_with_predicate()),
            );
            assert_eq!(
                parses[0].verb_phrase(),
                parses[1].verb_phrase(),
                "selected semantic AST changed under reversed registration for {source:?}",
            );
            let mut signatures = Vec::new();
            for parsed in &parses {
                let decision = parsed
                    .construction_decisions()
                    .iter()
                    .find(|decision| {
                        decision.selected().as_str() == winner
                            && decision
                                .alternatives()
                                .iter()
                                .any(|alternative| alternative.id().as_str() == loser)
                    })
                    .unwrap_or_else(|| {
                        panic!(
                            "{source:?} did not exercise declared edge {winner}>{loser}: {:#?}",
                            parsed.construction_decisions(),
                        )
                    });
                assert_eq!(
                    decision.reason(),
                    crate::forest::SelectionReason::Dominance,
                    "{winner}>{loser} must be the actual selection reason",
                );
                let mut candidates = decision
                    .alternatives()
                    .iter()
                    .map(|alternative| {
                        (
                            alternative.id().as_str(),
                            alternative.production_ordinal(),
                            alternative.is_dominated(),
                        )
                    })
                    .collect::<Vec<_>>();
                candidates.sort_unstable();
                let mut expected = vec![(winner, 0, false), (loser, 0, true)];
                expected.sort_unstable();
                assert_eq!(
                    candidates, expected,
                    "{winner}>{loser} must remove exactly its named subordinate",
                );
                assert_eq!(parsed.root_equal_cost_alternatives().len(), 2, "{source:?}");
                assert_eq!(parsed.root_tied_alternatives().len(), 1, "{source:?}");
                signatures.push(candidates);
            }
            assert_eq!(
                signatures[0], signatures[1],
                "tied and removed alternatives changed under reversed registration for {source:?}",
            );
        }
    }

    #[test]
    fn predicate_incomparable_tie_remains_visible_in_both_orders() {
        // Mutation caught: broaden an auxiliary dominance edge to the direct
        // object attachment or let stable registration identity change the
        // selected semantic value or the visible tied-candidate set.
        let source = "is attacking target player";
        let parses = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
            source,
            &crate::grammar::fixture_catalogs(),
            Nonterminal::VerbPhrase,
            &crate::identity::SelfReference::default(),
            GeneratedActivation::Groups(all_groups_with_predicate()),
        );
        assert_eq!(
            parses[0].verb_phrase(),
            parses[1].verb_phrase(),
            "the selected semantic AST must be registration-order neutral",
        );
        let expected = [
            ("verb_phrase_auxiliary", 0, false),
            ("verb_phrase_direct_object", 0, false),
        ];
        for parsed in &parses {
            let decision = parsed
                .construction_decisions()
                .iter()
                .find(|decision| {
                    decision.selected().as_str() == "verb_phrase_auxiliary"
                        && decision.alternatives().len() == 2
                })
                .unwrap_or_else(|| {
                    panic!(
                        "the incomparable predicate tie was not visible: {:#?}",
                        parsed.construction_decisions(),
                    )
                });
            assert_eq!(
                decision.reason(),
                crate::forest::SelectionReason::StableIdentity,
            );
            assert_eq!(
                decision
                    .alternatives()
                    .iter()
                    .map(|alternative| (
                        alternative.id().as_str(),
                        alternative.production_ordinal(),
                        alternative.is_dominated(),
                    ))
                    .collect::<Vec<_>>(),
                expected,
            );
            assert_eq!(parsed.root_equal_cost_alternatives().len(), 2);
            assert_eq!(parsed.root_tied_alternatives().len(), 2);
        }
    }

    #[test]
    fn generated_attachment_anchors_are_registration_order_neutral() {
        // Mutations caught: assign a Task 3 attachment a registration-order
        // tie-break, lose pre/post-object order during lowering, or let a
        // generated attachment diverge in AST shape or local cost from its
        // production counterpart.
        let self_reference = crate::identity::SelfReference::default();
        let catalogs = crate::grammar::fixture_catalogs();
        for source in [
            "Play only that card.",
            "Play that card again.",
            "You next cast a creature spell this turn.",
            "Phase out.",
            "The coin comes up heads.",
            "Activate no more than three times.",
            "Prevent all damage that would be dealt to target player or planeswalker this turn.",
            "This creature can't be blocked except by creatures with flying.",
        ] {
            let production = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
                source,
                &catalogs,
                Nonterminal::Sentence,
                &self_reference,
                GeneratedActivation::Production,
            );
            let generated = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
                source,
                &catalogs,
                Nonterminal::Sentence,
                &self_reference,
                GeneratedActivation::Groups(all_groups_with_predicate()),
            );
            let production_shape = format!(
                "{:#?}",
                production[0]
                    .sentence()
                    .unwrap_or_else(|| panic!("production root is not a sentence for {source:?}"))
            );
            let generated_shape = format!(
                "{:#?}",
                generated[0]
                    .sentence()
                    .unwrap_or_else(|| panic!("generated root is not a sentence for {source:?}"))
            );
            assert_eq!(
                format!(
                    "{:#?}",
                    production[1]
                        .sentence()
                        .expect("the reversed production root is a sentence")
                ),
                production_shape,
                "production reversal changed {source:?}"
            );
            assert_eq!(
                format!(
                    "{:#?}",
                    generated[1]
                        .sentence()
                        .expect("the reversed generated root is a sentence")
                ),
                generated_shape,
                "generated reversal changed {source:?}"
            );
            assert_eq!(generated_shape, production_shape, "{source:?}");
            assert_eq!(generated[0].cost(), production[0].cost(), "{source:?}");
            assert_eq!(generated[1].cost(), production[1].cost(), "{source:?}");
        }
    }

    #[test]
    fn predicate_family_has_exactly_the_required_34_id_census() {
        // Mutation caught: omit or rename the final causative declaration, or
        // accidentally admit an extra predicate-family owner while the group
        // is admitted to the production group.
        assert_eq!(
            PREDICATE_DECLARATION
                .constructions
                .iter()
                .map(|construction| construction.id)
                .collect::<Vec<_>>(),
            [
                "verb",
                "verb_phrase_base",
                "verb_phrase_auxiliary",
                "verb_phrase_auxiliary_proform",
                "verb_phrase_direct_object",
                "verb_phrase_indirect_object",
                "verb_phrase_adjective",
                "verb_phrase_prepositional",
                "verb_phrase_passive_shared_determiner_prepositional",
                "verb_phrase_except_by",
                "verb_phrase_infinitive",
                "verb_phrase_adverb",
                "verb_phrase_preverb_adverb",
                "verb_phrase_particle",
                "verb_phrase_coin_result",
                "verb_phrase_frequency",
                "frequency_phrase_adverb",
                "frequency_phrase",
                "verb_phrase_ability",
                "verb_phrase_quoted_ability",
                "verb_phrase_quoted_ability_coordination",
                "verb_phrase_ability_quoted_coordination",
                "verb_phrase_oracle_symbol",
                "verb_phrase_symbol_sequence",
                "mana_amount_symbol",
                "mana_amount_sequence",
                "mana_amount_list_single",
                "mana_amount_list_comma",
                "mana_amount_coordination",
                "mana_amount_coordination_oxford",
                "verb_phrase_mana_amount_coordination",
                "verb_phrase_power_toughness",
                "verb_phrase_quantity",
                "verb_phrase_causative",
            ]
        );
    }

    #[test]
    fn generated_sentence_root_reaches_the_causative_owner() {
        // Mutation caught: keep causative parsing available only through the
        // former handwritten RuleTag path, so the production Sentence
        // consumer cannot reach the declaration-owned construction.
        let parsed = crate::grammar::parse_nonterminal_with_activation(
            "You may have this creature enter.",
            &crate::grammar::fixture_catalogs(),
            Nonterminal::Sentence,
            GeneratedActivation::Groups(all_groups_with_predicate()),
        )
        .expect("the production generated family parses a causative sentence");
        assert!(parsed.construction_decisions().iter().any(|decision| {
            decision.selected().as_str() == "verb_phrase_causative"
                && decision.owner() == crate::construction::ConstructionOwner::Generated
        }));
        let crate::syntax::SentenceBody::Independent(crate::syntax::IndependentClause::Deontic(
            _,
            _,
            Some(predicate),
        )) = &parsed
            .sentence()
            .expect("the vertical root lowers a semantic sentence")
            .body
        else {
            panic!("expected the causative sentence's semantic deontic AST")
        };
        let crate::syntax::Predicate::Transitive(predicate) = predicate else {
            panic!("the causative host retains its direct-object causee")
        };
        assert!(matches!(
            predicate.elements.as_slice(),
            [crate::syntax::PredicateElement::Complement(
                crate::syntax::PredicateComplement::Infinitive(crate::syntax::InfinitiveClause {
                    marker: crate::syntax::InfinitiveMarker::Bare,
                    ..
                })
            )]
        ));
    }

    #[test]
    fn causative_builder_inverse_and_invalid_domain_matrix() {
        // Mutations caught: NON_CAUSATIVE_HOST accepts an ordinary transitive
        // host, FINITE_COMPLEMENT accepts a finite embedded predicate,
        // SWAP_CAUSEE_COMPLEMENT admits the complement before the causee,
        // SURPLUS_PRE_OBJECT_HOST_DEPENDENT admits an adverb before the
        // causee, and SURPLUS_HOST_DEPENDENT accepts material between the
        // causee and bare infinitive. The admitted case also pins host ->
        // causee -> complement event order through parts/rebuild and named
        // inverse selection.
        let causative_base = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Have, VerbSlot::Infinitive, 1)).unwrap(),
        )
        .unwrap();
        let host = build_verb_phrase_direct_object(causative_base.clone(), this_card()).unwrap();
        let complement = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Attack, VerbSlot::Infinitive, 0)).unwrap(),
        )
        .unwrap();

        let causative = build_verb_phrase_causative(host.clone(), complement.clone())
            .expect("causative have admits one causee then a complete bare infinitive");
        let (recovered_host, recovered_complement) = parts_verb_phrase_causative(&causative);
        assert_eq!(recovered_host, host);
        assert_eq!(recovered_complement, complement);
        assert_eq!(
            build_verb_phrase_causative(recovered_host, recovered_complement).unwrap(),
            causative
        );
        assert_eq!(
            selected_verb_phrase_construction(&causative),
            "verb_phrase_causative"
        );

        let ordinary_host = build_verb_phrase_direct_object(
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Play, VerbSlot::Infinitive, 0)).unwrap(),
            )
            .unwrap(),
            this_card(),
        )
        .unwrap();
        assert_eq!(
            build_verb_phrase_causative(ordinary_host, complement.clone()),
            Err(violation(
                "verb_phrase_causative",
                "the host is a complete causative infinitive with exactly one direct-object causee",
            ))
        );

        let finite_complement = build_verb_phrase_base(
            build_verb(lexical_head(
                Vocab::Attack,
                VerbSlot::Present {
                    person: Person::Third,
                    number: Number::Singular,
                },
                0,
            ))
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            build_verb_phrase_causative(host.clone(), finite_complement),
            Err(violation(
                "verb_phrase_causative",
                "the complement is a complete bare-infinitive predicate",
            ))
        );
        assert!(build_verb_phrase_causative(complement, host.clone()).is_err());

        let pre_object_surplus = build_verb_phrase_direct_object(
            build_verb_phrase_adverb(causative_base, Vocab::Again).unwrap(),
            this_card(),
        )
        .unwrap();
        assert_eq!(
            build_verb_phrase_causative(
                pre_object_surplus,
                build_verb_phrase_base(
                    build_verb(lexical_head(Vocab::Attack, VerbSlot::Infinitive, 0)).unwrap(),
                )
                .unwrap(),
            ),
            Err(violation(
                "verb_phrase_causative",
                "the host is a complete causative infinitive with exactly one direct-object causee",
            ))
        );

        let surplus = build_verb_phrase_adverb(host, Vocab::Again).unwrap();
        assert_eq!(
            build_verb_phrase_causative(
                surplus,
                build_verb_phrase_base(
                    build_verb(lexical_head(Vocab::Attack, VerbSlot::Infinitive, 0)).unwrap(),
                )
                .unwrap(),
            ),
            Err(violation(
                "verb_phrase_causative",
                "the host is a complete causative infinitive with exactly one direct-object causee",
            ))
        );
    }

    #[test]
    fn generated_predicate_inverse_renderer_owns_causative_recursion() {
        // Mutation caught: leave the generated predicate inverse without a
        // category-total renderer, so the causative host or its declaration-
        // internal bare-infinitive complement falls back to a parallel
        // predicate renderer.
        let host = build_verb_phrase_direct_object(
            build_verb_phrase_base(
                build_verb(lexical_head(Vocab::Have, VerbSlot::Infinitive, 1)).unwrap(),
            )
            .unwrap(),
            NounPhrase::ThisCard(crate::syntax::ThisCardForm::FullName),
        )
        .expect("the causative host admits its causee");
        let complement = build_verb_phrase_base(
            build_verb(lexical_head(Vocab::Enter, VerbSlot::Infinitive, 0)).unwrap(),
        )
        .expect("the bare causative complement builds");
        let causative = build_verb_phrase_causative(host, complement)
            .expect("the checked causative shape builds");

        assert_eq!(
            crate::renderer::render_generated_predicate_verb_phrase(&causative)
                .expect("the generated inverse renders every recursive field"),
            "have this card enter"
        );
    }
}
