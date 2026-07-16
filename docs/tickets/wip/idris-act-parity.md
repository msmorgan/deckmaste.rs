---
needs: []
---
Close the idris-side gaps the keyword-action migration left open (the Rust
side shipped; the soundness gate lags it in three places):

1. **`Composite` draws lose their telescope.** Rust draws emit as
   `Composite (Draw who n) …`, and `actionIntro (Action.Composite _ _) = []`
   (Core.idr) neither introduces the drawn-card/`amountAnte` antecedents nor
   descends into the body — while the (intentionally kept) bespoke
   `Action.Draw` supplies them. A card of the real shape "Draw two cards,
   then discard that many" re-emits to idris that FAILS the typecheck; the
   pre-migration `drawBy` emission accepted it, and `drawBy` (Core.idr) is
   now unreachable from the emitter. Latent only until a canon card pairs a
   draw with `ThatMany`. Either give the `Composite` intro a spec-aware
   descent or route emitted draws back through `drawBy`. (The bespoke
   `Action.Draw` itself stays — settled Rust/idris draw asymmetry.)

2. **The idris `mill` macro still encodes the retired per-card semantics.**
   Macros.idr's `mill` body is `Each (Existing (TopOfLibrary n)) (Act (Move
   It (ToZone Graveyard)))`, but the migration re-ruled mill as ONE batch
   ([CR#701.17a]: "mills as one group" is the exact distinction from draw's
   per-card [CR#121.2]); the Rust `Mill.ron` builds a `MoveGroup` and
   idris_emit emits `MoveArranged`. The soundness model's canonical mill
   desugaring contradicts both — align the macro body (and the adjacent
   comment arguing for the superseded per-card shape) with the batch form.

3. **`EventFilter::Act` has no idris `EventKind` mapping and nobody owns
   it.** The emitter gap message (idris_emit.rs) defers to
   "engine-keyword-action-intent Stage 2+", which shipped and is in `done/`
   without it. Every "whenever you mill/scry / ~ is destroyed" trigger card
   is un-emittable to the gate until the mapping lands. This ticket owns it
   now; fix the gap message to point here.
