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

/-- "Turn target creature face down. It's a 2/2 Cyberman artifact creature.": the listed
characteristics are the face-down spec [CR#708.2]. -/
def cyberConversion : Action :=
  Primitives.Action.turnFaceDown (target creature)
    { characteristics := some
        { types := [.artifact, .creature], subtypes := [creatureType "Cyberman"],
          power := stat 2, toughness := stat 2 } }
theorem okCyberConversion : Action.check (DRS.ofList []) cyberConversion = [] := by decide

def breakOpen : Action :=
  Primitives.Action.setStatus .faceUp
    (target (Primitives.Predicate.and [creature, faceDown, Primitives.Predicate.hasPossessor .controller anOpponent]))
theorem okBreakOpen : Action.check (DRS.ofList []) breakOpen = [] := by decide

def jushiApprentice : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .blue], Primitives.Cost.tapSymbol])
    (Primitives.Action.sequentially
      [ Primitives.Action.draw (.lit 1),
        Primitives.Action.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])) .atLeast (.lit 9))
          (Primitives.Action.setStatus .flipped thisCreature) none ])
theorem okJushiApprentice : Ability.check (DRS.ofList []) jushiApprentice = [] := by decide

def invasionOfDominaria : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Invasion of Dominaria", cost := some [generic 2, pip .white],
      types := [.battle], subtypes := [.of .battle "Siege"],
      text :=
        [ when (Primitives.GameEvent.enters thisSiege none)
            (Primitives.Action.sequentially [gainLife (.lit 4), Primitives.Action.draw (.lit 1)]) ],
      defense := stat 5 } }
  { characteristics :=
    { name := "Serra Faithkeeper", types := [.creature], subtypes := [creatureType "Angel"],
      text := [keyword (.named "flying"), keyword (.named "vigilance")], power := stat 4, toughness := stat 4 } }

/-- Missy -/
def missyFaceDownReturn : Ability :=
  whenever (Primitives.GameEvent.dies (a (Primitives.Predicate.and [creature, Primitives.Predicate.not artifact, Primitives.Predicate.otherThan thisCreature])))
    (Primitives.Action.sequentially
      [ Primitives.Action.move it Primitives.Location.anyLocation (some battlefield) .arrival none [(Primitives.MoveRider.faceDown
              { characteristics := some
                  { types := [.artifact, .creature], subtypes := [creatureType "Cyberman"],
                    power := stat 2, toughness := stat 2 } }), (Primitives.MoveRider.entersAs .tapped), (Primitives.MoveRider.under Primitives.NounPhrase.you)] ])
theorem okMissyFaceDownReturn : Ability.check (DRS.ofList []) missyFaceDownReturn = [] := by decide

/-- Yedora, Grave Gardener -/
def yedoraGraveGardener : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yedora, Grave Gardener", cost := some [generic 4, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Treefolk", creatureType "Druid"],
      text :=
        [ whenever (Primitives.GameEvent.dies (a (Primitives.Predicate.and [nontoken, creatureYouControl, Primitives.Predicate.otherThan thisCreature])))
            (offer
              (Primitives.Action.sequentially
                [ Primitives.Action.move it Primitives.Location.anyLocation (some battlefield) .arrival none [(Primitives.MoveRider.faceDown
                        { characteristics := some { types := [.land], subtypes := [landType "Forest"] } }), (Primitives.MoveRider.under (ownerOf it))] ])) ],
      power := stat 5, toughness := stat 5 } }

/-- Ral Zarek, Guest Lecturer's ultimate -/
def ralZarekGuestLecturerUltimate : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.flipCoins (Primitives.FlipScope.count (.lit 5)) none none,
      instruct (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (Primitives.Action.skipPart .turn (Primitives.Amount.letter .x)),
      Primitives.Action.define .x (Primitives.Amount.coinsShowing .heads) ]
theorem okRalZarekGuestLecturerUltimate :
    Action.check (DRS.ofList []) ralZarekGuestLecturerUltimate = [] := by decide

