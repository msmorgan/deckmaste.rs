//! Milestone-0 golden-formatting spike. `format_emission` survives into the
//! real emitter; `spike_sealed_tokens` is throwaway scaffolding mirroring
//! the macros crate's spike expansion.

use proc_macro2::TokenStream;

pub fn spike_sealed_tokens() -> TokenStream {
    quote::quote! {
        mod __spike_golden {
            #[derive(Debug, PartialEq, Eq)]
            pub struct SpikeError;

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
        pub use __spike_golden::SpikeCoordination;
        pub use __spike_golden::SpikeError;
    }
}

pub fn format_emission(tokens: TokenStream) -> String {
    let file = syn::parse2(tokens).expect("emitted tokens must parse as a Rust file");
    prettyplease::unparse(&file)
}
