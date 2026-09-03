module Experimental.ProofsD

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "an attacking creature exiled with this creature"
public export
badExiledWithAttacking : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, Macros.exiledWithThisArtifact] {zc = ok})
badExiledWithAttacking Oh impossible


||| "Draw a card."
public export
badSpellAbilityOnPermanent : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature])
       [Spell (Macros.draw You (Lit 1))] (Just (1, 1)) {tx = ok})
badSpellAbilityOnPermanent Oh impossible


||| "Creatures you control get +1/+1."
public export
badStaticOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Sorcery])
       [Static (Gets (Macros.allOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))] Nothing {tx = ok})
badStaticOnSorcery Oh impossible


||| "Flying"
public export
badKeywordOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Flying" Nothing Nothing] Nothing {tx = ok})
badKeywordOnInstant Oh impossible


||| "{T}: Draw a card."
public export
badTapSorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Tap Sorcery" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [Activated TapSymbol (Macros.draw You (Lit 1)) Nothing Nothing Nothing Nothing] Nothing {tx = ok})
badTapSorcery Oh impossible


||| a creature card printed with no power or toughness
public export
badCreatureCardNoPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) [] Nothing {bx = ok})
badCreatureCardNoPt MkCardBox impossible


||| a land card printed with "{1}"
public export
badLandWithManaCost : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Land]) [] Nothing {mc = ok})
badLandWithManaCost Oh impossible


||| "Legendary Legendary Creature"
public export
badDuplicateSupertype : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [Legendary, Legendary] (MkTypeLine [] [Creature])
       [] (Just (1, 1)) {sp = ok})
badDuplicateSupertype Oh impossible


||| "Land Creature Instant"
public export
badMixedPermanentSpellLine : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land, Creature, Instant]) [] (Just (1, 1)) {ln = ok})
badMixedPermanentSpellLine MkCardLine impossible


||| a card printed with an empty type line
public export
badCardNoTypes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] []) [] Nothing {ln = ok})
badCardNoTypes MkCardLine impossible


||| "Creature Creature"
public export
badCardDuplicateType : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) [] (MkTypeLine [] [Creature, Creature]) []
       (Just (2, 2)) {ln = ok})
badCardDuplicateType MkCardLine impossible


||| "Target creature gains a spell ability."
public export
badGainsSpellAbility : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (Spell (Macros.draw You (Lit 1))) Nothing {gr = ok})
badGainsSpellAbility Oh impossible


||| "Counter target creature."
public export
badCounterPermanent : Unspellable (Effect []) (\ok =>
  Macros.counterSpell (Macros.target Macros.creature) {ct = ok})
badCounterPermanent SpellCountered impossible


||| "Counter target creature or player."
public export
badCounterJoinedPlayer : Unspellable (Effect []) (\ok =>
  Macros.counterSpell (Macros.target Macros.anyTarget) {ct = ok})
badCounterJoinedPlayer JoinCountered impossible


||| "You may play a spell this turn."
public export
badPlayFromStack : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Deontic You Permit ["Play"] Agent Nothing
                  (DeonticCounterpart (Macros.a Macros.spell)) Nothing
                  (PlayRider Nothing Nothing Nothing False ItsOwnCost) {rd = ok})
               (Just Macros.thisTurn))
badPlayFromStack Oh impossible


||| "You may cast a land card from your graveyard this turn."
public export
badCastALand : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Deontic You Permit ["Cast"] Agent Nothing
                  (DeonticCounterpart
                     (Macros.a (And [Macros.land, InZone (Macros.graveyardOf You)])))
                  Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost)
                  {pt = ok})
               (Just Macros.thisTurn))
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
               (Just Macros.thisTurn))
badPlayFromWrongZone Oh impossible


||| "Put target creature onto the stack."
public export
badMoveToStack : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.stackZ [] {ok})
badMoveToStack BattlefieldOk impossible


||| "Put target instant card from a graveyard onto the battlefield tapped."
public export
badInstantOntoBattlefield : Unspellable (Effect []) (\ok =>
  Macros.putOntoBattlefieldTapped (Macros.target (And [HasType Instant, InZone Macros.graveyardZ])) {pl = ok})
badInstantOntoBattlefield Oh impossible


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
  Triggered Whenever (StatusEvent (Macros.a Permanent) Unflipped {at = ok}) [] Nothing [] Nothing Nothing Nothing (Macros.draw You (Lit 1)))
badUnflipEvent Oh impossible


public export
turnedFaceDownHeader : Ability
turnedFaceDownHeader =
  Triggered Whenever (StatusEvent (Macros.a Permanent) FaceDown)
            [] Nothing [] Nothing Nothing Nothing (Macros.draw You (Lit 1))




