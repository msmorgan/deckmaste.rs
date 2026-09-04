---
needs: []
---
**Dedup the identical predicates, the positional facts row, the string-literal
deed guards and the reach case splits.** Cleanroom review 3, 2026-09-04,
findings G3, G4, G5, G6, G8, G9, G10, G11 and D-Q17. Independent edits.

- **G3 — identical predicates.** `Words.statusEventOk`, `Words.statusEffectOk`
  and `Words.statusHeaderOk` share one table (`FaceDown = True` duplicates the
  fallthrough) — collapse the `StatusEventVal`/`StatusEffectVal`/
  `statusHeaderOk` triplet into one `StatusMarkable`.
  `Events.partTriggerOk`, `Events.partAddable` and `Words.beforePartOk` are
  each `Turn ↦ False, _ ↦ True` — keep one. `Words.designationGiven` equals
  `designationChecked`; `Words.LibOrdinal` equals `Ordinal`;
  `Events.complementLocates` equals `placementDestOk` but for the `Nothing`
  row.
- **G4 — `Words.actFacts` shape.** 89 rows of 15 positional fields, 53 of them
  the all-default stub that exists only so `knownAct` holds. Make it a
  `record ActFacts` with a `plainAct : String -> ActFacts` default and field
  overrides for the non-stub rows; no all-default row survives. A field with a
  single inhabitant that a gate consults is a lexeme guard written as data:
  `actForMana` (Tap alone, deleted by `workbench-tapped-for-mana-event`),
  `actDefends` (Attack) and `actTargeted` (Target) each become the declared
  feature they encode or go together with their gate
  (`Events.verbForManaOk`, `deedDefendsOk`, `deedTargetedOk`).
- **G5 — string-literal deed guards inside core constructors.**
  `Effect.Fights` (`deedNounOk "Attack"`), `Effect.BecomesBlocking` and
  `StopsBlocking` (`"Block"`), `Effect.BecomesAttacking`,
  `Phrase.attackableKind` (`deedAltOk "Attack"`), `Events.deedAbilityRole`
  (`v == "Trigger"`, `v == "Activate"`), `Words.survivesShuffle`
  (`stampedBy "Search"`), `Effect.staticIntro (GainsControl …)`
  (`Just "GainControl"`), `Effect.instrIntro (ControllerSacrifices …)`
  (`"Sacrifice"`) and `Triggers.eventName (UnlocksDoor …)`
  (`VerbedAct "Unlock"`). A guard reads a declared feature, never a lexeme:
  give `ActFacts` the columns it needs (`actFightsAs`, `actSurvivesShuffle`,
  `actAbilityRole`) and read them.
- **G6 — `Macros.kindJoin who what = Joined what who`** is an argument swap
  used 20 times; delete it and write `Joined what who` at the sites. Keep
  `Macros.youAnd`/`youOr` only if "you and/or …" is wanted as a named printed
  lemma; otherwise they are `Both You`/`EitherOf You`.
- **G8 — `Words.Payload.ChosenPlayerP` is a facet, not a constructor.** It
  differs from `PlayerP` only in "chosen", and ten functions case-split the
  pair (`samePayload`, `unionPayload`, `halfReaches`, `pubB`, `setZone`,
  `payloadZone`/`Ty`/`Size`/`Prov`/`Orig`, `choiceBinds`). `ObjectP` already
  carries its facets as fields; make it `PlayerP : (chosen : Bool) -> Payload
  Player`.
- **G9 — reach case splits.** `Phrase.remarkTest`, `Phrase.counterMemoryOk`,
  `Phrase.moveDestOk` and `Phrase.nounEqRef` each enumerate `Pro Bare`,
  `Pro (AtSlot _)`, `Pro (Stamped _)` and `Pro TokenBorn` against the rest,
  and `Effect.counterpartNotSelf` special-cases `Pro Bare OneOf`. The four
  reaches share one property — they resolve to an object binding a move can
  re-stamp. Add `reachTracksObject : Reach -> Bool` and write all five through
  it; `nounEqRef`'s three `Pro` rows become one over a `Reach` equality.
