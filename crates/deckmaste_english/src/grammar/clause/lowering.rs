use super::Auxiliary;
use super::BareNominalAdjunct;
use super::NounPhrase;
use super::Phrase;
use super::Predicate;
use super::PredicateForm;
use super::VerbDependent;
use super::VerbPhrase;
use super::VerbSlot;
use super::reduction::auxiliary_form;
use crate::constructions::predicate::FinishedPredicate;
use crate::grammar::reduction::predicate_form;
use crate::syntax::InfinitiveClause;

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
    modifier: &crate::syntax::CoordinatedModifier,
) -> Option<crate::syntax::CoordinatedAdjectivePhrase> {
    let first = modifier_as_predicative_adjective(modifier.first().clone())?;
    let mut rest = Vec::with_capacity(modifier.rest().len());
    for coordination in modifier.rest() {
        rest.push((
            coordination.conjunction(),
            modifier_as_predicative_adjective(coordination.modifier().clone())?,
        ));
    }
    crate::constructions::coordination::build_coordinated_adjective_members(first, rest).ok()
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
    let crate::syntax::NounPhraseKind::Nominal(nominal) = phrase.kind() else {
        return None;
    };
    nominal.head().noun().bare_nominal_adjunct()
}
