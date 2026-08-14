module Experimental.ProofsD

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off

-- Continuation of Experimental.ProofsC; see Unspellable there.


-- …and the battlefield type word is refused by the same table, which is
-- what keeps "a creature card exiled with this creature" (real, and
-- Sisters of Stone Death's own phrase) apart from "an attacking creature
-- exiled with this creature" (not): the card word travels with the
-- linkage, the status word does not.
public export
badExiledWithAttacking : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, Macros.exiledWithThisArtifact] {zc = ok})
badExiledWithAttacking MkZoneCoherent impossible


-- The linkage read does not NEGATE. "Not exiled with" is zero corpus
-- lines against a hundred seventy-five positive ones, and the reason is
-- what the phrase is for: a source-keyed group is named to be acted on,
-- and the cards outside it are described by another zone or another
-- phrase rather than by this one turned inside out.
public export
badNegatedExiledWith : Unspellable (Predicate [] Object) (\ok =>
  Not Macros.exiledWithThisArtifact {ng = ok})
badNegatedExiledWith MkNegatable impossible


-- A permanent card's text is never a SPELL ability. [CR#113.3a] defines
-- the category by when it is followed — "while an instant or sorcery
-- spell is resolving" — and a creature card's text is never that. This
-- is the container's central gate seen from the side the rule states.
public export
badSpellAbilityOnPermanent : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature])
       [Spell Macros.drawACard] (Just (1, 1)) {tx = ok})
badSpellAbilityOnPermanent MkCardText impossible


-- …and a spell card's text is not a STATIC ability, on the same rule's
-- other clause: [CR#113.3a] admits one only if it "fits the criteria
-- described" [CR#113.6], and that rule says abilities of an instant or
-- sorcery "usually function only while that object is on the stack"
-- where every `StaticEffect` row here establishes a continuous effect on
-- the battlefield.
public export
badStaticOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Sorcery])
       [Static (Gets (AllOf Macros.creatureYouControl) 1 1)] Nothing {tx = ok})
badStaticOnSorcery MkCardText impossible


-- The keyword row is shut on a spell card by MEASUREMENT rather than by
-- rule: the seven keywords this file carries are all [CR#702] abilities of
-- a permanent in combat, and no instant or sorcery in the supported corpus
-- is printed with one as a bare line — zero, for all seven.
public export
badKeywordOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Instant])
       [KeywordAbility Flying] Nothing {tx = ok})
badKeywordOnInstant MkCardText impossible


-- The ACTIVATED row is open on a spell card — cycling is one, printed on
-- sorceries — but not for every cost: [CR#113.6j] lets an activated
-- ability function off the battlefield exactly when its cost can be paid
-- there, and [CR#110.4] never puts an instant or sorcery card on the
-- battlefield. "{T}" means "Tap this permanent" [CR#107.5], so the line
-- is one no sorcery could ever activate, and zero Instant or Sorcery
-- cards print one. (`cycling`, whose cost is a symbol run and a discard,
-- is the shape that passes.)
public export
badTapSorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Tap Sorcery" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [Activated TapSymbol Macros.drawACard] Nothing {tx = ok})
badTapSorcery MkCardText impossible


-- A creature card writes its two numbers ([CR#208.1] — "a creature card
-- has two numbers separated by a slash printed in its lower right
-- corner"), which is `tokenPtOk`'s demand at the printed card.
public export
badCreatureCardNoPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) [] Nothing {pts = ok})
badCreatureCardNoPt MkCardPt impossible


-- A land card writes NO mana cost ([CR#202.1b]: "some objects have no
-- mana cost. This normally includes all land cards"), the absence being
-- an unpayable cost [CR#118.6] rather than an omission.
public export
badLandWithManaCost : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Land]) [] Nothing {mc = ok})
badLandWithManaCost MkCardCost impossible


-- The SUPERTYPE field carried no witness at all, so a word could be
-- printed twice. [CR#205.4b] makes a supertype a property an object HAS
-- or LACKS — one that "gains or loses a supertype … retains any OTHER
-- supertypes it had" — so "Legendary Legendary Creature" is one fact
-- written twice, the colors' refusal at the catalog list beside it.
public export
badDuplicateSupertype : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [Legendary, Legendary] (MkTypeLine [] [Creature])
       [] (Just (1, 1)) {sp = ok})
badDuplicateSupertype MkCardSupers impossible


-- A permanent type and a spell type do not share a line. [CR#110.4]
-- says "instant and sorcery cards can't enter the battlefield and thus
-- can't be permanents" and [CR#110.4a] lists the six that can, so this
-- line names a card that would have to be a permanent and not be one.
-- The ORDER check could not catch it, and that is the point: the ranks
-- put the spell types last, so [Land, Creature, Instant] ascends
-- perfectly and combination legality is a second question.
public export
badMixedPermanentSpellLine : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land, Creature, Instant]) [] (Just (1, 1)) {ln = ok})
badMixedPermanentSpellLine MkCardLine impossible


-- A card's type line says SOMETHING — [CR#205.1] has it contain "the
-- card's card type(s)" without qualification, where the subtypes and
-- supertypes are there "if applicable".
public export
badCardNoTypes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] []) [] Nothing {ln = ok})
badCardNoTypes MkCardLine impossible


