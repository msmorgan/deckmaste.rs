||| Compiler-checked refusal proofs migrated from Experimental.Cards.
module Experimental.Proofs

import Experimental
import Experimental.Macros
import public Experimental.Unspellable

%default total

-- Bare lowercase names in a signature must RESOLVE, never be auto-bound into
-- a fresh implicit: a silent hole makes a pin refuse for a reason that is not
-- the card's.
%unbound_implicits off


-- ===== Compiler-checked refusals =====

||| "Choose a flying."
||| A keyword is a modifier, never a noun head.
public export
badKeywordHead : Unspellable (Effect []) (\ok =>
  Choose (Macros.a (HasKeyword Flying) {hd = ok}))
badKeywordHead MkHeaded impossible


-- These choices bind their qualities, but their later readback surfaces are
-- name equality and numeric equality, not `OfChosen`.
-- CLAIM: the former badChosenCardNameRead pin has an uninhabited obligation -> badChosenCardNameRead
public export
badChosenCardNameRead :
  Unspellable
    (Predicate [MkBinding AD (Quality CardName) OneOf QualityP] Object)
    (\ok => OfChosen CardName {read = ok})
badChosenCardNameRead MkChosenQualityRead impossible

public export
badChosenNumberRead :
  Unspellable
    (Predicate [MkBinding AD (Quality Number) OneOf QualityP] Object)
    (\ok => OfChosen Number {read = ok})
badChosenNumberRead MkChosenQualityRead impossible


-- Two creatures have no single power [CR#208.1] — the aggregate is
-- written explicitly ("the total power of the sacrificed creatures",
-- Soulblast), and is future vocabulary.
-- CLAIM: the former badGroupPower pin has an uninhabited obligation -> badGroupPower
public export
badGroupPower : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (TargetGroup (Macros.exactly 2) Macros.creature),
                Macros.gainsLife You (Macros.powerOf Them {one = ok})])
badGroupPower Refl impossible


-- Two cards need not share an owner [CR#108.3] — oracle writes the
-- plural relational ("their owners' hands", Aether Burst), future
-- vocabulary.
-- CLAIM: the former badGroupOwner pin has an uninhabited obligation -> badGroupOwner
public export
badGroupOwner : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (TargetGroup (Macros.exactly 2) Macros.creature),
                Macros.losesLife (OwnerOf Them {one = ok}) (Lit 1)])
badGroupOwner Refl impossible


-- The binary fight frame takes singular combatants — a group versus
-- one has no defined pairing; the plural form is the reciprocal
-- "those creatures fight each other" (ledger).
-- CLAIM: the former badFightGroup pin has an uninhabited obligation -> badFightGroup
public export
badFightGroup : Unspellable (Effect []) (\ok =>
  Fights (TargetGroup (Macros.exactly 2) Macros.creature) {pa = ok}
         (Macros.target Macros.creature))
badFightGroup Refl impossible


