---
needs: []
---
Add a `Count` for the deciding seat's spendable (floated) mana, so mana-aware
strategy decisions become expressible.

`strategy-greedy-port` isolated this as the one missing primitive. Greedy's
competitive play is a careful mana-ramp gated on
`floated_mana + untapped_lands >= cost`, and a `CastSpell` only enters `legal`
once mana is **already floated** (`spendable_pool` is floated-only, `cast.rs`).
But the RON strategy language has **no `Count` for floated pool mana**, so the
ramp reachability gate can't be expressed. Empirically the gap is decisive, not
"within noise": a naive tap-out RON strategy wins **97%** vs the Rust greedy's
**49%** over 300 seeds — exact greedy equivalence is impossible without this.

Add a `Count` source for the seat's spendable pool total (read off
`GameState::spendable_pool` / the player's pool units), wired into `eval_count`
(evaluated against the strategy `eval_frame`, `You` = the seat). A new `Count`
variant or `QueryKey` — `StatOf` reads object characteristics, but this is a
player/pool quantity, so it is a new source, not a `StatOf` stat.

Then a ramp rule like `Activate when Compare(ManaAvailable, Less, <cost>)` taps
exactly until a spell is castable — matching greedy's careful ramp and enabling
realistic mana-aware sequencing for any deck strategy. Mana-awareness is
fundamental to MTG play, so this generalizes well beyond the greedy port. With
it, `strategy-greedy-port`'s original winrate-equivalence assertion (replaced
there by a functional tap-out matchup) becomes achievable as a follow-up.
