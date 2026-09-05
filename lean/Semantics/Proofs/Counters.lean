import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Counters

Port of `idris/src/Experimental/Proofs/Counters.idr`: the pins of the Counters family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Counters

/-- "a +1/+1 counter", printed. -/
def p11 : CounterKindSource := .printed plusOnePlusOne

/-- "of the chosen creature type" -/
theorem okChosenCreatureType :
    Predicate.check .object
      [⟨.a, .one, .quality (.subtype .creature)⟩]
      (ofChosen (.subtype .creature)) = [] := by
  decide

/-- "of the chosen creature type" -/
theorem badChosenWrongSort :
    Predicate.check .object [⟨.a, .one, .quality .color⟩]
      (ofChosen (.subtype .creature))
      = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

/-- "a counter of that kind" -/
theorem badChosenCounterKindRead :
    Predicate.check .object [⟨.a, .one, .quality .counterKind⟩]
      (ofChosen .counterKind) = [.chosenQualityRead .counterKind] := by
  decide

/-- "this creature" -/
theorem okAscribedSelf :
    NounPhrase.check (some .object) [] (.asType .creature .this none) = [] := by decide

/-- "this creature" -/
theorem badAscribedTarget :
    NounPhrase.check (some .object) [] (.asType .creature (target creature) none)
      = [.ascribable] := by
  decide

/-- "creature you control that isn't attacking" -/
theorem okConsistentConjunction :
    Predicate.check .object [] (.and [creature, .hasPossessor .controller .you, .not attacking])
      = [] := by
  decide

/-- "creature you control that you don't control" -/
theorem okControlSelfNegation :
    Predicate.check .object []
      (.and [.hasPossessor .controller .you, .not (.hasPossessor .controller .you)]) = [] := by
  decide

/-- "attacking noncreature" -/
theorem badAttackingNoncreature :
    Predicate.check .object [] (.and [attacking, .not creature]) = [.contradictionFree] := by
  decide

/-- "Whenever this creature blocks a creature or becomes blocked by a creature, that creature
gets −1/−1 until end of turn." -/
theorem okAltHeaderAgreeingReadback :
    Ability.check []
      (.triggered (blocks thisCreature (some (a creature)))
        [becomesBlocked thisCreature (some (a creature))] none [] none none none
        (gets (that (.type .creature)) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn)))
      = [] := by
  decide

/-- The Idris pin refutes the first anaphor; unresolved, the subject has no zone, and the
toughness half's `it` reads back the same missing antecedent. -/
theorem badAltHeaderMixedReadback :
    Ability.check []
      (.triggered (blocks thisCreature none)
        [becomesBlocked thisCreature (some (a creature))] none [] none none none
        (gets (that (.type .creature)) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn)))
      = [ .anaphor (.word (.type .creature)) .one 0, .zoneIs .battlefield,
          .anaphor .bare .one 0, .zoneIs .battlefield ] := by
  decide

theorem badThreeArmHeaderReadback :
    Ability.check []
      (.triggered (attacks thisCreature)
        [blocks thisCreature none, .becomesTarget thisCreature (a spell)] none [] none none none
        (.dealDamage thisCreature (powerOf (that (.type .creature))) (each .opponent)))
      = [.anaphor (.word (.type .creature)) .one 0] := by
  decide

theorem badJoinedHeaderReadback :
    Ability.check []
      (.triggered (.enters thisCreature none) [] none
        [⟨.dies (a creatureYouControl), [], none, none⟩] none none none
        (.putCounters (.lit 1) p11 it)) = [.anaphor .bare .one 0] := by
  decide

/-- "Remove a +1/+1 counter from target creature." -/
theorem okRemoveCounterFromTarget :
    Instruction.check [] (.removeCounters (some (exactly 1)) (some p11) (target creature))
      = [] := by
  decide

