import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Description

Port of `idris/src/Experimental/ProofsDescription.idr`: the pins of the Description family.
A twin states that the checker admits a spelling (`= []`); a pin states the one obligation
it refuses (`= [r]`), which is the Idris pin's claim (the term elaborates but for one
obligation) made explicit. Every statement is closed by `decide`, so the kernel runs the
checker.

The pins keep the Idris names and docstrings (the sentence each refuses).
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Description

private def actionResult (drs : DRS) (action : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs action).1

/-- "a creature target opponent controls" -/
theorem okControlledByOne :
    Predicate.checkAt .object (DRS.ofList []) (.hasPossessor .controller (target (.isOpponentOf .you))) = [] := by decide

/-- "a creature two target opponents control" -/
theorem badControlledByGroup :
    Predicate.checkAt .object (DRS.ofList [])
      (.hasPossessor .controller (.described (.target (exactly 2)) (.isOpponentOf .you))) = [.soleHolder] := by
  decide

/-- "each of up to two target creatures" -/
theorem okEachOfGroup :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.eachOf (.described (.target (upTo 2)) isCreature)) = [] := by decide

/-- "each of target creature" -/
theorem badEachOfSingular :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.eachOf (target isCreature)) = [.plural] := by decide

/-- "Look at the top card of target player's library." -/
theorem okSliceOfOnePossessor :
    Action.check (DRS.ofList []) (lookAt (.slice .top (.lit 1) (libraryOf (target .anyPlayer)))) = [] := by decide

/-- "Look at the top card of two target players' library." -/
theorem badSliceOfCountedPossessor :
    Action.check (DRS.ofList [])
      (lookAt (.slice .top (.lit 1) (libraryOf (.described (.target (exactly 2)) .anyPlayer))))
      = [.slicePossessor] := by
  decide

/-- "Tap target creature an opponent controls. That player loses 1 life." -/
theorem okThatPlayer :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .setStatus .tapped (target (.and [isCreature, .hasPossessor .controller anOpponent])),
          instruct (that .player) (loseLife (.lit 1)) ]) = [] := by
  decide

/-- "Tap target creature an opponent controls or land you control. That player loses 1 life." -/
theorem badDisjunctAntecedent :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .setStatus .tapped (target (.or [.and [isCreature, .hasPossessor .controller anOpponent],
                                           .and [isLand, .hasPossessor .controller .you]])),
          instruct (that .player) (loseLife (.lit 1)) ])
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Tap target creature with flying." -/
theorem okKeywordConjunction :
    Action.check (DRS.ofList [])
      (.setStatus .tapped (target (.and [isCreature, .hasKeyword (.the (.named "flying"))]))) = [] := by
  decide

/-- "Tap target creature with flying that doesn't have flying." -/
theorem okKeywordSelfNegation :
    Action.check (DRS.ofList [])
      (.setStatus .tapped (target (.and [isCreature, .hasKeyword (.the (.named "flying")),
                                         .not (.hasKeyword (.the (.named "flying")))]))) = [] := by
  decide

/-- "creature with power 2 or less" -/
theorem okContradictionFreeAnd :
    Predicate.checkAt .object (DRS.ofList []) (.and [isCreature, .compare [.stat .power] .atMost (.lit 2)]) = [] := by
  decide

/-- "blocked creature that's unblocked" -/
theorem okBlockedAndUnblocked :
    Predicate.checkAt .object (DRS.ofList []) (.and [isCreature, isBlocked, isUnblocked]) = [] := by decide

/-- "between two and three target creatures" -/
theorem okAscendingRange :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.described (.target (.range (some 2) (some 3))) isCreature) = [] := by
  decide

/-- "between three and two target creatures" -/
theorem badDescendingRange :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.described (.target (.range (some 3) (some 2))) isCreature)
      = [.wellFormedQ] := by
  decide

/-- "attacking artifact or attacking land" -/
theorem okWholeZoneJoin :
    Predicate.checkAt .object (DRS.ofList []) (.or [.and [isArtifact, isAttacking], .and [isLand, isAttacking]]) = [] := by
  decide

