import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Description

Port of `idris/src/Experimental/Cards/Description.idr`: the printed cards of the Description
family, and the phrase-level bench items beside them (a noun, predicate, condition,
instruction, or ability that elaborated in Idris), each paired with an `ok…` theorem that runs
the checker on it at the bindings its Idris type named.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Glyph of Destruction -/
def glyphOfDestruction : Action :=
  get (target (Primitives.Predicate.and [blocking, Primitives.Predicate.hasSubtype (creatureType "Wall"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
    (Primitives.Delta.up (.lit 10)) (Primitives.Delta.up (.lit 0)) (some untilEndOfCombat)
theorem okGlyphOfDestruction : Action.check (DRS.ofList []) glyphOfDestruction = [] := by decide

def rawNonattacking : Predicate := Primitives.Predicate.and [creature, Primitives.Predicate.not attacking, Primitives.Predicate.not blocking]
theorem okRawNonattacking : Predicate.checkAt .object (DRS.ofList []) rawNonattacking = [] := by decide

/-- Harmony of Nature -/
def harmonyOfNature : Action :=
  Primitives.Action.sequentially
    [ tap (counted anyNumber (Primitives.Predicate.and [untapped, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])),
      Primitives.Action.doForEach (theVerbed (.action "tap") .many (some (.type .creature)) .thisWay)
        (gainLife (.lit 4)) ]
theorem okHarmonyOfNature : Action.check (DRS.ofList []) harmonyOfNature = [] := by decide

/-- Rats of Rath -/
def ratsOfRath : Action :=
  destroy (target (Primitives.Predicate.and [Primitives.Predicate.or [artifact, creature, land], Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
theorem okRatsOfRath : Action.check (DRS.ofList []) ratsOfRath = [] := by decide

/-- "another creature or land", after a target on the battlefield. -/
def anotherDisjunctPhrase : Predicate := Primitives.Predicate.and [Primitives.Predicate.or [creature, land], Primitives.Predicate.other]
theorem okAnotherDisjunctPhrase :
    Predicate.checkAt .object (DRS.ofList [{ det := .target, plur := .one, payload := .object [] (some .battlefield) none none none }]) anotherDisjunctPhrase = [] := by
  decide

/-- Defeat -/
def defeat : Action :=
  destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .atMost (.lit 2)]))
theorem okDefeat : Action.check (DRS.ofList []) defeat = [] := by decide

/-- Terashi's Verdict -/
def terashisVerdict : Action :=
  destroy (target (Primitives.Predicate.and [creature, attacking, Primitives.Predicate.compare [.stat .power] .atMost (.lit 3)]))
theorem okTerashisVerdict : Action.check (DRS.ofList []) terashisVerdict = [] := by decide

/-- Pillar of Light -/
def pillarOfLight : Action :=
  exile (target (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .toughness] .atLeast (.lit 4)]))
theorem okPillarOfLight : Action.check (DRS.ofList []) pillarOfLight = [] := by decide

/-- Unholy Annex -/
def unholyAnnex : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.draw (.lit 1),
      Primitives.Action.doIf (exists_ (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Demon"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
        (Primitives.Action.sequentially [instruct (each (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (loseLife (.lit 2)), gainLife (.lit 2)])
        (some (loseLife (.lit 2))) ]
theorem okUnholyAnnex : Action.check (DRS.ofList []) unholyAnnex = [] := by decide

/-- War Screecher -/
def warScreecher : Action :=
  get (allOf (otherCreatureYouControl thisCreature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))
    (some untilEndOfTurn)
theorem okWarScreecher : Action.check (DRS.ofList []) warScreecher = [] := by decide

/-- Mind Flayer -/
def mindFlayer : Action :=
  gainControl (target creature)
    (some (Primitives.Duration.forAsLongAs (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you)))) (agent := Primitives.NounPhrase.you)
theorem okMindFlayer : Action.check (DRS.ofList []) mindFlayer = [] := by decide

/-- Arc Lightning -/
def arcLightning : Action :=
  dealDivided Primitives.NounPhrase.this (.lit 3) (Primitives.NounPhrase.described (Primitives.DetPhrase.target (oneThrough 3)) anyTarget)
theorem okArcLightning : Action.check (DRS.ofList []) arcLightning = [] := by decide

/-- Boulderfall -/
def boulderfall : Action :=
  dealDivided Primitives.NounPhrase.this (.lit 5) (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) anyTarget)
theorem okBoulderfall : Action.check (DRS.ofList []) boulderfall = [] := by decide

/-- Banisher Priest -/
def banisherPriest : Action :=
  exileUntil (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent]))
    (leavesBattlefield thisCreature)
theorem okBanisherPriest : Action.check (DRS.ofList []) banisherPriest = [] := by decide

/-- Tezzeret, Artifice Master -/
def tezzeretDrawTwo : Action :=
  Primitives.Action.replace (Primitives.Action.draw (.lit 1))
    (Primitives.Action.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            .atLeast (.lit 3))
      (Primitives.Action.draw (.lit 2)) none)
