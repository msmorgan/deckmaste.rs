import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Keyword

/-!
# Semantics.Cards.Counters

Port of `idris/src/Experimental/Cards/Counters.idr`: the printed cards of the Counters family
and the bench items beside them.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

/-- Crovax the Cursed -/
def crovaxTheCursed : Instruction :=
  .may .you (sacrifice .you (a creature))
    (some (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature))
    (some (.removeCounters (some (exactly 1)) (some (.printed plusOnePlusOne)) thisCreature))
theorem okCrovaxTheCursed : Instruction.check [] crovaxTheCursed = [] := by decide
def additiveEvolution : Instruction :=
  .sequentially
    [ create (.lit 1) (creatureToken 0 0 [.green, .blue] [creatureType "Fractal"]),
      .putCounters (.lit 3) (.printed plusOnePlusOne) it ]
theorem okAdditiveEvolution : Instruction.check [] additiveEvolution = [] := by decide
def battlegrowth : Instruction := .putCounters (.lit 1) (.printed plusOnePlusOne) (target creature)
theorem okBattlegrowth : Instruction.check [] battlegrowth = [] := by decide
def chainbreaker : Instruction :=
  .removeCounters (some (exactly 1)) (some (.printed minusOneMinusOne)) (target creature)
theorem okChainbreaker : Instruction.check [] chainbreaker = [] := by decide
def kaitoBaneOfNightmares : Instruction :=
  .sequentially
    [.setStatus .tapped (target creature), .putCounters (.lit 2) (.printed (.named "Stun")) it]
theorem okKaitoBaneOfNightmares : Instruction.check [] kaitoBaneOfNightmares = [] := by decide
def jhoiraOfTheGhitu : Ability :=
  activated
    (.compound [.mana [generic 2], .perform (exile (a (.and [.not land, .inZone (handOf .you)])))])
    (.putCounters (.lit 4) (.printed (.named "Time"))
      (theVerbed (.action "Exile") .card .attributive .one))
theorem okJhoiraOfTheGhitu : Ability.check [] jhoiraOfTheGhitu = [] := by decide
def alaundoTheSeer : Instruction :=
  .removeCounters (some (exactly 1)) (some (.printed (.named "Time")))
    (each (.and [.hasPossessor .owner .you, .inZone exileZone]))
theorem okAlaundoTheSeer : Instruction.check [] alaundoTheSeer = [] := by decide

def daydream : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daydream", cost := some [pip .white], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ exile (target creatureYouControl),
              returnToBattlefieldWithCounters (that .card) (ownerOf (that .card)) (.lit 1)
                plusOnePlusOne ]),
          keywordCosting "Flashback" (.mana [generic 2, pip .white]) ] } }

def ashnodsTransmogrant : Ability :=
  activated (.compound [.tapSymbol, .perform (sacrifice .you thisArtifact)])
    (.sequentially
      [ .putCounters (.lit 1) (.printed plusOnePlusOne) (target (.and [creature, .not artifact])),
        becomes (that (.type .creature)) { characteristics := { types := [.artifact] } } none ])
theorem okAshnodsTransmogrant : Ability.check [] ashnodsTransmogrant = [] := by decide
def azulaAlwaysLies : Instruction :=
  chooseModes (oneThrough 2)
    [ gets (target creature) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn),
      .putCounters (.lit 1) (.printed plusOnePlusOne) (target creature) ]
theorem okAzulaAlwaysLies : Instruction.check [] azulaAlwaysLies = [] := by decide
def bellowingAegisaur : Instruction :=
  .putCounters (.lit 1) (.printed plusOnePlusOne) (each (otherCreatureYouControl thisCreature))
theorem okBellowingAegisaur : Instruction.check [] bellowingAegisaur = [] := by decide
def carnifexDemon : Instruction :=
  .putCounters (.lit 1) (.printed minusOneMinusOne) (each (otherCreature thisCreature))
theorem okCarnifexDemon : Instruction.check [] carnifexDemon = [] := by decide
def ajaniAdversaryOfTyrants : Instruction :=
  .putCounters (.lit 1) (.printed plusOnePlusOne) (.eachOf (.described (.target (upTo 2)) creature))
theorem okAjaniAdversaryOfTyrants : Instruction.check [] ajaniAdversaryOfTyrants = [] := by decide
def naturesPanoply : Instruction :=
  .sequentially
    [ choose (.described (.target anyNumber) creature),
      .putCounters (.lit 1) (.printed plusOnePlusOne) (.eachOf them) ]
theorem okNaturesPanoply : Instruction.check [] naturesPanoply = [] := by decide
def armamentCorps : Instruction :=
  distributeCounters (.lit 2) plusOnePlusOne
    (.described (.target (oneThrough 2)) creatureYouControl)
theorem okArmamentCorps : Instruction.check [] armamentCorps = [] := by decide
def savagebornHydra : Ability :=
  activatedOnlyDuring (.mana [generic 1, hybridPip .red .green])
    (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature) .asSorcery
theorem okSavagebornHydra : Ability.check [] savagebornHydra = [] := by decide
def woeleecher : Ability :=
  activated (.compound [.mana [pip .white], .tapSymbol])
    (.ifDone
      (.removeCounters (some (exactly 1)) (some (.printed minusOneMinusOne)) (target creature))
      (some (gainsLife .you (.lit 2))) none)
theorem okWoeleecher : Ability.check [] woeleecher = [] := by decide
def anointerOfValor : Ability :=
  whenever (attacks (a creature))
    (mayWhen .you (.pay .you (.mana [generic 3]) .once)
      (.putCounters (.lit 1) (.printed plusOnePlusOne) (that (.type .creature))))
theorem okAnointerOfValor : Ability.check [] anointerOfValor = [] := by decide
def avianOddity : Instruction :=
  .putCounters (.lit 1) (.printed flyingCounter) (target creatureYouControl)
theorem okAvianOddity : Instruction.check [] avianOddity = [] := by decide
/-- Song of Eärendil -/
def songOfEarendil : Instruction :=
  .putCounters (.lit 1) (.printed flyingCounter)
    (each (.and [creature, .hasPossessor .controller .you, .not (.hasKeyword (.the "Flying"))]))
theorem okSongOfEarendil : Instruction.check [] songOfEarendil = [] := by decide
def gideonsAvenger : Ability :=
  whenever (.statusEvent (a (.and [creature, .hasPossessor .controller anOpponent])) .tapped)
    (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature)
theorem okGideonsAvenger : Ability.check [] gideonsAvenger = [] := by decide
def prologueToPhyresis : Instruction :=
  .putCounters (.lit 1) (.printed (.named "Poison")) (each .opponent)
theorem okPrologueToPhyresis : Instruction.check [] prologueToPhyresis = [] := by decide
def screechingScorchbeast : Ability :=
  whenever (attacks thisCreature) (.putCounters (.lit 2) (.printed (.named "Rad")) (each .anyPlayer))
