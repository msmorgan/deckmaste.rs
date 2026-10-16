---
needs: [lean-v3-proofs-and-cards]
---
# Retire deonticActionFor and cut over deontic templates to v3 Instructions

**Delete the `v2`-port `deonticActionFor (deed : Deed) (patient : NounPhrase)`
shim in `lean/Semantics/Macros.lean` and replace all `Deed`-taking deontic
templates and keyword helpers with `v3` `Instruction`-taking templates on the
Unified Action-Clause Spine.** Sub-ticket 1 of
[`lean-v3-authoring-surface`](lean-v3-authoring-surface.md). Standard
constraints apply.

## Context & Problem

During the `v2 -> v3` port, a plain `def deonticActionFor (deed : Deed)
(patient : NounPhrase) : Instruction` shim (`lean/Semantics/Macros.lean:867-916`)
was left in place so `objectCant`, `playerCant`, `cantMoreThan`, `cantDoTo`,
`canDoAsThough`, `mayPlayDeed`, and several `Declared.lean` helpers
(`cantBeBlockedBy`, `cantBeBlockedExceptBy`, `cantExcept`, `flyingExpansion`,
`warp`, `webSlinging`, `detain`) could keep taking legacy `v2` `Deed` values
(`(.action "untap")`, `(.core .loseGame)`, `(.core .attack)`).

In `v3` (`docs/decisions/semantics-v3.md` §3.1), `StaticSpec.cant`, `.must`,
`.may`, and `.gatedBy` take `(subj : Subject) (action : Instruction)` directly:
- Active prohibition: `cant subj action` (`Subject.actor np`, e.g.,
  *"This creature can't block"* -> `cant this (block any)`).
- Unspecified-agent passive prohibition: `cant .any action` (e.g.,
  *"This spell can't be countered"* -> `cant .any (counter this)`).
- Explicit-agent passive prohibition: `cant (.by agent) action` (e.g.,
  *"This creature can't be blocked by Goblins"* ->
  `cant (.by (a (.hasSubtype .goblin))) (block this)`).

## Scope

1. **Design and land `v3` `Instruction`-taking deontic `Template`s:**
   - Replace `Deed`-parameterized `objectCant`, `playerCant`, `cantMoreThan`,
     `cantDoTo`, `canDoAsThough`, and `mayPlayDeed` with surface-faithful
     `Instruction`-taking templates (`cant`, `cantBe`, `cantMoreThan`,
     `canAsThough`, `mayPlay`, `mayCast`, etc.) that expand directly into
     `StaticSpec.cant`, `.must`, `.may`, and `.gatedBy` with `DeonticMods`.
   - Update `cantBeBlockedBy`, `cantBeBlockedExceptBy`, `cantExcept`,
     `flyingExpansion`, `warp`, `webSlinging`, and `detain` so they construct
     `Instruction`s directly without `Deed` parameters or `deonticActionFor`.
2. **Delete `deonticActionFor` and `mayPlayDeed`:**
   - Remove `def deonticActionFor` from `lean/Semantics/Macros.lean` and ensure
     no helper maps a `Deed` back into a synthetic `Instruction`.
3. **Update all call sites in `Cards/*.lean` and `Proofs/*.lean`:**
   - Re-express all deontic card witnesses (`lean/Semantics/Cards/Deontic.lean`,
     `Static.lean`, `Keyword.lean`, etc.) and proof witnesses
     (`lean/Semantics/Proofs/Deontic.lean`, `DrtCards.lean`, etc.) onto the new
     `Instruction`-taking deontic templates.

## Proof

- `deonticActionFor` and `mayPlayDeed` are completely absent from `lean/Semantics/`.
- Every deontic template and call site in `lean/Semantics/` takes an
  `Instruction` (never a `Deed`) for the prohibited, required, permitted, or
  gated action.
- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
