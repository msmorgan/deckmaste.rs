module Experimental.ProofsB

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "any target and any target"
||| The class word is written ONCE per phrase; this names one referent twice.
public export
badDoubleAnyTarget : Unspellable (Predicate [] Object) (\ok =>
  And [AnyTarget, AnyTarget] {at = ok})
badDoubleAnyTarget MkAnyTargetLone impossible


||| "another other creature"
||| The selector order gives other/another a single slot per phrase.
public export
badDoubleOther : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing)] Object)
  (\ok => And [Macros.creature, Other, Other] {oa = ok})
badDoubleOther MkOtherAnchored impossible


||| "a creature the controller of any target controls"
||| A relative clause's possessor is a phrase like any other, so this
||| spells "any target" under the non-targeting determiner that forbids it.
public export
badAnyTargetEmbedded : Unspellable (Noun [] Object) (\ok =>
  Macros.a (And [Macros.creature, ControlledBy (ControllerOf (Macros.target AnyTarget))]) {af = ok})
badAnyTargetEmbedded MkAnyTargetFree impossible


||| "You discard it." of a referent nothing has placed
||| A read whose antecedent records no zone is not thereby a hand card, and
||| [CR#701.9a] moves one FROM a hand.
public export
badDiscardIt : Unspellable
  (Effect [MkBinding AD Object OneOf (ObjectP Nothing Nothing Nothing Nothing)])
  (\ok => Macros.discards You It {dk = ok})
badDiscardIt DiscardTracked impossible


||| "You discard any target."
||| The class word heads no zone clause and takes no [CR#109.2] default, so
||| it reaches discard with no zone at all.
public export
badDiscardAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.discards You (Macros.target AnyTarget) {dk = Builtin.fst ok, na = Builtin.snd ok})
badDiscardAnyTarget (_, MkNotAnyTarget) impossible


||| "a creature you control in your graveyard"
||| A controller relation presupposes the battlefield [CR#109.4], so this
||| places its referent in two zones at once.
public export
badControlledInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, ControlledBy You, InZone Macros.graveyardZ] {zc = ok})
badControlledInGraveyard MkZoneCoherent impossible


||| an empty sentence list
||| A sequence of no clauses is no instruction; nothing renders it.
public export
badEmptySequence : Unspellable (Effect []) (\ok =>
  Sequentially [] {ok})
badEmptySequence TwoUp impossible


||| "Destroy target creature." written as a one-element sequence
||| A sequence of one spells what the clause alone spells, and one meaning
||| gets one spelling.
public export
badSingletonSequence : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature)] {ok})
badSingletonSequence TwoUp impossible


||| "Destroy any target."
||| "Any target" NAMES [CR#115.4]'s damage class and so projects no zone;
||| destruction moves a battlefield permanent [CR#701.8a].
public export
badDestroyAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target AnyTarget) {ok = Builtin.fst ok, na = Builtin.snd ok})
badDestroyAnyTarget (OnField, _) impossible


||| "Tap any target."
||| The same silence one verb over: tapping takes a battlefield object
||| [CR#701.26a].
public export
badTapAnyTarget : Unspellable (Effect []) (\ok =>
  Tap (Macros.target AnyTarget) {ok})
badTapAnyTarget OnField impossible


||| "This deals 3 damage to any target. Destroy it."
||| The binding an any-target phrase introduces records the phrase's own
||| silence, so "it" inherits no battlefield.
public export
badDestroyAnyTargetRemention : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.target AnyTarget),
               Macros.destroy It {ok}])
badDestroyAnyTargetRemention OnField impossible


||| "Exile any target."
||| A placement SETS the zone, so nothing contradicts the silence; [CR#115.4]
||| rules it out at the phrase — the class spans players.
public export
badExileAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.exile (Macros.target AnyTarget) {na = ok})
badExileAnyTarget MkNotAnyTarget impossible


||| "Counter any target."
||| [CR#115.4] excludes a spell from the class, and [CR#112.1] makes a spell
||| a card on the stack, so countering demands that zone as evidence.
public export
badCounterAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.counterSpell (Macros.target AnyTarget) {zn = ok})
badCounterAnyTarget OnTheStack impossible


||| "This deals 1 damage to this spell."
||| The recipient gate reads the NOUN, and bare `This` is no damage
||| recipient [CR#120.1,120.1a].
public export
badDamageThis : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) This {rk = ok})
badDamageThis ObjectTakes impossible


||| a coordination of no alternatives
||| A coordination offers alternatives, so it needs two; at zero it spells
||| nothing at all.
public export
badEmptyOr : Unspellable (Predicate [] Object) (\ok =>
  Or [] {tw = ok})
badEmptyOr MkTwoDisjuncts impossible


||| "creature" written as a one-alternative coordination
||| At one alternative a coordination offers none and spells exactly what
||| the bare alternative spells.
public export
badSingletonOr : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.creature] {tw = ok})
badSingletonOr MkTwoDisjuncts impossible


||| "artifact or artifact"
||| An alternative repeated offers a choice between a thing and itself.
public export
badRepeatedDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.artifact, Macros.artifact] {dd = ok})
badRepeatedDisjunct MkDistinctDisjuncts impossible


||| "artifact or attacking"
||| Alternatives are PARALLEL — each must stand where the others stand — and
||| a bare status word leaves the second alternative nothing to be.
public export
badHeadlessDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.artifact, Attacking] {pd = ok})
badHeadlessDisjunct MkParallelDisjuncts impossible


