import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Anaphora

Port of `idris/src/Experimental/Proofs/Anaphora.idr`: the pins of the Anaphora family, in
theorem form (a twin `= []`, a pin `= [r]`), each closed by `decide`. The names and the
sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Anaphora

private def instructionResult (drs : DRS) (i : Instruction) : DRS :=
  (stepInstruction 64 .postEvent .obj drs i).1
private def nounResult (drs : DRS) (n : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .obj true n).1


/-- "Choose two — Draw a card; draw a card." [CR#700.2d] -/
theorem identicalModesAllowed :
    Instruction.check (DRS.ofList []) (chooseModes (exactly 2) [draw (.lit 1), draw (.lit 1)])
      = [] := by
  decide

/-- "Choose a player or planeswalker." -/
theorem okChoosePlayerOrPlaneswalker :
    Instruction.check (DRS.ofList [])
      (.choose none (a (.or [.hasType .planeswalker, .anyPlayer])) .openly none) =
          [] := by
  decide

/-- "Choose you." -/
theorem badChooseYou :
    Instruction.check (DRS.ofList []) (.choose none .you .openly none) = [.choiceClause] := by
        decide

theorem badConditionalArmAntecedent :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .doOnlyIf (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"]))
            (exists_ creatureYouControl) none,
          .putCounters (.lit 1) (.printed p1p1Counter) it .one true ]) = [.anaphor .bare .one 0] := by
  decide

theorem badBothArmsAntecedent :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ Primitives.Instruction.offer (gainLife (.lit 1))
            (some (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"])))
            (some (create (.lit 2) (creatureToken 1 1 [.white] [creatureType "Soldier"]))),
          .putCounters (.lit 1) (.printed p1p1Counter) it .one true ]) = [.anaphor .bare .one 0] := by
  decide

/-- "Put target creature onto the battlefield." -/
theorem okMoveToBattlefield :
    Instruction.check (DRS.ofList []) (.move (target creature) .anywhere (some battlefield) .arrival none []) = [] := by decide

/-- "Look at the top four cards of your library. You choose one of them." -/
theorem okAgentChoiceOfSome :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice (.lit 4)),
          .choose none (someOf (exactly 1) them) .openly none ]) = [] := by
  decide

/-- "Look at the top four cards of your library. Choose one of them." The chooser is the actor
in context, so the bare imperative is the same term as "You choose one of them." -/
theorem okChooseSomeOf :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice (.lit 4)),
          .choose none (someOf (exactly 1) them) .openly none ]) = [] := by
  decide

/-- "Exile target creature." -/
theorem okMoveToExile :
    Instruction.check (DRS.ofList []) (.move (target creature) .anywhere (some exileZone) .arrival none []) = [] := by decide

/-- "When this creature enters, if a creature died this turn, draw a card." -/
theorem okLookbackObjectDied :
    Ability.check (DRS.ofList [])
      (.triggered (.does .any (.move thisCreature .anywhere (some battlefield) .arrival none [])) [] none [] none none
        (some (.did ⟨.any, (.move (.asMarker .permanent (a creature)) (.zones [battlefield]) (some graveyard) .departure none []), (.duration soFarThisTurn none)⟩)) (draw (.lit 1)))
            = [] := by
  decide

/-- "When this creature enters, if you died this turn, draw a card." -/
theorem badLookbackPlayerDied :
    Ability.check (DRS.ofList [])
      (.triggered (.does .any (.move thisCreature .anywhere (some battlefield) .arrival none [])) [] none [] none none
        (some (.did ⟨.any, (.move (.asMarker .permanent .you) (.zones [battlefield]) (some graveyard) .departure none []), (.duration soFarThisTurn none)⟩)) (draw
          (.lit 1)))
      = [.kindMismatch .object .player] := by
  decide

theorem badLookbackObjectCast :
    Ability.check (DRS.ofList [])
      (.triggered (.does .any (.move thisCreature .anywhere (some battlefield) .arrival none [])) [] none [] none none
        (some (.did ⟨(.actor (a creature)), (.procedure .cast .any .itsOwnCost none none none), (.duration soFarThisTurn none)⟩)) (draw
          (.lit 1)))
      = [.kindMismatch .player .object] := by
  decide

