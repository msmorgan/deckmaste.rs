module Experimental.Proofs.Turn

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Sacrifice a creature."
public export
okSacrificeBattlefield : Instruction []
okSacrificeBattlefield = Macros.sacrifice You (Macros.a Macros.creature)

||| "Destroy target creature. At the beginning of the end step, sacrifice it."
public export
badStale : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
               Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (Macros.sacrifice You ((Macros.It)) {ok})])
badStale Oh impossible

public export
badDelayedOther : Unspellable (Instruction []) (\ok =>
  Sequentially [DealDamage This (Lit 2) (Macros.target Macros.anyTarget),
               Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (DealDamage This (Lit 1) (Macros.target (Macros.anyOtherTarget {ok})))])
badDelayedOther Refl impossible

public export
badStaleCarrier : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.exile (Macros.target Macros.creatureYouControl),
               Move (Macros.That (TypeW Creature) {ok}) Macros.battlefieldZ []])
badStaleCarrier Refl impossible

||| "Sacrifice a creature: Draw a card."
public export
okActivatedCostAndEffect : Ability
okActivatedCostAndEffect =
  Activated (Do (Macros.sacrifice You (Macros.a Macros.creature)))
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing

||| "Return a creature to its owner's hand: Tap it."
public export
badHiddenCost : Unspellable Ability (\ok =>
  Activated (Do (Move (Macros.a Macros.creature) Macros.handZ []))
            (SetStatus Tapped ((Macros.It) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}) Nothing Nothing Nothing Nothing)
badHiddenCost (Refl, _) impossible

||| "Discard a card, Sacrifice a creature: Exile it."
public export
badTwoCostMentions : Unspellable Ability (\ok =>
  Activated (Compound [Do ((Macros.discard You (Macros.a (InZone Macros.handZ)))),
                       Do (Macros.sacrifice You (Macros.a Macros.creature))])
            (Macros.exile ((Macros.It) {ok})) Nothing Nothing Nothing Nothing)
badTwoCostMentions Refl impossible

||| "Tap target creature you control. Sacrifice it."
public export
okSacrificeOnBattlefield : Instruction []
okSacrificeOnBattlefield =
  Sequentially [ SetStatus Tapped (Macros.target Macros.creatureYouControl)
               , Macros.sacrifice You (Macros.It) ]

||| "Exile target creature. Sacrifice it."
public export
badSacrificeExiled : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.exile (Macros.target Macros.creature),
               Macros.sacrifice You ((Macros.It)) {ok}])
badSacrificeExiled Oh impossible

public export
badDeadCreatureRead : Unspellable (Instruction []) (\ok =>
  Delayed (Dies (Macros.target Macros.creature)) [] (Just ThisTurn)
          (Move (Macros.That (TypeW Creature) {ok}) Macros.battlefieldZ []))
badDeadCreatureRead Refl impossible

||| "Discard a card: Return the discarded card to the battlefield."
public export
okVerbedDiscardedCard : Ability
okVerbedDiscardedCard =
  Activated (Do (Macros.discard You (Macros.a (InZone Macros.handZ))))
            (Move (Macros.TheVerbed "Discard" CardW Attributive OneOf)
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
            (Sequentially [Macros.exile (Macros.target Macros.creature),
                           Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing (Move (Macros.That CardW {ok}) Macros.battlefieldZ [])]) Nothing Nothing Nothing Nothing)
badBareCardRead Refl impossible

||| "Tap target creature."
public export
okTapOnBattlefield : Instruction []
okTapOnBattlefield = SetStatus Tapped (Macros.target Macros.creature)

||| "Tap target creature card in your graveyard."
public export
badTapGraveyard : Unspellable (Instruction []) (\ok =>
  SetStatus Tapped (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badTapGraveyard Oh impossible

||| "At the beginning of your upkeep, draw a card."
public export
okTriggerAtYourUpkeep : Ability
okTriggerAtYourUpkeep =
  Triggered At (BeginningOf ThePart Upkeep (ByPlayer You)) [] Nothing []
            Nothing Nothing Nothing (Draw You (Lit 1))

||| "At the beginning of your turn, draw a card."
public export
badTriggerAtYourTurn : Unspellable Ability (\ok =>
  Triggered At (BeginningOf ThePart Turn (ByPlayer You) {pu = ok}) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badTriggerAtYourTurn Oh impossible

||| "Whenever a creature dies, tap it."
public export
badTriggerTapsDeadCreature : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.a Macros.creature)) [] Nothing [] Nothing Nothing Nothing (SetStatus Tapped ((Macros.It)) {ok}))
badTriggerTapsDeadCreature Oh impossible

