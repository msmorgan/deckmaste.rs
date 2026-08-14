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
       [Static (Gets (AllOf Macros.creatureYouControl) 1 1)] Nothing {tx = ok})
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
                        (Cant Macros.thisCreature Attack Agent)
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
  Macros.gets (Macros.target Macros.creature) 1 1 (Just (Until (StartOf EndOfCombat Nothing))) {sp = ok})
badUntilEndOfCombatStep SpanStated impossible


||| "Destroy target creature. Exile the destroyed card."
||| The participle's two surfaces are not interchangeable per verb: the destroy
||| row writes the deictic only, "destroyed this way".
public export
badDestroyedAttributive : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                Macros.exile (TheVerbed Destroy CardW {marking = Attributive} {mk = ok})])
badDestroyedAttributive MkVerbedMarkingOk impossible


||| "target spell you control"
||| Real English, [CR#109.4] giving stack objects a controller too, but the
||| relation seeds the BATTLEFIELD where the phrase constrains to two zones.
public export
badControlledSpell : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.spell, ControlledBy You] {zc = ok})
badControlledSpell MkZoneCoherent impossible


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
  Macros.gets (Macros.target Macros.creature) 2 2
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