/-- "target creature that entered this turn" -/
theorem okHappenedToObjectEntry :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (.and [creature, (.did ⟨.any, (.move .which .anywhere (some battlefield) .arrival none []), (.duration soFarThisTurn none)⟩)])) = [] :=
        by
  decide

/-- "target creature who cast a spell this turn" -/
theorem badHappenedToObjectCast :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (.and [creature, (.did ⟨(.actor .which), (.procedure .cast .any .itsOwnCost none none none), (.duration soFarThisTurn none)⟩)]))
      = [.kindMismatch .player .object] := by
  decide

/-- "each opponent who died this turn" -/
theorem badHappenedToPlayerDied :
    NounPhrase.checkAt (some .player) (DRS.ofList [])
      (each (.and [.opponent, (.did ⟨.any, (.move (.asMarker .permanent .which) (.zones [battlefield]) (some graveyard) .departure none []), (.duration soFarThisTurn none)⟩)])) = [.kindMismatch .object .player] := by
  decide

/-- "Choose a creature you control." -/
theorem okChooseIndefinite :
    Instruction.check (DRS.ofList []) (.choose none (a creatureYouControl) .openly none) = [] :=
        by decide

/-- "Choose the creature with the least toughness among creatures you control." -/
theorem badChooseDefinite :
    Instruction.check (DRS.ofList [])
      (.choose none
        (the (.and [creature, .superlative .min (.stat .toughness) creatureYouControl]))
        .openly none) = [.choiceClause] := by
  decide

/-- "This deals 1 damage to that permanent or player." -/
theorem okUnionAnaphorAfterJoin :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ instruct .this (.dealDamage (.lit 3) (target (.or [creature, .anyPlayer])) .any),
          instruct .this (.dealDamage (.lit 1) (that .join) .any) ]) = [] := by
  decide

/-- "This deals 3 damage to that permanent or player." -/
theorem badUnionAnaphorNoAntecedent :
    Instruction.check (DRS.ofList []) (instruct .this (.dealDamage (.lit 3) (that .join) .any))
      = [.anaphor (.word .join) .one 0] := by
  decide

/-- "Destroy target creature. This deals 3 damage to that permanent or player." -/
theorem badUnionAnaphorOnObject :
    Instruction.check (DRS.ofList [])
      (.sequentially [destroy (target creature), instruct .this (.dealDamage (.lit 3) (that .join) .any)])
      = [.anaphor (.word .join) .one 0] := by
  decide

/-- A countered ability has left the stack. A subsequent stack-zone reference cannot
reach it [CR#701.6a]. -/
theorem badStackAnaphorAfterCounteringAbility :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ Primitives.Instruction.counterSpell (target (.abilityHead .anyActivated)), Primitives.Instruction.counterSpell (that .stack) ])
      = [.anaphor (.word .stack) .one 0] := by
  decide

def afterAnyTargetDamage : Bindings :=
  instructionResult (DRS.ofList []) (instruct .this (.dealDamage (.lit 3) (target anyTarget) .any))

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
      (.did ⟨(.by .you), (.createObject .anyAmount (.token (.copyOf (a (.and [creature, .isToken])) []) []) false none), (.duration soFarThisTurn none)⟩) = [] := by
  decide

/-- "creature that was dealt damage by this creature this turn" -/
theorem okDamageTakenComplement :
    Predicate.checkAt .object (DRS.ofList [])
      (.did ⟨(.actor thisCreature), (.dealDamage .anyAmount (.asMarker .permanent .which) .any), (.duration soFarThisTurn none)⟩) = [] := by
  decide

/-- "if you've cast a creature spell this turn" -/
theorem okCastComplementOnObject :
    Condition.check (DRS.ofList []) (.did ⟨(.actor .you), (.procedure .cast (a (.and [creature,
      spell])) .itsOwnCost none none none), (.duration soFarThisTurn none)⟩)
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
      (.did ⟨.any, (.move .which (.zones [battlefield]) none .departure none []), (.duration soFarThisTurn none)⟩)
      = [] := by
  decide

