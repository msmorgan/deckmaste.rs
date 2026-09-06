import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Keyword

Port of `idris/src/Experimental/Proofs/Keyword.idr`: the pins of the Keyword family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Keyword

/-- An activated ability with no timing, limit, guard, or activator. -/
def act (cost : Cost) (instruction : Instruction) : Ability :=
  .activated cost instruction none none none none

/-- "Sacrifice a creature: Draw a card." -/
theorem okSacrificeAsCost :
    Ability.check [] (act (.perform (sacrifice (a creature) (agent := .you))) (draw (.lit 1) (agent
        := .you))) = [] := by
  decide

/-- "Sacrifice a creature, Exile the sacrificed card: Draw a card." -/
theorem badCostReadsSiblingDeed :
    Ability.check []
      (act
        (.compound
          [ .perform (sacrifice (a creature) (agent := .you)),
            .perform (exile (theVerbed (.action "Sacrifice") .card .attributive .one)) ])
        (draw (.lit 1) (agent := .you))) = [.costAction] := by
  decide

/-- "Target creature gains flying." -/
theorem okGainsKeyword :
    Instruction.check [] (gain (target creature) (keyword "Flying") none) = [] := by decide

/-- "Target creature gains a spell ability." -/
theorem badGainsSpellAbility :
    Instruction.check [] (gain (target creature) (.spell none (draw (.lit 1) (agent := .you))) none)
      = [.grantable] := by
  decide

/-- "Creature spells you cast cost {1} less to cast." -/
theorem okCostSubjectOnStack :
    StaticSpec.check [] (.costShift (allOf (.and [creature, spell])) (.less (.lit 1) none)) = [] :=
        by
  decide

/-- "Creatures you control cost {1} less to cast." -/
theorem badCostSubjectOnBattlefield :
    StaticSpec.check [] (.costShift (allOf creatureYouControl) (.less (.lit 1) none))
      = [.costSubject] := by
  decide

/-- "Creatures you control have flying." -/
theorem okBattlefieldFlying :
    StaticSpec.check [] (.abilityGrant (allOf creatureYouControl) (keyword "Flying")) = [] := by
        decide

/-- "Creatures you control have convoke." -/
theorem badBattlefieldConvoke :
    StaticSpec.check [] (.abilityGrant (allOf creatureYouControl) (keyword "Convoke"))
      = [.grantSubject] := by
  decide

/-- "Target creature gets +1/+1, gains flying, and gains trample." -/
theorem okFlatCoordination :
    StaticSpec.check []
      (.conjunction none
        [ .modification (target creature) .power (.up (.lit 1)),
          .modification it .toughness (.up (.lit 1)),
          .abilityGrant it (keyword "Flying"),
          .abilityGrant it (keyword "Trample") ]) = [] := by
  decide

/-- "Target creature gets +1/+1 and gains flying and gains trample." -/
theorem badNestedCoordination :
    StaticSpec.check []
      (.conjunction none
        [ .conjunction none
            [ .modification (target creature) .power (.up (.lit 1)),
              .modification it .toughness (.up (.lit 1)),
              .abilityGrant it (keyword "Flying") ],
          .abilityGrant it (keyword "Trample") ]) = [.notCoord] := by
  decide

/-- "Whenever a creature enters, destroy that creature." -/
theorem okThatCreatureAfterAntecedent :
    Ability.check []
      (whenever (.enters (a creature) none) (destroy (that (.type .creature))))
      = [] := by
  decide

/-- "This creature gets +1/+1 and that creature has flying." -/
theorem badThatCreatureIsStaticSubject :
    Ability.check []
      (.static
        (.conjunction none
          [ .modification thisCreature .power (.up (.lit 1)),
            .modification it .toughness (.up (.lit 1)),
            .abilityGrant (that (.type .creature)) (keyword "Flying") ]))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Creatures you control get +1/+1 and they have flying." -/
theorem okCoordinatedPlural :
    Ability.check []
      (.static
        (.conjunction none
          [ .modification (allOf creatureYouControl) .power (.up (.lit 1)),
            .modification them .toughness (.up (.lit 1)),
            .abilityGrant them (keyword "Flying") ])) = [] := by
  decide

/-- "Enchanted creature gets +1/+1 and they have flying." -/
theorem badCoordinatedHostPlural :
    Ability.check []
      (.static
        (.conjunction none
          [ .modification (.attachHost .enchanted (.type .creature)) .power (.up (.lit 1)),
            .modification it .toughness (.up (.lit 1)),
            .abilityGrant them (keyword "Flying") ])) = [.anaphor .bare .many 0] := by
  decide

