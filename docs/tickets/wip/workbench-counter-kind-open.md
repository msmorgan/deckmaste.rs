---
needs: []
---
**Open `CounterKind` to a `Named label` constructor with a facts row, and
delete the closed enum's index tables.** Cleanroom review 2026-09-03, F1. The
counter kind is an open label the corpus keeps extending; the workbench froze
it into an enum.

`Words.CounterKind` (`Words.idr:3304–3330`) has 25 variants plus
`KeywordCounter k` (gated by `KeywordCounterEligible`, `Words.idr:2535`).
Counter kinds with no variant, motivating the opening: energy, oil, quest,
finality, fade, spore, depletion, level, verse, ki, divinity, soul, page,
fuse, flood, bounty, tide, growth, doom, study, strike, dream, brick, plague,
slime, fungus, egg, wish, stash, scream. Thallid (VINTAGE): "At the beginning
of your upkeep, put a spore counter on this creature." Probe:

```
p = PutCounters (Lit 1) (PrintedKind Spore) Macros.thisCreature
Error: Undefined name Spore.          (the twin with Charge typechecks)
```

`Energy`, `Fade`, `Oil`, `Defense` refuse the same way (Aether Chaser,
Blastoderm, Archfiend of the Dross, Portent Tracker). The closed enum
contradicts the open-label ruling that `ActFacts`/`KeywordLabel` already
follow, `subtypes-are-labels`, and the builtin-v2 ADR's counter-kind
declarations.

## Fix

`CounterKind = BoostCounter d d | KeywordCounter k | Named (label : String)`,
with a `counterFacts` row table for the kinds that carry rules data —
`counterScope` (`Words.idr:3343–3345`: poison, rad, experience and energy are
player-held) becomes a column. `counterIx`, `Eq CounterKind` and
`counterScope` (`Words.idr:3333–3392`) are replaced by the row table.

Fold the `CounterKindSource.PrintedKind` consumers in the same pass so
`RemoveCountersAmong` / `MoveCounters` / `LosesCounters` (`Effect.idr:1214–
1232`) stop taking `Maybe CounterKind` while `PutCounters` / `RemoveCounters`
take `Maybe (CounterKindSource bs)`; `namedKind` (`Effect.idr:107`) exists
only to bridge the two and goes with it.

Bench migration: 118 `PrintedKind` sites, 85 of them through
`Macros.plusOnePlusOne` / `minusOneMinusOne` / `flyingCounter`, so ~33 sites
change spelling. Chromatic Armor's two sleight-counter lines
(`Cards.idr:5531`) are blocked on this ticket and should land with it.

Size: M.

Done when: the build is 23/23 with 0 errors and 0 warnings; `CounterKind` is
three constructors and the label form is open; `counterIx`, `Eq CounterKind`
and the standalone `counterScope` table are gone, replaced by one
`counterFacts` row per rules-bearing kind; Thallid's spore counter and at
least one each of energy / oil / fade are typechecking bench witnesses;
`namedKind` is gone and the counter-kind slot has one type across
`PutCounters`/`RemoveCounters`/`MoveCounters`/`RemoveCountersAmong`; every
existing counter pin still refutes for its named reason. Standard constraints
apply.

## As landed

- `Words.CounterKind` is three constructors: `BoostCounter d d`,
  `KeywordCounter k` (unchanged gate `KeywordCounterEligible`) and
  `Named (label : String) {auto 0 ok : KnownCounter label}` — the open-label
  form, fail-closed over the table exactly as `KeywordCounter` is over
  `keywordFacts`.
- New `record CounterFacts` (`counterLabel`, `counterHolder : Kind`) and
  `counterFacts`, 56 rows: the enum's 23 non-parametric kinds plus the
  ticket's census list plus `Defense` [CR#122.1g], `Hone` [CR#122.1j] and
  `Sleight`. `counterFactsIn` / `counterFactsFor` / `knownCounter` /
  `KnownCounter` mirror `keywordFactsIn` / … / `KnownKeyword`.
- Duplicate-label assertion: `distinctCounterLabels` + the checked proof
  `counterLabelsDistinct : So (distinctCounterLabels counterFacts) = Oh`.