-- "a creature two target opponents control": an object has one
-- controller [CR#109.4]; the union possessor ("creatures your
-- opponents control") is the player-groups vocabulary (ledger).
-- (Probed at the predicate itself: inside a larger phrase the stuck
-- slot stalls the outer coherence search instead.)
-- CLAIM: the former badControlledByGroup pin has an uninhabited obligation -> badControlledByGroup
public export
badControlledByGroup : Unspellable (Predicate [] Object) (\ok =>
  ControlledBy (TargetGroup (Macros.exactly 2) Opponent) {one = ok})
badControlledByGroup Refl impossible


-- Hands and graveyards are per-player zones [CR#400.1]: one zone
-- owned by two players at once is unwritable.
-- CLAIM: the former badGraveyardOfGroup pin has an uninhabited obligation -> badGraveyardOfGroup
public export
badGraveyardOfGroup : Unspellable (ZoneExpr []) (\ok =>
  Macros.graveyardOf (TargetGroup (Macros.exactly 2) Opponent) {one = ok})
badGraveyardOfGroup Refl impossible


-- The minted dies-watcher is singular (Graceful Reprieve's shape);
-- plural watches wait for corpus evidence.
-- CLAIM: the former badDiesGroup pin has an uninhabited obligation -> badDiesGroup
public export
badDiesGroup : Unspellable (Effect []) (\ok =>
  Delayed (Dies (TargetGroup (Macros.exactly 2) Macros.creature))
          {span = Just ThisTurn} (Macros.gainsLife You (Lit 1)) {one = ok})
badDiesGroup Refl impossible


||| "each of target creature"
||| The complement to EachOf must be plural.
public export
badEachOfSingular : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.target Macros.creature) {pl = ok})
badEachOfSingular Refl impossible


||| "This deals 2 damage divided as you choose among target creature."
||| Damage division requires at least two objects to divide between.
public export
badDivideAmongSingular : Unspellable (Effect []) (\ok =>
  Macros.dealsDivided This (Lit 2) (Macros.target Macros.creature) {pl = ok})
badDivideAmongSingular Refl impossible


||| "Look at the top of each player's library."
||| Whose library is always one player's [CR#400.1].
public export
badSliceOfPluralPossessor : Unspellable (Effect []) (\ok =>
  Macros.lookAt (LibrarySlice OnTop (Lit 1) (Each AnyPlayer) {one = ok}))
badSliceOfPluralPossessor Refl impossible


||| "Whenever you cast all spells, draw a card."
||| Malformed trigger condition: Casts requires a singular spell.
public export
badCastsPluralComplement : Unspellable (Ability) (\ok =>
  Triggered Whenever (Casts You (AllOf Macros.spell) {one = ok})
            Macros.drawACard)
badCastsPluralComplement Refl impossible


||| "Tap target creature an opponent controls or land you control. That player loses 1 life."
||| Unspellable because its disjunction binds no coherent singular player antecedent.
public export
badDisjunctAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target (Or [And [Macros.creature, ControlledBy Macros.anOpponent],
                                        And [Macros.land, ControlledBy You]])),
                Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badDisjunctAntecedent Refl impossible


||| "Destroy target creature: Draw a card."
||| Destroy is not a cost verb.
public export
badDestroyAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.destroy (Macros.a Macros.creature)) {ok})
            Macros.drawACard)
badDestroyAsCost MkCostAction impossible


-- The full predicate equality table catches a keyword and its negation.
public export
badKeywordContradiction : Unspellable (Effect []) (\ok =>
  Tap (Macros.target (And [Macros.creature, HasKeyword Flying,
                            Not (HasKeyword Flying)] {cf = ok})))
badKeywordContradiction MkContradictionFree impossible


-- Forest presupposes land, so this conjunction contradicts itself.
public export
badForestNonland : Unspellable (Effect []) (\ok =>
  Tap (Macros.target (And [HasSubtype Forest, Not Macros.land] {cf = ok})))
badForestNonland MkContradictionFree impossible


-- One printed line carries Basic and Snow, so the distinctness the type
-- line asks for is per WORD and not per line — `badDuplicateSupertype`
-- pins the same table on the row that came before these.
public export
badDuplicateSnow : Unspellable Card (\ok =>
  Macros.card "" Nothing [Snow, Snow] (MkTypeLine [Forest] [Land]) [] Nothing {sp = ok})
badDuplicateSnow MkCardSupers impossible


-- "other" with no target before it: the presupposition has no witness.
-- Forward and self references are unspellable the same way — there is
-- no context in which a later mention precedes.
public export
badOther : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.target (Macros.anyOtherTarget {ok})))
badOther Refl impossible


-- A genuinely ambiguous pronoun: two singular Object mentions precede
-- "it", so the uniqueness gate refuses. (Not oracle-legal text — which
-- is the point: the controlled language never writes this.)
public export
badIt : Unspellable (Effect []) (\ok =>
  Sequentially [Fights (Macros.target Macros.creature) (Macros.target Macros.creature),
                Tap (It {ok})])
badIt Refl impossible


-- A group is not a singular antecedent: "each creature … it" has no
-- referent for "it" (the plurality guard).
public export
badTheyIt : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Each Macros.creature),
                Tap (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badTheyIt (Refl, _) impossible


-- The delay does not launder a dead referent: sacrifice's zone demand
-- reads fold-state through the boundary, and the destroyed target sits
-- in the graveyard ([CR#701.21a]; contrast Junkyo Bell, which legally
-- delays sacrificing a LIVE target — the distinction is the referent's
-- zone, never its determiner).
public export
badStale : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
               Delayed (BeginningOf EndStep Nothing) (Macros.sacrifice You It {ok})])
badStale OnField impossible


