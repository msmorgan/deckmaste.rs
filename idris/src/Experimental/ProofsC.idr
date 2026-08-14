module Experimental.ProofsC

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Gain control of target creature this turn."
||| The control grant writes the grants' current-turn word, never the
||| restrictions' "this turn".
public export
badGainControlThisTurn : Unspellable (Effect []) (\ok =>
  Macros.gainControl (Macros.target Macros.creature) (Just Macros.thisTurn) {sp = ok})
badGainControlThisTurn SpanStated impossible


||| "Target creature gains haste until the end of your next turn."
||| "Until the end of your next turn" is the CONTROL grant's alone; the
||| keyword grant does not write it.
public export
badKeywordGrantEndOfNextTurn : Unspellable (Effect []) (\ok =>
  Macros.gainsHaste (Macros.target Macros.creature) (Just (Until (EndOf Turn (Just Yours)))) {sp = ok})
badKeywordGrantEndOfNextTurn SpanStated impossible


||| "Target creature becomes an artifact in addition to its other types for as long as you control this creature."
||| Every construction but the type ADDITION writes "for as long as"; the
||| addition's own phrase pairs with the adverbial zero times.
public export
badTypeAdditionForAsLongAs : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact])
          (Just (ForAsLongAs (Matches Macros.thisCreature (ControlledBy You)))) {sp = ok})
badTypeAdditionForAsLongAs SpanStated impossible


||| "Each player creates a 1/1 green Plant creature token. Put a +1/+1 counter on it."
||| A distributed creation exports a PLURAL mention, so the singular pronoun
||| has nothing to resolve to.
public export
badDistributedCreationIt : Unspellable (Effect []) (\ok =>
  Sequentially [Create (Each AnyPlayer) (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant]) [],
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badDistributedCreationIt (_, MkCounterHolder) impossible


||| "each of each creature"
||| "Each of" reaches into a group MENTION, and the distributive is a
||| description.
public export
badEachOfDistributive : Unspellable (Noun [] Object) (\ok =>
  EachOf (Each Macros.creature) {gm = ok})
badEachOfDistributive MkGroupMention impossible


||| "each of all creatures"
||| Nor the universal, for the same reason: it too describes rather than
||| mentions.
public export
badEachOfAll : Unspellable (Noun [] Object) (\ok =>
  EachOf (AllOf Macros.creature) {gm = ok})
badEachOfAll MkGroupMention impossible


||| "each of each of up to two target creatures"
||| One determiner fills the position, so it does not stack.
public export
badNestedEachOf : Unspellable (Noun [] Object) (\ok =>
  EachOf (EachOf (TargetGroup (Macros.upTo 2) Macros.creature)) {gm = ok})
badNestedEachOf MkGroupMention impossible


||| "Put a +1/+1 counter on up to two target creatures."
||| A clause writing ONE per-member amount refuses a bare plural recipient;
||| "each of" is the word oracle writes instead.
public export
badBarePluralCounterRecipient : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) Macros.plusOnePlusOne (TargetGroup (Macros.upTo 2) Macros.creature) {pm = ok})
badBarePluralCounterRecipient MkPerMember impossible


||| "This deals 1 damage to up to two target creatures."
||| The damage verb reads the same way; its plural-looking form is singular
||| already ("to up to ONE target creature").
public export
badBarePluralDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (TargetGroup (Macros.upTo 2) Macros.creature) {pm = ok})
badBarePluralDamageRecipient MkPerMember impossible


||| "Choose any number of target creatures. Put a +1/+1 counter on them."
||| A plural READ is no better than a plural mention; the corpus's "counters
||| on them" lines are relative clauses, never a recipient.
public export
badThemCounterRecipient : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (TargetGroup Macros.anyNumber Macros.creature),
                PutCounters (Lit 1) Macros.plusOnePlusOne Them {pm = ok}])
badThemCounterRecipient MkPerMember impossible


||| "This deals 1 damage to all creatures."
||| And the universal determiner with them: the sweep is written
||| distributively, "damage to each creature".
public export
badAllOfDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (AllOf Macros.creature) {pm = ok})
badAllOfDamageRecipient MkPerMember impossible


||| "This deals 2 damage divided as you choose among each creature."
||| [CR#601.2d] has the caster divide over the targets they announced, so the
||| members are a MENTION and not a description.
public export
badDivideAmongDescription : Unspellable (Effect []) (\ok =>
  Macros.dealsDivided This (Lit 2) (Each Macros.creature) {gm = ok})
badDivideAmongDescription MkGroupMention impossible


||| "This deals 0 damage divided as you choose among one or two targets."
||| A division of nothing instructs nothing — the written-count discipline
||| reaching the new clause.
public export
badDivideZero : Unspellable (Effect []) (\ok =>
  Macros.dealsDivided This (Lit 0) (TargetGroup (Macros.oneThrough 2) AnyTarget) {wc = ok})
badDivideZero MkWrittenCount impossible


||| "Distribute two +1/+1 counters among one or two target creature cards in your graveyard."
||| The counter half keeps the battlefield demand its undivided twin carries.
public export
badDistributeCountersGraveyard : Unspellable (Effect []) (\ok =>
  Macros.distributeCounters (Lit 2) Macros.plusOnePlusOne
                     (TargetGroup (Macros.oneThrough 2) (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok})
badDistributeCountersGraveyard OnField impossible


||| "Put target creature into your library."
||| A library is ORDERED [CR#401.2], so the bare zone names no place to put
||| a card; every corpus placement spells a position.
public export
badMoveToBareLibrary : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) (ZoneAt Library Bare) {ok})
badMoveToBareLibrary BattlefieldOk impossible


||| "Look at the top card of your library. Put that card on the bottom of your library in any order."
||| The order rider needs two or more cards to order — [CR#401.4]'s own
||| condition, and English's.
public export
badSingularOrderRider : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt Macros.topCard, Move (That CardW) (Macros.onBottomIn AnyOrder) {arr = ok}])
badSingularOrderRider MkArrangementOk impossible


