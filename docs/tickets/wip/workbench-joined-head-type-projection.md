---
needs: []
---
# Project a joined noun's head type per half

The join landed as a general constructor and left three gates reading a joined
noun's head type through machinery that only ever saw one. All three residues
were found in review, none was in its round's scope, and all three want the same
missing piece: a per-half head-type projection over nouns.

## 1. `seedTy (Joined …)` prefers the left half and discards the right

From `docs/tickets/done/workbench-attackable-defender-join.md`:

> `Attackable` reads `nounTy`, and `seedTy (Joined l r)` prefers the left half
> and discards the right, so `attackableKind (a \/ b) t` re-checks the *same*
> `t` for both halves.

At `Object \/ Player` — every kind `Macros.kindJoin`, `Macros.anyTarget` and
`Macros.thatJoin` can build — that is exactly right, because the `Player` arm is
unconditional and the `Object` arm carries the whole check. A hand-written
`Joined` at `Object \/ Object` slips through: `Macros.a (Joined (HasType
Planeswalker) (HasType Creature))` is admitted as an attack defender while the
reversed order and `Joined AnyPlayer (HasType Creature)` are both refused. That
round's own diagnosis: "Closing it needs a per-half head-type projection over
nouns (`headTys` has no `Joined` case either) — a construction outside this
ticket."

## 2. A same-kind join bypasses `DamageableTy`

From `docs/tickets/done/workbench-union-family-macros.md`:

> `Joined` is general over two kinds, so `Joined (HasType Land) (HasType Land)`
> is a `Predicate bs (Object \/ Object)`, and `JoinTakes` admits it as a damage
> recipient with no zone and no type check — where the object-only phrase `And
> [creature, InZone graveyardZ]` is still refused by `ObjectTakes`.

Recorded there as the price of the general constructor rather than a decision
taken. It is the same hole seen from the `DamageRecipient` site: `JoinTakes`
asks nothing of either half. The attack gate is strictly stronger than the one
it mirrors, so fixing `seedTy`/`headTys` fixes the weaker site too — check that
it does rather than assuming it.

## 3. The kind-polymorphic slots the `AnyTargetFree` sweep opened

Same ledger: `CountOf`, `Aggregate`, `Exists`, `Each`, `Indefinite`, `Definite`,
`CountedGroup` and `AllOf` are kind-polymorphic, so each now admits a joined
predicate — `CountOf Macros.anyTarget` ("the number of any target")
type-checks. Recorded as tolerated overgeneration. Re-read it once the
projection exists: some of these may become refusable on the same evidence, and
the ones that do not should be named at their zero rather than left unmentioned.

## Not this ticket's

`lookbackComplementOk`'s refusal of every joined complement is routed to
[workbench-event-zone-and-cast-provenance](workbench-event-zone-and-cast-provenance.md),
which owns that table.

## Consumption boundary

`idris/src/Experimental.idr` (`seedTy`, `headTys`, `nounTy`, `Attackable` /
`attackableKind`, `DamageRecipient` / `JoinTakes` / `ObjectTakes`, the
kind-polymorphic phrase heads), `idris/src/Experimental/Words.idr` if the
projection needs a word of its own, the pin modules
`idris/src/Experimental/Proofs*.idr`, and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- A joined noun projects a head type per half, and `Attackable` checks each half
  against its own; the three admitted-by-accident terms above are re-probed and
  the outcome recorded for each.
- `JoinTakes` is re-read against the projection: either it checks its object
  half or its vacuity is argued from a rule.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
