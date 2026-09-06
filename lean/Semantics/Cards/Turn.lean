import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Anaphora

/-!
# Semantics.Cards.Turn

Port of `idris/src/Experimental/Cards/Turn.idr`: the printed cards of the Turn family (turn
parts, extra turns, additional phases, cast windows) and the bench items beside them.

Not ported: `fracturedPowerstonePlanarRoll` (the planar die is Planechase, out of scope).
`shapeshifter`'s printed `*`/`7-*` box is `none` under the printed-star ruling; the
characteristic-defining clauses set it.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

/-- Through the Breach Splice -/
def throughTheBreach : Instruction :=
  Primitives.Instruction.sequentially
    [ offer (move (a (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])) battlefield) (agent := Primitives.NounPhrase.you),
      gainHaste (that (.type .creature)) none,
      delay (Primitives.GameEvent.beginningOf .the .endStep Primitives.HeaderPossessor.noPossessor)
        (sacrifice (that (.type .creature)) (agent := Primitives.NounPhrase.you)) ]
theorem okThroughTheBreach : Instruction.check [] throughTheBreach = [] := by decide

def turnToMist : Instruction :=
  Primitives.Instruction.sequentially
    [ exile (target creature),
      delay (Primitives.GameEvent.beginningOf .the .endStep Primitives.HeaderPossessor.noPossessor) (move (that .card) battlefield) ]
theorem okTurnToMist : Instruction.check [] turnToMist = [] := by decide

def voyagerStaff : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.perform (sacrifice thisArtifact (agent := Primitives.NounPhrase.you))])
    (Primitives.Instruction.sequentially
      [ exile (target creature),
        delay (Primitives.GameEvent.beginningOf .the .endStep Primitives.HeaderPossessor.noPossessor)
          (move (theVerbed (.action "Exile") .card .attributive .one) battlefield) ])
theorem okVoyagerStaff : Ability.check [] voyagerStaff = [] := by decide

def staffOfNin : Ability :=
  at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you)) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okStaffOfNin : Ability.check [] staffOfNin = [] := by decide

def silentAssassin : Ability :=
  activated (Primitives.Cost.mana [generic 3, pip .black])
    (delay (Primitives.GameEvent.beginningOf .the .endOfCombat Primitives.HeaderPossessor.noPossessor)
      (destroy (target (Primitives.Predicate.and [blocking, creature]))))
theorem okSilentAssassin : Ability.check [] silentAssassin = [] := by decide

def kjeldoranFrostbeast : Ability :=
  at_ (Primitives.GameEvent.beginningOf .the .endOfCombat Primitives.HeaderPossessor.noPossessor)
    (destroy (allOf (Primitives.Predicate.and [ creature,
                            Primitives.Predicate.or [ Primitives.Predicate.inCombat .blockerOf (some thisCreature),
                                  Primitives.Predicate.inCombat .blockedBy (some thisCreature) ] ])))
theorem okKjeldoranFrostbeast : Ability.check [] kjeldoranFrostbeast = [] := by decide

/-- Tippy-Toe, Terrific Partner -/
def tippyToe : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer
    Primitives.NounPhrase.you)) (happened (Primitives.GameEvent.lifeChanges (relative .player) .up)
    Primitives.NounPhrase.you .thisTurn)
    (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okTippyToe : Ability.check [] tippyToe = [] := by decide

def brazenCannonade : Ability :=
  abilityWord "raid"
    (triggeredIf (Primitives.GameEvent.beginningOf .each .postcombatMain (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
      (happened (Primitives.GameEvent.combat .attackerOf (relative .player) none)
        Primitives.NounPhrase.you .thisTurn)
      (exile (topSlice (.lit 1))))
theorem okBrazenCannonade : Ability.check [] brazenCannonade = [] := by decide

def fourKnocks : Ability :=
  at_ (Primitives.GameEvent.beginningOf .the .firstMain (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you)) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okFourKnocks : Ability.check [] fourKnocks = [] := by decide

def hammerOfBogardan : Ability :=
  activatedOnlyDuring (Primitives.Cost.mana [generic 2, pip .red, pip .red, pip .red]) (move Primitives.NounPhrase.this hand)
    (Primitives.Timing.duringPart .upkeep (some Primitives.NounPhrase.you))
theorem okHammerOfBogardan : Ability.check [] hammerOfBogardan = [] := by decide

/-- Apathy -/
def apathy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Apathy", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          Primitives.Ability.static (doesntUntap (Primitives.NounPhrase.attachHost .enchanted (.type .creature))
            (some (controllerOf (Primitives.NounPhrase.attachHost .enchanted (.type .creature))))),
          at_ (beginningOfPossessed .the .upkeep
                (controllerOf (Primitives.NounPhrase.attachHost .enchanted (.type .creature))))
            (Primitives.Instruction.offer (discard (aAtRandom (Primitives.Predicate.inZone hand)) (agent := (that .player)))
              (some (untap (that (.type .creature)))) none (agent := (that .player))) ] } }

