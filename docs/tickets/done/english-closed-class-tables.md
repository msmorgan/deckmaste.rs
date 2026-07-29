---
needs: [english-structural-recovery-zero]
---
**One bidirectional spelling↔variant table per closed class, serving both
scanner and renderer.** Adjudicated from an external review 2026-07-24
(spot-verified in part). Each closed class is a bijection currently written
twice, in files that don't reference each other — adding a subordinator
means parallel edits to `grammar/mod.rs` and `renderer.rs` (the 2026-07-24
`until` addition did exactly this).

- Spelling pairs duplicated scanner↔renderer: determiners
  (`scan_determiner` ↔ `Determiner::render`), prepositions
  (`scan_preposition` ↔ `render_preposition`, ~19 arms each),
  subordinators, existentials (`there is`/`there's`/…), trigger words,
  predicate conjunctions; the demonstrative `this/that/these/those` table
  appears three times.
- Table data written as code: the scanners are long if-else chains and
  `scan_subject_auxiliary` is a 12-entry repeated tuple pattern — pure data
  that shrinks to declarative tables and merges naturally with the shared
  spelling tables above.
- Pronouns: `PRONOUN_FORMS` (word.rs:1732, ~80 lines) hand-restates
  `render_pronoun`, while the reverse index already derives the auxiliary
  entries by iterating `auxiliary_instances()` through the renderer
  (word.rs:1568) — use the identical pattern and delete the table.
- Same-spirit riders: the initial-sound vowel heuristic exists three times
  (`spelling_initial_sound`, the `Vocabulary::initial_sound` fallback,
  `is_vowel`); `contraction_suffix` dispatches on the rendered auxiliary
  string instead of the structured `Auxiliary`; and the top-level
  spaced-em-dash scan is written three-plus times in `grammar/ability.rs`
  (`peel_cost_flavor_header`, `peel_flavor_header`,
  `spaced_top_level_em_dash`, plus any flavor-word-round addition) —
  unify the scan helper the peels share.

Sequenced after the recovery campaign: these are the campaign's hot files.
Gate: representation-only — census byte-identical + clean roundtrip; exact
test-count parity. Standard constraints apply.
