---
needs: [lean-v3-this-way-lookback-grounding]
---
# `.procedure` records its own deed, not "cast", for every procedure

Standard constraints apply. Found landing `lean-v3-keyword-class-guards-and-observation`.

## Problem

The `.procedure` step in `lean/Semantics/Check/Phrase.lean` (`| .procedure _ subj pay res from_ of_`,
near `:3606`) ends with
`applyZoneMove drs5 refIds (some (.action "cast")) (some .stack)` in the post-event phase. The
procedure kind (first argument, ignored as `_`) is not read, so every procedure records a stack move
whose deed is "cast". VERIFIED 2026-10-22:

- `cast` (`Definitions/Actions.lean:43`): correct.
- `resolve` (`Definitions/Actions.lean:76`, added 2026-10-22): `resolve` is not a move to the stack
  at all; it records a "cast" stack move for the resolving spell.
- `activate` (`Definitions/Actions.lean:57`): records the same "cast" deed; an activation puts an
  ability on the stack but is not a cast [CR#602.2], and the object is an ability, not a card.

## Change

Read the procedure kind. Each procedure records its own deed (`cast`, `activate`), or none where no
move happens (`resolve`). Decide what, if anything, an activation records (the ability on the stack)
and say so. The `needs` entry exists because `lean-v3-this-way-lookback-grounding` reads recorded
deeds; land after it.

## Tests

- `activateRecordsActivateNotCast`, `resolveRecordsNoStackMove`, and `castStillRecordsCast`; a
  `did … (cast which) thisWay` after `activate` or `resolve` is refused or reads nothing, as the
  recorded deed dictates.
