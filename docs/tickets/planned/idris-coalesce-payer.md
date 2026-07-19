---
needs: []
---
Teach the Idris soundness core to derive a coalesced payer from an `AnyTarget` slot, so
"deals damage to any target unless that player pays" cards typecheck. The
`Reference::Coalesce` fixture (`plugins/testing/cards/Rhystic-toll DealDamage.ron`) models
`MustPay(actor: Coalesce([ControllerOf(Target(0)), Target(0)]), …, or_else: DealDamage(This,
3, Target(0)))` — the resolution-time toll [CR#118.12a] names a SINGLE payer derived from an
any-target slot [CR#115.4] two ways: a permanent's controller [CR#109.4], or — when the
target IS a player — that player directly. At runtime this is provably an `APlayer` (proven
both branches in `resolve::query::tests`), but Idris cannot derive it: an `AnyTarget` slot
folds to the top-of-lattice `Anything` kind, and `resolveTarget` has no implementation that
yields `Bound APlayer` from an `Anything`-kind coalescence. `card_RhystictollDealDamage` is
the sole remaining `idris-check` card failure after the any-target authoring fixes (commit
`trwwwpvu`); the tool still exits 0 (individual card failures are non-fatal), so this does not
block, but the card does not typecheck.

Add a coalescing/payer primitive to the core (`idris/src/`) that types
`Coalesce([ControllerOf(anyTarget), anyTarget])` as `Reference b APlayer` — e.g. a
`payerOf : Reference b Anything -> Reference b APlayer` derivation, or a `Coalesce` typing
rule that resolves to the join of its branches' kinds. Additive to the soundness core (no
surface change; the Rust/RON shape already exists and renders/executes correctly). Surfaced
by (not caused by) `core-copy-grammar` — it is a pre-existing any-target modeling gap that the
copy work's idris parity pass merely brought into focus.
