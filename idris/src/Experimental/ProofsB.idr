||| Unspellable pins, continued from Experimental.Proofs.
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
badDoubleAnyTarget Oh impossible


||| "another other creature"
||| The selector order gives other/another a single slot per phrase.
public export
badDoubleOther : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing)] Object)
  (\ok => And [Macros.creature, Other, Other] {oa = ok})
badDoubleOther Oh impossible


||| "a creature the controller of any target controls"
||| "Any target" here sits under the non-targeting determiner a possessor phrase imposes.
public export
badAnyTargetEmbedded : Unspellable (Noun [] Object) (\ok =>
  Macros.a (And [Macros.creature, ControlledBy (ControllerOf (Macros.target AnyTarget))]) {af = ok})
badAnyTargetEmbedded Oh impossible


||| "You discard it." of a referent nothing has placed
||| An antecedent recording no zone is no hand card, and [CR#701.9a] discards FROM a hand.
public export
badDiscardIt : Unspellable
  (Effect [MkBinding AD Object OneOf (ObjectP Nothing Nothing Nothing Nothing)])
  (\ok => Macros.discards You It {dk = ok})
badDiscardIt DiscardTracked impossible


||| "You discard any target."
||| The class word heads no zone clause and takes no [CR#109.2] default, so discard has no zone.
public export
badDiscardAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.discards You (Macros.target AnyTarget) {dk = Builtin.fst ok, na = Builtin.snd ok})
badDiscardAnyTarget (_, MkNotPlayerSpanning) impossible


||| "a creature you control in your graveyard"
||| A controller relation admits only the battlefield and the stack [CR#109.4], not a graveyard.
public export
badControlledInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, ControlledBy You, InZone Macros.graveyardZ] {zc = ok})
badControlledInGraveyard Oh impossible


||| an empty sentence list
||| A sequence of no clauses is no instruction.
public export
badEmptySequence : Unspellable (Effect []) (\ok =>
  Sequentially [] {ok})
badEmptySequence TwoUp impossible


||| "Destroy target creature." written as a one-element sequence
||| A sequence of one spells what the clause alone spells, and one meaning gets one spelling.
public export
badSingletonSequence : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature)] {ok})
badSingletonSequence TwoUp impossible


||| "Destroy any target."
||| "Any target" projects no zone [CR#115.4]; destruction moves a battlefield permanent [CR#701.8a].
public export
badDestroyAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target AnyTarget) {ok = Builtin.fst ok, na = Builtin.snd ok})
badDestroyAnyTarget (OnField, _) impossible


||| "Tap any target."
||| The same silence one verb over: tapping takes a battlefield object [CR#701.26a].
public export
badTapAnyTarget : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (Macros.target AnyTarget) {ok})
badTapAnyTarget OnField impossible


||| "This deals 3 damage to any target. Destroy it."
||| The binding records the any-target phrase's own silence, so "it" inherits no battlefield.
public export
badDestroyAnyTargetRemention : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.target AnyTarget),
               Macros.destroy It {ok}])
badDestroyAnyTargetRemention OnField impossible


||| "Exile any target."
||| [CR#115.4]'s class spans players, so the phrase is ruled out before a placement sets any zone.
public export
badExileAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.exile (Macros.target AnyTarget) {na = ok})
badExileAnyTarget MkNotPlayerSpanning impossible


||| "Counter any target."
||| [CR#115.4] excludes spells from the class, and countering demands a card on the stack [CR#112.1].
public export
badCounterAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.counterSpell (Macros.target AnyTarget) {zn = ok})
badCounterAnyTarget OnTheStack impossible


||| "This deals 1 damage to this spell."
||| The recipient gate reads the NOUN, and bare `This` is no damage recipient [CR#120.1,120.1a].
public export
badDamageThis : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) This {rk = ok})
badDamageThis ObjectTakes impossible


||| a coordination of no alternatives
||| A coordination offers alternatives, so it needs two; at zero it spells nothing.
public export
badEmptyOr : Unspellable (Predicate [] Object) (\ok =>
  Or [] {tw = ok})
badEmptyOr Oh impossible


||| "creature" written as a one-alternative coordination
||| At one alternative a coordination offers none and spells what the bare alternative spells.
public export
badSingletonOr : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.creature] {tw = ok})
badSingletonOr Oh impossible


