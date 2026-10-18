import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Keyword

Port of `idris/src/Experimental/Proofs/Keyword.idr`: the pins of the Keyword family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Keyword

private def nounResult (drs : DRS) (noun : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .other false noun).1

/-- An activated ability with no timing, limit, guard, or activator. -/
def activatedAbility (cost : Cost) (instruction : Action) : Ability :=
  .activated cost instruction none none none none

/-- "Sacrifice a creature: Draw a card." -/
theorem okSacrificeAsCost :
    Ability.check (DRS.ofList []) (activatedAbility (.perform (sacrifice (a isCreature))) (draw (.lit 1))) = [] := by
  decide

/-- "Target creature gains flying." -/
theorem okGainsKeyword :
    Action.check (DRS.ofList []) (gain (target isCreature) (keyword (.named "flying")) none) = [] := by decide

/-- "Creature spells you cast cost {1} less to cast." -/
theorem okCostSubjectOnStack :
    StaticSpec.check (DRS.ofList []) (.costShift (allOf (.and [isCreature, isSpell])) (.less (.lit 1) none)) = [] :=
        by
  decide

/-- "Creatures you control have flying." -/
theorem okBattlefieldFlying :
    StaticSpec.check (DRS.ofList []) (.abilityGrant (allOf (and [isCreature, youControl])) (keyword (.named "flying"))) = [] := by
        decide

/-- "Target creature gets +1/+1, gains flying, and gains trample." -/
theorem okFlatCoordination :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [ .modification (target isCreature) .power (.up (.lit 1)),
          .modification it .toughness (.up (.lit 1)),
          .abilityGrant it (keyword (.named "flying")),
          .abilityGrant it (keyword (.named "trample")) ]) = [] := by
  decide

/-- "Target creature gets +1/+1 and gains flying and gains trample." -/
theorem badNestedCoordination :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [ .conjunction none
            [ .modification (target isCreature) .power (.up (.lit 1)),
              .modification it .toughness (.up (.lit 1)),
              .abilityGrant it (keyword (.named "flying")) ],
          .abilityGrant it (keyword (.named "trample")) ]) = [.notCoord] := by
  decide

/-- "Whenever a creature enters, destroy that creature." -/
theorem okThatCreatureAfterAntecedent :
    Ability.check (DRS.ofList [])
      (whenever (does (a isCreature) enter) (destroy (that (.type .creature))))
      = [] := by
  decide

/-- "This creature gets +1/+1 and that creature has flying." -/
theorem staticSelfRemainsReadableAsThatCreature :
    let spec := StaticSpec.conjunction none
      [.modification thisCreature .power (.up (.lit 1)),
        .modification it .toughness (.up (.lit 1)),
        .abilityGrant (that (.type .creature)) (keyword (.named "flying"))]
    let after := (stepStaticSpec 64 (DRS.ofList []) spec).1
    Ability.check (DRS.ofList []) (.static spec) = [] ∧
      (firstReach (.word (.type .creature)) .one (nextClause after)).map (·.id) = some 0 ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

/-- "Creatures you control get +1/+1 and they have flying." -/
theorem okCoordinatedPlural :
    Ability.check (DRS.ofList [])
      (.static
        (.conjunction none
          [ .modification (allOf (and [isCreature, youControl])) .power (.up (.lit 1)),
            .modification them .toughness (.up (.lit 1)),
            .abilityGrant them (keyword (.named "flying")) ])) = [] := by
  decide

/-- "Enchanted creature gets +1/+1 and they have flying." -/
theorem badCoordinatedHostPlural :
    Ability.check (DRS.ofList [])
      (.static
        (.conjunction none
          [ .modification (.attachHost .enchanted (.type .creature)) .power (.up (.lit 1)),
            .modification it .toughness (.up (.lit 1)),
            .abilityGrant them (keyword (.named "flying")) ])) = [.anaphor .bare .many 0] := by
  decide

/-- "Create a 1/1 white Soldier creature token with flying." -/
theorem okTokenKeywordAbility :
    Action.check (DRS.ofList [])
      (create (.lit 1)
        (.written { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [creatureType "Soldier"],
            power := some (.lit 1), toughness := some (.lit 1), text := [keyword (.named "flying")] } }))
      = [] := by
  decide

