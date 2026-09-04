---
needs: [english-v2-stage-5-grammar-buildout-13-10]
---
Flavor words as an open declared class. Saga chapters carry an optional
second label (`II, III — Brimstone — Add {R}{R}{R}{R}.`, 21 faces) and
some modal modes carry a faction label (`• Khans — …`, `• Dragons — …`,
71 faces). Flavor words have no rules meaning and no inventory today.

The grammar side is already additive: `BlockLabel` is the label sum
(ability word, chapter numerals) and a mode marker precedes a mode body;
add one flavor-word arm to each fed by a new `flavor_words` stub family in
`plugins/builtin_v2` (declaration term, like ability words). Inventory
from the corpus census, never hand-typed; no vocab. Standard constraints
apply.

## Landing record

Measured on change `korsquwvysnx` with 16,702 covered lock identities. The
measured lock SHA-256 is
`837688617caf0b0ff0c3e6551fcf2d58772e463f71040c1c42e6d51f2cb26446`.

| gate | requested baseline | measured tree |
| --- | ---: | ---: |
| selected and covered units | 16,385 | 16,702 |
| ordinary parse failures | 16,256 | 15,939 |
| unique selections | 10,962 | 11,138 |
| specificity-resolved selections | 5,423 | 5,564 |
| unresolved ties | 0 | 0 |
| construction declarations | 388 | 398 |
| coverage-lock identities | 16,385 | 16,702 |

- Coverage and lock state: the complete requested-baseline-to-measured-tree
  lock diff is **+317/-0 rows**, from 49,035 lines and SHA-256
  `25f3bd5f47db192653a09fd8834d86654515a9ae551b1b9e4ff0e327a706ae27`
  to 49,352 lines and the measured hash above. The required refresh brought a
  16,555-row parent lock; this feature is **+147/-0** against that refreshed
  parent. Its isolated pre-refresh measurement was +146/-0; one further row
  (`32690080af8bf12b0972b3f1b85cb22598e9fd102c2cb5812b48091e754794d3`)
  becomes covered only in composition with the concurrently landed quoted
  terminator and was blessed after refresh. No retirement manifest was
  created or used. Selected-uncovered units, internal failures, exception
  resolutions, exception uses, round-trip mismatches, ownership failures,
  gaps, overlaps, synthetic claims, and provenance-plan mismatches are all
  zero. Literal/lexicon collisions remain 59.
- Selection census: across the requested baseline and measured tree, unique
  selections move 10,962 -> 11,138 and specificity-resolved selections move
  5,423 -> 5,564. The independently measured refreshed parent has 16,555
  selected units and 394 construction declarations; the final tree has
  16,702 and 398, so this feature contributes +147 selected units and four
  constructions after refresh.
- Declaration census: the Vintage-playable corpus snapshot contains 631
  distinct attested label surfaces in 651 label occurrences on 492 faces.
  Ordinary labelled blocks account for 441 surfaces/446 occurrences/400
  faces; Saga chapter secondary labels account for 38/38/21; labelled modal
  modes account for 154/167/72. `Domain` is already declared as an ability
  word, leaving exactly 630 disjoint new `FlavorWord` declarations. The
  inventory integration test derives that set independently from the flavor
  catalog and those three syntactic positions and checks exact equality with
  the nursery; no hand-authored vocabulary list is used.
- Positive artifacts after refresh: `cargo fmt --all -- --check`; strict
  all-target Clippy for `deckmaste_construction_core`,
  `deckmaste_english_v2`, and `xtask`; complete tests for
  `deckmaste_construction_core` and `deckmaste_english_v2`; `cargo test
  --workspace`; `cargo xtask english_v2 ambiguity --require-resolved --json`;
  and `cargo xtask english_v2 coverage --check --json` all exited zero. The
  corpus gates emitted only their non-failing busy-host performance warning.
- Assurance census: restored 0; re-spelled 0; ignored with blockers 0; added
  3 test functions; removed 0. The additions authenticate the census-derived
  declaration set, all three label positions with exact rendering and
  ownership, and rejection of an undeclared label.
- STOPs: the first corpus gate exposed 30 unresolved readings of the Saga
  secondary label `Domain`. Investigation showed that the surface already has
  the existing `AbilityWord` declaration required by the corpus. Resolution
  was to keep that identity out of the new family and make the optional
  chapter-secondary label a disjoint sum of ability-word and flavor-word
  declaration terms. The rerun and every post-refresh gate report zero ties;
  no project-ruling contradiction or lexeme-named grammar guard was shipped.

### Deviations and additions

- The ticket's 71-face modal estimate is 72 faces in the current frozen
  corpus snapshot. The generated family follows the measured snapshot and
  records the 154 distinct modal labels rather than preserving the estimate.
- The two requested flavor-word arms are represented by
  `flavor_word_label` and `flavor_word_mode_marker`. Supporting Saga secondary
  labels without duplicating `Domain` adds the abstract
  `ChapterSecondaryLabel` sum and two typed constructions,
  `chapter_secondary_ability_word_label` and
  `chapter_secondary_flavor_word_label`; no construction was removed.
- A distinct `FlavorWord` declaration kind, its fixed-term codec, compiler
  plumbing, public AST exports, diagnostics, meta declaration, and the 630
  census-derived nursery files are added so flavor words remain an open
  declaration class rather than vocabulary.
- The two non-alphabetic spellings (`... Catch` and `10,000 Needles`) carry
  explicit consonant-onset metadata; the other declarations use the ordinary
  derived onset.
- `deckmaste_construction_core` gains a test-only `deckmaste_data` dependency
  so the declaration-set assertion is derived directly from the frozen
  catalogs and card corpus instead of mirroring the generated files.
- The required refresh used jj-sensei harmony to merge three adjacent sorted
  additions in the generated coverage lock. The concurrent source edits
  merged without conflict, and inspection retained this feature strictly in
  the declaration codec, `ModeMarker`, and `BlockLabel`/chapter-secondary
  regions of `constructions.rs`.
- An intermediate full-workspace run stopped at the concurrent
  `deckmaste_construction` compiled-consumer red fixture because its local
  `ParserEnvironment` did not yet provide `declaration_noun_features`. The
  final required refresh incorporated that ticket's fix without changing any
  flavor-word metric, and the final `cargo test --workspace` run passes.
