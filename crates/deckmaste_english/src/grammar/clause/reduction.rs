use super::Auxiliary;
use super::AuxiliaryInflection;
use super::BareNominalAdjunct;
#[cfg(test)]
use super::ContractedSubjectKey;
use super::Features;
use super::PersonNumber;
use super::PredicateAttachmentPhase;
use super::PredicateComplementKind;
use super::PredicateForm;
use super::PredicateFrame;
use super::PredicateObjectState;
use super::Preposition;
use super::Reduced;
use super::VerbParticle;
use crate::features::ComplementRole;

pub(crate) fn reduce_generated_recipient_passive_nominal_adjunct_features(
    predicate: &Features,
    noun_phrase: &Features,
) -> Option<Features> {
    let Features::NounPhrase {
        adjunct: Some(adjunct),
        ..
    } = noun_phrase
    else {
        return None;
    };
    extend_predicate_features(predicate, PredicateAttachment::NominalAdjunct(*adjunct))
}

#[allow(
    clippy::too_many_lines,
    reason = "one exhaustive match per predicate-attachment variant is intentionally verbose"
)]
pub(crate) fn extend_predicate_features(
    predicate: &Features,
    attachment: PredicateAttachment,
) -> Option<Reduced> {
    let Features::VerbPhrase {
        form,
        passive,
        dependent_count,
        object,
        indirect_object,
        selected_preposition,
        phase,
        frame,
        head_is_copular,
        object_gap_requires_rules_object,
        subjunctive,
        ..
    } = predicate
    else {
        return None;
    };
    // The feature transition is also called by generated checked builders,
    // which do not pass through the chart's dot-1 prediction gate. Keep the
    // terminal and one-shot tail boundaries intrinsic to the transition so
    // direct construction cannot manufacture a value the parser would never
    // admit.
    if *phase == PredicateAttachmentPhase::ExceptionTail
        || (matches!(
            attachment,
            PredicateAttachment::Particle(_) | PredicateAttachment::CoinResult(_)
        ) && *phase != PredicateAttachmentPhase::Object)
    {
        return None;
    }
    let prepositional_role = match attachment {
        PredicateAttachment::Prepositional(preposition) => {
            Some(frame.prepositional_role(preposition)?)
        }
        _ => None,
    };
    let licensed = match attachment {
        PredicateAttachment::Adjunct | PredicateAttachment::Prepositional(_) => true,
        PredicateAttachment::Exception => predicate_arguments_complete(
            *frame,
            *passive,
            *object,
            *indirect_object,
            *selected_preposition,
        ),
        PredicateAttachment::DirectObject | PredicateAttachment::PronominalDirectObject => {
            frame.direct_object().accepts()
        }
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
        // Licensed exactly by the pending `Come` frame, never by a general
        // particle table; the `accepts_predicate_prefix` dot-1 gate already
        // enforces this before the scanner is even predicted, so this
        // duplicates that categorical gate rather than trusting it alone.
        PredicateAttachment::CoinResult(_) => frame.requires_coin_result(),
        PredicateAttachment::AbilityComplement | PredicateAttachment::QuotedObject => {
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
    // A recipient-passive frame retains its theme post-verbally, so a direct
    // object may attach under it in the passive; every other frame keeps the
    // blanket ban (an ordinary passive's promoted subject IS the theme, so a
    // further direct object is never a coherent reading).
    let direct_object_attaches_under_passive = matches!(
        attachment,
        PredicateAttachment::DirectObject | PredicateAttachment::PronominalDirectObject
    ) && frame.is_recipient_passive();
    if *passive
        && !direct_object_attaches_under_passive
        && !matches!(
            attachment,
            PredicateAttachment::Adjunct
                | PredicateAttachment::NominalAdjunct(_)
                | PredicateAttachment::AdjectiveComplement
                | PredicateAttachment::Prepositional(_)
                | PredicateAttachment::InfinitiveComplement
                | PredicateAttachment::Particle(_)
                | PredicateAttachment::Exception
        )
    {
        return None;
    }
    // `Come` forbids a direct object, so it never passivizes under this
    // frame; a coin-result tail attaching under `*passive` is unreachable in
    // practice, and the blanket ban above already rejects it since
    // `CoinResult` is not in the passive-exempt list.
    // The exception tail requires a passive host; it is not itself a
    // selected complement, so it is licensed only once the frame's ordinary
    // required arguments are already satisfied (checked above via
    // `predicate_arguments_complete`).
    if matches!(attachment, PredicateAttachment::Exception) && !*passive {
        return None;
    }
    let next_phase = match attachment {
        PredicateAttachment::DirectObject
        | PredicateAttachment::PronominalDirectObject
        | PredicateAttachment::IndirectObject
        | PredicateAttachment::AbilityComplement
        | PredicateAttachment::QuotedObject
        | PredicateAttachment::ScalarComplement
        | PredicateAttachment::StatisticComplement
        | PredicateAttachment::ScalarOrAbilityArgument => {
            if *phase != PredicateAttachmentPhase::Object {
                return None;
            }
            PredicateAttachmentPhase::Object
        }
        PredicateAttachment::Prepositional(_) => PredicateAttachmentPhase::PrepositionalTail,
        PredicateAttachment::Exception => PredicateAttachmentPhase::ExceptionTail,
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
        | PredicateAttachment::Particle(_)
        | PredicateAttachment::CoinResult(_) => PredicateAttachmentPhase::Tail,
    };
    let object = match (attachment, *object) {
        (
            PredicateAttachment::Adjunct
            | PredicateAttachment::IndirectObject
            | PredicateAttachment::NominalAdjunct(_)
            | PredicateAttachment::AdjectiveComplement
            | PredicateAttachment::Prepositional(_)
            | PredicateAttachment::InfinitiveComplement
            | PredicateAttachment::Particle(_)
            | PredicateAttachment::CoinResult(_)
            | PredicateAttachment::Exception,
            object,
        ) => object,
        (
            PredicateAttachment::DirectObject
            | PredicateAttachment::QuotedObject
            | PredicateAttachment::ScalarComplement
            | PredicateAttachment::StatisticComplement
            | PredicateAttachment::ScalarOrAbilityArgument,
            PredicateObjectState::None,
        ) => PredicateObjectState::Direct,
        (PredicateAttachment::PronominalDirectObject, PredicateObjectState::None) => {
            PredicateObjectState::PronominalDirect
        }
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
        Some(ComplementRole::SelectedComplement) if *selected_preposition => return None,
        Some(ComplementRole::SelectedComplement) => true,
        Some(ComplementRole::Adjunct) | None => *selected_preposition,
    };
    // A `CoinResult` attachment is what discharges the pending `Come` frame:
    // the completed predicate carries a frame that no longer rejects
    // completion checks. Every other attachment leaves the frame unchanged.
    let frame = if matches!(attachment, PredicateAttachment::CoinResult(_)) {
        frame.discharge_coin_result()
    } else {
        *frame
    };
    Some(Features::VerbPhrase {
        form: *form,
        passive: *passive,
        dependent_count: dependent_count.checked_add(1)?,
        object,
        indirect_object,
        selected_preposition,
        phase: next_phase,
        frame,
        bare: false,
        head_is_copular: *head_is_copular,
        object_gap_requires_rules_object: *object_gap_requires_rules_object,
        subjunctive: *subjunctive,
    })
}

#[allow(
    dead_code,
    reason = "attachment validation needs the coin-result category, not its value"
)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum PredicateAttachment {
    Adjunct,
    DirectObject,
    PronominalDirectObject,
    IndirectObject,
    NominalAdjunct(BareNominalAdjunct),
    AdjectiveComplement,
    Prepositional(Preposition),
    InfinitiveComplement,
    Particle(VerbParticle),
    /// The closed `come up heads`/`come up tails` result tail. Licensed
    /// only by the narrow pending `Come` frame
    /// (`PredicateFrame::requires_coin_result`); the reduction discharges
    /// that frame so the completed predicate never re-licenses it.
    CoinResult(crate::syntax::CoinSide),
    /// The closed `except by <PP>` exception tail. Licensed without
    /// consulting the verb frame's selected-preposition table — the `By` PP
    /// is the exceptional restriction's agent-like complement, not a
    /// selected complement of the passive verb.
    Exception,
    AbilityComplement,
    /// A quoted (or coordinated quoted) ability object. Licensed by the same
    /// frames that admit a keyword-ability complement (the grant verbs), but it
    /// is a complete direct object — unlike a bare keyword ability it never
    /// takes a following scalar argument — so it settles the object slot to
    /// `Direct`.
    QuotedObject,
    ScalarComplement,
    StatisticComplement,
    ScalarOrAbilityArgument,
}

/// Folds an auxiliary attaching from outside a verb phrase into that phrase's
/// passive determination, and applies the retained-object rule. Shared by
/// Both the declaration-owned auxiliary construction (where the auxiliary sits
/// inside the phrase) and the declaration-owned contracted-subject clause use
/// this helper, so the two paths cannot drift.
///
/// Returns `None` when the combination is ill-formed — a passive may keep a
/// direct object only as a recipient passive's retained theme, and never keeps
/// an explicit indirect object.
pub(crate) fn fold_auxiliary_passive(
    auxiliary: impl Into<super::AuxiliaryFeatures>,
    child_form: PredicateForm,
    child_passive: bool,
    object: PredicateObjectState,
    indirect_object: bool,
    frame: PredicateFrame,
) -> Option<bool> {
    let auxiliary = auxiliary.into();
    let passive = child_passive
        || (auxiliary.auxiliary == Auxiliary::Be && child_form == PredicateForm::PastParticiple);
    // A direct object survives passivization only under a recipient-passive
    // frame (the retained theme); every other frame keeps the blanket ban. An
    // indirect-object DEPENDENT (a literal NP, as opposed to the frame-level
    // promotion) is never valid under any passive: the recipient-passive's
    // indirect-object requirement is satisfied by the promotion itself, never
    // by a further explicit indirect object.
    if passive && (indirect_object || (object.has_direct_object() && !frame.is_recipient_passive()))
    {
        return None;
    }
    Some(passive)
}

pub(crate) fn predicate_arguments_complete(
    frame: PredicateFrame,
    passive: bool,
    object: PredicateObjectState,
    indirect_object: bool,
    selected_preposition: bool,
) -> bool {
    // A recipient-passive frame exists only to license the passive; it must
    // never be selectable in the active, or `deals 2 damage to X` would gain a
    // spurious double-object reading.
    if frame.is_recipient_passive() && !passive {
        return false;
    }
    // A frame pending a required `CoinResult` tail is never complete on its
    // own; only the generated `verb_phrase_coin_result` construction
    // discharges the coin-result frame.
    if frame.requires_coin_result() {
        return false;
    }
    let recipient_passive = passive && frame.is_recipient_passive();
    frame
        .direct_object()
        // Ordinary passivization promotes the direct object; recipient
        // passivization does not — the theme must still be present, retained.
        .is_satisfied_by((passive && !recipient_passive) || object.has_direct_object())
        && frame
            .indirect_object()
            // The recipient is the promoted subject, so the indirect-object
            // requirement is satisfied by the promotion itself.
            .is_satisfied_by(indirect_object || recipient_passive)
        && frame
            .selected_preposition()
            .is_satisfied_by(selected_preposition)
}

pub(crate) fn predicate_object_gap_complete(
    frame: PredicateFrame,
    indirect_object: bool,
    selected_preposition: bool,
) -> bool {
    if frame.is_recipient_passive() {
        return false;
    }
    if frame.requires_coin_result() {
        return false;
    }
    frame.direct_object().accepts()
        && frame.indirect_object().is_satisfied_by(indirect_object)
        && frame
            .selected_preposition()
            .is_satisfied_by(selected_preposition)
}
pub(crate) fn auxiliary_form(
    auxiliary: impl Into<super::AuxiliaryFeatures>,
    child: PredicateForm,
) -> Option<PredicateForm> {
    let auxiliary = auxiliary.into();
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
            Some(PredicateForm::Finite(Some(PersonNumber { person, number })))
        }
        AuxiliaryInflection::PastSubjunctive => Some(PredicateForm::Finite(None)),
        AuxiliaryInflection::PresentParticiple => Some(PredicateForm::PresentParticiple),
        AuxiliaryInflection::PastParticiple => Some(PredicateForm::PastParticiple),
    }
}