/-- "attacking artifact or land" -/
theorem okPartialZoneJoin :
    Predicate.checkAt .object (DRS.ofList []) (.or [.and [isArtifact, isAttacking], isLand]) = [] := by
  decide

/-- "creature you control or artifact you control" -/
theorem okDistinctStructuredDisjuncts :
    Predicate.checkAt .object (DRS.ofList [])
      (.or [.and [isCreature, .hasPossessor .controller .you],
            .and [isArtifact, .hasPossessor .controller .you]]) = [] := by
  decide

/-- "if this creature is attacking" -/
theorem okMatchesThisCreature : Condition.check (DRS.ofList []) (.matches thisCreature isAttacking) = [] := by decide

/-- "if target creature is an artifact" -/
theorem badMatchesTargetSubject :
    Condition.check (DRS.ofList []) (.matches (target isCreature) isArtifact) = [.testSubject] := by decide

/-- "Tap target creature. You gain 1 life if it's an artifact." -/
theorem okMatchesArtifact :
    Action.check (DRS.ofList [])
      (.sequentially [.setStatus .tapped (target isCreature),
                      doOnlyIf (gainLife (.lit 1)) (.matches it isArtifact) none]) =
                          [] := by
  decide

/-- "Tap target creature. You gain 1 life if it's." -/
theorem badMatchesNothing :
    Action.check (DRS.ofList [])
      (.sequentially [.setStatus .tapped (target isCreature),
                      doOnlyIf (gainLife (.lit 1)) (.matches it (.and [])) none])
      = [.predSays] := by
  decide

/-- "if the number of artifacts you control is 4 or greater" -/
theorem okCompareCountSubject :
    Condition.check (DRS.ofList [])
      (.compareAmt (countOf (.and [isArtifact, .hasPossessor .controller .you])) .atLeast (.lit 4))
      = [] := by
  decide

/-- "if 3 is 4 or greater" -/
theorem badCompareLiteralSubject :
    Condition.check (DRS.ofList []) (.compareAmt (.lit 3) .atLeast (.lit 4)) = [.readAmount] := by decide

/-- "each of each creature" -/
theorem badEachOfDistributive :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.eachOf (each isCreature)) = [.groupMention] := by decide

/-- "each of all creatures" -/
theorem badEachOfAll :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.eachOf (allOf isCreature)) = [.groupMention] := by decide

/-- "each of each of up to two target creatures" -/
theorem badNestedEachOf :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.eachOf (.eachOf (.described (.target (upTo 2)) isCreature)))
      = [.groupMention] := by
  decide

/-- "target white creature" -/
theorem okWhiteCreature :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (target (.and [isCreature, .colorIs .white])) = [] := by decide

/-- "Draw X cards, where X is the number of creatures you control." -/
theorem okSingleXRider :
    Action.check (DRS.ofList [])
      (.sequentially [.draw (.exactlyOf (.letter .x)), define .x (countOf (and [isCreature, youControl]))]) = []
          := by
  decide

theorem badDoubleXRider :
    Action.check (DRS.ofList [])
      (.sequentially [.draw (.exactlyOf (.letter .x)), define .x (countOf (and [isCreature, youControl])),
                      define .x (countOf isCreature)]) = [.openLetter .x] := by
  decide

/-- "Draw Y cards, where X is the number of creatures you control." -/
theorem badUnlicensedY :
    Action.check (DRS.ofList [])
      (.sequentially [.draw (.exactlyOf (.letter .y)), define .x (countOf (and [isCreature, youControl]))])
      = [.openLetter .x] := by
  decide

/-- "the power of target land creature": an animated land is a land AND a creature. -/
theorem okAnimatedLandPower :
    Amount.check (DRS.ofList []) (.statOf (.stat .power) (target (.and [isLand, isCreature]))) = [] := by decide

/-- "the greatest power among creatures" -/
theorem okPowerAmongObjects :
    Amount.check (DRS.ofList []) (aggregate .max (.stat .power) isCreature) = [] := by decide