||| "Whenever a creature leaves the battlefield, tap it."
public export
badLeavesThenTap : Unspellable Ability (\ok =>
  Triggered Whenever (Macros.leavesBattlefield (Macros.a Macros.creature)) [] Nothing [] Nothing Nothing Nothing (SetStatus Tapped ((Macros.It)) {ok}))
badLeavesThenTap Oh impossible

||| "Target creature gets +3/+3 until end of turn."
public export
okUntilEndOfTurnDuration : Instruction []
okUntilEndOfTurnDuration =
  Macros.gets (Macros.target Macros.creature) (Up (Lit 3)) (Up (Lit 3))
              (Just Macros.untilEndOfTurn)

||| "Target creature gets +3/+3 until the beginning of your next upkeep."
public export
badUntilBeginningOfUpkeep : Unspellable (Instruction []) (\ok =>
  Macros.gets (Macros.target Macros.creature) (Up (Lit 3)) (Up (Lit 3)) (Just (UntilEvent (BeginningOf ThePart Upkeep (ByPlayer You)))) {sp = ok})
badUntilBeginningOfUpkeep (Present {ok = Oh}) impossible

||| "Sacrifice a creature. When you do, draw a card."
public export
okReflexiveOnSacrifice : Instruction []
okReflexiveOnSacrifice =
  Reflexively (Macros.sacrifice You (Macros.a Macros.creature))
              (Draw You (Lit 1))

||| "This creature deals 3 damage to any target. When you do, draw a card."
public export
badReflexiveOnSourceDeed : Unspellable (Instruction []) (\ok =>
  Reflexively (DealDamage This (Lit 3) (Macros.target Macros.anyTarget)) (Draw You (Lit 1)) {en = ok})
badReflexiveOnSourceDeed Oh impossible

||| "You gain 2 life. When you do, draw a card."
public export
badReflexiveOnLifeGain : Unspellable (Instruction []) (\ok =>
  Reflexively (Macros.gainsLife You (Lit 2)) (Draw You (Lit 1)) {en = ok})
badReflexiveOnLifeGain Oh impossible

||| "Draw a card, then sacrifice a creature. When you do, draw a card."
public export
badReflexiveOnSequence : Unspellable (Instruction []) (\ok =>
  Reflexively (Sequentially [(Draw You (Lit 1)), Macros.sacrifice You (Macros.a Macros.creature)]) (Draw You (Lit 1)) {en = ok})
badReflexiveOnSequence Oh impossible

public export
badReflexiveOnDelayed : Unspellable (Instruction []) (\ok =>
  Reflexively (Delayed (BeginningOf ThePart EndStep (ByPlayer You)) [] Nothing (Draw You (Lit 1))) (Draw You (Lit 1)) {en = ok})
badReflexiveOnDelayed Oh impossible

||| "Regenerate this creature. If it regenerates this way, draw a card."
public export
okThisWayOnRegenerate : Instruction []
okThisWayOnRegenerate =
  ThisWay (Regenerate Macros.thisCreature) (Regenerates Macros.thisCreature)
          (Draw You (Lit 1))

public export
badThisWayOnDelayed : Unspellable (Instruction []) (\ok =>
  ThisWay (Delayed (BeginningOf ThePart EndStep (ByPlayer You)) [] Nothing (Draw You (Lit 1)))
          (Draws You) (Draw You (Lit 1)) {oc = ok})
badThisWayOnDelayed Oh impossible

public export
badReflexiveOnBranchedMay : Unspellable (Instruction []) (\ok =>
  Reflexively ((May You (Macros.sacrifice You (Macros.a Macros.creature)) (Just (Draw You (Lit 1))) Nothing)) (Draw You (Lit 1)) {en = ok})
badReflexiveOnBranchedMay Oh impossible

public export
badAfterReflexiveReadsTrigger : Unspellable (Instruction []) (\ok =>
  Sequentially [Reflexively (Macros.mills You (Lit 4) You)
                            (Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [White]
                                                     (MkTypeLine [] [Creature] [creatureType "Soldier"])
                                                     [] Nothing)),
                SetStatus Tapped (Macros.That (TypeW Creature) {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badAfterReflexiveReadsTrigger (Refl, _) impossible

||| "Sacrifice a creature. When you do, tap it."
public export
badReflexiveTapsSacrificed : Unspellable (Instruction []) (\ok =>
  Reflexively (Macros.sacrifice You (Macros.a Macros.creature)) (SetStatus Tapped ((Macros.It)) {ok}))
badReflexiveTapsSacrificed Oh impossible

||| "target creature blocking this creature"
public export
okBlockingThisCreature : Noun [] Object
okBlockingThisCreature =
  Macros.target (And [Macros.creature,
                      CombatRel BlockerOf Macros.thisCreature])

||| "target creature blocking target creature card in your graveyard"
public export
badBlockingGraveyardRelatum : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature,
                      CombatRel BlockerOf (Macros.target (And [Macros.creature,
                                                              InZone (Macros.graveyardOf You)])) {ok = ok}]))
