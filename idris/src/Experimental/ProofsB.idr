module Experimental.ProofsB

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off

-- Continuation of Experimental.Proofs; see Unspellable there.


-- The class word is written ONCE: no corpus line repeats it inside one
-- phrase, and "any target and any target" names one referent twice.
public export
badDoubleAnyTarget : Unspellable (Predicate [] Object) (\ok =>
  And [AnyTarget, AnyTarget] {at = ok})
badDoubleAnyTarget MkAnyTargetLone impossible


-- …and so is the modifier: the guide's selector order gives
-- other/another a single slot. (Posed in a creature-target context so
-- the anchor presupposition itself is satisfied — what refuses is the
-- doubling, not the witness search.)
public export
badDoubleOther : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing)] Object)
  (\ok => And [Macros.creature, Other, Other] {oa = ok})
badDoubleOther MkOtherAnchored impossible


-- The class word hides just as poorly inside an EMBEDDED noun: a
-- relative clause's possessor is a phrase like any other, so "a
-- creature the controller of any target controls" spells "any target"
-- under the non-targeting determiner that forbids it.
public export
badAnyTargetEmbedded : Unspellable (Noun [] Object) (\ok =>
  Macros.a (And [Macros.creature, ControlledBy (ControllerOf (Macros.target AnyTarget))]) {af = ok})
badAnyTargetEmbedded MkAnyTargetFree impossible


-- The untracked exception belongs to the bare self-reference alone —
-- it is not a free pass for every unplaced referent. A READ whose
-- antecedent records no zone is not thereby a hand card ([CR#701.9a]
-- moves one FROM a hand), which is precisely what the old zone-level
-- gate could not distinguish. (Posed at a zoneless binding — the shape
-- a singular object mention takes before anything places it.)
public export
badDiscardIt : Unspellable
  (Effect [MkBinding AD Object OneOf (ObjectP Nothing Nothing Nothing Nothing)])
  (\ok => Macros.discards You It {dk = ok})
badDiscardIt DiscardTracked impossible


-- …and the class word is no hand card either: it heads no zone clause
-- and takes no [CR#109.2] default (it describes no object to place),
-- so the phrase reaches discard with no zone at all — neither the bare
-- self-reference nor a tracked hand card, which is the whole of the
-- gate.
public export
badDiscardAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.discards You (Macros.target AnyTarget) {dk = Builtin.fst ok, na = Builtin.snd ok})
badDiscardAnyTarget (_, MkNotAnyTarget) impossible


-- A controller relation presupposes the battlefield: objects that are
-- neither on the stack nor on the battlefield "aren't controlled by any
-- player" ([CR#109.4]), so "a creature you control in your graveyard"
-- places its referent in two zones at once — the finding-43 shape once
-- more, a projection made honest and the existing coherence gate
-- supplying the refusal.
public export
badControlledInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, ControlledBy You, InZone Macros.graveyardZ] {zc = ok})
badControlledInGraveyard MkZoneCoherent impossible


-- A sequence of no clauses is no instruction at all. Core admits
-- `Sequentially([])` structurally — it is a data shape there, and an
-- empty body is a useful identity for the engine — but nothing writes
-- an empty sentence list, so the workbench refuses what it cannot
-- render.
public export
badEmptySequence : Unspellable (Effect []) (\ok =>
  Sequentially [] {ok})
badEmptySequence TwoUp impossible


-- …and a sequence of one clause is not a sequence either: it spells
-- exactly what the clause alone spells, and one meaning gets one
-- spelling.
public export
badSingletonSequence : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature)] {ok})
badSingletonSequence TwoUp impossible


-- "Any target" NAMES [CR#115.4]'s damage class — creature, player,
-- planeswalker, or battle — instead of describing an object, so
-- [CR#109.2] has nothing to place on the battlefield and the phrase
-- projects no zone. Destruction moves a battlefield permanent
-- ([CR#701.8a]); the corpus writes no "Destroy any target", and the
-- refusal now falls out of the zone gate the verb already had.
public export
badDestroyAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target AnyTarget) {ok = Builtin.fst ok, na = Builtin.snd ok})
badDestroyAnyTarget (OnField, _) impossible


-- The same refusal one verb over and at the raw constructor — tapping
-- takes a battlefield object ([CR#701.26a]) — so it is the projection
-- that changed, not one macro's demand.
public export
badTapAnyTarget : Unspellable (Effect []) (\ok =>
  Tap (Macros.target AnyTarget) {ok})
badTapAnyTarget OnField impossible


-- …and the mention is no better placed when it is read back: the
-- binding an any-target phrase introduces records the phrase's own
-- silence, so "it" inherits no battlefield the phrase never claimed.
public export
badDestroyAnyTargetRemention : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.target AnyTarget),
               Macros.destroy It {ok}])
badDestroyAnyTargetRemention OnField impossible


-- The PLACEMENT is where the silence had to be refused by name rather
-- than by a zone gate, this verb having none: a move SETS the zone, so
-- there was nothing for the projection to contradict and "exile any
-- target" went through. [CR#115.4] rules it out at the phrase — the
-- class spans players, and no placement takes one.
public export
badExileAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.exile (Macros.target AnyTarget) {na = ok})
badExileAnyTarget MkNotAnyTarget impossible


-- And the COUNTERING for the other half of the same rule: [CR#115.4]
-- says a spell "can't be chosen this way", and [CR#112.1] makes a spell
-- a card on the stack, so the demand is evidence of that zone and not a
-- `zoneFits` silence. Fifty-two lines write "Counter target spell." and
-- none writes the class word.
public export
badCounterAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.counterSpell (Macros.target AnyTarget) {zn = ok})
badCounterAnyTarget OnTheStack impossible


-- The recipient gate reads the NOUN, and that is what keeps the class
-- word's own row from becoming a zone-free hole: bare `This` is the
-- source as an object ("this spell") and projects no zone either, but
-- it is no damage recipient ([CR#120.1a] — a battle, a creature, or a
-- planeswalker; players by [CR#120.1]).
public export
badDamageThis : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) This {rk = ok})
badDamageThis ObjectTakes impossible


-- A coordination offers alternatives, so it needs two. At one it
-- offers none and spells exactly what the bare alternative spells; at
-- zero it spells nothing at all — the arity discipline `Sequentially`
-- already carries, one construction over.
public export
badEmptyOr : Unspellable (Predicate [] Object) (\ok =>
  Or [] {tw = ok})
badEmptyOr MkTwoDisjuncts impossible


public export
badSingletonOr : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.creature] {tw = ok})
badSingletonOr MkTwoDisjuncts impossible


-- …and the same argument once more at two: "artifact or artifact"
-- offers a choice between a thing and itself.
public export
badRepeatedDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.artifact, Macros.artifact] {dd = ok})
badRepeatedDisjunct MkDistinctDisjuncts impossible


-- Alternatives are PARALLEL — each one has to be able to stand where
-- the others stand. A head noun and a bare status word cannot:
-- "artifact or attacking" leaves the second alternative nothing to be,
-- and the guide says as much outright ("Repeat the carrier when the
-- alternatives have different domains or modifiers").
public export
badHeadlessDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.artifact, Attacking] {pd = ok})
badHeadlessDisjunct MkParallelDisjuncts impossible


