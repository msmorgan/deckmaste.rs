import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Keyword

/-!
# Semantics.Cards.Turn

Port of `idris/src/Experimental/Cards/Turn.idr`: the printed cards of the Turn family (turn
parts, extra turns, additional phases, cast windows) and the bench items beside them.

Not ported: `fracturedPowerstonePlanarRoll` (the planar die is Planechase, out of scope).
`shapeshifter`'s printed `*`/`7-*` box is `none` under the printed-star ruling; its
Characteristic-Defining Ability sets it.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Through the Breach Splice -/
def throughTheBreach : Action :=
  may
    (putOntoBattlefield (a (and [isCreature, in_ yourHand])))
    (sequentially
      [ gainHaste (that creature),
        atNext .endStep (sacrifice (that creature)) ])
theorem okThroughTheBreach : Action.check (DRS.ofList []) throughTheBreach = [] := by decide

def turnToMist : Action :=
  sequentially
    [ exile (target isCreature),
      atNext .endStep (returnToBattlefield (that card)) ]
theorem okTurnToMist : Action.check (DRS.ofList []) turnToMist = [] := by decide

def voyagerStaff : Ability :=
  activated (compound [[generic 2], sacrifice thisArtifact])
    (sequentially
      [ exile (target isCreature),
        atNext .endStep (returnToBattlefield (the (and [was (exile which), isCard]))) ])
theorem okVoyagerStaff : Ability.check (DRS.ofList []) voyagerStaff = [] := by decide

def staffOfNin : Ability :=
  at_ (beginningOfYour .upkeep) (draw 1)
theorem okStaffOfNin : Ability.check (DRS.ofList []) staffOfNin = [] := by decide

def silentAssassin : Ability :=
  activated [generic 3, pip .black]
    (atNext .endOfCombat (destroy (target (and [isBlocking, isCreature]))))
theorem okSilentAssassin : Ability.check (DRS.ofList []) silentAssassin = [] := by decide

def kjeldoranFrostbeast : Ability :=
  at_ (beginningOfEach .endOfCombat)
    (destroy (allOf (and [ isCreature,
                            or [ inCombat .blockerOf thisCreature,
                                  inCombat .blockedBy thisCreature ] ])))
theorem okKjeldoranFrostbeast : Ability.check (DRS.ofList []) kjeldoranFrostbeast = [] := by decide

/-- Tippy-Toe, Terrific Partner -/
def tippyToe : Ability :=
  at_ (beginningOfYour .endStep) (if_ := some (did you (gainLife anyAmount) soFarThisTurn))
    (draw 1)
theorem okTippyToe : Ability.check (DRS.ofList []) tippyToe = [] := by decide

def brazenCannonade : Ability :=
  raid
    (at_ (beginningOfYour .postcombatMain)
      (if_ := some (did you attack soFarThisTurn))
      (exile (topSlice 1)))
theorem okBrazenCannonade : Ability.check (DRS.ofList []) brazenCannonade = [] := by decide

def fourKnocks : Ability :=
  at_ (beginningOfYour .firstMain) (draw 1)
theorem okFourKnocks : Ability.check (DRS.ofList []) fourKnocks = [] := by decide

def hammerOfBogardan : Ability :=
  activatedOnlyDuring [generic 2, pip .red, pip .red, pip .red] (returnToHand this_)
    (duringPart .upkeep you)
theorem okHammerOfBogardan : Ability.check (DRS.ofList []) hammerOfBogardan = [] := by decide

/-- Apathy -/
def apathy : Spelled := spelled <| singleFacedCard
  { name := "Apathy", cost := [pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (doesntUntap (enchanted creature)
          (controllerOf (enchanted creature))),
        at_ (beginningOfPossessed .upkeep (controllerOf (enchanted creature)))
          (instruct (that player) (may (discard (aAtRandom (cardIn (handOf (that player)))))
            (untap (that creature)))) ] }

def felidarSovereign : Spelled := spelled <| singleFacedCard
  { name := "Felidar Sovereign", cost := [generic 4, pip .white, pip .white],
    types := [.creature], subtypes := [cat, beast],
    text :=
      [ vigilance, lifelink,
        at_ (beginningOfYour .upkeep)
          (if_ := compareAmt (lifeTotalOf you) .atLeast 40)
          winGame ],
    power := stat 4, toughness := stat 6 }

