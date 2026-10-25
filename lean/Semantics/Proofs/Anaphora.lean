import Semantics.Check
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Anaphora

Port of `idris/src/Experimental/Proofs/Anaphora.idr`: the pins of the Anaphora family, in
theorem form (a twin `= []`, a pin `= [r]`), each closed by `decide`. The names and the
sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Anaphora

private def actionResult (drs : DRS) (i : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs i).1
private def nounResult (drs : DRS) (n : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .obj true n).1

/-- "Choose two — Draw a card; draw a card." [CR#700.2d] -/
theorem identicalModesAllowed :
    Action.check (DRS.ofList []) (chooseModes (exactly 2) [draw 1, draw 1])
      = [] := by
  decide

/-- "Choose a player or planeswalker." -/
theorem okChoosePlayerOrPlaneswalker :
    Action.check (DRS.ofList [])
      (.choose none (a (.or [.hasType .planeswalker, .anyPlayer])) .openly none) =
          [] := by
  decide

/-- "Choose you." -/
theorem badChooseYou :
    Action.check (DRS.ofList []) (.choose none .you .openly none) = [.choiceClause] := by
        decide

/-- "Choose a creature card and/or a land card": a coordination is chosen when each member is. -/
theorem okChooseCoordinatedIndefinites :
    Action.check (DRS.ofList [])
      (choose (NounPhrase.andOr [a (and [isCreature, isCard]), a (and [isLand, isCard])])) = [] := by
  decide

/-- "Choose the creature and a creature": one member is not chosen, so neither is the
coordination ("the creature" is also refused: nothing in it makes it unique). -/
theorem badChooseCoordinationWithUnchosenMember :
    Action.check (DRS.ofList []) (choose (both (the isCreature) (a isCreature))) =
      [.uniquifying, .choiceClause] := by
  decide

/-- Each member of a chosen coordination is marked by its own determiner: both targets of
"choose target creature you control and target creature you don't control" are announced
targets [CR#601.2c], and the group that collects them is not. -/
theorem chosenCoordinationStampsEachTarget :
    ((actionResult (DRS.ofList [])
        (choose (both (target (and [isCreature, youControl]))
                      (target (and [isCreature, youDontControl]))))).referents.filter
      (·.announcedTargetResult)).map (·.det) = [.target, .target] := by
  decide

/-- Each indefinite member of a chosen coordination is announced as it is chosen [CR#608.2d]:
"choose a creature card and/or a land card" leaves two chosen cards and their group. -/
theorem chosenCoordinationAnnouncesEachIndefinite :
    (actionResult (DRS.ofList [])
        (choose (NounPhrase.andOr [a (and [isCreature, isCard]), a (and [isLand, isCard])]))
      ).referents.map (fun ref => (ref.det, ref.plur)) =
      [(.the, .many), (.part, .one), (.part, .one)] := by
  decide

/-- "Create a 1/1 white Soldier creature token if you control a creature. Put a +1/+1 counter
on it.": a conditional's branch continues when it has no otherwise-branch, wherever its condition
is printed, so "it" is the token, the nearer antecedent. -/
theorem conditionalArmAntecedentIsTheToken :
    let line := doOnlyIf (create 1 (.written (creatureToken 1 1 [.white] [soldier])))
      (exists_ (and [isCreature, youControl])) none
    let out := (stepAction 12 .postEvent .obj (DRS.ofList []) line).1
    Action.check (DRS.ofList [])
      (.sequentially [line, .putCounters 1 (.printed p1p1Counter) it]) = [] ∧
      (resolveCandidates .bare .one (nextClause out)).map (·.payload.orig) = [some .token] := by
  decide

theorem badBothArmsAntecedent :
    Action.check (DRS.ofList [])
      (.sequentially
        [ may (gainLife 1)
            (some (create 1 (.written (creatureToken 1 1 [.white] [soldier]))))
            (some (create 2 (.written (creatureToken 1 1 [.white] [soldier])))),
          .putCounters 1 (.printed p1p1Counter) it ]) = [.anaphor .bare .one 0] := by
  decide

/-- "Put target creature onto the battlefield." -/
theorem okMoveToBattlefield :
    Action.check (DRS.ofList []) (.move (target isCreature) .anyLocation (some battlefield) .arrival []) = [] := by decide

/-- "Look at the top four cards of your library. You choose one of them." -/
theorem okAgentChoiceOfSome :
    Action.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice 4),
          .choose none (someOf (exactly 1) them) .openly none ]) = [] := by
  decide

/-- "Look at the top four cards of your library. Choose one of them." The chooser is the actor
in context, so the bare imperative is the same term as "You choose one of them." -/
theorem okChooseSomeOf :
    Action.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice 4),
          .choose none (someOf (exactly 1) them) .openly none ]) = [] := by
  decide

/-- "Exile target creature." -/
theorem okMoveToExile :
    Action.check (DRS.ofList []) (.move (target isCreature) .anyLocation (some exileZone) .arrival []) = [] := by decide

/-- "When this creature enters, if a creature died this turn, draw a card." -/
theorem okLookbackObjectDied :
    Ability.check (DRS.ofList [])
      (when (does thisCreature enter)
        (if_ := some (.did { subj := .any
                             action := (.move (a isCreature) (battlefield) (some graveyard) .departure [])
                             within := some (.duration soFarThisTurn none) })) (draw 1))
            = [] := by
  decide

/-- "When this creature enters, if you died this turn, draw a card." -/
theorem badLookbackPlayerDied :
    Ability.check (DRS.ofList [])
      (when (does thisCreature enter)
        (if_ := some (.did { subj := .any
                             action := (.move .you (battlefield) (some graveyard) .departure [])
                             within := some (.duration soFarThisTurn none) })) (draw
          1))
      = [.kindLte .player (.join .object .pile)] := by
  decide

theorem badLookbackObjectCast :
    Ability.check (DRS.ofList [])
      (when (does thisCreature enter)
        (if_ := some (.did { subj := (.actor (a isCreature))
                             action := (.procedure .cast .any .itsOwnCost none none none)
                             within := some (.duration soFarThisTurn none) })) (draw
          1))
      = [.kindMismatch .player .object] := by
  decide

/-- "target creature that entered this turn" -/
theorem okHappenedToObjectEntry :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (.and [isCreature, (.did { subj := .any
                                         action := (.move .which .anyLocation (some battlefield) .arrival [])
                                         within := some (.duration soFarThisTurn none) })])) = [] :=
        by
  decide

/-- "target creature who cast a spell this turn" -/
theorem badHappenedToObjectCast :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (.and [isCreature, (.did { subj := (.actor .which)
                                         action := (.procedure .cast .any .itsOwnCost none none none)
                                         within := some (.duration soFarThisTurn none) })]))
      = [.kindMismatch .player .object] := by
  decide

