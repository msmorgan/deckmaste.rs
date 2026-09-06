import Semantics
import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Cards.Faces

Port of `idris/src/Experimental/Cards/Faces.idr`: the printed cards of the Faces family (the
frames that carry more than one set: transforming, flip, adventurer, split, room, leveler,
prototype) and the phrase-level bench items beside them, each with an `ok…` theorem.

Not ported: `shapeshifterBox` and `tarmogoyfBox` (a printed `*` box is `none` under the
printed-star ruling; there is no printed-box term to write).
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

def cyberConversion : Instruction :=
  .sequence
    [ .setStatus .faceDown (target creature),
      become it
        { characteristics :=
          { types := [.artifact, .creature], subtypes := [creatureType "Cyberman"],
            power := stat 2, toughness := stat 2 } }
        none ]
theorem okCyberConversion : Instruction.check [] cyberConversion = [] := by decide

def breakOpen : Instruction :=
  .setStatus .faceUp
    (target (.and [creature, faceDown, .hasPossessor .controller anOpponent]))
theorem okBreakOpen : Instruction.check [] breakOpen = [] := by decide

def jushiApprentice : Ability :=
  activated (.compound [.mana [generic 2, pip .blue], .tapSymbol])
    (.sequence
      [ .draw (.lit 1) (agent := .you),
        .doIf (.compareAmt (countOf (.inZone (handOf .you))) .atLeast (.lit 9))
          (.setStatus .flipped thisCreature) none ])
theorem okJushiApprentice : Ability.check [] jushiApprentice = [] := by decide

def invasionOfDominaria : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Invasion of Dominaria", cost := some [generic 2, pip .white],
      types := [.battle], subtypes := [.of .battle "Siege"],
      text :=
        [ when (.enters thisSiege none)
            (.sequence [gainLife (.lit 4) (agent := .you), .draw (.lit 1) (agent := .you)]) ],
      defense := stat 5 } }
  { characteristics :=
    { name := "Serra Faithkeeper", types := [.creature], subtypes := [creatureType "Angel"],
      text := [keyword "Flying", keyword "Vigilance"], power := stat 4, toughness := stat 4 } }

/-- Missy -/
def missyFaceDownReturn : Ability :=
  whenever (.dies (a (.and [creature, .not artifact, .otherThan thisCreature])))
    (.sequence
      [ .move it battlefield [.entersAs .faceDown, .entersAs .tapped, .under .you],
        .establish
          (.qualityChange it .sets
            (.bundle
              { characteristics :=
                { types := [.artifact, .creature], subtypes := [creatureType "Cyberman"],
                  power := stat 2, toughness := stat 2 } }
              none))
          none ])
theorem okMissyFaceDownReturn : Ability.check [] missyFaceDownReturn = [] := by decide

/-- Yedora, Grave Gardener -/
def yedoraGraveGardener : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yedora, Grave Gardener", cost := some [generic 4, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Treefolk", creatureType "Druid"],
      text :=
        [ whenever (.dies (a (.and [nontoken, creatureYouControl, .otherThan thisCreature])))
            (offer
              (.sequence
                [ .move it battlefield [.entersAs .faceDown, .under (ownerOf it)],
                  .establish
                    (.qualityChange it .sets
                      (.bundle
                        { characteristics := { types := [.land], subtypes := [landType "Forest"] } }
                        none))
                    none ]) (agent := .you)) ],
      power := stat 5, toughness := stat 5 } }

/-- Ral Zarek, Guest Lecturer's ultimate -/
def ralZarekGuestLecturerUltimate : Instruction :=
  .sequence
    [ .flipCoins (.count (.lit 5)) (agent := .you),
      .skipPart .turn (.letter .x) (agent := (target .opponent)),
      .define .x (.coinsShowing .heads) ]
theorem okRalZarekGuestLecturerUltimate :
    Instruction.check [] ralZarekGuestLecturerUltimate = [] := by decide

