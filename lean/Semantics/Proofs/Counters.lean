import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Check.Rules
import Semantics.Cards.Counters

/-!
# Semantics.Proofs.Counters

Port of `idris/src/Experimental/Proofs/Counters.idr`: the pins of the Counters family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

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
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.headed (.type .creature) .this) = [] := by decide

theorem badAscribedTarget :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.headed (.type .creature) (target isCreature))
      = [.ascribable] := by decide

theorem namedSelfTypeViewRetainsItsSelectedIdentity :
    let before := (stepNounPhrase 64 (DRS.ofList []) .other false
      (.firstMention "view.subject" .this)).1
    let noun := NounPhrase.headed (.type .creature) (.laterMention "view.subject" .this)
    let after := (stepNounPhrase 64 before .other false noun).1
    NounPhrase.checkAt (some .object) before noun = [] ∧
      after.nextId = before.nextId ∧
      after.visibleReferents.map (·.id) = [0] ∧
      (identityBinding "view.subject" after).map (fun r => (r.id, r.ty)) = some (0, [.creature]) := by decide

theorem badMarkerOnTarget :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.headed .spell (target isCreature)) =
      [.ascribable] := by decide

theorem namedSelfMarkerViewRetainsItsSelectedIdentity :
    let before := (stepNounPhrase 64 (DRS.ofList []) .other false
      (.firstMention "marker.subject" .this)).1
    let noun := NounPhrase.headed .spell (.laterMention "marker.subject" .this)
    let after := (stepNounPhrase 64 before .other false noun).1
    NounPhrase.checkAt (some .object) before noun = [] ∧
      after.nextId = before.nextId ∧
      (identityBinding "marker.subject" after).map (fun r => (r.id, r.zone)) = some (0, some .stack) := by decide

/-- "creature you control that isn't attacking" -/
theorem okConsistentConjunction :
    Predicate.checkAt .object (DRS.ofList []) (.and [isCreature, .hasPossessor .controller .you, .not isAttacking])
      = [] := by
  decide

/-- "creature you control that you don't control" -/
theorem okControlSelfNegation :
    Predicate.checkAt .object (DRS.ofList [])
      (.and [.hasPossessor .controller .you, .not (.hasPossessor .controller .you)]) = [] := by
  decide

/-- "Whenever this creature blocks a creature or becomes blocked by a creature, that creature
gets −1/−1 until end of turn." -/
theorem okAltHeaderAgreeingReadback :
    Ability.check (DRS.ofList [])
      (whenever (does thisCreature (block (a isCreature)))
        (or_ := [does (by_ (a isCreature)) (block thisCreature)])
        (get (that (.type .creature)) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn)))
      = [] := by
  decide

theorem agreeingAlternativeHeadersSelectTheOtherCreature :
    let after := (stepTriggerHeaderWith (stepOps 64) (DRS.ofList [])
      (does thisCreature (block (a isCreature)))
      [does (by_ (a isCreature)) (block thisCreature)] none none).1
    (firstReach (.word (.type .creature)) .one after).map (·.id) = some 2 ∧
      (after.findById? 2).map (·.det) = some .a ∧
      (after.findById? 0).map (·.det) = some .the := by decide

theorem alternateHeadersPublishTheirSelf :
    Ability.check (DRS.ofList [])
      (whenever (does thisCreature block)
        (or_ := [does (by_ (a isCreature)) (block thisCreature)])
        (get (that (.type .creature)) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn)))
      = [] ∧
      (firstReach (.word (.type .creature)) .one
        (nextClause (GameEvent.extend (DRS.ofList []) (does thisCreature block)))).map (·.id) = some 0 := by
  decide

theorem threeHeadersPublishTheirSelf :
    Ability.check (DRS.ofList [])
      (whenever (does thisCreature attack)
        (or_ := [does thisCreature block, .does (.by (a isSpell)) (.target thisCreature)])
        (Action.instruct thisCreature (.dealDamage (powerOf (that (.type .creature))) (each (.isOpponentOf .you)))))
      = [] ∧
      (firstReach (.word (.type .creature)) .one
        (nextClause (GameEvent.extend (DRS.ofList []) (does thisCreature attack)))).map (·.id) = some 0 := by
  decide

