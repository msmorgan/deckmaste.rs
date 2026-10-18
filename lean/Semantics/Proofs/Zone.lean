import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Zone

Port of `idris/src/Experimental/Proofs/Zone.idr`: the pins of the Zone family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Zone

/-- "Destroy target tapped creature." -/
theorem okTappedBattlefield :
    Action.check (DRS.ofList []) (destroy (target (.and [isCreature, isTapped]))) = [] := by decide

/-- "Destroy target tapped untapped creature." -/
theorem badTappedUntapped :
    Action.check (DRS.ofList []) (destroy (target (.and [isCreature, isTapped, isUntapped])))
      = [.contradictionFree] := by
  decide

/-- "Destroy target permanent instant." -/
theorem badPermanentInstant :
    Action.check (DRS.ofList []) (destroy (target (.and [isPermanent, .hasType .instant])))
      = [.deedFits, .contradictionFree] := by
  decide

/-- "Destroy target card token" contradicts the distinction between cards and tokens.
Its explicit token evidence supplies the battlefield zone. -/
theorem badCardTokenTarget :
    Action.check (DRS.ofList []) (destroy (target (.and [.isToken, .isCard])))
      = [.contradictionFree] := by
  decide

/-- "Tap target creature. Untap that token." -/
theorem badThatTokenOfCard :
    Action.check (DRS.ofList [])
      (.sequentially [.setStatus .tapped (target isCreature), .setStatus .untapped (that .token)])
      = [.anaphor (.word .token) .one 0] := by
  decide

/-- The distributive loop closes its local creature referents on exit. -/
theorem loopLocalMovedCardsDoNotEscape :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .doForEach (each (.isOpponentOf .you))
            (exile (a (.and [isCreature, .hasPossessor .controller (that .player)]))),
          putOntoBattlefield them ]) = [.anaphor .bare .many 0] := by decide

/-- The loop retains the moved outer card, whose exile zone forbids untapping. -/
theorem loopMovementPreservesUntapZoneRestriction :
    Action.check (DRS.ofList [])
      (.sequentially [tap (target isCreature), .doForEach (each (.isOpponentOf .you)) (exile it), untap it])
      = [.deedFits] := by
  decide

/-- "For each color among permanents on the battlefield, draw a card." -/
theorem okKindLoopKeepsOuter :
    Action.check (DRS.ofList [])
      (.doForEachKind .color (some (allOf isPermanent)) .color (draw (.lit 1)))
      = [] := by
  decide

/-- The color loop retains the moved outer card, whose exile zone forbids untapping. -/
theorem kindLoopMovementPreservesUntapZoneRestriction :
    Action.check (DRS.ofList [])
      (.sequentially
        [ tap (target isCreature),
          .doForEachKind .color (some (allOf isPermanent)) .color (exile it), untap it ])
      = [.deedFits] := by
  decide

/-- "Each opponent exiles a creature they control. Put those cards onto the battlefield." The
distributive twin of `okLoopMovesItsOwn`: the deed moves only what the agent phrase
introduced, so the enclosing stack survives and the exiled cards read back after the deed as
a plural. -/
theorem okDistributedMovesItsOwn :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (each (.isOpponentOf .you)) (exile (a (.and [isCreature, .hasPossessor .controller (that .player)]))),
          putOntoBattlefield them ]) = [] := by
  decide

/-- Moving an outer card during a player distribution preserves its singular identity and exile zone. -/
theorem distributedMovementPreservesUntapZoneRestriction :
    Action.check (DRS.ofList [])
      (.sequentially [tap (target isCreature), instruct (each (.isOpponentOf .you)) (exile it), untap it])
      = [.deedFits] := by
  decide

/-- "When target creature dies this turn, return that card to the battlefield." -/
theorem okDiesBattlefield :
    Action.check (DRS.ofList [])
      (.delay (does (target isCreature) die) [] (some thisTurn) (.move (that .card) .anyLocation (some battlefield) .arrival none []))
      = [] := by
  decide

