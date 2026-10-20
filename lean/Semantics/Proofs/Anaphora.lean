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
    Action.check (DRS.ofList []) (chooseModes (exactly 2) [draw (.lit 1), draw (.lit 1)])
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

/-- "Create a 1/1 white Soldier creature token if you control a creature. Put a +1/+1 counter
on it.": a conditional's branch continues when it has no otherwise-branch, wherever its condition
is printed, so "it" is the token, the nearer antecedent. -/
theorem conditionalArmAntecedentIsTheToken :
    let line := doOnlyIf (create (.lit 1) (.written (creatureToken 1 1 [.white] [creatureType "Soldier"])))
      (exists_ (and [isCreature, youControl])) none
    let out := (stepAction 12 .postEvent .obj (DRS.ofList []) line).1
    Action.check (DRS.ofList [])
      (.sequentially [line, .putCounters (.lit 1) (.printed p1p1Counter) it .one true]) = [] ∧
      (resolveCandidates .bare .one (nextClause out)).map (·.payload.orig) = [some .token] := by
  decide

theorem badBothArmsAntecedent :
    Action.check (DRS.ofList [])
      (.sequentially
        [ may (gainLife (.lit 1))
            (some (create (.lit 1) (.written (creatureToken 1 1 [.white] [creatureType "Soldier"]))))
            (some (create (.lit 2) (.written (creatureToken 1 1 [.white] [creatureType "Soldier"])))),
          .putCounters (.lit 1) (.printed p1p1Counter) it .one true ]) = [.anaphor .bare .one 0] := by
  decide

/-- "Put target creature onto the battlefield." -/
theorem okMoveToBattlefield :
    Action.check (DRS.ofList []) (.move (target isCreature) .anyLocation (some battlefield) .arrival none []) = [] := by decide

/-- "Look at the top four cards of your library. You choose one of them." -/
theorem okAgentChoiceOfSome :
    Action.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice (.lit 4)),
          .choose none (someOf (exactly 1) them) .openly none ]) = [] := by
  decide

/-- "Look at the top four cards of your library. Choose one of them." The chooser is the actor
in context, so the bare imperative is the same term as "You choose one of them." -/
theorem okChooseSomeOf :
    Action.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice (.lit 4)),
          .choose none (someOf (exactly 1) them) .openly none ]) = [] := by
  decide

/-- "Exile target creature." -/
theorem okMoveToExile :
    Action.check (DRS.ofList []) (.move (target isCreature) .anyLocation (some exileZone) .arrival none []) = [] := by decide

/-- "When this creature enters, if a creature died this turn, draw a card." -/
theorem okLookbackObjectDied :
    Ability.check (DRS.ofList [])
      (.triggered (.does .any (.move thisCreature .anyLocation (some battlefield) .arrival none [])) [] none [] none none
        (some (.did ⟨.any, (.move (a isCreature) (battlefield) (some graveyard) .departure none []), (.duration soFarThisTurn none)⟩)) (draw (.lit 1)))
            = [] := by
  decide

/-- "When this creature enters, if you died this turn, draw a card." -/
theorem badLookbackPlayerDied :
    Ability.check (DRS.ofList [])
      (.triggered (.does .any (.move thisCreature .anyLocation (some battlefield) .arrival none [])) [] none [] none none
        (some (.did ⟨.any, (.move .you (battlefield) (some graveyard) .departure none []), (.duration soFarThisTurn none)⟩)) (draw
          (.lit 1)))
      = [.kindLte .player (.join .object .pile)] := by
  decide

theorem badLookbackObjectCast :
    Ability.check (DRS.ofList [])
      (.triggered (.does .any (.move thisCreature .anyLocation (some battlefield) .arrival none [])) [] none [] none none
        (some (.did ⟨(.actor (a isCreature)), (.procedure .cast .any .itsOwnCost none none none), (.duration soFarThisTurn none)⟩)) (draw
          (.lit 1)))
      = [.kindMismatch .player .object] := by
  decide

