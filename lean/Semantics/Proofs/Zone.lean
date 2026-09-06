import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Zone

Port of `idris/src/Experimental/Proofs/Zone.idr`: the pins of the Zone family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Zone

/-- "Destroy target tapped creature." -/
theorem okTappedBattlefield :
    Instruction.check [] (destroy (target (.and [creature, tapped]))) = [] := by decide

/-- "Destroy target tapped creature card in your graveyard." -/
theorem badTappedGraveyard :
    Instruction.check [] (destroy (target (.and [creature, tapped, .inZone (graveyardOf .you)])))
      = [.zoneCoherent] := by
  decide

/-- "Destroy target tapped untapped creature." -/
theorem badTappedUntapped :
    Instruction.check [] (destroy (target (.and [creature, tapped, untapped])))
      = [.contradictionFree] := by
  decide

/-- "Untap target creature card in your graveyard." -/
theorem badUntapGraveyard :
    Instruction.check []
      (.setStatus .untapped (target (.and [creature, .inZone (graveyardOf .you)])))
      = [.zoneIs .battlefield] := by
  decide

/-- "Destroy target permanent instant." -/
theorem badPermanentInstant :
    Instruction.check [] (destroy (target (.and [permanent, .hasType .instant])))
      = [.contradictionFree] := by
  decide

theorem badThatPermanentDeparted :
    Instruction.check []
      (.sequence [destroy (target permanent), .setStatus .tapped (that .permanent)])
      = [.zoneIs .battlefield] := by
  decide

/-- "Destroy target token card in your graveyard." -/
theorem badTokenGraveyard :
    Instruction.check [] (destroy (target (.and [.isToken, .inZone (graveyardOf .you)])))
      = [.zoneCoherent] := by
  decide

/-- "Destroy target card token." The Idris pin names only the contradiction; its zone
obligation was discharged by the same impossible hypothesis (both are `So False`), which the
list form cannot hide: a placeless head has no battlefield to be destroyed on. -/
theorem badCardTokenTarget :
    Instruction.check [] (destroy (target (.and [.isToken, .isCard])))
      = [.contradictionFree, .zoneFits] := by
  decide

/-- "Tap target creature. Untap that token." -/
theorem badThatTokenOfCard :
    Instruction.check []
      (.sequence [.setStatus .tapped (target creature), .setStatus .untapped (that .token)])
      = [.anaphor (.word .token) .one 0, .zoneIs .battlefield] := by
  decide

/-- "For each opponent, exile a creature that player controls. Put those cards onto the
battlefield." (the printed shape of Breach the Multiverse's middle pair). The loop body moves
only what it introduced itself, so the enclosing stack survives and the exiled cards read back
after the loop as a plural. -/
theorem okLoopMovesItsOwn :
    Instruction.check []
      (.sequence
        [ .doForEach (each .opponent)
            (exile (a (.and [creature, .hasPossessor .controller (that .player)]))),
          putOntoBattlefield them ]) = [] := by
  decide

/-- "Tap target creature. For each opponent, exile it. Untap it." The loop body exiles an
object bound OUTSIDE the loop, so `forEachOf`'s introductions can no longer be told from the
mutated stack and the loop would republish the creature's stale battlefield zone. Refused at
`keepsOuter`; the unlooped spelling of the same sentence is refused one clause later, at
`untap`'s battlefield gate ([CR#701.26b], `badUnloopedZoneMoveRead`). -/
theorem badLoopedZoneMoveRead :
    Instruction.check []
      (.sequence [tap (target creature), .doForEach (each .opponent) (exile it), untap it])
      = [.keepsOuter] := by
  decide

