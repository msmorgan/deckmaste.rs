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

## Landing record

Implemented on change `ylroosyt`; the unchanged English v2 lock contains 20,254
covered identities. Standard constraints apply.

- One `semantic_macro` declaration now generates registration, lexical context,
  typed captures, and caller-scope handling. All 440 existing hand-written
  macro definitions use it. Direct calls return their semantic type; `expand`
  preserves the named authoring call without granting additional trust.
- Subject and amount captures share values by actual binding address, in
  declaration order at the expansion location. Nested captures forward aliases;
  splices preserve caller references and support explicit noun parameters,
  including callback type aliases. Internal binding carriers have no generated
  public primitive. Fight, regeneration, and player-counter removal use named
  captures; ordinary one-use counter movement needs none.
- Noun results retain their context, additions, returned address, and value.
  Scope exit and containing noun constructors preserve updates and explicit
  type/marker views. Temporary caller
  masks remain distinct from permanent forgetting. Existing classifier and
  identity checks follow scoped syntax, including enacted moves, opponent
  libraries, nested replacements, definition costs, and self-counterparts.
- Authoring inspects aliases, projections, containers, discarded values, and
  higher-order applications. Helper inspection skips only an instantiated
  definition graph free of semantic content; semantic types prevent concealed
  function parameters from bypassing evidence checks. Per-inspection caching
  avoids repeated ordinary list unfolding without changing the heartbeat limit.

### Assurance and scope

No silent loss: comparison with the pre-change sources preserves all 4,124
named declarations in the 44 original card/proof files. All 15 card source files,
including 816 `Spelled` definitions, are byte-identical. No card was removed or
newly admitted by an edited bench term.

Test accounting: restored 0; re-spelled 2 named theorems
(`conditionalForgettingKeepsTheOperandScope`, `badFightGroup`) plus the shared
`ActionFamilies.operand` helper; ignored 0; added 78 named theorems and 10
negative authoring guards; removed 0. The fight-group witness remains rejected:
its diagnostic is now singular-power/per-member damage failure, replacing
failures caused by the retired singular private operand representation.

`Proofs/MacroParameters.lean` distinguishes value capture from repeated reads,
independent identical selections from aliases, dependent declaration order from
body-use order, caller references from macro-local mentions, and explicit body
parameters from unlicensed semantic variables. It covers direct/explicit
expansion and all supported scoped result types. Existing proof modules remain
in the full gate; no `sorry`, new axioms, or native decision shortcuts were added.

Deviations and additions: Quantity, ZoneExpr, Condition, and GameEvent receive
the same scope forms as the six explicitly named result types, so semantic
expressions compose consistently. The noun-result representation and inspection
cache were required by returned capture identity and unchanged bench authoring.
The glossary's existing Semantic Macro definition now covers its parameter modes;
no new glossary concept was needed. Deferred collection/binder mechanisms,
cost-symbol expansion, and Rust implementation remain untouched.

The Rust grammar, declarations, coverage lock, and selection machinery are
unchanged. There are no changed construction analyses, licensing guards, lexical
ownership rules, homograph/form inventories, or lowering artifacts. Accordingly,
compiler roundtrip/census/CPU-per-byte measurements were not rerun for this Lean
change; `cargo xtask gate --changed` reports no affected workspace crates.

### Validation

The final `lean/scripts/build` warning-as-error gate passes all 77 jobs,
including the entire card bench and proof suites. All 78 new theorems were
inspected with `#print axioms`; their sole dependency is standard `propext`.
The final incremental build took 62.74 seconds at a starting one-minute host
load of 4.24, with 24 logical CPUs and Lake's default worker setting. This is
Lean build telemetry, not an English corpus performance measurement.

Citation checking reports 0 noncompliant strings and 0 stale citations; the
diff audit's one changed citation site was read against its rule text.
`cargo xtask gate --changed` reports no affected Rust crates, and
`kata kanban check` passes. Final refresh was a no-op. No unresolved
design/ruling contradiction remains.
