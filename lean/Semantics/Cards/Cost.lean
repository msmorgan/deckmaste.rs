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
  activated (compound [[pip .white], tapSymbol]) (tap (target isCreature))
theorem okMasterDecoy : Ability.check (DRS.ofList []) masterDecoy = [] := by decide
def cycling : Ability :=
  activated (compound [[generic 2], discard this_]) (draw 1)
theorem okCycling : Ability.check (DRS.ofList []) cycling = [] := by decide
def merrowGrimeblotter : Ability :=
  activated (compound [[generic 1, hybridPip .blue .black], untapSymbol])
    (get (target isCreature) (down 2) (down 0) untilEndOfTurn)
theorem okMerrowGrimeblotter : Ability.check (DRS.ofList []) merrowGrimeblotter = [] := by decide
def phyrexianSnowcrusher : Ability :=
  activated [generic 1, .snow]
    (get thisCreature (up 1) (up 0) untilEndOfTurn)
theorem okPhyrexianSnowcrusher : Ability.check (DRS.ofList []) phyrexianSnowcrusher = [] := by decide
def havocSower : Ability :=
  activated [generic 1, colorlessPip]
    (get thisCreature (up 2) (up 1) untilEndOfTurn)
theorem okHavocSower : Ability.check (DRS.ofList []) havocSower = [] := by decide
/-- Erebos, God of the Dead -/
def erebos : Ability :=
  activated (compound [[generic 1, pip .black], payLife 2]) (draw 1)
theorem okErebos : Ability.check (DRS.ofList []) erebos = [] := by decide
def baskingRootwalla : Ability :=
  activatedOnlyOnce [generic 1, pip .green]
    (get thisCreature (up 2) (up 2) untilEndOfTurn) oncePerTurn
theorem okBaskingRootwalla : Ability.check (DRS.ofList []) baskingRootwalla = [] := by decide
def securityDetail : Ability :=
  activatedOnlyOnceIf [pip .white, pip .white]
    (create 1 (creatureToken 1 1 [.white] [soldier])) oncePerTurn
    (not (exists_ (and [isCreature, youControl])))
theorem okSecurityDetail : Ability.check (DRS.ofList []) securityDetail = [] := by decide
def aphettoAlchemist : Ability :=
  activated tapSymbol (untap (target (or [isArtifact, isCreature])))
theorem okAphettoAlchemist : Ability.check (DRS.ofList []) aphettoAlchemist = [] := by decide

def charRumbler : Spelled := spelled <| singleFacedCard
  { name := "Char-Rumbler", cost := [generic 2, pip .red, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ doubleStrike,
        activated [pip .red]
          (get thisCreature (up 1) (up 0) untilEndOfTurn) ],
    power := some (.lit (-1)), toughness := stat 3 }

/-- Sisters of Stone Death -/
def sistersOfStoneDeathRecall : Ability :=
  activated [generic 2, pip .black]
    (putOntoBattlefieldUnderYourControl (a (and [isCreature, in_ (exiledWith thisCreature)])))
theorem okSistersOfStoneDeathRecall : Ability.check (DRS.ofList []) sistersOfStoneDeathRecall = [] := by decide
/-- Synod Sanctum -/
def synodSanctumReturn : Ability :=
  activated (compound [[generic 2], sacrifice thisArtifact])
    (returnToBattlefield (allOf exiledWithThisArtifact) [(under you)])
theorem okSynodSanctumReturn : Ability.check (DRS.ofList []) synodSanctumReturn = [] := by decide

def coldStorage : Spelled := spelled <| singleFacedCard
  { name := "Cold Storage", cost := [generic 4], types := [.artifact],
    text :=
      [ activated [generic 3] (exile (target (and [isCreature, youControl]))),
        activated (sacrifice thisArtifact)
          (returnToBattlefield (each (and [isCreature, exiledWithThisArtifact]))
            [(under you)]) ] }

def barlsCage : Ability :=
  activated [generic 3] (skipUntap (target isCreature) 1)
theorem okBarlsCage : Ability.check (DRS.ofList []) barlsCage = [] := by decide
def vodalianIllusionist : Ability :=
  activated (compound [[pip .blue, pip .blue], tapSymbol])
    (instruct (target isCreature) phaseOut)
theorem okVodalianIllusionist : Ability.check (DRS.ofList []) vodalianIllusionist = [] := by decide
def witchsMist : Ability :=
  activated (compound [[generic 2, pip .black], tapSymbol])
    (destroy (target (and [isCreature, was (dealDamage anyAmount which) soFarThisTurn])))
theorem okWitchsMist : Ability.check (DRS.ofList []) witchsMist = [] := by decide
def goadTargetCreature : Ability :=
  activated (compound [[generic 3], tapSymbol])
    (gainDesignation (target isCreature) goaded)
theorem okGoadTargetCreature : Ability.check (DRS.ofList []) goadTargetCreature = [] := by decide
/-- Krenko, Mob Boss -/
def krenko : Ability :=
  activated tapSymbol
    (sequentially
      [ create (letter .x) (creatureToken 1 1 [.red] [goblin]),
        define .x (countOf (and [goblin, youControl])) ])
theorem okKrenko : Ability.check (DRS.ofList []) krenko = [] := by decide
/-- Dokai, Weaver of Life -/
def dokai : Ability :=
  activated (compound [[generic 4, pip .green, pip .green], tapSymbol])
    (sequentially
      [ create 1 (creatureTokenOf (letter .x) (letter .x) [.green] [elemental]),
        define .x (countOf (and [isLand, youControl])) ])
theorem okDokai : Ability.check (DRS.ofList []) dokai = [] := by decide

