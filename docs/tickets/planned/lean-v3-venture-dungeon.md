---
needs: [semantics-v3-peer-files-and-toposort]
---
# Venture into the dungeon, dungeon cards and the initiative's venture triggers

Standard constraints apply.

Declare `ventureIntoTheDungeon` and its variant venture into [quality] [CR#701.49a..701.49d] in
`Definitions/Actions.lean`, then author the initiative's two "ventures into Undercity" triggers on
`initiativeDefinition` (`Definitions/DesignationRules.lean`) [CR#726.2]. Moved here from
`lean-v3-keyword-bodies-and-catalog-sync`, which declared learn and empower Jace.

Design (from ADR `semantics-v3.md` §0.1 items 4 and 5):

- A dungeon card is a card frame of its own: rooms, each a room ability, joined by arrows
  [CR#309.4,309.4c]. Add it to `Card` beside the other frames, with the check its frame needs.
- The venture marker's position is engine state. Venturing reads it through a Condition atom
  (the marker is in the bottommost room, or in a room with a given arrow), never by simulating it
  in the checker [CR#309.5a,309.5b,701.49b,701.49c].
- The dungeon card comes from outside the game: it is described with `outsideTheGame`
  (`Templates.lean`) [CR#701.49a,400.11], and is put into the command zone [CR#309.2b].
- A room ability's trigger is "When you move your venture marker into this room" [CR#309.4c].
  Completing a dungeon is removing it from the game [CR#309.7], and the bottommost-room rule is a
  state-based action [CR#309.6], a rules-table row.

Vintage-legal cards that venture: 38 in the MTGJSON snapshot, the initiative cards among them.
