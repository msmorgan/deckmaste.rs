---
needs: []
---
# Clash in the Lean v3 workbench

Standard constraints apply. Implemented when the first clash card is spelled in
`lean/Semantics/Cards/`; 28 cards in `data/derived/cards.jsonl` say "clash with an opponent".

Today `clash` (`lean/Semantics/Definitions/Actions.lean`) is only [CR#701.30a]'s reveal and
optional bottom: it has no opponent and no winner [CR#701.30b..701.30d]
(`lean-v3-rules-and-card-fidelity` §2, superseded here).

**Ruling (owner, 2026-10-25): option (b), the coin-flip precedent.**

- The clash action carries a win/lose outcome, as `flipCoins` carries its `FlipCall`
  (`Words.lean`). "If you win" is read back as `did you (clash … (some .wins)) thisWay`, as
  a coin flip's is read as `did you (flipCoins 1 (some .wins)) thisWay` (`Cards/Faces.lean`). No `wonClash` Condition is minted.
- A player wins a clash if they revealed a card with a higher mana value than all other cards
  revealed in it [CR#701.30d].
- "Clash with an opponent" [CR#701.30b], "Choose an opponent. You and that opponent each clash,"
  is one Definition. Its body keeps [CR#701.30c]: the clashing players reveal at the same time,
  decide in APNAP order [CR#101.4] where to put their cards, and the cards move at the same time.
  A per-player `instruct (eachOf …) clash` loop would serialize the reveals, against
  [CR#701.30c], so the body is not written that way.
- "Whenever you clash" triggers for each clashing player, so each player's clash is a clash
  event carrying the `clash` deed.
