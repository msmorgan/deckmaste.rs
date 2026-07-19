---
needs: []
---
Structural drift between the Idris model (`idris/src/Core.idr`) and Rust core,
found by the 2026-07-16 drift review. No automated check catches these (see
`ci-idris-gate`); each cluster needs a design decision — mirror it, or record
the asymmetry as sanctioned (the way Draw already is).

Rust-side constructs unrepresentable in Idris:

- `Type::Dungeon` (`type.rs`) — no `Type_` variant; a Dungeon-typed card
  cannot be emitted or typechecked.
- `ObjectKind::{Player, CardCopy}` (`filter.rs`) — Idris `ObjectKind` has 5 of
  Rust's 7 variants.
- `DesignationScope::Game` (`designation.rs`) — Idris `Scope` is Object/Player
  only; game-scoped designations (monarch/initiative) have no Idris form.
- `Duration::ForThisEvent` (`continuous.rs`) — no Idris `Duration` equivalent.
- The whole `EnterRider` family (`action.rs`): `Tapped`, `FaceDown`,
  `UnderControlOf`, `UnderOwnersControl`, `WithCounters` — Idris `Move` /
  `CreateToken` carry only `enteringAttacking`, so "enters tapped / with a
  counter / under an opponent's control" cannot be emitted. The most
  card-relevant cluster.
- `DeonticAction::Untap` (`deontic.rs`) — Idris models "doesn't untap" as
  `Replaces [Becomes Untapped]`; confirm behavioral equivalence or mirror.
- `Reference::{Opponent, Bound, Linked, Source}` and `Count::{Noted,
  ManaAvailable, Opponents, ThatMuch}` — the Noted / labeled-read family
  Idris retired.
- `Selection::AmongNoted` / `PilesOf`, `Ability::Innate`, and
  `Arrangement::AnyOrder` (Idris's nullary `ChosenOrder` conflates it with
  `ChosenOrder(who)`).
- **`AsThough` — divergent in BOTH directions, unrecorded** (2026-07-18
  deep-dive). Idris keeps a live combinator `AsThough : Condition b ->
  StaticEffect b -> StaticEffect b` (Core.idr) and builds `haste` on it
  ("as though not summoning-sick" grants attack). Rust deliberately rerouted
  the whole as-though family to other channels — deontic rows absorb
  permission-shaped cards (`May(Cast(window: …))`), `SpendAsThough` absorbs
  mana [CR#609.4b] — and Rust haste is a bare name flag read by the Creature
  TypeDef's `Cant` rows (settled, rules-taxonomy §10). The residual Rust
  `enum AsThough` is `Expanded`-only with **no base variant**, i.e.
  unconstructable, and the engine evaluates it nowhere; `idris_emit` gaps on
  both `AsThough` and `SpendAsThough`, so haste's two encodings are never
  checked equivalent. Decide: sanction like Draw (record both encodings +
  why), migrate Idris haste to the flag+exception form, or grow Rust base
  variants when a card demands one (`core-asthough-glaring-spotlight` is the
  candidate seed — its deontic AsThough-permission design is the Rust-side
  route). Note the Rust factorization's scaling cost: each new
  sickness-defeating effect edits the Creature macro's conditions, where an
  AsThough grant would compose from outside.

Idris-side constructs with no Rust counterpart (decide: implement in Rust or
mark model-only): `Zone::Sideboard`, the Vote one-shot, `Priced Downstream`
(ward-style deferred pricing), `ChosenObject`/`ChosenPlayer` references,
`EventAmount`/`ChosenNumber` counts, and `EventAgg`'s Min/Max/Average spread
over Rust's `EventSum`. Also `eventKindObjectSort` (Core.idr) is caller-less
since the `object_sort` emitter column was deleted — keep as documentation or
drop.
