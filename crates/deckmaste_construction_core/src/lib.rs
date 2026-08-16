mod model;
mod parse;
mod source;

pub use crate::model::*;

pub fn parse_declarations(tokens: proc_macro2::TokenStream) -> syn::Result<Declarations> {
    parse::parse_declarations(tokens)
}

pub fn generate(tokens: proc_macro2::TokenStream) -> syn::Result<Expansion> {
    let _declarations = parse_declarations(tokens)?;
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
