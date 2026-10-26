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
  turnFaceUp (target (and [isCreature, isFaceDown, anOpponentControls]))
theorem okBreakOpen : Action.check (DRS.ofList []) breakOpen = [] := by decide

def jushiApprentice : Ability :=
  activated (compound [[generic 2, pip .blue], tapSymbol])
    (sequentially
      [ draw 1,
        doIf (compareAmt (countOf (cardIn yourHand)) .atLeast 9)
          (flip thisCreature) ])
theorem okJushiApprentice : Ability.check (DRS.ofList []) jushiApprentice = [] := by decide

def invasionOfDominaria : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Invasion of Dominaria", cost := [generic 2, pip .white],
      types := [.battle], subtypes := [siege],
      text :=
        [ when (does thisSiege enter)
            (sequentially [gainLife 4, draw 1]) ],
      defense := stat 5 } }
  { characteristics :=
    { name := "Serra Faithkeeper", types := [.creature], subtypes := [angel],
      text := [flying, vigilance], power := stat 4, toughness := stat 4 } }

/-- Missy -/
def missyFaceDownReturn : Ability :=
  whenever (does (a (and [isCreature, non isArtifact, isOther])) die)
    (sequentially
      [ returnToBattlefield it [(faceDownAs
              { characteristics := some
                  { types := [.artifact, .creature], subtypes := [cyberman],
                    power := stat 2, toughness := stat 2 } }), (entersAs .tapped), (under you)] ])
theorem okMissyFaceDownReturn : Ability.check (DRS.ofList []) missyFaceDownReturn = [] := by decide

