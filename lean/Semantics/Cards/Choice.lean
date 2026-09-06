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
  .sequence [.draw (.lit 2) (agent := .you), discard (counted (exactly 2) (.inZone hand)) (agent :=
      .you)]
theorem okCarefulStudy : Instruction.check [] carefulStudy = [] := by decide

def zombieInfestation : Ability :=
  activated (.perform (discard (counted (exactly 2) (.inZone hand)) (agent := .you)))
    (create (.lit 1) (creatureToken 2 2 [.black] [creatureType "Zombie"]))
theorem okZombieInfestation : Ability.check [] zombieInfestation = [] := by decide

def fulgentDistraction : Instruction :=
  .sequence
    [ choose (.described (.target (exactly 2)) creature),
      .setStatus .tapped (those (.type .creature)),
      .unattach (allOf (.and [ .hasSubtype (artifactType "Equipment"),
                               .attachedTo (those (.type .creature)) ])) ]
theorem okFulgentDistraction : Instruction.check [] fulgentDistraction = [] := by decide

def continueSpell : Instruction :=
  .sequence
    [ choose (.described (.target (upTo 4)) (.and [creature, .inZone (graveyardOf .you)])),
      move them battlefield ]
theorem okContinueSpell : Instruction.check [] continueSpell = [] := by decide

def kindredDominance : Instruction :=
  .sequence
    [ choose (a (quality (.subtype .creature))),
      destroy (allOf (.and [creature, .not (ofChosen (.subtype .creature))])) ]
theorem okKindredDominance : Instruction.check [] kindredDominance = [] := by decide

def phantomBlade : Ability :=
  when (.enters thisEquipment none)
    (.sequence
      [ .attachTo it
          (.described (.target (upTo 1)) (.and [creature, .hasPossessor .controller .you])),
        destroy (.described (.target (upTo 1)) (.and [creature, .other])) ])
theorem okPhantomBlade : Ability.check [] phantomBlade = [] := by decide

/-- Braids's Frightful Return -/
def braidsFrightfulReturn : Instruction :=
  .offer (sacrifice (a creature) (agent := .you)) (some (discard (a (.inZone hand)) (agent := (each
      .opponent)))) none (agent := .you)
theorem okBraidsFrightfulReturn : Instruction.check [] braidsFrightfulReturn = [] := by decide
/-- Daretti, Ingenious Iconoclast -/
def darettisMinusOne : Instruction :=
  .offer (sacrifice (a artifact) (agent := .you)) (some (destroy (target (.or [artifact,
      creature])))) none (agent := .you)
theorem okDarettisMinusOne : Instruction.check [] darettisMinusOne = [] := by decide

def cheeringFanatic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cheering Fanatic", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin"],
      text :=
        [ whenever (attacks thisCreature)
            (.sequence
              [ choose (a (quality .cardName)),
                .establish
                  (.costShift (allOf (.and [spell, ofChosen .cardName])) (.less (.lit 1) none))
                  (some .thisTurn) ]) ],
      power := stat 2, toughness := stat 2 } }

def rainOfThorns : Instruction :=
  chooseModes (atLeast 1)
    [destroy (target artifact), destroy (target enchantment), destroy (target land)]
theorem okRainOfThorns : Instruction.check [] rainOfThorns = [] := by decide

def rankleMasterOfPranks : Instruction :=
  chooseModes anyNumber
    [ discard (a (.inZone hand)) (agent := (each .anyPlayer)),
      .sequence [loseLife (.lit 1) (agent := (each .anyPlayer)), .draw (.lit 1) (agent := (those
          .player))],
      sacrifice (aTheirChoice creature) (agent := (each .anyPlayer)) ]
theorem okRankleMasterOfPranks : Instruction.check [] rankleMasterOfPranks = [] := by decide

def myrkulsEdict : Instruction :=
  .sequence [choose (a .opponent), sacrifice (aTheirChoice creature) (agent := (that .player))]
theorem okMyrkulsEdict : Instruction.check [] myrkulsEdict = [] := by decide
def moltingHarpy : Instruction := doUnless (sacrifice thisCreature (agent := .you)) (.mana [generic
    2]) (agent := .you)
theorem okMoltingHarpy : Instruction.check [] moltingHarpy = [] := by decide
def carnophage : Instruction := doUnless (.setStatus .tapped thisCreature) (payLife .you 1) (agent
    := .you)
theorem okCarnophage : Instruction.check [] carnophage = [] := by decide
def solitaryConfinement : Instruction :=
  .offer (discard (a (.inZone hand)) (agent := .you)) none (some (sacrifice thisEnchantment (agent
      := .you))) (agent := .you)
theorem okSolitaryConfinement : Instruction.check [] solitaryConfinement = [] := by decide

def yasminKhan : Ability :=
  activated .tapSymbol
    (.sequence
      [ exile (topSlice (.lit 1)),
        .establish
          (mayPlayDeed (.action "Play") .you it none (.play none none none false .itsOwnCost))
          (some untilYourNextEndStep) ])