/-- "target creature that entered this turn" -/
theorem okHappenedToObjectEntry :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (.and [isCreature, (.did ⟨.any, (.move .which .anyLocation (some battlefield) .arrival none []), (.duration soFarThisTurn none)⟩)])) = [] :=
        by
  decide

/-- "target creature who cast a spell this turn" -/
theorem badHappenedToObjectCast :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (.and [isCreature, (.did ⟨(.actor .which), (.procedure .cast .any .itsOwnCost none none none), (.duration soFarThisTurn none)⟩)]))
      = [.kindMismatch .player .object] := by
  decide

/-- "each opponent who died this turn" -/
theorem badHappenedToPlayerDied :
    NounPhrase.checkAt (some .player) (DRS.ofList [])
      (each (.and [(.isOpponentOf .you), (.did ⟨.any, (.move .which (battlefield) (some graveyard) .departure none []), (.duration soFarThisTurn none)⟩)])) = [.kindLte .player (.join .object .pile)] := by
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
        [ instruct .this (.dealDamage (.lit 3) (target (.or [isCreature, .anyPlayer])) .any),
          instruct .this (.dealDamage (.lit 1) (that .join) .any) ]) = [] := by
  decide

/-- "This deals 3 damage to that permanent or player." -/
theorem badUnionAnaphorNoAntecedent :
    Action.check (DRS.ofList []) (instruct .this (.dealDamage (.lit 3) (that .join) .any))
      = [.anaphor (.word .join) .one 0] := by
  decide

/-- "Destroy target creature. This deals 3 damage to that permanent or player." -/
theorem badUnionAnaphorOnObject :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isCreature), instruct .this (.dealDamage (.lit 3) (that .join) .any)])
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
  actionResult (DRS.ofList []) (instruct .this (.dealDamage (.lit 3) (target isAnyTarget) .any))

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

/-- "a creature with flyings" -/
theorem badClassOfParamlessKeyword :
    Predicate.checkAt .object (DRS.ofList []) (.hasKeyword (.anyIn ⟨(.named "flying"), none⟩)) = [.knownKeywordTerm] := by
  decide

/-- "a creature with renown of any color" -/
theorem badSortedClassOnNumberKeyword :
    Predicate.checkAt .object (DRS.ofList []) (.hasKeyword (.anyIn ⟨(.named "renown"), some .color⟩))
      = [.knownKeywordTerm] := by
  decide

/-- "{T}: Draw a card. Activate only if you created a creature this turn." -/
theorem okTokenCreationLookbackComplement :
    Condition.check (DRS.ofList [])
      (.did ⟨(.by .you), (.createObject .anyAmount (.token (.copyOf (a (.and [isCreature, .isToken])) []) []) false none), (.duration soFarThisTurn none)⟩) = [] := by
  decide

/-- "creature that was dealt damage by this creature this turn" -/
theorem okDamageTakenComplement :
    Predicate.checkAt .object (DRS.ofList [])
      (.did ⟨(.actor thisCreature), (.dealDamage .anyAmount .which .any), (.duration soFarThisTurn none)⟩) = [] := by
  decide

/-- "if you've cast a creature spell this turn" -/
theorem okCastComplementOnObject :
    Condition.check (DRS.ofList []) (.did ⟨(.actor .you), (.procedure .cast (a (.and [isCreature,
      isSpell])) .itsOwnCost none none none), (.duration soFarThisTurn none)⟩)
      = [] := by
  decide

theorem badPlayerCastComplement :
    Condition.check (DRS.ofList []) (.did ⟨(.actor .you), (.procedure .cast anOpponent .itsOwnCost none none none), (.duration soFarThisTurn none)⟩)
      = [.kindMismatch .object .player] := by
  decide

