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
  With (Produce (Move (SelectFilter (IsType Creature)) Exile))
    (Delayed BeginningOfEndStep (Act (Move That Battlefield)))

-- NEGATIVE — each must be rejected --------------------------------------------

-- a 2nd target where only one was bound
failing
  tBadTargetRange : Effect Base
  tBadTargetRange = Targeted [anyTarget] (Act (DealDamage (SelectRef (GetTarget 1)) 1))

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
    (Delayed BeginningOfEndStep (Act (DealDamage (SelectRef (GetTarget 0)) 1)))

-- `IsTargeted` where no target is bound
failing
  tBadIsTargeted : Filter Base
  tBadIsTargeted = IsTargeted