def faceDownFlyingCounter : StaticSpec :=
  entersWithCounters (allOf (.and [creature, .hasPossessor .controller .you, faceDown]))
    (.lit 1) flyingCounter
theorem okFaceDownFlyingCounter : StaticSpec.check [] faceDownFlyingCounter = [] := by decide

def goblinArchaeologist : Ability :=
  activated (.compound [.mana [pip .red], .tapSymbol])
    (.sequence
      [ flipCoins 1 (agent := .you),
        doIf (.flipCalled .you .wins)
          (.sequence [destroy (target artifact), .setStatus .untapped thisCreature]),
        doIf (.flipCalled .you .loses) (sacrifice thisCreature (agent := .you)) ])
theorem okGoblinArchaeologist : Ability.check [] goblinArchaeologist = [] := by decide

def chanceEncounter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chance Encounter", cost := some [generic 2, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ whenever (.flipsCoin .you (some .wins))
            (.putCounters (.lit 1) (.printed (.named "Luck")) thisEnchantment),
          triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
            (.compareAmt (countersOn (.named "Luck") thisEnchantment) .atLeast (.lit 10))
            (.conclude .winGame (agent := .you)) ] } }

/-- Karplusan Minotaur's win arm -/
def karplusanMinotaurWinFlip : Ability :=
  whenever (.flipsCoin .you (some .wins)) (.dealDamage thisCreature (.lit 1) (target anyTarget))
theorem okKarplusanMinotaurWinFlip : Ability.check [] karplusanMinotaurWinFlip = [] := by decide

/-- Ral Zarek's ultimate -/
def ralZarekUltimate : Instruction :=
  .sequence [flipCoins 5 (agent := .you), .addTurn (.coinsShowing .heads) (agent := .you)]
theorem okRalZarekUltimate : Instruction.check [] ralZarekUltimate = [] := by decide

/-- Krark's Thumb -/
def krarksThumbExtraFlip : Instruction :=
  replaceEvent (flipsCoin .you)
    (.sequence [flipCoins 2 (agent := .you), .ignoreOutcomes (.chosen none (.lit 1))]) none
theorem okKrarksThumbExtraFlip : Instruction.check [] krarksThumbExtraFlip = [] := by decide

/-- Goblin Assassin -/
def goblinAssassinCoinTails : Instruction :=
  .sequence
    [ .flipCoins (.count (.lit 1)) (agent := (each .anyPlayer)),
      sacrifice (aTheirChoice creature) (agent := (each (.and [.anyPlayer, .coinCameUp .tails]))) ]
theorem okGoblinAssassinCoinTails : Instruction.check [] goblinAssassinCoinTails = [] := by decide

/-- Rakdos, the Showstopper -/
def rakdosShowstopperFlips : Instruction :=
  .sequence
    [ .flipCoins
        (.per (each (.and [ creature,
                            .not (.or [ .hasSubtype (creatureType "Demon"),
                                        .hasSubtype (creatureType "Devil"),
                                        .hasSubtype (creatureType "Imp") ]) ]))) (agent := .you),
      destroy (each (.and [creature, .coinCameUp .tails])) ]
theorem okRakdosShowstopperFlips : Instruction.check [] rakdosShowstopperFlips = [] := by decide

def merfolkSecretkeeper : Spelled := spelled <| .adventurer
  { characteristics :=
    { name := "Merfolk Secretkeeper", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      power := stat 0, toughness := stat 4 } }
  { characteristics :=
    { name := "Venture Deeper", cost := some [pip .blue], types := [.sorcery],
      subtypes := [spellType "Adventure"],
      text := [ .spell none (mill (.lit 4) they (agent := (target .anyPlayer))) ] } }

