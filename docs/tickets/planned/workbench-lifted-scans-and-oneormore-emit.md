---
needs: []
---
# Generate the lifted scans; fill the `OneOrMore` emit arm

Split out of `workbench-structure-phrase-and-toolchain` (2026-08-22): both
sections below are toolchain work — a Rust emit arm in
`crates/deckmaste_plugin/src/idris_emit.rs` and a generator feeding
`idris/scripts/build` — independent of that ticket's grammar merges, which
stay there. Neither touches a grammar constructor. The generator step belongs
beside `idris/scripts/emit-tables`, never in a deletion-slated crate. Read
finding 534 in `docs/memory/archive/binder-unification-probe-experiment-log.md`
before writing the `OneOrMore` arm (the counted-group/universal mark is
`CountD`).

## Generate the lifted scans instead of hand-writing them — NOT a grammar round

One predicate lifted over a mutually-recursive family of indexed datatypes costs
six hand-written total functions, each a case analysis whose non-leaf rows
delegate to the sibling at the argument's sort. Idris 2 has no generic-deriving
facility for this, so the six definitions are mechanical necessity rather than a
phrase costume — and that is precisely why they are a **generation** candidate
and not a macro one.

The structure-vs-phrase sweep that convicted eight other families ruled this one
genuinely distinct by Idris mechanics **but generatable, and recorded that as the
finding**: it is one predicate lifted over a mutually-recursive family, not a job
re-minted per container. Take that verdict as given; the costume lens should not
fire here.

- The named family: the six-function scan over the indexed datatypes, plus the
  roughly four more scans of the same shape beside it (the
  deep negation scan, the says/delta readers, and the zone scan). The multiplier
  is about four families × about six definitions, and the fix is **one
  generator**.
- The nine list-scanning witnesses over predicate lists, which the sweep flags as
  "likely a generation family of family 14's kind, not a costume" — confirm that
  before folding them in, and fold them in if it holds.
- Check the arity/flatness witnesses re-minted per layer **once**: their own
  docstring says strict positivity forbids a shared arity witness. That is a real
  Idris constraint that should be checked at one site rather than assumed at
  each; if it holds, record it once and stop re-litigating it per layer.

## Fill the `OneOrMore` arm in the Idris emit path — NOT a grammar round

Not a grammar round — a Rust-side gap of exactly one match arm. In
`crates/deckmaste_plugin/src/idris_emit.rs`, `emit_event_filter` currently
answers `EventFilter::OneOrMore(_) => return Err(gap(…))`, so every filter of
that shape refuses at emit time regardless of what the grammar can express.

- Implement the arm. The mark that separates a counted group from a universal is
  `CountD`; a taker should read finding 534's reasoning on that distinction
  before writing the arm, because getting it wrong silently emits a universal
  where the card writes "one or more".
- Nothing about the Idris side changes; this is the emitter catching up.

## Consumption boundary

`crates/deckmaste_plugin/src/idris_emit.rs`, the generator and
`idris/scripts/build` / `idris/scripts/emit-tables`. No grammar file changes.

## Acceptance

- `cargo test -p deckmaste_plugin` with a regression case distinguishing
  counted-group from universal; `idris/scripts/build` PASS; the generated
  scan definitions byte-identical to the hand-written ones on first
  generation (empty diff).

Standard constraints apply.

## As-landed

Landed: the `OneOrMore` arm's refusal, sharpened and pinned by a regression
case. **Not** landed: the generator. Both sections' premises were checked
against the primary sources first and both were partly wrong; the checks and
their evidence are below.

### `OneOrMore` — the arm cannot close against this emitter's target

`idris_emit.rs` emits `idris/src/Semantics.idr` (its module docstring says so,
and no Rust file anywhere references `Experimental`). `CountD` and
`CountedGroup` are in the **english_v2 workbench grammar**
(`Experimental/Words.idr:618`, `Experimental.idr:1292`), which this emitter
does not target. `CountD` is also a NOUN-PHRASE determiner, not an
event-pattern mark.

