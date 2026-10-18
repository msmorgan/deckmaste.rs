import Semantics
import Semantics.Definitions
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

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def forkedBolt : Action :=
  dealDivided Primitives.NounPhrase.this (.lit 2) (Primitives.NounPhrase.described (Primitives.DetPhrase.target (oneThrough 2)) isAnyTarget)
theorem okForkedBolt : Action.check (DRS.ofList []) forkedBolt = [] := by decide
def thoughtReflection : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.draw Primitives.Amount.anyAmount)) (Primitives.Action.draw (.lit 2)) none .repeatedly none)
theorem okThoughtReflection : Ability.check (DRS.ofList []) thoughtReflection = [] := by decide
def jorKadeen : Ability :=
  Primitives.Ability.static (onlyWhile (getsPt (allOf (and [isCreature, youControl])) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.up (.lit 0)))
    (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [isArtifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit 3)))
theorem okJorKadeen : Ability.check (DRS.ofList []) jorKadeen = [] := by decide
def abandonedOutpost : Ability := Primitives.Ability.static (entersTapped thisLand)
theorem okAbandonedOutpost : Ability.check (DRS.ofList []) abandonedOutpost = [] := by decide
/-- Time Vault -/
def timeVaultLock : Ability := Primitives.Ability.static (doesntUntap thisArtifact (some Primitives.NounPhrase.you))
theorem okTimeVaultLock : Ability.check (DRS.ofList []) timeVaultLock = [] := by decide
def hymnOfRebirth : Action :=
  putOntoBattlefieldUnderYourControl (target (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ graveyard]))
theorem okHymnOfRebirth : Action.check (DRS.ofList []) hymnOfRebirth = [] := by decide
def counterspell : Action := Action.counterSpell (target isSpell)
theorem okCounterspell : Action.check (DRS.ofList []) counterspell = [] := by decide

def anthemOfChampions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Anthem of Champions", cost := some [pip .green, pip .white], types := [.enchantment],
      text := [Primitives.Ability.static (getsPt (allOf (and [isCreature, youControl])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)))] } }

def adantoVanguard : Ability :=
  Primitives.Ability.static (onlyWhile (getsPt thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)))
    (Primitives.Condition.matches thisCreature isAttacking))
theorem okAdantoVanguard : Ability.check (DRS.ofList []) adantoVanguard = [] := by decide
/-- Nowhere to Run -/
def nowhereToRunTargetLine : StaticSpec :=
  canBeTargetedAsThough (allOf (and [isCreature, opponentsControl]))
    (allOf (Primitives.Predicate.or [isSpell, Primitives.Predicate.abilityHead .anyOnStack])) (Primitives.Predicate.not (Primitives.Predicate.hasKeyword (.the (.named "hexproof"))))
theorem okNowhereToRunTargetLine : StaticSpec.check (DRS.ofList []) nowhereToRunTargetLine = [] := by decide
/-- Hithlain Rope -/
def hithlainRopeSacrificeLock : StaticSpec := objectCant (.action "sacrifice") Primitives.NounPhrase.this
theorem okHithlainRopeSacrificeLock : StaticSpec.check (DRS.ofList []) hithlainRopeSacrificeLock = [] := by
  decide
/-- Lich's Mastery -/
def lichsMasteryGate : Ability := Primitives.Ability.static (playerCant (.core .loseGame) Primitives.NounPhrase.you)
theorem okLichsMasteryGate : Ability.check (DRS.ofList []) lichsMasteryGate = [] := by decide
def theGoldenThrone : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.conclude .loseGame)) (Primitives.Action.sequentially [exile thisArtifact, setLife (.lit 1)]) none .repeatedly none)
theorem okTheGoldenThrone : Ability.check (DRS.ofList []) theGoldenThrone = [] := by decide
def stunningReversal : Ability :=
  Primitives.Ability.spell none (Primitives.Action.establish
    (Primitives.StaticSpec.instead (Primitives.WouldClause.would (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.conclude .loseGame)) (Primitives.Action.sequentially [Primitives.Action.draw (.lit 7), setLife (.lit 1)]) none .nextTimeOnly none)
    (some thisTurn))
theorem okStunningReversal : Ability.check (DRS.ofList []) stunningReversal = [] := by decide
def pathOfBravery : Ability :=
  Primitives.Ability.static (onlyWhile (getsPt (allOf (and [isCreature, youControl])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)))
    (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .atLeast (Primitives.Amount.statOf (.playerStat .startingLifeTotal) Primitives.NounPhrase.you)))
theorem okPathOfBravery : Ability.check (DRS.ofList []) pathOfBravery = [] := by decide

def deathsShadow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death's Shadow", cost := some [pip .black], types := [.creature],
      subtypes := [creatureType "Avatar"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ getsPt thisCreature (Primitives.Delta.down (Primitives.Amount.letter .x)) (Primitives.Delta.down (Primitives.Amount.letter .x)),
              Primitives.StaticSpec.letterDefinition .x (lifeTotalOf Primitives.NounPhrase.you) ]) ],
      power := stat 13, toughness := stat 13 } }