badBlockingGraveyardRelatum Oh impossible

||| "Activate only during each player's end step."
public export
okDistributivePartWindow : Ability
okDistributivePartWindow =
  Activated (Mana [Macros.generic 2]) (Draw You (Lit 1))
            (Just (DuringPart EndStep (Just (Macros.each AnyPlayer))))
            Nothing Nothing Nothing

||| "{2}: Draw a card. Activate only during all players' end step." [CR#102.1]
public export
badPluralPartWindow : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) (Draw You (Lit 1)) (Just (DuringPart EndStep (Just (Macros.allOf AnyPlayer)) {wk = ok})) Nothing Nothing Nothing)
badPluralPartWindow Oh impossible

||| "until the beginning of your next upkeep"
public export
okDurationEndYourUpkeep : DurationEnd []
okDurationEndYourUpkeep = StartOf Upkeep (Just You)

||| "until the beginning of each player's next upkeep"
public export
badDurationEndEachPlayers : Unspellable (DurationEnd []) (\ok =>
  StartOf Upkeep (Just (Macros.each AnyPlayer)) {dp = ok})
badDurationEndEachPlayers Oh impossible

||| "until the end of your next combat"
public export
okDurationEndYourCombat : DurationEnd []
okDurationEndYourCombat = EndOf Combat (Just You)

||| "until the end of an opponent's combat"
public export
badDurationEndAnOpponent : Unspellable (DurationEnd []) (\ok =>
  EndOf Combat (Just Macros.anOpponent) {dp = ok})
badDurationEndAnOpponent Oh impossible

||| "You get an emblem with 'At the beginning of your end step, draw a card.'"
public export
okTriggeredEmblem : Instruction []
okTriggeredEmblem =
  GetsEmblem You
    [ Macros.triggered At (BeginningOf ThePart EndStep (ByPlayer You))
                       (Draw You (Lit 1)) ]

||| "You get an emblem with 'flying'."
public export
badKeywordEmblem : Unspellable (Instruction []) (\ok =>
  GetsEmblem You [KeywordAbility "Flying" Nothing Nothing] {ea = ok})
badKeywordEmblem Oh impossible

||| "You get an emblem."
public export
badEmptyEmblem : Unspellable (Instruction []) (\ok =>
  GetsEmblem You [] {ea = ok})
badEmptyEmblem Oh impossible

||| "… At the beginning of that turn's end step, you lose the game."
public export
okDeicticTurnAfterExtraTurn : Instruction []
okDeicticTurnAfterExtraTurn =
  Sequentially [ExtraTurn You (Lit 1),
                Delayed (BeginningOf ThePart EndStep Macros.thatTurns) [] Nothing
                        (Concludes LoseGame You)]

||| "Draw a card. At the beginning of that turn's end step, you lose the game."
public export
badDeicticTurnWithoutIntroducer : Unspellable (Instruction []) (\ok =>
  Sequentially [Draw You (Lit 1),
                Delayed (BeginningOf ThePart EndStep (Macros.thatTurns {ok})) [] Nothing
                        (Concludes LoseGame You)])
badDeicticTurnWithoutIntroducer Refl impossible

||| "After this combat phase, there is an additional upkeep step."
public export
okAdditionalUpkeep : Instruction []
okAdditionalUpkeep = AdditionalPart Nothing Upkeep (Just Combat) (Lit 1) Nothing

||| "After this combat phase, there is an additional turn."
public export
badAdditionalTurn : Unspellable (Instruction []) (\ok =>
  AdditionalPart Nothing Turn (Just Combat) (Lit 1) Nothing {ad = ok})
badAdditionalTurn Oh impossible

||| "Spells with the chosen name can't be cast."
public export
okCastSpellClass : StaticSpec [MkBinding AD (Quality CardName) OneOf QualityP]
okCastSpellClass =
  Macros.objectCant "Cast" (Macros.allOf (And [Macros.spell, Named ChosenName]))

||| "Spells with the chosen name can't be activated."
public export
badActivatedSpellClass : Unspellable
  (StaticSpec [MkBinding AD (Quality CardName) OneOf QualityP]) (\ok =>
  Macros.objectCant "Activate"
    (Macros.allOf (And [Macros.spell, Named ChosenName])) {dp = ok})
badActivatedSpellClass Oh impossible

||| "Activated abilities of artifacts can't be Nothing cast."
public export
badCastAbilityClass : Unspellable (StaticSpec []) (\ok =>
  Macros.objectCant "Cast"
    (Macros.allOf (And [AbilityHead AnyActivated, AbilityOf (Macros.allOf Macros.artifact)]))
    {dp = ok})
badCastAbilityClass Oh impossible

