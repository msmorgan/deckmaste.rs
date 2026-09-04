module Experimental.ProofsDeontic

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Target creature can't be blocked this turn."
public export
okCantBeBlocked : Effect []
okCantBeBlocked =
  Continuously {ts = StaticFirstDone}
    (Macros.deontic (Macros.target Macros.creature) Forbid ["Block"] Patient
                    NoDeonticPatient)
    (Just ThisTurn)

||| "Target creature can't be attacked this turn."
public export
badCantBeAttacked : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.target Macros.creature) Forbid ["Attack"] Patient NoDeonticPatient {dp = ok}) (Just ThisTurn))
badCantBeAttacked Oh impossible

||| "Target creature card in a graveyard can't block this turn."
public export
badCantInGraveyard : Unspellable (Effect []) (\ok =>
  Macros.cantBlock (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) (Just ThisTurn) {dp = ok})
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

||| "You pay 2 life."
public export
okPayLifeCost : Effect []
okPayLifeCost = Pay You (Macros.payLife You 2) PaidOnce

||| "You pay {T}."
public export
badPayTapSymbol : Unspellable (Effect []) (\ok =>
  Pay You TapSymbol PaidOnce {pb = ok})
badPayTapSymbol Oh impossible

||| "Creatures you control get +1/+1 until end of turn:"
public export
badContinuousAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.gets (Macros.allOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1))
                             (Just Macros.untilEndOfTurn)) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badContinuousAsCost Oh impossible

