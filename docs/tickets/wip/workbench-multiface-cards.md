---
needs: []
---
# Give the card record its faces, its layouts and its missing printed boxes

The workbench's `Card` is one face. Every multi-face layout the rules define is
undescribable, and the card record is also missing two printed boxes that are
not face questions but sit in the same record.

## Context

Source: [the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md),
axis 20 (2026-08-24) — graded **MISSING**, and "Not recorded", which is why this
is a ticket. Delta only: the card-level laws (`CardLine`, `CardText`,
`CardChapters`, `CardPt`, `CardCost`, `Experimental.idr:5367-5389`) already
exist and are the frame a face layout has to fit into.

## From the v1 comparison (2026-08-24)

> Crate: `Card::TwoFaced { layout: FaceLayout, front: CardFace, back: CardFace }`
> (`card.rs:87`) with `FaceLayout { Transforming, ModalDfc, Split, Adventure,
> Flip }` (`card.rs:59`), each CR-cited, and `CardFace` (`card.rs:18`) as a full
> characteristic set including `defense`.
>
> Workbench: `record Card` (`Experimental.idr:5391`) is
> `{ name, cost, supers, line, text, pt }` — one face, no `loyalty`, no
> `defense`, no layout, no second face. `Action::Transform` has a workbench
> counterpart only as `StatusVal` face-change; the *card shape* does not exist.
> Not recorded.

Verified in tree at `Experimental.idr:5391-5398`: the six fields are exactly as
described. `Characteristic` carries `Loyalty` (`Words.idr:50`) and
`LoyaltyCost` exists (`Words.idr:1905`), so loyalty is readable and payable but
has no printed box on the record to be read *from*.

## The five layouts, each its own rule

The crate's five layout values are five different rules, not one shape with a
flag, and each names what a "face" is for it:

- non-modal double-faced cards, which transform or convert [CR#712.2];
- modal double-faced cards, whose faces are usually independent [CR#712.3];
- split cards, two faces on one card with a normal back [CR#709.1];
- adventurer cards, a two-part frame with a smaller frame inset in the text box
  [CR#715.1];
- flip cards, a two-part frame where the upside-down text gives alternative
  characteristics [CR#710.1].

Decide whether these are one record with a layout tag (the crate's shape) or
five, and decide it against what the *text* of each face may say about the
other. The flip verb is already partly here:
[workbench-trigger-and-interception-residues](workbench-trigger-and-interception-residues.md)
records `SetStatus`/`statusEffectOk Flipped` open at 16 measured occurrences and
that flipping is one-way, so the flip layout's transition half is landed and its
*card shape* is not.

## The two missing printed boxes

Not faces, but the same record, and each already has a named payer:

- **Starting loyalty.** `Card` has no slot for it.
  [workbench-copy-family-residues](workbench-copy-family-residues.md) records
  the starting-loyalty readback twice over — "finding 639's unmodeled box a
  second time" (Ob Nixilis, the Adversary) — so the box is what both readbacks
  wait on.
- **Defense.** The crate's `CardFace` carries it; the workbench has no
  counterpart at all.

## The named payer already on the board

[workbench-description-and-player-sorted-reads](workbench-description-and-player-sorted-reads.md)
counts, among eleven supported cards for one reading, ten stopped elsewhere —
one of them "on an Adventure face". Adventure is the cheapest layout to name a
blocked bench line for; re-measure before scoping, and prefer a layout with a
whole card behind it over one with only a shape.

## Consumption boundary

The Idris workbench only: `idris/src/Experimental.idr` (`record Card` and the
card-level laws `CardLine`, `CardText`, `CardChapters`, `CardPt`, `CardCost`;
any face-scoped re-statement of them), `idris/src/Experimental/Words.idr`
(`PrintedStat` and a defense/loyalty box if either lands; a layout catalog),
the pin modules `idris/src/Experimental/Proofs*.idr`, evidence bench
`idris/src/Experimental/Cards.idr` — every existing entry is a single-face term
and must keep elaborating unchanged. No Rust crate.

## Acceptance

- The layout decision (one tagged record or five) is recorded where `Card` is
  defined, with the rule each layout answers.
- Every card-level law is re-stated per face or explicitly scoped to the whole
  card; none is left silently applying to one face of two.
- All 329 existing bench entries elaborate unchanged after the record grows.
- At least one multi-face card benches, named before the round starts; a layout
  that lands no card says so.
- Starting loyalty and defense land as printed boxes or are recorded as
  deliberately deferred with the readbacks that wait on them named.
- The flip layout reuses the landed `Flipped` status transition rather than
  minting a second one.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
