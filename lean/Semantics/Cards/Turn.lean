import Semantics
import Semantics.Definitions
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

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Through the Breach Splice -/
def throughTheBreach : Action :=
  may
    (putInto (a (and [isCreature, in_ (handOf you)])) battlefield)
    (some (sequentially
      [ gainHaste (that (.type .creature)) none,
        delay (beginningOfNext .endStep)
          (sacrifice (that (.type .creature))) ]))
theorem okThroughTheBreach : Action.check (DRS.ofList []) throughTheBreach = [] := by decide

def turnToMist : Action :=
  sequentially
    [ exile (target isCreature),
      delay (beginningOfNext .endStep) (putInto (that .card) battlefield) ]
theorem okTurnToMist : Action.check (DRS.ofList []) turnToMist = [] := by decide

def voyagerStaff : Ability :=
  activated (compound [mana [generic 2], perform (sacrifice thisArtifact)])
    (sequentially
      [ exile (target isCreature),
        delay (beginningOfNext .endStep)
          (putInto (theVerbed (.action "exile") .one (some .card)) battlefield) ])
theorem okVoyagerStaff : Ability.check (DRS.ofList []) voyagerStaff = [] := by decide

def staffOfNin : Ability :=
  at_ (beginningOfYour .upkeep) (draw (.lit 1))
theorem okStaffOfNin : Ability.check (DRS.ofList []) staffOfNin = [] := by decide

def silentAssassin : Ability :=
  activated (mana [generic 3, pip .black])
    (delay (beginningOfNext .endOfCombat)
      (destroy (target (and [isBlocking, isCreature]))))
theorem okSilentAssassin : Ability.check (DRS.ofList []) silentAssassin = [] := by decide

def kjeldoranFrostbeast : Ability :=
  at_ (beginningOfEach .endOfCombat)
    (destroy (allOf (and [ isCreature,
                            or [ inCombat .blockerOf (some thisCreature),
                                  inCombat .blockedBy (some thisCreature) ] ])))
theorem okKjeldoranFrostbeast : Ability.check (DRS.ofList []) kjeldoranFrostbeast = [] := by decide

/-- Tippy-Toe, Terrific Partner -/
def tippyToe : Ability :=
  triggeredIf (beginningOfYour .endStep) (happened (changeLife (up anyAmount)) (actor you) soFarThisTurn)
    (draw (.lit 1))
theorem okTippyToe : Ability.check (DRS.ofList []) tippyToe = [] := by decide

def brazenCannonade : Ability :=
  abilityWord "raid"
    (triggeredIf (beginningOfYour .postcombatMain)
      (happened attack (actor you) soFarThisTurn)
      (exile (topSlice (.lit 1))))
theorem okBrazenCannonade : Ability.check (DRS.ofList []) brazenCannonade = [] := by decide

def fourKnocks : Ability :=
  at_ (beginningOfYour .firstMain) (draw (.lit 1))
theorem okFourKnocks : Ability.check (DRS.ofList []) fourKnocks = [] := by decide

def hammerOfBogardan : Ability :=
  activatedOnlyDuring (mana [generic 2, pip .red, pip .red, pip .red]) (putInto «this» hand)
    (duringPart .upkeep (some you))
theorem okHammerOfBogardan : Ability.check (DRS.ofList []) hammerOfBogardan = [] := by decide

/-- Apathy -/
def apathy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Apathy", cost := some [pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isCreature,
          static (doesntUntap (attachHost .enchanted (.type .creature))
            (some (controllerOf (attachHost .enchanted (.type .creature))))),
          at_ (beginningOfPossessed .upkeep (controllerOf (attachHost .enchanted (.type .creature))))
            (instruct (that .player) (may (discard (aAtRandom (and [isCard, in_ (handOf (that .player))])))
              (some (untap (that (.type .creature)))))) ] } }

def felidarSovereign : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Felidar Sovereign", cost := some [generic 4, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Cat", creatureType "Beast"],
      text :=
        [ keyword (.named "vigilance"), keyword (.named "lifelink"),
          triggeredIf (beginningOfYour .upkeep)
            (compareAmt (lifeTotalOf you) .atLeast (.lit 40))
            winGame ],
      power := stat 4, toughness := stat 6 } }

