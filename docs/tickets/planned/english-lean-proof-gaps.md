---
needs: [english-lean-lexical-ambiguity]
---
# Close the vacuous and unanchored obligations in the English grammar model

A whole-project review of `english/` (2026-09-05, after the five
`english-lean-*` tickets landed) found the build, axiom audit and module
reachability clean, but six places where a named design obligation is
discharged by a theorem that cannot fail, never touches `Syntax`, or is
proved over a tree the grammar cannot derive. Each is a representation gap
to close with real model content, not a statement to narrow. The
[design](../../english-grammar-design.md) and
[decision](../../decisions/english-lean-design-workbench.md) still govern;
Lean proofs establish properties of this model only.

The [lexical/ambiguity ticket](english-lean-lexical-ambiguity.md) owns the new
admission interface, all-readings contract and non-Cartesian grammatical
witness. Record the exact obligations it actually discharges here; do not
repeat that implementation or assume every old gap disappeared. This residual
work is required for honest model claims, but is not a blanket prerequisite
to production breadth or its long-tail handoff.

Required:

1. **Tense.** The design map's row 1 lists clause tense and the glossary
   makes Tense distinct from Finiteness and Inflectional Form, but the model
   has no `Tense` and no limitation names its absence. Add or connect the
   grammatical judgment with an inhabitant and exclusion, preserving distinct
   Tense, Finiteness and Inflectional Form. Any narrower model scope must be
   stated honestly; declaring Oracle clauses tense-neutral to avoid the work
   is not an accepted decision.
2. **Flat coordination is missing, not optional.** Every coordinate
   production is binary, so an Oxford-comma list ("A, B, and C") has no
   derivation at all: no witness realizes a comma list, and the only
   three-way surface in the project is the repeated-coordinator form. The
   serial comma marks one flat n-ary Coordination on the surface, which is
   also how the Oracle English glossary already defines Coordination ("two
   or more coordinate units"); a nested-binary bracketing of such a list is a
   wrong analysis, and the repeated-coordinator form is the only place
   nesting is a real ambiguity. Add the n-ary production with its comma and
   coordinator realization, witness a three-item list with an exclusion of
   the nested bracketing for that surface, and re-state
   `FrameScope.flat_nested_differ` over derivable trees; today it compares a
   three-child node no production can build.
3. **Genitive countability.** `DeterminerUse.genitive` leaves `use`
   unconstrained, so `genitive_preserves_countability` and its negation
   share a proof. Investigate and express the head/Determiner
   countability constraint, rename the theorem to what it proves, and add a
   genitive `Derives`/`Realizes` witness. The earlier prescribed premise that
   a possessor determines the possessed head's countability is withdrawn;
   preserve the head's declared uses with actual grammatical evidence.
4. **Joint-alternative law.** `Scope.joint` is inhabited only over
   `Bool × Bool`; the nested-mobile witness that landed for it packs the full
   four-reading Cartesian product, the case where per-mobile projection is
   already sound. Supply a grammatical nested-mobile configuration whose
   mobiles are individually admissible but jointly not, and prove its
   package retains exactly the correlated rows.
5. **Anchor projection.** `Scope.Anchors`, `flat`, `nested`, `Scope.Reading`
   and `anchor_shape_separate` are referenced by nothing; the obligation to
   project frame-pair syntax onto the anchor abstraction is met only by
   `FrameScope.anchorCount : Syntax → Nat`. Either write the
   `Syntax → Anchors` projection and instantiate the anchor laws over it, or
   delete the dead abstraction and amend the design's "What the proofs
   establish" to say cardinality.
6. **Word payload.** `BoundaryInteractions.wordPayload` is consulted by no
   part of the grammar; `Atom.word` accepts any string, so the
   source-whitespace and quote-delimiter claim is unenforced. Make it a
   side condition where words enter surfaces (or on the `Lexicon`), or
   delete the three theorems and record source normalization as a
   production-only obligation.

Residual audit, accounting for repairs already landed by the lexical/ambiguity ticket: `Analysis.no_feature_bypass`
never uses its packing hypothesis; five packing witnesses use a `Unit` key so
their packing half is inert, and `anchor_shape_separate` /
`same_surface_separate` pin the key to the discriminating field; `Analysis.key`
has no proved lexeme/host/anchor invariant while the equal-text ruling is proved
only for `GrammaticalScope`; the superseded `Claims`/`Policy` layer is never
instantiated on syntax; `FrameScope` (model) imports `FrameInteractions`
(witnesses); `chapter` and `dieDashRow` are one rule twice and create duplicate
derivations, a defect distinct from valid ambiguity;
`attributive`, `bareMass`, `Placement.before` and `ScopeMove.auxiliary` have no
witness; `SharingInteractions` states no exclusion. Naming: `Complement`,
`quantity`/`quantify`, three unrelated public `Reading` types, and the
selection/packing vocabulary missing from the Oracle English glossary — amend
the glossary through the domain-modeling skill, never rename an entry to fit
an identifier.

Acceptance: every numbered item and every item in the residual audit has a
named disposition: repaired with structural evidence, superseded by a justified
current decision, or transferred to an explicit live owner with its obligation
intact. Transferred work is not reported as a repaired model claim; no theorem
removed without its replacement named; `english/scripts/build` green and the
axiom audit unchanged. Standard constraints apply.
