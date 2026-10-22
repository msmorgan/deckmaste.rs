import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Keyword

/-!
# Semantics.Cards.Counters

Port of `idris/src/Experimental/Cards/Counters.idr`: the printed cards of the Counters family
and the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Crovax the Cursed -/
def crovaxTheCursed : Action :=
  may (sacrifice (a isCreature))
    (putCounters 1 p1p1Counter thisCreature)
    (removeCounters (exactly 1) (some p1p1Counter) thisCreature true)
theorem okCrovaxTheCursed : Action.check (DRS.ofList []) crovaxTheCursed = [] := by decide
def additiveEvolution : Action :=
  sequentially
    [ createToken (creatureToken 0 0 [.green, .blue] [fractal]),
      putCounters 3 p1p1Counter it ]
theorem okAdditiveEvolution : Action.check (DRS.ofList []) additiveEvolution = [] := by decide
def battlegrowth : Action := putCounters 1 p1p1Counter (target isCreature)
theorem okBattlegrowth : Action.check (DRS.ofList []) battlegrowth = [] := by decide
def chainbreaker : Action :=
  removeCounters (exactly 1) m1m1Counter (target isCreature) true
theorem okChainbreaker : Action.check (DRS.ofList []) chainbreaker = [] := by decide
def kaitoBaneOfNightmares : Action :=
  sequentially
    [tap (target isCreature), putCounters 2 stunCounter it]
theorem okKaitoBaneOfNightmares : Action.check (DRS.ofList []) kaitoBaneOfNightmares = [] := by decide
def jhoiraOfTheGhitu : Ability :=
  activated
    (compound [[generic 2], exile (a (and [non isLand, in_ yourHand]))])
    (putCounters 4 timeCounter (the (and [was (exile which), isCard])))
theorem okJhoiraOfTheGhitu : Ability.check (DRS.ofList []) jhoiraOfTheGhitu = [] := by decide
def alaundoTheSeer : Action :=
  removeCounters (exactly 1) timeCounter (each
                                                                            (and
                                                                               [youOwn, in_
                                                                                  exileZone])) true
theorem okAlaundoTheSeer : Action.check (DRS.ofList []) alaundoTheSeer = [] := by decide

def daydream : Spelled := spelled <| singleFacedCard
  { name := "Daydream", cost := [pip .white], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ exile (target (and [isCreature, youControl])),
            returnToBattlefieldWithCounters (that card) (ownerOf (that card)) 1
              p1p1Counter ]),
        flashback [generic 2, pip .white] ] }

def ashnodsTransmogrant : Ability :=
  activated (compound [tapSymbol, sacrifice thisArtifact])
    (sequentially
      [ putCounters 1 p1p1Counter (target (and [isCreature, non isArtifact])),
        become (that creature) { characteristics := { types := [.artifact] } } ])
theorem okAshnodsTransmogrant : Ability.check (DRS.ofList []) ashnodsTransmogrant = [] := by decide
def azulaAlwaysLies : Action :=
  chooseModes (oneThrough 2)
    [ get (target isCreature) (down 1) (down 1) untilEndOfTurn,
      putCounters 1 p1p1Counter (target isCreature) ]
theorem okAzulaAlwaysLies : Action.check (DRS.ofList []) azulaAlwaysLies = [] := by decide
def bellowingAegisaur : Action :=
  putCounters 1 p1p1Counter
      (each (and [isCreature, youControl, isOther]))
theorem okBellowingAegisaur : Action.check (DRS.ofList []) bellowingAegisaur = [] := by decide
def carnifexDemon : Action :=
  putCounters 1 m1m1Counter (each (and [isCreature, isOther]))
theorem okCarnifexDemon : Action.check (DRS.ofList []) carnifexDemon = [] := by decide
def ajaniAdversaryOfTyrants : Action :=
  putCounters 1 p1p1Counter (eachOf (targets (upTo 2) isCreature))
theorem okAjaniAdversaryOfTyrants : Action.check (DRS.ofList []) ajaniAdversaryOfTyrants = [] := by decide
def naturesPanoply : Action :=
  sequentially
    [ choose (targets anyNumber isCreature),
      putCounters 1 p1p1Counter (eachOf them) ]
theorem okNaturesPanoply : Action.check (DRS.ofList []) naturesPanoply = [] := by decide
def armamentCorps : Action :=
  distributeCounters 2 p1p1Counter
    (targets (oneThrough 2) (and [isCreature, youControl]))
theorem okArmamentCorps : Action.check (DRS.ofList []) armamentCorps = [] := by decide
def savagebornHydra : Ability :=
  activatedOnlyDuring [generic 1, hybridPip .red .green]
    (putCounters 1 p1p1Counter thisCreature) asSorcery
theorem okSavagebornHydra : Ability.check (DRS.ofList []) savagebornHydra = [] := by decide
def woeleecher : Ability :=
  activated (compound [[pip .white], tapSymbol])
    (doIfDone
      (removeCounters (exactly 1) m1m1Counter (target isCreature) true)
      (gainLife 2))
theorem okWoeleecher : Ability.check (DRS.ofList []) woeleecher = [] := by decide
def anointerOfValor : Ability :=
  whenever (does (a isCreature) attack)
    (offerWhen (pay [generic 3] .once)
      (putCounters 1 p1p1Counter (that creature)))
theorem okAnointerOfValor : Ability.check (DRS.ofList []) anointerOfValor = [] := by decide
def avianOddity : Action :=
  putCounters 1 flyingCounter (target (and [isCreature, youControl]))
theorem okAvianOddity : Action.check (DRS.ofList []) avianOddity = [] := by decide
/-- Song of Eärendil -/
def songOfEarendil : Action :=
  putCounters 1 flyingCounter
    (each (and [isCreature, youControl, not (hasKeyword flying)]))
theorem okSongOfEarendil : Action.check (DRS.ofList []) songOfEarendil = [] := by decide
def gideonsAvenger : Ability :=
  whenever (does (a (and [isCreature, anOpponentControls])) (become tapped))
    (putCounters 1 p1p1Counter thisCreature)
theorem okGideonsAvenger : Ability.check (DRS.ofList []) gideonsAvenger = [] := by decide
def prologueToPhyresis : Action :=
  putCounters 1 poison eachOpponent
theorem okPrologueToPhyresis : Action.check (DRS.ofList []) prologueToPhyresis = [] := by decide
def screechingScorchbeast : Ability :=
  whenever (does thisCreature attack) (putCounters 2 radCounter eachPlayer)
theorem okScreechingScorchbeast : Ability.check (DRS.ofList []) screechingScorchbeast = [] := by decide
def merenOfClanNelToth : Ability :=
  whenever (does (a (and [isCreature, youControl, isOther])) die)
    (putCounters 1 experience you)
theorem okMerenOfClanNelToth : Ability.check (DRS.ofList []) merenOfClanNelToth = [] := by decide
def kratosStoicFather : Ability :=
  at_ (beginningOfYour .endStep)
    (putCounters (countersOn experience you) p1p1Counter
      (target isCreature))
