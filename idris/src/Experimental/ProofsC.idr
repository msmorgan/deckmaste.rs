module Experimental.ProofsC

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Create a 1/1 green Plant creature token. Put a +1/+1 counter on it."
public export
okCreatedThenCountered : Effect []
okCreatedThenCountered =
  Sequentially [Create You (Lit 1)
                       (TokenWritten
                          (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])) [],
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Macros.It OneOf)]


public export
badDistributedCreationIt : Unspellable (Effect []) (\ok =>
  Sequentially [Create (Macros.each AnyPlayer) (Lit 1)
                       (TokenWritten (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])) [],
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf) {ok})])
badDistributedCreationIt Refl impossible


||| "each of up to two target creatures"
public export
okEachOfTargetGroup : Noun [] Object
okEachOfTargetGroup = EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature)


||| "each of each creature"
public export
badEachOfDistributive : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.each Macros.creature) {gm = ok})
badEachOfDistributive Oh impossible


||| "each of all creatures"
public export
badEachOfAll : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.allOf Macros.creature) {gm = ok})
badEachOfAll Oh impossible


||| "each of each of up to two target creatures"
public export
badNestedEachOf : Unspellable (Noun [] Object) (\ok =>
  EachOf (EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature)) {gm = ok})
badNestedEachOf Oh impossible


||| "Put a +1/+1 counter on each of up to two target creatures."
public export
okDistributedCounterRecipient : Effect []
okDistributedCounterRecipient =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
              (EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature))


||| "Put a +1/+1 counter on up to two target creatures."
public export
badBarePluralCounterRecipient : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Described (TargetDet (Macros.upTo 2)) Macros.creature) {pm = ok})
badBarePluralCounterRecipient Oh impossible


||| "This deals 1 damage to each of up to two target creatures."
public export
okDistributedDamageRecipient : Effect []
okDistributedDamageRecipient =
  DealDamage This (Lit 1)
             (EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature))


||| "This deals 1 damage to up to two target creatures."
public export
badBarePluralDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Described (TargetDet (Macros.upTo 2)) Macros.creature) {pm = ok})
badBarePluralDamageRecipient Oh impossible


||| "Choose any number of target creatures. Put a +1/+1 counter on them."
public export
badThemCounterRecipient : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (Described (TargetDet Macros.anyNumber) Macros.creature) Nothing Openly,
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) ((Macros.It ManyOf)) {pm = ok}])
badThemCounterRecipient Oh impossible


||| "This deals 2 damage divided as you choose among two target creatures."
public export
okDivideAmongTargets : Effect []
okDivideAmongTargets =
  Macros.dealsDivided This (Lit 2)
                      (Described (TargetDet (Macros.oneThrough 2)) Macros.creature)


||| "This deals 2 damage divided as you choose among each creature."
public export
badDivideAmongDescription : Unspellable (Effect []) (\ok =>
  Macros.dealsDivided This (Lit 2) (Macros.each Macros.creature) {gm = ok})
badDivideAmongDescription Oh impossible


||| "Put target creature onto the battlefield."
public export
okMoveToBattlefield : Effect []
okMoveToBattlefield =
  Move (Macros.target Macros.creature) (ZoneAt Battlefield Bare) []


||| "Put target creature into your library."
public export
badMoveToBareLibrary : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) (ZoneAt Library Bare) [] {ok})
badMoveToBareLibrary BattlefieldOk impossible


||| "Reveal the top four cards. Put them on the bottom in any order."
public export
okPluralOrderRider : Effect []
okPluralOrderRider =
  Sequentially [Macros.revealCards (Macros.topSlice (Lit 4)),
                Move (Macros.That CardW ManyOf) (Macros.onBottomIn AnyOrder) []]


public export
badSingularOrderRider : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topSlice (Lit 1)), Move (Macros.That CardW OneOf) (Macros.onBottomIn AnyOrder) [] {arr = ok}])
badSingularOrderRider Oh impossible


