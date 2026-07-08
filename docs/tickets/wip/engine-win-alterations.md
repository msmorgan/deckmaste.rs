---
needs: []
design: true
---
Can't-lose/can't-win statics (OutcomeGate) and the alternate win/loss effect
verbs. Wire the `todo!("P0.W6")` seams: OutcomeGate suppression at the SBA
sweep (`sba.rs:32`, precedence-not-consumption per ADR U5) and the
`WinGame`/`LoseGame` verbs (`resolve.rs:1845,1848`) — win executes as a new
`PlayerWon` event, not derived opponent-losses. Author Platinum Angel and
Abyssal Persecutor as validation cards. Restart-the-game and controlling
another player's turn were split out to `engine-restart-game` and
`engine-control-another-player`.
