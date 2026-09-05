---
needs: [english-v2-np-postmodifiers, english-v2-lexeme-owned-verb-frames]
---
One subordinate-clause construction with a subordinator lexical slot (A4) —
v1 had 15 subordinators in one slot; v2 has 25 constructions (per
subordinator x finiteness) — both counts unstamped, re-measure at claim.
Finiteness is a feature, not a construction.

Acceptance: coupled replacement (each per-X family deleted with its general
construction landing), zero net coverage loss, ratchet up, zero ties or
STOP-and-report, no process artifacts in tracked source. Design only from v1
(docs/memory/scratch/plan09-postmortem/taxonomy-v1-v2.md) — no v1 code, types,
or feature vocabulary. Probe set: the audit §6 table filtered to this family.
Standard constraints apply.

2026-09-05: `english-v2-tail-restrictive-focus-adverb` introduces the minimal
general `ClauseTail` seam ahead of this ticket. It factors the current `if`,
`unless`, `as`, `as long as`, `for as long as`, `while`, `until`, and general
Predicate Adjunct tail shapes from their preposed/postposed attachment hosts.
Build the remaining lexical-subordinator and Finiteness work on that category;
do not re-derive a second tail family.
