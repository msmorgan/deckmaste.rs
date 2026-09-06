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
  .sequence
    [ offer (move (a (.and [creature, .inZone (handOf .you)])) battlefield) (agent := .you),
      gainHaste (that (.type .creature)) none,
      delay (.beginningOf .the .endStep .noPossessor)
        (sacrifice (that (.type .creature)) (agent := .you)) ]
theorem okThroughTheBreach : Instruction.check [] throughTheBreach = [] := by decide

def turnToMist : Instruction :=
  .sequence
    [ exile (target creature),
      delay (.beginningOf .the .endStep .noPossessor) (move (that .card) battlefield) ]
theorem okTurnToMist : Instruction.check [] turnToMist = [] := by decide

def voyagerStaff : Ability :=
  activated (.compound [.mana [generic 2], .perform (sacrifice thisArtifact (agent := .you))])
    (.sequence
      [ exile (target creature),
        delay (.beginningOf .the .endStep .noPossessor)
          (move (theVerbed (.action "Exile") .card .attributive .one) battlefield) ])
theorem okVoyagerStaff : Ability.check [] voyagerStaff = [] := by decide

def staffOfNin : Ability :=
  at_ (.beginningOf .the .upkeep (.byPlayer .you)) (.draw (.lit 1) (agent := .you))
theorem okStaffOfNin : Ability.check [] staffOfNin = [] := by decide

def silentAssassin : Ability :=
  activated (.mana [generic 3, pip .black])
    (delay (.beginningOf .the .endOfCombat .noPossessor)
      (destroy (target (.and [blocking, creature]))))
theorem okSilentAssassin : Ability.check [] silentAssassin = [] := by decide

def kjeldoranFrostbeast : Ability :=
  at_ (.beginningOf .the .endOfCombat .noPossessor)
    (destroy (allOf (.and [ creature,
                            .or [ .inCombat .blockerOf (some thisCreature),
                                  .inCombat .blockedBy (some thisCreature) ] ])))
theorem okKjeldoranFrostbeast : Ability.check [] kjeldoranFrostbeast = [] := by decide

/-- Tippy-Toe, Terrific Partner -/
def tippyToe : Ability :=
  triggeredIf (.beginningOf .the .endStep (.byPlayer .you)) (happened .lifeGain .you .thisTurn)
    (.draw (.lit 1) (agent := .you))
theorem okTippyToe : Ability.check [] tippyToe = [] := by decide

def brazenCannonade : Ability :=
  abilityWord "raid"
    (triggeredIf (.beginningOf .each .postcombatMain (.byPlayer .you))
      (happened .attackDeclaration .you .thisTurn)
      (exile (topSlice (.lit 1))))
theorem okBrazenCannonade : Ability.check [] brazenCannonade = [] := by decide

def fourKnocks : Ability :=
  at_ (.beginningOf .the .firstMain (.byPlayer .you)) (.draw (.lit 1) (agent := .you))
theorem okFourKnocks : Ability.check [] fourKnocks = [] := by decide

def hammerOfBogardan : Ability :=
  activatedOnlyDuring (.mana [generic 2, pip .red, pip .red, pip .red]) (move .this hand)
    (.duringPart .upkeep (some .you))
theorem okHammerOfBogardan : Ability.check [] hammerOfBogardan = [] := by decide

/-- Apathy -/
def apathy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Apathy", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" creature,
          .static (doesntUntap (.attachHost .enchanted (.type .creature))
            (some (controllerOf (.attachHost .enchanted (.type .creature))))),
          at_ (beginningOfPossessed .the .upkeep
                (controllerOf (.attachHost .enchanted (.type .creature))))
            (.offer (discard (aAtRandom (.inZone hand)) (agent := (that .player)))
              (some (untap (that (.type .creature)))) none (agent := (that .player))) ] } }

def felidarSovereign : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Felidar Sovereign", cost := some [generic 4, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Cat", creatureType "Beast"],
      text :=
        [ keyword "Vigilance", keyword "Lifelink",
          triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
            (.compareAmt (lifeTotalOf .you) .atLeast (.lit 40))
            (.conclude .winGame (agent := .you)) ],
      power := stat 4, toughness := stat 6 } }

