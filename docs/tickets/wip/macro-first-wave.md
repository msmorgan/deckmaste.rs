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

## Completion notes

- Graduation 5707 → 5878 (+171), wipe-first wizards regen. The
  declarative-subject production graduated 152 player-verb faces (30
  targeted mills among them); named-`cost:` Ward re-emission kept all 14
  ward cards; 84 graduated cards now carry `ability_word:` metadata; the
  48 `Unless(...)` faces re-read through the new macro.
- Ward re-spelled to the blessed `MustPay` triggered home; the engine toll
  now bills the TARGETING player ([CR#702.21a] "that player") — the old
  warded-controller default was wrong and its test flipped. Ward-{X} rides
  the new `TriggeredAbility::where_x` (+ `Endophora::where_x` →
  `price_variable_cost`), engine-executed in
  `stack::ward_x_prices_where_x_at_toll_resolution` (the Minthara shape;
  the full canon Minthara card is deferred — its experience-trigger and
  anthem lines need two unrelated render surfaces).
- Kicker engine slice: `AnnounceOptionalCosts` ([CR#601.2b] YesNo walk,
  repeatable rows re-offer), announced components folded into the payment
  ([CR#601.2f]), record promoted onto the stack entry, `PaidCost` /
  `TimesPaid` read it at resolution. Post-resolution rechecks (ETB "if it
  was kicked") stay engine-alt-costs follow-up.
- Chapter/Saga: `Chapter` macro over `OneOrMore`+`Crossed` ([CR#714.2b]),
  `LoreCounter` decl, Saga registry row confers the enters-with `Also`
  [CR#714.3a] + turn-based increment [CR#714.3c] (execution stage 3). The
  final-chapter sacrifice SBA is spelled CARD-side ([CR#714.4,714.2d] — a
  generic row cannot name a card's own final chapter number; the Idris
  History of Benalia exemplar), deviating from this ticket's bullet.
- Mill stayed the core `PlayerAction::Mill` primitive (already landed +
  engine-executed); the wave ships third-person verb TEMPLATES
  (`Mills`/`Draws`/`Discards`/`LosesLife`/`GainsLife`, kind
  `PlayerAction`) that the ONE production matches. The edict family
  ("each player sacrifices …") and second-person "you"-subject forms are
  NOT covered (subject vocabulary: target player/opponent, each
  player/opponent).
- Parse-side template codecs: plural (`${n:card|cards}`) and sign-strip
  (`${0:+}`) now match in `TemplateIndex` (`read_slot`), twinning the
  render codecs; plural renders oracle-style words ("a card",
  "three cards").
- Tribal Flames waiver CLEARED (waiver inventory now empty):
  `ability_word` render metadata ([CR#207.2c]) on all four ability
  structs + `Characteristic::BasicLandTypes` ([CR#205.3i]) fixing
  Domain's axis.
- `Effect::Unless` deleted from core (Idris never had it); `Unless` is a
  macro over `MustPay`, Echo/CumulativeUpkeep inlined, r2_audit re-blessed
  (path-spelling-only drift), engine toll mana now payable mid-resolution
  (`WorkItem::TollMana`).
- `DestroyNoRegen` rider = `Until(ForThisEvent, [Cant(Regenerate(...))])`
  over the new `DeonticAction::Regenerate` (mirrors Idris
  `Relation.Regenerate`); replacement-pipeline enforcement is a seam.
  `Exile`, `PreventNext`, `PreventAll` land as render names; ReturnToHand
  already lives on core `Action::ReturnToHand`, so no macro (a macro would
  shadow nothing — the name is taken by core).
- cards.elab.lock: +5 new rows (Tome Scour, Kicker Charm, Multikicker
  Chant, Test Saga, Ward X Creature), 2 moved (Tribal Flames, Ward
  Creature) — genuine IR changes.