||| "Put the rest into your graveyard."
||| "The rest" of WHAT: with no group in the discourse the complement has
||| nothing to be the rest of.
public export
badRestWithoutGroup : Unspellable (Effect []) (\ok =>
  Move (TheRest {ok}) Macros.graveyardZ)
badRestWithoutGroup Refl impossible


||| "Look at the top four cards of your library. Put the rest on the bottom."
||| With nothing taken out of the group, "the rest" IS the group and the
||| sentence would have written "them".
public export
badRestWithoutPart : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Move (TheRest {ok}) Macros.onBottomZ])
badRestWithoutPart Refl impossible


||| "Look at the top four cards … Put one into your hand, the rest on the bottom, the rest into your graveyard."
||| One disposition per remainder: the move spends the group, and once it has
||| been placed nothing is outstanding.
public export
badRestDisposedTwice : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt (Macros.topCards 4)
               , Move (Macros.oneOf Them) Macros.handZ
               , Move TheRest Macros.onBottomZ
               , Move (TheRest {ok}) Macros.graveyardZ
               ])
badRestDisposedTwice Refl impossible


||| "Target creature you control fights target creature you don't control. Put the rest into your graveyard."
||| Two separately announced targets are two mentions and never a pair
||| [CR#601.2c], so no complement can subtract inside them.
public export
badRestOverTwoAnnouncements : Unspellable (Effect []) (\ok =>
  Sequentially [ Fights (Macros.target Macros.creatureYouControl) (Macros.target Macros.creatureYouDontControl)
               , Move (TheRest {ok}) Macros.graveyardZ
               ])
badRestOverTwoAnnouncements Refl impossible


||| "Look at the top four cards of your library. Choose one of them."
||| The corpus names a partitive's chooser every time, and this clause has no
||| agent slot to name one.
public export
badChooseSomeOf : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Choose (Macros.oneOf Them) {ch = ok}])
badChooseSomeOf MkChoosable impossible


||| "Search your library for a land card, shuffle, then put it into your hand."
||| A shuffle destroys what the discourse knew about the library [CR#701.24a],
||| a reordered revealed card being a NEW object [CR#701.20d].
public export
badReadAfterShuffle : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.searchLibraryFor Macros.land, Macros.shuffle, Move (It {ok}) Macros.handZ])
badReadAfterShuffle Refl impossible


||| "Tap the top card of your library."
||| The slice is in a library, which every battlefield-demanding verb already
||| refuses.
public export
badTapLibraryTop : Unspellable (Effect []) (\ok =>
  Tap Macros.topCard {ok})
badTapLibraryTop OnField impossible


||| "Look at the top four cards of your library. Put those creature cards into your hand."
||| The slice names a place and describes no card [CR#400.2,401.2], so a typed
||| demonstrative has nothing to reach.
public export
badSliceTypeRead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Move (Those (TypeW Creature) {ok}) Macros.handZ])
badSliceTypeRead Refl impossible


||| "Reveal your library."
||| A library is not shown whole; what oracle exposes of one is a positioned
||| slice.
public export
badRevealWholeLibrary : Unspellable (Effect []) (\ok =>
  Expose Reveal You (ExposedZone Macros.yourLibrary {ok}))
badRevealWholeLibrary MkExposableZone impossible


||| "Reveal your graveyard."
||| Nor is a public zone: a graveyard is already visible to everyone
||| [CR#400.2], so revealing one says nothing.
public export
badRevealGraveyard : Unspellable (Effect []) (\ok =>
  Expose Reveal You (ExposedZone Macros.graveyardZ {ok}))
badRevealGraveyard MkExposableZone impossible


||| "Search the battlefield for a creature."
||| Nor is the battlefield searched: [CR#701.23a] is about finding a card
||| among cards you cannot otherwise read.
public export
badSearchBattlefield : Unspellable (Effect []) (\ok =>
  Search You Macros.battlefieldZ Macros.creature {sz = ok})
badSearchBattlefield MkSearchableZone impossible


||| "Search the top of your library for a land card."
||| [CR#701.23a] looks through "all cards in that zone" and [CR#401.2] makes
||| the library "a single face-down pile", so a position is not the zone.
public export
badSearchLibraryPosition : Unspellable (Effect []) (\ok =>
  Search You (LibraryAt OnTop Nothing Bare) Macros.land {wz = ok})
badSearchLibraryPosition MkWholeZone impossible


||| "Search the bottom of your library in a random order for a creature card."
||| The same refusal with the arrangement rider making it plain: a search
||| cannot look through cards "in a random order".
public export
badSearchLibraryPositionOrdered : Unspellable (Effect []) (\ok =>
  Search You (LibraryAt OnBottom (Just RandomOrder) Bare) Macros.creature {wz = ok})
badSearchLibraryPositionOrdered MkWholeZone impossible


||| "Search your library for a creature card in a graveyard."
||| The clause supplies the place, so a phrase carrying its own is two
||| answers to one question.
public export
badSearchZonedDescription : Unspellable (Effect []) (\ok =>
  Macros.searchLibraryFor (And [Macros.creature, InZone Macros.graveyardZ]) {zf = ok})
badSearchZonedDescription MkZoneFree impossible


||| "Look at the top zero cards of your library."
||| A zero slice instructs nothing — `WrittenCount`'s discipline at a new
||| count.
public export
badZeroSlice : Unspellable (Effect []) (\ok =>
  Macros.lookAt (Macros.topCards 0 {wc = ok}))
badZeroSlice MkWrittenCount impossible


||| "Mill zero cards."
||| And a zero mill likewise instructs nothing.
public export
badMillZero : Unspellable (Effect []) (\ok =>
  Macros.millCards 0 {wc = ok})
badMillZero MkWrittenCount impossible


||| "Each player mills a card. Exile it."
||| [CR#701.17a] has each milled-at player use their own library and
||| graveyard, so a DISTRIBUTED mill leaves a plural group.
public export
badDistributedMillSingular : Unspellable (Effect []) (\ok =>
  Sequentially [Mill (Each AnyPlayer) (Lit 1), Macros.exile (It {ok})])
badDistributedMillSingular Refl impossible


||| "one of a creature you control"
||| A partitive reaches into a GROUP, not a description: the determiner has
||| no members to pick from until a phrase has fixed them.
public export
badPartitiveOfDescription : Unspellable (Effect []) (\ok =>
  Macros.exile (SomeOf (Macros.exactly 1) (Macros.a Macros.creature) {gm = ok}))
badPartitiveOfDescription MkGroupMention impossible


||| "two of one of them"
||| Nor into another partitive: a part is what was taken, not a group to take
||| from.
public export
badPartitiveOfPartitive : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Macros.exile (Macros.oneOf (Macros.oneOf Them) {gm = ok})])
badPartitiveOfPartitive MkGroupMention impossible


||| "each of the rest"
||| Nor into the complement: the remainder is named rather than reached into.
public export
badEachOfTheRest : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt (Macros.topCards 4)
               , Move (Macros.oneOf Them) Macros.handZ
               , Macros.exile (EachOf TheRest {gm = ok})
               ])