/-- "each opponent who died this turn" -/
theorem badHappenedToPlayerDied :
    NounPhrase.checkAt (some .player) (DRS.ofList [])
      (each (.and [(.isOpponentOf .you), (.did { subj := .any
                                                 action := (.move .which (battlefield) (some graveyard) .departure [])
                                                 within := some (.duration soFarThisTurn none) })])) = [.kindLte .player (.join .object .pile)] := by
  decide

/-- "Choose a creature you control." -/
theorem okChooseIndefinite :
    Action.check (DRS.ofList []) (.choose none (a (and [isCreature, youControl])) .openly none) = [] :=
        by decide

/-- "Choose the creature with the least toughness among creatures you control." -/
theorem badChooseDefinite :
    Action.check (DRS.ofList [])
      (.choose none
        (the (.and [isCreature, .superlative .min (.stat .toughness) (and [isCreature, youControl])]))
        .openly none) = [.choiceClause] := by
  decide

/-- "This deals 1 damage to that permanent or player." -/
theorem okUnionAnaphorAfterJoin :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct .this (.dealDamage 3 (target (.or [isCreature, .anyPlayer]))),
          instruct .this (.dealDamage 1 (that .join)) ]) = [] := by
  decide

/-- "This deals 3 damage to that permanent or player." -/
theorem badUnionAnaphorNoAntecedent :
    Action.check (DRS.ofList []) (instruct .this (.dealDamage 3 (that .join)))
      = [.anaphor (.word .join) .one 0] := by
  decide

/-- "Destroy target creature. This deals 3 damage to that permanent or player." -/
theorem badUnionAnaphorOnObject :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isCreature), instruct .this (.dealDamage 3 (that .join))])
      = [.anaphor (.word .join) .one 0] := by
  decide

