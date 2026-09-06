import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Trigger

Port of `idris/src/Experimental/Proofs/Trigger.idr`: the pins of the Trigger family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Trigger

/-- "Whenever you cast a spell, draw a card." -/
theorem okCastsSingularComplement :
    Ability.check [] (whenever (.casts .you (a spell) none) (draw (.lit 1) (agent := .you)))
      = [] := by
  decide

/-- "Whenever you cast all spells, draw a card." -/
theorem badCastsPluralComplement :
    Ability.check [] (whenever (.casts .you (allOf spell) none) (draw (.lit 1) (agent := .you)))
      = [.singular] := by
  decide

/-- "Whenever this creature attacks while you're casting a spell, draw a card." -/
theorem okWhileDoingCast :
    Ability.check []
      (.triggered (attacks thisCreature) []
        (some (.whileDoing (.casts .you (a spell) none))) [] none none none
        (draw (.lit 1) (agent := .you))) = [] := by
  decide

/-- "Whenever this creature attacks while a creature is dying, draw a card." -/
theorem badWhileDoingMoment :
    Ability.check []
      (.triggered (attacks thisCreature) [] (some (.whileDoing (.dies (a creature))))
        [] none none none (draw (.lit 1) (agent := .you))) = [.eventUnderway] := by
  decide

/-- "Whenever a creature dies, draw a card." -/
theorem okNontargetDeathHeader :
    Ability.check [] (whenever (.dies (a creature)) (draw (.lit 1) (agent := .you))) = [] := by
  decide

/-- "Whenever target creature dies, draw a card." -/
theorem badTargetedDeathHeader :
    Ability.check [] (whenever (.dies (target creature)) (draw (.lit 1) (agent := .you)))
      = [.headerNontarget] := by
  decide

/-- "Whenever a creature attacks, that creature gets +2/+0 until end of turn." -/
theorem okThatCreatureAfterAttack :
    Ability.check []
      (whenever (attacks (a creature))
        (get (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn)))
      = [] := by
  decide

/-- The Idris pin refutes the anaphor; the unresolved "that creature" has no zone either. -/
theorem badThatCreatureIsSelf :
    Ability.check []
      (whenever (attacks thisCreature)
        (get (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn)))
      = [.anaphor (.word (.type .creature)) .one 0, .zoneIs .battlefield] := by
  decide

/-- "the last Intervention counter is removed from this enchantment by you" -/
theorem okAnnouncingRemovalAgent :
    GameEvent.check []
      (.counterEvent .removed (some (.named "Intervention")) thisEnchantment .emptying (some .you)
        false) = [] := by
  decide

theorem badAnnouncingRemovalAgent :
    GameEvent.check []
      (.counterEvent .removed (some (.named "Intervention")) thisEnchantment .emptying
        (some (target .anyPlayer)) false) = [.eventAgent] := by
  decide

/-- "Whenever a creature attacks a player, …" -/
theorem okSingularAttackDefender :
    GameEvent.check [] (.combat .attackerOf (a creature) (some (a .anyPlayer))) = [] := by decide

/-- "Whenever a creature attacks your opponents, …" -/
theorem badPluralAttackDefender :
    GameEvent.check [] (.combat .attackerOf (a creature) (some (.playerGroup .yourOpponents)))
      = [.singular] := by
  decide

