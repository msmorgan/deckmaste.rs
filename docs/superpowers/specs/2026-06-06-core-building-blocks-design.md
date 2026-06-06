# Core building blocks — design

2026-06-06. Companion to `docs/rules-taxonomy.md` (the survey this design
consolidates). Type sketches are shapes, not final Rust signatures; CR
citations are to `data/rules/cr.json`.

## Scope and principles

- **Vintage-legal mechanics only.** No ante zone, schemes, planes,
  vanguard, conspiracies, attractions. Stragglers survive through open
  vocabularies at no structural cost (the two vintage-legal `{TK}` cards —
  Ticket Turbotubes, Blorbian Buddy — need one declared counter kind).
- **CR-mirroring kinds** (chosen over minimal-core-plus-sugar and
  engine-first IR): each taxonomy kind is a Rust type factored the way the
  CR factors the concept. The reader rejects what the CR cannot express;
  the engine later interprets these types directly.
- **Macros abbreviate, the engine executes.** Macro expansion is pure,
  non-recursive substitution — no conditionals, no repetition, no computed
  names; cycles and depth are load errors. Anything that looks like control
  flow (for-each, if-you-do) is *data in the Effect AST*, interpreted by
  the engine. This invariant is permanent.
- **Tight grammar.** An enum position never carries a bare list with
  implicit semantics. Arrays appear only as named struct fields
  (`targets:`, `effects:`, `modes:`, `cost:`, `changes:`) or inside
  explicitly named constructors (`Sequence`, `AllOf`, `OneOf`, `These`).
  Verb object slots take one `Selection`.
- **Filters are context-free-correct.** Canonical macro bodies state the
  whole predicate (Flying's blocker set includes `InZone(Battlefield)` and
  `Type(Creature)`) even where engine context would make parts redundant.
- **Expansion remembers the top-level invocation.** Every macroable kind
  carries an `Expanded` variant (§9). `MacroKind` enumerates exactly the
  types that have one.

## 1. The selection complex

Card definitions are templates — they never contain runtime ObjectIds.
**Players are objects**: the engine gives players ObjectIds, so one Filter
ranges over both, and the old splits (Target::Player vs PermanentOfType,
ObjectSelector vs PlayerSelector, counters-on-players vs on-permanents,
designation scope) collapse.

### Filter — a predicate over objects

Compartmentalized for later use, flat in RON (untagged wrapper variants,
the `ManaSymbol::Simple` precedent):

```
Filter ::= Characteristic(CharacteristicFilter)   // Type, Subtype, Supertype, color,
                                                  // name, Stat(power|toughness|mv|…, Cmp), HasAbility
         | State(StateFilter)                     // Status, InZone, HasCounter, Designated(ident),
                                                  // RelatedBy(ident, Filter)
         | Relation(RelationFilter)               // Controller(Filter), Owner(Filter), AttachedTo(Filter),
                                                  // Attachment(Filter), OpponentOf(Filter), Player
         | Is(Reference)
         | AllOf([Filter]) | OneOf([Filter]) | Not(Filter)
```

Rules:

- **Primitives only.** Named conveniences are prelude macros carrying their
  CR citations: `YouControl` = `Controller(Is(You))`, `IsOpponent` =
  `AllOf([Player, OpponentOf(Is(You))])`, `IsNot(r)` = `Not(Is(r))`,
  `Historic` (CR 700.6), `AnyTarget` (CR 115.4), `PairedWith`,
  `ExiledWith`, every protection quality, every evasion blocker set.
- Within a Filter, quantification is uniformly implicit-existential
  ("enchanted by an Aura", `Controller(IsOpponent)`).
- Has-counter is State, not Characteristic (counters are not
  characteristics, CR 122.1).
- Derived designations (§8) make `Designated(Modified)` work with no
  special casing.

### Selection — Filter lifted into a choice context

