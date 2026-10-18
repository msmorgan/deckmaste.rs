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
    (some (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many))
    (some (removeCounters (exactly 1) (some (printedKind p1p1Counter)) thisCreature .many true))
theorem okCrovaxTheCursed : Action.check (DRS.ofList []) crovaxTheCursed = [] := by decide
def additiveEvolution : Action :=
  sequentially
    [ createToken (creatureToken 0 0 [.green, .blue] [creatureType "Fractal"]),
      putCounters (.lit 3) (printedKind p1p1Counter) it .many ]
theorem okAdditiveEvolution : Action.check (DRS.ofList []) additiveEvolution = [] := by decide
def battlegrowth : Action := putCounters (.lit 1) (printedKind p1p1Counter) (target isCreature) .many
theorem okBattlegrowth : Action.check (DRS.ofList []) battlegrowth = [] := by decide
def chainbreaker : Action :=
  removeCounters (exactly 1) (some (printedKind m1m1Counter)) (target isCreature) .many true
theorem okChainbreaker : Action.check (DRS.ofList []) chainbreaker = [] := by decide
def kaitoBaneOfNightmares : Action :=
  sequentially
    [setStatus .tapped (target isCreature), putCounters (.lit 2) (printedKind (.named "stunCounter")) it .many]
theorem okKaitoBaneOfNightmares : Action.check (DRS.ofList []) kaitoBaneOfNightmares = [] := by decide
def jhoiraOfTheGhitu : Ability :=
  activated
    (compound [mana [generic 2], perform (exile (a (and [not isLand, in_ (handOf you)])))])
    (putCounters (.lit 4) (printedKind (.named "timeCounter"))
      (theVerbed (.action "exile") .one (some .card)) .many)
theorem okJhoiraOfTheGhitu : Ability.check (DRS.ofList []) jhoiraOfTheGhitu = [] := by decide
def alaundoTheSeer : Action :=
  removeCounters (exactly 1) (some (printedKind (.named "timeCounter"))) (each (and [hasPossessor .owner you, in_ exileZone])) .many true
theorem okAlaundoTheSeer : Action.check (DRS.ofList []) alaundoTheSeer = [] := by decide

def daydream : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daydream", cost := some [pip .white], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ exile (target (and [isCreature, youControl])),
              returnToBattlefieldWithCounters (that .card) (ownerOf (that .card)) (.lit 1)
                p1p1Counter ]),
          keywordCosting (.named "flashback") (mana [generic 2, pip .white]) ] } }

def ashnodsTransmogrant : Ability :=
  activated (compound [tapSymbol, perform (sacrifice thisArtifact)])
    (sequentially
      [ putCounters (.lit 1) (printedKind p1p1Counter) (target (and [isCreature, not isArtifact])) .many,
        become (that (.type .creature)) { characteristics := { types := [.artifact] } } none ])
theorem okAshnodsTransmogrant : Ability.check (DRS.ofList []) ashnodsTransmogrant = [] := by decide
def azulaAlwaysLies : Action :=
  chooseModes (oneThrough 2)
    [ get (target isCreature) (down (.lit 1)) (down (.lit 1)) (some untilEndOfTurn),
      putCounters (.lit 1) (printedKind p1p1Counter) (target isCreature) .many ]
theorem okAzulaAlwaysLies : Action.check (DRS.ofList []) azulaAlwaysLies = [] := by decide
def bellowingAegisaur : Action :=
  putCounters (.lit 1) (printedKind p1p1Counter) (each ((and [isCreature, youControl, otherThan thisCreature]))) .many
theorem okBellowingAegisaur : Action.check (DRS.ofList []) bellowingAegisaur = [] := by decide
def carnifexDemon : Action :=
  putCounters (.lit 1) (printedKind m1m1Counter) (each ((and [isCreature, otherThan thisCreature]))) .many
theorem okCarnifexDemon : Action.check (DRS.ofList []) carnifexDemon = [] := by decide
def ajaniAdversaryOfTyrants : Action :=
  putCounters (.lit 1) (printedKind p1p1Counter) (eachOf (targets (upTo 2) isCreature)) .many
theorem okAjaniAdversaryOfTyrants : Action.check (DRS.ofList []) ajaniAdversaryOfTyrants = [] := by decide
def naturesPanoply : Action :=
  sequentially
    [ choose (targets anyNumber isCreature),
      putCounters (.lit 1) (printedKind p1p1Counter) (eachOf them) .many ]
theorem okNaturesPanoply : Action.check (DRS.ofList []) naturesPanoply = [] := by decide
def armamentCorps : Action :=
  distributeCounters (.lit 2) p1p1Counter
    (targets (oneThrough 2) (and [isCreature, youControl]))
theorem okArmamentCorps : Action.check (DRS.ofList []) armamentCorps = [] := by decide
def savagebornHydra : Ability :=
  activatedOnlyDuring (mana [generic 1, hybridPip .red .green])
    (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many) asSorcery
theorem okSavagebornHydra : Ability.check (DRS.ofList []) savagebornHydra = [] := by decide
def woeleecher : Ability :=
  activated (compound [mana [pip .white], tapSymbol])
    (doIfDone
      (removeCounters (exactly 1) (some (printedKind m1m1Counter)) (target isCreature) .many true)
      (some (gainLife (.lit 2))))
theorem okWoeleecher : Ability.check (DRS.ofList []) woeleecher = [] := by decide
def anointerOfValor : Ability :=
  whenever (does (actor (a isCreature)) attack)
    (offerWhen (pay (mana [generic 3]) .once)
      (putCounters (.lit 1) (printedKind p1p1Counter) (that (.type .creature)) .many))
theorem okAnointerOfValor : Ability.check (DRS.ofList []) anointerOfValor = [] := by decide
def avianOddity : Action :=
  putCounters (.lit 1) (printedKind flyingCounter) (target (and [isCreature, youControl])) .many
theorem okAvianOddity : Action.check (DRS.ofList []) avianOddity = [] := by decide
/-- Song of Eärendil -/
def songOfEarendil : Action :=
  putCounters (.lit 1) (printedKind flyingCounter)
    (each (and [isCreature, hasPossessor .controller you, not (hasKeyword (.the (.named "flying")))])) .many
theorem okSongOfEarendil : Action.check (DRS.ofList []) songOfEarendil = [] := by decide
def gideonsAvenger : Ability :=
  whenever (does any (setStatus .tapped (a (and [isCreature, hasPossessor .controller anOpponent]))))
    (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many)
theorem okGideonsAvenger : Ability.check (DRS.ofList []) gideonsAvenger = [] := by decide
def prologueToPhyresis : Action :=
  putCounters (.lit 1) (printedKind (.named "poison")) (each (isOpponentOf you)) .many
theorem okPrologueToPhyresis : Action.check (DRS.ofList []) prologueToPhyresis = [] := by decide
def screechingScorchbeast : Ability :=
  whenever (does (actor thisCreature) attack) (putCounters (.lit 2) (printedKind (.named "radCounter")) (each isPlayer) .many)
theorem okScreechingScorchbeast : Ability.check (DRS.ofList []) screechingScorchbeast = [] := by decide
def merenOfClanNelToth : Ability :=
  whenever (does (actor (a ((and [isCreature, youControl, otherThan thisCreature])))) die)
    (putCounters (.lit 1) (printedKind (.named "experience")) you .many)
