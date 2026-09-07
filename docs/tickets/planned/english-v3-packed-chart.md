---
needs: [english-v3-lexical-model]
---
# Pack incomplete and complete v3 derivations

Replace the prototype chart's explicit `PartialFamily` child vectors with a
packed representation for incomplete as well as completed derivations. Earley
items identify recognizer state; shared intermediate nodes or equivalent packed
edges retain alternative histories without enumerating their Cartesian product
through the agenda. Completed nodes preserve every family needed to recover a
Reading lazily.

Consume position-indexed lexical occurrences from `deckmaste_lexical`; retain
overlapping and multiword alternatives without retokenizing or contextual POS
selection. Define grammatical summary identity by future admissibility: two
constituents may share a packed node exactly when every possible parent would
treat them alike. Preserve open agreement, frame, extraction, sharing and
recoverability constraints until their governing context is available. Surface
positions and spelling evidence remain on lexical/literal leaves rather than in
grammatical summaries.

Exercise prediction, scanning, completion and nullable productions, including
shared-child growth after an item was first observed. Distinguish one Reading
with duplicate derivations from two grammatical Readings. Generated Reading
identity consists of lexical identities, constituent structure and grammatical
feature values; distinct source spellings remain evidence on their leaves.
Deduplicate equivalent derivations by that identity during lazy
materialization, never by materializing values to decide chart admission. Lazy
materialization must also detect cyclic derivations.

Acceptance measures lexical alternatives, items, intermediate nodes, completed
nodes, families, completion work and requested derivations on homographs,
agreement, overlapping multiword forms, nullable rules, ambiguous attachment
and a correlated-alternative counterexample. State the actual complexity bound
provided by the chosen representation and disclose where grammar/feature
cardinality enters it; do not claim cubic behavior from Earley ancestry alone.
Standard constraints apply.
