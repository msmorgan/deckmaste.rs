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
