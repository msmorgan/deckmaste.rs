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

## `Move` needs a `from`

The `Move` instruction carried only a destination; only the `ZoneChange` event
had an origin. So the discard term said nothing about a hand, and the agent was
the one thing tying it to a player. Owner's direction: "discard is specifically
move from hand to gy" and "it's intrinsically tied to a player because the card
must move from a hand".

**Done.** `Move` now has a required origin, `Move(subject, from, to, riders)`
in Lean and the mirror. Origin-agnostic text is an explicit zone expression,
`wherever` (Lean `ZoneExpr.wherever`, mirror `ZoneExpr::Wherever`, RON macro
`zones/wherever`), not an absent field: exile is "move it to the exile zone
from wherever it is" [CR#701.13a]. Each move states its origin — the action's
own where its rule gives one (discard from its owner's hand [CR#701.9a],
destroy and sacrifice from the battlefield [CR#701.8a,701.21a], counter from
the stack [CR#701.6a], mill from the library [CR#701.17a]), else the zone the
card or keyword text names ("from your graveyard"), else `wherever`. The
generic helper is `move(subject, from, to)`, origin required and second;
`exileBy` and `putOntoBattlefieldTappedAttacking` take a trailing `from`
defaulting to `wherever`, and the other helpers pass `wherever`. The checker
reads the origin as a zone expression and has no origin rules yet.

**Done: discard has no agent.** `discard(cards)` takes one parameter, the
cards; its body is the move from hand to graveyard, and the discarding player
is whoever owns that hand. `keyword_actions/discard.ron`, the Lean `discard`
macro, every RON caller (Dangerous Wager, Farm // Market, `recruit`, and the
raw Discard deeds of cycling, reinforce, retrace, transmute and madness) and
the Lean bench and pins are re-spelled; every pin keeps its asserted outcome.

The principle: an inline indefinite implies its own selection, and who makes
it is not written in the term — lowering derives it: the payer for a cost
[CR#118.1], the affected player for a discard [CR#701.9b]. Exceptions are
written: "at random" is marked on the indefinite (Hymn to Tourach, Stormbind),
and a different chooser is an explicit `choose` step because the card prints
it as its own sentence (Thoughtseize). So "Discard a card" as a cost is one
action, `discard(a(card in hand))`, and the refusal of a sequenced cost
(`badSequentialCost`) stands. Where the old agent was what named whose hand
("that player discards a card"), the hand now names the player once
(`handOf(…)`). `choose` names its chooser first (`choose(agent, subject)`), and
`selectAtRandom(quantity, what)` is a selection no player makes.

Still open:

- **An inline distributed form.** "Each opponent discards a card" has no inline
  agentless spelling that the checker reads as one card per player. Until it
  does, distribution is spelled as the distributed `choose` followed by one
  discard of what was chosen (`choose(each(opponent), a(card in hand))`, then
  `discard(them)`): the players choose in turn order, then the discards happen
  together [CR#101.4]. Burglar Rat proves it end to end.
- **The bare hand.** Under a distributed `choose`, and in an inline indefinite,
  a bare hand means the selector's own hand by convention; nothing checks it.
- **`choose` with no chooser.** `amass` and `proliferate` pass `None`; who
  chooses there is this ticket's question.
- **Player–card pairing.** A later step that needs each player paired with their
  own card ("each opponent discards a card, then loses life equal to that
  card's mana value") loses that pairing in a flat "those". No canon or bench
  card needs it today.
- **"Whenever you discard".** The constraint below still holds: the
  derivation of the discarding player from the hand the card left is not yet
  written.
- **Causative wording.** The card data's "have <player> <verb>" sentences all
  sit under "may" or a payment, and the term already names both players.
  Rendering that wording (derived from the enclosing subject, or an explicit
  wrapper) is a realization concern outside this ticket.

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
