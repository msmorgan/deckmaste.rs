module Experimental.ProofsDamage

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Target creature fights target creature."
public export
okFightCreatures : Instruction []
okFightCreatures =
  Fights (Macros.target Macros.creature) (Macros.target Macros.creature)

||| "Two target creatures fight target creature."
public export
badFightGroup : Unspellable (Instruction []) (\ok =>
  Fights (Described (TargetDet (Macros.exactly 2)) Macros.creature) {pa = ok}
         (Macros.target Macros.creature))
badFightGroup Refl impossible

||| "This deals 2 damage to any target and 1 damage to any other target."
public export
okAnyOtherTarget : Instruction []
okAnyOtherTarget =
  Sequentially [DealDamage This (Lit 2) (Macros.target Macros.anyTarget),
                DealDamage This (Lit 1) (Macros.target Macros.anyOtherTarget)]

||| "This deals 1 damage to any other target."
public export
badOther : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 1) (Macros.target (Macros.anyOtherTarget {ok})))
badOther Refl impossible

||| "Tap target creature. It gets -1/-1 until end of turn."
public export
okIt : Instruction []
okIt =
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                Macros.gets (Macros.It OneOf) (PtDown (Lit 1)) (PtDown (Lit 1))
                            (Just Macros.untilEndOfTurn)]

||| "Target creature fights target creature. Tap it."
public export
badIt : Unspellable (Instruction []) (\ok =>
  Sequentially [Fights (Macros.target Macros.creature) (Macros.target Macros.creature),
                SetStatus Tapped ((Macros.It OneOf) {ok})])
badIt Refl impossible

||| "Tap target creature."
public export
okTapBattlefield : Instruction []
okTapBattlefield = SetStatus Tapped (Macros.target Macros.creature)

||| "This deals 3 damage to each creature. Tap it."
public export
badTheyIt : Unspellable (Instruction []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.each Macros.creature),
                SetStatus Tapped ((Macros.It OneOf) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badTheyIt (Refl, _) impossible

||| "Choose two target creatures. Tap them."
public export
okThem : Instruction []
okThem =
  Sequentially [Choose Nothing (Described (TargetDet (Macros.exactly 2)) Macros.creature)
                       Openly,
                SetStatus Tapped (Macros.It ManyOf)]

||| "Choose two target creatures. Choose two target creatures. Tap them."
public export
badThemAmbig : Unspellable (Instruction []) (\ok =>
  Sequentially [Choose Nothing (Described (TargetDet (Macros.exactly 2)) Macros.creature) Openly,
               Choose Nothing (Described (TargetDet (Macros.exactly 2)) Macros.creature) Openly,
               SetStatus Tapped ((Macros.It ManyOf) {ok})])
badThemAmbig Refl impossible

public export
badInnerAmbig : Unspellable (Instruction []) (\ok =>
  Sequentially [Fights (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent]))
                       (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent])),
               Macros.losesLife (Macros.That PlayerW OneOf {ok}) (Lit 1)])
badInnerAmbig Refl impossible

||| "A creature doesn't untap during your untap step."
public export
okUntapLockBattlefield : Ability
okUntapLockBattlefield =
  Static (Macros.doesntUntap (Macros.a Macros.creature) (Just You))