/-- Yedora, Grave Gardener -/
def yedoraGraveGardener : Spelled := spelled <| singleFacedCard
  { name := "Yedora, Grave Gardener", cost := [generic 4, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [treefolk, druid],
    text :=
      [ whenever
          (does (a (and [isNontoken, and [isCreature, youControl], isOther])) die)
          (may
            (sequentially
              [ returnToBattlefield it [(faceDownAs
                      { characteristics := some { types := [.land], subtypes := [forest] }
                          }), (under (ownerOf it))] ])) ],
    power := stat 5, toughness := stat 5 }

/-- Ral Zarek, Guest Lecturer's ultimate -/
def ralZarekGuestLecturerUltimate : Action :=
  sequentially
    [ flipCoins 5,
      instruct (target isOpponent) (skipPart .turn (letter .x)),
      define .x (coinsShowing .heads) ]
theorem okRalZarekGuestLecturerUltimate :
    Action.check (DRS.ofList []) ralZarekGuestLecturerUltimate = [] := by decide

def faceDownFlyingCounter : StaticSpec :=
  entersWithCounters (allOf (and [isCreature, youControl, isFaceDown]))
    1 flyingCounter
theorem okFaceDownFlyingCounter : StaticSpec.check (DRS.ofList []) faceDownFlyingCounter = [] := by decide

def goblinArchaeologist : Ability :=
  activated (compound [[pip .red], tapSymbol])
    (sequentially
      [ flipCoins 1,
        doIf (did you (flipCoins 1 (some .wins)) thisWay)
          (sequentially [destroy (target isArtifact), untap thisCreature]),
        doIf (did you (flipCoins 1 (some .loses)) thisWay) (sacrifice thisCreature) ])
theorem okGoblinArchaeologist : Ability.check (DRS.ofList []) goblinArchaeologist = [] := by decide

def chanceEncounter : Spelled := spelled <| singleFacedCard
  { name := "Chance Encounter", cost := [generic 2, pip .red, pip .red],
    types := [.enchantment],
    text :=
      [ whenever (does you (flipCoins anyAmount (some .wins)))
          (putCounters 1 luckCounter thisEnchantment),
        at_ (beginningOfYour .upkeep)
          (if_ := compareAmt (countersOn luckCounter thisEnchantment) .atLeast 10)
          winGame ] }

/-- Karplusan Minotaur's win arm -/
def karplusanMinotaurWinFlip : Ability :=
  whenever (does you (flipCoins anyAmount (some .wins)))
      (instruct thisCreature (dealDamage 1 (target isAnyTarget)))
theorem okKarplusanMinotaurWinFlip : Ability.check (DRS.ofList []) karplusanMinotaurWinFlip = [] := by decide

/-- Ral Zarek's ultimate -/
def ralZarekUltimate : Action :=
  sequentially [flipCoins 5, extraTurn (coinsShowing .heads)]
theorem okRalZarekUltimate : Action.check (DRS.ofList []) ralZarekUltimate = [] := by decide

/-- Krark's Thumb -/
def krarksThumbExtraFlip : Action :=
  replaceEvent (would you flipCoins)
    (instead (sequentially [flipCoins 2, ignoreOutcomes (chosenResults none 1)])) none
theorem okKrarksThumbExtraFlip : Action.check (DRS.ofList []) krarksThumbExtraFlip = [] := by decide

/-- Goblin Assassin -/
def goblinAssassinCoinTails : Action :=
  sequentially
    [ instruct eachPlayer (flipCoins 1),
      instruct (each (and [isPlayer, coinCameUp .tails])) (sacrifice (aTheirChoice isCreature)) ]
theorem okGoblinAssassinCoinTails : Action.check (DRS.ofList []) goblinAssassinCoinTails = [] := by decide

/-- Rakdos, the Showstopper -/
def rakdosShowstopperFlips : Action :=
  sequentially
    [ flipACoinForEach (each (and [ isCreature,
                            not (or [ demon,
                                        devil,
                                        imp ]) ])),
      destroy (each (and [isCreature, coinCameUp .tails])) ]
theorem okRakdosShowstopperFlips : Action.check (DRS.ofList []) rakdosShowstopperFlips = [] := by decide

def merfolkSecretkeeper : Spelled := spelled <| .adventurer
  { characteristics :=
    { name := "Merfolk Secretkeeper", cost := [pip .blue], types := [.creature],
      subtypes := [merfolk, wizard],
      power := stat 0, toughness := stat 4 } }
  { characteristics :=
    { name := "Venture Deeper", cost := [pip .blue], types := [.sorcery],
      subtypes := [adventure],
      text := [ spell (instruct (target isPlayer) (mill 4)) ] } }

def orochiEggwatcher : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Orochi Eggwatcher", cost := [generic 2, pip .green], types := [.creature],
      subtypes := [snake, shaman],
      text :=
        [ activated (compound [[generic 2, pip .green], tapSymbol])
            (sequentially
              [ create 1 (creatureToken 1 1 [.green] [snake]),
                doIf (compareAmt (countOf (and [isCreature, youControl])) .atLeast 10)
                  (flip thisCreature) ]) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Shidako, Broodmistress", supertypes := [.legendary], types := [.creature],
      subtypes := [snake, shaman],
      text :=
        [ activated (compound [[pip .green], sacrifice (a isCreature)])
            (get (target isCreature) (up 3) (up 3) untilEndOfTurn) ],
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
      (is thisPlaneswalker (compare .atMost 2 (axes := [.counter loyaltyCounter]))))
    (transform thisPlaneswalker)
theorem okGarrukRelentlessFlip : Ability.check (DRS.ofList []) garrukRelentlessFlip = [] := by decide
/-- Garruk Relentless's first 0: "Garruk Relentless deals 3 damage to target creature. That
creature deals damage equal to its power to him." -/
def garrukRelentlessFight : Action :=
  sequentially
    [ instruct thisPlaneswalker (dealDamage 3 (target isCreature)),
      dealDamageOwnPower (that creature) thisPlaneswalker ]
theorem okGarrukRelentlessFight : Action.check (DRS.ofList []) garrukRelentlessFight = [] := by
  decide

/-- Mana Clash -/
def manaClashFlip : Action :=
  instruct (eachOf (youAnd (target isOpponent))) (flipCoins 1)
theorem okManaClashFlip : Action.check (DRS.ofList []) manaClashFlip = [] := by decide

def akkiLavarunner : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Akki Lavarunner", cost := [generic 3, pip .red], types := [.creature],
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
          static
            (replace (would (a (and [isSource, hasColor .red])) (dealDamage anyAmount (a isPlayer)))
              (instead (instruct it (dealDamage (plus thatMuch 1) (that player))))) ],
      power := stat 2, toughness := stat 2 } }

