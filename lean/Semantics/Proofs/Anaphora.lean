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

/-- "Choose two — Draw a card; draw a card." [CR#700.2d] -/
theorem identicalModesAllowed :
    Instruction.check [] (chooseModes (exactly 2) [draw .you (.lit 1), draw .you (.lit 1)])
      = [] := by
  decide

/-- "Choose a player or planeswalker." -/
theorem okChoosePlayerOrPlaneswalker :
    Instruction.check []
      (.choose none none (a (.or [.hasType .planeswalker, .anyPlayer])) .openly none) = [] := by
  decide

/-- "Choose you." -/
theorem badChooseYou :
    Instruction.check [] (.choose none none .you .openly none) = [.choiceClause] := by decide

theorem badConditionalArmAntecedent :
    Instruction.check []
      (.sequentially
        [ .onlyIf (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"]))
            (exists_ creatureYouControl) none,
          .putCounters (.lit 1) (.printed plusOnePlusOne) it ]) = [.anaphor .bare .one 0] := by
  decide

theorem badBothArmsAntecedent :
    Instruction.check []
      (.sequentially
        [ .may .you (gainsLife .you (.lit 1))
            (some (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"])))
            (some (create (.lit 2) (creatureToken 1 1 [.white] [creatureType "Soldier"]))),
          .putCounters (.lit 1) (.printed plusOnePlusOne) it ]) = [.anaphor .bare .one 0] := by
  decide

/-- "Put target creature onto the battlefield." -/
theorem okMoveToBattlefield :
    Instruction.check [] (.move (target creature) battlefield []) = [] := by decide

/-- "Put target creature into your library." -/
theorem badMoveToBareLibrary :
    Instruction.check [] (.move (target creature) library []) = [.destOk] := by decide

/-- "Look at the top four cards of your library. You choose one of them." -/
theorem okAgentChoiceOfSome :
    Instruction.check []
      (.sequentially
        [ lookAt (topSlice (.lit 4)),
          .choose none (some .you) (someOf (exactly 1) them) .openly none ]) = [] := by
  decide

/-- "Look at the top four cards of your library. Choose one of them." -/
theorem badChooseSomeOf :
    Instruction.check []
      (.sequentially
        [ lookAt (topSlice (.lit 4)),
          .choose none none (someOf (exactly 1) them) .openly none ]) = [.choiceClause] := by
  decide

/-- "Exile target creature." -/
theorem okMoveToExile :
    Instruction.check [] (.move (target creature) exileZone []) = [] := by decide

/-- "Put target creature onto the stack." -/
theorem badMoveToStack :
    Instruction.check [] (.move (target creature) stack []) = [.destOk] := by decide

/-- "When this creature enters, if a creature died this turn, draw a card." -/
theorem okLookbackObjectDied :
    Ability.check []
      (.triggered (.enters thisCreature none) [] none [] none none
        (some (.happened (a creature) (.mk .death .thisTurn none))) (draw .you (.lit 1))) = [] := by
  decide

/-- "When this creature enters, if you died this turn, draw a card." -/
theorem badLookbackPlayerDied :
    Ability.check []
      (.triggered (.enters thisCreature none) [] none [] none none
        (some (.happened .you (.mk .death .thisTurn none))) (draw .you (.lit 1)))
      = [.lookbackSubject] := by
  decide

theorem badLookbackObjectCast :
    Ability.check []
      (.triggered (.enters thisCreature none) [] none [] none none
        (some (.happened (a creature) (.mk .spellCast .thisTurn none))) (draw .you (.lit 1)))
      = [.lookbackSubject] := by
  decide

/-- "target creature that entered this turn" -/
theorem okHappenedToObjectEntry :
    NounPhrase.check (some .object) []
      (target (.and [creature, .happenedTo (.mk .entry .thisTurn none)])) = [] := by
  decide

/-- "target creature who cast a spell this turn" -/
theorem badHappenedToObjectCast :
    NounPhrase.check (some .object) []
      (target (.and [creature, .happenedTo (.mk .spellCast .thisTurn none)]))
      = [.lookbackSubject] := by
  decide

/-- "each opponent who died this turn" -/
theorem badHappenedToPlayerDied :
    NounPhrase.check (some .player) []
      (each (.and [.opponent, .happenedTo (.mk .death .thisTurn none)])) = [.lookbackSubject] := by
  decide

/-- "each opponent who a state matched this turn" -/
theorem badStateMatchLookback :
    NounPhrase.check (some .player) []
      (each (.and [.opponent, .happenedTo (.mk .stateMatch .thisTurn none)]))
      = [.lookbackSubject] := by
  decide

/-- "Choose a creature you control." -/
theorem okChooseIndefinite :
    Instruction.check [] (.choose none none (a creatureYouControl) .openly none) = [] := by decide

/-- "Choose the creature with the least toughness among creatures you control." -/
theorem badChooseDefinite :
    Instruction.check []
      (.choose none none
        (the (.and [creature, .superlative .min (.stat .toughness) creatureYouControl]))
        .openly none) = [.choiceClause] := by
  decide

/-- "This deals 1 damage to that permanent or player." -/
theorem okUnionAnaphorAfterJoin :
    Instruction.check []
      (.sequentially
        [ .dealDamage .this (.lit 3) (target (.or [creature, .anyPlayer])),
          .dealDamage .this (.lit 1) (that .join) ]) = [] := by
  decide

/-- "This deals 3 damage to that permanent or player." -/
theorem badUnionAnaphorNoAntecedent :
    Instruction.check [] (.dealDamage .this (.lit 3) (that .join))
      = [.anaphor (.word .join) .one 0] := by
  decide

/-- "Destroy target creature. This deals 3 damage to that permanent or player." -/
theorem badUnionAnaphorOnObject :
    Instruction.check []
      (.sequentially [destroy (target creature), .dealDamage .this (.lit 3) (that .join)])
      = [.anaphor (.word .join) .one 0] := by
  decide

/-- "Counter target activated ability. Counter that spell or ability." An ability on the
stack is an object in the stack zone [CR#109.1,113.1c,405.1], so the one stack read reaches
it. -/
theorem okStackAnaphorOnAbility :
    Instruction.check []
      (.sequentially
        [ .counterSpell (target (.abilityHead .anyActivated)), .counterSpell (that .stack) ])
      = [] := by
  decide

def afterAnyTargetDamage : Bindings :=
  Instruction.intro [] (.dealDamage .this (.lit 3) (target anyTarget))

/-- "This deals 3 damage to any target. This deals 1 damage to that permanent or player." -/
theorem okUnionAnaphorAfterAnyTarget :
    NounPhrase.check (some (.join .object .player)) afterAnyTargetDamage (that .join) = [] := by
  decide

/-- "This deals 3 damage to any target. Counter that spell or ability." Refused: an any-target
union is not on the stack. `that .join` spells the permanent-or-player read
(`okUnionAnaphorAfterJoin`). -/
theorem badStackAnaphorOnPlayerUnion :
    NounPhrase.check (some .object) afterAnyTargetDamage (that .stack)
      = [.anaphor (.word .stack) .one 0] := by
  decide

/-- "a creature with protection from a color" -/
theorem okKeywordClassWithSort :
    Predicate.check .object [] (.hasKeyword (.anyIn ⟨"Protection", some .color⟩)) = [] := by decide

/-- "a creature with flyings" -/
theorem badClassOfParamlessKeyword :
    Predicate.check .object [] (.hasKeyword (.anyIn ⟨"Flying", none⟩)) = [.knownKeywordTerm] := by
  decide

/-- "a creature with renown of any color" -/
theorem badSortedClassOnNumberKeyword :
    Predicate.check .object [] (.hasKeyword (.anyIn ⟨"Renown", some .color⟩))
      = [.knownKeywordTerm] := by
  decide

/-- "{T}: Draw a card. Activate only if you created a creature this turn." -/
theorem okTokenCreationLookbackComplement :
    Condition.check []
      (.happened .you (.mk .tokenCreation .thisTurn (some (.involving (a creature))))) = [] := by
  decide

/-- "{T}: Draw a card. Activate only if you created this turn." -/
theorem badBareTokenCreationLookback :
    Condition.check [] (.happened .you (.mk .tokenCreation .thisTurn none))
      = [.complementWritten] := by
  decide

/-- "creature that was dealt damage by this creature this turn" -/
theorem okDamageTakenComplement :
    Predicate.check .object []
      (.happenedTo (.mk .damageTaken .thisTurn (some (.involving thisCreature)))) = [] := by
  decide

/-- "Destroy target creature that attacked with this creature this turn." -/
theorem badAttackerComplementOnObject :
    Predicate.check .object []
      (.happenedTo (.mk .attackDeclaration .thisTurn (some (.involving thisCreature))))
      = [.lookbackComplement] := by
  decide

/-- "if you've cast a creature spell this turn" -/
theorem okCastComplementOnObject :
    Condition.check [] (.happened .you (.mk .spellCast .thisTurn (some (.involving (a creature)))))
      = [] := by
  decide

theorem badPlayerCastComplement :
    Condition.check [] (.happened .you (.mk .spellCast .thisTurn (some (.involving anOpponent))))
      = [.lookbackComplement] := by
  decide

/-- "target spell cast from your graveyard" -/
theorem okCastFromGraveyard :
    Predicate.check .object [] (.castFrom (graveyardOf .you)) = [] := by decide

/-- "Counter target spell cast from the stack." -/
theorem badCastFromStack :
    Predicate.check .object [] (.castFrom stack) = [.playableFrom] := by decide

/-- "… that was put somewhere from the battlefield this turn." -/
theorem okPlacementOriginBattlefield :
    Predicate.check .object []
      (.happenedTo (.mk .placement .thisTurn (some (.fromZones (.zones [battlefield]) none))))
      = [] := by
  decide

/-- "… that died from the battlefield this turn." -/
theorem badDeathOriginZone :
    Predicate.check .object []
      (.happenedTo (.mk .death .thisTurn (some (.fromZones (.zones [battlefield]) none))))
      = [.lookbackSource] := by
  decide

/-- "if you've cast a spell from your hand this turn" -/
theorem okCastOriginFromHand :
    Condition.check []
      (.happened .you (.mk .spellCast .thisTurn (some (.fromZones (.zones [hand]) none))))
        = [] := by
  decide

/-- "if you've cast a spell from the stack this turn" -/
theorem badCastOriginFromStack :
    Condition.check []
      (.happened .you (.mk .spellCast .thisTurn (some (.fromZones (.zones [stack]) none))))
      = [.lookbackSource] := by
  decide

/-- "if you've cast a spell from this turn" -/
theorem badEmptyOriginCoordination :
    Condition.check []
      (.happened .you (.mk .spellCast .thisTurn (some (.fromZones (.zones []) none))))
      = [.lookbackSource] := by
  decide

/-- "if you've cast a creature spell from your hand this turn" -/
theorem okOriginPayloadInvolving :
    Condition.check []
      (.happened .you
        (.mk .spellCast .thisTurn
          (some (.fromZones (.zones [hand]) (some (.involving (a creature))))))) = [] := by
  decide

/-- "if you've cast a spell from your hand from the command zone this turn" -/
theorem badNestedOriginPayload :
    Condition.check []
      (.happened .you
        (.mk .spellCast .thisTurn
          (some (.fromZones (.zones [hand]) (some (.fromZones (.zones [command]) none))))))
      = [.complementPlain] := by
  decide

/-- "if you've cast a spell from anywhere other than this turn" -/
theorem badEmptyOriginExclusion :
    Condition.check []
      (.happened .you (.mk .spellCast .thisTurn (some (.fromZones (.anywhereBut []) none))))
      = [.lookbackSource] := by
  decide

/-- "… that died this turn." -/
theorem okDeathLookbackWithoutOrigin :
    Predicate.check .object [] (.happenedTo (.mk .death .thisTurn none)) = [] := by decide

/-- "… that died from anywhere this turn." -/
theorem badDeathOriginAnywhere :
    Predicate.check .object []
      (.happenedTo (.mk .death .thisTurn (some (.fromZones .anywhere none))))
        = [.lookbackSource] := by
  decide

/-- "… that was put into a graveyard from the battlefield this turn." -/
theorem okPlacementIntoGraveyard :
    Predicate.check .object []
      (.happenedTo
        (.mk .placement .thisTurn
          (some (.intoZone graveyard (some (.fromZones (.zones [battlefield]) none)))))) = [] := by
  decide

/-- "… that was put into the battlefield this turn." -/
theorem badPlacementIntoBattlefield :
    Predicate.check .object []
      (.happenedTo (.mk .placement .thisTurn (some (.intoZone battlefield none))))
      = [.lookbackDest] := by
  decide

/-- "… that was put into your graveyard into exile this turn." -/
theorem badNestedDestination :
    Predicate.check .object []
      (.happenedTo
        (.mk .placement .thisTurn
          (some (.intoZone (graveyardOf .you) (some (.intoZone exileZone none))))))
      = [.complementSourced] := by
  decide

/-- "if you shuffled your library this way" -/
theorem okShuffleLocusAtLibrary :
    Condition.check []
      (.happened .you (.mk (.verbedAct "Shuffle") .thisWay (some (.atZone yourLibrary)))) = [] := by
  decide

/-- "if a creature died in your graveyard this way" -/
theorem badLocusOnDeath :
    Condition.check []
      (.happened (a creature) (.mk .death .thisWay (some (.atZone (graveyardOf .you)))))
      = [.lookbackLocus] := by
  decide

/-- "if you searched this way, shuffle" -/
theorem badBareSearchLookback :
    Condition.check [] (.happened .you (.mk (.verbedAct "Search") .thisWay none))
      = [.complementWritten] := by
  decide

/-- "if you shuffled your graveyard this way" -/
theorem badShuffleLocusAtGraveyard :
    Condition.check []
      (.happened .you (.mk (.verbedAct "Shuffle") .thisWay (some (.atZone (graveyardOf .you)))))
      = [.lookbackLocus] := by
  decide

/-- "target creature spell, once it resolves" -/
theorem okResolvedCreatureSpell :
    NounPhrase.check (some .object) [] (.resolvedPermanent (a (.and [.hasType .creature, spell])))
      = [] := by
  decide

theorem badResolvedInstant :
    NounPhrase.check (some .object) [] (.resolvedPermanent (a (.and [.hasType .instant, spell])))
      = [.permanentSpellType] := by
  decide

/-- "this creature, once it resolves" -/
theorem badResolvedOnBattlefield :
    NounPhrase.check (some .object) [] (.resolvedPermanent thisCreature) = [.zoneIs .stack] := by
  decide

/-- "if it entered this turn" -/
theorem okEntryLookbackWithoutOrigin :
    Predicate.check .object [] (.happenedTo (.mk .entry .thisTurn none)) = [] := by decide

/-- "if it entered from the battlefield" -/
theorem badEntryOriginBattlefield :
    Predicate.check .object []
      (.happenedTo (.mk .entry .thisTurn (some (.fromZones (.zones [battlefield]) none))))
      = [.lookbackSource] := by
  decide

/-- "For each opponent, you draw a card." -/
theorem okPluralForEach :
    Instruction.check [] (.forEachOf (each .opponent) (draw .you (.lit 1))) = [] := by decide

/-- "For each of target creature, its controller draws a card." -/
theorem badSingletonForEach :
    Instruction.check [] (.forEachOf (target creature) (draw .you (.lit 1)))
      = [.plural] := by decide

/-- "For each player, choose target permanent that player controls. Those players draw a
card." (Vaevictis Asmadi, the Dire's opening loop): the group survives it, pluralised. -/
theorem okLoopGroupSurvives :
    Instruction.check []
      (.sequentially
        [ .forEachOf (each .anyPlayer)
            (choose (target (.and [permanent, .hasPossessor .controller they]))),
          draw (those .player) (.lit 1) ]) = [] := by
  decide

/-- "For each player, choose target permanent that player controls. That player draws a
card." The element the loop binds is body-local, so only the group reads back, and only as a
plural. -/
theorem badLoopElementRead :
    Instruction.check []
      (.sequentially
        [ .forEachOf (each .anyPlayer)
            (choose (target (.and [permanent, .hasPossessor .controller they]))),
          draw (that .player) (.lit 1) ]) = [.anaphor (.word .player) .one 0] := by
  decide

/-- "if you activated an activated ability this turn" -/
theorem okActivationLookbackComplement :
    Condition.check []
      (.happened .you
        (.mk .abilityActivation .thisTurn (some (.involving (a (.abilityHead .anyActivated))))))
      = [] := by
  decide

/-- "if you activated a loyalty ability this turn" -/
theorem badBareActivationLookback :
    Condition.check [] (.happened .you (.mk .abilityActivation .thisTurn none))
      = [.complementWritten] := by
  decide

/-- "on top of your library in any order, third from the top" -/
theorem okOneEndArrangedOrdinal :
    ZoneExpr.check [] (.library (.oneEnd .top) (some .anyOrder) (some (.nth 3)) .bare) = [] := by
  decide

/-- "Put those cards on the top or bottom of your library in any order." -/
theorem badDisjunctionOrdered :
    ZoneExpr.check [] (.library (.eitherEnd none) (some .anyOrder) none .bare)
      = [.placeArrangementFits] := by
  decide

/-- "Shuffle those cards into your library." -/
theorem okShuffledPlain :
    ZoneExpr.check [] (.library .shuffled none none .bare) = [] := by decide

/-- "Shuffle those cards into your library in any order." -/
theorem badShuffledArranged :
    ZoneExpr.check [] (.library .shuffled (some .anyOrder) none .bare)
      = [.placeArrangementFits] := by
  decide

/-- "Shuffle it into its owner's library third from the top." -/
theorem badShuffledOrdinal :
    ZoneExpr.check [] (.library .shuffled none (some (.nth 3)) .bare) = [.placeOrdinalFits] := by
  decide

/-- "third from the top" -/
theorem okThirdFromTop :
    ZoneExpr.check [] (.library (.oneEnd .top) none (some (.nth 3)) .bare) = [] := by decide

/-- "zeroth from the top" -/
theorem badZerothFromTop :
    ZoneExpr.check [] (.library (.oneEnd .top) none (some (.nth 0)) .bare) = [.ordinalNonZero] := by
  decide

theorem copyParticipleUnwritten : participleOf "Copy" = none := by decide

/-- "Copy target instant or sorcery spell twice. You may choose new targets for those spells."
A copy of a spell is itself a spell [CR#707.10,112.1a], so the plural spell read reaches the
copies. -/
theorem okPluralSpellReadAfterCopy :
    Instruction.check []
      (.sequentially
        [ .copy .fromStack .you (target (.and [instantOrSorcery, spell])) (.lit 2) [],
          may .you (.chooseNewTargets (those .spell)) ]) = [] := by
  decide

/-- "Copy target instant or sorcery spell. You may choose new targets for the copy." -/
theorem okCopyReadAfterCopy :
    Instruction.check []
      (.sequentially
        [ .copy .fromStack .you (target (.and [instantOrSorcery, spell])) (.lit 1) [],
          may .you (.chooseNewTargets (that .copy)) ]) = [] := by
  decide

/-- "Copy target instant or sorcery spell. You may choose new targets for that spell."
Refused: the copy is itself a spell [CR#707.10], so the singular spell read reaches the
original and the copy alike. `that .copy` spells the copy (`okCopyReadAfterCopy`). -/
theorem badSingularSpellReadAfterCopy :
    Instruction.check []
      (.sequentially
        [ .copy .fromStack .you (target (.and [instantOrSorcery, spell])) (.lit 1) [],
          may .you (.chooseNewTargets (that .spell)) ]) = [.anaphor (.word .spell) .one 2] := by
  decide

/-- "Copy target activated ability twice. You may choose new targets for those abilities." A
copy of an ability is itself an ability [CR#707.10], so the plural ability read reaches the
copies. -/
theorem okPluralAbilityReadAfterCopy :
    Instruction.check []
      (.sequentially
        [ .copy .fromStack .you (target (.abilityHead .anyActivated)) (.lit 2) [],
          may .you (.chooseNewTargets (those .ability)) ]) = [] := by
  decide

/-- "Copy target activated ability. You may choose new targets for that ability." Refused:
the copy is itself an ability [CR#707.10], so the singular ability read reaches the original
and the copy alike. `that .abilityCopy` spells the copy (`okAbilityCopyReadAfterCopy`). -/
theorem badSingularAbilityReadAfterCopy :
    Instruction.check []
      (.sequentially
        [ .copy .fromStack .you (target (.abilityHead .anyActivated)) (.lit 1) [],
          may .you (.chooseNewTargets (that .ability)) ]) = [.anaphor (.word .ability) .one 2] := by
  decide

/-- "Copy target activated ability. You may choose new targets for the copy." -/
theorem okAbilityCopyReadAfterCopy :
    Instruction.check []
      (.sequentially
        [ .copy .fromStack .you (target (.abilityHead .anyActivated)) (.lit 1) [],
          may .you (.chooseNewTargets (that .abilityCopy)) ]) = [] := by
  decide

/-- An ability goes on the stack with no card associated with it [CR#405.1]. -/
theorem abilityIsOnTheStack : Payload.zone (.ability none) = some .stack := by decide

theorem joinedCreatureTy :
    tyOfReach (.word .join) .one
      (Instruction.intro [] (.dealDamage .this (.lit 3) (target (.or [creature, .anyPlayer]))))
      = some .creature := by
  decide

theorem anyTargetIsPlaceless : NounPhrase.zone [] (target anyTarget) = none := by decide

theorem anyTargetTakesDamage : (target anyTarget).damageRecipient [] = true := by decide

theorem youAndBindsNothing : NounPhrase.delta [] (youAnd thisCreature) = [] := by decide

/-- "If a player is dealt damage this way, you draw a card." -/
theorem okDealtThisWayAfterDamage :
    Instruction.check []
      (.sequentially
        [ .dealDamage .this (.lit 3) (target anyTarget),
          .if_ (.dealtThisWay .anyPlayer) (draw .you (.lit 1)) none ]) = [] := by
  decide

/-- "You draw a card. If a player is dealt damage this way, you draw a card." -/
theorem badDealtThisWayNoDamage :
    Instruction.check []
      (.sequentially
        [ draw .you (.lit 1), .if_ (.dealtThisWay .anyPlayer) (draw .you (.lit 1)) none ])
      = [.damageDealtInScope] := by
  decide

/-- "This deals 2 damage to any target. If a mana ability is dealt damage this way, draw a
card." Refused: what the description seeds is on the stack, and nothing on the stack is dealt
damage. `.dealtThisWay .anyPlayer` spells the recipient read (`okDealtThisWayAfterDamage`). -/
theorem badDealtThisWayAbility :
    Instruction.check []
      (.sequentially
        [ .dealDamage .this (.lit 2) (target anyTarget),
          .if_ (.dealtThisWay .isManaAbility) (draw .you (.lit 1)) none ])
      = [.zoneIs .stack] := by
  decide

/-- "the number of creatures that died this turn" -/
theorem okDeathTally :
    Amount.check [] (.eventTally .count (a creature) (.mk .death .thisTurn none)) = [] := by decide

/-- "the amount of creatures that died this turn" -/
theorem badDeathSum :
    Amount.check [] (.eventTally .sum (a creature) (.mk .death .thisTurn none)) = [.tallyOk] := by
  decide

/-- "creature that died this turn" -/
theorem okObjectDeathLookbackSubject :
    Predicate.check .object [] (.happenedTo (.mk .death .thisTurn none)) = [] := by decide

/-- "creature that won a coin flip this turn" -/
theorem badCreatureWonFlip :
    Predicate.check .object [] (.happenedTo (.mk .flipWin .thisTurn none))
      = [.lookbackSubject] := by
  decide

/-- "the number of dice you rolled this turn" -/
theorem okRollTally :
    Amount.check [] (.eventTally .count .you (.mk .diceRoll .thisTurn none)) = [] := by decide

/-- "the amount of dice you rolled this turn" -/
theorem badRollAsMagnitude :
    Amount.check [] (.eventTally .sum .you (.mk .diceRoll .thisTurn none)) = [.tallyOk] := by decide

theorem youWonAFlipThisTurn :
    Condition.check [] (.happened .you (.mk .flipWin .thisTurn none)) = [] := by decide

theorem youRolledADieThisTurn :
    Condition.check [] (.happened .you (.mk .diceRoll .thisTurn none)) = [] := by decide

/-- "Roll two d20. Ignore the lowest roll." -/
theorem okIgnoreAfterRoll :
    Instruction.check [] (.sequentially [rollDice .you 2 20, .ignoreOutcomes (.extreme .lowest)])
      = [] := by
  decide

/-- "Ignore the lowest roll." -/
theorem badIgnoreWithoutRoll :
    Instruction.check [] (.ignoreOutcomes (.extreme .lowest)) = [.ignorableFor] := by decide

def afterATwoDieRoll : Bindings := Instruction.intro [] (rollDice .you 2 6)

/-- "if you rolled doubles" -/
theorem okRolledDoublesAfterRoll : Condition.check afterATwoDieRoll .rolledDoubles = [] := by decide

/-- "If you rolled doubles, sacrifice this creature." -/
theorem badRolledDoublesWithoutRoll :
    Condition.check [] .rolledDoubles = [.outcomeInScope .rollResult 0] := by decide

def afterACoinFlip : Bindings := Instruction.intro [] (flipCoins .you 1)

/-- "a player whose coin comes up tails" -/
theorem okCoinCameUpOnPlayer :
    Predicate.check .player afterACoinFlip (.coinCameUp .tails)
      = [] := by
  decide

/-- "the damage whose coin comes up tails". Refused: only objects and players flip coins.
`.coinCameUp` on a player spells the coin read (`okCoinCameUpOnPlayer`). -/
theorem badCoinCameUpOnOutcome :
    Predicate.check .outcome afterACoinFlip (.coinCameUp .tails)
      = [.kindLte .outcome (.join .object .player)] := by
  decide

/-- "Whenever you roll a 4 or higher, …" -/
theorem okBoundedRollTest :
    GameEvent.check [] (.rollsDice .you .one none (.resultIn (.range (some 4) none))) = [] := by
  decide

/-- "Whenever you roll a 0, …" -/
theorem badZeroRollTest :
    GameEvent.check [] (.rollsDice .you .one none (.resultIn (.range none (some 0))))
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
    GameEvent.check [] (.paysCost (some (a .anyPlayer)) .unpaid thisCreature "CumulativeUpkeep")
      = [] := by
  decide

/-- "When a player doesn't pay this creature's flying, …" -/
theorem badPayCostlessKeyword :
    GameEvent.check [] (.paysCost (some (a .anyPlayer)) .unpaid thisCreature "Flying")
      = [.keywordCost "Flying"] := by
  decide

/-- "if you paid life this turn" -/
theorem youPaidLifeThisTurn :
    Condition.check [] (.happened .you (.mk .lifePayment .thisTurn none)) = [] := by decide

/-- "if you paid a cost this turn" -/
theorem badBarePaymentLookback :
    Condition.check [] (.happened .you (.mk .costPayment .thisTurn none)) = [.lookbackSubject] := by
  decide

def afterAnUpToChoice : Bindings := Instruction.intro [] (choose (counted (upTo 1) creature))

/-- "Choose up to one creature. Destroy the rest." -/
theorem okRestAfterAPartition :
    NounPhrase.check (some .object) afterAnUpToChoice (theRest .object) = [] := by decide

/-- "Destroy the rest." -/
theorem badRestWithoutAPartition :
    NounPhrase.check (some .object) [] (theRest .object) = [.theRestFits .object] := by decide

/-- "Choose up to one creature. Destroy the rest." -/
theorem okRestAfterCountedChoice :
    Instruction.check []
      (.sequentially [choose (counted (upTo 1) creature), destroy (theRest .object)]) = [] := by
  decide

/-- "Choose any number of target creatures. Destroy the rest." -/
theorem badRestAfterTargetChoice :
    Instruction.check []
      (.sequentially
        [choose (.described (.target anyNumber) creature), destroy (theRest .object)])
      = [.theRestFits .object] := by
  decide

def afterALook : Bindings := Instruction.intro [] (lookAt (topSlice (.lit 1)))

/-- "Look at the top card of your library. Put that card into your graveyard." -/
theorem okReadsLookedAtCard : NounPhrase.check (some .object) afterALook (that .card) = [] := by
  decide

def afterShuffledLook : Bindings :=
  Instruction.intro [] (.sequentially [lookAt (topSlice (.lit 1)), shuffle])

theorem badReadsShuffledLibraryCard :
    NounPhrase.check (some .object) afterShuffledLook (that .card)
      = [.anaphor (.word .card) .one 0] := by
  decide

/-- "Whenever you scry, …" -/
theorem okPatientlessScry :
    GameEvent.check [] (.verbedEvent (some .you) "Scry" none none) = [] := by decide

/-- "Whenever you scry a card, …" -/
theorem badScryPatient :
    GameEvent.check [] (.verbedEvent (some .you) "Scry" (some (a (.inZone library))) none)
      = [.verbPatientOk] := by
  decide

/-- "Whenever discards a card, …" -/
theorem badVoicelessAct :
    GameEvent.check [] (.verbedEvent none "Scry" none none) = [.verbedVoiceOk] := by decide

/-- "Whenever a card is put, …" -/
theorem badPassiveWithoutParticiple :
    GameEvent.check [] (.verbedEvent none "Put" (some (a .isCard)) none) = [.verbedVoiceOk] := by
  decide

/-- "Whenever a creature transforms into a Phyrexian, …" -/
theorem okIntransitiveBecomes :
    GameEvent.check []
      (.verbedEvent none "Transform" (some (a creature))
        (some (.hasSubtype (creatureType "Phyrexian")))) = [] := by
  decide

/-- "Whenever a card is milled into a Phyrexian, …" -/
theorem badBecomesWithoutIntransitive :
    GameEvent.check []
      (.verbedEvent none "Mill" (some (a (.inZone library)))
        (some (.hasSubtype (creatureType "Phyrexian")))) = [.verbBecomesOk] := by
  decide

/-- "Whenever you discard a card, …" -/
theorem okDiscardFromHand :
    GameEvent.check [] (.verbedEvent (some .you) "Discard" (some (a (.inZone hand))) none)
      = [] := by
  decide

/-- "Whenever a card in a graveyard is destroyed, …" -/
theorem badDestroyInGraveyard :
    GameEvent.check [] (.verbedEvent none "Destroy" (some (a (.inZone graveyard))) none)
      = [.zoneFits] := by
  decide

/-- "Whenever you discard a permanent you control, …" -/
theorem badDiscardFromBattlefield :
    GameEvent.check [] (.verbedEvent (some .you) "Discard" (some (a (.inZone battlefield))) none)
      = [.zoneFits] := by
  decide

/-- "if you drew a card this turn" -/
theorem okPlayerDrawLookback :
    Condition.check [] (.happened .you (.mk .cardDrawn .thisTurn none)) = [] := by decide

/-- "if you dealt damage to an opponent this turn" -/
theorem badPlayerDamageDealer :
    Condition.check [] (.happened .you (.mk .damageDealing .thisTurn none))
      = [.lookbackSubject] := by
  decide

theorem joinedDealerDamageComplement :
    lookbackComplementOk .damageDealing .object (.join .object .player) = true := by decide

theorem lastChosenPlayerRead :
    Predicate.check .player [ChoiceSort.binding .player, ChoiceSort.binding .player]
      theLastChosenPlayer = [] := by
  decide

/-- "... a chosen player. ... the chosen player." -/
theorem okDefiniteChosenPlayerRead :
    Instruction.check [ChoiceSort.binding .player]
      (.sequentially
        [ .dealDamage .this (.lit 3) (a chosenPlayer),
          .dealDamage .this (.lit 3) (the chosenPlayer) ])
      = [] := by
  decide

/-- "... a chosen player. ... that player." Only a definite description refers to the choice
[CR#607.2d]. -/
theorem badIndefiniteChosenPlayerRead :
    Instruction.check [ChoiceSort.binding .player]
      (.sequentially
        [ .dealDamage .this (.lit 3) (a chosenPlayer), .dealDamage .this (.lit 3) they ])
      = [.anaphor (.word .player) .one 2] := by
  decide

/-- "that turn" after exactly one extra turn is minted -/
theorem okThatTurnAfterOneTurn :
    NounPhrase.check (some .turnRef) (Instruction.intro [] (.extraTurn .you (.lit 1))) thatTurn
      = [] := by
  decide

theorem badThatTurnWithoutTurn :
    NounPhrase.check (some .turnRef) [] thatTurn = [.anaphor .thatTurn .one 0] := by decide

theorem noTokenAsThoseWithoutAntecedent : countTokenSpecs [] = 0 := by decide

theorem oneTokenIsOneSpec :
    countTokenSpecs
      [⟨.a, .one, .object (some .creature) (some .battlefield) none (some .token) none⟩]
      = 1 := by
  decide

theorem manyTokensAreOneSpec :
    countTokenSpecs
      [⟨.a, .many, .object (some .creature) (some .battlefield) none (some .token) none⟩]
      = 1 := by
  decide

/-- A non-token object leaves no definition whatever its plurality. -/
theorem oneNonTokenIsNoSpec :
    countTokenSpecs
      [⟨.the, .one, .object (some .creature) (some .battlefield) none none none⟩] = 0 := by
  decide

/-- "where X is ..." -/
theorem badDefineWithoutUse : anyOpenLetter .x [] = false := by decide

theorem costXStaysOpen :
    anyOpenLetter .x (Cost.intro [] (.loyaltySymbol .downX)) = true := by decide

theorem thisNeedsNoAntecedent : NounPhrase.check (some .object) [] .this = [] := by decide

/-- "You" -/
theorem youNeedsNoAntecedent : NounPhrase.check (some .player) [] .you = [] := by decide

theorem letterValIntroducesAtEmptyPrefix : Amount.check [] (.letter .x) = [] := by decide

theorem ownSurvivesSecondSingular :
    Instruction.check []
      (.sequentially
        [ exile (target artifact),
          dealsDamageOwnPower (Instruction.intro [] (exile (target artifact))) (target creature)
            (target anyTarget) ]) = [] := by
  decide

/-- "Exile target creature. Draw cards equal to its power." -/
theorem okItReadsTheOnlyBareSingular :
    Instruction.check []
      (.sequentially [exile (target creature), draw .you (.statOf (.stat .power) it)]) = [] := by
  decide

theorem badItAcrossOwnSlot :
    Instruction.check []
      (.sequentially
        [ exile (target artifact),
          .dealDamage (target creature) (.statOf (.stat .power) it) (target anyTarget) ])
      = [.anaphor .bare .one 2] := by
  decide

/-- "Draw cards equal to its power." -/
theorem badSingularReadOfBarePlural :
    Instruction.check []
      (.sequentially
        [ gets (bare creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
          draw .you (.statOf (.stat .power) it) ]) = [.anaphor .bare .one 0] := by
  decide

/-- "Destroy target creature. If this enchantment isn't a creature, it becomes an Angel
creature until end of turn." The "it" reads the condition's own subject, which is all a
condition publishes. -/
theorem okCondSubjectRead :
    Instruction.check []
      (.sequentially
        [ destroy (target creature),
          .if_ (.not (.matches thisEnchantment creature))
            (becomes
              (itCondSubject (Instruction.intro [] (destroy (target creature)))
                (.not (.matches thisEnchantment creature)))
              { types := [.creature], subtypes := [creatureType "Angel"] }
              (some untilEndOfTurn))
            none ]) = [] := by
  decide

/-- The same sentence with an unwindowed "it": the earlier clause's creature is still
readable, so the pronoun is ambiguous. A condition's nouns do not join the read stack;
"if … it …" reads the clause subject. -/
theorem badCondUnwindowedRead :
    Instruction.check []
      (.sequentially
        [ destroy (target creature),
          .if_ (.not (.matches thisEnchantment creature))
            (becomes it { types := [.creature], subtypes := [creatureType "Angel"] }
              (some untilEndOfTurn))
            none ]) = [.anaphor .bare .one 2] := by
  decide

theorem badOwnEmptyDelta :
    Instruction.check [] (dealsDamageOwnPower [] .this (target anyTarget))
      = [.anaphor .bare .one 0] := by
  decide

/-- "Target creature gets +1/+1" -/
theorem okOwnReadsOneInDelta :
    Instruction.check []
      (sharedSubject (target creature)
        [ .modify (.pro .bare .one (.top (NounPhrase.delta [] (target creature)).length)) .power
            (.up (.lit 1)),
          .modify (.pro .bare .one (.top (NounPhrase.delta [] (target creature)).length))
            .toughness (.up (.lit 1)) ]
        none) = [] := by
  decide

theorem badSharedSubjectTwoInDelta :
    Instruction.check []
      (sharedSubject (.both (target creature) (target artifact))
        [ .modify
            (.pro .bare .one
              (.top (NounPhrase.delta [] (.both (target creature) (target artifact))).length))
            .power (.up (.lit 1)),
          .modify
            (.pro .bare .one
              (.top (NounPhrase.delta [] (.both (target creature) (target artifact))).length))
            .toughness (.up (.lit 1)) ]
        none) = [.anaphor .bare .one 2, .anaphor .bare .one 2] := by
  decide

theorem badOwnTwoInDelta :
    Instruction.check []
      (dealsDamageOwnPower [] (.both (target creature) (target creature)) (target anyTarget))
      = [.anaphor .bare .one 2] := by
  decide

theorem distributedDeedReadsBackPlural :
    Instruction.check []
      (.sequentially
        [ discard (each .opponent) (a (.inZone hand)),
          exile (theVerbed "Discard" .card .attributive .many) ]) = [] := by
  decide

/-- "Discard a card. Exile the discarded card." -/
theorem okTheVerbedAfterSingularDiscard :
    Instruction.check []
      (.sequentially
        [ discard .you (a (.inZone hand)), exile (theVerbed "Discard" .card .attributive .one) ])
      = [] := by
  decide

/-- "Each opponent discards a card. Exile that card." -/
theorem badDistributedDiscardSingular :
    Instruction.check []
      (.sequentially
        [ discard (each .opponent) (a (.inZone hand)),
          exile (theVerbed "Discard" .card .attributive .one) ])
      = [.anaphor (.verbed "Discard" .card .attributive) .one 0] := by
  decide

/-- "Tap target creature." -/
theorem okTapBattlefieldPermanent :
    Instruction.check [] (.setStatus .tapped (target creature)) = [] := by decide

/-- "Tap the top card of your library." -/
theorem badTapLibraryTop :
    Instruction.check [] (.setStatus .tapped (topSlice (.lit 1)))
      = [.zoneIs .battlefield] := by decide

/-- "if there is no monarch" -/
theorem okNoHolderOnPlayer : Condition.check [] (.noHolder "the monarch") = [] := by decide

/-- "if there is no monstrous creature" -/
theorem badNoHolderOnObject :
    Condition.check [] (.noHolder "monstrous") = [.designationScope "monstrous"] := by decide

/-- "Roll five d6. Store those results on this creature." -/
theorem okStoreResultsAfterRoll :
    Instruction.check [] (.sequentially [rollDice .you 5 6, .storeResults thisCreature]) = [] := by
  decide

/-- "Store those results on this creature." -/
theorem badStoreResultsWithoutRoll :
    Instruction.check [] (.storeResults thisCreature) = [.outcomeInScope .rollResult 0] := by decide

/-- "If you would roll one or more d6, instead roll that many of those dice." -/
theorem okThoseDiceAfterRollEvent :
    Instruction.check []
      (ifWouldInstead (.rollsDice .you .many (some 6) .anyResult)
        (.rollDice .you .thatMuch .thoseDice) none) = [] := by
  decide

/-- "Roll that many dice." -/
theorem badAnaphoricSidesWithoutRoll :
    Instruction.check [] (.rollDice .you (.lit 1) .thoseDice)
      = [.outcomeInScope .diceRolled 0] := by
  decide

end Semantics.Proofs.Anaphora
