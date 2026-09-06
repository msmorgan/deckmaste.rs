import Semantics
import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Cards.Description

Port of `idris/src/Experimental/Cards/Description.idr`: the printed cards of the Description
family, and the phrase-level bench items beside them (a noun, predicate, condition,
instruction, or ability that elaborated in Idris), each paired with an `ok…` theorem that runs
the checker on it at the bindings its Idris type named.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

/-- Glyph of Destruction -/
def glyphOfDestruction : Instruction :=
  get (target (Primitives.Predicate.and [blocking, Primitives.Predicate.hasSubtype (creatureType "Wall"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
    (Primitives.Delta.up (.lit 10)) (Primitives.Delta.up (.lit 0)) (some untilEndOfCombat)
theorem okGlyphOfDestruction : Instruction.check [] glyphOfDestruction = [] := by decide

def rawNonattacking : Predicate := Primitives.Predicate.and [creature, Primitives.Predicate.not attacking, Primitives.Predicate.not blocking]
theorem okRawNonattacking : Predicate.check .object [] rawNonattacking = [] := by decide

/-- Harmony of Nature -/
def harmonyOfNature : Instruction :=
  Primitives.Instruction.sequence
    [ tap (counted anyNumber (Primitives.Predicate.and [untapped, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])),
      Primitives.Instruction.doForEach (theVerbed (.action "Tap") (.type .creature) .thisWay .many)
        (gainLife (.lit 4) (agent := Primitives.NounPhrase.you)) ]
theorem okHarmonyOfNature : Instruction.check [] harmonyOfNature = [] := by decide

/-- Rats of Rath -/
def ratsOfRath : Instruction :=
  destroy (target (Primitives.Predicate.and [Primitives.Predicate.or [artifact, creature, land], Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
theorem okRatsOfRath : Instruction.check [] ratsOfRath = [] := by decide

/-- "another creature or land", after a target on the battlefield. -/
def anotherDisjunctPhrase : Predicate := Primitives.Predicate.and [Primitives.Predicate.or [creature, land], Primitives.Predicate.other]
theorem okAnotherDisjunctPhrase :
    Predicate.check .object [⟨.target, .one, .object [] (some .battlefield) none none none⟩]
      anotherDisjunctPhrase = [] := by
  decide

/-- Defeat -/
def defeat : Instruction :=
  destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .power] .atMost (.lit 2)]))
theorem okDefeat : Instruction.check [] defeat = [] := by decide

/-- Terashi's Verdict -/
def terashisVerdict : Instruction :=
  destroy (target (Primitives.Predicate.and [creature, attacking, Primitives.Predicate.compare [.stat .power] .atMost (.lit 3)]))
theorem okTerashisVerdict : Instruction.check [] terashisVerdict = [] := by decide

/-- Pillar of Light -/
def pillarOfLight : Instruction :=
  exile (target (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .toughness] .atLeast (.lit 4)]))
theorem okPillarOfLight : Instruction.check [] pillarOfLight = [] := by decide

/-- Unholy Annex -/
def unholyAnnex : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you),
      Primitives.Instruction.doIf (exists_ (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Demon"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
        (Primitives.Instruction.sequence [loseLife (.lit 2) (agent := (each Primitives.Predicate.opponent)), gainLife (.lit 2) (agent :=
            Primitives.NounPhrase.you)])
        (some (loseLife (.lit 2) (agent := Primitives.NounPhrase.you))) ]
theorem okUnholyAnnex : Instruction.check [] unholyAnnex = [] := by decide

/-- War Screecher -/
def warScreecher : Instruction :=
  get (allOf (otherCreatureYouControl thisCreature)) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))
    (some untilEndOfTurn)
theorem okWarScreecher : Instruction.check [] warScreecher = [] := by decide

/-- Mind Flayer -/
def mindFlayer : Instruction :=
  gainControl (target creature)
    (some (Primitives.Duration.forAsLongAs (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you)))) (agent := Primitives.NounPhrase.you)
theorem okMindFlayer : Instruction.check [] mindFlayer = [] := by decide