def spontaneousMutation : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.down (Primitives.Amount.letter .x)) (Primitives.Delta.down (.lit 0)),
      Primitives.StaticSpec.letterDefinition .x (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])) ])
theorem okSpontaneousMutation : Ability.check (DRS.ofList []) spontaneousMutation = [] := by decide

def maro : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Maro", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])))] } }

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
theorem okScourgeOfTheSkyclaves : Ability.check (DRS.ofList []) scourgeOfTheSkyclaves = [] := by decide
def aettirAndPriwen : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ getsPt (Primitives.NounPhrase.attachHost .equipped (.type .creature)) (Primitives.Delta.set (Primitives.Amount.letter .x)) (Primitives.Delta.set (Primitives.Amount.letter .x)),
      Primitives.StaticSpec.letterDefinition .x (lifeTotalOf Primitives.NounPhrase.you) ])
theorem okAettirAndPriwen : Ability.check (DRS.ofList []) aettirAndPriwen = [] := by decide

def diminish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Diminish", cost := some [pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish (getsBase (target isCreature) (.lit 1) (.lit 1))
            (some untilEndOfTurn)) ] } }

def cycleOfLife : Action :=
  Primitives.Action.establish (getsBase (target (Primitives.Predicate.and [isCreature, castBy Primitives.NounPhrase.you])) (.lit 0) (.lit 1))
    (some (untilEvent (Primitives.GameEvent.beginningOf .upkeep (some Primitives.NounPhrase.you) (Primitives.TurnRef.next none))))
theorem okCycleOfLife : Action.check (DRS.ofList []) cycleOfLife = [] := by decide

def aboutFace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "About Face", cost := some [pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish (Primitives.StaticSpec.ptSwitch (target isCreature)) (some untilEndOfTurn)) ] } }

def roilingHorror : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .bothEach
    (minus (lifeTotalOf Primitives.NounPhrase.you)
      (lifeTotalOf
        (a (Primitives.Predicate.and [(Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you), Primitives.Predicate.superlative .max (.playerStat .lifeTotal) (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)])))))
theorem okRoilingHorror : Ability.check (DRS.ofList []) roilingHorror = [] := by decide

/-- Katara, the Fearless -/
def kataraTheFearless : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Katara, the Fearless", cost := some [pip .green, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Warrior", creatureType "Ally"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.additionalTriggers
            (Primitives.GameEvent.triggers (a (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyTriggered,
                         Primitives.Predicate.abilityOf (a (Primitives.Predicate.and [ Primitives.Predicate.hasSubtype (creatureType "Ally"),
                                               Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you ])) ])) none none)
            (exactly 1)) ],
      power := stat 3, toughness := stat 3 } }

/-- Rain of Gore -/
def rainOfGore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rain of Gore", cost := some [pip .black, pip .red], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead
      (Primitives.WouldClause.causedBy
        (Primitives.Causing.source (a (Primitives.Predicate.or [isSpell,
          Primitives.Predicate.abilityHead .anyOnStack])))
        (Primitives.WouldClause.would
          (Primitives.Subject.actor (controllerOf it))
          (Primitives.Action.changeLife (Primitives.Delta.up Primitives.Amount.anyAmount))))
      (instruct (that .player) (loseLife Primitives.Amount.thatMuch)) none .repeatedly none) ] } }

/-- Master Chef -/
def masterChefGrantedAbility : Ability :=
  Primitives.Ability.static (entersWithAdditionalCounters thisCreature (.lit 1) p1p1Counter)
theorem okMasterChefGrantedAbility : Ability.check (DRS.ofList []) masterChefGrantedAbility = [] := by decide

def anointedProcession : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Anointed Procession", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (Primitives.Action.createObject Primitives.Amount.anyAmount (Primitives.CreationSpec.token (Primitives.TokenSpec.copyOf (counted (atLeast 1) Primitives.Predicate.isToken) []) []) true (some Primitives.NounPhrase.you))) (Action.create (times (.lit 2) Primitives.Amount.groupSize) Primitives.TokenSpec.asThose []) none .repeatedly none) ] } }

def naturalAffinity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Natural Affinity", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.qualityChange (allOf isLand) .sets
              (Primitives.QualityPayload.bundle { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 } }
                (some .land)))
            (some untilEndOfTurn)) ] } }

def turnToFrog : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Turn to Frog", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.conjunction none
              [ Primitives.StaticSpec.allAbilityLoss (target isCreature) none,
                Primitives.StaticSpec.qualityChange it .sets
                  (Primitives.QualityPayload.bundle { characteristics := { colors := [.blue], subtypes := [creatureType "Frog"] } } none),
                getsPt it (Primitives.Delta.set (.lit 1)) (Primitives.Delta.set (.lit 1)) ])
            (some untilEndOfTurn)) ] } }