def orochiEggwatcher : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Orochi Eggwatcher", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Snake", creatureType "Shaman"],
      text :=
        [ activated (.compound [.mana [generic 2, pip .green], .tapSymbol])
            (.sequence
              [ create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Snake"]),
                doIf (.compareAmt (countOf creatureYouControl) .atLeast (.lit 10))
                  (.setStatus .flipped thisCreature) ]) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Shidako, Broodmistress", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Snake", creatureType "Shaman"],
      text :=
        [ activated (.compound [.mana [pip .green], .perform (sacrifice (a creature) (agent :=
            .you))])
            (get (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) ],
      power := stat 3, toughness := stat 3 } }

def planeswalkerBackWithoutLoyalty : CardFace :=
  { characteristics :=
    { name := "", supertypes := [.legendary], types := [.planeswalker],
      subtypes := [planeswalkerType "Arlinn"] } }
theorem planeswalkerBackWithoutLoyaltyOk :
    CardFace.check .back planeswalkerBackWithoutLoyalty = [] := by decide

/-- Garruk Relentless -/
def garrukRelentlessFlip : Ability :=
  when
    (.stateHolds
      (.matches thisPlaneswalker (.compare [.counter (.named "Loyalty")] .atMost (.lit 2))))
    (transform thisPlaneswalker)
theorem okGarrukRelentlessFlip : Ability.check [] garrukRelentlessFlip = [] := by decide

/-- Mana Clash -/
def manaClashFlip : Instruction :=
  .flipCoins (.count (.lit 1)) (agent := (.eachOf (.both .you (target .opponent))))
theorem okManaClashFlip : Instruction.check [] manaClashFlip = [] := by decide

def akkiLavarunner : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Akki Lavarunner", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Warrior"],
      text :=
        [ keyword "Haste",
          whenever (.dealsDamage .any thisCreature (some anOpponent))
            (.setStatus .flipped thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Tok-Tok, Volcano Born", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Shaman"],
      text :=
        [ keywordQuality "Protection" (.colorIs .red),
          .static (.damageRule .any (.dealtBy (a (.and [source, .colorIs .red])))
            (.toRecipient (a .anyPlayer)) (.scale (.shifted .up (.lit 1))) .repeatedly) ],
      power := stat 2, toughness := stat 2 } }

def bushiTenderfoot : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Bushi Tenderfoot", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ when
            (.dies (a (.and [ creature,
                              .happenedTo (.mk .damageTaken .thisTurn
                                (some (.involving thisCreature))) ])))
            (.setStatus .flipped thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Kenzo the Hardhearted", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Samurai"],
      text := [keyword "DoubleStrike", keywordNumber "Bushido" (.lit 2)],
      power := stat 3, toughness := stat 4 } }

/-- Kitsune Mystic -/
def kitsuneMysticFlip : Ability :=
  triggeredIf (.beginningOf .the .endStep .noPossessor)
    (.matches thisCreature
      (.attachedBy (some .enchanted)
        (counted (atLeast 2) (.hasSubtype (enchantmentType "Aura")))))
    (.setStatus .flipped thisCreature)
theorem okKitsuneMysticFlip : Ability.check [] kitsuneMysticFlip = [] := by decide

/-- Keeper of the Lens -/
def keeperOfTheLens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keeper of the Lens", cost := some [generic 1], types := [.artifact, .creature],
      subtypes := [creatureType "Golem"],
      text :=
        [ .static (.visibility .lookAt .you
            (.objects (allOf (.and [creature, faceDown, .not (.hasPossessor .controller .you)])))) ],
      power := stat 1, toughness := stat 2 } }

/-- Lens of Clarity -/
def lensOfClarity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lens of Clarity", cost := some [generic 1], types := [.artifact],
      text :=
        [ .static (.conjunction none
            [ .visibility .lookAt .you .topOfLibrary,
              .visibility .lookAt .you
                (.objects
                  (allOf (.and [creature, faceDown, .not (.hasPossessor .controller .you)]))) ]) ] } }

