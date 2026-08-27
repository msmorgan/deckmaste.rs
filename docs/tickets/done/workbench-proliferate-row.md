---
needs: []
---
# Proliferate's counter row — the self-reading kind-blind distributive

Routed from `workbench-verb-label-residues` (close, 2026-08-26). Proliferate's
chooser half spells today (verified there: `CountedGroup anyNumber` over a
joined counters-having description at `Object \/ Player`). The missing half
is the counter row: "one additional counter of each kind [it] already has",
per member — a distributive that reads the HOLDER'S OWN kinds, where
`PutCountersOfThoseKinds`/`GetsCountersOfThoseKinds` read an ANNOUNCED batch
and presuppose one, and `PutSameCounters` reads a different holder. Rule:
[CR#701.34a]. Unblocks Tromell (and the proliferate `verbFacts` row + macro,
one each, per the open-label mechanism).

## Consumption boundary

`idris/src/Experimental/Effect.idr` (the row + its nine tables),
`Macros.idr`, `Words.idr` (`verbFacts` row), `Cards.idr` bench,
`Proofs*.idr`. No Rust crate.

## Acceptance

- Tromell benches whole or is named at a non-counter blocker; proliferate
  expands in full under its label.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

### The row: `GiveCountersOfOwnKinds`

```idris
GiveCountersOfOwnKinds : {k : Kind} -> (on : Noun bs k) ->
                         {auto 0 hk : So (kindLte k (Object \/ Player))} ->
                         {auto 0 pm : PerMember on} -> Effect bs
```

The self-reading kind-blind distributive, and the fourth member of the
counter-anaphor family: `PutCountersOfThoseKinds` and
`GetsCountersOfThoseKinds` range over an ANNOUNCED batch's kinds and
presuppose one, `PutSameCounters` reads a DIFFERENT holder's counters,
and this row reads its own recipient's with nothing announced anywhere.

**Amount-fixed, not a slot.** [CR#701.34a] gives "one additional counter
of each kind", and the corpus was checked for a line that generalises
it: none does. "Proliferate twice" (Contagion Engine, Agent Frank
Horrigan) and "Proliferate X times" (Expansion Algorithm, Tromell)
iterate the whole action, which is `Repeated`; the multiplicative
neighbour "double the number of each kind of counter on [n]" is a
different operation with no per-kind increment at all (ledger).

**Kind-indexed at `Object \/ Player`,** [CR#122.1]'s own pair, because
[CR#701.34a]'s choice names both halves at once and the giving clause
distributes over the union. `CoinCameUp`/`DealtThisWay` are the idiom.
No counter-existence gate: [CR#701.34a] asks for a counter in the
CHOICE, and a recipient holding none is given none, which is the rule
applied rather than a defect.

Nine total tables gained arms: `heldUntilOk` False, `reflexEncloseUse`
`EncReflexive`, `thisWayOutcomeOk` True, `costActionOk` `costNounOk on`
(as `PutCounters`; the anaphoric rows are False only because no cost
announces a batch), `effEq` False, `effIntro`/`preIntro`/`annIntro`
`nomIntro on`, `deedDelta` `[]`.

### The expansion

`verbFacts` gained `MkVerbFacts "Proliferate" Nothing Nothing Nothing`:
[CR#701.34a] has the ACT make its own choice rather than take a patient
from the instructing clause, every printed line writes the verb with
nothing after it, nothing moves, and no printed line names a
proliferated permanent by participle.

`Macros.proliferate` builds the full body, the rule's own two clauses:

```idris
Enact "Proliferate"
  (Sequentially [ Choose (CountedGroup anyNumber proliferable) Nothing
                , GiveCountersOfOwnKinds (EachOf (Those JoinW)) ])
```

with `proliferable = kindJoin (CounterCompare Nothing AtLeast (Lit 1))
(And [Permanent, HasCounters Nothing])`, the chooser half the parent
verified, re-verified here through `kindJoin` (which fixes both halves'
kinds where a bare `Joined` leaves the player arm's ambiguous). The
giving clause reads the choice back as the union demonstrative
`Those JoinW` under `EachOf` — the distributive `PerMember` wants and
the only route to it, since `CountedGroup` is no `GroupMention`. The
macro's one hypothesis is that read's antecedent count, discharged by
search at every call site.

The player half is `CounterCompare`, not `HasCounters`: the latter has
no player cell, and [CR#122.1f] writes the player-side test in the
ranged shape. The printed reminder text drops [CR#701.34a]'s "that have
a counter"; the rule keeps it and the two agree on outcome.

### Witnesses

- **Bloom Hulk — whole.** `{3}{G}` 4/4, "When this creature enters,
  proliferate." Measured as the cheapest whole-card carrier in the
  corpus: of the 13 cards writing a bare "Proliferate." and the
  trigger-carried lines beside them, it is the only one whose entire
  text is one vanilla body plus one entry trigger (Copper Longlegs adds
  reach and a sacrifice cost, Ate-o'-Clock adds populate, investigate,
  regenerate and replicate).
- **Tromell, Seymour's Butler — whole, no blocker.** Both lines bench:
  the entry rider through `entersWithAdditionalCounters` over
  `Each (And [creature, nontoken, ControlledBy You, OtherThan This])`,
  and `{1}, {T}: Proliferate X times, where X is the number of nontoken
  creatures you control that entered this turn` as
  `Repeated (LetterVal X) proliferate` beside `Define X (CountOf …)`,
  the count read through the existing `HappenedTo Entry ThisTurn` cell.
  Nothing past the counter row was missing.

### No new pin

The row's only refusals are `PerMember` — a distributivity requirement
it shares with `PutCounters`, not a rules impossibility — and the kind
bound. The bound is NOT pinnable either: an ability on the stack is an
object [CR#109.1], so [CR#122.1] does not make a counter on one
impossible; the exclusion rests on `counterScope`, where every counter
kind sits on a permanent, a card or a player, and on no printed line
writing it. Recorded as tolerated under-generation, refused nowhere.

### Ledger

| item | needs |
|---|---|
| the remove twin | "choose any number of permanents, then remove from each a counter of each kind already there" (Unclaimed Bird) — the same self-reading kind-blind distributive in the taking direction, at `Object` alone |
| the multiplicative twin | "double the number of each kind of counter on [n]" (Gilder Bairn, Vorel of the Hull Clade, Deepglow Skate, Ferrafor, Arcade Cabinet, Arna Kennerüd, The First Tyrannic War, The Thing, Zimone, Miles Morales) and its player seat, "double the number of each kind of counter you have" (Aetheric Amplifier) — ~11 witnesses, no row |
| the proliferate EVENT | "Whenever you proliferate during your turn, …" (Contagion Dispenser): the verbed event over the new label, unbenched here |
| kind-blind move under a negated kind set | "move a counter of each kind not on Goldberry from another target permanent you control onto Goldberry" (Goldberry, River-Daughter) |

Gates: `idris/scripts/build` 23/23 from clean, 0 errors, 0 warnings;
`cite check --list-noncompliant` empty; `cite check` 0 stale (all four
rules cited were already in the lock, no bless); `cite audit --diff` 10
sites read, one claim re-scoped on the reading (an earlier draft said
[CR#701.34a] "counts proliferations" — the rule counts nothing, it fixes
the per-kind amount at one, which is what leaves a written count only
the whole action to iterate).