def felidarSovereign : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Felidar Sovereign", cost := some [generic 4, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Cat", creatureType "Beast"],
      text :=
        [ keyword "Vigilance", keyword "Lifelink",
          triggeredIf (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .atLeast (.lit 40))
            (Primitives.Instruction.conclude .winGame (agent := Primitives.NounPhrase.you)) ],
      power := stat 4, toughness := stat 6 } }

def gloriousEnforcer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glorious Enforcer", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying", keyword "Lifelink",
          triggeredIf (Primitives.GameEvent.beginningOf .the .combat (Primitives.HeaderPossessor.byPlayer (each Primitives.Predicate.anyPlayer)))
            (Primitives.Condition.compareAmt (lifeTotalOf Primitives.NounPhrase.you) .greater (lifeTotalOf anOpponent))
            (gain thisCreature (keyword "DoubleStrike") (some untilEndOfTurn)) ],
      power := stat 5, toughness := stat 5 } }

/-- Damia, Sage of Stone -/
def damia : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))) .less (.lit 7))
    (Primitives.Instruction.draw Primitives.Amount.theDifference (agent := Primitives.NounPhrase.you))
theorem okDamia : Ability.check [] damia = [] := by decide

def ivoryTower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ivory Tower", cost := some [generic 1], types := [.artifact],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Instruction.sequentially
              [ gainLife (Primitives.Amount.letter .x) (agent := Primitives.NounPhrase.you),
                Primitives.Instruction.define .x (minus (countOf (Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you))) (.lit 4)) ]) ] } }

/-- Ajani, Adversary of Tyrants -/
def ajanisEmblem : Instruction :=
  Primitives.Instruction.getEmblem
    [ at_ (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
        (create (.lit 3)
          { characteristics :=
            { colors := [.white], types := [.creature], subtypes := [creatureType "Cat"],
              text := [keyword "Lifelink"], power := stat 1, toughness := stat 1 } }) ] (agent :=
                  Primitives.NounPhrase.you)
theorem okAjanisEmblem : Instruction.check [] ajanisEmblem = [] := by decide

/-- Saheeli Rai -/
def saheelisCopy : Instruction :=
  Primitives.Instruction.sequentially
    [ Primitives.Instruction.create (.lit 1)
        (Primitives.TokenSpec.copyOf (target (Primitives.Predicate.and [Primitives.Predicate.or [artifact, creature], Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
          [Primitives.CopyExcept.types [.artifact] []])
        [] (agent := Primitives.NounPhrase.you),
      gainHaste (that .token) none,
      delay (Primitives.GameEvent.beginningOf .the .endStep Primitives.HeaderPossessor.noPossessor) (exile it) ]
theorem okSaheelisCopy : Instruction.check [] saheelisCopy = [] := by decide

def timeWalk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Time Walk", cost := some [generic 1, pip .blue], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Instruction.addTurn (.lit 1) (agent := Primitives.NounPhrase.you))] } }

def timeStretch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Time Stretch", cost := some [generic 8, pip .blue, pip .blue], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Instruction.addTurn (.lit 2) (agent := (target Primitives.Predicate.anyPlayer)))] } }

def timeSieve : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Time Sieve", cost := some [pip .blue, pip .black], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice (counted (exactly 5) artifact)
            (agent := Primitives.NounPhrase.you))])
            (Primitives.Instruction.addTurn (.lit 1) (agent := Primitives.NounPhrase.you)) ] } }

