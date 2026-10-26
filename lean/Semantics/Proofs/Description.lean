import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.Helpers

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
    Action.check (DRS.ofList []) (lookAt (.slice .top 1 (libraryOf (target .isPlayer)))) = [] := by decide

/-- "Look at the top card of two target players' library." -/
theorem badSliceOfCountedPossessor :
    Action.check (DRS.ofList [])
      (lookAt (.slice .top 1 (libraryOf (.described (.target (exactly 2)) .isPlayer))))
      = [.slicePossessor] := by
  decide

/-- "Tap target creature an opponent controls. That player loses 1 life." -/
theorem okThatPlayer :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .setStatus .tapped (target (.and [isCreature, .hasPossessor .controller anOpponent])),
          instruct (that .player) (loseLife 1) ]) = [] := by
  decide

/-- "Tap target creature an opponent controls or land you control. That player loses 1 life." -/
theorem badDisjunctAntecedent :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .setStatus .tapped (target (.or [.and [isCreature, .hasPossessor .controller anOpponent],
                                           .and [isLand, .hasPossessor .controller .you]])),
          instruct (that .player) (loseLife 1) ])
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
    Predicate.checkAt .object (DRS.ofList []) (.and [isCreature, .compare [.stat .power] .atMost 2]) = [] := by
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
                      doOnlyIf (gainLife 1) (.matches it isArtifact) none]) =
                          [] := by
  decide

/-- "Tap target creature. You gain 1 life if it's." -/
theorem badMatchesNothing :
    Action.check (DRS.ofList [])
      (.sequentially [.setStatus .tapped (target isCreature),
                      doOnlyIf (gainLife 1) (.matches it (.and [])) none])
      = [.predSays] := by
  decide

/-- "if the number of artifacts you control is 4 or greater" -/
theorem okCompareCountSubject :
    Condition.check (DRS.ofList [])
      (.compareAmt (countOf (.and [isArtifact, .hasPossessor .controller .you])) .atLeast 4)
      = [] := by
  decide

/-- "if 3 is 4 or greater" -/
theorem badCompareLiteralSubject :
    Condition.check (DRS.ofList []) (.compareAmt 3 .atLeast 4) = [.readAmount] := by decide

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
    Amount.check (DRS.ofList []) (aggregate .max (.stat .power) .isPlayer) = [.projScope .player] := by decide

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

/-! ## A definite description reads back a mention

