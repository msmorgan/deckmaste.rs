||| The regression suite (v2 — the antecedent stack). Building the package
||| (`scripts/build`) typechecks this module: the positives must typecheck, and
||| every `failing "<message>"` block must FAIL to typecheck WITH the pinned
||| message in the error (each asserts a type-level invariant still bites, and
||| the pin names the rule — a block that fails for an unrelated reason no
||| longer counts, killing the vacuous-block class). Each negative changes
||| exactly one thing from a known-good construction.
|||
||| These Idris negatives are the same soundness invariants the
||| `cargo xtask idris-check` re-emit gate enforces on the Rust card corpus:
||| each card is re-emitted to raw Core.idr and typechecked.
module Spec

import Core
import Macros

%default total

-- concrete announce-slot antecedents for raw-context positives/negatives
-- (what `Targeted [Target (^1) creature]` ANNOUNCES, spelled as a value —
-- these populate `Ctx.targets`, the announce-list channel, never the stack).
-- `public export` so the resolve proofs REDUCE through them; Capitalized —
-- an arg-free lowercase name in a type position would be implicitly bound.
public export
creatureSlotAnte : Ant
creatureSlotAnte = MkAnt (OfType Creature) AnObject One TargetSlot Nothing Nothing

public export
playerSlotAnte : Ant
playerSlotAnte = MkAnt Player APlayer One TargetSlot Nothing Nothing

public export
CtxCreatureTarget : Ctx
CtxCreatureTarget = bindTargets [creatureSlotAnte] Base

public export
CtxPlayerTarget : Ctx
CtxPlayerTarget = bindTargets [playerSlotAnte] Base

-- POSITIVE — must typecheck ---------------------------------------------------

