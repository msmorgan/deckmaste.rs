module Experimental.Proofs.Zone

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Destroy target tapped creature."
public export
okTappedBattlefield : Instruction []
okTappedBattlefield =
  Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped]))

||| "Destroy target tapped creature card in your graveyard."
public export
badTappedGraveyard : Unspellable (Instruction []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped,
                                       InZone (Macros.graveyardOf You)] {zc = ok})))
badTappedGraveyard Oh impossible

||| "Destroy target tapped untapped creature."
public export
badTappedUntapped : Unspellable (Instruction []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped, Macros.untapped] {cf = ok})))
badTappedUntapped Oh impossible

||| "Untap target creature card in your graveyard."
public export
badUntapGraveyard : Unspellable (Instruction []) (\ok =>
  SetStatus Untapped (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badUntapGraveyard Oh impossible

||| "Destroy target permanent instant."
public export
badPermanentInstant : Unspellable (Instruction []) (\ok =>
  Macros.destroy (Macros.target (And [Permanent, HasType Instant] {cf = ok})))
badPermanentInstant Oh impossible

public export
badThatPermanentDeparted : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Permanent),
               SetStatus Tapped (Macros.That PermanentW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badThatPermanentDeparted (Refl, _) impossible

||| "Destroy target token card in your graveyard."
public export
badTokenGraveyard : Unspellable (Instruction []) (\ok =>
  Macros.destroy (Macros.target (And [IsToken, InZone (Macros.graveyardOf You)] {zc = ok})))
badTokenGraveyard Oh impossible

||| "Destroy target card token."
public export
badCardTokenTarget : Unspellable (Instruction []) (\ok =>
  Macros.destroy (Macros.target (And [IsToken, IsCard] {cf = ok})))
badCardTokenTarget Oh impossible

||| "Tap target creature. Untap that token."
public export
badThatTokenOfCard : Unspellable (Instruction []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
               SetStatus Untapped (Macros.That TokenW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badThatTokenOfCard (Refl, _) impossible

||| "For each opponent, exile a creature that player controls. Put those cards
||| onto the battlefield." — the printed shape of Breach the Multiverse's middle
||| pair. The loop body moves only what it introduced itself, so the enclosing
||| stack survives and the exiled cards read back after the loop as a plural.
public export
okLoopMovesItsOwn : Instruction []
okLoopMovesItsOwn =
  Sequentially [ ForEachOf (Macros.each Opponent)
                   (Macros.exile (Macros.a (And [Macros.creature,
                      HasPossessor ControllerAx (Macros.That PlayerW)])))
               , Macros.putOntoBattlefield (Macros.Them) ]

||| "Tap target creature. For each opponent, exile it. Untap it." The loop body
||| exiles an object bound OUTSIDE the loop, so `ForEachOf`'s introductions can
||| no longer be told from the mutated stack and the loop would republish the
||| creature's stale battlefield zone. Refused at `KeepsOuter`; the unlooped
||| spelling of the same sentence is refused one clause later, at `untap`'s
||| battlefield gate ([CR#701.26b], `badUnloopedZoneMoveRead`).
public export
badLoopedZoneMoveRead : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.tap (Macros.target Macros.creature)
               , ForEachOf (Macros.each Opponent)
                           (Macros.exile (Macros.It)) {ko = ok}
               , Macros.untap (Macros.It) ])
badLoopedZoneMoveRead Refl impossible

||| "Tap target creature. Exile it. Untap it." — the same sentence without the
||| loop: exile puts the creature in the exile zone, where it is a new object
||| and no longer a permanent, so untapping it is refused [CR#400.7,701.26b].
public export
badUnloopedZoneMoveRead : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.tap (Macros.target Macros.creature)
               , Macros.exile (Macros.It)
               , Macros.untap (Macros.It) {ok} ])
badUnloopedZoneMoveRead Oh impossible

||| "For each color among permanents on the battlefield, draw a card."
public export
okKindLoopKeepsOuter : Instruction []
okKindLoopKeepsOuter =
  ForEachKindOf ColorAxis (Just (Macros.allOf Permanent)) Color (Draw You (Lit 1))

||| "Tap target creature. For each color among permanents on the battlefield,
||| exile it. Untap it." `ForEachKindOf` republishes its enclosing stack the
||| same way `ForEachOf` does, so a body that moves an object bound outside the
||| loop is refused at the same obligation [CR#400.7,701.26b].
public export
badKindLoopZoneMoveRead : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.tap (Macros.target Macros.creature)
               , ForEachKindOf ColorAxis (Just (Macros.allOf Permanent)) Color
                               (Macros.exile (Macros.It)) {ko = ok}
               , Macros.untap (Macros.It) ])
badKindLoopZoneMoveRead Refl impossible

||| "Each opponent exiles a creature they control. Put those cards onto the
||| battlefield." The distributive twin of `okLoopMovesItsOwn`: the deed moves
||| only what the agent phrase introduced, so the enclosing stack survives and
||| the exiled cards read back after the deed as a plural.
public export
okDistributedMovesItsOwn : Instruction []
okDistributedMovesItsOwn =
  Sequentially [ Macros.exiles (Macros.each Opponent)
                   (Macros.a (And [Macros.creature,
                      HasPossessor ControllerAx (Macros.That PlayerW)]))
               , Macros.putOntoBattlefield (Macros.Them) ]

||| "Tap target creature. Each opponent exiles it. Untap it." The distributive
||| deed exiles an object bound OUTSIDE the agent phrase, so `distributedDelta`
||| would republish the pre-deed stack — the creature's stale battlefield zone —
||| over the exile. Refused at `Enact`'s distribution obligation; the singular
||| spelling of the same sentence is refused one clause later, at `untap`'s
||| battlefield gate ([CR#400.7,701.26b], `badUnloopedZoneMoveRead`).
public export
badDistributedZoneMoveRead : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.tap (Macros.target Macros.creature)
               , Macros.exiles (Macros.each Opponent) (Macros.It) {ke = ok}
               , Macros.untap (Macros.It) ])
badDistributedZoneMoveRead EachOnlyAdds impossible
badDistributedZoneMoveRead EachClosesOwnParts impossible

||| "When target creature dies this turn, return that card to the battlefield."
public export
okDiesBattlefield : Instruction []
okDiesBattlefield =
  Delayed (Dies (Macros.target Macros.creature)) [] (Just ThisTurn)
          (Move (Macros.That CardW) Macros.battlefieldZ [])

public export
badDiesInGraveyard : Unspellable (Instruction []) (\ok =>
  Delayed (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok}) [] (Just ThisTurn)
          (Move (Macros.That CardW) Macros.battlefieldZ []))
badDiesInGraveyard Oh impossible

||| "not of a color other than white"
public export
okNegatedRestrictedQuality : Predicate [] (Quality Color)
okNegatedRestrictedQuality =
  Not (QualityNoun Color (Just (ColorOtherThan White)))

||| "noncolor"
public export
badNegatedQualityHead : Unspellable (Predicate [] (Quality Color)) (\ok =>
  Not (QualityNoun Color Nothing) {ng = ok})
badNegatedQualityHead Oh impossible

||| "non-player"
public export
badNegatedPlayerHead : Unspellable (Predicate [] Player) (\ok =>
  Not AnyPlayer {ng = ok})
badNegatedPlayerHead Oh impossible

||| "Tap target creature an opponent controls. That player loses 1 life."
public export
okPositiveAntecedent : Instruction []
okPositiveAntecedent =
  Sequentially [ SetStatus Tapped (Macros.target (And [Macros.creature,
                    HasPossessor ControllerAx Macros.anOpponent]))
               , Macros.losesLife (Macros.That PlayerW) (Lit 1) ]

||| "Tap target creature an opponent doesn't control. That player loses 1 life."
public export
badNegatedAntecedent : Unspellable (Instruction []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target (And [Macros.creature, Not (HasPossessor ControllerAx Macros.anOpponent)])),
                Macros.losesLife (Macros.That PlayerW {ok}) (Lit 1)])
