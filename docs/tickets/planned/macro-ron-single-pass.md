---
needs: []
---
# Parse each plugins_v2 declaration once at startup

**Priority: LOW.** Gains are minimal once `english-v3-test-binary-consolidation` lands (one process per package instead of about 94). User's priority call, 2026-10-07.

## Why

A startup profile on 2026-10-07 (instrumentation kept in workspace `english-v3-load-profile`, commit `mqwtzryt`, not integrated) measured english-v3 test-binary startup at about 5,200,000,000 ns, dominated by generic decoding compiled at opt-level 0 in `deckmaste_lexical_source` and `deckmaste_construction_core`. Those two dev-profile overrides are being landed separately (expected startup about 1,750,000,000 ns afterwards).

What remains here: `crates/deckmaste_construction_core/src/macro_def.rs:1906-1913` (`read_builtin_v2` -> `read_mapped`) parses each of the 899 declarations (277,804 bytes per pass) with `macro_ron::read_str`, builds a validation source map and normalizes. Then `crates/deckmaste_lexical_source/src/legacy/plugins.rs:30-39` re-reads every file from disk and runs `read_str` again only to obtain `definition.metadata`. The meta-macro set is built twice. Measured at opt-level 2: pass 1 about 228,000,000 ns, pass 2 about 196,000,000 ns (about 630 ns/byte).

## Goal

- Keep the parsed `Metadata` on `NormalizedDeclaration` (or expose the mapped read results to `plugins.rs`) so each declaration is parsed once.
- Build the meta-macro set once.
- Identical outputs. Prove it by comparing the resulting declaration set, metadata and lexicon inventory hash before and after; the full census is identical.
- Expected saving: about 190,000,000 ns per process start.

## Method

Standard constraints apply. Add a unit test that the single-pass metadata equals the former second-pass metadata for every builtin declaration. Record timings before and after as integer ns.

## Out of scope

A `build.rs` or checked-in expanded artifact for the 899 declarations, worth about another 220,000,000 ns. A possible later step, only after this ticket and `card-corpus-fast-lookup`.

## Landing record

Standard tiers. Plus: declaration-set, metadata and inventory-hash comparison before and after, the equivalence test, and before/after timings (integer ns, host load, worker count).
