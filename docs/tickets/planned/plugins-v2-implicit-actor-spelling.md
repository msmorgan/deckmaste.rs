---
needs: [semantics-v2-actor-handoff]
---
**Every RON helper, keyword action and canon card is spelled with the
implicit actor.** Landing (2) of the actor-handoff design agreed with the
owner on 2026-10-05; the decisions are recorded in
`semantics-v2-actor-handoff`, which must be done first (it adds `act`,
`actor` and `actorControls`). Standard constraints apply.

## The change

1. **Instruction helpers lose their agent parameter.** Every helper under
   `plugins_v2/builtin/macros/` that takes an `agent` (or a `who`/performer
   parameter that fills an agent field) drops it and writes `actor` in that
   slot: `draw(2)`, `scry(1)`, `may(body)`, `choose(a(…))`,
   `createToken(creatureToken(…))`, `sacrifice(a(creature))`, `loseLife(2)`,
   `mill(3)`, `revealHand`. The model's agent fields stay (they are deleted in
   `semantics-v2-drop-agent-fields`); only the RON spelling changes.
2. **Keyword action declarations lose `agent:` and their performer
   parameters.** The loader (`crates/deckmaste_semantics_v2/src/keywords.rs`,
   `keyword_action_body`) writes `Enact(Action(label), body, actor)` for every
   action that has a wrapper, discard included (owner: "yeah i guess this is
   right, albeit a tad redundant"). The meta-macro
   `macros/meta/KeywordAction.ron` drops its `agent` parameter. Bodies say
   `actor` where they said `you` (`actorControls` beside `youControl`,
   `handOf(actor)`); `you` keeps meaning the controller [CR#109.5].
   `adapt` and `monstrosity` keep their implicit `thisPermanent`;
   `explore(permanent)` keeps its permanent parameter.
3. **`exileBy` and the agent on `returnToHand` retire into the plain forms.**
   The eight files that call `exileBy` (ingest, scavenge, myriad, embalm,
   eternalize, recover, unearth, forage) write `exile(…)`, under `act` where
   another player exiles; `returnToHand` loses its `agent` parameter.
4. **Every keyword body and canon card is re-spelled**, reaching the owner's
   approved bodies for Thoughtseize, Burglar Rat and `amass`
   (`semantics-v2-actor-handoff`, "Target bodies"). A card where another
   player performs writes `act(player, …)`.
5. **Lean macros** that mirror these helpers (`lean/Semantics/Macros.lean`)
   follow where a RON helper is ported from one; the Lean bench itself is not
   re-spelled here.

## Left by `semantics-v2-actor-handoff` (2026-10-05)

- **Cards already on the actor.** Thoughtseize, Burglar Rat and Hymn to
  Tourach are spelled over `act`, and the helpers still taking an agent
  there already hold `actor` (`choose(actor, …)`, `loseLife(2, actor)` in
  Thoughtseize). `amass` writes `agent: actor` and `actor` in its
  `createToken` and `choose` slots. These need only their agent arguments
  removed. `theirHand`, `revealHand`, `act`, `actor` and `actorControls`
  exist.
- **The discard wrapper gains the actor and the actor's hand.**
  `keyword_actions/discard.ron` still has no agent and a bare hand
  (`move(Param(0), hand, graveyard)`, so the loader writes
  `Enact(Action("Discard"), …, None)`), and its comment still says "the
  deed takes no agent", which the 2026-10-05 ruling withdrew. The Lean
  `Actor.discard` writes the actor in both places:
  `.enact (.action "Discard") (.move subject (.zone .hand (.possessedBy
  .actor)) graveyard []) (agent := some .actor)` [CR#701.9a]. This landing
  makes the RON declaration match it.
- **`card` or `isCard`: a naming question for the owner.** The approved
  bodies write `a(card)`; no `card` macro exists, and the cards write
  `a(isCard)`, the existing identity alias of `Predicate.IsCard`. §12.1 of
  `docs/decisions/semantics-v2.md` allows one alias per constructor, so
  `card` would be a rename of `isCard` or a phrasing beside it. Ask before
  re-spelling; do not add both silently.
- **`army` is a subtype, not a predicate.** The approved `amass` body's
  `and([army, actorControls])` is written `and([hasSubtype(army), creature,
  actorControls])` (Lean `Actor.army`). A predicate named `army` beside
  the subtype declaration is possible (`creature` is both a type and a
  predicate declaration) but is a new phrasing, so it is the owner's call.
- **ADR sentences that become false here.** In
  `docs/decisions/semantics-v2.md` §11, "a keyword action's instruction and
  its `agent`" (the declarations lose `agent:`); re-check "a file writes
  `keyword_params` or `deed: None` where its definition does not take the
  derived arguments or deed" against the five keyword actions that still
  write `deed: None` (`create`, `reveal`, `search`, `shuffle`, `vote`) and
  `macros/meta/KeywordAction.ron`. §12.1's alias example `draw(amount: …,
  agent: …)` describes the constructor alias, which keeps its `agent` until
  `semantics-v2-drop-agent-fields`; it stays true here. The `Actor` bucket
  of §12.1's Lean-only list shrinks as RON helpers take these spellings;
  recount it with the method stated there. `lean/README.md` ("required
  player slots default to `.you`") is about the explicit-agent Lean macros
  and stays true while they exist.

## Proof (orchestrator's call)

`cargo xtask expansions` before and after cannot be byte-identical, because
`You` becomes the actor in agent slots. So:

- For every declaration and card that contains no handoff, the expansion after
  must equal the expansion before once the actor is mapped back to `You`.
  The mapping is a one-off comparison run from gitignored scratch; it is
  plan-scoped verifier code and does not enter `crates/`.
- Cards with a handoff are proven by `cargo xtask lean-check` alone.
- List in the landing record every declaration or card whose expansion differs
  after the mapping, with the reason; an unexplained difference is a STOP.
- Every helper parameter removed or reordered is listed by helper.

## Out of scope

- Deleting the agent fields from the model (`semantics-v2-drop-agent-fields`).
- New helpers (`plugins-v2-keyword-helper-additions`) and the gift variants
  (`plugins-v2-gift-variants`), which are written in this spelling after it
  lands.