badNegatedAntecedent Refl impossible

||| "Destroy target creature on the battlefield."
public export
okSingleZoneOnTarget : Instruction []
okSingleZoneOnTarget =
  Macros.destroy (Macros.target (And [Macros.creature, InZone Macros.battlefieldZ]))

||| "Destroy target creature on the battlefield in a graveyard."
public export
badConflictingZones : Unspellable (Instruction []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone Macros.battlefieldZ, InZone Macros.graveyardZ] {zc = ok})))
badConflictingZones Oh impossible

||| "Return target creature to its owner's hand."
public export
okMoveToHand : Instruction []
okMoveToHand = Move (Macros.target Macros.creature) Macros.handZ []

||| "Put target creature into target player's hand."
public export
badMoveToTargetsHand : Unspellable (Instruction []) (\ok =>
  Move (Macros.target Macros.creature) (Macros.handOf (Macros.target AnyPlayer)) [] {ok})
badMoveToTargetsHand HandOkBare impossible

||| "Destroy target creature."
public export
okDestroyBattlefield : Instruction []
okDestroyBattlefield = Macros.destroy (Macros.target Macros.creature)

||| "Destroy target creature card in your graveyard."
public export
badDestroyGraveyard : Unspellable (Instruction []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badDestroyGraveyard Oh impossible

||| "You discard a card."
public export
okDiscardHand : Instruction []
okDiscardHand = Macros.discard You (Macros.a (InZone Macros.handZ))

||| "You discard a creature."
public export
badDiscardBattlefield : Unspellable (Instruction []) (\ok =>
  Macros.discard You (Macros.a Macros.creature) {dk = ok})
badDiscardBattlefield DiscardTracked impossible

||| "Destroy target creature card in a graveyard."
public export
badDestroyGraveyardCard : Unspellable (Instruction []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) {ok})
badDestroyGraveyardCard Oh impossible

||| "Destroy a creature."
public export
okKnownVerbLabel : Instruction []
okKnownVerbLabel =
  Enact Nothing "Destroy"
        (Move (Macros.a Macros.creature) Macros.graveyardZ [])

||| A move labeled with a word outside the label catalog
public export
badUnknownVerbLabel : Unspellable (Instruction []) (\ok =>
  Enact Nothing "Descry" (Move (Macros.a Macros.creature) Macros.graveyardZ []) {kn = ok})
badUnknownVerbLabel ActInFactsTable impossible

||| "You exile a card from your hand."
public export
okAgentedKnownAct : Instruction []
okAgentedKnownAct =
  Enact (Just You) "Exile"
        (Move (Macros.a (InZone Macros.handZ)) Macros.exileZ [])

||| "You destroy target creature.": printed cards give Destroy a player
||| subject ("You destroy four lands", Burning of Xinye), so its facts row's
||| agent role names a player.
public export
okAgentedDestroy : Instruction []
okAgentedDestroy =
  Enact (Just You) "Destroy"
        (Move (Macros.target Macros.creature) Macros.graveyardZ [])