/-- Arc Lightning -/
def arcLightning : Instruction :=
  dealDivided Primitives.NounPhrase.this (.lit 3) (Primitives.NounPhrase.described (Primitives.DetPhrase.target (oneThrough 3)) anyTarget)
theorem okArcLightning : Instruction.check [] arcLightning = [] := by decide

/-- Boulderfall -/
def boulderfall : Instruction :=
  dealDivided Primitives.NounPhrase.this (.lit 5) (Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) anyTarget)
theorem okBoulderfall : Instruction.check [] boulderfall = [] := by decide

/-- Banisher Priest -/
def banisherPriest : Instruction :=
  exileUntil (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent]))
    (leavesBattlefield thisCreature)
theorem okBanisherPriest : Instruction.check [] banisherPriest = [] := by decide

/-- Tezzeret, Artifice Master -/
def tezzeretDrawTwo : Instruction :=
  Primitives.Instruction.replace (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
    (Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            .atLeast (.lit 3))
      (Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you)) none)
theorem okTezzeretDrawTwo : Instruction.check [] tezzeretDrawTwo = [] := by decide

/-- Zimone, Quandrix Prodigy -/
def zimoneDrawTwo : Instruction :=
  Primitives.Instruction.replace (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
    (Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atLeast (.lit 8))
      (Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you)) none)
theorem okZimoneDrawTwo : Instruction.check [] zimoneDrawTwo = [] := by decide

/-- Aerial Volley {G} — Instant. "Aerial Volley deals 3 damage divided as you choose among
one, two, or three target creatures with flying." -/
def aerialVolley : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aerial Volley", cost := some [pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (dealDivided Primitives.NounPhrase.this (.lit 3)
            (Primitives.NounPhrase.described (Primitives.DetPhrase.target (oneThrough 3))
              (Primitives.Predicate.and [creature, Primitives.Predicate.hasKeyword (.the "Flying")]))) ] } }

/-- Terrifying Presence -/
def terrifyingPresenceAnchor : Predicate := Primitives.Predicate.and [creature, Primitives.Predicate.otherThan (target creature)]
theorem okTerrifyingPresenceAnchor : Predicate.check .object [] terrifyingPresenceAnchor = [] := by
  decide

/-- Timely Reinforcements -/
def timelyReinforcements : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Timely Reinforcements", cost := some [generic 2, pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.doIf (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .less (lifeTotalOf anOpponent))
                (gainLife (.lit 6) (agent := Primitives.NounPhrase.you)) none,
              Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf creatureYouControl) .less
                      (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller anOpponent])))
                (create (.lit 3) (creatureToken 1 1 [.white] [creatureType "Soldier"])) none ]) ] } }

/-- Survival Cache -/
def survivalCache : Instruction :=
  Primitives.Instruction.sequence
    [ gainLife (.lit 2) (agent := Primitives.NounPhrase.you),
      Primitives.Instruction.doIf (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .greater (lifeTotalOf anOpponent))
        (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) none ]
theorem okSurvivalCache : Instruction.check [] survivalCache = [] := by decide

/-- Nightmarish End -/
def nightmarishEnd : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nightmarish End", cost := some [generic 2, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ get (target creature) (Primitives.Delta.down (Primitives.Amount.letter .x)) (Primitives.Delta.down (Primitives.Amount.letter .x))
                (some untilEndOfTurn),
              Primitives.Instruction.define .x (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))) ]) ] } }

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
  each (Primitives.Predicate.and [Primitives.Predicate.opponent, Primitives.Predicate.compare [.counter (.named "Poison")] .atLeast (.lit 3)])
theorem okCorruptedOpponents : NounPhrase.check (some .player) [] corruptedOpponents = [] := by
  decide

/-- War Tax -/
def warTaxScaledPayment : Instruction :=
  Primitives.Instruction.establish
    (deontic (allOf creature)
      (Primitives.Compulsion.gatedBy (scaledMana .generic
        (Primitives.Amount.arith .times (Primitives.Amount.letter .x)
          (countOf (Primitives.Predicate.and [creature, attacking, Primitives.Predicate.hasPossessor .controller they])))))
      [.core .attack] .agent Primitives.DeonticPatient.noPatient)
    (some Primitives.Duration.thisTurn)
