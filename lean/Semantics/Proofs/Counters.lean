import Semantics.Macros
import Semantics.Check.Card
import Semantics.Check.Rules
import Semantics.Macros.Declared

/-!
# Semantics.Proofs.Counters

Port of `idris/src/Experimental/Proofs/Counters.idr`: the pins of the Counters family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Counters

/-- "a +1/+1 counter", printed. -/
def p11 : CounterKindSource := .printed p1p1Counter

/-- "of the chosen creature type" -/
theorem okChosenCreatureType :
    Predicate.checkAt .object
      [({ det := .a, plur := .one, payload := .quality (.subtype .creature), idents := [] } : Referent)]
      (ofChosen (.subtype .creature)) = [] := by
  decide

/-- "of the chosen creature type" -/
theorem badChosenWrongSort :
    Predicate.checkAt .object [({ det := .a, plur := .one, payload := .quality .color, idents := [] } : Referent)]
      (ofChosen (.subtype .creature))
      = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

/-- "a counter of that kind" -/
theorem badChosenCounterKindRead :
    Predicate.checkAt .object [({ det := .a, plur := .one, payload := .quality .counterKind, idents := [] } : Referent)]
      (ofChosen .counterKind) = [.chosenQualityRead .counterKind] := by
  decide

/-- "this creature" -/
theorem okAscribedSelf :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.asType .creature .this none) = [] := by decide

theorem typeViewRefinesTheSelectedTargetInPlace :
    let noun := NounPhrase.asType .creature (target creature) none
    let after := (stepNounPhrase 64 (DRS.ofList []) .other false noun).1
    NounPhrase.checkAt (some .object) (DRS.ofList []) noun = [] ∧
      after.visibleReferents.map (·.id) = [0] ∧
      (after.findById? 0).map (fun r => (r.det, r.ty)) = some (.target, [.creature]) := by decide

/-- "creature you control that isn't attacking" -/
theorem okConsistentConjunction :
    Predicate.checkAt .object (DRS.ofList []) (.and [creature, .hasPossessor .controller .you, .not attacking])
      = [] := by
  decide

/-- "creature you control that you don't control" -/
theorem okControlSelfNegation :
    Predicate.checkAt .object (DRS.ofList [])
      (.and [.hasPossessor .controller .you, .not (.hasPossessor .controller .you)]) = [] := by
  decide

/-- "attacking noncreature" -/
theorem badAttackingNoncreature :
    Predicate.checkAt .object (DRS.ofList []) (.and [attacking, .not creature]) = [.contradictionFree] := by
  decide

/-- "Whenever this creature blocks a creature or becomes blocked by a creature, that creature
gets −1/−1 until end of turn." -/
theorem okAltHeaderAgreeingReadback :
    Ability.check (DRS.ofList [])
      (.triggered (blocks thisCreature (some (a creature)))
        [becomesBlocked thisCreature (some (a creature))] none [] none none none
        (get (that (.type .creature)) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn)))
      = [] := by
  decide

theorem agreeingAlternativeHeadersSelectTheOtherCreature :
    let after := (stepTriggerHeaderWith (stepOps 64) (DRS.ofList [])
      (blocks thisCreature (some (a creature)))
      [becomesBlocked thisCreature (some (a creature))] none none).1
    (firstReach (.word (.type .creature)) .one after).map (·.id) = some 2 ∧
      (after.findById? 2).map (·.det) = some .a ∧
      (after.findById? 0).map (·.det) = some .the := by decide

theorem alternateHeadersPublishTheirSelf :
    Ability.check (DRS.ofList [])
      (.triggered (blocks thisCreature none)
        [becomesBlocked thisCreature (some (a creature))] none [] none none none
        (get (that (.type .creature)) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn)))
      = [] ∧
      (firstReach (.word (.type .creature)) .one
        (nextClause (GameEvent.extend (DRS.ofList []) (blocks thisCreature none)))).map (·.id) = some 0 := by
  decide