def faceDownFlyingCounter : StaticSpec :=
  entersWithCounters (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, faceDown]))
    (.lit 1) flyingCounter
theorem okFaceDownFlyingCounter : StaticSpec.check (DRS.ofList []) faceDownFlyingCounter = [] := by decide

def goblinArchaeologist : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .red], Primitives.Cost.tapSymbol])
    (Primitives.Action.sequentially
      [ flipCoins 1,
        doIf (Primitives.Condition.flipCalled Primitives.NounPhrase.you .wins)
          (Primitives.Action.sequentially [destroy (target artifact), Primitives.Action.setStatus .untapped thisCreature]),
        doIf (Primitives.Condition.flipCalled Primitives.NounPhrase.you .loses) (sacrifice thisCreature) ])
theorem okGoblinArchaeologist : Ability.check (DRS.ofList []) goblinArchaeologist = [] := by decide

def chanceEncounter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chance Encounter", cost := some [generic 2, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.flipCoins (Primitives.FlipScope.count Primitives.Amount.anyAmount) (some .wins) none))
            (Primitives.Action.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "luckCounter")) thisEnchantment .many true),
          triggeredIf (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (Primitives.Condition.compareAmt (countersOn (.named "luckCounter") thisEnchantment) .atLeast (.lit 10))
            (Primitives.Action.conclude .winGame) ] } }

/-- Karplusan Minotaur's win arm -/
def karplusanMinotaurWinFlip : Ability :=
  whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.flipCoins (Primitives.FlipScope.count Primitives.Amount.anyAmount) (some .wins) none)) (instruct thisCreature (Primitives.Action.dealDamage (.lit 1) (target anyTarget) .noncombatOnly))
theorem okKarplusanMinotaurWinFlip : Ability.check (DRS.ofList []) karplusanMinotaurWinFlip = [] := by decide

/-- Ral Zarek's ultimate -/
def ralZarekUltimate : Action :=
  Primitives.Action.sequentially [flipCoins 5, extraTurn (Primitives.Amount.coinsShowing .heads)]
theorem okRalZarekUltimate : Action.check (DRS.ofList []) ralZarekUltimate = [] := by decide

/-- Krark's Thumb -/
def krarksThumbExtraFlip : Action :=
  replaceEvent (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.flipCoins (Primitives.FlipScope.count Primitives.Amount.anyAmount) none none))
    (Primitives.Action.sequentially [flipCoins 2, Primitives.Action.ignoreOutcomes (Primitives.IgnoredOutcomes.chosen none (.lit 1))]) none
theorem okKrarksThumbExtraFlip : Action.check (DRS.ofList []) krarksThumbExtraFlip = [] := by decide

/-- Goblin Assassin -/
def goblinAssassinCoinTails : Action :=
  Primitives.Action.sequentially
    [ instruct (each Primitives.Predicate.anyPlayer) (Primitives.Action.flipCoins (Primitives.FlipScope.count (.lit 1)) none none),
      instruct (each (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.coinCameUp .tails])) (sacrifice (aTheirChoice creature)) ]
theorem okGoblinAssassinCoinTails : Action.check (DRS.ofList []) goblinAssassinCoinTails = [] := by decide

/-- Rakdos, the Showstopper -/
def rakdosShowstopperFlips : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.flipCoins
        (Primitives.FlipScope.per (each (Primitives.Predicate.and [ creature,
                            Primitives.Predicate.not (Primitives.Predicate.or [ Primitives.Predicate.hasSubtype (creatureType "Demon"),
                                        Primitives.Predicate.hasSubtype (creatureType "Devil"),
                                        Primitives.Predicate.hasSubtype (creatureType "Imp") ]) ]))) none none,
      destroy (each (Primitives.Predicate.and [creature, Primitives.Predicate.coinCameUp .tails])) ]
theorem okRakdosShowstopperFlips : Action.check (DRS.ofList []) rakdosShowstopperFlips = [] := by decide