theorem okScreechingScorchbeast : Ability.check [] screechingScorchbeast = [] := by decide
def merenOfClanNelToth : Ability :=
  whenever (.dies (a (otherCreatureYouControl thisCreature)))
    (.putCounters (.lit 1) (.printed (.named "Experience")) .you)
theorem okMerenOfClanNelToth : Ability.check [] merenOfClanNelToth = [] := by decide
def kratosStoicFather : Ability :=
  at_ (.beginningOf .the .endStep (.byPlayer .you))
    (.putCounters (countersOn (.named "Experience") .you) (.printed plusOnePlusOne)
      (target creature))
theorem okKratosStoicFather : Ability.check [] kratosStoicFather = [] := by decide
def vashtaNerada : Ability :=
  triggeredIf (.beginningOf .the .endStep (.byPlayer (each .anyPlayer)))
    (happened .death (a creature) .thisTurn)
    (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature)
theorem okVashtaNerada : Ability.check [] vashtaNerada = [] := by decide
def furiousSpinesplitter : Ability :=
  at_ (.beginningOf .the .endStep (.byPlayer .you))
    (.putCounters (forEach 1 (.and [.opponent, happenedTo .damageTaken .thisTurn]))
      (.printed plusOnePlusOne) thisCreature)
theorem okFuriousSpinesplitter : Ability.check [] furiousSpinesplitter = [] := by decide
def paladinOfAtonement : Ability :=
  triggeredIf (.beginningOf .the .upkeep (.byPlayer (each .anyPlayer)))
    (happened .lifeLoss .you .lastTurn) (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature)
theorem okPaladinOfAtonement : Ability.check [] paladinOfAtonement = [] := by decide
def throneWarden : Ability :=
  triggeredIf (.beginningOf .the .endStep (.byPlayer .you))
    (.matches .you (.hasDesignation "the monarch" none))
    (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature)
theorem okThroneWarden : Ability.check [] throneWarden = [] := by decide
def bloodTyrant : Ability :=
  whenever (.losesGame (a .anyPlayer)) (.putCounters (.lit 5) (.printed plusOnePlusOne) thisCreature)
theorem okBloodTyrant : Ability.check [] bloodTyrant = [] := by decide

def stagBeetle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stag Beetle", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Insect"],
      text :=
        [ .static (.andAlso none
            [ entersWithCounters thisCreature (.letter .x) plusOnePlusOne,
              .definesLetter .x (countOf (otherCreature thisCreature)) ]) ],
      power := stat 0, toughness := stat 0 } }

def toweringTitan : Ability :=
  .static (.andAlso none
    [ entersWithCounters thisCreature (.letter .x) plusOnePlusOne,
      .definesLetter .x
        (aggregate .sum (.stat .toughness) (otherCreatureYouControl thisCreature)) ])
theorem okToweringTitan : Ability.check [] toweringTitan = [] := by decide

def coretapper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coretapper", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Myr"],
      text :=
        [ activated .tapSymbol (.putCounters (.lit 1) (.printed (.named "Charge")) (target artifact)),
          activated (.perform (sacrifice .you thisCreature))
            (.putCounters (.lit 2) (.printed (.named "Charge")) (target artifact)) ],
      power := stat 1, toughness := stat 1 } }

def divineIntervention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Intervention", cost := some [generic 6, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ .static (entersWithCounters thisEnchantment (.lit 2) (.named "Intervention")),
          at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (.removeCounters (some (exactly 1)) (some (.printed (.named "Intervention")))
              thisEnchantment),
          when (lastCounterRemovedBy (.named "Intervention") thisEnchantment .you) .gameDrawn ] } }

def celestialConvergence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celestial Convergence", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ .static (entersWithCounters thisEnchantment (.lit 7) (.named "Omen")),
          at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (.sequentially
              [ .removeCounters (some (exactly 1)) (some (.printed (.named "Omen"))) thisEnchantment,
                .if_ (.compareAmt (countersOn (.named "Omen") thisEnchantment) .atMost (.lit 0))
                  (.concludes .winGame
                    (the (.and [.anyPlayer, .superlative .max (.playerStat .lifeTotal) .anyPlayer])))
                  none,
                .if_ (.compareAmt
                        (countOf (.and [.anyPlayer, .superlative .max (.playerStat .lifeTotal) .anyPlayer]))
                        .atLeast (.lit 2))
                  .gameDrawn none ]) ] } }

def curseOfVengeance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curse of Vengeance", cost := some [pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura", enchantmentType "Curse"],
      text :=
        [ keywordSubject "Enchant" .anyPlayer,
          whenever (.casts (.attachHost .enchanted .player) (a spell) none)
            (.putCounters (.lit 1) (.printed (.named "Spite")) thisAura),
          when (.losesGame (.attachHost .enchanted .player))
            (.sequentially
              [ gainsLife .you (.letter .x), .draw .you (.letter .x),
                .define .x (countersOn (.named "Spite") thisAura) ]) ] } }

def passagewaySeer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Passageway Seer", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Tiefling", creatureType "Warlock"],
      text :=
        [ keyword "Lifelink",
          when (.enters thisCreature none) (.gainsDesignation .you "the initiative" .instructed none),
          triggeredIf (.beginningOf .the .endStep (.byPlayer .you))
            (.matches .you (.hasDesignation "the initiative" none))
            (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature) ],
      power := stat 2, toughness := stat 2 } }

def chainsaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chainsaw", cost := some [generic 1, pip .red], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ when (.enters thisEquipment none)
            (.dealDamage it (.lit 3) (.described (.target (upTo 1)) creature)),
          whenever (.dies (counted (atLeast 1) creature))
            (.putCounters (.lit 1) (.printed (.named "Rev")) thisEquipment),
          .static (.andAlso none
            [ .modify (.attachHost .equipped (.type .creature)) .power (.up (.letter .x)),
              .modify (.attachHost .equipped (.type .creature)) .toughness (.up (.lit 0)),
              .definesLetter .x (countersOn (.named "Rev") thisEquipment) ]),
          keywordCosting "Equip" (.mana [generic 3]) ] } }

/-- Soul's Might -/
def soulsMight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soul's Might", cost := some [generic 4, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ .putCounters (.letter .x) (.printed plusOnePlusOne) (target creature),
              .define .x (.statOf (.stat .power) (that (.type .creature))) ]) ] } }

def woodlandChampion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Woodland Champion", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Scout"],
      text :=
        [ whenever (.enters (counted (atLeast 1) (.and [.isToken, .hasPossessor .controller .you])) none)
            (.putCounters .groupSize (.printed plusOnePlusOne) thisCreature) ],
      power := stat 2, toughness := stat 2 } }

def voraciousBrood : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voracious Brood", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Alien", creatureType "Insect"],
      text :=
        [ .static (entersWithCounters thisCreature
            (forEach 1 (.and [creature, .inZone (graveyardOf .you)])) plusOnePlusOne),
          whenever (putIntoFrom (counted (atLeast 1) creature) (graveyardOf .you) .anywhere)
            (.putCounters .groupSize (.printed plusOnePlusOne) thisCreature) ],
      power := stat 1, toughness := stat 1 } }

