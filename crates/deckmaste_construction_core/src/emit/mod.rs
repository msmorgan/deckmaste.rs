use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::key as identifier_key;
use crate::identifier::local_name;
use crate::identifier::spelling_key;

pub(crate) mod ast;
pub(crate) mod build;
pub(crate) mod render;
pub(crate) mod rules;
pub(crate) mod runtime;
pub(crate) mod scanner;
pub(crate) mod terminal;
pub(crate) mod visit;

const RUST_SOURCE_MARGIN: usize = 100;

#[derive(Clone, Default)]
pub(super) struct LocalAllocator {
    used: HashSet<String>,
}

impl LocalAllocator {
    pub(super) fn reserve(&mut self, name: impl AsRef<str>) {
        self.used.insert(spelling_key(name.as_ref()));
    }

    pub(super) fn reserve_ident(&mut self, name: &syn::Ident) {
        self.used.insert(identifier_key(name));
    }

    pub(super) fn allocate(&mut self, preferred: &str) -> syn::Ident {
        let preferred = local_name(preferred);
        if self.used.insert(preferred.clone()) {
            return local_ident(&preferred);
        }
        for suffix in 2.. {
            let candidate = format!("{preferred}_{suffix}");
            if self.used.insert(candidate.clone()) {
                return local_ident(&candidate);
            }
        }
        unreachable!("the local binder suffix space is unbounded")
    }

    pub(super) fn allocate_ident(&mut self, preferred: &syn::Ident) -> syn::Ident {
        self.allocate(&identifier_key(preferred))
    }
}

fn local_ident(name: &str) -> syn::Ident {
    crate::identifier::emitted_ident(name, proc_macro2::Span::call_site())
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn local_allocator_keys_raw_names_semantically_and_legalizes_keywords() {
        let mut allocator = super::LocalAllocator::default();
        allocator.reserve("context");

        let cases = [
            ("r#payload", "payload"),
            ("payload", "payload_2"),
            ("r#context", "context_2"),
            ("where", "where_value"),
            ("r#where", "where_value_2"),
            ("r#self", "self_value"),
            ("Self", "self_value_2"),
            ("super", "super_value"),
            ("crate", "crate_value"),
            (
                "not an ident",
                "_generated_local_6e_6f_74_20_61_6e_20_69_64_65_6e_74",
            ),
        ];
        for (preferred, expected) in cases {
            assert_eq!(allocator.allocate(preferred), expected, "{preferred}");
        }
    }
}