theorem okTezzeretDrawTwo : Action.check (DRS.ofList []) tezzeretDrawTwo = [] := by decide

/-- Zimone, Quandrix Prodigy -/
def zimoneDrawTwo : Action :=
  Primitives.Action.replace (Primitives.Action.draw (.lit 1))
    (Primitives.Action.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit 8))
      (Primitives.Action.draw (.lit 2)) none)
theorem okZimoneDrawTwo : Action.check (DRS.ofList []) zimoneDrawTwo = [] := by decide

/-- Aerial Volley {G} — Instant. "Aerial Volley deals 3 damage divided as you choose among
one, two, or three target creatures with flying." -/
def aerialVolley : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aerial Volley", cost := some [pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (dealDivided Primitives.NounPhrase.this (.lit 3)
            (Primitives.NounPhrase.described (Primitives.DetPhrase.target (oneThrough 3))
              (Primitives.Predicate.and [creature, Primitives.Predicate.hasKeyword (.the (.named "flying"))]))) ] } }

/-- Terrifying Presence -/
def terrifyingPresenceAnchor : Predicate := Primitives.Predicate.and [creature, Primitives.Predicate.otherThan (target creature)]
theorem okTerrifyingPresenceAnchor : Predicate.checkAt .object (DRS.ofList []) terrifyingPresenceAnchor = [] := by
  decide

/-- Timely Reinforcements -/
def timelyReinforcements : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Timely Reinforcements", cost := some [generic 2, pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.doIf (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .less (lifeTotalOf anOpponent))
                (gainLife (.lit 6)) none,
              Primitives.Action.doIf (Primitives.Condition.compareAmt (countOf creatureYouControl) .less
                      (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])))
                (create (.lit 3) (creatureToken 1 1 [.white] [creatureType "Soldier"])) none ]) ] } }

/-- Survival Cache -/
def survivalCache : Action :=
  Primitives.Action.sequentially
    [ gainLife (.lit 2),
      Primitives.Action.doIf (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .greater (lifeTotalOf anOpponent))
        (Primitives.Action.draw (.lit 1)) none ]
theorem okSurvivalCache : Action.check (DRS.ofList []) survivalCache = [] := by decide

/-- Nightmarish End -/
def nightmarishEnd : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nightmarish End", cost := some [generic 2, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ get (target creature) (Primitives.Delta.down (Primitives.Amount.letter .x)) (Primitives.Delta.down (Primitives.Amount.letter .x))
                (some untilEndOfTurn),
              Primitives.Action.define .x (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])) ]) ] } }

/-- Topple {2}{W} — Sorcery. "Exile target creature with the greatest power among creatures
on the battlefield." -/
def topple : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Topple", cost := some [generic 2, pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (exile (target
            (Primitives.Predicate.and [creature,
                   Primitives.Predicate.superlative .max (.stat .power) (Primitives.Predicate.and [creature, permanent])]))) ] } }

/-- "each opponent with three or more poison counters" -/
def corruptedOpponents : NounPhrase :=
  each (Primitives.Predicate.and [(Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you), Primitives.Predicate.compare [.counter (.named "poison")] .atLeast (.lit 3)])
theorem okCorruptedOpponents : NounPhrase.checkAt (some .player) (DRS.ofList []) corruptedOpponents = [] := by
  decide

/-- War Tax -/
def warTaxScaledPayment : Action :=
  Primitives.Action.establish
    (Primitives.StaticSpec.gatedBy
      (scaledGenericMana
        (Primitives.Amount.arith .times (Primitives.Amount.letter .x)
          (countOf (Primitives.Predicate.and [creature, attacking,
            Primitives.Predicate.hasPossessor .controller they]))))
      (Primitives.Subject.actor (allOf creature)) (Primitives.Action.attack none)
      {})
    (some thisTurn)
