import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Faces

Port of `idris/src/Experimental/Cards/Faces.idr`: the printed cards of the Faces family (the
frames that carry more than one set: transforming, flip, adventurer, split, room, leveler,
prototype) and the phrase-level bench items beside them, each with an `ok…` theorem.

Not ported: `shapeshifterBox` and `tarmogoyfBox` (a printed `*` box is `none` under the
printed-star ruling; there is no printed-box term to write).
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- "Turn target creature face down. It's a 2/2 Cyberman artifact creature.": the listed
characteristics are the face-down spec [CR#708.2]. -/
def cyberConversion : Action :=
  turnFaceDown (target isCreature)
    { characteristics := some
        { types := [.artifact, .creature], subtypes := [cyberman],
          power := stat 2, toughness := stat 2 } }
theorem okCyberConversion : Action.check (DRS.ofList []) cyberConversion = [] := by decide

def breakOpen : Action :=
  turnFaceUp (target (and [isCreature, isFaceDown, controlledBy anOpponent]))
theorem okBreakOpen : Action.check (DRS.ofList []) breakOpen = [] := by decide

def jushiApprentice : Ability :=
  activated (compound [mana [generic 2, pip .blue], tapSymbol])
    (sequentially
      [ draw (.lit 1),
        doIf (compareAmt (countOf (cardIn yourHand)) .atLeast (.lit 9))
          (flip thisCreature) ])
theorem okJushiApprentice : Ability.check (DRS.ofList []) jushiApprentice = [] := by decide

def invasionOfDominaria : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Invasion of Dominaria", cost := some [generic 2, pip .white],
      types := [.battle], subtypes := [.of .battle "Siege"],
      text :=
        [ when (does thisSiege enter)
            (sequentially [gainLife (.lit 4), draw (.lit 1)]) ],
      defense := stat 5 } }
  { characteristics :=
    { name := "Serra Faithkeeper", types := [.creature], subtypes := [angel],
      text := [flying, vigilance], power := stat 4, toughness := stat 4 } }

/-- Missy -/
def missyFaceDownReturn : Ability :=
  whenever (does (a (and [isCreature, non isArtifact, otherThan thisCreature])) die)
    (sequentially
      [ putInto it battlefield (riders := [(faceDownAs
              { characteristics := some
                  { types := [.artifact, .creature], subtypes := [cyberman],
                    power := stat 2, toughness := stat 2 } }), (entersAs .tapped), (under you)]) ])
theorem okMissyFaceDownReturn : Ability.check (DRS.ofList []) missyFaceDownReturn = [] := by decide

/-- Yedora, Grave Gardener -/
def yedoraGraveGardener : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yedora, Grave Gardener", cost := some [generic 4, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [treefolk, druid],
      text :=
        [ whenever
            (does (a (and [isNontoken, and [isCreature, youControl], otherThan thisCreature])) die)
            (may
              (sequentially
                [ putInto it battlefield (riders := [(faceDownAs
                        { characteristics := some { types := [.land], subtypes := [forest] }
                            }), (under (ownerOf it))]) ])) ],
      power := stat 5, toughness := stat 5 } }

/-- Ral Zarek, Guest Lecturer's ultimate -/
def ralZarekGuestLecturerUltimate : Action :=
  sequentially
    [ flipCoins (.lit 5),
      instruct (target isOpponent) (skipPart .turn (letter .x)),
      define .x (coinsShowing .heads) ]
theorem okRalZarekGuestLecturerUltimate :
    Action.check (DRS.ofList []) ralZarekGuestLecturerUltimate = [] := by decide

def faceDownFlyingCounter : StaticSpec :=
  entersWithCounters (allOf (and [isCreature, youControl, isFaceDown]))
    (.lit 1) flyingCounter
theorem okFaceDownFlyingCounter : StaticSpec.check (DRS.ofList []) faceDownFlyingCounter = [] := by decide

def goblinArchaeologist : Ability :=
  activated (compound [mana [pip .red], tapSymbol])
    (sequentially
      [ flipCoins (.lit 1),
        doIf (flipCalled you .wins)
          (sequentially [destroy (target isArtifact), untap thisCreature]),
        doIf (flipCalled you .loses) (sacrifice thisCreature) ])