||| "artifact or artifact"
||| An alternative repeated offers a choice between a thing and itself.
public export
badRepeatedDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.artifact, Macros.artifact] {dd = ok})
badRepeatedDisjunct Oh impossible


||| "artifact or attacking"
||| Alternatives are parallel — each must stand where the others do — and a status word is no head.
public export
badHeadlessDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.artifact, Attacking] {pd = ok})
badHeadlessDisjunct Oh impossible


||| "in your hand or in your graveyard"
||| The parallel demand about place: this projection names one zone for both alternatives.
public export
badCrossZoneDisjunction : Unspellable (Predicate [] Object) (\ok =>
  Or [InZone Macros.handZ, InZone Macros.graveyardZ] {pd = ok})
badCrossZoneDisjunction Oh impossible


||| "any target or creature"
||| "Any target" is already the union [CR#115.4] fixes; coordinating it re-opens a closed class.
public export
badAnyTargetInOr : Unspellable (Predicate [] Object) (\ok =>
  Or [AnyTarget, Macros.creature] {cd = ok})
badAnyTargetInOr Oh impossible


||| "any target or creature" with the class word wrapped in a conjunction
||| Every alternative is read through `flattenPs`, so a singleton wrapper launders nothing.
public export
badAnyTargetInOrLaundered : Unspellable (Predicate [] Object) (\ok =>
  Or [And [AnyTarget], Macros.creature] {cd = ok})
badAnyTargetInOrLaundered Oh impossible


||| "other creature or land"
||| "Other" fills one selector slot for the whole coordinated phrase, so it is no alternative.
public export
badOtherInOr : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing)] Object)
  (\ok => Or [And [Macros.creature, Other], Macros.land] {cd = ok})
badOtherInOr Oh impossible


||| "(creature or land) or artifact"
||| Nesting is the flat coordination written with brackets oracle has no way to print.
public export
badNestedOr : Unspellable (Predicate [] Object) (\ok =>
  Or [Or [Macros.creature, Macros.land], Macros.artifact] {cd = ok})
badNestedOr Oh impossible


||| "This deals 2 damage to target artifact or enchantment."
||| A disjunctive head fixes no type, and damage reaches only a battle, creature, or planeswalker [CR#120.1a].
public export
badDamageDisjunctHead : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 2) (Macros.target (Or [Macros.artifact, Macros.enchantment])) {rk = ok})
badDamageDisjunctHead ObjectTakes impossible


||| "attacking or blocking creature in your graveyard"
||| Alternatives that agree on a zone still project it: the battlefield, never a graveyard.
public export
badAttackingOrBlockingInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Or [Attacking, Blocking], InZone Macros.graveyardZ] {zc = ok})
badAttackingOrBlockingInGraveyard Oh impossible


||| "noncreature that is attacking or blocking"
||| Only a creature can attack or block [CR#506.3], a presupposition both alternatives carry.
public export
badNoncreatureAttackingOrBlocking : Unspellable (Predicate [] Object) (\ok =>
  And [Not Macros.creature, Or [Attacking, Blocking]] {cf = ok})
badNoncreatureAttackingOrBlocking Oh impossible


||| "noncreature that is an attacking artifact or a blocking land"
||| A presupposition written inside an alternative still holds: both demand a creature [CR#506.3].
public export
badWrappedStatusLaunder : Unspellable (Predicate [] Object) (\ok =>
  And [Or [And [Macros.artifact, Attacking], And [Macros.land, Blocking]], Not Macros.creature] {cf = ok})
badWrappedStatusLaunder Oh impossible


||| "between three and two target creatures"
||| A range runs upward; a descending pair names an empty interval.
public export
badDescendingRange : Unspellable (Noun [] Object) (\ok =>
  TargetGroup (Range (Just 3) (Just 2)) Macros.creature {wf = ok})
badDescendingRange Oh impossible


||| "attacking artifact or land"
||| Alternatives that place their referent differently are not alternatives.
public export
badPartialZoneJoin : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.artifact, Attacking], Macros.land] {pd = ok})
badPartialZoneJoin Oh impossible