def merfolkSecretkeeper : Spelled := spelled <| .adventurer
  { characteristics :=
    { name := "Merfolk Secretkeeper", cost := some [pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      power := stat 0, toughness := stat 4 } }
  { characteristics :=
    { name := "Venture Deeper", cost := some [pip .blue], types := [.sorcery],
      subtypes := [spellType "Adventure"],
      text := [ Primitives.Ability.spell none (instruct (target Primitives.Predicate.anyPlayer) (mill (.lit 4) they)) ] } }

def orochiEggwatcher : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Orochi Eggwatcher", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Snake", creatureType "Shaman"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .green], Primitives.Cost.tapSymbol])
            (Primitives.Action.sequentially
              [ create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Snake"]),
                doIf (Primitives.Condition.compareAmt (countOf creatureYouControl) .atLeast (.lit 10))
                  (Primitives.Action.setStatus .flipped thisCreature) ]) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Shidako, Broodmistress", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Snake", creatureType "Shaman"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .green], Primitives.Cost.perform (sacrifice (a creature))])
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
      (Primitives.Condition.matches thisPlaneswalker (Primitives.Predicate.compare [.counter (.named "loyaltyCounter")] .atMost (.lit 2))))
    (transform thisPlaneswalker)
theorem okGarrukRelentlessFlip : Ability.check (DRS.ofList []) garrukRelentlessFlip = [] := by decide

/-- Mana Clash -/
def manaClashFlip : Action :=
  instruct (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.both Primitives.NounPhrase.you (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)))) (Primitives.Action.flipCoins (Primitives.FlipScope.count (.lit 1)) none none)
theorem okManaClashFlip : Action.check (DRS.ofList []) manaClashFlip = [] := by decide

def akkiLavarunner : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Akki Lavarunner", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Warrior"],
      text :=
        [ keyword (.named "haste"),
          whenever (Primitives.GameEvent.dealsDamage .any thisCreature (some anOpponent))
            (Primitives.Action.setStatus .flipped thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Tok-Tok, Volcano Born", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Shaman"],
      text :=
        [ keywordQuality (.named "protection") (Primitives.Predicate.colorIs .red),
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
                              happenedTo (Primitives.Subject.actor thisCreature)
                                (Primitives.Action.dealDamage Primitives.Amount.anyAmount
                                  (Primitives.NounPhrase.asMarker .permanent (relative .object)) .any)
                                soFarThisTurn ])))
            (Primitives.Action.setStatus .flipped thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Kenzo the Hardhearted", supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Samurai"],
      text := [keyword (.named "doubleStrike"), keywordNumber (.named "bushido") (.lit 2)],
      power := stat 3, toughness := stat 4 } }

/-- Kitsune Mystic -/
def kitsuneMysticFlip : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .endStep none (Primitives.TurnRef.each none))
    (Primitives.Condition.matches thisCreature
      (Primitives.Predicate.attachedBy (some .enchanted)
        (counted (atLeast 2) (Primitives.Predicate.hasSubtype (enchantmentType "Aura")))))
    (Primitives.Action.setStatus .flipped thisCreature)
theorem okKitsuneMysticFlip : Ability.check (DRS.ofList []) kitsuneMysticFlip = [] := by decide

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
            (Primitives.Action.attachTo
              (target (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (enchantmentType "Aura"), Primitives.Predicate.attachedTo (a creature)]))
              (a (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan (that (.type .creature))]))) ],
      power := stat 4, toughness := stat 5 } }

/-- Vesuvan Shapeshifter -/
def vesuvanShapeshifterCopyDuration : Action :=
  Primitives.Action.establish
    (Primitives.StaticSpec.copyChange thisCreature (that (.type .creature))
      [ Primitives.CopyExcept.ability
          (at_ (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
            (offer (Primitives.Action.setStatus .faceDown thisCreature))) ])
    (some (untilEvent (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.setStatus .faceDown thisCreature))))
