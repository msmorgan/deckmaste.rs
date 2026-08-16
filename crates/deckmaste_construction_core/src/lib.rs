mod emit;
mod feature;
mod format;
mod model;
mod parse;
mod plan;
mod source;
mod validate;

#[cfg(test)]
mod test_support;

pub use crate::model::*;
pub use crate::plan::DeclarationKey;
pub use crate::plan::DeclarationKind;
pub use crate::plan::EmissionPlan;
pub use crate::plan::GeneratedItem;
pub use crate::plan::ItemKey;
pub use crate::plan::NamedKind;
pub use crate::plan::TerminalContribution;
pub use crate::plan::TerminalKind;
pub use crate::plan::TerminalVariantContribution;
pub use crate::validate::ValidatedDeclarations;

pub fn parse_declarations(tokens: proc_macro2::TokenStream) -> syn::Result<Declarations> {
    parse::parse_declarations(tokens)
}

/// Validates and lowers parsed declarations into the sealed code-generation IR.
///
/// # Errors
///
/// Returns combined, span-bearing errors from the earliest validation pass that
/// finds invalid declarations.
pub fn validate_declarations(raw: Declarations) -> syn::Result<ValidatedDeclarations> {
    validate::validate_declarations(raw)
}

pub fn generate(tokens: proc_macro2::TokenStream) -> syn::Result<Expansion> {
    let declarations = validate_declarations(parse_declarations(tokens)?)?;
    let plan = plan::plan_emission(&declarations)?;
    Ok(Expansion { plan })
}

pub fn expand(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    generate(tokens).map_or_else(syn::Error::into_compile_error, |expansion| {
        expansion.tokens()
    })
}

pub fn invocation_from_source(source: &str) -> syn::Result<Invocation> {
    source::invocation_from_source(source)
}

/// Formats an expansion item-by-item as parsed Rust syntax.
///
/// # Errors
///
/// Returns an error if a planned item is no longer exactly one valid Rust item.
pub fn format_expansion(expansion: &Expansion) -> syn::Result<String> {
    format::format_plan(expansion.plan())
}

/// Formats one generated item as parsed Rust syntax.
///
/// # Errors
///
/// Returns an error if the generated tokens are not exactly one valid Rust
/// item.
pub fn format_generated_item(item: &GeneratedItem) -> syn::Result<String> {
    format::format_item(item)
}
