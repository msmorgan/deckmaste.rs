module Experimental.Proofs

import Experimental
import Experimental.Macros
import public Experimental.Unspellable

%default total

%unbound_implicits off


||| "... is equal to the chosen number", two number choices standing (Shapeshifter prints "last")
public export
badChosenNumberTwoStanding :
  Unspellable (Amount [qualityB Number, qualityB Number]) (\ok => ChosenNumber {ok})
badChosenNumberTwoStanding Refl impossible


||| "of the chosen number"
public export
badChosenNumberRead :
  Unspellable
    (Predicate [MkBinding AD (Quality Number) OneOf QualityP] Object)
    (\ok => OfChosen Number {read = ok})
badChosenNumberRead Oh impossible


||| "Choose two target creatures. You gain life equal to their power."
public export
badGroupPower : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (Macros.targets (Macros.exactly 2) Macros.creature) Nothing Openly,
                Macros.gainsLife You (Macros.powerOf Them {one = ok})])
badGroupPower Refl impossible


||| "Choose two target creatures. Their owner loses 1 life."
public export
badGroupOwner : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (Macros.targets (Macros.exactly 2) Macros.creature) Nothing Openly,
                Macros.losesLife (Macros.ownerOf Them {one = ok}) (Lit 1)])
badGroupOwner Refl impossible


||| "Two target creatures fight target creature."
public export
badFightGroup : Unspellable (Effect []) (\ok =>
  Fights (Macros.targets (Macros.exactly 2) Macros.creature) {pa = ok}
         (Macros.target Macros.creature))
badFightGroup Refl impossible


||| "a creature two target opponents control"
public export
badControlledByGroup : Unspellable (Predicate [] Object) (\ok =>
  HasPossessor ControllerAx (Macros.targets (Macros.exactly 2) Opponent) {ps = ok})
badControlledByGroup Oh impossible


||| "each of target creature"
public export
badEachOfSingular : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.target Macros.creature) {pl = ok})
badEachOfSingular Refl impossible


||| "Look at the top card of two target players' library."
public export
badSliceOfCountedPossessor : Unspellable (Effect []) (\ok =>
  Macros.lookAt (LibrarySlice OnTop (Lit 1)
                              (Macros.targets (Macros.exactly 2) AnyPlayer) {sp = ok}))
badSliceOfCountedPossessor Oh impossible


||| "Whenever you cast all spells, draw a card."
public export
badCastsPluralComplement : Unspellable (Ability) (\ok =>
  Triggered Whenever (Casts You (Macros.allOf Macros.spell) Nothing {one = ok}) [] Nothing [] Nothing Nothing Nothing
            (Macros.draw You (Lit 1)))
badCastsPluralComplement Refl impossible


public export
badDisjunctAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target (Or [And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent],
                                        And [Macros.land, HasPossessor ControllerAx You]])),
                Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badDisjunctAntecedent Refl impossible


||| "Tap target creature with flying that doesn't have flying."
public export
badKeywordContradiction : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (Macros.target (And [Macros.creature, HasKeyword (TheKeyword "Flying"),
                            Not (HasKeyword (TheKeyword "Flying"))] {cf = ok})))
badKeywordContradiction Oh impossible


||| "Tap target nonland Forest."
public export
badForestNonland : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (Macros.target (And [HasSubtype (landType "Forest"), Not Macros.land] {cf = ok})))
badForestNonland Oh impossible


||| "Snow Snow Land — Forest"
public export
badDuplicateSnow : Unspellable Card (\ok =>
  Macros.card "" Nothing [Snow, Snow] (MkTypeLine [landType "Forest"] [Land]) [] Nothing {sp = ok})
badDuplicateSnow Oh impossible


||| "This deals 1 damage to any other target."
public export
badOther : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.target (Macros.anyOtherTarget {ok})))
badOther Refl impossible