def gloriousEnforcer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glorious Enforcer", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword "Flying", keyword "Lifelink",
          triggeredIf (.beginningOf .the .combat (.byPlayer (each .anyPlayer)))
            (.compareAmt (lifeTotalOf .you) .greater (lifeTotalOf anOpponent))
            (gain thisCreature (keyword "DoubleStrike") (some untilEndOfTurn)) ],
      power := stat 5, toughness := stat 5 } }

/-- Damia, Sage of Stone -/
def damia : Ability :=
  triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
    (.compareAmt (countOf (.inZone (handOf .you))) .less (.lit 7))
    (.draw .theDifference (agent := .you))
theorem okDamia : Ability.check [] damia = [] := by decide

def ivoryTower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ivory Tower", cost := some [generic 1], types := [.artifact],
      text :=
        [ at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (.sequence
              [ gainLife (.letter .x) (agent := .you),
                .define .x (minus (countOf (.inZone (handOf .you))) (.lit 4)) ]) ] } }

/-- Ajani, Adversary of Tyrants -/
def ajanisEmblem : Instruction :=
  .getEmblem
    [ at_ (.beginningOf .the .endStep (.byPlayer .you))
        (create (.lit 3)
          { characteristics :=
            { colors := [.white], types := [.creature], subtypes := [creatureType "Cat"],
              text := [keyword "Lifelink"], power := stat 1, toughness := stat 1 } }) ] (agent :=
                  .you)
theorem okAjanisEmblem : Instruction.check [] ajanisEmblem = [] := by decide

/-- Saheeli Rai -/
def saheelisCopy : Instruction :=
  .sequence
    [ .create (.lit 1)
        (.copyOf (target (.and [.or [artifact, creature], .hasPossessor .controller .you]))
          [.types [.artifact] []])
        [] (agent := .you),
      gainHaste (that .token) none,
      delay (.beginningOf .the .endStep .noPossessor) (exile it) ]
theorem okSaheelisCopy : Instruction.check [] saheelisCopy = [] := by decide

def timeWalk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Time Walk", cost := some [generic 1, pip .blue], types := [.sorcery],
      text := [.spell none (.addTurn (.lit 1) (agent := .you))] } }

def timeStretch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Time Stretch", cost := some [generic 8, pip .blue, pip .blue], types := [.sorcery],
      text := [.spell none (.addTurn (.lit 2) (agent := (target .anyPlayer)))] } }

def timeSieve : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Time Sieve", cost := some [pip .blue, pip .black], types := [.artifact],
      text :=
        [ activated (.compound [.tapSymbol, .perform (sacrifice (counted (exactly 5) artifact)
            (agent := .you))])
            (.addTurn (.lit 1) (agent := .you)) ] } }

def finalFortune : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Final Fortune", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .addTurn (.lit 1) (agent := .you),
              delay (.beginningOf .the .endStep thatTurns) (.conclude .loseGame (agent := .you)) ])
                  ] } }

def finalFortuneExtraTurn : Instruction := .addTurn (.lit 1) (agent := .you)
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
        [ .spell none (.sequence
            [ .addTurn (.lit 1) (agent := .you),
              delay (.beginningOf .the .endStep thatTurns) (.conclude .loseGame (agent := .you)) ])
                  ] } }

def chanceForGlory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chance for Glory", cost := some [generic 1, pip .red, pip .white],
      types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .establish
                (.abilityGrant (allOf (.and [creature, .hasPossessor .controller .you]))
                  (keyword "Indestructible"))
                none,
              .addTurn (.lit 1) (agent := .you),
              delay (.beginningOf .the .endStep thatTurns) (.conclude .loseGame (agent := .you)) ])
                  ] } }

def aggravatedAssault : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggravated Assault", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ activatedOnlyDuring (.mana [generic 3, pip .red, pip .red])
            (.sequence
              [ .setStatus .untapped (allOf (.and [creature, .hasPossessor .controller .you])),
                addPartThen .combat (some .mainPhase) (.lit 1) .mainPhase ])
            .asSorcery ] } }