def herdBaloth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Herd Baloth", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ whenever (counterEvent .put plusOnePlusOne .many thisCreature)
            (may .you (create (.lit 1) (creatureToken 4 4 [.green] [creatureType "Beast"]))) ],
      power := stat 4, toughness := stat 4 } }

def flourishingDefenses : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Flourishing Defenses", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ whenever (counterEvent .put minusOneMinusOne .one (a creature))
            (may .you
              (create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Elf", creatureType "Warrior"]))) ] } }

def rakshasaVizier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rakshasa Vizier", cost := some [generic 2, pip .black, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Demon"],
      text :=
        [ whenever
            (putIntoFrom (counted (atLeast 1) (.inZone (graveyardOf .you))) exileZone
              (.zones [graveyardOf .you]))
            (.putCounters .groupSize (.printed plusOnePlusOne) thisCreature) ],
      power := stat 4, toughness := stat 4 } }

def foeLiage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foe-liage", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Plant", creatureType "Mutant"],
      text :=
        [ triggeredOnlyDuring (.enters (a land) none) (.duringPart .turn (some .you))
            (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature) ],
      power := stat 3, toughness := stat 3 } }

def hardenedScales : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hardened Scales", cost := some [pip .green], types := [.enchantment],
      text :=
        [ .static (.intercepts (counterEvent .put plusOnePlusOne .many (a creatureYouControl)) [] none
            (.putCounters (plus .thatMuch (.lit 1)) (.printed plusOnePlusOne) it) .repeatedly none) ] } }

def branchingEvolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Branching Evolution", cost := some [generic 2, pip .green], types := [.enchantment],
      text :=
        [ .static (.intercepts (counterEvent .put plusOnePlusOne .many (a creatureYouControl)) [] none
            (.putCounters (times (.lit 2) .thatMuch) (.printed plusOnePlusOne) (that (.type .creature)))
            .repeatedly none) ] } }

/-- Corpsejack Menace -/
def corpsejackMenace : Ability :=
  .static (.intercepts (counterEvent .put plusOnePlusOne .many (a creatureYouControl)) [] none
    (.putCounters (times (.lit 2) .thatMuch) (.printed plusOnePlusOne) it) .repeatedly none)
theorem okCorpsejackMenace : Ability.check [] corpsejackMenace = [] := by decide

/-- Doubling Season -/
def doublingSeason : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Doubling Season", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ .static (.intercepts (tokensCreatedByEffectUnder (counted (atLeast 1) .isToken) .you) [] none
            (.create .you (times (.lit 2) .groupSize) .asThose []) .repeatedly none),
          .static (.intercepts
            (manyCountersPutByEffect (a (.and [permanent, .hasPossessor .controller .you]))) [] none
            (.putCounters (times (.lit 2) .thatMuch) .those (that .permanent)) .repeatedly none) ] } }

/-- Doc Samson, Super Psychiatrist -/
def docSamsonDistributive : Ability :=
  .static (.intercepts
    (manyBareCountersPutBy .you (a (.and [permanent, .hasPossessor .controller .you]))) [] none
    (.putCounters (plus .thatMuch (.lit 1)) .those (that .permanent)) .repeatedly none)
theorem okDocSamsonDistributive : Ability.check [] docSamsonDistributive = [] := by decide

/-- Winding Constrictor -/
def windingConstrictor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Winding Constrictor", cost := some [pip .black, pip .green], types := [.creature],
      subtypes := [creatureType "Snake"],
      text :=
        [ .static (.intercepts
            (bareCounterEvent .put .many
              (a (.and [.or [artifact, creature], .hasPossessor .controller .you]))) [] none
            (.putCounters (plus .thatMuch (.lit 1)) .those (that .permanent)) .repeatedly none),
          .static (.intercepts (bareCounterEvent .put .many .you) [] none
            (.putCounters (plus .thatMuch (.lit 1)) .those .you) .repeatedly none) ],
      power := stat 2, toughness := stat 3 } }

/-- Aragorn, Company Leader -/
def aragornDistributive : Ability :=
  whenever (manyBareCountersPutBy .you thisCreature)
    (.putCounters (.lit 1) .those (.described (.target (upTo 1)) (otherCreature thisCreature)))
theorem okAragornDistributive : Ability.check [] aragornDistributive = [] := by decide
/-- Captain Marvel, Apex Avenger -/
def captainMarvelSameKinds : Ability :=
  triggeredIf (manyBareCountersPutBy .you (a (otherCreature thisCreature)))
    (.not (.matches it (.hasSubtype (creatureType "Kree"))))
    (may .you (.putCounters .thatMuch .those thisCreature))
theorem okCaptainMarvelSameKinds : Ability.check [] captainMarvelSameKinds = [] := by decide
/-- Denry Klin, Editor in Chief -/
def denryKlinSameKinds : Ability :=
  triggeredIf (.enters (a (.and [nontoken, creatureYouControl])) none)
    (.matches thisCreature (.hasCounters none))
    (.putCounters (.lit 1) (.sameAs thisCreature) (that (.type .creature)))
theorem okDenryKlinSameKinds : Ability.check [] denryKlinSameKinds = [] := by decide

/-- Denry Klin, Editor in Chief -/
def denryKlin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Denry Klin, Editor in Chief", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Cat", creatureType "Advisor"],
      text :=
        [ .static (.entersRider thisCreature
            (.withCounters (.lit 1)
              (.chosen [plusOnePlusOne, .keyword "FirstStrike", .keyword "Vigilance"]) .fresh)),
          denryKlinSameKinds ],
      power := stat 2, toughness := stat 2 } }

/-- Me, the Immortal -/
def meTheImmortalCounterMenu : Ability :=
  at_ (.beginningOf .the .combat (.byPlayer .you))
    (.putCounters (.lit 1)
      (.chosen [plusOnePlusOne, .keyword "FirstStrike", .keyword "Vigilance", .keyword "Menace"])
      thisCreature)
theorem okMeTheImmortalCounterMenu : Ability.check [] meTheImmortalCounterMenu = [] := by decide

/-- Star Pupil -/
def starPupil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Star Pupil", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ .static (entersWithCounters thisCreature (.lit 1) plusOnePlusOne),
          when (.dies thisCreature) (.putCounters (.lit 1) (.sameAs it) (target creatureYouControl)) ],
      power := stat 0, toughness := stat 0 } }

/-- Tromell, Seymour's Butler -/
def tromell : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tromell, Seymour's Butler", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Advisor"],
      text :=
        [ .static (entersWithAdditionalCounters
            (each (.and [creature, nontoken, .hasPossessor .controller .you, .otherThan thisCreature]))
            (.lit 1) plusOnePlusOne),
          activated (.compound [.mana [generic 1], .tapSymbol])
            (.sequentially
              [ .repeated (.letter .x) proliferate,
                .define .x
                  (countOf (.and [ nontoken, creature, .hasPossessor .controller .you,
                                   .happenedTo (.mk .entry .thisTurn none) ])) ]) ],
      power := stat 2, toughness := stat 3 } }

