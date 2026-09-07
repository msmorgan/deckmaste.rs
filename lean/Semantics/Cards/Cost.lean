import Semantics
import Semantics.Macros
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

open Semantics Semantics.Macros

namespace Semantics.Cards

def masterDecoy : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .white], Primitives.Cost.tapSymbol]) (tap (target creature))
theorem okMasterDecoy : Ability.check [] masterDecoy = [] := by decide
def cycling : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.perform (discard Primitives.NounPhrase.this (agent := Primitives.NounPhrase.you))]) (Primitives.Instruction.draw (.lit
      1) (agent := Primitives.NounPhrase.you))
theorem okCycling : Ability.check [] cycling = [] := by decide
def merrowGrimeblotter : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1, hybridPip .blue .black], Primitives.Cost.untapSymbol])
    (get (target creature) (Primitives.Delta.down (.lit 2)) (Primitives.Delta.down (.lit 0)) (some untilEndOfTurn))
theorem okMerrowGrimeblotter : Ability.check [] merrowGrimeblotter = [] := by decide
def phyrexianSnowcrusher : Ability :=
  activated (Primitives.Cost.mana [generic 1, .snow])
    (get thisCreature (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn))
theorem okPhyrexianSnowcrusher : Ability.check [] phyrexianSnowcrusher = [] := by decide
def havocSower : Ability :=
  activated (Primitives.Cost.mana [generic 1, colorlessPip])
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn))
theorem okHavocSower : Ability.check [] havocSower = [] := by decide
/-- Erebos, God of the Dead -/
def erebos : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1, pip .black], payLife Primitives.NounPhrase.you 2]) (Primitives.Instruction.draw (.lit 1) (agent :=
      Primitives.NounPhrase.you))
theorem okErebos : Ability.check [] erebos = [] := by decide
def baskingRootwalla : Ability :=
  activatedOnlyOnce (Primitives.Cost.mana [generic 1, pip .green])
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn)) Primitives.UsageLimit.oncePerTurn
theorem okBaskingRootwalla : Ability.check [] baskingRootwalla = [] := by decide
def securityDetail : Ability :=
  activatedOnlyOnceIf (Primitives.Cost.mana [pip .white, pip .white])
    (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"])) Primitives.UsageLimit.oncePerTurn
    (Primitives.Condition.not (exists_ (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
theorem okSecurityDetail : Ability.check [] securityDetail = [] := by decide
def aphettoAlchemist : Ability :=
  activated Primitives.Cost.tapSymbol (Primitives.Instruction.setStatus .untapped (target (Primitives.Predicate.or [artifact, creature])))
theorem okAphettoAlchemist : Ability.check [] aphettoAlchemist = [] := by decide

def charRumbler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Char-Rumbler", cost := some [generic 2, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ keyword "DoubleStrike",
          activated (Primitives.Cost.mana [pip .red])
            (get thisCreature (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ],
      power := some (.lit (-1)), toughness := stat 3 } }

/-- Sisters of Stone Death -/
def sistersOfStoneDeathRecall : Ability :=
  activated (Primitives.Cost.mana [generic 2, pip .black])
    (putOntoBattlefieldUnderYourControl (a (Primitives.Predicate.and [creature, Primitives.Predicate.exiledWith thisCreature])))
theorem okSistersOfStoneDeathRecall : Ability.check [] sistersOfStoneDeathRecall = [] := by decide
/-- Synod Sanctum -/
def synodSanctumReturn : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.perform (sacrifice thisArtifact (agent := Primitives.NounPhrase.you))])
    (putOntoBattlefieldUnderYourControl (allOf exiledWithThisArtifact))
theorem okSynodSanctumReturn : Ability.check [] synodSanctumReturn = [] := by decide

def coldStorage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cold Storage", cost := some [generic 4], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.mana [generic 3]) (exile (target creatureYouControl)),
          activated (Primitives.Cost.perform (sacrifice thisArtifact (agent := Primitives.NounPhrase.you)))
            (putOntoBattlefieldUnderYourControl (each (Primitives.Predicate.and [creature, exiledWithThisArtifact]))) ] } }

def barlsCage : Ability :=
  activated (Primitives.Cost.mana [generic 3]) (Primitives.Instruction.skipUntap (target creature) (.lit 1))
theorem okBarlsCage : Ability.check [] barlsCage = [] := by decide
def vodalianIllusionist : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .blue, pip .blue], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.setStatus .phasedOut (target creature))
theorem okVodalianIllusionist : Ability.check [] vodalianIllusionist = [] := by decide
def witchsMist : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .black], Primitives.Cost.tapSymbol])
    (destroy (target (Primitives.Predicate.and [creature, happenedTo
      (Primitives.GameEvent.isDealtDamage .any (Primitives.NounPhrase.asMarker .permanent (relative
      .object))) .thisTurn])))
theorem okWitchsMist : Ability.check [] witchsMist = [] := by decide
def goadTargetCreature : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.gainDesignation (target creature) "goaded" none)
theorem okGoadTargetCreature : Ability.check [] goadTargetCreature = [] := by decide
/-- Krenko, Mob Boss -/
def krenko : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Instruction.sequentially
      [ Primitives.Instruction.create (Primitives.Amount.letter .x) (Primitives.TokenSpec.written (creatureToken 1 1 [.red] [creatureType "Goblin"])) [] (agent
          := Primitives.NounPhrase.you),
        Primitives.Instruction.define .x (countOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Goblin"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ])
theorem okKrenko : Ability.check [] krenko = [] := by decide
/-- Dokai, Weaver of Life -/
def dokai : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 4, pip .green, pip .green], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.sequentially
      [ create (.lit 1) (creatureTokenOf (Primitives.Amount.letter .x) (Primitives.Amount.letter .x) [.green] [creatureType "Elemental"]),
        Primitives.Instruction.define .x (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ])
theorem okDokai : Ability.check [] dokai = [] := by decide

def ghalta : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghalta, Primal Hunger", cost := some [generic 10, pip .green, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elder", creatureType "Dinosaur"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.costShift Primitives.NounPhrase.this (Primitives.CostShift.less (Primitives.Amount.letter .x) none),
              Primitives.StaticSpec.letterDefinition .x (aggregate .sum (.stat .power) creatureYouControl) ]),
          keyword "Trample" ],
      power := stat 12, toughness := stat 12 } }

