module Experimental.ProofsB

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off




||| "another other creature"
public export
badDoubleOther : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing Nothing)] Object)
  (\ok => And [Macros.creature, Other, Other] {oa = ok})
badDoubleOther Oh impossible




||| "You discard it."
public export
badDiscardIt : Unspellable
  (Effect [MkBinding AD Object OneOf (ObjectP Nothing Nothing Nothing Nothing Nothing)])
  (\ok => Macros.discard You It {dk = ok})
badDiscardIt DiscardTracked impossible




||| "a creature you control in your graveyard"
public export
badControlledInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, HasPossessor ControllerAx You, InZone Macros.graveyardZ] {zc = ok})
badControlledInGraveyard Oh impossible


||| an empty sentence list
public export
badEmptySequence : Unspellable (Effect []) (\ok =>
  Sequentially [] {ne = ok})
badEmptySequence ItIsSucc impossible




||| "This deals 3 damage to any target. Destroy it."
public export
badDestroyAnyTargetRemention : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.target Macros.anyTarget),
               Macros.destroy It {ok}])
badDestroyAnyTargetRemention Oh impossible




||| "This deals 1 damage to this spell."
public export
badDamageThis : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) This {rk = ok})
badDamageThis ObjectTakes impossible


||| a coordination of no alternatives
public export
badEmptyOr : Unspellable (Predicate [] Object) (\ok =>
  Or [] {ne = ok})
badEmptyOr IsNonEmpty impossible


||| "artifact or artifact"
public export
badRepeatedDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.artifact, Macros.artifact] {dd = ok})
badRepeatedDisjunct Oh impossible


||| "artifact or attacking"
public export
badHeadlessDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.artifact, Attacking] {pd = ok})
badHeadlessDisjunct Oh impossible


||| "in your hand or in your graveyard"
public export
badCrossZoneDisjunction : Unspellable (Predicate [] Object) (\ok =>
  Or [InZone Macros.handZ, InZone Macros.graveyardZ] {pd = ok})
badCrossZoneDisjunction Oh impossible


||| "spell or permanent"
public export
badSpellOrPermanentSubject : Unspellable (Predicate [] Object) (\ok =>
  Or [Macros.spell, Permanent] {pd = ok})
badSpellOrPermanentSubject Oh impossible




||| "other creature or land"
public export
badOtherInOr : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing Nothing)] Object)
  (\ok => Or [And [Macros.creature, Other], Macros.land] {cd = ok})
badOtherInOr Oh impossible


||| "This deals 2 damage to target artifact or enchantment."
public export
badDamageDisjunctHead : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 2) (Macros.target (Or [Macros.artifact, Macros.enchantment])) {rk = ok})
badDamageDisjunctHead ObjectTakes impossible


||| "attacking or blocking creature in your graveyard"
public export
badAttackingOrBlockingInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Or [Attacking, Blocking], InZone Macros.graveyardZ] {zc = ok})
badAttackingOrBlockingInGraveyard Oh impossible


||| "noncreature that is attacking or blocking"
public export
badNoncreatureAttackingOrBlocking : Unspellable (Predicate [] Object) (\ok =>
  And [Not Macros.creature, Or [Attacking, Blocking]] {cf = ok})
badNoncreatureAttackingOrBlocking Oh impossible


||| "noncreature that is an attacking artifact or a blocking land"
public export
badWrappedStatusLaunder : Unspellable (Predicate [] Object) (\ok =>
  And [Or [And [Macros.artifact, Attacking], And [Macros.land, Blocking]], Not Macros.creature] {cf = ok})
badWrappedStatusLaunder Oh impossible


||| "blocked creature that's unblocked"
public export
badBlockedAndUnblocked : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Blocked, Macros.unblocked] {cf = ok})
badBlockedAndUnblocked Oh impossible


||| "between three and two target creatures"
public export
badDescendingRange : Unspellable (Noun [] Object) (\ok =>
  Macros.targets (Range (Just 3) (Just 2)) Macros.creature {ok = (MaxAtLeastOne, ok, ObjectTgt)})
badDescendingRange Oh impossible


