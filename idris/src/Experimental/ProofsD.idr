module Experimental.ProofsD

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "an attacking creature"
public export
okAttackingCreature : Predicate [] Object
okAttackingCreature = And [Attacking, Macros.creature]


||| "an attacking creature exiled with this creature"
public export
badExiledWithAttacking : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, Macros.exiledWithThisArtifact] {zc = ok})
badExiledWithAttacking Oh impossible


||| "Draw a card." printed on an instant
public export
okSpellAbilityOnInstant : Card
okSpellAbilityOnInstant =
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [Spell (Macros.draw You (Lit 1))] Nothing


||| "Draw a card."
public export
badSpellAbilityOnPermanent : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature])
       [Spell (Draw You (Lit 1))] (Just (1, 1)) {fl = ok})
badSpellAbilityOnPermanent MkFaceLaws impossible


||| "Creatures you control get +1/+1."
public export
badStaticOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Sorcery])
       [Static (Gets Adds (Macros.allOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))] Nothing {fl = ok})
badStaticOnSorcery MkFaceLaws impossible


||| "Flying"
public export
badKeywordOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Flying" Nothing Nothing] Nothing {fl = ok})
badKeywordOnInstant MkFaceLaws impossible


||| "{T}: Draw a card."
public export
badTapSorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Tap Sorcery" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [Activated TapSymbol (Draw You (Lit 1)) Nothing Nothing Nothing Nothing] Nothing {fl = ok})
badTapSorcery MkFaceLaws impossible


||| a creature card printed with no power or toughness
public export
badCreatureCardNoPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) [] Nothing {fl = ok})
badCreatureCardNoPt (MkFaceLaws {bx = MkCardBox}) impossible


||| a land card printed with "{1}"
public export
badLandWithManaCost : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Land]) [] Nothing {fl = ok})
badLandWithManaCost MkFaceLaws impossible


||| "Legendary Legendary Creature"
public export
badDuplicateSupertype : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [Legendary, Legendary] (MkTypeLine [] [Creature])
       [] (Just (1, 1)) {fl = ok})
badDuplicateSupertype MkFaceLaws impossible


||| "Land Creature Instant"
public export
badMixedPermanentSpellLine : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land, Creature, Instant]) [] (Just (1, 1)) {fl = ok})
badMixedPermanentSpellLine (MkFaceLaws {ln = MkCardLine}) impossible


||| a card printed with an empty type line
public export
badCardNoTypes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] []) [] Nothing {fl = ok})
badCardNoTypes (MkFaceLaws {ln = MkCardLine}) impossible


||| "Creature Creature"
public export
badCardDuplicateType : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) [] (MkTypeLine [] [Creature, Creature]) []
       (Just (2, 2)) {fl = ok})
badCardDuplicateType (MkFaceLaws {ln = MkCardLine}) impossible


||| "Target creature gains flying."
public export
okGainsKeyword : Effect []
okGainsKeyword =
  Macros.gains (Macros.target Macros.creature) (Macros.keyword "Flying") Nothing


||| "Target creature gains a spell ability."
public export
badGainsSpellAbility : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (Spell (Draw You (Lit 1))) Nothing {gr = ok})
badGainsSpellAbility Oh impossible


||| "Counter target spell."
public export
okCounterSpell : Effect []
okCounterSpell = Macros.counterSpell (Macros.target Macros.spell)


||| "Counter target creature."
public export
badCounterPermanent : Unspellable (Effect []) (\ok =>
  CounterSpell (Macros.target Macros.creature) {ct = ok})
badCounterPermanent StackSpell impossible


||| "Counter target creature or player."
public export
badCounterJoinedPlayer : Unspellable (Effect []) (\ok =>
  CounterSpell (Macros.target Macros.anyTarget) {ct = ok})
badCounterJoinedPlayer StackJoin impossible


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
    (Just Macros.thisTurn)


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
    (Just Macros.thisTurn)


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


||| "Exile target creature."
public export
okMoveToExile : Effect []
okMoveToExile = Move (Macros.target Macros.creature) Macros.exileZ []


||| "Put target creature onto the stack."
public export
badMoveToStack : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) (ZoneAt Stack Bare) [] {ok})
badMoveToStack BattlefieldOk impossible


||| "Put target creature card from a graveyard onto the battlefield tapped."
public export
okCreatureOntoBattlefieldTapped : Effect []
okCreatureOntoBattlefieldTapped =
  Macros.putOntoBattlefieldTapped
    (Macros.target (And [Macros.creature, InZone Macros.graveyardZ]))