badEachOfTheRest MkGroupMention impossible


||| "If target creature would die, exile it instead." with no duration
||| A durationless interception is the STATIC ABILITY line [CR#611.3]; every
||| ONE-SHOT interception writes a span.
public export
badStandingIntercept : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (Dies (Macros.target Macros.creature)) (Macros.exile It) Nothing {sp = ok})
badStandingIntercept SpanUnstated impossible


||| "Prevent all damage." with no duration
||| The same refusal on the shield: every "Prevent all …" line that states no
||| duration is a static ability.
public export
badStandingPrevention : Unspellable (Effect []) (\ok =>
  Macros.preventAll AnyDamage Everywhere Nothing {sp = ok})
badStandingPrevention SpanUnstated impossible


||| "Prevent all combat damage until end of turn."
||| Prevention writes ONE adverbial and it is "this turn"; the shield and the
||| grants share no current-turn word.
public export
badPreventUntilEndOfTurn : Unspellable (Effect []) (\ok =>
  Macros.preventAll CombatOnly Everywhere (Just Macros.untilEndOfTurn) {sp = ok})
badPreventUntilEndOfTurn SpanStated impossible


||| "If target creature would die, exile it instead for as long as you control a creature."
||| Nor a for-as-long-as one: no line conditions a shield on a tracked
||| predicate [CR#611.2b].
public export
badInterceptForAsLongAs : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (Dies (Macros.target Macros.creature)) (Macros.exile It)
                 (Just (ForAsLongAs (Exists Macros.creatureYouControl))) {sp = ok})
badInterceptForAsLongAs SpanStated impossible


||| "If target creature would be destroyed, exile it instead this turn."
||| Real oracle in another construction: "would be destroyed" is
||| regeneration's, whose replacement is [CR#614.8]'s four-part instruction.
public export
badInterceptDestruction : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (IsDestroyed (Macros.target Macros.creature)) (Macros.exile It) (Just Macros.thisTurn) {ok})
badInterceptDestruction MkInterceptable impossible


||| "The next time target creature would die this turn, exile it instead."
||| A creature dies once, so the two shields would be the same shield
||| [CR#614.3]; the multiplicity word is the event's to choose.
public export
badNextTimeWouldDie : Unspellable (Effect []) (\ok =>
  Macros.nextTimeWouldInstead (Dies (Macros.target Macros.creature)) (Macros.exile It) (Just Macros.thisTurn) {uo = ok})
badNextTimeWouldDie MkReplUseOk impossible


||| "If target creature card in your graveyard would die, exile it instead this turn."
||| Dying is the battlefield-to-graveyard transition [CR#700.4], so the
||| watched object stands on the battlefield.
public export
badWouldDieInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok})
                 (Macros.exile It) (Just Macros.thisTurn))
badWouldDieInGraveyard MkZoneFits impossible


||| "The next time you would draw a card this turn, create a 1/1 green Soldier creature token instead. Sacrifice it."
||| The replacement is a HOLE outward: [CR#614.7] has one whose intercepted
||| event never happens "simply doesn't do anything".
public export
badInterceptReplacementAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.nextTimeWouldInstead (Draws You)
                                      (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Soldier]))
                                      (Just Macros.thisTurn)
               , Macros.sacrifice You (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}
               ])
badInterceptReplacementAntecedent (Refl, _) impossible


||| "Target creature gets +2/+2 until this creature leaves the battlefield."
||| No line ends a CONTINUOUS effect at a leaves-the-battlefield event with a
||| clause this grammar writes.
public export
badGetsUntilLeavesBattlefield : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) 2 2 (Just (UntilEvent (Leaves Macros.thisCreature))) {sp = ok})
badGetsUntilLeavesBattlefield SpanStated impossible


||| "Destroy target creature until this creature leaves the battlefield."
||| The rider goes on a zone change to EXILE and on nothing else; nothing
||| returns from a graveyard "until".
public export
badHeldUntilDestroy : Unspellable (Effect []) (\ok =>
  HeldUntil (Macros.destroy (Macros.target Macros.creature)) (Leaves Macros.thisCreature) {ok})
badHeldUntilDestroy MkHeldClause impossible


||| "Exile target creature until this creature dies."
||| And the event half of the same gate: the rider waits for a
||| leaves-the-battlefield event and for no other.
public export
badHeldUntilDies : Unspellable (Effect []) (\ok =>
  HeldUntil (Macros.exile (Macros.target Macros.creature)) (Dies Macros.thisCreature) {hd = ok})
badHeldUntilDies MkHoldable impossible