def kitsuneMystic : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Kitsune Mystic", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Fox", creatureType "Wizard"],
      text := [kitsuneMysticFlip], power := stat 2, toughness := stat 3 } }
  { characteristics :=
    { name := "Autumn-Tail, Kitsune Sage", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Fox", creatureType "Wizard"],
      text :=
        [ activated (.mana [generic 1])
            (.attachTo
              (target (.and [.hasSubtype (enchantmentType "Aura"), .attachedTo (a creature)]))
              (a (.and [creature, .otherThan (that (.type .creature))]))) ],
      power := stat 4, toughness := stat 5 } }

/-- Vesuvan Shapeshifter -/
def vesuvanShapeshifterCopyDuration : Instruction :=
  .establish
    (.copyChange thisCreature (that (.type .creature))
      [ .ability
          (at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (offer (.setStatus .faceDown thisCreature) (agent := .you))) ])
    (some (.untilEvent (.statusEvent thisCreature .faceDown)))
theorem okVesuvanShapeshifterCopyDuration :
    Instruction.check [⟨.a, .one, .object (some .creature) (some .battlefield) none none none⟩]
      vesuvanShapeshifterCopyDuration = [] := by
  decide

/-- Arlinn Kord // Arlinn, Embraced by the Moon -/
def arlinnKord : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Arlinn Kord", cost := some [generic 2, pip .red, pip .green],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [planeswalkerType "Arlinn"],
      text :=
        [ activated (.loyaltySymbol (.up 1))
            (.establish
              (.conjunction none
                [ .modification (.described (.target (upTo 1)) creature) .power (.up (.lit 2)),
                  .modification it .toughness (.up (.lit 2)),
                  .abilityGrant it (keyword "Vigilance"),
                  .abilityGrant it (keyword "Haste") ])
              (some untilEndOfTurn)),
          activated (.loyaltySymbol .zero)
            (.sequence
              [ create (.lit 1) (creatureToken 2 2 [.green] [creatureType "Wolf"]),
                transform thisPlaneswalker ]) ],
      loyalty := stat 3 } }
  { characteristics :=
    { name := "Arlinn, Embraced by the Moon", supertypes := [.legendary],
      types := [.planeswalker], subtypes := [planeswalkerType "Arlinn"],
      text :=
        [ activated (.loyaltySymbol (.up 1))
            (.establish
              (.conjunction none
                [ .modification (allOf creatureYouControl) .power (.up (.lit 1)),
                  .modification them .toughness (.up (.lit 1)),
                  .abilityGrant them (keyword "Trample") ])
              (some untilEndOfTurn)),
          activated (.loyaltySymbol (.down 1))
            (.sequence
              [ .dealDamage .this (.lit 3) (target anyTarget), transform thisPlaneswalker ]),
          activated (.loyaltySymbol (.down 6))
            (.getEmblem
              [ .static (.conjunction none
                  [ .abilityGrant (allOf creatureYouControl) (keyword "Haste"),
                    .abilityGrant them
                      (activated .tapSymbol
                        (.dealDamage thisCreature (.statOf (.stat .power) thisCreature)
                          (target anyTarget))) ]) ] (agent := .you)) ] } }

/-- Neglected Heirloom // Ashmouth Blade -/
def neglectedHeirloom : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Neglected Heirloom", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 1)) (.up (.lit 1))),
          when
            (.verbedEvent none (.action "Transform")
              (some (.attachHost .equipped (.type .creature))) none)
            (transform thisEquipment),
          keywordCosting "Equip" (.mana [generic 1]) ] } }
  { characteristics :=
    { name := "Ashmouth Blade", types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ .static (.conjunction none
            [ .modification (.attachHost .equipped (.type .creature)) .power (.up (.lit 3)),
              .modification it .toughness (.up (.lit 3)),
              .abilityGrant it (keyword "FirstStrike") ]),
          keywordCosting "Equip" (.mana [generic 3]) ] } }

