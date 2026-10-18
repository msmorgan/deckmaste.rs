import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Description
import Semantics.Cards.Keyword
import Semantics.Cards.Static

/-!
# Semantics.Cards.Cost

Port of `idris/src/Experimental/Cards/Cost.idr`: the printed cards of the Cost family
(activation costs, cost changes, alternative costs, the cost's X) and the bench items beside
them; the file's `Refl` pins on `costLetters` are theorems.

Not ported: `missyChaosBranch` ("chaos ensues" is Planechase, out of scope).
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def masterDecoy : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .white], Primitives.Cost.tapSymbol]) (tap (target isCreature))
theorem okMasterDecoy : Ability.check (DRS.ofList []) masterDecoy = [] := by decide
def cycling : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.perform (discard Primitives.NounPhrase.this)]) (Primitives.Action.draw (.lit
      1))
theorem okCycling : Ability.check (DRS.ofList []) cycling = [] := by decide
def merrowGrimeblotter : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1, hybridPip .blue .black], Primitives.Cost.untapSymbol])
    (get (target isCreature) (Primitives.Delta.down (.lit 2)) (Primitives.Delta.down (.lit 0)) (some untilEndOfTurn))
theorem okMerrowGrimeblotter : Ability.check (DRS.ofList []) merrowGrimeblotter = [] := by decide
def phyrexianSnowcrusher : Ability :=
  activated (Primitives.Cost.mana [generic 1, .snow])
    (get thisCreature (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn))
theorem okPhyrexianSnowcrusher : Ability.check (DRS.ofList []) phyrexianSnowcrusher = [] := by decide
def havocSower : Ability :=
  activated (Primitives.Cost.mana [generic 1, colorlessPip])
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okHavocSower : Ability.check (DRS.ofList []) havocSower = [] := by decide
/-- Erebos, God of the Dead -/
def erebos : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1, pip .black], payLife 2]) (Primitives.Action.draw (.lit 1))
theorem okErebos : Ability.check (DRS.ofList []) erebos = [] := by decide
def baskingRootwalla : Ability :=
  activatedOnlyOnce (Primitives.Cost.mana [generic 1, pip .green])
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn)) oncePerTurn
theorem okBaskingRootwalla : Ability.check (DRS.ofList []) baskingRootwalla = [] := by decide
def securityDetail : Ability :=
  activatedOnlyOnceIf (Primitives.Cost.mana [pip .white, pip .white])
    (createToken (creatureToken 1 1 [.white] [creatureType "Soldier"])) oncePerTurn
    (Primitives.Condition.not (exists_ (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
theorem okSecurityDetail : Ability.check (DRS.ofList []) securityDetail = [] := by decide
def aphettoAlchemist : Ability :=
  activated Primitives.Cost.tapSymbol (Primitives.Action.setStatus .untapped (target (Primitives.Predicate.or [isArtifact, isCreature])))
theorem okAphettoAlchemist : Ability.check (DRS.ofList []) aphettoAlchemist = [] := by decide

def charRumbler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Char-Rumbler", cost := some [generic 2, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ keyword (.named "doubleStrike"),
          activated (Primitives.Cost.mana [pip .red])
            (get thisCreature (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ],
      power := some (.lit (-1)), toughness := stat 3 } }

/-- Sisters of Stone Death -/
def sistersOfStoneDeathRecall : Ability :=
  activated (Primitives.Cost.mana [generic 2, pip .black])
    (putOntoBattlefieldUnderYourControl (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ (Primitives.Location.exiledWith thisCreature)])))
theorem okSistersOfStoneDeathRecall : Ability.check (DRS.ofList []) sistersOfStoneDeathRecall = [] := by decide
/-- Synod Sanctum -/
def synodSanctumReturn : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.perform (sacrifice thisArtifact)])
    (putOntoBattlefieldUnderYourControl (allOf exiledWithThisArtifact))
theorem okSynodSanctumReturn : Ability.check (DRS.ofList []) synodSanctumReturn = [] := by decide

def coldStorage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cold Storage", cost := some [generic 4], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.mana [generic 3]) (exile (target (and [isCreature, youControl]))),
          activated (Primitives.Cost.perform (sacrifice thisArtifact))
            (putOntoBattlefieldUnderYourControl (each (Primitives.Predicate.and [isCreature, exiledWithThisArtifact]))) ] } }

def barlsCage : Ability :=
  activated (Primitives.Cost.mana [generic 3]) (Primitives.Action.skipUntap (target isCreature) (.lit 1))
theorem okBarlsCage : Ability.check (DRS.ofList []) barlsCage = [] := by decide
def vodalianIllusionist : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .blue, pip .blue], Primitives.Cost.tapSymbol])
    (Primitives.Action.setStatus .phasedOut (target isCreature))
theorem okVodalianIllusionist : Ability.check (DRS.ofList []) vodalianIllusionist = [] := by decide
def witchsMist : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .black], Primitives.Cost.tapSymbol])
    (destroy (target (Primitives.Predicate.and [isCreature, happenedTo Primitives.Subject.any (Primitives.Action.dealDamage Primitives.Amount.anyAmount (Primitives.NounPhrase.asMarker .permanent (relative
      .object)) .any) soFarThisTurn])))
theorem okWitchsMist : Ability.check (DRS.ofList []) witchsMist = [] := by decide
def goadTargetCreature : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.tapSymbol])
    (Primitives.Action.gainDesignation (target isCreature) (.named "goaded") none)
