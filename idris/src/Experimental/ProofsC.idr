module Experimental.ProofsC

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off

-- Continuation of Experimental.ProofsB; see Unspellable there.


-- The control grant writes the GRANTS' current-turn word and never the
-- restrictions': "gain control of target creature this turn" is written
-- zero times, and the four corpus lines pairing the two words are event
-- clauses ("that attacked you this turn") rather than adverbials.
public export
badGainControlThisTurn : Unspellable (Effect []) (\ok =>
  Macros.gainControl (Macros.target Macros.creature) (Just Macros.thisTurn) {sp = ok})
badGainControlThisTurn SpanStated impossible


-- "Until the end of your next turn" is the control grant's ALONE — the
-- cell chapter seventeen left `Unclaimed` and this chapter claimed. The
-- keyword grant does not write it (the eighty-three lines beside the
-- four control ones are play permissions, another construction's).
public export
badKeywordGrantEndOfNextTurn : Unspellable (Effect []) (\ok =>
  Macros.gainsHaste (Macros.target Macros.creature) (Just (Until (EndOf Turn (Just Yours)))) {sp = ok})
badKeywordGrantEndOfNextTurn SpanStated impossible


-- And "for as long as" is written by every construction but the type
-- ADDITION: the thirteen "becomes … for as long as" corpus lines are
-- type SETTINGS and copies, and the addition's own phrase pairs with
-- the adverbial zero times.
public export
badTypeAdditionForAsLongAs : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact])
          (Just (ForAsLongAs (Matches Macros.thisCreature (ControlledBy You)))) {sp = ok})
badTypeAdditionForAsLongAs SpanStated impossible


-- A distributed creation exports a PLURAL mention, so the singular
-- pronoun has nothing to resolve to: "Each player creates a 1/1 green
-- Plant creature token. Put a +1/+1 counter on IT" is unwritable, and it
-- is the same clause that reads back fine undistributed (Additive
-- Evolution's "create a 0/0 … Fractal creature token. Put three +1/+1
-- counters on it."). The count is one in both.
public export
badDistributedCreationIt : Unspellable (Effect []) (\ok =>
  Sequentially [Create (Each AnyPlayer) (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant]) [],
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badDistributedCreationIt (_, MkCounterHolder) impossible


-- …and it is a group MENTION and not a description: "each of each
-- creature" is written zero times, the plain distributive being what a
-- description takes.
public export
badEachOfDistributive : Unspellable (Noun [] Object) (\ok =>
  EachOf (Each Macros.creature) {gm = ok})
badEachOfDistributive MkGroupMention impossible


-- …nor the universal, for the same reason and with the same count:
-- "each of all creatures" is written zero times.
public export
badEachOfAll : Unspellable (Noun [] Object) (\ok =>
  EachOf (AllOf Macros.creature) {gm = ok})
badEachOfAll MkGroupMention impossible


-- …and it does not stack: one determiner fills the position, and "each
-- of each of them" spells nothing twice.
public export
badNestedEachOf : Unspellable (Noun [] Object) (\ok =>
  EachOf (EachOf (TargetGroup (Macros.upTo 2) Macros.creature)) {gm = ok})
badNestedEachOf MkGroupMention impossible


-- A clause writing ONE per-member amount refuses a bare plural
-- recipient: "put a +1/+1 counter on up to two target creatures" is the
-- sentence the corpus never writes, and "each of" is exactly the word it
-- writes instead (zero lines at two, three, or four against
-- twenty-nine at "each of up to two target creatures").
public export
badBarePluralCounterRecipient : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) Macros.plusOnePlusOne (TargetGroup (Macros.upTo 2) Macros.creature) {pm = ok})
badBarePluralCounterRecipient MkPerMember impossible


-- …and the damage verb reads the same way: "deals 1 damage to up to two
-- target creatures" is unwritten, while "to up to ONE target creature"
-- is twenty-eight lines and singular already.
public export
badBarePluralDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (TargetGroup (Macros.upTo 2) Macros.creature) {pm = ok})
badBarePluralDamageRecipient MkPerMember impossible


-- …and a plural READ is no better than a plural mention: the corpus's
-- "counters on them" lines are all relative clauses ("cards with intel
-- counters on them"), never a recipient.
public export
badThemCounterRecipient : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (TargetGroup Macros.anyNumber Macros.creature),
                PutCounters (Lit 1) Macros.plusOnePlusOne Them {pm = ok}])
badThemCounterRecipient MkPerMember impossible


-- …and the universal determiner with them: "deals 2 damage to all
-- creatures" is zero lines, the sweep being written distributively
-- ("damage to each creature", two hundred thirty-five).
public export
badAllOfDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (AllOf Macros.creature) {pm = ok})
badAllOfDamageRecipient MkPerMember impossible


