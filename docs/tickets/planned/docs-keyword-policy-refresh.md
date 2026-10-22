---
needs: []
---
`docs/keyword-policy.md` has drifted from the code it describes:

- §6 documents a plugin-load enforcement step (`keyword_shape` matching each
  `KeywordAbility` macro's param-type multiset against seven signatures, load
  error on mismatch, a `keywords: HashMap` registry on the plugin) that no
  longer exists in `crates/deckmaste_core/src/plugin.rs`. The `ParamShape` /
  `KeywordDecl` types survive as orphans — re-exported from `lib.rs`,
  exercised by one round-trip test, read by nothing else — and §6's "only
  reads are one test and a clone-forward" line is stale too (no clone-forward
  remains). Decide: reinstate the load-time shape check, or delete the orphan
  types and rewrite §6 to describe current reality.
- §2's three-class table sums to 244 under a "keyword abilities" heading; it
  only reconciles with rules-taxonomy's 192 abilities + 68 actions = 260 if
  the "Composite: 215" row silently spans abilities *and* actions. Label the
  rows so the counts read consistently.
- §11's keyword-action table misfiles four actions. Confirmed at head as of
  2026-10-21 (provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-docs.md`
  entries 12–15). In each case the Lean model is already right, so only the
  table changes.
  - **Detain** (line 260, under **GetDesignation**). Detain creates a
    restriction and no designation: until its controller's next turn, the
    permanent can't attack or block and its activated abilities can't be
    activated [CR#701.35a]. Goaded and monstrous are designations
    [CR#701.15b,701.37b]; detain is not one. Move it to the **continuous
    restriction** row beside exert.
  - **Double** (line 266, under **continuous P/T** only). Doubling has five
    forms. P/T is a continuous effect [CR#701.10a]. Life total, counters and
    mana are one-shot [CR#701.10d,701.10e,701.10f]. Damage is a replacement
    effect [CR#701.10g]. Split the row across those forms. Triple is P/T only
    [CR#701.11a] and stays in this row. In §12 (line 281), **RemoveDamage**
    is anchored on [CR#614.8], which is regeneration. Re-anchor it on
    [CR#701.69a,120.6,514.2].
  - **Meld** (line 269, "(merged objects)"). Only mutate merges [CR#730.1].
    Meld makes a single object represented by two cards [CR#701.42a,712.4a,712.21].
    Relabel the entry as a multi-card object (a melded permanent). The merge
    machinery [CR#730.2b] belongs to mutate alone. The game-model glossary
    already says "Only mutate merges".
  - **Venture** (line 270, under **variant-format machinery**). Venture works
    in ordinary games: dungeon cards are brought into the game by venturing
    [CR#309.2,701.49a]. Give it its own row (dungeon machinery). Only open an
    Attraction needs an Attraction deck [CR#701.51a]. Roll to visit is
    ungated [CR#701.52a].