def ancientStoneIdol : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.costShift Primitives.NounPhrase.this (Primitives.CostShift.less (forEach 1 (Primitives.Predicate.and [creature, attacking])) none))
theorem okAncientStoneIdol : Ability.check [] ancientStoneIdol = [] := by decide

/-- "<p> spells cost {N} more/less to cast" -/
def spellsCost (p : Predicate) (shift : CostShift) : StaticSpec := Primitives.StaticSpec.costShift (allOf p) shift

def thornOfAmethyst : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thorn of Amethyst", cost := some [generic 2], types := [.artifact],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.not creature, spell]) (Primitives.CostShift.more (.lit 1)))] } }

def ferozsBan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Feroz's Ban", cost := some [generic 6], types := [.artifact],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [creature, spell]) (Primitives.CostShift.more (.lit 2)))] } }

def urzasFilter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Filter", cost := some [generic 4], types := [.artifact],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [multicolored, spell]) (Primitives.CostShift.less (.lit 2) none))] } }

def emeraldMedallion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Emerald Medallion", cost := some [generic 2], types := [.artifact],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.colorIs .green, spell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none))] } }

/-- Highspire Bell-Ringer -/
def highspireBellRinger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Highspire Bell-Ringer", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Djinn", creatureType "Monk"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (Primitives.StaticSpec.costShift (the (Primitives.Predicate.and [spell, nthCastBy (.nth 2) Primitives.NounPhrase.you (.each .turn)])) (Primitives.CostShift.less
              (.lit 1) none)) ],
      power := stat 1, toughness := stat 4 } }

def foundryInspector : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foundry Inspector", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [artifact, spell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none))],
      power := stat 3, toughness := stat 2 } }

def daruWarchief : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daru Warchief", cost := some [generic 2, pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Soldier"), spell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none)),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Soldier"), creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 2))) ],
      power := stat 1, toughness := stat 1 } }

def grandArbiter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grand Arbiter Augustin IV", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Advisor"],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.colorIs .white, spell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none)),
          Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.colorIs .blue, spell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none)),
          Primitives.Ability.static (spellsCost (Primitives.Predicate.and [spell, castBy (Primitives.NounPhrase.playerGroup .yourOpponents)]) (Primitives.CostShift.more (.lit 1))) ],
      power := stat 2, toughness := stat 3 } }

def goblinElectromancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Goblin Electromancer", cost := some [pip .blue, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Wizard"],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [instantOrSorcery, spell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none))],
      power := stat 2, toughness := stat 2 } }

def arcaneMelee : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Melee", cost := some [generic 4, pip .blue], types := [.enchantment],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [instantOrSorcery, spell]) (Primitives.CostShift.less (.lit 2) none))] } }

def manaMatrix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Matrix", cost := some [generic 6], types := [.artifact],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.or [instant, enchantment], spell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 2) none)) ] } }

def auraOfSilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aura of Silence", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.or [artifact, enchantment], spell, castBy (Primitives.NounPhrase.playerGroup .yourOpponents)])
            (Primitives.CostShift.more (.lit 2))),
          activated (Primitives.Cost.perform (sacrifice thisEnchantment (agent := Primitives.NounPhrase.you)))
            (destroy (target (Primitives.Predicate.or [artifact, enchantment]))) ] } }

def chillerpillar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chillerpillar", cost := some [generic 3, pip .blue], supertypes := [.snow],
      types := [.creature], subtypes := [creatureType "Insect"],
      text :=
        [ activated (Primitives.Cost.mana [generic 4, .snow, .snow]) (makeMonstrous (.lit 2)),
          Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword "Flying"))
            (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasDesignation "monstrous" none))) ],
      power := stat 3, toughness := stat 3 } }

def nullhideFerox : Ability :=
  activated (Primitives.Cost.mana [generic 2])
    (Primitives.Instruction.establish (Primitives.StaticSpec.allAbilityLoss thisCreature none) (some untilEndOfTurn))
theorem okNullhideFerox : Ability.check [] nullhideFerox = [] := by decide

def causticTar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caustic Tar", cost := some [generic 4, pip .black, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .land))
            (activated Primitives.Cost.tapSymbol (loseLife (.lit 3) (agent := (target Primitives.Predicate.anyPlayer))))) ] } }

/-- Saheeli, Filigree Master -/
def saheelisEmblem : Instruction :=
  Primitives.Instruction.getEmblem
    [ Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [artifact, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
        (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
      Primitives.Ability.static (spellsCost (Primitives.Predicate.and [artifact, spell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none)) ] (agent :=
          Primitives.NounPhrase.you)
theorem okSaheelisEmblem : Instruction.check [] saheelisEmblem = [] := by decide

def jaceBeleren : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Jace Beleren", cost := some [generic 1, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Jace"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 2)) (Primitives.Instruction.draw (.lit 1) (agent := (each Primitives.Predicate.anyPlayer))),
          activated (Primitives.Cost.loyaltySymbol (.down 1)) (Primitives.Instruction.draw (.lit 1) (agent := (target Primitives.Predicate.anyPlayer))),
          activated (Primitives.Cost.loyaltySymbol (.down 10)) (mill (.lit 20) they (agent := (target Primitives.Predicate.anyPlayer)))
              ],
      loyalty := stat 3 } }

