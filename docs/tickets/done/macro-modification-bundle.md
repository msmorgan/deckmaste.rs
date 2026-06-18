---
needs: []
---
Make `Modification` a macro kind so change-bundling macros become possible — the
keystone being an `AddPowerToughness(Count, Count)` macro (template
`"gets +{0}/+{1}"`) that the pump/anthem emitters use instead of writing the
`[AddPower(Literal(N)), AddToughness(Literal(N))]` pair inline.

Why it doesn't work today: `Modification` derives only `Expand` (not
`SupportsMacros`) and isn't in `deckmaste_cards`'s `kinds()`, so no macro can
stand in a `changes: [...]` slot; and macros expand to a single value, so a
macro can't splice two ops into the list. The fix mirrors how `Filter` macros
work (a `Filter` macro can expand to one `AllOf([...])`): give `Modification` a
composite variant that bundles several ops into one value.

Three coupled changes:
1. Add a composite variant, e.g. `Modification::Several(Vec<Modification>)`
   (analogous to `Filter::AllOf`), in `crates/deckmaste_core/src/continuous.rs`.
2. Convert `Modification` from `#[derive(Expand)]` to
   `#[derive(SupportsMacros)]`, add an `Expanded(Expansion<Modification>)`
   variant (this is what remembers the `AddPowerToughness(3,3)` invocation so the
   template can render it back), and register `Modification::kind()` in
   `crates/deckmaste_cards/src/macros.rs` `kinds()`.
3. Build the "unnest after" flatten — NO precedent exists: `expand_all` over a
   `Vec` is strictly element-wise (`macro_ron/src/traverse.rs`, the `Vec` impl
   maps, never splices), so after expansion the changes list still holds
   `[Several([AddPower, AddToughness]), GainAbility(...)]`. A new normalization
   over the `changes: Vec<Modification>` container must splice `Several` into its
   parent (recursively). It can't live in `Modification`'s own `Expand` impl
   (`expand_all` returns `Self`, can't turn one element into many) — it's a pass
   over the container, run after `expand_all` and before the engine.

Pipeline once built: stored/read = `[Expanded(AddPowerToughness(3,3)),
GainAbility(...)]` (invocation remembered) → render walks the stored form via the
template (`"gets +3/+3 and gains trample"`) → engine path does `expand_all`
(→ `Several([...])`) then flatten (→ `[AddPower, AddToughness, GainAbility]`).

Caveat to settle in the design dialogue: unlike `Filter::AllOf` (a real
conjunction the engine evaluates) or `Effect::Sequence` (real ordered "then"),
this composite is semantically inert — `changes` is already a flat,
layer-spanning list, so `Several` exists only to be flattened away. It'd be the
first such "flatten-away" node. Decide: flatten at load (this ticket's plan), OR
teach the two engine change-loops in `layer.rs` (`for m in &effect.changes`) to
recurse through `Several` (smaller, but the node persists and the layer matcher
must skip it).

Then re-point the emitters: `parsers/modify.rs` `parse_pt_changes` (used by
`static_ability` anthems and `effect`'s durational pumps from `gen-pump-effect`)
emits `AddPowerToughness(Literal(N), Literal(N))` instead of the inline pair;
update the `PumpThisUntilEot` builtin macro body to match; refresh the affected
parser tests and regenerate wizards (the change is cosmetic in the graduated RON,
so confirm round-trip + no graduation count regression). Pure authoring/rendering
quality + DRY; the current inline form keeps working until this lands.
