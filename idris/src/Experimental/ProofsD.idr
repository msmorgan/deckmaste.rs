module Experimental.ProofsD

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "an attacking creature exiled with this creature"
||| The card word travels with the linkage but the status word does not:
||| attacking seeds the battlefield, where the linked cards are in exile.
public export
badExiledWithAttacking : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, Macros.exiledWithThisArtifact] {zc = ok})
badExiledWithAttacking MkZoneCoherent impossible


||| "not exiled with this artifact"
||| The linkage read does not NEGATE: a source-keyed group is named to be
||| acted on, and the cards outside it are described by another phrase.
public export
badNegatedExiledWith : Unspellable (Predicate [] Object) (\ok =>
  Not Macros.exiledWithThisArtifact {ng = ok})
badNegatedExiledWith MkNegatable impossible


||| "Draw a card." printed as a spell ability on a creature card
||| [CR#113.3a] defines the category by when it is followed, "while an
||| instant or sorcery spell is resolving", which a creature card never is.
public export
badSpellAbilityOnPermanent : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature])
       [Spell Macros.drawACard] (Just (1, 1)) {tx = ok})
badSpellAbilityOnPermanent MkCardText impossible


||| "Creatures you control get +1/+1." printed as a static ability on a sorcery
||| [CR#113.3a] admits one only if it fits [CR#113.6]'s criteria, and every
||| static row here establishes a continuous effect on the battlefield.
public export
badStaticOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Sorcery])
       [Static (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))] Nothing {tx = ok})
badStaticOnSorcery MkCardText impossible


||| "Flying" printed as a bare line on an instant card
||| Shut by MEASUREMENT rather than by rule: the seven keywords here are
||| [CR#702] abilities of a permanent in combat, and no spell card prints one.
public export
badKeywordOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Instant])
       [KeywordAbility Flying] Nothing {tx = ok})
badKeywordOnInstant MkCardText impossible


||| "{T}: Draw a card." printed on a sorcery card
||| [CR#113.6j] opens the row only where the cost can be paid off the
||| battlefield, and "{T}" taps a permanent [CR#107.5] no sorcery is [CR#110.4].
public export
badTapSorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Tap Sorcery" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [Activated TapSymbol Macros.drawACard] Nothing {tx = ok})
badTapSorcery MkCardText impossible


||| a creature card printed with no power or toughness
||| A creature card writes its two numbers, "separated by a slash printed in
||| its lower right corner" [CR#208.1].
public export
badCreatureCardNoPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) [] Nothing {pts = ok})
badCreatureCardNoPt MkCardPt impossible


||| a land card printed with "{1}"
||| A land card writes NO mana cost [CR#202.1b], the absence being an
||| unpayable cost [CR#118.6] rather than an omission.
public export
badLandWithManaCost : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Land]) [] Nothing {mc = ok})
badLandWithManaCost MkCardCost impossible


||| "Legendary Legendary Creature"
||| [CR#205.4b] makes a supertype a property an object HAS or LACKS, so the
||| word printed twice is one fact written twice.
public export
badDuplicateSupertype : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [Legendary, Legendary] (MkTypeLine [] [Creature])
       [] (Just (1, 1)) {sp = ok})
badDuplicateSupertype MkCardSupers impossible


||| "Land Creature Instant"
||| Instant and sorcery cards "can't enter the battlefield and thus can't be
||| permanents" [CR#110.4], and [CR#110.4a] lists the six types that can.
public export
badMixedPermanentSpellLine : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land, Creature, Instant]) [] (Just (1, 1)) {ln = ok})
badMixedPermanentSpellLine MkCardLine impossible


||| a card printed with an empty type line
||| [CR#205.1] has the line contain "the card's card type(s)" without
||| qualification, where subtypes and supertypes are there "if applicable".
public export
badCardNoTypes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] []) [] Nothing {ln = ok})
badCardNoTypes MkCardLine impossible


||| "Creature Artifact"
||| And the line writes its types in the printed ORDER: "artifact creature" is
||| the spelling, and the reverse is written nowhere.
public export
badCardTypeOrder : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) [] (MkTypeLine [] [Creature, Artifact]) []
       (Just (2, 2)) {ln = ok})
badCardTypeOrder MkCardLine impossible


||| "Target creature gains a spell ability."
||| A category error, not the quotation gap its siblings carry: this clause
||| grants to a permanent, and a spell ability is a resolving spell's [CR#113.3a].
public export
badGainsSpellAbility : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (Spell Macros.drawACard) Nothing {gr = ok})
badGainsSpellAbility MkGrantable impossible


||| "Counter target creature."
||| The countering's complement is a SPELL and the zone says so [CR#112.1]; a
||| battlefield permanent has already resolved.
public export
badCounterPermanent : Unspellable (Effect []) (\ok =>
  Macros.counterSpell (Macros.target Macros.creature) {zn = ok})
badCounterPermanent OnTheStack impossible


||| "Counter target spell: Draw a card."
||| Cancelling somebody else's spell is not a PAYMENT: [CR#602.1a] makes an
||| activation cost what the activator pays.
public export
badCounterAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.counterSpell (Macros.target Macros.spell)) {ok}) Macros.drawACard)
badCounterAsCost MkCostAction impossible


