module Experimental.ProofsDeontic

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Target creature can't be blocked this turn."
public export
okCantBeBlocked : Instruction []
okCantBeBlocked =
  Continuously
    (Macros.deontic (Macros.target Macros.creature) Forbid ["Block"] Patient
                    NoDeonticPatient)
    (Just ThisTurn)

||| "Target creature can't be attacked this turn."
public export
badCantBeAttacked : Unspellable (Instruction []) (\ok =>
  Continuously (Macros.deontic (Macros.target Macros.creature) Forbid ["Attack"] Patient NoDeonticPatient {dp = ok}) (Just ThisTurn))
badCantBeAttacked Oh impossible

||| "Target creature card in a graveyard can't block this turn."
public export
badCantInGraveyard : Unspellable (Instruction []) (\ok =>
  Macros.cantBlock (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) (Just ThisTurn) {dp = ok})
badCantInGraveyard Oh impossible

||| "creature with power 2 or less"
public export
okCreaturePower : Predicate [] Object
okCreaturePower = And [Compare [StatAxis Power] AtMost (Lit 2), Macros.creature]

||| "noncreature with power 2 or less"
public export
badNoncreaturePower : Unspellable (Predicate [] Object) (\ok =>
  And [Compare [StatAxis Power] AtMost (Lit 2), Not Macros.creature] {cf = ok})
badNoncreaturePower Oh impossible

||| "with power 2 or less or with toughness 2 or less"
public export
okDistinctComparisonDisjuncts : Predicate [] Object
okDistinctComparisonDisjuncts =
  Or [Compare [StatAxis Power] AtMost (Lit 2),
      Compare [StatAxis Toughness] AtMost (Lit 2)]

||| "with power 2 or less or with power 2 or less"
public export
okRepeatedComparisonDisjunct : Predicate [] Object
okRepeatedComparisonDisjunct =
  Or [Compare [StatAxis Power] AtMost (Lit 2),
      Compare [StatAxis Power] AtMost (Lit 2)]

||| "with power 2 or less or mana value 3 or less"
public export
badMixedCharacteristicDisjunct : Unspellable (Predicate [] Object) (\ok =>
  Or [Compare [StatAxis Power] AtMost (Lit 2),
      Compare [StatAxis ManaValue] AtMost (Lit 3)] {pd = ok})
badMixedCharacteristicDisjunct Oh impossible

||| "You pay 2 life."
public export
okPayLifeCost : Instruction []
okPayLifeCost = Pay You (Macros.payLife You 2) PaidOnce

||| "You pay {T}."
public export
badPayTapSymbol : Unspellable (Instruction []) (\ok =>
  Pay You TapSymbol PaidOnce {pb = ok})
badPayTapSymbol Oh impossible

||| "Sacrifice a creature: Draw a card."
public export
okDeedAsCost : Ability
okDeedAsCost =
  Activated (Do (Macros.sacrifice You (Macros.a Macros.creature)))
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing

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
                           (Macros.exile (Macros.target Macros.creature))) {ok})
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
  Activated (Do (HeldUntil (Macros.exile (Macros.target Macros.creature))
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

||| "Discard a card, Sacrifice a creature: Draw a card."
public export
okCompoundCost : Ability
okCompoundCost =
  Activated (Compound [Do (Macros.discard You (Macros.a (InZone Macros.handZ))),
                       Do (Macros.sacrifice You (Macros.a Macros.creature))])
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing

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
okPlayFromGraveyard : Instruction []
okPlayFromGraveyard =
  Continuously
               (Deontic You Permit ["Play"] Agent Nothing
                  (DeonticCounterpart (Macros.a (InZone Macros.graveyardZ)))
                  Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost))
               (Just ThisTurn)

||| "You may play a creature this turn."
public export
badPlayFromBattlefield : Unspellable (Instruction []) (\ok =>
  Continuously (Deontic You Permit ["Play"] Agent Nothing
                  (DeonticCounterpart (Macros.a Macros.creature)) Nothing
                  (PlayRider Nothing Nothing Nothing False ItsOwnCost) {rd = ok})
               (Just ThisTurn))
badPlayFromBattlefield Oh impossible

||| "You may play a land card from your graveyard this turn."
public export
okPlayLandFromGraveyard : Instruction []
okPlayLandFromGraveyard =
  Continuously
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
badPlayFromStack : Unspellable (Instruction []) (\ok =>
  Continuously (Deontic You Permit ["Play"] Agent Nothing
                  (DeonticCounterpart (Macros.a Macros.spell)) Nothing
                  (PlayRider Nothing Nothing Nothing False ItsOwnCost) {rd = ok})
               (Just ThisTurn))
badPlayFromStack Oh impossible

||| "You may cast a creature card from your graveyard this turn."
public export
okCastCreatureFromGraveyard : Instruction []
okCastCreatureFromGraveyard =
  Continuously
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
badCastALand : Unspellable (Instruction []) (\ok =>
  Continuously (Deontic You Permit ["Cast"] Agent Nothing
                  (DeonticCounterpart
                     (Macros.a (And [Macros.land, InZone (Macros.graveyardOf You)])))
                  Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost)
                  {pt = ok})
               (Just ThisTurn))