/-- Harvest Hand // Scrounged Scythe -/
def harvestHand : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Harvest Hand", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Scarecrow"],
      text := [ when (.dies thisCreature) (returnToBattlefieldTransformed it .you) ],
      power := stat 2, toughness := stat 2 } }
  { characteristics :=
    { name := "Scrounged Scythe", types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ .static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 1)) (.up (.lit 1))),
          .static
            (onlyWhile (.abilityGrant (.attachHost .equipped (.type .creature)) (keyword "Menace"))
              (.matches (.attachHost .equipped (.type .creature))
                (.hasSubtype (creatureType "Human")))),
          keywordCosting "Equip" (.mana [generic 2]) ] } }

/-- Cult of the Waxing Moon -/
def cultOfTheWaxingMoon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cult of the Waxing Moon", cost := some [generic 4, pip .green],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Shaman"],
      text :=
        [ whenever
            (.verbedEvent none (.action "Transform")
              (some (a (.and [permanent, .hasPossessor .controller .you])))
              (some (.and [creature, .not (.hasSubtype (creatureType "Human"))])))
            (create (.lit 1) (creatureToken 2 2 [.green] [creatureType "Wolf"])) ],
      power := stat 5, toughness := stat 4 } }

/-- Chittering Host's face, as both melding cards carry it. -/
def chitteringHost : CardFace :=
  { characteristics :=
    { name := "Chittering Host", types := [.creature],
      subtypes := [creatureType "Eldrazi", creatureType "Horror"],
      text :=
        [ keyword "Haste", keyword "Menace",
          when (.enters thisCreature none)
            (.establish
              (.conjunction none
                [ .modification (allOf (otherCreatureYouControl thisCreature)) .power (.up (.lit
                    1)),
                  .modification them .toughness (.up (.lit 0)),
                  .abilityGrant them (keyword "Menace") ])
              (some untilEndOfTurn)) ],
      power := stat 5, toughness := stat 6 } }
def chitteringHostOnScavengers : CardFace := chitteringHost
theorem okChitteringHostOnScavengers : CardFace.check .back chitteringHostOnScavengers = [] := by
  decide
/-- Chittering Host as the GRAF RATS card carries it -/
def chitteringHostOnGrafRats : CardFace := chitteringHost
theorem okChitteringHostOnGrafRats : CardFace.check .back chitteringHostOnGrafRats = [] := by
  decide

/-- Midnight Scavengers // Chittering Host -/
def midnightScavengers : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Midnight Scavengers", cost := some [generic 4, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Rogue"],
      text :=
        [ when (.enters thisCreature none)
            (offer
              (returnTo
                (target (.and [ creature, .inZone (graveyardOf .you),
                                .compare [.stat .manaValue] .atMost (.lit 3) ]))
                hand []) (agent := .you)) ],
      power := stat 3, toughness := stat 3 } }
  chitteringHostOnScavengers

def meldThemInto : Instruction :=
  .sequence
    [ exile
        (allOf (.and [ creature, .hasPossessor .controller .you,
                       .or [ .named (.printed "Graf Rats"),
                             .named (.printed "Midnight Scavengers") ] ])),
      meldInto (themVerbed (.action "Exile")) "Chittering Host" ]
theorem okMeldThemInto : Instruction.check [] meldThemInto = [] := by decide

/-- Profit // Loss -/
def profitLoss : Spelled := spelled <| .split
  { characteristics :=
    { name := "Profit", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ .spell none
            (get (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)),
          keyword "Fuse" ] } }
  { characteristics :=
    { name := "Loss", cost := some [generic 2, pip .black], types := [.instant],
      text :=
        [ .spell none
            (get (allOf creatureYourOpponentsControl) (.down (.lit 1)) (.down (.lit 1))
              (some untilEndOfTurn)),
          keyword "Fuse" ] } }

