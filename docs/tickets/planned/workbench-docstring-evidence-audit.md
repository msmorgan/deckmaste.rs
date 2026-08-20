---
needs: []
---
# Settle the workbench's contradicted and uncounted docstring claims

A full sweep of the workbench turned up two kinds of evidence debt in the
sources' own docstrings, and neither was adjudicated — no source file was edited.
**Seven live discrepancies**, where the sources contradict themselves or where a
gate's shape and its evidence disagree. And a long tail of **unmeasured
assertions**, where a claim carries no corpus count behind it or a count whose
denominator is not stated. They are one claimable unit because they are one pass
over the same docstrings with the same question — what number is behind this
sentence — and because a discrepancy found while re-measuring an uncounted claim
should be resolved on the spot rather than filed twice.

Neither half flips a cell. This is an **investigation**: acceptance is a
**recorded decision** at each site — a corrected count, a corrected cell, a
supplied measurement, or an explicit "qualitative, not measured" marker — written
into the docstring, not into a note elsewhere.

## Part 1 — the seven live discrepancies

All seven are **REPORTED-NOT-RESOLVED**, each recorded here with the evidence
already assembled. Three are arithmetic or stale text and want a re-measurement;
four want a decision.

### Re-measure and correct

1. **`riderAct Regenerated` is 135 where `ObjectAct`'s own split gives 138.**
   `Words.idr` (restated in `Experimental.idr` at the `Effect.CantBe` row).
   `ObjectAct`'s comment gives the corpus split as "156 supported sentences… 138
   sentences attach to a destruction… 18 state 'this turn'", identifying the 138
   as the one-shot-rider population; `riderAct`'s comment two paragraphs later
   says `Regenerated` is 135, "which is what the round was for", and the `CantBe`
   row carries 135 forward. Either the 138 includes three sentences that are not
   `riderAct`'s, or one count is stale.
2. **The `Keyword` docstring's composite-row count goes stale mid-docstring.**
   `Words.idr`. It states "`Haste` and `Flying` are the two composite rows here",
   then says the grant round adds four more (convoke, improvise, storm,
   lifelink), then describes `Reach` as "a composite the carrier spells, exactly
   as `Haste` and `Flying` are" — a seventh. The original claim is never
   corrected. Read narrowly it is superseded rather than contradicted, but a
   reader taking "the two composite rows" at face value is wrong.
3. **`PreventCut`'s arithmetic: 88 against 60 + 21 = 81.** `Experimental.idr`.
   The docstring calls the family "88 event-shaped sentences", then sums 47 of 49
   one-shots plus 13 of 39 standing reducers for `CutAll` (60) and 21 all standing
   reducers for `CutSome` — seven short. The three refused shapes counted
   alongside ("all but N" 5, "half…rounded" 2, the where-X rider 3) sum to 10 and
   do not close the gap either.

### Decide

4. **`attachHeadOk Equipped PermanentW = False` against its own cited evidence.**
   `Words.idr`: the comment for the Planeswalker cell reasons from [CR#702.6e]'s
   "as though that planeswalker were a creature" that **Luxior's line says
   "equipped permanent"**, not "equipped planeswalker" (zero occurrences of the
   latter) — positive evidence for the phrase — and the `PermanentW` cell is
   coded False two lines later with no comment explaining the refusal. Either the
   Luxior line does not license this table's cell (a different constructed phrase
   this table does not model) or the cell is a stale missed True.
5. **`comparableBound` is relation-blind where its evidence is equality-only.**
   `Experimental.idr`. `comparableBound a = writtenBound a || readAmount a`
   admits a summed bound (`Aggregate` with `AggregateOp = Sum`) at *every*
   comparator, since `CompareAmt` places no relation restriction on the bound —
   but the corpus evidence for the correction that admitted it is **12/12 lines
   at the equality relation only** ("mana value equal to 1 plus…"). The other
   four relations against a summed bound are unattested and explicitly queued for
   their own round. The type system permits a construction class the corpus has
   never measured.