theorem okKratosStoicFather : Ability.check (DRS.ofList []) kratosStoicFather = [] := by decide
def vashtaNerada : Ability :=
  at_ (beginningOfPossessed .endStep eachPlayer)
    (if_ := some (did (a isCreature) die soFarThisTurn))
    (putCounters 1 p1p1Counter thisCreature)
theorem okVashtaNerada : Ability.check (DRS.ofList []) vashtaNerada = [] := by decide
def furiousSpinesplitter : Ability :=
  at_ (beginningOfYour .endStep)
    (putCounters (forEach 1 (and
      [isOpponent, was (dealDamage anyAmount which) soFarThisTurn]))
      p1p1Counter thisCreature)
theorem okFuriousSpinesplitter : Ability.check (DRS.ofList []) furiousSpinesplitter = [] := by decide
def paladinOfAtonement : Ability :=
  at_ (beginningOfPossessed .upkeep eachPlayer)
    (if_ := some (did you (loseLife anyAmount) lastTurn))
    (putCounters 1 p1p1Counter thisCreature)
theorem okPaladinOfAtonement : Ability.check (DRS.ofList []) paladinOfAtonement = [] := by decide
def throneWarden : Ability :=
  at_ (beginningOfYour .endStep)
    (if_ := is you (hasDesignation monarch))
    (putCounters 1 p1p1Counter thisCreature)
theorem okThroneWarden : Ability.check (DRS.ofList []) throneWarden = [] := by decide
def bloodTyrant : Ability :=
  whenever (does (a isPlayer) loseGame)
      (putCounters 5 p1p1Counter thisCreature)
theorem okBloodTyrant : Ability.check (DRS.ofList []) bloodTyrant = [] := by decide

def stagBeetle : Spelled := spelled <| singleFacedCard
  { name := "Stag Beetle", cost := [generic 3, pip .green, pip .green], types := [.creature],
    subtypes := [insect],
    text :=
      [ static (conjunction [ entersWithCounters thisCreature (letter .x) p1p1Counter,
            letterDefinition .x (countOf (and [isCreature, isOther])) ]) ],
    power := stat 0, toughness := stat 0 }

def toweringTitan : Ability :=
  static (conjunction [ entersWithCounters thisCreature (letter .x) p1p1Counter,
      letterDefinition .x
        (aggregate .sum (.stat .toughness)
           (and [isCreature, youControl, isOther])) ])
theorem okToweringTitan : Ability.check (DRS.ofList []) toweringTitan = [] := by decide

def coretapper : Spelled := spelled <| singleFacedCard
  { name := "Coretapper", cost := [generic 2], types := [.artifact, .creature],
    subtypes := [myr],
    text :=
      [ activated tapSymbol (putCounters 1 chargeCounter (target isArtifact)),
        activated (sacrifice thisCreature)
          (putCounters 2 chargeCounter (target isArtifact)) ],
    power := stat 1, toughness := stat 1 }

def divineIntervention : Spelled := spelled <| singleFacedCard
  { name := "Divine Intervention", cost := [generic 6, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (entersWithCounters thisEnchantment 2 interventionCounter),
        at_ (beginningOfYour .upkeep)
          (removeCounters (exactly 1) interventionCounter thisEnchantment true),
        when (lastCounterRemovedBy interventionCounter thisEnchantment you) drawGame ] }

def celestialConvergence : Spelled := spelled <| singleFacedCard
  { name := "Celestial Convergence", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (entersWithCounters thisEnchantment 7 omenCounter),
        at_ (beginningOfYour .upkeep)
          (sequentially
            [ removeCounters (exactly 1) omenCounter thisEnchantment true,
              doIf (compareAmt (countersOn omenCounter thisEnchantment) .atMost 0)
                (instruct (the (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer])) winGame),
              doIf (compareAmt
                      (countOf (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer]))
                      .atLeast 2)
                drawGame ]) ] }

def curseOfVengeance : Spelled := spelled <| singleFacedCard
  { name := "Curse of Vengeance", cost := [pip .black], types := [.enchantment],
    subtypes := [aura, curse],
    text :=
      [ enchant isPlayer,
        whenever (does (enchanted player) (cast (a isSpell)))
          (putCounters 1 spiteCounter thisAura),
        when (does (enchanted player) loseGame)
          (sequentially
            [ gainLife (letter .x), draw (letter .x),
              define .x (countersOn spiteCounter thisAura) ]) ] }

def passagewaySeer : Spelled := spelled <| singleFacedCard
  { name := "Passageway Seer", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [tiefling, warlock],
    text :=
      [ lifelink,
        when (does thisCreature enter) (gainDesignation you initiative),
        at_ (beginningOfYour .endStep)
          (if_ := is you (hasDesignation initiative))
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 2, toughness := stat 2 }

def chainsaw : Spelled := spelled <| singleFacedCard
  { name := "Chainsaw", cost := [generic 1, pip .red], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ when (does thisEquipment enter)
          (instruct it (dealDamage 3 (targets (upTo 1) isCreature))),
        whenever (does (counted (orMore 1) isCreature) die)
          (putCounters 1 revCounter thisEquipment),
        static (conjunction [ getsPt (equipped creature) (up (letter .x)) (up 0),
            letterDefinition .x (countersOn revCounter thisEquipment) ]),
        equip [generic 3] ] }

/-- Soul's Might -/
def soulsMight : Spelled := spelled <| singleFacedCard
  { name := "Soul's Might", cost := [generic 4, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ putCounters (letter .x) p1p1Counter (target isCreature),
            define .x (powerOf (that creature)) ]) ] }

def woodlandChampion : Spelled := spelled <| singleFacedCard
  { name := "Woodland Champion", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [elf, scout],
    text :=
      [ whenever (does (counted (orMore 1) (and [isToken, youControl])) enter)
          (putCounters groupSize p1p1Counter thisCreature) ],
    power := stat 2, toughness := stat 2 }

def voraciousBrood : Spelled := spelled <| singleFacedCard
  { name := "Voracious Brood", cost := some [generic 2, pip .green], types := [.creature],
    subtypes := [alien, insect],
    text :=
      [ static (entersWithCounters thisCreature
          (forEach 1 (and [isCreature, in_ yourGraveyard])) p1p1Counter),
        whenever (does any
                    (putInto (counted (orMore 1) (and [isCard, isCreature])) yourGraveyard
                       (from_ := some any)))
          (putCounters groupSize p1p1Counter thisCreature) ],
    power := stat 1, toughness := stat 1 }

def herdBaloth : Spelled := spelled <| singleFacedCard
  { name := "Herd Baloth", cost := [generic 3, pip .green, pip .green], types := [.creature],
    subtypes := [beast],
    text :=
      [ whenever (countersPut p1p1Counter anyAmount thisCreature)
          (may (createToken (creatureToken 4 4 [.green] [beast]))) ],
    power := stat 4, toughness := stat 4 }