def elspethSunsChampion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elspeth, Sun's Champion", cost := some [generic 4, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Elspeth"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 1))
            (create (.lit 3) (creatureToken 1 1 [.white] [creatureType "Soldier"])),
          activated (Primitives.Cost.loyaltySymbol (.down 3))
            (destroy (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4)]))),
          activated (Primitives.Cost.loyaltySymbol (.down 7))
            (Primitives.Instruction.getEmblem
              [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
                  [ Primitives.StaticSpec.modification (allOf creatureYouControl) .power (Primitives.Delta.up (.lit 2)),
                    Primitives.StaticSpec.modification (itsOther (allOf creatureYouControl) (Primitives.Delta.up (.lit 2))) .toughness
                        (Primitives.Delta.up (.lit 2)),
                    Primitives.StaticSpec.abilityGrant them (keyword "Flying") ]) ] (agent := Primitives.NounPhrase.you)) ],
      loyalty := stat 4 } }

/-- Elspeth's Talent -/
def elspethsTalentGrant : StaticSpec :=
  Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .planeswalker))
    (activated (Primitives.Cost.loyaltySymbol (.up 1))
      (create (.lit 3) (creatureToken 1 1 [.white] [creatureType "Soldier"])))
theorem okElspethsTalentGrant : StaticSpec.check [] elspethsTalentGrant = [] := by decide

def crystalBall : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crystal Ball", cost := some [generic 3], types := [.artifact],
      text := [activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.tapSymbol]) (scry (.lit 2) (agent :=
          Primitives.NounPhrase.you))] } }

/-- Angus Mackenzie -/
def angusMackenzie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Angus Mackenzie", cost := some [pip .green, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ activatedOnlyDuring (Primitives.Cost.compound [Primitives.Cost.mana [pip .green, pip .white, pip .blue], Primitives.Cost.tapSymbol])
            (preventAll .combatOnly Primitives.DamageScope.everywhere (some Primitives.Duration.thisTurn)) (Primitives.Timing.beforePart .combatDamage none) ],
      power := stat 2, toughness := stat 2 } }

/-- Vampire Hexmage -/
def vampireHexmage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vampire Hexmage", cost := some [pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Vampire", creatureType "Shaman"],
      text :=
        [ keyword "FirstStrike",
          activated (Primitives.Cost.perform (sacrifice thisCreature (agent := Primitives.NounPhrase.you))) (removeAllCounters none
              (target permanent)) ],
      power := stat 2, toughness := stat 1 } }

def mizziumTransreliquat : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mizzium Transreliquat", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.mana [generic 3])
            (Primitives.Instruction.establish (Primitives.StaticSpec.copyChange thisArtifact (target artifact) []) (some untilEndOfTurn)),
          activated (Primitives.Cost.mana [generic 1, pip .blue, pip .red])
            (Primitives.Instruction.establish (Primitives.StaticSpec.copyChange thisArtifact (target artifact) [Primitives.CopyExcept.thisAbility]) none) ] } }

def tranquilGrove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tranquil Grove", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ activated (Primitives.Cost.mana [generic 1, pip .green, pip .green])
            (destroy (allOf (Primitives.Predicate.and [enchantment, Primitives.Predicate.otherThan Primitives.NounPhrase.this]))) ] } }

def aggressiveMining : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggressive Mining", cost := some [generic 3, pip .red], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (cantDoTo (.action "Play") Primitives.NounPhrase.you (allOf land)),
          activatedOnlyOnce (Primitives.Cost.perform (sacrifice (a land) (agent := Primitives.NounPhrase.you))) (Primitives.Instruction.draw (.lit 2) (agent
              := Primitives.NounPhrase.you)) Primitives.UsageLimit.oncePerTurn ] } }

def rootGreevil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Root Greevil", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ activated
            (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .green], Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisCreature
                (agent := Primitives.NounPhrase.you))])
            (destroy (allOf (Primitives.Predicate.and [enchantment, Primitives.Predicate.ofYourChoice .color none]))) ],
      power := stat 2, toughness := stat 3 } }

def riptideChronologist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riptide Chronologist", cost := some [generic 3, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .blue], Primitives.Cost.perform (sacrifice thisCreature (agent :=
            Primitives.NounPhrase.you))])
            (Primitives.Instruction.setStatus .untapped (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.ofYourChoice (.subtype .creature) none]))) ],
      power := stat 1, toughness := stat 3 } }

def urzasIncubator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Incubator", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact (.subtype .creature)),
          Primitives.Ability.static (spellsCost (Primitives.Predicate.and [creature, spell, ofChosen (.subtype .creature)]) (Primitives.CostShift.less (.lit 2) none)) ] } }

def etchingsOfTheChosen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Etchings of the Chosen", cost := some [generic 1, pip .white, pip .black],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, ofChosen (.subtype .creature)]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          activated
            (Primitives.Cost.compound
              [ Primitives.Cost.mana [generic 1],
                Primitives.Cost.perform (sacrifice (a (Primitives.Predicate.and [creature, ofChosen (.subtype .creature)])) (agent :=
                    Primitives.NounPhrase.you)) ])
            (gain (target creatureYouControl) (keyword "Indestructible") (some untilEndOfTurn)) ] }
                }

/-- Volrath's Laboratory -/
def volrathsLaboratory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Volrath's Laboratory", cost := some [generic 5], types := [.artifact],
      text :=
        [ Primitives.Ability.static volrathsLaboratoryChoice,
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 5], Primitives.Cost.tapSymbol])
            (create (.lit 1)
              { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
                qualities := [Primitives.TokenQuality.withQuality (ofChosen .color), Primitives.TokenQuality.withQuality (ofChosen (.subtype .creature))] }) ] } }

/-- Ersatz Gnomes -/
def ersatzGnomes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ersatz Gnomes", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Gnome"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.establish (Primitives.StaticSpec.qualityChange (target spell) .sets (Primitives.QualityPayload.colored (.some []))) none),
          activated Primitives.Cost.tapSymbol (becomeColor (target permanent) (.some []) (some untilEndOfTurn)) ],
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
        [ keyword "Flying",
          activated (Primitives.Cost.mana [pip .blue])
            (Primitives.Instruction.establish (Primitives.StaticSpec.qualityChange (target permanent) .adds (Primitives.QualityPayload.colored (.some [.blue])))
              (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Arcum's Weathervane -/
def arcumsWeathervane : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcum's Weathervane", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.qualityChange (target (Primitives.Predicate.and [land, Primitives.Predicate.hasSupertype .snow])) .loses
                (Primitives.QualityPayload.bundle { characteristics := { supertypes := [.snow] } } none))
              none),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.qualityChange (target (Primitives.Predicate.and [land, Primitives.Predicate.hasSupertype .basic, Primitives.Predicate.not (Primitives.Predicate.hasSupertype
                  .snow)])) .adds
                (Primitives.QualityPayload.bundle { characteristics := { supertypes := [.snow] } } none))
              none) ] } }