||| "Exile target creature with three time counters on it until this creature leaves the battlefield."
||| The exile the rider takes is the UNRIDDEN one: [CR#610.3] schedules a
||| return no ability has to ask for, where counters wait to be read.
public export
badHeldUntilWithCounters : Unspellable (Effect []) (\ok =>
  HeldUntil (Macros.exileWithCounters (Macros.target Macros.creature) (Lit 3) Time)
            (Leaves Macros.thisCreature) {ok})
badHeldUntilWithCounters MkHeldClause impossible


||| "Exile target creature until this creature leaves the battlefield. Put that card into your hand."
||| The undo is scheduled on an event that has not happened, so the clause
||| contributes its announcement and not the exile's retag.
public export
badHeldUntilExileRetag : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.exileUntil (Macros.target Macros.creature) (Leaves Macros.thisCreature)
               , Move (That CardW {ok}) Macros.handZ
               ])
badHeldUntilExileRetag Refl impossible


||| "If you would draw a card, draw two instead — and instead of that, draw three."
||| A replacement gets "only one opportunity to affect an event or any
||| modified events that may replace that event" [CR#614.5].
public export
badNestedInstead : Unspellable (Effect []) (\ok =>
  Macros.insteadOf (Macros.insteadOf Macros.drawACard (Macros.drawCards 2)) (Macros.drawCards 3) {na = ok})
badNestedInstead MkNotInstead impossible


||| "If this would deal 3 damage to any target, you gain that much life instead."
||| [CR#614.6] makes a replaced event "never happen", so its OUTCOME is not
||| there to read — though its announced target is [CR#601.2c].
public export
badInsteadReadsReplacedOutcome : Unspellable (Effect []) (\ok =>
  Macros.insteadOf (DealDamage This (Lit 3) (Macros.target AnyTarget))
            (Macros.gainsLife You (ThatMuch {ok})))
badInsteadReadsReplacedOutcome Refl impossible


||| "If this would deal 3 damage to target creature and draw a card, you gain that much life instead."
||| The same refusal through a SEQUENCE, where the announcement channel is a
||| hole rather than the last clause's post-state.
public export
badInsteadReadsReplacedSequenceOutcome : Unspellable (Effect []) (\ok =>
  Macros.insteadOf (Sequentially [DealDamage This (Lit 3) (Macros.target Macros.creature), Macros.drawACard])
            (Macros.gainsLife You (ThatMuch {ok})))
badInsteadReadsReplacedSequenceOutcome Refl impossible


||| "…you gain that much life instead." over a conditional wrapping an optional clause
||| And through that recursion too: the nested may's damage outcome is not
||| there to read either.
public export
badConditionalInsteadReadsMayOutcome : Unspellable (Effect []) (\ok =>
  Macros.insteadOf (If (Macros.may You (DealDamage This (Lit 2) (Macros.target Macros.creature)))
                (Exists Macros.creature)
                Nothing)
            (Macros.gainsLife You (ThatMuch {ok})))
badConditionalInsteadReadsMayOutcome Refl impossible


||| "Draw a card: Draw a card."
||| A draw is not a payment: [CR#602.1a] makes a cost what the ACTIVATOR
||| pays.
public export
badDrawAsCost : Unspellable Ability (\ok =>
  Activated (Do Macros.drawACard {ok}) Macros.drawACard)
badDrawAsCost MkCostAction impossible


||| "You gain 2 life: Draw a card."
||| The life row is DIRECTIONAL: paying life is a cost, and the cards
||| printing a gain-life one [CR#119.7] spell it as ALTERNATIVE [CR#118.9].
public export
badGainLifeCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.gainsLife You (Lit 2)) {ok}) Macros.drawACard)
badGainLifeCost MkCostAction impossible


||| "Sacrifice a creature, Exile the sacrificed card: Draw a card."
||| [CR#601.2h] pays the components in two tiers and each "in any order", so
||| no component may presuppose that a SIBLING has already been paid.
public export
badCostReadsSiblingDeed : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.sacrifice You (Macros.a Macros.creature)),
                       Do (Macros.exile (TheVerbed Sacrifice CardW)) {ok}])
            Macros.drawACard)
badCostReadsSiblingDeed MkCostAction impossible


||| "Sacrifice target creature: Draw a card."
||| [CR#601.2c] announces targets while the ability is still proposed and
||| [CR#601.2h] pays costs at the END, so the determiner goes past the colon.
public export
badTargetedCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice You (Macros.target Macros.creature)) {ok}) Macros.drawACard)
badTargetedCost MkCostAction impossible


||| "An opponent pays 2 life: Draw a card."
||| The payer is the ACTIVATOR: the activation cost "must be paid by the
||| player who is activating it" [CR#602.1a].
public export
badForeignPayerCost : Unspellable Ability (\ok =>
  Activated (Macros.payLife Macros.anOpponent 2) Macros.drawACard {py = ok})
badForeignPayerCost MkCostPaidByYou impossible


||| "An opponent sacrifices a creature: Draw a card."
||| The subjected cost verbs the same way: real English as a resolving
||| clause, and no payment of YOURS before a colon.
public export
badForeignSacrificeCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice Macros.anOpponent (Macros.a Macros.creature))) Macros.drawACard {py = ok})
badForeignSacrificeCost MkCostPaidByYou impossible