||| "in your hand or in your graveyard"
||| The parallel demand about PLACE: cross-zone alternatives are real oracle
||| under a shared preposition, but this projection names one zone.
public export
badCrossZoneDisjunction : Unspellable (Predicate [] Object) (\ok =>
  Or [InZone Macros.handZ, InZone Macros.graveyardZ] {pd = ok})
badCrossZoneDisjunction MkParallelDisjuncts impossible


||| "any target or creature"
||| "Any target" is ALREADY the union [CR#115.4] fixes, so coordinating it
||| with an alternative re-opens a closed class.
public export
badAnyTargetInOr : Unspellable (Predicate [] Object) (\ok =>
  Or [AnyTarget, Macros.creature] {cd = ok})
badAnyTargetInOr MkCoordinableDisjuncts impossible


||| "any target or creature" with the class word wrapped in a conjunction
||| Every alternative is read through `flattenPs`, so a singleton wrapper
||| launders nothing.
public export
badAnyTargetInOrLaundered : Unspellable (Predicate [] Object) (\ok =>
  Or [And [AnyTarget], Macros.creature] {cd = ok})
badAnyTargetInOrLaundered MkCoordinableDisjuncts impossible


||| "other creature or land"
||| "Other" fills one selector slot for the whole coordinated phrase, so it
||| is not an alternative either.
public export
badOtherInOr : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing)] Object)
  (\ok => Or [And [Macros.creature, Other], Macros.land] {cd = ok})
badOtherInOr MkCoordinableDisjuncts impossible


||| "(creature or land) or artifact"
||| Nesting is the flat coordination written with brackets oracle has no way
||| to print.
public export
badNestedOr : Unspellable (Predicate [] Object) (\ok =>
  Or [Or [Macros.creature, Macros.land], Macros.artifact] {cd = ok})
badNestedOr MkCoordinableDisjuncts impossible


||| "non-(creature or land)"
||| Negation attaches to one modifier at a time; the writer spells
||| "noncreature, nonland card" instead.
public export
badNegatedDisjunction : Unspellable (Predicate [] Object) (\ok =>
  Not (Or [Macros.creature, Macros.land]) {ng = ok})
badNegatedDisjunction MkNegatable impossible


||| "This deals 2 damage to target artifact or enchantment."
||| A disjunctive head fixes no type at all, and damage reaches only a
||| battle, a creature, or a planeswalker [CR#120.1a].
public export
badDamageDisjunctHead : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 2) (Macros.target (Or [Macros.artifact, Macros.enchantment])) {rk = ok})
badDamageDisjunctHead ObjectTakes impossible


||| "attacking or blocking creature in your graveyard"
||| Alternatives that AGREE on a zone still project it, so the referent
||| stands on the battlefield and cannot also be in a graveyard.
public export
badAttackingOrBlockingInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Or [Attacking, Blocking], InZone Macros.graveyardZ] {zc = ok})
badAttackingOrBlockingInGraveyard MkZoneCoherent impossible


||| "noncreature that is attacking or blocking"
||| Only a creature can attack or block [CR#506.3], a presupposition the
||| disjunction inherits from both alternatives at once.
public export
badNoncreatureAttackingOrBlocking : Unspellable (Predicate [] Object) (\ok =>
  And [Not Macros.creature, Or [Attacking, Blocking]] {cf = ok})
badNoncreatureAttackingOrBlocking MkContradictionFree impossible


||| "noncreature that is an attacking artifact or a blocking land"
||| A presupposition written inside an alternative is written all the same:
||| both alternatives demand a creature [CR#506.3].
public export
badWrappedStatusLaunder : Unspellable (Predicate [] Object) (\ok =>
  And [Or [And [Macros.artifact, Attacking], And [Macros.land, Blocking]], Not Macros.creature] {cf = ok})
badWrappedStatusLaunder MkContradictionFree impossible


||| "between three and two target creatures"
||| A range runs upward; a descending pair names an empty interval.
public export
badDescendingRange : Unspellable (Noun [] Object) (\ok =>
  TargetGroup (Range (Just 3) (Just 2)) Macros.creature {wf = ok})
badDescendingRange MkWellFormedQ impossible


||| "attacking artifact or land"
||| Alternatives that place their referent differently are not alternatives:
||| the first stands on the battlefield and the second says nothing.
public export
badPartialZoneJoin : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.artifact, Attacking], Macros.land] {pd = ok})
badPartialZoneJoin MkParallelDisjuncts impossible


||| "zero or more target creatures"
||| A second spelling of "any number of target creatures", [CR#107.1c]
||| already having "any number" permit zero.
public export
badZeroLowerRange : Unspellable (Noun [] Object) (\ok =>
  TargetGroup (Range (Just 0) Nothing) Macros.creature {wf = ok})
badZeroLowerRange MkWellFormedQ impossible


||| "another target artifact or enchantment" anchored to an earlier land
||| A coordinated head offers one type per alternative, and a land anchors
||| none of them.
public export
badDisjunctiveOtherCrossHead : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Land) (Just Battlefield) Nothing Nothing)] Object)
  (\ok => And [Or [Macros.artifact, Macros.enchantment], Other] {oa = ok})
badDisjunctiveOtherCrossHead MkOtherAnchored impossible