-- …and it writes them in the printed ORDER, which is `typesOrdered`'s
-- rank at its third reader: "artifact creature" is five hundred
-- ninety-four lines and "creature artifact" none.
public export
badCardTypeOrder : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) [] (MkTypeLine [] [Creature, Artifact]) []
       (Just (2, 2)) {ln = ok})
badCardTypeOrder MkCardLine impossible


-- A clause cannot GRANT a spell ability, and this refusal is a category
-- error rather than the quotation gap its three siblings carry: a
-- `Gains` clause grants to a permanent on the battlefield and a spell
-- ability is something an instant or sorcery spell has while it resolves
-- ([CR#113.3a]).
public export
badGainsSpellAbility : Unspellable (Effect []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (Spell Macros.drawACard) Nothing {gr = ok})
badGainsSpellAbility MkGrantable impossible


-- The countering's complement is a SPELL, and the zone is what says so
-- ([CR#112.1] — "a spell is a card on the stack"). A battlefield
-- permanent has already resolved and there is nothing left to cancel.
public export
badCounterPermanent : Unspellable (Effect []) (\ok =>
  Macros.counterSpell (Macros.target Macros.creature) {zn = ok})
badCounterPermanent OnTheStack impossible


-- Cancelling somebody else's spell is not a PAYMENT. [CR#602.1a] makes
-- an activation cost what the activator pays, and the cost table's
-- measurement is the same one finding 159 made of destroy: zero corpus
-- lines write a counter before a colon.
public export
badCounterAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.counterSpell (Macros.target Macros.spell)) {ok}) Macros.drawACard)
badCounterAsCost MkCostAction impossible


-- The stack is not a place one plays a card FROM: [CR#112.1] makes an
-- object there a spell, and a spell has already been cast. That is the
-- battlefield refusal one step earlier.
public export
badPlayFromStack : Unspellable (Effect []) (\ok =>
  Continuously (MayPlay You (Macros.a Macros.spell) {pz = ok}) (Just Macros.thisTurn))
badPlayFromStack MkPlaySource impossible


-- "Cast" excludes the LAND, and the rule is [CR#305.9]: "if an object is
-- both a land and another card type, it can be played only as a land. It
-- can't be cast as a spell." The general verb is what those lines write
-- ("You may play lands from your graveyard").
public export
badCastALand : Unspellable (Effect []) (\ok =>
  Continuously (MayPlay You (Macros.a (And [Macros.land, InZone (Macros.graveyardOf You)]))
                           {verb = Cast} {cv = ok})
               (Just Macros.thisTurn))
badCastALand MkCastableTy impossible


-- A written source phrase must AGREE with what the complement already
-- says: the permission's two ways of naming a zone are one fact, and
-- `zoneFits` is the same silence-is-no-evidence reading every other
-- zone demand uses.
public export
badPlayFromWrongZone : Unspellable (Effect []) (\ok =>
  Continuously (MayPlay You (Macros.a (And [Macros.creature, InZone Macros.exileZ]))
                           {from = Just (Macros.graveyardOf You)} {pz = ok})
               (Just Macros.thisTurn))
badPlayFromWrongZone MkPlaySource impossible


-- Nothing MOVES to the stack. [CR#601.2a] puts a card there as the first
-- step of casting it, which is an action a player takes and not a
-- placement a sentence writes — core excludes the same destination by
-- name (`Destination`'s `exclude(Library, Stack)`).
public export
badMoveToStack : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) Macros.stackZ {ok})
badMoveToStack BattlefieldOk impossible


-- The battlefield destination asks about its PATIENT and not only about
-- its own phrase: [CR#110.4] says "instant and sorcery cards can't enter
-- the battlefield and thus can't be permanents" and [CR#110.4a] lists
-- the six types that can, so a graveyard instant has no placement.
-- (`Placeable` reads the PROJECTED head type, so the untyped phrase
-- still places — Oblivion Ring's "return the exiled card to the
-- battlefield" writes no type word, and over-refusal is the one
-- direction these gates may not err in.)
public export
badInstantOntoBattlefield : Unspellable (Effect []) (\ok =>
  Macros.putOntoBattlefieldTapped (Macros.target (And [HasType Instant, InZone Macros.graveyardZ])) {pl = ok})
badInstantOntoBattlefield MkPlaceable impossible


-- …and no cost component places anything on the battlefield. Every
-- zone-change verb the corpus writes before a colon REMOVES or
-- DOWNGRADES — sacrifice three hundred forty-one components, discard
-- sixty-eight, exile twenty-eight, "Return … to its owner's hand" nine,
-- and the six "Put …" costs name a graveyard, the top of a library or a
-- counter — against zero battlefield entries. [CR#601.2h] pays the cost
-- to activate; a placement is what the ability buys.
public export
badMoveOntoBattlefieldAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.putOntoBattlefield (Macros.a (And [Macros.creature, InZone (Macros.graveyardOf You)]))) {ok})
            Macros.drawACard)
badMoveOntoBattlefieldAsCost MkCostAction impossible


-- "Unless" IS the negation, so it takes a negated condition and spells
-- the positive underneath. A positive condition under the word would be
-- the negation written twice, which no line writes.
public export
badUnlessOnPositive : Unspellable Ability (\ok =>
  Static (Conditionally (Exists (And [Macros.artifact, ControlledBy You]))
                        (Cant Macros.thisCreature Attack Agent)
                        {marking = Unless} {mk = ok}))
badUnlessOnPositive MkMarkingOk impossible


-- The end-of-combat header takes no POSSESSOR. Zero lines write "at your
-- end of combat" or its neighbours; the four "end of combat on your …"
-- lines are [CR#511.2]'s other reading, the phase-endpoint duration that
-- `EndOf Combat` already spells.
public export
badTriggerAtYourEndOfCombat : Unspellable Ability (\ok =>
  Triggered At (BeginningOf EndOfCombat (Just Yours) {pu = ok}) Macros.drawACard)
badTriggerAtYourEndOfCombat MkPartTriggerable impossible


-- …and the sixth part names no duration endpoint at all, [CR#511.2]
-- putting "until end of combat" at the end of the combat PHASE, which is
-- `Combat`'s cell. One phrase, one slot.
public export
badUntilEndOfCombatStep : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) 1 1 (Just (Until (StartOf EndOfCombat Nothing))) {sp = ok})
badUntilEndOfCombatStep SpanStated impossible


-- The participle's two surfaces are not interchangeable per verb.
-- "Destroyed this way" is fifty-one lines and "the destroyed [noun]" is
-- zero, so the destroy row writes the deictic only — which is what makes
-- this a table and not a free slot.
public export
badDestroyedAttributive : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                Macros.exile (TheVerbed Destroy CardW {marking = Attributive} {mk = ok})])
badDestroyedAttributive MkVerbedMarkingOk impossible