||| "Target creature fights target creature. Tap it."
public export
badIt : Unspellable (Effect []) (\ok =>
  Sequentially [Fights (Macros.target Macros.creature) (Macros.target Macros.creature),
                SetStatus Tapped (It {ok})])
badIt Refl impossible


||| "This deals 3 damage to each creature. Tap it."
public export
badTheyIt : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.each Macros.creature),
                SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badTheyIt (Refl, _) impossible


||| "Destroy target creature. At the beginning of the end step, sacrifice it."
public export
badStale : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
               Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (Macros.sacrifice You It {ok})])
badStale Oh impossible


public export
badDelayedOther : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 2) (Macros.target Macros.anyTarget),
               Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (DealDamage This (Lit 1) (Macros.target (Macros.anyOtherTarget {ok})))])
badDelayedOther Refl impossible


public export
badStaleCarrier : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile You (Macros.target Macros.creatureYouControl),
               Move (That (TypeW Creature) {ok}) Macros.battlefieldZ []])
badStaleCarrier Refl impossible


||| "Return a creature to its owner's hand: Tap it."
public export
badHiddenCost : Unspellable Ability (\ok =>
  Activated (Do (Move (Macros.a Macros.creature) Macros.handZ []))
            (SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}) Nothing Nothing Nothing Nothing)
badHiddenCost (Refl, _) impossible


||| "Discard a card, Sacrifice a creature: Exile it."
public export
badTwoCostMentions : Unspellable Ability (\ok =>
  Activated (Compound [Do ((Macros.discard You (Macros.a (InZone Macros.handZ)))),
                       Do (Macros.sacrifice You (Macros.a Macros.creature))])
            (Macros.exile You (It {ok})) Nothing Nothing Nothing Nothing)
badTwoCostMentions Refl impossible


||| "Exile target creature. Sacrifice it."
public export
badSacrificeExiled : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile You (Macros.target Macros.creature),
               Macros.sacrifice You It {ok}])
badSacrificeExiled Oh impossible


public export
badDeadCreatureRead : Unspellable (Effect []) (\ok =>
  Delayed (Dies (Macros.target Macros.creature)) [] (Just ThisTurn)
          (Move (That (TypeW Creature) {ok}) Macros.battlefieldZ []))
badDeadCreatureRead Refl impossible


||| "of the chosen creature type"
public export
badChosenWrongSort : Unspellable
  (Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object)
  (\ok => OfChosen (SubtypeQ Creature) {ok})
badChosenWrongSort Refl impossible

||| "a counter of that kind"
public export
badChosenCounterKindRead : Unspellable
  (Predicate [MkBinding AD (Quality CounterKindQ) OneOf QualityP] Object)
  (\read => OfChosen CounterKindQ {read})
badChosenCounterKindRead Oh impossible


||| "Choose two target creatures. Choose two target creatures. Tap them."
public export
badThemAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (Macros.targets (Macros.exactly 2) Macros.creature) Nothing Openly,
               Choose (Macros.targets (Macros.exactly 2) Macros.creature) Nothing Openly,
               SetStatus Tapped (Them {ok})])
badThemAmbig Refl impossible


public export
badInnerAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [Fights (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent]))
                       (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent])),
               Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badInnerAmbig Refl impossible


||| "Discard a card: Return the sacrificed card to the battlefield."
public export
badVerbedWrongVerb : Unspellable Ability (\ok =>
  Activated (Do ((Macros.discard You (Macros.a (InZone Macros.handZ)))))
            (Move (TheVerbed "Sacrifice" CardW Attributive {ok}) Macros.battlefieldZ []) Nothing Nothing Nothing Nothing)
badVerbedWrongVerb Refl impossible


||| "Sacrifice an artifact: Return the sacrificed creature to the battlefield."
public export
badVerbedWrongNoun : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice You (Macros.a (HasType Artifact))))
            (Move (TheVerbed "Sacrifice" (TypeW Creature) Attributive {ok}) Macros.battlefieldZ []) Nothing Nothing Nothing Nothing)