def finalFortune : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Final Fortune", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.addTurn (.lit 1) (agent := Primitives.NounPhrase.you),
              delay (Primitives.GameEvent.beginningOf .the .endStep thatTurns) (Primitives.Instruction.conclude .loseGame (agent := Primitives.NounPhrase.you)) ])
                  ] } }

def finalFortuneExtraTurn : Instruction := Primitives.Instruction.addTurn (.lit 1) (agent := Primitives.NounPhrase.you)
theorem okFinalFortuneExtraTurn : Instruction.check [] finalFortuneExtraTurn = [] := by decide
def finalFortuneThatTurn : NounPhrase := thatTurn
theorem okFinalFortuneThatTurn :
    NounPhrase.check (some .turnRef) (Instruction.intro [] finalFortuneExtraTurn)
      finalFortuneThatTurn = [] := by
  decide

def lastChance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Last Chance", cost := some [pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.addTurn (.lit 1) (agent := Primitives.NounPhrase.you),
              delay (Primitives.GameEvent.beginningOf .the .endStep thatTurns) (Primitives.Instruction.conclude .loseGame (agent := Primitives.NounPhrase.you)) ])
                  ] } }

def chanceForGlory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chance for Glory", cost := some [generic 1, pip .red, pip .white],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.establish
                (Primitives.StaticSpec.abilityGrant (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
                  (keyword "Indestructible"))
                none,
              Primitives.Instruction.addTurn (.lit 1) (agent := Primitives.NounPhrase.you),
              delay (Primitives.GameEvent.beginningOf .the .endStep thatTurns) (Primitives.Instruction.conclude .loseGame (agent := Primitives.NounPhrase.you)) ])
                  ] } }

def aggravatedAssault : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggravated Assault", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ activatedOnlyDuring (Primitives.Cost.mana [generic 3, pip .red, pip .red])
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.setStatus .untapped (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])),
                addPartThen .combat (some .mainPhase) (.lit 1) .mainPhase ])
            Primitives.Timing.asSorcery ] } }

def relentlessAssault : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Relentless Assault", cost := some [generic 2, pip .red, pip .red],
      types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.setStatus .untapped
                (allOf (Primitives.Predicate.and [creature, happenedTo (Primitives.GameEvent.combat
                  .attackerOf (Primitives.NounPhrase.asMarker .permanent (relative .object)) none)
                  .thisTurn])),
              addPartThen .combat (some .mainPhase) (.lit 1) .mainPhase ]) ] } }

def fullThrottleFirstLine : Instruction := addPart .combat (some .mainPhase) (.lit 2)
theorem okFullThrottleFirstLine : Instruction.check [] fullThrottleFirstLine = [] := by decide
def yshtolaAdditionalEndStep : Instruction := addPart .endStep none (.lit 1)
theorem okYshtolaAdditionalEndStep : Instruction.check [] yshtolaAdditionalEndStep = [] := by decide

/-- Sphinx of the Second Sun -/
def sphinxOfTheSecondSun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphinx of the Second Sun", cost := some [generic 6, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword "Flying",
          at_ (Primitives.GameEvent.beginningOf .each .postcombatMain (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (addPart .beginningPhase (some .postcombatMain) (.lit 1)) ],
      power := stat 6, toughness := stat 6 } }

/-- Obeka, Splitter of Seconds -/
def obekaSplitterOfSeconds : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Obeka, Splitter of Seconds",
      cost := some [generic 1, pip .blue, pip .black, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Ogre", creatureType "Warlock"],
      text :=
        [ keyword "Menace",
          whenever (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
            (getAdditionalPart .upkeep Primitives.Amount.thatMuch (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 5 } }

/-- Paradox Haze -/
def paradoxHaze : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Paradox Haze", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" Primitives.Predicate.anyPlayer,
          at_ (Primitives.GameEvent.nthOccurrence (.nth 1) (some .turn)
                (beginningOfPossessed .the .upkeep (Primitives.NounPhrase.attachHost .enchanted .player)))
            (getAdditionalPart .upkeep (.lit 1) (agent := (that .player))) ] } }

