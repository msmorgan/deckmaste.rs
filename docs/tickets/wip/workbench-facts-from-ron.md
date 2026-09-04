---
needs: []
---
**Derive the facts-table rows from the RON stubs, make the parameter column a
per-row list of admitted shapes, and check the label sets match both ways.**
Fresh workbench review 2026-09-03, F3 + F7 + F16, resolved by ruling.

**Ruling (settled 2026-09-03): open string labels over facts tables stay;
enums are declined.** The rows are exactly the labels the Comprehensive Rules
define — keyword abilities [CR#702], keyword actions [CR#701] — plus the
counter kinds the CR names and the printed ones the corpus attests. A label
outside the tables is refused. Rows are **derived from the RON stubs**
(`plugins/builtin_v2/macros/stubs/{keyword_abilities,keyword_actions,counter_kinds,designations}/*.ron`;
fields `params`, `spelling`, `grammar`, `body`) wherever the stub determines
the column — `bodied`, `paidCost`, `regime`, and the admitted param shapes as
a **list** read off `params` — and hand-written only where a column exists
solely for an Idris gate. Custom (non-CR, non-corpus) label sets are a later
bridge, not this ticket.

**Per-row shape lists (F3).** `Words.KeywordFacts.paramShape` is one
`KeywordParamShape` and `Words.paramShapeFits` admits only that shape (plus
`CompoundParam h → CostParam`), so CR-defined variants are refused: P3
`keywordQuality "Hexproof" (ColorIs Black)` (Knight of Grace, "hexproof from
black" [CR#702.11d], 19 cards) → `So False`; P4 `keywordQualityCosting
"Cycling" (HasSubtype Plains) {2}` (Eternal Dragon, "plainscycling {2}"
[CR#702.29e], 88 cards) → `So False`; P4b the structurally identical Equip
compound is admitted. Fix: `paramShapes : List KeywordParamShape` (Hexproof
`[NoParam, QualityParam]`, Cycling `[CostParam, CompoundParam QualityHead]`,
landwalk-style rows unchanged) and `paramShapeFits` an `elem`.

**Column drift (F7).** `MkActFacts` takes 15 positional fields,
`MkKeywordFacts` 9 booleans, `Events.MkEventFacts` 8. Generated rows are
named-field record updates over a default row, so the F3 column addition is
local and a new column does not renumber every row by hand.

**Unknown labels (F16 and P3's message).** An unrecognised label fails today
as a bare `So False`. Give the membership gates a named witness
(`KnownKeyword`/`KnownAct`/`KnownCounter`-style) so the refusal names the
label and the table. Add the missing rows while there: `keywordFacts` for
Delve, Infect, Cascade, Prowess, Split second and Phasing, and for Decayed,
Exalted and Shadow with `counterEligible = True` [CR#122.1b]; `actFacts` for
Venture.

The check is an `xtask` subcommand run in both directions: every stub label
has a table row, and every table row has a stub. It fails on a label present
on one side only. The Idris keyword table carries ≈90 rows against 195
keyword-ability stubs and 70 keyword-action stubs, so the first run is a
worklist, not a green gate — land it with the difference recorded in the
landing record.

Size: S–M for the shape list and the rows; M for the generator and check.

Done when: Knight of Grace and Eternal Dragon are typechecking bench
witnesses; `paramShapeFits` is an `elem` over a per-row list; the facts rows
are generated from the stubs and the generated file is regenerable from a
clean tree; the label-set check passes in both directions (or names its
remaining difference and the ticket records it); an unknown keyword is refused
by a named witness and pinned, probed non-vacuous; P6's `KeywordCounter
"Ward"`/`"Shroud"` refusals still hold [CR#122.1b]; the build is 44/44 with 0
errors and 0 warnings. Standard constraints apply, plus the RON-shaped
constraint: a core constructor is admissible only if the RON re-emitter can
produce it from a RON node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

**Generation decision: generated, for `keywordFacts` only.** `cargo xtask facts
generate` writes `idris/src/Experimental/FactsGen.idr` — one header line and
96 rows, each a named-field record update over `defaultKeywordFacts` (F7), so
a new column never renumbers a row. It is committed and regenerable from a
clean tree; `cargo xtask facts check` regenerates in memory and fails on any
byte difference, so the file cannot be hand-edited into drift.

The generated module carries data only. The types, the `KeywordFacts` record
(with its `bodied` CR doc), `paramShapeFits` and the new `paramShapesFit` live
beside it in a hand-written `idris/src/Experimental/KeywordShapes.idr`, which
`FactsGen` imports and `Words` re-exports. Module count 44 → 46 (the brief
budgeted one new module; two keeps generated code free of hand-written prose
and gate logic — recorded as a deviation).

**Derived vs hand-kept columns.** Only `paramShapes` is derived: the stub's
`params` signature maps to one shape (`[Cost]` → `CostParam`, `[Amount, Cost]`
→ `CompoundParam NumberHead`, `[Quality, Cost]` → `CompoundParam
QualityHead`, absent → `NoParam`). Every other column is an xtask-side
overlay, because the `keyword_abilities` stubs carry only `name`, `spelling`,
`grammar` and `params` — no stub in the tree has a `body` field, and
`meta/KeywordAbility.ron` declares no field that could carry `bodied`,
`paidCost` or `regime`. Moving those into the stubs needs a stub-schema
change; recorded as residue.

The overlay also carries `extra`, the further shapes the CR admits for the
same keyword: Hexproof `[NoParam, QualityParam]` [CR#702.11d], Cycling and
Equip `[CostParam, CompoundParam QualityHead]` [CR#702.29e], and the three
rows whose stub is missing. Across the 87 pre-existing rows the stub-derived
shape disagreed with the hand-written one exactly once (Equip), so the union
narrows nothing.

**F3.** `KeywordFacts.paramShape : KeywordParamShape` became `paramShapes :
List KeywordParamShape`; `paramShapesFit` is `any paramShapeFits` over that
list and `Effect.keywordParamFits` calls it. `keywordParamShape` became
`keywordParamShapes`; `keywordCosts`, `keywordParamless` and
`keywordFamilyOk` are re-spelled over the list.

**F16.** `KnownKeyword`, `KnownKeywordTerm`, `KnownAct` and `KnownCounter` are
now `data` types (`KeywordInFactsTable`, `KeywordTermInFactsTable`,
`ActInFactsTable`, `CounterInFactsTable`) carrying an erased `knownX v = True`
proof, so an unknown label is refused by name and table rather than by a bare
`So False`:

```
Can't find an implementation for KnownKeyword "Zorp".
Can't find an implementation for KnownCounter "Zorp".
Can't find an implementation for KnownAct "Zorp".
Can't find an implementation for KnownKeywordTerm (TheKeyword "Zorp").
```

**The check.** `cargo xtask facts labels` reports the two-way difference for
three tables and exits 1 on any difference. Its first run, recorded as the
ticket asked:

```
keyword abilities: 195 stubs, 96 rows, 102 stubs without a row, 3 rows without a stub
keyword actions:   70 stubs, 40 rows, 47 stubs without a row, 17 rows without a stub
counter kinds:     29 stubs, 71 rows,  0 stubs without a row, 42 rows without a stub
```

**New rows.** `keywordFacts` gained Delve, Infect, Cascade, Prowess,
SplitSecond, Phasing, Decayed, Exalted and Shadow — the last three with
`counterEligible := True` [CR#122.1b], whose keyword-counter list is exactly
the twelve rows already eligible plus those three. `actFacts` gained
`"Venture Into The Dungeon"` (spaced like the existing `"The Ring Tempts You"`
row so the label check matches its `VentureIntoTheDungeon` stub), modelled on
that row: every column at its default.

**Bench witnesses.** `Cards.Keyword.knightOfGraceHexproofFromBlack`
(`keywordQuality "Hexproof" (ColorIs Black)`) and
`eternalDragonPlainscycling` (`keywordQualityCosting "Cycling" (HasSubtype
(landType "Plains")) {2}`) typecheck — P3 and P4 were `So False` before.
`ProofsKeyword` gained `badUnknownKeywordClass` (refused by the named witness)
with `okKnownKeywordClass` as its non-vacuity twin.

**Residues** (none in scope for this ticket):
- 102 keyword-ability stubs still have no row; each needs its gate columns
  authored by hand until the stub schema carries them.
- `bodied` / `paidCost` / `regime` / `onPermanentCard` / `onSpellCard` /
  `wantsModes` / `counterEligible` remain hand-kept in xtask's overlay:
  `meta/KeywordAbility.ron` has no field for them.
- BandsWithOther, Multikicker and PartnerWith have no stub; their shapes come
  from the overlay alone. Adding stubs was out of scope.
- 17 `actFacts` rows have no keyword-action stub (Attack, Block, Copy, Draw,
  Put, Return, Target, Trigger, Cast, Play, Counter, Activate, GainControl,
  GainLife, LoseGame, WinGame, Spend, Crew, Saddle, Unlock, Fully Unlock,
  Regenerate, Vote — the turn-and-game deed vocabulary, which is wider than
  [CR#701]). The row→stub direction is not a defect for that table and the
  check's failure there should be re-scoped, not silenced.
- 42 `counterFacts` rows have no counter-kind stub (the corpus-attested
  printed counters).
- Designations are not covered by `facts labels`: the 19 stub names
  (Commander, Initiative, DayNight, Level, Sector, Solved, Harnessed …) are
  not the Idris `Designation` constructor names, so the mapping is its own
  decision.

## Landing record

Numbers before → after: `keywordFacts` 87 → 96 rows; `actFacts` 39 → 40;
`counterFacts` 59 (unchanged); Idris modules 44 → 46. Coverage lock
untouched; no construction added or retired.

Gates (all foreground):

- `cargo check --workspace` — `Finished \`dev\` profile [unoptimized +
  debuginfo] target(s) in 14.84s`
- `cargo fmt` — clean; `cargo clippy -p xtask --all-targets` — 0 warnings
  (two `#[expect(…, reason = …)]` added: `struct_excessive_bools` and
  `too_many_lines` on the overlay data table)
- `cargo test -p xtask` — `test result: ok. 436 passed; 0 failed; 1 ignored`
  (plus the 12/1/1 binary and integration suites), including 6 new
  `facts::tests::*`
- `cargo xtask facts check` — `…/idris/src/Experimental/FactsGen.idr is up to
  date`
- `cargo xtask facts labels` — `Error: the stub and table label sets differ`
  (exit 1, the recorded worklist above)
- `cd idris && ./scripts/build` — clean build, `46/46: Building Cards
  (src/Cards.idr)`, 0 errors, 0 Warning lines, `real 1m11.602s`
- `cargo xtask cite check --list-noncompliant` — 1 hit, pre-existing and
  outside this diff (`docs/tickets/done/workbench-turn-parts.md:59`, a quoted
  bare rule number in prose); 0 in the diff
- `cargo xtask cite check` — `checked 18264 citations against cr.txt
  (eff. 2026-08-07); 0 stale`
- `cargo xtask cite bless` — `blessed 1587 rules`; three rules newly
  registered in `cr-citations.lock` (702.85a, 702.147a, 702.28a), each text
  read against the row citing it
- `cargo xtask cite audit --diff` — 16 sites audited, each rule read against
  its claim ([CR#702.11d] hexproof-from-[quality], [CR#702.29e] typecycling,
  [CR#702] the keyword-ability section, [CR#701] the keyword-action section,
  [CR#122.1b] the keyword-counter list, the three new 702 entries, and the
  unchanged `bodied` list)

Timings: `idris2 --check Experimental/Words.idr` 3.289s before → 3.631s after
(the after figure builds `KeywordShapes` and `FactsGen` too); clean full
build 1m11.6s.

Pin probes (foreground, ad hoc modules typechecked and removed):

- `KeywordCounter "Ward"` and `KeywordCounter "Shroud"` still refuse
  (`Can't find an implementation for So False`) while `KeywordCounter
  "Decayed"` now typechecks — P6 holds and the new eligible rows are
  non-vacuous [CR#122.1b].
- The four unknown-label refusals name their label and table (quoted above).
- Every `Proofs*` module re-elaborated in the clean 46/46 build.

Assurance counts: restored 0; re-spelled 6 (`badUnknownKeywordPredicate`,
`badUnknownCounterLabel`, `badUnknownVerbLabel`,
`badKeywordCounterNamedPlainly`, `badClassOfParamlessKeyword`,
`badSortedClassOnNumberKeyword` — each `Oh impossible` re-spelled to the named
witness constructor, same card, same refusal); ignored with blockers 0; added
10 (2 bench witnesses, 1 pin, 1 positive twin, 6 xtask unit tests); removed 0.

Deviations and additions:

- **Two new Idris modules, not one.** `Experimental.KeywordShapes`
  (hand-written) and `Experimental.FactsGen` (generated). Keeping the record's
  `bodied` doc comment and `paramShapeFits` out of generated code needs the
  split; both are listed in `mtg.ipkg` and `mtg-dev.ipkg`.
- **Only `keywordFacts` is generated.** `actFacts`, `counterFacts` and
  `designationFacts` stay hand-written: no stub determines any of their
  columns. The `keyword_actions` stubs carry no `params` at all, and their
  `participle` (5 stubs) and `frame_set: Intransitive` (14 stubs) do not mean
  what Idris's `participle` (7 rows) and `actIntransitive` (Transform,
  Convert) mean. Generating them would relocate hand data into Rust without
  deriving anything.
- **The witnesses carry `knownX v = True`, not `So (knownX v)`.** With an
  `{auto 0 ok : So (knownAct v)}` field the inner proof search picked up
  unrelated `So True` locals in `Macros.idr` ("Multiple solutions found"); a
  `default Oh` field is elaborated against the abstract label and fails at the
  declaration. The Bool equality searches unambiguously.
- **`paramShapeFits` is kept**, unchanged, and joined by `paramShapesFit =
  any paramShapeFits`, so the `CompoundParam h` / `CostParam` optional-head
  rule still holds inside the list.
- **Hexproof's family widened.** Adding `QualityParam` to its row makes
  `keywordFamilyOk (MkKeywordFamily "Hexproof" (Just sort))` true where it was
  false. That is the CR's own hexproof-from-[quality] family [CR#702.11d]; no
  pin refused it, and the build is green.
- **`Multikicker`, `BandsWithOther`, `PartnerWith`** keep their current shapes
  from the overlay's `extra` because they have no stub. Adding stubs was
  explicitly out of scope.
- **`facts labels` is a separate subcommand from `facts check`**, so the
  byte-identity gate stays green while the label worklist reports its
  difference and exits 1, as the ticket anticipated.
- **Three CR rules cited for the first time** ([CR#702.85a], [CR#702.147a],
  [CR#702.28a]). Their code sites are in `crates/xtask/`, which
  `cite-config.json` excludes from scanning, so they entered the lock via this
  record; `cargo xtask cite bless` registered all three and their texts were
  read against the rows citing them.

STOP taken: none.