/-- Glassworks // Shattered Yard -/
def glassworksShatteredYard : Spelled := spelled <| .sharedLineSplit
  { characteristics := { types := [.enchantment], subtypes := [enchantmentType "Room"] } }
  { name := "Glassworks", cost := some [generic 2, pip .red],
    text :=
      [ when (.unlocksDoor .you .thisDoor)
          (.dealDamage thisRoom (.lit 4)
            (target (.and [creature, .hasPossessor .controller anOpponent]))) ] }
  { name := "Shattered Yard", cost := some [generic 4, pip .red],
    text :=
      [ at_ (.beginningOf .the .endStep (.byPlayer .you))
          (.dealDamage thisRoom (.lit 1) (each .opponent)) ] }

/-- Balemurk Leech -/
def balemurkLeech : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Balemurk Leech", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Leech"],
      text :=
        [ abilityWord "eerie"
            (triggeredJoined
              (.enters (a (.and [enchantment, .hasPossessor .controller .you])) none)
              [ joinedHead
                  (.verbedEvent (some .you) (.core .fullyUnlock)
                    (some (a (.hasSubtype (enchantmentType "Room")))) none) ]
              (loseLife (.lit 1) (agent := (each .opponent)))) ],
      power := stat 2, toughness := stat 2 } }

/-- Ghostly Keybearer -/
def ghostlyKeybearer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostly Keybearer", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          whenever (dealsCombatDamage thisCreature (a .anyPlayer))
            (.unlock (.doorOf (some .locked)
              (.described (.target (upTo 1))
                (.and [.hasSubtype (enchantmentType "Room"), .hasPossessor .controller .you])))) ],
      power := stat 3, toughness := stat 3 } }

/-- Riddles in the Dark -/
def riddlesInTheDark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riddles in the Dark", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ lookAt (topSlice (.lit 4)),
              .separateIntoPiles them 2 [.faceDown, .faceUp] (agent := .you),
              choose onePile (agent := some anOpponent),
              move (that .pile) hand,
              move (theOther .pile) graveyard ]) ] } }

/-- Fortune's Favor -/
def fortunesFavor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fortune's Favor", cost := some [generic 3, pip .blue], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .expose .lookAt (.cards (topSlice (.lit 4))) (agent := (target .opponent)),
              .separateIntoPiles them 2 [.faceDown, .faceUp] (agent := they),
              move onePile hand,
              move (theOther .pile) graveyard ]) ] } }

/-- Curator of Destinies -/
def curatorOfDestinies : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curator of Destinies", cost := some [generic 4, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Sphinx"],
      text :=
        [ .static (objectCant (.action "Counter") .this),
          keyword "Flying",
          when (.enters thisCreature none)
            (.sequence
              [ lookAt (topSlice (.lit 5)),
                .separateIntoPiles them 2 [.faceDown, .faceUp] (agent := .you),
                choose onePile (agent := some anOpponent),
                move (that .pile) hand,
                move (theOther .pile) graveyard ]) ],
      power := stat 5, toughness := stat 5 } }

/-- Atris, Oracle of Half-Truths -/
def atrisOracleOfHalfTruths : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Atris, Oracle of Half-Truths", cost := some [generic 2, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Advisor"],
      text :=
        [ keyword "Menace",
          when (.enters thisCreature none)
            (.sequence
              [ .expose .lookAt (.cards (topSlice (.lit 3))) (agent := (target .opponent)),
                .separateIntoPiles them 2 [.faceDown, .faceUp] (agent := they),
                move onePile hand,
                move (theOther .pile) graveyard ]) ],
      power := stat 3, toughness := stat 2 } }

