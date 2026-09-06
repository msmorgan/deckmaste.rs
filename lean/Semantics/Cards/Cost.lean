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
  activated (.compound [.mana [pip .white], .tapSymbol]) (tap (target creature))
theorem okMasterDecoy : Ability.check [] masterDecoy = [] := by decide
def cycling : Ability :=
  activated (.compound [.mana [generic 2], .perform (discard .this (agent := .you))]) (.draw (.lit
      1) (agent := .you))
theorem okCycling : Ability.check [] cycling = [] := by decide
def merrowGrimeblotter : Ability :=
  activated (.compound [.mana [generic 1, hybridPip .blue .black], .untapSymbol])
    (get (target creature) (.down (.lit 2)) (.down (.lit 0)) (some untilEndOfTurn))
theorem okMerrowGrimeblotter : Ability.check [] merrowGrimeblotter = [] := by decide
def phyrexianSnowcrusher : Ability :=
  activated (.mana [generic 1, .snow])
    (get thisCreature (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn))
theorem okPhyrexianSnowcrusher : Ability.check [] phyrexianSnowcrusher = [] := by decide
def havocSower : Ability :=
  activated (.mana [generic 1, colorlessPip])
    (get thisCreature (.up (.lit 2)) (.up (.lit 1)) (some untilEndOfTurn))
theorem okHavocSower : Ability.check [] havocSower = [] := by decide
/-- Erebos, God of the Dead -/
def erebos : Ability :=
  activated (.compound [.mana [generic 1, pip .black], payLife .you 2]) (.draw (.lit 1) (agent :=
      .you))
theorem okErebos : Ability.check [] erebos = [] := by decide
def baskingRootwalla : Ability :=
  activatedOnlyOnce (.mana [generic 1, pip .green])
    (get thisCreature (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn)) .oncePerTurn
theorem okBaskingRootwalla : Ability.check [] baskingRootwalla = [] := by decide
def securityDetail : Ability :=
  activatedOnlyOnceIf (.mana [pip .white, pip .white])
    (create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"])) .oncePerTurn
    (.not (exists_ (.and [creature, .hasPossessor .controller .you])))
theorem okSecurityDetail : Ability.check [] securityDetail = [] := by decide
def aphettoAlchemist : Ability :=
  activated .tapSymbol (.setStatus .untapped (target (.or [artifact, creature])))
theorem okAphettoAlchemist : Ability.check [] aphettoAlchemist = [] := by decide

def charRumbler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Char-Rumbler", cost := some [generic 2, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ keyword "DoubleStrike",
          activated (.mana [pip .red])
            (get thisCreature (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn)) ],
      power := some (.lit (-1)), toughness := stat 3 } }

/-- Sisters of Stone Death -/
def sistersOfStoneDeathRecall : Ability :=
  activated (.mana [generic 2, pip .black])
    (putOntoBattlefieldUnderYourControl (a (.and [creature, .exiledWith thisCreature])))
theorem okSistersOfStoneDeathRecall : Ability.check [] sistersOfStoneDeathRecall = [] := by decide
/-- Synod Sanctum -/
def synodSanctumReturn : Ability :=
  activated (.compound [.mana [generic 2], .perform (sacrifice thisArtifact (agent := .you))])
    (putOntoBattlefieldUnderYourControl (allOf exiledWithThisArtifact))
theorem okSynodSanctumReturn : Ability.check [] synodSanctumReturn = [] := by decide

def coldStorage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cold Storage", cost := some [generic 4], types := [.artifact],
      text :=
        [ activated (.mana [generic 3]) (exile (target creatureYouControl)),
          activated (.perform (sacrifice thisArtifact (agent := .you)))
            (putOntoBattlefieldUnderYourControl (each (.and [creature, exiledWithThisArtifact]))) ] } }

def barlsCage : Ability :=
  activated (.mana [generic 3]) (.skipUntap (target creature) (.lit 1))
theorem okBarlsCage : Ability.check [] barlsCage = [] := by decide
def vodalianIllusionist : Ability :=
  activated (.compound [.mana [pip .blue, pip .blue], .tapSymbol])
    (.setStatus .phasedOut (target creature))
theorem okVodalianIllusionist : Ability.check [] vodalianIllusionist = [] := by decide
def witchsMist : Ability :=
  activated (.compound [.mana [generic 2, pip .black], .tapSymbol])
    (destroy (target (.and [creature, happenedTo .damageTaken .thisTurn])))
theorem okWitchsMist : Ability.check [] witchsMist = [] := by decide
def goadTargetCreature : Ability :=
  activated (.compound [.mana [generic 3], .tapSymbol])
    (.gainDesignation (target creature) "goaded" .instructed none)
theorem okGoadTargetCreature : Ability.check [] goadTargetCreature = [] := by decide
/-- Krenko, Mob Boss -/
def krenko : Ability :=
  activated .tapSymbol
    (.sequence
      [ .create (.letter .x) (.written (creatureToken 1 1 [.red] [creatureType "Goblin"])) [] (agent
          := .you),
        .define .x (countOf (.and [.hasSubtype (creatureType "Goblin"), .hasPossessor .controller .you])) ])
theorem okKrenko : Ability.check [] krenko = [] := by decide
/-- Dokai, Weaver of Life -/
def dokai : Ability :=
  activated (.compound [.mana [generic 4, pip .green, pip .green], .tapSymbol])
    (.sequence
      [ create (.lit 1) (creatureTokenOf (.letter .x) (.letter .x) [.green] [creatureType "Elemental"]),
        .define .x (countOf (.and [land, .hasPossessor .controller .you])) ])
theorem okDokai : Ability.check [] dokai = [] := by decide

