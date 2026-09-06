---
needs: [lean-core-bindings-and-composition]
---
# Share event descriptions and remove specialized static wrappers

Unify the event and applicability families while retaining the scopes their
consumers observe. Apply the
[campaign contract](lean-core-bindings-and-composition.md#campaign-contract).
These decisions were agreed on 2026-09-06.

## Events and action provenance

- Replace `GameEvent.dies`, `leaves`, `enters`, and `putInto` with macros over
  a zone-transition pattern. Preserve origin/destination constraints and
  whether the subject is observed before or after the transition. Optional
  endpoints must not permit meaningless combinations silently.
- Share the damage-event pattern underlying `isDealtDamage` and
  `dealsDamage`. Active/passive wording and textual argument order belong to
  macros, not a semantic voice field. Preserve damage roles and binding order
  as exposed by those macros.
- Share action descriptors for `castBy`, `castFrom`, `wasCast`, and the
  corresponding activation reads. Keep explicit distinctions between a
  currently proposed action, the action associated with the current object
  incarnation, and a historical occurrence. Share actor, origin, and rank
  constraints without reducing everything to `happenedTo`. Cost modifiers
  can inspect a proposed cast; a permanent can retain permitted cast
  provenance across its arrival; a copied decision is not a new action.
  Preserve uniqueness from ranked reads and rank the intended actor's action
  stream before restricting it to the described subject.

## Static specs

- Fold `entryChoice` and `attachmentChoice` into macros over an instruction
  performed as an event occurs. Preserve the original event and publication
  of the declared choice across ability lines. An ordinary replacement whose
  body only chooses would lose those obligations. Put disclosure on the
  choice itself. Sanctuary Blade and Psychic Paper distinguish the attachment
  occasion and subsequent use of the choice.
- Express turn-position applicability through ordinary conditions, eliminating
  the separate `partScope` wrapper in favor of shared conditional
  applicability. Preserve whose turn/part is being described.
- Expand `offBattlefieldScope` into explicit affected selections. Preserve
  the distinction between control on the battlefield and ownership in the
  relevant other zones; do not mechanically reuse "you control" for every
  domain. Arcane Adaptation and Encroaching Mycosynth are witnesses.
- Remove `retention` as an arbitrary `StaticSpec` wrapper. It qualifies the
  attachment consequences of one protection instance and the appropriate
  excluded attachments. It must not weaken that instance's damage, targeting,
  or blocking consequences, or any other instance of protection. Preserve
  the already-attached selection at the relevant time; Benevolent Blessing
  and Black Ward exercise the distinction.

Keep `establish`. Scoped numeric definitions are owned by the prerequisite;
ability-removal edits are owned by the characteristic ticket. These are
different obligations, not four cases of a generic `scoped` wrapper.

## Completion evidence

Re-spell affected event, provenance, protection, and as-event-choice witnesses.
Check transition observation points, both damage phrasings, proposed versus
associated versus historical actions, linked choice publication, explicit
owner/controller domains, and protection-instance isolation. Run
`lean/scripts/build`. Standard constraints apply.