/-- Garruk Relentless // Garruk, the Veil-Cursed -/
def garrukRelentless : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Garruk Relentless", cost := some [generic 3, pip .green],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [planeswalkerType "Garruk"],
      text :=
        [ garrukRelentlessFlip,
          activated (.loyaltySymbol .zero)
            (.sequence
              [ .dealDamage thisPlaneswalker (.lit 3) (target creature),
                .dealDamage (that (.type .creature))
                  (.statOf (.stat .power) (that (.type .creature))) thisPlaneswalker ]),
          activated (.loyaltySymbol .zero)
            (create (.lit 1) (creatureToken 2 2 [.green] [creatureType "Wolf"])) ],
      loyalty := stat 3 } }
  { characteristics :=
    { name := "Garruk, the Veil-Cursed", supertypes := [.legendary], types := [.planeswalker],
      subtypes := [planeswalkerType "Garruk"],
      text :=
        [ activated (.loyaltySymbol (.up 1))
            (create (.lit 1)
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [creatureType "Wolf"],
                  text := [keyword "Deathtouch"], power := stat 1, toughness := stat 1 } }),
          activated (.loyaltySymbol (.down 1))
            (.doIfDone (sacrifice (a creature) (agent := .you))
              (some (.sequence
                [ searchLibraryFor (exactly 1) creature, revealIt, move foundCard hand,
                  shuffle ]))
              none),
          activated (.loyaltySymbol (.down 3))
            (.sequence
              [ .establish
                  (.conjunction none
                    [ .abilityGrant (allOf creatureYouControl) (keyword "Trample"),
                      .modification them .power (.up (.letter .x)),
                      .modification them .toughness (.up (.letter .x)) ])
                  (some untilEndOfTurn),
                .define .x (countOf (.and [creature, .inZone (graveyardOf .you)])) ]) ] } }

/-- Brimstone Mage -/
def brimstoneMage : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Brimstone Mage", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Shaman"],
      text := [levelUp (.mana [generic 3, pip .red])], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 1 2) 2 3
      [activated .tapSymbol (.dealDamage thisCreature (.lit 1) (target anyTarget))],
    levelBand (.atLeast 3) 2 4
      [activated .tapSymbol (.dealDamage thisCreature (.lit 3) (target anyTarget))] ]

/-- Student of Warfare -/
def studentOfWarfare : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Student of Warfare", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text := [levelUp (.mana [pip .white])], power := stat 1, toughness := stat 1 } }
  [ levelBand (.between 2 6) 3 3 [keyword "FirstStrike"],
    levelBand (.atLeast 7) 4 4 [keyword "DoubleStrike"] ]

/-- Kargan Dragonlord -/
def karganDragonlord : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Kargan Dragonlord", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior"],
      text := [levelUp (.mana [pip .red])], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 4 7) 4 4 [keyword "Flying"],
    levelBand (.atLeast 8) 8 8
      [ keyword "Flying", keyword "Trample",
        activated (.mana [pip .red])
          (get thisCreature (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn)) ] ]

/-- Arcane Proxy -/
def arcaneProxy : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Arcane Proxy", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Wizard"],
      text :=
        [ triggeredIf (.enters thisCreature none) (.matches it (castBy .you))
            (.sequence
              [ exile
                  (target (.and [ instantOrSorcery, .isCard,
                                  .compare [.stat .manaValue] .atMost
                                    (.statOf (.stat .power) thisCreature),
                                  .inZone (graveyardOf .you) ])),
                .copy .fromCardZone (that .card) (.lit 1) [] (agent := .you),
                .establish
                  (mayPlayDeed (.action "Cast") .you (that .copy) none
                    (.play none none none false .withoutPaying))
                  none ]) ],
      power := stat 4, toughness := stat 3 } }
  (prototypeAlt [generic 1, pip .blue, pip .blue] 2 1)

/-- Blitz Automaton -/
def blitzAutomaton : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Blitz Automaton", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"], text := [keyword "Haste"],
      power := stat 6, toughness := stat 4 } }
  (prototypeAlt [generic 2, pip .red] 3 2)

/-- Goring Warplow -/
def goringWarplow : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Goring Warplow", cost := some [generic 6], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"], text := [keyword "Deathtouch"],
      power := stat 5, toughness := stat 4 } }
  (prototypeAlt [generic 1, pip .black] 1 1)

end Semantics.Cards
