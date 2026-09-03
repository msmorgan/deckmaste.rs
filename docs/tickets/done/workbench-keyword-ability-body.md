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

## As landed

- `KeywordAbility k param body` (`Effect.idr`): a third positional slot
  `body : Maybe (AbilityAt [])`, gated by `KeywordBodyFits k body`. Bodiless
  keywords are `Nothing` in that one slot — not a second constructor; this
  mirrors the existing `param : Maybe (KeywordParam bs)` slot and keeps one
  carrier, as `Enact` has one.
- The body is closed at `[]`, like `ParamCost : Cost []` and
  `ParamSubject : Predicate [] k`: the CR expansion is context-free, and at an
  abstract `bs` the trigger's own `anyTargetedAt`/`countReach` obligations
  cannot reduce.
- `keywordBodyFits` (Effect.idr): `Nothing` always fits; a `Just` fits only a
  `Triggered` body whose event regime equals the facts row's `regime`, on a
  keyword whose row is `bodied`. A non-`Triggered` body never fits.
- `KeywordFacts` gains a `bodied` column: True for the eight keywords the CR
  defines as triggered abilities with a quoted expansion (Ward, Storm, Renown,
  CumulativeUpkeep, Echo, Bushido, Annihilator, Afterlife). The row's role is
  unchanged — it is still the label gate; `keywordBodied` is its accessor.
- `Macros.renownExpansion` / `stormExpansion` / `cumulativeUpkeepExpansion`
  carry the CR text; `Macros.renown n`, `Macros.storm`, `Macros.cumulativeUpkeep c`
  are the one spelling per keyword (label + body). `Macros.keyword` and the six
  other `keyword*` macros expand through the same constructor with `Nothing`.
- The old effect-level `Macros.renown` is `Macros.becomesRenowned`; `renown`
  now takes `Nat` ([CR#702.112a]'s "Renown N"), which the param and the body
  both need.
- Bench re-spellings: Akroan Sergeant and Consul's Lieutenant through
  `Macros.renown 1` (Akroan Sergeant's hand-written expansion trigger is now
  the keyword's body); Amphibian Downpour and Prismari, the Inspiration through
  `Macros.storm`; fourteen cumulative-upkeep sites through
  `Macros.cumulativeUpkeep`, including Glacial Chasm, Polar Kraken and Heat Wave.
- Pins `ProofsG.badBodyOnBodilessKeyword` (a renown body under "Flying") and
  `ProofsG.badRenownWithStormExpansion` (a "Renown 1" label over storm's
  expansion). Both probed non-vacuous.
- Not done: Wall of Shards keeps the bodiless `keywordCosting` spelling — see
  the STOP below.

## Landing record

Numbers before/after:

- `KeywordAbility` explicit params 2 → 3; `AbilityAt` constructors 8 → 8.
- `KeywordFacts` columns 8 → 9, rows 87 → 87, `bodied = True` rows 0 → 8.
- `Cost` constructors 9 → 10 (`ScaledCost`); `MarkerWord` 2 → 4
  (`SpellMarker`, `PermanentMarker`).
- Macros: +8 definitions (`thisPermanent`, `thisSpell`, the three `*Expansion`s,
  the three keyword spellings), 1 renamed (`renown` → `becomesRenowned`).
- ProofsG pins +2.
- Bench spellings of the three keywords: 3 divergent shapes → 1 (18 sites; 1
  left behind, below).

Gates:

- `cd idris && ./scripts/build` (after `rm -rf build`) — `23/23: Building Cards
  (src/Cards.idr)`; 0 Error lines, 0 Warning lines.
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` — `checked 17734 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite audit --diff < /tmp/kb.diff` — `audited 4 citation site(s)`;
  each rule read against its claim. No `bless` run: all eight rules were already
  in `cr-citations.lock`.
- Pin probe: with the pins deliberately mis-stated (Renown over the renown
  expansion, Storm over the storm expansion) the compiler answers
  `misPin1 Oh is not a valid impossible case` / `misPin2 Oh is not a valid
  impossible case`; the landed pins compile. Positive twins
  (`KeywordAbility "Renown" … (Just (renownExpansion 1))`,
  `KeywordAbility "Storm" Nothing (Just stormExpansion)`) typecheck as terms.

Assurance counts: restored 0; re-spelled 18 bench sites plus 44 mechanical
`Nothing`-body additions at raw `KeywordAbility` sites in Cards and Proofs;
ignored 0; added 2 pins; removed 0.

### Deviations and additions

- **`Cost.ScaledCost c amt`** (Effect.idr, six delegating clauses).
  [CR#702.24a] is "pay [cost] for each age counter on it"; `ScaledMana` scales
  only a single mana unit, so the expansion was unspellable without it. Beyond
  the ticket's letter (the keyword carrier, its facts, macros and witnesses).
- **`MarkerWord.SpellMarker` / `PermanentMarker`** (Words.idr) with
  `Macros.thisSpell` / `thisPermanent`. `nounZone This = Nothing`, so `Casts`
  refused "this spell" (`OnStack Nothing`) and `sacrifice` refused "this
  permanent" (`OnBattlefield Nothing`); [CR#702.40a] and [CR#702.24a] both
  name them. `AsMarker` is the existing self-mention-with-zone family.
- **`KeywordFacts.bodied`**: a new column, not a new role. The ticket says the
  facts row stays the label gate; without a column there is nothing for the
  "body on a keyword whose row has none" pin to read.
- **The gate is one-directional.** A body is *permitted* where the row is
  `bodied`, never *required*: refusing `KeywordAbility "Renown" (Just …)
  Nothing` would refuse a sentence the CR makes perfectly meaningful (the
  printed card prints the label alone). The collapse to one spelling is carried
  by the macros.
- **`Macros.renown` takes `Nat`**, not `Amount []`. [CR#702.112a]'s N is a
  literal, and the same number is needed in the param and inside the body.
- **`cumulativeUpkeep` carries `Payable` / `CostPaidByYou` auto-implicits**,
  inherited from `Pay`.
- **Storm's body drops two clauses of [CR#702.40a]**: "before it" (`Lookback`
  has `ThisTurn` but no before-this-object window) and the "If the spell has any
  targets" guard (no such `Condition`; the bench's other copy cards spell the
  bare `may You (ChooseNewTargets …)` the same way). The count is "each other
  spell cast this turn", `OtherThan thisSpell`.
- **Cumulative upkeep's body names `thisPermanent` where the CR says "it"**
  twice. A self-mention introduces no antecedent, so `It` does not resolve
  there; the referent is identical.

### STOP

Wall of Shards ("Cumulative upkeep — an opponent gains 1 life") cannot take the
body. `Macros.cumulativeUpkeep` builds `Pay You …`, and
`costPaidByYou (Do (ChangeLife who _)) = nounIsYou who` refuses a cost whose
life-changer is an opponent — `So False`. Resolution: the card keeps its
`Macros.keywordCosting "CumulativeUpkeep"` spelling (label plus cost, no body),
which the one-directional gate still admits. The body was **not** weakened to
make it fit. Whether `costPaidByYou` should distinguish the payer from the
agent of the paid action is a separate ruling; it is the only bench card
affected.