badVerbedWrongNoun Refl impossible


public export
badVerbedAmbig : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.sacrifice You (Macros.a Macros.creature)),
                       Do (Macros.sacrifice You (Macros.a Macros.creature))])
            (Move (TheVerbed "Sacrifice" CardW Attributive {ok}) Macros.battlefieldZ []) Nothing Nothing Nothing Nothing)
badVerbedAmbig Refl impossible


public export
badBareCardRead : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice You (Macros.a Macros.creature)))
            (Sequentially [Macros.exile You (Macros.target Macros.creature),
                           Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (Move (That CardW {ok}) Macros.battlefieldZ [])]) Nothing Nothing Nothing Nothing)
badBareCardRead Refl impossible


||| "Tap target creature card in your graveyard."
public export
badTapGraveyard : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badTapGraveyard Oh impossible


||| "Destroy target tapped creature card in your graveyard."
public export
badTappedGraveyard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped,
                                       InZone (Macros.graveyardOf You)] {zc = ok})))
badTappedGraveyard Oh impossible


||| "Destroy target tapped untapped creature."
public export
badTappedUntapped : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped, Macros.untapped] {cf = ok})))
badTappedUntapped Oh impossible


||| "Untap target creature card in your graveyard."
public export
badUntapGraveyard : Unspellable (Effect []) (\ok =>
  SetStatus Untapped (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badUntapGraveyard Oh impossible


||| "Destroy target permanent instant."
public export
badPermanentInstant : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Permanent, HasType Instant] {cf = ok})))
badPermanentInstant Oh impossible


public export
badThatPermanentDeparted : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Permanent),
               SetStatus Tapped (That PermanentW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badThatPermanentDeparted (Refl, _) impossible


||| "Destroy target token card in your graveyard."
public export
badTokenGraveyard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [IsToken, InZone (Macros.graveyardOf You)] {zc = ok})))
badTokenGraveyard Oh impossible


||| "Destroy target nontoken token."
public export
badNontokenToken : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [IsToken, Macros.nontoken] {cf = ok})))
badNontokenToken Oh impossible


||| "Tap target creature. Untap that token."
public export
badThatTokenOfCard : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
               SetStatus Untapped (That TokenW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badThatTokenOfCard (Refl, _) impossible


public export
badUntapLockGraveyard : Unspellable Ability (\ok =>
  Static (Macros.doesntUntap (Macros.a (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                             (Just You) {dp = ok}))
badUntapLockGraveyard Oh impossible


||| "Target creature card in your graveyard fights target creature."
public export
badFightGraveyard : Unspellable (Effect []) (\ok =>
  Fights (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {za = ok}
         (Macros.target Macros.creature))
badFightGraveyard Oh impossible


||| "Target land fights target creature you don't control."
public export
badFightLand : Unspellable (Effect []) (\ok =>
  Fights (Macros.target (HasType Land)) {ta = ok} (Macros.target Macros.creatureYouDontControl))
badFightLand Oh impossible


||| "Target permanent fights target creature." [CR#701.14a]
public export
badFightPermanent : Unspellable (Effect []) (\ok =>
  Fights (Macros.target Permanent) {ta = ok} (Macros.target Macros.creature))
badFightPermanent Oh impossible


public export
badDiesInGraveyard : Unspellable (Effect []) (\ok =>
  Delayed (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok}) [] (Just ThisTurn)
          (Move (That CardW) Macros.battlefieldZ []))
badDiesInGraveyard Oh impossible


||| "Destroy target creature. This deals 3 damage to it."
public export
badDamageGraveyardCard : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
               DealDamage This (Lit 3) It {rk = ok}])
badDamageGraveyardCard ObjectTakes impossible


||| "This deals 1 damage to a color."
public export
badDamageToColor : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.a (QualityNoun Color Nothing)) {rk = ok})
badDamageToColor ObjectTakes impossible


