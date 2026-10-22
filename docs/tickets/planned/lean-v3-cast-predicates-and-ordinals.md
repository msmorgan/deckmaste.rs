---
needs: []
---
# Cast and activation provenance through `did`, and one event-ordinal shape over `Period`

Standard constraints apply. Provenance: row B6 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md). Supersedes
the provenance half of `lean-core-action-provenance` (`maybe/`, deleted with this minting; its
"this way" half went to `lean-v3-this-way-and-stop-trigger-folds`). Its three temporal regimes are
carried below.

## Ruling (owner, 2026-10-22)

- Delete `Predicate.castBy`, `castFrom` and `activatedBy` in favour of `Predicate.did`:
  `did player (cast which)`, `did any (cast which (from_ := some zone))`, `did player (activate
  which)`.
- Unify the event ordinals: `DidClause` gains `ordinal : Option (Ordinal × Option Period)`, and
  `nthOccurrence` on `GameEvent` and `WouldClause` takes `Option Period`, since `Period.part`
  subsumes `TurnPart`.
- The reported checker bug, that `.castBy` / `.castFrom` fall through `ops.pred` without stepping
  player or zone, is fixed by the deletion. Verify and record it.

## Verified (2026-10-22)

Paths are under `lean/Semantics/`.

- `Predicate.castBy (caster : NounPhrase) (rank : Option (Ordinal × Period))`
  (`Abilities.lean:227`), `castFrom (zone : Location)` (`:228`), `activatedBy (activator :
  NounPhrase)` (`:300`), `Predicate.did (clause : DidClause)` (`:236`). Templates: `castBy`,
  `nthCastBy` (`Templates.lean:602-607`), `castFrom` (`:635`), `Predicate.activatedBy`
  (`:626-627`). The name table (`:337-338`) shares `activatedBy` with the Ability Template of that
  name (`:1898`, *Soul Ransom*), which stays.
- There is no `Action.cast` or `Action.activate`: both are Definitions (`cast (subject) (from_ :
  Option Location := none) (payment) (resulting)`, `Definitions/Actions.lean:43`; `activate`,
  `:57`) over `Action.procedure (kind) (subject) (payment) (resulting) (from_) (of_)`
  (`Abilities.lean:1276`). `cast which (from_ := some zone)` is well-typed.
- `DidClause` (`Abilities.lean:150`) has `subj`, `action`, `within` and no ordinal.
  `GameEvent.nthOccurrence (ordinal) (per : Option TurnPart) (event)` (`:509`) and
  `WouldClause.nthOccurrence … (per : Option TurnPart) (clause)` (`:836`); Templates
  `Templates.lean:1759-1761`, `:1682-1684`. `Period = turn | yourTurn | game | part (part :
  TurnPart) | thisObject` (`Words.lean:617-622`), documented already as what "a usage limit or an
  event ordinal counts within". `TurnPart` (`:604-614`) has `turn`, and no `game` or `yourTurn`.
- **The checker bug is real for `castBy` and `castFrom`.** Neither has a case in `pred`
  (`Check/Phrase.lean:2185-2364`), so both reach `| _ => (drs, [])` (`:2364`): the caster, the
  rank and the zone are never stepped. The only other reader is `Predicate.uniquifies`
  (`Check/PredicateShape.lean:84`), which reads the rank alone. Evaluated: `castBy (a
  isCreature)`, `castBy they` and `castBy it` check `[]`; `castFrom (graveyardOf they)` checks `[]`
  where `in_ (graveyardOf they)` refuses `anaphor player`. **`activatedBy` is stepped** (`:2313`,
  `ops.np`), so `activatedBy they` is refused, but it has no player kind check: `activatedBy (a
  isCreature)` checks `[]`.