||| "creature you control or creature you control"
||| The same repetition spelled with a modifier: member equality looks
||| inside the conjunction too.
public export
badRepeatedStructuredDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.creature, ControlledBy You], And [Macros.creature, ControlledBy You]] {dd = ok})
badRepeatedStructuredDisjunct MkDistinctDisjuncts impossible


||| a sequence written as one element of a sequence
||| A sequence's ELEMENTS are clauses; nesting re-mints the right-nested
||| tree the n-ary list replaced.
public export
badNestedSequence : Unspellable (Effect []) (\ok =>
  Sequentially
    ((Sequentially [Macros.destroy (Macros.target Macros.creature), Macros.exile (Macros.target Macros.creature)]
      :: (Macros.destroy (Macros.target Macros.land) :: Nil)) {ns = ok}))
badNestedSequence MkNotSeq impossible


||| "Target land can't attack this turn."
||| Only a creature can attack or block [CR#506.3], so a land has no grant
||| for the deed to remove.
public export
badCantAttackLand : Unspellable (Effect []) (\ok =>
  Macros.cantAttack (Macros.target Macros.land) (Just Macros.thisTurn) {dp = ok})
badCantAttackLand Participant impossible


||| "Target creature or land can't block this turn."
||| A coordinated head fixes no type, and an untyped head cannot prove
||| participation — the silence is not permission.
public export
badCantDisjunctSubject : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target (Or [Macros.creature, Macros.land])) (Just Macros.thisTurn) {dp = ok})
badCantDisjunctSubject Participant impossible


||| "Target creature can't be attacked this turn."
||| The phrase is real oracle, but only a player, a planeswalker, or a
||| battle can be attacked [CR#506.3].
public export
badCantBeAttacked : Unspellable (Effect []) (\ok =>
  Continuously (Cant (Macros.target Macros.creature) Attack Patient {dp = ok}) (Just Macros.thisTurn))
badCantBeAttacked Participant impossible


||| "Target creature card in a graveyard can't block this turn."
||| A permanent that leaves the battlefield is removed from combat
||| [CR#506.4], so a graveyard card has no deed to be denied.
public export
badCantInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) (Just Macros.thisTurn) {zn = ok})
badCantInGraveyard MkZoneFits impossible


||| "Any target can't block this turn."
||| The class word names [CR#115.4]'s damage class and describes no object,
||| so it fixes no type for the deed's head demand.
public export
badCantAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target AnyTarget) (Just Macros.thisTurn) {dp = ok})
badCantAnyTarget Participant impossible


||| "Target creature can't block until end of turn."
||| The one-shot restriction's word is "this turn"; "until end of turn"
||| belongs to the grants.
public export
badCantUntilEndOfTurn : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target Macros.creature) (Just Macros.untilEndOfTurn) {sp = ok})
badCantUntilEndOfTurn SpanStated impossible


||| "Target creature gains flying this turn."
||| The inverse, which keeps the restriction's word from opening a hole: no
||| line grants an ability "this turn".
public export
badGainsThisTurn : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying) (Just Macros.thisTurn) {sp = ok})
badGainsThisTurn SpanStated impossible


||| "Target creature gets +3/+3 this turn."
||| The stat change writes the grant's adverbial too — "until end of turn",
||| never "this turn".
public export
badGetsThisTurn : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) 3 3 (Just Macros.thisTurn) {sp = ok})
badGetsThisTurn SpanStated impossible


||| "Target creature can't block." with no duration
||| A durationless "can't" is the STATIC ability line, a different
||| construction; the restriction row is the one that may not omit the span.
public export
badStaticCant : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target Macros.creature) Nothing {sp = ok})
badStaticCant SpanUnstated impossible


||| "Target creature gets +3/+3 until your next upkeep."
||| The upkeep endpoint belongs to the KEYWORD grant alone; no stat change
||| takes it.
public export
badGetsUntilYourNextUpkeep : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) 3 3 (Just Macros.untilYourNextUpkeep) {sp = ok})
badGetsUntilYourNextUpkeep SpanStated impossible


||| "Target creature gains flying until the end of your next turn."
||| Real oracle English, but every line writing it is a play permission, a
||| control grant, or a can't-cast — never a keyword grant.
public export
badGainsUntilEndOfYourNextTurn : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying) (Just (Until (EndOf Turn (Just Yours)))) {sp = ok})
badGainsUntilEndOfYourNextTurn SpanStated impossible


||| "Target creature can't block until end of combat."
||| The combat endpoint is the grants' too; the restriction's two words stay
||| "this turn" and "until your next turn".
public export
badCantUntilEndOfCombat : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target Macros.creature) (Just Macros.untilEndOfCombat) {sp = ok})
badCantUntilEndOfCombat SpanStated impossible


||| "Target creature gains flying until your next untap step."
||| The unattested end of the table: no line ends a duration at an untap
||| step in words this vocabulary has.
public export
badGainsUntilUntapStep : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying)
        (Just (Until (StartOf UntapStep (Just Yours)))) {sp = ok})
badGainsUntilUntapStep SpanStated impossible


||| "target with power 2 or less"
||| A bound is a MODIFIER: it describes something and names nothing, so it
||| cannot be what a determiner determines.
public export
badBareComparison : Unspellable (Noun [] Object) (\ok =>
  Macros.target (Compare Power OrLess (Lit 2)) {hd = ok})
badBareComparison MkHeaded impossible