public export
badUntapNextGraveyard : Unspellable (Effect []) (\ok =>
  DoesntUntapNext (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) (Lit 1) {ok = ok})
badUntapNextGraveyard OnField impossible


public export
badUntapNextAmbiguousIt : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                SetStatus Tapped (Macros.target Macros.artifact),
                DoesntUntapNext (It {ok = ok}) (Lit 1)])
badUntapNextAmbiguousIt Refl impossible




||| "Exile target creature tapped."
public export
badExileTapped : Unspellable (Effect []) (\ok =>
  Enact "Exile" (Move (Macros.target Macros.creature) Macros.exileZ
                      [EntersTapped] {rf = ok}))
badExileTapped Oh impossible


||| "Turn target creature card in your graveyard face down."
public export
badTurnFaceDownGraveyard : Unspellable (Effect []) (\ok =>
  SetStatus FaceDown (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok = ok})
badTurnFaceDownGraveyard OnField impossible


||| "Target creature card in your hand phases out."
public export
badPhasesOutInHand : Unspellable (Effect []) (\ok =>
  SetStatus PhasedOut (Macros.target (And [Macros.creature, InZone (Macros.handOf You)])) {ok = ok})
badPhasesOutInHand OnField impossible


||| "Remove target creature card in your graveyard from combat."
public export
badRemoveFromCombatGraveyard : Unspellable (Effect []) (\ok =>
  RemoveFromCombat (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok = ok})
badRemoveFromCombatGraveyard OnField impossible


||| "target creature blocking target creature card in your graveyard"
public export
badBlockingGraveyardRelatum : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature,
                      CombatRel BlockerOf (Macros.target (And [Macros.creature,
                                                              InZone (Macros.graveyardOf You)])) {ok = ok}]))
badBlockingGraveyardRelatum Oh impossible


public export
badUntapCapGraveyardSet : Unspellable Ability (\ok =>
  Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1
                              (And [Macros.creature, InZone (Macros.graveyardOf You)]) {pt = ok}))
badUntapCapGraveyardSet Oh impossible


||| "Put a poison counter on target creature."
public export
badPutPoisonOnCreature : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind Poison) (Macros.target Macros.creature) {sc = ok})
badPutPoisonOnCreature Oh impossible


||| "You get a +1/+1 counter."
public export
badGetsBoostCounter : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) You {sc = ok})
badGetsBoostCounter Oh impossible


||| "Each opponent loses all +1/+1 counters."
public export
badLosesAllBoostCounters : Unspellable (Effect []) (\ok =>
  LosesCounters (Macros.each Opponent) (Just Macros.plusOnePlusOne) Nothing
                   {pk = KindNamed {sc = ok}})
badLosesAllBoostCounters Refl impossible


||| "the number of +1/+1 counters you have"
public export
badCountersHeldByPlayer : Unspellable (Amount []) (\ok =>
  CountersOn Macros.plusOnePlusOne You {sc = ok})
badCountersHeldByPlayer Refl impossible


||| "When the last poison counter is removed from this creature, draw a card."
public export
badLastPoisonCounterRemoved : Unspellable Ability (\ok =>
  Triggered When (LastCounterRemoved Poison Macros.thisCreature Nothing {sc = ok}) [] Nothing [] Nothing Nothing Nothing (Macros.draw You (Lit 1)))
badLastPoisonCounterRemoved Refl impossible


public export
badExileCheckOnSortedSelf : Unspellable Ability (\ok =>
  Triggered When (LastCounterRemoved Time Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Matches Macros.thisCreature (InZone Macros.exileZ)
                                         {zc = ok})) (Macros.draw You (Lit 1)))
badExileCheckOnSortedSelf Oh impossible


||| "When this creature enters, if you died this turn, draw a card."
public export
badLookbackPlayerDied : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Happened Death You Lookback.ThisTurn Nothing {sb = ok})) (Macros.draw You (Lit 1)))
badLookbackPlayerDied MkLookbackSubject impossible


public export
badLookbackObjectCast : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Happened SpellCast (Macros.a Macros.creature)
                                          Lookback.ThisTurn Nothing {sb = ok})) (Macros.draw You (Lit 1)))
badLookbackObjectCast MkLookbackSubject impossible


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


||| "target colorless white creature"
public export
badColorlessWhite : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, IsColorless, ColorIs White] {cf = ok}))
badColorlessWhite Oh impossible