/-- A countered ability has left the stack. A subsequent stack-zone reference cannot
reach it [CR#701.6a]. -/
theorem badStackAnaphorAfterCounteringAbility :
    Action.check (DRS.ofList [])
      (.sequentially
        [ counter (target (.abilityHead .anyActivated)), counter (that .stack) ])
      = [.anaphor (.word .stack) .one 0] := by
  decide

def afterAnyTargetDamage : DRS :=
  actionResult (DRS.ofList []) (instruct .this (.dealDamage 3 (target isAnyTarget)))

/-- "This deals 3 damage to any target. This deals 1 damage to that permanent or player." -/
theorem okUnionAnaphorAfterAnyTarget :
    NounPhrase.checkAt (some (.join .object .player)) afterAnyTargetDamage (that .join) = [] := by
  decide

/-- "This deals 3 damage to any target. Counter that spell or ability." Refused: an any-target
union is not on the stack. `that .join` spells the permanent-or-player read
(`okUnionAnaphorAfterJoin`). -/
theorem badStackAnaphorOnPlayerUnion :
    NounPhrase.checkAt (some .object) afterAnyTargetDamage (that .stack)
      = [.anaphor (.word .stack) .one 0] := by
  decide

/-- "a creature with protection from a color" -/
theorem okKeywordClassWithSort :
    Predicate.checkAt .object (DRS.ofList []) (.hasKeyword (.anyIn ⟨(.named "protection"), some .color⟩)) = [] := by decide

/-- "{T}: Draw a card. Activate only if you created a creature this turn." -/
theorem okTokenCreationLookbackComplement :
    Condition.check (DRS.ofList [])
      (.did { subj := (.by .you)
              action := (.createObject .anyAmount (.token (.copyOf (a (.and [isCreature, .isToken])) []) []) none)
              within := some (.duration soFarThisTurn none) }) = [] := by
  decide

/-- "creature that was dealt damage by this creature this turn" -/
theorem okDamageTakenComplement :
    Predicate.checkAt .object (DRS.ofList [])
      (.did { subj := (.actor thisCreature)
              action := (.dealDamage .anyAmount .which)
              within := some (.duration soFarThisTurn none) }) = [] := by
  decide

/-- "if you've cast a creature spell this turn" -/
theorem okCastComplementOnObject :
    Condition.check (DRS.ofList []) (.did { subj := (.actor .you)
                                            action := (.procedure .cast (a (.and [isCreature,
                                              isSpell])) .itsOwnCost none none none)
                                            within := some (.duration soFarThisTurn none) })
      = [] := by
  decide

theorem badPlayerCastComplement :
    Condition.check (DRS.ofList []) (.did { subj := (.actor .you)
                                            action := (.procedure .cast anOpponent .itsOwnCost none none none)
                                            within := some (.duration soFarThisTurn none) })
      = [.kindMismatch .object .player] := by
  decide

/-- "target spell cast from your graveyard": the spell's own casting [CR#400.7]. -/
theorem okCastFromGraveyard :
    Predicate.checkAt .object (DRS.ofList [])
      (did any (cast which (from_ := graveyardOf you)) thisObject) = [] := by decide

/-- "spells cast by a creature": a player casts a spell [CR#601.2]. -/
theorem badCastByNonPlayer :
    Predicate.checkAt .object (DRS.ofList []) (did (a isCreature) (cast which) thisObject) =
      [.kindMismatch .player .object] := by decide

/-- "spells they cast", with nothing for the pronoun to read, and "spells it cast", whose "it"
can read only the spell itself, which casts nothing: a player casts a spell [CR#601.2]. -/
theorem badCastByUnresolvedPronoun :
    Predicate.checkAt .object (DRS.ofList []) (did they (cast which) thisObject) =
        [.anaphor (.word .player) .one 0] ∧
      Predicate.checkAt .object (DRS.ofList []) (did it (cast which) thisObject) =
        [.kindMismatch .player .object] := by
  decide

/-- "the second spell you cast each turn" counts within a turn; an object's own existence is no
span an event ordinal counts within [CR#400.7], though it is the span the lookback reads. -/
theorem ownCastingRankIsAnEventOrdinal :
    Predicate.checkAt .object (DRS.ofList [])
        (did you (cast which) thisObject (ordinal := some (.nth 2, some .turn))) = [] ∧
      Predicate.checkAt .object (DRS.ofList [])
        (did you (cast which) thisObject (ordinal := some (.nth 2, some .thisObject))) =
        [.eventOrdinalPeriod .thisObject] := by decide

/-- "spells cast from their graveyard", with nothing for "their" to read, is refused as "in
their graveyard" is. -/
theorem badCastFromUnresolvedZone :
    Predicate.checkAt .object (DRS.ofList [])
        (did any (cast which (from_ := graveyardOf they)) thisObject) =
        [.anaphor (.word .player) .one 0] ∧
      Predicate.checkAt .object (DRS.ofList []) (.in_ (graveyardOf they)) =
        [.anaphor (.word .player) .one 0] := by decide

/-- "abilities activated by a creature": a player activates an ability [CR#602.2]. -/
theorem badActivatedByNonPlayer :
    Predicate.checkAt .object (DRS.ofList []) (did (a isCreature) (activate which) thisObject) =
        [.kindMismatch .player .object] ∧
      Predicate.checkAt .object (DRS.ofList []) (did you (activate which) thisObject) = [] := by
  decide

/-- "… that was put somewhere from the battlefield this turn." -/
theorem okPlacementOriginBattlefield :
    Predicate.checkAt .object (DRS.ofList [])
      (.did { subj := .any
              action := (.move .which (battlefield) none .departure [])
              within := some (.duration soFarThisTurn none) })
      = [] := by
  decide

/-- "if you've cast a spell from your hand this turn" -/
theorem okCastOriginFromHand :
    Condition.check (DRS.ofList [])
      (.did { subj := (.actor .you)
              action := (.procedure .cast .any .itsOwnCost none (some (hand)) none)
              within := some (.duration soFarThisTurn none) })
        = [] := by
  decide

/-- "if you've cast a creature spell from your hand this turn" -/
theorem okOriginPayloadInvolving :
    Condition.check (DRS.ofList [])
      (.did { subj := (.actor .you)
              action := (.procedure .cast (a (.and [isCreature, isSpell])) .itsOwnCost none (some (hand)) none)
              within := some (.duration soFarThisTurn none) }) = [] := by
  decide

/-- "… that died this turn." -/
theorem okDeathLookbackWithoutOrigin :
    Predicate.checkAt .object (DRS.ofList []) (.did { subj := .any
                                                      action := (.move .which (battlefield) (some graveyard) .departure [])
                                                      within := some (.duration soFarThisTurn none) }) = [] := by decide

/-- "… that was put into a graveyard from the battlefield this turn." -/
theorem okPlacementIntoGraveyard :
    Predicate.checkAt .object (DRS.ofList [])
      (.did { subj := .any
              action := (.move .which (battlefield) (some graveyard) .arrival [])
              within := some (.duration soFarThisTurn none) }) = [] :=
          by
  decide

/-- "… that was put into the battlefield this turn." -/
theorem okPlacementIntoBattlefield :
    Predicate.checkAt .object (DRS.ofList [])
      (.did { subj := .any
              action := (.move .which .anyLocation (some battlefield) .arrival [])
              within := some (.duration soFarThisTurn none) })
      = [] := by
  decide

/-- The context after an earlier action of the same ability, whose event a "this way" reads
back. -/
private def afterEarlier (earlier : Action) : DRS :=
  nextClause (stepAction 64 .postEvent .obj (DRS.ofList []) earlier).1

/-- "Shuffle your library. … if you shuffled your library this way" -/
theorem okShuffledYourLibraryThisWay :
    Condition.check (afterEarlier shuffle)
      (.did { subj := (.actor .you)
              action := (.ownScope (some shuffle) (.shuffle (.zone .library (some .you))))
              within := some .thisWay })
      = [] := by
  decide

/-- The search action carries its mandatory scope directly: "Search your library for any number
of cards. … if you searched your library this way". -/
theorem searchLookbackCarriesItsScope :
    Condition.check (afterEarlier (search yourLibrary (counted anyNumber .isCard)))
      (.did { subj := .actor .you
              action := .search yourLibrary (counted anyNumber .isCard)
              within := some .thisWay })
      = [] := by decide

/-- "Shuffle your library. … if you shuffled your graveyard this way" -/
theorem badShuffledYourGraveyardThisWay :
    Condition.check (afterEarlier shuffle)
      (.did { subj := (.actor .you)
              action := (.ownScope (some shuffle) (.shuffle (graveyardOf .you)))
              within := some .thisWay })
      = [.shuffleZone] := by
  decide

/-- "target creature spell, once it resolves" -/
theorem okResolvedCreatureSpell :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.becomes (a (.and [.hasType .creature, isSpell])))
      = [] := by
  decide

/-- "if it entered this turn" -/
theorem okEntryLookbackWithoutOrigin :
    Predicate.checkAt .object (DRS.ofList []) (.did { subj := .any
                                                      action := (.move .which .anyLocation (some battlefield) .arrival [])
                                                      within := some (.duration soFarThisTurn none) }) = [] :=
      by decide

/-- "For each opponent, you draw a card." -/
theorem okPluralForEach :
    Action.check (DRS.ofList []) (.doForEach (each (.isOpponentOf .you)) (draw 1)) = [] := by
        decide

/-- "For each of target creature, its controller draws a card." -/
theorem badSingletonForEach :
    Action.check (DRS.ofList []) (.doForEach (target isCreature) (draw 1))
      = [.plural] := by decide

/-- "For each player, choose target permanent that player controls. Those players draw a
card." (Vaevictis Asmadi, the Dire's opening loop): the group survives it, pluralised. -/
theorem okLoopGroupSurvives :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .doForEach (each .anyPlayer)
            (choose (target (.and [isPermanent, .hasPossessor .controller they]))),
          instruct (those .player) (draw 1) ]) = [] := by
  decide

/-- "For each player, choose target permanent that player controls. That player draws a
card." The element the loop binds is body-local, so only the group reads back, and only as a
plural. -/
theorem badLoopElementRead :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .doForEach (each .anyPlayer)
            (choose (target (.and [isPermanent, .hasPossessor .controller they]))),
          instruct (that .player) (draw 1) ]) = [.anaphor (.word .player) .one 0] := by
  decide

/-- "if you activated an activated ability this turn" -/
theorem okActivationLookbackComplement :
    Condition.check (DRS.ofList [])
      (.did { subj := (.actor .you)
              action := (.procedure .activate (a (.abilityHead .anyActivated)) .itsOwnCost none none none)
              within := some (.duration soFarThisTurn none) })
      = [] := by
  decide

/-- "on top of your library in any order, third from the top": the place and its depth, and
its order as the move's rider. -/
theorem okOneEndArrangedOrdinal :
    Location.check (DRS.ofList []) (.fromEnd .top (.nth 3) (.zone .library none)) ++
      MoveRider.checkAll (DRS.ofList []) [(.inOrder .anyOrder)] = [] := by
  decide

/-- "Shuffle those cards into your library.": the library they go into, then its shuffle. -/
theorem okShuffledPlain :
    Location.check (DRS.ofList []) (.zone .library none) ++ Action.check (DRS.ofList []) (.shuffle (.zone .library none))
      = [] := by decide

/-- "third from the top" -/
theorem okThirdFromTop :
    Location.check (DRS.ofList []) (.fromEnd .top (.nth 3) (.zone .library none)) = [] := by decide