```
Selection ::= Target(Filter) | Targets(Quantity, Filter) | UpToTargets(Quantity, Filter)
            | AnyNumberTargets(Filter) | Each(Filter) | All(Filter)
            | Choose(Filter) | ChooseN(Quantity, Filter) | Random(Quantity, Filter)
            | Superlative(measure, Filter)                 // bolster's "least toughness"
            | That(Reference) | These([Reference])         // references lift into Selection
```

Selection is **not** a Filter variant, deliberately:

1. *Closure*: filters compose under AllOf/OneOf/Not; quantifiers don't.
   `Not(Target(…))` must be unwritable.
2. *Filters answer, Selections oblige*: a Filter is a question askable of
   any object at any instant (protection qualities, evasion sets, event
   participants, continuous-effect scopes take bare Filter). A Selection
   has a chooser, a timing (announce vs resolution), a count, legality
   recheck — and it **binds References** (`Target(0)` exists because a
   target Selection bound it). Filters never bind.
3. *Position safety*: bare-Filter positions must not admit "protection
   from target creature". The CR makes the same cut: in "target
   [something]", *target* is the mechanic, "[something]" the filter
   (115.1a).

### Reference — the bound variables

```
Reference ::= This | You | Target(index)
            | That | ThatPlayer | Bound(role)        // trigger/event participants
            | Linked(key)                            // CR 607: exiled-with, the chosen value, cost paid
            | ControllerOf(Reference) | OwnerOf(Reference)
            | EnchantedObject | EquippedCreature | AttachedToOf(Reference) | …
```

Every binder (target Selections, Event patterns, `Choose` instructions,
linked actions) exports references; effect bodies are written against
them. Magnitude anaphora ("that much") lives in Quantity, not here —
references name objects, quantities name amounts.

## 2. Ability anatomy

```
Ability ::= Spell(Resolvable)
          | Activated  { cost: Cost, body: Resolvable, restrictions: [ActivationRestriction] }
          | Triggered  (TriggeredAbility)
          | Static     { condition: Option<Condition>, effects: [StaticEffect] }
          | Expanded   (Expansion<Ability>)            // §9 — absorbs the old Keyword variant

TriggeredAbility = { event: Event, if: Option<Condition>, limits: [TriggerLimit], body: Resolvable }

Resolvable ::= Plain { targets: [Selection], effect: Effect }
             | Modal { choose: ChooseSpec, modes: [Mode] }

Mode = { targets: [Selection], effect: Effect, cost: Option<Cost> }      // per-mode cost: CR 700.2h
```

- **Modal is hoisted into `Resolvable`**, the payload shared by Spell,
  Activated, and Triggered — because modes are chosen before targets
  (601.2b/603.3c), each mode owns its target list (700.2c, 115.8), and
  retargeting can't change the mode (115.7). Modal spells, modal triggers,
  and charms come for free; modes don't nest. `ChooseSpec` carries count
  (one / up to N / "one or both" / any number), repeat-allowed (700.2d),
  and chooser (700.2e).
- **Targets are an explicit announce list** referenced by index — today's
  `targets: [...], effect: DealDamage(Target(0), 3)` shape, with
  `Selection` entries.
- **Mana-ability and loyalty-ability classification is derived, never
  stored**: CR 605 defines mana abilities by predicate (no targets ∧ could
  add mana ∧ not loyalty), CR 606 by cost shape (loyalty-counter
  components). Storing a flag would create a lie to validate.
- **CDA is the one explicit flag** (on Static); the validator checks
  604.3's criteria.
- **`TriggeredAbility` is a named struct because it recurs**: delayed
  (603.7) and reflexive (603.12) triggers are the same value created
  inside an Effect.
- Where-it-functions (113.6 zones: hand, graveyard, stack, …) defaults by
  kind with an override slot; non-battlefield cases mostly come from
  keyword macros that know their zone.

## 3. StaticEffect and the continuous-effect unification

