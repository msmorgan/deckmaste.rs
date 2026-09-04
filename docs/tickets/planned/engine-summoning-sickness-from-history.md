---
needs: [engine-combatant-role, engine-history-tallies-cache]
---
Drop the tracked `GameObject.summoning_sick` bool and derive summoning sickness
purely from history, once the history-tally cache makes that cheap. After
`engine-combatant-role`, the bool has exactly one data consumer — the
Combatant role's conditional `Cant` conferral, which reads it via
`StatePredicate::SummoningSick` (plus a TUI display read). That predicate is the
seam this ticket replaces: swap its evaluation from a field read to a history
query, then delete the field and its four maintenance sites (`step.rs` set on
ETB / mint / control-change, clear at controller-turn-start).

**The equivalent history query.** A Combatant is sick iff it "entered its
controller's battlefield this turn" — i.e. the most recent event that put it
under its *current* controller happened during that controller's current turn.
Both events are already recorded facts: ETB is `ZoneChange(→Battlefield)`,
control-gain is `ControlChanged{of, to}`. So the derived form is roughly
`Happened(Or([ZoneChange(what: Ref(This), to: Battlefield),
ControlChanged(of: Ref(This), to: <controller>)]), within: <controller-turn>)`.

**Two blockers, both real:**

1. **The window must be controller-relative, not `ThisTurn`.** `Lookback::ThisTurn`
   is the *active* player's turn; sickness clears at the *controller's* turn start
   (`step.rs` clears only when `controller == active`). They diverge for a creature
   controlled by the non-active player (flash/Threaten on an opponent's turn): it
   stays sick until its controller's next turn begins, but `ThisTurn` would report
   it non-sick on the intervening turn. Needs a "since the controller's most recent
   turn began" window — the `SinceYour(PhaseStep)` family is controller-relative but
   is built for trigger watchers and has no clean "turn began" step marker; verify /
   extend it to bind correctly in a per-object layer-gather `Matches(This, …)`
   context.

2. **Perf — hence the `engine-history-tallies-cache` dependency.** The conferral
   condition is evaluated per creature per layer pass (fixpoint, ≤16 passes). Two
   history scans per creature per pass in that hot path is unacceptable without the
   O(1) history-tally cache. Do not claim this before that cache lands (and, per the
   project rule, before the engine is functionally complete — optimizations last).

Net effect: one fewer piece of per-object mutable state, sickness expressed as
data over existing history primitives — the minimal-primitives end state. Purely
a simplification; no behavior change (the derived query must be exactly
equivalent to the bool it replaces, including the control-change window).