def gloriousEnforcer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Glorious Enforcer", cost := some [generic 5, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Angel"],
      text :=
        [ keyword (.named "flying"), keyword (.named "lifelink"),
          triggeredIf (beginningOfPossessed .combat (each isPlayer))
            (compareAmt (lifeTotalOf you) .greater (lifeTotalOf anOpponent))
            (gain thisCreature (keyword (.named "doubleStrike")) (some untilEndOfTurn)) ],
      power := stat 5, toughness := stat 5 } }

/-- Damia, Sage of Stone -/
def damia : Ability :=
  triggeredIf (beginningOfYour .upkeep)
    (compareAmt (countOf (and [isCard, in_ (handOf you)])) .less (.lit 7))
    (draw theDifference)
theorem okDamia : Ability.check (DRS.ofList []) damia = [] := by decide

def ivoryTower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ivory Tower", cost := some [generic 1], types := [.artifact],
      text :=
        [ at_ (beginningOfYour .upkeep)
            (sequentially
              [ gainLife (letter .x),
                define .x (minus (countOf (and [isCard, in_ (handOf you)])) (.lit 4)) ]) ] } }

/-- Ajani, Adversary of Tyrants -/
def ajanisEmblem : Action :=
  getEmblem
    [ at_ (beginningOfYour .endStep)
        (createToken
          { characteristics :=
            { colors := [.white], types := [.creature], subtypes := [creatureType "Cat"],
              text := [keyword (.named "lifelink")], power := stat 1, toughness := stat 1 } } (count := (.lit 3))) ]
theorem okAjanisEmblem : Action.check (DRS.ofList []) ajanisEmblem = [] := by decide

/-- Saheeli Rai -/
def saheelisCopy : Action :=
  sequentially
    [ create (.lit 1)
        (copyOf (target (and [or [isArtifact, isCreature], hasPossessor .controller you]))
          [isAlso [.artifact] []])
        [],
      gainHaste (that .token) none,
      delay (beginningOfNext .endStep) (exile it) ]
theorem okSaheelisCopy : Action.check (DRS.ofList []) saheelisCopy = [] := by decide

def timeWalk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Time Walk", cost := some [generic 1, pip .blue], types := [.sorcery],
      text := [spell (extraTurn (.lit 1))] } }

def timeStretch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Time Stretch", cost := some [generic 8, pip .blue, pip .blue], types := [.sorcery],
      text := [spell (instruct (target isPlayer) (extraTurn (.lit 2)))] } }

def timeSieve : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Time Sieve", cost := some [pip .blue, pip .black], types := [.artifact],
      text :=
        [ activated (compound [tapSymbol, perform (sacrifice (counted (exactly 5) isArtifact))])
            (extraTurn (.lit 1)) ] } }

def finalFortune : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Final Fortune", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ spell (sequentially
            [ extraTurn (.lit 1),
              delay (beginningOfThatTurns .endStep) loseGame ])
                  ] } }

def finalFortuneExtraTurn : Action := extraTurn (.lit 1)
theorem okFinalFortuneExtraTurn : Action.check (DRS.ofList []) finalFortuneExtraTurn = [] := by decide
def finalFortuneThatTurn : TurnRef := thatTime
theorem okFinalFortuneThatTurn :
    TurnRef.check ((stepAction 512 .postEvent .obj (DRS.ofList []) finalFortuneExtraTurn).1) finalFortuneThatTurn = [] := by
  decide

def lastChance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Last Chance", cost := some [pip .red, pip .red], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ extraTurn (.lit 1),
              delay (beginningOfThatTurns .endStep) loseGame ])
                  ] } }

def chanceForGlory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chance for Glory", cost := some [generic 1, pip .red, pip .white],
      types := [.instant],
      text :=
        [ spell (sequentially
            [ establish
                (abilityGrant (allOf (and [isCreature, hasPossessor .controller you]))
                  (keyword (.named "indestructible"))),
              extraTurn (.lit 1),
              delay (beginningOfThatTurns .endStep) loseGame ])
                  ] } }

def aggravatedAssault : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aggravated Assault", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ activatedOnlyDuring (mana [generic 3, pip .red, pip .red])
            (sequentially
              [ setStatus .untapped (allOf (and [isCreature, hasPossessor .controller you])),
                additionalPhase .combat (some .mainPhase) (.lit 1) (some .mainPhase) ])
            asSorcery ] } }

def relentlessAssault : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Relentless Assault", cost := some [generic 2, pip .red, pip .red],
      types := [.sorcery],
      text :=
        [ spell (sequentially
            [ setStatus .untapped
                (allOf (and [isCreature, happenedTo (actor (asMarker .permanent (relative .object))) attack soFarThisTurn])),
              additionalPhase .combat (some .mainPhase) (.lit 1) (some .mainPhase) ]) ] } }

