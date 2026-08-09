use super::Agreement;
use super::Auxiliary;
use super::AuxiliaryInflection;
use super::BareNominalAdjunct;
use super::Child;
use super::ContractedSubjectKey;
use super::CopulaAgreement;
use super::Demonstrative;
use super::EnglishGrammar;
use super::Features;
use super::GapState;
use super::Number;
use super::ParseCost;
use super::Person;
use super::PredicateAttachmentPhase;
use super::PredicateComplementKind;
use super::PredicateForm;
use super::PredicateFrame;
use super::PredicateObjectState;
use super::Preposition;
use super::PronounCase;
use super::Reduced;
use super::RuleTag;
use super::VerbParticle;
use super::lowering::is_modal;
use super::propagate;
use crate::features::ComplementRole;
use crate::features::Conjunction;

pub(in crate::grammar) fn reduce_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::VerbPhraseCoordinatedAdjective
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
            reduce_simple_clause(tag, children)
        }
        RuleTag::ClauseVariableValueConstraint => reduce_variable_value_constraint(children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseCoordinationCopularNounPrepositional
        | RuleTag::ClauseCoordinationCopularNounPrepositionalComma
        | RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic
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
        | RuleTag::ClauseRestrictionRun => reduce_composed_clause(tag, children),
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
    // Gate exactly on `Features::Subordinator(While)` before predicting
    // `GerundClause`: no other subordinator gains this fronted-gerund shape,
    // so this never cascades into a generic fronted-gerund production.
    if tag == RuleTag::ClauseSubordinateGerundBefore {
        return matches!(
            features,
            Features::Subordinator(crate::syntax::Subordinator::While)
        );
    }
    // The finite verbal quantifier float's host gate: only a plural subject,
    // or a second-person subject (the grammar's `you` feature is
    // second-person singular even for a plural discourse referent, so
    // `person == Second` is required for `You each ...`), in non-object case
    // may scan the dedicated `each` lexeme here. `I each` must not be
    // admitted, so this is not loosened to every non-third-singular subject.
    if tag == RuleTag::SimpleClauseSubjectDistributiveEach {
        return matches!(
            features,
            Features::NounPhrase {
                agreement: Some(a),
                pronoun_case,
                ..
            } if *pronoun_case != Some(PronounCase::Object)
                && (a.number == Number::Plural || a.person == Person::Second)
        );
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
    let precedence = u32::from(tag == RuleTag::ClauseSubordinateAfter);
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
        precedence,
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
/// inside the phrase) and [`RuleTag::SimpleClauseContractedSubject`] (where it
/// is contracted onto the subject) use this helper, so the two paths cannot
/// drift.
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

pub(in crate::grammar) fn predicate_arguments_complete(
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

pub(super) fn predicate_object_gap_complete(
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
    reason = "simple-clause reduction logic is intentionally long"
)]
pub(super) fn reduce_simple_clause(
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
                subjunctive,
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
            let host_addressee_subject = *pronoun_case == Some(PronounCase::Subject)
                && subject_agreement.person == Person::Second;
            let host_modal = predicate_agreement.is_none();
            Some(simple_clause_reduction(
                Some(*subject_agreement),
                true,
                true,
                object.has_direct_object(),
                host_addressee_subject,
                host_modal,
                *subjunctive,
            ))
        }
        RuleTag::SimpleClauseSubjectDistributiveEach => {
            // Defensive recheck of the dot-1 host gate in
            // `accepts_predicate_prefix`: plural or second-person subject,
            // non-object case.
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
            if subject_agreement.number != Number::Plural
                && subject_agreement.person != Person::Second
            {
                return None;
            }
            // The predicate must be a genuinely finite, agreeing verb phrase
            // (excludes third-singular `gets` and the agreement-neutral
            // modal path — the predicate fact this floated `each` needs is
            // only available here, at reduce).
            let Features::VerbPhrase {
                form: PredicateForm::Finite(Some(predicate_agreement)),
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                subjunctive,
                ..
            } = children.get(2)?.features
            else {
                return None;
            };
            if *predicate_agreement != *subject_agreement {
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
            let host_addressee_subject = *pronoun_case == Some(PronounCase::Subject)
                && subject_agreement.person == Person::Second;
            Some(simple_clause_reduction(
                Some(*subject_agreement),
                true,
                true,
                object.has_direct_object(),
                host_addressee_subject,
                false,
                *subjunctive,
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
                passive: child_passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                subjunctive,
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
            let passive = fold_auxiliary_passive(
                *auxiliary,
                *child_form,
                *child_passive,
                *object,
                *indirect_object,
                *frame,
            )?;
            if !predicate_arguments_complete(
                *frame,
                passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            let host_addressee_subject = subject_agreement.person == Person::Second;
            // `auxiliary_form` only yields `Finite(Some(_))` here (the
            // `Finite(None)` base-modal path never satisfies this pattern),
            // so a contracted-subject host never carries `host_modal`.
            let host_modal = false;
            Some(simple_clause_reduction(
                Some(*subject_agreement),
                true,
                true,
                object.has_direct_object(),
                host_addressee_subject,
                host_modal,
                *subjunctive,
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
                subjunctive,
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
                    false,
                    false,
                    *subjunctive,
                )),
                PredicateForm::Finite(agreement) => Some(simple_clause_reduction(
                    *agreement,
                    false,
                    false,
                    object.has_direct_object(),
                    false,
                    agreement.is_none(),
                    *subjunctive,
                )),
                PredicateForm::Infinitive => Some(simple_clause_reduction(
                    None,
                    false,
                    false,
                    object.has_direct_object(),
                    false,
                    false,
                    *subjunctive,
                )),
                PredicateForm::PresentParticiple | PredicateForm::PastParticiple => None,
            }
        }
        RuleTag::ClauseSimple => {
            let Features::SimpleClause {
                agreement,
                has_subject,
                standalone,
                host_addressee_subject,
                host_modal,
                subjunctive,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Clause {
                agreement: *agreement,
                standalone: *standalone,
                finite: *has_subject,
                host_addressee_subject: *host_addressee_subject,
                host_modal: *host_modal,
                subjunctive: *subjunctive,
            })
        }
        RuleTag::ClauseElliptical => Some(Features::Clause {
            agreement: None,
            standalone: false,
            finite: false,
            host_addressee_subject: false,
            host_modal: false,
            subjunctive: false,
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
                host_addressee_subject: false,
                host_modal: false,
                subjunctive: false,
            })
        }
        RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderNegated
        | RuleTag::CopularRemainderDistributiveEach => Some(Features::None),
        RuleTag::CopularRemainderCoordinatedAdjective => {
            // Only an all-adjective coordinated run predicates as a copular
            // adjective complement; a coordinated run holding a noun reading
            // (supertypes such as `snow` scan as both) is rejected here so the
            // adjective-reading conjuncts are the ones lowering converts.
            let Features::CoordinatedModifier {
                all_adjectives: true,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::None)
        }
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
                object_gap_requires_rules_object,
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
                gap: GapState::Object,
                antecedent_agreement: None,
                object_gap_requires_rules_object: *object_gap_requires_rules_object,
                bare_copular_tail: false,
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
                object_gap_requires_rules_object,
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
                gap: GapState::Object,
                antecedent_agreement: None,
                object_gap_requires_rules_object: *object_gap_requires_rules_object,
                bare_copular_tail: false,
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
                gap: GapState::Subject,
                antecedent_agreement: Some(predicate_agreement),
                object_gap_requires_rules_object: false,
                bare_copular_tail: false,
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
                bare,
                head_is_copular,
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
                gap: GapState::Subject,
                antecedent_agreement: *antecedent_agreement,
                object_gap_requires_rules_object: false,
                bare_copular_tail: *bare && *head_is_copular,
            })
        }
        RuleTag::RelativeSubjectDistributiveEach => {
            // `RelativeMarker` carries `Features::None`, so no dot-1 gate is
            // possible; require a complete, plural, finite verb phrase here
            // and publish that agreement so the existing nominal-relative
            // attachment gate checks the plural antecedent.
            let Features::VerbPhrase {
                form: PredicateForm::Finite(Some(agreement)),
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                bare,
                head_is_copular,
                ..
            } = children.get(2)?.features
            else {
                return None;
            };
            if agreement.number != Number::Plural {
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
            Some(Features::RelativeClause {
                gap: GapState::Subject,
                antecedent_agreement: Some(*agreement),
                object_gap_requires_rules_object: false,
                bare_copular_tail: *bare && *head_is_copular,
            })
        }
        RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::RelativeContractedCopularCoordinatedAdjective => {
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
            // The coordinated variant only predicates when every conjunct is an
            // adjective; a noun-reading coordinated run is rejected so the
            // adjective-reading conjuncts are the ones lowering keeps.
            if tag == RuleTag::RelativeContractedCopularCoordinatedAdjective
                && !matches!(
                    children.get(1)?.features,
                    Features::CoordinatedModifier {
                        all_adjectives: true,
                        ..
                    }
                )
            {
                return None;
            }
            Some(Features::RelativeClause {
                gap: GapState::Subject,
                antecedent_agreement: Some(*agreement),
                object_gap_requires_rules_object: false,
                bare_copular_tail: false,
            })
        }
        _ => None,
    }
}