||| "You may play a spell this turn." of an object on the stack
||| The stack is not a place one plays a card FROM: [CR#112.1] makes an object
||| there a spell, and a spell has already been cast.
public export
badPlayFromStack : Unspellable (Effect []) (\ok =>
  Continuously (MayPlay You (Macros.a Macros.spell) {pz = ok}) (Just Macros.thisTurn))
badPlayFromStack MkPlaySource impossible


||| "You may cast a land card from your graveyard this turn."
||| "Cast" excludes the LAND [CR#305.9]: such an object "can be played only as
||| a land. It can't be cast as a spell."
public export
badCastALand : Unspellable (Effect []) (\ok =>
  Continuously (MayPlay You (Macros.a (And [Macros.land, InZone (Macros.graveyardOf You)]))
                           {verb = Cast} {cv = ok})
               (Just Macros.thisTurn))
badCastALand MkCastableTy impossible


||| "You may play a creature card in exile from your graveyard this turn."
||| A written source phrase must AGREE with what the complement already says:
||| the permission's two ways of naming a zone are one fact.
public export
badPlayFromWrongZone : Unspellable (Effect []) (\ok =>
  Continuously (MayPlay You (Macros.a (And [Macros.creature, InZone Macros.exileZ]))
                           {from = Just (Macros.graveyardOf You)} {pz = ok})
               (Just Macros.thisTurn))
badPlayFromWrongZone MkPlaySource impossible


||| "Put target creature onto the stack."
||| Nothing MOVES to the stack: [CR#601.2a] puts a card there as the first step
||| of casting it, which is an action a player takes and not a placement.
public export
badMoveToStack : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.stackZ {ok})
badMoveToStack BattlefieldOk impossible


||| "Put target instant card from a graveyard onto the battlefield tapped."
||| The destination asks about its PATIENT: instant and sorcery cards "can't
||| enter the battlefield" [CR#110.4], and [CR#110.4a] lists the six that can.
public export
badInstantOntoBattlefield : Unspellable (Effect []) (\ok =>
  Macros.putOntoBattlefieldTapped (Macros.target (And [HasType Instant, InZone Macros.graveyardZ])) {pl = ok})
badInstantOntoBattlefield MkPlaceable impossible


||| "Put a creature card from your graveyard onto the battlefield: Draw a card."
||| No cost component places anything on the battlefield: [CR#601.2h] pays the
||| cost to activate, and a placement is what the ability buys.
public export
badMoveOntoBattlefieldAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.putOntoBattlefield (Macros.a (And [Macros.creature, InZone (Macros.graveyardOf You)]))) {ok})
            Macros.drawACard)
badMoveOntoBattlefieldAsCost MkCostAction impossible


||| "unless" written over a positive condition
||| "Unless" IS the negation, taking a negated condition and spelling the
||| positive underneath; a positive one under the word negates twice.
public export
badUnlessOnPositive : Unspellable Ability (\ok =>
  Static (Conditionally (Exists (And [Macros.artifact, ControlledBy You]))
                        (Deontic Macros.thisCreature Forbid Attack Agent Nothing)
                        {marking = Unless} {mk = ok}))
badUnlessOnPositive MkMarkingOk impossible


||| "At your end of combat, draw a card."
||| The end-of-combat header takes no POSSESSOR; the "end of combat on your …"
||| lines are [CR#511.2]'s other reading, the phase-endpoint duration.
public export
badTriggerAtYourEndOfCombat : Unspellable Ability (\ok =>
  Triggered At (BeginningOf EndOfCombat (Just Yours) {pu = ok}) Macros.drawACard)
badTriggerAtYourEndOfCombat MkPartTriggerable impossible


||| "Target creature gets +1/+1 until the end-of-combat step."
||| And the sixth part names no duration endpoint at all, [CR#511.2] putting
||| "until end of combat" at the end of the combat PHASE. One phrase, one slot.
public export
badUntilEndOfCombatStep : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 1)) (PtUp (Lit 1)) (Just (Until (StartOf EndOfCombat Nothing))) {sp = ok})
badUntilEndOfCombatStep SpanStated impossible


||| "Destroy target creature. Exile the destroyed card."
||| The participle's two surfaces are not interchangeable per verb: the destroy
||| row writes the deictic only, "destroyed this way".
public export
badDestroyedAttributive : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                Macros.exile (TheVerbed Destroy CardW {marking = Attributive} {mk = ok})])
badDestroyedAttributive MkVerbedMarkingOk impossible


||| "Whenever a permanent becomes flipped, draw a card."
||| Flipping exists under its own verb [CR#710], but no line writes it as a
||| becomes-status event.
public export
badBecomesFlipped : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) Flipped {at = ok}) Macros.drawACard)
badBecomesFlipped MkStatusEventVal impossible


||| "Whenever a permanent becomes unflipped, draw a card."
||| Stronger than its siblings: flipping is one-way [CR#710.4], so the
||| transition cannot happen at all.
public export
badBecomesUnflipped : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) Unflipped {at = ok}) Macros.drawACard)
badBecomesUnflipped MkStatusEventVal impossible


||| "Whenever a permanent becomes face up, draw a card."
||| Turning face up exists under its own verb [CR#708], but no line writes it
||| as a becomes-status event.
public export
badBecomesFaceUp : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) FaceUp {at = ok}) Macros.drawACard)
badBecomesFaceUp MkStatusEventVal impossible


||| "Whenever a permanent becomes face down, draw a card."
||| Turning face down likewise [CR#708]: the operation is real, and the event
||| word for it is unwritten.
public export
badBecomesFaceDown : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) FaceDown {at = ok}) Macros.drawACard)
badBecomesFaceDown MkStatusEventVal impossible


||| "target creature the controller of any target controls"
||| The singular quantity licenses the class word as the phrase's HEAD, not
||| in a possessor, where a counted mention forbids it exactly as "a" does.
public export
badEmbeddedAnyTargetExact1 : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, ControlledBy (ControllerOf (Macros.target AnyTarget))]) {af = ok})
badEmbeddedAnyTargetExact1 MkAnyTargetAtCount impossible


||| "Whenever a permanent becomes phased in, draw a card."
||| Phasing exists under its own verb [CR#702.26], but no line writes it as a
||| becomes-status event.
public export
badBecomesPhasedIn : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) PhasedIn {at = ok}) Macros.drawACard)
badBecomesPhasedIn MkStatusEventVal impossible


||| "Whenever a permanent becomes phased out, draw a card."
||| Phasing out likewise [CR#702.26]: the operation is real, and the event word
||| for it is unwritten.
public export
badBecomesPhasedOut : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) PhasedOut {at = ok}) Macros.drawACard)
badBecomesPhasedOut MkStatusEventVal impossible


||| "At a creature becomes tapped, draw a card."
||| [CR#603.2b] keeps "at" for phases and steps, as it does for every other
||| object event.
public export
badAtBecomesTapped : Unspellable Ability (\ok =>
  Triggered At (BecomesStatus (Macros.a Macros.creature) Tapped) Macros.drawACard {wo = ok})
badAtBecomesTapped MkTriggerWordOk impossible


||| "If target creature would become tapped, exile it instead this turn."
||| Trigger-only, reader by reader: nothing intercepts a becomes-tapped.
public export
badInterceptBecomesTapped : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (BecomesStatus (Macros.target Macros.creature) Tapped)
                 (Macros.exile It) (Just Macros.thisTurn)
                 {ok = Builtin.fst ok, uo = Builtin.snd ok})