def ghalta : Spelled := spelled <| singleFacedCard
  { name := "Ghalta, Primal Hunger", cost := [generic 10, pip .green, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [elder, dinosaur],
    text :=
      [ static (conjunction [ costShift this_ (less (letter .x)),
            letterDefinition .x (aggregate .sum (.stat .power) (and [isCreature, youControl])) ]),
        trample ],
    power := stat 12, toughness := stat 12 }

def ancientStoneIdol : Ability :=
  static (costShift this_ (less (forEach 1 (and [isCreature, isAttacking]))))
theorem okAncientStoneIdol : Ability.check (DRS.ofList []) ancientStoneIdol = [] := by decide

/-- "<p> spells cost {N} more/less to cast" -/
def spellsCost (p : Predicate) (shift : CostShift) : StaticSpec := costShift (allOf p) shift

def thornOfAmethyst : Spelled := spelled <| singleFacedCard
  { name := "Thorn of Amethyst", cost := [generic 2], types := [.artifact],
    text := [static (spellsCost (and [non isCreature, isSpell]) (more 1))] }

def ferozsBan : Spelled := spelled <| singleFacedCard
  { name := "Feroz's Ban", cost := [generic 6], types := [.artifact],
    text := [static (spellsCost (and [isCreature, isSpell]) (more 2))] }

def urzasFilter : Spelled := spelled <| singleFacedCard
  { name := "Urza's Filter", cost := [generic 4], types := [.artifact],
    text := [static (spellsCost (and [isMulticolored, isSpell]) (less 2))] }

def emeraldMedallion : Spelled := spelled <| singleFacedCard
  { name := "Emerald Medallion", cost := [generic 2], types := [.artifact],
    text := [static (spellsCost (and [hasColor .green, isSpell, did you (cast which) thisObject])
        (less 1))] }

/-- Highspire Bell-Ringer -/
def highspireBellRinger : Spelled := spelled <| singleFacedCard
  { name := "Highspire Bell-Ringer", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [djinn, monk],
    text :=
      [ flying,
        static (costShift
          (the (and [ isSpell,
                      did you (cast which) thisObject (ordinal := some (.nth 2, some .turn)) ]))
          (less 1)) ],
    power := stat 1, toughness := stat 4 }

def foundryInspector : Spelled := spelled <| singleFacedCard
  { name := "Foundry Inspector", cost := [generic 3], types := [.artifact, .creature],
    subtypes := [construct],
    text := [static (spellsCost (and [isArtifact, isSpell, did you (cast which) thisObject])
        (less 1))],
    power := stat 3, toughness := stat 2 }

def daruWarchief : Spelled := spelled <| singleFacedCard
  { name := "Daru Warchief", cost := [generic 2, pip .white, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ static (spellsCost (and [soldier, isSpell, did you (cast which) thisObject]) (less 1)),
        static (getsPt (allOf (and [soldier, isCreature, youControl]))
          (up 1) (up 2)) ],
    power := stat 1, toughness := stat 1 }

def grandArbiter : Spelled := spelled <| singleFacedCard
  { name := "Grand Arbiter Augustin IV", cost := [generic 2, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, advisor],
    text :=
      [ static (spellsCost (and [hasColor .white, isSpell, did you (cast which) thisObject])
          (less 1)),
        static (spellsCost (and [hasColor .blue, isSpell, did you (cast which) thisObject])
          (less 1)),
        static (spellsCost (and [isSpell, did yourOpponents (cast which) thisObject]) (more 1)) ],
    power := stat 2, toughness := stat 3 }

def goblinElectromancer : Spelled := spelled <| singleFacedCard
  { name := "Goblin Electromancer", cost := [pip .blue, pip .red], types := [.creature],
    subtypes := [goblin, wizard],
    text := [static (spellsCost (and [isInstantOrSorcery, isSpell,
        did you (cast which) thisObject]) (less 1))],
    power := stat 2, toughness := stat 2 }

def arcaneMelee : Spelled := spelled <| singleFacedCard
  { name := "Arcane Melee", cost := [generic 4, pip .blue], types := [.enchantment],
    text := [static (spellsCost (and [isInstantOrSorcery, isSpell]) (less 2))] }

def manaMatrix : Spelled := spelled <| singleFacedCard
  { name := "Mana Matrix", cost := [generic 6], types := [.artifact],
    text :=
      [ static (spellsCost (and [or [isInstant, isEnchantment], isSpell,
          did you (cast which) thisObject]) (less 2)) ] }

def auraOfSilence : Spelled := spelled <| singleFacedCard
  { name := "Aura of Silence", cost := [generic 1, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (spellsCost (and [or [isArtifact, isEnchantment], isSpell,
          did yourOpponents (cast which) thisObject])
          (more 2)),
        activated (sacrifice thisEnchantment)
          (destroy (target (or [isArtifact, isEnchantment]))) ] }

def chillerpillar : Spelled := spelled <| singleFacedCard
  { name := "Chillerpillar", cost := [generic 3, pip .blue], supertypes := [.snow],
    types := [.creature], subtypes := [insect],
    text :=
      [ activated [generic 4, .snow, .snow] (monstrosity 2),
        static (onlyWhile (abilityGrant thisCreature flying)
          (is thisCreature (hasDesignation monstrous))) ],
    power := stat 3, toughness := stat 3 }

def nullhideFerox : Ability :=
  activated [generic 2]
    (establishThroughout (allAbilityLoss thisCreature) untilEndOfTurn)
theorem okNullhideFerox : Ability.check (DRS.ofList []) nullhideFerox = [] := by decide

def causticTar : Spelled := spelled <| singleFacedCard
  { name := "Caustic Tar", cost := [generic 4, pip .black, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isLand,
        static (abilityGrant (enchanted land)
          (activated tapSymbol (instruct (target isPlayer) (loseLife 3)))) ] }

/-- Saheeli, Filigree Master -/
def saheelisEmblem : Action :=
  getEmblem
    [ static (getsPt (allOf (and [isArtifact, isCreature, youControl]))
        (up 1) (up 1)),
      static (spellsCost (and [isArtifact, isSpell, did you (cast which) thisObject]) (less 1)) ]
theorem okSaheelisEmblem : Action.check (DRS.ofList []) saheelisEmblem = [] := by decide

def jaceBeleren : Spelled := spelled <| singleFacedCard
  { name := "Jace Beleren", cost := [generic 1, pip .blue, pip .blue],
    supertypes := [.legendary], types := [.planeswalker], subtypes := [jace],
    text :=
      [ activated (loyaltySymbol (up 2)) (instruct eachPlayer (draw 1)),
        activated (loyaltySymbol (down 1)) (instruct (target isPlayer) (draw 1)),
        activated (loyaltySymbol (down 10)) (instruct (target isPlayer) (mill 20))
            ],
    loyalty := stat 3 }

def elspethSunsChampion : Spelled := spelled <| singleFacedCard
  { name := "Elspeth, Sun's Champion", cost := [generic 4, pip .white, pip .white],
    supertypes := [.legendary], types := [.planeswalker], subtypes := [elspeth],
    text :=
      [ activated (loyaltySymbol (up 1))
          (create 3 (creatureToken 1 1 [.white] [soldier])),
        activated (loyaltySymbol (down 3))
          (destroy (allOf (and [isCreature, hasPower .atLeast 4]))),
        activated (loyaltySymbol (down 7))
          (getEmblem
            [ static (conjunction [ getsPt (allOf (and [isCreature, youControl])) (up 2) (up 2),
                  abilityGrant them flying ]) ]) ],
    loyalty := stat 4 }

/-- Elspeth's Talent -/
def elspethsTalentGrant : StaticSpec :=
  abilityGrant (enchanted planeswalker)
    (activated (loyaltySymbol (up 1))
      (create 3 (creatureToken 1 1 [.white] [soldier])))
theorem okElspethsTalentGrant : StaticSpec.check (DRS.ofList []) elspethsTalentGrant = [] := by decide

def crystalBall : Spelled := spelled <| singleFacedCard
  { name := "Crystal Ball", cost := [generic 3], types := [.artifact],
    text := [activated (compound [[generic 1], tapSymbol]) (scry 2)] }

/-- Angus Mackenzie -/
def angusMackenzie : Spelled := spelled <| singleFacedCard
  { name := "Angus Mackenzie", cost := [pip .green, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ activatedOnlyDuring (compound [[pip .green, pip .white, pip .blue], tapSymbol])
          (preventAll any thisTurn (some .combat))
          (beforePart .combatDamage none) ],
    power := stat 2, toughness := stat 2 }

/-- Vampire Hexmage -/
def vampireHexmage : Spelled := spelled <| singleFacedCard
  { name := "Vampire Hexmage", cost := [pip .black, pip .black], types := [.creature],
    subtypes := [vampire, shaman],
    text :=
      [ firstStrike,
        activated (sacrifice thisCreature) (removeAllCounters none
            (target isPermanent)) ],
    power := stat 2, toughness := stat 1 }

def mizziumTransreliquat : Spelled := spelled <| singleFacedCard
  { name := "Mizzium Transreliquat", cost := [generic 3], types := [.artifact],
    text :=
      [ activated [generic 3]
          (establishThroughout (copyChange thisArtifact (target isArtifact)) untilEndOfTurn),
        activated [generic 1, pip .blue, pip .red]
          (establish (copyChange thisArtifact (target isArtifact) [exceptThisAbility])) ] }

def tranquilGrove : Spelled := spelled <| singleFacedCard
  { name := "Tranquil Grove", cost := [generic 1, pip .green], types := [.enchantment],
    text :=
      [ activated [generic 1, pip .green, pip .green]
          (destroy (allOf (and [isEnchantment, isOther]))) ] }

def aggressiveMining : Spelled := spelled <| singleFacedCard
  { name := "Aggressive Mining", cost := [generic 3, pip .red], types := [.enchantment],
    text :=
      [ static (cant you (play (allOf isLand))),
        activatedOnlyOnce (sacrifice (a isLand)) (draw 2) oncePerTurn ] }

def rootGreevil : Spelled := spelled <| singleFacedCard
  { name := "Root Greevil", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [beast],
    text :=
      [ activated
          (compound [[generic 2, pip .green], tapSymbol, sacrifice thisCreature])
          (destroy (allOf (and [isEnchantment, ofYourChoice .color]))) ],
    power := stat 2, toughness := stat 3 }

def riptideChronologist : Spelled := spelled <| singleFacedCard
  { name := "Riptide Chronologist", cost := [generic 3, pip .blue, pip .blue],
    types := [.creature], subtypes := [human, wizard],
    text :=
      [ activated (compound [[pip .blue], sacrifice thisCreature])
          (untap (allOf (and [isCreature, ofYourChoice (.subtype .creature)]))) ],
    power := stat 1, toughness := stat 3 }

def urzasIncubator : Spelled := spelled <| singleFacedCard
  { name := "Urza's Incubator", cost := [generic 3], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact (.subtype .creature)),
        static (spellsCost (and [isCreature, isSpell, ofChosen (.subtype .creature)]) (less 2)) ] }