def flourishingDefenses : Spelled := spelled <| singleFacedCard
  { name := "Flourishing Defenses", cost := [generic 4, pip .green], types := [.enchantment],
    text :=
      [ whenever (countersPut m1m1Counter 1 (a isCreature))
          (may
            (createToken (creatureToken 1 1 [.green] [elf, warrior]))) ] }

def rakshasaVizier : Spelled := spelled <| singleFacedCard
  { name := "Rakshasa Vizier", cost := [generic 2, pip .black, pip .green, pip .blue],
    types := [.creature], subtypes := [demon],
    text :=
      [ whenever
          (does any
             (putInto (counted (orMore 1) (cardIn yourGraveyard)) exileZone
                (from_ := yourGraveyard)))
          (putCounters groupSize p1p1Counter thisCreature) ],
    power := stat 4, toughness := stat 4 }

def foeLiage : Spelled := spelled <| singleFacedCard
  { name := "Foe-liage", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [plant, mutant],
    text :=
      [ whenever (does (a isLand) enter) (during := duringPart .turn (some you))
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 3, toughness := stat 3 }

def hardenedScales : Spelled := spelled <| singleFacedCard
  { name := "Hardened Scales", cost := [pip .green], types := [.enchantment],
    text :=
      [ static (replace (would any (putCounters anyAmount p1p1Counter (a (and [isCreature, youControl])) false)) (instead (putCounters (plus thatMuch 1) p1p1Counter it))) ] }

def branchingEvolution : Spelled := spelled <| singleFacedCard
  { name := "Branching Evolution", cost := [generic 2, pip .green], types := [.enchantment],
    text :=
      [ static
          (replace
             (would any
                (putCounters anyAmount p1p1Counter
                   (a (and [isCreature, youControl])) false))
          (instead (putCounters (times 2 thatMuch) p1p1Counter (that creature)))) ] }

/-- Corpsejack Menace -/
def corpsejackMenace : Ability :=
  static (replace (would any (putCounters anyAmount p1p1Counter (a (and [isCreature, youControl])) false)) (instead (putCounters (times 2 thatMuch) p1p1Counter it)))
theorem okCorpsejackMenace : Ability.check (DRS.ofList []) corpsejackMenace = [] := by decide

/-- Doubling Season -/
def doublingSeason : Spelled := spelled <| singleFacedCard
  { name := "Doubling Season", cost := [generic 4, pip .green], types := [.enchantment],
    text :=
      [ static (replace (would any (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) []) true you)) (instead (create (times 2 groupSize) asThose []))),
        static (replace
                  (would any
                     (putCounters anyAmount anyKind (a (and [isPermanent, youControl]))))
                  (instead (putCounters (times 2 thatMuch) thoseCounters
                                   (that permanent)))) ] }

/-- Doc Samson, Super Psychiatrist -/
def docSamsonDistributive : Ability :=
  static (replace
            (would you
               (putCounters anyAmount anyKind (a (and [isPermanent, youControl]))
                  false)) (instead (putCounters (plus thatMuch 1) thoseCounters (that permanent))))
theorem okDocSamsonDistributive : Ability.check (DRS.ofList []) docSamsonDistributive = [] := by decide

/-- Winding Constrictor -/
def windingConstrictor : Spelled := spelled <| singleFacedCard
  { name := "Winding Constrictor", cost := [pip .black, pip .green], types := [.creature],
    subtypes := [snake],
    text :=
      [ static
          (replace
             (would any
                (putCounters anyAmount anyKind (a (and [or [isArtifact, isCreature], youControl]))
                   false))
             (instead (putCounters (plus thatMuch 1) thoseCounters
                                    (that permanent)))),
        static (replace (would any (putCounters anyAmount anyKind you false)) (instead (putCounters (plus thatMuch 1) thoseCounters you))) ],
    power := stat 2, toughness := stat 3 }

/-- Aragorn, Company Leader -/
def aragornDistributive : Ability :=
  whenever (manyBareCountersPutBy you thisCreature)
    (putCounters 1 thoseCounters
       (targets (upTo 1) (and [isCreature, isOther])))
theorem okAragornDistributive : Ability.check (DRS.ofList []) aragornDistributive = [] := by decide
/-- Captain Marvel, Apex Avenger -/
def captainMarvelSameKinds : Ability :=
  whenever (manyBareCountersPutBy you (a (and [isCreature, isOther])))
    (if_ := some (not (is it kree)))
    (may (putCounters thatMuch thoseCounters thisCreature))
theorem okCaptainMarvelSameKinds : Ability.check (DRS.ofList []) captainMarvelSameKinds = [] := by decide
/-- Denry Klin, Editor in Chief -/
def denryKlinSameKinds : Ability :=
  whenever (does (a (and [isNontoken, and [isCreature, youControl]])) enter)
    (if_ := is thisCreature hasCounter)
    (putCounters 1 (sameKindAs thisCreature) (that creature))
theorem okDenryKlinSameKinds : Ability.check (DRS.ofList []) denryKlinSameKinds = [] := by decide

/-- Denry Klin, Editor in Chief -/
def denryKlin : Spelled := spelled <| singleFacedCard
  { name := "Denry Klin, Editor in Chief", cost := [generic 2, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [cat, advisor],
    text :=
      [ static (enters thisCreature
          [withCounters 1
            (chosenKind [p1p1Counter, firstStrikeCounter, vigilanceCounter]) .fresh]),
        denryKlinSameKinds ],
    power := stat 2, toughness := stat 2 }

/-- Me, the Immortal -/
def meTheImmortalCounterMenu : Ability :=
  at_ (beginningOfYour .combat)
    (putCounters 1
      (chosenKind [p1p1Counter, firstStrikeCounter, vigilanceCounter, menaceCounter])
      thisCreature)
theorem okMeTheImmortalCounterMenu : Ability.check (DRS.ofList []) meTheImmortalCounterMenu = [] := by decide

/-- Star Pupil -/
def starPupil : Spelled := spelled <| singleFacedCard
  { name := "Star Pupil", cost := [pip .white], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ static (entersWithCounters thisCreature 1 p1p1Counter),
        when (does thisCreature die)
            (putCounters 1 (sameKindAs it) (target (and [isCreature, youControl]))) ],
    power := stat 0, toughness := stat 0 }

