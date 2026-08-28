||| Unspellable pins, continued from Experimental.Proofs.
module Experimental.ProofsB

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off




||| "another other creature"
||| The selector order gives other/another a single slot per phrase.
public export
badDoubleOther : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing)] Object)
  (\ok => And [Macros.creature, Other, Other] {oa = ok})
badDoubleOther Oh impossible




||| "You discard it." of a referent nothing has placed
||| An antecedent recording no zone is no hand card, and [CR#701.9a] discards FROM a hand.
public export
badDiscardIt : Unspellable
  (Effect [MkBinding AD Object OneOf (ObjectP Nothing Nothing Nothing Nothing)])
  (\ok => Macros.discards You It {dk = ok})
badDiscardIt DiscardTracked impossible




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
  Sequentially [] {ne = ok})
badEmptySequence ItIsSucc impossible




||| "This deals 3 damage to any target. Destroy it."
||| The binding records the any-target phrase's own silence, so "it" inherits no battlefield.
public export
badDestroyAnyTargetRemention : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.target Macros.anyTarget),
               Macros.destroy It {ok}])
badDestroyAnyTargetRemention OnField impossible




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
  Or [] {ne = ok})
badEmptyOr IsNonEmpty impossible


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
||| The parallel demand about place: this projection names one zone for both
||| alternatives. It stands after the type half of the same demand was
||| relaxed, and the two halves part for a reason. A head word carries the
||| type it presupposes ([CR#205.3c] correlates a subtype to its own card
||| type), so alternatives that each write a head may name different types
||| and the phrase presupposes none. A head word does NOT carry a place:
||| [CR#109.2a] locates a card-worded description by the zone the phrase
||| states, and
||| `phraseZone` defaults an unstated one to the battlefield -- so a pair of
||| arms naming two zones would project none and be read onto the
||| battlefield, which is a mis-placement and not a widening. Retiring it
||| needs a union-valued `seedZone`, not a deleted gate.
||| The same refusal answers "target spell or permanent" (the Lace cycle,
||| Blind Seer, Aether Gust and kin): a spell is on the stack [CR#109.2b]
||| and a permanent on the battlefield, so that phrase is this one wearing
||| a different head. It does NOT answer "from your graveyard or from
||| exile" (Doc Aurlock): casting is history rather than a location, so
||| `CastFrom` seeds no zone and those arms are parallel already.
public export
badCrossZoneDisjunction : Unspellable (Predicate [] Object) (\ok =>
  Or [InZone Macros.handZ, InZone Macros.graveyardZ] {pd = ok})
badCrossZoneDisjunction Oh impossible


||| "spell or permanent"
||| The cross-zone refusal at the union subject the Lace cycle wants: a
||| spell is on the stack [CR#109.2b], a permanent on the battlefield, and
||| one phrase names one place [CR#109.2a]. Not a kind question -- both arms
||| are objects -- and not a headedness one; the projection is the whole of
||| what refuses it.
public export
badSpellOrPermanentSubject : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.spell, Permanent] {pd = ok})
badSpellOrPermanentSubject Oh impossible




||| "other creature or land"
||| "Other" fills one selector slot for the whole coordinated phrase, so it is no alternative.
public export
badOtherInOr : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing)] Object)
  (\ok => Or [And [Macros.creature, Other], Macros.land] {cd = ok})
badOtherInOr Oh impossible


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


||| "creature you control or creature you control"
||| The same repetition spelled with a modifier: member equality looks inside the conjunction too.
public export
badRepeatedStructuredDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.creature, ControlledBy You], And [Macros.creature, ControlledBy You]] {dd = ok})
badRepeatedStructuredDisjunct Oh impossible


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
  Continuously (Macros.deontic (Macros.target Macros.creature) Forbid ["Attack"] Patient NoDeonticPatient {dp = ok}) (Just Macros.thisTurn))
badCantBeAttacked Participant impossible


||| "Target creature card in a graveyard can't block this turn."
||| A permanent leaving the battlefield is removed from combat [CR#506.4]; a graveyard card has no deed.
public export
badCantInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) (Just Macros.thisTurn) {zn = ok})
badCantInGraveyard Oh impossible




||| "noncreature with power 2 or less"
||| Only a creature has power [CR#208.3], so bounding power while denying the type describes nothing.
public export
badNoncreaturePower : Unspellable (Predicate [] Object) (\ok =>
  And [Compare Power AtMost (Lit 2), Not Macros.creature] {cf = ok})
badNoncreaturePower Oh impossible


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
                OnlyIf (Macros.gainsLife You (Lit 1)) (Matches It (And []) {sy = ok}) Nothing])
badMatchesNothing Oh impossible


||| "Destroy target creature if it's in a graveyard."
||| A postposed condition is checked as the clause resolves, before the deed, so it reads the announced subject, which stands on the battlefield [CR#109.2].
public export
badTrailingPostStateZone : Unspellable (Effect []) (\ok =>
  OnlyIf (Macros.destroy (Macros.target Macros.creature)) (Matches It (InZone Macros.graveyardZ) {zc = ok}) Nothing)
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
  Sequentially [OnlyIf (Macros.gainsLife You (Lit 2)) (Exists Macros.creatureYouControl) Nothing,
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
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [Black] (MkTypeLine [creatureType "Zombie"] [Artifact])
                          [] Nothing) {sf = ok})