||| "{2}: Draw a card. Activate only during that turn's end step."
public export
badThatTurnsPartWindow : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) (Macros.draw You (Lit 1)) (Just (DuringPart EndStep (Just ThatTurns) {wk = ok})) Nothing Nothing Nothing)
badThatTurnsPartWindow Oh impossible


||| "{2}: Draw a card. Activate only before that turn's attackers are declared."
public export
badThatTurnsAttackWindow : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) (Macros.draw You (Lit 1)) (Just (BeforePoint AttackersDeclared (Just ThatTurns) {pk = ok})) Nothing Nothing Nothing)
badThatTurnsAttackWindow Oh impossible


||| "Target land attacks each combat if able."
public export
badMustAttackLand : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.target Macros.land) Require ["Attack"] Agent NoDeonticPatient {dp = ok})
               (Just Macros.thisTurn))
badMustAttackLand Oh impossible


||| "target creature card in your graveyard that is your Ring-bearer"
public export
badRingBearerInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, HasDesignation RingBearer,
       InZone (Macros.graveyardOf You)] {zc = ok})
badRingBearerInGraveyard Oh impossible


public export
badThatCreatureIsSelf : Unspellable Ability (\ok =>
  Triggered Whenever (Attacks Macros.thisCreature NoDefender) [] Nothing [] Nothing Nothing Nothing
            (Macros.gets (That (TypeW Creature) {ok = ok}) (PtUp (Lit 2)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn)))
badThatCreatureIsSelf Refl impossible


||| "This creature can't attack target creature this turn."
public export
badForbidAttackWithPatient : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                        (DeonticCounterpart (Macros.target Macros.creature)) {pt = ok})
               (Just Macros.thisTurn))
badForbidAttackWithPatient Oh impossible


||| "Target creature blocks it this turn"
public export
badBlocksItself : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.target Macros.creature) Require ["Block"] Agent
                               (DeonticCounterpart It) {pt = ok})
               (Just Macros.thisTurn))
badBlocksItself Oh impossible


||| "As long as this creature is attacking, that creature gets +2/+0."
public export
badThatCreatureIsCondSubject : Unspellable Ability (\ok =>
  Static (Macros.asLongAs (Matches Macros.thisCreature Attacking)
                          (Gets (That (TypeW Creature) {ok = ok}) (PtUp (Lit 2)) (PtUp (Lit 0)))))
badThatCreatureIsCondSubject Refl impossible


||| "Equipped land gets +1/+1."
public export
badEquippedLand : Unspellable Ability (\ok =>
  Static (Gets (AttachHost Equipped (TypeW Land) {ok = ok}) (PtUp (Lit 1)) (PtUp (Lit 1))))
badEquippedLand Oh impossible


||| "Fortified creature gets +1/+1."
public export
badFortifiedCreature : Unspellable Ability (\ok =>
  Static (Gets (AttachHost Fortified (TypeW Creature) {ok = ok}) (PtUp (Lit 1)) (PtUp (Lit 1))))
badFortifiedCreature Oh impossible


||| "Target player can't lose the game."
public export
badTargetedOutcomeGate : Unspellable Ability (\ok =>
  Static (Macros.playerCant "LoseGame" (Macros.target AnyPlayer)) {ut = ok})
badTargetedOutcomeGate Oh impossible


||| "This creature deals 2 damage to your opponents."
public export
badPluralPlayerDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage Macros.thisCreature (Lit 2) (PlayerGroup YourOpponents) {pm = ok})
badPluralPlayerDamageRecipient Oh impossible


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


public export
badDoubleStaticRider : Unspellable (StaticEffect []) (\ok =>
  AndAlso [ Gets Macros.thisCreature (PtUp (LetterVal X)) (PtUp (Lit 0))
          , Define X (Macros.countOf Macros.creatureYouControl)
          , Define X (Macros.countOf Macros.creature) {ok} ])
badDoubleStaticRider Oh impossible


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


||| "Creatures you control cost {1} less to cast."
public export
badCostSubjectOnBattlefield : Unspellable (StaticEffect []) (\ok =>
  CostsToCast (Macros.allOf Macros.creatureYouControl) (CostLess (Lit 1) Nothing) {cs = ok})
badCostSubjectOnBattlefield MkCostSubject impossible




||| "until the beginning of each player's next upkeep"
public export
badDurationEndEachPlayers : Unspellable DurationEnd (\ok =>
  StartOf Upkeep (Just EachPlayers) {dp = ok})
badDurationEndEachPlayers Oh impossible


||| "until the end of that turn's combat"
public export
badDurationEndThatTurns : Unspellable DurationEnd (\ok =>
  EndOf Combat (Just ThatTurns) {dp = ok})
badDurationEndThatTurns Oh impossible
