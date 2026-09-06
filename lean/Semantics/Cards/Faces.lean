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
  Primitives.Instruction.sequence
    [ Primitives.Instruction.setStatus .faceDown (target creature),
      become it
        { characteristics :=
          { types := [.artifact, .creature], subtypes := [creatureType "Cyberman"],
            power := stat 2, toughness := stat 2 } }
        none ]
theorem okCyberConversion : Instruction.check [] cyberConversion = [] := by decide

def breakOpen : Instruction :=
  Primitives.Instruction.setStatus .faceUp
    (target (Primitives.Predicate.and [creature, faceDown, Primitives.Predicate.hasPossessor .controller anOpponent]))
theorem okBreakOpen : Instruction.check [] breakOpen = [] := by decide

def jushiApprentice : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .blue], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.sequence
      [ Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you),
        Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))) .atLeast (.lit 9))
          (Primitives.Instruction.setStatus .flipped thisCreature) none ])
theorem okJushiApprentice : Ability.check [] jushiApprentice = [] := by decide

def invasionOfDominaria : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Invasion of Dominaria", cost := some [generic 2, pip .white],
      types := [.battle], subtypes := [.of .battle "Siege"],
      text :=
        [ when (Primitives.GameEvent.enters thisSiege none)
            (Primitives.Instruction.sequence [gainLife (.lit 4) (agent := Primitives.NounPhrase.you), Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)]) ],
      defense := stat 5 } }
  { characteristics :=
    { name := "Serra Faithkeeper", types := [.creature], subtypes := [creatureType "Angel"],
      text := [keyword "Flying", keyword "Vigilance"], power := stat 4, toughness := stat 4 } }

/-- Missy -/
def missyFaceDownReturn : Ability :=
  whenever (Primitives.GameEvent.dies (a (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact, Primitives.Predicate.otherThan thisCreature])))
    (Primitives.Instruction.sequence
      [ Primitives.Instruction.move it battlefield [Primitives.TokenRider.entersAs .faceDown, Primitives.TokenRider.entersAs .tapped, Primitives.TokenRider.under Primitives.NounPhrase.you],
        Primitives.Instruction.establish
          (Primitives.StaticSpec.qualityChange it .sets
            (Primitives.QualityPayload.bundle
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
        [ whenever (Primitives.GameEvent.dies (a (Primitives.Predicate.and [nontoken, creatureYouControl, Primitives.Predicate.otherThan thisCreature])))
            (offer
              (Primitives.Instruction.sequence
                [ Primitives.Instruction.move it battlefield [Primitives.TokenRider.entersAs .faceDown, Primitives.TokenRider.under (ownerOf it)],
                  Primitives.Instruction.establish
                    (Primitives.StaticSpec.qualityChange it .sets
                      (Primitives.QualityPayload.bundle
                        { characteristics := { types := [.land], subtypes := [landType "Forest"] } }
                        none))
                    none ]) (agent := Primitives.NounPhrase.you)) ],
      power := stat 5, toughness := stat 5 } }

/-- Ral Zarek, Guest Lecturer's ultimate -/
def ralZarekGuestLecturerUltimate : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.flipCoins (Primitives.FlipScope.count (.lit 5)) (agent := Primitives.NounPhrase.you),
      Primitives.Instruction.skipPart .turn (Primitives.Amount.letter .x) (agent := (target Primitives.Predicate.opponent)),
      Primitives.Instruction.define .x (Primitives.Amount.coinsShowing .heads) ]
theorem okRalZarekGuestLecturerUltimate :
    Instruction.check [] ralZarekGuestLecturerUltimate = [] := by decide

def faceDownFlyingCounter : StaticSpec :=
  entersWithCounters (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, faceDown]))
    (.lit 1) flyingCounter
theorem okFaceDownFlyingCounter : StaticSpec.check [] faceDownFlyingCounter = [] := by decide

def goblinArchaeologist : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .red], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.sequence
      [ flipCoins 1 (agent := Primitives.NounPhrase.you),
        doIf (Primitives.Condition.flipCalled Primitives.NounPhrase.you .wins)
          (Primitives.Instruction.sequence [destroy (target artifact), Primitives.Instruction.setStatus .untapped thisCreature]),
        doIf (Primitives.Condition.flipCalled Primitives.NounPhrase.you .loses) (sacrifice thisCreature (agent := Primitives.NounPhrase.you)) ])
theorem okGoblinArchaeologist : Ability.check [] goblinArchaeologist = [] := by decide