theorem okWarTaxScaledPayment : Instruction.check [letterB .x] warTaxScaledPayment = [] := by
  decide

/-- Croaking Counterpart -/
def croakingCounterpartCopy : Instruction :=
  Primitives.Instruction.create (.lit 1)
    (Primitives.TokenSpec.copyOf (target (Primitives.Predicate.and [creature, Primitives.Predicate.not (Primitives.Predicate.hasSubtype (creatureType "Frog"))]))
      [ Primitives.CopyExcept.chars {
                 colors := [.green], subtypes := [creatureType "Frog"], power := stat 1,
                 toughness := stat 1 } false ])
    [] (agent := Primitives.NounPhrase.you)
theorem okCroakingCounterpartCopy : Instruction.check [] croakingCounterpartCopy = [] := by decide

/-- Blessed Reversal -/
def blessedReversal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blessed Reversal", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (gainLife
            (times (.lit 3) (countOf (Primitives.Predicate.and [creature, Primitives.Predicate.inCombat .attackerOf (some Primitives.NounPhrase.you)]))) (agent
                := Primitives.NounPhrase.you)) ] } }

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
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ destroy (allOf creature),
              Primitives.Instruction.create (.lit 1)
                (Primitives.TokenSpec.written
                  { characteristics :=
                    { types := [.artifact, .creature],
                      subtypes := [creatureType "Phyrexian", creatureType "Horror"],
                      power := some (Primitives.Amount.letter .x), toughness := some (Primitives.Amount.letter .x) } })
                [] (agent := Primitives.NounPhrase.you),
              Primitives.Instruction.define .x (Primitives.Amount.countOf (theVerbed (.action "Destroy") (.type .creature) .thisWay .many)) ]) ] } }

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
        [ Primitives.Ability.spell none (Primitives.Instruction.doIf
            (Primitives.Condition.and
              [ Primitives.Condition.matches Primitives.NounPhrase.this (Primitives.Predicate.castFrom (handOf Primitives.NounPhrase.you)),
                happenedInvolving .spellCast Primitives.NounPhrase.you .thisGame
                  (a (Primitives.Predicate.and [ spell, Primitives.Predicate.otherThan Primitives.NounPhrase.this,
                             Primitives.Predicate.named (Primitives.NameSource.printed "Approach of the Second Sun") ])) ])
            (Primitives.Instruction.conclude .winGame (agent := Primitives.NounPhrase.you))
            (some (Primitives.Instruction.sequence
              [ move Primitives.NounPhrase.this (nthFromTop (.nth 7)), gainLife (.lit 7) (agent := Primitives.NounPhrase.you) ]))) ] } }

/-- "a loyalty ability of enchanted planeswalker" -/
def loyaltyAbilityOfEnchanted : NounPhrase :=
  a (Primitives.Predicate.and [Primitives.Predicate.abilityHead .loyalty, Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type .planeswalker))])
theorem okLoyaltyAbilityOfEnchanted :
    NounPhrase.check (some .object) [] loyaltyAbilityOfEnchanted = [] := by
  decide

/-- Balance of Power -/
def balanceOfPower : Instruction :=
  Primitives.Instruction.doIf (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.inZone (handOf (target Primitives.Predicate.opponent)))) .greater
          (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))))
    (Primitives.Instruction.draw Primitives.Amount.theDifference (agent := Primitives.NounPhrase.you)) none
theorem okBalanceOfPower : Instruction.check [] balanceOfPower = [] := by decide

/-- Spark Fiend -/
def sparkFiendUpkeepRoll : Instruction :=
  Primitives.Instruction.sequence
    [ rollDice 2 6 (agent := Primitives.NounPhrase.you),
      doIf (Primitives.Condition.compareAmt (Primitives.Amount.theOutcome .rollResult) .eq (.lit 7)) (sacrifice thisCreature (agent :=
          Primitives.NounPhrase.you)) ]
theorem okSparkFiendUpkeepRoll : Instruction.check [] sparkFiendUpkeepRoll = [] := by decide

