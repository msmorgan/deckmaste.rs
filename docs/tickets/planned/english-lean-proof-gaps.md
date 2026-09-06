---
needs: []
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

Required:

1. **Tense.** The design map's row 1 lists clause tense and the glossary
   makes Tense distinct from Finiteness and Inflectional Form, but the model
   has no `Tense` and no limitation names its absence. Either add the
   judgment with an inhabitant and an exclusion, or record in the design
   document the decision that Oracle English needs none and why.
2. **Flat-versus-nested fence.** `FrameScope.flat_nested_differ` compares a
   three-child `.coordinate` node, but every coordinate production is
   binary, so flat n-ary coordination is unrepresented and the design's
   "cardinality fence" claim rests on an underivable tree. Represent flat
   coordination (or record that Oracle English coordination is binary and
   retire the claim), and re-state the fence over derivable trees.
3. **Genitive countability.** `DeterminerUse.genitive` leaves `use`
   unconstrained, so `genitive_preserves_countability` and its negation
   share a proof. Give the constructor a premise (the possessor's nominal
   use determines the head's), rename the theorem to what it proves, and add
   a genitive `Derives`/`Realizes` witness; there is none today.
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

In scope with the above, no separate tickets: `Analysis.no_feature_bypass`
never uses its packing hypothesis; five packing witnesses use a `Unit` key so
their packing half is inert, and `anchor_shape_separate` /
`same_surface_separate` pin the key to the discriminating field; `Analysis.key`
has no proved lexeme/host/anchor invariant while the equal-text ruling is proved
only for `GrammaticalScope`; the superseded `Claims`/`Policy` layer is never
instantiated on syntax; `FrameScope` (model) imports `FrameInteractions`
(witnesses); `chapter` and `dieDashRow` are one rule twice and force a tie;
`attributive`, `bareMass`, `Placement.before` and `ScopeMove.auxiliary` have no
witness; `SharingInteractions` states no exclusion. Naming: `Complement`,
`quantity`/`quantify`, three unrelated public `Reading` types, and the
selection/packing vocabulary missing from the Oracle English glossary — amend
the glossary through the domain-modeling skill, never rename an entry to fit
an identifier.

Acceptance: each of the six items resolved by model change with a witness
that inspects structure, or by a recorded design decision; no theorem
removed without its replacement named; `english/scripts/build` green and the
axiom audit unchanged. Standard constraints apply.
