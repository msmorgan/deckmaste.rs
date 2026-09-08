//! Bidirectional declarations for the v3 packed chart.
//!
//! Lossy surface alternatives are compile-time errors:
//!
//! ```compile_fail
//! deckmaste_construction_v3::constructions! {
//!     mod lossy {
//!         category Phrase();
//!         construction Lost: Phrase {
//!             form [head: lexical(Noun)];
//!             form [];
//!         }
//!     }
//! }
//! ```
//!
//! Recursion that can revisit a Category without consuming input is rejected:
//!
//! ```compile_fail
//! deckmaste_construction_v3::constructions! {
//!     mod cyclic {
//!         category Phrase();
//!         construction Cycle: Phrase { form [child: optional(Phrase)]; }
//!     }
//! }
//! ```

use proc_macro::TokenStream;

#[proc_macro]
pub fn constructions(input: TokenStream) -> TokenStream {
    deckmaste_construction_v3_core::expand(input.into()).into()
}
