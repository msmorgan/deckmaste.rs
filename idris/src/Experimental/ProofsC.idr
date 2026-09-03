||| Unspellable pins, continued from Experimental.ProofsB.
module Experimental.ProofsC

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Each player creates a 1/1 green Plant creature token. Put a +1/+1 counter on it."
||| A distributed creation exports a plural mention, so the singular pronoun resolves to nothing.
public export
badDistributedCreationIt : Unspellable (Effect []) (\ok =>
  Sequentially [Create (Each AnyPlayer) (Lit 1)
                       (TokenWritten (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])) [],
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (It {ok})])
badDistributedCreationIt Refl impossible


||| "each of each creature"
||| "Each of" reaches into a group MENTION, and the distributive is a description.
public export
badEachOfDistributive : Unspellable (Noun [] Object) (\ok =>
  EachOf (Each Macros.creature) {gm = ok})
badEachOfDistributive Oh impossible


||| "each of all creatures"
||| Nor the universal, for the same reason: it too describes rather than mentions.
public export
badEachOfAll : Unspellable (Noun [] Object) (\ok =>
  EachOf (AllOf Macros.creature) {gm = ok})
badEachOfAll Oh impossible


||| "each of each of up to two target creatures"
||| One determiner fills the position, so it does not stack.
public export
badNestedEachOf : Unspellable (Noun [] Object) (\ok =>
  EachOf (EachOf (TargetGroup (Macros.upTo 2) Macros.creature)) {gm = ok})
badNestedEachOf Oh impossible


||| "Put a +1/+1 counter on up to two target creatures."
||| A clause writing one per-member amount refuses a bare plural recipient; "each of" is the word.
public export
badBarePluralCounterRecipient : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (TargetGroup (Macros.upTo 2) Macros.creature) {pm = ok})
badBarePluralCounterRecipient Oh impossible


||| "This deals 1 damage to up to two target creatures."
||| The damage verb reads the same way: its plural-looking recipient is singular already.
public export
badBarePluralDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (TargetGroup (Macros.upTo 2) Macros.creature) {pm = ok})
badBarePluralDamageRecipient Oh impossible


||| "Choose any number of target creatures. Put a +1/+1 counter on them."
||| A plural read is no better than a plural mention at the recipient slot.
public export
badThemCounterRecipient : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (TargetGroup Macros.anyNumber Macros.creature) Nothing Openly,
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Them {pm = ok}])
badThemCounterRecipient Oh impossible


||| "This deals 2 damage divided as you choose among each creature."
||| [CR#601.2d] divides over the announced targets, so the members are a mention and not a description.
public export
badDivideAmongDescription : Unspellable (Effect []) (\ok =>
  Macros.dealsDivided This (Lit 2) (Each Macros.creature) {gm = ok})
badDivideAmongDescription Oh impossible


||| "Put target creature into your library."
||| A library is ordered [CR#401.2], so the bare zone names no place to put a card.
public export
badMoveToBareLibrary : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) (ZoneAt Library Bare) (MkMoveRiders [] Nothing Nothing) {ok})
badMoveToBareLibrary BattlefieldOk impossible


||| "Look at the top card of your library. Put that card on the bottom of your library in any order."
||| The order rider needs two or more cards to order — [CR#401.4]'s own condition, and English's.
public export
badSingularOrderRider : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt Macros.topCard, Move (That CardW) (Macros.onBottomIn AnyOrder) (MkMoveRiders [] Nothing Nothing) {arr = ok}])
badSingularOrderRider Oh impossible


||| "Put the rest into your graveyard."
||| "The rest" of what: with no group in the discourse there is nothing to be the rest of.
public export
badRestWithoutGroup : Unspellable (Effect []) (\ok =>
  Move (TheRest {ok}) Macros.graveyardZ (MkMoveRiders [] Nothing Nothing))
badRestWithoutGroup Oh impossible


||| "Look at the top four cards of your library. Put the rest on the bottom."
||| With nothing taken out, "the rest" is the group, which the sentence would call "them".
public export
badRestWithoutPart : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Move (TheRest {ok}) Macros.onBottomZ (MkMoveRiders [] Nothing Nothing)])
badRestWithoutPart Oh impossible


