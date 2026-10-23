---
needs: []
---
# Stratum 7: Clause coordination inside a preposed clause

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Tell a comma-delimited Clause coordination's comma apart from a comma-set post-dependent's comma inside a preposed clause. Witness: Mishra, Claimed by Gix, *If [Mishra, Claimed by Gix and a creature named Phyrexian Dragon Engine are attacking, and you both own and control them], exile them*. It breaks the formation rule of `english-v4-comma-scope-nodule-1` (a dependent set off before its head holds no unclosed comma). Probe for other real Oracle text with the same shape while landing it.

This slice unlocks no face alone: Mishra also needs `english-v4-s7-then-coordination` for its `, then meld ...`. Land it beside that slice, which touches the same comma machinery.

## Grounding

- `docs/oracle-style-guide.md` §3, "Commas and the serial comma".
- `docs/decisions/english-v4.md` §4.4, principle 1.

## Witness cards

- Mishra, Claimed by Gix

## Estimated faces

About 0 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

Yes (certain): the `internal_comma` formation rule in `crates/deckmaste_syntax/src/step.rs`.

## Pending judgments this slice makes hold

None are pending against this slice. Add a positive judgment for each witness (`crates/deckmaste_english_v4/judgments/`) asserting its intended analysis.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
