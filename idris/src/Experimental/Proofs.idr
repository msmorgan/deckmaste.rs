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