||| "This deals 1 damage to target artifact."
public export
badDamageArtifact : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.target (HasType Artifact)) {rk = ok})
badDamageArtifact ObjectTakes impossible


||| "noncolor"
public export
badNegatedQualityHead : Unspellable (Predicate [] (Quality Color)) (\ok =>
  Not (QualityNoun Color Nothing) {ng = ok})
badNegatedQualityHead Oh impossible


||| "non-player"
public export
badNegatedPlayerHead : Unspellable (Predicate [] Player) (\ok =>
  Not AnyPlayer {ng = ok})
badNegatedPlayerHead Oh impossible


||| "Tap target creature an opponent doesn't control. That player loses 1 life."
public export
badNegatedAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target (And [Macros.creature, Not (HasPossessor ControllerAx Macros.anOpponent)])),
                Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badNegatedAntecedent Refl impossible


||| "Destroy target creature on the battlefield in a graveyard."
public export
badConflictingZones : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone Macros.battlefieldZ, InZone Macros.graveyardZ] {zc = ok})))
badConflictingZones Oh impossible


||| "Choose target color."
public export
badTargetColor : Unspellable (Effect []) (\ok =>
  Choose (Macros.target (QualityNoun Color Nothing) {tk = ok}) Nothing Openly)
badTargetColor ObjectTgt impossible


||| "Destroy target creature. It gets +3/+3 until end of turn."
public export
badGetsGraveyard : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                Macros.gets It (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn) {ok}])
badGetsGraveyard Oh impossible


||| "Put target creature into target player's hand."
public export
badMoveToTargetsHand : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) (Macros.handOf (Macros.target AnyPlayer)) [] {ok})
badMoveToTargetsHand HandOkBare impossible


||| "Destroy target creature card in your graveyard."
public export
badDestroyGraveyard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badDestroyGraveyard Oh impossible


||| "You discard a creature."
public export
badDiscardBattlefield : Unspellable (Effect []) (\ok =>
  Macros.discard You (Macros.a Macros.creature) {dk = ok})
badDiscardBattlefield DiscardTracked impossible


||| "Destroy target creature card in a graveyard."
public export
badDestroyGraveyardCard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) {ok})
badDestroyGraveyardCard Oh impossible


||| A move labeled with a word outside the label catalog
public export
badUnknownVerbLabel : Unspellable (Effect []) (\ok =>
  Enact "Descry" (Move (Macros.a Macros.creature) Macros.graveyardZ []) {kn = ok})
badUnknownVerbLabel Oh impossible


||| "Choose zero target creatures."
public export
badZeroGroup : Unspellable (Effect []) (\ok =>
  Choose (Macros.targets (Macros.exactly 0) Macros.creature {ok}) Nothing Openly)
badZeroGroup (MaxAtLeastOne, _, _) impossible


||| "your battlefield"
public export
badOwnedBattlefield : Unspellable (ZoneExpr []) (\ok =>
  ZoneAt Battlefield (PossessedBy You {ps = ok}))
badOwnedBattlefield HandIsOwned impossible


public export
badDiscardedCreatureWord : Unspellable Ability (\ok =>
  Activated (Do (Macros.discard You (Macros.aAtRandom (And [Macros.creature, InZone Macros.handZ]))))
            (DealDamage This
                          (Macros.manaValueOf (TheVerbed "Discard" (TypeW Creature) Attributive {ok}))
                          (Macros.target Macros.anyTarget)) Nothing Nothing Nothing Nothing)
badDiscardedCreatureWord Refl impossible


||| "Destroy a creature of their choice."
public export
badUnboundTheirChoice : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.aTheirChoice Macros.creature {ch = ok}))
badUnboundTheirChoice Refl impossible


||| "This deals that much damage to any target."
public export
badThatMuchUnbound : Unspellable (Effect []) (\ok =>
  DealDamage This (ThatMuch {ok}) (Macros.target Macros.anyTarget))