/-- "Whenever you attack, …": the active player declares the attack [CR#508.1], and the ability
triggers on the creatures they control being declared [CR#508.3d]. -/
theorem okPlayerAttacksHeader : GameEvent.check [] (attacks .you) = [] := by decide

/-- "Whenever target creature card in your graveyard attacks, …" -/
theorem badGraveyardAttacker :
    GameEvent.check [] (attacks (target (.and [creature, .inZone (graveyardOf .you)])))
      = [.attacker] := by
  decide

/-- "Whenever a creature enters during your turn, draw a card." -/
theorem okHeaderOwnTurnWindow :
    Ability.check []
      (.triggered (.enters (a creature) none) [] none []
        (some (.duringPart .turn (some .you))) none none (draw (.lit 1) (agent := .you))) = [] := by
  decide

/-- "Whenever a creature enters during the turn, draw a card." -/
theorem badHeaderBareTurnWindow :
    Ability.check []
      (.triggered (.enters (a creature) none) [] none []
        (some (.duringPart .turn none)) none none (draw (.lit 1) (agent := .you))) = [.windowOk] :=
            by
  decide

/-- "If one or more tokens would be created under your control, …" -/
theorem okTokenCreationSubject :
    GameEvent.check [] (.tokensCreated (counted (atLeast 1) .isToken) false none (some .you))
      = [] := by
  decide

/-- "If one or more creatures would be created under your control, …" -/
theorem badNonTokenCreationSubject :
    GameEvent.check [] (.tokensCreated (counted (atLeast 1) creature) false none (some .you))
      = [.tokenPhrase] := by
  decide

/-- "Whenever you draw a card, draw a card. This triggers only once each turn." -/
theorem okTriggerLimitOffChapter :
    Ability.check []
      (.triggered (.draws .you) [] none [] none (some .oncePerTurn) none
        (draw (.lit 1) (agent := .you))) = [] := by
  decide

/-- "I — Draw a card. This ability triggers only once each turn." -/
theorem badChapterLimit :
    Ability.check []
      (.triggered (.chapterMark [1]) [] none [] none (some .oncePerTurn) none
        (draw (.lit 1) (agent := .you))) = [.chapterDefaults] := by
  decide

/-- "I — , if you control a creature, draw a card." -/
theorem badChapterIntervening :
    Ability.check []
      (.triggered (.chapterMark [1]) [] none [] none none
        (some (exists_ (.and [creature, .hasPossessor .controller .you]))) (draw (.lit 1) (agent :=
            .you)))
      = [.chapterDefaults] := by
  decide

/-- "Whenever a creature attacks a player" -/
theorem okPlayerAttackDefender :
    GameEvent.check [] (.combat .attackerOf (a creature) (some (a .anyPlayer))) = [] := by decide

/-- "Whenever a creature attacks a planeswalker or a creature" -/
theorem badMixedAttackDefenderHalves :
    GameEvent.check []
      (.combat .attackerOf (a creature)
        (some (a (.or [.hasType .planeswalker, .hasType .creature])))) = [.attackable] := by
  decide

def afterLifePayment : Bindings := GameEvent.after [] (.paysLife (a .anyPlayer))

/-- "Whenever a player pays life, that player draws a card." -/
theorem okLifePaymentPayerReadback :
    NounPhrase.check (some .player) afterLifePayment (that .player) = [] := by decide

def afterPassivePayment : Bindings :=
  GameEvent.after [] (.paysCost none .paid thisCreature "CumulativeUpkeep")

/-- "Whenever this creature's cumulative upkeep is paid, that player …" -/
theorem badPassivePayerReadback :
    NounPhrase.check (some .player) afterPassivePayment (that .player)
      = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Whenever a player pays life, you gain that much life." -/
theorem okLifePaymentThatMuch : Amount.check afterLifePayment .thatMuch = [] := by decide

def afterKeywordCostPayment : Bindings :=
  GameEvent.after [] (.paysCost (some .you) .paid thisEnchantment "CumulativeUpkeep")

theorem badKeywordCostPaymentThatMuch :
    Amount.check afterKeywordCostPayment .thatMuch = [.quantOutcomeInScope 0] := by decide

/-- "whenever one or more time counters are put on this enchantment" -/
theorem okManyCountersOnPlacement :
    GameEvent.check [] (.counterEvent .put (some (.named "Time")) thisEnchantment .many none false)
      = [] := by
  decide

/-- "when the last time counter is put on this enchantment" -/
theorem badLastCounterOnPlacement :
    GameEvent.check [] (.counterEvent .put (some (.named "Time")) thisEnchantment .emptying none
        false)
      = [.counterBatchOk] := by
  decide

/-- "Whenever you create one or more tokens, …" -/
theorem okTokensCreatedByYou :
    GameEvent.check [] (.tokensCreated (counted (atLeast 1) .isToken) false (some .you) none)
      = [] := by
  decide

/-- "Whenever an effect and you would create one or more tokens, …" -/
theorem badTokensCreatedByCauserAndPlayer :
    GameEvent.check [] (.tokensCreated (counted (atLeast 1) .isToken) true (some .you) none)
      = [.verbedVoiceOk] := by
  decide

/-- "Whenever one or more tokens are created under your opponents' control, …" -/
theorem badTokensCreatedUnderPlural :
    GameEvent.check []
      (.tokensCreated (counted (atLeast 1) .isToken) false none
        (some (.playerGroup .yourOpponents))) = [.verbedVoiceOk] := by
  decide

/-- "Whenever an opponent creates one or more tokens, …" -/
theorem badTokensCreatedByMintingNoun :
    GameEvent.check [] (.tokensCreated (counted (atLeast 1) .isToken) false (some anOpponent) none)
      = [.verbedVoiceOk] := by
  decide

/-- "Whenever this creature becomes the target of a spell, …" -/
theorem okSpellTargetingEvent :
    GameEvent.check [] (.becomesTarget thisCreature (a spell)) = [] := by decide

/-- "Whenever you become the target of a player, …" -/
theorem badPlayerTargetingEvent :
    GameEvent.check [] (.becomesTarget .you (a .anyPlayer)) = [.targeter .player] := by decide

/-- "If an ability ... triggers, it triggers an additional time." -/
theorem okMultipliedTrigger :
    StaticSpec.check []
      (.additionalTriggers
        (.triggers (a (.and [.abilityHead .anyTriggered, .abilityOf (a creatureYouControl)])))
        (exactly 1)) = [] := by
  decide

/-- "If a creature you control dies, that ability triggers an additional time." -/
theorem badMultipliedNonTrigger :
    StaticSpec.check [] (.additionalTriggers (.dies (a creatureYouControl)) (exactly 1))
      = [.triggerCountOk] := by
  decide

/-- "Exchange life totals with target opponent." -/
theorem okExchangeTwoParties :
    Instruction.check [] (.exchange (.lifeTotals (.both .you (target .opponent)))) = [] := by
  decide

/-- "Exchange life totals with target opponent" with one party: the row states both halves at
once, so an exchange that can't be completed in its entirety is unwritable rather than half-done
[CR#701.12a,701.12c]. -/
theorem badExchangeOneParty :
    Instruction.check [] (.exchange (.lifeTotals (target .opponent))) = [.twoParties] := by decide

/-- "You and your opponents exchange life totals." -/
theorem badExchangePluralParty :
    Instruction.check [] (.exchange (.lifeTotals (.both .you (.playerGroup .yourOpponents))))
      = [.twoParties] := by
  decide

/-- "Whenever an opponent commits a crime, draw a card." -/
theorem okCrimeBySinglePlayer :
    Ability.check [] (whenever (.commitsCrime anOpponent) (draw (.lit 1) (agent := .you)))
      = [] := by
  decide

/-- "Whenever all players commit a crime, draw a card." A crime targets an opponent, something
an opponent controls, or a card in an opponent's graveyard [CR#700.13]; the whole player set has
no opponent, so such a crime has no opponent-owned object. One player at a time commits one
(`okCrimeBySinglePlayer`). -/
theorem badCrimeByAllPlayers :
    Ability.check [] (whenever (.commitsCrime (allOf .anyPlayer)) (draw (.lit 1) (agent := .you)))
      = [.singular] := by
  decide

/-- "… you may remove a +1/+1 counter from this creature. If you do, increase or decrease the
result by 1." -/
def rollShiftBody : Instruction :=
  .offer (.removeCounters (some (exactly 1)) (some (.printed plusOnePlusOne)) thisCreature)
    (some (shiftResult (.lit 1))) none (agent := .you)

/-- "After you roll a die, you may remove a +1/+1 counter from this creature. If you do,
increase or decrease the result by 1." (Xenosquirrels) -/
theorem okAfterRollShift :
    Ability.check [] (after (.rollsDice .you .one none .anyResult) rollShiftBody)
      = [] := by
  decide

/-- The same body after a header that rolls no die: with no result announced there is nothing
to increase or decrease. -/
theorem badShiftWithoutRoll :
    Ability.check [] (after (attacks thisCreature) rollShiftBody)
      = [.outcomeInScope .rollResult 0] := by
  decide

/-- "0 — Draw a card.": chapter symbols start at I, which represents 1 [CR#714.2a]. -/
theorem badChapterZero :
    Ability.check [] (when (.chapterMark [0]) (draw (.lit 1) (agent := .you)))
      = [.chapterMarks] := by
  decide

end Semantics.Proofs.Trigger
