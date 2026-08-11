use super::Agreement;
use super::Auxiliary;
use super::AuxiliaryInflection;
use super::BareNominalAdjunct;
use super::Child;
#[cfg(test)]
use super::ContractedSubjectKey;
use super::CopulaAgreement;
use super::EnglishGrammar;
use super::Features;
use super::ParseCost;
use super::PredicateAttachmentPhase;
use super::PredicateComplementKind;
use super::PredicateForm;
use super::PredicateFrame;
use super::PredicateObjectState;
use super::Preposition;
use super::Reduced;
use super::RuleTag;
use super::VerbParticle;
use super::propagate;
use crate::features::ComplementRole;
use crate::features::Conjunction;

pub(in crate::grammar) fn reduce_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::VerbPhraseCoordinatedAdjective => reduce_predicate(tag, children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseCoordinationCopularNounPrepositional
        | RuleTag::ClauseCoordinationCopularNounPrepositionalComma
        | RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic => {
            reduce_composed_clause(tag, children)
        }
        _ => None,
    }
}

pub(in crate::grammar) fn accepts_predicate_prefix(
    tag: RuleTag,
    completed_children: usize,
    features: &Features,
) -> bool {
    if completed_children != 1 {
        return true;
    }
    if let Some(accepts) = accepts_shared_copular_coordination_prefix(tag, features) {
        return accepts;
    }
    if tag != RuleTag::VerbPhraseCoordinatedAdjective {
        return true;
    }
    let Features::VerbPhrase { frame, phase, .. } = features else {
        return false;
    };
    // The exception tail is a closed terminal phase: once reached, reject
    // every other predicate-extension tag, not only a second exception tail.
    if *phase == PredicateAttachmentPhase::ExceptionTail {
        return false;
    }
    frame.licenses_complement(PredicateComplementKind::Adjective)
}

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

fn accepts_shared_copular_coordination_prefix(tag: RuleTag, features: &Features) -> Option<bool> {
    matches!(
        tag,
        RuleTag::ClauseCoordinationCopularNounPrepositional
            | RuleTag::ClauseCoordinationCopularNounPrepositionalComma
            | RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic
    )
    .then(|| {
        matches!(
            features,
            Features::Clause {
                agreement: Some(_),
                standalone: true,
                finite: true,
                subjunctive: false,
                ..
            }
        )
    })
}

pub(in crate::grammar) fn reduction_cost(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> ParseCost {
    // The finite-first shared-predicate reading (a modal/finite clause hosting
    // a subjectless standalone-imperative continuation, asyndetic or
    // `then`/`and`-joined) is a narrow additive allowance layered on top of the
    // existing coordination gate. Dispreference it so it never outranks an
    // existing winning parse of a currently-supported card and only wins when
    // no subject/`and`/`then`-independent reading exists for the same span.
    let finite_first_shared_predicate = matches!(
        tag,
        RuleTag::ClauseCoordination
            | RuleTag::ClauseCoordinationComma
            | RuleTag::ClauseCoordinationAsyndetic
    ) && matches!(
        children.first().map(|child| child.features),
        Some(Features::Clause { finite: true, .. })
    ) && matches!(
        children.last().map(|child| child.features),
        Some(Features::SimpleClause {
            agreement: None,
            has_subject: false,
            standalone: true,
            ..
        })
    );
    ParseCost {
        precedence: 0,
        reading_dispreference: u32::from(finite_first_shared_predicate),
        ..ParseCost::default()
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "grammar reduction rule set is intentionally long"
)]
pub(super) fn reduce_predicate(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::VerbPhraseCoordinatedAdjective => {
            // Only a coordinated run whose every conjunct is an adjective
            // predicates as an adjective complement. A bare coordinated *noun*
            // pair after a verb (`Enchant creature or Vehicle`) keeps its
            // ordinary coordinated-noun-object parse rather than reducing here
            // and then failing to lower.
            let Features::CoordinatedModifier {
                all_adjectives: true,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            extend_predicate(children.first()?, PredicateAttachment::AdjectiveComplement)
        }
        _ => None,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "one exhaustive match per predicate-attachment variant is intentionally verbose"
)]
pub(super) fn extend_predicate(
    predicate: &Child<'_, EnglishGrammar<'_, '_>>,
    attachment: PredicateAttachment,
) -> Option<Reduced> {
    extend_predicate_features(predicate.features, attachment)
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
    let direct_object_attaches_under_passive =
        matches!(attachment, PredicateAttachment::DirectObject) && frame.is_recipient_passive();
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

#[allow(
    clippy::too_many_lines,
    reason = "composed-clause reduction logic is intentionally long"
)]
pub(super) fn reduce_composed_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::ClauseCoordinationCopularNounPrepositional
        | RuleTag::ClauseCoordinationCopularNounPrepositionalComma
        | RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic => {
            reduce_shared_copular_coordination(tag, children)
        }
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic => {
            if tag != RuleTag::ClauseCoordinationAsyndetic {
                let conjunction_index = if tag == RuleTag::ClauseCoordinationComma { 2 } else { 1 };
                let Features::Conjunction(
                    Conjunction::And | Conjunction::Or | Conjunction::Then | Conjunction::AndOr,
                ) = children.get(conjunction_index)?.features
                else {
                    return None;
                };
            }
            let Features::Clause {
                agreement: first_agreement,
                standalone: true,
                finite,
                host_addressee_subject: first_host_addressee_subject,
                host_modal: first_host_modal,
                subjunctive: first_subjunctive,
            } = children.first()?.features
            else {
                return None;
            };
            let last = children.last()?;
            let Features::SimpleClause {
                agreement: next_agreement,
                has_subject,
                standalone,
                subjunctive: next_subjunctive,
                ..
            } = last.features
            else {
                return None;
            };
            // A subjunctive-flagged clause never surfaces as a coordinated
            // member (only a generated `as though` attachment may consume one).
            if *first_subjunctive || *next_subjunctive {
                return None;
            }
            let host_adopts_imperative = *first_host_addressee_subject || *first_host_modal;
            // A finite, subject-bearing first clause may still host a bare
            // (subjectless, standalone) imperative continuation asyndetically —
            // "you may search ..., reveal it" — even though the first-clause
            // conditions below would otherwise disqualify it. The continuation
            // side (`next_agreement`/`standalone`) still must hold, and the
            // host must adopt the imperative: a genuine addressee-`you`
            // subject or a base-inflection modal (`may`/`can`/...) shares the
            // continuation's implicit "you", so a third-person, modal-less
            // host (in practice an opaque-noun subject like "When ..."
            // misparsed as a nominal, or its nearest-conjunct-donated
            // agreement) cannot adopt it and stays honestly unparsed.
            let continuation_is_bare_imperative =
                next_agreement.is_none() && !*has_subject && *standalone;
            if tag == RuleTag::ClauseCoordinationAsyndetic
                && !(continuation_is_bare_imperative && host_adopts_imperative)
                && (first_agreement.is_some()
                    || *finite
                    || *has_subject
                    || next_agreement.is_some()
                    || !*standalone)
            {
                return None;
            }
            if !coordination_agrees(children.first()?.features, last.features) {
                return None;
            }
            Some(Features::Clause {
                agreement: *first_agreement,
                standalone: true,
                finite: *finite,
                host_addressee_subject: *first_host_addressee_subject,
                host_modal: *first_host_modal,
                subjunctive: false,
            })
        }
        _ => None,
    }
}