/-- "not of a color other than white" -/
theorem okNegatedRestrictedQuality :
    Predicate.checkAt (.quality .color) (DRS.ofList [])
      (.not (.qualityNoun .color (some (.colorOtherThan .white))))
      = [] := by
  decide

/-- "noncolor" -/
theorem badNegatedQualityHead :
    Predicate.checkAt (.quality .color) (DRS.ofList []) (.not (.qualityNoun .color none)) = [.negatable] := by
  decide

/-- "non-player" -/
theorem badNegatedPlayerHead : Predicate.checkAt .player (DRS.ofList []) (.not .anyPlayer) = [.negatable] := by
  decide

/-- "Tap target creature an opponent controls. That player loses 1 life." -/
theorem okPositiveAntecedent :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .setStatus .tapped (target (.and [isCreature, .hasPossessor .controller anOpponent])),
          instruct (that .player) (loseLife (.lit 1)) ]) = [] := by
  decide

/-- "Tap target creature an opponent doesn't control. That player loses 1 life." -/
theorem badNegatedAntecedent :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .setStatus .tapped
            (target (.and [isCreature, .not (.hasPossessor .controller anOpponent)])),
          instruct (that .player) (loseLife (.lit 1)) ]) = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Destroy target creature on the battlefield." -/
theorem okSingleZoneOnTarget :
    Action.check (DRS.ofList []) (destroy (target (.and [isCreature, .in_ battlefield]))) = [] := by decide

/-- "Return target creature to its owner's hand." -/
theorem okMoveToHand : Action.check (DRS.ofList []) (.move (target isCreature) .anyLocation (some hand) .arrival none []) = [] := by decide

/-- "Destroy target creature." -/
theorem okDestroyBattlefield : Action.check (DRS.ofList []) (destroy (target isCreature)) = [] := by decide

/-- "You discard a card." -/
theorem okDiscardHand : Action.check (DRS.ofList []) (discard (a (.and [.isCard, .in_ yourHand]))) = [] := by
    decide

/-- "Destroy a creature." -/
theorem okKnownKeywordAction :
    Action.check (DRS.ofList []) (.enact (.action "destroy") (.move (a isCreature) ((.zone .battlefield none)) (some graveyard) .arrival none []) none none)
      = [] := by
  decide

/-- A keyword action the registry does not declare -/
theorem badUnknownKeywordAction :
    Action.check (DRS.ofList []) (.enact (.action "descry") (.move (a isCreature) .anyLocation (some graveyard) .arrival none []) none none)
      = [.knownAct (.action "descry")] := by
  decide

/-- "You exile a card from your hand." -/
theorem okAgentedKnownAct :
    Action.check (DRS.ofList [])
      (.enact (.action "exile") (.move (a (.and [.isCard, .in_ hand])) .anyLocation (some exileZone) .arrival none []) none none)
      = [] := by
  decide