theorem okWarTaxScaledPayment : Action.check (DRS.ofList [letterB .x]) warTaxScaledPayment = [] := by
  decide

/-- Croaking Counterpart -/
def croakingCounterpartCopy : Action :=
  Action.create (.lit 1)
    (Primitives.TokenSpec.copyOf (target (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.hasSubtype (creatureType "Frog"))]))
      (copyCharacteristics {
                 colors := [.green], subtypes := [creatureType "Frog"], power := stat 1,
                 toughness := stat 1 } false))
    []
theorem okCroakingCounterpartCopy : Action.check (DRS.ofList []) croakingCounterpartCopy = [] := by decide

/-- Blessed Reversal -/
def blessedReversal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blessed Reversal", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (gainLife
            (times (.lit 3) (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.inCombat .attackerOf (some Primitives.NounPhrase.you)])))) ] } }

/-- Extinction -/
def extinction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Extinction", cost := some [generic 4, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (destroy (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.ofYourChoice (.subtype .creature) none]))) ] } }

/-- Defensive Maneuvers -/
def defensiveManeuvers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Defensive Maneuvers", cost := some [generic 3, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (get (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.ofYourChoice (.subtype .creature) none]))
            (Primitives.Delta.up (.lit 0)) (Primitives.Delta.up (.lit 4)) (some untilEndOfTurn)) ] } }

/-- Witch's Vengeance -/
def witchsVengeance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Witch's Vengeance", cost := some [generic 1, pip .black, pip .black],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (get (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.ofYourChoice (.subtype .creature) none]))
            (Primitives.Delta.down (.lit 3)) (Primitives.Delta.down (.lit 3)) (some untilEndOfTurn)) ] } }

/-- Phyrexian Rebirth -/
def phyrexianRebirth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Rebirth", cost := some [generic 4, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ destroy (allOf creature),
              Action.create (.lit 1)
                (Primitives.TokenSpec.written
                  { characteristics :=
                    { types := [.artifact, .creature],
                      subtypes := [creatureType "Phyrexian", creatureType "Horror"],
                      power := some (Primitives.Amount.letter .x), toughness := some (Primitives.Amount.letter .x) } })
                [],
              Primitives.Action.define .x (Primitives.Amount.countOf (theVerbed (.action "destroy") .many (some (.type .creature)) .thisWay)) ]) ] } }

/-- Damning Verdict -/
def damningVerdict : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damning Verdict", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text := [ Primitives.Ability.spell none (destroy (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.hasCounters none)]))) ] } }

/-- Hazardous Conditions -/
def hazardousConditions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hazardous Conditions", cost := some [generic 2, pip .black, pip .green],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (get (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.hasCounters none)]))
            (Primitives.Delta.down (.lit 2)) (Primitives.Delta.down (.lit 2)) (some untilEndOfTurn)) ] } }

/-- Approach of the Second Sun -/
def approachOfTheSecondSun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Approach of the Second Sun", cost := some [generic 6, pip .white],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.doIf
            (Primitives.Condition.and
              [ Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.castFrom (handOf Primitives.NounPhrase.you)),
                happened (Primitives.Action.procedure .cast ((a
                  (Primitives.Predicate.and [ spell, Primitives.Predicate.otherThan
                  Primitives.NounPhrase.this,
                             Primitives.Predicate.named (Primitives.NameSource.printed
                               "Approach of the Second Sun") ]))) Primitives.PlayPayment.itsOwnCost none none none) (Primitives.Subject.actor Primitives.NounPhrase.you) thisGame ])
            (Primitives.Action.conclude .winGame)
            (some (Primitives.Action.sequentially
              [ Primitives.Action.move Primitives.NounPhrase.this Primitives.Location.anyLocation (some (Primitives.Location.fromEnd .top (.nth 7) (Primitives.Location.zone .library none))) .arrival none [], gainLife (.lit 7) ]))) ] } }

/-- "a loyalty ability of enchanted planeswalker" -/
def loyaltyAbilityOfEnchanted : NounPhrase :=
  a (Primitives.Predicate.and [Primitives.Predicate.abilityHead .loyalty, Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type .planeswalker))])
