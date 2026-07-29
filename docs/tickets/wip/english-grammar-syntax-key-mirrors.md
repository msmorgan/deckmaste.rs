---
needs: [english-structural-recovery-zero]
---
**Collapse the grammar-side `*Key` mirror hierarchy onto the syntax types it
shadows.** Adjudicated from an external review 2026-07-24, spot-verified
against the code (line cites as of that date).

- `QuantityKey` (grammar/mod.rs:573), `DeterminerKey` (:682),
  `IndefiniteArticleKey`, `DemonstrativeKey`, and `FrequencyKey` each shadow
  a syntax type, bridged by hand-written `.syntax()` conversions. Two
  semantic tables exist twice: `QuantityKey::cardinality` vs
  `Quantity::noun_cardinality`, and `DeterminerKey::cardinality` vs
  `Determiner::noun_cardinality` — the syntax side admits it
  (phrase.rs:226: "Mirrors `DeterminerKey::cardinality` in the grammar").
  A new determiner touches four or five sites nothing forces into sync.
  Either key the grammar directly on the syntax types or derive the keys;
  exactly one cardinality table survives.
- Rider: `OpacityMode` (grammar/mod.rs:331) and `OpacityProfile` (:343) are
  identical two-variant enums twelve lines apart bridged by `.mode()` —
  merge them.

Sequenced after the recovery campaign (the `needs:` gate) because the
campaign appends to `grammar/mod.rs` every round. Distinct from
[[english-ast-grouping]] (syntax-side struct dedup): this is the
grammar/syntax boundary itself.

Gate: this is representation-only — `cargo xtask english recovery` census
byte-identical in every cell and clean roundtrip, before vs after. Standard
constraints apply.