/-- "You destroy target creature.": printed cards give Destroy a player subject ("You destroy
four lands", Burning of Xinye), so its facts row's agent role names a player. -/
theorem okAgentedDestroy :
    Action.check (DRS.ofList [])
      (.enact (.action "destroy") (.move (target isCreature) ((.zone .battlefield none)) (some graveyard) .arrival none []) none none)
      = [] := by
  decide

/-- "Target opponent fights target creature." A spell or ability instructs a CREATURE to fight
[CR#701.14a], so `"Fight"`'s facts row gives its agent role a permanent and no player, and a
player handed the deed is refused; the printed sentence hands it the first fighting creature,
`instruct(<first>, fight(<second>))`. -/
theorem badAgentedAgentlessAct :
    Action.check (DRS.ofList [])
      (instruct (target (.isOpponentOf .you)) (.enact (.action "fight") (.simultaneously [
        instruct theActor (.dealDamage (.statOf (.stat .power) theActor) (.firstMention "fight.0" (target isCreature)) .any),
        instruct (.laterMention "fight.0" (target isCreature))
          (.dealDamage (.statOf (.stat .power) (.laterMention "fight.0" (target isCreature))) theActor .any)]) none none))
      = [.kindMismatch .object .player, .kindMismatch .object .player] := by
  decide

/-- "1 life for each creature" -/
theorem okForEachOne : Amount.check (DRS.ofList []) (forEach (.lit 1) isCreature) = [] := by decide

/-- "0 life for each creature" does not supply a nonzero per-member multiplier. -/
theorem badForEachZero :
    Amount.check (DRS.ofList []) (forEach (.lit 0) isCreature) = [.amtNonZero] := by decide

/-- "creature on the battlefield" -/
theorem okConsistentZoneConjunct :
    Predicate.checkAt .object (DRS.ofList []) (.and [isCreature, .in_ battlefield]) = [] := by decide

/-- "creature of the chosen color" -/
theorem okConsistentQualityConjunct :
    Predicate.checkAt .object [({ det := .a, plur := .one, payload := .quality .color, idents := [] } : Referent)]
      (.and [isCreature, ofChosen .color]) = [] := by
  decide

/-- "of the chosen color and not of the chosen color" -/
theorem okQualitySelfNegation :
    Predicate.checkAt .object [({ det := .a, plur := .one, payload := .quality .color, idents := [] } : Referent)]
      (.and [ofChosen .color, .not (ofChosen .color)]) = [] := by
  decide

/-- "creature that is a noncreature" -/
theorem okNestedSelfNegation :
    Predicate.checkAt .object (DRS.ofList []) (.and [isCreature, .and [.not isCreature]]) = [] := by decide

/-- "Discard a card." -/
theorem okDiscardACardFromHand :
    Action.check (DRS.ofList []) (discard (a (.and [.isCard, .in_ yourHand]))) = [] := by decide

/-- "another creature" -/
theorem okOtherAnchored :
    Predicate.checkAt .object
      [({ det := .target, plur := .one, payload := .object [.creature] (some .battlefield) none none none, idents := [] } : Referent)]
      (.and [isCreature, .other]) = [] := by
  decide

/-- "another other creature" -/
theorem badDoubleOther :
    Predicate.checkAt .object
      [({ det := .target, plur := .one, payload := .object [.creature] (some .battlefield) none none none, idents := [] } : Referent)]
      (.and [isCreature, .other, .other]) = [.otherAnchored] := by
  decide

/-- "You discard a card." -/
theorem okDiscardHandCard :
    Action.check [({ det := .a, plur := .one, payload := .object [] none none none none, idents := [] } : Referent)]
      (discard (a (.and [.isCard, .in_ yourHand]))) = [] := by
  decide

/-- "creature card in a graveyard" -/
theorem okZoneCoherentAnd :
    Predicate.checkAt .object (DRS.ofList []) (.and [isCreature, .in_ graveyard]) = [] := by decide

/-- "artifact or enchantment" -/
theorem okFlatDisjunction : Predicate.checkAt .object (DRS.ofList []) (.or [isArtifact, isEnchantment]) = [] := by
  decide

/-- a coordination of no alternatives -/
theorem badEmptyOr : Predicate.checkAt .object (DRS.ofList []) (.or []) = [.nonEmpty] := by decide

/-- "artifact or artifact" -/
theorem okRepeatedDisjunct : Predicate.checkAt .object (DRS.ofList []) (.or [isArtifact, isArtifact]) = [] := by decide

/-- "artifact or attacking": accepted since `parallelDisjuncts` retired (ruling 2026-10-09). -/
theorem okHeadlessDisjunct :
    Predicate.checkAt .object (DRS.ofList []) (.or [isArtifact, isAttacking]) = [] := by decide

/-- "in your hand or in your graveyard" -/
theorem okCrossZoneDisjunction :
    Predicate.checkAt .object (DRS.ofList []) (.or [.in_ hand, .in_ graveyard]) = [] := by
  decide

/-- "spell or permanent": both are objects, and the join is a legal subject (owner ruling
2026-10-09, retiring `parallelDisjuncts`). -/
theorem okSpellOrPermanentSubject :
    Predicate.checkAt .object (DRS.ofList []) (.or [isSpell, isPermanent]) = [] := by decide

/-- "Destroy target creature if it's on the battlefield." -/
theorem okMatchesZoneFits :
    Action.check (DRS.ofList [])
      (.doOnlyIf (destroy (target isCreature)) (.matches it (.in_ battlefield)) none) = [] := by
  decide

/-- "Choose a card in your hand. You discard that card." -/
theorem okChosenCardRemention :
    Action.check (DRS.ofList []) (.sequentially [choose (a (.and [.isCard, .in_ yourHand])), discard (that .card)])
      = [] := by
  decide

/-- "Draw a card. Exile that card." -/
theorem drawnCardCanBeExiled :
    Action.check (DRS.ofList []) (.sequentially [draw (.lit 1), exile (that .card)])
      = [] := by
  decide

theorem drawnCardDemonstrativeSelectsTheHandCard :
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) (draw (.lit 1))).1
    (firstReach (.word .card) .one (nextClause after)).map (fun ref => (ref.id, ref.zone)) =
      some (0, some .hand) := by decide

