---
needs: [lean-v3-authoring-impl-11-replacement-and-entry-unification]
---
# Type `instead` as a dedicated `Replacement` rather than `Action → Action`

**Introduce a dedicated `Replacement` type in `lean/Semantics/Abilities.lean`
(symmetric with `WouldClause`) so `instead` produces a `Replacement` rather than
an `Action`, making `spell (instead ...)`, `instead (instead ...)`, and
`replace (would ...) (draw (.lit 2))` compile-time type errors (`[CR#614.1a,614.15,616.1a]`).**
Twelfth implementation step of [`lean-v3-authoring-surface`](lean-v3-authoring-surface.md).
Standard constraints apply.

## Scope

### 1. Core AST `Replacement` Type (`lean/Semantics/Abilities.lean`)

- Add `Replacement` to the mutual inductive block in `lean/Semantics/Abilities.lean`:
  ```lean
  inductive Replacement where
    | inCaller (body : Replacement)
    | instead (action : Action) (when : Option Condition := none)
  ```
- Retype the replacement parameter of:
  - `StaticSpec.replace (event : WouldClause) (replacement : Replacement) ...` (`[CR#614.1a]`)
  - `Action.replace (replaced : Action) (replacement : Replacement)` (`[CR#614.15,616.1a]`)
  - `replaceEvent` and `replaceNextEvent` in `lean/Semantics/Templates.lean`
- Add `CallerScope Replacement` (`lean/Semantics/CallerScope.lean`),
  `Authoring.inspect` support (`lean/Semantics/Authoring.lean`), and checker
  traversal (`stepReplacementWith` in `lean/Semantics/Check/Phrase.lean`,
  `Check/LocalShape.lean`, `Check/CardFacts.lean`, `Check/NumberSlots.lean`).

### 2. `instead` Template & Un-Inverted Call Sites (`Templates.lean`, `Definitions/`, `Cards/`)

- Change `semantic_template instead` in `lean/Semantics/Templates.lean` from
  `Action → Action` to:
  ```lean
  semantic_template instead (action : Action) (when : Option Condition := none) : Replacement :=
    .instead action when
  ```
  and provide conditional replacement phrasing (via `expectedTypeDispatch` on
  `doIf (cond : Condition) (then_ : Replacement) : Replacement` and/or
  `insteadIf (cond : Condition) (action : Action) : Replacement := .instead action (some cond)`).
- Un-invert the replacement call sites where `may` or `doIfDone` sat outside
  `instead`:
  - `dredge` (`lean/Semantics/Definitions/Keywords.lean`, `[CR#702.52a]`):
    `replace (would you (draw (.lit 1))) (instead (may (sequentially [mill amount, returnToHand this_ yourGraveyard])))`
  - `bewitchingLeechcraft` (`lean/Semantics/Cards/Counters.lean`):
    `replace (would thisCreature (become untapped)) (instead (doIfDone (removeCounters (exactly 1) (some (printedKind p1p1Counter)) thisCreature .many true) (some (untap thisCreature))))`
- Update all `StaticSpec.replace` and `Action.replace` sites across
  `Definitions/`, `Cards/`, and `Proofs/` (including `regeneration`'s
  `regenerationApplication` replacement in `Definitions/Actions.lean` and
  `Cards/Static.lean`).

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
