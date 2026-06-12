# Skill conformance tables — the P0 progress meter

Concept-by-concept parity between the mtg-rules skill's reference docs
(pinned baseline: docs/rules-taxonomy.md §10) and deckmaste, maintained by
the P0 alignment effort (docs/todo.md priority 0). One row per skill
concept; statuses:

- **✓** — grammar exists in core (engine behavior may still be a seam).
- **partial** — some of the concept's fields/cases exist; the gap is named.
- **MISSING (P0.W*n*)** — no grammar yet; tagged with the master plan's wave.
- **engine-seam** — grammar complete; consumption is a tagged `todo!`
  (post-P0 conversion backlog; inventory: `rg 'todo!\("P0\.' crates/`).

Each P0 wave updates its rows on completion. P0's exit criterion: every
remaining non-✓ row reads **engine-seam**.

## 1. Predicates & selectors (`queries.md` §1–2 ↔ `Filter`/`TargetSpec`/`Selection`)

| skill concept | deckmaste | status |
|---|---|---|
| name test (identity-aware self-names) | `Named(Ident)`; self = `Ref(This)` | ✓ |
| color test, monocolored | `ColorIs(Color)` | ✓ |
| multicolored / colorless tests | `Multicolored` / `Colorless` atoms | ✓ grammar; eval engine-seam |
| mana value comparison | `Stat(ManaValue, Cmp, Count)` | ✓ |
| type / subtype / supertype | `Type` / `Subtype` / `Supertype` | ✓ |
| P/T, loyalty, defense comparisons (layer-output reads) | `Stat(…)` over the layered view | ✓ |
| has-ability / lacks-ability | `HasAbility(Ident)` / `Not(…)` | ✓ |
| implicit zone quantifier (bare desc = battlefield permanent) | parser convention; canonical filters spell `InZone` explicitly | ✓ (by policy) |
| status tests (tapped / flipped / face-down / phased) | `Status(Status)` | ✓ grammar; tapped live, other object flags engine-seam (face-down model P0.W6) |
| combat-state tests (attacking, blocking, unblocked) | `StateFilter::{Attacking, Blocking, Unblocked}` | ✓ grammar; eval engine-seam |
| face-down characteristic exposure | — | MISSING (P0.W6) |
| controller / owner / opponent-of | `Controller` / `Owner` / `OpponentOf` | ✓ |
| attached-to / attachment | `AttachedTo` / `Attachment` | ✓ |
| generic relations (paired-with, exiled-with, …) | `RelatedBy(Ident, Filter)` | ✓ |
| cause-agent predicate ("destroyed by a spell an opponent controls") | `CausePattern.agent` | ✓ grammar; matching engine-seam |
| targeting tests ("with N targets", "that targets …") | — | MISSING (P0.W4b mini-dialogue) |
| zone tests | `InZone(Zone)` — seven zones; no ante (variant-gated) | ✓ |
| has-counter | `HasCounter(Ident)` | ✓ |
| designations, stored + derived | `Designated(Ident)` reads LIVE off the engine `DesignationStore` | ✓ — registry live; granting effects engine-seam (table 6) |
| player-property tests (life-total comparisons, speed) | designations cover flags; numeric player stats | partial — MISSING (P0.W4) |
| `target [desc]` | `Target(Quantity, Filter)` | ✓ |
| exactly-N / up-to-N / any-number selection | `Quantity::{Exactly, AtMost, AnyNumber, …}` | ✓ |
| variable-count targets, count locked at announce | `Count::X`; `LockPoint::Announce` | ✓ types; threading through decisions is P0.W2/W3 |
| `any target` shorthand | builtin Filter macro (`CreatureOrPlayer`-family) | ✓ |
| `each [desc]` (untargeted universal) | `Selection::Each(Filter)` | ✓ |
| `among [previously computed set]` | — | MISSING (P0.W4b mini-dialogue) |
| division/distribution among targets | — | MISSING (P0.W3) |
| "another/other" source-default exclusion | `AllOf([…, Not(Ref(This))])` | ✓ |
| "other" co-target set-distinctness (final-set check) | — | MISSING (P0.W4b mini-dialogue) |
| set-level cardinality constraints (menace) | `CountBound` on `DeonticAction::Block` | ✓ |
| random selection | `Selection::Random(Quantity, Filter)` | ✓ |