-- …and the members must be a MENTION and not a description, since
-- [CR#601.2d] has the caster announce the division over the targets they
-- announced: "divided as you choose among each creature" is unwritten,
-- and every corpus line writes a counted target group or a plural read.
public export
badDivideAmongDescription : Unspellable (Effect []) (\ok =>
  Macros.dealsDivided This (Lit 2) (Each Macros.creature) {gm = ok})
badDivideAmongDescription MkGroupMention impossible


-- …and a division of nothing instructs nothing, which is the written-count
-- discipline reaching the new clause.
public export
badDivideZero : Unspellable (Effect []) (\ok =>
  Macros.dealsDivided This (Lit 0) (TargetGroup (Macros.oneThrough 2) AnyTarget) {wc = ok})
badDivideZero MkWrittenCount impossible


-- …and the counter half keeps the battlefield demand its undivided twin
-- carries: no corpus line distributes counters onto cards in a
-- graveyard.
public export
badDistributeCountersGraveyard : Unspellable (Effect []) (\ok =>
  Macros.distributeCounters (Lit 2) Macros.plusOnePlusOne
                     (TargetGroup (Macros.oneThrough 2) (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok})
badDistributeCountersGraveyard OnField impossible


-- A library is ORDERED ([CR#401.2]), so "into your library" names no
-- place to put a card and oracle never writes it: every corpus
-- placement spells a position. `DestOk` has no row for the bare zone,
-- which is core's `exclude(Library)` on `Destination` exactly.
public export
badMoveToBareLibrary : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) (ZoneAt Library Bare) {ok})
badMoveToBareLibrary BattlefieldOk impossible


-- The order rider needs two or more cards to order — [CR#401.4]'s own
-- condition, and English's: "put it on the bottom of your library in
-- any order" is zero lines.
public export
badSingularOrderRider : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt Macros.topCard, Move (That CardW) (Macros.onBottomIn AnyOrder) {arr = ok}])
badSingularOrderRider MkArrangementOk impossible


-- "The rest" of WHAT: with no group in the discourse the complement has
-- nothing to be the rest of.
public export
badRestWithoutGroup : Unspellable (Effect []) (\ok =>
  Move (TheRest {ok}) Macros.graveyardZ)
badRestWithoutGroup Refl impossible


-- …and with a group but nothing taken out of it, "the rest" IS the
-- group and the sentence would have written "them".
public export
badRestWithoutPart : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Move (TheRest {ok}) Macros.onBottomZ])
badRestWithoutPart Refl impossible


-- One disposition per remainder, per path. "The rest" names what is
-- OUTSTANDING of a group, and once it has been placed nothing is:
-- Impulse writes one partition and one disposition, and the only three
-- corpus lines carrying two "the rest" phrases put them in mutually
-- exclusive if/instead/otherwise arms, which this refusal leaves
-- writable because a branch arm is typed in the discourse BEFORE the
-- disposition. The move spends the group (`groupSpent`).
public export
badRestDisposedTwice : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt (Macros.topCards 4)
               , Move (Macros.oneOf Them) Macros.handZ
               , Move TheRest Macros.onBottomZ
               , Move (TheRest {ok}) Macros.graveyardZ
               ])
badRestDisposedTwice Refl impossible


-- The direction chapter twenty-two refused, and it stays refused: two
-- separately announced targets are two mentions and never a pair
-- ([CR#601.2c], finding 127), so no complement can subtract inside
-- them. What changed is that a group ASSEMBLED by one phrase now
-- exists — not that announcements can be added up.
public export
badRestOverTwoAnnouncements : Unspellable (Effect []) (\ok =>
  Sequentially [ Fights (Macros.target Macros.creatureYouControl) (Macros.target Macros.creatureYouDontControl)
               , Move (TheRest {ok}) Macros.graveyardZ
               ])
badRestOverTwoAnnouncements Refl impossible


-- A partitive is a selection, but the corpus names its chooser every
-- time ("an opponent chooses two of them") and `Choose` has no agent
-- slot; an agentless "Choose one of them" is unwritten English.
public export
badChooseSomeOf : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Choose (Macros.oneOf Them) {ch = ok}])
badChooseSomeOf MkChoosable impossible


-- A shuffle randomizes the library and destroys what the discourse
-- knew about it ([CR#701.24a]; [CR#701.20d] makes a reordered revealed
-- card a NEW object), so a card still in the library cannot be read
-- back afterwards. Every search sentence places its find first for
-- exactly this reason.
public export
badReadAfterShuffle : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.searchLibraryFor Macros.land, Macros.shuffle, Move (It {ok}) Macros.handZ])
badReadAfterShuffle Refl impossible


-- The slice is in a library, so every battlefield-demanding verb
-- refuses it through the demand it already carried.
public export
badTapLibraryTop : Unspellable (Effect []) (\ok =>
  Tap Macros.topCard {ok})
badTapLibraryTop OnField impossible


-- …and it describes no card, which is the hidden zone's honesty made
-- structural ([CR#400.2,401.2]): the phrase names a place, so a typed
-- demonstrative has nothing to reach.
public export
badSliceTypeRead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Move (Those (TypeW Creature) {ok}) Macros.handZ])
badSliceTypeRead Refl impossible


-- A library is not shown whole: "reveal your library" is zero lines,
-- and what oracle exposes of a library is a positioned slice.
public export
badRevealWholeLibrary : Unspellable (Effect []) (\ok =>
  Expose Reveal You (ExposedZone Macros.yourLibrary {ok}))
badRevealWholeLibrary MkExposableZone impossible


