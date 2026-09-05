import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Choice

Port of `idris/src/Experimental/Cards/Choice.idr`: the printed cards of the Choice family
(choices, modes, may, permissions to play, chosen qualities) and the bench items beside them.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

def carefulStudy : Instruction :=
  .sequentially [.draw .you (.lit 2), discard .you (counted (exactly 2) (.inZone hand))]
theorem okCarefulStudy : Instruction.check [] carefulStudy = [] := by decide

def zombieInfestation : Ability :=
  activated (.perform (discard .you (counted (exactly 2) (.inZone hand))))
    (create (.lit 1) (creatureToken 2 2 [.black] [creatureType "Zombie"]))
theorem okZombieInfestation : Ability.check [] zombieInfestation = [] := by decide

def fulgentDistraction : Instruction :=
  .sequentially
    [ choose (.described (.target (exactly 2)) creature),
      .setStatus .tapped (those (.type .creature)),
      .unattach (allOf (.and [ .hasSubtype (artifactType "Equipment"),
                               .attachedTo (those (.type .creature)) ])) ]
theorem okFulgentDistraction : Instruction.check [] fulgentDistraction = [] := by decide

def continueSpell : Instruction :=
  .sequentially
    [ choose (.described (.target (upTo 4)) (.and [creature, .inZone (graveyardOf .you)])),
      move them battlefield ]
theorem okContinueSpell : Instruction.check [] continueSpell = [] := by decide

def kindredDominance : Instruction :=
  .sequentially
    [ choose (a (quality (.subtype .creature))),
      destroy (allOf (.and [creature, .not (ofChosen (.subtype .creature))])) ]
theorem okKindredDominance : Instruction.check [] kindredDominance = [] := by decide

def phantomBlade : Ability :=
  when (.enters thisEquipment none)
    (.sequentially
      [ .attachTo it
          (.described (.target (upTo 1)) (.and [creature, .hasPossessor .controller .you])),
        destroy (.described (.target (upTo 1)) (.and [creature, .other])) ])
theorem okPhantomBlade : Ability.check [] phantomBlade = [] := by decide

/-- Braids's Frightful Return -/
def braidsFrightfulReturn : Instruction :=
  .may .you (sacrifice .you (a creature)) (some (discard (each .opponent) (a (.inZone hand)))) none
theorem okBraidsFrightfulReturn : Instruction.check [] braidsFrightfulReturn = [] := by decide
/-- Daretti, Ingenious Iconoclast -/
def darettisMinusOne : Instruction :=
  .may .you (sacrifice .you (a artifact)) (some (destroy (target (.or [artifact, creature])))) none
theorem okDarettisMinusOne : Instruction.check [] darettisMinusOne = [] := by decide

def cheeringFanatic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cheering Fanatic", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ whenever (attacks thisCreature)
            (.sequentially
              [ choose (a (quality .cardName)),
                .continuously
                  (.costs (allOf (.and [spell, ofChosen .cardName])) (.less (.lit 1) none))
                  (some .thisTurn) ]) ],
      power := stat 2, toughness := stat 2 } }

def rainOfThorns : Instruction :=
  chooseModes (atLeast 1)
    [destroy (target artifact), destroy (target enchantment), destroy (target land)]
theorem okRainOfThorns : Instruction.check [] rainOfThorns = [] := by decide

def rankleMasterOfPranks : Instruction :=
  chooseModes anyNumber
    [ discard (each .anyPlayer) (a (.inZone hand)),
      .sequentially [losesLife (each .anyPlayer) (.lit 1), .draw (those .player) (.lit 1)],
      sacrifice (each .anyPlayer) (aTheirChoice creature) ]
theorem okRankleMasterOfPranks : Instruction.check [] rankleMasterOfPranks = [] := by decide

def myrkulsEdict : Instruction :=
  .sequentially [choose (a .opponent), sacrifice (that .player) (aTheirChoice creature)]
theorem okMyrkulsEdict : Instruction.check [] myrkulsEdict = [] := by decide
def moltingHarpy : Instruction := unless_ .you (sacrifice .you thisCreature) (.mana [generic 2])
theorem okMoltingHarpy : Instruction.check [] moltingHarpy = [] := by decide
def carnophage : Instruction := unless_ .you (.setStatus .tapped thisCreature) (payLife .you 1)
theorem okCarnophage : Instruction.check [] carnophage = [] := by decide
def solitaryConfinement : Instruction :=
  .may .you (discard .you (a (.inZone hand))) none (some (sacrifice .you thisEnchantment))
theorem okSolitaryConfinement : Instruction.check [] solitaryConfinement = [] := by decide

def yasminKhan : Ability :=
  activated .tapSymbol
    (.sequentially
      [ exile (topSlice (.lit 1)),
        .continuously
          (mayPlayDeed (.action "Play") .you it none (.play none none none false .itsOwnCost))
          (some untilYourNextEndStep) ])
theorem okYasminKhan : Ability.check [] yasminKhan = [] := by decide

/-- Brazen Cannonade -/
def brazenCannonadePermission : Instruction :=
  .sequentially
    [ exile (topSlice (.lit 1)),
      .continuously
        (mayPlayDeed (.action "Play") .you it none (.play none none none false .itsOwnCost))
        (some (.until_ (.endOf .combat (some .you)))) ]
theorem okBrazenCannonadePermission : Instruction.check [] brazenCannonadePermission = [] := by
  decide

def thousandMoonsCrackshot : Ability :=
  whenever (attacks thisCreature)
    (mayWhen .you (.pay .you (.mana [generic 2, pip .white]) .once)
      (.setStatus .tapped (target creature)))
theorem okThousandMoonsCrackshot : Ability.check [] thousandMoonsCrackshot = [] := by decide

def zimoneQuandrixProdigy : Ability :=
  activated (.compound [.mana [generic 1], .tapSymbol])
    (may .you (putOntoBattlefieldTapped (a (.and [land, .inZone (handOf .you)]))))
theorem okZimoneQuandrixProdigy : Ability.check [] zimoneQuandrixProdigy = [] := by decide

def preeminentCaptain : Ability :=
  whenever (attacks thisCreature)
    (may .you (putOntoBattlefieldTappedAttacking
      (a (.and [.hasSubtype (creatureType "Soldier"), creature, .inZone (handOf .you)]))))
theorem okPreeminentCaptain : Ability.check [] preeminentCaptain = [] := by decide

def skaabRuinator : Ability :=
  .static (mayPlayDeed (.action "Cast") .you .this none
    (.play (some (graveyardOf .you)) none none false .itsOwnCost))
theorem okSkaabRuinator : Ability.check [] skaabRuinator = [] := by decide

/-- Raffine's Guidance -/
def raffinesGuidance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Raffine's Guidance", cost := some [pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (getsPt (.attachHost .enchanted (.type .creature)) (.up (.lit 1)) (.up (.lit 1))),
          .static (mayPlayDeed (.action "Cast") .you .this none
            (.play (some (graveyardOf .you)) none none false
              (.payingInstead (.mana [generic 2, pip .white])))) ] } }

/-- Scourge of Nel Toth -/
def scourgeOfNelToth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scourge of Nel Toth", cost := some [generic 5, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Zombie", creatureType "Dragon"],
      text :=
        [ keyword "Flying",
          .static (mayPlayDeed (.action "Cast") .you .this none
            (.play (some (graveyardOf .you)) none none false
              (.payingInstead (.compound
                [ .mana [pip .black, pip .black],
                  .perform (sacrifice .you (counted (exactly 2) creature)) ])))) ],
      power := stat 6, toughness := stat 6 } }

