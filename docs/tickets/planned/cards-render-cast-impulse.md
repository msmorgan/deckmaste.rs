---
needs: []
---
# cards-render-cast-impulse — render arms for the impulse + emblem-trigger phrasing

Chandra, Torch of Defiance (`plugins/canon/cards/Chandra, Torch of Defiance.ron`) is in
canon with a **render-fidelity waiver** (`// waiver:` in the card). Its engine semantics
are fully test-proven (`crates/deckmaste_engine/tests/planeswalker_chandra.rs`); only the
card-text RENDER of two constructs is unbuilt. Build the two arms, drop the waiver.

## The two unbuilt render arms

1. **Exile-top-and-bind composition.** The `+1` impulse encodes as
   `Each(binder: Existing(TopOfLibrary(1)), effect: With(binder: Produce(Move(It, Exile)),
   body: May(...)))`. The renderer emits "For each …/the produced object" instead of
   "Exile the top card of your library. … that card". Needs a render arm that recognizes
   the `Each(Existing(TopOfLibrary(n)))` + `Produce(Move(It, Exile))` shape and phrases it
   as the printed impulse ("Exile the top card…" + a `That(Card)` → "that card" anaphor
   in the nested `Cast`).

2. **Emblem-trigger phrasing.** The `−7` `GetEmblem([Triggered(event: Cast(who: Ref(You)),
   …)])` renders the onset `Cast { what: Any }` awkwardly and the emblem's self-reference
   `This` generically. Should read "Whenever you cast a spell, this emblem deals 5 damage
   to any target" — i.e. `Cast{what: Any}` → "a spell", and an emblem-context `This` →
   "this emblem".

Both are bidirectional-grammar work in `crates/deckmaste_cards/src/render/effect.rs`
(and wherever the emblem/trigger onset renders). The `Cast` verb itself already renders.

## Acceptance

- Chandra renders faithfully (matches oracle text closely enough to pass the fidelity
  gate); the `// waiver:` block is removed from the card.
- The cards conformance/fidelity suite passes with the waiver gone.
- No engine/semantic changes — render-only. `render-template-first` / bidirectional
  parse⇄render truth preserved.
