# English grammar design in Lean

Accepted 2026-09-05. Design the intended Oracle English grammar top-down in a
Lean `English` module beside `Semantics`, then migrate `english_v2`'s
`constructions!` grammar to that design. Consolidation and missing general
grammar are both in scope. The failure frontier resumes driving work only for
the subsequent long tail. The [wayfinder](../english-grammar-wayfinder.md)
records the work sequence and disposition of earlier tickets.

## The workbench's contract

`English` is a formal model of the proposed grammatical structure. It belongs
to the Oracle English bounded context in [CONTEXT-MAP.md](../../CONTEXT-MAP.md);
`Semantics` belongs to Game Model. Grammatical acceptance does not require
semantic validity. The [rewrite's source hierarchy](english-v2-rewrite.md#source-hierarchy-and-support-corpus)
still applies: the style guide supplies structural direction, the supported
Oracle corpus supplies conformance evidence, and rules meaning is consulted
where needed. Existing grammar shapes and acceptance are evidence to examine,
not the specification of the new model.

Model categories, features, lexical frames, composition, and realization as
relations. An executable parser, renderer, card validator, code generator, or
production consumer of Lean is not a prerequisite or deliverable. Introduce
executable helpers only to answer a named design question. The choice of
indices versus ordinary syntax plus judgments is a design question for the
model, not inherited from the old Idris representation.

Realization must connect structures to surfaces before claiming anything about
textual ambiguity. Distinguish raw grammatical ambiguity, selection ties, and
ambiguity deliberately retained in a canonical representation. The existing
[scope ruling](english-v2-rewrite.md#amendment-one-scope-device-principles-before-packing-2026-09-04)
preserves alternatives for Semantics; uniqueness of a representative does not
assert uniqueness of interpretation. Define admissible derivations separately
from preference and packing. Acyclic preference alone does not rule out ties,
and an arbitrary deterministic choice does not justify a reading.

Use concrete witnesses and bounded, explicitly stated theorem obligations to
refine the design. A model-wide absence of unintended ambiguity is not a
promised result or a completion gate. State lexical, feature, and fragment
assumptions; show nontrivial inhabitants alongside exclusion claims. Lean
proofs establish properties of this model, not correspondence to Oracle
English or correctness of the Rust implementation.

## Production migration

Retain `english_v2` and its declaration-owned generated types, parsing,
rendering, and traversal. The generic chart engine and declaration compiler
are reuse candidates. Scanning, materialization, environment handling,
selection, and attachment may need adaptation; retaining their present
details is not a constraint on the grammatical design. Compiler changes are
in scope when demonstrated by a grammatical requirement. A separate
`english_v3`, extraction of a universal runtime, and automatic Lean-to-Rust
generation are not scheduled by this decision.

Design the whole intended grammar before fixing the production replacement
boundaries. Implement coherent portions incrementally, replacing superseded
constructions together with their consumers and tests. The migration design
must distinguish inherited linguistic decisions from implementation hypotheses
and resolve conflicts explicitly. The earlier Group R order and the
largest-first tail loop no longer schedule this phase.

Production migrations retain the rewrite's structural laws and the
PROVE/DISCLOSE/REPORT landing contract. Workbench tickets instead validate their
Lean definitions, witnesses, and named proofs; they do not run production
coverage ceremonies or claim coverage gains. No proof transfers to Rust merely
because its types resemble the Lean model. Review the correspondence and test
the migrated structure through the production implementation.

## Returning to the long tail

The model accounts for the intended scope, with omissions and open decisions
explicit. Completion means implementing and validating the agreed design,
then making a revisable judgment from representative remaining failures that
the ordinary long-tail process is useful again. Neither a coverage percentage
nor an exhaustive classification proving that no foundational gap remains is
required for that judgment. Newly discovered general omissions reopen design.

Unclaimed grammar tickets wait for the migration design, which preserves their
regression obligations while regrouping their implementation work. WIP tickets
and claims are untouched; their integrated results are inputs to the migration.

## Uncertainty

The expected benefit is fewer separately authored combinations and less
temporary compatibility work. Faster corpus completion is a hypothesis, not a
promise. Global uniqueness proofs, the amount of counterexample automation,
and which existing selection mechanisms survive remain open. The formal model
is useful even if production never consumes it and no executable parser is built.