def candlesOfLeng : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Candles of Leng", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 4], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequentially
              [ revealCards (topSlice (.lit 1)),
                Primitives.Instruction.doIf (Primitives.Condition.matches it (Primitives.Predicate.named (Primitives.NameSource.sameAs (a (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you))))))
                  (move it graveyard) (some (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))) ]) ] } }

def sphinxOfTheChimes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphinx of the Chimes", cost := some [generic 4, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          activated
            (Primitives.Cost.perform (discard
              (withTheSameName (counted (exactly 2) (Primitives.Predicate.and [Primitives.Predicate.not land, Primitives.Predicate.inZone hand]))) (agent :=
                  Primitives.NounPhrase.you)))
            (Primitives.Instruction.draw (.lit 4) (agent := Primitives.NounPhrase.you)) ],
      power := stat 5, toughness := stat 6 } }

def endlessAtlas : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Endless Atlas", cost := some [generic 2], types := [.artifact],
      text :=
        [ activatedOnlyIf (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol]) (Primitives.Instruction.draw (.lit 1) (agent :=
            Primitives.NounPhrase.you))
            (Primitives.Condition.exists_ (withTheSameName (counted (atLeast 3) (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))) ] } }

/-- Amoeboid Changeling -/
def amoeboidChangelingTypeAbilities : List Ability :=
  [ activated Primitives.Cost.tapSymbol
      (Primitives.Instruction.establish (Primitives.StaticSpec.qualityChange (target creature) .adds (Primitives.QualityPayload.everyTypeOf .creature)) (some
          untilEndOfTurn)),
    activated Primitives.Cost.tapSymbol
      (Primitives.Instruction.establish (Primitives.StaticSpec.qualityChange (target creature) .loses (Primitives.QualityPayload.everyTypeOf .creature)) (some
          untilEndOfTurn)) ]
theorem okAmoeboidChangelingTypeAbilities :
    Ability.checkText [] amoeboidChangelingTypeAbilities = [] := by decide

/-- Fluctuator -/
def fluctuator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fluctuator", cost := some [generic 2], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead (.keyword "Cycling"), Primitives.Predicate.activatedBy Primitives.NounPhrase.you]))
            (Primitives.CostShift.less (.lit 2) none)) ] } }

/-- Boom Scholar -/
def boomScholarExhaustDiscount : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.costShift
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead (.keyword "Exhaust"),
                   Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.otherThan thisCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ]))
    (Primitives.CostShift.less (.lit 2) none))
theorem okBoomScholarExhaustDiscount : Ability.check [] boomScholarExhaustDiscount = [] := by decide
/-- Hulk, Gamma Goliath -/
def hulkPowerUpDiscount : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.costShift
    (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead (.keyword "PowerUp"),
                   Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan thisCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ]))
    (Primitives.CostShift.less (.lit 3) none))
theorem okHulkPowerUpDiscount : Ability.check [] hulkPowerUpDiscount = [] := by decide

/-- Kopala, Warden of Waves -/
def kopalaWardenOfWaves : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kopala, Warden of Waves", cost := some [generic 1, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [ spell, castBy (Primitives.NounPhrase.playerGroup .yourOpponents),
                           Primitives.Predicate.targets (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasSubtype (creatureType "Merfolk"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
                             .someTarget ]))
            (Primitives.CostShift.more (.lit 2))),
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.activatedBy (Primitives.NounPhrase.playerGroup .yourOpponents),
                           Primitives.Predicate.targets (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasSubtype (creatureType "Merfolk"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
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
              [ spellsCost (Primitives.Predicate.and [spell, castBy (Primitives.NounPhrase.playerGroup .yourOpponents)]) (Primitives.CostShift.more (.lit 1)),
                Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.activatedBy (Primitives.NounPhrase.playerGroup
                    .yourOpponents),
                                      Primitives.Predicate.not Primitives.Predicate.isManaAbility ]))
                  (Primitives.CostShift.more (.lit 1)) ])),
          keywordNumber "Afterlife" (.lit 1) ],
      power := stat 2, toughness := stat 1 } }

/-- Vexing Shusher -/
def vexingShusher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vexing Shusher", cost := some [hybridPip .red .green, hybridPip .red .green],
      types := [.creature], subtypes := [creatureType "Goblin", creatureType "Shaman"],
      text :=
        [ Primitives.Ability.static (objectCant (.action "Counter") Primitives.NounPhrase.this),
          activated (Primitives.Cost.mana [hybridPip .red .green])
            (Primitives.Instruction.establish (objectCant (.action "Counter") (target spell)) none) ],
      power := stat 2, toughness := stat 2 } }

def trainingGrounds : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Training Grounds", cost := some [pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf (allOf
            creatureYouControl)]))
            (Primitives.CostShift.less (.lit 2) (some (.lit 1)))) ] } }

/-- Power Artifact -/
def powerArtifact : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Power Artifact", cost := some [pip .blue, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" artifact,
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type .artifact))]))
            (Primitives.CostShift.less (.lit 2) (some (.lit 1)))) ] } }

/-- Fervent Champion -/
def ferventChampionEquipDiscount : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.costShift
    (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead (.keyword "Equip"), Primitives.Predicate.activatedBy Primitives.NounPhrase.you, Primitives.Predicate.targets thisCreature .someTarget]))
    (Primitives.CostShift.less (.lit 3) none))
