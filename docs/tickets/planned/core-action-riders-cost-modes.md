---
needs: [cards-elaborator-tables, cards-elab-load-gate]
---
**Verb deltas (enter riders, destinations, argument structure) and the
cost-mode family (optional costs + modal cost riders).** The action-vocabulary
half of the grammar reshape, batched so canon takes one mechanical edit pass
and wizards one regeneration.

Related: [[core-alt-costs]] (its "if the X cost was paid" linkage lands here as
`PaidCost`), [[engine-alt-costs]], [[core-destination-library-overlap]].

## Enter riders and destinations

```rust
Move(Reference, Destination, riders: [EnterRider] = []),
MoveGroup(GroupRef, Arrangement, Destination, riders: [] ),
Create(Count, TokenSpec, riders: [EnterRider] = []),
enum EnterRider { Tapped, FaceDown, UnderControlOf(Reference), UnderOwnersControl,
                  Attacking(Option<Reference>), WithCounters(CounterRef, Count) }
enum Destination { Zone(Zone /* Stack, Sideboard excluded */),
                   Library(Anchor /* REQUIRED */) }
enum Arrangement { ChosenOrder(Reference), AnyOrder, SameOrder, RandomOrder }
```

- Riders are battlefield-only — an elaborator rule with a reject fixture, not
  a comment. Ordered-zone positions exist only via `Library(Anchor)`;
  Stack/Sideboard are not destinations (unrepresentable).
- `MoveGroup(g, AnyOrder, Library(FromTop(0)))` = Brainstorm's "on top of your
  library in any order" — the group-arrangement acceptance fixture.
- `Duration::ForThisEvent` — a rider duration in force while the carrying
  instruction's event executes; the footing for "It can't be regenerated."
  [CR#701.19c] (the `DestroyNoRegen` macro rides it later,
  [[macro-first-wave]]).
- Render-back requirement (enforced once [[cards-fidelity-target-sunset]]
  lands): riders the oracle sentence prints must render — Path-to-Exile-style
  search-tapped, reanimation-under-your-control, and blink encodings carry
  their riders explicitly.

## Verb argument structure

Rule: every verb carries its CR keyword action's full argument structure.

```rust
Search { for: Filter, in: [ZoneOf], by: Reference = You, quantity: Quantity = One,
         if_none: Option<Box<Effect>> },   // explicit whiff branch
Discard { count: Count, who: Reference = You, what: Option<Reference>, random: bool = false },
Draw { count: Count, who: Reference = You },
Mill { count: Count, who: Reference = You },
Reveal { what: GroupRef, to: Option<Reference> },  // to = a player ⇒ "look at" [CR#701.20e]
DealDamage(Participant, Count, source: Reference = This),
Fight(Reference, Reference),   // PRIMITIVE with native semantics [CR#701.14a..701.14d]
FlipCoins(Count), RollDice(Count, Sides), ExtraPhase(PhaseStep, Reference),
BecomeDay, BecomeNight, VentureIntoDungeon(Reference), TheRingTempts(Reference),
```

- Player verbs carry `who`; `who ≠ You` is the declarative-subject case whose
  renderer/parser production lands with [[macro-first-wave]].
- Whiff semantics are DATA: a whiffed `Search` skips dependent clauses via
  elaborated `needs`/`depends_on`, never engine grace [CR#701.23b]; reading
  the product inside an `if_none` branch is a load error.
- `Fight`'s execution semantics (both-or-neither, self-fight) are engine work
  in [[engine-fact-record-batch]]; this ticket lands the shape.

## Cost modes

```rust
struct OptionalCost { components: [CostComponent], tag: CostTag, repeatable: bool = false }
// read by Condition::PaidCost(tag), Count::TimesPaid(tag), Filter::WasPaidWith(tag)
struct ChooseSpec { count: Quantity, up_to: bool = false, repeats: bool = false,
                    chooser: Reference = You, rider: Option<ModalCostRider> }
enum ModalCostRider { Entwine(Cost), Escalate(Cost) }
```

- `OptionalCost` is the kicker/multikicker/buyback identity: one tag, three
  read channels; the bespoke `WasKicked` condition is retired (multikicker =
  `repeatable: true` + `TimesPaid`).
- Announcement semantics: modes and optional-cost intentions are declared at
  mode choice [CR#601.2b]; per-mode targets are chosen only for chosen modes
  [CR#601.2c]; the total cost locks at cost lock [CR#601.2f].
- `Entwine(c)`: printed count stays `Exactly(1)`; the caster may instead
  choose ALL modes by adding `c` [CR#702.42a]. `Escalate(c)`: printed "one or
  more" count; total cost += `c` × (modes chosen − 1) [CR#702.120a].
- Acceptance fixture: **Collective Defiance** — escalate rider + per-mode
  targets + the mode-1 discard-then-draw amount anaphora in one card.

## Done

- Shapes above land in `deckmaste_core` with round-trip serde tests; riders
  battlefield-gate + `if_none` product-read + `Library(Anchor)` rules each
  have a reject fixture.
- `WasKicked` deleted; kicker-family macros re-expressed over `OptionalCost`
  (macro bodies only; no new keyword work).
- Canon edited mechanically where shapes changed; wizards regenerated.

## Verification

- `cargo test --workspace`; `cargo xtask validate` clean on hand-authored
  plugins; `cargo xtask generate plugins/wizards` then re-validate.
- `cargo xtask elaborate --lock` re-blessed deliberately.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