||| "attacking artifact or land"
public export
badPartialZoneJoin : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.artifact, Attacking], Macros.land] {pd = ok})
badPartialZoneJoin Oh impossible


||| "creature you control or creature you control"
public export
badRepeatedStructuredDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.creature, HasPossessor ControllerAx You], And [Macros.creature, HasPossessor ControllerAx You]] {dd = ok})
badRepeatedStructuredDisjunct Oh impossible


||| "Target land can't attack this turn."
public export
badCantAttackLand : Unspellable (Effect []) (\ok =>
  Macros.cantAttack (Macros.target Macros.land) (Just Macros.thisTurn) {dp = ok})
badCantAttackLand Oh impossible


||| "Target creature or land can't block this turn."
public export
badCantDisjunctSubject : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target (Or [Macros.creature, Macros.land])) (Just Macros.thisTurn) {dp = ok})
badCantDisjunctSubject Oh impossible


||| "Target creature can't be attacked this turn."
public export
badCantBeAttacked : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.target Macros.creature) Forbid ["Attack"] Patient NoDeonticPatient {dp = ok}) (Just Macros.thisTurn))
badCantBeAttacked Oh impossible


||| "Target creature card in a graveyard can't block this turn."
public export
badCantInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) (Just Macros.thisTurn) {dp = ok})
badCantInGraveyard Oh impossible




||| "noncreature with power 2 or less"
public export
badNoncreaturePower : Unspellable (Predicate [] Object) (\ok =>
  And [Compare [CharAxis Power] AtMost (Lit 2), Not Macros.creature] {cf = ok})
badNoncreaturePower Oh impossible


||| "with power 2 or less or with power 2 or less"
public export
badRepeatedComparisonDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Compare [CharAxis Power] AtMost (Lit 2),
      Compare [CharAxis Power] AtMost (Lit 2)] {dd = ok})
badRepeatedComparisonDisjunct Oh impossible


||| "with power 2 or less or mana value 3 or less"
public export
badMixedCharacteristicDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Compare [CharAxis Power] AtMost (Lit 2),
      Compare [CharAxis ManaValue] AtMost (Lit 3)] {pd = ok})
badMixedCharacteristicDisjunct Oh impossible




||| "if target creature is an artifact"
public export
badMatchesTargetSubject : Unspellable (Condition []) (\ok =>
  Matches (Macros.target Macros.creature) Macros.artifact {bl = ok})
badMatchesTargetSubject Oh impossible


||| "Tap target creature. You gain 1 life if it's."
public export
badMatchesNothing : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                OnlyIf (Macros.gainsLife You (Lit 1)) (Matches It (And []) {sy = ok}) Nothing])
badMatchesNothing Oh impossible


||| "Destroy target creature if it's in a graveyard."
public export
badTrailingPostStateZone : Unspellable (Effect []) (\ok =>
  OnlyIf (Macros.destroy (Macros.target Macros.creature)) (Matches It (InZone Macros.graveyardZ) {zc = ok}) Nothing)
badTrailingPostStateZone Oh impossible


||| "if 3 is 4 or greater"
public export
badCompareLiteralSubject : Unspellable (Condition []) (\ok =>
  CompareAmt (Lit 3) AtLeast (Lit 4) {rd = ok})
badCompareLiteralSubject Oh impossible




||| "You gain 2 life if you control a creature. Tap it."
public export
badConditionAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [OnlyIf (Macros.gainsLife You (Lit 2)) (Macros.exists Macros.creatureYouControl) Nothing,
                SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badConditionAntecedent (Refl, _) impossible


||| "You may sacrifice a creature. If you don't, exile it."
public export
badIfNotReadsMayBody : Unspellable (Effect []) (\ok =>
  (May You (Macros.sacrifice You (Macros.a Macros.creature)) Nothing (Just (Macros.exile You (It {ok})))))
badIfNotReadsMayBody Refl impossible


||| "Create a 1/1 black Zombie artifact token."
public export
badZombieArtifactToken : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [Black] (MkTypeLine [creatureType "Zombie"] [Artifact])
                          [] Nothing) {wf = ok})
badZombieArtifactToken (Oh, Oh, Oh, Oh, Oh, Oh) impossible


