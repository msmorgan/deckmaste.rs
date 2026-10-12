---
needs: [grammar-size-report-and-ratchet]
---
# Analysis key in the census

**Not claimable until the owner has approved the written key definition below and it has been pasted into this ticket.** The definition is PENDING.

Give every reading a second identity, its analysis key, and record it in the census. The tree stays the Reading; the key says which head each dependent attaches to, and in what function. This is the instrument that decides what a later change may remove: the approved cases (order of applying modifiers to one head, grouping within a same-coordinator chain, clause versus verb-phrase coordination with nothing to tell them apart) are those where trees differ and the key does not.

A Reading value carries constructor name, form index and named ordered fields (`crates/deckmaste_construction_v3_core/src/emit.rs:692–711`), not declared head or function annotations, so computing a key needs a substantive per-construction mapping. The definition must say how it is declared and where it lives.

## Definition to be approved (PENDING)

A short written definition, approved by the owner and pasted here before this ticket is claimable, must settle:

- occurrence anchors: two occurrences of one lexeme in a text have distinct identities, anchored to the word's position in the text;
- scope: how modals, determiners and non-intersective modifiers are represented, so that distinct scope analyses keep distinct keys;
- coordination hosts: which node hosts a coordination and how shared material is owned;
- gaps and shared dependents: how a dependent that serves two heads, or a gap, is recorded;
- frame roles: how a selected-frame complement is named;
- what happens for a construction with no mapping: the key computation fails loudly for that face and the census reports it; it never falls back to a lossy key.

The key must not erase differences that are not approved for removal: a differing lexeme or form, a different head, a different function stay distinct keys, so "Destroy target creature with flying." keeps its two readings and "cast" keeps present and preterite.

## Change

- Compute the key for each enumerated reading, as the definition states.
- The census records, per unit, the number of trees and the number of distinct keys, and `diff` reports the set of keys before and after.
- A candidate renderer prints a reading in readable bracketed form with heads and functions; the judgment set and its judging process need it.
- One census run then gives the answer to how much of the 724,921 readings is derivational artifact. Report it.
- Tests: a positive test for exactly each of the three approved collapses (two trees, one key), and a negative test for every distinction the definition protects (modal scope, determiner scope, non-intersective modifier scope, attachment to different heads, coordination host, gap versus no gap, repeated lexeme) showing distinct keys.
- Output stays under `target/english-v3/`. The landing updates the census description in `crates/deckmaste_english_v3/README.md`.

## Gate

Reading digests identical on every face, enforced by the census check mode from `grammar-size-report-and-ratchet` for an identical-digest landing. Report: census delta (none); size delta per declaration kind from the size tool; the judgment-set line does not apply yet.
