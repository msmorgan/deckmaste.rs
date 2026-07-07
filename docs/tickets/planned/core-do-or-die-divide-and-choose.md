---
needs: []
---
**Encode Do or Die via the typed two-pile primitive; delete the last stringly
binder.** Do or Die is the ONLY canon card still identifying piles by string
label, and it is one of the idris-check emit gaps — one ticket closes both.

## The stringly binder

`plugins/canon/cards/Do or Die.ron` today threads string labels from the
split to the choice:

```
SeparatePiles(
  group: SelectAll(AllOf([Type(Creature), ControlledBy(Ref(It))])),
  into: ["a", "b"],                 // ← string pile labels
  then: ChoosePile(
    from: Labels(["a", "b"]),        // ← same strings, re-referenced
    by: That(Player),
    then: Each(binder: Existing(Them(Pile)), effect: DestroyNoRegen(It)),
  ),
)
```

The `"a"`/`"b"` labels are the last stringly binders in the corpus (core
`Binder` itself is already label-free after the Target(n)/naming passes). They
are exactly the shape the authored-surface contract rejects (no `Label`/`As`/
`into:` string binders).

## The typed replacement

The Idris north-star already carries a positional two-pile primitive,
`DivideAndChoose` (`idris/src/Core.idr`) — separate into two piles, opponent/
chooser picks one, the chosen pile takes the consequence — with NO string
labels. Do or Die is *"Separate all creatures target player controls into two
piles. Destroy all creatures in the pile of that player's choice. They can't be
regenerated."* ([CR#700.3a..700.3b]), which is precisely this shape.

Task: give Rust a `DivideAndChoose`-equivalent effect (or teach the emitter to
lower the two-pile `SeparatePiles`/`ChoosePile` special case onto it) so Do or
Die authors WITHOUT string labels, and re-encode the card to it. The general
N-pile `SeparatePiles`/`ChoosePile` may stay for the genuinely N-way cards
(Whims of the Fates); the two-pile chosen-consequence case is the one that maps
to `DivideAndChoose`.

## Done

- Do or Die re-encoded with no `into:`/`Labels(...)` string labels.
- `grep -rn 'into:\|Labels(' plugins/canon/cards/` is empty.
- idris-check: Do or Die emits and typechecks (canon count 51→52); the emit gap
  *"Effect::SeparatePiles not yet mapped (Idris's DivideAndChoose has a
  different two-pile shape)"* is gone.
- Engine behavior (separate → opponent chooses → destroy-no-regen the chosen
  pile) unchanged; a semantic engine test pins it.