||| "You pay by sacrificing a creature."
||| "Pay" is one English VERB: a sacrifice is a payment [CR#118.1] and is not
||| payABLE — the non-mana unless lines write their own verb.
public export
badPayBySacrificing : Unspellable (Effect []) (\ok =>
  Pay You (Do (Macros.sacrifice You (Macros.a Macros.creature))) {pb = ok})
badPayBySacrificing MkPayable impossible


||| "you pay" over a component naming an opponent
||| The pay clause spells its subject ONCE, so the component under the verb
||| names the same player.
public export
badMismatchedPayer : Unspellable (Effect []) (\ok =>
  Macros.mayElse You (Pay You (Macros.payLife Macros.anOpponent 1) {ag = ok}) Macros.drawACard)
badMismatchedPayer MkPayAgrees impossible


||| "You pay {T}."
||| The tap symbol is a cost that exists only before a colon [CR#107.5], and
||| no sentence spells it as a verb phrase.
public export
badPayTapSymbol : Unspellable (Effect []) (\ok =>
  Pay You TapSymbol {pb = ok})
badPayTapSymbol MkPayable impossible


||| "You pay {1}, {T}."
||| No line writes "pay" over a comma-joined cost: the compound is a cost
||| SHAPE, not a complement English's pay-verb takes.
public export
badPayCompound : Unspellable (Effect []) (\ok =>
  Pay You (Compound [Mana [Macros.generic 1], TapSymbol]) {pb = ok})
badPayCompound MkPayable impossible


||| "Target creature gains '{1}: Draw a card' until end of turn."
||| English grants an activated ability by QUOTING it, and this grammar has
||| no quotation.
public export
badGainsActivated : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature)
               (Activated (Mana [Macros.generic 1]) Macros.drawACard)
               (Just Macros.untilEndOfTurn) {gr = ok})
badGainsActivated MkGrantable impossible


||| a compound written as one element of a compound
||| A compound's ELEMENTS are components: nesting re-mints the right-nested
||| tree the telescope replaced.
public export
badNestedCompound : Unspellable Ability (\ok =>
  Activated (Compound ((Compound [Mana [Macros.generic 1], TapSymbol] :: (TapSymbol :: Nil)) {nc = ok}))
            Macros.drawACard)
badNestedCompound MkNotCompound impossible


||| "{1}:" written as a one-element compound
||| A compound of one is the component itself spelled a second way — the
||| singleton refusal at the cost layer.
public export
badSingletonCompound : Unspellable Ability (\ok =>
  Activated (Compound [Mana [Macros.generic 1]] {two = ok}) Macros.drawACard)
badSingletonCompound TwoUp impossible


||| "{T}, {T}: Draw a card."
||| One self-tap per cost: "a permanent that's already tapped can't be
||| tapped again to pay the cost" [CR#107.5,118.3].
public export
badDoubleTapCost : Unspellable Ability (\ok =>
  Activated (Compound [TapSymbol, TapSymbol]) Macros.drawACard {tp = ok})
badDoubleTapCost MkCostTapOnce impossible


||| ": Draw a card." opening on an empty symbol run
||| A component's symbol RUN is written: the empty list is real on a CARD
||| [CR#202.1b], but the payment of nothing before a colon is "{0}" [CR#118.5].
public export
badEmptyManaCost : Unspellable Ability (\ok =>
  Activated (Mana [] {wr = ok}) Macros.drawACard)
badEmptyManaCost MkManaRun impossible


||| "{W/W/P}"
||| A hybrid Phyrexian symbol names two DIFFERENT colors, [CR#107.4f]
||| counting ten of them — the unordered pairs of five distinct colors.
public export
badSameColorPhyrexian : Unspellable ManaSymbol (\ok =>
  Phyrexian White (Just White) {ds = ok})
badSameColorPhyrexian MkPhyrexianDistinct impossible


||| "{U/U}"
||| [CR#107.4e] makes the symbol "a cost that can be paid in one of two
||| ways", and two ways spelled the same word is one way.
public export
badSameColorHybrid : Unspellable ManaSymbol (\ok =>
  Macros.hybridPip Blue Blue {ds = ok})
badSameColorHybrid MkHalvesDistinct impossible


||| "Sacrifice a creature. If you don't, exile it."
||| The declined arm runs only when the payment was never started
||| [CR#118.12], so the body's phrase named nothing it can read.
public export
badIfNotReadsMandatoryBody : Unspellable (Effect []) (\ok =>
  Macros.doElse (Macros.sacrifice You (Macros.a Macros.creature)) (Macros.exile (It {ok})))
badIfNotReadsMandatoryBody Refl impossible


||| "Tap target creature unless its controller pays {1}."
||| [CR#118.12a]'s rewrite puts the may first, so the payer phrase is typed
||| before the clause that announces what it reads.
public export
badUnlessAnaphoricPayer : Unspellable (Effect []) (\ok =>
  Macros.mayElse (ControllerOf (It {ok})) (Pay You (Mana [Macros.generic 1])) (Tap (Macros.target Macros.creature)))
badUnlessAnaphoricPayer Refl impossible


||| "At this creature enters, draw a card."
||| [CR#603.2b] gives "at the beginning of" a phase or step its own clause;
||| no object event takes the word.
public export
badAtEnters : Unspellable Ability (\ok =>
  Triggered At (Enters Macros.thisCreature) Macros.drawACard {wo = ok})
badAtEnters MkTriggerWordOk impossible


||| "When this enters, draw a card."
||| The type word is what places the referent [CR#109.2]; bare "this" is the
||| source as an object and stands nowhere.
public export
badEntersBareThis : Unspellable Ability (\ok =>
  Triggered When (Enters This {ss = ok}) Macros.drawACard)
badEntersBareThis MkSelfSorted impossible


||| "Whenever target creature dies, draw a card."
||| An ordinary trigger's header announces no TARGET: [CR#115.1d] chooses
||| them "as the ability is put on the stack", after the event.
public export
badTargetedDeathHeader : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.target Macros.creature)) Macros.drawACard {hn = ok})
badTargetedDeathHeader MkHeaderNontarget impossible


||| "When the beginning of your upkeep, draw a card."
||| The inverse half that makes the table a table: the turn-part beginning
||| takes neither English word.
public export
badWhenUpkeep : Unspellable Ability (\ok =>
  Triggered When (BeginningOf Upkeep (Just Yours)) Macros.drawACard {wo = ok})
badWhenUpkeep MkTriggerWordOk impossible


||| "Whenever a creature is destroyed, draw a card."
||| Real oracle English in one mood and none in the other: the phrase is
||| regeneration's [CR#614.8], and the header for that event is "dies".
public export
badTriggerOnDestruction : Unspellable Ability (\ok =>
  Triggered Whenever (IsDestroyed (Macros.a Macros.creature)) Macros.drawACard
            {tr = Builtin.fst ok, wo = Builtin.snd ok})
badTriggerOnDestruction (MkTriggerable, _) impossible


||| "At the beginning of the untap step, draw a card."
||| The untap step's beginning is written zero times in every possession —
||| the turn-part grid's emptiest row.
public export
badTriggerAtUntapStep : Unspellable Ability (\ok =>
  Triggered At (BeginningOf UntapStep Nothing {pu = ok}) Macros.drawACard)
badTriggerAtUntapStep MkPartTriggerable impossible


||| "At the beginning of your turn, draw a card."
||| Nor the turn's own beginning: the UPKEEP is what English names there.
public export
badTriggerAtYourTurn : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Turn (Just Yours) {pu = ok}) Macros.drawACard)
badTriggerAtYourTurn MkPartTriggerable impossible


||| "At the beginning of the upkeep, draw a card."
||| The headers that look unpossessed name a quantifier or a nominal
||| possessor, and this vocabulary's is a two-word pronominal one.
public export
badTriggerAtTheUpkeep : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Upkeep Nothing {pu = ok}) Macros.drawACard)
badTriggerAtTheUpkeep MkPartTriggerable impossible


