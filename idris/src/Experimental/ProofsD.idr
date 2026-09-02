||| Unspellable pins, continued from Experimental.ProofsC.
module Experimental.ProofsD

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "an attacking creature exiled with this creature"
||| The status word seeds the battlefield, where the linked cards are in exile [CR#607.2a].
public export
badExiledWithAttacking : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, Macros.exiledWithThisArtifact] {zc = ok})
badExiledWithAttacking Oh impossible


||| "Draw a card." printed as a spell ability on a creature card
||| [CR#113.3a] defines the category by resolving as an instant or sorcery spell.
public export
badSpellAbilityOnPermanent : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature])
       [Spell Macros.drawACard] (Just (1, 1)) {tx = ok})
badSpellAbilityOnPermanent Oh impossible


||| "Creatures you control get +1/+1." printed as a static ability on a sorcery
||| [CR#113.3a] admits one only if it fits [CR#113.6]; this row needs a battlefield continuous effect.
public export
badStaticOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Sorcery])
       [Static (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))] Nothing {tx = ok})
badStaticOnSorcery Oh impossible


||| "Flying" printed as a bare line on an instant card
||| These keywords are [CR#702] abilities of a permanent in combat, which no spell card is.
public export
badKeywordOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Flying" Nothing] Nothing {tx = ok})
badKeywordOnInstant Oh impossible


||| "{T}: Draw a card." printed on a sorcery card
||| [CR#113.6j] needs a cost payable off the battlefield, and "{T}" taps a permanent [CR#107.5,110.4].
public export
badTapSorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Tap Sorcery" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [Activated TapSymbol Macros.drawACard Nothing Nothing Nothing Nothing] Nothing {tx = ok})
badTapSorcery Oh impossible


||| a creature card printed with no power or toughness
||| A creature card writes its two numbers [CR#208.1].
public export
badCreatureCardNoPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) [] Nothing {bx = ok})
badCreatureCardNoPt MkCardBox impossible


||| a land card printed with "{1}"
||| A land card writes no mana cost [CR#202.1b]; the absence is an unpayable cost [CR#118.6].
public export
badLandWithManaCost : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Land]) [] Nothing {mc = ok})
badLandWithManaCost Oh impossible


||| "Legendary Legendary Creature"
||| A supertype is a property an object has or lacks [CR#205.4b], so the word twice is one fact twice.
public export
badDuplicateSupertype : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [Legendary, Legendary] (MkTypeLine [] [Creature])
       [] (Just (1, 1)) {sp = ok})
badDuplicateSupertype Oh impossible


||| "Land Creature Instant"
||| Instants and sorceries can't enter the battlefield [CR#110.4]; [CR#110.4a] lists the six that can.
public export
badMixedPermanentSpellLine : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land, Creature, Instant]) [] (Just (1, 1)) {ln = ok})
badMixedPermanentSpellLine MkCardLine impossible


||| a card printed with an empty type line
||| [CR#205.1] has the type line contain the card's card type(s) unconditionally.
public export
badCardNoTypes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] []) [] Nothing {ln = ok})
badCardNoTypes MkCardLine impossible


||| "Creature Creature"
||| A type line names each of the card's types once; a type written twice is a word written twice.
public export
badCardDuplicateType : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) [] (MkTypeLine [] [Creature, Creature]) []
       (Just (2, 2)) {ln = ok})
badCardDuplicateType MkCardLine impossible


||| "Target creature gains a spell ability."
||| A category error: this clause grants to a permanent, and a spell ability is a resolving spell's [CR#113.3a].
public export
badGainsSpellAbility : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (Spell Macros.drawACard) Nothing {gr = ok})
badGainsSpellAbility Oh impossible


||| "Counter target creature."
||| The countering's complement is a spell [CR#112.1]; a battlefield permanent has already resolved.
public export
badCounterPermanent : Unspellable (Effect []) (\ok =>
  Macros.counterSpell (Macros.target Macros.creature) {ct = ok})
badCounterPermanent SpellCountered impossible