/-- "if you've cast a spell from your hand this turn" -/
theorem okCastOriginFromHand :
    Condition.check (DRS.ofList [])
      (.did ⟨(.actor .you), (.procedure .cast .any .itsOwnCost none (some (.zones [hand])) none), (.duration soFarThisTurn none)⟩)
        = [] := by
  decide

/-- "if you've cast a creature spell from your hand this turn" -/
theorem okOriginPayloadInvolving :
    Condition.check (DRS.ofList [])
      (.did ⟨(.actor .you), (.procedure .cast (a (.and [creature, spell])) .itsOwnCost none (some (.zones [hand])) none), (.duration soFarThisTurn none)⟩) = [] := by
  decide

/-- "… that died this turn." -/
theorem okDeathLookbackWithoutOrigin :
    Predicate.checkAt .object (DRS.ofList []) (.did ⟨.any, (.move (.asMarker .permanent .which) (.zones [battlefield]) (some graveyard) .departure none []), (.duration soFarThisTurn none)⟩) = [] := by decide

/-- "… that was put into a graveyard from the battlefield this turn." -/
theorem okPlacementIntoGraveyard :
    Predicate.checkAt .object (DRS.ofList [])
      (.did ⟨.any, (.move .which (.zones [battlefield]) (some graveyard) .arrival none []), (.duration soFarThisTurn none)⟩) = [] :=
          by
  decide

/-- "… that was put into the battlefield this turn." -/
theorem okPlacementIntoBattlefield :
    Predicate.checkAt .object (DRS.ofList [])
      (.did ⟨.any, (.move .which .anywhere (some battlefield) .arrival none []), (.duration soFarThisTurn none)⟩)
      = [] := by
  decide

/-- "if you shuffled your library this way" -/
theorem okShuffleLocusAtLibrary :
    Condition.check (DRS.ofList [])
      (.did ⟨(.actor .you), (.enact (.action "shuffle") (.shuffle (.library (.possessedBy .you))) none (some yourLibrary)), .thisWay⟩)
      = [] := by
  decide

/-- The search action carries its mandatory scope directly. -/
theorem searchLookbackCarriesItsScope :
    Condition.check (DRS.ofList [])
      (.did ⟨.actor .you, .search (.oneZone yourLibrary) (.range none none) .isCard, .thisWay⟩)
      = [] := by decide

/-- "if you shuffled your graveyard this way" -/
theorem badShuffleLocusAtGraveyard :
    Condition.check (DRS.ofList [])
      (.did ⟨(.actor .you), (.enact (.action "shuffle") (.shuffle (.library (.possessedBy .you))) none (some (graveyardOf
          .you))), .thisWay⟩)
      = [.lookbackLocus] := by
  decide

/-- "target creature spell, once it resolves" -/
theorem okResolvedCreatureSpell :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.resolvedPermanent (a (.and [.hasType .creature, spell])))
      = [] := by
  decide

theorem badResolvedInstant :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.resolvedPermanent (a (.and [.hasType .instant, spell])))
      = [.permanentSpellType] := by
  decide

/-- "if it entered this turn" -/
theorem okEntryLookbackWithoutOrigin :
    Predicate.checkAt .object (DRS.ofList []) (.did ⟨.any, (.move .which .anywhere (some battlefield) .arrival none []), (.duration soFarThisTurn none)⟩) = [] :=
      by decide

/-- "For each opponent, you draw a card." -/
theorem okPluralForEach :
    Instruction.check (DRS.ofList []) (.doForEach (each .opponent) (draw (.lit 1))) = [] := by
        decide

/-- "For each of target creature, its controller draws a card." -/
theorem badSingletonForEach :
    Instruction.check (DRS.ofList []) (.doForEach (target creature) (draw (.lit 1)))
      = [.plural] := by decide

/-- "For each player, choose target permanent that player controls. Those players draw a
card." (Vaevictis Asmadi, the Dire's opening loop): the group survives it, pluralised. -/
theorem okLoopGroupSurvives :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .doForEach (each .anyPlayer)
            (choose (target (.and [permanent, .hasPossessor .controller they]))),
          instruct (those .player) (draw (.lit 1)) ]) = [] := by
  decide