-- COMPAT PINS: `compat`'s valuation over one representative Sort per key,
-- pinned by Refl — one boolean per pair. If `compat` moves, this Refl breaks
-- before it can silently drift ([CR#108.2,110.1,205.2a,405.1]).
public export
compatRowPairs : List (Sort, Sort)
compatRowPairs =
  [ (Player, Player), (Card, Card), (Token, Token), (Spell, Spell)
  , (StackObject, Spell), (StackObject, StackObject)
  , (Permanent, Permanent), (Permanent, OfType Creature), (Permanent, Token)
  , (OfType Creature, OfType Creature), (Amount, Amount), (Pile, Pile) ]

tCompatRows : map (\p => compat (fst p) (snd p)) Spec.compatRowPairs
            = [True, True, True, True, True, True, True, True, True, True, True, True]
tCompatRows = Refl

-- …and the NEGATIVE SPACE the table leaves closed: "that card" never reaches
-- a token ([CR#108.2b]); a typed noun reaches only its own type ([CR#205.2a]);
-- widening is one-directional (a creature anaphor doesn't reach a bare
-- Permanent antecedent); the wildcard never reaches a value antecedent.
public export
compatClosedPairs : List (Sort, Sort)
compatClosedPairs =
  [ (Card, Token), (Card, Player), (OfType Creature, OfType Land)
  , (OfType Creature, Permanent), (Spell, StackObject), (Amount, Card) ]

tCompatClosed : map (\p => compat (fst p) (snd p)) Spec.compatClosedPairs
              = [False, False, False, False, False, False]
tCompatClosed = Refl

tWildNeverAmount : wildReaches Amount = False
tWildNeverAmount = Refl

-- pin `zoneSort`'s full valuation ([CR#110.1,112.1,108.2]).
tZoneSorts : map Core.zoneSort [Battlefield, Stack, Graveyard, Hand, Library, Exile, Command, Sideboard]
           = the (List Sort) [Permanent, Spell, Card, Card, Card, Card, Card, Card]
tZoneSorts = Refl

-- the planar-die union LOSES the amount ([CR#706.7]): "roll one or more dice" incl. planar can't read a
-- numeric result, because `RollPlanarDie` carries none ([CR#901.9d]).
tRollUnionAmountless : kindsHaveAmount [RollDice, RollPlanarDie Nothing] = False
tRollUnionAmountless = Refl

-- a pure numeric-die query keeps its amount; `RollDice`/`FlipCoin` alone are roll-more-eligible
-- randomness ([CR#614.3]), the planar die is not ([CR#901.9d]).
tRollDiceAloneHasAmount : kindsHaveAmount [RollDice] = True
tRollDiceAloneHasAmount = Refl

-- the planar-die union is likewise NOT randomness-eligible ([CR#614.3,901.9d]): this gates the future
-- `ReplaceRoll` — a "roll one or more dice" that includes the planar die can't be Krark's Thumb'd.
tRollPlanarUnionNotRandomness : kindsAreRandomness [RollDice, RollPlanarDie Nothing] = False
tRollPlanarUnionNotRandomness = Refl

-- the positive companion: `RollDice`/`FlipCoin` alone stay randomness-eligible ([CR#614.3]).
tRollDiceFlipCoinAreRandomness : kindsAreRandomness [RollDice, FlipCoin Nothing] = True
tRollDiceFlipCoinAreRandomness = Refl

-- lifted onto the `EventQuery` wrapper (a named `EventQuery Base` value, mirroring `tBecomesBlocked`
-- et al. — pins `b` before elaborating `MkEventQuery`, avoiding the ambiguous-`b` trap an inline
-- `isRandomnessQuery (MkEventQuery ...) = False` hits with no outer `EventQuery b` signature).
tRollPlanarQuery : EventQuery Base
tRollPlanarQuery = MkEventQuery [RollDice, RollPlanarDie Nothing] []

tEventQueryRollPlanarNotRandomness : isRandomnessQuery Spec.tRollPlanarQuery = False
tEventQueryRollPlanarNotRandomness = Refl

-- THE stack-weakening lemma, exercised ([type-theory#4]): pushing a
-- NON-candidate (here a player role antecedent, against a "that card" read)
-- moves no resolution — one lemma over the one resolve function. Sorted
-- anaphors are deliberately NOT weakenable by a COMPATIBLE push: that
-- instability IS the R2 ambiguity gate (see tBadAmbiguousThat).
tWeaken : resolveStack (Just Card) One
            (MkAnt Player APlayer One EventRole Nothing Nothing
              :: [MkAnt Card AnObject One Product Nothing Nothing])
        = resolveStack (Just Card) One [MkAnt Card AnObject One Product Nothing Nothing]
tWeaken = weakenResolve {w = Just Card} {cd = One}
                        {s = [MkAnt Card AnObject One Product Nothing Nothing]}
                        (MkAnt Player APlayer One EventRole Nothing Nothing) Refl

-- an announced slot is read back BY POSITION ([CR#115.3,601.2c]) — Lightning
-- Bolt's `DealDamage(This, 3, Target(0))`. This is the ONLY way to name a
-- target; `tBadItReadsNoTarget`/`tBadThatReadsNoTarget` below pin that the
-- anaphors cannot.
tTargetInScope : OneShotEffect Base
tTargetInScope = Targeted [Target (^1) creature] (Act (destroy (Target 0)))

-- the slot's KIND comes from the slot: reading it at the wrong kind is a type
-- error, not a runtime surprise (`tBadLifeOfCreatureTarget`).
tTargetKindFromSlot : OneShotEffect Base
tTargetKindFromSlot = Targeted [Target (^1) creature] (Act (Tap (Target 0)))

-- a Many-binder's group is read back as the plural anaphor `They`,
-- iterated with `Each` (the group-move shape).
tTheyInWith : OneShotEffect Base
tTheyInWith = With (Choose (^2) inHand) (Each (Existing They) (Act (Move It (ToZone Graveyard))))

-- a ONE-binder's choice is the deterministic FRAME — `It` reads it directly.
tItInWith : OneShotEffect Base
tItInWith = With (ChooseOne creature) (Act (destroy It))

-- a multi-type card may carry one subtype per card type [CR#205.3c]
tLandCreature : Card
tLandCreature = Normal $ ^:
  { name := Just "Test Land Creature"
  , types := [Land, Creature]
  , subtypes := [landType "Island", creatureType "Bear"]
  }

-- a produced object, bound by a `With (Produce …)`, SURVIVES into a delayed
-- body ([CR#603.7c] — Products survive; targets don't): the exiled object
-- answers to "card" in exile, so the delayed body reads `That Card`.
tThatSurvivesDelay : OneShotEffect Base
tThatSurvivesDelay =
  With (Produce (Move (Only creature) (ToZone Exile)))
    (Delayed nextEndStep (Act (Move (That Card) (ToZone Battlefield))))

-- the TELESCOPE ([CR#608.2d]): sentence order IS binder order. Cloudshift —
-- "Exile target creature you control, then return that card to the
-- battlefield": the exile clause's product answers to "card" (its new zone),
-- so the second clause's `That Card` resolves with no binder inversion.
tTelescopeCloudshift : OneShotEffect Base
tTelescopeCloudshift =
  Targeted [Target (^1) (And [creature, ControlledBy you])]
    (Sequentially [ Act (Move (Target 0) (ToZone Exile))
              , Act (Move (That Card) (ToZone Battlefield)) ])

-- "Create two tokens. THEY gain haste. Sacrifice them at the next end step."
-- — the Many product antecedent ([CR#111.2]), read as `Them Token`/`They`,
-- surviving into the delayed body (the Chandra [0] shape, unrepresentable
-- in v1 where `Produce` was One-only).
tTelescopeTokens : OneShotEffect Base
tTelescopeTokens =
  Sequentially [ Act (CreateToken (^2) (^: { types := [Creature], subtypes := [creatureType "Elemental"], colors := [Red], power := Just 1, toughness := Just 1 }))
           , Continuously UntilEndOfTurn (Each (Existing (Them Token)) (Modify It (GrantAbility (keyword Haste))))
           , Delayed nextEndStep (Each (Existing They) (Act (Move It (ToZone Graveyard)))) ]

-- "Draw three cards … gain THAT MUCH life" — the value anaphor over the
-- amount antecedent a card-flow verb pushes ([CR#608.2i]).
tThatMany : OneShotEffect Base
tThatMany = Sequentially [ Act (Draw (^3)), Act (GainLife ThatMany) ]

-- the INDEFINITE choice + the May-intro flow (Through the Breach): "You
-- may put a creature card from your hand onto the battlefield. That
-- permanent gains haste. Sacrifice it at the next end step." The
-- `With (ChooseOne …) … It` binds the chosen card, the Move pushes the
-- battlefield product (`Permanent` — its new zone's noun), and a DECLINED
-- May's products are runtime-skipped, not scope-blocked ([CR#701.23b,608.2d]).
tIndefiniteMay : OneShotEffect Base
tIndefiniteMay =
  Sequentially [ May (With (ChooseOne (And [inHand, creature])) (Act (Move It (ToZone Battlefield))))
           , Continuously UntilEndOfTurn (Modify (That Permanent) (GrantAbility (keyword Haste)))
           , Delayed nextEndStep (Act (Move (That Permanent) (ToZone Graveyard))) ]

-- POSITIONAL announce-slot reads ([CR#115.3]): two same-sort slots would trip
-- the R2 gate on a sorted anaphor (tBadAmbiguousThat), so the Arc Trail shape
-- reads them back by index (`Target n`) instead — and `Distinct` pins the
-- "any OTHER target" co-target constraint ([CR#115.7e]).
tLabeledSlots : OneShotEffect Base
tLabeledSlots =
  Targeted [ anyTarget, Distinct [0] anyTarget ]
    (Sequentially [ Act (DealDamage (^2) (Target {k = Anything} 0))
              , Act (DealDamage (^1) (Target {k = Anything} 1)) ])

-- an event-role antecedent serves the SORTED anaphor (the stack side of the
-- caps machinery): a discard trigger's object went to a graveyard, so its
-- role antecedent answers to "card" ([CR#603.2e,400.7e]).
tEventRoleThat : Ability Base
tEventRoleThat = Triggered (MkEventQuery [Discard] [Actor opponent])
  (Act (Move (That Card) (ToZone Exile)))

-- …and where the event's role antecedent COLLIDES with an announced slot,
-- a POSITIONAL read disambiguates without loosening R2 (the Goblin Medics
-- canon shape — mirror of `plugins/canon/cards/Goblin Medics.ron`).
tTriggerTargetLabel : Ability Base
tTriggerTargetLabel = Triggered (MkEventQuery [Becomes Tapped] [Agent (SameAs This)])
  (Targeted [anyTarget] (Act (DealDamage (^1) (Target {k = Anything} 0))))

-- branching effects typecheck
tMay : OneShotEffect Base
tMay = May (Act (Draw (^1)))

tIf : OneShotEffect Base
tIf = If yourTurn (Act (Draw (^1)))

-- a one-shot creating a continuous effect for a duration
tContinuously : OneShotEffect Base
tContinuously = Continuously UntilEndOfTurn (Modify This (ApplyAll (modifyPT (Up (^1)))))

-- a modal effect: choose one of two modes ([CR#700.2]; the mode vector is
-- non-empty BY CONSTRUCTION — `Vect (S n)`)
tModal : OneShotEffect Base
tModal = Modal (MkChooseSpec (^1))
  [ MkMode (Act (Draw (^1)))
  , MkMode (Each (Existing (SelectAll creature)) (Act (DealDamage (^2) It))) {cost = Just (Do (LoseLife (Literal 2)))}  -- mode cost is now a full Cost
  ]

-- VARIABLE-count modals: the choose-count is a `Quantity`. "Choose one or both" = `between (^1) (^2)`;
-- "choose one or more" (escalate-style) = `atLeast (^1)` (unbounded upper = implicitly the mode count).
tModalVariable : List (OneShotEffect Base)
tModalVariable =
  [ Modal (MkChooseSpec (between (^1) (^2))) [ MkMode (Act (Draw (^1))), MkMode (Act (GainLife (^2))) ]
  , Modal (MkChooseSpec (atLeast (^1)))      [ MkMode (Act (Draw (^1))), MkMode (Act (GainLife (^2))) ] ]

-- `Reflexive` NESTS: inside a `With`, its body still sees the bound product
-- (no sibling scan; the full outer context, [CR#603.12a])
tReflexiveSeesThat : OneShotEffect Base
tReflexiveSeesThat =
  With (Produce (Move (Only creature) (ToZone Exile)))
    (Reflexive (Act (Move (That Card) (ToZone Battlefield))))

-- `Each` binds `It` per element; the body references `It`
tForEach : OneShotEffect Base
tForEach = Each (Existing (SelectAll (creature)))
  (Act (DealDamage (^1) It))

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

-- designations: ONE predicate, scope by type — `HasDesignation monarch` is a PLAYER test (you're the
-- monarch), `HasDesignation monstrous` an OBJECT test. The carrier follows `designationKindScope`.
tMonarchTest : Predicate Base APlayer
tMonarchTest = HasDesignation monarch

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

-- the ONE-binders: each binds a single object — a choice binds a FRAME (read
-- deterministically), a produce/search binder a whiffable Product (read by sort)
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
  (Replaces (MkEventQuery [DealDamage Nothing] [Patient (SameAs This)]) (Sequentially []) {limit = UpTo (^3)})

-- Ward {2} ([CR#702.21a]): NO new machinery — a triggered ability over existing parts. When an opponent
-- casts a spell targeting This, that player (`EventActor`) MAY pay {2}; if not, the spell (`EventObject`)
-- is countered. Targets / the collapsed May(Pay) (the unless-pay, [CR#118.12a]) / Counter were all already here.
tWard : Ability Base
tWard = Triggered (MkEventQuery [Begins Cast] [Patient (Targets (SameAs This)), Actor opponent])
  (mayPayCostBy EventActor (Mana [^2]) Nothing (Just (Act (Counter EventObject))))

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
-- attach, the host read back positionally). The falls-off SBA is conferred by the Aura
-- SUBTYPE (`subtypeConfers`). The non-cast "choose a valid host on ETB" rule rides the `Also`.
tEnchant : List (Ability Base)
tEnchant = enchant creature

tAuraEnters : StaticEffect Base
tAuraEnters = Also thisEnters (With (ChooseOne creature) (Act (Attach This It)))

tAuraFallsOff : StaticEffect Base
tAuraFallsOff = Sba (Not (LegallyAttached This)) (Act (Move This (ToZone Graveyard)))

-- "your unspent mana doesn't empty" (Kruphix/Omnath) — a pool-policy static.
tManaPersists : StaticEffect Base
tManaPersists = ManaPersists you

-- conferrals are just `Ability`s (no `Property` wrapper): a +1/+1 counter confers its OWN P/T pump
-- (`counterConfers` → `Static (Modify …)`), and the Saga subtype confers the lore-increment `TurnBased`
-- action (`subtypeConfers`) — engine-read, no special-casing.
tCounterConfers : List (Ability Base)
tCounterConfers = counterConfers p1p1

tSubtypeConfers : List (Ability Base)
tSubtypeConfers = subtypeConfers (saga)

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
tBecomeCopy : Modification CtxCreatureTarget
tBecomeCopy = BecomeCopyOf (Target 0)

tCopy : OneShotEffect Base
tCopy = Targeted [Target (^1) (IsKind Spell)] (Act (Copy (Target 0) []))

-- TOKEN copy of a permanent ([CR#707.2]) — the shapes the Rust `TokenSpec::Copy` emitter (`idris_emit.rs`)
-- lowers to. Bare (`Copy r []`), and "a copy, except it's a 4/4" ([CR#707.9d]): the copiable-value
-- alterations ride `Copy`'s OWN modification list as sibling higher-layer mods, never bundled into the copy.
tCopyTokenBare : OneShotEffect Base
tCopyTokenBare = Targeted [Target (^1) creature] (Act (Copy (Target 0) []))

tCopyToken44 : OneShotEffect Base
tCopyToken44 = Targeted [Target (^1) creature]
  (Act (Copy (Target 0) [Alter Power (Set (Literal 4)), Alter Toughness (Set (Literal 4))]))

-- becomes-a-copy as the StaticEffect the Rust `BecomesCopy` emitter lowers to ([CR#707.4], layer 1) — and,
-- with `This`, the same body an enters-as-a-copy `AsCopy` rider ([CR#707.5]) applies as its object arrives.
-- Bare = `Modify <who> (BecomeCopyOf <src>)`; with "except" mods = `ApplyAll [BecomeCopyOf <src>, <mods>]`,
-- each exception a SIBLING higher-layer mod ([CR#707.9]), never bundled into `BecomeCopyOf`.
tBecomesCopyStatic : StaticEffect CtxCreatureTarget
tBecomesCopyStatic = Modify This (BecomeCopyOf (Target 0))

tBecomesCopy44 : StaticEffect CtxCreatureTarget
tBecomesCopy44 = Modify This
  (ApplyAll [BecomeCopyOf (Target 0), Alter Power (Set (Literal 4)), Alter Toughness (Set (Literal 4))])

-- stack-object redirection. `ChangeTarget … This` is Spellskite (named new target);
-- `ChooseNewTargets` is Bolt Bend / Redirect (a player picks). Both ride the original targetspec.
tChangeTarget : OneShotEffect Base
tChangeTarget = Targeted [Target (^1) spellOrAbility] (Act (ChangeTarget (Target 0) This))

-- Bolt Bend ([CR#115.7d]) end-to-end: TARGET a "spell or ability with a SINGLE target" — the single-target
-- restriction is just `TargetCount Eq (^1)` (an existing predicate, no new machinery) conjoined with
-- `spellOrAbility` — then the caster picks its new target. Pins the restriction TO the redirect action.
tBoltBend : OneShotEffect Base
tBoltBend =
  Targeted [Target (^1) (And [spellOrAbility, TargetCount Eq (^1)])]
    (Act (ChooseNewTargets (Target 0)))

-- the structural holes: aggregate-stat cost (Crew), all-counters move (Ozolith), alternative base
-- cost (the base-SWAP type, distinct from CostChange). Solemnity is subsumed by Replaces+skip (a card).
tCrewCost : Cost Base
tCrewCost = TapTotal Power AtLeast (^3) creature

-- every-kind move (Ozolith / Fate Transfer): `MoveCounters AllKinds`
tMoveAllCounters : OneShotEffect Base
tMoveAllCounters = Targeted [Target (^1) creature] (Act (MoveCounters AllKinds This (Target 0)))

-- single-kind move (Power Conduit / Leech Bonder): the general primitive that was previously inexpressible
tMoveSomeCounters : OneShotEffect Base
tMoveSomeCounters = Targeted [Target (^1) creature] (Act (MoveCounters (Some p1p1 (^1)) This (Target 0)))

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

-- cost-payment DECISIONS, the collapsed `May (Act (Pay cost))` shape ([CR#118.12a]): MAY-pay
-- (optional, reward + downside) and MUST-pay (pay or be punished) are the same node, `ifDid`
-- present vs absent. The full `Cost` algebra rides both (here life / mana); the MayPay "if
-- they do" branch runs in the PAYMENT's caps ([CR#601.2f], `intro`'s `Pay` arm).
tMayPay : OneShotEffect Base
tMayPay = mayPayCostBy You (Do (LoseLife (Literal 2))) (Just (Act (Draw (^1)))) (Just (Act (LoseLife (^1))))

tMustPay : OneShotEffect Base
tMustPay = mayPayCostBy You (Mana [^2]) Nothing (Just (Act (Counter (Only (IsKind Spell)))))

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
tCounters = Sequentially [ Each (Existing (SelectAll creature)) (Act (PutCounters p1p1 (Literal 1) It))
                     , Each (Existing (SelectAll (Not (HasCounter p1p1)))) (Act (destroy It)) ]

-- anthem: a static `Each` over a controller-predicate filter, with layer mods
tAnthem : Ability Base
tAnthem = Static (Each (Existing (SelectAll (And [hasType Creature, ControlledBy you]))) (Modify It (ApplyAll [Alter Power (Up (^1)), Alter Toughness (Up (^1)), Alter Subtypes (Add (creatureType "Bear"))])))

-- a loyalty ability: an Activated ability whose cost removes Loyalty counters
tLoyalty : Ability Base
tLoyalty = Activated (Do (RemoveCounters loyaltyCounter (Literal 2) This)) (Act (Draw (^1)))

-- the value language: arithmetic, player attributes, counters-on, new stats, that-much
tValues : List (Count Base)
tValues =
  [ Plus (lifeTotal You) (handSize You)
  , Times (CountMatching creature) (Literal 2)
  , Half RoundUp (StatOf This Power)
  , CountersOn p1p1 This
  , ManaValueOf This                                     -- derived mana value ([CR#202.3]) — not a characteristic
  , Min (CountersOn p1p1 This) (CountersOn m1m1 This)   -- net counters after annihilation
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

-- the GLOBAL [CR#704.5] state-based actions AS DATA. Each `SbaRule` carries its domain (`scope`),
-- trigger (`when`), AND action (`thenDo`). Deathtouch [CR#704.5h] is ABSENT by design: it's intrinsic
-- to the `Deathtouch` keyword ([CR#702.2c] prospective lethality), not a keyword-independent global
-- rule, so the engine bakes it in.
tGlobalSbas : List SbaRule
tGlobalSbas =
  [ -- lethal damage [CR#704.5g]: a creature with toughness > 0 whose marked damage ≥ toughness is destroyed
    MkSbaRule (And [creature, InZone Battlefield])
              (And [ Compare (StatOf This Toughness) Greater (^0)
                   , Compare (Damage This) AtLeast (StatOf This Toughness) ])
              (Act (destroy This))
    -- 0-toughness [CR#704.5f]: toughness ≤ 0 → PUT INTO graveyard (a `Move`, NOT a `Destroy` — regeneration
    -- can't replace it; the Move-vs-Destroy choice is exactly what encodes that)
  , MkSbaRule (And [creature, InZone Battlefield])
              (Compare (StatOf This Toughness) AtMost (^0))
              (Act (Move This (ToZone Graveyard)))
    -- loyalty-0 [CR#704.5i]: a planeswalker with 0 loyalty counters → put into graveyard
  , MkSbaRule (And [hasType Planeswalker, InZone Battlefield])
              (Compare (CountersOn loyaltyCounter This) Eq (^0))
              (Act (Move This (ToZone Graveyard))) ]

-- new verbs (scry/token/search/copy) all typecheck. (Fight is now a RON macro
-- over `DealDamage`; its emitted form is covered by the canon idris-check, not a
-- hand-written helper here.)
tVerbs : List (OneShotEffect Base)
tVerbs =
  [ scry (Literal 2)
  , Act (CreateToken (Literal 2) (^: { name := Just "Soldier", types := [Creature], colors := [White], power := Just 1, toughness := Just 1 }))
  , With (SearchOne {from = [Library, Graveyard]} (HasName "Forest")) (Act (Move (That Card) (ToZone Hand)))  -- tutor across two zones; the found card is a whiffable Product, noun `Card`
  , Act (Copy (Only (IsKind Spell)) []) ]

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
-- opponent is a TARGET (player-predicate `opponent`), so `whose` is that announced player (`It`).
tSearchOther : OneShotEffect Base
tSearchOther = Targeted [Target (^1) opponent]
  (With (SearchOne {whose = Target 0} creature) (Act (Move (That Card) (ToZone Battlefield))))

-- a conditional static, and an activation-limited (loyalty-style) ability
tConditionalStatic : Ability Base
tConditionalStatic = Static (While (exists (ControlledBy opponent)) (Modify This (ApplyAll (modifyPT (Up (^1))))))

tLimitedAbility : Ability Base
tLimitedAbility =
  Activated (Do (RemoveCounters loyaltyCounter (Literal 1) This)) (Act (Draw (^1))) {window = AsSorcery, limits = [OncePerTurn]}

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
failing "Mismatch between: Type_ and Color"
  tBadSetColorValue : Modification Base
  tBadSetColorValue = Alter Colors (Set [Creature])

-- a "*/*" creature: printed power/toughness are a `Count` (CDA), not a bare Int
tCDA : Card
tCDA = Normal $ ^:
  { name := Just "Test CDA"
  , types := [Creature]
  , power := Just (CountMatching (hasType Land))
  , toughness := Just (Plus (CountMatching (hasType Land)) (Literal 1)) }

-- a target's KIND comes from its slot's filter — a PLAYER target reads as a
-- player, and the positional read carries that kind out of the announce list
-- (`tBadLifeOfCreatureTarget` pins the other direction).
tPlayerTarget : Count CtxPlayerTarget
tPlayerTarget = lifeTotal (Target 0)

-- "each player" is a player-`Selection`; `Each` binds a player `It` (EachPlayer dissolved)
tEachPlayerForEach : OneShotEffect Base
tEachPlayerForEach = Each (Existing eachPlayer) (Act (Draw {actor = It} (^1)))

-- MIXED-kind multi-target (Donate: "target player gains control of target
-- permanent"): each slot is named by its INDEX, and each read takes its kind
-- from the slot it names. Sorts do no disambiguating work here — they cannot,
-- because an anaphor never sees a target — so the shape needs no labels and no
-- R2 exemption, and it would read the same if both slots shared a sort
-- (`tTwoSameSortSlots`).
tMixedTargets : OneShotEffect Base
tMixedTargets =
  Targeted [Target (^1) Anyone, Target (^1) (And [permanent, ControlledBy you])]
    (Continuously Forever (Modify (Target 1) (GainControl (Target 0))))

-- TWO SAME-SORT slots (the fight family) — the shape the old model could not
-- express at all: a sorted anaphor was ambiguous across them (the R2 gate), so
-- it needed the labelling apparatus. Indices name them outright.
tTwoSameSortSlots : OneShotEffect Base
tTwoSameSortSlots =
  Targeted [Target (^1) creature, Target (^1) creature]
    (Sequentially [ Act (destroy (Target 0)), Act (Tap (Target 1)) ])

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

-- `Single` demotes a selection to its sole element (the dual of `Only`).
tSingle : Reference Base AnObject
tSingle = Single (SelectAll creature)

-- a PLURAL target slot (1–2) feeds divided damage; the announced group is the plural anaphor
-- `They` ([CR#601.2d]), and divided damage is the general `Distribute`: total `(^2)` split among
-- the group, each element dealt its `Allotment` (the Arc Lightning canon shape).
tPluralTarget : OneShotEffect Base
tPluralTarget = Targeted [Target (between (^1) (^2)) (Or [creature, Anyone])]
  (Distribute (^2) (Existing (Targets 0)) (Act (DealDamage Allotment It)))

-- the SAME `Distribute` over a different body: "distribute three +1/+1 counters among any number of target
-- creatures" (Hunting Triad) — `PutCounters` per element, each getting its `Allotment`. Carrier-typed.
tDistributeCounters : OneShotEffect Base
tDistributeCounters = Targeted [Target (between (^1) (^3)) creature]
  (Distribute (^3) (Existing (Targets 0)) (Act (PutCounters p1p1 Allotment It)))

-- NEGATIVE — each must be rejected, WITH the pinned message ------------------

-- THE ANAPHOR SURFACE (R1/R2/R3 + survival rules) -----------------------------

-- `It` with NO antecedent: no binder, empty stack.
failing "innermostBinder (Base .stack)"
  tBadItOutside : Reference Base AnObject
  tBadItOutside = It

-- a sorted anaphor with NO compatible antecedent.
failing "resolveStack (Just Card) One (Base .stack)"
  tBadThatNoAntecedent : Reference Base AnObject
  tBadThatNoAntecedent = That Card

-- a plural anaphor with NO Many antecedent in scope.
failing "resolveStack Nothing Many (Base .stack)"
  tBadTheyOutside : Selection Base AnObject
  tBadTheyOutside = They

-- THE STALE TARGET ([CR#603.7c]): a `Delayed` body is a LATER resolution and
-- has no announce list of its own (`unbindTargets` empties it), so the slot
-- index goes out of range and the read will not typecheck. This is the
-- obligation `Reference.Target` carries: without it a positional read would be
-- context-free and a stale `Target 0` here would sail through — the old model
-- caught this only as a side effect of targets sitting on the anaphor stack.
failing "unbindTargets"
  tBadDelayedTarget : OneShotEffect Base
  tBadDelayedTarget = Targeted [Target (^1) creature]
    (Sequentially [ Act (Move (Target 0) (ToZone Exile))
              , Delayed nextEndStep (Act (Move (Target 0) (ToZone Battlefield))) ])

-- a `Distinct` constraint naming an announce sibling that doesn't exist
-- ([CR#115.7e,601.2c]).
failing "distinctOk"
  tBadDistinctSibling : OneShotEffect Base
  tBadDistinctSibling = Targeted [Distinct [1] anyTarget] (Act (DealDamage (^1) (Target 0)))

-- AN ANAPHOR NEVER READS A TARGET. A slot pushes no antecedent, so in a body
-- whose only "candidate" is an announced target the sorted anaphor has nothing
-- to resolve against — it is Unbound, not a guess. `tTwoSameSortSlots` is the
-- same two-slot shape read positionally, which is fine: with targets off the
-- stack, two same-sort slots cannot make anything ambiguous, and the labelling
-- apparatus the old model needed for the fight family is unnecessary.
failing "resolveStack (Just (OfType Creature)) One"
  tBadThatReadsNoTarget : OneShotEffect Base
  tBadThatReadsNoTarget = Targeted [Target (^1) creature, Target (^1) creature]
    (Act (destroy (That (OfType Creature))))

-- ...and the wildcard cannot either: a lone announced slot is no antecedent.
-- This is Lightning Bolt's OLD spelling (`DealDamage(This, 3, It)`) — now a
-- load error rather than a read that happened to land on the target.
failing "resolveStack Nothing One"
  tBadItReadsNoTarget : OneShotEffect Base
  tBadItReadsNoTarget = Targeted [Target (^1) creature] (Act (destroy It))

-- ...nor the plural one: a plural slot is read as `Targets n`, never `They`.
failing "resolveStack Nothing Many"
  tBadTheyReadsNoTarget : OneShotEffect Base
  tBadTheyReadsNoTarget = Targeted [Target (between (^1) (^3)) creature]
    (Each (Existing They) (Act (Tap It)))

-- an out-of-range slot index — the announce list has one entry, so `Target 1`
-- names nothing ([CR#115.3]).
failing "resolveTarget One 1"
  tBadTargetOutOfRange : OneShotEffect Base
  tBadTargetOutOfRange = Targeted [Target (^1) creature] (Act (destroy (Target 1)))

-- a MANY slot read singularly would silently take one of several — the
-- positional twin of the cardinality split (`Targets n` is the group read).
failing "resolveTarget One 0"
  tBadTargetFromMany : OneShotEffect Base
  tBadTargetFromMany = Targeted [Target (between (^1) (^3)) creature]
    (Act (destroy (Target 0)))

-- ...and a ONE slot has no group read.
failing "resolveTarget Many 0"
  tBadTargetsFromOne : OneShotEffect Base
  tBadTargetsFromOne = Targeted [Target (^1) creature]
    (Each (Existing (Targets 0)) (Act (Tap It)))

-- a SINGULAR anaphor cannot read a MANY antecedent (the cardinality split;
-- read the group as `They`/`Them`).
failing "resolveStack (Just Token) One"
  tBadOneFromMany : OneShotEffect Base
  tBadOneFromMany = Sequentially [ Act (CreateToken (^2) (^: { types := [Creature] }))
                             , Act (destroy (That Token)) ]

-- ...nor a plural anaphor a ONE antecedent (read it as `It`/`That w`) — over a
-- BINDER's antecedent, the only kind an anaphor can see now.
failing "innermostFrame ((bindThat (binderAnte (ChooseOne creature)) Base) .stack)"
  tBadTheyFromOne : OneShotEffect Base
  tBadTheyFromOne = With (ChooseOne creature) (Each (Existing They) (Act (Tap It)))

-- tokens aren't cards ([CR#108.2b]): "that card" never reaches a token
-- antecedent — the compat table's most load-bearing closed pair.
failing "resolveStack (Just Card) One ((intro"
  tBadTokenIsNotCard : OneShotEffect Base
  tBadTokenIsNotCard = Sequentially [ Act (CreateToken (^1) (^: { types := [Creature] }))
                                , Act (Move (That Card) (ToZone Exile)) ]

-- `Allotment` outside any `Distribute` body.
failing "hasAllot (Base .stack) = True"
  tBadAllotmentOutside : Count Base
  tBadAllotmentOutside = Allotment

-- `ThatMany` with no amount antecedent in scope.
failing "candidates (Just Amount) One (Base .stack)"
  tBadThatManyNoAmount : Count Base
  tBadThatManyNoAmount = ThatMany

-- TWO amount antecedents make `ThatMany` a guess — the R2 gate ranges over
-- value anaphora too ([CR#608.2i]).
failing "candidates (Just Amount) One ((intro"
  tBadAmbiguousAmount : OneShotEffect Base
  tBadAmbiguousAmount = Sequentially [ Act (Draw (^2)), Act (GainLife (^3)), Act (LoseLife ThatMany) ]

-- an ORDERED zone is no bare destination ([CR#401.4]) — position it with
-- `ToLibrary (FromTop …)`.
failing "implementation for Void"
  tBadDestinationLibrary : Action Base
  tBadDestinationLibrary = Move This (ToZone Library)

-- THE CARRIED-OVER V1 GATES ---------------------------------------------------

-- a target slot can't target ZERO — `NonZeroQ` rejects a statically-zero upper bound
failing "NonZeroQ"
  tBadZeroTarget : TargetSpec Base AnObject
  tBadZeroTarget = Target (^0) creature

-- a card with NO card types is rejected — `CharacteristicsOk` (the one lenient well-formedness floor)
failing "implementation for NonEmpty"
  tBadTypeless : Card
  tBadTypeless = Normal $ ^: { name := Just "Typeless" }

-- a two-faced card's BACK face is well-formedness-checked too, not just the front — a typeless back fails
failing "implementation for NonEmpty"
  tBadTwoFacedBack : Card
  tBadTwoFacedBack = TwoFaced Split (^: { types := [Instant] }) (^: { name := Just "Back" })

-- the GOOD direction of the scope index: a player counter/designation predicate at a PLAYER index
-- typechecks (`counterKindScope poison` / `designationKindScope monarch` reduce to `APlayer`).
tPoisonPredicate : Predicate Base APlayer
tPoisonPredicate = HasCounter poison

tMonarchPredicate : Predicate Base APlayer
tMonarchPredicate = HasDesignation monarch

-- a PLAYER-carried counter can't go on an object — `counterKindScope poison` REDUCES to `APlayer`, so
-- `This` (an `AnObject` reference) is rejected with no runtime check. That the open value's scope
-- projection still reduces at the type index is the load-bearing property this test pins.
failing "APlayer"
  tBadPoisonOnObject : Action Base
  tBadPoisonOnObject = PutCounters poison (^1) This

-- granting a PLAYER designation to an object is a type error — `designationKindScope monarch` reduces to `APlayer`
failing "APlayer"
  tBadDesignationScope : Action Base
  tBadDesignationScope = GrantDesignation monarch This

-- the `Predicate`-indexed constructors discriminate too: `HasCounter poison` is a PLAYER predicate
-- (`counterKindScope poison` reduces to `APlayer`), so forcing it at an object index is a type error.
failing "APlayer"
  tBadPoisonPredicate : Predicate Base AnObject
  tBadPoisonPredicate = HasCounter poison

-- likewise `HasDesignation monarch` is a PLAYER predicate; forcing it at an object index is rejected.
failing "APlayer"
  tBadMonarchPredicate : Predicate Base AnObject
  tBadMonarchPredicate = HasDesignation monarch

-- replacing the AMOUNT of an amountless event is rejected — a Cast has no numeric payload
failing "False = True"
  tBadReplaceAmountless : StaticEffect Base
  tBadReplaceAmountless = ReplaceAmount (MkEventQuery [Begins Cast] []) (^0)

-- folding the amount of an amountless event is rejected likewise
failing "False = True"
  tBadEventAggAmountless : Count Base
  tBadEventAggAmountless = EventAgg SumOf (MkEventQuery [Begins Cast] [])

-- "becomes summoning-sick" isn't a transition event — `IsBecomesState SummoningSick = Void`
failing "implementation for Void"
  tBadBecomesSummoningSick : EventKind
  tBadBecomesSummoningSick = Becomes SummoningSick

-- projecting a NON-object `Countable` is rejected — only `Objects` is `Projectable`, so `Project (Events …)`
-- has no `Projectable (Events …)` proof (you cannot bind `It` over an atomic event).
failing "Projectable (Events"
  tBadProjectEvents : Projection Base AnObject
  tBadProjectEvents = Project (Events (MkEventQuery [DealDamage Nothing] [])) (Literal 0)

-- `CountDistinct` is gated by `readableOn`: an object-only characteristic over a non-object source is
-- rejected — "distinct powers of the mana you spent" is nonsense (`readableOn Power ManaSpent = Void`).
failing "implementation for Void"
  tBadDistinctStatOfMana : Count Base
  tBadDistinctStatOfMana = CountDistinct Power ManaSpent

-- ...and a non-colour characteristic over events is rejected too (`Name` reads nothing off an event).
failing "implementation for Void"
  tBadDistinctNameOfEvents : Count Base
  tBadDistinctNameOfEvents = CountDistinct Name (Events (MkEventQuery [DealDamage Nothing] []))

-- `Pick` is gated to the EXTREMAL ops by `IsExtremal`: argmax-by-SUM is meaningless (`IsExtremal SumOf` is uninhabited).
failing "IsExtremal SumOf"
  tBadPickNonExtremal : Selection Base AnObject
  tBadPickNonExtremal = Pick SumOf (eachOf creature (StatOf It Power))

-- an empty symbol disjunction ("devotion to no colours") is rejected — the restored `NonEmpty` guard.
failing "NonEmpty []"
  tBadEmptySymbolOr : Countable Base
  tBadEmptySymbolOr = ManaSymbols This (Or [])

-- a `Distribute` share (`Allotment`) can't leak into a `Projection` accessor — `eachOf`/`Project` rebind `It`
-- via `bindIt`, which clears the `Allot` antecedent, so `Allotment` has no proof there (it was indexed to
-- a DIFFERENT loop element).
failing "bindIt (loopOf creature)"
  tBadAllotmentInProjection : Projection Base AnObject
  tBadAllotmentInProjection = eachOf creature Allotment

-- THE INVALID-REFERENCE GATE: an event anaphor is valid only where the event SUPPLIES it (`eventQueryCaps`).
-- `EventObject` ("that card") in a step-begin body — a `BeginStep` event has no object.
failing ".hasObject = True"
  tBadEventObjectNoObject : Ability Base
  tBadEventObjectNoObject =
    Triggered (MkEventQuery [BeginStep (BeginningPhase UpkeepStep)] []) (Act (Move EventObject (ToZone Exile)))

-- `EventAmount` (the amount) in a `Begins Cast` body — a cast carries no amount.
failing ".hasAmount = True"
  tBadThatMuchNoAmount : StaticEffect Base
  tBadThatMuchNoAmount = Replaces (MkEventQuery [Begins Cast] []) (Act (DealDamage EventAmount This))

-- `EventActor` ("that player") in a Destroy body — a destruction has no actor.
failing ".hasActor = True"
  tBadEventActorNoActor : Ability Base
  tBadEventActorNoActor = Triggered (MkEventQuery [Destroy] []) (Conclude (WinGame EventActor))

-- `EventPatient` where the event fixes no patient kind: a draw acts on no
-- patient ([CR#120.3,608.2k]), so the read has no sound binder.
failing ".patientKind = Just"
  tBadEventPatientNoKind : Ability Base
  tBadEventPatientNoKind = Triggered (MkEventQuery [Draw] []) (Act (destroy EventPatient))

-- `DefendingPlayer` where no combat onset supplies one: an enters trigger has
-- no defender ([CR#506.2,508.5]).
failing ".hasDefender = True"
  tBadDefenderNoCombat : Ability Base
  tBadDefenderNoCombat = Triggered thisEnters (Act (LoseLife {actor = DefendingPlayer} (^1)))

-- ...and the anaphora DO work where the event supplies them: `EventActor` in a `Begins Cast` body (the caster).
tEventActorValid : Ability Base
tEventActorValid = Triggered (MkEventQuery [Begins Cast] []) (Conclude (WinGame EventActor))

-- MULTI-KIND SOUNDNESS (the EventQuery restructure): a multi-kind query's caps are the INTERSECTION —
-- the body gets only anaphora EVERY listed kind supplies. `EventActor` under `[Begins Cast, Destroy]` is
-- rejected (a Destroy event has no actor), so the old union-cap leak (A6) is gone.
failing "Begins Cast, Destroy"
  tBadEventActorMultiKind : Ability Base
  tBadEventActorMultiKind = Triggered (MkEventQuery [Begins Cast, Destroy] []) (Conclude (WinGame EventActor))

-- ...but when EVERY listed kind supplies the anaphor it's fine: "attacks or blocks" both supply an
-- object, so `EventObject` is valid (Smuggler's Copter's single trigger over two kinds).
tEventObjectMultiKind : Ability Base
tEventObjectMultiKind =
  Triggered (MkEventQuery [Begins Attack, Begins Block] []) (Act (Move EventObject (ToZone Exile)))

-- "whenever a creature enters, draw THAT MANY cards" — meaningless: a creature entering (`ZoneChanged`)
-- carries no amount, so `EventAmount` has no referent. The caps gate rejects it.
failing ".hasAmount = True"
  tBadDrawThatManyOnEnter : Ability Base
  tBadDrawThatManyOnEnter =
    Triggered (MkEventQuery [ZoneChanged Nothing (Just Battlefield)] [Agent creature])
      (Act (Draw EventAmount))

-- BOUNDED-NUMERIC gates. An inverted range ("between 5 and 2") — `OrderedRange` rejects `lo > hi`.
failing "OrderedRange"
  tBadInvertedRange : Bindable Base Many AnObject
  tBadInvertedRange = Choose (between (^5) (^2)) creature

-- `MainPhase` is a closed 2-value enum now, not a `Nat` — `MainPhase 99` doesn't typecheck.
failing "Num MainPhaseKind"
  tBadMainPhase99 : PhaseStep
  tBadMainPhase99 = MainPhase 99

-- a modal "choose 5" of a single mode — `modalCountOk` bounds the literal count by the mode count.
failing "modalCountOk"
  tBadModalOverCount : OneShotEffect Base
  tBadModalOverCount = Modal (MkChooseSpec (^5)) [ MkMode (Act (Draw (^1))) ]

-- a modal with NO modes — the `Vect (S n)` mode vector has no empty form.
failing "Mismatch between: 0 and S"
  tBadModalEmptyModes : OneShotEffect Base
  tBadModalEmptyModes = Modal (MkChooseSpec (^1)) []

-- a 0-way mode domain — `ModeDomainOk (AMode 0)` is `LT 0 0` = uninhabited.
failing "LTE 1 0"
  tBadModeDomainZero : Ability Base
  tBadModeDomainZero = AsEnters (AMode 0) []

-- (No `tBad…` for "produce {X}/{W/P}" or "cost with any-color": printed `ManaSymbol` and `ProducedMana`
-- are SEPARATE types, so those are unrepresentable by construction — a `failing` block there would only
-- test the type distinction, not the model.)

-- a 0-size block is rejected — a declared block has ≥1 blocker (`NonZeroQ` on `BlockedBy`'s size)
failing "NonZeroQ"
  tBadZeroBlock : StaticEffect Base
  tBadZeroBlock = cant (BlockedBy (SameAs This) (^0))

-- `OfChosen` with no as-enters choice in scope — `IsCharDomain Nothing = Void` denies the anaphor
failing "IsCharDomain (Base .chosenKind)"
  tBadOfChosenNoChoice : Predicate Base AnObject
  tBadOfChosenNoChoice = OfChosen

-- `ChosenIs` past the mode count is rejected — `LT 2 2` is uninhabited (a 2-mode card, index 2)
failing "LTE 3 2"
  tBadChosenMode : Condition (bindChosen (AMode 2) Base)
  tBadChosenMode = ChosenIs 2

-- `OfChosen` on a MODE choice is rejected — a mode isn't a characteristic (`IsCharDomain (AMode _) = Void`)
failing "IsCharDomain ((bindChosen (AMode 2)"
  tBadOfChosenMode : Predicate (bindChosen (AMode 2) Base) AnObject
  tBadOfChosenMode = OfChosen

-- `OfChosen` on an as-enters ENTITY choice is rejected — an object is identity, not a characteristic, and
-- it binds `chosenRefKind` (NOT `chosenKind`), so `OfChosen`'s `IsCharDomain (chosenKind b)` finds
-- `Nothing` → `Void`. Read a chosen object with `ChosenObject`/`SameAs`, never `OfChosen`.
failing "bindChosenRef AnObject"
  tBadOfChosenObject : Predicate (bindChosenRef AnObject Base) AnObject
  tBadOfChosenObject = OfChosen

-- a subtype whose category admits none of the card's types [CR#205.3d]
failing "categoryTypes (subtypeCategory"
  tBadSubtype : Card
  tBadSubtype = Normal $ ^:
    { name := Just "Bad", types := [Creature], subtypes := [aura] }

-- the split makes the old `CountOf (During …)` category error ILL-TYPED: `CountOf`
-- takes a `Predicate`, but `During` (a game-state test) is a `Condition`.
failing "and Countable"
  tBadCountOfCondition : Count Base
  tBadCountOfCondition = CountOf (During (MainPhase PreCombat))

-- `EventObject` ("that card") is rejected outside a trigger/replacement/delayed body
-- (the caps are `NoCaps` there) — the review fix that closed the ungated-anaphora hole.
failing "(Base .eventCaps) .hasObject"
  tBadEventObjectOutside : Reference Base AnObject
  tBadEventObjectOutside = EventObject

-- one Reference, but the kind still bites: a player has no power/toughness
failing "Mismatch between: APlayer and AnObject"
  tBadStatOfPlayer : Count Base
  tBadStatOfPlayer = StatOf You Power       -- You : APlayer, StatOf wants AnObject

-- ...and an object has no life total
failing "Mismatch between: AnObject and APlayer"
  tBadLifeOfObject : Count Base
  tBadLifeOfObject = lifeTotal This         -- This : AnObject, lifeTotal wants APlayer

-- kind strictness through the stack: a CREATURE target's anaphor can't be
-- read as a player — the resolved kind rides the antecedent.
failing "CtxCreatureTarget .stack"
  tBadLifeOfCreatureTarget : Count CtxCreatureTarget
  tBadLifeOfCreatureTarget = lifeTotal It
