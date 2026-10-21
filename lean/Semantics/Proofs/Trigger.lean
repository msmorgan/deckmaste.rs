import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Piles
import Semantics.Cards.Turn

/-!
# Semantics.Proofs.Trigger

Port of `idris/src/Experimental/Proofs/Trigger.idr`: the pins of the Trigger family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Trigger

/-- "Whenever you cast a spell, draw a card." -/
theorem okCastsSingularComplement :
    Ability.check (DRS.ofList []) (whenever (.does (.actor .you) (.procedure .cast (a isSpell) .itsOwnCost none none none)) (draw (.lit 1)))
      = [] := by
  decide

/-- "Whenever you cast all spells, draw a card." -/
theorem badCastsPluralComplement :
    Ability.check (DRS.ofList []) (whenever (.does (.actor .you) (.procedure .cast (allOf isSpell) .itsOwnCost none none none)) (draw (.lit 1)))
      = [.singular] := by
  decide

/-- "Whenever this creature attacks while you're casting a spell, draw a card." -/
theorem okWhileDoingCast :
    Ability.check (DRS.ofList [])
      (whenever (does thisCreature attack)
        (while_ := some (.whileDoing (.does (.actor .you) (.procedure .cast (a isSpell) .itsOwnCost none none none))))
        (draw (.lit 1))) = [] := by
  decide

/-- "Whenever a creature dies, draw a card." -/
theorem okNontargetDeathHeader :
    Ability.check (DRS.ofList []) (whenever (does (a isCreature) die) (draw (.lit 1))) = [] := by
  decide

/-- "Whenever a creature attacks, that creature gets +2/+0 until end of turn." -/
theorem okThatCreatureAfterAttack :
    Ability.check (DRS.ofList [])
      (whenever (does (a isCreature) attack)
        (get (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn)))
      = [] := by
  decide

theorem selfAttackPublishesItsCreature :
    let event := does thisCreature attack
    let after := GameEvent.extend (DRS.ofList []) event
    Ability.check (DRS.ofList [])
      (whenever event
        (get (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn))) = [] ∧
      (firstReach (.word (.type .creature)) .one (nextClause after)).map (·.id) = some 0 ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

/-- "the last Intervention counter is removed from this enchantment by you" -/
theorem okAnnouncingRemovalAgent :
    GameEvent.check (DRS.ofList [])
      (.does (.by .you) (.removeCounters none (some (.printed (.named "interventionCounter"))) thisEnchantment .emptying false)) = [] := by
  decide

theorem badAnnouncingRemovalAgent :
    GameEvent.check (DRS.ofList [])
      (.does (.by (target .anyPlayer)) (.removeCounters none (some (.printed (.named "interventionCounter"))) thisEnchantment .emptying false)) = [.eventAgent] := by
  decide

/-- "Whenever a creature attacks a player, …" -/
theorem okSingularAttackDefender :
    GameEvent.check (DRS.ofList []) (.does (.actor (a isCreature)) (.attack (some (a .anyPlayer)))) = [] := by decide

/-- "Whenever a creature attacks your opponents, …" -/
theorem badPluralAttackDefender :
    GameEvent.check (DRS.ofList []) (.does (.actor (a isCreature)) (.attack (some (.described .all (.isOpponentOf .you)))))
      = [.singular] := by
  decide