||| "Whenever a creature dies, tap it."
||| [CR#603.6] has a zone-change trigger "look for the object in the zone
||| that it moved to", so the referent is a card in a graveyard [CR#701.26a].
public export
badTriggerTapsDeadCreature : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.a Macros.creature)) (Tap It {ok}))
badTriggerTapsDeadCreature OnField impossible


||| "Whenever a creature leaves the battlefield, tap it."
||| [CR#603.6c] checks the object "only in the first zone that it went to",
||| which the sentence never names: the binding survives, its zone does not.
public export
badLeavesThenTap : Unspellable Ability (\ok =>
  Triggered Whenever (Leaves (Macros.a Macros.creature)) (Tap It {ok}))
badLeavesThenTap OnField impossible


||| "Target creature can't attack." as a static ability line
||| A static ability does not TARGET [CR#115.1a..115.1e]; of the nearest case
||| [CR#115.1b] says "an Aura permanent doesn't target anything".
public export
badStaticTargets : Unspellable Ability (\ok =>
  Static (Deontic (Macros.target Macros.creature) Forbid Attack Agent Nothing) {ut = ok})
badStaticTargets MkUntargeting impossible


||| "You gain control of all creatures you control." as a static line
||| The one static effect English does not state as a line: its stative form
||| is a different VERB, "You control enchanted creature".
public export
badStaticGainsControl : Unspellable Ability (\ok =>
  Static (GainsControl You (AllOf Macros.creatureYouControl)) {ln = ok})
badStaticGainsControl MkStaticLine impossible


||| "As long as you control an artifact, you gain control of a creature."
||| The wrapper launders nothing: [CR#604.1] has a static ability "written
||| as a statement", so qualifying an unstatable one leaves it unstatable.
public export
badConditionalGainControl : Unspellable Ability (\ok =>
  Static (Macros.asLongAs (Exists Macros.artifact) (GainsControl You (Macros.a Macros.creature))) {ln = ok})
badConditionalGainControl MkStaticLine impossible


||| "As long as you control a creature, creatures you control get +1/+1." as a clause
||| The conditional static is an ability line and nothing else: [CR#611.2b]'s
||| "FOR as long as" is the duration, [CR#611.3a]'s bare one is this.
public export
badConditionalClause : Unspellable (Effect []) (\ok =>
  Continuously (Macros.asLongAs (Exists Macros.creatureYouControl)
                         (Gets (AllOf Macros.creatureYouControl) 1 1))
               Nothing {sp = ok})
badConditionalClause SpanUnstated impossible


||| a statement conditioned twice
||| No line conditions a statement twice — the singleton discipline of the
||| cost layer, one type up.
public export
badDoubleConditional : Unspellable Ability (\ok =>
  Static (Macros.asLongAs (Exists Macros.creatureYouControl)
                   (Macros.asLongAs (Exists (And [Macros.artifact, ControlledBy You]))
                             (Gets (AllOf Macros.creatureYouControl) 1 1)) {nn = ok}))
badDoubleConditional MkNotConditional impossible


||| "Creatures you control enter tapped." written as a clause
||| The entry rider is a static ability and not a clause [CR#603.6d]; the
||| one-shot twin is a rider on a MOVE.
public export
badEntryRiderClause : Unspellable (Effect []) (\ok =>
  Continuously (Macros.entersTapped (AllOf Macros.creatureYouControl)) Nothing {sp = ok})
badEntryRiderClause SpanUnstated impossible


||| "This land enters attacking."
||| The line writes ONE of the two riders: a permanent's own static ability
||| applies whenever it enters, from any zone in any step.
public export
badEntersAttackingLine : Unspellable Ability (\ok =>
  Static (EntersRider (AsType Land This) EntersAttacking {ro = ok}))
badEntersAttackingLine MkEntryRiderOk impossible


||| "Exile the top card of your library. You may play it." with no duration
||| The permission writes spans freely and states none only when it is the
||| card's own line.
public export
badStandingPermission : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile Macros.topCard, Continuously (MayPlay You It) Nothing {sp = ok}])
badStandingPermission SpanUnstated impossible


||| "Exile the top card of your library. You may play it until end of combat."
||| Nor at an endpoint the permission does not write: "until end of combat"
||| is the two GRANTS' word.
public export
badPermissionUntilEndOfCombat : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile Macros.topCard,
                Continuously (MayPlay You It) (Just Macros.untilEndOfCombat) {sp = ok}])
badPermissionUntilEndOfCombat SpanStated impossible


||| "Target creature gains flying until your next end step."
||| That cell is the PERMISSION's alone: every line writing it permits
||| playing just-exiled cards, and no grant writes it.
public export
badGainsUntilYourNextEndStep : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying) (Just Macros.untilYourNextEndStep) {sp = ok})
badGainsUntilYourNextEndStep SpanStated impossible


||| "You may play a creature this turn." of a battlefield permanent
||| A permanent on the battlefield has already been played, and [CR#604.6]
||| files the permission where "you could cast or play it from".
public export
badPlayFromBattlefield : Unspellable (Effect []) (\ok =>
  Continuously (MayPlay You (Macros.a Macros.creature) {pz = ok}) (Just Macros.thisTurn))
badPlayFromBattlefield MkPlaySource impossible


||| "Target creature gains 'When this creature enters, draw a card' until end of turn."
||| English grants a triggered ability by QUOTING it, exactly as it grants an
||| activated one, and this grammar has no quotation.
public export
badGainsTriggered : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (Triggered When (Enters Macros.thisCreature) Macros.drawACard)
        (Just Macros.untilEndOfTurn) {gr = ok})
badGainsTriggered MkGrantable impossible


||| "Target creature gains 'Creatures you control can't attack' until end of turn."
||| And a static ability the same way.
public export
badGainsStatic : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (Static (Deontic (AllOf Macros.creatureYouControl) Forbid Attack Agent Nothing))
        (Just Macros.untilEndOfTurn) {gr = ok})