theorem okVesuvanShapeshifterCopyDuration :
    Action.check (DRS.ofList [{ det := .a, plur := .one, payload := .object [.creature] (some .battlefield) none none none }])
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
            (Primitives.Action.establish
              (Primitives.StaticSpec.conjunction none
                [ Primitives.StaticSpec.modification (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1)) creature) .power (Primitives.Delta.up (.lit 2)),
                  Primitives.StaticSpec.modification it .toughness (Primitives.Delta.up (.lit 2)),
                  Primitives.StaticSpec.abilityGrant it (keyword (.named "vigilance")),
                  Primitives.StaticSpec.abilityGrant it (keyword (.named "haste")) ])
              (some untilEndOfTurn)),
          activated (Primitives.Cost.loyaltySymbol .zero)
            (Primitives.Action.sequentially
              [ create (.lit 1) (creatureToken 2 2 [.green] [creatureType "Wolf"]),
                transform thisPlaneswalker ]) ],
      loyalty := stat 3 } }
  { characteristics :=
    { name := "Arlinn, Embraced by the Moon", supertypes := [.legendary],
      types := [.planeswalker], subtypes := [planeswalkerType "Arlinn"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 1))
            (Primitives.Action.establish
              (Primitives.StaticSpec.conjunction none
                [ Primitives.StaticSpec.modification (allOf creatureYouControl) .power (Primitives.Delta.up (.lit 1)),
                  Primitives.StaticSpec.modification them .toughness (Primitives.Delta.up (.lit 1)),
                  Primitives.StaticSpec.abilityGrant them (keyword (.named "trample")) ])
              (some untilEndOfTurn)),
          activated (Primitives.Cost.loyaltySymbol (.down 1))
            (Primitives.Action.sequentially
              [ instruct Primitives.NounPhrase.this (Primitives.Action.dealDamage (.lit 3) (target anyTarget) .noncombatOnly), transform thisPlaneswalker ]),
          activated (Primitives.Cost.loyaltySymbol (.down 6))
            (Primitives.Action.getEmblem
              [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
                  [ Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword (.named "haste")),
                    Primitives.StaticSpec.abilityGrant them
                      (activated Primitives.Cost.tapSymbol
                        (instruct thisCreature (Primitives.Action.dealDamage (Primitives.Amount.statOf (.stat .power) thisCreature) (target anyTarget) .noncombatOnly))) ]) ]) ] } }

/-- Neglected Heirloom // Ashmouth Blade -/
def neglectedHeirloom : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Neglected Heirloom", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          when
            (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.turnOver ((Primitives.NounPhrase.attachHost .equipped (.type .creature)))))
            (transform thisEquipment),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 1]) ] } }
  { characteristics :=
    { name := "Ashmouth Blade", types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .power (Primitives.Delta.up (.lit 3)),
              Primitives.StaticSpec.modification it .toughness (Primitives.Delta.up (.lit 3)),
              Primitives.StaticSpec.abilityGrant it (keyword (.named "firstStrike")) ]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 3]) ] } }

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
            (onlyWhile (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (keyword (.named "menace")))
              (Primitives.Condition.matches (Primitives.NounPhrase.attachHost .equipped (.type .creature))
                (Primitives.Predicate.hasSubtype (creatureType "Human")))),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 2]) ] } }

/-- Cult of the Waxing Moon -/
def cultOfTheWaxingMoon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cult of the Waxing Moon", cost := some [generic 4, pip .green],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Shaman"],
      text :=
        [ whenever
            (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.turnOver ((a (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))))
            (create (.lit 1) (creatureToken 2 2 [.green] [creatureType "Wolf"])) ],
      power := stat 5, toughness := stat 4 } }

/-- Chittering Host's face, as both melding cards carry it. -/
def chitteringHost : CardFace :=
  { characteristics :=
    { name := "Chittering Host", types := [.creature],
      subtypes := [creatureType "Eldrazi", creatureType "Horror"],
      text :=
        [ keyword (.named "haste"), keyword (.named "menace"),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Action.establish
              (Primitives.StaticSpec.conjunction none
                [ getsPt (allOf (otherCreatureYouControl thisCreature))
                    (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)),
                  Primitives.StaticSpec.abilityGrant them (keyword (.named "menace")) ])
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
                (target (Primitives.Predicate.and [ creature, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you),
                                Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 3) ]))
                hand [])) ],
      power := stat 3, toughness := stat 3 } }
  chitteringHostOnScavengers