## 2. Event ontology (`events.md` §2–3 ↔ core `Event`, engine `GameEvent`)

| skill concept | deckmaste | status |
|---|---|---|
| zone-change master event (object, from, to, position, face, cause) | core `Event::ZoneMove`; engine `ZoneWillChange`/`ZoneChanged` | partial — `face` + `cause` fields MISSING (P0.W3/W6) |
| named views: dies / enters | builtin `Dies`/`ThisDies`/`Enters`/`ThisEnters` macros | ✓ |
| named views: sacrificed / discarded / played | cause triples on `ZoneWillChange`/`ZoneChanged` | ✓ |
| named views: destroyed (cause-restricted) | engine destroy verb exists; no cause-filtered trigger view | partial (P0.W3 cause triple) |
| named views: milled (top-of-library nuance) | — | MISSING (P0.W3) |
| named views: exiled / cast | `ZoneChanged` to exile; `SpellCast` | ✓ |
| enters checked against already-modified object | layers-before-triggers discipline | ✓ |
| damage event (source, recipient, amount, combat?, flags) | engine `DamageDealt` | ✓ engine; grammar trigger coverage partial |
| life loss / gain (per-source events) | `LifeLost` / `LifeGained` | ✓ |
| life set-to-N (= gain/loss of difference) | — | MISSING (P0.W3) |
| counter placed / removed (objects AND players) | `CounterPlaced`/`CounterRemoved` + `PutCounters`/`RemoveCounters` verbs | ✓ grammar; apply/storage engine-seam |
| tap / untap (no-op = no event) | `Tapped` / `Untapped`, transition-only | ✓ |
| becomes-target (announce-time) | — | MISSING (P0.W3) |
| attack / block declaration events | `Attacking` / `Blocked` | ✓ |
| phase / step / turn entry | `TurnBegan` / `StepBegan`; core `BeginningOf(Phase, WhoseTurn)` | ✓ |
| day/night flip | registry holds the game-scope `Mode`; flip EVENT grammar absent | MISSING (deferred from W5 → P0.W6 dialogue) |
| phase in / out (explicitly NOT a zone change) | — | MISSING (deferred from W5 → P0.W6 dialogue) |
| coin flip / die roll (ignored-roll never happened) | `CoinFlipped`/`DieRolled` + `FlipCoins`/`RollDice` verbs | ✓ grammar; apply engine-seam |
| shuffle (also an information event) | — | MISSING (P0.W3) |
| reveal / look (scoped visibility window) | — | MISSING (P0.W6) |
| control change + becomes-deltas (transition-only) | core `StateBecomes` (tapped/untapped/attacking/blocked) | partial — control change MISSING (deferred from W5 → P0.W6 dialogue) |
| cause triple (verb, agency, agent) as event data | core `Agency`/`CausePattern`; engine `Cause` on zone changes + `Tapped` | ✓ — named views are constructors over ONE encoding; pattern matching engine-seam |
| replaced events never trigger; look-back-in-time triggers | `ZoneWillChange` stage + LKI snapshots | ✓ (engine) |

## 3. Decision kinds (`choices.md` §2–4 ↔ engine `PendingDecision`/`Action`)