pub(super) fn reduce_copular_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let contracted = tag == RuleTag::ClauseContractedCopular;
    let (agreement, subjunctive) = if contracted {
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
        (*agreement, false)
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
        let Features::Copula(copula) = children.get(1)?.features else {
            return None;
        };
        match copula {
            CopulaAgreement::Indicative(copula_agreement) => {
                if *subject_agreement != *copula_agreement {
                    return None;
                }
                (*subject_agreement, false)
            }
            // Recognition-level licensing: no agreement constraint, but the
            // clause is marked subjunctive, so every consumer other than the
            // `as though` gate (clause.rs:2056) rejects it.
            CopulaAgreement::PastSubjunctive => (*subject_agreement, true),
        }
    };
    Some(Features::Clause {
        agreement: Some(agreement),
        standalone: true,
        finite: true,
        host_addressee_subject: false,
        host_modal: false,
        subjunctive,
    })
}

pub(super) fn reduce_variable_value_constraint(
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let Features::Quantity(_) = children.first()?.features else {
        return None;
    };
    let Features::Auxiliary(modal) = children.get(1)?.features else {
        return None;
    };
    if !is_modal(modal.auxiliary) {
        return None;
    }
    let Features::Auxiliary(copula) = children.get(2)?.features else {
        return None;
    };
    if copula.auxiliary != Auxiliary::Be || copula.inflection != AuxiliaryInflection::Base {
        return None;
    }
    let Features::Number { .. } = children.get(3)?.features else {
        return None;
    };
    Some(Features::Clause {
        agreement: None,
        standalone: true,
        finite: true,
        host_addressee_subject: false,
        host_modal: false,
        subjunctive: false,
    })
}

