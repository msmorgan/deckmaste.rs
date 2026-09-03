module Experimental.Proofs

import Experimental
import Experimental.Macros
import public Experimental.Unspellable

%default total

%unbound_implicits off


||| "the chosen number", one number choice standing
public export
okChosenNumberOneStanding : Amount [qualityB Number]
okChosenNumberOneStanding = ChosenNumber

||| "... is equal to the chosen number", two number choices standing (Shapeshifter prints "last")
public export
badChosenNumberTwoStanding :
  Unspellable (Amount [qualityB Number, qualityB Number]) (\ok => ChosenNumber {ok})
badChosenNumberTwoStanding Refl impossible


||| "of the chosen color"
public export
okChosenColorRead :
  Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object
okChosenColorRead = OfChosen Color

||| "of the chosen number"
public export
badChosenNumberRead :
  Unspellable
    (Predicate [MkBinding AD (Quality Number) OneOf QualityP] Object)
    (\ok => OfChosen Number {read = ok})
badChosenNumberRead Oh impossible


||| "Choose target creature. You gain life equal to its power."
public export
okSinglePower : Effect []
okSinglePower =
  Sequentially [Choose (Macros.target Macros.creature) Nothing Openly,
                Macros.gainsLife You (Macros.powerOf It)]

||| "Choose two target creatures. You gain life equal to their power."
public export
badGroupPower : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (Described (TargetDet (Macros.exactly 2)) Macros.creature) Nothing Openly,
                Macros.gainsLife You (StatOf Power ((Macros.It ManyOf)) {one = ok})])
badGroupPower Refl impossible


||| "Choose target creature. Its owner loses 1 life."
public export
okSingleOwner : Effect []
okSingleOwner =
  Sequentially [Choose (Macros.target Macros.creature) Nothing Openly,
                Macros.losesLife (Macros.ownerOf It) (Lit 1)]

||| "Choose two target creatures. Their owner loses 1 life."
public export
badGroupOwner : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (Described (TargetDet (Macros.exactly 2)) Macros.creature) Nothing Openly,
                Macros.losesLife (Macros.ownerOf ((Macros.It ManyOf)) {one = ok}) (Lit 1)])
badGroupOwner Refl impossible


||| "Target creature fights target creature."
public export
okFightCreatures : Effect []
okFightCreatures =
  Fights (Macros.target Macros.creature) (Macros.target Macros.creature)

||| "Two target creatures fight target creature."
public export
badFightGroup : Unspellable (Effect []) (\ok =>
  Fights (Described (TargetDet (Macros.exactly 2)) Macros.creature) {pa = ok}
         (Macros.target Macros.creature))
badFightGroup Refl impossible


||| "a creature target opponent controls"
public export
okControlledByOne : Predicate [] Object
okControlledByOne = HasPossessor ControllerAx (Macros.target Opponent)

||| "a creature two target opponents control"
public export
badControlledByGroup : Unspellable (Predicate [] Object) (\ok =>
  HasPossessor ControllerAx (Described (TargetDet (Macros.exactly 2)) Opponent) {ps = ok})
badControlledByGroup Oh impossible


||| "each of up to two target creatures"
public export
okEachOfGroup : Noun [] Object
okEachOfGroup = EachOf (Macros.targets (Macros.upTo 2) Macros.creature)

||| "each of target creature"
public export
badEachOfSingular : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.target Macros.creature) {pl = ok})
badEachOfSingular Refl impossible


||| "Look at the top card of target player's library."
public export
okSliceOfOnePossessor : Effect []
okSliceOfOnePossessor =
  Macros.lookAt (LibrarySlice OnTop (Lit 1) (Macros.target AnyPlayer))

||| "Look at the top card of two target players' library."
public export
badSliceOfCountedPossessor : Unspellable (Effect []) (\ok =>
  Macros.lookAt (LibrarySlice OnTop (Lit 1)
                              (Described (TargetDet (Macros.exactly 2)) AnyPlayer) {sp = ok}))
badSliceOfCountedPossessor Oh impossible


||| "Whenever you cast a spell, draw a card."
public export
okCastsSingularComplement : Ability
okCastsSingularComplement =
  Triggered Whenever (Casts You (Macros.a Macros.spell) Nothing) [] Nothing []
            Nothing Nothing Nothing (Macros.draw You (Lit 1))