theorem badJoinedHeaderReadback :
    Ability.check (DRS.ofList [])
      (when (does thisCreature enter)
        (joins := [⟨does (a (and [isCreature, youControl])) die, [], none, none⟩])
        (.putCounters (.lit 1) p11 it .one true)) = [.anaphor .bare .one 0] := by
  decide

/-- "Remove a +1/+1 counter from target creature." -/
theorem okRemoveCounterFromTarget :
    Action.check (DRS.ofList []) (.removeCounters (some (exactly 1)) (some p11) (target isCreature) .one true)
      = [] := by
  decide

theorem destructionPreservesMovedCardIdentityWithoutCounterMemory :
    let destroyed := destroy (target isCreature)
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) destroyed).1
    after.findById? 0 = none ∧
      (firstReach .bare .one (nextClause after)).map (·.id) = some 1 ∧
      (after.findById? 1).map (fun r => (r.zone, r.idents)) = some (some .graveyard, []) := by decide

theorem newCountersOnMovedCardCanBeRemoved :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isCreature),
        .putCounters (.lit 1) p11 (that .card) .one true,
        .removeCounters (some (exactly 1)) (some p11) (that .card) .one true]) = [] := by decide

theorem countersPlacedDuringMoveCanBeRemoved :
    Action.check (DRS.ofList [])
      (.sequentially [
        .move (target isCreature) .anyLocation (some exileZone) .arrival none
          [(.withCounters (.lit 1) p11 .fresh)],
        .removeCounters (some (exactly 1)) (some p11) (that .card) .one true]) = [] := by decide

/-- "Move a counter from target creature onto this creature." -/
theorem okMoveCounterOntoThis :
    Action.check (DRS.ofList []) (moveCounters (.lit 1) (target isCreature) thisCreature) = [] := by
  decide

/-- "Move a +1/+1 counter from this creature onto target creature.": moving a counter removes it
and puts it [CR#122.5], so the move publishes both outcomes, once each. -/
theorem moveCounterPublishesRemovalAndPlacement :
    let after := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (moveCounters (.lit 1) thisCreature (target isCreature) (some p11))).1
    countOutcomes .countersRemoved after = 1 ∧ countOutcomes .countersPut after = 1 := by
  decide

