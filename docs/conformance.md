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
| multicolored / colorless tests | — | MISSING (P0.W4) |
| mana value comparison | `Stat(ManaValue, Cmp, Count)` | ✓ |
| type / subtype / supertype | `Type` / `Subtype` / `Supertype` | ✓ |
| P/T, loyalty, defense comparisons (layer-output reads) | `Stat(…)` over the layered view | ✓ |
| has-ability / lacks-ability | `HasAbility(Ident)` / `Not(…)` | ✓ |
| implicit zone quantifier (bare desc = battlefield permanent) | parser convention; canonical filters spell `InZone` explicitly | ✓ (by policy) |
| status tests (tapped / flipped / face-down / phased) | `Status(Status)` | ✓ grammar; object flags partial (P0.W5 seam) |
| combat-state tests (attacking, blocking, unblocked) | `StateBecomes` events only — no Filter atom | MISSING (P0.W4) |
| face-down characteristic exposure | — | MISSING (P0.W6) |
| controller / owner / opponent-of | `Controller` / `Owner` / `OpponentOf` | ✓ |
| attached-to / attachment | `AttachedTo` / `Attachment` | ✓ |
| generic relations (paired-with, exiled-with, …) | `RelatedBy(Ident, Filter)` | ✓ |
| cause-agent predicate ("destroyed by a spell an opponent controls") | — | MISSING (P0.W3, rides the cause triple) |
| targeting tests ("with N targets", "that targets …") | — | MISSING (P0.W4) |
| zone tests | `InZone(Zone)` — seven zones; no ante (variant-gated) | ✓ |
| has-counter | `HasCounter(Ident)` | ✓ |
| designations, stored + derived | `Designated(Ident)` | ✓ grammar; storage P0.W5 seam |
| player-property tests (life-total comparisons, speed) | designations cover flags; numeric player stats | partial — MISSING (P0.W4) |
| `target [desc]` | `Target(Quantity, Filter)` | ✓ |
| exactly-N / up-to-N / any-number selection | `Quantity::{Exactly, AtMost, AnyNumber, …}` | ✓ |
| variable-count targets, count locked at announce | `Count::X`; `LockPoint::Announce` | ✓ types; threading through decisions is P0.W2/W3 |
| `any target` shorthand | builtin Filter macro (`CreatureOrPlayer`-family) | ✓ |
| `each [desc]` (untargeted universal) | `Selection::Each(Filter)` | ✓ |
| `among [previously computed set]` | — | MISSING (P0.W4) |
| division/distribution among targets | — | MISSING (P0.W3) |
| "another/other" source-default exclusion | `AllOf([…, Not(Ref(This))])` | ✓ |
| "other" co-target set-distinctness (final-set check) | — | MISSING (P0.W4) |
| set-level cardinality constraints (menace) | `CountBound` on `DeonticAction::Block` | ✓ |
| random selection | `Selection::Random(Quantity, Filter)` | ✓ |

## 2. Event ontology (`events.md` §2–3 ↔ core `Event`, engine `GameEvent`)

| skill concept | deckmaste | status |
|---|---|---|
| zone-change master event (object, from, to, position, face, cause) | core `Event::ZoneMove`; engine `ZoneWillChange`/`ZoneChanged` | partial — `face` + `cause` fields MISSING (P0.W3/W6) |
| named views: dies / enters | builtin `Dies`/`ThisDies`/`Enters`/`ThisEnters` macros | ✓ |
| named views: sacrificed / discarded | engine `Sacrificed` / `Discarded` | ✓ |
| named views: destroyed (cause-restricted) | engine destroy verb exists; no cause-filtered trigger view | partial (P0.W3 cause triple) |
| named views: milled (top-of-library nuance) | — | MISSING (P0.W3) |
| named views: exiled / cast / played-land | `ZoneChanged` to exile; `SpellCast`; `LandPlayed` | ✓ |
| enters checked against already-modified object | layers-before-triggers discipline | ✓ |
| damage event (source, recipient, amount, combat?, flags) | engine `DamageDealt` | ✓ engine; grammar trigger coverage partial |
| life loss / gain (per-source events) | `LifeLost` / `LifeGained` | ✓ |
| life set-to-N (= gain/loss of difference) | — | MISSING (P0.W3) |
| counter placed / removed (objects AND players) | — | MISSING (P0.W3) |
| tap / untap (no-op = no event) | `Tapped` / `Untapped`, transition-only | ✓ |
| becomes-target (announce-time) | — | MISSING (P0.W3) |
| attack / block declaration events | `Attacking` / `Blocked` | ✓ |
| phase / step / turn entry | `TurnBegan` / `StepBegan`; core `BeginningOf(Phase, WhoseTurn)` | ✓ |
| day/night flip | — | MISSING (P0.W5) |
| phase in / out (explicitly NOT a zone change) | — | MISSING (P0.W5) |
| coin flip / die roll (ignored-roll never happened) | — | MISSING (P0.W3) |
| shuffle (also an information event) | — | MISSING (P0.W3) |
| reveal / look (scoped visibility window) | — | MISSING (P0.W6) |
| control change + becomes-deltas (transition-only) | core `StateBecomes` (tapped/untapped/attacking/blocked) | partial — control change MISSING (P0.W5) |
| cause triple (verb, agency, agent) as event data | implicit in variant choice | MISSING (P0.W3) — the headline gap |
| replaced events never trigger; look-back-in-time triggers | `ZoneWillChange` stage + LKI snapshots | ✓ (engine) |

## 3. Decision kinds (`choices.md` §2–4 ↔ engine `PendingDecision`/`Action`)

| skill concept | deckmaste | status |
|---|---|---|
| modes of a spell / activated ability (announce-locked) | — | MISSING (P0.W3) |
| cost intentions: alternative/additional, X, splice, hybrid/Phyrexian | — | MISSING (P0.W2) |
| targets, incl. variable count | `ChooseTargets` | ✓ |
| division / distribution among targets | — | MISSING (P0.W3) |
| triggered-ability modes/targets at stack-put | targets only | partial (P0.W3) |
| resolution-stage choices (named-player options) | `ChooseManaColor`, `DiscardCards` as instances | partial — no general kind (P0.W3) |
| vote (turn-order, from a specified player) | — | MISSING (P0.W3) |
| attack / block declaration | `DeclareAttackers` / `DeclareBlockers` | ✓ |
| combat damage assignment (whole-assignment legality) | `AssignCombatDamage` | ✓ |
| order own simultaneous triggers | `OrderTriggers` | ✓ |
| replacement/prevention application order | — | MISSING (P0.W4) |
| fixed-window yes/no ("… unless you pay") | — | MISSING (P0.W3) |
| pre-game: first turn, mulligans + London bottoming, companion, opening-hand | — | MISSING (P0.W3) |
| special actions beyond land play | `Action::PlayLand` only | MISSING (P0.W3 grammar; 116-machinery post-P0) |
| decider field (other-player choosers) | implicit `player` per variant | MISSING (P0.W3 schema) |
| visibility classes (open / committed-hidden + audit duty) | all implicitly open | MISSING (P0.W3 schema; audit duty P0.W6) |
| constraint arbitration (maximize-without-violating) | the Deontic rows ARE the input language | engine-seam (solver post-P0; P0.W1 guards live) |
| randomness as pseudo-decider (flip/roll kinds) | `Selection::Random` grammar only | MISSING (P0.W3) |

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