/-- Tromell, Seymour's Butler -/
def tromell : Spelled := spelled <| singleFacedCard
  { name := "Tromell, Seymour's Butler", cost := [generic 2, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [elf, advisor],
    text :=
      [ static (entersWithAdditionalCounters
          (each (and [isCreature, isNontoken, youControl, isOther]))
          1 p1p1Counter),
        activated (compound [[generic 1], tapSymbol])
          (sequentially
            [ repeatTimes (letter .x) proliferate,
              define .x
                (countOf (and [ isNontoken, isCreature, youControl,
                                 did which enter soFarThisTurn ])) ]) ],
    power := stat 2, toughness := stat 3 }

/-- Runadi, Behemoth Caller -/
def runadiBehemothCaller : Ability :=
  static (abilityGrant
    (allOf (and [ isCreature, youControl,
                   compare .atLeast 3 (axes := [.counter p1p1Counter]) ]))
    haste)
theorem okRunadiBehemothCaller : Ability.check (DRS.ofList []) runadiBehemothCaller = [] := by decide
/-- Boon of Safety -/
def boonOfSafetyPut : Action :=
  putCounters 1 shieldCounter (target isCreature)
theorem okBoonOfSafetyPut : Action.check (DRS.ofList []) boonOfSafetyPut = [] := by decide
/-- Vivien's Talent and Teferi's Talent -/
def talentLoyaltyPut : Action :=
  putCounters 1 loyaltyCounter (enchanted
                                                                  planeswalker)
theorem okTalentLoyaltyPut : Action.check (DRS.ofList []) talentLoyaltyPut = [] := by decide
/-- Simic Fluxmage -/
def simicFluxmageMove : Action :=
  moveCounters 1 thisCreature (target isCreature) p1p1Counter
theorem okSimicFluxmageMove : Action.check (DRS.ofList []) simicFluxmageMove = [] := by decide
/-- Rikku, Resourceful Guardian -/
def rikkuStealMove : Action :=
  moveCounters 1 (target (and [isCreature, anOpponentControls])) (target
                                                                                (and
                                                                                   [isCreature,
                                                                                    youControl]))
theorem okRikkuStealMove : Action.check (DRS.ofList []) rikkuStealMove = [] := by decide
/-- Littjara Mirrorlake -/
def littjaraMirrorlakeCopy : Action :=
  create 1
    (copyOf (target (and [isCreature, youControl]))
      [withEntryCounters 1 p1p1Counter .additional])
    []
theorem okLittjaraMirrorlakeCopy : Action.check (DRS.ofList []) littjaraMirrorlakeCopy = [] := by decide

def shalaiAndHallar : Spelled := spelled <| singleFacedCard
  { name := "Shalai and Hallar", cost := [generic 1, pip .red, pip .green, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [angel, elf],
    text :=
      [ flying, vigilance,
        whenever (countersPut p1p1Counter anyAmount (a (and [isCreature, youControl])))
          (instruct thisCreature (dealDamage thatMuch (target isOpponent))) ],
    power := stat 3, toughness := stat 3 }

def primalVigor : Spelled := spelled <| singleFacedCard
  { name := "Primal Vigor", cost := [generic 4, pip .green], types := [.enchantment],
    text :=
      [ static (replace (would any (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) []) false)) (instead (create (times 2 groupSize) asThose []))),
        static
            (replace
               (would any
                  (putCounters anyAmount p1p1Counter (a isCreature) false))
            (instead (putCounters (times 2 thatMuch) p1p1Counter (that creature)))) ] }

/-- Patrolling Peacemaker -/
def patrollingPeacemaker : Spelled := spelled <| singleFacedCard
  { name := "Patrolling Peacemaker", cost := [generic 2, pip .white],
    types := [.artifact, .creature], subtypes := [robot, soldier],
    text :=
      [ static (entersWithCounters thisCreature 2 p1p1Counter),
        whenever (does anOpponent commitCrime) proliferate ],
    power := stat 0, toughness := stat 0 }

/-- Galloping Lizrog -/
def gallopingLizrog : Spelled := spelled <| singleFacedCard
  { name := "Galloping Lizrog", cost := [generic 3, pip .green, pip .blue],
    types := [.creature], subtypes := [frog, lizard],
    text :=
      [ trample,
        when (does thisCreature enter)
          (may
            (removeCounters anyNumber p1p1Counter (among (allOf (and [isCreature, youControl]))) true)
            (putCounters (times 2 removedThisWay) p1p1Counter thisCreature)) ],
    power := stat 3, toughness := stat 3 }

/-- Novijen Sages -/
def novijenSagesDraw : Ability :=
  activated
    (compound
      [ [generic 1],
        (removeCounters (exactly 2) p1p1Counter (among (allOf (and [isCreature, youControl]))) true) ])
    (draw 1)
theorem okNovijenSagesDraw : Ability.check (DRS.ofList []) novijenSagesDraw = [] := by decide
/-- Cyclone -/
def cycloneUpkeepPayment : Ability :=
  at_ (beginningOfYour .upkeep)
    (sequentially
      [ putCounters 1 windCounter thisEnchantment,
        doUnless (sacrifice thisEnchantment)
          (scaled [pip .green]
            (times 1 (countersOn windCounter thisEnchantment))) ])
theorem okCycloneUpkeepPayment : Ability.check (DRS.ofList []) cycloneUpkeepPayment = [] := by decide

def stormwildCapridor : Spelled := spelled <| singleFacedCard
  { name := "Stormwild Capridor", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [bird, goat],
    text :=
      [ flying,
        static (damageRule unattributed (toRecipient thisCreature)
          (prevent allDamage
            (putCounters preventedThisWay p1p1Counter thisCreature))
          .repeatedly (some .noncombat)) ],
    power := stat 1, toughness := stat 3 }

def testOfFaith : Spelled := spelled <| singleFacedCard
  { name := "Test of Faith", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout (damageRule unattributed (toRecipient (target isCreature))
            (prevent (theNext 3)
              (some
                 (putCounters preventedThisWay p1p1Counter (that creature))))
            .repeatedly) thisTurn) ] }

def temper : Spelled := spelled <| singleFacedCard
  { name := "Temper", cost := [.variable, generic 1, pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout (damageRule unattributed (toRecipient (target isCreature))
            (prevent (theNext (letter .x))
              (some
                 (putCounters preventedThisWay p1p1Counter (that creature))))
            .repeatedly) thisTurn) ] }

def phytohydra : Spelled := spelled <| singleFacedCard
  { name := "Phytohydra", cost := [generic 2, pip .green, pip .white, pip .white],
    types := [.creature], subtypes := [plant, hydra],
    text :=
      [ static (replace (would any (dealDamage anyAmount thisCreature)) (instead (putCounters thatMuch p1p1Counter it))) ],
    power := stat 1, toughness := stat 1 }

def agelessEntity : Spelled := spelled <| singleFacedCard
  { name := "Ageless Entity", cost := [generic 3, pip .green, pip .green], types := [.creature],
    subtypes := [elemental],
    text :=
      [ whenever (does you (gainLife anyAmount))
          (putCounters thatMuch p1p1Counter thisCreature) ],
    power := stat 4, toughness := stat 4 }

/-- Chromatic Armor -/
def chromaticArmor : Spelled := spelled <| singleFacedCard
  { name := "Chromatic Armor", cost := [generic 1, pip .white, pip .blue],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (entersChoosing thisAura .color),
        static (entersWithCounters thisAura 1 sleightCounter),
        static (damageRule (dealtBy (allOf (and [isSource, ofTheLastChosen .color])))
          (toRecipient (enchanted creature)) (prevent allDamage) .repeatedly),
        activated (mana [.variable])
          (sequentially
            [ putCounters 1 sleightCounter thisAura,
              choose (a (quality .color)),
              define .x (countersOn sleightCounter thisAura) ]) ] }

