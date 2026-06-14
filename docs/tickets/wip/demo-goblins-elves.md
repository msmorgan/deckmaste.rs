---
needs: [gen-token-effect]
---
Build the demo's two decks so their cards are GENERATED from MTGJSON through the
extract→graduate pipeline and stay gitignored (like `plugins/wizards`) — never
canonized. Only the decklists are committed. The point is the impressive one: a
fresh clone holds `plugins/demo/decks/{goblins,elves}.txt`, basic lands, and the
handful of already-canon cards the lists reuse (e.g. Lightning Bolt) — and almost
nothing else — yet a full Goblins-vs-Elves game runs, every other card materialized
locally from bulk data, mysteriously absent from the repository. It is also a forcing
function for coverage and an honest demo: each card graduates through the pipeline
rather than being special-cased.

Because the cards come from gitignored generated output, the demo shares the repo's
normal prerequisites — local MTGJSON (`scripts/fetch_data`) plus a generate step,
same as `plugins/wizards` and the engine tests. Worth a line in the README "see it
work" steps so an outside cloner knows the demo's cards are produced locally, not
shipped.

Open design point — where the generated demo cards land and how `build_game()` finds
them: either resolve the decklists straight against the full gitignored
`plugins/wizards` corpus, or add a decklist-scoped `generate` target that emits just
the demo's ~30 cards into a gitignored `plugins/demo/cards/`. Lean toward the scoped
target (lighter than the ~31k-file corpus, and "the demo generates its own cards" is
the cleaner story). Then point `build_game()` at [canon (Bolt/basics), builtin, the
generated demo cards] instead of canon + builtin alone.

Curate a coherent Goblins deck (tribal lord, token makers incl. a Krenko-style
scaler, a sac-outlet, keyworded bodies, burn) and Elves deck (tribal lord, mana
dorks, ETB / scaling value, a token maker, keyworded bodies). Method: pick the
decklists, generate their cards from MTGJSON, and let the pipeline declines name the
remaining parser gaps — predictable ones are `gen-token-effect` (hard dep),
`gen-dynamic-count`, `gen-sacrifice-cost`; file further sub-tickets as found. Goal:
two ~15-card decks exercising statics/layers, tokens, mana & activated abilities,
triggers/stack, targeting, and keyworded combat. Part of tui-client.