theorem okFerventChampionEquipDiscount : Ability.check [] ferventChampionEquipDiscount = [] := by decide

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
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [spell, Primitives.Predicate.colorIs .white]) (Primitives.CostShift.more (.lit 3))),
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [enchantment, Primitives.Predicate.colorIs .white]))]))
            (Primitives.CostShift.more (.lit 3))) ] } }

def bureauHeadmaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bureau Headmaster", cost := some [pip .red, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [spell, Primitives.Predicate.hasSubtype (artifactType "Equipment"), castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 1) none)),
          Primitives.Ability.static (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead (.keyword "Equip"), Primitives.Predicate.activatedBy Primitives.NounPhrase.you]))
              (Primitives.CostShift.less (.lit 1) none)) ],
      power := stat 2, toughness := stat 2 } }

def oppressiveRaysLine : StaticSpec :=
  Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type
      .creature))]))
    (Primitives.CostShift.more (.lit 3))
theorem okOppressiveRaysLine : StaticSpec.check [] oppressiveRaysLine = [] := by decide

def eidolonOfObstruction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eidolon of Obstruction", cost := some [generic 1, pip .white],
      types := [.enchantment, .creature], subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "FirstStrike",
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .loyalty,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup .yourOpponents)])) ]))
            (Primitives.CostShift.more (.lit 1))) ],
      power := stat 2, toughness := stat 1 } }

def forceOfWill : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Force of Will", cost := some [generic 3, pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.compound
              [ payLife Primitives.NounPhrase.you 1,
                Primitives.Cost.perform (exile (a (Primitives.Predicate.and [Primitives.Predicate.colorIs .blue, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])) (agent := some
                    Primitives.NounPhrase.you)) ]))),
          Primitives.Ability.spell none (Primitives.Instruction.counterSpell (target spell)) ] } }

def demonOfDeathsGate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Demon of Death's Gate", cost := some [generic 6, pip .black, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Demon"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.compound
              [ payLife Primitives.NounPhrase.you 6,
                Primitives.Cost.perform (sacrifice (counted (exactly 3) (Primitives.Predicate.and [creature, Primitives.Predicate.colorIs .black])) (agent
                    := Primitives.NounPhrase.you)) ]))),
          keyword "Flying", keyword "Trample" ],
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
    (Primitives.Instruction.establish (objectCant (.action "Regenerate") (target creature)) (some Primitives.Duration.thisTurn))
theorem okHurrJackalAbility : Ability.check [] hurrJackalAbility = [] := by decide

def wickedAkuba : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wicked Akuba", cost := some [pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ activated (Primitives.Cost.mana [pip .black])
            (loseLife
              (.lit 1) (agent := (target (Primitives.Predicate.and [Primitives.Predicate.anyPlayer,
                happenedTo (Primitives.GameEvent.dealsDamage .any thisCreature (some (relative
                .player))) .thisTurn])))) ],
      power := stat 2, toughness := stat 2 } }

def idolOfOblivion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Idol of Oblivion", cost := some [generic 2], types := [.artifact],
      text :=
        [ activatedOnlyIf Primitives.Cost.tapSymbol (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
            (happened (Primitives.GameEvent.tokensCreated (a Primitives.Predicate.isToken) false
              (some (relative .player)) none) Primitives.NounPhrase.you .thisTurn),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 8], Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisArtifact
              (agent := Primitives.NounPhrase.you))])
            (create (.lit 1) (creatureToken 10 10 [] [creatureType "Eldrazi"])) ] } }

def patricianGeist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Patrician Geist", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit", creatureType "Knight"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Spirit"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          Primitives.Ability.static (spellsCost (Primitives.Predicate.and [spell, castBy Primitives.NounPhrase.you, Primitives.Predicate.castFrom (graveyardOf Primitives.NounPhrase.you)]) (Primitives.CostShift.less (.lit 1) none)) ],
      power := stat 2, toughness := stat 2 } }

def shatteredEgo : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shattered Ego", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.down (.lit 3)) (Primitives.Delta.down (.lit 0))),
          activated (Primitives.Cost.mana [generic 3, pip .blue, pip .blue])
            (move (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (nthFromTop (.nth 3))) ] } }

def shuFarmer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shu Farmer", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ activatedOnlyDuring Primitives.Cost.tapSymbol (gainLife (.lit 1) (agent := Primitives.NounPhrase.you)) (Primitives.Timing.beforePart
            .declareAttackers (some Primitives.NounPhrase.you)) ],
      power := stat 1, toughness := stat 1 } }

def elspethsTalent : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elspeth's Talent", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" (Primitives.Predicate.hasType .planeswalker),
          Primitives.Ability.static elspethsTalentGrant,
          whenever (Primitives.GameEvent.activates Primitives.NounPhrase.you loyaltyAbilityOfEnchanted)
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.conjunction none
                [ Primitives.StaticSpec.modification (allOf creatureYouControl) .power (Primitives.Delta.up (.lit 2)),
                  Primitives.StaticSpec.modification (itsOther (allOf creatureYouControl) (Primitives.Delta.up (.lit 2))) .toughness (Primitives.Delta.up
                      (.lit 2)),
                  Primitives.StaticSpec.abilityGrant them (keyword "Vigilance") ])
              (some untilEndOfTurn)) ] } }

/-- Void Maw -/
def voidMawPutCost : Ability :=
  activated (Primitives.Cost.perform (put (a (Primitives.Predicate.exiledWith thisCreature)) graveyard (agent := Primitives.NounPhrase.you)))
    (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn))
theorem okVoidMawPutCost : Ability.check [] voidMawPutCost = [] := by decide
/-- Ghor-Clan Rampager -/
def ghorClanRampager : Ability :=
  abilityWord "bloodrush"
    (activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .red, pip .green], Primitives.Cost.perform (discard Primitives.NounPhrase.this (agent := Primitives.NounPhrase.you))])
      (establishFor (target (Primitives.Predicate.and [creature, attacking]))
        [ Primitives.StaticSpec.modification (ownSubject (target (Primitives.Predicate.and [creature, attacking]))) .power (Primitives.Delta.up (.lit 4)),
          Primitives.StaticSpec.modification (ownSubject (target (Primitives.Predicate.and [creature, attacking]))) .toughness (Primitives.Delta.up (.lit
              4)),
          Primitives.StaticSpec.abilityGrant (ownSubject (target (Primitives.Predicate.and [creature, attacking]))) (keyword "Trample") ]
        (some untilEndOfTurn)))
