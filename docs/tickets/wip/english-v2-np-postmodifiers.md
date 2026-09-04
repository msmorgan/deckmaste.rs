---
needs: [english-v2-adjunct-class]
---
Repeatable, order-free NP postmodifiers (A2) — v2 fixes one slot each
(relative -> locative PP -> with); replace with one postmodifier position
that repeats, attachment per the recorded rule. Expected unlock: "Creatures
with flying you control" and the with-postmodifier reminder bodies.

Baselines: re-measure at claim (the lock's covered count and construction
count on the tree you claim from — several landings are integrating
concurrently); acceptance is >= that count AND lock diff -0 rows.

Carried from the adjunct-class review (binding):
- H1: delete the `head != CoreVerbIdentity::Control` blacklist (and its
  Seedborn Muse comment); declare the adjunct licence on the verb's
  valence row (core_verbs.ron / KeywordAction grammar) and derive
  attachment from it. Probes: "Destroy each creature you sacrifice during
  your upkeep." attaches `during` to the predicate; "Untap all permanents
  you control during each other player's untap step." still correct; four
  must-not-move NP-postmodifier witnesses of your choice from the corpus.
- M1: fold the four byte-identical host adapters into one construction
  with a host role (abstract sum); expected count -3.
- M3: the contracted-perfect and reduced-passive `opt PredicateAdjunct`
  slots are unguarded — guard them by the declared licence (the DSL cannot
  `checked by` an `opt`; find the general route or STOP with the limit).
- Decide and pin: "Draw a card after your library." rejects (after takes
  no object complement) — negative oracle.
- Restore the two ordinal assertions the adjunct landing weakened.
- Assurance line adds "assertions weakened" and "negative->positive"
  counts with the strings named.
- STOP FENCE: any guard naming a verb, noun, preposition, construction, or
  card identity is STOP-and-report (CLAUDE.md).

Acceptance: coupled replacement (each per-X family deleted with its general
construction landing), zero net coverage loss, ratchet up, zero ties or
STOP-and-report, no process artifacts in tracked source. Design only from v1
(docs/memory/scratch/plan09-postmortem/taxonomy-v1-v2.md) — no v1 code, types,
or feature vocabulary. Probe set: the audit §6 table filtered to this family.
Standard constraints apply.