theorem threeHeadersPublishTheirSelf :
    Ability.check (DRS.ofList [])
      (.triggered (attacks thisCreature)
        [blocks thisCreature none, .does (.by (a spell)) (.target thisCreature)] none [] none none none
        (Instruction.instruct thisCreature (.dealDamage (powerOf (that (.type .creature))) (each .opponent) .any)))
      = [] ∧
      (firstReach (.word (.type .creature)) .one
        (nextClause (GameEvent.extend (DRS.ofList []) (attacks thisCreature)))).map (·.id) = some 0 := by
  decide

theorem badJoinedHeaderReadback :
    Ability.check (DRS.ofList [])
      (.triggered (Declared.enters thisCreature none) [] none
        [⟨Declared.dies (a creatureYouControl), [], none, none⟩] none none none
        (.putCounters (.lit 1) p11 it .one true)) = [.anaphor .bare .one 0] := by
  decide

/-- "Remove a +1/+1 counter from target creature." -/
theorem okRemoveCounterFromTarget :
    Instruction.check (DRS.ofList []) (.removeCounters (some (exactly 1)) (some p11) (target creature) .one true)
      = [] := by
  decide

theorem counterRemovalReadsTheFreshMovedCard :
    let destroyed := destroy (target creature)
    let after := (stepInstruction 64 .postEvent .obj (DRS.ofList []) destroyed).1
    Instruction.check (DRS.ofList [])
      (.sequentially [destroyed, .removeCounters (some (exactly 1)) (some p11) it .one true]) = [] ∧
      after.findById? 0 = none ∧
      (firstReach .bare .one (nextClause after)).map (·.id) = some 1 ∧
      (after.findById? 1).map (fun r => (r.zone, r.idents)) = some (some .graveyard, []) := by decide

/-- "Move a counter from target creature onto this creature." -/
theorem okMoveCounterOntoThis :
    Instruction.check (DRS.ofList []) (.moveCounters (.lit 1) none (target creature) thisCreature) = [] := by
  decide

/-- "Create a 1/1 creature creature token." -/
theorem badTokenDuplicateType :
    Instruction.check (DRS.ofList [])
      (create (.lit 1)
        { characteristics :=
          { types := [.creature, .creature], power := some (.lit 1), toughness := some (.lit 1) } })
      = [.distinct] := by
  decide

/-- "Create a 1/1 white white Soldier creature token." -/
theorem badTokenDuplicateColor :
    Instruction.check (DRS.ofList [])
      (create (.lit 1) (creatureToken 1 1 [.white, .white] [creatureType "Soldier"]))
      = [.distinct] := by
  decide

/-- "Target opponent loses 2 life. You gain that much life." -/
theorem okThatMuchAfterOutcome :
    Instruction.check (DRS.ofList [])
      (.sequentially [instruct (target .opponent) (loseLife (.lit 2)), gainLife .thatMuch]) = [] := by
  decide

/-- "This deals 2 damage to target creature and you gain that much life." -/
theorem badSimultaneousReadsOutcome :
    Instruction.check (DRS.ofList [])
      (.simultaneously [(Instruction.instruct .this (.dealDamage (.lit 2) (target creature) .any)), gainLife .thatMuch])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "You may create a token and put a +1/+1 counter on it." -/
theorem badSimultaneousReadsMayDeed :
    Instruction.check (DRS.ofList [])
      (.simultaneously
        [ offer (create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Plant"])),
          (.putCounters (.lit 1) p11 it .one true) ]) = [.anaphor .bare .one 0] := by
  decide

/-- "You may have this deal 2 damage and you gain that much life." -/
theorem badSimultaneousReadsMayOutcome :
    Instruction.check (DRS.ofList [])
      (.simultaneously
        [offer (Instruction.instruct .this (.dealDamage (.lit 2) (target creature) .any)), gainLife .thatMuch])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Create a Plant token and a Soldier token. Put a +1/+1 counter on it." -/
theorem badBatchTwoCreatesThenIt :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .simultaneously
            [ create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Plant"]),
              create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"]) ],
          (.putCounters (.lit 1) p11 it .one true) ]) = [.anaphor .bare .one 2] := by
  decide