theorem okGoadTargetCreature : Ability.check (DRS.ofList []) goadTargetCreature = [] := by decide
/-- Krenko, Mob Boss -/
def krenko : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Action.sequentially
      [ create (Primitives.Amount.letter .x) (Primitives.TokenSpec.written (creatureToken 1 1 [.red] [creatureType "Goblin"])) [],
        Primitives.Action.define .x (countOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Goblin"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ])
theorem okKrenko : Ability.check (DRS.ofList []) krenko = [] := by decide
/-- Dokai, Weaver of Life -/
def dokai : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 4, pip .green, pip .green], Primitives.Cost.tapSymbol])
    (Primitives.Action.sequentially
      [ createToken (creatureTokenOf (Primitives.Amount.letter .x) (Primitives.Amount.letter .x) [.green] [creatureType "Elemental"]),
        Primitives.Action.define .x (countOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ])
theorem okDokai : Ability.check (DRS.ofList []) dokai = [] := by decide

def ghalta : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghalta, Primal Hunger", cost := some [generic 10, pip .green, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elder", creatureType "Dinosaur"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.costShift Primitives.NounPhrase.this (Primitives.CostShift.less (Primitives.Amount.letter .x) none),
              Primitives.StaticSpec.letterDefinition .x (aggregate .sum (.stat .power) (and [isCreature, youControl])) ]),
          keyword (.named "trample") ],
      power := stat 12, toughness := stat 12 } }

def ancientStoneIdol : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.costShift Primitives.NounPhrase.this (Primitives.CostShift.less (forEach (.lit 1) (Primitives.Predicate.and [isCreature, isAttacking])) none))
theorem okAncientStoneIdol : Ability.check (DRS.ofList []) ancientStoneIdol = [] := by decide

/-- "<p> spells cost {N} more/less to cast" -/
def spellsCost (p : Predicate) (shift : CostShift) : StaticSpec := Primitives.StaticSpec.costShift (allOf p) shift

def thornOfAmethyst : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thorn of Amethyst", cost := some [generic 2], types := [.artifact],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.not isCreature, isSpell]) (Primitives.CostShift.more (.lit 1)))] } }

def ferozsBan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Feroz's Ban", cost := some [generic 6], types := [.artifact],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isCreature, isSpell]) (Primitives.CostShift.more (.lit 2)))] } }

def urzasFilter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Filter", cost := some [generic 4], types := [.artifact],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isMulticolored, isSpell]) (Primitives.CostShift.less (.lit 2) none))] } }

def emeraldMedallion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Emerald Medallion", cost := some [generic 2], types := [.artifact],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.colorIs .green, isSpell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none))] } }

/-- Highspire Bell-Ringer -/
def highspireBellRinger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Highspire Bell-Ringer", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Djinn", creatureType "Monk"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (Primitives.StaticSpec.costShift (the (Primitives.Predicate.and [isSpell, nthCastBy (.nth 2) Primitives.NounPhrase.you .turn])) (Primitives.CostShift.less
              (.lit 1) none)) ],
      power := stat 1, toughness := stat 4 } }

def foundryInspector : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foundry Inspector", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isArtifact, isSpell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none))],
      power := stat 3, toughness := stat 2 } }

def daruWarchief : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daru Warchief", cost := some [generic 2, pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Soldier"), isSpell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none)),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Soldier"), isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 2))) ],
      power := stat 1, toughness := stat 1 } }

def grandArbiter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grand Arbiter Augustin IV", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Advisor"],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.colorIs .white, isSpell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none)),
          Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.colorIs .blue, isSpell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none)),
          Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isSpell, castBy (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))]) (Primitives.CostShift.more (.lit 1))) ],
      power := stat 2, toughness := stat 3 } }

def goblinElectromancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Goblin Electromancer", cost := some [pip .blue, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Wizard"],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isInstantOrSorcery, isSpell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none))],
      power := stat 2, toughness := stat 2 } }

def arcaneMelee : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Melee", cost := some [generic 4, pip .blue], types := [.enchantment],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isInstantOrSorcery, isSpell]) (Primitives.CostShift.less (.lit 2) none))] } }

def manaMatrix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Matrix", cost := some [generic 6], types := [.artifact],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.or [isInstant, isEnchantment], isSpell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 2) none)) ] } }

def auraOfSilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aura of Silence", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.or [isArtifact, isEnchantment], isSpell, castBy (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))])
            (Primitives.CostShift.more (.lit 2))),
          activated (Primitives.Cost.perform (sacrifice thisEnchantment))
            (destroy (target (Primitives.Predicate.or [isArtifact, isEnchantment]))) ] } }

def chillerpillar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chillerpillar", cost := some [generic 3, pip .blue], supertypes := [.snow],
      types := [.creature], subtypes := [creatureType "Insect"],
      text :=
        [ activated (Primitives.Cost.mana [generic 4, .snow, .snow]) (monstrosity (.lit 2)),
          Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword (.named "flying")))
            (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasDesignation (.named "monstrous") none))) ],
      power := stat 3, toughness := stat 3 } }

def nullhideFerox : Ability :=
  activated (Primitives.Cost.mana [generic 2])
    (Primitives.Action.establish (Primitives.StaticSpec.allAbilityLoss thisCreature none) (some untilEndOfTurn))
theorem okNullhideFerox : Ability.check (DRS.ofList []) nullhideFerox = [] := by decide

def causticTar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caustic Tar", cost := some [generic 4, pip .black, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isLand,
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .land))
            (activated Primitives.Cost.tapSymbol (instruct (target Primitives.Predicate.anyPlayer) (loseLife (.lit 3))))) ] } }

/-- Saheeli, Filigree Master -/
def saheelisEmblem : Action :=
  Primitives.Action.getEmblem
    [ Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [isArtifact, isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
        (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
      Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isArtifact, isSpell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none)) ]
theorem okSaheelisEmblem : Action.check (DRS.ofList []) saheelisEmblem = [] := by decide

def jaceBeleren : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Jace Beleren", cost := some [generic 1, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Jace"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 2)) (instruct (each Primitives.Predicate.anyPlayer) (Primitives.Action.draw (.lit 1))),
          activated (Primitives.Cost.loyaltySymbol (.down 1)) (instruct (target Primitives.Predicate.anyPlayer) (Primitives.Action.draw (.lit 1))),
          activated (Primitives.Cost.loyaltySymbol (.down 10)) (instruct (target Primitives.Predicate.anyPlayer) (mill (.lit 20) they))
              ],
      loyalty := stat 3 } }