def gloriousEnforcer : Spelled := spelled <| singleFacedCard
  { name := "Glorious Enforcer", cost := [generic 5, pip .white, pip .white],
    types := [.creature], subtypes := [angel],
    text :=
      [ flying, lifelink,
        at_ (beginningOfPossessed .combat eachPlayer)
          (if_ := compareAmt (lifeTotalOf you) .greater (lifeTotalOf anOpponent))
          (gain thisCreature doubleStrike untilEndOfTurn) ],
    power := stat 5, toughness := stat 5 }

/-- Damia, Sage of Stone -/
def damia : Ability :=
  at_ (beginningOfYour .upkeep)
    (if_ := compareAmt (countOf (cardIn yourHand)) .less 7)
    (draw theDifference)
theorem okDamia : Ability.check (DRS.ofList []) damia = [] := by decide

def ivoryTower : Spelled := spelled <| singleFacedCard
  { name := "Ivory Tower", cost := [generic 1], types := [.artifact],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (sequentially
            [ gainLife (letter .x),
              define .x (minus (countOf (cardIn yourHand)) 4) ]) ] }

/-- Ajani, Adversary of Tyrants -/
def ajanisEmblem : Action :=
  getEmblem
    [ at_ (beginningOfYour .endStep)
        (createToken
          { characteristics :=
            { colors := [.white], types := [.creature], subtypes := [cat],
              text := [lifelink], power := stat 1, toughness := stat 1 } } (count := 3)) ]
theorem okAjanisEmblem : Action.check (DRS.ofList []) ajanisEmblem = [] := by decide

/-- Saheeli Rai -/
def saheelisCopy : Action :=
  sequentially
    [ create 1
        (copyOf (target (and [or [isArtifact, isCreature], youControl]))
          [isAlso [.artifact] []])
        [],
      gainHaste thatToken,
      atNext .endStep (exile it) ]
theorem okSaheelisCopy : Action.check (DRS.ofList []) saheelisCopy = [] := by decide

def timeWalk : Spelled := spelled <| singleFacedCard
  { name := "Time Walk", cost := [generic 1, pip .blue], types := [.sorcery],
    text := [spell (extraTurn 1)] }

def timeStretch : Spelled := spelled <| singleFacedCard
  { name := "Time Stretch", cost := [generic 8, pip .blue, pip .blue], types := [.sorcery],
    text := [spell (instruct (target isPlayer) (extraTurn 2))] }

def timeSieve : Spelled := spelled <| singleFacedCard
  { name := "Time Sieve", cost := [pip .blue, pip .black], types := [.artifact],
    text :=
      [ activated (compound [tapSymbol, sacrifice (counted (exactly 5) isArtifact)])
          (extraTurn 1) ] }

def finalFortune : Spelled := spelled <| singleFacedCard
  { name := "Final Fortune", cost := [pip .red, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ extraTurn 1,
            delay (beginningOfThatTurns .endStep) loseGame ])
                ] }

def finalFortuneExtraTurn : Action := extraTurn 1
theorem okFinalFortuneExtraTurn : Action.check (DRS.ofList []) finalFortuneExtraTurn = [] := by decide
def finalFortuneThatTurn : TurnRef := thatTime
theorem okFinalFortuneThatTurn :
    TurnRef.check ((stepAction 512 .postEvent .obj (DRS.ofList []) finalFortuneExtraTurn).1) finalFortuneThatTurn = [] := by
  decide

def lastChance : Spelled := spelled <| singleFacedCard
  { name := "Last Chance", cost := [pip .red, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ extraTurn 1,
            delay (beginningOfThatTurns .endStep) loseGame ])
                ] }

def chanceForGlory : Spelled := spelled <| singleFacedCard
  { name := "Chance for Glory", cost := [generic 1, pip .red, pip .white],
    types := [.instant],
    text :=
      [ spell (sequentially
          [ establish
              (abilityGrant (allOf (and [isCreature, youControl]))
                indestructible),
            extraTurn 1,
            delay (beginningOfThatTurns .endStep) loseGame ])
                ] }