/-- Erebos, God of the Dead -/
def devotionCondition : Condition := Primitives.Condition.compareAmt (Primitives.Amount.devotion Primitives.NounPhrase.you (Primitives.ColorTerm.lit .black) none) .less (.lit 5)
theorem okDevotionCondition : Condition.check [] devotionCondition = [] := by decide

/-- Multiple Choice -/
def multipleChoiceFirstArm : Instruction :=
  Primitives.Instruction.doIf (Primitives.Condition.compareAmt (Primitives.Amount.letter .x) .eq (.lit 1))
    (Primitives.Instruction.sequence [scry (.lit 1) (agent := Primitives.NounPhrase.you), Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)]) none
theorem okMultipleChoiceFirstArm : Instruction.check [] multipleChoiceFirstArm = [] := by decide

/-- Multiple Choice, fourth arm -/
def multipleChoiceFourthGate : Condition := Primitives.Condition.compareAmt (Primitives.Amount.letter .x) .atLeast (.lit 4)
theorem okMultipleChoiceFourthGate : Condition.check [] multipleChoiceFourthGate = [] := by decide

/-- Fell the Mighty -/
def fellTheMighty : Instruction :=
  destroy
    (allOf (Primitives.Predicate.and [creature,
                  Primitives.Predicate.compare [.stat .power] .greater (Primitives.Amount.statOf (.stat .power) (target creature))]))
theorem okFellTheMighty : Instruction.check [] fellTheMighty = [] := by decide

def targetPlayerOrPlaneswalker : NounPhrase := target (Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.anyPlayer])
theorem okTargetPlayerOrPlaneswalker :
    NounPhrase.check none [] targetPlayerOrPlaneswalker = [] := by decide

def targetOpponentOrPlaneswalker : NounPhrase := target (Primitives.Predicate.or [Primitives.Predicate.hasType .planeswalker, Primitives.Predicate.opponent])
theorem okTargetOpponentOrPlaneswalker :
    NounPhrase.check none [] targetOpponentOrPlaneswalker = [] := by decide

/-- Baleful Mastery's paid read -/
def balefulMasteryPaidRead : Ability :=
  Primitives.Ability.spell none (Primitives.Instruction.doIf (costWasPaid .theAlternative none Primitives.NounPhrase.this) (Primitives.Instruction.draw (.lit 1) (agent := (a
      Primitives.Predicate.opponent))) none)
theorem okBalefulMasteryPaidRead : Ability.check [] balefulMasteryPaidRead = [] := by decide

/-- Karai, Future of the Foot -/
def karaiSneakPaidThisTurn : Amount := paidCostRead (.byKeyword "Sneak") (some .thisTurn) Primitives.NounPhrase.this
theorem okKaraiSneakPaidThisTurn : Amount.check [] karaiSneakPaidThisTurn = [] := by decide

/-- Requiting Hex's read -/
def requitingHexAdditionalRead : Ability :=
  Primitives.Ability.spell none (Primitives.Instruction.doIf (costWasPaid .theAdditional none Primitives.NounPhrase.this) (gainLife (.lit 2) (agent := Primitives.NounPhrase.you))
      none)
theorem okRequitingHexAdditionalRead : Ability.check [] requitingHexAdditionalRead = [] := by
  decide

/-- Lucid Dreams -/
def lucidDreams : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lucid Dreams", cost := some [generic 3, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.draw (Primitives.Amount.letter .x) (agent := Primitives.NounPhrase.you),
              Primitives.Instruction.define .x (Primitives.Amount.distinctCount .cardType (allOf (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)))) ]) ] } }

/-- Rogues' Gallery -/
def roguesGallery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rogues' Gallery", cost := some [generic 2, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.doForEachKind .color none .color
            (move (Primitives.NounPhrase.described (Primitives.DetPhrase.target (upTo 1))
                     (Primitives.Predicate.and [creature, ofChosen .color, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)]))
                  hand)) ] } }

/-- Bioplasm -/
def bioplasmAttack : GameEvent := Primitives.GameEvent.combat .attackerOf thisCreature none
theorem okBioplasmAttack : GameEvent.check [] bioplasmAttack = [] := by decide
def bioplasmExile : Instruction := exile (topSlice (.lit 1))
theorem okBioplasmExile :
    Instruction.check (GameEvent.after [] bioplasmAttack) bioplasmExile = [] := by decide