public export
badUntapLockGraveyard : Unspellable Ability (\ok =>
  Static (Macros.doesntUntap (Macros.a (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                             (Just You) {dp = ok}))
badUntapLockGraveyard Oh impossible

||| "Target creature card in your graveyard fights target creature."
public export
badFightGraveyard : Unspellable (Instruction []) (\ok =>
  Fights (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {za = ok}
         (Macros.target Macros.creature))
badFightGraveyard Oh impossible

||| "Target land fights target creature you don't control."
public export
badFightLand : Unspellable (Instruction []) (\ok =>
  Fights (Macros.target (HasType Land)) {ta = ok} (Macros.target Macros.creatureYouDontControl))
badFightLand Oh impossible

||| "Target permanent fights target creature." [CR#701.14a]
public export
badFightPermanent : Unspellable (Instruction []) (\ok =>
  Fights (Macros.target Permanent) {ta = ok} (Macros.target Macros.creature))
badFightPermanent Oh impossible

||| "This deals 3 damage to target creature."
public export
okDamageCreature : Instruction []
okDamageCreature = DealDamage This (Lit 3) (Macros.target Macros.creature)

||| "Destroy target creature. This deals 3 damage to it."
public export
badDamageGraveyardCard : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
               DealDamage This (Lit 3) ((Macros.It OneOf)) {rk = ok}])
badDamageGraveyardCard ObjectTakes impossible

||| "This deals 1 damage to a color."
public export
badDamageToColor : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 1) (Macros.a (QualityNoun Color Nothing)) {rk = ok})
badDamageToColor ObjectTakes impossible

||| "This deals 1 damage to target artifact."
public export
badDamageArtifact : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 1) (Macros.target (HasType Artifact)) {rk = ok})
badDamageArtifact ObjectTakes impossible

||| "... loses 1 life for each attacking creature. You gain that much life."
public export
okThatMuchBound : Instruction []
okThatMuchBound =
  Sequentially [Macros.losesLife (Macros.target Opponent)
                  (Macros.forEach 1 (And [Attacking, Macros.creature,
                                          HasPossessor ControllerAx You])),
                Macros.gainsLife You ThatMuch]

||| "This deals that much damage to any target."
public export
badThatMuchUnbound : Unspellable (Instruction []) (\ok =>
  DealDamage This (ThatMuch {ok}) (Macros.target Macros.anyTarget))
badThatMuchUnbound Refl impossible

public export
badThatMuchAmbig : Unspellable (Instruction []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.target Macros.anyTarget),
                Macros.losesLife You (Lit 2),
                Macros.gainsLife You (ThatMuch {ok})])
badThatMuchAmbig Refl impossible

||| "This deals 3 damage to target creature. Destroy it."
public export
okDestroyDamagedCreature : Instruction []
okDestroyDamagedCreature =
  Sequentially [DealDamage This (Lit 3) (Macros.target Macros.creature),
                Macros.destroy (Macros.It OneOf)]

||| "This deals 3 damage to any target. Destroy it."
public export
badDestroyAnyTargetRemention : Unspellable (Instruction []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.target Macros.anyTarget),
               Macros.destroy ((Macros.It OneOf)) {ok}])
badDestroyAnyTargetRemention Oh impossible

||| "This deals 1 damage to target creature."
public export
okDamageOneToCreature : Instruction []
okDamageOneToCreature = DealDamage This (Lit 1) (Macros.target Macros.creature)

||| "This deals 1 damage to this spell."
public export
badDamageThis : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 1) This {rk = ok})
badDamageThis ObjectTakes impossible

||| "creature you control or artifact you control"
public export
okStructuredDisjunction : Predicate [] Object
okStructuredDisjunction =
  Or [And [Macros.creature, HasPossessor ControllerAx You],
      And [Macros.artifact, HasPossessor ControllerAx You]]

||| "other creature or land"
public export
badOtherInOr : Unspellable
  (Predicate [MkBinding TargetD Object OneOf
                        (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing Nothing)] Object)
  (\ok => Or [And [Macros.creature, Other], Macros.land] {cd = ok})
badOtherInOr Oh impossible

||| "This deals 2 damage to target creature or planeswalker." [CR#120.1]
public export
okDamageDisjunctHead : Instruction []
okDamageDisjunctHead =
  DealDamage This (Lit 2) (Macros.target (Or [Macros.creature, HasType Planeswalker]))

||| "This deals 2 damage to target artifact or enchantment."
public export
badDamageDisjunctHead : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 2) (Macros.target (Or [Macros.artifact, Macros.enchantment])) {rk = ok})
badDamageDisjunctHead ObjectTakes impossible

||| "attacking or blocking creature in your graveyard"
public export
badAttackingOrBlockingInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Or [Attacking, Blocking], InZone Macros.graveyardZ] {zc = ok})
badAttackingOrBlockingInGraveyard Oh impossible

