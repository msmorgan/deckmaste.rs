//! Checked construction and rendering API for adjectives.
//!
//! Every builder validates the derived declaration before returning the
//! canonical syntax value. The syntax fields are read-only outside this crate;
//! semantic accessors provide immutable views without exposing declaration
//! inverses as a parallel public API.

use crate::RenderError;
pub use crate::constructions::adjective::build_adjective;
pub use crate::constructions::adjective::build_adjective_phrase;
pub use crate::constructions::adjective::build_adjective_phrase_comparison;
pub use crate::constructions::adjective::build_adjective_phrase_degree_measure;
pub use crate::constructions::adjective::build_adjective_phrase_face_down;
pub use crate::constructions::adjective::build_adjective_phrase_face_up;
pub use crate::constructions::adjective::build_adjective_phrase_infinitive;
pub use crate::constructions::adjective::build_adjective_phrase_prepositional;
pub use crate::constructions::adjective::build_comparison_standard;
pub use crate::constructions::adjective::build_comparison_than;
pub use crate::constructions::adjective::build_comparison_than_or_equal_to;
use crate::syntax::AdjectivePhrase;

/// Renders a checked adjective phrase through its declaration-generated
/// inverse. `name` and `is_legendary` supply the ordinary card-identity
/// context for a nested comparison standard.
///
/// # Errors
///
/// Returns an error when a nested lexical item has no surface form.
pub fn render(
    phrase: &AdjectivePhrase,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    crate::renderer::render_adjective_phrase(phrase, name, is_legendary)
}
