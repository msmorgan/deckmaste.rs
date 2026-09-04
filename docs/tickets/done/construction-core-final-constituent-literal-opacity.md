---
needs: [construction-core-final-constituent-primitive]
---
Final-constituent primitive: trailing form literals must be opaque (final-
constituent landing review F1, HIGH). In the compiler's rightmost-leaf fold
(`crates/deckmaste_construction_core/src/emit/final_constituent.rs`), the
`AtomPlan::Literal` (and `SentenceInitialLiteral`) arm emits `fallback`, so a
trailing literal is transparent and the predicate reports the leaf BEFORE it.
Live defect: `Target creature gains "When this creature dies, draw a card."
instead` — no final period, quoted block not surface-final — is SELECTED via
`AbilityBodyQuoteTerminatedStatement` and round-trips. The deleted walker's
`_ => false` rejected it. Zero corpus exposure, so every gate is green.

Fix: `Literal`/`SentenceInitialLiteral` → `false` (a literal IS a leaf and is
never the sought category); keep `Bound`/`Circumfix` transparent (that is what
makes `quoted_ability` resolve). Adjacent `VerbFixed`/`OpenDeclaration` arms
already return `false`. Add an emitter fixture with a separate trailing literal
asserting the predicate is false there, and an english_v2 `is_err()` negative
on the sentence above (this closes the property the prior F3 left unpinned:
the existing two `is_err()` lines reach the predicate but carry no quoted
block). Record erratum: the landing record's "token-creation analysis the
walker omitted" is the WINNER (already admitted); the newly reachable loser is
`PredicateAdjunctPredicate` (elliptical fused determinative + duration
adjunct); the `//` card named is the Pest Problem face.

Gate scope: emit/ change → `cargo test --workspace`. Coverage must not move
(zero corpus exposure); a drop is a STOP. Standard constraints apply.

## Landing record

Measured on change `ouvrxqxtoztnnwplqqvrvpsqryutptqo` with 16,771 covered
lock identities. The measured lock has 49,421 lines and SHA-256
`2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`.

| gate | refreshed parent | measured tree |
| --- | ---: | ---: |
| selected and covered units | 16,771 | 16,771 |
| ordinary parse failures | 15,870 | 15,870 |
| unique selections | 11,515 | 11,515 |
| specificity-resolved selections | 5,256 | 5,256 |
| unresolved ties | 0 | 0 |
| construction declarations | 397 | 397 |
| coverage-lock identities | 16,771 | 16,771 |

- Coverage and lock state: a direct foreground probe of the refreshed parent
  and the post-refresh measured tree produced identical coverage summaries.
  No lock row, production construction, or source fingerprint changed, and no
  retirement manifest was created or used. Selected-uncovered units, internal
  failures, exception resolutions, exception uses, round-trip mismatches,
  ownership failures, gaps, overlaps, synthetic claims, and provenance-plan
  mismatches are all zero. Literal / lexicon collisions remain 59.
- Selection census and prior-record erratum: the prior landing's admitted
  token-creation analysis remains the winner. The now-rejected competing
  analysis is `PredicateAdjunctPredicate`, formed from an elliptical fused
  determinative plus duration adjunct; the card printed with `//` in that
  record is the Pest Problem face of Ratcatcher Trainee // Pest Problem. The
  corpus census does not move because the malformed periodless witness itself
  has zero corpus exposure.
- Positive gate artifacts after the required changed refresh: `cargo fmt
  --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`; `cargo xtask english_v2 ambiguity
  --require-resolved`; `cargo xtask english_v2 coverage --check`; and `cargo
  xtask cite check` all exited zero. The focused emitter fixture and English
  quote-boundary regression also passed. The coverage gate reported the
  complete zero-failure ownership and round-trip summary above.
- Performance advisory: on the final measured tree, ambiguity took 46.480 s
  at 166,323 accepted thread-CPU nanoseconds per byte (load 23.31 / 15.53 /
  18.88) and coverage took 30.505 s at 112,970 nanoseconds per byte (load
  23.91 / 16.54 / 19.09), each with 24 workers. The direct refreshed-parent
  probes took 22.044 s at 112,859 nanoseconds per byte for ambiguity (load
  13.57 / 15.83 / 21.00) and 19.004 s at 102,978 nanoseconds per byte for
  coverage (load 13.21 / 15.54 / 20.76). All four exceeded the 16.26 s
  quiet-host ceiling while host load was elevated; the advisory fired and the
  gates remained green.
- Assurance census: restored 0; re-spelled 0; ignored with blockers 0; added 1
  compiler test function and 1 English negative assertion; removed 0. The
  compiler fixture gives a category-valued role its own trailing literal and
  authenticates the emitted `false`; the English witness first reproduced the
  live false selection and now rejects it.
- Deviations and additions: none beyond the ticket. One fixture-only
  `LiteralEnded` construction was added to express the requested separate
  trailing literal; no production construction was added or removed. Clippy's
  identical-arm finding was resolved by grouping literal and already-opaque
  terminal atom patterns into the same `false` arm.
- STOPs: none. Six required refreshes changed the feature base without a file
  conflict; the sixth added only an unrelated ticket erratum, and all gates
  were rerun after it. Coverage did not drop.

### Erratum (landing review, 2026-09-03)

- Review ACCEPT (0 HIGH, 0 MEDIUM). Coverage summary JSON byte-identical
  across the fix; per-byte CPU within 0.8% of the parent (103,813 vs 103,002
  ns/B coverage; 110,155 vs 111,096 ns/B ambiguity) — no parse-time cost.
- `SentenceInitialLiteral` is load-bearing here, not decorative: `dash_head =
  clause sentence_initial(" —")` is a sentence-initial literal in final
  position and its predicate flips to `false`.
- The `Bound`/`Circumfix` transparency in `emit/final_constituent.rs` is now
  load-bearing and undocumented: the primitive is "rightmost non-delimiter
  constituent", not "rightmost surface leaf". A short comment naming that and
  the two `checked by` call sites it protects is a deliverable of
  `english-v2-visitor-leaf-traversal-property` (same directory).
