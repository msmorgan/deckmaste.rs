---
needs: [lean-core-bindings-and-composition, lean-core-relations-and-results, lean-core-characteristics-and-keyword-arguments, lean-core-events-and-static-specs]
---
# Expand action macros over shared operations

Retire the agreed action special cases after their shared
structures exist. Apply the
[campaign contract](lean-core-bindings-and-composition.md#campaign-contract).
These decisions, including the exceptions below, were agreed on 2026-09-06.
The prerequisites are deliberate: this is the consumer pass over the new
binding, query, edit, and event vocabulary.

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
  author to supply an unknown old host.
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
  `create` and `getEmblem` become authoring macros over this family.
- Expand unlock/lock through a typed selection of a half and granting/removing
  its declared designation. Add the missing removal and selected-half
  vocabulary; legality comes from declarations rather than named-door checks.
  Declaration constraints must preserve the holder, uniqueness, and lifetime
  distinctions needed by the designation operations.
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

**Limit the exchange reduction to its demonstrated operand overlap.** Fold
`Exchanged.lifeTotals` into numerical operands where the shared selections
preserve "two target players" as well as two explicit values. Mirror Universe
and Soul Conduit exercise two life totals; Tree of Redemption exercises life
versus toughness. Keep Exchange as a labelled macro over a joint semantic
body. Do not promise that its outer constructor disappears, rename the same
family and claim a reduction, or add a generic "whole group can occur"
expression. Other exchange payloads remain unless the accepted structures
demonstrate an equivalent smaller body. The unresolved "or base toughness"
wording remains with [workbench-last-shapes](workbench-last-shapes.md).

Retain `turnOver` as the lower face-switching operation. The query ticket
replaces `isTransformed` with a current-face read; neither change folds
front/back into physical `Status`.

## Completion evidence and overlap

Re-spell affected macros, cards, and positive/negative pins through the new
operations. Verify the expansions themselves, their required refusals, and
published bindings; accepted card syntax is not proof of runtime execution.
Run `lean/scripts/build` and declaration/facts consistency checks for changed
registry properties. Standard constraints apply.

This is the Lean pass. The completed
[Rust fight decomposition](../done/core-fight-primitive-to-macro.md) and the
planned [runtime designation work](core-getdesignation-scopes-and-eviction.md)
are related implementation history/work, not additional deliveries here.