theorem okGoblinArchaeologist : Ability.check (DRS.ofList []) goblinArchaeologist = [] := by decide

def chanceEncounter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chance Encounter", cost := some [generic 2, pip .red, pip .red],
      types := [.enchantment],
      text :=
        [ whenever (does you (flipCoins anyAmount (some .wins)))
            (putCounters (.lit 1) (printedKind luckCounter) thisEnchantment .many),
          triggeredIf (beginningOfYour .upkeep)
            (compareAmt (countersOn luckCounter thisEnchantment) .atLeast (.lit 10))
            winGame ] } }

/-- Karplusan Minotaur's win arm -/
def karplusanMinotaurWinFlip : Ability :=
  whenever (does you (flipCoins anyAmount (some .wins)))
      (instruct thisCreature (dealDamage (.lit 1) (target isAnyTarget) .noncombatOnly))
theorem okKarplusanMinotaurWinFlip : Ability.check (DRS.ofList []) karplusanMinotaurWinFlip = [] := by decide

/-- Ral Zarek's ultimate -/
def ralZarekUltimate : Action :=
  sequentially [flipCoins (.lit 5), extraTurn (coinsShowing .heads)]
theorem okRalZarekUltimate : Action.check (DRS.ofList []) ralZarekUltimate = [] := by decide

/-- Krark's Thumb -/
def krarksThumbExtraFlip : Action :=
  replaceEvent (would you flipCoins)
    (sequentially [flipCoins (.lit 2), ignoreOutcomes (chosenResults none (.lit 1))]) none
theorem okKrarksThumbExtraFlip : Action.check (DRS.ofList []) krarksThumbExtraFlip = [] := by decide

/-- Goblin Assassin -/
def goblinAssassinCoinTails : Action :=
  sequentially
    [ instruct eachPlayer (flipCoins (.lit 1)),
      instruct (each (and [isPlayer, coinCameUp .tails])) (sacrifice (aTheirChoice isCreature)) ]
theorem okGoblinAssassinCoinTails : Action.check (DRS.ofList []) goblinAssassinCoinTails = [] := by decide

/-- Rakdos, the Showstopper -/
def rakdosShowstopperFlips : Action :=
  sequentially
    [ flipACoinForEach (each (and [ isCreature,
                            not (or [ hasSubtype demon,
                                        hasSubtype devil,
                                        hasSubtype imp ]) ])),
      destroy (each (and [isCreature, coinCameUp .tails])) ]
theorem okRakdosShowstopperFlips : Action.check (DRS.ofList []) rakdosShowstopperFlips = [] := by decide

def merfolkSecretkeeper : Spelled := spelled <| .adventurer
  { characteristics :=
    { name := "Merfolk Secretkeeper", cost := some [pip .blue], types := [.creature],
      subtypes := [merfolk, wizard],
      power := stat 0, toughness := stat 4 } }
  { characteristics :=
    { name := "Venture Deeper", cost := some [pip .blue], types := [.sorcery],
      subtypes := [adventure],
      text := [ spell (instruct (target isPlayer) (mill (.lit 4))) ] } }

def orochiEggwatcher : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Orochi Eggwatcher", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [snake, shaman],
      text :=
        [ activated (compound [mana [generic 2, pip .green], tapSymbol])
            (sequentially
              [ createToken (creatureToken 1 1 [.green] [snake]),
                doIf (compareAmt (countOf (and [isCreature, youControl])) .atLeast (.lit 10))
                  (flip thisCreature) ]) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Shidako, Broodmistress", supertypes := [.legendary], types := [.creature],
      subtypes := [snake, shaman],
      text :=
        [ activated (compound [mana [pip .green], perform (sacrifice (a isCreature))])
            (get (target isCreature) (up (.lit 3)) (up (.lit 3)) (some untilEndOfTurn)) ],
      power := stat 3, toughness := stat 3 } }

def planeswalkerBackWithoutLoyalty : CardFace :=
  { characteristics :=
    { name := "", supertypes := [.legendary], types := [.planeswalker],
      subtypes := [arlinn] } }
theorem planeswalkerBackWithoutLoyaltyOk :
    CardFace.check .back planeswalkerBackWithoutLoyalty = [] := by decide

/-- Garruk Relentless -/
def garrukRelentlessFlip : Ability :=
  when
    (stateHolds
      (matches_ thisPlaneswalker (compare .atMost (.lit 2) (axes := [.counter loyaltyCounter]))))
    (transform thisPlaneswalker)