||| "Counter target creature or player."
||| [CR#701.6a] counters by removing from the stack, and [CR#109.1]'s list of what an object is names no player, so nothing a union reaching a player denotes is ever countered.
public export
badCounterJoinedPlayer : Unspellable (Effect []) (\ok =>
  Macros.counterSpell (Macros.target Macros.anyTarget) {ct = ok})
badCounterJoinedPlayer JoinCountered impossible


||| "You may play a spell this turn." of an object on the stack
||| [CR#112.1] makes an object on the stack a spell, and a spell has already been cast.
public export
badPlayFromStack : Unspellable (Effect []) (\ok =>
  Continuously (Deontic You Permit ["Play"] Agent
                  (DeonticCounterpart (Macros.a Macros.spell)) Nothing
                  (PlayRider Nothing Nothing Nothing False ItsOwnCost) {rd = ok})
               (Just Macros.thisTurn))
badPlayFromStack Oh impossible


||| "You may cast a land card from your graveyard this turn."
||| A land card "can be played only as a land. It can't be cast as a spell" [CR#305.9].
public export
badCastALand : Unspellable (Effect []) (\ok =>
  Continuously (Deontic You Permit ["Cast"] Agent
                  (DeonticCounterpart
                     (Macros.a (And [Macros.land, InZone (Macros.graveyardOf You)])))
                  Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost)
                  {pt = ok})
               (Just Macros.thisTurn))
badCastALand Oh impossible


||| "You may play a creature card in exile from your graveyard this turn."
||| A written source phrase must agree with the zone the complement already names.
public export
badPlayFromWrongZone : Unspellable (Effect []) (\ok =>
  Continuously (Deontic You Permit ["Play"] Agent
                  (DeonticCounterpart
                     (Macros.a (And [Macros.creature, InZone Macros.exileZ])))
                  Nothing
                  (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing
                             False ItsOwnCost)
                  {rd = ok})
               (Just Macros.thisTurn))
badPlayFromWrongZone Oh impossible


||| "Put target creature onto the stack."
||| [CR#601.2a] puts a card on the stack as part of casting, which is no placement.
public export
badMoveToStack : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.stackZ (MkMoveRiders [] Nothing Nothing) {ok})
badMoveToStack BattlefieldOk impossible


||| "Put target instant card from a graveyard onto the battlefield tapped."
||| Instant and sorcery cards can't enter the battlefield [CR#110.4]; [CR#110.4a] lists the six that can.
public export
badInstantOntoBattlefield : Unspellable (Effect []) (\ok =>
  Macros.putOntoBattlefieldTapped (Macros.target (And [HasType Instant, InZone Macros.graveyardZ])) {pl = ok})
badInstantOntoBattlefield Oh impossible


||| "unless" written over a positive condition
||| "Unless" is the negation, so a positive condition under the word negates twice.
public export
badUnlessOnPositive : Unspellable Ability (\ok =>
  Static (Conditionally (Exists (And [Macros.artifact, ControlledBy You]))
                        (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent NoDeonticPatient) Unless {mk = ok}))
badUnlessOnPositive MkMarkingOk impossible


||| a trigger header watching a permanent become unflipped
||| Flipping is one-way [CR#710.4], so there is no transition to observe.
public export
badUnflipEvent : Unspellable Ability (\ok =>
  Triggered Whenever (StatusEvent (Macros.a Permanent) Unflipped {at = ok}) [] Nothing [] Nothing Nothing Nothing Macros.drawACard)
badUnflipEvent Oh impossible


||| a trigger header watching a permanent be turned face down
||| The header's own table refuses it. "Turned face up" heads 132
||| supported occurrences; "turned face down" occurs once in the whole
||| supported corpus and that once is a DURATION ENDPOINT (Vesuvan
||| Shapeshifter), which reads the same transition through
||| `statusEventOk` and is unaffected by this refusal. The split is what
||| lets the endpoint write while the header stays at its measured zero.
public export
badTurnedFaceDownHeader : Unspellable Ability (\ok =>
  Triggered Whenever (StatusEvent (Macros.a Permanent) FaceDown) [] Nothing [] Nothing Nothing Nothing Macros.drawACard {hs = ok})
badTurnedFaceDownHeader Oh impossible




||| "Target creature card in your graveyard doesn't untap during its controller's next untap step."
||| The timed clause's subject stands on the battlefield [CR#701.26a].
public export
badUntapNextGraveyard : Unspellable (Effect []) (\ok =>
  DoesntUntapNext (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) (Lit 1) {ok = ok})
badUntapNextGraveyard OnField impossible


||| "Tap target creature. Tap target artifact. It doesn't untap during its controller's next untap step."
||| "It" after two singular object introductions reaches two mentions and resolves neither.
public export
badUntapNextAmbiguousIt : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                SetStatus Tapped (Macros.target Macros.artifact),
                DoesntUntapNext (It {ok = ok}) (Lit 1)])
badUntapNextAmbiguousIt Refl impossible




||| "Exile target creature tapped."
||| Status words are the battlefield's alone [CR#110.5,110.5b]; an exile writes the counter rider only.
public export
badExileTapped : Unspellable (Effect []) (\ok =>
  Enact "Exile" (Move (Macros.target Macros.creature) Macros.exileZ
                      (MkMoveRiders [EntersTapped] Nothing Nothing) {rf = ok}))
badExileTapped Oh impossible


||| "Turn target creature card in your graveyard face down."
||| [CR#110.5d]: only permanents have status, so a graveyard card is neither face up nor face down.
public export
badTurnFaceDownGraveyard : Unspellable (Effect []) (\ok =>
  SetStatus FaceDown (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok = ok})
badTurnFaceDownGraveyard OnField impossible


||| "Target creature card in your hand phases out."
||| Phasing starts on the battlefield and changes no zone [CR#702.26d]; a hand card has no status [CR#110.5].
public export
badPhasesOutInHand : Unspellable (Effect []) (\ok =>
  SetStatus PhasedOut (Macros.target (And [Macros.creature, InZone (Macros.handOf You)])) {ok = ok})
badPhasesOutInHand OnField impossible


||| "Remove target creature card in your graveyard from combat."
||| Combat is the battlefield's: [CR#506.4] lists how a permanent leaves it.
public export
badRemoveFromCombatGraveyard : Unspellable (Effect []) (\ok =>
  RemoveFromCombat (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok = ok})
badRemoveFromCombatGraveyard OnField impossible


||| "target creature blocking target creature card in your graveyard"
||| The relation holds between two creatures in combat [CR#509.1g], so the relatum names no other zone.
public export
badBlockingGraveyardRelatum : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature,
                      BlockerOf (Macros.target (And [Macros.creature,
                                                     InZone (Macros.graveyardOf You)])) {zn = ok}]))
badBlockingGraveyardRelatum Oh impossible


||| "Players can't untap more than one creature card in your graveyard during their untap steps."
||| [CR#502.3] untaps the permanents a player controls, which a graveyard phrase contradicts.
public export
badUntapCapGraveyardSet : Unspellable Ability (\ok =>
  Static (CantMoreThan (PlayerGroup AllPlayers) "Untap" 1
                         (And [Macros.creature, InZone (Macros.graveyardOf You)]) {zn = ok}))
badUntapCapGraveyardSet Oh impossible


||| "Put a poison counter on target creature."
||| [CR#122.1] puts a counter on an object or a player and the two never cross; poison is a player's.
public export
badPutPoisonOnCreature : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind Poison) (Macros.target Macros.creature) {sc = ok})
badPutPoisonOnCreature Oh impossible


||| "You get a +1/+1 counter."
||| The same table read the other way: a stat counter is an object's [CR#122.1a].
public export
badGetsBoostCounter : Unspellable (Effect []) (\ok =>
  GetsCounters You (Lit 1) Macros.plusOnePlusOne {sc = ok})
badGetsBoostCounter Refl impossible


||| "Each opponent loses all +1/+1 counters."
||| A named kind on the player's removal verb must be a player's kind [CR#122.1].
public export
badLosesAllBoostCounters : Unspellable (Effect []) (\ok =>
  LosesCounters (Each Opponent) (Just Macros.plusOnePlusOne) Nothing
                   {pk = KindNamed {sc = ok}})
badLosesAllBoostCounters Refl impossible


||| "the number of +1/+1 counters you have"
||| The read agrees with its holder's sort: a stat counter sits on an object [CR#122.1a].
public export
badCountersHeldByPlayer : Unspellable (Amount []) (\ok =>
  CountersOn Macros.plusOnePlusOne You {sc = ok})
badCountersHeldByPlayer Refl impossible


||| "When the last poison counter is removed from this creature, draw a card."
||| The last-removal event watches an object's holding, and poison is a player's kind [CR#122.1].
public export
badLastPoisonCounterRemoved : Unspellable Ability (\ok =>
  Triggered When (LastCounterRemoved Poison Macros.thisCreature Nothing {sc = ok}) [] Nothing [] Nothing Nothing Nothing Macros.drawACard)
badLastPoisonCounterRemoved Refl impossible


||| "When the last time counter is removed from this card, if this creature is exiled, draw a card."
||| The sorted self-word seeds the battlefield [CR#109.2], so it cannot be asked whether it is elsewhere.
public export
badExileCheckOnSortedSelf : Unspellable Ability (\ok =>
  Triggered When (LastCounterRemoved Time Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Matches Macros.thisCreature (InZone Macros.exileZ)
                                         {zc = ok})) Macros.drawACard)
badExileCheckOnSortedSelf Oh impossible


||| "When this creature enters, if you died this turn, draw a card."
||| Each event names the sort its history subject takes, and dying is an object's.
public export
badLookbackPlayerDied : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Happened Death You Lookback.ThisTurn Nothing {sb = ok})) Macros.drawACard)
badLookbackPlayerDied MkLookbackSubject impossible


