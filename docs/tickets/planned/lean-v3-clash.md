---
needs: [semantics-v3-peer-files-and-toposort]
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
- "Clash with an opponent" is one Definition, [CR#701.30b]'s composition: "Choose an opponent. You
  and that opponent each clash," plus the win/lose outcome read back with `did … thisWay`
  [CR#701.30d].
- The simultaneous reveal, the APNAP decisions and the simultaneous move [CR#701.30c] are the
  engine's procedure (owner update 2026-10-25; `docs/decisions/semantics-v3.md` §0.1 item 4:
  timing and order are the engine's). The body does not encode them.
- "Whenever you clash" triggers for each clashing player, so each player's clash is a clash
  event carrying the `clash` deed.
