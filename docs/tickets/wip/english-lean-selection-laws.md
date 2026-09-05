---
needs: [english-lean-grammar-composition]
---
# Formalize ambiguity, selection, and packing laws

Extend `lean/English/` with explicit preference and ambiguity-packing relations
over the grammatical derivations and surfaces already modeled. Follow the
[Lean design decision](../../decisions/english-lean-design-workbench.md) and
the rewrite's declared-principles and scope rulings. Existing Rust algorithms
are implementations to assess, not definitions of what the laws must mean.

Start with a finite named theorem inventory and explicit assumptions. Include
selection soundness (a selected analysis is admitted), enumeration independence
when candidate collections are represented as lists, consistency of the admitted
preference rules, and exact preservation of alternatives by the modeled packing
relation. State uniqueness for a specific nontrivial fragment or ambiguity
class where justified. Distinguish a unique representative from a unique reading.

Supply inhabited examples and counterexamples at the claim boundaries: an
acyclic preference with incomparable survivors; distinct analyses sharing a
surface; alternatives that must pack; and distinct structures that must remain
separate. Use the existing cross-host and conjunct-boundary ticket witnesses
as challenges. Generalize beyond those witnesses where the model supports a
precise theorem. If a proposed law is false, retain the certified counterexample
and resolve the design before narrowing the statement; do not silently remove
valid derivations to obtain a proof.

Acceptance: the named obligations are proved without `sorry`, `admit`, or
unjustified axioms, or explicitly refuted with a resolved replacement claim;
report their assumptions and axiom audit. Define admissibility independently
of selection and packing so the claims are not implementation restatements.
No promised theorem of global grammar unambiguity, no precedence by construction
name, and no executable parser are required. Standard constraints apply.
