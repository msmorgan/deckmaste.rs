# Project decisions

These documents record durable project contracts for contributors. They describe
the intended design rather than a dated implementation inventory; current code
wins if an incidental implementation description drifts. Changing a decision
requires explicit review rather than an opportunistic refactor.

- [Authored card surface](authored-card-surface.md) — Keep card RON terse,
  structural, and stable.
- [Effect atom independence](effect-atom-independence.md) — An effect atom reads
  only its arguments and explicitly bound references.
- [Invalid authoring fizzles](invalid-authoring-fizzles.md) — Bad authored
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
- [Idris is a soundness gate](idris-is-a-soundness-gate.md) — Idris validates
  authored model shape without becoming a second engine.
- [Macro templates are bidirectional](macro-templates-are-bidirectional.md) —
  One typed template drives both rendering and parsing.
- [State-based actions are data](state-based-actions-are-data.md) — Ordinary
  state-based actions are swappable rules over explicit scopes.
- [English clauses are structural](english-clauses-are-structural.md) — Parse
  dependency, valency, voice, modality, and attachment before Magic lowering.
- [English productions ship their inverse](english-productions-ship-their-inverse.md) —
  A grammar slice lands only with its exact renderer inverse.
