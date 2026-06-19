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
tTargetInScope : Reference (bindTargets 1 Base)
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
    (Delayed BeginningOfEndStep (Act (Move That Battlefield)))

-- branching effects typecheck
tMay : Effect Base
tMay = May (Act (Draw (cast 1)))

tIf : Effect Base
tIf = If YourTurn (Act (Draw (cast 1)))

-- a one-shot creating a continuous effect for a duration
tContinuously : Effect Base
tContinuously = Continuously (Modify This [PlusPT 1 1]) UntilEndOfTurn

-- a modal effect: choose one of two modes
tModal : Effect Base
tModal = Modal (Choose (cast 1))
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
  (Act (DealDamage (SelectAll (isRef It)) (cast 1)))

-- the collapse: one predicate language. A CLOSED condition can read `This`
-- ("if ~ is a creature") — something a `Filter` could never say...
tClosedTypeCond : Condition Base
tClosedTypeCond = HasType This Creature

-- ...and a filter is just that same language with `Subject` in scope.
tSubjectFilter : Filter Base
tSubjectFilter = Where (HasType Subject Creature)

-- the unified `Quantity` (one `Range` constructor) + its helpers all typecheck
tQuantities : List (Selection Base)
tQuantities =
  [ SelectChoose (cast 2) creature              -- exactly 2 (the bare-numeral path)
  , SelectChoose (atLeast (cast 1)) creature
  , SelectChoose (atMost (cast 3)) creature
  , SelectChoose (between (cast 1) (cast 3)) creature
  , SelectChoose anyNumber creature
  ]

-- NEGATIVE — each must be rejected --------------------------------------------

-- a 2nd target where only one was bound
failing
  tBadTargetRange : Effect Base
  tBadTargetRange = Targeted [anyTarget] (Act (DealDamage (SelectAll (isRef (GetTarget 1))) (cast 1)))

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
    (Delayed BeginningOfEndStep (Act (DealDamage (SelectAll (isRef (GetTarget 0))) (cast 1))))

-- `It` with no enclosing `ForEach`
failing
  tBadItOutside : Reference Base
  tBadItOutside = It

-- the gate stays: `Subject` is rejected in a CLOSED condition (no candidate),
-- which is why `Condition` is still useful for triggered intervening-ifs
failing
  tBadSubjectClosed : Condition Base
  tBadSubjectClosed = HasType Subject Creature
