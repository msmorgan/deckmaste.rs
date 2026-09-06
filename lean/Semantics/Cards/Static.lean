import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Choice
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Static

Port of `idris/src/Experimental/Cards/Static.idr`: the printed cards of the Static family
(continuous clauses: pumps, definitions, ascriptions, locks, alternative costs, visibility) and
the bench items beside them.

`maro`, `peopleOfTheWoods` and `cultivatorColossus` print `*` boxes: those slots are `none`
under the printed-star ruling, and the characteristic-defining clauses set them.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

def forkedBolt : Instruction :=
  dealDivided Primitives.NounPhrase.this (.lit 2) (Primitives.NounPhrase.described (Primitives.DetPhrase.target (oneThrough 2)) anyTarget)
theorem okForkedBolt : Instruction.check [] forkedBolt = [] := by decide
def thoughtReflection : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.draws Primitives.NounPhrase.you) [] none (Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you)) .repeatedly none)
theorem okThoughtReflection : Ability.check [] thoughtReflection = [] := by decide
def jorKadeen : Ability :=
  Primitives.Ability.static (onlyWhile (getsPt (allOf creatureYouControl) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 0)))
    (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit 3)))
theorem okJorKadeen : Ability.check [] jorKadeen = [] := by decide
def abandonedOutpost : Ability := Primitives.Ability.static (entersTapped thisLand)
theorem okAbandonedOutpost : Ability.check [] abandonedOutpost = [] := by decide
/-- Time Vault -/
def timeVaultLock : Ability := Primitives.Ability.static (doesntUntap thisArtifact (some Primitives.NounPhrase.you))
theorem okTimeVaultLock : Ability.check [] timeVaultLock = [] := by decide
def hymnOfRebirth : Instruction :=
  putOntoBattlefieldUnderYourControl (target (Primitives.Predicate.and [creature, Primitives.Predicate.inZone graveyard]))
theorem okHymnOfRebirth : Instruction.check [] hymnOfRebirth = [] := by decide
def counterspell : Instruction := Primitives.Instruction.counterSpell (target spell)
theorem okCounterspell : Instruction.check [] counterspell = [] := by decide

def anthemOfChampions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Anthem of Champions", cost := some [pip .green, pip .white], types := [.enchantment],
      text := [Primitives.Ability.static (getsPt (allOf creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)))] } }

def adantoVanguard : Ability :=
  Primitives.Ability.static (onlyWhile (getsPt thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)))
    (Primitives.Condition.matches thisCreature attacking))
theorem okAdantoVanguard : Ability.check [] adantoVanguard = [] := by decide
/-- Nowhere to Run -/
def nowhereToRunTargetLine : StaticSpec :=
  canBeTargetedAsThough (allOf creatureYourOpponentsControl)
    (allOf (Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack])) (Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the "Hexproof")))
theorem okNowhereToRunTargetLine : StaticSpec.check [] nowhereToRunTargetLine = [] := by decide
/-- Hithlain Rope -/
def hithlainRopeSacrificeLock : StaticSpec := objectCant (.action "Sacrifice") Primitives.NounPhrase.this
theorem okHithlainRopeSacrificeLock : StaticSpec.check [] hithlainRopeSacrificeLock = [] := by
  decide
/-- Lich's Mastery -/
def lichsMasteryGate : Ability := Primitives.Ability.static (playerCant (.core .loseGame) Primitives.NounPhrase.you)
theorem okLichsMasteryGate : Ability.check [] lichsMasteryGate = [] := by decide
def theGoldenThrone : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.losesGame Primitives.NounPhrase.you) [] none
    (Primitives.Instruction.sequentially [exile thisArtifact, setLife (.lit 1) (agent := Primitives.NounPhrase.you)]) .repeatedly none)
theorem okTheGoldenThrone : Ability.check [] theGoldenThrone = [] := by decide
def stunningReversal : Ability :=
  Primitives.Ability.spell none (Primitives.Instruction.establish
    (Primitives.StaticSpec.replacement (Primitives.GameEvent.losesGame Primitives.NounPhrase.you) [] none
      (Primitives.Instruction.sequentially [Primitives.Instruction.draw (.lit 7) (agent := Primitives.NounPhrase.you), setLife (.lit 1) (agent := Primitives.NounPhrase.you)]) .nextTimeOnly
          none)
    (some Primitives.Duration.thisTurn))
theorem okStunningReversal : Ability.check [] stunningReversal = [] := by decide
def pathOfBravery : Ability :=
  Primitives.Ability.static (onlyWhile (getsPt (allOf creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)))
    (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .atLeast (Primitives.Amount.statOf (.playerStat .startingLifeTotal) Primitives.NounPhrase.you)))
theorem okPathOfBravery : Ability.check [] pathOfBravery = [] := by decide

def deathsShadow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death's Shadow", cost := some [pip .black], types := [.creature],
      subtypes := [creatureType "Avatar"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification thisCreature .power (Primitives.Delta.down (Primitives.Amount.letter .x)),
              Primitives.StaticSpec.modification thisCreature .toughness (Primitives.Delta.down (Primitives.Amount.letter .x)),
              Primitives.StaticSpec.letterDefinition .x (lifeTotalOf Primitives.NounPhrase.you) ]) ],
      power := stat 13, toughness := stat 13 } }