def elspethSunsChampion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elspeth, Sun's Champion", cost := some [generic 4, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Elspeth"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 1))
            (createToken (creatureToken 1 1 [.white] [creatureType "Soldier"]) (count := (.lit 3))),
          activated (Primitives.Cost.loyaltySymbol (.down 3))
            (destroy (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4)]))),
          activated (Primitives.Cost.loyaltySymbol (.down 7))
            (Primitives.Action.getEmblem
              [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
                  [ getsPt (allOf (and [isCreature, youControl])) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)),
                    Primitives.StaticSpec.abilityGrant them (keyword (.named "flying")) ]) ]) ],
      loyalty := stat 4 } }

/-- Elspeth's Talent -/
def elspethsTalentGrant : StaticSpec :=
  Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .planeswalker))
    (activated (Primitives.Cost.loyaltySymbol (.up 1))
      (createToken (creatureToken 1 1 [.white] [creatureType "Soldier"]) (count := (.lit 3))))
theorem okElspethsTalentGrant : StaticSpec.check (DRS.ofList []) elspethsTalentGrant = [] := by decide

def crystalBall : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crystal Ball", cost := some [generic 3], types := [.artifact],
      text := [activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.tapSymbol]) (scry (.lit 2))] } }

/-- Angus Mackenzie -/
def angusMackenzie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Angus Mackenzie", cost := some [pip .green, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ activatedOnlyDuring (Primitives.Cost.compound [Primitives.Cost.mana [pip .green, pip .white, pip .blue], Primitives.Cost.tapSymbol])
            (preventAll .combatOnly Primitives.DamageScope.everywhere (some thisTurn)) (beforePart .combatDamage none) ],
      power := stat 2, toughness := stat 2 } }

/-- Vampire Hexmage -/
def vampireHexmage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vampire Hexmage", cost := some [pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Vampire", creatureType "Shaman"],
      text :=
        [ keyword (.named "firstStrike"),
          activated (Primitives.Cost.perform (sacrifice thisCreature)) (removeAllCounters none
              (target isPermanent)) ],
      power := stat 2, toughness := stat 1 } }

def mizziumTransreliquat : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mizzium Transreliquat", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.mana [generic 3])
            (Primitives.Action.establish (Primitives.StaticSpec.copyChange thisArtifact (target isArtifact) []) (some untilEndOfTurn)),
          activated (Primitives.Cost.mana [generic 1, pip .blue, pip .red])
            (Primitives.Action.establish (Primitives.StaticSpec.copyChange thisArtifact (target isArtifact) [Primitives.CopyExcept.thisAbility]) none) ] } }

def tranquilGrove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tranquil Grove", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ activated (Primitives.Cost.mana [generic 1, pip .green, pip .green])
            (destroy (allOf (Primitives.Predicate.and [isEnchantment, Primitives.Predicate.otherThan Primitives.NounPhrase.this]))) ] } }

def aggressiveMining : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggressive Mining", cost := some [generic 3, pip .red], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (cant (actor you) (play (allOf isLand))),
          activatedOnlyOnce (Primitives.Cost.perform (sacrifice (a isLand))) (Primitives.Action.draw (.lit 2)) oncePerTurn ] } }

def rootGreevil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Root Greevil", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ activated
            (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .green], Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisCreature)])
            (destroy (allOf (Primitives.Predicate.and [isEnchantment, Primitives.Predicate.ofYourChoice .color none]))) ],
      power := stat 2, toughness := stat 3 } }

def riptideChronologist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riptide Chronologist", cost := some [generic 3, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .blue], Primitives.Cost.perform (sacrifice thisCreature)])
            (Primitives.Action.setStatus .untapped (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.ofYourChoice (.subtype .creature) none]))) ],
      power := stat 1, toughness := stat 3 } }

def urzasIncubator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Incubator", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact (.subtype .creature)),
          Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isCreature, isSpell, ofChosen (.subtype .creature)]) (Primitives.CostShift.less (.lit 2) none)) ] } }

def etchingsOfTheChosen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Etchings of the Chosen", cost := some [generic 1, pip .white, pip .black],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, ofChosen (.subtype .creature)]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          activated
            (Primitives.Cost.compound
              [ Primitives.Cost.mana [generic 1],
                Primitives.Cost.perform (sacrifice (a (Primitives.Predicate.and [isCreature, ofChosen (.subtype .creature)]))) ])
            (gain (target (and [isCreature, youControl])) (keyword (.named "indestructible")) (some untilEndOfTurn)) ] }
                }

/-- Volrath's Laboratory -/
def volrathsLaboratory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Volrath's Laboratory", cost := some [generic 5], types := [.artifact],
      text :=
        [ Primitives.Ability.static volrathsLaboratoryChoice,
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 5], Primitives.Cost.tapSymbol])
            (createToken
              { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
                qualities := [Primitives.TokenQuality.withQuality (ofChosen .color), Primitives.TokenQuality.withQuality (ofChosen (.subtype .creature))] }) ] } }

/-- Ersatz Gnomes -/
def ersatzGnomes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ersatz Gnomes", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Gnome"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Action.establish (Primitives.StaticSpec.qualityChange (target isSpell) .sets (Primitives.QualityPayload.colored (.some []))) none),
          activated Primitives.Cost.tapSymbol (becomeColor (target isPermanent) (.some []) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Scrapbasket -/
def scrapbasket : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scrapbasket", cost := some [generic 4], types := [.artifact, .creature],
      subtypes := [creatureType "Scarecrow"],
      text := [activated (Primitives.Cost.mana [generic 1]) (becomeColor thisCreature .every (some
          untilEndOfTurn))],
      power := stat 3, toughness := stat 2 } }