theorem okYasminKhan : Ability.check [] yasminKhan = [] := by decide

/-- Brazen Cannonade -/
def brazenCannonadePermission : Instruction :=
  .sequence
    [ exile (topSlice (.lit 1)),
      .establish
        (mayPlayDeed (.action "Play") .you it none (.play none none none false .itsOwnCost))
        (some (.until_ (.endOf .combat (some .you)))) ]
theorem okBrazenCannonadePermission : Instruction.check [] brazenCannonadePermission = [] := by
  decide

def thousandMoonsCrackshot : Ability :=
  whenever (attacks thisCreature)
    (offerWhen (.pay (.mana [generic 2, pip .white]) .once (agent := .you))
      (.setStatus .tapped (target creature)) (agent := .you))
theorem okThousandMoonsCrackshot : Ability.check [] thousandMoonsCrackshot = [] := by decide

def zimoneQuandrixProdigy : Ability :=
  activated (.compound [.mana [generic 1], .tapSymbol])
    (offer (putOntoBattlefieldTapped (a (.and [land, .inZone (handOf .you)]))) (agent := .you))
theorem okZimoneQuandrixProdigy : Ability.check [] zimoneQuandrixProdigy = [] := by decide

def preeminentCaptain : Ability :=
  whenever (attacks thisCreature)
    (offer (putOntoBattlefieldTappedAttacking
      (a (.and [.hasSubtype (creatureType "Soldier"), creature, .inZone (handOf .you)]))) (agent :=
          .you))
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
                  .perform (sacrifice (counted (exactly 2) creature) (agent := .you)) ])))) ],
      power := stat 6, toughness := stat 6 } }

def escapeToTheWilds : Instruction :=
  .sequence
    [ exile (topSlice (.lit 5)),
      .establish
        (mayPlayDeed (.action "Play") .you (theVerbed (.action "Exile") .card .thisWay .many) none
          (.play none none none false .itsOwnCost))
        (some (.until_ (.endOf .turn (some .you)))) ]
theorem okEscapeToTheWilds : Instruction.check [] escapeToTheWilds = [] := by decide

/-- Muse Vessel -/
def museVesselPlay : Ability :=
  activated (.mana [generic 1])
    (.sequence
      [ choose (a exiledWithThisArtifact),
        .establish
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
  .static (.conjunction none
    [ .modification thisCreature .power (.up (.letter .x)),
      .modification thisCreature .toughness (.up (.letter .y)),
      .letterDefinition .x
        (.statOf (.stat .power) (the (.and [creature, .exiledWith thisCreature]))),
      .letterDefinition .y (.statOf (.stat .toughness) (that .card)) ])
theorem okPhyrexianIngesterPump : Ability.check [] phyrexianIngesterPump = [] := by decide

/-- Phyrexian Ingester -/
def phyrexianIngester : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Ingester", cost := some [generic 6, pip .blue], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Beast"],
      text :=
        [ abilityWord "imprint"
            (when (.enters thisCreature none)
              (offer (exile (target (.and [creature, nontoken]))) (agent := .you))),
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
            (offer (.draw (.lit 1) (agent := .you)) (agent := .you)) ],
      power := stat 2, toughness := stat 4 } }

def murmursFromBeyond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Murmurs from Beyond", cost := some [generic 2, pip .blue], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .spell none (.sequence
            [ revealCards (topSlice (.lit 3)),
              choose (someOf (exactly 1) them) (agent := some (a .opponent)),
              move (that .card) graveyard,
              move (theRest .object) hand ]) ] } }

/-- Helica Glider -/
def helicaGlider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Helica Glider", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Nightmare", creatureType "Squirrel"],
      text :=
        [ .static (.entryRider thisCreature
            (.withCounters (.lit 1) (.chosen [flyingCounter, .keyword "FirstStrike"]) .fresh)) ],
      power := stat 2, toughness := stat 2 } }

/-- Eager Construct -/
def eagerConstruct : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eager Construct", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text := [when (.enters thisCreature none) (offer (scry (.lit 1) (agent := they)) (agent :=
          (each .anyPlayer)))],
      power := stat 2, toughness := stat 2 } }

/-- Rune Snag -/
def runeSnag : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rune Snag", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ .spell none (doUnless (.counterSpell it)
            (.compound
              [ .mana [generic 2],
                scaledMana .generic
                  (times (.lit 2)
                    (countOf (.and [.named (.printed "Rune Snag"), .inZone graveyard]))) ]) (agent
                        := (controllerOf (target spell)))) ] } }

/-- Tahngarth, First Mate -/
def tahngarthChoosesDefender : Instruction :=
  .choose none
    (a (.and [ .or [.hasType .planeswalker, .anyPlayer],
               .inCombat .attackedBy (some (that .player)) ]))
    .openly none (agent := none)
theorem okTahngarthChoosesDefender :
    Instruction.check (GameEvent.intro [] tahngarthHeader) tahngarthChoosesDefender = [] := by
  decide

def reefShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reef Shaman", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Shaman"],
      text :=
        [ activated .tapSymbol
            (.establish
              (.qualityChange (target land) .sets
                (.chosenQuality (.ofYourChoice (.subtype .land) (some .basicTypesOnly))))
              (some untilEndOfTurn)) ],
      power := stat 0, toughness := stat 2 } }

def grixisIllusionist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grixis Illusionist", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated .tapSymbol
            (.establish
              (.qualityChange (target (.and [land, .hasPossessor .controller .you])) .sets
                (.chosenQuality (.ofYourChoice (.subtype .land) (some .basicTypesOnly))))
              (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

def distantMelody : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Distant Melody", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ choose (a (quality (.subtype .creature))),
              .draw
                (forEach 1
                  (.and [permanent, .hasPossessor .controller .you, ofChosen (.subtype .creature)]))
                      (agent := .you) ]) ] } }

def cripplingFear : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crippling Fear", cost := some [generic 2, pip .black, pip .black],
      types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ choose (a (quality (.subtype .creature))),
              get (allOf (.and [creature, .not (ofChosen (.subtype .creature))]))
                (.down (.lit 3)) (.down (.lit 3)) (some untilEndOfTurn) ]) ] } }

/-- Koh, the Face Stealer -/
def kohChooser : Ability :=
  activated (payLife .you 1) (choose (a (.and [creature, .exiledWith .this])))
theorem okKohChooser : Ability.check [] kohChooser = [] := by decide
/-- Forgotten Lore -/
def forgottenLoreChoice : Instruction :=
  choose (a (.inZone (graveyardOf .you))) (agent := some (target .opponent))
theorem okForgottenLoreChoice : Instruction.check [] forgottenLoreChoice = [] := by decide

/-- Psychic Paper minus its three-way coordination -/
def psychicPaperChoiceAndReads : List Ability :=
  [ .static (.conjunction none
      [ attachChoosing thisEquipment .cardName,
        attachChoosing thisEquipment (.subtype .creature) ]),
    .static (.conjunction none
      [ .qualityChange (.attachHost .equipped (.type .creature)) .sets
          (.chosenQuality (ofTheLastChosen .cardName)),
        .qualityChange (.attachHost .equipped (.type .creature)) .sets
          (.chosenQuality (ofTheLastChosen (.subtype .creature))) ]) ]
theorem okPsychicPaperChoiceAndReads : Ability.checkText [] psychicPaperChoiceAndReads = [] := by
  decide

def xenograft : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Xenograft", cost := some [generic 4, pip .blue], types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment (.subtype .creature)),
          .static (.qualityChange (allOf (.and [creature, .hasPossessor .controller .you])) .adds
            (.chosenQuality (ofChosen (.subtype .creature)))) ] } }

/-- Convincing Mirage -/
def convincingMirage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Convincing Mirage", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          .static (entersChoosingFrom thisAura (.subtype .land) .basicTypesOnly),
          .static (.qualityChange (.attachHost .enchanted (.type .land)) .sets
            (.chosenQuality (ofChosen (.subtype .land)))) ] } }

/-- Realmwright -/
def realmwright : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Realmwright", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Vedalken", creatureType "Wizard"],
      text :=
        [ .static (entersChoosingFrom thisCreature (.subtype .land) .basicTypesOnly),
          .static (.qualityChange (allOf (.and [land, .hasPossessor .controller .you])) .adds
            (.chosenQuality (ofChosen (.subtype .land)))) ],
      power := stat 1, toughness := stat 1 } }

def adaptiveAutomaton : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adaptive Automaton", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text :=
        [ .static (entersChoosing thisCreature (.subtype .creature)),
          .static (.qualityChange thisCreature .adds (.chosenQuality (ofChosen (.subtype
              .creature)))),
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
            (.establish
              (.qualityChange thisCreature .sets
                (.chosenQuality (.ofYourChoice (.subtype .creature) none)))
              (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 1 } }

def arcaneAdaptation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Adaptation", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment (.subtype .creature)),
          .static (.offBattlefieldScope
            (.qualityChange (allOf (.and [creature, .hasPossessor .controller .you])) .adds
              (.chosenQuality (ofChosen (.subtype .creature))))) ] } }

def conspiracy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Conspiracy", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment (.subtype .creature)),
          .static (.offBattlefieldScope
            (.qualityChange (allOf (.and [creature, .hasPossessor .controller .you])) .sets
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
            (.sequence
              [ choose (a (qualityFrom (.subtype .creature) (.typeOtherThan (creatureType "Wall")))),
                .establish
                  (.qualityChange (target creature) .sets (.chosenQuality (ofChosen (.subtype
                      .creature))))
                  (some untilEndOfTurn) ]) ],
      power := stat 1, toughness := stat 1 } }