||| "noncreature with power 2 or less"
||| Only a creature has power [CR#208.3], so a phrase that bounds power and
||| denies the type describes nothing.
public export
badNoncreaturePower : Unspellable (Predicate [] Object) (\ok =>
  And [Compare Power OrLess (Lit 2), Not Macros.creature] {cf = ok})
badNoncreaturePower MkContradictionFree impossible


||| "not with power 2 or less"
||| Oracle flips the comparator instead, and can: the game's numbers are
||| integers [CR#107.1], leaving no gap for a negation to name.
public export
badNegatedComparison : Unspellable (Predicate [] Object) (\ok =>
  Not (Compare Power OrLess (Lit 2)) {ng = ok})
badNegatedComparison MkNegatable impossible


||| "creature with power 2 or less and power 4 or greater"
||| One phrase, one bound: an interval is a construction with its own word,
||| not two qualifiers stacked.
public export
badDoubleComparison : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Compare Power OrLess (Lit 2),
       Compare Power OrGreater (Lit 4)] {lc = ok})
badDoubleComparison MkLoneComparison impossible


||| "with this creature's power or less"
||| The bound is WRITTEN — a numeral or the announced X. Oracle spells a
||| phrasal standard in the other frame's word order.
public export
badPhrasalBound : Unspellable (Predicate [] Object) (\ok =>
  Compare Power OrLess (Macros.powerOf This) {wb = ok})
badPhrasalBound MkWrittenBound impossible


||| "with power 2 or less or with power 2 or less"
||| An alternative repeated word for word is no alternative, and a bound is
||| compared by all three of its written parts.
public export
badRepeatedComparisonDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Compare Power OrLess (Lit 2),
      Compare Power OrLess (Lit 2)] {dd = ok})
badRepeatedComparisonDisjunct MkDistinctDisjuncts impossible


||| "with power 2 or less or mana value 3 or less"
||| Alternatives must presuppose alike: a power bound demands a creature
||| where a mana value bound demands nothing.
public export
badMixedCharacteristicDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Compare Power OrLess (Lit 2),
      Compare ManaValue OrLess (Lit 3)] {pd = ok})
badMixedCharacteristicDisjunct MkParallelDisjuncts impossible


||| "any target with power 2 or less"
||| [CR#115.4] fixes the class by rule and a bound would narrow it, so the
||| class word takes no qualifier but "other".
public export
badAnyTargetComparison : Unspellable (Predicate [] Object) (\ok =>
  And [AnyTarget, Compare Power OrLess (Lit 2)] {at = ok})
badAnyTargetComparison MkAnyTargetLone impossible


||| "if you control attacking"
||| A condition quantifies over a DESCRIPTION, so the phrase inside it has
||| to name something.
public export
badExistsUnheaded : Unspellable (Condition []) (\ok =>
  Exists Attacking {hd = ok})
badExistsUnheaded MkHeaded impossible


||| "if you control any target"
||| The class word NAMES [CR#115.4]'s damage class and describes no object,
||| so there is nothing for an existential to be true of.
public export
badExistsAnyTarget : Unspellable (Condition []) (\ok =>
  Exists AnyTarget {af = ok})
badExistsAnyTarget MkAnyTargetFree impossible


||| "Destroy target artifact if it's any target."
||| Nor inside the reference-matches frame: the class word fixes the class
||| by rule rather than describing the referent.
public export
badMatchesAnyTarget : Unspellable (Effect []) (\ok =>
  If (Macros.destroy (Macros.target Macros.artifact)) (Macros.itsA AnyTarget {af = ok}) Nothing)
badMatchesAnyTarget MkAnyTargetFree impossible


||| "if target creature is an artifact"
||| The condition's SUBJECT is a read, never a mention: "if target …" is
||| rules-legitimate [CR#601.2c] but unrepresentable here, so refused whole.
public export
badMatchesTargetSubject : Unspellable (Condition []) (\ok =>
  Matches (Macros.target Macros.creature) Macros.artifact {bl = ok})
badMatchesTargetSubject MkBindingless impossible


||| "Tap target creature. You gain 1 life if it's."
||| A description that says NOTHING tests nothing; every corpus line writes
||| at least one word after the copula.
public export
badMatchesNothing : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target Macros.creature),
                If (Macros.gainsLife You (Lit 1)) (Matches It (And []) {sy = ok}) Nothing])
badMatchesNothing MkPredSays impossible


||| "Tap target creature. You gain 1 life if it isn't a nonartifact."
||| A condition negates a POSITIVE description; the doubled negation the
||| predicate layer already refuses of itself is not laundered here.
public export
badNegatedNegativeMatch : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target Macros.creature),
                If (Macros.gainsLife You (Lit 1))
                   (Macros.notSo (Matches It (Not Macros.artifact)) {ng = ok})
                   Nothing])
badNegatedNegativeMatch MkCondNegatable impossible