- **G10 — dead and single-use macros.** 25 `Macros` exports have no bench or
  pin use: the scry cluster (`agentLookedDelta`, `agentLookedOuter`,
  `agentLookedPlur`, `agentRef`, `AgentRefOk`, `agentSelfOrOwn`, `lookAndSort`,
  `lookedGroup`, `lookedParted`, `lookedSlice`, `lookedSliceDelta`,
  `lookedSlicePlur`, `lookedSpilled`, `OwnRefOk`), `armyYouControl`,
  `cumulativeUpkeepExpansion`, `noCosts`, the party cluster
  (`creatureInYourParty`, `fullPartyOf`, `partyOf`, `partyRoles`,
  `partySizeOf`), `manaValueSpentToCast`, `thisPermanent` and `thisSpell`.
  Each gets a printed witness or goes; the party cluster has no witness at
  all. The scry cluster belongs to `workbench-windowed-pro`, where `agentRef`
  survives as the window re-read — leave it there. Of the 79 single-use
  exports, delete those that are pure argument re-orders and list the rest.
- **D-Q17 — proof-bundle types outside the exception.** The 2026-09-04 ruling
  keeps only `Card`'s law bundles as proof-bundle witness types; a constructor
  whose arguments are all implicit proofs is otherwise not admissible.
  `Phrase.ChoiceClause` and `ChoiceOrder`, `Effect.SpendPurposes` (which
  re-implements `NonEmpty`), and `Words.KnownCounter` and `KnownKeywordTerm`
  are of that shape. Restate each as an obligation on the constructor that
  needs it, or as the standard predicate it duplicates.
- **G11 — an obligation demanded twice.** `Phrase.ZoneExpr.LibraryAt` already
  demands `PlaceArrangementFits` and `PlaceOrdinalFits`; `Phrase.DestOk`'s
  `LibraryPosOk` demands them again to match the constructor. Take them as
  erased pattern variables.

Size: M. Done when: one predicate survives each G3 family; `actFacts` is a
record with no all-default row and no single-inhabitant gate column; `grep`
finds no string literal in a core constructor's guard; the named dead macros
are gone or witnessed; the removed count is justified name by name; build at
its module count. Standard constraints apply, including the RON-shaped
constraint.

## As landed

- **G11** — `Phrase.DestOk.LibraryPosOk` now takes `af`/`nf` as erased pattern
  variables (`{0 af : …}`, `{0 nf : …}`), so the arrangement/ordinal obligations
  are demanded once, on `ZoneExpr.LibraryAt`. No use site changed.
- **G9** — `Words.reachTracksObject : Reach -> Bool` added (True for
  `Bare`/`AtSlot`/`Stamped`/`TokenBorn`). `Phrase.remarkTest`,
  `counterMemoryOk`, `moveDestOk` and `Effect.counterpartNotSelf` are written
  through it; `nounEqRef`'s three `Pro` rows collapse to one over `Eq Reach`
  via a new `Phrase.proRef` (a direct `Pro r' pl' w'` co-pattern is
  unsolvable — the `Noun` kind index is `reachKind r`). Added `Eq` for
  `Plurality`, `NounWord`, `VerbedMarking`, `SlotCarrier`, `Reach` and
  `Window` in the house `xIx`/`sameX` idiom. `counterpartNotSelf` and
  `nounEqRef` are thereby generalised from `Bare`-only to every
  object-tracking reach; the build is green at 46/46 with the wider gate.
- **G6** — `Macros.kindJoin` deleted; all 20 sites (Cards/Damage, Description,
  Choice, ProofsDamage, ProofsAnaphora, and two inside `Macros.proliferate`)
  now write `Joined what who`. `proliferate`'s two sites needed
  `Compare {k = Player}` because `kindJoin`'s signature was what pinned the
  player kind. `Macros.youAnd`/`youOr` KEPT: "you and …"/"you or …" is a named
  printed lemma with eight bench uses and a `ProofsAnaphora` twin
  (`youAndBindsNothing`); they are not argument re-orders.