theorem okGhorClanRampager : Ability.check [] ghorClanRampager = [] := by decide
/-- Tymora's Invoker -/
def tymorasInvoker : Ability :=
  flavorWord "Sleight of Hand" (activated (Primitives.Cost.mana [generic 8]) (Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you)))
theorem okTymorasInvoker : Ability.check [] tymorasInvoker = [] := by decide
/-- Skyblade's Boon -/
def skybladesBoonReturn : Ability :=
  activatedOnlyIf (Primitives.Cost.mana [generic 2, pip .white]) (move Primitives.NounPhrase.this hand)
    (Primitives.Condition.or [Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.inZone battlefield), Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you))])
theorem okSkybladesBoonReturn : Ability.check [] skybladesBoonReturn = [] := by decide
/-- Bamboozling Beeble -/
def bamboozlingBeebleIgnore : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.tapSymbol])
    (replaceNextEvent (Primitives.GameEvent.rollsDice (target Primitives.Predicate.anyPlayer) .many none Primitives.RollWatch.anyResult)
      (Primitives.Instruction.sequentially
        [ Primitives.Instruction.rollDice (plus Primitives.Amount.thatMuch (.lit 1)) .thoseDice (agent := they),
          Primitives.Instruction.ignoreOutcomes (Primitives.IgnoredOutcomes.chosen (some Primitives.NounPhrase.you) (.lit 1)) ])
      (some Primitives.Duration.thisTurn))
theorem okBamboozlingBeebleIgnore : Ability.check [] bamboozlingBeebleIgnore = [] := by decide
/-- General Tazri's pump -/
def generalTazriPump : Ability :=
  activated (Primitives.Cost.mana [pip .white, pip .blue, pip .black, pip .red, pip .green])
    (Primitives.Instruction.sequentially
      [ get (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasSubtype (creatureType "Ally"), Primitives.Predicate.hasPossessor .controller
          Primitives.NounPhrase.you]))
          (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (Primitives.Amount.letter .x)) (some untilEndOfTurn),
        Primitives.Instruction.define .x (Primitives.Amount.distinctCount .color (those (.type .creature))) ])
theorem okGeneralTazriPump : Ability.check [] generalTazriPump = [] := by decide
/-- Diplomatic Escort -/
def diplomaticEscortLine : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .blue], Primitives.Cost.tapSymbol, Primitives.Cost.perform (discard (a (Primitives.Predicate.inZone hand)) (agent
      := Primitives.NounPhrase.you))])
    (Primitives.Instruction.counterSpell
      (target (Primitives.Predicate.and [Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack], Primitives.Predicate.targets (a creature) .someTarget])))
theorem okDiplomaticEscortLine : Ability.check [] diplomaticEscortLine = [] := by decide

/-- Vorel of the Hull Clade -/
def vorelOfTheHullClade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vorel of the Hull Clade", cost := some [generic 1, pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Merfolk"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .green, pip .blue], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.doubleCounters (target (Primitives.Predicate.or [artifact, creature, land]))) ],
      power := stat 1, toughness := stat 4 } }

/-- Prosperity -/
theorem prosperityCostLetters : costLetters (some [.variable, pip .blue]) = [letterB .x] := rfl
def prosperityTextLetter : Amount := Primitives.Amount.letter .x
theorem prosperityTextReadsCostLetter :
    Amount.introduced (costLetters (some [.variable, pip .blue])) prosperityTextLetter = [] := by
  decide
theorem noVariableSymbolNoLetter : costLetters (some [generic 1, pip .blue]) = [] := by decide
theorem noCostNoLetter : costLetters none = [] := by decide

/-- Sugar Coat -/
def sugarCoat : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sugar Coat", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keyword "Flash",
          keywordSubject "Enchant" (Primitives.Predicate.or [creature, Primitives.Predicate.hasSubtype (artifactType "Food")]),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted .permanent) .sets
                (Primitives.QualityPayload.bundle
                  { characteristics :=
                    { types := [.artifact], subtypes := [artifactType "Food"],
                      text :=
                        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice
                            thisArtifact (agent := Primitives.NounPhrase.you))])
                            (gainLife (.lit 3) (agent := Primitives.NounPhrase.you)) ] } }
                  none),
              Primitives.StaticSpec.allAbilityLoss it none ]) ] } }

/-- Doc Aurlock, Grizzled Genius -/
def docAurlockCost : StaticSpec :=
  spellsCost (Primitives.Predicate.and [spell, castBy Primitives.NounPhrase.you, Primitives.Predicate.or [Primitives.Predicate.castFrom (graveyardOf Primitives.NounPhrase.you), Primitives.Predicate.castFrom exileZone]])
    (Primitives.CostShift.less (.lit 2) none)
theorem okDocAurlockCost : StaticSpec.check [] docAurlockCost = [] := by decide
/-- Obelisk of Undoing -/
def obeliskOfUndoing : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 6], Primitives.Cost.tapSymbol])
    (returnTo (target (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) hand [])
theorem okObeliskOfUndoing : Ability.check [] obeliskOfUndoing = [] := by decide
/-- Flickering Ward -/
def flickeringWardBounce : Ability := activated (Primitives.Cost.mana [pip .white]) (returnTo thisAura hand [])
theorem okFlickeringWardBounce : Ability.check [] flickeringWardBounce = [] := by decide
/-- Soul Conduit -/
def soulConduitExchange : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 6], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.exchange (Primitives.Exchanged.lifeTotals (Primitives.NounPhrase.described (Primitives.DetPhrase.target (exactly 2)) Primitives.Predicate.anyPlayer)))
theorem okSoulConduitExchange : Ability.check [] soulConduitExchange = [] := by decide