||| "This deals 2 damage to target creature. Tap it."
public export
okTapDamagedCreature : Instruction []
okTapDamagedCreature =
  Sequentially [DealDamage This (Lit 2) (Macros.target Macros.creature),
                SetStatus Tapped (Macros.It OneOf)]

||| "You gain 2 life if you control a creature. Tap it."
public export
badConditionAntecedent : Unspellable (Instruction []) (\ok =>
  Sequentially [OnlyIf (Macros.gainsLife You (Lit 2)) (Macros.exists Macros.creatureYouControl) Nothing,
                SetStatus Tapped ((Macros.It OneOf) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badConditionAntecedent (Refl, _) impossible

||| "You may sacrifice a creature. If you don't, exile it."
public export
badIfNotReadsMayBody : Unspellable (Instruction []) (\ok =>
  (May You (Macros.sacrifice You (Macros.a Macros.creature)) Nothing (Just (Macros.exile You ((Macros.It OneOf) {ok})))))
badIfNotReadsMayBody Refl impossible

||| "other than this creature"
public export
okComplementThisCreature : Predicate [] Object
okComplementThisCreature = OtherThan Macros.thisCreature

||| "other than a creature"
public export
badComplementAnchorAnnounces : Unspellable (Predicate [] Object) (\ok =>
  OtherThan (Macros.a Macros.creature) {ca = ok})
badComplementAnchorAnnounces MkComplementAnchor impossible

||| "other than up to two target creatures"
public export
badPluralComplementAnchor : Unspellable (Predicate [] Object) (\ok =>
  OtherThan (Described (TargetDet (Macros.upTo 2)) Macros.creature) {ca = ok})
badPluralComplementAnchor MkComplementAnchor impossible

||| "each creature other than this land"
public export
badComplementCrossHead : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 1) (Macros.each (And [Macros.creature, OtherThan Macros.thisLand] {oa = ok})))
badComplementCrossHead Oh impossible

||| "each creature other than this creature other than this creature"
public export
badDoubleComplement : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 1)
             (Macros.each (And [Macros.creature, OtherThan Macros.thisCreature, OtherThan Macros.thisCreature] {oa = ok})))
badDoubleComplement Oh impossible

||| "each other creature other than this creature"
public export
badOtherAndComplement : Unspellable (Instruction []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                DealDamage This (Lit 1)
                           (Macros.each (And [Macros.creature, Other, OtherThan Macros.thisCreature] {oa = ok}))])
badOtherAndComplement Oh impossible

||| "creature other than this creature, or land"
public export
badComplementInOr : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.creature, OtherThan Macros.thisCreature], Macros.land] {cd = ok})
badComplementInOr Oh impossible

||| "This deals 2 damage divided as you choose among two target creatures."
public export
okDivideAmongTargets : Instruction []
okDivideAmongTargets =
  Macros.dealsDivided This (Lit 2)
                      (Described (TargetDet (Macros.oneThrough 2)) Macros.creature)

||| "This deals 2 damage divided as you choose among each creature."
public export
badDivideAmongDescription : Unspellable (Instruction []) (\ok =>
  Macros.dealsDivided This (Lit 2) (Macros.each Macros.creature) {gm = ok})
badDivideAmongDescription Oh impossible

||| "Put one of them into your hand and the rest into your graveyard."
public export
okRestAfterPart : Instruction []
okRestAfterPart =
  Sequentially [ Macros.lookAt (Macros.topSlice (Lit 4))
               , Move (Macros.someOf (Macros.exactly 1) (Macros.It ManyOf)) Macros.handZ []
               , Move (Macros.theRest Object) Macros.graveyardZ []
               ]

||| "Put the rest into your graveyard."
public export
badRestWithoutGroup : Unspellable (Instruction []) (\ok =>
  Move (Macros.theRest Object {ok}) Macros.graveyardZ [])
badRestWithoutGroup Oh impossible