-- Nor is a public zone: a graveyard is already visible to everyone
-- ([CR#400.2]), so revealing one says nothing and no line does it.
public export
badRevealGraveyard : Unspellable (Effect []) (\ok =>
  Expose Reveal You (ExposedZone Macros.graveyardZ {ok}))
badRevealGraveyard MkExposableZone impossible


-- Nor is the battlefield searched: [CR#701.23a] is about finding a card
-- among cards you cannot otherwise read.
public export
badSearchBattlefield : Unspellable (Effect []) (\ok =>
  Search You Macros.battlefieldZ Macros.creature {sz = ok})
badSearchBattlefield MkSearchableZone impossible


-- Nor is a POSITION in a library a zone a search looks through, which is
-- the same rule read one step further in: [CR#701.23a] looks at "all
-- cards in that zone" and [CR#401.2] makes the library "a single
-- face-down pile", so "the top of your library" is a place in the zone
-- and not the zone. The sort alone could not tell them apart —
-- `zoneSort` projects `Library` off either — so the clause asks the
-- PHRASE as well (`WholeZone`). The arrangement rider makes the nonsense
-- plain: a search cannot look through cards "in a random order".
public export
badSearchLibraryPosition : Unspellable (Effect []) (\ok =>
  Search You (LibraryAt OnTop Nothing Bare) Macros.land {wz = ok})
badSearchLibraryPosition MkWholeZone impossible


public export
badSearchLibraryPositionOrdered : Unspellable (Effect []) (\ok =>
  Search You (LibraryAt OnBottom (Just RandomOrder) Bare) Macros.creature {wz = ok})
badSearchLibraryPositionOrdered MkWholeZone impossible


-- The search's description is the zone's, not another zone's: the
-- clause supplies the place, so a phrase carrying its own is two
-- answers to one question.
public export
badSearchZonedDescription : Unspellable (Effect []) (\ok =>
  Macros.searchLibraryFor (And [Macros.creature, InZone Macros.graveyardZ]) {zf = ok})
badSearchZonedDescription MkZoneFree impossible


-- A zero slice and a zero mill instruct nothing, which is
-- `WrittenCount`'s discipline reaching two more counts.
public export
badZeroSlice : Unspellable (Effect []) (\ok =>
  Macros.lookAt (Macros.topCards 0 {wc = ok}))
badZeroSlice MkWrittenCount impossible


public export
badMillZero : Unspellable (Effect []) (\ok =>
  Macros.millCards 0 {wc = ok})
badMillZero MkWrittenCount impossible


-- A DISTRIBUTED mill leaves a plural group, and the count is not what
-- says so — the subject is. [CR#701.17a] has each milled-at player put
-- that many cards from their own library into their own graveyard, so
-- "each player mills a card" puts one card per player there and the
-- sentence after it cannot say "it". Locke, Treasure Hunter reads the
-- group plural in the next breath ("each player mills a card. … you may
-- cast a spell from among those cards"; the trigger shell and the
-- among-restriction are ledgered). Derived exactly as `Create`'s
-- distributed count is (`outputPlur`).
public export
badDistributedMillSingular : Unspellable (Effect []) (\ok =>
  Sequentially [Mill (Each AnyPlayer) (Lit 1), Macros.exile (It {ok})])
badDistributedMillSingular Refl impossible


-- A partitive reaches into a GROUP, not into a description: "one of a
-- creature you control" is not English, and the determiner has no
-- members to pick from until some phrase has fixed them.
public export
badPartitiveOfDescription : Unspellable (Effect []) (\ok =>
  Macros.exile (SomeOf (Macros.exactly 1) (Macros.a Macros.creature) {gm = ok}))
badPartitiveOfDescription MkGroupMention impossible


-- …nor into another partitive: a part is what was taken, not a group to
-- take from ("two of one of them" is zero lines).
public export
badPartitiveOfPartitive : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.lookAt (Macros.topCards 4), Macros.exile (Macros.oneOf (Macros.oneOf Them) {gm = ok})])
badPartitiveOfPartitive MkGroupMention impossible


-- …nor into the complement: "each of the rest" is zero lines, the
-- remainder being named rather than reached into.
public export
badEachOfTheRest : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt (Macros.topCards 4)
               , Move (Macros.oneOf Them) Macros.handZ
               , Macros.exile (EachOf TheRest {gm = ok})
               ])
badEachOfTheRest MkGroupMention impossible


-- A durationless interception is the STATIC ABILITY line, not a clause.
-- "If a creature an opponent controls would die, exile it instead" is a
-- permanent's own ability ([CR#611.3] — a continuous effect from a
-- static ability lasts while the ability functions and states no
-- duration), and the corpus divides cleanly: every ONE-SHOT
-- interception writes a span. So `absentOk Replacement` is `False` and
-- the whole clause is unwritable here, which is `badStaticCant`'s
-- refusal a third time.
public export
badStandingIntercept : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (Dies (Macros.target Macros.creature)) (Macros.exile It) Nothing {sp = ok})
badStandingIntercept SpanUnstated impossible


-- The same refusal on the shield: forty-nine "Prevent all …" lines
-- state no duration and every one of them is a static ability
-- ("Prevent all combat damage that would be dealt to this creature").
public export
badStandingPrevention : Unspellable (Effect []) (\ok =>
  Macros.preventAll AnyDamage Everywhere Nothing {sp = ok})
badStandingPrevention SpanUnstated impossible


-- Prevention writes ONE adverbial. Two hundred fifty-five prevention
-- lines carry "this turn"; "prevent … until end of turn" is written
-- zero times, all scopes. The shield and the grants do not share a
-- current-turn word any more than the restriction and the grants do.
public export
badPreventUntilEndOfTurn : Unspellable (Effect []) (\ok =>
  Macros.preventAll CombatOnly Everywhere (Just Macros.untilEndOfTurn) {sp = ok})
badPreventUntilEndOfTurn SpanStated impossible