def unnaturalSelection : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unnatural Selection", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ activated (.mana [generic 1])
            (.sequence
              [ choose (a (qualityFrom (.subtype .creature) (.typeOtherThan (creatureType "Wall")))),
                .establish
                  (.qualityChange (target creature) .sets (.chosenQuality (ofChosen (.subtype
                      .creature))))
                  (some untilEndOfTurn) ]) ] } }

def standardize : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Standardize", cost := some [pip .blue, pip .blue], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ choose (a (qualityFrom (.subtype .creature) (.typeOtherThan (creatureType "Wall")))),
              .establish
                (.qualityChange (each creature) .sets (.chosenQuality (ofChosen (.subtype
                    .creature))))
                (some untilEndOfTurn) ]) ] } }

def silverquillSilencer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Silverquill Silencer", cost := some [pip .white, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ .static (entersChoosingFrom thisCreature .cardName (.nameOfCard (.not land))),
          whenever (.casts anOpponent (a (.and [spell, .named .chosen])) none)
            (.sequence [loseLife (.lit 3) (agent := they), .draw (.lit 1) (agent := .you)]) ],
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
        [ .spell none (.sequence
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
        [ .spell none (.sequence
            [ choose (a (quality .cardName)),
              .establish
                (.conjunction none
                  [ objectCant (.action "Cast") (allOf (.and [spell, .named .chosen])),
                    objectCant (.action "Play") (allOf (.and [land, .named .chosen])) ])
                (some untilYourNextTurn) ]),
          .spell none (.draw (.lit 1) (agent := .you)) ] } }

def foundingOfOmashu : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Founding of Omashu", cost := some [generic 2, pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Saga"],
      text :=
        [ when (.chapterMark [1]) (create (.lit 2) (creatureToken 1 1 [.white] [creatureType "Ally"])),
          when (.chapterMark [2])
            (.offer (discard (a (.inZone hand)) (agent := .you)) (some (.draw (.lit 1) (agent :=
                .you))) none (agent := .you)),
          when (.chapterMark [3])
            (get (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn)) ] }
                }

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
        [ .spell none (.establish
            (mayPlayDeed (.action "Cast") .you (allOf spell) asThoughFlash
              (.play none none none false .itsOwnCost))
            (some .thisTurn)),
          .spell none (.draw (.lit 1) (agent := .you)) ] } }

def gisaAndGeralf : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gisa and Geralf", cost := some [generic 2, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ when (.enters thisCreature none) (mill (.lit 4) .you (agent := .you)),
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
  .conjunction none [playLandsFromTop, castFromTop spell]
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
            (gainLife (.lit 1) (agent := .you)) ],
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
          .static (.conjunction none [castFromTop (.and [spell, artifact]), castFromTop (.and
              [spell, colorless])]),
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
      text := [.spell none (.establish (mayPlayAdditionalLands .you (upTo 3)) (some .thisTurn))] } }

def explore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Explore", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ .establish (mayPlayAdditionalLands .you (exactly 1)) (some .thisTurn),
              .draw (.lit 1) (agent := .you) ]) ] } }

def urbanEvolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urban Evolution", cost := some [generic 3, pip .green, pip .blue],
      types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ .draw (.lit 3) (agent := .you),
              .establish (mayPlayAdditionalLands .you (exactly 1)) (some .thisTurn) ]) ] } }

def dryadOfTheIlysianGrove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dryad of the Ilysian Grove", cost := some [generic 2, pip .green],
      types := [.enchantment, .creature], subtypes := [creatureType "Nymph", creatureType "Dryad"],
      text :=
        [ .static (mayPlayAdditionalLands .you (exactly 1)),
          .static (.qualityChange (allOf (.and [land, .hasPossessor .controller .you])) .adds
            (.everyTypeOf .basicLand)) ],
      power := stat 2, toughness := stat 4 } }

/-- Ashes of the Fallen -/
def ashesOfTheFallen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ashes of the Fallen", cost := some [generic 2], types := [.artifact],
      text :=
        [ .static (entersChoosing thisArtifact (.subtype .creature)),
          .static (.qualityChange (each (.and [creature, .inZone (graveyardOf .you)])) .adds
            (.chosenQuality (ofChosen (.subtype .creature)))) ] } }

def hellkiteCharger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hellkite Charger", cost := some [generic 4, pip .red, pip .red],
      types := [.creature], subtypes := [creatureType "Dragon"],
      text :=
        [ keyword "Flying", keyword "Haste",
          whenever (attacks thisCreature)
            (.offer (.pay (.mana [generic 5, pip .red, pip .red]) .once (agent := .you))
              (some (.sequence
                [ .setStatus .untapped (allOf (.and [creature, attacking])),
                  addPart .combat none (.lit 1) ]))
              none (agent := .you)) ],
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
  .sequence
    [ .choose none (a (.inZone (graveyardOf .you))) .openly none (agent := (some (target
        .opponent))),
      .offer (.pay (.mana [pip .green]) .once (agent := .you)) (some (.repeat_
          .againExcludingChosen)) none (agent := .you) ]
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
          whenever (.enters (a creatureYouControl) none) (offer (gainLife (.lit 1) (agent := .you))
              (agent := .you)) ] } }

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
            (doUnless (offer (.draw (.lit 1) (agent := .you)) (agent := .you)) (.mana [generic 1])
                (agent := (that .player))) ] } }

