---
needs: [plugin-crate-split]
---
**Create `deckmaste_authoring`: fork the (freshly reshaped) core grammar
types into the authored-rules-grammar crate, taking the macro machinery
with them.** Design:
`docs/decisions/authoring-spelling-lowering.md`
(§1, §4, §11-12). This is the single source of truth every container is
written in — cards, tokens, and the `rules/` engine tables.

## Scope

- Duplicate the grammar type families from `deckmaste_core` as an exact
  mirror (variant names and shapes unchanged — canon must re-parse
  byte-identically).
- The macro layer moves here: `SupportsMacros` derives, `#[macro_ron(…)]`
  attributes (embed/flatten/default), the kind registry
  (`deckmaste_core::ron::kinds()` relocates), literal-reader plumbing.
- `deckmaste_core` is NOT touched in this ticket (stripping is
  `core-demacro`, after the repoint) — both crates are transiently
  macro-aware.
- Fork commit message states the provenance (blame lineage breaks here by
  design).

## Gates

Standard constraints apply. The new crate builds and round-trips canon RON
through its own reader byte-identically; existing pipelines still run on
core unchanged (zero behavior change).
