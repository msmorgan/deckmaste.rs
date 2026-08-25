---
needs: []
---
# The choice frame that licenses a later read

Two ledger items from
`docs/tickets/done/workbench-amount-comparison-and-quantity.md`, both NOT
landed, both blocked on the same shape: a choice clause that mints nothing a
later clause can read back off.

## 1. Duneblast — "Choose up to one creature. Destroy the rest."

Quoted from that round's ledger (oracle text verified there):

> `TheRest`'s presupposition is a partitioned group, and a choice mints none:
> neither `countGroups` nor `countParts` sees a choice clause. Landing it needs
> either a new `TheRest` licensor over the world's creatures or a different row,
> which is a ruling this round did not have.

The pair's other half **is** landed — Berserker's Frenzy's counted choice
benches as `berserkersFrenzyLowRoll` — so the delta is exactly the complement
read, not the choice.

## 2. Boreas Charger / Sandstone Oracle / Slithermuse — deferred whole

> "Choose an opponent who controls more lands than you … the difference" needs a
> predicate-level member-relative comparison AND a gap-licensing choice frame.
> Neither exists; building half of it lands none of the three.

Two pieces, and that round's own warning is that half of them is worth nothing:

- **the member-relative comparison at predicate level** — a description picking
  a player by comparing their count against the reader's, rather than against a
  literal; and
- **the gap-licensing choice frame** — the choice that makes "the difference"
  readable afterwards.

Take them together, and note the second is the same phenomenon as §1's: a choice
that leaves a readable residue behind it.

## Neighbours

`AggregateOver`'s own binder is landed and benched; the amount round's other
relativized-count blockers (per-member event mentions, the `{X} less` cost
rider, the Suspect counter kind) are routed elsewhere and are not this round's.
The chooser POSITIONS — who may choose and what a choice may bind — belong to
[workbench-choice-chosen-and-ascription](workbench-choice-chosen-and-ascription.md);
this ticket owns only what a choice leaves behind for the next clause to read.

## Consumption boundary

`idris/src/Experimental.idr` (`Effect.Choose` and its intro, `TheRest` and its
licensor, `countGroups`/`countParts`, `CompareAmt`'s operand slots, the
predicate-level comparison), `idris/src/Experimental/Words.idr` if the
comparison needs a word of its own, the pin modules
`idris/src/Experimental/Proofs*.idr`, and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Duneblast benches whole, or the licensor question ends in a written ruling.
- All three of Boreas Charger, Sandstone Oracle and Slithermuse bench — not one
  or two; the round that deferred them said building half lands none.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
