//! `constructions!` — the construction declaration compiler's proc-macro
//! facade. Parses a declaration group, validates it (all errors reported
//! together as `EC…` compile errors spanned to the declaration), and emits
//! the generated module. All logic lives in
//! `deckmaste_construction_compiler`; this crate is the thin pipe.
//!
//! # Consumer contract
//!
//! - Every call site must depend on `deckmaste_construction_compiler` (as a
//!   normal dependency): the emitted declaration-data table names that crate's
//!   `runtime` types by absolute path.
//! - A call site that opts any construction into `deserialize` must itself
//!   depend on `serde` with the `derive` feature, and every field type bound by
//!   that construction must implement `serde::Deserialize` — the generated impl
//!   resolves `serde::…` and those bounds at the call site.
//! - Generated code opens with `use super::*;`: category, codec, and witness
//!   payload types are resolved in the invoking scope.
//! - A group declaring any `free` witness requires the call site to depend on
//!   `deckmaste_features` (as a normal dependency): the generated payload
//!   assertion resolves `::deckmaste_features::SurfaceWitnessPayload` by
//!   absolute path.

use proc_macro::TokenStream;

#[proc_macro]
pub fn constructions(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let group = match deckmaste_construction_compiler::parse::parse_group(input) {
        Ok(group) => group,
        Err(error) => return error.into_compile_error().into(),
    };
    match deckmaste_construction_compiler::validate::validate(&group) {
        Ok(validated) => deckmaste_construction_compiler::emit::emit_group(&validated).into(),
        Err(diags) => deckmaste_construction_compiler::render::to_compile_errors(&diags).into(),
    }
}
