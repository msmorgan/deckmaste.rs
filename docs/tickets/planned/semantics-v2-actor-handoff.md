---
needs: [plugins-v2-keyword-bodies-over-helpers]
---
**Instructions have an implicit performer, the actor, and another player
performs an instruction only through a handoff, `act(player, instruction)`.**
Design agreed with the owner on 2026-10-05; this ticket is landing (1) of
six. Lean first, then the Rust mirror, the RON macros, a few canon cards,
the ADR ruling and the glossary. Standard constraints apply.

Owner, 2026-10-05: "Perhaps instructions have an implicit performer, which can
be handed off (rebinding "you" to another player) with a helper like
`act(playerRef, instruction)`? ... Maybe the explicit actors version is more a
concern for lowering/core." This reverses the 2026-10-04 direction (agent
written explicitly and first, `createToken(you, …)`), which this file used to
plan as `semantics-v2-action-agents`.

## What is decided

- **Spelling.** No instruction writes who performs it: `draw(2)`, `scry(1)`,
  `may(body)`, `choose(a(…))`, `createToken(creatureToken(…))`,
  `sacrifice(a(creature))`, `loseLife(2)`, `mill(3)`. Another player acts
  through `act(player, instruction)`, and one `act` may wrap a
  `sequentially([...])`. The handoff is the only place a performer is
  written. (The helpers reach this spelling in landing (2),
  `plugins-v2-implicit-actor-spelling`.)