theorem okGarrukRelentlessFlip : Ability.check (DRS.ofList []) garrukRelentlessFlip = [] := by decide

/-- Mana Clash -/
def manaClashFlip : Action :=
  instruct (eachOf (youAnd (target isOpponent))) (flipCoins (.lit 1))
theorem okManaClashFlip : Action.check (DRS.ofList []) manaClashFlip = [] := by decide

def akkiLavarunner : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Akki Lavarunner", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [goblin, warrior],
      text :=
        [ haste,
          whenever (does thisCreature (dealDamage (to := anOpponent)))
            (flip thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Tok-Tok, Volcano Born", supertypes := [.legendary], types := [.creature],
      subtypes := [goblin, shaman],
      text :=
        [ protection (hasColor .red),
          static (damageRule .any (dealtBy (a (and [isSource, hasColor .red])))
            (toRecipient (a isPlayer)) (scale (shifted .up (.lit 1))) .repeatedly) ],
      power := stat 2, toughness := stat 2 } }

def bushiTenderfoot : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Bushi Tenderfoot", cost := some [pip .white], types := [.creature],
      subtypes := [human, soldier],
      text :=
        [ when
            (does (a (and [ isCreature,
                              happenedTo thisCreature
                                (dealDamage anyAmount
                                  which)
                                soFarThisTurn ])) die)
            (flip thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Kenzo the Hardhearted", supertypes := [.legendary], types := [.creature],
      subtypes := [human, samurai],
      text := [doubleStrike, bushido (.lit 2)],
      power := stat 3, toughness := stat 4 } }

/-- Kitsune Mystic -/
def kitsuneMysticFlip : Ability :=
  triggeredIf (beginningOfEach .endStep)
    (matches_ thisCreature
      (attachedBy (some .enchanted)
        (counted (atLeast 2) (hasSubtype aura))))
    (flip thisCreature)
theorem okKitsuneMysticFlip : Ability.check (DRS.ofList []) kitsuneMysticFlip = [] := by decide

/-- Keeper of the Lens -/
def keeperOfTheLens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keeper of the Lens", cost := some [generic 1], types := [.artifact, .creature],
      subtypes := [golem],
      text :=
        [ static (visibility .lookAt you
            (objects (allOf (and [isCreature, isFaceDown, youDontControl])))) ],
      power := stat 1, toughness := stat 2 } }

/-- Lens of Clarity -/
def lensOfClarity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lens of Clarity", cost := some [generic 1], types := [.artifact],
      text :=
        [ static (conjunction [ visibility .lookAt you topOfLibrary,
              visibility .lookAt you
                (objects
                  (allOf (and [isCreature, isFaceDown, youDontControl]))) ]) ] } }

def kitsuneMystic : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Kitsune Mystic", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [fox, wizard],
      text := [kitsuneMysticFlip], power := stat 2, toughness := stat 3 } }
  { characteristics :=
    { name := "Autumn-Tail, Kitsune Sage", supertypes := [.legendary], types := [.creature],
      subtypes := [fox, wizard],
      text :=
        [ activated (mana [generic 1])
            (attachTo
              (target (and [hasSubtype aura, attachedTo (a isCreature)]))
              (a (and [isCreature, otherThan (that creature)]))) ],
      power := stat 4, toughness := stat 5 } }

/-- Vesuvan Shapeshifter -/
def vesuvanShapeshifterCopyDuration : Action :=
  establishThroughout (copyChange thisCreature (that creature)
      [ alsoHas
          (at_ (beginningOfYour .upkeep)
            (may (turnFaceDown thisCreature faceDownCreature))) ])
    (untilEvent (does any (turnFaceDown thisCreature faceDownCreature)))
theorem okVesuvanShapeshifterCopyDuration :
    Action.check (DRS.ofList [{ det := .a, plur := .one, payload := .object [.creature] (some .battlefield) none none none }])
      vesuvanShapeshifterCopyDuration = [] := by
  decide