||| "Look at the top four cards of your library. Put the rest on the bottom."
public export
badRestWithoutPart : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.lookAt ((Macros.topSlice (Lit 4))), Move (Macros.theRest Object {ok}) Macros.onBottomZ []])
badRestWithoutPart Oh impossible

public export
badRestDisposedTwice : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.lookAt ((Macros.topSlice (Lit 4)))
               , Move (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))) Macros.handZ []
               , Move (Macros.theRest Object) Macros.onBottomZ []
               , Move (Macros.theRest Object {ok}) Macros.graveyardZ []
               ])
badRestDisposedTwice Oh impossible

public export
badRestOverTwoAnnouncements : Unspellable (Instruction []) (\ok =>
  Sequentially [ Fights (Macros.target Macros.creatureYouControl) (Macros.target Macros.creatureYouDontControl)
               , Move (Macros.theRest Object {ok}) Macros.graveyardZ []
               ])
badRestOverTwoAnnouncements Oh impossible

||| "This creature deals 2 damage to each opponent."
public export
okEachOpponentDamage : Instruction []
okEachOpponentDamage =
  DealDamage Macros.thisCreature (Lit 2) (Macros.each Opponent)

||| "This creature deals 2 damage to your opponents."
public export
badPluralPlayerDamageRecipient : Unspellable (Instruction []) (\ok =>
  DealDamage Macros.thisCreature (Lit 2) (PlayerGroup YourOpponents) {pm = ok})
badPluralPlayerDamageRecipient Oh impossible

||| "Prevent all damage that would be dealt to any player or permanent."
public export
okPreventDealtToPermanent : StaticSpec []
okPreventDealtToPermanent =
  DamageRule AnyDamage Unattributed
             (ToRecipient (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
             (Prevent CutAll Nothing) Repeatedly

||| "Prevent all damage that would be dealt to this this turn."
public export
badPreventedBareThis : Unspellable (StaticSpec []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient This {rk = ok})
             (Prevent CutAll Nothing) Repeatedly)
badPreventedBareThis ObjectTakes impossible

||| "Prevent all damage that would be dealt to target artifact this turn."
public export
badPreventDealtToArtifact : Unspellable (StaticSpec []) (\ok =>
  DamageRule AnyDamage Unattributed
             (ToRecipient (Macros.target Macros.artifact) {rk = ok})
             (Prevent CutAll Nothing) Repeatedly)
badPreventDealtToArtifact ObjectTakes impossible

||| "… is dealt to target attacking creature instead."
public export
okRedirectToSingleCreature : StaticSpec []
okRedirectToSingleCreature =
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Redirect CutAll (Macros.target (And [Macros.creature, Attacking])))
             Repeatedly

||| "All damage that would be dealt to you is dealt to target artifact instead."
public export
badRedirectToArtifact : Unspellable (StaticSpec []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Redirect CutAll (Macros.target Macros.artifact) {rk = ok}) Repeatedly)
badRedirectToArtifact ObjectTakes impossible

public export
badRedirectToPlural : Unspellable (StaticSpec []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Redirect CutAll (Macros.allOf Macros.creatureYouControl) {one = ok}) Repeatedly)
badRedirectToPlural Refl impossible

||| "… If damage from a red source is prevented this way, you gain 3 life."
public export
okPreventedFromSourceAnnounced : StaticSpec []
okPreventedFromSourceAnnounced =
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Prevent CutAll
                (Just (If (PreventedFromSource (And [Macros.source, ColorIs Red]))
                          (Macros.gainsLife You (Lit 3)) Nothing)))
             Repeatedly

||| "If damage from a red source is prevented this way, you gain 3 life."
public export
badPreventedFromSourceUnannounced : Unspellable (Instruction []) (\ok =>
  If (PreventedFromSource (And [Macros.source, ColorIs Red]) {ok})
     (Macros.gainsLife You (Lit 3)) Nothing)
badPreventedFromSourceUnannounced Refl impossible