def ghalta : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghalta, Primal Hunger", cost := some [generic 10, pip .green, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Elder", creatureType "Dinosaur"],
      text :=
        [ .static (.conjunction none
            [ .costShift .this (.less (.letter .x) none),
              .letterDefinition .x (aggregate .sum (.stat .power) creatureYouControl) ]),
          keyword "Trample" ],
      power := stat 12, toughness := stat 12 } }

def ancientStoneIdol : Ability :=
  .static (.costShift .this (.less (forEach 1 (.and [creature, attacking])) none))
theorem okAncientStoneIdol : Ability.check [] ancientStoneIdol = [] := by decide

/-- "<p> spells cost {N} more/less to cast" -/
def spellsCost (p : Predicate) (shift : CostShift) : StaticSpec := .costShift (allOf p) shift

def thornOfAmethyst : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thorn of Amethyst", cost := some [generic 2], types := [.artifact],
      text := [.static (spellsCost (.and [.not creature, spell]) (.more (.lit 1)))] } }

def ferozsBan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Feroz's Ban", cost := some [generic 6], types := [.artifact],
      text := [.static (spellsCost (.and [creature, spell]) (.more (.lit 2)))] } }

def urzasFilter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Filter", cost := some [generic 4], types := [.artifact],
      text := [.static (spellsCost (.and [multicolored, spell]) (.less (.lit 2) none))] } }

def emeraldMedallion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Emerald Medallion", cost := some [generic 2], types := [.artifact],
      text := [.static (spellsCost (.and [.colorIs .green, spell, castBy .you]) (.less (.lit 1) none))] } }

/-- Highspire Bell-Ringer -/
def highspireBellRinger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Highspire Bell-Ringer", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Djinn", creatureType "Monk"],
      text :=
        [ keyword "Flying",
          .static (.costShift (the (.and [spell, nthCastBy (.nth 2) .you (.each .turn)])) (.less
              (.lit 1) none)) ],
      power := stat 1, toughness := stat 4 } }

def foundryInspector : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foundry Inspector", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Construct"],
      text := [.static (spellsCost (.and [artifact, spell, castBy .you]) (.less (.lit 1) none))],
      power := stat 3, toughness := stat 2 } }

def daruWarchief : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daru Warchief", cost := some [generic 2, pip .white, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ .static (spellsCost (.and [.hasSubtype (creatureType "Soldier"), spell, castBy .you]) (.less (.lit 1) none)),
          .static (getsPt (allOf (.and [.hasSubtype (creatureType "Soldier"), creature, .hasPossessor .controller .you]))
            (.up (.lit 1)) (.up (.lit 2))) ],
      power := stat 1, toughness := stat 1 } }

def grandArbiter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grand Arbiter Augustin IV", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Advisor"],
      text :=
        [ .static (spellsCost (.and [.colorIs .white, spell, castBy .you]) (.less (.lit 1) none)),
          .static (spellsCost (.and [.colorIs .blue, spell, castBy .you]) (.less (.lit 1) none)),
          .static (spellsCost (.and [spell, castBy (.playerGroup .yourOpponents)]) (.more (.lit 1))) ],
      power := stat 2, toughness := stat 3 } }

def goblinElectromancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Goblin Electromancer", cost := some [pip .blue, pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Wizard"],
      text := [.static (spellsCost (.and [instantOrSorcery, spell, castBy .you]) (.less (.lit 1) none))],
      power := stat 2, toughness := stat 2 } }

def arcaneMelee : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Melee", cost := some [generic 4, pip .blue], types := [.enchantment],
      text := [.static (spellsCost (.and [instantOrSorcery, spell]) (.less (.lit 2) none))] } }

def manaMatrix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Matrix", cost := some [generic 6], types := [.artifact],
      text :=
        [ .static (spellsCost (.and [.or [instant, enchantment], spell, castBy .you]) (.less (.lit 2) none)) ] } }

def auraOfSilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aura of Silence", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ .static (spellsCost (.and [.or [artifact, enchantment], spell, castBy (.playerGroup .yourOpponents)])
            (.more (.lit 2))),
          activated (.perform (sacrifice thisEnchantment (agent := .you)))
            (destroy (target (.or [artifact, enchantment]))) ] } }

def chillerpillar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chillerpillar", cost := some [generic 3, pip .blue], supertypes := [.snow],
      types := [.creature], subtypes := [creatureType "Insect"],
      text :=
        [ activated (.mana [generic 4, .snow, .snow]) (makeMonstrous (.lit 2)),
          .static (onlyWhile (.abilityGrant thisCreature (keyword "Flying"))
            (.matches thisCreature (.hasDesignation "monstrous" none))) ],
      power := stat 3, toughness := stat 3 } }

def nullhideFerox : Ability :=
  activated (.mana [generic 2])
    (.establish (.allAbilityLoss thisCreature none) (some untilEndOfTurn))
theorem okNullhideFerox : Ability.check [] nullhideFerox = [] := by decide

def causticTar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caustic Tar", cost := some [generic 4, pip .black, pip .black], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          .static (.abilityGrant (.attachHost .enchanted (.type .land))
            (activated .tapSymbol (loseLife (.lit 3) (agent := (target .anyPlayer))))) ] } }

/-- Saheeli, Filigree Master -/
def saheelisEmblem : Instruction :=
  .getEmblem
    [ .static (getsPt (allOf (.and [artifact, creature, .hasPossessor .controller .you]))
        (.up (.lit 1)) (.up (.lit 1))),
      .static (spellsCost (.and [artifact, spell, castBy .you]) (.less (.lit 1) none)) ] (agent :=
          .you)
theorem okSaheelisEmblem : Instruction.check [] saheelisEmblem = [] := by decide

def jaceBeleren : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Jace Beleren", cost := some [generic 1, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Jace"],
      text :=
        [ activated (.loyaltySymbol (.up 2)) (.draw (.lit 1) (agent := (each .anyPlayer))),
          activated (.loyaltySymbol (.down 1)) (.draw (.lit 1) (agent := (target .anyPlayer))),
          activated (.loyaltySymbol (.down 10)) (mill (.lit 20) they (agent := (target .anyPlayer)))
              ],
      loyalty := stat 3 } }

