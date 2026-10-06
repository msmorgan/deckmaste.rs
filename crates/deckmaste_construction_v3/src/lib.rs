//! Bidirectional declarations for the v3 packed chart.
//!
//! Ordered frame segments use `segment Predicate` on their declaration,
//! `share_segments left, right` on coordination, and
//! `discharge_segments head, tail` on the shared lexical host. Segment fields
//! must match complete typed lexical frame layouts. Open dependencies are part
//! of packed admission state and cannot be exposed as complete Readings. The
//! first segment retains the overt head's frame choice; subsequent segments
//! may use corresponding slots in another frame declared by that lexical owner.
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