/-- Indigo Faerie -/
def indigoFaerie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Indigo Faerie", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Wizard"],
      text :=
        [ keyword (.named "flying"),
          activated (Primitives.Cost.mana [pip .blue])
            (Primitives.Action.establish (Primitives.StaticSpec.qualityChange (target isPermanent) .adds (Primitives.QualityPayload.colored (.some [.blue])))
              (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Arcum's Weathervane -/
def arcumsWeathervane : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcum's Weathervane", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol])
            (Primitives.Action.establish
              (Primitives.StaticSpec.qualityChange (target (Primitives.Predicate.and [isLand, Primitives.Predicate.hasSupertype .snow])) .loses
                (Primitives.QualityPayload.bundle { characteristics := { supertypes := [.snow] } } none))
              none),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol])
            (Primitives.Action.establish
              (Primitives.StaticSpec.qualityChange (target (Primitives.Predicate.and [isLand, Primitives.Predicate.hasSupertype .basic, Primitives.Predicate.not (Primitives.Predicate.hasSupertype
                  .snow)])) .adds
                (Primitives.QualityPayload.bundle { characteristics := { supertypes := [.snow] } } none))
              none) ] } }

def candlesOfLeng : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Candles of Leng", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 4], Primitives.Cost.tapSymbol])
            (Primitives.Action.sequentially
              [ reveal (topSlice (.lit 1)),
                Primitives.Action.doIf (Primitives.Condition.matches it ((Primitives.Predicate.shares .cardName .share (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])))))
                  (putInto it graveyard) (some (Primitives.Action.draw (.lit 1))) ]) ] } }

def sphinxOfTheChimes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphinx of the Chimes", cost := some [generic 4, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword (.named "flying"),
          activated
            (Primitives.Cost.perform (discard (withTheSameName (counted (exactly 2) (Primitives.Predicate.and [Primitives.Predicate.not isLand, Primitives.Predicate.in_ yourHand])))))
            (Primitives.Action.draw (.lit 4)) ],
      power := stat 5, toughness := stat 6 } }

def endlessAtlas : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Endless Atlas", cost := some [generic 2], types := [.artifact],
      text :=
        [ activatedOnlyIf (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol]) (Primitives.Action.draw (.lit 1))
            (Primitives.Condition.exists_ (withTheSameName (counted (atLeast 3) (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))) ] } }

/-- Amoeboid Changeling -/
def amoeboidChangelingTypeAbilities : List Ability :=
  [ activated Primitives.Cost.tapSymbol
      (Primitives.Action.establish (Primitives.StaticSpec.qualityChange (target isCreature) .adds (Primitives.QualityPayload.everyTypeOf .creature)) (some
          untilEndOfTurn)),
    activated Primitives.Cost.tapSymbol
      (Primitives.Action.establish (Primitives.StaticSpec.qualityChange (target isCreature) .loses (Primitives.QualityPayload.everyTypeOf .creature)) (some
          untilEndOfTurn)) ]
theorem okAmoeboidChangelingTypeAbilities :
    Ability.checkText (DRS.ofList []) amoeboidChangelingTypeAbilities = [] := by decide

/-- Fluctuator -/
def fluctuator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fluctuator", cost := some [generic 2], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead (.keyword (.named "cycling")), Primitives.Predicate.activatedBy Primitives.NounPhrase.you]))
            (Primitives.CostShift.less (.lit 2) none)) ] } }

/-- Boom Scholar -/
def boomScholarExhaustDiscount : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.costShift
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead (.keyword (.named "exhaust")),
                   Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.otherThan thisCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ]))
    (Primitives.CostShift.less (.lit 2) none))
theorem okBoomScholarExhaustDiscount : Ability.check (DRS.ofList []) boomScholarExhaustDiscount = [] := by decide
/-- Hulk, Gamma Goliath -/
def hulkPowerUpDiscount : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.costShift
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead (.keyword (.named "powerUp")),
                   Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.otherThan thisCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ]))
    (Primitives.CostShift.less (.lit 3) none))
theorem okHulkPowerUpDiscount : Ability.check (DRS.ofList []) hulkPowerUpDiscount = [] := by decide

/-- Kopala, Warden of Waves -/
def kopalaWardenOfWaves : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kopala, Warden of Waves", cost := some [generic 1, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [ isSpell, castBy (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)),
                           Primitives.Predicate.targets (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasSubtype (creatureType "Merfolk"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
                             .someTarget ]))
            (Primitives.CostShift.more (.lit 2))),
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.activatedBy (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)),
                           Primitives.Predicate.targets (a (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasSubtype (creatureType "Merfolk"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
                             .someTarget ]))
            (Primitives.CostShift.more (.lit 2))) ],
      power := stat 2, toughness := stat 2 } }

/-- Tithe Taker -/
def titheTaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tithe Taker", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you)
            (Primitives.StaticSpec.conjunction none
              [ spellsCost (Primitives.Predicate.and [isSpell, castBy (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))]) (Primitives.CostShift.more (.lit 1)),
                Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.activatedBy (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)),
                                      Primitives.Predicate.not Primitives.Predicate.isManaAbility ]))
                  (Primitives.CostShift.more (.lit 1)) ])),
          keywordNumber (.named "afterlife") (.lit 1) ],
      power := stat 2, toughness := stat 1 } }

/-- Vexing Shusher -/
def vexingShusher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vexing Shusher", cost := some [hybridPip .red .green, hybridPip .red .green],
      types := [.creature], subtypes := [creatureType "Goblin", creatureType "Shaman"],
      text :=
        [ Primitives.Ability.static (cant Primitives.Subject.any (counter thisSpell)),
          activated (Primitives.Cost.mana [hybridPip .red .green])
            (Primitives.Action.establish (cant Primitives.Subject.any (counter (target isSpell))) none) ],
      power := stat 2, toughness := stat 2 } }

def trainingGrounds : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Training Grounds", cost := some [pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf (allOf
            (and [isCreature, youControl]))]))
            (Primitives.CostShift.less (.lit 2) (some (.lit 1)))) ] } }

/-- Power Artifact -/
def powerArtifact : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Power Artifact", cost := some [pip .blue, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isArtifact,
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type .artifact))]))
            (Primitives.CostShift.less (.lit 2) (some (.lit 1)))) ] } }

/-- Fervent Champion -/
def ferventChampionEquipDiscount : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.costShift
    (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead (.keyword (.named "equip")), Primitives.Predicate.activatedBy Primitives.NounPhrase.you, Primitives.Predicate.targets thisCreature .someTarget]))
    (Primitives.CostShift.less (.lit 3) none))