||| "You fight target creature." A spell or ability instructs a CREATURE to
||| fight [CR#701.14a], so `"Fight"`'s facts row gives its agent role no
||| player and the agented voice is refused; the printed sentence names the
||| fighting creatures, `Effect.Fights`.
public export
badAgentedAgentlessAct : Unspellable (Instruction []) (\ok =>
  Enact (Just You) "Fight"
        (Fights (Macros.target Macros.creature) (Macros.target Macros.creature)) {ag = ok})
badAgentedAgentlessAct Oh impossible

||| "1 life for each creature"
public export
okForEachOne : Amount []
okForEachOne = Macros.forEach 1 Macros.creature

||| "1 life for each 0 creatures"
public export
badForEachZero : Unspellable (Amount []) (\ok =>
  Macros.forEach 0 Macros.creature {nz = ok})
badForEachZero Oh impossible

||| "creature on the battlefield"
public export
okConsistentZoneConjunct : Predicate [] Object
okConsistentZoneConjunct = And [Macros.creature, InZone Macros.battlefieldZ]

||| "creature that isn't on the battlefield"
public export
badNotOnBattlefield : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Not (InZone Macros.battlefieldZ)] {zc = ok})
badNotOnBattlefield Oh impossible

||| "creature of the chosen color"
public export
okConsistentQualityConjunct :
  Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object
okConsistentQualityConjunct = And [Macros.creature, Macros.ofChosen Color]

||| "of the chosen color and not of the chosen color"
public export
okQualitySelfNegation : Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object
okQualitySelfNegation = And [Macros.ofChosen Color, Not (Macros.ofChosen Color)]

||| "creature that is a noncreature"
public export
okNestedSelfNegation : Predicate [] Object
okNestedSelfNegation = And [Macros.creature, And [Not Macros.creature]]

||| "attacking card in your hand"
public export
badAttackingInHand : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, InZone Macros.handZ] {zc = ok})
badAttackingInHand Oh impossible

||| "Discard a card."
public export
okDiscardACardFromHand : Instruction []
okDiscardACardFromHand = Macros.discard You (Macros.a (InZone Macros.handZ))

||| "Discard this creature."
public export
badDiscardThisCreature : Unspellable (Instruction []) (\ok =>
  Macros.discard You Macros.thisCreature {dk = ok})
badDiscardThisCreature DiscardTracked impossible

||| "another creature"
public export
okOtherAnchored :
  Predicate [MkBinding TargetD Object OneOf
                       (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing Nothing)] Object
okOtherAnchored = And [Macros.creature, Other]

||| "another other creature"
public export
badDoubleOther : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing Nothing)] Object)
  (\ok => And [Macros.creature, Other, Other] {oa = ok})
badDoubleOther Oh impossible

||| "You discard a card."
public export
okDiscardHandCard :
  Instruction [MkBinding AD Object OneOf
                    (ObjectP Nothing Nothing Nothing Nothing Nothing)]
okDiscardHandCard = Macros.discard You (Macros.a (InZone Macros.handZ))

||| "You discard it."
public export
badDiscardIt : Unspellable
  (Instruction [MkBinding AD Object OneOf (ObjectP Nothing Nothing Nothing Nothing Nothing)])
  (\ok => Macros.discard You ((Macros.It)) {dk = ok})
badDiscardIt DiscardTracked impossible

||| "creature card in a graveyard"
public export
okZoneCoherentAnd : Predicate [] Object
okZoneCoherentAnd = And [Macros.creature, InZone Macros.graveyardZ]

||| "a creature you control in your graveyard"
public export
badControlledInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, HasPossessor ControllerAx You, InZone Macros.graveyardZ] {zc = ok})
badControlledInGraveyard Oh impossible

||| "artifact or enchantment"
public export
okFlatDisjunction : Predicate [] Object
okFlatDisjunction = Or [Macros.artifact, Macros.enchantment]

||| a coordination of no alternatives
public export
badEmptyOr : Unspellable (Predicate [] Object) (\ok =>
  Or [] {ne = ok})
badEmptyOr IsNonEmpty impossible

||| "artifact or artifact"
public export
okRepeatedDisjunct : Predicate [] Object
okRepeatedDisjunct = Or [Macros.artifact, Macros.artifact]

||| "artifact or attacking"
public export
badHeadlessDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.artifact, Attacking] {pd = ok})
badHeadlessDisjunct Oh impossible

||| "in your hand or in your graveyard"
public export
badCrossZoneDisjunction : Unspellable (Predicate [] Object) (\ok =>
  Or [InZone Macros.handZ, InZone Macros.graveyardZ] {pd = ok})
badCrossZoneDisjunction Oh impossible

||| "spell or permanent"
public export
badSpellOrPermanentSubject : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.spell, Permanent] {pd = ok})
badSpellOrPermanentSubject Oh impossible

||| "Destroy target creature if it's on the battlefield."
public export
okMatchesZoneFits : Instruction []
okMatchesZoneFits =
  OnlyIf (Macros.destroy (Macros.target Macros.creature))
         (Matches (Macros.It) (InZone Macros.battlefieldZ)) Nothing

||| "Destroy target creature if it's in a graveyard."
public export
badTrailingPostStateZone : Unspellable (Instruction []) (\ok =>
  OnlyIf (Macros.destroy (Macros.target Macros.creature)) (Matches ((Macros.It)) (InZone Macros.graveyardZ) {zc = ok}) Nothing)
badTrailingPostStateZone Oh impossible

||| "Choose a card in your hand. You discard that card."
public export
okChosenCardRemention : Instruction []
okChosenCardRemention =
  Sequentially [Macros.choose (Macros.a (InZone Macros.handZ)),
                Macros.discard You (Macros.That CardW)]