| skill concept | deckmaste | status |
|---|---|---|
| modes of a spell / activated ability (announce-locked) | `PendingDecision::ChooseModes` shell | ✓ schema; surfacing engine-seam |
| cost intentions: alternative/additional, X, splice, hybrid/Phyrexian | `CostChange::Additional`, `AlternativeCost` grammar | ✓ grammar; the announce DECISION kinds remain MISSING (P0.W3) |
| targets, incl. variable count | `ChooseTargets` | ✓ |
| division / distribution among targets | `PendingDecision::Division` shell | ✓ schema; surfacing engine-seam |
| triggered-ability modes/targets at stack-put | targets only | partial (P0.W3) |
| resolution-stage choices (named-player options) | `ChooseManaColor`, `DiscardCards` as instances | partial — no general kind (P0.W3) |
| vote (turn-order, from a specified player) | `PendingDecision::Vote` shell | ✓ schema; surfacing engine-seam |
| attack / block declaration | `DeclareAttackers` / `DeclareBlockers` | ✓ |
| combat damage assignment (whole-assignment legality) | `AssignCombatDamage` | ✓ |
| order own simultaneous triggers | `OrderTriggers` | ✓ |
| replacement/prevention application order | `PendingDecision::OrderReplacements` shell | ✓ schema; [CR#616.1] fixpoint engine-seam |
| fixed-window yes/no ("… unless you pay") | `PendingDecision::YesNo` shell | ✓ schema; surfacing engine-seam |
| pre-game: first turn, mulligans + London bottoming, companion, opening-hand | `PreGame(PreGameKind)` shell (bottoming = committed-hidden) | ✓ schema; surfacing engine-seam |
| special actions beyond land play | `Action::Special(SpecialAction)` over the closed list | ✓ shell; 116-machinery post-P0 |
| decider field (other-player choosers) | `DeciderSpec` via `DecisionPoint` | ✓ |
| visibility classes (open / committed-hidden + audit duty) | `Visibility` via `DecisionPoint` | ✓ schema; audit duty P0.W6 |
| constraint arbitration (maximize-without-violating) | the Deontic rows ARE the input language | engine-seam (solver post-P0; P0.W1 guards live) |
| randomness as pseudo-decider (flip/roll kinds) | `DeciderSpec::Rng`; flip/roll verbs + events | ✓ grammar; execution engine-seam |

## 3b. Memory, queries & copies (`queries.md` §3–5, [CR#607.2,707.10])

| skill concept | deckmaste | status |
|---|---|---|
| linked slots / chosen-value anaphora (write side) | `NotedKind` + `ChooseAndNote(key, kind)` + `Effect::Noting{key, effect}` | ✓ grammar; slot store engine-seam (deferred from W5) |
| noted reads | `Reference::Linked(key)`, `Count::Noted(key)` | ✓ grammar; eval engine-seam |
| engine-tracked history counts | `Count::Query(QueryKey)` — CardsDrawn/LandsPlayed EVALUATE off live tallies; StormCount seam | ✓ |
| copy-on-stack vs cast-a-copy ([CR#707.10,707.12]) | `CopySpell(Selection)` verb; cast-a-copy rides the 601 pipeline later | ✓ grammar; execution engine-seam |
| target re-check + fizzle ([CR#608.2b]) | `targets_still_legal` at resolution | partial — LKI fallback for departed sources is a seam |
| ⊥ semantics ([CR#107.2] coercion, skip-on-undefined) | documented convention | partial — formalize at first ⊥ collision (UD-6 ADR) |

## 4. Temporal & deontic modifiers (`temporal.md`, `deontics.md` §2–3 ↔ `temporal.rs`/`deontic.rs`)

| skill concept | deckmaste | status |
|---|---|---|
| four polarities over typed actions (May/Cant/Must/Gate) | `Deontic` over `DeonticAction` | ✓ grammar; legality evaluation is engine-seam (P0.W1 presence guards live in `legal.rs`) |
| resolution Toll (price bites at resolution) | `Effect::Unless` (named the Toll node) | ✓ |
| Only window refinement — activation timing | `ActivatedAbility.window` | ✓ grammar; InstantSpeed/SorcerySpeed evaluated, other windows engine-seam |
| cast-permission window (flash-style May rows) | `DeonticAction::Cast { window }` | ✓ grammar; consumption engine-seam (cast guard) |
| AsThough premises (scoped counterfactuals) | `StaticEffect::AsThough` (`SpendManaAsAnyColor` + macros) | partial — premises accrete; consumption engine-seam |
| shared Window vocabulary, reading per position | `Window` (speeds, ThisTurn, DuringTurn, DuringStep) | ✓ |
| skipped-window semantics ("the next" skips skipped) | — | MISSING (P0.W3) |
| duration taxonomy (fixed / until-event / for-as-long-as / rest-of-game) | `Duration::{FixedUntil(TurnMarker), UntilEvent, ForAsLongAs, EndOfGame}` | ✓ grammar; sweeps beyond end-of-turn + predicate tracking engine-seam (creation guard in `resolve.rs`) |
| `started` latch, never-started/already-ended edges | engine effect-instance record | engine-seam (arrives with ForAsLongAs tracking) |
| lock-point axis on stored values | `LockPoint` (8 points) | ✓ type; threading is P0.W2/W3 |
| once-per-turn limiter scopes (object vs controller) | `UseLimit::{OncePerTurn, OncePerGame}` | partial — controller-scoped trigger flavor needs a scope distinction (P0.W3) |

## 5. Costs & mana (`costs.md`, `mana.md` ↔ `cost.rs`/`mana.rs`/`continuous.rs`)

| skill concept | deckmaste | status |
|---|---|---|
| printed mana cost / activation cost positions | face `mana_cost`; `ActivatedAbility.cost` | ✓ |
| additional costs, mandatory + optional/kicker (pipeline-positional) | `CostChange::Additional { components, optional }` | ✓ grammar; pipeline application engine-seam |
| alternative cost, one-per-spell, rides the cast permission | `AlternativeCost::{Free, Components}` on `May(Cast(cost: …))` | ✓ grammar; announce selection + one-per-spell rule engine-seam |
| declaration toll / resolution toll | `Deontic::Gate` / `Effect::Unless` | ✓ |
| recurring slots (echo, cumulative upkeep) | — | MISSING (keyword-macro buildout, post-P0) |
| special-action costs (X chosen before payment) | — | MISSING (P0.W3 special actions) |
| total-cost pipeline + lock ([CR#601.2f]) | `TotalCost { base, trace, locked }` | ✓ type; runtime application engine-seam (P0.W2 guard live in `legal.rs`) |
| cost-modification hook (convoke/delve/improvise/assist/waterbend) | the composite-given primitive | engine-seam (payment-substitution interface, post-P0) |
| symbol vocabulary, cost-side (generic, colored, {C}, X, hybrid, Phyrexian, snow) | `ManaSymbol` complete | ✓ grammar; payment evaluates simple symbols only — X/hybrid/Phyrexian/snow spells are never OFFERED (scoped absence, engine-seam) |
| {0} vs no-mana-cost ([CR#118.5..118.6]) | `[]` = absent/unpayable (can_cast gate); `[Generic(0)]` = {0} | ✓ |
| alternative unlocks an unpayable base ([CR#118.6a]) | grammar ✓ | engine-seam |
| multi-way symbol announce timing ([CR#118.13]) | — | MISSING (P0.W3 decision schema) |
| mana unit schema: type + source snapshot + riders + persistence | `ManaProduction`/`ManaRider` grammar; pool = six counts | ✓ grammar; pool units engine-seam (production guard live in `resolve.rs`) |
| spend restrictions / on-spend effects / on-spend triggers / persistence | `ManaRider::{SpendOnly, GrantOnSpend, TriggerOnSpend, Persistent}` | ✓ grammar |
| production-side symbol readings (hybrid choice, Phyrexian color, generic→colorless) | `ManaSpec` | ✓ |
| undefined-type production = no mana; "could produce" ([CR#106.7]) | — | MISSING (engine query, post-P0) |
| mana abilities never forced; no auto-tap | explicit-choice policy | ✓ |
| mana abilities mid-payment ([CR#601.2g]) | — | MISSING (P0.W3 decision flow) |
| payment as transactional batch + [CR#733.1] rewind | — | MISSING (P0.W3 cause-tagged event batches) |
| pool empties per step/phase; per-unit persistence override | `ManaEmptied` turn-based action | ✓ engine; override engine-seam |

## 6. Designations, emblems & state instances (`designations.md`, `state.md` ↔ engine `state.rs`)

| skill concept | deckmaste | status |
|---|---|---|
| designation scopes: game value / player value / per-object instances | `DesignationStore{game, players, objects}`; `DesignationValue::{Flag, Holder, Mode}` | ✓ — registry live, granting effects engine-seam |
| object designation = grantor-parameterized temporary static payload (goad) | decl payload is the TEMPLATE; `DesignationInstance{grantor, duration}` supplies the bindings | ✓ storage; payload application = the layers pipeline's designation source (engine-seam) |
| multiplicity: per-grantor instances on independent clocks ([CR#701.15b..701.15c]) | `Vec<DesignationInstance>` per (object, name) — never a merged grantor set | ✓ |
| derived reads (`Designated(name)` never goes stale) | live registry read in `target.rs` (object entry, or the player's for proxies) | ✓ |
| emblems: command-zone ability holders, never on the battlefield ([CR#114.1,114.4]) | `PlayerAction::GetEmblem(Vec<Ability>)`; `ObjectKind::Emblem` | ✓ grammar; minting engine-seam |
| commander designation (damage ledger, command-zone replacement) | — | deferred — variant-gated; arrives with variant support as a designation + damage-result reader |