||| "Put target instant card from a graveyard onto the battlefield tapped."
public export
badInstantOntoBattlefield : Unspellable (Effect []) (\ok =>
  Macros.putOntoBattlefieldTapped (Macros.target (And [HasType Instant, InZone Macros.graveyardZ])) {pl = ok})
badInstantOntoBattlefield Oh impossible


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


public export
turnedFaceDownHeader : Ability
turnedFaceDownHeader =
  Triggered Whenever (StatusEvent (Macros.a Permanent) FaceDown)
            [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1))




||| "Target creature doesn't untap during its controller's next untap step."
public export
okDoesntUntapNextOnBattlefield : Effect []
okDoesntUntapNextOnBattlefield =
  DoesntUntapNext (Macros.target Macros.creature) (Lit 1)


public export
badUntapNextGraveyard : Unspellable (Effect []) (\ok =>
  DoesntUntapNext (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) (Lit 1) {ok = ok})
badUntapNextGraveyard Oh impossible


||| "Tap target creature. It doesn't untap during its controller's next
||| untap step."
public export
okUntapNextSingleIt : Effect []
okUntapNextSingleIt =
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                DoesntUntapNext It (Lit 1)]


public export
badUntapNextAmbiguousIt : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                SetStatus Tapped (Macros.target Macros.artifact),
                DoesntUntapNext ((Macros.It OneOf) {ok = ok}) (Lit 1)])
badUntapNextAmbiguousIt Refl impossible




||| "Exile target creature with a +1/+1 counter on it."
public export
okExileWithCounterRider : Effect []
okExileWithCounterRider =
  Enact Nothing "Exile"
        (Move (Macros.target Macros.creature) Macros.exileZ
              [WithCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Fresh])


||| "Exile target creature tapped."
public export
badExileTapped : Unspellable (Effect []) (\ok =>
  Enact Nothing "Exile" (Move (Macros.target Macros.creature) Macros.exileZ
                      [EntersTapped] {rf = ok}))
badExileTapped Oh impossible


||| "Turn target creature face down."
public export
okTurnFaceDownOnBattlefield : Effect []
okTurnFaceDownOnBattlefield =
  SetStatus FaceDown (Macros.target Macros.creature)


||| "Turn target creature card in your graveyard face down."
public export
badTurnFaceDownGraveyard : Unspellable (Effect []) (\ok =>
  SetStatus FaceDown (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok = ok})
badTurnFaceDownGraveyard Oh impossible


||| "Target creature card in your hand phases out."
public export
badPhasesOutInHand : Unspellable (Effect []) (\ok =>
  SetStatus PhasedOut (Macros.target (And [Macros.creature, InZone (Macros.handOf You)])) {ok = ok})
badPhasesOutInHand Oh impossible


||| "Remove target creature from combat."
public export
okRemoveFromCombatOnBattlefield : Effect []
okRemoveFromCombatOnBattlefield =
  RemoveFromCombat (Macros.target Macros.creature)


||| "Remove target creature card in your graveyard from combat."
public export
badRemoveFromCombatGraveyard : Unspellable (Effect []) (\ok =>
  RemoveFromCombat (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok = ok})
badRemoveFromCombatGraveyard Oh impossible


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


||| "Players can't untap more than one creature during each untap step."
public export
okUntapCapBattlefieldSet : Ability
okUntapCapBattlefieldSet =
  Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1
                              Macros.creature)


public export
badUntapCapGraveyardSet : Unspellable Ability (\ok =>
  Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1
                              (And [Macros.creature, InZone (Macros.graveyardOf You)]) {pt = ok}))
badUntapCapGraveyardSet Oh impossible


||| "Put a +1/+1 counter on target creature."
public export
okPutBoostCounterOnCreature : Effect []
okPutBoostCounterOnCreature =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
              (Macros.target Macros.creature)


||| "Put a poison counter on target creature."
public export
badPutPoisonOnCreature : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind (Named "Poison")) (Macros.target Macros.creature) {sc = ok})
badPutPoisonOnCreature Oh impossible


||| "You get a +1/+1 counter."
public export
badGetsBoostCounter : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) You {sc = ok})
badGetsBoostCounter Oh impossible


||| "Each opponent loses all poison counters."
public export
okLosesAllPoisonCounters : Effect []
okLosesAllPoisonCounters =
  LosesCounters (Macros.each Opponent) (Just (PrintedKind (Named "Poison")))
                Nothing


||| "Each opponent loses all +1/+1 counters."
public export
badLosesAllBoostCounters : Unspellable (Effect []) (\ok =>
  LosesCounters (Macros.each Opponent) (Just (PrintedKind Macros.plusOnePlusOne)) Nothing
                   {sc = ok})
