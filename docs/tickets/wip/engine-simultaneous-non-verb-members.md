---
needs: []
design: true
---
**Engine: a `Simultaneously` member must be an `Act` verb, and a supported
canon card needs a `Continuously` member today.**

`crates/deckmaste_engine/src/resolve/effect.rs`'s `OneShotEffect::Simultaneously`
arm rejects any member that isn't `OneShotEffect::Act(_)`. Note that
`Act(Action::Composite { .. })` still passes — "verb" here means "wrapped in
`Act`", not "primitive". What it excludes is the structural kinds:
`Continuously`, `Sequentially`, `If`, `May`, a nested `Simultaneously`.

**This is reachable, not hypothetical.** `plugins/canon/cards/Avarice Totem.ron`
("{5}: Exchange control of this artifact and target nonland permanent.",
`supported: true` in the derived corpus) has exactly that shape — a
`Simultaneously` of two mirrored `Continuously(EndOfGame, Modify(_,
SetController(ControllerOf(_))))` halves. Activating it panics. Only render
and fidelity tests touch the card, so CI is quiet.

The card's own comment records that the `Continuously` spelling was chosen
deliberately over the working `ExchangeControl` builtin macro, because that
macro expands to the one-shot `Action::GainControl`, which `idris_emit`
declines — the continuous-modification form is the only one with a real Idris
emit path. So the engine gap was known and accepted at authoring time.

**Widening the match arm is not the fix.** The batch pipeline assumes every
member reduces to `GameEvent`s via `action_items`, merged into one
`Occurrence::Batch` ([CR#608.2f]). A `Continuously` member never produces an
event at all — it mints a static row directly ([CR#611.2c] fixes its affected
set at that moment), so it has nothing to contribute to the batch and nothing
to test for emptiness under the all-or-nothing rule ([CR#701.12a]: if the
entire exchange can't be completed, no part of it occurs). Design how a
non-event, state-mutating member participates in that doctrine — most likely
compute its scope and legality read-only alongside the verb members'
`action_items`, and mint the row only once no member has come up empty.

Start narrow: admit `Continuously` members, the only shape a real card needs.
Leave `Sequentially`/`If`/nested `Simultaneously` to a follow-up if a card
ever forces them.

Done when Avarice Totem's ability resolves as one simultaneous all-or-nothing
control swap ([CR#701.12b]), a semantic engine test pins it including the
same-controller no-op that [CR#701.12b] requires, and any remaining non-`Act`
shape is still loud.

Effort: **M**. Design input needed on the read-only-then-commit split.

## Done

`OneShotEffect::Simultaneously` now splits into a read-only phase A
(classify + evaluate every member against the pre-state) and a mutating
phase B (timestamp allocation, `self.continuous.push`, event commit), gated
by a per-kind void test: an `Act` member is empty when it yields no events;
a `Continuously(Modify(r, _))` member is empty when `r`'s resolved `Locked`
scope is empty. Only `Act` and `Continuously(Modify(_, _))` members are
admitted; `Sequentially`/`If`/`May`/nested `Simultaneously`, and any other
`Continuously` body (`Floating` scope included), stay loud `todo!` seams —
reassigned to `engine-simultaneous-member-breadth`, minted here as their
scoping owner, since this ticket closes without covering them.

Beyond the ticket's stated scope: `layer.rs`'s `resolve_new_controller` only
resolves `SetController(You)` at apply time (no `Frame` there to chase a
live `Target`/`ControllerOf` read), so Avarice Totem's
`SetController(ControllerOf(Target(0)))` would have silently no-op'd even
once the panic was gone. Fixed by resolving the `SetController` value at mint time instead — the
game read it needs happens once, when the effect applies ([CR#608.2h]) —
via `eval_player_ref`, rewriting the stored change to the `You` shape
`resolve_new_controller` already handles, with the resolved player captured
as the minted `ContinuousEffect`'s `controller`.

Tests (`crates/deckmaste_engine/src/resolve/effect.rs`):
`avarice_totem_swaps_control_through_two_continuously_members` (cross-seat
swap, both halves reading the pre-exchange controllers) and
`avarice_totem_same_controller_exchange_is_a_no_op` ([CR#701.12b] — two
inert same-value rows, control observably unchanged).
