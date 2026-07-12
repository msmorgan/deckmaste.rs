---
needs: [engine-randomness]
---
Replacement interception for the randomness events. `StaticEffect::ReplaceRoll`
(the Krark's Thumb family — "flip/roll that many plus N, then ignore …") is
authorable and idris-checks but is deliberately not gathered by the collector;
now that `FlipCoins`/`RollDice` execute (emitting `CoinFlipped`/`DieRolled`
batches through the cant→replace→apply pipeline), there is a live event for it
to intercept. Scope: gather `ReplaceRoll` in the collector; route the flip/roll
batches through the replacement registry; surface the ignore selection as a
decision (an ignored roll never happened — no triggers, no effects
[CR#706.6]; ties for lowest are the player's choice); effects that FIX a
result or a winner [CR#705.3]; the die-roll modifier pipeline
([CR#706.2a..706.2b]) making `natural` ≠ `result` real.