-- "Another target" inside a delayed clause can only be distinct from
-- the DELAYED ability's own targets — it announces in its own event
-- ([CR#603.3d,601.2c]), so the outer clause's settled targets are no
-- witness for the presupposition. (Soundness-derived: the corpus
-- writes no such line; Swooping Pteranodon shows the two halves — a
-- fresh "target land" announced at delay time reading back "that
-- creature" from the outer clause.)
public export
badDelayedOther : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 2) (Macros.target AnyTarget),
               Delayed (BeginningOf EndStep Nothing) (DealDamage This (Lit 1) (Macros.target (Macros.anyOtherTarget {ok})))])
badDelayedOther Refl impossible


-- After the exile, the referent no longer answers to "creature": its
-- carrier is derived from the RETAGGED zone ([CR#110.1]), so the typed
-- demonstrative has no antecedent — the carrier-word rule as a type
-- error ("that card" is the spelling that resolves; see `cloudshift`).
public export
badStaleCarrier : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile (Macros.target Macros.creatureYouControl),
               Move (That (TypeW Creature) {ok}) Macros.battlefieldZ])
badStaleCarrier Refl impossible


-- A hidden-zone cost mention is unreadable past the colon: the card
-- bounced to hand is not among the public survivors ([CR#400.2] —
-- hand is hidden; no [CR#400.7] exception reaches it, and
-- Soratami Cloudskater-family costs write no such read back).
public export
badHiddenCost : Unspellable Ability (\ok =>
  Activated (Do (Move (Macros.a Macros.creature) Macros.handZ))
            (Tap (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}))
badHiddenCost (Refl, _) impossible


-- Two cost moves leave two candidate antecedents ("Discard a card,
-- Sacrifice a creature: …" — Falkenrath Pit Fighter-family): a bare
-- "It" past the colon is ambiguous. Real costs of this shape read
-- back with definite descriptions (the participle reads of chapter
-- eight), never a bare pronoun. (The verb is exile — zone-blind —
-- so the pin isolates the ambiguity, not a zone gate.)
public export
badTwoCostMentions : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.discardsACard You),
                       Do (Macros.sacrifice You (Macros.a Macros.creature))])
            (Macros.exile (It {ok})))
badTwoCostMentions Refl impossible


-- The zone half of sacrifice's implicit restriction as a type error:
-- an exiled referent is not sacrificeable [CR#701.21a].
public export
badSacrificeExiled : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile (Macros.target Macros.creature),
               Macros.sacrifice You It {ok}])
badSacrificeExiled OnField impossible


-- After the watched target dies, it no longer answers to "creature":
-- the event retag flips the carrier ([CR#700.4,110.1]) — "that card"
-- is the spelling that resolves (see `gracefulReprieve`).
public export
badDeadCreatureRead : Unspellable (Effect []) (\ok =>
  Delayed (Dies (Macros.target Macros.creature)) {span = Just ThisTurn}
          (Move (That (TypeW Creature) {ok}) Macros.battlefieldZ))
badDeadCreatureRead Refl impossible


-- "The chosen type" with only a color chosen: the quality read is
-- sort-filtered — no witness.
public export
badChosenWrongSort : Unspellable
  (Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object)
  (\ok => OfChosen CreatureType {ok})
badChosenWrongSort Refl impossible


-- Two group mentions leave "them" ambiguous — the plural wildcard has
-- the same strict-uniqueness gate as the singular.
public export
badThemAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (TargetGroup (Macros.exactly 2) Macros.creature),
               Choose (TargetGroup (Macros.exactly 2) Macros.creature),
               Tap (Them {ok})])
badThemAmbig Refl impossible


-- Two predicate-inner opponents leave "that player" ambiguous — the
-- uniqueness gate reaches inside relative clauses too. (Not
-- oracle-legal text; the guide would repeat the noun.)
public export
badInnerAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [Fights (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent]))
                       (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent])),
               Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badInnerAmbig Refl impossible


-- The participle's verb filter has no witness: the cost discarded,
-- nothing was sacrificed.
public export
badVerbedWrongVerb : Unspellable Ability (\ok =>
  Activated (Do (Macros.discardsACard You))
            (Move (TheVerbed Sacrifice CardW {ok}) Macros.battlefieldZ))
badVerbedWrongVerb Refl impossible


-- The noun word misses on the type axis: an artifact was sacrificed,
-- so "the sacrificed creature" has no referent.
public export
badVerbedWrongNoun : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice You (Macros.a (HasType Artifact))))
            (Move (TheVerbed Sacrifice (TypeW Creature) {ok}) Macros.battlefieldZ))