def etchingsOfTheChosen : Spelled := spelled <| singleFacedCard
  { name := "Etchings of the Chosen", cost := [generic 1, pip .white, pip .black],
    types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (getsPt (allOf (and [isCreature, youControl, ofChosen (.subtype .creature)]))
          (up 1) (up 1)),
        activated
          (compound
            [ [generic 1],
              (sacrifice (a (and [isCreature, ofChosen (.subtype .creature)]))) ])
          (gain (target (and [isCreature, youControl])) indestructible untilEndOfTurn) ] }

/-- Volrath's Laboratory -/
def volrathsLaboratory : Spelled := spelled <| singleFacedCard
  { name := "Volrath's Laboratory", cost := [generic 5], types := [.artifact],
    text :=
      [ static volrathsLaboratoryChoice,
        activated (compound [[generic 5], tapSymbol])
          (create 1 (written
            { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
              qualities := [withQuality (ofChosen .color), withQuality (ofChosen (.subtype .creature))] })) ] }

/-- Ersatz Gnomes -/
def ersatzGnomes : Spelled := spelled <| singleFacedCard
  { name := "Ersatz Gnomes", cost := [generic 3], types := [.artifact, .creature],
    subtypes := [gnome],
    text :=
      [ activated tapSymbol
          (becomeColor (target isSpell) (.some [])),
        activated tapSymbol (becomeColor (target isPermanent) (.some []) untilEndOfTurn) ],
    power := stat 1, toughness := stat 1 }