The shared currency between "anthem" and "+3/+3 until end of turn" is
`StaticEffect`; the difference is who wraps it.

```
StaticEffect ::= Modify       { of: Scope, changes: [Modification] }
              | Restriction   (Restriction)        // "can't": evasion, can't attack/block/be targeted, can't cast
              | Requirement   (Requirement)        // "must": goaded, must attack/block
              | Permission    (Permission)         // may cast from graveyard, flash-likes, "as though" (611.3d)
              | CostModifier  { of: Filter, change: CostChange }   // CR 118.7; Increase(Cost) | Reduce(…)
              | Replacement   (Replacement)        // §5
              | Prevention    (Prevention)         // §5

Scope ::= That(Reference) | These([Reference]) | Matching(Filter)

Static ability:        { condition, effects: [StaticEffect] }   // duration implicit: while it functions (611.3)
One-shot-created:      Continuously { effect: StaticEffect, duration: Duration }   // 611.2
```

- **Anthem**: `Modify(of: Matching(AllOf([Type(Creature), YouControl])), changes: [AddPower(1), AddToughness(1)])`
  inside a Static ability.
- **Giant Growth**: `Continuously(effect: Modify(of: That(Target(0)), changes: [AddPower(3), AddToughness(3)]), duration: UntilEndOfTurn)`.
- Identical Modify values; the two visible differences are scope shape and
  duration; the third — lock-in — is provenance the engine applies:
  resolution-created characteristic-changers fix their set at creation
  even when filter-shaped (611.2c); statics float (611.3). Timestamps
  likewise.

### Modification — flat primitive ops

```
Modification ::= SetPower(Q) | AddPower(Q) | SetToughness(Q) | AddToughness(Q) | SwitchPowerToughness
               | SetColors(…) | AddColors(…)
               | SetCardTypes(…) | AddCardTypes(…) | SetSubtypes(…) | AddSubtypes(…)
               | SetSupertypes(…) | AddSupertypes(…)
               | GainAbility(Ability) | LoseAbility(…) | LoseAllAbilities | CantHaveAbility(…)
               | SetController(…) | SetText(…) | SetBaseLoyalty(Q) | SetBaseDefense(Q)
               | BecomeBasicLandType([LandType])       // CR 305.7, quarantined — see below
```

- **Layers are derived, never written**: `Add*` stats → 7c, `Set*` stats →
  7b (7a when the Static is CDA-flagged), `Switch` → 7d, types → 4, colors
  → 5, abilities → 6, controller → 2, text → 3. `changes` is a list
  because one effect can span layers applied to the same set (613.6).
- **`SetSubtypes` carries no class slot** — the affected subtype class is
  derivable from the values (Zombie can only be a creature type), matching
  how the CR scopes subtype-setting implicitly (205.1b).
- **`BecomeBasicLandType` is a dedicated intrinsic** for the 305.7 bundle
  (replace land types ∧ lose printed abilities ∧ gain the mana ability) —
  Blood Moon's iconic edge case gets exactly one name and nothing else
  grows a slot for it. Not reachable from the plain `Set*` ops, hence
  intrinsic rather than macro.

### Restriction polarity

The CR splits combat/targeting legality into *restrictions* ("can't" —
absolute) and *requirements* ("must" — maximized but violable), evaluated
asymmetrically (509.1b–c). **Evasion abilities are Restrictions**:

```ron
// CR 702.9 — Flying, in the prelude
CantBeBlockedExceptBy(AllOf([
    InZone(Battlefield),
    Type(Creature),
    OneOf([HasAbility(Flying), HasAbility(Reach)]),
]))
```

Menace adds a count slot (`min: 2`), skulk a stat comparison against
`This`. Goaded's "attacks each combat if able" is a Requirement.

### Duration

`UntilEndOfTurn | UntilYourNextTurn | UntilEndOfCombat | While(Condition)`
(with the never-started rule, 611.2b) `| UntilEvent(Event)` (engine pairs
the undo one-shot, 610.3) `| EndOfGame`.