def fullThrottleFirstLine : Action := additionalPhase .combat (some .mainPhase) (.lit 2)
theorem okFullThrottleFirstLine : Action.check (DRS.ofList []) fullThrottleFirstLine = [] := by decide
def yshtolaAdditionalEndStep : Action := additionalStep .endStep none (.lit 1)
theorem okYshtolaAdditionalEndStep : Action.check (DRS.ofList []) yshtolaAdditionalEndStep = [] := by decide

/-- Sphinx of the Second Sun -/
def sphinxOfTheSecondSun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sphinx of the Second Sun", cost := some [generic 6, pip .blue, pip .blue],
      types := [.creature], subtypes := [creatureType "Sphinx"],
      text :=
        [ keyword (.named "flying"),
          at_ (beginningOfYour .postcombatMain)
            (additionalPhase .beginningPhase (some .postcombatMain) (.lit 1)) ],
      power := stat 6, toughness := stat 6 } }

/-- Obeka, Splitter of Seconds -/
def obekaSplitterOfSeconds : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Obeka, Splitter of Seconds",
      cost := some [generic 1, pip .blue, pip .black, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Ogre", creatureType "Warlock"],
      text :=
        [ keyword (.named "menace"),
          whenever (does (actor thisCreature) (dealDamage (to := a isPlayer) (kind := .combatOnly)))
            (additionalStep .upkeep none thatMuch) ],
      power := stat 2, toughness := stat 5 } }

/-- Paradox Haze -/
def paradoxHaze : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Paradox Haze", cost := some [generic 2, pip .blue], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject (.named "enchant") isPlayer,
          at_ (nthOccurrence (.nth 1) (beginningOfPossessed .upkeep (attachHost .enchanted .player)) (some .turn))
            (instruct (that .player) (additionalStep .upkeep none (.lit 1))) ] } }

def ninthDoctorAdditionalUpkeep : Action := additionalStep .upkeep none (.lit 1)
theorem okNinthDoctorAdditionalUpkeep :
    Action.check (DRS.ofList []) ninthDoctorAdditionalUpkeep = [] := by decide

/-- Rites of Flourishing -/
def ritesOfFlourishing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rites of Flourishing", cost := some [generic 2, pip .green],
      types := [.enchantment],
      text :=
        [ at_ (beginningOfPossessed .drawStep (each isPlayer))
            (instruct (that .player) (draw (.lit 1))),
          static (can (actor eachPlayer) (play (counted (additional (exactly 1)) isLand))
            (timing := some onEachOfTheirTurns)) ] } }

def odricLunarchMarshal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Odric, Lunarch Marshal", cost := some [generic 3, pip .white],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ alsoForKeywords
            (triggeredIf (beginningOfPossessed .combat (each isPlayer))
              (exists_ (and [ isCreature, hasPossessor .controller you,
                               hasKeyword (.the (.named "firstStrike")) ]))
              (establish (abilityGrant (allOf (and [isCreature, youControl])) (keyword (.named "firstStrike")))
                (some untilEndOfTurn)))
            (["flying", "deathtouch", "doubleStrike", "haste", "hexproof", "indestructible",
              "lifelink", "menace", "reach", "skulk", "trample", "vigilance"].map (fun k => .the (.named k))) ],
      power := stat 3, toughness := stat 3 } }

def bleedingEffect : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bleeding Effect", cost := some [generic 2, pip .white, pip .black],
      types := [.enchantment],
      text :=
        [ alsoForKeywords
            (triggeredIf (beginningOfYour .combat)
              (exists_ (and [ isCreature, in_ (graveyardOf you),
                               hasKeyword (.the (.named "flying")) ]))
              (establish (abilityGrant (allOf (and [isCreature, youControl])) (keyword (.named "flying")))
                (some untilEndOfTurn)))
            (["firstStrike", "doubleStrike", "deathtouch", "hexproof", "indestructible",
              "lifelink", "menace", "reach", "trample", "vigilance"].map (fun k => .the (.named k))) ] } }

def fettergeist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fettergeist", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Spirit"],
      text :=
        [ keyword (.named "flying"),
          at_ (beginningOfYour .upkeep)
            (doUnless (sacrifice thisCreature)
              (scaledGenericMana
                (forEach (.lit 1)
                  (and [isCreature, hasPossessor .controller you, otherThan thisCreature])))) ],
      power := stat 3, toughness := stat 4 } }