def escapeToTheWilds : Instruction :=
  .sequentially
    [ exile (topSlice (.lit 5)),
      .continuously
        (mayPlayDeed (.action "Play") .you (theVerbed (.action "Exile") .card .thisWay .many) none
          (.play none none none false .itsOwnCost))
        (some (.until_ (.endOf .turn (some .you)))) ]
theorem okEscapeToTheWilds : Instruction.check [] escapeToTheWilds = [] := by decide

/-- Muse Vessel -/
def museVesselPlay : Ability :=
  activated (.mana [generic 1])
    (.sequentially
      [ choose (a exiledWithThisArtifact),
        .continuously
          (mayPlayDeed (.action "Play") .you (that .card) none
            (.play none none none false .itsOwnCost))
          (some .thisTurn) ])
theorem okMuseVesselPlay : Ability.check [] museVesselPlay = [] := by decide

def demonicConsultationChoice : Instruction := choose (a (quality .cardName))
theorem okDemonicConsultationChoice : Instruction.check [] demonicConsultationChoice = [] := by
  decide
def voidChoice : Instruction := choose (a (quality .number))
theorem okVoidChoice : Instruction.check [] voidChoice = [] := by decide
def ashnodsBattleGear : Ability := .static (mayDeclineUntap thisArtifact (some .you))
theorem okAshnodsBattleGear : Ability.check [] ashnodsBattleGear = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngesterPump : Ability :=
  .static (.andAlso none
    [ .modify thisCreature .power (.up (.letter .x)),
      .modify thisCreature .toughness (.up (.letter .y)),
      .definesLetter .x
        (.statOf (.stat .power) (the (.and [creature, .exiledWith thisCreature]))),
      .definesLetter .y (.statOf (.stat .toughness) (that .card)) ])
theorem okPhyrexianIngesterPump : Ability.check [] phyrexianIngesterPump = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngester : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Ingester", cost := some [generic 6, pip .blue], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Beast"],
      text :=
        [ abilityWord "imprint"
            (when (.enters thisCreature none)
              (may .you (exile (target (.and [creature, nontoken]))))),
          phyrexianIngesterPump ],
      power := stat 3, toughness := stat 3 } }

def misterFantastic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mister Fantastic, Reed Richards", cost := some [generic 3, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Scientist", creatureType "Hero"],
      text :=
        [ keyword "Reach",
          whenever
            (.enters (counted (atLeast 1) (.and [.isToken, .hasPossessor .controller .you])) none)
            (may .you (.draw .you (.lit 1))) ],
      power := stat 2, toughness := stat 4 } }

def murmursFromBeyond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Murmurs from Beyond", cost := some [generic 2, pip .blue], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .spell none (.sequentially
            [ revealCards (topSlice (.lit 3)),
              chooses (a .opponent) (someOf (exactly 1) them),
              move (that .card) graveyard,
              move (theRest .object) hand ]) ] } }

/-- Helica Glider -/
def helicaGlider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Helica Glider", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Nightmare", creatureType "Squirrel"],
      text :=
        [ .static (.entersRider thisCreature
            (.withCounters (.lit 1) (.chosen [flyingCounter, .keyword "FirstStrike"]) .fresh)) ],
      power := stat 2, toughness := stat 2 } }

/-- Eager Construct -/
def eagerConstruct : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eager Construct", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text := [when (.enters thisCreature none) (may (each .anyPlayer) (scry they (.lit 1)))],
      power := stat 2, toughness := stat 2 } }

/-- Rune Snag -/
def runeSnag : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rune Snag", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ .spell none (unless_ (controllerOf (target spell)) (.counterSpell it)
            (.compound
              [ .mana [generic 2],
                scaledMana .generic
                  (times (.lit 2)
                    (countOf (.and [.named (.printed "Rune Snag"), .inZone graveyard]))) ])) ] } }

/-- Tahngarth, First Mate -/
def tahngarthChoosesDefender : Instruction :=
  .choose none none
    (a (.and [ .or [.hasType .planeswalker, .anyPlayer],
               .inCombat .attackedBy (some (that .player)) ]))
    .openly none
theorem okTahngarthChoosesDefender :
    Instruction.check (GameEvent.intro [] tahngarthHeader) tahngarthChoosesDefender = [] := by
  decide

def reefShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reef Shaman", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Shaman"],
      text :=
        [ activated .tapSymbol
            (.continuously
              (.becomes (target land) .sets
                (.chosenQuality (.ofYourChoice (.subtype .land) (some .basicTypesOnly))))
              (some untilEndOfTurn)) ],
      power := stat 0, toughness := stat 2 } }

def grixisIllusionist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grixis Illusionist", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated .tapSymbol
            (.continuously
              (.becomes (target (.and [land, .hasPossessor .controller .you])) .sets
                (.chosenQuality (.ofYourChoice (.subtype .land) (some .basicTypesOnly))))
              (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

def distantMelody : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Distant Melody", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ choose (a (quality (.subtype .creature))),
              .draw .you
                (forEach 1
                  (.and [permanent, .hasPossessor .controller .you, ofChosen (.subtype .creature)])) ]) ] } }

def cripplingFear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crippling Fear", cost := some [generic 2, pip .black, pip .black],
      types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ choose (a (quality (.subtype .creature))),
              gets (allOf (.and [creature, .not (ofChosen (.subtype .creature))]))
                (.down (.lit 3)) (.down (.lit 3)) (some untilEndOfTurn) ]) ] } }

/-- Koh, the Face Stealer -/
def kohChooser : Ability :=
  activated (payLife .you 1) (choose (a (.and [creature, .exiledWith .this])))
theorem okKohChooser : Ability.check [] kohChooser = [] := by decide
/-- Forgotten Lore -/
def forgottenLoreChoice : Instruction :=
  chooses (target .opponent) (a (.inZone (graveyardOf .you)))
theorem okForgottenLoreChoice : Instruction.check [] forgottenLoreChoice = [] := by decide

/-- Psychic Paper minus its three-way coordination -/
def psychicPaperChoiceAndReads : List Ability :=
  [ .static (.andAlso none
      [ attachChoosing thisEquipment .cardName,
        attachChoosing thisEquipment (.subtype .creature) ]),
    .static (.andAlso none
      [ .becomes (.attachHost .equipped (.type .creature)) .sets
          (.chosenQuality (ofTheLastChosen .cardName)),
        .becomes (.attachHost .equipped (.type .creature)) .sets
          (.chosenQuality (ofTheLastChosen (.subtype .creature))) ]) ]
theorem okPsychicPaperChoiceAndReads : Ability.checkText [] psychicPaperChoiceAndReads = [] := by
  decide

def xenograft : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Xenograft", cost := some [generic 4, pip .blue], types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment (.subtype .creature)),
          .static (.becomes (allOf (.and [creature, .hasPossessor .controller .you])) .adds
            (.chosenQuality (ofChosen (.subtype .creature)))) ] } }

/-- Convincing Mirage -/
def convincingMirage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Convincing Mirage", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          .static (entersChoosingFrom thisAura (.subtype .land) .basicTypesOnly),
          .static (.becomes (.attachHost .enchanted (.type .land)) .sets
            (.chosenQuality (ofChosen (.subtype .land)))) ] } }