- `counterIx` (25 clauses) is deleted; `counterScope` is 3 clauses reading
  `counterHolder` off the row; `Eq CounterKind` is 3 direct clauses (kept —
  `Eq ProjAxis`'s `CounterAxis` arm and `predEq` consume it).
- `Effect.namedKind` is gone. `RemoveCountersAmong`, `MoveCounters` and
  `LosesCounters` take `Maybe (CounterKindSource bs)` under
  `OptCounterSourceScope`, so the counter-kind slot has one type across
  `PutCounters` / `RemoveCounters` / `MoveCounters` / `RemoveCountersAmong` /
  `LosesCounters`. `Macros.removeCounters` / `removeAllCounters` /
  `losesAllCounters` take the source directly.
- Witnesses: `thallid` (full card — spore counter put at upkeep, three
  removed as an activated cost), `aetherChaserEnergy` (energy, player-held),
  `blastodermEntersFading` + `blastodermFadeUpkeep` (fade),
  `archfiendOfTheDrossEntersOiled` + `archfiendOfTheDrossOilUpkeep` (oil).
  `chromaticArmor` gained its two sleight-counter lines: the enters rider and
  the `{X}` activated ability with `Define X (CountersOn (Named "Sleight") …)`.
- New pins (ProofsG): `badUnknownCounterLabel` ("Put a zorp counter on target
  creature") and `badKeywordCounterNamedPlainly` ("Put a flying counter on
  target creature", [CR#122.1b] — flying is a keyword counter, so it is
  spelled `KeywordCounter "Flying"`).
- Undone: nothing in the letter.

## Landing record

Numbers before/after: `CounterKind` constructors 26 → 3; `counterScope`
clauses 25 → 3; `counterIx` 25 clauses → 0; `counterFacts` rows 0 → 56;
spellable non-boost non-keyword counter labels 23 → 56; `namedKind` 1 → 0.
`Words.idr` cold `idris2 --check` 2.6s → 3.3s; full cold `./scripts/build`
43.1s → 42.8s.

Gates: `cd idris && ./scripts/build` → `23/23: Building Cards
(src/Cards.idr)`, 0 Error and 0 Warning lines. `cargo xtask cite check
--list-noncompliant` → `0 non-compliant citation-looking string(s)`. `cargo
xtask cite check` → `checked 17743 citations against cr.txt (eff. 2026-08-07);
0 stale`. `jj --no-pager diff --git | cargo xtask cite audit --diff` →
`audited 8 citation site(s)`; each read against the rule text ([CR#122.1] is
the object-or-player claim; [CR#122.1b] lists flying among the keyword-counter
keywords). No `cite bless` was needed — both cited rules were already in
`cr-citations.lock`, which is unchanged.

Assurance: restored 0; re-spelled 8 pins (`badPutPoisonOnCreature`,
`badLosesAllBoostCounters`, `badLastPoisonCounterRemoved`,
`badExileCheckOnSortedSelf`, `badAnnouncingRemovalAgent`,
`badGetsChargeCounter`, `badPoisonCounterDescription`,
`badMixedScopeCounterMenu`); ignored 0; added 2 pins and 1 table proof
(`counterLabelsDistinct`) and 6 witnesses; removed 0. Every one of the ten
pins was probed by mis-stating it once and watching `… is not a valid
impossible case` appear; `counterLabelsDistinct` was probed by duplicating the
"Spore" row (`Mismatch between: True and False`). Positive twins
`PutCounters (Lit 1) (PrintedKind (Named "Spore")) (Macros.target
Macros.creature)` and the same with `KeywordCounter "Flying"` both typecheck
in a scratch module, since deleted.

Deviations and additions:

- `Eq CounterKind` is retained (3 direct clauses) rather than deleted as the
  ticket's "Done when" reads: `Words.ProjAxis`'s `Eq` and `Phrase.predEq`
  compare `CounterAxis` relata. The index table behind it (`counterIx`) is
  gone, which is what the fix paragraph asked for.
- `counterFacts` carries one column, not a keyword flag. [CR#122.1b] makes a
  keyword counter a distinct shape, already the sealed `KeywordCounter`
  constructor gated over `keywordFacts.counterEligible`, so an `isKeyword`
  column on `Named` rows would be constantly False.
- `Words.CounterKindNamed` stays. `Phrase.HasCounters` and
  `Triggers.CounterEvent` sit upstream of `Effect`, where `CounterKindSource`
  lives, so they keep the `Maybe CounterKind` slot; the ticket named only the
  Effect rows.
- The table holds 56 labels, not every kind in the corpus: the corpus tail
  runs to several hundred (flame, fate, pressure, point, …). Adding one is a
  one-line row, which is the point of the shape.
- Aether Chaser, Blastoderm and Archfiend of the Dross landed as
  `Ability`/`StaticEffect` witnesses named by doc comment, not full
  `Macros.card` entries. Blastoderm's printed body is "Shroud / Fading 3" and
  `Fading` has no `keywordFacts` row, so a full card would have asserted
  reminder text as the printed lines.
- The `Named` label for the old `LoyaltyCounter` constructor is "Loyalty";
  the constructor's suffix existed only to dodge a name clash.

STOP: none taken.
