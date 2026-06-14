---
needs: []
---
Make the full set of legal actions at each decision point enumerable and rich
enough for a UI to present, without the caller re-deriving legality. Partly exists:
`legal_actions(state, player) -> Vec<Action>`, `legal_attackers`/`legal_blockers`/
`legal_targets` (legal.rs), and the `DecisionPoint`/`Decision`/`Action` types
(decide.rs). Audit coverage against everything a player can actually do at priority
— cast each castable hand card with a payable cost, activate each available ability
(including mana abilities), play a land — plus the forced-choice points (targets,
attackers, blockers, modes, trigger ordering), and fill the gaps so each
`DecisionPoint` carries the decider and the complete option set with enough context
(cost, targets, source) for a renderer to show it. Enabler for
`tui-decision-actions`; part of `tui-client`.