/-- "Destroy target creature. Remove a +1/+1 counter from it." -/
theorem badRemoveCountersDead :
    Instruction.check []
      (.sequentially
        [destroy (target creature), .removeCounters (some (exactly 1)) (some p11) it])
      = [.counterMemory] := by
  decide

/-- "Move a counter from target creature onto this creature." -/
theorem okMoveCounterOntoThis :
    Instruction.check [] (.moveCounters (.lit 1) none (target creature) thisCreature) = [] := by
  decide

/-- "Move a counter from target creature onto it." -/
theorem badMoveCountersSelf :
    Instruction.check [] (.moveCounters (.lit 1) none (target creature) it)
      = [.moveDestination] := by
  decide

/-- "Create a 1/1 creature creature token." -/
theorem badTokenDuplicateType :
    Instruction.check []
      (create (.lit 1)
        { characteristics :=
          { types := [.creature, .creature], power := some (.lit 1), toughness := some (.lit 1) } })
      = [.tokenCanonical] := by
  decide

/-- "Create a 1/1 white white Soldier creature token." -/
theorem badTokenDuplicateColor :
    Instruction.check []
      (create (.lit 1) (creatureToken 1 1 [.white, .white] [creatureType "Soldier"]))
      = [.tokenCanonical] := by
  decide

/-- "Target opponent loses 2 life. You gain that much life." -/
theorem okThatMuchAfterOutcome :
    Instruction.check []
      (.sequentially [losesLife (target .opponent) (.lit 2), gainsLife .you .thatMuch]) = [] := by
  decide

/-- "This deals 2 damage to target creature and you gain that much life." -/
theorem badSimultaneousReadsOutcome :
    Instruction.check []
      (.simultaneously [.dealDamage .this (.lit 2) (target creature), gainsLife .you .thatMuch])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "You may create a token and put a +1/+1 counter on it." -/
