//! Derives for `macro_ron` consumers: `SupportsMacros` (macro-aware
//! `Serialize`/`Deserialize` + the `SupportsMacros` and `Expand` traits) and
//! `Expand` (recursion-only, for plain grammar types).

use proc_macro::TokenStream;

mod generate;
mod input;

// `serde` is registered as a helper attribute so struct-variant fields can
// carry `#[serde(...)]` attrs (forwarded onto the generated helper structs)
// without the enum deriving serde's traits itself.
#[proc_macro_derive(SupportsMacros, attributes(macro_ron, serde))]
pub fn derive_supports_macros(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    input::parse(&input)
        .and_then(|input| generate::supports_macros(&input))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

// `macro_ron` is registered so a stray marker on an Expand-only type gets
// this derive's "markers belong to SupportsMacros" error instead of rustc's
// unresolved-attribute one.
#[proc_macro_derive(Expand, attributes(macro_ron))]
pub fn derive_expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    generate::expand_only(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