/-- Realmwright -/
def realmwright : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Realmwright", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Vedalken", creatureType "Wizard"],
      text :=
        [ .static (entersChoosingFrom thisCreature (.subtype .land) .basicTypesOnly),
          .static (.becomes (allOf (.and [land, .hasPossessor .controller .you])) .adds
            (.chosenQuality (ofChosen (.subtype .land)))) ],
      power := stat 1, toughness := stat 1 } }

def adaptiveAutomaton : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adaptive Automaton", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text :=
        [ .static (entersChoosing thisCreature (.subtype .creature)),
          .static (.becomes thisCreature .adds (.chosenQuality (ofChosen (.subtype .creature)))),
          .static (getsPt
            (allOf (.and [ creature, .hasPossessor .controller .you, .otherThan thisCreature,
                           ofChosen (.subtype .creature) ]))
            (.up (.lit 1)) (.up (.lit 1))) ],
      power := stat 2, toughness := stat 2 } }

def mistformDreamer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistform Dreamer", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Illusion"],
      text :=
        [ keyword "Flying",
          activated (.mana [generic 1])
            (.continuously
              (.becomes thisCreature .sets
                (.chosenQuality (.ofYourChoice (.subtype .creature) none)))
              (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 1 } }

def arcaneAdaptation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Adaptation", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment (.subtype .creature)),
          .static (.alsoOffBattlefield
            (.becomes (allOf (.and [creature, .hasPossessor .controller .you])) .adds
              (.chosenQuality (ofChosen (.subtype .creature))))) ] } }

def conspiracy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conspiracy", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment (.subtype .creature)),
          .static (.alsoOffBattlefield
            (.becomes (allOf (.and [creature, .hasPossessor .controller .you])) .sets
              (.chosenQuality (ofChosen (.subtype .creature))))) ] } }

def declarationOfNaught : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Declaration of Naught", cost := some [pip .blue, pip .blue], types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment .cardName),
          activated (.mana [pip .blue]) (.counterSpell (target (.and [spell, .named .chosen]))) ] } }

def imagecrafter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Imagecrafter", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated .tapSymbol
            (.sequentially
              [ choose (a (qualityFrom (.subtype .creature) (.typeOtherThan (creatureType "Wall")))),
                .continuously
                  (.becomes (target creature) .sets (.chosenQuality (ofChosen (.subtype .creature))))
                  (some untilEndOfTurn) ]) ],
      power := stat 1, toughness := stat 1 } }

def unnaturalSelection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unnatural Selection", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ activated (.mana [generic 1])
            (.sequentially
              [ choose (a (qualityFrom (.subtype .creature) (.typeOtherThan (creatureType "Wall")))),
                .continuously
                  (.becomes (target creature) .sets (.chosenQuality (ofChosen (.subtype .creature))))
                  (some untilEndOfTurn) ]) ] } }

def standardize : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Standardize", cost := some [pip .blue, pip .blue], types := [.instant],
      text :=
        [ .spell none (.sequentially
            [ choose (a (qualityFrom (.subtype .creature) (.typeOtherThan (creatureType "Wall")))),
              .continuously
                (.becomes (each creature) .sets (.chosenQuality (ofChosen (.subtype .creature))))
                (some untilEndOfTurn) ]) ] } }

def silverquillSilencer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Silverquill Silencer", cost := some [pip .white, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ .static (entersChoosingFrom thisCreature .cardName (.nameOfCard (.not land))),
          whenever (.casts anOpponent (a (.and [spell, .named .chosen])) none)
            (.sequentially [losesLife they (.lit 3), .draw .you (.lit 1)]) ],
      power := stat 3, toughness := stat 2 } }

def meddlingMage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meddling Mage", cost := some [pip .white, pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ .static (entersChoosingFrom thisCreature .cardName (.nameOfCard (.not land))),
          .static (objectCant (.action "Cast") (allOf (.and [spell, .named .chosen]))) ],
      power := stat 2, toughness := stat 2 } }

/-- Nyxathid -/
def nyxathid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nyxathid", cost := some [generic 1, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ .static (entersChoosingPlayer thisCreature (some (.players .opponent))),
          .static (getsPt thisCreature
            (.down (countOf (.inZone (handOf (the chosenPlayer)))))
            (.down (countOf (.inZone (handOf (the chosenPlayer)))))) ],
      power := stat 7, toughness := stat 7 } }

/-- Expel the Interlopers -/
def expelTheInterlopers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Expel the Interlopers", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ choose (a (qualityFrom .number (.number (fromTo 0 10)))),
              destroy (allOf (.and [creature, .compare [.stat .power] .atLeast chosenNumber])) ]) ] } }

def nevermore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nevermore", cost := some [generic 1, pip .white, pip .white], types := [.enchantment],
      text :=
        [ .static (entersChoosingFrom thisEnchantment .cardName (.nameOfCard (.not land))),
          .static (objectCant (.action "Cast") (allOf (.and [spell, .named .chosen]))) ] } }

def necromentiaChoice : Instruction :=
  choose (a (qualityFrom .cardName (.nameOfCard (.not (.and [.hasSupertype .basic, land])))))
theorem okNecromentiaChoice : Instruction.check [] necromentiaChoice = [] := by decide

def conjurersBan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conjurer's Ban", cost := some [pip .white, pip .black], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ choose (a (quality .cardName)),
              .continuously
                (.andAlso none
                  [ objectCant (.action "Cast") (allOf (.and [spell, .named .chosen])),
                    objectCant (.action "Play") (allOf (.and [land, .named .chosen])) ])
                (some untilYourNextTurn) ]),
          .spell none (.draw .you (.lit 1)) ] } }

def foundingOfOmashu : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Founding of Omashu", cost := some [generic 2, pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Saga"],
      text :=
        [ when (.chapterMark [1]) (create (.lit 2) (creatureToken 1 1 [.white] [creatureType "Ally"])),
          when (.chapterMark [2])
            (.may .you (discard .you (a (.inZone hand))) (some (.draw .you (.lit 1))) none),
          when (.chapterMark [3])
            (gets (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn)) ] } }

/-- "as though it had flash" -/
def asThoughFlash : Option AsThough := some (.of (.hasKeyword (.the "Flash")))

def vedalkenOrrery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vedalken Orrery", cost := some [generic 4], types := [.artifact],
      text :=
        [ .static (mayPlayDeed (.action "Cast") .you (allOf spell) asThoughFlash
            (.play none none none false .itsOwnCost)) ] } }

def shimmerMyr : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmer Myr", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Myr"],
      text :=
        [ keyword "Flash",
          .static (mayPlayDeed (.action "Cast") .you (allOf (.and [spell, artifact])) asThoughFlash
            (.play none none none false .itsOwnCost)) ],
      power := stat 2, toughness := stat 2 } }

def quickSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Quick Sliver", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Sliver"],
      text :=
        [ keyword "Flash",
          .static (mayPlayDeed (.action "Cast") (a .anyPlayer)
            (allOf (.and [spell, .hasSubtype (creatureType "Sliver")])) asThoughFlash
            (.play none none none false .itsOwnCost)) ],
      power := stat 1, toughness := stat 1 } }

def vernalEquinox : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vernal Equinox", cost := some [generic 3, pip .green], types := [.enchantment],
      text :=
        [ .static (mayPlayDeed (.action "Cast") (a .anyPlayer)
            (allOf (.and [spell, .or [creature, enchantment]])) asThoughFlash
            (.play none none none false .itsOwnCost)) ] } }