badInterceptBecomesTapped (MkInterceptable, _) impossible


||| "Exile target creature until a creature becomes untapped."
||| Nor does the [CR#610.3] rider wait for one: its whole corpus is the
||| departure.
public export
badHeldUntilBecomesUntapped : Unspellable (Effect []) (\ok =>
  Macros.exileUntil (Macros.target Macros.creature) (BecomesStatus (Macros.a Macros.creature) Untapped) {hd = ok})
badHeldUntilBecomesUntapped MkHoldable impossible


||| "When target creature becomes tapped, sacrifice that creature."
||| Nor does the delayed clause: the family stays the end-step beginning, the
||| departure, and the death.
public export
badDelayedOnBecomesTapped : Unspellable (Effect []) (\ok =>
  Delayed (BecomesStatus (Macros.target Macros.creature) Tapped)
          (Macros.sacrifice You (That (TypeW Creature))) {aw = ok})
badDelayedOnBecomesTapped MkAwaitable impossible


||| "Target creature gets +2/+2 until this creature becomes untapped."
||| No measured duration ends at a status transition: the tapped-STATE span
||| is "for as long as … remains tapped" [CR#611.2b], a condition, not an end.
public export
badGetsUntilBecomesUntapped : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 2)) (PtUp (Lit 2))
       (Just (UntilEvent (BecomesStatus Macros.thisCreature Untapped))) {sp = ok})
badGetsUntilBecomesUntapped SpanStated impossible


||| "Target creature card in your graveyard doesn't untap during its controller's next untap step."
||| The timed clause's subject stands on the battlefield [CR#701.26a], the tap
||| row's own demand at the new consumer.
public export
badUntapNextGraveyard : Unspellable (Effect []) (\ok =>
  DoesntUntapNext (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) 1 {ok = ok})
badUntapNextGraveyard OnField impossible


||| "Tap target creature. Tap target artifact. It doesn't untap during its controller's next untap step."
||| "It" after two singular object introductions reaches two mentions and
||| resolves neither.
public export
badUntapNextAmbiguousIt : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target Macros.creature),
                Tap (Macros.target Macros.artifact),
                DoesntUntapNext (It {ok = ok}) 1])
badUntapNextAmbiguousIt Refl impossible


||| "Target creature doesn't untap during its controller's next three untap steps."
||| The count vocabulary is closed at the attested one and two.
public export
badUntapNextThree : Unspellable (Effect []) (\ok =>
  DoesntUntapNext (Macros.target Macros.creature) 3 {ct = ok})
badUntapNextThree OneNextStep impossible


||| "Whenever you cast any target, draw a card."
||| The cast event's complement is the same phrase in the same zone and takes
||| the same strict demand: the headers name a spell, never the damage class.
public export
badCastsAnyTarget : Unspellable Ability (\ok =>
  Triggered Whenever
            (Casts You (Macros.target AnyTarget)
                   {zn = Builtin.fst ok}
                   {nt = MkNontarget {ok = Builtin.fst (Builtin.snd ok)}})
            Macros.drawACard
            {hn = MkHeaderNontarget {ok = Builtin.snd (Builtin.snd ok)}})
badCastsAnyTarget (_, (Refl, _)) impossible