def ninthDoctorAdditionalUpkeep : Instruction := getAdditionalPart .upkeep (.lit 1) (agent := Primitives.NounPhrase.you)
theorem okNinthDoctorAdditionalUpkeep :
    Instruction.check [] ninthDoctorAdditionalUpkeep = [] := by decide

/-- Rites of Flourishing -/
def ritesOfFlourishing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rites of Flourishing", cost := some [generic 2, pip .green],
      types := [.enchantment],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .the .drawStep (Primitives.HeaderPossessor.byPlayer (each Primitives.Predicate.anyPlayer)))
            (Primitives.Instruction.draw (.lit 1) (agent := (that .player))),
          Primitives.Ability.static (mayPlayAdditionalLands (each Primitives.Predicate.anyPlayer) (exactly 1)) ] } }

def odricLunarchMarshal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Odric, Lunarch Marshal", cost := some [generic 3, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ Primitives.Ability.alsoForKeywords
            (triggeredIf (Primitives.GameEvent.beginningOf .the .combat (Primitives.HeaderPossessor.byPlayer (each Primitives.Predicate.anyPlayer)))
              (exists_ (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                               Primitives.Predicate.hasKeyword (.the "FirstStrike") ]))
              (Primitives.Instruction.establish (Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword "FirstStrike"))
                (some untilEndOfTurn)))
            (["Flying", "Deathtouch", "DoubleStrike", "Haste", "Hexproof", "Indestructible",
              "Lifelink", "Menace", "Reach", "Skulk", "Trample", "Vigilance"].map .the) ],
      power := stat 3, toughness := stat 3 } }

def bleedingEffect : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bleeding Effect", cost := some [generic 2, pip .white, pip .black],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.alsoForKeywords
            (triggeredIf (Primitives.GameEvent.beginningOf .the .combat (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
              (exists_ (Primitives.Predicate.and [ creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you),
                               Primitives.Predicate.hasKeyword (.the "Flying") ]))
              (Primitives.Instruction.establish (Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword "Flying"))
                (some untilEndOfTurn)))
            (["FirstStrike", "DoubleStrike", "Deathtouch", "Hexproof", "Indestructible",
              "Lifelink", "Menace", "Reach", "Trample", "Vigilance"].map .the) ] } }

def fettergeist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fettergeist", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (doUnless (sacrifice thisCreature (agent := Primitives.NounPhrase.you))
              (scaledMana .generic
                (forEach 1
                  (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you, Primitives.Predicate.otherThan thisCreature])))
                      (agent := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 4 } }

def chainVeilEndStep : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Condition.not (happened (Primitives.GameEvent.activates (relative .player) (a
      (Primitives.Predicate.and [Primitives.Predicate.abilityHead .loyalty,
      Primitives.Predicate.abilityOf (a (Primitives.Predicate.hasType .planeswalker))])))
      Primitives.NounPhrase.you .thisTurn))
    (loseLife (.lit 2) (agent := Primitives.NounPhrase.you))
theorem okChainVeilEndStep : Ability.check [] chainVeilEndStep = [] := by decide

def curseOfTheBloodyTome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curse of the Bloody Tome", cost := some [generic 2, pip .blue],
      types := [.enchantment], subtypes := [enchantmentType "Aura", enchantmentType "Curse"],
      text :=
        [ keywordSubject "Enchant" Primitives.Predicate.anyPlayer,
          at_ (beginningOfPossessed .the .upkeep (Primitives.NounPhrase.attachHost .enchanted .player))
            (mill (.lit 2) they (agent := (that .player))) ] } }

def shriekingAffliction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shrieking Affliction", cost := some [pip .black], types := [.enchantment],
      text :=
        [ triggeredIf (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer (each Primitives.Predicate.opponent)))
            (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.inZone (handOf (that .player)))) .atMost (.lit 1))
            (loseLife (.lit 3) (agent := they)) ] } }