||| "Whenever you cast all spells, draw a card."
public export
badCastsPluralComplement : Unspellable (Ability) (\ok =>
  Triggered Whenever (Casts You (Macros.allOf Macros.spell) Nothing {one = ok}) [] Nothing [] Nothing Nothing Nothing
            (Draw You (Lit 1)))
badCastsPluralComplement Refl impossible


||| "Tap target creature an opponent controls. That player loses 1 life."
public export
okThatPlayer : Effect []
okThatPlayer =
  Sequentially [SetStatus Tapped (Macros.target (And [Macros.creature,
                  HasPossessor ControllerAx Macros.anOpponent])),
                Macros.losesLife (That PlayerW) (Lit 1)]

public export
badDisjunctAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target (Or [And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent],
                                        And [Macros.land, HasPossessor ControllerAx You]])),
                Macros.losesLife (Macros.That PlayerW OneOf {ok}) (Lit 1)])
badDisjunctAntecedent Refl impossible


||| "Tap target creature with flying."
public export
okKeywordConjunction : Effect []
okKeywordConjunction =
  SetStatus Tapped (Macros.target (And [Macros.creature,
                                        HasKeyword (TheKeyword "Flying")]))

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


||| "Snow-Covered Forest — Basic Snow Land — Forest"
public export
okSingleSnow : Card
okSingleSnow =
  Macros.card "Snow-Covered Forest" Nothing [Basic, Snow]
              (MkTypeLine [landType "Forest"] [Land]) [] Nothing

||| "Snow Snow Land — Forest"
public export
badDuplicateSnow : Unspellable Card (\ok =>
  Macros.card "" Nothing [Snow, Snow] (MkTypeLine [landType "Forest"] [Land]) [] Nothing {fl = ok})
badDuplicateSnow MkFaceLaws impossible


||| "This deals 2 damage to any target and 1 damage to any other target."
public export
okAnyOtherTarget : Effect []
okAnyOtherTarget =
  Sequentially [DealDamage This (Lit 2) (Macros.target Macros.anyTarget),
                DealDamage This (Lit 1) (Macros.target Macros.anyOtherTarget)]

||| "This deals 1 damage to any other target."
public export
badOther : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.target (Macros.anyOtherTarget {ok})))
badOther Refl impossible


||| "Tap target creature. It gets -1/-1 until end of turn."
public export
okIt : Effect []
okIt =
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                Macros.gets It (PtDown (Lit 1)) (PtDown (Lit 1))
                            (Just Macros.untilEndOfTurn)]

||| "Target creature fights target creature. Tap it."
public export
badIt : Unspellable (Effect []) (\ok =>
  Sequentially [Fights (Macros.target Macros.creature) (Macros.target Macros.creature),
                SetStatus Tapped ((Macros.It OneOf) {ok})])
badIt Refl impossible


||| "Tap target creature."
public export
okTapBattlefield : Effect []
okTapBattlefield = SetStatus Tapped (Macros.target Macros.creature)

||| "This deals 3 damage to each creature. Tap it."
public export
badTheyIt : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.each Macros.creature),
                SetStatus Tapped ((Macros.It OneOf) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badTheyIt (Refl, _) impossible


||| "Sacrifice a creature."
public export
okSacrificeBattlefield : Effect []
okSacrificeBattlefield = Macros.sacrifice You (Macros.a Macros.creature)

||| "Destroy target creature. At the beginning of the end step, sacrifice it."
public export
badStale : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
               Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (Macros.sacrifice You ((Macros.It OneOf)) {ok})])
badStale Oh impossible


public export
badDelayedOther : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 2) (Macros.target Macros.anyTarget),
               Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (DealDamage This (Lit 1) (Macros.target (Macros.anyOtherTarget {ok})))])
badDelayedOther Refl impossible


public export
badStaleCarrier : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile You (Macros.target Macros.creatureYouControl),
               Move (Macros.That (TypeW Creature) OneOf {ok}) Macros.battlefieldZ []])
badStaleCarrier Refl impossible


||| "Return a creature to its owner's hand: Tap it."
public export
badHiddenCost : Unspellable Ability (\ok =>
  Activated (Do (Move (Macros.a Macros.creature) Macros.handZ []))
            (SetStatus Tapped ((Macros.It OneOf) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}) Nothing Nothing Nothing Nothing)
