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
