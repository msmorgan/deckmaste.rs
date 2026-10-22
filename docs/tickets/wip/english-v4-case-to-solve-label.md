---
needs: []
---
# A Case's *To solve —* is a keyword label over a Condition, never a Flavor Word

Split from `english-v4-case-solve-condition` (retired by the owner's round-8
treatment plan). This is its catalog half, which is real coverage.

## Owner's ruling (round 7, item 8)

"To solve DashCondition variant on "Solved" in keyword_abilities.ron". The
other half of that ruling, excluding the preterite on a matrix Declarative,
is retired: it is an admission veto, which round 8 freezes
(`docs/decisions/english-v4.md` §4.4).

## Owner's ruling (round 8, 2026-10-22)

*To solve* is its own catalog entry, not a variant of Solved:
`(rule: "[CR#719.3a]", name: "To Solve", lemma: "to solve", parameters: [[DashCondition]])`.
[CR#719.3] calls it one of the Case's "two special keyword abilities", and the
CR never defines it as a variant of Solved [CR#702.169a], so it is no **Keyword
Variant** and the glossary entry stays as it is. This supersedes the
`variants:` half of the round-7 ruling and the glossary amendment below.

## Evidence

Measured on `qynmxmro` with `cargo xtask english-v4 parse`:

- *To solve — Three or more creatures attacked this turn.* has one Reading:
  `FlavorLabelledAbility` over a `SentenceAbility`, with a preterite
  `SubjectPredicate`.
- *To solve — You control three or more artifacts.* has the same shape, with a
  present Clause.

A Flavor Word has no rules meaning [CR#207.2d]. *To solve — [Condition]* has
one: "At the beginning of your end step, if [condition] and this Case is not
solved, this Case becomes solved" [CR#719.3a]. `docs/oracle-style-guide.md` §15,
"Cases", treats it as a fixed label that "states the solve condition", and §14,
"Ability words and flavor words", says a flavor word has no rules meaning. So
every Reading of every *To solve* line is wrong. Those faces count as read only
because no judgment says otherwise. Fifteen Oracle records contain *To
solve —*. The one Case judgment, `existential_there.txt:5` (Case of the
Shifting Visage), asserts only the Clause.

In `crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron`, `Solved`
(`[CR#702.169]`, `parameters: [[DashAbility]]`) has no variant. `Companion`
shows the `DashCondition` parameter (`parameters: [[DashCondition]]`), and
`Banding` shows the `variants:` form (`(lemma: …, parameters: …)`).

## Change

- Declare *To solve* as its own keyword ability in `keyword_abilities.ron`,
  after the [CR#702] entries, with `parameters: [[DashCondition]]`, citing
  [CR#719.3a] (round-8 ruling above). Then the label is a registered keyword
  run, and the Italic Head rule keeps it from being a Flavor Word.
- `tests/cr_coverage.rs` checks keyword abilities against the [CR#702]
  headings; it also checks each keyword defined outside them against its rule's
  text.
- The glossary's **Keyword Variant** is unchanged: *To solve* is no variant.
- Add judgments: the preterite and present witnesses above read with *To solve*
  as the keyword label over a Condition, and with no Flavor Word label
  (`judgments/italic_heads.txt`).

## Done when

- Every *To solve —* line reads with the keyword label over its Condition, and
  none reads as a Flavor Word.
- No face is lost.
- Every judgment holds.
- `cargo xtask english-v4 strata --check` passes.
- The landing reports the cheapest tier: ambiguous units with a tier of one,
  the tiers' sum and the largest tier, before and after. No judgment-set entry
  goes from an all-endorsed cheapest tier to one that is not.
- No new `Summary`, `State` or `ProductionDecl` field, and no new `closure.rs`
  pass.

Standard constraints apply.

Census delta (measured on `rponomnp` → `nwsmxmnk`): faces read 28,800 → 28,800, product 213,916 → 213,916, Surplus 88,285 → 88,285, cheapest tier singleton 14,798/16,558 → 14,798/16,558 (summed 18,913 → 18,913, largest 8 → 8); the 13 *To solve* units change only from a Flavor Word label to the keyword, with the same counts; lost faces: none.
Judgment set: 3,039 enforced, 208 pending, all holding; 2 entries added, none retired.
