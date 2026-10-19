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
  activated (compound [mana [pip .white], tapSymbol]) (tap (target isCreature))
theorem okMasterDecoy : Ability.check (DRS.ofList []) masterDecoy = [] := by decide
def cycling : Ability :=
  activated (compound [mana [generic 2], perform (discard this_)]) (draw (.lit
      1))
theorem okCycling : Ability.check (DRS.ofList []) cycling = [] := by decide
def merrowGrimeblotter : Ability :=
  activated (compound [mana [generic 1, hybridPip .blue .black], untapSymbol])
    (get (target isCreature) (down (.lit 2)) (down (.lit 0)) (some untilEndOfTurn))
theorem okMerrowGrimeblotter : Ability.check (DRS.ofList []) merrowGrimeblotter = [] := by decide
def phyrexianSnowcrusher : Ability :=
  activated (mana [generic 1, .snow])
    (get thisCreature (up (.lit 1)) (up (.lit 0)) (some untilEndOfTurn))
theorem okPhyrexianSnowcrusher : Ability.check (DRS.ofList []) phyrexianSnowcrusher = [] := by decide
def havocSower : Ability :=
  activated (mana [generic 1, colorlessPip])
    (get thisCreature (up (.lit 2)) (up (.lit 1)) (some untilEndOfTurn))
theorem okHavocSower : Ability.check (DRS.ofList []) havocSower = [] := by decide
/-- Erebos, God of the Dead -/
def erebos : Ability :=
  activated (compound [mana [generic 1, pip .black], payLife 2]) (draw (.lit 1))
theorem okErebos : Ability.check (DRS.ofList []) erebos = [] := by decide
def baskingRootwalla : Ability :=
  activatedOnlyOnce (mana [generic 1, pip .green])
    (get thisCreature (up (.lit 2)) (up (.lit 2)) (some untilEndOfTurn)) oncePerTurn
theorem okBaskingRootwalla : Ability.check (DRS.ofList []) baskingRootwalla = [] := by decide
def securityDetail : Ability :=
  activatedOnlyOnceIf (mana [pip .white, pip .white])
    (createToken (creatureToken 1 1 [.white] [soldier])) oncePerTurn
    (not (exists_ (and [isCreature, youControl])))
theorem okSecurityDetail : Ability.check (DRS.ofList []) securityDetail = [] := by decide
def aphettoAlchemist : Ability :=
  activated tapSymbol (untap (target (or [isArtifact, isCreature])))
theorem okAphettoAlchemist : Ability.check (DRS.ofList []) aphettoAlchemist = [] := by decide

def charRumbler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Char-Rumbler", cost := some [generic 2, pip .red, pip .red], types := [.creature],
      subtypes := [elemental],
      text :=
        [ doubleStrike,
          activated (mana [pip .red])
            (get thisCreature (up (.lit 1)) (up (.lit 0)) (some untilEndOfTurn)) ],
      power := some (.lit (-1)), toughness := stat 3 } }

/-- Sisters of Stone Death -/
def sistersOfStoneDeathRecall : Ability :=
  activated (mana [generic 2, pip .black])
    (putOntoBattlefieldUnderYourControl (a (and [isCreature, in_ (exiledWith thisCreature)])))
theorem okSistersOfStoneDeathRecall : Ability.check (DRS.ofList []) sistersOfStoneDeathRecall = [] := by decide
/-- Synod Sanctum -/
def synodSanctumReturn : Ability :=
  activated (compound [mana [generic 2], perform (sacrifice thisArtifact)])
    (putOntoBattlefieldUnderYourControl (allOf exiledWithThisArtifact))
theorem okSynodSanctumReturn : Ability.check (DRS.ofList []) synodSanctumReturn = [] := by decide

def coldStorage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cold Storage", cost := some [generic 4], types := [.artifact],
      text :=
        [ activated (mana [generic 3]) (exile (target (and [isCreature, youControl]))),
          activated (perform (sacrifice thisArtifact))
            (putOntoBattlefieldUnderYourControl (each (and [isCreature, exiledWithThisArtifact]))) ] } }

def barlsCage : Ability :=
  activated (mana [generic 3]) (skipUntap (target isCreature) (.lit 1))
theorem okBarlsCage : Ability.check (DRS.ofList []) barlsCage = [] := by decide
def vodalianIllusionist : Ability :=
  activated (compound [mana [pip .blue, pip .blue], tapSymbol])
    (instruct (target isCreature) phaseOut)
theorem okVodalianIllusionist : Ability.check (DRS.ofList []) vodalianIllusionist = [] := by decide
def witchsMist : Ability :=
  activated (compound [mana [generic 2, pip .black], tapSymbol])
    (destroy (target (and [isCreature, happenedTo any (dealDamage anyAmount which) soFarThisTurn])))
theorem okWitchsMist : Ability.check (DRS.ofList []) witchsMist = [] := by decide
def goadTargetCreature : Ability :=
  activated (compound [mana [generic 3], tapSymbol])
    (gainDesignation (target isCreature) goaded)
theorem okGoadTargetCreature : Ability.check (DRS.ofList []) goadTargetCreature = [] := by decide
/-- Krenko, Mob Boss -/
def krenko : Ability :=
  activated tapSymbol
    (sequentially
      [ createToken (creatureToken 1 1 [.red] [goblin]) (letter .x),
        define .x (countOf (and [hasSubtype goblin, youControl])) ])