badLosesAllBoostCounters Oh impossible


||| "the number of +1/+1 counters on this creature"
public export
okCountersHeldByObject : Amount []
okCountersHeldByObject = CountersOn Macros.plusOnePlusOne Macros.thisCreature


||| "the number of +1/+1 counters you have"
public export
badCountersHeldByPlayer : Unspellable (Amount []) (\ok =>
  CountersOn Macros.plusOnePlusOne You {sc = ok})
badCountersHeldByPlayer Refl impossible


||| "When the last +1/+1 counter is removed from this creature, draw a card."
public export
okLastBoostCounterRemoved : Ability
okLastBoostCounterRemoved =
  Triggered When
            (Macros.lastCounterRemoved Macros.plusOnePlusOne
                                       Macros.thisCreature)
            [] Nothing [] Nothing Nothing Nothing (Macros.draw You (Lit 1))


||| "When the last poison counter is removed from this creature, draw a card."
public export
badLastPoisonCounterRemoved : Unspellable Ability (\ok =>
  Triggered When (Macros.lastCounterRemoved (Named "Poison") Macros.thisCreature {sc = ok}) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badLastPoisonCounterRemoved Refl impossible


||| "if this creature is attacking"
public export
okMatchesBattlefieldZone : Ability
okMatchesBattlefieldZone =
  Triggered When (Macros.lastCounterRemoved (Named "Time") Macros.thisCreature)
            [] Nothing [] Nothing Nothing
            (Just (Matches Macros.thisCreature Attacking))
            (Macros.draw You (Lit 1))


public export
badExileCheckOnSortedSelf : Unspellable Ability (\ok =>
  Triggered When (Macros.lastCounterRemoved (Named "Time") Macros.thisCreature) [] Nothing [] Nothing Nothing (Just (Matches Macros.thisCreature (InZone Macros.exileZ)
                                         {zc = ok})) (Draw You (Lit 1)))
badExileCheckOnSortedSelf Oh impossible


||| "When this creature enters, if a creature died this turn, draw a card."
public export
okLookbackObjectDied : Ability
okLookbackObjectDied =
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing []
            Nothing Nothing
            (Just (Happened Death (Macros.a Macros.creature) Lookback.ThisTurn
                            Nothing))
            (Macros.draw You (Lit 1))


||| "When this creature enters, if you died this turn, draw a card."
public export
badLookbackPlayerDied : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Happened Death You Lookback.ThisTurn Nothing {sb = ok})) (Draw You (Lit 1)))
badLookbackPlayerDied MkLookbackSubject impossible


public export
badLookbackObjectCast : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Happened SpellCast (Macros.a Macros.creature)
                                          Lookback.ThisTurn Nothing {sb = ok})) (Draw You (Lit 1)))
badLookbackObjectCast MkLookbackSubject impossible


||| "target creature that entered this turn"
public export
okHappenedToObjectEntry : Noun [] Object
okHappenedToObjectEntry =
  Macros.target (And [Macros.creature,
                      HappenedTo Entry Lookback.ThisTurn Nothing])


||| "target creature who cast a spell this turn"
public export
badHappenedToObjectCast : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, HappenedTo SpellCast Lookback.ThisTurn Nothing {sb = ok}]))
badHappenedToObjectCast MkLookbackSubject impossible


||| "each opponent who died this turn"
public export
badHappenedToPlayerDied : Unspellable (Noun [] Player) (\ok =>
  Macros.each (And [Opponent, HappenedTo Death Lookback.ThisTurn Nothing {sb = ok}]))
badHappenedToPlayerDied MkLookbackSubject impossible


||| "each opponent who a state matched this turn"
public export
badStateMatchLookback : Unspellable (Noun [] Player) (\ok =>
  Macros.each (And [Opponent, HappenedTo StateMatch Lookback.ThisTurn Nothing {sb = ok}]))
badStateMatchLookback MkLookbackSubject impossible


||| "target white creature"
public export
okWhiteCreature : Noun [] Object
okWhiteCreature = Macros.target (And [Macros.creature, ColorIs White])


||| "target colorless white creature"
public export
badColorlessWhite : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, IsColorless, ColorIs White] {cf = ok}))
badColorlessWhite Oh impossible


||| "Activate only during each player's end step."
public export
okDistributivePartWindow : Ability
okDistributivePartWindow =
  Activated (Mana [Macros.generic 2]) (Macros.draw You (Lit 1))
            (Just (DuringPart EndStep (Just (Macros.each AnyPlayer))))
            Nothing Nothing Nothing