||| "Exile target creature tapped."
||| An exile writes the counter rider and nothing else: [CR#110.5b]'s untapped
||| default and [CR#110.5]'s status words are the battlefield's alone.
public export
badExileTapped : Dependent.Unspellable (Effect []) (\x, y =>
  Composite Exile (Move (Macros.target Macros.creature) Macros.exileZ
                        {riders = MkMoveRiders [EntersTapped] Nothing}
                        {rf = MkRidersFit {ok = x}})
                  {ok = y})
badExileTapped (Refl ** _) impossible


||| "Turn target creature card in your graveyard face down."
||| The face verb takes a permanent [CR#708.7], so the demand is the tap
||| row's zonal one and a graveyard phrase contradicts it.
public export
badTurnFaceDownGraveyard : Unspellable (Effect []) (\ok =>
  ToFace FaceDown (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok = ok})
badTurnFaceDownGraveyard OnField impossible


||| "Target creature card in your hand phases out."
||| Phasing changes no zone [CR#702.26d] because it starts on the
||| battlefield: a hand card has no phasing status to change [CR#110.5].
public export
badPhasesOutInHand : Unspellable (Effect []) (\ok =>
  Phases PhasedOut (Macros.target (And [Macros.creature, InZone (Macros.handOf You)])) {ok = ok})
badPhasesOutInHand OnField impossible


||| "Whenever a permanent is turned face down, draw a card."
||| The turning is real and rules-legal [CR#708.2a]; the trigger header
||| over it is written zero times against the up cell's mass.
public export
badTurnedFaceDownEvent : Unspellable Ability (\ok =>
  Triggered Whenever (IsTurnedFace (Macros.a Permanent) FaceDown {at = ok}) Macros.drawACard)
badTurnedFaceDownEvent MkFaceEventVal impossible


||| "At a creature phases out, draw a card."
||| [CR#603.2b] keeps "at" for phases and steps, as it does for every other
||| object event.
public export
badAtPhasesOut : Unspellable Ability (\ok =>
  Triggered At (PhaseTransition (Macros.a Macros.creature) PhasedOut) Macros.drawACard {wo = ok})
badAtPhasesOut MkTriggerWordOk impossible


||| "If target creature would phase out, exile it instead this turn."
||| Trigger-only, reader by reader: nothing replaces a phasing.
public export
badInterceptPhasesOut : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (PhaseTransition (Macros.target Macros.creature) PhasedOut)
                 (Macros.exile It) (Just Macros.thisTurn)
                 {ok = Builtin.fst ok, uo = Builtin.snd ok})
badInterceptPhasesOut (MkInterceptable, _) impossible


||| "Exile target creature until a permanent you control is turned face up."
||| Nor does the [CR#610.3] rider wait for a turning: its whole corpus is
||| the departure.
public export
badHeldUntilTurnedFaceUp : Unspellable (Effect []) (\ok =>
  Macros.exileUntil (Macros.target Macros.creature)
                    (IsTurnedFace (Macros.a (And [Permanent, ControlledBy You])) FaceUp) {hd = ok})
badHeldUntilTurnedFaceUp MkHoldable impossible


||| "Remove target creature card in your graveyard from combat."
||| Combat is the battlefield's, [CR#506.4] listing the ways a PERMANENT
||| leaves it; the tap row's zonal demand at a new consumer.
public export
badRemoveFromCombatGraveyard : Unspellable (Effect []) (\ok =>
  RemoveFromCombat (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok = ok})
badRemoveFromCombatGraveyard OnField impossible


||| "target creature blocking target creature card in your graveyard"
||| The relation holds between two creatures in combat [CR#509.1g], so the
||| relatum may not name another zone.
public export
badBlockingGraveyardRelatum : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature,
                      BlockerOf (Macros.target (And [Macros.creature,
                                                     InZone (Macros.graveyardOf You)])) {zn = ok}]))
badBlockingGraveyardRelatum MkZoneFits impossible


||| "target creature not blocked by this creature"
||| The relational row does not negate: zero corpus lines, where the bare
||| designation's negation is ordinary ("nonattacking, nonblocking").
public export
badNegatedBlockedBy : Unspellable (Predicate [] Object) (\ok =>
  Not (BlockedBy Macros.thisCreature) {ng = ok})
badNegatedBlockedBy MkNegatable impossible


||| "At a creature becomes blocked, draw a card."
||| [CR#603.2b] keeps "at" for phases and steps, as it does for every other
||| object event.
public export
badAtBecomesBlocked : Unspellable Ability (\ok =>
  Triggered At (BecomesBlocked (Macros.a Macros.creature) Nothing) Macros.drawACard {wo = ok})
badAtBecomesBlocked MkTriggerWordOk impossible


||| "Whenever a creature blocks this, draw a card."
||| The written partner is type-ascribed for the reason the subject is: a
||| bare self offers no evidence of the battlefield [CR#109.2], and the
||| corpus writes "blocks this creature" without exception.
public export
badBlocksBareThisPartner : Unspellable Ability (\ok =>
  Triggered Whenever (Blocks (Macros.a Macros.creature) (Just This)
                             {bp = OnePartner {ss = ok}}) Macros.drawACard)
badBlocksBareThisPartner MkSelfSorted impossible


||| "If target creature would become blocked, exile it instead this turn."
||| Trigger-only, reader by reader: nothing replaces a block declaration —
||| the abilities that stop one are restrictions checked as it is made
||| [CR#509.1b].
public export
badInterceptBecomesBlocked : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (BecomesBlocked (Macros.target Macros.creature) Nothing)
                 (Macros.exile It) (Just Macros.thisTurn)
                 {ok = Builtin.fst ok, uo = Builtin.snd ok})
badInterceptBecomesBlocked (MkInterceptable, _) impossible


||| "Your opponents can't untap more than three lands during their untap steps."
||| The bound vocabulary is closed at the attested one and two; the domain
||| written here is the emblem's own and well-formed, so only the count refuses.
public export
badUntapCapThree : Unspellable Ability (\ok =>
  Static (CantUntapMoreThan (PlayerGroup YourOpponents) 3 Macros.land {bd = ok}))
badUntapCapThree OneUntap impossible


||| "Players can't untap more than one creature card in your graveyard during their untap steps."
||| [CR#502.3]'s untap reaches the permanents a player controls, so the capped
||| set describes the battlefield and a graveyard phrase contradicts it.
public export
badUntapCapGraveyardSet : Unspellable Ability (\ok =>
  Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1
                            (And [Macros.creature, InZone (Macros.graveyardOf You)]) {zn = ok}))
badUntapCapGraveyardSet MkZoneFits impossible


||| "Players can't untap more than one tapped during their untap steps."
||| The capped set writes a head noun; the status word is a modifier and heads
||| nothing, exactly as a determiner slot refuses it.
public export
badUntapCapHeadless : Unspellable Ability (\ok =>
  Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.tapped {hd = ok}))
badUntapCapHeadless MkHeaded impossible


||| "Players can't untap more than one land until your next untap step."
||| The cap names the interval it governs in its own words and takes no
||| ENDPOINT: no line in the family writes "next", and no duration adverbial
||| ends at an untap step.
public export
badUntapCapUntilNextUntapStep : Unspellable (Effect []) (\ok =>
  Continuously (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.land)
               (Just (Until (StartOf UntapStep (Just Yours)))) {sp = ok})
badUntapCapUntilNextUntapStep SpanStated impossible


||| "Put a poison counter on target creature."
||| [CR#122.1] places a counter on an object OR a player and the two never
||| cross: the poison kind is a player's, so the put verb refuses it.
public export
badPutPoisonOnCreature : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) Poison (Macros.target Macros.creature) {sc = ok})
badPutPoisonOnCreature Refl impossible


||| "You get a +1/+1 counter."
||| The same table read the other way: the stat counter is an object's
||| [CR#122.1a], so the player verb refuses it.
public export
badGetsBoostCounter : Unspellable (Effect []) (\ok =>
  GetsCounters You (Lit 1) Macros.plusOnePlusOne {sc = ok})
badGetsBoostCounter Refl impossible


||| "Each opponent loses all +1/+1 counters."
||| A named kind on the player's removal verb must be a player's; the
||| unnamed cell is what "loses all counters" writes instead.
public export
badLosesAllBoostCounters : Unspellable (Effect []) (\ok =>
  LosesAllCounters (Each Opponent) (Just Macros.plusOnePlusOne)
                   {pk = OneKindLost {sc = ok}})
badLosesAllBoostCounters Refl impossible


||| "the number of +1/+1 counters you have"
||| The read agrees with its holder's sort: a stat counter sits on an
||| object [CR#122.1a], so no player phrase can hold one.
public export
badCountersHeldByPlayer : Unspellable (Amount []) (\ok =>
  CountersOn Macros.plusOnePlusOne You {sc = ok})
badCountersHeldByPlayer Refl impossible


||| "When the last poison counter is removed from this creature, draw a card."
||| The last-removal event watches an OBJECT's holding; the poison kind is
||| a player's and no player can be its subject at all.
public export
badLastPoisonCounterRemoved : Unspellable Ability (\ok =>
  Triggered When (LastCounterRemoved Poison Macros.thisCreature {sc = ok}) Macros.drawACard)
badLastPoisonCounterRemoved Refl impossible


||| "At the last time counter is removed from this card, draw a card."
||| [CR#603.2b] keeps "at" for phases and steps, as it does for every other
||| object event.
public export
badAtLastCounterRemoved : Unspellable Ability (\ok =>
  Triggered At (LastCounterRemoved Time Macros.thisCreature) Macros.drawACard {wo = ok})
badAtLastCounterRemoved MkTriggerWordOk impossible


||| "When the last time counter is removed from this card, if this creature is exiled, draw a card."
||| A phrase that places its referent places it: the sorted self-word seeds the
||| battlefield [CR#109.2], so it cannot be asked whether it is somewhere else.
||| The BARE self-word states no zone and is what a zone check reads.
public export
badExileCheckOnSortedSelf : Unspellable Ability (\ok =>
  Triggered When (LastCounterRemoved Time Macros.thisCreature) Macros.drawACard
            {intervening = Just (Matches Macros.thisCreature (InZone Macros.exileZ)
                                         {zc = ok})})
badExileCheckOnSortedSelf MkZoneFits impossible


||| "When this creature enters, if you died this turn, draw a card."
||| Each event names the sort its history subject takes, and dying is an
||| object's: the death read is written over creatures and never over a player.
public export
badLookbackPlayerDied : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature) Macros.drawACard
            {intervening = Just (Happened Death You Lookback.ThisTurn {sb = ok})})
badLookbackPlayerDied MkLookbackSubject impossible


||| "When this creature enters, if a creature cast a spell this turn, draw a card."
||| The same table the other way: casting is read over a player [CR#601.2], and
||| the corpus writes "you've cast" and "a player cast" and no object subject.
public export
badLookbackObjectCast : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature) Macros.drawACard
            {intervening = Just (Happened SpellCast (Macros.a Macros.creature)
                                          Lookback.ThisTurn {sb = ok})})
badLookbackObjectCast MkLookbackSubject impossible


||| "When this creature enters, if an upkeep began this turn, draw a card."
||| A turn-part beginning is not read as history: [CR#603.2b] gives it a trigger
||| header and the lookback position is written zero times.
public export
badLookbackPartBeginning : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature) Macros.drawACard
            {intervening = Just (Happened PartBeginning (Macros.a Macros.creature)
                                          Lookback.ThisTurn {sb = ok})})
badLookbackPartBeginning MkLookbackSubject impossible


||| "target creature who cast a spell this turn"
||| The head noun IS the history read's subject, so the phrase's own sort has
||| to be one the event takes: casting is a player's [CR#601.2] and a creature
||| head cannot ask it.
public export
badHappenedToObjectCast : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, HappenedTo SpellCast Lookback.ThisTurn {sb = ok}]))
badHappenedToObjectCast MkLookbackSubject impossible


||| "each opponent who died this turn"
||| The same table refusing at its SECOND reader: dying is an object's event,
||| and a player head cannot ask it here any more than a player subject could
||| ask it in the condition position.
public export
badHappenedToPlayerDied : Unspellable (Noun [] Player) (\ok =>
  Each (And [Opponent, HappenedTo Death Lookback.ThisTurn {sb = ok}]))
badHappenedToPlayerDied MkLookbackSubject impossible


||| "target colorless white creature"
||| [CR#105.2c] gives a colorless object "no color", so the two words describe
||| nothing together — the status pair's contradiction at the one other place a
||| closed value pair has one.
public export
badColorlessWhite : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, IsColorless, ColorIs White] {cf = ok}))
badColorlessWhite MkContradictionFree impossible