/-- "Create a 1/1 white Soldier creature token with flying." -/
theorem okTokenKeywordAbility :
    Instruction.check []
      (create (.lit 1)
        { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [creatureType "Soldier"],
            power := some (.lit 1), toughness := some (.lit 1), text := [keyword "Flying"] } })
      = [] := by
  decide

/-- "Create a 1/1 white Soldier creature token with 'Draw two cards.'" -/
theorem badTokenSpellAbility :
    Instruction.check []
      (create (.lit 1)
        { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [creatureType "Soldier"],
            power := some (.lit 1), toughness := some (.lit 1),
            text := [.spell none (draw (.lit 1) (agent := .you))] } }) = [.tokenAbilities] := by
  decide

/-- "Creatures you control have '{T}: Draw a card.'" -/
theorem okQuotedGrantOnPermanent :
    StaticSpec.check [] (.abilityGrant (allOf creatureYouControl) (act .tapSymbol (draw (.lit 1)
        (agent := .you))))
      = [] := by
  decide

/-- "Instant and sorcery spells you cast have '{T}: Draw a card.'" -/
theorem badQuotedGrantOnSpell :
    StaticSpec.check []
      (.abilityGrant (allOf (.and [instantOrSorcery, spell, castBy .you]))
        (act .tapSymbol (draw (.lit 1) (agent := .you)))) = [.grantSubject] := by
  decide

/-- "of the creature type of your choice" -/
theorem okYourChoiceCreatureType :
    Predicate.check .object [] (.ofYourChoice (.subtype .creature) none) = [] := by decide

/-- "of the number of your choice" -/
theorem badYourChoiceNumber :
    Predicate.check .object [] (.ofYourChoice .number none) = [.chosenQualityRead .number] := by
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
    (getsPt (allOf (.and [creature, ofChosen (.subtype .creature)])) (.up (.lit 1)) (.up (.lit 1)))

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
                    (.keyword "Protection" [.quality (ofChosen .color)] none)),
                .static (Primitives.StaticSpec.entryChoice thisCreature (.quality .color) none .openly) ] } })
      = [.choiceRef .theChoice (.quality .color) 0] := by
  decide

theorem badAscribedQualityBeforeChoice :
    Card.check
      (enchantmentWith
        [ .static
            (Primitives.StaticSpec.qualityChange (allOf (.and [creature, .hasPossessor .controller .you])) .adds
              (.chosenQuality (ofChosen (.subtype .creature)))),
          choosesCreatureType ]) = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

/-- "{T}: Draw a card. Activate only once each turn." -/
theorem okActivatedTurnLimit :
    Ability.check [] (.activated .tapSymbol (draw (.lit 1) (agent := .you)) none (some .oncePerTurn)
        none none)
      = [] := by
  decide

/-- "{T}: Draw a card. Do this only once each turn." -/
theorem badActionLimitOnActivated :
    Ability.check []
      (.activated .tapSymbol (draw (.lit 1) (agent := .you)) none (some .actionOncePerTurn) none
          none)
      = [.untriggeredLimit] := by
  decide

/-- "I — while you control a creature, draw a card." -/
theorem badChapterWhile :
    Ability.check []
      (.triggered (.chapterMark [1]) []
        (some (.whileTrue (exists_ (.and [creature, .hasPossessor .controller .you])))) [] none
        none none (draw (.lit 1) (agent := .you))) = [.chapterDefaults] := by
  decide

/-- "I — and whenever you draw a card, draw a card." -/
theorem badChapterJoin :
    Ability.check []
      (.triggered (.chapterMark [1]) [] none [⟨.draws .you, [], none, none⟩]
        none none none (draw (.lit 1) (agent := .you))) = [.chapterDefaults] := by
  decide

/-- "As long as a card exiled with this creature has flying, this creature has flying." -/
def flyingWhileExiledFlying : Ability :=
  .static
    (.conditional (.abilityGrant thisCreature (keyword "Flying"))
      (exists_ (.and [.exiledWith thisCreature, .hasKeyword (.the "Flying")])) .asLongAs)

/-- "The same is true for menace and trample." -/
theorem okKeywordListOverGrant :
    Ability.check [] (.alsoForKeywords flyingWhileExiledFlying [.the "Menace", .the "Trample"])
      = [] := by
  decide