def borneUponAWind : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Borne Upon a Wind", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ .spell none (.continuously
            (mayPlayDeed (.action "Cast") .you (allOf spell) asThoughFlash
              (.play none none none false .itsOwnCost))
            (some .thisTurn)),
          .spell none (.draw .you (.lit 1)) ] } }

def gisaAndGeralf : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gisa and Geralf", cost := some [generic 2, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ when (.enters thisCreature none) (mills .you (.lit 4) .you),
          .static (mayPlayDeed (.action "Cast") .you
            (a (.and [spell, creature, .hasSubtype (creatureType "Zombie")])) none
            (.play (some (graveyardOf .you)) (some .onceEachYourTurn) none false .itsOwnCost)) ],
      power := stat 4, toughness := stat 4 } }

/-- "play lands from the top of your library" -/
def playLandsFromTop : StaticSpec :=
  mayPlayDeed (.action "Play") .you (allOf land) none (.play (some onTop) none none false .itsOwnCost)
/-- "cast <p> spells from the top of your library" -/
def castFromTop (p : Predicate) : StaticSpec :=
  mayPlayDeed (.action "Cast") .you (allOf p) none (.play (some onTop) none none false .itsOwnCost)

/-- Future Sight; Magus of the Future -/
def playLandsAndCastSpellsFromTop : StaticSpec :=
  .andAlso none [playLandsFromTop, castFromTop spell]
theorem okPlayLandsAndCastSpellsFromTop :
    StaticSpec.check [] playLandsAndCastSpellsFromTop = [] := by decide
/-- Assemble the Players -/
def castSmallCreatureFromTopOnceEachTurn : StaticSpec :=
  mayPlayDeed (.action "Cast") .you
    (a (.and [spell, creature, .compare [.stat .power] .atMost (.lit 2)])) none
    (.play (some onTop) (some .onceEachTurn) none false .itsOwnCost)
theorem okCastSmallCreatureFromTopOnceEachTurn :
    StaticSpec.check [] castSmallCreatureFromTopOnceEachTurn = [] := by decide

def courserOfKruphix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Courser of Kruphix", cost := some [generic 1, pip .green, pip .green],
      types := [.enchantment, .creature], subtypes := [creatureType "Centaur"],
      text :=
        [ .static (.visibility .reveal .you .topOfLibrary),
          .static playLandsFromTop,
          whenever (.enters (a (.and [land, .hasPossessor .controller .you])) none)
            (gainsLife .you (.lit 1)) ],
      power := stat 2, toughness := stat 4 } }

def korlessaScaleSinger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Korlessa, Scale Singer", cost := some [pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Dragon", creatureType "Bard"],
      text :=
        [ .static (.visibility .lookAt .you .topOfLibrary),
          .static (castFromTop (.and [spell, .hasSubtype (creatureType "Dragon")])) ],
      power := stat 1, toughness := stat 4 } }

def precognitionField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Precognition Field", cost := some [generic 3, pip .blue], types := [.enchantment],
      text :=
        [ .static (.visibility .lookAt .you .topOfLibrary),
          .static (castFromTop (.and [spell, instantOrSorcery])),
          activated (.mana [generic 3]) (exile (.librarySlice .top (.lit 1) .you)) ] } }

def mysticForge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mystic Forge", cost := some [generic 4], types := [.artifact],
      text :=
        [ .static (.visibility .lookAt .you .topOfLibrary),
          .static (.andAlso none [castFromTop (.and [spell, artifact]), castFromTop (.and [spell, colorless])]),
          activated (.compound [.tapSymbol, payLife .you 1]) (exile (.librarySlice .top (.lit 1) .you)) ] } }

def exploration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exploration", cost := some [pip .green], types := [.enchantment],
      text := [.static (mayPlayAdditionalLands .you (exactly 1))] } }

def oracleOfMulDaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oracle of Mul Daya", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Shaman"],
      text :=
        [ .static (mayPlayAdditionalLands .you (exactly 1)),
          .static (.visibility .reveal .you .topOfLibrary),
          .static playLandsFromTop ],
      power := stat 2, toughness := stat 2 } }

def azusaLostButSeeking : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Azusa, Lost but Seeking", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Monk"],
      text := [.static (mayPlayAdditionalLands .you (exactly 2))],
      power := stat 1, toughness := stat 2 } }

def summerBloom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Summer Bloom", cost := some [generic 1, pip .green], types := [.sorcery],
      text := [.spell none (.continuously (mayPlayAdditionalLands .you (upTo 3)) (some .thisTurn))] } }

def explore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Explore", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ .continuously (mayPlayAdditionalLands .you (exactly 1)) (some .thisTurn),
              .draw .you (.lit 1) ]) ] } }

def urbanEvolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urban Evolution", cost := some [generic 3, pip .green, pip .blue],
      types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ .draw .you (.lit 3),
              .continuously (mayPlayAdditionalLands .you (exactly 1)) (some .thisTurn) ]) ] } }

def dryadOfTheIlysianGrove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dryad of the Ilysian Grove", cost := some [generic 2, pip .green],
      types := [.enchantment, .creature], subtypes := [creatureType "Nymph", creatureType "Dryad"],
      text :=
        [ .static (mayPlayAdditionalLands .you (exactly 1)),
          .static (.becomes (allOf (.and [land, .hasPossessor .controller .you])) .adds
            (.everyTypeOf .basicLand)) ],
      power := stat 2, toughness := stat 4 } }

/-- Ashes of the Fallen -/
def ashesOfTheFallen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ashes of the Fallen", cost := some [generic 2], types := [.artifact],
      text :=
        [ .static (entersChoosing thisArtifact (.subtype .creature)),
          .static (.becomes (each (.and [creature, .inZone (graveyardOf .you)])) .adds
            (.chosenQuality (ofChosen (.subtype .creature)))) ] } }

def hellkiteCharger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hellkite Charger", cost := some [generic 4, pip .red, pip .red],
      types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword "Flying", keyword "Haste",
          whenever (attacks thisCreature)
            (.may .you (.pay .you (.mana [generic 5, pip .red, pip .red]) .once)
              (some (.sequentially
                [ .setStatus .untapped (allOf (.and [creature, attacking])),
                  additionalPart .combat none (.lit 1) ]))
              none) ],
      power := stat 5, toughness := stat 5 } }

/-- Foriysian Brigade -/
def foriysianBrigade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foriysian Brigade", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [.static (mayBlockAdditional thisCreature (exactly 1))],
      power := stat 2, toughness := stat 4 } }

/-- Two-Headed Giant of Foriys -/
def twoHeadedGiantOfForiys : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Two-Headed Giant of Foriys", cost := some [generic 4, pip .red],
      types := [.creature], subtypes := [creatureType "Giant"],
      text := [keyword "Trample", .static (mayBlockAdditional thisCreature (exactly 1))],
      power := stat 4, toughness := stat 4 } }

def highGround : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "High Ground", cost := some [pip .white], types := [.enchantment],
      text := [.static (mayBlockAdditional (each creatureYouControl) (exactly 1))] } }

/-- Watcher in the Web -/
def watcherInTheWeb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Watcher in the Web", cost := some [generic 4, pip .green], types := [.creature],
      subtypes := [creatureType "Spider"],
      text := [keyword "Reach", .static (mayBlockAdditional thisCreature (exactly 7))],
      power := stat 2, toughness := stat 5 } }