badHiddenCost (Refl, _) impossible


||| "Discard a card, Sacrifice a creature: Exile it."
public export
badTwoCostMentions : Unspellable Ability (\ok =>
  Activated (Compound [Do ((Macros.discard You (Macros.a (InZone Macros.handZ)))),
                       Do (Macros.sacrifice You (Macros.a Macros.creature))])
            (Macros.exile You ((Macros.It OneOf) {ok})) Nothing Nothing Nothing Nothing)
badTwoCostMentions Refl impossible


||| "Exile target creature. Sacrifice it."
public export
badSacrificeExiled : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile You (Macros.target Macros.creature),
               Macros.sacrifice You ((Macros.It OneOf)) {ok}])
badSacrificeExiled Oh impossible


public export
badDeadCreatureRead : Unspellable (Effect []) (\ok =>
  Delayed (Dies (Macros.target Macros.creature)) [] (Just ThisTurn)
          (Move (Macros.That (TypeW Creature) OneOf {ok}) Macros.battlefieldZ []))
badDeadCreatureRead Refl impossible


||| "of the chosen creature type"
public export
okChosenCreatureType :
  Predicate [MkBinding AD (Quality (SubtypeQ Creature)) OneOf QualityP] Object
okChosenCreatureType = OfChosen (SubtypeQ Creature)

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


||| "Choose two target creatures. Tap them."
public export
okThem : Effect []
okThem =
  Sequentially [Choose (Macros.targets (Macros.exactly 2) Macros.creature)
                       Nothing Openly,
                SetStatus Tapped Them]

||| "Choose two target creatures. Choose two target creatures. Tap them."
public export
badThemAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (Described (TargetDet (Macros.exactly 2)) Macros.creature) Nothing Openly,
               Choose (Described (TargetDet (Macros.exactly 2)) Macros.creature) Nothing Openly,
               SetStatus Tapped ((Macros.It ManyOf) {ok})])
badThemAmbig Refl impossible


public export
badInnerAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [Fights (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent]))
                       (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent])),
               Macros.losesLife (Macros.That PlayerW OneOf {ok}) (Lit 1)])
badInnerAmbig Refl impossible


||| "Discard a card: Return the discarded card to the battlefield."
public export
okVerbedDiscardedCard : Ability
okVerbedDiscardedCard =
  Activated (Do (Macros.discard You (Macros.a (InZone Macros.handZ))))
            (Move (TheVerbed "Discard" CardW Attributive)
                  Macros.battlefieldZ [])
            Nothing Nothing Nothing Nothing

||| "Discard a card: Return the sacrificed card to the battlefield."
public export
badVerbedWrongVerb : Unspellable Ability (\ok =>
  Activated (Do ((Macros.discard You (Macros.a (InZone Macros.handZ)))))
            (Move (Macros.TheVerbed "Sacrifice" CardW Attributive OneOf {ok}) Macros.battlefieldZ []) Nothing Nothing Nothing Nothing)
badVerbedWrongVerb Refl impossible


||| "Sacrifice an artifact: Return the sacrificed creature to the battlefield."
public export
badVerbedWrongNoun : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice You (Macros.a (HasType Artifact))))
            (Move (Macros.TheVerbed "Sacrifice" (TypeW Creature) Attributive OneOf {ok}) Macros.battlefieldZ []) Nothing Nothing Nothing Nothing)
badVerbedWrongNoun Refl impossible


public export
badVerbedAmbig : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.sacrifice You (Macros.a Macros.creature)),
                       Do (Macros.sacrifice You (Macros.a Macros.creature))])
            (Move (Macros.TheVerbed "Sacrifice" CardW Attributive OneOf {ok}) Macros.battlefieldZ []) Nothing Nothing Nothing Nothing)
badVerbedAmbig Refl impossible


public export
badBareCardRead : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice You (Macros.a Macros.creature)))
            (Sequentially [Macros.exile You (Macros.target Macros.creature),
                           Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (Move (Macros.That CardW OneOf {ok}) Macros.battlefieldZ [])]) Nothing Nothing Nothing Nothing)
badBareCardRead Refl impossible


||| "Tap target creature card in your graveyard."
public export
badTapGraveyard : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badTapGraveyard Oh impossible