A `the` description whose predicate does not make it unique reads back the one earlier mention in
scope whose recorded facts say what its predicate says (owner ruling 2026-10-24): after "Choose
target creature you control and target creature you don't control", "the creature you control"
is the first target. The targets were chosen as the spell was cast [CR#601.2c]; the description
chooses nothing. A predicate that makes the description unique describes afresh. -/

private def creatureYouControl : Predicate := and [isCreature, youControl]
private def creatureYouDontControl : Predicate := and [isCreature, youDontControl]

private def chosenFightPair : DRS :=
  actionResult (DRS.ofList [])
    (choose (both (target creatureYouControl) (target creatureYouDontControl)))

/-- The id of the one target `chosenFightPair` recorded with controller fact `fact`. -/
private def fightPairTarget (fact : Bool) : List ReferentId :=
  (chosenFightPair.referents.filter fun ref =>
    ref.det == .target && ref.controlledByYou == some fact).map (·.id)

/-- "Choose target creature you control and target creature you don't control. The creature you
control gets +2/+1 until end of turn.": each description reads back the target its restriction
picks out, and introduces no object ("you control" mentions you, as any description does). -/
theorem okDefiniteReadsBackChosenTarget :
    (stepNounPhrase 64 chosenFightPair .other false (the creatureYouControl)).2 =
        (fightPairTarget true, []) ∧
      (stepNounPhrase 64 chosenFightPair .other false (the creatureYouDontControl)).2 =
        (fightPairTarget false, []) ∧
      fightPairTarget true ≠ fightPairTarget false ∧
      ((stepNounPhrase 64 chosenFightPair .other false (the creatureYouControl)).1.referents.filter
        fun ref => ref.id ≥ chosenFightPair.nextId && ref.kind == .object).isEmpty ∧
      Action.check (DRS.ofList [])
        (sequentially
          [ choose (both (target creatureYouControl) (target creatureYouDontControl)),
            get (the creatureYouControl) (up 2) (up 1) untilEndOfTurn ]) = [] := by
  decide

/-- "Choose target creature you control. Tap the creature you control.": the readback does not
depend on coordination. -/
theorem okDefiniteReadsBackOneTarget :
    Action.check (DRS.ofList [])
      (sequentially [choose (target creatureYouControl), tap (the creatureYouControl)]) = [] := by
  decide

/-- "Tap the creature you control." with no mention in scope, and "the creature an opponent
controls" after "target creature you don't control", which does not say an opponent controls
it: no mention matches, and "the" names nothing unique. -/
theorem badDefiniteNoMatchingMention :
    Action.check (DRS.ofList []) (tap (the creatureYouControl)) = [.uniquifying] ∧
      (stepNounPhrase 64 chosenFightPair .other false
        (the (and [isCreature, anOpponentControls]))).2.2 = [.uniquifying] := by
  decide

/-- "Choose target creature you control and target creature you control. Tap the creature you
control.": two mentions match, and "the" names neither. -/
theorem badDefiniteTwoMatchingMentions :
    Action.check (DRS.ofList [])
      (sequentially
        [ choose (both (target creatureYouControl) (target creatureYouControl)),
          tap (the creatureYouControl) ]) = [.uniquifying] := by
  decide

/-- "the creature with the greatest power" after "Choose target creature you control and target
creature you don't control": the superlative makes it unique, so it describes afresh and never
reads back an earlier creature mention. -/
theorem okSuperlativeDescribesAfresh :
    let (out, ids, errors) := stepNounPhrase 64 chosenFightPair .other false
        (the (and [isCreature, superlative .max (.stat .power) isCreature]))
    errors = [] ∧ ids.length = 1 ∧ ids = (headerFreshReferents chosenFightPair out).map (·.id) := by
  decide

/-- "the monarch draws a card": only one player can be the monarch at a time [CR#725.3]. -/
theorem okTheMonarch :
    Action.check (DRS.ofList [])
      (instruct (the (and [isPlayer, .hasDesignation monarch none])) (draw 1)) = [] := by decide

/-- "the initiative holder": only one player can have the initiative at a time [CR#726.3]. -/
theorem okTheInitiativeHolder :
    NounPhrase.checkAt (some .player) (DRS.ofList [])
      (the (and [isPlayer, .hasDesignation initiative none])) = [] := by decide

/-- "the player with the city's blessing": several players can each have it, so `the` is
refused. -/
theorem badTheCitysBlessing :
    NounPhrase.checkAt (some .player) (DRS.ofList [])
      (the (and [isPlayer, .hasDesignation citysBlessing none])) = [.uniquifying] := by decide

/-- "the player with the highest life total" -/
theorem okPlayerStatSuperlative :
    Predicate.checkAt .player (DRS.ofList []) (.superlative .max (.playerStat .lifeTotal) .isPlayer) = [] := by
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
    Predicate.checkAt .object (DRS.ofList []) (.hasDesignation (.named "goaded" (.heldBy .object)) none) = [] := by decide

/-- "target creature that is the monarch" -/
theorem badObjectMonarch :
    Predicate.checkAt .object (DRS.ofList []) (.hasDesignation (.named "monarch" (.heldBy .player)) none)
      = [.designationHolder (.named "monarch" (.heldBy .player)) .object] := by
  decide

/-- "one of the top two cards of your library" -/
theorem okPartitiveOfSlice :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.someOf (.counted (exactly 1)) none (.slice .top 2 (libraryOf .you))) = [] := by
  decide

/-- "one of one or more creatures" -/
theorem badPartitiveOfCountedGroup :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.someOf (.counted (exactly 1)) none (counted (orMore 1) isCreature)) = [.partitiveBase] := by
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
        (create 1 (.written (creatureToken 1 1 [.black] [zombie])))
        (some (.setStatus .tapped it)))
      = [.anaphor .bare .one 0] := by
  decide

