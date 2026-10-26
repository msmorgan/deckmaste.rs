# Project decisions

> 2026-10-26: `deckmaste_english_v4` and `deckmaste_lexical_source_v4` are now `deckmaste_english` and `deckmaste_lexical_source`; the census command is `cargo xtask english`.

These documents record durable project contracts for contributors. They describe
the intended design rather than a dated implementation inventory; current code
wins if an incidental implementation description drifts. Changing a decision
requires explicit review rather than an opportunistic refactor.

- [Authored card surface](authored-card-surface.md) — Keep card RON terse,
  structural, and stable.
- [Effect atom independence](effect-atom-independence.md) — An effect atom reads
  only its arguments and explicitly bound references.
- [Invalid semantic input fizzles](invalid-semantic-input-fizzles.md) — Bad semantic
  references degrade to no-ops instead of crashing play.
- [Engine and runner boundary](engine-runner-boundary.md) — Core exposes legal
  decisions; consumers own convenience policy.
- [Minimal core and data-driven rules](minimal-core-data-driven-rules.md) —
  Compose named mechanics from primitives and declared data.
- [Macros are declarative](macros-are-declarative.md) — Plugin macros are typed
  templates, not a control-flow language.
- [Conferrals come from registries](conferrals-come-from-registries.md) —
  Registries are the authority for open named behavior.
- [Types grant capabilities](types-grant-capabilities.md) — Current
  characteristics positively grant the capabilities subsystems consume.
- [Lean is the workbench](lean-is-the-workbench.md) — Active semantics modeling
  and theorem-form pins; Idris is reference only.
- [Macro templates are bidirectional](macro-templates-are-bidirectional.md) —
  One typed template drives both rendering and parsing.
- [State-based actions are data](state-based-actions-are-data.md) — Ordinary
  state-based actions are swappable rules over explicit scopes.
- [Semantics, spelling, lowering](semantics-spelling-lowering.md) — One
  semantics grammar; English relates two-way via spelling, core derives
  one-way via lowering.
- [English grammar: lexical analysis, Readings, and how the grammar is kept small](english-lexical-analysis.md)
  — Engine contract, round-trip laws, Reading normal forms, and how words reach
  the grammar.
- [English v4](english-v4.md) — The governing record for
  `deckmaste_syntax`, `deckmaste_english_v4`, and
  `deckmaste_lexical_source_v4`: data-driven schema/production engine,
  frame-driven complementation, CR-catalog separation, stratified buildout, and
  two-stage cutover.
- [Semantics v3](semantics-v3.md) — Clean-sheet DRT checker (`DRS`,
  `ReferentId`, nested boxes, `J1`, `J2`, `D′`, `AC>R1`), `Definition` vs.
  `Template` split, and Unified Action-Clause Spine (`instruct`, `does`,
  `would`/`instead`, `cant`/`must`/`may`/`gatedBy`, `did`/`tally`, `by`, `any`,
  `which`), executed Lean-first and mirrored into `deckmaste_semantics_v3` and
  `plugins_v3/`.
- [Card authoring binds no implicits](card-authoring-binds-no-implicits.md) —
  Card authoring binds no implicit parameters; core constructors take every
  field required. Superseded for the Lean workbench (2026-10-21): Templates may
  take optional named parameters after the required ones.
- [Workbench RON-shaped and label rulings](workbench-ron-shaped-and-label-rulings.md)
  — Registry label and RON-shaped syntax rulings.
- [The kind index joins; union marking is spelling](kind-index-joins-union-marking-is-spelling.md) —
  Semantics takes the join lattice; the marked union constructions are
  spelling-boundary knowledge.
- [Core is explicit regions](core-explicit-regions.md) — Every core binding
  is a declared param or numbered def; references are register reads;
  lowering resolves once.

Superseded historical decision records live in [`docs/archive/decisions/`](../archive/decisions/)
and are not active authority.