badVerbedWrongNoun Refl impossible


-- Two same-verb stamps leave the participle ambiguous — the same
-- strict uniqueness as every read (real costs of this shape name
-- distinct verbs, which is what the filter buys).
public export
badVerbedAmbig : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.sacrifice You (Macros.a Macros.creature)),
                       Do (Macros.sacrifice You (Macros.a Macros.creature))])
            (Move (TheVerbed Sacrifice CardW {ok}) Macros.battlefieldZ))
badVerbedAmbig Refl impossible


-- Voyager Staff's shape with the bare demonstrative: two card
-- mentions (the sacrificed self, the exiled target) make "that card"
-- ambiguous — the participle is what real text switches to here.
public export
badBareCardRead : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice You Macros.thisArtifact))
            (Sequentially [Macros.exile (Macros.target Macros.creature),
                           Delayed (BeginningOf EndStep Nothing) (Move (That CardW {ok}) Macros.battlefieldZ)]))
badBareCardRead Refl impossible


-- Tapping takes a battlefield object [CR#701.26a]: a graveyard card
-- cannot be tapped.
public export
badTapGraveyard : Unspellable (Effect []) (\ok =>
  Tap (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badTapGraveyard OnField impossible


-- a bare status word heads nothing: "choose a tapped" / "destroy target
-- tapped" are unwritable — the modifier needs a head beside it.
public export
badBareTappedHead : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target Macros.tapped {hd = ok}))
badBareTappedHead MkHeaded impossible


-- status is only a battlefield permanent's ([CR#110.5d]): a "tapped
-- creature card in your graveyard" places its referent in two zones at
-- once and describes nothing.
public export
badTappedGraveyard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped,
                                       InZone (Macros.graveyardOf You)] {zc = ok})))
badTappedGraveyard MkZoneCoherent impossible


-- one value per category ([CR#110.5]): "tapped untapped creature"
-- describes nothing, and the refusal is the category clash, not a
-- negation pair — neither word is spelled as the other's "non-".
public export
badTappedUntapped : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped, Macros.untapped] {cf = ok})))
badTappedUntapped MkContradictionFree impossible


-- the pair is two WORDS: "nontapped" is written zero times, so the
-- status word does not negate — the opposite value is its own row.
public export
badNonTapped : Unspellable (Predicate [] Object) (\ok =>
  Not Macros.tapped {ng = ok})
badNonTapped MkNegatable impossible