||| "Look at the top four cards … Put one into your hand, the rest on the bottom, the rest into your graveyard."
||| One disposition per remainder: the move spends the group and leaves nothing outstanding.
public export
badRestDisposedTwice : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt (Macros.topCards 4)
               , Move (Macros.oneOf Them) Macros.handZ (MkMoveRiders [] Nothing Nothing)
               , Move TheRest Macros.onBottomZ (MkMoveRiders [] Nothing Nothing)
               , Move (TheRest {ok}) Macros.graveyardZ (MkMoveRiders [] Nothing Nothing)
               ])
badRestDisposedTwice Oh impossible


||| "Target creature you control fights target creature you don't control. Put the rest into your graveyard."
||| Two separately announced targets are two mentions, never a pair [CR#601.2c].
public export
badRestOverTwoAnnouncements : Unspellable (Effect []) (\ok =>
  Sequentially [ Fights (Macros.target Macros.creatureYouControl) (Macros.target Macros.creatureYouDontControl)
               , Move (TheRest {ok}) Macros.graveyardZ (MkMoveRiders [] Nothing Nothing)
               ])
badRestOverTwoAnnouncements Oh impossible


||| "Look at the top four cards of your library. Choose one of them."
||| A partitive needs a chooser, and this clause has no agent slot to name one.
public export
badChooseSomeOf : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Choose (Macros.oneOf Them) Nothing Openly {ch = ok}])
badChooseSomeOf BareChoice impossible


||| "Tap the top card of your library."
||| The slice is in a library, which every battlefield-demanding verb already refuses.
public export
badTapLibraryTop : Unspellable (Effect []) (\ok =>
  SetStatus Tapped Macros.topCard {ok})
badTapLibraryTop OnField impossible


||| "Look at the top four cards of your library. Put those creature cards into your hand."
||| The slice names a place and describes no card [CR#400.2,401.2], so a typed demonstrative reaches nothing.
public export
badSliceTypeRead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Move (Those (TypeW Creature) {ok}) Macros.handZ (MkMoveRiders [] Nothing Nothing)])
badSliceTypeRead Refl impossible


||| "Reveal your graveyard."
||| A graveyard is already visible to everyone [CR#400.2], so revealing one says nothing.
public export
badRevealGraveyard : Unspellable (Effect []) (\ok =>
  Expose Reveal You (ExposedZone Macros.graveyardZ {ok}))
badRevealGraveyard Oh impossible


||| "Search your library for a creature card in a graveyard."
||| The clause supplies the place, so a phrase carrying its own is two answers to one question.
public export
badSearchZonedDescription : Unspellable (Effect []) (\ok =>
  Macros.searchLibraryFor (And [Macros.creature, InZone Macros.graveyardZ]) {zf = ok})
badSearchZonedDescription Refl impossible


||| "Each player mills a card. Exile it."
||| [CR#701.17a] mills each player from their own library, so a distributed mill leaves a plural group.
public export
badDistributedMillSingular : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.mills (Each AnyPlayer) (Lit 1) (Each AnyPlayer)
               , Macros.exile (It {ok}) ])
badDistributedMillSingular Refl impossible


||| "one of a creature you control"
||| A partitive reaches into a group, not a description: no members exist until a phrase fixes them.
public export
badPartitiveOfDescription : Unspellable (Effect []) (\ok =>
  Macros.exile (SomeOf (CountedSlice (Macros.exactly 1)) Nothing (Macros.a Macros.creature) {gm = ok}))
badPartitiveOfDescription Oh impossible


||| "two of one of them"
||| Nor into another partitive: a part is what was taken, not a group to take from.
public export
badPartitiveOfPartitive : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Macros.exile (Macros.oneOf (Macros.oneOf Them) {gm = ok})])
badPartitiveOfPartitive Oh impossible


||| "each of the rest"
||| Nor into the complement: the remainder is named rather than reached into.
public export
badEachOfTheRest : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt (Macros.topCards 4)
               , Move (Macros.oneOf Them) Macros.handZ (MkMoveRiders [] Nothing Nothing)
               , Macros.exile (EachOf TheRest {gm = ok})
               ])
badEachOfTheRest Oh impossible


||| "If target creature card in your graveyard would die, exile it instead this turn."
||| Dying is the battlefield-to-graveyard transition [CR#700.4], so the watched object is on the battlefield.
public export
badWouldDieInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok})
                 (Macros.exile It) (Just Macros.thisTurn))
badWouldDieInGraveyard Oh impossible


