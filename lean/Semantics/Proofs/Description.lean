import Semantics.Macros
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

open Semantics Semantics.Macros

namespace Semantics.Proofs.Description

/-- "a creature target opponent controls" -/
theorem okControlledByOne :
    Predicate.check .object [] (.hasPossessor .controller (target .opponent)) = [] := by decide

/-- "a creature two target opponents control" -/
theorem badControlledByGroup :
    Predicate.check .object []
      (.hasPossessor .controller (.described (.target (exactly 2)) .opponent)) = [.soleHolder] := by
  decide

/-- "each of up to two target creatures" -/
theorem okEachOfGroup :
    NounPhrase.check (some .object) [] (.eachOf (.described (.target (upTo 2)) creature)) = [] := by decide

/-- "each of target creature" -/
theorem badEachOfSingular :
    NounPhrase.check (some .object) [] (.eachOf (target creature)) = [.plural] := by decide

/-- "Look at the top card of target player's library." -/
theorem okSliceOfOnePossessor :
    Instruction.check [] (lookAt (.librarySlice .top (.lit 1) (target .anyPlayer))) = [] := by decide

/-- "Look at the top card of two target players' library." -/
theorem badSliceOfCountedPossessor :
    Instruction.check []
      (lookAt (.librarySlice .top (.lit 1) (.described (.target (exactly 2)) .anyPlayer)))
      = [.slicePossessor] := by
  decide

/-- "Tap target creature an opponent controls. That player loses 1 life." -/
theorem okThatPlayer :
    Instruction.check []
      (.sequentially
        [ .setStatus .tapped (target (.and [creature, .hasPossessor .controller anOpponent])),
          losesLife (that .player) (.lit 1) ]) = [] := by
  decide

/-- "Tap target creature an opponent controls or land you control. That player loses 1 life." -/
theorem badDisjunctAntecedent :
    Instruction.check []
      (.sequentially
        [ .setStatus .tapped (target (.or [.and [creature, .hasPossessor .controller anOpponent],
                                           .and [land, .hasPossessor .controller .you]])),
          losesLife (that .player) (.lit 1) ])
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Tap target creature with flying." -/
theorem okKeywordConjunction :
    Instruction.check []
      (.setStatus .tapped (target (.and [creature, .hasKeyword (.the "Flying")]))) = [] := by
  decide

/-- "Tap target creature with flying that doesn't have flying." -/
theorem okKeywordSelfNegation :
    Instruction.check []
      (.setStatus .tapped (target (.and [creature, .hasKeyword (.the "Flying"),
                                         .not (.hasKeyword (.the "Flying"))]))) = [] := by
  decide

/-- "Tap target nonland Forest." -/
theorem badForestNonland :
    Instruction.check []
      (.setStatus .tapped (target (.and [.hasSubtype (landType "Forest"), .not land])))
      = [.contradictionFree] := by
  decide

/-- "creature with power 2 or less" -/
theorem okContradictionFreeAnd :
    Predicate.check .object [] (.and [creature, .compare [.stat .power] .atMost (.lit 2)]) = [] := by
  decide

/-- "noncreature that is attacking or blocking" -/
theorem badNoncreatureAttackingOrBlocking :
    Predicate.check .object [] (.and [.not creature, .or [attacking, blocking]])
      = [.contradictionFree] := by
  decide

/-- "noncreature that is an attacking artifact or a blocking land" -/
theorem badWrappedStatusLaunder :
    Predicate.check .object []
      (.and [.or [.and [artifact, attacking], .and [land, blocking]], .not creature])
      = [.contradictionFree] := by
  decide

/-- "blocked creature that's unblocked" -/
theorem okBlockedAndUnblocked :
    Predicate.check .object [] (.and [creature, blocked, unblocked]) = [] := by decide

/-- "between two and three target creatures" -/
theorem okAscendingRange :
    NounPhrase.check (some .object) [] (.described (.target (.range (some 2) (some 3))) creature) = [] := by
  decide

/-- "between three and two target creatures" -/
theorem badDescendingRange :
    NounPhrase.check (some .object) [] (.described (.target (.range (some 3) (some 2))) creature)
      = [.wellFormedQ] := by
  decide

/-- "attacking artifact or attacking land" -/
theorem okWholeZoneJoin :
    Predicate.check .object [] (.or [.and [artifact, attacking], .and [land, attacking]]) = [] := by
  decide