/-- Runadi, Behemoth Caller -/
def runadiBehemothCaller : Ability :=
  .static (.gains
    (allOf (.and [ creature, .hasPossessor .controller .you,
                   .compare [.counter plusOnePlusOne] .atLeast (.lit 3) ]))
    (keyword "Haste"))
theorem okRunadiBehemothCaller : Ability.check [] runadiBehemothCaller = [] := by decide
/-- Boon of Safety -/
def boonOfSafetyPut : Instruction :=
  .putCounters (.lit 1) (.printed (.named "Shield")) (target creature)
theorem okBoonOfSafetyPut : Instruction.check [] boonOfSafetyPut = [] := by decide
/-- Vivien's Talent and Teferi's Talent -/
def talentLoyaltyPut : Instruction :=
  .putCounters (.lit 1) (.printed (.named "Loyalty")) (.attachHost .enchanted (.type .planeswalker))
theorem okTalentLoyaltyPut : Instruction.check [] talentLoyaltyPut = [] := by decide
/-- Simic Fluxmage -/
def simicFluxmageMove : Instruction :=
  .moveCounters (.lit 1) (some (.printed plusOnePlusOne)) thisCreature (target creature)
theorem okSimicFluxmageMove : Instruction.check [] simicFluxmageMove = [] := by decide
/-- Rikku, Resourceful Guardian -/
def rikkuStealMove : Instruction :=
  .moveCounters (.lit 1) none (target (.and [creature, .hasPossessor .controller anOpponent]))
    (target creatureYouControl)
theorem okRikkuStealMove : Instruction.check [] rikkuStealMove = [] := by decide
/-- Littjara Mirrorlake -/
def littjaraMirrorlakeCopy : Instruction :=
  .create .you (.lit 1)
    (.copyOf (target (.and [creature, .hasPossessor .controller .you]))
      [.entersWithCounters (.lit 1) plusOnePlusOne .additional])
    []
theorem okLittjaraMirrorlakeCopy : Instruction.check [] littjaraMirrorlakeCopy = [] := by decide

def shalaiAndHallar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shalai and Hallar", cost := some [generic 1, pip .red, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Angel", creatureType "Elf"],
      text :=
        [ keyword "Flying", keyword "Vigilance",
          whenever (counterEvent .put plusOnePlusOne .many (a creatureYouControl))
            (.dealDamage thisCreature .thatMuch (target .opponent)) ],
      power := stat 3, toughness := stat 3 } }

def primalVigor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Primal Vigor", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ .static (.intercepts (tokensCreated (counted (atLeast 1) .isToken)) [] none
            (.create .you (times (.lit 2) .groupSize) .asThose []) .repeatedly none),
          .static (.intercepts (counterEvent .put plusOnePlusOne .many (a creature)) [] none
            (.putCounters (times (.lit 2) .thatMuch) (.printed plusOnePlusOne) (that (.type .creature)))
            .repeatedly none) ] } }

/-- Patrolling Peacemaker -/
def patrollingPeacemaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Patrolling Peacemaker", cost := some [generic 2, pip .white],
      types := [.artifact, .creature], subtypes := [creatureType "Robot", creatureType "Soldier"],
      text :=
        [ .static (entersWithCounters thisCreature (.lit 2) plusOnePlusOne),
          whenever (.commitsCrime anOpponent) proliferate ],
      power := stat 0, toughness := stat 0 } }

/-- Galloping Lizrog -/
def gallopingLizrog : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Galloping Lizrog", cost := some [generic 3, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Frog", creatureType "Lizard"],
      text :=
        [ keyword "Trample",
          when (.enters thisCreature none)
            (.may .you
              (.removeCounters (some anyNumber) (some (.printed plusOnePlusOne))
                (among (allOf creatureYouControl)))
              (some (.putCounters (times (.lit 2) removedThisWay) (.printed plusOnePlusOne) thisCreature))
              none) ],
      power := stat 3, toughness := stat 3 } }

/-- Novijen Sages -/
def novijenSagesDraw : Ability :=
  activated
    (.compound
      [ .mana [generic 1],
        .perform (.removeCounters (some (exactly 2)) (some (.printed plusOnePlusOne))
          (among (allOf creatureYouControl))) ])
    (.draw .you (.lit 1))
theorem okNovijenSagesDraw : Ability.check [] novijenSagesDraw = [] := by decide
/-- Cyclone -/
def cycloneUpkeepPayment : Ability :=
  at_ (.beginningOf .the .upkeep (.byPlayer .you))
    (.sequentially
      [ .putCounters (.lit 1) (.printed (.named "Wind")) thisEnchantment,
        unless_ .you (sacrifice .you thisEnchantment)
          (scaledMana (.run [pip .green])
            (times (.lit 1) (countersOn (.named "Wind") thisEnchantment))) ])
theorem okCycloneUpkeepPayment : Ability.check [] cycloneUpkeepPayment = [] := by decide

def stormwildCapridor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stormwild Capridor", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Bird", creatureType "Goat"],
      text :=
        [ keyword "Flying",
          .static (.damageRule .noncombatOnly .unattributed (.toRecipient thisCreature)
            (.prevent .all
              (some (.putCounters preventedThisWay (.printed plusOnePlusOne) thisCreature)))
            .repeatedly) ],
      power := stat 1, toughness := stat 3 } }

def testOfFaith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Test of Faith", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ .spell none (.continuously
            (.damageRule .any .unattributed (.toRecipient (target creature))
              (.prevent (.shield (.lit 3))
                (some (.putCounters preventedThisWay (.printed plusOnePlusOne) (that (.type .creature)))))
              .repeatedly)
            (some .thisTurn)) ] } }

def temper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Temper", cost := some [.variable, generic 1, pip .white], types := [.instant],
      text :=
        [ .spell none (.continuously
            (.damageRule .any .unattributed (.toRecipient (target creature))
              (.prevent (.shield (.letter .x))
                (some (.putCounters preventedThisWay (.printed plusOnePlusOne) (that (.type .creature)))))
              .repeatedly)
            (some .thisTurn)) ] } }

def phytohydra : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phytohydra", cost := some [generic 2, pip .green, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Plant", creatureType "Hydra"],
      text :=
        [ .static (.intercepts (.isDealtDamage .any thisCreature) [] none
            (.putCounters .thatMuch (.printed plusOnePlusOne) it) .repeatedly none) ],
      power := stat 1, toughness := stat 1 } }

def agelessEntity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ageless Entity", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (.lifeChanges .you .up) (.putCounters .thatMuch (.printed plusOnePlusOne) thisCreature) ],
      power := stat 4, toughness := stat 4 } }