/-- Scrapbasket -/
def scrapbasket : Spelled := spelled <| singleFacedCard
  { name := "Scrapbasket", cost := [generic 4], types := [.artifact, .creature],
    subtypes := [scarecrow],
    text := [activated [generic 1] (becomeColor thisCreature .every (some
        untilEndOfTurn))],
    power := stat 3, toughness := stat 2 }

/-- Indigo Faerie -/
def indigoFaerie : Spelled := spelled <| singleFacedCard
  { name := "Indigo Faerie", cost := [generic 1, pip .blue], types := [.creature],
    subtypes := [faerie, wizard],
    text :=
      [ flying,
        activated [pip .blue]
          (establishThroughout
             (qualityChange (target isPermanent) .adds
                (colored (.some [.blue]))) untilEndOfTurn) ],
    power := stat 1, toughness := stat 1 }

/-- Arcum's Weathervane -/
def arcumsWeathervane : Spelled := spelled <| singleFacedCard
  { name := "Arcum's Weathervane", cost := [generic 2], types := [.artifact],
    text :=
      [ activated (compound [[generic 2], tapSymbol])
          (establish
            (qualityChange (target (and [isLand, hasSupertype .snow])) .loses
              (bundle { characteristics := { supertypes := [.snow] } }))),
        activated (compound [[generic 2], tapSymbol])
          (establish
            (qualityChange (target (and [isLand, hasSupertype .basic, not (hasSupertype
                .snow)])) .adds
              (bundle { characteristics := { supertypes := [.snow] } }))) ] }

def candlesOfLeng : Spelled := spelled <| singleFacedCard
  { name := "Candles of Leng", cost := [generic 2], types := [.artifact],
    text :=
      [ activated (compound [[generic 4], tapSymbol])
          (sequentially
            [ reveal (topSlice 1),
              doIf (is it (shares .cardName .share (a (cardIn yourGraveyard))))
                (putInto it graveyard) (draw 1) ]) ] }

def sphinxOfTheChimes : Spelled := spelled <| singleFacedCard
  { name := "Sphinx of the Chimes", cost := [generic 4, pip .blue, pip .blue], types := [.creature],
    subtypes := [sphinx],
    text :=
      [ flying,
        activated
          (perform
             (discard (withTheSameName (counted (exactly 2) (and [non isLand, in_ yourHand])))))
          (draw 4) ],
    power := stat 5, toughness := stat 6 }

def endlessAtlas : Spelled := spelled <| singleFacedCard
  { name := "Endless Atlas", cost := [generic 2], types := [.artifact],
    text :=
      [ activatedOnlyIf (compound [[generic 2], tapSymbol]) (draw 1)
          (thereAre (withTheSameName (counted (orMore 3) (and [isLand, youControl])))) ] }

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
def fluctuator : Spelled := spelled <| singleFacedCard
  { name := "Fluctuator", cost := [generic 2], types := [.artifact],
    text :=
      [ static (costShift (allOf (and [abilityHead (.keyword Definitions.cycling),
          did you (activate which) thisObject]))
          (less 2)) ] }

/-- Boom Scholar -/
def boomScholarExhaustDiscount : Ability :=
  static (costShift
    (allOf (and [ abilityHead (.keyword exhaust),
                   abilityOf (allOf (and [isPermanent, isOther, youControl])) ]))
    (less 2))
theorem okBoomScholarExhaustDiscount : Ability.check (DRS.ofList []) boomScholarExhaustDiscount = [] := by decide
/-- Hulk, Gamma Goliath -/
def hulkPowerUpDiscount : Ability :=
  static (costShift
    (allOf (and [ abilityHead (.keyword powerUp),
                   abilityOf (allOf (and [isCreature, isOther, youControl])) ]))
    (less 3))
theorem okHulkPowerUpDiscount : Ability.check (DRS.ofList []) hulkPowerUpDiscount = [] := by decide

/-- Kopala, Warden of Waves -/
def kopalaWardenOfWaves : Spelled := spelled <| singleFacedCard
  { name := "Kopala, Warden of Waves", cost := [generic 1, pip .blue, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [merfolk, wizard],
    text :=
      [ static (costShift
          (allOf (and [ isSpell, did yourOpponents (cast which) thisObject,
                         targeting (a (and [isCreature, merfolk, youControl]))
                           .someTarget ]))
          (more 2)),
        static (costShift
          (allOf (and [ abilityHead .anyActivated, did yourOpponents (activate which) thisObject,
                         targeting (a (and [isCreature, merfolk, youControl]))
                           .someTarget ]))
          (more 2)) ],
    power := stat 2, toughness := stat 2 }

/-- Tithe Taker -/
def titheTaker : Spelled := spelled <| singleFacedCard
  { name := "Tithe Taker", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ static (partScope .turn you
          (conjunction [ spellsCost (and [isSpell, did yourOpponents (cast which) thisObject])
              (more 1),
              costShift (allOf (and [ abilityHead .anyActivated,
                  did yourOpponents (activate which) thisObject,
                                    not isManaAbility ]))
                (more 1) ])),
        afterlife 1 ],
    power := stat 2, toughness := stat 1 }