def vault75MiddleSchool : Spelled := spelled <| singleFacedCard
  { name := "Vault 75: Middle School", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment], subtypes := [saga],
    text :=
      [ when (chapterMark [1])
          (exile (allOf (and [isCreature, hasPower .atLeast 4]))),
        when (chapterMark [2, 3])
          (putCounters 1 p1p1Counter (each (and [isCreature, youControl]))) ] }

def keldonWarcaller : Spelled := spelled <| singleFacedCard
  { name := "Keldon Warcaller", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [human, warrior],
    text :=
      [ whenever (does thisCreature attack)
          (putCounters 1 loreCounter
            (target (and [saga, youControl]))) ],
    power := stat 2, toughness := stat 2 }

def magistratesScepter : Spelled := spelled <| singleFacedCard
  { name := "Magistrate's Scepter", cost := [generic 3], types := [.artifact],
    text :=
      [ activated (compound [[generic 4], tapSymbol])
          (putCounters 1 chargeCounter thisArtifact),
        activated
          (compound
            [ tapSymbol,
              (removeCounters (exactly 3) chargeCounter thisArtifact true) ])
          (extraTurn 1) ] }

def grumgully : Spelled := spelled <| singleFacedCard
  { name := "Grumgully, the Generous", cost := [generic 1, pip .red, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [goblin, shaman],
    text :=
      [ static (entersWithAdditionalCounters
          (each (and [ isCreature, youControl,
                        non human, isOther ]))
          1 p1p1Counter) ],
    power := stat 3, toughness := stat 3 }

/-- Metallic Mimic. "This creature is the chosen type in addition to its other types" is a
Characteristic-Defining Ability: it depends on the choice made as it enters, as Shapeshifter's
power does, and depending on a choice is no condition [CR#604.3a]. Its counter on the other
creatures of the chosen type as they enter stays a separate ability, affecting other objects
[CR#604.3a]. -/
def metallicMimic : Spelled := spelled <| singleFacedCard
  { name := "Metallic Mimic", cost := [generic 2], types := [.artifact, .creature],
    subtypes := [shapeshifter],
    text :=
      [ static (entersChoosing thisCreature (.subtype .creature)),
        static (qualityDefinition thisCreature .adds (chosenQuality (ofChosen (.subtype
            .creature)))),
        static (entersWithAdditionalCounters
          (each (and [ isCreature, youControl, ofChosen (.subtype .creature),
                        isOther ]))
          1 p1p1Counter) ],
    power := stat 2, toughness := stat 1 }

def curatorBeastieLine : StaticSpec :=
  entersWithAdditionalCounters (allOf (and [isCreature, youControl, isColorless]))
    2 p1p1Counter
theorem okCuratorBeastieLine : StaticSpec.check (DRS.ofList []) curatorBeastieLine = [] := by decide
def renataLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (and [isCreature, youControl, isOther])) 1
    p1p1Counter
theorem okRenataLine : StaticSpec.check (DRS.ofList []) renataLine = [] := by decide
def entDraughtBasinAbility : Ability :=
  activatedOnlyDuring (compound [mana [.variable], tapSymbol])
    (putCounters 1 p1p1Counter
      (target (and [isCreature, hasPower .eq (letter .x)])))
    asSorcery
theorem okEntDraughtBasinAbility : Ability.check (DRS.ofList []) entDraughtBasinAbility = [] := by decide

def matopiGolem : Spelled := spelled <| singleFacedCard
  { name := "Matopi Golem", cost := [generic 5], types := [.artifact, .creature],
    subtypes := [golem],
    text :=
      [ activated [generic 1]
          (triggerThisWay (regenerate thisCreature) (does any (regeneration thisCreature))
            (putCounters 1 m1m1Counter it)) ],
    power := stat 3, toughness := stat 3 }

def bramblewoodParagon : Spelled := spelled <| singleFacedCard
  { name := "Bramblewood Paragon", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [elf, warrior],
    text :=
      [ static (entersWithAdditionalCounters
          (each (and [ isCreature, warrior, youControl,
                        isOther ]))
          1 p1p1Counter),
        static (abilityGrant
          (each (and [isCreature, youControl, hasCounter p1p1Counter]))
          trample) ],
    power := stat 2, toughness := stat 2 }

def oonasBlackguard : Spelled := spelled <| singleFacedCard
  { name := "Oona's Blackguard", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [faerie, rogue],
    text :=
      [ flying,
        static (entersWithAdditionalCounters
          (each (and [ isCreature, rogue, youControl,
                        isOther ]))
          1 p1p1Counter),
        whenever
          (does (a (and [isCreature, youControl, hasCounter p1p1Counter]))
             (dealDamage (to := a isPlayer) (kind := some .combat)))
          (discard (a (cardIn (handOf (that player))))) ],
    power := stat 1, toughness := stat 1 }

def crumblingAshes : Spelled := spelled <| singleFacedCard
  { name := "Crumbling Ashes", cost := [generic 1, pip .black], types := [.enchantment],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (destroy (target (and [isCreature, hasCounter m1m1Counter]))) ] }

def hunterOfEyeblights : Spelled := spelled <| singleFacedCard
  { name := "Hunter of Eyeblights", cost := [generic 3, pip .black, pip .black],
    types := [.creature], subtypes := [elf, assassin],
    text :=
      [ when (does thisCreature enter)
          (putCounters 1 p1p1Counter (target (and [isCreature, youDontControl]))),
        activated (compound [[generic 2, pip .black], tapSymbol])
          (destroy (target (and [isCreature, hasCounter]))) ],
    power := stat 3, toughness := stat 3 }

def pridemalkin : Spelled := spelled <| singleFacedCard
  { name := "Pridemalkin", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [cat],
    text :=
      [ when (does thisCreature enter)
          (putCounters 1 p1p1Counter (target (and [isCreature, youControl]))),
        static (abilityGrant
          (each (and [isCreature, youControl, hasCounter p1p1Counter]))
          trample) ],
    power := stat 2, toughness := stat 1 }

def aboroth : Spelled := spelled <| singleFacedCard
  { name := "Aboroth", cost := [generic 4, pip .green, pip .green], types := [.creature],
    subtypes := [elemental],
    text :=
      [ cumulativeUpkeep (putCounters 1 m1m1Counter thisCreature) ],
    power := stat 9, toughness := stat 9 }

def shelteringAncient : Spelled := spelled <| singleFacedCard
  { name := "Sheltering Ancient", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [treefolk],
    text :=
      [ trample,
        cumulativeUpkeep
          (putCounters 1 p1p1Counter
            (a (and [isCreature, anOpponentControls]))) ],
    power := stat 5, toughness := stat 5 }

def coverOfWinterPut : Ability :=
  activated (mana [.snow]) (putCounters 1 ageCounter thisEnchantment)
theorem okCoverOfWinterPut : Ability.check (DRS.ofList []) coverOfWinterPut = [] := by decide

def urborgScavengers : Spelled := spelled <| singleFacedCard
  { name := "Urborg Scavengers", cost := [generic 2, pip .black], types := [.creature],
    subtypes := [spirit],
    text :=
      [ whenever (does thisCreature enter) (or_ := [does thisCreature attack])
          (sequentially
            [ exile (target (cardIn graveyard)),
              putCounters 1 p1p1Counter thisCreature ]),
        alsoForKeywords
          (static (onlyWhile (abilityGrant thisCreature flying)
            (exists_ (and [in_ (exiledWith thisCreature), hasKeyword flying]))))
          (["firstStrike", "doubleStrike", "deathtouch", "haste", "hexproof", "indestructible",
            "lifelink", "menace", "reach", "trample", "vigilance"].map (fun k => .the (.named k))) ],
    power := stat 2, toughness := stat 2 }

def sengirVampire : Spelled := spelled <| singleFacedCard
  { name := "Sengir Vampire", cost := [generic 3, pip .black, pip .black], types := [.creature],
    subtypes := [vampire],
    text :=
      [ flying,
        whenever
          (does (a (and [ isCreature,
                            did thisCreature (dealDamage anyAmount which) soFarThisTurn ])) die)
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 4, toughness := stat 4 }

def mildManneredLibrarian : Spelled := spelled <| singleFacedCard
  { name := "Mild-Mannered Librarian", cost := [pip .green], types := [.creature],
    subtypes := [human],
    text :=
      [ activatedOnlyOnce [generic 3, pip .green]
          (sequentially
            [ establish
                (qualityChange thisCreature .sets
                  (bundle { characteristics := { subtypes := [werewolf] } })),
              putCounters 2 p1p1Counter it,
              draw 1 ])
          oncePerGame ],
    power := stat 1, toughness := stat 1 }

def infernalVessel : Spelled := spelled <| singleFacedCard
  { name := "Infernal Vessel", cost := some [generic 2, pip .black], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ when (does thisCreature die) (if_ := some (not (is it demon)))
          (sequentially
            [ returnToBattlefieldWithCounters it (ownerOf it) 2 p1p1Counter,
              become it { characteristics := { subtypes := [demon] } } ]) ],
    power := stat 2, toughness := stat 1 }

/-- Vivien's Talent -/
def viviensTalentTrigger : Ability :=
  whenever (does (a (and [isNontoken, and [isCreature, youControl]])) enter)
    (putCounters 1 loyaltyCounter (enchanted planeswalker))
theorem okViviensTalentTrigger : Ability.check (DRS.ofList []) viviensTalentTrigger = [] := by decide
/-- Bioessence Hydra -/
def bioessenceHydraTrigger : Ability :=
  whenever
    (countersPut loyaltyCounter anyAmount
      (allOf (and [isPlaneswalker, youControl])))
    (putCounters thatMuch p1p1Counter thisCreature)
theorem okBioessenceHydraTrigger : Ability.check (DRS.ofList []) bioessenceHydraTrigger = [] := by decide
/-- Vraska, Betrayal's Sting -/
def vraskaBetrayalsStingUltimate : Action :=
  doIf (compareAmt (countersOn poison (target isPlayer)) .less 9)
    (putCounters theDifference poison they)
theorem okVraskaBetrayalsStingUltimate :
    Action.check (DRS.ofList []) vraskaBetrayalsStingUltimate = [] := by decide
/-- Vexing Puzzlebox -/
def vexingPuzzleboxCounters : Ability :=
  whenever (does you (rollDice anyAmount .anyDie))
    (putCounters (theOutcome .rollResult) chargeCounter thisArtifact)
theorem okVexingPuzzleboxCounters : Ability.check (DRS.ofList []) vexingPuzzleboxCounters = [] := by decide
def spaceFamilyGoblinsonRoll : Ability :=
  whenever (does you (rollDice 1 .anyDie))
    (putCounters 1 p1p1Counter thisCreature)
theorem okSpaceFamilyGoblinsonRoll : Ability.check (DRS.ofList []) spaceFamilyGoblinsonRoll = [] := by decide
/-- Atomwheel Acrobats -/
def atomwheelAcrobatsRoll : Ability :=
  whenever (does you (rollDice (watch := resultIn (oneThrough 2))))
    (putCounters thatMuch p1p1Counter thisCreature)
theorem okAtomwheelAcrobatsRoll : Ability.check (DRS.ofList []) atomwheelAcrobatsRoll = [] := by decide
/-- Resolute Veggiesaur -/
def resoluteVeggiesaurThirdDie : Ability :=
  whenever (nthOccurrence (.nth 3) (does you (rollDice 1 .anyDie)) (some .turn))
    (putCounters 1 p1p1Counter thisCreature)
theorem okResoluteVeggiesaurThirdDie : Ability.check (DRS.ofList []) resoluteVeggiesaurThirdDie = [] := by decide
/-- Run the Play (Striding Shotcaller's other half) -/
def runThePlayCounters : Action :=
  putCounters 1 p1p1Counter
    (eachOf (targets (upToOf (letter .x)) isCreature))
theorem okRunThePlayCounters : Action.check (DRS.ofList [letterB .x]) runThePlayCounters = [] := by decide
/-- Font of Agonies -/
def fontOfAgoniesTrigger : Ability :=
  whenever (does you (pay (loseLife anyAmount) .once))
    (putCounters thatMuch bloodCounter thisEnchantment)
theorem okFontOfAgoniesTrigger : Ability.check (DRS.ofList []) fontOfAgoniesTrigger = [] := by decide
/-- Heart of Kiran -/
def heartOfKiranCrewAltCost : Ability :=
  static (altCost (allOf (and [abilityHead (.keyword crew), abilityOf this_]))
    (some
       (perform
          (removeCounters (exactly 1) loyaltyCounter
             (a (and [isPlaneswalker, youControl])) true))))
theorem okHeartOfKiranCrewAltCost : Ability.check (DRS.ofList []) heartOfKiranCrewAltCost = [] := by decide
/-- Tainted Adversary -/
def taintedAdversaryOffer : Ability :=
  when (does thisCreature enter)
    (offerWhen (pay [generic 2, pip .black] .anyNumberOfTimes)
       (putCounters thatMuch p1p1Counter thisCreature))
theorem okTaintedAdversaryOffer : Ability.check (DRS.ofList []) taintedAdversaryOffer = [] := by decide
/-- Korvold, Gleeful Glutton -/
def korvoldCombatTrigger : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
    (sequentially
      [ putCounters (letter .x) p1p1Counter thisCreature,
        draw (letter .x),
        define .x (distinctCount .permanentType (allOf (cardIn yourGraveyard))) ])
theorem okKorvoldCombatTrigger : Ability.check (DRS.ofList []) korvoldCombatTrigger = [] := by decide
/-- Mirelurk Queen -/
def mirelurkQueenTrigger : Ability :=
  whenever
    (does any (millCards (counted (orMore 1) (and [non isLand, isCard, in_ library]))))
    (sequentially [draw 1, putCounters 1 p1p1Counter
        thisCreature]) (limit := oncePerTurn)
theorem okMirelurkQueenTrigger : Ability.check (DRS.ofList []) mirelurkQueenTrigger = [] := by decide

/-- Whirling Dervish -/
def whirlingDervish : Spelled := spelled <| singleFacedCard
  { name := "Whirling Dervish", cost := some [pip .green, pip .green], types := [.creature],
    subtypes := [human, monk],
    text :=
      [ protection (hasColor .black),
        at_ (beginningOfPossessed .endStep eachPlayer)
          (if_ := some (did thisCreature (dealDamage anyAmount anOpponent) soFarThisTurn))
          (putCounters 1 p1p1Counter it) ],
    power := stat 1, toughness := stat 1 }

/-- Ichor Shade -/
def ichorShade : Spelled := spelled <| singleFacedCard
  { name := "Ichor Shade", cost := [generic 2, pip .black], types := [.creature],
    subtypes := [phyrexian, shade],
    text :=
      [ at_ (beginningOfYour .endStep)
          (if_ := some
            (was
              (putInto (a (or [isArtifact, isCreature])) graveyard (from_ := battlefield))
              soFarThisTurn))
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 2, toughness := stat 3 }

/-- Asmira, Holy Avenger -/
def asmiraHolyAvenger : Spelled := spelled <| singleFacedCard
  { name := "Asmira, Holy Avenger", cost := [generic 2, pip .green, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ flying,
        at_ (beginningOfPossessed .endStep eachPlayer)
          (putCounters
            (eventCount (putInto (a isCreature) yourGraveyard (from_ := battlefield)) any
               soFarThisTurn)
            p1p1Counter thisCreature) ],
    power := stat 2, toughness := stat 3 }

/-- Thought Sponge -/
def thoughtSponge : Spelled := spelled <| singleFacedCard
  { name := "Thought Sponge", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [sponge],
    text :=
      [ flash,
        static (entersWithCounters thisCreature greatestCardsAnOpponentDrew p1p1Counter),
        when (does thisCreature die) (draw (powerOf thisCreature)) ],
    power := stat 1, toughness := stat 1 }

/-- Skeleton Ship -/
def skeletonShip : Spelled := spelled <| singleFacedCard
  { name := "Skeleton Ship", cost := [generic 3, pip .blue, pip .black],
    supertypes := [.legendary], types := [.creature], subtypes := [skeleton],
    text :=
      [ when
          (stateHolds
            (not (exists_ (and [isLand, island, youControl]))))
          (sacrifice thisCreature),
        activated tapSymbol (putCounters 1 m1m1Counter (target isCreature)) ],
    power := stat 0, toughness := stat 3 }

/-- Contractual Safeguard -/
def contractualSafeguardPass : Action :=
  sequentially
    [ choose (a (counterKindOn (a (and [isCreature, youControl])))),
      putCounters 1 thatKind (each (and [isCreature, youControl, isOther])) ]
theorem okContractualSafeguardPass : Action.check (DRS.ofList []) contractualSafeguardPass = [] := by decide

/-- Feral Contest -/
def feralContest : Spelled := spelled <| singleFacedCard
  { name := "Feral Contest", cost := [generic 3, pip .green], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ putCounters 1 p1p1Counter (target (and [isCreature, youControl])),
            establishThroughout (must (target (and [isCreature, isOther]))
                                   (block it)) thisTurn ]) ] }