theorem badBatchTwoOutcomesThenThatMuch :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ .simultaneously
            [(Instruction.instruct .this (.dealDamage (.lit 2) (target creature) .any)), instruct (target .opponent) (loseLife (.lit 3))],
          gainLife .thatMuch ]) = [.quantOutcomeInScope 2] := by
  decide

/-- "Create a 1/1 green Plant creature token. Put a +1/+1 counter on it." -/
theorem okCreatedThenCountered :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ Primitives.Instruction.create (.lit 1) (.written (creatureToken 1 1 [.green] [creatureType "Plant"])) [],
          (.putCounters (.lit 1) p11 it .one true) ]) = [] := by
  decide

theorem badDistributedCreationIt :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ instruct (each .anyPlayer) (Primitives.Instruction.create (.lit 1)
            (.written (creatureToken 1 1 [.green] [creatureType "Plant"])) []),
          (.putCounters (.lit 1) p11 it .one true) ]) = [.anaphor .bare .one 0] := by
  decide

/-- "Put a +1/+1 counter on each of up to two target creatures." -/
theorem okDistributedCounterRecipient :
    Instruction.check (DRS.ofList [])
      (.putCounters (.lit 1) p11 (.eachOf (.described (.target (upTo 2)) creature)) .one true) = [] := by
  decide

/-- "Put a +1/+1 counter on up to two target creatures." -/
theorem badBarePluralCounterRecipient :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) p11 (.described (.target (upTo 2)) creature) .one true)
      = [.perMember] := by
  decide

/-- "This deals 1 damage to each of up to two target creatures." -/
theorem okDistributedDamageRecipient :
    Instruction.check (DRS.ofList [])
      (Instruction.instruct .this (.dealDamage (.lit 1) (.eachOf (.described (.target (upTo 2)) creature)) .any)) = [] := by
  decide

/-- "This deals 1 damage to up to two target creatures." -/
theorem badBarePluralDamageRecipient :
    Instruction.check (DRS.ofList []) (Instruction.instruct .this (.dealDamage (.lit 1) (.described (.target (upTo 2)) creature) .any))
      = [.perMember] := by
  decide

/-- "Choose any number of target creatures. Put a +1/+1 counter on them." -/
theorem badThemCounterRecipient :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [choose (.described (.target anyNumber) creature), (.putCounters (.lit 1) p11 them .one true)])
      = [.perMember] := by
  decide

/-- "Exile target creature with a +1/+1 counter on it." -/
theorem okExileWithCounterRider :
    Instruction.check (DRS.ofList [])
      (.enact (.action "exile") (.move (target creature) .anywhere (some exileZone) .arrival none [.placement (.withCounters (.lit 1) p11 .fresh)]) none none) =
            [] := by
  decide

/-- "Put a +1/+1 counter on target creature." -/
theorem okPutBoostCounterOnCreature :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) p11 (target creature) .one true) = [] := by decide

/-- "Put a poison counter on target creature." -/
theorem badPutPoisonOnCreature :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) (.printed (.named "poison")) (target creature) .one true)
      = [.counterSourceScope] := by
  decide

/-- "You get a +1/+1 counter." -/
theorem badGetsBoostCounter :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) p11 .you .one true) = [.counterSourceScope] := by decide