/-- "zeroth from the top" -/
theorem badZerothFromTop :
    Location.check (DRS.ofList []) (.fromEnd .top (.nth 0) (.zone .library none)) = [.ordinalNonZero] := by
  decide

/-- "Copy target instant or sorcery spell twice. You may choose new targets for those spells."
A copy of a spell is itself a spell [CR#707.10,112.1a], so the plural spell read reaches the
copies. -/
theorem okPluralSpellReadAfterCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.and [isInstantOrSorcery, isSpell])) 2 [],
          may (.chooseNewTargets (those .spell)) ]) = [] := by
  decide

/-- "Copy target instant or sorcery spell. You may choose new targets for the copy." -/
theorem okCopyReadAfterCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.and [isInstantOrSorcery, isSpell])) 1 [],
          may (.chooseNewTargets (that .copy)) ]) = [] := by
  decide

/-- "Copy target instant or sorcery spell. You may choose new targets for that spell."
The copy is itself a spell [CR#707.10]; AC>R1 selects its newer referent. -/
theorem singularSpellReadSelectsCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.and [isInstantOrSorcery, isSpell])) 1 [],
          may (.chooseNewTargets (that .spell)) ]) = [] := by
  decide

/-- "Copy target activated ability twice. You may choose new targets for those abilities." A
copy of an ability is itself an ability [CR#707.10], so the plural ability read reaches the
copies. -/
theorem okPluralAbilityReadAfterCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.abilityHead .anyActivated)) 2 [],
          may (.chooseNewTargets (those .ability)) ]) = [] := by
  decide

/-- "Copy target activated ability. You may choose new targets for that ability."
The copy is an ability [CR#707.10]; AC>R1 selects its newer referent. -/
theorem singularAbilityReadSelectsCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.abilityHead .anyActivated)) 1 [],
          may (.chooseNewTargets (that .ability)) ]) = [] := by
  decide

/-- "Copy target activated ability. You may choose new targets for the copy." -/
theorem okAbilityCopyReadAfterCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.abilityHead .anyActivated)) 1 [],
          may (.chooseNewTargets (that (.copied .ability))) ]) = [] := by
  decide

/-- An ability goes on the stack with no card associated with it [CR#405.1]. -/
theorem abilityIsOnTheStack : Payload.zone (.ability none) = some .stack := by decide

theorem joinedCreatureTy :
    tyOfReach (.word .join) .one
      (actionResult (DRS.ofList []) (instruct .this (.dealDamage 3 (target (.or [isCreature, .anyPlayer])))))
      = [.creature] := by
  decide

theorem anyTargetIsPlaceless : NounPhrase.zone (DRS.ofList []) (target isAnyTarget) = none := by decide

theorem anyTargetTakesDamage :
    Action.check (DRS.ofList [])
      (instruct .this (.dealDamage 1 (target isAnyTarget))) = [] := by decide

/-- "Tap target creature and target artifact. Untap them." -/
theorem okCoordinationThenThem :
    Action.check (DRS.ofList [])
      (.sequentially [tap (and [target isCreature, target isArtifact]), untap them]) = [] := by
  decide

/-- "them" after "target creature and target artifact" reads the group the coordination
publishes, and "that creature" and "that artifact" each still read their own conjunct. -/
theorem coordinationPublishesItsGroup :
    let after := nextClause (actionResult (DRS.ofList [])
      (tap (and [target isCreature, target isArtifact])))
    (resolveCandidates .bare .many after).map (fun ref => (ref.plur, ref.groupKey == some ref.id)) =
        [(.many, true)] ∧
      countReach (.word (.type .creature)) .one after = 1 ∧
      countReach (.word (.type .artifact)) .one after = 1 := by decide

/-- "Tap target creature and all other creatures with the same name as that creature. Untap
them.": the plural conjunct is part of the group, not a second antecedent beside it. -/
theorem okSingularAndPluralConjunctsThenThem :
    Action.check (DRS.ofList [])
      (.sequentially
        [ tap (and [target isCreature,
            allOf (and [isCreature, shares .cardName .share (that creature),
              otherThan (that creature)])]),
          untap them ]) = [] := by decide

/-- Capricious Efreet's targets: "target nonland permanent you control and up to two target
nonland permanents you don't control", then "them". -/
theorem okSingularAndCountedTargetsThenThem :
    Action.check (DRS.ofList [])
      (.sequentially
        [ tap (and [target (and [non isLand, isPermanent, youControl]),
            targets (upTo 2) (and [non isLand, isPermanent])]),
          untap them ]) = [] := by decide

/-- "Exile target creature and all other creatures with that name. Return the exiled cards to
the battlefield.": the participial reads the coordination's group. -/
theorem okExiledCardsAfterSingularAndPluralConjuncts :
    Action.check (DRS.ofList [])
      (.sequentially
        [ exile (and [target isCreature,
            allOf (and [isCreature, shares .cardName .share (that creature),
              otherThan (that creature)])]),
          returnTo (thePlural (and [isCard, was (exile which) thisWay])) battlefield [] ]) = [] := by
  decide

/-- "Sacrifice this creature and any number of lands. Search your library for up to that many
land cards.": the group size is the coordination's group. -/
theorem okGroupSizeAfterCoordination :
    Action.check (DRS.ofList [])
      (.sequentially
        [ sacrifice (and [thisCreature, counted anyNumber isLand]),
          search yourLibrary (counted (upToOf groupSize) isLand) ]) = [] := by decide

/-- Kiora, Master of the Depths: "Reveal the top four cards of your library. Put a creature card
and/or a land card from among them into your hand and the rest into your graveyard.": the
coordination of parts is a part of the revealed cards, not a second group beside them. -/
theorem okTheRestAfterCoordinatedParts :
    Action.check (DRS.ofList [])
      (.sequentially
        [ reveal (topSlice 4),
          putInto (andOr [fromAmong (exactly 1) isCreature them,
            fromAmong (exactly 1) isLand them]) hand,
          putInto (theRest .object) graveyard ]) = [] := by decide

/-- "Look at the top three cards of your library. Put one of them and one of them into your hand
and the other into your graveyard." -/
theorem okTheOtherAfterCoordinatedParts :
    Action.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice 3),
          putInto (and [someOf (exactly 1) them, someOf (exactly 1) them]) hand,
          putInto (theOther .object) graveyard ]) = [] := by decide

/-- "you and this creature" publishes both participants and the group they make up. -/
theorem youAndPublishesBothParticipants :
    (nounResult (DRS.ofList []) (youAnd thisCreature)).referents.map (fun ref => (ref.kind, ref.plur)) =
      [(.join .player .object, .many), (.object, .one), (.player, .one)] := by decide

/-- "If a player is dealt damage this way, you draw a card." -/
theorem okDealtThisWayAfterDamage :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct .this (.dealDamage 3 (target isAnyTarget)),
          .doIf (was (dealDamage anyAmount (a isPlayer)) thisWay) (draw 1) none ]) = [] := by
  decide