theorem okLoyaltyAbilityOfEnchanted :
    NounPhrase.checkAt (some .object) (DRS.ofList []) loyaltyAbilityOfEnchanted = [] := by
  decide

/-- Balance of Power -/
def balanceOfPower : Action :=
  Primitives.Action.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf (target (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)))])) .greater
          (countOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])))
    (Primitives.Action.draw Primitives.Amount.theDifference) none
theorem okBalanceOfPower : Action.check (DRS.ofList []) balanceOfPower = [] := by decide

/-- Spark Fiend -/
def sparkFiendUpkeepRoll : Action :=
  Primitives.Action.sequentially
    [ rollDice 2 6,
      doIf (Primitives.Condition.compareAmt (Primitives.Amount.theOutcome .rollResult) .eq (.lit 7)) (sacrifice thisCreature) ]
theorem okSparkFiendUpkeepRoll : Action.check (DRS.ofList []) sparkFiendUpkeepRoll = [] := by decide

/-- Erebos, God of the Dead -/
def devotionCondition : Condition := Primitives.Condition.compareAmt (Primitives.Amount.devotion Primitives.NounPhrase.you (Primitives.ColorTerm.lit .black) none) .less (.lit 5)
theorem okDevotionCondition : Condition.check (DRS.ofList []) devotionCondition = [] := by decide

/-- Multiple Choice -/
def multipleChoiceFirstArm : Action :=
  Primitives.Action.doIf (Primitives.Condition.compareAmt (Primitives.Amount.letter .x) .eq (.lit 1))
    (Primitives.Action.sequentially [scry (.lit 1), Primitives.Action.draw (.lit 1)]) none
theorem okMultipleChoiceFirstArm : Action.check (DRS.ofList []) multipleChoiceFirstArm = [] := by decide

/-- Multiple Choice, fourth arm -/
def multipleChoiceFourthGate : Condition := Primitives.Condition.compareAmt (Primitives.Amount.letter .x) .atLeast (.lit 4)
theorem okMultipleChoiceFourthGate : Condition.check (DRS.ofList []) multipleChoiceFourthGate = [] := by decide

/-- Fell the Mighty -/
def fellTheMighty : Action :=
  destroy
    (allOf (Primitives.Predicate.and [creature,
                  Primitives.Predicate.compare [.stat .power] .greater (Primitives.Amount.statOf (.stat .power) (target creature))]))
theorem okFellTheMighty : Action.check (DRS.ofList []) fellTheMighty = [] := by decide

def targetPlayerOrPlaneswalker : NounPhrase := target (Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.anyPlayer])
theorem okTargetPlayerOrPlaneswalker :
    NounPhrase.checkAt none (DRS.ofList []) targetPlayerOrPlaneswalker = [] := by decide

def targetOpponentOrPlaneswalker : NounPhrase := target (Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)])
theorem okTargetOpponentOrPlaneswalker :
    NounPhrase.checkAt none (DRS.ofList []) targetOpponentOrPlaneswalker = [] := by decide

/-- Baleful Mastery's paid read -/
def balefulMasteryPaidRead : Ability :=
  Primitives.Ability.spell none (Primitives.Action.doIf (costWasPaid .theAlternative none Primitives.NounPhrase.this) (instruct (a (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you)) (Primitives.Action.draw (.lit 1))) none)
theorem okBalefulMasteryPaidRead : Ability.check (DRS.ofList []) balefulMasteryPaidRead = [] := by decide

/-- Karai, Future of the Foot -/
def karaiSneakPaidThisTurn : Amount := paidCostRead (.byKeyword (.named "sneak")) (some thisTurn) Primitives.NounPhrase.this
theorem okKaraiSneakPaidThisTurn : Amount.check (DRS.ofList []) karaiSneakPaidThisTurn = [] := by decide

/-- Requiting Hex's read -/
def requitingHexAdditionalRead : Ability :=
  Primitives.Ability.spell none (Primitives.Action.doIf (costWasPaid .theAdditional none Primitives.NounPhrase.this) (gainLife (.lit 2))
      none)
theorem okRequitingHexAdditionalRead : Ability.check (DRS.ofList []) requitingHexAdditionalRead = [] := by
  decide