#[cfg(test)]
mod feature_identity_tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;

    use super::*;
    use crate::word::AuxiliaryInstance;

    fn hash(features: &Features) -> u64 {
        let mut hasher = DefaultHasher::new();
        features.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn auxiliary_surface_witness_does_not_change_chart_features() {
        let full = AuxiliaryInstance {
            auxiliary: Auxiliary::Do,
            inflection: AuxiliaryInflection::Base,
            contracted_negation: crate::features::Contraction::Full,
        };
        let contracted = AuxiliaryInstance {
            contracted_negation: crate::features::Contraction::Contracted,
            ..full
        };

        let full_auxiliary = Features::auxiliary(full);
        let contracted_auxiliary = Features::auxiliary(contracted);
        assert_eq!(full_auxiliary, contracted_auxiliary);
        assert_eq!(hash(&full_auxiliary), hash(&contracted_auxiliary));

        let subject = ContractedSubjectKey::Pronoun(crate::word::Pronoun::You);
        let agreement = subject.agreement();
        let full_subject = Features::SubjectAuxiliary {
            subject,
            agreement,
            auxiliary: full.into(),
        };
        let contracted_subject = Features::SubjectAuxiliary {
            subject,
            agreement,
            auxiliary: contracted.into(),
        };
        assert_eq!(full_subject, contracted_subject);
        assert_eq!(hash(&full_subject), hash(&contracted_subject));
    }
}