def chanceEncounter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chance Encounter", cost := some [generic 2, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.flipsCoin Primitives.NounPhrase.you (some .wins))
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Luck")) thisEnchantment),
          triggeredIf (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Condition.compareAmt (countersOn (.named "Luck") thisEnchantment) .atLeast (.lit 10))
            (Primitives.Instruction.conclude .winGame (agent := Primitives.NounPhrase.you)) ] } }

/-- Karplusan Minotaur's win arm -/
def karplusanMinotaurWinFlip : Ability :=
  whenever (Primitives.GameEvent.flipsCoin Primitives.NounPhrase.you (some .wins)) (Primitives.Instruction.dealDamage thisCreature (.lit 1) (target anyTarget))
theorem okKarplusanMinotaurWinFlip : Ability.check [] karplusanMinotaurWinFlip = [] := by decide

/-- Ral Zarek's ultimate -/
def ralZarekUltimate : Instruction :=
  Primitives.Instruction.sequence [flipCoins 5 (agent := Primitives.NounPhrase.you), Primitives.Instruction.addTurn (Primitives.Amount.coinsShowing .heads) (agent := Primitives.NounPhrase.you)]
theorem okRalZarekUltimate : Instruction.check [] ralZarekUltimate = [] := by decide

/-- Krark's Thumb -/
def krarksThumbExtraFlip : Instruction :=
  replaceEvent (flipsCoin Primitives.NounPhrase.you)
    (Primitives.Instruction.sequence [flipCoins 2 (agent := Primitives.NounPhrase.you), Primitives.Instruction.ignoreOutcomes (Primitives.IgnoredOutcomes.chosen none (.lit 1))]) none
theorem okKrarksThumbExtraFlip : Instruction.check [] krarksThumbExtraFlip = [] := by decide

/-- Goblin Assassin -/
def goblinAssassinCoinTails : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.flipCoins (Primitives.FlipScope.count (.lit 1)) (agent := (each Primitives.Predicate.anyPlayer)),
      sacrifice (aTheirChoice creature) (agent := (each (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.coinCameUp .tails]))) ]
theorem okGoblinAssassinCoinTails : Instruction.check [] goblinAssassinCoinTails = [] := by decide

/-- Rakdos, the Showstopper -/
def rakdosShowstopperFlips : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.flipCoins
        (Primitives.FlipScope.per (each (Primitives.Predicate.and [ creature,
                            Primitives.Predicate.not (Primitives.Predicate.or [ Primitives.Predicate.hasSubtype (creatureType "Demon"),
                                        Primitives.Predicate.hasSubtype (creatureType "Devil"),
                                        Primitives.Predicate.hasSubtype (creatureType "Imp") ]) ]))) (agent := Primitives.NounPhrase.you),
      destroy (each (Primitives.Predicate.and [creature, Primitives.Predicate.coinCameUp .tails])) ]
theorem okRakdosShowstopperFlips : Instruction.check [] rakdosShowstopperFlips = [] := by decide

def merfolkSecretkeeper : Spelled := spelled <| .adventurer
  { characteristics :=
    { name := "Merfolk Secretkeeper", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      power := stat 0, toughness := stat 4 } }
  { characteristics :=
    { name := "Venture Deeper", cost := some [pip .blue], types := [.sorcery],
      subtypes := [spellType "Adventure"],
      text := [ Primitives.Ability.spell none (mill (.lit 4) they (agent := (target Primitives.Predicate.anyPlayer))) ] } }

def orochiEggwatcher : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Orochi Eggwatcher", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Snake", creatureType "Shaman"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .green], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequence
              [ create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Snake"]),
                doIf (Primitives.Condition.compareAmt (countOf creatureYouControl) .atLeast (.lit 10))
                  (Primitives.Instruction.setStatus .flipped thisCreature) ]) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Shidako, Broodmistress", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Snake", creatureType "Shaman"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .green], Primitives.Cost.perform (sacrifice (a creature) (agent :=
            Primitives.NounPhrase.you))])
            (get (target creature) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 3)) (some untilEndOfTurn)) ],
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
    (Primitives.GameEvent.stateHolds
      (Primitives.Condition.matches thisPlaneswalker (Primitives.Predicate.compare [.counter (.named "Loyalty")] .atMost (.lit 2))))
    (transform thisPlaneswalker)
theorem okGarrukRelentlessFlip : Ability.check [] garrukRelentlessFlip = [] := by decide

