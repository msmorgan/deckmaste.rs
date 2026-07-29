---
needs: []
---
`graveyard_count_filter` (`crates/deckmaste_migrations/src/parsers/count.rs`,
~line 105) re-implements the graveyard-card filter atom assembly
(`InZone(Graveyard)` / `Owner(Ref(You))` / singleton-unwrap) that `effect.rs`
already builds — a third source of truth. Its output is byte-identical to
`effect::graveyard_card_filter` (owned: your-graveyard) and
`effect::any_graveyard_card_filter` (unowned: a-graveyard) in every case,
including the singleton-unwrap and the `?` / `return None` decline paths; only
`graveyard_card_type` was made `pub(super)` and reused. A future change to the
graveyard-card predicate (a new zone-scope atom, a different singleton
convention) would have to be edited in count.rs AND both effect.rs builders, and
any drift silently produces mismatched RON between "for each … in graveyard"
scalers and the effect-side graveyard filters.

Fix: make `graveyard_card_filter` / `any_graveyard_card_filter` `pub(super)` and
have `graveyard_count_filter` delegate — `if owned {
effect::graveyard_card_filter(subject) } else {
effect::any_graveyard_card_filter(subject) }` — keeping only the count-specific
suffix-stripping local. Single source of truth for the filter shape.

Verify: `cargo test -p deckmaste_migrations` (the graveyard-scaler parse tests)
stays green and the emitted RON for "for each … in your graveyard" / "… in a
graveyard" is unchanged byte-for-byte.

(Surfaced by the code review on `engine-block-legality-query`; the finding is in
the for-each-pump / graveyard-scaler work, not that feature.)

Completed 2026-07-28: the count parser now delegates to the two effect-parser
builders; migration tests and a full Wizards regeneration stayed unchanged.
