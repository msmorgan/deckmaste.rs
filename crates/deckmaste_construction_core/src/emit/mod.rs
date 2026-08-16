use proc_macro2::TokenStream;
use quote::quote;

pub(crate) mod ast;
pub(crate) mod build;
pub(crate) mod render;
pub(crate) mod rules;
pub(crate) mod terminal;
pub(crate) mod visit;

const RUST_SOURCE_MARGIN: usize = 100;

pub(super) fn call_match_arm(
    pattern: &TokenStream,
    call: &TokenStream,
    indentation: usize,
) -> TokenStream {
    let candidate = quote! { #pattern => #call, };
    if indentation + compact_rust_width(&candidate) + 1 >= RUST_SOURCE_MARGIN {
        quote! { #pattern => { #call; } }
    } else {
        quote! { #pattern => #call }
    }
}

fn compact_rust_width(tokens: &TokenStream) -> usize {
    tokens
        .to_string()
        .replace(" :: ", "::")
        .replace(" . ", ".")
        .replace(" (", "(")
        .replace("[ ", "[")
        .replace("{ ", "{")
        .replace(" )", ")")
        .replace(" ]", "]")
        .replace(" }", "}")
        .replace(" ,", ",")
        .replace(" ;", ";")
        .chars()
        .count()
}