||| "zero or more target creatures"
||| A second spelling of "any number of target creatures", which already permits zero [CR#107.1c].
public export
badZeroLowerRange : Unspellable (Noun [] Object) (\ok =>
  TargetGroup (Range (Just 0) Nothing) Macros.creature {wf = ok})
badZeroLowerRange Oh impossible


||| "creature you control or creature you control"
||| The same repetition spelled with a modifier: member equality looks inside the conjunction too.
public export
badRepeatedStructuredDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.creature, ControlledBy You], And [Macros.creature, ControlledBy You]] {dd = ok})
badRepeatedStructuredDisjunct Oh impossible


||| a sequence written as one element of a sequence
||| A sequence's elements are clauses; nesting re-mints the tree the n-ary list replaced.
public export
badNestedSequence : Unspellable (Effect []) (\ok =>
  Sequentially
    ((Sequentially [Macros.destroy (Macros.target Macros.creature), Macros.exile (Macros.target Macros.creature)]
      :: (Macros.destroy (Macros.target Macros.land) :: Nil)) {ns = ok}))
badNestedSequence Oh impossible


||| "Target land can't attack this turn."
||| Only a creature can attack or block [CR#506.3], so a land has no grant for the deed to remove.
public export
badCantAttackLand : Unspellable (Effect []) (\ok =>
  Macros.cantAttack (Macros.target Macros.land) (Just Macros.thisTurn) {dp = ok})
badCantAttackLand Participant impossible


||| "Target creature or land can't block this turn."
||| A coordinated head fixes no type, and an untyped head cannot prove participation.
public export
badCantDisjunctSubject : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target (Or [Macros.creature, Macros.land])) (Just Macros.thisTurn) {dp = ok})
badCantDisjunctSubject Participant impossible


||| "Target creature can't be attacked this turn."
||| Only a player, a planeswalker, or a battle can be attacked [CR#506.3].
public export
badCantBeAttacked : Unspellable (Effect []) (\ok =>
  Continuously (Deontic (Macros.target Macros.creature) Forbid Attack Patient NoDeonticPatient {dp = ok}) (Just Macros.thisTurn))
badCantBeAttacked Participant impossible


||| "Target creature card in a graveyard can't block this turn."
||| A permanent leaving the battlefield is removed from combat [CR#506.4]; a graveyard card has no deed.
public export
badCantInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) (Just Macros.thisTurn) {zn = ok})
badCantInGraveyard Oh impossible


||| "Any target can't block this turn."
||| The class word names [CR#115.4]'s damage class and fixes no type for the deed's head demand.
public export
badCantAnyTarget : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target AnyTarget) (Just Macros.thisTurn) {dp = ok})
badCantAnyTarget Participant impossible


||| "noncreature with power 2 or less"
||| Only a creature has power [CR#208.3], so bounding power while denying the type describes nothing.
public export
badNoncreaturePower : Unspellable (Predicate [] Object) (\ok =>
  And [Compare Power AtMost (Lit 2), Not Macros.creature] {cf = ok})
badNoncreaturePower Oh impossible


||| "creature with power 2 or less and power 4 or greater"
||| One phrase, one bound: an interval is its own construction, not two stacked qualifiers.
public export
badDoubleComparison : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Compare Power AtMost (Lit 2),
       Compare Power AtLeast (Lit 4)] {lc = ok})
badDoubleComparison Oh impossible


||| "with power 2 or less or with power 2 or less"
||| An alternative repeated word for word is no alternative.
public export
badRepeatedComparisonDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Compare Power AtMost (Lit 2),
      Compare Power AtMost (Lit 2)] {dd = ok})
badRepeatedComparisonDisjunct Oh impossible


||| "with power 2 or less or mana value 3 or less"
||| Alternatives must presuppose alike: a power bound demands a creature, a mana value bound none.
public export
badMixedCharacteristicDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Compare Power AtMost (Lit 2),
      Compare ManaValue AtMost (Lit 3)] {pd = ok})
badMixedCharacteristicDisjunct Oh impossible