/-- "a +1/+1 counter": a +X/+Y counter is a declared kind like any other [CR#122.1a]. -/
theorem okBoostCounterKind : CounterKind.check p1p1Counter = [] := by decide

/-- "a 1/1 counter": a counter adds to or subtracts from power and toughness [CR#122.1a] and
never sets either, so no counter declaration is a 1/1 counter, and the name is unknown. -/
theorem badSetBoostCounter :
    CounterKind.check (.named "1/1Counter") = [.knownCounter] := by decide

/-- "Each opponent loses all poison counters." -/
theorem okLosesAllPoisonCounters :
    Instruction.check (DRS.ofList []) (loseCounters (some (.printed (.named "poison"))) none (agent := (each
        .opponent)))
      = [] := by
  decide

/-- "Each opponent loses all +1/+1 counters." -/
theorem badLosesAllBoostCounters :
    Instruction.check (DRS.ofList []) (loseCounters (some p11) none (agent := (each .opponent)))
      = [.counterSourceScope] := by
  decide

/-- "the number of +1/+1 counters on this creature" -/
theorem okCountersHeldByObject :
    Amount.check (DRS.ofList []) (countersOn p1p1Counter thisCreature) = [] := by decide

/-- "the number of +1/+1 counters you have" -/
theorem badCountersHeldByPlayer :
    Amount.check (DRS.ofList []) (countersOn p1p1Counter .you) = [.kindMismatch .object .player] := by
  decide

/-- "When the last +1/+1 counter is removed from this creature, draw a card." -/
theorem okLastBoostCounterRemoved :
    Ability.check (DRS.ofList [])
      (when (lastCounterRemoved p1p1Counter thisCreature) (draw (.lit 1)))
      = [] := by
  decide

/-- "When the last poison counter is removed from this creature, draw a card." -/
theorem badLastPoisonCounterRemoved :
    Ability.check (DRS.ofList [])
      (when (lastCounterRemoved (.named "poison") thisCreature) (draw (.lit 1)))
      = [.counterKindNamed .object] := by
  decide

/-- "Put a charge counter on this artifact." -/
theorem okChargeCounterOnArtifact :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) (.printed (.named "chargeCounter")) thisArtifact .one true)
      = [] := by
  decide

/-- "Each player gets a charge counter." -/
theorem badGetsChargeCounter :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) (.printed (.named "chargeCounter")) .you .one true)
      = [.counterSourceScope] := by
  decide

/-- "… that many plus one +1/+1 counters are put on it instead" -/
theorem okManyCounterBatchSize :
    StaticSpec.check (DRS.ofList [])
      (.instead (.would .any (.putCounters .anyAmount (.printed p1p1Counter) (a creatureYouControl) .many false)) (.putCounters (plus .thatMuch (.lit 1)) p11 it .one true) none .repeatedly none) = [] := by
  decide

theorem badSingularCounterBatchSize :
    StaticSpec.check (DRS.ofList [])
      (.instead (.would .any (.putCounters .anyAmount (.printed p1p1Counter) (a creatureYouControl) .one false)) (.putCounters (plus .thatMuch (.lit 1)) p11 it .one true) none .repeatedly none)
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Whenever one or more +1/+1 counters are put on a creature you control, …" -/
theorem okUncausedCounterWithAgent :
    GameEvent.check (DRS.ofList [])
      (.does (.actor .you) (.putCounters .anyAmount (.printed p1p1Counter) (a creatureYouControl) .many false))
      = [] := by
  decide

theorem badCausedCounterWithAgent :
    GameEvent.check (DRS.ofList [])
      (.does (.actor .you) (.putCounters .anyAmount (.printed p1p1Counter) (a creatureYouControl) .many true))
      = [.causedByOk] := by
  decide

/-- "each creature with a +1/+1 counter on it" -/
theorem okPlusOneCounterDescription :
    Predicate.checkAt .object (DRS.ofList []) (.hasCounters (some p1p1Counter)) = [] := by decide

/-- "each creature with a poison counter on it" -/
theorem badPoisonCounterDescription :
    Predicate.checkAt .object (DRS.ofList []) (.hasCounters (some (.named "poison")))
      = [.counterKindNamed .object] := by
  decide

/-- "a first strike counter" -/
theorem okFirstStrikeKeywordCounter :
    CounterKind.check (.named "firstStrikeCounter") = [] := by
  decide

/-- "a cumulative upkeep counter" -/
theorem badCumulativeUpkeepCounter :
    CounterKind.check (.named "cumulativeUpkeepCounter") = [.knownCounter] := by decide

/-- "your choice of a +1/+1 counter or a first strike counter on it." -/
theorem okCounterMenu :
    StaticSpec.check (DRS.ofList [])
      (.entryRider thisCreature
        (.withCounters (.lit 1) (.chosen [p1p1Counter, .named "firstStrikeCounter"]) .fresh))
      = [] := by
  decide

