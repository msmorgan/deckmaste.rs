---
needs: [semantics-v3-peer-files-and-toposort]
---
# "Regenerate target creature" publishes its target to the following text

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against the tree at `wmyykqnv`. Paths are under `lean/Semantics/`.

## The defect

*Debt of Loyalty* (Vintage-legal) reads "Regenerate target creature. You gain control of that
creature if it regenerates this way." It cannot be written. A probe on 2026-10-25 of the
spelling below returned `[anaphor …, anaphor …, thisWayUngrounded, anaphor (word creature) …]`.

`regenerate` (`Definitions/Actions.lean:587-594`) is a shield: it establishes a single-use
replacement of the subject's destruction [CR#701.19a]. The subject is mentioned only inside the
replacement's `would` pattern, which the checker steps as a hypothetical box, so its target never
reaches the following text. The pin `regenerationPatternDoesNotLeakIntoFollowingText`
(`Proofs/BindingIdentity.lean:268-269`) asserts that `regenerateBy (target isCreature)` publishes
no target. That pin asserts the defect.

A target is chosen as the spell is cast [CR#601.2c,115.1], not when the shield applies, and
"that creature" names it. The ADR already admits this for conditions: "A referent a condition
introduces … is a hypothetical and never a candidate, though targets the action branch announces
are ([CR#601.2c])" (`docs/decisions/semantics-v3.md` §0.1 item 2, step 2).

## Change

1. When the stepper steps an `establish` whose spec is a `replace` (the shield form), a mention
   with the `target` determiner inside the replacement's `would` pattern is announced in the box
   enclosing the `establish`, with its `.target` stamp, so later text reads it by D′ ("it") and
   AC>R1 ("that creature"). Non-target mentions in the pattern stay hypothetical. Add no AST node,
   field or variant ([CR#701.19a] is unchanged; §0.1 item 5(c)).
2. Spell *Debt of Loyalty* in `Cards/` as a `spelled` card:
   ```lean
   .triggerReflexively (regenerate (target isCreature))
     (some (does any (regeneration it)))
     (gainControl (that creature))
   ```
   following *Matopi Golem*'s reflexive "When it regenerates this way" (`Cards/Counters.lean:714`,
   pinned by `regenerationEventDescribesTheApplicationRatherThanTheShield`,
   `Proofs/ActionFamilies.lean:298-312`). If `triggerReflexively` is not the form for a
   postposed "if it regenerates this way" on a spell, use the form of the "If it regenerates this way, draw a card" fixture at
   `Proofs/Turn.lean:229` and say which.

## What must not change

- `regenerate` stays a shield with `.ownScope none`; only `regeneration` carries the deed
  (owner ruling 2026-10-25 in `lean-v3-deed-duplicates`; [CR#701.19c]).
- `regenerateTargetsOnce` (`Proofs/BindingIdentity.lean:261-266`): the application still keeps
  the one targeted identity.
- A non-target subject ("Regenerate this creature") publishes nothing new: the conjunct
  `countOnes .object shield = 0` of
  `regenerationEventDescribesTheApplicationRatherThanTheShield` stays.
- Conditions' hypothetical referents stay hypothetical.

## Pins

- Rewritten: `regenerationPatternDoesNotLeakIntoFollowingText` →
  `regeneratedTargetIsReadBack`, asserting `targets (actionResult (DRS.ofList []) (regenerateBy (target isCreature))) = 1`, because the old claim is the defect.
- Added: `debtOfLoyalty` (spelled). Every fixture must be a Vintage-legal card (owner,
  2026-10-25), so no invented "Regenerate target creature. Tap it." pin is added.
- Must stay green: `regenerateTargetsOnce`, `okRegenerationApplication`
  (`Proofs/BindingIdentity.lean:272`), `regenerationEventDescribesTheApplicationRatherThanTheShield`,
  `matopiGolem`, `modifyingProhibitionReadsOnlyThePrecedingActionsObjects`
  (`Proofs/Deontic.lean:631-648`).

## Acceptance

`./lean/scripts/build` (`--wfail`), `cargo xtask lean check`, the CR citation gates.