/-- Thranduil's Company -/
def thranduilsCompany : Spelled := spelled <| singleFacedCard
  { name := "Thranduil's Company", cost := [generic 2, pip .green, pip .blue],
    types := [.creature], subtypes := [elf, soldier],
    text :=
      [ static (onlyWhile (can you (play (counted (additional (exactly 1)) isLand)))
          (exists_ (and [ and [isCreature, isOther], elf,
                           youControl ]))),
        landfall
          (whenever (does (a (and [isLand, youControl])) enter)
            (sequentially
              [ putCounters 2 p1p1Counter
                  (target (and [isCreature, youControl])),
                gain it vigilance untilEndOfTurn ])) ],
    power := stat 3, toughness := stat 4 }

/-- Stunning Shot -/
def stunningShot : Spelled := spelled <| singleFacedCard
  { name := "Stunning Shot", cost := [generic 1, pip .white], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ putCounters 2 p1p1Counter
              (targets (upTo 1) (and [isCreature, youControl])),
            tap (targets (upTo 1) (and [isCreature, anOpponentControls])),
            putCounters 1 stunCounter it ]) ] }

/-- Stonebinder's Familiar -/
def stonebindersFamiliar : Spelled := spelled <| singleFacedCard
  { name := "Stonebinder's Familiar", cost := [pip .white], types := [.creature],
    subtypes := [spirit, dog],
    text :=
      [ whenever (does any (putInto (counted (orMore 1) isCard) exileZone))
          (during := duringPart .turn (some you))
          (putCounters 1 p1p1Counter thisCreature) (limit := oncePerTurn) ],
    power := stat 1, toughness := stat 1 }