theorem okMerenOfClanNelToth : Ability.check (DRS.ofList []) merenOfClanNelToth = [] := by decide
def kratosStoicFather : Ability :=
  at_ (beginningOfYour .endStep)
    (putCounters (countersOn (.named "experience") you) (printedKind p1p1Counter)
      (target isCreature) .many)
theorem okKratosStoicFather : Ability.check (DRS.ofList []) kratosStoicFather = [] := by decide
def vashtaNerada : Ability :=
  triggeredIf (beginningOfPossessed .endStep (each isPlayer))
    (happened (move (a isCreature)
      (battlefield) (some graveyard) .departure) any soFarThisTurn)
    (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many)
theorem okVashtaNerada : Ability.check (DRS.ofList []) vashtaNerada = [] := by decide
def furiousSpinesplitter : Ability :=
  at_ (beginningOfYour .endStep)
    (putCounters (forEach (.lit 1) (and
      [(isOpponentOf you), happenedTo any (dealDamage anyAmount (relative
      .player)) soFarThisTurn]))
      (printedKind p1p1Counter) thisCreature .many)
theorem okFuriousSpinesplitter : Ability.check (DRS.ofList []) furiousSpinesplitter = [] := by decide
def paladinOfAtonement : Ability :=
  triggeredIf (beginningOfPossessed .upkeep (each isPlayer))
    (happened (changeLife (down anyAmount)) (actor you) lastTurn) (putCounters (.lit 1) (printedKind
      p1p1Counter) thisCreature .many)
theorem okPaladinOfAtonement : Ability.check (DRS.ofList []) paladinOfAtonement = [] := by decide
def throneWarden : Ability :=
  triggeredIf (beginningOfYour .endStep)
    (matches_ you (hasDesignation (.named "monarch")))
    (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many)
theorem okThroneWarden : Ability.check (DRS.ofList []) throneWarden = [] := by decide
def bloodTyrant : Ability :=
  whenever (does (actor (a isPlayer)) loseGame) (putCounters (.lit 5) (printedKind p1p1Counter) thisCreature .many)
theorem okBloodTyrant : Ability.check (DRS.ofList []) bloodTyrant = [] := by decide

def stagBeetle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stag Beetle", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Insect"],
      text :=
        [ static (conjunction [ entersWithCounters thisCreature (letter .x) p1p1Counter,
              letterDefinition .x (countOf ((and [isCreature, otherThan thisCreature]))) ]) ],
      power := stat 0, toughness := stat 0 } }

def toweringTitan : Ability :=
  static (conjunction [ entersWithCounters thisCreature (letter .x) p1p1Counter,
      letterDefinition .x
        (aggregate .sum (.stat .toughness) ((and [isCreature, youControl, otherThan thisCreature]))) ])
theorem okToweringTitan : Ability.check (DRS.ofList []) toweringTitan = [] := by decide

def coretapper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coretapper", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Myr"],
      text :=
        [ activated tapSymbol (putCounters (.lit 1) (printedKind (.named "chargeCounter")) (target isArtifact) .many),
          activated (perform (sacrifice thisCreature))
            (putCounters (.lit 2) (printedKind (.named "chargeCounter")) (target isArtifact) .many) ],
      power := stat 1, toughness := stat 1 } }

def divineIntervention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Divine Intervention", cost := some [generic 6, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ static (entersWithCounters thisEnchantment (.lit 2) (.named "interventionCounter")),
          at_ (beginningOfYour .upkeep)
            (removeCounters (exactly 1) (some (printedKind (.named "interventionCounter"))) thisEnchantment .many true),
          when (lastCounterRemovedBy (.named "interventionCounter") thisEnchantment you) drawGame ] } }

def celestialConvergence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Celestial Convergence", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ static (entersWithCounters thisEnchantment (.lit 7) (.named "omenCounter")),
          at_ (beginningOfYour .upkeep)
            (sequentially
              [ removeCounters (exactly 1) (some (printedKind (.named "omenCounter"))) thisEnchantment .many true,
                doIf (compareAmt (countersOn (.named "omenCounter") thisEnchantment) .atMost (.lit 0))
                  (instruct (the (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer])) winGame),
                doIf (compareAmt
                        (countOf (and [isPlayer, superlative .max (.playerStat .lifeTotal) isPlayer]))
                        .atLeast (.lit 2))
                  drawGame ]) ] } }

def curseOfVengeance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curse of Vengeance", cost := some [pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura", enchantmentType "Curse"],
      text :=
        [ keywordSubject (.named "enchant") isPlayer,
          whenever (does (actor (attachHost .enchanted .player)) (procedure .cast ((a isSpell)) itsOwnCost))
            (putCounters (.lit 1) (printedKind (.named "spiteCounter")) thisAura .many),
          when (does (actor (attachHost .enchanted .player)) loseGame)
            (sequentially
              [ gainLife (letter .x), draw (letter .x),
                define .x (countersOn (.named "spiteCounter") thisAura) ]) ] } }

def passagewaySeer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Passageway Seer", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [creatureType "Tiefling", creatureType "Warlock"],
      text :=
        [ keyword (.named "lifelink"),
          when (does (actor thisCreature) enter) (gainDesignation you (.named "initiative")),
          triggeredIf (beginningOfYour .endStep)
            (matches_ you (hasDesignation (.named "initiative")))
            (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 2, toughness := stat 2 } }

def chainsaw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chainsaw", cost := some [generic 1, pip .red], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ when (does (actor thisEquipment) enter)
            (instruct it (dealDamage (.lit 3) (targets (upTo 1) isCreature) .noncombatOnly)),
          whenever (does (actor (counted (atLeast 1) isCreature)) die)
            (putCounters (.lit 1) (printedKind (.named "revCounter")) thisEquipment .many),
          static (conjunction [ getsPt (attachHost .equipped (.type .creature)) (up (letter .x)) (up (.lit 0)),
              letterDefinition .x (countersOn (.named "revCounter") thisEquipment) ]),
          keywordCosting (.named "equip") (mana [generic 3]) ] } }

/-- Soul's Might -/
def soulsMight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soul's Might", cost := some [generic 4, pip .green], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ putCounters (letter .x) (printedKind p1p1Counter) (target isCreature) .many,
              define .x (statOf (.stat .power) (that (.type .creature))) ]) ] } }

def woodlandChampion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Woodland Champion", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Scout"],
      text :=
        [ whenever (does (actor (counted (atLeast 1) (and [isToken, hasPossessor .controller you]))) enter)
            (putCounters groupSize (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 2, toughness := stat 2 } }

def voraciousBrood : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Voracious Brood", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Alien", creatureType "Insect"],
      text :=
        [ static (entersWithCounters thisCreature
            (forEach (.lit 1) (and [isCreature, in_ (graveyardOf you)])) p1p1Counter),
          whenever (does any (putInto (counted (atLeast 1) (and [isCard, isCreature])) (graveyardOf you) (from_ := some any)))
            (putCounters groupSize (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 1, toughness := stat 1 } }

def herdBaloth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Herd Baloth", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ whenever (counterEvent .put p1p1Counter .many thisCreature)
            (may (createToken (creatureToken 4 4 [.green] [creatureType "Beast"]))) ],
      power := stat 4, toughness := stat 4 } }

