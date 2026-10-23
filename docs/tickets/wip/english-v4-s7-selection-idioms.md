---
needs: []
---
# Stratum 7: *From among*, *at random* and *chosen this way*

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read *from among*, *at random* and *chosen this way* postmodifiers and placements: Gorex, the Tombshell; Eye of Duskmantle; Expand the Sphere; Niv-Mizzet Reborn; Distended Mindbender; Crabomination; Chaos Defiler; Arcum's Whistle; Impact Resonance; Fork (*except that the copy is red*); Cataclysm.

## Grounding

- `docs/oracle-style-guide.md` §12, "Look, reveal, and the top of a library"; §5, "This way".

## Witness cards

- Gorex, the Tombshell
- Eye of Duskmantle
- Expand the Sphere
- Fork
- Cataclysm

## Estimated faces

About 11 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

No. Grammar only.

## Pending judgments this slice makes hold

None are pending against this slice. Add a positive judgment for each witness (`crates/deckmaste_english_v4/judgments/`) asserting its intended analysis.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