def aggravatedAssault : Spelled := spelled <| singleFacedCard
  { name := "Aggravated Assault", cost := [generic 2, pip .red], types := [.enchantment],
    text :=
      [ activatedOnlyDuring [generic 3, pip .red, pip .red]
          (sequentially
            [ untap (allOf (and [isCreature, youControl])),
              additionalPhase .combat (some .mainPhase) 1 (some .mainPhase) ])
          asSorcery ] }

def relentlessAssault : Spelled := spelled <| singleFacedCard
  { name := "Relentless Assault", cost := [generic 2, pip .red, pip .red],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ untap
              (allOf
                 (and
                    [isCreature, did which attack soFarThisTurn])),
            additionalPhase .combat (some .mainPhase) 1 (some .mainPhase) ]) ] }

def fullThrottleFirstLine : Action := additionalPhase .combat (some .mainPhase) 2
theorem okFullThrottleFirstLine : Action.check (DRS.ofList []) fullThrottleFirstLine = [] := by decide
def yshtolaAdditionalEndStep : Action := additionalStep .endStep none 1
theorem okYshtolaAdditionalEndStep : Action.check (DRS.ofList []) yshtolaAdditionalEndStep = [] := by decide

/-- Sphinx of the Second Sun -/
def sphinxOfTheSecondSun : Spelled := spelled <| singleFacedCard
  { name := "Sphinx of the Second Sun", cost := [generic 6, pip .blue, pip .blue],
    types := [.creature], subtypes := [sphinx],
    text :=
      [ flying,
        at_ (beginningOfYour .postcombatMain)
          (additionalPhase .beginningPhase (some .postcombatMain) 1) ],
    power := stat 6, toughness := stat 6 }