/-- "attacking artifact or land" -/
theorem badPartialZoneJoin :
    Predicate.check .object [] (.or [.and [artifact, attacking], land]) = [.parallelDisjuncts] := by
  decide

/-- "creature you control or artifact you control" -/
theorem okDistinctStructuredDisjuncts :
    Predicate.check .object []
      (.or [.and [creature, .hasPossessor .controller .you],
            .and [artifact, .hasPossessor .controller .you]]) = [] := by
  decide

/-- "if this creature is attacking" -/
theorem okMatchesThisCreature : Condition.check [] (.matches thisCreature attacking) = [] := by decide

/-- "if target creature is an artifact" -/
theorem badMatchesTargetSubject :
    Condition.check [] (.matches (target creature) artifact) = [.testSubject] := by decide

/-- "Tap target creature. You gain 1 life if it's an artifact." -/
theorem okMatchesArtifact :
    Instruction.check []
      (.sequentially [.setStatus .tapped (target creature),
                      .onlyIf (gainsLife .you (.lit 1)) (.matches it artifact) none]) = [] := by
  decide

/-- "Tap target creature. You gain 1 life if it's." -/
theorem badMatchesNothing :
    Instruction.check []
      (.sequentially [.setStatus .tapped (target creature),
                      .onlyIf (gainsLife .you (.lit 1)) (.matches it (.and [])) none])
      = [.predSays] := by
  decide

/-- "if the number of artifacts you control is 4 or greater" -/
theorem okCompareCountSubject :
    Condition.check []
      (.compareAmt (countOf (.and [artifact, .hasPossessor .controller .you])) .atLeast (.lit 4))
      = [] := by
  decide

/-- "if 3 is 4 or greater" -/
theorem badCompareLiteralSubject :
    Condition.check [] (.compareAmt (.lit 3) .atLeast (.lit 4)) = [.readAmount] := by decide

/-- "each of each creature" -/
theorem badEachOfDistributive :
    NounPhrase.check (some .object) [] (.eachOf (each creature)) = [.groupMention] := by decide

/-- "each of all creatures" -/
theorem badEachOfAll :
    NounPhrase.check (some .object) [] (.eachOf (allOf creature)) = [.groupMention] := by decide

/-- "each of each of up to two target creatures" -/
theorem badNestedEachOf :
    NounPhrase.check (some .object) [] (.eachOf (.eachOf (.described (.target (upTo 2)) creature)))
      = [.groupMention] := by
  decide

/-- "target white creature" -/
theorem okWhiteCreature :
    NounPhrase.check (some .object) [] (target (.and [creature, .colorIs .white])) = [] := by decide

/-- "target colorless white creature" -/
theorem badColorlessWhite :
    NounPhrase.check (some .object) [] (target (.and [creature, colorless, .colorIs .white]))
      = [.contradictionFree] := by
  decide

/-- "Draw X cards, where X is the number of creatures you control." -/
theorem okSingleXRider :
    Instruction.check []
      (.sequentially [.draw .you (.letter .x), .define .x (countOf creatureYouControl)]) = [] := by
  decide

theorem badDoubleXRider :
    Instruction.check []
      (.sequentially [.draw .you (.letter .x), .define .x (countOf creatureYouControl),
                      .define .x (countOf creature)]) = [.openLetter .x] := by
  decide

/-- "Draw Y cards, where X is the number of creatures you control." -/
theorem badUnlicensedY :
    Instruction.check []
      (.sequentially [.draw .you (.letter .y), .define .x (countOf creatureYouControl)])
      = [.openLetter .x] := by
  decide

/-- "the power of target land creature": an animated land is a land AND a creature. -/
theorem okAnimatedLandPower :
    Amount.check [] (.statOf (.stat .power) (target (.and [land, creature]))) = [] := by decide