def spontaneousMutation : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .power (Primitives.Delta.down (Primitives.Amount.letter .x)),
      Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .toughness (Primitives.Delta.down (.lit 0)),
      Primitives.StaticSpec.letterDefinition .x (countOf (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you))) ])
theorem okSpontaneousMutation : Ability.check [] spontaneousMutation = [] := by decide

def maro : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Maro", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))))] } }

def peopleOfTheWoods : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "People of the Woods", cost := some [pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Human"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .toughnessAlone
            (countOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (landType "Forest"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) ],
      power := stat 1 } }

def scourgeOfTheSkyclaves : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach
    (minus (.lit 20) (aggregate .max (.playerStat .lifeTotal) Primitives.Predicate.anyPlayer)))
theorem okScourgeOfTheSkyclaves : Ability.check [] scourgeOfTheSkyclaves = [] := by decide
def aettirAndPriwen : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .power (Primitives.Delta.set (Primitives.Amount.letter .x)),
      Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .equipped (.type .creature)) .toughness (Primitives.Delta.set (Primitives.Amount.letter .x)),
      Primitives.StaticSpec.letterDefinition .x (lifeTotalOf Primitives.NounPhrase.you) ])
theorem okAettirAndPriwen : Ability.check [] aettirAndPriwen = [] := by decide

def diminish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Diminish", cost := some [pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish (getsBase (target creature) (.lit 1) (.lit 1))
            (some untilEndOfTurn)) ] } }

def cycleOfLife : Instruction :=
  Primitives.Instruction.establish (getsBase (target (Primitives.Predicate.and [creature, castBy Primitives.NounPhrase.you])) (.lit 0) (.lit 1))
    (some (Primitives.Duration.until_ (Primitives.DurationEnd.startOf .upkeep (some Primitives.NounPhrase.you))))
theorem okCycleOfLife : Instruction.check [] cycleOfLife = [] := by decide

def aboutFace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "About Face", cost := some [pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish (Primitives.StaticSpec.ptSwitch (target creature)) (some untilEndOfTurn)) ] } }

def roilingHorror : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach
    (minus (lifeTotalOf Primitives.NounPhrase.you)
      (lifeTotalOf
        (a (Primitives.Predicate.and [Primitives.Predicate.opponent, Primitives.Predicate.superlative .max (.playerStat .lifeTotal) Primitives.Predicate.opponent])))))
theorem okRoilingHorror : Ability.check [] roilingHorror = [] := by decide

/-- Katara, the Fearless -/
def kataraTheFearless : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Katara, the Fearless", cost := some [pip .green, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior", creatureType "Ally"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.additionalTriggers
            (Primitives.GameEvent.triggers
              (a (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyTriggered,
                         Primitives.Predicate.abilityOf (a (Primitives.Predicate.and [ Primitives.Predicate.hasSubtype (creatureType "Ally"),
                                               Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you ])) ])))
            (exactly 1)) ],
      power := stat 3, toughness := stat 3 } }

/-- Rain of Gore -/
def rainOfGore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rain of Gore", cost := some [pip .black, pip .red], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement
            (Primitives.GameEvent.causes (Primitives.Causing.source (a (Primitives.Predicate.or [spell, Primitives.Predicate.abilityHead .anyOnStack])))
              (Primitives.GameEvent.lifeChanges (controllerOf it) .up))
            [] none (loseLife Primitives.Amount.thatMuch (agent := (that .player))) .repeatedly none) ] } }

/-- Master Chef -/
def masterChefGrantedAbility : Ability :=
  Primitives.Ability.static (entersWithAdditionalCounters thisCreature (.lit 1) plusOnePlusOne)
theorem okMasterChefGrantedAbility : Ability.check [] masterChefGrantedAbility = [] := by decide

def anointedProcession : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Anointed Procession", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement (tokensCreatedByEffectUnder (counted (atLeast 1) Primitives.Predicate.isToken) Primitives.NounPhrase.you)
            [] none (Primitives.Instruction.create (times (.lit 2) Primitives.Amount.groupSize) Primitives.TokenSpec.asThose [] (agent := Primitives.NounPhrase.you)) .repeatedly
                none) ] } }

def naturalAffinity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Natural Affinity", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.qualityChange (allOf land) .sets
              (Primitives.QualityPayload.bundle { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 } }
                (some .land)))
            (some untilEndOfTurn)) ] } }

def turnToFrog : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Turn to Frog", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.conjunction none
              [ Primitives.StaticSpec.allAbilityLoss (target creature) none,
                Primitives.StaticSpec.qualityChange it .sets
                  (Primitives.QualityPayload.bundle { characteristics := { colors := [.blue], subtypes := [creatureType "Frog"] } } none),
                Primitives.StaticSpec.modification it .power (Primitives.Delta.set (.lit 1)),
                Primitives.StaticSpec.modification (itsOther it (Primitives.Delta.set (.lit 1))) .toughness (Primitives.Delta.set (.lit 1)) ])
            (some untilEndOfTurn)) ] } }

def humility : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Humility", cost := some [generic 2, pip .white, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.allAbilityLoss (allOf creature) none,
              Primitives.StaticSpec.modification them .power (Primitives.Delta.set (.lit 1)),
              Primitives.StaticSpec.modification (itsOther them (Primitives.Delta.set (.lit 1))) .toughness (Primitives.Delta.set (.lit 1)) ]) ] } }

