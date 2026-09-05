mod emit;
mod feature;
mod format;
mod identifier;
pub mod macro_def;
mod model;
mod morphology;
mod parse;
mod plan;
mod report;
mod semantic;
mod source;
mod validate;

#[cfg(test)]
mod test_support;

pub use crate::model::*;
pub use crate::plan::DeclarationKey;
pub use crate::plan::EmissionPlan;
pub use crate::plan::GeneratedItem;
pub use crate::plan::ItemKey;
pub use crate::plan::NamedKind;
pub use crate::plan::SourceDeclarationKind;
pub use crate::plan::TerminalContribution;
pub use crate::plan::TerminalKind;
pub use crate::plan::TerminalSurfaceContribution;
pub use crate::plan::TerminalVariantContribution;
pub use crate::report::EscapeHatchReport;
pub use crate::report::MorphologyIrregular;
pub use crate::report::MorphologyOverride;
pub use crate::report::TerminalBindingDeclaration;
pub use crate::report::TerminalBindingDeclarationKind;
pub use crate::validate::ValidatedDeclarations;

/// Returns the sealed construction feature keys used by grammar declarations.
pub fn feature_keys() -> impl ExactSizeIterator<Item = &'static str> {
    feature::Feature::all()
        .iter()
        .copied()
        .map(feature::Feature::key)
}

/// Parses one grammar-wide declaration invocation.
///
/// # Errors
///
/// Returns a span-bearing syntax error for malformed or deferred MVP input.
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

/// Parses, validates, and plans all generated Rust items.
///
/// # Errors
///
/// Returns a span-bearing parse, validation, or internal emission error.
pub fn generate(tokens: proc_macro2::TokenStream) -> syn::Result<Expansion> {
    let declarations = validate_declarations(parse_declarations(tokens)?)?;
    generate_from_semantic(declarations.semantic())
}

fn generate_from_semantic(semantic: &semantic::SemanticPlan) -> syn::Result<Expansion> {
    validate::validate_declaration_verb_consumers(semantic)?;
    let plan = plan::plan_emission(semantic)?;
    let escape_hatches = report::escape_hatch_report(semantic)?;
    Ok(Expansion {
        plan,
        escape_hatches,
    })
}

pub fn expand(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    generate(tokens).map_or_else(syn::Error::into_compile_error, |expansion| {
        expansion.tokens()
    })
}

/// Extracts the sole direct grammar-wide invocation from Rust source.
///
/// # Errors
///
/// Returns an error for missing, repeated, or indirect invocations.
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
