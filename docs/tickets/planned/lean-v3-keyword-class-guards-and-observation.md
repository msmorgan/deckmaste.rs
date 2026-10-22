---
needs: []
---
# Keyword halves without unprinted card-class guards, and observation points by the CR

Standard constraints apply. Provenance: rows A3 and A4 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md). One ticket,
because both edit the haunt definition.

## Rulings (owner, 2026-10-22)

**A3.** Remove the unprinted card-class guard conditions (`is this_ isPermanentType`,
`is this_ isInstantOrSorcery`) from gift, ascend and haunt. Each half is already scoped by its
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

## STOP items (a ruling premise is false as encoded)

1. **Haunt's spell half is not scoped by its event.** [CR#702.55a] says "When this spell is put
   into a graveyard during its resolution". The encoded event is a bare
   `move thisSpell stack graveyard`, which a countered haunt *creature* spell also matches, and
   that trigger functions from the graveyard [CR#113.6k]. With the guard gone, a countered haunt
   creature would haunt. (With the guard, a countered haunt instant already matches: a
   pre-existing gap.) `Causing` (`Abilities.lean:515-518`) has no "during its resolution" form
   [CR#608.2n]. Do not remove haunt's guards until the owner rules how "during its resolution" is
   written; then write it on the event and remove the guards.
2. **Ascend's static half would function on an instant or sorcery spell.** Abilities of an instant
   or sorcery function on the stack [CR#113.6], and ascend there is a spell ability only
   [CR#702.131a]. Unguarded, the static half on an instant grants the city's blessing while the
   spell is on the stack, even if it is then countered. The ruling relies on [CR#113.3a]; that
   rule makes text on an instant a spell ability unless it is a static ability fitting
   [CR#113.6], and the guard-free definition gives the instant a static ability. Owner confirms
   before ascend's static guard goes.

Gift's premise holds: its permanent half is an entering trigger, which an instant never meets, and
its spell half is a spell ability [CR#113.3a,702.174b]. Gift proceeds without waiting.

## Change

1. Gift (`:786`, `:789`): the intervening "if" keeps only `keywordCostPaid gift this_` (the gift
   cost row the audit's seed table kept); the spell half drops the outer `doIf`.
2. Ascend and haunt: as gift, once the STOP items are ruled.
3. A4: write the three moves with `putInto … (from_ := stack)`, or the Template that fits, so the
   observation point is computed. Buyback's and rebound's moves sit inside replacement clauses
   [CR#702.27a,702.88a]; keep their structure.
4. `lean/CONTRACTS.md:906-936`: rewrite "A keyword definition split by card class". A keyword that
   means different abilities on a permanent and on an instant or sorcery writes both halves
   unguarded; each is scoped by its ability category and its event (this ruling, dated). Drop the
   `isPermanentCard` / `isInstantOrSorcery` Template sentence and the 2026-10-06 citation as the
   governing rule. Update the pins bullet.
5. Glossary `docs/contexts/game-model/CONTEXT.md`, **Card Class**: its second sentence says such
   a keyword "guards each half of its one definition by the Card Class the half applies to". That
   is now false; amend it through the `domain-modeling` skill, with the CR citations kept.

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
  `:20-46`, `okFlyingGuardedToPermanentCard :50`). Rewrite each to assert the unguarded halves
  check clean on both classes; retire `okFlyingGuardedToPermanentCard` only if its guarded form
  no longer means anything, and say so.
- Keep passing: `okHaunt` (`Proofs/LinkedReadback.lean:322`), `okAscend`
  (`Proofs/OffStack.lean:94`), `okWhileHoldsCitysBlessing` (`:39-47`), `okRebound`
  (`Proofs/OffStack.lean:91`).
- Add `hauntSpellHalfObservesOnArrival`, `buybackMoveObservesOnArrival` and
  `reboundMoveObservesOnArrival` (`rfl` on the definitions' `at_`).
- Add, with STOP 1's ruling, a pin that haunt's spell trigger event carries "during its
  resolution".

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
  three moves here. Whichever lands second adapts.