/-- "You draw a card. If a player is dealt damage this way, you draw a card.": nothing earlier
dealt damage, so "this way" reads back no event. -/
theorem badDealtThisWayNoDamage :
    Action.check (DRS.ofList [])
      (.sequentially
        [ draw 1, .doIf (was (dealDamage anyAmount (a isPlayer)) thisWay) (draw 1) none ])
      = [.thisWayUngrounded] := by
  decide

/-- "the number of creatures that died this turn" -/
theorem okDeathTally :
    Amount.check (DRS.ofList []) (.tally { subj := .any
                                           action := .move (a isCreature) (battlefield) (some graveyard) .departure []
                                           within := some (.duration soFarThisTurn none) } .events) = [] := by decide

/-- "the amount of creatures that died this turn" -/
theorem badDeathSum :
    Amount.check (DRS.ofList []) (.tally { subj := .any
                                           action := .move (a isCreature) (battlefield) (some graveyard) .departure []
                                           within := some (.duration soFarThisTurn none) } .quantity) = [.tallyOk] := by
  decide

/-- "creature that died this turn" -/
theorem okObjectDeathLookbackSubject :
    Predicate.checkAt .object (DRS.ofList []) (.did { subj := .any
                                                      action := (.move .which (battlefield) (some graveyard) .departure [])
                                                      within := some (.duration soFarThisTurn none) }) = [] := by decide

/-- "creature that won a coin flip this turn" -/
theorem badCreatureWonFlip :
    Predicate.checkAt .object (DRS.ofList []) (.did { subj := .actor .which
                                                      action := .flipCoins (.count 1) (some .wins) none
                                                      within := some (.duration soFarThisTurn none) })
      = [.kindMismatch .player .object] := by
  decide

/-- "the number of dice you rolled this turn" -/
theorem okRollTally :
    Amount.check (DRS.ofList []) (.tally { subj := .actor .you
                                           action := .rollDice 1 .anyDie .anyResult
                                           within := some (.duration soFarThisTurn none) } .events) = [] := by decide

/-- "the amount of dice you rolled this turn" -/
theorem badRollAsMagnitude :
    Amount.check (DRS.ofList []) (.tally { subj := .actor .you
                                           action := .rollDice 1 .anyDie .anyResult
                                           within := some (.duration soFarThisTurn none) } .quantity) = [.tallyOk] := by decide

theorem youWonAFlipThisTurn :
    Condition.check (DRS.ofList []) (.did { subj := .actor .you
                                            action := .flipCoins (.count 1) (some .wins) none
                                            within := some (.duration soFarThisTurn none) }) =
      [] := by decide

theorem youRolledADieThisTurn :
    Condition.check (DRS.ofList []) (.did { subj := .actor .you
                                            action := .rollDice 1 .anyDie .anyResult
                                            within := some (.duration soFarThisTurn none) }) = [] := by decide

/-- "Roll two d20. Ignore the lowest roll." -/
theorem okIgnoreAfterRoll :
    Action.check (DRS.ofList []) (.sequentially [rollDice 2 (.sides 20), .ignoreOutcomes (.extreme
        .lowest)])
      = [] := by
  decide

/-- "Ignore the lowest roll." -/
theorem badIgnoreWithoutRoll :
    Action.check (DRS.ofList []) (.ignoreOutcomes (.extreme .lowest)) = [.ignorableFor] := by decide

def afterATwoDieRoll : DRS := actionResult (DRS.ofList []) (rollDice 2 (.sides 6))

/-- "if you rolled doubles" -/
theorem okRolledDoublesAfterRoll : Condition.check afterATwoDieRoll .rolledDoubles = [] := by decide

/-- "If you rolled doubles, sacrifice this creature." -/
theorem badRolledDoublesWithoutRoll :
    Condition.check (DRS.ofList []) .rolledDoubles = [.outcomeInScope .rollResult 0] := by decide

def afterACoinFlip : DRS := actionResult (DRS.ofList []) (flipCoins 1)

/-- "a player whose coin comes up tails" -/
theorem okCoinCameUpOnPlayer :
    Predicate.checkAt .player afterACoinFlip (.coinCameUp .tails)
      = [] := by
  decide

/-- "the damage whose coin comes up tails". Refused: only objects and players flip coins.
`.coinCameUp` on a player spells the coin read (`okCoinCameUpOnPlayer`). -/
theorem badCoinCameUpOnOutcome :
    Predicate.checkAt .outcome afterACoinFlip (.coinCameUp .tails)
      = [.kindLte .outcome (.join .object .player)] := by
  decide

/-- "Whenever you roll a 4 or higher, …" -/
theorem okBoundedRollTest :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.rollDice 1 .anyDie (.resultIn (.range (some 4) none)))) = [] := by
  decide

/-- "Whenever you roll a 0, …" -/
theorem badZeroRollTest :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.rollDice 1 .anyDie (.resultIn (.range none (some 0)))))
      = [.nonZeroQ] := by
  decide

/-- "Roll two d6. Ignore the lowest roll." -/
theorem okExtremeOverRolls :
    Action.check afterATwoDieRoll (.ignoreOutcomes (.extreme .lowest)) = [] := by decide

/-- "If you would flip a coin, instead flip two coins and ignore the lower one." -/
theorem badExtremeOverFlips :
    Action.check afterACoinFlip (.ignoreOutcomes (.extreme .lowest)) = [.ignorableFor] := by
  decide

/-- "When a player doesn't pay this creature's cumulative upkeep, …" -/
theorem okPayKeywordWithACost :
    GameEvent.check (DRS.ofList []) (.does (.actor (a .anyPlayer)) (.pay .anyCost .once .unpaid (some thisCreature) (some (.named "cumulativeUpkeep"))))
      = [] := by
  decide

/-- "if you paid life this turn" -/
theorem youPaidLifeThisTurn :
    Condition.check (DRS.ofList []) (.did { subj := .actor .you
                                            action := .pay (.perform (.changeLife (.down .anyAmount))) .once .paid none none
                                            within := some (.duration soFarThisTurn none) }) = [] := by decide

def afterAnUpToChoice : DRS := actionResult (DRS.ofList []) (choose (counted (upTo 1) isCreature))

/-- "Choose up to one creature. Destroy the rest." -/
theorem okRestAfterAPartition :
    NounPhrase.checkAt (some .object) afterAnUpToChoice (theRest .object) = [] := by decide

/-- "Destroy the rest." -/
theorem badRestWithoutAPartition :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (theRest .object) = [.theRestFits .object] := by decide

/-- "Choose up to one creature. Destroy the rest." -/
theorem okRestAfterCountedChoice :
    Action.check (DRS.ofList [])
      (.sequentially [choose (counted (upTo 1) isCreature), destroy (theRest .object)]) = [] := by
  decide