||| "The next time you would draw a card this turn, create a 1/1 green Soldier creature token instead. Sacrifice it."
||| [CR#614.7] makes a replacement whose event never happens do nothing, so it announces no referent.
public export
badInterceptReplacementAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.nextTimeWouldInstead (Draws You)
                                      (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Soldier"]))
                                      (Just Macros.thisTurn)
               , Macros.sacrifice You (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}
               ])
badInterceptReplacementAntecedent (Refl, _) impossible


||| "Exile target creature until this creature leaves the battlefield. Put that card into your hand."
||| The undo is scheduled on an event that has not happened, so the clause contributes no retag.
public export
badHeldUntilExileRetag : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.exileUntil (Macros.target Macros.creature) (Macros.leavesBattlefield Macros.thisCreature)
               , Move (That CardW {ok}) Macros.handZ (MkMoveRiders [] Nothing Nothing)
               ])
badHeldUntilExileRetag Refl impossible


||| "If you would draw a card, draw two instead — and instead of that, draw three."
||| A replacement gets only one opportunity to affect an event or its modified successors [CR#614.5].
public export
badNestedInstead : Unspellable (Effect []) (\ok =>
  Macros.insteadOf (Macros.insteadOf Macros.drawACard (Macros.drawCards 2)) (Macros.drawCards 3) {na = ok})
badNestedInstead Oh impossible


||| "Sacrifice a creature, Exile the sacrificed card: Draw a card."
||| [CR#601.2h] pays components in any order, so none may presuppose a sibling already paid.
public export
badCostReadsSiblingDeed : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.sacrifice You (Macros.a Macros.creature)),
                       Do (Macros.exile (TheVerbed "Sacrifice" CardW Attributive)) {ok}])
            Macros.drawACard Nothing Nothing Nothing Nothing)
badCostReadsSiblingDeed Oh impossible


||| "An opponent pays 2 life: Draw a card."
||| An activation cost must be paid by the player activating the ability [CR#602.1a].
public export
badForeignPayerCost : Unspellable Ability (\ok =>
  Activated (Macros.payLife Macros.anOpponent 2) Macros.drawACard Nothing Nothing Nothing Nothing {py = ok})
badForeignPayerCost Oh impossible


||| "An opponent sacrifices a creature: Draw a card."
||| The subjected cost verbs the same way: no payment of yours stands before a colon [CR#602.1a].
public export
badForeignSacrificeCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice Macros.anOpponent (Macros.a Macros.creature))) Macros.drawACard Nothing Nothing Nothing Nothing {py = ok})
badForeignSacrificeCost Oh impossible


||| "you pay"
public export
badMismatchedPayer : Unspellable (Effect []) (\ok =>
  Macros.mayElse You (Pay You (Macros.payLife Macros.anOpponent 1) PaidOnce {ag = ok}) Macros.drawACard)
badMismatchedPayer Oh impossible


||| "You pay {T}."
||| The tap symbol is a cost that exists only before a colon [CR#107.5], never as a verb phrase.
public export
badPayTapSymbol : Unspellable (Effect []) (\ok =>
  Pay You TapSymbol PaidOnce {pb = ok})
badPayTapSymbol Oh impossible


||| "Creatures you control get +1/+1 until end of turn:"
public export
badContinuousAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1))
                             (Just Macros.untilEndOfTurn)) {ok})
            Macros.drawACard Nothing Nothing Nothing Nothing)
badContinuousAsCost Oh impossible


