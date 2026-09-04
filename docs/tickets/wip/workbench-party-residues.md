---
needs: []
---
**Close the two gaps the party round left in its witnesses.** Residue of
`workbench-party-and-outlaw` (2026-09-04):

- **Crime.** At Knifepoint is benched as its first ability only; its second
  ("Whenever you commit a crime, …") needs a game event for committing a crime
  [CR#700.13] — a player casting a spell, activating an ability, or putting a
  triggered ability on the stack that targets an opponent or their permanent,
  spell, ability, or graveyard card. Add the event to `Triggers` as one
  positional row (RON-shaped), bench At Knifepoint's second ability and one
  more vintage-legal crime card, pin a CR-meaningless target (a crime with no
  opponent-owned object).
- **"That weren't chosen this way."** Stick Together's closing line is
  spelled as a `ForEachOf (each AnyPlayer)` loop holding the four
  `Choose (UpTo 1)` roles and `sacrifice They theRest`, because a
  distributive deed cannot host a group spend (`EnactKeepsOuter`) and no
  predicate reads "not chosen this way". Decide whether an exclusion
  predicate over a prior choice (`NotChosenBy`-style, counted-uniqueness
  gated) is a real vocabulary gap or whether the loop reading is the settled
  shape for "each player … then each player sacrifices … that weren't
  chosen"; if the former, add it and re-spell the witness back to the printed
  distributive shape.

Size: S–M. Done when: both witnesses read their printed card in full; pins
probed non-vacuous; build at its module count. Standard constraints apply,
including the RON-shaped constraint.
