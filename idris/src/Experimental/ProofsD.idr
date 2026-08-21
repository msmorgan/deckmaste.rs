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


||| "not exiled with this artifact"
||| The linkage read does not negate: a source-keyed group is named to be acted on.
public export
badNegatedExiledWith : Unspellable (Predicate [] Object) (\ok =>
  Not Macros.exiledWithThisArtifact {ng = ok})
badNegatedExiledWith Oh impossible


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
       [KeywordAbility Flying] Nothing {tx = ok})
badKeywordOnInstant Oh impossible


||| "{T}: Draw a card." printed on a sorcery card
||| [CR#113.6j] needs a cost payable off the battlefield, and "{T}" taps a permanent [CR#107.5,110.4].
public export
badTapSorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Tap Sorcery" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [Activated TapSymbol Macros.drawACard] Nothing {tx = ok})
badTapSorcery Oh impossible


||| a creature card printed with no power or toughness
||| A creature card writes its two numbers [CR#208.1].
public export
badCreatureCardNoPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) [] Nothing {pts = ok})
badCreatureCardNoPt MkCardPt impossible


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
  Macros.counterSpell (Macros.target Macros.creature) {zn = ok})
badCounterPermanent OnTheStack impossible


||| "Counter target spell: Draw a card."
||| Cancelling another's spell is no payment: an activation cost is what the activator pays [CR#602.1a].
public export
badCounterAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.counterSpell (Macros.target Macros.spell)) {ok}) Macros.drawACard)
badCounterAsCost Oh impossible


||| "You may play a spell this turn." of an object on the stack
||| [CR#112.1] makes an object on the stack a spell, and a spell has already been cast.
public export
badPlayFromStack : Unspellable (Effect []) (\ok =>
  Continuously (MayPlay You (Macros.a Macros.spell) {pz = ok}) (Just Macros.thisTurn))
badPlayFromStack MkPlaySource impossible


||| "You may cast a land card from your graveyard this turn."
||| A land card "can be played only as a land. It can't be cast as a spell" [CR#305.9].
public export
badCastALand : Unspellable (Effect []) (\ok =>
  Continuously (MayPlay You (Macros.a (And [Macros.land, InZone (Macros.graveyardOf You)]))
                           {verb = Cast} {cv = ok})
               (Just Macros.thisTurn))
badCastALand MkCastableTy impossible


||| "You may play a creature card in exile from your graveyard this turn."
||| A written source phrase must agree with the zone the complement already names.
public export
badPlayFromWrongZone : Unspellable (Effect []) (\ok =>
  Continuously (MayPlay You (Macros.a (And [Macros.creature, InZone Macros.exileZ]))
                           {from = Just (Macros.graveyardOf You)} {pz = ok})
               (Just Macros.thisTurn))
badPlayFromWrongZone MkPlaySource impossible


||| "Put target creature onto the stack."
||| [CR#601.2a] puts a card on the stack as part of casting, which is no placement.
public export
badMoveToStack : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.stackZ {ok})
badMoveToStack BattlefieldOk impossible


||| "Put target instant card from a graveyard onto the battlefield tapped."
||| Instant and sorcery cards can't enter the battlefield [CR#110.4]; [CR#110.4a] lists the six that can.
public export
badInstantOntoBattlefield : Unspellable (Effect []) (\ok =>
  Macros.putOntoBattlefieldTapped (Macros.target (And [HasType Instant, InZone Macros.graveyardZ])) {pl = ok})
badInstantOntoBattlefield Oh impossible


||| "Put a creature card from your graveyard onto the battlefield: Draw a card."
||| [CR#601.2h] pays the cost to activate; a battlefield placement is what the ability buys.
public export
badMoveOntoBattlefieldAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.putOntoBattlefield (Macros.a (And [Macros.creature, InZone (Macros.graveyardOf You)]))) {ok})
            Macros.drawACard)
badMoveOntoBattlefieldAsCost Oh impossible


