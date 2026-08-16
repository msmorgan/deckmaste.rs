mod feature;
mod model;
mod parse;
mod source;
mod validate;

pub use crate::model::*;
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
    let _declarations = validate_declarations(parse_declarations(tokens)?)?;
    Ok(Expansion::default())
}

pub fn expand(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    generate(tokens).map_or_else(syn::Error::into_compile_error, |expansion| expansion.tokens)
}

pub fn invocation_from_source(source: &str) -> syn::Result<Invocation> {
    source::invocation_from_source(source)
}

pub fn format_expansion(expansion: &Expansion) -> syn::Result<String> {
    let file = syn::parse2::<syn::File>(expansion.tokens.clone())?;
    Ok(prettyplease::unparse(&file))
}