def meldThemInto : Action :=
  Primitives.Action.sequentially
    [ exile
        (allOf (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                       Primitives.Predicate.or [ Primitives.Predicate.named (Primitives.NameSource.printed "Graf Rats"),
                             Primitives.Predicate.named (Primitives.NameSource.printed "Midnight Scavengers") ] ])),
      meldInto (theVerbed (.action "exile") .many) "Chittering Host" ]
theorem okMeldThemInto : Action.check (DRS.ofList []) meldThemInto = [] := by decide

/-- Profit // Loss -/
def profitLoss : Spelled := spelled <| .split
  { characteristics :=
    { name := "Profit", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (get (allOf creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)),
          keyword (.named "fuse") ] } }
  { characteristics :=
    { name := "Loss", cost := some [generic 2, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (get (allOf creatureYourOpponentsControl) (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1))
              (some untilEndOfTurn)),
          keyword (.named "fuse") ] } }

/-- Glassworks // Shattered Yard -/
def glassworksShatteredYard : Spelled := spelled <| .sharedLineSplit
  { characteristics := { types := [.enchantment], subtypes := [enchantmentType "Room"] } }
  { name := "Glassworks", cost := some [generic 2, pip .red],
    text :=
      [ when (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.unlock Primitives.Door.thisDoor))
          (instruct thisRoom (Primitives.Action.dealDamage (.lit 4) (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])) .noncombatOnly)) ] }
  { name := "Shattered Yard", cost := some [generic 4, pip .red],
    text :=
      [ at_ (Primitives.GameEvent.beginningOf .endStep (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none))
          (instruct thisRoom (Primitives.Action.dealDamage (.lit 1) (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) .noncombatOnly)) ] }

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
                  (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.fullyUnlock ((a (Primitives.Predicate.hasSubtype (enchantmentType "Room"))))))
                      ]
              (instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (loseLife (.lit 1)))) ],
      power := stat 2, toughness := stat 2 } }

/-- Ghostly Keybearer -/
def ghostlyKeybearer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostly Keybearer", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword (.named "flying"),
          whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
            (Primitives.Action.unlock (Primitives.Door.doorOf (some .locked)
              (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
                (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (enchantmentType "Room"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))) ],
      power := stat 3, toughness := stat 3 } }

/-- Riddles in the Dark -/
def riddlesInTheDark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riddles in the Dark", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ lookAt (topSlice (.lit 4)),
              Primitives.Action.separateIntoPiles them 2 [.faceDown, .faceUp],
              instruct anOpponent (choose onePile),
              move (that .pile) hand,
              move (theOther .pile) graveyard ]) ] } }

/-- Fortune's Favor -/
def fortunesFavor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fortune's Favor", cost := some [generic 3, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ instruct (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (Primitives.Action.expose .lookAt (Primitives.Exposed.cards (topSlice (.lit 4)))),
              instruct they (Primitives.Action.separateIntoPiles them 2 [.faceDown, .faceUp]),
              move onePile hand,
              move (theOther .pile) graveyard ]) ] } }