def gandalfWhiteRider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gandalf, White Rider", cost := some [generic 3, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Avatar", creatureType "Wizard"],
      text :=
        [ keyword "Vigilance",
          whenever (.casts .you (a spell) none)
            (.sequence
              [ get (each creatureYouControl) (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn),
                scry (.lit 1) (agent := .you) ]),
          when (.dies thisCreature) (offer (move it (nthFromTop (.nth 5))) (agent := .you)) ],
      power := stat 3, toughness := stat 3 } }

def helmOfPossession : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Helm of Possession", cost := some [generic 4], types := [.artifact],
      text :=
        [ .static (mayDeclineUntap thisArtifact (some .you)),
          activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice (a creature)
              (agent := .you))])
            (gainControl (target creature)
              (some (.forAsLongAs
                (.and [ .matches thisArtifact (.hasPossessor .controller .you),
                        .matches thisArtifact tapped ]))) (agent := .you)) ] } }

def override : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Override", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ .spell none (doUnless (.counterSpell it)
            (scaledMana .generic (forEach 1 (.and [artifact, .hasPossessor .controller .you])))
                (agent := (controllerOf (target spell)))) ] } }

def rakshasasDisdain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rakshasa's Disdain", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ .spell none (doUnless (.counterSpell it)
            (scaledMana .generic (forEach 1 (.inZone (graveyardOf .you)))) (agent := (controllerOf
                (target spell)))) ] } }

def megatherium : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Megatherium", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ keyword "Trample",
          when (.enters thisCreature none)
            (doUnless (sacrifice thisCreature (agent := .you))
              (scaledMana .generic (forEach 1 (.inZone (handOf .you)))) (agent := .you)) ],
      power := stat 4, toughness := stat 4 } }

def killingWave : Instruction :=
  .doForEach (each creature)
    (doUnless (sacrifice it (agent := they)) (.perform (loseLife (.letter .x) (agent := they)))
        (agent := (controllerOf it)))
theorem okKillingWave : Instruction.check [] killingWave = [] := by decide
def fadeAway : Instruction :=
  .doForEach (each creature)
    (doUnless (sacrifice (a permanent) (agent := they)) (.mana [generic 1]) (agent := (controllerOf
        it)))
theorem okFadeAway : Instruction.check [] fadeAway = [] := by decide

def tidalFlats : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tidal Flats", cost := some [pip .blue], types := [.enchantment],
      text :=
        [ activated (.mana [pip .blue, pip .blue])
            (.doForEach (each (.and [creature, attacking, .not (.hasKeyword (.the "Flying"))]))
              (.offer (.pay (.mana [generic 1]) .once (agent := they)) none
                (some (gain
                  (allOf (.and [ creature, .hasPossessor .controller .you,
                                 .inCombat .blockerOf (some (that (.type .creature))) ]))
                  (keyword "FirstStrike") (some untilEndOfTurn))) (agent := (controllerOf it)))) ] }
                      }

def primalSurge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Primal Surge", cost := some [generic 8, pip .green, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ exile (topSlice (.lit 1)),
              .doIf (itsA permanentCard)
                (.offer (putOntoBattlefield it) (some (.repeat_ .again)) none (agent := .you)) none
                    ]) ] } }

def cultivatorColossus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cultivator Colossus", cost := some [generic 4, pip .green, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Plant", creatureType "Beast"],
      text :=
        [ keyword "Trample",
          .static (.ptDefinition thisCreature .bothEach
            (countOf (.and [land, .hasPossessor .controller .you]))),
          when (.enters thisCreature none)
            (.offer (putOntoBattlefieldTapped (a (.and [land, .inZone (handOf .you)])))
              (some (.sequence [.draw (.lit 1) (agent := .you), .repeat_ .again])) none (agent :=
                  .you)) ] } }

def zimoneAndDina : Ability :=
  activated (.compound [.tapSymbol, .perform (sacrifice (a (otherCreature thisCreature)) (agent :=
      .you))])
    (.sequence
      [ .draw (.lit 1) (agent := .you),
        offer (putOntoBattlefieldTapped (a (.and [land, .inZone (handOf .you)]))) (agent := .you),
        .doIf (.compareAmt (countOf (.and [land, .hasPossessor .controller .you])) .atLeast (.lit
            8))
          (.repeat_ (.moreTimes (.lit 1))) none ])
theorem okZimoneAndDina : Ability.check [] zimoneAndDina = [] := by decide

def cryptLurker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crypt Lurker", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Horror"],
      text :=
        [ when (.enters thisCreature none)
            (.offer
              (chooseModes (exactly 1)
                [ sacrifice (a creature) (agent := .you),
                  discard (a (.and [creature, .inZone hand])) (agent := .you) ])
              (some (.draw (.lit 1) (agent := .you))) none (agent := .you)) ],
      power := stat 3, toughness := stat 4 } }