/-- "Double the number of each kind of counter on target creature" [CR#701.10e]: counters are
put, so the doubling publishes that it put counters, as a group of every kind's. -/
theorem doublingCountersPublishesCountersPut :
    let after :=
      (stepAction 64 .postEvent .obj (DRS.ofList []) (doubleCounters (target isCreature))).1
    Action.check (DRS.ofList []) (doubleCounters (target isCreature)) = [] ∧
      after.visibleReferents.any (fun ref => ref.payload == .outcome .countersPut) = true := by
  decide

/-- "the number of counters of that kind on this creature" reads the one bound counter kind: with
none bound it has no kind to read, and with one bound it reads it. -/
theorem boundCounterReadsOnlyTheBoundKind :
    Amount.check (DRS.ofList []) (.statOf .boundCounter thisCreature) =
        [.choiceRef .theChoice (.quality .counterKind) 0] ∧
      Amount.check (DRS.ofList [qualityB .counterKind]) (.statOf .boundCounter thisCreature) = [] ∧
      Amount.check (DRS.ofList [qualityB .counterKind, qualityB .counterKind])
        (.statOf .boundCounter thisCreature) =
        [.choiceRef .theChoice (.quality .counterKind) 2] := by
  decide

/-- "Create a 1/1 creature creature token." -/
theorem badTokenDuplicateType :
    Action.check (DRS.ofList [])
      (create (.lit 1)
        (.written { characteristics :=
          { types := [.creature, .creature], power := some (.lit 1), toughness := some (.lit 1) } }))
      = [.distinct] := by
  decide

/-- "Create a 1/1 white white Soldier creature token." -/
theorem badTokenDuplicateColor :
    Action.check (DRS.ofList [])
      (create (.lit 1) (.written (creatureToken 1 1 [.white, .white] [soldier])))
      = [.distinct] := by
  decide

/-- "Target opponent loses 2 life. You gain that much life." -/
theorem okThatMuchAfterOutcome :
    Action.check (DRS.ofList [])
      (.sequentially [instruct (target (.isOpponentOf .you)) (loseLife (.lit 2)), gainLife .thatMuch]) = [] := by
  decide

/-- "This deals 2 damage to target creature and you gain that much life." -/
theorem badSimultaneousReadsOutcome :
    Action.check (DRS.ofList [])
      (.simultaneously [(Action.instruct .this (.dealDamage (.lit 2) (target isCreature))), gainLife .thatMuch])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "You may create a token and put a +1/+1 counter on it." -/
theorem badSimultaneousReadsMayDeed :
    Action.check (DRS.ofList [])
      (.simultaneously
        [ may (create (.lit 1) (.written (creatureToken 1 1 [.green] [plant]))),
          (.putCounters (.lit 1) p11 it .one true) ]) = [.anaphor .bare .one 0] := by
  decide

/-- "You may have this deal 2 damage and you gain that much life." -/
theorem badSimultaneousReadsMayOutcome :
    Action.check (DRS.ofList [])
      (.simultaneously
        [may (Action.instruct .this (.dealDamage (.lit 2) (target isCreature))), gainLife .thatMuch])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Create a Plant token and a Soldier token. Put a +1/+1 counter on it." -/
theorem badBatchTwoCreatesThenIt :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .simultaneously
            [ create (.lit 1) (.written (creatureToken 1 1 [.green] [plant])),
              create (.lit 1) (.written (creatureToken 1 1 [.white] [soldier])) ],
          (.putCounters (.lit 1) p11 it .one true) ]) = [.anaphor .bare .one 2] := by
  decide

theorem badBatchTwoOutcomesThenThatMuch :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .simultaneously
            [(Action.instruct .this (.dealDamage (.lit 2) (target isCreature))), instruct (target (.isOpponentOf .you)) (loseLife (.lit 3))],
          gainLife .thatMuch ]) = [.quantOutcomeInScope 2] := by
  decide

/-- "Create a 1/1 green Plant creature token. Put a +1/+1 counter on it." -/
theorem okCreatedThenCountered :
    Action.check (DRS.ofList [])
      (.sequentially
        [ create (.lit 1) (.written (creatureToken 1 1 [.green] [plant])) [],
          (.putCounters (.lit 1) p11 it .one true) ]) = [] := by
  decide

theorem badDistributedCreationIt :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (each .anyPlayer) (create (.lit 1)
            (.written (creatureToken 1 1 [.green] [plant])) []),
          (.putCounters (.lit 1) p11 it .one true) ]) = [.anaphor .bare .one 0] := by
  decide

/-- "Put a +1/+1 counter on each of up to two target creatures." -/
theorem okDistributedCounterRecipient :
    Action.check (DRS.ofList [])
      (.putCounters (.lit 1) p11 (.eachOf (.described (.target (upTo 2)) isCreature)) .one true) = [] := by
  decide

/-- "Put a +1/+1 counter on up to two target creatures." -/
theorem badBarePluralCounterRecipient :
    Action.check (DRS.ofList []) (.putCounters (.lit 1) p11 (.described (.target (upTo 2)) isCreature) .one true)
      = [.perMember] := by
  decide

/-- "This deals 1 damage to each of up to two target creatures." -/
theorem okDistributedDamageRecipient :
    Action.check (DRS.ofList [])
      (Action.instruct .this (.dealDamage (.lit 1) (.eachOf (.described (.target (upTo 2)) isCreature)))) = [] := by
  decide