/-- Lucid Dreams -/
def lucidDreams : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lucid Dreams", cost := some [generic 3, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.draw (Primitives.Amount.letter .x),
              Primitives.Action.define .x (Primitives.Amount.distinctCount .cardType (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)]))) ]) ] } }

/-- Rogues' Gallery -/
def roguesGallery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rogues' Gallery", cost := some [generic 2, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.doForEachKind .color none .color
            (move (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
                     (Primitives.Predicate.and [creature, ofChosen .color, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)]))
                  hand)) ] } }

/-- Bioplasm -/
def bioplasmAttack : GameEvent := Primitives.GameEvent.does (Primitives.Subject.actor thisCreature) (Primitives.Action.attack none)
theorem okBioplasmAttack : GameEvent.check (DRS.ofList []) bioplasmAttack = [] := by decide
def bioplasmExile : Action := exile (topSlice (.lit 1))
theorem okBioplasmExile :
    Action.check ((stepGameEvent 512 (DRS.ofList []) bioplasmAttack).1) bioplasmExile = [] := by decide
def bioplasmAfterExile : DRS := (stepAction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) bioplasmAttack).1) bioplasmExile).1
def bioplasmExiledCard : NounPhrase := theVerbed (.action "exile") .one (some .card)
theorem okBioplasmExiledCard :
    NounPhrase.checkAt (some .object) bioplasmAfterExile bioplasmExiledCard = [] := by decide

/-- "the top card of each player's library" -/
def playersTopCardSlice : NounPhrase := Primitives.NounPhrase.slice .top (.lit 1) (library (allOf Primitives.Predicate.anyPlayer))
theorem okPlayersTopCardSlice : NounPhrase.checkAt (some .object) (DRS.ofList []) playersTopCardSlice = [] := by
  decide

/-- Deepglow Skate's recipient -/
def deepglowSkateRecipient : NounPhrase := Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) permanent
theorem deepglowSkateRecipientRefused : deepglowSkateRecipient.perMemberOk = false := by decide

/-- Weftwalking -/
def weftwalkingShuffle : Action :=
  Primitives.Action.sequentially
    [ shuffleInto (Primitives.NounPhrase.both (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (handOf Primitives.NounPhrase.you)])) (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.in_ (graveyardOf Primitives.NounPhrase.you)]))),
      Primitives.Action.draw (.lit 7) ]
theorem okWeftwalkingShuffle : Action.check (DRS.ofList []) weftwalkingShuffle = [] := by decide

/-- Gaea's Revenge's protection-shaped phrase -/
def nongreenSpellsOrAbilities : Predicate :=
  Primitives.Predicate.or [ Primitives.Predicate.and [spell, Primitives.Predicate.not (Primitives.Predicate.colorIs .green)],
        Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyOnStack,
               Primitives.Predicate.abilityOf (a (Primitives.Predicate.and [source, Primitives.Predicate.not (Primitives.Predicate.colorIs .green)])) ] ]
theorem okNongreenSpellsOrAbilities :
    Predicate.checkAt .object (DRS.ofList []) nongreenSpellsOrAbilities = [] := by decide

/-- Hot Pursuit -/
def twoOrMorePlayersHaveLost : Condition :=
  Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and
    [Primitives.Predicate.anyPlayer, happenedTo (Primitives.Subject.actor (relative .object)) (Primitives.Action.conclude .loseGame) thisGame])) .atLeast (.lit 2)
theorem okTwoOrMorePlayersHaveLost : Condition.check (DRS.ofList []) twoOrMorePlayersHaveLost = [] := by decide

def commanderCreaturesYouOwn : Predicate :=
  Primitives.Predicate.and [creature, Primitives.Predicate.hasDesignation (.named "commander") none, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you]
theorem okCommanderCreaturesYouOwn : Predicate.checkAt .object (DRS.ofList []) commanderCreaturesYouOwn = [] := by
  decide

/-- Idol of Endurance -/
def creatureSpellFromAmongExiled : NounPhrase :=
  fromAmong (exactly 1) (Primitives.Predicate.and [creature, spell]) (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, exiledWithThisArtifact]))
theorem okCreatureSpellFromAmongExiled :
    NounPhrase.checkAt (some .object) (DRS.ofList []) creatureSpellFromAmongExiled = [] := by decide

def ringHasTemptedYouTwiceThisGame : Condition :=
  Primitives.Condition.compareAmt (eventCount (theRingTemptsYou) (Primitives.Subject.actor Primitives.NounPhrase.you) thisGame)
    .atLeast (.lit 2)