def bushiTenderfoot : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Bushi Tenderfoot", cost := [pip .white], types := [.creature],
      subtypes := [human, soldier],
      text :=
        [ when
            (does (a (and [ isCreature,
                              did thisCreature (dealDamage anyAmount
                                  which) soFarThisTurn ])) die)
            (flip thisCreature) ],
      power := stat 1, toughness := stat 1 } }
  { characteristics :=
    { name := "Kenzo the Hardhearted", supertypes := [.legendary], types := [.creature],
      subtypes := [human, samurai],
      text := [doubleStrike, bushido 2],
      power := stat 3, toughness := stat 4 } }

/-- Kitsune Mystic -/
def kitsuneMysticFlip : Ability :=
  at_ (beginningOfEach .endStep)
    (if_ := is thisCreature
      (attachedBy (some .enchanted)
        (counted (orMore 2) aura)))
    (flip thisCreature)
theorem okKitsuneMysticFlip : Ability.check (DRS.ofList []) kitsuneMysticFlip = [] := by decide

/-- Keeper of the Lens -/
def keeperOfTheLens : Spelled := spelled <| singleFacedCard
  { name := "Keeper of the Lens", cost := [generic 1], types := [.artifact, .creature],
    subtypes := [golem],
    text :=
      [ static (visibility .lookAt you
          (objects (allOf (and [isCreature, isFaceDown, youDontControl])))) ],
    power := stat 1, toughness := stat 2 }

/-- Lens of Clarity -/
def lensOfClarity : Spelled := spelled <| singleFacedCard
  { name := "Lens of Clarity", cost := [generic 1], types := [.artifact],
    text :=
      [ static (conjunction [ visibility .lookAt you topOfLibrary,
            visibility .lookAt you
              (objects
                (allOf (and [isCreature, isFaceDown, youDontControl]))) ]) ] }