/-- "Creatures you control have '{T}: Draw a card.'" -/
theorem okQuotedGrantOnPermanent :
    StaticSpec.check (DRS.ofList []) (.abilityGrant (allOf (and [isCreature, youControl])) (activatedAbility .tapSymbol (draw (.lit 1))))
      = [] := by
  decide

/-- "of the creature type of your choice" -/
theorem okYourChoiceCreatureType :
    Predicate.checkAt .object (DRS.ofList []) (.ofYourChoice (.subtype .creature) none) = [] := by decide

/-- "of the number of your choice" -/
theorem badYourChoiceNumber :
    Predicate.checkAt .object (DRS.ofList []) (.ofYourChoice .number none) = [.chosenQualityRead .number] := by
  decide

/-- An enchantment with the given text. -/
def enchantmentWith (text : List Ability) : Card :=
  .singleFaced { characteristics := { name := "", types := [.enchantment], text } }

/-- "As this enchantment enters, choose a creature type." -/
def choosesCreatureType : Ability :=
  .static (Primitives.StaticSpec.entryChoice thisEnchantment (.quality (.subtype .creature)) none .openly)

/-- "Creatures of the chosen type get +1/+1." -/
def chosenTypeGets : Ability :=
  .static
    (getsPt (allOf (.and [isCreature, ofChosen (.subtype .creature)])) (.up (.lit 1)) (.up (.lit 1)))

/-- "As this enchantment enters, choose a creature type. Creatures of the chosen type get
+1/+1." -/
theorem okReaderAfterChooser :
    Card.check (enchantmentWith [choosesCreatureType, chosenTypeGets]) = [] := by decide

theorem badReaderBeforeChooser :
    Card.check (enchantmentWith [chosenTypeGets, choosesCreatureType])
      = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

theorem badTwoChoosersOneSortRead :
    Card.check (enchantmentWith [choosesCreatureType, choosesCreatureType, chosenTypeGets])
      = [.choiceRef .theChoice (.quality (.subtype .creature)) 2] := by
  decide

theorem badChosenReadWrongSort :
    Card.check
      (enchantmentWith
        [ .static (Primitives.StaticSpec.entryChoice thisEnchantment (.quality .color) none .openly),
          chosenTypeGets ]) = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

theorem badChosenProtectionBeforeChoice :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.creature], subtypes := [creatureType "Angel"],
            power := stat 2, toughness := stat 2,
            text :=
              [ .static
                  (.abilityGrant thisCreature
                    (.keyword (.named "protection") [.quality (ofChosen .color)] [])),
                .static (Primitives.StaticSpec.entryChoice thisCreature (.quality .color) none .openly) ] } })
      = [.choiceRef .theChoice (.quality .color) 0] := by
  decide

theorem badAscribedQualityBeforeChoice :
    Card.check
      (enchantmentWith
        [ .static
            (Primitives.StaticSpec.qualityChange (allOf (.and [isCreature, .hasPossessor .controller .you])) .adds
              (.chosenQuality (ofChosen (.subtype .creature)))),
          choosesCreatureType ]) = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

/-- "{T}: Draw a card. Activate only once each turn." -/
theorem okActivatedTurnLimit :
    Ability.check (DRS.ofList []) (.activated .tapSymbol (draw (.lit 1)) none (some oncePerTurn)
        none none)
      = [] := by
  decide

/-- "As long as a card exiled with this creature has flying, this creature has flying." -/
def flyingWhileExiledFlying : Ability :=
  .static
    (.conditional (.abilityGrant thisCreature (keyword (.named "flying")))
      (exists_ (.and [.in_ (.exiledWith thisCreature), .hasKeyword (.the (.named "flying"))])) .asLongAs)

/-- "The same is true for menace and trample." -/
theorem okKeywordListOverGrant :
    Ability.check (DRS.ofList []) (.alsoForKeywords flyingWhileExiledFlying [.the (.named "menace"), .the (.named "trample")])
      = [] := by
  decide

/-- "Creatures you control get +1/+1. The same is true for menace and trample." The Idris pin
refutes the extension; the keyword list has no base keyword to repeat either. -/
theorem badKeywordListOnPlainLine :
    Ability.check (DRS.ofList [])
      (.alsoForKeywords (.static (getsPt (allOf (and [isCreature, youControl])) (.up (.lit 1)) (.up (.lit 1))))
        [.the (.named "menace"), .the (.named "trample")]) = [.keywordExtendable, .keywordListOk] := by
  decide

