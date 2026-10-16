---
needs: [lean-v3-authoring-1-deontic-cutover]
---
# Unify verb and observation templates across does, would, did, and cant

**Provide `Instruction` templates for intransitive and transitive verbs and
observations (`die`, `enter`, `leave`, `putInto`, `phaseOut`, `phaseIn`,
`turnFaceUp`, `loseGame`, `winGame`, `castSpell`, `attack`, `block`,
`dealDamage`, etc.) expanding into the minimal Core AST primitives so `does`,
`would`, `did`, and `cant` share a single verb vocabulary without raw
constructor calls.** Sub-ticket 2 of
[`lean-v3-authoring-surface`](lean-v3-authoring-surface.md). Standard
constraints apply.

## Context & Problem

In `v3` (`docs/decisions/semantics-v3.md` §3.1), `GameEvent.does`,
`WouldClause.would`, `DidClause.did`, and `StaticSpec.cant`/`.must`/`.may`/`.gatedBy`
all embed `action : Instruction`. However, in `lean/Semantics/Macros/Primitives.lean`
and `lean/Semantics/Macros.lean`, many observation and movement verbs (`dies`,
`enters`, `leaves`, `putInto`, `isDealtDamage`, `dealsDamage`, `attacks`,
`blocks`, `becomesBlocked`, `flipsCoin`) only exist as `GameEvent`-returning
templates.

Whenever those same verbs are needed in a replacement effect (`would`), a
lookback query (`did`), or a prohibition/permission (`cant`/`may`), card and
proof authors have had to drop down to raw 6-argument `Instruction.move` or raw
`.setStatus` / `.procedure` / `.conclude` constructors:
- *Misery's Shadow* (`Cards/Trigger.lean:38`):
  `would .any (Primitives.Instruction.move (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller opponent])) (some battlefield) (counted .one graveyard) .departure none [])`
- *Bot Bashing Time* (`Cards/Damage.lean:102`):
  `did .any (Primitives.Instruction.move (which .object) (some battlefield) (counted .one graveyard) .departure (some .destroy) []) .thisTurn`
- *Kalitas, Traitor of Ghet* (`Proofs/DrtWitnesses.lean:10`):
  `would .any (Primitives.Instruction.move (a (Primitives.Predicate.and [nonToken, creature, Primitives.Predicate.hasPossessor .controller opponent])) (some battlefield) (counted .one graveyard) .departure none [])`

## Scope

1. **Keep the Core AST primitives minimal (`Words.lean`, `Events.lean`, `Abilities.lean`):**
   - Do not add redundant verb constructors to `Instruction`; keep
     `Instruction.move`, `.setStatus`, `.procedure`, `.conclude`, `.enact`, etc.
     as the minimal core primitive basis for lowering.
2. **Design and land `Instruction` verb templates:**
   - Introduce surface-faithful `Instruction` templates for zone transitions,
     status changes, procedures, and outcomes (`die`, `enter`, `leave`,
     `putInto`, `phaseOut`, `phaseIn`, `turnFaceUp`, `loseGame`, `winGame`,
     `castSpell`, `activateAbility`, `playLand`, `attack`, `block`,
     `dealDamage`, `combatDamageTo`, etc.).
   - Settle and document the argument convention for subject-performed verbs
     (`die` `[CR#700.4]`, `enter`, `leave`, `phaseOut` `[CR#702.26b]`, `explore`
     `[CR#701.44a]`, `connive` `[CR#701.50a]`) vs. transitive actions (`destroy`
     `[CR#701.8]`, `exile` `[CR#701.13]`, `sacrifice` `[CR#701.21]`, `discard`
     `[CR#701.9]`) across `does`, `would`, `did` (including relative clauses with
     `which`), and `cant`.
   - Express convenience `GameEvent` templates (`dies`, `enters`, `leaves`, etc.)
     in terms of those `Instruction` templates so there is one definition of
     each verb's semantics.
3. **Re-author `Cards/*.lean` and `Proofs/*.lean` event/replacement/lookback sites:**
   - Replace raw `Primitives.Instruction.move`, `.setStatus`, `.procedure`, and
     `.conclude` calls inside `does`, `would`, `did`, and `cant` across
     `lean/Semantics/Cards/` and `lean/Semantics/Proofs/` with the new verb
     templates.

## Proof

- Zero raw 6-argument `Instruction.move` calls remain in `lean/Semantics/Cards/`
  for standard verbs (`die`, `enter`, `leave`, `putInto`).
- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
