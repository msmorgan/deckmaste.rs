---
needs: [gen-token-effect]
---
Build the demo's two decks from freshly-converted cards — a forcing function for
coverage and an honest demo, since every card graduates through the pipeline rather
than being special-cased — plus the already-canon Lightning Bolt in the red deck and
basic lands. Curate a coherent Goblins deck (tribal lord, token makers incl. a
Krenko-style scaler, a sac-outlet, keyworded bodies, burn) and Elves deck (tribal
lord, mana dorks, ETB / scaling value, a token maker, keyworded bodies); graduate
each per `docs/card-data.md`. Method: pick the decklists, run them through
`graduate`, and let the declines name the remaining gaps — predictable ones are
`gen-token-effect` (hard dep), `gen-dynamic-count`, `gen-sacrifice-cost`; file
further parser-gap sub-tickets as found. Goal: two ~15-card decks exercising
statics/layers, tokens, mana & activated abilities, triggers/stack, targeting, and
keyworded combat. Part of tui-client.
