---
needs: []
---
# Fold definitions, continuations, repetition, and coordination

Implement the concrete binding/composition folds below. The user narrowed the
constructor campaign on 2026-09-06: larger mechanisms are design work in
`maybe`, not prerequisites for these folds.

## Campaign contract

The five planned tickets target Lean syntax, computed attributes, macros, and
witnesses. The authority remains [the workbench boundary](../../../lean/README.md),
[trusted macro expansion](../../../lean/CONTRACTS.md), and
[the constructor/macro decision](../../decisions/semantics-v2.md#6-the-constructormacro-boundary).
Rust execution and frozen Idris are outside these implementation tickets.

A family must share semantic operations or laws; moving cases to a helper
inductive or renaming a primitive is not by itself a reduction. Record outer
constructor counts and helper vocabulary separately, without a target count.
Preserve rules distinctions and reference scopes; wording belongs in macros
and labels provide observability rather than missing meaning. The planned
folds may introduce the small typed payloads they specify, but do not authorize
the parked binding, query, result, or event mechanisms implicitly.

The other independently claimable tickets are:

- [Attachment and face predicates](lean-core-attachment-and-face-predicates.md).
- [Characteristic edits and keyword arguments](lean-core-characteristics-and-keyword-arguments.md).
- [Events and static specs](lean-core-events-and-static-specs.md).
- [Action expansions](lean-core-actions.md).

All five touch shared inductives or checker traversals. Coordinate overlapping
implementation work; absence of dependency edges is not a promise of disjoint files.

## Change

- Unify `Instruction.define` and `StaticSpec.letterDefinition` as one numeric
  definition form, preserving each enclosing context's scope and cost
  admissibility. Earlier uses of X remain licensed, and a trailing definition
  can read an earlier target or objects affected this way. Preserve number
  regimes and duplicate-definition refusals. This is not permission to hoist
  a definition, introduce a general binder framework, or blindly alias the
  instruction to `establish` without handling its cost-admissibility difference.
- Fold `offer` and `doIfDone` into one action with continuations and an
  optional/required policy, retaining a deciding player where applicable.
  The positive branch tests the decision or start of payment, not whether
  resulting events happened [CR#118.12]. It receives the body's introductions;
  the negative branch retains the incoming scope. Preserve those distinctions
  even though the existing forms have matching fields. The full `Compulsion`
  enum, including prohibition and toll, is not this policy.
- Fold `Instruction.repeatTimes` into the repetition family with a fixed-count
  alternative carrying the amount and body. Preserve the other repetition
  modes and their existing scopes, outer bindings, result publication, and
  cost checks. Do not replace `againExcludingChosen` or redesign carried
  selections in this pass.
- Use lists for semantic conjunction and alternatives in predicates,
  conditions, noun phrases, and costs. Preserve cardinality obligations and
  the distinction between conjunction, alternatives, sequential composition,
  and simultaneous composition. Fixed-arity wording is macro syntax. Use
  `sequentially`/`simultaneously` for the instruction pair; retain `establish`.

## Deferred and dropped work

[Member binding, aggregation, and numeric reads](../maybe/lean-core-numeric-bindings-and-aggregation.md)
are separately assessable design questions. [Carried repetition selections](../maybe/lean-core-repetition-selections.md)
are also parked. Neither is needed to complete the four folds above.

Drop the proposed sharing of `other`, `notChosen`, and `otherThan` from this
campaign for insufficient demonstrated benefit. Keep their existing sources,
scopes, and publication behavior. Distinct sources do not prove that no shared
operation is possible; no such redesign is commissioned here.

## Blast radius

The coordination change touches every `both`/`eitherOf`/`either`/`compound`
site in the bench and pin suites, the widest re-spell in this campaign.
Iterate on a card subset and run the full build once at the end. The landing
record's restored/re-spelled/removed counts are where a silently weakened
pin would show; a negative twin that loses its exact refusal list is a
regression, not a re-spelling.

## Completion evidence

Re-spell the affected cards and named pins. Soul's Might and Phyrexian Rebirth
exercise definitions using earlier introductions; check both continuation
scopes, fixed-count repetition, retained repeat policies, and coordination
cardinality. Record each retired constructor's replacement and any deliberate
change to an old refusal or published binding. Run `lean/scripts/build`.
Standard constraints apply. The parked Idris
[segment-indexed telescope](../maybe/workbench-segment-indexed-telescope.md)
is not reopened.