/-- Obeka, Splitter of Seconds -/
def obekaSplitterOfSeconds : Spelled := spelled <| singleFacedCard
  { name := "Obeka, Splitter of Seconds",
    cost := [generic 1, pip .blue, pip .black, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [ogre, warlock],
    text :=
      [ menace,
        whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (additionalStep .upkeep none thatMuch) ],
    power := stat 2, toughness := stat 5 }

/-- Paradox Haze -/
def paradoxHaze : Spelled := spelled <| singleFacedCard
  { name := "Paradox Haze", cost := [generic 2, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isPlayer,
        at_ (nthOccurrence (.nth 1) (beginningOfPossessed .upkeep (enchanted player))
               (some .turn))
          (instruct (that player) (additionalStep .upkeep none 1)) ] }

def ninthDoctorAdditionalUpkeep : Action := additionalStep .upkeep none 1
theorem okNinthDoctorAdditionalUpkeep :
    Action.check (DRS.ofList []) ninthDoctorAdditionalUpkeep = [] := by decide

/-- Rites of Flourishing -/
def ritesOfFlourishing : Spelled := spelled <| singleFacedCard
  { name := "Rites of Flourishing", cost := [generic 2, pip .green],
    types := [.enchantment],
    text :=
      [ at_ (beginningOfPossessed .drawStep eachPlayer)
          (instruct (that player) (draw 1)),
        static (can eachPlayer (play (counted (additional (exactly 1)) isLand))
          (timing := onEachOfTheirTurns)) ] }

def odricLunarchMarshal : Spelled := spelled <| singleFacedCard
  { name := "Odric, Lunarch Marshal", cost := [generic 3, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ alsoForKeywords
          (fun k => at_ (beginningOfPossessed .combat eachPlayer)
            (doOnlyIf
              (establishThroughout
                (keywordGrant (allOf (and [isCreature, youControl])) k
                  (and [isCreature, youControl, hasKeyword k]))
                untilEndOfTurn)
              (exists_ (and [isCreature, youControl, hasKeyword k]))))
          [ firstStrike, flying, deathtouch, doubleStrike, haste, hexproofAbilities,
            indestructible, lifelink, menace, reach, skulk, trample, vigilance ] ],
    power := stat 3, toughness := stat 3 }

def bleedingEffect : Spelled := spelled <| singleFacedCard
  { name := "Bleeding Effect", cost := [generic 2, pip .white, pip .black],
    types := [.enchantment],
    text :=
      [ alsoForKeywords
          (fun k => at_ (beginningOfYour .combat)
            (doOnlyIf
              (establishThroughout
                (keywordGrant (allOf (and [isCreature, youControl])) k
                  (and [isCreature, in_ yourGraveyard, hasKeyword k]))
                untilEndOfTurn)
              (exists_ (and [isCreature, in_ yourGraveyard, hasKeyword k]))))
          [ flying, firstStrike, doubleStrike, deathtouch, hexproofAbilities, indestructible,
            lifelink, menace, reach, trample, vigilance ] ] }

def fettergeist : Spelled := spelled <| singleFacedCard
  { name := "Fettergeist", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [spirit],
    text :=
      [ flying,
        at_ (beginningOfYour .upkeep)
          (doUnless (sacrifice thisCreature)
            (scaledGenericMana
              (forEach 1
                (and [isCreature, youControl, isOther])))) ],
    power := stat 3, toughness := stat 4 }

def chainVeilEndStep : Ability :=
  at_ (beginningOfYour .endStep)
    (if_ := some (not (did you (activate (a
      (and [abilityHead .loyalty,
      abilityOf (a isPlaneswalker)]))) soFarThisTurn)))
    (loseLife 2)
theorem okChainVeilEndStep : Ability.check (DRS.ofList []) chainVeilEndStep = [] := by decide

def curseOfTheBloodyTome : Spelled := spelled <| singleFacedCard
  { name := "Curse of the Bloody Tome", cost := [generic 2, pip .blue],
    types := [.enchantment], subtypes := [aura, curse],
    text :=
      [ enchant isPlayer,
        at_ (beginningOfPossessed .upkeep (enchanted player))
          (instruct (that player) (mill 2)) ] }

def shriekingAffliction : Spelled := spelled <| singleFacedCard
  { name := "Shrieking Affliction", cost := [pip .black], types := [.enchantment],
    text :=
      [ at_ (beginningOfPossessed .upkeep eachOpponent)
          (if_ := compareAmt (countOf (cardIn (handOf (that player)))) .atMost 1)
          (instruct they (loseLife 3)) ] }

def centaurOfAttention : Spelled := spelled <| singleFacedCard
  { name := "Centaur of Attention", cost := [generic 3, pip .green, pip .green],
    types := [.creature], subtypes := [centaur, performer],
    text :=
      [ when (does thisCreature enter)
          (sequentially [rollDice 5 (.sides 6), storeResults thisCreature]),
        at_ (beginningOfYour .combat)
          (may (rerollStored anyNumber thisCreature)),
        static (conjunction [ getsPt thisCreature (up (letter .x)) (up (letter .x)),
            letterDefinition .x (greatestStoredMatch thisCreature) ]) ],
    power := stat 3, toughness := stat 3 }

/-- Shapeshifter: its printed `*`/`7-*` box is filled by a Characteristic-Defining Ability
[CR#208.2a], one that reads the last chosen number. -/
def shapeshifter : Spelled := spelled <| singleFacedCard
  { name := "Shapeshifter", cost := [generic 6], types := [.artifact, .creature],
    subtypes := [Definitions.shapeshifter],
    text :=
      [ static (entersChoosingFrom thisCreature .number (number (fromTo 0 7))),
        at_ (beginningOfYour .upkeep)
          (may (choose (a (qualityFrom .number (number (fromTo 0 7)))))),
        static (ptDefinitions thisCreature theLastChosenNumber
          (minus 7 theLastChosenNumber)) ] }

def selfSacrificeThenExile : Ability :=
  activated (sacrifice thisArtifact)
    (sequentially
      [ exile (target isCreature),
        atNext .endStep (putOntoBattlefield (that card)) ])
theorem okSelfSacrificeThenExile : Ability.check (DRS.ofList []) selfSacrificeThenExile = [] := by decide

/-- Mirror Universe -/
def mirrorUniverse : Spelled := spelled <| singleFacedCard
  { name := "Mirror Universe", cost := [generic 6], types := [.artifact],
    text :=
      [ activatedOnlyDuring (compound [tapSymbol, sacrifice thisArtifact])
          (exchange (lifeTotals (youAnd (target isOpponent))))
          (duringPart .upkeep you) ] }

/-- Concerted Effort -/
def concertedEffort : Spelled := spelled <| singleFacedCard
  { name := "Concerted Effort", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ alsoForKeywords
          (fun k => at_ (beginningOfPossessed .upkeep eachPlayer)
            (doOnlyIf
              (establishThroughout
                (keywordGrant (allOf (and [isCreature, youControl])) k
                  (and [isCreature, youControl, hasKeyword k]))
                untilEndOfTurn)
              (exists_ (and [isCreature, youControl, hasKeyword k]))))
          [ flying, fear, firstStrike, doubleStrike, landwalkAbilities, protectionAbilities,
            trample, vigilance ] ] }

/-- Land Tax -/
def landTax : Spelled := spelled <| singleFacedCard
  { name := "Land Tax", cost := [pip .white], types := [.enchantment],
    text :=
      [ at_ (beginningOfYour .upkeep) (if_ := exists_ opponentWithMoreLands)
          (may
            (sequentially
              [ search yourLibrary (counted (upTo 3) (and [isLand, hasSupertype .basic])),
                reveal (those card),
                putInto (those card) hand,
                shuffle ])) ] }

/-- Beckoning Will-o'-Wisp -/
def beckoningWillOWisp : Spelled := spelled <| singleFacedCard
  { name := "Beckoning Will-o'-Wisp", cost := [generic 2, pip .white],
    types := [.creature], subtypes := [spirit],
    text :=
      [ flying,
        flavorWord "Lure the Unwary"
          (at_ (beginningOfYour .combat) (choose anOpponent)),
        static (getsPt (allOf (and [isCreature, inCombat .attackerOf (the isChosenPlayer)]))
          (up 1) (up 0)) ],
    power := stat 1, toughness := stat 3 }

/-- Triarch Stalker -/
def triarchStalker : Spelled := spelled <| singleFacedCard
  { name := "Triarch Stalker", cost := [generic 3, pip .black, pip .black],
    types := [.artifact, .creature], subtypes := [necron],
    text :=
      [ flavorWord "Targeting Relay"
          (at_ (beginningOfYour .combat) (choose anOpponent)),
        static (abilityGrant (allOf (and [isCreature, inCombat .attackerOf (the
            isChosenPlayer)]))
          menace) ],
    power := stat 4, toughness := stat 5 }

def raphaelAdditionalCombat : Action := additionalPhase .combat (some .combat) 1
theorem okRaphaelAdditionalCombat : Action.check (DRS.ofList []) raphaelAdditionalCombat = [] := by decide
/-- Karn Liberated -/
def karnRestart : Action := restartGame
theorem okKarnRestart : Action.check (DRS.ofList []) karnRestart = [] := by decide

def waxWane : Spelled := spelled <| .split
  { characteristics :=
    { name := "Wax", cost := [pip .green], types := [.instant],
      text :=
        [ spell (get (target isCreature) (up 2) (up 2) untilEndOfTurn) ] } }
  { characteristics :=
    { name := "Wane", cost := [pip .white], types := [.instant],
      text := [spell (destroy (target isEnchantment))] } }

/-- Teleport -/
def teleport : Spelled := spelled <| singleFacedCard
  { name := "Teleport", cost := [pip .blue, pip .blue, pip .blue], types := [.instant],
    text :=
      [ spell (establishThroughout (cant any (block (target isCreature))) thisTurn)
          (timing := duringPart .declareAttackers none) ] }

/-- Dazzling Beauty's cast window; its targeted effect and delayed "next turn's upkeep" draw
need machinery outside this ticket. -/
def dazzlingBeautyCastRestriction : Timing := duringPart .declareBlockers none

/-- Thawing Glaciers -/
def thawingGlaciers : Spelled := spelled <| singleFacedCard
  { name := "Thawing Glaciers", types := [.land],
    text :=
      [ static (entersTapped thisLand),
        activated (compound [[generic 1], tapSymbol])
          (sequentially
            [ search yourLibrary (a (and [isLand, hasSupertype .basic])),
              putOntoBattlefieldTapped (that card),
              shuffle,
              atNext .cleanup (returnToHand thisLand) ]) ] }

/-- Blood Frenzy -/
def bloodFrenzy : Spelled := spelled <| singleFacedCard
  { name := "Blood Frenzy", cost := [generic 1, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
            [ get (target (and [isCreature, or [isAttacking, isBlocking]]))
                (up 4) (up 0) untilEndOfTurn,
              atNext .endStep (destroy (that creature)) ]) (timing :=
                                                                       some
                                                                          (beforePart
                                                                             .combatDamage none)) ] }