def flourishingDefenses : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Flourishing Defenses", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ whenever (counterEvent .put m1m1Counter .one (a isCreature))
            (may
              (createToken (creatureToken 1 1 [.green] [creatureType "Elf", creatureType
                  "Warrior"]))) ] } }

def rakshasaVizier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rakshasa Vizier", cost := some [generic 2, pip .black, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Demon"],
      text :=
        [ whenever
            (does any (putInto (counted (atLeast 1) (and [isCard, in_ (graveyardOf you)])) exileZone (from_ := some (graveyardOf you))))
            (putCounters groupSize (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 4, toughness := stat 4 } }

def foeLiage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foe-liage", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Plant", creatureType "Mutant"],
      text :=
        [ triggeredOnlyDuring (does (actor (a isLand)) enter) (duringPart .turn (some you))
            (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 3, toughness := stat 3 } }

def hardenedScales : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hardened Scales", cost := some [pip .green], types := [.enchantment],
      text :=
        [ static (instead (would any (putCounters anyAmount (printedKind p1p1Counter) (a (and [isCreature, youControl])) .many false)) (putCounters (plus thatMuch (.lit 1)) (printedKind p1p1Counter) it .many)) ] } }

def branchingEvolution : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Branching Evolution", cost := some [generic 2, pip .green], types := [.enchantment],
      text :=
        [ static (instead (would any (putCounters anyAmount (printedKind p1p1Counter) (a (and [isCreature, youControl])) .many false)) (putCounters (times (.lit 2) thatMuch) (printedKind p1p1Counter) (that (.type .creature)) .many)) ] } }

/-- Corpsejack Menace -/
def corpsejackMenace : Ability :=
  static (instead (would any (putCounters anyAmount (printedKind p1p1Counter) (a (and [isCreature, youControl])) .many false)) (putCounters (times (.lit 2) thatMuch) (printedKind p1p1Counter) it .many))
theorem okCorpsejackMenace : Ability.check (DRS.ofList []) corpsejackMenace = [] := by decide

/-- Doubling Season -/
def doublingSeason : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Doubling Season", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ static (instead (would any (createObject anyAmount (token (copyOf (counted (atLeast 1) isToken)) []) true (some you))) (create (times (.lit 2) groupSize) asThose [])),
          static (instead (would any (putCounters anyAmount anyKind (a (and [isPermanent, hasPossessor .controller you])) .many)) (putCounters (times (.lit 2) thatMuch) thoseCounters (that .permanent) .many)) ] } }

/-- Doc Samson, Super Psychiatrist -/
def docSamsonDistributive : Ability :=
  static (instead (would (actor you) (putCounters anyAmount anyKind (a (and [isPermanent, hasPossessor .controller you])) .many false)) (putCounters (plus thatMuch (.lit 1)) thoseCounters (that .permanent) .many))
theorem okDocSamsonDistributive : Ability.check (DRS.ofList []) docSamsonDistributive = [] := by decide

/-- Winding Constrictor -/
def windingConstrictor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Winding Constrictor", cost := some [pip .black, pip .green], types := [.creature],
      subtypes := [creatureType "Snake"],
      text :=
        [ static (instead (would any (putCounters anyAmount anyKind (a (and [or [isArtifact, isCreature], hasPossessor .controller you])) .many false)) (putCounters (plus thatMuch (.lit 1)) thoseCounters (that .permanent) .many)),
          static (instead (would any (putCounters anyAmount anyKind you .many false)) (putCounters (plus thatMuch (.lit 1)) thoseCounters you .many)) ],
      power := stat 2, toughness := stat 3 } }

/-- Aragorn, Company Leader -/
def aragornDistributive : Ability :=
  whenever (manyBareCountersPutBy you thisCreature)
    (putCounters (.lit 1) thoseCounters (targets (upTo 1) ((and [isCreature, otherThan thisCreature]))) .many)
theorem okAragornDistributive : Ability.check (DRS.ofList []) aragornDistributive = [] := by decide
/-- Captain Marvel, Apex Avenger -/
def captainMarvelSameKinds : Ability :=
  triggeredIf (manyBareCountersPutBy you (a ((and [isCreature, otherThan thisCreature]))))
    (not (matches_ it (hasSubtype (creatureType "Kree"))))
    (may (putCounters thatMuch thoseCounters thisCreature .many))
theorem okCaptainMarvelSameKinds : Ability.check (DRS.ofList []) captainMarvelSameKinds = [] := by decide
/-- Denry Klin, Editor in Chief -/
def denryKlinSameKinds : Ability :=
  triggeredIf (does (actor (a (and [isNontoken, and [isCreature, youControl]]))) enter)
    (matches_ thisCreature hasCounter)
    (putCounters (.lit 1) (sameKindAs thisCreature) (that (.type .creature)) .many)
theorem okDenryKlinSameKinds : Ability.check (DRS.ofList []) denryKlinSameKinds = [] := by decide

/-- Denry Klin, Editor in Chief -/
def denryKlin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Denry Klin, Editor in Chief", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Cat", creatureType "Advisor"],
      text :=
        [ static (entryRider thisCreature
            (withCounters (.lit 1)
              (chosenKind [p1p1Counter, .named "firstStrikeCounter", .named "vigilanceCounter"]) .fresh)),
          denryKlinSameKinds ],
      power := stat 2, toughness := stat 2 } }

/-- Me, the Immortal -/
def meTheImmortalCounterMenu : Ability :=
  at_ (beginningOfYour .combat)
    (putCounters (.lit 1)
      (chosenKind [p1p1Counter, .named "firstStrikeCounter", .named "vigilanceCounter", .named "menaceCounter"])
      thisCreature .many)
theorem okMeTheImmortalCounterMenu : Ability.check (DRS.ofList []) meTheImmortalCounterMenu = [] := by decide

/-- Star Pupil -/
def starPupil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Star Pupil", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ static (entersWithCounters thisCreature (.lit 1) p1p1Counter),
          when (does (actor thisCreature) die) (putCounters (.lit 1) (sameKindAs it) (target (and [isCreature, youControl])) .many) ],
      power := stat 0, toughness := stat 0 } }

/-- Tromell, Seymour's Butler -/
def tromell : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tromell, Seymour's Butler", cost := some [generic 2, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Advisor"],
      text :=
        [ static (entersWithAdditionalCounters
            (each (and [isCreature, isNontoken, hasPossessor .controller you, otherThan thisCreature]))
            (.lit 1) p1p1Counter),
          activated (compound [mana [generic 1], tapSymbol])
            (sequentially
              [ repeatTimes (letter .x) proliferate,
                define .x
                  (countOf (and [ isNontoken, isCreature, hasPossessor .controller you,
                                   did (⟨any, (move (relative .object) any (some battlefield) .arrival), (within soFarThisTurn)⟩) ])) ]) ],
      power := stat 2, toughness := stat 3 } }

/-- Runadi, Behemoth Caller -/
def runadiBehemothCaller : Ability :=
  static (abilityGrant
    (allOf (and [ isCreature, hasPossessor .controller you,
                   compare .atLeast (.lit 3) (axes := [.counter p1p1Counter]) ]))
    (keyword (.named "haste")))