/-- "Creatures you control get +1/+1. The same is true for menace and trample." The Idris pin
refutes the extension; the keyword list has no base keyword to repeat either. -/
theorem badKeywordListOnPlainLine :
    Ability.check []
      (.alsoForKeywords (.static (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1))))
        [.the "Menace", .the "Trample"]) = [.keywordExtendable, .keywordListOk] := by
  decide

theorem badEmptyKeywordList :
    Ability.check [] (.alsoForKeywords flyingWhileExiledFlying []) = [.keywordListOk] := by decide

theorem badKeywordListRepeatingBase :
    Ability.check []
      (.alsoForKeywords flyingWhileExiledFlying [.the "Menace", .the "Flying", .the "Trample"])
      = [.keywordListOk] := by
  decide

theorem badParameterisedKeywordInList :
    Ability.check [] (.alsoForKeywords flyingWhileExiledFlying [.the "Ward"])
      = [.keywordListOk] := by
  decide

/-- "the number of basic land types among lands you control" -/
theorem okBasicLandTypeAxis :
    Amount.check []
      (.distinctCount (.subtype .land .basicOnly)
        (allOf (.and [land, .hasPossessor .controller .you]))) = [] := by
  decide

/-- "the number of basic creature types among creatures you control" -/
theorem badBasicCreatureTypeAxis :
    Amount.check [] (.distinctCount (.subtype .creature .basicOnly) (allOf creatureYouControl))
      = [.kindAxisSort] := by
  decide

/-- "Echo {2}{W}" -/
theorem okParameterisedEcho :
    Ability.check [] (keywordCosting "Echo" (.mana [generic 2, pip .white])) = [] := by decide

/-- "Echo" -/
theorem badBareEcho :
    Ability.check [] (.keyword "Echo" [] none) = [.keywordParamFits "Echo"] := by decide

/-- "each creature with flying" -/
theorem okKnownKeywordClass : AbilityClass.known (.keyword "Flying") = true := by decide

/-- "each creature with flyign": the refusal names the label and the table -/
theorem badUnknownKeywordClass : AbilityClass.known (.keyword "Flyign") = false := by decide

/-- "{T}: This deals X damage to any target, where X is 3." -/
theorem okActivatedOpensItsOwnLetter :
    Ability.check []
      (activated .tapSymbol
        (.sequentially
          [.dealDamage .this (.letter .x) (target anyTarget), Primitives.Instruction.define .x (.lit 3)])) = [] := by
  decide

/-- "{T}: … , where X is 3" -/
theorem badActivatedClosesCardLetter :
    Ability.check (costLetters (some [.variable])) (activated .tapSymbol (Primitives.Instruction.define .x (.lit 3)))
      = [.openLetter .x] := by
  decide

theorem sharedSubjectSurvivesSecondSingular :
    Instruction.check []
      (.sequentially
        [ exile (target artifact),
          establishFor (target creature)
            [ .modification (ownSubject (target creature)) .power (.up (.lit 1)),
              .modification (ownSubject (target creature)) .toughness (.up (.lit 1)),
              .abilityGrant (ownSubject (target creature)) (keyword "Flying") ]
            (some untilEndOfTurn) ]) = [] := by
  decide

/-- "target creature": the subject a shared-subject clause re-reads -/
theorem okSharedSubjectDelta :
    NounPhrase.check (some .object) (selfSubjIntro [] (target creature))
      (ownSubject (target creature)) = [] := by
  decide

theorem badSharedSubjectEmptyDelta :
    NounPhrase.check (some .object) [] (ownSubject (itVerbed (.action "Untap")))
      = [.anaphor .bare .one 0] := by
  decide

/-- "Pay 2 life: Draw a card." -/
theorem okOwnPayerCost : Ability.check [] (act (payLife .you 2) (draw (.lit 1) (agent := .you))) =
    [] := by
  decide

/-- "An opponent pays 2 life: Draw a card." -/
theorem badForeignPayerCost :
    Ability.check [] (act (payLife anOpponent 2) (draw (.lit 1) (agent := .you))) = [.costPaidByYou]
        := by
  decide

/-- "An opponent sacrifices a creature: Draw a card." -/
theorem badForeignSacrificeCost :
    Ability.check [] (act (.perform (sacrifice (a creature) (agent := anOpponent))) (draw (.lit 1)
        (agent := .you)))
      = [.costPaidByYou] := by
  decide