||| "unless" written over a positive condition
||| "Unless" is the negation, so a positive condition under the word negates twice.
public export
badUnlessOnPositive : Unspellable Ability (\ok =>
  Static (Conditionally (Exists (And [Macros.artifact, ControlledBy You]))
                        (Deontic Macros.thisCreature Forbid Attack Agent Nothing)
                        {marking = Unless} {mk = ok}))
badUnlessOnPositive MkMarkingOk impossible


||| "At your end of combat, draw a card."
||| The end-of-combat header takes no possessor; the possessed phrase is [CR#511.2]'s duration reading.
public export
badTriggerAtYourEndOfCombat : Unspellable Ability (\ok =>
  Triggered At (BeginningOf EndOfCombat (ByWord Yours) {pu = ok}) Macros.drawACard)
badTriggerAtYourEndOfCombat Oh impossible


||| "Target creature gets +1/+1 until the end-of-combat step."
||| [CR#511.2] puts "until end of combat" at the end of the combat phase; the step names no endpoint.
public export
badUntilEndOfCombatStep : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 1)) (PtUp (Lit 1)) (Just (Until (StartOf EndOfCombat Nothing))) {sp = ok})
badUntilEndOfCombatStep SpanStated impossible


||| "Destroy target creature. Exile the destroyed card."
||| The destroy row writes the deictic only, "destroyed this way".
public export
badDestroyedAttributive : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                Macros.exile (TheVerbed Destroy CardW {marking = Attributive} {mk = ok})])
badDestroyedAttributive Oh impossible


||| a trigger header watching a permanent flip
||| The flip category has an effect frame and no event frame [CR#710.1a].
public export
badFlipEvent : Unspellable Ability (\ok =>
  Triggered Whenever (StatusEvent (Macros.a Permanent) Flipped {at = ok}) Macros.drawACard)
badFlipEvent Oh impossible


||| a trigger header watching a permanent become unflipped
||| Flipping is one-way [CR#710.4], so there is no transition to observe.
public export
badUnflipEvent : Unspellable Ability (\ok =>
  Triggered Whenever (StatusEvent (Macros.a Permanent) Unflipped {at = ok}) Macros.drawACard)
badUnflipEvent Oh impossible




||| "target creature the controller of any target controls"
||| The singular quantity licenses the class word as head, not inside a possessor.
public export
badEmbeddedAnyTargetExact1 : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, ControlledBy (ControllerOf (Macros.target AnyTarget))]) {af = ok})
badEmbeddedAnyTargetExact1 Oh impossible




||| "At a creature becomes tapped, draw a card."
||| [CR#603.2b] keeps "at" for phases and steps, as it does for every other object event.
public export
badAtBecomesTapped : Unspellable Ability (\ok =>
  Triggered At (StatusEvent (Macros.a Macros.creature) Tapped) Macros.drawACard {wo = ok})
badAtBecomesTapped Oh impossible


||| "If target creature would become tapped, exile it instead this turn."
||| Trigger-only, reader by reader: nothing intercepts a becomes-tapped.
public export
badInterceptBecomesTapped : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (StatusEvent (Macros.target Macros.creature) Tapped)
                 (Macros.exile It) (Just Macros.thisTurn)
                 {ok = Builtin.fst ok, uo = Builtin.snd ok})
badInterceptBecomesTapped (Oh, _) impossible


||| "Exile target creature until a creature becomes untapped."
||| The [CR#610.3] rider waits on a departure, never on a status transition.
public export
badHeldUntilBecomesUntapped : Unspellable (Effect []) (\ok =>
  Macros.exileUntil (Macros.target Macros.creature) (StatusEvent (Macros.a Macros.creature) Untapped) {hd = ok})
badHeldUntilBecomesUntapped Oh impossible


||| "When target creature becomes tapped, sacrifice that creature."
||| The delayed clause's event family is the end-step beginning, the departure and the death.
public export
badDelayedOnBecomesTapped : Unspellable (Effect []) (\ok =>
  Delayed (StatusEvent (Macros.target Macros.creature) Tapped)
          (Macros.sacrifice You (That (TypeW Creature))) {aw = ok})
badDelayedOnBecomesTapped Oh impossible


||| "Target creature gets +2/+2 until this creature becomes untapped."
||| A tapped-state span is "for as long as … remains tapped" [CR#611.2b], a condition and not an end.
public export
badGetsUntilBecomesUntapped : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 2)) (PtUp (Lit 2))
       (Just (UntilEvent (StatusEvent Macros.thisCreature Untapped))) {sp = ok})