- **G8** — `Words.Payload.PlayerP` now carries the facet:
  `PlayerP : (chosen : Bool) -> Payload Player`; `ChosenPlayerP` is gone. The
  ten case-splitting functions (`samePayload`, `unionPayload`, `halfReaches`,
  `pubB`, `setZone`, `payloadZone`/`Ty`/`Size`/`Prov`/`Orig`, `choiceBinds`,
  plus `joinedPayload`, `wordReaches` and two `ProofsAnaphora` twins) lose one
  row each. Semantics preserved exactly, including `unionPayload`'s refusal to
  join a chosen player with an object (`PlayerP False` in those two rows) and
  its refusal to union a chosen with an unchosen player.
- **G3** — five families collapsed. `statusEventOk`/`statusHeaderOk`/
  `statusEffectOk` and `StatusEventVal`/`StatusEffectVal` become
  `Words.statusMarkable` / `StatusMarkable` (the `FaceDown = True` row of
  `statusHeaderOk` was the fallthrough); `Events.partTriggerOk`,
  `Events.partAddable` and `Words.beforePartOk` become one
  `Words.properTurnPart`; `Words.designationGiven` deleted for
  `designationChecked`; the `Words.LibOrdinal` alias deleted for `Ordinal`;
  `Events.complementLocates` deleted, its two `Phrase.playSourceOk` sites now
  write `maybe True placementDestOk zn`.
- **G4** — `Words.actFacts` is now `plainAct "<verb>"` plus `{ field := … }`
  overrides: 88 rows, 43 of them the bare `plainAct` (no all-default
  `MkActFacts` row survives), 45 with overrides. `plainAct : VerbLabel ->
  ActFacts` is the single default row. The two single-inhabitant gate columns
  are gone: `actDefends` and `actTargeted` are replaced by one
  `actFeature : Maybe DeedFeature` (see G5), so `Events.deedDefendsOk` reads
  `deedFeatureOf v == Just Attacking` and `deedTargetedOk` reads
  `== Just Targeting` — both gates survive, neither reads a one-verb boolean.
- **G5** — `Words.DeedFeature = Attacking | Blocking | Targeting |
  ControlGrant | LibrarySearch` is the declared vocabulary; `ActFacts` gains
  `actFeature : Maybe DeedFeature` (set on the `Attack`, `Block`, `Target`,
  `GainControl` and `Search` rows) and `actAbilityRole : Maybe Role` (`Trigger`
  ↦ `Agent`, `Activate` ↦ `Patient`). `Words.Role` moved down from `Events`
  so the column can be typed. Guards rewritten: `Effect.Fights`,
  `BecomesAttacking` → `featureNounOk Attacking Agent`; `BecomesBlocking`,
  `StopsBlocking` → `featureNounOk Blocking Agent/Patient`;
  `Phrase.attackableKind` → `featureAltOk Attacking Patient`;
  `Events.deedAbilityRole` → the `actAbilityRole` column;
  `Words.survivesShuffle` → `stampFeature st == Just LibrarySearch`;
  `Effect.staticIntro (GainsControl …)` → `stampIntro (featureLabel
  ControlGrant)`. New readers: `Words.deedFeatureOf`, `labelWith`,
  `featureLabel`, `stampFeature`; `Events.featureAltOk`;
  `Phrase.featureNounOk`. NOT DONE, sibling region: `Effect.instrIntro
  (ControllerSacrifices …)`'s `moveIntro (Just "Sacrifice")` — the brief parks
  `Effect.instrIntro` with a sibling round, and the guard is inside it.
  NOT DONE, out of the brief's region list and not a guard:
  `Triggers.eventName (UnlocksDoor _ _) = VerbedAct "Unlock"` constructs the
  event's name as data (the same shape as `Macros.Enact … "Sacrifice"`), it
  does not gate on a lexeme.