||| "any target with power 2 or less"
||| [CR#115.4] fixes the class by rule, so the class word takes no qualifier but "other".
public export
badAnyTargetComparison : Unspellable (Predicate [] Object) (\ok =>
  And [AnyTarget, Compare Power AtMost (Lit 2)] {at = ok})
badAnyTargetComparison Oh impossible


||| "if you control any target"
||| The class word names [CR#115.4]'s damage class and describes no object for an existential.
public export
badExistsAnyTarget : Unspellable (Condition []) (\ok =>
  Exists AnyTarget {af = ok})
badExistsAnyTarget Oh impossible


||| "Destroy target artifact if it's any target."
||| The class word fixes its class by rule [CR#115.4] rather than describing the referent.
public export
badMatchesAnyTarget : Unspellable (Effect []) (\ok =>
  If (Macros.destroy (Macros.target Macros.artifact)) (Macros.itsA AnyTarget {af = ok}) Nothing)
badMatchesAnyTarget Oh impossible


||| "if target creature is an artifact"
||| The condition's subject is a read, never a mention, so "if target …" is unrepresentable here.
public export
badMatchesTargetSubject : Unspellable (Condition []) (\ok =>
  Matches (Macros.target Macros.creature) Macros.artifact {bl = ok})
badMatchesTargetSubject Refl impossible


||| "Tap target creature. You gain 1 life if it's."
||| A description that says nothing tests nothing.
public export
badMatchesNothing : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                If (Macros.gainsLife You (Lit 1)) (Matches It (And []) {sy = ok}) Nothing])
badMatchesNothing Oh impossible


||| "Destroy target creature if it's in a graveyard."
||| A trailing condition reads the announced subject, which stands on the battlefield [CR#109.2a].
public export
badTrailingPostStateZone : Unspellable (Effect []) (\ok =>
  If (Macros.destroy (Macros.target Macros.creature)) (Matches It (InZone Macros.graveyardZ) {zc = ok}) Nothing)
badTrailingPostStateZone Oh impossible


||| "if 3 is 4 or greater"
||| A comparison measures a read against a written value; a numeral states arithmetic, not game state.
public export
badCompareLiteralSubject : Unspellable (Condition []) (\ok =>
  CompareAmt (Lit 3) AtLeast (Lit 4) {rd = ok})
badCompareLiteralSubject Oh impossible




||| "You gain 2 life if you control a creature. Tap it."
||| A condition introduces nothing: it may have been false, leaving no referent to speak of.
public export
badConditionAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [If (Macros.gainsLife You (Lit 2)) (Exists Macros.creatureYouControl) Nothing,
                SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badConditionAntecedent (Refl, _) impossible


||| "You may sacrifice a creature. If you don't, exile it."
||| The if-you-don't arm runs exactly when the body did not, so the body's phrase named nothing.
public export
badIfNotReadsMayBody : Unspellable (Effect []) (\ok =>
  Macros.mayElse You (Macros.sacrifice You (Macros.a Macros.creature)) (Macros.exile (It {ok})))
badIfNotReadsMayBody Refl impossible


||| "Create a 1/1 black Zombie artifact token."
||| A subtype sits on one card type's own set [CR#205.1a], and this one is a creature type [CR#205.3m].
public export
badZombieArtifactToken : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1, Lit 1)) [Black] (MkTypeLine [Zombie] [Artifact])
                          [] Nothing) {sf = ok})
badZombieArtifactToken Oh impossible


||| "Create a white Soldier creature token." with no power or toughness
||| A token has only the characteristics its effect defines [CR#111.3]; a creature needs P/T [CR#208.1].
public export
badCreatureTokenNoPt : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken Nothing [White] (MkTypeLine [Soldier] [Creature]) [] Nothing) {tp = ok})
badCreatureTokenNoPt Oh impossible


||| "Create a 1/1 white token."
||| A token is a permanent [CR#111.1], so its line names a card type [CR#111.10].
public export
badTypelessToken : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1, Lit 1)) [White] (MkTypeLine [] []) [] Nothing) {tt = ok})
badTypelessToken Oh impossible


||| "Create a 1/1 red Soldier creature token that's attacking."
||| [CR#508.4] designates it attacking and taps nothing; the tap is declare-attackers' [CR#508.1f].
public export
badAttackingUntapped : Unspellable (Effect []) (\ok =>
  Create You (Lit 1) (TokenWritten (Macros.creatureTok 1 1 [Red] [Soldier]))
             [EntersAttacking] {rr = ok})
badAttackingUntapped Oh impossible


||| "Put a +1/+1 counter on target creature card in your graveyard."
||| The counter row's zones are battlefield objects; a graveyard card is not one.
public export
badPutCountersGraveyard : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) Macros.plusOnePlusOne (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok})
badPutCountersGraveyard Oh impossible