/-- "Whenever you attack, …": the active player declares the attack [CR#508.1], and the ability
triggers on the creatures they control being declared [CR#508.3d]. -/
theorem okPlayerAttacksHeader : GameEvent.check (DRS.ofList []) (does you attack) = [] := by decide

/-- "Whenever a creature enters during your turn, draw a card." -/
theorem okHeaderOwnTurnWindow :
    Ability.check (DRS.ofList [])
      (whenever (does (a isCreature) enter) (during := some (duringPart .turn (some .you)))
        (draw (.lit 1))) = [] := by
  decide

/-- "If one or more tokens would be created under your control, …" -/
theorem okTokenCreationSubject :
    GameEvent.check (DRS.ofList []) (.does .any (.createObject .anyAmount (.token (.copyOf (counted (atLeast 1) .isToken) []) []) false (some .you)))
      = [] := by
  decide

/-- "If one or more creatures would be created under your control, …" -/
theorem badNonTokenCreationSubject :
    GameEvent.check (DRS.ofList []) (.does .any (.createObject .anyAmount (.token (.copyOf (counted (atLeast 1) isCreature) []) []) false (some .you)))
      = [.tokenPhrase] := by
  decide

/-- "Whenever you draw a card, draw a card. This triggers only once each turn." -/
theorem okTriggerLimitOffChapter :
    Ability.check (DRS.ofList [])
      (whenever (.does (.actor .you) (.draw (.exactlyOf .anyAmount)))
        (draw (.lit 1)) (limit := some oncePerTurn)) = [] := by
  decide

/-- "Whenever a creature attacks a player" -/
theorem okPlayerAttackDefender :
    GameEvent.check (DRS.ofList []) (.does (.actor (a isCreature)) (.attack (some (a .anyPlayer)))) = [] := by decide

def afterLifePayment : DRS := GameEvent.extend (DRS.ofList []) (.does (.actor (a .anyPlayer)) (.changeLife (.down .anyAmount)))

/-- "Whenever a player pays life, that player draws a card." -/
theorem okLifePaymentPayerReadback :
    NounPhrase.checkAt (some .player) afterLifePayment (that .player) = [] := by decide

def afterPassivePayment : DRS :=
  GameEvent.extend (DRS.ofList []) (.does .any (.pay .itsManaCost .once .paid (some thisCreature) (some (.named "cumulativeUpkeep"))))

/-- "Whenever this creature's cumulative upkeep is paid, that player …" -/
theorem badPassivePayerReadback :
    NounPhrase.checkAt (some .player) afterPassivePayment (that .player)
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Whenever a player pays life, you gain that much life." -/
theorem okLifePaymentThatMuch : Amount.check afterLifePayment .thatMuch = [] := by decide

def afterKeywordCostPayment : DRS :=
  GameEvent.extend (DRS.ofList []) (.does (.actor .you) (.pay .itsManaCost .once .paid (some thisEnchantment) (some (.named "cumulativeUpkeep"))))

theorem badKeywordCostPaymentThatMuch :
    Amount.check afterKeywordCostPayment .thatMuch = [.quantOutcomeInScope 0] := by decide

/-- "whenever one or more time counters are put on this enchantment" -/
theorem okManyCountersOnPlacement :
    GameEvent.check (DRS.ofList []) (.does .any (.putCounters .anyAmount (.printed (.named "timeCounter")) thisEnchantment .many false))
      = [] := by
  decide

/-- "Whenever you create one or more tokens, …" -/
theorem okTokensCreatedByYou :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.createObject .anyAmount (.token (.copyOf (counted (atLeast 1) .isToken) []) []) false none))
      = [] := by
  decide

/-- "Whenever an effect and you would create one or more tokens, …" -/
theorem badTokensCreatedByCauserAndPlayer :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.createObject .anyAmount (.token (.copyOf (counted (atLeast 1) .isToken) []) []) true none))
      = [.verbedVoiceOk] := by
  decide

/-- "Whenever one or more tokens are created under your opponents' control, …" -/
theorem badTokensCreatedUnderPlural :
    GameEvent.check (DRS.ofList [])
      (.does .any (.createObject .anyAmount (.token (.copyOf (counted (atLeast 1) .isToken) []) []) false (some (.described .all (.isOpponentOf .you))))) = [.verbedVoiceOk] := by
  decide

/-- "Whenever an opponent creates one or more tokens, …" -/
theorem badTokensCreatedByMintingNoun :
    GameEvent.check (DRS.ofList []) (.does (.actor anOpponent) (.createObject .anyAmount (.token (.copyOf (counted (atLeast 1) .isToken) []) []) false none))
      = [.verbedVoiceOk] := by
  decide

/-- "Whenever this creature becomes the target of a spell, …" -/
theorem okSpellTargetingEvent :
    GameEvent.check (DRS.ofList []) (.does (.by (a isSpell)) (.target thisCreature)) = [] := by decide

