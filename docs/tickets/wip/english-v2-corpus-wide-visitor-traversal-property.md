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
