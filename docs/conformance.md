# Skill conformance tables — the P0 progress meter

Concept-by-concept parity between the mtg-rules skill's reference docs
(pinned baseline: docs/rules-taxonomy.md §10) and deckmaste, maintained by
the P0 alignment effort (docs/todo.md priority 0). One row per skill
concept; statuses:

- **✓** — grammar exists in core (engine behavior may still be a seam).
- **partial** — some of the concept's fields/cases exist; the gap is named.
- **MISSING** — no grammar yet; tagged with its owner (during P0: the
  responsible wave; after closeout: the post-P0 grammar backlog).
- **engine-seam** — grammar complete; consumption is a tagged `todo!`
  (post-P0 conversion backlog; inventory: `rg 'todo!\("P0\.' crates/`).

Each P0 wave updates its rows on completion. P0's exit criterion
(amended at W7 closeout, user-approved): every remaining non-✓ row reads
**engine-seam** or carries an explicit owner tag — *post-P0 grammar
backlog (needs design dialogue)*, *variant-gated*, *UD-blocked*, or
*runner-by-design*. **P0 CLOSED 2026-06-12** — the seam inventory
(`rg 'todo!\("P0\.' crates/`) is the post-P0 work list.

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
| status tests (tapped / flipped / face-down / phased) | `Status(Status)` — the full [CR#110.5] 4×2 vocabulary | ✓ grammar; tapped live, other object flags engine-seam |
| combat-state tests (attacking, blocking, unblocked) | `StateFilter::{Attacking, Blocking, Unblocked}` | ✓ grammar; eval engine-seam |
| face-down characteristic exposure | `FaceDownSpec` (listed characteristics [CR#708.2]; `Default` = the 2/2 [CR#708.2a]) | ✓ grammar; object face flags + look rights ([CR#406.3,708.5]) engine-seam |
| controller / owner / opponent-of | `Controller` / `Owner` / `OpponentOf` | ✓ |
| attached-to / attachment | `AttachedTo` / `Attachment` | ✓ |
| generic relations (paired-with, exiled-with, …) | `RelatedBy(Ident, Filter)` | ✓ |
| cause-agent predicate ("destroyed by a spell an opponent controls") | `CausePattern.agent` | ✓ grammar; matching engine-seam |
| targeting tests ("with N targets", "that targets …") | `StateFilter::{Targets(Filter), TargetCount(CountBound)}` ([CR#115.9a..115.9c]; no-LKI current-state semantics doc-pinned) | ✓ grammar; eval engine-seam (stage-2 catch-all trips) |
| zone tests | `InZone(Zone)` — seven zones; no ante (variant-gated) | ✓ |
| has-counter | `HasCounter(Ident)` — LIVE read; player counters via the proxy object's map | ✓ |
| designations, stored + derived | `Designated(Ident)` reads LIVE off the engine `DesignationStore` | ✓ — registry live; granting effects engine-seam (table 6) |
| player-property tests (life-total comparisons, speed) | designations cover flags; numeric player stats absent | partial — post-P0 grammar backlog (needs design dialogue) |
| `target [desc]` | `Target(Quantity, Filter)` | ✓ |
| exactly-N / up-to-N / any-number selection | `Quantity::{Exactly, AtMost, AnyNumber, …}` | ✓ |
| variable-count targets, count locked at announce | `Count::X`; `LockPoint::Announce` | ✓ types; threading through the decision flow engine-seam |
| `any target` shorthand | builtin Filter macro (`CreatureOrPlayer`-family) | ✓ |
| `each [desc]` (untargeted universal) | `Selection::Each(Filter)` | ✓ |
| `among [previously computed set]` | `Selection::AmongNoted(key, Quantity)` — the domain is the `Noting` record, never re-evaluated | ✓ grammar; slot store engine-seam |
| division/distribution among targets | — ([CR#601.2d] announced-with-targets vs [CR#608.2d] at-resolution) | MISSING — post-P0 grammar backlog (needs design dialogue) |
| "another/other" source-default exclusion | `AllOf([…, Not(Ref(This))])` | ✓ |
| "other" co-target set-distinctness (final-set check) | `TargetSpec::Distinct(siblings, spec)` ([CR#115.7e] — final set, never fixed-binding) | ✓ grammar; enforcement engine-seam (loud) |
| set-level cardinality constraints (menace) | `CountBound` on `DeonticAction::Block` | ✓ |
| random selection | `Selection::Random(Quantity, Filter)` | ✓ |

## 2. Event ontology (`events.md` §2–3 ↔ core `Event`, engine `GameEvent`)

| skill concept | deckmaste | status |
|---|---|---|
| zone-change master event (object, from, to, position, face, cause) | core `Event::ZoneMove`; engine `ZoneWillChange`/`ZoneChanged` — all six coordinates | ✓ — every emitter is face-up today (morph/manifest post-P0); face-narrowed patterns trip the seam |
| named views: dies / enters | builtin `Dies`/`ThisDies`/`Enters`/`ThisEnters` macros | ✓ |
| named views: sacrificed / discarded / played | cause triples on `ZoneWillChange`/`ZoneChanged` | ✓ |
| named views: destroyed (cause-restricted) | verb "Destroy" cause rides BOTH causes ([CR#701.8b]: the `Destroy` arm + the lethal-damage SBA); builtin `Destroyed` macro | ✓ grammar; cause-pattern matching engine-seam (P0.W3) |
| named views: milled (top-of-library nuance) | — (needs a from-top verb) | MISSING — post-P0 grammar backlog (needs design dialogue) |
| named views: exiled / cast | `ZoneChanged` to exile; `SpellCast` | ✓ |
| enters checked against already-modified object | layers-before-triggers discipline | ✓ |
| damage event (source, recipient, amount, combat?, flags) | engine `DamageDealt` | ✓ engine; combat flag + trigger-view narrowing engine-seam |
| life loss / gain (per-source events) | `LifeLost` / `LifeGained` | ✓ |
| life set-to-N (= gain/loss of difference) | `PlayerAction::SetLife(Count)` resolves REAL to the gain/loss ([CR#119.5]; equal = no event) | ✓ |
| counter placed / removed (objects AND players) | `CounterPlaced`/`CounterRemoved` + `PutCounters`/`RemoveCounters` verbs | ✓ grammar; apply/storage engine-seam |
| tap / untap (no-op = no event) | `Tapped` / `Untapped`, transition-only | ✓ |
| becomes-target (announce-time) | `Event::BecomesTarget{what, by}` ([CR#601.2c]; ward the exemplar [CR#702.21a]) + shaped `BecameTarget` | ✓ grammar; announce emission engine-seam |
| attack / block declaration events | `Attacking` / `Blocked` | ✓ |
| phase / step / turn entry | `TurnBegan` / `StepBegan`; core `BeginningOf(Phase, WhoseTurn)` | ✓ |
| day/night flip | `Event::DesignationChanged` ([CR#731.1a]) + shaped engine event; registry holds the `Mode` | ✓ grammar; flip emission engine-seam |
| phase in / out (explicitly NOT a zone change) | `StateFilterEvent::Phased(Phasing)` ([CR#702.26b]) | ✓ grammar; phasing machinery engine-seam |
| coin flip / die roll (ignored-roll never happened) | `CoinFlipped`/`DieRolled` + `FlipCoins`/`RollDice` verbs | ✓ grammar; apply engine-seam |
| shuffle (also an information event) | `PlayerAction::Shuffle` + `Shuffled` apply — REAL, seeded rng ([CR#701.24a]; UD-8) | ✓ (revealed-state reset [CR#701.20d] = seam) |
| reveal / look (scoped visibility window) | `Reveal{what, to}` verb ([CR#701.20a,701.20e]; cost-eligible) + shaped `Revealed` event | ✓ grammar; emit + window lifetimes engine-seam |
| control change + becomes-deltas (transition-only) | core `StateBecomes` (tapped/untapped/attacking/blocked + phased/turned-face/designated/controlled-by) + shaped `ControlChanged` event | ✓ grammar; new-delta matching + L2 emission engine-seam |
| cause triple (verb, agency, agent) as event data | core `Agency`/`CausePattern`; engine `Cause` on zone changes + `Tapped` | ✓ — named views are constructors over ONE encoding; pattern matching engine-seam |
| replaced events never trigger; look-back-in-time triggers | `ZoneWillChange` stage + LKI snapshots | ✓ (engine) |

## 3. Decision kinds (`choices.md` §2–4 ↔ engine `PendingDecision`/`Action`)

| skill concept | deckmaste | status |
|---|---|---|
| modes of a spell / activated ability (announce-locked) | `PendingDecision::ChooseModes` shell | ✓ schema; surfacing engine-seam |
| cost intentions: alternative/additional, X, splice, hybrid/Phyrexian | `CostChange::Additional`, `AlternativeCost` grammar + `PendingDecision::ChooseCostOptions` shell ([CR#601.2b]) | ✓ grammar+schema; surfacing engine-seam |
| targets, incl. variable count | `ChooseTargets` | ✓ |
| division / distribution among targets | `PendingDecision::Division` shell | ✓ schema; surfacing engine-seam |
| triggered-ability modes/targets at stack-put | targets wired; the `ChooseModes` shell applies at placement too | engine-seam (placement flow reuses the existing kinds) |
| resolution-stage choices (named-player options) | `ChooseManaColor`, `DiscardCards` as instances | engine-seam — generalizing the kind is code shape, not grammar |
| vote (turn-order, from a specified player) | `PendingDecision::Vote` shell | ✓ schema; surfacing engine-seam |
| attack / block declaration | `DeclareAttackers` / `DeclareBlockers` | ✓ |
| combat damage assignment (whole-assignment legality) | `AssignCombatDamage` | ✓ |
| order own simultaneous triggers | `OrderTriggers` | ✓ |
| replacement/prevention application order | `PendingDecision::OrderReplacements` shell | ✓ schema; [CR#616.1] fixpoint engine-seam |
| fixed-window yes/no ("… unless you pay") | `PendingDecision::YesNo` shell | ✓ schema; surfacing engine-seam |
| pre-game: first turn, mulligans + London bottoming, companion, opening-hand | `PreGame(PreGameKind)` shell (bottoming = committed-hidden) | ✓ schema; surfacing engine-seam |
| special actions beyond land play | `Action::Special(SpecialAction)` over the closed list | ✓ shell; 116-machinery post-P0 |
| decider field (other-player choosers) | `DeciderSpec` via `DecisionPoint` | ✓ |
| visibility classes (open / committed-hidden + audit duty) | `Visibility` via `DecisionPoint` | ✓ schema; audit duty (commit-and-open at forced reveals, [CR#708.9]) engine-seam |
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
| skipped-window semantics ("the next" skips skipped) | — | MISSING — post-P0 grammar backlog (needs design dialogue) |
| duration taxonomy (fixed / until-event / for-as-long-as / rest-of-game) | `Duration::{FixedUntil(TurnMarker), UntilEvent, ForAsLongAs, EndOfGame}` | ✓ grammar; sweeps beyond end-of-turn + predicate tracking engine-seam (creation guard in `resolve.rs`) |
| `started` latch, never-started/already-ended edges | engine effect-instance record | engine-seam (arrives with ForAsLongAs tracking) |
| lock-point axis on stored values | `LockPoint` (10 points) | ✓ type; threading through the decision flow engine-seam |
| once-per-turn limiter scopes (object vs controller) | `UseLimit::{OncePerTurn, OncePerGame}` | partial — post-P0 grammar backlog (RON-compat makes the scope respelling design-heavy) |

## 5. Costs & mana (`costs.md`, `mana.md` ↔ `cost.rs`/`mana.rs`/`continuous.rs`)

| skill concept | deckmaste | status |
|---|---|---|
| printed mana cost / activation cost positions | face `mana_cost`; `ActivatedAbility.cost` | ✓ |
| additional costs, mandatory + optional/kicker (pipeline-positional) | `CostChange::Additional { components, optional }` | ✓ grammar; pipeline application engine-seam |
| alternative cost, one-per-spell, rides the cast permission | `AlternativeCost::{Free, Components}` on `May(Cast(cost: …))` | ✓ grammar; announce selection + one-per-spell rule engine-seam |
| declaration toll / resolution toll | `Deontic::Gate` / `Effect::Unless` | ✓ |
| recurring slots (echo, cumulative upkeep) | — | MISSING (keyword-macro buildout, post-P0) |
| special-action costs (X chosen before payment) | — | MISSING — post-P0 grammar backlog (needs design dialogue, with the 116-machinery) |
| total-cost pipeline + lock ([CR#601.2f]) | `TotalCost { base, trace, locked }` | ✓ type; runtime application engine-seam (P0.W2 guard live in `legal.rs`) |
| cost-modification hook (convoke/delve/improvise/assist/waterbend) | the composite-given primitive | engine-seam (payment-substitution interface, post-P0) |
| symbol vocabulary, cost-side (generic, colored, {C}, X, hybrid, Phyrexian, snow) | `ManaSymbol` complete | ✓ grammar; payment evaluates simple symbols only — X/hybrid/Phyrexian/snow spells are never OFFERED (scoped absence, engine-seam) |
| {0} vs no-mana-cost ([CR#118.5..118.6]) | `[]` = absent/unpayable (can_cast gate); `[Generic(0)]` = {0} | ✓ |
| alternative unlocks an unpayable base ([CR#118.6a]) | grammar ✓ | engine-seam |
| multi-way symbol announce timing ([CR#118.13]) | rides the `ChooseCostOptions` announce shell | ✓ schema; surfacing engine-seam |
| mana unit schema: type + source snapshot + riders + persistence | `ManaProduction`/`ManaRider` grammar; pool = six counts | ✓ grammar; pool units engine-seam (production guard live in `resolve.rs`) |
| spend restrictions / on-spend effects / on-spend triggers / persistence | `ManaRider::{SpendOnly, GrantOnSpend, TriggerOnSpend, Persistent}` | ✓ grammar |
| production-side symbol readings (hybrid choice, Phyrexian color, generic→colorless) | `ManaSpec` | ✓ |
| undefined-type production = no mana; "could produce" ([CR#106.7]) | — | MISSING (engine query, post-P0) |
| mana abilities never forced; no auto-tap | explicit-choice policy | ✓ |
| mana abilities mid-payment ([CR#601.2g]) | — | engine-seam — pure decision flow (allow mana activations while `PayMana` is pending); no grammar involved |
| payment as transactional batch + [CR#733.1] rewind | — | engine-seam — event-log mechanics (batch + irreversibility marker); no grammar involved (UD-10 decides knowledge semantics) |
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

## 7. Outcomes & information (`outcomes.md`, `information.md` ↔ `action.rs`/`continuous.rs`/engine `sba.rs`/`event.rs`)

| skill concept | deckmaste | status |
|---|---|---|
| loss SBAs: life ([CR#704.5a]) / empty draw ([CR#704.5b]) / poison ([CR#704.5c]) | `sba::sweep` + `LossReason` — all three LIVE (poison reads the proxy's counter map, dormant until counter apply lands) | ✓ |
| effect outcomes: "loses" / "wins the game" ([CR#104.3e,104.2b]) | `PlayerAction::{LoseGame, WinGame}` riding `By(player, …)` | ✓ grammar; resolve arms engine-seam |
| concession ([CR#104.3a] — unstoppable, pierces every gate) | `Action::Concede` — REAL and ENUMERATED at every choice boundary ("you can also concede"; runner filters); answers ANY pending decision; two-player terminal tested | ✓ (multiplayer leave-game cleanup [CR#800.4a] = loud seam) |
| can't-lose / can't-win gates (U5 settled: precedence per check, not consumption) | `StaticEffect::OutcomeGate{who, gate}` over `OutcomeGateKind::{CantLose, CantWin}` | ✓ grammar; SBA-sweep presence guard trips on any gate row |
| win∧lose → lose arbitration ([CR#104.3f]); same-result SBA batch replacement ([CR#704.7]) | doc-pinned on the gate/verbs | engine-seam (arrives with the outcome verbs) |
| last-player-standing win / all-lose draw ([CR#104.2a,104.4a]) | `check_game_end` → `GameOutcome::{Win, Draw}` | ✓ |
| mandatory-loop draw ([CR#104.4b]) | — | engine-seam, BLOCKED on UD-11 equality (no monitor = no trip point; note on `check_game_end`) |
| restart ([CR#727.1] terminal-with-carryover, restarter starts [CR#727.1a]) | `PlayerAction::RestartGame` (actor binding = the restarter) | ✓ grammar; resolve arm engine-seam |
| subgames ([CR#729] — a context push, not a restart) | — | deferred (variant-adjacent; noted on the verb's doc) |
| elimination fallout ([CR#800.4a] objects leave, control ends, residue exiled) | — | loud seam in the concede arm (multiplayer only; two-player terminal needs none) |
| zone visibility defaults ([CR#400.2] hidden = property of the ZONE) | `Zone::is_hidden()` (hand, library) | ✓ |
| face-down committed payload ([CR#708.2]; default 2/2 [CR#708.2a]) | `FaceDownSpec` + `Face` on zone events | ✓ grammar; object flags engine-seam (table 1) |
| look rights are STATEFUL per (player, object) ([CR#406.3] persists past the grant) | — | engine-seam (grant records arrive with face-down exile) |
| differentiation duty + reveal-on-leave audit ([CR#708.6,708.9]) | — | engine-seam (commit-and-open bookkeeping) |
| reveal / look operations ([CR#701.20a,701.20e]) | `Reveal{what, to}` (cost-eligible) + `Revealed` event | ✓ grammar; window lifetimes engine-seam |
| event audience annotations (information.md §6 projection boundary) | `GameEvent::audience(&state)` → `Audience::{Public, Restricted}` — DERIVED, hidden→hidden moves restrict to the owner | ✓ (coarse; refinements ride face-down/look machinery) |
| per-player information-set projection | — | runner concern BY DESIGN (the full-info punt; annotations above are its contract) |