/-- "Choose any number of target creatures. Destroy the rest." -/
theorem badRestAfterTargetChoice :
    Action.check (DRS.ofList [])
      (.sequentially
        [choose (.described (.target anyNumber) isCreature), destroy (theRest .object)])
      = [.theRestFits .object] := by
  decide

def afterALook : DRS := actionResult (DRS.ofList []) (lookAt (topSlice 1))

/-- "Look at the top card of your library. Put that card into your graveyard." -/
theorem okReadsLookedAtCard : NounPhrase.checkAt (some .object) afterALook (that .card) = [] := by
  decide

def afterShuffledLook : DRS :=
  actionResult (DRS.ofList []) (.sequentially [lookAt (topSlice 1), shuffle])

theorem badReadsShuffledLibraryCard :
    NounPhrase.checkAt (some .object) afterShuffledLook (that .card)
      = [.anaphor (.word .card) .one 0] := by
  decide

/-- "Whenever you scry, …" -/
theorem okPatientlessScry :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (scry 1)) = [] := by decide

/-- An unspecified event actor is represented explicitly by `Subject.any`. -/
theorem scryPatternAllowsUnspecifiedActor :
    GameEvent.check (DRS.ofList []) (.does .any (scry 1)) = [] := by decide

/-- "Whenever a creature transforms into a Phyrexian, …" -/
theorem okIntransitiveBecomes :
    GameEvent.check (DRS.ofList [])
      (.does .any (.ownScope (some (.action "transform"))
        (.turnOver (a isCreature) (some (.hasSubtype (phyrexian)))))) = [] := by
  decide

/-- "Whenever you discard a card, …" -/
theorem okDiscardFromHand :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.ownScope (some (.action "discard")) (discard (a (.and [.isCard, .in_ hand])))))
      = [] := by
  decide

/-- "if you drew a card this turn" -/
theorem okPlayerDrawLookback :
    Condition.check (DRS.ofList []) (.did { subj := (.actor .you)
                                            action := (.draw (.exactlyOf .anyAmount))
                                            within := some (.duration soFarThisTurn none) }) = [] := by decide

/-- "if you dealt damage to an opponent this turn" -/
theorem badPlayerDamageDealer :
    Condition.check (DRS.ofList []) (.did { subj := (.actor .you)
                                            action := (.dealDamage .anyAmount .any)
                                            within := some (.duration soFarThisTurn none) })
      = [.kindMismatch .object .player] := by
  decide

theorem joinedDealerDamageComplement :
    Predicate.checkAt .object (DRS.ofList [])
      (.did { subj := .actor .which
              action := .dealDamage .anyAmount (a (.or [isCreature, .anyPlayer]))
              within := some (.duration soFarThisTurn none) })
      = [] := by decide

theorem lastChosenPlayerRead :
    Predicate.checkAt .player [ChoiceSort.binding .player, ChoiceSort.binding .player]
      theLastChosenPlayer = [] := by
  decide

/-- "... a chosen player. ... the chosen player." -/
theorem okDefiniteChosenPlayerRead :
    Action.check [ChoiceSort.binding .player]
      (.sequentially
        [ instruct .this (.dealDamage 3 (a isChosenPlayer)),
          instruct .this (.dealDamage 3 (the isChosenPlayer)) ])
      = [] := by
  decide

/-- "... a chosen player. ... that player." AC>R1 selects the player just mentioned. -/
theorem chosenPlayerDemonstrativeSelectsNewMention :
    Action.check [ChoiceSort.binding .player]
      (.sequentially
        [ instruct .this (.dealDamage 3 (a isChosenPlayer)), instruct .this (.dealDamage 3 they) ])
      = [] := by
  decide

/-- "that turn" after exactly one extra turn is minted -/
theorem okThatTurnAfterOneTurn :
    TurnRef.check (actionResult (DRS.ofList []) (extraTurn 1)) .that = [] := by
  decide

theorem badThatTurnWithoutTurn :
    TurnRef.check (DRS.ofList []) .that = [.turnAntecedent 0] := by decide

theorem noTokenAsThoseWithoutAntecedent : countTokenSpecs (DRS.ofList []) = 0 := by decide

theorem oneTokenIsOneSpec :
    countTokenSpecs
      (DRS.ofList [{ det := .a, plur := .one, payload := .object [.creature] (some .battlefield) none (some .token) none }])
      = 1 := by
  decide

theorem manyTokensAreOneSpec :
    countTokenSpecs
      (DRS.ofList [{ det := .a, plur := .many, payload := .object [.creature] (some .battlefield) none (some .token) none }])
      = 1 := by
  decide

/-- A non-token object leaves no definition whatever its plurality. -/
theorem oneNonTokenIsNoSpec :
    countTokenSpecs
      (DRS.ofList [{ det := .the, plur := .one, payload := .object [.creature] (some .battlefield) none none none }]) = 0 := by
  decide

/-- "where X is ..." -/
theorem badDefineWithoutUse : anyOpenLetter .x (DRS.ofList []) = false := by decide

theorem costXStaysOpen :
    anyOpenLetter .x (stepCost 64 (DRS.ofList []) (.loyaltySymbol (.down (.letter .x)))).1 = true := by decide

theorem thisNeedsNoAntecedent : NounPhrase.checkAt (some .object) (DRS.ofList []) .this = [] := by decide

/-- "You" -/
theorem youNeedsNoAntecedent : NounPhrase.checkAt (some .player) (DRS.ofList []) .you = [] := by decide

theorem letterValIntroducesAtEmptyPrefix : Amount.check (DRS.ofList []) (.letter .x) = [] := by decide

theorem ownSurvivesSecondSingular :
    Action.check (DRS.ofList [])
      (.sequentially
        [ exile (target isArtifact),
          dealDamageOwnPower (target isCreature)
            (target isAnyTarget) ]) = [] := by
  decide

/-- "Exile target creature. Draw cards equal to its power." -/
theorem okItReadsTheOnlyBareSingular :
    Action.check (DRS.ofList [])
      (.sequentially [exile (target isCreature), draw (.statOf (.stat .power) it)]) = []
          := by
  decide

theorem possessiveItReadsCurrentSubject :
    Action.check (DRS.ofList [])
      (.sequentially
        [ exile (target isArtifact),
          instruct (target isCreature) (.dealDamage (.statOf (.stat .power) it) (target isAnyTarget)) ])
      = [] := by
  decide

/-- "Draw cards equal to its power." -/
theorem badSingularReadOfBarePlural :
    Action.check (DRS.ofList [])
      (.sequentially
        [ get (bare (and [isCreature, youControl])) (.up 1) (.up 1) (some untilEndOfTurn),
          draw (.statOf (.stat .power) it) ]) = [.anaphor .bare .one 0] := by
  decide