theorem okKrenko : Ability.check (DRS.ofList []) krenko = [] := by decide
/-- Dokai, Weaver of Life -/
def dokai : Ability :=
  activated (compound [mana [generic 4, pip .green, pip .green], tapSymbol])
    (sequentially
      [ createToken (creatureTokenOf (letter .x) (letter .x) [.green] [elemental]),
        define .x (countOf (and [isLand, youControl])) ])
theorem okDokai : Ability.check (DRS.ofList []) dokai = [] := by decide

def ghalta : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghalta, Primal Hunger", cost := some [generic 10, pip .green, pip .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [elder, dinosaur],
      text :=
        [ static (conjunction [ costShift this_ (less (letter .x)),
              letterDefinition .x (aggregate .sum (.stat .power) (and [isCreature, youControl])) ]),
          trample ],
      power := stat 12, toughness := stat 12 } }

def ancientStoneIdol : Ability :=
  static (costShift this_ (less (forEach (.lit 1) (and [isCreature, isAttacking]))))
theorem okAncientStoneIdol : Ability.check (DRS.ofList []) ancientStoneIdol = [] := by decide

/-- "<p> spells cost {N} more/less to cast" -/
def spellsCost (p : Predicate) (shift : CostShift) : StaticSpec := costShift (allOf p) shift

def thornOfAmethyst : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thorn of Amethyst", cost := some [generic 2], types := [.artifact],
      text := [static (spellsCost (and [non isCreature, isSpell]) (more (.lit 1)))] } }

def ferozsBan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Feroz's Ban", cost := some [generic 6], types := [.artifact],
      text := [static (spellsCost (and [isCreature, isSpell]) (more (.lit 2)))] } }

def urzasFilter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Filter", cost := some [generic 4], types := [.artifact],
      text := [static (spellsCost (and [isMulticolored, isSpell]) (less (.lit 2)))] } }

def emeraldMedallion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Emerald Medallion", cost := some [generic 2], types := [.artifact],
      text := [static (spellsCost (and [hasColor .green, isSpell, castBy you]) (less (.lit 1)))] } }

/-- Highspire Bell-Ringer -/
def highspireBellRinger : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Highspire Bell-Ringer", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [djinn, monk],
      text :=
        [ flying,
          static (costShift (the (and [isSpell, nthCastBy (.nth 2) you .turn])) (less
              (.lit 1))) ],
      power := stat 1, toughness := stat 4 } }

def foundryInspector : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Foundry Inspector", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [construct],
      text := [static (spellsCost (and [isArtifact, isSpell, castBy you]) (less (.lit 1)))],
      power := stat 3, toughness := stat 2 } }

def daruWarchief : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Daru Warchief", cost := some [generic 2, pip .white, pip .white], types := [.creature],
      subtypes := [human, soldier],
      text :=
        [ static (spellsCost (and [hasSubtype soldier, isSpell, castBy you]) (less (.lit 1))),
          static (getsPt (allOf (and [hasSubtype soldier, isCreature, youControl]))
            (up (.lit 1)) (up (.lit 2))) ],
      power := stat 1, toughness := stat 1 } }

def grandArbiter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grand Arbiter Augustin IV", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, advisor],
      text :=
        [ static (spellsCost (and [hasColor .white, isSpell, castBy you]) (less (.lit 1))),
          static (spellsCost (and [hasColor .blue, isSpell, castBy you]) (less (.lit 1))),
          static (spellsCost (and [isSpell, castBy yourOpponents]) (more (.lit 1))) ],
      power := stat 2, toughness := stat 3 } }

def goblinElectromancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Goblin Electromancer", cost := some [pip .blue, pip .red], types := [.creature],
      subtypes := [goblin, wizard],
      text := [static (spellsCost (and [isInstantOrSorcery, isSpell, castBy you]) (less (.lit 1)))],
      power := stat 2, toughness := stat 2 } }

def arcaneMelee : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcane Melee", cost := some [generic 4, pip .blue], types := [.enchantment],
      text := [static (spellsCost (and [isInstantOrSorcery, isSpell]) (less (.lit 2)))] } }

def manaMatrix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Matrix", cost := some [generic 6], types := [.artifact],
      text :=
        [ static (spellsCost (and [or [isInstant, isEnchantment], isSpell, castBy you]) (less (.lit 2))) ] } }

def auraOfSilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aura of Silence", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ static (spellsCost (and [or [isArtifact, isEnchantment], isSpell, castBy yourOpponents])
            (more (.lit 2))),
          activated (perform (sacrifice thisEnchantment))
            (destroy (target (or [isArtifact, isEnchantment]))) ] } }

def chillerpillar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chillerpillar", cost := some [generic 3, pip .blue], supertypes := [.snow],
      types := [.creature], subtypes := [insect],
      text :=
        [ activated (mana [generic 4, .snow, .snow]) (monstrosity (.lit 2)),
          static (onlyWhile (abilityGrant thisCreature flying)
            (matches_ thisCreature (hasDesignation monstrous))) ],
      power := stat 3, toughness := stat 3 } }

def nullhideFerox : Ability :=
  activated (mana [generic 2])
    (establishThroughout (allAbilityLoss thisCreature) untilEndOfTurn)
theorem okNullhideFerox : Ability.check (DRS.ofList []) nullhideFerox = [] := by decide

def causticTar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Caustic Tar", cost := some [generic 4, pip .black, pip .black], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isLand,
          static (abilityGrant (enchanted land)
            (activated tapSymbol (instruct (target isPlayer) (loseLife (.lit 3))))) ] } }