def humility : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Humility", cost := some [generic 2, pip .white, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.allAbilityLoss (allOf isCreature) none,
              getsPt them (Primitives.Delta.set (.lit 1)) (Primitives.Delta.set (.lit 1)) ]) ] } }

/-- Tahngarth, First Mate -/
def tahngarthAttacksThatJoin : Action := Primitives.Action.becomeAttacking thisCreature (some thatJoin)
theorem okTahngarthAttacksThatJoin :
    Action.check (nextClause ((stepAction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) tahngarthHeader).1) tahngarthChoosesDefender).1))
      tahngarthAttacksThatJoin = [] := by
  decide

def mindlockOrb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mindlock Orb", cost := some [generic 3, pip .blue], types := [.artifact],
      text := [Primitives.Ability.static (playerCant (.action "search") (allOf Primitives.Predicate.anyPlayer))] } }

def silence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Silence", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.establish (playerCant (.action "cast") (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)))
            (some thisTurn)) ] } }

def shadowOfDoubt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shadow of Doubt", cost := some [hybridPip .blue .black, hybridPip .blue .black],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.establish (playerCant (.action "search") (allOf Primitives.Predicate.anyPlayer))
                (some thisTurn),
              Primitives.Action.draw (.lit 1) ]) ] } }

def omenMachineDraw : Ability := Primitives.Ability.static (playerCant (.core .draw) (allOf Primitives.Predicate.anyPlayer))
theorem okOmenMachineDraw : Ability.check (DRS.ofList []) omenMachineDraw = [] := by decide
def solfataraLandLock : Action :=
  Primitives.Action.establish (playerCant (.action "play") (target Primitives.Predicate.anyPlayer)) (some thisTurn)
theorem okSolfataraLandLock : Action.check (DRS.ofList []) solfataraLandLock = [] := by decide

/-- "each nonbasic land is a <type>": a moon effect. -/
def nonbasicLandsAre (type : String) : StaticSpec :=
  Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [isLand, Primitives.Predicate.not (Primitives.Predicate.hasSupertype .basic)])) .sets
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
        [ Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf isLand) .adds
            (Primitives.QualityPayload.bundle { characteristics := { subtypes := [landType "Forest"] } } none)) ] } }

def rallyTheRanks : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rally the Ranks", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (getsPt
            (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, ofChosen (.subtype .creature)]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))) ] } }

def sharedTriumph : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shared Triumph", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [isCreature, ofChosen (.subtype .creature)]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))) ] } }

def hallOfTriumph : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hall of Triumph", cost := some [generic 3], supertypes := [.legendary],
      types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact .color),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, ofChosen .color]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))) ] } }

def engineeredPlague : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Engineered Plague", cost := some [generic 2, pip .black], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (entersChoosing thisEnchantment (.subtype .creature)),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [isCreature, ofChosen (.subtype .creature)]))
            (Primitives.Delta.down (.lit 1)) (Primitives.Delta.down (.lit 1))) ] } }

/-- Volrath's Laboratory -/
def volrathsLaboratoryChoice : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [entersChoosing thisArtifact .color, entersChoosing thisArtifact (.subtype .creature)]
theorem okVolrathsLaboratoryChoice : StaticSpec.check (DRS.ofList []) volrathsLaboratoryChoice = [] := by decide
/-- Call to Arms -/
def callToArmsChoice : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ entersChoosing thisEnchantment .color,
      entersChoosingPlayer thisEnchantment (some (Primitives.ChoiceDomain.players (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))) ]
theorem okCallToArmsChoice : StaticSpec.check (DRS.ofList []) callToArmsChoice = [] := by decide

def encroachingMycosynth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Encroaching Mycosynth", cost := some [generic 3, pip .blue], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.and [
              allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.not isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [Primitives.Predicate.or [isArtifact, Primitives.Predicate.hasType .battle, isCreature, isEnchantment, Primitives.Predicate.hasType .planeswalker], isSpell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, isPermanentCard, Primitives.Predicate.not isLand, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.in_ battlefield)]) ])
                .adds
              (Primitives.QualityPayload.bundle { characteristics := { types := [.artifact] } } none)) ] } }

/-- Darkest Hour -/
def darkestHour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darkest Hour", cost := some [pip .black], types := [.enchantment],
      text := [Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf isCreature) .sets (Primitives.QualityPayload.colored (.some [.black])))] } }

/-- Thran Lens -/
def thranLens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thran Lens", cost := some [generic 2], types := [.artifact],
      text := [Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf isPermanent) .sets (Primitives.QualityPayload.colored (.some [])))] } }

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
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.conjunction none
              [ Primitives.StaticSpec.qualityChange (allOf isCreature) .sets (Primitives.QualityPayload.colored (.some [.black])),
                Primitives.StaticSpec.qualityChange (allOf isLand) .sets
                  (Primitives.QualityPayload.bundle { characteristics := { subtypes := [landType "Swamp"] } } none) ])
            (some untilEndOfTurn)) ] } }