||| "Create a white Soldier creature token."
public export
badCreatureTokenNoPt : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken Nothing [White] (MkTypeLine [creatureType "Soldier"] [Creature]) [] Nothing) {wf = ok})
badCreatureTokenNoPt (Oh, Oh, Oh, Oh, Oh, Oh) impossible


||| "Create a 1/1 white token."
public export
badTypelessToken : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [White] (MkTypeLine [] []) [] Nothing) {wf = ok})
badTypelessToken (Oh, Oh, Oh, Oh, Oh, Oh) impossible


||| "Destroy target creature. Remove a +1/+1 counter from it."
public export
badRemoveCountersDead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.plusOnePlusOne)) It {cm = ok}])
badRemoveCountersDead Oh impossible


||| "Move a counter from target creature onto it."
public export
badMoveCountersSelf : Unspellable (Effect []) (\ok =>
  MoveCounters (Lit 1) Nothing (Macros.target Macros.creature) It {md = ok})
badMoveCountersSelf Oh impossible


||| "Create a 1/1 creature creature token."
public export
badTokenDuplicateType : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [] (MkTypeLine [] [Creature, Creature])
                          [] Nothing) {wf = ok})
badTokenDuplicateType (Oh, Oh, Oh, Oh, Oh, Oh) impossible


||| "Create a 1/1 white white Soldier creature token."
public export
badTokenDuplicateColor : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (Macros.creatureTok 1 1 [White, White] [creatureType "Soldier"]) {wf = ok})
badTokenDuplicateColor (Oh, Oh, Oh, Oh, Oh, Oh) impossible


||| "Target land becomes a Zombie in addition to its other types."
public export
badBecomesZombieLand : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.land) (Macros.subtypesOnly [creatureType "Zombie"]) Nothing {ok = ok})
badBecomesZombieLand Oh impossible


||| "Target creature becomes in addition to its other types."
public export
badBecomesNothing : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (MkTypeLine [] []) Nothing {ok = ok})
badBecomesNothing Oh impossible


||| "Target creature becomes a creature in addition to its other types."
public export
badBecomesOwnType : Unspellable (Effect []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Creature]) Nothing {ok = ok})
badBecomesOwnType Oh impossible


public export
badOtherwiseReadsIfArm : Unspellable (Effect []) (\ok =>
  OnlyIf (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [creatureType "Zombie"]))
     (Macros.exists Macros.creatureYouControl)
     (Just (SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok})))
badOtherwiseReadsIfArm (Refl, _) impossible


||| "Choose one — Destroy target artifact."
public export
badModalOneMode : Unspellable (Effect []) (\ok =>
  Modal (Macros.upTo 1) [Macros.destroy (Macros.target Macros.artifact)] {tw = ok})
badModalOneMode Oh impossible


||| "Choose three — Destroy target artifact; or destroy target enchantment."
public export
badModalOverreach : Unspellable (Effect []) (\ok =>
  Modal (Macros.exactly 3) [Macros.destroy (Macros.target Macros.artifact),
                            Macros.destroy (Macros.target Macros.enchantment)] {mf = ok})
badModalOverreach Oh impossible


||| "Choose one — Destroy target artifact; or tap it."
public export
badModalReadsAcrossModes : Unspellable (Effect []) (\ok =>
  Macros.chooseOne [Macros.destroy (Macros.target Macros.artifact),
                    SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badModalReadsAcrossModes (_, Oh) impossible


public export
badReadsAfterModal : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.chooseOne [Macros.destroy (Macros.target Macros.artifact), Macros.destroy (Macros.target Macros.enchantment)],
                SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badReadsAfterModal (_, Oh) impossible


||| "Draw a card. Exile that card."
public export
badDrawnCardRemention : Unspellable (Effect []) (\ok =>
  Sequentially [(Macros.draw You (Lit 1)), Macros.exile You (That CardW {ok})])
badDrawnCardRemention Refl impossible


||| "Choose two — Draw a card; draw a card." [CR#700.2d]
public export
identicalModesAllowed : Effect []
identicalModesAllowed =
  Macros.chooseTwo [(Macros.draw You (Lit 1)), (Macros.draw You (Lit 1))]


||| "Choose you."
public export
badChooseYou : Unspellable (Effect []) (\ok =>
  Choose You Nothing Openly {ch = ok})
badChooseYou BareChoice impossible


public export
badConditionalArmAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [OnlyIf (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
                   (Macros.exists Macros.creatureYouControl)
                   Nothing,
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (It {ok})])
badConditionalArmAntecedent Refl impossible


public export
badBothArmsAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [May You (Macros.gainsLife You (Lit 1))
                     (Just (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])))
                     (Just (Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))),
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (It {ok})])
badBothArmsAntecedent Refl impossible