/-- "the power of target land": a noncreature permanent has no power [CR#208.3]. -/
theorem badLandPower :
    Amount.check [] (.statOf (.stat .power) (target land)) = [.statHeadTysOk] := by decide

/-- "the loyalty of target battle": loyalty is printed on planeswalkers [CR#209.1]. -/
theorem badBattleLoyalty :
    Amount.check [] (.statOf (.stat .loyalty) (target (.hasType .battle))) = [.statHeadTysOk] := by decide

/-- "the greatest power among creatures" -/
theorem okPowerAmongObjects :
    Amount.check [] (aggregate .max (.stat .power) creature) = [] := by decide

/-- "the greatest power among players" -/
theorem badPowerAmongPlayers :
    Amount.check [] (aggregate .max (.stat .power) .anyPlayer) = [.projScope .player] := by decide

/-- "the highest life total among creatures you control" -/
theorem badLifeTotalAmongObjects :
    Amount.check [] (aggregate .max (.playerStat .lifeTotal) creatureYouControl)
      = [.projScope .object] := by
  decide

/-- "the creature with the greatest power" -/
theorem okExtremalSelection :
    Predicate.check .object [] (.superlative .max (.stat .power) creature) = [] := by decide

/-- "the creature with the total power among creatures you control" -/
theorem badSumSelection :
    Predicate.check .object [] (.superlative .sum (.stat .power) creature) = [.isExtremal] := by decide

/-- "the creature with the least toughness among creatures you control" -/
theorem okDefiniteSuperlative :
    NounPhrase.check (some .object) []
      (the (.and [creature, .superlative .min (.stat .toughness) creatureYouControl])) = [] := by
  decide

/-- "the creature" -/
theorem badBareDefinite :
    NounPhrase.check (some .object) [] (the creature) = [.uniquifying] := by decide

/-- "the player with the highest life total" -/
theorem okPlayerStatSuperlative :
    Predicate.check .player [] (.superlative .max (.playerStat .lifeTotal) .anyPlayer) = [] := by
  decide

/-- "the creature with the highest life total among creatures you control" -/
theorem badLifeTotalSuperlative :
    Predicate.check .object [] (.superlative .max (.playerStat .lifeTotal) creature)
      = [.projScope .object] := by
  decide

/-- "this creature" -/
theorem okAscribeCreature : NounPhrase.check (some .object) [] (.asType .creature .this none) = [] := by decide

/-- "this instant" -/
theorem badAscribeInstant :
    NounPhrase.check (some .object) [] (.asType .instant .this none) = [.ascriptionOk] := by decide

/-- "this sorcery" -/
theorem badAscribeSorcery :
    NounPhrase.check (some .object) [] (.asType .sorcery .this none) = [.ascriptionOk] := by decide

/-- "this kindred" -/
theorem badAscribeKindred :
    NounPhrase.check (some .object) [] (.asType .kindred .this none) = [.ascriptionOk] := by decide

/-- "this Aura land" -/
theorem badAscribeForeignSubtype :
    NounPhrase.check (some .object) [] (.asType .land .this (some (enchantmentType "Aura")))
      = [.ascriptionOk] := by
  decide

/-- "target creature that's goaded" -/
theorem okObjectDesignation :
    Predicate.check .object [] (.hasDesignation "goaded" none) = [] := by decide

/-- "target creature that is the monarch" -/
theorem badObjectMonarch :
    Predicate.check .object [] (.hasDesignation "the monarch" none)
      = [.designationHolder "the monarch" .object] := by
  decide

/-- "one of the top two cards of your library" -/
theorem okPartitiveOfSlice :
    NounPhrase.check (some .object) []
      (.someOf (.counted (exactly 1)) none (.librarySlice .top (.lit 2) .you)) = [] := by
  decide

/-- "one of one or more creatures" -/
theorem badPartitiveOfCountedGroup :
    NounPhrase.check (some .object) []
      (.someOf (.counted (exactly 1)) none (counted (atLeast 1) creature)) = [.partitiveBase] := by
  decide

/-- "Destroy target creature. Its controller loses life equal to its power." -/
theorem okItAfterAntecedent :
    Instruction.check []
      (.sequentially [destroy (target creature), losesLife (controllerOf it) (.statOf (.stat .power) it)])
      = [] := by
  decide

theorem badOtherwiseReadsLeadingArm :
    Instruction.check []
      (.if_ (exists_ creatureYouControl)
        (create (.lit 1) (creatureToken 1 1 [.black] [creatureType "Zombie"]))
        (some (.setStatus .tapped it)))
      = [.anaphor .bare .one 0, .zoneIs .battlefield] := by
  decide

theorem badLeadingConditionAntecedent :
    Instruction.check []
      (.sequentially
        [ .if_ (.compareAmt (lifeTotalOf .you) .less
                     (lifeTotalOf anOpponent))
            (gainsLife .you (.lit 6)) none,
          losesLife (that .player) (.lit 1) ])
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "If you control an artifact and an enchantment, …" -/
theorem okFlatConjunction :
    Condition.check []
      (.and [exists_ (.and [artifact, .hasPossessor .controller .you]),
             exists_ (.and [enchantment, .hasPossessor .controller .you])]) = [] := by
  decide

/-- "If you control an artifact, create a token." -/
theorem badSingletonConjunction :
    Condition.check [] (.and [exists_ (.and [artifact, .hasPossessor .controller .you])])
      = [.atLeastTwo] := by
  decide

/-- "If you control an artifact and an enchantment, and you control a land, …" -/
theorem badNestedConjunction :
    Condition.check []
      (.and [ .and [exists_ (.and [artifact, .hasPossessor .controller .you]),
                    exists_ (.and [enchantment, .hasPossessor .controller .you])],
              exists_ (.and [land, .hasPossessor .controller .you]) ]) = [.flatConjuncts] := by
  decide

/-- "Roll two d6. If you rolled 7, sacrifice this creature." -/
theorem okTotalAfterRoll :
    Instruction.check []
      (.sequentially [ rollDice .you 2 6,
                       if_ (.compareAmt (.theOutcome .rollResult) .eq (.lit 7))
                              (sacrifice .you thisCreature) ]) = [] := by
  decide

/-- "If you rolled 7, sacrifice this creature." -/
theorem badTotalWithoutRoll :
    Instruction.check []
      (if_ (.compareAmt (.theOutcome .rollResult) .eq (.lit 7)) (sacrifice .you thisCreature))
      = [.outcomeInScope .rollResult 0] := by
  decide

/-- "Look at the top card of your library. Put that card into your graveyard." -/
theorem okReadsLookedAtLibraryCard :
    NounPhrase.check (some .object) (Instruction.intro [] (lookAt (topSlice (.lit 1)))) (that .card)
      = [] := by
  decide

theorem badReadsShuffledIntoLibraryCard :
    NounPhrase.check (some .object)
      (Instruction.intro [] (.sequentially [lookAt (topSlice (.lit 1)), shuffleInto .you .this]))
      (that .card) = [.anaphor (.word .card) .one 0] := by
  decide

/-- "if up to three is 4 or greater" -/
theorem badCompareCeilingSubject :
    Condition.check [] (.compareAmt (.upTo (.lit 3)) .atLeast (.lit 4)) = [.readAmount] := by decide

/-- "target monocolored permanent": exactly one color -/
theorem monocoloredIsOneColor : Predicate.check .object [] (.colorCount .eq 1) = [] := by decide

/-- "target colorless permanent": zero colors is how "colorless" is spelled [CR#105.2c]. -/
theorem okExactlyZeroColors :
    Predicate.check .object [] (.colorCount .eq 0) = [] := by decide

/-- "target permanent that's exactly six colors" -/
theorem badExactlySixColors :
    Predicate.check .object [] (.colorCount .eq 6) = [.colorBoundOk] := by decide

/-- "if this creature's kicker cost was paid" -/
theorem okPaidCostOnKeywordWithACost :
    Amount.check [] (.paid (.readback (.byKeyword "Kicker") none) .this) = [] := by decide

/-- "if this creature's flying cost was paid" -/
theorem badPaidCostOnCostlessKeyword :
    Amount.check [] (.paid (.readback (.byKeyword "Flying") none) .this) = [.paidFacetNamed] := by
  decide

/-- "for each time it was kickre'd" -/
theorem badTimesPaidUnknownKeyword :
    Amount.check [] (.paid (.timesPaid (.byKeyword "Kickre")) thisCreature) = [.paidFacetNamed] := by
  decide

/-- "for each color of mana spent to cast this spell" -/
theorem okColorsSpentOnThis : Amount.check [] (colorsSpentToCast .this) = [] := by decide

/-- "for each color of mana spent to cast a creature on the battlefield" -/
theorem badPaidReadOffStack :
    Amount.check [] (.paid .colorsSpent (a creature)) = [.paidSubject] := by decide

/-- "if colored mana was spent to cast it" is a read of `paid colorsSpent` -/
theorem coloredManaSpentIsPaidColorsSpent :
    coloredManaSpentToCast .this = .compareAmt (.paid .colorsSpent .this) .atLeast (.lit 1) := rfl

/-- "spell that targets this creature" -/
theorem okSpellTargeter :
    Predicate.check .object [] (.targets thisCreature .someTarget) = [] := by decide

/-- "player that targets this creature" -/
theorem badPlayerTargeter :
    Predicate.check .player [] (.targets thisCreature .someTarget) = [.targeter .player] := by decide

/-- "a creature with power or toughness 2 or greater" -/
theorem okSingleScopeAxisComparison :
    Predicate.check .object [] (.compare [.stat .power, .stat .toughness] .atLeast (.lit 2)) = [] := by
  decide

/-- "a creature with power or life total 3 or greater" -/
theorem badMixedAxisComparison :
    Predicate.check .object [] (.compare [.stat .power, .playerStat .lifeTotal] .greater (.lit 1))
      = [.axesAt .object] := by
  decide

/-- "a player with 13 or less life" -/
theorem okPlayerReadInPlayerDomain :
    Predicate.check .player []
      (.and [.anyPlayer, .compare [.playerStat .lifeTotal] .atMost (.lit 13)]) = [] := by
  decide

/-- "an object with 13 or less life" -/
theorem badPlayerReadInObjectDomain :
    Predicate.check .object [] (.compare [.playerStat .lifeTotal] .atMost (.lit 13))
      = [.axesAt .object] := by
  decide

/-- "target card on the stack" -/
theorem okCardAndSpell : Predicate.check .object [] cardOnTheStack = [] := by decide

/-- "each token on the battlefield" -/
theorem okTokenAndPermanent : Predicate.check .object [] tokenOnTheBattlefield = [] := by decide

/-- "a card token" -/
theorem badCardToken :
    Predicate.check .object [] (.and [.isCard, .isToken]) = [.contradictionFree] := by decide

/-- "an emblem permanent" -/
theorem badEmblemPermanent :
    Predicate.check .object [] (.and [emblem, permanent]) = [.zoneCoherent] := by decide

/-- "a card that is a copy of a card" -/
theorem badCardCopyOfACard :
    Predicate.check .object [] (.and [.isCard, copyOfACard]) = [.contradictionFree] := by decide

/-- "Target creature can't attack this turn." -/
theorem okCantAttackCreature :
    Instruction.check [] (cantAttack (target creature) (some .thisTurn)) = [] := by decide

/-- "Target land can't attack this turn." [CR#508.1a,205.1b,208.3a] -/
theorem okCantAttackLand :
    Instruction.check [] (cantAttack (target land) (some .thisTurn)) = [] := by decide

/-- "Target land can't attack." -/
theorem okCantAttackLandNoSpan : Instruction.check [] (cantAttack (target land) none) = [] := by decide

/-- "Target creature can't block this turn." -/
theorem okCantBlockCreature :
    Instruction.check [] (cantBlock (target creature) (some .thisTurn)) = [] := by decide

/-- "Target creature or land can't block this turn." [CR#205.1b,208.3a] -/
theorem okCantDisjunctSubject :
    Instruction.check [] (cantBlock (target (.or [creature, land])) (some .thisTurn)) = [] := by decide

/-- "your party": one each of Cleric, Rogue, Warrior and Wizard [CR#700.8]. -/
theorem okPartyOfFourRoles : NounPhrase.check (some .object) [] party = [] := by decide

/-- "one each of Cleric and Cleric": a repeated role counts one creature twice [CR#700.8b]. -/
theorem badRepeatedPartyRole :
    NounPhrase.check (some .object) []
      (.oneEachOf [.hasSubtype (creatureType "Cleric"), .hasSubtype (creatureType "Cleric")]
        (allOf creatureYouControl)) = [.rolesOk] := by
  decide

/-- "one each of nothing" [CR#700.8]. -/
theorem badEmptyPartyRoles :
    NounPhrase.check (some .object) [] (.oneEachOf [] (allOf creatureYouControl)) = [.rolesOk] := by decide

/-- "a fortified land" -/
theorem okFortifiedLand :
    NounPhrase.check (some .object) [] (a (.and [land, .isAttached (some .fortified)])) = [] := by decide

/-- "a fortified creature" [CR#301.6,301.5]. -/
theorem badFortifiedCreature :
    NounPhrase.check (some .object) [] (a (.and [creature, .isAttached (some .fortified)]))
      = [.contradictionFree] := by
  decide

end Semantics.Proofs.Description
