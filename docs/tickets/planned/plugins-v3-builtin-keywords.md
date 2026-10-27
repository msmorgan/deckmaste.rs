---
needs: [lean-v3-bundle-loader-and-dsl]
---
# Migrate `Definitions/Keywords.lean` into `plugins_v3/builtin/`

**Extract `lean/Semantics/Definitions/Keywords.lean` (~1,930 lines) into individual `.lean` files
under `plugins_v3/builtin/` and load them via `load_bundle "../plugins_v3/builtin"` at the bottom
of `lean/Semantics/Definitions.lean`.** Second of four atomic tickets migrating Lean declarations
into `plugins_v3/builtin/` ([`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §§4.4–4.5).
Standard constraints apply.

## Scope

1. **Extract every keyword ability `Definition` in `lean/Semantics/Definitions/Keywords.lean`:**
   - Write each keyword ability in its own loose file
     `plugins_v3/builtin/definitions/keyword_ability/<name>.lean` using the `keyword` `{ field := val, ... }`
     command macro (`lemma`, `spelling`, optional lexical fields `verbs`/`nouns`/`adjectives`/`determinatives`/`bound`, and `body`).
   - Self-referencing keywords (`flying` `[CR#702.9b]`, `suspend` `[CR#702.62a]`, etc.) reference their
     own identifier directly with zero `.named "..."` string literals.
   - Omit manual `inCaller`, `firstMention`/`laterMention`, and `Ability.keyword` boilerplate
     (handled automatically by `keyword`).

2. **Extract the 3 keyword-dependent templates in `Definitions/Keywords.lean`:**
   - Move `castWithSplice`, `hasProtectionFromEach`, and `typedCycling` into
     `plugins_v3/builtin/templates/{condition,predicate,ability}/<name>.lean` using the `template`
     command macro.

3. **Wire `load_bundle "../plugins_v3/builtin"` in `lean/Semantics/Definitions.lean`:**
   - Replace `import Semantics.Definitions.Keywords` at the bottom of `lean/Semantics/Definitions.lean`
     with `load_bundle "../plugins_v3/builtin"` and delete `lean/Semantics/Definitions/Keywords.lean`.

## Proof

- `lean/Semantics/Definitions/Keywords.lean` is deleted; all its keyword definitions and templates
  live as individual `.lean` files under `plugins_v3/builtin/` and elaborate via `load_bundle`.
- Zero raw `KeywordLabel.named` / `.named` string workarounds remain in keyword bodies.
- `cargo xtask lean check` (`lake build --wfail`) exits 0 with zero errors and zero warnings across
  `Semantics`, `SemanticsCards`, and `SemanticsProofs`.