In `Semantics.idr` the emit target is `MkEventQuery kinds facets`, and neither
coordinate can carry [CR#603.2c]'s batch quantifier:

- no `EventKind` constructor takes a multiplicity (`Semantics.idr:459..529`);
- `Facet` is `Actor`/`Agent`/`Patient`/`Within`/`Whenever`/`IsNth` plus
  `And`/`Or`/`Not` (`Semantics.idr:1591..1617`). `IsNth` counts OCCURRENCES
  within a `Window`, not members within one occurrence, so it is not this;
- `Triggered`'s `limits` is `OncePerTurn`/`OncePerGame`/`LoyaltyOncePerTurn`
  (`Semantics.idr:271`) — an ability-usage cap, not a per-batch match rule.

Emitting the operand alone would write the per-member universal — exactly the
silent widening finding 534 warns about — so the arm still refuses. What
landed instead:

- the gap now names the real blocker and cites [CR#603.2c], replacing
  `"EventFilter::OneOrMore not yet mapped"`;
- `idris_emit::tests::one_or_more_refuses_rather_than_emitting_the_universal`
  pins the distinction where it can currently be drawn: the bare zone-change
  filter emits `(ZoneChanged (Just Battlefield) (Just Graveyard))` with no
  facets, and the `OneOrMore`-wrapped filter must NOT emit that same thing.

Closing the arm needs a `Semantics.idr` coordinate (a batch `Facet`, or a
field on `EventQuery`). That is a grammar-constructor change, which this
ticket and its parent both put out of scope — so it is a queue entry, not a
gap this ticket can fill.

Adjacent, recorded not built: `EventFilter::Nth` gaps at
`idris_emit.rs` beside `OneOrMore`, yet `Facet.IsNth : Nat -> Window -> Facet`
is its exact counterpart. That arm looks closable today.

### The lifted scans — STOPPED, premise refuted

The ticket's "about four families x about six definitions, all the same shape"
does not hold. Inventory, all in `idris/src/Experimental.idr` (line ranges
cover the declaration plus its clauses):

| family | functions | lines | shape |
| --- | --- | --- | --- |
| any-target scan | `nameSrcAnyTargetFree` 85-88, `anyTargetFree` 1174-1219, `anyTargetFreeAll` 1222-1224, `nounAnyTargetFree` 1375-1401, `placeAnyTargetFree` 1404-1407, `zoneAnyTargetFree` 1410-1414, `complementAnyTargetFree` 1556-1560, `amtAnyTargetFree` 1618-1635 | 112 | structural lift |
| delta readers | `nameSrcDelta` 79-82, `nounDelta` 1442-1473, `predDelta` 1482-1506, `predDeltaAll` 1509-1511, `placeDelta` 1514-1517, `zoneDelta` 1520-1524, `searchDelta` 1541-1543, `complementDelta` 1550-1553, `amtDelta` 1599-1615, `condDelta` 2024-2032 | 106 | measured table |
| says scan | `predSays` 1063-1108, `predSaysAny` 1111-1113 | 49 | measured table |
| deep negation scan | `predNegFree` 1120-1165, `predNegFreeAll` 1168-1171 | 50 | near-structural |
| zone scan | `seedZone` 283-309, `seedZoneAll` 312-314, `seedZoneJoin` 319-321, `allSeedZone` 326-329, `searchZone` 1536-1538, `sourceZone` 2110-2113, `nounZone` 4117-4151 | 79 | measured table |

Only the any-target scan is a structural lift — a fold whose every row is
derivable from the constructor's argument types. The rest carry per-constructor
semantic content that no generator can derive, only restate:

- `nounDelta` picks a determiner tag per row (`EachD`/`AD`/`TheD`/`TargetD`/
  `CountD`/`AllD`/`PartD`), plus a plurality and a phrase profile;
- `predDelta` ends in a catch-all `_ = []` and deliberately does NOT descend
  where the structural rule would (`Not p = []`, `Or ps = []`);
- `seedZone` answers a measured zone per row (`Attacking = Just Battlefield`,
  `CastBy _ = Nothing` under a [CR#400.7d,702.40a] comment), with a catch-all;
- `predSays` is `True` everywhere except `And ps = predSaysAny ps`, `Or _ =
  True`, `Not p = predSays p` — three judged rows, not a lift.

Whether to descend into an argument is itself a per-family decision, not a
type-level fact: `anyTargetFree (Compare _ _ b) = amtAnyTargetFree b` against
`predNegFree (Compare _ _ _) = True` on the same constructor.

Two further obstacles to the byte-identical bar, independent of the above:

1. **The definitions cannot leave the file or be gathered into one region.**
   They are mutually recursive with the datatypes they scan — `Predicate.And`
   carries `{auto 0 zc : ZoneCoherent ps}` and `ZoneCoherent ps = So (zonesOk
   ps)`, `Noun`'s constructors carry `{auto 0 af : AnyTargetFree p}` — so they
   must stay inside the one 5225-line `mutual` block, at 30-odd disjoint sites
   interleaved with the declarations. A generator would have to splice in place
   between markers, not emit a module. Probe 1 below shows the block is
   sensitive to exactly this kind of move.
2. **The hand-written text is not uniform.** `amtAnyTargetFree`'s clause order
   is not `Amount`'s declaration order (`StatOf` before `PlayerStatOf`);
   binders are sometimes named-and-unused (`AsType t n`, `Indefinite m p`,
   `ZoneAt z Bare`) where siblings write `_`; `placeAnyTargetFree` and
   `zoneAnyTargetFree` split sub-patterns (`EitherEnd Nothing` /
   `EitherEnd (Just n)`) that arity alone does not predict; two clauses wrap.
   Byte-identity needs a normalizing pass over the hand-written text first.

Stopping rather than half-landing: a generator covering only the any-target
scan derives 112 of the ~396 lines named, and buys that by adding a build step
plus marker scaffolding inside the workbench's central grammar file, leaving
the other three families hand-written beside it. Re-open with the scope
corrected to the any-target scan alone if that trade is wanted.

### The nine list-scanning witnesses — REFUTED, not the same family

`ZoneCoherent` 545, `ContradictionFree` 780, `OtherAnchored` 833,
`AnyTargetLone` 960, `ParallelDisjuncts` 998, `CoordinableDisjuncts` 1024,
`DistinctDisjuncts` 1040 — seven, not nine. `LoneComparison` and
`TwoDisjuncts` do not exist in the file; `Predicate.Or`'s arity witness is
Prelude's `NonEmpty`.

Each is a two-line `Name ps = So (f ps)` alias over a DIFFERENT predicate
(`zonesOk`, `contradictionFree`, `otherAnchorOk`, `anyTargetLone`,
`parallelDisjuncts`, `coordinableAll`, `noRepeatedPair`). They share the alias
line and nothing else, so they are not a lifted scan over a mutually-recursive
family and do not fold into the generation family. The one genuinely uniform
sub-shape nearby is the all-elements list fold (`anyTargetFreeAll`,
`predNegFreeAll`, `coordinableAll`, `zoneAdmitsAll`, `predDeltaAll`, ...) —
three lines each, below any generator's break-even.

### Strict positivity and a shared arity witness — REFUTED as stated, with the real constraint

The claim is mis-attributed: `atLeastTwoCs` (`Experimental.idr:1977`) has no
docstring. The file's only positivity note is on `anyTargetOkAt`
(`Experimental.idr:1239`) and is about a different function (`flattenPs`).
There is also no per-layer duplication to remove — `atLeastTwoCs` is the file's
only list-arity function, and `Predicate.Or` already shares Prelude's
`NonEmpty`.

Two probes were run on the real file (both reverted; `idris/` is unchanged):

- **Probe 1 — shared witness as `So` of a polymorphic Bool.** Added
  `atLeastTwoL : List a -> Bool` / `AtLeastTwoL xs = So (atLeastTwoL xs)` to
  `Experimental/Words.idr` and pointed `TwoConjuncts` at it. **FAILS:**
  `Error: Condition is not total, not strictly positive`, cascading to
  `Duration`, `Effect`, `Effects`, `Exists`, `UntilEvent`, `Visibility`,
  `WhereLetter`, `WhereLetterStatic`.
- **Probe 2 — shared witness as DATA.** Replaced it with
  `data AtLeastTwoL : List a -> Type where TwoUpL : AtLeastTwoL (x :: y :: xs)`
  and pointed `TwoConjuncts` at that. **PASSES 18/18.**

Verdict: strict positivity does not forbid a shared arity witness. It forbids
one spelled as `So` of an out-of-block Bool function — the block can only see
through a function it contains. The workable shape is a data witness, which is
what `Predicate.Or`'s `NonEmpty` already is. Cost of a switch: `So`-shaped
pins matching on `Oh` must match the data constructor instead
(`ProofsG.idr:157` `badSingletonConjunction` warns under probe 2). Recorded
once; do not re-litigate per layer.
