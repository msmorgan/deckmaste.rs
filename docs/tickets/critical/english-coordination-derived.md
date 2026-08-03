---
needs: [english-construction-compiler, english-coordination-comma-defects]
---
**Milestone one: derive the noun/nominal coordination family.**

First migrated family under `docs/decisions/english-grammar-is-derived.md`,
built on the partition and intended trees `english-coordination-comma-defects`
establishes. That ticket's defect-1 parse-selection residue, where
cost-tuning is documented ineffective, transfers here as the first proof
obligation: the wrong outer split contains a binary-comma noun coordination
that no derived construction licenses.

Coverage: binary coordination without a comma; mandatory Oxford commas for
three or more members; asyndetic comma lists; shared determiners; nested
binary coordination; repeated prepositions (`from X, from Y, and from Z`);
and the surrounding power/toughness attachment fixtures.

Execute as one vertical migration:

1. Record the post-defect recovery, round-trip, direct-AST, and `inspect`
   baseline, including every remaining wrong-outer-split and P/T-attachment
   fixture transferred from `english-coordination-comma-defects`.
2. Add the noun/nominal declarations behind the inactive generated registry
   owner. Exercise every listed surface form in both directions before
   changing production routing.
3. Re-measure punctuation by the round-`sfdiet` method while the stored fields
   still exist: render with the derived value, then run `cargo xtask english
   roundtrip --list`. Delete only the noun/nominal comma fields proved
   derivable from member count and conjunction shape. Clause coordination and
   `CoordinationJunction` retain their documented surface witnesses.
4. Flip the registry owner once for the whole family, migrate affected
   `deckmaste_frames` and serialized-AST consumers, then delete the handwritten
   parse, reduction, lowering, renderer, constructors, and duplicate
   registrations in the same change.
5. Run the structural audit and the normal, reversed, and fixed-shuffle
   registration suites after deletion so a surviving order dependency cannot
   hide behind the removed implementation.

The pilot succeeds only if adding or changing a coordination surface form
requires one declarative edit and both directions change together. A
prototype that wraps the existing paired code, needs string-shaped escape
hatches, or leaves a second semantic description in renderer arms does not
qualify. If a construction law fails: record the exact failed law or backend
limitation, remove the prototype, and re-review the decision.

Gates: `cargo xtask english roundtrip --require-clean`; recovery census
byte-identical to the post-defect baseline through this milestone; direct
AST fixtures pin the attachment and coordination invariants, with `inspect`
naming the construction and decisive constraint on ambiguous fixtures;
negative fixtures reject illegal binary Oxford commas as a
noun/nominal-family rule (binary comma stays legitimate at clause level and
in `CoordinationJunction`'s documented residue); `deckmaste_frames`
compile/unify/render gates stay green.

Completion makes `english-derived-family-inventory` ready. That ticket is the
committed continuation of the migration, not a referendum on broader adoption;
the heterogeneous and member-scoped coordination forms remain designed by
`english-coordination-structural-design` but must be assigned to a derived
family migration there.

Standard constraints apply.
