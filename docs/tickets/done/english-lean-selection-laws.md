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


## Landing record

**PROVE.** Change `oxmxsopv` adds the finite selection/packing inventory in
`docs/english-grammar-design.md`: selection soundness and enumeration
independence, consistency and finite survivor existence under ordered declared
principles, exact nonempty scope packages, and uniqueness for the stated
modifier-scope class. `English.SelectionWitnesses` connects the generic laws
to independently admitted shared `Syntax` trees and equal surfaces.
`English.Scope` checks the explicitly abstracted boundary/anchor obligations.
`cd lean && ./scripts/build` passes all 67 jobs with warnings as errors.
Lean LSP MCP diagnostics are clean on all three new modules; all 39 public
claims were checked with LSP `lean_verify`, using only subsets of the standard
three axioms. Four private proof helpers are audited through their public
consumers. No proof placeholders or custom axioms were added. Scanner matches
for `opaque` are the two comments describing role-edge boundaries.

Assertion counts: 43 added (39 public, four private), 0 restored, 0 re-spelled,
0 ignored, 0 removed. All prior 74 assertions remain unchanged. No production
parser, corpus identity, coverage gate, or construction declaration changed;
no loss or gain is claimed. Production structural-law and coverage ceremonies
are outside this workbench ticket under the accepted Lean design decision.

**DISCLOSE.** The acyclic-but-tied and independent-recombination counterexamples
are certified and retained. Their replacement claims are finite survivor
existence and exact preservation of the joint alternative relation. The
cross-host example is a synthetic grammatical witness with one overt
auxiliary and three admitted attachment readings. Same-text homographs remain
separate when keys preserve lexical identity. All examples are synthetic;
there are no new card-text or Comprehensive Rules claims.

Deviations and additions: added general maximum/existence and unpacking laws,
a four-feature precedence witness, an inadmissible-competitor witness, and
category-site exclusions to make the required claims inhabited and restrictive.
Added `Scope.lean` separately because its boundary and anchor abstractions are
not a second grammatical derivation. The inherited frame-pair and Class D
challenges are represented by explicit abstractions, not claimed as completed
production or whole-card proofs. Their grammatical correspondence, feature
assignment, document interaction, and nested-mobile witness are expressly
routed to the unclaimed `english-lean-grammar-design-review` ticket. No other
WIP ticket was touched. No STOP, regression, or ruling contradiction occurred;
routine elaboration errors in new drafts were corrected. No glossary gap was
needed; the added names denote formal relations over the existing vocabulary.

**REPORT.** Proof scope is the modeled fragments and stated lexical/feature
assumptions; this is not a global unambiguity theorem or Rust correspondence
proof. Corpus counts, construction counts, selection census, lexical-overlap
inventories and production performance telemetry were not measured because
production was unchanged. Model evidence belongs to `oxmxsopv`; see the design
document's named inventory and limits for review.
