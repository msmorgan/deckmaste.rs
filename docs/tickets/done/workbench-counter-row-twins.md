---
needs: []
---
# The self-reading counter row's twins: remove, double, and the negated set

Routed from `workbench-proliferate-row` (close, 2026-08-27), which landed
`GiveCountersOfOwnKinds` (the self-reading kind-blind distributive at
`Object \/ Player`, [CR#701.34a]). Its measured siblings, one family:

1. **The remove twin** — "choose any number of permanents, then remove from
   each a counter of each kind already there" (Unclaimed Bird): same
   self-reading distributive, taking DIRECTION, at `Object` alone.
2. **The multiplicative twin** — "double the number of each kind of counter
   on [n]" (~10 cards: Gilder Bairn, Vorel, Deepglow Skate, Ferrafor,
   Arcade Cabinet, Arna Kennerüd, The First Tyrannic War, The Thing,
   Zimone, Miles Morales) plus the player seat ("… you have", Aetheric
   Amplifier). A multiplication over the holder's own kinds, not a per-kind
   increment — no row.
3. **The kind-blind move under a NEGATED kind set** (Goldberry,
   River-Daughter) — the self-reading move that excludes named kinds.

Weigh one parameterized row (direction/operation slot) against three
siblings before minting; `GiveCountersOfOwnKinds`' nine-table pattern is the
model either way.

## Consumption boundary

`idris/src/Experimental/Effect.idr` (+ its nine tables per row), `Words.idr`
if an operation word is needed, `Macros.idr`, `Cards.idr` bench,
`Proofs*.idr`. No Rust crate.

## Acceptance

- Each numbered item lands or ends in a written rule-backed verdict; at
  least Vorel or Deepglow Skate benches whole for item 2.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-anaphora-e-partitive-surfaces (close, 2026-08-27):** the COUNTER partitive over a described group — Tayam's distributive removal family (19 distinct lines): the slice is counters and the domain a description, so neither `SomeOf` (objects, mention) nor `Distribute` (no removal arm) reaches it. Counter-distribution machinery, so it lands here.

## As landed

The weighing the ticket asked for came out AGAINST the parameterized row.
A direction/operation slot on `GiveCountersOfOwnKinds` would have to serve a
removing direction that is at a measured zero and a multiplying operation
that differs in arithmetic rather than direction — so the multiplicative
twin is a row of its own and the other two siblings are verdicts.

**Item 1 — the remove twin: a MEASURED ZERO, not a pin.** The ticket's only
named evidence, Unclaimed Bird, is `"supported": false` in
`data/derived/cards.jsonl` (a Mirran/Phyrexian team card), so it may not be
benched and may not carry a row. Broadening the search past it, no supported
line writes the self-reading distributive in the removing direction: `remove
(a|one) counter of each kind …` matches nothing supported. Recorded as a
measured zero, with no Unspellable pin — nothing about the removing direction
is rules-impossible, and a count is not a refusal.

**Item 2 — the multiplicative twin: `DoubleCountersOfOwnKinds`, built and
benched.** `Effect.idr` gains the row beside `GiveCountersOfOwnKinds`, with
the same `{k : Kind}` / `So (kindLte k (Object \/ Player))` / `PerMember`
shape and the same nine tables (`heldUntilOk`, `reflexEncloseUse`,
`thisWayOutcomeOk`, `costActionOk`, `effEq`, `effIntro`, `preIntro`,
`annIntro`, `deedDelta`). Kind-indexed at [CR#122.1]'s own object-or-player
pair because the player seat is printed (Aetheric Amplifier's "…you have");
amount-fixed on a measurement, not an assumption — doubling is the only
multiplier any supported line writes (`triple the number of` matches nothing
supported), so a written count outside the operation is `Repeated`.
Measured family: 11 supported cards — Aetheric Amplifier, Arcade Cabinet,
Arna Kennerüd, Deepglow Skate, Ferrafor, Gilder Bairn, Miles Morales,
The First Tyrannic War, The Thing, Vorel of the Hull Clade, Zimone (the
ticket's "~10 plus the player seat", confirmed).
Bench: **Vorel of the Hull Clade whole**, plus `doubleYourOwnCounters` for
the player seat.

**Deviation — Deepglow Skate does not bench, for a reason that is not this
row's.** "…on any number of target permanents" is a bare plural recipient,
and `PerMember` — every counter row's gate, `PutCounters` included — refuses
it: the printed lines that distribute a counter operation over a group write
the word ("on each of up to four target creatures") and this one does not.
Recorded as `deepglowSkateRecipientRefused : perMemberOk (TargetGroup
anyNumber Permanent) = False`. Widening `PerMember` is a question about every
counter row at once, not about the doubling row, so it is not taken here.
Acceptance is met by Vorel, which the ticket named as the alternative.

**Item 3 — the kind-blind move under a NEGATED kind set: a verdict, at a
measured 1.** Goldberry, River-Daughter is the whole supported family
(`counter of each kind not on …` matches it and nothing else). Not minted:
`MoveCounters`' `kind : Maybe CounterKind` slot answers "does the sentence
name a kind", which cannot express a kind SET, still less a negated one; and
the exclusion is a second self-read (the DESTINATION's own kinds), where
`GiveCountersOfOwnKinds` and the new doubling row each read one holder. Two
new shapes for one line. No pin: nothing here is rules-impossible — [CR#122.5]
bounds a move by whether each half can happen and says nothing against a
kind-set complement — so this is a measured 1 and a cost judgement, not a
refusal.

**Routed ledger item — the counter partitive over a described group: measured,
not built.** `counters from among <described group>` measures at 18 supported
cards in the removal/move sense (Dawnhand Dissident, Eventide's Shadow,
Galloping Lizrog, Hierophant Bio-Titan, Hopeful Initiate, Iron Spider,
Jetfire, Light Up the Night, Novijen Sages, Ooze Flux, Overseer of Vault 76,
Quilled Greatwurm, Retribution of the Ancients, Sensational Spider-Man,
Slippery Bogbonder, Tayam, Tekuthal, The Filigree Sylex), against the
ticket's inherited "19 distinct lines"; two further "from among" lines
(Aragorn, Company Leader and Elspeth Resplendent) are the unrelated
choose-a-kind-from-a-list idiom. The shape wanted is a slice whose MEMBERS
are counters and whose domain is a described group of holders — a
`SomeOf`-shaped partitive at the counter slot, with the count taken over the
group and not per member. That is a new slice shape plus its gates, a round
of its own, and it is not one of this ticket's numbered items; it stays
routed and unbuilt. **Remainder for the coordinator.**