/-- "This creature enters with your choice of a counter on it." -/
theorem badEmptyCounterMenu :
    StaticSpec.check (DRS.ofList [])
      (.entryRider thisCreature (.withCounters (.lit 1) (.chosen []) .fresh)) = [.nonEmpty] := by
  decide

/-- "Put your choice of a +1/+1 counter or a first strike counter on target creature." -/
theorem okSameScopeCounterMenu :
    Instruction.check (DRS.ofList [])
      (.putCounters (.lit 1) (.chosen [p1p1Counter, .named "firstStrikeCounter"]) (target creature) .one true)
      = [] := by
  decide

/-- "Put your choice of a +1/+1 counter or a poison counter on target creature." -/
theorem badMixedScopeCounterMenu :
    Instruction.check (DRS.ofList [])
      (.putCounters (.lit 1) (.chosen [p1p1Counter, .named "poison"]) (target creature) .one true)
      = [.counterSourceScope] := by
  decide

/-- "Put a poison counter on target opponent." -/
theorem okPoisonCounterLabel :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) (.printed (.named "poison")) (target .opponent) .one true)
      = [] := by
  decide

/-- "Put a zorp counter on target creature." -/
theorem badUnknownCounterLabel :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) (.printed (.named "Zorp")) (target creature) .one true)
      = [.knownCounter] := by
  decide

/-- "Put a flying counter on target creature." [CR#122.1b], written with the keyword's label:
the kind is the declaration's name, `flyingCounter`, and "Flying" names no counter. -/
theorem badKeywordCounterNamedPlainly :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) (.printed (.named "Flying")) (target creature) .one true)
      = [.knownCounter] := by
  decide

/-- "Put a flying counter on target creature." [CR#122.1b], written with the declared name. -/
theorem okKeywordCounterByName :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) (.printed flyingCounter) (target creature) .one true)
      = [] := by
  decide

def afterCountersPut : DRS :=
  GameEvent.extend (DRS.ofList [])
    (.does (.actor .you) (.putCounters .anyAmount .anyKind (a (.and [creature, .otherThan .this])) .many false))

/-- "Put that many counters of each of those kinds on this creature." -/
theorem okThoseKindsAfterCountersPut :
    Instruction.check afterCountersPut (.putCounters .thatMuch .those .this .one true) = [] := by decide

/-- "Put a counter of each of those kinds on target creature." -/
theorem badThoseKindsUnannounced :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) .those (target creature) .one true)
      = [.outcomeInScope .countersPut 0] := by
  decide

/-- "If ... +1/+1 counters ... that many plus one are put instead." -/
theorem okInterceptsCounterEvent :
    StaticSpec.check (DRS.ofList [])
      (.instead (.would .any (.putCounters .anyAmount (.printed p1p1Counter) (a creatureYouControl) .many false)) (.putCounters (plus .thatMuch (.lit 1)) p11 it .one true) none .repeatedly none) = [] := by
  decide

/-- "Target creature becomes an artifact in addition to its other types." -/
theorem okBecomesArtifact :
    Instruction.check (DRS.ofList [])
      (become (target creature) { characteristics := { types := [.artifact] } } none) = [] := by
  decide

/-- "Target creature becomes in addition to its other types." -/
theorem badBecomesNothing :
    Instruction.check (DRS.ofList []) (become (target creature) { characteristics := {} } none)
      = [.lineNonEmpty] := by
  decide

/-- The Idris pin refutes the anaphor. -/
theorem badOtherwiseReadsIfArm :
    Instruction.check (DRS.ofList [])
      (.doOnlyIf (create (.lit 1) (creatureToken 1 1 [.black] [creatureType "Zombie"]))
        (exists_ creatureYouControl) (some (.setStatus .tapped it))) = [.anaphor .bare .one 0] := by
  decide

