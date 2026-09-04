---
needs: [english-v2-plan-09-effect-and-predicate-grammar]
---
Gate AST visitor traversal corpus-wide, then retire the per-fixture preorder
vectors it subsumes.

Traversal is the one thing the seven corpus gates do not touch: `coverage`,
`roundtrip` and `ambiguity` never drive the generated `Visitor`. The only
assurance the visitor has today is ~27 integration tests carrying a literal
`&[&str]` preorder vector per fixture sentence (about 3,000 lines, the largest
single test in the tree among them). Those are change-detectors — any grammar
edit invalidates dozens of vectors mechanically — and they cover a hand-picked
50 sentences rather than the 6,558 selected units.

The property to gate is: every node reachable from a selected parse is visited
exactly once, in surface order.

**Why this is not a one-line addition.** Two things the shape needs do not
exist yet:

- The generated `Visitor` trait has no universal node hook. Observing a
  traversal today means hand-implementing one callback per node type, which is
  how the existing tests do it for their small node sets. A corpus-wide
  property needs the compiler to emit a generic enter/leave hook (or an
  equivalent walk that reports node identity), which is a
  `deckmaste_construction_core` change to `emit/visit.rs`.
- "In surface order" has no oracle from the AST alone. Byte spans live in the
  ownership claims, not in AST nodes — deliberately. Either the traversal hook
  carries the node's claim span, or the property weakens to "exactly once" plus
  agreement with the ownership claim order the `coverage` gate already computes.

Decide that shape first; it is a compiler-surface decision, not a gate tweak.
Nothing here may change the grammar, the coverage numbers, or the add-only
ratchet.

Acceptance: a corpus-wide traversal property runs inside the existing coverage
gate over every selected unit with zero failures and the selected/covered
counts unchanged; the per-fixture preorder-vector tests are deleted in the same
change; the gate reports its own traversal counter alongside the existing ones.

Note on the render-from-AST direction, which is sometimes proposed as a sibling
property: it does not belong in a corpus gate. Re-parsing a unit's rendered
bytes is vacuous while `roundtrip` already proves the rendered bytes equal the
input. The direction that needs proving — build an AST by hand, render it,
parse it back — has no corpus source, and `tests/vertical_slice.rs` already
holds it.

## Landing record (2026-09-03)

Measured on change `npypvsqz` with 16,702 covered lock identities.

- Compiler shape: the generated `Visitor` now has a default no-op
  `enter_construction` hook. Every concrete construction walker calls it once,
  before visiting children, with that construction's rule identity. The
  coverage gate records those identities and requires exact equality with the
  selected candidate's ordered construction path; the existing ownership gate
  independently continues to prove surface-byte order and coverage.
- Coverage before/after: 16,702 -> 16,702 selected and covered units; 15,939
  parse failures; 0 selected-uncovered, unresolved-tie, internal-failure,
  exception-resolution, exception-use, round-trip, ownership, gap, overlap,
  synthetic-claim, or provenance-plan-mismatch cases. The new traversal census
  is 826,642 expected nonterminal nodes -> 826,642 visited constructions with 0
  traversal failures.
- Construction and selection censuses are unchanged: 398 -> 398 construction
  declarations; 11,132 -> 11,132 unique and 5,570 -> 5,570
  specificity-resolved selections.
- Coverage lock: schema 4 and byte-unchanged at 49,352 lines, with 16,702
  covered identities, source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`,
  normalization digest
  `f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`,
  and SHA-256
  `837688617caf0b0ff0c3e6551fcf2d58772e463f71040c1c42e6d51f2cb26446`.
- Assurance: restored 0; re-spelled 0; ignored with blockers 0; added 0
  standalone tests; removed 2 vector-only standalone tests. Existing emitter
  inventory/structure tests and coverage report/runner tests were extended.
  The nominal suite retains its AST-shape, selection-path, specificity,
  rendering, ownership, semantic-count, and rejection assertions while
  deleting 3,235 lines of superseded literal preorder infrastructure and
  vectors.
- Positive artifacts: `cargo test --workspace`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `cargo xtask
  english_v2 coverage --check`, and `cargo xtask english_v2 ambiguity
  --require-resolved` are green. Both corpus commands emitted only their
  existing load-sensitive common-path wall-clock warning.
- Deviations and additions: chose ordered rule identities rather than adding
  source spans to AST nodes; this preserves the deliberate ownership/AST seam
  while proving the requested exactly-once preorder against an independent
  parser artifact. The coverage report schema advances 5 -> 6 for
  `visited_constructions` and traversal-failure evidence. No grammar,
  construction, coverage-lock, or add-only-ratchet change. No scratch or probe
  tree was created.
- STOPs: none.

### Erratum (landing review, 2026-09-03)

- The pinned property is construction-nodes-only: the corpus gate compares the
  ordered construction identities the visitor enters against the selected
  derivation's construction path. It does not observe lexeme leaf visits — a
  visitor that skips every leaf stays green at the corpus gate (leaf visiting
  is pinned only by unit tests in `ability_logic.rs`). Follow-up:
  `english-v2-visitor-leaf-traversal-property`.
- Performance advisory (measured by the reviewer, absent from the record):
  parse cost 103–144 µs/B on the landed tree vs 148–149 µs/B on the parent at
  comparable host load (17–68, other executors running) — no measurable
  parse-time cost; the 16.26s ceiling warning fired on both trees from load.
- The two removed tests were `visitor_callbacks_have_literal_full_preorders`
  and `visitor_callbacks_have_literal_typed_preorders` (literal preorder
  vectors superseded by the gate); two further tests lost only their
  `visitor_events` element, all other assertions byte-identical.
