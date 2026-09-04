---
needs: []
---
**Split `CardFace` from `Characteristics` on the Rust side.** Residue of
`workbench-card-face-record` (2026-09-04), which made the split in the
workbench and stopped at the Rust alias `pub type CardFace =
Characteristics` (`crates/deckmaste_card/src/card.rs`): 749 references
across 11 crates plus the serde shape of every card RON. The ruling stands
— a face has characteristics [CR#109.3] and possibly layout-specific data;
alternative characteristics (flip, adventure [CR#710.1,715.2]) are not
faces — so core, card and lowering should refuse a face where alternative
characteristics are meant, the way the workbench pin
`badFaceAsFlipAlternative` does.

Fix: `struct CardFace { characteristics: Characteristics, … }` in
`deckmaste_card` (or wherever the drift guard says the type belongs), the
flip/adventure alternatives typed `Characteristics`, a serde shape that keeps
every card RON loading (a `#[serde(transparent)]` or migration step —
decide from the guard), the drift guard re-run, a Rust test mirroring the
workbench pin. `deckmaste_semantics` untouched.

Size: M–L. Done when: no alias remains; the alternatives cannot be passed a
face; every card RON loads; `cargo test --workspace` green. Standard
constraints apply.