/-- Saheeli, Filigree Master -/
def saheelisEmblem : Action :=
  getEmblem
    [ static (getsPt (allOf (and [isArtifact, isCreature, youControl]))
        (up (.lit 1)) (up (.lit 1))),
      static (spellsCost (and [isArtifact, isSpell, castBy you]) (less (.lit 1))) ]
theorem okSaheelisEmblem : Action.check (DRS.ofList []) saheelisEmblem = [] := by decide

def jaceBeleren : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Jace Beleren", cost := some [generic 1, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [jace],
      text :=
        [ activated (loyaltySymbol (.up 2)) (instruct eachPlayer (draw (.lit 1))),
          activated (loyaltySymbol (.down 1)) (instruct (target isPlayer) (draw (.lit 1))),
          activated (loyaltySymbol (.down 10)) (instruct (target isPlayer) (mill (.lit 20)))
              ],
      loyalty := stat 3 } }

def elspethSunsChampion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elspeth, Sun's Champion", cost := some [generic 4, pip .white, pip .white],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [elspeth],
      text :=
        [ activated (loyaltySymbol (.up 1))
            (createToken (creatureToken 1 1 [.white] [soldier]) (count := .lit 3)),
          activated (loyaltySymbol (.down 3))
            (destroy (allOf (and [isCreature, hasPower .atLeast (.lit 4)]))),
          activated (loyaltySymbol (.down 7))
            (getEmblem
              [ static (conjunction [ getsPt (allOf (and [isCreature, youControl])) (up (.lit 2)) (up (.lit 2)),
                    abilityGrant them flying ]) ]) ],
      loyalty := stat 4 } }

/-- Elspeth's Talent -/
def elspethsTalentGrant : StaticSpec :=
  abilityGrant (enchanted planeswalker)
    (activated (loyaltySymbol (.up 1))
      (createToken (creatureToken 1 1 [.white] [soldier]) (count := .lit 3)))
theorem okElspethsTalentGrant : StaticSpec.check (DRS.ofList []) elspethsTalentGrant = [] := by decide

def crystalBall : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crystal Ball", cost := some [generic 3], types := [.artifact],
      text := [activated (compound [mana [generic 1], tapSymbol]) (scry (.lit 2))] } }

/-- Angus Mackenzie -/
def angusMackenzie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Angus Mackenzie", cost := some [pip .green, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, cleric],
      text :=
        [ activatedOnlyDuring (compound [mana [pip .green, pip .white, pip .blue], tapSymbol])
            (preventAll .combatOnly everywhere (some thisTurn)) (beforePart .combatDamage none) ],
      power := stat 2, toughness := stat 2 } }

/-- Vampire Hexmage -/
def vampireHexmage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vampire Hexmage", cost := some [pip .black, pip .black], types := [.creature],
      subtypes := [vampire, shaman],
      text :=
        [ firstStrike,
          activated (perform (sacrifice thisCreature)) (removeAllCounters none
              (target isPermanent)) ],
      power := stat 2, toughness := stat 1 } }

def mizziumTransreliquat : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mizzium Transreliquat", cost := some [generic 3], types := [.artifact],
      text :=
        [ activated (mana [generic 3])
            (establishThroughout (copyChange thisArtifact (target isArtifact)) untilEndOfTurn),
          activated (mana [generic 1, pip .blue, pip .red])
            (establish (copyChange thisArtifact (target isArtifact) [exceptThisAbility])) ] } }

def tranquilGrove : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tranquil Grove", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ activated (mana [generic 1, pip .green, pip .green])
            (destroy (allOf (and [isEnchantment, otherThan this_]))) ] } }

def aggressiveMining : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggressive Mining", cost := some [generic 3, pip .red], types := [.enchantment],
      text :=
        [ static (cant you (play (allOf isLand))),
          activatedOnlyOnce (perform (sacrifice (a isLand))) (draw (.lit 2)) oncePerTurn ] } }

def rootGreevil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Root Greevil", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [beast],
      text :=
        [ activated
            (compound [mana [generic 2, pip .green], tapSymbol, perform (sacrifice thisCreature)])
            (destroy (allOf (and [isEnchantment, ofYourChoice .color]))) ],
      power := stat 2, toughness := stat 3 } }

def riptideChronologist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riptide Chronologist", cost := some [generic 3, pip .blue, pip .blue],
      types := [.creature], subtypes := [human, wizard],
      text :=
        [ activated (compound [mana [pip .blue], perform (sacrifice thisCreature)])
            (untap (allOf (and [isCreature, ofYourChoice (.subtype .creature)]))) ],
      power := stat 1, toughness := stat 3 } }

def urzasIncubator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Urza's Incubator", cost := some [generic 3], types := [.artifact],
      text :=
        [ static (entersChoosing thisArtifact (.subtype .creature)),
          static (spellsCost (and [isCreature, isSpell, ofChosen (.subtype .creature)]) (less (.lit 2))) ] } }

def etchingsOfTheChosen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Etchings of the Chosen", cost := some [generic 1, pip .white, pip .black],
      types := [.enchantment],
      text :=
        [ static (entersChoosing thisEnchantment (.subtype .creature)),
          static (getsPt (allOf (and [isCreature, youControl, ofChosen (.subtype .creature)]))
            (up (.lit 1)) (up (.lit 1))),
          activated
            (compound
              [ mana [generic 1],
                perform (sacrifice (a (and [isCreature, ofChosen (.subtype .creature)]))) ])
            (gain (target (and [isCreature, youControl])) indestructible (some untilEndOfTurn)) ] }
                }

