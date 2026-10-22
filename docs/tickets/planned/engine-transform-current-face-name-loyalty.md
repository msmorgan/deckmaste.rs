---
needs: [engine-transform]
---
Route the remaining current-characteristic reads through the CURRENT face for a
back-up double-faced permanent — surfaced by the `engine-transform` whole-branch
review. No longer latent (2026-10-21): canon Delver of Secrets transforms, so a
back-up permanent is reachable; the wrong reads below are still unexercised by a
current card.

`engine-transform` converted `base_values` / `abilities_of_source` /
`printed_base_len` / player-statics to the face-aware `face_of`, but these sites
still read `derive::face` (FRONT) where a battlefield back-up permanent should use
its back face:
- `sba.rs:419` (was 358) — legend-rule name grouping [CR#704.5j]: a back-up legendary
  transforming permanent is grouped by its FRONT name. (The legendary CHECK one line
  up already uses the layered view; only the name key is stale.)
- `target.rs:355-369` (was 279) — the `Named` and `NamedReg` predicates [CR#201]
  match the FRONT name.
- `target.rs:601` / `activate.rs:599` / `layer.rs:790` — loyalty base reads FRONT;
  matters only for a DFC whose back is a planeswalker (intertwined with
  enters-transformed and the `planeswalker-loyalty` work).

Fix: switch these to `face_of` (or the layered view) for a battlefield object. Keep
cast-time / off-battlefield / mana-value reads on FRONT ([CR#712.8a,712.8e]) — that
boundary is correct, not accidental. Prerequisite of `engine-day-night`.

## Added 2026-10-21

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #14.

- **Copiable values.** `crates/deckmaste_engine/src/copy.rs:102-115`
  `copiable_values` reads `derive::face` (front), and `CopiableValues` has no back
  face or side. [CR#707.8]: copy "the copiable values of the face that's currently
  up"; [CR#707.8a]: a token copy of a double-faced permanent "is a double-faced
  token" and enters with its back face up if the original's back face was up.
  Carry both faces plus the current side through the copy path (Cackling
  Counterpart on a transformed Delver makes a double-faced token, back face up).
- **`distinct_keys_derived`** (`layer.rs:1621`, Name at `:1649`) reads the front
  face for types, subtypes, basic land types and name.
- **Legend-rule grouping by name set.** Group legend-rule candidates by
  name-set intersection, not exact string ([CR#201.2a]: "at least one name in
  common"; [CR#704.5j]).
- **Check before fixing:** the layered view (`DerivedObject`, `layer.rs:131`) has no
  name field, so the legend key and `Named` may also ignore copy effects (a Clone
  copying a legend keyed by "Clone"). If so, route name reads through a layered
  name, which covers both the face and the copy cases.

Per [CR#712.8e] mana value stays on the front face's mana cost.