/-- Vexing Shusher -/
def vexingShusher : Spelled := spelled <| singleFacedCard
  { name := "Vexing Shusher", cost := [hybridPip .red .green, hybridPip .red .green],
    types := [.creature], subtypes := [goblin, shaman],
    text :=
      [ static (cant any (counter thisSpell)),
        activated [hybridPip .red .green]
          (establish (cant any (counter (target isSpell)))) ],
    power := stat 2, toughness := stat 2 }

def trainingGrounds : Spelled := spelled <| singleFacedCard
  { name := "Training Grounds", cost := [pip .blue], types := [.enchantment],
    text :=
      [ static (costShift (allOf (and [abilityHead .anyActivated, abilityOf (allOf
          (and [isCreature, youControl]))]))
          (less 2 (some 1))) ] }

/-- Power Artifact -/
def powerArtifact : Spelled := spelled <| singleFacedCard
  { name := "Power Artifact", cost := [pip .blue, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isArtifact,
        static (costShift
          (allOf (and [abilityHead .anyActivated, abilityOf (enchanted artifact)]))
          (less 2 (some 1))) ] }

/-- Fervent Champion -/
def ferventChampionEquipDiscount : Ability :=
  static (costShift
    (allOf (and [abilityHead (.keyword equip), did you (activate which) thisObject,
        targeting thisCreature .someTarget]))
    (less 3))
theorem okFerventChampionEquipDiscount : Ability.check (DRS.ofList []) ferventChampionEquipDiscount = [] := by decide

def suppressionField : Spelled := spelled <| singleFacedCard
  { name := "Suppression Field", cost := [generic 1, pip .white], types := [.enchantment],
    text :=
      [ static (costShift (allOf (and [abilityHead .anyActivated, not isManaAbility]))
          (more 2)) ] }

def gloom : Spelled := spelled <| singleFacedCard
  { name := "Gloom", cost := [generic 2, pip .black], types := [.enchantment],
    text :=
      [ static (spellsCost (and [isSpell, hasColor .white]) (more 3)),
        static (costShift
          (allOf (and [abilityHead .anyActivated, abilityOf (allOf (and [isEnchantment, hasColor .white]))]))
          (more 3)) ] }

def bureauHeadmaster : Spelled := spelled <| singleFacedCard
  { name := "Bureau Headmaster", cost := [pip .red, pip .white], types := [.creature],
    subtypes := [human, assassin],
    text :=
      [ static (spellsCost (and [isSpell, equipment, did you (cast which) thisObject]) (less 1)),
        static (costShift (allOf (and [abilityHead (.keyword equip),
            did you (activate which) thisObject]))
            (less 1)) ],
    power := stat 2, toughness := stat 2 }

def oppressiveRaysLine : StaticSpec :=
  costShift (allOf (and [abilityHead .anyActivated, abilityOf (enchanted (.type
      .creature))]))
    (more 3)
theorem okOppressiveRaysLine : StaticSpec.check (DRS.ofList []) oppressiveRaysLine = [] := by decide

def eidolonOfObstruction : Spelled := spelled <| singleFacedCard
  { name := "Eidolon of Obstruction", cost := [generic 1, pip .white],
    types := [.enchantment, .creature], subtypes := [spirit],
    text :=
      [ firstStrike,
        static (costShift
          (allOf (and [ abilityHead .loyalty,
                         abilityOf (allOf (and [isPlaneswalker, opponentsControl])) ]))
          (more 1)) ],
    power := stat 2, toughness := stat 1 }

def forceOfWill : Spelled := spelled <| singleFacedCard
  { name := "Force of Will", cost := [generic 3, pip .blue, pip .blue], types := [.instant],
    text :=
      [ static (altCost this_
          (compound
            [ payLife 1,
              (exile (a (and [hasColor .blue, in_ yourHand]))) ])),
        spell (counter (target isSpell)) ] }

def demonOfDeathsGate : Spelled := spelled <| singleFacedCard
  { name := "Demon of Death's Gate", cost := [generic 6, pip .black, pip .black, pip .black],
    types := [.creature], subtypes := [demon],
    text :=
      [ static (altCost this_
          (compound
            [ payLife 6,
              (sacrifice (counted (exactly 3) (and [isCreature, hasColor .black]))) ])),
        flying, trample ],
    power := stat 9, toughness := stat 9 }

def drudgeSkeletons : Spelled := spelled <| singleFacedCard
  { name := "Drudge Skeletons", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [skeleton],
    text := [activated [pip .black] (regenerate thisCreature)],
    power := stat 1, toughness := stat 1 }

def asphodelWanderer : Spelled := spelled <| singleFacedCard
  { name := "Asphodel Wanderer", cost := [pip .black], types := [.creature],
    subtypes := [skeleton, soldier],
    text := [activated [generic 2, pip .black] (regenerate thisCreature)],
    power := stat 1, toughness := stat 1 }

def hurrJackalAbility : Ability :=
  activated tapSymbol
    (establishThroughout (cant any (regeneration (target isCreature))) thisTurn)
theorem okHurrJackalAbility : Ability.check (DRS.ofList []) hurrJackalAbility = [] := by decide

def wickedAkuba : Spelled := spelled <| singleFacedCard
  { name := "Wicked Akuba", cost := [pip .black, pip .black], types := [.creature],
    subtypes := [spirit],
    text :=
      [ activated [pip .black]
          (instruct
             (target
                (and
                   [ isPlayer,
                     did thisCreature (dealDamage anyAmount which) soFarThisTurn ]))
            (loseLife 1)) ],
    power := stat 2, toughness := stat 2 }

