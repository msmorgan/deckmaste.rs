---
needs: [english-v4-stratum-7-discourse-and-long-tail, english-v4-syntax-engine-obligations, english-v4-oracle-editorial-corrections]
---
# Cutover 1: Switch to `english_v4` and delete `v3` and `plugins_v2/builtin`

Execute Cutover 1 of [`docs/decisions/english-v4.md`](../../decisions/english-v4.md)
(§7):

1. **Switch Active Consumers:** Point `crates/xtask` and workspace consumers
   from `deckmaste_english_v3` / `deckmaste_lexical_source` to
   `deckmaste_english_v4` / `deckmaste_lexical_source_v4`.
2. **Verify Migrated Test Obligations & Delete Obsoleted `v3` and `plugins_v2/builtin` Artifacts:**
   - Verify that every still-wanted positive and negative grammatical obligation
     identified from `crates/deckmaste_english_v3/tests/suite/` is asserted in
     the `deckmaste_english_v4` judgment suite, and report the retired `v3`
     wave test files (those pinning `v3` AST constructor shapes, policing flags,
     or exact reading counts) with their retirement reasons. The per-test
     record is `docs/english-v3-test-audit.md`; its `lexical-v3` gaps must be
     closed or ruled out before deletion.
   - Delete `crates/deckmaste_english_v3`, `crates/deckmaste_construction_v3`,
     `crates/deckmaste_construction_v3_core`, `crates/deckmaste_lexical_source`,
     `crates/deckmaste_macro_def` (once no crate depends on it), and
     `plugins_v2/builtin/`.
3. **Glossary Audit:** Verify that [`docs/contexts/oracle-english/CONTEXT.md`](../../contexts/oracle-english/CONTEXT.md)
   reflects every public grammatical term in `deckmaste_english_v4` with zero
   leftover `v3` terms.

## Gate

- `cargo test --workspace` and `cargo clippy --workspace --tests -- -D warnings`
  pass with `v3` and `plugins_v2/builtin/` deleted.
- `cargo xtask cite check --list-noncompliant` and `cargo xtask cite check` pass
  with 0 stale citations.

Standard constraints apply.