-- Nor a for-as-long-as one: no corpus line conditions a shield on a
-- tracked predicate ([CR#611.2b]'s adverbial), which is what keeps the
-- widest class in the table from swallowing the two new rows.
public export
badInterceptForAsLongAs : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (Dies (Macros.target Macros.creature)) (Macros.exile It)
                 (Just (ForAsLongAs (Exists Macros.creatureYouControl))) {sp = ok})
badInterceptForAsLongAs SpanStated impossible


-- Real oracle English, no clause of ours: thirty-one lines write "would
-- be destroyed" and twenty-five of them are regeneration's own reminder
-- text, whose replacement is [CR#614.8]'s four-part instruction — tap
-- it, remove it from combat, heal the damage on it — not one part of
-- which this vocabulary writes. `EventUnclaimed` says exactly that, and
-- keeps it apart from the events nothing writes at all.
public export
badInterceptDestruction : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (IsDestroyed (Macros.target Macros.creature)) (Macros.exile It) (Just Macros.thisTurn) {ok})
badInterceptDestruction MkInterceptable impossible


-- The multiplicity word is the event's to choose. "The next time
-- [subject] would die" is written zero times against fifty-seven
-- conditional lines, because a creature dies once and the two shields
-- would be the same shield ([CR#614.3]).
public export
badNextTimeWouldDie : Unspellable (Effect []) (\ok =>
  Macros.nextTimeWouldInstead (Dies (Macros.target Macros.creature)) (Macros.exile It) (Just Macros.thisTurn) {uo = ok})
badNextTimeWouldDie MkReplUseOk impossible


-- Dying is the battlefield-to-graveyard transition ([CR#700.4]), so the
-- watched object stands on the battlefield — `EventQuery`'s own demand
-- asked by the other reader of the same event.
public export
badWouldDieInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok})
                 (Macros.exile It) (Just Macros.thisTurn))
badWouldDieInGraveyard MkZoneFits impossible


-- The replacement is a HOLE outward. [CR#614.7] says a replacement
-- effect whose intercepted event never happens "simply doesn't do
-- anything", so a token the replacement would have made is not there
-- for the next sentence to read — which is the conditional arm's
-- refusal (finding 100) at a second site.
public export
badInterceptReplacementAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.nextTimeWouldInstead (Draws You)
                                      (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Soldier]))
                                      (Just Macros.thisTurn)
               , Macros.sacrifice You (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}
               ])
badInterceptReplacementAntecedent (Refl, _) impossible


-- No corpus line ends a CONTINUOUS effect at a leaves-the-battlefield
-- event with a clause this grammar writes. Of the ninety-one lines that
-- write the phrase, eighty-six are [CR#610.3] zone changes and three
-- are [CR#610.4] phasings — neither a continuous effect — and the three
-- that are one set a base type or make a copy. `Unclaimed` is the cell,
-- and it names the constructions the grammar is missing rather than
-- claiming the phrase.
public export
badGetsUntilLeavesBattlefield : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) 2 2 (Just (UntilEvent (Leaves Macros.thisCreature))) {sp = ok})
badGetsUntilLeavesBattlefield SpanStated impossible


-- The rider goes on a zone change to EXILE and on nothing else: nothing
-- returns from a graveyard "until", and the corpus writes the rider
-- with the exile verb every time.
public export
badHeldUntilDestroy : Unspellable (Effect []) (\ok =>
  HeldUntil (Macros.destroy (Macros.target Macros.creature)) (Leaves Macros.thisCreature) {ok})
badHeldUntilDestroy MkHeldClause impossible


-- And the event half of the same gate: the rider waits for a
-- leaves-the-battlefield event and for no other. Not one line writes
-- "exile [object] until [something] dies".
public export
badHeldUntilDies : Unspellable (Effect []) (\ok =>
  HeldUntil (Macros.exile (Macros.target Macros.creature)) (Dies Macros.thisCreature) {hd = ok})
badHeldUntilDies MkHoldable impossible


-- …and the exile the rider takes is the UNRIDDEN one. The two
-- constructions say opposite things about the same card: [CR#610.3]
-- schedules a return that no ability has to ask for, where counters sit
-- in exile waiting for an ability to read them, and no corpus line asks
-- for both — "exile … with … counters on it until …" is zero lines.
public export
badHeldUntilWithCounters : Unspellable (Effect []) (\ok =>
  HeldUntil (Macros.exileWithCounters (Macros.target Macros.creature) (Lit 3) Time)
            (Leaves Macros.thisCreature) {ok})
badHeldUntilWithCounters MkHeldClause impossible


-- The held object's ZONE is not settled by the clause: the undo is
-- scheduled on an event that has not happened, so the exiled creature
-- may be in exile or back on the battlefield when a later sentence
-- reads it. The clause contributes its announcement and not the exile's
-- retag, so a graveyard-demanding read of the exiled card finds nothing
-- there to move.
public export
badHeldUntilExileRetag : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.exileUntil (Macros.target Macros.creature) (Leaves Macros.thisCreature)
               , Move (That CardW {ok}) Macros.handZ
               ])
badHeldUntilExileRetag Refl impossible


-- A replacement effect gets "only one opportunity to affect an event or
-- any modified events that may replace that event" ([CR#614.5]), so a
-- replacement of a replacement is not a sentence English writes.
public export
badNestedInstead : Unspellable (Effect []) (\ok =>
  Macros.insteadOf (Macros.insteadOf Macros.drawACard (Macros.drawCards 2)) (Macros.drawCards 3) {na = ok})