badCastALand Oh impossible

||| "You may play a creature card in exile from your graveyard this turn."
public export
badPlayFromWrongZone : Unspellable (Instruction []) (\ok =>
  Continuously (Deontic You Permit ["Play"] Agent Nothing
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
            (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                            NoDeonticPatient)
            (NotCond (Macros.exists (And [Macros.artifact,
                                          HasPossessor ControllerAx You])))
            Unless)

||| "unless"
public export
badUnlessOnPositive : Unspellable Ability (\ok =>
  Static (Conditionally (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent NoDeonticPatient)
                        (Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You]))
                        Unless {mk = ok}))
badUnlessOnPositive MkMarkingOk impossible

||| a trigger header watching a permanent become unflipped
public export
badUnflipEvent : Unspellable Ability (\ok =>
  Triggered Whenever (StatusEvent (Macros.a Permanent) Unflipped {at = ok}) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badUnflipEvent Oh impossible

||| "Target creature attacks each combat if able."
public export
okMustAttackCreature : Instruction []
okMustAttackCreature =
  Continuously
               (Macros.deontic (Macros.target Macros.creature) Require
                               ["Attack"] Agent NoDeonticPatient)
               (Just ThisTurn)

||| "Target land attacks each combat if able.": a land an effect has made a
||| creature can attack [CR#205.1b], and the requirement is created even while
||| it is not one [CR#208.3a].
public export
okMustAttackLand : Instruction []
okMustAttackLand =
  Continuously (Macros.deontic (Macros.target Macros.land) Require ["Attack"] Agent NoDeonticPatient)
               (Just ThisTurn)

||| "your Ring-bearer": a creature holds the Ring-bearer designation for a
||| player, so the possessive is written [CR#701.54e].
public export
okRingBearerHolder : Predicate [] Object
okRingBearerHolder = HasDesignation RingBearer (Just You)

||| "your monarch": the monarch IS a player, so the designation has no
||| possessor — refused at the holder slot, not at the designation
||| [CR#725.1].
public export
badMonarchHolder : Unspellable (Predicate [] Player) (\ok =>
  HasDesignation Monarch (Just You) {hp = ok})
badMonarchHolder Oh impossible

||| "target creature you control that is your Ring-bearer"
public export
okRingBearerOnBattlefield : Predicate [] Object
okRingBearerOnBattlefield =
  And [Macros.creature, HasDesignation RingBearer (Just You),
       InZone Macros.battlefieldZ]

||| "target creature card in your graveyard that is your Ring-bearer"
public export
badRingBearerInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, HasDesignation RingBearer (Just You),
       InZone (Macros.graveyardOf You)] {zc = ok})
badRingBearerInGraveyard Oh impossible

||| "This creature can't attack target planeswalker this turn."
public export
okForbidAttackPlaneswalker : Instruction []
okForbidAttackPlaneswalker =
  Continuously
               (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                        (DeonticCounterpart
                           (Macros.target (HasType Planeswalker))))
               (Just ThisTurn)

||| "This creature can't attack target creature this turn."
public export
badForbidAttackWithPatient : Unspellable (Instruction []) (\ok =>
  Continuously (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                        (DeonticCounterpart (Macros.target Macros.creature)) {pt = ok})
               (Just ThisTurn))
badForbidAttackWithPatient Oh impossible

||| "Target creature blocks it this turn"
public export
badBlocksItself : Unspellable (Instruction []) (\ok =>
  Continuously (Macros.deontic (Macros.target Macros.creature) Require ["Block"] Agent
                               (DeonticCounterpart ((Macros.It OneOf))) {pt = ok})
               (Just ThisTurn))
badBlocksItself Oh impossible

||| "Enchanted creature can't block."
public export
okEnchantedCreatureCantBlock : Ability
okEnchantedCreatureCantBlock =
  Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                  Forbid ["Block"] Agent NoDeonticPatient)

||| "Enchanted land gets +1/+1 and can't block.": a land an effect has made a
||| creature can block [CR#205.1b], and the restriction is created even while
||| it is not one [CR#208.3a].
public export
okCoordinatedLandHostBlocks : Ability
okCoordinatedLandHostBlocks =
  Static (AndAlso Nothing [ Gets Adds (AttachHost Enchanted (TypeW Land))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Macros.deontic ((Macros.It OneOf)) Forbid ["Block"] Agent NoDeonticPatient ])

||| "This deals 4 damage to target creature. The damage can't be prevented."
public export
okTheDamageAfterDealing : Instruction []
okTheDamageAfterDealing =
  Sequentially [ DealDamage This (Lit 4) (Macros.target Macros.creature)
               , Continuously
                   (CantPrevent AnyDamage ThatDamage NoPreventionOnly) Nothing ]

||| "You gain 3 life. The damage can't be prevented."
public export
badTheDamageAfterLifeGain : Unspellable (Instruction []) (\ok =>
  Sequentially [ ChangeLife You (LifeUp (Lit 3))
               , Continuously
                   (CantPrevent AnyDamage (ThatDamage {ok}) NoPreventionOnly) Nothing ])
badTheDamageAfterLifeGain Oh impossible

public export
afterDamageDealt : Bindings
afterDamageDealt =
  instrIntro (the (Instruction [])
    (DealDamage This (Lit 4) (Macros.target Macros.creature)))

||| "This deals 4 damage to target creature. The damage can't be prevented."
public export
okTheDamageAnnounced : StaticSpec ProofsDeontic.afterDamageDealt
okTheDamageAnnounced = CantPrevent AnyDamage ThatDamage NoPreventionOnly

||| "The damage can't be prevented."
public export
badTheDamageUnannounced : Unspellable (StaticSpec []) (\ok =>
  CantPrevent AnyDamage (ThatDamage {ok}) NoPreventionOnly)
badTheDamageUnannounced Oh impossible

||| "You may cast spells as though they had flash."
public export
okObjectPremiseAtCast : StaticSpec []
okObjectPremiseAtCast =
  Deontic You Permit ["Cast"] Agent Nothing
          (DeonticCounterpart (Macros.allOf Macros.spell))
          (Just (AsThoughOf (HasKeyword (TheKeyword "Flash"))))
          (PlayRider Nothing Nothing Nothing False ItsOwnCost)

||| "This creature can attack as though it were mana of any color."
public export
badManaPremiseAtAttack : Unspellable (StaticSpec []) (\ok =>
  Deontic Macros.thisCreature Permit ["Attack"] Agent Nothing NoDeonticPatient
          (Just (AsThoughMana Nothing MatchAnyColor Nothing)) NoDeonticRider
          {at = ok})
badManaPremiseAtAttack Oh impossible

||| "This creature can't be blocked by more than one creature."
public export
okBlockBoundOnBlock : StaticSpec []
okBlockBoundOnBlock =
  Deontic Macros.thisCreature Forbid ["Block"] Patient (Just (MoreThan (Lit 1)))
          (DeonticCounterpart (Macros.allOf Macros.creature)) Nothing
          NoDeonticRider

||| "This spell can't be countered more than once."
public export
badCounterBoundTwice : Unspellable (StaticSpec []) (\ok =>
  Deontic This Forbid ["Counter"] Patient (Just (MoreThan (Lit 1))) NoDeonticPatient
          Nothing NoDeonticRider {bd = ok})
badCounterBoundTwice Oh impossible

||| "You may spend mana as though it weren't a creature."
public export
badObjectPremiseAtSpend : Unspellable (StaticSpec []) (\ok =>
  Deontic You Permit ["Spend"] Agent Nothing NoDeonticPatient
          (Just (AsThoughOf (Not Macros.creature))) NoDeonticRider {at = ok})
badObjectPremiseAtSpend Oh impossible

||| "Each opponent discards a card, if those cards are creature cards."
public export
distributedDeedReadsBackPluralUnderCondition : Instruction []
distributedDeedReadsBackPluralUnderCondition =
  OnlyIf (Macros.discard (Macros.each Opponent) (Macros.a (InZone Macros.handZ)))
         (Matches (Macros.That CardW ManyOf) Macros.creature) Nothing

||| "... sacrificed permanents can't be regenerated."
public export
distributedDeedRiderReadsBackPlural : Instruction []
distributedDeedRiderReadsBackPlural =
  CantBe (Macros.sacrifice (Macros.each Opponent) (Macros.a Macros.creature))
         "Regenerate" (Macros.TheVerbed "Sacrifice" PermanentW Attributive ManyOf)

||| "... dealt damage, they lose half their life, rounded up."
public export
enchantedPlayerDamageReadsBackAsThey : Ability
enchantedPlayerDamageReadsBackAsThey =
  Macros.triggered Whenever (IsDealtDamage AnyDamage (AttachHost Enchanted PlayerW))
                   (Macros.losesLife They (Half RoundUp (PlayerStatOf LifeTotal They)))

||| "Target creature attacks a player other than you during its
||| controller's next turn if able." No printed card on the bench.
public export
goadedAttacksOther : Instruction []
goadedAttacksOther =
  Continuously (Macros.deontic (Macros.target Macros.creature) Require ["Attack"] Agent
                        (DefendingPlayer (Macros.a (Macros.otherPlayer))))
               (Just Macros.untilYourNextTurn)

||| "as you scry" — a concurrent window with no printed card on the bench.
public export
whileScrying : Concurrent []
whileScrying = WhileDoing (VerbedEvent (Just You) "Scry" Nothing Nothing)
