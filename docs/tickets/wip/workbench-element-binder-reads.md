---
needs: []
---
# The ordinal cast's element binder, and the per-member event count inside `AggregateOver`

Routed from `workbench-event-zone-and-cast-provenance`'s §6 dispositions
(split 2026-08-26; see
`docs/tickets/planned/workbench-event-zone-and-cast-provenance.md`). The
splitter's disposition table names `workbench-amount-comparison-and-quantity`
as the target for both items below, but that ticket is
**`docs/tickets/done/workbench-amount-comparison-and-quantity.md` — closed**.
Done tickets are not plans, so both items mint here instead.

## The ordinal cast's residual 5 — a noun-phrase element binder over a turn's casts

From the split report: *"Half delivered, half misrouted."*
`GameEvent.NthOccurrence` (`Triggers.idr:348`) landed in the event-algebra
round and its own docstring gives the cast spelling verbatim — the 20
trigger-header lines ("your first spell") write today. The residual 5
(Maelstrom Nexus, Rain of Riches, Wild-Magic Sorcerer, The Twelfth Doctor,
Zimone) want a noun-phrase read — "the first spell you cast each turn" —
which the parent ticket itself described as:

> "An ELEMENT BINDER over a turn's casts — the same shape
> `workbench-amount-comparison-and-quantity`'s relativized per-element count
> wants — and the per-turn reset is a second unbuilt thing beside it."

The next-spell and first-spell keyword GRANTS ("the next spell you cast this
turn has cascade") wait on this binder. They are [CR#611.2f]'s regime, a
continuous effect that begins to apply when the player next puts an
appropriate spell on the stack; they are not the keyword row's own gap.

### The Once Upon a Time re-check

Also routed from the split report's §6 (originally from
`workbench-conditional-and-coordination`, closed): Once Upon a Time's
history identity read ("if this spell is the first spell you've cast this
game") is **dormant** — "neither built nor pinned; no rule makes it
meaningless and no buildable card pays it" (parent's own line). The split
report notes it is "worth one cheap re-check now that `NthOccurrence`
exists — 'the first spell you've cast this game' is an ordinal read, and the
ordinal wrapper landed after this item was recorded." **Assign the re-check
to whoever takes the ordinal binder** — it is not a round of its own.

## The per-member event count inside `AggregateOver`'s binder body

From the split report's §6: "Belongs with the element binder, same as the
ordinal residue." Thought Sponge's "the greatest number of cards an opponent
has drawn this turn" and the Windfall / Jace's Archivist cycle's "cards a
player discarded this way" need a per-member event subject inside a binder
body; the body admits an `Amount` but no per-member event subject exists
today. Route to this ticket together with the ordinal 5, per the split
report.

## Consumption boundary

`idris/src/Experimental/Phrase.idr` (`AggregateOver` and its binder body,
the element-binder machinery), `idris/src/Experimental/Events.idr`
(`GameEvent.NthOccurrence`, event-count readers the binder wraps),
`idris/src/Experimental/Cards.idr` bench, `idris/src/Experimental/Proofs*.idr`.
No Rust crate.

## Acceptance

- The ordinal cast's noun-phrase reading lands over the same element-binder
  shape as `AggregateOver`'s relativized per-element count, or ends in a
  written rule-backed verdict naming what blocks it; the per-turn reset is
  answered together or explicitly deferred with its own statement of what
  remains.
- Maelstrom Nexus, Rain of Riches, Wild-Magic Sorcerer, The Twelfth Doctor
  and Zimone are benched, or the residual blocker is named per-card.
- Once Upon a Time's history identity read is re-checked against the landed
  `NthOccurrence` ordinal wrapper and either benched or re-recorded as
  dormant with the current reason.
- `AggregateOver`'s binder body gains a per-member event subject, or the gap
  is re-recorded with the reason it does not fit this round; Thought Sponge
  and the Windfall/Jace's Archivist cycle bench or are named as the residual.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## Routed ledger items

- **Routed from workbench-anaphora-mentions-and-creation (split into five
  sub-tickets, 2026-08-27):** the plural read-back mention — "Each player
  scries N" is unspellable: `Each` mints a plural (`ManyOf`) player binding
  where the anaphor wants a single `Player` mention. Same per-member
  binder-read shape as this ticket's own `AggregateOver` item above; land
  together.
