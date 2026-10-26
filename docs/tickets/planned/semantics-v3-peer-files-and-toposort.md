---
needs: [semantics-v3-dsl-and-plugins]
---
# Peer loose files for Templates and Definitions with topological sort into Lean

**Author every `Template` and `Definition` as a loose peer file in `plugins_v3/`
and topologically sort their declaration dependency graph to emit them in
proving order for Lean (`cargo xtask lean check`).** Standard constraints apply.

## Problem

In the hand-layered Lean workbench (`lean/Semantics/`), authored declarations
are partitioned across a fixed module import chain:

`Templates.lean` → `Definitions/Phrasings.lean` → `Definitions/Actions.lean` → `Definitions/Keywords.lean` → `Definitions/CardTypeRules.lean`

Because Lean requires every referenced symbol to be declared earlier in the
import graph (or earlier in the same file), this fixed module boundary gets in
the way of ergonomic authoring:

1. **Templates split across four modules by what they call:** A `Template` that
   composes over a CR §701 `Definition` ([CR#701.1]) or CR §702 `Definition`
   ([CR#702.1]) cannot live in `Templates.lean`. That forced a separate
   `Definitions/Phrasings.lean` module plus `/-! ## Templates over Definitions -/`
   sections inside `Definitions/Actions.lean` (`destroyCantRegenerate`,
   `exileUntilLeaves`, `mayCastExiled`, `controllerSacrifices`, `doesntUntap`,
   `mayDeclineUntap`, `putIntoLibraryNthFromTop`, `takeExtraTurnAfterThisOne`)
   and `Definitions/Keywords.lean` (`castWithSplice`, `hasProtectionFromEach`,
   `typedCycling`).
2. **Action Definitions unable to reference Keyword Definitions:** Because
   `Definitions/Actions.lean` is imported by `Definitions/Keywords.lean`, a
   keyword action whose CR rule references a keyword ability cannot refer to
   that keyword's `Definition` and must drop down to a raw `.named` string:
   - `earthbend` (`Definitions/Actions.lean:445`, [CR#701.66a], *"Target land
     you control becomes a 0/0 creature with haste…"*) writes
     `gains (.named "haste") ...` instead of `gains haste ...`.
   - `timeTravel` (`Definitions/Actions.lean:622`, [CR#701.56a], *"each
     suspended card you own…"*) writes `withKeyword (.named "suspend")` instead
     of `withKeyword suspend`.
3. **Manual intra-file declaration ordering:** Authors must manually order
   helper templates and definitions within each file before any caller that uses
   them.

## Design

1. **Loose peer files (`Template(...)` and `Definition(...)`):**
   - Every `Template` and every `Definition` (CR §701 keyword actions
     [CR#701.1], CR §702 keyword abilities [CR#702.1], and
     `RegistryDefinition`s for counters, subtypes, designations, relations,
     card types, and flavor words) lives as a loose file in `plugins_v3/`.
   - All `Template` and `Definition` files are peers in one flat declaration
     namespace. A file's role is determined solely by its top-level wrapper
     (`Template(...)` vs `Definition(...)`, `docs/decisions/semantics-v3.md`
     §2), never by directory path or module layer:
     - Any `Template` may reference any `Definition` or `Template`.
     - Any `Definition` (including an action `Definition` such as `earthbend` or
       `timeTravel`) may reference any other `Definition` (such as `haste` or
       `suspend`) or `Template`.
2. **Declaration-level dependency graph and deterministic topological sort:**
   - Each loose file defines its primary declaration identifier (plus any
     companion term macro registered alongside a `Definition`, such as a
     keyword ability's `KeywordLabel` constant or a registry declaration's
     term/label constant) and references a set of free declaration identifiers
     in its parameters and body.
   - The `xtask` / `deckmaste_semantics_v3` Lean emitter constructs the
     directed dependency graph across all peer `Template` and `Definition`
     declarations:
     - An edge `A → B` exists whenever declaration `B` references declaration
       `A` (or a companion symbol provided by `A`).
     - **Cycle refusal:** If the dependency graph contains a cycle, the loader /
       emitter refuses with a diagnostic naming the exact cycle of declarations
       and their file paths.
     - **Deterministic topological order:** Declarations are sorted via Kahn's
       algorithm using a lexicographically ordered ready queue (`BTreeSet` by
       declaration name) so the emitted Lean declaration order is deterministic
       and independent of filesystem traversal order.
   - The emitter writes the topologically sorted `semantic_template` and
     `semantic_definition` declarations into Lean so `cargo xtask lean check`
     (`lake build --wfail`) elaborates and proves every declaration, rule row,
     card, and theorem without hand-maintained module layering.
3. **Retire the hand-layered Lean declaration modules and cross-layer workarounds:**
   - Replace the hand-maintained `lean/Semantics/Templates.lean`,
     `lean/Semantics/Definitions/Phrasings.lean`,
     `lean/Semantics/Definitions/Actions.lean`,
     `lean/Semantics/Definitions/Keywords.lean`,
     `lean/Semantics/Definitions/CardTypeRules.lean`,
     `lean/Semantics/Definitions/SubtypeRules.lean`, and
     `lean/Semantics/Definitions/DesignationRules.lean` layering with the
     topologically sorted output emitted from `plugins_v3/`.
   - Re-spell `earthbend` ([CR#701.66a]) to reference `haste` directly and
     `timeTravel` ([CR#701.56a]) to reference `suspend` directly instead of raw
     `.named "..."` literals.

## Proof

- Every `Template` and `Definition` in `plugins_v3/` loads as a peer loose file
  with no module-layer partition between templates, phrasings, actions,
  keywords, and registry definitions.
- Dependency cycle detection is covered by a unit test that constructs a
  two-declaration cycle and asserts the cycle diagnostic.
- `earthbend` references `haste` and `timeTravel` references `suspend` with zero
  raw `.named` workarounds.
- `cargo xtask lean check` (`lake build --wfail`),
  `cargo test -p deckmaste_semantics_v3 -p xtask`, and
  `cargo clippy -p deckmaste_semantics_v3 -p xtask --tests -- -D warnings` pass.
