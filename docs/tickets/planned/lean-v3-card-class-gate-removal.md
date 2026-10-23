---
needs: []
---
# Remove the keyword card-class gate (a card-authoring mistake is not a text-meaning error)

Standard constraints apply. Lean first (`lean/Semantics/`). Carries ruling #2, Option A, of the
owner's batch 4 (2026-10-22), which supersedes the 2026-08-22 ruling that kept the gate.

## Ruling (owner, 2026-10-22, batch 4)

Placing a keyword on the wrong card class (flying on an instant, flashback on a creature) is a
card-authoring mistake, not a text-meaning error, so the checker does not refuse it. Drop
`onPermanentCard`, `onInstantOrSorceryCard` and `keywordCardOk`. The ruling also says to remove
ascend's hand-written card-class guard; that part is a STOP, below.

## Verified (2026-10-22, this workspace; paths under `lean/Semantics/`)

- `keywordCardOk`: `Check/Card.lean:29-31`, reached from `classAbilityOk`'s keyword arm
  (`Check/Card.lean:70`, `| _ + 1, .keyword k _ _ => keywordCardOk c k`) and so from
  `cardTextOk` (`:80`) and `Card.check` (`:286`, refusal `.cardText`).
- `onPermanentCard` and `onInstantOrSorceryCard` columns: `Check/FactTypes.lean:70-71`; rows in
  `Check/Facts.lean` (67 rows set `onInstantOrSorceryCard := true`; 12 set `onPermanentCard :=
  false`: flashback, fuse, overload, cleave, harmonize, awaken, buyback, aftermath, cipher, epic,
  jumpStart, rebound). `lean/CONTRACTS.md:967`
  (and its section "A keyword definition split by card class", `:938`) describe them.
- Unrelated namesake: `Proofs/KeywordByCardClass.lean:19` defines `onPermanentCard : Condition`
  ("if this is a permanent card"), the class-guard example of the flying pin. It is not the
  column; keep it.
