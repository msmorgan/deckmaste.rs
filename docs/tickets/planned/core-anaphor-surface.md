---
needs: [core-eventfilter-master-forms, core-action-riders-cost-modes, engine-fact-record-batch]
---
**The anaphor surface: sorted anaphors over the antecedent stack, the
telescope `Seq`, and the R2 ambiguity gate.** The authored surface becomes
English-ordered — clauses introduce entities (targets, products, event roles,
choices) onto the ordered antecedent stack, and references are sorted anaphors
resolved to the nearest compatible antecedent by the deterministic elaborator.
Sentence order IS binder order.

Wire policy (a binding ruling): the wire serializes SURFACE anaphors
(`It`/`That(Card)`/`They`) — never resolved indices. Resolution is a pure
function of the tree; the R2 gate makes silent re-pointing structurally
impossible (an insertion flips "resolves" → "load error", never → "resolves
differently"); `cards.elab.lock` + `cargo xtask elaborate --dump` pin and
expose the computed indices in review.

Related: [[core-with-rebindable-that]], [[endophora-consolidation]],
[[engine-bound-references]] (labels/`The` generalize the named-role store).

## Surface

```rust
enum Sort { Player, Card, Token, Spell, StackObject, Permanent,
            OfType(CardType), Amount, Pile }
enum Reference {
    This, You,                          // source / controller [CR#113.7]
    It,                                 // nearest singular antecedent, any sort
    That(Sort),                         // nearest singular antecedent OF THIS SORT
    The(Ident),                         // labeled antecedent (ambiguity fallback)
    Target(usize),                      // explicit slot — stays legal, DEPRECATED
                                        //   (sunset: [[cards-fidelity-target-sunset]])
    A { filter: Filter, by: Box<Reference> /* = actor */ },  // indefinite; actor
                                        //   chooses; pushes a Chosen antecedent
    ControllerOf(_), OwnerOf(_), AttachHostOf(_), AttachedTo(_),
    Linked(Ident),                      // [CR#607] linkage store
    EventObject, EventPatient, EventActor, EventSource, DefendingPlayer,  // caps-gated
}
enum GroupRef { They, Them(Sort), TheGroup(Ident), Targets(usize),
                Matching(Filter), Union([GroupRef]), Random(Quantity, Filter),
                TopOfLibrary { count, of }, BottomOfLibrary { .. },
                ChooseN { q, filter, by }, PilesOf { note: Ident, of: Reference },
                Pick { op, of, by } }
// value anaphora: Count::ThatMany (spelling alias ThatMuch, renders "that much"),
//                 Allotment, ChosenNumber, Noted(Ident)
```

`Kind { Obj, Player, Any }` — `Any` is the join, "any target" [CR#115.4].

## Introduction (the emitted `intro` table)

Every producing clause pushes antecedents; the rows are emitted from Idris and
CR-cited. Highlights: a `Targeted` slot pushes One/Many per its quantity;
`Move`/`MoveGroup` push a product with the destination's `expected_zone`
(battlefield → `Permanent`, refined to `OfType(t)` when the moved filter pins
a type; stack → `Spell`; else `Card` — this is why "exile target **creature**
… return that **card**" resolves); `Search` pushes a whiffable product;
`Create`'s cardinality is DERIVED from the fact-signature product-arity
column, never author-asserted; amount-bearing verbs additionally push an
`Amount` antecedent ("that much"); trigger/replacement/payment bodies push one
antecedent per caps guarantee; `Each`/`Divide` push `Loop` (+ `Allot`)
antecedents popped after the body; `May.if_did` elaborates in the paid
effect's context; `MayPay.and_then` at the cost's caps.

Survival rules: `Delayed` bodies drop all `TargetSlot` antecedents and keep
products with `expected_zone` [CR#603.7c]; `Instead`/`Also`/static bodies push
event roles and set `may_target = false`; `Reflexive` sees the full outer
context [CR#603.12a].

## Resolution (normative; implemented in Rust here, mirrored in Idris v2)

- **R1 (nearest):** an anaphor resolves to the nearest stack entry whose kind,
  cardinality (`It`/`That` = One; `They`/`Them` = Many; `ThatMany` = Amount),
  and sort are compatible (`It` = wildcard; `Permanent` reaches
  `Permanent`/`OfType(_)`; `OfType(t)` reaches exactly `OfType(t)`;
  `StackObject` reaches `Spell`/`StackObject`).
- **R2 (uniqueness gate):** elaboration FAILS if a second same-kind,
  any-compatible antecedent exists anywhere in the stack, unless the nearer
  matches the anaphor's sort exactly and the farther only via wildcard
  widening. Error text: "ambiguous reference — add `Label`/`The`, or use
  `Target(n)`." The gate freezes only after the corpus dry-run
  ([[cards-corpus-dry-run]]).
- **R3 (no forward references):** strictly leftward; "It becomes night" is the
  verb `BecomeNight`, never a reference.

## Effects

- **Telescope `Seq([Effect])`:** clause i+1 elaborates in `intro(clause i,
  ctx)` — the parser's binder-inversion pass dies; parsers emit sentence-order
  `Seq`.
- `SeparatePiles { group, into: [Ident], by, note, then }` — n labeled piles
  as Many antecedents; `note:` persists them as noted groups keyed by
  (note, label, divider), read back via `PilesOf { note, of }`;
  `ChoosePile { from: Labels([Ident]) | Noted { note, of }, by, random, then }`.
  Fixtures: a 2-pile no-note card (Fact-or-Fiction shape) and the
  all-players-pile-then-all-players-sacrifice shape (Whims of the Fates) —
  per-player noted piles, two `Each` loops matching the oracle's "Then".
- `Until(Duration, [StaticEffect])` takes a LIST; fixed-vs-live affected sets
  are a per-part class column in the emitted tables [CR#611.2c]:
  characteristic-/controller-modifying parts gather once at start;
  deontic/prevention/replacement/cost parts stay live. `Static` ability
  position stays live re-gathering [CR#604].
- `Divide` reads its amount only through `Allotment`; a range target slot
  pushes a Many antecedent (`They` reads it; singular `It` there fails).
- Whiff discipline: product reads downstream of a whiffable clause carry
  elaborated `depends_on` and are runtime-skipped, defined not grace
  [CR#701.23b].

Signature acceptance fixture (exile-and-return-at-end-step, the
Otherworldly-Journey shape): slot 0 pushed → `Move(It, Exile)` reads it and
pushes a `Card` product with `expected_zone: Exile` → `Delayed` drops the
target antecedent [CR#603.7c], keeps the product → `That(Card)` resolves to
the exiled card; `That(Creature)` is the pinned load-error twin.

## Done

- Anaphors + telescope + piles land; intro/survival/compat tables emitted and
  consumed; R1–R3 implemented with `E-BIND-*` codes; every rule has a reject
  fixture (unbound anaphor, ambiguous `It`, forward reference, sort mismatch,
  singular-vs-Many, `ThatMany` with no amount antecedent).
- Parsers emit sentence-order `Seq`; graduation count does not regress and is
  expected to rise (search-put chains, delayed templates).
- Canon accepts new spellings; `Target(n)` still loads (deprecation warning
  only).

## Verification

- `idris2 --build mtg.ipkg` + `idris2 --exec emitTables` (in `idris/`).
- `cargo test --workspace`; `cargo xtask validate` clean; wizards regenerated
  and re-validated; `cargo xtask elaborate --lock` re-blessed deliberately.
- `cargo xtask elaborate --dump` reviewed for the signature fixtures.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