def idolOfOblivion : Spelled := spelled <| singleFacedCard
  { name := "Idol of Oblivion", cost := [generic 2], types := [.artifact],
    text :=
      [ activatedOnlyIf tapSymbol (draw 1)
          (did you (createObject anyAmount (token (copyOf (a isToken)) [])) soFarThisTurn),
        activated (compound [[generic 8], tapSymbol, sacrifice thisArtifact])
          (create 1 (creatureToken 10 10 [] [eldrazi])) ] }

def patricianGeist : Spelled := spelled <| singleFacedCard
  { name := "Patrician Geist", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [spirit, knight],
    text :=
      [ flying,
        static (getsPt (allOf (and [spirit, youControl, isOther]))
          (up 1) (up 1)),
        static (spellsCost (and [isSpell,
            did you (cast which (from_ := yourGraveyard)) thisObject]) (less 1)) ],
    power := stat 2, toughness := stat 2 }

def shatteredEgo : Spelled := spelled <| singleFacedCard
  { name := "Shattered Ego", cost := [pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (getsPt (enchanted creature) (down 3) (down 0)),
        activated [generic 3, pip .blue, pip .blue]
          (putInto (enchanted creature) (fromTop 3 library)) ] }

def shuFarmer : Spelled := spelled <| singleFacedCard
  { name := "Shu Farmer", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [human],
    text :=
      [ activatedOnlyDuring tapSymbol (gainLife 1) (beforePart
          .declareAttackers you) ],
    power := stat 1, toughness := stat 1 }

def elspethsTalent : Spelled := spelled <| singleFacedCard
  { name := "Elspeth's Talent", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isPlaneswalker,
        static elspethsTalentGrant,
        whenever (does you (activate loyaltyAbilityOfEnchanted))
          (establishFor (allOf (and [isCreature, youControl]))
            [getsPt them (up 2) (up 2), abilityGrant them vigilance]
            untilEndOfTurn) ] }

/-- Void Maw -/
def voidMawPutCost : Ability :=
  activated (putInto (a (in_ (exiledWith thisCreature))) graveyard)
    (get thisCreature (up 2) (up 2) untilEndOfTurn)
theorem okVoidMawPutCost : Ability.check (DRS.ofList []) voidMawPutCost = [] := by decide
/-- Ghor-Clan Rampager -/
def ghorClanRampager : Ability :=
  bloodrush
    (activated (compound [[pip .red, pip .green], discard this_])
      (establishFor (target (and [isCreature, isAttacking]))
        [getsPt it (up 4) (up 4), abilityGrant it trample]
        untilEndOfTurn))
theorem okGhorClanRampager : Ability.check (DRS.ofList []) ghorClanRampager = [] := by decide
/-- Tymora's Invoker -/
def tymorasInvoker : Ability :=
  flavorWord "Sleight of Hand" (activated [generic 8] (draw 2))
theorem okTymorasInvoker : Ability.check (DRS.ofList []) tymorasInvoker = [] := by decide
/-- Skyblade's Boon -/
def skybladesBoonReturn : Ability :=
  activatedOnlyIf [generic 2, pip .white] (returnToHand this_)
    (or [is this_ (in_ battlefield), is this_ (cardIn yourGraveyard)])
theorem okSkybladesBoonReturn : Ability.check (DRS.ofList []) skybladesBoonReturn = [] := by decide
/-- Bamboozling Beeble -/
def bamboozlingBeebleIgnore : Ability :=
  activated (compound [[generic 1], tapSymbol])
    (replaceNextEvent (would (target isPlayer) (rollDice anyAmount .anyDie))
      (instead (sequentially
        [ instruct they (rollDice (plus thatMuch 1) .thoseDice .anyResult),
          ignoreOutcomes (chosenResults you 1) ]))
      thisTurn)
theorem okBamboozlingBeebleIgnore : Ability.check (DRS.ofList []) bamboozlingBeebleIgnore = [] := by decide
/-- General Tazri's pump -/
def generalTazriPump : Ability :=
  activated [pip .white, pip .blue, pip .black, pip .red, pip .green]
    (sequentially
      [ get (each (and [isCreature, ally, youControl]))
          (up (letter .x)) (up (letter .x)) untilEndOfTurn,
        define .x (distinctCount .color (those creature)) ])
theorem okGeneralTazriPump : Ability.check (DRS.ofList []) generalTazriPump = [] := by decide
/-- Diplomatic Escort -/
def diplomaticEscortLine : Ability :=
  activated (compound [[pip .blue], tapSymbol, discard (a (cardIn yourHand))])
    (counter
      (target (and [or [isSpell, abilityHead .anyOnStack], targeting (a isCreature) .someTarget])))
theorem okDiplomaticEscortLine : Ability.check (DRS.ofList []) diplomaticEscortLine = [] := by decide

/-- Vorel of the Hull Clade -/
def vorelOfTheHullClade : Spelled := spelled <| singleFacedCard
  { name := "Vorel of the Hull Clade", cost := [generic 1, pip .green, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, merfolk],
    text :=
      [ activated (compound [[pip .green, pip .blue], tapSymbol])
          (doubleCounters (target (or [isArtifact, isCreature, isLand]))) ],
    power := stat 1, toughness := stat 4 }

/-- Prosperity -/
theorem prosperityCostLetters : costLetters [.variable, pip .blue] = [letterB .x] := rfl
def prosperityTextLetter : Amount := letter .x
theorem prosperityTextReadsCostLetter :
    let context := DRS.ofList (costLetters (some [.variable, pip .blue]))
    let out := (stepAmount 64 context prosperityTextLetter).1
    out.referents.filter (fun ref => ref.id ≥ context.nextId) = [] := by decide
theorem noVariableSymbolNoLetter : costLetters [generic 1, pip .blue] = [] := by decide
theorem noCostNoLetter : costLetters none = [] := by decide