def chainVeilEndStep : Ability :=
  triggeredIf (beginningOfYour .endStep)
    (not (happened (procedure .activate (a
      (and [abilityHead .loyalty,
      abilityOf (a (hasType .planeswalker))])) itsOwnCost) (actor you) soFarThisTurn))
    (loseLife (.lit 2))
theorem okChainVeilEndStep : Ability.check (DRS.ofList []) chainVeilEndStep = [] := by decide

def curseOfTheBloodyTome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Curse of the Bloody Tome", cost := some [generic 2, pip .blue],
      types := [.enchantment], subtypes := [enchantmentType "Aura", enchantmentType "Curse"],
      text :=
        [ keywordSubject (.named "enchant") isPlayer,
          at_ (beginningOfPossessed .upkeep (attachHost .enchanted .player))
            (instruct (that .player) (mill (.lit 2) they)) ] } }

def shriekingAffliction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shrieking Affliction", cost := some [pip .black], types := [.enchantment],
      text :=
        [ triggeredIf (beginningOfPossessed .upkeep (each (isOpponentOf you)))
            (compareAmt (countOf (and [isCard, in_ (handOf (that .player))])) .atMost (.lit 1))
            (instruct they (loseLife (.lit 3))) ] } }

def centaurOfAttention : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Centaur of Attention", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [creatureType "Centaur", creatureType "Performer"],
      text :=
        [ when (does (actor thisCreature) enter)
            (sequentially [rollDice (.lit 5) (.sides 6), storeResults thisCreature]),
          at_ (beginningOfYour .combat)
            (may (rerollStored anyNumber thisCreature)),
          static (conjunction [ getsPt thisCreature (up (letter .x)) (up (letter .x)),
              letterDefinition .x (greatestStoredMatch thisCreature) ]) ],
      power := stat 3, toughness := stat 3 } }

/-- Shapeshifter -/
def shapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shapeshifter", cost := some [generic 6], types := [.artifact, .creature],
      subtypes := [creatureType "Shapeshifter"],
      text :=
        [ static (entersChoosingFrom thisCreature .number (number (fromTo 0 7))),
          at_ (beginningOfYour .upkeep)
            (may (choose (a (qualityFrom .number (number (fromTo 0 7)))))),
          static (conjunction [ ptDefinition thisCreature .powerAlone theLastChosenNumber,
              ptDefinition thisCreature .toughnessAlone (minus (.lit 7) theLastChosenNumber) ]) ] }
                  }

def selfSacrificeThenExile : Ability :=
  activated (perform (sacrifice thisArtifact))
    (sequentially
      [ exile (target isCreature),
        delay (beginningOfNext .endStep) (move (that .card) any (some battlefield) .arrival) ])
theorem okSelfSacrificeThenExile : Ability.check (DRS.ofList []) selfSacrificeThenExile = [] := by decide

/-- Mirror Universe -/
def mirrorUniverse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirror Universe", cost := some [generic 6], types := [.artifact],
      text :=
        [ activatedOnlyDuring (compound [tapSymbol, perform (sacrifice thisArtifact)])
            (exchange (lifeTotals (both you (target (isOpponentOf you)))))
            (duringPart .upkeep (some you)) ] } }

/-- Concerted Effort -/
def concertedEffort : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Concerted Effort", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ alsoForKeywords
            (triggeredIf (beginningOfPossessed .upkeep (each isPlayer))
              (exists_ (and [ isCreature, hasPossessor .controller you,
                               hasKeyword (.the (.named "flying")) ]))
              (establish (abilityGrant (allOf (and [isCreature, youControl])) (keyword (.named "flying")))
                (some untilEndOfTurn)))
            [ .the (.named "fear"), .the (.named "firstStrike"), .the (.named "doubleStrike"),
              .anyIn ⟨(.named "landwalk"), none⟩, .anyIn ⟨(.named "protection"), none⟩,
              .the (.named "trample"), .the (.named "vigilance") ] ] } }

/-- Land Tax -/
def landTax : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Land Tax", cost := some [pip .white], types := [.enchantment],
      text :=
        [ triggeredIf (beginningOfYour .upkeep) (exists_ opponentWithMoreLands)
            (may
              (sequentially
                [ searchLibraryFor (upTo 3) (and [isLand, hasSupertype .basic]),
                  reveal (those .card),
                  putInto (those .card) hand,
                  shuffle ])) ] } }