/-- Volrath's Laboratory -/
def volrathsLaboratory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Volrath's Laboratory", cost := some [generic 5], types := [.artifact],
      text :=
        [ static volrathsLaboratoryChoice,
          activated (compound [mana [generic 5], tapSymbol])
            (createToken
              { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
                qualities := [withQuality (ofChosen .color), withQuality (ofChosen (.subtype .creature))] }) ] } }

/-- Ersatz Gnomes -/
def ersatzGnomes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ersatz Gnomes", cost := some [generic 3], types := [.artifact, .creature],
      subtypes := [gnome],
      text :=
        [ activated tapSymbol
            (establish (qualityChange (target isSpell) .sets (colored (.some [])))),
          activated tapSymbol (becomeColor (target isPermanent) (.some []) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Scrapbasket -/
def scrapbasket : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scrapbasket", cost := some [generic 4], types := [.artifact, .creature],
      subtypes := [scarecrow],
      text := [activated (mana [generic 1]) (becomeColor thisCreature .every (some
          untilEndOfTurn))],
      power := stat 3, toughness := stat 2 } }

/-- Indigo Faerie -/
def indigoFaerie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Indigo Faerie", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [faerie, wizard],
      text :=
        [ flying,
          activated (mana [pip .blue])
            (establishThroughout
               (qualityChange (target isPermanent) .adds
                  (colored (.some [.blue]))) untilEndOfTurn) ],
      power := stat 1, toughness := stat 1 } }

/-- Arcum's Weathervane -/
def arcumsWeathervane : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Arcum's Weathervane", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (compound [mana [generic 2], tapSymbol])
            (establish
              (qualityChange (target (and [isLand, hasSupertype .snow])) .loses
                (bundle { characteristics := { supertypes := [.snow] } }))),
          activated (compound [mana [generic 2], tapSymbol])
            (establish
              (qualityChange (target (and [isLand, hasSupertype .basic, not (hasSupertype
                  .snow)])) .adds
                (bundle { characteristics := { supertypes := [.snow] } }))) ] } }

def candlesOfLeng : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Candles of Leng", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (compound [mana [generic 4], tapSymbol])
            (sequentially
              [ reveal (topSlice (.lit 1)),
                doIf (matches_ it (shares .cardName .share (a (cardIn yourGraveyard))))
                  (putInto it graveyard) (some (draw (.lit 1))) ]) ] } }

def sphinxOfTheChimes : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphinx of the Chimes", cost := some [generic 4, pip .blue, pip .blue], types := [.creature],
      subtypes := [sphinx],
      text :=
        [ flying,
          activated
            (perform
               (discard (withTheSameName (counted (exactly 2) (and [non isLand, in_ yourHand])))))
            (draw (.lit 4)) ],
      power := stat 5, toughness := stat 6 } }

def endlessAtlas : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Endless Atlas", cost := some [generic 2], types := [.artifact],
      text :=
        [ activatedOnlyIf (compound [mana [generic 2], tapSymbol]) (draw (.lit 1))
            (thereAre (withTheSameName (counted (atLeast 3) (and [isLand, youControl])))) ] } }

/-- Amoeboid Changeling -/
def amoeboidChangelingTypeAbilities : List Ability :=
  [ activated tapSymbol
      (establishThroughout (qualityChange (target isCreature) .adds (everyTypeOf .creature))
         untilEndOfTurn),
    activated tapSymbol
      (establishThroughout (qualityChange (target isCreature) .loses (everyTypeOf .creature))
         untilEndOfTurn) ]
theorem okAmoeboidChangelingTypeAbilities :
    Ability.checkText (DRS.ofList []) amoeboidChangelingTypeAbilities = [] := by decide

/-- Fluctuator -/
def fluctuator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fluctuator", cost := some [generic 2], types := [.artifact],
      text :=
        [ static (costShift (allOf (and [abilityHead (.keyword Definitions.cycling), activatedBy you]))
            (less (.lit 2))) ] } }

/-- Boom Scholar -/
def boomScholarExhaustDiscount : Ability :=
  static (costShift
    (allOf (and [ abilityHead (.keyword exhaust),
                   abilityOf (allOf (and [isPermanent, otherThan thisCreature, youControl])) ]))
    (less (.lit 2)))
theorem okBoomScholarExhaustDiscount : Ability.check (DRS.ofList []) boomScholarExhaustDiscount = [] := by decide
/-- Hulk, Gamma Goliath -/
def hulkPowerUpDiscount : Ability :=
  static (costShift
    (allOf (and [ abilityHead (.keyword powerUp),
                   abilityOf (allOf (and [isCreature, otherThan thisCreature, youControl])) ]))
    (less (.lit 3)))
theorem okHulkPowerUpDiscount : Ability.check (DRS.ofList []) hulkPowerUpDiscount = [] := by decide

/-- Kopala, Warden of Waves -/
def kopalaWardenOfWaves : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kopala, Warden of Waves", cost := some [generic 1, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [merfolk, wizard],
      text :=
        [ static (costShift
            (allOf (and [ isSpell, castBy yourOpponents,
                           targeting (a (and [isCreature, hasSubtype merfolk, youControl]))
                             .someTarget ]))
            (more (.lit 2))),
          static (costShift
            (allOf (and [ abilityHead .anyActivated, activatedBy yourOpponents,
                           targeting (a (and [isCreature, hasSubtype merfolk, youControl]))
                             .someTarget ]))
            (more (.lit 2))) ],
      power := stat 2, toughness := stat 2 } }

