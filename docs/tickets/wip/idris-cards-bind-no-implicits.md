---
needs: [idris-adopt-base-library]
---
# Card authoring binds no implicits

Ruling (2026-08-21): a card in the authoring bench must never bind an implicit.
No `{ok = …}` proof, no `{param = Just …}` keyword argument — a card should
read more or less like Haskell: constructors and card-language functions
applied to positional arguments. Where a construction has an optional slot, the
card language supplies a **wrapping macro** (a plain function, per
`docs/decisions/macros-are-declarative.md` and the card-language role in
`docs/decisions/kind-index-joins-union-marking-is-spelling.md`) that fills it;
the card calls the macro. Default-valued implicits are not an authoring
surface.

Measured at minting, `idris/src/Experimental/Cards.idr` has 183 explicit
bindings (`grep -cE '\{[a-zA-Z_]+ *= '`), in two classes:

- **Proof bindings the elaborator could not find (~31):**
  `{na = MkNotPlayerSpanning}` ×18, `{tb = MillB {na = …}}` ×9,
  `{ok = ExileB}`, `{riders = MkMoveRiders}` ×3. These are elaboration leaking
  into authoring. `NotPlayerSpanning` was kept as a `data` wrapper in
  `idris-gates-reflect-with-so` "for inference" and still needs binding at
  every site, so the failure predates `So`: the nested `Move … {na} ` inside
  `MillB {na}` says the index is not determined by the term. Fix the
  constructor or its index so auto-search succeeds, or route the obligation
  through a macro that supplies it — never leave it to the card.
- **Default keyword arguments (~150):** `{param = Just …}` 49,
  `{intervening = Just …}` 24, `{from = Just …}` 18, `{verb = Cast}` 15,
  `{dom = Just …}` 13, `{mark = Additional}` 9, and the long tail
  (`window`, `limit`, `what`, `asThough`, `whom`, `under`, `guard`, `alt`,
  `marking`, `followedBy`, `by`, `sub`, `span`, `many`, `counters`, `cause`).
  Each becomes a wrapping macro in `idris/src/Experimental/Macros.idr` (or a
  positional argument where the slot is not optional in practice — judge per
  slot from the bench: a slot bound at most sites is not optional). The
  constructor may keep the implicit for the macro's use, but it must be
  invisible from `Cards.idr`.

Semantics do not change: every card keeps the same constructions; only the
spelling through which the card reaches them changes. Pins and witnesses in the
`Proofs*` files are the evidence that nothing moved.

Write the ruling up as an ADR in `docs/decisions/` when the bench is clean, so
it binds future constructions; link it from the decisions README.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/Events.idr`, `idris/src/Experimental/Macros.idr`,
`idris/src/Experimental/Cards.idr`, `docs/decisions/`. No Rust crate is
touched.

## Acceptance

- `grep -cE '\{[a-zA-Z_]+ *= ' idris/src/Experimental/Cards.idr` is 0.
- `idris/scripts/build` PASS from a clean `build/`, no witness lost, no pin
  silently passing.
- The ADR exists and names the wrapping-macro rule and the proof-binding ban.

Standard constraints apply.