/-- Tahngarth, First Mate -/
def tahngarthAttacksThatJoin : Instruction := Primitives.Instruction.becomeAttacking thisCreature (some thatJoin)
theorem okTahngarthAttacksThatJoin :
    Instruction.check (Instruction.intro (GameEvent.intro [] tahngarthHeader) tahngarthChoosesDefender)
      tahngarthAttacksThatJoin = [] := by
  decide

def mindlockOrb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mindlock Orb", cost := some [generic 3, pip .blue], types := [.artifact],
      text := [Primitives.Ability.static (playerCant (.action "Search") (Primitives.NounPhrase.playerGroup .allPlayers))] } }

def silence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Silence", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish (playerCant (.action "Cast") (Primitives.NounPhrase.playerGroup .yourOpponents))
            (some Primitives.Duration.thisTurn)) ] } }

def shadowOfDoubt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shadow of Doubt", cost := some [hybridPip .blue .black, hybridPip .blue .black],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.establish (playerCant (.action "Search") (Primitives.NounPhrase.playerGroup .allPlayers))
                (some Primitives.Duration.thisTurn),
              Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you) ]) ] } }

def omenMachineDraw : Ability := Primitives.Ability.static (playerCant (.core .draw) (Primitives.NounPhrase.playerGroup .allPlayers))
theorem okOmenMachineDraw : Ability.check [] omenMachineDraw = [] := by decide
def solfataraLandLock : Instruction :=
  Primitives.Instruction.establish (playerCant (.action "Play") (target Primitives.Predicate.anyPlayer)) (some Primitives.Duration.thisTurn)
theorem okSolfataraLandLock : Instruction.check [] solfataraLandLock = [] := by decide

/-- "each nonbasic land is a <type>": a moon effect. -/
def nonbasicLandsAre (type : String) : StaticSpec :=
  Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [land, Primitives.Predicate.not (Primitives.Predicate.hasSupertype .basic)])) .sets
    (Primitives.QualityPayload.bundle { characteristics := { subtypes := [landType type] } } none)

def bloodMoon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Moon", cost := some [generic 2, pip .red], types := [.enchantment],
      text := [Primitives.Ability.static (nonbasicLandsAre "Mountain")] } }

def magusOfTheMoon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magus of the Moon", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text := [Primitives.Ability.static (nonbasicLandsAre "Mountain")], power := stat 2, toughness := stat 2 } }

def harbingerOfTheSeas : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harbinger of the Seas", cost := some [generic 1, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text := [Primitives.Ability.static (nonbasicLandsAre "Island")], power := stat 2, toughness := stat 2 } }

def yavimayaCradleOfGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yavimaya, Cradle of Growth", supertypes := [.legendary], types := [.land],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf land) .adds
            (Primitives.QualityPayload.bundle { characteristics := { subtypes := [landType "Forest"] } } none)) ] } }

def rallyTheRanks : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rally the Ranks", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (getsPt
            (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, ofChosen (.subtype .creature)]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))) ] } }

def sharedTriumph : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shared Triumph", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [creature, ofChosen (.subtype .creature)]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))) ] } }

def hallOfTriumph : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hall of Triumph", cost := some [generic 3], supertypes := [.legendary],
      types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact .color),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, ofChosen .color]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))) ] } }

def engineeredPlague : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Engineered Plague", cost := some [generic 2, pip .black], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [creature, ofChosen (.subtype .creature)]))
            (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1))) ] } }

/-- Volrath's Laboratory -/
def volrathsLaboratoryChoice : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [entersChoosing thisArtifact .color, entersChoosing thisArtifact (.subtype .creature)]
theorem okVolrathsLaboratoryChoice : StaticSpec.check [] volrathsLaboratoryChoice = [] := by decide
/-- Call to Arms -/
def callToArmsChoice : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ entersChoosing thisEnchantment .color,
      entersChoosingPlayer thisEnchantment (some (Primitives.ChoiceDomain.players Primitives.Predicate.opponent)) ]
theorem okCallToArmsChoice : StaticSpec.check [] callToArmsChoice = [] := by decide

def encroachingMycosynth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Encroaching Mycosynth", cost := some [generic 3, pip .blue], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.offBattlefieldScope
            (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.not land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
                .adds
              (Primitives.QualityPayload.bundle { characteristics := { types := [.artifact] } } none))) ] } }

/-- Darkest Hour -/
def darkestHour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darkest Hour", cost := some [pip .black], types := [.enchantment],
      text := [Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf creature) .sets (Primitives.QualityPayload.colored (.some [.black])))] } }

/-- Thran Lens -/
def thranLens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thran Lens", cost := some [generic 2], types := [.artifact],
      text := [Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf permanent) .sets (Primitives.QualityPayload.colored (.some [])))] } }

/-- Ghostflame Sliver -/
def ghostflameSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostflame Sliver", cost := some [pip .black, pip .red], types := [.creature],
      subtypes := [creatureType "Sliver"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.hasSubtype (creatureType "Sliver"))) .sets (Primitives.QualityPayload.colored
            (.some []))) ],
      power := stat 2, toughness := stat 2 } }

