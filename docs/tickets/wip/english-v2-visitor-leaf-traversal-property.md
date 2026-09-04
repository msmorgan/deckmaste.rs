---
needs: [english-v2-corpus-wide-visitor-traversal-property]
---
Extend the corpus-wide traversal property to lexeme leaves (visitor-traversal
landing review MEDIUM-1). Today the coverage gate proves every selected unit's
visitor enters exactly its ordered construction path, but leaf visits are
unobserved: a generated visitor that drops every lexeme/terminal leaf visit
(`emit/visit.rs` leaf arm returning `None`) leaves `coverage --check` green.
The materialized derivation already knows its leaves in surface order; record
the visitor's leaf visits alongside `enter_construction` and require ordered
equality with the derivation's leaves (or the surface token sequence) per
selected unit, reported in the coverage report as visited/expected leaf counts
with per-unit failure evidence. Mutation-verify: the leaf-drop mutation must
fail the gate. Zero grammar change; lock byte-unchanged. Gate scope: emit/
change → `cargo test --workspace`. Standard constraints apply.

Also (literal-opacity landing review LOW-1, same `emit/` directory): add a
short comment at the `Bound`/`Circumfix` arms of `emit/final_constituent.rs`
stating that the fold deliberately looks through delimiters — the predicate is
"rightmost non-delimiter constituent" — and that `quoted_ability` and the two
quote-terminator `checked by` sites depend on it. Facts only; no essay.

## Landing record (2026-09-04)

Measured on change `tlpzkynnuqmzkqvtrolkswylkmvqluus` with 16,771 covered
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
| expected / visited construction entries | 731,469 / 731,469 | 731,469 / 731,469 |
| expected / visited lexeme leaves | not reported | 233,679 / 233,679 |

- Property implementation: each materialized derivation now retains its
  ordered, non-literal terminal Lexeme classes in surface order. The generated
  visitor reports each such leaf through the default no-op `enter_leaf` hook
  immediately before its typed callback, and the coverage gate requires the
  selected derivation's exact sequence to equal the visitor's sequence for
  every selected unit. Coverage report schema 7 carries per-unit expected and
  visited sequences, aggregate counts, and explicit mismatch evidence.
- Coverage and selection census: selected-uncovered units, leaf traversal
  failures, construction traversal failures, internal failures, exception
  resolutions and uses, round-trip mismatches, ownership failures, gaps,
  overlaps, synthetic claims, and provenance-plan mismatches are all zero.
  Literal / lexicon collisions remain 59. There are no newly covered
  identities and no changed selected analyses; unique and
  specificity-resolved selections are byte-for-byte census-equivalent to the
  refreshed parent.
- Coverage lock: byte-unchanged. Its source fingerprint is
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and its normalization digest is
  `f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`.
  No production Construction was added or removed, and no retirement manifest
  was created or used.
- Mutation verification: replacing the generated leaf hook emission with an
  empty token stream made `cargo xtask english_v2 coverage --check` exit 1:
  16,388 selected units became uncovered, expected leaves remained 233,679,
  visited leaves fell to 0, and the report showed the expected arrays against
  empty actual arrays. The mutation was immediately restored before the final
  gates.
- Positive gate artifacts after the required refresh: `cargo fmt --all --
  --check`; the exact generated-helper-cycle regression; `cargo test
  --workspace`; `cargo xtask english_v2 ambiguity --require-resolved`; and
  `cargo xtask english_v2 coverage --check` all exited zero. The workspace
  suite included 396 construction-core tests, 143 English-v2 library tests,
  and 422 xtask tests (1 ignored), plus integration and doc tests. A nonbinding
  clippy probe reached ten pre-existing `unneeded_wildcard_pattern` findings in
  refreshed-parent `deckmaste_lowering/src/card.rs`; this feature does not
  touch that file.
- Performance advisory: after the post-commit changed refresh,
  measured-tree coverage took 25.704 s at 111,764 accepted thread-CPU
  nanoseconds per byte (load 22.99 / 22.01 / 23.17), and ambiguity took 24.134
  s at 112,330 ns/B (load 21.26 / 21.85 / 23.18), with `pgrep -c -x codex`
  equal to 1 before both. Direct refreshed-parent coverage took 23.275 s at
  106,088 ns/B (load 12.75 / 19.54 / 22.69) with 4 Codex processes, and
  ambiguity took 27.644 s at 125,567 ns/B (load 21.63 / 20.95 / 23.03) with 3.
  All four used 24 workers and exceeded the 16.26 s quiet-host ceiling under
  elevated load or concurrent Codex activity; the advisory fired and both
  gates remained green.
- Assurance census: restored 0; re-spelled 0; ignored with blockers 0; added 0
  standalone test functions; removed 0. Six existing test functions were
  extended: three emitter fixtures and three coverage/report fixtures. An
  existing exact cycle regression caught a transient RHS-indexing error during
  implementation; indexed lookup now preserves malformed-family rejection.
- Deviations and additions: no grammar, selection policy, Construction, or
  lock change. Additions are the requested traversal evidence, report schema
  7, and the requested short delimiter-transparency comment. `open
  declaration` uses its declared semantic terminal label; no identity-specific
  guard was added.
- Glossary gap: **terminal leaf traversal** — the ordered visits to a
  derivation's non-literal terminal Lexemes in surface order. Existing Oracle
  English glossary entries define Construction, Constituent, and Lexeme, but
  do not name this evidence stream.
- STOPs: none. One refresh was delayed by a sibling workspace's divergent
  change; it was left untouched and the later refresh succeeded. The required
  post-commit refresh rewrote the stack, so the parent and measured tree were
  both remeasured and this record was updated. Coverage did not drop, no tie
  appeared, and no newly covered negative or wrong parse was admitted.