||| "At the beginning of your upkeep, draw a card."
public export
okSingularPartPossessor : Ability
okSingularPartPossessor =
  Triggered At (BeginningOf ThePart Upkeep (ByPlayer You)) [] Nothing []
           Nothing Nothing Nothing (Draw You (Lit 1))

||| "At the beginning of all players' upkeep, draw a card." [CR#102.1]
public export
badPluralPartPossessor : Unspellable Ability (\ok =>
  Triggered At (BeginningOf ThePart Upkeep (ByPlayer (Macros.allOf AnyPlayer)) {pu = ok}) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badPluralPartPossessor Oh impossible

||| "creature that could block each attacking creature"
public export
okCouldBlockAttacker : Predicate [] Object
okCouldBlockAttacker =
  CombatRel CouldBlock (Macros.allOf (And [Macros.creature, Attacking]))

||| "player an opponent is attacking" — the attacker of an attacked player may
||| be a player [CR#506.2].
public export
okAttackedByPlayer : Predicate [] Player
okAttackedByPlayer = CombatRel AttackedBy Macros.anOpponent

||| "player attacked by target creature card in your graveyard"
public export
badAttackedByGraveyardRelatum : Unspellable (Predicate [] Player) (\ok =>
  CombatRel AttackedBy (Macros.target (And [Macros.creature,
                                            InZone (Macros.graveyardOf You)])) {ok = ok})
badAttackedByGraveyardRelatum Oh impossible

||| "creature that could block target creature card in your graveyard"
public export
badCouldBlockGraveyardRelatum : Unspellable (Predicate [] Object) (\ok =>
  CombatRel CouldBlock (Macros.target (And [Macros.creature,
                                            InZone (Macros.graveyardOf You)])) {ok = ok})
badCouldBlockGraveyardRelatum Oh impossible

||| "Sacrifice a creature. If you do, draw a card."
public export
okIfDoneWithArm : Instruction []
okIfDoneWithArm =
  IfDone (Macros.sacrifice You (Macros.a Macros.creature))
         (Just (Draw You (Lit 1))) Nothing

||| "Sacrifice a creature."
public export
badIfDoneWithNeitherArm : Unspellable (Instruction []) (\ok =>
  IfDone (Macros.sacrifice You (Macros.a Macros.creature)) Nothing Nothing {br = ok})
badIfDoneWithNeitherArm Oh impossible

||| "This creature deals 3 damage to any target. If you do, draw a card."
public export
badIfDoneOverAgentlessBody : Unspellable (Instruction []) (\ok =>
  IfDone (DealDamage Macros.thisCreature (Lit 3) (Macros.target Macros.anyTarget))
         (Just (Draw You (Lit 1))) Nothing {en = ok})
badIfDoneOverAgentlessBody Oh impossible

||| "Take an extra turn after this one. If you do, draw a card."
public export
badIfDoneOverScheduledBody : Unspellable (Instruction []) (\ok =>
  IfDone (ExtraTurn You (Lit 1)) (Just (Draw You (Lit 1))) Nothing {en = ok})
badIfDoneOverScheduledBody Oh impossible

public export
oneExtraTurn : Bindings
oneExtraTurn = instrIntro {bs = []} (ExtraTurn You (Lit 1))

||| "Take an extra turn after this one. Skip the draw step of that turn."
public export
okThatTurnAfterASingleTurn : Noun Turn.oneExtraTurn TurnRef
okThatTurnAfterASingleTurn = Macros.thatTurn {bs = Turn.oneExtraTurn}

public export
twoExtraTurns : Bindings
twoExtraTurns =
  instrIntro {bs = instrIntro {bs = []} (ExtraTurn You (Lit 1))} (ExtraTurn You (Lit 1))

public export
badThatTurnAfterTwoTurns : Unspellable (Noun twoExtraTurns TurnRef) (\ok =>
  Macros.thatTurn {bs = twoExtraTurns} {ok})
badThatTurnAfterTwoTurns Refl impossible

||| "create a legendary 20/20 black Avatar creature token named Marit Lage"
public export
okTokenSingleSupertype : Instruction []
okTokenSingleSupertype =
  Macros.create (Lit 1)
    (MkToken (Just (Lit 20 ** Lit 20)) [Black]
       (MkTypeLine [Legendary] [Creature] [creatureType "Avatar"]) [] (Just "Marit Lage"))

||| "create a legendary legendary 20/20 black Avatar creature token"
public export
badTokenDuplicateSupertype : Unspellable (Instruction []) (\ok =>
  Macros.create (Lit 1)
    (MkToken (Just (Lit 20 ** Lit 20)) [Black]
       (MkTypeLine [Legendary, Legendary] [Creature] [creatureType "Avatar"]) [] (Just "Marit Lage"))
    {tc = ok})
badTokenDuplicateSupertype Oh impossible
