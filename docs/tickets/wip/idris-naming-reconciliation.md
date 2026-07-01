---
needs: []
---
**The Idris model is the canonical CONCEPT source; reconcile names to it (naming
reconciliation, corrected).** This supersedes the 2026-06-29 decision originally
recorded here — "the Rust idiom is canonical; rename the Idris model to match." That
reading let concept-drift hide behind name-agreement: because a Rust name already
existed, the idris→rust port reused it even where the Rust *concept* was weaker, and
the shared name then masked the divergence. That is exactly what produced the
fragmented `Subject` / `ThatObject` / `ThatPlayer` anaphors and the first-of-many
group-move bug — `With(Choose(2), Move(That, …))` silently read only the FIRST of
the two chosen cards, because a many-binder's `That` had been collapsed onto a
single-object read while keeping the reassuring `That` name. See
[[core-bindable-unification]] for the north star.

## Policy (corrected)

**Mirror Idris *concepts*. Keep a Rust *spelling* only where the concept is
identical. Never preserve a weaker Rust concept behind a matching name.**

The Idris model is dependently typed, so each concept is pinned by its type — the
cardinality index, the `RefKind`, the `Bindings` typestate. A Rust name that "looks
the same" but carries a looser concept is a silent downgrade, not a convenience.
When a name and a concept disagree, the concept wins: rename, split, or fold the
Rust side to match the Idris concept, even at the cost of an established spelling.
Name-agreement is never evidence of concept-agreement — check the concept.

## Worked example — `Binder` / `It` / `That` by slot (this whole initiative)

The `core-anaphor-mirror` initiative is the reference application of the policy:

- **One `Binder`** (Idris `Bindable`) feeds `With` / `Each` / `DivideAmong`,
  replacing the two-names-one-concept pair (`With.binder` + `Each.over: Selection`).
  Cardinality (One/Many) rides the variant, mirroring the Idris cardinality index;
  a many-binder bound as a group is what made the first-of-many bug fixable.
- **`It`** — ONE anaphor, the per-element role, deliberately OVERLOADED across
  iteration (the `Each` / `DivideAmong` loop element) AND projection (the candidate
  a per-object filter / extremal `Pick` is currently testing). It is NOT split per
  use-site; the old `Subject` role folds into it.
- **`That`** stays `That`, resolved BY SLOT: a one-binder's `That` is a single
  `Reference::That`; a many-binder's group `That` is a `Selection::That` (same
  spelling, the concept distinguished by which slot reads it). The invented
  `Selection::Those` is dropped.
- Event roles fold to the Idris-shaped set: `EventObject` (the doer/source) and
  `EventActor` (the responsible player), with `EventPatient` / `DefendingPlayer`.

## Corrected anaphor mapping (replaces this ticket's earlier mapping line)

The earlier mapping `It` / `That` → `ThatObject` / `ThatPlayer` ("Rust keeps these
as aliases of EventAgent/EventActor") was wrong on every count — it is the very
conflation the policy now forbids. Replaced by the reconciliation actually landed in
`core-bindable-unification` (#1):

- `It` → **`It`** — one anaphor, overloaded across iteration AND projection; do NOT
  split it into object/player or per-use variants.
- `That` → **`That`** — same spelling, same concept (singular vs group resolved by
  slot, not by a new name).
- `Subject` / `ThatObject` / `ThatPlayer` → folded into **`It`** (the projection /
  iteration element) and the event roles **`EventObject`** / **`EventActor`**. These
  three Rust spellings were the fragmentation the policy forbids: three weaker,
  slot-specific concepts wearing distinct names where Idris has one overloaded `It`
  plus the event roles.

## Rename list, reframed under the corrected policy

### Concept-correcting (landed in #1–#4 by mirroring Idris — recorded, not pending)
- anaphors: `Subject` / `ThatObject` / `ThatPlayer` / `Selection::Those` →
  `It` / `That` / `EventObject` / `EventActor` (above).
- `Each` / `DivideAmong` / `With` unified on one `Binder` (Idris `Bindable`). This
  REVERSES the earlier `Each → ForEach` rename: `Each` is both the Idris concept and
  the kept Rust spelling.
- event source: keep `EventObject` (Idris), reversing the earlier
  `EventObject → EventAgent`; the redundant `EventAgent` alias is dropped, leaving
  `EventObject` (source) + `EventActor` (player).

### New — `Bindings` → `Endophora`
Rename the Idris `Bindings` typestate record (`Core.idr`, `record Bindings`) to
`Endophora`. Rationale: the record is exactly the set of *text-internal*
(endophoric) references in scope — in BOTH directions: **anaphora** (backward —
"choose a creature; destroy **it**", read as `It` / `That`) and **cataphora**
(forward — "deal 2 to **each creature**", the distributive element introduced by
the quantifier that binds it). Every field it tracks (`targetKinds`, `thatKind`,
`itKind`, `eventCaps`, `chosenKind` / `chosenRefKind`, `hasAllotment`) is a bound,
text-internal reference, and nothing else belongs in it. `This` / `You` /
`Opponent` stay OUTSIDE it because they are *exophoric* — situational references
resolved from game state, never bound by the text. So `Endophora` is both the
correct linguistic cover term AND an exact description of the set the record
tracks. The Rust-side consolidation of the same set is tracked by
[[endophora-consolidation]].

### Pure-spelling reconciliations (identical concept — the licensed "keep a spelling" case)
These differ ONLY in spelling, with identical concepts — the one case the policy
permits keeping a Rust spelling. Low-stakes, no concept at risk; carry them as
cosmetic cleanup, not as part of the soundness work.
- `Cmp`: `Equal` / `GreaterEq` / `LessEq` ⇄ `Eq` / `AtLeast` / `AtMost`
  (`Greater` / `Less` already match).
- `ObjectKind`: `IsCard` / `IsEmblem` / `IsSpell` / `IsToken` / `IsAbility` ⇄
  `Card` / `Emblem` / `Spell` / `Token` / `Ability` (the enum name supplies the
  `Is`).
- relations: `Opponent` / `Teammate` ⇄ `OpponentOf` / `TeammateOf` (the `…Of`
  suffix matching `ControllerOf` / `OwnerOf`).
- `PipClass`: `GenericPip` / `ColorPip` ⇄ `Generic` / `Colored`.
- distribution verb: Idris `Distribute` ⇄ Rust `DivideAmong` (same
  `Bindable b Many` divided-distribution concept — verify it stays the same concept
  before adopting either spelling).

Already identical across both models (no change): `AdditionalCost`, `MayPay`,
`MustPay`, `Unless`, `PayPips`, `TapTotal`, `MoveCounters`, `Destination`, `Anchor`,
`FromTop` / `FromBottom`, `ManaCostOf`, `CountDistinct`, `Allotment`, `TurnOf`, and
the `Quantity → Range` + named-macro approach.

Out of scope (an addition, not a rename): Rust `Count::Plus` has no Idris
counterpart — only adopt it into the Idris `Count` if you also want the operator
there.

Severity: **policy correction + consistency.** The concept-correcting items are
soundness-relevant and already landed in #1–#4; `Bindings → Endophora` and the
pure-spelling items are cleanup. The `.idr` / `.md` files are cite-scanned, so
re-run `cargo xtask cite check` after touching them.
