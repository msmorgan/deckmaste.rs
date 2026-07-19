---
needs: [engine-transform]
---
Route the remaining current-characteristic reads through the CURRENT face for a
back-up double-faced permanent — surfaced by the `engine-transform` whole-branch
review. Latent: no back-up permanent is reachable until `engine-day-night` or a
canon transform card lands.

`engine-transform` converted `base_values` / `abilities_of_source` /
`printed_base_len` / player-statics to the face-aware `face_of`, but these sites
still read `derive::face` (FRONT) where a battlefield back-up permanent should use
its back face:
- `sba.rs:358` — legend-rule name grouping [CR#704.5j]: a back-up legendary
  transforming permanent is grouped by its FRONT name. (The legendary CHECK one line
  up already uses the layered view; only the name key is stale.)
- `target.rs:279` — the "named [X]" predicate [CR#201] matches the FRONT name.
- `target.rs:601` / `activate.rs:599` / `layer.rs:790` — loyalty base reads FRONT;
  matters only for a DFC whose back is a planeswalker (intertwined with
  enters-transformed and the `planeswalker-loyalty` work).

Fix: switch these to `face_of` (or the layered view) for a battlefield object. Keep
cast-time / off-battlefield / mana-value reads on FRONT ([CR#712.8a,712.8e]) — that
boundary is correct, not accidental. Prerequisite of `engine-day-night`.
