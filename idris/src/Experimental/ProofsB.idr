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