||| a replacement written as a cost
public export
badInsteadAsCost : Unspellable Ability (\ok =>
  Activated (Do (InsteadOf (Macros.destroy (Macros.target Macros.creature))
                           (Macros.exile You (Macros.target Macros.creature))) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badInsteadAsCost Oh impossible

||| a delayed trigger written as a cost
public export
badDelayedAsCost : Unspellable Ability (\ok =>
  Activated (Do (Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (Draw You (Lit 1))) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badDelayedAsCost Oh impossible

||| an "until" rider written as a cost
public export
badHeldUntilAsCost : Unspellable Ability (\ok =>
  Activated (Do (HeldUntil (Macros.exile You (Macros.target Macros.creature))
                           (Dies (Macros.a Macros.creature))) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badHeldUntilAsCost Oh impossible

||| a reflexive trigger written as a cost
public export
badReflexiveAsCost : Unspellable Ability (\ok =>
  Activated (Do (Reflexively (Macros.gainsLife You (Lit 2)) (Draw You (Lit 1))) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badReflexiveAsCost Oh impossible

||| "You skip your next turn:"
public export
badSkipAsCost : Unspellable Ability (\ok =>
  Activated (Do (SkipsNext You Turn (Lit 1)) {ok}) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badSkipAsCost Oh impossible

||| "You pay 2 life:"
public export
badPayAsCost : Unspellable Ability (\ok =>
  Activated (Do (Pay You (Macros.payLife You 2) PaidOnce) {ok}) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badPayAsCost Oh impossible

||| "Discard a card, then sacrifice a creature:"
public export
badSequentialCost : Unspellable Ability (\ok =>
  Activated (Do (Sequentially [(Macros.discard You (Macros.a (InZone Macros.handZ))),
                               Macros.sacrifice You (Macros.a Macros.creature)]) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badSequentialCost Oh impossible

||| "Discard a card and sacrifice a creature simultaneously:"
public export
badSimultaneousCost : Unspellable Ability (\ok =>
  Activated (Do (Simultaneously [(Macros.discard You (Macros.a (InZone Macros.handZ))),
                                 Macros.sacrifice You (Macros.a Macros.creature)]) {ok})
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badSimultaneousCost Oh impossible

||| "Repeat this process:"
public export
badRepeatAsCost : Unspellable Ability (\ok =>
  Activated (Do (Repeat Again) {ok}) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badRepeatAsCost Oh impossible

||| a compound cost of no components
public export
badEmptyCompound : Unspellable Ability (\ok =>
  Activated (Compound [] {ne = ok}) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badEmptyCompound ItIsSucc impossible

||| "Creatures can't attack."
public export
okStaticUntargeting : Ability
okStaticUntargeting =
  Static (Macros.deontic (Macros.allOf Macros.creature) Forbid ["Attack"] Agent
                         NoDeonticPatient)

||| "Target creature can't attack."
public export
badStaticTargets : Unspellable Ability (\ok =>
  Static (Macros.deontic (Macros.target Macros.creature) Forbid ["Attack"] Agent NoDeonticPatient) {ut = ok})
badStaticTargets Oh impossible

||| "You may play a card in your graveyard this turn."
public export
okPlayFromGraveyard : Effect []
okPlayFromGraveyard =
  Continuously {ts = StaticFirstDone}
               (Deontic You Permit ["Play"] Agent Nothing
                  (DeonticCounterpart (Macros.a (InZone Macros.graveyardZ)))
                  Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost))
               (Just ThisTurn)

||| "You may play a creature this turn."
public export
badPlayFromBattlefield : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Deontic You Permit ["Play"] Agent Nothing
                  (DeonticCounterpart (Macros.a Macros.creature)) Nothing
                  (PlayRider Nothing Nothing Nothing False ItsOwnCost) {rd = ok})
               (Just ThisTurn))
badPlayFromBattlefield Oh impossible

||| "You may play a land card from your graveyard this turn."
public export
okPlayLandFromGraveyard : Effect []
okPlayLandFromGraveyard =
  Continuously {ts = StaticFirstDone}
    (Deontic You Permit ["Play"] Agent Nothing
       (DeonticCounterpart
          (Macros.a (And [Macros.land,
                          InZone (Macros.graveyardOf You)])))
       Nothing
       (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing False
                  ItsOwnCost))
    (Just ThisTurn)

||| "You may play a spell this turn."
public export
badPlayFromStack : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Deontic You Permit ["Play"] Agent Nothing
                  (DeonticCounterpart (Macros.a Macros.spell)) Nothing
                  (PlayRider Nothing Nothing Nothing False ItsOwnCost) {rd = ok})
               (Just ThisTurn))
badPlayFromStack Oh impossible

||| "You may cast a creature card from your graveyard this turn."
public export
okCastCreatureFromGraveyard : Effect []
okCastCreatureFromGraveyard =
  Continuously {ts = StaticFirstDone}
    (Deontic You Permit ["Cast"] Agent Nothing
       (DeonticCounterpart
          (Macros.a (And [Macros.creature,
                          InZone (Macros.graveyardOf You)])))
       Nothing
       (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing False
                  ItsOwnCost))
    (Just ThisTurn)

||| "You may cast a land card from your graveyard this turn."
public export
badCastALand : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Deontic You Permit ["Cast"] Agent Nothing
                  (DeonticCounterpart
                     (Macros.a (And [Macros.land, InZone (Macros.graveyardOf You)])))
                  Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost)
                  {pt = ok})
               (Just ThisTurn))
badCastALand Oh impossible

||| "You may play a creature card in exile from your graveyard this turn."
public export
badPlayFromWrongZone : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Deontic You Permit ["Play"] Agent Nothing
                  (DeonticCounterpart
                     (Macros.a (And [Macros.creature, InZone Macros.exileZ])))
                  Nothing
                  (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing
                             False ItsOwnCost)
                  {rd = ok})
               (Just ThisTurn))
badPlayFromWrongZone Oh impossible

||| "unless" on a negated condition
public export
okUnlessOnNegated : Ability
okUnlessOnNegated =
  Static (Conditionally
            (NotCond (Macros.exists (And [Macros.artifact,
                                          HasPossessor ControllerAx You])))
            (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                            NoDeonticPatient)
            Unless {st = Static.CondFirstDone})

||| "unless"
public export
badUnlessOnPositive : Unspellable Ability (\ok =>
  Static (Conditionally (Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You]))
                        (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent NoDeonticPatient)
                        Unless {st = Static.CondFirstDone} {mk = ok}))
badUnlessOnPositive MkMarkingOk impossible

||| a trigger header watching a permanent become unflipped
public export
badUnflipEvent : Unspellable Ability (\ok =>
  Triggered Whenever (StatusEvent (Macros.a Permanent) Unflipped {at = ok}) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badUnflipEvent Oh impossible

||| "Target creature attacks each combat if able."
public export
okMustAttackCreature : Effect []
okMustAttackCreature =
  Continuously {ts = StaticFirstDone}
               (Macros.deontic (Macros.target Macros.creature) Require
                               ["Attack"] Agent NoDeonticPatient)
               (Just ThisTurn)

||| "Target land attacks each combat if able."
public export
badMustAttackLand : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.target Macros.land) Require ["Attack"] Agent NoDeonticPatient {dp = ok})
               (Just ThisTurn))
badMustAttackLand Oh impossible

||| "target creature card in your graveyard that is your Ring-bearer"
public export
badRingBearerInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, HasDesignation RingBearer,
       InZone (Macros.graveyardOf You)] {zc = ok})
badRingBearerInGraveyard Oh impossible

||| "This creature can't attack target planeswalker this turn."
public export
okForbidAttackPlaneswalker : Effect []
okForbidAttackPlaneswalker =
  Continuously {ts = StaticFirstDone}
               (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                        (DeonticCounterpart
                           (Macros.target (HasType Planeswalker))))
               (Just ThisTurn)

||| "This creature can't attack target creature this turn."
public export
badForbidAttackWithPatient : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                        (DeonticCounterpart (Macros.target Macros.creature)) {pt = ok})
               (Just ThisTurn))
badForbidAttackWithPatient Oh impossible

||| "Target creature blocks it this turn"
public export
badBlocksItself : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.target Macros.creature) Require ["Block"] Agent
                               (DeonticCounterpart ((Macros.It OneOf))) {pt = ok})
               (Just ThisTurn))