||| "… You gain life equal to the damage prevented this way."
public export
okPreventedThisWayAnnounced : StaticSpec []
okPreventedThisWayAnnounced =
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Prevent CutAll (Just (Macros.gainsLife You Macros.preventedThisWay)))
             Repeatedly

public export
badPreventedThisWayAfterDamage : Unspellable (Instruction []) (\ok =>
  Sequentially [ DealDamage This (Lit 3) (Macros.target Macros.creature)
               , ChangeLife You (LifeUp (Macros.preventedThisWay {ok})) ])
badPreventedThisWayAfterDamage Refl impossible

||| "You gain life equal to the damage prevented this way."
public export
badPreventedThisWayUnannounced : Unspellable (Instruction []) (\ok =>
  ChangeLife You (LifeUp (Macros.preventedThisWay {ok})))
badPreventedThisWayUnannounced Refl impossible

||| "… They gain 2 life for each card less than two they drew this way."
public export
okShortOfCeilingAnnounced : Instruction []
okShortOfCeilingAnnounced =
  Sequentially [ Macros.may (Macros.each AnyPlayer) (Draw They (UpTo (Lit 2)))
               , Macros.gainsLife They (Macros.times 2 Macros.shortOfCeiling) ]

||| "You gain 2 life for each card less than two you draw this way."
public export
badShortOfCeilingUnannounced : Unspellable (Instruction []) (\ok =>
  Macros.gainsLife You (Macros.times 2 (Macros.shortOfCeiling {ok})))
badShortOfCeilingUnannounced Refl impossible

public export
badShieldSizedByItsOwnPrevention : Unspellable (StaticSpec []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Prevent (Shield (Macros.preventedThisWay {ok})) Nothing) Repeatedly)
badShieldSizedByItsOwnPrevention Refl impossible

||| "Prevent the next 3 damage that would be dealt to you this turn."
public export
okShieldRepeatedly : StaticSpec []
okShieldRepeatedly =
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Prevent (Shield (Lit 3)) Nothing) Repeatedly

public export
badShieldNextTimeOnly : Unspellable (StaticSpec []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Prevent (Shield (Lit 3)) Nothing) NextTimeOnly {su = ok})
badShieldNextTimeOnly Oh impossible

||| "This creature deals 3 damage to target creature."
public export
okDamageToCreature : Instruction []
okDamageToCreature = DealDamage This (Lit 3) (Macros.target Macros.creature)

||| "This creature deals 3 damage to target source."
public export
badDamageToSource : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 3) (Macros.target Macros.source) {rk = ok})
badDamageToSource ObjectTakes impossible

||| "…target nonartifact…"
public export
okNonartifact : Predicate [] Object
okNonartifact = Not Macros.artifact

||| "…target nonsource…"
public export
badNonsource : Unspellable (Predicate [] Object) (\ok =>
  Not Macros.source {ng = ok})
badNonsource Oh impossible

public export
badRedirectToGroup : Unspellable (StaticSpec []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Redirect CutAll
                (Macros.youAnd (Macros.allOf (And [Permanent, HasPossessor ControllerAx You])))
                {one = ok})
             Repeatedly)
badRedirectToGroup Refl impossible

||| "Target opponent loses 1 life. You gain that much life."
public export
okThatMuchAfterLifeLoss : Instruction []
okThatMuchAfterLifeLoss =
  Sequentially [ Macros.losesLife (Macros.target Opponent) (Lit 1)
               , Macros.gainsLife You ThatMuch ]