badNestedInstead MkNotInstead impossible


-- [CR#614.6]: a replaced event "never happens". So the replaced
-- clause's OUTCOME is not there to read — "that much" after a damage
-- clause that was replaced measures nothing — even though the same
-- clause's announced target is ([CR#601.2c]). `annIntro` and `effIntro`
-- divide exactly here.
public export
badInsteadReadsReplacedOutcome : Unspellable (Effect []) (\ok =>
  Macros.insteadOf (DealDamage This (Lit 3) (Macros.target AnyTarget))
            (Macros.gainsLife You (ThatMuch {ok})))
badInsteadReadsReplacedOutcome Refl impossible


-- The same refusal through a SEQUENCE, which is where it used to leak:
-- the replacement was typed in `preIntro replaced`, and a sequence's
-- pre-state is its last clause's over the DEED telescope, so an earlier
-- step's outcome walked into a replacement for an event that never
-- happened. The announcement channel is a hole at a sequence, its
-- elements being typed over each other's `effIntro` (`annIntro`).
public export
badInsteadReadsReplacedSequenceOutcome : Unspellable (Effect []) (\ok =>
  Macros.insteadOf (Sequentially [DealDamage This (Lit 3) (Macros.target Macros.creature), Macros.drawACard])
            (Macros.gainsLife You (ThatMuch {ok})))
badInsteadReadsReplacedSequenceOutcome Refl impossible


-- And through a CONDITIONAL wrapping an optional clause, which is the
-- recursion that carried either defect: chapter twenty-five opened the
-- conditional's announcement channel on `preIntro e`, correctly for the
-- flat clause it was opened for (Overload's "that artifact", Colossal
-- Growth's "that creature") and not for a composite. It is structural
-- now, so the nested may's damage outcome is not there to read.
public export
badConditionalInsteadReadsMayOutcome : Unspellable (Effect []) (\ok =>
  Macros.insteadOf (If (Macros.may You (DealDamage This (Lit 2) (Macros.target Macros.creature)))
                (Exists Macros.creature)
                Nothing)
            (Macros.gainsLife You (ThatMuch {ok})))
badConditionalInsteadReadsMayOutcome Refl impossible


-- A draw is not a payment. The colon used to accept any clause at all,
-- which is what the ledger's cost-GRAMMAR entry named: [CR#602.1a] makes
-- a cost what the ACTIVATOR pays, and no corpus line writes "Draw a
-- card:" before a colon (zero, against eleven hundred eighty-four
-- sacrifice components).
public export
badDrawAsCost : Unspellable Ability (\ok =>
  Activated (Do Macros.drawACard {ok}) Macros.drawACard)
badDrawAsCost MkCostAction impossible


-- The life row is DIRECTIONAL: ninety-five "Pay N life" components
-- against zero gain-life ones, so paying life is a cost and gaining it
-- is not. A gain-life cost does exist — [CR#119.7] speaks of "a cost
-- that involves having that player gain life" — and the cards that print
-- one spell it as an ALTERNATIVE cost ([CR#118.9]), a base swap rather
-- than an activation cost.
public export
badGainLifeCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.gainsLife You (Lit 2)) {ok}) Macros.drawACard)
badGainLifeCost MkCostAction impossible


-- A cost component may not read a SIBLING component's deed. The
-- telescope threads the stamp because the ability BODY reads it (Bosh,
-- Iron Golem), but [CR#601.2h] pays the components in two tiers and each
-- of them "in any order", so no component may presuppose that a SIBLING
-- has already been paid — and no corpus line writes one that does.
public export
badCostReadsSiblingDeed : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.sacrifice You (Macros.a Macros.creature)),
                       Do (Macros.exile (TheVerbed Sacrifice CardW)) {ok}])
            Macros.drawACard)
badCostReadsSiblingDeed MkCostAction impossible


-- Nor may one carry a TARGET. [CR#601.2c] announces targets while the
-- ability is still being proposed and [CR#601.2h] pays the costs at the
-- END of that same procedure, so the determiner belongs past the colon
-- and never before it.
public export
badTargetedCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice You (Macros.target Macros.creature)) {ok}) Macros.drawACard)
badTargetedCost MkCostAction impossible


-- And the payer is the ACTIVATOR: [CR#602.1a] says the activation cost
-- "must be paid by the player who is activating it", so a component that
-- names its payer names "you" (Erebos, God of the Dead's "Pay 2 life")
-- and no line charges an opponent. The demand is the POSITION's and not
-- the clause's — the same life component under the `Pay` clause names
-- whoever the sentence names, which is the hundred seventy-seven
-- "unless its controller / that player / any player pays" lines
-- `badUnlessAnaphoricPayer` is measured against.
public export
badForeignPayerCost : Unspellable Ability (\ok =>
  Activated (Macros.payLife Macros.anOpponent 2) Macros.drawACard {py = ok})
badForeignPayerCost MkCostPaidByYou impossible


-- …and the subjected cost verbs the same way: "an opponent sacrifices a
-- creature" is real English as a resolving clause and is no payment of
-- YOURS before a colon.
public export
badForeignSacrificeCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice Macros.anOpponent (Macros.a Macros.creature))) Macros.drawACard {py = ok})
badForeignSacrificeCost MkCostPaidByYou impossible