## 4. Effect, Action, Cost

### Effect — an enum, not a Vec

```
Effect ::= Act(Action-or-verb-call)               // single instruction stands bare
         | Sequence([Effect])                     // explicit "then" (608.2c)
         | Continuously { effect: StaticEffect, duration: Duration }
         | May    { do: Effect, if_did: Option<Effect>, if_not: Option<Effect> }
         | If     { condition: Condition, then: Effect, else: Option<Effect> }
         | Unless { do: Effect, unless: …, who: … }          // CR 118.12a
         | ForEach{ over: Filter, do: Effect }               // binds the iterated object
         | Choose { … }                                       // at-resolution binder (608.2d)
         | Delayed(TriggeredAbility) | Reflexive(TriggeredAbility)
         | Expanded(Expansion<Effect>)
```

Single-instruction bodies need no wrapper: `effect: DealDamage(Target(0), 3)`
stays exactly as Lightning Bolt writes it. The structural forms are the
corpus's connective tissue ("you may" 4,453 lines, "if you do" 1,029,
"unless" 749, "for each" 1,609) — data the engine interprets; the macro
layer never sees control flow.

### Action — intrinsics only; keyword actions are declared vocabulary

The Rust `Action` enum holds only engine intrinsics — verbs whose
semantics cannot be data: zone-move, set-status, put/remove counters, deal
damage, gain/lose life, draw-primitive, add-mana, grant-designation,
choice/vote forms. **Every CR 701 keyword action is a declaration** in a
plugin (`plugins/builtin/definitions/keyword_actions/` already exists):
name, parameter signature, definition body in terms of other verbs —
loaded like macros (unknown verb / wrong arity = load error), written as
`Sacrifice(…)` in card RON, carried name-preserved via `Expanded` (§9).

- **Verb identity must survive**: "whenever you *sacrifice*" and "if you
  would *draw*" match on names (sacrifice ≠ destroy ≠ generic
  put-into-graveyard, 701.21a; draw replacement, 614.11). Executing a
  declared verb logs both the named event and its expansion's primitive
  events, so sacrifice-triggers and dies-triggers both fire — CR behavior.
- **Verb object slots are unary Selections**: `Sacrifice(Choose(Type(Creature)))`,
  `Sacrifice(This)`, `Destroy(Each(AllOf([Type(Creature), Tapped])))`,
  `DealDamage(Target(0), 3)`.
- Compound verbs (investigate, amass, explore, airbend, monstrosity) are
  declarations over primitives — the taxonomy doc's primitive-vs-compound
  split made literal. New and custom sets mint verbs by dropping a file.

### Cost — parallel components, not Action reuse

```
Cost          = [CostComponent]
CostComponent ::= Mana(ManaCost) | Tap | Untap        // the {T}/{Q} symbols (107.5)
                | Do(verb-call)                        // cost-eligible verbs only, validated at load
```

Cost-position is constrained (payer performs, nothing targets, 601.2b–c;
paid in full or not at all, 118.3) — a separate type makes "Destroy all
creatures" unwritable as a cost. **Loyalty costs are counter ops**:
`[+1]`/`[−3]` = `Do(PutCounters(This, Loyalty, 1))` /
`Do(RemoveCounters(This, Loyalty, 3))` (CR 606.4 says exactly this), with
RON sugar via Cost macros; loyalty-ability classification checks for them.
Energy likewise: `RemoveCounters(You, Energy, n)`.

Running examples:

```ron
// Lightning Bolt
Spell(targets: [Target(AnyTarget)], effect: DealDamage(That(Target(0)), 3))

// Treasure
Activated(cost: [Tap, SacrificeThis], effect: AddMana(1, AnyColor))
// SacrificeThis: prelude Cost macro for Do(Sacrifice(This))
```

The shelved tokens migration unblocks at exactly this point.

## 5. Event, Condition, Replacement/Prevention

