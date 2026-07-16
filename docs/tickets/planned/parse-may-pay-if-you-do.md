---
needs: []
---
Parse the reflexive-optional clause family: `you may <action>. If you do,
<effect>` (and the `If you don't, <effect>` complement), standalone or as a
trigger body — `When ~ enters, you may pay {2}. If you do, draw a card.`,
`you may discard a card. If you do, draw two cards.` The engine side is done
(`core-may-pay-must-pay`): the may/must decision and its follow-up branches
exist; what's missing is the parser production in
`crates/deckmaste_migrations/src/parsers/effect.rs` that folds the two
sentences into one optional-with-consequence effect (the "if you do" sentence
binds to the offer, it is NOT an independent clause — today each sentence fails
alone and together).

Shapes to cover, by frequency: `you may pay {COST}. If you do, …` (the
dominant form), `you may <verb-phrase>. If you do, …`, the negative
`If you don't, …` tail, and both branches present.

**~648 of 17,022 one-away cards** (2026-07-16 tally over
`plugins/wizards/cards/*.ron.todo` — cards whose ONLY remaining `Unparsed`
line matches this class). Largest single parse-only family in the corpus.

Verify: `cargo xtask generate plugins/wizards` graduation delta; spot-check a
may-pay ETB card and a may-discard card render back verbatim
(`cargo xtask fidelity`).