theorem okFerventChampionEquipDiscount : Ability.check (DRS.ofList []) ferventChampionEquipDiscount = [] := by decide

def suppressionField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Suppression Field", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.not Primitives.Predicate.isManaAbility]))
            (Primitives.CostShift.more (.lit 2))) ] } }

def gloom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gloom", cost := some [generic 2, pip .black], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isSpell, Primitives.Predicate.colorIs .white]) (Primitives.CostShift.more (.lit 3))),
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [isEnchantment, Primitives.Predicate.colorIs .white]))]))
            (Primitives.CostShift.more (.lit 3))) ] } }

def bureauHeadmaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bureau Headmaster", cost := some [pip .red, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isSpell, Primitives.Predicate.hasSubtype (artifactType "Equipment"), castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none)),
          Primitives.Ability.static (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead (.keyword (.named "equip")), Primitives.Predicate.activatedBy Primitives.NounPhrase.you]))
              (Primitives.CostShift.less (.lit 1) none)) ],
      power := stat 2, toughness := stat 2 } }

def oppressiveRaysLine : StaticSpec :=
  Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type
      .creature))]))
    (Primitives.CostShift.more (.lit 3))
theorem okOppressiveRaysLine : StaticSpec.check (DRS.ofList []) oppressiveRaysLine = [] := by decide

def eidolonOfObstruction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eidolon of Obstruction", cost := some [generic 1, pip .white],
      types := [.enchantment, .creature], subtypes := [creatureType "Spirit"],
      text :=
        [ keyword (.named "firstStrike"),
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .loyalty,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))])) ]))
            (Primitives.CostShift.more (.lit 1))) ],
      power := stat 2, toughness := stat 1 } }

def forceOfWill : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Force of Will", cost := some [generic 3, pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.compound
              [ payLife 1,
                Primitives.Cost.perform (exile (a (Primitives.Predicate.and [Primitives.Predicate.colorIs .blue, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)]))) ]))),
          Primitives.Ability.spell none (counter (target isSpell)) ] } }

def demonOfDeathsGate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Demon of Death's Gate", cost := some [generic 6, pip .black, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Demon"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.compound
              [ payLife 6,
                Primitives.Cost.perform (sacrifice (counted (exactly 3) (Primitives.Predicate.and [isCreature, Primitives.Predicate.colorIs .black]))) ]))),
          keyword (.named "flying"), keyword (.named "trample") ],
      power := stat 9, toughness := stat 9 } }

def drudgeSkeletons : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drudge Skeletons", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Skeleton"],
      text := [activated (Primitives.Cost.mana [pip .black]) (regenerate thisCreature)],
      power := stat 1, toughness := stat 1 } }

def asphodelWanderer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Asphodel Wanderer", cost := some [pip .black], types := [.creature],
      subtypes := [creatureType "Skeleton", creatureType "Soldier"],
      text := [activated (Primitives.Cost.mana [generic 2, pip .black]) (regenerate thisCreature)],
      power := stat 1, toughness := stat 1 } }

def hurrJackalAbility : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Action.establish (cant Primitives.Subject.any (regeneration (target isCreature))) (some thisTurn))
theorem okHurrJackalAbility : Ability.check (DRS.ofList []) hurrJackalAbility = [] := by decide

def wickedAkuba : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wicked Akuba", cost := some [pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ activated (Primitives.Cost.mana [pip .black])
            (instruct (target (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, happenedTo (Primitives.Subject.actor thisCreature) (Primitives.Action.dealDamage Primitives.Amount.anyAmount (relative .player) .any) soFarThisTurn])) (loseLife
              (.lit 1))) ],
      power := stat 2, toughness := stat 2 } }

def idolOfOblivion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Idol of Oblivion", cost := some [generic 2], types := [.artifact],
      text :=
        [ activatedOnlyIf Primitives.Cost.tapSymbol (Primitives.Action.draw (.lit 1))
            (happened (Primitives.Action.createObject Primitives.Amount.anyAmount (Primitives.CreationSpec.token (Primitives.TokenSpec.copyOf (a Primitives.Predicate.isToken) []) []) false none) (Primitives.Subject.actor Primitives.NounPhrase.you) soFarThisTurn),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 8], Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisArtifact)])
            (createToken (creatureToken 10 10 [] [creatureType "Eldrazi"])) ] } }

def patricianGeist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Patrician Geist", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit", creatureType "Knight"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Spirit"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isSpell, castBy Primitives.NounPhrase.you, Primitives.Predicate.castFrom (graveyardOf Primitives.NounPhrase.you)]) (Primitives.CostShift.less (.lit 1) none)) ],
      power := stat 2, toughness := stat 2 } }

def shatteredEgo : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shattered Ego", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.down (.lit 3)) (Primitives.Delta.down (.lit 0))),
          activated (Primitives.Cost.mana [generic 3, pip .blue, pip .blue])
            (Primitives.Action.move (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) Primitives.Location.anyLocation (some (Primitives.Location.fromEnd .top (.nth 3) (Primitives.Location.zone .library none))) .arrival none []) ] } }

def shuFarmer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shu Farmer", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ activatedOnlyDuring Primitives.Cost.tapSymbol (gainLife (.lit 1)) (beforePart
            .declareAttackers (some Primitives.NounPhrase.you)) ],
      power := stat 1, toughness := stat 1 } }

def elspethsTalent : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elspeth's Talent", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") (Primitives.Predicate.hasType .planeswalker),
          Primitives.Ability.static elspethsTalentGrant,
          whenever (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .activate loyaltyAbilityOfEnchanted Primitives.PlayPayment.itsOwnCost none none none))
            (Primitives.Action.establish
              (Primitives.StaticSpec.conjunction none
                [ getsPt (allOf (and [isCreature, youControl])) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up
                      (.lit 2)),
                  Primitives.StaticSpec.abilityGrant them (keyword (.named "vigilance")) ])
              (some untilEndOfTurn)) ] } }