def bioplasmAfterExile : Bindings := Instruction.intro (GameEvent.after [] bioplasmAttack) bioplasmExile
def bioplasmExiledCard : NounPhrase := theVerbed (.action "Exile") .card .attributive .one
theorem okBioplasmExiledCard :
    NounPhrase.check (some .object) bioplasmAfterExile bioplasmExiledCard = [] := by decide

/-- "the top card of each player's library" -/
def playersTopCardSlice : NounPhrase := Primitives.NounPhrase.librarySlice .top (.lit 1) (Primitives.NounPhrase.playerGroup .allPlayers)
theorem okPlayersTopCardSlice : NounPhrase.check (some .object) [] playersTopCardSlice = [] := by
  decide

/-- Deepglow Skate's recipient -/
def deepglowSkateRecipient : NounPhrase := Primitives.NounPhrase.described (Primitives.DetPhrase.target anyNumber) permanent
theorem deepglowSkateRecipientRefused : deepglowSkateRecipient.perMemberOk = false := by decide

/-- Weftwalking -/
def weftwalkingShuffle : Instruction :=
  Primitives.Instruction.sequence
    [ shuffleInto (Primitives.NounPhrase.both (allOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))) (allOf (Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you))))
        (agent := Primitives.NounPhrase.you),
      Primitives.Instruction.draw (.lit 7) (agent := Primitives.NounPhrase.you) ]
theorem okWeftwalkingShuffle : Instruction.check [] weftwalkingShuffle = [] := by decide

/-- Gaea's Revenge's protection-shaped phrase -/
def nongreenSpellsOrAbilities : Predicate :=
  Primitives.Predicate.or [ Primitives.Predicate.and [spell, Primitives.Predicate.not (Primitives.Predicate.colorIs .green)],
        Primitives.Predicate.and [ Primitives.Predicate.abilityHead .anyOnStack,
               Primitives.Predicate.abilityOf (a (Primitives.Predicate.and [source, Primitives.Predicate.not (Primitives.Predicate.colorIs .green)])) ] ]
theorem okNongreenSpellsOrAbilities :
    Predicate.check .object [] nongreenSpellsOrAbilities = [] := by decide

/-- Hot Pursuit -/
def twoOrMorePlayersHaveLost : Condition :=
  Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, happenedTo .gameLoss .thisGame])) .atLeast (.lit 2)
theorem okTwoOrMorePlayersHaveLost : Condition.check [] twoOrMorePlayersHaveLost = [] := by decide

def commanderCreaturesYouOwn : Predicate :=
  Primitives.Predicate.and [creature, Primitives.Predicate.hasDesignation "commander" none, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you]
theorem okCommanderCreaturesYouOwn : Predicate.check .object [] commanderCreaturesYouOwn = [] := by
  decide

/-- Idol of Endurance -/
def creatureSpellFromAmongExiled : NounPhrase :=
  fromAmong (exactly 1) (Primitives.Predicate.and [creature, spell]) (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, exiledWithThisArtifact]))
theorem okCreatureSpellFromAmongExiled :
    NounPhrase.check (some .object) [] creatureSpellFromAmongExiled = [] := by decide

def ringHasTemptedYouTwiceThisGame : Condition :=
  Primitives.Condition.compareAmt (eventCount (.verbedAct (.action "The Ring Tempts You")) Primitives.NounPhrase.you .thisGame)
    .atLeast (.lit 2)
theorem okRingHasTemptedYouTwiceThisGame :
    Condition.check [] ringHasTemptedYouTwiceThisGame = [] := by decide

def creatureCardAnywhere : Predicate := Primitives.Predicate.and [creature, Primitives.Predicate.isCard]
theorem okCreatureCardAnywhere : Predicate.check .object [] creatureCardAnywhere = [] := by decide
def permanentCardAnywhere : Predicate := permanentCard
theorem okPermanentCardAnywhere : Predicate.check .object [] permanentCardAnywhere = [] := by decide

/-- "target card on the stack" -/
def targetCardOnTheStack : NounPhrase := target cardOnTheStack
theorem okTargetCardOnTheStack : NounPhrase.check (some .object) [] targetCardOnTheStack = [] := by
  decide