||| "Destroy target creature. Remove a +1/+1 counter from it."
||| The removal twin reads the same fold-state: a destroyed referent has no counters to take off.
public export
badRemoveCountersDead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                RemoveCounters (Lit 1) Macros.plusOnePlusOne It {zn = ok}])
badRemoveCountersDead Oh impossible


||| "Create a 1/1 creature creature token."
||| Same surface phrase: a type word written twice is a word written twice.
public export
badTokenDuplicateType : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1, Lit 1)) [] (MkTypeLine [] [Creature, Creature])
                          [] Nothing) {tc = ok})
badTokenDuplicateType Oh impossible


||| "Create a 1/1 white white Soldier creature token."
||| Same surface phrase: a color written twice is a word written twice.
public export
badTokenDuplicateColor : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (Macros.creatureTok 1 1 [White, White] [Soldier]) {tc = ok})
badTokenDuplicateColor Oh impossible


||| "Draw zero cards."
||| A written action count is at least one: a draw moves a card [CR#121.1], and zero instructs nothing.
public export
badDrawZero : Unspellable (Effect []) (\ok =>
  Macros.drawCards 0 {wc = ok})
badDrawZero Oh impossible


||| "Create zero 1/1 white Soldier creature tokens."
||| A token is a marker put onto the battlefield [CR#111.1], and a zero of one instructs nothing.
public export
badCreateZero : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 0) (Macros.creatureTok 1 1 [White] [Soldier]) {wc = ok})
badCreateZero Oh impossible


||| "Put zero +1/+1 counters on target creature."
||| A counter is a marker placed on something [CR#122.1], and a zero of one instructs nothing.
public export
badPutZeroCounters : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 0) Macros.plusOnePlusOne (Macros.target Macros.creature) {wc = ok})
badPutZeroCounters Oh impossible


||| "Target land becomes a Zombie in addition to its other types."
||| A creature subtype has nowhere to sit on a land [CR#205.1a]; the line must name the card type.
public export
badBecomesZombieLand : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.land) (Macros.subtypesOnly [Zombie]) Nothing {af = ok})
badBecomesZombieLand Oh impossible


||| "Target creature becomes in addition to its other types."
||| The clause has to say WHAT: an empty type line adds nothing and spells no phrase.
public export
badBecomesNothing : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (MkTypeLine [] []) Nothing
                 {ne = Builtin.fst ok, nw = Builtin.snd ok})
badBecomesNothing (Oh, _) impossible


||| "Target creature becomes a creature in addition to its other types."
||| The clause retains what the object had and states what it gains [CR#205.1b]; this states nothing.
public export
badBecomesOwnType : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Creature]) Nothing {nw = ok})
badBecomesOwnType Oh impossible


||| "Create a 1/1 black Zombie creature token if you control a creature. Otherwise, tap it."
||| The "Otherwise" arm runs when the condition was false, so the clause it replaces never happened.
public export
badOtherwiseReadsIfArm : Unspellable (Effect []) (\ok =>
  If (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [Zombie]))
     (Exists Macros.creatureYouControl)
     (Just (SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok})))
badOtherwiseReadsIfArm (Refl, _) impossible


