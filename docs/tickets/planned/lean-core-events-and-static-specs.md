---
needs: []
---
# Fold event patterns and static applicability forms

Implement the concrete event and static-spec folds below, preserving their
existing scopes. Apply the
[campaign contract](lean-core-bindings-and-composition.md#campaign-contract).
Scope narrowed by the user on 2026-09-06; no new provenance, relation-query,
or general as-event instruction mechanism is a prerequisite.

## Event patterns

- Replace `GameEvent.dies`, `leaves`, `enters`, and `putInto` with macros over
  a zone-transition pattern. Preserve origin/destination constraints and
  whether the subject is observed before or after the transition. Optional
  endpoints must not silently admit meaningless combinations.
- Share the damage-event pattern underlying `isDealtDamage` and `dealsDamage`.
  Active/passive wording and textual argument order belong to macros, not a
  semantic voice field. Preserve damage roles and the binding order exposed
  by those macros.

## Static specs

- Fold `entryChoice` and `attachmentChoice` into a single choice spec with
  an entry/attachment occasion enum, subject, choice sort/domain, and
  disclosure where applicable. Preserve the original event and the current
  publication of declared choices across ability lines. Keep the two
  authoring forms as macros. This is an occasion-enum fold, not a general
  construct for performing arbitrary instructions as events occur.
- Express turn-position applicability through ordinary conditions, eliminating
  `partScope` in favor of shared conditional applicability. Preserve whose
  turn/part is described.
- Expand `offBattlefieldScope` into explicit affected selections using the
  existing noun/predicate vocabulary. Preserve control on the battlefield
  versus ownership in the relevant other zones. Arcane Adaptation and
  Encroaching Mycosynth are witnesses; a general relation-query family is
  unnecessary for these explicit selections.

Keep `establish`. Numeric definition folding belongs to the composition ticket;
ability-removal edits belong to the characteristic ticket. Those are ownership
boundaries, not prerequisites for this ticket's changes.

## Completion evidence and deferred work

Re-spell affected event and static witnesses. Check transition observation
points, both damage phrasings, entry/attachment occasion and linked choice
publication (including Sanctuary Blade and Psychic Paper), and explicit
owner/controller domains. Run `lean/scripts/build`. Standard constraints apply.

Leave the current provenance reads and protection retention in place.
[Action provenance](../maybe/lean-core-action-provenance.md),
[protection retention](../maybe/lean-core-protection-retention.md), and the
[general as-event instruction construct](../maybe/lean-core-as-event-instructions.md)
are separate design tickets. The latter does not block the occasion-enum fold.