/-- Memoricide -/
def memoricideSearch : Instruction :=
  .sequence
    [ choose (a (qualityFrom .cardName (.nameOfCard (.not land)))),
      searchZonesOf (target .anyPlayer) (exactly 1) (.named .chosen),
      .shuffle (agent := (that .player)) ]
theorem okMemoricideSearch : Instruction.check [] memoricideSearch = [] := by decide
/-- Lost Hours -/
def lostHoursPlacement : Instruction :=
  .sequence
    [ revealTheirHand (agent := (target .anyPlayer)),
      choose (a (.and [.not land, .inZone (handOf they)])),
      put it (nthFromTop (.nth 3)) (agent := (that .player)) ]
theorem okLostHoursPlacement : Instruction.check [] lostHoursPlacement = [] := by decide
/-- Aether Gust -/
def aetherGustPlacement : Instruction :=
  .sequence
    [ choose (target (.and [permanent, .colorIs .red])),
      put it (choiceOfTopOrBottom they) (agent := (ownerOf it)) ]
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
            (offer (.draw (.upTo (.lit 3)) (agent := they)) (agent := (each .opponent))) ],
      power := stat 6, toughness := stat 6 } }

/-- Memory Plunder -/
def memoryPlunder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Memory Plunder",
      cost := some [hybridPip .blue .black, hybridPip .blue .black, hybridPip .blue .black,
                    hybridPip .blue .black],
      types := [.instant],
      text :=
        [ .spell none (.establish
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
def tranquilFrillbackOffer : Instruction := offer (.pay (.mana [pip .green]) (.upTo 3) (agent :=
    .you)) (agent := .you)
theorem okTranquilFrillbackOffer : Instruction.check [] tranquilFrillbackOffer = [] := by decide

/-- Caller of the Hunt -/
def callerOfTheHunt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caller of the Hunt", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ .static (.addedCost (.perform (choose (a (quality (.subtype .creature))))) false),
          .static (.ptDefinition thisCreature .bothEach
            (countOf (.and [creature, ofChosen (.subtype .creature)]))) ] } }

/-- Duneblast -/
def duneblast : Instruction :=
  .sequence [choose (counted (upTo 1) creature), destroy (theRest .object)]
theorem okDuneblast : Instruction.check [] duneblast = [] := by decide
/-- Boreas Charger -/
def boreasChargerChoice : Instruction := choose (a opponentWithMoreLands)
theorem okBoreasChargerChoice : Instruction.check [] boreasChargerChoice = [] := by decide
/-- Boreas Charger's spell text -/
def boreasChargerSpell : Instruction :=
  .sequence
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
            (.sequence
              [ choose anOpponent,
                .doIf (.compareAmt (countOf (.inZone (handOf (that .player)))) .greater
                        (countOf (.inZone (handOf .you))))
                  (.draw .theDifference (agent := .you)) none ]) ],
      power := stat 4, toughness := stat 4 } }

/-- Slithermuse -/
def slithermuseTrigger : Ability :=
  when (leavesBattlefield thisCreature)
    (.sequence
      [ choose anOpponent,
        .doIf (.compareAmt (countOf (.inZone (handOf (that .player)))) .greater
                (countOf (.inZone (handOf .you))))
          (.draw .theDifference (agent := .you)) none ])
theorem okSlithermuseTrigger : Ability.check [] slithermuseTrigger = [] := by decide

/-- Celestial Judgment -/
def celestialJudgment : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celestial Judgment", cost := some [generic 4, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ .doForEachKind (.value .power) (some (allOf creature)) .number
                (choose (a (.and [creature, .compare [.stat .power] .eq chosenNumber]))),
              destroy (each (.and [creature, .notChosen])) ]) ] } }

/-- World Queller -/
def worldQuellerChoice : Instruction :=
  .offer (choose (a (quality .cardType)))
    (some (sacrifice (aTheirChoice (.and [permanent, ofChosen .cardType])) (agent := (each
        .anyPlayer)))) none (agent := .you)
theorem okWorldQuellerChoice : Instruction.check [] worldQuellerChoice = [] := by decide

/-- Moonlit Meditation -/
def moonlitMeditation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Moonlit Meditation", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" (.and [.or [artifact, creature], .hasPossessor .controller .you]),
          .static (.replacement (.tokensCreated (counted (atLeast 1) .isToken) false (some .you)
              none)
            [] none
            (offer (.create .groupSize (.copyOf (.attachHost .enchanted .permanent) []) [] (agent :=
                .you)) (agent := .you))
            .repeatedly (some .oncePerTurn)) ] } }

def discardUpToTwoThenDrawThatMany : Instruction :=
  .sequence [discard (counted (upTo 2) (.inZone hand)) (agent := .you), .draw .groupSize (agent :=
      .you)]