/-- Tithe Taker -/
def titheTaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tithe Taker", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [human, soldier],
      text :=
        [ static (partScope .turn (some you)
            (conjunction [ spellsCost (and [isSpell, castBy yourOpponents]) (more (.lit 1)),
                costShift (allOf (and [ abilityHead .anyActivated, activatedBy yourOpponents,
                                      not isManaAbility ]))
                  (more (.lit 1)) ])),
          afterlife (.lit 1) ],
      power := stat 2, toughness := stat 1 } }

/-- Vexing Shusher -/
def vexingShusher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vexing Shusher", cost := some [hybridPip .red .green, hybridPip .red .green],
      types := [.creature], subtypes := [goblin, shaman],
      text :=
        [ static (cant any (counter thisSpell)),
          activated (mana [hybridPip .red .green])
            (establish (cant any (counter (target isSpell)))) ],
      power := stat 2, toughness := stat 2 } }

def trainingGrounds : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Training Grounds", cost := some [pip .blue], types := [.enchantment],
      text :=
        [ static (costShift (allOf (and [abilityHead .anyActivated, abilityOf (allOf
            (and [isCreature, youControl]))]))
            (less (.lit 2) (some (.lit 1)))) ] } }

/-- Power Artifact -/
def powerArtifact : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Power Artifact", cost := some [pip .blue, pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isArtifact,
          static (costShift
            (allOf (and [abilityHead .anyActivated, abilityOf (enchanted artifact)]))
            (less (.lit 2) (some (.lit 1)))) ] } }

/-- Fervent Champion -/
def ferventChampionEquipDiscount : Ability :=
  static (costShift
    (allOf (and [abilityHead (.keyword equip), activatedBy you, targeting thisCreature .someTarget]))
    (less (.lit 3)))
theorem okFerventChampionEquipDiscount : Ability.check (DRS.ofList []) ferventChampionEquipDiscount = [] := by decide

def suppressionField : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Suppression Field", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ static (costShift (allOf (and [abilityHead .anyActivated, not isManaAbility]))
            (more (.lit 2))) ] } }

def gloom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gloom", cost := some [generic 2, pip .black], types := [.enchantment],
      text :=
        [ static (spellsCost (and [isSpell, hasColor .white]) (more (.lit 3))),
          static (costShift
            (allOf (and [abilityHead .anyActivated, abilityOf (allOf (and [isEnchantment, hasColor .white]))]))
            (more (.lit 3))) ] } }

def bureauHeadmaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bureau Headmaster", cost := some [pip .red, pip .white], types := [.creature],
      subtypes := [human, assassin],
      text :=
        [ static (spellsCost (and [isSpell, hasSubtype equipment, castBy you]) (less (.lit 1))),
          static (costShift (allOf (and [abilityHead (.keyword equip), activatedBy you]))
              (less (.lit 1))) ],
      power := stat 2, toughness := stat 2 } }

def oppressiveRaysLine : StaticSpec :=
  costShift (allOf (and [abilityHead .anyActivated, abilityOf (enchanted (.type
      .creature))]))
    (more (.lit 3))
theorem okOppressiveRaysLine : StaticSpec.check (DRS.ofList []) oppressiveRaysLine = [] := by decide

def eidolonOfObstruction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eidolon of Obstruction", cost := some [generic 1, pip .white],
      types := [.enchantment, .creature], subtypes := [spirit],
      text :=
        [ firstStrike,
          static (costShift
            (allOf (and [ abilityHead .loyalty,
                           abilityOf (allOf (and [isPlaneswalker, opponentsControl])) ]))
            (more (.lit 1))) ],
      power := stat 2, toughness := stat 1 } }

def forceOfWill : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Force of Will", cost := some [generic 3, pip .blue, pip .blue], types := [.instant],
      text :=
        [ static (altCost this_
            (some (compound
              [ payLife 1,
                perform (exile (a (and [hasColor .blue, in_ yourHand]))) ]))),
          spell (counter (target isSpell)) ] } }

def demonOfDeathsGate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Demon of Death's Gate", cost := some [generic 6, pip .black, pip .black, pip .black],
      types := [.creature], subtypes := [demon],
      text :=
        [ static (altCost this_
            (some (compound
              [ payLife 6,
                perform (sacrifice (counted (exactly 3) (and [isCreature, hasColor .black]))) ]))),
          flying, trample ],
      power := stat 9, toughness := stat 9 } }

def drudgeSkeletons : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Drudge Skeletons", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [skeleton],
      text := [activated (mana [pip .black]) (regenerate thisCreature)],
      power := stat 1, toughness := stat 1 } }

def asphodelWanderer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Asphodel Wanderer", cost := some [pip .black], types := [.creature],
      subtypes := [skeleton, soldier],
      text := [activated (mana [generic 2, pip .black]) (regenerate thisCreature)],
      power := stat 1, toughness := stat 1 } }

def hurrJackalAbility : Ability :=
  activated tapSymbol
    (establishThroughout (cant any (regeneration (target isCreature))) thisTurn)
theorem okHurrJackalAbility : Ability.check (DRS.ofList []) hurrJackalAbility = [] := by decide

def wickedAkuba : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Wicked Akuba", cost := some [pip .black, pip .black], types := [.creature],
      subtypes := [spirit],
      text :=
        [ activated (mana [pip .black])
            (instruct
               (target
                  (and
                     [isPlayer, happenedTo thisCreature (dealDamage anyAmount which)
                        soFarThisTurn])) (loseLife
              (.lit 1))) ],
      power := stat 2, toughness := stat 2 } }