theorem badEmptyKeywordList :
    Ability.check (DRS.ofList []) (.alsoForKeywords flyingWhileExiledFlying []) = [.keywordListOk] := by decide

theorem badKeywordListRepeatingBase :
    Ability.check (DRS.ofList [])
      (.alsoForKeywords flyingWhileExiledFlying [.the (.named "menace"), .the (.named "flying"), .the (.named "trample")])
      = [.keywordListOk] := by
  decide

theorem badParameterisedKeywordInList :
    Ability.check (DRS.ofList []) (.alsoForKeywords flyingWhileExiledFlying [.the (.named "ward")])
      = [.keywordListOk] := by
  decide

/-- "the number of basic land types among lands you control" -/
theorem okBasicLandTypeAxis :
    Amount.check (DRS.ofList [])
      (.distinctCount (.subtype .land .basicOnly)
        (allOf (.and [isLand, .hasPossessor .controller .you]))) = [] := by
  decide

/-- "the number of basic creature types among creatures you control" -/
theorem badBasicCreatureTypeAxis :
    Amount.check (DRS.ofList []) (.distinctCount (.subtype .creature .basicOnly) (allOf (and [isCreature, youControl])))
      = [.kindAxisSort] := by
  decide

/-- "Echo {2}{W}" -/
theorem okParameterisedEcho :
    Ability.check (DRS.ofList []) (keywordCosting (.named "echo") (.mana [generic 2, pip .white])) = [] := by decide

/-- "Echo" -/
theorem badBareEcho :
    Ability.check (DRS.ofList []) (.keyword (.named "echo") [] []) = [.keywordParamFits (.named "echo")] := by decide

/-- "each creature with flying" -/
theorem okKnownKeywordClass : AbilityClass.known (.keyword (.named "flying")) = true := by decide

/-- "each creature with flyign": the refusal names the label and the table -/
theorem badUnknownKeywordClass : AbilityClass.known (.keyword (.named "flyign")) = false := by decide

/-- "{T}: This deals X damage to any target, where X is 3." -/
theorem okActivatedOpensItsOwnLetter :
    Ability.check (DRS.ofList [])
      (activated .tapSymbol
        (.sequentially
          [(Action.instruct .this (.dealDamage (.letter .x) (target isAnyTarget) .any)), Primitives.Action.define .x (.lit 3)])) = [] := by
  decide

/-- "{T}: … , where X is 3" -/
theorem badActivatedClosesCardLetter :
    Ability.check (costLetters (some [.variable])) (activated .tapSymbol (Primitives.Action.define .x (.lit 3)))
      = [.openLetter .x] := by
  decide

theorem activatedManaCostIntroducesItsOwnX :
    Ability.check (costLetters (some [.variable]))
      (activated (.mana [.variable]) (Templates.instruct .you (draw (.letter .x)))) = [] := by
  decide

theorem sharedSubjectSurvivesSecondSingular :
    Action.check (DRS.ofList [])
      (.sequentially
        [ exile (target isArtifact),
          establishFor (.firstMention "sharedSubject.subject" (target isCreature))
            [ .modification (.laterMention "sharedSubject.subject" (target isCreature)) .power (.up (.lit 1)),
              .modification (.laterMention "sharedSubject.subject" (target isCreature)) .toughness (.up (.lit 1)),
              .abilityGrant (.laterMention "sharedSubject.subject" (target isCreature)) (keyword (.named "flying")) ]
            (some untilEndOfTurn) ]) = [] := by
  decide

/-- "target creature": the subject a shared-subject clause re-reads -/
theorem okSharedSubjectDelta :
    NounPhrase.checkAt (some .object) (nounResult (DRS.ofList []) (.firstMention "sharedSubject.subject" (target isCreature)))
      (.laterMention "sharedSubject.subject" (target isCreature)) = [] := by
  decide

theorem badSharedSubjectEmptyDelta :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.laterMention "sharedSubject.subject" (theVerbed (.action "untap") .one))
      = [.bindingIdentity "sharedSubject.subject"] := by
  decide