badGetsUntilBecomesUntapped SpanStated impossible


||| "Target creature card in your graveyard doesn't untap during its controller's next untap step."
||| The timed clause's subject stands on the battlefield [CR#701.26a].
public export
badUntapNextGraveyard : Unspellable (Effect []) (\ok =>
  DoesntUntapNext (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) 1 {ok = ok})
badUntapNextGraveyard OnField impossible


||| "Tap target creature. Tap target artifact. It doesn't untap during its controller's next untap step."
||| "It" after two singular object introductions reaches two mentions and resolves neither.
public export
badUntapNextAmbiguousIt : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                SetStatus Tapped (Macros.target Macros.artifact),
                DoesntUntapNext (It {ok = ok}) 1])
badUntapNextAmbiguousIt Refl impossible


||| "Target creature doesn't untap during its controller's next three untap steps."
||| The count vocabulary is closed at one and two.
public export
badUntapNextThree : Unspellable (Effect []) (\ok =>
  DoesntUntapNext (Macros.target Macros.creature) 3 {ct = ok})
badUntapNextThree OneNextStep impossible


||| "Whenever you cast any target, draw a card."
||| The cast header names a spell, never [CR#115.4]'s damage class.
public export
badCastsAnyTarget : Unspellable Ability (\ok =>
  Triggered Whenever
            (Casts You (Macros.target AnyTarget)
                   {zn = Builtin.fst ok}
                   {nt = Builtin.fst (Builtin.snd ok)})
            Macros.drawACard
            {hn = Builtin.snd (Builtin.snd ok)})
badCastsAnyTarget (_, (Oh, _)) impossible


||| "Exile target creature tapped."
||| Status words are the battlefield's alone [CR#110.5,110.5b]; an exile writes the counter rider only.
public export
badExileTapped : Dependent.Unspellable (Effect []) (\x, y =>
  Composite Exile (Move (Macros.target Macros.creature) Macros.exileZ
                        {riders = MkMoveRiders [EntersTapped] Nothing}
                        {rf = x})
                  {ok = y})
badExileTapped (Oh ** _) impossible


||| "Turn target creature card in your graveyard face down."
||| The face verb takes a permanent [CR#708.7], which a graveyard phrase contradicts.
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


||| "Whenever a permanent is turned face down, draw a card."
||| The turning is rules-legal [CR#708.2a], but this table has no trigger-header row over it.
public export
badTurnedFaceDownEvent : Unspellable Ability (\ok =>
  Triggered Whenever (StatusEvent (Macros.a Permanent) FaceDown {at = ok}) Macros.drawACard)
badTurnedFaceDownEvent Oh impossible


||| "At a creature phases out, draw a card."
||| [CR#603.2b] keeps "at" for phases and steps, as it does for every other object event.
public export
badAtPhasesOut : Unspellable Ability (\ok =>
  Triggered At (StatusEvent (Macros.a Macros.creature) PhasedOut) Macros.drawACard {wo = ok})
badAtPhasesOut Oh impossible


||| "If target creature would phase out, exile it instead this turn."
||| Trigger-only, reader by reader: nothing replaces a phasing.
public export
badInterceptPhasesOut : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (StatusEvent (Macros.target Macros.creature) PhasedOut)
                 (Macros.exile It) (Just Macros.thisTurn)
                 {ok = Builtin.fst ok, uo = Builtin.snd ok})
badInterceptPhasesOut (Oh, _) impossible


||| "Exile target creature until a permanent you control is turned face up."
||| The [CR#610.3] rider waits on a departure, never on a turning.
public export
badHeldUntilTurnedFaceUp : Unspellable (Effect []) (\ok =>
  Macros.exileUntil (Macros.target Macros.creature)
                    (StatusEvent (Macros.a (And [Permanent, ControlledBy You])) FaceUp) {hd = ok})
badHeldUntilTurnedFaceUp Oh impossible


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


||| "target creature not blocked by this creature"
||| The relational row does not negate; the bare designation's negation is a different word.
public export
badNegatedBlockedBy : Unspellable (Predicate [] Object) (\ok =>
  Not (BlockedBy Macros.thisCreature) {ng = ok})
badNegatedBlockedBy Oh impossible


||| "At a creature becomes blocked, draw a card."
||| [CR#603.2b] keeps "at" for phases and steps, as it does for every other object event.
public export
badAtBecomesBlocked : Unspellable Ability (\ok =>
  Triggered At (BecomesBlocked (Macros.a Macros.creature) Nothing) Macros.drawACard {wo = ok})
badAtBecomesBlocked Oh impossible


||| "Whenever a creature blocks this, draw a card."
||| A bare self offers no evidence of the battlefield [CR#109.2], so the partner is type-ascribed.
public export
badBlocksBareThisPartner : Unspellable Ability (\ok =>
  Triggered Whenever (Blocks (Macros.a Macros.creature) (Just This)
                             {bp = OnePartner {ss = ok}}) Macros.drawACard)
badBlocksBareThisPartner Oh impossible


||| "If target creature would become blocked, exile it instead this turn."
||| Nothing replaces a block declaration; the abilities that stop one are restrictions [CR#509.1b].
public export
badInterceptBecomesBlocked : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (BecomesBlocked (Macros.target Macros.creature) Nothing)
                 (Macros.exile It) (Just Macros.thisTurn)
                 {ok = Builtin.fst ok, uo = Builtin.snd ok})
badInterceptBecomesBlocked (Oh, _) impossible


||| "Your opponents can't untap more than three lands during their untap steps."
||| The bound vocabulary is closed at one and two, so only the count refuses here.
public export
badUntapCapThree : Unspellable Ability (\ok =>
  Static (CantUntapMoreThan (PlayerGroup YourOpponents) 3 Macros.land {bd = ok}))
badUntapCapThree OneUntap impossible


||| "Players can't untap more than one creature card in your graveyard during their untap steps."
||| [CR#502.3] untaps the permanents a player controls, which a graveyard phrase contradicts.
public export
badUntapCapGraveyardSet : Unspellable Ability (\ok =>
  Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1
                            (And [Macros.creature, InZone (Macros.graveyardOf You)]) {zn = ok}))
badUntapCapGraveyardSet Oh impossible


||| "Players can't untap more than one tapped during their untap steps."
||| The capped set writes a head noun, and a status word is a modifier that heads nothing.
public export
badUntapCapHeadless : Unspellable Ability (\ok =>
  Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.tapped {hd = ok}))
badUntapCapHeadless Oh impossible


||| "Players can't untap more than one land until your next untap step."
||| The cap names its interval in its own words and takes no endpoint adverbial.
public export
badUntapCapUntilNextUntapStep : Unspellable (Effect []) (\ok =>
  Continuously (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.land)
               (Just (Until (StartOf UntapStep (Just Yours)))) {sp = ok})
badUntapCapUntilNextUntapStep SpanStated impossible


||| "Put a poison counter on target creature."
||| [CR#122.1] puts a counter on an object or a player and the two never cross; poison is a player's.
public export
badPutPoisonOnCreature : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) Poison (Macros.target Macros.creature) {sc = ok})
badPutPoisonOnCreature Refl impossible


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
  LosesAllCounters (Each Opponent) (Just Macros.plusOnePlusOne)
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
  Triggered When (LastCounterRemoved Poison Macros.thisCreature {sc = ok}) Macros.drawACard)
badLastPoisonCounterRemoved Refl impossible


||| "At the last time counter is removed from this card, draw a card."
||| [CR#603.2b] keeps "at" for phases and steps, as it does for every other object event.
public export
badAtLastCounterRemoved : Unspellable Ability (\ok =>
  Triggered At (LastCounterRemoved Time Macros.thisCreature) Macros.drawACard {wo = ok})
badAtLastCounterRemoved Oh impossible


||| "When the last time counter is removed from this card, if this creature is exiled, draw a card."
||| The sorted self-word seeds the battlefield [CR#109.2], so it cannot be asked whether it is elsewhere.
public export
badExileCheckOnSortedSelf : Unspellable Ability (\ok =>
  Triggered When (LastCounterRemoved Time Macros.thisCreature) Macros.drawACard
            {intervening = Just (Matches Macros.thisCreature (InZone Macros.exileZ)
                                         {zc = ok})})
badExileCheckOnSortedSelf Oh impossible


||| "When this creature enters, if you died this turn, draw a card."
||| Each event names the sort its history subject takes, and dying is an object's.
public export
badLookbackPlayerDied : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature) Macros.drawACard
            {intervening = Just (Happened Death You Lookback.ThisTurn {sb = ok})})
badLookbackPlayerDied MkLookbackSubject impossible


||| "When this creature enters, if a creature cast a spell this turn, draw a card."
||| The same table the other way: casting is read over a player [CR#601.2].
public export
badLookbackObjectCast : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature) Macros.drawACard
            {intervening = Just (Happened SpellCast (Macros.a Macros.creature)
                                          Lookback.ThisTurn {sb = ok})})
badLookbackObjectCast MkLookbackSubject impossible


||| "When this creature enters, if an upkeep began this turn, draw a card."
||| A turn-part beginning takes a trigger header [CR#603.2b] and is not read as history.
public export
badLookbackPartBeginning : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature) Macros.drawACard
            {intervening = Just (Happened PartBeginning (Macros.a Macros.creature)
                                          Lookback.ThisTurn {sb = ok})})
badLookbackPartBeginning MkLookbackSubject impossible


||| "target creature who cast a spell this turn"
||| The head noun is the history read's subject, and casting is a player's event [CR#601.2].
public export
badHappenedToObjectCast : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, HappenedTo SpellCast Lookback.ThisTurn {sb = ok}]))
badHappenedToObjectCast MkLookbackSubject impossible