theorem twoDrawnCardsPublishAPluralReferent :
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) (draw (.lit 2))).1
    (firstReach (.word .card) .many (nextClause after)).map (fun ref => (ref.id, ref.zone)) =
      some (0, some .hand) ∧ firstReach (.word .card) .one (nextClause after) = none := by decide

/-- "Draw a card and you gain 1 life." -/
theorem okNonEmptyBatch :
    Action.check (DRS.ofList []) (.simultaneously [draw (.lit 1), gainLife (.lit 1)]) = [] := by
  decide

/-- an empty batch -/
theorem badEmptySimultaneous : Action.check (DRS.ofList []) (.simultaneously []) = [.nonEmpty] := by
    decide

/-- "Deal 2 damage to target creature. Then tap it." -/
theorem okTapAfterSimultaneousDamage :
    Action.check (DRS.ofList [])
      (.sequentially [.simultaneously [(Action.instruct .this (.dealDamage (.lit 2) (target isCreature) .any))], tap it])
      = [] := by
  decide

/-- "Gain control of target creature." -/
theorem okGainControlBattlefield :
    Action.check (DRS.ofList []) (gainControl (target isCreature) none (controller := .you)) = [] := by decide

/-- "Reveal the top four cards. Put them on the bottom in any order." -/
theorem okPluralOrderRider :
    Action.check (DRS.ofList [])
      (.sequentially
        [reveal (topSlice (.lit 4)), (.move (those .card) .anyLocation (some onBottom) .arrival none [(.inOrder .anyOrder)])]) = [] := by
  decide

/-- "Look at the top four cards of your library. Put them into your hand." -/
theorem okSliceCardRead :
    Action.check (DRS.ofList []) (.sequentially [lookAt (topSlice (.lit 4)), (.move (those .card) .anyLocation (some hand) .arrival none [])])
      = [] := by
  decide

theorem badSliceTypeRead :
    Action.check (DRS.ofList [])
      (.sequentially [lookAt (topSlice (.lit 4)), (.move (those (.type .creature)) .anyLocation (some hand) .arrival none [])])
      = [.anaphor (.word (.type .creature)) .many 0] := by
  decide

/-- "Search your library for a creature card." -/
theorem okSearchZoneFreeDescription :
    Action.check (DRS.ofList []) (searchLibraryFor (exactly 1) isCreature) = [] := by decide

/-- "If you would search your library for a creature card, instead search your library for a
creature card and reveal that card." [CR#614.1a] -/
theorem okInsteadOfSearchRevealsIt :
    Action.check (DRS.ofList []) (.replace (searchLibraryFor (exactly 1) isCreature)
      (.sequentially [searchLibraryFor (exactly 1) isCreature,
        reveal (theVerbed (.action "search") .one)])) = [] := by
  decide