/-- "Whenever you become the target of a player, …" -/
theorem badPlayerTargetingEvent :
    GameEvent.check (DRS.ofList []) (.does (.by (a .anyPlayer)) (.target .you)) = [.targeter .player] := by decide

/-- "If an ability ... triggers, it triggers an additional time." -/
theorem okMultipliedTrigger :
    StaticSpec.check (DRS.ofList [])
      (.additionalTriggers
        (.does (.actor (a (.and [.abilityHead .anyTriggered, .abilityOf (a (and [isCreature, youControl]))])))
          .trigger)
        (exactly 1)) = [] := by
  decide

/-- "A triggered ability triggers" is the ability performing `trigger` [CR#603.2], and the
ability is its event's agent: the template writes that event. -/
theorem triggersIsTheAbilityTriggering :
    Templates.triggers (a (.abilityHead .anyTriggered))
      = .does (.actor (a (.abilityHead .anyTriggered))) .trigger := rfl

/-- "Whenever a player triggers an ability": a player is not an ability, so it cannot be the
event's agent. -/
theorem badPlayerTriggers :
    GameEvent.check (DRS.ofList []) (Templates.triggers (a .anyPlayer)) = [.eventAgent] := by
  decide

/-- Day and night are designations the game itself has [CR#731.1]: "it becomes night" is the game
gaining night, and "it's night" the game having it. -/
theorem dayAndNightAreTheGamesDesignations :
    gameBecomes night = .does .any (.gameGains night) ∧
      setGameDesignation day = .gameGains day ∧
      gameIs day = .gameHas day := ⟨rfl, rfl, rfl⟩

/-- "Whenever it becomes night, draw a card.": the game gaining night is a trigger event. -/
theorem okWheneverItBecomesNight :
    Ability.check (DRS.ofList []) (whenever (gameBecomes night) (draw (.lit 1))) = [] := by
  decide

/-- "If it's neither day nor night, it becomes day.": the game reads and gains its own
designations. -/
theorem okItBecomesDayIfNeither :
    Action.check (DRS.ofList [])
      (.doIf (.and [.not (gameIs day), .not (gameIs night)]) (setGameDesignation day) none)
      = [] := by
  decide

/-- The game holds only a game-wide designation, and a game-wide one only the game holds. -/
theorem badGameHoldsAnotherScope :
    Action.check (DRS.ofList []) (setGameDesignation monarch) = [.designationScope monarch] ∧
      Condition.check (DRS.ofList []) (gameIs monarch) = [.designationScope monarch] ∧
      Action.check (DRS.ofList []) (.gainDesignation .you day none) =
        [.designationScope day, .designationHolder day .player] := by
  decide

/-- "The game is a draw" is the draw outcome concluded [CR#104.4c]. -/
theorem drawGameConcludes : Templates.drawGame = .conclude .drawGame none := rfl

/-- "Exchange life totals with target opponent." -/
theorem okExchangeTwoParties :
    Action.check (DRS.ofList []) (.exchange (.lifeTotals (both .you (target (.isOpponentOf .you))))) = [] := by
  decide

/-- "Whenever an opponent commits a crime, draw a card." -/
theorem okCrimeBySinglePlayer :
    Ability.check (DRS.ofList []) (whenever (.does (.actor anOpponent) .commitCrime) (draw (.lit 1)))
      = [] := by
  decide

/-- "Whenever all players commit a crime, draw a card." A crime targets an opponent, something
an opponent controls, or a card in an opponent's graveyard [CR#700.13]; the whole player set has
no opponent, so such a crime has no opponent-owned object. One player at a time commits one
(`okCrimeBySinglePlayer`). -/
theorem badCrimeByAllPlayers :
    Ability.check (DRS.ofList []) (whenever (.does (.actor (allOf .anyPlayer)) .commitCrime) (draw (.lit 1)))
      = [.singular] := by
  decide

/-- "… you may remove a +1/+1 counter from this creature. If you do, increase or decrease the
result by 1." -/
def rollShiftBody : Action :=
  may (.removeCounters (some (exactly 1)) (some (.printed p1p1Counter)) thisCreature .one true)
    (some (shiftResult (.lit 1)))

/-- "After you roll a die, you may remove a +1/+1 counter from this creature. If you do,
increase or decrease the result by 1." (Xenosquirrels) -/
theorem okAfterRollShift :
    Ability.check (DRS.ofList []) (after (.does (.actor .you) (.rollDice .anyAmount .anyDie .one .anyResult)) rollShiftBody)
      = [] := by
  decide

/-- The same body after a header that rolls no die: with no result announced there is nothing
to increase or decrease. -/
theorem badShiftWithoutRoll :
    Ability.check (DRS.ofList []) (after (does thisCreature attack) rollShiftBody)
      = [.outcomeInScope .rollResult 0] := by
  decide

/-- "0 — Draw a card.": chapter symbols start at I, which represents 1 [CR#714.2a]. -/
theorem badChapterZero :
    Ability.check (DRS.ofList []) (when (.chapterMark [0]) (draw (.lit 1)))
      = [.chapterMarks] := by
  decide

/-! ## Trigger words

`when`, `whenever`, `at_` and `after` are one triggered ability [CR#603.1] spelled with the word
the card prints; each takes every optional part of the header. -/

/-- "Whenever an opponent loses life during your turn, if you control an artifact, draw a card.
This ability triggers only once each turn.": a window, an intervening "if" [CR#603.4] and a limit
on one header. -/
def wheneverDuringIfOnlyOnce : Ability :=
  whenever (does anOpponent (loseLife anyAmount)) (during := some (duringPart .turn (some you)))
    (if_ := some (exists_ (and [isArtifact, youControl]))) (draw (.lit 1))
    (limit := some oncePerTurn)

/-- Required pin: the header's window, intervening "if" and limit are fields of one triggered
ability, and the trigger word is none of them. -/
theorem wheneverDuringIfOnlyOnceIsOneTrigger :
    wheneverDuringIfOnlyOnce =
      .triggered (does anOpponent (loseLife anyAmount)) [] none []
        (some (duringPart .turn (some you))) (some oncePerTurn)
        (some (exists_ (and [isArtifact, youControl]))) (draw (.lit 1)) :=
  rfl

theorem okWheneverDuringIfOnlyOnce :
    Ability.check (DRS.ofList []) wheneverDuringIfOnlyOnce = [] := by
  decide

/-- The event of a card's first triggered ability. -/
private def firstTriggerEvent (card : Spelled) : Option GameEvent :=
  match card.card with
  | .singleFaced face =>
    match (face.characteristics.text.head? : Option Ability) with
    | some (.triggered event _ _ _ _ _ _ _) => some event
    | _ => none
  | _ => none

/-- Required pin: *Tezzeret's Gatebreaker*'s "When this artifact enters" is the intransitive
`enter` with this artifact as its Actor [CR#603.6a]. -/
theorem gatebreakerTriggersOnThisArtifactEntering :
    firstTriggerEvent Cards.tezzeretsGatebreaker = some (.does (.actor thisArtifact) enter) :=
  rfl

/-- Whether a card's first ability, under its "The same is true for …", is a trigger with no
intervening "if" whose effect is conditional. -/
private def conditionIsInTheEffect (card : Spelled) : Bool :=
  match card.card with
  | .singleFaced face =>
    match (face.characteristics.text.head? : Option Ability) with
    | some (.alsoForKeywords (.triggered _ _ _ _ _ _ none (.doIf _ _ none)) _) => true
    | _ => false
  | _ => false

/-- Required pin: *Odric, Lunarch Marshal*'s "creatures you control gain first strike until end of
turn if a creature you control has first strike" prints its "if" after the effect, so it is not an
intervening "if" [CR#603.4] and the condition is part of the effect; so are *Bleeding Effect*'s
and *Concerted Effort*'s. -/
theorem trailingIfIsInTheEffect :
    conditionIsInTheEffect Cards.odricLunarchMarshal = true ∧
      conditionIsInTheEffect Cards.bleedingEffect = true ∧
      conditionIsInTheEffect Cards.concertedEffort = true := by
  decide

end Semantics.Proofs.Trigger