||| "each opponent who died this turn"
||| The same table at its second reader: dying is an object's event, so a player head cannot ask it.
public export
badHappenedToPlayerDied : Unspellable (Noun [] Player) (\ok =>
  Each (And [Opponent, HappenedTo Death Lookback.ThisTurn {sb = ok}]))
badHappenedToPlayerDied MkLookbackSubject impossible


||| "target colorless white creature"
||| [CR#105.2c] gives a colorless object no color, so the two words describe nothing together.
public export
badColorlessWhite : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, IsColorless, ColorIs White] {cf = ok}))
badColorlessWhite Oh impossible


||| "target nonmulticolored permanent"
||| The three colour-count words take no prefix negation, unlike the five colour words.
public export
badNonMulticolored : Unspellable (Predicate [] Object) (\ok =>
  Not Multicolored {ng = ok})
badNonMulticolored Oh impossible


||| "At the beginning of an opponent's upkeep, draw a card."
||| The trigger header's possessor table does not admit this possessor, though the activation window's does.
public export
badTriggerAtAnOpponentsUpkeep : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Upkeep (ByWord AnOpponents) {pu = ok}) Macros.drawACard)
badTriggerAtAnOpponentsUpkeep Oh impossible


||| "At the beginning of each opponent's first main phase, draw a card."
||| A new turn part does not inherit a possessor's cells from the header table.
public export
badTriggerAtEachOpponentsFirstMain : Unspellable Ability (\ok =>
  Triggered At (BeginningOf FirstMain (ByWord EachOpponents) {pu = ok}) Macros.drawACard)