badThatMuchUnbound Refl impossible


public export
badThatMuchAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.target Macros.anyTarget),
                Macros.losesLife You (Lit 2),
                Macros.gainsLife You (ThatMuch {ok})])
badThatMuchAmbig Refl impossible


||| "1 life for each 0 creatures"
public export
badForEachZero : Unspellable (Amount []) (\ok =>
  Macros.nForEach 0 Macros.creature {nz = ok})
badForEachZero ItIsSucc impossible


||| "creature that isn't on the battlefield"
public export
badNotOnBattlefield : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Not (InZone Macros.battlefieldZ)] {zc = ok})
badNotOnBattlefield Oh impossible


||| "of the chosen color and not of the chosen color"
public export
badQualityContradiction : Unspellable
  (Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object)
  (\ok => And [OfChosen Color, Not (OfChosen Color)] {cf = ok})
badQualityContradiction Oh impossible


||| "creature that is a noncreature"
public export
badNestedContradiction : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, And [Not Macros.creature]] {cf = ok})
badNestedContradiction Oh impossible


||| "attacking card in your hand"
public export
badAttackingInHand : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, InZone Macros.handZ] {zc = ok})
badAttackingInHand Oh impossible


||| "Discard this creature."
public export
badDiscardThisCreature : Unspellable (Effect []) (\ok =>
  Macros.discard You Macros.thisCreature {dk = ok})
badDiscardThisCreature DiscardTracked impossible


||| "this creature"
public export
badAscribedTarget : Unspellable (Noun [] Object) (\ok =>
  AsType Creature (Macros.target Macros.creature) Nothing {asc = ok})
badAscribedTarget Oh impossible


||| "creature you control that you don't control"
public export
badControlContradiction : Unspellable (Predicate [] Object) (\ok =>
  And [HasPossessor ControllerAx You, Not (HasPossessor ControllerAx You)] {cf = ok})
badControlContradiction Oh impossible


||| "attacking noncreature"
public export
badAttackingNoncreature : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, Not Macros.creature] {cf = ok})
badAttackingNoncreature Oh impossible


public export
badAltHeaderMixedReadback : Unspellable Ability (\ok =>
  Triggered Whenever (Blocks Macros.thisCreature Nothing)
            [BecomesBlocked Macros.thisCreature
                            (Just (Macros.a Macros.creature))]
            Nothing [] Nothing Nothing Nothing
            (Macros.gets (That (TypeW Creature) {ok = Builtin.fst ok}) (PtDown (Lit 1))
                         (PtDown (Lit 1)) {ok = Builtin.snd ok} (Just Macros.untilEndOfTurn)))
badAltHeaderMixedReadback (Refl, _) impossible


public export
badThreeArmHeaderReadback : Unspellable Ability (\ok =>
  Triggered Whenever (Macros.attacks Macros.thisCreature)
            [ Blocks Macros.thisCreature Nothing
            , BecomesTarget Macros.thisCreature (Macros.a Macros.spell) ]
            Nothing [] Nothing Nothing Nothing
            (DealDamage Macros.thisCreature (Macros.powerOf (It {ok = ok}))
                        (Macros.each Opponent)))
badThreeArmHeaderReadback Refl impossible


public export
badJoinedHeaderReadback : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing
            [ Macros.joinedHead Whenever
                (Dies (Macros.a Macros.creatureYouControl)) ]
            Nothing Nothing Nothing
            (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                         (It {ok = ok})))
badJoinedHeaderReadback Refl impossible


||| "Whenever this creature attacks while a creature is dying, draw a card."
public export
badWhileDoingMoment : Unspellable Ability (\ok =>
  Triggered Whenever (Macros.attacks Macros.thisCreature) []
            (Just (WhileDoing (Dies (Macros.a Macros.creature)) {up = ok}))
            [] Nothing Nothing Nothing
            (Macros.draw You (Lit 1)))
badWhileDoingMoment Oh impossible