/-- Mana Clash -/
def manaClashFlip : Instruction :=
  Primitives.Instruction.flipCoins (Primitives.FlipScope.count (.lit 1)) (agent := (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.both Primitives.NounPhrase.you (target Primitives.Predicate.opponent))))
theorem okManaClashFlip : Instruction.check [] manaClashFlip = [] := by decide

def akkiLavarunner : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Akki Lavarunner", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Warrior"],
      text :=
        [ keyword "Haste",
          whenever (Primitives.GameEvent.dealsDamage .any thisCreature (some anOpponent))
            (Primitives.Instruction.setStatus .flipped thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Tok-Tok, Volcano Born", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Shaman"],
      text :=
        [ keywordQuality "Protection" (Primitives.Predicate.colorIs .red),
          Primitives.Ability.static (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (a (Primitives.Predicate.and [source, Primitives.Predicate.colorIs .red])))
            (Primitives.DamageScope.toRecipient (a Primitives.Predicate.anyPlayer)) (Primitives.DamageOp.scale (Primitives.DamageScale.shifted .up (.lit 1))) .repeatedly) ],
      power := stat 2, toughness := stat 2 } }

def bushiTenderfoot : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Bushi Tenderfoot", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ when
            (Primitives.GameEvent.dies (a (Primitives.Predicate.and [ creature,
                              Primitives.Predicate.happenedTo (Primitives.LookbackClause.mk
                                (Primitives.GameEvent.dealsDamage .any thisCreature (some
                                (Primitives.NounPhrase.asMarker .permanent (relative .object))))
                                .thisTurn) ])))
            (Primitives.Instruction.setStatus .flipped thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Kenzo the Hardhearted", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Samurai"],
      text := [keyword "DoubleStrike", keywordNumber "Bushido" (.lit 2)],
      power := stat 3, toughness := stat 4 } }

/-- Kitsune Mystic -/
def kitsuneMysticFlip : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .the .endStep Primitives.HeaderPossessor.noPossessor)
    (Primitives.Condition.matches thisCreature
      (Primitives.Predicate.attachedBy (some .enchanted)
        (counted (atLeast 2) (Primitives.Predicate.hasSubtype (enchantmentType "Aura")))))
    (Primitives.Instruction.setStatus .flipped thisCreature)
theorem okKitsuneMysticFlip : Ability.check [] kitsuneMysticFlip = [] := by decide

/-- Keeper of the Lens -/
def keeperOfTheLens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keeper of the Lens", cost := some [generic 1], types := [.artifact, .creature],
      subtypes := [creatureType "Golem"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .lookAt Primitives.NounPhrase.you
            (Primitives.VisibleThing.objects (allOf (Primitives.Predicate.and [creature, faceDown, Primitives.Predicate.not (Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you)])))) ],
      power := stat 1, toughness := stat 2 } }

/-- Lens of Clarity -/
def lensOfClarity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lens of Clarity", cost := some [generic 1], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.visibility .lookAt Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary,
              Primitives.StaticSpec.visibility .lookAt Primitives.NounPhrase.you
                (Primitives.VisibleThing.objects
                  (allOf (Primitives.Predicate.and [creature, faceDown, Primitives.Predicate.not (Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you)]))) ]) ] } }

def kitsuneMystic : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Kitsune Mystic", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Fox", creatureType "Wizard"],
      text := [kitsuneMysticFlip], power := stat 2, toughness := stat 3 } }
  { characteristics :=
    { name := "Autumn-Tail, Kitsune Sage", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Fox", creatureType "Wizard"],
      text :=
        [ activated (Primitives.Cost.mana [generic 1])
            (Primitives.Instruction.attachTo
              (target (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (enchantmentType "Aura"), Primitives.Predicate.attachedTo (a creature)]))
              (a (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan (that (.type .creature))]))) ],
      power := stat 4, toughness := stat 5 } }

/-- Vesuvan Shapeshifter -/
def vesuvanShapeshifterCopyDuration : Instruction :=
  Primitives.Instruction.establish
    (Primitives.StaticSpec.copyChange thisCreature (that (.type .creature))
      [ Primitives.CopyExcept.ability
          (at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (offer (Primitives.Instruction.setStatus .faceDown thisCreature) (agent := Primitives.NounPhrase.you))) ])
    (some (Primitives.Duration.untilEvent (Primitives.GameEvent.statusEvent thisCreature .faceDown)))