theorem okRunadiBehemothCaller : Ability.check (DRS.ofList []) runadiBehemothCaller = [] := by decide
/-- Boon of Safety -/
def boonOfSafetyPut : Action :=
  putCounters (.lit 1) (printedKind (.named "shieldCounter")) (target isCreature) .many
theorem okBoonOfSafetyPut : Action.check (DRS.ofList []) boonOfSafetyPut = [] := by decide
/-- Vivien's Talent and Teferi's Talent -/
def talentLoyaltyPut : Action :=
  putCounters (.lit 1) (printedKind (.named "loyaltyCounter")) (attachHost .enchanted (.type .planeswalker)) .many
theorem okTalentLoyaltyPut : Action.check (DRS.ofList []) talentLoyaltyPut = [] := by decide
/-- Simic Fluxmage -/
def simicFluxmageMove : Action :=
  moveCounters (.lit 1) thisCreature (target isCreature) (some (printedKind p1p1Counter))
theorem okSimicFluxmageMove : Action.check (DRS.ofList []) simicFluxmageMove = [] := by decide
/-- Rikku, Resourceful Guardian -/
def rikkuStealMove : Action :=
  moveCounters (.lit 1) (target (and [isCreature, hasPossessor .controller anOpponent])) (target (and [isCreature, youControl]))
theorem okRikkuStealMove : Action.check (DRS.ofList []) rikkuStealMove = [] := by decide
/-- Littjara Mirrorlake -/
def littjaraMirrorlakeCopy : Action :=
  create (.lit 1)
    (copyOf (target (and [isCreature, hasPossessor .controller you]))
      [withEntryCounters (.lit 1) p1p1Counter .additional])
    []
theorem okLittjaraMirrorlakeCopy : Action.check (DRS.ofList []) littjaraMirrorlakeCopy = [] := by decide

def shalaiAndHallar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shalai and Hallar", cost := some [generic 1, pip .red, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Angel", creatureType "Elf"],
      text :=
        [ keyword (.named "flying"), keyword (.named "vigilance"),
          whenever (counterEvent .put p1p1Counter .many (a (and [isCreature, youControl])))
            (instruct thisCreature (dealDamage thatMuch (target (isOpponentOf you)) .noncombatOnly)) ],
      power := stat 3, toughness := stat 3 } }

def primalVigor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Primal Vigor", cost := some [generic 4, pip .green], types := [.enchantment],
      text :=
        [ static (instead (would any (createObject anyAmount (token (copyOf (counted (atLeast 1) isToken)) []) false)) (create (times (.lit 2) groupSize) asThose [])),
          static (instead (would any (putCounters anyAmount (printedKind p1p1Counter) (a isCreature) .many false)) (putCounters (times (.lit 2) thatMuch) (printedKind p1p1Counter) (that (.type .creature)) .many)) ] } }

/-- Patrolling Peacemaker -/
def patrollingPeacemaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Patrolling Peacemaker", cost := some [generic 2, pip .white],
      types := [.artifact, .creature], subtypes := [creatureType "Robot", creatureType "Soldier"],
      text :=
        [ static (entersWithCounters thisCreature (.lit 2) p1p1Counter),
          whenever (does (actor anOpponent) commitCrime) proliferate ],
      power := stat 0, toughness := stat 0 } }

/-- Galloping Lizrog -/
def gallopingLizrog : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Galloping Lizrog", cost := some [generic 3, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Frog", creatureType "Lizard"],
      text :=
        [ keyword (.named "trample"),
          when (does (actor thisCreature) enter)
            (may
              (removeCounters anyNumber (some (printedKind p1p1Counter)) (among (allOf (and [isCreature, youControl]))) .many true)
              (some (putCounters (times (.lit 2) removedThisWay) (printedKind p1p1Counter) thisCreature .many))) ],
      power := stat 3, toughness := stat 3 } }

/-- Novijen Sages -/
def novijenSagesDraw : Ability :=
  activated
    (compound
      [ mana [generic 1],
        perform (removeCounters (exactly 2) (some (printedKind p1p1Counter)) (among (allOf (and [isCreature, youControl]))) .many true) ])
    (draw (.lit 1))
theorem okNovijenSagesDraw : Ability.check (DRS.ofList []) novijenSagesDraw = [] := by decide
/-- Cyclone -/
def cycloneUpkeepPayment : Ability :=
  at_ (beginningOfYour .upkeep)
    (sequentially
      [ putCounters (.lit 1) (printedKind (.named "windCounter")) thisEnchantment .many,
        doUnless (sacrifice thisEnchantment)
          (scaled (mana [pip .green])
            (times (.lit 1) (countersOn (.named "windCounter") thisEnchantment))) ])
theorem okCycloneUpkeepPayment : Ability.check (DRS.ofList []) cycloneUpkeepPayment = [] := by decide

def stormwildCapridor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stormwild Capridor", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Bird", creatureType "Goat"],
      text :=
        [ keyword (.named "flying"),
          static (damageRule .noncombatOnly unattributed (toRecipient thisCreature)
            (prevent allDamage
              (some (putCounters preventedThisWay (printedKind p1p1Counter) thisCreature .many)))
            .repeatedly) ],
      power := stat 1, toughness := stat 3 } }

def testOfFaith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Test of Faith", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ spell (establish
            (damageRule .any unattributed (toRecipient (target isCreature))
              (prevent (theNext (.lit 3))
                (some (putCounters preventedThisWay (printedKind p1p1Counter) (that (.type .creature)) .many)))
              .repeatedly)
            (some thisTurn)) ] } }

def temper : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Temper", cost := some [.variable, generic 1, pip .white], types := [.instant],
      text :=
        [ spell (establish
            (damageRule .any unattributed (toRecipient (target isCreature))
              (prevent (theNext (letter .x))
                (some (putCounters preventedThisWay (printedKind p1p1Counter) (that (.type .creature)) .many)))
              .repeatedly)
            (some thisTurn)) ] } }

def phytohydra : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phytohydra", cost := some [generic 2, pip .green, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Plant", creatureType "Hydra"],
      text :=
        [ static (instead (would any (dealDamage anyAmount thisCreature)) (putCounters thatMuch (printedKind p1p1Counter) it .many)) ],
      power := stat 1, toughness := stat 1 } }

def agelessEntity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ageless Entity", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ whenever (does (actor you) (changeLife (up anyAmount))) (putCounters thatMuch (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 4, toughness := stat 4 } }

/-- Chromatic Armor -/
def chromaticArmor : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chromatic Armor", cost := some [generic 1, pip .white, pip .blue],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          static (entersChoosing thisAura .color),
          static (entersWithCounters thisAura (.lit 1) (.named "sleightCounter")),
          static (damageRule .any (dealtBy (allOf (and [isSource, ofTheLastChosen .color])))
            (toRecipient (attachHost .enchanted (.type .creature))) (prevent allDamage) .repeatedly),
          activated (mana [.variable])
            (sequentially
              [ putCounters (.lit 1) (printedKind (.named "sleightCounter")) thisAura .many,
                choose (a (quality .color)),
                define .x (countersOn (.named "sleightCounter") thisAura) ]) ] } }

def vault75MiddleSchool : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vault 75: Middle School", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment], subtypes := [enchantmentType "Saga"],
      text :=
        [ when (chapterMark [1])
            (exile (allOf (and [isCreature, hasPower .atLeast (.lit 4)]))),
          when (chapterMark [2, 3])
            (putCounters (.lit 1) (printedKind p1p1Counter) (each (and [isCreature, youControl])) .many) ] } }