||| "target nonmulticolored permanent"
||| The three colour-COUNT words take no prefix negation: "nonmulticolored",
||| "noncolorless" and "nonmonocolored" are zero corpus lines apiece, where the
||| five colour words and the supertypes are negated in quantity.
public export
badNonMulticolored : Unspellable (Predicate [] Object) (\ok =>
  Not Multicolored {ng = ok})
badNonMulticolored MkNegatable impossible


||| "At the beginning of an opponent's upkeep, draw a card."
||| The two part-and-possessor tables DISAGREE here and the pin is that
||| disagreement: the header writes this possessor zero times, where the
||| activation window writes it ("Activate only during an opponent's upkeep").
public export
badTriggerAtAnOpponentsUpkeep : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Upkeep (Just AnOpponents) {pu = ok}) Macros.drawACard)
badTriggerAtAnOpponentsUpkeep MkPartTriggerable impossible


||| "At the beginning of each opponent's first main phase, draw a card."
||| The opponent quantifier is written at the upkeep and the end step and at
||| no main phase: a new part does not inherit a possessor's cells.
public export
badTriggerAtEachOpponentsFirstMain : Unspellable Ability (\ok =>
  Triggered At (BeginningOf FirstMain (Just EachOpponents) {pu = ok}) Macros.drawACard)
badTriggerAtEachOpponentsFirstMain MkPartTriggerable impossible


||| "{2}: Draw a card. Activate only during your end step."
||| The window's own table refusing: the end step is a trigger header's part
||| (359 lines) and no activation restriction names it.
public export
badWindowDuringYourEndStep : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) Macros.drawACard
            {window = Just (DuringPart EndStep (Just Yours) {wk = ok})})
badWindowDuringYourEndStep MkWindowOk impossible


