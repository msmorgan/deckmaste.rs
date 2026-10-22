---
needs: []
---
# "It can't be regenerated" is `establish (cant …) none`: retire `Duration.modifiesPrecedingAction`

Standard constraints apply. Provenance: the durations item of the owner's rulings on
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md) (its seed
row `Duration.forThisEvent`), which reverses part of
[lean-v3-resolution-riders-and-guards](../done/lean-v3-resolution-riders-and-guards.md) §1.

## Ruling (owner, 2026-10-22)

Delete `Duration.modifiesPrecedingAction` (landed today in `lean-v3-resolution-riders-and-guards`)
and write "It can't be regenerated" as `establish (cant …) none`. This is stated to be the
existing convention: *Pinpoint Avalanche*, *Vexing Shusher* and *Boseiju* use `none` for a
[CR#608.2c] action-scoped prohibition, while game-long prohibitions say `some restOfGame`
explicitly (*Stigma Lasher*, epic). Record the convention in `lean/CONTRACTS.md` and in the
glossary's **Duration** entry, citing [CR#608.2c].

This replaces the earlier ruling of the same day (`lean-v3-resolution-riders-and-guards` §1:
"Keep an explicit marker (`Duration.modifiesPrecedingAction`), not `duration := none`, which
elsewhere means until the end of the game").

## Verified (2026-10-22)

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

## STOP item (the stated convention is not what the repo says)

The code change is ruled and can be prepared; the wording of the convention in `CONTRACTS.md` and
the glossary waits for the owner, because three of its premises are false:

1. *Stigma Lasher* is not on the bench. The game-long witness is *Screaming Nemesis*
   (`Cards/Damage.lean:703`, `establishThroughout (cant they (gainLife anyAmount)) restOfGame`).
2. *Vexing Shusher*'s "Target spell can't be countered" and *Boseiju*'s "that spell can't be
   countered" modify no preceding action. They are no-duration effects under [CR#611.2a] that end
   in practice when the spell leaves the stack [CR#400.7]. Only *Pinpoint Avalanche*'s "The damage
   can't be prevented" is a [CR#608.2c] rider (and `lean-v3-damage-composition` rewrites its tree).
3. The repo's own definition of `none` is the opposite of "action-scoped": the `Duration` doc
   (`Abilities.lean:181-182`) and the glossary say "with no duration stated, an effect lasts until
   the game ends [CR#611.2a]", and `none` is written for effects that do last the game: control
   grants (`Cards/Mana.lean:518`; `CONTRACTS.md:1292-1293` cites [CR#611.2a]), copy changes
   (`Cards/Cost.lean:299`, `Cards/Trigger.lean:356`), quality and text changes
   (`Cards/Cost.lean:394,398`, `Cards/Counters.lean:766`, `Cards/Keyword.lean:1572`,
   `Cards/Mana.lean:157`, `Definitions/Actions.lean:301,660`, `Definitions/Keywords.lean:274`).

What holds: `none` means "no Duration printed". Under that reading, "Destroy target creature. It
can't be regenerated." written as `establish (cant …) none` is, by [CR#611.2a], a prohibition on
that creature until the game ends. The two readings differ observably when the creature is not
destroyed (indestructible): under [CR#608.2c] as the earlier ruling read it, a later regeneration
shield on it still works; under [CR#611.2a], it never does. The owner rules which the convention
note says (for example: "`none` is no printed Duration; a no-duration prohibition that modifies
the action before it binds only that action's event [CR#608.2c], every other one lasts until the
game ends [CR#611.2a]; a printed 'for the rest of the game' is `restOfGame`"), and whether the
engine or the term tells the two apart.

## Change

1. `Templates.lean:1520-1526`: `doUnderProhibition` writes `establish (cant any forbidden {})
   none`. Delete `Duration.modifiesPrecedingAction` and its doc, and `stepDurationWith`'s case
   (`Check/Phrase.lean:907`). Fix the `Duration` doc (`Abilities.lean:181-182`) to the ruled
   wording.
2. `Definitions/Actions.lean:137-140`: `destroyCantRegenerate`'s doc.
3. `lean/CONTRACTS.md:1130-1139`: replace the marker paragraph with the convention note (after the
   STOP), citing [CR#608.2c,611.2a,701.19c]; keep the `doAndForbid` sentence (a printed "this turn"
   keeps its duration).
4. ADR `docs/decisions/semantics-v3.md:577`: drop `modifiesPrecedingAction` from the `Duration`
   bullet.
5. Glossary **Duration**: replace the "lasts only for the Event of the Action before it" clause
   with the convention, through the `domain-modeling` skill, keeping [CR#611.2a,608.2c,701.19c].

## Tests

- Rewrite `destroyCantRegenerateRidesTheDestroy` (`Proofs/Deontic.lean:281-284`, `rfl`, doc
  `:277-280`): the expected tree is `establish … none` after the destroy, as a separate clause, so
  its subject stays a mention read back from the destroy. Rename it if "rides" no longer fits.
- Keep passing: `distributedDeedRiderReadsBackPlural` and
  `regenerationProhibitionBindsTheDestroyed` (`Proofs/Deontic.lean:270-275,294-300`),
  `printedThisTurnProhibitionLastsTheTurn` (`:288-291`), and the bench cards above.
- Add `gameLongProhibitionsPrintTheirDuration`: epic's and *Screaming Nemesis*'s prohibitions carry
  `some restOfGame`, and *Terror*'s rider carries `none`.

## Other tickets on the same files

- `lean-v3-damage-composition` rewrites *Pinpoint Avalanche* (`preventionBan` →
  `cant any (preventDamage …)`), still with no duration.
- `lean-v3-cause-flag-fold` edits other regions of `Abilities.lean`, `Templates.lean` and
  `Check/Phrase.lean`.
- `lean-v3-pay-life-action` changes *Snuff Out*'s cost tree (`Cards/Deontic.lean:449`).
