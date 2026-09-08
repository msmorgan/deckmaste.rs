//! A directly testable compiler from bidirectional declarations to v3
//! projections.
//!
//! The compiler has no parser, lexical catalog, or morphology dependency.

mod emit;
mod ir;
mod parse;

use proc_macro2::TokenStream;

/// Validate a declaration and generate all projections from its normalized IR.
///
/// # Errors
/// Reports the declaration and violated compilation obligation at its source
/// span.
pub fn compile(input: TokenStream) -> syn::Result<TokenStream> {
    let declaration = syn::parse2(input)?;
    let ir = ir::validate(declaration)?;
    emit::emit(&ir)
}

/// Proc-macro boundary: compilation failures become Rust diagnostics.
#[must_use]
pub fn expand(input: TokenStream) -> TokenStream {
    compile(input).unwrap_or_else(syn::Error::into_compile_error)
}

#[cfg(test)]
mod tests;
