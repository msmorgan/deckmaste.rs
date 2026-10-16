---
needs:
  - lean-v3-authoring-1-deontic-cutover
  - lean-v3-authoring-2-verb-and-clause-spine
  - lean-v3-authoring-3-nominals-and-predicates
  - lean-v3-authoring-4-declared-bodies
---
# Refactor the Lean v3 authoring surface before mirroring into Rust and plugins_v3

**Refactor the `lean/Semantics/` authoring surface so that authored cards and
keyword definitions read like Oracle English while expanding into the minimal
Core AST primitives, and retire all `v1`/`v2` module, macro, and alias
holdovers before `semantics-v3-dsl-and-plugins` mirrors the workbench into
`crates/deckmaste_semantics_v3` and `plugins_v3/`.** Umbrella epic for:
[1 deontic cutover](lean-v3-authoring-1-deontic-cutover.md),
[2 verb and clause spine](lean-v3-authoring-2-verb-and-clause-spine.md),
[3 nominals and predicates](lean-v3-authoring-3-nominals-and-predicates.md), and
[4 declared bodies and legacy name retirement](lean-v3-authoring-4-declared-bodies.md).
Standard constraints apply.

## Governing Principles

1. **Minimal Core AST Primitives (`Words.lean`, `Events.lean`, `Abilities.lean`):**
   - The Core AST remains the minimal set of orthogonal primitives so
     `deckmaste_lowering_v3` handles a small, closed constructor basis rather
     than dozens of overlapping surface forms.
2. **Surface-Faithful Authoring via `Template`s and `Definition`s:**
   - The surface of an authored card in `lean/Semantics/Cards/` or a keyword
     `Definition` must remain directly parallel to the English card text so
     spelling attachment and the card ingestion pipeline work cleanly.
   - Because `lean/Semantics/Authoring.lean` records `.call name args` on every
     `semantic_template` and `semantic_definition`, rich surface vocabulary is
     built as `Template`s and `Definition`s expanding into the minimal Core AST
     primitives.
3. **Literal and Enum Coercions Belong to the Rust `v3` DSL:**
   - Bare numeral coercion (`1` vs `.lit 1`) and leading `.` enum prefixes are
     handled by `deckmaste_dsl_v3` in `semantics-v3-dsl-and-plugins`; Lean
     retains explicit `.lit` and `.` constructors where Lean's elaborator
     requires them.
4. **No `v1`/`v2` Holdover Names:**
   - Retire the `Semantics.Macros` hierarchy, `Declared.lean` split,
     `semantic_macro` command, `MacroParameters` / `MacroContext` / `MacroSplice`
     identifiers, empty `Phrase.lean` / `Triggers.lean` shims, `Binding` /
     `Bindings` aliases, and the `Semantic Macro` glossary entry in favor of
     `Primitives`, `Templates`, `Definitions`, `CallerScope`, `AuthoringContext`,
     `Referent`, and `DRS`.

## Sub-Tickets

1. [`lean-v3-authoring-1-deontic-cutover`](lean-v3-authoring-1-deontic-cutover.md):
   Retire the `deonticActionFor` `v2`-port shim and all `Deed`-taking deontic
   signatures in favor of `v3` `Instruction`-taking templates (`cant`, `cantBe`,
   `cantMoreThan`, `canAsThough`, `mayPlay`, `mayCast`).
2. [`lean-v3-authoring-2-verb-and-clause-spine`](lean-v3-authoring-2-verb-and-clause-spine.md):
   Introduce `Instruction` templates for verbs and observations (`die`, `enter`,
   `leave`, `putInto`, `phaseOut`, `phaseIn`, `turnFaceUp`, `loseGame`,
   `winGame`, `castSpell`, `attack`, `block`, `dealDamage`, etc.) so `does`,
   `would`, `did`, and `cant` share one verb vocabulary without raw 6-argument
   `Instruction.move` or `.setStatus` / `.procedure` / `.conclude` calls.
3. [`lean-v3-authoring-3-nominals-and-predicates`](lean-v3-authoring-3-nominals-and-predicates.md):
   Replace hardcoded compound noun macros and raw `Primitives.*` constructor
   chains in `Cards/*.lean` with composable noun-phrase, determiner, and
   predicate modifiers (`youControl`, `controlledBy`, `opponentsControl`,
   `youOwn`, `ownedBy`, `cardIn`, `withKeyword`, `otherThan`, `which`, etc.) and
   clean top-level combinator templates.
4. [`lean-v3-authoring-4-declared-bodies`](lean-v3-authoring-4-declared-bodies.md):
   Consolidate `Macros.lean` and `Macros/Declared.lean` into
   `lean/Semantics/Primitives.lean`, `lean/Semantics/Templates.lean`, and
   `lean/Semantics/Definitions.lean` (with `Definition` bodies re-expressed
   through `Template` helpers), retire all remaining `v1`/`v2` holdover module
   and identifier names, and amend `docs/contexts/game-model/CONTEXT.md`.

## Proof

- All four sub-tickets are integrated in `docs/tickets/done/`.
- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