def centaurOfAttention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Centaur of Attention", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Centaur", creatureType "Performer"],
      text :=
        [ when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequentially [rollDice 5 6 (agent := Primitives.NounPhrase.you), Primitives.Instruction.storeResults thisCreature]),
          at_ (Primitives.GameEvent.beginningOf .the .combat (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (offer (Primitives.Instruction.rerollStored anyNumber thisCreature (agent := Primitives.NounPhrase.you)) (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.modification thisCreature .power (Primitives.Delta.up (Primitives.Amount.letter .x)),
              Primitives.StaticSpec.modification thisCreature .toughness (Primitives.Delta.up (Primitives.Amount.letter .x)),
              Primitives.StaticSpec.letterDefinition .x (Primitives.Amount.greatestStoredMatch thisCreature) ]) ],
      power := stat 3, toughness := stat 3 } }

/-- Shapeshifter -/
def shapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shapeshifter", cost := some [generic 6], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ Primitives.Ability.static (entersChoosingFrom thisCreature .number (Primitives.ChoiceDomain.number (fromTo 0 7))),
          at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (offer (choose (a (qualityFrom .number (Primitives.ChoiceDomain.number (fromTo 0 7))))) (agent := Primitives.NounPhrase.you)),
          Primitives.Ability.static (Primitives.StaticSpec.conjunction none
            [ Primitives.StaticSpec.ptDefinition thisCreature .powerAlone theLastChosenNumber,
              Primitives.StaticSpec.ptDefinition thisCreature .toughnessAlone (minus (.lit 7) theLastChosenNumber) ]) ] }
                  }

def selfSacrificeThenExile : Ability :=
  activated (Primitives.Cost.perform (sacrifice thisArtifact (agent := Primitives.NounPhrase.you)))
    (Primitives.Instruction.sequentially
      [ exile (target creature),
        Primitives.Instruction.delay (Primitives.GameEvent.beginningOf .the .endStep Primitives.HeaderPossessor.noPossessor) [] none
          (Primitives.Instruction.move (that .card) (some battlefield) []) ])
theorem okSelfSacrificeThenExile : Ability.check [] selfSacrificeThenExile = [] := by decide

/-- Mirror Universe -/
def mirrorUniverse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirror Universe", cost := some [generic 6], types := [.artifact],
      text :=
        [ activatedOnlyDuring (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisArtifact (agent :=
            Primitives.NounPhrase.you))])
            (Primitives.Instruction.exchange (Primitives.Exchanged.lifeTotals (Primitives.NounPhrase.both Primitives.NounPhrase.you (target Primitives.Predicate.opponent))))
            (Primitives.Timing.duringPart .upkeep (some Primitives.NounPhrase.you)) ] } }

/-- Concerted Effort -/
def concertedEffort : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Concerted Effort", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ Primitives.Ability.alsoForKeywords
            (triggeredIf (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer (each Primitives.Predicate.anyPlayer)))
              (exists_ (Primitives.Predicate.and [ creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                               Primitives.Predicate.hasKeyword (.the "Flying") ]))
              (Primitives.Instruction.establish (Primitives.StaticSpec.abilityGrant (allOf creatureYouControl) (keyword "Flying"))
                (some untilEndOfTurn)))
            [ .the "Fear", .the "FirstStrike", .the "DoubleStrike",
              .anyIn ⟨"Landwalk", none⟩, .anyIn ⟨"Protection", none⟩,
              .the "Trample", .the "Vigilance" ] ] } }

/-- Land Tax -/
def landTax : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Land Tax", cost := some [pip .white], types := [.enchantment],
      text :=
        [ triggeredIf (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you)) (exists_ opponentWithMoreLands)
            (offer
              (Primitives.Instruction.sequentially
                [ searchLibraryFor (upTo 3) (Primitives.Predicate.and [land, Primitives.Predicate.hasSupertype .basic]),
                  revealCards (those .card),
                  move (those .card) hand,
                  shuffle ]) (agent := Primitives.NounPhrase.you)) ] } }

/-- Beckoning Will-o'-Wisp -/
def beckoningWillOWisp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Beckoning Will-o'-Wisp", cost := some [generic 2, pip .white],
      types := [.creature], subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          flavorWord "Lure the Unwary"
            (at_ (Primitives.GameEvent.beginningOf .the .combat (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you)) (choose (a Primitives.Predicate.opponent))),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.inCombat .attackerOf (some (the chosenPlayer))]))
            (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 0))) ],
      power := stat 1, toughness := stat 3 } }