fn reduce_shared_copular_coordination(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let (conjunction_index, copula_index, noun_phrase_index, preposition_index) = match tag {
        RuleTag::ClauseCoordinationCopularNounPrepositional => (Some(1), 2, 3, 4),
        RuleTag::ClauseCoordinationCopularNounPrepositionalComma => (Some(2), 3, 4, 5),
        RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic => (None, 2, 3, 4),
        _ => return None,
    };
    let Features::Clause {
        agreement: Some(host_agreement),
        standalone: true,
        finite: true,
        subjunctive: false,
        ..
    } = children.first()?.features
    else {
        return None;
    };
    if let Some(index) = conjunction_index
        && !matches!(
            children.get(index)?.features,
            Features::Conjunction(Conjunction::And)
        )
    {
        return None;
    }
    let Features::Copula(CopulaAgreement::Indicative(copula_agreement)) =
        children.get(copula_index)?.features
    else {
        return None;
    };
    if host_agreement != copula_agreement
        || !matches!(
            children.get(noun_phrase_index)?.features,
            Features::NounPhrase { .. }
        )
        || !matches!(
            children.get(preposition_index)?.features,
            Features::PrepositionalPhrase {
                preposition: Preposition::In,
                ..
            }
        )
    {
        return None;
    }
    Some(propagate(children.first()?))
}

fn coordination_agrees(first_features: &Features, next_features: &Features) -> bool {
    let Features::Clause {
        agreement: first,
        finite: first_finite,
        host_addressee_subject,
        host_modal: first_is_modal,
        ..
    } = first_features
    else {
        return false;
    };
    let Features::SimpleClause {
        agreement: next,
        has_subject: next_has_subject,
        standalone: next_standalone,
        host_modal: next_is_modal,
        ..
    } = next_features
    else {
        return false;
    };
    let first_adopts_imperative = *host_addressee_subject || *first_is_modal;
    let shared_finite_predicate = matches!(
        (first, next, next_standalone),
        (Some(left), Some(right), false) if left == right
    );
    let imperative_sequence =
        !*first_finite && matches!((first, next, next_standalone), (None, None, true));
    let subjectless_modal_predicate =
        *next_is_modal && matches!((first, next, next_standalone), (Some(_), None, false));
    // A first clause whose host adopts the imperative (a genuine
    // addressee-`you` subject or a base-inflection modal, often "you may
    // search ..." or "Its controller may search ...") followed by a
    // subjectless standalone imperative continuation ("..., then
    // shuffle") — the `then`-tail counterpart to the asyndetic allowance in
    // `reduce_composed_clause`.
    let hosted_imperative = first_adopts_imperative && next.is_none() && *next_standalone;

    *next_has_subject
        || shared_finite_predicate
        || imperative_sequence
        || subjectless_modal_predicate
        || hosted_imperative
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
            Some(PredicateForm::Finite(Some(Agreement { person, number })))
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