/-- Void Maw -/
def voidMawPutCost : Ability :=
  activated (Primitives.Cost.perform (put (a (Primitives.Predicate.in_ (Primitives.Location.exiledWith thisCreature))) graveyard))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn))
theorem okVoidMawPutCost : Ability.check (DRS.ofList []) voidMawPutCost = [] := by decide
/-- Ghor-Clan Rampager -/
def ghorClanRampager : Ability :=
  abilityWord "bloodrush"
    (activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .red, pip .green], Primitives.Cost.perform (discard Primitives.NounPhrase.this)])
      (establishFor (target (Primitives.Predicate.and [isCreature, isAttacking]))
        [ getsPt ((target (Primitives.Predicate.and [isCreature, isAttacking]))) (Primitives.Delta.up (.lit 4)) (Primitives.Delta.up (.lit
              4)),
          Primitives.StaticSpec.abilityGrant ((target (Primitives.Predicate.and [isCreature, isAttacking]))) (keyword (.named "trample")) ]
        (some untilEndOfTurn)))
theorem okGhorClanRampager : Ability.check (DRS.ofList []) ghorClanRampager = [] := by decide
/-- Tymora's Invoker -/
def tymorasInvoker : Ability :=
  flavorWord "Sleight of Hand" (activated (Primitives.Cost.mana [generic 8]) (Primitives.Action.draw (.lit 2)))
theorem okTymorasInvoker : Ability.check (DRS.ofList []) tymorasInvoker = [] := by decide
/-- Skyblade's Boon -/
def skybladesBoonReturn : Ability :=
  activatedOnlyIf (Primitives.Cost.mana [generic 2, pip .white]) (putInto Primitives.NounPhrase.this hand)
    (Primitives.Condition.or [Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.in_ battlefield), Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])])
theorem okSkybladesBoonReturn : Ability.check (DRS.ofList []) skybladesBoonReturn = [] := by decide
/-- Bamboozling Beeble -/
def bamboozlingBeebleIgnore : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.tapSymbol])
    (replaceNextEvent (Primitives.WouldClause.would (Primitives.Subject.actor (target Primitives.Predicate.anyPlayer)) (Primitives.Action.rollDice Primitives.Amount.anyAmount .anyDie .many Primitives.RollWatch.anyResult))
      (Primitives.Action.sequentially
        [ instruct they (Primitives.Action.rollDice (plus Primitives.Amount.thatMuch (.lit 1)) .thoseDice .many .anyResult),
          Primitives.Action.ignoreOutcomes (Primitives.IgnoredOutcomes.chosen (some Primitives.NounPhrase.you) (.lit 1)) ])
      (some thisTurn))
theorem okBamboozlingBeebleIgnore : Ability.check (DRS.ofList []) bamboozlingBeebleIgnore = [] := by decide
/-- General Tazri's pump -/
def generalTazriPump : Ability :=
  activated (Primitives.Cost.mana [pip .white, pip .blue, pip .black, pip .red, pip .green])
    (Primitives.Action.sequentially
      [ get (each (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasSubtype (creatureType "Ally"), Primitives.Predicate.hasPossessor .controller
          Primitives.NounPhrase.you]))
          (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (Primitives.Amount.letter .x)) (some untilEndOfTurn),
        Primitives.Action.define .x (Primitives.Amount.distinctCount .color (those (.type .creature))) ])
theorem okGeneralTazriPump : Ability.check (DRS.ofList []) generalTazriPump = [] := by decide
/-- Diplomatic Escort -/
def diplomaticEscortLine : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .blue], Primitives.Cost.tapSymbol, Primitives.Cost.perform (discard (a (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ yourHand])))])
    (counter
      (target (Primitives.Predicate.and [Primitives.Predicate.or [isSpell, Primitives.Predicate.abilityHead .anyOnStack], Primitives.Predicate.targets (a isCreature) .someTarget])))
theorem okDiplomaticEscortLine : Ability.check (DRS.ofList []) diplomaticEscortLine = [] := by decide

/-- Vorel of the Hull Clade -/
def vorelOfTheHullClade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vorel of the Hull Clade", cost := some [generic 1, pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Merfolk"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .green, pip .blue], Primitives.Cost.tapSymbol])
            (Primitives.Action.doubleCounters (target (Primitives.Predicate.or [isArtifact, isCreature, isLand]))) ],
      power := stat 1, toughness := stat 4 } }

/-- Prosperity -/
theorem prosperityCostLetters : costLetters (some [.variable, pip .blue]) = [letterB .x] := rfl
def prosperityTextLetter : Amount := Primitives.Amount.letter .x
theorem prosperityTextReadsCostLetter :
    let context := DRS.ofList (costLetters (some [.variable, pip .blue]))
    let out := (stepAmount 64 context prosperityTextLetter).1
    out.referents.filter (fun ref => ref.id ≥ context.nextId) = [] := by decide
theorem noVariableSymbolNoLetter : costLetters (some [generic 1, pip .blue]) = [] := by decide
theorem noCostNoLetter : costLetters none = [] := by decide

/-- Sugar Coat -/
def sugarCoat : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sugar Coat", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword (.named "flash"),
          keywordSubject (.named "enchant") (Primitives.Predicate.or [isCreature, Primitives.Predicate.hasSubtype (artifactType "Food")]),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted .permanent) .sets
                (Primitives.QualityPayload.bundle
                  { characteristics :=
                    { types := [.artifact], subtypes := [artifactType "Food"],
                      text :=
                        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice
                            thisArtifact)])
                            (gainLife (.lit 3)) ] } }
                  none),
              Primitives.StaticSpec.allAbilityLoss it none ]) ] } }

/-- Doc Aurlock, Grizzled Genius -/
def docAurlockCost : StaticSpec :=
  spellsCost (Primitives.Predicate.and [isSpell, castBy Primitives.NounPhrase.you, Primitives.Predicate.or [Primitives.Predicate.castFrom (graveyardOf Primitives.NounPhrase.you), Primitives.Predicate.castFrom exileZone]])
    (Primitives.CostShift.less (.lit 2) none)