theorem okDiscardUpToTwoThenDrawThatMany :
    Instruction.check [] discardUpToTwoThenDrawThatMany = [] := by decide
/-- Truce and Temporary Truce -/
def drawUpToTwoThenGainPerShortfall : Instruction :=
  .sequence
    [ offer (.draw (.upTo (.lit 2)) (agent := they)) (agent := (each .anyPlayer)),
      gainLife (times (.lit 2) shortOfCeiling) (agent := they) ]
theorem okDrawUpToTwoThenGainPerShortfall :
    Instruction.check [] drawUpToTwoThenGainPerShortfall = [] := by decide

/-- Commune with the Gods -/
def communeWithTheGods : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Commune with the Gods", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ revealCards (topSlice (.lit 5)),
              offer (move (fromAmong (exactly 1) (.or [creature, enchantment]) them) hand) (agent :=
                  .you),
              move (theRest .object) graveyard ]) ] } }

/-- Nautiloid Ship -/
def nautiloidShipTrigger : Ability :=
  whenever (dealsCombatDamage thisVehicle (a .anyPlayer))
    (offer (putOntoBattlefieldUnderYourControl (a (.and [creature, .exiledWith thisVehicle])))
        (agent := .you))
theorem okNautiloidShipTrigger : Ability.check [] nautiloidShipTrigger = [] := by decide

/-- Summon: Esper Valigarmanda's II/III/IV body, in part -/
def summonEsperValigarmandaCast : StaticSpec :=
  .conjunction none
    [ mayPlayDeed (.action "Cast") .you (a (.and [instantOrSorcery, .exiledWith thisSaga])) none
        (.play none none none false .itsOwnCost),
      maySpendAsThough .you none .anyType
        (some (.toCast (.and [instantOrSorcery, .exiledWith thisSaga]))) ]
theorem okSummonEsperValigarmandaCast :
    StaticSpec.check [] summonEsperValigarmandaCast = [] := by decide
/-- Rogue Class's level-3 body -/
def rogueClassLevelThree : StaticSpec :=
  .conjunction none
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
          .static (.controlGrant .you (.attachHost .enchanted (.type .creature))),
          activatedBy
            (.perform (.repeatTimes (.lit 2)
              (.sequence [choose (a (.inZone hand)), discard (that .card) (agent := .you)])))
            (.sequence [sacrificeIt (agent := (controllerOf thisAura)), .draw (.lit 2) (agent :=
                they)])
            (.playerGroup .yourOpponents) ] } }

/-- Vraska's Scorn -/
def vraskasScorn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vraska's Scorn", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ loseLife (.lit 4) (agent := (target .opponent)),
              offer
                (.sequence
                  [ searchLibraryOrGraveyard (.named (.printed "Vraska, Scheming Gorgon")),
                    revealCards it, move it hand ]) (agent := .you),
              .doIf (happenedAt (.verbedAct (.action "Search")) .you .thisWay yourLibrary) shuffle
                none ]) ] } }

/-- Old-Growth Dryads -/
def oldGrowthDryads : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Old-Growth Dryads", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Dryad"],
      text :=
        [ when (.enters thisCreature none)
            (offer
              (.sequence
                [ searchTheirLibraryFor (.and [land, .hasSupertype .basic]) (agent := they),
                  putOntoBattlefieldTapped foundCard,
                  .shuffle (agent := they) ]) (agent := (each .opponent))) ],
      power := stat 3, toughness := stat 3 } }

/-- Verity Circle -/
def verityCircle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Verity Circle", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ triggeredIf
            (.statusEvent (a (.and [creature, .hasPossessor .controller anOpponent])) .tapped)
            (.matches it (.not (.inCombat .declaredAttacker none)))
            (offer (.draw (.lit 1) (agent := .you)) (agent := .you)),
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
          .static (.conditional
            (mayPlayDeed (.action "Cast") .you
              (allOf (.and [spell, .hasSubtype (creatureType "Knight")])) none
              (.play (some (graveyardOf .you)) none none false .itsOwnCost))
            (.matches .this (.inZone battlefield)) .asLongAs),
          when (.dies thisCreature) (loseLife (.lit 2) (agent := .you)) ],
      power := stat 3, toughness := stat 3 } }

/-- Apex of Power -/
def apexOfPowerCast : Instruction :=
  .sequence
    [ exile (.librarySlice .top (.lit 7) .you),
      .establish
        (mayPlayDeed (.action "Cast") .you (fromAmong anyNumber spell them) none
          (.play none none none false .itsOwnCost))
        (some .thisTurn) ]