||| a replacement written as a cost
||| [CR#118.1] makes a cost an action a player carries out; a replacement [CR#614.6] happens instead of an event.
public export
badInsteadAsCost : Unspellable Ability (\ok =>
  Activated (Do (InsteadOf (Macros.destroy (Macros.target Macros.creature))
                           (Macros.exile (Macros.target Macros.creature))) {ok})
            Macros.drawACard Nothing Nothing Nothing Nothing)
badInsteadAsCost Oh impossible


||| a delayed trigger written as a cost
||| [CR#118.1] makes a cost an action taken now; a delayed trigger [CR#603.7a] only sets one up for later.
public export
badDelayedAsCost : Unspellable Ability (\ok =>
  Activated (Do (Delayed (BeginningOf EndStep NoPossessor) [] Nothing Macros.drawACard) {ok})
            Macros.drawACard Nothing Nothing Nothing Nothing)
badDelayedAsCost Oh impossible


||| an "until" rider written as a cost
||| [CR#118.1] makes a cost an action a player carries out; the rider [CR#610.3] schedules a later return.
public export
badHeldUntilAsCost : Unspellable Ability (\ok =>
  Activated (Do (HeldUntil (Macros.exile (Macros.target Macros.creature))
                           (Dies (Macros.a Macros.creature))) {ok})
            Macros.drawACard Nothing Nothing Nothing Nothing)
badHeldUntilAsCost Oh impossible


||| a reflexive trigger written as a cost
||| [CR#118.1] makes a cost an action taken now; [CR#603.12] creates a trigger that fires afterwards.
public export
badReflexiveAsCost : Unspellable Ability (\ok =>
  Activated (Do (Reflexively (Macros.gainsLife You (Lit 2)) Macros.drawACard) {ok})
            Macros.drawACard Nothing Nothing Nothing Nothing)
badReflexiveAsCost Oh impossible


||| "You skip your next turn:"
public export
badSkipAsCost : Unspellable Ability (\ok =>
  Activated (Do (SkipsNext You Turn (Lit 1)) {ok}) Macros.drawACard Nothing Nothing Nothing Nothing)
badSkipAsCost Oh impossible


||| "You pay 2 life:"
public export
badPayAsCost : Unspellable Ability (\ok =>
  Activated (Do (Pay You (Macros.payLife You 2) PaidOnce) {ok}) Macros.drawACard Nothing Nothing Nothing Nothing)
badPayAsCost Oh impossible


||| "Discard a card, then sacrifice a creature:"
public export
badSequentialCost : Unspellable Ability (\ok =>
  Activated (Do (Sequentially [Macros.discardsACard You,
                               Macros.sacrifice You (Macros.a Macros.creature)]) {ok})
            Macros.drawACard Nothing Nothing Nothing Nothing)
badSequentialCost Oh impossible


||| "Discard a card and sacrifice a creature simultaneously:"
public export
badSimultaneousCost : Unspellable Ability (\ok =>
  Activated (Do (Simultaneously [Macros.discardsACard You,
                                 Macros.sacrifice You (Macros.a Macros.creature)]) {ok})
            Macros.drawACard Nothing Nothing Nothing Nothing)
badSimultaneousCost Oh impossible


||| "Repeat this process:"
public export
badRepeatAsCost : Unspellable Ability (\ok =>
  Activated (Do (Repeat Again) {ok}) Macros.drawACard Nothing Nothing Nothing Nothing)
badRepeatAsCost Oh impossible


||| a compound cost of no components
||| A cost telescope of nothing is no cost: there is nothing to pay.
public export
badEmptyCompound : Unspellable Ability (\ok =>
  Activated (Compound [] {ne = ok}) Macros.drawACard Nothing Nothing Nothing Nothing)
badEmptyCompound ItIsSucc impossible


public export
nestedCompoundCost : Ability
nestedCompoundCost =
  Activated (Compound [Compound [Mana [Macros.generic 1], TapSymbol], TapSymbol])
            Macros.drawACard Nothing Nothing Nothing Nothing


||| "{T}, {T}: Draw a card."
||| An already tapped permanent cannot be tapped again to pay a cost [CR#107.5,118.3].
public export
badDoubleTapCost : Unspellable Ability (\ok =>
  Activated (Compound [TapSymbol, TapSymbol]) Macros.drawACard Nothing Nothing Nothing Nothing {tp = ok})
badDoubleTapCost Oh impossible


||| ": Draw a card."
public export
badEmptyManaCost : Unspellable Ability (\ok =>
  Activated (Mana [] {wr = ok}) Macros.drawACard Nothing Nothing Nothing Nothing)
badEmptyManaCost IsNonEmpty impossible


||| "{W/W/P}"
||| A hybrid Phyrexian symbol names two different colors [CR#107.4f].
public export
badSameColorPhyrexian : Unspellable ManaSymbol (\ok =>
  Phyrexian White (Just White) {ds = ok})
badSameColorPhyrexian Oh impossible


||| "{U/U}"
||| [CR#107.4e] makes the symbol a cost payable one of two ways, and two same ways is one way.
public export
badSameColorHybrid : Unspellable ManaSymbol (\ok =>
  Macros.hybridPip Blue Blue {ds = ok})
badSameColorHybrid Oh impossible


||| "Sacrifice a creature. If you don't, exile it."
||| The didn't-arm runs precisely because the instruction's action did not happen [CR#609.3], so the body named nothing for "it" to read.
public export
badIfNotReadsMandatoryBody : Unspellable (Effect []) (\ok =>
  Macros.doElse (Macros.sacrifice You (Macros.a Macros.creature)) (Macros.exile (It {ok})))
badIfNotReadsMandatoryBody Refl impossible


||| "Counter target spell unless its controller taps."
public export
badUnlessTapSymbol : Unspellable (Effect []) (\ok =>
  Unless (Macros.counterSpell (Macros.target Macros.spell)) (ControllerOf It) TapSymbol {pb = ok})
badUnlessTapSymbol Oh impossible


||| "Whenever target creature dies, draw a card."
||| An ordinary trigger's header announces no target: [CR#115.1d] chooses them after the event.
public export
badTargetedDeathHeader : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.target Macros.creature)) [] Nothing [] Nothing Nothing Nothing Macros.drawACard {hn = ok})
badTargetedDeathHeader Oh impossible


||| "At the beginning of your turn, draw a card."
||| [CR#603.2b] triggers on a phase or step beginning; a turn is neither [CR#500.1].
public export
badTriggerAtYourTurn : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Turn (ByWord Yours) {pu = ok}) [] Nothing [] Nothing Nothing Nothing Macros.drawACard)
badTriggerAtYourTurn Oh impossible


||| "Whenever a creature dies, tap it."
||| [CR#603.6] looks for the object in the zone it moved to: a card in a graveyard [CR#701.26a].
public export
badTriggerTapsDeadCreature : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.a Macros.creature)) [] Nothing [] Nothing Nothing Nothing (SetStatus Tapped It {ok}))
badTriggerTapsDeadCreature OnField impossible


||| "Whenever a creature leaves the battlefield, tap it."
||| [CR#603.6c] checks the object only in the first zone it went to, which the sentence never names.
public export
badLeavesThenTap : Unspellable Ability (\ok =>
  Triggered Whenever (Macros.leavesBattlefield (Macros.a Macros.creature)) [] Nothing [] Nothing Nothing Nothing (SetStatus Tapped It {ok}))
badLeavesThenTap OnField impossible


||| "Target creature can't attack."
public export
badStaticTargets : Unspellable Ability (\ok =>
  Static (Macros.deontic (Macros.target Macros.creature) Forbid ["Attack"] Agent NoDeonticPatient) {ut = ok})
badStaticTargets Oh impossible


||| "You may play a creature this turn."
public export
badPlayFromBattlefield : Unspellable (Effect []) (\ok =>
  Continuously (Deontic You Permit ["Play"] Agent
                  (DeonticCounterpart (Macros.a Macros.creature)) Nothing
                  (PlayRider Nothing Nothing Nothing False ItsOwnCost) {rd = ok})
               (Just Macros.thisTurn))
badPlayFromBattlefield Oh impossible


||| "Target creature gets +3/+3 until the beginning of your next upkeep."
public export
badUntilBeginningOfUpkeep : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just (UntilEvent (BeginningOf Upkeep (ByWord Yours)))) {sp = ok})
badUntilBeginningOfUpkeep (SpanStated {ok = Oh}) impossible


||| "This creature deals 3 damage to any target. When you do, draw a card."
||| The anaphor is a pro-verb whose subject is a player, and a damage clause's agent is an object [CR#120.1].
public export
badReflexiveOnSourceDeed : Unspellable (Effect []) (\ok =>
  Reflexively (DealDamage This (Lit 3) (Macros.target Macros.anyTarget)) Macros.drawACard {en = ok})
badReflexiveOnSourceDeed Oh impossible


||| "You gain 2 life. When you do, draw a card."
||| [CR#119.9] makes the player the patient of a life gain, with no action to inflect.
public export
badReflexiveOnLifeGain : Unspellable (Effect []) (\ok =>
  Reflexively (Macros.gainsLife You (Lit 2)) Macros.drawACard {en = ok})
badReflexiveOnLifeGain Oh impossible


||| "Draw a card, then sacrifice a creature. When you do, draw a card."
||| One verb phrase, because "do" abbreviates one; two sentences hang it on the last.
public export
badReflexiveOnSequence : Unspellable (Effect []) (\ok =>
  Reflexively (Sequentially [Macros.drawACard, Macros.sacrifice You (Macros.a Macros.creature)]) Macros.drawACard {en = ok})
badReflexiveOnSequence Oh impossible


||| "At the beginning of your next end step, draw a card. When you do, draw a card."
||| A clause that schedules its action has not taken it [CR#603.12].
public export
badReflexiveOnDelayed : Unspellable (Effect []) (\ok =>
  Reflexively (Delayed (BeginningOf EndStep (ByWord Yours)) [] Nothing Macros.drawACard) Macros.drawACard {en = ok})
badReflexiveOnDelayed Oh impossible


||| "At the beginning of your next end step, draw a card. When you draw a card this way, draw a card."
||| The scheduled draw is a separate delayed ability's deed [CR#603.7e], not something the enclosure did, so "this way" names nothing [CR#603.12].
public export
badThisWayOnDelayed : Unspellable (Effect []) (\ok =>
  ThisWay (Delayed (BeginningOf EndStep (ByWord Yours)) [] Nothing Macros.drawACard)
          (Draws You) Macros.drawACard {oc = ok})
badThisWayOnDelayed Oh impossible


||| "You may sacrifice a creature. If you do, draw a card. When you do, draw a card."
||| [CR#118.12]'s "if you do" and [CR#603.12]'s "when you do" ask the same question of one choice.
public export
badReflexiveOnBranchedMay : Unspellable (Effect []) (\ok =>
  Reflexively (Macros.mayThen You (Macros.sacrifice You (Macros.a Macros.creature)) Macros.drawACard) Macros.drawACard {en = ok})
badReflexiveOnBranchedMay Oh impossible


||| "Mill four cards. When you do, create a 1/1 white Soldier creature token. Tap that creature."
||| [CR#603.3] stacks a triggered ability only at the next priority, after the resolution finishes.
public export
badAfterReflexiveReadsTrigger : Unspellable (Effect []) (\ok =>
  Sequentially [Reflexively (Macros.mills You (Lit 4) You)
                            (Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [White]
                                                     (MkTypeLine [creatureType "Soldier"] [Creature])
                                                     [] Nothing)),
                SetStatus Tapped (That (TypeW Creature) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badAfterReflexiveReadsTrigger (Refl, _) impossible


||| "Sacrifice a creature. When you do, tap it."
||| The reflexive reads the enclosure's post-state, so the sacrificed creature is already in its graveyard.
public export
badReflexiveTapsSacrificed : Unspellable (Effect []) (\ok =>
  Reflexively (Macros.sacrifice You (Macros.a Macros.creature)) (SetStatus Tapped It {ok}))
badReflexiveTapsSacrificed OnField impossible


||| "Put target creature into its owner's graveyard tapped."
||| An arrival rider is battlefield-only: status belongs to permanents [CR#110.5,110.5b].
public export
badMoveRidersToGraveyard : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.graveyardZ (MkMoveRiders [EntersTapped] Nothing Nothing) {rf = ok})
badMoveRidersToGraveyard Oh impossible


||| "Put target creature into its owner's hand under your control."
||| An object neither on the stack nor on the battlefield has no controller [CR#109.4].
public export
badMoveControlToHand : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.handZ (MkMoveRiders [] (Just You) Nothing) {rf = ok})
badMoveControlToHand Oh impossible


||| "Put target creature onto the battlefield under the other players' control."
||| One controller [CR#109.4]; the plural relational is not a spelling this vocabulary has.
public export
badMoveRidersPluralController : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.battlefieldZ (MkMoveRiders [] (Just (AllOf Macros.otherPlayer)) Nothing {one = ok}))
badMoveRidersPluralController OneController impossible


||| "cards exiled with target creature"
||| A linkage read names the exiles of the ability's own object [CR#607.1,406.6].
public export
badExiledWithOtherSource : Unspellable (Predicate [] Object) (\ok =>
  ExiledWith (Macros.target Macros.creature) {ls = ok})
badExiledWithOtherSource SelfLinked impossible


||| "a card exiled with an artifact"
||| The same gate: the linkage is one object's note about its own exiles [CR#607.1].
public export
badExiledWithDescribedSource : Unspellable (Predicate [] Object) (\ok =>
  ExiledWith (Macros.a Macros.artifact) {ls = ok})
badExiledWithDescribedSource SelfLinked impossible


||| "a card you control exiled with this artifact"
||| The linked cards are in exile [CR#607.2a], and such an object has no controller [CR#109.4].
public export
badExiledWithControlled : Unspellable (Predicate [] Object) (\ok =>
  And [ControlledBy You, Macros.exiledWithThisArtifact] {zc = ok})
badExiledWithControlled Oh impossible
