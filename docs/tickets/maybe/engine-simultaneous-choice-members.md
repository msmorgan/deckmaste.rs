---
needs: []
design: true
---
**Engine: a `Simultaneously` member whose verb needs an in-resolution choice
panics. No card needs it yet; the design question is real.**

`crates/deckmaste_engine/src/resolve/effect.rs` requires every `Act` member's
`action_items` lowering to yield nothing but `WorkItem::Emit`. Several
`Action` variants lower to something else when the verb needs a decision
before it can become concrete events — `ChooseManaColor`/`ChooseManaMode`,
`FlipCoins { called: true }`, `Retarget`, `ChooseNoteNumber`/
`ChooseNoteCardName` (all in `resolve/player_action.rs`).

**Nothing authored reaches this.** The corpus's only `Simultaneously` users
are the `Fight` and `ExchangeControl` builtin macros, Avarice Totem, and Axis
of Mortality. All are pure verb pairs, except Avarice Totem, which trips
`engine-simultaneous-non-verb-members`' guard first. The real template that
would need this is Equipment/Aura swap ("Exchange this Equipment with an
Equipment card in your hand") — a resolution-time choice of which card. No
such card is authored, not even as a `.ron.todo` stub.

**The design gap.** `Simultaneously`'s doctrine is one snapshot: every
member's items are evaluated up front against the current state, before any
applies. A decision in this engine is asynchronous — surfaced via
`self.pending` and resumed by `submit_decision`, the `ChoiceContinuation`
machinery `engine-resolve-effects` built for `May`/`Unless`/`Modal`. So
admitting a choice-bearing member forces a choice: does the decision resolve
*before* the snapshot, so every member still evaluates against one state with
the outcome already known? Or does the whole `Simultaneously` pause mid-batch
and re-enter with the resolved reference substituted — which sits badly with
one-snapshot, since other members' state could move while the decision is
outstanding?

Do not build ahead of card pressure. This is filed so that whoever hits the
panic finds the question already framed; the answer should be informed by
whatever real card finally forces it.

Effort: **L**.
