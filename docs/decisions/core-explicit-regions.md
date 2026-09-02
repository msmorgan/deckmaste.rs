# Core is explicit regions

**Every binding in `deckmaste_core` is a declared parameter or a numbered
definition, and every reference is a register read. Core never resolves
anything; lowering resolves once, and the engine reads by index.**

Decided 2026-09-02. Amends [Semantics, spelling, lowering](semantics-spelling-lowering.md)
§17: the per-scope slot environment it previews becomes closed regions with
declared parameters, and the frame-local candidate tier it kept is gone.
Supersedes the `core-reference-slots` ticket (deleted the same day) and the
engine tickets its stages absorb (listed in each stage ticket).

## The contract

1. **Regions.** A region is a closed function: `Region { params, body }`.
   Regions are an ability, each modal mode, each delayed or reflexive
   trigger body, each carried body (`Composite`, a floating replacement),
   each `Each`/`Distribute` body, and every per-candidate predicate
   (`SelectAll`, a `TargetSpec` filter, `Where`, `Pick`, `Aggregate`, a
   continuous per-subject read). A region reads nothing it did not declare.
2. **Params.** `Param { def: DefId, kind, provenance }`. Provenance is typed
   data, never a string: source, controller, event object/patient/actor,
   defending player, announced target `k`, announced X, capture of the
   parent's register `r`, linked memory cell `m`, loop element, allotment,
   candidate. The engine dispatches denotation on it: an announced slot
   keeps the partial-fizzle read [CR#608.2b], a trigger source reads LKI, a
   move product is a new object [CR#400.7]. Param order per region kind is
   fixed by lowering: source, controller, event roles, targets, X; a delayed
   body's captures follow its own event roles; a loop body's element (and
   share) and a predicate's candidate stand alone.
3. **Definitions.** `DefId`s are sequential in textual order per region,
   params first. Only an instruction with a program point defines: a
   decision (`Choose`, `ChooseValue`, `Search`, `SeparatePiles`,
   `ChoosePile`), an action's product (`Act { dest: Option<DefId>, action }`),
   a dig (`RevealUntil`), a cost's paid product, and `Let { dest, expr }`
   when English fixes an evaluation moment, as "that many" does after a
   count [CR#608.2h].
4. **References.** `Reference::Reg(RefId)` is the only binding read. `This`,
   `You`, `Opponent`, `DefendingPlayer`, `EventObject`, `EventPatient`,
   `EventActor`, `It`, `That(Sort)`, `Target(n)`, `Bound(Ident)`,
   `Linked(Ident)`, `Source`, `Selection::They`/`Them`/`Targets(n)`/
   `PilesOf`/`AmongNoted`, and `Count::ThatMany`/`ThatMuch`/`Allotment`
   leave core; semantics keeps its anaphors. Pure derivations
   (`ControllerOf`, `OwnerOf`, `AttachHostOf`, `Coalesce`, `Single`,
   `OpponentOf`, counts over selections) stay nestable expressions
   evaluated at the read site: they introduce no binding and get no def
   unless a `Let` pins them.
5. **Instruction vs expression.** An instruction has a program point: an
   effect, a decision, or a `Let`. An expression is pure and decision-free.
   The validator rejects a decision-bearing form (`Choose`, `Random`, a
   decision-bearing `Pick`) inside an expression. Every pause in a
   resolution therefore lands on an instruction boundary, and resume is
   "write the register, advance".
6. **Structured, not flat.** Instructions carry nested blocks (`If`, `May`,
   `Modal`, `Repeat`, `Batch`, `Simultaneously`, `Each`, `Distribute`,
   `Search.if_none`). A def is visible to later instructions in its block
   and in nested blocks, never after the block closes. There is no phi: a
   value needed after a branch is defined before it.
7. **Captures.** `Delayed`, `Reflexive`, and carried bodies declare
   `captures: [RefId]`; the created region takes them as params with
   capture provenance, snapshotted at creation [CR#603.7,603.12]. Nothing
   else crosses a region boundary.
8. **Linked memory** [CR#607]. A card declares its memory cells;
   `Remember { cell, value: RefId }` writes one; a reading ability takes the
   cell as a param with linked provenance.
9. **Announcement is declared on the ability.** Targets and costs are fields
   of `SpellAbility`, `ActivatedAbility`, `TriggeredAbility`, and `Mode`,
   never effect nodes [CR#601.2b,601.2c]. The cost is a block run at
   announcement in the ability's own activation, so a paid product is a def
   the body reads. `OneShotEffect::Targeted` and the root `AdditionalCost`
   are deleted.
10. **Validate at load, fizzle at run.** `deckmaste_core::validate` checks
    def sequence, dominance, kind, region closure, and the expression
    carve-out. After it, a runtime read can only find a bound-but-departed
    value, which stays under [Invalid semantic input fizzles](invalid-semantic-input-fizzles.md).
    "Never bound" is unrepresentable.
11. **The engine environment is an activation record.** One activation per
    region entry: a `Vec<Value>` sized at load, kept in a per-resolution
    table on `GameState`; a work item carries an activation id, never a
    copy, because `Sequentially` clones frames into child work items up
    front. `Frame` collapses to that id plus payment state. `eval_reference`
    is an indexed read plus provenance dispatch returning one product (live
    id, optional LKI). A predicate over N candidates writes its candidate
    param N times, sequentially.
12. **Resolution happens once, in lowering.** Lowering is the compiler:
    scope elaboration, param declaration, lambda lifting for predicates,
    closure conversion for captures, R1/R2 as per-card diagnostics. The
    Idris mirror stays the certifier of semantic input, and the two
    derivations are gate-compared (§17's certifier/resolver split,
    unchanged).
13. **Numeric ids, typed riders, a printer.** §14's rejection of stored
    binder names stands. Core is generated, so verbosity is free; a
    pretty-printer that shows riders is the debugging affordance.

## Rejected

- **A flat CFG with phi nodes (LLVM IR).** Built for optimization passes the
  engine never runs; this interpreter pauses on decisions and re-enters
  nested bodies.
- **A def per derivation.** Doubles the instruction count without adding a
  binding; `Let` covers the cases where the moment matters, and memoizing
  pure expressions stays an engine option.
- **A frame-local tier for loop variables and candidates** (the
  `core-reference-slots` default). A second environment with its own read
  rule is the hiding this decision forbids.
- **A register file inside `Frame`.** See law 11.
- **A `_v2` core beside the old.** The stage chain evolves one core and
  deletes each channel's old variants as its explicit form lands.

## Staging

Four stages, each behind `core-demacro`: after it, engine fixtures spell
semantics and `lower`, so the plugin corpus and the fixtures are untouched by
core-shape changes and each stage is lowering plus engine plus core types.

`core-regions-substrate` → `core-regions-discourse` →
`core-regions-costs-and-captures`, with `core-regions-piles` off the
discourse stage. Each landed stage carried residue into a follow-up rather
than reopening its own ticket, so the live node set is larger than the four:
`core-regions-substrate-closeout` (the compat register spellings and the
frame collapse), `core-regions-discourse-closeout` (runtime nearest-antecedent
search, the shared choice slot, per-card lowering diagnostics, the
resolver/certifier differential gate), `core-regions-test-restoration` (the
assurance the discourse stage deleted, plus that stage's seven named
fixtures), `core-regions-captures-and-memory` (laws 7 and 8, split off when
the cost half alone ran to ~4k lines), and `core-regions-witness-fixtures`
(hand-spelled semantic fixtures for the nineteen inherited witness cards,
which the wizards corpus cannot exercise). A stage is closed by its own
ticket plus its follow-ups, not by its ticket alone.

Core's shape is independent of which semantics lowers into it. The chain
builds the resolver on today's `deckmaste_semantics` → core path as the
proof of concept; the `semantics_v2` being designed in the workbench gets its
own front half later and reuses the region-building and R1/R2 back half.
Nothing in this contract waits on `semantics_v2`.