/-- "For each player, choose target permanent that player controls. That player draws a
card." The element the loop binds is body-local, so only the group reads back, and only as a
plural. -/
theorem badLoopElementRead :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .doForEach (each .anyPlayer)
            (choose (target (.and [permanent, .hasPossessor .controller they]))),
          instruct (that .player) (draw (.lit 1)) ]) = [.anaphor (.word .player) .one 0] := by
  decide

/-- "if you activated an activated ability this turn" -/
theorem okActivationLookbackComplement :
    Condition.check (DRS.ofList [])
      (.did ⟨(.actor .you), (.procedure .activate (a (.abilityHead .anyActivated)) .itsOwnCost none none none), (.duration soFarThisTurn none)⟩)
      = [] := by
  decide

/-- "on top of your library in any order, third from the top": the place, and its order and
depth as the move's riders. -/
theorem okOneEndArrangedOrdinal :
    ZoneExpr.check (DRS.ofList []) onTop ++ MoveRider.checkAll (DRS.ofList []) (some .placement) [.placement (.inOrder .anyOrder), .placement (.nthFromEnd (.nth 3))]
      = [] := by
  decide

/-- "Shuffle those cards into your library.": the library they go into, then its shuffle. -/
theorem okShuffledPlain :
    ZoneExpr.check (DRS.ofList []) (.zone .library .bare) ++ Instruction.check (DRS.ofList []) (.shuffle (.library .bare))
      = [] := by decide

/-- "third from the top" -/
theorem okThirdFromTop : PilePlacement.check (DRS.ofList []) (.nthFromEnd (.nth 3)) = [] := by decide

/-- "zeroth from the top" -/
theorem badZerothFromTop :
    PilePlacement.check (DRS.ofList []) (.nthFromEnd (.nth 0)) = [.ordinalNonZero] := by
  decide

theorem copyParticipleUnwritten : participleOf (.core .copy) = none := by decide

/-- "Copy target instant or sorcery spell twice. You may choose new targets for those spells."
A copy of a spell is itself a spell [CR#707.10,112.1a], so the plural spell read reaches the
copies. -/
theorem okPluralSpellReadAfterCopy :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.and [instantOrSorcery, spell])) (.lit 2) [],
          offer (.chooseNewTargets (those .spell)) ]) = [] := by
  decide

/-- "Copy target instant or sorcery spell. You may choose new targets for the copy." -/
theorem okCopyReadAfterCopy :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.and [instantOrSorcery, spell])) (.lit 1) [],
          offer (.chooseNewTargets (that .copy)) ]) = [] := by
  decide

/-- "Copy target instant or sorcery spell. You may choose new targets for that spell."
The copy is itself a spell [CR#707.10]; AC>R1 selects its newer referent. -/
theorem singularSpellReadSelectsCopy :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.and [instantOrSorcery, spell])) (.lit 1) [],
          offer (.chooseNewTargets (that .spell)) ]) = [] := by
  decide

/-- "Copy target activated ability twice. You may choose new targets for those abilities." A
copy of an ability is itself an ability [CR#707.10], so the plural ability read reaches the
copies. -/
theorem okPluralAbilityReadAfterCopy :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.abilityHead .anyActivated)) (.lit 2) [],
          offer (.chooseNewTargets (those .ability)) ]) = [] := by
  decide

/-- "Copy target activated ability. You may choose new targets for that ability."
The copy is an ability [CR#707.10]; AC>R1 selects its newer referent. -/
theorem singularAbilityReadSelectsCopy :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.abilityHead .anyActivated)) (.lit 1) [],
          offer (.chooseNewTargets (that .ability)) ]) = [] := by
  decide

/-- "Copy target activated ability. You may choose new targets for the copy." -/
theorem okAbilityCopyReadAfterCopy :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.abilityHead .anyActivated)) (.lit 1) [],
          offer (.chooseNewTargets (that (.copied .ability))) ]) = [] := by
  decide