||| "Destroy target tapped creature."
public export
okTappedBattlefield : Effect []
okTappedBattlefield =
  Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped]))

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
               SetStatus Tapped (Macros.That PermanentW OneOf {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
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
               SetStatus Untapped (Macros.That TokenW OneOf {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badThatTokenOfCard (Refl, _) impossible


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


||| "When target creature dies this turn, return that card to the battlefield."
public export
okDiesBattlefield : Effect []
okDiesBattlefield =
  Delayed (Dies (Macros.target Macros.creature)) [] (Just ThisTurn)
          (Move (That CardW) Macros.battlefieldZ [])

public export
badDiesInGraveyard : Unspellable (Effect []) (\ok =>
  Delayed (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok}) [] (Just ThisTurn)
          (Move (Macros.That CardW OneOf) Macros.battlefieldZ []))
badDiesInGraveyard Oh impossible


||| "This deals 3 damage to target creature."
public export
okDamageCreature : Effect []
okDamageCreature = DealDamage This (Lit 3) (Macros.target Macros.creature)

||| "Destroy target creature. This deals 3 damage to it."
public export
badDamageGraveyardCard : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
               DealDamage This (Lit 3) ((Macros.It OneOf)) {rk = ok}])
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


||| "not of a color other than white"
public export
okNegatedRestrictedQuality : Predicate [] (Quality Color)
okNegatedRestrictedQuality =
  Not (QualityNoun Color (Just (ColorOtherThan White)))

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
                Macros.losesLife (Macros.That PlayerW OneOf {ok}) (Lit 1)])
badNegatedAntecedent Refl impossible


||| "Destroy target creature on the battlefield in a graveyard."
public export
badConflictingZones : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone Macros.battlefieldZ, InZone Macros.graveyardZ] {zc = ok})))
badConflictingZones Oh impossible


||| "Choose target creature."
public export
okTargetCreature : Effect []
okTargetCreature = Choose (Macros.target Macros.creature) Nothing Openly

||| "Choose target color."
public export
badTargetColor : Unspellable (Effect []) (\ok =>
  Choose (Macros.target (QualityNoun Color Nothing) {tk = ok}) Nothing Openly)
badTargetColor ObjectTgt impossible


||| "Target creature gets +3/+3 until end of turn."
public export
okGetsBattlefield : Effect []
okGetsBattlefield =
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 3)) (PtUp (Lit 3))
              (Just Macros.untilEndOfTurn)

||| "Destroy target creature. It gets +3/+3 until end of turn."
public export
badGetsGraveyard : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                Macros.gets ((Macros.It OneOf)) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn) {ok}])
badGetsGraveyard Oh impossible


||| "Return target creature to its owner's hand."
public export
okMoveToHand : Effect []
okMoveToHand = Move (Macros.target Macros.creature) Macros.handZ []

||| "Put target creature into target player's hand."
public export
badMoveToTargetsHand : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) (Macros.handOf (Macros.target AnyPlayer)) [] {ok})
badMoveToTargetsHand HandOkBare impossible


||| "Destroy target creature."
public export
okDestroyBattlefield : Effect []
okDestroyBattlefield = Macros.destroy (Macros.target Macros.creature)

||| "Destroy target creature card in your graveyard."
public export
badDestroyGraveyard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badDestroyGraveyard Oh impossible


||| "You discard a card."
public export
okDiscardHand : Effect []
okDiscardHand = Macros.discard You (Macros.a (InZone Macros.handZ))

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


||| "Destroy a creature."
public export
okKnownVerbLabel : Effect []
okKnownVerbLabel =
  Enact Nothing "Destroy"
        (Move (Macros.a Macros.creature) Macros.graveyardZ [])

||| A move labeled with a word outside the label catalog
public export
badUnknownVerbLabel : Unspellable (Effect []) (\ok =>
  Enact Nothing "Descry" (Move (Macros.a Macros.creature) Macros.graveyardZ []) {kn = ok})
badUnknownVerbLabel Oh impossible


||| "Choose two target creatures."
public export
okTwoGroup : Effect []
okTwoGroup =
  Choose (Macros.targets (Macros.exactly 2) Macros.creature) Nothing Openly