/-- "Create a 1/1 black Zombie creature token. Create two of those tokens." -/
theorem okAnaphoricTokenAfterToken :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [ create (.lit 1) (creatureToken 1 1 [.black] [creatureType "Zombie"]),
          Primitives.Instruction.create (.lit 2) .asThose [] ]) = [] := by
  decide

/-- "Destroy target creature. Create two of those tokens." -/
theorem badAnaphoricTokenAfterNonToken :
    Instruction.check (DRS.ofList [])
      (.sequentially [destroy (target creature), Primitives.Instruction.create (.lit 2) .asThose []])
      = [.tokenSpecInScope 0] := by
  decide

theorem badAnaphoricTokenAfterTwoDistinctSpecs :
    Instruction.check (DRS.ofList [])
      (.sequentially
        [create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Plant"]),
          create (.lit 1) (creatureToken 1 1 [.white] [creatureType "Soldier"]),
          Primitives.Instruction.create (.lit 1) .asThose []]) = [.tokenSpecInScope 2] := by decide

/-- "Put a +1/+1 counter on target creature card in your graveyard." No printed card on the
bench. -/
theorem counterOnGraveyardCard :
    Instruction.check (DRS.ofList [])
      (.putCounters (.lit 1) p11 (target (.and [creature, .inZone (graveyardOf .you)])) .one true) = [] := by
  decide

/-- "put a counter of each kind that's on this creature on target creature" -/
theorem okSameKindsOnCreature :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) (.sameAs thisCreature) (target creature) .one true)
      = [] := by
  decide

/-- "put a counter of each kind that's on this creature on target player": a +X/+Y counter
modifies an object's power and toughness [CR#122.1a], so the kinds read off a creature stay on
objects. -/
theorem badSameKindsOnPlayer :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 1) (.sameAs thisCreature) (target .anyPlayer) .one true)
      = [.counterSourceScope] := by
  decide

/-- "put seven counters of each kind that's on this creature on target creature": the
same-counters reading carries the number as well as the kinds, so the amount slot holds no
other literal. -/
theorem badSameKindsSevenEach :
    Instruction.check (DRS.ofList []) (.putCounters (.lit 7) (.sameAs thisCreature) (target creature) .one true)
      = [.kindAmountOk] := by
  decide

/-- "a permanent your team controls" -/
def teamPermanent : NounPhrase :=
  a (.and [permanent, .hasPossessor .controller (.playerGroup .yourTeam)])

/-- "If one or more counters would be put on a permanent your team controls, that many plus
one of each of those kinds are put on that permanent instead." The replaced event names no
agent. -/
theorem okAgentlessCounterReplacement :
    StaticSpec.check (DRS.ofList [])
      (.instead (.would .any (.putCounters .anyAmount .anyKind teamPermanent .many false)) (.putCounters (plus .thatMuch (.lit 1)) .those (that .permanent) .one true) none .repeatedly none)
      = [] := by
  decide

/-- "one or more counters would be put on a permanent your team controls" -/
theorem okAgentlessCounterPutEvent :
    GameEvent.check (DRS.ofList []) (.does .any (.putCounters .anyAmount .anyKind teamPermanent .many false)) = [] := by decide

/-- The same event claiming both a player who puts the counters and an effect that causes
them: a counter-put event has one cause, not two. -/
theorem badCounterEventAgentAndEffect :
    GameEvent.check (DRS.ofList []) (.does (.actor .you) (.putCounters .anyAmount .anyKind teamPermanent .many true))
      = [.causedByOk] := by
  decide

/-- "{T}: Exile a card from your hand and put four time counters on it. Then remove a time
counter from each other card you own in exile." "other" is anchored on the card this same
ability exiled and granted to. -/
theorem okOtherThanExiledByThisAbility :
    Ability.check (DRS.ofList [])
      (activated
        (.compound [.tapSymbol, .perform (exile (a (.and [.not land, .inZone (handOf .you)])))])
        (.sequentially
          [ (.putCounters (.lit 4) (.printed (.named "timeCounter")) (theVerbed (.action "exile") .one (some .card)) .one true),
            (.removeCounters (some (exactly 1)) (some (.printed (.named "timeCounter"))) (each (.and
                [ .otherThan (theVerbed (.action "exile") .one (some .card)),
                  .hasPossessor .owner .you, .inZone exileZone ])) .one true) ])) = [] := by
  decide