def elspethSunsChampion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elspeth, Sun's Champion", cost := some [generic 4, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Elspeth"],
      text :=
        [ activated (.loyaltySymbol (.up 1))
            (create (.lit 3) (creatureToken 1 1 [.white] [creatureType "Soldier"])),
          activated (.loyaltySymbol (.down 3))
            (destroy (allOf (.and [creature, .compare [.stat .power] .atLeast (.lit 4)]))),
          activated (.loyaltySymbol (.down 7))
            (.getEmblem
              [ .static (.conjunction none
                  [ .modification (allOf creatureYouControl) .power (.up (.lit 2)),
                    .modification (itsOther (allOf creatureYouControl) (.up (.lit 2))) .toughness
                        (.up (.lit 2)),
                    .abilityGrant them (keyword "Flying") ]) ] (agent := .you)) ],
      loyalty := stat 4 } }

/-- Elspeth's Talent -/
def elspethsTalentGrant : StaticSpec :=
  .abilityGrant (.attachHost .enchanted (.type .planeswalker))
    (activated (.loyaltySymbol (.up 1))
      (create (.lit 3) (creatureToken 1 1 [.white] [creatureType "Soldier"])))
theorem okElspethsTalentGrant : StaticSpec.check [] elspethsTalentGrant = [] := by decide

def crystalBall : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crystal Ball", cost := some [generic 3], types := [.artifact],
      text := [activated (.compound [.mana [generic 1], .tapSymbol]) (scry (.lit 2) (agent :=
          .you))] } }

/-- Angus Mackenzie -/
def angusMackenzie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Angus Mackenzie", cost := some [pip .green, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ activatedOnlyDuring (.compound [.mana [pip .green, pip .white, pip .blue], .tapSymbol])
            (preventAll .combatOnly .everywhere (some .thisTurn)) (.beforePart .combatDamage none) ],
      power := stat 2, toughness := stat 2 } }

/-- Vampire Hexmage -/
def vampireHexmage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vampire Hexmage", cost := some [pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Vampire", creatureType "Shaman"],
      text :=
        [ keyword "FirstStrike",
          activated (.perform (sacrifice thisCreature (agent := .you))) (removeAllCounters none
              (target permanent)) ],
      power := stat 2, toughness := stat 1 } }

def mizziumTransreliquat : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mizzium Transreliquat", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (.mana [generic 3])
            (.establish (.copyChange thisArtifact (target artifact) []) (some untilEndOfTurn)),
          activated (.mana [generic 1, pip .blue, pip .red])
            (.establish (.copyChange thisArtifact (target artifact) [.thisAbility]) none) ] } }

def tranquilGrove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tranquil Grove", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ activated (.mana [generic 1, pip .green, pip .green])
            (destroy (allOf (.and [enchantment, .otherThan .this]))) ] } }

def aggressiveMining : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggressive Mining", cost := some [generic 3, pip .red], types := [.enchantment],
      text :=
        [ .static (cantDoTo (.action "Play") .you (allOf land)),
          activatedOnlyOnce (.perform (sacrifice (a land) (agent := .you))) (.draw (.lit 2) (agent
              := .you)) .oncePerTurn ] } }

def rootGreevil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Root Greevil", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [creatureType "Beast"],
      text :=
        [ activated
            (.compound [.mana [generic 2, pip .green], .tapSymbol, .perform (sacrifice thisCreature
                (agent := .you))])
            (destroy (allOf (.and [enchantment, .ofYourChoice .color none]))) ],
      power := stat 2, toughness := stat 3 } }

def riptideChronologist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riptide Chronologist", cost := some [generic 3, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (.compound [.mana [pip .blue], .perform (sacrifice thisCreature (agent :=
            .you))])
            (.setStatus .untapped (allOf (.and [creature, .ofYourChoice (.subtype .creature) none]))) ],
      power := stat 1, toughness := stat 3 } }

def urzasIncubator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Incubator", cost := some [generic 3], types := [.artifact],
      text :=
        [ .static (entersChoosing thisArtifact (.subtype .creature)),
          .static (spellsCost (.and [creature, spell, ofChosen (.subtype .creature)]) (.less (.lit 2) none)) ] } }

def etchingsOfTheChosen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Etchings of the Chosen", cost := some [generic 1, pip .white, pip .black],
      types := [.enchantment],
      text :=
        [ .static (entersChoosing thisEnchantment (.subtype .creature)),
          .static (getsPt (allOf (.and [creature, .hasPossessor .controller .you, ofChosen (.subtype .creature)]))
            (.up (.lit 1)) (.up (.lit 1))),
          activated
            (.compound
              [ .mana [generic 1],
                .perform (sacrifice (a (.and [creature, ofChosen (.subtype .creature)])) (agent :=
                    .you)) ])
            (gain (target creatureYouControl) (keyword "Indestructible") (some untilEndOfTurn)) ] }
                }

/-- Volrath's Laboratory -/
def volrathsLaboratory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Volrath's Laboratory", cost := some [generic 5], types := [.artifact],
      text :=
        [ .static volrathsLaboratoryChoice,
          activated (.compound [.mana [generic 5], .tapSymbol])
            (create (.lit 1)
              { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
                qualities := [.withQuality (ofChosen .color), .withQuality (ofChosen (.subtype .creature))] }) ] } }

