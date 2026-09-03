---
needs: [english-v2-homograph-feature]
---
Narrow the genitive possessor (restoration-landing-review M1).
`possessive_singular_nominal` — added by the restoration for the
demonstrative-possessive reroute, undisclosed in its record — admits a
bare singular count noun as possessor: "Destroy creature's controller."
selects while "Destroy creature." correctly fails. The possessor position
takes a determined reference phrase (or a proper/mass nominal), same
licensing as any other singular count nominal position; keep the
demonstrative possessive route it was added for. Also: the restoration's
+7 came entirely from the unlisted `genitive_determiner_coordination_
reference`; keep it, document it. Probe: the sentence above rejects;
"target creature's controller", "that creature's owner", "their owners'
libraries" select. Zero net loss, lock current. Metric rule: `literal_lexicon_collisions` (72) movement is accounted row-by-row, never bumped. Fixture hazard: synthetic surfaces owned by an unlicensed vocab member fail to load; use novel surfaces. Standard constraints apply.

## Landing record (2026-09-03)

- Construction count: 378 -> 378. `possessive_singular_nominal` now admits
  only proper singular nominals; ordinary singular-count possessors therefore
  enter through the same determined-reference path as other noun phrases, while
  the existing proper- and mass-nominal exceptions remain available. The
  demonstrative possessive construction is retained, and common-count
  demonstratives such as `that creature's` now use the licensed general
  genitive-determiner route.
- Coverage and lock state: selected and covered units 16,174 -> 16,237 (+63),
  parse failures 16,467 -> 16,404, and selected-uncovered, unresolved,
  internal, round-trip, ownership, gap, overlap, synthetic, and provenance
  counters remain zero. The 63 additions are attested determined possessors in
  scalar values; no covered identity was lost. The lock now contains 16,237
  identities (SHA-256
  `482785cac240a055b484152861421caf19799e10fdc6667ce239db0df78d66a8`).
- Collision accounting: `literal_lexicon_collisions` is 72 -> 72, with the
  exact row set unchanged. No form-literal, vocabulary, or lexeme surface was
  added, removed, or reclassified, so there are zero added and zero removed
  collision rows; the metric was not bumped.
- Restoration accounting: `genitive_determiner_coordination_reference` is
  retained unchanged. The restoration's previously unlisted +7 coverage came
  entirely from that construction.
- Probe artifacts: `Destroy creature's controller.` and the bare control
  `Destroy creature.` reject; `Destroy target creature's controller.`,
  `Destroy that creature's owner.`, and `Destroy their owners' libraries.`
  select uniquely. `Destroy Merfolk's controller.` and `Destroy control's
  owner.` preserve the proper- and mass-nominal exceptions. Fixtures use the
  novel context surface `Novel Context`.
- Assurance: restored 0; re-spelled 1
  (`finite_subject_coordination_and_exclusion_are_linguistic_structure`, whose
  exact visitor path now records the general genitive-determiner structure);
  ignored 0; added 1
  (`genitive_possessors_require_a_licensed_noun_phrase`); removed 0.
- Positive artifacts: the complete `deckmaste_english_v2` and
  `deckmaste_construction_core` test suites are green; the focused coverage and
  coverage-lock tests are green; `cargo xtask english_v2 coverage --check` is
  green; clippy is green for construction-core, English v2, and xtask across
  all targets with warnings denied; formatting check is green. The workspace
  suite reaches the known unrelated `deckmaste_construction` trybuild failure:
  four of 28 expected-stderr snapshots receive pre-existing `parser-metrics`
  cfg warnings, the same limitation already recorded by landed tickets.
- Deviations and additions: `genitive_scalar_value` no longer owns a fixed
  leading `the`; the determiner belongs to its `Possessive` child. This was
  necessary to apply ordinary possessor licensing compositionally and preserves
  the zero-loss requirement while admitting the 63 attested determined
  possessors. No construction was added or removed.
- STOPs: none.


## Erratum (genitive landing review, 2026-09-03)

Selection census shift unquantified in the record: unique 10,487 -> 10,731,
specificity-resolved 5,687 -> 5,506 — 205 already-covered units changed
their winning analysis when the demonstrative-possessive candidate stopped
competing. Residue: bare SUBTYPE possessors still select ("Destroy
Goblin's controller.") — the narrowing reads the proper-classified
vocabulary only; `possessive_singular_nominal` and
`demonstrative_possessive_reference` are now corpus-inert.