/-- Nightcreep -/
def nightcreep : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nightcreep", cost := some [pip .black, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.conjunction none
              [ Primitives.StaticSpec.qualityChange (allOf creature) .sets (Primitives.QualityPayload.colored (.some [.black])),
                Primitives.StaticSpec.qualityChange (allOf land) .sets
                  (Primitives.QualityPayload.bundle { characteristics := { subtypes := [landType "Swamp"] } } none) ])
            (some untilEndOfTurn)) ] } }

/-- Celestial Dawn -/
def celestialDawnAscriptions : List Ability :=
  [ Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .sets
      (Primitives.QualityPayload.bundle { characteristics := { subtypes := [landType "Plains"] } } none)),
    Primitives.Ability.static (Primitives.StaticSpec.offBattlefieldScope
      (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.not land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .sets
        (Primitives.QualityPayload.colored (.some [.white])))) ]
theorem okCelestialDawnAscriptions : Ability.checkText [] celestialDawnAscriptions = [] := by decide

/-- Transguild Courier -/
def transguildCourier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Transguild Courier", cost := some [generic 4], types := [.artifact, .creature],
      subtypes := [creatureType "Golem"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.qualityChange thisCreature .sets (Primitives.QualityPayload.colored .every))],
      power := stat 3, toughness := stat 3 } }

/-- Booby Trap -/
def boobyTrapNameChoice : StaticSpec :=
  entersChoosingFrom thisArtifact .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not (Primitives.Predicate.and [Primitives.Predicate.hasSupertype .basic, land])))
theorem okBoobyTrapNameChoice : StaticSpec.check [] boobyTrapNameChoice = [] := by decide

def dovinsVeto : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dovin's Veto", cost := some [pip .white, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.static (objectCant (.action "Counter") Primitives.NounPhrase.this),
          Primitives.Ability.spell none (Primitives.Instruction.counterSpell (target (Primitives.Predicate.and [spell, Primitives.Predicate.not creature]))) ] } }

def goblinSpy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Goblin Spy", cost := some [pip .red], types := [.creature],
      subtypes := [creatureType "Goblin", creatureType "Rogue"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary)],
      power := stat 1, toughness := stat 1 } }

def futureSight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Future Sight", cost := some [generic 2, pip .blue, pip .blue, pip .blue],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static playLandsAndCastSpellsFromTop ] } }

def magusOfTheFuture : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magus of the Future", cost := some [generic 2, pip .blue, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static playLandsAndCastSpellsFromTop ],
      power := stat 2, toughness := stat 3 } }

def telepathy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Telepathy", cost := some [pip .blue], types := [.enchantment],
      text := [Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (Primitives.NounPhrase.playerGroup .yourOpponents) Primitives.VisibleThing.wholeHand)] } }

def assembleThePlayers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Assemble the Players", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .lookAt Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static castSmallCreatureFromTopOnceEachTurn ] } }

def prismaticOmen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismatic Omen", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
            (Primitives.QualityPayload.everyTypeOf .basicLand)) ] } }

def mistformUltimus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistform Ultimus", cost := some [generic 3, pip .blue], supertypes := [.legendary],
      types := [.creature], subtypes := [creatureType "Illusion"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.qualityChange thisCreature .adds (Primitives.QualityPayload.everyTypeOf .creature))],
      power := stat 3, toughness := stat 3 } }

def volatileClaws : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Volatile Claws", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.establish
            (Primitives.StaticSpec.conjunction none
              [ Primitives.StaticSpec.modification (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .power (Primitives.Delta.up
                  (.lit 2)),
                Primitives.StaticSpec.modification (itsOther (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
                    (Primitives.Delta.up (.lit 2)))
                  .toughness (Primitives.Delta.up (.lit 0)),
                Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
                  (Primitives.QualityPayload.everyTypeOf .creature) ])
            (some untilEndOfTurn)) ] } }

/-- Nameless Inversion -/
def namelessInversionBody : Instruction :=
  Primitives.Instruction.establish
    (Primitives.StaticSpec.conjunction none
      [ Primitives.StaticSpec.modification (target creature) .power (Primitives.Delta.up (.lit 3)),
        Primitives.StaticSpec.modification (itsOther (target creature) (Primitives.Delta.up (.lit 3))) .toughness (Primitives.Delta.down (.lit 3)),
        Primitives.StaticSpec.qualityChange it .loses (Primitives.QualityPayload.everyTypeOf .creature) ])
    (some untilEndOfTurn)
theorem okNamelessInversionBody : Instruction.check [] namelessInversionBody = [] := by decide
/-- Ego Erasure -/
def egoErasureBody : Instruction :=
  Primitives.Instruction.establish
    (Primitives.StaticSpec.conjunction none
      [ Primitives.StaticSpec.modification (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (target Primitives.Predicate.anyPlayer)]))
          .power
          (Primitives.Delta.down (.lit 2)),
        Primitives.StaticSpec.modification (itsOther (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (target
            Primitives.Predicate.anyPlayer)]))
            (Primitives.Delta.down (.lit 2)))
          .toughness (Primitives.Delta.up (.lit 0)),
        Primitives.StaticSpec.qualityChange them .loses (Primitives.QualityPayload.everyTypeOf .creature) ])
    (some untilEndOfTurn)
