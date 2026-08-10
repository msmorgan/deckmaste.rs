//! Checked construction, projection, and rendering API for noun phrases.
//!
//! Builders return sealed [`NounPhrase`] values after validating their
//! generated declarations. Matching `parts_*` functions expose typed,
//! immutable projections.

use deckmaste_construction_compiler::runtime::DeclarationViolation;

use crate::RenderError;
pub use crate::constructions::noun_phrase::RulesObjectNounPhrase;
pub use crate::constructions::noun_phrase::build_noun_phrase_any_number_of;
pub use crate::constructions::noun_phrase::build_noun_phrase_demonstrative;
pub use crate::constructions::noun_phrase::build_noun_phrase_each_partitive;
pub use crate::constructions::noun_phrase::build_noun_phrase_full_this_card;
pub use crate::constructions::noun_phrase::build_noun_phrase_half;
pub use crate::constructions::noun_phrase::build_noun_phrase_half_rounded_down;
pub use crate::constructions::noun_phrase::build_noun_phrase_half_rounded_up;
pub use crate::constructions::noun_phrase::build_noun_phrase_minus;
pub use crate::constructions::noun_phrase::build_noun_phrase_nominal;
pub use crate::constructions::noun_phrase::build_noun_phrase_object_pronoun;
pub use crate::constructions::noun_phrase::build_noun_phrase_partitive;
pub use crate::constructions::noun_phrase::build_noun_phrase_possessive_this_card;
pub use crate::constructions::noun_phrase::build_noun_phrase_quantity;
pub use crate::constructions::noun_phrase::build_noun_phrase_reciprocal;
pub use crate::constructions::noun_phrase::build_noun_phrase_set_exception_bare;
pub use crate::constructions::noun_phrase::build_noun_phrase_set_exception_for;
pub use crate::constructions::noun_phrase::build_noun_phrase_subject_pronoun;
pub use crate::constructions::noun_phrase::build_noun_phrase_this_card;
pub use crate::constructions::noun_phrase::build_rules_object_noun_phrase;
pub use crate::constructions::noun_phrase::parts_noun_phrase_any_number_of;
pub use crate::constructions::noun_phrase::parts_noun_phrase_demonstrative;
pub use crate::constructions::noun_phrase::parts_noun_phrase_each_partitive;
pub use crate::constructions::noun_phrase::parts_noun_phrase_full_this_card;
pub use crate::constructions::noun_phrase::parts_noun_phrase_half;
pub use crate::constructions::noun_phrase::parts_noun_phrase_half_rounded_down;
pub use crate::constructions::noun_phrase::parts_noun_phrase_half_rounded_up;
pub use crate::constructions::noun_phrase::parts_noun_phrase_minus;
pub use crate::constructions::noun_phrase::parts_noun_phrase_nominal;
pub use crate::constructions::noun_phrase::parts_noun_phrase_object_pronoun;
pub use crate::constructions::noun_phrase::parts_noun_phrase_partitive;
pub use crate::constructions::noun_phrase::parts_noun_phrase_possessive_this_card;
pub use crate::constructions::noun_phrase::parts_noun_phrase_quantity;
pub use crate::constructions::noun_phrase::parts_noun_phrase_reciprocal;
pub use crate::constructions::noun_phrase::parts_noun_phrase_set_exception_bare;
pub use crate::constructions::noun_phrase::parts_noun_phrase_set_exception_for;
pub use crate::constructions::noun_phrase::parts_noun_phrase_subject_pronoun;
pub use crate::constructions::noun_phrase::parts_noun_phrase_this_card;
pub use crate::constructions::noun_phrase::parts_rules_object_noun_phrase;
use crate::syntax::NounPhrase;
use crate::syntax::NounPhraseCoordination;
use crate::syntax::NounPhraseKind;

/// Builds a complete noun-phrase coordination through the C01 declaration.
///
/// # Errors
///
/// Returns an error when the member list violates the declaration's length or
/// conjunction requirements.
pub fn build_noun_phrase_coordination(
    first: Box<NounPhrase>,
    rest: Vec<NounPhraseCoordination>,
) -> Result<NounPhrase, DeclarationViolation> {
    let coordination = crate::syntax::CoordinatedNounPhrase::try_new(first, rest)?;
    Ok(NounPhrase::from_coordination_declaration(coordination))
}

/// Projects a complete noun-phrase coordination into owned declaration parts.
///
/// # Panics
///
/// Panics when `value` was not built by the coordination declaration.
#[must_use]
pub fn parts_noun_phrase_coordination(
    value: &NounPhrase,
) -> (NounPhrase, Vec<NounPhraseCoordination>) {
    let NounPhraseKind::Coordinated(coordination) = value.kind() else {
        panic!("noun_phrase_coordination admits only Coordinated")
    };
    (
        coordination.first().as_ref().clone(),
        coordination.rest().clone(),
    )
}

/// Renders a checked noun phrase using the supplied card identity context.
///
/// # Errors
///
/// Returns an error when a nested lexical item has no surface form or a
/// self-reference cannot be rendered from the supplied name.
pub fn render(value: &NounPhrase, name: &str, is_legendary: bool) -> Result<String, RenderError> {
    crate::renderer::render_noun_phrase(value, name, is_legendary)
}