/-- Nazgûl -/
def nazgulRingTrigger : Ability :=
  whenever (does you theRingTemptsYou)
    (putCounters 1 p1p1Counter
      (each (and [wraith, youControl])))
theorem okNazgulRingTrigger : Ability.check (DRS.ofList []) nazgulRingTrigger = [] := by decide

/-- Captain Marvel, Apex Avenger -/
def captainMarvelApexAvenger : Spelled := spelled <| singleFacedCard
  { name := "Captain Marvel, Apex Avenger", cost := [generic 5, pip .red, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, kree, hero],
    text :=
      [ flying, doubleStrike, indestructible,
        whenever
          (does (by_ you) (putCounters anyAmount anyKind (a (and [isCreature, isOther])) false))
          (if_ := is it (not kree))
          (may (putCounters thatMuch thoseCounters this_)) ],
    power := stat 4, toughness := stat 4 }

/-- Blood Spatter Analysis -/
def bloodSpatterAnalysis : Spelled := spelled <| singleFacedCard
  { name := "Blood Spatter Analysis", cost := [pip .black, pip .red], types := [.enchantment],
    text :=
      [ when (does thisEnchantment enter)
          (instruct thisEnchantment
             (dealDamage 3 (target (and [isCreature, anOpponentControls])))),
        whenever (does (counted (orMore 1) isCreature) die)
          (sequentially
            [ mill 1,
              putCounters 1 bloodstainCounter thisEnchantment,
              triggerReflexively
                (doOnlyIf (sacrifice thisEnchantment)
                  (compareAmt (countersOn bloodstainCounter thisEnchantment) .atLeast 5))
                (returnToHand (target (and [isCreature, isCard, in_ yourGraveyard]))) ]) ] }

