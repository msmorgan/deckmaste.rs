---
needs: []
---
# Replace erased library errors with typed thiserror errors

Convert reusable library APIs that return `anyhow::Error` to concrete error
types derived with the workspace-pinned `thiserror`. Start with
`deckmaste_lexical_source::load_workspace`: callers currently have to inspect
messages to distinguish invalid declarations, unresolved owners or markers,
duplicate properties, decoding failures and filesystem failures.

User decision (2026-09-08): erased errors are limiting for crates like these;
callers need proper error types. The earlier `thiserror-adoption` ticket kept
`anyhow` at application boundaries. Preserve that distinction: CLI orchestration
may add `anyhow` context around typed library errors.

Audit the surviving library boundaries at implementation time, including
lexical-source loading, catalogs, snapshot data, spelling and migrations.
Convert cohesive error families per owning crate and update their callers.
Respect the existing crate-retirement decisions; do not refactor an obsolete
implementation merely to remove its dependency. Record each remaining
library-side `anyhow` use with its reason and surviving replacement owner.

Errors must expose actionable variants and relevant structured fields such as
paths, declaration identities and feature names. Preserve underlying I/O,
decoding and dependency errors through `source()`. Keep useful diagnostic
context; a string-only variant or an `Other(anyhow::Error)` wrapper does not
restore a typed boundary. Reuse existing concrete dependency errors where
appropriate instead of introducing a parallel classification.

Acceptance: callers can distinguish representative failure causes by matching
variants and fields without parsing `Display` text or downcasting an erased
error. Tests exercise real failing loader/validation paths and verify their
structured context and source chains; existing successful loading and CLI
diagnostics still work. Report the boundary inventory and remove unused
`anyhow` dependencies from converted libraries. Standard constraints apply.

## Boundary inventory

| Owner | Boundary after this change | Remaining `anyhow` and its disposition |
| --- | --- | --- |
| `deckmaste_lexical_source` | `LoadError`: I/O, RON decoding, vocabulary syntax, dependency declarations/catalogs, owners, markers, features, overrides and duplicate identities | None; dependency removed. |
| `deckmaste_catalogs` | `CatalogError`: generation, canonical and legacy loading/writing, directory comparison and type-line parsing; `OutputProblem` distinguishes unsafe output destinations | None; dependency removed. |
| `deckmaste_data` | `DataError` for filesystem access; existing `serde_json::Error` and `OracleCardReadError` for decoding/streaming | None; dependency removed. |
| `deckmaste_construction_core` | Existing typed `ReadError` plus `DeclarationReaderError` for embedded meta-macro parsing/registration; reader failures retain their sources | None. |
| `deckmaste_migrations` | `ExtractError`, `LayoutError`, `StubError`, `SnapshotError`; `todo_card::render` reuses `ron::Error` | `resolve.rs`, `graduate.rs`, and `parsers/{activated_ability,alternative_cost,condition,cost,effect,keyword_ability,loyalty_ability,mana_ability,macro_template,modal,replacement,spell_ability,static_ability,triggered_ability}.rs` still orchestrate the v1 plugin/template renderer. Convert their surviving interfaces when the legacy-render dependency is shed at cutover; the replacement owners are migrations and the independent English-v3 grammar. No erased wrapper added to a converted boundary. |
| `deckmaste_plugin` | Surviving decklist module exposes `DeckError` and `DeckParseError` with path, line, count and dependency sources; provenance already has no `anyhow` | `plugin.rs`, `validate.rs`, `idris_emit.rs` retain erased v1 plugin loading, validation and Idris orchestration errors until deletion. Replacement: semantics-v2 loading/lowering and the Lean workbench. |
| `deckmaste_legacy_render` | Obsolete renderer left intact | `fidelity.rs` and `template/index.rs` retain their existing errors until deletion; replacement realization belongs to English-v3. |
| `deckmaste_spelling` | Obsolete splice machinery left intact | `compile.rs`, `guard.rs`, `lexicon.rs`, `render.rs`, `unify.rs`, `witness.rs` retain their existing errors until retirement. Replacement lexical analysis/realization belongs to `deckmaste_lexical` and `deckmaste_english_v3`, which already expose concrete errors. |
| `deckmaste_tui` | Application orchestration (`app.rs`, `game.rs`); public entry returns `ExitCode` | Retained at the application boundary. |
| `xtask` | CLI orchestration adds `anyhow` context around concrete library errors | Retained as requested. |

The retirement boundary follows `docs/decisions/semantics-v2.md` §14 and
`docs/decisions/english-lexical-analysis.md`: migrations survives but sheds the
legacy renderer, while plugin loading, Idris and spelling splice machinery are
retired. All other workspace libraries have no runtime `anyhow` dependency.

## Landing record

Implementation and verification were completed in the named
`library-typed-errors` workspace, change `kmokxsrv`.

PROVE: no declaration, lexical identity, construction, licensing guard, or
successful loading behavior was changed. Existing successful loading,
realization and reanalysis tests are retained. Error tests match variants and
fields through actual loader/validation failures and check concrete source
chains. No tests removed or newly ignored: restored 0, re-spelled 12, added 5,
removed 0; existing ignore attributes and blockers are unchanged.

DISCLOSE: no newly covered identities or constructions, selection changes,
terminology gaps, or ruling contradictions. Additions beyond the initial loader
are the catalog/data boundaries, embedded declaration-reader failures,
surviving extraction/stub/snapshot/RON-output APIs, and decklist APIs, justified
by the boundary audit above. Per the user's follow-up, four builtin declaration
tests now validate authored spellings, grammar, morphology, signatures and
semantic bodies without comparing the downloaded inventories to fixed counts
or exact sets. Controlled catalog-parser fixtures still assert exact results.
The pre-change catalog generator independently produced 23 artifact types from
the current snapshot too; the assertion expecting 22 was stale. No production
catalog input or parser was altered to fit the old count. Eight English-v2
compile-fail snapshots gain only the installed compiler's new namespace note;
each preserves the same private-constructor rejection, error code and help.
No compiler, syntax or generated construction behavior was changed. No new registry or grammar implementation.

REPORT: corpus coverage and performance figures were not remeasured; this
change does not alter grammar or consumed declarations.

Verification on `kmokxsrv`: `cargo xtask gate --changed --run` exited 0 and
selected:

```sh
cargo test -p deckmaste_data -p deckmaste_catalogs -p deckmaste_construction_core -p deckmaste_english -p deckmaste_lexical_source -p deckmaste_semantics -p deckmaste_lowering -p deckmaste_plugin -p deckmaste_engine -p deckmaste_legacy_render -p deckmaste_migrations -p deckmaste_noncanon -p deckmaste_spelling -p deckmaste_tui -p deckmaste -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
```

Across the 101 Cargo suite results: 5,169 passed, 0 failed, 78 existing ignored;
the compile-fail suite additionally exercised all 96 negative fixtures.
`cargo fmt --all --check` passed. Clippy passed for all targets of the migrated
library crates and for construction-core tests, with existing warnings only.
The four re-spelled declaration suites also passed separately. The independent
pre-change generator reproduced the artifact-count mismatch before the user's
follow-up changed that test contract; it was not an error-migration regression.
An initial broader `cargo check --workspace` could not fetch an uncached LSP
support dependency; the required changed-crate gate subsequently completed.

Provisioning initially failed because the coordinator's cached xtask binary
embedded a removed workspace path. Rebuilt xtask in the feature workspace,
used an ignored real `data/` directory with links to shared inputs (the hook's
`data` symlink would otherwise be tracked), generated missing canonical
catalogs locally, and regenerated wizards. These fixture repairs are untracked.