||| "When this creature enters, if a creature cast a spell this turn, draw a card."
||| The same table the other way: casting is read over a player [CR#601.2].
public export
badLookbackObjectCast : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Happened SpellCast (Macros.a Macros.creature)
                                          Lookback.ThisTurn Nothing {sb = ok})) Macros.drawACard)
badLookbackObjectCast MkLookbackSubject impossible


||| "target creature who cast a spell this turn"
||| The head noun is the history read's subject, and casting is a player's event [CR#601.2].
public export
badHappenedToObjectCast : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, HappenedTo SpellCast Lookback.ThisTurn Nothing {sb = ok}]))
badHappenedToObjectCast MkLookbackSubject impossible


||| "each opponent who died this turn"
||| The same table at its second reader: dying is an object's event, so a player head cannot ask it.
public export
badHappenedToPlayerDied : Unspellable (Noun [] Player) (\ok =>
  Each (And [Opponent, HappenedTo Death Lookback.ThisTurn Nothing {sb = ok}]))
badHappenedToPlayerDied MkLookbackSubject impossible


||| "each opponent who a state matched this turn"
||| [CR#603.8]'s state trigger fires when a game state is true "rather than triggering when an event occurs", so nothing happened and no participant of it can be looked back on afterwards -- at either kind.
public export
badStateMatchLookback : Unspellable (Noun [] Player) (\ok =>
  Each (And [Opponent, HappenedTo StateMatch Lookback.ThisTurn Nothing {sb = ok}]))
badStateMatchLookback MkLookbackSubject impossible


||| "target colorless white creature"
||| [CR#105.2c] gives a colorless object no color, so the two words describe nothing together.
public export
badColorlessWhite : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, IsColorless, ColorIs White] {cf = ok}))
badColorlessWhite Oh impossible