||| "Target land attacks each combat if able."
||| The SAME table refusing at the other polarity: only a creature attacks
||| [CR#506.3], and the requirement reads the restriction's own grid rather
||| than a mirror of it — this pin and `badCantAttackLand` share one P, which
||| is the claim stated as a proof.
public export
badMustAttackLand : Unspellable (Effect []) (\ok =>
  Continuously (Deontic (Macros.target Macros.land) Require Attack Agent Nothing {dp = ok})
               (Just Macros.thisTurn))
badMustAttackLand Participant impossible


||| "This creature attacks target creature each combat if able."
||| The attack requirement names no defender: goad's "attacks a player other
||| than you" leg belongs to a DESIGNATION [CR#701.15b] — "neither an ability
||| nor part of the permanent's copiable values" — and to the designations
||| family, so the cell refuses a patient outright.
public export
badMustAttackWithPatient : Unspellable Ability (\ok =>
  Static (Deontic Macros.thisCreature Require Attack Agent (Just (Macros.target Macros.creature))
                  {pt = DeonticPatientWritten {ok = ok}}))
badMustAttackWithPatient Refl impossible


||| "Target creature blocks this turn if able."
||| The other direction of the same table: the block requirement REQUIRES its
||| patient, all 38 lines naming what must be blocked, so the bare form is not
||| English.
public export
badMustBlockNoPatient : Unspellable (Effect []) (\ok =>
  Continuously (Deontic (Macros.target Macros.creature) Require Block Agent Nothing
                        {pt = NoDeonticPatient {ok = ok}})
               (Just Macros.thisTurn))
badMustBlockNoPatient Refl impossible


||| "Target artifact may choose not to untap during your untap step this turn."
||| The permission to decline is a printed static ability and never a clause:
||| all seven lines are card text with no duration adverbial, which is the
||| restriction class's own arrangement.
public export
badDeclineUntapClause : Unspellable (Effect []) (\ok =>
  Continuously (MayDeclineUntap (Macros.target Macros.artifact)) Nothing {sp = ok})
badDeclineUntapClause SpanUnstated impossible


||| "At the beginning of your end step, if it's day, draw a card."
||| The game's two designations are written asymmetrically: the CHECK is four
||| lines of "it's night" and zero of "it's day", where the transition writes
||| both directions freely [CR#731.1].
public export
badItIsDay : Unspellable Ability (\ok =>
  Triggered At (BeginningOf EndStep (Just Yours)) Macros.drawACard
            {intervening = Just (ItIsNow Day {tc = ok})})
badItIsDay MkTimeChecked impossible


||| "if you aren't the monarch"
||| The designation read does not negate: "isn't the monarch" is zero lines,
||| and the one negative the family writes — "there is no monarch" — asks
||| whether ANY player holds it, which is another construction.
public export
badNegatedMonarch : Unspellable (Predicate [] Player) (\ok =>
  Not (HasPlayerDesignation Monarch) {ng = ok})
badNegatedMonarch MkNegatable impossible


||| "target creature card in your graveyard that is your Ring-bearer"
||| [CR#701.54e] builds the read out of three conjuncts and the first is the
||| battlefield, so the phrase seeds it and a graveyard clause contradicts.
public export
badRingBearerInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, YourRingBearer, InZone (Macros.graveyardOf You)] {zc = ok})
badRingBearerInGraveyard MkZoneCoherent impossible


||| "Whenever this creature attacks, that creature gets +2/+0 until end of turn."
||| English's demonstratives skip the speaker: "that creature" never picks out
||| the trigger's own subject, so the mention the event mints is visible to "it"
||| and invisible here — and the corpus writes this sentence zero times.
public export
badThatCreatureIsSelf : Unspellable Ability (\ok =>
  Triggered Whenever (Attacks Macros.thisCreature)
            (Macros.gets (That (TypeW Creature) {ok = ok}) (PtUp (Lit 2)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn)))
badThatCreatureIsSelf Refl impossible


||| "This creature can't attack target creature this turn."
||| The restriction names no defender in the patient slot: "can't attack you"
||| writes a DEFENDING PLAYER, a different participant with no noun here, and
||| no line writes an object there at all.
public export
badForbidAttackWithPatient : Unspellable (Effect []) (\ok =>
  Continuously (Deontic Macros.thisCreature Forbid Attack Agent
                        (Just (Macros.target Macros.creature))
                        {pt = DeonticPatientWritten {ok = ok}})
               (Just Macros.thisTurn))
badForbidAttackWithPatient Refl impossible


||| "This creature can't be blocked by target creature unless you pay {1}."
||| The gate's patient cell is the block AGENT's alone: the four
||| can't-be-blocked-unless lines name no blocker, writing the defending
||| player's payment instead.
public export
badGateBlockPatientWithPatient : Unspellable Ability (\ok =>
  Static (Deontic Macros.thisCreature (GatedBy (Mana [Macros.generic 1]))
                  Block Patient (Just (Macros.target Macros.creature))
                  {pt = DeonticPatientWritten {ok = ok}}))
badGateBlockPatientWithPatient Refl impossible


||| "As long as this creature is attacking, that creature gets +2/+0."
||| The demonstrative screen at the second minting site: "that creature" never
||| picks out the sentence's own subject, so the mention the container mints is
||| visible to "it" and invisible here — and the corpus writes this sentence
||| zero times, exactly as it writes zero of the trigger-header twin.
public export
badThatCreatureIsCondSubject : Unspellable Ability (\ok =>
  Static (Macros.asLongAs (Matches Macros.thisCreature Attacking)
                          (Gets (That (TypeW Creature) {ok = ok}) (PtUp (Lit 2)) (PtUp (Lit 0)))))
badThatCreatureIsCondSubject Refl impossible


||| "Equipped land gets +1/+1."
||| [CR#301.5a] names one host for an Equipment and it is a creature; the
||| land-hosting attachment is a Fortification and writes its own participle.
public export
badEquippedLand : Unspellable Ability (\ok =>
  Static (Gets (AttachHost Equipped (TypeW Land) {ok = ok}) (PtUp (Lit 1)) (PtUp (Lit 1))))
badEquippedLand MkAttachHeadOk impossible


||| "Fortified creature gets +1/+1."
||| The mirror: [CR#301.6] applies the Equipment rules "to Fortifications in
||| relation to LANDS", so the fortified participle takes no creature head.
public export
badFortifiedCreature : Unspellable Ability (\ok =>
  Static (Gets (AttachHost Fortified (TypeW Creature) {ok = ok}) (PtUp (Lit 1)) (PtUp (Lit 1))))
badFortifiedCreature MkAttachHeadOk impossible


||| "Target player can't lose the game."
||| The gate is a printed static line and a static line announces no target
||| ([CR#115.1d] chooses targets as an ability goes on the stack, and a static
||| never does); the corpus writes the subject as "you", "players" or "your
||| opponents" and never as a target.
public export
badTargetedOutcomeGate : Unspellable Ability (\ok =>
  Static (OutcomeGate CantLose (Macros.target AnyPlayer)) {ut = ok})
badTargetedOutcomeGate MkUntargeting impossible


||| "You lose the game: Draw a card."
||| The outcome is never a COST. [CR#104.3e] has an effect STATE that a player
||| loses; a cost is something a player pays to do a thing, and no line prices
||| an ability at the game.
public export
badConcludesAsCost : Unspellable Ability (\ok =>
  Activated (Do (Concludes LoseGame You) {ok = ok}) Macros.drawACard)
badConcludesAsCost MkCostAction impossible


||| "Each player can't untap more than one creature during their untap step."
||| The cap's subject is closed at the three phrases its seven lines write —
||| the bare plural, "you" and "your opponents" — and a distributive
||| quantifier is none of them.
public export
badUntapCapEachPlayer : Unspellable Ability (\ok =>
  Static (CantUntapMoreThan (Each AnyPlayer) 1 Macros.creature {cs = ok}))
badUntapCapEachPlayer MkCapSubject impossible


||| "This creature deals 2 damage to your opponents."
||| One magnitude and one recipient phrase, and a bare plural says neither
||| whether it is each member's nor the group's; the corpus deals to "each
||| opponent" and writes this phrase only as a trigger's condition.
public export
badPluralPlayerDamageRecipient : Unspellable (Effect []) (\ok =>
  DealDamage Macros.thisCreature (Lit 2) (PlayerGroup YourOpponents) {pm = ok})
badPluralPlayerDamageRecipient MkPerMember impossible


||| "creatures players control"
||| The possessor set is written as "your opponents" and never as the bare
||| plural: "players control" is zero relative-clause lines, the corpus saying
||| it with "each player" or with no possessor at all.
public export
badControlledByAllPlayers : Unspellable (Predicate [] Object) (\ok =>
  ControlledBy (PlayerGroup AllPlayers) {ps = ok})
badControlledByAllPlayers MkPossessor impossible


||| "cards in players' graveyards"
||| The possessive reader answers from the same table as the control clause and
||| refuses the same word: the bare plural possessive is zero lines, written
||| "all players' hands" or "each player's graveyard" instead.
public export
badOwnedByAllPlayers : Unspellable (ZoneExpr []) (\ok =>
  Macros.graveyardOf (PlayerGroup AllPlayers) {pn = ok})
badOwnedByAllPlayers MkPossessor impossible


||| "At a player losing the game, put five +1/+1 counters on this creature."
||| [CR#603.2b] fixes `At` to a turn-part beginning, and the eight watching
||| lines write only the other two words — five `Whenever`, two `When`.
public export
badGameLossAtTrigger : Unspellable Ability (\ok =>
  Triggered At (LosesGame (Macros.a AnyPlayer))
            (PutCounters (Lit 5) Macros.plusOnePlusOne Macros.thisCreature) {wo = ok})
badGameLossAtTrigger MkTriggerWordOk impossible


||| "if a player has lost the game this turn"
||| The loss is watched and replaced, never QUERIED: no line asks whether a
||| player lost inside a window, and the two lines that look back at losses
||| count players in a standing state instead ([CR#603.10f] makes the trigger
||| itself look back, which is a different mechanism from this query).
public export
badGameLossLookback : Unspellable (Condition []) (\ok =>
  Happened GameLoss (Macros.a AnyPlayer) Lookback.ThisTurn {sb = ok})
badGameLossLookback MkLookbackSubject impossible


||| "When a player next loses the game this turn, draw a card."
||| The delayed form is unwritten: the corpus's only "next" over this event is
||| the REPLACEMENT's multiplicity ("the next time you would lose the game"),
||| which bounds how often an instead applies and does not wait for an event.
public export
badDelayedGameLoss : Unspellable (Effect []) (\ok =>
  Delayed (LosesGame You) (Draw You (Lit 1)) {aw = ok})
badDelayedGameLoss MkAwaitable impossible


||| "You gain life equal to your opponents' life totals."
||| One player has one life total, so the read is singular. The plural phrase
||| occurs eight times and every one is the EXCHANGE ([CR#701.12c]), which
||| swaps two totals and reads neither as an amount.
public export
badPluralLifeTotalRead : Unspellable (Amount []) (\ok =>
  PlayerStatOf LifeTotal (PlayerGroup YourOpponents) {one = ok})
badPluralLifeTotalRead Refl impossible


||| "You gain life equal to each player's life total."
||| The distributive names no one player, so there is no total to read. It may
||| still RECEIVE a set — "each player's life total becomes …" is written — and
||| that is a different slot from this one.
public export
badDistributiveLifeTotalRead : Unspellable (Amount []) (\ok =>
  PlayerStatOf LifeTotal (Each AnyPlayer) {one = ok})
badDistributiveLifeTotalRead Refl impossible


||| "with power less than or equal to twice this creature's power"
||| A comparison's standard is something already there to be pointed at, and a
||| scaled product is an amount a clause computes for an instruction.
public export
badScaledBound : Unspellable (Predicate [] Object) (\ok =>
  Compare Power AtMost (Times 2 (Macros.powerOf This)) {cb = ok})
badScaledBound MkComparableBound impossible


||| "if your life total is less than or equal to twice an opponent's life total"
||| The same refusal at the condition frame, which reads the one bound table:
||| the scaled product, the sum and the outcome read are bounds zero times.
public export
badScaledConditionBound : Unspellable (Condition []) (\ok =>
  CompareAmt (PlayerStatOf LifeTotal You) AtMost
             (Times 2 (PlayerStatOf LifeTotal Macros.anOpponent)) {cb = ok})
badScaledConditionBound MkComparableBound impossible


||| "Draw cards equal to the difference."
||| The margin is read off a comparison, and a sentence that made none has no
||| difference to name.
public export
badUnlicensedDifference : Unspellable (Effect []) (\ok =>
  Draw You (TheDifference {ok}))
badUnlicensedDifference Refl impossible


||| "When this creature enters, if you control a creature, draw cards equal to
||| the difference."
||| A condition that holds by no AMOUNT leaves no margin; only a comparison
||| does, which is why the licence is the comparison and not the "if".
public export
badNonComparisonDifference : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature)
            (Draw You (TheDifference {ok}))
            {intervening = Just (Exists Macros.creatureYouControl)})
badNonComparisonDifference Refl impossible


||| "At the beginning of your upkeep, if you have fewer than seven cards in
||| hand, draw a card if the difference is 3 or greater."
||| The margin is a magnitude a clause READS, never a state a clause measures;
||| what oracle compares is the explicit "difference between" phrase.
public export
badDifferenceSubject : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Upkeep (Just Yours))
            (If Macros.drawACard
                (CompareAmt (TheDifference {ok = Refl}) AtLeast (Lit 3) {rd = ok})
                Nothing)
            {intervening = Just (CompareAmt (CountOf (InZone (Macros.handOf You)))
                                            Less (Lit 7))})
badDifferenceSubject MkReadAmount impossible


||| "Draw X cards."
||| The letter is a name the sentence introduced; a sentence that defined no
||| letter has no X to say [CR#107.3c].
public export
badUnlicensedX : Unspellable (Effect []) (\ok =>
  Draw You (DefinedLetter LetterX {ok}))
badUnlicensedX Refl impossible


||| "Draw X cards, where X is 4."
||| A rider is worth writing when the value has to be worked out; a written
||| value would be written, and oracle writes the numeral instead.
public export
badWrittenXDef : Unspellable (Effect []) (\ok =>
  WhereLetter LetterX (Lit 4) {xd = ok} (Draw You (DefinedLetter LetterX {ok = Refl})))
badWrittenXDef MkLetterDefinition impossible


||| "Draw X cards, where X is the number of creatures you control, where X is
||| the number of creatures on the battlefield."
||| One ability defines one letter: the seven cards that write two riders give
||| each its own ability, and a letter defined twice names nothing.
public export
badDoubleXRider : Unspellable (Effect []) (\ok =>
  WhereLetter LetterX (CountOf Macros.creatureYouControl)
              (WhereLetter LetterX (CountOf Macros.creature)
                           (Draw You (DefinedLetter LetterX {ok}))))
badDoubleXRider Refl impossible


||| "Target creature gets +0/-2 until end of turn."
||| A written zero is signless in meaning, so the slot takes its partner's
||| sign and only that: oracle writes "-0/-2" [CR#613.4c].
public export
badDisagreeingZeroPump : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 0)) (PtDown (Lit 2))
       (Just Macros.untilEndOfTurn) {ps = ok})
badDisagreeingZeroPump MkPumpSigns impossible


||| "Draw Y cards, where X is the number of creatures you control."
||| Y is a name of its own and follows X's rules, not X's definition
||| [CR#107.3p]; a sentence that named no Y has no Y to say.
public export
badUnlicensedY : Unspellable (Effect []) (\ok =>
  WhereLetter LetterX (CountOf Macros.creatureYouControl)
              (Draw You (DefinedLetter LetterY {ok})))
badUnlicensedY Refl impossible


||| "Target creature gets +X/+X, where X is the number of creatures you
||| control, until end of turn."
||| The adverbial belongs to the clause and the rider binds the whole clause:
||| 245 pump lines write the duration first and none writes it after the rider,
||| so the inside form is one sentence's second spelling [CR#611.2a].
public export
badRiderInsideDuration : Unspellable (Effect []) (\ok =>
  Continuously (WhereLetterStatic LetterX (CountOf Macros.creatureYouControl)
                                  (Gets (Macros.target Macros.creature)
                                        (PtUp (DefinedLetter LetterX))
                                        (PtUp (DefinedLetter LetterX))))
               (Just Macros.untilEndOfTurn) {nr = ok})
badRiderInsideDuration MkNotLetterRider impossible


||| "This creature gets +X/+0, where X is the number of creatures you control,
||| where X is the number of creatures on the battlefield."
||| One statement defines one letter exactly as one instruction does, and a
||| letter defined twice names nothing [CR#107.3].
public export
badDoubleStaticRider : Unspellable (StaticEffect []) (\ok =>
  WhereLetterStatic LetterX (CountOf Macros.creatureYouControl)
                    (WhereLetterStatic LetterX (CountOf Macros.creature)
                                       (Gets Macros.thisCreature
                                             (PtUp (DefinedLetter LetterX {ok}))
                                             (PtUp (Lit 0)))))
badDoubleStaticRider Refl impossible


||| "the greatest power among players"
||| Power is a creature card's own number [CR#208.1], so the fold's axis
||| and its domain must agree in sort; no line reads power off a player.
public export
badPowerAmongPlayers : Unspellable (Amount []) (\ok =>
  Aggregate MaxOf (CharAxis Power) AnyPlayer {sc = ok})
badPowerAmongPlayers Refl impossible


||| "the highest life total among creatures you control"
||| The same table read the other way: a life total is something each
||| PLAYER has [CR#119.1], so an object-sorted domain refuses that axis.
public export
badLifeTotalAmongObjects : Unspellable (Amount []) (\ok =>
  Aggregate MaxOf (PlayerStatAxis LifeTotal) Macros.creatureYouControl {sc = ok})
badLifeTotalAmongObjects Refl impossible


||| "Creatures you control cost {1} less to cast."
||| A cost statement is about a SPELL [CR#609.2], and a battlefield noun is
||| not one; 615 of the family's 628 subjects write the word itself.
public export
badCostSubjectOnBattlefield : Unspellable (StaticEffect []) (\ok =>
  CostsToCast (AllOf Macros.creatureYouControl) (CostLess (Lit 1)) {cs = ok})
badCostSubjectOnBattlefield MkCostSubject impossible


||| "This spell costs {0} less to cast."
||| A reduction English writes is at least one: "{0}" is the payment of
||| nothing [CR#118.5], and a reduction of it moves no cost.
public export
badZeroCostShift : Unspellable (StaticEffect []) (\ok =>
  CostsToCast This (CostLess (Lit 0)) {wc = ok})
badZeroCostShift MkWrittenCount impossible


||| "Spells cost {1} less to cast." written as a resolving clause
||| Every cost statement in the corpus is a static ability's own line, so
||| there is no clause for its span to be absent from [CR#604.1].
public export
badCostClause : Unspellable (Effect []) (\ok =>
  Continuously (CostsToCast This (CostLess (Lit 1))) Nothing {sp = ok})
badCostClause SpanUnstated impossible


||| "a creature card you cast in your graveyard"
||| The cast relation puts its referent on the stack [CR#601.2a,112.1]; the
||| zone a spell was cast FROM is the origin qualifier and not where it stands.
public export
badCastInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, CastBy You, InZone Macros.graveyardZ] {zc = ok})
badCastInGraveyard MkZoneCoherent impossible


||| "spells players cast"
||| The cast clause answers the possessor table with the other two readers and
||| refuses the same word: the bare plural is zero lines, written "your
||| opponents" or with no possessor at all.
public export
badCastByAllPlayers : Unspellable (Predicate [] Object) (\ok =>
  CastBy (PlayerGroup AllPlayers) {ps = ok})
badCastByAllPlayers MkPossessor impossible