def keldonWarcaller : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Keldon Warcaller", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior"],
      text :=
        [ whenever (does (actor thisCreature) attack)
            (putCounters (.lit 1) (printedKind (.named "loreCounter"))
              (target (and [hasSubtype (enchantmentType "Saga"), hasPossessor .controller you])) .many) ],
      power := stat 2, toughness := stat 2 } }

def magistratesScepter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magistrate's Scepter", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (compound [mana [generic 4], tapSymbol])
            (putCounters (.lit 1) (printedKind (.named "chargeCounter")) thisArtifact .many),
          activated
            (compound
              [ tapSymbol,
                perform (removeCounters (exactly 3) (some (printedKind (.named "chargeCounter"))) thisArtifact .many true) ])
            (extraTurn (.lit 1)) ] } }

def grumgully : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grumgully, the Generous", cost := some [generic 1, pip .red, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Shaman"],
      text :=
        [ static (entersWithAdditionalCounters
            (each (and [ isCreature, hasPossessor .controller you,
                          not (hasSubtype (creatureType "Human")), otherThan thisCreature ]))
            (.lit 1) p1p1Counter) ],
      power := stat 3, toughness := stat 3 } }

def metallicMimic : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Metallic Mimic", cost := some [generic 2], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ static (entersChoosing thisCreature (.subtype .creature)),
          static (qualityChange thisCreature .adds (chosenQuality (ofChosen (.subtype
              .creature)))),
          static (entersWithAdditionalCounters
            (each (and [ isCreature, hasPossessor .controller you, ofChosen (.subtype .creature),
                          otherThan thisCreature ]))
            (.lit 1) p1p1Counter) ],
      power := stat 2, toughness := stat 1 } }

def curatorBeastieLine : StaticSpec :=
  entersWithAdditionalCounters (allOf (and [isCreature, hasPossessor .controller you, isColorless]))
    (.lit 2) p1p1Counter
theorem okCuratorBeastieLine : StaticSpec.check (DRS.ofList []) curatorBeastieLine = [] := by decide
def renataLine : StaticSpec :=
  entersWithAdditionalCounters
    (each (and [isCreature, hasPossessor .controller you, otherThan thisCreature])) (.lit 1)
    p1p1Counter
theorem okRenataLine : StaticSpec.check (DRS.ofList []) renataLine = [] := by decide
def entDraughtBasinAbility : Ability :=
  activatedOnlyDuring (compound [mana [.variable], tapSymbol])
    (putCounters (.lit 1) (printedKind p1p1Counter)
      (target (and [isCreature, hasPower .eq (letter .x)])) .many)
    asSorcery
theorem okEntDraughtBasinAbility : Ability.check (DRS.ofList []) entDraughtBasinAbility = [] := by decide

def matopiGolem : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Matopi Golem", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [creatureType "Golem"],
      text :=
        [ activated (mana [generic 1])
            (triggerThisWay (regenerate thisCreature) (does any (regenerationApplication thisCreature))
              (putCounters (.lit 1) (printedKind m1m1Counter) it .many)) ],
      power := stat 3, toughness := stat 3 } }

def bramblewoodParagon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bramblewood Paragon", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Warrior"],
      text :=
        [ static (entersWithAdditionalCounters
            (each (and [ isCreature, hasSubtype (creatureType "Warrior"), hasPossessor .controller you,
                          otherThan thisCreature ]))
            (.lit 1) p1p1Counter),
          static (abilityGrant
            (each (and [isCreature, hasPossessor .controller you, hasCounter (some p1p1Counter)]))
            (keyword (.named "trample"))) ],
      power := stat 2, toughness := stat 2 } }

def oonasBlackguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Oona's Blackguard", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text :=
        [ keyword (.named "flying"),
          static (entersWithAdditionalCounters
            (each (and [ isCreature, hasSubtype (creatureType "Rogue"), hasPossessor .controller you,
                          otherThan thisCreature ]))
            (.lit 1) p1p1Counter),
          whenever
            (does (actor (a (and [isCreature, hasPossessor .controller you, hasCounter (some p1p1Counter)]))) (dealDamage (to := a isPlayer) (kind := .combatOnly)))
            (discard (a (and [isCard, in_ (handOf (that .player))]))) ],
      power := stat 1, toughness := stat 1 } }

def crumblingAshes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crumbling Ashes", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ at_ (beginningOfYour .upkeep)
            (destroy (target (and [isCreature, hasCounter (some m1m1Counter)]))) ] } }

def hunterOfEyeblights : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hunter of Eyeblights", cost := some [generic 3, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Elf", creatureType "Assassin"],
      text :=
        [ when (does (actor thisCreature) enter)
            (putCounters (.lit 1) (printedKind p1p1Counter) (target (and [isCreature, youDontControl])) .many),
          activated (compound [mana [generic 2, pip .black], tapSymbol])
            (destroy (target (and [isCreature, hasCounter]))) ],
      power := stat 3, toughness := stat 3 } }

def pridemalkin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pridemalkin", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [creatureType "Cat"],
      text :=
        [ when (does (actor thisCreature) enter)
            (putCounters (.lit 1) (printedKind p1p1Counter) (target (and [isCreature, youControl])) .many),
          static (abilityGrant
            (each (and [isCreature, hasPossessor .controller you, hasCounter (some p1p1Counter)]))
            (keyword (.named "trample"))) ],
      power := stat 2, toughness := stat 1 } }

def aboroth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aboroth", cost := some [generic 4, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ cumulativeUpkeep (perform (putCounters (.lit 1) (printedKind m1m1Counter) thisCreature .many)) ],
      power := stat 9, toughness := stat 9 } }

def shelteringAncient : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sheltering Ancient", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Treefolk"],
      text :=
        [ keyword (.named "trample"),
          cumulativeUpkeep
            (perform (putCounters (.lit 1) (printedKind p1p1Counter)
              (a (and [isCreature, hasPossessor .controller anOpponent])) .many)) ],
      power := stat 5, toughness := stat 5 } }

def coverOfWinterPut : Ability :=
  activated (mana [.snow]) (putCounters (.lit 1) (printedKind (.named "ageCounter")) thisEnchantment .many)
theorem okCoverOfWinterPut : Ability.check (DRS.ofList []) coverOfWinterPut = [] := by decide

def urborgScavengers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urborg Scavengers", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ triggeredOr (does (actor thisCreature) enter) [does (actor thisCreature) attack]
            (sequentially
              [ exile (target (and [isCard, in_ graveyard])),
                putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many ]),
          alsoForKeywords
            (static (onlyWhile (abilityGrant thisCreature (keyword (.named "flying")))
              (exists_ (and [in_ (exiledWith thisCreature), hasKeyword (.the (.named "flying"))]))))
            (["firstStrike", "doubleStrike", "deathtouch", "haste", "hexproof", "indestructible",
              "lifelink", "menace", "reach", "trample", "vigilance"].map (fun k => .the (.named k))) ],
      power := stat 2, toughness := stat 2 } }

def sengirVampire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sengir Vampire", cost := some [generic 3, pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Vampire"],
      text :=
        [ keyword (.named "flying"),
          whenever (does (actor (a (and [isCreature, happenedTo (actor thisCreature) (dealDamage anyAmount             (asMarker .permanent (relative .object))) soFarThisTurn]))) die)
            (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 4, toughness := stat 4 } }