/-- An ability goes on the stack with no card associated with it [CR#405.1]. -/
theorem abilityIsOnTheStack : Payload.zone (.ability none) = some .stack := by decide

theorem joinedCreatureTy :
    tyOfReach (.word .join) .one
      (instructionResult (DRS.ofList []) (instruct .this (.dealDamage (.lit 3) (target (.or [creature, .anyPlayer])) .any)))
      = [.creature] := by
  decide

theorem anyTargetIsPlaceless : NounPhrase.zone (DRS.ofList []) (target anyTarget) = none := by decide

theorem anyTargetTakesDamage :
    Instruction.check (DRS.ofList [])
      (instruct .this (.dealDamage (.lit 1) (target anyTarget) .any)) = [] := by decide

theorem youAndPublishesBothParticipants :
    (nounResult (DRS.ofList []) (youAnd thisCreature)).referents.map (·.kind) =
      [.object, .player] := by decide

/-- "If a player is dealt damage this way, you draw a card." -/
theorem okDealtThisWayAfterDamage :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ instruct .this (.dealDamage (.lit 3) (target anyTarget) .any),
          .doIf (.dealtThisWay .anyPlayer) (draw (.lit 1)) none ]) = [] := by
  decide

/-- "You draw a card. If a player is dealt damage this way, you draw a card." -/
theorem badDealtThisWayNoDamage :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ draw (.lit 1), .doIf (.dealtThisWay .anyPlayer) (draw (.lit 1)) none ])
      = [.damageDealtInScope] := by
  decide

/-- "the number of creatures that died this turn" -/
theorem okDeathTally :
    Amount.check (DRS.ofList []) (.tally ⟨.any, .move (a creature) (.zones [battlefield]) (some graveyard) .departure none [], .duration soFarThisTurn none⟩ .events) = [] := by decide

/-- "the amount of creatures that died this turn" -/
theorem badDeathSum :
    Amount.check (DRS.ofList []) (.tally ⟨.any, .move (a creature) (.zones [battlefield]) (some graveyard) .departure none [], .duration soFarThisTurn none⟩ .quantity) = [.tallyOk] := by
  decide

/-- "creature that died this turn" -/
theorem okObjectDeathLookbackSubject :
    Predicate.checkAt .object (DRS.ofList []) (.did ⟨.any, (.move (.asMarker .permanent .which) (.zones [battlefield]) (some graveyard) .departure none []), (.duration soFarThisTurn none)⟩) = [] := by decide

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
    Instruction.check (DRS.ofList []) (.sequentially [rollDice 2 20, .ignoreOutcomes (.extreme
        .lowest)])
      = [] := by
  decide

/-- "Ignore the lowest roll." -/
theorem badIgnoreWithoutRoll :
    Instruction.check (DRS.ofList []) (.ignoreOutcomes (.extreme .lowest)) = [.ignorableFor] := by decide

def afterATwoDieRoll : Bindings := instructionResult (DRS.ofList []) (rollDice 2 6)

/-- "if you rolled doubles" -/
theorem okRolledDoublesAfterRoll : Condition.check afterATwoDieRoll .rolledDoubles = [] := by decide

/-- "If you rolled doubles, sacrifice this creature." -/
theorem badRolledDoublesWithoutRoll :
    Condition.check (DRS.ofList []) .rolledDoubles = [.outcomeInScope .rollResult 0] := by decide

def afterACoinFlip : Bindings := instructionResult (DRS.ofList []) (flipCoins 1)

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
    Instruction.check afterATwoDieRoll (.ignoreOutcomes (.extreme .lowest)) = [] := by decide

/-- "If you would flip a coin, instead flip two coins and ignore the lower one." -/
theorem badExtremeOverFlips :
    Instruction.check afterACoinFlip (.ignoreOutcomes (.extreme .lowest)) = [.ignorableFor] := by
  decide

/-- "When a player doesn't pay this creature's cumulative upkeep, …" -/
theorem okPayKeywordWithACost :
    GameEvent.check (DRS.ofList []) (.does (.actor (a .anyPlayer)) (.pay .anyCost .once .unpaid (some thisCreature) (some (.named "cumulativeUpkeep"))))
      = [] := by
  decide