def kitsuneMystic : Spelled := spelled <| .flip
  { characteristics :=
    { name := "Kitsune Mystic", cost := [generic 3, pip .white], types := [.creature],
      subtypes := [fox, wizard],
      text := [kitsuneMysticFlip], power := stat 2, toughness := stat 3 } }
  { characteristics :=
    { name := "Autumn-Tail, Kitsune Sage", supertypes := [.legendary], types := [.creature],
      subtypes := [fox, wizard],
      text :=
        [ activated [generic 1]
            (attachTo
              (target (and [aura, attachedTo (a isCreature)]))
              (a (and [isCreature, isOther]))) ],
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

/-- "if a player cast two or more spells last turn": the greatest number of spells any one player
cast last turn is at least two. The count is per player: `aggregateOver .max isPlayer` takes each
player's own `eventCount` (their `they`) and the greatest of them, so two players casting one
spell each does not satisfy it. A `DidClause.ordinal` ("cast a second spell") names no per-player
period (`Period` has none), and a counted patient ("cast two or more spells") counts spells, not
casting events [CR#601.2a]. -/
def aPlayerCastTwoOrMoreLastTurn : Condition :=
  compareAmt
    (aggregateOver .max isPlayer (eventCount (cast (a isSpell)) they lastTurn)) .atLeast 2
theorem okAPlayerCastTwoOrMoreLastTurn :
    Condition.check (DRS.ofList []) aPlayerCastTwoOrMoreLastTurn = [] := by decide

/-- The per-player reading is the aggregate's domain binding: the same count with no domain to
bind `they` finds no player to read. -/
theorem badEventCountWithoutPlayerDomain :
    Condition.check (DRS.ofList [])
      (compareAmt (eventCount (cast (a isSpell)) they lastTurn) .atLeast 2) =
      [.anaphor (.word .player) .one 0] := by decide

/-- A counted patient of a casting refuses: one casting casts one spell [CR#601.2a]. -/
theorem badCountedCastPatientLastTurn :
    Condition.check (DRS.ofList [])
      (did (a isPlayer) (cast (counted (orMore 2) isSpell)) lastTurn) = [.singular] := by decide

/-- Afflicted Deserter // Werewolf Ransacker -/
def afflictedDeserter : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Afflicted Deserter", cost := [generic 3, pip .red], types := [.creature],
      subtypes := [human, werewolf],
      text :=
        [ at_ (beginningOfEach .upkeep)
            (if_ := some (not (did any (cast (a isSpell)) lastTurn)))
            (transform thisCreature) ],
      power := stat 3, toughness := stat 2 } }
  { characteristics :=
    { name := "Werewolf Ransacker", types := [.creature], subtypes := [werewolf],
      text :=
        [ whenever
            (does any (transform thisCreature (into := named "Werewolf Ransacker")))
            (sequentially
              [ may (destroy (target isArtifact)),
                doIf (did (that artifact) (putInto theActor graveyard) thisWay)
                  (instruct thisCreature (dealDamage 3 (controllerOf (that artifact)))) ]),
          at_ (beginningOfEach .upkeep)
            (if_ := aPlayerCastTwoOrMoreLastTurn)
            (transform thisCreature) ],
      power := stat 5, toughness := stat 4 } }
theorem okAfflictedDeserter : afflictedDeserter.card.check = [] := afflictedDeserter.ok

/-- Arlinn Kord // Arlinn, Embraced by the Moon -/
def arlinnKord : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Arlinn Kord", cost := [generic 2, pip .red, pip .green],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [arlinn],
      text :=
        [ activated (loyaltySymbol (up 1))
            (establishFor (targets (upTo 1) isCreature)
              [ getsPt it (up 2) (up 2),
                abilityGrant it vigilance,
                abilityGrant it haste ]
              untilEndOfTurn),
          activated (loyaltySymbol (up 0))
            (sequentially
              [ create 1 (creatureToken 2 2 [.green] [wolf]),
                transform thisPlaneswalker ]) ],
      loyalty := stat 3 } }
  { characteristics :=
    { name := "Arlinn, Embraced by the Moon", supertypes := [.legendary],
      types := [.planeswalker], subtypes := [arlinn],
      text :=
        [ activated (loyaltySymbol (up 1))
            (establishFor (allOf (and [isCreature, youControl]))
              [getsPt them (up 1) (up 1), abilityGrant them trample]
              untilEndOfTurn),
          activated (loyaltySymbol (down 1))
            (sequentially
              [ instruct this_ (dealDamage 3 (target isAnyTarget)), transform thisPlaneswalker ]),
          activated (loyaltySymbol (down 6))
            (getEmblem
              [ static (conjunction [ abilityGrant (allOf (and [isCreature, youControl])) haste,
                    abilityGrant them
                      (activated tapSymbol
                        (dealDamageOwnPower thisCreature (target isAnyTarget))) ]) ]) ] } }

/-- Neglected Heirloom // Ashmouth Blade -/
def neglectedHeirloom : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Neglected Heirloom", cost := [generic 1], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up 1) (up 1)),
          when
            (does any (turnOver (equipped creature)))
            (transform thisEquipment),
          equip [generic 1] ] } }
  { characteristics :=
    { name := "Ashmouth Blade", types := [.artifact], subtypes := [equipment],
      text :=
        [ static (conjunction [ getsPt (equipped creature) (up 3) (up 3),
              abilityGrant it firstStrike ]),
          equip [generic 3] ] } }

/-- Harvest Hand // Scrounged Scythe -/
def harvestHand : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Harvest Hand", cost := [generic 3], types := [.artifact, .creature],
      subtypes := [scarecrow],
      text := [ when (does thisCreature die) (returnToBattlefieldTransformed it you) ],
      power := stat 2, toughness := stat 2 } }
  { characteristics :=
    { name := "Scrounged Scythe", types := [.artifact], subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up 1) (up 1)),
          static
            (onlyWhile (abilityGrant (equipped creature) menace)
              (is (equipped creature)
                human)),
          equip [generic 2] ] } }

