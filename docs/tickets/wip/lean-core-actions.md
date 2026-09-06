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

## Implementation progress

Implemented in `zonrmskl`: combat and attachment updates, turn/part insertion,
and shared token/emblem creation. The refreshed partial tree passed the full
`lean/scripts/build` with warnings fatal (74 jobs), including 11 new
ActionFamilies proofs.

Implemented in `ymmuznru`: optional-destination movement, ability-location
correction, and `clearDamage`. The present-destination branch preserves its
checks; the absent branch records no arrival zone and forbids arrival riders.
Ability bindings retain their location and copy origin through movement,
element selection and unions. An exiled ability remains an ability reference
but cannot be countered/copied as a stack object. Moving `thisAbility` also
preserves its ability identity. The clear-damage primitive checks an object on
the battlefield, admits noncreature permanents with marked damage [CR#120.6],
publishes its subject, and adds no damage outcome.

The ticket expressly authorizes retiring the false blanket movement law:
`badAbilityMovedToAZone` is re-spelled as `okAbilityMovedToAZone`, changing its
former `[movable]` result to acceptance. No other existing refusal assertion
changes. Thirty-five surviving theorem statements and two authoring rejection
examples are re-spelled; three of those theorems retain definitional-equality
checks against the new optional-destination shape. Twenty-one new theorems
cover movement publication, reference restrictions, absent-destination rider
checks, origin/location preservation, and marked-damage removal. The new
Marked Damage glossary entry and the movement section of `lean/CONTRACTS.md`
record these meanings; `cr-citations.lock` registers the verified damage rule.

The final `ymmuznru` tree passes `lean/scripts/build` with warnings fatal
(74 jobs). A source audit found no lost existing assertions other than the
explicitly retired false ability-movement law above. Citation validation has
zero noncompliant strings and zero stale rules; ten changed citation sites
were checked against their text. `cargo xtask gate --changed` reports no
affected Rust crates. Refresh before verification was a no-op. The first full
movement gate also passed; it was rerun after review added the `thisAbility`
identity case. English coverage remains the unchanged 20,254 identities; no
parser coverage or runtime execution claim is made.

### Outstanding contract decisions

- **Counter macro:** the mixed spell/ability target needs conditional movement.
  Existing `doIf` hides its condition's mentions, as pinned by
  `Description.badLeadingConditionAntecedent`; putting the target in that
  condition loses its later reference. `counterSpell` remains until the scope
  decision is made. The independent movement/ability-zone correction is done.
- **Fight:** current source admissibility differs from own-power damage.
  `Damage.okFightLand` accepts its land operand, while `badFightPermanent`
  requires a deed-noun refusal. A naive damage expansion changes those results
  and needs a scope for each fighter's own-power read.
- **Regeneration:** the direct instruction and a replacement expansion differ
  in introduced bindings and refusal multiplicity. The graveyard negative
  requires one `zoneIs battlefield` refusal; a naive expansion adds `zoneFits`.
  The independent clear-marked-damage primitive is done.
- **Counter removal:** `loseCounters` reads its player before its amount;
  `removeCounters` reads quantity before holder. A player mentioned in the
  first operand can currently supply the amount's life-total read. The
  removed-counter outcome is authorized, but losing that operand scope is not.

No existing scope law was changed to force a fold. The proposed scope split
would park those four macro designs and land the implemented families; that
proposal has not been adopted. This ticket remains incomplete.
