---
needs: [lean-v3-authoring-impl-2-templates-and-definitions]
---
# `lean-v3-authoring-impl-3-cards-reauthoring`

Re-author all 15 card corpus files in `lean/Semantics/Cards/*.lean` onto the curated
`Templates.lean` and `Definitions.lean` surface, eliminating all transitional `Primitives.*`
qualifications and completing the `lean-v3-authoring-surface` epic.

See [`lean-v3-authoring-surface`](../planned/lean-v3-authoring-surface.md) (§§1–6) for the
complete settled design specification.

## Why this is the third and final implementation ticket

With the Core AST and `Check/Phrase.lean` settled in
`lean-v3-authoring-impl-1-core-ast-and-checker` and `Macros.lean` + `Macros/Declared.lean`
consolidated into `Primitives.lean` + `Templates.lean` + `Definitions.lean` in
`lean-v3-authoring-impl-2-templates-and-definitions`, every card file in `lean/Semantics/Cards/*.lean`
can be re-authored in a single pass directly onto the final `Templates.lean` and
`Definitions.lean` surface without revisiting card files multiple times.

## Scope

### 1. Close remaining `Templates.lean` & `Definitions.lean` gaps from `impl-2` (§§2.4, 5.4)

1. **Eliminate the 4 remaining `Primitives.*` defaults in `Definitions.lean`:**
   - Add `semantic_template itsOwnCost : PlayPayment := .itsOwnCost` (plus `withoutPaying : PlayPayment` and `payingInstead (cost : Cost) : PlayPayment`) and `semantic_template anyBallot : Ballot := .anyBallot` to `Templates.lean` so `cast`, `play`, `activate`, and `vote` in `Definitions.lean` use template defaults and `Definitions.lean` has zero `Primitives.*` references.
2. **Eliminate exponential elaboration on nested `and` / `or` / `not`:**
   - Replace the `export` overloads on `and`, `or`, and `not` in `Templates.lean` (which branch $3^d$ bottom-up across `Predicate`, `Condition`, `NounPhrase`, and `Location`, taking ~2s at depth 5 and ~12s at depth 6) with single-method typeclasses (or top-down expected-type dispatch, as `AnyValue` does for `any`) that propagate the expected type in $O(1)$ per node while still unfolding cleanly in `Authoring.inspect` to the underlying `@[semantic_template]` calls.
3. **Close the 3 card-bench expressiveness gaps:**
   - ***Spirit of the Labyrinth* (*"Each player can't draw more than one card each turn"*):** Add `Amount.moreThan (bound : Amount)` (matching `Amount.upTo (bound : Amount)`) and its template so `cant eachPlayer (draw (moreThan 1)) (timing := some eachTurn)` expresses the bound instead of `draw anyAmount`.
   - ***Mungha Wurm* & *Rites of Flourishing*:** Add `duringYourUntapStep` and `onEachOfTheirTurns` timing templates to `Templates.lean` and use them on those cards.

### 2. Re-author all 15 `lean/Semantics/Cards/*.lean` files (§§1–6)

Re-author all card definitions across `lean/Semantics/Cards/*.lean` so that:
- Every card ability, trigger, cost, static effect, continuous modification, and action
  is expressed through the curated `Templates.lean` and `Definitions.lean` vocabulary.
- All transitional `Primitives.*` qualifications are replaced by curated `Templates.lean` or
  `Definitions.lean` references.
- Zero `Primitives.*` references remain anywhere in `lean/Semantics/Cards/*.lean` or
  `lean/Semantics/Definitions.lean`.
- If any card requires a domain concept not yet covered by `Templates.lean` or `Definitions.lean`,
  promote that concept to a clean, reusable `semantic_template` in `Templates.lean` (or a named
  rule-grounded definition in `Definitions.lean`) rather than reaching for `Primitives.*`.

### 3. Glossary (`Action` vs. `Instruction`) and `lean/CONTRACTS.md` sweep (§6.3)

- In `docs/contexts/game-model/CONTEXT.md`, rename the **Instruction** entry to **Action** (listing
  `Instruction` under `_Avoid_`) and update remaining `instruction` parameter/doc names in
  `lean/Semantics/`.
- Sweep the remaining stale pre-DRT (`Bindings`, `Window`, `capture`, etc.) identifiers in
  `lean/CONTRACTS.md` so the document reflects the live DRT checker and `v3` AST.

## Proof

- `rg 'Primitives\.' lean/Semantics/Cards/ lean/Semantics/Definitions.lean` returns 0 matches.
- `./lean/scripts/build` (`lake build --wfail`) and `cargo xtask lean check` pass with zero
  warnings and zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` empty and `cargo xtask cite check` 0 stale.