/-- "Each player mills a card. Exile it." -/
theorem badDistributedMillSingular :
    Action.check (DRS.ofList [])
      (.sequentially [instruct (each .anyPlayer) (mill (.lit 1) (each .anyPlayer)), exile it])
      = [.anaphor .bare .one 0] := by
  decide

/-- "one of up to two target creatures" -/
theorem okPartitiveOfTargetGroup :
    Action.check (DRS.ofList [])
      (exile (.someOf (.counted (exactly 1)) none (.described (.target (upTo 2)) isCreature)))
        = [] := by
  decide

/-- "one of a creature you control" -/
theorem badPartitiveOfDescription :
    Action.check (DRS.ofList []) (exile (.someOf (.counted (exactly 1)) none (a isCreature)))
      = [.partitiveBase] := by
  decide

/-- "Look at the top four cards of your library. Exile one of them." -/
theorem okPartitiveOfThem :
    Action.check (DRS.ofList [])
      (.sequentially [lookAt (topSlice (.lit 4)), exile (someOf (exactly 1) them)]) = [] := by
  decide

/-- "two of one of them" -/
theorem badPartitiveOfPartitive :
    Action.check (DRS.ofList [])
      (.sequentially
        [lookAt (topSlice (.lit 4)), exile (someOf (exactly 1) (someOf (exactly 1) them))])
      = [.partitiveBase] := by
  decide

/-- "each of the rest" -/
theorem badEachOfTheRest :
    Action.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice (.lit 4)), (.move (someOf (exactly 1) them) .anyLocation (some hand) .arrival none []),
          exile (.eachOf (theRest .object)) ]) = [.groupMention] := by
  decide

/-- "If a creature you control would die this turn, exile it instead." -/
theorem okWouldDieOnBattlefield :
    Action.check (DRS.ofList [])
      (replaceEvent (.would .any (.move (target (and [isCreature, youControl])) (battlefield) (some graveyard) .departure none [])) (exile it) (some thisTurn)) = [] := by
  decide

/-- "Reveal the top card of your library. Put that card into your hand." -/
theorem okSingularCardRetag :
    Action.check (DRS.ofList [])
      (.sequentially [reveal (topSlice (.lit 1)), (.move (that .card) .anyLocation (some hand) .arrival none [])]) = [] := by
  decide