/-- The definite source remains available outside its negated property test. -/
theorem conditionItReadsCurrentSource :
    Action.check (DRS.ofList [])
      (.sequentially
        [ destroy (target isCreature),
          .doIf (.not (.matches thisEnchantment isCreature))
            (become it
              { characteristics := { types := [.creature], subtypes := [angel] } }
              (some untilEndOfTurn))
            none ]) = [] := by
  decide

theorem okOwnPowerWithoutNewMention :
    Action.check (DRS.ofList []) (dealDamageOwnPower .this (target isAnyTarget))
      = [] := by
  decide

/-- "Target creature gets +1/+1": the shared subject is read once by the stat change.
`GetsBothDeltas.selectedPtSubjectHasStableIdentity` pins that subject's identity. -/
theorem okOwnReadsOneInDelta :
    Action.check (DRS.ofList [])
      (establishFor (target isCreature)
        [ getsPt it
            (.up 1) (.up 1) ]
        none) = [] := by
  decide

/-- "Both target creature and target artifact get +1/+1", the shared subject read as "it":
ambiguous between the two, refused once. The corresponding modern shape is also pinned by
`GetsBothDeltas.ambiguousSingularPtSubjectIsRefused`. -/
theorem badSharedSubjectTwoInDelta :
    Action.check (DRS.ofList [])
      (establishFor (both (target isCreature) (target isArtifact))
        [ getsPt
            it
            (.up 1) (.up 1) ]
        none) = [.anaphor .bare .one 2] := by
  decide

/-- "Both target creatures deal damage equal to its power to any target": the singular "its"
has two equally ranked antecedents, so it is refused as an anaphor. -/
theorem badOwnTwoInDelta :
    (Action.check (DRS.ofList [])
      (dealDamageOwnPower (both (target isCreature) (target isCreature)) (target isAnyTarget))).any
        (fun refusal => match refusal with
          | .anaphor .bare .one 2 => true
          | _ => false) = true := by
  decide

/-- "<X> deals damage equal to its power to <Y>" has the pronoun form's tree: the Template keeps
the wording, and "its" is the ordinary possessive pronoun. -/
theorem dealDamageOwnPowerIsThePronounForm (source recipient : NounPhrase) :
    dealDamageOwnPower source recipient = instruct source (dealDamage (powerOf it) recipient) :=
  rfl

theorem distributedDeedReadsBackPlural :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .sequentially [instruct (each (.isOpponentOf .you)) (choose (a (.and [.isCard, .in_ hand]))), discard them],
          exile (thePlural (and [was (discard which), isCard])) ]) = [] := by
  decide

/-- "Discard a card. Exile the discarded card." -/
theorem okTheDiscardedCardAfterSingularDiscard :
    Action.check (DRS.ofList [])
      (.sequentially
        [ discard (a (.and [.isCard, .in_ yourHand])),
          exile (the (and [was (discard which), isCard])) ])
      = [] := by
  decide

/-- "Each opponent discards a card. Exile the discarded card.": the cards discarded are a group,
which a singular "the" reads only when a linked ability discarded them [CR#607.3]. -/
theorem badDistributedDiscardSingular :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .sequentially [instruct (each (.isOpponentOf .you)) (choose (a (.and [.isCard, .in_ hand]))), discard them],
          exile (the (and [was (discard which), isCard])) ])
      = [.participialDescription (.deed (.action "discard")) 0] := by
  decide

/-- "Tap target creature." -/
theorem okTapBattlefieldPermanent :
    Action.check (DRS.ofList []) (.setStatus .tapped (target isCreature)) = [] := by decide

/-- "if there is no monarch" -/
theorem okNoHolderOnPlayer : Condition.check (DRS.ofList []) (thereIsNo (.named "monarch" (.heldBy .player))) = [] := by decide

/-- "if there is no monstrous creature" -/
theorem okNoHolderOnObject :
    Condition.check (DRS.ofList [])
      (.not (exists_ (and [isCreature, .hasDesignation (.named "monstrous" (.heldBy .object)) none]))) = [] := by
  decide

/-- "if there is no monarch creature": a player designation on an object is refused. -/
theorem badPlayerDesignationOnObject :
    Condition.check (DRS.ofList [])
      (.not (exists_ (and [isCreature, .hasDesignation (.named "monarch" (.heldBy .player)) none]))) =
      [ .designationHolder (.named "monarch" (.heldBy .player)) .object,
        .kindMismatch .object .player ] := by
  decide

/-- "Roll five d6. Store those results on this creature." -/
theorem okStoreResultsAfterRoll :
    Action.check (DRS.ofList []) (.sequentially [rollDice 5 (.sides 6), .storeResults thisCreature]) = []
        := by
  decide

/-- "Store those results on this creature." -/
theorem badStoreResultsWithoutRoll :
    Action.check (DRS.ofList []) (.storeResults thisCreature) = [.outcomeInScope .rollResult 0] := by decide

/-- "If you would roll one or more d6, instead roll that many of those dice." -/
theorem okThoseDiceAfterRollEvent :
    Action.check (DRS.ofList [])
      (replaceEvent (.would (.actor .you) (.rollDice .anyAmount (.sides 6) .anyResult))
        (instead (.rollDice .thatMuch .thoseDice .anyResult)) none) = [] := by
  decide

/-- "Roll that many dice." -/
theorem badAnaphoricSidesWithoutRoll :
    Action.check (DRS.ofList []) (.rollDice 1 .thoseDice .anyResult)
      = [.outcomeInScope .diceRolled 0] := by
  decide

theorem singularSpellReadHasCopyIdentity :
    let after := actionResult (DRS.ofList [])
      (.copy .fromStack (target (.and [isInstantOrSorcery, isSpell])) 1 [])
    let selected := resolveCandidates (.word .spell) .one (nextClause after)
    selected.map (·.payload.orig) = [some .copy] ∧
      selected.map (·.id) = [after.nextId - 1] := by decide

theorem singularAbilityReadHasCopyIdentity :
    let after := actionResult (DRS.ofList [])
      (.copy .fromStack (target (.abilityHead .anyActivated)) 1 [])
    let selected := resolveCandidates (.word .ability) .one (nextClause after)
    selected.map (·.payload.orig) = [some .copy] ∧
      selected.map (·.id) = [after.nextId - 1] := by decide

theorem chosenPlayerDemonstrativeHasNewMentionIdentity :
    let before := DRS.ofList [ChoiceSort.binding .player]
    let after := actionResult before (instruct .this (.dealDamage 3 (a isChosenPlayer)))
    (resolveCandidates (.word .player) .one (nextClause after)).map (·.id) =
      [after.nextId - 2] := by decide

