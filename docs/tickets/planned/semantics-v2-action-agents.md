---
needs: [plugins-v2-keyword-bodies-over-helpers]
---
**An action takes an agent exactly when the outcome depends on who or what
performs it.** Owner's direction, 2026-10-04 ("No more implicit actors";
`scry(you, 1)`). Design-bearing and term-changing; standard constraints apply.
Working direction, not a ruling.

## The principle

- The agent is written explicitly and comes first. No macro defaults it.
- It is a player where the rules need one: `scry` reads "your library"
  [CR#701.22a]; `create` has no performer in its definition [CR#701.7a] but
  "the player who creates a token is its owner" [CR#111.2], so it takes one.
- It is a permanent where a permanent performs the action: spells and
  abilities "instruct a permanent to explore" [CR#701.44a].
- It is absent where the object alone determines everything: "To discard a
  card, move it from its owner's hand to that player's graveyard"
  [CR#701.9a]. The same holds for returning a card to its owner's hand. `may`
  might need none either.
- Where it is absent, the TERM drops the agent too. A macro that hard-codes
  `You` inside is the implicit agent this ticket removes.

## Constraints

- "Whenever you discard" must keep working: with no agent on the move, the
  discarding player is the player whose hand the card left. No event macro or
  canon card writes a discard trigger today, so add the derivation, and a
  canon card that proves it, before removing the agent.
- Decide each action against the whole CR, not only its own definition;
  `create` is the worked example of why. Sacrifice is a movement whose
  definition does name a performer, "its controller moves it" [CR#701.21a];
  decide whether that is an agent or derivable from the permanent.

## Starting evidence

A first-pass table (one defining sentence per action, 2026-10-04) classed the
65 declared actions as player, permanent, none or unclear. Its "none" and
"unclear" rows are unreliable for the reason above and must be re-derived.
It also found declarations whose `Enact` wrapper carries no agent while the
body names one inside (`bolster`, `populate`, `timeTravel`, `explore`,
`endure`, `recruit`), and two whose permanent is an implicit "this" (`adapt`,
`monstrosity`).

## Why after the sweep

Once bodies are written over helpers, this change is a signature change on a
helper and a one-line edit at each caller, visible in
`cargo xtask expansions` as exactly the terms that moved.