/-- "Exile target creature until this creature leaves the battlefield. Put that card into
its owner's hand." The held clause announces the exiled object where it now is
[CR#701.13a], so the card read resolves. -/
theorem okHeldUntilExileRetag :
    Action.check (DRS.ofList [])
      (.sequentially
        [ exileUntil (target isCreature) (does thisCreature leave),
          (.move (that .card) .anyLocation (some hand) .arrival none []) ]) = [] := by
  decide

/-- "Put target creature onto the battlefield tapped." -/
theorem okMoveRidersToBattlefield :
    Action.check (DRS.ofList []) (.move (target isCreature) .anyLocation (some battlefield) .arrival none [(.entersAs .tapped)]) = [] := by decide

/-- "Put target creature onto the battlefield under your control." -/
theorem okMoveRidersSingularController :
    Action.check (DRS.ofList []) (.move (target isCreature) .anyLocation (some battlefield) .arrival none [(.under .you)]) = [] := by decide

/-- "a card exiled with this permanent" -/
theorem okExiledWithThis : Predicate.checkAt .object (DRS.ofList []) (.in_ (.exiledWith .this)) = [] := by decide

/-- "cards exiled with target creature" -/
theorem badExiledWithOtherSource :
    Predicate.checkAt .object (DRS.ofList []) (.in_ (.exiledWith (target isCreature))) = [.linkSource] := by decide

/-- "a card exiled with an artifact" -/
theorem badExiledWithDescribedSource :
    Predicate.checkAt .object (DRS.ofList []) (.in_ (.exiledWith (a isArtifact))) = [.linkSource] := by decide

/-- "a creature you control" -/
theorem okZoneCoherentConjunction :
    Predicate.checkAt .object (DRS.ofList []) (.and [isCreature, .hasPossessor .controller .you]) = [] := by decide

/-- "an attacking creature" -/
theorem okAttackingCreature : Predicate.checkAt .object (DRS.ofList []) (.and [isAttacking, isCreature]) = [] := by
  decide

/-- "Put target creature card from a graveyard onto the battlefield tapped." -/
theorem okCreatureOntoBattlefieldTapped :
    Action.check (DRS.ofList [])
      (putOntoBattlefieldTapped (target (.and [isCreature, .in_ graveyard]))) = [] := by
  decide

/-- "Remove target creature from combat." -/
theorem okRemoveFromCombatOnBattlefield :
    Action.check (DRS.ofList []) (Primitives.Action.removeFromCombat (target isCreature)) = [] := by decide

/-- "Players can't untap more than one creature during each untap step." -/
theorem okUntapCapBattlefieldSet :
    Ability.check (DRS.ofList [])
      (.static (cant (actor (.described .all .anyPlayer)) (untap (counted (moreThan 1) isCreature))))
      = [] := by
  decide

/-- "if this creature is attacking" -/
theorem okMatchesBattlefieldZone :
    Ability.check (DRS.ofList [])
      (.triggered (lastCounterRemoved (.named "timeCounter") thisCreature) [] none [] none none
        (some (.matches thisCreature isAttacking)) (draw (.lit 1))) = [] := by
  decide

/-- "Draw cards equal to the difference." under a comparison -/
theorem okDifferenceAfterComparison :
    Action.check (DRS.ofList [])
      (.doIf (.compareAmt (countOf (.and [.isCard, .in_ (handOf .you)])) .less (.lit 7))
        (draw .theDifference) none) = [] := by
  decide

/-- "Draw cards equal to the difference." -/
theorem badUnlicensedDifference :
    Action.check (DRS.ofList []) (draw .theDifference) = [.differenceInScope 0] := by decide

/-- "When this creature enters, if you control fewer than seven creatures, draw cards equal
to the difference." -/
theorem okDifferenceUnderComparisonTrigger :
    Ability.check (DRS.ofList [])
      (.triggered (does thisCreature enter) [] none [] none none
        (some (.compareAmt (countOf (and [isCreature, youControl])) .less (.lit 7)))
        (draw .theDifference)) = [] := by
  decide

theorem badNonComparisonDifference :
    Ability.check (DRS.ofList [])
      (.triggered (does thisCreature enter) [] none [] none none
        (some (exists_ (and [isCreature, youControl]))) (draw .theDifference)) = [.differenceInScope 0]
            := by
  decide

/-- "Goad target creature." -/
theorem okGoadOnBattlefield :
    Action.check (DRS.ofList []) (.gainDesignation (target isCreature) (.named "goaded") none) = [] := by
  decide

/-- "goad target creature card in your graveyard" -/
theorem badGoadInGraveyard :
    Action.check (DRS.ofList [])
      (.gainDesignation (target (.and [isCreature, .in_ (graveyardOf .you)])) (.named "goaded")
        none) = [.designationHolder (.named "goaded") .object] := by
  decide

/-- "Whenever a creature attacks a planeswalker, …" -/
theorem okPlaneswalkerAttackDefender :
    GameEvent.check (DRS.ofList []) (.does (.actor (a isCreature)) (.attack (some (a (.hasType .planeswalker)))))
      = [] := by
  decide

/-- "Look at the top four cards of your library. An opponent chooses one of them." -/
theorem okAgentChoosesSomeOf :
    Action.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice (.lit 4)),
          instruct (a (.isOpponentOf .you)) (.choose none (someOf (exactly 1) them) .openly none) ]) =
              [] := by
  decide

theorem badAgentChooseTheRest :
    Action.check (DRS.ofList [])
      (.sequentially
        [ lookAt (topSlice (.lit 4)), (.move (someOf (exactly 1) them) .anyLocation (some hand) .arrival none []),
          instruct (a (.isOpponentOf .you)) (.choose none (theRest .object) .openly none) ])
            = [.choiceClause] := by
  decide