/-- "Tap target creature. If you control an artifact, exile that creature. Otherwise, sacrifice
it. Return the exiled card to the battlefield." Whichever branch is taken, what it did to a
referent in scope before the `doIf` stands after it (`semantics-v3.md` §1.3, box 2): "the exiled
card" and "the sacrificed card" each read the target as the branch that moved it left it; a deed
neither branch did is still no antecedent. -/
theorem otherwiseKeepsWhatEitherBranchDidToAnOuterReferent :
    let line := fun (later : Action) => Action.sequentially
      [ .setStatus .tapped (target isCreature),
        .doIf (exists_ (and [isArtifact, youControl])) (exile (that (.type .creature)))
          (some (sacrifice it)),
        later ]
    let returned := fun (deed : Predicate) =>
      Action.check (DRS.ofList []) (line (putInto (the (and [deed, isCard])) battlefield))
    returned (was (exile which)) = [] ∧ returned (was (sacrifice which)) = [] ∧
      returned (was (destroy which)) =
        [.participialDescription (.deed (.action "destroy")) 0] := by
  decide

theorem noElseConditionExportsItsOpponent :
    let conditional := .doIf (.compareAmt (lifeTotalOf .you) .less
      (lifeTotalOf anOpponent)) (gainLife 6) none
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) conditional).1
    Action.check (DRS.ofList [])
      (.sequentially [conditional, instruct (that .player) (loseLife 1)]) = [] ∧
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
      (.sequentially [ rollDice 2 (.sides 6),
                       doIf (.compareAmt (.theOutcome .rollResult) .eq 7)
                              (sacrifice thisCreature) ]) = [] := by
  decide

/-- "If you rolled 7, sacrifice this creature." -/
theorem badTotalWithoutRoll :
    Action.check (DRS.ofList [])
      (doIf (.compareAmt (.theOutcome .rollResult) .eq 7) (sacrifice thisCreature))
      = [.outcomeInScope .rollResult 0] := by
  decide

/-- "Look at the top card of your library. Put that card into your graveyard." -/
theorem okReadsLookedAtLibraryCard :
    NounPhrase.checkAt (some .object) (actionResult (DRS.ofList []) (lookAt (topSlice 1))) (that .card)
      = [] := by
  decide

theorem badReadsShuffledIntoLibraryCard :
    NounPhrase.checkAt (some .object)
      (actionResult (DRS.ofList []) (.sequentially [lookAt (topSlice 1), shuffleInto .this]))
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

/-- "for each color of mana spent to cast this spell" -/
theorem okColorsSpentOnThis : Amount.check (DRS.ofList []) (colorsSpentToCast .this) = [] := by decide

/-- "spell that targets this creature" -/
theorem okSpellTargeter :
    Predicate.checkAt .object (DRS.ofList []) (.targets thisCreature .someTarget) = [] := by decide

/-- "player that targets this creature" -/
theorem badPlayerTargeter :
    Predicate.checkAt .player (DRS.ofList []) (.targets thisCreature .someTarget) = [.targeter .player] := by decide

/-- "a creature with power or toughness 2 or greater" -/
theorem okSingleScopeAxisComparison :
    Predicate.checkAt .object (DRS.ofList []) (.compare [.stat .power, .stat .toughness] .atLeast 2) = [] := by
  decide

/-- "a creature with power or life total 3 or greater" -/
theorem badMixedAxisComparison :
    Predicate.checkAt .object (DRS.ofList []) (.compare [.stat .power, .playerStat .lifeTotal] .greater 1)
      = [.axesAt .object] := by
  decide

/-- "a player with 13 or less life" -/
theorem okPlayerReadInPlayerDomain :
    Predicate.checkAt .player (DRS.ofList [])
      (.and [.isPlayer, .compare [.playerStat .lifeTotal] .atMost 13]) = [] := by
  decide

/-- "an object with 13 or less life" -/
theorem badPlayerReadInObjectDomain :
    Predicate.checkAt .object (DRS.ofList []) (.compare [.playerStat .lifeTotal] .atMost 13)
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
    (slice .top 1 (libraryOf eachPlayer)).plur = .many ∧
      (slice .top 1 (graveyardOf eachPlayer)).plur = .many ∧
      (slice .top 1 (libraryOf you)).plur = .one := by
  decide

end Semantics.Proofs.Description