/-- Ersatz Gnomes -/
def ersatzGnomes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ersatz Gnomes", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [creatureType "Gnome"],
      text :=
        [ activated .tapSymbol
            (.establish (.qualityChange (target spell) .sets (.colored (.some []))) none),
          activated .tapSymbol (becomeColor (target permanent) (.some []) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Scrapbasket -/
def scrapbasket : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scrapbasket", cost := some [generic 4], types := [.artifact, .creature],
      subtypes := [creatureType "Scarecrow"],
      text := [activated (.mana [generic 1]) (becomeColor thisCreature .every (some
          untilEndOfTurn))],
      power := stat 3, toughness := stat 2 } }

/-- Indigo Faerie -/
def indigoFaerie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Indigo Faerie", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Wizard"],
      text :=
        [ keyword "Flying",
          activated (.mana [pip .blue])
            (.establish (.qualityChange (target permanent) .adds (.colored (.some [.blue])))
              (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Arcum's Weathervane -/
def arcumsWeathervane : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcum's Weathervane", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (.compound [.mana [generic 2], .tapSymbol])
            (.establish
              (.qualityChange (target (.and [land, .hasSupertype .snow])) .loses
                (.bundle { characteristics := { supertypes := [.snow] } } none))
              none),
          activated (.compound [.mana [generic 2], .tapSymbol])
            (.establish
              (.qualityChange (target (.and [land, .hasSupertype .basic, .not (.hasSupertype
                  .snow)])) .adds
                (.bundle { characteristics := { supertypes := [.snow] } } none))
              none) ] } }

def candlesOfLeng : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Candles of Leng", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (.compound [.mana [generic 4], .tapSymbol])
            (.sequence
              [ revealCards (topSlice (.lit 1)),
                .doIf (.matches it (.named (.sameAs (a (.inZone (graveyardOf .you))))))
                  (move it graveyard) (some (.draw (.lit 1) (agent := .you))) ]) ] } }

def sphinxOfTheChimes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphinx of the Chimes", cost := some [generic 4, pip .blue, pip .blue], types := [.creature],
      subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          activated
            (.perform (discard
              (withTheSameName (counted (exactly 2) (.and [.not land, .inZone hand]))) (agent :=
                  .you)))
            (.draw (.lit 4) (agent := .you)) ],
      power := stat 5, toughness := stat 6 } }

def endlessAtlas : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Endless Atlas", cost := some [generic 2], types := [.artifact],
      text :=
        [ activatedOnlyIf (.compound [.mana [generic 2], .tapSymbol]) (.draw (.lit 1) (agent :=
            .you))
            (.exists_ (withTheSameName (counted (atLeast 3) (.and [land, .hasPossessor .controller .you])))) ] } }

/-- Amoeboid Changeling -/
def amoeboidChangelingTypeAbilities : List Ability :=
  [ activated .tapSymbol
      (.establish (.qualityChange (target creature) .adds (.everyTypeOf .creature)) (some
          untilEndOfTurn)),
    activated .tapSymbol
      (.establish (.qualityChange (target creature) .loses (.everyTypeOf .creature)) (some
          untilEndOfTurn)) ]
theorem okAmoeboidChangelingTypeAbilities :
    Ability.checkText [] amoeboidChangelingTypeAbilities = [] := by decide

/-- Fluctuator -/
def fluctuator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fluctuator", cost := some [generic 2], types := [.artifact],
      text :=
        [ .static (.costShift (allOf (.and [.abilityHead (.keyword "Cycling"), .activatedBy .you]))
            (.less (.lit 2) none)) ] } }

/-- Boom Scholar -/
def boomScholarExhaustDiscount : Ability :=
  .static (.costShift
    (allOf (.and [ .abilityHead (.keyword "Exhaust"),
                   .abilityOf (allOf (.and [permanent, .otherThan thisCreature, .hasPossessor .controller .you])) ]))
    (.less (.lit 2) none))
theorem okBoomScholarExhaustDiscount : Ability.check [] boomScholarExhaustDiscount = [] := by decide
/-- Hulk, Gamma Goliath -/
def hulkPowerUpDiscount : Ability :=
  .static (.costShift
    (allOf (.and [ .abilityHead (.keyword "PowerUp"),
                   .abilityOf (allOf (.and [creature, .otherThan thisCreature, .hasPossessor .controller .you])) ]))
    (.less (.lit 3) none))
theorem okHulkPowerUpDiscount : Ability.check [] hulkPowerUpDiscount = [] := by decide

/-- Kopala, Warden of Waves -/
def kopalaWardenOfWaves : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kopala, Warden of Waves", cost := some [generic 1, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ .static (.costShift
            (allOf (.and [ spell, castBy (.playerGroup .yourOpponents),
                           .targets (a (.and [creature, .hasSubtype (creatureType "Merfolk"), .hasPossessor .controller .you]))
                             .someTarget ]))
            (.more (.lit 2))),
          .static (.costShift
            (allOf (.and [ .abilityHead .anyActivated, .activatedBy (.playerGroup .yourOpponents),
                           .targets (a (.and [creature, .hasSubtype (creatureType "Merfolk"), .hasPossessor .controller .you]))
                             .someTarget ]))
            (.more (.lit 2))) ],
      power := stat 2, toughness := stat 2 } }

/-- Tithe Taker -/
def titheTaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tithe Taker", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ .static (.partScope .turn (some .you)
            (.conjunction none
              [ spellsCost (.and [spell, castBy (.playerGroup .yourOpponents)]) (.more (.lit 1)),
                .costShift (allOf (.and [ .abilityHead .anyActivated, .activatedBy (.playerGroup
                    .yourOpponents),
                                      .not .isManaAbility ]))
                  (.more (.lit 1)) ])),
          keywordNumber "Afterlife" (.lit 1) ],
      power := stat 2, toughness := stat 1 } }

/-- Vexing Shusher -/
def vexingShusher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vexing Shusher", cost := some [hybridPip .red .green, hybridPip .red .green],
      types := [.creature], subtypes := [creatureType "Goblin", creatureType "Shaman"],
      text :=
        [ .static (objectCant (.action "Counter") .this),
          activated (.mana [hybridPip .red .green])
            (.establish (objectCant (.action "Counter") (target spell)) none) ],
      power := stat 2, toughness := stat 2 } }

