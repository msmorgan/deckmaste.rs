---
needs: []
---
# Unify Lean binding reads and instruction composition

Replace the special binding and control forms in `lean/Semantics` with shared
structures that preserve their scopes. Decisions agreed with the user on
2026-09-06, following the constructor inventory and design interview.

## Campaign contract

This and the four tickets below implement the agreed constructor reductions
in the Lean semantics workbench. The authority remains
[the workbench boundary](../../../lean/README.md),
[trusted macro expansion](../../../lean/CONTRACTS.md), and
[the constructor/macro decision](../../decisions/semantics-v2.md#6-the-constructormacro-boundary).
This is syntax, computed attributes, macros, and their witnesses; Rust execution
and the frozen Idris model are outside this campaign.

A family must share semantic operations or laws. Moving cases into a helper
inductive, or renaming a primitive, is not by itself a reduction. Missing
reusable primitives are allowed, including temporary constructor growth.
Record outer-constructor counts and total helper vocabulary separately; there
is no target count. Current checker limitations are evidence to examine, not
automatic requirements of the replacement. Preserve distinctions that the
rules and reference scopes need. Wording belongs in macros, and labels provide
observability rather than supplying a missing body.

Companion work:

- [Relation queries and result collections](lean-core-relations-and-results.md).
- [Characteristic edits and keyword arguments](lean-core-characteristics-and-keyword-arguments.md).
- [Events and static specs](lean-core-events-and-static-specs.md).
- [Action and cost expansions](lean-core-actions-and-costs.md).

## Change

- Add an explicit collection-member binder usable by numeric folds. The
  query ticket uses it for counting and projected aggregates; authoring
  macros hide the binder without conflating selection and measurement.
- Fold `Amount.thatMuch`, `theOutcome`, `groupSize`, and `theDifference`
  through typed binding reads and projections. They are not all outcomes:
  a group can belong to a proposed event, and a difference comes from a
  comparison. Preserve each read's category, cardinality, scope, and ambiguity
  rules. Two eligible numeric outcomes must still make bare "that much"
  ambiguous; an explicitly selected outcome category can disambiguate it.
  Audit which numeric reads are admitted on comparison sides rather than
  copying constructor-specific exclusions blindly.
- Share set exclusion between `Predicate.other`, `notChosen`, and
  `otherThan`. Their sources remain distinct: applicable targets, all eligible
  standing choices, and explicit anchors. Standing choices can include
  several players and are not a unique-antecedent read. Preserve nested
  explicit mentions and closure of standing-choice scopes; move restrictions
  justified solely by wording to macros.
- Unify `Instruction.define` and `StaticSpec.letterDefinition` as a scoped
  numeric definition. Preserve trailing "where X is ...": earlier uses of X
  are licensed, and the right-hand side can read an earlier target or objects
  affected this way. This is neither a forward-only `let` nor a blind rewrite
  to `establish`. Preserve number regimes, cost admissibility, and rejection
  of duplicate definitions across the new enclosing scope.
- Fold `offer` and `doIfDone` into one action with continuations and an
  optional/required policy, retaining a deciding player where applicable.
  The positive branch tests the decision or start of payment, not whether
  resulting events happened [CR#118.12]. The positive branch receives the
  body's introductions; the negative branch retains the incoming scope.
  The full `Compulsion` enum, including prohibition and toll, is not this policy.
- Give every expanded repetition an explicit process body and iteration
  policy. Fold `repeatTimes` into that structure. Macros resolve "repeat this
  process"; fixed counts, after-iteration decisions, and stopping conditions
  remain distinct. Carry prior selections explicitly instead of preserving
  `againExcludingChosen` as a bespoke policy. Preserve outer bindings,
  per-iteration choices, result publication, and payment admissibility.
- Use lists for semantic conjunction and alternatives in predicates,
  conditions, noun phrases, and costs. Preserve cardinality obligations and
  the difference between conjunction, alternatives, sequential composition,
  and simultaneous composition. Fixed-arity wording is macro syntax. Use
  the agreed pair `sequentially`/`simultaneously` for instruction composition;
  retain `establish`.

## Blast radius

The coordination change touches every `both`/`eitherOf`/`either`/`compound`
site in the bench and pin suites, the widest re-spell in this campaign.
Iterate on a card subset and run the full build once at the end. The landing
record's restored/re-spelled/removed counts are where a silently weakened
pin would show; a negative twin that loses its exact refusal list is a
regression, not a re-spelling.

## Completion evidence

Re-spell the affected cards and named pins through the replacement forms.
Exercise ambiguous numeric reads, comparison gaps that do not escape negation
or alternatives, multiple standing choices, both continuation scopes, and
repetition with fresh versus carried choices. Soul's Might and Phyrexian
Rebirth witness definitions that depend on earlier introductions; Ad Nauseam,
Another Round, and Forgotten Lore distinguish repetition policies.

For each retired constructor, record its replacement and any deliberate
change to an old refusal or published binding. Run `lean/scripts/build`.
Standard constraints apply. This does not reopen the parked Idris
[segment-indexed telescope](../maybe/workbench-segment-indexed-telescope.md).