theorem okEgoErasureBody : Instruction.check [] egoErasureBody = [] := by decide
/-- Lithoform Blight -/
def lithoformBlightLoss : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .land)) .loses (Primitives.QualityPayload.everyTypeOf .land),
      Primitives.StaticSpec.allAbilityLoss it none ]
theorem okLithoformBlightLoss : StaticSpec.check [] lithoformBlightLoss = [] := by decide

/-- Energybending -/
def energybending : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Energybending", cost := some [generic 2], types := [.instant],
      subtypes := [spellType "Lesson"],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.establish
                (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
                  (Primitives.QualityPayload.everyTypeOf .basicLand))
                (some untilEndOfTurn),
              Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you) ]) ] } }

/-- Seedborn Muse -/
def seedbornMuse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seedborn Muse", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Spirit"],
      text :=
        [ Primitives.Ability.static (untapsDuring (allOf (Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you)) (some (each otherPlayer))) ],
      power := stat 2, toughness := stat 4 } }

/-- Unwinding Clock -/
def unwindingClock : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unwinding Clock", cost := some [generic 4], types := [.artifact],
      text :=
        [ Primitives.Ability.static (untapsDuring (allOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            (some (each otherPlayer))) ] } }

/-- Thousand Moons Infantry -/
def thousandMoonsInfantry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thousand Moons Infantry", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [Primitives.Ability.static (untapsDuring thisCreature (some (each otherPlayer)))],
      power := stat 2, toughness := stat 4 } }

/-- Blatant Thievery -/
def blatantThievery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blatant Thievery", cost := some [generic 4, pip .blue, pip .blue, pip .blue],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.doForEach (each Primitives.Predicate.opponent)
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.controlGrant Primitives.NounPhrase.you (target (Primitives.Predicate.hasPossessor .controller (that .player)))) none)) ] } }

/-- "activated abilities of <p> can't be activated" -/
def cantActivateAbilitiesOf (p : Predicate) : StaticSpec :=
  objectCant (.action "Activate")
    (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf (allOf p)]))

def cursedTotem : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cursed Totem", cost := some [generic 2], types := [.artifact],
      text := [Primitives.Ability.static (cantActivateAbilitiesOf creature)] } }

def nullRod : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Null Rod", cost := some [generic 2], types := [.artifact],
      text := [Primitives.Ability.static (cantActivateAbilitiesOf artifact)] } }

def stonySilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stony Silence", cost := some [generic 1, pip .white], types := [.enchantment],
      text := [Primitives.Ability.static (cantActivateAbilitiesOf artifact)] } }

def dampingMatrix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damping Matrix", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (objectCant (.action "Activate")
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.or [artifact, creature])),
                           Primitives.Predicate.not Primitives.Predicate.isManaAbility ]))) ] } }

def crash : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crash", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (sacrifice (a (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Mountain")])) (agent
                := Primitives.NounPhrase.you))))),
          Primitives.Ability.spell none (destroy (target artifact)) ] } }

def moggSalvage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mogg Salvage", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this none)
            (Primitives.Condition.and
              [ exists_ (Primitives.Predicate.and [ land, Primitives.Predicate.hasSubtype (landType "Island"),
                                Primitives.Predicate.hasPossessor .controller anOpponent ]),
                exists_ (Primitives.Predicate.and [ land, Primitives.Predicate.hasSubtype (landType "Mountain"),
                                Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you ]) ])),
          Primitives.Ability.spell none (destroy (target artifact)) ] } }

def abolish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abolish", cost := some [generic 1, pip .white, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (discard (a (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (landType "Plains"), Primitives.Predicate.inZone hand]))
                (agent := Primitives.NounPhrase.you))))),
          Primitives.Ability.spell none (destroy (target (Primitives.Predicate.or [artifact, enchantment]))) ] } }

def gush : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gush", cost := some [generic 4, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (move
              (counted (exactly 2)
                (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Island"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
              hand)))),
          Primitives.Ability.spell none (Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you)) ] } }

def sunscour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sunscour", cost := some [generic 5, pip .white, pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (exile
              (counted (exactly 2) (Primitives.Predicate.and [Primitives.Predicate.colorIs .white, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])) (agent := some
                  Primitives.NounPhrase.you))))),
          Primitives.Ability.spell none (destroy (allOf creature)) ] } }

def massacre : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Massacre", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this none)
            (Primitives.Condition.and
              [ exists_ (Primitives.Predicate.and [ land, Primitives.Predicate.hasSubtype (landType "Plains"),
                                Primitives.Predicate.hasPossessor .controller anOpponent ]),
                exists_ (Primitives.Predicate.and [ land, Primitives.Predicate.hasSubtype (landType "Swamp"),
                                Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you ]) ])),
          Primitives.Ability.spell none (get (allOf creature) (Primitives.Delta.down (.lit 2)) (Primitives.Delta.down (.lit 2)) (some untilEndOfTurn))
              ] } }

def rouse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rouse", cost := some [generic 1, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this (some (payLife Primitives.NounPhrase.you 2)))
            (exists_ (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Swamp"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
          Primitives.Ability.spell none (get (target creature) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ]
              } }