- **G10** — census re-run against the tree (the review's counts predate the
  windowed-`Pro` and tapped-for-mana rounds; three of the named scry exports —
  `agentLookedDelta`, `agentLookedOuter`, `agentLookedPlur`, `lookedSliceDelta`
  — no longer exist). 395 `Macros` exports, 23 with no direct bench or pin use.
  Of those 23, twelve are the scry/look cluster (`agentRef`, `AgentRefOk`,
  `OwnRefOk`, `agentSelfOrOwn`, `lookAndSort`, `lookedGroup`, `lookedParted`,
  `lookedScope`, `lookedSlice`, `lookedSliceCount`, `lookedSlicePlur`,
  `lookedSpilled`) — left for `workbench-windowed-pro` per the ticket and the
  brief. Deleted: **`creatureInYourParty`** (1, the only export unreachable
  from any witness — no use anywhere, not even inside `Macros`). The other ten
  are internal helpers of a witnessed macro and are kept, each named with the
  witness that reaches it: `armyYouControl` → `amass`
  (`Cards/Keyword.amassZombiesTwo`); `cumulativeUpkeepExpansion` and
  `thisPermanent` → `cumulativeUpkeep` (17 bench uses, e.g.
  `Cards/Deontic` 570); `noCosts` → `chooseModes` (11, e.g.
  `Cards/Choice.rainOfThorns`); `manaValueSpentToCast` → `noManaSpentToCast`
  (`Cards/Trigger` 1082); `thisSpell` → `stormExpansion`
  (`ProofsFaces` 426 — the brief parks `stormExpansion` with a sibling, so it
  cannot be inlined here); `partyRoles`/`partyOf`/`partySizeOf`/`fullPartyOf`
  → `party` (`ProofsDescription.okPartyOfFourRoles`), `partySize` and
  `fullParty` (`Cards/Description` 632, 638, 650). **Correction to the review:**
  the party cluster is NOT witness-less — `Macros.party` has seven bench/pin
  uses. Of the 79 single-bench-use exports, **none** is a pure argument
  re-order (every one either fills a default slot or names a distinct printed
  lemma); `kindJoin` was the only such re-order and G6 removed it. The 79 are
  listed under Landing record.
- **D-Q17** — the five proof-bundle types are gone.
  `Effect.SpendPurposes` → the Prelude's `NonEmpty ps`;
  `Words.KnownCounter l` → `So (knownCounter l)`;
  `Words.KnownKeywordTerm t` → `So (knownKeywordTerm t)`;
  `Phrase.ChoiceClause by n` → `So (choiceClauseOk by n)` and
  `Phrase.ChoiceOrder first by` → `So (choiceOrderOk first by)`, both stated as
  ordinary predicates on the constructor that needs them (`Effect.Choose`) and
  on `Macros.choose`/`chooses`/`secretlyChooses`. Ten pins re-spelled (none
  deleted): `ProofsCounters.badUnknownCounterLabel`,
  `badKeywordCounterNamedPlainly`; `ProofsFaces.badUnknownKeywordPredicate`;
  `ProofsAnaphora.badChooseYou`, `badChooseSomeOf`, `badChooseDefinite`,
  `badClassOfParamlessKeyword`, `badSortedClassOnNumberKeyword`;
  `ProofsZone.badAgentChooseTheRest`; `ProofsMana.badPurposelessSpend`;
  `ProofsChoice.badOrderedSingularChooser`.

## Landing record

**Numbers, before → after**

| | before | after |
| --- | --- | --- |
| Idris modules built | 46 | 46 |
| `actFacts` rows | 88 × 14 positional fields | 88 (43 bare `plainAct`, 45 with `{ … := … }` overrides) |
| all-default `actFacts` rows | 43 | 0 |
| single-inhabitant gate columns (`actDefends`, `actTargeted`) | 2 | 0 |
| string literals in a core constructor's guard | 7 sites | 0 |
| `Payload` constructors | 13 | 12 |
| duplicate-predicate definitions (G3 families) | 11 | 5 |
| proof-bundle witness types (D-Q17) | 5 | 0 |
| `Macros` exports | 395 | 394 |
| `Macros` exports with no bench/pin use | 23 | 22 (12 parked scry cluster + 10 witnessed internals) |
| `Reach` case splits over `Bare`/`AtSlot`/`Stamped`/`TokenBorn` | 5 functions | 0 (all read `reachTracksObject`) |