/-- "target spell cast from your graveyard" -/
theorem okCastFromGraveyard :
    Predicate.checkAt .object (DRS.ofList []) (.castFrom (graveyardOf .you)) = [] := by decide

/-- "… that was put somewhere from the battlefield this turn." -/
theorem okPlacementOriginBattlefield :
    Predicate.checkAt .object (DRS.ofList [])
      (.did ⟨.any, (.move .which (battlefield) none .departure none []), (.duration soFarThisTurn none)⟩)
      = [] := by
  decide

/-- "if you've cast a spell from your hand this turn" -/
theorem okCastOriginFromHand :
    Condition.check (DRS.ofList [])
      (.did ⟨(.actor .you), (.procedure .cast .any .itsOwnCost none (some (hand)) none), (.duration soFarThisTurn none)⟩)
        = [] := by
  decide

/-- "if you've cast a creature spell from your hand this turn" -/
theorem okOriginPayloadInvolving :
    Condition.check (DRS.ofList [])
      (.did ⟨(.actor .you), (.procedure .cast (a (.and [isCreature, isSpell])) .itsOwnCost none (some (hand)) none), (.duration soFarThisTurn none)⟩) = [] := by
  decide

/-- "… that died this turn." -/
theorem okDeathLookbackWithoutOrigin :
    Predicate.checkAt .object (DRS.ofList []) (.did ⟨.any, (.move .which (battlefield) (some graveyard) .departure none []), (.duration soFarThisTurn none)⟩) = [] := by decide

/-- "… that was put into a graveyard from the battlefield this turn." -/
theorem okPlacementIntoGraveyard :
    Predicate.checkAt .object (DRS.ofList [])
      (.did ⟨.any, (.move .which (battlefield) (some graveyard) .arrival none []), (.duration soFarThisTurn none)⟩) = [] :=
          by
  decide

/-- "… that was put into the battlefield this turn." -/
theorem okPlacementIntoBattlefield :
    Predicate.checkAt .object (DRS.ofList [])
      (.did ⟨.any, (.move .which .anyLocation (some battlefield) .arrival none []), (.duration soFarThisTurn none)⟩)
      = [] := by
  decide

/-- "if you shuffled your library this way" -/
theorem okShuffleLocusAtLibrary :
    Condition.check (DRS.ofList [])
      (.did ⟨(.actor .you), (.enact (.action "shuffle") (.shuffle (.zone .library (some .you))) none (some yourLibrary)), .thisWay⟩)
      = [] := by
  decide

/-- The search action carries its mandatory scope directly. -/
theorem searchLookbackCarriesItsScope :
    Condition.check (DRS.ofList [])
      (.did ⟨.actor .you, .search (yourLibrary) (.range none none) .isCard, .thisWay⟩)
      = [] := by decide

/-- "if you shuffled your graveyard this way" -/
theorem badShuffleLocusAtGraveyard :
    Condition.check (DRS.ofList [])
      (.did ⟨(.actor .you), (.enact (.action "shuffle") (.shuffle (.zone .library (some .you))) none (some (graveyardOf
          .you))), .thisWay⟩)
      = [.lookbackLocus] := by
  decide

/-- "target creature spell, once it resolves" -/
theorem okResolvedCreatureSpell :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.resolvedPermanent (a (.and [.hasType .creature, isSpell])))
      = [] := by
  decide

/-- "if it entered this turn" -/
theorem okEntryLookbackWithoutOrigin :
    Predicate.checkAt .object (DRS.ofList []) (.did ⟨.any, (.move .which .anyLocation (some battlefield) .arrival none []), (.duration soFarThisTurn none)⟩) = [] :=
      by decide

/-- "For each opponent, you draw a card." -/
theorem okPluralForEach :
    Action.check (DRS.ofList []) (.doForEach (each (.isOpponentOf .you)) (draw (.lit 1))) = [] := by
        decide

