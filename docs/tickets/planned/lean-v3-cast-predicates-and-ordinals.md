---
needs: []
---
# Cast-predicate stepping fix, and one event-ordinal shape over `Period`

Standard constraints apply. Provenance: row B6 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md). Supersedes
the provenance half of `lean-core-action-provenance` (`maybe/`, deleted with this minting; its
"this way" half went to `lean-v3-this-way-and-stop-trigger-folds`). Its three temporal regimes are
carried below.

## Ruling (owner, 2026-10-22, third batch): B6 scoped back

B6 now does two things, and the third-batch text supersedes the deletion wording below:

1. **Fixes the checker bug**: `.castBy` and `.castFrom` step and check their player and zone
   arguments. VERIFIED (2026-10-22): `pred` in `Check/Phrase.lean` has no `castBy` or `castFrom` case
   (`activatedBy` is stepped at `:2313`), so both reach `| _ => (drs, [])` (`:2364`).
2. **Lands the ordinal unification**: `DidClause.ordinal : Option (Ordinal × Option Period)`, and
   `nthOccurrence` taking `Option Period` (Change step 1).

Deleting `castBy`, `castFrom` and `activatedBy` moves to the new planned ticket
[lean-v3-own-casting-lookback-span](lean-v3-own-casting-lookback-span.md), which `needs` this one. The
`LookbackSpan.thisObject` analysis, the witnesses and `badBareDidCastIsParticipial` moved there as a
candidate design, not yet ruled. The provenance deletion and its tests are not part of this ticket. In this ticket `castBy`'s `rank` stays `Option (Ordinal × Period)` unless the
ordinal unification needs it widened to `Option Period`; the implementer says which.

## Ruling (owner, 2026-10-22; deletion superseded by the third batch)

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

## Ruling (owner, 2026-10-22, second batch): the deletion is in scope

The first-batch STOP (the span-less `did` forms are refused today) is settled by a new span, not by
exempting the checks. The old text is kept below for its evaluated findings.

- Add `thisObject` to `LookbackSpan` (`Abilities.lean:135-144`), reusing the meaning of
  `Period.thisObject` (`Words.lean:620-622`: the object's own existence, until it changes zones and
  becomes a new object [CR#400.7]). VERIFY: true. `LookbackSpan` is `Abilities.lean:135`, and its
  checker arm is `stepLookbackSpanWith` (`Check/Phrase.lean:932-940`): add `| .thisObject => (drs, [])`
  beside `.thisWay | .triggering` (`:940`).
- "Artifact spells you cast" (*Foundry Inspector*, *Fist of Suns*), "if you cast it from your hand"
  (*Arcane Proxy*, *Mulldrifter*) and "spend this mana only to cast a spell from your graveyard" are
  written as `did … (within := some .thisObject)` [CR#400.7]: the "proposed action" and "current
  incarnation" regimes. A "historical window" ("a spell you cast this turn", *Cycle of Life*) keeps
  its duration span.
- `within = none` stays reserved for attributive participles (`DidClause` doc, `Abilities.lean:145-149`).
- Then fold `castBy`, `castFrom` and `activatedBy` into `Predicate.did` within this ticket. The
  castBy/castFrom stepping bug dies with them.

**The three refusals, re-examined (each passes with `.thisObject`; no narrowed STOP remains).**
Evaluated by reading the code; the implementer confirms by running the pins.

1. `participialDescription`, through `DidClause.anchor?` (`Check/Phrase.lean:239-245`): the arm
   `| some _ => none` returns no anchor for any written span, `.thisObject` included. Only
   `none | some .thisWay` can anchor, so the bare form (no span) is still, correctly, a Participial
   Description, and the `.thisObject` form is not.
2. `unanchoredLookback`, through `stepRelativeDidWith` (`:1112`, and the same refusal at `:2735`,
   `:2810`): the refusal is `refuse clause.within.isSome`, so any written span, `.thisObject`
   included, passes. No change to the three call sites.
3. `uniquifying`, through `uniquifies` (`Check/PredicateShape.lean:79-86`): a `the`-headed
   description needs a uniquifying predicate (`Check/Phrase.lean:2458`). `.thisObject` forms are not
   `the`-headed ("artifact spells you cast", "a spell"), so they never meet the check; a `the …
   did … thisObject` would be refused, rightly, since the span does not single anything out. The
   ordinal is what uniquifies ("the first spell you've cast this game"): `uniquifies` loses its
   `castBy _ rank => rank.isSome` arm and gains `| .did clause => clause.ordinal.isSome`, reading
   the new `DidClause.ordinal` field.

### The first-batch STOP (history, kept for the evaluated findings)

**The `did` forms without a span are refused today**, by design of the lookback reading
(evaluated):

- `allOf (and [isArtifact, isSpell, did you (cast which)])` → `participialDescription (.deed
  (.action "cast"))`.
- `target (and [isSpell, did any (cast which (from_ := some graveyard))])` →
  `unanchoredLookback`. `activate which` behaves the same.
- `the (and [isSpell, did you (cast which) thisTurn])` → `uniquifying`, where `nthCastBy` passes
  through `uniquifies`.

Only spanned lookbacks passed (`Proofs/Anaphora.lean:243,250`). The bare forms were the readings
`lean-core-action-provenance` called "proposed action" and "current incarnation", as against a
"historical window".

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
2. **Provenance deletion**: moved to [lean-v3-own-casting-lookback-span](lean-v3-own-casting-lookback-span.md) (the `LookbackSpan.thisObject` span, the rewrite of every `castBy` / `castFrom` / `activatedBy` use, and the deletions with their site lists).
3. **Stepping fix**: give `pred` cases for `castBy` (caster as a player, rank) and `castFrom` (zone) in `Check/Phrase.lean` (`:2185-2364`), and a player kind check on `activatedBy`'s activator.
4. Docs: `lean/CONTRACTS.md:1273,1280` (ordinal only); ADR `docs/decisions/semantics-v3.md`, `:444`
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
- The span pins (`okArtifactSpellsYouCastReadsItsOwnCasting`, `okCastItFromYourHandReadsItsOwnCasting`, `okSpendOnlyToCastFromYourGraveyard`, `badBareDidCastIsParticipial`, `okThisObjectPassesTheLookbackChecks`) and the `did` rewrites of `okCastFromGraveyard`, `okBracketedDescription`, `cycleOfLifeKeepsItsCastCreatureTarget` moved to [lean-v3-own-casting-lookback-span](lean-v3-own-casting-lookback-span.md). Here, those and the card pins keep passing unchanged on the existing predicates.
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
- `lean-v3-keyword-class-guards-and-observation` (second-batch ruling 3a) edits rebound's
  replacement clause in `Definitions/Keywords.lean` (`:1294-1300`), which holds `castFrom` here
  (`:1298`; the list above says `:1292`): whichever lands second adapts.