/-- Forgotten Lore -/
def forgottenLoreRepeat : Instruction :=
  .sequentially
    [ .choose none (some (target .opponent)) (a (.inZone (graveyardOf .you))) .openly none,
      .may .you (.pay .you (.mana [pip .green]) .once) (some (.repeat_ .againExcludingChosen)) none ]
theorem okForgottenLoreRepeat : Instruction.check [] forgottenLoreRepeat = [] := by decide

/-- Leyline of the Meek -/
def leylineOfTheMeek : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leyline of the Meek", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ .mayBeginOnBattlefield,
          .static (getsPt (allOf (.and [creature, .isToken])) (.up (.lit 1)) (.up (.lit 1))) ] } }

/-- Leyline of Vitality -/
def leylineOfVitality : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leyline of Vitality", cost := some [generic 2, pip .green, pip .green],
      types := [.enchantment],
      text :=
        [ .mayBeginOnBattlefield,
          .static (getsPt (allOf creatureYouControl) (.up (.lit 0)) (.up (.lit 1))),
          whenever (.enters (a creatureYouControl) none) (may .you (gainsLife .you (.lit 1))) ] } }

def phyrexianRevoker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Revoker", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Phyrexian", creatureType "Horror"],
      text :=
        [ .static (entersChoosingFrom thisCreature .cardName (.nameOfCard (.not land))),
          .static (objectCant (.action "Activate")
            (allOf (.and [ .abilityHead .anyActivated,
                           .abilityOf (allOf (.and [source, .named .chosen])) ]))) ],
      power := stat 2, toughness := stat 1 } }

def voidstoneGargoyle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voidstone Gargoyle", cost := some [generic 3, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Gargoyle"],
      text :=
        [ keyword "Flying",
          .static (entersChoosingFrom thisCreature .cardName (.nameOfCard (.not land))),
          .static (objectCant (.action "Cast") (allOf (.and [spell, .named .chosen]))),
          .static (objectCant (.action "Activate")
            (allOf (.and [ .abilityHead .anyActivated,
                           .abilityOf (allOf (.and [source, .named .chosen])) ]))) ],
      power := stat 3, toughness := stat 3 } }

def pithingNeedle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pithing Needle", cost := some [generic 1], types := [.artifact],
      text :=
        [ .static (entersChoosing thisArtifact .cardName),
          .static (objectCant (.action "Activate")
            (allOf (.and [ .abilityHead .anyActivated,
                           .abilityOf (allOf (.and [source, .named .chosen])),
                           .not .isManaAbility ]))) ] } }

/-- Rhystic Study -/
def rhysticStudy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rhystic Study", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ whenever (.casts anOpponent (a spell) none)
            (unless_ (that .player) (may .you (.draw .you (.lit 1))) (.mana [generic 1])) ] } }

def gandalfWhiteRider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gandalf, White Rider", cost := some [generic 3, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Avatar", creatureType "Wizard"],
      text :=
        [ keyword "Vigilance",
          whenever (.casts .you (a spell) none)
            (.sequentially
              [ gets (each creatureYouControl) (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn),
                scry .you (.lit 1) ]),
          when (.dies thisCreature) (may .you (move it (nthFromTop (.nth 5)))) ],
      power := stat 3, toughness := stat 3 } }

def helmOfPossession : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Helm of Possession", cost := some [generic 4], types := [.artifact],
      text :=
        [ .static (mayDeclineUntap thisArtifact (some .you)),
          activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice .you (a creature))])
            (gainControl .you (target creature)
              (some (.forAsLongAs
                (.and [ .matches thisArtifact (.hasPossessor .controller .you),
                        .matches thisArtifact tapped ])))) ] } }

def override : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Override", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ .spell none (unless_ (controllerOf (target spell)) (.counterSpell it)
            (scaledMana .generic (forEach 1 (.and [artifact, .hasPossessor .controller .you])))) ] } }

def rakshasasDisdain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rakshasa's Disdain", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ .spell none (unless_ (controllerOf (target spell)) (.counterSpell it)
            (scaledMana .generic (forEach 1 (.inZone (graveyardOf .you))))) ] } }

def megatherium : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Megatherium", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ keyword "Trample",
          when (.enters thisCreature none)
            (unless_ .you (sacrifice .you thisCreature)
              (scaledMana .generic (forEach 1 (.inZone (handOf .you))))) ],
      power := stat 4, toughness := stat 4 } }

def killingWave : Instruction :=
  .forEachOf (each creature)
    (unless_ (controllerOf it) (sacrifice they it) (.perform (losesLife they (.letter .x))))
theorem okKillingWave : Instruction.check [] killingWave = [] := by decide
def fadeAway : Instruction :=
  .forEachOf (each creature)
    (unless_ (controllerOf it) (sacrifice they (a permanent)) (.mana [generic 1]))
theorem okFadeAway : Instruction.check [] fadeAway = [] := by decide

def tidalFlats : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tidal Flats", cost := some [pip .blue], types := [.enchantment],
      text :=
        [ activated (.mana [pip .blue, pip .blue])
            (.forEachOf (each (.and [creature, attacking, .not (.hasKeyword (.the "Flying"))]))
              (.may (controllerOf it) (.pay they (.mana [generic 1]) .once) none
                (some (gains
                  (allOf (.and [ creature, .hasPossessor .controller .you,
                                 .inCombat .blockerOf (some (that (.type .creature))) ]))
                  (keyword "FirstStrike") (some untilEndOfTurn))))) ] } }

def primalSurge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Primal Surge", cost := some [generic 8, pip .green, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ exile (topSlice (.lit 1)),
              .if_ (itsA permanentCard)
                (.may .you (putOntoBattlefield it) (some (.repeat_ .again)) none) none ]) ] } }

def cultivatorColossus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cultivator Colossus", cost := some [generic 4, pip .green, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Plant", creatureType "Beast"],
      text :=
        [ keyword "Trample",
          .static (.definesPt thisCreature .bothEach
            (countOf (.and [land, .hasPossessor .controller .you]))),
          when (.enters thisCreature none)
            (.may .you (putOntoBattlefieldTapped (a (.and [land, .inZone (handOf .you)])))
              (some (.sequentially [.draw .you (.lit 1), .repeat_ .again])) none) ] } }

def zimoneAndDina : Ability :=
  activated (.compound [.tapSymbol, .perform (sacrifice .you (a (otherCreature thisCreature)))])
    (.sequentially
      [ .draw .you (.lit 1),
        may .you (putOntoBattlefieldTapped (a (.and [land, .inZone (handOf .you)]))),
        .if_ (.compareAmt (countOf (.and [land, .hasPossessor .controller .you])) .atLeast (.lit 8))
          (.repeat_ (.moreTimes (.lit 1))) none ])
theorem okZimoneAndDina : Ability.check [] zimoneAndDina = [] := by decide

def cryptLurker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crypt Lurker", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Horror"],
      text :=
        [ when (.enters thisCreature none)
            (.may .you
              (chooseModes (exactly 1)
                [ sacrifice .you (a creature),
                  discard .you (a (.and [creature, .inZone hand])) ])
              (some (.draw .you (.lit 1))) none) ],
      power := stat 3, toughness := stat 4 } }

/-- Memoricide -/
def memoricideSearch : Instruction :=
  .sequentially
    [ choose (a (qualityFrom .cardName (.nameOfCard (.not land)))),
      searchZonesOf (target .anyPlayer) (exactly 1) (.named .chosen),
      .shuffle (that .player) ]
