---
needs: []
---
Parse the library-search (tutor) effect family: `Search your library for a
basic land card, reveal it, put it into your hand, then shuffle.` and its
variants — destination battlefield (optionally `tapped`), hand, graveyard;
`up to N cards`, `up to X … with different names`; multi-filter searches
(`a basic Plains, Swamp, or Forest card`); opponent-subject
(`its controller may search their library …`).

The grammar primitives exist: `Binder::Search` / `Binder::SearchOne`
(`crates/deckmaste_core/src/binder.rs`) plus reveal and zone-move actions.
Missing is the effect production in
`crates/deckmaste_migrations/src/parsers/effect.rs` that recognizes the
search‑reveal‑move‑shuffle sentence chain and lowers it onto those binders.
The chain is highly formulaic — one production with slots for filter, count,
destination, tapped-ness, and reveal-ness covers the overwhelming majority.

**~562 of 17,022 one-away cards** (2026-07-16 tally), the largest
single-sentence effect family. Fetchland/tutor staples throughout.

Verify: `cargo xtask generate plugins/wizards` graduation delta;
`cargo xtask fidelity` on a hand-destination and a battlefield-tapped card.