def mildManneredLibrarian : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mild-Mannered Librarian", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ activatedOnlyOnce (mana [generic 3, pip .green])
            (sequentially
              [ establish
                  (qualityChange thisCreature .sets
                    (bundle { characteristics := { subtypes := [creatureType "Werewolf"] } })),
                putCounters (.lit 2) (printedKind p1p1Counter) it .many,
                draw (.lit 1) ])
            oncePerGame ],
      power := stat 1, toughness := stat 1 } }

def infernalVessel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Infernal Vessel", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ triggeredIf (does (actor thisCreature) die) (not (matches_ it (hasSubtype (creatureType "Demon"))))
            (sequentially
              [ returnToBattlefieldWithCounters it (ownerOf it) (.lit 2) p1p1Counter,
                become it { characteristics := { subtypes := [creatureType "Demon"] } } none ]) ],
      power := stat 2, toughness := stat 1 } }

/-- Vivien's Talent -/
def viviensTalentTrigger : Ability :=
  whenever (does (actor (a (and [isNontoken, and [isCreature, youControl]]))) enter)
    (putCounters (.lit 1) (printedKind (.named "loyaltyCounter")) (attachHost .enchanted (.type .planeswalker)) .many)
theorem okViviensTalentTrigger : Ability.check (DRS.ofList []) viviensTalentTrigger = [] := by decide
/-- Bioessence Hydra -/
def bioessenceHydraTrigger : Ability :=
  whenever
    (counterEvent .put (.named "loyaltyCounter") .many
      (allOf (and [hasType .planeswalker, hasPossessor .controller you])))
    (putCounters thatMuch (printedKind p1p1Counter) thisCreature .many)
theorem okBioessenceHydraTrigger : Ability.check (DRS.ofList []) bioessenceHydraTrigger = [] := by decide
/-- Vraska, Betrayal's Sting -/
def vraskaBetrayalsStingUltimate : Action :=
  doIf (compareAmt (countersOn (.named "poison") (target isPlayer)) .less (.lit 9))
    (putCounters theDifference (printedKind (.named "poison")) they .many)
theorem okVraskaBetrayalsStingUltimate :
    Action.check (DRS.ofList []) vraskaBetrayalsStingUltimate = [] := by decide
/-- Vexing Puzzlebox -/
def vexingPuzzleboxCounters : Ability :=
  whenever (does (actor you) (rollDice anyAmount .anyDie .many))
    (putCounters (theOutcome .rollResult) (printedKind (.named "chargeCounter")) thisArtifact .many)
theorem okVexingPuzzleboxCounters : Ability.check (DRS.ofList []) vexingPuzzleboxCounters = [] := by decide
def spaceFamilyGoblinsonRoll : Ability :=
  whenever (does (actor you) (rollDice anyAmount .anyDie))
    (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many)
theorem okSpaceFamilyGoblinsonRoll : Ability.check (DRS.ofList []) spaceFamilyGoblinsonRoll = [] := by decide
/-- Atomwheel Acrobats -/
def atomwheelAcrobatsRoll : Ability :=
  whenever (does (actor you) (rollDice (watch := resultIn (range (some 1) (some 2)))))
    (putCounters thatMuch (printedKind p1p1Counter) thisCreature .many)
theorem okAtomwheelAcrobatsRoll : Ability.check (DRS.ofList []) atomwheelAcrobatsRoll = [] := by decide
/-- Resolute Veggiesaur -/
def resoluteVeggiesaurThirdDie : Ability :=
  whenever (nthOccurrence (.nth 3) (does (actor you) (rollDice anyAmount .anyDie)) (some .turn))
    (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many)
theorem okResoluteVeggiesaurThirdDie : Ability.check (DRS.ofList []) resoluteVeggiesaurThirdDie = [] := by decide
/-- Run the Play (Striding Shotcaller's other half) -/
def runThePlayCounters : Action :=
  putCounters (.lit 1) (printedKind p1p1Counter)
    (eachOf (targets (upToOf (letter .x)) isCreature)) .many
theorem okRunThePlayCounters : Action.check (DRS.ofList [letterB .x]) runThePlayCounters = [] := by decide
/-- Font of Agonies -/
def fontOfAgoniesTrigger : Ability :=
  whenever (does (actor you) (pay (perform (changeLife (down anyAmount))) .once)) (putCounters thatMuch (printedKind (.named "bloodCounter")) thisEnchantment .many)
theorem okFontOfAgoniesTrigger : Ability.check (DRS.ofList []) fontOfAgoniesTrigger = [] := by decide
/-- Heart of Kiran -/
def heartOfKiranCrewAltCost : Ability :=
  static (altCost (allOf (and [abilityHead (.keyword (.named "crew")), abilityOf «this»]))
    (some (perform (removeCounters (exactly 1) (some (printedKind (.named "loyaltyCounter"))) (a (and [hasType .planeswalker, hasPossessor .controller you])) .many true))))
theorem okHeartOfKiranCrewAltCost : Ability.check (DRS.ofList []) heartOfKiranCrewAltCost = [] := by decide
/-- Tainted Adversary -/
def taintedAdversaryOffer : Ability :=
  when (does (actor thisCreature) enter)
    (triggerReflexively (may (pay (mana [generic 2, pip .black]) .anyNumberOfTimes))
      (putCounters thatMuch (printedKind p1p1Counter) thisCreature .many))
theorem okTaintedAdversaryOffer : Ability.check (DRS.ofList []) taintedAdversaryOffer = [] := by decide
/-- Korvold, Gleeful Glutton -/
def korvoldCombatTrigger : Ability :=
  whenever (does (actor thisCreature) (dealDamage (to := a isPlayer) (kind := .combatOnly)))
    (sequentially
      [ putCounters (letter .x) (printedKind p1p1Counter) thisCreature .many,
        draw (letter .x),
        define .x (distinctCount .permanentType (allOf (and [isCard, in_ (graveyardOf you)]))) ])
theorem okKorvoldCombatTrigger : Ability.check (DRS.ofList []) korvoldCombatTrigger = [] := by decide
/-- Mirelurk Queen -/
def mirelurkQueenTrigger : Ability :=
  triggeredOnlyOnce
    (does any (Semantics.Action.ownScope (some (.action "mill"))
      (Semantics.Action.enact (.action "mill")
        (move (Semantics.NounPhrase.inCaller ((counted (atLeast 1) (and [not isLand,
        in_ (zone .library none)]))))
          (zone .library none)
          (some graveyard) .arrival) none none)))
    oncePerTurn
    (sequentially [draw (.lit 1), putCounters (.lit 1) (printedKind p1p1Counter)
        thisCreature .many])
theorem okMirelurkQueenTrigger : Ability.check (DRS.ofList []) mirelurkQueenTrigger = [] := by decide

/-- Whirling Dervish -/
def whirlingDervish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Whirling Dervish", cost := some [pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Monk"],
      text :=
        [ keywordQuality (.named "protection") (hasColor .black),
          triggeredIf (beginningOfPossessed .endStep (each isPlayer))
            (happened (dealDamage anyAmount anOpponent) (actor thisCreature) soFarThisTurn)
            (putCounters (.lit 1) (printedKind p1p1Counter) it .many) ],
      power := stat 1, toughness := stat 1 } }

