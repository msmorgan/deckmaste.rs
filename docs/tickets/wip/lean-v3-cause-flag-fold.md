---
needs: []
---
# "By an effect" has one encoding: retire `byEffect` and `move`'s `cause`

Standard constraints apply. Provenance: row B1 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md).

## Ruling (owner, 2026-10-22)

- Delete `byEffect : Bool` from `createObject`, `putCounters` and `removeCounters`, and delete
  `cause : Option Causing` from `Action.move` (claimed dead).
- "By an effect" is written `GameEvent.causes .anEffect` / `WouldClause.causedBy .anEffect`
  [CR#614.16,703.1]. Whether an imperative, a cost or a turn-based action is an effect is already
  fixed by the node that encloses it.
- ~~Delete `eventVoiceChecks` and `Refusal.verbedVoiceOk`, with their two tests.~~ Superseded by
  the second-batch ruling below: only the `byEffect` conjuncts go.
- `lean-v3-saga-rules-fidelity` landed `sagaLoreIncrement` with `byEffect := false` and the pin
  `sagaLoreIncrementIsNoEffect`. Rewrite both so the turn-based row is no effect by its enclosing
  node.

The CR distinction stays: "if an effect would create … / put … counters" applies to a resolving
spell's or ability's effect, or to a replacement or prevention effect [CR#614.16,609.1], and a
turn-based action is no effect [CR#703.1], so a Saga's lore counter [CR#714.3c] is not doubled by
*Doubling Season*. Only its triplicated encoding goes.

## Verified (2026-10-22)

Paths are under `lean/Semantics/`.

- `createObject … (byEffect : Bool) (under …)` (`Abilities.lean:1204`), `putCounters …
  (byEffect : Bool)` (`:1213-1214`), `removeCounters … (byEffect : Bool)` (`:1220-1221`).
  `byEffect` is read only by `eventVoiceChecks`; every other match binds it as `_` or `..`
  (`Check/CardFacts.lean:73`, `Check/NumberSlots.lean:190,194,196`,
  `Check/Phrase.lean:195,1751,3340,3354,3364`, `Check/Events.lean:40,55,75,76`,
  `Check/LocalShape.lean:142-143`).
- `GameEvent.causes (cause : Causing) (event)` (`Abilities.lean:511`), `Causing.anEffect`
  (`:518`), `WouldClause.causedBy (cause : Causing) (clause)` (`:830`). **No Template writes
  `.anEffect`** (only `bySource`, `Templates.lean:2183`, and `byEvent`, `:2185`), and `spelled`
  refuses raw constructors (`Check/Card.lean:424-426`), so one is needed.
- Turn-based actions are `Conferral.turnBased (part) (then_ : Action)` (`Rules.lean:75`, checked
  at `Check/Rules.lean:40`).
- **`move`'s `cause`: dead in effect, not in syntax.** No Card, Definition or Template sets it to
  `some`; about 140 sites pass `none`. Eleven core pins in `Check.lean` set `some .anEffect`
  (`:53,70,74,91,112,165,167,182,244,245,261`: explore, connive, airbend, the later-mention,
  Kalitas, sacrifice-draw and slice C/D pins), and the checker steps it (`optCause`,
  `Check/Phrase.lean:3219`), but `.anEffect` steps to nothing (`:1127`). Deleting it loses no
  check. Template parameter: `Templates.lean:2138-2141`.
- **`eventVoiceChecks` (`Check/Phrase.lean:1016-1031`) checks more than `byEffect`.** Called from
  `did` (`:1100`), `would` (`:1142`) and `does` (`:2926`), it refuses a counter event's agent that
  is not an event agent (`.eventAgent`, `:1021`), an agent together with `byEffect`
  (`.causedByOk`, `:1022`), a token phrase that is not one (`.tokenPhrase`, `:1025`), a non-player
  `under` (kind check, `:1027`), and, as `.verbedVoiceOk` (`:1029-1030`), a creation event whose
  agent or `under` is plural or not an event agent, or that has both an agent and `byEffect`.
  Refusals: `verbedVoiceOk` (`Check/Refusal.lean:134`), `causedByOk` (`:137`).
- **The tests are not in `Proofs/EventPatterns.lean`.** `verbedVoiceOk` is pinned by
  `badTokensCreatedByCauserAndPlayer`, `badTokensCreatedUnderPlural` and
  `badTokensCreatedByMintingNoun` (`Proofs/Trigger.lean:151,157,163`); `causedByOk` by
  `badCausedCounterWithAgent` and `badCounterEventAgentAndEffect` (`Proofs/Counters.lean:372,534`).
  Of these only `badTokensCreatedByCauserAndPlayer` and the two `causedByOk` pins fail because of
  `byEffect`. `badAnnouncingRemovalAgent` (`Proofs/Trigger.lean:65`, `.eventAgent`) and
  `badNonTokenCreationSubject` (`:98`, `.tokenPhrase`) depend on `eventVoiceChecks` without
  `byEffect`.
- The Saga row is in `Proofs/Rules.lean`, not `Rules.lean`: `sagaLoreIncrement` (`:286-287`) is
  `.putCounters 1 (.printed (.named "loreCounter")) thisSaga false`, used under `.turnBased
  .firstMain` (`:298-299`); `sagaLoreIncrementIsNoEffect` (`:313-314`) is an `rfl` on that tree,
  and its docstring (`:307-312`) explains it by "not `byEffect`".

## Ruling (owner, 2026-10-22, second batch)

**2a.** Delete only the `byEffect` conjuncts of `eventVoiceChecks`: the `causedByOk` refusal
(`Check/Phrase.lean:1022`, `Refusal.lean:137`) and the `!(byEffect && agent.isSome)` part of
`verbedVoiceOk` (`Check/Phrase.lean:1029-1030`). Keep the `.eventAgent`, `.tokenPhrase`,
`under`-kind and plurality checks. `eventVoiceChecks` itself stays, and so does the rest of
`verbedVoiceOk` (renaming it to a plurality name is allowed). This settles the old STOP 1.

**2b.** Drop the refusal of an event that has both a performer and an effect cause. The two are
orthogonal. Witness: *Wilt-Leaf Liege*, "If a spell or ability an opponent controls causes you to
discard a card, …": a performer (you discard) under a cause (a spell or ability an opponent
controls). Pin that tree as accepted. This settles the old STOP 2, and removes the "keep an
equivalent refusal" interim instruction: no refusal replaces `causedByOk`.

## Change

1. Add a Template for `Causing.anEffect` (name at the implementer's discretion; `byAnEffect` reads
   with `bySource`/`byEvent`).
2. Delete the three `byEffect` fields and `move`'s `cause`. Every match and constructor site
   changes arity: `Check/Card.lean:60`, `Check/Events.lean:62`, `Check/NumberSlots.lean:164`,
   `Check/Phrase.lean:187,225,1750,3203` and `Proofs/EventPatterns.lean:143,145,152` for `move`;
   `Check/CardFacts.lean:73`, `Check/NumberSlots.lean:190,194,196`,
   `Check/Phrase.lean:1751,3340,3354,3364` and `Check/Queries.lean:164,168,178,195` for the
   other three. Delete `optCause` (`Check/Phrase.lean:3219`) and the `cause` Template parameter.
3. Templates that carry the Bool (`Templates.lean`): `getEmblem` (`:76`), `loseCounters`
   (`:1210-1212`), `removeCounters` (`:1378-1380`), `removeAllCounters` (`:1386-1388`),
   `createDoubleFacedToken` (`:1509`), `createObject` (`:1510-1512`), `moveCounters`
   (`:1572,1574`), `putCounters` (`:1582-1584`), and the event Templates passing `false`
   (`countersPut` `:1704`, `countersRemoved` `:1709`, `bareCountersPut` `:1712`,
   `bareCountersRemoved` `:1715`, `lastCounterRemovedBy` `:1719`, `manyBareCountersPutBy`
   `:1725`, `lastCounterRemoved` `:1739`). `manyCountersPutByEffect` (`:1721-1722`),
   `tokenCreationEvent`, `tokensCreated` and `tokensCreatedByEffectUnder` (`:1727-1736`) have no
   users outside `Templates.lean`: rewrite `manyCountersPutByEffect` and
   `tokensCreatedByEffectUnder` over `causes`/`causedBy` with the new Template, or delete them,
   and say which.
4. `Definitions/Actions.lean:30` (`create` passes `true none`) and every `move` site in
   `Definitions/Actions.lean` and `Definitions/Keywords.lean` (each passes `none`).
5. Cards whose Oracle text says "an effect" gain `causedBy`: *Doubling Season* (both lines,
   `Cards/Counters.lean:322-330`; its counter line gets "by an effect" today only from the
   Template default) and *Anointed Procession* (`Cards/Static.lean:168`). Cards with an explicit
   `false` drop it with no `causedBy`, since their text has no "an effect": *Hardened Scales*,
   *Branching Evolution*, *Corpsejack Menace*, *Doc Samson*, *Winding Constrictor*, *Primal
   Vigor* (`Cards/Counters.lean:304,312,318,336,348,352,469,473`), `Cards/Counters.lean:988`,
   *Divine Visitation*, *Adrix and Nev*, *Pir* (`Cards/Keyword.lean:289,302,1135`),
   `Cards/Choice.lean:1029`, *Idol of Oblivion* (`Cards/Cost.lean:621`). The explicit-`true`
   `removeCounters` sites (costs and imperatives in `Cards/Counters.lean`,
   `Cards/Description.lean:480`, `Cards/Mana.lean`) drop it; cost-position puts and removes were
   marked "by an effect" although a cost is no effect, which the fold fixes.
6. The Saga row: `sagaLoreIncrement` is `.putCounters 1 (.printed (.named "loreCounter"))
   thisSaga`; it is no effect because the `turnBased` conferral carries it [CR#703.1,714.3c].
7. Delete the `byEffect` conjuncts of `eventVoiceChecks` (ruling 2a): the `causedByOk` refusal and
   the `!(byEffect && agent.isSome)` part of `verbedVoiceOk`; keep every other check. Delete
   `Refusal.causedByOk` (`Check/Refusal.lean:137`), which nothing else emits. No replacement
   refusal for "performer and effect cause" (ruling 2b).
8. ADR `docs/decisions/semantics-v3.md:360` (`move(patient, from, to, obs, cause)`) and `:413`
   (`zoneChange (… cause)`): drop `cause`. `:466` ("source, event, or effect", `causedBy`) gains
   the "if an effect would" use. `lean/CONTRACTS.md` says nothing about `byEffect`; record in the
   `Causing` row (`:796`) that "by an effect" is `causes`/`causedBy .anEffect` and that an
   Action's being an effect follows from its enclosing node.

## Tests

- Rewrite `sagaLoreIncrementIsNoEffect` as `sagaLoreIncrementIsTurnBased`: the Saga definition's
  conferral is `.turnBased .firstMain` over the increment, which carries no cause. The old pin
  asserted the retired flag.
- Rewrite `okRidersOverATokenCreation` (`Proofs/Static.lean:265`): "If you would create one or
  more tokens" is written today as by an effect with `under := some you`, which the text does not
  say.
- Add `doublingSeasonWatchesAnEffect`: both of *Doubling Season*'s replacement clauses are
  `causedBy .anEffect` (`rfl` or a shape match), and *Corpsejack Menace*'s is not.
- `badTokensCreatedByCauserAndPlayer` (`Proofs/Trigger.lean:151`), `badCausedCounterWithAgent` and
  `badCounterEventAgentAndEffect` (`Proofs/Counters.lean:372,534`): they pin the retired refusals
  (`verbedVoiceOk`'s agent-with-`byEffect` clause, `causedByOk`), which ruling 2b removes. Retire
  them and say so by name in the landing. Where the same trees stay wanted, rewrite them as `ok`
  pins on the `causedBy .anEffect` / `causes .anEffect` form with an agent (performer and cause
  together are accepted).
- Add `okWiltLeafLiegeEffectCausedDiscard`: the *Wilt-Leaf Liege* tree, a performer (`you`) with
  `causes`/`causedBy` over "a spell or ability an opponent controls", checks clean. The card is not
  on the bench (VERIFY: no `Wilt-Leaf Liege` in `lean/Semantics`); spell the card or the trigger
  or replacement fragment, and assert the intended tree, not a count.
- Keep `badTokensCreatedUnderPlural`, `badTokensCreatedByMintingNoun`, `badAnnouncingRemovalAgent`,
  `badNonTokenCreationSubject` asserting their refusals (renamed refusal allowed): ruling 2a keeps
  the agent, token-phrasing, `under`-kind and plurality checks.
- Mechanical arity rewrites, claims unchanged: `Proofs/Trigger.lean` (`:60,65,92,98,139,145,250`),
  `Proofs/Counters.lean` (about 43 sites, including the `rfl` pins
  `theLastEncodesSuspendsLastCounterTrigger` `:698` and `theNthEncodesTheCounterThreshold`
  `:712`), `Proofs/EventPatterns.lean:308,318`, `Proofs/ActionFamilies.lean:51,54,62`,
  `Proofs/Actor.lean:104,159,669,684`, `Proofs/Anaphora.lean:43,52,206`,
  `Proofs/DrtCards.lean:9,201,216`, `Proofs/KeywordActionResidues.lean:42,132`,
  `Proofs/Piles.lean:257,264`, one site each in `BindingIdentity`, `CorpusRecency`,
  `DrtWitnesses`, `HistoricalMove`, `KeywordAbilityCensus`, `ReferenceScopes`, `SelectionEvidence`,
  `Check/Queries.lean:167,176,192`, and the `Check.lean` core pins. Deleting `move`'s `cause` also
  touches every `move` constructor in about thirty Proofs files (chiefly `Zone`, `Anaphora`,
  `ActionFamilies`, `EventPatterns`, `HistoricalMove`, `ActionForms`, `Authoring`, `CallerScope`,
  `Choice`, `ControllerSacrifice`, `DefinitionsAndTemplates`, `Lookback`, `MovementScopes`,
  `OffStack`, `RefusalReachability`).

## Other tickets on the same files

This ticket touches the most files of the set; land it first or last, not alongside the others.

- `lean-v3-damage-composition` rewrites `Proofs/Rules.lean` (the damage rows use `loseCounters`)
  and needs the `.anEffect` Template for wither and infect.
- `lean-v3-keyword-class-guards-and-observation` edits three `move` sites in
  `Definitions/Keywords.lean`.
- `lean-v3-pay-life-action`, `lean-v3-cast-predicates-and-ordinals` and
  `lean-v3-this-way-and-stop-trigger-folds` edit `Proofs/Trigger.lean`, `Proofs/Anaphora.lean`,
  `Proofs/EventPatterns.lean` and `Templates.lean`.
- `lean-v3-regeneration-rider-duration` edits other regions of `Abilities.lean`, `Templates.lean`
  and `Check/Phrase.lean`.
