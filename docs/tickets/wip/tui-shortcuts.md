---
needs: [tui-decision-actions]
---
The convenience layer over the decision loop: auto-resolve decisions with a single
legal answer (one trigger needs no ordering prompt; a forced choice with one option),
and two virtual passes — "pass until an opponent gets a non-pass action" and "pass
until my next turn" — that submit passes until the condition holds. Honest: they only
auto-submit choices the engine would accept, and stop the moment a real decision
appears. Part of tui-client.