def relentlessAssault : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Relentless Assault", cost := some [generic 2, pip .red, pip .red],
      types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ .setStatus .untapped
                (allOf (.and [creature, happenedTo .attackDeclaration .thisTurn])),
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
          at_ (.beginningOf .each .postcombatMain (.byPlayer .you))
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
          whenever (dealsCombatDamage thisCreature (a .anyPlayer))
            (getAdditionalPart .upkeep .thatMuch (agent := .you)) ],
      power := stat 2, toughness := stat 5 } }

/-- Paradox Haze -/
def paradoxHaze : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Paradox Haze", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" .anyPlayer,
          at_ (.nthOccurrence (.nth 1) (some .turn)
                (beginningOfPossessed .the .upkeep (.attachHost .enchanted .player)))
            (getAdditionalPart .upkeep (.lit 1) (agent := (that .player))) ] } }

def ninthDoctorAdditionalUpkeep : Instruction := getAdditionalPart .upkeep (.lit 1) (agent := .you)
theorem okNinthDoctorAdditionalUpkeep :
    Instruction.check [] ninthDoctorAdditionalUpkeep = [] := by decide

/-- Rites of Flourishing -/
def ritesOfFlourishing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rites of Flourishing", cost := some [generic 2, pip .green],
      types := [.enchantment],
      text :=
        [ at_ (.beginningOf .the .drawStep (.byPlayer (each .anyPlayer)))
            (.draw (.lit 1) (agent := (that .player))),
          .static (mayPlayAdditionalLands (each .anyPlayer) (exactly 1)) ] } }

def odricLunarchMarshal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Odric, Lunarch Marshal", cost := some [generic 3, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ .alsoForKeywords
            (triggeredIf (.beginningOf .the .combat (.byPlayer (each .anyPlayer)))
              (exists_ (.and [ creature, .hasPossessor .controller .you,
                               .hasKeyword (.the "FirstStrike") ]))
              (.establish (.abilityGrant (allOf creatureYouControl) (keyword "FirstStrike"))
                (some untilEndOfTurn)))
            (["Flying", "Deathtouch", "DoubleStrike", "Haste", "Hexproof", "Indestructible",
              "Lifelink", "Menace", "Reach", "Skulk", "Trample", "Vigilance"].map .the) ],
      power := stat 3, toughness := stat 3 } }

def bleedingEffect : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bleeding Effect", cost := some [generic 2, pip .white, pip .black],
      types := [.enchantment],
      text :=
        [ .alsoForKeywords
            (triggeredIf (.beginningOf .the .combat (.byPlayer .you))
              (exists_ (.and [ creature, .inZone (graveyardOf .you),
                               .hasKeyword (.the "Flying") ]))
              (.establish (.abilityGrant (allOf creatureYouControl) (keyword "Flying"))
                (some untilEndOfTurn)))
            (["FirstStrike", "DoubleStrike", "Deathtouch", "Hexproof", "Indestructible",
              "Lifelink", "Menace", "Reach", "Trample", "Vigilance"].map .the) ] } }

def fettergeist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fettergeist", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (doUnless (sacrifice thisCreature (agent := .you))
              (scaledMana .generic
                (forEach 1
                  (.and [creature, .hasPossessor .controller .you, .otherThan thisCreature])))
                      (agent := .you)) ],
      power := stat 3, toughness := stat 4 } }

def chainVeilEndStep : Ability :=
  triggeredIf (.beginningOf .the .endStep (.byPlayer .you))
    (.not (happenedInvolving .abilityActivation .you .thisTurn
      (a (.and [.abilityHead .loyalty, .abilityOf (a (.hasType .planeswalker))]))))
    (loseLife (.lit 2) (agent := .you))
theorem okChainVeilEndStep : Ability.check [] chainVeilEndStep = [] := by decide