/-- "For each of target creature, its controller draws a card." -/
theorem badSingletonForEach :
    Action.check (DRS.ofList []) (.doForEach (target isCreature) (draw (.lit 1)))
      = [.plural] := by decide

/-- "For each player, choose target permanent that player controls. Those players draw a
card." (Vaevictis Asmadi, the Dire's opening loop): the group survives it, pluralised. -/
theorem okLoopGroupSurvives :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .doForEach (each .anyPlayer)
            (choose (target (.and [isPermanent, .hasPossessor .controller they]))),
          instruct (those .player) (draw (.lit 1)) ]) = [] := by
  decide

/-- "For each player, choose target permanent that player controls. That player draws a
card." The element the loop binds is body-local, so only the group reads back, and only as a
plural. -/
theorem badLoopElementRead :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .doForEach (each .anyPlayer)
            (choose (target (.and [isPermanent, .hasPossessor .controller they]))),
          instruct (that .player) (draw (.lit 1)) ]) = [.anaphor (.word .player) .one 0] := by
  decide

/-- "if you activated an activated ability this turn" -/
theorem okActivationLookbackComplement :
    Condition.check (DRS.ofList [])
      (.did ⟨(.actor .you), (.procedure .activate (a (.abilityHead .anyActivated)) .itsOwnCost none none none), (.duration soFarThisTurn none)⟩)
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

theorem copyParticipleUnwritten : participleOf (.core .copy) = none := by decide

/-- "Copy target instant or sorcery spell twice. You may choose new targets for those spells."
A copy of a spell is itself a spell [CR#707.10,112.1a], so the plural spell read reaches the
copies. -/
theorem okPluralSpellReadAfterCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.and [isInstantOrSorcery, isSpell])) (.lit 2) [],
          may (.chooseNewTargets (those .spell)) ]) = [] := by
  decide

/-- "Copy target instant or sorcery spell. You may choose new targets for the copy." -/
theorem okCopyReadAfterCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.and [isInstantOrSorcery, isSpell])) (.lit 1) [],
          may (.chooseNewTargets (that .copy)) ]) = [] := by
  decide

/-- "Copy target instant or sorcery spell. You may choose new targets for that spell."
The copy is itself a spell [CR#707.10]; AC>R1 selects its newer referent. -/
theorem singularSpellReadSelectsCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.and [isInstantOrSorcery, isSpell])) (.lit 1) [],
          may (.chooseNewTargets (that .spell)) ]) = [] := by
  decide

/-- "Copy target activated ability twice. You may choose new targets for those abilities." A
copy of an ability is itself an ability [CR#707.10], so the plural ability read reaches the
copies. -/
theorem okPluralAbilityReadAfterCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.abilityHead .anyActivated)) (.lit 2) [],
          may (.chooseNewTargets (those .ability)) ]) = [] := by
  decide

/-- "Copy target activated ability. You may choose new targets for that ability."
The copy is an ability [CR#707.10]; AC>R1 selects its newer referent. -/
theorem singularAbilityReadSelectsCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.abilityHead .anyActivated)) (.lit 1) [],
          may (.chooseNewTargets (that .ability)) ]) = [] := by
  decide

/-- "Copy target activated ability. You may choose new targets for the copy." -/
theorem okAbilityCopyReadAfterCopy :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.abilityHead .anyActivated)) (.lit 1) [],
          may (.chooseNewTargets (that (.copied .ability))) ]) = [] := by
  decide

