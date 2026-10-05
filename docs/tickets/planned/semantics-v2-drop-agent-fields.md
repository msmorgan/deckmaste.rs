---
needs: [plugins-v2-implicit-actor-spelling]
---
**Delete the agent fields from the model once nothing spells them.** Landing
(3) of the actor-handoff design agreed with the owner on 2026-10-05 (see
`semantics-v2-actor-handoff`). After `plugins-v2-implicit-actor-spelling`
every helper fills these fields with `actor`, so the field says nothing the
enclosing handoff does not. Owner: "Maybe the explicit actors version is more
a concern for lowering/core." Lean first, then the mirror. Standard
constraints apply.

## The change

1. **Lean** (`lean/Semantics/Abilities.lean`). Delete the 16 required agent
   fields — `conclude`, `separateIntoPiles`, `vote`, `copy`, `changeLife`,
   `addMana`, `draw`, `expose`, `search`, `shuffle`, `flipCoins`, `rollDice`,
   `rerollStored`, `createObject`, `pay`, `skipPart` — the 3 optional ones —
   `choose`, `enact`, `insertPart` — and `ContinuationPolicy.optional`'s
   agent. The checker reads the performer from the enclosing handoff (or the
   controller when there is none) wherever it read the field:
   `agentIntro`/`agentCtx` (`Check/Phrase.lean`), `Instruction.check`
   (`Check/AbilityRules.lean`), `Instruction.profile`, `mayCtx`,
   `enactKeepsOuter`, and the cost rules that read `NounPhrase.isYou` on an
   agent (`Check/Abilities.lean` ~L805, `changeLife` and `enact`). `agentRef`
   (`Macros.lean` ~L732) and the macros that pass `agent :=` follow.
2. **The Lean bench** (`lean/Semantics/Cards`) is re-spelled over `act`: the
   453 `agent := Primitives.NounPhrase.you` arguments are dropped, and every
   other explicit agent (`each …`, `target …`, `controllerOf …`, `they`,
   `that …`, about 214 more sites) becomes an `act` around the instruction.
   Pins keep their asserted outcomes.
3. **Mirror.** `crates/deckmaste_semantics_v2` drops the same fields;
   `lean_drift` holds.
4. **RON.** The helpers stop writing `actor` into the deleted slots;
   `keywords.rs` writes `Enact(Action(label), body)`.

## Proof

- `cargo xtask lean-check` passes with every pin at its old outcome.
- `cargo xtask expansions` before/after differs only by the removed fields:
  for every declaration and card, the expansion after equals the expansion
  before with each `agent: actor` (or the field's position) removed. List any
  other difference.
- Every bench theorem re-spelled is counted in the landing record
  (re-spelled, none removed).

## Out of scope

- How lowering redistributes a handoff to the performing players
  (`lowering-v2`).
