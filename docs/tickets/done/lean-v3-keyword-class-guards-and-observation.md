---
needs: []
---
# Keyword halves without unprinted card-class guards, and observation points by the CR

Standard constraints apply. Provenance: rows A3 and A4 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md). One ticket,
because both edit the haunt definition.

## Rulings (owner, 2026-10-22)

**A3.** Remove the unprinted card-class guard conditions (`is this_ isPermanentType`,
`is this_ isInstantOrSorcery`) from gift, ascend and haunt (**ascend's are kept**, 3b below). Each half is already scoped by its
ability category and event [CR#113.3a,113.6,603.4,608.2n,702.55a,702.131a,702.131b,702.174b].
This re-rules the 2026-10-06 convention (`semantics-v2.md` §7) that `lean/CONTRACTS.md`
"A keyword definition split by card class" records. It interacts with
[lean-v3-facts-from-declarations](../wip/lean-v3-facts-from-declarations.md) (see "Other tickets").

**A4.** Remove the hand-set departure observation points from haunt, buyback and rebound
[CR#603.10a]. A spell put into a graveyard from the stack is none of the look-back zone changes
[CR#603.10a].

## Verified (2026-10-22)

Paths are under `lean/Semantics/`.

**A3 guards** (`Definitions/Keywords.lean`):

- Ascend `:239-257`: the static half is `onlyIfSo (.whileHolds …) (is this_ isPermanentType)`
  (`:241-248`), the spell half `doIf (is this_ isInstantOrSorcery) (…)` (`:249-257`).
- Gift `:782-794`: the trigger's `if_ := some (and [is this_ isPermanentType, keywordCostPaid gift
  this_])` (`:786`); the spell half `doIf (is this_ isInstantOrSorcery) (doIf (keywordCostPaid …)
  …)` (`:789`).
- Haunt `:842-853`: `if_ := is this_ isPermanentType` (`:845`) and `if_ := is this_
  isInstantOrSorcery` (`:850`).
- The guard Templates are `isInstantOrSorcery` (`Templates.lean:400`) and `isPermanentType`
  (`:403-405`). `lean/CONTRACTS.md:906-936` names `isPermanentCard` as the Template; the code uses
  `isPermanentType`.
- Out of the ruling: cipher's `doIf (is this_ .isCard)` (`:388`) tests card against copy, not
  class; `resulting := some isInstantOrSorcery` (`:737`, `:973`) is no guard.
- Evaluated during verification: with the guards removed, gift's and haunt's halves check `[]` and
  `cardTextOk` holds on both classes; ascend likewise.

**A4 observation points**: the field is `at_ : ObservationPoint` (`arrival | departure`,
`Abilities.lean:21-22`), the fourth positional argument of raw `Action.move`
(`Abilities.lean:1168-1169`); there is no `observation` field and no default. The computed value
comes from the private Template `zoneChange` (`Templates.lean:1106-1109`), which `putInto`
(`:1112-1114`), `returnTo` (`:1180`) and the battlefield puts use; `Location.observedOnDeparture`
(`Abilities.lean:1407-1424`) is `false` for stack to graveyard. The three sites are buyback
`Definitions/Keywords.lean:357`, haunt `:849` and rebound `:1291`, each
`.move thisSpell stack graveyard .departure none []`. `putInto thisSpell graveyard (from_ := stack)`
produces the same move with `.arrival` (evaluated). The checker reads `at_` only for a battlefield
origin (`Check/Phrase.lean:3204-3210`), so no check result changes. `CONTRACTS.md:1245-1256` keeps
the raw field for mixed origins and needs no change.

## Rulings (owner, 2026-10-22, second batch)

**3a. "During its resolution" composes; no one-off Causing constructor.** Haunt's spell half is
scoped by its event [CR#702.55a,608.2n]: "When this spell is put into a graveyard during its
resolution". A countered haunt spell is put into a graveyard from the stack too, but not during
its resolution [CR#608.2n], so it must not trigger.

- Add `.resolve` to `Procedure` (`Words.lean:34-35`, currently `activate | cast | play`; VERIFY:
  true, but the file:line is `Words.lean:34`, not `~183`). Add a Definition `resolve (subject)`
  beside `cast`/`play`/`activate` (`Definitions/Actions.lean:43-64`) over `Action.procedure`; the
  payment, `from_` and `resulting` arguments do not apply to it, so pass the neutral values. The
  compiler lists every exhaustive match on `Procedure` that must admit the new case.
- Write the qualifier with an existing form over `does any (resolve thisSpell)`: either
  `Causing.event` (`Abilities.lean:518-521`, `source | event (event : GameEvent) | anEffect`; VERIFY:
  corrected, it is `:520`, not `~596`) as `GameEvent.causes (.event …) <move>` on a trigger event and
  `WouldClause.causedBy (.event …) <clause>` on a replacement's would-clause, or `Timing.whileDoing`
  (`Abilities.lean:175`; VERIFY: true) in the timing slot. The implementer chooses, uses the same one
  at all three sites, and states the reason in the landing. Considerations: `Causing.event` states
  the relation [CR#608.2n] gives ("as the final part of … resolution" the spell is put into the
  graveyard, so the resolution causes the move) and reads the same on a trigger's event and on a
  replacement's would-clause; `Timing.whileDoing` is the literal "during" of [CR#702.55a] but is a
  co-occurrence, not a cause, and lives in the timing slot only.
- Apply it to haunt's spell half [CR#702.55a,608.2n], and also to buyback [CR#702.27a] and rebound
  [CR#702.88a]: both replace "putting it into your graveyard as it resolves", but encode a bare
  stack-to-graveyard move, so a countered spell with buyback or rebound wrongly returns to hand or
  is exiled (VERIFY: true; the sites are `Definitions/Keywords.lean:357` buyback and `:1297`
  rebound, `.move thisSpell stack graveyard .departure none []`; rebound is `:1294`, not `~1291`).
- Then remove haunt's guards (`Definitions/Keywords.lean:851`, `:856`). Current haunt spell-half
  event: `:855`.

**3b. KEEP ascend's guards** (`Definitions/Keywords.lean:248`, `:250`). They are resolved together
with the card-class / functioning-zone gate of
[lean-v3-facts-from-declarations](../wip/lean-v3-facts-from-declarations.md)
[CR#113.6,702.131a,702.131b]. Cross-reference only: that ticket is WIP in another workspace and is
not edited here. The old STOP 2 (an unguarded static half would function on a spell on the stack)
is why: it moves to that gate.

**Gift proceeds as ruled** (A3 for gift, unchanged): its permanent half is an entering trigger and
its spell half a spell ability [CR#113.3a,702.174b]. Line numbers have drifted since the
verification above: gift is `:788-801` (guards `:792`, `:795`), haunt `:848-859`.

## Change

1. Gift (`:792`, `:795`): the intervening "if" keeps only `keywordCostPaid gift this_` (the gift
   cost row the audit's seed table kept); the spell half drops the outer `doIf`.
2. Haunt: add `Procedure.resolve` and the `resolve` Definition (3a), write the spell half's event
   as resolution-caused (or `whileDoing`), then drop both guards. Buyback (`:357`) and rebound
   (`:1297`) get the same qualifier on their would-clause. Ascend is left as is (3b).
3. A4: write the three moves with `putInto … (from_ := stack)`, or the Template that fits, so the
   observation point is computed. Buyback's and rebound's moves sit inside replacement clauses
   [CR#702.27a,702.88a]; keep their structure and add the resolution qualifier to them.
4. `lean/CONTRACTS.md:906-936`: rewrite "A keyword definition split by card class". A keyword that
   means different abilities on a permanent and on an instant or sorcery writes both halves
   unguarded when each is scoped by its ability category and its event (gift, haunt; this ruling,
   dated); ascend's guards stay until the gate of `lean-v3-facts-from-declarations` decides them.
   Drop the `isPermanentCard` / `isInstantOrSorcery` Template sentence and the 2026-10-06 citation
   as the governing rule. Update the pins bullet. Record that "during its resolution" is written
   with the resolve Procedure and the chosen form.
5. Glossary `docs/contexts/game-model/CONTEXT.md`, **Card Class**: its second sentence says such
   a keyword "guards each half of its one definition by the Card Class the half applies to". That
   is now false for gift and haunt; amend it through the `domain-modeling` skill, with the CR
   citations kept, and with ascend named as the one half still guarded until the gate lands.

## Tests

- Rewrite `declaredGiftTriggerHasInterveningIf` (`Proofs/GiftVariants.lean:84-87`, `rfl`): the
  intervening "if" is `keywordCostPaid (.named "gift") this_` alone.
- Rewrite the hand models that mirror the guarded declaration, keeping their claims (each half
  reads the chosen player, on both card classes): `Proofs/GiftVariants.lean` (`onPermanent :26`,
  `onInstantOrSorcery :29`, `gift :41-47`, pins `:58,63,69`) and
  `Proofs/LinkedChoiceReadback.lean` (`permanentHalf :38-41`, `spellHalf :45-46`,
  `okBothHalvesReadChosenPlayer :82`, `okPermanentClassReadsChosenPlayer :87`,
  `okSpellClassReadsChosenPlayer :93`).
- `Proofs/KeywordByCardClass.lean`: its premise is the guarded halves (module doc `:5-12`,
  `okEachHalfChecksClassBlind`, `okBothHalvesOnPermanentCard`, `okBothHalvesOnInstantCard`
  `:20-46`, `okFlyingGuardedToPermanentCard :50`). Rewrite each to assert the unguarded gift and haunt halves
  check clean on both classes (ascend stays guarded: keep its assertions); retire `okFlyingGuardedToPermanentCard` only if its guarded form
  no longer means anything, and say so.
- Keep passing: `okHaunt` (`Proofs/LinkedReadback.lean:322`), `okAscend`
  (`Proofs/OffStack.lean:94`), `okWhileHoldsCitysBlessing` (`:39-47`), `okRebound`
  (`Proofs/OffStack.lean:91`).
- Add `hauntSpellHalfObservesOnArrival`, `buybackMoveObservesOnArrival` and
  `reboundMoveObservesOnArrival` (`rfl` on the definitions' `at_`).
- Add `counteredHauntSpellDoesNotTrigger`, `counteredBuybackSpellDoesNotReplace` and
  `counteredReboundSpellDoesNotReplace`: each of the three events (haunt's trigger event, buyback's
  and rebound's would-clauses) carries the `resolve` qualifier, and the `counter` Definition's own
  stack-to-graveyard move (`Definitions/Actions.lean`, `ownScope (enact counter (move subject stack
  graveyard …))`) does not. The Lean pins are on the terms; whether a countered spell triggers is the
  engine mirror's, which the landing note names.
- Add `resolveProcedureChecks`: `does any (resolve thisSpell)` checks clean and `resolve` is
  admitted wherever `cast`/`play`/`activate` are.

## Other tickets on the same files

- [lean-v3-facts-from-declarations](../wip/lean-v3-facts-from-declarations.md), Step 1 item 5
  (done in its workspace, not landed): deletes `keywordCardOk` and the `onPermanentCard` /
  `onInstantOrSorceryCard` columns, keeps the guarded halves, retitles the same CONTRACTS section
  ("the guarded halves") and extends `Proofs/KeywordByCardClass.lean`. This ruling removes the
  halves' guards that ticket keeps. The two compose (one removes the keyword-level gate, this one
  the per-half guards), but they edit the same section and file: land this one after it, and
  rewrite its retitled section rather than the one quoted above.
- `lean-v3-becomes-blocked-status` and `lean-v3-alternative-cost-unification` edit other hunks of
  `Definitions/Keywords.lean`.
- `lean-v3-cause-flag-fold` changes `Action.move`'s arity (it deletes `cause`), which touches the
  three moves here. Whichever lands second adapts. It also adds the `.anEffect` Template next to
  `Causing`, where this ticket's `Causing.event` use sits.
- 3a now also edits buyback (`:353-358`) and rebound (`:1294-1300`), which the other tickets above
  do not touch; `lean-v3-alternative-cost-unification` edits `Definitions/Keywords.lean` near
  them. `lean-v3-cast-predicates-and-ordinals` rewrites rebound's `castFrom` (`:1292`, now
  `:1298`) in the same replacement clause: whichever lands second adapts.
- `lean-v3-facts-from-declarations` keeps owning ascend's guards and the class gate (3b).