/-- Ichor Shade -/
def ichorShade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ichor Shade", cost := some [generic 2, pip .black], types := [.creature],
      subtypes := [creatureType "Phyrexian", creatureType "Shade"],
      text :=
        [ triggeredIf (beginningOfYour .endStep)
            (did (⟨any, (move (a (or [isArtifact, isCreature])) (battlefield) (some graveyard) .departure), (within soFarThisTurn)⟩))
            (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 2, toughness := stat 3 } }

/-- Asmira, Holy Avenger -/
def asmiraHolyAvenger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Asmira, Holy Avenger", cost := some [generic 2, pip .green, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ keyword (.named "flying"),
          at_ (beginningOfPossessed .endStep (each isPlayer))
            (putCounters
              (eventTally (⟨any, (move (a isCreature) (battlefield) (some (graveyardOf you)) .departure), (within soFarThisTurn)⟩) .events)
              (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 2, toughness := stat 3 } }

/-- Thought Sponge -/
def thoughtSponge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thought Sponge", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Sponge"],
      text :=
        [ keyword (.named "flash"),
          static (entersWithCounters thisCreature greatestCardsAnOpponentDrew p1p1Counter),
          when (does (actor thisCreature) die) (draw (statOf (.stat .power) thisCreature)) ],
      power := stat 1, toughness := stat 1 } }

/-- Skeleton Ship -/
def skeletonShip : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Skeleton Ship", cost := some [generic 3, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Skeleton"],
      text :=
        [ when
            (stateHolds
              (not (exists_ (and [isLand, hasSubtype (landType "Island"), hasPossessor .controller you]))))
            (sacrifice thisCreature),
          activated tapSymbol (putCounters (.lit 1) (printedKind m1m1Counter) (target isCreature) .many) ],
      power := stat 0, toughness := stat 3 } }

/-- Contractual Safeguard -/
def contractualSafeguardPass : Action :=
  sequentially
    [ choose (a (counterKindOn (a (and [isCreature, youControl])))),
      putCounters (.lit 1) thatKind (each ((and [isCreature, youControl, otherThan it]))) .many ]
theorem okContractualSafeguardPass : Action.check (DRS.ofList []) contractualSafeguardPass = [] := by decide

/-- Feral Contest -/
def feralContest : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Feral Contest", cost := some [generic 3, pip .green], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ putCounters (.lit 1) (printedKind p1p1Counter) (target (and [isCreature, youControl])) .many,
              establish (must (actor (target (and [isCreature, isOther]))) (block it)) (some thisTurn) ]) ] } }

private semantic_template companyRecipient : NounPhrase :=
  .firstMention "thranduilsCompany.recipient" (target (and [isCreature, youControl]))
private semantic_template companyRecipientAgain : NounPhrase :=
  .laterMention "thranduilsCompany.recipient" (target (and [isCreature, youControl]))

/-- Thranduil's Company -/
def thranduilsCompany : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thranduil's Company", cost := some [generic 2, pip .green, pip .blue],
      types := [.creature], subtypes := [creatureType "Elf", creatureType "Soldier"],
      text :=
        [ static (onlyWhile (can (actor you) (play (counted (additional (exactly 1)) isLand)))
            (exists_ (and [ and [isCreature, otherThan thisCreature], hasSubtype (creatureType "Elf"),
                             hasPossessor .controller you ]))),
          abilityWord "landfall"
            (whenever (does (actor (a (and [isLand, hasPossessor .controller you]))) enter)
              (sequentially
                [ putCounters (.lit 2) (printedKind p1p1Counter)
                    companyRecipient .many,
                  gain
                    companyRecipientAgain
                    (keyword (.named "vigilance")) (some untilEndOfTurn) ])) ],
      power := stat 3, toughness := stat 4 } }

private semantic_template stunningShotRecipient : NounPhrase :=
  .firstMention "stunningShot.tappedCreature"
    (.described (.target (upTo 1)) (.and [isCreature, .hasPossessor .controller anOpponent]))
private semantic_template stunningShotRecipientAgain : NounPhrase :=
  .laterMention "stunningShot.tappedCreature"
    (.described (.target (upTo 1)) (.and [isCreature, .hasPossessor .controller anOpponent]))

/-- Stunning Shot -/
def stunningShot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stunning Shot", cost := some [generic 1, pip .white], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ putCounters (.lit 2) (printedKind p1p1Counter)
                (targets (upTo 1) (and [isCreature, youControl])) .many,
              tap stunningShotRecipient,
              putCounters (.lit 1) (printedKind (.named "stunCounter"))
                stunningShotRecipientAgain .many ]) ] } }

/-- Stonebinder's Familiar -/
def stonebindersFamiliar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stonebinder's Familiar", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Spirit", creatureType "Dog"],
      text :=
        [ triggeredOnlyOnceDuring (does any (putInto (counted (atLeast 1) isCard) exileZone))
            (duringPart .turn (some you)) oncePerTurn
            (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 1, toughness := stat 1 } }

/-- Nazgûl -/
def nazgulRingTrigger : Ability :=
  whenever (does (actor you) (theRingTemptsYou))
    (putCounters (.lit 1) (printedKind p1p1Counter)
      (each (and [hasSubtype (creatureType "Wraith"), hasPossessor .controller you])) .many)
theorem okNazgulRingTrigger : Ability.check (DRS.ofList []) nazgulRingTrigger = [] := by decide

/-- Captain Marvel, Apex Avenger -/
def captainMarvelApexAvenger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Captain Marvel, Apex Avenger", cost := some [generic 5, pip .red, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Kree", creatureType "Hero"],
      text :=
        [ keyword (.named "flying"), keyword (.named "doubleStrike"), keyword (.named "indestructible"),
          triggeredIf
            (does (by_ you) (putCounters anyAmount anyKind (a (and [isCreature, otherThan «this»])) .many false))
            (matches_ it (not (hasSubtype (creatureType "Kree"))))
            (may (putCounters thatMuch thoseCounters «this» .many)) ],
      power := stat 4, toughness := stat 4 } }

/-- Blood Spatter Analysis -/
def bloodSpatterAnalysis : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Spatter Analysis", cost := some [pip .black, pip .red], types := [.enchantment],
      text :=
        [ when (does (actor thisEnchantment) enter)
            (instruct thisEnchantment (dealDamage (.lit 3) (target (and [isCreature, hasPossessor .controller anOpponent])) .noncombatOnly)),
          whenever (does (actor (counted (atLeast 1) isCreature)) die)
            (sequentially
              [ mill (.lit 1) you,
                putCounters (.lit 1) (printedKind (.named "bloodstainCounter")) thisEnchantment .many,
                triggerReflexively
                  (doOnlyIf (sacrifice thisEnchantment)
                    (compareAmt (countersOn (.named "bloodstainCounter") thisEnchantment) .atLeast (.lit 5)))
                  (putInto (target (and [isCreature, isCard, in_ (graveyardOf you)])) hand) ]) ] } }