/-- "Pay 2 life: Draw a card." -/
theorem okOwnPayerCost : Ability.check (DRS.ofList []) (activatedAbility (payLife 2) (draw (.lit 1))) =
    [] := by
  decide

/-- "Counter target spell." -/
theorem okCounterSpell : Action.check (DRS.ofList []) (counter (target isSpell)) = [] := by decide

/-- "Counter target creature or player." -/
theorem badCounterJoinedPlayer :
    Action.check (DRS.ofList []) (counter (target isAnyTarget))
      = [.deedFits, .kindLte (.join .object .player) (.join .object .pile)] := by
  decide

/-- "Companion — Each permanent card in your starting deck has mana value 2 or less." -/
theorem okCompanionCharacteristicRead :
    Ability.check (DRS.ofList [])
      (companion
        (.everyCardIs isPermanentCard
          (.aCharacteristic (.compare [.stat .manaValue] .atMost (.lit 2))))) = [] := by
  decide

/-- "Companion — Each card on the battlefield in your starting deck ...": a zone is a place
objects can occupy [CR#400.1], not a characteristic [CR#109.3]. Companion evaluates its
condition against the starting deck before the game begins [CR#702.139a]. `isPermanentCard` spells "each permanent card". -/
theorem badCompanionLocationScope :
    Ability.check (DRS.ofList [])
      (companion
        (.everyCardIs (.in_ battlefield)
          (.aCharacteristic (.compare [.stat .manaValue] .atMost (.lit 2)))))
      = [.deckReadable] := by
  decide

/-- "Companion — Each permanent card in your starting deck is tapped.": whether a permanent is
tapped is not a characteristic [CR#109.3]. Companion evaluates the starting deck before
the game begins [CR#702.139a]. -/
theorem badCompanionBattlefieldStatus :
    Ability.check (DRS.ofList [])
      (companion
        (.everyCardIs isPermanentCard (.aCharacteristic (.hasStatus .tapped))))
      = [.deckReadable] := by
  decide

/-- "Companion — Each nonland card in your starting deck shares a card type." -/
theorem okCompanionSharedCardType :
    Ability.check (DRS.ofList []) (companion (.groupAgrees (.agree .cardType .share (allOf (.and [.not isLand, .isCard]))))) = [] := by
  decide

/-- "Companion — Each nonland card in your starting deck shares a color.": color is a
characteristic [CR#109.3] and the deck-building condition is checked against the starting deck
[CR#702.139a]. -/
theorem okCompanionSharedColor :
    Ability.check (DRS.ofList []) (companion (.groupAgrees (.agree .color .share (allOf (.and [.not isLand, .isCard]))))) = [] := by
  decide

/-- "Companion — Each nonland card in your starting deck shares a counter kind.": a kind of
counter is not among an object's characteristics [CR#109.3]. -/
theorem badCompanionSharedCounterKind :
    Ability.check (DRS.ofList []) (companion (.groupAgrees (.agree .counterKind .share (allOf (.and [.not isLand, .isCard])))))
      = [.deckComparable] := by
  decide

/-- "Each player scries 1.": one scry clause over a distributed player reference
[CR#701.22a,701.22c]. -/
theorem okEachPlayerScriesOne :
    Action.check (DRS.ofList []) (Templates.instruct (each .anyPlayer) (scry (.lit 1))) = [] := by decide

/-- "Fateseal 2.": the sorted library is an opponent's [CR#701.29a]. -/
theorem okFatesealAnOpponent :
    Action.check (DRS.ofList []) (fateseal (.lit 2)) = [] := by decide

/-- "Choose flying or trample. This creature gains that ability until end of turn." -/
theorem okThatAbilityAfterChoice :
    Action.check (DRS.ofList [])
      (.sequentially
        [ choose (a (qualityFrom .ability (.abilitiesAmong [.the (.named "flying"), .the (.named "trample")]))),
          gain thisCreature (.thatAbility .theChoice) (some untilEndOfTurn) ]) = [] := by
  decide

/-- "This creature gains that ability until end of turn", with no ability chosen anywhere in
the text. -/
theorem badThatAbilityWithoutChoice :
    Action.check (DRS.ofList []) (gain thisCreature (.thatAbility .theChoice) (some untilEndOfTurn))
      = [.choiceRef .theChoice (.quality .ability) 0] := by
  decide

end Semantics.Proofs.Keyword
