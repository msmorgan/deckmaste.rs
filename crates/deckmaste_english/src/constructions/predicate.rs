//! Compiler-derived English predicate declarations.

#![allow(
    dead_code,
    clippy::unnecessary_wraps,
    reason = "the inactive declaration is exercised through generated adapters in tests"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

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
use crate::syntax::AdjectivePhrase;
use crate::syntax::NounPhrase;
use crate::syntax::Phrase;
use crate::syntax::PrepositionalPhrase;
use crate::word::AuxiliaryInstance;
use crate::word::Vocabulary;

type Verb = VerbAnalysis;

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
    value.declaration_proform_part().is_none()
        && value.declaration_auxiliary_parts().is_some()
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

fn from_direct_object_lens_parts(
    mut before: Vec<VerbDependent>,
    dependent: Option<NounPhrase>,
    after: Vec<VerbDependent>,
    shell: VerbPhrase,
) -> VerbPhrase {
    if let Some(dependent) = dependent {
        let source = VerbPhrase::declaration_from_dependent_projection(
            shell.clone(),
            before.iter().chain(&after).cloned().collect(),
        );
        before.push(
            match crate::grammar::lowered_nominal_adjunct_kind(&source, &dependent) {
                Some(crate::word::BareNominalAdjunct::Temporal) => {
                    VerbDependent::Temporal(dependent)
                }
                Some(crate::word::BareNominalAdjunct::Manner) => VerbDependent::Manner(dependent),
                None => VerbDependent::DirectObject(dependent),
            },
        );
    }
    before.extend(after);
    VerbPhrase::declaration_from_dependent_projection(shell, before)
}

fn into_direct_object_lens_parts(
    value: VerbPhrase,
) -> (
    Vec<VerbDependent>,
    Option<NounPhrase>,
    Vec<VerbDependent>,
    VerbPhrase,
) {
    let (mut dependents, shell) = value.declaration_into_dependent_projection();
    let Some(index) = dependents.iter().rposition(|dependent| {
        matches!(
            dependent,
            VerbDependent::DirectObject(_) | VerbDependent::Temporal(_) | VerbDependent::Manner(_)
        )
    }) else {
        return (dependents, None, Vec::new(), shell);
    };
    let after = dependents.split_off(index + 1);
    let (VerbDependent::DirectObject(dependent)
    | VerbDependent::Temporal(dependent)
    | VerbDependent::Manner(dependent)) = dependents.remove(index)
    else {
        unreachable!("the direct-object lens index pins its variants")
    };
    (dependents, Some(dependent), after, shell)
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
    |_: &InfinitiveClause| true
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
    ) && value.declaration_core_features().is_some()
}

deckmaste_constructions_macro::constructions! {
    group predicate;

    lens verb_phrase_direct_object bind VerbPhrase via from_direct_object_lens_parts, into_direct_object_lens_parts {
        before: vec VerbDependent,
        object: opt NounPhrase,
        after: vec VerbDependent,
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
        form only @ 0 = identity(head);
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
        form only @ 0 inverse check(is_verb_phrase_auxiliary) = identity(auxiliary) predicate;
        dominates verb_phrase_adjective;
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
        lens verb_phrase_direct_object from predicate {
            focus object with object;
        }
        derive features: Features = reduce_verb_phrase_direct_object_features(predicate, object);
        derive argument_complete: Features = complete_direct_object(predicate, object);
        derive reduced_passive: Features = reduce_reduced_passive_direct(predicate, object);
        derive precedence: Features = disprefer_active_temporal_attachment(predicate, object);
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
        derive features: Features = reduce_verb_phrase_adjective_features(predicate, adjective);
        derive argument_complete: Features = complete_adjective(predicate, adjective);
        derive object_gap: Features = reduce_object_gap_adjective(predicate, adjective);
        derive base_precedence_2: Features = reduce_verb_phrase_adjective_features(predicate, adjective);
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

    construction verb_phrase_infinitive: VerbPhrase {
        bind VerbPhrase {
            predicate: hole VerbPhrase,
            infinitive: hole InfinitiveClause,
        }
        lens verb_phrase_infinitive from predicate {
            focus infinitive with infinitive;
        }
        derive features: Features = reduce_verb_phrase_infinitive_features(predicate, infinitive);
        derive argument_complete: Features = complete_infinitive(predicate, infinitive);
        derive object_gap: Features = reduce_object_gap_infinitive(predicate, infinitive);
        form only @ 0 inverse check(is_verb_phrase_infinitive) = predicate infinitive;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&PREDICATE_DECLARATION];

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::sync::OnceLock;

    use super::*;
    use crate::catalog::Catalogs;
    use crate::features::Contraction;
    use crate::features::Number;
    use crate::features::Person;
    use crate::features::VerbSlot;
    use crate::grammar::GeneratedActivation;
    use crate::grammar::Nonterminal;
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

    fn full_auxiliary(auxiliary: Auxiliary, inflection: AuxiliaryInflection) -> AuxiliaryInstance {
        AuxiliaryInstance {
            auxiliary,
            inflection,
            contracted_negation: Contraction::Full,
        }
    }

    fn all_groups_with_predicate() -> &'static [&'static GroupData] {
        static GROUPS: OnceLock<&'static [&'static GroupData]> = OnceLock::new();
        GROUPS.get_or_init(|| {
            let mut groups = crate::constructions::GROUPS.to_vec();
            groups.push(&PREDICATE_DECLARATION);
            Box::leak(groups.into_boxed_slice())
        })
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
        // category is not `Features::Verb`, or let the handwritten mirror
        // mask a broken inactive generated lowering path.
        assert!(
            !crate::constructions::GROUPS
                .iter()
                .any(|group| group.name == "predicate"),
            "the Task 1 declaration compiles without production activation"
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
        // arbitrary reduced-recipient-passive theme, or let a handwritten
        // mirror hide a broken generated lowering path.
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
        assert!(
            crate::grammar::parse_nonterminal_with_activation(
                "dealt a card",
                &Catalogs::default(),
                Nonterminal::ReducedRecipientPassive,
                activation,
            )
            .is_err()
        );
    }

    #[test]
    fn inactive_generated_adjective_pp_selection_matches_in_both_orders() {
        // Mutations caught: charge adjective like PP instead of +2 versus +1,
        // or let that defect make equal-cost alternatives depend on
        // registration order. The conditional active-temporal cost has its
        // own exact projection test in `grammar::generated`.
        let source = "Nissa's power is equal to the number of lands you control.";
        let self_reference = crate::identity::SelfReference::new("Nissa Revane", true);
        let catalogs = crate::grammar::fixture_catalogs();
        let handwritten = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
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
        let handwritten_shape = format!(
            "{:#?}",
            handwritten[0]
                .sentence()
                .expect("the handwritten parse lowers a sentence")
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
                handwritten[1]
                    .sentence()
                    .expect("the reversed handwritten parse lowers a sentence")
            ),
            handwritten_shape,
            "handwritten selection changed under reversal for {source:?}"
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
        assert_eq!(generated_shape, handwritten_shape, "{source:?}");
        assert_eq!(generated[0].cost(), handwritten[0].cost(), "{source:?}");
        assert_eq!(generated[1].cost(), handwritten[1].cost(), "{source:?}");
    }
}