||| "Choose zero target creatures."
public export
badZeroGroup : Unspellable (Effect []) (\ok =>
  Choose (Described (TargetDet (Macros.exactly 0)) Macros.creature {ok}) Nothing Openly)
badZeroGroup (MaxAtLeastOne, _, _) impossible


||| "your hand"
public export
okOwnedHand : ZoneExpr []
okOwnedHand = ZoneAt Hand (PossessedBy You)

||| "your battlefield"
public export
badOwnedBattlefield : Unspellable (ZoneExpr []) (\ok =>
  ZoneAt Battlefield (PossessedBy You {ps = ok}))
badOwnedBattlefield HandIsOwned impossible


public export
badDiscardedCreatureWord : Unspellable Ability (\ok =>
  Activated (Do (Macros.discard You (Macros.aAtRandom (And [Macros.creature, InZone Macros.handZ]))))
            (DealDamage This
                          (StatOf ManaValue (Macros.TheVerbed "Discard" (TypeW Creature) Attributive OneOf {ok}))
                          (Macros.target Macros.anyTarget)) Nothing Nothing Nothing Nothing)
badDiscardedCreatureWord Refl impossible


||| "Target player sacrifices a creature of their choice."
public export
okBoundTheirChoice : Effect []
okBoundTheirChoice =
  Macros.sacrifice (Macros.target AnyPlayer)
                   (Macros.aTheirChoice Macros.creature)

||| "Destroy a creature of their choice."
public export
badUnboundTheirChoice : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.aTheirChoice Macros.creature {ch = ok}))
badUnboundTheirChoice Refl impossible


||| "... loses 1 life for each attacking creature. You gain that much life."
public export
okThatMuchBound : Effect []
okThatMuchBound =
  Sequentially [Macros.losesLife (Macros.target Opponent)
                  (Macros.forEach (And [Attacking, Macros.creature,
                                        HasPossessor ControllerAx You])),
                Macros.gainsLife You ThatMuch]

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


||| "1 life for each creature"
public export
okForEachOne : Amount []
okForEachOne = Macros.nForEach 1 Macros.creature

||| "1 life for each 0 creatures"
public export
badForEachZero : Unspellable (Amount []) (\ok =>
  Macros.forEach 0 Macros.creature {nz = ok})
badForEachZero Oh impossible


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
okAscribedSelf : Noun [] Object
okAscribedSelf = AsType Creature This Nothing

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
            (Macros.gets (Macros.That (TypeW Creature) OneOf {ok = Builtin.fst ok}) (PtDown (Lit 1))
                         (PtDown (Lit 1)) {ok = Builtin.snd ok} (Just Macros.untilEndOfTurn)))
badAltHeaderMixedReadback (Refl, _) impossible


public export
badThreeArmHeaderReadback : Unspellable Ability (\ok =>
  Triggered Whenever (Macros.attacks Macros.thisCreature)
            [ Blocks Macros.thisCreature Nothing
            , BecomesTarget Macros.thisCreature (Macros.a Macros.spell) ]
            Nothing [] Nothing Nothing Nothing
            (DealDamage Macros.thisCreature (StatOf Power ((Macros.It OneOf) {ok = ok}))
                        (Macros.each Opponent)))
badThreeArmHeaderReadback Refl impossible


public export
badJoinedHeaderReadback : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing
            [ Macros.joinedHead Whenever
                (Dies (Macros.a Macros.creatureYouControl)) ]
            Nothing Nothing Nothing
            (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                         ((Macros.It OneOf) {ok = ok})))
badJoinedHeaderReadback Refl impossible


||| "Whenever this creature attacks while you're casting a spell, draw a card."
public export
okWhileDoingCast : Ability
okWhileDoingCast =
  Triggered Whenever (Macros.attacks Macros.thisCreature) []
            (Just (WhileDoing (Casts You (Macros.a Macros.spell) Nothing)))
            [] Nothing Nothing Nothing
            (Macros.draw You (Lit 1))

||| "Whenever this creature attacks while a creature is dying, draw a card."
public export
badWhileDoingMoment : Unspellable Ability (\ok =>
  Triggered Whenever (Macros.attacks Macros.thisCreature) []
            (Just (WhileDoing (Dies (Macros.a Macros.creature)) {up = ok}))
            [] Nothing Nothing Nothing
            (Draw You (Lit 1)))
badWhileDoingMoment Oh impossible