/-- Berserk -/
def berserk : Spelled := spelled <| singleFacedCard
  { name := "Berserk", cost := [pip .green], types := [.instant],
    text :=
      [ spell (sequentially
            [ establishFor (target isCreature)
                [ abilityGrant it trample,
                  getsPt it (up (letter .x)) (up 0),
                  letterDefinition .x (powerOf it) ]
                untilEndOfTurn,
              atNext .endStep (doOnlyIf (destroy (that creature))
                  (did (that
                    creature) attack soFarThisTurn)) ])
                        (timing := beforePart .combatDamage none) ] }

/-! ## Cast as though it had flash, sacrificed at cleanup

Each prints "You may cast this spell as though it had flash. If you cast it any time a sorcery
couldn't have been cast, the controller of the permanent it becomes sacrifices it at the
beginning of the next cleanup step." (`asThoughFlashSacrificedAtCleanup`). -/

/-- Armor of Thorns -/
def armorOfThorns : Spelled := spelled <| singleFacedCard
  { name := "Armor of Thorns", cost := [generic 1, pip .green], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant (and [isCreature, non (hasColor .black)]),
        static (getsPt (enchanted creature) (up 2) (up 2)) ] }

/-- Grave Servitude -/
def graveServitude : Spelled := spelled <| singleFacedCard
  { name := "Grave Servitude", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant isCreature,
        static (conjunction
          [ getsPt (enchanted creature) (up 3) (down 1),
            qualityChange it .sets (colored (.some [.black])) ]
          (subject := some (enchanted creature))) ] }