||| "Destroy target creature if it's in a graveyard."
||| A trailing condition is EVALUATED before the clause it modifies, so its
||| subject is the announced one — on the battlefield [CR#109.2a].
public export
badTrailingPostStateZone : Unspellable (Effect []) (\ok =>
  If (Macros.destroy (Macros.target Macros.creature)) (Matches It (InZone Macros.graveyardZ) {zc = ok}) Nothing)
badTrailingPostStateZone MkZoneFits impossible


||| "if 3 is 4 or greater"
||| A written comparison measures a READ against a written value, and a
||| numeral states an arithmetic fact, not a fact about the game.
public export
badCompareLiteralSubject : Unspellable (Condition []) (\ok =>
  CompareAmt (Lit 3) OrGreater (Lit 4) {rd = ok})
badCompareLiteralSubject MkReadAmount impossible


||| "if X is 4 or greater"
||| The announced X is a written value too [CR#107.3a], so it is a bound and
||| never a subject.
public export
badCompareXSubject : Unspellable (Condition []) (\ok =>
  CompareAmt XVal OrGreater (Lit 4) {rd = ok})
badCompareXSubject MkReadAmount impossible


||| "if the number of creatures you control is this creature's power or greater"
||| The bound stays written on this side of the frame as well; a phrasal
||| standard is real oracle in the other frame's word order.
public export
badConditionPhrasalBound : Unspellable (Condition []) (\ok =>
  CompareAmt (CountOf Macros.creatureYouControl) OrGreater (Macros.powerOf This) {wb = ok})
badConditionPhrasalBound MkWrittenBound impossible


||| "You gain 2 life if you control a creature. Tap it."
||| A condition introduces nothing: it may have been false, and then there
||| was no such creature to speak of.
public export
badConditionAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [If (Macros.gainsLife You (Lit 2)) (Exists Macros.creatureYouControl) Nothing,
                Tap (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badConditionAntecedent (Refl, _) impossible


||| "You may sacrifice a creature. If you don't, exile it."
||| The if-you-DON'T arm runs exactly when the body did not, so the body's
||| phrase never named anything.
public export
badIfNotReadsMayBody : Unspellable (Effect []) (\ok =>
  Macros.mayElse You (Macros.sacrifice You (Macros.a Macros.creature)) (Macros.exile (It {ok})))
badIfNotReadsMayBody Refl impossible


||| "Create a 1/1 black Zombie artifact token."
||| A subtype sits on one card type's own set [CR#205.1a] and every subtype
||| here is a creature type [CR#205.3m], so this one has nowhere to sit.
public export
badZombieArtifactToken : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (1, 1)) [Black] (MkTypeLine [Zombie] [Artifact])
                          [] Nothing) {sf = ok})
badZombieArtifactToken MkSubtypesFit impossible


||| "Create a white Soldier creature token." with no power or toughness
||| A token has only the characteristics its creating effect defines
||| [CR#111.3], so a creature token's P/T [CR#208.1] has to be written.
public export
badCreatureTokenNoPt : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken Nothing [White] (MkTypeLine [Soldier] [Creature]) [] Nothing) {tp = ok})
badCreatureTokenNoPt MkTokenPt impossible


||| "Create a 1/1 white token."
||| A token is a PERMANENT [CR#111.1], so its line names a card type; the
||| type-less spelling is the predefined-name catalog's [CR#111.10].
public export
badTypelessToken : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (1, 1)) [White] (MkTypeLine [] []) [] Nothing) {tt = ok})
badTypelessToken MkTokenTyped impossible


||| "Create a 1/1 red Soldier creature token that's attacking."
||| [CR#508.4] designates such a creature attacking and taps nothing, the tap
||| belonging to declare-attackers [CR#508.1f], so a writer says both words.
public export
badAttackingUntapped : Unspellable (Effect []) (\ok =>
  Create You (Lit 1) (Macros.creatureTok 1 1 [Red] [Soldier]) [EntersAttacking] {rr = ok})
badAttackingUntapped MkRidersOk impossible


||| "Put a +1/+1 counter on target creature card in your graveyard."
||| The GRAVEYARD is the counter table's measured silence: the corpus puts
||| the counter on a battlefield object, or returns the card first.
public export
badPutCountersGraveyard : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) Macros.plusOnePlusOne (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok})
badPutCountersGraveyard MkCounterHolder impossible


||| "Destroy target creature. Remove a +1/+1 counter from it."
||| The removal twin reads the same fold-state: a destroyed referent has no
||| counters to take off.
public export
badRemoveCountersDead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                RemoveCounters (Lit 1) Macros.plusOnePlusOne It {zn = ok}])
badRemoveCountersDead MkCounterHolder impossible


||| "Create a 1/1 creature artifact token."
||| A token's stated characteristics ARE its text [CR#111.3], so the bundle
||| is a surface phrase: the type words fall into one written order.
public export
badTokenTypeOrder : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (1, 1)) [] (MkTypeLine [] [Creature, Artifact])
                          [] Nothing) {tc = ok})
badTokenTypeOrder MkTokenCanonical impossible


||| "Create a 1/1 white white Soldier creature token."
||| Same surface phrase, the duplicate half: a color written twice is a word
||| written twice, and no line writes one.
public export
badTokenDuplicateColor : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (Macros.creatureTok 1 1 [White, White] [Soldier]) {tc = ok})
badTokenDuplicateColor MkTokenCanonical impossible


||| "Draw zero cards."
||| A written action count is at least one: [CR#121.1] makes a draw the
||| movement of a card, and a zero of one instructs nothing.
public export
badDrawZero : Unspellable (Effect []) (\ok =>
  Macros.drawCards 0 {wc = ok})
badDrawZero MkWrittenCount impossible


||| "Create zero 1/1 white Soldier creature tokens."
||| The same count demand: [CR#111.1] makes a token a marker put onto the
||| battlefield, and a zero of one instructs nothing.
public export
badCreateZero : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 0) (Macros.creatureTok 1 1 [White] [Soldier]) {wc = ok})
badCreateZero MkWrittenCount impossible