-- The controller relation still seeds the BATTLEFIELD, and the stack
-- row's arrival is what makes that a measured refusal rather than a
-- caveat: [CR#109.4] gives stack objects a controller too, so "target
-- spell you control" is real English (seventy-one lines, plus six for an
-- opponent and two negated), and the phrase is refused because
-- `seedZone` names one zone where the relation constrains to two. Every
-- one of those lines is copy machinery, which is why the shape is
-- ledgered rather than repaired here.
public export
badControlledSpell : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.spell, ControlledBy You] {zc = ok})
badControlledSpell MkZoneCoherent impossible


-- The six unattested values, one pin each so a future broadening fails
-- at the VALUE gate with the failure attributable to nothing else: the
-- subject and context are ordinary throughout. The zero counts are
-- exact and individually queried ("becomes flipped/unflipped/face
-- up/face down/phased in/phased out" — zero supported lines apiece);
-- the operations exist under their own verbs ([CR#708] for the face
-- pair, [CR#710] for flip, [CR#702.26] for phasing) and are the
-- ledger's. Unflipped is stronger still: flipping is one-way, so the
-- transition cannot happen ([CR#710.4]).
public export
badBecomesFlipped : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) Flipped {at = ok}) Macros.drawACard)
badBecomesFlipped MkStatusEventVal impossible


public export
badBecomesUnflipped : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) Unflipped {at = ok}) Macros.drawACard)
badBecomesUnflipped MkStatusEventVal impossible


public export
badBecomesFaceUp : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) FaceUp {at = ok}) Macros.drawACard)
badBecomesFaceUp MkStatusEventVal impossible


