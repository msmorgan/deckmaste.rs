---
needs: []
---
**Two crate gaps in `deckmaste_semantics_v2` found by
`semantics-v2-macro-bodies-keyword-actions`.** Terra tier; standard
constraints apply.

- `ron::param_types()` is a closed list of eight; a body needing a
  `Subtype`, `ZoneExpr`, `Quantity`, `TokenSpec`, or `Instruction` argument
  cannot declare it (Amass, Create, Meld, Vote, Search, Face a Villainous
  Choice). Per `semantics-v2.md` §12 every `SupportsMacros` kind is a
  parameter type; make the list total the way `every_supports_macros_type_is_a_kind`
  keeps the kind set total.
- A declaration named like a native constructor at its own position is
  silently shadowed (`Shuffle` registers but can never be invoked;
  `semantics-spelling-lowering.md` §6's collision diagnostic is the v1
  precedent). The reader refuses the collision at load with both names.

Then give Amass, Create, Meld, Vote, Search, and Face a Villainous Choice
their bodies from the Lean macros (their STOP text names each shape), with a
canon card each where the corpus has one, proving through `lean-check`.
