---
needs: []
---
**Fight is performed by a creature: the first creature is the performer
and the second is the parameter.** "A spell or ability may instruct a
creature to fight another creature … Each of those creatures deals damage
equal to its power to the other creature" [CR#701.14a]. Routed from
`builtin-v2-keyword-spelling-templates-residue`, owner decision D2
(2026-10-07): the subject is never a slot, and the performer is `actor`,
never a parameter. Small. Standard constraints apply.

## Why it stopped

The residue landing tried fight as `params: [Subject]` with an `actor`
body, and Prey Upon as
`act(target(creatureYouControl), fight(target(creatureYouDontControl)))`.
`cargo xtask lean check` gave canon 133/134. Prey Upon was refused with
`[kindMismatch player object, enactAgentOk]`. The deed table row
`("fight", {})` in `lean/Semantics/Check/Words.lean` names no performer.
The semantics-v2 ADR §7, amendment 2026-10-06, records this as a ruling:
"A deed whose facts row names no performer ("fight" [CR#701.14a]) records
none, and is refused when a handoff names one."

## The work

1. In `lean/Semantics/Check/Words.lean`, `("fight", {})` becomes
   `("fight", { agentRole := permanentAgent })`. Explore, endure, adapt,
   harness and monstrosity already have that row.
2. In the semantics-v2 ADR §7, amendment 2026-10-06, the sentence that
   names "fight" as the no-performer example gets another example, or is
   reworded.
3. `plugins_v2/builtin/macros/keyword_actions/fight.ron` becomes
   `params: [Subject]`. Its body is
   `dealDamage(actor, powerOf(actor), Param(0))` and
   `dealDamage(Param(0), powerOf(Param(0)), actor)`.
4. Prey Upon and every other fight caller are re-spelled to the handoff
   shape, `act(<first>, fight(<second>))`. Today Prey Upon is the only
   caller.
5. Fight's spelling becomes `fight <Param(0)>`. Empty `PENDING_RULING` in
   `crates/deckmaste_construction_core/tests/builtin_v2_keyword_spelling_templates.rs`
   so that the test asserts all 258 keyword declarations have templates,
   and update the fight pin.

## Proof

- `cargo xtask lean check`: canon 134/134, testing 7/7.
- `cargo xtask expansions` before and after, `diff -r`: only fight and its
  callers differ.
- `cargo test -p` passes for deckmaste_construction_core,
  deckmaste_semantics_v2 and xtask.
- `cargo xtask lexical --card-name "Black Lotus" --export`: byte-identical.