/-- "This deals 1 damage to up to two target creatures." -/
theorem badBarePluralDamageRecipient :
    Action.check (DRS.ofList []) (Action.instruct .this (.dealDamage (.lit 1) (.described (.target (upTo 2)) isCreature)))
      = [.perMember] := by
  decide

/-- "Choose any number of target creatures. Put a +1/+1 counter on them." -/
theorem badThemCounterRecipient :
    Action.check (DRS.ofList [])
      (.sequentially
        [choose (.described (.target anyNumber) isCreature), (.putCounters (.lit 1) p11 them .one true)])
      = [.perMember] := by
  decide

/-- "Exile target creature with a +1/+1 counter on it." -/
theorem okExileWithCounterRider :
    Action.check (DRS.ofList [])
      (.enact (.action "exile") (.move (target isCreature) .anyLocation (some exileZone) .arrival none [(.withCounters (.lit 1) p11 .fresh)]) none none) =
            [] := by
  decide

/-- "Put a +1/+1 counter on target creature." -/
theorem okPutBoostCounterOnCreature :
    Action.check (DRS.ofList []) (.putCounters (.lit 1) p11 (target isCreature) .one true) = [] := by decide

/-- "a +1/+1 counter": a +X/+Y counter is a declared kind like any other [CR#122.1a]. -/
theorem okBoostCounterKind : CounterKind.check p1p1Counter = [] := by decide

/-- "a 1/1 counter": a counter adds to or subtracts from power and toughness [CR#122.1a] and
never sets either, so no counter declaration is a 1/1 counter, and the name is unknown. -/
theorem badSetBoostCounter :
    CounterKind.check (.named "1/1Counter") = [.knownCounter] := by decide

/-- "Each opponent loses all poison counters." -/
theorem okLosesAllPoisonCounters :
    Action.check (DRS.ofList []) (loseCounters (some (.printed (.named "poison"))) none (agent := each
        (.isOpponentOf .you)))
      = [] := by
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

/-- "Put a charge counter on this artifact." -/
theorem okChargeCounterOnArtifact :
    Action.check (DRS.ofList []) (.putCounters (.lit 1) (.printed (.named "chargeCounter")) thisArtifact .one true)
      = [] := by
  decide

/-- "… that many plus one +1/+1 counters are put on it instead" -/
theorem okManyCounterBatchSize :
    StaticSpec.check (DRS.ofList [])
      (.replace (.would .any (.putCounters .anyAmount (.printed p1p1Counter) (a (and [isCreature, youControl])) .many false))
        (instead (.putCounters (plus .thatMuch (.lit 1)) p11 it .one true))
        none .repeatedly none) = [] := by
  decide

theorem badSingularCounterBatchSize :
    StaticSpec.check (DRS.ofList [])
      (.replace (.would .any (.putCounters .anyAmount (.printed p1p1Counter) (a (and [isCreature, youControl])) .one false))
        (instead (.putCounters (plus .thatMuch (.lit 1)) p11 it .one true))
        none .repeatedly none)
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Whenever one or more +1/+1 counters are put on a creature you control, …" -/
theorem okUncausedCounterWithAgent :
    GameEvent.check (DRS.ofList [])
      (.does (.actor .you) (.putCounters .anyAmount (.printed p1p1Counter) (a (and [isCreature, youControl])) .many false))
      = [] := by
  decide

theorem badCausedCounterWithAgent :
    GameEvent.check (DRS.ofList [])
      (.does (.actor .you) (.putCounters .anyAmount (.printed p1p1Counter) (a (and [isCreature, youControl])) .many true))
      = [.causedByOk] := by
  decide

/-- "each creature with a +1/+1 counter on it" -/
theorem okPlusOneCounterDescription :
    Predicate.checkAt .object (DRS.ofList []) (.hasCounters (some p1p1Counter)) = [] := by decide

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
      (enters thisCreature
        [.withCounters (.lit 1) (.chosen [p1p1Counter, .named "firstStrikeCounter"]) .fresh])
      = [] := by
  decide

/-- "This creature enters with your choice of a counter on it." -/
theorem badEmptyCounterMenu :
    StaticSpec.check (DRS.ofList [])
      (enters thisCreature [.withCounters (.lit 1) (.chosen []) .fresh]) = [.nonEmpty] := by
  decide

/-- "Put your choice of a +1/+1 counter or a first strike counter on target creature." -/
theorem okSameScopeCounterMenu :
    Action.check (DRS.ofList [])
      (.putCounters (.lit 1) (.chosen [p1p1Counter, .named "firstStrikeCounter"]) (target isCreature) .one true)
      = [] := by
  decide

/-- "Put a poison counter on target opponent." -/
theorem okPoisonCounterLabel :
    Action.check (DRS.ofList []) (.putCounters (.lit 1) (.printed (.named "poison")) (target (.isOpponentOf .you)) .one true)
      = [] := by
  decide

/-- "Put a zorp counter on target creature." -/
theorem badUnknownCounterLabel :
    Action.check (DRS.ofList []) (.putCounters (.lit 1) (.printed (.named "Zorp")) (target isCreature) .one true)
      = [.knownCounter] := by
  decide

/-- "Put a flying counter on target creature." [CR#122.1b], written with the keyword's label:
the kind is the declaration's name, `flyingCounter`, and "Flying" names no counter. -/
theorem badKeywordCounterNamedPlainly :
    Action.check (DRS.ofList []) (.putCounters (.lit 1) (.printed (.named "Flying")) (target isCreature) .one true)
      = [.knownCounter] := by
  decide

/-- "Put a flying counter on target creature." [CR#122.1b], written with the declared name. -/
theorem okKeywordCounterByName :
    Action.check (DRS.ofList []) (.putCounters (.lit 1) (.printed flyingCounter) (target isCreature) .one true)
      = [] := by
  decide

def afterCountersPut : DRS :=
  GameEvent.extend (DRS.ofList [])
    (.does (.actor .you) (.putCounters .anyAmount .anyKind (a (.and [isCreature, .other])) .many false))

/-- "Put that many counters of each of those kinds on this creature." -/
theorem okThoseKindsAfterCountersPut :
    Action.check afterCountersPut (.putCounters .thatMuch .those .this .one true) = [] := by decide

/-- "Put a counter of each of those kinds on target creature." -/
theorem badThoseKindsUnannounced :
    Action.check (DRS.ofList []) (.putCounters (.lit 1) .those (target isCreature) .one true)
      = [.outcomeInScope .countersPut 0] := by
  decide

/-- "If ... +1/+1 counters ... that many plus one are put instead." -/
theorem okInterceptsCounterEvent :
    StaticSpec.check (DRS.ofList [])
      (.replace (.would .any (.putCounters .anyAmount (.printed p1p1Counter) (a (and [isCreature, youControl])) .many false))
        (instead (.putCounters (plus .thatMuch (.lit 1)) p11 it .one true))
        none .repeatedly none) = [] := by
  decide

/-- "Target creature becomes an artifact in addition to its other types." -/
theorem okBecomesArtifact :
    Action.check (DRS.ofList [])
      (become (target isCreature) { characteristics := { types := [.artifact] } } none) = [] := by
  decide

/-- "Target creature becomes in addition to its other types." -/
theorem badBecomesNothing :
    Action.check (DRS.ofList []) (become (target isCreature) { characteristics := {} } none)
      = [.lineNonEmpty] := by
  decide

/-- The Idris pin refutes the anaphor. -/
theorem badOtherwiseReadsIfArm :
    Action.check (DRS.ofList [])
      (doOnlyIf (create (.lit 1) (.written (creatureToken 1 1 [.black] [zombie])))
        (exists_ (and [isCreature, youControl])) (some (.setStatus .tapped it))) = [.anaphor .bare .one 0] := by
  decide

/-- "Create a 1/1 black Zombie creature token. Create two of those tokens." -/
theorem okAnaphoricTokenAfterToken :
    Action.check (DRS.ofList [])
      (.sequentially
        [ create (.lit 1) (.written (creatureToken 1 1 [.black] [zombie])),
          create (.lit 2) .asThose [] ]) = [] := by
  decide

/-- "Destroy target creature. Create two of those tokens." -/
theorem badAnaphoricTokenAfterNonToken :
    Action.check (DRS.ofList [])
      (.sequentially [destroy (target isCreature), create (.lit 2) .asThose []])
      = [.tokenSpecInScope 0] := by
  decide

theorem badAnaphoricTokenAfterTwoDistinctSpecs :
    Action.check (DRS.ofList [])
      (.sequentially
        [create (.lit 1) (.written (creatureToken 1 1 [.green] [plant])),
          create (.lit 1) (.written (creatureToken 1 1 [.white] [soldier])),
          create (.lit 1) .asThose []]) = [.tokenSpecInScope 2] := by decide

/-- "Put a +1/+1 counter on target creature card in your graveyard." No printed card on the
bench. -/
theorem counterOnGraveyardCard :
    Action.check (DRS.ofList [])
      (.putCounters (.lit 1) p11 (target (.and [isCreature, .in_ (graveyardOf .you)])) .one true) = [] := by
  decide

/-- "put a counter of each kind that's on this creature on target creature" -/
theorem okSameKindsOnCreature :
    Action.check (DRS.ofList []) (.putCounters (.lit 1) (.sameAs thisCreature) (target isCreature) .one true)
      = [] := by
  decide

/-- "a permanent your team controls" -/
def teamPermanent : NounPhrase :=
  a (.and [isPermanent, .hasPossessor .controller (.or [.you, .described (.a .unmarked) (.isTeammateOf .you)])])

/-- "If one or more counters would be put on a permanent your team controls, that many plus
one of each of those kinds are put on that permanent instead." The replaced event names no
agent. -/
theorem okAgentlessCounterReplacement :
    StaticSpec.check (DRS.ofList [])
      (.replace (.would .any (.putCounters .anyAmount .anyKind teamPermanent .many false))
        (instead (.putCounters (plus .thatMuch (.lit 1)) .those (that .permanent) .one true))
        none .repeatedly none)
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

private def exileCost : Cost :=
  .compound [.tapSymbol, .perform (exile (a (.and [.not isLand, .in_ (handOf .you)])))]

private def countersOnTheExiledCard : Action :=
  .sequentially
    [ (.putCounters (.lit 4) (.printed (.named "timeCounter"))
        (the (and [was (exile which), isCard])) .one true),
      (.removeCounters (some (exactly 1)) (some (.printed (.named "timeCounter"))) (each (.and
          [ .other, .hasPossessor .owner .you, .in_ exileZone ])) .one true) ]

/-- "{T}: Exile a card from your hand and put four time counters on it. Then remove a time
counter from each other card you own in exile." The bare "other" is anchored on the card this
same ability exiled and granted to, its nearest earlier mention (`semantics-v3.md` §0.1). -/
theorem okOtherAnchorsOnTheCardThisAbilityExiled :
    let afterCost := (stepCost 64 (DRS.ofList []) exileCost).1
    let after := (stepAction 64 .postEvent .obj (nextClause (publicOnly afterCost))
      countersOnTheExiledCard).1
    Ability.check (DRS.ofList []) (activated exileCost countersOnTheExiledCard) = [] ∧
      after.contrastAnchors.length = 1 ∧
      (after.contrastAnchors.head?.bind fun
        | .mention id => after.findById? id
        | _ => none).map (·.zone) = some (some .exile) := by
  decide

/-- The same last sentence with a bare "other": with nothing mentioned earlier, it contrasts with
the Source (`semantics-v3.md` §0.1). -/
theorem okBareOtherAnchorsOnTheSource :
    let effect := Action.removeCounters (some (exactly 1)) (some (.printed (.named "timeCounter")))
      (each (.and [.other, .hasPossessor .owner .you, .in_ exileZone])) .one true
    Ability.check (DRS.ofList []) (activated .tapSymbol effect) = [] ∧
      (stepAction 64 .postEvent .obj (DRS.ofList []) effect).1.contrastAnchors = [.source] := by
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
        (countOf (.and [.hasSupertype .snow, isLand,
          .hasPossessor .controller (controllerOf (.attachHost .enchanted (.type .creature)))]))
        (minus (toughnessOf (.attachHost .enchanted (.type .creature))) (.lit 1)))
      = [] := by
  decide

/-- The −1/−1 counter's declared name is its kind, everywhere, its own definition included
(ruling 2026-10-10): the name expands to the kind reference, never to the definition. -/
theorem m1m1CounterNameIsItsKind :
    Semantics.Definitions.m1m1Counter = CounterKind.named "m1m1Counter" := rfl

/-- "whichever is greater" is the same aggregate under `max`. -/
theorem okGreaterOf :
    greaterOf (.lit 1) (.lit 2) = .aggregateOf .max [.lit 1, .lit 2] := rfl

/-! ### Anaphora across `putCounters`

A pronoun after "put … counters on <target>" reads the ordinary DRS: the counters introduce no
competing referent, so the printed card needs no card-private first/later mention. -/

private def printedText (card : Spelled) : List Ability :=
  match card.card with
  | .singleFaced face => face.characteristics.text
  | _ => []

/-- A spell ability's body, or an (ability-word headed) triggered ability's header and body, run
from the empty DRS. -/
private def lineTrace (ability : Ability) : DRS × List Refusal :=
  let ability := match ability with
    | .italicHead _ inner => inner
    | ability => ability
  match ability with
  | .spell none body => stepAction 64 .postEvent .obj (DRS.ofList []) body
  | .triggered event alternatives while_ [] none none none body =>
    let (header, headerErrors) :=
      stepTriggerHeaderWith (stepOps 64) (DRS.ofList []) event alternatives while_ none
    let (out, bodyErrors) := stepAction 64 .postEvent .obj (nextClause header) body
    (out, headerErrors ++ bodyErrors)
  | _ => (DRS.ofList [], [.recursionLimit])

/-- Thranduil's Company: "Whenever a land you control enters, put two +1/+1 counters on target
creature you control. It gains vigilance until end of turn." The header reads the land (3); "it"
reads the target creature (5), not the land. -/
theorem thranduilsCompanyItIsTheTargetCreature :
    let result := match (printedText Cards.thranduilsCompany)[1]? with
      | some line => lineTrace line
      | none => (DRS.ofList [], [.nonEmpty])
    result.2 = [] ∧ result.1.resolvedIds = [5, 3] ∧
      (result.1.findById? 5).map (fun r => (r.det, r.ty)) = some (.target, [.creature]) ∧
      (result.1.findById? 3).map (fun r => (r.det, r.ty)) = some (.a, [.land]) := by
  decide

/-- Stunning Shot: "Put two +1/+1 counters on up to one target creature you control. Tap up to
one target creature an opponent controls and put a stun counter on it." "it" reads the second
target (4, the tapped creature), not the first (1). -/
theorem stunningShotItIsTheTappedCreature :
    let result := match (printedText Cards.stunningShot)[0]? with
      | some line => lineTrace line
      | none => (DRS.ofList [], [.nonEmpty])
    result.2 = [] ∧ result.1.resolvedIds = [4] ∧
      (result.1.findById? 4).map (fun r => (r.det, r.clause)) = some (.target, 1) ∧
      (result.1.findById? 1).map (fun r => (r.det, r.clause)) = some (.target, 0) := by
  decide

end Semantics.Proofs.Counters