/-- "the greatest power among players" -/
theorem badPowerAmongPlayers :
    Amount.check (DRS.ofList []) (aggregate .max (.stat .power) .anyPlayer) = [.projScope .player] := by decide

/-- "the highest life total among creatures you control" -/
theorem badLifeTotalAmongObjects :
    Amount.check (DRS.ofList []) (aggregate .max (.playerStat .lifeTotal) (and [isCreature, youControl]))
      = [.projScope .object] := by
  decide

/-- "the creature with the greatest power" -/
theorem okExtremalSelection :
    Predicate.checkAt .object (DRS.ofList []) (.superlative .max (.stat .power) isCreature) = [] := by decide

/-- "the creature with the total power among creatures you control" -/
theorem badSumSelection :
    Predicate.checkAt .object (DRS.ofList []) (.superlative .sum (.stat .power) isCreature) = [.isExtremal] := by decide

/-- "the creature with the least toughness among creatures you control" -/
theorem okDefiniteSuperlative :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (the (.and [isCreature, .superlative .min (.stat .toughness) (and [isCreature, youControl])])) = [] := by
  decide

/-- "the creature" -/
theorem badBareDefinite :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (the isCreature) = [.uniquifying] := by decide

/-- "the player with the highest life total" -/
theorem okPlayerStatSuperlative :
    Predicate.checkAt .player (DRS.ofList []) (.superlative .max (.playerStat .lifeTotal) .anyPlayer) = [] := by
  decide

/-- "the creature with the highest life total among creatures you control" -/
theorem badLifeTotalSuperlative :
    Predicate.checkAt .object (DRS.ofList []) (.superlative .max (.playerStat .lifeTotal) isCreature)
      = [.projScope .object] := by
  decide

/-- "this creature" -/
theorem okAscribeCreature : NounPhrase.checkAt (some .object) (DRS.ofList []) (.headed (.type .creature) .this) = [] := by decide

/-- "target creature that's goaded" -/
theorem okObjectDesignation :
    Predicate.checkAt .object (DRS.ofList []) (.hasDesignation (.named "goaded") none) = [] := by decide

/-- "target creature that is the monarch" -/
theorem badObjectMonarch :
    Predicate.checkAt .object (DRS.ofList []) (.hasDesignation (.named "monarch") none)
      = [.designationHolder (.named "monarch") .object] := by
  decide

/-- "one of the top two cards of your library" -/
theorem okPartitiveOfSlice :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.someOf (.counted (exactly 1)) none (.slice .top (.lit 2) (libraryOf .you))) = [] := by
  decide

/-- "one of one or more creatures" -/
theorem badPartitiveOfCountedGroup :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.someOf (.counted (exactly 1)) none (counted (atLeast 1) isCreature)) = [.partitiveBase] := by
  decide

/-- "Destroy target creature. Its controller loses life equal to its power." -/
theorem okItAfterAntecedent :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isCreature), instruct (controllerOf it) (loseLife (.statOf (.stat .power) it))])
      = [] := by
  decide

theorem badOtherwiseReadsLeadingArm :
    Action.check (DRS.ofList [])
      (.doIf (exists_ (and [isCreature, youControl]))
        (create (.lit 1) (.written (creatureToken 1 1 [.black] [zombie])))
        (some (.setStatus .tapped it)))
      = [.anaphor .bare .one 0] := by
  decide

theorem noElseConditionExportsItsOpponent :
    let conditional := .doIf (.compareAmt (lifeTotalOf .you) .less
      (lifeTotalOf anOpponent)) (gainLife (.lit 6)) none
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) conditional).1
    Action.check (DRS.ofList [])
      (.sequentially [conditional, instruct (that .player) (loseLife (.lit 1))]) = [] ∧
      (resolveCandidates (.word .player) .one (nextClause after)).map (·.id) = [1] ∧
      (after.findById? 1).map (·.kind) = some .player := by decide