||| "Put zero +1/+1 counters on target creature."
||| And once more: [CR#122.1] makes a counter a marker placed on something,
||| and a zero of one instructs nothing.
public export
badPutZeroCounters : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 0) Macros.plusOnePlusOne (Macros.target Macros.creature) {wc = ok})
badPutZeroCounters MkWrittenCount impossible


||| "Target land becomes a Zombie in addition to its other types."
||| A creature subtype has nowhere to sit on a land [CR#205.1a]; the line
||| must name the card type or find it on the subject, as amass [CR#701.47a].
public export
badBecomesZombieLand : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.land) (Macros.subtypesOnly [Zombie]) Nothing {af = ok})
badBecomesZombieLand MkAddedFits impossible


||| "Target creature becomes in addition to its other types."
||| The clause has to say WHAT: an empty type line adds nothing and spells
||| no phrase.
public export
badBecomesNothing : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (MkTypeLine [] []) Nothing
                 {ne = Builtin.fst ok, nw = Builtin.snd ok})
badBecomesNothing (MkLineNonEmpty, _) impossible


||| "Target creature card in your graveyard becomes an artifact in addition to its other types."
||| A type change is a continuous effect on a permanent: a graveyard card
||| has no types for the clause to add to.
public export
badBecomesInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) (Macros.typesOnly [Artifact]) Nothing {zn = ok})
badBecomesInGraveyard MkZoneFits impossible


||| "Target creature becomes an artifact … until end of combat."
||| The combat endpoint stays the GRANTS' alone; the type addition writes
||| end-of-turn lines and no end-of-combat line at all.
public export
badBecomesUntilEndOfCombat : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.untilEndOfCombat) {sp = ok})
badBecomesUntilEndOfCombat SpanStated impossible


||| "Target creature becomes an artifact … this turn."
||| The restriction's current-turn word is not the type addition's either.
public export
badBecomesThisTurn : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.thisTurn) {sp = ok})
badBecomesThisTurn SpanStated impossible


||| "Target creature becomes an artifact … until your next upkeep."
||| The upkeep endpoint stays the keyword grant's alone, as it is against
||| the stat delta.
public export
badBecomesUntilYourNextUpkeep : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.untilYourNextUpkeep) {sp = ok})
badBecomesUntilYourNextUpkeep SpanStated impossible


||| "Zombie that isn't a creature"
||| A subtype word PRESUPPOSES its set's card type [CR#205.1a], so the
||| phrase describes nothing.
public export
badZombieNoncreature : Unspellable (Predicate [] Object) (\ok =>
  And [HasSubtype Zombie, Not Macros.creature] {cf = ok})
badZombieNoncreature MkContradictionFree impossible


||| "Target creature becomes a creature in addition to its other types."
||| The clause RETAINS what the object had and states what it gains
||| [CR#205.1b], so stating only what the subject already is states nothing.
public export
badBecomesOwnType : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Creature]) Nothing {nw = ok})
badBecomesOwnType MkAddsSomething impossible


||| "Target artifact you control becomes a creature … until your next turn."
||| No line writes a NAKED type addition across turns; the three that look
||| like it carry the duration elsewhere or fuse a second construction.
public export
badTypeAdditionAcrossTurns : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target (And [Macros.artifact, ControlledBy You]))
          (Macros.typesOnly [Creature])
          (Just Macros.untilYourNextTurn) {sp = ok})
badTypeAdditionAcrossTurns SpanStated impossible


||| "Create a 1/1 black Zombie creature token if you control a creature. Otherwise, tap it."
||| The "Otherwise" arm runs exactly when the condition was false, so the
||| clause it replaces never happened and its phrase named nothing.
public export
badOtherwiseReadsIfArm : Unspellable (Effect []) (\ok =>
  If (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [Zombie]))
     (Exists Macros.creatureYouControl)
     (Just (Tap (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok})))
badOtherwiseReadsIfArm (Refl, _) impossible


