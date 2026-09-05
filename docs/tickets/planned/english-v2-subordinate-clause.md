---
needs: [english-v2-grammar-migration-design, english-v2-np-postmodifiers]
---
> **Migration routing (2026-09-05).** This unclaimed ticket waits on
> `english-v2-grammar-migration-design` under the
> [Lean design decision](../../decisions/english-lean-design-workbench.md).
> That design task must reconcile and repin this ticket before it becomes
> executable. The prior body below preserves examples, regression and
> re-coverage obligations, and proposed mechanisms; its old sequence,
> implementation prescriptions, and coverage-ratchet acceptance do not
> override the new design process or the current landing contract.

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
general clause-tail seam ahead of this ticket. It factors the current `if`,
`unless`, `as`, `as long as`, `for as long as`, `while`, `until`, and general
Predicate Adjunct tail shapes from their preposed/postposed attachment hosts
into one member construction each, gathered by `PreposedClauseTail` and
`SimplePostposedClauseTail` / `PostposedClauseTail`. Build the remaining
lexical-subordinator and Finiteness work on those categories; do not re-derive
a second tail family.