-- The same demand about PLACE. Oracle does write cross-zone
-- alternatives — "an Equipment card from your hand or graveyard",
-- seventeen corpus lines — but under a shared preposition and with a
-- zone that can name two. This projection names one, so the phrase is
-- refused rather than quietly placed on the battlefield by default
-- (the axis is ledgered).
public export
badCrossZoneDisjunction : Unspellable (Predicate [] Object) (\ok =>
  Or [InZone Macros.handZ, InZone Macros.graveyardZ] {pd = ok})
badCrossZoneDisjunction MkParallelDisjuncts impossible


-- "Any target" is ALREADY the union [CR#115.4] fixes — creatures,
-- players, planeswalkers, or battles — so coordinating it with an
-- alternative re-opens a closed class.
public export
badAnyTargetInOr : Unspellable (Predicate [] Object) (\ok =>
  Or [AnyTarget, Macros.creature] {cd = ok})
badAnyTargetInOr MkCoordinableDisjuncts impossible


-- …and a singleton conjunction wrapped around it launders nothing:
-- every alternative is read through `flattenPs`, which is the lesson
-- the negation row learned when `And [x]` could still hide anything.
public export
badAnyTargetInOrLaundered : Unspellable (Predicate [] Object) (\ok =>
  Or [And [AnyTarget], Macros.creature] {cd = ok})
badAnyTargetInOrLaundered MkCoordinableDisjuncts impossible


-- "Other" fills one selector slot for the whole coordinated phrase
-- ("Another target Wolf or Werewolf you control"), so it is not an
-- alternative either — and the corpus puts it outside the coordination
-- every time it appears with one.
public export
badOtherInOr : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing)] Object)
  (\ok => Or [And [Macros.creature, Other], Macros.land] {cd = ok})
badOtherInOr MkCoordinableDisjuncts impossible


-- Nesting is the flat coordination written with brackets oracle has no
-- way to print. Core reaches the same place by flattening the two
-- spellings together in `normalize`; the workbench refuses the second
-- one instead.
public export
badNestedOr : Unspellable (Predicate [] Object) (\ok =>
  Or [Or [Macros.creature, Macros.land], Macros.artifact] {cd = ok})
badNestedOr MkCoordinableDisjuncts impossible


-- Negation attaches to one modifier at a time. The writer spells
-- "noncreature, nonland card" — comma-chained atoms, plentiful — and
-- never "non-(creature or land)", which the corpus does not write once.
public export
badNegatedDisjunction : Unspellable (Predicate [] Object) (\ok =>
  Not (Or [Macros.creature, Macros.land]) {ng = ok})
badNegatedDisjunction MkNegatable impossible


-- Damage can't be dealt to an object that isn't a battle, a creature,
-- or a planeswalker ([CR#120.1a]), and a disjunctive head fixes no
-- type at all — so the phrase that declines to say WHICH type it names
-- cannot become a damage recipient on the strength of that silence.
-- (The refusal surfaces at the recipient gate, as `badDamageThis`
-- does; what closed underneath it is `DamageableTy`'s untyped row.)
public export
badDamageDisjunctHead : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 2) (Macros.target (Or [Macros.artifact, Macros.enchantment])) {rk = ok})
badDamageDisjunctHead ObjectTakes impossible


-- The contrast positive's mirror: alternatives that AGREE on a zone
-- still project it, so an attacking-or-blocking creature stands on the
-- battlefield and cannot also be in a graveyard.
public export
badAttackingOrBlockingInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Or [Attacking, Blocking], InZone Macros.graveyardZ] {zc = ok})
badAttackingOrBlockingInGraveyard MkZoneCoherent impossible


-- …and they project the TYPE they agree on the same way: only a
-- creature can attack or block ([CR#506.3]), a presupposition the
-- disjunction inherits from both alternatives at once.
public export
badNoncreatureAttackingOrBlocking : Unspellable (Predicate [] Object) (\ok =>
  And [Not Macros.creature, Or [Attacking, Blocking]] {cf = ok})
badNoncreatureAttackingOrBlocking MkContradictionFree impossible


-- A presupposition written inside an alternative is written all the
-- same. Both alternatives here are attacking or blocking, so both
-- demand a creature ([CR#506.3]) and the conjunction may not go on to
-- negate the type — but the conjunction BURIED in each alternative hid
-- that from the scan, `flattenPs` leaving an `Or` whole by design.
public export
badWrappedStatusLaunder : Unspellable (Predicate [] Object) (\ok =>
  And [Or [And [Macros.artifact, Attacking], And [Macros.land, Blocking]], Not Macros.creature] {cf = ok})
badWrappedStatusLaunder MkContradictionFree impossible


-- A range runs upward: "between three and two target creatures" names
-- an empty interval, and the number read off the maximum would call
-- that plural besides.
public export
badDescendingRange : Unspellable (Noun [] Object) (\ok =>
  TargetGroup (Range (Just 3) (Just 2)) Macros.creature {wf = ok})
badDescendingRange MkWellFormedQ impossible


-- Alternatives that place their referent differently are not
-- alternatives at all: "attacking artifact" stands on the battlefield
-- and a bare "land" says nothing about where it stands. Compared
-- through [CR#109.2]'s default the two looked parallel, and the
-- disagreement came back instead as a projection of NOTHING — which
-- was enough to let `And [this, InZone graveyardZ]` stand, a phrase
-- placing its referent in two zones at once. Refused at the
-- coordination, where the disagreement is.
public export
badPartialZoneJoin : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.artifact, Attacking], Macros.land] {pd = ok})
badPartialZoneJoin MkParallelDisjuncts impossible


-- …and it starts at one. "Zero or more target creatures" is a second
-- spelling of "any number of target creatures" — [CR#107.1c] has "any
-- number" permit zero already — and the corpus writes only the second.
public export
badZeroLowerRange : Unspellable (Noun [] Object) (\ok =>
  TargetGroup (Range (Just 0) Nothing) Macros.creature {wf = ok})
badZeroLowerRange MkWellFormedQ impossible


-- A coordinated head offers one type per alternative, not none: an
-- earlier LAND anchors "another target artifact or enchantment" no
-- better than it anchors "another target artifact" (`badOtherCrossHead`).
-- Posed at a land target, the anchor being the point.
public export
badDisjunctiveOtherCrossHead : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Land) (Just Battlefield) Nothing Nothing)] Object)
  (\ok => And [Or [Macros.artifact, Macros.enchantment], Other] {oa = ok})
badDisjunctiveOtherCrossHead MkOtherAnchored impossible


-- "Artifact or artifact" is caught by comparing the two words; the
-- same repetition spelled with a modifier went through, the member
-- equality having declined to look inside a conjunction at all.
public export
badRepeatedStructuredDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.creature, ControlledBy You], And [Macros.creature, ControlledBy You]] {dd = ok})
badRepeatedStructuredDisjunct MkDistinctDisjuncts impossible


-- A sequence's ELEMENTS are clauses. Nesting one re-mints the
-- right-nested tree the n-ary list replaced and spells a
-- three-sentence card a second way; what may still hold a sequence is
-- a clause slot taking a body, not an element position.
public export
badNestedSequence : Unspellable (Effect []) (\ok =>
  Sequentially
    ((Sequentially [Macros.destroy (Macros.target Macros.creature), Macros.exile (Macros.target Macros.creature)]
      :: (Macros.destroy (Macros.target Macros.land) :: Nil)) {ns = ok}))
