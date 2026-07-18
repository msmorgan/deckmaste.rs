---
needs: []
---
Durational effects render the "until end of turn" duration clause in the WRONG
position. `crates/deckmaste_cards/src/render/effect.rs`'s duration handling
FRONTS the clause ("Until end of turn, ~ gets …") for any non-literal P/T
delta, but the printed oracle MID-INSERTS it ("~ gets +2/+0 until end of turn
for each …"). This blocks overall fidelity for every one-shot durational pump.

Surfaced by the for-each pump macros (`P1P1ForEach` et al.): Goblin Piledriver
and Might of the Masses now render the pump clause + selection noun correctly
but still FAIL fidelity solely on the duration placement. Pre-existing and
independent of that work — the fix is in the shared duration renderer.

Fix: mid-insert the duration phrase after the verb/delta rather than fronting
it (mirror the literal-delta path, which already places it correctly).

Verify: `cargo xtask fidelity` on Goblin Piledriver / Might of the Masses — the
duration clause sits mid-sentence and the cards round-trip clean; no regression
to effects that legitimately front a duration.