||| "… it deals that much damage plus that much instead."
public export
badScaleShiftByThatMuch : Unspellable (StaticSpec []) (\ok =>
  DamageRule AnyDamage (DealtBy (Macros.a Macros.source))
             (ToRecipient (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
             (Scale (Shifted ShiftUp (ThatMuch {ok}))) Repeatedly)
badScaleShiftByThatMuch Refl impossible

public export
badScaleToArtifact : Unspellable (StaticSpec []) (\ok =>
  DamageRule AnyDamage (DealtBy (Macros.a Macros.source))
             (ToRecipient (Macros.target Macros.artifact) {rk = ok})
             (Scale (Multiplied Doubled)) Repeatedly)
badScaleToArtifact ObjectTakes impossible

public export
badPreventedThisWayAfterDamageEvent : Unspellable Ability (\ok =>
  Triggered Whenever (IsDealtDamage AnyDamage Macros.thisCreature) [] Nothing [] Nothing Nothing Nothing
            (DealDamage ((Macros.It OneOf)) (Macros.preventedThisWay {ok = ok}) (Macros.target Macros.anyTarget)))
badPreventedThisWayAfterDamageEvent Refl impossible

public export
badThatMuchAfterDeath : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.a Macros.creature)) [] Nothing [] Nothing Nothing Nothing
            (DealDamage Macros.thisCreature (ThatMuch {ok = ok})
                        (Macros.target Macros.anyTarget)))
badThatMuchAfterDeath Refl impossible

||| "… That creature deals damage equal to its power to this creature."
public export
okThatCreatureAfterDamage : Instruction []
okThatCreatureAfterDamage =
  Sequentially [ DealDamage Macros.thisCreature
                            (StatOf Power Macros.thisCreature)
                            (Macros.target Macros.creature)
               , DealDamage (Macros.That (TypeW Creature) OneOf)
                            (StatOf Power (Macros.It OneOf))
                            Macros.thisCreature ]

public export
badThatCreatureIsDamagedSelf : Unspellable Ability (\ok =>
  Triggered Whenever (IsDealtDamage AnyDamage Macros.thisCreature) [] Nothing [] Nothing Nothing Nothing
            (DealDamage (Macros.That (TypeW Creature) OneOf {ok = ok}) ThatMuch
                        (Macros.target Macros.anyTarget)))
badThatCreatureIsDamagedSelf Refl impossible

||| "Prevent all damage sources of the last chosen color would deal to you."
public export
okLastChosenAfterChooser : Card
okLastChosenAfterChooser =
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort Color) Nothing
                              Openly)
       , Static (DamageRule AnyDamage
                   (DealtBy (Macros.allOf (And [Macros.source,
                                                Macros.ofTheLastChosen Color])))
                   (ToRecipient You) (Prevent CutAll Nothing) Repeatedly) ]
       Nothing

||| "sources of the last chosen color"
public export
badLastChosenColorNoChooser : Unspellable (Predicate [] Object) (\ok =>
  Macros.ofTheLastChosen Color {ok = ok})
badLastChosenColorNoChooser Oh impossible

public export
badLastChosenBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.allOf (And [Macros.source,
                                                Macros.ofTheLastChosen Color {ok = ok}]))) (ToRecipient You) (Prevent CutAll Nothing) Repeatedly)
       , Static (EntersChoice Macros.thisEnchantment (QSort Color) Nothing Openly) ]
       Nothing)
badLastChosenBeforeChooser Oh impossible

public export
badLastChosenWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly)
       , Static (DamageRule AnyDamage (DealtBy (Macros.allOf (And [Macros.source,
                                                Macros.ofTheLastChosen Color {ok = ok}]))) (ToRecipient You) (Prevent CutAll Nothing) Repeatedly) ]
       Nothing)
badLastChosenWrongSort Oh impossible

||| "the greatest life total among all players"
public export
okPlayerAggregate : Amount []
okPlayerAggregate =
  Aggregate MaxOf (PlayerStatAxis LifeTotal) (Macros.allOf AnyPlayer)

||| "the total power of target creature"
public export
badSingularAggregate : Unspellable (Amount []) (\ok =>
  Aggregate SumOf (StatAxis Power) (Macros.target Macros.creature)
              {pl = ok})
badSingularAggregate Refl impossible

||| "the greatest life total among all creatures"
public export
badAggregateWrongSort : Unspellable (Amount []) (\ok =>
  Aggregate MaxOf (PlayerStatAxis LifeTotal) (Macros.allOf Macros.creature)
              {sc = ok})
badAggregateWrongSort Refl impossible