badNestedSequence MkNotSeq impossible


-- Only a creature can attack or block ([CR#506.3]), so a land has no
-- grant for the deed to remove. The demand is the fight slots' shape
-- over a DIFFERENT table: `combatant` answers a non-combat damage deed
-- ([CR#701.14d]), `deedType` the combat grants themselves.
public export
badCantAttackLand : Unspellable (Effect []) (\ok =>
  Macros.cantAttack (Macros.target Macros.land) (Just Macros.thisTurn) {dp = ok})
badCantAttackLand Participant impossible


-- A coordinated head fixes no type (finding 50), and an untyped head
-- cannot prove participation: "target creature or land" would have to
-- carry the attack grant on an alternative that has none. The silence
-- is not permission — `DamageableTy` learned the same lesson.
public export
badCantDisjunctSubject : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target (Or [Macros.creature, Macros.land])) (Just Macros.thisTurn) {dp = ok})
badCantDisjunctSubject Participant impossible


-- Splitting the voice off the deed word made "can't be attacked"
-- WRITABLE as a term for the first time, so the table has to say why
-- it is not writable of a creature: only a player, a planeswalker, or
-- a battle can be attacked ([CR#506.3]). The phrase itself is real
-- oracle ("The Aetherspark can't be attacked"; "until your next turn,
-- you can't be attacked except by creatures with flying") and waits on
-- those types and on the ledgered player subject — which is why this
-- voice gets no macro of its own.
public export
badCantBeAttacked : Unspellable (Effect []) (\ok =>
  Continuously (Cant (Macros.target Macros.creature) Attack Patient {dp = ok}) (Just Macros.thisTurn))
badCantBeAttacked Participant impossible


-- Combat is fought on the battlefield: a permanent that leaves it is
-- removed from combat ([CR#506.4]), so a graveyard card has no deed to
-- be denied. The gate is the one destroy, tap, and "gets" already
-- carry.
public export
badCantInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) (Just Macros.thisTurn) {zn = ok})
badCantInGraveyard MkZoneFits impossible


-- The class word names [CR#115.4]'s damage class, describes no object,
-- and so places none — and the restriction needed no rule of its own to
-- refuse it, and as of chapter twenty-eight the refusal lands one
-- demand over: the zone demand is `zoneFits`' now and silence passes it,
-- so what catches the class word is the DEED's head-type demand — a
-- phrase that describes no object fixes no type, which is
-- `badCantDisjunctSubject`'s refusal reaching a second silent phrase.
public export
badCantAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target AnyTarget) (Just Macros.thisTurn) {dp = ok})
badCantAnyTarget Participant impossible


-- The same span, the wrong word. Two hundred ninety-six corpus lines
-- write a one-shot restriction and every one of them says "this turn";
-- "until end of turn" belongs to the grants, which invert the count
-- (`badGainsThisTurn`). Under the envelope the refusal is one table
-- read: `spanUse` calls the bare end-of-turn endpoint `BothGrants`, and
-- `admitsSpan` gives the restriction row no share of it.
public export
badCantUntilEndOfTurn : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target Macros.creature) (Just Macros.untilEndOfTurn) {sp = ok})
badCantUntilEndOfTurn SpanStated impossible


-- …and the inverse, which is what keeps the new word from opening a
-- hole: no corpus line grants an ability "this turn".
public export
badGainsThisTurn : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying) (Just Macros.thisTurn) {sp = ok})
badGainsThisTurn SpanStated impossible


-- The stat change writes the grant's adverbial too — "Target creature
-- gets +3/+3 until end of turn", never "this turn".
public export
badGetsThisTurn : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) 3 3 (Just Macros.thisTurn) {sp = ok})
badGetsThisTurn SpanStated impossible


-- A durationless "can't" is the STATIC ability line ("Enchanted
-- creature can't attack", Pacifism), which is a different construction
-- and the parked ability layer's. The slot is now optional for everyone
-- — the grants need it, since the unwritten span is [CR#611.2a]'s
-- end-of-game default — so the refusal moved from the ABSENT SLOT to
-- the reason for it: `absentOk DeedRestriction = False`, the one row of
-- that table that says no.
public export
badStaticCant : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target Macros.creature) Nothing {sp = ok})
badStaticCant SpanUnstated impossible


-- The upkeep endpoint belongs to the KEYWORD grant alone. Two corpus
-- lines write "until your next upkeep" and both grant an ability
-- (Gabriel Angelfire, and one forestwalk line); not one stat change
-- takes it. The decomposition is what made the cell writable at all —
-- and the table is what keeps the two grants from sharing it just
-- because they share the other three.
public export
badGetsUntilYourNextUpkeep : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) 3 3 (Just Macros.untilYourNextUpkeep) {sp = ok})
badGetsUntilYourNextUpkeep SpanStated impossible


-- Real oracle English, no clause of ours: "until the end of your next
-- turn" runs to eighty-three lines, and every one of them is a play
-- permission, a control grant, or a can't-cast — never a keyword grant.
-- The `Unclaimed` row exists to say exactly this, and to keep it apart
-- from the endpoints nothing writes at all (`badGainsUntilUntapStep`).
public export
badGainsUntilEndOfYourNextTurn : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying) (Just (Until (EndOf Turn (Just Yours)))) {sp = ok})
badGainsUntilEndOfYourNextTurn SpanStated impossible


-- The combat endpoint is the grants' too — Glyph of Destruction's
-- "+10/+0" and one banding line — and no corpus line ends a single-deed
-- restriction there. The restriction's two words stay "this turn" and
-- "until your next turn".
public export
badCantUntilEndOfCombat : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target Macros.creature) (Just Macros.untilEndOfCombat) {sp = ok})
badCantUntilEndOfCombat SpanStated impossible


-- And the unattested end of the table: no corpus line ends a duration
-- at an untap step in words this vocabulary has. The one line that ends
-- one there possesses it with a NOUN — "until its controller's next
-- untap step" — which is neither of `Whose`'s two words, and its clause
-- (a base-type setting) is a layer word with no construction here
-- either, so the row waits on both.
public export
badGainsUntilUntapStep : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying)
        (Just (Until (StartOf UntapStep (Just Yours)))) {sp = ok})
badGainsUntilUntapStep SpanStated impossible


-- A bound is a MODIFIER: "with power 2 or less" describes something
-- and names nothing, so it cannot be what a determiner determines.
-- The same refusal "attacking" and "other" already take.
public export
badBareComparison : Unspellable (Noun [] Object) (\ok =>
  Macros.target (Compare Power OrLess (Lit 2)) {hd = ok})
badBareComparison MkHeaded impossible