-- "Pay" is one English VERB and a cost is the whole thing an ability
-- charges. A sacrifice is a payment ([CR#118.1] — a cost is "an action
-- or payment") and is not payABLE:
-- the seventy-one non-mana unless lines write their own verb ("unless
-- you sacrifice a land"), never "pay".
public export
badPayBySacrificing : Unspellable (Effect []) (\ok =>
  Pay You (Do (Macros.sacrifice You (Macros.a Macros.creature))) {pb = ok})
badPayBySacrificing MkPayable impossible


-- The pay clause spells its subject ONCE, so the component under the
-- verb names the same player: "you pay 1 life" (Carnophage) is one payer
-- written once, and "you pay [an opponent pays 1 life]" is a sentence
-- with two. Conservative in the direction the telescope allows — the
-- imperative's component must name you, a named subject's is left to the
-- anaphoric-payer family `badUnlessAnaphoricPayer` measures.
public export
badMismatchedPayer : Unspellable (Effect []) (\ok =>
  Macros.mayElse You (Pay You (Macros.payLife Macros.anOpponent 1) {ag = ok}) Macros.drawACard)
badMismatchedPayer MkPayAgrees impossible


-- The tap symbol still more sharply: it is a cost that exists only
-- before a colon ([CR#107.5] gives it its meaning there), and no
-- sentence anywhere spells it as a verb phrase.
public export
badPayTapSymbol : Unspellable (Effect []) (\ok =>
  Pay You TapSymbol {pb = ok})
badPayTapSymbol MkPayable impossible


-- And no line writes "pay" over a comma-joined cost: the compound is a
-- cost SHAPE, not a complement English's pay-verb takes.
public export
badPayCompound : Unspellable (Effect []) (\ok =>
  Pay You (Compound [Mana [Macros.generic 1], TapSymbol]) {pb = ok})
badPayCompound MkPayable impossible


-- Growing the container is what made this refusal necessary. English
-- grants an activated ability by QUOTING it — "Enchanted land has \"{T}:
-- Add {B}\"", twenty-five lines, and the equipped/all-Slivers twins with
-- it — which is a construction this grammar has no quotation for, where
-- "gains flying" is a bare keyword.
public export
badGainsActivated : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature)
               (Activated (Mana [Macros.generic 1]) Macros.drawACard)
               (Just Macros.untilEndOfTurn) {gr = ok})
badGainsActivated MkGrantable impossible


-- A compound's ELEMENTS are components: nesting re-mints the
-- right-nested tree the telescope replaced, and core reaches the flat
-- form by normalizing instead (`Cost::normalize`).
public export
badNestedCompound : Unspellable Ability (\ok =>
  Activated (Compound ((Compound [Mana [Macros.generic 1], TapSymbol] :: (TapSymbol :: Nil)) {nc = ok}))
            Macros.drawACard)
badNestedCompound MkNotCompound impossible


-- And a compound of one is the component itself spelled a second way —
-- the singleton-sequence refusal at the cost layer.
public export
badSingletonCompound : Unspellable Ability (\ok =>
  Activated (Compound [Mana [Macros.generic 1]] {two = ok}) Macros.drawACard)
badSingletonCompound TwoUp impossible


-- One self-tap per cost. [CR#107.5] says it in as many words — "a
-- permanent that's already tapped can't be tapped again to pay the
-- cost" — and [CR#118.3] refuses a payment the payer has not got the
-- resources for, so a second "{T}" charges a state the permanent has
-- once. No corpus line writes "{T}, {T}:" and none writes the untap
-- symbol beside it either, the two spending the same state.
public export
badDoubleTapCost : Unspellable Ability (\ok =>
  Activated (Compound [TapSymbol, TapSymbol]) Macros.drawACard {tp = ok})
badDoubleTapCost MkCostTapOnce impossible


-- A cost component's symbol RUN is written. The empty list is a real
-- value on a CARD — [CR#202.1b]'s "no mana cost", the unpayable absence
-- a land prints — but before a colon the run is the component's whole
-- spelling, so an empty one spells an activation line opening on a bare
-- colon. The payment of nothing is "{0}" ([CR#118.5]), one symbol.
public export
badEmptyManaCost : Unspellable Ability (\ok =>
  Activated (Mana [] {wr = ok}) Macros.drawACard)
badEmptyManaCost MkManaRun impossible


-- A hybrid Phyrexian symbol names two DIFFERENT colors, and [CR#107.4f]
-- says so by counting: five ordinary Phyrexian symbols and "ten hybrid
-- Phyrexian mana symbols", ten being the unordered pairs of five
-- distinct colors. There is no "{W/W/P}" to write.
public export
badSameColorPhyrexian : Unspellable ManaSymbol (\ok =>
  Phyrexian White (Just White) {ds = ok})
badSameColorPhyrexian MkPhyrexianDistinct impossible


-- …and the ordinary hybrid the same way: [CR#107.4e] makes the symbol "a
-- cost that can be paid in one of two ways", and two ways spelled the
-- same word is one way.
public export
badSameColorHybrid : Unspellable ManaSymbol (\ok =>
  Macros.hybridPip Blue Blue {ds = ok})
badSameColorHybrid MkHalvesDistinct impossible


-- The arm asymmetry carries to the MANDATORY twin unchanged, which is
-- the check that the two shapes really are one node: the declined arm
-- runs only when the payment was never started ([CR#118.12] — "started
-- to pay a mandatory cost, regardless of what events actually
-- occurred"), so the body's phrase named nobody it can read.
public export
badIfNotReadsMandatoryBody : Unspellable (Effect []) (\ok =>
  Macros.doElse (Macros.sacrifice You (Macros.a Macros.creature)) (Macros.exile (It {ok})))
badIfNotReadsMandatoryBody Refl impossible


-- The unless family's OTHER half, and the shape of its wall: a hundred
-- twenty-five "unless its/their controller pays" lines, forty-three
-- "unless that player pays" and nine "unless any player pays" name their
-- payer with an anaphor into the MAIN clause ("Return target creature to
-- its owner's hand unless its controller pays {1}"). [CR#118.12a]'s
-- rewrite is not linearization-preserving — it puts the may first — so
-- the payer phrase is typed before the clause that announces what it
-- reads, and "it" has nothing to reach.
public export
badUnlessAnaphoricPayer : Unspellable (Effect []) (\ok =>
  Macros.mayElse (ControllerOf (It {ok})) (Pay You (Mana [Macros.generic 1])) (Tap (Macros.target Macros.creature)))
badUnlessAnaphoricPayer Refl impossible


-- [CR#603.2b] gives "at the beginning of" a phase or step its own
-- clause, and the corpus honors it without exception: one thousand six
-- hundred seventy-eight headers open with "At the beginning of" and
-- every one of them names a turn part. No object event takes the word.
public export
badAtEnters : Unspellable Ability (\ok =>
  Triggered At (Enters Macros.thisCreature) Macros.drawACard {wo = ok})
badAtEnters MkTriggerWordOk impossible


-- The trigger WORD table is right and the SUBJECT was not: "When this
-- enters" and "Whenever this enters" are written zero times apiece,
-- against eighteen hundred eighteen "When this creature enters" and
-- seventy-four "Whenever this creature enters", and every other
-- object-subject event measures the same way (dies three hundred
-- eighty-one, attacks five hundred sixty-eight, blocks sixty-nine,
-- deals combat damage two hundred thirty-one — all against zero bare
-- ones). The type word is what places the referent ([CR#109.2]); bare
-- `This` is the source as an object and stands nowhere.
public export
badEntersBareThis : Unspellable Ability (\ok =>
  Triggered When (Enters This {ss = ok}) Macros.drawACard)
badEntersBareThis MkSelfSorted impossible


-- An ordinary trigger's header announces no TARGET. [CR#115.1d] chooses
-- a triggered ability's targets "as the ability is put on the stack",
-- which is after the event that put it there, so the header is not where
-- one can be announced — the ability targets one clause later
-- (`eliteJavelineer`). The delayed carrier is the exception the corpus
-- writes and it still builds (`gracefulReprieve`).
public export
badTargetedDeathHeader : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.target Macros.creature)) Macros.drawACard {hn = ok})
badTargetedDeathHeader MkHeaderNontarget impossible


-- …and the inverse, which is the half that makes the table a table: the
-- turn-part beginning takes neither English word, "When the beginning of
-- your upkeep" being written zero times against six hundred forty-one
-- "At the beginning of your upkeep".
public export
badWhenUpkeep : Unspellable Ability (\ok =>
  Triggered When (BeginningOf Upkeep (Just Yours)) Macros.drawACard {wo = ok})
badWhenUpkeep MkTriggerWordOk impossible


-- Real oracle English in one mood and none in the other: thirty-one
-- lines write "would be destroyed" (regeneration's reminder text,
-- [CR#614.8]) and NOT ONE writes "whenever [something] is destroyed" as
-- a header — the modern templating for that event is "dies", which is a
-- different row. `EventUnclaimed` is the only class no reader claims.
public export
badTriggerOnDestruction : Unspellable Ability (\ok =>
  Triggered Whenever (IsDestroyed (Macros.a Macros.creature)) Macros.drawACard
            {tr = Builtin.fst ok, wo = Builtin.snd ok})
badTriggerOnDestruction (MkTriggerable, _) impossible


-- The untap step's beginning is written zero times in every possession —
-- the turn-part grid's emptiest row, and `PartUnattested` says so.
public export
badTriggerAtUntapStep : Unspellable Ability (\ok =>
  Triggered At (BeginningOf UntapStep Nothing {pu = ok}) Macros.drawACard)
badTriggerAtUntapStep MkPartTriggerable impossible


-- Nor the turn's own beginning: "at the beginning of your turn" is zero
-- lines, because the UPKEEP is what English names there.
public export
badTriggerAtYourTurn : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Turn (Just Yours) {pu = ok}) Macros.drawACard)
badTriggerAtYourTurn MkPartTriggerable impossible


-- The other silence, and it is the possessor gap chapter twenty-seven
-- measured from the activation side: "At the beginning of each upkeep"
-- (thirty-six), "each opponent's upkeep" (thirty-three) and "the upkeep
-- of enchanted creature's controller" (twenty-seven) are real headers
-- whose possessor is a quantifier or a nominal, and `Whose` is a
-- two-word pronominal vocabulary. Unpossessed is not what they write.
public export
badTriggerAtTheUpkeep : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Upkeep Nothing {pu = ok}) Macros.drawACard)
badTriggerAtTheUpkeep MkPartTriggerable impossible


-- The trigger's effect reads the event's AFTER-discourse, and this is
-- the refusal that shows it: [CR#603.6] has a zone-change trigger "look
-- for the object in the zone that it moved to", so after "Whenever a
-- creature dies" the referent is a card in a graveyard and tapping it is
-- the dead-referent refusal ([CR#701.26a]). The interception's
-- replacement, over the same event row, taps it perfectly well — the
-- event has not happened there ([CR#614.6]).
public export
badTriggerTapsDeadCreature : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.a Macros.creature)) (Tap It {ok}))
badTriggerTapsDeadCreature OnField impossible


-- …and the DEPARTURE is the same refusal where the destination is not
-- stated. [CR#603.6c] has a leaves-the-battlefield ability check for the
-- object "only in the first zone that it went to", and the sentence
-- never names that zone, so an unknown destination is not still the
-- battlefield: the binding survives the event and its zone does not.
public export
badLeavesThenTap : Unspellable Ability (\ok =>
  Triggered Whenever (Leaves (Macros.a Macros.creature)) (Tap It {ok}))
badLeavesThenTap OnField impossible


-- A static ability does not TARGET. [CR#115.1a..115.1e] enumerate what
-- can — an instant or sorcery spell, an activated ability, a triggered
-- ability, and the keyword abilities that represent those — and
-- [CR#115.1b] says of the nearest case outright that "an Aura permanent
-- doesn't target anything; only the spell is targeted". The same
-- statement is impeccable as a resolving CLAUSE with a span
-- (`cantAttack (target creature) (Just thisTurn)`), which is what makes
-- this the line's own refusal rather than the deontic's.
public export
badStaticTargets : Unspellable Ability (\ok =>
  Static (Cant (Macros.target Macros.creature) Attack Agent) {ut = ok})
badStaticTargets MkUntargeting impossible


-- The one static effect English does not state as a line. Its stative
-- form is a different VERB — "You control enchanted creature" (seven
-- lines) — where every other row inflects the same verb it writes as a
-- clause ("gains"/"has", "becomes"/"is"). Zero lines write a
-- durationless "gains control of".
public export
badStaticGainsControl : Unspellable Ability (\ok =>
  Static (GainsControl You (AllOf Macros.creatureYouControl)) {ln = ok})
badStaticGainsControl MkStaticLine impossible


-- …and the "as long as" wrapper does not launder it. The mood is the
-- statement's and not the qualifier's: [CR#604.1] has a static ability
-- "written as a statement" that is "simply true", so qualifying an
-- unstatable one leaves it unstatable — Control Magic writes "You
-- control enchanted creature" whether or not a condition rides on it.
public export
badConditionalGainControl : Unspellable Ability (\ok =>
  Static (Macros.asLongAs (Exists Macros.artifact) (GainsControl You (Macros.a Macros.creature))) {ln = ok})
badConditionalGainControl MkStaticLine impossible


-- "As long as" is not a duration adverbial and its statement is not a
-- resolving clause: the conditional static is an ability line and
-- nothing else, so `absentOk Conditional` is `False` and every
-- `admitsSpan` cell with it is too. One preposition tells the two
-- constructions apart — [CR#611.2b]'s "FOR as long as" is the duration
-- (two hundred ten lines, and `Duration.ForAsLongAs` already spells it),
-- [CR#611.3a]'s bare "as long as" is this (nine hundred twelve).
public export
badConditionalClause : Unspellable (Effect []) (\ok =>
  Continuously (Macros.asLongAs (Exists Macros.creatureYouControl)
                         (Gets (AllOf Macros.creatureYouControl) 1 1))
               Nothing {sp = ok})
badConditionalClause SpanUnstated impossible


-- And no line conditions a statement twice: the singleton discipline
-- `badNestedCompound` keeps at the cost layer, one type up.
public export
badDoubleConditional : Unspellable Ability (\ok =>
  Static (Macros.asLongAs (Exists Macros.creatureYouControl)
                   (Macros.asLongAs (Exists (And [Macros.artifact, ControlledBy You]))
                             (Gets (AllOf Macros.creatureYouControl) 1 1)) {nn = ok}))
badDoubleConditional MkNotConditional impossible


-- The entry rider is a static ability and not a clause either
-- ([CR#603.6d] says so in as many words), which is the third of the
-- three families chapter twenty-five sent to a container that did not
-- exist. "Put [card] onto the battlefield tapped" is the one-shot twin
-- and it is a rider on a MOVE, not a continuous effect (ledger).
public export
badEntryRiderClause : Unspellable (Effect []) (\ok =>
  Continuously (Macros.entersTapped (AllOf Macros.creatureYouControl)) Nothing {sp = ok})
badEntryRiderClause SpanUnstated impossible


-- …and the line writes ONE of the two riders. A token's with-clause can
-- say "tapped and attacking" because a resolving effect knows there is a
-- combat; a permanent's own static ability applies whenever it enters,
-- from any zone in any step, and the corpus writes the second rider on
-- such a line zero times.
public export
badEntersAttackingLine : Unspellable Ability (\ok =>
  Static (EntersRider (AsType Land This) EntersAttacking {ro = ok}))
badEntersAttackingLine MkEntryRiderOk impossible


-- The permission divides the same way, and it is the sharpest of the
-- three because it writes spans freely: ninety-eight "this turn",
-- twenty-two "until the end of your next turn", twenty-six "for as long
-- as", eight "until your next end step" — and states none at all only
-- when it is the card's own line ("You may cast this card from your
-- graveyard").
public export
badStandingPermission : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile Macros.topCard, Continuously (MayPlay You It) Nothing {sp = ok}])
badStandingPermission SpanUnstated impossible