/-- Cult of the Waxing Moon -/
def cultOfTheWaxingMoon : Spelled := spelled <| singleFacedCard
  { name := "Cult of the Waxing Moon", cost := [generic 4, pip .green],
    types := [.creature], subtypes := [human, shaman],
    text :=
      [ whenever
          (does any (transform (a (and [isPermanent, youControl]))
            (into := some (and [isCreature, not (hasSubtype human)]))))
          (create 1 (creatureToken 2 2 [.green] [wolf])) ],
    power := stat 5, toughness := stat 4 }

/-- Chittering Host's face, as both melding cards carry it. -/
def chitteringHost : CardFace :=
  { characteristics :=
    { name := "Chittering Host", types := [.creature],
      subtypes := [eldrazi, horror],
      text :=
        [ haste, menace,
          when (does thisCreature enter)
            (establishFor (allOf (and [isCreature, youControl, isOther]))
              [getsPt them (up 1) (up 0), abilityGrant them menace]
              untilEndOfTurn) ],
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
    { name := "Midnight Scavengers", cost := [generic 4, pip .black], types := [.creature],
      subtypes := [human, rogue],
      text :=
        [ when (does thisCreature enter)
            (may
              (returnToHand (target (and [ isCreature, in_ yourGraveyard,
                                hasManaValue .atMost 3 ])) yourGraveyard)) ],
      power := stat 3, toughness := stat 3 } }
  chitteringHostOnScavengers

def meldThemInto : Action :=
  sequentially
    [ exile
        (allOf (and [ isCreature, youControl,
                       or [ named "Graf Rats",
                             named "Midnight Scavengers" ] ])),
      meldInto them "Chittering Host" ]
theorem okMeldThemInto : Action.check (DRS.ofList []) meldThemInto = [] := by decide

/-- Titania, Voice of Gaea: "At the beginning of your upkeep, if there are four or more land cards
in your graveyard and you both own and control Titania, Voice of Gaea and a land named Argoth,
Sanctum of Nature, exile them, then meld them into Titania, Gaea Incarnate." The coordinated
subject of the property test holds an indefinite conjunct, read existentially, and publishes its
group (owner ruling 2026-10-22). -/
def titaniaVoiceOfGaeaMeld : Ability :=
  at_ (beginningOfYour .upkeep)
    (if_ := some (and
      [ thereAre (counted (orMore 4) (and [isLand, cardIn yourGraveyard])),
        is (both thisCreature (a (and [isLand, named "Argoth, Sanctum of Nature"])))
          (and [youOwn, youControl]) ]))
    (sequentially [exile them, meldInto them "Titania, Gaea Incarnate"])
theorem okTitaniaVoiceOfGaeaMeld :
    Ability.check (DRS.ofList []) titaniaVoiceOfGaeaMeld = [] := by decide

/-- In *Titania, Voice of Gaea*, "them" reads the group of Titania and the land named Argoth
(referent 4), not the land cards counted in the graveyard. -/
theorem titaniaThemReadsTitaniaAndArgoth :
    let condition : Condition := and
      [ thereAre (counted (orMore 4) (and [isLand, cardIn yourGraveyard])),
        is (both thisCreature (a (and [isLand, named "Argoth, Sanctum of Nature"])))
          (and [youOwn, youControl]) ]
    let after := nextClause (stepCondition 512 (DRS.ofList []) condition).1
    (resolveCandidates .bare .many after).map (fun ref => (ref.id, ref.groupKey)) =
      [(4, some 4)] := by
  decide

/-- Profit // Loss -/
def profitLoss : Spelled := spelled <| .split
  { characteristics :=
    { name := "Profit", cost := [generic 1, pip .white], types := [.instant],
      text :=
        [ spell (get (allOf (and [isCreature, youControl])) (up 1) (up 1) untilEndOfTurn),
          fuse ] } }
  { characteristics :=
    { name := "Loss", cost := [generic 2, pip .black], types := [.instant],
      text :=
        [ spell (get (allOf (and [isCreature, opponentsControl])) (down 1) (down 1)
              untilEndOfTurn),
          fuse ] } }

/-- Glassworks // Shattered Yard -/
def glassworksShatteredYard : Spelled := spelled <| .sharedLineSplit
  { characteristics := { types := [.enchantment], subtypes := [room] } }
  { name := "Glassworks", cost := [generic 2, pip .red],
    text :=
      [ when (does you (unlock thisDoor))
          (instruct thisRoom
             (dealDamage 4 (target (and [isCreature, anOpponentControls])))) ] }
  { name := "Shattered Yard", cost := [generic 4, pip .red],
    text :=
      [ at_ (beginningOfYour .endStep)
          (instruct thisRoom (dealDamage 1 eachOpponent)) ] }

/-- Balemurk Leech -/
def balemurkLeech : Spelled := spelled <| singleFacedCard
  { name := "Balemurk Leech", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [leech],
    text :=
      [ eerie
          (whenever
            (does (a (and [isEnchantment, youControl])) enter)
            (joins := [ joinedHead
                (does you (fullyUnlock (a room)))
                    ])
            (instruct eachOpponent (loseLife 1))) ],
    power := stat 2, toughness := stat 2 }

/-- Ghostly Keybearer -/
def ghostlyKeybearer : Spelled := spelled <| singleFacedCard
  { name := "Ghostly Keybearer", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [spirit],
    text :=
      [ flying,
        whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (unlock (doorOf (some .locked)
            (targets (upTo 1) (and [room, youControl])))) ],
    power := stat 3, toughness := stat 3 }

/-- Riddles in the Dark -/
def riddlesInTheDark : Spelled := spelled <| singleFacedCard
  { name := "Riddles in the Dark", cost := [generic 2, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [ lookAt (topSlice 4),
            separateIntoPiles them 2 [.faceDown, .faceUp],
            instruct anOpponent (choose onePile),
            putInto (that .pile) hand,
            putInto (theOther .pile) graveyard ]) ] }

