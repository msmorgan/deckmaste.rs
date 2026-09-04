---
needs: []
---
**Ruling: where do the keyword gate columns live?** Residue of
`workbench-facts-residues` (2026-09-04, STOP 3). The seven hand-kept
columns of the workbench's `keywordFacts` (`counterEligible`, `regime`,
`onPermanentCard`, `onSpellCard`, `paidCost`, `bodied`, `wantsModes`) are
authored in xtask's overlay (`crates/xtask/src/facts.rs`) because
`plugins/builtin_v2/macros/meta/KeywordAbility.ron`'s `metadata` block
deserialises into `deckmaste_construction_core::macro_def::Metadata`, which
is `#[serde(deny_unknown_fields)]` over `spelling`/`grammar`/`noun_class`/
`category`, is `include_str!`-embedded in that crate, and is read by
`deckmaste_english_v2::environment`. Carrying the columns in the stubs
means widening that English-v2 seam and touching all 195 stubs.

Options: (a) leave the overlay as the columns' home and record that the
stub schema is English-facing only; (b) add an optional `facts` block to the
stub schema (ignored by the English seam) and move the columns there so the
generator derives every `keywordFacts` field. The user decides; the chosen
option is recorded in `docs/decisions/workbench-ron-shaped-and-label-rulings.md`.

Size: ruling (S to record; M to implement (b)).

Ruling 2026-09-04: deferred. The gate columns stay in xtask's overlay; the
expectation is that a keyword's facts derive from its macro definition once
the semantics-v2 keyword macros are written, so no stub-schema field is
added before then.