/-- Triarch Stalker -/
def triarchStalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Triarch Stalker", cost := some [generic 3, pip .black, pip .black],
      types := [.artifact, .creature], subtypes := [creatureType "Necron"],
      text :=
        [ flavorWord "Targeting Relay"
            (at_ (Primitives.GameEvent.beginningOf .the .combat (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you)) (choose (a Primitives.Predicate.opponent))),
          Primitives.Ability.static (Primitives.StaticSpec.abilityGrant (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.inCombat .attackerOf (some (the
              chosenPlayer))]))
            (keyword "Menace")) ],
      power := stat 4, toughness := stat 5 } }

def raphaelAdditionalCombat : Instruction := addPart .combat (some .combat) (.lit 1)
theorem okRaphaelAdditionalCombat : Instruction.check [] raphaelAdditionalCombat = [] := by decide
/-- Karn Liberated -/
def karnRestart : Instruction := Primitives.Instruction.restartGame
theorem okKarnRestart : Instruction.check [] karnRestart = [] := by decide

def waxWane : Spelled := spelled <| .split
  { characteristics :=
    { name := "Wax", cost := some [pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (get (target creature) (Primitives.Delta.up (.lit 2)) (Primitives.Delta.up (.lit 2)) (some untilEndOfTurn)) ] } }
  { characteristics :=
    { name := "Wane", cost := some [pip .white], types := [.instant],
      text := [Primitives.Ability.spell none (destroy (target enchantment))] } }

/-- Teleport -/
def teleport : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Teleport", cost := some [pip .blue, pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell (some (Primitives.Timing.duringPart .declareAttackers none))
            (forbidBeingBlocked (target creature) (some Primitives.Duration.thisTurn)) ] } }

/-- Dazzling Beauty's cast window; its targeted effect and delayed "next turn's upkeep" draw
need machinery outside this ticket. -/
def dazzlingBeautyCastRestriction : Timing := Primitives.Timing.duringPart .declareBlockers none

/-- Thawing Glaciers -/
def thawingGlaciers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thawing Glaciers", types := [.land],
      text :=
        [ Primitives.Ability.static (entersTapped thisLand),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequentially
              [ searchLibraryFor (exactly 1) (Primitives.Predicate.and [land, Primitives.Predicate.hasSupertype .basic]),
                putOntoBattlefieldTapped (that .card),
                shuffle,
                delay (Primitives.GameEvent.beginningOf .the .cleanup Primitives.HeaderPossessor.noPossessor) (move thisLand hand) ]) ] } }

/-- Blood Frenzy -/
def bloodFrenzy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Frenzy", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell (some (Primitives.Timing.beforePart .combatDamage none))
            (Primitives.Instruction.sequentially
              [ get (target (Primitives.Predicate.and [creature, Primitives.Predicate.or [attacking, blocking]]))
                  (Primitives.Delta.up (.lit 4)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn),
                delay (Primitives.GameEvent.beginningOf .the .endStep Primitives.HeaderPossessor.noPossessor)
                  (destroy (that (.type .creature))) ]) ] } }

/-- Berserk -/
def berserk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Berserk", cost := some [pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell (some (Primitives.Timing.beforePart .combatDamage none))
            (Primitives.Instruction.sequentially
              [ establishFor (target creature)
                  [ Primitives.StaticSpec.abilityGrant (ownSubject (target creature)) (keyword "Trample"),
                    Primitives.StaticSpec.modification (ownSubject (target creature)) .power (Primitives.Delta.up (Primitives.Amount.letter .x)),
                    Primitives.StaticSpec.modification it .toughness (Primitives.Delta.up (.lit 0)),
                    Primitives.StaticSpec.letterDefinition .x (Primitives.Amount.statOf (.stat .power) it) ]
                  (some untilEndOfTurn),
                delay (Primitives.GameEvent.beginningOf .the .endStep Primitives.HeaderPossessor.noPossessor)
                  (Primitives.Instruction.doOnlyIf (destroy (that (.type .creature)))
                    (happened (Primitives.GameEvent.combat .attackerOf
                      (Primitives.NounPhrase.asMarker .permanent (relative .object)) none) (that
                      (.type .creature)) .thisTurn) none) ]) ] } }

end Semantics.Cards