def idolOfOblivion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Idol of Oblivion", cost := some [generic 2], types := [.artifact],
      text :=
        [ activatedOnlyIf tapSymbol (draw (.lit 1))
            (happened (createObject anyAmount (token (copyOf (a isToken)) []) false) you
               soFarThisTurn),
          activated (compound [mana [generic 8], tapSymbol, perform (sacrifice thisArtifact)])
            (createToken (creatureToken 10 10 [] [eldrazi])) ] } }

def patricianGeist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Patrician Geist", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [spirit, knight],
      text :=
        [ flying,
          static (getsPt (allOf (and [hasSubtype spirit, youControl, otherThan thisCreature]))
            (up (.lit 1)) (up (.lit 1))),
          static (spellsCost (and [isSpell, castBy you, castFrom yourGraveyard]) (less (.lit 1))) ],
      power := stat 2, toughness := stat 2 } }

def shatteredEgo : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shattered Ego", cost := some [pip .blue], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (getsPt (enchanted creature) (down (.lit 3)) (down (.lit 0))),
          activated (mana [generic 3, pip .blue, pip .blue])
            (putInto (enchanted creature) (fromTop 3 library)) ] } }

def shuFarmer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shu Farmer", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [human],
      text :=
        [ activatedOnlyDuring tapSymbol (gainLife (.lit 1)) (beforePart
            .declareAttackers (some you)) ],
      power := stat 1, toughness := stat 1 } }

def elspethsTalent : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elspeth's Talent", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment], subtypes := [aura],
      text :=
        [ enchant isPlaneswalker,
          static elspethsTalentGrant,
          whenever (does you (activate loyaltyAbilityOfEnchanted))
            (establishThroughout
               (conjunction [ getsPt (allOf (and [isCreature, youControl])) (up (.lit 2)) (up
                      (.lit 2)),
                  abilityGrant them vigilance ]) untilEndOfTurn) ] } }

/-- Void Maw -/
def voidMawPutCost : Ability :=
  activated (perform (put (a (in_ (exiledWith thisCreature))) graveyard))
    (get thisCreature (up (.lit 2)) (up (.lit 2)) (some untilEndOfTurn))
theorem okVoidMawPutCost : Ability.check (DRS.ofList []) voidMawPutCost = [] := by decide
/-- Ghor-Clan Rampager -/
def ghorClanRampager : Ability :=
  abilityWord "bloodrush"
    (activated (compound [mana [pip .red, pip .green], perform (discard this_)])
      (establishFor (target (and [isCreature, isAttacking]))
        [ getsPt (target (and [isCreature, isAttacking])) (up (.lit 4)) (up (.lit
              4)),
          abilityGrant (target (and [isCreature, isAttacking])) trample ]
        (some untilEndOfTurn)))
theorem okGhorClanRampager : Ability.check (DRS.ofList []) ghorClanRampager = [] := by decide
/-- Tymora's Invoker -/
def tymorasInvoker : Ability :=
  flavorWord "Sleight of Hand" (activated (mana [generic 8]) (draw (.lit 2)))
theorem okTymorasInvoker : Ability.check (DRS.ofList []) tymorasInvoker = [] := by decide
/-- Skyblade's Boon -/
def skybladesBoonReturn : Ability :=
  activatedOnlyIf (mana [generic 2, pip .white]) (putInto this_ hand)
    (or [matches_ this_ (in_ battlefield), matches_ this_ (cardIn yourGraveyard)])
theorem okSkybladesBoonReturn : Ability.check (DRS.ofList []) skybladesBoonReturn = [] := by decide
/-- Bamboozling Beeble -/
def bamboozlingBeebleIgnore : Ability :=
  activated (compound [mana [generic 1], tapSymbol])
    (replaceNextEvent (would (target isPlayer) (rollDice anyAmount .anyDie .many))
      (instead (sequentially
        [ instruct they (rollDice (plus thatMuch (.lit 1)) .thoseDice .many .anyResult),
          ignoreOutcomes (chosenResults (some you) (.lit 1)) ]))
      (some thisTurn))
theorem okBamboozlingBeebleIgnore : Ability.check (DRS.ofList []) bamboozlingBeebleIgnore = [] := by decide
/-- General Tazri's pump -/
def generalTazriPump : Ability :=
  activated (mana [pip .white, pip .blue, pip .black, pip .red, pip .green])
    (sequentially
      [ get (each (and [isCreature, hasSubtype ally, youControl]))
          (up (letter .x)) (up (letter .x)) (some untilEndOfTurn),
        define .x (distinctCount .color (those creature)) ])
theorem okGeneralTazriPump : Ability.check (DRS.ofList []) generalTazriPump = [] := by decide
/-- Diplomatic Escort -/
def diplomaticEscortLine : Ability :=
  activated (compound [mana [pip .blue], tapSymbol, perform (discard (a (cardIn yourHand)))])
    (counter
      (target (and [or [isSpell, abilityHead .anyOnStack], targeting (a isCreature) .someTarget])))
theorem okDiplomaticEscortLine : Ability.check (DRS.ofList []) diplomaticEscortLine = [] := by decide

/-- Vorel of the Hull Clade -/
def vorelOfTheHullClade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vorel of the Hull Clade", cost := some [generic 1, pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, merfolk],
      text :=
        [ activated (compound [mana [pip .green, pip .blue], tapSymbol])
            (doubleCounters (target (or [isArtifact, isCreature, isLand]))) ],
      power := stat 1, toughness := stat 4 } }