||| "{2}: Draw a card. Activate only during that turn's end step."
||| An activation restriction introduces no turn, so the deictic possessor reaches no antecedent.
public export
badThatTurnsPartWindow : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) Macros.drawACard (Just (DuringPart EndStep (Just ThatTurns) {wk = ok})) Nothing Nothing Nothing)
badThatTurnsPartWindow Oh impossible


||| "{2}: Draw a card. Activate only before that turn's attackers are declared."
||| An activation restriction introduces no turn, so the deictic possessor reaches no antecedent.
public export
badThatTurnsAttackWindow : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) Macros.drawACard (Just (BeforePoint AttackersDeclared (Just ThatTurns) {pk = ok})) Nothing Nothing Nothing)
badThatTurnsAttackWindow Oh impossible


||| "Target land attacks each combat if able."
||| Only a creature attacks [CR#506.3], and the requirement reads the restriction's own grid.
public export
badMustAttackLand : Unspellable (Effect []) (\ok =>
  Continuously (Macros.deontic (Macros.target Macros.land) Require ["Attack"] Agent NoDeonticPatient {dp = ok})
               (Just Macros.thisTurn))
badMustAttackLand Participant impossible


||| "target creature card in your graveyard that is your Ring-bearer"
||| [CR#701.54e]'s first conjunct is the battlefield, which a graveyard clause contradicts.
public export
badRingBearerInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, HasDesignation RingBearer,
       InZone (Macros.graveyardOf You)] {zc = ok})