def thwart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thwart", cost := some [generic 2, pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (move
              (counted (exactly 3)
                (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Island"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
              hand)))),
          Primitives.Ability.spell none (Primitives.Instruction.counterSpell (target spell)) ] } }

def theLadyOfOtariaLine : StaticSpec :=
  Primitives.StaticSpec.altCost Primitives.NounPhrase.this
    (some (Primitives.Cost.perform (Primitives.Instruction.setStatus .tapped
      (counted (exactly 3)
        (Primitives.Predicate.and [ creature, Primitives.Predicate.hasSubtype (creatureType "Dwarf"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                untapped ])))))
theorem okTheLadyOfOtariaLine : StaticSpec.check [] theLadyOfOtariaLine = [] := by decide

/-- Clergy of the Holy Nimbus -/
def clergyOfTheHolyNimbus : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.verbedEvent
    none (.action "Destroy") (some thisCreature) none none) [] none
    (Primitives.Instruction.regenerate it) .repeatedly none)
theorem okClergyOfTheHolyNimbus : Ability.check [] clergyOfTheHolyNimbus = [] := by decide
/-- Rampant Frogantua -/
def rampantFrogantuaPump : Ability :=
  Primitives.Ability.static (getsPt thisCreature
    (Primitives.Delta.up (forEach 10 (Primitives.Predicate.and [Primitives.Predicate.anyPlayer,
      happenedTo (Primitives.GameEvent.losesGame (relative .player)) .thisGame])))
    (Primitives.Delta.up (forEach 10 (Primitives.Predicate.and [Primitives.Predicate.anyPlayer,
      happenedTo (Primitives.GameEvent.losesGame (relative .player)) .thisGame]))))
theorem okRampantFrogantuaPump : Ability.check [] rampantFrogantuaPump = [] := by decide
/-- Maskwood Nexus -/
def maskwoodNexusTypes : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.offBattlefieldScope (Primitives.StaticSpec.qualityChange (allOf creatureYouControl) .adds (Primitives.QualityPayload.everyTypeOf
      .creature)))
theorem okMaskwoodNexusTypes : Ability.check [] maskwoodNexusTypes = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorEquippedPermanent : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .equipped .permanent) .adds
    (Primitives.QualityPayload.bundle { characteristics := { types := [.creature] } } none))
theorem okLuxiorEquippedPermanent : Ability.check [] luxiorEquippedPermanent = [] := by decide
/-- Nahiri, the Unforgiving's compleated reminder -/
def nahiriCompleatedEntry : Ability :=
  Primitives.Ability.static (entersWithFewerCounters thisPlaneswalker (.lit 2) (.named "Loyalty"))
theorem okNahiriCompleatedEntry : Ability.check [] nahiriCompleatedEntry = [] := by decide
/-- Nimble Mongoose -/
def nimbleMongoose : Ability :=
  abilityWord "threshold"
    (Primitives.Ability.static (onlyWhile (getsPt thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)))
      (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you))) .atLeast (.lit 7))))
theorem okNimbleMongoose : Ability.check [] nimbleMongoose = [] := by decide
/-- Anax, Hardened in the Forge -/
def anaxPowerDefinition : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .powerAlone (Primitives.Amount.devotion Primitives.NounPhrase.you (Primitives.ColorTerm.lit .red) none))
theorem okAnaxPowerDefinition : Ability.check [] anaxPowerDefinition = [] := by decide
/-- Aspect of Wolf -/
def aspectOfWolf : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .power (Primitives.Delta.up (Primitives.Amount.letter .x)),
      Primitives.StaticSpec.modification (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) .toughness (Primitives.Delta.up (Primitives.Amount.letter .y)),
      Primitives.StaticSpec.letterDefinition .x (Primitives.Amount.half .down
        (countOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (landType "Forest"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
      Primitives.StaticSpec.letterDefinition .y (Primitives.Amount.half .up
        (countOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (landType "Forest"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) ])
theorem okAspectOfWolf : Ability.check [] aspectOfWolf = [] := by decide
/-- Lhurgoyf -/
def lhurgoyfDefinition : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.ptDefinition thisCreature .powerAlone (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.inZone graveyard])),
      Primitives.StaticSpec.ptDefinition thisCreature .toughnessAlone (plus Primitives.Amount.thatMuch (.lit 1)) ])
theorem okLhurgoyfDefinition : Ability.check [] lhurgoyfDefinition = [] := by decide

/-- Invigorate -/
def invigorate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Invigorate", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this (some (Primitives.Cost.perform (gainLife (.lit 3) (agent :=
            anOpponent)))))
            (exists_ (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Forest"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
          Primitives.Ability.spell none (get (target creature) (Primitives.Delta.up (.lit 4)) (Primitives.Delta.up (.lit 4)) (some untilEndOfTurn)) ]
              } }

/-- Deflecting Swat -/
def deflectingSwatCommanderAltCost : Ability :=
  Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this none)
    (exists_ (Primitives.Predicate.and [Primitives.Predicate.hasDesignation "commander" none, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
theorem okDeflectingSwatCommanderAltCost :
    Ability.check [] deflectingSwatCommanderAltCost = [] := by decide

/-- Fist of Suns -/
def fistOfSuns : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fist of Suns", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost (allOf (Primitives.Predicate.and [spell, castBy Primitives.NounPhrase.you]))
            (some (Primitives.Cost.mana [pip .white, pip .blue, pip .black, pip .red, pip .green]))) ] } }