||| "Draw a card. Exile that card."
public export
badDrawnCardRemention : Unspellable (Instruction []) (\ok =>
  Sequentially [(Draw You (Lit 1)), Macros.exile (Macros.That CardW {ok})])
badDrawnCardRemention Refl impossible

||| "Draw a card and you gain 1 life."
public export
okNonEmptyBatch : Instruction []
okNonEmptyBatch =
  Simultaneously [Draw You (Lit 1), Macros.gainsLife You (Lit 1)]

||| an empty batch
public export
badEmptySimultaneous : Unspellable (Instruction []) (\ok =>
  Simultaneously [] {ne = ok})
badEmptySimultaneous ItIsSucc impossible

||| "Exile target creature and destroy that card."
public export
badSimultaneousReadsRetag : Unspellable (Instruction []) (\ok =>
  Simultaneously [Macros.exile (Macros.target Macros.creature),
                  Macros.destroy (Macros.That CardW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badSimultaneousReadsRetag (_, Oh) impossible

||| "Deal 2 damage to target creature. Then tap it."
public export
okTapAfterSimultaneousDamage : Instruction []
okTapAfterSimultaneousDamage =
  Sequentially [ Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature)]
               , Macros.tap (Macros.It) ]

||| "Exile target creature. Then tap it." A simultaneous move announces the
||| object in the exile zone [CR#701.13a] and only untapped permanents can be
||| tapped [CR#701.26a]; `okTapAfterSimultaneousDamage` spells the admitted read.
public export
badTapAfterSimultaneousExile : Unspellable (Instruction []) (\ok =>
  Sequentially [ Simultaneously [Macros.exile (Macros.target Macros.creature)]
               , Macros.tap ((Macros.It)) {ok} ])
badTapAfterSimultaneousExile Oh impossible

||| The same sentence with the exile as its own clause.
public export
badTapAfterSequentialExile : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.exile (Macros.target Macros.creature)
               , Macros.tap ((Macros.It)) {ok} ])
badTapAfterSequentialExile Oh impossible

||| "Gain control of target creature."
public export
okGainControlBattlefield : Instruction []
okGainControlBattlefield =
  Macros.gainControl You (Macros.target Macros.creature) Nothing

||| "Gain control of target creature card in a graveyard."
public export
badGainControlGraveyard : Unspellable (Instruction []) (\ok =>
  Macros.gainControl You (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) Nothing {zn = ok})
badGainControlGraveyard Oh impossible

||| "Reveal the top four cards. Put them on the bottom in any order."
public export
okPluralOrderRider : Instruction []
okPluralOrderRider =
  Sequentially [Macros.revealCards (Macros.topSlice (Lit 4)),
                Move (Macros.Those CardW) (Macros.onBottomIn AnyOrder) []]

public export
badSingularOrderRider : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topSlice (Lit 1)), Move (Macros.That CardW) (Macros.onBottomIn AnyOrder) [] {arr = ok}])
badSingularOrderRider Oh impossible

||| "Look at the top four cards of your library. Put them into your hand."
public export
okSliceCardRead : Instruction []
okSliceCardRead =
  Sequentially [Macros.lookAt (Macros.topSlice (Lit 4)),
                Move (Macros.Those CardW) Macros.handZ []]

public export
badSliceTypeRead : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.lookAt ((Macros.topSlice (Lit 4))), Move (Macros.Those (TypeW Creature) {ok}) Macros.handZ []])
badSliceTypeRead Refl impossible

||| "Search your library for a creature card."
public export
okSearchZoneFreeDescription : Instruction []
okSearchZoneFreeDescription = Macros.searchLibraryFor (Macros.exactly 1) Macros.creature

||| "If you would search your library for a creature card, instead search your
||| library for a creature card and reveal that card." [CR#614.1a]
public export
okInsteadOfSearchRevealsIt : Instruction []
okInsteadOfSearchRevealsIt =
  InsteadOf (Macros.searchLibraryFor (Macros.exactly 1) Macros.creature) Macros.revealsIt

||| "Search your library for a creature card in a graveyard."
public export
badSearchZonedDescription : Unspellable (Instruction []) (\ok =>
  Macros.searchLibraryFor (Macros.exactly 1) (And [Macros.creature, InZone Macros.graveyardZ]) {zf = ok})
badSearchZonedDescription Refl impossible

||| "Each player mills a card. Exile it."
public export
badDistributedMillSingular : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.mills (Macros.each AnyPlayer) (Lit 1) (Macros.each AnyPlayer)
               , Macros.exile ((Macros.It) {ok}) ])
badDistributedMillSingular Refl impossible

||| "one of up to two target creatures"
public export
okPartitiveOfTargetGroup : Instruction []
okPartitiveOfTargetGroup =
  Macros.exile (SomeOf (CountedSlice (Macros.exactly 1)) Nothing
                           (Described (TargetDet (Macros.upTo 2)) Macros.creature))

||| "one of a creature you control"
public export
badPartitiveOfDescription : Unspellable (Instruction []) (\ok =>
  Macros.exile (SomeOf (CountedSlice (Macros.exactly 1)) Nothing (Macros.a Macros.creature) {gm = ok}))
badPartitiveOfDescription Oh impossible

||| "Look at the top four cards of your library. Exile one of them."
public export
okPartitiveOfThem : Instruction []
okPartitiveOfThem =
  Sequentially [Macros.lookAt (Macros.topSlice (Lit 4)),
                Macros.exile (Macros.someOf (Macros.exactly 1) (Macros.Them))]