/-- Celestial Dawn -/
def celestialDawnAscriptions : List Ability :=
  [ Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .sets
      (Primitives.QualityPayload.bundle { characteristics := { subtypes := [landType "Plains"] } } none)),
    Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.and [
              allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.not isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [isSpell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.not isLand, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.in_ battlefield)]) ]) .sets
        (Primitives.QualityPayload.colored (.some [.white]))) ]
theorem okCelestialDawnAscriptions : Ability.checkText (DRS.ofList []) celestialDawnAscriptions = [] := by decide

/-- Transguild Courier -/
def transguildCourier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Transguild Courier", cost := some [generic 4], types := [.artifact, .creature],
      subtypes := [creatureType "Golem"],
      text := [Primitives.Ability.static (Primitives.StaticSpec.qualityChange thisCreature .sets (Primitives.QualityPayload.colored .every))],
      power := stat 3, toughness := stat 3 } }

/-- Booby Trap -/
def boobyTrapNameChoice : StaticSpec :=
  entersChoosingFrom thisArtifact .cardName (Primitives.ChoiceDomain.nameOfCard (Primitives.Predicate.not (Primitives.Predicate.and [Primitives.Predicate.hasSupertype .basic, isLand])))
theorem okBoobyTrapNameChoice : StaticSpec.check (DRS.ofList []) boobyTrapNameChoice = [] := by decide

def dovinsVeto : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dovin's Veto", cost := some [pip .white, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.static (objectCant (.action "counter") thisSpell),
          Primitives.Ability.spell none (Action.counterSpell (target (Primitives.Predicate.and [isSpell, Primitives.Predicate.not isCreature]))) ] } }

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
      text := [Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) Primitives.VisibleThing.wholeHand)] } }

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
        [ Primitives.Ability.static (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
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
        [ Primitives.Ability.spell none (Primitives.Action.establish
            (Primitives.StaticSpec.conjunction none
              [ getsPt (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) (Primitives.Delta.up
                  (.lit 2)) (Primitives.Delta.up (.lit 0)),
                Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
                  (Primitives.QualityPayload.everyTypeOf .creature) ])
            (some untilEndOfTurn)) ] } }

/-- Nameless Inversion -/
def namelessInversionBody : Action :=
  Primitives.Action.establish
    (Primitives.StaticSpec.conjunction none
      [ getsPt (target isCreature) (Primitives.Delta.up (.lit 3)) (Primitives.Delta.down (.lit 3)),
        Primitives.StaticSpec.qualityChange it .loses (Primitives.QualityPayload.everyTypeOf .creature) ])
    (some untilEndOfTurn)
theorem okNamelessInversionBody : Action.check (DRS.ofList []) namelessInversionBody = [] := by decide
/-- Ego Erasure -/
def egoErasureBody : Action :=
  Primitives.Action.establish
    (Primitives.StaticSpec.conjunction none
      [ getsPt (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller (target Primitives.Predicate.anyPlayer)])) (Primitives.Delta.down (.lit 2)) (Primitives.Delta.up (.lit 0)),
        Primitives.StaticSpec.qualityChange them .loses (Primitives.QualityPayload.everyTypeOf .creature) ])
    (some untilEndOfTurn)
theorem okEgoErasureBody : Action.check (DRS.ofList []) egoErasureBody = [] := by decide
/-- Lithoform Blight -/
def lithoformBlightLoss : StaticSpec :=
  Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .land)) .loses (Primitives.QualityPayload.everyTypeOf .land),
      Primitives.StaticSpec.allAbilityLoss it none ]
theorem okLithoformBlightLoss : StaticSpec.check (DRS.ofList []) lithoformBlightLoss = [] := by decide

/-- Energybending -/
def energybending : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Energybending", cost := some [generic 2], types := [.instant],
      subtypes := [spellType "Lesson"],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.establish
                (Primitives.StaticSpec.qualityChange (allOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .adds
                  (Primitives.QualityPayload.everyTypeOf .basicLand))
                (some untilEndOfTurn),
              Primitives.Action.draw (.lit 1) ]) ] } }

/-- Seedborn Muse -/
def seedbornMuse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seedborn Muse", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Spirit"],
      text :=
        [ Primitives.Ability.static (untapsDuring (allOf (Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you)) (some (each isOtherPlayer))) ],
      power := stat 2, toughness := stat 4 } }

/-- Unwinding Clock -/
def unwindingClock : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unwinding Clock", cost := some [generic 4], types := [.artifact],
      text :=
        [ Primitives.Ability.static (untapsDuring (allOf (Primitives.Predicate.and [isArtifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            (some (each isOtherPlayer))) ] } }

/-- Thousand Moons Infantry -/
def thousandMoonsInfantry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thousand Moons Infantry", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text := [Primitives.Ability.static (untapsDuring thisCreature (some (each isOtherPlayer)))],
      power := stat 2, toughness := stat 4 } }