||| "other than a creature"
public export
badComplementAnchorAnnounces : Unspellable (Predicate [] Object) (\ok =>
  OtherThan (Macros.a Macros.creature) {ca = ok})
badComplementAnchorAnnounces MkComplementAnchor impossible


||| "other than up to two target creatures"
public export
badPluralComplementAnchor : Unspellable (Predicate [] Object) (\ok =>
  OtherThan (Macros.targets (Macros.upTo 2) Macros.creature) {ca = ok})
badPluralComplementAnchor MkComplementAnchor impossible


||| "each creature other than this land"
public export
badComplementCrossHead : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.each (And [Macros.creature, OtherThan Macros.thisLand] {oa = ok})))
badComplementCrossHead Oh impossible


||| "each creature other than this creature other than this creature"
public export
badDoubleComplement : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1)
             (Macros.each (And [Macros.creature, OtherThan Macros.thisCreature, OtherThan Macros.thisCreature] {oa = ok})))
badDoubleComplement Oh impossible


||| "each other creature other than this creature"
public export
badOtherAndComplement : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                DealDamage This (Lit 1)
                           (Macros.each (And [Macros.creature, Other, OtherThan Macros.thisCreature] {oa = ok}))])
badOtherAndComplement Oh impossible


||| "creature other than this creature, or land"
public export
badComplementInOr : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.creature, OtherThan Macros.thisCreature], Macros.land] {cd = ok})
badComplementInOr Oh impossible


||| an empty batch
public export
badEmptySimultaneous : Unspellable (Effect []) (\ok =>
  Simultaneously [] {ne = ok})
badEmptySimultaneous ItIsSucc impossible


||| "Exile target creature and destroy that card."
public export
badSimultaneousReadsRetag : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.exile You (Macros.target Macros.creature),
                  Macros.destroy (That CardW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badSimultaneousReadsRetag (_, Oh) impossible


||| "This deals 2 damage to target creature and you gain that much life."
public export
badSimultaneousReadsOutcome : Unspellable (Effect []) (\ok =>
  Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                  Macros.gainsLife You (ThatMuch {ok})])
badSimultaneousReadsOutcome Refl impossible


||| "You may create a token and put a +1/+1 counter on it."
public export
badSimultaneousReadsMayDeed : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.may You (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])),
                  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (It {ok})])
badSimultaneousReadsMayDeed Refl impossible


||| "You may have this deal 2 damage and you gain that much life."
public export
badSimultaneousReadsMayOutcome : Unspellable (Effect []) (\ok =>
  Simultaneously [Macros.may You (DealDamage This (Lit 2) (Macros.target Macros.creature)),
                  Macros.gainsLife You (ThatMuch {ok})])
badSimultaneousReadsMayOutcome Refl impossible


||| "Create a Plant token and a Soldier token. Put a +1/+1 counter on it."
public export
badBatchTwoCreatesThenIt : Unspellable (Effect []) (\ok =>
  Sequentially [Simultaneously [Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Plant"]),
                               Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])],
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (It {ok})])
badBatchTwoCreatesThenIt Refl impossible


public export
badBatchTwoOutcomesThenThatMuch : Unspellable (Effect []) (\ok =>
  Sequentially [Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                               Macros.losesLife (Macros.target Opponent) (Lit 3)],
                Macros.gainsLife You (ThatMuch {ok})])
badBatchTwoOutcomesThenThatMuch Refl impossible


||| "Gain control of target creature card in a graveyard."
public export
badGainControlGraveyard : Unspellable (Effect []) (\ok =>
  Macros.gainControl You (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) Nothing {zn = ok})
badGainControlGraveyard Oh impossible