theorem okDocAurlockCost : StaticSpec.check (DRS.ofList []) docAurlockCost = [] := by decide
/-- Obelisk of Undoing -/
def obeliskOfUndoing : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 6], Primitives.Cost.tapSymbol])
    (returnTo (target (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) hand [])
theorem okObeliskOfUndoing : Ability.check (DRS.ofList []) obeliskOfUndoing = [] := by decide
/-- Flickering Ward -/
def flickeringWardBounce : Ability := activated (Primitives.Cost.mana [pip .white]) (returnTo thisAura hand [])
theorem okFlickeringWardBounce : Ability.check (DRS.ofList []) flickeringWardBounce = [] := by decide
/-- Soul Conduit -/
def soulConduitExchange : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 6], Primitives.Cost.tapSymbol])
    (Primitives.Action.exchange (Primitives.Exchanged.lifeTotals (Primitives.NounPhrase.described (Primitives.DetPhrase.target (exactly 2)) Primitives.Predicate.anyPlayer)))
theorem okSoulConduitExchange : Ability.check (DRS.ofList []) soulConduitExchange = [] := by decide

/-- Death-Mask Duplicant -/
def deathMaskDuplicant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death-Mask Duplicant", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ abilityWord "imprint"
            (activated (Primitives.Cost.mana [generic 1]) (exile (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])))),
          Primitives.Ability.alsoForKeywords
            (Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword (.named "flying")))
              (exists_ (Primitives.Predicate.and [Primitives.Predicate.in_ (Primitives.Location.exiledWith thisCreature), Primitives.Predicate.hasKeyword (.the (.named "flying"))]))))
            [ .the (.named "fear"), .the (.named "firstStrike"), .the (.named "doubleStrike"), .the (.named "haste"), landwalkAbilities,
              protectionAbilities, .the (.named "trample") ] ],
      power := stat 5, toughness := stat 5 } }

/-- Darksteel Garrison -/
def darksteelGarrison : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darksteel Garrison", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Fortification"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .fortified (.type .land)) (keyword (.named "indestructible"))),
          whenever (Primitives.GameEvent.does Primitives.Subject.any (Primitives.Action.tapForMana (Primitives.NounPhrase.attachHost .fortified (.type .land)) none))
            (get (target isCreature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)),
          keywordCosting (.named "fortify") (Primitives.Cost.mana [generic 3]) ] } }

/-- Embercleave -/
def embercleave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Embercleave", cost := some [generic 4, pip .red, pip .red], supertypes := [.legendary],
      types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ keyword (.named "flash"),
          Primitives.Ability.static (Primitives.StaticSpec.costShift Primitives.NounPhrase.this
            (Primitives.CostShift.less (forEach (.lit 1) (Primitives.Predicate.and [isCreature, isAttacking, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) none)),
          when (does (actor thisEquipment) enter) (attachTo it (target (and [isCreature, youControl]))),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction (some (Primitives.NounPhrase.attachHost .equipped (.type .creature)))
            [ getsPt ((Primitives.NounPhrase.attachHost .equipped (.type .creature))) (Primitives.Delta.up (.lit
                1)) (Primitives.Delta.up
                  (.lit 1)),
              Primitives.StaticSpec.abilityGrant ((Primitives.NounPhrase.attachHost .equipped (.type .creature))) (keyword
                  (.named "doubleStrike")),
              Primitives.StaticSpec.abilityGrant ((Primitives.NounPhrase.attachHost .equipped (.type .creature))) (keyword
                  (.named "trample")) ]),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 3]) ] } }

/-- Tattoo Ward -/
def tattooWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tattoo Ward", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.conjunction (some (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
              [ getsPt ((Primitives.NounPhrase.attachHost .enchanted (.type .creature))) (Primitives.Delta.up
                  (.lit 1)) (Primitives.Delta.up (.lit 1)),
                Primitives.StaticSpec.abilityGrant ((Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
                  (keywordQuality (.named "protection") isEnchantment) ])
            thisAura),
          activated (Primitives.Cost.perform (sacrifice thisAura)) (destroy (target isEnchantment)) ]
              } }

/-- Floating Shield -/
def floatingShield : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Floating Shield", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          Primitives.Ability.static (entersChoosing thisAura .color),
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keywordQuality (.named "protection")
                (ofChosen .color)))
            thisAura),
          activated (Primitives.Cost.perform (sacrifice thisAura))
            (gain (target isCreature) (keywordQuality (.named "protection") (ofChosen .color)) (some
                untilEndOfTurn)) ] } }

/-- Ghostfire Blade -/
def ghostfireBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostfire Blade", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2))),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 3]),
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead (.keyword (.named "equip")), Primitives.Predicate.abilityOf Primitives.NounPhrase.this,
                           Primitives.Predicate.targets (a (Primitives.Predicate.and [isCreature, isColorless])) .someTarget ]))
            (Primitives.CostShift.less (.lit 2) none)) ] } }

/-- Academy Journeymage -/
def academyJourneymage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Academy Journeymage", cost := some [generic 4, pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (onlyIfSo (Primitives.StaticSpec.costShift Primitives.NounPhrase.this (Primitives.CostShift.less (.lit 1) none))
            (exists_ (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Wizard"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
          when (does (actor thisCreature) enter)
            (putInto (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller anOpponent])) hand) ],
      power := stat 3, toughness := stat 2 } }

/-- Alabaster Leech -/
def alabasterLeech : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Alabaster Leech", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Leech"],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [isSpell, Primitives.Predicate.colorIs .white, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.bySymbols (Primitives.ManaCostTerm.lit [pip .white]) true false))],
      power := stat 1, toughness := stat 3 } }

/-- Edgewalker -/
def edgewalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Edgewalker", cost := some [generic 1, pip .white, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Cleric"), isSpell, castBy Primitives.NounPhrase.you])
            (Primitives.CostShift.bySymbols (Primitives.ManaCostTerm.lit [pip .white, pip .black]) false true)) ],
      power := stat 2, toughness := stat 2 } }

