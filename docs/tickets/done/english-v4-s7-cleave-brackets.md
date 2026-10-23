---
needs: []
---
# Stratum 7: Cleave bracketed text

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read Cleave's bracketed optional text (*Destroy target [attacking] creature.*). The text must read with and without the brackets, and realization must rebuild the brackets.

## Grounding

- [CR#702.148a].
- `docs/oracle-style-guide.md` §3, "Bullets, vertical bars, and square brackets".

## Witness cards

- Lantern Flare
- Winged Portent
- Path of Peril
- Fierce Retribution
- Wash Away

## Estimated faces

About 11 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

Likely.

## Pending judgments this slice makes hold

None are pending against this slice. Add a positive judgment for each witness (`crates/deckmaste_english_v4/judgments/`) asserting its intended analysis.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

Census delta (measured on `orvwrtxl` → `xplzttyr`): faces read 32,575 → 32,586 (Lantern Flare, Alchemist's Gambit, Winged Portent, Inspired Idea, Fierce Retribution, Alchemist's Retrieval, Wash Away, Lunar Rejection, Path of Peril, Parasitic Grasp, Dread Fugue: 0 → 1–8 Readings each, the only units that changed; Dig Up stays unread); total Readings 315,953 → 315,978 and Surplus 132,429 → 132,439. Lost faces: none.
Judgment set (`xplzttyr`): 3,108 enforced and 284 pending, all enforced holding; the 15 new pending entries of `cleave_bracketed_text.txt` (Stratum 7) all hold.
