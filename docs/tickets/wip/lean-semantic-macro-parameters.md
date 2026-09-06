---
needs: []
---
# Lean semantic macro declarations and typed parameters

Replace hand-written macro operand frames and numeric references with one
declaration mechanism for semantic macros. A macro author writes named, typed
parameters and one expansion body; the declaration generates the binding and
authoring machinery. Apply the mechanism across semantic result types, including
nouns, predicates, amounts, costs, instructions, and static specs.

Design agreed with the user on 2026-09-06 after the constructor folds. This ticket
records the implementation work; its creation does not start that work.

## Agreed interface

There is exactly one authored definition per macro. Supporting definitions may
be generated. When the expected semantic type is known, a call such as
`fight it targetCreature` expands automatically. Explicit `expand` is available
for inspection or type inference; callers need not routinely write `.body`.

Definitions resemble ordinary Lean definitions with parameter-mode annotations.
This is schematic syntax, not a claim that these annotations already compile:

```lean
semantic_macro example
    (subject : capture NounPhrase)
    (amount : capture Amount)
    (body : splice Instruction)
    (label : String) : Instruction :=
  ...
```

The body uses parameter names directly. Generate binding references, expansion
support, and authoring registration from that declaration. Exact annotation
spellings and the choice of coercion or elaboration machinery remain to validate.

## Parameter semantics

- **Captured subject:** select or resolve the subject once; every use of that
  parameter denotes the same subject. Capturing identity does not snapshot its
  subsequently read properties. Two distinct arguments containing identical
  selection syntax remain distinct selections; do not merge them by syntax or
  payload equality.
- **Captured value:** read a value once and share that value across parameter
  uses. This differs from inserting an Amount expression that is read again at
  each use.
- **Expression or body:** substitute semantic syntax without implied capture.
  Reusing an instruction body permits repeated execution; a predicate remains
  applicable to its candidates. Supplied syntax remains subject to authoring
  inspection.
- **Ordinary configuration:** labels, enums, lists, and other inputs that build
  the expansion remain ordinary typed Lean arguments. A container of semantic
  syntax still requires recursive authoring inspection.
- **Parameterized body:** a supplied body can explicitly accept a subject
  introduced by the receiving macro. This is the sanctioned connection between
  the body's caller scope and the macro's local subjects.

Infer parameter modes from types where unambiguous; declarations can state them
explicitly where a type admits multiple meanings. Add further modes only for a
concrete macro need.

Captures occur left to right in declaration order at the expansion's location.
A later argument can refer to something introduced by an earlier one. Do not
derive capture order from first use in the body. Configuration arguments and
spliced bodies do not themselves introduce captures.

Preserve surrounding scope: a macro inside a condition or repeated body captures
there, without hoisting across that boundary. Sharing a selection across
repetitions requires an explicit outer binding. Supplied bodies preserve caller
references; mentions introduced by the receiving macro cannot accidentally
retarget their pronouns. Access to macro-local subjects uses explicit body
parameters.

Nested macros forward existing captures without selecting or reading again.
A fresh expression passed to a capturing parameter gets a fresh capture at the
inner expansion's location. Generated references must remain hygienic under
nesting; authors never choose operand indices.

Capture alone does not impose an all-subjects-must-resolve precondition on the
entire body. Such requirements belong explicitly in the macro body using
ordinary semantic conditions. This supersedes the failure policy bundled into
the current `withOperands` contract; retain rules-correct macro preconditions
explicitly when migrating it.

## Expanded form and authoring boundary

The semantic result is the expanded body plus the necessary typed bindings,
with a structure suitable for the expected Rust representation. Execution needs
no macro-definition lookup. Keyword labels remain observable through `enact`;
the expanded body supplies their meaning.

Retain the separate authoring record of the named call and its arguments through
automatic expansion, explicit expansion, and nested calls. Neither a coercion
nor a generic expansion helper may become an escape hatch for raw semantic
syntax. Check supplied semantic arguments, including bodies and nested data,
while the registered macro definition remains the trusted expansion boundary.

Start from [the authoring and operand contracts](../../../lean/CONTRACTS.md),
[the declaration and inspection machinery](../../../lean/Semantics/Authoring.lean),
and [the current macro definitions](../../../lean/Semantics/Macros.lean).
The current authoring traversal inspects every argument to a registered macro
and rejects semantic free variables; explicit body parameters need deliberate
handling. Automatic coercion and typed macro applications are not implemented
by the existing registration attributes alone.

## Completion

Implement the shared declaration mechanism and migrate the existing manual
capture macros to named parameters. Replace or subsume their ad hoc operand
machinery; keep internal binding carriers out of the generated public primitive
interface. Ordinary semantic macro definitions must use the same declaration
path without duplicate authored definitions or unnecessary capture frames.

Demonstrate direct and explicit expansion, typed modes across semantic result
types, ordered dependent captures, independent identical arguments, nested
forwarding, caller-scope preservation, explicit body parameters, and capture
placement under conditions and repetition. Include meaningful checks that
distinguish a captured value from a repeatedly read expression and establish
that capture itself adds no whole-body success requirement.

Retain negative authoring coverage for raw nested arguments, aliases, opaque
content, semantic parameters without evidence, discarded values, and projections;
exercise the new expansion paths against the same boundary. Preserve existing
card coverage and re-express affected proofs against the new shape. Update the
contracts and any necessary glossary entries to describe the landed model.
Standard constraints apply.

The deferred collection-member binder, aggregation and numeric-anaphora redesign,
result collections, cost-symbol expansion, and Rust implementation remain out of
scope. Capturing an Amount parameter does not pull those designs into this ticket.