/-- Blatant Thievery -/
def blatantThievery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blatant Thievery", cost := some [generic 4, pip .blue, pip .blue, pip .blue],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.doForEach (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))
            (Primitives.Action.establish
              (Primitives.StaticSpec.controlGrant Primitives.NounPhrase.you (target (Primitives.Predicate.hasPossessor .controller (that .player)))) none)) ] } }

/-- "activated abilities of <p> can't be activated" -/
def cantActivateAbilitiesOf (p : Predicate) : StaticSpec :=
  objectCant (.action "activate")
    (allOf (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyActivated, Primitives.Predicate.abilityOf (allOf p)]))

def cursedTotem : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cursed Totem", cost := some [generic 2], types := [.artifact],
      text := [Primitives.Ability.static (cantActivateAbilitiesOf isCreature)] } }

def nullRod : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Null Rod", cost := some [generic 2], types := [.artifact],
      text := [Primitives.Ability.static (cantActivateAbilitiesOf isArtifact)] } }

def stonySilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stony Silence", cost := some [generic 1, pip .white], types := [.enchantment],
      text := [Primitives.Ability.static (cantActivateAbilitiesOf isArtifact)] } }

def dampingMatrix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damping Matrix", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (objectCant (.action "activate")
            (allOf (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyActivated,
                           Primitives.Predicate.abilityOf (allOf (Primitives.Predicate.or [isArtifact, isCreature])),
                           Primitives.Predicate.not Primitives.Predicate.isManaAbility ]))) ] } }

def crash : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crash", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (sacrifice (a (Primitives.Predicate.and [isLand, Primitives.Predicate.hasSubtype (landType "Mountain")])))))),
          Primitives.Ability.spell none (destroy (target isArtifact)) ] } }

def moggSalvage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mogg Salvage", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this none)
            (Primitives.Condition.and
              [ exists_ (Primitives.Predicate.and [ isLand, Primitives.Predicate.hasSubtype (landType "Island"),
                                Primitives.Predicate.hasPossessor .controller anOpponent ]),
                exists_ (Primitives.Predicate.and [ isLand, Primitives.Predicate.hasSubtype (landType "Mountain"),
                                Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you ]) ])),
          Primitives.Ability.spell none (destroy (target isArtifact)) ] } }

def abolish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abolish", cost := some [generic 1, pip .white, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (discard (a (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (landType "Plains"), Primitives.Predicate.in_ yourHand])))))),
          Primitives.Ability.spell none (destroy (target (Primitives.Predicate.or [isArtifact, isEnchantment]))) ] } }

def gush : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gush", cost := some [generic 4, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (move
              (counted (exactly 2)
                (Primitives.Predicate.and [isLand, Primitives.Predicate.hasSubtype (landType "Island"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
              hand)))),
          Primitives.Ability.spell none (Primitives.Action.draw (.lit 2)) ] } }

def sunscour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sunscour", cost := some [generic 5, pip .white, pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (exile
              (counted (exactly 2) (Primitives.Predicate.and [Primitives.Predicate.colorIs .white, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])))))),
          Primitives.Ability.spell none (destroy (allOf isCreature)) ] } }

def massacre : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Massacre", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this none)
            (Primitives.Condition.and
              [ exists_ (Primitives.Predicate.and [ isLand, Primitives.Predicate.hasSubtype (landType "Plains"),
                                Primitives.Predicate.hasPossessor .controller anOpponent ]),
                exists_ (Primitives.Predicate.and [ isLand, Primitives.Predicate.hasSubtype (landType "Swamp"),
                                Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you ]) ])),
          Primitives.Ability.spell none (get (allOf isCreature) (Primitives.Delta.down (.lit 2)) (Primitives.Delta.down (.lit 2)) (some untilEndOfTurn))
              ] } }

def rouse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rouse", cost := some [generic 1, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this (some (payLife 2)))
            (exists_ (Primitives.Predicate.and [isLand, Primitives.Predicate.hasSubtype (landType "Swamp"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
          Primitives.Ability.spell none (get (target isCreature) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ]
              } }

def thwart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thwart", cost := some [generic 2, pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (move
              (counted (exactly 3)
                (Primitives.Predicate.and [isLand, Primitives.Predicate.hasSubtype (landType "Island"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
              hand)))),
          Primitives.Ability.spell none (Action.counterSpell (target isSpell)) ] } }