badZombieArtifactToken Oh impossible


||| "Create a white Soldier creature token." with no power or toughness
||| A token has only the characteristics its effect defines [CR#111.3]; a creature needs P/T [CR#208.1].
public export
badCreatureTokenNoPt : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken Nothing [White] (MkTypeLine [creatureType "Soldier"] [Creature]) [] Nothing) {tp = ok})
badCreatureTokenNoPt Oh impossible


||| "Create a 1/1 white token."
||| A token is a permanent [CR#111.1], so its line names a card type [CR#111.10].
public export
badTypelessToken : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [White] (MkTypeLine [] []) [] Nothing) {tt = ok})
badTypelessToken Oh impossible


||| "Destroy target creature. Remove a +1/+1 counter from it."
||| [CR#122.2]: the counters ceased to exist as the creature moved, and the graveyard card is a new object [CR#400.7].
public export
badRemoveCountersDead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                RemoveCounters (Lit 1) (Just Macros.plusOnePlusOne) It {cm = ok}])
badRemoveCountersDead Oh impossible


||| "Move a counter from target creature onto it."
||| [CR#122.5] names the same-object case among the ones that make a move impossible:
||| the counter would have to be removed from and put onto one object.
public export
badMoveCountersSelf : Unspellable (Effect []) (\ok =>
  MoveCounters (Lit 1) Nothing (Macros.target Macros.creature) It {md = ok})
badMoveCountersSelf Oh impossible


||| "Create a 1/1 creature creature token."
||| Same surface phrase: a type word written twice is a word written twice.
public export
badTokenDuplicateType : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [] (MkTypeLine [] [Creature, Creature])
                          [] Nothing) {tc = ok})
badTokenDuplicateType Oh impossible


||| "Create a 1/1 white white Soldier creature token."
||| Same surface phrase: a color written twice is a word written twice.
public export
badTokenDuplicateColor : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (Macros.creatureTok 1 1 [White, White] [creatureType "Soldier"]) {tc = ok})
badTokenDuplicateColor Oh impossible


||| "Target land becomes a Zombie in addition to its other types."
||| A creature subtype has nowhere to sit on a land [CR#205.1a]; the line must name the card type.
public export
badBecomesZombieLand : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.land) (Macros.subtypesOnly [creatureType "Zombie"]) Nothing {af = ok})
badBecomesZombieLand Oh impossible


||| "Target creature becomes in addition to its other types."
||| The clause has to say WHAT: an empty type line adds nothing and spells no phrase.
||| One gate now where two stood, and the refusal is unchanged: the
||| bundle-level `AdditionSaysSomething` admits an absent type line only
||| where the bundle writes a colour instead (Indigo Faerie), and this
||| bundle writes nothing at all.
public export
badBecomesNothing : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (MkTypeLine [] []) Nothing {sw = ok})
badBecomesNothing Oh impossible


||| "Target creature becomes a creature in addition to its other types."
||| The clause retains what the object had and states what it gains [CR#205.1b]; this states nothing.
public export
badBecomesOwnType : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Creature]) Nothing {sw = ok})
badBecomesOwnType Oh impossible


||| "Create a 1/1 black Zombie creature token if you control a creature. Otherwise, tap it."
||| The "Otherwise" arm runs when the condition was false, so the clause it replaces never happened.
public export
badOtherwiseReadsIfArm : Unspellable (Effect []) (\ok =>
  OnlyIf (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [creatureType "Zombie"]))
     (Exists Macros.creatureYouControl)
     (Just (SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok})))
badOtherwiseReadsIfArm (Refl, _) impossible


||| "Choose one — Destroy target artifact."
||| A modal offers two or more options [CR#700.2]; one mode is the sentence with a choice bolted on.
public export
badModalOneMode : Unspellable (Effect []) (\ok =>
  Modal (Macros.upTo 1) [Macros.destroy (Macros.target Macros.artifact)] {tw = ok})
badModalOneMode TwoUp impossible


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
  Choose You Nothing {ch = ok})
badChooseYou BareChoice impossible


||| "Create a 1/1 white Soldier creature token if you control a creature. Put a +1/+1 counter on it."
||| A conditioned clause is a hole: the condition may be false, and then its phrase named nothing.
public export
badConditionalArmAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [OnlyIf (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
                   (Exists Macros.creatureYouControl)
                   Nothing,
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (It {ok})])
badConditionalArmAntecedent Refl impossible


||| "You may gain 1 life. If you do, create a token. If you don't, create two. Put a +1/+1 counter on it."
||| The may exports its body and neither arm: the branch turns on the choice, not the outcome [CR#118.12].
public export
badBothArmsAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [May (Just You) (Macros.gainsLife You (Lit 1))
                     (Just (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])))
                     (Just (Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))),
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (It {ok})])
badBothArmsAntecedent Refl impossible


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
  Simultaneously [] {ne = ok})
badEmptySimultaneous ItIsSucc impossible


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
  Simultaneously [Macros.may You (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])),
                  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (It {ok})])
badSimultaneousReadsMayDeed Refl impossible


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
  Sequentially [Simultaneously [Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Plant"]),
                               Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])],
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (It {ok})])
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