def curseOfTheBloodyTome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curse of the Bloody Tome", cost := some [generic 2, pip .blue],
      types := [.enchantment], subtypes := [enchantmentType "Aura", enchantmentType "Curse"],
      text :=
        [ keywordSubject "Enchant" .anyPlayer,
          at_ (beginningOfPossessed .the .upkeep (.attachHost .enchanted .player))
            (mill (.lit 2) they (agent := (that .player))) ] } }

def shriekingAffliction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shrieking Affliction", cost := some [pip .black], types := [.enchantment],
      text :=
        [ triggeredIf (.beginningOf .the .upkeep (.byPlayer (each .opponent)))
            (.compareAmt (countOf (.inZone (handOf (that .player)))) .atMost (.lit 1))
            (loseLife (.lit 3) (agent := they)) ] } }

def centaurOfAttention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Centaur of Attention", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Centaur", creatureType "Performer"],
      text :=
        [ when (.enters thisCreature none)
            (.sequence [rollDice 5 6 (agent := .you), .storeResults thisCreature]),
          at_ (.beginningOf .the .combat (.byPlayer .you))
            (offer (.rerollStored anyNumber thisCreature (agent := .you)) (agent := .you)),
          .static (.conjunction none
            [ .modification thisCreature .power (.up (.letter .x)),
              .modification thisCreature .toughness (.up (.letter .x)),
              .letterDefinition .x (.greatestStoredMatch thisCreature) ]) ],
      power := stat 3, toughness := stat 3 } }

/-- Shapeshifter -/
def shapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shapeshifter", cost := some [generic 6], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ .static (entersChoosingFrom thisCreature .number (.number (fromTo 0 7))),
          at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (offer (choose (a (qualityFrom .number (.number (fromTo 0 7))))) (agent := .you)),
          .static (.conjunction none
            [ .ptDefinition thisCreature .powerAlone theLastChosenNumber,
              .ptDefinition thisCreature .toughnessAlone (minus (.lit 7) theLastChosenNumber) ]) ] }
                  }

def selfSacrificeThenExile : Ability :=
  activated (.perform (sacrifice thisArtifact (agent := .you)))
    (.sequence
      [ exile (target creature),
        .delay (.beginningOf .the .endStep .noPossessor) [] none
          (.move (that .card) battlefield []) ])
theorem okSelfSacrificeThenExile : Ability.check [] selfSacrificeThenExile = [] := by decide

/-- Mirror Universe -/
def mirrorUniverse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirror Universe", cost := some [generic 6], types := [.artifact],
      text :=
        [ activatedOnlyDuring (.compound [.tapSymbol, .perform (sacrifice thisArtifact (agent :=
            .you))])
            (.exchange (.lifeTotals (.both .you (target .opponent))))
            (.duringPart .upkeep (some .you)) ] } }

/-- Concerted Effort -/
def concertedEffort : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Concerted Effort", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ .alsoForKeywords
            (triggeredIf (.beginningOf .the .upkeep (.byPlayer (each .anyPlayer)))
              (exists_ (.and [ creature, .hasPossessor .controller .you,
                               .hasKeyword (.the "Flying") ]))
              (.establish (.abilityGrant (allOf creatureYouControl) (keyword "Flying"))
                (some untilEndOfTurn)))
            [ .the "Fear", .the "FirstStrike", .the "DoubleStrike",
              .anyIn ⟨"Landwalk", none⟩, .anyIn ⟨"Protection", none⟩,
              .the "Trample", .the "Vigilance" ] ] } }

/-- Land Tax -/
def landTax : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Land Tax", cost := some [pip .white], types := [.enchantment],
      text :=
        [ triggeredIf (.beginningOf .the .upkeep (.byPlayer .you)) (exists_ opponentWithMoreLands)
            (offer
              (.sequence
                [ searchLibraryFor (upTo 3) (.and [land, .hasSupertype .basic]),
                  revealCards (those .card),
                  move (those .card) hand,
                  shuffle ]) (agent := .you)) ] } }

