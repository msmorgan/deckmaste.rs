---
needs: [english-v2-polarity-feature]
---
Card type as a derived feature on nominal MODIFIERS (A8b): 24 of 34
modifiers are per-card-type constructions; collapse to one modifier
construction with the type as a feature. The ADR-blessed 34-construction
card-kind NOUN partition (english-v2-rewrite.md ~:547-551) is a separate
kept decision, out of scope here.

Acceptance: coupled replacement (each per-X family deleted with its general
construction landing), zero net coverage loss, ratchet up, zero ties or
STOP-and-report, no process artifacts in tracked source. Design only from v1
(docs/memory/scratch/plan09-postmortem/taxonomy-v1-v2.md) — no v1 code, types,
or feature vocabulary. Probe set: the audit §6 table filtered to this family.
Standard constraints apply.