/-- Death-Mask Duplicant -/
def deathMaskDuplicant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death-Mask Duplicant", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ abilityWord "imprint"
            (activated (Primitives.Cost.mana [generic 1]) (exile (target (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])))),
          Primitives.Ability.alsoForKeywords
            (Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.abilityGrant thisCreature (keyword "Flying"))
              (exists_ (Primitives.Predicate.and [Primitives.Predicate.exiledWith thisCreature, Primitives.Predicate.hasKeyword (.the "Flying")]))))
            [ .the "Fear", .the "FirstStrike", .the "DoubleStrike", .the "Haste", landwalkAbilities,
              protectionAbilities, .the "Trample" ] ],
      power := stat 5, toughness := stat 5 } }

/-- Darksteel Garrison -/
def darksteelGarrison : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darksteel Garrison", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Fortification"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .fortified (.type .land)) (keyword "Indestructible")),
          whenever (Primitives.GameEvent.tappedForMana none (Primitives.NounPhrase.attachHost .fortified (.type .land)) none)
            (get (target creature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)),
          keywordCosting "Fortify" (Primitives.Cost.mana [generic 3]) ] } }

/-- Embercleave -/
def embercleave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Embercleave", cost := some [generic 4, pip .red, pip .red], supertypes := [.legendary],
      types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ keyword "Flash",
          Primitives.Ability.static (Primitives.StaticSpec.costShift Primitives.NounPhrase.this
            (Primitives.CostShift.less (forEach 1 (Primitives.Predicate.and [creature, attacking, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) none)),
          when (Primitives.GameEvent.enters thisEquipment none) (Primitives.Instruction.attachTo it (target creatureYouControl)),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction (some (Primitives.NounPhrase.attachHost .equipped (.type .creature)))
            [ Primitives.StaticSpec.modification (ownSubject (Primitives.NounPhrase.attachHost .equipped (.type .creature))) .power (Primitives.Delta.up (.lit
                1)),
              Primitives.StaticSpec.modification (ownSubject (Primitives.NounPhrase.attachHost .equipped (.type .creature))) .toughness (Primitives.Delta.up
                  (.lit 1)),
              Primitives.StaticSpec.abilityGrant (ownSubject (Primitives.NounPhrase.attachHost .equipped (.type .creature))) (keyword
                  "DoubleStrike"),
              Primitives.StaticSpec.abilityGrant (ownSubject (Primitives.NounPhrase.attachHost .equipped (.type .creature))) (keyword
                  "Trample") ]),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 3]) ] } }

/-- Tattoo Ward -/
def tattooWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tattoo Ward", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.conjunction (some (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
              [ Primitives.StaticSpec.modification (ownSubject (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) .power (Primitives.Delta.up
                  (.lit 1)),
                Primitives.StaticSpec.modification (ownSubject (Primitives.NounPhrase.attachHost .enchanted (.type .creature))) .toughness
                    (Primitives.Delta.up (.lit 1)),
                Primitives.StaticSpec.abilityGrant (ownSubject (Primitives.NounPhrase.attachHost .enchanted (.type .creature)))
                  (keywordQuality "Protection" enchantment) ])
            thisAura),
          activated (Primitives.Cost.perform (sacrifice thisAura (agent := Primitives.NounPhrase.you))) (destroy (target enchantment)) ]
              } }

/-- Floating Shield -/
def floatingShield : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Floating Shield", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (entersChoosing thisAura .color),
          Primitives.Ability.static (Primitives.StaticSpec.retention
            (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (keywordQuality "Protection"
                (ofChosen .color)))
            thisAura),
          activated (Primitives.Cost.perform (sacrifice thisAura (agent := Primitives.NounPhrase.you)))
            (gain (target creature) (keywordQuality "Protection" (ofChosen .color)) (some
                untilEndOfTurn)) ] } }

/-- Ghostfire Blade -/
def ghostfireBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostfire Blade", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2))),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 3]),
          Primitives.Ability.static (Primitives.StaticSpec.costShift
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead (.keyword "Equip"), Primitives.Predicate.abilityOf Primitives.NounPhrase.this,
                           Primitives.Predicate.targets (a (Primitives.Predicate.and [creature, colorless])) .someTarget ]))
            (Primitives.CostShift.less (.lit 2) none)) ] } }

/-- Academy Journeymage -/
def academyJourneymage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Academy Journeymage", cost := some [generic 4, pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (onlyIfSo (Primitives.StaticSpec.costShift Primitives.NounPhrase.this (Primitives.CostShift.less (.lit 1) none))
            (exists_ (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Wizard"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
          when (Primitives.GameEvent.enters thisCreature none)
            (move (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])) hand) ],
      power := stat 3, toughness := stat 2 } }

/-- Alabaster Leech -/
def alabasterLeech : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Alabaster Leech", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Leech"],
      text := [Primitives.Ability.static (spellsCost (Primitives.Predicate.and [spell, Primitives.Predicate.colorIs .white, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.run [pip .white] true false))],
      power := stat 1, toughness := stat 3 } }

/-- Edgewalker -/
def edgewalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Edgewalker", cost := some [generic 1, pip .white, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Cleric"), spell, castBy Primitives.NounPhrase.you])
            (Primitives.CostShift.run [pip .white, pip .black] false true)) ],
      power := stat 2, toughness := stat 2 } }

/-- Cavern-Hoard Dragon -/
def cavernHoardDragonRider : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.costShift Primitives.NounPhrase.this (Primitives.CostShift.less (Primitives.Amount.letter .x) none),
      Primitives.StaticSpec.letterDefinition .x
        (Primitives.Amount.aggregateOver .max Primitives.Predicate.opponent (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller they]))) ])
theorem okCavernHoardDragonRider : Ability.check [] cavernHoardDragonRider = [] := by decide
/-- Shadowspear -/
def shadowspearStrip : Ability :=
  activated (Primitives.Cost.mana [generic 1])
    (Primitives.Instruction.establish
      (Primitives.StaticSpec.abilityLoss (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup
          .yourOpponents)]))
        [Primitives.AbilityLost.written (keyword "Hexproof"), Primitives.AbilityLost.written (keyword "Indestructible")])
      (some untilEndOfTurn))
