//! Checked public construction and rendering facade for activation costs.

pub use deckmaste_construction_compiler::runtime::DeclarationViolation as BuildError;

use crate::RenderError;
use crate::syntax::Cost;
use crate::syntax::CostComponent;
use crate::syntax::FlavorHeader;

/// Builds a cost through the declaration's checked ingress.
pub fn build_cost(
    flavor_header: Option<FlavorHeader>,
    components: Vec<CostComponent>,
) -> Result<Cost, BuildError> {
    crate::constructions::ability::build(flavor_header, components)
}

/// Projects a cost through its declaration-owned representation.
pub fn parts_cost(value: &Cost) -> Result<(Option<FlavorHeader>, Vec<CostComponent>), BuildError> {
    Ok(crate::constructions::ability::parts(value))
}

/// Renders a cost through the generated inverse dispatcher.
pub fn render(value: &Cost, name: &str, is_legendary: bool) -> Result<String, RenderError> {
    crate::renderer::render_cost(value, name, is_legendary)
}
