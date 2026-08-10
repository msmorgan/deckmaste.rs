//! Checked public construction and rendering facade for keyword-ability lines.

pub use deckmaste_construction_compiler::runtime::DeclarationViolation as BuildError;

use crate::RenderError;
use crate::syntax::KeywordAbility;
use crate::syntax::KeywordAbilityList;
use crate::syntax::Paragraph;

/// Builds a nonempty keyword-ability line through the declaration's checked
/// ingress.
pub fn build_keyword_line(
    abilities: Vec<KeywordAbility>,
    trailing: Option<Paragraph>,
) -> Result<KeywordAbilityList, BuildError> {
    crate::constructions::ability::build_keyword_list(abilities, trailing)
}

/// Projects a keyword-ability line through its declaration-owned
/// representation.
pub fn parts_keyword_line(
    value: &KeywordAbilityList,
) -> Result<(Vec<KeywordAbility>, Option<Paragraph>), BuildError> {
    Ok(crate::constructions::ability::keyword_list_parts(value))
}

/// Renders a keyword-ability line through the generated inverse dispatcher.
pub fn render(
    value: &KeywordAbilityList,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    crate::renderer::render_keyword_line(value, name, is_legendary)
}
