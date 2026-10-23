---
needs: []
---
# Stratum 7: Verbless *with*/*without* small clauses and *repeat this process*

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read:
- verbless *with* or *without* small clauses (*Play with your hand revealed*: Enduring Renewal, Field of Dreams, Future Sight, Telepathy; all four are known-wrong-only today);
- *Repeat this process* (Eureka, known-wrong-only; `starting_with_adjunct.txt:3`).

## Grounding

- CGEL, Ch. 14 §10, pp. 1266–1267.

## Witness cards

- Enduring Renewal
- Field of Dreams
- Future Sight
- Telepathy
- Eureka

## Estimated faces

About 5 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

No. Grammar only.

## Pending judgments this slice makes hold

`starting_with_adjunct.txt:3`.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