/-- "if you paid life this turn" -/
theorem youPaidLifeThisTurn :
    Condition.check (DRS.ofList []) (.did ⟨.actor .you, .pay (.perform (.changeLife (.down .anyAmount))) .once .paid none none, .duration soFarThisTurn none⟩) = [] := by decide

def afterAnUpToChoice : Bindings := instructionResult (DRS.ofList []) (choose (counted (upTo 1) creature))

/-- "Choose up to one creature. Destroy the rest." -/
theorem okRestAfterAPartition :
    NounPhrase.checkAt (some .object) afterAnUpToChoice (theRest .object) = [] := by decide

/-- "Destroy the rest." -/
theorem badRestWithoutAPartition :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (theRest .object) = [.theRestFits .object] := by decide

/-- "Choose up to one creature. Destroy the rest." -/
theorem okRestAfterCountedChoice :
    Instruction.check (DRS.ofList [])
      (.sequentially [choose (counted (upTo 1) creature), destroy (theRest .object)]) = [] := by
  decide

/-- "Choose any number of target creatures. Destroy the rest." -/
theorem badRestAfterTargetChoice :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [choose (.described (.target anyNumber) creature), destroy (theRest .object)])
      = [.theRestFits .object] := by
  decide

def afterALook : Bindings := instructionResult (DRS.ofList []) (lookAt (topSlice (.lit 1)))

/-- "Look at the top card of your library. Put that card into your graveyard." -/
theorem okReadsLookedAtCard : NounPhrase.checkAt (some .object) afterALook (that .card) = [] := by
  decide

def afterShuffledLook : Bindings :=
  instructionResult (DRS.ofList []) (.sequentially [lookAt (topSlice (.lit 1)), shuffle])

theorem badReadsShuffledLibraryCard :
    NounPhrase.checkAt (some .object) afterShuffledLook (that .card)
      = [.anaphor (.word .card) .one 0] := by
  decide

/-- "Whenever you scry, …" -/
theorem okPatientlessScry :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (scry (.lit 1))) = [] := by decide

/-- Scry has no patient slot; its written parameter is an amount. -/
theorem scryHasNoPatientRole :
    ((deedFacts (.action "scry")).bind (fun facts => facts.patientRole.sort)).isNone = true := by decide

/-- An unspecified event actor is represented explicitly by `Subject.any`. -/
theorem scryPatternAllowsUnspecifiedActor :
    GameEvent.check (DRS.ofList []) (.does .any (scry (.lit 1))) = [] := by decide

/-- The put deed supplies no standalone participial noun phrase. -/
theorem putHasNoStandaloneParticiple : participleOf (.core .put) = none := by decide

/-- "Whenever a creature transforms into a Phyrexian, …" -/
theorem okIntransitiveBecomes :
    GameEvent.check (DRS.ofList [])
      (.does .any (.enact (.action "transform") (.turnOver (a creature)) (some (.hasSubtype (creatureType "Phyrexian"))) none)) = [] := by
  decide

/-- "Whenever a card is milled into a Phyrexian, …" -/
theorem badBecomesWithoutIntransitive :
    GameEvent.check (DRS.ofList [])
      (.does .any (.enact (.action "mill") (mill (.lit 1) .you) (some (.hasSubtype (creatureType "Phyrexian"))) none)) = [.verbBecomesOk] := by
  decide

/-- "Whenever you discard a card, …" -/
theorem okDiscardFromHand :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.enact (.action "discard") (discard (a (.and [.isCard, .inZone hand]))) none none))
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
      (.did ⟨.actor .which, .dealDamage .anyAmount (a (.or [creature, .anyPlayer])) .any, .duration soFarThisTurn none⟩)
      = [] := by decide

theorem lastChosenPlayerRead :
    Predicate.checkAt .player [ChoiceSort.binding .player, ChoiceSort.binding .player]
      theLastChosenPlayer = [] := by
  decide