badGainsStatic MkGrantable impossible


||| "When target creature dies until end of turn, return that card to the battlefield."
||| [CR#603.7b] gives a delayed trigger one stated duration and names the
||| phrase: "unless it has a stated duration, such as 'this turn'".
public export
badDelayedUntilEndOfTurn : Unspellable (Effect []) (\ok =>
  Delayed (Dies (Macros.target Macros.creature)) {span = Just Macros.untilEndOfTurn}
          (Move (That CardW) Macros.battlefieldZ) {so = ok})
badDelayedUntilEndOfTurn DelayFor impossible


||| "When target creature attacks, sacrifice that creature."
||| The delayed clause reads three events of the ten, and the attack is not
||| one: the family is the end-step beginning, the departure and the death.
public export
badDelayedOnAttack : Unspellable (Effect []) (\ok =>
  Delayed (Attacks (Macros.target Macros.creature)) (Macros.sacrifice You (That (TypeW Creature))) {aw = ok})
badDelayedOnAttack MkAwaitable impossible


||| "If a creature would enter, exile it instead this turn."
||| The enter-interceptions the corpus writes are the entry RIDER
||| [CR#603.6d], so the would-clause has nothing left to say about the event.
public export
badInterceptEnters : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (Enters (Macros.a Macros.creature)) (Macros.exile It) (Just Macros.thisTurn)
                        {ok = Builtin.fst ok, uo = Builtin.snd ok})
badInterceptEnters (MkInterceptable, _) impossible


||| "Exile target creature until a creature enters."
||| Nor does the [CR#610.3] rider wait for one: its whole corpus is the
||| departure.
public export
badHeldUntilEnters : Unspellable (Effect []) (\ok =>
  Macros.exileUntil (Macros.target Macros.creature) (Enters (Macros.a Macros.creature)) {hd = ok})
badHeldUntilEnters MkHoldable impossible


||| "Target creature gets +3/+3 until the beginning of your next upkeep." on the event axis
||| One phrase, one slot: the duration adverbial's own row already spells
||| this endpoint, so the event axis must not spell it a second way.
public export
badUntilBeginningOfUpkeep : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) 3 3 (Just (UntilEvent (BeginningOf Upkeep (Just Yours)))) {sp = ok})
badUntilBeginningOfUpkeep SpanStated impossible


