---
needs: []
---
# Every construction takes all its parameters as required positionals

## Ruling (2026-08-22)

Default slots (`{default … name}`) on core constructors are banned — not for
authoring convenience, which macros supply, but for **translation**: a
realised card macro tree must translate into an Idris term positionally,
term-for-term, so a card can be verified later without a permutation or
defaulting step in between. 41 `{default …}` slots exist in
`idris/src/Experimental.idr` (e.g. `MayPlay`'s `window`, `Move`'s `riders`,
`KeywordAbility`'s `param`, the trigger header's four, `LosesCounters`'s
`amt`). Each becomes an explicit argument; the common case is a wrapping
macro. `docs/decisions/card-authoring-binds-no-implicits.md` is amended to
state the contract (done with this ticket's minting).

`{auto 0 …}` proof gates are not slots and stay.

## Consumption boundary

`idris/src/Experimental*.idr`, `Macros.idr`, `Cards.idr` (must still bind
no implicits), `Proofs*.idr`. Mechanical; Sonnet-grade once the slot
inventory is listed.

## Acceptance

- `grep -c '{default' idris/src/Experimental*.idr` → 0; `Cards.idr` binds
  0 implicits; every former default has a macro supplying it.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
