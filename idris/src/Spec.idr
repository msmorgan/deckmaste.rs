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

-- `That` is available inside a `With`-bound endophora (kind comes from the binding)
tThatInWith : Selection (bindThat Many AnObject Base) AnObject
tThatInWith = That

-- a ONE-binder's `That` reads back as a single `Reference` (no `Each`/`Single`)
tThatOneInWith : Reference (bindThat One AnObject Base) AnObject
tThatOneInWith = That

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
  With (Produce (Move (Only creature) (ToZone Exile)))
    (Delayed nextEndStep (Act (Move That (ToZone Battlefield))))

-- branching effects typecheck
tMay : OneShotEffect Base
tMay = May (Act (Draw (^1)))

tIf : OneShotEffect Base
tIf = If yourTurn (Act (Draw (^1)))

-- a one-shot creating a continuous effect for a duration
tContinuously : OneShotEffect Base
tContinuously = Continuously UntilEndOfTurn (Modify This (ApplyAll (modifyPT (Up (^1)))))

-- a modal effect: choose one of two modes
tModal : OneShotEffect Base
tModal = Modal (MkChooseSpec (^1))
  [ MkMode (Act (Draw (^1)))
  , MkMode (Each (Existing (SelectAll creature)) (Act (DealDamage It (^2)))) {cost = Just (Do (LoseLife (Literal 2)))}  -- mode cost is now a full Cost
  ]

-- VARIABLE-count modals: the choose-count is a `Quantity`. "Choose one or both" = `between (^1) (^2)`;
-- "choose one or more" (escalate-style) = `atLeast (^1)` (unbounded upper = implicitly the mode count).
tModalVariable : List (OneShotEffect Base)
tModalVariable =
  [ Modal (MkChooseSpec (between (^1) (^2))) [ MkMode (Act (Draw (^1))), MkMode (Act (GainLife (^2))) ]
  , Modal (MkChooseSpec (atLeast (^1)))      [ MkMode (Act (Draw (^1))), MkMode (Act (GainLife (^2))) ] ]

-- `Reflexive` NESTS: inside a `With`, its body still sees `That` (no sibling scan)
tReflexiveSeesThat : OneShotEffect Base
tReflexiveSeesThat =
  With (Produce (Move (Only creature) (ToZone Exile)))
    (Reflexive (Act (Move That (ToZone Battlefield))))

-- `Each` binds `It` per element; the body references `It`
tForEach : OneShotEffect Base
tForEach = Each (Existing (SelectAll (creature)))
  (Act (DealDamage It (^1)))

-- a CLOSED condition reaches a named object via `Matches` (apply a predicate to a
-- reference) — "if ~ is a creature".
tClosedTypeCond : Condition Base
tClosedTypeCond = Matches This (hasType Creature)

-- ...and a filter is just a `Predicate` — the candidate is implicit, no `Subject`.
tSubjectFilter : Predicate Base AnObject
tSubjectFilter = hasType Creature