def trainingGrounds : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Training Grounds", cost := some [pip .blue], types := [.enchantment],
      text :=
        [ .static (.costShift (allOf (.and [.abilityHead .anyActivated, .abilityOf (allOf
            creatureYouControl)]))
            (.less (.lit 2) (some (.lit 1)))) ] } }

/-- Power Artifact -/
def powerArtifact : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Power Artifact", cost := some [pip .blue, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" artifact,
          .static (.costShift
            (allOf (.and [.abilityHead .anyActivated, .abilityOf (.attachHost .enchanted (.type .artifact))]))
            (.less (.lit 2) (some (.lit 1)))) ] } }

/-- Fervent Champion -/
def ferventChampionEquipDiscount : Ability :=
  .static (.costShift
    (allOf (.and [.abilityHead (.keyword "Equip"), .activatedBy .you, .targets thisCreature .someTarget]))
    (.less (.lit 3) none))
theorem okFerventChampionEquipDiscount : Ability.check [] ferventChampionEquipDiscount = [] := by decide

def suppressionField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Suppression Field", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ .static (.costShift (allOf (.and [.abilityHead .anyActivated, .not .isManaAbility]))
            (.more (.lit 2))) ] } }

def gloom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gloom", cost := some [generic 2, pip .black], types := [.enchantment],
      text :=
        [ .static (spellsCost (.and [spell, .colorIs .white]) (.more (.lit 3))),
          .static (.costShift
            (allOf (.and [.abilityHead .anyActivated, .abilityOf (allOf (.and [enchantment, .colorIs .white]))]))
            (.more (.lit 3))) ] } }

def bureauHeadmaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bureau Headmaster", cost := some [pip .red, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Assassin"],
      text :=
        [ .static (spellsCost (.and [spell, .hasSubtype (artifactType "Equipment"), castBy .you]) (.less (.lit 1) none)),
          .static (.costShift (allOf (.and [.abilityHead (.keyword "Equip"), .activatedBy .you]))
              (.less (.lit 1) none)) ],
      power := stat 2, toughness := stat 2 } }

def oppressiveRaysLine : StaticSpec :=
  .costShift (allOf (.and [.abilityHead .anyActivated, .abilityOf (.attachHost .enchanted (.type
      .creature))]))
    (.more (.lit 3))
theorem okOppressiveRaysLine : StaticSpec.check [] oppressiveRaysLine = [] := by decide

def eidolonOfObstruction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eidolon of Obstruction", cost := some [generic 1, pip .white],
      types := [.enchantment, .creature], subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "FirstStrike",
          .static (.costShift
            (allOf (.and [ .abilityHead .loyalty,
                           .abilityOf (allOf (.and [.hasType .planeswalker, .hasPossessor .controller (.playerGroup .yourOpponents)])) ]))
            (.more (.lit 1))) ],
      power := stat 2, toughness := stat 1 } }

def forceOfWill : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Force of Will", cost := some [generic 3, pip .blue, pip .blue], types := [.instant],
      text :=
        [ .static (.altCost .this
            (some (.compound
              [ payLife .you 1,
                .perform (exile (a (.and [.colorIs .blue, .inZone (handOf .you)])) (agent := some
                    .you)) ]))),
          .spell none (.counterSpell (target spell)) ] } }

def demonOfDeathsGate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Demon of Death's Gate", cost := some [generic 6, pip .black, pip .black, pip .black],
      types := [.creature], subtypes := [creatureType "Demon"],
      text :=
        [ .static (.altCost .this
            (some (.compound
              [ payLife .you 6,
                .perform (sacrifice (counted (exactly 3) (.and [creature, .colorIs .black])) (agent
                    := .you)) ]))),
          keyword "Flying", keyword "Trample" ],
      power := stat 9, toughness := stat 9 } }

def drudgeSkeletons : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drudge Skeletons", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Skeleton"],
      text := [activated (.mana [pip .black]) (.regenerate thisCreature)],
      power := stat 1, toughness := stat 1 } }

def asphodelWanderer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Asphodel Wanderer", cost := some [pip .black], types := [.creature],
      subtypes := [creatureType "Skeleton", creatureType "Soldier"],
      text := [activated (.mana [generic 2, pip .black]) (.regenerate thisCreature)],
      power := stat 1, toughness := stat 1 } }

def hurrJackalAbility : Ability :=
  activated .tapSymbol
    (.establish (objectCant (.action "Regenerate") (target creature)) (some .thisTurn))
theorem okHurrJackalAbility : Ability.check [] hurrJackalAbility = [] := by decide

def wickedAkuba : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wicked Akuba", cost := some [pip .black, pip .black], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ activated (.mana [pip .black])
            (loseLife
              (.lit 1) (agent := (target (.and [.anyPlayer, happenedToInvolving .damageTaken
                  .thisTurn thisCreature])))) ],
      power := stat 2, toughness := stat 2 } }

def idolOfOblivion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Idol of Oblivion", cost := some [generic 2], types := [.artifact],
      text :=
        [ activatedOnlyIf .tapSymbol (.draw (.lit 1) (agent := .you))
            (happenedInvolving .tokenCreation .you .thisTurn (a .isToken)),
          activated (.compound [.mana [generic 8], .tapSymbol, .perform (sacrifice thisArtifact
              (agent := .you))])
            (create (.lit 1) (creatureToken 10 10 [] [creatureType "Eldrazi"])) ] } }

def patricianGeist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Patrician Geist", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit", creatureType "Knight"],
      text :=
        [ keyword "Flying",
          .static (getsPt (allOf (.and [.hasSubtype (creatureType "Spirit"), .hasPossessor .controller .you, .otherThan thisCreature]))
            (.up (.lit 1)) (.up (.lit 1))),
          .static (spellsCost (.and [spell, castBy .you, .castFrom (graveyardOf .you)]) (.less (.lit 1) none)) ],
      power := stat 2, toughness := stat 2 } }

