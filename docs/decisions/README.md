# Project decisions

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
- [English grammar](english-lexical-analysis.md) — The one governing record for
  `deckmaste_english_v3`: engine contract, Readings, words, gate and size.
- [Semantics v3](semantics-v3.md) — Clean-sheet DRT checker (`DRS`,
  `ReferentId`, nested boxes, `J1`, `J2`, `D′`, `AC>R1`), `Definition` vs.
  `Template` split, and Unified Action-Clause Spine (`instruct`, `does`,
  `would`/`instead`, `cant`/`must`/`may`/`gatedBy`, `did`/`tally`, `by`, `any`,
  `which`), executed Lean-first and mirrored into `deckmaste_semantics_v3` and
  `plugins_v3/`.
- [The kind index joins; union marking is spelling](kind-index-joins-union-marking-is-spelling.md) —
  Semantics takes the join lattice; the marked union constructions are
  spelling-boundary knowledge.
- [Core is explicit regions](core-explicit-regions.md) — Every core binding
  is a declared param or numbered def; references are register reads;
  lowering resolves once.

Superseded historical decision records live in [`docs/archive/decisions/`](../archive/decisions/)
and are not active authority.
