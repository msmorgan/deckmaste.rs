---
needs: []
---
# Randomness tail: roll replacements' body, Planechase events, per-object flips

Routed from `workbench-randomness-residues` (close, 2026-08-26), which landed
the instruction-side ignores (`IgnoreRolls`/`ShiftResult`), `RollPlanarDie`,
result-conditioned headers, noted results, roll-set reads, and `CoinCameUp`.
Five measured remainders:

1. **The replacement side of roll modifiers/ignores** — `Intercepts
   (RollsDice …)` is writable but its body is not: `eventIntro` announces
   neither die count nor kind, so "instead roll that many dice plus one"
   has no antecedent and `RollDice`'s `sides : Nat` has no anaphoric arm.
   Never pinnable [CR#706.2,706.6].
2. **Planechase events + the "chaos ensues" instruction** — the planar die's
   faces get their meaning from [CR#901.9a..901.9c] as EVENTS (needing
   `EventName` rows); "chaos ensues" is a printed instruction [CR#311.7]
   (2 supported lines).
3. **"a die's highest natural result" test** (per [CR#706.2]'s natural
   result) — one measured line.
4. **"your third die each turn"** — untried; possibly composes from
   `NthOccurrence` + `RollsDice` already; probe before minting.
5. **The per-object coin flip** — "flip a coin for each creature" (Rakdos,
   the Showstopper): `FlipCoins` takes a player subject only.

## Consumption boundary

`idris/src/Experimental/Events.idr`, `Triggers.idr`, `Phrase.idr`,
`Effect.idr`; `Cards.idr` bench; `Proofs*.idr`. No Rust crate. The parent
fence carries: watch/read vocabulary only, no randomness engine.

## Acceptance

- Each item lands or ends in a written rule-backed verdict; no silent drops.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