||| "up to X | Draw a card."
public export
badAmountRollRow : Unspellable (Instruction []) (\ok =>
  Sequentially [(Macros.rollDice You 1 20),
                ResultsTable [MkRollRow (UpToOf (LetterVal X))
                                        (Draw You (Lit 1)) {lt = ok}]])
badAmountRollRow Oh impossible

public export
badCreatureHalfRead : Unspellable (Instruction []) (\ok =>
  Sequentially
    [ DealDamage This (Lit 3)
        (Macros.target (Macros.kindJoin AnyPlayer (HasType Planeswalker)))
    , (Macros.discard
        (EitherOf (Pro (UnionHalf PlayerW) OneOf)
                  (Macros.controllerOf (Macros.That (TypeW Creature) OneOf {ok = ok})))
        (Macros.a (InZone Macros.handZ))) ])
badCreatureHalfRead Refl impossible

||| "This deals 3 damage to any target."
public export
okJoinDamageRecipient : Instruction []
okJoinDamageRecipient =
  DealDamage This (Lit 3) (Macros.target Macros.anyTarget)

||| "This deals 3 damage to a land or a land."
public export
badSameKindJoinDamage : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 3)
             (Macros.a (Joined (HasType Land) (HasType Land))) {rk = ok})
badSameKindJoinDamage JoinTakes impossible

||| "Create a 1/1 white Soldier creature token."
public export
okSoldierToken : Instruction []
okSoldierToken =
  Macros.create (Lit 1)
                (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])

||| "Create a 1/1 black Zombie artifact token."
public export
badZombieArtifactToken : Unspellable (Instruction []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [Black] (MkTypeLine [creatureType "Zombie"] [Artifact])
                          [] Nothing) {wf = ok})
badZombieArtifactToken (Oh, Oh, Oh, Oh, Oh, Oh) impossible

||| "Create a white Soldier creature token."
public export
badCreatureTokenNoPt : Unspellable (Instruction []) (\ok =>
  Macros.create (Lit 1) (MkToken Nothing [White] (MkTypeLine [creatureType "Soldier"] [Creature]) [] Nothing) {wf = ok})
badCreatureTokenNoPt (Oh, Oh, Oh, Oh, Oh, Oh) impossible

||| "Create a 1/1 white token."
public export
badTypelessToken : Unspellable (Instruction []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [White] (MkTypeLine [] []) [] Nothing) {wf = ok})
badTypelessToken (Oh, Oh, Oh, Oh, Oh, Oh) impossible

||| "You gain life equal to your life total."
public export
okSingularLifeTotalRead : Amount []
okSingularLifeTotalRead = PlayerStatOf LifeTotal You

||| "You gain life equal to your opponents' life totals."
public export
badPluralLifeTotalRead : Unspellable (Amount []) (\ok =>
  PlayerStatOf LifeTotal (PlayerGroup YourOpponents) {one = ok})
badPluralLifeTotalRead Refl impossible

||| "You gain life equal to each player's life total."
public export
badDistributiveLifeTotalRead : Unspellable (Amount []) (\ok =>
  PlayerStatOf LifeTotal (Macros.each AnyPlayer) {one = ok})
badDistributiveLifeTotalRead Refl impossible

||| "Destroy target creature."
public export
okDestroyCreature : Instruction []
okDestroyCreature = Macros.destroy (Macros.target Macros.creature)

||| "Destroy target source."
public export
badDestroySource : Unspellable (Instruction []) (\ok =>
  Macros.destroy (Macros.target Macros.source) {ok})
badDestroySource Oh impossible

||| "Target creature gets +1/+1 until end of turn."
public export
okGetsCreature : Instruction []
okGetsCreature =
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 1)) (PtUp (Lit 1))
              (Just Macros.untilEndOfTurn)

||| "Target source gets +1/+1 until end of turn." [CR#609.7a]
public export
badGetsSource : Unspellable (Instruction []) (\ok =>
  Macros.gets (Macros.target Macros.source) (PtUp (Lit 1)) (PtUp (Lit 1)) {ok}
              (Just Macros.untilEndOfTurn))
badGetsSource Oh impossible
