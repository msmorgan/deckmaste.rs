---
needs: [lean-grammatical-reference-scopes]
---
[design] **Express the relational subject and sacrifice deed currently fused
in `Instruction.controllerSacrifices`.** The constructor in
`lean/Semantics/Abilities.lean` has dedicated validation and profile arms in
`Check/AbilityRules.lean` and `Check/Abilities.lean`. Its profile introduces a
player, moves the subject, and stamps the sacrifice provenance. The existing
`Cards/Anaphora.lean` witness is `arcumDagssonSacrifice` with
`okArcumDagssonSacrifice`.

Use the grammatical-reference decision to represent the controller of the
named object as the subject of the deed and read the same object in its body.
Do not duplicate the target declaration when expanding the phrase. Determine
whether the existing noun/clause machinery suffices or which general mechanism
is missing; the exact replacement is not settled. The established direction
removes constructors whose meaning is a composition of other constructs.

Done when the existing witness is re-spelled and still passes, the later
player and moved-object reads retain their identities and provenance, and
`controllerSacrifices` plus its dedicated checker/profile arms are removed.
Pin target multiplicity and reference behavior through Lean LSP, and preserve
the existing card and pin assertions in `lean/scripts/build`.
