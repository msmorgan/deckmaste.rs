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
  and theorem-form pins; Idris is reference only. The Rust card gate is pending.
- [Idris is a soundness gate](idris-is-a-soundness-gate.md) — Superseded for
  workbench choice; its legacy emitter remains until the Lean card gate lands.
- [Macro templates are bidirectional](macro-templates-are-bidirectional.md) —
  One typed template drives both rendering and parsing.
- [State-based actions are data](state-based-actions-are-data.md) — Ordinary
  state-based actions are swappable rules over explicit scopes.
- [English clauses are structural](english-clauses-are-structural.md) — Parse
  dependency, valency, voice, modality, and attachment before Magic lowering.
- [English coordination is structural](english-coordination-is-structural.md) —
  Keep shared context explicit and coordinate typed, contiguous members.
- [English productions ship their inverse](english-productions-ship-their-inverse.md) —
  A grammar slice lands only with its exact renderer inverse.
- [Semantics, spelling, lowering](semantics-spelling-lowering.md) — One
  semantics grammar; English relates two-way via spelling, core derives
  one-way via lowering.
- [English grammar is derived](english-grammar-is-derived.md) — One
  construction declaration compiles parse, render, and build.
- [English v2 rewrite](english-v2-rewrite.md) — A fresh declaration-owned
  parser replaces the English stack; the declaration is the type.
- [English grammar design in Lean](english-lean-design-workbench.md) — Formal
  grammar modeling and proofs guide an in-place `english_v2` grammar migration.
- [Builtin-v2 macro spelling and grammar](builtin-v2-macro-spelling-and-grammar.md) —
  Open plugin macros own positional semantic frames and the lexical facts
  needed to parse them.
- [Semantics v2](semantics-v2.md) — Semantics encodes surface English in
  situ and lowering owns all rearrangement; `deckmaste_semantics_v2` mirrors
  the Lean syntax over the `plugins_v2` format, and v1 retires at parity.
- [The kind index joins; union marking is spelling](kind-index-joins-union-marking-is-spelling.md) —
  Semantics takes the join lattice; the marked union constructions are
  spelling-boundary knowledge.
- [Oracle text is forward-anaphoric](oracle-text-is-forward-anaphoric.md) —
  Every anaphor resolves backward over the reading-order prefix; the lifting
  devices are lowering's job.
- [Card authoring binds no implicits](card-authoring-binds-no-implicits.md) —
  A card applies constructors and card-language functions positionally; a
  wrapping macro fills every optional slot and proof.
- [Core is explicit regions](core-explicit-regions.md) — Every core binding
  is a declared param or numbered def; references are register reads;
  lowering resolves once.
- [The workbench is RON-shaped](workbench-ron-shaped-and-label-rulings.md) —
  Every workbench construction is re-emittable RON; labels stay open over
  tables derived from the stubs.
