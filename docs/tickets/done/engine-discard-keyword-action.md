---
needs: [engine-keyword-action-intent]
---
Fold **Discard** into the unified CR-701 keyword-action family built by
[[engine-keyword-action-intent]] — Discard becomes `Composite(tag, body)` wrapped
by the one present-tense `Act` event, exactly like Destroy / Mill / Draw. This is
the "(Discard later)" follow-up deferred out of that ticket; direction (user,
2026-07-14): "it should become composite like the others."

## Why deferred until now

The keyword-action ticket shaped Discard but left it unbuilt ("next atom once a
fixture forces it"). Discard is genuinely harder than Draw/Mill: those are
deterministic top-of-zone moves with pure-data bodies, whereas Discard's body is
a **player choosing cards from a hidden zone** ([CR#701.9b]) — plus the
"at random" and "another player chooses" variants, and the [CR#701.9c]
hidden-zone / undefined-characteristics wrinkle. It wants a fixture-driven design
pass, not a mechanical fold. That fixture now exists: **madness** ([CR#702.35])
is a *replacement* of the discard-to-graveyard move ("if a player would discard
this card... exiles it instead"), so it forces `Act(Discard)` to be a real,
replaceable event with an intent window — the same role `Act(Destroy)` plays for
regeneration/indestructible.

## Current bespoke shape (to retire)

- `PlayerAction::Discard { count, what, random }` (`crates/deckmaste_core/src/action.rs:302`)
  — resolved directly, no intent event. `what` names *which* cards (omitted =
  discarding player chooses `count` from hand, [CR#701.9b]); `random: true` is the
  discard-at-random form; `Discard(count: Literal(1), what: This)` is cycling's
  "discard this card" cost ([CR#702.29a]).
- `WillDiscard` — **shaped, unbuilt** (`crates/deckmaste_engine/src/event.rs:161`),
  the placeholder for madness's replacement window. Retire the *idea* of it: like
  `WillDraw`/`WillDestroy`, the unified `Act(Discard)` subsumes it — no separate
  intent event.
- `Cause::discard` verb already exists (`event.rs:102`) — reuse for the
  cause-tag on the committed hand→graveyard move.
- Macro `Discards.ron` exists; add/reconcile a `Discard` (You-form) twin,
  mirroring `Draw`/`Draws`, `Mill`/`Mills`.

## Target shape — `Composite(Discard(who, count), body)`

Mirror Draw, but the body's binder is a **choice from hand**, not a
top-of-library slice:

- **Tag facet:** `KeywordAction::Discard(Reference /*who*/, Count)`. Deontic /
  trigger / replacement layers key on `Act(Discard(who))` — "a player discards".
  Madness = `Replaces(Act(Discard(Ref(This))) → exile instead of Graveyard)`
  ([CR#702.35a]); "can't discard" / "discards at random instead" become
  `CantHappen` / `Replaces` over the one atom.
- **Body facet:** the hand→graveyard move of the chosen cards ([CR#701.9a]) —
  `Each(binder: Existing(<selection>), effect: Move(It, Graveyard))`, where
  `<selection>` is a hand selection that carries the chooser + count + random
  flag (the `what`/`random` detail lives in the Selection, not the tag). The
  discarding player choosing which cards ([CR#701.9b]) is a real decision inside
  the body; the "another player chooses" / "at random" variants are selection
  parameters, not new atoms. Confirm the existing `Selection` vocabulary can
  express "N chosen from hand (by chooser P, optionally at random)"; extend it if
  not (Selection is sacred surface — [Authored card surface](../../decisions/authored-card-surface.md) — so
  any new variant is a design call, flagged before building).

## Open design questions (resolve before building)

- **Per-card vs batch.** Draw decomposes to N single-card attempts ([CR#121.2]);
  Mill is a simultaneous batch ([CR#701.17a]). Which is Discard? [CR#701.9]
  reads as a single choice of `count` cards then a move — likely a **batch**
  (one selection, group move), closer to Mill than Draw. Confirm against the CR
  and any multi-card-discard trigger fixtures ("whenever you discard a card"
  firing once per card vs once per event).
- **Undefined characteristics** ([CR#701.9c]) — the discard-to-hidden-zone
  cost-illegality case (a discarded card whisked to a hidden zone has undefined
  characteristics; a cost keyed on its characteristics is illegal, game rewinds).
  Decide whether this lands in `Act(Discard)` apply or stays a
  cost/handling-illegal concern ([CR#733]).
- **`what: This`** (cycling / "discard this card") — a degenerate zero-choice
  selection; make sure the composite form still expresses it without a player
  decision.

## Scope / parity

- **Rust:** retire `PlayerAction::Discard`; add `KeywordAction::Discard`;
  `Action::discard(who, count, …)` ctor emitting the Composite; resolve →
  `Act(Discard)`; apply commits the chosen hand→graveyard move (cause-tagged
  Discard); render arms (imperative "Discard N" + "{} discards {}"), including
  `each_collective`; cost-lint eligibility.
- **idris:** add `Discard` to `KeywordActionSpec` (parity with
  `Scry`/`Mill`/`Draw`); `discard`/`discards` macro in `Macros.idr`; migrate any
  `Cards.idr`/`Spec.idr` discard sites; re-emit gate green.
- **Tests:** a madness fixture exercising the `Act(Discard)` replacement window;
  chosen-card + at-random + `what: This` discard fixtures; wizards regen + full
  suites.

## Done when

Zero bespoke `PlayerAction::Discard` / `WillDiscard`; Discard authored as
`Composite(Discard(who, n), <hand→graveyard>)`; madness expressible as a
replacement over `Act(Discard)`; Rust + idris re-emit + wizards corpus green.

## Resolution (2026-07-16)

Built as specced; rulings settled during the fold:

- **Batch choice, per-card events.** One `DiscardCards` decision picks the
  cards ([CR#701.9b]); the submission mints one dual-facet `Act(Discard)`
  per card (who + card + Hand→Graveyard facet, cause `Discard`), each
  individually cantable/replaceable — a multi-discard fires "whenever a
  player discards a card" once per card (Liliana's Caress ruling), and
  madness reroutes ITS card only ([CR#702.35a]). Cleanup's
  discard-to-hand-size and cost payments ride the same `discard_batch`, so
  madness works off cycling and cleanup discards for free.
- **`Selection::FromHand { count, whose, random }`** — the one new Selection
  primitive (idris twin added), following the zone-anchored family
  (`TopOfLibrary`/`TopOfGraveyard`). **No `chooser` field**: "another player
  chooses" is authored as look + choose + bound discard (the Coercion
  shape, `Action::discard_what`), never a selection flag.
- **Bound form** ("discard this card", [CR#702.29a]) =
  `Composite(Discard(who, 1), Move(what, Graveyard))` — destroy's
  single-move lane, committed atomically, no decision.
- **Costs**: `CostComponent::Do` widened from `Box<PlayerAction>` to
  `Box<Action>` (= the idris `Do : Action b -> Cost b`); existing RON
  spellings unchanged via the `By` embed. New cost macros `DiscardThis`
  (cycling/reinforce) and `DiscardCards(n)` (Blood token); effect macros
  `Discard`/`Discards` rebuilt as composites + `DiscardAtRandom`/
  `DiscardsAtRandom` twins.
- **[CR#701.9c]** (hidden-zone discard, undefined characteristics → illegal
  cost payment, [CR#733] rewind) — a cost/handling-illegal concern, NOT this
  atom; lands with the [CR#733] machinery.
- **Madness proper** still needs its keyword macro + the hand-zone static
  sourcing ([[engine-static-ability-zone-gating]] — replacements are
  gathered from the battlefield today); the `Act(Discard)` window itself is
  built and fixture-tested (`madness_style_replacement_exiles_its_card_only`,
  the Bag-of-Holding shape).
