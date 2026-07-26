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
  (tests/public_api.rs). Re-adding needs a fix for that over-fire, not just
  a re-add — see the pointer comment at
  `ABILITY_DERIVED_KEYWORD_ACTION_VERBS` (catalog.rs).
  **Diagnosis (round ctrsubj, 2026-07-25 — source-read, not yet run):**
  it is NOT the single-root-lowering defect — that defect destroys parses
  (`Err(Lowering)` on decline, grammar/mod.rs) and cannot manufacture a
  tree, while Henry Wu's wrong reading lowered and rendered. The lead is a
  **frame-completeness hole on a composition path**: both
  `HAVE_PREDICATE_FRAMES` entries (word.rs) require a direct object, yet
  the wrong tree had `have` as a no-object intransitive — a state
  `predicate_arguments_complete` should reject — and
  `reduce_composed_clause` (grammar/clause.rs) contains ZERO calls to
  `predicate_arguments_complete` (coordination composes already-reduced
  children). Experiment for the round that picks this up: (1) with
  `exploit` re-added locally (not committed), parse Henry Wu's line and
  dump the winning derivation's RULE CHAIN (not just the AST) to find the
  rule that admitted an object-less `have` verb phrase; (2) check whether
  that rule's features arm calls `predicate_arguments_complete` — if not,
  that is the defect and the fix is categorical (validate the frame at
  that site); (3) contrast with Colonel Autumn (uncoordinated, stayed
  correct) as the minimal pair isolating coordination; (4) only if the
  frame WAS validated is this a genuine ambiguity preferred wrongly, which
  needs categorical discrimination elsewhere — never cost tuning.
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