-- new filter atoms (close the audit's #1 hole): a numeric STAT comparison ("creature with power ≤
-- 2") and runtime OBJECT STATE ("an attacking creature") — both now `Predicate`s, not just `Condition`s.
tStatFilter : Predicate Base AnObject
tStatFilter = And [creature, StatCmp Power AtMost (^2)]

-- DURATIVE aspect of the relation spine (`Holds r role`): "an attacking creature" (cf the retired
-- `HasState Attacking`).
tStateFilter : Predicate Base AnObject
tStateFilter = And [creature, Holds Attack Agent, Not (HasState Tapped)]

-- filter atoms: multicolored / colorless objects; stack-object filters (a spell targeting you, a single-
-- target spell); and the COMBAT states via the spine — `Blocking`/`Blocked` are the two SLOTS of one Block
-- relation, and "unblocked" is DERIVED (an attacker that is no block's patient). `Controls` is the
-- `ControlledBy` inverse (a player).
tFilterAtoms : List (Predicate Base AnObject)
tFilterAtoms =
  [ Multicolored
  , IsColorless
  , And [IsKind Spell, Targets (SameAs You)]
  , TargetCount Eq (^1)
  , Holds Block Agent                                                -- "a blocking creature"
  , Holds Block Patient                                             -- "a blocked creature"
  , And [creature, Holds Attack Agent, Not (Holds Block Patient)] ]  -- "an unblocked attacker" (derived, no state)

tControlsPlayer : Predicate Base APlayer
tControlsPlayer = Controls creature

-- INCHOATIVE aspect (`Begins r`): ONE onset event per attack/block; a FACET picks the side. `Begins Attack`
-- unifies what were the paired `Becomes Attacking` (doer side) and `Becomes Attacked` (patient side).
--   * "whenever this is blocked" — the patient side of a Block onset:
tBecomesBlocked : EventQuery Base
tBecomesBlocked = MkEventQuery [Begins Block] [Patient (SameAs This)]

-- DEFENDER-SIDE of combat ([CR#508.1]): an attack is attacker (DOER) → defender (PATIENT), the defender
-- kind-poly (a player, planeswalker, or battle) on the now kind-poly `Patient` facet.
--   * OBJECT defender — "whenever this planeswalker is attacked" (patient side of an Attack onset):
tBecomesAttacked : EventQuery Base
tBecomesAttacked = MkEventQuery [Begins Attack] [Patient (SameAs This)]

--   * the durative filter "the attacked planeswalker" — `Holds` is object-only (only objects bear durative
--     state; a player defender has none, and rides the event's `Patient` facet instead):
tAttackedFilter : Predicate Base AnObject
tAttackedFilter = And [hasType Planeswalker, Holds Attack Patient]

--   * PLAYER defender — "whenever you're attacked" (the kind-poly patient of an Attack onset):
tYouAreAttacked : EventQuery Base
tYouAreAttacked = MkEventQuery [Begins Attack] [Patient you]

-- the DOER side: "whenever this attacks or blocks" (Smuggler's Copter) — one `Begins` per relation, the
-- `Agent` facet pinning the attacker/blocker (the counterpart to the patient-side defender pins above).
tAttacksOrBlocks : EventQuery Base
tAttacksOrBlocks = MkEventQuery [Begins Attack, Begins Block] [Agent (SameAs This)]

-- designations: ONE predicate, scope by type — `HasDesignation Monarch` is a PLAYER test (you're the
-- monarch), `HasDesignation Monstrous` an OBJECT test. The carrier follows `designationScope`.
tMonarchTest : Predicate Base APlayer
tMonarchTest = HasDesignation Monarch

-- an as-enters value choice in scope: `OfChosen` reads "the chosen color" under a `bindChosen AColor`
-- binding (Iona: "spells of the chosen color"). The card-level `AsEnters AColor` opens this binding.
tOfChosen : Predicate (bindChosen AColor Base) AnObject
tOfChosen = And [IsKind Spell, OfChosen]

-- a MODAL as-enters choice: `ChosenIs i` reads the chosen mode, bounded by the mode count (Citadel/
-- Outpost Siege gate each ability on it). `AMode 2` ⇒ valid indices 0 and 1.
tChosenMode : Condition (bindChosen (AMode 2) Base)
tChosenMode = ChosenIs 1

-- restricted mana ([CR#106.6]): per-mana `riders` — `SpendOnly` constrains the spend; `GrantOnSpend`
-- rides the paid-for spell, bound as `It` — Cavern's "creature spell of the chosen type, uncounterable".
tRestrictedMana : Action (bindChosen ACreatureType Base)
tRestrictedMana = AddMana (^1) AnyColor
  { riders = [ SpendOnly (And [IsKind Spell, creature, OfChosen])
             , GrantOnSpend (cant (Enact Counter spellOrAbility (SameAs It))) ] }

-- the unified `Quantity` (one `Range` constructor) + its helpers all typecheck — `Choose` is a MANY binder
tQuantities : List (Bindable Base Many AnObject)
tQuantities =
  [ Choose (^2) creature              -- exactly 2 (the bare-numeral path)
  , Choose (atLeast (^1)) creature
  , Choose (atMost (^3)) creature
  , Choose (between (^1) (^3)) creature
  , Choose anyNumber creature
  ]

-- the ONE-binders: each binds a single object, read back as `That : Reference` (no `Each`/`Single`)
tOneBinders : List (Bindable Base One AnObject)
tOneBinders =
  [ Produce (Move (Only creature) (ToZone Exile))
  , ChooseOne creature
  , SearchOne {from = [Library]} (HasName "Forest")
  , TheRef (Only creature)
  ]

-- the event-query language: facets conjoin (`And`), `Not` negates, timing via
-- `Whenever (TurnOf …)` — "a creature died, not during your turn".
tEventQuery : EventQuery Base
tEventQuery = MkEventQuery [ZoneChanged (Just Battlefield) (Just Graveyard)]
                      [ Agent creature
                      , Not (Whenever (TurnOf you)) ]

-- PAYLOAD replacement: the event survives but its amount is rewritten — Furnace of Rath doubles damage
-- by scaling `EventAmount` (the event's own amount). The `newAmount` reads the event body.
tReplaceAmount : StaticEffect Base
tReplaceAmount = ReplaceAmount (MkEventQuery [DealDamage Nothing] []) (Times EventAmount (^2))

-- prevention is a REPLACEMENT (the damage amount set to zero) — fine as ReplaceAmount. But
-- indestructible is a PROHIBITION, not a replace-with-nothing: `keyword Indestructible` desugars to
-- `CantHappen (destroy of This)` (the two are semantically distinct — see CantHappen's note).
tPrevention : StaticEffect Base
tPrevention = ReplaceAmount (MkEventQuery [DealDamage Nothing] []) (^0)

-- CONSUMABLE shields (the `Replaces` use-limit). Regeneration = the next destroy → heal/tap/remove, one
-- use (`regenerate` macro). Prevention = "prevent the next 3 damage to This" — a damage `Replaces` whose
-- `UpTo (^3)` limit is consumed by 3 damage-points (`Patient (SameAs This)` = the affected object).
tRegenerate : OneShotEffect Base
tRegenerate = regenerate

tPreventNext : OneShotEffect Base
tPreventNext = Continuously UntilEndOfTurn
  (Replaces (MkEventQuery [DealDamage Nothing] [Patient (SameAs This)]) (Sequence []) {limit = UpTo (^3)})

-- Ward {2} ([CR#702.21a]): NO new machinery — a triggered ability over existing parts. When an opponent
-- casts a spell targeting This, that player (`EventActor`) MAY pay {2}; if not, the spell (`EventObject`)
-- is countered. Targets / MayPay (the unless-pay) / Counter were all already here.
tWard : Ability Base
tWard = Triggered (MkEventQuery [Begins Cast] [Patient (Targets (SameAs This)), Actor opponent])
  (MayPay {actor = EventActor} (Mana [^2]) (Sequence []) {or_else = Just (Act (Counter EventObject))})

tIndestructible : Ability Base
tIndestructible = keyword Indestructible

-- Devoid: "this is colorless" — now expressible via the unified `Set` clearing the color set.
tDevoid : Ability Base
tDevoid = keyword Devoid

-- protection from red: the DEBT bundle (damage/enchant/block/target from red) as ONE keyword macro.
tProtection : Ability Base
tProtection = protection (hasColor Red)

tOutcomeGate : StaticEffect Base
tOutcomeGate = OutcomeGate CantLose you

-- "Enchant creature" is the `enchant` MACRO — a bundle, not a keyword: (1) the PERMISSION to attach
-- (attaching is forbidden by DEFAULT, so it ENABLES this aura to attach to creatures — the dual of a
-- planeswalker granting `Can (Enact Attack … This)`); (2) the aura's SPELL (cast → target a host →
-- `Attach This (GetTarget 0)`). The falls-off SBA is conferred by the Aura SUBTYPE (`subtypeConfers`). The
-- non-cast "choose a valid host on ETB" rule is still pending a constrained-choice primitive.
tEnchant : List (Ability Base)
tEnchant = enchant creature

tAuraEnters : StaticEffect Base
tAuraEnters = Also thisEnters (With (ChooseOne creature) (Act (Attach This That)))

tAuraFallsOff : StaticEffect Base
tAuraFallsOff = Sba (Not (LegallyAttached This)) (Act (Move This (ToZone Graveyard)))

-- "your unspent mana doesn't empty" (Kruphix/Omnath) — a pool-policy static.
tManaPersists : StaticEffect Base
tManaPersists = ManaPersists you

-- conferrals are just `Ability`s (no `Property` wrapper): a +1/+1 counter confers its OWN P/T pump
-- (`counterConfers` → `Static (Modify …)`), and the Saga subtype confers the lore-increment `TurnBased`
-- action (`subtypeConfers`) — engine-read, no special-casing.
tCounterConfers : List (Ability Base)
tCounterConfers = counterConfers P1P1

tSubtypeConfers : List (Ability Base)
tSubtypeConfers = subtypeConfers (^Saga)

-- attacking a planeswalker: the TYPE confers a deontic permitting attacks on IT (`typeConfers`), not a
-- widening of the attacker. `Enact Attack`'s patient is kind-poly, so the permission names the permanent
-- itself (`SameAs This`, an OBJECT) as the defender.
tTypeConfers : List (Ability Base)
tTypeConfers = typeConfers Planeswalker

tAttacksObject : Deed Base
tAttacksObject = Enact Attack (hasType Creature) (SameAs This)

-- note read-back: a chosen card NAME is read by `OfChosen` (Meddling Mage), a chosen NUMBER by `ChosenNumber`.
tChosenName : Predicate (bindChosen AName Base) AnObject
tChosenName = And [IsKind Spell, OfChosen]

tChosenNumber : Count (bindChosen ANumber Base)
tChosenNumber = ChosenNumber

-- ...and a chosen PLAYER by `ChosenPlayer` ("as ~ enters, choose a player; that player …").
tChosenPlayer : Reference (bindChosenRef APlayer Base) APlayer
tChosenPlayer = ChosenPlayer

-- ...and a chosen OBJECT by `ChosenObject` — Clone ([CR#706.2]): "as ~ enters, you may have it enter as a
-- copy of a creature you choose." The copy is a continuous self-modification reading the chosen object.
-- The "a creature" restriction now rides `AsEntersChoosing`'s filter (no longer a separable gap); the
-- "you may" is the one remaining separable refinement.
tClone : Ability Base
tClone = AsEntersChoosing AnObject creature [ Static (Modify This (BecomeCopyOf ChosenObject)) ]

-- "sacrifice a [pred]" as a COST — the payer chooses which (not a specific `Sacrifice This`).
tSacrificeCost : Cost Base
tSacrificeCost = Do (Sacrifice creature)

-- phasing: the `PhasedOut` state filters a phased permanent; `PhaseOut` is the verb.
tPhasedFilter : Predicate Base AnObject
tPhasedFilter = And [creature, HasState PhasedOut]

-- morph: the `morph` macro = CastFaceDown ({3}) + TurnFaceUp ([cost]); `FaceDown` filters a morphed
-- permanent (its 2/2-vanilla body is the engine's global [CR#708.2] override).
tMorph : Ability Base
tMorph = morph (Mana [^1, ^Blue])

tFaceDownFilter : Predicate Base AnObject
tFaceDownFilter = And [creature, HasState FaceDown]

-- copy (minimal): a permanent BECOMES a copy of a reference (layer-1 Modification); a token COPY of a
-- reference. "a copy, except …" layers on as a separate higher-layer mod, not bundled here.
tBecomeCopy : Modification (bindTargets [AnObject] Base)
tBecomeCopy = BecomeCopyOf (GetTarget 0)

tCopy : Action (bindTargets [AnObject] Base)
tCopy = Copy (GetTarget 0)

-- stack-object redirection. `ChangeTarget … This` is Spellskite (named new target);
-- `ChooseNewTargets` is Bolt Bend / Redirect (a player picks). Both ride the original targetspec.
tChangeTarget : Action (bindTargets [AnObject] Base)
tChangeTarget = ChangeTarget (GetTarget 0) This

tChooseNewTargets : Action (bindTargets [AnObject] Base)
tChooseNewTargets = ChooseNewTargets (GetTarget 0)

-- Bolt Bend ([CR#115.7d]) end-to-end: TARGET a "spell or ability with a SINGLE target" — the single-target
-- restriction is just `TargetCount Eq (^1)` (an existing predicate, no new machinery) conjoined with
-- `spellOrAbility` — then the caster picks its new target. Pins the restriction TO the redirect action.
tBoltBend : OneShotEffect Base
tBoltBend =
  Targeted [Target (^1) (And [spellOrAbility, TargetCount Eq (^1)])]
    (Act (ChooseNewTargets (GetTarget 0)))

-- the structural holes: aggregate-stat cost (Crew), all-counters move (Ozolith), alternative base
-- cost (the base-SWAP type, distinct from CostChange). Solemnity is subsumed by Replaces+skip (a card).
tCrewCost : Cost Base
tCrewCost = TapTotal Power AtLeast (^3) creature

-- every-kind move (Ozolith / Fate Transfer): `MoveCounters AllKinds`
tMoveAllCounters : OneShotEffect Base
tMoveAllCounters = Targeted [Target (^1) creature] (Act (MoveCounters AllKinds This (GetTarget 0)))

-- single-kind move (Power Conduit / Leech Bonder): the general primitive that was previously inexpressible
tMoveSomeCounters : OneShotEffect Base
tMoveSomeCounters = Targeted [Target (^1) creature] (Act (MoveCounters (Some P1P1 (^1)) This (GetTarget 0)))

tMayCastFor : StaticEffect Base
tMayCastFor = MayCastFor [Do (LoseLife (^1))]

-- cast-from-zone: the alt-cost's `from` defaults to Hand; a non-default zone is the flashback family
-- ("cast this from your graveyard for {3}{U}"). The exile-after / exile-N riders compose on separately.
tCastFromGrave : StaticEffect Base
tCastFromGrave = MayCastFor [Mana [^3, ^Blue]] {from = [Graveyard]}

-- a log-derived history count feeds a condition, and a game `Outcome` wraps into an effect
tHistoryThenWin : OneShotEffect Base
tHistoryThenWin =
  If (Compare (CountEvents (MkEventQuery [Begins Cast] [Actor you, Within ThisGame])) AtLeast (Literal 2))
     (Conclude (WinGame You))

-- an activated ability: a multi-component cost algebra + an effect
tActivated : Ability Base
tActivated = Activated (Costs [Mana [^2], Do (Tap This), Do (LoseLife (Literal 1))])
                       (Act (Draw (^1)))

-- cost-payment DECISIONS (supersede `Unless`): MAY-pay (optional, reward + downside) and
-- MUST-pay (pay or be punished). The full `Cost` algebra rides both (here life / mana).
tMayPay : OneShotEffect Base
tMayPay = MayPay (Do (LoseLife (Literal 2))) (Act (Draw (^1))) {or_else = Just (Act (LoseLife (^1)))}

tMustPay : OneShotEffect Base
tMustPay = MustPay (Mana [^2]) (Act (Counter (Only (IsKind Spell))))

-- scaled cost: "{2} for each creature" — `Scaled` pays the inner cost once per the count.
tScaledCost : Cost Base
tScaledCost = Scaled (CountMatching creature) (Mana [^2])

-- continuous cost modification: affinity = a `Reduce` `ScaledBy` a count (one recursive node), and
-- devotion is just a `Count`, so it drops into mana production and cost-scaling alike.
tAffinity : StaticEffect Base
tAffinity = CostModifier (SameAs This) (ScaledBy (Reduce [^1]) (CountMatching (hasType Artifact)))

-- devotion to {B}{G} = SUM, over permanents you control, of the {B}/{G} pips in each one's printed cost
-- ([CR#700.5]): an `Aggregate SumOf` of a per-permanent symbol-count. A single `Or`-predicate (not two summed
-- counts), so a {B/G} hybrid pip counts once. (Was the bespoke `Devotion [Black, Green]`.)
tDevotion : Count Base
tDevotion = Aggregate SumOf (eachOf (And [permanent, ControlledBy you])
                                    (CountOf (ManaSymbols It (Or [CountsAs Black, CountsAs Green]))))

-- counters: the `HasCounter` predicate facet + the put/remove verbs
tCounters : OneShotEffect Base
tCounters = Sequence [ Each (Existing (SelectAll creature)) (Act (PutCounters P1P1 (Literal 1) It))
                     , Each (Existing (SelectAll (Not (HasCounter P1P1)))) (Act (Destroy It)) ]

-- anthem: a static `ModifyAll` over a controller-predicate filter, with layer mods
tAnthem : Ability Base
tAnthem = Static (Each (Existing (SelectAll (And [hasType Creature, ControlledBy you]))) (Modify It (ApplyAll [Alter Power (Up (^1)), Alter Toughness (Up (^1)), Alter Subtypes (Add (^Bear))])))

-- a loyalty ability: an Activated ability whose cost removes Loyalty counters
tLoyalty : Ability Base
tLoyalty = Activated (Do (RemoveCounters Loyalty (Literal 2) This)) (Act (Draw (^1)))

-- the value language: arithmetic, player attributes, counters-on, new stats, that-much
tValues : List (Count Base)
tValues =
  [ Plus (lifeTotal You) (handSize You)
  , Times (CountMatching creature) (Literal 2)
  , Half RoundUp (StatOf This Power)
  , CountersOn P1P1 This
  , ManaValueOf This                                     -- derived mana value ([CR#202.3]) — not a characteristic
  , Min (CountersOn P1P1 This) (CountersOn M1M1 This)   -- net counters after annihilation
  , Max (StatOf This Power) (^0)
  , Damage This                                          -- marked damage
  , EventAgg SumOf (MkEventQuery [DealDamage Nothing] [Actor opponent])    -- fold matching events' amounts (old `EventSum`); kinds amount-gated
  , Aggregate MaxOf (eachOf (And [permanent, creature, ControlledBy you]) (StatOf It Power))   -- "the greatest power among creatures you control" (on the battlefield, [CR#109.2])
  , Aggregate SumOf (eachOf (And [permanent, creature, ControlledBy you]) (StatOf It Power)) ] -- "the total power of creatures you control"

-- the aggregation value-language's headline cases, each the real card/keyword it models — positive coverage
-- for `CountDistinct` (characteristic × valid source), the `AverageOf`/`MinOf` folds, and the `Players` source.
tAggregations : List (Count Base)
tAggregations =
  [ CountDistinct Subtypes (Objects (And [permanent, hasType Land, ControlledBy you]))  -- Domain (granularity from the source filter)
  , CountDistinct Power (Objects (And [permanent, ControlledBy you]))                   -- Collector's Cage: distinct powers
  , CountDistinct Colors ManaSpent                                                      -- Sunburst / Converge: distinct colours of mana spent
  , CountDistinct Name (Objects (InZone Graveyard))                                     -- distinct names in graveyards
  , Aggregate (AverageOf RoundDown) (eachOf (And [permanent, creature, ControlledBy you]) (StatOf It Power))  -- rounded mean power
  , Aggregate MinOf (eachOf (And [permanent, creature, ControlledBy you]) (StatOf It Toughness))              -- least toughness
  , CountOf (Players (Controls creature)) ]                                                    -- number of players who control a creature

-- the extremal-ELEMENT form (`Pick`): "the creature with the greatest power among creatures you control" —
-- returns a `Selection`, reusing the very `Projection` the `Aggregate MaxOf` above folds to a value.
tGreatestPowerCreature : Selection Base AnObject
tGreatestPowerCreature = Pick MaxOf (eachOf (And [permanent, creature, ControlledBy you]) (StatOf It Power))

-- the GLOBAL [CR#704.5] state-based actions AS DATA — replacing the old loose `tLethalSba`, which was a
-- bare `Condition` with no scope (it silently assumed `This` was a creature) and no effect (the destroy
-- was gone). Each `SbaRule` now carries its domain (`scope`), trigger (`when`), AND action (`thenDo`).
-- Deathtouch [CR#704.5h] is ABSENT by design: it's intrinsic to the `Deathtouch` keyword ([CR#702.2c]
-- prospective lethality), not a keyword-independent global rule, so the engine bakes it in.
tGlobalSbas : List SbaRule
tGlobalSbas =
  [ -- lethal damage [CR#704.5g]: a creature with toughness > 0 whose marked damage ≥ toughness is destroyed
    MkSbaRule (And [creature, InZone Battlefield])
              (And [ Compare (StatOf This Toughness) Greater (^0)
                   , Compare (Damage This) AtLeast (StatOf This Toughness) ])
              (Act (Destroy This))
    -- 0-toughness [CR#704.5f]: toughness ≤ 0 → PUT INTO graveyard (a `Move`, NOT a `Destroy` — regeneration
    -- can't replace it; the Move-vs-Destroy choice is exactly what encodes that)
  , MkSbaRule (And [creature, InZone Battlefield])
              (Compare (StatOf This Toughness) AtMost (^0))
              (Act (Move This (ToZone Graveyard)))
    -- loyalty-0 [CR#704.5i]: a planeswalker with 0 loyalty counters → put into graveyard
  , MkSbaRule (And [hasType Planeswalker, InZone Battlefield])
              (Compare (CountersOn Loyalty This) Eq (^0))
              (Act (Move This (ToZone Graveyard))) ]

-- new verbs (scry/fight/token/search/copy) all typecheck
tVerbs : List (OneShotEffect Base)
tVerbs =
  [ scry (Literal 2)
  , fight This (Only creature)
  , Act (CreateToken (Literal 2) (^: { name := Just "Soldier", types := [Creature], colors := [White], power := Just 1, toughness := Just 1 }))
  , With (SearchOne {from = [Library, Graveyard]} (HasName "Forest")) (Act (Move That (ToZone Hand)))  -- tutor across two zones
  , Act (Copy (Only (IsKind Spell))) ]

-- a token whose P/T is a `Count b` known at creation — "an X/X where X = creatures you control".
-- This is the payoff of parameterizing `Characteristics` by `b`: a card `Face` is `Characteristics
-- Base`, but a token's stats can read the live context, and both share the `^: { … }` builder.
tDynamicToken : OneShotEffect Base
tDynamicToken = Act (CreateToken (^1)
  (^: { name := Just "Ooze", types := [Creature], colors := [Green]
      , power := Just (CountMatching creature), toughness := Just (CountMatching creature) }))

-- a NAMELESS token (name defaults to Nothing): most tokens have no name. Only the lenient floor
-- (CharacteristicsOk: ≥1 type) is required — no name, no P/T-vs-type coupling (Vehicle/Tarmogoyf).
tNamelessToken : OneShotEffect Base
tNamelessToken = Act (CreateToken (^2)
  (^: { types := [Creature], colors := [White], power := Just 1, toughness := Just 1 }))

-- searching ANOTHER player's library (Bribery: "search target OPPONENT's library"): the
-- opponent is now a TARGET (player-predicate `opponent`), so `whose` is that targeted player.
tSearchOther : OneShotEffect Base
tSearchOther = Targeted [Target (^1) opponent]
  (With (SearchOne {whose = GetTarget 0} creature) (Act (Move That (ToZone Battlefield))))

-- a conditional static, and an activation-limited (loyalty-style) ability
tConditionalStatic : Ability Base
tConditionalStatic = Static (While (exists (ControlledBy opponent)) (Modify This (ApplyAll (modifyPT (Up (^1))))))

tLimitedAbility : Ability Base
tLimitedAbility =
  Activated (Do (RemoveCounters Loyalty (Literal 1) This)) (Act (Draw (^1))) {window = AsSorcery, limits = [OncePerTurn]}

-- P/T in the value language: SIGNED deltas (Alter Power/Toughness Up/Down) and a dynamic base via `Set`.
tPTMods : List (Modification Base)
tPTMods =
  [ Alter Power (Up (Literal 2)), Alter Toughness (Down (Literal 1))     -- "+2/-1"
  , Alter Power (Set (CountMatching creature)), Alter Toughness (Set (CountMatching creature)) ]   -- a dynamic base-P/T set (the real */1+* CDA is card_Tarmogoyf)

-- the `Set` op overwrites ANY characteristic, value-typed by `CharValue`: "becomes blue", "loses all
-- creature types" (`Alter Subtypes (Set [])`), "becomes a legendary artifact creature".
tSetChars : List (Modification Base)
tSetChars =
  [ Alter Colors (Set [Blue])
  , Alter Subtypes (Set [])
  , Alter Types (Set [Artifact, Creature])
  , Alter Supertypes (Set [Legendary]) ]

-- ...and it's VALUE-TYPED by construction: a non-Color value for `Colors` is a type error.
failing
  tBadSetColorValue : Modification Base
  tBadSetColorValue = Alter Colors (Set [Creature])

-- a "*/*" creature: printed power/toughness are a `Count` (CDA), not a bare Int
tCDA : Card
tCDA = Normal $ ^:
  { name := Just "Test CDA"
  , types := [Creature]
  , power := Just (CountMatching (hasType Land))
  , toughness := Just (Plus (CountMatching (hasType Land)) (Literal 1)) }

-- Stage 2: a target's kind comes from its slot's filter — a PLAYER target reads as a player
tPlayerTarget : Count (bindTargets [APlayer] Base)
tPlayerTarget = lifeTotal (GetTarget 0)

-- "each player" is a player-`Selection`; `Each` binds a player `It` (EachPlayer dissolved)
tEachPlayerForEach : OneShotEffect Base
tEachPlayerForEach = Each (Existing eachPlayer) (Act (Draw {actor = It} (^1)))

-- MIXED-kind multi-target (Donate: "target player gains control of target permanent"):
-- slot 0 is a player, slot 1 an object — each `GetTarget` strictly kinded by its own slot.
tMixedTargets : OneShotEffect Base
tMixedTargets =
  Targeted [Target (^1) Anyone, Target (^1) (And [permanent, ControlledBy you])]
    (Continuously Forever (Modify (GetTarget 1) (GainControl (GetTarget 0))))

-- `Or` computes its result kind by JOINING its arms' kinds (`\/`): same-kind stays
-- precise (`AnObject`), a mix of object + player widens to `Anything` — no `Widen` needed.
tOneOfKinds : (Predicate Base AnObject, Predicate Base Anything)
tOneOfKinds = (Or [creature, permanent], Or [creature, Anyone])

-- the join identity: an EMPTY `Or` folds to `Empty` (a vacuous union — matches nothing).
-- `Empty` is a distinct bottom kind, unusable where a real `AnObject`/`APlayer` is wanted.
tEmptyOneOf : Predicate Base Empty
tEmptyOneOf = Or []

-- a deontic toll (`Priced Downstream`): Propaganda — creatures can't attack you UNLESS {2} is paid (cost FIRST). A toll is
-- pay-to-DO-the-action; ward is NOT one (it's a trigger that counters AFTER — see tWard).
tToll : StaticEffect Base
tToll = Priced Downstream (Mana [^2]) (Enact Attack creature you)

-- `keyword` desugars a spec to its `Ability`, in three flavors (all pinned by Refl): a DEONTIC
-- keyword is a `Composite` with a `cant` clause; an engine-PRIMITIVE keyword is `Bare`; a
-- grammar FLAG (Reach) is a `Composite []`. `tHexproofFrom` shows the parameterized "from" case.
tDefender : keyword Defender = the (Ability Base) (Keyword (Composite Defender [Static (cant (Enact Attack (SameAs This) Anyone))]))
tDefender = Refl

tFirstStrikeBare : keyword FirstStrike = the (Ability Base) (Keyword (Bare FirstStrike))
tFirstStrikeBare = Refl

tReachComposite : keyword Reach = the (Ability Base) (Keyword (Composite Reach []))
tReachComposite = Refl

tHexproofFrom : Ability Base
tHexproofFrom = keyword (Hexproof (Just (hasColor Red)))

-- the deontic permission floor `Can` (the 5th polarity, pairing with `cant`)
tDeonticCan : Ability Base
tDeonticCan = Static (Can (Enact Attack (SameAs This) Anyone))

-- `AsThough` wraps a clause in a scoped counterfactual: "attack this turn as though it didn't
-- have defender" — a permission whose premise lifts defender's `cant`.
tAsThough : OneShotEffect Base
tAsThough = Continuously UntilEndOfTurn
  (AsThough (Matches This (Not (HasKeyword Defender))) (Can (Enact Attack (SameAs This) Anyone)))

-- Flash's desugaring is pinned by Refl: a `Can`-cast at instant speed (a widened window).
tFlashWindow : keyword Flash = the (Ability Base) (Keyword (Composite Flash [Static (Can (Enact Cast (SameAs You) (SameAs This)) {window = Just AsInstant})]))
tFlashWindow = Refl

-- Menace's desugaring (Refl): a SET-LEVEL `cant` forbidding the lone-blocker (size-1) block —
-- `BlockedBy` constrains the whole set, unlike the per-blocker `Blocks` that flying uses.
tMenace : keyword Menace = the (Ability Base) (Keyword (Composite Menace [Static (cant (BlockedBy (SameAs This) (^1)))]))
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

-- a PLURAL target slot (1–2) feeds divided damage; the kind is the union (`Anything`). Divided damage is
-- the general `Distribute`: total `(^2)` split among the target group, each element dealt its `Allotment`.
tPluralTarget : OneShotEffect Base
tPluralTarget = Targeted [Target (between (^1) (^2)) (Or [creature, Anyone])]
  (Distribute (^2) (Existing (GetTargets 0)) (Act (DealDamage It Allotment)))

-- the SAME `Distribute` over a different body: "distribute three +1/+1 counters among any number of target
-- creatures" (Hunting Triad) — `PutCounters` per element, each getting its `Allotment`. Carrier-typed.
tDistributeCounters : OneShotEffect Base
tDistributeCounters = Targeted [Target (between (^1) (^3)) creature]
  (Distribute (^3) (Existing (GetTargets 0)) (Act (PutCounters P1P1 Allotment It)))

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

-- a PLAYER-carried counter can't go on an object — `counterScope Poison = APlayer`, so `This`
-- (an `AnObject` reference) is rejected with no runtime check. The dependent carrier is load-bearing.
failing
  tBadPoisonOnObject : Action Base
  tBadPoisonOnObject = PutCounters Poison (^1) This

-- granting a PLAYER designation to an object is a type error — `designationScope Monarch = APlayer`
failing
  tBadDesignationScope : Action Base
  tBadDesignationScope = GrantDesignation Monarch This

-- replacing the AMOUNT of an amountless event is rejected — a Cast has no numeric payload
failing
  tBadReplaceAmountless : StaticEffect Base
  tBadReplaceAmountless = ReplaceAmount (MkEventQuery [Begins Cast] []) (^0)

-- folding the amount of an amountless event is rejected likewise
failing
  tBadEventAggAmountless : Count Base
  tBadEventAggAmountless = EventAgg SumOf (MkEventQuery [Begins Cast] [])

-- "becomes summoning-sick" isn't a transition event — `IsBecomesState SummoningSick = Void`
failing
  tBadBecomesSummoningSick : EventKind
  tBadBecomesSummoningSick = Becomes SummoningSick

-- projecting a NON-object `Countable` is rejected — only `Objects` is `Projectable`, so `Project (Events …)`
-- has no `Projectable (Events …)` proof (you cannot bind `It` over an atomic event).
failing
  tBadProjectEvents : Projection Base
  tBadProjectEvents = Project (Events (MkEventQuery [DealDamage Nothing] [])) (Literal 0)

-- `CountDistinct` is gated by `readableOn`: an object-only characteristic over a non-object source is
-- rejected — "distinct powers of the mana you spent" is nonsense (`readableOn Power ManaSpent = Void`).
failing
  tBadDistinctStatOfMana : Count Base
  tBadDistinctStatOfMana = CountDistinct Power ManaSpent

-- ...and a non-colour characteristic over events is rejected too (`Name` reads nothing off an event).
failing
  tBadDistinctNameOfEvents : Count Base
  tBadDistinctNameOfEvents = CountDistinct Name (Events (MkEventQuery [DealDamage Nothing] []))

-- `Pick` is gated to the EXTREMAL ops by `IsExtremal`: argmax-by-SUM is meaningless (`IsExtremal SumOf` is uninhabited).
failing
  tBadPickNonExtremal : Selection Base AnObject
  tBadPickNonExtremal = Pick SumOf (eachOf creature (StatOf It Power))

-- an empty symbol disjunction ("devotion to no colours") is rejected — the restored `NonEmpty` guard.
failing
  tBadEmptySymbolOr : Countable Base
  tBadEmptySymbolOr = ManaSymbols This (Or [])

-- a `Distribute` share (`Allotment`) can't leak into a `Projection` accessor — `eachOf`/`Project` rebind `It`
-- via `bindIt`, which clears `hasAllotment`, so `Allotment` has no proof there (it was indexed to a DIFFERENT loop element).
failing
  tBadAllotmentInProjection : Projection Base
  tBadAllotmentInProjection = eachOf creature Allotment

-- THE INVALID-REFERENCE GATE: an event anaphor is valid only where the event SUPPLIES it (`eventQueryCaps`).
-- `EventObject` ("that card") in a step-begin body — a `BeginStep` event has no object.
failing
  tBadEventObjectNoObject : Ability Base
  tBadEventObjectNoObject =
    Triggered (MkEventQuery [BeginStep (BeginningPhase UpkeepStep)] []) (Act (Move EventObject (ToZone Exile)))

-- `EventAmount` (the amount) in a `Begins Cast` body — a cast carries no amount.
failing
  tBadThatMuchNoAmount : StaticEffect Base
  tBadThatMuchNoAmount = Replaces (MkEventQuery [Begins Cast] []) (Act (DealDamage This EventAmount))

-- `EventActor` ("that player") in a Destroy body — a destruction has no actor.
failing
  tBadEventActorNoActor : Ability Base
  tBadEventActorNoActor = Triggered (MkEventQuery [Destroy] []) (Conclude (WinGame EventActor))

-- ...and the anaphora DO work where the event supplies them: `EventActor` in a `Begins Cast` body (the caster).
tEventActorValid : Ability Base
tEventActorValid = Triggered (MkEventQuery [Begins Cast] []) (Conclude (WinGame EventActor))

-- MULTI-KIND SOUNDNESS (the EventQuery restructure): a multi-kind query's caps are the INTERSECTION —
-- the body gets only anaphora EVERY listed kind supplies. `EventActor` under `[Begins Cast, Destroy]` is
-- rejected (a Destroy event has no actor), so the old union-cap leak (A6) is gone.
failing
  tBadEventActorMultiKind : Ability Base
  tBadEventActorMultiKind = Triggered (MkEventQuery [Begins Cast, Destroy] []) (Conclude (WinGame EventActor))

-- ...but when EVERY listed kind supplies the anaphor it's fine: "attacks or blocks" both supply an
-- object, so `EventObject` is valid (Smuggler's Copter's single trigger over two kinds).
tEventObjectMultiKind : Ability Base
tEventObjectMultiKind =
  Triggered (MkEventQuery [Begins Attack, Begins Block] []) (Act (Move EventObject (ToZone Exile)))

-- "whenever a creature enters, draw THAT MANY cards" — meaningless: a creature entering (`ZoneChanged`)
-- carries no amount, so `EventAmount` has no referent. The caps gate rejects it.
failing
  tBadDrawThatManyOnEnter : Ability Base
  tBadDrawThatManyOnEnter =
    Triggered (MkEventQuery [ZoneChanged Nothing (Just Battlefield)] [Agent creature])
      (Act (Draw EventAmount))

-- BOUNDED-NUMERIC gates. An inverted range ("between 5 and 2") — `OrderedRange` rejects `lo > hi`.
failing
  tBadInvertedRange : Bindable Base AnObject
  tBadInvertedRange = Choose (between (^5) (^2)) creature

-- `MainPhase` is a closed 2-value enum now, not a `Nat` — `MainPhase 99` doesn't typecheck.
failing
  tBadMainPhase99 : PhaseStep
  tBadMainPhase99 = MainPhase 99

-- a modal "choose 5" of a single mode — `ModalCountOk` bounds the literal count by the mode count.
failing
  tBadModalOverCount : OneShotEffect Base
  tBadModalOverCount = Modal (MkChooseSpec (^5)) [ MkMode (Act (Draw (^1))) ]

-- a modal with NO modes — `NonEmpty modes` rejects it.
failing
  tBadModalEmptyModes : OneShotEffect Base
  tBadModalEmptyModes = Modal (MkChooseSpec (^1)) []

-- a 0-way mode domain — `ModeDomainOk (AMode 0)` is `LT 0 0` = uninhabited.
failing
  tBadModeDomainZero : Ability Base
  tBadModeDomainZero = AsEnters (AMode 0) []

-- (No `tBad…` for "produce {X}/{W/P}" or "cost with any-color": printed `ManaSymbol` and `ProducedMana`
-- are SEPARATE types, so those are unrepresentable by construction — a `failing` block there would only
-- test the type distinction, not the model.)

-- a 0-size block is rejected — a declared block has ≥1 blocker (`NonZeroQ` on `BlockedBy`'s size)
failing
  tBadZeroBlock : StaticEffect Base
  tBadZeroBlock = cant (BlockedBy (SameAs This) (^0))

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

-- `OfChosen` on an as-enters ENTITY choice is rejected — an object is identity, not a characteristic, and
-- it binds `chosenRefKind` (NOT `chosenKind`), so `OfChosen`'s `IsCharDomain (chosenKind b)` finds
-- `Nothing` → `Void`. Read a chosen object with `ChosenObject`/`SameAs`, never `OfChosen`.
failing
  tBadOfChosenObject : Predicate (bindChosenRef AnObject Base) AnObject
  tBadOfChosenObject = OfChosen

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

-- `It` with no enclosing `Each`
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
  tBadLifeOfObject = lifeTotal This         -- This : AnObject, lifeTotal wants APlayer

-- Stage-2 strictness: a CREATURE target can't be read as a player — the hole the flex
-- `GetTarget` left open in Stage 1, now closed (its kind comes from `targetKinds`).
failing
  tBadLifeOfCreatureTarget : Count (bindTargets [AnObject] Base)
  tBadLifeOfCreatureTarget = lifeTotal (GetTarget 0)