/-- An ability goes on the stack with no card associated with it [CR#405.1]. -/
theorem abilityIsOnTheStack : Payload.zone (.ability none) = some .stack := by decide

theorem joinedCreatureTy :
    tyOfReach (.word .join) .one
      (actionResult (DRS.ofList []) (instruct .this (.dealDamage (.lit 3) (target (.or [isCreature, .anyPlayer])) .any)))
      = [.creature] := by
  decide

theorem anyTargetIsPlaceless : NounPhrase.zone (DRS.ofList []) (target isAnyTarget) = none := by decide

theorem anyTargetTakesDamage :
    Action.check (DRS.ofList [])
      (instruct .this (.dealDamage (.lit 1) (target isAnyTarget) .any)) = [] := by decide

theorem youAndPublishesBothParticipants :
    (nounResult (DRS.ofList []) (youAnd thisCreature)).referents.map (·.kind) =
      [.object, .player] := by decide

/-- "If a player is dealt damage this way, you draw a card." -/
theorem okDealtThisWayAfterDamage :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct .this (.dealDamage (.lit 3) (target isAnyTarget) .any),
          .doIf (.dealtThisWay .anyPlayer) (draw (.lit 1)) none ]) = [] := by
  decide

/-- "You draw a card. If a player is dealt damage this way, you draw a card." -/
theorem badDealtThisWayNoDamage :
    Action.check (DRS.ofList [])
      (.sequentially
        [ draw (.lit 1), .doIf (.dealtThisWay .anyPlayer) (draw (.lit 1)) none ])
      = [.damageDealtInScope] := by
  decide

/-- "the number of creatures that died this turn" -/
theorem okDeathTally :
    Amount.check (DRS.ofList []) (.tally ⟨.any, .move (a isCreature) (battlefield) (some graveyard) .departure none [], .duration soFarThisTurn none⟩ .events) = [] := by decide

/-- "the amount of creatures that died this turn" -/
theorem badDeathSum :
    Amount.check (DRS.ofList []) (.tally ⟨.any, .move (a isCreature) (battlefield) (some graveyard) .departure none [], .duration soFarThisTurn none⟩ .quantity) = [.tallyOk] := by
  decide

/-- "creature that died this turn" -/
theorem okObjectDeathLookbackSubject :
    Predicate.checkAt .object (DRS.ofList []) (.did ⟨.any, (.move .which (battlefield) (some graveyard) .departure none []), (.duration soFarThisTurn none)⟩) = [] := by decide

/-- "creature that won a coin flip this turn" -/
theorem badCreatureWonFlip :
    Predicate.checkAt .object (DRS.ofList []) (.did ⟨.actor .which, .flipCoins (.count (.lit 1)) (some .wins) none, .duration soFarThisTurn none⟩)
      = [.kindMismatch .player .object] := by
  decide

/-- "the number of dice you rolled this turn" -/
theorem okRollTally :
    Amount.check (DRS.ofList []) (.tally ⟨.actor .you, .rollDice (.lit 1) .anyDie .one .anyResult, .duration soFarThisTurn none⟩ .events) = [] := by decide

/-- "the amount of dice you rolled this turn" -/
theorem badRollAsMagnitude :
    Amount.check (DRS.ofList []) (.tally ⟨.actor .you, .rollDice (.lit 1) .anyDie .one .anyResult, .duration soFarThisTurn none⟩ .quantity) = [.tallyOk] := by decide

theorem youWonAFlipThisTurn :
    Condition.check (DRS.ofList []) (.did ⟨.actor .you, .flipCoins (.count (.lit 1)) (some .wins) none, .duration soFarThisTurn none⟩) =
      [] := by decide

theorem youRolledADieThisTurn :
    Condition.check (DRS.ofList []) (.did ⟨.actor .you, .rollDice (.lit 1) .anyDie .one .anyResult, .duration soFarThisTurn none⟩) = [] := by decide

/-- "Roll two d20. Ignore the lowest roll." -/
theorem okIgnoreAfterRoll :
    Action.check (DRS.ofList []) (.sequentially [rollDice (.lit 2) (.sides 20), .ignoreOutcomes (.extreme
        .lowest)])
      = [] := by
  decide

/-- "Ignore the lowest roll." -/
theorem badIgnoreWithoutRoll :
    Action.check (DRS.ofList []) (.ignoreOutcomes (.extreme .lowest)) = [.ignorableFor] := by decide