/-- "each token on the battlefield" -/
def eachTokenOnTheBattlefield : NounPhrase := allOf tokenOnTheBattlefield
theorem okEachTokenOnTheBattlefield :
    NounPhrase.check (some .object) [] eachTokenOnTheBattlefield = [] := by decide
/-- "each emblem you own" -/
def eachEmblemYouOwn : NounPhrase := allOf (Primitives.Predicate.and [emblem, Primitives.Predicate.hasPossessor .owner Primitives.NounPhrase.you])
theorem okEachEmblemYouOwn : NounPhrase.check (some .object) [] eachEmblemYouOwn = [] := by decide
/-- "a copy of a card" -/
def aCopyOfACard : NounPhrase := a copyOfACard
theorem okACopyOfACard : NounPhrase.check (some .object) [] aCopyOfACard = [] := by decide

/-- Keeper of the Flame, Keeper of the Light -/
def opponentWithMoreLifeThanYou : Predicate :=
  Primitives.Predicate.and [Primitives.Predicate.opponent, Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you)]
theorem okOpponentWithMoreLifeThanYou :
    Predicate.check .player [] opponentWithMoreLifeThanYou = [] := by decide
/-- Namor, Atlantean King -/
def playerWithMoreLifeThanYou : Predicate :=
  Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you)]
theorem okPlayerWithMoreLifeThanYou :
    Predicate.check .player [] playerWithMoreLifeThanYou = [] := by decide
def noOpponentHasMoreLifeThanYou : Condition :=
  Primitives.Condition.not (exists_ (Primitives.Predicate.and [Primitives.Predicate.opponent, Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you)]))
theorem okNoOpponentHasMoreLifeThanYou :
    Condition.check [] noOpponentHasMoreLifeThanYou = [] := by decide
def someOpponentLacksMoreLifeThanYou : Condition :=
  exists_ (Primitives.Predicate.and [Primitives.Predicate.opponent,
                 Primitives.Predicate.not (Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you))])
theorem okSomeOpponentLacksMoreLifeThanYou :
    Condition.check [] someOpponentLacksMoreLifeThanYou = [] := by decide
def yourOpponentsHaveMoreLifeThanYou : Condition :=
  Primitives.Condition.matches (Primitives.NounPhrase.playerGroup .yourOpponents)
    (Primitives.Predicate.compare [.playerStat .lifeTotal] .greater (lifeTotalOf Primitives.NounPhrase.you))
theorem okYourOpponentsHaveMoreLifeThanYou :
    Condition.check [] yourOpponentsHaveMoreLifeThanYou = [] := by decide

/-- Disarm {U} — Instant. "Unattach all Equipment from target creature." -/
def disarm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disarm", cost := some [pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.unattach (allOf
            (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (artifactType "Equipment"),
                   Primitives.Predicate.attachedTo (target creature)]))) ] } }

def rawExile : Instruction := exile (target creature)
theorem okRawExile : Instruction.check [] rawExile = [] := by decide
def disenchant : Instruction := destroy (target (Primitives.Predicate.or [artifact, enchantment]))
theorem okDisenchant : Instruction.check [] disenchant = [] := by decide
def icyManipulator : Instruction := Primitives.Instruction.setStatus .tapped (target (Primitives.Predicate.or [artifact, creature, land]))
theorem okIcyManipulator : Instruction.check [] icyManipulator = [] := by decide
def divination : Instruction := Primitives.Instruction.draw (.lit 2) (agent := Primitives.NounPhrase.you)
theorem okDivination : Instruction.check [] divination = [] := by decide
def ancestralRecall : Instruction := Primitives.Instruction.draw (.lit 3) (agent := (target Primitives.Predicate.anyPlayer))
theorem okAncestralRecall : Instruction.check [] ancestralRecall = [] := by decide
def lifeTotalBecomesOne : Instruction := setLife (.lit 1) (agent := (target Primitives.Predicate.anyPlayer))
theorem okLifeTotalBecomesOne : Instruction.check [] lifeTotalBecomesOne = [] := by decide