/-- Beckoning Will-o'-Wisp -/
def beckoningWillOWisp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Beckoning Will-o'-Wisp", cost := some [generic 2, pip .white],
      types := [.creature], subtypes := [creatureType "Spirit"],
      text :=
        [ keyword (.named "flying"),
          flavorWord "Lure the Unwary"
            (at_ (beginningOfYour .combat) (choose (a (isOpponentOf you)))),
          static (getsPt (allOf (and [isCreature, inCombat .attackerOf (some (the isChosenPlayer))]))
            (up (.lit 1)) (up (.lit 0))) ],
      power := stat 1, toughness := stat 3 } }

/-- Triarch Stalker -/
def triarchStalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Triarch Stalker", cost := some [generic 3, pip .black, pip .black],
      types := [.artifact, .creature], subtypes := [creatureType "Necron"],
      text :=
        [ flavorWord "Targeting Relay"
            (at_ (beginningOfYour .combat) (choose (a (isOpponentOf you)))),
          static (abilityGrant (allOf (and [isCreature, inCombat .attackerOf (some (the
              isChosenPlayer))]))
            (keyword (.named "menace"))) ],
      power := stat 4, toughness := stat 5 } }

def raphaelAdditionalCombat : Action := additionalPhase .combat (some .combat) (.lit 1)
theorem okRaphaelAdditionalCombat : Action.check (DRS.ofList []) raphaelAdditionalCombat = [] := by decide
/-- Karn Liberated -/
def karnRestart : Action := restartGame
theorem okKarnRestart : Action.check (DRS.ofList []) karnRestart = [] := by decide

def waxWane : Spelled := spelled <| .split
  { characteristics :=
    { name := "Wax", cost := some [pip .green], types := [.instant],
      text :=
        [ spell (get (target isCreature) (up (.lit 2)) (up (.lit 2)) (some untilEndOfTurn)) ] } }
  { characteristics :=
    { name := "Wane", cost := some [pip .white], types := [.instant],
      text := [spell (destroy (target isEnchantment))] } }

/-- Teleport -/
def teleport : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Teleport", cost := some [pip .blue, pip .blue, pip .blue], types := [.instant],
      text :=
        [ spell (establish (cant any (block (target isCreature))) (some thisTurn)) (timing := (some (duringPart .declareAttackers none))) ] } }

/-- Dazzling Beauty's cast window; its targeted effect and delayed "next turn's upkeep" draw
need machinery outside this ticket. -/
def dazzlingBeautyCastRestriction : Timing := duringPart .declareBlockers none

/-- Thawing Glaciers -/
def thawingGlaciers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thawing Glaciers", types := [.land],
      text :=
        [ static (entersTapped thisLand),
          activated (compound [mana [generic 1], tapSymbol])
            (sequentially
              [ searchLibraryFor (exactly 1) (and [isLand, hasSupertype .basic]),
                putOntoBattlefieldTapped (that .card),
                shuffle,
                delay (beginningOfNext .cleanup) (putInto thisLand hand) ]) ] } }

/-- Blood Frenzy -/
def bloodFrenzy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Frenzy", cost := some [generic 1, pip .red], types := [.instant],
      text :=
        [ spell (sequentially
              [ get (target (and [isCreature, or [isAttacking, isBlocking]]))
                  (up (.lit 4)) (up (.lit 0)) (some untilEndOfTurn),
                delay (beginningOfNext .endStep)
                  (destroy (that (.type .creature))) ]) (timing := (some (beforePart .combatDamage none))) ] } }

/-- Berserk -/
def berserk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Berserk", cost := some [pip .green], types := [.instant],
      text :=
        [ spell (sequentially
              [ establishFor (target isCreature)
                  [ abilityGrant ((target isCreature)) (keyword (.named "trample")),
                    modification ((target isCreature)) .power (up (letter .x)),
                    modification it .toughness (up (.lit 0)),
                    letterDefinition .x (statOf (.stat .power) it) ]
                  (some untilEndOfTurn),
                delay (beginningOfNext .endStep)
                  (doOnlyIf (destroy (that (.type .creature)))
                    (happened attack (actor (that
                      (.type .creature))) soFarThisTurn)) ]) (timing := (some (beforePart .combatDamage none))) ] } }

end Semantics.Cards