def theLadyOfOtariaLine : StaticSpec :=
  Primitives.StaticSpec.altCost Primitives.NounPhrase.this
    (some (Primitives.Cost.perform (Primitives.Action.setStatus .tapped
      (counted (exactly 3)
        (Primitives.Predicate.and [ isCreature, Primitives.Predicate.hasSubtype (creatureType "Dwarf"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                isUntapped ])))))
theorem okTheLadyOfOtariaLine : StaticSpec.check (DRS.ofList []) theLadyOfOtariaLine = [] := by decide

/-- Clergy of the Holy Nimbus -/
def clergyOfTheHolyNimbus : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.instead (Primitives.WouldClause.would Primitives.Subject.any (destroy (thisCreature))) (regenerationApplication it) none .repeatedly none)
theorem okClergyOfTheHolyNimbus : Ability.check (DRS.ofList []) clergyOfTheHolyNimbus = [] := by decide
/-- Rampant Frogantua -/
def rampantFrogantuaPump : Ability :=
  Primitives.Ability.static (getsPt thisCreature
    (Primitives.Delta.up (forEach 10 (Primitives.Predicate.and [Primitives.Predicate.anyPlayer,
      happenedTo (Primitives.Subject.actor (relative .object)) (Primitives.Action.conclude .loseGame) thisGame])))
    (Primitives.Delta.up (forEach 10 (Primitives.Predicate.and [Primitives.Predicate.anyPlayer,
      happenedTo (Primitives.Subject.actor (relative .object)) (Primitives.Action.conclude .loseGame) thisGame]))))
theorem okRampantFrogantuaPump : Ability.check (DRS.ofList []) rampantFrogantuaPump = [] := by decide
/-- Maskwood Nexus -/
def maskwoodNexusTypes : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.and [
              allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [isCreature, isSpell, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]),
              allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, isCreature, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you, Primitives.Predicate.not (Primitives.Predicate.in_ battlefield)]) ]) .adds (Primitives.QualityPayload.everyTypeOf
      .creature))
theorem okMaskwoodNexusTypes : Ability.check (DRS.ofList []) maskwoodNexusTypes = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorEquippedPermanent : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .equipped .permanent) .adds
    (Primitives.QualityPayload.bundle { characteristics := { types := [.creature] } } none))
theorem okLuxiorEquippedPermanent : Ability.check (DRS.ofList []) luxiorEquippedPermanent = [] := by decide
/-- Nahiri, the Unforgiving's compleated reminder -/
def nahiriCompleatedEntry : Ability :=
  Primitives.Ability.static (entersWithFewerCounters thisPlaneswalker (.lit 2) (.named "loyaltyCounter"))
theorem okNahiriCompleatedEntry : Ability.check (DRS.ofList []) nahiriCompleatedEntry = [] := by decide
/-- Nimble Mongoose -/
def nimbleMongoose : Ability :=
  abilityWord "threshold"
    (Primitives.Ability.static (onlyWhile (getsPt thisCreature (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)))
      (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])) .atLeast (.lit 7))))
theorem okNimbleMongoose : Ability.check (DRS.ofList []) nimbleMongoose = [] := by decide
/-- Anax, Hardened in the Forge -/
def anaxPowerDefinition : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .powerAlone (Primitives.Amount.devotion Primitives.NounPhrase.you (Primitives.ColorTerm.lit .red) none))
theorem okAnaxPowerDefinition : Ability.check (DRS.ofList []) anaxPowerDefinition = [] := by decide
/-- Aspect of Wolf -/
def aspectOfWolf : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (Primitives.Amount.letter .y)),
      Primitives.StaticSpec.letterDefinition .x (Primitives.Amount.half .down
        (countOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (landType "Forest"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
      Primitives.StaticSpec.letterDefinition .y (Primitives.Amount.half .up
        (countOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (landType "Forest"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) ])
theorem okAspectOfWolf : Ability.check (DRS.ofList []) aspectOfWolf = [] := by decide
/-- Lhurgoyf -/
def lhurgoyfDefinition : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.ptDefinition thisCreature .powerAlone (countOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.in_ graveyard])),
      Primitives.StaticSpec.ptDefinition thisCreature .toughnessAlone (plus Primitives.Amount.thatMuch (.lit 1)) ])
theorem okLhurgoyfDefinition : Ability.check (DRS.ofList []) lhurgoyfDefinition = [] := by decide

/-- Invigorate -/
def invigorate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Invigorate", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this (some (Primitives.Cost.perform (instruct anOpponent (gainLife (.lit 3))))))
            (exists_ (Primitives.Predicate.and [isLand, Primitives.Predicate.hasSubtype (landType "Forest"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))),
          Primitives.Ability.spell none (get (target isCreature) (Primitives.Delta.up (.lit 4)) (Primitives.Delta.up (.lit 4)) (some untilEndOfTurn)) ]
              } }

/-- Deflecting Swat -/
def deflectingSwatCommanderAltCost : Ability :=
  Primitives.Ability.static (onlyWhile (Primitives.StaticSpec.altCost Primitives.NounPhrase.this none)
    (exists_ (Primitives.Predicate.and [Primitives.Predicate.hasDesignation (.named "commander") none, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
theorem okDeflectingSwatCommanderAltCost :
    Ability.check (DRS.ofList []) deflectingSwatCommanderAltCost = [] := by decide

/-- Fist of Suns -/
def fistOfSuns : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fist of Suns", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost (allOf (Primitives.Predicate.and [isSpell, castBy Primitives.NounPhrase.you]))
            (some (Primitives.Cost.mana [pip .white, pip .blue, pip .black, pip .red, pip .green]))) ] } }

