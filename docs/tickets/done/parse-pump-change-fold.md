---
needs: []
---
**Fold the `±N/±N` change into the `PowerAndToughnessUp`/`Down` macros on the
durational (pump) path too, so the macro surface persists into generated cards
everywhere — not just on static anthems.** Closes the asymmetry left by the
template-forward parser fold.

Related: [[macro-second-wave]], [[macro-first-wave]].

## The asymmetry

The static-ability parser (`crates/deckmaste_migrations/src/parsers/static_ability.rs`,
`parse_pt`) already folds an anthem's change through the reverse `TemplateIndex`:
it calls `ctx.index.match_with("Modification", "gets +N/+N", count_delim_slot_reader)`
and emits `PowerAndToughnessUp(n, n)` / `PowerAndToughnessDown(n, n)`, which then
persists into the generated card.

The **durational** path does not. `effect::parse_pump` (reached from the
one-shot / spell / triggered / replacement productions for "… gets +N/+N until
end of turn") shares the `±N/±N` change grammar via
`modify::parse_pt_changes`, but never consults the reverse index — it emits the
structural `Several([Power(Up(n)), Toughness(Up(n))])`. So "Target creature gets
+2/+2 until end of turn" graduates without ever retaining the
`PowerAndToughnessUp` macro, while the static anthem does.

(Note the `parse_pump` doc comment already flags this: "the changes are written
inline because this production does not consult the reverse `TemplateIndex` (a
later task will fold `±N/±N` here …)". This ticket is that later task.)

## Why it's safe / low-risk

`Modification::flatten` (`crates/deckmaste_core/src/continuous.rs`) collapses
BOTH the macro form (`Expanded(PowerAndToughnessUp(p, t))` → `Several([Power(Up(p)),
Toughness(Up(t))])`) and the structural form to the identical flat op list before
the engine or the render's `modifications_predicate` see them. So graduation,
render fidelity, and the Idris typecheck are unaffected by whether the source
carries the macro or the structural ops — this change is **purely about
surface-retention**, the whole point of the macro system (macros persist
unexpanded into generated cards).

The hardcoded `PowerAndToughnessUp` shortcut that USED to give the pump path a
half-fold (both-positive only, index-bypassing) was deleted when the static fold
landed — core lowering now emits only structural ops, and the macro name comes
exclusively from the index. This ticket restores the macro surface on the pump
path the RIGHT way: via the index, covering all sign combinations.

## Scope

- Thread the resolve `ctx` into `effect::parse_pump` and apply the same index
  fold as `static_ability::parse_pt` before emitting the change (reuse the
  existing bounded `count_delim_slot_reader`). The grant-tail combo ("gets +2/+2
  and gains haste until end of turn") must still fall through to the core
  `Several(...)` fallback when the whole `gets …` phrase doesn't fully match the
  `Modification` template — same full-consumption guard the static fold uses.
- The productions that reach `parse_pump` (spell / triggered / replacement /
  one-shot) inherit the fold for free once `parse_pump` folds; confirm each still
  type-checks after `parse_pump` gains `ctx` (the surrounding productions already
  thread `ctx`).
- Update `parse_pump`'s doc comment to state the fold now happens (drop the
  "a later task will …" promise).
- Optional extension (call out, do NOT silently expand): the durational **subject**
  ("creatures you control get +N/+N until end of turn") could also fold to the
  Selection macros the way the static anthem's subject does — a natural follow-up,
  but the change-side fold is the headline of this ticket.

## Done

- `effect::parse_pump` folds `±N/±N` to `PowerAndToughnessUp`/`Down` via the
  index; a durational-pump card (e.g. a "+2/+2 until end of turn" pump) graduates
  with the macro RETAINED, verified by a `parsed_with_macros`/builtin-index test.
- The ~11 durational/pump tests currently asserting structural `Several([Power…,
  Toughness…])` (across effect.rs / spell_ability.rs / triggered_ability.rs /
  replacement.rs / resolve.rs) are updated to the folded macro form where the
  builtin index is in play; empty-index tests keep the structural fallback.
- `parse_pump` doc comment updated.

## Verification

- `cargo test --workspace` green; `cargo clippy --all-targets -- -D warnings` clean.
- Wipe-first wizards regen; graduation count unchanged or higher (flatten makes
  the two forms engine-identical, so no card should drop).
- `cargo xtask fidelity plugins/canon` green (render is flatten-first, unaffected).