theorem okMemoricideSearch : Instruction.check [] memoricideSearch = [] := by decide
/-- Lost Hours -/
def lostHoursPlacement : Instruction :=
  .sequentially
    [ revealsTheirHand (target .anyPlayer),
      choose (a (.and [.not land, .inZone (handOf they)])),
      puts (that .player) it (nthFromTop (.nth 3)) ]
theorem okLostHoursPlacement : Instruction.check [] lostHoursPlacement = [] := by decide
/-- Aether Gust -/
def aetherGustPlacement : Instruction :=
  .sequentially
    [ choose (target (.and [permanent, .colorIs .red])),
      puts (ownerOf it) it (choiceOfTopOrBottom they) ]
theorem okAetherGustPlacement : Instruction.check [] aetherGustPlacement = [] := by decide
/-- Fastbond -/
def fastbondLands : Ability := .static (mayPlayAdditionalLands .you anyNumber)
theorem okFastbondLands : Ability.check [] fastbondLands = [] := by decide
/-- Panglacial Wurm -/
def panglacialWurmCast : Ability :=
  .static (mayPlayDeed (.action "Cast") .you .this none
    (.play (some yourLibrary) none (some .whileSearchingLibrary) false .itsOwnCost))
theorem okPanglacialWurmCast : Ability.check [] panglacialWurmCast = [] := by decide

/-- Shah of Naar Isle -/
def shahOfNaarIsle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shah of Naar Isle", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Efreet"],
      text :=
        [ keyword "Trample",
          keywordCosting "Echo" (.mana [generic 0]),
          when (.paysCost none .paid thisCreature "Echo")
            (may (each .opponent) (.draw they (.upTo (.lit 3)))) ],
      power := stat 6, toughness := stat 6 } }

/-- Memory Plunder -/
def memoryPlunder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Memory Plunder",
      cost := some [hybridPip .blue .black, hybridPip .blue .black, hybridPip .blue .black,
                    hybridPip .blue .black],
      types := [.instant],
      text :=
        [ .spell none (.continuously
            (mayPlayDeed (.action "Cast") .you (target (.and [instantOrSorcery, .isCard])) none
              (.play (some (graveyardOf anOpponent)) none none false .withoutPaying))
            none) ] } }

/-- Omniscience -/
def omniscience : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Omniscience", cost := some [generic 7, pip .blue, pip .blue, pip .blue],
      types := [.enchantment],
      text :=
        [ .static (mayPlayDeed (.action "Cast") .you (allOf spell) none
            (.play (some (handOf .you)) none none false .withoutPaying)) ] } }

/-- Tranquil Frillback -/
def tranquilFrillbackOffer : Instruction := may .you (.pay .you (.mana [pip .green]) (.upTo 3))
theorem okTranquilFrillbackOffer : Instruction.check [] tranquilFrillbackOffer = [] := by decide

/-- Caller of the Hunt -/
def callerOfTheHunt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caller of the Hunt", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ .static (.addedCost (.perform (choose (a (quality (.subtype .creature))))) false),
          .static (.definesPt thisCreature .bothEach
            (countOf (.and [creature, ofChosen (.subtype .creature)]))) ] } }

/-- Duneblast -/
def duneblast : Instruction :=
  .sequentially [choose (counted (upTo 1) creature), destroy (theRest .object)]
theorem okDuneblast : Instruction.check [] duneblast = [] := by decide
/-- Boreas Charger -/
def boreasChargerChoice : Instruction := choose (a opponentWithMoreLands)
theorem okBoreasChargerChoice : Instruction.check [] boreasChargerChoice = [] := by decide
/-- Boreas Charger's spell text -/
def boreasChargerSpell : Instruction :=
  .sequentially
    [ choose (a opponentWithMoreLands),
      searchLibraryFor (.exactlyOf .theDifference) (.hasSubtype (landType "Plains")),
      revealCards (those .card),
      putOntoBattlefieldTapped (someOf (exactly 1) (those .card)),
      move (theRest .object) hand ]
theorem okBoreasChargerSpell : Instruction.check [] boreasChargerSpell = [] := by decide

/-- Sandstone Oracle -/
def sandstoneOracle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sandstone Oracle", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          when (.enters thisCreature none)
            (.sequentially
              [ choose anOpponent,
                .if_ (.compareAmt (countOf (.inZone (handOf (that .player)))) .greater
                        (countOf (.inZone (handOf .you))))
                  (.draw .you .theDifference) none ]) ],
      power := stat 4, toughness := stat 4 } }

/-- Slithermuse -/
def slithermuseTrigger : Ability :=
  when (leavesBattlefield thisCreature)
    (.sequentially
      [ choose anOpponent,
        .if_ (.compareAmt (countOf (.inZone (handOf (that .player)))) .greater
                (countOf (.inZone (handOf .you))))
          (.draw .you .theDifference) none ])
theorem okSlithermuseTrigger : Ability.check [] slithermuseTrigger = [] := by decide

/-- Celestial Judgment -/
def celestialJudgment : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celestial Judgment", cost := some [generic 4, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ .forEachKindOf (.value .power) (some (allOf creature)) .number
                (choose (a (.and [creature, .compare [.stat .power] .eq chosenNumber]))),
              destroy (each (.and [creature, .notChosen])) ]) ] } }

/-- World Queller -/
def worldQuellerChoice : Instruction :=
  .may .you (choose (a (quality .cardType)))
    (some (sacrifice (each .anyPlayer) (aTheirChoice (.and [permanent, ofChosen .cardType])))) none
theorem okWorldQuellerChoice : Instruction.check [] worldQuellerChoice = [] := by decide

/-- Moonlit Meditation -/
def moonlitMeditation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Moonlit Meditation", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" (.and [.or [artifact, creature], .hasPossessor .controller .you]),
          .static (.intercepts (.tokensCreated (counted (atLeast 1) .isToken) false (some .you) none)
            [] none
            (may .you (.create .you .groupSize (.copyOf (.attachHost .enchanted .permanent) []) []))
            .repeatedly (some .oncePerTurn)) ] } }

def discardUpToTwoThenDrawThatMany : Instruction :=
  .sequentially [discard .you (counted (upTo 2) (.inZone hand)), .draw .you .groupSize]
theorem okDiscardUpToTwoThenDrawThatMany :
    Instruction.check [] discardUpToTwoThenDrawThatMany = [] := by decide
/-- Truce and Temporary Truce -/
def drawUpToTwoThenGainPerShortfall : Instruction :=
  .sequentially
    [ may (each .anyPlayer) (.draw they (.upTo (.lit 2))),
      gainsLife they (times (.lit 2) shortOfCeiling) ]
theorem okDrawUpToTwoThenGainPerShortfall :
    Instruction.check [] drawUpToTwoThenGainPerShortfall = [] := by decide

/-- Commune with the Gods -/
def communeWithTheGods : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Commune with the Gods", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ revealCards (topSlice (.lit 5)),
              may .you (move (fromAmong (exactly 1) (.or [creature, enchantment]) them) hand),
              move (theRest .object) graveyard ]) ] } }

/-- Nautiloid Ship -/
def nautiloidShipTrigger : Ability :=
  whenever (dealsCombatDamage thisVehicle (a .anyPlayer))
    (may .you (putOntoBattlefieldUnderYourControl (a (.and [creature, .exiledWith thisVehicle]))))
theorem okNautiloidShipTrigger : Ability.check [] nautiloidShipTrigger = [] := by decide