||| "Choose one — Destroy target artifact."
||| A modal offers two or more options [CR#700.2]; one mode is the sentence with a choice bolted on.
public export
badModalOneMode : Unspellable (Effect []) (\ok =>
  Modal (Macros.upTo 1) [Macros.destroy (Macros.target Macros.artifact)] {tw = ok})
badModalOneMode TwoUp impossible


||| "Choose two — Destroy target artifact; or destroy target enchantment."
||| A headcount that fixes the whole list instructs no choice, and [CR#700.2] requires choosing.
public export
badModalFixedWhole : Unspellable (Effect []) (\ok =>
  Macros.chooseTwo [Macros.destroy (Macros.target Macros.artifact),
                    Macros.destroy (Macros.target Macros.enchantment)] {mf = ok})
badModalFixedWhole Oh impossible


||| "Choose three — Destroy target artifact; or destroy target enchantment."
||| Nor may a headcount reach PAST the list: three of two options names nothing at all.
public export
badModalOverreach : Unspellable (Effect []) (\ok =>
  Modal (Macros.exactly 3) [Macros.destroy (Macros.target Macros.artifact),
                            Macros.destroy (Macros.target Macros.enchantment)] {mf = ok})
badModalOverreach Oh impossible


||| "Choose one — Destroy target artifact; or tap it."
||| Modes are chosen at cast [CR#700.2a] and an unchosen mode announces no targets [CR#700.2c].
public export
badModalReadsAcrossModes : Unspellable (Effect []) (\ok =>
  Macros.chooseOne [Macros.destroy (Macros.target Macros.artifact),
                    SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badModalReadsAcrossModes (_, OnField) impossible


||| "Choose one — Destroy target artifact; or destroy target enchantment. Tap it."
||| A clause after the modal reads into a mode that may not have been chosen [CR#700.2c].
public export
badReadsAfterModal : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.chooseOne [Macros.destroy (Macros.target Macros.artifact), Macros.destroy (Macros.target Macros.enchantment)],
                SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badReadsAfterModal (_, OnField) impossible


||| "Draw a card. Exile that card."
||| A drawn card is not a mention, so nothing later can read it.
public export
badDrawnCardRemention : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.drawACard, Macros.exile (That CardW {ok})])
badDrawnCardRemention Refl impossible


||| "Choose up to two — Destroy target artifact; or destroy target enchantment; or draw a card."
||| The "up to" modal headcount is capped at one in this vocabulary.
public export
badModalUpToTwo : Unspellable (Effect []) (\ok =>
  Modal (Macros.upTo 2) [Macros.destroy (Macros.target Macros.artifact),
                        Macros.destroy (Macros.target Macros.enchantment),
                        Macros.drawACard] {mh = ok})
badModalUpToTwo Oh impossible


||| "Choose one — Draw a card; or draw a card."
||| A player normally cannot choose the same mode more than once [CR#700.2,700.2d].
public export
badDuplicateModes : Unspellable (Effect []) (\ok =>
  Macros.chooseOne [Macros.drawACard, Macros.drawACard] {dm = ok})
badDuplicateModes Refl impossible


||| "Choose you."
||| A choice binds a new referent out of a described set; a definite participant describes none.
public export
badChooseYou : Unspellable (Effect []) (\ok =>
  Choose You {ch = ok})
badChooseYou BareChoice impossible


||| "Create a 1/1 white Soldier creature token if you control a creature. Put a +1/+1 counter on it."
||| A conditioned clause is a hole: the condition may be false, and then its phrase named nothing.
public export
badConditionalArmAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [If (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier]))
                   (Exists Macros.creatureYouControl)
                   Nothing,
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badConditionalArmAntecedent (_, Oh) impossible


||| "You may gain 1 life. If you do, create a token. If you don't, create two. Put a +1/+1 counter on it."
||| The may exports its body and neither arm: the branch turns on the choice, not the outcome [CR#118.12].
public export
badBothArmsAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [May (Just You) (Macros.gainsLife You (Lit 1))
                     (Just (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier])))
                     (Just (Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [Soldier]))),
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badBothArmsAntecedent (_, Oh) impossible


||| "other than a creature"
||| The complement's anchor may not be indefinite: it would announce an unspelled second referent.
public export
badComplementAnchorAnnounces : Unspellable (Predicate [] Object) (\ok =>
  OtherThan (Macros.a Macros.creature) {ca = ok})
badComplementAnchorAnnounces MkComplementAnchor impossible


||| "other than up to two target creatures"
||| The anchor is singular; this constructor subtracts one referent, not a group.
public export
badPluralComplementAnchor : Unspellable (Predicate [] Object) (\ok =>
  OtherThan (TargetGroup (Macros.upTo 2) Macros.creature) {ca = ok})
badPluralComplementAnchor MkComplementAnchor impossible


||| "each creature other than this land"
||| The anchor must be something the phrase could have described; a cross-head anchor subtracts nothing.
public export
badComplementCrossHead : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Each (And [Macros.creature, OtherThan Macros.thisLand] {oa = ok})))
badComplementCrossHead Oh impossible


||| "each creature other than this creature other than this creature"
||| One selector slot per phrase, whichever spelling fills it: two complements are two "other"s.
public export
badDoubleComplement : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1)
             (Each (And [Macros.creature, OtherThan Macros.thisCreature, OtherThan Macros.thisCreature] {oa = ok})))
