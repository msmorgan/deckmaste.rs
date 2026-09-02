---
needs: [english-v2-frame-key-reduction]
---
General prepositional phrase, adjunct class, and repeatable postmodifiers
(taxonomy audit A1/A2/A3 — the structural core of the v1-inventory
adoption).

RULING PREREQUISITE (lands first, user-ruled, recorded in the rewrite
ADR): the single derived attachment rule — verb-selected prepositions
attach at the frame (already ruled); every other PP/adverbial attaches to
the nearest constituent that licenses it (low attachment: an NP
postmodifier reading wins when the nominal licenses that preposition
class — "creature with flying", "card in your graveyard" — otherwise the
PP is a predicate adjunct). Two surviving readings for the same bytes is
a genuine tie and a STOP, never a preference weight.

A1: ONE PP construction with a preposition lexical slot, replacing the 13
per-preposition categories; temporal/locative/manner/purpose/duration are
values of the adjunct class, not categories. A3: one predicate-adjunct
class consuming PPs and adverbials (G2's temporal residue lands here).
A2: NP postmodifiers repeatable and order-free (v2 fixes one slot each:
relative -> locative -> with). Expected to unlock the audit's failing
ordinary probes ("Creatures with flying you control", postposed "at the
beginning of the next end step").

Acceptance: coupled replacement (each per-X family deleted with its
general construction landing), zero net coverage loss, ratchet up, zero
ties or STOP-and-report, no process artifacts in tracked source. Design
only from v1 (docs/memory/scratch/plan09-postmortem/taxonomy-v1-v2.md):
no v1 code, types, or feature vocabulary is ported. Probe set: the audit
§6 table filtered to this family. Standard constraints apply.
