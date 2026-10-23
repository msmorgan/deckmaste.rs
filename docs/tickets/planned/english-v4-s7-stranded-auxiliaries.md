---
needs: []
---
# Stratum 7: Stranded auxiliaries beyond *do*

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Extend auxiliary stranding beyond *do* and *don't*, which already read:
- modals and *be* (*If you can't*, *If they can't*, *If it is*, *If it isn't*);
- *If a player does (either)*;
- *If they don't pay*;
- relatives (*each opponent who can't discards*, *For each opponent who can't, ...*);
- *If you pay*, with the object omitted;
- the reflexive trigger as a modal header's lead (*When you do, choose one —*, [CR#603.12]; 6 faces).

Stranding every auxiliary may add stranded Readings inside longer Clauses. Measure admitted Readings before and after and report them.

## Grounding

- CGEL, Ch. 17 §7.1, pp. 1519–1523.
- CGEL, Ch. 17 §7.4, p. 1527.
- `docs/oracle-style-guide.md` §8, "Delayed and reflexive triggers".
- [CR#603.12].

## Witness cards

- Archdemon of Greed
- Brain Pry
- Aclazotz, Deepest Betrayal
- Clackbridge Troll
- Voltstorm Angel

## Estimated faces

About 36 faces (an estimate from the 2026-10-23 census, not a measured figure).
14 more faces also need `english-v4-s7-then-coordination`.

## Engine

No. Grammar only.

## Pending judgments this slice makes hold

`instead_replacement.txt:27`.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
