---
needs: []
design: true
---
Restart the game [CR#727] (Karn Liberated): a terminal-with-carryover that
immediately ends the current game with no winner/loser/draw and rebuilds in
place — every card involved comes along (ownership unchanged [CR#727.2]), the
restarting effect's controller is the new starting player [CR#727.1a], and the
remainder of the resolving effect runs just before the first untap [CR#727.4].
Needs new grammar: an exemption selector ("leaving in exile all non-Aura
permanent cards exiled with ~") plus exiled-with-~ object linkage the engine
doesn't track. The `PlayerAction::RestartGame` verb and its `resolve.rs:1851`
`todo!` seam already exist; Idris has no `RestartGame` (add `RestartGame :
Outcome b` to close the emit gap). Split out of engine-win-alterations.
