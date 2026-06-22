||| The regression suite. Building the package (`scripts/build`) typechecks this
||| module: the positives must typecheck, and every `failing` block must FAIL to
||| typecheck (each asserts a type-level invariant still bites). A `failing`
||| block that ever compiles makes the build error, so the suite is self-checking.
||| Each negative changes exactly one thing from a known-good construction.
module Spec

import Core
import Macros

-- POSITIVE — must typecheck ---------------------------------------------------

-- target 0 is referenceable once a 1-target scope is open
tTargetInScope : Reference (bindTargets 1 Base) AnObject
tTargetInScope = GetTarget 0

-- `That` is available inside a `With`-bound bindings
tThatInWith : Selection (bindThat Base)
tThatInWith = That

-- a multi-type card may carry one subtype per card type [CR#205.3c]
tLandCreature : Card
tLandCreature = Normal $ fromDefault
  { name := "Test Land Creature"
  , types := [Land, Creature]
  , subtypes := [cast Island, cast Bear]
  }

-- `That`, bound by a `With`, SURVIVES into a delayed body (captured); targets don't
tThatSurvivesDelay : Effect Base
tThatSurvivesDelay =
  With (Produce (Move (SelectAll (creature)) Exile))
    (Delayed nextEndStep (Act (Move That Battlefield)))

-- branching effects typecheck
tMay : Effect Base
tMay = May (Act (Draw (cast 1)))

tIf : Effect Base
tIf = If yourTurn (Act (Draw (cast 1)))

-- a one-shot creating a continuous effect for a duration
tContinuously : Effect Base
tContinuously = Continuously (Modify This [ModifyPT (cast 1) (cast 1)]) UntilEndOfTurn

-- a modal effect: choose one of two modes
tModal : Effect Base
tModal = Modal (MkChooseSpec (cast 1))
  [ MkMode (Act (Draw (cast 1)))
  , MkMode (Act (DealDamage (SelectAll (creature)) (cast 2)))
  ]

-- `Reflexive` NESTS: inside a `With`, its body still sees `That` (no sibling scan)
tReflexiveSeesThat : Effect Base
tReflexiveSeesThat =
  With (Produce (Move (SelectAll (creature)) Exile))
    (Reflexive (Act (Move That Battlefield)))

-- `ForEach` binds `It` per element; the body references `It`
tForEach : Effect Base
tForEach = ForEach (SelectAll (creature))
  (Act (DealDamage (SelectAll (SameAs It)) (cast 1)))

-- a CLOSED condition reaches a named object via `Matches` (apply a predicate to a
-- reference) — "if ~ is a creature".
tClosedTypeCond : Condition Base
tClosedTypeCond = Matches This (HasType Creature)

-- ...and a filter is just a `Predicate` — the candidate is implicit, no `Subject`.
tSubjectFilter : Predicate Base
tSubjectFilter = HasType Creature

-- the unified `Quantity` (one `Range` constructor) + its helpers all typecheck
tQuantities : List (Bindable Base)
tQuantities =
  [ Choose (cast 2) creature              -- exactly 2 (the bare-numeral path)
  , Choose (atLeast (cast 1)) creature
  , Choose (atMost (cast 3)) creature
  , Choose (between (cast 1) (cast 3)) creature
  , Choose anyNumber creature
  ]

-- the event-query language: facets conjoin (`Query`), `Except` negates, timing via
-- `DuringTurn` — "a creature died, not during your turn".
tEventQuery : EventQuery Base
tEventQuery = Query [ KindIs (ZoneChanged (Just Battlefield) (Just Graveyard))
                    , SourceMatches creature
                    , Except (DuringTurn You) ]

-- a log-derived history count feeds a condition, and a game `Outcome` wraps into an effect
tHistoryThenWin : Effect Base
tHistoryThenWin =
  If (Compare (EventCount (Query [KindIs Cast, ActorIs You, Within ThisGame])) GreaterEq (Literal 2))
     (Conclude (WinGame You))

-- an activated ability: a multi-component cost algebra + an effect
tActivated : Ability
tActivated = Activated (Costs [Mana [cast 2], TapSelf, PayLife (Literal 1)])
                       (Act (Draw (cast 1)))

-- counters: the `HasCounter` predicate facet + the put/remove verbs
tCounters : Effect Base
tCounters = Sequence [ Act (PutCounters P1P1 (Literal 1) (SelectAll creature))
                     , Act (Destroy (SelectAll (IsNot (HasCounter P1P1)))) ]

-- anthem: a static `ModifyAll` over a controller-predicate filter, with layer mods
tAnthem : Ability
tAnthem = Static (ModifyAll (AllOf [HasType Creature, ControlledBy You]) [ModifyPT (cast 1) (cast 1), AddSubtype (cast Bear)])

-- a loyalty ability: an Activated ability whose cost removes Loyalty counters
tLoyalty : Ability
tLoyalty = Activated (RemoveCounters Loyalty (Literal 2)) (Act (Draw (cast 1)))

-- the value language: arithmetic, player attributes, counters-on, new stats, that-much
tValues : List (Count Base)
tValues =
  [ Plus (LifeTotal You) (HandSize Opponent)
  , Times (CountOf creature) (Literal 2)
  , HalfUp (StatOf This Power)
  , CountersOn P1P1 This
  , StatOf This ManaValue
  , ThatMuch ]

-- new verbs (scry/fight/token/search/copy) all typecheck
tVerbs : List (Effect Base)
tVerbs =
  [ Act (Scry (Literal 2))
  , Act (Fight (SelectAll (SameAs This)) (SelectAll creature))
  , Act (CreateToken (Literal 2) (MkToken "Soldier" [Creature] [] [White] 1 1))
  , With (Search {from = [Library, Graveyard]} (cast 1) (HasName "Forest")) (Act (Move That Hand))  -- tutor across two zones
  , Act (CopySpell (SelectAll (IsKind IsSpell))) ]

-- searching ANOTHER player's library (Bribery-style): `whose` names the zone owner;
-- the found creature goes to the battlefield via an owner-routed Move.
tSearchOther : Effect Base
tSearchOther = With (Search {whose = Opponent} (cast 1) creature) (Act (Move That Battlefield))

-- a conditional static, and an activation-limited (loyalty-style) ability
tConditionalStatic : Ability
tConditionalStatic = Static (While (exists (ControlledBy Opponent)) (Modify This [ModifyPT (cast 1) (cast 1)]))

tLimitedAbility : Ability
tLimitedAbility =
  Activated (RemoveCounters Loyalty (Literal 1)) (Act (Draw (cast 1))) {limits = [SorcerySpeed, OncePerTurn]}

-- P/T is in the value language now: SIGNED deltas (Up/Down) and a dynamic base via SetPT
tPTMods : List (Modification Base)
tPTMods =
  [ ModifyPT (Up (Literal 2)) (Down (Literal 1))     -- "+2/-1"
  , SetPT (CountOf creature) (CountOf creature) ]     -- Tarmogoyf-style "*/*" base P/T

-- a "*/*" creature: printed power/toughness are a `Count` (CDA), not a bare Int
tCDA : Card
tCDA = Normal $ fromDefault
  { name := "Test CDA"
  , types := [Creature]
  , power := Just (CountOf (HasType Land))
  , toughness := Just (Plus (CountOf (HasType Land)) (Literal 1)) }

-- NEGATIVE — each must be rejected --------------------------------------------

-- a 2nd target where only one was bound
failing
  tBadTargetRange : Effect Base
  tBadTargetRange = Targeted [anyTarget] (Act (DealDamage (SelectAll (SameAs (GetTarget 1))) (cast 1)))

-- `That` with no enclosing `With`
failing
  tBadThatOutsideWith : Selection Base
  tBadThatOutsideWith = That

-- a subtype whose category isn't among the card's types [CR#205.3d]
failing
  tBadSubtype : Card
  tBadSubtype = Normal $ fromDefault
    { name := "Bad", types := [Creature], subtypes := [cast Aura] }

-- a target leaking into a delayed body (`unbindTargets` clears it; only `That` crosses)
failing
  tBadDelayedTarget : Effect Base
  tBadDelayedTarget = Targeted [anyTarget]
    (Delayed nextEndStep (Act (DealDamage (SelectAll (SameAs (GetTarget 0))) (cast 1))))

-- `It` with no enclosing `ForEach`
failing
  tBadItOutside : Reference Base AnObject
  tBadItOutside = It

-- the split makes the old `CountOf (During …)` category error ILL-TYPED: `CountOf`
-- takes a `Predicate`, but `During` (a game-state test) is a `Condition`.
failing
  tBadCountOfCondition : Count Base
  tBadCountOfCondition = CountOf (During (MainPhase 0))

-- `EventObject` ("that card") is rejected outside a trigger/replacement/delayed body
-- (no `eventBound`) — the review fix that closed the ungated-anaphora hole.
failing
  tBadEventObjectOutside : Reference Base AnObject
  tBadEventObjectOutside = EventObject

-- one Reference, but the kind still bites: a player has no power/toughness
failing
  tBadStatOfPlayer : Count Base
  tBadStatOfPlayer = StatOf You Power       -- You : APlayer, StatOf wants AnObject

-- ...and an object has no life total
failing
  tBadLifeOfObject : Count Base
  tBadLifeOfObject = LifeTotal This         -- This : AnObject, LifeTotal wants APlayer
