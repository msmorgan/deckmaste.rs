use proc_macro::TokenStream;

#[proc_macro]
pub fn constructions(input: TokenStream) -> TokenStream {
    deckmaste_construction_core::expand(input.into()).into()
}