/-- Berserker's Frenzy's roll -/
def berserkersFrenzyRoll : Instruction :=
  Primitives.Instruction.sequence [rollDice 2 20 (agent := Primitives.NounPhrase.you), Primitives.Instruction.ignoreOutcomes (Primitives.IgnoredOutcomes.extreme .lowest)]
theorem okBerserkersFrenzyRoll : Instruction.check [] berserkersFrenzyRoll = [] := by decide
def ironMastiffIgnore : Instruction :=
  Primitives.Instruction.sequence
    [ Primitives.Instruction.rollDice
        (countOf (Primitives.Predicate.and [Primitives.Predicate.anyPlayer, Primitives.Predicate.inCombat .attackedBy (some (Primitives.NounPhrase.combatPlayer .attacking))]))
        (.sides 20) (agent := Primitives.NounPhrase.you),
      Primitives.Instruction.ignoreOutcomes (Primitives.IgnoredOutcomes.allBut .highest) ]
theorem okIronMastiffIgnore : Instruction.check [] ironMastiffIgnore = [] := by decide

/-- Xenosquirrels -/
def xenosquirrelsShift : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.rollsDice Primitives.NounPhrase.you .one none Primitives.RollWatch.anyResult) [] none [] none none none
    (Primitives.Instruction.offer
      (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed plusOnePlusOne)) thisCreature)
      (some (shiftResult (.lit 1))) none (agent := Primitives.NounPhrase.you))
theorem okXenosquirrelsShift : Ability.check [] xenosquirrelsShift = [] := by decide

theorem playersTopCardIsPlural : playersTopCardSlice.plur = .many := by decide

/-- Aetheric Amplifier -/
def doubleYourOwnCounters : Instruction := Primitives.Instruction.doubleCounters Primitives.NounPhrase.you
theorem okDoubleYourOwnCounters : Instruction.check [] doubleYourOwnCounters = [] := by decide

def permanentCardPhrase : Predicate := permanentCard
theorem permanentCardIsPlaceless : permanentCardPhrase.phraseZone .object = none := by decide

/-- Archpriest of Iona -/
def archpriestOfIonaPower : Ability := Primitives.Ability.static (Primitives.StaticSpec.ptDefinition thisCreature .powerAlone partySize)
theorem okArchpriestOfIonaPower : Ability.check [] archpriestOfIonaPower = [] := by decide
def archpriestOfIonaFullParty : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .the .combat (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you)) fullParty
    (Primitives.Instruction.sequence
      [ get (target creature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn),
        gain it (keyword "Flying") (some untilEndOfTurn) ])
theorem okArchpriestOfIonaFullParty : Ability.check [] archpriestOfIonaFullParty = [] := by decide

/-- Squad Commander -/
def squadCommanderTokens : Ability :=
  when (Primitives.GameEvent.enters thisCreature none)
    (create partySize (creatureToken 1 1 [.white] [creatureType "Kor", creatureType "Warrior"]))
theorem okSquadCommanderTokens : Ability.check [] squadCommanderTokens = [] := by decide

/-- At Knifepoint -/
def atKnifepointOutlaws : Ability :=
  Primitives.Ability.static (Primitives.StaticSpec.partScope .turn (some Primitives.NounPhrase.you) (Primitives.StaticSpec.abilityGrant (allOf outlawYouControl) (keyword
      "FirstStrike")))
def atKnifepointCrime : Ability :=
  Primitives.Ability.triggered (Primitives.GameEvent.commitsCrime Primitives.NounPhrase.you) [] none [] none (some Primitives.UsageLimit.oncePerTurn) none
    (create (.lit 1)
      { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [creatureType "Mercenary"],
          text :=
            [ activatedOnlyDuring Primitives.Cost.tapSymbol
                (get (target creatureYouControl) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0))
                  (some untilEndOfTurn))
                Primitives.Timing.asSorcery ],
          power := stat 1, toughness := stat 1 } })
def atKnifepoint : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "At Knifepoint", cost := some [generic 1, pip .black, pip .red],
      types := [.enchantment], text := [atKnifepointOutlaws, atKnifepointCrime] } }

end Semantics.Cards
