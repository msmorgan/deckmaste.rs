---
needs: []
design: true
---
**Encode Do or Die via the typed two-pile primitive; delete the last stringly
binder.** Do or Die is the ONLY canon card still identifying piles by string
label, and it is one of the idris-check emit gaps — one ticket closes both.

## Needs a design pass + split — investigated 2026-07-08

The premise is partly accurate (the stringly binder exists as described; Idris
`DivideAndChoose` exists at `idris/src/Semantics.idr`; the Rust grammar lacks it
and the emitter stubs it at `crates/deckmaste_plugin/src/idris_emit.rs`, which
now walks semantic terms). But three
blockers require decisions this ticket doesn't make, and the "engine unchanged"
premise has drifted. Recommend splitting into a `core:` emit/render/RON slice
(cleanly doable) vs a separate engine-resolution ticket, and deciding the
`other`-field representation first.

1. **Idris shape is not a clean 1:1 (mandatory `other`).** Idris
   `DivideAndChoose` takes both `chosen` AND `other` as mandatory sub-effects
   (only `chooser` defaults). Do or Die has no other-pile consequence. Mapping
   "no other consequence" needs a choice: make Rust `DivideAndChoose.other` an
   `Option`/default and have the emitter synthesize the Idris no-op
   `Sequentially []`, or force RON to spell an explicit empty `other`.
2. **Done item 4 ("engine unchanged; a semantic test pins it") is drifted — the
   engine never resolved this card.** `crates/deckmaste_engine/src/resolve/effect.rs`
   has no arm for `SeparatePiles`/`ChoosePile`/`DivideAndChoose`; they hit the
   catch-all `todo!` choice seam, and there is no existing Do-or-Die / pile
   engine test. So there is no behavior to leave "unchanged," and a semantic
   test can't pin a `todo!`-panic. Building it means designing a divide-and-choose
   subsystem with no engine precedent (divider partition decision, chooser
   pile-pick decision, binding chosen/other as the `Them(Pile)` Many anaphor).
   `ChoiceContinuation::ArrangePiles`/`arrange_piles()` is scry-style ordering
   within a known pile, not partition-and-choose — not reusable here.
3. **`no_dead_grammar` ripple.** Do or Die is the sole exerciser of
   `SeparatePiles`, `ChoosePile`, and `PileSource::Labels`. Re-encoding onto a
   new `DivideAndChoose` variant makes those three shapes dead; the test needs
   new DEFERRED exemptions (for the not-yet-built Whims of the Fates N-pile case)
   that this ticket doesn't mention. Also note the ticket's two suggested paths
   are mutually exclusive with its own Done: emitter-lowering keeps `into:`/
   `Labels` in the RON (fails Done 1–2); only a new core variant clears the grep
   (but triggers blockers 2 and 3).

Cleanly doable + specified: add a Rust `DivideAndChoose` core variant, wire the
emitter (closing the emit gap, canon count 51→52), add a render arm, re-encode
the RON without labels. Not specified / needs design: the `other` representation
(blocker 1), the engine resolution + decision surfacing that Done item 4 assumes
(blocker 2), and the `no_dead_grammar` exemptions (blocker 3). Drift surfaced by
the batch executor.

## The stringly binder

`plugins/canon/cards/Do or Die.ron` today threads string labels from the
split to the choice:

```
SeparatePiles(
  group: SelectAll(And([Type(Creature), ControlledBy(Ref(It))])),
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
`DivideAndChoose` (`idris/src/Semantics.idr`) — separate into two piles, opponent/
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