/-- Rooftop Storm -/
def rooftopStorm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rooftop Storm", cost := some [generic 5, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost
            (allOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Zombie"), creature, spell, castBy Primitives.NounPhrase.you]))
            (some (Primitives.Cost.mana [generic 0]))) ] } }

/-- Voltage Surge's declaration -/
def voltageSurgeAddedCost : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.addedCost (Primitives.Cost.perform (sacrifice (a artifact) (agent := Primitives.NounPhrase.you))) true)
theorem okVoltageSurgeAddedCost : Ability.check [] voltageSurgeAddedCost = [] := by decide
/-- Tarmogoyf -/
def tarmogoyfDefinition : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.ptDefinition thisCreature .powerAlone
        (Primitives.Amount.distinctCount .cardType (allOf (Primitives.Predicate.inZone (graveyardOf (Primitives.NounPhrase.playerGroup .allPlayers))))),
      Primitives.StaticSpec.ptDefinition thisCreature .toughnessAlone (plus Primitives.Amount.thatMuch (.lit 1)) ])
theorem okTarmogoyfDefinition : Ability.check [] tarmogoyfDefinition = [] := by decide
/-- Consuming Blob's definition -/
def consumingBlobDefinition : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.ptDefinition thisCreature .powerAlone
        (Primitives.Amount.distinctCount .cardType (allOf (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)))),
      Primitives.StaticSpec.ptDefinition thisCreature .toughnessAlone (plus Primitives.Amount.thatMuch (.lit 1)) ])
theorem okConsumingBlobDefinition : Ability.check [] consumingBlobDefinition = [] := by decide
/-- Nighthawk Scavenger's definition -/
def nighthawkScavengerDefinition : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .powerAlone
    (plus (.lit 1)
      (Primitives.Amount.distinctCount .cardType (allOf (Primitives.Predicate.inZone (graveyardOf (Primitives.NounPhrase.playerGroup .yourOpponents)))))))
theorem okNighthawkScavengerDefinition : Ability.check [] nighthawkScavengerDefinition = [] := by
  decide
/-- Faeburrow Elder's pump -/
def faeburrowElderPump : Ability :=
  Primitives.Ability.static (getsPt thisCreature
    (Primitives.Delta.up (times (.lit 1)
      (Primitives.Amount.distinctCount .color (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))))
    (Primitives.Delta.up (times (.lit 1)
      (Primitives.Amount.distinctCount .color (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))))
theorem okFaeburrowElderPump : Ability.check [] faeburrowElderPump = [] := by decide
/-- Bonds of Faith -/
def bondsOfFaithPump : Ability :=
  Primitives.Ability.static (onlyWhile
    (getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)))
    (Primitives.Condition.matches it (Primitives.Predicate.hasSubtype (creatureType "Human"))))
theorem okBondsOfFaithPump : Ability.check [] bondsOfFaithPump = [] := by decide

/-- Field of Dreams -/
def fieldOfDreams : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Field of Dreams", cost := some [pip .blue], supertypes := [.world],
      types := [.enchantment],
      text := [Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (Primitives.NounPhrase.playerGroup .allPlayers) Primitives.VisibleThing.topOfLibrary)] } }

/-- Lantern of Insight -/
def lanternOfInsightRider : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (Primitives.NounPhrase.playerGroup .allPlayers) Primitives.VisibleThing.topOfLibrary)
theorem okLanternOfInsightRider : Ability.check [] lanternOfInsightRider = [] := by decide

/-- Revelation -/
def revelation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Revelation", cost := some [pip .green], supertypes := [.world], types := [.enchantment],
      text := [Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (Primitives.NounPhrase.playerGroup .allPlayers) Primitives.VisibleThing.wholeHand)] } }

/-- Phyrexian Unlife -/
def phyrexianUnlifeImmunity : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.deonticRule Primitives.NounPhrase.you Primitives.Compulsion.forbid [.core .loseGame] .agent none Primitives.DeonticPatient.noPatient none
    (Primitives.DeonticRider.stateBased .nonpositiveLife))
theorem okPhyrexianUnlifeImmunity : Ability.check [] phyrexianUnlifeImmunity = [] := by decide
/-- Umbris, Fear Manifest -/
def umbrisPump : Ability :=
  Primitives.Ability.static (getsPt thisCreature
    (Primitives.Delta.up (forEach 1
      (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.hasPossessor .owner (Primitives.NounPhrase.playerGroup .yourOpponents), Primitives.Predicate.inZone exileZone])))
    (Primitives.Delta.up (forEach 1
      (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.hasPossessor .owner (Primitives.NounPhrase.playerGroup .yourOpponents), Primitives.Predicate.inZone exileZone]))))
theorem okUmbrisPump : Ability.check [] umbrisPump = [] := by decide
/-- Turbulent Fen -/
def turbulentFen : Ability :=
  Primitives.Ability.static (onlyUnless (entersTapped thisLand)
    (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup .yourOpponents)]))
      .atLeast (.lit 8)))
theorem okTurbulentFen : Ability.check [] turbulentFen = [] := by decide
/-- Bastion Protector -/
def bastionProtectorPump : Ability :=
  Primitives.Ability.static (getsPt
    (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasDesignation "commander" none, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
    (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)))
