---
needs: [tui-board-view, engine-decision-enumeration]
---
Turn the board view into a player: at each `DecisionPoint`, present the engine's
enumerated legal actions (cast from hand, activate ability, play land, declare
attackers/blockers, choose targets/modes) as selectable options, build the matching
`Decision` / `Action`, and submit it. Target and option selection reuse the board
selection UI; spacebar passes priority. The honest core of the client — every move
comes from the engine, none hand-rolled. Part of tui-client.