/-- Sugar Coat -/
def sugarCoat : Spelled := spelled <| singleFacedCard
  { name := "Sugar Coat", cost := [generic 2, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ flash,
        enchant (or [isCreature, food]),
        static (conjunction [ qualityChange (enchanted permanent) .sets
              (bundle
                { characteristics :=
                  { types := [.artifact], subtypes := [food],
                    text :=
                      [ activated (compound [[generic 2], tapSymbol, (sacrifice
                          thisArtifact)])
                          (gainLife 3) ] } }),
            allAbilityLoss it ]) ] }

/-- Doc Aurlock, Grizzled Genius -/
def docAurlockCost : StaticSpec :=
  spellsCost
    (and [isSpell, did you (cast which (from_ := some (or [yourGraveyard, exileZone]))) thisObject])
    (less 2)
theorem okDocAurlockCost : StaticSpec.check (DRS.ofList []) docAurlockCost = [] := by decide
/-- Obelisk of Undoing -/
def obeliskOfUndoing : Ability :=
  activated (compound [[generic 6], tapSymbol])
    (returnToHand (target (and [isPermanent, youOwn, youControl])))
theorem okObeliskOfUndoing : Ability.check (DRS.ofList []) obeliskOfUndoing = [] := by decide
/-- Soul Conduit -/
def soulConduitExchange : Ability :=
  activated (compound [[generic 6], tapSymbol])
    (exchange (lifeTotals (targets (exactly 2) isPlayer)))
theorem okSoulConduitExchange : Ability.check (DRS.ofList []) soulConduitExchange = [] := by decide

/-- Death-Mask Duplicant -/
def deathMaskDuplicant : Spelled := spelled <| singleFacedCard
  { name := "Death-Mask Duplicant", cost := [generic 7], types := [.artifact, .creature],
    subtypes := [shapeshifter],
    text :=
      [ imprint
          (activated [generic 1] (exile (target (and [isCreature, in_ yourGraveyard])))),
        alsoForKeywords
          (fun k => static (onlyWhile
            (keywordGrant thisCreature k (and [in_ (exiledWith thisCreature), hasKeyword k]))
            (exists_ (and [in_ (exiledWith thisCreature), hasKeyword k]))))
          [ flying, fear, firstStrike, doubleStrike, haste, landwalkAbilities,
            protectionAbilities, trample ] ],
    power := stat 5, toughness := stat 5 }

/-- Darksteel Garrison -/
def darksteelGarrison : Spelled := spelled <| singleFacedCard
  { name := "Darksteel Garrison", cost := [generic 2], types := [.artifact],
    subtypes := [fortification],
    text :=
      [ static (abilityGrant (fortified land) indestructible),
        whenever (does (fortified land) (become tapped))
          (get (target isCreature) (up 1) (up 1) untilEndOfTurn),
        fortify [generic 3] ] }

/-- Embercleave -/
def embercleave : Spelled := spelled <| singleFacedCard
  { name := "Embercleave", cost := [generic 4, pip .red, pip .red], supertypes := [.legendary],
    types := [.artifact], subtypes := [equipment],
    text :=
      [ flash,
        static (costShift this_
          (less (forEach 1 (and [isCreature, isAttacking, youControl])))),
        when (does thisEquipment enter) (attachTo it (target (and [isCreature, youControl]))),
        static (conjunction [ getsPt (equipped creature) (up 1) (up
                1),
            abilityGrant (equipped creature) doubleStrike,
            abilityGrant (equipped creature) trample ] (subject :=
                                                                   some
                                                                      (equipped
                                                                         creature))),
        equip [generic 3] ] }

/-- Tattoo Ward -/
def tattooWard : Spelled := spelled <| singleFacedCard
  { name := "Tattoo Ward", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
          [ getsPt (enchanted creature) (up 1) (up 1),
            retention (abilityGrant (enchanted creature) (protection isEnchantment)) thisAura ]
          (subject := some (enchanted creature))),
        activated (sacrifice thisAura) (destroy (target isEnchantment)) ]
            }

/-- Floating Shield -/
def floatingShield : Spelled := spelled <| singleFacedCard
  { name := "Floating Shield", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (entersChoosing thisAura .color),
        static (retention
          (abilityGrant (enchanted creature) (protection
              (ofChosen .color)))
          thisAura),
        activated (sacrifice thisAura)
          (gain (target isCreature) (protection (ofChosen .color)) (some
              untilEndOfTurn)) ] }

/-- Flickering Ward -/
def flickeringWard : Spelled := spelled <| singleFacedCard
  { name := "Flickering Ward", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (entersChoosing thisAura .color),
        static (retention
          (abilityGrant (enchanted creature) (protection (ofChosen .color)))
          thisAura),
        activated [pip .white] (returnToHand thisAura) ] }

/-- Ghostfire Blade -/
def ghostfireBlade : Spelled := spelled <| singleFacedCard
  { name := "Ghostfire Blade", cost := [generic 1], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt (equipped creature) (up 2) (up 2)),
        equip [generic 3],
        static (costShift
          (allOf (and [ abilityHead (.keyword equip), abilityOf this_,
                         targeting (a (and [isCreature, isColorless])) .someTarget ]))
          (less 2)) ] }

/-- Academy Journeymage -/
def academyJourneymage : Spelled := spelled <| singleFacedCard
  { name := "Academy Journeymage", cost := [generic 4, pip .blue], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ static (onlyIfSo (costShift this_ (less 1))
          (exists_ (and [wizard, youControl]))),
        when (does thisCreature enter)
          (returnToHand (target (and [isCreature, anOpponentControls]))) ],
    power := stat 3, toughness := stat 2 }

/-- Alabaster Leech -/
def alabasterLeech : Spelled := spelled <| singleFacedCard
  { name := "Alabaster Leech", cost := [pip .white], types := [.creature],
    subtypes := [leech],
    text := [static (spellsCost (and [isSpell, hasColor .white, did you (cast which) thisObject])
        (moreBySymbols (symbols [pip .white])))],
    power := stat 1, toughness := stat 3 }