- **`you` is always the controller** of the spell or ability [CR#109.5],
  everywhere, keyword action bodies included. Owner: "I don't like
  exceptions, let's be consistent." Inside a handoff the other player is
  `they` / `theirHand` (ordinary anaphora) or `actor`.
- **`actor`** is a noun phrase meaning "whoever is performing this
  instruction": the controller unless an enclosing `act` says otherwise.
  Keyword action bodies use it where they said `you` (`actorControls` beside
  `youControl`, `handOf(actor)`). `act(you, …)` is legal and needed, for
  example inside another player's scope: "each opponent may pay {2}; if they
  don't, you draw a card".
- **A distributed handoff pairs each player with their own card.**
  `act(each(opponent), body)` means each opponent performs the whole body.
  Burglar Rat becomes `act(each(opponent), discard(a(card)))`. That the
  discards happen at the same time [CR#101.4] is a lowering concern, not
  written.
- **The verb supplies its own restriction.** A discard is from the actor's
  hand [CR#701.9a] and a sacrifice is of something the actor controls
  [CR#701.21a], so neither is written: `discard(a(card))`,
  `discard(aRandom(card))`, `sacrifice(a(creature))`. This is the existing
  rule in `docs/decisions/semantics-v2.md` §7 ("A verb's implicit restrictions
  must be expressible both ways — as predicate conjuncts on a choice…"); the
  checker may need to learn it. `choose` still names a zone when it needs one
  (Thoughtseize: `choose(a(and([not(land), cardIn(handOf(they))])))`).
- **Every named action records its actor, discard included.** Owner: "yeah i
  guess this is right, albeit a tad redundant". In landing (2) keyword action
  declarations lose `agent:` and their performer parameters, and the loader
  writes `Enact(Action(label), body, actor)` for every action with a wrapper.
  `amass` gains its wrapper in this landing (below), so "whenever you amass"
  and "the Army you amassed" [CR#701.47b,701.47c] have something to name.
- **Cast permissions follow the same rule.** `mayCastFrom(what, zone,
  paying)` permits the actor; airbend's "its owner may cast it" is
  `act(ownerOf(it), establish(mayCastFrom(it, exileZone, mana([2]))))`. The
  helper itself is `plugins-v2-keyword-helper-additions`; this ticket's
  prototype must confirm that `actor` reads as the controller inside a static
  ability.
- **The handoff is a model node, not an expander feature.** `macro_ron` has no
  ambient or inherited values (arguments expand frameless; a default sees only
  its own call's arguments), and threading text would paste
  `target(opponent)` into every step, making two targets.
  `docs/decisions/macros-are-declarative.md` puts behaviour that cannot be a
  template in the model.
- **The model change is minimal.** Add (a) a handoff instruction and (b) the
  `actor` noun phrase. The 16 required `agent` fields (`conclude`,
  `separateIntoPiles`, `vote`, `copy`, `changeLife`, `addMana`, `draw`,
  `expose`, `search`, `shuffle`, `flipCoins`, `rollDice`, `rerollStored`,
  `createObject`, `pay`, `skipPart`), the 3 optional ones (`choose`,
  `enact`, `insertPart`) and `ContinuationPolicy.optional`'s agent all STAY;
  the helpers fill them with `actor`. Deleting them is
  `semantics-v2-drop-agent-fields`. The Lean bench (`lean/Semantics/Cards`,
  453 explicit `agent := Primitives.NounPhrase.you`) must keep proving
  untouched in this landing.
- **Order of landings** (owner: "sure"): (1) this ticket; (2)
  `plugins-v2-implicit-actor-spelling`; (3) `semantics-v2-drop-agent-fields`;
  (4) the keyword defects (`plugins-v2-keyword-body-defects` and the tickets
  split from it); (5) `semantics-v2-gets-both-deltas`; (6)
  `semantics-v2-counter-kind-is-a-name`.

### Target bodies the owner approved ("lgtm")

These are the end state after landing (2):

```ron
// Thoughtseize
sequentially([
    act(target(anyPlayer), revealHand),
    choose(a(and([not(land), cardIn(handOf(they))]))),
    act(they, discard(that(Card))),
    loseLife(2),
])
// Burglar Rat
act(each(opponent), discard(a(card)))
// amass
sequentially([
    doIf(not(exists(and([army, actorControls]))), createToken(creatureToken(0, 0, [Black], [Param(0), army]))),
    choose(a(and([army, actorControls]))),
    putCounters(Param(1), p1p1Counter, that(Creature)),
    doIf(not(matches(it, hasSubtype(Param(0)))), establish(addSubtype(it, Param(0)))),
])
// Azog, Moria's Ruin: "Its controller amasses Goblins X"
act(controllerOf(it), amass(goblin, x))
```

`revealHand` and `theirHand` do not exist yet (today:
`revealTheirHand(agent)`, `handOf(they)`); add them here.

## Why no per-action table decides who performs

Movement actions need a performer too, so the 2026-10-04 plan to class each
action as taking an agent or not is withdrawn. Hylda of the Icy Crown:
"Whenever you tap an untapped creature an opponent controls". Shared Fate: a
player exiles cards from an opponent's library, then looks at "cards they
exiled". Cosi's Trickster: "Whenever an opponent shuffles their library".
Other players do perform keyword actions: Azog, Moria's Ruin (amass), six
cards investigate, five scry, two blight. `adapt` and `monstrosity` are only
ever a card's own and keep their implicit `thisPermanent`;
`explore(permanent)` keeps its permanent parameter.

## What exists to build on (Lean)

`enact v e agent` and `withContinuation (.optional agent)` already check their
body under `agentIntro bs agent`: `lean/Semantics/Check/Phrase.lean`
(`NounPhrase.agentIntroduced` ~L1881, `agentIntro`, `agentCtx`),
`Check/AbilityRules.lean` (`Instruction.check`), `Check/Abilities.lean`
(`Instruction.profile`, `mayCtx`, `enactKeepsOuter`, `eachStackOk`). `.you`
is a closed constant: it introduces no binding, so `they` can never refer to
it; `NounPhrase.isYou` is read by cost rules (`Check/Abilities.lean` ~L805).
`agentRef` (`Macros.lean` ~L732) is how a macro re-reads its own agent. Pins
that touch this: `Proofs/Choice.lean` `okAgentScopedChoice`,
`badUnchooseredTheyControl`, `okDistributedChoiceReadsAsGroup`,
`badDistributedChoiceReadSingular`.

## The work, in order

1. **Lean prototype.** Add the handoff instruction and the `actor` noun phrase
   in `lean/Semantics/Abilities.lean` / `Phrase.lean`, checked like `enact`'s
   body under `agentIntro`. Before fixing the shape, answer and record in the
   landing record:
   - Does `actor` read as the controller inside a static ability (airbend's
     `establish(mayCastFrom(…))` under `act(ownerOf(it), …)`)?
   - Does `discard(a(card))` with no zone check? If not, what is the smallest
     rule that makes it check (the verb's own restriction, §7 of the ADR)?
   - Can the checker require a discard's actor to own the hand the card
     leaves?
   - What bindings remain after an `act`: may a later step say `they` for the
     player handed to (Thoughtseize's `choose(… handOf(they) …)` after
     `act(target(anyPlayer), revealHand)`), and does `act(each(opponent), …)`
     leave a group?
2. **Lean pins.** One pin per answer above, plus Thoughtseize, Burglar Rat,
   Hymn to Tourach and `act(controllerOf(it), amass(goblin, x))` checked
   through the handoff, and a refusal pin for each refused shape the prototype
   finds. The four pins listed above keep their asserted outcomes.
3. **Rust mirror.** The same two constructors in `crates/deckmaste_semantics_v2`;
   `tests/lean_drift.rs` must hold.
4. **RON macros.** `act` (instruction), `actor` (noun phrase), `actorControls`
   (predicate), plus `revealHand` and `theirHand`.
5. **`amass` gains its wrapper.** `keyword_actions/amass.ron` drops
   `deed: None`, so the loader wraps it as `Action("Amass")`, and writes the
   approved body with `actor` in its agent slots. Fix its comment to the
   rule's wording: "Put N +1/+1 counters on that creature" [CR#701.47a], not
   "on it". (The comment fix is also listed in
   `plugins-v2-keyword-helper-additions`; whichever lands first does it.)
6. **Canon proof.** Re-spell Thoughtseize, Burglar Rat and Hymn to Tourach over
   `act`. Expected Hymn body, derived from the rule above and not separately
   approved: `act(target(anyPlayer), discard(random(exactly(2), card)))`.
   *Orchestrator's call, not the owner's:* in this landing the helpers still
   take their agent parameters, so these cards may pass `you`/`actor`
   explicitly where a helper still demands it (`choose(actor, …)`,
   `loseLife(2, you)`); the approved bodies above are reached in landing (2).
   Pulling those helpers' signature changes forward is allowed if it keeps
   the rest of the tree unchanged.
7. **ADR ruling.** In `docs/decisions/semantics-v2.md` §7 (L161-170), add a
   ruling dated 2026-10-05 (the owner agreed to a dated ruling so agents can
   tell which source is newer). It supersedes "Agents are explicit; core
   constructors take every field required. The imperative's unpronounced
   subject is supplied by the frame as an explicit `You` in the term." and
   the word "require" in "Agentive verbs … put that performer in clause
   position and require it … a dependent context cannot re-use the subject
   term at each inner slot, so the slot rides the clause and lowering
   redistributes it." The performer is implicit (the actor) and written only
   by a handoff. Constructors still take every field (until
   `semantics-v2-drop-agent-fields`), so the ruling keeps "core constructors
   take every field required" and says that
   `macros-are-declarative.md` and `card-authoring-binds-no-implicits.md` are
   not contradicted (no expander feature is added).
8. **Glossary.** Add **Actor** and the handoff to
   `docs/contexts/game-model/CONTEXT.md`, with CR citations, through the
   `domain-modeling` skill. The glossary today defines neither "Actor",
   "Performer" nor "Handoff", and no `_Avoid_` line names them; "agent"
   appears only in **Choice** and **Decision Point**, where it means the
   external party an engine Decision Point waits on, not the performer of an
   instruction. The new entries must keep the two apart (an `_Avoid_` line on
   Actor for "agent" in the performer sense is the likely shape).

## Proof this landing gives

- `cargo xtask lean-check` passes, with the new pins and every existing pin at
  its old outcome; the bench under `lean/Semantics/Cards` is untouched.
- `lean_drift` holds.
- `cargo xtask expansions` before/after differs only in `amass`, the three
  re-spelled canon cards and the new macros; list each changed term in the
  landing record.
- The prototype's four answers are recorded, each with its pin.

## Done on 2026-10-04, still standing

- `Move(subject, from, to, riders)` has a required origin, in Lean and the
  mirror. Origin-agnostic text is the explicit zone expression `wherever`
  (Lean `ZoneExpr.wherever`, mirror `ZoneExpr::Wherever`, RON
  `zones/wherever`): exile is "move it to the exile zone from wherever it is"
  [CR#701.13a]. The generic helper is `move(subject, from, to)`.
- `cardIn(zone)` is how RON writes "a card in <zone>"; `yourHand` names your
  hand and `handOf(…)` another player's.
- Random selection: `aRandom`, `random(quantity, what)` and `selectRandom`.

## Superseded

- **Agent-first explicit spelling** (`createToken(you, …)`, `choose(you, …)`,
  `may(you, …)`, "No macro defaults it") is withdrawn. Writing the performer at
  every instruction is what the handoff replaces; the owner's words are quoted
  at the top.
- **The per-action agent table** (an action takes an agent "exactly when the
  outcome depends on who or what performs it"; the first-pass table classing
  the 65 actions as player, permanent, none or unclear) is withdrawn. The
  evidence under "Why no per-action table" shows movement actions need a
  performer as well, so every named action records its actor.
- **"Discard has no agent"** is partly superseded: `discard(cards)` keeps its
  single parameter, but its wrapper records the actor like every other action.
- **The distributed `choose` then `discard(them)` spelling** (Burglar Rat's
  current body) is replaced by `act(each(opponent), discard(a(card)))`, and
  with it the unchecked convention that a bare hand under a distributed
  `choose` means each chooser's own hand.
- **`choose` with no chooser** (`amass`, `proliferate` pass `None`) is
  answered: the chooser is the actor.

## Old open points, under the new model

- **"Whenever <player> discards".** No longer needs a derivation from the hand
  the card left: the discard's wrapper records its actor. The proof is Megrim,
  "Whenever an opponent discards a card, this enchantment deals 2 damage to
  that player.", which joins canon in `plugins-v2-keyword-helper-additions`
  with the `discards(player, card)` event helper. ("Whenever you discard a
  card" is already pinned: `Proofs/Anaphora.lean` `okDiscardFromHand`.)
- **Player–card pairing** ("each opponent discards a card, then loses life
  equal to that card's mana value") is solved by the per-player body:
  `act(each(opponent), sequentially([...]))` keeps each player with their own
  card.
- **Causative wording** ("have <player> <verb>") stays a realization concern
  outside this ticket.

## Out of scope

- Re-spelling every helper, keyword action and canon card
  (`plugins-v2-implicit-actor-spelling`).
- Deleting the agent fields and re-spelling the Lean bench
  (`semantics-v2-drop-agent-fields`).
- `mayCastFrom` and the airbend re-spelling
  (`plugins-v2-keyword-helper-additions`).
- How lowering redistributes a handoff to the performing players.
