---
needs: []
---
Structural drift between the Idris model (`idris/src/Core.idr`) and Rust core,
originally found by the 2026-07-16 drift review. Per-cluster decisions taken
2026-07-19 (walked one by one); this ticket is now the execution checklist.

**Policy: MIRROR by default** (user ruling 2026-07-19). Converge the two sides
rather than record a sanctioned asymmetry — the Draw-style "both encodings + why"
hatch is deprecated (Draw itself sunset, converging when `idris-retire-cards-idr`
lands). Sanction only where mirror is genuinely infeasible (an Idris-only model
probe with no Rust meaning). NOTE: bare variant-count drift was misleading on
almost every cluster — the real question was *why* the shapes differ, and often
the smaller side was the more correct one. Re-verify against current code before
executing; this ticket dates and drifts.

## Grow Idris (mirror Rust → Idris)

- **`Type::Dungeon`** — add a nullary `Dungeon` to Idris `Type_` [CR#300.1]. The
  deeper composite-type unification (Idris adopts open `TypeDef`+confers like
  Rust) is spun out to `maybe/idris-composite-card-types`; this ticket does the
  substrate add only.
- **`ObjectKind::CardCopy`** — add `CardCopy` to Idris `ObjectKind` (a real
  object kind, [CR#109.1]). Do NOT add `Player`: players are not objects
  [CR#109.1]; Idris models them via the `APlayer` sort. Rust's `ObjectKind::Player`
  is a filtering convenience — record, don't mirror.
- **`DesignationScope::Game`** — add `Game` to Idris `Scope`; change
  `scopeRef : Scope -> Maybe RefKind` (Object→`Just AnObject`, Player→`Just
  APlayer`, Game→`Nothing`, since a game-global has no carrier reference). Every
  `scopeRef` caller must handle `Nothing`.
- **`EnterRider`** (most card-relevant) — add an Idris `EnterRider` ADT and
  replace the lone `enteringAttacking` field on `Move`/`CreateToken` with
  `{default [] enterRiders : List (EnterRider b)}`. Include NOW: `Tapped`,
  `WithCounters(counter, count)`, `UnderControlOf(player)`, `UnderOwnersControl`,
  `AsCopy` (reuse the existing `Copy`/`BecomeCopyOf` payload, [CR#707.9]), plus
  `Attacking` (absorbs the old field). **Defer `FaceDown`** — Idris has no
  face-down state yet (→ `engine-face-down`).
- **`Reference::Linked`** — add a linked-ability reference [CR#607] to Idris
  (Oblivion Ring's "the exiled card"); currently the `idris_emit.rs:572` gap.
- **`Arrangement`** — disambiguate: rename Idris's nullary `ChosenOrder` →
  `AnyOrder` (its real meaning is owner-default, [CR#401.4]) and add
  `ChosenOrder : Reference b APlayer -> Arrangement` for an explicit non-owner
  chooser ("target player arranges them"). Matches Rust's 4-way.

## Fix BOTH to the CR-correct primitive

- **`DeonticAction::Untap` → `CantHappen(Becomes Untapped)`** [CR#614.17]. "Doesn't
  untap" is a can't-happen effect (the untap event never *would* happen,
  [CR#502.3] "keep from untapping"), NOT a replacement ([CR#614.17] "aren't
  replacement effects") and not a player-action restriction. Idris: replace the
  `Replaces [Becomes Untapped]` note/impl (`Core.idr:274`) with `CantHappen`
  (already exists, `Core.idr:2877`). Rust: retire `DeonticAction::Untap`, model
  via `StaticEffect::CantHappen(Becomes Untapped)` (already exists). Winter Orb /
  Static Orb / exert [CR#701.43a] / Frost Titan unify as windowed `CantHappen`.

## Don't mirror (sanctioned — genuinely not card grammar / correct as-is)

- **`Duration::ForThisEvent`** — an engine instruction-scoped lowering rider
  (footing for DestroyNoRegen); Idris expresses "can't be regenerated"
  deontically (`cant (Enact Regenerate …)`). Keep engine-only. Verify at
  execution that Idris captures the *event-scoping* via its window/`Replaces`
  mechanism; if not, that's a deontic-scoping gap, not a missing `Duration`.
- **`Count::ManaAvailable`** — a strategy-only reader of the floated mana pool
  (`strategy.rs:753`); no card reads it. Runner-side ([[engine-runner-boundary]]);
  Idris correctly excludes it from card grammar.
- **`Ability::Innate`** — an engine look-through wrapper ([CR#113.12]), like
  `AsThough::Expanded`. Engine-only; no Idris mirror.
- **`Reference::{Opponent, Source}`, `Count::{Opponents, ThatMuch}`** — Idris
  folds these into general mechanisms: Opponent → a player selection/predicate;
  Opponents → `CountOf` over an opponent-selection; `Source` → `This` [CR#113.7]
  in predicate scope; `ThatMuch` ≡ Idris `ThatMany` [CR#608.2i]. No mirror.

## Rust's in-flight retirement (Idris already correct)

- **`Reference::Bound`, `Count::Noted`, `Selection::AmongNoted`** — the stringly
  labeled-anaphora family (zero canon usage). Idris correctly uses positional
  anaphora (`It`/`That`/`ThatMany`) per the no-stringly-binders ruling
  ([[authored-surface-ergonomics-rulings]]). Rust is retiring these via the
  planned `engine-bound-references` + `parse-subject-filter-stringly-channels`
  tickets. No Idris change; converge by Rust's retirement.
- **`Selection::PilesOf`** — stringly pile-division read; defer until the piles
  mechanic is built, then give Idris a positional model.

## Rust catches up to Idris (Idris is the reference model)

- **as-enters-choices** (`ChosenObject` / `ChosenPlayer` / `ChosenNumber` via
  `AsEntersChoosing`) — Idris modeled these ahead; Rust implements via the
  planned `core-as-enters-choices` ticket (Iona / Meddling Mage / Steely Resolve,
  the BLOCKED-13 cluster). Idris stays the reference.

## Idris-model-completeness carve-out (implement in Rust when scheduled)

- **`Zone::Sideboard`** (wishes / outside-the-game), **`Vote`** (council) — real
  but unbuilt mechanics; Idris-ahead is legitimate. Rust implements when the
  mechanic is scheduled; keep the Idris forms.
- **`Priced Downstream`** — the ward-style tax; Rust already models this as
  `Toll` (`deontic.rs:336`). Verify behavioral equivalence and record as a
  vocabulary difference, not drift.
- **`EventAgg` Min/Max/Average vs `EventSum`** — Idris folds event amounts by any
  op; Rust is sum-only (`count.rs:270`). Keep Rust sum-only; Idris is the
  reference. Grow Rust to `EventAgg(op)` only when a card needs min/max/avg over
  event amounts.

## Already resolved / no-op

- **`AsThough`** — stale: Rust grew `AsThough::Counterfactual`
  (`deontic.rs:71`), the engine evaluates it (`legal.rs:1051`), and it emits.
  Only `SpendAsThough` remains a one-way `idris_emit` gap — fold into the emit-gap
  list below.
- **`eventKindObjectSort`** — now caller-ful (`queryObjectSort`→`queryRoles`
  chain); the "caller-less, drop?" premise is stale. No action.

## Rust→Idris emit-gap arms (mirror candidates, surfaced 2026-07-19)

`Card::TwoFaced` (`idris_emit.rs:3608`, unblocks Wear//Tear + Brazen Borrower +
Delver in one arm — cheapest win), `Action::CreateReplacement` (`:1759`),
`Count::EventCount` (`:1125`), `EventFilter::Nth` (`:3039`), `Reference::Linked`
(`:572`, = the mirror above), subtype-confer `Property::TurnBased` (`:335`),
and `SpendAsThough`. These bridge the emit path so the affected cards exercise
`Core.idr` arms; schedule alongside the grow-Idris work.

Standard constraints apply. Deltas: `idris-check` is a local-only gate (no idris2
in CI); several items (Untap, EnterRider) touch BOTH Rust core and `Core.idr` —
regenerate wizards after core enum changes; the `scopeRef` signature change ripples
to all callers.
