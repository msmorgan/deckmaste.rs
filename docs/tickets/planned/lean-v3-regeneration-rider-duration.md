---
needs: []
---
# Duration convention: `none`, `modifiesPrecedingAction` and `restOfGame`

Standard constraints apply. Provenance: the durations item of the owner's rulings on
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md) (its seed
row `Duration.forThisEvent`), and
[lean-v3-resolution-riders-and-guards](../done/lean-v3-resolution-riders-and-guards.md) §1, which
introduced `Duration.modifiesPrecedingAction`.

## Ruling (owner, 2026-10-22, second batch)

This reverses the first-batch ruling #6 of the same day (delete `modifiesPrecedingAction`, write
the rider as `establish (cant …) none`). That ruling is withdrawn.

- KEEP `Duration.modifiesPrecedingAction` for [CR#608.2c] riders that modify the preceding action.
  "Destroy … It can't be regenerated." stays `some .modifiesPrecedingAction`
  (`doUnderProhibition`, unchanged). *Pinpoint Avalanche*'s "The damage can't be prevented" moves
  to it as well.
- Convention, recorded in `lean/CONTRACTS.md` and the glossary **Duration** entry as part of this
  ticket's landing:
  - `none` means no printed Duration: the effect lasts until the game ends [CR#611.2a]. *Vexing
    Shusher* and *Boseiju* are of this kind.
  - A [CR#608.2c] rider that modifies the preceding action is `some .modifiesPrecedingAction`.
  - A printed "for the rest of the game" is `some restOfGame`. The game-long witness is *Screaming
    Nemesis* (`Cards/Damage.lean:703`), not *Stigma Lasher*.
- The indestructible-creature witness becomes a test: a regeneration shield created after a
  destroy that did not destroy the creature (it is indestructible) still works, because the
  prohibition was bound to the destroy's event and not to the creature for the rest of the game.
- Coordinate with `lean-v3-damage-composition`, which rewrites *Pinpoint Avalanche*'s tree:
  whichever lands second carries the duration over.

The first-batch STOP's three false premises are settled by this ruling: *Stigma Lasher* is
replaced by *Screaming Nemesis*; *Vexing Shusher* and *Boseiju* are `none` (no printed Duration,
[CR#611.2a]) and not [CR#608.2c] riders; the repo's `none` is "until the game ends", which the
convention keeps.

## Verified (2026-10-22; second batch re-verified)

Paths are under `lean/Semantics/`.

- `Action.establish (spec : StaticSpec) (duration : Option Duration)` (`Abilities.lean:1200`);
  Template `establish spec (duration := none)` (`Templates.lean:1544-1545`);
  `establishThroughout spec (duration : Duration)` wraps it in `some` (`:1470-1471`). "Rest of the
  game" is the Template `restOfGame : Duration := .between none none` (`Templates.lean:875`), not
  a constructor.
- `Duration.modifiesPrecedingAction` (`Abilities.lean:188-193`). Its only producer is
  `doUnderProhibition` (`Templates.lean:1520-1526`: `sequentially [action, establish (cant any
  forbidden {}) (some .modifiesPrecedingAction)]`); its only checker code is
  `stepDurationWith`'s `| .modifiesPrecedingAction => (drs, [])` (`Check/Phrase.lean:907`).
  `Check/CardFacts.lean:125-132` falls through `_`.
- Users of `doUnderProhibition` (source unchanged, tree changes): `destroyCantRegenerate`
  (`Definitions/Actions.lean:137-142`, doc cites [CR#608.2c]) on *Terror*, *Snuff Out*, *Wrath of
  God*, *Damnation* (`Cards/Deontic.lean:443,451,456,460`) and *Do or Die* (`Cards/Piles.lean:180`);
  directly, *Damn* (`Cards/Deontic.lean:523`) and *Nekrataal* (`:533`).
- *Pinpoint Avalanche*: `establish (preventionBan thatDamage .noPreventionOnly)`, duration omitted
  (`Cards/Deontic.lean:274-280`). *Vexing Shusher*: `establish (cant any (counter (target
  isSpell)))`, omitted (`Cards/Cost.lean:494-501`). *Boseiju*: `establish (cant any (counter (that
  .spell)))`, omitted (`Cards/Mana.lean:912-917`); *Delighted Halfling* (`:908`) the same. Epic:
  `establishThroughout (cant you (cast .any)) restOfGame` (`Definitions/Keywords.lean:624`).
- Docs that state the marker: `Abilities.lean:181-193` (the `Duration` doc), `lean/CONTRACTS.md`
  "Template and definition parameters and expanded actions" (`:1130-1139`), ADR
  `docs/decisions/semantics-v3.md:577`, glossary **Duration** (`docs/contexts/game-model/CONTEXT.md`,
  the sentence "or lasts only for the Event of the Action before it …").

- *Pinpoint Avalanche* re-verified (VERIFY: true): `Cards/Deontic.lean:274-280`, the tree is
  `sequentially [instruct this_ (dealDamage 4 (target isCreature)), establish (preventionBan
  thatDamage .noPreventionOnly)]` with the duration omitted, i.e. `none` (the `establish` Template
  default, `Templates.lean:1544-1545`). `stepDurationWith`'s `.modifiesPrecedingAction` arm
  (`Check/Phrase.lean:907`) steps nothing, so the new duration changes no check result.

## Change

1. Keep `Duration.modifiesPrecedingAction` (`Abilities.lean:188-193`), `doUnderProhibition`
   (`Templates.lean:1520-1526`) and `stepDurationWith`'s case (`Check/Phrase.lean:907`) as they
   are. `destroyCantRegenerate`'s doc (`Definitions/Actions.lean:137-142`) needs no change.
2. *Pinpoint Avalanche* (`Cards/Deontic.lean:274-280`): `establish … (some .modifiesPrecedingAction)`
   after the damage step. Today that is `establish (preventionBan thatDamage .noPreventionOnly)
   (some .modifiesPrecedingAction)`; once `lean-v3-damage-composition` step 1 lands it is `establish
   (cant any (preventDamage .all (damage := some thatDamage))) (some .modifiesPrecedingAction)`.
   If that ticket lands first it carries the duration over, and this ticket then only checks it.
3. `Abilities.lean:181-193`, the `Duration` doc: state the three-way convention (`none` = until the
   game ends [CR#611.2a]; `modifiesPrecedingAction` = [CR#608.2c] rider; printed "for the rest of
   the game" = `restOfGame`), keeping the existing sentences.
4. `lean/CONTRACTS.md:1130-1139`: keep the paragraph, add the convention, citing
   [CR#608.2c,611.2a,701.19c], and name the witnesses (Terror, Pinpoint Avalanche; Vexing Shusher,
   Boseiju; epic, Screaming Nemesis). Keep the `doAndForbid` sentence (a printed "this turn" keeps
   its duration).
5. Glossary **Duration** (`docs/contexts/game-model/CONTEXT.md:846-858`), through the
   `domain-modeling` skill: add the convention beside the existing "lasts only for the Event of the
   Action before it" clause, keeping [CR#611.2a,608.2c,701.19c].
6. ADR `docs/decisions/semantics-v3.md:577` already lists `modifiesPrecedingAction`; leave it.

## Tests

- Keep passing, unchanged: `destroyCantRegenerateRidesTheDestroy` (`Proofs/Deontic.lean:281-284`),
  `distributedDeedRiderReadsBackPlural`, `regenerationProhibitionBindsTheDestroyed`
  (`:270-275,294-300`), `printedThisTurnProhibitionLastsTheTurn` (`:288-291`), and the bench cards
  above.
- Add `indestructibleVictimKeepsLaterRegenerationShield`: the witness. For "Destroy target
  creature. It can't be regenerated." the prohibition's duration is `some .modifiesPrecedingAction`,
  neither `none` nor `restOfGame`, so it binds the destroy's event only and a regeneration shield
  made after a destroy that did not destroy the creature is not forbidden. The Lean pin is on the
  term (`rfl`); the behaviour belongs to the engine mirror, which this ticket names in its landing
  note.
- Add `durationConventionWitnesses`: epic's and *Screaming Nemesis*'s prohibitions carry `some
  restOfGame`; *Terror*'s rider and *Pinpoint Avalanche*'s "can't be prevented" carry `some
  .modifiesPrecedingAction`; *Vexing Shusher*'s and *Boseiju*'s carry `none`.

## Other tickets on the same files

- `lean-v3-damage-composition` rewrites *Pinpoint Avalanche*'s tree (`preventionBan` →
  `cant any (preventDamage …)`); the duration is now `some .modifiesPrecedingAction` in both
  forms, and whichever ticket lands second carries it over.
- `lean-v3-cause-flag-fold` edits other regions of `Abilities.lean`, `Templates.lean` and
  `Check/Phrase.lean`.
- `lean-v3-pay-life-action` changes *Snuff Out*'s cost tree (`Cards/Deontic.lean:449`).
