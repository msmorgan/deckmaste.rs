---
needs: []
---
# Write the transform verb, and settle the face layouts the card round left out

`docs/tickets/done/workbench-multiface-cards.md` gave the card record its faces,
its five layouts and its two missing printed boxes, and closed with a ledger of
named blockers. Five of them are one region — what a face *does* and which
multi-face shapes the layout catalog still refuses — and nothing owns them.

## 1. Transform as a verb — the largest of the five

Quoted from that ledger:

> `Action::Transform`'s counterpart — a `transform`/`convert` effect and its
> trigger event [CR#701.27,701.28]; every non-Siege TDFC's front face writes one
> in printed text.

The Siege witness that round benched does not need it, which is exactly why the
verb went unbuilt. Two carriers are waiting on it and named:

- **Arlinn Kord // Arlinn, Embraced by the Moon** — box law probed
  (`planeswalkerBackWithoutLoyaltyOk`); the whole card additionally wants "up to
  one target", an emblem body and a `Wolf` subtype row.
- **Garruk Relentless // Garruk, the Veil-Cursed** — same box law, same block;
  the front's state trigger writes the verb. Also wants a `Garruk` subtype row.

Note that the verb vocabulary is now open
(`docs/tickets/done/workbench-verb-labels-open.md`): a keyword action is a data
row plus a macro. Check whether the transform verb is that shape before
designing a constructor.

## 2. Meld

> out of scope, recorded in the `Card` docstring — a two-card object;
> [CR#712.4b] and [CR#712.21] make it a different shape from a face layout.

So this is not a sixth layout constructor by default. Decide what it is.

## 3. Rooms — the unlock designations and the special action

`SplitCard` already admits a shared type line ([CR#709.5]). What is missing:

> the "left/right half unlocked" designations [CR#709.5c] and the unlock special
> action [CR#709.5e].

## 4. Fuse

> a keyword row; chose Wax // Wane over Wear // Tear to avoid it — [CR#702.102].

## 5. The Adventure exile-and-recast rider

> the "then exile this card, you may cast the creature later from exile" rider
> is reminder text on the printed card and has no row — [CR#715.3d].

Its reminder-text status is the argument for leaving it out; make that the
verdict or overturn it, but do not leave it unstated a second time.

## Neighbour, so it is not re-measured here

The Adventure/Siege re-measure the card round asked for is routed to
[workbench-description-and-player-sorted-reads](workbench-description-and-player-sorted-reads.md),
which owns the eleven-card reading in question.

## Consumption boundary

`idris/src/Experimental.idr` (the effect row and its trigger event, the
designations, the special action), `idris/src/Experimental/Words.idr` (the
layout catalog, `Designation`, any label row the verb needs),
`idris/src/Experimental/Events.idr` for the event side, the pin modules
`idris/src/Experimental/Proofs*.idr`, and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- The transform verb exists with its event, and both named planeswalker
  carriers are either benched or reduced to blockers that are not the verb.
- Meld, Rooms' unlock pair, fuse and the Adventure rider each end with a landed
  row or a written verdict naming its rule — none is left unmentioned.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-verb-label-residues (close, 2026-08-26):** manifest [CR#701.40a] — no face-down `TokenRider` arm, and nothing lists a face-down permanent's characteristics [CR#708.2]. A face/permanent-characteristics gap; manifest dread's STRUCTURE already composes from the landed look-partition, so this is its single blocker (Curator Beastie the carrier).

- **Routed from workbench-event-disjunction-seat (close, 2026-08-27):** "the creature that spell becomes as it resolves" (Illusionary Mask's subject) — the face-down/permanent-spell becoming read; a face question, so it lands here.