### Event — declared names over intrinsic patterns

Events are to verbs what derived designations are to stored ones. The core
has only pattern forms:

```
Event ::= Performed { verb: VerbPattern, by: Filter, on: Filter }   // matches the action log
        | ZoneMove  { what: Filter, from: ZonePattern, to: ZonePattern }
        | BeginningOf(StepOrPhase, WhoseTurn)
        | StateBecomes { of: Filter, state: … }       // transitions only (603.2e)
        | OneOfEvents([Event]) | Expanded(Expansion<Event>) | …
```

Prelude/wizards declare the names: `Dies` = `ZoneMove(from: Battlefield,
to: Graveyard)` (700.4 — "dies" is literally a derived event name in the
rules), `Enters`, `Landfall`, `YouCastNoncreature`, …. Trigger matching is
structural on patterns, so event macros can expand freely; verb-name
matching (`Performed(Sacrifice, …)`) distinguishes sacrifice from
destruction. Look-back-in-time semantics (603.10) hang off the pattern
form. Each form binds fixed roles (`That`, `ThatPlayer`) for the body.
Intervening-if and once-each-turn limits live on `TriggeredAbility` (§2).

### Condition — a small combinator set

```
Condition ::= Compare(Quantity, Cmp, Quantity) | Exists(Filter) | Is(Reference, Filter)
            | Happened { event: Event, within: ThisTurn | … }       // turn-history memory
            | AllOf([Condition]) | OneOf([Condition]) | Not(Condition)
            | Expanded(Expansion<Condition>)
```

`Happened` covers morbid/raid/"was kicked" (cast-memory keys through
Linked references). **Ability words are declared Condition macros** —
`Threshold`, `Delirium`, `Morbid` — with their CR citations, matching the
existing `plugins/wizards/ability_words/` stubs.

### Replacement / Prevention — the CR's closed template list

```
Replacement ::= Instead  { would: Event, instead: Effect }    // 614.1a
              | Skip     { what }                              // 614.1b
              | AsEnters { choices/state }                     // 614.1c/614.12
              | Redirect { … }                                 // 614.9
Prevention  ::= PreventNext { n: Quantity, from: Filter, to: Filter, duration }   // 615.7
              | PreventNextInstance { … }                      // 615.8
              | PreventAll { from: Filter, to: Filter, duration }                  // 615.10
```

`EntersTapped`, `EntersWith(counters)`, `Regenerate` are prelude macros
over these forms.

## 6. Vocabulary leaves

- **Quantity** — `Literal(n) | X | CountOf(Filter) | StatOf(Reference, stat) | ThatMuch | Sum(…)`,
  with `where X is` definitions attaching where X is introduced.
- **Zone** — library, hand, battlefield, graveyard, stack, exile, command.
  No ante (out of scope). Positions (`Top(n)`, `Bottom`), owner, and
  exile-pile facing live on a `ZoneRef` wrapper.
- **ManaSpec** — symbols, `AnyColor`, `AnyOneColor`, `AnyType`, plus the
  spend-restriction rider.
- **TokenSpec** — CardFace-shaped characteristics or
  `CopyOf(Reference, except: […])`; predefined tokens stay in
  `plugins/builtin/tokens/`.
- **CounterKind** — open `Ident` vocabulary, declared with optional payload
  (`GainAbility(Flying)` for keyword counters; replacement payloads for
  stun/shield). Ticket is just another declaration.