/-- "Tap target creature. Exile it. Untap it." (the same sentence without the loop): exile
puts the creature in the exile zone, where it is a new object and no longer a permanent, so
untapping it is refused [CR#400.7,701.26b]. -/
theorem badUnloopedZoneMoveRead :
    Instruction.check [] (.sequence [tap (target creature), exile it, untap it])
      = [.zoneIs .battlefield] := by
  decide

/-- "For each color among permanents on the battlefield, draw a card." -/
theorem okKindLoopKeepsOuter :
    Instruction.check []
      (.doForEachKind .color (some (allOf permanent)) .color (draw (.lit 1) (agent := .you)))
      = [] := by
  decide

/-- "Tap target creature. For each color among permanents on the battlefield, exile it. Untap
it." `forEachKindOf` republishes its enclosing stack the same way `forEachOf` does, so a body
that moves an object bound outside the loop is refused at the same obligation
[CR#400.7,701.26b]. -/
theorem badKindLoopZoneMoveRead :
    Instruction.check []
      (.sequence
        [ tap (target creature),
          .doForEachKind .color (some (allOf permanent)) .color (exile it), untap it ])
      = [.keepsOuter] := by
  decide

/-- "Each opponent exiles a creature they control. Put those cards onto the battlefield." The
distributive twin of `okLoopMovesItsOwn`: the deed moves only what the agent phrase
introduced, so the enclosing stack survives and the exiled cards read back after the deed as
a plural. -/
theorem okDistributedMovesItsOwn :
    Instruction.check []
      (.sequence
        [ exile (a (.and [creature, .hasPossessor .controller (that .player)])) (agent := some (each
            .opponent)),
          putOntoBattlefield them ]) = [] := by
  decide

/-- "Tap target creature. Each opponent exiles it. Untap it." The distributive deed exiles an
object bound OUTSIDE the agent phrase, so `distributedDelta` would republish the pre-deed
stack, the creature's stale battlefield zone, over the exile. Refused at `enact`'s
distribution obligation; the singular spelling of the same sentence is refused one clause
later, at `untap`'s battlefield gate ([CR#400.7,701.26b], `badUnloopedZoneMoveRead`). -/
theorem badDistributedZoneMoveRead :
    Instruction.check []
      (.sequence [tap (target creature), exile it (agent := some (each .opponent)), untap it])
      = [.enactKeepsOuter] := by
  decide

/-- "When target creature dies this turn, return that card to the battlefield." -/
theorem okDiesBattlefield :
    Instruction.check []
      (.delay (.dies (target creature)) [] (some .thisTurn) (.move (that .card) battlefield []))
      = [] := by
  decide

theorem badDiesInGraveyard :
    Instruction.check []
      (.delay (.dies (target (.and [creature, .inZone (graveyardOf .you)]))) [] (some .thisTurn)
        (.move (that .card) battlefield [])) = [.zoneIs .battlefield] := by
  decide

/-- "not of a color other than white" -/
theorem okNegatedRestrictedQuality :
    Predicate.check (.quality .color) []
      (.not (.qualityNoun .color (some (.colorOtherThan .white))))
      = [] := by
  decide

/-- "noncolor" -/
theorem badNegatedQualityHead :
    Predicate.check (.quality .color) [] (.not (.qualityNoun .color none)) = [.negatable] := by
  decide

/-- "non-player" -/
theorem badNegatedPlayerHead : Predicate.check .player [] (.not .anyPlayer) = [.negatable] := by
  decide

/-- "Tap target creature an opponent controls. That player loses 1 life." -/
theorem okPositiveAntecedent :
    Instruction.check []
      (.sequence
        [ .setStatus .tapped (target (.and [creature, .hasPossessor .controller anOpponent])),
          loseLife (.lit 1) (agent := (that .player)) ]) = [] := by
  decide

/-- "Tap target creature an opponent doesn't control. That player loses 1 life." -/
theorem badNegatedAntecedent :
    Instruction.check []
      (.sequence
        [ .setStatus .tapped
            (target (.and [creature, .not (.hasPossessor .controller anOpponent)])),
          loseLife (.lit 1) (agent := (that .player)) ]) = [.anaphor (.word .player) .one 0] := by
  decide

/-- "Destroy target creature on the battlefield." -/
theorem okSingleZoneOnTarget :
    Instruction.check [] (destroy (target (.and [creature, .inZone battlefield]))) = [] := by decide

/-- "Destroy target creature on the battlefield in a graveyard." -/
theorem badConflictingZones :
    Instruction.check []
      (destroy (target (.and [creature, .inZone battlefield, .inZone graveyard])))
      = [.zoneCoherent] := by
  decide

/-- "Return target creature to its owner's hand." -/
theorem okMoveToHand : Instruction.check [] (.move (target creature) hand []) = [] := by decide

/-- "Put target creature into target player's hand." -/
theorem badMoveToTargetsHand :
    Instruction.check [] (.move (target creature) (handOf (target .anyPlayer)) []) = [.destOk] := by
  decide

/-- "Destroy target creature." -/
theorem okDestroyBattlefield : Instruction.check [] (destroy (target creature)) = [] := by decide

/-- "Destroy target creature card in your graveyard." -/
theorem badDestroyGraveyard :
    Instruction.check [] (destroy (target (.and [creature, .inZone (graveyardOf .you)])))
      = [.zoneFits] := by
  decide

/-- "You discard a card." -/
theorem okDiscardHand : Instruction.check [] (discard (a (.inZone hand)) (agent := .you)) = [] := by
    decide

/-- "You discard a creature." -/
theorem badDiscardBattlefield :
    Instruction.check [] (discard (a creature) (agent := .you)) = [.zoneFits] := by decide

/-- "Destroy target creature card in a graveyard." -/
theorem badDestroyGraveyardCard :
    Instruction.check [] (destroy (target (.and [creature, .inZone graveyard]))) = [.zoneFits] := by
  decide

/-- "Destroy a creature." -/
theorem okKnownKeywordAction :
    Instruction.check [] (.enact (.action "Destroy") (.move (a creature) graveyard []) (agent :=
        none))
      = [] := by
  decide

/-- A keyword action the registry does not declare -/
theorem badUnknownKeywordAction :
    Instruction.check [] (.enact (.action "Descry") (.move (a creature) graveyard []) (agent :=
        none))
      = [.knownAct (.action "Descry")] := by
  decide

/-- "You exile a card from your hand." -/
theorem okAgentedKnownAct :
    Instruction.check []
      (.enact (.action "Exile") (.move (a (.inZone hand)) exileZone []) (agent := (some .you)))
      = [] := by
  decide

/-- "You destroy target creature.": printed cards give Destroy a player subject ("You destroy
four lands", Burning of Xinye), so its facts row's agent role names a player. -/
theorem okAgentedDestroy :
    Instruction.check []
      (.enact (.action "Destroy") (.move (target creature) graveyard []) (agent := (some .you)))
      = [] := by
  decide

/-- "You fight target creature." A spell or ability instructs a CREATURE to fight
[CR#701.14a], so `"Fight"`'s facts row gives its agent role no player and the agented voice
is refused; the printed sentence names the fighting creatures, `Instruction.fights`. -/
theorem badAgentedAgentlessAct :
    Instruction.check []
      (.enact (.action "Fight") (.fight (target creature) (target creature)) (agent := (some .you)))
      = [.enactAgentOk] := by
  decide

/-- "1 life for each creature" -/
theorem okForEachOne : Amount.check [] (forEach 1 creature) = [] := by decide

/-- "1 life for each 0 creatures" -/
theorem badForEachZero : Amount.check [] (forEach 0 creature) = [.amtNonZero] := by decide

/-- "creature on the battlefield" -/
theorem okConsistentZoneConjunct :
    Predicate.check .object [] (.and [creature, .inZone battlefield]) = [] := by decide

/-- "creature that isn't on the battlefield" -/
theorem badNotOnBattlefield :
    Predicate.check .object [] (.and [creature, .not (.inZone battlefield)]) = [.zoneCoherent] := by
  decide

/-- "creature of the chosen color" -/
theorem okConsistentQualityConjunct :
    Predicate.check .object [⟨.a, .one, .quality .color⟩]
      (.and [creature, ofChosen .color]) = [] := by
  decide

/-- "of the chosen color and not of the chosen color" -/
theorem okQualitySelfNegation :
    Predicate.check .object [⟨.a, .one, .quality .color⟩]
      (.and [ofChosen .color, .not (ofChosen .color)]) = [] := by
  decide

/-- "creature that is a noncreature" -/
theorem okNestedSelfNegation :
    Predicate.check .object [] (.and [creature, .and [.not creature]]) = [] := by decide

/-- "attacking card in your hand" -/
theorem badAttackingInHand :
    Predicate.check .object [] (.and [attacking, .inZone hand]) = [.zoneCoherent] := by decide

/-- "Discard a card." -/
theorem okDiscardACardFromHand :
    Instruction.check [] (discard (a (.inZone hand)) (agent := .you)) = [] := by decide

/-- "Discard this creature." -/
theorem badDiscardThisCreature :
    Instruction.check [] (discard thisCreature (agent := .you)) = [.zoneFits] := by decide

/-- "another creature" -/
theorem okOtherAnchored :
    Predicate.check .object
      [⟨.target, .one, .object (some .creature) (some .battlefield) none none none⟩]
      (.and [creature, .other]) = [] := by
  decide

/-- "another other creature" -/
theorem badDoubleOther :
    Predicate.check .object
      [⟨.target, .one, .object (some .creature) (some .battlefield) none none none⟩]
      (.and [creature, .other, .other]) = [.otherAnchored] := by
  decide

/-- "You discard a card." -/
theorem okDiscardHandCard :
    Instruction.check [⟨.a, .one, .object none none none none none⟩]
      (discard (a (.inZone hand)) (agent := .you)) = [] := by
  decide

/-- "You discard it." -/
theorem badDiscardIt :
    Instruction.check [⟨.a, .one, .object none none none none none⟩] (discard it (agent := .you))
      = [.zoneFits] := by
  decide

/-- "creature card in a graveyard" -/
theorem okZoneCoherentAnd :
    Predicate.check .object [] (.and [creature, .inZone graveyard]) = [] := by decide

/-- "a creature you control in your graveyard" -/
theorem badControlledInGraveyard :
    Predicate.check .object [] (.and [creature, .hasPossessor .controller .you, .inZone graveyard])
      = [.zoneCoherent] := by
  decide

/-- "artifact or enchantment" -/
theorem okFlatDisjunction : Predicate.check .object [] (.or [artifact, enchantment]) = [] := by
  decide

/-- a coordination of no alternatives -/
theorem badEmptyOr : Predicate.check .object [] (.or []) = [.nonEmpty] := by decide

/-- "artifact or artifact" -/
theorem okRepeatedDisjunct : Predicate.check .object [] (.or [artifact, artifact]) = [] := by decide

/-- "artifact or attacking" -/
theorem badHeadlessDisjunct :
    Predicate.check .object [] (.or [artifact, attacking]) = [.parallelDisjuncts] := by decide

/-- "in your hand or in your graveyard" -/
theorem badCrossZoneDisjunction :
    Predicate.check .object [] (.or [.inZone hand, .inZone graveyard]) = [.parallelDisjuncts] := by
  decide

/-- "spell or permanent" -/
theorem badSpellOrPermanentSubject :
    Predicate.check .object [] (.or [spell, permanent]) = [.parallelDisjuncts] := by decide

/-- "Destroy target creature if it's on the battlefield." -/
theorem okMatchesZoneFits :
    Instruction.check []
      (.doOnlyIf (destroy (target creature)) (.matches it (.inZone battlefield)) none) = [] := by
  decide

/-- "Destroy target creature if it's in a graveyard." -/
theorem badTrailingPostStateZone :
    Instruction.check []
      (.doOnlyIf (destroy (target creature)) (.matches it (.inZone graveyard)) none)
        = [.zoneFits] := by
  decide

/-- "Choose a card in your hand. You discard that card." -/
theorem okChosenCardRemention :
    Instruction.check [] (.sequence [choose (a (.inZone hand)), discard (that .card) (agent :=
        .you)])
      = [] := by
  decide

/-- "Draw a card. Exile that card." -/
theorem badDrawnCardRemention :
    Instruction.check [] (.sequence [draw (.lit 1) (agent := .you), exile (that .card)])
      = [.anaphor (.word .card) .one 0] := by
  decide

/-- "Draw a card and you gain 1 life." -/
theorem okNonEmptyBatch :
    Instruction.check [] (.performSimultaneously [draw (.lit 1) (agent := .you), gainLife (.lit 1)
        (agent := .you)]) = [] := by
  decide

/-- an empty batch -/
theorem badEmptySimultaneous : Instruction.check [] (.performSimultaneously []) = [.nonEmpty] := by
    decide

/-- "Exile target creature and destroy that card." -/
theorem badSimultaneousReadsRetag :
    Instruction.check [] (.performSimultaneously [exile (target creature), destroy (that .card)])
      = [.zoneFits] := by
  decide

/-- "Deal 2 damage to target creature. Then tap it." -/
theorem okTapAfterSimultaneousDamage :
    Instruction.check []
      (.sequence [.performSimultaneously [.dealDamage .this (.lit 2) (target creature)], tap it])
      = [] := by
  decide

/-- "Exile target creature. Then tap it." A simultaneous move announces the object in the
exile zone [CR#701.13a] and only untapped permanents can be tapped [CR#701.26a];
`okTapAfterSimultaneousDamage` spells the admitted read. -/
theorem badTapAfterSimultaneousExile :
    Instruction.check [] (.sequence [.performSimultaneously [exile (target creature)], tap it])
      = [.zoneIs .battlefield] := by
  decide

/-- The same sentence with the exile as its own clause. -/
theorem badTapAfterSequentialExile :
    Instruction.check [] (.sequence [exile (target creature), tap it])
      = [.zoneIs .battlefield] := by
  decide

/-- "Gain control of target creature." -/
theorem okGainControlBattlefield :
    Instruction.check [] (gainControl (target creature) none (agent := .you)) = [] := by decide

/-- "Gain control of target creature card in a graveyard." -/
theorem badGainControlGraveyard :
    Instruction.check [] (gainControl (target (.and [creature, .inZone graveyard])) none (agent :=
        .you))
      = [.zoneIs .battlefield] := by
  decide

/-- "Reveal the top four cards. Put them on the bottom in any order." -/
theorem okPluralOrderRider :
    Instruction.check []
      (.sequence
        [revealCards (topSlice (.lit 4)), .move (those .card) (onBottomIn .anyOrder) []]) = [] := by
  decide

theorem badSingularOrderRider :
    Instruction.check []
      (.sequence [lookAt (topSlice (.lit 1)), .move (that .card) (onBottomIn .anyOrder) []])
      = [.arrangementOk] := by
  decide

/-- "Look at the top four cards of your library. Put them into your hand." -/
theorem okSliceCardRead :
    Instruction.check [] (.sequence [lookAt (topSlice (.lit 4)), .move (those .card) hand []])
      = [] := by
  decide

theorem badSliceTypeRead :
    Instruction.check []
      (.sequence [lookAt (topSlice (.lit 4)), .move (those (.type .creature)) hand []])
      = [.anaphor (.word (.type .creature)) .many 0] := by
  decide

/-- "Search your library for a creature card." -/
theorem okSearchZoneFreeDescription :
    Instruction.check [] (searchLibraryFor (exactly 1) creature) = [] := by decide

/-- "If you would search your library for a creature card, instead search your library for a
creature card and reveal that card." [CR#614.1a] -/
theorem okInsteadOfSearchRevealsIt :
    Instruction.check [] (.replace (searchLibraryFor (exactly 1) creature) revealIt) = [] := by
  decide

/-- "Search your library for a creature card in a graveyard." -/
theorem badSearchZonedDescription :
    Instruction.check [] (searchLibraryFor (exactly 1) (.and [creature, .inZone graveyard]))
      = [.zoneCoherent] := by
  decide

/-- "Each player mills a card. Exile it." -/
theorem badDistributedMillSingular :
    Instruction.check []
      (.sequence [mill (.lit 1) (each .anyPlayer) (agent := (each .anyPlayer)), exile it])
      = [.anaphor .bare .one 0] := by
  decide

/-- "one of up to two target creatures" -/
theorem okPartitiveOfTargetGroup :
    Instruction.check []
      (exile (.someOf (.counted (exactly 1)) none (.described (.target (upTo 2)) creature)))
        = [] := by
  decide

/-- "one of a creature you control" -/
theorem badPartitiveOfDescription :
    Instruction.check [] (exile (.someOf (.counted (exactly 1)) none (a creature)))
      = [.partitiveBase] := by
  decide

/-- "Look at the top four cards of your library. Exile one of them." -/
theorem okPartitiveOfThem :
    Instruction.check []
      (.sequence [lookAt (topSlice (.lit 4)), exile (someOf (exactly 1) them)]) = [] := by
  decide

/-- "two of one of them" -/
theorem badPartitiveOfPartitive :
    Instruction.check []
      (.sequence
        [lookAt (topSlice (.lit 4)), exile (someOf (exactly 1) (someOf (exactly 1) them))])
      = [.partitiveBase] := by
  decide

/-- "each of the rest" -/
theorem badEachOfTheRest :
    Instruction.check []
      (.sequence
        [ lookAt (topSlice (.lit 4)), .move (someOf (exactly 1) them) hand [],
          exile (.eachOf (theRest .object)) ]) = [.groupMention] := by
  decide

/-- "If a creature you control would die this turn, exile it instead." -/
theorem okWouldDieOnBattlefield :
    Instruction.check []
      (replaceEvent (.dies (target creatureYouControl)) (exile it) (some .thisTurn)) = [] := by
  decide

theorem badWouldDieInGraveyard :
    Instruction.check []
      (replaceEvent (.dies (target (.and [creature, .inZone (graveyardOf .you)]))) (exile it)
        (some .thisTurn)) = [.zoneIs .battlefield] := by
  decide

/-- "Reveal the top card of your library. Put that card into your hand." -/
theorem okSingularCardRetag :
    Instruction.check []
      (.sequence [revealCards (topSlice (.lit 1)), .move (that .card) hand []]) = [] := by
  decide

/-- "Exile target creature until this creature leaves the battlefield. Put that card into
its owner's hand." The held clause announces the exiled object where it now is
[CR#701.13a], so the card read resolves. -/
theorem okHeldUntilExileRetag :
    Instruction.check []
      (.sequence
        [ exileUntil (target creature) (leavesBattlefield thisCreature),
          .move (that .card) hand [] ]) = [] := by
  decide

/-- "Put target creature onto the battlefield tapped." -/
theorem okMoveRidersToBattlefield :
    Instruction.check [] (.move (target creature) battlefield [.entersAs .tapped]) = [] := by decide

/-- "Put target creature into its owner's graveyard tapped." -/
theorem badMoveRidersToGraveyard :
    Instruction.check [] (.move (target creature) graveyard [.entersAs .tapped])
      = [.ridersFit] := by
  decide

/-- "Put target creature into its owner's hand under your control." -/
theorem badMoveControlToHand :
    Instruction.check [] (.move (target creature) hand [.under .you]) = [.ridersFit] := by decide

/-- "Put target creature onto the battlefield under your control." -/
theorem okMoveRidersSingularController :
    Instruction.check [] (.move (target creature) battlefield [.under .you]) = [] := by decide

/-- "Put target creature onto the battlefield under the other players' control." -/
theorem badMoveRidersPluralController :
    Instruction.check [] (.move (target creature) battlefield [.under (allOf otherPlayer)])
      = [.ctrlOverride] := by
  decide

/-- "a card exiled with this permanent" -/
theorem okExiledWithThis : Predicate.check .object [] (.exiledWith .this) = [] := by decide

/-- "cards exiled with target creature" -/
theorem badExiledWithOtherSource :
    Predicate.check .object [] (.exiledWith (target creature)) = [.linkSource] := by decide

/-- "a card exiled with an artifact" -/
theorem badExiledWithDescribedSource :
    Predicate.check .object [] (.exiledWith (a artifact)) = [.linkSource] := by decide

/-- "a creature you control" -/
theorem okZoneCoherentConjunction :
    Predicate.check .object [] (.and [creature, .hasPossessor .controller .you]) = [] := by decide

/-- "a card you control exiled with this artifact" -/
theorem badExiledWithControlled :
    Predicate.check .object [] (.and [.hasPossessor .controller .you, exiledWithThisArtifact])
      = [.zoneCoherent] := by
  decide

/-- "an attacking creature" -/
theorem okAttackingCreature : Predicate.check .object [] (.and [attacking, creature]) = [] := by
  decide

/-- "an attacking creature exiled with this creature" -/
theorem badExiledWithAttacking :
    Predicate.check .object [] (.and [attacking, exiledWithThisArtifact]) = [.zoneCoherent] := by
  decide

/-- "Put target creature card from a graveyard onto the battlefield tapped." -/
theorem okCreatureOntoBattlefieldTapped :
    Instruction.check []
      (putOntoBattlefieldTapped (target (.and [creature, .inZone graveyard]))) = [] := by
  decide

/-- "Put target instant card from a graveyard onto the battlefield tapped." -/
theorem badInstantOntoBattlefield :
    Instruction.check []
      (putOntoBattlefieldTapped (target (.and [.hasType .instant, .inZone graveyard])))
      = [.placeable] := by
  decide

/-- "Remove target creature from combat." -/
theorem okRemoveFromCombatOnBattlefield :
    Instruction.check [] (.removeFromCombat (target creature)) = [] := by decide

/-- "Remove target creature card in your graveyard from combat." -/
theorem badRemoveFromCombatGraveyard :
    Instruction.check []
      (.removeFromCombat (target (.and [creature, .inZone (graveyardOf .you)])))
      = [.zoneIs .battlefield] := by
  decide

/-- "Players can't untap more than one creature during each untap step." -/
theorem okUntapCapBattlefieldSet :
    Ability.check []
      (.static (cantMoreThan (.playerGroup .allPlayers) (.action "Untap") 1 creature))
      = [] := by
  decide

theorem badUntapCapGraveyardSet :
    Ability.check []
      (.static
        (cantMoreThan (.playerGroup .allPlayers) (.action "Untap") 1
          (.and [creature, .inZone (graveyardOf .you)]))) = [.deonticPatientOk] := by
  decide

/-- "if this creature is attacking" -/
theorem okMatchesBattlefieldZone :
    Ability.check []
      (.triggered (lastCounterRemoved (.named "Time") thisCreature) [] none [] none none
        (some (.matches thisCreature attacking)) (draw (.lit 1) (agent := .you))) = [] := by
  decide

theorem badExileCheckOnSortedSelf :
    Ability.check []
      (.triggered (lastCounterRemoved (.named "Time") thisCreature) [] none [] none none
        (some (.matches thisCreature (.inZone exileZone))) (draw (.lit 1) (agent := .you))) =
            [.zoneFits] := by
  decide

/-- "Draw cards equal to the difference." under a comparison -/
theorem okDifferenceAfterComparison :
    Instruction.check []
      (.doIf (.compareAmt (countOf (.inZone (handOf .you))) .less (.lit 7))
        (draw .theDifference (agent := .you)) none) = [] := by
  decide

/-- "Draw cards equal to the difference." -/
theorem badUnlicensedDifference :
    Instruction.check [] (draw .theDifference (agent := .you)) = [.gapInScope 0] := by decide

/-- "When this creature enters, if you control fewer than seven creatures, draw cards equal
to the difference." -/
theorem okDifferenceUnderComparisonTrigger :
    Ability.check []
      (.triggered (.enters thisCreature none) [] none [] none none
        (some (.compareAmt (countOf creatureYouControl) .less (.lit 7)))
        (draw .theDifference (agent := .you))) = [] := by
  decide

theorem badNonComparisonDifference :
    Ability.check []
      (.triggered (.enters thisCreature none) [] none [] none none
        (some (exists_ creatureYouControl)) (draw .theDifference (agent := .you))) = [.gapInScope 0]
            := by
  decide

/-- "Goad target creature." -/
theorem okGoadOnBattlefield :
    Instruction.check [] (.gainDesignation (target creature) "goaded" .instructed none) = [] := by
  decide

/-- "goad target creature card in your graveyard" -/
theorem badGoadInGraveyard :
    Instruction.check []
      (.gainDesignation (target (.and [creature, .inZone (graveyardOf .you)])) "goaded" .instructed
        none) = [.designationHolder "goaded" .object] := by
  decide

/-- "Whenever a creature attacks a planeswalker, …" -/
theorem okPlaneswalkerAttackDefender :
    GameEvent.check [] (.combat .attackerOf (a creature) (some (a (.hasType .planeswalker))))
      = [] := by
  decide

/-- "Whenever a creature attacks another creature, …" -/
theorem badCreatureAttackDefender :
    GameEvent.check [] (.combat .attackerOf (a creature) (some (a creature))) = [.attackable] := by
  decide

/-- "Look at the top four cards of your library. An opponent chooses one of them." -/
theorem okAgentChoosesSomeOf :
    Instruction.check []
      (.sequence
        [ lookAt (topSlice (.lit 4)),
          .choose none (someOf (exactly 1) them) .openly none (agent := (some (a .opponent))) ]) =
              [] := by
  decide

theorem badAgentChooseTheRest :
    Instruction.check []
      (.sequence
        [ lookAt (topSlice (.lit 4)), .move (someOf (exactly 1) them) hand [],
          .choose none (theRest .object) .openly none (agent := (some (a .opponent))) ])
            = [.choiceClause] := by
  decide

/-- "… two cards with the same name in your hand …" -/
theorem okPluralNameAgreement :
    NounPhrase.check (some .object) []
      (.namesAgree .sameName (counted (exactly 2) (.and [.not land, .inZone hand]))) = [] := by
  decide

/-- "… target creature with different names." -/
theorem badSingularNameAgreement :
    NounPhrase.check (some .object) [] (.namesAgree .differentNames (target creature))
      = [.plural] := by
  decide

/-- "This spell can't be countered." -/
theorem okCantBeCountered :
    StaticSpec.check [] (objectCant (.action "Counter") .this) = [] := by decide

/-- "Creature cards in graveyards can't be countered." -/
theorem badCounteredInGraveyard :
    StaticSpec.check []
      (objectCant (.action "Counter") (allOf (.and [creature, .inZone graveyard])))
      = [.deedFits] := by
  decide

/-- "You may look at the top card of your library any time." -/
theorem okLookAtTopOfLibrary :
    StaticSpec.check [] (.visibility .lookAt .you .topOfLibrary) = [] := by decide

/-- "You may look at your hand any time." -/
theorem badLookAtHandRider :
    StaticSpec.check [] (.visibility .lookAt .you .wholeHand) = [.visibilityOk] := by decide

/-- "Lands you control are every creature type.": an object can't gain a subtype that
corresponds to none of its card types [CR#205.3d]. -/
theorem badEveryCreatureTypeOnLand :
    StaticSpec.check []
      (.qualityChange (allOf (.and [land, .hasPossessor .controller .you])) .adds
        (.everyTypeOf .creature)) = [.becomesOk] := by
  decide

/-- "Creatures you control are every basic land type.": an object can't gain a subtype that
corresponds to none of its card types [CR#205.3d]. -/
theorem badEveryBasicLandTypeOnCreature :
    StaticSpec.check []
      (.qualityChange (allOf (.and [creature, .hasPossessor .controller .you])) .adds
        (.everyTypeOf .basicLand)) = [.becomesOk] := by
  decide

/-- "Regenerate target creature." -/
theorem okRegenerateCreature :
    Instruction.check [] (.regenerate (target creature))
      = [] := by decide

/-- "Regenerate target creature card in your graveyard." -/
theorem badRegenerateInGraveyard :
    Instruction.check [] (.regenerate (a (.and [creature, .inZone graveyard])))
      = [.zoneIs .battlefield] := by
  decide

theorem badRegenerateBareThis :
    Instruction.check [] (.regenerate .this) = [.zoneIs .battlefield] := by decide

/-- "Creatures can't be regenerated." -/
theorem okRegenerationBanOnBattlefield :
    StaticSpec.check [] (objectCant (.action "Regenerate") (allOf creature)) = [] := by decide

/-- "Creature cards in your graveyard can't be regenerated." -/
theorem badRegeneratedInGraveyard :
    StaticSpec.check [] (objectCant (.action "Regenerate") (a (.and [creature, .inZone graveyard])))
      = [.deedFits] := by
  decide

/-- "Each opponent discards a card. Simultaneously, exile those cards." -/
theorem distributedDeedReadsBackPluralUnderAnnouncement :
    Instruction.check []
      (.performSimultaneously [discard (a (.inZone hand)) (agent := (each .opponent)), exile (those
          .card)])
        = [] := by
  decide

/-- "Discard a card. Simultaneously, exile that card." -/
theorem okThatAfterSingularDiscard :
    Instruction.check [] (.performSimultaneously [discard (a (.inZone hand)) (agent := .you), exile
        (that .card)])
      = [] := by
  decide

/-- "Each opponent discards a card. Simultaneously, exile that card." -/
theorem badDistributedAnnouncedDiscardSingular :
    Instruction.check []
      (.performSimultaneously [discard (a (.inZone hand)) (agent := (each .opponent)), exile (that
          .card)])
      = [.anaphor (.word .card) .one 0] := by
  decide

/-- "Draw a card. You gain 1 life." -/
theorem okNonEmptySequence :
    Instruction.check [] (.sequence [draw (.lit 1) (agent := .you), gainLife (.lit 1) (agent :=
        .you)]) = [] := by
  decide

/-- an empty sentence list -/
theorem badEmptySequence : Instruction.check [] (.sequence []) = [.nonEmpty] := by decide

/-- "Reveal your hand." -/
theorem okRevealHand : Instruction.check [] (.expose .reveal (.zone hand) (agent := .you)) = [] :=
    by decide

/-- "Reveal your graveyard." -/
theorem badRevealGraveyard :
    Instruction.check [] (.expose .reveal (.zone graveyard) (agent := .you))
      = [.exposableZone .graveyard] := by
  decide

/-- "Sacrifice a creature." -/
theorem okSacrificeBattlefieldNoun :
    Instruction.check [] (sacrifice (a creature) (agent := .you)) = [] := by decide

theorem badInterceptReplacementAntecedent :
    Instruction.check []
      (.sequence
        [ replaceNextEvent (.draws .you)
            (create (.lit 1) (creatureToken 1 1 [.green] [creatureType "Soldier"]))
            (some .thisTurn),
          sacrifice it (agent := .you) ]) = [.anaphor .bare .one 0, .zoneFits] := by
  decide

/-- "If you would draw a card, draw two cards instead." -/
theorem okFlatInstead :
    Instruction.check [] (.replace (draw (.lit 1) (agent := .you)) (draw (.lit 2) (agent := .you)))
        = [] := by decide

theorem badNestedInstead :
    Instruction.check []
      (.replace (.replace (draw (.lit 1) (agent := .you)) (draw (.lit 2) (agent := .you))) (draw
          (.lit 3) (agent := .you)))
      = [.notInstead] := by
  decide

/-- "Untap target creature." -/
theorem okUntapInstruction :
    Instruction.check [] (.setStatus .untapped (target creature)) = [] := by decide

/-- "Unflip target creature." -/
theorem badUnflipInstruction :
    Instruction.check [] (.setStatus .unflipped (target creature)) = [.statusMarkable] := by decide

/-- "… if a creature died this turn, …" -/
theorem okDeathLookback :
    Condition.check [] (.happened (a creature) (.mk .death .thisTurn none)) = [] := by decide

/-- "… if a creature was put this turn, …" -/
theorem badPlacementLookback :
    Condition.check [] (.happened (a creature) (.mk .placement .thisTurn none))
      = [.complementWritten] := by
  decide

/-- "Counter target activated ability." -/
theorem okCounterAbility :
    Instruction.check [] (.counterSpell (target (.abilityHead .anyActivated))) = [] := by decide

/-- "Exile target creature." -/
theorem okObjectMovedToAZone :
    Instruction.check [] (.move (target creature) exileZone []) = [] := by decide

/-- "Exile target activated ability." An ability on the stack is an object [CR#113.1c] that
ceases to exist when it leaves the stack [CR#608.2n]; it never changes zones. -/
theorem badAbilityMovedToAZone :
    Instruction.check [] (.move (target (.abilityHead .anyActivated)) exileZone [])
      = [.movable] := by
  decide

/-- "Exchange control of target artifact and target creature." -/
theorem okExchangeControlOnField :
    Instruction.check [] (.exchange (.controlOf (target artifact) (target creature)))
      = [] := by decide

/-- "Exchange control of target artifact and target creature card in your graveyard." A card
that isn't a permanent or a spell has no controller [CR#108.4], so there is nothing to exchange
[CR#701.12b]. -/
theorem badExchangeControlInGraveyard :
    Instruction.check []
      (.exchange
        (.controlOf (target artifact) (target (.and [creature, .inZone (graveyardOf .you)]))))
      = [.controlExchangeZone] := by
  decide

/-- "Exchange this Aura with an Aura card in your hand." (aura swap, [CR#702.65a]) -/
theorem okExchangeCardsAcrossZones :
    Instruction.check []
      (.exchange
        (.cardsAcross thisAura (a (.and [.hasSubtype (enchantmentType "Aura"), .inZone hand]))))
      = [] := by
  decide

/-- "Exchange a card in your hand with a card in your hand." Cards are exchanged with cards in
a DIFFERENT zone [CR#701.12d]. -/
theorem badExchangeCardsSameZone :
    Instruction.check [] (.exchange (.cardsAcross (a (.inZone hand)) (a (.inZone hand))))
      = [.cardSwapZones] := by
  decide

/-- "Exchange your hand and graveyard." -/
theorem okExchangeTwoZones :
    Instruction.check [] (.exchange (.zones hand graveyard))
      = [] := by decide

/-- "Exchange your hand and your hand." An exchange of two zones puts each zone's cards in
the other zone [CR#701.12d,701.12f]. -/
theorem badExchangeZoneWithItself :
    Instruction.check [] (.exchange (.zones hand hand)) = [.zoneSwap] := by decide

/-- "Exchange your hand and the battlefield." Only cards are exchanged between zones this way
[CR#701.12d]; the battlefield holds permanents [CR#110.1]. -/
theorem badExchangeBattlefieldZone :
    Instruction.check [] (.exchange (.zones hand battlefield)) = [.zoneSwap] := by decide

/-- "the exiled card": the definite reference is licensed by the link to the exiling ability
printed on the same object [CR#607.2a], and still resolves when that ability exiled more than
one card [CR#607.3]. -/
theorem okTheExiledCard : NounPhrase.check (some .object) [] (the (.exiledWith .this)) = [] := by
  decide

/-- "the card in exile": the exile zone holds any number of cards, and nothing links the
reference to one of them. -/
theorem badTheCardInExile :
    NounPhrase.check (some .object) [] (the (.inZone exileZone)) = [.uniquifying] := by decide

/-- "Exchange your life total with this creature's toughness." (Tree of Redemption): two
settable values [CR#701.12g]. -/
theorem okExchangeTwoValues :
    Instruction.check [] (.exchange (.values (lifeTotalOf .you) (toughnessOf thisCreature)))
      = [] := by
  decide

/-- "Exchange this creature's power with this creature's power.": each value would become
equal to its own previous value [CR#701.12g]. -/
theorem badExchangeValueWithItself :
    Instruction.check [] (.exchange (.values (powerOf thisCreature) (powerOf thisCreature)))
      = [.selfExchanged] := by
  decide

/-- "Exchange this creature's power with three.": a literal is not a value the game can set
[CR#701.12g]. -/
theorem badExchangeLiteralValue :
    Instruction.check [] (.exchange (.values (powerOf thisCreature) (.lit 3)))
      = [.settableValue] := by
  decide

/-- "Exchange the text boxes of this creature and another creature." [CR#701.12h] -/
theorem okExchangeTextBoxes :
    Instruction.check []
      (.exchange (.textBoxes thisCreature (a (.and [creature, .otherThan .this])))) = [] := by
  decide

/-- "Exchange the text boxes of this creature and a creature card in your graveyard.": a text
box is exchanged between permanents on the battlefield. -/
theorem badExchangeTextBoxInGraveyard :
    Instruction.check []
      (.exchange (.textBoxes thisCreature (a (.and [creature, .inZone (graveyardOf .you)]))))
      = [.zoneIs .battlefield] := by
  decide

end Semantics.Proofs.Zone
