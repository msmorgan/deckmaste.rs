---
needs: [plugins-v2-implicit-actor-spelling, semantics-v2-group-handoff-publishes-no-outcome, semantics-v2-deed-performer-roles]
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

Two tickets come first. `semantics-v2-group-handoff-publishes-no-outcome`:
until a group handoff publishes the total "that much" reads, extort must
write `changeLife` with each opponent as its agent, so that field cannot be
deleted. `semantics-v2-deed-performer-roles`: until the twelve deeds the
checker gives no player performer accept the actor, their declarations write
`agent: None` and the loader keeps the `agent` field.

## Left by plugins-v2-implicit-actor-spelling (2026-10-05)

After that landing, these are the only places in `plugins_v2/` that still
pass a performer (or an explicit "no performer") to an instruction helper,
an alias or a keyword action. Line numbers are as of that landing.

- **The `changeLife` alias keeps its `agent` parameter, for extort alone.**
  `plugins_v2/builtin/macros/keyword_abilities/extort.ron:20` writes
  `changeLife(down(1), each(opponent))`: handed off as
  `act(each(opponent), loseLife(1))`, the following `gainLife(thatMuch)`
  ("the total life lost this way") is refused, Lean
  `quantOutcomeInScope 0` (canon card Syndic of Tithes). Every other life
  change goes through `loseLife`/`gainLife`, under `act` for another player.
- **The keyword-action loader accepts `agent: None`** (and refuses any other
  value) for the twelve deeds whose `actFacts` row gives no player agent, so
  Lean `enactAgentOk` refuses a recorded actor on them:
  `keyword_actions/adapt.ron:14`, `bolster.ron:18`, `counter.ron:14`,
  `detain.ron:16`, `endure.ron:14`, `explore.ron:19`, `harness.ron:16`,
  `heal.ron:14`, `meld.ron:15`, `monstrosity.ron:17`, `populate.ron:17`,
  `timeTravel.ron:19` (all under `plugins_v2/builtin/macros/`). The meta
  `macros/meta/KeywordAction.ron` and `deckmaste_semantics_v2::keywords`
  carry the field for them.
- **The optional-agent helpers keep their parameter** (`enact`,
  `insertPart`, `returnTo`, `returnToBattlefield`, `meldInto`,
  `exileWithCounters`; Lean `Option NounPhrase := none`). Calls that pass
  one:
  - `plugins_v2/builtin/macros/keyword_abilities/unearth.ron:16`:
    `returnToBattlefield(subject: this, agent: actor, from: graveyardOf(you))`.
  - `plugins_v2/builtin/macros/keyword_abilities/station.ron:14` (agent at
    line 20): `enact(Action("Tap"), setStatus(…), you)`.
  - `plugins_v2/canon/cards/Glimpse of Freedom.ron:18` (agent at line 21):
    `enact(verb: Action("Exile"), …, agent: actor)`.
  - `plugins_v2/canon/cards/Incarnation Technique.ron:17` (agent at line
    20): `enact(verb: Core(Return), …, agent: actor)`.
  - `agent: None` written out: `Krosan Grip.ron:17`, `Capsize.ron:18`,
    `Dead Revels.ron:21`, `Grim Harvest.ron:17`,
    `Sublime Exhalation.ron:17` (all `plugins_v2/canon/cards/`, on `enact`).
- **Bodies that write `actor` in a constructor's agent slot** (these follow
  the fields when they go): `instructions/createToken.ron:11` and the other
  helpers and aliases whose body says `actor`; `keyword_actions/search.ron`,
  `shuffle.ron`, `vote.ron` (raw `Search`, `Shuffle`, `Vote`).

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