/-- Prosperity -/
theorem prosperityCostLetters : costLetters (some [.variable, pip .blue]) = [letterB .x] := rfl
def prosperityTextLetter : Amount := letter .x
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
      subtypes := [aura],
      text :=
        [ flash,
          enchant (or [isCreature, hasSubtype food]),
          static (conjunction [ qualityChange (enchanted permanent) .sets
                (bundle
                  { characteristics :=
                    { types := [.artifact], subtypes := [food],
                      text :=
                        [ activated (compound [mana [generic 2], tapSymbol, perform (sacrifice
                            thisArtifact)])
                            (gainLife (.lit 3)) ] } }),
              allAbilityLoss it ]) ] } }

/-- Doc Aurlock, Grizzled Genius -/
def docAurlockCost : StaticSpec :=
  spellsCost (and [isSpell, castBy you, or [castFrom yourGraveyard, castFrom exileZone]])
    (less (.lit 2))
theorem okDocAurlockCost : StaticSpec.check (DRS.ofList []) docAurlockCost = [] := by decide
/-- Obelisk of Undoing -/
def obeliskOfUndoing : Ability :=
  activated (compound [mana [generic 6], tapSymbol])
    (returnToHand (target (and [isPermanent, youOwn, youControl])))
theorem okObeliskOfUndoing : Ability.check (DRS.ofList []) obeliskOfUndoing = [] := by decide
/-- Flickering Ward -/
def flickeringWardBounce : Ability := activated (mana [pip .white]) (returnToHand thisAura)
theorem okFlickeringWardBounce : Ability.check (DRS.ofList []) flickeringWardBounce = [] := by decide
/-- Soul Conduit -/
def soulConduitExchange : Ability :=
  activated (compound [mana [generic 6], tapSymbol])
    (exchange (lifeTotals (targets (exactly 2) isPlayer)))
theorem okSoulConduitExchange : Ability.check (DRS.ofList []) soulConduitExchange = [] := by decide

/-- Death-Mask Duplicant -/
def deathMaskDuplicant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death-Mask Duplicant", cost := some [generic 7], types := [.artifact, .creature],
      subtypes := [shapeshifter],
      text :=
        [ abilityWord "imprint"
            (activated (mana [generic 1]) (exile (target (and [isCreature, in_ yourGraveyard])))),
          alsoForKeywords
            (static (onlyWhile (abilityGrant thisCreature flying)
              (exists_ (and [in_ (exiledWith thisCreature), hasKeyword flying]))))
            [ fear, firstStrike, doubleStrike, haste, landwalkAbilities,
              protectionAbilities, trample ] ],
      power := stat 5, toughness := stat 5 } }

/-- Darksteel Garrison -/
def darksteelGarrison : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darksteel Garrison", cost := some [generic 2], types := [.artifact],
      subtypes := [fortification],
      text :=
        [ static (abilityGrant (fortified land) indestructible),
          whenever (does any (tapForMana (fortified land)))
            (get (target isCreature) (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn)),
          fortify (mana [generic 3]) ] } }

/-- Embercleave -/
def embercleave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Embercleave", cost := some [generic 4, pip .red, pip .red], supertypes := [.legendary],
      types := [.artifact], subtypes := [equipment],
      text :=
        [ flash,
          static (costShift this_
            (less (forEach (.lit 1) (and [isCreature, isAttacking, youControl])))),
          when (does thisEquipment enter) (attachTo it (target (and [isCreature, youControl]))),
          static (conjunction [ getsPt (equipped creature) (up (.lit
                1)) (up
                  (.lit 1)),
              abilityGrant (equipped creature) doubleStrike,
              abilityGrant (equipped creature) trample ] (subject :=
                                                                     some
                                                                        (equipped
                                                                           creature))),
          equip (mana [generic 3]) ] } }

/-- Tattoo Ward -/
def tattooWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tattoo Ward", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (retention
            (conjunction [ getsPt (enchanted creature) (up
                  (.lit 1)) (up (.lit 1)),
                abilityGrant (enchanted creature)
                  (protection isEnchantment) ] (subject :=
                                                                            some
                                                                               (enchanted
                                                                                  (.type
                                                                                     .creature))))
            thisAura),
          activated (perform (sacrifice thisAura)) (destroy (target isEnchantment)) ]
              } }

/-- Floating Shield -/
def floatingShield : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Floating Shield", cost := some [generic 2, pip .white], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (entersChoosing thisAura .color),
          static (retention
            (abilityGrant (enchanted creature) (protection
                (ofChosen .color)))
            thisAura),
          activated (perform (sacrifice thisAura))
            (gain (target isCreature) (protection (ofChosen .color)) (some
                untilEndOfTurn)) ] } }

/-- Ghostfire Blade -/
def ghostfireBlade : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostfire Blade", cost := some [generic 1], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up (.lit 2)) (up (.lit 2))),
          equip (mana [generic 3]),
          static (costShift
            (allOf (and [ abilityHead (.keyword equip), abilityOf this_,
                           targeting (a (and [isCreature, isColorless])) .someTarget ]))
            (less (.lit 2))) ] } }

/-- Academy Journeymage -/
def academyJourneymage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Academy Journeymage", cost := some [generic 4, pip .blue], types := [.creature],
      subtypes := [human, wizard],
      text :=
        [ static (onlyIfSo (costShift this_ (less (.lit 1)))
            (exists_ (and [hasSubtype wizard, youControl]))),
          when (does thisCreature enter)
            (putInto (target (and [isCreature, anOpponentControls])) hand) ],
      power := stat 3, toughness := stat 2 } }

/-- Alabaster Leech -/
def alabasterLeech : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Alabaster Leech", cost := some [pip .white], types := [.creature],
      subtypes := [leech],
      text := [static (spellsCost (and [isSpell, hasColor .white, castBy you]) (bySymbols (symbols [pip .white]) true false))],
      power := stat 1, toughness := stat 3 } }