/-- Chromatic Armor -/
def chromaticArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chromatic Armor", cost := some [generic 1, pip .white, pip .blue],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (entersChoosing thisAura .color),
          .static (entersWithCounters thisAura (.lit 1) (.named "Sleight")),
          .static (.damageRule .any (.dealtBy (allOf (.and [source, ofTheLastChosen .color])))
            (.toRecipient (.attachHost .enchanted (.type .creature))) (.prevent .all none) .repeatedly),
          activated (.mana [.variable])
            (.sequentially
              [ .putCounters (.lit 1) (.printed (.named "Sleight")) thisAura,
                choose (a (quality .color)),
                .define .x (countersOn (.named "Sleight") thisAura) ]) ] } }

def vault75MiddleSchool : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vault 75: Middle School", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment], subtypes := [enchantmentType "Saga"],
      text :=
        [ when (.chapterMark [1])
            (exile (allOf (.and [creature, .compare [.stat .power] .atLeast (.lit 4)]))),
          when (.chapterMark [2, 3])
            (.putCounters (.lit 1) (.printed plusOnePlusOne) (each creatureYouControl)) ] } }

def keldonWarcaller : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keldon Warcaller", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior"],
      text :=
        [ whenever (attacks thisCreature)
            (.putCounters (.lit 1) (.printed (.named "Lore"))
              (target (.and [.hasSubtype (enchantmentType "Saga"), .hasPossessor .controller .you]))) ],
      power := stat 2, toughness := stat 2 } }

def magistratesScepter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magistrate's Scepter", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (.compound [.mana [generic 4], .tapSymbol])
            (.putCounters (.lit 1) (.printed (.named "Charge")) thisArtifact),
          activated
            (.compound
              [ .tapSymbol,
                .perform (.removeCounters (some (exactly 3)) (some (.printed (.named "Charge")))
                  thisArtifact) ])
            (.extraTurn .you (.lit 1)) ] } }

def grumgully : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grumgully, the Generous", cost := some [generic 1, pip .red, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Shaman"],
      text :=
        [ .static (entersWithAdditionalCounters
            (each (.and [ creature, .hasPossessor .controller .you,
                          .not (.hasSubtype (creatureType "Human")), .otherThan thisCreature ]))
            (.lit 1) plusOnePlusOne) ],
      power := stat 3, toughness := stat 3 } }

def metallicMimic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Metallic Mimic", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ .static (entersChoosing thisCreature (.subtype .creature)),
          .static (.becomes thisCreature .adds (.chosenQuality (ofChosen (.subtype .creature)))),
          .static (entersWithAdditionalCounters
            (each (.and [ creature, .hasPossessor .controller .you, ofChosen (.subtype .creature),
                          .otherThan thisCreature ]))
            (.lit 1) plusOnePlusOne) ],
      power := stat 2, toughness := stat 1 } }

def curatorBeastieLine : StaticSpec :=
  entersWithAdditionalCounters (allOf (.and [creature, .hasPossessor .controller .you, colorless]))
    (.lit 2) plusOnePlusOne
theorem okCuratorBeastieLine : StaticSpec.check [] curatorBeastieLine = [] := by decide
def renataLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (.and [creature, .hasPossessor .controller .you, .otherThan thisCreature])) (.lit 1)
    plusOnePlusOne
theorem okRenataLine : StaticSpec.check [] renataLine = [] := by decide
def entDraughtBasinAbility : Ability :=
  activatedOnlyDuring (.compound [.mana [.variable], .tapSymbol])
    (.putCounters (.lit 1) (.printed plusOnePlusOne)
      (target (.and [creature, .compare [.stat .power] .eq (.letter .x)])))
    .asSorcery
theorem okEntDraughtBasinAbility : Ability.check [] entDraughtBasinAbility = [] := by decide

def matopiGolem : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Matopi Golem", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Golem"],
      text :=
        [ activated (.mana [generic 1])
            (.thisWay (.regenerate thisCreature) (regenerates thisCreature)
              (.putCounters (.lit 1) (.printed minusOneMinusOne) it)) ],
      power := stat 3, toughness := stat 3 } }

def bramblewoodParagon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bramblewood Paragon", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Warrior"],
      text :=
        [ .static (entersWithAdditionalCounters
            (each (.and [ creature, .hasSubtype (creatureType "Warrior"), .hasPossessor .controller .you,
                          .otherThan thisCreature ]))
            (.lit 1) plusOnePlusOne),
          .static (.gains
            (each (.and [creature, .hasPossessor .controller .you, .hasCounters (some plusOnePlusOne)]))
            (keyword "Trample")) ],
      power := stat 2, toughness := stat 2 } }

def oonasBlackguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oona's Blackguard", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text :=
        [ keyword "Flying",
          .static (entersWithAdditionalCounters
            (each (.and [ creature, .hasSubtype (creatureType "Rogue"), .hasPossessor .controller .you,
                          .otherThan thisCreature ]))
            (.lit 1) plusOnePlusOne),
          whenever
            (dealsCombatDamage
              (a (.and [creature, .hasPossessor .controller .you, .hasCounters (some plusOnePlusOne)]))
              (a .anyPlayer))
            (discard (that .player) (a (.inZone hand))) ],
      power := stat 1, toughness := stat 1 } }

def crumblingAshes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crumbling Ashes", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (destroy (target (.and [creature, .hasCounters (some minusOneMinusOne)]))) ] } }

def hunterOfEyeblights : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hunter of Eyeblights", cost := some [generic 3, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Elf", creatureType "Assassin"],
      text :=
        [ when (.enters thisCreature none)
            (.putCounters (.lit 1) (.printed plusOnePlusOne) (target creatureYouDontControl)),
          activated (.compound [.mana [generic 2, pip .black], .tapSymbol])
            (destroy (target (.and [creature, .hasCounters none]))) ],
      power := stat 3, toughness := stat 3 } }

def pridemalkin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pridemalkin", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Cat"],
      text :=
        [ when (.enters thisCreature none)
            (.putCounters (.lit 1) (.printed plusOnePlusOne) (target creatureYouControl)),
          .static (.gains
            (each (.and [creature, .hasPossessor .controller .you, .hasCounters (some plusOnePlusOne)]))
            (keyword "Trample")) ],
      power := stat 2, toughness := stat 1 } }

def aboroth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aboroth", cost := some [generic 4, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ cumulativeUpkeep (.perform (.putCounters (.lit 1) (.printed minusOneMinusOne) thisCreature)) ],
      power := stat 9, toughness := stat 9 } }

def shelteringAncient : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sheltering Ancient", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Treefolk"],
      text :=
        [ keyword "Trample",
          cumulativeUpkeep
            (.perform (.putCounters (.lit 1) (.printed plusOnePlusOne)
              (a (.and [creature, .hasPossessor .controller anOpponent])))) ],
      power := stat 5, toughness := stat 5 } }

def coverOfWinterPut : Ability :=
  activated (.mana [.snow]) (.putCounters (.lit 1) (.printed (.named "Age")) thisEnchantment)
theorem okCoverOfWinterPut : Ability.check [] coverOfWinterPut = [] := by decide