theorem okRingHasTemptedYouTwiceThisGame :
    Condition.check (DRS.ofList []) ringHasTemptedYouTwiceThisGame = [] := by decide

def creatureCardAnywhere : Predicate := Primitives.Predicate.and [creature, Primitives.Predicate.isCard]
theorem okCreatureCardAnywhere : Predicate.checkAt .object (DRS.ofList []) creatureCardAnywhere = [] := by decide
def permanentCardAnywhere : Predicate := permanentCard
theorem okPermanentCardAnywhere : Predicate.checkAt .object (DRS.ofList []) permanentCardAnywhere = [] := by decide

/-- "target card on the stack" -/
def targetCardOnTheStack : NounPhrase := target cardOnTheStack
theorem okTargetCardOnTheStack : NounPhrase.checkAt (some .object) (DRS.ofList []) targetCardOnTheStack = [] := by
  decide
/-- "each token on the battlefield" -/
def eachTokenOnTheBattlefield : NounPhrase := allOf tokenOnTheBattlefield
theorem okEachTokenOnTheBattlefield :
    NounPhrase.checkAt (some .object) (DRS.ofList []) eachTokenOnTheBattlefield = [] := by decide
/-- "each emblem you own" -/
def eachEmblemYouOwn : NounPhrase := allOf (Primitives.Predicate.and [emblem, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you])
theorem okEachEmblemYouOwn : NounPhrase.checkAt (some .object) (DRS.ofList []) eachEmblemYouOwn = [] := by decide
/-- "a copy of a card" -/
def aCopyOfACard : NounPhrase := a copyOfACard
theorem okACopyOfACard : NounPhrase.checkAt (some .object) (DRS.ofList []) aCopyOfACard = [] := by decide

/-- Keeper of the Flame, Keeper of the Light -/
def opponentWithMoreLifeThanYou : Predicate :=
  Primitives.Predicate.and [(Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you), Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you)]
theorem okOpponentWithMoreLifeThanYou :
    Predicate.checkAt .player (DRS.ofList []) opponentWithMoreLifeThanYou = [] := by decide
/-- Namor, Atlantean King -/
def playerWithMoreLifeThanYou : Predicate :=
  Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you)]
theorem okPlayerWithMoreLifeThanYou :
    Predicate.checkAt .player (DRS.ofList []) playerWithMoreLifeThanYou = [] := by decide
def noOpponentHasMoreLifeThanYou : Condition :=
  Primitives.Condition.not (exists_ (Primitives.Predicate.and [(Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you), Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you)]))
theorem okNoOpponentHasMoreLifeThanYou :
    Condition.check (DRS.ofList []) noOpponentHasMoreLifeThanYou = [] := by decide
def someOpponentLacksMoreLifeThanYou : Condition :=
  exists_ (Primitives.Predicate.and [(Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you),
                 Primitives.Predicate.not (Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you))])
theorem okSomeOpponentLacksMoreLifeThanYou :
    Condition.check (DRS.ofList []) someOpponentLacksMoreLifeThanYou = [] := by decide
def yourOpponentsHaveMoreLifeThanYou : Condition :=
  Primitives.Condition.matches (allOf (Primitives.Predicate.isOpponentOf Primitives.NounPhrase.you))
    (Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you))
theorem okYourOpponentsHaveMoreLifeThanYou :
    Condition.check (DRS.ofList []) yourOpponentsHaveMoreLifeThanYou = [] := by decide

/-- Disarm {U} — Instant. "Unattach all Equipment from target creature." -/
def disarm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disarm", cost := some [pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.unattach (allOf
            (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (artifactType "Equipment"),
                   Primitives.Predicate.attachedTo (target creature)]))) ] } }