/-- Edgewalker -/
def edgewalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Edgewalker", cost := some [generic 1, pip .white, pip .black], types := [.creature],
      subtypes := [human, cleric],
      text :=
        [ static (spellsCost (and [hasSubtype cleric, isSpell, castBy you])
            (bySymbols (symbols [pip .white, pip .black]) false true)) ],
      power := stat 2, toughness := stat 2 } }

/-- Cavern-Hoard Dragon -/
def cavernHoardDragonRider : Ability :=
  static (conjunction [ costShift this_ (less (letter .x)),
      letterDefinition .x
        (aggregateOver .max isOpponent (countOf (and [isArtifact, controlledBy they]))) ])
theorem okCavernHoardDragonRider : Ability.check (DRS.ofList []) cavernHoardDragonRider = [] := by decide
/-- Shadowspear -/
def shadowspearStrip : Ability :=
  activated (mana [generic 1])
    (establishThroughout (abilityLoss (allOf (and [isPermanent, opponentsControl]))
        [written hexproof, written
           indestructible]) untilEndOfTurn)
theorem okShadowspearStrip : Ability.check (DRS.ofList []) shadowspearStrip = [] := by decide
/-- Shay Cormac -/
def shayCormacStrip : Ability :=
  activated (mana [generic 1])
    (establishThroughout (abilityLoss (allOf (and [isPermanent, opponentsControl]))
        [ written hexproof, written indestructible, abilities protectionAbilities,
          written shroud, abilities wardAbilities ]) untilEndOfTurn)
theorem okShayCormacStrip : Ability.check (DRS.ofList []) shayCormacStrip = [] := by decide

/-- Shelkin Brownie -/
def shelkinBrownie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shelkin Brownie", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [ouphe],
      text :=
        [ activated (compound [tapSymbol])
            (establishThroughout
               (abilityLoss (target isCreature)
                  [abilities bandsWithOtherAbilities]) untilEndOfTurn) ],
      power := stat 1, toughness := stat 1 } }

/-- Ahn-Crop Invader -/
def ahnCropInvader : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ahn-Crop Invader", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [zombie, minotaur, warrior],
      text :=
        [ static (partScope .turn (some you) (abilityGrant thisCreature firstStrike)),
          activated (compound [mana [generic 1], perform (sacrifice (a (and [isCreature,
              otherThan this_])))])
            (get thisCreature (up (.lit 2)) (up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 2 } }

/-- Nesting Dragon -/
def nestingDragonInnerToken : Ability :=
  activated (mana [pip .red])
    (establishThroughout
       (conjunction [ getsPt thisToken (up (.lit 1)) (up (.lit 0)) ]) untilEndOfTurn)
theorem okNestingDragonInnerToken : Ability.check (DRS.ofList []) nestingDragonInnerToken = [] := by decide

/-- Leonin Bola -/
def leoninBola : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Leonin Bola", cost := some [generic 1], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (abilityGrant (equipped creature)
            (activated (compound [tapSymbol, perform (unattach (theGrantor .permanent))])
              (tap (target isCreature)))),
          equip (mana [generic 1]) ] } }

/-- Alluring Suitor // Deadly Dancer -/
def deadlyDancerPump : Ability :=
  activated (mana [pip .red, pip .red])
    (establishThroughout
       (conjunction
          [ modification (eachOf (both thisCreature (target (and [isCreature, otherThan this_]))))
            .power (up (.lit 1)),
          modification (eachOf (both thisCreature it)) .toughness (up (.lit 0)) ]) untilEndOfTurn)
theorem okDeadlyDancerPump : Ability.check (DRS.ofList []) deadlyDancerPump = [] := by decide

/-- Unesh, Criosphinx Sovereign -/
def uneshCriosphinxSovereign : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unesh, Criosphinx Sovereign", cost := some [generic 4, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [sphinx],
      text :=
        [ flying,
          static (spellsCost (and [hasSubtype sphinx, isSpell, castBy you]) (less (.lit 2))),
          whenever
            (does (eitherOf thisCreature
              (a (and [hasSubtype sphinx, youControl, otherThan thisCreature]))) enter)
            (sequentially
              [ reveal (topSlice (.lit 4)),
                instruct anOpponent (separateIntoPiles them 2),
                putInto onePile hand,
                putInto (theOther .pile) graveyard ]) ],
      power := stat 4, toughness := stat 4 } }

def opt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Opt", cost := some [pip .blue], types := [.instant],
      text := [spell (sequentially [scry (.lit 1), draw (.lit 1)])] } }

def serumVisions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Serum Visions", cost := some [pip .blue], types := [.sorcery],
      text := [spell (sequentially [draw (.lit 1), scry (.lit 2)])] } }

def consider : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Consider", cost := some [pip .blue], types := [.instant],
      text := [spell (sequentially [surveil (.lit 1), draw (.lit 1)])] } }

def wordsOfWisdom : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Words of Wisdom", cost := some [generic 1, pip .blue], types := [.instant],
      text := [spell (sequentially [draw (.lit 2), instruct (each isOtherPlayer) (draw (.lit 1))])] } }

def deathWard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death Ward", cost := some [pip .white], types := [.instant],
      text := [spell (regenerate (target isCreature))] } }

def bareTextLetter : Amount := letter .x
theorem textAloneOnceMintedItsOwnLetter :
    (stepAmount 64 (DRS.ofList []) bareTextLetter).1.referents = [letterB .x] := by decide

end Semantics.Cards
