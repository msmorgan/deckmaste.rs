---
needs: []
---
**Give `KeywordAbility` a body slot mirroring `Enact`, and spell the three
divergent bench keywords one way.** Cleanroom review 2026-09-03, R6, ruled.

`KeywordAbility k param` (`Effect.idr:2460`) has no body slot, so the "label +
expanded body" ruling — which names keyword actions *and* abilities — has a
carrier only for actions (`Enact`). The bench consequently does three
different things on one file: Renown body-only (Akroan Sergeant
`Cards.idr:6395`, the trigger through `Macros.renown`, no label), Storm
label-only (Amphibian Downpour `Cards.idr:3383`, `Macros.keyword "Storm"`, no
copy body), Cumulative upkeep label-plus-cost-only (Glacial Chasm
`Cards.idr:8463`, Polar Kraken `Cards.idr:8499`, Heat Wave
`Cards.idr:14433`).

## The ruling

`KeywordAbility k param body`, mirroring `Enact`. The **body is the CR
expansion** — [CR#702.112a] for renown, [CR#702.40a] for storm,
[CR#702.24a] for cumulative upkeep — and the **label is what the card
prints**. The facts row stays exactly as it is: the label gate. The three
bench spellings collapse into that one shape.

Rule numbers come from `data/rules/cr.txt`, not from memory; read each
expansion against the constructor it is spelled into before blessing the
citations.

Size: M.

Done when: the build is 23/23 with 0 errors and 0 warnings; `KeywordAbility`
carries a positional body slot (no default) and the facts row is unchanged in
role; Akroan Sergeant, Amphibian Downpour, Glacial Chasm, Polar Kraken and
Heat Wave all spell label plus body and typecheck; `Macros.renown` and
`Macros.keyword` expand through the one shape; the three CR citations are
blessed and audited against the rule text; a pin refutes a body that does not
match its keyword's facts row, and it is non-vacuous. Standard constraints
apply.