def afterATwoDieRoll : DRS := actionResult (DRS.ofList []) (rollDice (.lit 2) (.sides 6))

/-- "if you rolled doubles" -/
theorem okRolledDoublesAfterRoll : Condition.check afterATwoDieRoll .rolledDoubles = [] := by decide

/-- "If you rolled doubles, sacrifice this creature." -/
theorem badRolledDoublesWithoutRoll :
    Condition.check (DRS.ofList []) .rolledDoubles = [.outcomeInScope .rollResult 0] := by decide

def afterACoinFlip : DRS := actionResult (DRS.ofList []) (flipCoins (.lit 1))

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
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.rollDice (.lit 1) .anyDie .one (.resultIn (.range (some 4) none)))) = [] := by
  decide

/-- "Whenever you roll a 0, …" -/
theorem badZeroRollTest :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.rollDice (.lit 1) .anyDie .one (.resultIn (.range none (some 0)))))
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
    Condition.check (DRS.ofList []) (.did ⟨.actor .you, .pay (.perform (.changeLife (.down .anyAmount))) .once .paid none none, .duration soFarThisTurn none⟩) = [] := by decide

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

def afterALook : DRS := actionResult (DRS.ofList []) (lookAt (topSlice (.lit 1)))

/-- "Look at the top card of your library. Put that card into your graveyard." -/
theorem okReadsLookedAtCard : NounPhrase.checkAt (some .object) afterALook (that .card) = [] := by
  decide

def afterShuffledLook : DRS :=
  actionResult (DRS.ofList []) (.sequentially [lookAt (topSlice (.lit 1)), shuffle])

theorem badReadsShuffledLibraryCard :
    NounPhrase.checkAt (some .object) afterShuffledLook (that .card)
      = [.anaphor (.word .card) .one 0] := by
  decide

/-- "Whenever you scry, …" -/
theorem okPatientlessScry :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (scry (.lit 1))) = [] := by decide

/-- An unspecified event actor is represented explicitly by `Subject.any`. -/
theorem scryPatternAllowsUnspecifiedActor :
    GameEvent.check (DRS.ofList []) (.does .any (scry (.lit 1))) = [] := by decide

/-- The put deed supplies no standalone participial noun phrase. -/
theorem putHasNoStandaloneParticiple : participleOf (.core .put) = none := by decide

/-- "Whenever a creature transforms into a Phyrexian, …" -/
theorem okIntransitiveBecomes :
    GameEvent.check (DRS.ofList [])
      (.does .any (.enact (.action "transform") (.turnOver (a isCreature)) (some (.hasSubtype (creatureType "Phyrexian"))) none)) = [] := by
  decide

/-- "Whenever you discard a card, …" -/
theorem okDiscardFromHand :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.enact (.action "discard") (discard (a (.and [.isCard, .in_ hand]))) none none))
      = [] := by
  decide

/-- "if you drew a card this turn" -/
theorem okPlayerDrawLookback :
    Condition.check (DRS.ofList []) (.did ⟨(.actor .you), (.draw .anyAmount), (.duration soFarThisTurn none)⟩) = [] := by decide

/-- "if you dealt damage to an opponent this turn" -/
theorem badPlayerDamageDealer :
    Condition.check (DRS.ofList []) (.did ⟨(.actor .you), (.dealDamage .anyAmount .any .any), (.duration soFarThisTurn none)⟩)
      = [.kindMismatch .object .player] := by
  decide

theorem joinedDealerDamageComplement :
    Predicate.checkAt .object (DRS.ofList [])
      (.did ⟨.actor .which, .dealDamage .anyAmount (a (.or [isCreature, .anyPlayer])) .any, .duration soFarThisTurn none⟩)
      = [] := by decide

theorem lastChosenPlayerRead :
    Predicate.checkAt .player [ChoiceSort.binding .player, ChoiceSort.binding .player]
      theLastChosenPlayer = [] := by
  decide

