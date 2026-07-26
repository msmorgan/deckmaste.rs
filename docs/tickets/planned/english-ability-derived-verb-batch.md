---
needs: []
---
**Deferred ability-derived verbs: `exploit`, `champion`, `evolve`,
`enlist`, `mentor`, `train`.** Each needs its own bounded design (round
kwverbs, 2026-07-25):
- **`exploit` (23 groups — the big one)**: attempted at kwverbs Gate B and
  REVERTED on a measured over-fire — Henry Wu, InGen Geneticist (`Henry Wu
  and other Human creatures you control have exploit.`): with the verb
  registered, the coordinated subject re-bracketed so `have` went
  elliptical and `exploit` split off as a second coordinated
  IntransitivePredicate — the keyword-ability atom object flipped to a verb
  reading. Colonel Autumn (uncoordinated subject) stayed correct. A
  permanent regression-guard test reproduces the failure
  (tests/public_api.rs). Re-adding needs a fix for that over-fire (why does
  the coordinated-subject path prefer the verb split? possibly the
  single-root-lowering defect, possibly a cost tie), not just a re-add —
  see the pointer comment at `ABILITY_DERIVED_KEYWORD_ACTION_VERBS`
  (catalog.rs).
- **`champion` (1 group)**: one verb witness corpus-wide (Mistbind Clique
  `is championed with`), needs a passive-with-`with`-PP frame from a single
  data point, and its only face carries a second `NoCompleteParse`
  (`Champion a Faerie` keyword-argument line) so it cannot clear anyway.
  While it stays out, `copular_complement_head_is_opaque`
  (grammar/ability.rs) is LOAD-BEARING and must not be removed.
- **`evolve` (2), `enlist` (2), `mentor` (1, strongest noun homograph —
  card names + `has mentor`), `train` (1, ability spelled `Training` vs
  verb `trains` — lemma-mismatch design question for
  `KeywordActionHead`)**: per-verb homograph analysis required; six groups
  do not justify batch-importing four unbounded homographs.