/-- Arlinn Kord // Arlinn, Embraced by the Moon -/
def arlinnKord : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Arlinn Kord", cost := some [generic 2, pip .red, pip .green],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [arlinn],
      text :=
        [ activated (loyaltySymbol (.up 1))
            (establishThroughout
               (conjunction [ modification (targets (upTo 1) isCreature) .power (up (.lit 2)),
                  modification it .toughness (up (.lit 2)),
                  abilityGrant it vigilance,
                  abilityGrant it haste ]) untilEndOfTurn),
          activated (loyaltySymbol .zero)
            (sequentially
              [ createToken (creatureToken 2 2 [.green] [wolf]),
                transform thisPlaneswalker ]) ],
      loyalty := stat 3 } }
  { characteristics :=
    { name := "Arlinn, Embraced by the Moon", supertypes := [.legendary],
      types := [.planeswalker], subtypes := [arlinn],
      text :=
        [ activated (loyaltySymbol (.up 1))
            (establishThroughout
               (conjunction
                  [ modification (allOf (and [isCreature, youControl])) .power (up (.lit 1)),
                  modification them .toughness (up (.lit 1)),
                  abilityGrant them trample ]) untilEndOfTurn),
          activated (loyaltySymbol (.down 1))
            (sequentially
              [ instruct this_ (dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly), transform thisPlaneswalker ]),
          activated (loyaltySymbol (.down 6))
            (getEmblem
              [ static (conjunction [ abilityGrant (allOf (and [isCreature, youControl])) haste,
                    abilityGrant them
                      (activated tapSymbol
                        (instruct thisCreature
                           (dealDamage (powerOf thisCreature) (target isAnyTarget)
                              .noncombatOnly))) ]) ]) ] } }

/-- Neglected Heirloom // Ashmouth Blade -/
def neglectedHeirloom : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Neglected Heirloom", cost := some [generic 1], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up (.lit 1)) (up (.lit 1))),
          when
            (does any (turnOver (equipped creature)))
            (transform thisEquipment),
          equip (mana [generic 1]) ] } }
  { characteristics :=
    { name := "Ashmouth Blade", types := [.artifact], subtypes := [equipment],
      text :=
        [ static (conjunction [ modification (equipped creature) .power (up (.lit 3)),
              modification it .toughness (up (.lit 3)),
              abilityGrant it firstStrike ]),
          equip (mana [generic 3]) ] } }

/-- Harvest Hand // Scrounged Scythe -/
def harvestHand : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Harvest Hand", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [scarecrow],
      text := [ when (does thisCreature die) (returnToBattlefieldTransformed it you) ],
      power := stat 2, toughness := stat 2 } }
  { characteristics :=
    { name := "Scrounged Scythe", types := [.artifact], subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up (.lit 1)) (up (.lit 1))),
          static
            (onlyWhile (abilityGrant (equipped creature) menace)
              (matches_ (equipped creature)
                (hasSubtype human))),
          equip (mana [generic 2]) ] } }

/-- Cult of the Waxing Moon -/
def cultOfTheWaxingMoon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cult of the Waxing Moon", cost := some [generic 4, pip .green],
      types := [.creature], subtypes := [human, shaman],
      text :=
        [ whenever
            (does any (turnOver (a (and [isPermanent, youControl]))))
            (createToken (creatureToken 2 2 [.green] [wolf])) ],
      power := stat 5, toughness := stat 4 } }

/-- Chittering Host's face, as both melding cards carry it. -/
def chitteringHost : CardFace :=
  { characteristics :=
    { name := "Chittering Host", types := [.creature],
      subtypes := [eldrazi, horror],
      text :=
        [ haste, menace,
          when (does thisCreature enter)
            (establishThroughout
               (conjunction
                  [ getsPt (allOf (and [isCreature, youControl, otherThan thisCreature]))
                    (up (.lit 1)) (up (.lit 0)),
                  abilityGrant them menace ]) untilEndOfTurn) ],
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
      subtypes := [human, rogue],
      text :=
        [ when (does thisCreature enter)
            (may
              (returnToHand (target (and [ isCreature, in_ yourGraveyard,
                                hasManaValue .atMost (.lit 3) ])))) ],
      power := stat 3, toughness := stat 3 } }
  chitteringHostOnScavengers

def meldThemInto : Action :=
  sequentially
    [ exile
        (allOf (and [ isCreature, youControl,
                       or [ named (printedName "Graf Rats"),
                             named (printedName "Midnight Scavengers") ] ])),
      meldInto (theVerbed (.action "exile") .many) "Chittering Host" ]