badTriggerAtEachOpponentsFirstMain Oh impossible


||| "{2}: Draw a card. Activate only during your end step."
||| The end step is a trigger header's part; no activation restriction names it.
public export
badWindowDuringYourEndStep : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) Macros.drawACard
            {window = Just (DuringPart EndStep (Just Yours) {wk = ok})})
badWindowDuringYourEndStep Oh impossible


||| "Target land attacks each combat if able."
||| Only a creature attacks [CR#506.3], and the requirement reads the restriction's own grid.
public export
badMustAttackLand : Unspellable (Effect []) (\ok =>
  Continuously (Deontic (Macros.target Macros.land) Require Attack Agent Nothing {dp = ok})
               (Just Macros.thisTurn))
badMustAttackLand Participant impossible


||| "This creature attacks target creature each combat if able."
||| The attack requirement names no defender; goad's leg is a designation [CR#701.15b].
public export
badMustAttackWithPatient : Unspellable Ability (\ok =>
  Static (Deontic Macros.thisCreature Require Attack Agent (Just (Macros.target Macros.creature))
                  {pt = DeonticPatientWritten {ok = ok}}))
badMustAttackWithPatient Oh impossible


||| "Target creature blocks this turn if able."
||| The block requirement requires its patient, so the bare form is not English.
public export
badMustBlockNoPatient : Unspellable (Effect []) (\ok =>
  Continuously (Deontic (Macros.target Macros.creature) Require Block Agent Nothing
                        {pt = NoDeonticPatient {ok = ok}})
               (Just Macros.thisTurn))
badMustBlockNoPatient Oh impossible


||| "Target artifact may choose not to untap during your untap step this turn."
||| The permission to decline is a printed static ability and never a clause.
public export
badDeclineUntapClause : Unspellable (Effect []) (\ok =>
  Continuously (MayDeclineUntap (Macros.target Macros.artifact)) Nothing {sp = ok})
badDeclineUntapClause SpanUnstated impossible


||| "At the beginning of your end step, if it's day, draw a card."
||| The designation check writes only "it's night", where the transition writes both directions [CR#731.1].
public export
badItIsDay : Unspellable Ability (\ok =>
  Triggered At (BeginningOf EndStep (ByWord Yours)) Macros.drawACard
            {intervening = Just (GameIs Day {at = ok})})
badItIsDay Oh impossible


||| "if you aren't the monarch"
||| The designation read does not negate; "there is no monarch" is another construction.
public export
badNegatedMonarch : Unspellable (Predicate [] Player) (\ok =>
  Not (HasDesignation Monarch) {ng = ok})
badNegatedMonarch Oh impossible


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
  Triggered Whenever (Attacks Macros.thisCreature)
            (Macros.gets (That (TypeW Creature) {ok = ok}) (PtUp (Lit 2)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn)))
badThatCreatureIsSelf Refl impossible


||| "This creature can't attack target creature this turn."
||| The restriction's patient is a defending player, and no noun here writes one.
public export
badForbidAttackWithPatient : Unspellable (Effect []) (\ok =>
  Continuously (Deontic Macros.thisCreature Forbid Attack Agent
                        (Just (Macros.target Macros.creature))
                        {pt = DeonticPatientWritten {ok = ok}})
               (Just Macros.thisTurn))
badForbidAttackWithPatient Oh impossible


||| "This creature can't be blocked by target creature unless you pay {1}."
||| The gate's patient cell belongs to the block agent alone.
public export
badGateBlockPatientWithPatient : Unspellable Ability (\ok =>
  Static (Deontic Macros.thisCreature (GatedBy (Mana [Macros.generic 1]))
                  Block Patient (Just (Macros.target Macros.creature))
                  {pt = DeonticPatientWritten {ok = ok}}))
badGateBlockPatientWithPatient Oh impossible


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
  Static (OutcomeGate CantLose (Macros.target AnyPlayer)) {ut = ok})