/-- The same last sentence with a bare "other": with nothing announced, there is no anchor for
the complement to be other THAN. The Idris pin refutes the anchor; the list also names the
targeting law the same bare "other" fails. -/
theorem badBareOtherWithoutAnchor :
    Ability.check (DRS.ofList [])
      (activated .tapSymbol
        (.removeCounters (some (exactly 1)) (some (.printed (.named "timeCounter"))) (each (.and [.other, .hasPossessor .owner .you, .inZone exileZone])) .one true))
      = [.anyTargeted .object, .otherAnchored] := by
  decide

/-! ## The +1/+1 and −1/−1 annihilation rule and "the smaller of" -/

/-- "N, where N is the smaller of the number of +1/+1 and −1/−1 counters on it" [CR#704.5q]. -/
def annihilationCount : Amount :=
  smallerOf (countersOn p1p1Counter .this) (countersOn m1m1Counter .this)

/-- The state-based conferral on the −1/−1 counter's definition [CR#704.5q,122.3]: "If a
permanent has both a +1/+1 counter and a −1/−1 counter on it, N +1/+1 and N −1/−1 counters
are removed from it", the two removals at once, as one action of the check [CR#704.3]. It
reads the count of its own kind, which a state-based conferral may (ruling 2026-10-10). -/
def annihilation : Conferral :=
  .stateBased (.matches .this (.and [.hasCounters (some p1p1Counter), .hasCounters (some m1m1Counter)]))
    (.simultaneously
      [ (.removeCounters (some (.exactlyOf annihilationCount)) (some (.printed p1p1Counter)) .this .one true),
        (.removeCounters (some (.exactlyOf annihilationCount)) (some (.printed m1m1Counter)) .this .one true) ])

/-- "the smaller of" two counts is an amount, and so is each of them. -/
theorem okAnnihilationCount : Amount.check (DRS.ofList []) annihilationCount = [] := by decide

/-- The rule proves as a conferral. -/
theorem okAnnihilation : Conferral.check annihilation = [] := by decide

/-- The −1/−1 counter's definition: its per-counter boost and its once-per-holder rule. -/
theorem okM1m1CounterDefinition :
    Definition.check
      (.counter m1m1Counter .object [boost (.down (.lit 1)) (.down (.lit 1)), annihilation])
      = [] := by
  decide

/-- The listed amounts are checked as amounts: "that much" with no outcome in scope is refused
inside "the smaller of" exactly as it is outside it. -/
theorem badSmallerOfUnscopedThatMuch :
    Amount.check (DRS.ofList []) (smallerOf (.lit 1) .thatMuch) = [.quantOutcomeInScope 0] := by decide

/-- Snowblind's Y, the non-rule witness: "Y is equal to X or to enchanted creature's toughness
minus 1, whichever is smaller", X read in its "otherwise" branch, "the number of snow lands its
controller controls". -/
theorem okSnowblindWhicheverIsSmaller :
    Amount.check (DRS.ofList [])
      (smallerOf
        (countOf (.and [.hasSupertype .snow, land,
          .hasPossessor .controller (controllerOf (.attachHost .enchanted (.type .creature)))]))
        (minus (toughnessOf (.attachHost .enchanted (.type .creature))) (.lit 1)))
      = [] := by
  decide

/-- The −1/−1 counter's declared name is its kind, everywhere, its own definition included
(ruling 2026-10-10): the name expands to the kind reference, never to the definition. -/
theorem m1m1CounterNameIsItsKind :
    Semantics.Macros.Declared.m1m1Counter = CounterKind.named "m1m1Counter" := rfl

/-- "whichever is greater" is the same aggregate under `max`. -/
theorem okGreaterOf :
    greaterOf (.lit 1) (.lit 2) = .aggregateOf .max [.lit 1, .lit 2] := rfl

end Semantics.Proofs.Counters