/-- "Counter target spell." -/
theorem okCounterSpell : Instruction.check [] (Primitives.Instruction.counterSpell (target spell)) = [] := by decide

/-- "Counter target creature." -/
theorem badCounterPermanent :
    Instruction.check [] (Primitives.Instruction.counterSpell (target creature)) = [.zoneFits, .zoneFits, .zoneFits] := by decide

/-- "Counter target creature or player." -/
theorem badCounterJoinedPlayer :
    Instruction.check [] (Primitives.Instruction.counterSpell (target anyTarget)) = [.zoneFits, .zoneFits] := by decide

/-- "Companion — Each permanent card in your starting deck has mana value 2 or less." -/
theorem okCompanionCharacteristicRead :
    Ability.check []
      (companion
        (.everyCardIs permanentCard
          (.aCharacteristic (.compare [.stat .manaValue] .atMost (.lit 2))))) = [] := by
  decide

/-- "Companion — Each card on the battlefield in your starting deck ...": a zone is a place
objects are during a game [CR#400.1] and the starting deck is outside it [CR#103.2b], so it is
not a characteristic [CR#109.3]. `permanentCard` spells "each permanent card". -/
theorem badCompanionZoneScope :
    Ability.check []
      (companion
        (.everyCardIs (.inZone battlefield)
          (.aCharacteristic (.compare [.stat .manaValue] .atMost (.lit 2)))))
      = [.deckReadable] := by
  decide

/-- "Companion — Each permanent card in your starting deck is tapped.": whether a permanent is
tapped is not a characteristic [CR#109.3], and a card set aside for the starting deck is not on
the battlefield [CR#103.2b]. -/
theorem badCompanionBattlefieldStatus :
    Ability.check []
      (companion
        (.everyCardIs permanentCard (.aCharacteristic (.hasStatus .tapped))))
      = [.deckReadable] := by
  decide

/-- "Companion — Each nonland card in your starting deck shares a card type." -/
theorem okCompanionSharedCardType :
    Ability.check [] (companion (.cardsShare (.and [.not land, .isCard]) .cardType)) = [] := by
  decide

/-- "Companion — Each nonland card in your starting deck shares a color.": color is a
characteristic [CR#109.3] and the deck-building condition is checked against the starting deck
[CR#702.139a]. -/
theorem okCompanionSharedColor :
    Ability.check [] (companion (.cardsShare (.and [.not land, .isCard]) .color)) = [] := by
  decide

/-- "Companion — Each nonland card in your starting deck shares a counter kind.": a kind of
counter is not among an object's characteristics [CR#109.3]. -/
theorem badCompanionSharedCounterKind :
    Ability.check [] (companion (.cardsShare (.and [.not land, .isCard]) .counterKind))
      = [.deckComparable] := by
  decide

/-- "Each player scries 1.": one scry clause over a distributed player reference
[CR#701.22a,701.22c]. -/
theorem okEachPlayerScriesOne :
    Instruction.check [] (scry (.lit 1) (agent := (each .anyPlayer))) = [] := by decide

/-- "Fateseal 2.": the sorted library is an opponent's [CR#701.29a]. -/
theorem okFatesealAnOpponent :
    Instruction.check [] (fateseal anOpponent (.lit 2) (agent := .you)) = [] := by decide

/-- "Fateseal 2" over your own library: fateseal is defined only over an opponent's library
[CR#701.29a]; looking at your own top cards and sorting them is scry [CR#701.22a], spelled
`scry`. -/
theorem badFatesealYourOwnLibrary :
    Instruction.check [] (fateseal .you (.lit 2) (agent := .you)) = [.opponentsLibrary] := by decide

/-- "Choose flying or trample. This creature gains that ability until end of turn." -/
theorem okThatAbilityAfterChoice :
    Instruction.check []
      (.sequentially
        [ choose (a (qualityFrom .ability (.abilitiesAmong [.the "Flying", .the "Trample"]))),
          gain thisCreature (.thatAbility .theChoice) (some untilEndOfTurn) ]) = [] := by
  decide

/-- "This creature gains that ability until end of turn", with no ability chosen anywhere in
the text. -/
theorem badThatAbilityWithoutChoice :
    Instruction.check [] (gain thisCreature (.thatAbility .theChoice) (some untilEndOfTurn))
      = [.choiceRef .theChoice (.quality .ability) 0] := by
  decide

end Semantics.Proofs.Keyword