-- "phased-in" is written zero times as a description; the value exists
-- in the closed product ([CR#110.5]) and its surface cell refuses.
public export
badPhasedInWord : Unspellable (Predicate [] Object) (\ok =>
  HasStatus PhasedIn {at = ok})
badPhasedInWord MkStatusWord impossible


-- untap takes a battlefield object, the tap row's own demand mirrored
-- ([CR#701.26b]; `badTapGraveyard`'s twin).
public export
badUntapGraveyard : Unspellable (Effect []) (\ok =>
  Untap (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badUntapGraveyard OnField impossible


-- "permanent" beside a projected instant head describes nothing
-- ([CR#110.4] — "instant and sorcery cards can't enter the battlefield
-- and thus can't be permanents").
public export
badPermanentInstant : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Permanent, HasType Instant] {cf = ok})))
badPermanentInstant MkContradictionFree impossible


-- "that permanent" after its referent left: destruction retags to the
-- graveyard, [CR#110.1] takes the word away with the zone, and the
-- current-state read reaches nothing.
public export
badThatPermanentDeparted : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Permanent),
               Tap (That PermanentW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badThatPermanentDeparted (Refl, _) impossible


-- a token off the battlefield has ceased to exist ([CR#111.7]): "token
-- card in your graveyard" describes nothing.
public export
badTokenGraveyard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [IsToken, InZone (Macros.graveyardOf You)] {zc = ok})))
badTokenGraveyard MkZoneCoherent impossible


-- "nontoken token" is the negation pair the scan already refuses.
public export
badNontokenToken : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [IsToken, Macros.nontoken] {cf = ok})))
badNontokenToken MkContradictionFree impossible


-- a referent nothing minted as a token is never "that token": the
-- origin field is written only by the create clause ([CR#111.1]).
public export
badThatTokenOfCard : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target Macros.creature),
               Untap (That TokenW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badThatTokenOfCard (Refl, _) impossible


-- the lock's subject stands on the battlefield, `badCantInGraveyard`'s
-- twin.
public export
badUntapLockGraveyard : Unspellable Ability (\ok =>
  Static (DoesntUntap (Macros.a (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok}))
badUntapLockGraveyard MkZoneFits impossible


-- a durationless "doesn't untap" CLAUSE is the static ability line and
-- not a clause at all — `badStaticCant`'s shape with the new statement.
public export
badUntapLockClause : Unspellable (Effect []) (\ok =>
  Continuously (DoesntUntap (AsType Artifact This)) Nothing {sp = ok})
badUntapLockClause SpanUnstated impossible


-- Only battlefield creatures fight [CR#701.14b]: a graveyard card
-- cannot.
public export
badFightGraveyard : Unspellable (Effect []) (\ok =>
  Fights (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {za = ok}
         (Macros.target Macros.creature))
badFightGraveyard OnField impossible


-- Only combatant types fight [CR#701.14a]: a land's TypeDef declares
-- no fight participation.
public export
badFightLand : Unspellable (Effect []) (\ok =>
  Fights (Macros.target (HasType Land)) {ta = ok} (Macros.target Macros.creatureYouDontControl))
badFightLand Fighter impossible


-- Dying is the battlefield-to-graveyard transition [CR#700.4]: an
-- already-graveyard card cannot die this turn. The demand is
-- `zoneFits`' as of chapter twenty-eight rather than `OnBattlefield`'s,
-- and the refusal is unchanged by the loosening: a phrase that STATES
-- the graveyard still contradicts the battlefield, while the phrase
-- that states nothing ("this creature", the trigger corpus's own
-- subject at two thousand four hundred seventy-three lines) passes on
-- its silence.
public export
badDiesInGraveyard : Unspellable (Effect []) (\ok =>
  Delayed (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok}) {span = Just ThisTurn}
          (Move (That CardW) Macros.battlefieldZ))
badDiesInGraveyard MkZoneFits impossible


-- Damage reaches players and battlefield objects only [CR#120.1]:
-- the destroyed referent sits in the graveyard.
public export
badDamageGraveyardCard : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
               DealDamage This (Lit 3) It {rk = ok}])
badDamageGraveyardCard ObjectTakes impossible


-- A quality cannot take damage [CR#120.1].
public export
badDamageToColor : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.a (QualityNoun Color)) {rk = ok})
badDamageToColor ObjectTakes impossible


-- Damage goes to battles, creatures, planeswalkers, or players
-- [CR#120.1a]: a noncreature artifact takes none.
public export
badDamageArtifact : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.target (HasType Artifact)) {rk = ok})
badDamageArtifact ObjectTakes impossible


-- A phrase needs a positive head ([CR#105.1,608.2d]): "choose a
-- noncolor" heads nothing — Not is a modifier, never a head.
public export
badNegatedQualityHead : Unspellable (Effect []) (\ok =>
  Choose (Macros.a (Not (QualityNoun Color)) {hd = ok}))
badNegatedQualityHead MkHeaded impossible


-- "Target non-player" is refused EARLIER than headlessness now: the
-- universal player word names one of the people in the game
-- ([CR#102.1]), so there is no complement class for "non-" to take —
-- the same argument the class word's row makes, and the negation is
-- unwritable before the phrase ever reaches the head gate. Probed at
-- the bare predicate: nested inside `target`'s auto search this
-- failure misreports as the outer `Headed` one.
public export
badNegatedPlayerHead : Unspellable (Predicate [] Player) (\ok =>
  Not AnyPlayer {ng = ok})
badNegatedPlayerHead MkNegatable impossible


-- Negation binds nothing: "a creature an opponent DOESN'T control"
-- names no opponent for "that player" to read.
public export
badNegatedAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [Tap (Macros.target (And [Macros.creature, Not (ControlledBy Macros.anOpponent)])),
                Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badNegatedAntecedent Refl impossible


-- An object is in ONE zone: contradicted zone conjuncts refuse in
-- either order.
public export
badConflictingZones : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone Macros.battlefieldZ, InZone Macros.graveyardZ] {zc = ok})))
badConflictingZones MkZoneCoherent impossible


-- Targets are objects and players [CR#115.1]: "target color" is
-- unwritten — qualities are chosen, never targeted.
public export
badTargetColor : Unspellable (Effect []) (\ok =>
  Choose (Macros.target (QualityNoun Color) {tk = ok}))
badTargetColor ObjectTgt impossible


-- The one-shot stat modification takes a battlefield object: a dead
-- referent doesn't get +3/+3.
public export
badGetsGraveyard : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                Macros.gets It 3 3 (Just Macros.untilEndOfTurn) {ok}])
badGetsGraveyard MkZoneFits impossible


-- A card never enters another player's hand [CR#400.3]: owned
-- destinations are owner-routed, so this is unwritable.
public export
badMoveToTargetsHand : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) (Macros.handOf (Macros.target AnyPlayer)) {ok})
badMoveToTargetsHand HandOkBare impossible


-- Only a battlefield permanent is destroyable [CR#701.8a].
public export
badDestroyGraveyard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badDestroyGraveyard OnField impossible


-- Discarding moves a card from a HAND [CR#701.9a]: a battlefield
-- creature is not discardable.
public export
badDiscardBattlefield : Unspellable (Effect []) (\ok =>
  Macros.discards You (Macros.a Macros.creature) {dk = ok})
badDiscardBattlefield DiscardTracked impossible


-- The tag and its body agree: a Destroy-tagged exile would let
-- indestructible cant an exile [CR#701.8b,702.12b].
public export
badDestroyTaggedExile : Unspellable (Effect []) (\ok =>
  Composite Destroy (Move (Macros.target Macros.creature) Macros.exileZ) {ok})
badDestroyTaggedExile DestroyB impossible


-- The zone demand lives on the tag-body relation: the raw Composite
-- spelling proves what the macro proves [CR#701.8a].
public export
badCompositeDestroyGraveyard : Unspellable (Effect []) (\ok =>
  Composite Destroy (Move (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) Macros.graveyardZ) {ok = DestroyB {z = ok}})
badCompositeDestroyGraveyard OnField impossible


-- An agentive tag cannot shed its actor [CR#701.21a]: the raw
-- Composite spelling of sacrifice is refused outright.
public export
badAgentlessSacrifice : Unspellable (Effect []) (\ok =>
  Composite Sacrifice (Move (Macros.a Macros.creature) Macros.graveyardZ) {ok = SacrificeB} {na = ok})
badAgentlessSacrifice DestroyNA impossible


-- Discarding moves a hand card [CR#701.9a]: the demand rides the tag
-- relation, so a battlefield "discard" is unspellable under Does too
-- — and with it the forged stamp `TheVerbed Discard` would read.
public export
badDoesDiscardBattlefield : Unspellable (Effect []) (\ok =>
  Does You Discard (Move (Macros.a Macros.creature) Macros.graveyardZ) {tb = DiscardB {d = ok}})
badDoesDiscardBattlefield DiscardTracked impossible


-- A written quantity permits at least one: "zero target creatures" is
-- unwritten English, and so is the "up to zero" spelling of it — the
-- demand is on the quantity's MAXIMUM. (One IS writable: it is the
-- singular form the `target` macro spells, rendered without its
-- numeral.)
public export
badZeroGroup : Unspellable (Effect []) (\ok =>
  Choose (TargetGroup (Macros.exactly 0) Macros.creature {nz = Builtin.fst ok} {wf = Builtin.snd ok}))
badZeroGroup (MaxAtLeastOne, _) impossible


-- …and the shared zones take no possessor at all: [CR#400.1] gives
-- each player a library, a hand, and a graveyard and shares the rest,
-- so "your battlefield" is not a phrase. The refusal is the ZONE's,
-- not the possessor's — the noun here is impeccable.
public export
badOwnedBattlefield : Unspellable (ZoneExpr []) (\ok =>
  ZoneAt Battlefield (OwnedBy You {ps = ok}))
badOwnedBattlefield HandIsOwned impossible


-- A bare type word denotes a permanent [CR#109.2], and what was
-- discarded left a HAND: "the discarded creature" is unwritten (the
-- corpus discard family reads "the discarded card" only) — the
-- stamp's at-verb frame refuses the type word.
public export
badDiscardedCreatureWord : Unspellable Ability (\ok =>
  Activated (Do (Macros.discards You (Macros.aAtRandom (And [Macros.creature, InZone Macros.handZ]))))
            (DealDamage This
                          (Macros.manaValueOf (TheVerbed Discard (TypeW Creature) {ok}))
                          (Macros.target AnyTarget)))
badDiscardedCreatureWord Refl impossible


-- "Of their choice" is a possessive pronoun: it demands a player
-- antecedent (a subject or one distributive group [CR#608.2d]) —
-- bare "Destroy a creature of their choice" is unwritten.
public export
badUnboundTheirChoice : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.aTheirChoice Macros.creature {ch = ok}))
badUnboundTheirChoice Refl impossible


-- "That much" with nothing done yet: no outcome to read.
public export
badThatMuchUnbound : Unspellable (Effect []) (\ok =>
  DealDamage This (ThatMuch {ok}) (Macros.target AnyTarget))
badThatMuchUnbound Refl impossible


-- Two event clauses leave "that much" ambiguous — outcomes obey the
-- same strict uniqueness as every read.
public export
badThatMuchAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.target AnyTarget),
                Macros.losesLife You (Lit 2),
                Macros.gainsLife You (ThatMuch {ok})])
badThatMuchAmbig Refl impossible


-- ===== Chapter thirteen negatives: the refuse-nonsense wave =====
-- (Probed at the smallest construct that carries the gate — a bare
-- `Predicate`/`Amount`/`Noun` where one exists: an auto-search failure
-- nested inside another auto stalls the OUTER search and misreports.)

-- "Other" needs a head-COMPATIBLE anchor. The guide reserves
-- "another" for excluding the source or first referent and writes two
-- separately described roles WITHOUT it ("target creature and target
-- planeswalker"); the corpus pairs "other" only with overlapping
-- heads. A land target is no anchor for "another creature" — even
-- though sharing an object across such slots is rules-legal
-- ([CR#601.2c]), which is what makes this templating, gated at the
-- conjunction where the head type is known.
public export
badOtherCrossHead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target (HasType Land)),
                Macros.destroy (Macros.target (And [Macros.creature, Other] {oa = ok}))])
badOtherCrossHead MkOtherAnchored impossible


-- Every corpus for-each domain is noun-headed: "for each you control"
-- names no set to count — the positive-head demand the determiners
-- already carry (finding 39), now on the counted-set amount.
public export
badForEachHeadless : Unspellable (Amount []) (\ok =>
  Macros.forEach (ControlledBy You) {hd = ok})
badForEachHeadless MkHeaded impossible


-- A written numeral is at least one — "1 life for each 0 creatures" is
-- unwritten English. (The comparisons that legitimately carry zero read
-- a count rather than write one; `Lit` stays ungated.)
public export
badForEachZero : Unspellable (Amount []) (\ok =>
  Macros.nForEach 0 Macros.creature {nz = ok})
badForEachZero OneUp impossible


-- Zone negation itself is REAL oracle — "Each Vampire creature card
-- you own that isn't on the battlefield has madness." (Falkenrath
-- Gorger) — so `Not (InZone …)` stays writable. What it cannot do is
-- contradict the zone the phrase itself places its referent in: a bare
-- "creature" means the battlefield [CR#109.2].
public export
badNotOnBattlefield : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Not (InZone Macros.battlefieldZ)] {zc = ok})
badNotOnBattlefield MkZoneCoherent impossible


-- No member negates a sibling: "of the chosen color and not of the
-- chosen color" describes nothing.
public export
badQualityContradiction : Unspellable
  (Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object)
  (\ok => And [OfChosen Color, Not (OfChosen Color)] {cf = ok})
badQualityContradiction MkContradictionFree impossible


-- …and the nested spelling is refused the same way — the member scan
-- flattens conjunctions, so a clash one level down is no laundering.
-- (A nested ZONE clash is the same nonsense but trips the zone gate
-- first, so the nested probe uses a type contradiction to keep the pin
-- unambiguous.)
public export
badNestedContradiction : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, And [Not Macros.creature]] {cf = ok})
badNestedContradiction MkContradictionFree impossible


-- Attackers are declared from creatures their controller controls
-- ([CR#508.1a]) and leaving the battlefield removes a permanent from
-- combat ([CR#506.4]), so the status word seeds its own zone and
-- clashes with an explicit hand clause — no new gate, the existing
-- coherence one.
public export
badAttackingInHand : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, InZone Macros.handZ] {zc = ok})
badAttackingInHand MkZoneCoherent impossible


-- A COLLECTIVE group damage subject is unattested: oracle distributes
-- the frame ("Each creature you control deals 1 damage to that
-- creature.", Case of the Gateway Express) or names one source.
public export
badGroupDamageSource : Unspellable (Effect []) (\ok =>
  DealDamage (TargetGroup (Macros.exactly 2) Macros.creature) (Lit 3) (Macros.target AnyPlayer) {ds = ok})
badGroupDamageSource MkDamageSource impossible


-- The class word is never negated: [CR#115.4] defines "any target"
-- positively as the damage target class, and the guide forbids using it
-- as a synonym for "any object" — so there is nothing for "non-" to
-- take a complement in.
public export
badNegatedAnyTarget : Unspellable (Predicate [] Object) (\ok =>
  Not AnyTarget {ng = ok})
badNegatedAnyTarget MkNegatable impossible


-- …and it takes no modifier but "other" (Arc Trail's "any other
-- target"): "any target in a graveyard" would be that forbidden
-- synonym, spelled as a restriction.
public export
badAnyTargetInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [AnyTarget, InZone Macros.graveyardZ] {at = ok})
badAnyTargetInGraveyard MkAnyTargetLone impossible


-- "Any target" is ITSELF the targeting form, so only the targeting
-- determiners admit it: "a any target" and "each any target" are
-- unwritable.
public export
badAnyTargetUnderA : Unspellable (Noun [] Object) (\ok =>
  Macros.a AnyTarget {af = ok})
badAnyTargetUnderA MkAnyTargetFree impossible


-- …and the targeting determiner admits it only where the count is the
-- CASTER's to make: "any target" is the singular damage-class form
-- [CR#115.4] defines (bolt, Arc Trail, Pyromancy), and every plural
-- spelling the corpus writes runs from one — "up to two targets" (Fall
-- of the Titans), "one, two, or three targets" (Arc Lightning), "any
-- number of targets" (Boulderfall). A FIXED plural count is the one
-- thing it never writes: "two targets" as a phrase is zero lines and
-- "among two targets" is zero lines, so "two any targets" stays closed.
public export
badGroupAnyTarget : Unspellable (Noun [] Object) (\ok =>
  TargetGroup (Macros.exactly 2) AnyTarget {af = ok})
badGroupAnyTarget MkAnyTargetAtCount impossible


-- A type-worded self-reference denotes the PERMANENT ([CR#109.2]), and
-- discarding moves a card from a HAND ([CR#701.9a]): "discard this
-- creature" is unwritable — cycling's cost says "this card"
-- (`cyclingCost`).
public export
badDiscardThisCreature : Unspellable (Effect []) (\ok =>
  Macros.discards You Macros.thisCreature {dk = ok})
badDiscardThisCreature DiscardTracked impossible


-- The ascription is the SOURCE's, and only the source's: "target
-- creature" already says its type in the predicate it carries, so
-- sorting it a second time spells nothing new — and it would carry
-- [CR#109.2]'s battlefield projection onto a phrase that never argued
-- for it. The closed table is the whole refusal.
public export
badAscribedTarget : Unspellable (Noun [] Object) (\ok =>
  AsType Creature (Macros.target Macros.creature) {asc = ok})
badAscribedTarget AscribeThis impossible


-- Negation is ATOMIC: oracle's non-/isn't/doesn't attaches to one
-- modifier, and a conjunction is negated per-member (De Morgan is the
-- writer's job). A singleton `And` would otherwise launder every
-- Negatable ban — `predEq (And _) _ = False` makes the wrapper
-- invisible to the contradiction scan as well.
public export
badNegatedConjunction : Unspellable (Predicate [] Object) (\ok =>
  Not (And [Macros.creature]) {ng = ok})
badNegatedConjunction MkNegatable impossible


-- The syntactically identical contradiction, no longer laundered by a
-- vacuous member equality: "you" denotes the same player at both
-- mentions, so the phrase asserts and denies one fact of one referent.
-- `nounEqRef You You` is exactly the case the conservative relation
-- can prove — two target mentions would not be, and are not.
public export
badControlContradiction : Unspellable (Predicate [] Object) (\ok =>
  And [ControlledBy You, Not (ControlledBy You)] {cf = ok})
badControlContradiction MkContradictionFree impossible


-- Only a creature can attack ([CR#506.3]), so the status word
-- presupposes the type as well as the zone and "attacking noncreature"
-- describes nothing — the finding-43 shape again: a projection made
-- honest, the refusal falling out of the existing coherence gate.
public export
badAttackingNoncreature : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, Not Macros.creature] {cf = ok})
badAttackingNoncreature MkContradictionFree impossible