def urborgScavengers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urborg Scavengers", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ triggeredOr (.enters thisCreature none) [attacks thisCreature]
            (.sequentially
              [ exile (target (.inZone graveyard)),
                .putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature ]),
          .alsoForKeywords
            (.static (onlyWhile (.gains thisCreature (keyword "Flying"))
              (exists_ (.and [.exiledWith thisCreature, .hasKeyword (.the "Flying")]))))
            (["FirstStrike", "DoubleStrike", "Deathtouch", "Haste", "Hexproof", "Indestructible",
              "Lifelink", "Menace", "Reach", "Trample", "Vigilance"].map .the) ],
      power := stat 2, toughness := stat 2 } }

def sengirVampire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sengir Vampire", cost := some [generic 3, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Vampire"],
      text :=
        [ keyword "Flying",
          whenever (.dies (a (.and [creature, happenedToInvolving .damageTaken .thisTurn thisCreature])))
            (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature) ],
      power := stat 4, toughness := stat 4 } }

def mildManneredLibrarian : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mild-Mannered Librarian", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ activatedOnlyOnce (.mana [generic 3, pip .green])
            (.sequentially
              [ .continuously
                  (.becomes thisCreature .sets
                    (.bundle { characteristics := { subtypes := [creatureType "Werewolf"] } } none))
                  none,
                .putCounters (.lit 2) (.printed plusOnePlusOne) it,
                .draw .you (.lit 1) ])
            .oncePerGame ],
      power := stat 1, toughness := stat 1 } }

def infernalVessel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Infernal Vessel", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ triggeredIf (.dies thisCreature) (itIsntA (.hasSubtype (creatureType "Demon")))
            (.sequentially
              [ returnToBattlefieldWithCounters it (ownerOf it) (.lit 2) plusOnePlusOne,
                becomes it { characteristics := { subtypes := [creatureType "Demon"] } } none ]) ],
      power := stat 2, toughness := stat 1 } }

/-- Vivien's Talent -/
def viviensTalentTrigger : Ability :=
  whenever (.enters (a (.and [nontoken, creatureYouControl])) none)
    (.putCounters (.lit 1) (.printed (.named "Loyalty")) (.attachHost .enchanted (.type .planeswalker)))
theorem okViviensTalentTrigger : Ability.check [] viviensTalentTrigger = [] := by decide
/-- Bioessence Hydra -/
def bioessenceHydraTrigger : Ability :=
  whenever
    (counterEvent .put (.named "Loyalty") .many
      (allOf (.and [.hasType .planeswalker, .hasPossessor .controller .you])))
    (.putCounters .thatMuch (.printed plusOnePlusOne) thisCreature)
theorem okBioessenceHydraTrigger : Ability.check [] bioessenceHydraTrigger = [] := by decide
/-- Vraska, Betrayal's Sting -/
def vraskaBetrayalsStingUltimate : Instruction :=
  .if_ (.compareAmt (countersOn (.named "Poison") (target .anyPlayer)) .less (.lit 9))
    (.putCounters .theDifference (.printed (.named "Poison")) they) none
theorem okVraskaBetrayalsStingUltimate :
    Instruction.check [] vraskaBetrayalsStingUltimate = [] := by decide
/-- Vexing Puzzlebox -/
def vexingPuzzleboxCounters : Ability :=
  whenever (.rollsDice .you .many none .anyResult)
    (.putCounters (.theOutcome .rollResult) (.printed (.named "Charge")) thisArtifact)
theorem okVexingPuzzleboxCounters : Ability.check [] vexingPuzzleboxCounters = [] := by decide
def spaceFamilyGoblinsonRoll : Ability :=
  whenever (.rollsDice .you .one none .anyResult)
    (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature)
theorem okSpaceFamilyGoblinsonRoll : Ability.check [] spaceFamilyGoblinsonRoll = [] := by decide
/-- Atomwheel Acrobats -/
def atomwheelAcrobatsRoll : Ability :=
  whenever (youRollResultIn (.range (some 1) (some 2)))
    (.putCounters .thatMuch (.printed plusOnePlusOne) thisCreature)
theorem okAtomwheelAcrobatsRoll : Ability.check [] atomwheelAcrobatsRoll = [] := by decide
/-- Resolute Veggiesaur -/
def resoluteVeggiesaurThirdDie : Ability :=
  whenever (.nthOccurrence (.nth 3) (some .turn) (.rollsDice .you .one none .anyResult))
    (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature)
theorem okResoluteVeggiesaurThirdDie : Ability.check [] resoluteVeggiesaurThirdDie = [] := by decide
/-- Run the Play (Striding Shotcaller's other half) -/
def runThePlayCounters : Instruction :=
  .putCounters (.lit 1) (.printed plusOnePlusOne)
    (.eachOf (.described (.target (.upToOf (.letter .x))) creature))
theorem okRunThePlayCounters : Instruction.check [letterB .x] runThePlayCounters = [] := by decide
/-- Font of Agonies -/
def fontOfAgoniesTrigger : Ability :=
  whenever (.paysLife .you) (.putCounters .thatMuch (.printed (.named "Blood")) thisEnchantment)
theorem okFontOfAgoniesTrigger : Ability.check [] fontOfAgoniesTrigger = [] := by decide
/-- Heart of Kiran -/
def heartOfKiranCrewAltCost : Ability :=
  .static (.altCost (allOf (.and [.abilityHead (.keyword "Crew"), .abilityOf .this]))
    (some (.perform (.removeCounters (some (exactly 1)) (some (.printed (.named "Loyalty")))
      (a (.and [.hasType .planeswalker, .hasPossessor .controller .you]))))))
theorem okHeartOfKiranCrewAltCost : Ability.check [] heartOfKiranCrewAltCost = [] := by decide
/-- Tainted Adversary -/
def taintedAdversaryOffer : Ability :=
  when (.enters thisCreature none)
    (.reflexively (may .you (.pay .you (.mana [generic 2, pip .black]) .anyNumberOfTimes))
      (.putCounters .thatMuch (.printed plusOnePlusOne) thisCreature))
theorem okTaintedAdversaryOffer : Ability.check [] taintedAdversaryOffer = [] := by decide
/-- Korvold, Gleeful Glutton -/
def korvoldCombatTrigger : Ability :=
  whenever (dealsCombatDamage thisCreature (a .anyPlayer))
    (.sequentially
      [ .putCounters (.letter .x) (.printed plusOnePlusOne) thisCreature,
        .draw .you (.letter .x),
        .define .x (.distinctCount .permanentType (allOf (.inZone (graveyardOf .you)))) ])
theorem okKorvoldCombatTrigger : Ability.check [] korvoldCombatTrigger = [] := by decide
/-- Mirelurk Queen -/
def mirelurkQueenTrigger : Ability :=
  triggeredOnlyOnce
    (.verbedEvent none (.action "Mill")
      (some (counted (atLeast 1) (.and [.not land, .inZone library]))) none)
    .oncePerTurn
    (.sequentially [.draw .you (.lit 1), .putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature])
theorem okMirelurkQueenTrigger : Ability.check [] mirelurkQueenTrigger = [] := by decide

/-- Whirling Dervish -/
def whirlingDervish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Whirling Dervish", cost := some [pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Monk"],
      text :=
        [ keywordQuality "Protection" (.colorIs .black),
          triggeredIf (.beginningOf .the .endStep (.byPlayer (each .anyPlayer)))
            (happenedInvolving .damageDealing thisCreature .thisTurn anOpponent)
            (.putCounters (.lit 1) (.printed plusOnePlusOne) it) ],
      power := stat 1, toughness := stat 1 } }