badTargetedOutcomeGate Oh impossible


||| "You lose the game: Draw a card."
||| [CR#104.3e] has an effect state that a player loses; the outcome is never a cost.
public export
badConcludesAsCost : Unspellable Ability (\ok =>
  Activated (Do (Concludes LoseGame You) {ok = ok}) Macros.drawACard)
badConcludesAsCost Oh impossible


||| "Each player can't untap more than one creature during their untap step."
||| The cap's subject is closed at the bare plural, "you" and "your opponents".
public export
badUntapCapEachPlayer : Unspellable Ability (\ok =>
  Static (CantUntapMoreThan (Each AnyPlayer) 1 Macros.creature {cs = ok}))
badUntapCapEachPlayer Oh impossible


||| "This creature deals 2 damage to your opponents."
||| One magnitude and one recipient phrase, and a bare plural says neither each member's nor the group's.
public export
badPluralPlayerDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage Macros.thisCreature (Lit 2) (PlayerGroup YourOpponents) {pm = ok})
badPluralPlayerDamageRecipient Oh impossible


||| "creatures players control"
||| The possessor set is written "your opponents", never as a bare plural.
public export
badControlledByAllPlayers : Unspellable (Predicate [] Object) (\ok =>
  ControlledBy (PlayerGroup AllPlayers) {ps = ok})