/-- "If you control an artifact and an enchantment, …" -/
theorem okFlatConjunction :
    Condition.check (DRS.ofList [])
      (.and [exists_ (.and [isArtifact, .hasPossessor .controller .you]),
             exists_ (.and [isEnchantment, .hasPossessor .controller .you])]) = [] := by
  decide

/-- "If you control an artifact, create a token." -/
theorem badSingletonConjunction :
    Condition.check (DRS.ofList []) (.and [exists_ (.and [isArtifact, .hasPossessor .controller .you])])
      = [.atLeastTwo] := by
  decide

/-- "If you control an artifact and an enchantment, and you control a land, …" -/
theorem badNestedConjunction :
    Condition.check (DRS.ofList [])
      (.and [ .and [exists_ (.and [isArtifact, .hasPossessor .controller .you]),
                    exists_ (.and [isEnchantment, .hasPossessor .controller .you])],
              exists_ (.and [isLand, .hasPossessor .controller .you]) ]) = [.flatConjuncts] := by
  decide

/-- "Roll two d6. If you rolled 7, sacrifice this creature." -/
theorem okTotalAfterRoll :
    Action.check (DRS.ofList [])
      (.sequentially [ rollDice (.lit 2) (.sides 6),
                       doIf (.compareAmt (.theOutcome .rollResult) .eq (.lit 7))
                              (sacrifice thisCreature) ]) = [] := by
  decide

/-- "If you rolled 7, sacrifice this creature." -/
theorem badTotalWithoutRoll :
    Action.check (DRS.ofList [])
      (doIf (.compareAmt (.theOutcome .rollResult) .eq (.lit 7)) (sacrifice thisCreature))
      = [.outcomeInScope .rollResult 0] := by
  decide

/-- "Look at the top card of your library. Put that card into your graveyard." -/
theorem okReadsLookedAtLibraryCard :
    NounPhrase.checkAt (some .object) (actionResult (DRS.ofList []) (lookAt (topSlice (.lit 1)))) (that .card)
      = [] := by
  decide

theorem badReadsShuffledIntoLibraryCard :
    NounPhrase.checkAt (some .object)
      (actionResult (DRS.ofList []) (.sequentially [lookAt (topSlice (.lit 1)), shuffleInto .this]))
      (that .card) = [.anaphor (.word .card) .one 0] := by
  decide

/-- "target monocolored permanent": exactly one color -/
theorem monocoloredIsOneColor : Predicate.checkAt .object (DRS.ofList []) (.colorCount .eq 1) = [] := by decide