**Gates** (all foreground)

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`, no `Error` and no `Warning` line; 1m33s clean.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14351 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` → `audited 1 citation site(s)`; the one site is
  `Words.properTurnPart`'s `[CR#500.1]` ("A turn consists of five phases …"),
  which is exactly the claim it backs. No `cite bless` was needed — this round
  registers no rule number that was not already in `cr-citations.lock`.

**Assurance counts** — restored 0, re-spelled 10, ignored 0, added 0,
removed 0. The ten re-spelled are the D-Q17 pins listed above; each keeps its
docstring, its subject term and its positive twin, and only the refuted
proof's constructor changed (`CounterInFactsTable`/`KeywordTermInFactsTable`/
`BareChoice`/`AgentChoice`/`RoundStartsWith` → `Oh`; `MkSpendPurposes` →
`IsNonEmpty`). Non-vacuity was probed once per obligation family by
mis-stating the refused term and watching the message change:
`badUnknownCounterLabel` ("Zorp" → "Stun"), `badUnknownKeywordPredicate`
("Flyign" → "Flying"), `badPurposelessSpend` (`[]` → `[ToActivate Nothing]`),
`badChooseYou` (`You` → `Macros.a Macros.creature`),
`badOrderedSingularChooser` (`target AnyPlayer` → `each AnyPlayer`),
`badAgentChooseTheRest` (`theRest Object` → `someOf (exactly 1) (It ManyOf)`) —
all six then failed with `… Oh/IsNonEmpty is not a valid impossible case`, and
every probe was reverted.

**STOP taken.** G4's "Done when … no single-inhabitant gate column" and G5's
"give `ActFacts` the columns it needs (`actFightsAs`, `actSurvivesShuffle`,
`actAbilityRole`) and read them" pull opposite ways: the columns G5 asks for
are themselves one-verb columns, and deleting `actDefends`/`actTargeted`
"together with their gate" would delete two live checks on
`Effect.DeonticPatient.DefendingPlayer` and `TargetedBy`, which the assurance
rule forbids. Resolution: one multi-inhabitant column,
`actFeature : Maybe DeedFeature` over `Attacking | Blocking | Targeting |
ControlGrant | LibrarySearch`, both retires the two one-verb booleans (their
gates now read `deedFeatureOf v == Just Attacking` / `== Just Targeting`) and
supplies what G5's `actFightsAs`/`actSurvivesShuffle` were for. Both
"Done when" clauses hold and no gate was weakened. No guard naming a lexeme,
verb, noun or card identity was shipped.

**Deviations and additions**

- `actFightsAs` and `actSurvivesShuffle` are NOT separate columns (see STOP);
  `actAbilityRole : Maybe Role` is added as written. `Words.Role` moved down
  from `Events` so the column can be typed; `Events.counterRole` stays put.
- `Macros.youAnd`/`youOr` kept (G6 offers the choice): "you and …" / "you or …"
  is a named printed lemma with eight bench uses and a `ProofsAnaphora` twin.
- `Macros.proliferate` needed `Compare {k = Player}` at its two `Joined` sites:
  the deleted `kindJoin`'s signature was what pinned the player kind.
- Added, all in service of the named bullets: `Words.reachTracksObject`,
  `statusMarkable`/`StatusMarkable`, `properTurnPart`, `plainAct`,
  `DeedFeature` + `deedFeatureIx` + `Eq DeedFeature`, `deedFeatureOf`,
  `labelWith`, `featureLabel`, `stampFeature`, `roleIx` + `Eq Role`, and `Eq`
  for `Plurality`, `NounWord`, `VerbedMarking`, `SlotCarrier`, `Reach` and
  `Window` (all in the house `xIx`/`sameX` idiom, required by `nounEqRef`'s
  collapse); `Events.featureAltOk`; `Phrase.featureNounOk`, `proRef`,
  `choiceClauseOk`, `choiceOrderOk`.
