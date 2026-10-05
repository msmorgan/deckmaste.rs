---
needs: [semantics-v2-counter-kind-is-a-name]
---
**A designation reads bare by its declaration name (`monstrous`, `goaded`),
not as a string.** Split from `semantics-v2-counter-kind-is-a-name` on
2026-10-05 because it is a second landing over a different type; it follows
that ticket so the typed-name mechanism is built once. Lean first; standard
constraints apply.

## What is decided

- Owner, 2026-10-05, on reading designations bare: "I'd prefer to avoid
  stringly typed so yes."
- Orchestrator's call (the owner did not object): `DesignationLabel` becomes a
  small proper type rather than a `String` alias (Lean `abbrev
  DesignationLabel := String`, `lean/Semantics/Words.lean` ~L169; mirror
  `pub type DesignationLabel = String`,
  `crates/deckmaste_semantics_v2/src/words.rs` ~L265), so the `denoted_by`
  marker attaches to it the way it does to `CounterKind` after
  `semantics-v2-counter-kind-is-a-name`.
- The name is the declaration name, verbatim, as for counter kinds.

## The work

1. **Lean.** Retype `DesignationLabel`; the designation facts lookup and
   `RoomHalf.designation` (`Check/Words.lean`) follow. Pins keep their asserted
   outcomes.
2. **Mirror.** Follow in `crates/deckmaste_semantics_v2`, with `denoted_by` on
   the new type; `lean_drift` holds.
3. **Declarations.** The designation declarations under
   `plugins_v2/builtin/macros/designations/` write their own label as a
   string today (`Designation(label: "monstrous", …)`); they take the name.
4. **Bodies.** Re-spell the string labels in the 11 files that write one
   (`gainDesignation`, `setGameDesignation`, `makeMonstrous`, `designated`,
   `hasDesignation`, `yourCommander`, `goad`, `harness`, `monstrosity`,
   `suspect`, `renown`): `hasDesignation("renowned")` becomes
   `hasDesignation(renowned)`.

## What the counter landing gives this one

`semantics-v2-counter-kind-is-a-name` built the pattern; it is three pieces,
each copyable:

- **The type.** Lean `inductive CounterKind | named (name : String)` with a
  `CounterKind.name` accessor; mirror `enum CounterKind { Named { name: String
  } }` with `name()`, carrying `#[macro_ron(denoted_by(Counter, term =
  crate::rules::Definition::counter_term))]` unchanged. A one-constructor
  inductive because `denoted_by` attaches only to an enum. For designations:
  `DesignationLabel.named (name : String)`, mirror `Named { name }`, and a
  `#[macro_ron(denoted_by(Designation, term =
  crate::rules::Definition::designation_term))]` with a new
  `designation_term` projection (Lean `Definition.designationTerm`; the doc
  comment on `Definition.subtypeTerm` already anticipates it).
- **The name comes from the meta, not the body.** The `denoted_by`
  projection reads the EXPANDED body, which does not know the macro's name,
  so the definition node must carry it, and a declaration must not write it
  (that would be self-reference). The counter meta takes the definition's
  fields as parameters and writes `Counter(kind: Named(name: Param(name)), …)`
  itself; `meta/Designation.ron` would take `scope`, `effectful`, `zone`,
  `type`, `half` and write `Designation(label: Named(name: Param(name)), …)`.
  `deckmaste_construction_core`'s declaration schema then needs a
  `DiagnosticDesignation` with a derived body (see `DiagnosticCounterKind`
  and `DiagnosticSubtype` in `macro_def.rs`).
- **Facts.** The generator reads each body through the builtin plugin's
  macros (`counter_definitions` in `crates/xtask/src/facts/lean.rs`) and
  refuses a term whose name is not the declaration's own; the facts row's
  label is the name. Every Lean pin, bench card and RON body that writes a
  label string then re-spells to the name (`.named "monstrous"`, bare
  `monstrous` in RON); the counter landing's normaliser for the
  expansion/card-dump comparison is at the orchestrator's scratchpad
  (`counters/normalise.py`).

One difference: designation labels are today the spelled phrase
("the monarch", "left half unlocked"), not the declaration name, so every
label changes text, and any check that reads the label as English (Room
halves, `RoomHalf.designation`) must read the name instead.

## Proof

`cargo xtask lean-check` passes; `cargo xtask expansions` before/after is
identical except for the label's printed form, every change listed.

## Related

`semantics-v2-designation-storage-columns` (planned) owns the storage columns
the designation declarations lost; it does not block this.
`plugins-v2-out-of-scope-keywords` removes `sector`; whichever lands second
has one designation fewer to re-spell.