badControlledByAllPlayers Oh impossible


||| "cards in players' graveyards"
||| The possessive reader answers from the same table and refuses the bare plural too.
public export
badOwnedByAllPlayers : Unspellable (ZoneExpr []) (\ok =>
  Macros.graveyardOf (PlayerGroup AllPlayers) {pn = ok})
badOwnedByAllPlayers Oh impossible


||| "At a player losing the game, put five +1/+1 counters on this creature."
||| [CR#603.2b] fixes the At header to a turn-part beginning.
public export
badGameLossAtTrigger : Unspellable Ability (\ok =>
  Triggered At (LosesGame (Macros.a AnyPlayer))
            (PutCounters (Lit 5) Macros.plusOnePlusOne Macros.thisCreature) {wo = ok})
badGameLossAtTrigger Oh impossible


||| "if a player has lost the game this turn"
||| The loss is watched and replaced, never queried; [CR#603.10f]'s look-back is the trigger's own.
public export
badGameLossLookback : Unspellable (Condition []) (\ok =>
  Happened GameLoss (Macros.a AnyPlayer) Lookback.ThisTurn {sb = ok})
badGameLossLookback MkLookbackSubject impossible


||| "When a player next loses the game this turn, draw a card."
||| The only "next" over this event is a replacement's multiplicity, which waits for no event.
public export
badDelayedGameLoss : Unspellable (Effect []) (\ok =>
  Delayed (LosesGame You) (Draw You (Lit 1)) {aw = ok})
badDelayedGameLoss Oh impossible


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


||| "with power less than or equal to twice this creature's power"
||| A comparison's standard is something already there to point at, not an amount a clause computes.
public export
badScaledBound : Unspellable (Predicate [] Object) (\ok =>
  Compare Power AtMost (Times 2 (Macros.powerOf This)) {cb = ok})
badScaledBound Oh impossible


||| "if your life total is less than or equal to twice an opponent's life total"
||| The same refusal at the condition frame, which reads the one bound table.
public export
badScaledConditionBound : Unspellable (Condition []) (\ok =>
  CompareAmt (PlayerStatOf LifeTotal You) AtMost
             (Times 2 (PlayerStatOf LifeTotal Macros.anOpponent)) {cb = ok})
badScaledConditionBound Oh impossible


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
  Triggered When (Enters Macros.thisCreature)
            (Draw You (TheDifference {ok}))
            {intervening = Just (Exists Macros.creatureYouControl)})
badNonComparisonDifference Refl impossible


||| "At the beginning of your upkeep, if you have fewer than seven cards in hand, draw a card if the difference is 3 or greater."
||| The margin is a magnitude a clause reads, never a state a clause measures.
public export
badDifferenceSubject : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Upkeep (ByWord Yours))
            (If Macros.drawACard
                (CompareAmt (TheDifference {ok = Refl}) AtLeast (Lit 3) {rd = ok})
                Nothing)
            {intervening = Just (CompareAmt (CountOf (InZone (Macros.handOf You)))
                                            Less (Lit 7))})
badDifferenceSubject Oh impossible