||| "{2}: Draw a card. Activate only during all players' end step." [CR#102.1]
public export
badPluralPartWindow : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) (Draw You (Lit 1)) (Just (DuringPart EndStep (Just (Macros.allOf AnyPlayer)) {wk = ok})) Nothing Nothing Nothing)
badPluralPartWindow Oh impossible


||| "Activate only before each player's attackers are declared."
public export
okDistributiveAttackWindow : Ability
okDistributiveAttackWindow =
  Activated (Mana [Macros.generic 2]) (Macros.draw You (Lit 1))
            (Just (BeforeAttackersDeclared (Just (Macros.each AnyPlayer))))
            Nothing Nothing Nothing


||| "{2}: Draw a card. Activate only before all players' attackers are declared." [CR#102.1]
public export
badPluralAttackWindow : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) (Draw You (Lit 1)) (Just (BeforeAttackersDeclared (Just (Macros.allOf AnyPlayer)) {pk = ok})) Nothing Nothing Nothing)
badPluralAttackWindow Oh impossible


||| "Target creature attacks each combat if able."
public export
okMustAttackCreature : Effect []
okMustAttackCreature =
  Continuously {ts = StaticFirstDone}
               (Macros.deontic (Macros.target Macros.creature) Require
                               ["Attack"] Agent NoDeonticPatient)
               (Just Macros.thisTurn)


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


||| "Whenever a creature attacks, that creature gets +2/+0 until end of turn."
public export
okThatCreatureAfterAttack : Ability
okThatCreatureAfterAttack =
  Triggered Whenever (Attacks (Macros.a Macros.creature) NoDefender)
            [] Nothing [] Nothing Nothing Nothing
            (Macros.gets (That (TypeW Creature)) (PtUp (Lit 2)) (PtUp (Lit 0))
                         (Just Macros.untilEndOfTurn))


public export
badThatCreatureIsSelf : Unspellable Ability (\ok =>
  Triggered Whenever (Attacks Macros.thisCreature NoDefender) [] Nothing [] Nothing Nothing Nothing
            (Macros.gets (Macros.That (TypeW Creature) OneOf {ok = Builtin.fst ok}) (PtUp (Lit 2)) (PtUp (Lit 0))
                         {ok = Builtin.snd ok} (Just Macros.untilEndOfTurn)))
badThatCreatureIsSelf (Refl, _) impossible


||| "This creature can't attack target planeswalker this turn."
public export
okForbidAttackPlaneswalker : Effect []
okForbidAttackPlaneswalker =
  Continuously {ts = StaticFirstDone}
               (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                        (DeonticCounterpart
                           (Macros.target (HasType Planeswalker))))
               (Just Macros.thisTurn)


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


||| "As long as this creature is attacking, this creature gets +2/+0."
public export
okGetsBattlefieldSubject : Ability
okGetsBattlefieldSubject =
  Static (Macros.asLongAs (Matches Macros.thisCreature Attacking)
                          (Gets Adds Macros.thisCreature (PtUp (Lit 2))
                                (PtUp (Lit 0))))


||| "As long as this creature is attacking, that creature gets +2/+0."
public export
badThatCreatureIsCondSubject : Unspellable Ability (\ok =>
  Static (Macros.asLongAs (Matches Macros.thisCreature Attacking)
                          (Gets Adds (Macros.That (TypeW Creature) OneOf {ok = Builtin.fst ok}) (PtUp (Lit 2)) (PtUp (Lit 0))
                                {ok = Builtin.snd ok})))
badThatCreatureIsCondSubject (Refl, _) impossible


||| "Equipped creature gets +1/+1."
public export
okEquippedCreature : Ability
okEquippedCreature =
  Static (Gets Adds (AttachHost Equipped (TypeW Creature)) (PtUp (Lit 1))
               (PtUp (Lit 1)))


||| "Equipped land gets +1/+1."
public export
badEquippedLand : Unspellable Ability (\ok =>
  Static (Gets Adds (AttachHost Equipped (TypeW Land) {ok = ok}) (PtUp (Lit 1)) (PtUp (Lit 1))))
badEquippedLand Oh impossible


||| "Fortified creature gets +1/+1."
public export
badFortifiedCreature : Unspellable Ability (\ok =>
  Static (Gets Adds (AttachHost Fortified (TypeW Creature) {ok = ok}) (PtUp (Lit 1)) (PtUp (Lit 1))))
badFortifiedCreature Oh impossible


