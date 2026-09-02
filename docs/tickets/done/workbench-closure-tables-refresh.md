# workbench-closure-tables-refresh

`docs/idris-workbench-closure-tables.md` is stale after the choice split
(B–E, done 2026-08-27): §2.1's `Words.idr:1501`/`1866` anchors are dead
(the subtype-labels conversion removed the per-word census), §2.5 predates
the split and still lists `SetsChosenBasicType` (folded by choice-B). Moved
or added rows to reflect, named in choice-D's As-landed section:
`BecomesAlso`'s widening column and distinctness gate (`TokenCanonical`),
the chosen-quality rows' dropped zone demand, `TokenRider`'s `StatusVal`
catalog, and new rows `SetsColor`, `LosesEveryType`, `ColorSpec`. A
doc-refresh pass against the current grammar; verify each row against the
code, not the old table.

## As landed

Scope: `docs/idris-workbench-closure-tables.md` header, §2.1 and §2.5 only.
Every other section is untouched and keeps the 2026-08-22 demotion notice,
which the header now scopes to them explicitly.

**Counts.** §2.1 holds 167 rows (unchanged — no row was dropped), §2.5 holds
99 (96 + 3 new). 150 of §2.1's rows and 93 of §2.5's now carry a
`<File>.idr:<line>` anchor; every one of those 263 anchors was re-pointed from
the dead `Experimental.idr:NNNN` / stale `Words.idr:NNNN` form and verified by
script to land on a line naming its own declaration. No `Experimental.idr:`
anchor remains in either section.

**Rows with no live site.** 15 in §2.1 and 3 in §2.5 are marked
`site not found 2026-09-02` and kept (`boundedIncrease`, `modalHead`,
`AtLeastOne`, `arrangementFits`, `ordinalFits`, `searchableZone`,
`NextUntapCount`, `PhaseCount`, `JoinedPlayer`, `JoinedClass`, `counterZone`,
`CounterHolder`, `loyaltyStepWritten`/`LoyaltyStep`, `typeRank`/`typesOrdered`,
`SpanUse`; `NotLoyalty`, `freedomFits`/`FreedomFits`,
`repeatCount`/`RepeatCount`). 9 more in §2.1 and 6 in §2.5 name their
successor instead: `verbAgentive` and `sameKeyword` into the fact tables,
`ascribesAsSubtype` retired with the subtype-labels conversion,
`OutcomeGateKind`/`PlayerAct`/`ObjectAct`/`riderAct` into
`Events.deedFacts` (Events.idr:885), `ExtraTurnCount` into `ExtraTurn`'s
`Amount`, `SkipCount` with `SkipsNext`, `CapBound` into `CantMoreThan`'s
`k : Nat`, `WhereLetterStatic` and `notLetterRider` deleted by the letters
round, and `PlayerCant`/`ObjectCant`/`OutcomeGate` folded into `Deontic`.

**Choice-split reconciliation.**

- `SetsChosenBasicType` — constructor confirmed gone; the row now records the
  choice-B fold into `AddsChosenQuality`/`SetsChosenQuality` over
  `QualitySort`'s host-carrying `SubtypeQ`, with the domain on
  `OfYourChoice`'s `ChoiceDomain` slot (Phrase.idr:458).
- New rows added, in file order: `LosesEveryType` (Effect.idr:708),
  `SetsColor` (Effect.idr:763), `ColorSpec` (Words.idr:5695, with
  `colorSpecOk`).
- `BecomesAlso` — `TokenCanonical` (Effect.idr:223) recorded as the
  distinctness gate standing where `ColorsDistinct` did, and the Widening cell
  corrected: the colour-only addition it called unmodelled now writes, through
  `AdditionSaysSomething` (Effect.idr:258).
- Chosen-quality rows — the dropped subject-zone demand recorded on both, with
  the docstring's reason and its recorded overgeneration.
- `TokenRider` (§2.1) — the `StatusVal`-indexed `EntersAs` arm recorded, with
  `EntersTransformed`/`EntersMelded` as new non-status rows and `EntersTapped`
  demoted to an alias.
- Keyword-2 — `attachHeadOk`'s Enchanted catch-all note kept and the stale §7
  `Equipped`×`PermanentW` discrepancy called out as stale in the row.