/-- "... a chosen player. ... the chosen player." -/
theorem okDefiniteChosenPlayerRead :
    Instruction.check [ChoiceSort.binding .player]
      (.sequentially
        [ instruct .this (.dealDamage (.lit 3) (a chosenPlayer) .any),
          instruct .this (.dealDamage (.lit 3) (the chosenPlayer) .any) ])
      = [] := by
  decide

/-- "... a chosen player. ... that player." AC>R1 selects the player just mentioned. -/
theorem chosenPlayerDemonstrativeSelectsNewMention :
    Instruction.check [ChoiceSort.binding .player]
      (.sequentially
        [ instruct .this (.dealDamage (.lit 3) (a chosenPlayer) .any), instruct .this (.dealDamage (.lit 3) they .any) ])
      = [] := by
  decide

/-- "that turn" after exactly one extra turn is minted -/
theorem okThatTurnAfterOneTurn :
    TurnRef.check (instructionResult (DRS.ofList []) (extraTurn (.lit 1))) .that = [] := by
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
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ exile (target artifact),
          dealDamageOwnPower (target creature)
            (target anyTarget) ]) = [] := by
  decide

/-- "Exile target creature. Draw cards equal to its power." -/
theorem okItReadsTheOnlyBareSingular :
    Instruction.check (DRS.ofList [])
      (.sequentially [exile (target creature), draw (.statOf (.stat .power) it)]) = []
          := by
  decide

theorem possessiveItReadsCurrentSubject :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ exile (target artifact),
          instruct (target creature) (.dealDamage (.statOf (.stat .power) it) (target anyTarget) .any) ])
      = [] := by
  decide

/-- "Draw cards equal to its power." -/
theorem badSingularReadOfBarePlural :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ get (bare creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
          draw (.statOf (.stat .power) it) ]) = [.anaphor .bare .one 0] := by
  decide

/-- "Destroy target creature. If this enchantment isn't a creature, it becomes an Angel
creature until end of turn." The "it" reads the condition's own subject, which is all a
condition publishes. -/
theorem okCondSubjectRead :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ destroy (target creature),
          .doIf (.not (.matches thisEnchantment creature))
            (become
              it
              { characteristics := { types := [.creature], subtypes := [creatureType "Angel"] } }
              (some untilEndOfTurn))
            none ]) = [] := by
  decide

/-- The definite source remains available outside its negated property test. -/
theorem conditionItReadsCurrentSource :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ destroy (target creature),
          .doIf (.not (.matches thisEnchantment creature))
            (become it
              { characteristics := { types := [.creature], subtypes := [creatureType "Angel"] } }
              (some untilEndOfTurn))
            none ]) = [] := by
  decide

theorem okOwnPowerWithoutNewMention :
    Instruction.check (DRS.ofList []) (dealDamageOwnPower .this (target anyTarget))
      = [] := by
  decide

/-- "Target creature gets +1/+1": the shared subject read once, by the one stat change. (The
two-modification spelling it had is `GetsBothDeltas.okOwnReadsOneInDeltaTwoHalves`.) -/
theorem okOwnReadsOneInDelta :
    Instruction.check (DRS.ofList [])
      (establishFor (target creature)
        [ .ptModification it
            (.up (.lit 1)) (.up (.lit 1)) ]
        none) = [] := by
  decide

/-- "Both target creature and target artifact get +1/+1", the shared subject read as "it":
ambiguous between the two, refused once. (The two-modification spelling it had, refused once
per half, is `GetsBothDeltas.badSharedSubjectTwoInDeltaTwoHalves`.) -/
theorem badSharedSubjectTwoInDelta :
    Instruction.check (DRS.ofList [])
      (establishFor (Primitives.NounPhrase.both (target creature) (target artifact))
        [ .ptModification
            it
            (.up (.lit 1)) (.up (.lit 1)) ]
        none) = [.anaphor .bare .one 2] := by
  decide

theorem badOwnTwoInDelta :
    Instruction.check (DRS.ofList [])
      (dealDamageOwnPower (Primitives.NounPhrase.both (target creature) (target creature)) (target anyTarget))
      = [.singular] := by
  decide

theorem distributedDeedReadsBackPlural :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .sequentially [instruct (each .opponent) (choose (a (.and [.isCard, .inZone hand]))), discard them],
          exile (theVerbed (.action "discard") .many (some .card)) ]) = [] := by
  decide