/-- Fortune's Favor -/
def fortunesFavor : Spelled := spelled <| singleFacedCard
  { name := "Fortune's Favor", cost := [generic 3, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [ instruct (target isOpponent) (lookAt (topSlice 4)),
            instruct they (separateIntoPiles them 2 [.faceDown, .faceUp]),
            putInto onePile hand,
            putInto (theOther .pile) graveyard ]) ] }

/-- Curator of Destinies -/
def curatorOfDestinies : Spelled := spelled <| singleFacedCard
  { name := "Curator of Destinies", cost := [generic 4, pip .blue, pip .blue],
    types := [.creature], subtypes := [sphinx],
    text :=
      [ static (cant any (counter thisSpell)),
        flying,
        when (does thisCreature enter)
          (sequentially
            [ lookAt (topSlice 5),
              separateIntoPiles them 2 [.faceDown, .faceUp],
              instruct anOpponent (choose onePile),
              putInto (that .pile) hand,
              putInto (theOther .pile) graveyard ]) ],
    power := stat 5, toughness := stat 5 }

/-- Atris, Oracle of Half-Truths -/
def atrisOracleOfHalfTruths : Spelled := spelled <| singleFacedCard
  { name := "Atris, Oracle of Half-Truths", cost := [generic 2, pip .blue, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, advisor],
    text :=
      [ menace,
        when (does thisCreature enter)
          (sequentially
            [ instruct (target isOpponent) (lookAt (topSlice 3)),
              instruct they (separateIntoPiles them 2 [.faceDown, .faceUp]),
              putInto onePile hand,
              putInto (theOther .pile) graveyard ]) ],
    power := stat 3, toughness := stat 2 }