||| "Put one of them into your hand and the rest into your graveyard."
public export
okRestAfterPart : Effect []
okRestAfterPart =
  Sequentially [ Macros.lookAt (Macros.topSlice (Lit 4))
               , Move (Macros.someOf (Macros.exactly 1) (Macros.It ManyOf)) Macros.handZ []
               , Move Macros.theRest Macros.graveyardZ []
               ]


||| "Put the rest into your graveyard."
public export
badRestWithoutGroup : Unspellable (Effect []) (\ok =>
  Move (Macros.theRest {ok}) Macros.graveyardZ [])
badRestWithoutGroup Oh impossible


||| "Look at the top four cards of your library. Put the rest on the bottom."
public export
badRestWithoutPart : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt ((Macros.topSlice (Lit 4))), Move (Macros.theRest {ok}) Macros.onBottomZ []])
badRestWithoutPart Oh impossible


public export
badRestDisposedTwice : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt ((Macros.topSlice (Lit 4)))
               , Move (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))) Macros.handZ []
               , Move Macros.theRest Macros.onBottomZ []
               , Move (Macros.theRest {ok}) Macros.graveyardZ []
               ])
badRestDisposedTwice Oh impossible


public export
badRestOverTwoAnnouncements : Unspellable (Effect []) (\ok =>
  Sequentially [ Fights (Macros.target Macros.creatureYouControl) (Macros.target Macros.creatureYouDontControl)
               , Move (Macros.theRest {ok}) Macros.graveyardZ []
               ])
badRestOverTwoAnnouncements Oh impossible


||| "Look at the top four cards of your library. You choose one of them."
public export
okAgentChoiceOfSome : Effect []
okAgentChoiceOfSome =
  Sequentially [Macros.lookAt (Macros.topSlice (Lit 4)),
                Choose (Macros.someOf (Macros.exactly 1) (Macros.It ManyOf)) (Just You) Openly]


||| "Look at the top four cards of your library. Choose one of them."
public export
badChooseSomeOf : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt ((Macros.topSlice (Lit 4))), Choose (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))) Nothing Openly {ch = ok}])
badChooseSomeOf BareChoice impossible


||| "Tap target creature."
public export
okTapBattlefieldPermanent : Effect []
okTapBattlefieldPermanent = SetStatus Tapped (Macros.target Macros.creature)


||| "Tap the top card of your library."
public export
badTapLibraryTop : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (Macros.topSlice (Lit 1)) {ok})
badTapLibraryTop Oh impossible


||| "Look at the top four cards of your library. Put them into your hand."
public export
okSliceCardRead : Effect []
okSliceCardRead =
  Sequentially [Macros.lookAt (Macros.topSlice (Lit 4)),
                Move (Macros.That CardW ManyOf) Macros.handZ []]


public export
badSliceTypeRead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt ((Macros.topSlice (Lit 4))), Move (Macros.That (TypeW Creature) ManyOf {ok}) Macros.handZ []])
badSliceTypeRead Refl impossible


||| "Reveal your hand."
public export
okRevealHand : Effect []
okRevealHand = Expose Reveal You (ExposedZone Macros.handZ)


||| "Reveal your graveyard."
public export
badRevealGraveyard : Unspellable (Effect []) (\ok =>
  Expose Reveal You (ExposedZone Macros.graveyardZ {ok}))
badRevealGraveyard Oh impossible


||| "Search your library for a creature card."
public export
okSearchZoneFreeDescription : Effect []
okSearchZoneFreeDescription = Macros.searchLibraryFor (Macros.exactly 1) Macros.creature


||| "Search your library for a creature card in a graveyard."
public export
badSearchZonedDescription : Unspellable (Effect []) (\ok =>
  Macros.searchLibraryFor (Macros.exactly 1) (And [Macros.creature, InZone Macros.graveyardZ]) {zf = ok})
badSearchZonedDescription Refl impossible