/-- "... a chosen player. ... the chosen player." -/
theorem okDefiniteChosenPlayerRead :
    Action.check [ChoiceSort.binding .player]
      (.sequentially
        [ instruct .this (.dealDamage (.lit 3) (a isChosenPlayer) .any),
          instruct .this (.dealDamage (.lit 3) (the isChosenPlayer) .any) ])
      = [] := by
  decide

/-- "... a chosen player. ... that player." AC>R1 selects the player just mentioned. -/
theorem chosenPlayerDemonstrativeSelectsNewMention :
    Action.check [ChoiceSort.binding .player]
      (.sequentially
        [ instruct .this (.dealDamage (.lit 3) (a isChosenPlayer) .any), instruct .this (.dealDamage (.lit 3) they .any) ])
      = [] := by
  decide

/-- "that turn" after exactly one extra turn is minted -/
theorem okThatTurnAfterOneTurn :
    TurnRef.check (actionResult (DRS.ofList []) (extraTurn (.lit 1))) .that = [] := by
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
    anyOpenLetter .x (stepCost 64 (DRS.ofList []) (.loyaltySymbol .downX)).1 = true := by decide

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
          instruct (target isCreature) (.dealDamage (.statOf (.stat .power) it) (target isAnyTarget) .any) ])
      = [] := by
  decide

/-- "Draw cards equal to its power." -/
theorem badSingularReadOfBarePlural :
    Action.check (DRS.ofList [])
      (.sequentially
        [ get (bare (and [isCreature, youControl])) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
          draw (.statOf (.stat .power) it) ]) = [.anaphor .bare .one 0] := by
  decide

/-- The definite source remains available outside its negated property test. -/
theorem conditionItReadsCurrentSource :
    Action.check (DRS.ofList [])
      (.sequentially
        [ destroy (target isCreature),
          .doIf (.not (.matches thisEnchantment isCreature))
            (become it
              { characteristics := { types := [.creature], subtypes := [creatureType "Angel"] } }
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
        [ Templates.ptModification it
            (.up (.lit 1)) (.up (.lit 1)) ]
        none) = [] := by
  decide

/-- "Both target creature and target artifact get +1/+1", the shared subject read as "it":
ambiguous between the two, refused once. The corresponding modern shape is also pinned by
`GetsBothDeltas.ambiguousSingularPtSubjectIsRefused`. -/
theorem badSharedSubjectTwoInDelta :
    Action.check (DRS.ofList [])
      (establishFor (both (target isCreature) (target isArtifact))
        [ Templates.ptModification
            it
            (.up (.lit 1)) (.up (.lit 1)) ]
        none) = [.anaphor .bare .one 2] := by
  decide

theorem badOwnTwoInDelta :
    Action.check (DRS.ofList [])
      (dealDamageOwnPower (both (target isCreature) (target isCreature)) (target isAnyTarget))
      = [.singular] := by
  decide

theorem distributedDeedReadsBackPlural :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .sequentially [instruct (each (.isOpponentOf .you)) (choose (a (.and [.isCard, .in_ hand]))), discard them],
          exile (theVerbed (.action "discard") .many (some .card)) ]) = [] := by
  decide

/-- "Discard a card. Exile the discarded card." -/
theorem okTheVerbedAfterSingularDiscard :
    Action.check (DRS.ofList [])
      (.sequentially
        [ discard (a (.and [.isCard, .in_ yourHand])),
          exile (theVerbed (.action "discard") .one (some .card)) ])
      = [] := by
  decide

/-- "Each opponent discards a card. Exile that card." -/
theorem badDistributedDiscardSingular :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .sequentially [instruct (each (.isOpponentOf .you)) (choose (a (.and [.isCard, .in_ hand]))), discard them],
          exile (theVerbed (.action "discard") .one (some .card)) ])
      = [.anaphor (.verbed (.action "discard") (some .card) .attributive) .one 0] := by
  decide

