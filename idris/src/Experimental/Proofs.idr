||| Compiler-checked refusal proofs migrated from Experimental.Cards.
module Experimental.Proofs

import Experimental
import Experimental.Macros

%default total

-- Bare lowercase names in a signature must RESOLVE, never be auto-bound into
-- a fresh implicit: a silent hole makes a pin refuse for a reason that is not
-- the card's.
%unbound_implicits off

||| `Unspellable <sort> <phrase>` — a whole phrase of that sort cannot be
||| written: the phrase, parameterised by whatever obligation it leaves open,
||| has no proof of that obligation. The sort is explicit because a refused
||| phrase may be an `Effect []`, an `Ability`, a `Card`, or smaller.
|||
||| The line is spelled out in full with the real constructors, so a pin
||| states the phrase it refuses rather than a fragment asserted to come
||| from one. `P` is never written down — the elaborator infers it from the
||| line itself, so a pin cannot refute the wrong obligation: there is
||| nowhere to name one.
public export
0 Unspellable : (0 a : Type) -> {0 P : Type} -> ((0 _ : P) -> a) -> Type
Unspellable _ {P = p} _ = Not p


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