badBlocksItself Oh impossible

||| "Enchanted creature can't block."
public export
okEnchantedCreatureCantBlock : Ability
okEnchantedCreatureCantBlock =
  Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                  Forbid ["Block"] Agent NoDeonticPatient)

||| "Enchanted land gets +1/+1 and can't block."
public export
badCoordinatedLandHostBlocks : Unspellable Ability (\ok =>
  Static (AndAlso Nothing [ Gets Adds (AttachHost Enchanted (TypeW Land))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Macros.deontic ((Macros.It OneOf)) Forbid ["Block"] Agent NoDeonticPatient {dp = ok} ]))
badCoordinatedLandHostBlocks Oh impossible

||| "This deals 4 damage to target creature. The damage can't be prevented."
public export
okTheDamageAfterDealing : Effect []
okTheDamageAfterDealing =
  Sequentially [ DealDamage This (Lit 4) (Macros.target Macros.creature)
               , Continuously {ts = StaticFirstDone}
                   (CantPrevent AnyDamage ThatDamage NoPreventionOnly) Nothing ]

||| "The damage can't be prevented."
public export
badTheDamageUnannounced : Unspellable (StaticEffect []) (\ok =>
  CantPrevent AnyDamage (ThatDamage {ok}) NoPreventionOnly)
badTheDamageUnannounced Oh impossible

||| "You gain 3 life. The damage can't be prevented."
public export
badTheDamageAfterLifeGain : Unspellable (Effect []) (\ok =>
  Sequentially [ ChangeLife You (Up (Lit 3))
               , Continuously {ts = StaticFirstDone}
                   (CantPrevent AnyDamage (ThatDamage {ok}) NoPreventionOnly) Nothing ])
badTheDamageAfterLifeGain Oh impossible

||| "You may cast spells as though they had flash."
public export
okObjectPremiseAtCast : StaticEffect []
okObjectPremiseAtCast =
  Deontic You Permit ["Cast"] Agent Nothing
          (DeonticCounterpart (Macros.allOf Macros.spell))
          (Just (AsThoughOf (HasKeyword (TheKeyword "Flash"))))
          (PlayRider Nothing Nothing Nothing False ItsOwnCost)

||| "This creature can attack as though it were mana of any color."
public export
badManaPremiseAtAttack : Unspellable (StaticEffect []) (\ok =>
  Deontic Macros.thisCreature Permit ["Attack"] Agent Nothing NoDeonticPatient
          (Just (AsThoughMana Nothing MatchAnyColor Nothing)) NoDeonticRider
          {at = ok})
badManaPremiseAtAttack Oh impossible

||| "This creature can't be blocked by more than one creature."
public export
okBlockBoundOnBlock : StaticEffect []
okBlockBoundOnBlock =
  Deontic Macros.thisCreature Forbid ["Block"] Patient (Just (MoreThan (Lit 1)))
          (DeonticCounterpart (Macros.allOf Macros.creature)) Nothing
          NoDeonticRider

||| "This spell can't be countered more than once."
public export
badCounterBoundTwice : Unspellable (StaticEffect []) (\ok =>
  Deontic This Forbid ["Counter"] Patient (Just (MoreThan (Lit 1))) NoDeonticPatient
          Nothing NoDeonticRider {bd = ok})
badCounterBoundTwice Oh impossible

||| "You may spend mana as though it weren't a creature."
public export
badObjectPremiseAtSpend : Unspellable (StaticEffect []) (\ok =>
  Deontic You Permit ["Spend"] Agent Nothing NoDeonticPatient
          (Just (AsThoughOf (Not Macros.creature))) NoDeonticRider {at = ok})
badObjectPremiseAtSpend Oh impossible

||| "Each opponent discards a card, if those cards are creature cards."
public export
distributedDeedReadsBackPluralUnderCondition : Effect []
distributedDeedReadsBackPluralUnderCondition =
  OnlyIf (Macros.discard (Macros.each Opponent) (Macros.a (InZone Macros.handZ)))
         (Matches (Macros.That CardW ManyOf) Macros.creature) Nothing

||| "... sacrificed permanents can't be regenerated."
public export
distributedDeedRiderReadsBackPlural : Effect []
distributedDeedRiderReadsBackPlural =
  CantBe (Macros.sacrifice (Macros.each Opponent) (Macros.a Macros.creature))
         "Regenerate" (Macros.TheVerbed "Sacrifice" PermanentW Attributive ManyOf)

||| "... dealt damage, they lose half their life, rounded up."
public export
enchantedPlayerDamageReadsBackAsThey : Ability
enchantedPlayerDamageReadsBackAsThey =
  Macros.triggered Whenever (IsDealtDamage AnyDamage (AttachHost Enchanted PlayerW))
                   (Macros.losesLife They (Half RoundUp (PlayerStatOf LifeTotal They)))