def shatteredEgo : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shattered Ego", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (getsPt (.attachHost .enchanted (.type .creature)) (.down (.lit 3)) (.down (.lit 0))),
          activated (.mana [generic 3, pip .blue, pip .blue])
            (move (.attachHost .enchanted (.type .creature)) (nthFromTop (.nth 3))) ] } }

def shuFarmer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shu Farmer", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ activatedOnlyDuring .tapSymbol (gainLife (.lit 1) (agent := .you)) (.beforePart
            .declareAttackers (some .you)) ],
      power := stat 1, toughness := stat 1 } }

def elspethsTalent : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elspeth's Talent", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" (.hasType .planeswalker),
          .static elspethsTalentGrant,
          whenever (.activates .you loyaltyAbilityOfEnchanted)
            (.establish
              (.conjunction none
                [ .modification (allOf creatureYouControl) .power (.up (.lit 2)),
                  .modification (itsOther (allOf creatureYouControl) (.up (.lit 2))) .toughness (.up
                      (.lit 2)),
                  .abilityGrant them (keyword "Vigilance") ])
              (some untilEndOfTurn)) ] } }

/-- Void Maw -/
def voidMawPutCost : Ability :=
  activated (.perform (put (a (.exiledWith thisCreature)) graveyard (agent := .you)))
    (get thisCreature (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn))
theorem okVoidMawPutCost : Ability.check [] voidMawPutCost = [] := by decide
/-- Ghor-Clan Rampager -/
def ghorClanRampager : Ability :=
  abilityWord "bloodrush"
    (activated (.compound [.mana [pip .red, pip .green], .perform (discard .this (agent := .you))])
      (establishFor (target (.and [creature, attacking]))
        [ .modification (ownSubject (target (.and [creature, attacking]))) .power (.up (.lit 4)),
          .modification (ownSubject (target (.and [creature, attacking]))) .toughness (.up (.lit
              4)),
          .abilityGrant (ownSubject (target (.and [creature, attacking]))) (keyword "Trample") ]
        (some untilEndOfTurn)))
theorem okGhorClanRampager : Ability.check [] ghorClanRampager = [] := by decide
/-- Tymora's Invoker -/
def tymorasInvoker : Ability :=
  flavorWord "Sleight of Hand" (activated (.mana [generic 8]) (.draw (.lit 2) (agent := .you)))
theorem okTymorasInvoker : Ability.check [] tymorasInvoker = [] := by decide
/-- Skyblade's Boon -/
def skybladesBoonReturn : Ability :=
  activatedOnlyIf (.mana [generic 2, pip .white]) (move .this hand)
    (.or [.matches .this (.inZone battlefield), .matches .this (.inZone (graveyardOf .you))])
theorem okSkybladesBoonReturn : Ability.check [] skybladesBoonReturn = [] := by decide
/-- Bamboozling Beeble -/
def bamboozlingBeebleIgnore : Ability :=
  activated (.compound [.mana [generic 1], .tapSymbol])
    (replaceNextEvent (.rollsDice (target .anyPlayer) .many none .anyResult)
      (.sequence
        [ .rollDice (plus .thatMuch (.lit 1)) .thoseDice (agent := they),
          .ignoreOutcomes (.chosen (some .you) (.lit 1)) ])
      (some .thisTurn))
theorem okBamboozlingBeebleIgnore : Ability.check [] bamboozlingBeebleIgnore = [] := by decide
/-- General Tazri's pump -/
def generalTazriPump : Ability :=
  activated (.mana [pip .white, pip .blue, pip .black, pip .red, pip .green])
    (.sequence
      [ get (each (.and [creature, .hasSubtype (creatureType "Ally"), .hasPossessor .controller
          .you]))
          (.up (.letter .x)) (.up (.letter .x)) (some untilEndOfTurn),
        .define .x (.distinctCount .color (those (.type .creature))) ])
theorem okGeneralTazriPump : Ability.check [] generalTazriPump = [] := by decide
/-- Diplomatic Escort -/
def diplomaticEscortLine : Ability :=
  activated (.compound [.mana [pip .blue], .tapSymbol, .perform (discard (a (.inZone hand)) (agent
      := .you))])
    (.counterSpell
      (target (.and [.or [spell, .abilityHead .anyOnStack], .targets (a creature) .someTarget])))
theorem okDiplomaticEscortLine : Ability.check [] diplomaticEscortLine = [] := by decide

/-- Vorel of the Hull Clade -/
def vorelOfTheHullClade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vorel of the Hull Clade", cost := some [generic 1, pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Merfolk"],
      text :=
        [ activated (.compound [.mana [pip .green, pip .blue], .tapSymbol])
            (.doubleCounters (target (.or [artifact, creature, land]))) ],
      power := stat 1, toughness := stat 4 } }

/-- Prosperity -/
theorem prosperityCostLetters : costLetters (some [.variable, pip .blue]) = [letterB .x] := rfl
def prosperityTextLetter : Amount := .letter .x
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
          keywordSubject "Enchant" (.or [creature, .hasSubtype (artifactType "Food")]),
          .static (.conjunction none
            [ .qualityChange (.attachHost .enchanted .permanent) .sets
                (.bundle
                  { characteristics :=
                    { types := [.artifact], subtypes := [artifactType "Food"],
                      text :=
                        [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice
                            thisArtifact (agent := .you))])
                            (gainLife (.lit 3) (agent := .you)) ] } }
                  none),
              .allAbilityLoss it none ]) ] } }

