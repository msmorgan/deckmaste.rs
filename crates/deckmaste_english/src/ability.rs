//! Checked public construction and rendering facade for complete abilities.

pub use deckmaste_construction_compiler::runtime::DeclarationViolation as BuildError;

use crate::RenderError;
use crate::syntax::Ability;
use crate::syntax::AbilityHeader;
use crate::syntax::AbilityKind;

/// Builds one ability through the declaration's checked ingress.
///
/// # Errors
///
/// Returns an error for an invalid frame payload.
pub fn build_ability(
    header: Option<AbilityHeader>,
    kind: AbilityKind,
) -> Result<Ability, BuildError> {
    crate::constructions::ability::build_ability_root(header, kind)
}

/// Renders a declaration-checked ability from its semantic fields.
///
/// # Errors
///
/// Returns an error if the ability or one of its components has no exact
/// inverse.
pub fn render(value: &Ability, name: &str, is_legendary: bool) -> Result<String, RenderError> {
    crate::renderer::render_ability(value, name, is_legendary)
}