/-- Rooftop Storm -/
def rooftopStorm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rooftop Storm", cost := some [generic 5, pip .blue], types := [.enchantment],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost
            (allOf (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Zombie"), isCreature, isSpell, castBy Primitives.NounPhrase.you]))
            (some (Primitives.Cost.mana [generic 0]))) ] } }

/-- Voltage Surge's declaration -/
def voltageSurgeAddedCost : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.addedCost (Primitives.Cost.perform (sacrifice (a isArtifact))) true .once)
theorem okVoltageSurgeAddedCost : Ability.check (DRS.ofList []) voltageSurgeAddedCost = [] := by decide
/-- Tarmogoyf -/
def tarmogoyfDefinition : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.ptDefinition thisCreature .powerAlone
        (Primitives.Amount.distinctCount .cardType (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf (allOf Primitives.Predicate.anyPlayer))]))),
      Primitives.StaticSpec.ptDefinition thisCreature .toughnessAlone (plus Primitives.Amount.thatMuch (.lit 1)) ])
theorem okTarmogoyfDefinition : Ability.check (DRS.ofList []) tarmogoyfDefinition = [] := by decide
/-- Consuming Blob's definition -/
def consumingBlobDefinition : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.ptDefinition thisCreature .powerAlone
        (Primitives.Amount.distinctCount .cardType (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)]))),
      Primitives.StaticSpec.ptDefinition thisCreature .toughnessAlone (plus Primitives.Amount.thatMuch (.lit 1)) ])
theorem okConsumingBlobDefinition : Ability.check (DRS.ofList []) consumingBlobDefinition = [] := by decide
/-- Nighthawk Scavenger's definition -/
def nighthawkScavengerDefinition : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .powerAlone
    (plus (.lit 1)
      (Primitives.Amount.distinctCount .cardType (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)))])))))
theorem okNighthawkScavengerDefinition : Ability.check (DRS.ofList []) nighthawkScavengerDefinition = [] := by
  decide
/-- Faeburrow Elder's pump -/
def faeburrowElderPump : Ability :=
  Primitives.Ability.static (getsPt thisCreature
    (Primitives.Delta.up (times (.lit 1)
      (Primitives.Amount.distinctCount .color (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))))
    (Primitives.Delta.up (times (.lit 1)
      (Primitives.Amount.distinctCount .color (allOf (Primitives.Predicate.and [isPermanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))))
theorem okFaeburrowElderPump : Ability.check (DRS.ofList []) faeburrowElderPump = [] := by decide
/-- Bonds of Faith -/
def bondsOfFaithPump : Ability :=
  Primitives.Ability.static (onlyWhile
    (getsPt (Primitives.NounPhrase.attachHost .enchanted (.type .creature)) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)))
    (Primitives.Condition.matches it (Primitives.Predicate.hasSubtype (creatureType "Human"))))
theorem okBondsOfFaithPump : Ability.check (DRS.ofList []) bondsOfFaithPump = [] := by decide

/-- Field of Dreams -/
def fieldOfDreams : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Field of Dreams", cost := some [pip .blue], supertypes := [.world],
      types := [.enchantment],
      text := [Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (allOf Primitives.Predicate.anyPlayer) Primitives.VisibleThing.topOfLibrary)] } }

/-- Lantern of Insight -/
def lanternOfInsightRider : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (allOf Primitives.Predicate.anyPlayer) Primitives.VisibleThing.topOfLibrary)
theorem okLanternOfInsightRider : Ability.check (DRS.ofList []) lanternOfInsightRider = [] := by decide

/-- Revelation -/
def revelation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Revelation", cost := some [pip .green], supertypes := [.world], types := [.enchantment],
      text := [Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal (allOf Primitives.Predicate.anyPlayer) Primitives.VisibleThing.wholeHand)] } }

/-- Phyrexian Unlife -/
def phyrexianUnlifeImmunity : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.cant (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.conclude .loseGame (some .nonpositiveLife)) {})
theorem okPhyrexianUnlifeImmunity : Ability.check (DRS.ofList []) phyrexianUnlifeImmunity = [] := by decide
/-- Umbris, Fear Manifest -/
def umbrisPump : Ability :=
  Primitives.Ability.static (getsPt thisCreature
    (Primitives.Delta.up (forEach 1
      (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.hasPossessor .owner (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)), Primitives.Predicate.in_ exileZone])))
    (Primitives.Delta.up (forEach 1
      (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.hasPossessor .owner (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)), Primitives.Predicate.in_ exileZone]))))
theorem okUmbrisPump : Ability.check (DRS.ofList []) umbrisPump = [] := by decide
/-- Turbulent Fen -/
def turbulentFen : Ability :=
  Primitives.Ability.static (onlyUnless (entersTapped thisLand)
    (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))]))
      .atLeast (.lit 8)))