||| "two of one of them"
public export
badPartitiveOfPartitive : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.lookAt ((Macros.topSlice (Lit 4))), Macros.exile (Macros.someOf (Macros.exactly 1) (Macros.someOf (Macros.exactly 1) ((Macros.Them))) {gm = ok})])
badPartitiveOfPartitive Oh impossible

||| "each of the rest"
public export
badEachOfTheRest : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.lookAt ((Macros.topSlice (Lit 4)))
               , Move (Macros.someOf (Macros.exactly 1) ((Macros.Them))) Macros.handZ []
               , Macros.exile (EachOf (Macros.theRest Object) {gm = ok})
               ])
badEachOfTheRest Oh impossible

||| "If a creature you control would die this turn, exile it instead."
public export
okWouldDieOnBattlefield : Instruction []
okWouldDieOnBattlefield =
  Macros.ifWouldInstead (Dies (Macros.target Macros.creatureYouControl))
                 (Macros.exile (Macros.It)) (Just ThisTurn)

public export
badWouldDieInGraveyard : Unspellable (Instruction []) (\ok =>
  Macros.ifWouldInstead (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok})
                 (Macros.exile ((Macros.It))) (Just ThisTurn))
badWouldDieInGraveyard Oh impossible

||| "Reveal the top card of your library. Put that card into your hand."
public export
okSingularCardRetag : Instruction []
okSingularCardRetag =
  Sequentially [ Macros.revealCards (Macros.topSlice (Lit 1))
               , Move (Macros.That CardW) Macros.handZ []
               ]

||| "Exile target creature until this creature leaves the battlefield. Put
||| that card into its owner's hand." The held clause announces the exiled
||| object where it now is [CR#701.13a], so the card read resolves.
public export
okHeldUntilExileRetag : Instruction []
okHeldUntilExileRetag =
  Sequentially [ Macros.exileUntil (Macros.target Macros.creature) (Macros.leavesBattlefield Macros.thisCreature)
               , Move (Macros.That CardW) Macros.handZ []
               ]

||| "Put target creature onto the battlefield tapped."
public export
okMoveRidersToBattlefield : Instruction []
okMoveRidersToBattlefield =
  Move (Macros.target Macros.creature) Macros.battlefieldZ [EntersTapped]

||| "Put target creature into its owner's graveyard tapped."
public export
badMoveRidersToGraveyard : Unspellable (Instruction []) (\ok =>
  Move (Macros.target Macros.creature) Macros.graveyardZ [EntersTapped] {rf = ok})
badMoveRidersToGraveyard Oh impossible

||| "Put target creature into its owner's hand under your control."
public export
badMoveControlToHand : Unspellable (Instruction []) (\ok =>
  Move (Macros.target Macros.creature) Macros.handZ [Under You] {rf = ok})
badMoveControlToHand Oh impossible

||| "Put target creature onto the battlefield under your control."
public export
okMoveRidersSingularController : Instruction []
okMoveRidersSingularController =
  Move (Macros.target Macros.creature) Macros.battlefieldZ [Under You]

||| "Put target creature onto the battlefield under the other players' control."
public export
badMoveRidersPluralController : Unspellable (Instruction []) (\ok =>
  Move (Macros.target Macros.creature) Macros.battlefieldZ [Under (Macros.allOf Macros.otherPlayer) {one = ok}])
badMoveRidersPluralController OneController impossible

||| "a card exiled with this permanent"
public export
okExiledWithThis : Predicate [] Object
okExiledWithThis = ExiledWith This

||| "cards exiled with target creature"
public export
badExiledWithOtherSource : Unspellable (Predicate [] Object) (\ok =>
  ExiledWith (Macros.target Macros.creature) {ls = ok})
badExiledWithOtherSource SelfLinked impossible

||| "a card exiled with an artifact"
public export
badExiledWithDescribedSource : Unspellable (Predicate [] Object) (\ok =>
  ExiledWith (Macros.a Macros.artifact) {ls = ok})
badExiledWithDescribedSource SelfLinked impossible

||| "a creature you control"
public export
okZoneCoherentConjunction : Predicate [] Object
okZoneCoherentConjunction = And [Macros.creature, HasPossessor ControllerAx You]

||| "a card you control exiled with this artifact"
public export
badExiledWithControlled : Unspellable (Predicate [] Object) (\ok =>
  And [HasPossessor ControllerAx You, Macros.exiledWithThisArtifact] {zc = ok})
badExiledWithControlled Oh impossible

||| "an attacking creature"
public export
okAttackingCreature : Predicate [] Object
okAttackingCreature = And [Attacking, Macros.creature]

||| "an attacking creature exiled with this creature"
public export
badExiledWithAttacking : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, Macros.exiledWithThisArtifact] {zc = ok})
badExiledWithAttacking Oh impossible

||| "Put target creature card from a graveyard onto the battlefield tapped."
public export
okCreatureOntoBattlefieldTapped : Instruction []
okCreatureOntoBattlefieldTapped =
  Macros.putOntoBattlefieldTapped
    (Macros.target (And [Macros.creature, InZone Macros.graveyardZ]))

||| "Put target instant card from a graveyard onto the battlefield tapped."
public export
badInstantOntoBattlefield : Unspellable (Instruction []) (\ok =>
  Macros.putOntoBattlefieldTapped (Macros.target (And [HasType Instant, InZone Macros.graveyardZ])) {pl = ok})
badInstantOntoBattlefield Oh impossible

||| "Remove target creature from combat."
public export
okRemoveFromCombatOnBattlefield : Instruction []
okRemoveFromCombatOnBattlefield =
  RemoveFromCombat (Macros.target Macros.creature)

||| "Remove target creature card in your graveyard from combat."
public export
badRemoveFromCombatGraveyard : Unspellable (Instruction []) (\ok =>
  RemoveFromCombat (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok = ok})