theorem okBastionProtectorPump : Ability.check [] bastionProtectorPump = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorTypeSetting : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .equipped .permanent) .loses
        (Primitives.QualityPayload.bundle { characteristics := { types := [.planeswalker] } } none),
      Primitives.StaticSpec.qualityChange it .adds (Primitives.QualityPayload.bundle { characteristics := { types := [.creature] } } none) ])
theorem okLuxiorTypeSetting : Ability.check [] luxiorTypeSetting = [] := by decide
/-- Kasmina, Enigma Sage -/
def kasminaLoyaltySharing : StaticSpec :=
  Primitives.StaticSpec.abilityGrantFrom
    (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan Primitives.NounPhrase.this]))
    [.loyalty] Primitives.NounPhrase.this none
theorem okKasminaLoyaltySharing : StaticSpec.check [] kasminaLoyaltySharing = [] := by decide
/-- Nicol Bolas, Dragon-God -/
def nicolBolasDragonGodSharing : StaticSpec :=
  Primitives.StaticSpec.abilityGrantFrom Primitives.NounPhrase.this [.loyalty]
    (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.otherThan Primitives.NounPhrase.this, Primitives.Predicate.inZone battlefield])) none
theorem okNicolBolasDragonGodSharing : StaticSpec.check [] nicolBolasDragonGodSharing = [] := by
  decide
/-- Myr Welder -/
def myrWelderBorrowedAbilities : StaticSpec :=
  Primitives.StaticSpec.abilityGrantFrom thisCreature [.anyActivated] (allOf (Primitives.Predicate.exiledWith Primitives.NounPhrase.this)) none
theorem okMyrWelderBorrowedAbilities : StaticSpec.check [] myrWelderBorrowedAbilities = [] := by
  decide
/-- Sharkey, Tyrant of the Shire -/
def sharkeyBorrowedLandAbilities : StaticSpec :=
  Primitives.StaticSpec.abilityGrantFrom Primitives.NounPhrase.this [.anyActivated]
    (allOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller (Primitives.NounPhrase.playerGroup .yourOpponents)]))
    (some Primitives.Predicate.isManaAbility)
theorem okSharkeyBorrowedLandAbilities : StaticSpec.check [] sharkeyBorrowedLandAbilities = [] := by
  decide

/-- Skill Borrower -/
def skillBorrower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Skill Borrower", cost := some [generic 2, pip .blue], types := [.artifact, .creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static (onlyWhile
            (Primitives.StaticSpec.abilityGrantFrom thisCreature [.anyActivated] (topSlice (.lit 1)) none)
            (Primitives.Condition.matches (topSlice (.lit 1)) (Primitives.Predicate.or [artifact, creature]))) ],
      power := stat 1, toughness := stat 3 } }

/-- Emissary of Grudges -/
def emissaryOfGrudgesEntry : StaticSpec :=
  entersChoosingPlayerSecretly thisCreature (some (Primitives.ChoiceDomain.players Primitives.Predicate.opponent))
theorem okEmissaryOfGrudgesEntry : StaticSpec.check [] emissaryOfGrudgesEntry = [] := by decide

def aerialAssault : Instruction := destroy (target (Primitives.Predicate.and [creature, tapped]))
theorem okAerialAssault : Instruction.check [] aerialAssault = [] := by decide
def asphyxiate : Instruction := destroy (target (Primitives.Predicate.and [creature, untapped]))
theorem okAsphyxiate : Instruction.check [] asphyxiate = [] := by decide
def vindicate : Instruction := destroy (target permanent)
theorem okVindicate : Instruction.check [] vindicate = [] := by decide

def counterspellCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Counterspell", cost := some [pip .blue, pip .blue], types := [.instant],
      text := [Primitives.Ability.spell none counterspell] } }

def hymnOfRebirthCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hymn of Rebirth", cost := some [generic 3, pip .green, pip .white],
      types := [.sorcery], text := [Primitives.Ability.spell none hymnOfRebirth] } }

def chandrasPyrohelixCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chandra's Pyrohelix", cost := some [generic 1, pip .red], types := [.instant],
      text := [Primitives.Ability.spell none forkedBolt] } }

/-- Smite -/
def smite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Smite", cost := some [pip .white], types := [.instant],
      text := [Primitives.Ability.spell none (destroy (target (Primitives.Predicate.and [creature, blocked])))] } }

def eerieUltimatum : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eerie Ultimatum",
      cost := some [pip .white, pip .white, pip .black, pip .black, pip .black, pip .green, pip .green],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (move
            (withDifferentNames
              (counted anyNumber (Primitives.Predicate.and [permanentCard, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])))
            battlefield) ] } }

/-- Gray Merchant of Asphodel -/
def grayMerchantDrain : Instruction :=
  Primitives.Instruction.sequentially
    [loseLife (Primitives.Amount.letter .x) (agent := (each Primitives.Predicate.opponent)), Primitives.Instruction.define .x (Primitives.Amount.devotion Primitives.NounPhrase.you (Primitives.ColorTerm.lit .black)
        none)]
theorem okGrayMerchantDrain : Instruction.check [] grayMerchantDrain = [] := by decide

end Semantics.Cards
