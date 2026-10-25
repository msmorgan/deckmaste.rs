---
needs: []
---
# Companion's pregame reveal as a start-of-game rule

Standard constraints apply.

"Before the game begins, you may reveal one card you own from outside the game with a companion
ability whose condition is fulfilled by your starting deck" [CR#702.139a,103.2b]. This is a
start-of-game rule, a rules-table row (`docs/decisions/semantics-v3.md` §0.1 item 4), not part
of the keyword's in-game body. `companion`'s body writes the in-game special action
[CR#116.2g] (`Definitions/Keywords.lean`). It does not yet restrict it to the card revealed
this way.

Write the row: the reveal, its condition read through the keyword's `DeckCondition` against the
starting deck after sideboard cards are set aside [CR#702.139b], and the mark that the card was
revealed this way. Then gate the special action on that mark. The start-of-game procedure's order
[CR#103] is the engine's.