/-- "target colorless permanent": zero colors is how "colorless" is spelled [CR#105.2c]. -/
theorem okExactlyZeroColors :
    Predicate.checkAt .object (DRS.ofList []) (.colorCount .eq 0) = [] := by decide

/-- "if this creature's kicker cost was paid" -/
theorem okPaidCostOnKeywordWithACost :
    Amount.check (DRS.ofList []) (.paid (.readback (.byKeyword (.named "kicker")) none) .this) = [] := by decide

/-- "if this creature's flying cost was paid" -/
theorem badPaidCostOnCostlessKeyword :
    Amount.check (DRS.ofList []) (.paid (.readback (.byKeyword (.named "flying")) none) .this) = [.linkedFacetNamed] := by
  decide

/-- "for each time it was kickre'd" -/
theorem badTimesPaidUnknownKeyword :
    Amount.check (DRS.ofList []) (.paid (.timesPaid (.byKeyword (.named "kickre"))) thisCreature) = [.linkedFacetNamed] := by
  decide

/-- "for each color of mana spent to cast this spell" -/
theorem okColorsSpentOnThis : Amount.check (DRS.ofList []) (colorsSpentToCast .this) = [] := by decide

/-- "if colored mana was spent to cast it" is a read of `paid colorsSpent` -/
theorem coloredManaSpentIsPaidColorsSpent :
    coloredManaSpentToCast .this = .compareAmt (.paid .colorsSpent .this) .atLeast (.lit 1) := rfl

/-- "spell that targets this creature" -/
theorem okSpellTargeter :
    Predicate.checkAt .object (DRS.ofList []) (.targets thisCreature .someTarget) = [] := by decide

/-- "player that targets this creature" -/
theorem badPlayerTargeter :
    Predicate.checkAt .player (DRS.ofList []) (.targets thisCreature .someTarget) = [.targeter .player] := by decide

/-- "a creature with power or toughness 2 or greater" -/
theorem okSingleScopeAxisComparison :
    Predicate.checkAt .object (DRS.ofList []) (.compare [.stat .power, .stat .toughness] .atLeast (.lit 2)) = [] := by
  decide

/-- "a creature with power or life total 3 or greater" -/
theorem badMixedAxisComparison :
    Predicate.checkAt .object (DRS.ofList []) (.compare [.stat .power, .playerStat .lifeTotal] .greater (.lit 1))
      = [.axesAt .object] := by
  decide

/-- "a player with 13 or less life" -/
theorem okPlayerReadInPlayerDomain :
    Predicate.checkAt .player (DRS.ofList [])
      (.and [.anyPlayer, .compare [.playerStat .lifeTotal] .atMost (.lit 13)]) = [] := by
  decide

/-- "an object with 13 or less life" -/
theorem badPlayerReadInObjectDomain :
    Predicate.checkAt .object (DRS.ofList []) (.compare [.playerStat .lifeTotal] .atMost (.lit 13))
      = [.axesAt .object] := by
  decide

/-- "target card on the stack" -/
theorem okCardAndSpell : Predicate.checkAt .object (DRS.ofList []) cardOnTheStack = [] := by decide

/-- "each token on the battlefield" -/
theorem okTokenAndPermanent : Predicate.checkAt .object (DRS.ofList []) tokenOnTheBattlefield = [] := by decide

/-- "Target creature can't attack this turn." -/
theorem okCantAttackCreature :
    Action.check (DRS.ofList []) (establish (cant (target isCreature) attack) (some thisTurn)) = [] := by decide

/-- "Target land can't attack this turn." [CR#508.1a,205.1b] -/
theorem okCantAttackLand :
    Action.check (DRS.ofList []) (establish (cant (target isLand) attack) (some thisTurn)) = [] := by decide

/-- "Target land can't attack." -/
theorem okCantAttackLandNoSpan : Action.check (DRS.ofList []) (establish (cant (target isLand) attack) none) = [] := by
    decide

/-- "Target creature can't block this turn." -/
theorem okCantBlockCreature :
    Action.check (DRS.ofList []) (establish (cant (target isCreature) block) (some thisTurn)) = [] := by decide

/-- "Target creature or land can't block this turn." [CR#205.1b,509.1a] -/
theorem okCantDisjunctSubject :
    Action.check (DRS.ofList []) (establish (cant (target (.or [isCreature, isLand])) block) (some thisTurn)) = [] := by
        decide

/-- "your party": one each of Cleric, Rogue, Warrior and Wizard [CR#700.8]. -/
theorem okPartyOfFourRoles : NounPhrase.checkAt (some .object) (DRS.ofList []) party = [] := by decide

/-- "one each of nothing" [CR#700.8]. -/
theorem badEmptyPartyRoles :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.oneEachOf [] (allOf (and [isCreature, youControl]))) = [.nonEmpty] := by decide

/-- "a fortified land" -/
theorem okFortifiedLand :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (a (.and [isLand, isAttached (some .fortified)])) = [] := by decide

/-- "the top card of each player's library" is one card per player, collected: plural, so it reads
back as "those cards". "the top card of your library" stays one card. -/
theorem eachPlayerTopCardSliceIsPlural :
    (slice .top (.lit 1) (libraryOf eachPlayer)).plur = .many ∧
      (slice .top (.lit 1) (graveyardOf eachPlayer)).plur = .many ∧
      (slice .top (.lit 1) (libraryOf you)).plur = .one := by
  decide

end Semantics.Proofs.Description