/-- Summon: Esper Valigarmanda's II/III/IV body, in part -/
def summonEsperValigarmandaCast : StaticSpec :=
  .andAlso none
    [ mayPlayDeed (.action "Cast") .you (a (.and [instantOrSorcery, .exiledWith thisSaga])) none
        (.play none none none false .itsOwnCost),
      maySpendAsThough .you none .anyType
        (some (.toCast (.and [instantOrSorcery, .exiledWith thisSaga]))) ]
theorem okSummonEsperValigarmandaCast :
    StaticSpec.check [] summonEsperValigarmandaCast = [] := by decide
/-- Rogue Class's level-3 body -/
def rogueClassLevelThree : StaticSpec :=
  .andAlso none
    [ mayPlayDeed (.action "Play") .you (allOf (.exiledWith thisClass)) none
        (.play none none none false .itsOwnCost),
      maySpendAsThough .you none .anyColor (some (.toCast (.exiledWith thisClass))) ]
theorem okRogueClassLevelThree : StaticSpec.check [] rogueClassLevelThree = [] := by decide

/-- Soul Ransom -/
def soulRansom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soul Ransom", cost := some [generic 2, pip .blue, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.gainsControl .you (.attachHost .enchanted (.type .creature))),
          activatedBy
            (.perform (.repeated (.lit 2)
              (.sequentially [choose (a (.inZone hand)), discard .you (that .card)])))
            (.sequentially [sacrificeIt (controllerOf thisAura), .draw they (.lit 2)])
            (.playerGroup .yourOpponents) ] } }

/-- Vraska's Scorn -/
def vraskasScorn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vraska's Scorn", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ losesLife (target .opponent) (.lit 4),
              may .you
                (.sequentially
                  [ searchLibraryOrGraveyard (.named (.printed "Vraska, Scheming Gorgon")),
                    revealCards it, move it hand ]),
              .if_ (happenedAt (.verbedAct (.action "Search")) .you .thisWay yourLibrary) shuffle
                none ]) ] } }

/-- Old-Growth Dryads -/
def oldGrowthDryads : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Old-Growth Dryads", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Dryad"],
      text :=
        [ when (.enters thisCreature none)
            (may (each .opponent)
              (.sequentially
                [ playerSearchesTheirLibraryFor they (.and [land, .hasSupertype .basic]),
                  putOntoBattlefieldTapped foundCard,
                  .shuffle they ])) ],
      power := stat 3, toughness := stat 3 } }

/-- Verity Circle -/
def verityCircle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Verity Circle", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ triggeredIf
            (.statusEvent (a (.and [creature, .hasPossessor .controller anOpponent])) .tapped)
            (.matches it (.not (.inCombat .declaredAttacker none)))
            (may .you (.draw .you (.lit 1))),
          activated (.mana [generic 4, pip .blue])
            (tap (target (.and [creature, .not (.hasKeyword (.the "Flying"))]))) ] } }

/-- Danitha, New Benalia's Light -/
def danithaNewBenaliasLight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Danitha, New Benalia's Light", cost := some [generic 1, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text :=
        [ keyword "Vigilance", keyword "Trample", keyword "Lifelink",
          .static (mayPlayDeed (.action "Cast") .you
            (a (.and [ spell, .or [ .hasSubtype (enchantmentType "Aura"),
                                    .hasSubtype (artifactType "Equipment") ] ]))
            none (.play (some (graveyardOf .you)) (some .onceEachYourTurn) none false .itsOwnCost)) ],
      power := stat 2, toughness := stat 2 } }

/-- Muldrotha, the Gravetide -/
def muldrothaLandWindow : StaticSpec :=
  mayPlayDeed (.action "Play") .you (a land) none
    (.play (some (graveyardOf .you)) none (some .duringEachOfYourTurns) false .itsOwnCost)
theorem okMuldrothaLandWindow : StaticSpec.check [] muldrothaLandWindow = [] := by decide
/-- Nahiri's Lithoforming -/
def nahiriExtraLands : StaticSpec := mayPlayAdditionalLands .you (.exactlyOf (.letter .x))
theorem okNahiriExtraLands : StaticSpec.check [letterB .x] nahiriExtraLands = [] := by decide

/-- Haakon, Stromgald Scourge -/
def haakonStromgaldScourge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Haakon, Stromgald Scourge", cost := some [generic 1, pip .black, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Zombie", creatureType "Knight"],
      text :=
        [ .static (mayPlayDeed (.action "Cast") .you .this none
            (.play (some (graveyardOf .you)) none none true .itsOwnCost)),
          .static (.conditionally
            (mayPlayDeed (.action "Cast") .you
              (allOf (.and [spell, .hasSubtype (creatureType "Knight")])) none
              (.play (some (graveyardOf .you)) none none false .itsOwnCost))
            (.matches .this (.inZone battlefield)) .asLongAs),
          when (.dies thisCreature) (losesLife .you (.lit 2)) ],
      power := stat 3, toughness := stat 3 } }

/-- Apex of Power -/
def apexOfPowerCast : Instruction :=
  .sequentially
    [ exile (.librarySlice .top (.lit 7) .you),
      .continuously
        (mayPlayDeed (.action "Cast") .you (fromAmong anyNumber spell them) none
          (.play none none none false .itsOwnCost))
        (some .thisTurn) ]
theorem okApexOfPowerCast : Instruction.check [] apexOfPowerCast = [] := by decide
/-- Blight Herder -/
def blightHerderCast : Instruction :=
  may .you
    (move (counted (exactly 2)
            (.and [.isCard, .hasPossessor .owner (.playerGroup .yourOpponents), .inZone exileZone]))
      graveyard)
theorem okBlightHerderCast : Instruction.check [] blightHerderCast = [] := by decide

/-- True-Name Nemesis -/
def trueNameNemesis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "True-Name Nemesis", cost := some [generic 1, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Merfolk", creatureType "Rogue"],
      text :=
        [ .static (entersChoosingPlayer thisCreature none),
          keywordQuality "Protection" chosenPlayer ],
      power := stat 3, toughness := stat 1 } }

/-- Akiri, Fearless Voyager -/
def akiriUnattachOffer : Ability :=
  activated (.mana [pip .white])
    (.may .you
      (.unattach (a (.and [ .hasSubtype (artifactType "Equipment"),
                            .attachedTo (a creatureYouControl) ])))
      (some (.setStatus .tapped (that (.type .creature)))) none)
theorem okAkiriUnattachOffer : Ability.check [] akiriUnattachOffer = [] := by decide
/-- Summoning Materia -/
def summoningMateriaTopCast : Ability :=
  .static (onlyWhile (castFromTop (.and [spell, creature]))
    (.matches thisEquipment (.attachedTo (a creature))))
theorem okSummoningMateriaTopCast : Ability.check [] summoningMateriaTopCast = [] := by decide
/-- Vizier of the Menagerie -/
def vizierOfTheMenagerieSpend : StaticSpec :=
  maySpendAsThough .you none .anyType (some (.toCast (.and [creature, spell])))
theorem okVizierOfTheMenagerieSpend : StaticSpec.check [] vizierOfTheMenagerieSpend = [] := by
  decide

/-- Conspicuous Snoop -/
def conspicuousSnoop : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conspicuous Snoop", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Rogue"],
      text :=
        [ .static (.visibility .reveal .you .topOfLibrary),
          .static (mayPlayDeed (.action "Play") .you
            (allOf (.and [spell, .hasSubtype (creatureType "Goblin")])) none
            (.play (some onTop) none none false .itsOwnCost)),
          .static (onlyWhile
            (.gainsAbilitiesOf thisCreature [.anyActivated] (topSlice (.lit 1)) none)
            (.matches (topSlice (.lit 1)) (.hasSubtype (creatureType "Goblin")))) ],
      power := stat 2, toughness := stat 2 } }