/-- Doc Aurlock, Grizzled Genius -/
def docAurlockCost : StaticSpec :=
  spellsCost (.and [spell, castBy .you, .or [.castFrom (graveyardOf .you), .castFrom exileZone]])
    (.less (.lit 2) none)
theorem okDocAurlockCost : StaticSpec.check [] docAurlockCost = [] := by decide
/-- Obelisk of Undoing -/
def obeliskOfUndoing : Ability :=
  activated (.compound [.mana [generic 6], .tapSymbol])
    (returnTo (target (.and [permanent, .hasPossessor .owner .you, .hasPossessor .controller .you])) hand [])
theorem okObeliskOfUndoing : Ability.check [] obeliskOfUndoing = [] := by decide
/-- Flickering Ward -/
def flickeringWardBounce : Ability := activated (.mana [pip .white]) (returnTo thisAura hand [])
theorem okFlickeringWardBounce : Ability.check [] flickeringWardBounce = [] := by decide
/-- Soul Conduit -/
def soulConduitExchange : Ability :=
  activated (.compound [.mana [generic 6], .tapSymbol])
    (.exchange (.lifeTotals (.described (.target (exactly 2)) .anyPlayer)))
theorem okSoulConduitExchange : Ability.check [] soulConduitExchange = [] := by decide

/-- Death-Mask Duplicant -/
def deathMaskDuplicant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death-Mask Duplicant", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ abilityWord "imprint"
            (activated (.mana [generic 1]) (exile (target (.and [creature, .inZone (graveyardOf .you)])))),
          .alsoForKeywords
            (.static (onlyWhile (.abilityGrant thisCreature (keyword "Flying"))
              (exists_ (.and [.exiledWith thisCreature, .hasKeyword (.the "Flying")]))))
            [ .the "Fear", .the "FirstStrike", .the "DoubleStrike", .the "Haste", landwalkAbilities,
              protectionAbilities, .the "Trample" ] ],
      power := stat 5, toughness := stat 5 } }

/-- Darksteel Garrison -/
def darksteelGarrison : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darksteel Garrison", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Fortification"],
      text :=
        [ .static (.abilityGrant (.attachHost .fortified (.type .land)) (keyword "Indestructible")),
          whenever (.tappedForMana none (.attachHost .fortified (.type .land)) none)
            (get (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)),
          keywordCosting "Fortify" (.mana [generic 3]) ] } }

/-- Embercleave -/
def embercleave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Embercleave", cost := some [generic 4, pip .red, pip .red], supertypes := [.legendary],
      types := [.artifact], subtypes := [artifactType "Equipment"],
      text :=
        [ keyword "Flash",
          .static (.costShift .this
            (.less (forEach 1 (.and [creature, attacking, .hasPossessor .controller .you])) none)),
          when (.enters thisEquipment none) (.attachTo it (target creatureYouControl)),
          .static (.conjunction (some (.attachHost .equipped (.type .creature)))
            [ .modification (ownSubject (.attachHost .equipped (.type .creature))) .power (.up (.lit
                1)),
              .modification (ownSubject (.attachHost .equipped (.type .creature))) .toughness (.up
                  (.lit 1)),
              .abilityGrant (ownSubject (.attachHost .equipped (.type .creature))) (keyword
                  "DoubleStrike"),
              .abilityGrant (ownSubject (.attachHost .equipped (.type .creature))) (keyword
                  "Trample") ]),
          keywordCosting "Equip" (.mana [generic 3]) ] } }

/-- Tattoo Ward -/
def tattooWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tattoo Ward", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (.retention
            (.conjunction (some (.attachHost .enchanted (.type .creature)))
              [ .modification (ownSubject (.attachHost .enchanted (.type .creature))) .power (.up
                  (.lit 1)),
                .modification (ownSubject (.attachHost .enchanted (.type .creature))) .toughness
                    (.up (.lit 1)),
                .abilityGrant (ownSubject (.attachHost .enchanted (.type .creature)))
                  (keywordQuality "Protection" enchantment) ])
            thisAura),
          activated (.perform (sacrifice thisAura (agent := .you))) (destroy (target enchantment)) ]
              } }

/-- Floating Shield -/
def floatingShield : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Floating Shield", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (entersChoosing thisAura .color),
          .static (.retention
            (.abilityGrant (.attachHost .enchanted (.type .creature)) (keywordQuality "Protection"
                (ofChosen .color)))
            thisAura),
          activated (.perform (sacrifice thisAura (agent := .you)))
            (gain (target creature) (keywordQuality "Protection" (ofChosen .color)) (some
                untilEndOfTurn)) ] } }

/-- Ghostfire Blade -/
def ghostfireBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostfire Blade", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 2)) (.up (.lit 2))),
          keywordCosting "Equip" (.mana [generic 3]),
          .static (.costShift
            (allOf (.and [ .abilityHead (.keyword "Equip"), .abilityOf .this,
                           .targets (a (.and [creature, colorless])) .someTarget ]))
            (.less (.lit 2) none)) ] } }

/-- Academy Journeymage -/
def academyJourneymage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Academy Journeymage", cost := some [generic 4, pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ .static (onlyIfSo (.costShift .this (.less (.lit 1) none))
            (exists_ (.and [.hasSubtype (creatureType "Wizard"), .hasPossessor .controller .you]))),
          when (.enters thisCreature none)
            (move (target (.and [creature, .hasPossessor .controller anOpponent])) hand) ],
      power := stat 3, toughness := stat 2 } }

/-- Alabaster Leech -/
def alabasterLeech : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Alabaster Leech", cost := some [pip .white], types := [.creature],
      subtypes := [creatureType "Leech"],
      text := [.static (spellsCost (.and [spell, .colorIs .white, castBy .you]) (.run [pip .white] true false))],
      power := stat 1, toughness := stat 3 } }

