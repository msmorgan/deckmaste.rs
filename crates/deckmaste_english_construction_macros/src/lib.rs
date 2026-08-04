//! Proc-macro entry points for the construction compiler. Milestone 0 ships
//! only the hardcoded `spike_sealed!` feasibility macro, which expands on its
//! own and takes no dependency on `deckmaste_english_construction_compiler`;
//! the real `constructicon!` entry point, which will parse into that crate's
//! declaration model and route through its validator, replaces it in a later
//! milestone.

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

            // Opt-in validating deserialization: a private raw mirror is
            // deserialized structurally, then handed to the same validator
            // as public construction. There is no other deserialize path.
            impl<'de> serde::Deserialize<'de> for SpikeCoordination {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    #[derive(serde::Deserialize)]
                    struct Raw {
                        members: u16,
                    }
                    let raw = Raw::deserialize(deserializer)?;
                    SpikeCoordination::try_new(raw.members)
                        .map_err(|SpikeError| serde::de::Error::custom("members must be at least 1"))
                }
            }

            // Serde OPT-OUT twin: sealed the same way, deliberately no
            // Deserialize impl — absence is the guarantee.
            pub struct SpikeOpenRecord {
                label: u16,
            }

            impl SpikeOpenRecord {
                pub fn try_new(label: u16) -> Result<Self, SpikeError> {
                    Ok(Self { label })
                }

                pub fn label(&self) -> u16 {
                    self.label
                }
            }
        }
        pub use #module::SpikeCoordination;
        pub use #module::SpikeError;
        pub use #module::SpikeOpenRecord;
    })
}