def rawExile : Action := exile (target creature)
theorem okRawExile : Action.check (DRS.ofList []) rawExile = [] := by decide
def disenchant : Action := destroy (target (Primitives.Predicate.or [artifact, enchantment]))
theorem okDisenchant : Action.check (DRS.ofList []) disenchant = [] := by decide
def icyManipulator : Action := Primitives.Action.setStatus .tapped (target (Primitives.Predicate.or [artifact, creature, land]))
theorem okIcyManipulator : Action.check (DRS.ofList []) icyManipulator = [] := by decide
def divination : Action := Primitives.Action.draw (.lit 2)
theorem okDivination : Action.check (DRS.ofList []) divination = [] := by decide
def ancestralRecall : Action := instruct (target Primitives.Predicate.anyPlayer) (Primitives.Action.draw (.lit 3))
theorem okAncestralRecall : Action.check (DRS.ofList []) ancestralRecall = [] := by decide
def lifeTotalBecomesOne : Action := instruct (target Primitives.Predicate.anyPlayer) (setLife (.lit 1))
theorem okLifeTotalBecomesOne : Action.check (DRS.ofList []) lifeTotalBecomesOne = [] := by decide

/-- Berserker's Frenzy's roll -/
def berserkersFrenzyRoll : Action :=
  Primitives.Action.sequentially [rollDice 2 20, Primitives.Action.ignoreOutcomes (Primitives.IgnoredOutcomes.extreme .lowest)]
theorem okBerserkersFrenzyRoll : Action.check (DRS.ofList []) berserkersFrenzyRoll = [] := by decide
def ironMastiffIgnore : Action :=
  Primitives.Action.sequentially
    [ Primitives.Action.rollDice
        (countOf (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.inCombat .attackedBy (some (Primitives.NounPhrase.combatPlayer .attacking))]))
        (.sides 20) .many .anyResult,
      Primitives.Action.ignoreOutcomes (Primitives.IgnoredOutcomes.allBut .highest) ]
theorem okIronMastiffIgnore : Action.check (DRS.ofList []) ironMastiffIgnore = [] := by decide

/-- Xenosquirrels -/
def xenosquirrelsShift : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.rollDice Primitives.Amount.anyAmount .anyDie .one Primitives.RollWatch.anyResult)) [] none [] none none none
    (Primitives.Action.offer
      (Primitives.Action.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed p1p1Counter)) thisCreature .many true)
      (some (shiftResult (.lit 1))) none)
theorem okXenosquirrelsShift : Ability.check (DRS.ofList []) xenosquirrelsShift = [] := by decide

theorem playersTopCardIsPlural : playersTopCardSlice.plur = .many := by decide

/-- Aetheric Amplifier -/
def doubleYourOwnCounters : Action := Primitives.Action.doubleCounters Primitives.NounPhrase.you
theorem okDoubleYourOwnCounters : Action.check (DRS.ofList []) doubleYourOwnCounters = [] := by decide

def permanentCardPhrase : Predicate := permanentCard
theorem permanentCardIsPlaceless : permanentCardPhrase.phraseZone .object = none := by decide

/-- Archpriest of Iona -/
def archpriestOfIonaPower : Ability := Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .powerAlone partySize)
theorem okArchpriestOfIonaPower : Ability.check (DRS.ofList []) archpriestOfIonaPower = [] := by decide
def archpriestOfIonaFullParty : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .combat (some Primitives.NounPhrase.you) (Primitives.TurnRef.each none)) fullParty
    (Primitives.Action.sequentially
      [ get (target creature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn),
        gain it (keyword (.named "flying")) (some untilEndOfTurn) ])
theorem okArchpriestOfIonaFullParty : Ability.check (DRS.ofList []) archpriestOfIonaFullParty = [] := by decide

/-- Squad Commander -/
def squadCommanderTokens : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (create partySize (creatureToken 1 1 [.white] [creatureType "Kor", creatureType "Warrior"]))
theorem okSquadCommanderTokens : Ability.check (DRS.ofList []) squadCommanderTokens = [] := by decide

/-- At Knifepoint -/
def atKnifepointOutlaws : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you) (Primitives.StaticSpec.abilityGrant (allOf outlawYouControl) (keyword
      (.named "firstStrike"))))
def atKnifepointCrime : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.commitCrime)) [] none [] none (some oncePerTurn) none
    (create (.lit 1)
      { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [creatureType "Mercenary"],
          text :=
            [ activatedOnlyDuring Primitives.Cost.tapSymbol
                (get (target creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0))
                  (some untilEndOfTurn))
                asSorcery ],
          power := stat 1, toughness := stat 1 } })
def atKnifepoint : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "At Knifepoint", cost := some [generic 1, pip .black, pip .red],
      types := [.enchantment], text := [atKnifepointOutlaws, atKnifepointCrime] } }

end Semantics.Cards