||| "Each player mills a card. Exile it."
public export
badDistributedMillSingular : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.mills (Macros.each AnyPlayer) (Lit 1) (Macros.each AnyPlayer)
               , Macros.exile You ((Macros.It OneOf) {ok}) ])
badDistributedMillSingular Refl impossible


||| "one of up to two target creatures"
public export
okPartitiveOfTargetGroup : Effect []
okPartitiveOfTargetGroup =
  Macros.exile You (SomeOf (CountedSlice (Macros.exactly 1)) Nothing
                           (Described (TargetDet (Macros.upTo 2)) Macros.creature))


||| "one of a creature you control"
public export
badPartitiveOfDescription : Unspellable (Effect []) (\ok =>
  Macros.exile You (SomeOf (CountedSlice (Macros.exactly 1)) Nothing (Macros.a Macros.creature) {gm = ok}))
badPartitiveOfDescription Oh impossible


||| "Look at the top four cards of your library. Exile one of them."
public export
okPartitiveOfThem : Effect []
okPartitiveOfThem =
  Sequentially [Macros.lookAt (Macros.topSlice (Lit 4)),
                Macros.exile You (Macros.someOf (Macros.exactly 1) (Macros.It ManyOf))]


||| "two of one of them"
public export
badPartitiveOfPartitive : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt ((Macros.topSlice (Lit 4))), Macros.exile You (Macros.someOf (Macros.exactly 1) (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))) {gm = ok})])
badPartitiveOfPartitive Oh impossible


||| "each of the rest"
public export
badEachOfTheRest : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt ((Macros.topSlice (Lit 4)))
               , Move (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))) Macros.handZ []
               , Macros.exile You (EachOf Macros.theRest {gm = ok})
               ])
badEachOfTheRest Oh impossible


||| "If a creature you control would die this turn, exile it instead."
public export
okWouldDieOnBattlefield : Effect []
okWouldDieOnBattlefield =
  Macros.ifWouldInstead (Dies (Macros.target Macros.creatureYouControl))
                 (Macros.exile You (Macros.It OneOf)) (Just ThisTurn)


public export
badWouldDieInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok})
                 (Macros.exile You ((Macros.It OneOf))) (Just ThisTurn))
badWouldDieInGraveyard Oh impossible


||| "Sacrifice a creature."
public export
okSacrificeBattlefieldNoun : Effect []
okSacrificeBattlefieldNoun = Macros.sacrifice You (Macros.a Macros.creature)


public export
badInterceptReplacementAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.nextTimeWouldInstead (Draws You)
                                      (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Soldier"]))
                                      (Just ThisTurn)
               , Macros.sacrifice You ((Macros.It OneOf) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}
               ])
badInterceptReplacementAntecedent (Refl, _) impossible


||| "Reveal the top card of your library. Put that card into your hand."
public export
okSingularCardRetag : Effect []
okSingularCardRetag =
  Sequentially [ Macros.revealCards (Macros.topSlice (Lit 1))
               , Move (Macros.That CardW OneOf) Macros.handZ []
               ]


public export
badHeldUntilExileRetag : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.exileUntil (Macros.target Macros.creature) (Macros.leavesBattlefield Macros.thisCreature)
               , Move (Macros.That CardW OneOf {ok}) Macros.handZ []
               ])
badHeldUntilExileRetag Refl impossible


||| "If you would draw a card, draw two cards instead."
public export
okFlatInstead : Effect []
okFlatInstead =
  InsteadOf (Draw You (Lit 1)) (Draw You (Lit 2))


public export
badNestedInstead : Unspellable (Effect []) (\ok =>
  InsteadOf (InsteadOf (Draw You (Lit 1)) ((Draw You (Lit 2)))) ((Draw You (Lit 3))) {na = ok})
badNestedInstead Oh impossible


||| "Sacrifice a creature: Draw a card."
public export
okSacrificeAsCost : Ability
okSacrificeAsCost =
  Activated (Do (Macros.sacrifice You (Macros.a Macros.creature)))
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing


||| "Sacrifice a creature, Exile the sacrificed card: Draw a card."
public export
badCostReadsSiblingDeed : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.sacrifice You (Macros.a Macros.creature)),
                       Do (Macros.exile You (Macros.TheVerbed "Sacrifice" CardW Attributive OneOf)) {ok}])
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badCostReadsSiblingDeed Oh impossible


||| "Pay 2 life: Draw a card."
public export
okOwnPayerCost : Ability
okOwnPayerCost =
  Activated (Macros.payLife You 2) (Draw You (Lit 1))
            Nothing Nothing Nothing Nothing


||| "An opponent pays 2 life: Draw a card."
public export
badForeignPayerCost : Unspellable Ability (\ok =>
  Activated (Macros.payLife Macros.anOpponent 2) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing {py = ok})
badForeignPayerCost Oh impossible


||| "An opponent sacrifices a creature: Draw a card."
public export
badForeignSacrificeCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice Macros.anOpponent (Macros.a Macros.creature))) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing {py = ok})
badForeignSacrificeCost Oh impossible


||| "you pay 1 life"
public export
okMatchedPayer : Effect []
okMatchedPayer =
  (May You (Pay You (Macros.payLife You 1) PaidOnce) Nothing
       (Just (Draw You (Lit 1))))


||| "you pay"
public export
badMismatchedPayer : Unspellable (Effect []) (\ok =>
  (May You (Pay You (Macros.payLife Macros.anOpponent 1) PaidOnce {ag = ok}) Nothing (Just (Draw You (Lit 1)))))
badMismatchedPayer Oh impossible


||| "You pay 2 life."
public export
okPayLifeCost : Effect []
okPayLifeCost = Pay You (Macros.payLife You 2) PaidOnce


||| "You pay {T}."
public export
badPayTapSymbol : Unspellable (Effect []) (\ok =>
  Pay You TapSymbol PaidOnce {pb = ok})
badPayTapSymbol Oh impossible


||| "Creatures you control get +1/+1 until end of turn:"
public export
badContinuousAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.gets (Macros.allOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1))
                             (Just Macros.untilEndOfTurn)) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badContinuousAsCost Oh impossible


