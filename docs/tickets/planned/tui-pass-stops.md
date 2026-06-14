---
needs: [tui-shortcuts]
---
Configurable pass stops, like real clients. Today's "pass" shortcut wakes on a fixed
set of conditions (mandatory decision / non-empty stack / combat / your main phase).
Real clients (MTGO F-key stops, Arena) let a player pick which boundaries auto-pass
should stop at — opponent's upkeep, beginning of combat, each end step, before
declaring attackers, etc. — and ignore priority everywhere else. This generalizes the
per-player pass state from a hard-coded stop set into a selectable set of stop points,
with a small UI to toggle them. Part of tui-client.
