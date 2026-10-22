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

- Declare *To solve* as a `variants:` entry on `Solved`, with
  `parameters: [[DashCondition]]`, citing `[CR#719.3a]`. Then the label is a
  registered keyword run, and the Italic Head rule keeps it from being a Flavor
  Word.
- The glossary's **Keyword Variant** is "a keyword ability the Comprehensive
  Rules define as a variant of another". *To solve* is defined with Case cards
  [CR#719.3a], not as a variant of *Solved* [CR#702.169a]. Amend the entry, through the `domain-modeling`
  skill with a CR citation, to cover a label the catalog declares under its
  keyword. If that cannot be done without changing what the term means, STOP
  and return to the owner.
- Add judgments: the preterite and present witnesses above read with *To solve*
  as the keyword label over a Condition, and with no Flavor Word label.

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

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