theorem okApexOfPowerCast : Instruction.check [] apexOfPowerCast = [] := by decide
/-- Blight Herder -/
def blightHerderCast : Instruction :=
  offer
    (move (counted (exactly 2)
            (.and [.isCard, .hasPossessor .owner (.playerGroup .yourOpponents), .inZone exileZone]))
      graveyard) (agent := .you)
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
    (.offer
      (.unattach (a (.and [ .hasSubtype (artifactType "Equipment"),
                            .attachedTo (a creatureYouControl) ])))
      (some (.setStatus .tapped (that (.type .creature)))) none (agent := .you))
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
            (.abilityGrantFrom thisCreature [.anyActivated] (topSlice (.lit 1)) none)
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
  activatedOnlyOnce (.perform (.expose .reveal (.choice .player) (agent := .you)))
    (.doOnlyIf (.chooseNewTargets (target (.or [spell, .abilityHead .anyOnStack])))
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
      text := [.spell none (.draw (.letter .x) (agent := (each .anyPlayer)))] } }

def collectiveUnconscious : Instruction := .draw (forEach 1 creatureYouControl) (agent := .you)
theorem okCollectiveUnconscious : Instruction.check [] collectiveUnconscious = [] := by decide
def killiansConfidence : Instruction :=
  .sequence
    [get (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn), .draw (.lit 1)
        (agent := .you)]
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
        [ .spell none (discard (countedAtRandom (exactly 2) (.inZone hand)) (agent := (target
            .anyPlayer))) ] } }

/-- Caught in the Crossfire -/
def caughtInTheCrossfire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caught in the Crossfire", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ .spell none (chooseSpree
            [ (some (.mana [generic 1]),
               .dealDamage .this (.lit 2) (each (.and [creature, outlaw]))),
              (some (.mana [generic 1]),
               .dealDamage .this (.lit 2) (each (.and [creature, .not outlaw]))) ]) ] } }

/-- Requisition Raid -/
def requisitionRaid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Requisition Raid", cost := some [pip .white], types := [.sorcery],
      text :=
        [ .spell none (chooseSpree
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
        [ .spell none (chooseSpree
            [ (some (.mana [generic 1]),
               .setStatus .untapped
                 (allOf (.and [creature, .hasPossessor .controller (target .anyPlayer)]))),
              (some (.mana [generic 1]),
               gain (target creature) (keyword "DoubleStrike") (some untilEndOfTurn)) ]) ] } }

/-- Consuming Tide -/
def consumingTide : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consuming Tide", cost := some [generic 2, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ choose
                (a (.and [permanent, .not land, .hasPossessor .controller they])) (agent := some
                    (each .anyPlayer)),
              returnTo (allOf (.and [permanent, .not land, .notChosen])) hand [],
              .doForEach
                (each (.compareOver .opponent (countOf (.inZone (handOf they))) .greater
                  (countOf (.inZone (handOf .you)))))
                (.draw (.lit 1) (agent := .you)) ]) ] } }

/-- Stick Together: a chosen party is four up-to-one choices, not the group the game computes
[CR#700.8d]. -/
def stickTogether : Instruction :=
  .sequence
    [ choose
        (counted (upTo 1)
          (.and [creature, .hasSubtype (creatureType "Cleric"), .hasPossessor .controller they]))
              (agent := some (each .anyPlayer)),
      choose
        (counted (upTo 1)
          (.and [creature, .hasSubtype (creatureType "Rogue"), .hasPossessor .controller they]))
              (agent := some (each .anyPlayer)),
      choose
        (counted (upTo 1)
          (.and [creature, .hasSubtype (creatureType "Warrior"), .hasPossessor .controller they]))
              (agent := some (each .anyPlayer)),
      choose
        (counted (upTo 1)
          (.and [creature, .hasSubtype (creatureType "Wizard"), .hasPossessor .controller they]))
              (agent := some (each .anyPlayer)),
      sacrifice (theRest .object) (agent := (each .anyPlayer)) ]
theorem okStickTogether : Instruction.check [] stickTogether = [] := by decide

/-- Disciple of Caelus Nin: "starting with you" fixes the order the players choose in
[CR#101.4]. -/
def discipleOfCaelusNin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disciple of Caelus Nin", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ when (.enters thisCreature none)
            (.sequence
              [ .choose (some .you)
                  (counted (upTo 5) (.and [permanent, .hasPossessor .controller they])) .openly none
                      (agent := (some (each .anyPlayer))),
                .setStatus .phasedOut
                  (allOf (.and [permanent, .otherThan thisCreature, .notChosen])) ]),
          .static (.deonticRule (allOf permanent) .forbid [.ofAbility "Phasing"] .agent none
              .noPatient
            none .noRider) ],
      power := stat 3, toughness := stat 4 } }

/-- Sculpted Sunburst: "if you chose a creature this way" reads the standing choices back, the
same manner "not chosen ... this way" excludes [CR#101.4]. -/
def sculptedSunburst : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpted Sunburst", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ choose (a creatureYouControl),
              choose
                (a (comparesOwnStat .power (.and [creature, .hasPossessor .controller they])
                  .atMost (.statOf (.stat .power) it))) (agent := some (each .opponent)),
              .doIf (.choseThisWay .you creature) (exile (each (.and [creature, .notChosen]))) none
                  ]) ] } }

end Semantics.Cards