/-- Ichor Shade -/
def ichorShade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ichor Shade", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Shade"],
      text :=
        [ triggeredIf (.beginningOf .the .endStep (.byPlayer .you))
            (.happened (a (.or [artifact, creature]))
              (.mk .placement .thisTurn
                (some (.intoZone graveyard (some (.fromZones (.zones [battlefield]) none))))))
            (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature) ],
      power := stat 2, toughness := stat 3 } }

/-- Asmira, Holy Avenger -/
def asmiraHolyAvenger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Asmira, Holy Avenger", cost := some [generic 2, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keyword "Flying",
          at_ (.beginningOf .the .endStep (.byPlayer (each .anyPlayer)))
            (.putCounters
              (.eventTally .count (a creature)
                (.mk .placement .thisTurn
                  (some (.intoZone (graveyardOf .you) (some (.fromZones (.zones [battlefield]) none))))))
              (.printed plusOnePlusOne) thisCreature) ],
      power := stat 2, toughness := stat 3 } }

/-- Thought Sponge -/
def thoughtSponge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thought Sponge", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Sponge"],
      text :=
        [ keyword "Flash",
          .static (entersWithCounters thisCreature greatestCardsAnOpponentDrew plusOnePlusOne),
          when (.dies thisCreature) (.draw .you (.statOf (.stat .power) thisCreature)) ],
      power := stat 1, toughness := stat 1 } }

/-- Skeleton Ship -/
def skeletonShip : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Skeleton Ship", cost := some [generic 3, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Skeleton"],
      text :=
        [ when
            (.stateHolds
              (.not (exists_ (.and [land, .hasSubtype (landType "Island"), .hasPossessor .controller .you]))))
            (sacrifice .you thisCreature),
          activated .tapSymbol (.putCounters (.lit 1) (.printed minusOneMinusOne) (target creature)) ],
      power := stat 0, toughness := stat 3 } }

/-- Contractual Safeguard -/
def contractualSafeguardPass : Instruction :=
  .sequentially
    [ choose (a (.counterKindOn (a creatureYouControl))),
      .putCounters (.lit 1) .bound (each (otherCreatureYouControl it)) ]
theorem okContractualSafeguardPass : Instruction.check [] contractualSafeguardPass = [] := by decide

/-- Feral Contest -/
def feralContest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Feral Contest", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ .putCounters (.lit 1) (.printed plusOnePlusOne) (target creatureYouControl),
              mustBlockIt (target (.and [creature, .other])) (some .thisTurn) ]) ] } }

/-- Thranduil's Company -/
def thranduilsCompany : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thranduil's Company", cost := some [generic 2, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Elf", creatureType "Soldier"],
      text :=
        [ .static (onlyWhile (mayPlayAdditionalLands .you (exactly 1))
            (exists_ (.and [ otherCreature thisCreature, .hasSubtype (creatureType "Elf"),
                             .hasPossessor .controller .you ]))),
          abilityWord "landfall"
            (whenever (.enters (a (.and [land, .hasPossessor .controller .you])) none)
              (.sequentially
                [ .putCounters (.lit 2) (.printed plusOnePlusOne) (target creatureYouControl),
                  gains
                    (itPrior (.putCounters (.lit 2) (.printed plusOnePlusOne) (target creatureYouControl)))
                    (keyword "Vigilance") (some untilEndOfTurn) ])) ],
      power := stat 3, toughness := stat 4 } }

/-- Stunning Shot -/
def stunningShot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stunning Shot", cost := some [generic 1, pip .white], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ .putCounters (.lit 2) (.printed plusOnePlusOne)
                (.described (.target (upTo 1)) creatureYouControl),
              tap (.described (.target (upTo 1)) (.and [creature, .hasPossessor .controller anOpponent])),
              .putCounters (.lit 1) (.printed (.named "Stun"))
                (itPrior
                  (tap (.described (.target (upTo 1))
                    (.and [creature, .hasPossessor .controller anOpponent])))) ]) ] } }

/-- Stonebinder's Familiar -/
def stonebindersFamiliar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stonebinder's Familiar", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Spirit", creatureType "Dog"],
      text :=
        [ .triggered (.putInto (counted (atLeast 1) .isCard) exileZone none) [] none []
            (some (.duringPart .turn (some .you))) (some .oncePerTurn) none
            (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature) ],
      power := stat 1, toughness := stat 1 } }

/-- Nazgûl -/
def nazgulRingTrigger : Ability :=
  whenever (.verbedEvent (some .you) (.action "The Ring Tempts You") none none)
    (.putCounters (.lit 1) (.printed plusOnePlusOne)
      (each (.and [.hasSubtype (creatureType "Wraith"), .hasPossessor .controller .you])))
theorem okNazgulRingTrigger : Ability.check [] nazgulRingTrigger = [] := by decide

/-- Captain Marvel, Apex Avenger -/
def captainMarvelApexAvenger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Captain Marvel, Apex Avenger", cost := some [generic 5, pip .red, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Kree", creatureType "Hero"],
      text :=
        [ keyword "Flying", keyword "DoubleStrike", keyword "Indestructible",
          triggeredIf
            (.counterEvent .put none (a (.and [creature, .otherThan .this])) .many (some .you) false)
            (.matches it (.not (.hasSubtype (creatureType "Kree"))))
            (may .you (.putCounters .thatMuch .those .this)) ],
      power := stat 4, toughness := stat 4 } }

/-- Blood Spatter Analysis -/
def bloodSpatterAnalysis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Spatter Analysis", cost := some [pip .black, pip .red], types := [.enchantment],
      text :=
        [ when (.enters thisEnchantment none)
            (.dealDamage thisEnchantment (.lit 3)
              (target (.and [creature, .hasPossessor .controller anOpponent]))),
          whenever (.dies (counted (atLeast 1) creature))
            (.sequentially
              [ mills .you (.lit 1) .you,
                .putCounters (.lit 1) (.printed (.named "Bloodstain")) thisEnchantment,
                .reflexively
                  (.onlyIf (sacrifice .you thisEnchantment)
                    (.compareAmt (countersOn (.named "Bloodstain") thisEnchantment) .atLeast (.lit 5))
                    none)
                  (move (target (.and [creature, .isCard, .inZone (graveyardOf .you)])) hand) ]) ] } }

