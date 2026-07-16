---
needs: []
design: true
---
Parse the reveal-hand + choose family: `Target opponent reveals their hand.
You choose a nonland card from it. That player discards that card.` and the
kin (`… you choose a card from it and exile that card`, `Target player
reveals their hand and you choose a card of that color from it …`). The
reveal primitive exists (`PlayerAction::Reveal`,
`crates/deckmaste_core/src/action.rs`); the missing piece is the
choose-from-the-revealed-set binder chain — a selection whose domain is
another player's just-revealed hand, carried across sentences to the
discard/exile action.

**Why design-gated**: the cross-sentence carry (revealed set → `from it` →
`that card`) needs the same binding story as `parse-cross-sentence-anaphora`,
plus a selection-domain form for "the revealed cards" — decide whether the
whole chain folds into one effect production (the sentences are formulaic)
or rides the general anaphora machinery once that lands.

**~113 of 17,022 one-away cards** (2026-07-16 tally); classic discard-tutor
staples (Thoughtseize-alikes).

Verify: `cargo xtask generate plugins/wizards` graduation delta;
`cargo xtask idris-check` on a graduated reveal-choose card.
