---
needs: []
---
# Precompute legendary nicknames instead of streaming the card corpus at every start

**Priority: LOW.** Gains are minimal once `english-v3-test-binary-consolidation` lands (one process per package instead of about 94). User's priority call, 2026-10-07.

## Why

A startup profile on 2026-10-07 (instrumentation kept in workspace `english-v3-load-profile`, commit `mqwtzryt`, not integrated) measured english-v3 test-binary startup at about 5,200,000,000 ns, dominated by generic decoding compiled at opt-level 0 in `deckmaste_lexical_source` and `deckmaste_construction_core`. Those two dev-profile overrides are being landed separately (expected startup about 1,750,000,000 ns afterwards).

What remains here: `crates/deckmaste_lexical_source/src/lib.rs:53` calls into `src/card_names.rs:14-33`, which streams the entire 203,205,300-byte `oracle-cards.jsonl` (38,705 records) on every process start to collect 36,821 titles and 4,263 legendary units, ending with 1,806 nicknames. Measured about 3,700,000,000 ns at opt-level 0 and about 680,000,000 ns at opt-level 2, plus about 40,000,000 ns of title indexing. xtask parses the same file a second time for corpus selection.

## Goal

- Generate the nickname list once via `cargo xtask generate` into `data/gen/catalogs/` (already loaded in about 10,000,000 ns by `catalogs::CatalogSet::load`), keyed by the corpus file's hash so a stale artifact is detected.
- Startup no longer reads the JSONL.
- The lexicon is identical. Prove it: lexical inventory SHA and the full census identical face by face against the claim parent.
- Expected saving: about 700,000,000 ns per process start after the opt-level fix.

## Method

Standard constraints apply. The generated file is a build artifact under the existing generated-catalog conventions: check how `data/gen/catalogs` is produced and whether it is gitignored or tracked, and follow that. Add a test asserting the generated nicknames equal the streamed computation. Record timings before and after as integer ns.

## Out of scope

xtask's own corpus-selection parse. Possible follow-up: a name/id index.

## Landing record

Standard tiers. Plus: inventory SHA and census comparison against the claim parent, the staleness-detection test, and before/after timings (integer ns, host load, worker count).
