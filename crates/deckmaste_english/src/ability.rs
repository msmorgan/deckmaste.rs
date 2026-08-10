//! Checked public construction and rendering facade for complete abilities.

pub use deckmaste_construction_compiler::runtime::DeclarationViolation as BuildError;

use crate::RenderError;
use crate::catalog::CatalogAtom;
use crate::syntax::Ability;
use crate::syntax::AbilityKind;
use crate::syntax::FlavorHeader;

/// Builds one ability through the declaration's checked ingress.
pub fn build_ability(
    ability_word: Option<CatalogAtom>,
    flavor_header: Option<FlavorHeader>,
    kind: AbilityKind,
) -> Result<Ability, BuildError> {
    crate::constructions::ability::build_ability_root(ability_word, flavor_header, kind)
}

/// Projects an ability through its declaration-owned representation.
pub fn parts_ability(
    value: &Ability,
) -> Result<(Option<CatalogAtom>, Option<FlavorHeader>, AbilityKind), BuildError> {
    crate::constructions::ability::ability_root_parts(value)
}

/// Renders an ability through the generated inverse dispatcher.
pub fn render(value: &Ability, name: &str, is_legendary: bool) -> Result<String, RenderError> {
    crate::renderer::render_ability(value, name, is_legendary)
}