/-- Beckoning Will-o'-Wisp -/
def beckoningWillOWisp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Beckoning Will-o'-Wisp", cost := some [generic 2, pip .white],
      types := [.creature], subtypes := [creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          flavorWord "Lure the Unwary"
            (at_ (.beginningOf .the .combat (.byPlayer .you)) (choose (a .opponent))),
          .static (getsPt (allOf (.and [creature, .inCombat .attackerOf (some (the chosenPlayer))]))
            (.up (.lit 1)) (.up (.lit 0))) ],
      power := stat 1, toughness := stat 3 } }

/-- Triarch Stalker -/
def triarchStalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Triarch Stalker", cost := some [generic 3, pip .black, pip .black],
      types := [.artifact, .creature], subtypes := [creatureType "Necron"],
      text :=
        [ flavorWord "Targeting Relay"
            (at_ (.beginningOf .the .combat (.byPlayer .you)) (choose (a .opponent))),
          .static (.abilityGrant (allOf (.and [creature, .inCombat .attackerOf (some (the
              chosenPlayer))]))
            (keyword "Menace")) ],
      power := stat 4, toughness := stat 5 } }

def raphaelAdditionalCombat : Instruction := addPart .combat (some .combat) (.lit 1)
theorem okRaphaelAdditionalCombat : Instruction.check [] raphaelAdditionalCombat = [] := by decide
/-- Karn Liberated -/
def karnRestart : Instruction := .restartGame
theorem okKarnRestart : Instruction.check [] karnRestart = [] := by decide

def waxWane : Spelled := spelled <| .split
  { characteristics :=
    { name := "Wax", cost := some [pip .green], types := [.instant],
      text :=
        [ .spell none
            (get (target creature) (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn)) ] } }
  { characteristics :=
    { name := "Wane", cost := some [pip .white], types := [.instant],
      text := [.spell none (destroy (target enchantment))] } }

/-- Teleport -/
def teleport : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Teleport", cost := some [pip .blue, pip .blue, pip .blue], types := [.instant],
      text :=
        [ .spell (some (.duringPart .declareAttackers none))
            (forbidBeingBlocked (target creature) (some .thisTurn)) ] } }

/-- Dazzling Beauty's cast window; its targeted effect and delayed "next turn's upkeep" draw
need machinery outside this ticket. -/
def dazzlingBeautyCastRestriction : Timing := .duringPart .declareBlockers none

/-- Thawing Glaciers -/
def thawingGlaciers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thawing Glaciers", types := [.land],
      text :=
        [ .static (entersTapped thisLand),
          activated (.compound [.mana [generic 1], .tapSymbol])
            (.sequence
              [ searchLibraryFor (exactly 1) (.and [land, .hasSupertype .basic]),
                putOntoBattlefieldTapped (that .card),
                shuffle,
                delay (.beginningOf .the .cleanup .noPossessor) (move thisLand hand) ]) ] } }

/-- Blood Frenzy -/
def bloodFrenzy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Frenzy", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ .spell (some (.beforePart .combatDamage none))
            (.sequence
              [ get (target (.and [creature, .or [attacking, blocking]]))
                  (.up (.lit 4)) (.up (.lit 0)) (some untilEndOfTurn),
                delay (.beginningOf .the .endStep .noPossessor)
                  (destroy (that (.type .creature))) ]) ] } }

/-- Berserk -/
def berserk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Berserk", cost := some [pip .green], types := [.instant],
      text :=
        [ .spell (some (.beforePart .combatDamage none))
            (.sequence
              [ establishFor (target creature)
                  [ .abilityGrant (ownSubject (target creature)) (keyword "Trample"),
                    .modification (ownSubject (target creature)) .power (.up (.letter .x)),
                    .modification it .toughness (.up (.lit 0)),
                    .letterDefinition .x (.statOf (.stat .power) it) ]
                  (some untilEndOfTurn),
                delay (.beginningOf .the .endStep .noPossessor)
                  (.doOnlyIf (destroy (that (.type .creature)))
                    (happened .attackDeclaration (that (.type .creature)) .thisTurn) none) ]) ] } }

end Semantics.Cards