- `StaticSpec.onInstantOrSorceryCardOk` (`Check/Card.lean:45`), and the `.spell`, `.activated`,
  `.static`, `.mayBeginOnBattlefield` arms of `classAbilityOk`, are **not** keyword gates and
  stay: they classify spell and static abilities, which functions-in-which-zone rules govern
  [CR#113.6]. `lean-v3-checker-runtime-state-audit` classes them.

## The six pins that assert the gate: delete them

Each is a `Card.check … = [.cardText]` refusal pin in `Proofs/Faces.lean`. **Delete all six; do
not rewrite them to accept.** With the gate gone, a pin that a sorcery with flying checks clean
asserts nothing wanted (CLAUDE.md "Assurance": a test that pins an unwanted behaviour is retired).
The refusal each asserted is no longer a property of the checker, so there is nothing left to
assert.

| Pin (`Proofs/Faces.lean`) | Line | What it asserted |
|---|---|---|
| `badKeywordOnInstant` | 118 | flying on an instant is refused |
| `badProtectionOnInstant` | 222 | protection from red on an instant is refused |
| `badEquipOnSorcery` | 249 | equip {2} on a sorcery is refused |
| `badCumulativeUpkeepOnSpell` | 499 | cumulative upkeep {2} on a sorcery is refused |
| `badUnearthOnSpellCard` | 565 | unearth {B} on an instant is refused |
| `badFlashbackOnPermanentCard` | 573 | flashback {2}{U} on an artifact is refused |

Do not add pins that the gate is gone (no "flying on an instant checks clean"). Note for
`lean-v3-facts-from-declarations`: its Step 1 says "No Lean pin asserts the keyword card-class gate
negatively" and plans to add pins that the gate is gone. Both are wrong: the six pins above assert
it negatively, and the owner's ruling is that they are deleted, not flipped.

Keep, as printed-card witnesses and not as gate pins: `okSpellKeywordsOnPermanentCards` (split
second on a creature, replicate on a creature) and `okPermanentKeywordsOnSpellCards`
(web-slinging on an instant, *Puncture Blast*'s wither on an instant) in
`Proofs/KeywordByCardClass.lean:69-77`. Reword their section docstring (`:61-65`, "A keyword's
card-class columns admit each class…"), since the columns are gone; keep the [CR#702.80c] note on
wither.

## Change

1. Delete `keywordCardOk` (`Check/Card.lean:29-31`); `classAbilityOk`'s keyword arm (`:70`)
   becomes `true`. Drop `onPermanentCard` and `onInstantOrSorceryCard` from `KeywordFacts`
   (`Check/FactTypes.lean:70-71`) and from every `keywordFacts` row (`Check/Facts.lean`).
2. Delete the six pins above.
3. `lean/CONTRACTS.md`: delete the `keywordCardOk` bullet (about `:967`) and retitle the section
   "A keyword definition split by card class" (`:938`) for what remains, the guarded
   halves (ascend).
4. `Proofs/KeywordByCardClass.lean`: the header docstring (`:1-13`) says ascend keeps its guards
   "until the card-class gate of `lean-v3-facts-from-declarations` decides them"; point it at this
   ticket's STOP.
5. ADR `semantics-v3.md` §0.1 item 5 records the observation-not-rule character of the columns;
   cite this ruling there if it states the gate.

## STOP: ascend's card-class guard (awaiting the owner)

The ruling says "remove ascend's hand-written card-class guard". The implementer does **not** act
on that sentence; this item waits for the owner.

- By [CR#702.131a,702.131b], ascend means a **spell ability** on an instant or sorcery and a
  **static ability** on a permanent.
- The definition is `Definitions/Keywords.lean:240-258`: the static half is `onlyIfSo (.whileHolds
  … (gainDesignation .you citysBlessing restOfGame)) (is this_ isPermanentType)`, the spell half
  is `doIf (is this_ isInstantOrSorcery) (…)`.
- Without the guard, the static half on an instant functions while the instant is on the stack
  [CR#113.6], so the city's blessing comes even if the spell is countered. That is a different
  game from [CR#702.131a].
- So the guard is part of the keyword's **meaning** (which half applies to which class), not a
  refusal. It is not a gate in the sense of the ruling: removing `keywordCardOk` leaves it intact.
- **Proposed:** keep a class-dependent ascend definition (the two halves stay guarded by class), as
  the earlier landing `lean-v3-keyword-class-guards-and-observation` did (its ruling 3b kept
  ascend's guards).

Pins that stay while this is open: `okEachHalfChecksClassBlind`, `okBothHalvesOnPermanentCard`,
`okBothHalvesOnInstantCard` (`Proofs/KeywordByCardClass.lean`).

## Coordination

- **`lean-v3-facts-from-declarations` (WIP, owned elsewhere), Step 1 item 5**: its uncommitted work
  already drops the two columns and `keywordCardOk`. This ticket and that step land together or in
  sequence; whichever lands second reconciles. If that step lands first, this ticket reduces to
  the six deletions, the CONTRACTS/docstring edits and the ascend STOP. Either way the six pins are
  deleted, not flipped, and that ticket's Step 1 test bullet "Add card-class pins that the gate is
  gone" is not carried out.
- `needs` is empty: the work does not wait on that ticket.
- `lean-v3-checker-runtime-state-audit` (§5 line "keyword card-class gates … Removing them agrees
  with the boundary") and `lean-v3-proofs-and-docs-alignment` mention the gates; this ticket is the
  removal they point at.
- Later tickets that delete `Check/Facts.lean` (`lean-v3-keyword-facts-deleted`) assume this one
  landed or that step 1 did.

## Tests

- Retired: the six pins named above, each for the reason "it asserted a card-authoring gate the
  owner removed".
- Kept: `okSpellKeywordsOnPermanentCards`, `okPermanentKeywordsOnSpellCards` (printed witnesses);
  `DrtCards.lean:52-58` and `Faces.lean:52-57` (spell and static class rules, not keyword gates).
- Gates: `lean/scripts/build` (`lake build --wfail`), the CR citation gates. `cargo xtask lean
  check` only if plugin definitions changed (none are expected).