theorem badSimultaneousReadsMayDeed :
    Instruction.check []
      (.simultaneously
        [ may .you (create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Plant"])),
          .putCounters (.lit 1) p11 it ]) = [.anaphor .bare .one 0] := by
  decide

/-- "You may have this deal 2 damage and you gain that much life." -/
theorem badSimultaneousReadsMayOutcome :
    Instruction.check []
      (.simultaneously
        [may .you (.dealDamage .this (.lit 2) (target creature)), gainsLife .you .thatMuch])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Create a Plant token and a Soldier token. Put a +1/+1 counter on it." -/
theorem badBatchTwoCreatesThenIt :
    Instruction.check []
      (.sequentially
        [ .simultaneously
            [ create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Plant"]),
              create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"]) ],
          .putCounters (.lit 1) p11 it ]) = [.anaphor .bare .one 2] := by
  decide

theorem badBatchTwoOutcomesThenThatMuch :
    Instruction.check []
      (.sequentially
        [ .simultaneously
            [.dealDamage .this (.lit 2) (target creature), losesLife (target .opponent) (.lit 3)],
          gainsLife .you .thatMuch ]) = [.quantOutcomeInScope 2] := by
  decide

/-- "Create a 1/1 green Plant creature token. Put a +1/+1 counter on it." -/
theorem okCreatedThenCountered :
    Instruction.check []
      (.sequentially
        [ .create .you (.lit 1) (.written (creatureToken 1 1 [.green] [creatureType "Plant"])) [],
          .putCounters (.lit 1) p11 it ]) = [] := by
  decide

theorem badDistributedCreationIt :
    Instruction.check []
      (.sequentially
        [ .create (each .anyPlayer) (.lit 1)
            (.written (creatureToken 1 1 [.green] [creatureType "Plant"])) [],
          .putCounters (.lit 1) p11 it ]) = [.anaphor .bare .one 0] := by
  decide

/-- "Put a +1/+1 counter on each of up to two target creatures." -/
theorem okDistributedCounterRecipient :
    Instruction.check []
      (.putCounters (.lit 1) p11 (.eachOf (.described (.target (upTo 2)) creature))) = [] := by
  decide

/-- "Put a +1/+1 counter on up to two target creatures." -/
theorem badBarePluralCounterRecipient :
    Instruction.check [] (.putCounters (.lit 1) p11 (.described (.target (upTo 2)) creature))
      = [.perMember] := by
  decide

/-- "This deals 1 damage to each of up to two target creatures." -/
theorem okDistributedDamageRecipient :
    Instruction.check []
      (.dealDamage .this (.lit 1) (.eachOf (.described (.target (upTo 2)) creature))) = [] := by
  decide

/-- "This deals 1 damage to up to two target creatures." -/
theorem badBarePluralDamageRecipient :
    Instruction.check [] (.dealDamage .this (.lit 1) (.described (.target (upTo 2)) creature))
      = [.perMember] := by
  decide

/-- "Choose any number of target creatures. Put a +1/+1 counter on them." -/
theorem badThemCounterRecipient :
    Instruction.check []
      (.sequentially
        [choose (.described (.target anyNumber) creature), .putCounters (.lit 1) p11 them])
      = [.perMember] := by
  decide

/-- "Exile target creature with a +1/+1 counter on it." -/
theorem okExileWithCounterRider :
    Instruction.check []
      (.enact none "Exile"
        (.move (target creature) exileZone [.withCounters (.lit 1) p11 .fresh])) = [] := by
  decide

/-- "Exile target creature tapped." -/
theorem badExileTapped :
    Instruction.check []
      (.enact none "Exile" (.move (target creature) exileZone [.entersAs .tapped]))
      = [.ridersFit] := by
  decide

/-- "Put a +1/+1 counter on target creature." -/
theorem okPutBoostCounterOnCreature :
    Instruction.check [] (.putCounters (.lit 1) p11 (target creature)) = [] := by decide

/-- "Put a poison counter on target creature." -/
theorem badPutPoisonOnCreature :
    Instruction.check [] (.putCounters (.lit 1) (.printed (.named "Poison")) (target creature))
      = [.counterSourceScope] := by
  decide

/-- "You get a +1/+1 counter." -/
theorem badGetsBoostCounter :
    Instruction.check [] (.putCounters (.lit 1) p11 .you) = [.counterSourceScope] := by decide

/-- "a +1/+1 counter": a counter kind adds to or subtracts from power and toughness
[CR#122.1a]; it never sets either to a number. -/
theorem okBoostCounterKind : CounterKind.check (.boost (.up 1) (.up 1)) = [] := by decide

/-- "a 1/1 counter" -/
theorem badSetBoostCounter :
    CounterKind.check (.boost (.set 1) (.up 1)) = [.knownCounter] := by decide

/-- "Each opponent loses all poison counters." -/
theorem okLosesAllPoisonCounters :
    Instruction.check [] (.losesCounters (each .opponent) (some (.printed (.named "Poison"))) none)
      = [] := by
  decide

/-- "Each opponent loses all +1/+1 counters." -/
theorem badLosesAllBoostCounters :
    Instruction.check [] (.losesCounters (each .opponent) (some p11) none)
      = [.counterSourceScope] := by
  decide

/-- "the number of +1/+1 counters on this creature" -/
theorem okCountersHeldByObject :
    Amount.check [] (countersOn plusOnePlusOne thisCreature) = [] := by decide

/-- "the number of +1/+1 counters you have" -/
theorem badCountersHeldByPlayer :
    Amount.check [] (countersOn plusOnePlusOne .you) = [.kindMismatch .object .player] := by
  decide

/-- "When the last +1/+1 counter is removed from this creature, draw a card." -/
theorem okLastBoostCounterRemoved :
    Ability.check []
      (when (lastCounterRemoved plusOnePlusOne thisCreature) (draw .you (.lit 1)))
      = [] := by
  decide

/-- "When the last poison counter is removed from this creature, draw a card." -/
theorem badLastPoisonCounterRemoved :
    Ability.check []
      (when (lastCounterRemoved (.named "Poison") thisCreature) (draw .you (.lit 1)))
      = [.counterKindNamed .object] := by
  decide

/-- "Put a charge counter on this artifact." -/
theorem okChargeCounterOnArtifact :
    Instruction.check [] (.putCounters (.lit 1) (.printed (.named "Charge")) thisArtifact)
      = [] := by
  decide

/-- "Each player gets a charge counter." -/
theorem badGetsChargeCounter :
    Instruction.check [] (.putCounters (.lit 1) (.printed (.named "Charge")) .you)
      = [.counterSourceScope] := by
  decide

/-- "… that many plus one +1/+1 counters are put on it instead" -/
theorem okManyCounterBatchSize :
    StaticSpec.check []
      (.intercepts
        (.counterEvent .put (some plusOnePlusOne) (a creatureYouControl) .many none false) [] none
        (.putCounters (plus .thatMuch (.lit 1)) p11 it) .repeatedly none) = [] := by
  decide

theorem badSingularCounterBatchSize :
    StaticSpec.check []
      (.intercepts
        (.counterEvent .put (some plusOnePlusOne) (a creatureYouControl) .one none false) [] none
        (.putCounters (plus .thatMuch (.lit 1)) p11 it) .repeatedly none)
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Whenever one or more +1/+1 counters are put on a creature you control, …" -/
theorem okUncausedCounterWithAgent :
    GameEvent.check []
      (.counterEvent .put (some plusOnePlusOne) (a creatureYouControl) .many (some .you) false)
      = [] := by
  decide

theorem badCausedCounterWithAgent :
    GameEvent.check []
      (.counterEvent .put (some plusOnePlusOne) (a creatureYouControl) .many (some .you) true)
      = [.causedByOk] := by
  decide

/-- "each creature with a +1/+1 counter on it" -/
theorem okPlusOneCounterDescription :
    Predicate.check .object [] (.hasCounters (some plusOnePlusOne)) = [] := by decide

/-- "each creature with a poison counter on it" -/
theorem badPoisonCounterDescription :
    Predicate.check .object [] (.hasCounters (some (.named "Poison")))
      = [.counterKindNamed .object] := by
  decide

/-- "a first strike counter" -/
theorem okFirstStrikeKeywordCounter : CounterKind.check (.keyword "FirstStrike") = [] := by
  decide

/-- "a cumulative upkeep counter" -/
theorem badCumulativeUpkeepCounter :
    CounterKind.check (.keyword "CumulativeUpkeep") = [.knownCounter] := by decide

/-- "your choice of a +1/+1 counter or a first strike counter on it." -/
theorem okCounterMenu :
    StaticSpec.check []
      (.entersRider thisCreature
        (.withCounters (.lit 1) (.chosen [plusOnePlusOne, .keyword "FirstStrike"]) .fresh))
      = [] := by
  decide

/-- "This creature enters with your choice of a counter on it." -/
theorem badEmptyCounterMenu :
    StaticSpec.check []
      (.entersRider thisCreature (.withCounters (.lit 1) (.chosen []) .fresh)) = [.nonEmpty] := by
  decide

/-- "Put your choice of a +1/+1 counter or a first strike counter on target creature." -/
theorem okSameScopeCounterMenu :
    Instruction.check []
      (.putCounters (.lit 1) (.chosen [plusOnePlusOne, .keyword "FirstStrike"]) (target creature))
      = [] := by
  decide

/-- "Put your choice of a +1/+1 counter or a poison counter on target creature." -/
theorem badMixedScopeCounterMenu :
    Instruction.check []
      (.putCounters (.lit 1) (.chosen [plusOnePlusOne, .named "Poison"]) (target creature))
      = [.counterSourceScope] := by
  decide

/-- "Put a poison counter on target opponent." -/
theorem okPoisonCounterLabel :
    Instruction.check [] (.putCounters (.lit 1) (.printed (.named "Poison")) (target .opponent))
      = [] := by
  decide

/-- "Put a zorp counter on target creature." -/
theorem badUnknownCounterLabel :
    Instruction.check [] (.putCounters (.lit 1) (.printed (.named "Zorp")) (target creature))
      = [.knownCounter] := by
  decide

/-- "Put a flying counter on target creature." [CR#122.1b] -/
theorem badKeywordCounterNamedPlainly :
    Instruction.check [] (.putCounters (.lit 1) (.printed (.named "Flying")) (target creature))
      = [.knownCounter] := by
  decide

def afterCountersPut : Bindings :=
  GameEvent.after []
    (.counterEvent .put none (a (.and [creature, .otherThan .this])) .many (some .you) false)

/-- "Put that many counters of each of those kinds on this creature." -/
theorem okThoseKindsAfterCountersPut :
    Instruction.check afterCountersPut (.putCounters .thatMuch .those .this) = [] := by decide

/-- "Put a counter of each of those kinds on target creature." -/
theorem badThoseKindsUnannounced :
    Instruction.check [] (.putCounters (.lit 1) .those (target creature))
      = [.outcomeInScope .countersPut 0] := by
  decide

/-- "If ... +1/+1 counters ... that many plus one are put instead." -/
theorem okInterceptsCounterEvent :
    StaticSpec.check []
      (.intercepts
        (.counterEvent .put (some plusOnePlusOne) (a creatureYouControl) .many none false) [] none
        (.putCounters (plus .thatMuch (.lit 1)) p11 it) .repeatedly none) = [] := by
  decide

theorem badTriggeringReplaced :
    StaticSpec.check []
      (.intercepts
        (.triggers
          (a (.and
            [ .abilityHead .anyTriggered,
              .abilityOf (a (.and [permanent, .hasPossessor .controller .you])) ])))
        [] none (draw .you (.lit 1)) .repeatedly none) = [.interceptable] := by
  decide

/-- "Target creature becomes an artifact in addition to its other types." -/
theorem okBecomesArtifact :
    Instruction.check []
      (becomes (target creature) { characteristics := { types := [.artifact] } } none) = [] := by
  decide

/-- "Target land becomes a Zombie in addition to its other types." -/
theorem badBecomesZombieLand :
    Instruction.check []
      (becomes (target land) { characteristics := { subtypes := [creatureType "Zombie"] } } none)
      = [.becomesOk] := by
  decide

/-- "Target creature becomes in addition to its other types." -/
theorem badBecomesNothing :
    Instruction.check [] (becomes (target creature) { characteristics := {} } none)
      = [.becomesOk] := by
  decide

/-- "Target creature becomes a creature in addition to its other types." -/
theorem badBecomesOwnType :
    Instruction.check []
      (becomes (target creature) { characteristics := { types := [.creature] } } none)
      = [.becomesOk] := by
  decide

/-- The Idris pin refutes the anaphor; the unresolved `it` has no zone either. -/
theorem badOtherwiseReadsIfArm :
    Instruction.check []
      (.onlyIf (create (.lit 1) (creatureToken 1 1 [.black] [creatureType "Zombie"]))
        (exists_ creatureYouControl) (some (.setStatus .tapped it))) = [.anaphor .bare .one 0, .zoneIs .battlefield] := by
  decide

/-- "Create a 1/1 black Zombie creature token. Create two of those tokens." -/
theorem okAnaphoricTokenAfterToken :
    Instruction.check []
      (.sequentially
        [ create (.lit 1) (creatureToken 1 1 [.black] [creatureType "Zombie"]),
          .create .you (.lit 2) .asThose [] ]) = [] := by
  decide

/-- "Destroy target creature. Create two of those tokens." -/
theorem badAnaphoricTokenAfterNonToken :
    Instruction.check []
      (.sequentially [destroy (target creature), .create .you (.lit 2) .asThose []])
      = [.tokenSpecInScope 0] := by
  decide

/-- "Put a +1/+1 counter on target creature card in your graveyard." No printed card on the
bench. -/
theorem counterOnGraveyardCard :
    Instruction.check []
      (.putCounters (.lit 1) p11 (target (.and [creature, .inZone (graveyardOf .you)]))) = [] := by
  decide

/-- "put a counter of each kind that's on this creature on target creature" -/
theorem okSameKindsOnCreature :
    Instruction.check [] (.putCounters (.lit 1) (.sameAs thisCreature) (target creature))
      = [] := by
  decide

/-- "put a counter of each kind that's on this creature on target player": a +X/+Y counter
modifies an object's power and toughness [CR#122.1a], so the kinds read off a creature stay on
objects. -/
theorem badSameKindsOnPlayer :
    Instruction.check [] (.putCounters (.lit 1) (.sameAs thisCreature) (target .anyPlayer))
      = [.counterSourceScope] := by
  decide

/-- "put seven counters of each kind that's on this creature on target creature": the
same-counters reading carries the number as well as the kinds, so the amount slot holds no
other literal. -/
theorem badSameKindsSevenEach :
    Instruction.check [] (.putCounters (.lit 7) (.sameAs thisCreature) (target creature))
      = [.kindAmountOk] := by
  decide

/-- "a permanent your team controls" -/
def teamPermanent : NounPhrase :=
  a (.and [permanent, .hasPossessor .controller (.playerGroup .yourTeam)])

/-- "If one or more counters would be put on a permanent your team controls, that many plus
one of each of those kinds are put on that permanent instead." The replaced event names no
agent. -/
theorem okAgentlessCounterReplacement :
    StaticSpec.check []
      (.intercepts (.counterEvent .put none teamPermanent .many none false) [] none
        (.putCounters (plus .thatMuch (.lit 1)) .those (that .permanent)) .repeatedly none)
      = [] := by
  decide

/-- "one or more counters would be put on a permanent your team controls" -/
theorem okAgentlessCounterPutEvent :
    GameEvent.check [] (.counterEvent .put none teamPermanent .many none false) = [] := by decide

/-- The same event claiming both a player who puts the counters and an effect that causes
them: a counter-put event has one cause, not two. -/
theorem badCounterEventAgentAndEffect :
    GameEvent.check [] (.counterEvent .put none teamPermanent .many (some .you) true)
      = [.causedByOk] := by
  decide

/-- "{T}: Exile a card from your hand and put four time counters on it. Then remove a time
counter from each other card you own in exile." "other" is anchored on the card this same
ability exiled and granted to. -/
theorem okOtherThanExiledByThisAbility :
    Ability.check []
      (activated
        (.compound [.tapSymbol, .perform (exile (a (.and [.not land, .inZone (handOf .you)])))])
        (.sequentially
          [ .putCounters (.lit 4) (.printed (.named "Time"))
              (theVerbed "Exile" .card .attributive .one),
            .removeCounters (some (exactly 1)) (some (.printed (.named "Time")))
              (each (.and
                [ .otherThan (theVerbed "Exile" .card .attributive .one),
                  .hasPossessor .owner .you, .inZone exileZone ])) ])) = [] := by
  decide

/-- The same last sentence with a bare "other": with nothing announced, there is no anchor for
the complement to be other THAN. The Idris pin refutes the anchor; the list also names the
targeting law the same bare "other" fails. -/
theorem badBareOtherWithoutAnchor :
    Ability.check []
      (activated .tapSymbol
        (.removeCounters (some (exactly 1)) (some (.printed (.named "Time")))
          (each (.and [.other, .hasPossessor .owner .you, .inZone exileZone]))))
      = [.anyTargeted .object, .otherAnchored] := by
  decide

end Semantics.Proofs.Counters
