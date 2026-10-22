---
needs: [lean-v3-this-way-lookback-grounding]
---
# Fold the "this way" conditions into `did`, and stopping a delayed trigger into `cant … trigger`

Standard constraints apply. Provenance: rows B4 and B5 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md). Supersedes
the `-ThisWay` half of `lean-core-action-provenance` (deleted with this minting; its provenance
half went to `lean-v3-cast-predicates-and-ordinals`).

## Rulings (owner, 2026-10-22)

**B4.** Retire `Condition.dealtThisWay`, `choseThisWay`, `flipCalled` and `flipFace` in favour of
`Condition.did … thisWay`. [lean-v3-this-way-lookback-grounding](../wip/lean-v3-this-way-lookback-grounding.md)
(committed, not yet landed) grounds "this way" lookbacks; this ticket needs it.

**B5.** Retire `Action.stopTriggering` in favour of `establish (cant (.actor ability) trigger)
none` over `Action.trigger` [CR#116.2c]. Check that *Quenchable Fire*'s `delayUnlessPaid` still
works.

## Verified (2026-10-22)

Paths are under `lean/Semantics/`.

**B4.**

- The four constructors: `dealtThisWay (predicate)` (`Abilities.lean:472`), `choseThisWay
  (chooser) (predicate)` (`:473`), `flipCalled (caller) (call : FlipCall)` (`:475`), `flipFace
  (face : CoinFace)` (`:476`). `preventedFromSource` (`:474`) stays (but see
  `lean-v3-damage-composition`, which adds a prevention Action).
- **Correction to the spelling.** `Condition.did` takes a `DidClause` (`Abilities.lean:465`),
  `{ subj; action; within : Option LookbackSpan }` (`:150-153`). "This way" is
  `within := some .thisWay` (`LookbackSpan.thisWay`, `:141`). Templates: `Condition.did subj action
  within` (`Templates.lean:2168`), `thisWay` (`:2154`). A `did` with no span is refused
  `.unanchoredLookback` (`Check/Phrase.lean:2796-2800`).
- Checker cases `Check/Phrase.lean:2838-2855`: `dealtThisWay` steps its predicate, then
  `damageDealtInScope`; `choseThisWay` steps the chooser, checks a player, steps the predicate,
  then `choiceInScope`; `flipCalled` steps the caller (no kind check), then `coinFlipInScope`;
  `flipFace` is `coinFlipInScope` only. `dealDamage`, `choose` and `flipCoins` are event heads
  (`Check/Events.lean:25-43`).
- The audit's unchecked premise holds (evaluated): `did any (dealDamage anyAmount (a isPlayer))
  thisWay` publishes the player, so *Screaming Nemesis*'s "they" reads it; after `dealtThisWay`,
  which publishes nothing, "they" refuses `anaphor player`, so today it reads the target instead.
- All four card rewrites check `[]` on the current tree (evaluated).
- **Why the `needs`**: on trunk a `thisWay` span is not grounded (`stepLookbackSpanWith`'s
  `.thisWay | .triggering => (drs, [])`, `Check/Phrase.lean:940`). The `did` forms with nothing
  before them check `[]` today, so retiring the four constructors before the grounding lands
  would turn their three refusal pins into passes. The grounding ticket records caused events
  (`CausedEvent`, `DRS.causedEvents`) and adds `Refusal.thisWayUngrounded`; it touches none of
  the four constructors.

**B5.**

- `Action.stopTriggering (ability : NounPhrase)` (`Abilities.lean:1258-1260`; the `delay` doc at
  `:1251-1255` names it), used once, in `delayUnlessPaid` (`Templates.lean:1343-1353`):
  `sequentially [delay event [] none action, establish (specialAction (some player) none cost
  (stopTriggering (laterMention "delay.trigger" thisAbility))) (some (between none (some
  event)))]`. *Quenchable Fire* (`Cards/Damage.lean:996-1000`) calls the Template.
- `Action.trigger` (`Abilities.lean:1127`) is a deontic head (`Check/Events.lean:58`). An ability
  is a valid `.actor`: `triggers` writes `.does (.actor ability) .trigger`
  (`Templates.lean:1766-1769`), `triggerActorRefusals` (`Check/Phrase.lean:1038-1044`) refuses a
  non-ability actor, and read ahead already writes `cant (.actor (allOf chapter abilities))
  .trigger` (`Definitions/Keywords.lean:1280-1286`).
- `cant (subj : Subject) (action) …` (`Templates.lean:1045`); `establish (spec) (duration : Option
  Duration := none)` (`:1544`).
- **The replacement works** (evaluated): `delayUnlessPaid` with the special action's action
  `establish (cant (.actor (laterMention "delay.trigger" thisAbility)) trigger) none` checks `[]`
  for *Quenchable Fire*, and the three refusals the current pins assert reproduce on it
  (`that .ability` → `anaphor`; no `delay` → `bindingIdentity "delay.trigger"`; a player subject
  → `eventAgent`).
- Checker code specific to `stopTriggering`: `Check/Phrase.lean:3071-3073` and
  `Check/NumberSlots.lean:202`.

## Change

1. B4: rewrite *Screaming Nemesis* (`Cards/Damage.lean:702`), *Sonic Shrieker* (`:715`),
   *Sculpted Sunburst* (`Cards/Choice.lean:1385`, and its doc `:1373-1374`) and *Goblin
   Archaeologist* (`Cards/Faces.lean:96,98`) to `did … thisWay` over `dealDamage`, `choose` and
   `flipCoins` (with `some .wins|.loses`). Keep the Templates' printed names if cards spell them
   (`choseThisWay` `Templates.lean:2096-2097`, `dealtThisWay` `:2101`, `flipCalled` `:2105-2106`,
   `flipFace` `:2107`) by re-bodying them over `did`, or delete them; say which. Delete the four
   constructors and their checker cases. Keep the refusals and helpers other code uses
   (`damageDealtInScope`, `coinFlipInScope`, `choiceInScope`, `FlipCall`, `CoinFace`).
   `lean/CONTRACTS.md:744` (`ChoiceInScope` row names `choseThisWay`).
2. B5: replace the special action's action in `delayUnlessPaid` with `establish (cant (.actor …)
   trigger) none`; delete the constructor, its checker and number-slot arms, and the `delay` doc's
   mention. `lean/CONTRACTS.md:575` (the `delay` row); ADR `docs/decisions/semantics-v3.md:581`
   ("`Instruction.stopTriggering` stops a delayed triggered ability").

## Tests

- B4, rewrite to `did … thisWay`; after the grounding lands the refusal pins expect its refusal
  (`thisWayUngrounded` or as named there), which is the same claim, a lookback with nothing to
  look back to: `okDealtThisWayAfterDamage` and `badDealtThisWayNoDamage`
  (`Proofs/Anaphora.lean:513,521`), `alternativeReadsTheAmountWithoutObservingTheUntakenDamage`
  (`Proofs/ActionFamilies.lean:287`, second conjunct `:290`), `okFlipArmAfterFlip` and
  `badFlipArmWithoutFlip` (`Proofs/Faces.lean:597,604`), `okChoseThisWayAfterChoice` and
  `badChoseThisWayWithoutAChoice` (`Proofs/Choice.lean:438,447`), `okGoblinArchaeologist`
  (`Cards/Faces.lean:99`).
- B4, add `screamingNemesisTheyReadsTheDamagedPlayer`: "they" resolves to the player the `did`
  clause publishes, not to the target.
- B5, rewrite `quenchableFireStopsItsTriggerBySpecialAction` (`Proofs/Damage.lean:54`, `rfl` on
  the tree) and `stopTriggeringReadsTheDelayedTriggerByIdentity` (`:68`, rename to the new form;
  keep its four conjuncts' claims). `okQuenchableFire` (`Cards/Damage.lean:1001`) keeps passing.

## Other tickets on the same files

- [lean-v3-this-way-lookback-grounding](../wip/lean-v3-this-way-lookback-grounding.md): also edits
  `Check/Phrase.lean`, `Proofs/Anaphora.lean` and `lean/CONTRACTS.md:569-575` (the `delay` row).
- `lean-v3-damage-composition` rewrites `Cards/Damage.lean` and `Proofs/Damage.lean` (other cards
  and pins).
- `lean-v3-cast-predicates-and-ordinals` edits `Condition`/`Predicate` neighbours in
  `Abilities.lean`, `Check/Phrase.lean` and `Proofs/Anaphora.lean`.
- `lean-v3-cause-flag-fold` touches `Proofs/Anaphora.lean` and `Proofs/ActionFamilies.lean`.