/-- Lightning Reflexes -/
def lightningReflexes : Spelled := spelled <| singleFacedCard
  { name := "Lightning Reflexes", cost := [generic 1, pip .red], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant isCreature,
        static (conjunction
          [ getsPt (enchanted creature) (up 1) (up 0),
            abilityGrant (enchanted creature) firstStrike ]
          (subject := some (enchanted creature))) ] }

/-- Mystic Veil -/
def mysticVeil : Spelled := spelled <| singleFacedCard
  { name := "Mystic Veil", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant isCreature,
        static (abilityGrant (enchanted creature) shroud) ] }

/-- Parapet -/
def parapet : Spelled := spelled <| singleFacedCard
  { name := "Parapet", cost := [generic 1, pip .white], types := [.enchantment],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        static (getsPt (allOf (and [isCreature, youControl])) (up 0) (up 1)) ] }

/-- Relic Ward -/
def relicWard : Spelled := spelled <| singleFacedCard
  { name := "Relic Ward", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant isArtifact,
        static (abilityGrant (enchanted artifact) shroud) ] }

/-- Soar -/
def soar : Spelled := spelled <| singleFacedCard
  { name := "Soar", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant isCreature,
        static (conjunction
          [ getsPt (enchanted creature) (up 0) (up 1),
            abilityGrant (enchanted creature) flying ]
          (subject := some (enchanted creature))) ] }

/-- Spider Climb -/
def spiderClimb : Spelled := spelled <| singleFacedCard
  { name := "Spider Climb", cost := [pip .green], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant isCreature,
        static (conjunction
          [ getsPt (enchanted creature) (up 0) (up 3),
            abilityGrant (enchanted creature) reach ]
          (subject := some (enchanted creature))) ] }