public export
badBecomesFaceDown : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) FaceDown {at = ok}) Macros.drawACard)
badBecomesFaceDown MkStatusEventVal impossible


-- The singular quantity licenses the class word as the phrase's HEAD,
-- not wherever it can hide: "target creature the controller of any
-- target controls" spells it in a possessor, where a counted mention
-- forbids it exactly as "a" does (`badAnyTargetEmbedded`).
public export
badEmbeddedAnyTargetExact1 : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, ControlledBy (ControllerOf (Macros.target AnyTarget))]) {af = ok})
badEmbeddedAnyTargetExact1 MkAnyTargetAtCount impossible


public export
badBecomesPhasedIn : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) PhasedIn {at = ok}) Macros.drawACard)
badBecomesPhasedIn MkStatusEventVal impossible


public export
badBecomesPhasedOut : Unspellable Ability (\ok =>
  Triggered Whenever (BecomesStatus (Macros.a Permanent) PhasedOut {at = ok}) Macros.drawACard)
badBecomesPhasedOut MkStatusEventVal impossible


-- [CR#603.2b] keeps `At` for phases and steps: "At a creature becomes
-- tapped" is unwritten, as it is for every other object event.
public export
badAtBecomesTapped : Unspellable Ability (\ok =>
  Triggered At (BecomesStatus (Macros.a Macros.creature) Tapped) Macros.drawACard {wo = ok})
badAtBecomesTapped MkTriggerWordOk impossible


-- Trigger-only, reader by reader — `badInterceptEnters`'s shape at the
-- new event. Nothing intercepts a becomes-tapped: no "if [it] would
-- become tapped, … instead" line exists.
public export
badInterceptBecomesTapped : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead (BecomesStatus (Macros.target Macros.creature) Tapped)
                 (Macros.exile It) (Just Macros.thisTurn)
                 {ok = Builtin.fst ok, uo = Builtin.snd ok})
badInterceptBecomesTapped (MkInterceptable, _) impossible


-- …and the [CR#610.3] rider does not wait for one: "exile it until
-- [something] becomes untapped" is written zero times, the rider's
-- whole corpus being the departure.
public export
badHeldUntilBecomesUntapped : Unspellable (Effect []) (\ok =>
  Macros.exileUntil (Macros.target Macros.creature) (BecomesStatus (Macros.a Macros.creature) Untapped) {hd = ok})
badHeldUntilBecomesUntapped MkHoldable impossible


-- …nor does the delayed clause: the delayed family stays the end-step
-- beginning, the departure, and the death.
public export
badDelayedOnBecomesTapped : Unspellable (Effect []) (\ok =>
  Delayed (BecomesStatus (Macros.target Macros.creature) Tapped)
          (Macros.sacrifice You (That (TypeW Creature))) {aw = ok})
badDelayedOnBecomesTapped MkAwaitable impossible


-- No measured duration ends at a status transition: the tapped-STATE
-- span is "for as long as … remains tapped" ([CR#611.2b]), a condition
-- inside the for-as-long-as adverbial and not an event endpoint (forty
-- of forty corpus "remains tapped" lines; zero write "until … becomes
-- untapped" in a clause this grammar spells).
public export
badGetsUntilBecomesUntapped : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.creature) 2 2
       (Just (UntilEvent (BecomesStatus Macros.thisCreature Untapped))) {sp = ok})
badGetsUntilBecomesUntapped SpanStated impossible


-- the timed clause's subject stands on the battlefield, `Tap`/`Untap`'s
-- own demand at the new row ([CR#701.26a]'s zone, `badUntapGraveyard`'s
-- twin).
public export
badUntapNextGraveyard : Unspellable (Effect []) (\ok =>
  DoesntUntapNext (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) 1 {ok = ok})
badUntapNextGraveyard OnField impossible


-- "it" after two singular object introductions reaches two mentions and
-- resolves neither — the strict uniqueness gate, unchanged at the new
-- consumer.
public export
badUntapNextAmbiguousIt : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target Macros.creature),
                Tap (Macros.target Macros.artifact),
                DoesntUntapNext (It {ok = ok}) 1])
badUntapNextAmbiguousIt Refl impossible


-- the count vocabulary is closed at the attested one and two: "next
-- three untap steps" is written zero times, and the table refuses it.
public export
badUntapNextThree : Unspellable (Effect []) (\ok =>
  DoesntUntapNext (Macros.target Macros.creature) 3 {ct = ok})
badUntapNextThree OneNextStep impossible