/-- "Tap target creature." -/
theorem okTapBattlefieldPermanent :
    Action.check (DRS.ofList []) (.setStatus .tapped (target isCreature)) = [] := by decide

/-- "if there is no monarch" -/
theorem okNoHolderOnPlayer : Condition.check (DRS.ofList []) (thereIsNo (.named "monarch")) = [] := by decide

/-- "if there is no monstrous creature" -/
theorem okNoHolderOnObject :
    Condition.check (DRS.ofList [])
      (.not (exists_ (and [isCreature, .hasDesignation (.named "monstrous") none]))) = [] := by
  decide

/-- "if there is no monarch creature": a player designation on an object is refused. -/
theorem badPlayerDesignationOnObject :
    Condition.check (DRS.ofList [])
      (.not (exists_ (and [isCreature, .hasDesignation (.named "monarch") none]))) =
      [.designationHolder (.named "monarch") .object] := by
  decide

/-- "Roll five d6. Store those results on this creature." -/
theorem okStoreResultsAfterRoll :
    Action.check (DRS.ofList []) (.sequentially [rollDice (.lit 5) (.sides 6), .storeResults thisCreature]) = []
        := by
  decide

/-- "Store those results on this creature." -/
theorem badStoreResultsWithoutRoll :
    Action.check (DRS.ofList []) (.storeResults thisCreature) = [.outcomeInScope .rollResult 0] := by decide

/-- "If you would roll one or more d6, instead roll that many of those dice." -/
theorem okThoseDiceAfterRollEvent :
    Action.check (DRS.ofList [])
      (replaceEvent (.would (.actor .you) (.rollDice .anyAmount (.sides 6) .many .anyResult))
        (instead (.rollDice .thatMuch .thoseDice .many .anyResult)) none) = [] := by
  decide

/-- "Roll that many dice." -/
theorem badAnaphoricSidesWithoutRoll :
    Action.check (DRS.ofList []) (.rollDice (.lit 1) .thoseDice .one .anyResult)
      = [.outcomeInScope .diceRolled 0] := by
  decide

theorem singularSpellReadHasCopyIdentity :
    let after := actionResult (DRS.ofList [])
      (.copy .fromStack (target (.and [isInstantOrSorcery, isSpell])) (.lit 1) [])
    let selected := resolveCandidates (.word .spell) .one (nextClause after)
    selected.map (·.payload.orig) = [some .copy] ∧
      selected.map (·.id) = [after.nextId - 1] := by decide

theorem singularAbilityReadHasCopyIdentity :
    let after := actionResult (DRS.ofList [])
      (.copy .fromStack (target (.abilityHead .anyActivated)) (.lit 1) [])
    let selected := resolveCandidates (.word .ability) .one (nextClause after)
    selected.map (·.payload.orig) = [some .copy] ∧
      selected.map (·.id) = [after.nextId - 1] := by decide

theorem chosenPlayerDemonstrativeHasNewMentionIdentity :
    let before := DRS.ofList [ChoiceSort.binding .player]
    let after := actionResult before (instruct .this (.dealDamage (.lit 3) (a isChosenPlayer) .any))
    (resolveCandidates (.word .player) .one (nextClause after)).map (·.id) =
      [after.nextId - 2] := by decide

theorem possessiveItHasCurrentSubjectIdentity :
    let after := actionResult (DRS.ofList [])
      (.sequentially [exile (target isArtifact),
        instruct (.firstMention "p05.subject" (target isCreature))
          (.dealDamage (.statOf (.stat .power) it) (target isAnyTarget) .any)])
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
        .dealDamage (.lit 2) (controllerOf (that .permanent)) .noncombatOnly,
        untap (that .permanent)]) = [.anaphor (.word .permanent) .one 0] := by decide

end Semantics.Proofs.Anaphora