- Card references corrected: `Cards/Trigger.lean:481` is *Wavebreak Hippocamp* ("first spell
  during each opponent's turn", `nthOccurrence (.nth 1) …` with `per = none` and a `during`
  timing); the "second spell each turn" trigger is *Trouble in Pairs* (`Cards/Trigger.lean:670-671`).
  *Once Upon a Time* (`:166`) is a predicate fragment `nthCastBy (.nth 1) you .game`; *Highspire
  Bell-Ringer* is `Cards/Cost.lean:154`.

## STOP item (a ruling premise is false as a drop-in)

**The `did` forms without a span are refused today**, by design of the lookback reading
(evaluated):

- `allOf (and [isArtifact, isSpell, did you (cast which)])` → `participialDescription (.deed
  (.action "cast"))`: with a Deed and no span, `DidClause.anchor?` (`Check/Phrase.lean:239-245`)
  makes a `the`/`all`/`each` description a Participial Description.
- `target (and [isSpell, did any (cast which (from_ := some graveyard))])` →
  `unanchoredLookback` (`stepRelativeDidWith`, `:1112`). `activate which` behaves the same.
- `the (and [isSpell, did you (cast which) thisTurn])` → `uniquifying`, where `nthCastBy` passes
  through `uniquifies`.

Only spanned lookbacks pass today (`allOf … did you (cast which) (within soFarThisTurn)`;
`Proofs/Anaphora.lean:243,250`). The bare forms in the ruling are the readings
`lean-core-action-provenance` called "proposed action" (*Foundry Inspector*'s cost reduction on
"artifact spells you cast"; *Fist of Suns*) and "current incarnation" ("if you cast it from your
hand", *Arcane Proxy*, *Mulldrifter*; "spend this mana only to cast a spell from your graveyard"),
as against a "historical window" ("a spell you cast this turn"). The deletion needs a way to write
those two without a span: a `LookbackSpan` (or regime) for the object's own casting
[CR#400.7,601.2], an exemption in `anchor?` and in `stepRelativeDidWith`, and `uniquifies` reading
the new ordinal. That design is not in the ruling; the owner rules it before the predicates go.
The ordinal unification below does not wait on it.

## Change

1. **Ordinals.** `GameEvent.nthOccurrence` and `WouldClause.nthOccurrence` take `per : Option
   Period`; `DidClause` gains `ordinal : Option (Ordinal × Option Period) := none`. Give the field
   a default and move the roughly 70 anonymous `⟨subj, action, within⟩` constructions and the
   pattern at `Check/Phrase.lean:1069` to named fields (`Proofs/Anaphora.lean` 33,
   `Templates.lean:1001,1288,1292,2166,2170,2175,2178`, `Proofs/Lookback.lean` 6,
   `Proofs/HistoricalMove.lean` 4, `Proofs/DrtCards.lean` 3, `Proofs/CorpusRecency.lean` 3,
   `Definitions/Actions.lean:431,442,557`, `Proofs/EventPatterns.lean` 2,
   `Proofs/Damage.lean:788,794`, `Check.lean:96`, and one each in `Zone`,
   `RefusalReachability`, `Choice`, `BindingIdentity`, `Cards/Trigger.lean`). Decide and pin one
   spelling of "each turn": `Period.turn` and `Period.part .turn` would both say it; refuse or
   normalise the other. `Period.thisObject` means something only for a Usage Limit
   (`Abilities.lean:548-549`); refuse it as an event ordinal's period. The `nthOccurrence` readers
   all ignore `per` (`Check/Phrase.lean:1152,1763,2938`, `Check/Events.lean:90`,
   `Check/CardFacts.lean:79,140`, `Check/NumberSlots.lean:218`, `Check/Card.lean:62`) and keep
   doing so.
2. **Provenance**, after the STOP is ruled: rewrite every use to `did`, delete the three
   predicates, their Templates and the name-table entry, `uniquifies`' `castBy` arm and
   `activatedBy` in `seedZone`/`seedsAbility` (`Check/Phrase.lean:158,461`; each use also has
   `abilityHead`, which seeds the same).
   - `castBy`: `Cards/Cost.lean:146,161,168,178,179,180,186,196,202,234,469,486,545,632,748,868,876,954`,
     `Cards/Keyword.lean:152,157,182,1085,1100`, `Cards/Static.lean:118,641,649` (`:118`, *Cycle of
     Life*'s "creature you cast this turn", is a historical window), `Cards/Mana.lean:204`,
     `Cards/Deontic.lean:306`, `Cards/Trigger.lean:700`, `Cards/Faces.lean:616`,
     `Proofs/GetsBothDeltas.lean:181`; `nthCastBy`: `Cards/Trigger.lean:166`,
     `Cards/Cost.lean:154`.
   - `castFrom`: `Definitions/Keywords.lean:221,1292` (rebound),
     `Cards/Trigger.lean:596,722`, `Cards/Keyword.lean:1085`, `Cards/Damage.lean:812,1287`,
     `Cards/Mana.lean:299,575,582,594`, `Cards/Description.lean:250`, `Cards/Cost.lean:632,748`,
     `Proofs/TextChanges.lean:34`, `Proofs/Anaphora.lean:231`.
   - `activatedBy` (Predicate): `Cards/Cost.lean:444,474,487,523,546`.
3. Docs: `lean/CONTRACTS.md:1273,1280`; ADR `docs/decisions/semantics-v3.md:419`, `:444`
   (`nthOccurrence … Option TurnPart`), `:578` (`Period`). Glossary **Usage Limit** already says an
   event's ordinal is not a Usage Limit; keep that, and say in the `Period` doc that one type now
   serves both.

## Tests

- Record the bug with refusal pins that fail on trunk and pass after: a caster that is not a
  player, and an unresolved pronoun in the caster or in the zone, are refused (`castByNonPlayer`,
  `castByUnresolvedPronoun`, `castFromUnresolvedZone`, or as the new form names them); likewise a
  non-player activator.
- Rewrite `firstTimeEachTurnIsTheWatchedEventsOrdinal` (`Proofs/EventPatterns.lean:319-320`, `rfl`
  with `some .turn`) and `eventModifiersPreserveFacts` (`Proofs/Lookback.lean:141`, a `part :
  Option TurnPart` binder).
- After the STOP: `okCastFromGraveyard` (`Proofs/Anaphora.lean:230`), `okBracketedDescription`
  (`Proofs/TextChanges.lean:32`), `cycleOfLifeKeepsItsCastCreatureTarget`
  (`Proofs/GetsBothDeltas.lean:178`, `rfl`), `okOnceUponATimeFirstCast` (`Cards/Trigger.lean:167`),
  and every card pin at the sites above (e.g. `okTroubleInPairsArms` `Cards/Trigger.lean:673`,
  `okResoluteVeggiesaurThirdDie` `Cards/Counters.lean:820`, `okWavebreakHippocamp`
  `Cards/Trigger.lean:483`).
- Add `onceUponATimeAndTroubleInPairsShareTheOrdinalShape`: "the first spell you've cast this
  game" and "your second spell each turn" both carry `(Ordinal × Option Period)`.

## Other tickets on the same files

- `lean-v3-alternative-cost-unification` writes *Omniscience*'s and keeps *Fist of Suns*' subjects
  with `castBy`/`castFrom`; whichever lands second rewrites them.
- [lean-v3-this-way-lookback-grounding](../wip/lean-v3-this-way-lookback-grounding.md) edits
  `stepDidClauseWith` and `anchor?` in `Check/Phrase.lean` and `Proofs/{Anaphora,Lookback,
  HistoricalMove,DrtCards}.lean`: land after it.
- `lean-v3-this-way-and-stop-trigger-folds` and `lean-v3-cause-flag-fold` share
  `Proofs/Anaphora.lean`, `Proofs/EventPatterns.lean` and `Templates.lean`.
