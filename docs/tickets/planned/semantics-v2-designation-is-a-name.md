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

## Proof

`cargo xtask lean-check` passes; `cargo xtask expansions` before/after is
identical except for the label's printed form, every change listed.

## Related

`semantics-v2-designation-storage-columns` (planned) owns the storage columns
the designation declarations lost; it does not block this.
`plugins-v2-out-of-scope-keywords` removes `sector`; whichever lands second
has one designation fewer to re-spell.