/-- "… two cards with the same name in your hand …" -/
theorem okPluralNameAgreement :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.agree .cardName .share (counted (exactly 2) (.and [.not isLand, .in_ hand]))) = [] := by
  decide

/-- "… target creature with different names." -/
theorem badSingularNameAgreement :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.agree .cardName .differ (target isCreature))
      = [.plural] := by
  decide

/-- "This spell can't be countered." -/
theorem okCantBeCountered :
    StaticSpec.check (DRS.ofList []) (cant any (counter thisSpell)) = [] := by decide

/-- "Creature cards in graveyards can't be countered." -/
theorem badCounteredInGraveyard :
    StaticSpec.check (DRS.ofList [])
      (cant any (counter (allOf (.and [isCreature, .in_ graveyard]))))
      = [.deedFits] := by
  decide

/-- "You may look at the top card of your library any time." -/
theorem okLookAtTopOfLibrary :
    StaticSpec.check (DRS.ofList []) (.visibility .lookAt .you .topOfLibrary) = [] := by decide

/-- "Regenerate target creature." -/
theorem okRegenerateCreature :
    Action.check (DRS.ofList []) (regenerate (target isCreature))
      = [] := by decide

/-- "Creatures can't be regenerated." -/
theorem okRegenerationBanOnBattlefield :
    StaticSpec.check (DRS.ofList []) (cant any (regeneration (allOf isCreature))) = [] := by decide

/-- "Creature cards in your graveyard can't be regenerated." -/
theorem badRegeneratedInGraveyard :
    StaticSpec.check (DRS.ofList []) (cant any (regeneration (a (.and [isCreature, .in_ graveyard]))))
      = [.deedFits] := by
  decide

/-- "Each opponent discards a card. Simultaneously, exile those cards." -/
theorem simultaneousDiscardDoesNotExposePluralClauseMate :
    Action.check (DRS.ofList [])
      (.simultaneously [.sequentially [instruct (each (.isOpponentOf .you)) (choose (a (.and [.isCard, .in_ hand]))), discard them], exile (those
          .card)])
        = [.anaphor (.word .card) .many 0] := by
  decide

/-- "Discard a card. Simultaneously, exile that card." -/
theorem simultaneousDiscardDoesNotExposeSingularClauseMate :
    Action.check (DRS.ofList []) (.simultaneously [discard (a (.and [.isCard, .in_ yourHand])), exile
        (that .card)])
      = [.anaphor (.word .card) .one 0] := by
  decide

/-- "Each opponent discards a card. Simultaneously, exile that card." -/
theorem badDistributedAnnouncedDiscardSingular :
    Action.check (DRS.ofList [])
      (.simultaneously [.sequentially [instruct (each (.isOpponentOf .you)) (choose (a (.and [.isCard, .in_ hand]))), discard them], exile (that
          .card)])
      = [.anaphor (.word .card) .one 0] := by
  decide

/-- "Draw a card. You gain 1 life." -/
theorem okNonEmptySequence :
    Action.check (DRS.ofList []) (.sequentially [draw (.lit 1), gainLife (.lit 1)]) = [] := by
  decide

/-- an empty sentence list -/
theorem badEmptySequence : Action.check (DRS.ofList []) (.sequentially []) = [.nonEmpty] := by decide

/-- "Reveal your hand." -/
theorem okRevealHand : Action.check (DRS.ofList []) (.expose .reveal (.zone hand)) = [] :=
    by decide

/-- "Sacrifice a creature." -/
theorem okSacrificeBattlefieldNoun :
    Action.check (DRS.ofList []) (sacrifice (a isCreature)) = [] := by decide

theorem badInterceptReplacementAntecedent :
    Action.check (DRS.ofList [])
      (.sequentially
        [ replaceNextEvent (.would (.actor .you) (.draw .anyAmount))
            (create (.lit 1) (.written (creatureToken 1 1 [.green] [creatureType "Soldier"])))
            (some thisTurn),
          sacrifice it ]) = [.anaphor .bare .one 0] := by
  decide