-- Only a creature has power ([CR#208.3] — a noncreature permanent has
-- none, and a noncreature object off the battlefield has one only if
-- it is printed there), so a phrase that bounds power and denies the
-- type describes nothing. The presupposition rides the bound exactly
-- as it rides "attacking" (`badAttackingNoncreature`), the type table
-- supplying the answer in place of a fixed row.
public export
badNoncreaturePower : Unspellable (Predicate [] Object) (\ok =>
  And [Compare Power OrLess (Lit 2), Not Macros.creature] {cf = ok})
badNoncreaturePower MkContradictionFree impossible


-- Oracle never negates a bound. It flips the comparator instead, and
-- can: the game's numbers are integers ([CR#107.1]), so "not power 2
-- or less" IS "power 3 or greater" exactly, with no gap between the
-- two for a negation to name. The corpus writes no "non-", no "doesn't
-- have power", and no "without power 2" — the same De Morgan the
-- writer does for conjunctions (finding 42), here made exact by the
-- discreteness rather than by chaining.
public export
badNegatedComparison : Unspellable (Predicate [] Object) (\ok =>
  Not (Compare Power OrLess (Lit 2)) {ng = ok})
badNegatedComparison MkNegatable impossible


-- One phrase, one bound. The empty pair is the obvious case — power at
-- most two and at least four describes nothing — but the gate is a
-- multiplicity cap rather than a range solver, so the SATISFIABLE pair
-- is refused on the same evidence: no corpus noun phrase carries two
-- bounds at all. An interval is a construction with its own word, not
-- two qualifiers stacked.
public export
badDoubleComparison : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Compare Power OrLess (Lit 2),
       Compare Power OrGreater (Lit 4)] {lc = ok})
badDoubleComparison MkLoneComparison impossible


-- The bound is WRITTEN — a numeral or the announced X — and a phrasal
-- standard belongs to the other frame. Oracle writes "with power less
-- than this creature's power", never "with this creature's power or
-- less": admitting the amount here would spell a real comparison in a
-- word order the language does not use, so the frame is refused and
-- the comparison-to-a-phrase family waits on the ledger.
public export
badPhrasalBound : Unspellable (Predicate [] Object) (\ok =>
  Compare Power OrLess (Macros.powerOf This) {wb = ok})
badPhrasalBound MkWrittenBound impossible


-- An alternative repeated word for word is no alternative, and a bound
-- is compared by all three of its written parts to see it — the row
-- `predEq` gained this chapter, on the lesson `And` taught it
-- (`badRepeatedStructuredDisjunct`).
public export
badRepeatedComparisonDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Compare Power OrLess (Lit 2),
      Compare Power OrLess (Lit 2)] {dd = ok})
badRepeatedComparisonDisjunct MkDistinctDisjuncts impossible


-- Alternatives must presuppose alike, and two characteristics do not:
-- a power bound demands a creature where a mana value bound demands
-- nothing, so "with power 2 or less or mana value 3 or less" commits
-- on one side and stays silent on the other. This is the guide's
-- repeat-the-carrier rule reaching a new pair of seeds with no gate of
-- its own — finding 51's shape, one chapter on.
public export
badMixedCharacteristicDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Compare Power OrLess (Lit 2),
      Compare ManaValue OrLess (Lit 3)] {pd = ok})
badMixedCharacteristicDisjunct MkParallelDisjuncts impossible


-- The class word takes no modifiers but "other" ([CR#115.4] fixes the
-- class by rule, and a bound would narrow it), so the qualifier is
-- refused by the gate that was already there — no comparison rule
-- needed to say so.
public export
badAnyTargetComparison : Unspellable (Predicate [] Object) (\ok =>
  And [AnyTarget, Compare Power OrLess (Lit 2)] {at = ok})
badAnyTargetComparison MkAnyTargetLone impossible


-- A condition quantifies over a DESCRIPTION, so the phrase inside it
-- has to name something: "if you control attacking" heads nothing, the
-- same refusal the indefinite article and the for-each domain already
-- make. The existence row needed no rule of its own — it re-keys onto
-- the head projection the noun layer computes.
public export
badExistsUnheaded : Unspellable (Condition []) (\ok =>
  Exists Attacking {hd = ok})
badExistsUnheaded MkHeaded impossible


-- The class word NAMES [CR#115.4]'s damage class and describes no
-- object, so there is nothing for an existential to be true of. Third
-- consumer of the same gate (the article and the for-each domain being
-- the first two), and it cost one hypothesis rather than a rule.
public export
badExistsAnyTarget : Unspellable (Condition []) (\ok =>
  Exists AnyTarget {af = ok})
badExistsAnyTarget MkAnyTargetFree impossible


-- Nor inside the reference-matches frame: "if it's any target" would
-- test a phrase that fixes the class by rule rather than describing the
-- referent.
public export
badMatchesAnyTarget : Unspellable (Effect []) (\ok =>
  If (Macros.destroy (Macros.target Macros.artifact)) (Macros.itsA AnyTarget {af = ok}) Nothing)
badMatchesAnyTarget MkAnyTargetFree impossible


-- The condition's SUBJECT is a read and never a mention. This is the
-- refusal that keeps `condDelta`'s opacity honest instead of merely
-- convenient: "If target creature has toughness 5 or greater, it gets
-- +4/-4 until end of turn" (Blood Lust) announces a target inside the
-- if-clause that the consequent then reads, which [CR#601.2c] makes
-- legitimate — targets are announced as the spell is cast, whatever
-- clause spells them — and which this grammar cannot represent without
-- letting conditions bind. Ten corpus lines write "if target …"; the
-- family is refused whole rather than mis-bound, and waits on the
-- ledger.
public export
badMatchesTargetSubject : Unspellable (Condition []) (\ok =>
  Matches (Macros.target Macros.creature) Macros.artifact {bl = ok})
badMatchesTargetSubject MkBindingless impossible


-- A description that says NOTHING tests nothing. The copular frame does
-- not demand a HEAD — "if it's attacking" and "if it's tapped" are real
-- oracle and head nothing — so the empty conjunction slipped past the
-- gate the existential uses (`Headed`) and had to be refused by the
-- weaker one instead (`predSays`). Every corpus line writes at least one
-- word after the copula.
public export
badMatchesNothing : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target Macros.creature),
                If (Macros.gainsLife You (Lit 1)) (Matches It (And []) {sy = ok}) Nothing])
badMatchesNothing MkPredSays impossible


-- A condition negates a POSITIVE description. "If it isn't a
-- non-artifact" is a negation of a negation, which the predicate layer
-- already refuses of itself (`negatable (Not _) = False`,
-- `badDoubleNegation`) and which the condition frame could launder by
-- taking its "not" of an already-negative phrase. Corpus writes the
-- single negation everywhere ([CR#701.47a]'s "If it isn't a [subtype]",
-- `amassZombiesTwo`) and the doubled one nowhere.
public export
badNegatedNegativeMatch : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target Macros.creature),
                If (Macros.gainsLife You (Lit 1))
                   (Macros.notSo (Matches It (Not Macros.artifact)) {ng = ok})
                   Nothing])
badNegatedNegativeMatch MkCondNegatable impossible


