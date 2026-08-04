//! Thin proc-macro facade over `deckmaste_english_construction_compiler`.
//! Milestone 0 ships only the hardcoded `spike_sealed!` feasibility macro;
//! the real `constructicon!` entry point replaces it in a later milestone.

use proc_macro::TokenStream;

// Milestone-0 feasibility spike. Deliberately hardcoded: it exists to prove
// module-privacy sealing, span quality, and validated ingress through a
// function-like macro, and is replaced by `constructicon!` in a later
// milestone. Input is a bare identifier naming the generated module.
#[proc_macro]
pub fn spike_sealed(input: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(input as syn::Ident);
    expand_spike(&name)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_spike(name: &syn::Ident) -> Result<proc_macro2::TokenStream, syn::Error> {
    let module = quote::format_ident!("__spike_{name}");
    Ok(quote::quote! {
        mod #module {
            #[derive(Debug, PartialEq, Eq)]
            pub struct SpikeError;

            #[derive(Debug, PartialEq, Eq)]
            pub struct SpikeCoordination {
                members: u16,
            }

            impl SpikeCoordination {
                pub fn try_new(members: u16) -> Result<Self, SpikeError> {
                    if members == 0 {
                        return Err(SpikeError);
                    }
                    Ok(Self { members })
                }

                pub fn members(&self) -> u16 {
                    self.members
                }
            }
        }
        pub use #module::SpikeCoordination;
        pub use #module::SpikeError;
    })
}