/-- "Discard a card. Exile the discarded card." -/
theorem okTheVerbedAfterSingularDiscard :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ discard (a (.and [.isCard, .inZone yourHand])),
          exile (theVerbed (.action "discard") .one (some .card)) ])
      = [] := by
  decide

/-- "Each opponent discards a card. Exile that card." -/
theorem badDistributedDiscardSingular :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .sequentially [instruct (each .opponent) (choose (a (.and [.isCard, .inZone hand]))), discard them],
          exile (theVerbed (.action "discard") .one (some .card)) ])
      = [.anaphor (.verbed (.action "discard") (some .card) .attributive) .one 0] := by
  decide

/-- "Tap target creature." -/
theorem okTapBattlefieldPermanent :
    Instruction.check (DRS.ofList []) (.setStatus .tapped (target creature)) = [] := by decide

/-- "if there is no monarch" -/
theorem okNoHolderOnPlayer : Condition.check (DRS.ofList []) (.noHolder (.named "monarch")) = [] := by decide

/-- "if there is no monstrous creature" -/
theorem badNoHolderOnObject :
    Condition.check (DRS.ofList []) (.noHolder (.named "monstrous")) = [.designationScope (.named "monstrous")] := by decide

/-- "Roll five d6. Store those results on this creature." -/
theorem okStoreResultsAfterRoll :
    Instruction.check (DRS.ofList []) (.sequentially [rollDice 5 6, .storeResults thisCreature]) = []
        := by
  decide

/-- "Store those results on this creature." -/
theorem badStoreResultsWithoutRoll :
    Instruction.check (DRS.ofList []) (.storeResults thisCreature) = [.outcomeInScope .rollResult 0] := by decide

/-- "If you would roll one or more d6, instead roll that many of those dice." -/
theorem okThoseDiceAfterRollEvent :
    Instruction.check (DRS.ofList [])
      (replaceEvent (.would (.actor .you) (.rollDice .anyAmount (.sides 6) .many .anyResult))
        (.rollDice .thatMuch .thoseDice .many .anyResult) none) = [] := by
  decide

/-- "Roll that many dice." -/
theorem badAnaphoricSidesWithoutRoll :
    Instruction.check (DRS.ofList []) (.rollDice (.lit 1) .thoseDice .one .anyResult)
      = [.outcomeInScope .diceRolled 0] := by
  decide

theorem singularSpellReadHasCopyIdentity :
    let after := instructionResult (DRS.ofList [])
      (.copy .fromStack (target (.and [instantOrSorcery, spell])) (.lit 1) [])
    let selected := resolveCandidates (.word .spell) .one (nextClause after)
    selected.map (·.payload.orig) = [some .copy] ∧
      selected.map (·.id) = [after.nextId - 1] := by decide

theorem singularAbilityReadHasCopyIdentity :
    let after := instructionResult (DRS.ofList [])
      (.copy .fromStack (target (.abilityHead .anyActivated)) (.lit 1) [])
    let selected := resolveCandidates (.word .ability) .one (nextClause after)
    selected.map (·.payload.orig) = [some .copy] ∧
      selected.map (·.id) = [after.nextId - 1] := by decide

theorem chosenPlayerDemonstrativeHasNewMentionIdentity :
    let before := DRS.ofList [ChoiceSort.binding .player]
    let after := instructionResult before (instruct .this (.dealDamage (.lit 3) (a chosenPlayer) .any))
    (resolveCandidates (.word .player) .one (nextClause after)).map (·.id) =
      [after.nextId - 2] := by decide

theorem possessiveItHasCurrentSubjectIdentity :
    let after := instructionResult (DRS.ofList [])
      (.sequentially [exile (target artifact),
        instruct (.firstMention "p05.subject" (target creature))
          (.dealDamage (.statOf (.stat .power) it) (target anyTarget) .any)])
    (after.referents.filter (fun ref => ref.idents.contains "p05.subject")).map
      (·.pronounResolved) = [true] := by decide

end Semantics.Proofs.Anaphora