-- A trailing condition is EVALUATED before the clause it modifies and
-- WRITTEN after it, and reading it in the clause's post-state let it ask
-- about a world the clause had not made: "Destroy target creature if
-- it's in a graveyard" typechecked because the destroy had already
-- retagged its own target. Chapter twenty-one types the condition in
-- `preIntro` — what the clause's phrases ANNOUNCED, with the announced
-- zone — so the subject here is on the battlefield, and `ZoneFits`
-- refuses the description that puts it elsewhere ([CR#109.2a]). Overload
-- is unaffected: mana value belongs to every object [CR#202.3] and is
-- read zone-free.
public export
badTrailingPostStateZone : Unspellable (Effect []) (\ok =>
  If (Macros.destroy (Macros.target Macros.creature)) (Matches It (InZone Macros.graveyardZ) {zc = ok}) Nothing)
badTrailingPostStateZone MkZoneFits impossible


-- A written comparison measures a READ against a written value, and a
-- numeral is not a read: "if 3 is 4 or greater" states an arithmetic
-- fact, not a fact about the game. The subject table is `writtenBound`'s
-- exact complement, and this is the cell where they differ most
-- visibly.
public export
badCompareLiteralSubject : Unspellable (Condition []) (\ok =>
  CompareAmt (Lit 3) OrGreater (Lit 4) {rd = ok})
badCompareLiteralSubject MkReadAmount impossible


-- The announced X is a written value too ([CR#107.3a]) — the OTHER
-- amount `writtenBound` says yes to — so it is a bound and never a
-- subject. No corpus line compares a bare X against a numeral; where X
-- is tested at all, what is measured is the phrase X was defined from.
public export
badCompareXSubject : Unspellable (Condition []) (\ok =>
  CompareAmt XVal OrGreater (Lit 4) {rd = ok})
badCompareXSubject MkReadAmount impossible


-- And the bound stays written on this side of the frame as well. The
-- phrasal standard ("less than or equal to the number of Islands you
-- control") is real oracle English and a real comparison, and it is
-- still the other frame's — with its own word order and its own
-- comparator words — so the condition frame refuses it exactly as the
-- postnominal qualifier does (`badPhrasalBound`, chapter sixteen).
public export
badConditionPhrasalBound : Unspellable (Condition []) (\ok =>
  CompareAmt (CountOf Macros.creatureYouControl) OrGreater (Macros.powerOf This) {wb = ok})
badConditionPhrasalBound MkWrittenBound impossible


-- A condition introduces nothing. The mention written inside one is
-- reachable while the condition is being written — that is the
-- telescope — but the clause that follows the conditional cannot read
-- it, because the condition may have been false and then there was no
-- such creature to speak of. `predDelta (Or _) = []` at clause level,
-- and refused by the pronoun's own uniqueness gate rather than by a
-- rule about conditions.
public export
badConditionAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [If (Macros.gainsLife You (Lit 2)) (Exists Macros.creatureYouControl) Nothing,
                Tap (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badConditionAntecedent (Refl, _) impossible


-- The two branches of a may are not two spellings of one arm. The
-- if-you-DON'T arm runs exactly when the body did not, so the body's
-- phrase never named anything: "You may sacrifice a creature. If you
-- don't, exile it." is unwritable, and the corpus writes no such line —
-- every declined arm read this pass reaches the decider or the
-- sentences before the may. The taken arm has the opposite type and
-- reads the body in full (`darettisMinusOne`).
public export
badIfNotReadsMayBody : Unspellable (Effect []) (\ok =>
  Macros.mayElse You (Macros.sacrifice You (Macros.a Macros.creature)) (Macros.exile (It {ok})))
badIfNotReadsMayBody Refl impossible


-- A subtype sits on one card type's own set ([CR#205.1a] names the six
-- sets; [CR#205.3m] makes every subtype here a creature type), so a
-- token whose line names
-- a subtype its types cannot carry describes nothing: a "Zombie artifact
-- token" writes a creature type onto an object with no creature type.
public export
badZombieArtifactToken : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (1, 1)) [Black] (MkTypeLine [Zombie] [Artifact])
                          [] Nothing) {sf = ok})
badZombieArtifactToken MkSubtypesFit impossible


-- A token has only the characteristics its creating effect defines
-- ([CR#111.3]), so a creature token's power and toughness ([CR#208.1])
-- have to be written — and every corpus creature-token line writes them.
-- (The converse is deliberately NOT demanded: a Vehicle token carries a
-- P/T with no creature type, [CR#301.7].)
public export
badCreatureTokenNoPt : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken Nothing [White] (MkTypeLine [Soldier] [Creature]) [] Nothing) {tp = ok})
badCreatureTokenNoPt MkTokenPt impossible


-- A token is a PERMANENT ([CR#111.1]), so its line names at least one
-- card type. The type-less spelling is the predefined name ("create a
-- Treasure token", [CR#111.10]) — core's own separate `TokenSpec::Named`
-- row — and it waits with that catalog. (Probed with no subtype either,
-- so the refusal is the missing type and not a subtype with nowhere to
-- sit.)
public export
badTypelessToken : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (1, 1)) [White] (MkTypeLine [] []) [] Nothing) {tt = ok})
badTypelessToken MkTokenTyped impossible


-- The arrival riders are not a free product: sixty-six corpus lines
-- create a token "tapped and attacking" and a hundred fifty-nine write
-- the prenominal "a tapped … token", while ATTACKING WITHOUT TAPPED is
-- written zero times. The rules are why the pair is written out rather
-- than derived: [CR#508.4] gives the attacking designation to a creature
-- put onto the battlefield attacking and taps nothing — the tap belongs
-- to the declare-attackers turn-based action ([CR#508.1f]), which such a
-- creature never went through — so a writer who wants both has to say
-- both, and every writer does.
public export
badAttackingUntapped : Unspellable (Effect []) (\ok =>
  Create You (Lit 1) (Macros.creatureTok 1 1 [Red] [Soldier]) [EntersAttacking] {rr = ok})
badAttackingUntapped MkRidersOk impossible


-- The GRAVEYARD is the counter table's measured silence and stays shut
-- with the zone gate widened: all seventeen lines writing "counter" and
-- "in a graveyard" together put the counter on a battlefield object and
-- read the graveyard for a count, or return the card to the battlefield
-- first ("Return this card from your graveyard to the battlefield with a
-- finality counter on it").
public export
badPutCountersGraveyard : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) Macros.plusOnePlusOne (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok})
badPutCountersGraveyard MkCounterHolder impossible


-- …and the removal twin reads the same fold-state: a destroyed referent
-- has no counters to take off. The destination is what decides — the
-- same clause with an EXILE in front of it is Jhoira of the Ghitu.
public export
badRemoveCountersDead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                RemoveCounters (Lit 1) Macros.plusOnePlusOne It {zn = ok}])
badRemoveCountersDead MkCounterHolder impossible


-- A token's stated characteristics ARE its text ([CR#111.3]), so the
-- bundle is a surface phrase and not a set of facts about an object: a
-- color written twice is a word written twice, and no corpus line writes
-- one. The order is likewise the phrase's: measured over supported
-- oracle, "artifact creature" runs five hundred ninety-four lines to
-- "creature artifact"'s none, and the four type words this vocabulary
-- has fall into one total order (`typeRank`). Colors take the
-- duplicate demand and NOT the ordering one, because Additive Evolution
-- writes "a 0/0 green and blue Fractal creature token" and the mana
-- order would have spelled it the other way round.
public export
badTokenTypeOrder : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (1, 1)) [] (MkTypeLine [] [Creature, Artifact])
                          [] Nothing) {tc = ok})
badTokenTypeOrder MkTokenCanonical impossible


public export
badTokenDuplicateColor : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (Macros.creatureTok 1 1 [White, White] [Soldier]) {tc = ok})
badTokenDuplicateColor MkTokenCanonical impossible


-- A written action count is at least one. [CR#121.1] makes a draw the
-- movement of a card, [CR#111.1] a token a marker put onto the
-- battlefield, [CR#122.1] a counter a marker placed on something, and a
-- zero of any of them instructs nothing — which is why no corpus line
-- spells one, as a numeral or as a determiner, in any scope. What stays
-- writable is the count that EVALUATES to zero: X is announced zero
-- (its controller's to choose and announce, [CR#107.3a]) and a for-each
-- domain can be empty, so only the literal
-- spelling is refused (`writtenCount`).
public export
badDrawZero : Unspellable (Effect []) (\ok =>
  Macros.drawCards 0 {wc = ok})
badDrawZero MkWrittenCount impossible


public export
badCreateZero : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 0) (Macros.creatureTok 1 1 [White] [Soldier]) {wc = ok})
badCreateZero MkWrittenCount impossible


public export
badPutZeroCounters : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 0) Macros.plusOnePlusOne (Macros.target Macros.creature) {wc = ok})
badPutZeroCounters MkWrittenCount impossible


-- A creature subtype has nowhere to sit on a land ([CR#205.1a] — a
-- subtype correlated with a card type the object doesn't have is not one
-- of its subtypes), so the added line has to name the card type itself,
-- as "becomes a Spirit artifact creature" does, or find it on the
-- subject, as amass's "it becomes a Zombie" does of an Army creature
-- ([CR#701.47a]).
public export
badBecomesZombieLand : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.land) (Macros.subtypesOnly [Zombie]) Nothing {af = ok})
badBecomesZombieLand MkAddedFits impossible


-- "Becomes in addition to its other types" has to say WHAT: an empty
-- type line adds nothing and spells no phrase.
public export
badBecomesNothing : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (MkTypeLine [] []) Nothing
                 {ne = Builtin.fst ok, nw = Builtin.snd ok})
badBecomesNothing (MkLineNonEmpty, _) impossible


-- A type change is a continuous effect on a permanent: a graveyard card
-- has no types for the clause to add to on the battlefield.
public export
badBecomesInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) (Macros.typesOnly [Artifact]) Nothing {zn = ok})
badBecomesInGraveyard MkZoneFits impossible


-- The combat endpoint stays the GRANTS' alone. Chapter seventeen could
-- not tell "until end of turn" and "until end of combat" apart, both
-- being written by the stat delta and the keyword grant and by neither
-- restriction; the type addition tells them apart, writing eighteen
-- end-of-turn lines and no end-of-combat line at all — which is why
-- `BothGrants` split and `GrantsAndTypes` exists.
public export
badBecomesUntilEndOfCombat : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.untilEndOfCombat) {sp = ok})
badBecomesUntilEndOfCombat SpanStated impossible