||| "You can't lose the game."
public export
okUntargetedOutcomeGate : Ability
okUntargetedOutcomeGate = Static (Macros.playerCant "LoseGame" You)


||| "Target player can't lose the game."
||| Refused as a Static; Continuously (playerCant …) spells the sentence.
public export
badTargetedOutcomeGate : Unspellable Ability (\ok =>
  Static (Macros.playerCant "LoseGame" (Macros.target AnyPlayer)) {ut = ok})
badTargetedOutcomeGate Oh impossible


||| "This creature deals 2 damage to each opponent."
public export
okEachOpponentDamage : Effect []
okEachOpponentDamage =
  DealDamage Macros.thisCreature (Lit 2) (Macros.each Opponent)


||| "This creature deals 2 damage to your opponents."
public export
badPluralPlayerDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage Macros.thisCreature (Lit 2) (PlayerGroup YourOpponents) {pm = ok})
badPluralPlayerDamageRecipient Oh impossible


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


||| "Draw cards equal to the difference." under a comparison
public export
okDifferenceAfterComparison : Effect []
okDifferenceAfterComparison =
  If (CompareAmt (Macros.countOf (InZone (Macros.handOf You))) Less (Lit 7))
     (Draw You TheDifference) Nothing


||| "Draw cards equal to the difference."
public export
badUnlicensedDifference : Unspellable (Effect []) (\ok =>
  Draw You (TheDifference {ok}))
badUnlicensedDifference Refl impossible


public export
badNonComparisonDifference : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Macros.exists Macros.creatureYouControl))
            (Draw You (TheDifference {ok})))
badNonComparisonDifference Refl impossible


||| "Draw X cards, where X is the number of creatures you control."
public export
okSingleXRider : Effect []
okSingleXRider =
  Sequentially [ Draw You (LetterVal X)
               , Define X (Macros.countOf Macros.creatureYouControl) ]


public export
badDoubleXRider : Unspellable (Effect []) (\ok =>
  Sequentially [ Draw You (LetterVal X)
               , Define X (Macros.countOf Macros.creatureYouControl)
               , Define X (Macros.countOf Macros.creature) {ok} ])
badDoubleXRider Oh impossible


||| "Draw Y cards, where X is the number of creatures you control."
public export
badUnlicensedY : Unspellable (Effect []) (\ok =>
  Sequentially [ Draw You (LetterVal Y)
               , Define X (Macros.countOf Macros.creatureYouControl) {ok} ])
badUnlicensedY Oh impossible


||| "This creature gets +X/+0, where X is the number of creatures you
||| control."
public export
okSingleStaticXRider : StaticEffect []
okSingleStaticXRider =
  AndAlso Nothing [ Gets Adds Macros.thisCreature (PtUp (LetterVal X))
                         (PtUp (Lit 0))
                  , Define X (Macros.countOf Macros.creatureYouControl) ]


public export
badDoubleStaticRider : Unspellable (StaticEffect []) (\ok =>
  AndAlso Nothing [ Gets Adds Macros.thisCreature (PtUp (LetterVal X)) (PtUp (Lit 0))
          , Define X (Macros.countOf Macros.creatureYouControl)
          , Define X (Macros.countOf Macros.creature) {ok} ])
badDoubleStaticRider Oh impossible


||| "the greatest power among creatures"
public export
okPowerAmongObjects : Amount []
okPowerAmongObjects = Macros.aggregate MaxOf (CharAxis Power) Macros.creature


||| "the greatest power among players"
public export
badPowerAmongPlayers : Unspellable (Amount []) (\ok =>
  Macros.aggregate MaxOf (CharAxis Power) AnyPlayer {sc = ok})
badPowerAmongPlayers Refl impossible


||| "the highest life total among creatures you control"
public export
badLifeTotalAmongObjects : Unspellable (Amount []) (\ok =>
  Macros.aggregate MaxOf (PlayerStatAxis LifeTotal) Macros.creatureYouControl {sc = ok})
badLifeTotalAmongObjects Refl impossible


||| "Creature spells you cast cost {1} less to cast."
public export
okCostSubjectOnStack : StaticEffect []
okCostSubjectOnStack =
  CostsToCast (Macros.allOf (And [Macros.creature, Macros.spell]))
              (CostLess (Lit 1) Nothing)


||| "Creatures you control cost {1} less to cast."
public export
badCostSubjectOnBattlefield : Unspellable (StaticEffect []) (\ok =>
  CostsToCast (Macros.allOf Macros.creatureYouControl) (CostLess (Lit 1) Nothing) {cs = ok})
badCostSubjectOnBattlefield MkCostSubject impossible




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