/-- Ballot Broker -/
def ballotBroker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ballot Broker", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Advisor"],
      text := [.static (mayVoteAdditional .you (exactly 1))],
      power := stat 2, toughness := stat 3 } }

/-- Emissary of Grudges -/
def emissaryOfGrudgesReveal : Ability :=
  activatedOnlyOnce (.perform (.expose .reveal .you (.choice .player)))
    (.onlyIf (.chooseNewTargets (target (.or [spell, .abilityHead .anyOnStack])))
      (.and
        [ .matches it (.hasPossessor .controller (the chosenPlayer)),
          .matches it
            (.targets (youOr (a (.and [permanent, .hasPossessor .controller .you]))) .someTarget) ])
      none)
    .oncePerGame
theorem okEmissaryOfGrudgesReveal :
    Ability.check [ChoiceSort.binding .player] emissaryOfGrudgesReveal = [] := by decide

def prosperity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prosperity", cost := some [.variable, pip .blue], types := [.sorcery],
      text := [.spell none (.draw (each .anyPlayer) (.letter .x))] } }

def collectiveUnconscious : Instruction := .draw .you (forEach 1 creatureYouControl)
theorem okCollectiveUnconscious : Instruction.check [] collectiveUnconscious = [] := by decide
def killiansConfidence : Instruction :=
  .sequentially
    [gets (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn), .draw .you (.lit 1)]
theorem okKilliansConfidence : Instruction.check [] killiansConfidence = [] := by decide

def scatheZombies : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scathe Zombies", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Zombie"], power := stat 2, toughness := stat 2 } }

/-- A basic land card of the named type. -/
def basicLand (name : String) : Card :=
  .singleFaced
    { characteristics :=
      { name, supertypes := [.basic], types := [.land], subtypes := [landType name] } }
def basicLandCards : List Card :=
  ["Plains", "Island", "Swamp", "Mountain", "Forest"].map basicLand
theorem okBasicLandCards : basicLandCards.flatMap Card.check = [] := by decide

def snowCoveredForest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Snow-Covered Forest", supertypes := [.basic, .snow], types := [.land],
      subtypes := [landType "Forest"] } }

/-- Not Forgotten -/
def notForgottenPlacement : Instruction :=
  move (target (.inZone graveyard)) (choiceOfTopOrBottom .you)
theorem okNotForgottenPlacement : Instruction.check [] notForgottenPlacement = [] := by decide

def duneblastCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Duneblast", cost := some [generic 4, pip .white, pip .black, pip .green],
      types := [.sorcery], text := [.spell none duneblast] } }

def boreasChargerDifference : Amount := .theDifference
theorem okBoreasChargerDifference :
    Amount.check (Instruction.intro [] boreasChargerChoice) boreasChargerDifference = [] := by decide

/-- Hymn to Tourach -/
def hymnToTourach : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hymn to Tourach", cost := some [pip .black, pip .black], types := [.sorcery],
      text :=
        [ .spell none (discard (target .anyPlayer) (countedAtRandom (exactly 2) (.inZone hand))) ] } }

/-- Caught in the Crossfire -/
def caughtInTheCrossfire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caught in the Crossfire", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ .spell none (spree
            [ (some (.mana [generic 1]),
               .dealDamage .this (.lit 2) (each (.and [creature, outlaw]))),
              (some (.mana [generic 1]),
               .dealDamage .this (.lit 2) (each (.and [creature, .not outlaw]))) ]) ] } }

/-- Requisition Raid -/
def requisitionRaid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Requisition Raid", cost := some [pip .white], types := [.sorcery],
      text :=
        [ .spell none (spree
            [ (some (.mana [generic 1]), destroy (target artifact)),
              (some (.mana [generic 1]), destroy (target enchantment)),
              (some (.mana [generic 1]),
               .putCounters (.lit 1) (.printed plusOnePlusOne)
                 (each (.and [creature, .hasPossessor .controller (target .anyPlayer)]))) ]) ] } }

/-- Rustler Rampage -/
def rustlerRampage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rustler Rampage", cost := some [pip .white], types := [.instant],
      text :=
        [ .spell none (spree
            [ (some (.mana [generic 1]),
               .setStatus .untapped
                 (allOf (.and [creature, .hasPossessor .controller (target .anyPlayer)]))),
              (some (.mana [generic 1]),
               gains (target creature) (keyword "DoubleStrike") (some untilEndOfTurn)) ]) ] } }

/-- Consuming Tide -/
def consumingTide : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consuming Tide", cost := some [generic 2, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ chooses (each .anyPlayer)
                (a (.and [permanent, .not land, .hasPossessor .controller they])),
              returnTo (allOf (.and [permanent, .not land, .notChosen])) hand [],
              .forEachOf
                (each (.compareOver .opponent (countOf (.inZone (handOf they))) .greater
                  (countOf (.inZone (handOf .you)))))
                (.draw .you (.lit 1)) ]) ] } }

/-- Stick Together: a chosen party is four up-to-one choices, not the group the game computes
[CR#700.8d]. -/
def stickTogether : Instruction :=
  .sequentially
    [ chooses (each .anyPlayer)
        (counted (upTo 1)
          (.and [creature, .hasSubtype (creatureType "Cleric"), .hasPossessor .controller they])),
      chooses (each .anyPlayer)
        (counted (upTo 1)
          (.and [creature, .hasSubtype (creatureType "Rogue"), .hasPossessor .controller they])),
      chooses (each .anyPlayer)
        (counted (upTo 1)
          (.and [creature, .hasSubtype (creatureType "Warrior"), .hasPossessor .controller they])),
      chooses (each .anyPlayer)
        (counted (upTo 1)
          (.and [creature, .hasSubtype (creatureType "Wizard"), .hasPossessor .controller they])),
      sacrifice (each .anyPlayer) (theRest .object) ]
theorem okStickTogether : Instruction.check [] stickTogether = [] := by decide

/-- Disciple of Caelus Nin: "starting with you" fixes the order the players choose in
[CR#101.4]. -/
def discipleOfCaelusNin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disciple of Caelus Nin", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ when (.enters thisCreature none)
            (.sequentially
              [ .choose (some .you) (some (each .anyPlayer))
                  (counted (upTo 5) (.and [permanent, .hasPossessor .controller they])) .openly none,
                .setStatus .phasedOut
                  (allOf (.and [permanent, .otherThan thisCreature, .notChosen])) ]),
          .static (.deontic (allOf permanent) .forbid [.ofAbility "Phasing"] .agent none .noPatient
            none .noRider) ],
      power := stat 3, toughness := stat 4 } }

/-- Sculpted Sunburst: "if you chose a creature this way" reads the standing choices back, the
same manner "not chosen ... this way" excludes [CR#101.4]. -/
def sculptedSunburst : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpted Sunburst", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ choose (a creatureYouControl),
              chooses (each .opponent)
                (a (comparesOwnStat .power (.and [creature, .hasPossessor .controller they])
                  .atMost (.statOf (.stat .power) it))),
              .if_ (.choseThisWay .you creature) (exile (each (.and [creature, .notChosen]))) none ]) ] } }

end Semantics.Cards