theorem okMeldThemInto : Action.check (DRS.ofList []) meldThemInto = [] := by decide

/-- Profit // Loss -/
def profitLoss : Spelled := spelled <| .split
  { characteristics :=
    { name := "Profit", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ spell (get (allOf (and [isCreature, youControl])) (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn)),
          fuse ] } }
  { characteristics :=
    { name := "Loss", cost := some [generic 2, pip .black], types := [.instant],
      text :=
        [ spell (get (allOf (and [isCreature, opponentsControl])) (down (.lit 1)) (down (.lit 1))
              (some untilEndOfTurn)),
          fuse ] } }

/-- Glassworks // Shattered Yard -/
def glassworksShatteredYard : Spelled := spelled <| .sharedLineSplit
  { characteristics := { types := [.enchantment], subtypes := [room] } }
  { name := "Glassworks", cost := some [generic 2, pip .red],
    text :=
      [ when (does you (unlock thisDoor))
          (instruct thisRoom
             (dealDamage (.lit 4) (target (and [isCreature, controlledBy anOpponent]))
                .noncombatOnly)) ] }
  { name := "Shattered Yard", cost := some [generic 4, pip .red],
    text :=
      [ at_ (beginningOfYour .endStep)
          (instruct thisRoom (dealDamage (.lit 1) eachOpponent .noncombatOnly)) ] }

/-- Balemurk Leech -/
def balemurkLeech : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Balemurk Leech", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [leech],
      text :=
        [ abilityWord "eerie"
            (triggeredJoined
              (does (a (and [isEnchantment, youControl])) enter)
              [ joinedHead
                  (does you (fullyUnlock (a (hasSubtype room))))
                      ]
              (instruct eachOpponent (loseLife (.lit 1)))) ],
      power := stat 2, toughness := stat 2 } }

/-- Ghostly Keybearer -/
def ghostlyKeybearer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostly Keybearer", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [spirit],
      text :=
        [ flying,
          whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := .combatOnly)))
            (unlock (doorOf (some .locked)
              (targets (upTo 1) (and [hasSubtype room, youControl])))) ],
      power := stat 3, toughness := stat 3 } }

/-- Riddles in the Dark -/
def riddlesInTheDark : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riddles in the Dark", cost := some [generic 2, pip .blue], types := [.instant],
      text :=
        [ spell (sequentially
            [ lookAt (topSlice (.lit 4)),
              separateIntoPiles them 2 [.faceDown, .faceUp],
              instruct anOpponent (choose onePile),
              putInto (that .pile) hand,
              putInto (theOther .pile) graveyard ]) ] } }

/-- Fortune's Favor -/
def fortunesFavor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fortune's Favor", cost := some [generic 3, pip .blue], types := [.instant],
      text :=
        [ spell (sequentially
            [ instruct (target isOpponent) (lookAt (topSlice (.lit 4))),
              instruct they (separateIntoPiles them 2 [.faceDown, .faceUp]),
              putInto onePile hand,
              putInto (theOther .pile) graveyard ]) ] } }

/-- Curator of Destinies -/
def curatorOfDestinies : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curator of Destinies", cost := some [generic 4, pip .blue, pip .blue],
      types := [.creature], subtypes := [sphinx],
      text :=
        [ static (cant any (counter thisSpell)),
          flying,
          when (does thisCreature enter)
            (sequentially
              [ lookAt (topSlice (.lit 5)),
                separateIntoPiles them 2 [.faceDown, .faceUp],
                instruct anOpponent (choose onePile),
                putInto (that .pile) hand,
                putInto (theOther .pile) graveyard ]) ],
      power := stat 5, toughness := stat 5 } }

/-- Atris, Oracle of Half-Truths -/
def atrisOracleOfHalfTruths : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Atris, Oracle of Half-Truths", cost := some [generic 2, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, advisor],
      text :=
        [ menace,
          when (does thisCreature enter)
            (sequentially
              [ instruct (target isOpponent) (lookAt (topSlice (.lit 3))),
                instruct they (separateIntoPiles them 2 [.faceDown, .faceUp]),
                putInto onePile hand,
                putInto (theOther .pile) graveyard ]) ],
      power := stat 3, toughness := stat 2 } }

