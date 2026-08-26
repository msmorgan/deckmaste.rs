---
needs: []
---
# The holder-sourced "same number of each kind of counter" read

Routed from `workbench-counter-distributive-residues` (close, 2026-08-26).
The announced-batch half of the "same number and kind" axis landed there
(`PutCountersOfThoseKinds ThatMuch`, Captain Marvel benched). The
holder-sourced half did not: Denry Klin's counters come from a HOLDER whose
mention is unwritten on the surface (named only in the intervening-if), so
the shape choice — a holder slot on the row vs an anaphor over the
condition's mention — is open, not mechanical. Measured size: 1 supported
line (Denry Klin); Dominion Saboteur additionally blocked on the copy-entry
frame (routed to workbench-copy-family-residues). [CR#122.8] performs the
same read for its leaves-the-battlefield case and marks it as not a move
[CR#122.5].

## Consumption boundary

`idris/src/Experimental/Effect.idr` (the row), `Cards.idr` (bench),
`Proofs*.idr` if a pin falls out. No Rust crate.

## Acceptance

- Denry Klin benches whole or the shape question ends in a written ruling.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Standard constraints applied. `idris/scripts/build` PASS from a clean
`build/` — 23/23 modules, 0 errors, 0 warnings; every `Proofs*` module
re-elaborated and no pin silently passes (none added: the row introduces no
new refusal, only `PerMember` on its destination, already pinned).

### The shape ruling

**The source is a slot, not an anaphor.**

> Seven of the eight supported lines that perform this read write the source
> on the effect's own surface — "put **its** counters on [dst]". Only Denry
> Klin elides it, and what fixes the referent there is [CR#201.5] — text
> referring to the object it's on by name means that particular object
> ([CR#201.5c] for the shortened "Denry") — not a discourse mention. An
> anaphor gate records a mention and counts it; [CR#201.5] fixes the referent
> whether or not there is a mention to count, so a `countOutcomes`-style
> presupposition would be recording the wrong fact about why the clause
> reads. The intervening "if" is a [CR#603.4] gate on the ability, checked
> twice and otherwise inert; it is not the effect's antecedent. One row with
> a written `src`, therefore, and Denry fills it with the self-reference its
> condition names.

The re-measure is what settled it. The parent round measured the exact
phrase "same number of each kind of counter" and found Denry alone; measuring
the *operation* instead ("put its counters on") finds the same read spelled
with its source written, across a family the size of which makes the elision
Denry's spelling quirk rather than the construction's shape.

### The row

```idris
PutSameCounters : (src : Noun bs Object) ->
                  (dst : Noun (nomIntro src) Object) ->
                  {auto 0 pm : PerMember dst} -> Effect bs
```

Grounding: [CR#122.8] and [CR#122.9] write this operation out in rules text
and say in so many words that it is not a move [CR#122.5] — "the player
doesn't move counters from one object to the other. Rather, the player puts
the same number of each kind of counter the first object had onto the second
object." That settles the gates, which are the row's whole difference from
`MoveCounters`:

- **no `CounterMemory` on `src`.** [CR#122.5]'s impossibility list bounds a
  move; this is not one. The rule is written *for* a source that has left the
  battlefield, and `CountersOn` already reads a dead referent's counters
  (Vexing Sphinx) without that gate.
- **no `MoveDestination` on `dst`.** Same reason: the same-object refusal is
  a move's, [CR#122.5]'s first listed case.
- **no kind slot, no amount slot.** What `src` holds is both the kinds and
  the counts, so neither is a slot — the contrast with
  `PutCountersOfThoseKinds`, which carries `amt` because a card can say
  "twice that many".

Rows added to all nine total tables (`heldUntilOk`, `reflexEncloseUse`,
`thisWayOutcomeOk`, `costActionOk`, `effEq`, `effIntro`, `preIntro`,
`annIntro`, `deedDelta`), each following its `MoveCounters` neighbour;
`costActionOk` is `costNounOk src && costNounOk dst` for `MoveCounters`'
reason — the clause names its own objects, so nothing about a cost position
stops it.

Tolerated overgeneration, named at its zero: a plural `src`. Nothing in
[CR#122.1] makes summing two holders' counters meaningless — counters of the
same name are interchangeable — and nothing writes one.

### Family, re-measured

Measured this round with the corpus scripts. The holder-sourced read at the
parent round's phrasing is Denry Klin alone, as recorded. Measured as the
operation, `PutSameCounters` covers a second spelling — "put its counters on
[dst]" — whose lines are all supported, across Broodguard Elite, Buzzard-Wasp
Colony, Dockworker Drone, Enduring Bondwarden, Essence Channeler, Hei Bai,
Heroic Sacrifice, Host of the Hereafter, Parish-Blade Trainee, Sin, Spiteful
Squad and Star Pupil. One node, two spellings, as
`PutCountersOfThoseKinds` already is.

### Cards and phrases benched

- **Star Pupil** (whole) — `starPupil`. The entry counter plus
  `PutSameCounters It (target creature you control)`; `It` names a creature
  that has left the battlefield, the case [CR#122.8] is written for, and the
  bench is the witness that this row does not carry `CounterMemory`.
- **Denry Klin, Editor in Chief** (the trigger) — `denryKlinSameKinds`,
  under `triggeredIf` with `Matches thisCreature (HasCounters Nothing)` as
  the intervening clause and `That (TypeW Creature)` as the recipient.
  Not whole: the entry line, "enters with your choice of a +1/+1, first
  strike, or vigilance counter on it", has no row — no entry mark chooses
  its kind.

### Ledger

- **The chosen counter kind** — Denry Klin's first line, and a family in its
  own right. `EntersWithCounters` and `PutCounters` each take one
  `CounterKind`; "your choice of a +1/+1, first strike, or vigilance counter"
  picks one from a written menu. `EntersChoice` is the nearest row and does
  not reach it: `QualitySort` has no counter-kind member, and `ChoiceDomain`
  has no menu constructor — its four domains are all open-ended
  ("other than", "above"), where these lines write a closed list of two to
  four kinds. Measured this round: twenty-one supported lines over
  thirty-three cards, ten of them entry-side ("enters with your choice of …",
  which is riot's own reminder and a run of two-kind creatures beside Denry
  Klin) and the rest the plain put. Two spellings for the menu — "your choice
  of a X, a Y, or a Z counter" and Aragorn's "your choice of a counter from
  among X, Y, Z, and W" — and Bribe Taker writes one whose second arm is a
  kind read off the board rather than a listed kind. This is the only thing
  between Denry Klin and a whole card, and it wants its own round.
- **Dominion Saboteur** — "it enters with additional counters on it equal to
  the same number and kinds of counters the copied permanent has on it"
  writes this read into `ExceptEntersWithCounters`, on the copy-entry frame
  owned by `workbench-copy-family-residues`. Not chased here; the read itself
  is now `PutSameCounters`'s, so only the entry frame is outstanding.