/-- Garruk Relentless // Garruk, the Veil-Cursed -/
def garrukRelentless : Spelled := spelled <| .transforming
  { characteristics :=
    { name := "Garruk Relentless", cost := [generic 3, pip .green],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [garruk],
      text :=
        [ garrukRelentlessFlip,
          activated (loyaltySymbol (up 0)) garrukRelentlessFight,
          activated (loyaltySymbol (up 0))
            (create 1 (creatureToken 2 2 [.green] [wolf])) ],
      loyalty := stat 3 } }
  { characteristics :=
    { name := "Garruk, the Veil-Cursed", supertypes := [.legendary], types := [.planeswalker],
      subtypes := [garruk],
      text :=
        [ activated (loyaltySymbol (up 1))
            (create 1 (written
              { characteristics :=
                { colors := [.black], types := [.creature], subtypes := [wolf],
                  text := [deathtouch], power := stat 1, toughness := stat 1 } })),
          activated (loyaltySymbol (down 1))
            (doIfDone (sacrifice (a isCreature))
              (sequentially
                [ search yourLibrary (a isCreature), reveal it, putInto it hand,
                  shuffle ])),
          activated (loyaltySymbol (down 3))
            (sequentially
              [ establishFor (allOf (and [isCreature, youControl]))
                  [abilityGrant them trample, getsPt them (up (letter .x)) (up (letter .x))]
                  untilEndOfTurn,
                define .x (countOf (and [isCreature, in_ yourGraveyard])) ]) ] } }

/-- Brimstone Mage -/
def brimstoneMage : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Brimstone Mage", cost := [generic 2, pip .red], types := [.creature],
      subtypes := [human, shaman],
      text := [levelUp [generic 3, pip .red]], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 1 2) 2 3
      [activated tapSymbol (instruct thisCreature (dealDamage 1 (target isAnyTarget)))],
    levelBand (.plus 3) 2 4
      [activated tapSymbol (instruct thisCreature (dealDamage 3 (target isAnyTarget)))] ]

/-- Student of Warfare -/
def studentOfWarfare : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Student of Warfare", cost := [pip .white], types := [.creature],
      subtypes := [human, knight],
      text := [levelUp [pip .white]], power := stat 1, toughness := stat 1 } }
  [ levelBand (.between 2 6) 3 3 [firstStrike],
    levelBand (.plus 7) 4 4 [doubleStrike] ]

/-- Kargan Dragonlord -/
def karganDragonlord : Spelled := spelled <| .leveler
  { characteristics :=
    { name := "Kargan Dragonlord", cost := [pip .red, pip .red], types := [.creature],
      subtypes := [human, warrior],
      text := [levelUp [pip .red]], power := stat 2, toughness := stat 2 } }
  [ levelBand (.between 4 7) 4 4 [flying],
    levelBand (.plus 8) 8 8
      [ flying, trample,
        activated [pip .red]
          (get thisCreature (up 1) (up 0) untilEndOfTurn) ] ]

/-- Arcane Proxy -/
def arcaneProxy : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Arcane Proxy", cost := [generic 7], types := [.artifact, .creature],
      subtypes := [wizard],
      text :=
        [ prototype,
          when (does thisCreature enter) (if_ := is it (did you (cast which) thisObject))
            (sequentially
              [ exile
                  (target (and [ isInstantOrSorcery, isCard,
                                  hasManaValue .atMost (powerOf thisCreature),
                                  in_ yourGraveyard ])),
                copy .fromCardZone (that card) 1 [],
                may (cast (that .copy) (payment := withoutPaying)) ]) ],
      power := stat 4, toughness := stat 3 } }
  (prototypeAlt [generic 1, pip .blue, pip .blue] 2 1)

/-- Blitz Automaton -/
def blitzAutomaton : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Blitz Automaton", cost := [generic 7], types := [.artifact, .creature],
      subtypes := [construct], text := [prototype, haste],
      power := stat 6, toughness := stat 4 } }
  (prototypeAlt [generic 2, pip .red] 3 2)

/-- Goring Warplow -/
def goringWarplow : Spelled := spelled <| .prototype
  { characteristics :=
    { name := "Goring Warplow", cost := [generic 6], types := [.artifact, .creature],
      subtypes := [construct], text := [prototype, deathtouch],
      power := stat 5, toughness := stat 4 } }
  (prototypeAlt [generic 1, pip .black] 1 1)

end Semantics.Cards
