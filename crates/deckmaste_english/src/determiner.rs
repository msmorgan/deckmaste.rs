//! Checked construction and projection API for determiners and possessors.
//!
//! Every builder validates its D01 declaration before returning a value.
//! [`Determiner`] and [`Possessor`] expose only immutable semantic views, so
//! callers cannot bypass the generated construction constraints.
//!
//! [`Determiner`]: crate::syntax::Determiner
//! [`Possessor`]: crate::syntax::Possessor

use deckmaste_construction_compiler::runtime::DeclarationViolation;

use crate::syntax::AdjectivePhrase;
use crate::syntax::ClosedDeterminer;
use crate::syntax::Demonstrative;
use crate::syntax::Determiner;
use crate::syntax::NominalPhrase;
use crate::syntax::Quantity;
use crate::syntax::ThisCardForm;
use crate::word::NounInstance;
use crate::word::Pronoun;

fn closed(identity: ClosedDeterminer) -> Determiner {
    build_determiner_closed(identity)
        .expect("every closed determiner identity satisfies its declaration")
}

#[must_use]
pub fn the() -> Determiner {
    closed(ClosedDeterminer::The)
}

#[must_use]
pub fn each() -> Determiner {
    closed(ClosedDeterminer::Each)
}

#[must_use]
pub fn another() -> Determiner {
    closed(ClosedDeterminer::Another)
}

#[must_use]
pub fn indefinite() -> Determiner {
    closed(ClosedDeterminer::Indefinite)
}

#[must_use]
pub fn demonstrative(value: Demonstrative) -> Determiner {
    closed(ClosedDeterminer::Demonstrative(value))
}

#[must_use]
pub fn possessive_pronoun(value: Pronoun) -> Determiner {
    closed(ClosedDeterminer::PossessivePronoun(value))
}

#[must_use]
pub fn all() -> Determiner {
    closed(ClosedDeterminer::All)
}

#[must_use]
pub fn any() -> Determiner {
    closed(ClosedDeterminer::Any)
}

#[must_use]
pub fn no() -> Determiner {
    closed(ClosedDeterminer::No)
}

#[must_use]
/// Builds a target determiner with an optional quantity.
///
/// # Panics
///
/// Panics only if the declaration rejects an already validated quantity.
pub fn target(quantity: Option<Quantity>) -> Determiner {
    match quantity {
        Some(quantity) => build_determiner_quantified_target(quantity),
        None => build_determiner_target(),
    }
    .expect("target determiner inputs satisfy their declaration")
}

#[must_use]
/// Builds a direct quantity determiner.
///
/// # Panics
///
/// Panics only if the declaration rejects an already validated quantity.
pub fn quantity(quantity: Quantity) -> Determiner {
    build_determiner_quantity(quantity)
        .expect("validated quantities satisfy the determiner declaration")
}

#[must_use]
/// Builds a self-reference possessive determiner.
///
/// # Panics
///
/// Panics only if the declaration rejects a closed self-reference form.
pub fn possessive_this_card(form: ThisCardForm) -> Determiner {
    build_determiner_possessive_this_card(form)
        .expect("self-reference forms satisfy the possessive declaration")
}

#[must_use]
/// Builds a possessive determiner from a checked nominal projection.
///
/// # Panics
///
/// Panics if `possessor` is outside the checked D01 possessive-nominal domain.
pub fn possessive_nominal(possessor: NominalPhrase) -> Determiner {
    build_determiner_possessive_noun(possessor)
        .expect("validated possessive nominals satisfy the determiner declaration")
}

/// Builds a closed determiner identity.
///
/// # Errors
///
/// Returns a declaration violation if the identity is outside D01's closed
/// domain.
pub fn build_determiner_closed(
    identity: ClosedDeterminer,
) -> Result<Determiner, DeclarationViolation> {
    crate::constructions::determiner::build_determiner_closed(identity)
}

#[must_use]
pub fn parts_determiner_closed(value: &Determiner) -> ClosedDeterminer {
    crate::constructions::determiner::parts_determiner_closed(value)
}

