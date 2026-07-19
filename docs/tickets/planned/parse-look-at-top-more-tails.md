---
needs: []
---
Follow-up wave to `parse-look-at-top` (done), which covered the exile-top move
and the type-gated reveal-to-hand tail. **~99 single-card "Look at the top card
of your library. `<tail>`" cards remain unparsed**, with tails beyond the two
already shipped:

- "you may put it onto the battlefield" (`Move(It, Battlefield)`)
- "you may put it into your graveyard" without a Surveil keyword (rare — the
  keyword covers most)
- "you may cast it" / "you may play it this turn" (cast/play-from-top —
  **blocked by `engine-cast-from-zones`**; leave these until it lands)
- untyped "you may reveal it and put it into your hand" (no "if it's a … card"
  gate), and other single-card look-then-optional-move shapes.

Approach: the settled whole-shape macro pattern (see the
`parse-multi-sentence-whole-shape-macro` memory / the shipped `ExileTop.ron` +
`LookAtTopRevealToHand.ron`) — ONE `OneShotEffect`-kind `.ron` macro per tail
under `plugins/builtin/macros/effect/`, template spanning both sentences, body
`Each(Existing(TopOfLibrary(count: 1)), <tail>)`; NO standalone "look at the top
card" head production (it would let `parse_sequence` preempt the whole-shape
match). Only build a macro a real corpus card exercises (no dead grammar).

Not in scope: the peek-N-and-keep-some shapes ("put one into your hand and **the
rest** on the bottom") — those need the complement primitive
(`balance-choose-complement-player-selection`), not a tail macro. The
cast/play-from-top tails wait on `engine-cast-from-zones`.

Verify: enumerate the remaining tails from `plugins/wizards/cards/*.ron.todo`
(grep "Look at the top card of your library."); add the parser-only tail macros;
`cargo xtask generate plugins/wizards` graduation delta; `cargo xtask fidelity
plugins/wizards` round-trips the new shapes clean. Standard constraints apply.
