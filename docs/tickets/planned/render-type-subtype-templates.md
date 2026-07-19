---
needs: []
---
**Render type/subtype references using their macro `template` (on-card
spelling), not the raw identifier.** A subtype whose identifier differs from its
printed form — `Urzas`→"Urza's", `Ctan`→"C'tan", `AssemblyWorker`→
"Assembly-Worker", `TimeLord`→"Time Lord" (see the `template:` overrides in
`plugins/*/macros/types/**`) — currently renders the bare ident, because render
is a PURE function of the stored value and takes no `MacroSet`
(`crates/deckmaste_cards/src/render/mod.rs:142,381`), so a name→registry lookup
at render time is impossible. Every subtype/type render site reads the raw
`Ident` (`render/condition.rs:305`, `render/fragment.rs:729,1007`).

This is a LATENT gap, not caused by any card yet (no card filters on a
templated subtype today), and it affects BOTH:
- printed type-lines (`subtypes: [Urzas]` → the struct's raw name renders
  "Urzas"), and
- filter atoms (`Subtype(Urzas)` after `filter-permanent-of-type-subtype`).

The counter analogue shows the same shape: `P1P1Counter`→"+1/+1" is a
HARDCODED table (`render/effect.rs:1638`), not a template read.

## Direction

The one render path that DOES emit a macro's template is `Predicate::Expanded`
(and the per-position `::Expanded` twins): the template + args are captured
INLINE on the value at expansion, and `crates/deckmaste_cards/src/render/
template.rs` fills that stored template. So route templated type/subtype values
through that mechanism — capture the macro `template` inline at expansion so
render emits the on-card spelling — rather than teaching render to reach a
registry (it structurally can't). Design the boundary: which values carry an
inline template vs stay raw, and how the `subtype_exclusion_prefix` /
`basic_land_you_control` raw-ident sites interoperate.

## Scope

Render fidelity, `planned/`. Fixes type-lines and filter atoms in one stroke.
Standard constraints apply (render parity for the non-templated majority must be
byte-identical). Related: `filter-permanent-of-type-subtype` (the filter payload
whose templated values this renders), `render-multi-element-fidelity-gaps`.