||| "Draw X cards."
||| The letter is a name the sentence introduced [CR#107.3c]; this one defined none.
public export
badUnlicensedX : Unspellable (Effect []) (\ok =>
  Draw You (DefinedLetter LetterX {ok}))
badUnlicensedX Refl impossible


||| "Draw X cards, where X is 4."
||| A rider is for a value that must be worked out; a written value is written as the numeral.
public export
badWrittenXDef : Unspellable (Effect []) (\ok =>
  WhereLetter LetterX (Lit 4) {xd = ok} (Draw You (DefinedLetter LetterX {ok = Refl})))
badWrittenXDef Oh impossible


||| "Draw X cards, where X is the number of creatures you control, where X is the number of creatures on the battlefield."
||| One ability defines one letter [CR#107.3]; a letter defined twice names nothing.
public export
badDoubleXRider : Unspellable (Effect []) (\ok =>
  WhereLetter LetterX (CountOf Macros.creatureYouControl)
              (WhereLetter LetterX (CountOf Macros.creature)
                           (Draw You (DefinedLetter LetterX {ok}))))
badDoubleXRider Refl impossible


||| "Target creature gets +0/-2 until end of turn."
||| A written zero is signless, so the slot takes its partner's sign [CR#613.4c].
public export
badDisagreeingZeroPump : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 0)) (PtDown (Lit 2))
       (Just Macros.untilEndOfTurn) {ps = ok})
badDisagreeingZeroPump Oh impossible


||| "Draw Y cards, where X is the number of creatures you control."
||| Y is a name of its own and follows X's rules, not X's definition [CR#107.3p].
public export
badUnlicensedY : Unspellable (Effect []) (\ok =>
  WhereLetter LetterX (CountOf Macros.creatureYouControl)
              (Draw You (DefinedLetter LetterY {ok})))
badUnlicensedY Refl impossible


||| "Target creature gets +X/+X, where X is the number of creatures you control, until end of turn."
||| The adverbial belongs to the clause and the rider binds the whole clause [CR#611.2a].
public export
badRiderInsideDuration : Unspellable (Effect []) (\ok =>
  Continuously (WhereLetterStatic LetterX (CountOf Macros.creatureYouControl)
                                  (Gets (Macros.target Macros.creature)
                                        (PtUp (DefinedLetter LetterX))
                                        (PtUp (DefinedLetter LetterX))))
               (Just Macros.untilEndOfTurn) {nr = ok})
badRiderInsideDuration Oh impossible


||| "This creature gets +X/+0, where X is the number of creatures you control, where X is the number of creatures on the battlefield."
||| One statement defines one letter [CR#107.3]; a letter defined twice names nothing.
public export
badDoubleStaticRider : Unspellable (StaticEffect []) (\ok =>
  WhereLetterStatic LetterX (CountOf Macros.creatureYouControl)
                    (WhereLetterStatic LetterX (CountOf Macros.creature)
                                       (Gets Macros.thisCreature
                                             (PtUp (DefinedLetter LetterX {ok}))
                                             (PtUp (Lit 0)))))
badDoubleStaticRider Refl impossible


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
  CostsToCast (AllOf Macros.creatureYouControl) (CostLess (Lit 1)) {cs = ok})
badCostSubjectOnBattlefield MkCostSubject impossible


||| "This spell costs {0} less to cast."
||| "{0}" is the payment of nothing [CR#118.5], and a reduction of it moves no cost.
public export
badZeroCostShift : Unspellable (StaticEffect []) (\ok =>
  CostsToCast This (CostLess (Lit 0)) {wc = ok})
badZeroCostShift Oh impossible


||| "Spells cost {1} less to cast." written as a resolving clause
||| A cost statement is a static ability's own line [CR#604.1], with no clause to carry a span.
public export
badCostClause : Unspellable (Effect []) (\ok =>
  Continuously (CostsToCast This (CostLess (Lit 1))) Nothing {sp = ok})
badCostClause SpanUnstated impossible


||| "a creature card you cast in your graveyard"
||| The cast relation puts its referent on the stack [CR#601.2a,112.1]; the origin zone is another qualifier.
public export
badCastInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, CastBy You, InZone Macros.graveyardZ] {zc = ok})
badCastInGraveyard Oh impossible


||| "spells players cast"
||| The cast clause answers the same possessor table and refuses the bare plural too.
public export
badCastByAllPlayers : Unspellable (Predicate [] Object) (\ok =>
  CastBy (PlayerGroup AllPlayers) {ps = ok})
badCastByAllPlayers Oh impossible