theorem okShadowspearStrip : Ability.check [] shadowspearStrip = [] := by decide
/-- Shay Cormac -/
def shayCormacStrip : Ability :=
  activated (Primitives.Cost.mana [generic 1])
    (Primitives.Instruction.establish
      (Primitives.StaticSpec.abilityLoss (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup
          .yourOpponents)]))
        [ Primitives.AbilityLost.written (keyword "Hexproof"), Primitives.AbilityLost.written (keyword "Indestructible"), Primitives.AbilityLost.term protectionAbilities,
          Primitives.AbilityLost.written (keyword "Shroud"), Primitives.AbilityLost.term wardAbilities ])
      (some untilEndOfTurn))
theorem okShayCormacStrip : Ability.check [] shayCormacStrip = [] := by decide

/-- Shelkin Brownie -/
def shelkinBrownie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shelkin Brownie", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Ouphe"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol])
            (Primitives.Instruction.establish (Primitives.StaticSpec.abilityLoss (target creature) [Primitives.AbilityLost.term bandsWithOtherAbilities])
              (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Ahn-Crop Invader -/
def ahnCropInvader : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ahn-Crop Invader", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Zombie", creatureType "Minotaur", creatureType "Warrior"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you) (Primitives.StaticSpec.abilityGrant thisCreature (keyword
            "FirstStrike"))),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.perform (sacrifice (a (Primitives.Predicate.and [creature,
              Primitives.Predicate.otherThan Primitives.NounPhrase.this])) (agent := Primitives.NounPhrase.you))])
            (get thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 2 } }

/-- Nesting Dragon -/
def nestingDragonInnerToken : Ability :=
  activated (Primitives.Cost.mana [pip .red])
    (Primitives.Instruction.establish
      (Primitives.StaticSpec.conjunction none
        [ Primitives.StaticSpec.modification (Primitives.NounPhrase.asMarker .token Primitives.NounPhrase.this) .power (Primitives.Delta.up (.lit 1)),
          Primitives.StaticSpec.modification (Primitives.NounPhrase.asMarker .token Primitives.NounPhrase.this) .toughness (Primitives.Delta.up (.lit 0)) ])
      (some untilEndOfTurn))
theorem okNestingDragonInnerToken : Ability.check [] nestingDragonInnerToken = [] := by decide

/-- Leonin Bola -/
def leoninBola : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leonin Bola", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (Primitives.NounPhrase.attachHost .equipped (.type .creature))
            (activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (Primitives.Instruction.unattach (Primitives.NounPhrase.theGrantor .permanent))])
              (Primitives.Instruction.setStatus .tapped (target creature)))),
          keywordCosting "Equip" (Primitives.Cost.mana [generic 1]) ] } }

/-- Alluring Suitor // Deadly Dancer -/
def deadlyDancerPump : Ability :=
  activated (Primitives.Cost.mana [pip .red, pip .red])
    (Primitives.Instruction.establish
      (Primitives.StaticSpec.conjunction none
        [ Primitives.StaticSpec.modification (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.both thisCreature (target (Primitives.Predicate.and [creature, Primitives.Predicate.otherThan Primitives.NounPhrase.this]))))
            .power (Primitives.Delta.up (.lit 1)),
          Primitives.StaticSpec.modification (Primitives.NounPhrase.eachOf (Primitives.NounPhrase.both thisCreature it)) .toughness (Primitives.Delta.up (.lit 0)) ])
      (some untilEndOfTurn))
theorem okDeadlyDancerPump : Ability.check [] deadlyDancerPump = [] := by decide

/-- Unesh, Criosphinx Sovereign -/
def uneshCriosphinxSovereign : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unesh, Criosphinx Sovereign", cost := some [generic 4, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (spellsCost (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Sphinx"), spell, castBy Primitives.NounPhrase.you]) (Primitives.CostShift.less (.lit 2) none)),
          whenever
            (Primitives.GameEvent.enters (Primitives.NounPhrase.eitherOf thisCreature
              (a (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Sphinx"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature])))
              none)
            (Primitives.Instruction.sequentially
              [ revealCards (topSlice (.lit 4)),
                Primitives.Instruction.separateIntoPiles them 2 [] (agent := anOpponent),
                move onePile hand,
                move (theOther .pile) graveyard ]) ],
      power := stat 4, toughness := stat 4 } }

def opt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Opt", cost := some [pip .blue], types := [.instant],
      text := [Primitives.Ability.spell none (Primitives.Instruction.sequentially [scry (.lit 1) (agent := Primitives.NounPhrase.you), Primitives.Instruction.draw (.lit 1) (agent :=
          Primitives.NounPhrase.you)])] } }

def serumVisions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Serum Visions", cost := some [pip .blue], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Instruction.sequentially [Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you), scry (.lit 2) (agent :=
          Primitives.NounPhrase.you)])] } }

def consider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consider", cost := some [pip .blue], types := [.instant],
      text := [Primitives.Ability.spell none (Primitives.Instruction.sequentially [surveil (.lit 1) (agent := Primitives.NounPhrase.you), Primitives.Instruction.draw (.lit 1) (agent :=
          Primitives.NounPhrase.you)])] } }

def wordsOfWisdom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Words of Wisdom", cost := some [generic 1, pip .blue], types := [.instant],
      text := [Primitives.Ability.spell none (Primitives.Instruction.sequentially [Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you), Primitives.Instruction.draw (.lit 1) (agent :=
          (each otherPlayer))])] } }

def deathWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death Ward", cost := some [pip .white], types := [.instant],
      text := [Primitives.Ability.spell none (regenerate (target creature))] } }

def bareTextLetter : Amount := Primitives.Amount.letter .x
theorem textAloneOnceMintedItsOwnLetter : Amount.introduced [] bareTextLetter = [letterB .x] := rfl

end Semantics.Cards