-- …and the restriction's current-turn word is not the type addition's
-- either. Coward // Killer writes both adverbials in one sentence and
-- gives "this turn" to the "can't block" half, which is the division
-- stated by a card rather than by a count.
public export
badBecomesThisTurn : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.thisTurn) {sp = ok})
badBecomesThisTurn SpanStated impossible


-- …and the upkeep endpoint stays the keyword grant's alone, as it was
-- against the stat delta (`badGetsUntilYourNextUpkeep`): two corpus
-- lines write "until your next upkeep" and both grant an ability.
public export
badBecomesUntilYourNextUpkeep : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.untilYourNextUpkeep) {sp = ok})
badBecomesUntilYourNextUpkeep SpanStated impossible


-- A subtype word PRESUPPOSES its set's card type ([CR#205.1a]), so "an
-- Army that isn't a creature" describes nothing — the finding-43 shape
-- again, the presupposition riding the word and the existing coherence
-- gate supplying the refusal. What it does NOT do is project that type,
-- which is what keeps `clavilenoPhrase` writable.
public export
badZombieNoncreature : Unspellable (Predicate [] Object) (\ok =>
  And [HasSubtype Zombie, Not Macros.creature] {cf = ok})
badZombieNoncreature MkContradictionFree impossible


-- "In addition to its other types" RETAINS what the object had and
-- states what it gains ([CR#205.1b]), so a clause that states only what
-- its subject already is states nothing at all. Tezzeret's adds creature
-- to an ARTIFACT and Neurok Transmuter's adds artifact to a CREATURE;
-- no line adds a type to a subject that already heads it. Subtypes are
-- never provably redundant here — no mention carries its subtypes — and
-- [CR#701.47a] guards that case in the text instead, with a condition
-- ("If it isn't a [subtype], …") rather than a grammar rule.
public export
badBecomesOwnType : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Creature]) Nothing {nw = ok})
badBecomesOwnType MkAddsSomething impossible


-- No line writes a NAKED type addition across turns. Chapter nineteen
-- opened the cell on one apparent witness and flagged it for
-- re-measurement; the re-measurement is chapter twenty-one's and closes
-- it. Two hundred sixty-four supported lines write "in addition to
-- its/their/his/her other types" and exactly three carry "until your
-- next turn": Rootwise Survivor's duration belongs to the separate haste
-- grant in the next sentence, Absorbing Man's clause is a copy
-- construction, and Tezzeret, Cruel Machinist's "becomes a 5/5 creature
-- in addition to its other types" fuses a base power/toughness setting
-- onto the addition — the compound construction this vocabulary has no
-- word for, and which waits on the ledger.
public export
badTypeAdditionAcrossTurns : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target (And [Macros.artifact, ControlledBy You]))
          (Macros.typesOnly [Creature])
          (Just Macros.untilYourNextTurn) {sp = ok})
badTypeAdditionAcrossTurns SpanStated impossible


-- The "Otherwise" arm runs exactly when the condition was false, so the
-- clause it replaces never happened and its phrase never named anything:
-- an arm that reads the if-arm's token is unwritable, which is `May`'s
-- declined-arm asymmetry (`badIfNotReadsMayBody`) one construction over.
-- The corpus writes no such line either — every else arm read this pass
-- reaches the sentences before the conditional or nothing at all.
public export
badOtherwiseReadsIfArm : Unspellable (Effect []) (\ok =>
  If (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [Zombie]))
     (Exists Macros.creatureYouControl)
     (Just (Tap (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok})))
badOtherwiseReadsIfArm (Refl, _) impossible


