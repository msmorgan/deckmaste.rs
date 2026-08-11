//! Checked public construction and rendering facade for keyword-ability lines.

pub use deckmaste_construction_compiler::runtime::DeclarationViolation as BuildError;

use crate::RenderError;
use crate::syntax::KeywordAbility;
use crate::syntax::KeywordAbilityList;
use crate::syntax::KeywordListSeparator;
use crate::syntax::Paragraph;
use crate::syntax::SeparatedNonEmpty;

/// Builds a keyword-ability line through the declaration's checked ingress.
///
/// # Errors
///
/// Returns an error when a field violates the `keyword_line` declaration.
pub fn build_keyword_line(
    abilities: SeparatedNonEmpty<KeywordAbility, KeywordListSeparator>,
    trailing: Option<Paragraph>,
) -> Result<KeywordAbilityList, BuildError> {
    crate::constructions::ability::build_keyword_list(abilities, trailing)
}

/// Renders a declaration-checked keyword line from its typed items and
/// separators.
///
/// # Errors
///
/// Returns an error if the line or one of its components has no exact inverse.
pub fn render(
    value: &KeywordAbilityList,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    crate::renderer::render_keyword_line(value, name, is_legendary)
}