**Numbers.** 47 §2.1 rows and 49 §2.5 rows carry
`corpus counts unsourced (pre-split docstring, not re-measured)`: the split
rewrote or dropped most docstring censuses, and no current source backs them.
9 rows take a count the current docstring states with its own measurement date
(`AltCost`, `LosesType`, `DoesntRemove`, `CantMoreThan`, `SpendPurpose`,
`PlayerGroupWord`, `ColorFreedom`, `TokenRider`, `NounWord`). Nothing was
re-measured against the corpus in this pass, so no cell claims one.

**Cells corrected against the code, not just re-anchored.** `CardType` 9→15
rows (now CR-closed); `Zone` 6→7 (Command ported); `Supertype` 3→5 (World and
Ongoing landed); `DamageableTy` 2→3 (the refused battle row landed);
`ascribesAsType` Battle False→True; `Characteristic` gained Loyalty;
`QualitySort` 4→6; `OutcomeSort` 5→14; `NounWord` 8→13; `Targetable` 2→4;
`Phrasal` 4→5; `AbilityClass` 3→5; `Designation` 14→16; `CounterKind` →23
rows including `LoyaltyCounter` (the snapshot's Loyalty refusal is stale);
`TurnPart` 10→11; `Owner` 7→8; `Compulsion` 3→4; `Cost` 7→9;
`ProducedMana` 3→9 (both rows recorded as not ported have landed);
`ManaRider` 1→3; `CopyExcept` 6→9 (the refused NAME and enters-with
exceptions landed); `PreventCut` 2→4 (the two refused shapes landed);
`Repetition` 3→5; `ColorFreedom` 2→3; `KeywordParamShape` 5→6;
`StaticEffect` head →54 rows. `LibOrdinal` is no longer a closed five-row
count — `Ordinal` is `Nth (n : Nat)`, so §1's flip-risk #1 (the Sixth hole) is
moot, as is #4 (the five closed `{1,2}` count tables, all retired).
`Subtype` is `MkSubtype host label`, so the ~130-row catalog, its per-word
census and the Phyrexian name-straddle ledger are all gone.
`Keyword`/`VerbName` are `String` labels over `keywordFacts` (63 rows) and
`verbFacts` (19 rows).

**Remainders (not done here, out of scope).**

- §1's flip-risk ranking, §7 and the Totals table still describe the demoted
  snapshot and now contradict §2.1/§2.5 in several places named above
  (LibOrdinal, the five count tables, `Subtype`, `Equipped`×`PermanentW`,
  `PreventCut`'s arithmetic). A follow-up should either refresh them or cut
  them.
- §2.2/§2.3/§2.4/§2.6 keep dead `Experimental.idr:NNNN` anchors.
- `boundedIncrease`, `modalHead`, `AtLeastOne`, `arrangementFits`,
  `ordinalFits`, `searchableZone`, `NextUntapCount`, `PhaseCount`,
  `counterZone`, `CounterHolder`, `typeRank`/`typesOrdered`, `SpanUse`,
  `NotLoyalty`, `freedomFits`, `repeatCount` have no successor I could
  positively identify; each is marked, not guessed.
- `idris/src/Experimental/{Effect,Phrase,Macros,Cards,Proofs*}.idr` were being
  edited by another session while this pass ran (Effect.idr grew ~60 lines
  mid-verification). Anchors were re-verified against the tree as of the final
  check; `Effect.idr` and `Phrase.idr` anchors may need one more sweep once
  that work lands.

**Closing sweep (after this lane's three grammar tickets landed).** The
predicted drift happened and was repaired mechanically: of the 233 anchored
rows in §2.1 and §2.5, **221 were still correct and 12 had drifted** — all of
them in `Phrase.idr` and `Words.idr` below the lines this lane's rows were
inserted at. Each was re-pointed to the nearest line naming its own
declaration, and a second pass reports 233 correct and 0 unresolved. The
`wc -l` source inventory in the header was re-taken at the same time.

The method that did it is now recorded in the header's **Refresh method**
list, which is what the ticket asked for: the three fields at the head of each
refreshed row (`<File>.idr:<line>` and the backticked name) are enough for a
throwaway script to assert every anchor and repair a drifted one, so the next
grammar round costs seconds rather than a re-read. That check is the cheap
half of a refresh; the cells still want a human pass.