-- A modal offers TWO OR MORE options ([CR#700.2] says so in its own
-- definition), so a one-mode "modal" is not one — it is the sentence
-- itself with a choice clause bolted on front, and no card writes it.
public export
badModalOneMode : Unspellable (Effect []) (\ok =>
  Modal (Macros.upTo 1) [Macros.destroy (Macros.target Macros.artifact)] {tw = ok})
badModalOneMode TwoUp impossible


-- A headcount that fixes the whole list instructs no choice: "Choose two
-- —" over exactly two modes has one answer, and [CR#700.2] calls a spell
-- modal for the INSTRUCTIONS to choose. Zero cards write it — every
-- printed exact headcount is strictly under its list, and the two forms
-- whose top reaches the list ("one or both", "one or more") are ranges.
public export
badModalFixedWhole : Unspellable (Effect []) (\ok =>
  Macros.chooseTwo [Macros.destroy (Macros.target Macros.artifact),
                    Macros.destroy (Macros.target Macros.enchantment)] {mf = ok})
badModalFixedWhole MkModesFit impossible


-- Nor may a headcount reach PAST the list: three of two options names
-- nothing at all.
public export
badModalOverreach : Unspellable (Effect []) (\ok =>
  Modal (Macros.exactly 3) [Macros.destroy (Macros.target Macros.artifact),
                            Macros.destroy (Macros.target Macros.enchantment)] {mf = ok})
badModalOverreach MkModesFit impossible


-- The mode list is a LIST and not a telescope: the modes are chosen at
-- cast ([CR#700.2a]) and an unchosen one's targets are never announced
-- ([CR#700.2c] — the spell is "treated as though it did not have those
-- targets"), so a mode that reads a sibling's mention reads something
-- that may never have existed. Every bullet in the corpus
-- that opens with a pronoun reaches PAST the modal to the trigger before
-- it — Kogla and Yidaro's two modes both read the same outside antecedent
-- and neither reads the other — and zero read a sibling.
public export
badModalReadsAcrossModes : Unspellable (Effect []) (\ok =>
  Macros.chooseOne [Macros.destroy (Macros.target Macros.artifact),
                    Tap (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badModalReadsAcrossModes (_, OnField) impossible


-- And nothing after the modal reads into it, for the same reason pointed
-- forward: Blood on the Snow writes "Then return a creature or
-- planeswalker card … from your graveyard" — a description covering both
-- modes' outcomes — exactly where an anaphor would have gone.
public export
badReadsAfterModal : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.chooseOne [Macros.destroy (Macros.target Macros.artifact), Macros.destroy (Macros.target Macros.enchantment)],
                Tap (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badReadsAfterModal (_, OnField) impossible


-- WHICH condition frames take a "not" is a closed table. The comparison
-- frame does not: a bound has a negative of its own, and English writes
-- that instead — "if its power isn't 4 or greater" is written zero times.
public export
badNegatedComparisonCondition : Unspellable (Effect []) (\ok =>
  If (Macros.destroy (Macros.target Macros.artifact))
     (Macros.notSo (CompareAmt (Macros.manaValueOf It) OrLess (Lit 2)) {ng = ok})
     Nothing)
badNegatedComparisonCondition MkCondNegatable impossible


-- Nor does a negation take one: no corpus line writes a condition under
-- two of them, English collapsing the pair into the positive.
public export
badDoubleNegatedCondition : Unspellable (Effect []) (\ok =>
  If (Macros.destroy (Macros.target Macros.artifact))
     (Macros.notSo (Macros.notSo (Exists Macros.creatureYouControl)) {ng = ok})
     Nothing)
badDoubleNegatedCondition MkCondNegatable impossible


-- A drawn card is not a mention. The corpus never reads one back across a
-- sentence boundary — "Draw a card." followed by "it" or "that card" is
-- written zero times, and the card IS read only inside the coordination
-- that reveals it ("Draw a card and reveal it. If it isn't a land card,
-- discard it."), a verb-phrase coordination this grammar does not spell.
public export
badDrawnCardRemention : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.drawACard, Macros.exile (That CardW {ok})])
badDrawnCardRemention Refl impossible


-- The modal headcount vocabulary is CLOSED over what oracle writes, not
-- over what the range algebra permits. "Choose up to two —" and "Choose
-- up to three —" are written zero times in any scope, where "Choose up
-- to one —" heads five lines, so the "up to" head is capped at one and
-- the range that would spell the others is refused (`modalHead`). The
-- fixed counts stop at three (Mishra, Eminent One) and the two open tops
-- take their maximum from the list, which is what "or both" and "or
-- more" say.
public export
badModalUpToTwo : Unspellable (Effect []) (\ok =>
  Modal (Macros.upTo 2) [Macros.destroy (Macros.target Macros.artifact),
                        Macros.destroy (Macros.target Macros.enchantment),
                        Macros.drawACard] {mh = ok})
badModalUpToTwo MkModalHead impossible


-- Two identical modes are one mode written twice, and the choice between
-- them decides nothing — [CR#700.2] wants "instructions for a player to
-- choose a number of those options", and [CR#700.2d] has a player
-- normally unable to "choose the same mode more than once", the cards
-- that lift it saying so in words rather than by printing the bullet
-- twice. The check is structural and conservative (`effEq`, `predEq`'s
-- discipline one layer up): it catches the degenerate repetition, not
-- every semantic twin.
public export
badDuplicateModes : Unspellable (Effect []) (\ok =>
  Macros.chooseOne [Macros.drawACard, Macros.drawACard] {dm = ok})
badDuplicateModes Refl impossible


-- A choice BINDS a new referent out of a described set, so the phrase
-- has to describe one. Corpus writes "choose a/an …", "choose target …",
-- "choose two …", "choose up to …", "choose any number of …", "choose
-- another …" — selections, every one — and writes "choose you", "choose
-- it", and "choose them" zero times each. Choosing an already definite
-- participant selects among nothing and would announce a mention the
-- clause did not bind, which is chapter twenty's introduction discipline
-- asked of the choice clause (`choosable`). "Choose a player" and
-- "Choose an opponent" are unaffected — the gate is about the
-- determiner, not the kind (`myrkulsEdict`).
public export
badChooseYou : Unspellable (Effect []) (\ok =>
  Choose You {ch = ok})
badChooseYou MkChoosable impossible


-- A conditioned clause is a HOLE: the condition may be false, and then
-- the clause never ran and its phrase named nothing. So the token
-- "create a 1/1 white Soldier creature token if you control a creature"
-- may make cannot be the "it" of the sentence after the conditional —
-- which is the else arm's rule (`badOtherwiseReadsIfArm`) and the
-- declined may's (`badIfNotReadsMayBody`) read from outside instead of
-- from inside. Oracle writes the re-binding rather than the anaphor
-- where it means to reach the object: amass's second sentence chooses an
-- Army rather than saying "it" of the token its first sentence may have
-- created ([CR#701.47a], `amassZombiesTwo`).
public export
badConditionalArmAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [If (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier]))
                   (Exists Macros.creatureYouControl)
                   Nothing,
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badConditionalArmAntecedent (_, MkCounterHolder) impossible


-- With BOTH arms written, the may exports its BODY and neither arm.
-- [CR#118.12] says why: the branch checks "whether the player chose to
-- pay an optional cost … regardless of what events actually occurred",
-- so exactly one arm ran and the sentences after the may cannot know
-- which — reading the if-you-do arm's token here is reading one branch
-- as though it were both. Crovax the Cursed is the positive that writes
-- the pair (`crovaxTheCursed`), and it reads neither arm afterward.
public export
badBothArmsAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [May (Just You) (Macros.gainsLife You (Lit 1))
                     (Just (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier])))
                     (Just (Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [Soldier]))),
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badBothArmsAntecedent (_, MkCounterHolder) impossible


-- The complement's anchor may not be an INDEFINITE: "each other
-- creature" announces one phrase, and "other than a creature" would
-- announce a second referent the sentence never spelled. Unwritten
-- English, and it stays refused.
public export
badComplementAnchorAnnounces : Unspellable (Predicate [] Object) (\ok =>
  OtherThan (Macros.a Macros.creature) {ca = ok})
badComplementAnchorAnnounces MkComplementAnchor impossible


-- The anchor is SINGULAR, which is the second half of the correction and
-- the one that keeps the deferred family deferred: this constructor
-- subtracts ONE referent, and a plural anchor passed to it is group
-- subtraction — finding 111's subset complement, which no corpus line
-- writes ("other than them/those/these" returns zero lines in supported
-- and all-cards scope alike). Written over a counted target, which asks
-- the number question and nothing else; the plural READ ("other than
-- those creatures") is the same gate a group mention later.
public export
badPluralComplementAnchor : Unspellable (Predicate [] Object) (\ok =>
  OtherThan (TargetGroup (Macros.upTo 2) Macros.creature) {ca = ok})
badPluralComplementAnchor MkComplementAnchor impossible


-- The anchor has to be something the phrase could have described:
-- "each other creature" anchored to a LAND subtracts nothing, and no
-- corpus line pairs "other" with a cross-head anchor — the same
-- evidence `anyTargetedTy` reads for the bare word.
public export
badComplementCrossHead : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Each (And [Macros.creature, OtherThan Macros.thisLand] {oa = ok})))
badComplementCrossHead MkOtherAnchored impossible


-- One selector slot per phrase, whichever spelling fills it: two
-- complements are two "other"s, and the guide gives the word one
-- position.
public export
badDoubleComplement : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1)
             (Each (And [Macros.creature, OtherThan Macros.thisCreature, OtherThan Macros.thisCreature] {oa = ok})))
badDoubleComplement MkOtherAnchored impossible


-- And the two spellings share that slot: a phrase cannot write the bare
-- "other" and an anchored one at once.
public export
badOtherAndComplement : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target Macros.creature),
                DealDamage This (Lit 1)
                           (Each (And [Macros.creature, Other, OtherThan Macros.thisCreature] {oa = ok}))])
badOtherAndComplement MkOtherAnchored impossible


-- A word that fills one phrase-level slot is not an ALTERNATIVE, which
-- is `badOtherInOr`'s refusal reaching the second spelling too.
public export
badComplementInOr : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.creature, OtherThan Macros.thisCreature], Macros.land] {cd = ok})
badComplementInOr MkCoordinableDisjuncts impossible


