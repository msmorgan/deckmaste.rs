---
needs: [macro-typed-holes-contracts]
---
**The first macro wave over the v2 grammar + the declarative-subject parser
production.** Everything English names is a macro; core keeps only nodes
pinned to a CR event family that replacements can intercept independently.
Each wave macro ships with a render template and an acceptance test.

Related: [[core-saga-chapters]], [[engine-sagas]], [[macro-amass]],
[[macro-keyword-templates]].

## The wave

- **Ward** — ONE blessed spelling: `Composite` conferring
  `Innate(Triggered(event: BecomesTarget(what: Ref(This), by:
  opponent-controlled stack object), effect: MustPay(actor: controller of
  that stack object, cost: the ward cost, or_else: Counter(it))))`
  [CR#702.21a]. Any alternative ward home (cost-shaped/"toll" encodings) is
  deleted. Ward `{X}` puts the X definition in the conferred ability's
  `where_x`, evaluated when the ward TRIGGER RESOLVES, not locked at trigger
  time [CR#702.21b] — acceptance fixture: Minthara, Merciless Soul ("ward
  {X}, where X is the number of experience counters you have").
- **Kicker** — `CostOption` over an `OptionalCost` tagged `Kicker`; reads are
  `PaidCost(Kicker)`; multikicker = `repeatable: true` + `TimesPaid(Kicker)`.
- **Event verbs** — `Dies(f)` = battlefield→graveyard `ZoneChange`;
  `Sacrificed`/`Enters`/`ThisEnters`/`ThisAttacks`/`NextEndStep` — thin
  macros over master forms; the entailment table does the matching work.
- **Chapter** (sagas as data): `Innate(Triggered(event:
  OneOrMore(CounterPlaced(kind: Lore, on: Ref(This))), if: Crossed(value:
  CountersOn(Lore, This), threshold: AtLeast(n)), effect: …))` [CR#714.2b];
  the Saga subtype registry row confers the enters-with-counter `Also`
  [CR#714.3a], the turn-based lore increment, and the final-chapter sacrifice
  SBA.
- **Mill** — subject-bearing: `params: {n: Count, who: Default(Reference,
  You)}`, template `"${who} mills ${n:card|cards}"`, body = `MoveGroup` of
  the top `n` of `who`'s library to the graveyard in the same order.
- Plus: `AnyTarget`, `ReturnToHand`, `Exile`, `Scry`, `DestroyNoRegen`
  (destroy + the instruction-scoped "can't be regenerated" rider
  [CR#701.19c]), `Unless` (English order over `MustPay`; parser emitter
  retargeted), `PreventNext`/`PreventAll` (render names over the Prevention
  class), `PumpThisUntilEot`, quantity helpers.
- **Ability words are render metadata** (`ability_word:` field on the
  ability), NEVER macros — no empty-macro-per-ability-word tier.

## Declarative-subject parser production

Player verbs whose `who ≠ You` render subject-first ("Each opponent mills two
cards", "Target player discards a card"). Exactly ONE new parser production
covers the declarative-subject bucket (mill/discard/edict family, ~1,459+
faces on mill alone): parse the subject phrase into `who`, hand the verb
phrase to the existing macro-template path. No per-verb parser arms.

## Done

- Every wave macro registered with template + typed params + acceptance card;
  ward-X, kicker, and a saga are engine-executed in tests (not just
  graduated).
- The declarative-subject production lands; graduation strictly rises (record
  the before/after counts in completion notes).
- Renderer output for the wave passes the fidelity gate.

## Verification

- `cargo test --workspace` green; `cargo xtask validate` clean.
- `cargo xtask graduate` (or the pipeline's graduation run) — count strictly
  greater than before this ticket.
- `cargo xtask fidelity` green on canon; `cargo xtask elaborate --lock`
  re-blessed only for genuinely new cards.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
