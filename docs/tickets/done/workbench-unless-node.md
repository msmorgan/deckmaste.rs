---
needs: []
---
# An `Unless` node: pay-to-escape, forward-bound

Ruling 2026-08-22. "Counter target spell unless its controller pays {3}"
(Mana Leak; Rhystic Study, Daze, Propaganda-style lines) is unrepresentable:
the effect introduces the mention and the unless-clause reads it back ("its
controller"), and no node carries that. `May` cannot: it binds the offer before
the effect (the prenex lift), and reordering its fields breaks every other
`May`. Mint

```idris
Unless : (e : Effect bs) -> (who : Noun (preIntro e) Player) ->
         (c : Cost (nomIntro who)) -> Effect bs
```

— effect first, payer and cost read `preIntro e`, the `OnlyIf` topology. The
pay arm only: "unless [condition]" is `OnlyIf e (NotCond c)`, which the
de Morgan fix now admits — do not mint a condition arm. Readers (`effIntro`,
`annIntro`, spans) gain a clause; the rules meaning is [CR#118.12a] (the
player may pay; if not, the effect happens). Bench Mana Leak and Rhystic
Study; delete `badUnlessAnaphoricPayer`.

## Consumption boundary
`idris/src/Experimental.idr`, `Words.idr`, `Macros.idr`, `Cards.idr`,
`Proofs*.idr`.

## Acceptance
`Unless` exists with the stated signature; Mana Leak benches; build PASS; no
implicits in cards; cites 0/0. Standard constraints apply.

## As landed (2026-08-22)

`Unless` is minted in the `Effect` mutual block (`Experimental.idr`) with the
pinned explicit signature plus the two `So`-gates `Pay` already carries —
`{auto 0 pb : Payable c}` and `{auto 0 ag : PayAgrees who c}` — since the
unless-arm is a cost paid on resolution [CR#118.12]. No condition arm.

**CR delta.** [CR#118.12a] reads "[Do something] unless [a player does
something else]", broader than the ticket's "pays": the CR arm is any action,
which [CR#118.12] then calls a cost. The node carries the cost arm only, and
the constructor docstring says so.

**Readers extended** (nine; found by building — the whole `Effect` match set):
`heldUntilOk` (False), `reflexEncloseUse` (`EncNotOneAction`), `costActionOk`
(False — an offer another player answers at resolution instructs nothing at
payment), `effEq` (False), `effIntro` (`bs` — the deed is conditional),
`preIntro`/`annIntro` (`annIntro e`), `replacedCtx` (`replacedCtx e`),
`deedDelta` (`[]`). Each mirrors the `If` clause beside it; `condDelta` is
uniformly `[]`, so the payment mention adds no binding.

**Witnesses** (`Experimental/Cards.idr`): `manaLeak` — "Counter target spell
unless its controller pays {3}", the payer `ControllerOf It` reading the
target the effect just named; `rhysticStudy` — "Whenever an opponent casts a
spell, you may draw a card unless that player pays {1}", `Unless` over an
offered `May` body with `That PlayerW` reaching the trigger's caster.

**Pin.** `badUnlessAnaphoricPayer` deleted (its sentence now typechecks —
probed). Replaced by `badUnlessTapSymbol` in `ProofsC.idr`: "Counter target
spell unless its controller taps" — [CR#107.5] makes {T} a cost only before a
colon. Probe: `Can't find an implementation for So False`.

**Not needed.** No `unlessPays` macro: no card site binds an implicit
(`Experimental/Cards.idr` greps 0), because the gates are `auto` and
`Macros.counterSpell` already carries `CounterSpell`'s zone implicit.

**Gates.** Clean 18/18 exit 0; cards implicit grep 0; `cite check
--list-noncompliant` empty, 0 stale of 15982 (no `bless` — [CR#118.12a,107.5]
already locked); `cite audit --diff` read on all 6 sites.