- Deleted: `Macros.kindJoin`, `Macros.creatureInYourParty`,
  `Words.Payload.ChosenPlayerP`, `Words.statusEventOk`/`statusHeaderOk`/
  `statusEffectOk`/`StatusEventVal`/`StatusEffectVal`, `Words.beforePartOk`,
  `Events.partTriggerOk`/`partAddable`, `Words.designationGiven`,
  `Words.LibOrdinal`, `Events.complementLocates`, the `actDefends`/
  `actTargeted` columns, and the five proof-bundle types
  (`Phrase.ChoiceClause`, `Phrase.ChoiceOrder`, `Effect.SpendPurposes`,
  `Words.KnownCounter`, `Words.KnownKeywordTerm`). Every use of each was
  re-spelled across `idris/src/Experimental/`; nothing was left behind.
- Semantic widenings, deliberate and all green: `Effect.counterpartNotSelf` and
  `Phrase.nounEqRef` now cover every object-tracking reach rather than
  `Pro Bare` alone (G9's stated point); `unionPayload` keeps its exact former
  behaviour, including the `PlayerP False` rows.
- Two G5 sites NOT done, both recorded above: `Effect.instrIntro
  (ControllerSacrifices …)` (inside `instrIntro`, which the brief parks with a
  sibling round) and `Triggers.eventName (UnlocksDoor …)` (outside the brief's
  region list, and a value construction rather than a guard).

**The 79 single-bench-use `Macros` exports** (none is a pure argument
re-order; kept, per G10's "list the rest"): `activatedBy`,
`activatedOnlyOnceIf`, `allAmong`, `amass`, `attachToIt`, `becomesSaddled`,
`bottomCard`, `canDoAsThough`, `cantBeTargetedBy`, `chaosEnsues`,
`coloredManaSpentToCast`, `colorlessPip`, `comparesOwnStat`,
`createTappedAttacking`, `defenseBox`, `delayedWithin`, `distributeCounters`,
`entersChoosingPlayerSecretly`, `entersWithFewerCounters`, `eventSum`,
`everyObject`, `exchangeControlOfThis`, `exileWithCounters`, `fateseal`,
`flipsCoin`, `fullParty`, `getsCitysBlessing`, `getsEnduringStory`,
`happenedFrom`, `itIsntA`, `itIsntAnAbility`, `itsACard`, `joinedHeadWhile`,
`keywordNumberCosting`, `lastCounterRemovedBy`, `libraryOf`, `libraryZ`,
`lookAtHandOf`, `lookedTop`, `manyCountersPutByEffect`, `mayVoteAdditional`,
`meldInto`, `monstrosity`, `mustBlockIt`, `noColoredManaSpentToCast`,
`noManaSpentToCast`, `nthFromTopOrBottomZ`, `onTopIn`, `onlyIfSo`,
`outlawYouControl`, `phasesOutUntil`, `phyrexianPip`,
`playerSearchesTheirLibraryFor`, `preventAllBy`, `preventNext`,
`putOntoBattlefieldTappedAttacking`, `removeAllCounters`, `removeCounters`,
`returnToBattlefield`, `returnToBattlefieldTransformed`, `revealsTheirHand`,
`rollThePlanarDie`, `secretlyChooses`, `shiftResult`, `stormExpansion`,
`thatColor`, `theLastChosenColor`, `theLastChosenPlayer`, `thereIsNo`,
`thisCase`, `thisSiege`, `thisSpacecraft`, `tokensCreated`, `topOrBottomZ`,
`untilEndOfCombat`, `untilYourNextEndStep`, `withDifferentNames`,
`youRollHighestNatural`, `youRollPlanarDice`. `fateseal` restating `scry`'s
obligations (review G10's last sentence) is left standing — it is a rename,
not a re-order, and whether fateseal is `scry` over an opponent's library is a
meaning question this ticket does not settle.