/-- Garruk Relentless // Garruk, the Veil-Cursed -/
def garrukRelentless : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Garruk Relentless", cost := some [generic 3, pip .green],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [garruk],
      text :=
        [ garrukRelentlessFlip,
          activated (loyaltySymbol .zero)
            (sequentially
              [ instruct thisPlaneswalker (dealDamage (.lit 3) (target isCreature) .noncombatOnly),
                instruct (that creature) (dealDamage (powerOf (that creature))
                                                     thisPlaneswalker .noncombatOnly) ]),
          activated (loyaltySymbol .zero)
            (createToken (creatureToken 2 2 [.green] [wolf])) ],
      loyalty := stat 3 } }
  { characteristics :=
    { name := "Garruk, the Veil-Cursed", supertypes := [.legendary], types := [.planeswalker],
      subtypes := [garruk],
      text :=
        [ activated (loyaltySymbol (.up 1))
            (createToken
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [wolf],
                  text := [deathtouch], power := stat 1, toughness := stat 1 } }),
          activated (loyaltySymbol (.down 1))
            (doIfDone (sacrifice (a isCreature))
              (some (sequentially
                [ searchLibraryFor (exactly 1) isCreature, reveal it, putInto it hand,
                  shuffle ]))),
          activated (loyaltySymbol (.down 3))
            (sequentially
              [ establishThroughout
                  (conjunction [ abilityGrant (allOf (and [isCreature, youControl])) trample,
                      getsPt them (up (letter .x)) (up (letter .x)) ]) untilEndOfTurn,
                define .x (countOf (and [isCreature, in_ yourGraveyard])) ]) ] } }

/-- Brimstone Mage -/
def brimstoneMage : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Brimstone Mage", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [human, shaman],
      text := [levelUp (mana [generic 3, pip .red])], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 1 2) 2 3
      [activated tapSymbol (instruct thisCreature (dealDamage (.lit 1) (target isAnyTarget) .noncombatOnly))],
    levelBand (.atLeast 3) 2 4
      [activated tapSymbol (instruct thisCreature (dealDamage (.lit 3) (target isAnyTarget) .noncombatOnly))] ]

/-- Student of Warfare -/
def studentOfWarfare : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Student of Warfare", cost := some [pip .white], types := [.creature],
      subtypes := [human, knight],
      text := [levelUp (mana [pip .white])], power := stat 1, toughness := stat 1 } }
  [ levelBand (.between 2 6) 3 3 [firstStrike],
    levelBand (.atLeast 7) 4 4 [doubleStrike] ]

/-- Kargan Dragonlord -/
def karganDragonlord : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Kargan Dragonlord", cost := some [pip .red, pip .red], types := [.creature],
      subtypes := [human, warrior],
      text := [levelUp (mana [pip .red])], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 4 7) 4 4 [flying],
    levelBand (.atLeast 8) 8 8
      [ flying, trample,
        activated (mana [pip .red])
          (get thisCreature (up (.lit 1)) (up (.lit 0)) (some untilEndOfTurn)) ] ]

/-- Arcane Proxy -/
def arcaneProxy : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Arcane Proxy", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [wizard],
      text :=
        [ triggeredIf (does thisCreature enter) (matches_ it (castBy you))
            (sequentially
              [ exile
                  (target (and [ isInstantOrSorcery, isCard,
                                  hasManaValue .atMost (powerOf thisCreature),
                                  in_ yourGraveyard ])),
                copy .fromCardZone (that card) (.lit 1) [],
                establish
                  (can you (cast (that .copy) (payment := withoutPaying))) ]) ],
      power := stat 4, toughness := stat 3 } }
  (prototypeAlt [generic 1, pip .blue, pip .blue] 2 1)

/-- Blitz Automaton -/
def blitzAutomaton : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Blitz Automaton", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [construct], text := [haste],
      power := stat 6, toughness := stat 4 } }
  (prototypeAlt [generic 2, pip .red] 3 2)

/-- Goring Warplow -/
def goringWarplow : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Goring Warplow", cost := some [generic 6], types := [.artifact, .creature],
      subtypes := [construct], text := [deathtouch],
      power := stat 5, toughness := stat 4 } }
  (prototypeAlt [generic 1, pip .black] 1 1)

end Semantics.Cards