theorem okTurbulentFen : Ability.check (DRS.ofList []) turbulentFen = [] := by decide
/-- Bastion Protector -/
def bastionProtectorPump : Ability :=
  Primitives.Ability.static (getsPt
    (allOf (Primitives.Predicate.and [isCreature, Primitives.Predicate.hasDesignation (.named "commander") none, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
    (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)))
theorem okBastionProtectorPump : Ability.check (DRS.ofList []) bastionProtectorPump = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorTypeSetting : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.conjunction none
    [ Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .equipped .permanent) .loses
        (Primitives.QualityPayload.bundle { characteristics := { types := [.planeswalker] } } none),
      Primitives.StaticSpec.qualityChange it .adds (Primitives.QualityPayload.bundle { characteristics := { types := [.creature] } } none) ])
theorem okLuxiorTypeSetting : Ability.check (DRS.ofList []) luxiorTypeSetting = [] := by decide
/-- Kasmina, Enigma Sage -/
def kasminaLoyaltySharing : StaticSpec :=
  Primitives.StaticSpec.abilityGrantFrom
    (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan Primitives.NounPhrase.this]))
    [.loyalty] Primitives.NounPhrase.this none
theorem okKasminaLoyaltySharing : StaticSpec.check (DRS.ofList []) kasminaLoyaltySharing = [] := by decide
/-- Nicol Bolas, Dragon-God -/
def nicolBolasDragonGodSharing : StaticSpec :=
  Primitives.StaticSpec.abilityGrantFrom Primitives.NounPhrase.this [.loyalty]
    (allOf (Primitives.Predicate.and [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.otherThan Primitives.NounPhrase.this, Primitives.Predicate.in_ battlefield])) none
theorem okNicolBolasDragonGodSharing : StaticSpec.check (DRS.ofList []) nicolBolasDragonGodSharing = [] := by
  decide
/-- Myr Welder -/
def myrWelderBorrowedAbilities : StaticSpec :=
  Primitives.StaticSpec.abilityGrantFrom thisCreature [.anyActivated] (allOf (Primitives.Predicate.in_ (Primitives.Location.exiledWith Primitives.NounPhrase.this))) none
theorem okMyrWelderBorrowedAbilities : StaticSpec.check (DRS.ofList []) myrWelderBorrowedAbilities = [] := by
  decide
/-- Sharkey, Tyrant of the Shire -/
def sharkeyBorrowedLandAbilities : StaticSpec :=
  Primitives.StaticSpec.abilityGrantFrom Primitives.NounPhrase.this [.anyActivated]
    (allOf (Primitives.Predicate.and [isLand, Primitives.Predicate.hasPossessor .controller (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))]))
    (some Primitives.Predicate.isManaAbility)
theorem okSharkeyBorrowedLandAbilities : StaticSpec.check (DRS.ofList []) sharkeyBorrowedLandAbilities = [] := by
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
            (Primitives.Condition.matches (topSlice (.lit 1)) (Primitives.Predicate.or [isArtifact, isCreature]))) ],
      power := stat 1, toughness := stat 3 } }

/-- Emissary of Grudges -/
def emissaryOfGrudgesEntry : StaticSpec :=
  entersChoosingPlayerSecretly thisCreature (some (Primitives.ChoiceDomain.players (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)))
theorem okEmissaryOfGrudgesEntry : StaticSpec.check (DRS.ofList []) emissaryOfGrudgesEntry = [] := by decide

def aerialAssault : Action := destroy (target (Primitives.Predicate.and [isCreature, isTapped]))
theorem okAerialAssault : Action.check (DRS.ofList []) aerialAssault = [] := by decide
def asphyxiate : Action := destroy (target (Primitives.Predicate.and [isCreature, isUntapped]))
theorem okAsphyxiate : Action.check (DRS.ofList []) asphyxiate = [] := by decide
def vindicate : Action := destroy (target isPermanent)
theorem okVindicate : Action.check (DRS.ofList []) vindicate = [] := by decide

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
      text := [Primitives.Ability.spell none (destroy (target (Primitives.Predicate.and [isCreature, isBlocked])))] } }

def eerieUltimatum : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eerie Ultimatum",
      cost := some [pip .white, pip .white, pip .black, pip .black, pip .black, pip .green, pip .green],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (move
            (withDifferentNames
              (counted anyNumber (Primitives.Predicate.and [isPermanentCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)])))
            battlefield) ] } }

/-- Gray Merchant of Asphodel -/
def grayMerchantDrain : Action :=
  Primitives.Action.sequentially
    [instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (loseLife (Primitives.Amount.letter .x)), Primitives.Action.define .x (Primitives.Amount.devotion Primitives.NounPhrase.you (Primitives.ColorTerm.lit .black)
        none)]
theorem okGrayMerchantDrain : Action.check (DRS.ofList []) grayMerchantDrain = [] := by decide

end Semantics.Cards
