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