/// Builds the bare `target` determiner.
///
/// # Errors
///
/// Returns a declaration violation if the declaration cannot build its closed
/// form.
pub fn build_determiner_target() -> Result<Determiner, DeclarationViolation> {
    crate::constructions::determiner::build_determiner_target()
}

pub fn parts_determiner_target(value: &Determiner) {
    crate::constructions::determiner::parts_determiner_target(value);
}

/// Builds a quantified `target` determiner.
///
/// # Errors
///
/// Returns a declaration violation if `quantity` cannot head a target
/// determiner.
pub fn build_determiner_quantified_target(
    quantity: Quantity,
) -> Result<Determiner, DeclarationViolation> {
    crate::constructions::determiner::build_determiner_quantified_target(quantity)
}

#[must_use]
pub fn parts_determiner_quantified_target(value: &Determiner) -> Quantity {
    crate::constructions::determiner::parts_determiner_quantified_target(value)
}

/// Builds a direct quantity determiner.
///
/// # Errors
///
/// Returns a declaration violation if `quantity` cannot determine a noun.
pub fn build_determiner_quantity(quantity: Quantity) -> Result<Determiner, DeclarationViolation> {
    crate::constructions::determiner::build_determiner_quantity(quantity)
}

#[must_use]
pub fn parts_determiner_quantity(value: &Determiner) -> Quantity {
    crate::constructions::determiner::parts_determiner_quantity(value)
}

/// Builds a self-reference possessive determiner.
///
/// # Errors
///
/// Returns a declaration violation if `form` is not admitted by D01.
pub fn build_determiner_possessive_this_card(
    form: ThisCardForm,
) -> Result<Determiner, DeclarationViolation> {
    crate::constructions::determiner::build_determiner_possessive_this_card(form)
}

#[must_use]
pub fn parts_determiner_possessive_this_card(value: &Determiner) -> ThisCardForm {
    crate::constructions::determiner::parts_determiner_possessive_this_card(value)
}

/// Starts a possessive-nominal projection from a noun identity.
///
/// # Errors
///
/// Returns a declaration violation if `head` is not a valid bare nominal head.
pub fn build_possessive_noun_base(
    head: NounInstance,
) -> Result<NominalPhrase, DeclarationViolation> {
    crate::constructions::determiner::build_possessive_noun_base(head)
}

#[must_use]
pub fn parts_possessive_noun_base(value: &NominalPhrase) -> NounInstance {
    crate::constructions::determiner::parts_possessive_noun_base(value)
}

/// Adds a determiner to an undetermined possessive nominal.
///
/// # Errors
///
/// Returns a declaration violation for duplicate determination or failed
/// agreement.
pub fn build_possessive_noun_determined(
    determiner: Determiner,
    possessor: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    crate::constructions::determiner::build_possessive_noun_determined(determiner, possessor)
}

#[must_use]
pub fn parts_possessive_noun_determined(value: &NominalPhrase) -> (Determiner, NominalPhrase) {
    crate::constructions::determiner::parts_possessive_noun_determined(value)
}

/// Converts a checked possessive nominal into a determiner.
///
/// # Errors
///
/// Returns a declaration violation if `possessor` is outside the possessive
/// domain.
pub fn build_determiner_possessive_noun(
    possessor: NominalPhrase,
) -> Result<Determiner, DeclarationViolation> {
    crate::constructions::determiner::build_determiner_possessive_noun(possessor)
}

#[must_use]
pub fn parts_determiner_possessive_noun(value: &Determiner) -> NominalPhrase {
    crate::constructions::determiner::parts_determiner_possessive_noun(value)
}

/// Prefixes an adjective to an undetermined possessive nominal.
///
/// # Errors
///
/// Returns a declaration violation when the adjective or possessor state is
/// inadmissible.
pub fn build_possessive_noun_adjective(
    adjective: AdjectivePhrase,
    possessor: NominalPhrase,
) -> Result<NominalPhrase, DeclarationViolation> {
    crate::constructions::determiner::build_possessive_noun_adjective(adjective, possessor)
}

#[must_use]
pub fn parts_possessive_noun_adjective(value: &NominalPhrase) -> (AdjectivePhrase, NominalPhrase) {
    crate::constructions::determiner::parts_possessive_noun_adjective(value)
}