theorem okVesuvanShapeshifterCopyDuration :
    Instruction.check [⟨.a, .one, .object [.creature] (some .battlefield) none none none⟩]
      vesuvanShapeshifterCopyDuration = [] := by
  decide

/-- Arlinn Kord // Arlinn, Embraced by the Moon -/
def arlinnKord : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Arlinn Kord", cost := some [generic 2, pip .red, pip .green],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [planeswalkerType "Arlinn"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 1))
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.conjunction none
                [ Primitives.StaticSpec.modification (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) creature) .power (Primitives.Delta.up (.lit 2)),
                  Primitives.StaticSpec.modification it .toughness (Primitives.Delta.up (.lit 2)),
                  Primitives.StaticSpec.abilityGrant it (keyword "Vigilance"),
                  Primitives.StaticSpec.abilityGrant it (keyword "Haste") ])
              (some untilEndOfTurn)),
          activated (Primitives.Cost.loyaltySymbol .zero)
            (Primitives.Instruction.sequence
              [ create (.lit 1) (creatureToken 2 2 [.green] [creatureType "Wolf"]),
                transform thisPlaneswalker ]) ],
      loyalty := stat 3 } }
  { characteristics :=
    { name := "Arlinn, Embraced by the Moon", supertypes := [.legendary],
      types := [.planeswalker], subtypes := [planeswalkerType "Arlinn"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 1))
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.conjunction none
                [ Primitives.StaticSpec.modification (allOf creatureYouControl) .power (Primitives.Delta.up (.lit 1)),
                  Primitives.StaticSpec.modification them .toughness (Primitives.Delta.up (.lit 1)),
                  Primitives.StaticSpec.abilityGrant them (keyword "Trample") ])
              (some untilEndOfTurn)),
          activated (Primitives.Cost.loyaltySymbol (.down 1))
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 3) (target anyTarget), transform thisPlaneswalker ]),
          activated (Primitives.Cost.loyaltySymbol (.down 6))
            (Primitives.Instruction.getEmblem
              [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
                  [ Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword "Haste"),
                    Primitives.StaticSpec.abilityGrant them
                      (activated Primitives.Cost.tapSymbol
                        (Primitives.Instruction.dealDamage thisCreature (Primitives.Amount.statOf (.stat .power) thisCreature)
                          (target anyTarget))) ]) ] (agent := Primitives.NounPhrase.you)) ] } }

/-- Neglected Heirloom // Ashmouth Blade -/
def neglectedHeirloom : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Neglected Heirloom", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          when
            (Primitives.GameEvent.verbedEvent none (.action "Transform")
              (some (Primitives.NounPhrase.attachHost .equipped (.type .creature))) none none)
            (transform thisEquipment),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 1]) ] } }
  { characteristics :=
    { name := "Ashmouth Blade", types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .power (Primitives.Delta.up (.lit 3)),
              Primitives.StaticSpec.modification it .toughness (Primitives.Delta.up (.lit 3)),
              Primitives.StaticSpec.abilityGrant it (keyword "FirstStrike") ]),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 3]) ] } }

/-- Harvest Hand // Scrounged Scythe -/
def harvestHand : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Harvest Hand", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Scarecrow"],
      text := [ when (Primitives.GameEvent.dies thisCreature) (returnToBattlefieldTransformed it Primitives.NounPhrase.you) ],
      power := stat 2, toughness := stat 2 } }
  { characteristics :=
    { name := "Scrounged Scythe", types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          Primitives.Ability.static
            (onlyWhile (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (keyword "Menace"))
              (Primitives.Condition.matches (Primitives.NounPhrase.attachHost .equipped (.type .creature))
                (Primitives.Predicate.hasSubtype (creatureType "Human")))),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 2]) ] } }

/-- Cult of the Waxing Moon -/
def cultOfTheWaxingMoon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cult of the Waxing Moon", cost := some [generic 4, pip .green],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Shaman"],
      text :=
        [ whenever
            (Primitives.GameEvent.verbedEvent none (.action "Transform")
              (some (a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
              (some (Primitives.Predicate.and [creature, Primitives.Predicate.not
                (Primitives.Predicate.hasSubtype (creatureType "Human"))])) none)
            (create (.lit 1) (creatureToken 2 2 [.green] [creatureType "Wolf"])) ],
      power := stat 5, toughness := stat 4 } }