/-- Ward of Lights: its protection doesn't remove it, as Flickering Ward's doesn't
[CR#702.16n]. -/
def wardOfLights : Spelled := spelled <| singleFacedCard
  { name := "Ward of Lights", cost := [pip .white, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        enchant isCreature,
        static (entersChoosing thisAura .color),
        static (retention
          (abilityGrant (enchanted creature) (protection (ofChosen .color)))
          thisAura) ] }

/-! ## Enchant creature put onto the battlefield with this

Each one's entering ability gives it an enchant ability naming the creature that ability puts
onto the battlefield, puts the creature there, attaches the Aura to it, and creates a delayed
trigger that sacrifices the creature when the Aura leaves [CR#603.7a,603.7c]. The granted enchant ability is linked to the ability that grants it, so "put
onto the battlefield with [this object]" reads the creature that ability puts, though a later step
puts it [CR#607.1a,607.2c]. -/

/-- "creature card in a graveyard" -/
def creatureCardInAGraveyard : Predicate := and [isCreature, isCard, in_ graveyard]

/-- "creature put onto the battlefield with <source>" [CR#607.2c] -/
def creaturePutOntoTheBattlefieldWith (source : NounPhrase) : Predicate :=
  and [isCreature, did (by_ source) (putOntoBattlefield which)]

/-- "When <source> leaves the battlefield, that creature's controller sacrifices it." -/
def sacrificedWhenItLeaves (source : NounPhrase) : Action :=
  delay (does source (leave battlefield)) (instruct (controllerOf (that creature)) (sacrifice it))

/-- "it loses 'enchant creature card in a graveyard' and gains 'enchant creature put onto the
battlefield with this Aura.'" -/
def swapsEnchantForLinkedEntry : Action :=
  establish (conjunction
    [ abilityLoss it [written (enchant creatureCardInAGraveyard)],
      abilityGrant it (enchant (creaturePutOntoTheBattlefieldWith thisAura)) ]
    (subject := some it))

/-- Necromancy -/
def necromancy : Spelled := spelled <| singleFacedCard
  { name := "Necromancy", cost := [generic 2, pip .black], types := [.enchantment],
    text :=
      [ asThoughFlashSacrificedAtCleanup,
        when (does thisEnchantment enter)
          (sequentially
            [ establish (conjunction
                [ addSubtype it aura,
                  abilityGrant it (enchant (creaturePutOntoTheBattlefieldWith this_)) ]
                (subject := some it)),
              putOntoBattlefieldUnderYourControl (target creatureCardInAGraveyard),
              attachTo thisEnchantment it,
              sacrificedWhenItLeaves thisEnchantment ])
          (if_ := some (is it (in_ battlefield))) ] }

/-- Animate Dead -/
def animateDead : Spelled := spelled <| singleFacedCard
  { name := "Animate Dead", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant creatureCardInAGraveyard,
        when (does thisAura enter)
          (sequentially
            [ swapsEnchantForLinkedEntry,
              returnTo (enchanted (.ofType .card .creature)) battlefield [under you],
              attachTo thisAura it,
              sacrificedWhenItLeaves thisAura ])
          (if_ := some (is it (in_ battlefield))),
        static (getsPt (enchanted creature) (down 1) (up 0)) ] }

/-- Dance of the Dead -/
def danceOfTheDead : Spelled := spelled <| singleFacedCard
  { name := "Dance of the Dead", cost := [generic 1, pip .black], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant creatureCardInAGraveyard,
        when (does thisAura enter)
          (sequentially
            [ swapsEnchantForLinkedEntry,
              putInto (enchanted (.ofType .card .creature)) battlefield
                (riders := [entersAs .tapped, under you]),
              attachTo thisAura it,
              sacrificedWhenItLeaves thisAura ])
          (if_ := some (is it (in_ battlefield))),
        static (conjunction
          [ getsPt (enchanted creature) (up 1) (up 1),
            doesntUntap it (controllerOf it) ]
          (subject := some (enchanted creature))),
        at_ (beginningOfPossessed .upkeep (controllerOf (enchanted creature)))
          (instruct (that player) (may (pay [generic 1, pip .black] .once)
            (some (untap (that creature))))) ] }

end Semantics.Cards
