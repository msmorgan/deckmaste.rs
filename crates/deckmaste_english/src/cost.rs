//! Checked public construction and rendering facade for activation costs.

pub use deckmaste_construction_compiler::runtime::DeclarationViolation as BuildError;

use crate::RenderError;
use crate::syntax::Cost;
use crate::syntax::CostComponent;
use crate::syntax::FlavorHeader;

/// Builds a cost through the declaration's checked ingress.
///
/// # Errors
///
/// Returns an error when `components` is empty or the fields otherwise violate
/// the `cost` declaration.
pub fn build_cost(
    flavor_header: Option<FlavorHeader>,
    components: Vec<CostComponent>,
) -> Result<Cost, BuildError> {
    crate::constructions::ability::build(flavor_header, components)
}

/// Renders a declaration-checked cost from its semantic components.
///
/// # Errors
///
/// Returns an error if the cost or one of its components has no exact inverse.
pub fn render(value: &Cost, name: &str, is_legendary: bool) -> Result<String, RenderError> {
    crate::renderer::render_cost(value, name, is_legendary)
}