/-- Cavern-Hoard Dragon -/
def cavernHoardDragonRider : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.costShift Primitives.NounPhrase.this (Primitives.CostShift.less (Primitives.Amount.letter .x) none),
      Primitives.StaticSpec.letterDefinition .x
        (Primitives.Amount.aggregateOver .max (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you) (countOf (Primitives.Predicate.and [isArtifact, Primitives.Predicate.hasPossessor .controller they]))) ])
theorem okCavernHoardDragonRider : Ability.check (DRS.ofList []) cavernHoardDragonRider = [] := by decide
/-- Shadowspear -/
def shadowspearStrip : Ability :=
  activated (Primitives.Cost.mana [generic 1])
    (Primitives.Action.establish
      (Primitives.StaticSpec.abilityLoss (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))]))
        [Primitives.AbilityLost.written (keyword (.named "hexproof")), Primitives.AbilityLost.written (keyword (.named "indestructible"))])
      (some untilEndOfTurn))
theorem okShadowspearStrip : Ability.check (DRS.ofList []) shadowspearStrip = [] := by decide
/-- Shay Cormac -/
def shayCormacStrip : Ability :=
  activated (Primitives.Cost.mana [generic 1])
    (Primitives.Action.establish
      (Primitives.StaticSpec.abilityLoss (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))]))
        [ Primitives.AbilityLost.written (keyword (.named "hexproof")), Primitives.AbilityLost.written (keyword (.named "indestructible")), Primitives.AbilityLost.term protectionAbilities,
          Primitives.AbilityLost.written (keyword (.named "shroud")), Primitives.AbilityLost.term wardAbilities ])
      (some untilEndOfTurn))
theorem okShayCormacStrip : Ability.check (DRS.ofList []) shayCormacStrip = [] := by decide

/-- Shelkin Brownie -/
def shelkinBrownie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shelkin Brownie", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Ouphe"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol])
            (Primitives.Action.establish (Primitives.StaticSpec.abilityLoss (target isCreature) [Primitives.AbilityLost.term bandsWithOtherAbilities])
              (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Ahn-Crop Invader -/
def ahnCropInvader : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ahn-Crop Invader", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Zombie", creatureType "Minotaur", creatureType "Warrior"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you) (Primitives.StaticSpec.abilityGrant thisCreature (keyword
            (.named "firstStrike")))),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.perform (sacrifice (a (Primitives.Predicate.and [isCreature,
              Primitives.Predicate.otherThan Primitives.NounPhrase.this])))])
            (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 2 } }

/-- Nesting Dragon -/
def nestingDragonInnerToken : Ability :=
  activated (Primitives.Cost.mana [pip .red])
    (Primitives.Action.establish
      (Primitives.StaticSpec.conjunction none
        [ getsPt (Primitives.NounPhrase.asMarker .token Primitives.NounPhrase.this) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) ])
      (some untilEndOfTurn))
theorem okNestingDragonInnerToken : Ability.check (DRS.ofList []) nestingDragonInnerToken = [] := by decide

/-- Leonin Bola -/
def leoninBola : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leonin Bola", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .equipped (.type .creature))
            (activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (Primitives.Action.unattach (Primitives.NounPhrase.theGrantor .permanent))])
              (Primitives.Action.setStatus .tapped (target isCreature)))),
          keywordCosting (.named "equip") (Primitives.Cost.mana [generic 1]) ] } }

/-- Alluring Suitor // Deadly Dancer -/
def deadlyDancerPump : Ability :=
  activated (Primitives.Cost.mana [pip .red, pip .red])
    (Primitives.Action.establish
      (Primitives.StaticSpec.conjunction none
        [ Primitives.StaticSpec.modification (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.both thisCreature (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.otherThan Primitives.NounPhrase.this]))))
            .power (Primitives.Delta.up (.lit 1)),
          Primitives.StaticSpec.modification (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.both thisCreature it)) .toughness (Primitives.Delta.up (.lit 0)) ])
      (some untilEndOfTurn))
theorem okDeadlyDancerPump : Ability.check (DRS.ofList []) deadlyDancerPump = [] := by decide

/-- Unesh, Criosphinx Sovereign -/
def uneshCriosphinxSovereign : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unesh, Criosphinx Sovereign", cost := some [generic 4, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword (.named "flying"),
          Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Sphinx"), isSpell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 2) none)),
          whenever
            (does (actor (Primitives.NounPhrase.eitherOf thisCreature
              (a (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Sphinx"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature])))) enter)
            (Primitives.Action.sequentially
              [ reveal (topSlice (.lit 4)),
                instruct anOpponent (Primitives.Action.separateIntoPiles them 2 []),
                putInto onePile hand,
                putInto (theOther .pile) graveyard ]) ],
      power := stat 4, toughness := stat 4 } }

def opt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Opt", cost := some [pip .blue], types := [.instant],
      text := [Primitives.Ability.spell none (Primitives.Action.sequentially [scry (.lit 1), Primitives.Action.draw (.lit 1)])] } }

def serumVisions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Serum Visions", cost := some [pip .blue], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Action.sequentially [Primitives.Action.draw (.lit 1), scry (.lit 2)])] } }

def consider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consider", cost := some [pip .blue], types := [.instant],
      text := [Primitives.Ability.spell none (Primitives.Action.sequentially [surveil (.lit 1), Primitives.Action.draw (.lit 1)])] } }

def wordsOfWisdom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Words of Wisdom", cost := some [generic 1, pip .blue], types := [.instant],
      text := [Primitives.Ability.spell none (Primitives.Action.sequentially [Primitives.Action.draw (.lit 2), instruct (each isOtherPlayer) (Primitives.Action.draw (.lit 1))])] } }

def deathWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death Ward", cost := some [pip .white], types := [.instant],
      text := [Primitives.Ability.spell none (regenerate (target isCreature))] } }

def bareTextLetter : Amount := Primitives.Amount.letter .x
theorem textAloneOnceMintedItsOwnLetter :
    (stepAmount 64 (DRS.ofList []) bareTextLetter).1.referents = [letterB .x] := by decide

end Semantics.Cards