badRingBearerInGraveyard Oh impossible


||| "Whenever this creature attacks, that creature gets +2/+0 until end of turn."
||| English's demonstratives skip the speaker: "that creature" never picks out the trigger's own subject.
public export
badThatCreatureIsSelf : Unspellable Ability (\ok =>
  Triggered Whenever (Attacks Macros.thisCreature NoDefender) [] Nothing [] Nothing Nothing Nothing
            (Macros.gets (That (TypeW Creature) {ok = ok}) (PtUp (Lit 2)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn)))
badThatCreatureIsSelf Refl impossible


||| "This creature can't attack target creature this turn." — and its
||| requiring twin, "this creature attacks target creature each combat
||| if able": one cell, since only a player, a planeswalker or a battle
||| is attacked [CR#506.3].
public export
badForbidAttackWithPatient : Unspellable (Effect []) (\ok =>
  Continuously (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                        (DeonticCounterpart (Macros.target Macros.creature)) {pt = ok})
               (Just Macros.thisTurn))
badForbidAttackWithPatient Oh impossible


||| "Target creature blocks it this turn" -- with the pronoun resolving
||| to the very creature the statement made block.
||| A creature never blocks itself: [CR#509.1a] has the defending player
||| choose the blockers from among the creatures they control and, for
||| each, "one creature for it to block that's attacking that player",
||| while [CR#508.1a] has the active player choose the attackers from
||| among the creatures THEY control. The two participants are always
||| under different controllers, so no game state satisfies the
||| statement. A bare `It` is exactly that statement here: the
||| counterpart is typed at the subject's output and the subject made the
||| only Object announcement the pronoun could read. `ItOtherThan` is the
||| positive path and is what `mustBlockIt` writes.
public export
badBlocksItself : Unspellable (Effect []) (\ok =>
  Continuously (Macros.deontic (Macros.target Macros.creature) Require ["Block"] Agent
                               (DeonticCounterpart It) {pt = ok})
               (Just Macros.thisTurn))
badBlocksItself Oh impossible


||| "As long as this creature is attacking, that creature gets +2/+0."
||| The demonstrative screen at the second minting site: it never picks out the sentence's own subject.
public export
badThatCreatureIsCondSubject : Unspellable Ability (\ok =>
  Static (Macros.asLongAs (Matches Macros.thisCreature Attacking)
                          (Gets (That (TypeW Creature) {ok = ok}) (PtUp (Lit 2)) (PtUp (Lit 0)))))
badThatCreatureIsCondSubject Refl impossible


||| "Equipped land gets +1/+1."
||| [CR#301.5a] names one host for an Equipment and it is a creature.
public export
badEquippedLand : Unspellable Ability (\ok =>
  Static (Gets (AttachHost Equipped (TypeW Land) {ok = ok}) (PtUp (Lit 1)) (PtUp (Lit 1))))
badEquippedLand Oh impossible


||| "Fortified creature gets +1/+1."
||| The mirror: [CR#301.6] applies the Equipment rules to Fortifications in relation to lands.
public export
badFortifiedCreature : Unspellable Ability (\ok =>
  Static (Gets (AttachHost Fortified (TypeW Creature) {ok = ok}) (PtUp (Lit 1)) (PtUp (Lit 1))))
badFortifiedCreature Oh impossible


||| "Target player can't lose the game."
||| The gate is a static line, and a static ability announces no target [CR#115.1d].
public export
badTargetedOutcomeGate : Unspellable Ability (\ok =>
  Static (Macros.playerCant "LoseGame" (Macros.target AnyPlayer)) {ut = ok})
badTargetedOutcomeGate Oh impossible


||| "This creature deals 2 damage to your opponents."
||| One magnitude and one recipient phrase, and a bare plural says neither each member's nor the group's.
public export
badPluralPlayerDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage Macros.thisCreature (Lit 2) (PlayerGroup YourOpponents) {pm = ok})
badPluralPlayerDamageRecipient Oh impossible


||| "You gain life equal to your opponents' life totals."
||| One player has one life total, so the read is singular [CR#119.1].
public export
badPluralLifeTotalRead : Unspellable (Amount []) (\ok =>
  PlayerStatOf LifeTotal (PlayerGroup YourOpponents) {one = ok})
badPluralLifeTotalRead Refl impossible


||| "You gain life equal to each player's life total."
||| The distributive names no one player, so there is no total to read.
public export
badDistributiveLifeTotalRead : Unspellable (Amount []) (\ok =>
  PlayerStatOf LifeTotal (Each AnyPlayer) {one = ok})
badDistributiveLifeTotalRead Refl impossible


||| "Draw cards equal to the difference."
||| The margin is read off a comparison, and a sentence that made none has no difference to name.
public export
badUnlicensedDifference : Unspellable (Effect []) (\ok =>
  Draw You (TheDifference {ok}))
badUnlicensedDifference Refl impossible


||| "When this creature enters, if you control a creature, draw cards equal to the difference."
||| A condition that holds by no amount leaves no margin; only a comparison licenses one.
public export
badNonComparisonDifference : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Exists Macros.creatureYouControl))
            (Draw You (TheDifference {ok})))
badNonComparisonDifference Refl impossible


||| "Draw X cards, where X is the number of creatures you control, where X is the number of creatures on the battlefield."
||| One statement settles every instance of X [CR#107.3i]; the second
||| definition finds none open to define.
public export
badDoubleXRider : Unspellable (Effect []) (\ok =>
  Sequentially [ Draw You (LetterVal X)
               , Define X (CountOf Macros.creatureYouControl)
               , Define X (CountOf Macros.creature) {ok} ])
badDoubleXRider Oh impossible


||| "Draw Y cards, where X is the number of creatures you control."
||| Y is a name of its own and follows X's rules, not X's definition [CR#107.3p].
public export
badUnlicensedY : Unspellable (Effect []) (\ok =>
  Sequentially [ Draw You (LetterVal Y)
               , Define X (CountOf Macros.creatureYouControl) {ok} ])
badUnlicensedY Oh impossible


||| "This creature gets +X/+0, where X is the number of creatures you control, where X is the number of creatures on the battlefield."
||| The static twin of `badDoubleXRider`, and the same rule [CR#107.3i].
public export
badDoubleStaticRider : Unspellable (StaticEffect []) (\ok =>
  AndAlso [ Gets Macros.thisCreature (PtUp (LetterVal X)) (PtUp (Lit 0))
          , Define X (CountOf Macros.creatureYouControl)
          , Define X (CountOf Macros.creature) {ok} ])
badDoubleStaticRider Oh impossible


||| "the greatest power among players"
||| Power is a creature's own number [CR#208.1], so the fold's axis and domain must agree in sort.
public export
badPowerAmongPlayers : Unspellable (Amount []) (\ok =>
  Aggregate MaxOf (CharAxis Power) AnyPlayer {sc = ok})
badPowerAmongPlayers Refl impossible


||| "the highest life total among creatures you control"
||| The same table the other way: a life total is a player's [CR#119.1].
public export
badLifeTotalAmongObjects : Unspellable (Amount []) (\ok =>
  Aggregate MaxOf (PlayerStatAxis LifeTotal) Macros.creatureYouControl {sc = ok})
badLifeTotalAmongObjects Refl impossible


||| "Creatures you control cost {1} less to cast."
||| A cost statement is about a spell [CR#609.2], and a battlefield noun is not one.
public export
badCostSubjectOnBattlefield : Unspellable (StaticEffect []) (\ok =>
  CostsToCast (AllOf Macros.creatureYouControl) (CostLess (Lit 1) Nothing) {cs = ok})
badCostSubjectOnBattlefield MkCostSubject impossible




||| "until the beginning of each player's next upkeep"
||| A duration ends at one moment, and [CR#500.1] runs every phase and step
||| on every turn, so a quantifier possessor names several ends and no end.
public export
badDurationEndEachPlayers : Unspellable DurationEnd (\ok =>
  StartOf Upkeep (Just EachPlayers) {dp = ok})
badDurationEndEachPlayers Oh impossible


||| "until the end of that turn's combat"
||| The deictic possessor names a TURN, not whose turn part the end falls in.
public export
badDurationEndThatTurns : Unspellable DurationEnd (\ok =>
  EndOf Combat (Just ThatTurns) {dp = ok})
badDurationEndThatTurns Oh impossible
