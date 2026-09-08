---
needs: []
---
**A declaration's body is the semantic meaning of the macro; facts derive
from it; Rust owns no mapping into the RON files.** Rulings (user,
2026-09-07), superseding `facts-generator-sheds-v1`'s "a declaration
carries its own facts columns as its body" and the xtask overlays that
`facts-generator-sheds-v1` left standing.

1. **Definition nodes in Lean.** `lean/Semantics/Words.lean` (beside
   `CounterKind`) gains definition constructors, the idiom of
   `Ability.keyword (keyword) (params) (body : List Ability)`:
   `Counter (label) (holder : Kind) (confers : List Conferral)`;
   `Subtype (category) (label) (rules : List <rule record>)` carrying the
   four rules-defined conferrals (equipment, fortification, aura, saga —
   `docs/tickets/done/builtin-v2-noncreature-subtype-stubs.md`) and empty
   otherwise; `Designation (label) (scope) (effectful) (zone) (type)
   (half)` — the current `DesignationFacts` columns as a semantic node,
   with `semantics-v2-designation-storage-columns`' three dropped columns
   reconsidered as fields of it. Each with CR citation and pins, mirrored
   under the drift test. The 20 counter bodies already written as
   `Counter(name:, confers:)` are the shape; the 51 `CounterFacts` rows,
   19 `DesignationFacts` rows and 458 empty subtype bodies are rewritten
   to definitions mechanically (scoped script, comments untouched).
2. **The facts generator derives.** `cargo xtask facts generate` computes
   `CounterFacts`, `DesignationFacts`, `SubtypeFacts`, `KeywordFacts` and
   `ActFacts` rows from the declarations alone. Every xtask overlay
   retires: `ACTION_OVERLAY` (65 rows), `overlay()` (keyword gate
   columns), the subtype frame rows, `DESIGNATION_MAP`, and the
   `label_of`/`name_of` name mapping. For each column the landing record
   states one of: *derived* (from the body, with the derivation named) or
   *declared* (a field of the definition node the macro itself carries).
   A column that is neither is a STOP. `Facts.lean` and `FactsGen.idr`
   are expected byte-identical; a difference is a defect in the
   derivation unless the record shows the overlay value was wrong
   against the CR, in which case the derived value wins and the row is
   listed.
3. Then `plugins-v2-subtypes-macro-only` and a counters twin can run: a
   definition body is an expansion the macro can carry, so `Subtype` and
   `CounterKind` may join the restricted kinds.

Design-bearing; Lean first; `lean-check` 118/118 and 2/2 and
`lean/Generated` byte-identical throughout. Standard constraints apply.