/-- Bewitching Leechcraft -/
def bewitchingLeechcraft : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bewitching Leechcraft", cost := some [generic 1, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          when (does (actor thisAura) enter) (tap (attachHost .enchanted (.type .creature))),
          static (abilityGrant (attachHost .enchanted (.type .creature))
            (static (instead (would any (setStatus .untapped thisCreature)) (doIfDone
                (removeCounters (exactly 1) (some (printedKind p1p1Counter)) thisCreature .many true)
                (some (untap thisCreature))) (some (duringPart .untapStep (some you)))))) ] } }

def agentOfTheShadowThievesGrantedTrigger : Ability :=
  triggeredIf (does (actor thisCreature) (attack (some (a isPlayer)))) (not (exists_ (and [ (isOpponentOf you),
                                 compare .greater (lifeTotalOf (that .player)) (axes := [.playerStat .lifeTotal]) ]))) (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many)
theorem okAgentOfTheShadowThievesGrantedTrigger :
    Ability.check (DRS.ofList []) agentOfTheShadowThievesGrantedTrigger = [] := by decide

/-- Bloodline Pretender -/
def bloodlinePretender : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloodline Pretender", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ keyword (.named "changeling"),
          static (entersChoosing thisCreature (.subtype .creature)),
          whenever
            (does (actor (a (and [ isCreature, hasPossessor .controller you, otherThan thisCreature,
                                ofChosen (.subtype .creature) ]))) enter)
            (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 2, toughness := stat 2 } }

/-- Chamber Sentry -/
def chamberSentryDamage : Ability :=
  activated
    (compound
      [ mana [.variable], tapSymbol,
        perform (removeCounters (exactlyOf (letter .x)) (some (printedKind p1p1Counter)) thisCreature .many true) ])
    (instruct «this» (dealDamage (letter .x) (target isAnyTarget) .noncombatOnly))
theorem okChamberSentryDamage : Ability.check (DRS.ofList []) chamberSentryDamage = [] := by decide

/-- Menacing Ogre -/
def menacingOgre : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Menacing Ogre", cost := some [generic 3, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Ogre"],
      text :=
        [ keyword (.named "trample"), keyword (.named "haste"),
          when (does (actor thisCreature) enter)
            (sequentially
              [ instruct (each isPlayer) (choose (disclosure := .secretly) (a (quality .number))),
                revealChoices .numbers,
                instruct (each (and [isPlayer, choseExtreme .max])) (loseLife thatMuch),
                doIf (matches_ you (choseExtreme .max))
                  (putCounters (.lit 2) (printedKind p1p1Counter) thisCreature .many) ]) ],
      power := stat 3, toughness := stat 3 } }

/-- Charnel Troll -/
def charnelTroll : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Charnel Troll", cost := some [generic 1, pip .black, pip .green], types := [.creature],
      subtypes := [creatureType "Troll"],
      text :=
        [ keyword (.named "trample"),
          at_ (beginningOfYour .upkeep)
            (doIfDone (exile (a (and [isCreature, in_ (graveyardOf you)])))
              (some (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many))
              (some (sacrifice thisCreature))),
          activated
            (compound
              [ mana [pip .black, pip .green],
                perform (discard (a (and [isCreature, in_ yourHand]))) ])
            (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many) ],
      power := stat 4, toughness := stat 4 } }

/-- Mistbreath Elder -/
def mistbreathElder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistbreath Elder", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Frog", creatureType "Warrior"],
      text :=
        [ at_ (beginningOfYour .upkeep)
            (doIfDone (returnTo (a ((and [isCreature, youControl, otherThan thisCreature]))) hand [])
              (some (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many))
              (some (may (returnTo thisCreature hand [])))) ],
      power := stat 2, toughness := stat 2 } }

/-- Dark Depths -/
def darkDepths : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dark Depths", supertypes := [.legendary, .snow], types := [.land],
      text :=
        [ static (entersWithCounters thisLand (.lit 10) (.named "iceCounter")),
          activated (mana [generic 3])
            (removeCounters (exactly 1) (some (printedKind (.named "iceCounter"))) thisLand),
          when (stateHolds (not (matches_ thisLand (hasCounter (some (.named "iceCounter"))))))
            (doIfDone (sacrifice thisLand) (some (createToken maritLage)))
                ] } }

/-- Thallid -/
def thallid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thallid", cost := some [pip .green], types := [.creature],
      subtypes := [creatureType "Fungus"],
      text :=
        [ at_ (beginningOfYour .upkeep)
            (putCounters (.lit 1) (printedKind (.named "sporeCounter")) thisCreature .many),
          activated
            (perform (removeCounters (exactly 3) (some (printedKind (.named "sporeCounter"))) thisCreature .many true))
            (createToken (creatureToken 1 1 [.green] [creatureType "Saproling"])) ],
      power := stat 1, toughness := stat 1 } }

/-- Aether Chaser -/
def aetherChaserEnergy : Ability :=
  when (does (actor thisCreature) enter) (putCounters (.lit 2) (printedKind (.named "energy")) you .many)
theorem okAetherChaserEnergy : Ability.check (DRS.ofList []) aetherChaserEnergy = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossEntersOiled : StaticSpec :=
  entersWithCounters thisCreature (.lit 4) (.named "oilCounter")
theorem okArchfiendOfTheDrossEntersOiled :
    StaticSpec.check (DRS.ofList []) archfiendOfTheDrossEntersOiled = [] := by decide
/-- Archfiend of the Dross -/
def archfiendOfTheDrossOilUpkeep : Ability :=
  at_ (beginningOfYour .upkeep)
    (removeCounters (exactly 1) (some (printedKind (.named "oilCounter"))) thisCreature .many true)
theorem okArchfiendOfTheDrossOilUpkeep : Ability.check (DRS.ofList []) archfiendOfTheDrossOilUpkeep = [] := by
  decide
def neurokTransmuter : Action :=
  become (target isCreature) { characteristics := { types := [.artifact] } } (some untilEndOfTurn)
theorem okNeurokTransmuter : Action.check (DRS.ofList []) neurokTransmuter = [] := by decide
def syphonMind : Action :=
  sequentially
    [ sequentially [instruct (each isOtherPlayer) (choose (a (and [isCard, in_ hand]))), discard them],
      doForEach (theVerbed (.action "discard") .many (some .card) .thisWay) (draw (.lit 1)) ]
theorem okSyphonMind : Action.check (DRS.ofList []) syphonMind = [] := by decide
def peek : Action := sequentially [lookAtHandOf (target isPlayer), draw (.lit 1)]
theorem okPeek : Action.check (DRS.ofList []) peek = [] := by decide
/-- Bumi, King of Three Trials -/
def bumiScryMode : Action := instruct (target isPlayer) (scry (.lit 3))
theorem okBumiScryMode : Action.check (DRS.ofList []) bumiScryMode = [] := by decide
/-- Final Act -/
def finalActCounterMode : Action := loseAllCounters none (agent := each (isOpponentOf you))
theorem okFinalActCounterMode : Action.check (DRS.ofList []) finalActCounterMode = [] := by decide
def leeches : Action :=
  loseAllCounters (some (printedKind (.named "poison"))) (agent := target isPlayer)
theorem okLeeches : Action.check (DRS.ofList []) leeches = [] := by decide

/-- Woeleecher -/
def woeleecherWhole : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Woeleecher", cost := some [generic 5, pip .white], types := [.creature],
      subtypes := [creatureType "Elemental"], text := [woeleecher],
      power := stat 3, toughness := stat 5 } }

end Semantics.Cards
