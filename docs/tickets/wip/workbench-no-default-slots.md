---
needs: []
---
# Every construction takes all its parameters as required positionals

## Ruling (2026-08-22)

Default slots (`{default … name}`) on core constructors are banned — not for
authoring convenience, which macros supply, but for **translation**: a
realised card macro tree must translate into an Idris term positionally,
term-for-term, so a card can be verified later without a permutation or
defaulting step in between. 41 `{default …}` slots exist in
`idris/src/Experimental.idr` (e.g. `MayPlay`'s `window`, `Move`'s `riders`,
`KeywordAbility`'s `param`, the trigger header's four, `LosesCounters`'s
`amt`). Each becomes an explicit argument; the common case is a wrapping
macro. `docs/decisions/card-authoring-binds-no-implicits.md` is amended to
state the contract (done with this ticket's minting).

`{auto 0 …}` proof gates are not slots and stay.

## Consumption boundary

`idris/src/Experimental*.idr`, `Macros.idr`, `Cards.idr` (must still bind
no implicits), `Proofs*.idr`. Mechanical; Sonnet-grade once the slot
inventory is listed.

## Acceptance

- `grep -c '{default' idris/src/Experimental*.idr` → 0; `Cards.idr` binds
  0 implicits; every former default has a macro supplying it.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

All 41 `{default …}` slots on 28 constructors in `idris/src/Experimental.idr`
are gone; each is now a named positional argument in the place the slot sat.
`{auto 0 …}` gates are untouched. Call sites outside `Cards.idr`
(`Experimental.idr`, `Macros.idr`, `Proofs*.idr`) write the former default
value; `Cards.idr` writes none of them — every bench site reaches the
constructor through a macro, and `grep -cE '\{[a-zA-Z_]+ *= '` there is still
`0`.

### Slot table

`n·` is the argument's position among the constructor's explicit arguments.
"Macro" is what supplies the old default for the bench; *(new)* macros were
added by this ticket, the rest already existed.

| Constructor | argument (position) | old default | macro supplying it |
| --- | --- | --- | --- |
| `LibraryAt` | `off : Maybe LibOrdinal` (3·) | `Nothing` | `onTopZ`, `onBottomZ`, `topOrBottomZ`, `onTopIn`, `onBottomIn`, `choiceOfTopOrBottom` |
| `QualityNoun` | `dom : Maybe (ChoiceDomain q)` (2·) | `Nothing` | `quality` *(new)* |
| `HappenedTo` | `what : Maybe (EventComplement …)` (3·) | `Nothing` | `happenedTo` *(new)* |
| `AsType` | `sub : Maybe Subtype` (3·) | `Nothing` | `thisCreature`, `thisArtifact`, `thisEnchantment`, `thisLand`, `thisPlaneswalker` *(new)* |
| `TheVerbed` | `marking : VerbedMarking` (3·) | `Attributive` | `theVerbed` *(new)* |
| `ThoseVerbed` | `marking : VerbedMarking` (3·) | `Attributive` | `thoseVerbed` *(new)* |
| `EventCount` | `what : Maybe (EventComplement …)` (4·) | `Nothing` | `eventCount` *(new)* |
| `Happened` | `what : Maybe (EventComplement …)` (4·) | `Nothing` | `happened` *(new)* |
| `Attacks` | `whom : Maybe (Noun … Player)` (2·) | `Nothing` | `attacks` *(new)* |
| `LastCounterRemoved` | `by : Maybe (Noun bs Player)` (3·) | `Nothing` | — (no bench site; `lastCounterRemovedBy` covers the written form) |
| `PutInto` | `from : Maybe (EventSource bs)` (3·) | `Nothing` | — (no bench site; `putIntoFrom` covers the written form) |
| `CounterEvent` | `many : CounterBatch` (4·) | `ManyCounters` | `manyCounterEvent` *(new)* |
| `CounterEvent` | `by : Maybe (Noun bs Player)` (5·) | `Nothing` | `manyCounterEvent` *(new)* |
| `CounterEvent` | `cause : Maybe Causer` (6·) | `Nothing` | `manyCounterEvent` *(new)* |
| `TokensCreated` | `cause : Maybe Causer` (2·) | `Nothing` | `tokensCreated` *(new)* |
| `TokensCreated` | `by : Maybe (Noun bs Player)` (3·) | `Nothing` | `tokensCreated` *(new)* |
| `TokensCreated` | `under : Maybe (Noun bs Player)` (4·) | `Nothing` | `tokensCreated` *(new)* |
| `Conditionally` | `marking : CondMarking` (3·) | `AsLongAs` | `asLongAs` |
| `OnlyWhile` | `marking : CondMarking` (3·) | `AsLongAs` | `onlyWhile` |
| `MayPlay` | `verb : PlayVerb` (3·) | `Play` | `mayPlay` *(new)* |
| `MayPlay` | `from : Maybe (ZoneExpr …)` (4·) | `Nothing` | `mayPlay` *(new)* |
| `MayPlay` | `asThough : Maybe PlayAsThough` (5·) | `Nothing` | `mayPlay` *(new)* |
| `MayPlay` | `limit : Maybe PlayLimit` (6·) | `Nothing` | `mayPlay` *(new)* |
| `MayPlay` | `window : Maybe PlayWindow` (7·) | `Nothing` | `mayPlay` *(new)* |
| `EntersWithCounters` | `mark : EntryCounterMark` (4·) | `Fresh` | `entersWithCounters` (widened, see note 2) |
| `EntersChoice` | `dom : Maybe (ChoiceDomain q)` (3·) | `Nothing` | `entersChoosing` *(new)* |
| `MkMoveRiders` | `counters : Maybe (CounterRider bs)` (3·) | `Nothing` | `noRiders` *(new)*; no bench site |
| `GainsDesignation` | `span : Maybe (Duration …)` (4·) | `Nothing` | `gainsDesignation` *(new)* |
| `Choose` | `by : Maybe (Noun bs Player)` (2·) | `Nothing` | `choose` *(new)* |
| `Move` | `riders : MoveRiders …` (3·) | `MkMoveRiders [] Nothing` | `move` *(new)*, over `noRiders` *(new)* |
| `LosesCounters` | `amt : Maybe (Amount …)` (3·) | `Nothing` | `losesAllCounters` |
| `Delayed` | `span : Maybe (Duration bs)` (2·) | `Nothing` | `delayed` *(new)* |
| `AdditionalPart` | `followedBy : Maybe TurnPart` (4·) | `Nothing` | `additionalPart` *(new)* |
| `KeywordAbility` | `param : Maybe (KeywordParam bs)` (2·) | `Nothing` | `keyword` *(new)* |
| `Activated` | `window : Maybe Timing` (3·) | `Nothing` | `activated` *(new)* |
| `Activated` | `limit : Maybe UsageLimit` (4·) | `Nothing` | `activated` *(new)* |
| `Activated` | `guard : Maybe (Condition bs)` (5·) | `Nothing` | `activated` *(new)* |
| `Triggered` | `alt : Maybe (GameEvent bs)` (3·) | `Nothing` | `triggered` *(new)* |
| `Triggered` | `window : Maybe TriggerWindow` (4·) | `Nothing` | `triggered` *(new)* |
| `Triggered` | `limit : Maybe UsageLimit` (5·) | `Nothing` | `triggered` *(new)* |
| `Triggered` | `intervening : Maybe (Condition …)` (6·) | `Nothing` | `triggered` *(new)* |

Bench renames: 589 `Cards.idr` sites moved from a constructor to a macro
(`Triggered`→`Macros.triggered` 135, `Activated`→`Macros.activated` 131,
`KeywordAbility`→`Macros.keyword` 126, `Move`→`Macros.move` 42,
`Choose`→`Macros.choose` 27, `EntersChoice`→`Macros.entersChoosing` 21,
`Attacks`→`Macros.attacks` 21, `Conditionally`→`Macros.asLongAs` 12,
`QualityNoun`→`Macros.quality` 10, `HappenedTo`→`Macros.happenedTo` 8,
`Delayed`→`Macros.delayed` 8, `Happened`→`Macros.happened` 7,
`CounterEvent`→`Macros.manyCounterEvent` 7, `EntersWithCounters` 6,
`TheVerbed` 5, `AsType` 5, `MayPlay` 4, `GainsDesignation` 4,
`AdditionalPart` 4, `LibraryAt` 2, `ThoseVerbed` 1, `TokensCreated` 1,
`EventCount` 1).

### Elaboration notes

1. `ComplementWritten Nothing` is NOT solvable inside the macro: its
   `LeftBare` constructor carries `So (bareLookbackOk ev ks)`, which mentions
   the macro's own `ev` and `k`. `happenedTo`, `happened` and `eventCount`
   therefore forward it as `{auto 0 cw : ComplementWritten (the (Maybe
   (EventComplement … ev k)) Nothing)}` so the search still runs at the card
   site. This is the ADR's forwarding rule, not a weakened gate.
2. `entersWithCounters` was widened from `(k : Nat)` to `(amt : Amount bs)`.
   The bench writes Amount-valued counts (`DefinedLetter LetterX`,
   `Macros.forEach …`) that the `Nat` form could not express, and a second
   near-identical macro name would have been worse than one. Its single
   existing caller (`workhorse`) now writes `(Lit 4)` — the same term the
   macro used to build.
3. Gates that mention only the now-fixed default value stay solved inside the
   macro and are not forwarded: `RidersFit noRiders _` (reduces to `So True`),
   `AttackDefender Nothing`, `CreationVoice Nothing Nothing Nothing`,
   `DelaySpanOk Nothing` and `FollowerPart Nothing`. Gates that do mention a
   macro argument (`KeywordParamFits k …`, `ChapterDefaults ev …`,
   `CostTapOnce cost`, `ZoneFits (nounZone n) …`, …) are forwarded.
4. No pin lost its witness: the `Proofs*.idr` negatives keep every
   `{… = ok}` binding and only gained the positional defaults, and
   `scripts/build` is 19/19 with the `impossible` clauses still refused.

### Reading of the docstring instruction

The one-line docstrings went on the nineteen new macros (each names the
English it spells). No per-constructor "this slot is explicit" line was added
to `Experimental.idr`: the positional type says it, the ADR states the
contract globally, and 28 near-identical lines would be sprawl. Flagged here
for veto.
