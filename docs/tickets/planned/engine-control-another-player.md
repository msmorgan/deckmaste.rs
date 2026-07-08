---
needs: []
design: true
---
Controlling another player [CR#723] (Mindslaver, Worst Fears, Sorin Markov -7):
"you control target player during that player's next turn" — you make all
choices and decisions that player is allowed to make [CR#723.5], but object
control, resources, and active-player status stay with them [CR#723.3,723.5a].
Needs: a `PlayerAction::ControlPlayer(Reference)` verb (Idris shell exists at
`Core.idr:2093`; no Rust variant yet); per-player `pending_controller` +
next-turn duration consumed in `begin_turn` (overwrite last-wins [CR#723.1a],
dropped if the controller left [CR#800.4b]); and the decision-routing remap —
a `GameState::actual_decider(&pending)` that maps the nominal
`PendingDecision::decider_player()` to the controller while control is live,
honoring the documented `DeciderSpec` nominal→actual contract
(`decision.rs:12`). Concede never redirects [CR#723.6]. Limited-duration control
(Word of Command, Opposition Agent) and all turn-STRUCTURE alteration (extra
turns/skips — see engine-turn-modification) are out of scope. Split out of
engine-win-alterations.
