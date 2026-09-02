---
needs: [english-v2-general-predication]
---
Polarity and card-type as features, not constructions (taxonomy audit
A8). 24 of 34 nominal modifiers are per-card-type constructions and
negation ("non-", "not") is carved per construction; make card type and
polarity derived features on the general modifier/nominal constructions,
deleting the per-type and per-polarity families. Runs last so it sweeps
the general shapes the three prior chunks land. Note the ADR-blessed
34-construction card-kind NOUN partition (english-v2-rewrite.md
~:547-551) is a separate, kept decision — this ticket is about
MODIFIERS; revisiting the noun partition needs its own ruling.

Acceptance: coupled replacement (each per-X family deleted with its
general construction landing), zero net coverage loss, ratchet up, zero
ties or STOP-and-report, no process artifacts in tracked source. Design
only from v1 (docs/memory/scratch/plan09-postmortem/taxonomy-v1-v2.md):
no v1 code, types, or feature vocabulary is ported. Probe set: the audit
§6 table filtered to this family. Standard constraints apply.