/-- Chittering Host's face, as both melding cards carry it. -/
def chitteringHost : CardFace :=
  { characteristics :=
    { name := "Chittering Host", types := [.creature],
      subtypes := [creatureType "Eldrazi", creatureType "Horror"],
      text :=
        [ keyword "Haste", keyword "Menace",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.conjunction none
                [ Primitives.StaticSpec.modification (allOf (otherCreatureYouControl thisCreature)) .power (Primitives.Delta.up (.lit
                    1)),
                  Primitives.StaticSpec.modification them .toughness (Primitives.Delta.up (.lit 0)),
                  Primitives.StaticSpec.abilityGrant them (keyword "Menace") ])
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
        [ when (Primitives.GameEvent.enters thisCreature none)
            (offer
              (returnTo
                (target (Primitives.Predicate.and [ creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you),
                                Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 3) ]))
                hand []) (agent := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 3 } }
  chitteringHostOnScavengers

def meldThemInto : Instruction :=
  Primitives.Instruction.sequence
    [ exile
        (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                       Primitives.Predicate.or [ Primitives.Predicate.named (Primitives.NameSource.printed "Graf Rats"),
                             Primitives.Predicate.named (Primitives.NameSource.printed "Midnight Scavengers") ] ])),
      meldInto (themVerbed (.action "Exile")) "Chittering Host" ]
theorem okMeldThemInto : Instruction.check [] meldThemInto = [] := by decide

/-- Profit // Loss -/
def profitLoss : Spelled := spelled <| .split
  { characteristics :=
    { name := "Profit", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (get (allOf creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)),
          keyword "Fuse" ] } }
  { characteristics :=
    { name := "Loss", cost := some [generic 2, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (get (allOf creatureYourOpponentsControl) (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1))
              (some untilEndOfTurn)),
          keyword "Fuse" ] } }

/-- Glassworks // Shattered Yard -/
def glassworksShatteredYard : Spelled := spelled <| .sharedLineSplit
  { characteristics := { types := [.enchantment], subtypes := [enchantmentType "Room"] } }
  { name := "Glassworks", cost := some [generic 2, pip .red],
    text :=
      [ when (Primitives.GameEvent.unlocksDoor Primitives.NounPhrase.you Primitives.Door.thisDoor)
          (Primitives.Instruction.dealDamage thisRoom (.lit 4)
            (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent]))) ] }
  { name := "Shattered Yard", cost := some [generic 4, pip .red],
    text :=
      [ at_ (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
          (Primitives.Instruction.dealDamage thisRoom (.lit 1) (each Primitives.Predicate.opponent)) ] }

/-- Balemurk Leech -/
def balemurkLeech : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Balemurk Leech", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Leech"],
      text :=
        [ abilityWord "eerie"
            (triggeredJoined
              (Primitives.GameEvent.enters (a (Primitives.Predicate.and [enchantment, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) none)
              [ joinedHead
                  (Primitives.GameEvent.verbedEvent (some Primitives.NounPhrase.you) (.core .fullyUnlock)
                    (some (a (Primitives.Predicate.hasSubtype (enchantmentType "Room")))) none none)
                      ]
              (loseLife (.lit 1) (agent := (each Primitives.Predicate.opponent)))) ],
      power := stat 2, toughness := stat 2 } }

/-- Ghostly Keybearer -/
def ghostlyKeybearer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostly Keybearer", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
            (Primitives.Instruction.unlock (Primitives.Door.doorOf (some .locked)
              (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
                (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (enchantmentType "Room"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))) ],
      power := stat 3, toughness := stat 3 } }

/-- Riddles in the Dark -/
def riddlesInTheDark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riddles in the Dark", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ lookAt (topSlice (.lit 4)),
              Primitives.Instruction.separateIntoPiles them 2 [.faceDown, .faceUp] (agent := Primitives.NounPhrase.you),
              choose onePile (agent := some anOpponent),
              move (that .pile) hand,
              move (theOther .pile) graveyard ]) ] } }

/-- Fortune's Favor -/
def fortunesFavor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fortune's Favor", cost := some [generic 3, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.expose .lookAt (Primitives.Exposed.cards (topSlice (.lit 4))) (agent := (target Primitives.Predicate.opponent)),
              Primitives.Instruction.separateIntoPiles them 2 [.faceDown, .faceUp] (agent := they),
              move onePile hand,
              move (theOther .pile) graveyard ]) ] } }