badRemoveFromCombatGraveyard Oh impossible

||| "Players can't untap more than one creature during each untap step."
public export
okUntapCapBattlefieldSet : Ability
okUntapCapBattlefieldSet =
  Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1
                              Macros.creature)

public export
badUntapCapGraveyardSet : Unspellable Ability (\ok =>
  Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1
                              (And [Macros.creature, InZone (Macros.graveyardOf You)]) {pt = ok}))
badUntapCapGraveyardSet Oh impossible

||| "if this creature is attacking"
public export
okMatchesBattlefieldZone : Ability
okMatchesBattlefieldZone =
  Triggered When (Macros.lastCounterRemoved (NamedCounter "Time") Macros.thisCreature)
            [] Nothing [] Nothing Nothing
            (Just (Matches Macros.thisCreature Attacking))
            (Draw You (Lit 1))

public export
badExileCheckOnSortedSelf : Unspellable Ability (\ok =>
  Triggered When (Macros.lastCounterRemoved (NamedCounter "Time") Macros.thisCreature) [] Nothing [] Nothing Nothing (Just (Matches Macros.thisCreature (InZone Macros.exileZ)
                                         {zc = ok})) (Draw You (Lit 1)))
badExileCheckOnSortedSelf Oh impossible

||| "Draw cards equal to the difference." under a comparison
public export
okDifferenceAfterComparison : Instruction []
okDifferenceAfterComparison =
  If (CompareAmt (Macros.countOf (InZone (Macros.handOf You))) Less (Lit 7))
     (Draw You TheDifference) Nothing

||| "Draw cards equal to the difference."
public export
badUnlicensedDifference : Unspellable (Instruction []) (\ok =>
  Draw You (TheDifference {ok}))
badUnlicensedDifference Refl impossible

||| "When this creature enters, if you control fewer than seven creatures,
||| draw cards equal to the difference."
public export
okDifferenceUnderComparisonTrigger : Ability
okDifferenceUnderComparisonTrigger =
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing
            (Just (CompareAmt (Macros.countOf Macros.creatureYouControl) Less (Lit 7)))
            (Draw You TheDifference)

public export
badNonComparisonDifference : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Macros.exists Macros.creatureYouControl))
            (Draw You (TheDifference {ok})))
badNonComparisonDifference Refl impossible

||| "Goad target creature."
public export
okGoadOnBattlefield : Instruction []
okGoadOnBattlefield =
  GainsDesignation (Macros.target Macros.creature) Goaded Instructed Nothing

||| "goad target creature card in your graveyard"
public export
badGoadInGraveyard : Unspellable (Instruction []) (\ok =>
  GainsDesignation (Macros.target (And [Macros.creature,
                                        InZone (Macros.graveyardOf You)]))
                   Goaded Instructed Nothing {zn = ok})
badGoadInGraveyard (HolderOnField {ok = Oh}) impossible

||| "Whenever a creature attacks a planeswalker, …"
public export
okPlaneswalkerAttackDefender : GameEvent []
okPlaneswalkerAttackDefender =
  Attacks (Macros.a Macros.creature)
          (OneDefender (Macros.a (HasType Planeswalker)))

||| "Whenever a creature attacks another creature, …"
public export
badCreatureAttackDefender : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (Macros.a Macros.creature) {at = ok}))
badCreatureAttackDefender Oh impossible

||| "Look at the top four cards of your library. An opponent chooses one of
||| them."
public export
okAgentChoosesSomeOf : Instruction []
okAgentChoosesSomeOf =
  Sequentially [ Macros.lookAt (Macros.topSlice (Lit 4))
               , Choose Nothing (Just (Macros.a Opponent))
                        (Macros.someOf (Macros.exactly 1) (Macros.Them)) Openly Nothing ]

public export
badAgentChooseTheRest : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.lookAt ((Macros.topSlice (Lit 4)))
               , Move (Macros.someOf (Macros.exactly 1) ((Macros.Them))) Macros.handZ []
               , Choose Nothing (Just (Macros.a Opponent)) (Macros.theRest Object) Openly Nothing {ch = ok} ])
badAgentChooseTheRest Oh impossible

||| "… two cards with the same name in your hand …"
public export
okPluralNameAgreement : Noun [] Object
okPluralNameAgreement =
  NamesAgree SameName (Macros.counted (Macros.exactly 2)
                                      (And [Not Macros.land, InZone Macros.handZ]))

||| "… target creature with different names."
public export
badSingularNameAgreement : Unspellable (Noun [] Object) (\ok =>
  NamesAgree DifferentNames (Macros.target Macros.creature) {pl = ok})
badSingularNameAgreement Refl impossible

||| "This spell can't be countered."
public export
okCantBeCountered : StaticSpec []
okCantBeCountered = Macros.objectCant "Counter" This

||| "Creature cards in graveyards can't be countered."
public export
badCounteredInGraveyard : Unspellable (StaticSpec []) (\ok =>
  Macros.objectCant "Counter"
    (Macros.allOf (And [Macros.creature, InZone Macros.graveyardZ])) {dp = ok})
badCounteredInGraveyard Oh impossible

||| "You may look at the top card of your library any time."
public export
okLookAtTopOfLibrary : StaticSpec []
okLookAtTopOfLibrary = Visibility LookAt You TopOfLibrary

||| "You may look at your hand any time."
public export
badLookAtHandRider : Unspellable (StaticSpec []) (\ok =>
  Visibility LookAt You WholeHand {vo = ok})
badLookAtHandRider Oh impossible

