---
needs: [cards-elaborator-tables, cards-elab-load-gate]
---
**Master-form `EventFilter` replaces `Event`, and the CR-cited verb entailment
table kills the stringly `Performed` table.** This is the one breaking grammar
change with reach — but canon spells events via macros, so redefining macro
bodies migrates most of the corpus silently; expect ~10 of the ~52 canon cards
to need mechanical hand edits. Wizards cards regenerate; parsers retarget the
event macros.

Related: [[engine-history-windows]] (its `SinceYourLastUpkeep` ask becomes
`Lookback::SinceYour(PhaseStep)` here), [[core-count-query]] (event counting
lands as `Countable::Events`).

## Master forms

Each authored form carries ONLY the fields its kind supplies — dead facets
(`what:` on a step event, an amount on a designation change) are
unrepresentable by construction, not checker-caught. Forms lower to a
field-atom normal form over the single engine fact record `{kind, object,
patient, actor, source, from, to, cause, amount, counter, batch, before,
after, time}`.

```rust
enum EventFilter {  // ⟨macro⟩; fields optional = unconstrained
    ZoneChange   { what: Filter, from: Option<ZoneP>, to: Option<ZoneP>, cause: Option<Cause> },
    Damage       { source: Filter, to: Participant, combat: Option<bool>, amount: Option<AmountCmp> },
    LifeGained   { who: Filter, amount: Option<AmountCmp> },
    LifeLost     { who: Filter, amount: Option<AmountCmp> },
    Drawn        { who: Filter, amount: Option<AmountCmp> },
    CounterPlaced  { kind: Option<CounterRef>, on: Participant, amount: Option<AmountCmp> },
    CounterRemoved { kind: Option<CounterRef>, on: Participant, amount: Option<AmountCmp> },
    Cast         { who: Filter, what: Filter },       // onset family [CR#603.2]
    Played       { who: Filter, what: Filter },
    ActivatedAb  { who: Filter, what: Filter },
    AttackDeclared { by: Filter, against: Filter },
    BlockDeclared  { by: Filter, of: Filter },
    Attached     { what: Filter, to: Filter },
    StateBecame  { of: Filter, becomes: StateChange },
    BecomesTarget{ what: Participant, by: Filter, source: Option<Filter> },
                 // TWO ARMS: `by` = the targeting STACK OBJECT (shroud),
                 // `source` = its source (hexproof-from) [CR#702.11d,702.16b]
    StepBegins   { at: PhaseStep, whose: WhoseTurn },
    ControlChanged { of: Filter, to: Filter },
    DesignationChanged { name: Ident, of: Filter },
    TokenCreated { what: Filter, by: Filter },
    Used         { of: Reference },                   // object-scoped use history
    CoinFlipped  { by: Filter, won: Option<bool> },
    DiceRolled   { by: Filter },
    BecameDay, BecameNight,                           // expletive-"it" verbs
    // algebra & refinements, lane-gated:
    AllOf([EventFilter]), OneOf([EventFilter]), Not(Box<EventFilter>),
    OneOrMore(Box<EventFilter>),        // batch: matches the Occurrence once [CR#603.3b]
    Nth { n: usize /* ≥1 checked */, of: Box<EventFilter>, within: Lookback },
    When(Box<EventFilter>, Condition),  // game-state refinement
    Within(Box<EventFilter>, Lookback), // history lanes ONLY
}
enum CauseVerb { Sacrifice, Destroy, Discard, Exile, Mill, Play, Fight,
                 Explore, Regenerate /* closed */ }
struct Cause { verb: Option<CauseVerb>, agency: Option<Agency>, agent: Option<Filter> }
struct AmountCmp(Cmp, Count);
```

## Verbs are causes; the entailment table

The engine already stores sacrifice/discard/play as cause-verbs on
`ZoneChanged` facts (crates/deckmaste_engine/src/trigger.rs); the grammar
aligns to that. English trigger verbs become event MACROS: `Dies(f)` ≡
`ZoneChange(what: f, from: Battlefield, to: Graveyard)`; `Sacrificed(f)` adds
`cause: (verb: Sacrifice)`. The **entailment table** (core data, one CR-cited
row per `CauseVerb`) normalizes each cause verb to its fact form —
`Sacrifice ⇒ ZoneChange{from: Battlefield, to: Graveyard}` [CR#701.21a],
`Mill ⇒ ZoneChange{from: Library, to: Graveyard}` [CR#701.13a] — so
Dies-matches-sacrifice and graveyard-replacement-intercepts-sacrifice are
structural. `Event::Performed { verb: "…" }` and every reader of its strings
are deleted from `deckmaste_core`; the engine matcher half is
[[engine-eventfilter-bridge]].

## Caps, lanes, and the residual kind pass

- `caps : EventFilter → Caps` where `Caps = {object: Option<Sort>, patient:
  Option<Sort>, source, actor, amount, defender, counter}` — per-form rows
  emitted from Idris into `data/grammar-tables/`, each CR-cited. `AllOf` =
  union with patient-sort refinement; `OneOf` = meet; `Not` = no guarantees;
  `OneOrMore` aggregates (object → group, amount → sum). Trigger/replacement/
  payment binders introduce exactly one antecedent per guarantee — caps and
  anaphora are one machine.
- Lane gating (emitted table): `Triggered.event`/`CantHappen` require every
  `OneOf` disjunct to bottom out in a master form (kind-anchored — no
  freeze-everything `CantHappen`) [CR#603.2]; replacement lanes are
  would-facts with `may_target = false` bodies [CR#614]; `Within` is illegal
  in live lanes and REQUIRED under `Happened`/`Events(…)` history counting (no
  silent default window); `Nth` legal in both; `OneOrMore`/`Crossed` only in
  trigger/replacement lanes; `Delayed.event` fire-once with targets dropped
  [CR#603.7c].
- Residual kind-consistency pass, four rules, four fixtures: (a) `Not`'s
  operand bottoms out in master forms; (b) `When`/`Nth`/`OneOrMore` inherit
  the operand's caps; (c) `AllOf` over DIFFERENT master forms =
  `E-CAPS-CONTRADICTION`; (d) `OneOf` checked per disjunct.
- `Lookback { ThisTurn, ThisGame, LastTurn, ThisCombat, ThisStep,
  SinceYour(PhaseStep) }` split from `Timing { InstantSpeed, SorcerySpeed }` —
  two types, no shared enum.
- Intervening-if stays ability-level, never an event atom [CR#603.4]:
  `Triggered { event, if: Option<Condition>, … }` with the condition
  elaborated in the event-extended context. `Condition::Crossed { value,
  threshold }` reads the fact record's `before`/`after` [CR#714.2b].

## Done

- `EventFilter` replaces `Event` across core; `Performed` and its string verbs
  gone from core; entailment table landed as CR-cited data.
- Event macros (`Dies`, `Enters`, `ThisEnters`, `NextEndStep`, …) redefined
  over master forms; canon loads clean after ≤ ~10 mechanical edits.
- Caps + lane rows emitted from Idris; elaborator enforces lane gates and the
  four-rule kind pass, each with a reject fixture (`E-CAPS-*`).
- Wizards regenerated cleanly; parsers emit the event macros.

## Verification

- `idris2 --build mtg.ipkg` + `idris2 --exec emitTables` (in `idris/`).
- `cargo test --workspace`; `cargo xtask validate` clean on hand-authored
  plugins; `cargo xtask generate plugins/wizards` then re-validate.
- `cargo xtask elaborate --lock` re-blessed deliberately (diff reviewed).
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty; `cite
  audit --diff` read against every entailment/caps/lane row.