/-- Curator of Destinies -/
def curatorOfDestinies : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curator of Destinies", cost := some [generic 4, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Sphinx"],
      text :=
        [ Primitives.Ability.static (objectCant (.action "Counter") Primitives.NounPhrase.this),
          keyword "Flying",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequence
              [ lookAt (topSlice (.lit 5)),
                Primitives.Instruction.separateIntoPiles them 2 [.faceDown, .faceUp] (agent := Primitives.NounPhrase.you),
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
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.expose .lookAt (Primitives.Exposed.cards (topSlice (.lit 3))) (agent := (target Primitives.Predicate.opponent)),
                Primitives.Instruction.separateIntoPiles them 2 [.faceDown, .faceUp] (agent := they),
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
          activated (Primitives.Cost.loyaltySymbol .zero)
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.dealDamage thisPlaneswalker (.lit 3) (target creature),
                Primitives.Instruction.dealDamage (that (.type .creature))
                  (Primitives.Amount.statOf (.stat .power) (that (.type .creature))) thisPlaneswalker ]),
          activated (Primitives.Cost.loyaltySymbol .zero)
            (create (.lit 1) (creatureToken 2 2 [.green] [creatureType "Wolf"])) ],
      loyalty := stat 3 } }
  { characteristics :=
    { name := "Garruk, the Veil-Cursed", supertypes := [.legendary], types := [.planeswalker],
      subtypes := [planeswalkerType "Garruk"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 1))
            (create (.lit 1)
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [creatureType "Wolf"],
                  text := [keyword "Deathtouch"], power := stat 1, toughness := stat 1 } }),
          activated (Primitives.Cost.loyaltySymbol (.down 1))
            (Primitives.Instruction.doIfDone (sacrifice (a creature) (agent := Primitives.NounPhrase.you))
              (some (Primitives.Instruction.sequence
                [ searchLibraryFor (exactly 1) creature, revealIt, move foundCard hand,
                  shuffle ]))
              none),
          activated (Primitives.Cost.loyaltySymbol (.down 3))
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.establish
                  (Primitives.StaticSpec.conjunction none
                    [ Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword "Trample"),
                      Primitives.StaticSpec.modification them .power (Primitives.Delta.up (Primitives.Amount.letter .x)),
                      Primitives.StaticSpec.modification them .toughness (Primitives.Delta.up (Primitives.Amount.letter .x)) ])
                  (some untilEndOfTurn),
                Primitives.Instruction.define .x (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) ]) ] } }

/-- Brimstone Mage -/
def brimstoneMage : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Brimstone Mage", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Shaman"],
      text := [levelUp (Primitives.Cost.mana [generic 3, pip .red])], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 1 2) 2 3
      [activated Primitives.Cost.tapSymbol (Primitives.Instruction.dealDamage thisCreature (.lit 1) (target anyTarget))],
    levelBand (.atLeast 3) 2 4
      [activated Primitives.Cost.tapSymbol (Primitives.Instruction.dealDamage thisCreature (.lit 3) (target anyTarget))] ]

/-- Student of Warfare -/
def studentOfWarfare : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Student of Warfare", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text := [levelUp (Primitives.Cost.mana [pip .white])], power := stat 1, toughness := stat 1 } }
  [ levelBand (.between 2 6) 3 3 [keyword "FirstStrike"],
    levelBand (.atLeast 7) 4 4 [keyword "DoubleStrike"] ]

/-- Kargan Dragonlord -/
def karganDragonlord : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Kargan Dragonlord", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior"],
      text := [levelUp (Primitives.Cost.mana [pip .red])], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 4 7) 4 4 [keyword "Flying"],
    levelBand (.atLeast 8) 8 8
      [ keyword "Flying", keyword "Trample",
        activated (Primitives.Cost.mana [pip .red])
          (get thisCreature (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ] ]

/-- Arcane Proxy -/
def arcaneProxy : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Arcane Proxy", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Wizard"],
      text :=
        [ triggeredIf (Primitives.GameEvent.enters thisCreature none) (Primitives.Condition.matches it (castBy Primitives.NounPhrase.you))
            (Primitives.Instruction.sequence
              [ exile
                  (target (Primitives.Predicate.and [ instantOrSorcery, Primitives.Predicate.isCard,
                                  Primitives.Predicate.compare [.stat .manaValue] .atMost
                                    (Primitives.Amount.statOf (.stat .power) thisCreature),
                                  Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you) ])),
                Primitives.Instruction.copy .fromCardZone (that .card) (.lit 1) [] (agent := Primitives.NounPhrase.you),
                Primitives.Instruction.establish
                  (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (that .copy) none
                    (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.withoutPaying))
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