/-- Bewitching Leechcraft -/
def bewitchingLeechcraft : Spelled := spelled <| singleFacedCard
  { name := "Bewitching Leechcraft", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        when (does thisAura enter) (tap (enchanted creature)),
        static (abilityGrant (enchanted creature)
          (static (replace (would thisCreature (become untapped)) (instead
              (doIfDone
                (removeCounters (exactly 1) p1p1Counter thisCreature true)
                (untap thisCreature))) (duringPart .untapStep (some you))))) ] }

def agentOfTheShadowThievesGrantedTrigger : Ability :=
  whenever (does thisCreature (attack (a isPlayer))) (if_ := some (not
                                                                          (exists_
                                                                             (and [ isOpponent,
                                 compare .greater (lifeTotalOf (that player))
                                     (axes := [.playerStat .lifeTotal]) ]))))
                                     (putCounters 1 p1p1Counter thisCreature)
theorem okAgentOfTheShadowThievesGrantedTrigger :
    Ability.check (DRS.ofList []) agentOfTheShadowThievesGrantedTrigger = [] := by decide

/-- Bloodline Pretender -/
def bloodlinePretender : Spelled := spelled <| singleFacedCard
  { name := "Bloodline Pretender", cost := [generic 3], types := [.artifact, .creature],
    subtypes := [shapeshifter],
    text :=
      [ changeling,
        static (entersChoosing thisCreature (.subtype .creature)),
        whenever
          (does (a (and [ isCreature, youControl, isOther,
                              ofChosen (.subtype .creature) ])) enter)
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 2, toughness := stat 2 }

/-- Chamber Sentry -/
def chamberSentryDamage : Ability :=
  activated
    (compound
      [ mana [.variable], tapSymbol,
        (removeCounters (exactlyOf (letter .x)) p1p1Counter thisCreature true) ])
    (instruct this_ (dealDamage (letter .x) (target isAnyTarget)))
theorem okChamberSentryDamage : Ability.check (DRS.ofList []) chamberSentryDamage = [] := by decide

/-- Menacing Ogre -/
def menacingOgre : Spelled := spelled <| singleFacedCard
  { name := "Menacing Ogre", cost := [generic 3, pip .red, pip .red], types := [.creature],
    subtypes := [ogre],
    text :=
      [ trample, haste,
        when (does thisCreature enter)
          (sequentially
            [ instruct eachPlayer (choose (disclosure := .secretly) (a (quality .number))),
              revealChoices .numbers,
              instruct (each (and [isPlayer, choseExtreme .max])) (loseLife thatMuch),
              doIf (is you (choseExtreme .max))
                (putCounters 2 p1p1Counter thisCreature) ]) ],
    power := stat 3, toughness := stat 3 }

/-- Charnel Troll -/
def charnelTroll : Spelled := spelled <| singleFacedCard
  { name := "Charnel Troll", cost := [generic 1, pip .black, pip .green], types := [.creature],
    subtypes := [troll],
    text :=
      [ trample,
        at_ (beginningOfYour .upkeep)
          (doIfDone (exile (a (and [isCreature, in_ yourGraveyard])))
            (putCounters 1 p1p1Counter thisCreature)
            (sacrifice thisCreature)),
        activated
          (compound
            [ [pip .black, pip .green],
              (discard (a (and [isCreature, in_ yourHand]))) ])
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 4, toughness := stat 4 }

/-- Mistbreath Elder -/
def mistbreathElder : Spelled := spelled <| singleFacedCard
  { name := "Mistbreath Elder", cost := [pip .green], types := [.creature],
    subtypes := [frog, warrior],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (doIfDone (returnToHand (a (and [isCreature, youControl, isOther])))
            (putCounters 1 p1p1Counter thisCreature)
            (may (returnToHand thisCreature))) ],
    power := stat 2, toughness := stat 2 }

/-- Dark Depths -/
def darkDepths : Spelled := spelled <| singleFacedCard
  { name := "Dark Depths", supertypes := [.legendary, .snow], types := [.land],
    text :=
      [ static (entersWithCounters thisLand 10 iceCounter),
        activated [generic 3]
          (removeCounters (exactly 1) iceCounter thisLand),
        when (stateHolds (not (is thisLand (hasCounter iceCounter))))
          (doIfDone (sacrifice thisLand) (createToken maritLage))
              ] }

/-- Thallid -/
def thallid : Spelled := spelled <| singleFacedCard
  { name := "Thallid", cost := [pip .green], types := [.creature],
    subtypes := [fungus],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (putCounters 1 sporeCounter thisCreature),
        activated
          (removeCounters (exactly 3) sporeCounter thisCreature true)
          (createToken (creatureToken 1 1 [.green] [saproling])) ],
    power := stat 1, toughness := stat 1 }

/-- Aether Chaser -/
def aetherChaserEnergy : Ability :=
  when (does thisCreature enter) (putCounters 2 energy you)
theorem okAetherChaserEnergy : Ability.check (DRS.ofList []) aetherChaserEnergy = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossEntersOiled : StaticSpec :=
  entersWithCounters thisCreature 4 oilCounter
theorem okArchfiendOfTheDrossEntersOiled :
    StaticSpec.check (DRS.ofList []) archfiendOfTheDrossEntersOiled = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossOilUpkeep : Ability :=
  at_ (beginningOfYour .upkeep)
    (removeCounters (exactly 1) oilCounter thisCreature true)
theorem okArchfiendOfTheDrossOilUpkeep : Ability.check (DRS.ofList []) archfiendOfTheDrossOilUpkeep = [] := by
  decide
def neurokTransmuter : Action :=
  become (target isCreature) { characteristics := { types := [.artifact] } } untilEndOfTurn
theorem okNeurokTransmuter : Action.check (DRS.ofList []) neurokTransmuter = [] := by decide
def syphonMind : Action :=
  sequentially
    [ sequentially [instruct (each isOtherPlayer) (choose (a (cardIn hand))), discard them],
      doForEach (each (and [isCard, was (discard which) thisWay])) (draw 1) ]
theorem okSyphonMind : Action.check (DRS.ofList []) syphonMind = [] := by decide
def peek : Action := sequentially [lookAtHandOf (target isPlayer), draw 1]
theorem okPeek : Action.check (DRS.ofList []) peek = [] := by decide
/-- Bumi, King of Three Trials -/
def bumiScryMode : Action := instruct (target isPlayer) (scry 3)
theorem okBumiScryMode : Action.check (DRS.ofList []) bumiScryMode = [] := by decide
/-- Final Act -/
def finalActCounterMode : Action := loseAllCounters none (agent := eachOpponent)
theorem okFinalActCounterMode : Action.check (DRS.ofList []) finalActCounterMode = [] := by decide
def leeches : Action :=
  loseAllCounters poison (agent := target isPlayer)
theorem okLeeches : Action.check (DRS.ofList []) leeches = [] := by decide

/-- Woeleecher -/
def woeleecherWhole : Spelled := spelled <| singleFacedCard
  { name := "Woeleecher", cost := [generic 5, pip .white], types := [.creature],
    subtypes := [elemental], text := [woeleecher],
    power := stat 3, toughness := stat 5 }

end Semantics.Cards