/-- Curator of Destinies -/
def curatorOfDestinies : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curator of Destinies", cost := some [generic 4, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Sphinx"],
      text :=
        [ Primitives.Ability.static (objectCant (.action "counter") thisSpell),
          keyword (.named "flying"),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Action.sequentially
              [ lookAt (topSlice (.lit 5)),
                Primitives.Action.separateIntoPiles them 2 [.faceDown, .faceUp],
                instruct anOpponent (choose onePile),
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
        [ keyword (.named "menace"),
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Action.sequentially
              [ instruct (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (Primitives.Action.expose .lookAt (Primitives.Exposed.cards (topSlice (.lit 3)))),
                instruct they (Primitives.Action.separateIntoPiles them 2 [.faceDown, .faceUp]),
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
            (Primitives.Action.sequentially
              [ instruct thisPlaneswalker (Primitives.Action.dealDamage (.lit 3) (target creature) .noncombatOnly),
                instruct (that (.type .creature)) (Primitives.Action.dealDamage (Primitives.Amount.statOf (.stat .power) (that (.type .creature))) thisPlaneswalker .noncombatOnly) ]),
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
                  text := [keyword (.named "deathtouch")], power := stat 1, toughness := stat 1 } }),
          activated (Primitives.Cost.loyaltySymbol (.down 1))
            (Primitives.Action.doIfDone (sacrifice (a creature))
              (some (Primitives.Action.sequentially
                [ searchLibraryFor (exactly 1) creature, revealCards it, move it hand,
                  shuffle ]))
              none),
          activated (Primitives.Cost.loyaltySymbol (.down 3))
            (Primitives.Action.sequentially
              [ Primitives.Action.establish
                  (Primitives.StaticSpec.conjunction none
                    [ Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword (.named "trample")),
                      getsPt them (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (Primitives.Amount.letter .x)) ])
                  (some untilEndOfTurn),
                Primitives.Action.define .x (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])) ]) ] } }

/-- Brimstone Mage -/
def brimstoneMage : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Brimstone Mage", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Shaman"],
      text := [levelUp (Primitives.Cost.mana [generic 3, pip .red])], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 1 2) 2 3
      [activated Primitives.Cost.tapSymbol (instruct thisCreature (Primitives.Action.dealDamage (.lit 1) (target anyTarget) .noncombatOnly))],
    levelBand (.atLeast 3) 2 4
      [activated Primitives.Cost.tapSymbol (instruct thisCreature (Primitives.Action.dealDamage (.lit 3) (target anyTarget) .noncombatOnly))] ]

/-- Student of Warfare -/
def studentOfWarfare : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Student of Warfare", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Knight"],
      text := [levelUp (Primitives.Cost.mana [pip .white])], power := stat 1, toughness := stat 1 } }
  [ levelBand (.between 2 6) 3 3 [keyword (.named "firstStrike")],
    levelBand (.atLeast 7) 4 4 [keyword (.named "doubleStrike")] ]

/-- Kargan Dragonlord -/
def karganDragonlord : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Kargan Dragonlord", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior"],
      text := [levelUp (Primitives.Cost.mana [pip .red])], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 4 7) 4 4 [keyword (.named "flying")],
    levelBand (.atLeast 8) 8 8
      [ keyword (.named "flying"), keyword (.named "trample"),
        activated (Primitives.Cost.mana [pip .red])
          (get thisCreature (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ] ]

/-- Arcane Proxy -/
def arcaneProxy : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Arcane Proxy", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Wizard"],
      text :=
        [ triggeredIf (Primitives.GameEvent.enters thisCreature none) (Primitives.Condition.matches it (castBy Primitives.NounPhrase.you))
            (Primitives.Action.sequentially
              [ exile
                  (target (Primitives.Predicate.and [ instantOrSorcery, Primitives.Predicate.isCard,
                                  Primitives.Predicate.compare [.stat .manaValue] .atMost
                                    (Primitives.Amount.statOf (.stat .power) thisCreature),
                                  Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you) ])),
                Primitives.Action.copy .fromCardZone (that .card) (.lit 1) [],
                Primitives.Action.establish
                  (mayPlayDeed (.action "cast") Primitives.NounPhrase.you (that .copy) none
                    none none none Primitives.PlayPayment.withoutPaying)
                  none ]) ],
      power := stat 4, toughness := stat 3 } }
  (prototypeAlt [generic 1, pip .blue, pip .blue] 2 1)

/-- Blitz Automaton -/
def blitzAutomaton : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Blitz Automaton", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"], text := [keyword (.named "haste")],
      power := stat 6, toughness := stat 4 } }
  (prototypeAlt [generic 2, pip .red] 3 2)

/-- Goring Warplow -/
def goringWarplow : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Goring Warplow", cost := some [generic 6], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"], text := [keyword (.named "deathtouch")],
      power := stat 5, toughness := stat 4 } }
  (prototypeAlt [generic 1, pip .black] 1 1)

end Semantics.Cards
