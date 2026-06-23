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
tTargetInScope : Reference (bindTargets [AnObject] Base) AnObject
tTargetInScope = GetTarget 0

-- `That` is available inside a `With`-bound bindings (kind comes from the binding)
tThatInWith : Selection (bindThat AnObject Base) AnObject
tThatInWith = That

-- a multi-type card may carry one subtype per card type [CR#205.3c]
tLandCreature : Card
tLandCreature = Normal $ ^:
  { name := Just "Test Land Creature"
  , types := [Land, Creature]
  , subtypes := [^Island, ^Bear]
  }

-- `That`, bound by a `With`, SURVIVES into a delayed body (captured); targets don't
tThatSurvivesDelay : OneShotEffect Base
tThatSurvivesDelay =
  With (Produce (Move (Only creature) Exile))
    (Delayed nextEndStep (ForEach That (Act (Move It Battlefield))))

-- branching effects typecheck
tMay : OneShotEffect Base
tMay = May (Act (Draw (^1)))

tIf : OneShotEffect Base
tIf = If yourTurn (Act (Draw (^1)))

-- a one-shot creating a continuous effect for a duration
tContinuously : OneShotEffect Base
tContinuously = Continuously (Modify This [ModifyPT (^1) (^1)]) UntilEndOfTurn

-- a modal effect: choose one of two modes
tModal : OneShotEffect Base
tModal = Modal (MkChooseSpec (^1))
  [ MkMode (Act (Draw (^1)))
  , MkMode (ForEach (SelectAll creature) (Act (DealDamage It (^2)))) {cost = Just (PayLife (Literal 2))}  -- mode cost is now a full Cost
  ]

-- `Reflexive` NESTS: inside a `With`, its body still sees `That` (no sibling scan)
tReflexiveSeesThat : OneShotEffect Base
tReflexiveSeesThat =
  With (Produce (Move (Only creature) Exile))
    (Reflexive (ForEach That (Act (Move It Battlefield))))

-- `ForEach` binds `It` per element; the body references `It`
tForEach : OneShotEffect Base
tForEach = ForEach (SelectAll (creature))
  (Act (DealDamage It (^1)))

-- a CLOSED condition reaches a named object via `Matches` (apply a predicate to a
-- reference) — "if ~ is a creature".
tClosedTypeCond : Condition Base
tClosedTypeCond = Matches This (HasType Creature)

-- ...and a filter is just a `Predicate` — the candidate is implicit, no `Subject`.
tSubjectFilter : Predicate Base AnObject
tSubjectFilter = HasType Creature

-- new filter atoms (close the audit's #1 hole): a numeric STAT comparison ("creature with power ≤
-- 2") and runtime OBJECT STATE ("an attacking creature") — both now `Predicate`s, not just `Condition`s.
tStatFilter : Predicate Base AnObject
tStatFilter = And [creature, StatCmp Power LessEq (^2)]

tStateFilter : Predicate Base AnObject
tStateFilter = And [creature, HasState Attacking, Not (HasState Tapped)]

-- an as-enters value choice in scope: `OfChosen` reads "the chosen color" under a `bindChosen AColor`
-- binding (Iona: "spells of the chosen color"). The card-level `AsEntersChoosing` opens this binding.
tOfChosen : Predicate (bindChosen AColor Base) AnObject
tOfChosen = And [IsKind IsSpell, OfChosen]

-- a MODAL as-enters choice: `ChosenIs i` reads the chosen mode, bounded by the mode count (Citadel/
-- Outpost Siege gate each ability on it). `AMode 2` ⇒ valid indices 0 and 1.
tChosenMode : Condition (bindChosen (AMode 2) Base)
tChosenMode = ChosenIs 1

-- restricted mana ([CR#106.5]): `onlyToCast` constrains the spend; `confers` rides the paid-for spell,
-- which is bound as `It` — Cavern's "creature spell of the chosen type, and that spell can't be countered".
tRestrictedMana : Action (bindChosen ACreatureType Base)
tRestrictedMana = AddMana [AnyColor]
  { onlyToCast = Just (And [IsKind IsSpell, creature, OfChosen])
  , confers    = [Cant (Countered (SameAs It))] }

-- the unified `Quantity` (one `Range` constructor) + its helpers all typecheck
tQuantities : List (Bindable Base AnObject)
tQuantities =
  [ Choose (^2) creature              -- exactly 2 (the bare-numeral path)
  , Choose (atLeast (^1)) creature
  , Choose (atMost (^3)) creature
  , Choose (between (^1) (^3)) creature
  , Choose anyNumber creature
  ]

-- the event-query language: facets conjoin (`And`), `Not` negates, timing via
-- `DuringTurn` — "a creature died, not during your turn".
tEventQuery : EventQuery Base
tEventQuery = And [ KindIs (ZoneChanged (Just Battlefield) (Just Graveyard))
                    , SourceMatches creature
                    , Not (DuringTurn you) ]

-- a log-derived history count feeds a condition, and a game `Outcome` wraps into an effect
tHistoryThenWin : OneShotEffect Base
tHistoryThenWin =
  If (Compare (EventCount (And [KindIs Cast, ActorIs you, Within ThisGame])) GreaterEq (Literal 2))
     (Conclude (WinGame You))

-- an activated ability: a multi-component cost algebra + an effect
tActivated : Ability Base
tActivated = Activated (Costs [Mana [^2], TapSelf, PayLife (Literal 1)])
                       (Act (Draw (^1)))

-- cost-payment DECISIONS (supersede `Unless`): MAY-pay (optional, reward + downside) and
-- MUST-pay (pay or be punished). The full `Cost` algebra rides both (here life / mana).
tMayPay : OneShotEffect Base
tMayPay = MayPay (PayLife (Literal 2)) (Act (Draw (^1))) {or_else = Just (Act (LoseLife (^1)))}

tMustPay : OneShotEffect Base
tMustPay = MustPay (Mana [^2]) (Act (Counter (Only (IsKind IsSpell))))

-- scaled cost: "{2} for each creature" — `Scaled` pays the inner cost once per the count.
tScaledCost : Cost Base
tScaledCost = Scaled (CountOf creature) (Mana [^2])

-- counters: the `HasCounter` predicate facet + the put/remove verbs
tCounters : OneShotEffect Base
tCounters = Sequence [ ForEach (SelectAll creature) (Act (PutCounters P1P1 (Literal 1) It))
                     , ForEach (SelectAll (Not (HasCounter P1P1))) (Act (Destroy It)) ]

-- anthem: a static `ModifyAll` over a controller-predicate filter, with layer mods
tAnthem : Ability Base
tAnthem = Static (ModifyAll (And [HasType Creature, ControlledBy you]) [ModifyPT (^1) (^1), AddSubtype (^Bear)])

-- a loyalty ability: an Activated ability whose cost removes Loyalty counters
tLoyalty : Ability Base
tLoyalty = Activated (RemoveCounters Loyalty (Literal 2)) (Act (Draw (^1)))

-- the value language: arithmetic, player attributes, counters-on, new stats, that-much
tValues : List (Count Base)
tValues =
  [ Plus (LifeTotal You) (HandSize You)
  , Times (CountOf creature) (Literal 2)
  , HalfUp (StatOf This Power)
  , CountersOn P1P1 This
  , StatOf This ManaValue
  , ThatMuch ]

-- new verbs (scry/fight/token/search/copy) all typecheck
tVerbs : List (OneShotEffect Base)
tVerbs =
  [ Act (Scry (Literal 2))
  , Act (Fight This (Only creature))
  , Act (CreateToken (Literal 2) (^: { name := Just "Soldier", types := [Creature], colors := [White], power := Just 1, toughness := Just 1 }))
  , With (Search {from = [Library, Graveyard]} (^1) (HasName "Forest")) (ForEach That (Act (Move It Hand)))  -- tutor across two zones
  , Act (CopySpell (Only (IsKind IsSpell))) ]

-- a token whose P/T is a `Count b` known at creation — "an X/X where X = creatures you control".
-- This is the payoff of parameterizing `Characteristics` by `b`: a card `Face` is `Characteristics
-- Base`, but a token's stats can read the live context, and both share the `^: { … }` builder.
tDynamicToken : OneShotEffect Base
tDynamicToken = Act (CreateToken (^1)
  (^: { name := Just "Ooze", types := [Creature], colors := [Green]
      , power := Just (CountOf creature), toughness := Just (CountOf creature) }))

-- a NAMELESS token (name defaults to Nothing): most tokens have no name. Only the lenient floor
-- (CharacteristicsOk: ≥1 type) is required — no name, no P/T-vs-type coupling (Vehicle/Tarmogoyf).
tNamelessToken : OneShotEffect Base
tNamelessToken = Act (CreateToken (^2)
  (^: { types := [Creature], colors := [White], power := Just 1, toughness := Just 1 }))

-- searching ANOTHER player's library (Bribery: "search target OPPONENT's library"): the
-- opponent is now a TARGET (player-predicate `opponent`), so `whose` is that targeted player.
tSearchOther : OneShotEffect Base
tSearchOther = Targeted [Target (^1) opponent]
  (With (Search {whose = GetTarget 0} (^1) creature) (ForEach That (Act (Move It Battlefield))))

-- a conditional static, and an activation-limited (loyalty-style) ability
tConditionalStatic : Ability Base
tConditionalStatic = Static (While (exists (ControlledBy opponent)) (Modify This [ModifyPT (^1) (^1)]))

tLimitedAbility : Ability Base
tLimitedAbility =
  Activated (RemoveCounters Loyalty (Literal 1)) (Act (Draw (^1))) {window = SorceryWindow, limits = [OncePerTurn]}

-- P/T is in the value language now: SIGNED deltas (Up/Down) and a dynamic base via SetPT
tPTMods : List (Modification Base)
tPTMods =
  [ ModifyPT (Up (Literal 2)) (Down (Literal 1))     -- "+2/-1"
  , SetPT (CountOf creature) (CountOf creature) ]     -- Tarmogoyf-style "*/*" base P/T

-- a "*/*" creature: printed power/toughness are a `Count` (CDA), not a bare Int
tCDA : Card
tCDA = Normal $ ^:
  { name := Just "Test CDA"
  , types := [Creature]
  , power := Just (CountOf (HasType Land))
  , toughness := Just (Plus (CountOf (HasType Land)) (Literal 1)) }

-- Stage 2: a target's kind comes from its slot's filter — a PLAYER target reads as a player
tPlayerTarget : Count (bindTargets [APlayer] Base)
tPlayerTarget = LifeTotal (GetTarget 0)

-- "each player" is a player-`Selection`; `ForEach` binds a player `It` (EachPlayer dissolved)
tEachPlayerForEach : OneShotEffect Base
tEachPlayerForEach = ForEach eachPlayer (Act (Draw {actor = It} (^1)))

-- MIXED-kind multi-target (Donate: "target player gains control of target permanent"):
-- slot 0 is a player, slot 1 an object — each `GetTarget` strictly kinded by its own slot.
tMixedTargets : OneShotEffect Base
tMixedTargets =
  Targeted [Target (^1) Anyone, Target (^1) (And [permanent, ControlledBy you])]
    (Continuously (Modify (GetTarget 1) [GainControl (GetTarget 0)]) Permanent)

-- `Or` computes its result kind by JOINING its arms' kinds (`\/`): same-kind stays
-- precise (`AnObject`), a mix of object + player widens to `Anything` — no `Widen` needed.
tOneOfKinds : (Predicate Base AnObject, Predicate Base Anything)
tOneOfKinds = (Or [creature, permanent], Or [creature, Anyone])

-- the join identity: an EMPTY `Or` folds to `Empty` (a vacuous union — matches nothing).
-- `Empty` is a distinct bottom kind, unusable where a real `AnObject`/`APlayer` is wanted.
tEmptyOneOf : Predicate Base Empty
tEmptyOneOf = Or []

-- a deontic Toll: Ward {2} — being targeted by an opponent's spell/ability stays fully legal,
-- but a trigger counters it unless {2} is paid (cost FIRST). The 4th polarity (Cant/Must/Gate
-- ride the deontic cards; Toll here).
tWard : StaticEffect Base
tWard = Toll (Mana [^2]) (BeTargeted (SameAs This) {by = ControlledBy opponent})

-- `keyword` desugars a spec to its `Ability`, in three flavors (all pinned by Refl): a DEONTIC
-- keyword is a `Composite` with a `Cant` clause; an engine-PRIMITIVE keyword is `Bare`; a
-- grammar FLAG (Reach) is a `Composite []`. `tHexproofFrom` shows the parameterized "from" case.
tDefender : keyword Defender = the (Ability Base) (Keyword (Composite Defender [Static (Cant (Attacks (SameAs This)))]))
tDefender = Refl

tFirstStrikeBare : keyword FirstStrike = the (Ability Base) (Keyword (Bare FirstStrike))
tFirstStrikeBare = Refl

tReachComposite : keyword Reach = the (Ability Base) (Keyword (Composite Reach []))
tReachComposite = Refl

tHexproofFrom : Ability Base
tHexproofFrom = keyword (Hexproof (Just (HasColor Red)))

-- the deontic permission floor `Can` (the 5th polarity, pairing with `Cant`)
tDeonticCan : Ability Base
tDeonticCan = Static (Can (Attacks (SameAs This)))

-- `AsThough` wraps a clause in a scoped counterfactual: "attack this turn as though it didn't
-- have defender" — a permission whose premise lifts defender's `Cant`.
tAsThough : OneShotEffect Base
tAsThough = Continuously
  (AsThough (Matches This (Not (HasKeyword Defender))) (Can (Attacks (SameAs This))))
  UntilEndOfTurn

-- Flash's desugaring is pinned by Refl: a `Can`-cast at instant speed (a widened window).
tFlashWindow : keyword Flash = the (Ability Base) (Keyword (Composite Flash [Static (Can (Casts (SameAs You) (SameAs This)) {window = Just InstantWindow})]))
tFlashWindow = Refl

-- Menace's desugaring (Refl): a SET-LEVEL `Cant` forbidding the lone-blocker (size-1) block —
-- `BlockedBy` constrains the whole set, unlike the per-blocker `Blocks` that flying uses.
tMenace : keyword Menace = the (Ability Base) (Keyword (Composite Menace [Static (Cant (BlockedBy (SameAs This) (^1)))]))
tMenace = Refl

-- Haste is a GRANTABLE keyword built from the as-though machinery (continuous; lifts summoning
-- sickness). Typechecks as an `Ability`; granted in real use via `GrantAbility (keyword Haste)`.
tHaste : Ability Base
tHaste = keyword Haste

-- the `^` prefix alias = `promote`: terse in lists / delimited position (`[^Red, ^1]`). In a
-- juxtaposed ARGUMENT it needs parens — `Draw (^1)` — since bare `Draw ^1` reads `^` as infix.
tPromoteOp : ManaCost
tPromoteOp = [^Red, ^1, ^Blue]

tPromoteOpArg : OneShotEffect Base
tPromoteOpArg = Act (Draw (^1))

-- `Single` demotes a selection to its sole element (the dual of `Only`); `GetTarget n` is sugar
-- for `Single (GetTargets n)`, so a plural slot is referenced as the group `GetTargets`.
tSingle : Reference Base AnObject
tSingle = Single (SelectAll creature)

-- a PLURAL target slot (1–2) feeds divided damage; the kind is the union (`Anything`)
tPluralTarget : OneShotEffect Base
tPluralTarget = Targeted [Target (between (^1) (^2)) (Or [creature, Anyone])]
  (Act (DealDamageDivided (^2) (GetTargets 0)))

-- NEGATIVE — each must be rejected --------------------------------------------

-- a 2nd target where only one was bound
failing
  tBadTargetRange : OneShotEffect Base
  tBadTargetRange = Targeted [anyTarget] (Act (DealDamage (GetTarget 1) (^1)))

-- a target slot can't target ZERO — `NonZeroQ` rejects a statically-zero upper bound
failing
  tBadZeroTarget : TargetSpec Base AnObject
  tBadZeroTarget = Target (^0) creature

-- a card with NO card types is rejected — `CharacteristicsOk` (the one lenient well-formedness floor)
failing
  tBadTypeless : Card
  tBadTypeless = Normal $ ^: { name := Just "Typeless" }

-- a two-faced card's BACK face is well-formedness-checked too, not just the front — a typeless back fails
failing
  tBadTwoFacedBack : Card
  tBadTwoFacedBack = TwoFaced Split (^: { types := [Instant] }) (^: { name := Just "Back" })

-- a 0-size block is rejected — a declared block has ≥1 blocker (`NonZeroQ` on `BlockedBy`'s size)
failing
  tBadZeroBlock : StaticEffect Base
  tBadZeroBlock = Cant (BlockedBy (SameAs This) (^0))

-- `OfChosen` with no as-enters choice in scope — `IsCharDomain Nothing = Void` denies the anaphor
failing
  tBadOfChosenNoChoice : Predicate Base AnObject
  tBadOfChosenNoChoice = OfChosen

-- `ChosenIs` past the mode count is rejected — `LT 2 2` is uninhabited (a 2-mode card, index 2)
failing
  tBadChosenMode : Condition (bindChosen (AMode 2) Base)
  tBadChosenMode = ChosenIs 2

-- `OfChosen` on a MODE choice is rejected — a mode isn't a characteristic (`IsCharDomain (AMode _) = Void`)
failing
  tBadOfChosenMode : Predicate (bindChosen (AMode 2) Base) AnObject
  tBadOfChosenMode = OfChosen

-- `That` with no enclosing `With`
failing
  tBadThatOutsideWith : Selection Base
  tBadThatOutsideWith = That

-- a subtype whose category isn't among the card's types [CR#205.3d]
failing
  tBadSubtype : Card
  tBadSubtype = Normal $ ^:
    { name := Just "Bad", types := [Creature], subtypes := [^Aura] }

-- a target leaking into a delayed body (`unbindTargets` clears it; only `That` crosses)
failing
  tBadDelayedTarget : OneShotEffect Base
  tBadDelayedTarget = Targeted [anyTarget]
    (Delayed nextEndStep (Act (DealDamage (GetTarget 0) (^1))))

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

-- Stage-2 strictness: a CREATURE target can't be read as a player — the hole the flex
-- `GetTarget` left open in Stage 1, now closed (its kind comes from `targetKinds`).
failing
  tBadLifeOfCreatureTarget : Count (bindTargets [AnObject] Base)
  tBadLifeOfCreatureTarget = LifeTotal (GetTarget 0)
