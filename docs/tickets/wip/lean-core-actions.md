---
needs: []
---
# Expand action macros over shared operations

Retire the concrete action special cases using the existing composition,
replacement, and payload vocabulary plus the local operation folds below. Apply the
[campaign contract](../done/lean-core-bindings-and-composition.md#campaign-contract).
Scope narrowed by the user on 2026-09-06. This ticket does not depend on the
other planned folds or the parked query, binding, and event mechanisms.

## Action families

- Make `fight` a normal keyword-action macro whose two damage instructions
  use the two subjects' own powers and compose simultaneously. Preserve the
  required subject checks through the expansion. The shared simultaneous
  constructor already has a place beside sequential composition; no generic
  transaction, scheduling, rollback, or feasibility subsystem is requested.
- Share writable combat and attachment operations. The writable vocabulary
  is smaller than the query `CombatRelation`, which also contains historical,
  inverse, and could-block relations. Fold `becomeAttacking`,
  `becomeBlocking`, `stopBlocking`, and `removeFromCombat` through participation
  and relation updates, preserving blockedness independently of blocking
  edges and the admitted attacked planeswalker/battle subjects. Fold
  `attachTo`/`unattach` through explicit relation updates without requiring an
  author to supply an unknown old host. Use a small write vocabulary over
  the existing `AttachMove` and combat participation state; do not build a
  relation model from the write side. Relation queries are parked in
  [relation queries](../maybe/lean-core-relation-queries.md) and are not a
  prerequisite.
- Generalize zone movement/exit so a destination can be absent where that
  has meaning. The Counter macro retains its label and expands a spell to
  its owner's graveyard, or removes an ability from the stack without a
  destination [CR#701.6a]. Correct the blanket prohibition on ability zone
  changes: ending a turn or combat exiles all stack objects [CR#724.1b,724.2b].
  Re-spell the affected refusal pins rather than preserving that false law.
- Share fresh-object creation across typed token and emblem specifications,
  with a common count and player and uniform publication of created objects.
  Preserve the specifications' distinct admissible contents and destinations:
  tokens enter the battlefield; emblems are created in the command zone.
  `create` and `getEmblem` become authoring macros over this family. Reuse
  existing characteristic payloads; the characteristic-edit redesign is not
  required to group the creation forms.
- Expand regeneration through the existing replacement machinery plus an
  explicit clear-marked-damage operation. The replacement clears damage,
  taps the permanent, and removes it from combat when applicable. A resolving
  shield covers the next destruction this turn; a static replacement covers
  each destruction. Label the actual regeneration replacement when it applies,
  not the installation of a shield [CR#701.19a..701.19c].
- Fold `addTurn` and `addPart` into one insertion family for turns, phases,
  and steps, with player and relative placement. An inserted turn publishes
  the reference needed by "that turn". Preserve the placement constraints;
  the syntax ticket does not design the execution queue.

## Counter removal

Fold `loseCounters` into a macro over `removeCounters`, which already admits
players as well as objects. Preserve argument/reference scope and expose the
same removed-counter result independently of the printed verb. "All" and
an exact amount use the shared quantity vocabulary. Suncleanser witnesses
the two surface forms.

## Explicit limits on reduction

**Retain the existing cost-symbol constructors and payloads for now.** The
user deferred their expansion on 2026-09-06 to avoid introducing unnecessary
machinery. The design is parked in
[cost-symbol expansion](../maybe/lean-core-cost-symbol-expansion.md), which
does not block this ticket. Property propagation is not a delivery here.

**Retain `Instruction.draw`, its Core Deed, and its event.** The draw macro
expands to that primitive. Library-to-hand movement does not constitute a
draw [CR#121.5], and a label cannot supply the missing body. No generic
request/attempt framework is part of this campaign merely to remove draw.

Retain the existing Room/designation and exchange shapes. Their possible
reductions are parked in [Room declarations](../maybe/lean-core-room-declarations.md)
and [exchange operands](../maybe/lean-core-exchange-operands.md). Neither
requires generic feasibility machinery as part of this ticket.

Retain `turnOver` as the lower face-switching operation. The predicate ticket
replaces `isTransformed` with a current-face read; neither change folds
front/back into physical `Status`.

## Completion evidence and overlap

Re-spell affected macros, cards, and positive/negative pins through the new
operations. Verify the expansions themselves, their required refusals, and
published bindings; accepted card syntax is not proof of runtime execution.
Run `lean/scripts/build`. Coordinate overlapping edits to shared inductives
and checker traversals; independent claimability does not imply disjoint files.
Standard constraints apply.

This is the Lean pass. The completed
[Rust fight decomposition](../done/core-fight-primitive-to-macro.md) and the
planned [runtime designation work](core-getdesignation-scopes-and-eviction.md)
are related implementation history/work, not additional deliveries here.

## Result

Combat and attachment writes, turn/part insertion, and token/emblem creation
share structural operations. Movement accepts an absent destination and keeps
ability identity and location. `clearDamage` removes marked damage without
requiring the permanent to remain a creature.

Fight, Counter, regeneration, and losing counters are registered macros. Fight
captures both subjects before its whole-event guard and simultaneous own-power
damage. Regeneration distinguishes a single-use shield from its application;
Clergy of the Holy Nimbus uses the repeated application. Counter distinguishes
spell movement from ability exit. Losing counters keeps the player-before-amount
scope and publishes the shared removed-counter result.

The user clarified that this is a prototype: existing checker behavior and pins
are not compatibility contracts. The earlier proposed deferral was rejected.
The implementation corrects obsolete checks and documents those corrections.

## Landing record

### PROVE

Measured on `nuxvpwpu`, English lock covered count **20,254**. The implementation
stack also includes `zonrmskl` (operation families) and `ymmuznru` (movement and
marked damage).

- Full Lean gate: **74 jobs passed**, warnings fatal, after the final refresh.
  The focused capture/macro checks also pass (32 jobs), including the refreshed
  Caustic Bronco witness.
- Rust closure: `cargo xtask gate --changed` reports **no affected workspace
  crates**. This change touches Lean, its documentation, and the citation lock.
- All **815** existing `Spelled` card declarations remain. No parser identity
  was added or removed; parser construction, lexical-ownership, roundtrip, tie,
  and licensing-checker measurements were not rerun for this Lean-only change.
- Named theorem inventory: **1,539 → 1,597**. **58 added**, **95 re-spelled or
  corrected** (including four renamed witnesses), **0 restored**, **0 ignored**,
  and **0 assertions removed**. All four retired names have replacements below.
  Two authoring rejection examples also use the new movement shape.
- Citation validation: **0 noncompliant, 0 stale**. The combined feature diff has
  **18 changed citation sites**, read against the local rules text. No new
  checker guard names a card or surface lexeme; action labels occur in trusted
  macro expansions.

### DISCLOSE

The four renamed witnesses deliberately correct prior behavior:

- `Zone.badAbilityMovedToAZone` → `okAbilityMovedToAZone`: a stack ability can
  be exiled; blanket non-movability was false.
- `Anaphora.okStackAnaphorOnAbility` → `badStackAnaphorAfterCounteringAbility`:
  countering removes the ability from the stack, so the following stack read
  must fail.
- `Damage.badFightPermanent` → `okFightPermanentUnderCreatureGuard`: fight's
  creature guard replaces the unrelated attacking-creature refusal.
- `Anaphora.badOwnEmptyDelta` → `okOwnPowerWithoutNewMention`: an explicit
  source can read its own power without inventing a pronoun in an empty window.

Seven other existing negative witnesses remain negative with diagnostics from
the shared operations: `Keyword.badCounterPermanent`,
`Keyword.badCounterJoinedPlayer`, `Damage.badFightGroup`,
`Damage.badFightGraveyard`, `Zone.badRegenerateInGraveyard`,
`Zone.badRegenerateBareThis`, and `Anaphora.badOwnTwoInDelta`. Expanded bodies
may report the same violated obligation at several constituent operations.

Deviations and additions:

- A local operand binder (`withOperands` / `Window.operand`) was necessary to
  capture references once in their actual context. An empty-context pattern
  cannot safely capture the first fighter across a later target that reuses X.
  This does not implement the parked collection-member/aggregation mechanism.
- The checker uses private frames and hidden slots to keep aliases together
  when ordinary discourse forgets a reference. Nested frames, equal-looking
  distinct targets, movement through aliases, conditional forgetting, and
  explicit resolved-permanent views have regression witnesses. The shuffle
  cases are synthetic checker stress tests, not additional card work.
- Conditional alternatives keep the primary clause's pre-state and stated
  numeric magnitude, not its executed movement or damage event. Branch-local
  mentions stay local; common facts about earlier mentions are joined. Ertai's
  Trickery and Caustic Bronco exercise the two sides of this distinction.
- Marked Damage and Regeneration glossary entries and `lean/CONTRACTS.md`
  document the meanings. The general programming notion of a lexical operand
  is documented in the workbench contract rather than added to the game glossary.
- Review found three capture bugs (alias splitting after filtering, loss of a
  frame at a conditional join, and a discarded resolved-permanent view). All
  three were fixed with direct regression witnesses before landing.

### REPORT

Core `Instruction`: **70 → 62**. The added operation-family payloads are
`CombatParticipation` (**3** constructors), `CombatUpdate` (**2**), and
`CreationSpec` (**2**). `Window` is **5 → 6**. Checker-private `Payload` is
**10 → 12** for the lexical frame and hidden-reference wrapper. These helper
counts are separate from the Instruction reduction. Cost symbols, draw, Room
operations, exchange, and `turnOver` retain their existing shapes.

The unchanged English coverage lock contains **20,254** identities. Parser
selection census, construction count, homograph/form-overlap inventories, and
coverage-command performance were not remeasured; no parser throughput or
runtime execution claim is made. Lean compilation and checked card syntax are
not a proof that an engine executes the expansions correctly. The expansions
were reviewed directly against the cited rules.