||| "This creature deals 3 damage to any target. When you do, draw a card."
||| The anaphor is a PRO-VERB whose subject is a player, and [CR#120.1] gives
||| a damage clause an OBJECT as its agent.
public export
badReflexiveOnSourceDeed : Unspellable (Effect []) (\ok =>
  Reflexively (DealDamage This (Lit 3) (Macros.target AnyTarget)) Macros.drawACard {en = ok})
badReflexiveOnSourceDeed MkReflexEnclosure impossible


||| "You gain 2 life. When you do, draw a card."
||| [CR#119.9] rewrites the trigger as "whenever a source causes [a player]
||| to gain life", making the player the PATIENT with no action to inflect.
public export
badReflexiveOnLifeGain : Unspellable (Effect []) (\ok =>
  Reflexively (Macros.gainsLife You (Lit 2)) Macros.drawACard {en = ok})
badReflexiveOnLifeGain MkReflexEnclosure impossible


||| "Draw a card, then sacrifice a creature. When you do, draw a card."
||| One verb phrase, because "do" abbreviates one; a card with two sentences
||| before the anaphor hangs it on the LAST of them.
public export
badReflexiveOnSequence : Unspellable (Effect []) (\ok =>
  Reflexively (Sequentially [Macros.drawACard, Macros.sacrifice You (Macros.a Macros.creature)]) Macros.drawACard {en = ok})
badReflexiveOnSequence MkReflexEnclosure impossible


||| "At the beginning of your next end step, draw a card. When you do, draw a card."
||| A clause that SCHEDULES its action has not taken it, and [CR#603.12]
||| triggers on whether the event "occurred earlier during the resolution".
public export
badReflexiveOnDelayed : Unspellable (Effect []) (\ok =>
  Reflexively (Delayed (BeginningOf EndStep (Just Yours)) Macros.drawACard) Macros.drawACard {en = ok})
badReflexiveOnDelayed MkReflexEnclosure impossible


||| "You may sacrifice a creature. If you do, draw a card. When you do, draw a card."
||| One offer, one reader: [CR#118.12]'s "if you do" and [CR#603.12]'s "when
||| you do" ask the same question of the same choice.
public export
badReflexiveOnBranchedMay : Unspellable (Effect []) (\ok =>
  Reflexively (Macros.mayThen You (Macros.sacrifice You (Macros.a Macros.creature)) Macros.drawACard) Macros.drawACard {en = ok})
badReflexiveOnBranchedMay MkReflexEnclosure impossible


||| "You may sacrifice a creature. If you don't, draw a card. When you do, draw a card."
||| The declined arm is refused by the same row: [CR#603.12] licenses a
||| negative reflexive, but English writes it zero times.
public export
badReflexiveOnDeclinedMay : Unspellable (Effect []) (\ok =>
  Reflexively (Macros.mayElse You (Macros.sacrifice You (Macros.a Macros.creature)) Macros.drawACard) Macros.drawACard {en = ok})
badReflexiveOnDeclinedMay MkReflexEnclosure impossible


||| "Target opponent gains control of this. When they do, draw a card."
||| The family's one real line establishes a continuous effect instead of
||| naming an action, and wants a counter kind this vocabulary lacks.
public export
badReflexiveOnGainsControl : Unspellable (Effect []) (\ok =>
  Reflexively (Macros.gainsControl (Macros.target Opponent) This Nothing) Macros.drawACard {en = ok})
badReflexiveOnGainsControl MkReflexEnclosure impossible


||| "Mill four cards. When you do, create a 1/1 white Soldier creature token. Tap that creature."
||| [CR#603.3] puts a triggered ability on the stack "the next time a player
||| would receive priority", so the resolution finishes before it runs.
public export
badAfterReflexiveReadsTrigger : Unspellable (Effect []) (\ok =>
  Sequentially [Reflexively (Macros.millCards 4)
                            (Macros.create (Lit 1) (MkToken (Just (1, 1)) [White]
                                                     (MkTypeLine [Soldier] [Creature])
                                                     [] Nothing)),
                Tap (That (TypeW Creature) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badAfterReflexiveReadsTrigger (Refl, _) impossible


||| "Sacrifice a creature. When you do, tap it."
||| INWARD the reflexive reads the enclosure's post-state, the action having
||| DONE, so the sacrificed creature is already in its graveyard.
public export
badReflexiveTapsSacrificed : Unspellable (Effect []) (\ok =>
  Reflexively (Macros.sacrifice You (Macros.a Macros.creature)) (Tap It {ok}))
badReflexiveTapsSacrificed OnField impossible


||| "Put target creature into its owner's graveyard tapped."
||| An arrival rider is BATTLEFIELD-only: [CR#110.5b] states the default the
||| rider overrides, and [CR#110.5] gives status to permanents alone.
public export
badMoveRidersToGraveyard : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.graveyardZ
       {riders = MkMoveRiders [EntersTapped] Nothing} {rf = ok})
badMoveRidersToGraveyard MkRidersFit impossible


||| "Put target creature into its owner's hand under your control."
||| An object neither on the stack nor on the battlefield "aren't controlled
||| by any player" [CR#109.4], so a hand arrival has no controller to name.
public export
badMoveControlToHand : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.handZ {riders = MkMoveRiders [] (Just You)} {rf = ok})
badMoveControlToHand MkRidersFit impossible


||| "Put target creature onto the battlefield attacking."
||| Attacking alone is written zero times in either frame: such a creature is
||| put there tapped as well.
public export
badMoveAttackingUntapped : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.battlefieldZ
       {riders = MkMoveRiders [EntersAttacking] Nothing {ro = ok}})
badMoveAttackingUntapped MkRidersOk impossible


||| "Put target creature onto the battlefield attacking and tapped."
||| And the order is fixed there too, no line writing "attacking and tapped".
public export
badMoveRidersReversed : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.battlefieldZ
       {riders = MkMoveRiders [EntersAttacking, EntersTapped] Nothing {ro = ok}})
badMoveRidersReversed MkRidersOk impossible


||| "Put target creature onto the battlefield under the other players' control."
||| ONE controller [CR#109.4], the plural relational being a spelling this
||| vocabulary does not have.
public export
badMoveRidersPluralController : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.battlefieldZ
       {riders = MkMoveRiders [] (Just (AllOf Macros.otherPlayer)) {one = ok}})
badMoveRidersPluralController OneController impossible


||| "Put target creature into its owner's graveyard with a +1/+1 counter on it."
||| The COUNTER rider is gated apart from the other two: [CR#122.1a]'s
||| other-zone clause is real, but no line writes one into a graveyard.
public export
badMoveCountersToGraveyard : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.graveyardZ
       {riders = MkMoveRiders [] Nothing
                              {counters = Just (MkCounterRider (Lit 1) Macros.plusOnePlusOne)}} {rf = ok})
badMoveCountersToGraveyard MkRidersFit impossible


||| "Exile target creature with zero time counters on it."
||| The rider's count is a WRITTEN magnitude like every other, so the
||| unwritable zero is unwritable here too.
public export
badExileZeroCounters : Unspellable (Effect []) (\ok =>
  Macros.exileWithCounters (Macros.target Macros.creature) (Lit 0) Time {wc = ok})
badExileZeroCounters MkWrittenCount impossible


||| "cards exiled with target creature"
||| A linkage read names the exiles of the ability's OWN object, [CR#607.1]
||| building the relation out of two abilities "printed on it" [CR#406.6].
public export
badExiledWithOtherSource : Unspellable (Predicate [] Object) (\ok =>
  ExiledWith (Macros.target Macros.creature) {ls = ok})
badExiledWithOtherSource SelfLinked impossible


||| "a card exiled with an artifact"
||| A DESCRIBED source is refused by the same gate: the linkage is one
||| object's note about its own exiles, not a relation to whatever exiled it.
public export
badExiledWithDescribedSource : Unspellable (Predicate [] Object) (\ok =>
  ExiledWith (Macros.a Macros.artifact) {ls = ok})
badExiledWithDescribedSource SelfLinked impossible


||| "a card you control exiled with this artifact"
||| The linked cards are IN EXILE [CR#607.2a], and an object neither on the
||| stack nor on the battlefield has no controller at all [CR#109.4].
public export
badExiledWithControlled : Unspellable (Predicate [] Object) (\ok =>
  And [ControlledBy You, Macros.exiledWithThisArtifact] {zc = ok})
badExiledWithControlled MkZoneCoherent impossible