/-- Edgewalker -/
def edgewalker : Spelled := spelled <| singleFacedCard
  { name := "Edgewalker", cost := [generic 1, pip .white, pip .black], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ static (spellsCost (and [cleric, isSpell, did you (cast which) thisObject])
          (lessBySymbols (symbols [pip .white, pip .black]) (coloredOnly := true))) ],
    power := stat 2, toughness := stat 2 }

/-- Cavern-Hoard Dragon -/
def cavernHoardDragonRider : Ability :=
  static (conjunction [ costShift this_ (less (letter .x)),
      letterDefinition .x
        (aggregateOver .max isOpponent (countOf (and [isArtifact, controlledBy they]))) ])
theorem okCavernHoardDragonRider : Ability.check (DRS.ofList []) cavernHoardDragonRider = [] := by decide
/-- Shadowspear -/
def shadowspearStrip : Ability :=
  activated [generic 1]
    (establishThroughout (abilityLoss (allOf (and [isPermanent, opponentsControl]))
        [written hexproof, written
           indestructible]) untilEndOfTurn)
theorem okShadowspearStrip : Ability.check (DRS.ofList []) shadowspearStrip = [] := by decide
/-- Shay Cormac -/
def shayCormacStrip : Ability :=
  activated [generic 1]
    (establishThroughout (abilityLoss (allOf (and [isPermanent, opponentsControl]))
        [ written hexproof, written indestructible, abilities protectionAbilities,
          written shroud, abilities wardAbilities ]) untilEndOfTurn)
theorem okShayCormacStrip : Ability.check (DRS.ofList []) shayCormacStrip = [] := by decide

/-- Shelkin Brownie -/
def shelkinBrownie : Spelled := spelled <| singleFacedCard
  { name := "Shelkin Brownie", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [ouphe],
    text :=
      [ activated (compound [tapSymbol])
          (establishThroughout
             (abilityLoss (target isCreature)
                [abilities bandsWithOtherAbilities]) untilEndOfTurn) ],
    power := stat 1, toughness := stat 1 }

/-- Ahn-Crop Invader -/
def ahnCropInvader : Spelled := spelled <| singleFacedCard
  { name := "Ahn-Crop Invader", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [zombie, minotaur, warrior],
    text :=
      [ static (partScope .turn you (abilityGrant thisCreature firstStrike)),
        activated (compound [[generic 1], (sacrifice (a (and [isCreature,
            isOther])))])
          (get thisCreature (up 2) (up 0) untilEndOfTurn) ],
    power := stat 2, toughness := stat 2 }

/-- Nesting Dragon -/
def nestingDragonInnerToken : Ability :=
  activated [pip .red]
    (establishThroughout
       (conjunction [ getsPt thisToken (up 1) (up 0) ]) untilEndOfTurn)
theorem okNestingDragonInnerToken : Ability.check (DRS.ofList []) nestingDragonInnerToken = [] := by decide

/-- Leonin Bola -/
def leoninBola : Spelled := spelled <| singleFacedCard
  { name := "Leonin Bola", cost := [generic 1], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (abilityGrant (equipped creature)
          (activated (compound [tapSymbol, unattach (theGrantor .permanent)])
            (tap (target isCreature)))),
        equip [generic 1] ] }

/-- Alluring Suitor // Deadly Dancer -/
def deadlyDancerPump : Ability :=
  activated [pip .red, pip .red]
    (establishThroughout
       (getsPt (eachOf (both thisCreature (target (and [isCreature, isOther]))))
          (up 1) (up 0)) untilEndOfTurn)
theorem okDeadlyDancerPump : Ability.check (DRS.ofList []) deadlyDancerPump = [] := by decide

/-- Unesh, Criosphinx Sovereign -/
def uneshCriosphinxSovereign : Spelled := spelled <| singleFacedCard
  { name := "Unesh, Criosphinx Sovereign", cost := [generic 4, pip .blue, pip .blue],
    supertypes := [.legendary], types := [.creature], subtypes := [sphinx],
    text :=
      [ flying,
        static (spellsCost (and [sphinx, isSpell, did you (cast which) thisObject]) (less 2)),
        whenever
          (does (eitherOf thisCreature
            (a (and [sphinx, youControl, isOther]))) enter)
          (sequentially
            [ reveal (topSlice 4),
              instruct anOpponent (separateIntoPiles them 2),
              putInto onePile hand,
              putInto (theOther .pile) graveyard ]) ],
    power := stat 4, toughness := stat 4 }

def opt : Spelled := spelled <| singleFacedCard
  { name := "Opt", cost := [pip .blue], types := [.instant],
    text := [spell (sequentially [scry 1, draw 1])] }

def serumVisions : Spelled := spelled <| singleFacedCard
  { name := "Serum Visions", cost := [pip .blue], types := [.sorcery],
    text := [spell (sequentially [draw 1, scry 2])] }

def consider : Spelled := spelled <| singleFacedCard
  { name := "Consider", cost := [pip .blue], types := [.instant],
    text := [spell (sequentially [surveil 1, draw 1])] }

def wordsOfWisdom : Spelled := spelled <| singleFacedCard
  { name := "Words of Wisdom", cost := [generic 1, pip .blue], types := [.instant],
    text := [spell (sequentially [draw 2, instruct (each isOtherPlayer) (draw 1)])] }

def deathWard : Spelled := spelled <| singleFacedCard
  { name := "Death Ward", cost := [pip .white], types := [.instant],
    text := [spell (regenerate (target isCreature))] }

def bareTextLetter : Amount := letter .x
theorem textAloneOnceMintedItsOwnLetter :
    (stepAmount 64 (DRS.ofList []) bareTextLetter).1.referents = [letterB .x] := by decide

end Semantics.Cards
