---
needs: []
---
**Engine: there is no runtime library-search primitive.**

`Binder::Search` / `Binder::SearchOne` carry the Idris constructors' resolver
inputs, but nothing executes them — `resolve/effect.rs` and `activate.rs` both
surface a labeled seam rather than resolve one.

`parse-tutor-search` (done) built the parser half: it recognizes the
search-reveal-move-shuffle sentence chain and lowers it onto these binders. It
assumed the engine-side primitive existed. It does not, so a tutor parses
correctly and then panics on resolution.

Build the search step per [CR#701.23]:

- Look at all cards in the searched zone, even a hidden one ([CR#701.23a]).
- Fail-to-find: a stated-quality search never compels finding a match that is
  present ([CR#701.23b]); an undefined quality lets the player search but find
  nothing ([CR#701.23c]); a bare quantity must find that many, or as many as
  exist ([CR#701.23d]).
- Reveal only when the effect says to ([CR#701.23e]).
- Shuffle afterwards ([CR#701.24a]), including when nothing was found
  ([CR#701.24d]).
- Bind the result as `That` — singular for `SearchOne`, a group for `Search`,
  matching the existing binder-cardinality split. The `if_none` arm runs on a
  failed find.

Resolving these through the plain `ChooseObjects` chooser is the tempting
shortcut and is wrong: it would silently skip the reveal and the shuffle, so a
tutor would leave the library in a known order.

Out of scope unless a corpus card drives it: simultaneous multi-player search
ordering ([CR#701.23i]) and searching outside the game ([CR#701.23j]).

Effort: **M**.