/-- Bewitching Leechcraft -/
def bewitchingLeechcraft : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bewitching Leechcraft", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          when (.enters thisAura none) (tap (.attachHost .enchanted (.type .creature))),
          .static (.gains (.attachHost .enchanted (.type .creature))
            (.static (.intercepts (.statusEvent thisCreature .untapped) []
              (some (.duringPart .untapStep (some .you)))
              (.ifDone
                (.removeCounters (some (exactly 1)) (some (.printed plusOnePlusOne)) thisCreature)
                (some (untap thisCreature)) none)
              .repeatedly none))) ] } }

def agentOfTheShadowThievesGrantedTrigger : Ability :=
  .triggered (.combat .attackerOf thisCreature (some (a .anyPlayer))) [] none [] none none
    (some (.not (exists_ (.and [ .opponent,
                                 .compare [.playerStat .lifeTotal] .greater
                                   (lifeTotalOf (that .player)) ]))))
    (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature)
theorem okAgentOfTheShadowThievesGrantedTrigger :
    Ability.check [] agentOfTheShadowThievesGrantedTrigger = [] := by decide

/-- Bloodline Pretender -/
def bloodlinePretender : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodline Pretender", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ keyword "Changeling",
          .static (entersChoosing thisCreature (.subtype .creature)),
          whenever
            (.enters (a (.and [ creature, .hasPossessor .controller .you, .otherThan thisCreature,
                                ofChosen (.subtype .creature) ])) none)
            (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature) ],
      power := stat 2, toughness := stat 2 } }

/-- Chamber Sentry -/
def chamberSentryDamage : Ability :=
  activated
    (.compound
      [ .mana [.variable], .tapSymbol,
        .perform (.removeCounters (some (.exactlyOf (.letter .x))) (some (.printed plusOnePlusOne))
          thisCreature) ])
    (.dealDamage .this (.letter .x) (target anyTarget))
theorem okChamberSentryDamage : Ability.check [] chamberSentryDamage = [] := by decide

/-- Menacing Ogre -/
def menacingOgre : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Menacing Ogre", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Ogre"],
      text :=
        [ keyword "Trample", keyword "Haste",
          when (.enters thisCreature none)
            (.sequentially
              [ secretlyChooses (each .anyPlayer) (a (quality .number)),
                .choicesRevealed .numbers,
                losesLife (each (.and [.anyPlayer, .choseExtreme .max])) .thatMuch,
                if_ (.matches .you (.choseExtreme .max))
                  (.putCounters (.lit 2) (.printed plusOnePlusOne) thisCreature) ]) ],
      power := stat 3, toughness := stat 3 } }

/-- Charnel Troll -/
def charnelTroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Charnel Troll", cost := some [generic 1, pip .black, pip .green], types := [.creature],
      subtypes := [creatureType "Troll"],
      text :=
        [ keyword "Trample",
          at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (.ifDone (exile (a (.and [creature, .inZone (graveyardOf .you)])))
              (some (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature))
              (some (sacrifice .you thisCreature))),
          activated
            (.compound
              [ .mana [pip .black, pip .green],
                .perform (discard .you (a (.and [creature, .inZone hand]))) ])
            (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature) ],
      power := stat 4, toughness := stat 4 } }

/-- Mistbreath Elder -/
def mistbreathElder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistbreath Elder", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Frog", creatureType "Warrior"],
      text :=
        [ at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (.ifDone (returnTo (a (otherCreatureYouControl thisCreature)) hand [])
              (some (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature))
              (some (may .you (returnTo thisCreature hand [])))) ],
      power := stat 2, toughness := stat 2 } }

/-- Dark Depths -/
def darkDepths : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dark Depths", supertypes := [.legendary, .snow], types := [.land],
      text :=
        [ .static (entersWithCounters thisLand (.lit 10) (.named "Ice")),
          activated (.mana [generic 3])
            (removeCounters (exactly 1) (some (.printed (.named "Ice"))) thisLand),
          when (.stateHolds (.not (.matches thisLand (.hasCounters (some (.named "Ice"))))))
            (.ifDone (sacrifice .you thisLand) (some (create (.lit 1) maritLage)) none) ] } }

/-- Thallid -/
def thallid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thallid", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Fungus"],
      text :=
        [ at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (.putCounters (.lit 1) (.printed (.named "Spore")) thisCreature),
          activated
            (.perform (.removeCounters (some (exactly 3)) (some (.printed (.named "Spore"))) thisCreature))
            (create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Saproling"])) ],
      power := stat 1, toughness := stat 1 } }

/-- Aether Chaser -/
def aetherChaserEnergy : Ability :=
  when (.enters thisCreature none) (.putCounters (.lit 2) (.printed (.named "Energy")) .you)
theorem okAetherChaserEnergy : Ability.check [] aetherChaserEnergy = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossEntersOiled : StaticSpec :=
  entersWithCounters thisCreature (.lit 4) (.named "Oil")
theorem okArchfiendOfTheDrossEntersOiled :
    StaticSpec.check [] archfiendOfTheDrossEntersOiled = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossOilUpkeep : Ability :=
  at_ (.beginningOf .the .upkeep (.byPlayer .you))
    (.removeCounters (some (exactly 1)) (some (.printed (.named "Oil"))) thisCreature)
theorem okArchfiendOfTheDrossOilUpkeep : Ability.check [] archfiendOfTheDrossOilUpkeep = [] := by
  decide
def neurokTransmuter : Instruction :=
  becomes (target creature) { characteristics := { types := [.artifact] } } (some untilEndOfTurn)
theorem okNeurokTransmuter : Instruction.check [] neurokTransmuter = [] := by decide
def syphonMind : Instruction :=
  .sequentially
    [ discard (each otherPlayer) (a (.inZone hand)),
      .forEachOf (theVerbed (.action "Discard") .card .thisWay .many) (.draw .you (.lit 1)) ]
theorem okSyphonMind : Instruction.check [] syphonMind = [] := by decide
def peek : Instruction := .sequentially [lookAtHandOf (target .anyPlayer), .draw .you (.lit 1)]
theorem okPeek : Instruction.check [] peek = [] := by decide
/-- Bumi, King of Three Trials -/
def bumiScryMode : Instruction := scry (target .anyPlayer) (.lit 3)
theorem okBumiScryMode : Instruction.check [] bumiScryMode = [] := by decide
/-- Final Act -/
def finalActCounterMode : Instruction := losesAllCounters (each .opponent) none
theorem okFinalActCounterMode : Instruction.check [] finalActCounterMode = [] := by decide
def leeches : Instruction :=
  losesAllCounters (target .anyPlayer) (some (.printed (.named "Poison")))
theorem okLeeches : Instruction.check [] leeches = [] := by decide

/-- Woeleecher -/
def woeleecherWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Woeleecher", cost := some [generic 5, pip .white], types := [.creature],
      subtypes := [creatureType "Elemental"], text := [woeleecher],
      power := stat 3, toughness := stat 5 } }

end Semantics.Cards