/-- Edgewalker -/
def edgewalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Edgewalker", cost := some [generic 1, pip .white, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ .static (spellsCost (.and [.hasSubtype (creatureType "Cleric"), spell, castBy .you])
            (.run [pip .white, pip .black] false true)) ],
      power := stat 2, toughness := stat 2 } }

/-- Cavern-Hoard Dragon -/
def cavernHoardDragonRider : Ability :=
  .static (.conjunction none
    [ .costShift .this (.less (.letter .x) none),
      .letterDefinition .x
        (.aggregateOver .max .opponent (countOf (.and [artifact, .hasPossessor .controller they]))) ])
theorem okCavernHoardDragonRider : Ability.check [] cavernHoardDragonRider = [] := by decide
/-- Shadowspear -/
def shadowspearStrip : Ability :=
  activated (.mana [generic 1])
    (.establish
      (.abilityLoss (allOf (.and [permanent, .hasPossessor .controller (.playerGroup
          .yourOpponents)]))
        [.written (keyword "Hexproof"), .written (keyword "Indestructible")])
      (some untilEndOfTurn))
theorem okShadowspearStrip : Ability.check [] shadowspearStrip = [] := by decide
/-- Shay Cormac -/
def shayCormacStrip : Ability :=
  activated (.mana [generic 1])
    (.establish
      (.abilityLoss (allOf (.and [permanent, .hasPossessor .controller (.playerGroup
          .yourOpponents)]))
        [ .written (keyword "Hexproof"), .written (keyword "Indestructible"), .term protectionAbilities,
          .written (keyword "Shroud"), .term wardAbilities ])
      (some untilEndOfTurn))
theorem okShayCormacStrip : Ability.check [] shayCormacStrip = [] := by decide

/-- Shelkin Brownie -/
def shelkinBrownie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shelkin Brownie", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Ouphe"],
      text :=
        [ activated (.compound [.tapSymbol])
            (.establish (.abilityLoss (target creature) [.term bandsWithOtherAbilities])
              (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Ahn-Crop Invader -/
def ahnCropInvader : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ahn-Crop Invader", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Zombie", creatureType "Minotaur", creatureType "Warrior"],
      text :=
        [ .static (.partScope .turn (some .you) (.abilityGrant thisCreature (keyword
            "FirstStrike"))),
          activated (.compound [.mana [generic 1], .perform (sacrifice (a (.and [creature,
              .otherThan .this])) (agent := .you))])
            (get thisCreature (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 2 } }

/-- Nesting Dragon -/
def nestingDragonInnerToken : Ability :=
  activated (.mana [pip .red])
    (.establish
      (.conjunction none
        [ .modification (.asMarker .token .this) .power (.up (.lit 1)),
          .modification (.asMarker .token .this) .toughness (.up (.lit 0)) ])
      (some untilEndOfTurn))
theorem okNestingDragonInnerToken : Ability.check [] nestingDragonInnerToken = [] := by decide

/-- Leonin Bola -/
def leoninBola : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leonin Bola", cost := some [generic 1], types := [.artifact],
      subtypes := [artifactType "Equipment"],
      text :=
        [ .static (.abilityGrant (.attachHost .equipped (.type .creature))
            (activated (.compound [.tapSymbol, .perform (.unattach (.theGrantor .permanent))])
              (.setStatus .tapped (target creature)))),
          keywordCosting "Equip" (.mana [generic 1]) ] } }

/-- Alluring Suitor // Deadly Dancer -/
def deadlyDancerPump : Ability :=
  activated (.mana [pip .red, pip .red])
    (.establish
      (.conjunction none
        [ .modification (.eachOf (.both thisCreature (target (.and [creature, .otherThan .this]))))
            .power (.up (.lit 1)),
          .modification (.eachOf (.both thisCreature it)) .toughness (.up (.lit 0)) ])
      (some untilEndOfTurn))
theorem okDeadlyDancerPump : Ability.check [] deadlyDancerPump = [] := by decide

/-- Unesh, Criosphinx Sovereign -/
def uneshCriosphinxSovereign : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unesh, Criosphinx Sovereign", cost := some [generic 4, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          .static (spellsCost (.and [.hasSubtype (creatureType "Sphinx"), spell, castBy .you]) (.less (.lit 2) none)),
          whenever
            (.enters (.eitherOf thisCreature
              (a (.and [.hasSubtype (creatureType "Sphinx"), .hasPossessor .controller .you, .otherThan thisCreature])))
              none)
            (.sequence
              [ revealCards (topSlice (.lit 4)),
                .separateIntoPiles them 2 [] (agent := anOpponent),
                move onePile hand,
                move (theOther .pile) graveyard ]) ],
      power := stat 4, toughness := stat 4 } }

def opt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Opt", cost := some [pip .blue], types := [.instant],
      text := [.spell none (.sequence [scry (.lit 1) (agent := .you), .draw (.lit 1) (agent :=
          .you)])] } }

def serumVisions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Serum Visions", cost := some [pip .blue], types := [.sorcery],
      text := [.spell none (.sequence [.draw (.lit 1) (agent := .you), scry (.lit 2) (agent :=
          .you)])] } }

def consider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consider", cost := some [pip .blue], types := [.instant],
      text := [.spell none (.sequence [surveil (.lit 1) (agent := .you), .draw (.lit 1) (agent :=
          .you)])] } }

def wordsOfWisdom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Words of Wisdom", cost := some [generic 1, pip .blue], types := [.instant],
      text := [.spell none (.sequence [.draw (.lit 2) (agent := .you), .draw (.lit 1) (agent :=
          (each otherPlayer))])] } }

def deathWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death Ward", cost := some [pip .white], types := [.instant],
      text := [.spell none (.regenerate (target creature))] } }

def bareTextLetter : Amount := .letter .x
theorem textAloneOnceMintedItsOwnLetter : Amount.introduced [] bareTextLetter = [letterB .x] := rfl

end Semantics.Cards