||| a replacement written as a cost
public export
badInsteadAsCost : Unspellable Ability (\ok =>
  Activated (Do (InsteadOf (Macros.destroy (Macros.target Macros.creature))
                           (Macros.exile You (Macros.target Macros.creature))) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badInsteadAsCost Oh impossible


||| a delayed trigger written as a cost
public export
badDelayedAsCost : Unspellable Ability (\ok =>
  Activated (Do (Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (Draw You (Lit 1))) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badDelayedAsCost Oh impossible


||| an "until" rider written as a cost
public export
badHeldUntilAsCost : Unspellable Ability (\ok =>
  Activated (Do (HeldUntil (Macros.exile You (Macros.target Macros.creature))
                           (Dies (Macros.a Macros.creature))) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badHeldUntilAsCost Oh impossible


||| a reflexive trigger written as a cost
public export
badReflexiveAsCost : Unspellable Ability (\ok =>
  Activated (Do (Reflexively (Macros.gainsLife You (Lit 2)) (Draw You (Lit 1))) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badReflexiveAsCost Oh impossible


||| "You skip your next turn:"
public export
badSkipAsCost : Unspellable Ability (\ok =>
  Activated (Do (SkipsNext You Turn (Lit 1)) {ok}) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badSkipAsCost Oh impossible


||| "You pay 2 life:"
public export
badPayAsCost : Unspellable Ability (\ok =>
  Activated (Do (Pay You (Macros.payLife You 2) PaidOnce) {ok}) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badPayAsCost Oh impossible


||| "Discard a card, then sacrifice a creature:"
public export
badSequentialCost : Unspellable Ability (\ok =>
  Activated (Do (Sequentially [(Macros.discard You (Macros.a (InZone Macros.handZ))),
                               Macros.sacrifice You (Macros.a Macros.creature)]) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badSequentialCost Oh impossible


||| "Discard a card and sacrifice a creature simultaneously:"
public export
badSimultaneousCost : Unspellable Ability (\ok =>
  Activated (Do (Simultaneously [(Macros.discard You (Macros.a (InZone Macros.handZ))),
                                 Macros.sacrifice You (Macros.a Macros.creature)]) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badSimultaneousCost Oh impossible


||| "Repeat this process:"
public export
badRepeatAsCost : Unspellable Ability (\ok =>
  Activated (Do (Repeat Again) {ok}) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badRepeatAsCost Oh impossible


||| a compound cost of no components
public export
badEmptyCompound : Unspellable Ability (\ok =>
  Activated (Compound [] {ne = ok}) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badEmptyCompound ItIsSucc impossible


public export
nestedCompoundCost : Ability
nestedCompoundCost =
  Activated (Compound [Compound [Mana [Macros.generic 1], TapSymbol], TapSymbol])
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing


||| "{1}, {T}: Draw a card."
public export
okSingleTapCost : Ability
okSingleTapCost =
  Activated (Compound [Mana [Macros.generic 1], TapSymbol])
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing


||| "{T}, {T}: Draw a card."
public export
badDoubleTapCost : Unspellable Ability (\ok =>
  Activated (Compound [TapSymbol, TapSymbol]) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing {tp = ok})
badDoubleTapCost Oh impossible


||| "{1}: Draw a card."
public export
okNonEmptyManaCost : Ability
okNonEmptyManaCost =
  Activated (Mana [Macros.generic 1]) (Draw You (Lit 1))
            Nothing Nothing Nothing Nothing


||| ": Draw a card."
public export
badEmptyManaCost : Unspellable Ability (\ok =>
  Activated (Mana [] {wr = ok}) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badEmptyManaCost IsNonEmpty impossible


||| "{W/U/P}"
public export
okDistinctPhyrexian : ManaSymbol
okDistinctPhyrexian = Phyrexian White (Just Blue)


||| "{W/W/P}"
public export
badSameColorPhyrexian : Unspellable ManaSymbol (\ok =>
  Phyrexian White (Just White) {ds = ok})
badSameColorPhyrexian Oh impossible


||| "{U/B}"
public export
okDistinctHybrid : ManaSymbol
okDistinctHybrid = Macros.hybridPip Blue Black


||| "{U/U}"
public export
badSameColorHybrid : Unspellable ManaSymbol (\ok =>
  Macros.hybridPip Blue Blue {ds = ok})
badSameColorHybrid Oh impossible


||| "Sacrifice a creature. If you don't, exile it."
public export
badIfNotReadsMandatoryBody : Unspellable (Effect []) (\ok =>
  (IfDone (Macros.sacrifice You (Macros.a Macros.creature)) Nothing (Just (Macros.exile You ((Macros.It OneOf) {ok})))))
badIfNotReadsMandatoryBody Refl impossible


||| "Counter target spell unless its controller pays {3}."
public export
okUnlessManaCost : Effect []
okUnlessManaCost =
  Unless (CounterSpell (Macros.target Macros.spell))
         (Macros.controllerOf (Macros.It OneOf)) (Mana [Macros.generic 3])


||| "Counter target spell unless its controller taps."
public export
badUnlessTapSymbol : Unspellable (Effect []) (\ok =>
  Unless (CounterSpell (Macros.target Macros.spell)) (Macros.controllerOf ((Macros.It OneOf))) TapSymbol {pb = ok})
badUnlessTapSymbol Oh impossible


||| "Whenever a creature dies, draw a card."
public export
okNontargetDeathHeader : Ability
okNontargetDeathHeader =
  Triggered Whenever (Dies (Macros.a Macros.creature)) [] Nothing []
            Nothing Nothing Nothing (Draw You (Lit 1))


||| "Whenever target creature dies, draw a card."
public export
badTargetedDeathHeader : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.target Macros.creature)) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)) {hn = ok})
badTargetedDeathHeader Oh impossible


||| "At the beginning of your upkeep, draw a card."
public export
okTriggerAtYourUpkeep : Ability
okTriggerAtYourUpkeep =
  Triggered At (BeginningOf ThePart Upkeep (ByPlayer You)) [] Nothing []
            Nothing Nothing Nothing (Draw You (Lit 1))


||| "At the beginning of your turn, draw a card."
public export
badTriggerAtYourTurn : Unspellable Ability (\ok =>
  Triggered At (BeginningOf ThePart Turn (ByPlayer You) {pu = ok}) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badTriggerAtYourTurn Oh impossible


||| "Whenever a creature dies, tap it."
public export
badTriggerTapsDeadCreature : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.a Macros.creature)) [] Nothing [] Nothing Nothing Nothing (SetStatus Tapped ((Macros.It OneOf)) {ok}))
badTriggerTapsDeadCreature Oh impossible


||| "Whenever a creature leaves the battlefield, tap it."
public export
badLeavesThenTap : Unspellable Ability (\ok =>
  Triggered Whenever (Macros.leavesBattlefield (Macros.a Macros.creature)) [] Nothing [] Nothing Nothing Nothing (SetStatus Tapped ((Macros.It OneOf)) {ok}))
badLeavesThenTap Oh impossible


||| "Creatures can't attack."
public export
okStaticUntargeting : Ability
okStaticUntargeting =
  Static (Macros.deontic (Macros.allOf Macros.creature) Forbid ["Attack"] Agent
                         NoDeonticPatient)


||| "Target creature can't attack."
||| Refused as a Static; Continuously (deontic …) spells the sentence.
public export
badStaticTargets : Unspellable Ability (\ok =>
  Static (Macros.deontic (Macros.target Macros.creature) Forbid ["Attack"] Agent NoDeonticPatient) {ut = ok})
badStaticTargets Oh impossible


||| "You may play a card in your graveyard this turn."
public export
okPlayFromGraveyard : Effect []
okPlayFromGraveyard =
  Continuously {ts = StaticFirstDone}
               (Deontic You Permit ["Play"] Agent Nothing
                  (DeonticCounterpart (Macros.a (InZone Macros.graveyardZ)))
                  Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost))
               (Just ThisTurn)


||| "You may play a creature this turn."
public export
badPlayFromBattlefield : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Deontic You Permit ["Play"] Agent Nothing
                  (DeonticCounterpart (Macros.a Macros.creature)) Nothing
                  (PlayRider Nothing Nothing Nothing False ItsOwnCost) {rd = ok})
               (Just ThisTurn))
badPlayFromBattlefield Oh impossible


||| "Target creature gets +3/+3 until end of turn."
public export
okUntilEndOfTurnSpan : Effect []
okUntilEndOfTurnSpan =
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 3)) (PtUp (Lit 3))
              (Just Macros.untilEndOfTurn)


||| "Target creature gets +3/+3 until the beginning of your next upkeep."
||| Refused as UntilEvent; Macros.untilYourNextUpkeep spells the duration.
public export
badUntilBeginningOfUpkeep : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just (UntilEvent (BeginningOf ThePart Upkeep (ByPlayer You)))) {sp = ok})
badUntilBeginningOfUpkeep (Present {ok = Oh}) impossible


||| "Sacrifice a creature. When you do, draw a card."
public export
okReflexiveOnSacrifice : Effect []
okReflexiveOnSacrifice =
  Reflexively (Macros.sacrifice You (Macros.a Macros.creature))
              (Draw You (Lit 1))


||| "This creature deals 3 damage to any target. When you do, draw a card."
public export
badReflexiveOnSourceDeed : Unspellable (Effect []) (\ok =>
  Reflexively (DealDamage This (Lit 3) (Macros.target Macros.anyTarget)) (Draw You (Lit 1)) {en = ok})
badReflexiveOnSourceDeed Oh impossible


||| "You gain 2 life. When you do, draw a card."
public export
badReflexiveOnLifeGain : Unspellable (Effect []) (\ok =>
  Reflexively (Macros.gainsLife You (Lit 2)) (Draw You (Lit 1)) {en = ok})
badReflexiveOnLifeGain Oh impossible


||| "Draw a card, then sacrifice a creature. When you do, draw a card."
public export
badReflexiveOnSequence : Unspellable (Effect []) (\ok =>
  Reflexively (Sequentially [(Draw You (Lit 1)), Macros.sacrifice You (Macros.a Macros.creature)]) (Draw You (Lit 1)) {en = ok})
badReflexiveOnSequence Oh impossible


public export
badReflexiveOnDelayed : Unspellable (Effect []) (\ok =>
  Reflexively (Delayed (BeginningOf ThePart EndStep (ByPlayer You)) [] Nothing (Draw You (Lit 1))) (Draw You (Lit 1)) {en = ok})
badReflexiveOnDelayed Oh impossible


||| "Regenerate this creature. If it regenerates this way, draw a card."
public export
okThisWayOnRegenerate : Effect []
okThisWayOnRegenerate =
  ThisWay (Regenerate Macros.thisCreature) (Regenerates Macros.thisCreature)
          (Draw You (Lit 1))


public export
badThisWayOnDelayed : Unspellable (Effect []) (\ok =>
  ThisWay (Delayed (BeginningOf ThePart EndStep (ByPlayer You)) [] Nothing (Draw You (Lit 1)))
          (Draws You) (Draw You (Lit 1)) {oc = ok})
badThisWayOnDelayed Oh impossible


public export
badReflexiveOnBranchedMay : Unspellable (Effect []) (\ok =>
  Reflexively ((May You (Macros.sacrifice You (Macros.a Macros.creature)) (Just (Draw You (Lit 1))) Nothing)) (Draw You (Lit 1)) {en = ok})
badReflexiveOnBranchedMay Oh impossible


public export
badAfterReflexiveReadsTrigger : Unspellable (Effect []) (\ok =>
  Sequentially [Reflexively (Macros.mills You (Lit 4) You)
                            (Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [White]
                                                     (MkTypeLine [creatureType "Soldier"] [Creature])
                                                     [] Nothing)),
                SetStatus Tapped (Macros.That (TypeW Creature) OneOf {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badAfterReflexiveReadsTrigger (Refl, _) impossible


||| "Sacrifice a creature. When you do, tap it."
public export
badReflexiveTapsSacrificed : Unspellable (Effect []) (\ok =>
  Reflexively (Macros.sacrifice You (Macros.a Macros.creature)) (SetStatus Tapped ((Macros.It OneOf)) {ok}))
badReflexiveTapsSacrificed Oh impossible


||| "Put target creature onto the battlefield tapped."
public export
okMoveRidersToBattlefield : Effect []
okMoveRidersToBattlefield =
  Move (Macros.target Macros.creature) Macros.battlefieldZ [EntersTapped]


||| "Put target creature into its owner's graveyard tapped."
public export
badMoveRidersToGraveyard : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.graveyardZ [EntersTapped] {rf = ok})
badMoveRidersToGraveyard Oh impossible


||| "Put target creature into its owner's hand under your control."
public export
badMoveControlToHand : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.handZ [Under You] {rf = ok})
badMoveControlToHand Oh impossible


||| "Put target creature onto the battlefield under your control."
public export
okMoveRidersSingularController : Effect []
okMoveRidersSingularController =
  Move (Macros.target Macros.creature) Macros.battlefieldZ [Under You]


||| "Put target creature onto the battlefield under the other players' control."
public export
badMoveRidersPluralController : Unspellable (Effect []) (\ok =>
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
