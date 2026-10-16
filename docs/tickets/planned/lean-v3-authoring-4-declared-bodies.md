---
needs: [lean-v3-authoring-3-nominals-and-predicates]
---
# Consolidate Templates and Definitions and retire Macros and v1/v2 holdover names

**Retire the `Semantics.Macros` hierarchy, the `Macros.lean` vs.
`Macros/Declared.lean` split, and all remaining `v1`/`v2` holdover module,
attribute, and type-alias names across `lean/Semantics/` and
`docs/contexts/game-model/CONTEXT.md`, consolidating into
`lean/Semantics/Primitives.lean`, `lean/Semantics/Templates.lean`, and
`lean/Semantics/Definitions.lean` with `Definition` bodies expressed through
`Template` helpers.** Sub-ticket 4 of
[`lean-v3-authoring-surface`](lean-v3-authoring-surface.md). Absorbs the
`lean-macros-cutover` and `plugins-v2-bodies-through-helpers` residues formerly
parked in `semantics-v3-dsl-and-plugins`. Standard constraints apply.

## Context & Problem

In `v2`, `lean/Semantics/Macros/Declared.lean` was generated from `plugins_v2/`
via `macro_ron` sample probing as 6,933 lines of single-line fully-qualified raw
`Semantics.Instruction.*` constructor dumps alongside handwritten
`lean/Semantics/Macros.lean`. In `v3` (`docs/decisions/semantics-v3.md` §0, §2),
Lean is the first-class authority and the undifferentiated "Semantic Macro"
concept is replaced by the explicit two-tier split between **`Definition`** and
**`Template`**. Holding onto `Semantics.Macros`, `Declared.lean`,
`semantic_macro`, `MacroParameters`, `Phrase.lean` / `Triggers.lean` shims,
`Binding` / `Bindings`, and `Payload.gap` preserves obsolete `v1`/`v2`
scaffolding.

## Scope

1. **Retire `Semantics.Macros` and the `Macros.lean` / `Declared.lean` split:**
   - Replace `lean/Semantics/Macros.lean`, `lean/Semantics/Macros/Primitives.lean`,
     `lean/Semantics/Macros/Declared.lean`,
     `lean/Semantics/Macros/CharacteristicInputs.lean`, and
     `lean/Semantics/Macros/Checks.lean` with:
     - `lean/Semantics/Primitives.lean` (`Semantics.Primitives` namespace for
       `declare_semantic_primitives` + primitive constructor aliases and
       `CharacteristicInputs`)
     - `lean/Semantics/Templates.lean` (the single authoritative module of
       `semantic_template` phrasing helpers)
     - `lean/Semantics/Definitions.lean` (the single authoritative module of
       `semantic_definition` rules-defined vocabulary: CR `[CR#701.1]` keyword
       actions, CR `[CR#702.1]` keyword abilities, and registry definitions)
     - `lean/Semantics/Proofs/DefinitionsAndTemplates.lean` (replacing
       `Macros/Checks.lean`)
   - Reconcile the 15 signature drifts (`amass`, `modular`, `bushido`, `renown`,
     `fateseal`, `move`, `create`, `createTappedToken`, `creatureToken[Of]`,
     `returnToBattlefield`, `establishFor`, `forEach`, `companion`,
     `Actor.mayCastFrom`), 2 body drifts (`gainControl`, `levelUp`), and
     helper consolidations (`lookAndSortSlice` retiring `lookAndSort`,
     `lookAndSortInto`, `lookedCards`) from `lean-macros-cutover`.
   - Re-express `Definition` bodies in `lean/Semantics/Definitions.lean` through
     `Template` helpers from `lean/Semantics/Templates.lean` (absorbing
     `plugins-v2-bodies-through-helpers`, including eliding `Printed(Param(kind))`
     in `entersWithCounters`, `entersWithAdditionalCounters`,
     `entersWithFewerCounters`, and `exileWithCounters`).
2. **Retire `Macro*` machinery in `Authoring.lean` and `MacroParameters.lean`:**
   - Rename `lean/Semantics/MacroParameters.lean` (`MacroExpansion`,
     `MacroSplice`) and `lean/Semantics/Proofs/MacroParameters.lean` to
     `lean/Semantics/CallerScope.lean` (`CallerScope`, `CallerSplice`) and
     `lean/Semantics/Proofs/CallerScope.lean`.
   - In `lean/Semantics/Authoring.lean`, retire the raw `semantic_macro` command
     and `macroAttribute` so only `semantic_definition` and `semantic_template`
     exist; rename `MacroContext` -> `AuthoringContext`,
     `contextualMacroAttribute` -> `contextualAuthoringAttribute`,
     `macro_splice_type%` -> `caller_splice_type%`, and
     `Form.onlyMacros` / `Forms.onlyMacros` -> `Form.onlyAuthored` /
     `Forms.onlyAuthored`.
3. **Retire remaining `v2` file shims and type/constructor holdover names:**
   - Delete the empty 12-line comment-only re-export shims
     `lean/Semantics/Phrase.lean` and `lean/Semantics/Triggers.lean` (and fold
     the 1-function `Check/Triggers.lean`, `Check/EventContext.lean`, and
     `Check/PhraseRules.lean` micro-files into `Check/Phrase.lean`).
   - Delete `abbrev Binding := Referent` and `abbrev Bindings := DRS` in
     `lean/Semantics/Check/Words.lean` and update the remaining uses in
     `Cards/` and `Proofs/` to `Referent` and `DRS`.
   - Rename `Payload.gap`, `Referent.isGap`, `gapB`, `countGaps`, and
     `Refusal.gapInScope` (which now exclusively represent `Amount.theDifference`)
     to `Payload.difference`, `Referent.isDifference`, `differenceB`,
     `countDifferences`, and `Refusal.differenceInScope`.
4. **Amend Game Model Glossary (`docs/contexts/game-model/CONTEXT.md`):**
   - Replace the legacy **Semantic Macro** entry (`CONTEXT.md:569-575`) with
     **Definition** and **Template** (`[CR#701.1,702.1]`), listing
     `_Avoid_: Semantic Macro, macro`.

## Proof

- `lean/Semantics/Macros.lean`, `lean/Semantics/Macros/`,
  `lean/Semantics/MacroParameters.lean`, `lean/Semantics/Phrase.lean`, and
  `lean/Semantics/Triggers.lean` no longer exist.
- `grep -rn "semantic_macro\|MacroContext\|MacroExpansion\|MacroSplice\|abbrev Binding" lean/Semantics/`
  returns zero matches.
- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
