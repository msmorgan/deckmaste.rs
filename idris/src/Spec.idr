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
  With (Produce (Move (SelectAll (IsType Creature)) Exile))
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
  , MkMode (Act (DealDamage (SelectAll (IsType Creature)) (cast 2)))
  ]

-- `Reflexive` NESTS: inside a `With`, its body still sees `That` (no sibling scan)
tReflexiveSeesThat : Effect Base
tReflexiveSeesThat =
  With (Produce (Move (SelectAll (IsType Creature)) Exile))
    (Reflexive (Act (Move That Battlefield)))

-- `ForEach` binds `It` per element; the body references `It`
tForEach : Effect Base
tForEach = ForEach (SelectAll (IsType Creature))
  (Act (DealDamage (SelectAll (IsRef It)) (cast 1)))

-- NEGATIVE — each must be rejected --------------------------------------------

-- a 2nd target where only one was bound
failing
  tBadTargetRange : Effect Base
  tBadTargetRange = Targeted [anyTarget] (Act (DealDamage (SelectAll (IsRef (GetTarget 1))) (cast 1)))

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
    (Delayed BeginningOfEndStep (Act (DealDamage (SelectAll (IsRef (GetTarget 0))) (cast 1))))

-- `IsTargeted` where no target is bound
failing
  tBadIsTargeted : Filter Base
  tBadIsTargeted = IsTargeted

-- `It` with no enclosing `ForEach`
failing
  tBadItOutside : Reference Base
  tBadItOutside = It