6. **`admitsSpan CostModification` declines to flip a cell one attested line
   fills.** `Events.idr`. Of 653 lines exactly one (Cheering Fanatic) carries a
   duration and its cell is **deliberately not flipped**, the stated reason being
   that flipping it would rename a `SpanUse` row. Because `SpanUse`'s row names
   *are* the measured construction sets, the row's name is doing work a
   measurement normally does. This is a deliberate under-admission with an
   attested counterexample already printed, recorded as ledger rather than as a
   cell — a tension between a naming convention and a measurement, not a slip. It
   also ranks thirteenth among the workbench's flip risks; landing the
   chosen-name-subject construction forces the question.
7. **`annIntro`'s "on a FLAT clause the two agree exactly" has an exception in
   the source.** `Experimental.idr`, `preIntro` against `annIntro`, found by a
   row-by-row diff of the two 51-row tables. Ten rows differ; six differ only in
   which function name recurses; the four substantive ones are `May`,
   `Sequentially`, `Simultaneously` — and `ExtraTurn`. `annIntro`'s docstring
   predicts and justifies the first three by name
   (`badSimultaneousReadsMayDeed`, `badInsteadReadsReplacedSequenceOutcome`) and
   then states the general claim. `ExtraTurn w _` is a flat row where `annIntro`
   prepends `turnRefB` and `preIntro` does not. Either the "exactly" is a slight
   overstatement (the turn referent is a deed and `preIntro` is right to withhold
   it, so the sentence should read "…agree except where the clause MAKES a
   referent") or one of the two rows is wrong.

## Part 2 — the unmeasured assertions

Every claim a docstring makes **without a corpus count behind it, or with a count
whose denominator is not stated**. None is an error — several are flagged as
unmeasured by the source itself — but each is a place a later round would lean on
an unchecked number. The workbench's whole discipline is that a row is bought by
a witness, so an unbacked claim is a debt.

The job: for each site below, either supply the measurement in the docstring or
mark the claim explicitly qualitative. Both outcomes are fine; leaving the reader
unable to tell which is not.

### `Experimental/Words.idr`

`comparedType` (the Vehicle noncreature-printed-power exception under
[CR#208.3], "never written", no count) · `Kind`/`Ability` (the PHRASAL claim
cites co-occurring modifiers as qualitative evidence, no per-modifier line count)
· `OutcomeSort.DamagePrevented` ("36 sentences" of prevention lines marked "this
way", a watch item because the same marking recurs at `VerbedMarking` with 43
Mill lines — different constructions, not conflated, but worth a denominator) ·
`modalHead` (the attested-heads list asserted as the full attested set, refused
heads counted at zero, no positive per-head counts) · `Origin` ("the two rows do
NOT interact… no reader tests both", no count) · `VisibleThing` (the
possessive-agreement claim covers 21 reveal and 44 look-at lines, but
`WholeHand`'s own reveal/look-at split is not given) · `verbedWordOk`'s
`TokenW`/`CopyW` participial screen (zeros asserted with no total-occurrence
denominator) · `EntryCounterMark` (per-cluster counts over 578 supported
occurrences with no finding number for the underlying recon) · `PlayerGroupWord`,
`JoinedPlayer`, `JoinedClass`, `ScaleFactor`, `ShiftDir`, `NextUntapCount`,
`ExtraTurnCount`, `SkipCount`, `PhaseCount` (specific counts, no finding
citation) · `typeRank` Battle/Instant/Sorcery (**stated outright as unwitnessed**
— a convention living in a closed-count table beside witnessed ranks) ·
`keywordCounterOk` (per-cell zeros with no total to normalize against) ·
`Owner.ThatTurns` (the "one part" claim's measurement site is in another file) ·
`SpanUse` (per-adverbial counts whose assigning function lives at `spanUse`
elsewhere, so no cell could be cross-checked).

### `Experimental/Events.idr`

`CondMarking` ("the corpus is lopsided", no line counts for either word) ·
`deonticPatientOk ForbidT Block Agent` (14 counted, the bare-form majority not) ·
`deonticPatientOk GateT Block Agent` (the plural-subject count not given) ·
`triggerWordOk` `LifeGain`/`LifeLoss` (answered from quoted English, where the
sibling rows a few lines away are measured) · `admitsSpan` for `PtDelta` and
`KeywordGrant` (True/False asserted with no inline count where the grid's
neighbours have them).

### `Experimental.idr`

`ZoneScope` OwnedBy (the plural relational possessor family, no count) ·
`OfLastChosenColor` (total across 12 carriers, per-carrier repeatability not
counted) · `HasCounters` (player-scoped description names 6 cards, no aggregate
line count against the 221 object-scope figure) · `AnyTargetLone` (17-occurrence
split 4+13, no raw pre-support-filtering count) · `negatable` at `Permanent`
(**"was not measured by this round's recon"** — an explicit in-line admission) ·
`YouAnd` ("the corpus agrees — no supported sentence reads the group back", no
count of how many were checked) · `TheRest` ("almost all", no
numerator/denominator) · `elemIntro` (19 "it" / 14 "that <noun>", denominator not
stated so the split cannot be checked exhaustive) · `Amount.CountOf` ("every
corpus for-each domain is noun-HEADED", no count) · `EventSource.FromAnywhere`
(argued from rule text alone, where neighbours carry 19-header/4-header counts) ·
`anchorPhrase`'s False cells for
`CountedGroup`/`AllOf`/`EachOf`/`YouAnd`/`LibrarySlice`/`SomeOf`/`TheRest` (each
"a set, not a referent", no line count, unlike the neighbouring
`TargetGroup`/`Definite` cells) · `GameEvent.BeginningOf` ("the ONLY event with
no noun phrase in it at all", no count) · `GameEvent.Casts` ("divides on both",
no split count, where `DealsCombatDamage` gives 625/688) · `ChapterMark`'s
threshold tally ("an unchecked side condition", no count of how many of the 581
supported chapter lines need it) · `TokenSpec.TokenWritten` (its own line count
never stated; siblings are 43 and 288, so its population is implied by
subtraction) · `CostShift` ("about twenty-five lines") · `Gains` (26 supported
cards for two-level nested quotation, no denominator) · `AddsChosenQuality` (12
supported subtype lines splitting into named groups summing to 10, "and 2 more"
unnamed) · `Repeat` (44 over 44 asserted, but the sub-ledgers 42+2 and 9+6+2
overlap unstated) · `Repeat`'s exception-on-process cells (2 lines, the two
Lores, blocked on "chooser machinery" not further quantified) · `ChangeLife` (no
corpus count at all) · `Expose` (no sentence count for the look/reveal split) ·
`Continuously` (span-adverbial table described as corpus-determined with no
numbers) · `If` and `WhereLetter` (linearization availability called "a
linearization side condition, unchecked here") · `moveIntro`'s `AsType` stamp
conservatism ("waits on a corpus witness") · `headerWindowOk`'s MainPhase gap (3
named cards want the cell; the general MainPhase-Yours row opens later in the
file while the specific cell stays pinned, and the two comments are never
reconciled) · `staticOnSpellCardOk`'s Conditionally-recursion claim ("an attested
cell, so the recursion widens nothing", from exactly 2 named cards both said to
be blocked on unstated clauses) · the `Card` record's name field ("nothing here
reads it back", no count).

## Relation to the closure tables

`workbench-closure-table-review` sweeps the same sources for rows whose evidence
is *stale*; this ticket takes the rows whose evidence is *missing* or
*self-contradictory*. A row that turns out to be one of the other's is routed
across with the number attached, not filed twice.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/Events.idr` — docstrings only, except where a decision on
one of the seven changes an admission; evidence bench
`idris/src/Experimental/Cards.idr` in that case. A measurement that turns out to
refute a cell is not this ticket's to fix: route it to the ticket that owns the
cell and record the number here.

## Acceptance

- Each of the seven ends as a corrected count, a corrected cell, or a recorded
  decision in the docstring at the site — not as a note elsewhere.
- Any cell that flips lands with its witness or its pin, and any count that
  changes is re-measured rather than reconciled by arithmetic.
- Every site in part 2 ends with a count, a denominator, or an explicit
  "qualitative, not measured" marker in its own docstring.
- Part 2 flips no cell; it audits claims, it does not change admissions.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
