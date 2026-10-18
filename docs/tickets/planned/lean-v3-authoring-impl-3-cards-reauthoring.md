---
needs: [lean-v3-authoring-impl-2-templates-and-definitions]
---
# `lean-v3-authoring-impl-3-cards-reauthoring`

Re-author all 15 card corpus files in `lean/Semantics/Cards/*.lean` onto the curated
`Templates.lean` and `Definitions.lean` surface, eliminating all transitional `Primitives.*`
qualifications and completing the `lean-v3-authoring-surface` epic.

See [`lean-v3-authoring-surface`](../planned/lean-v3-authoring-surface.md) (§§1–6) for the
complete settled design specification.

## Why this is the third and final implementation ticket

With the Core AST and `Check/Phrase.lean` settled in
`lean-v3-authoring-impl-1-core-ast-and-checker` and `Macros.lean` + `Macros/Declared.lean`
consolidated into `Primitives.lean` + `Templates.lean` + `Definitions.lean` in
`lean-v3-authoring-impl-2-templates-and-definitions`, every card file in `lean/Semantics/Cards/*.lean`
can be re-authored in a single pass directly onto the final `Templates.lean` and
`Definitions.lean` surface without revisiting card files multiple times.

## Scope

### 1. Re-author all 15 `lean/Semantics/Cards/*.lean` files (§§1–6)

Re-author all card definitions across `lean/Semantics/Cards/*.lean` so that:
- Every card ability, trigger, cost, static effect, continuous modification, and instruction
  is expressed through the curated `Templates.lean` and `Definitions.lean` vocabulary.
- All transitional `Primitives.*` qualifications introduced during Tickets 1 and 2 are replaced
  by curated `Templates.lean` or `Definitions.lean` references.
- Zero `Primitives.*` references remain anywhere in `lean/Semantics/Cards/*.lean` (or in
  `lean/Semantics/Definitions.lean` if any slipped through Ticket 2).
- If any card requires a domain concept not yet covered by `Templates.lean` or `Definitions.lean`,
  promote that concept to a clean, reusable `semantic_template` in `Templates.lean` (or a named
  rule-grounded definition in `Definitions.lean`) rather than reaching for `Primitives.*`.

### 2. Extend `lean/Semantics/Check/Authoring.lean` to enforce `Cards/*.lean` hygiene (§4.1)

- Verify that `lean/Semantics/Check/Authoring.lean` checks every declaration across both
  `Definitions.lean` and `Cards/*.lean` (or add a structural check confirming zero `Primitives.*`
  constant references in `Cards/*.lean` and `Definitions.lean`).

## Verification

- `rg 'Primitives\.' lean/Semantics/Cards/ lean/Semantics/Definitions.lean` returns 0 matches.
- `cargo xtask lean check` passes with zero warnings or errors.
- `cargo xtask cite check --list-noncompliant` empty and `cargo xtask cite check` 0 stale.
