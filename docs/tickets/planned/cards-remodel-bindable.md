---
needs: [core-bindable-unification, engine-anaphor-threading, ron-emitter-bindable]
---
Re-model the hand-authored cards on the unified `Bindable`/anaphor model (see
[[core-bindable-unification]]). With the type, engine, and emitter layers in
place, the corpus moves to the sound shapes — and the mis-modeled cards are fixed.

## Changes (idris/src/Cards.idr shows the target shapes)

- **Brainstorm** → `Each(Choose(2, inHand), Move(It, Library(FromTop(0))))` —
  iterate the chosen two, moving each `It` (fixes the first-of-many: was
  `With(Choose(2), Move(That))` moving one). Resolves the
  `core-many-binder-group-move` seam.
- **Pyroclasm / Flame Rift** → `Each(<Bindable over creatures/players>,
  DealDamage(It, n))` — `It`, not `ThatObject`.
- **Arc Lightning** → `Targeted([1–3 any], Distribute(amount: 3,
  Existing(GetTargets(0)), DealDamage(It, Allotment)))`.
- **Scry / Surveil / Fateseal**, Enchant, Scavenge, and any other card/macro
  using the old anaphors — move to `Bindable` + `It`/`That`.
- Sweep `plugins/canon` + `plugins/builtin` for the old vocabulary
  (`ThatObject`/`Those`/`Subject`/old `Each.over: Selection`).

## Done
- `cargo test --workspace` green; `cargo +nightly fmt`, `cargo clippy
  --all-targets` clean on touched code; `cargo xtask cite check` 0 stale /
  0 non-compliant. Regenerate `plugins/wizards` and confirm no old-shape output
  (cf. the A/B method already used: 0 regressions, 0 old shapes in fresh output).