badDoubleComplement Oh impossible


||| "each other creature other than this creature"
||| The two spellings share one selector slot, so a phrase writes bare "other" or an anchored one.
public export
badOtherAndComplement : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                DealDamage This (Lit 1)
                           (Each (And [Macros.creature, Other, OtherThan Macros.thisCreature] {oa = ok}))])
badOtherAndComplement Oh impossible


||| "creature other than this creature, or land"
||| A word that fills one phrase-level slot is not an ALTERNATIVE.
public export
badComplementInOr : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.creature, OtherThan Macros.thisCreature], Macros.land] {cd = ok})
badComplementInOr Oh impossible


||| an empty batch
||| A batch is at least two parts, and nothing at all instructs nothing.
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
||| A batch's elements are clauses, not batches.
public export
badNestedSimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously
    ((Simultaneously [Macros.destroy (Macros.target Macros.creature), Macros.destroy (Macros.target Macros.artifact)]
      :: (Macros.destroy (Macros.target Macros.land) :: Nil)) {ns = ok}))
badNestedSimultaneous Oh impossible


||| a sequence written as one element of a batch
||| An ordered list inside an unordered one contradicts its container.
public export
badSequenceInsideSimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously
    ((Sequentially [Macros.destroy (Macros.target Macros.creature), Macros.destroy (Macros.target Macros.artifact)]
      :: (Macros.destroy (Macros.target Macros.land) :: Nil)) {nq = ok}))
badSequenceInsideSimultaneous Oh impossible


||| "Exile target creature and destroy that card." as one instruction
||| A batch's elements share one pre-state [CR#608.2f], so an element reads no sibling's deed.
public export
badSimultaneousReadsRetag : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.exile (Macros.target Macros.creature),
                  Macros.destroy (That CardW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badSimultaneousReadsRetag (_, OnField) impossible


||| "This deals 2 damage to target creature and you gain that much life." as one instruction
||| The same pre-state holds for magnitudes: no damage is dealt when a sibling is typed [CR#608.2f].
public export
badSimultaneousReadsOutcome : Unspellable (Effect []) (\ok =>
  Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                  Macros.gainsLife You (ThatMuch {ok})])
badSimultaneousReadsOutcome Refl impossible


||| "You may create a token and put a +1/+1 counter on it." as one instruction
||| The offer turns on the choice to pay [CR#118.12], and a batch acts at once [CR#608.2f].
public export
badSimultaneousReadsMayDeed : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.may You (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant])),
                  PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok = Builtin.fst ok}) {zn = Builtin.snd ok}])
badSimultaneousReadsMayDeed (_, Oh) impossible


||| "You may have this deal 2 damage and you gain that much life." as one instruction
||| The magnitude twin, reached through the same optional wrapper.
public export
badSimultaneousReadsMayOutcome : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.may You (DealDamage This (Lit 2) (Macros.target Macros.creature)),
                  Macros.gainsLife You (ThatMuch {ok})])
badSimultaneousReadsMayOutcome Refl impossible


||| "Create a Plant token and a Soldier token. Put a +1/+1 counter on it."
||| A batch leaves every element's deed behind [CR#608.2f], so two creates leave no single "it".
public export
badBatchTwoCreatesThenIt : Unspellable (Effect []) (\ok =>
  Sequentially [Simultaneously [Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [Plant]),
                               Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier])],
                PutCounters (Lit 1) Macros.plusOnePlusOne (It {ok})])
badBatchTwoCreatesThenIt Refl impossible


||| "This deals 2 damage to target creature and target opponent loses 3 life. You gain that much life."
||| The magnitude twin outward: two outcomes in one batch, and "that much" does not say which.
public export
badBatchTwoOutcomesThenThatMuch : Unspellable (Effect []) (\ok =>
  Sequentially [Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                               Macros.losesLife (Macros.target Opponent) (Lit 3)],
                Macros.gainsLife You (ThatMuch {ok})])
badBatchTwoOutcomesThenThatMuch Refl impossible


||| "Gain control of target creature card in a graveyard."
||| An object neither on the stack nor on the battlefield has no controller [CR#109.4].
public export
badGainControlGraveyard : Unspellable (Effect []) (\ok =>
  Macros.gainControl (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) Nothing {zn = ok})
badGainControlGraveyard Oh impossible