-- "Non-other" is unwritten, as "non-other" always was.
public export
badNegatedComplement : Unspellable (Predicate [] Object) (\ok =>
  Not (OtherThan Macros.thisCreature) {ng = ok})
badNegatedComplement MkNegatable impossible


-- A batch is at least TWO parts, the arity demand `Sequentially` makes
-- and for its reasons: nothing at all, and a second spelling of one
-- clause.
public export
badEmptySimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously [] {ok})
badEmptySimultaneous TwoUp impossible


public export
badSingletonSimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.destroy (Macros.target Macros.creature)] {ok})
badSingletonSimultaneous TwoUp impossible


-- Its elements are clauses and not batches — the re-minted tree
-- `NotSeq` refuses one construction over.
public export
badNestedSimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously
    ((Simultaneously [Macros.destroy (Macros.target Macros.creature), Macros.destroy (Macros.target Macros.artifact)]
      :: (Macros.destroy (Macros.target Macros.land) :: Nil)) {ns = ok}))
badNestedSimultaneous MkNotSim impossible


-- Nor SEQUENCES: an ordered list inside an unordered one contradicts the
-- container it sits in, and its announcements would reach the next
-- element only from its last clause besides.
public export
badSequenceInsideSimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously
    ((Sequentially [Macros.destroy (Macros.target Macros.creature), Macros.destroy (Macros.target Macros.artifact)]
      :: (Macros.destroy (Macros.target Macros.land) :: Nil)) {nq = ok}))
badSequenceInsideSimultaneous MkNotSeq impossible


-- What a batch's elements share is ONE pre-state — [CR#608.2f]'s rule
-- for one instruction spread over several objects, whose own example is
-- a control grant (Blatant Thievery gains control of every target
-- "simultaneously") — so an element may read what a sibling ANNOUNCED
-- and nothing a sibling DID.
-- The zone retag is the sharp case: after an exile the card word
-- reaches the referent, and inside the batch it does not, because
-- nothing has been exiled yet.
public export
badSimultaneousReadsRetag : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.exile (Macros.target Macros.creature),
                  Macros.destroy (That CardW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badSimultaneousReadsRetag (_, OnField) impossible


-- The event OUTCOME is the same fact for magnitudes: no damage has been
-- dealt when a sibling is typed, so "that much" reads nothing.
public export
badSimultaneousReadsOutcome : Unspellable (Effect []) (\ok =>
  Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                  Macros.gainsLife You (ThatMuch {ok})])
badSimultaneousReadsOutcome Refl impossible


-- The MAY is the same fact through a wrapper, and chapter twenty-six's
-- headline: the batch's telescope threaded `preIntro`, which is not the
-- announcement-only function it was taken for — `preIntro (May …)` is
-- `mayIntro`, the optional clause's DEED. So a sibling could put a
-- counter on a token the player may not have made. [CR#118.12] makes
-- the offer a cost checked by whether the player chose to pay it,
-- "regardless of what events actually occurred", and [CR#608.2f]
-- processes the batch's actions at once, so nothing in the batch's one
-- pre-state can be the token. The telescope threads `annIntro` now.
public export
badSimultaneousReadsMayDeed : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.may You (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant])),
                  PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badSimultaneousReadsMayDeed (_, MkCounterHolder) impossible


-- …and the magnitude twin, which is `badSimultaneousReadsOutcome`
-- reached through the same wrapper.
public export
badSimultaneousReadsMayOutcome : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.may You (DealDamage This (Lit 2) (Macros.target Macros.creature)),
                  Macros.gainsLife You (ThatMuch {ok})])
badSimultaneousReadsMayOutcome Refl impossible


-- OUTWARD, a batch leaves behind EVERY element's deed and not the last
-- one's: two creates in one instruction leave two tokens ([CR#608.2f]
-- processes both actions), so the sentence after cannot say "it".
-- Chapter twenty-two folded the last element's `effIntro` and recorded
-- the simplification; this is the row that shows it was one
-- (`simIntro`, `deedDelta`).
public export
badBatchTwoCreatesThenIt : Unspellable (Effect []) (\ok =>
  Sequentially [Simultaneously [Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant]),
                               Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier])],
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok})])
badBatchTwoCreatesThenIt Refl impossible


-- The magnitude twin outward: two outcomes in one batch, and "that
-- much" does not say which.
public export
badBatchTwoOutcomesThenThatMuch : Unspellable (Effect []) (\ok =>
  Sequentially [Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                               Macros.losesLife (Macros.target Opponent) (Lit 3)],
                Macros.gainsLife You (ThatMuch {ok})])
badBatchTwoOutcomesThenThatMuch Refl impossible


-- Control is a PERMANENT's ([CR#110.2] gives every permanent a
-- controller; [CR#109.4] gives an object that is neither on the stack
-- nor on the battlefield none), so the grant takes a battlefield
-- referent like every other continuous clause.
public export
badGainControlGraveyard : Unspellable (Effect []) (\ok =>
  Macros.gainControl (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) Nothing {zn = ok})
badGainControlGraveyard MkZoneFits impossible
