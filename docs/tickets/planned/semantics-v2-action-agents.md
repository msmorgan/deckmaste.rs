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
generic `move` helper takes a trailing `from` defaulting to `wherever`; the
other helpers pass `wherever`. The checker reads the origin as a zone
expression and has no origin rules yet.

**Not done: dropping discard's agent.** Owner's model: the choice is its own
step with its own actor, and the discard is a pure object move —
`discard(cards)` for determined cards, and `chooseAndDiscard(player, n)` as
"`player` chooses n cards in hand, then those are discarded", with a
distributing player passed directly (`chooseAndDiscard(each(opponent), 1)`):
the players choose in turn order, then the discards happen simultaneously
[CR#101.4]. The player is mentioned once, as the chooser; the hand stays bare.

Checked against the Lean checker (no declaration changed):

- Resolution-time discards work. The three Anaphora pins re-spell as
  choose-by-player then discard the chosen, and give their old results (each
  opponent: plural read-back clean, singular refused with the same `anaphor`
  refusal; you: singular read-back clean). Liliana of the Veil's +1, Syphon
  Mind, the `that player` trigger, Mox Diamond's offer, the draw-then-discard
  read-back and "each player may discard their hand" all check. A random
  discard needs no chooser: two cards at random in target opponent's hand.
- **Discard as a cost does not.** `Instruction.costActionOk` takes one action,
  and `.sequentially [choose, discard]` is refused `costAction`: Diplomatic
  Escort, Korlash's grandeur cost, Sphinx of the Chimes, Vexing Sphinx's
  cumulative upkeep, and every other "Discard a card:" cost on the bench, and
  `retrace`'s "discarding a land card". The smallest change would be to admit,
  in a cost, a choose whose agent is the payer followed by one costed action
  on what was chosen. That relaxes a check rule, so it waits for a decision.
- **The read-back number is fixed in a RON body.** The discard must read the
  chosen cards back with `it` (one chooser, one card) or `them` (several
  choosers or cards), and a RON macro cannot pick one from its arguments.

Open point: a later step that needs each player paired with their own card
("each opponent discards a card, then loses life equal to that card's mana
value") loses that pairing in a flat "those". No canon or bench card needs it
today.

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