theorem possessiveItHasCurrentSubjectIdentity :
    let after := actionResult (DRS.ofList [])
      (.sequentially [exile (target isArtifact),
        instruct (.firstMention "p05.subject" (target isCreature))
          (.dealDamage (.statOf (.stat .power) it) (target isAnyTarget))])
    (after.referents.filter (fun ref => ref.idents.contains "p05.subject")).map
      (·.pronounResolved) = [true] := by decide

/-- "Its controller" read after a destroy reaches the permanent by last known information: the
controller is a player, and the permanent stays in the graveyard. -/
theorem destroyedPermanentControllerReadUsesLastKnownInformation :
    let destroyed := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (destroy (target isArtifact))).1
    let (after, ids, errors) := stepNounPhrase 64 (nextClause destroyed) .obj true
      (controllerOf (that .permanent))
    errors = [] ∧ ids.length = 1 ∧
      (ids.filterMap after.findById?).all (fun r => r.kind == .player) ∧
      after.referents.any (fun r => r.id == 1 && r.zone == some .graveyard) ∧
      after.characteristicRead = false := by decide

/-- Reading the destroyed permanent's controller does not bring it back into view: "untap that
permanent" afterwards has no antecedent. -/
theorem controllerReadDoesNotPermitUntappingDestroyedPermanent :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isArtifact),
        .dealDamage 2 (controllerOf (that .permanent)),
        untap (that .permanent)]) = [.anaphor (.word .permanent) .one 0] := by decide

/-- "Whenever you roll a die" (`.lit 1`, matched by each die rolled [CR#603.2c]) and "Whenever you
roll one or more dice" (`anyAmount`, matched by the roll) are distinct patterns: the first
publishes one die's result, the second the roll's. -/
theorem rollADieAndRollOneOrMoreDiceAreDistinctPatterns :
    countOutcomes .rollResult
        (GameEvent.extend (DRS.ofList []) (.does (.actor .you) (rollDice 1 .anyDie))) = 1 ∧
      countOutcomes .diceRolled
        (GameEvent.extend (DRS.ofList []) (.does (.actor .you) (rollDice 1 .anyDie))) = 0 ∧
      countOutcomes .diceRolled
        (GameEvent.extend (DRS.ofList []) (.does (.actor .you) (rollDice anyAmount .anyDie))) = 1 := by
  decide

/-! ## Readbacks after a mill

To mill a number of cards is to put that many from the top of one's library into one's graveyard
[CR#701.17a]. The milled group's number is the argument's: "mill a card" moves one card, read
back as "that card", "it" or "the card put into your graveyard this way" exactly as after "put the
top card of your library into your graveyard"; "mill two cards" and "mill X cards" move a group. -/

/-- The card put into your graveyard this way, as a "this way" Participial Description reads it. -/
private def cardMilledThisWay (det : Predicate → NounPhrase) : NounPhrase :=
  det (and [isCard, was (putInto which yourGraveyard) thisWay])

/-- "Mill a card. Return that card / it / the card put into your graveyard this way to your
hand." -/
theorem singularReadbackAfterMillingOneCard :
    Action.check (DRS.ofList []) (sequentially [mill (.lit 1), returnToHand (that .card)]) = [] ∧
    Action.check (DRS.ofList []) (sequentially [mill (.lit 1), returnToHand it]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially [mill (.lit 1), returnToHand (cardMilledThisWay the)]) = [] := by decide

/-- One milled card reads as one moved card does: "mill a card" and "put the top card of your
library into your graveyard" agree on every singular and plural readback. -/
theorem millingOneCardReadsAsMovingTheTopCard :
    ([that .card, it, cardMilledThisWay the, them, those .card].map fun np =>
      Action.check (DRS.ofList []) (sequentially [mill (.lit 1), returnToHand np])) =
    ([that .card, it, cardMilledThisWay the, them, those .card].map fun np =>
      Action.check (DRS.ofList [])
        (sequentially [putInto (topSlice (.lit 1)) graveyard, returnToHand np])) := by decide

/-- "Mill a card. Return them to your hand." has no group to read: one card was milled. -/
theorem pluralReadbackAfterMillingOneCardIsRefused :
    Action.check (DRS.ofList []) (sequentially [mill (.lit 1), returnToHand them]) =
      [.anaphor .bare .many 0] ∧
    Action.check (DRS.ofList []) (sequentially [mill (.lit 1), returnToHand (those .card)]) =
      [.anaphor (.word .card) .many 0] := by decide

/-- "Mill two cards" and "mill X cards" move a group: the plural and "each" readbacks read it,
the singular ones find no one card. -/
theorem singularReadbackAfterMillingAGroupIsRefused :
    Action.check (DRS.ofList []) (sequentially [mill (.lit 2), returnToHand (that .card)]) =
      [.anaphor (.word .card) .one 0] ∧
    Action.check (DRS.ofList []) (sequentially [mill (.lit 2), returnToHand it]) =
      [.anaphor .bare .one 0] ∧
    Action.check (DRS.ofList [])
      (sequentially [mill (.lit 2), returnToHand (cardMilledThisWay the)]) =
      [.participialDescription (.zoneChange none .graveyard) 0] ∧
    Action.check (DRS.ofList []) (sequentially [mill (letter .x), returnToHand (that .card)]) =
      [.anaphor (.word .card) .one 0] ∧
    Action.check (DRS.ofList []) (sequentially [mill (letter .x), returnToHand it]) =
      [.anaphor .bare .one 0] ∧
    Action.check (DRS.ofList [])
      (sequentially [mill (letter .x), returnToHand (cardMilledThisWay the)]) =
      [.participialDescription (.zoneChange none .graveyard) 0] := by decide

/-- After a group mill the plural readbacks read the group. -/
theorem pluralReadbackAfterMillingAGroup :
    Action.check (DRS.ofList []) (sequentially [mill (.lit 2), returnToHand them]) = [] ∧
    Action.check (DRS.ofList []) (sequentially [mill (.lit 2), returnToHand (those .card)]) = [] ∧
    Action.check (DRS.ofList [])
      (sequentially [mill (.lit 2), returnToHand (cardMilledThisWay each)]) = [] ∧
    Action.check (DRS.ofList []) (sequentially [mill (letter .x), returnToHand them]) = [] := by
  decide

/-- "Target creature deals 1 damage to itself": a reflexive reads its own clause's subject.
"Destroy itself" has no subject to read, and is refused as a reflexive, not as a missing "that
permanent". -/
theorem reflexiveReadsItsClausesSubject :
    Action.check (DRS.ofList []) (instruct (target isCreature) (dealDamage 1 itself)) = [] ∧
      Action.check (DRS.ofList []) (destroy itself) = [.reflexiveAntecedent .one 0] := by
  decide

end Semantics.Proofs.Anaphora
