---
needs: []
---
A many-binder `With(Choose(N, …), body)` binds N objects as `Selection::Those`,
but a verb body acts on a single `Reference`. To act on ALL N (e.g. Brainstorm's
"put two cards on top of your library"), the body must be
`Each(over: Those, Move(ThatObject, …))`. Two things make that not yet clean:

1. **Brainstorm currently moves only one of two cards.** `plugins/canon/cards/Brainstorm.ron`
   ships as `With(Choose(Exactly(2), …), Move(That, Library(FromTop(0))))`. `That`
   is the singleton anaphor (`Reference::That` → `frame.those.first()`), so the
   engine moves the FIRST chosen card only. It is gate-green solely because there
   is no engine-resolution test for Brainstorm — the render test passes because
   the `With(Choose(2))` that-phrase ("2 cards from your hand") substitutes into
   `Move(That, …)` to read "put 2 cards … on top." This is a real resolution bug.

2. **The engine-correct shape renders awkwardly.** `With(Choose(2, …),
   Each(over: Those, Move(ThatObject, Library(FromTop(0)))))` resolves correctly
   (moves both), but the renderer's generic `Each` path emits "For each 2 cards
   from your hand, put it on top of your library." A collective-rendering path is
   needed: `Each(over: Those, Move(ThatObject, <dest>))` should read "put <those>
   on <dest>" (mirroring the `Each(group, DealDamage(ThatObject, n))` → "deal N to
   each <group>" collapse already added in `render/effect.rs`). "In any order"
   ordering for top-of-library placement is a further nuance (cf. `Distribute`).

**Fix:** add the renderer collective collapse for `Each(over: Those, <verb>(ThatObject, …))`,
then switch Brainstorm (and any other many-binder group-move) to the
`Each(over: Those, …)` body, and add an engine-resolution test asserting BOTH
cards move. Surfaced during the core-verb-patient-cardinality refactor (the
verb→`Reference` split made group-moves go through `Each`/`Those`).