||| "Choose one — Destroy target artifact."
||| A modal offers TWO OR MORE options [CR#700.2], so a one-mode "modal" is
||| the sentence itself with a choice clause bolted on front.
public export
badModalOneMode : Unspellable (Effect []) (\ok =>
  Modal (Macros.upTo 1) [Macros.destroy (Macros.target Macros.artifact)] {tw = ok})
badModalOneMode TwoUp impossible


||| "Choose two — Destroy target artifact; or destroy target enchantment."
||| A headcount that fixes the whole list instructs no choice, and [CR#700.2]
||| calls a spell modal for the INSTRUCTIONS to choose.
public export
badModalFixedWhole : Unspellable (Effect []) (\ok =>
  Macros.chooseTwo [Macros.destroy (Macros.target Macros.artifact),
                    Macros.destroy (Macros.target Macros.enchantment)] {mf = ok})
badModalFixedWhole MkModesFit impossible


||| "Choose three — Destroy target artifact; or destroy target enchantment."
||| Nor may a headcount reach PAST the list: three of two options names
||| nothing at all.
public export
badModalOverreach : Unspellable (Effect []) (\ok =>
  Modal (Macros.exactly 3) [Macros.destroy (Macros.target Macros.artifact),
                            Macros.destroy (Macros.target Macros.enchantment)] {mf = ok})
badModalOverreach MkModesFit impossible


||| "Choose one — Destroy target artifact; or tap it."
||| The mode list is no telescope: modes are chosen at cast [CR#700.2a] and
||| an unchosen one's targets are never announced [CR#700.2c].
public export
badModalReadsAcrossModes : Unspellable (Effect []) (\ok =>
  Macros.chooseOne [Macros.destroy (Macros.target Macros.artifact),
                    Tap (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badModalReadsAcrossModes (_, OnField) impossible


||| "Choose one — Destroy target artifact; or destroy target enchantment. Tap it."
||| And nothing after the modal reads into it, for the same reason pointed
||| forward; oracle writes a description covering both outcomes instead.
public export
badReadsAfterModal : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.chooseOne [Macros.destroy (Macros.target Macros.artifact), Macros.destroy (Macros.target Macros.enchantment)],
                Tap (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badReadsAfterModal (_, OnField) impossible


||| "Destroy target artifact if its mana value isn't 2 or less."
||| WHICH condition frames take a "not" is a closed table, and the
||| comparison frame does not: a bound has a negative of its own.
public export
badNegatedComparisonCondition : Unspellable (Effect []) (\ok =>
  If (Macros.destroy (Macros.target Macros.artifact))
     (Macros.notSo (CompareAmt (Macros.manaValueOf It) OrLess (Lit 2)) {ng = ok})
     Nothing)
badNegatedComparisonCondition MkCondNegatable impossible


||| "Destroy target artifact if it isn't the case that you don't control a creature."
||| Nor does a negation take one: English collapses the pair into the
||| positive.
public export
badDoubleNegatedCondition : Unspellable (Effect []) (\ok =>
  If (Macros.destroy (Macros.target Macros.artifact))
     (Macros.notSo (Macros.notSo (Exists Macros.creatureYouControl)) {ng = ok})
     Nothing)
badDoubleNegatedCondition MkCondNegatable impossible


||| "Draw a card. Exile that card."
||| A drawn card is not a mention; the card is read only inside the
||| coordination that reveals it, which this grammar does not spell.
public export
badDrawnCardRemention : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.drawACard, Macros.exile (That CardW {ok})])
badDrawnCardRemention Refl impossible


||| "Choose up to two — Destroy target artifact; or destroy target enchantment; or draw a card."
||| The modal headcount vocabulary is CLOSED over what oracle writes: the
||| "up to" head is capped at one.
public export
badModalUpToTwo : Unspellable (Effect []) (\ok =>
  Modal (Macros.upTo 2) [Macros.destroy (Macros.target Macros.artifact),
                        Macros.destroy (Macros.target Macros.enchantment),
                        Macros.drawACard] {mh = ok})
badModalUpToTwo MkModalHead impossible


||| "Choose one — Draw a card; or draw a card."
||| Two identical modes are one mode written twice, and a player normally
||| cannot "choose the same mode more than once" [CR#700.2,700.2d].
public export
badDuplicateModes : Unspellable (Effect []) (\ok =>
  Macros.chooseOne [Macros.drawACard, Macros.drawACard] {dm = ok})
badDuplicateModes Refl impossible


||| "Choose you."
||| A choice BINDS a new referent out of a described set, so the phrase has
||| to describe one; an already definite participant selects among nothing.
public export
badChooseYou : Unspellable (Effect []) (\ok =>
  Choose You {ch = ok})
badChooseYou MkChoosable impossible


||| "Create a 1/1 white Soldier creature token if you control a creature. Put a +1/+1 counter on it."
||| A conditioned clause is a HOLE: the condition may be false, and then the
||| clause never ran and its phrase named nothing.
public export
badConditionalArmAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [If (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier]))
                   (Exists Macros.creatureYouControl)
                   Nothing,
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badConditionalArmAntecedent (_, MkCounterHolder) impossible


||| "You may gain 1 life. If you do, create a token. If you don't, create two. Put a +1/+1 counter on it."
||| With BOTH arms written the may exports its BODY and neither arm: the
||| branch turns on the choice to pay, not on what occurred [CR#118.12].
public export
badBothArmsAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [May (Just You) (Macros.gainsLife You (Lit 1))
                     (Just (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier])))
                     (Just (Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [Soldier]))),
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badBothArmsAntecedent (_, MkCounterHolder) impossible


||| "other than a creature"
||| The complement's anchor may not be an INDEFINITE: it would announce a
||| second referent the sentence never spelled.
public export
badComplementAnchorAnnounces : Unspellable (Predicate [] Object) (\ok =>
  OtherThan (Macros.a Macros.creature) {ca = ok})
badComplementAnchorAnnounces MkComplementAnchor impossible


||| "other than up to two target creatures"
||| The anchor is SINGULAR: this constructor subtracts ONE referent, and a
||| plural anchor is group subtraction, which no corpus line writes.
public export
badPluralComplementAnchor : Unspellable (Predicate [] Object) (\ok =>
  OtherThan (TargetGroup (Macros.upTo 2) Macros.creature) {ca = ok})
badPluralComplementAnchor MkComplementAnchor impossible


||| "each creature other than this land"
||| The anchor has to be something the phrase could have described; a
||| cross-head anchor subtracts nothing.
public export
badComplementCrossHead : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Each (And [Macros.creature, OtherThan Macros.thisLand] {oa = ok})))
badComplementCrossHead MkOtherAnchored impossible


||| "each creature other than this creature other than this creature"
||| One selector slot per phrase, whichever spelling fills it: two
||| complements are two "other"s.
public export
badDoubleComplement : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1)
             (Each (And [Macros.creature, OtherThan Macros.thisCreature, OtherThan Macros.thisCreature] {oa = ok})))