#[allow(
    clippy::fn_params_excessive_bools,
    reason = "each bool is an independently-computed SimpleClause feature bit; \
              grouping into enums would obscure the 1:1 field mapping"
)]
pub(super) fn simple_clause_reduction(
    agreement: Option<Agreement>,
    has_subject: bool,
    standalone: bool,
    has_direct_object: bool,
    host_addressee_subject: bool,
    host_modal: bool,
    subjunctive: bool,
) -> Reduced {
    Features::SimpleClause {
        agreement,
        has_subject,
        standalone,
        has_direct_object,
        host_addressee_subject,
        host_modal,
        subjunctive,
    }
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
            // member (only `ClauseSubordinateAfter`/`…Comma` under
            // `as though` may consume one).
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
        RuleTag::ClauseSubordinateBefore => {
            let Features::Subordinator(subordinator) = children.first()?.features else {
                return None;
            };
            conditional_reduction(*subordinator, children.get(1)?, children.get(3)?)
        }
        RuleTag::ClauseSubordinateGerundBefore => {
            // The dot-1 gate already required `Features::Subordinator(While)`
            // before `GerundClause` was predicted; re-check here rather than
            // trust it alone, and require a complete gerund and an
            // independent, non-subjunctive matrix.
            let Features::Subordinator(crate::syntax::Subordinator::While) =
                children.first()?.features
            else {
                return None;
            };
            if !matches!(children.get(1)?.features, Features::GerundClause) {
                return None;
            }
            fronted_attachment_reduction(children.get(3)?)
        }
        RuleTag::ClauseAdverbBefore => fronted_attachment_reduction(children.get(1)?),
        RuleTag::ClauseSentenceAdverbialBefore => fronted_attachment_reduction(children.get(2)?),
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
                host_addressee_subject,
                host_modal,
                subjunctive,
            } = consequence.features
            else {
                return None;
            };
            if *subjunctive {
                return None;
            }
            Some(Features::Clause {
                agreement: *agreement,
                standalone: true,
                finite: *finite,
                host_addressee_subject: *host_addressee_subject,
                host_modal: *host_modal,
                subjunctive: false,
            })
        }
        RuleTag::ClauseSubordinateAfter => {
            let Features::Subordinator(subordinator) = children.get(1)?.features else {
                return None;
            };
            conditional_reduction(*subordinator, children.get(2)?, children.first()?)
        }
        RuleTag::ClauseSubordinateAfterComma => {
            let Features::Subordinator(subordinator) = children.get(2)?.features else {
                return None;
            };
            conditional_reduction(*subordinator, children.get(3)?, children.first()?)
        }
        RuleTag::ClauseSubordinateAfterInfinitive => {
            let Features::Clause {
                agreement,
                standalone: true,
                finite,
                host_addressee_subject,
                host_modal,
                subjunctive,
            } = children.first()?.features
            else {
                return None;
            };
            if *subjunctive {
                return None;
            }
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
                host_addressee_subject: *host_addressee_subject,
                host_modal: *host_modal,
                subjunctive: false,
            })
        }
        RuleTag::ExceptionRiderSingle => {
            let Features::Clause {
                standalone: true, ..
            } = children.get(1)?.features
            else {
                return None;
            };
            Some(Features::ExceptionRider)
        }
        RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford => {
            if !matches!(children.first()?.features, Features::ExceptionRider) {
                return None;
            }
            let Features::Clause {
                standalone: true, ..
            } = children.last()?.features
            else {
                return None;
            };
            if tag != RuleTag::ExceptionRiderComma {
                let conjunction_index = if tag == RuleTag::ExceptionRiderOxford { 2 } else { 1 };
                let Features::Conjunction(Conjunction::And | Conjunction::Or | Conjunction::Then) =
                    children.get(conjunction_index)?.features
                else {
                    return None;
                };
            }
            Some(Features::ExceptionRider)
        }
        RuleTag::ClauseExcepted => {
            let Features::Clause {
                agreement,
                standalone: true,
                finite,
                host_addressee_subject,
                host_modal,
                subjunctive,
            } = children.first()?.features
            else {
                return None;
            };
            if *subjunctive {
                return None;
            }
            if !matches!(children.get(2)?.features, Features::ExceptionRider) {
                return None;
            }
            Some(Features::Clause {
                agreement: *agreement,
                standalone: true,
                finite: *finite,
                host_addressee_subject: *host_addressee_subject,
                host_modal: *host_modal,
                subjunctive: false,
            })
        }
        RuleTag::ClauseRestrictionMember => {
            if children.len() == 3 {
                // `[only, Subordinator, Clause]` — the `if`-clause member.
                if let Features::Clause {
                    standalone: true, ..
                } = children.get(2)?.features
                {
                    return Some(Features::RestrictionMember);
                }
            }
            // `[only, Prepositional]`, `[only, Adverb]`, `[only, Adverb,
            // NounPhrase]` — no cross-child feature agreement needed; the
            // grammatical shape alone licenses these.
            Some(Features::RestrictionMember)
        }
        RuleTag::ClauseRestrictionRun => {
            match children.len() {
                2 => {
                    // The `[Clause, RestrictionRun]` attachment.
                    let Features::Clause {
                        agreement,
                        standalone: true,
                        finite,
                        host_addressee_subject,
                        host_modal,
                        subjunctive,
                    } = children.first()?.features
                    else {
                        return None;
                    };
                    if *subjunctive {
                        return None;
                    }
                    if !matches!(children.get(1)?.features, Features::RestrictionRun) {
                        return None;
                    }
                    Some(Features::Clause {
                        agreement: *agreement,
                        standalone: true,
                        finite: *finite,
                        host_addressee_subject: *host_addressee_subject,
                        host_modal: *host_modal,
                        subjunctive: false,
                    })
                }
                3 => {
                    // `[Member, Conjunction, Member]` (base pair) or
                    // `[Run, Comma, Member]` (asyndetic growth).
                    let first = children.first()?.features;
                    let first_ok = matches!(
                        first,
                        Features::RestrictionMember | Features::RestrictionRun
                    );
                    let last_ok = matches!(children.last()?.features, Features::RestrictionMember);
                    let conjunction_ok = !matches!(
                        children.get(1)?.features,
                        Features::Conjunction(
                            Conjunction::Or
                                | Conjunction::Then
                                | Conjunction::Plus
                                | Conjunction::AndOr
                        )
                    );
                    if first_ok && last_ok && conjunction_ok {
                        Some(Features::RestrictionRun)
                    } else {
                        None
                    }
                }
                4 => {
                    // `[Run, Comma, Conjunction, Member]` (Oxford growth).
                    if !matches!(children.first()?.features, Features::RestrictionRun) {
                        return None;
                    }
                    if !matches!(children.last()?.features, Features::RestrictionMember) {
                        return None;
                    }
                    if !matches!(
                        children.get(2)?.features,
                        Features::Conjunction(Conjunction::And)
                    ) {
                        return None;
                    }
                    Some(Features::RestrictionRun)
                }
                _ => None,
            }
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

pub(super) fn fronted_attachment_reduction(
    matrix: &Child<'_, EnglishGrammar<'_, '_>>,
) -> Option<Reduced> {
    let Features::Clause {
        agreement,
        standalone: true,
        finite,
        host_addressee_subject,
        host_modal,
        subjunctive,
    } = matrix.features
    else {
        return None;
    };
    if *subjunctive {
        return None;
    }
    Some(Features::Clause {
        agreement: *agreement,
        standalone: true,
        finite: *finite,
        host_addressee_subject: *host_addressee_subject,
        host_modal: *host_modal,
        subjunctive: false,
    })
}

pub(super) fn conditional_reduction(
    subordinator: crate::syntax::Subordinator,
    condition: &Child<'_, EnglishGrammar<'_, '_>>,
    consequence: &Child<'_, EnglishGrammar<'_, '_>>,
) -> Option<Reduced> {
    let Features::Clause {
        standalone: true,
        finite: true,
        subjunctive: condition_subjunctive,
        ..
    } = condition.features
    else {
        return None;
    };
    // Licensing gate: a past-subjunctive condition clause (`it were ...`) is
    // only ever well-formed under `as though` — every other subordinator
    // (`until`, `as long as`, `where`, ...) must reject it outright. See
    // `Features::Subordinator`/`AuxiliaryInflection::PastSubjunctive`.
    if *condition_subjunctive && !matches!(subordinator, crate::syntax::Subordinator::AsThough) {
        return None;
    }
    let Features::Clause {
        agreement,
        standalone: true,
        finite,
        host_addressee_subject,
        host_modal,
        subjunctive: consequence_subjunctive,
    } = consequence.features
    else {
        return None;
    };
    // The matrix clause itself is never subjunctive in this construction.
    if *consequence_subjunctive {
        return None;
    }
    Some(Features::Clause {
        agreement: *agreement,
        standalone: true,
        finite: *finite,
        host_addressee_subject: *host_addressee_subject,
        host_modal: *host_modal,
        // The composed clause is not itself subjunctive: the flag is
        // consumed by this gate, never propagated further.
        subjunctive: false,
    })
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

    fn standalone_clause() -> Features {
        Features::Clause {
            agreement: None,
            standalone: true,
            finite: true,
            host_addressee_subject: false,
            host_modal: false,
            subjunctive: false,
        }
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

    #[test]
    fn exception_punctuation_does_not_change_chart_features() {
        let ignored = Features::None;
        let clause = standalone_clause();
        let single = reduce_composed_clause(
            RuleTag::ExceptionRiderSingle,
            &[Child { features: &ignored }, Child { features: &clause }],
        )
        .expect("single exception rider must reduce");
        let comma = reduce_composed_clause(
            RuleTag::ExceptionRiderComma,
            &[
                Child { features: &single },
                Child { features: &ignored },
                Child { features: &clause },
            ],
        )
        .expect("comma exception rider must reduce");

        assert_eq!(single, comma);
        assert_eq!(hash(&single), hash(&comma));
    }
}