- **Designation** — open `Ident` vocabulary per the taxonomy doc §8:
  declaration carries definition (stored | derived-from-predicate), scope
  (Object | Player | Game), shape (Flag | Enum | Number | Relation),
  uniqueness, default persistence (object lifetime is the free default via
  400.7), payload abilities (suspected's menace). Granting a derived
  designation is a load error.

## 7. Module layout

One kind per module in `deckmaste_core`, serde names doubling as
`MacroKind` position names (extend the `position_names_track_the_core_types`
test pattern to each): `filter`, `selection`, `reference`, `event`,
`condition`, `cost`, `effect`, `continuous` (StaticEffect, Modification,
Duration), `replacement`, `quantity`, `zone`, `counter`, `designation`,
`token`, revised `ability`; existing `mana`, `color`, `subtype`, `card`,
`type`, `symbol`, `ident` stay.

## 8. Declaration mechanisms

Two, not three:

1. **Macro expansion with memory** (`MacroSet`, extended): all sugar — and
   keyword abilities and keyword actions are the *same* mechanism; kinds
   differ only in whether the engine consults the remembered name
   (verbs: event matching; ability names: `HasAbility(Flying)`; filters:
   provenance only).
2. **Vocabulary declarations**: names that are data with metadata, no
   expansion — subtypes (exists), designations, counter kinds.

## 9. Expanded everywhere — why MacroKind exists

Every macroable kind carries:

```
Expanded(Expansion<Self>)
Expansion<T> = { name: Ident, args: source-form args, value: Box<T> }
```

- The reader, on expanding a macro at a `T` position, wraps the result in
  `T::Expanded` with the invocation; nested invocations wrap at their own
  positions, so chains (`Woods` → `Forest` → `LandType("Forest")`) nest
  naturally.
- **Serialization writes the invocation back** (`AnyTarget`, not the
  filter): card files round-trip as written; `value` is reconstructed on
  read, not serialized. A file's meaning depends on the plugin set in
  scope — already true of the whole plugin system.
- `Ability::Keyword` dissolves into the uniform variant (today's
  `KeywordAbility`/`Expanded<T>` generalize into `Expansion<T>`).
- **`MacroKind` = exactly the types bearing `Expanded`**: Ability,
  CardFace, Subtype, Filter, Selection, Reference, Event, Condition,
  Effect, Action (declared verbs are `Action::Expanded` — that's how
  `Cost::Do` and the event log see names), Cost, TokenSpec. That is its
  reason to exist.

## 10. Migration impact

- `Target`, `Selector`, `Effect` in `ability.rs` are replaced by the
  selection complex + Effect AST. Two cards and three tokens consume them;
  `Lightning Bolt.ron` changes `targets: [AnyTarget]` →
  `targets: [Target(AnyTarget)]`; Treasure/Clue/Food RON is already
  written in this design's shape.
- `_005`/`_006` outputs (basic lands, vanilla creatures) are untouched.
- The tokens migration (jj bookmark `tokens-shelved`) unblocks on §4.
- Build order (corpus-frequency-driven, from the taxonomy doc): selection
  complex + Quantity → Cost + verbs → Event/Reference/Condition → Effect
  AST → StaticEffect/Duration → Replacement/Mode → leaves as needed.

## 11. Invariants

1. Macros never compute: substitution only; cycles/depth are load errors;
   control flow is Effect data.
2. No bare arrays with implicit semantics; named constructors or named
   struct fields only.
3. Verb object slots are unary Selections.
4. Canonical filters are context-free-correct.
5. Layers are derived from Modification ops, never written in data.
6. Classification (mana ability, loyalty ability) is derived, never
   stored; CDA is the one explicit, validated flag.
7. Designations: never copiable, never abilities; stored ones die with
   object identity (400.7); derived ones re-evaluate; granting a derived
   one is a load error.
8. Open vocabularies (subtypes, designations, counters, verbs, keyword
   names, ability words) are never closed Rust enums.

## Open questions (deferred, non-blocking)

- Exact `Expansion` args representation (raw RON spans vs typed) and how
  much of the chain serializes.
- `Bound(role)` naming scheme vs fixed per-pattern roles only.
- Whether `Superlative` and division-among-targets need first-class
  Selection support immediately or arrive with their first cards.
- `ChooseSpec` ↔ pawprint/vote generality (700.2i, 701.38) — defer until a
  card forces it.