||| "Lands you control are every creature type.": an object can't gain a
||| subtype that corresponds to none of its card types [CR#205.3d].
public export
badEveryCreatureTypeOnLand : Unspellable (StaticSpec []) (\ok =>
  Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Adds (EveryTypeOf CreatureSpace)
                {ok = ok})
badEveryCreatureTypeOnLand Oh impossible

||| "Creatures you control are every basic land type.": an object can't gain
||| a subtype that corresponds to none of its card types [CR#205.3d].
public export
badEveryBasicLandTypeOnCreature : Unspellable (StaticSpec []) (\ok =>
  Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (EveryTypeOf BasicLandSpace)
                {ok = ok})
badEveryBasicLandTypeOnCreature Oh impossible

||| "Regenerate target creature."
public export
okRegenerateCreature : Instruction []
okRegenerateCreature = Regenerate (Macros.target Macros.creature)

||| "Regenerate target creature card in your graveyard."
public export
badRegenerateInGraveyard : Unspellable (Instruction []) (\ok =>
  Regenerate (Macros.a (And [Macros.creature, InZone Macros.graveyardZ]))
             {zn = ok})
badRegenerateInGraveyard Oh impossible

public export
badRegenerateBareThis : Unspellable (Instruction []) (\ok =>
  Regenerate This {zn = ok})
badRegenerateBareThis Oh impossible

||| "Creatures can't be regenerated."
public export
okRegenerationBanOnBattlefield : StaticSpec []
okRegenerationBanOnBattlefield =
  Macros.objectCant "Regenerate" (Macros.allOf Macros.creature)

||| "Creature cards in your graveyard can't be regenerated."
public export
badRegeneratedInGraveyard : Unspellable (StaticSpec []) (\ok =>
  Macros.objectCant "Regenerate"
                    (Macros.a (And [Macros.creature, InZone Macros.graveyardZ]))
                    {dp = ok})
badRegeneratedInGraveyard Oh impossible

||| "Each opponent discards a card. Simultaneously, exile those cards."
public export
distributedDeedReadsBackPluralUnderAnnouncement : Instruction []
distributedDeedReadsBackPluralUnderAnnouncement =
  Simultaneously [ Macros.discard (Macros.each Opponent) (Macros.a (InZone Macros.handZ))
                 , Macros.exile (Macros.Those CardW) ]

||| "Discard a card. Simultaneously, exile that card."
public export
okThatAfterSingularDiscard : Instruction []
okThatAfterSingularDiscard =
  Simultaneously [ Macros.discard You (Macros.a (InZone Macros.handZ))
                 , Macros.exile (Macros.That CardW) ]

||| "Each opponent discards a card. Simultaneously, exile that card."
public export
badDistributedAnnouncedDiscardSingular : Unspellable (Instruction []) (\ok =>
  Simultaneously [ Macros.discard (Macros.each Opponent) (Macros.a (InZone Macros.handZ))
                 , Macros.exile (Macros.That CardW {ok}) ])
badDistributedAnnouncedDiscardSingular Refl impossible

||| "Draw a card. You gain 1 life."
public export
okNonEmptySequence : Instruction []
okNonEmptySequence =
  Sequentially [Draw You (Lit 1), Macros.gainsLife You (Lit 1)]

||| an empty sentence list
public export
badEmptySequence : Unspellable (Instruction []) (\ok =>
  Sequentially [] {ne = ok})
badEmptySequence ItIsSucc impossible

||| "Reveal your hand."
public export
okRevealHand : Instruction []
okRevealHand = Expose Reveal You (ExposedZone Macros.handZ)

||| "Reveal your graveyard."
public export
badRevealGraveyard : Unspellable (Instruction []) (\ok =>
  Expose Reveal You (ExposedZone Macros.graveyardZ {ok}))
badRevealGraveyard Oh impossible

||| "Sacrifice a creature."
public export
okSacrificeBattlefieldNoun : Instruction []
okSacrificeBattlefieldNoun = Macros.sacrifice You (Macros.a Macros.creature)

public export
badInterceptReplacementAntecedent : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.nextTimeWouldInstead (Draws You)
                                      (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Soldier"]))
                                      (Just ThisTurn)
               , Macros.sacrifice You ((Macros.It) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}
               ])
badInterceptReplacementAntecedent (Refl, _) impossible

||| "If you would draw a card, draw two cards instead."
public export
okFlatInstead : Instruction []
okFlatInstead =
  InsteadOf (Draw You (Lit 1)) (Draw You (Lit 2))

public export
badNestedInstead : Unspellable (Instruction []) (\ok =>
  InsteadOf (InsteadOf (Draw You (Lit 1)) ((Draw You (Lit 2)))) ((Draw You (Lit 3))) {na = ok})
badNestedInstead Oh impossible

||| "Untap target creature."
public export
okUntapInstruction : Instruction []
okUntapInstruction = SetStatus Untapped (Macros.target Macros.creature)

||| "Unflip target creature."
public export
badUnflipInstruction : Unspellable (Instruction []) (\ok =>
  SetStatus Unflipped (Macros.target Macros.creature) {at = ok})
badUnflipInstruction Oh impossible

||| "… if a creature died this turn, …"
public export
okDeathLookback : Condition []
okDeathLookback =
  Happened (Macros.a Macros.creature) (MkLookback Death ThisTurn Nothing)

||| "… if a creature was put this turn, …"
public export
badPlacementLookback : Unspellable (Condition []) (\ok =>
  Happened (Macros.a Macros.creature) (MkLookback Placement ThisTurn Nothing {cw = LeftBare {ok = ok}}))
badPlacementLookback Oh impossible

||| "Counter target activated ability."
public export
okCounterAbility : Instruction []
okCounterAbility = CounterSpell (Macros.target (AbilityHead AnyActivated))

||| "Exile target creature."
public export
okObjectMovedToAZone : Instruction []
okObjectMovedToAZone =
  Move (Macros.target Macros.creature) Macros.exileZ []

||| "Exile target activated ability." -- an ability on the stack is an object
||| [CR#113.1c] that ceases to exist when it leaves the stack [CR#608.2n];
||| it never changes zones.
public export
badAbilityMovedToAZone : Unspellable (Instruction []) (\ok =>
  Move (Macros.target (AbilityHead AnyActivated)) Macros.exileZ []
       {mk = ObjectMoves {nb = ok}})
badAbilityMovedToAZone PayloadIsObject impossible

||| "Exchange control of target artifact and target creature."
public export
okExchangeControlOnField : Instruction []
okExchangeControlOnField =
  Exchange (ControlOf (Macros.target Macros.artifact)
                      (Macros.target Macros.creature))

||| "Exchange control of target artifact and target creature card in your
||| graveyard." A card that isn't a permanent or a spell has no controller
||| [CR#108.4], so there is nothing to exchange [CR#701.12b].
public export
badExchangeControlInGraveyard : Unspellable (Instruction []) (\ok =>
  Exchange (ControlOf (Macros.target Macros.artifact)
                      (Macros.target (And [Macros.creature,
                                           InZone (Macros.graveyardOf You)]))
                      {bz = ok}))
badExchangeControlInGraveyard Oh impossible

||| "Exchange this Aura with an Aura card in your hand." (aura swap,
||| [CR#702.65a])
public export
okExchangeCardsAcrossZones : Instruction []
okExchangeCardsAcrossZones =
  Exchange (CardsAcross Macros.thisAura
              (Macros.a (And [HasSubtype (enchantmentType "Aura"),
                              InZone Macros.handZ])))

||| "Exchange a card in your hand with a card in your hand." Cards are
||| exchanged with cards in a DIFFERENT zone [CR#701.12d].
public export
badExchangeCardsSameZone : Unspellable (Instruction []) (\ok =>
  Exchange (CardsAcross (Macros.a (InZone Macros.handZ))
                        (Macros.a (InZone Macros.handZ)) {dz = ok}))
badExchangeCardsSameZone Oh impossible

||| "Exchange your hand and graveyard."
public export
okExchangeTwoZones : Instruction []
okExchangeTwoZones = Exchange (Zones Macros.handZ Macros.graveyardZ)

||| "Exchange your hand and your hand." An exchange of two zones puts each
||| zone's cards in the other zone [CR#701.12d,701.12f].
public export
badExchangeZoneWithItself : Unspellable (Instruction []) (\ok =>
  Exchange (Zones Macros.handZ Macros.handZ {zs = ok}))
badExchangeZoneWithItself Oh impossible

||| "Exchange your hand and the battlefield." Only cards are exchanged between
||| zones this way [CR#701.12d]; the battlefield holds permanents [CR#110.1].
public export
badExchangeBattlefieldZone : Unspellable (Instruction []) (\ok =>
  Exchange (Zones Macros.handZ Macros.battlefieldZ {zs = ok}))
badExchangeBattlefieldZone Oh impossible

||| "the exiled card" — the definite reference is licensed by the link to the
||| exiling ability printed on the same object [CR#607.2a], and still resolves
||| when that ability exiled more than one card [CR#607.3].
public export
okTheExiledCard : Noun [] Object
okTheExiledCard = Macros.the (ExiledWith This)

||| "the card in exile" — the exile zone holds any number of cards, and nothing
||| links the reference to one of them.
public export
badTheCardInExile : Unspellable (Noun [] Object) (\ok =>
  Macros.the (InZone Macros.exileZ) {ok})
badTheCardInExile Oh impossible

||| "Exchange your life total with this creature's toughness." (Tree of
||| Redemption) — two settable values [CR#701.12g].
public export
okExchangeTwoValues : Instruction []
okExchangeTwoValues =
  Exchange (Values (PlayerStatOf LifeTotal You)
                   (StatOf Toughness Macros.thisCreature))

||| "Exchange this creature's power with this creature's power." — each value
||| would become equal to its own previous value [CR#701.12g].
public export
badExchangeValueWithItself : Unspellable (Instruction []) (\ok =>
  Exchange (Values (StatOf Power Macros.thisCreature)
                   (StatOf Power Macros.thisCreature) {nv = ok}))
badExchangeValueWithItself Oh impossible

||| "Exchange this creature's power with three." — a literal is not a value the
||| game can set [CR#701.12g].
public export
badExchangeLiteralValue : Unspellable (Instruction []) (\ok =>
  Exchange (Values (StatOf Power Macros.thisCreature) (Lit 3) {vb = ok}))
badExchangeLiteralValue Oh impossible

||| "Exchange the text boxes of this creature and another creature."
||| [CR#701.12h]
public export
okExchangeTextBoxes : Instruction []
okExchangeTextBoxes =
  Exchange (TextBoxes Macros.thisCreature
              (Macros.a (And [Macros.creature, OtherThan This])))

||| "Exchange the text boxes of this creature and a creature card in your
||| graveyard." — a text box is exchanged between permanents on the battlefield.
public export
badExchangeTextBoxInGraveyard : Unspellable (Instruction []) (\ok =>
  Exchange (TextBoxes Macros.thisCreature
              (Macros.a (And [Macros.creature, InZone (Macros.graveyardOf You)]))
              {bz = ok}))
badExchangeTextBoxInGraveyard Oh impossible