/-- "If you would draw a card, draw two cards instead." -/
theorem okFlatInstead :
    Action.check (DRS.ofList []) (.replace (draw (.lit 1)) (draw (.lit 2)))
        = [] := by decide

/-- "Untap target creature." -/
theorem okUntapInstruction :
    Action.check (DRS.ofList []) (.setStatus .untapped (target isCreature)) = [] := by decide

/-- "… if a creature died this turn, …" -/
theorem okDeathLookback :
    Condition.check (DRS.ofList []) (.did ⟨.any, .move (a isCreature) (battlefield) (some graveyard) .departure none [], .duration soFarThisTurn none⟩) = [] := by decide

/-- "Counter target activated ability." -/
theorem okCounterAbility :
    Action.check (DRS.ofList []) (counter (target (.abilityHead .anyActivated))) = [] := by decide

/-- "Exile target creature." -/
theorem okObjectMovedToAZone :
    Action.check (DRS.ofList []) (.move (target isCreature) .anyLocation (some exileZone) .arrival none []) = [] := by decide

/-- Abilities can be exiled with other objects on the stack [CR#724.1b,724.2b].
The former `badAbilityMovedToAZone` pin asserted the explicitly retired blanket prohibition. -/
theorem okAbilityMovedToAZone :
    Action.check (DRS.ofList []) (.move (target (.abilityHead .anyActivated)) .anyLocation (some exileZone) .arrival none [])
      = [] := by
  decide

/-- "Exchange control of target artifact and target creature." -/
theorem okExchangeControlOnField :
    Action.check (DRS.ofList []) (.exchange (.controlOf (target isArtifact) (target isCreature)))
      = [] := by decide

/-- "Exchange this Aura with an Aura card in your hand." (aura swap, [CR#702.65a]) -/
theorem okExchangeCardsAcrossZones :
    Action.check (DRS.ofList [])
      (.exchange
        (.cardsAcross thisAura (a (.and [.hasSubtype (enchantmentType "Aura"), .in_ hand]))))
      = [] := by
  decide

/-- "Exchange your hand and graveyard." -/
theorem okExchangeTwoZones :
    Action.check (DRS.ofList []) (.exchange (.zones hand graveyard))
      = [] := by decide

/-- "the exiled card": the definite reference is licensed by the link to the exiling ability
printed on the same object [CR#607.2a], and still resolves when that ability exiled more than
one card [CR#607.3]. -/
theorem okTheExiledCard : NounPhrase.checkAt (some .object) (DRS.ofList []) (the (.in_ (.exiledWith .this))) = [] := by
  decide

/-- "the card in exile": the exile zone holds any number of cards, and nothing links the
reference to one of them. -/
theorem badTheCardInExile :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (the (.in_ exileZone)) = [.uniquifying] := by decide

/-- "Exchange your life total with this creature's toughness." (Tree of Redemption): two
settable values [CR#701.12g]. -/
theorem okExchangeTwoValues :
    Action.check (DRS.ofList []) (.exchange (.values (lifeTotalOf .you) (toughnessOf thisCreature)))
      = [] := by
  decide

/-- "Exchange this creature's power with three.": a literal is not a value the game can set
[CR#701.12g]. -/
theorem badExchangeLiteralValue :
    Action.check (DRS.ofList []) (.exchange (.values (powerOf thisCreature) (.lit 3)))
      = [.settableValue] := by
  decide

/-- "Exchange the text boxes of this creature and another creature." [CR#701.12h]: a Text
Change [CR#612.5] that lasts indefinitely because it states no duration [CR#611.2a]. -/
theorem okExchangeTextBoxes :
    Action.check (DRS.ofList [])
      (.establish (.textChange thisCreature (.exchangeWith (a (.and [isCreature, .otherThan .this]))))
        none) = [] := by
  decide

end Semantics.Proofs.Zone