badDoubleComplement MkOtherAnchored impossible


||| "each other creature other than this creature"
||| The two spellings share that slot: a phrase cannot write the bare
||| "other" and an anchored one at once.
public export
badOtherAndComplement : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target Macros.creature),
                DealDamage This (Lit 1)
                           (Each (And [Macros.creature, Other, OtherThan Macros.thisCreature] {oa = ok}))])
badOtherAndComplement MkOtherAnchored impossible


||| "creature other than this creature, or land"
||| A word that fills one phrase-level slot is not an ALTERNATIVE.
public export
badComplementInOr : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.creature, OtherThan Macros.thisCreature], Macros.land] {cd = ok})
badComplementInOr MkCoordinableDisjuncts impossible


||| "not other than this creature"
||| "Non-other" is unwritten, in this spelling as in the bare one.
public export
badNegatedComplement : Unspellable (Predicate [] Object) (\ok =>
  Not (OtherThan Macros.thisCreature) {ng = ok})
badNegatedComplement MkNegatable impossible


||| an empty batch
||| A batch is at least TWO parts, and nothing at all instructs nothing.
public export
badEmptySimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously [] {ok})
badEmptySimultaneous TwoUp impossible


||| "Destroy target creature." written as a one-element batch
||| The same arity demand at one: a second spelling of the bare clause.
public export
badSingletonSimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.destroy (Macros.target Macros.creature)] {ok})
badSingletonSimultaneous TwoUp impossible


||| a batch written as one element of a batch
||| Its elements are clauses and not batches — the re-minted tree refused
||| one construction over.
public export
badNestedSimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously
    ((Simultaneously [Macros.destroy (Macros.target Macros.creature), Macros.destroy (Macros.target Macros.artifact)]
      :: (Macros.destroy (Macros.target Macros.land) :: Nil)) {ns = ok}))
badNestedSimultaneous MkNotSim impossible


||| a sequence written as one element of a batch
||| An ordered list inside an unordered one contradicts the container it
||| sits in.
public export
badSequenceInsideSimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously
    ((Sequentially [Macros.destroy (Macros.target Macros.creature), Macros.destroy (Macros.target Macros.artifact)]
      :: (Macros.destroy (Macros.target Macros.land) :: Nil)) {nq = ok}))
badSequenceInsideSimultaneous MkNotSeq impossible


||| "Exile target creature and destroy that card." as one instruction
||| A batch's elements share ONE pre-state [CR#608.2f], so an element reads
||| what a sibling ANNOUNCED and nothing a sibling DID — nothing is exiled yet.
public export
badSimultaneousReadsRetag : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.exile (Macros.target Macros.creature),
                  Macros.destroy (That CardW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badSimultaneousReadsRetag (_, OnField) impossible


||| "This deals 2 damage to target creature and you gain that much life." as one instruction
||| The event OUTCOME is the same fact for magnitudes: no damage has been
||| dealt when a sibling is typed.
public export
badSimultaneousReadsOutcome : Unspellable (Effect []) (\ok =>
  Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                  Macros.gainsLife You (ThatMuch {ok})])
badSimultaneousReadsOutcome Refl impossible


||| "You may create a token and put a +1/+1 counter on it." as one instruction
||| The offer turns on the choice to pay, not on what occurred [CR#118.12],
||| and the batch processes its actions at once [CR#608.2f].
public export
badSimultaneousReadsMayDeed : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.may You (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant])),
                  PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badSimultaneousReadsMayDeed (_, MkCounterHolder) impossible


||| "You may have this deal 2 damage and you gain that much life." as one instruction
||| The magnitude twin, reached through the same optional wrapper.
public export
badSimultaneousReadsMayOutcome : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.may You (DealDamage This (Lit 2) (Macros.target Macros.creature)),
                  Macros.gainsLife You (ThatMuch {ok})])
badSimultaneousReadsMayOutcome Refl impossible


||| "Create a Plant token and a Soldier token. Put a +1/+1 counter on it."
||| OUTWARD, a batch leaves behind EVERY element's deed [CR#608.2f], so two
||| creates leave two tokens and the sentence after cannot say "it".
public export
badBatchTwoCreatesThenIt : Unspellable (Effect []) (\ok =>
  Sequentially [Simultaneously [Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant]),
                               Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier])],
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok})])
badBatchTwoCreatesThenIt Refl impossible


||| "This deals 2 damage to target creature and target opponent loses 3 life. You gain that much life."
||| The magnitude twin outward: two outcomes in one batch, and "that much"
||| does not say which.
public export
badBatchTwoOutcomesThenThatMuch : Unspellable (Effect []) (\ok =>
  Sequentially [Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                               Macros.losesLife (Macros.target Opponent) (Lit 3)],
                Macros.gainsLife You (ThatMuch {ok})])
badBatchTwoOutcomesThenThatMuch Refl impossible


||| "Gain control of target creature card in a graveyard."
||| Control is a PERMANENT's [CR#110.2], and an object neither on the stack
||| nor on the battlefield has no controller [CR#109.4].
public export
badGainControlGraveyard : Unspellable (Effect []) (\ok =>
  Macros.gainControl (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) Nothing {zn = ok})
badGainControlGraveyard MkZoneFits impossible
