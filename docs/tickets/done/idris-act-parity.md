---
needs: []
---
Close the idris-side gaps the keyword-action migration left open. Items 2 and 3
shipped in this feature; item 1 remains a labeled latent gap (no canon card
exercises it yet).

1. **(LATENT) `Composite` draws lose their telescope.** Rust draws emit as
   `Composite (Draw who n) …`, and `actionIntro (Action.Composite _ _) = []`
   (Core.idr) still neither introduces the drawn-card/`amountAnte` antecedents
   nor descends into the body — while the intentionally-kept bespoke
   `Action.Draw` supplies them. A card of the shape "Draw two cards, then
   discard that many" re-emits to idris and FAILS the typecheck; `drawBy`
   (Core.idr) is unreachable from the emitter. Neither canon card triggers this
   (Anje's Ravager draws three with no `ThatMany` back-reference), so it is
   deferred until a canon card pairs a draw with `ThatMany` — at which point,
   either give the `Composite` intro a spec-aware descent or route emitted
   draws back through `drawBy`. (The bespoke `Action.Draw` stays — settled
   Rust/idris draw asymmetry.)

2. **(DONE) The idris `mill` macro now encodes the batch semantics.**
   `Macros.idr`'s `mill n` is `Batch n (Act (Composite (Mill You) (Act
   (MoveArranged (TopOfLibrary (^1)) ChosenOrder (ToZone Graveyard)))))`,
   matching the Rust re-ruling of mill as ONE batch ([CR#701.17a]) — the Rust
   `Mill.ron` builds a `MoveGroup`, `idris_emit` emits `MoveArranged`, and the
   macro body (and its adjacent comment, which used to argue for the superseded
   per-card shape) now agree.

3. **(DONE) `EventFilter::Act` has an idris `EventKind` mapping.**
   `idris_emit.rs`'s `EventFilter::Act` arm lowers to
   `(vec![act_event_kind(verb)?], actor_agent_facets(who, on)?)`, and
   `Core.idr` gained the `EventKind` constructors
   (`Mill`/`Scry`/`Surveil`/`Fateseal`/`Fight` beside `Destroy`). The
   `KeywordActionSpec` type was re-keyed entity-first to match Rust emission
   (Scry/Surveil carry only a `Count`; Mill/Draw only a player; Fateseal a
   player and a count; Discard a player and a count; Destroy/Fight the fighters)
   with unit-test coverage of each spec's argument shape. The old emitter gap
   message is gone.
