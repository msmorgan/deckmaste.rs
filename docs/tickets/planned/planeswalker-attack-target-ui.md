---
needs: [engine-planeswalkers]
---
The engine already threads an attack target per attacker (`Decision::Attackers` carries
`(attacker, target)` pairs; `PendingDecision::DeclareAttackers` surfaces `legal_targets` =
the defending player's proxy plus each planeswalker they control). Two consumer-side
residuals remain, both deferred so the core landed without a UI blow-up:

1. Rich attack-target selection. The AI strategies and the TUI declare-attackers flow
   currently always target the defending player's proxy (behavior preserved from before
   planeswalkers existed). Give the AI a policy for when to attack a planeswalker instead,
   and give the TUI a picker so a human can choose each attacker's target from
   `legal_targets`.

2. `Must(Attack)` retarget. The attack requirement's `on`-match still tests only the sole
   defender proxy (latent site around `decide.rs`), so a "must attack if able" creature is
   not yet forced toward a specific chosen planeswalker target. Generalize the match to the
   pair's chosen target once selection (item 1) can produce non-proxy targets.

Neither is reachable-wrong today (consumers only ever pass the player proxy), so this is an
enhancement, not a correctness fix.
