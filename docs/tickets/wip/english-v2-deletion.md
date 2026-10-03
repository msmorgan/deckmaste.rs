---
needs: []
---
# Retire the superseded English parser and its exclusive dependencies

Completed under the user's 2026-10-03 instruction to delete the parser and
things it alone depended on. This expands the original directory-deletion
scope to retire its exclusive `deckmaste_construction` proc macro and tests,
rather than retaining that compiler with a vendored parser fixture.
`deckmaste_construction_core` remains a shared declaration reader used by the
lexical-source adapter and semantics tooling.

## Landing record

Removed the parser directory, exclusive proc macro, obsolete coverage lock,
and root workspace/exclusion entries. Cargo.lock no longer contains either
retired package. Active source readers and instructions refer to surviving
owners. The rewrite ADR is explicitly superseded; old ADR text and done tickets
remain historical evidence.

Moved 43 vocabulary/morphology declarations into
`deckmaste_lexical_source/lexicon/vocabulary.rs` and the verb inventory into
`lexicon/verbs.ron`. The source reader parses these as declaration data without
compiling the retired grammar. A temporary comparison loaded both the complete
former declaration file and the extracted vocabulary under the relocated paths:
all 34,866 normalized lexical declarations were identical. Source provenance
now names the lexical-source inventory. Old construction-literal/affix audit
messages are retired with the removed grammar; no lexical entry or grammatical
Reading is filtered. No v3 runtime or grammar declaration changed. No corpus
coverage gain or cutover-parity claim is made.

Restored: 0; added persistent tests: 0; newly ignored tests: 0. The existing
changed-path gate tests are updated for the surviving dependency graph.
Removed: 454 test functions/attributes in the already-excluded parser and 55
in its exclusive proc macro. Each retires its own deleted subject, rather than
a behavior still owned by a surviving crate. Retired tests belong exclusively to the removed parser/proc-macro subjects;
the original requirement to preserve that proc macro's runtime fixtures is
superseded by the user's explicit dependency retirement. The shared declaration
reader and v3 suites remain intact.

Validation and environmental limits are recorded below. The full corpus was
not rerun: lexical equality and the unchanged v3 implementation establish the
scope of this removal, and the existing coverage gap remains owned by v3 work.

Checks: `cargo check -p deckmaste_lexical_source -p xtask --offline` passed.
`cargo test -p deckmaste_lexical_source -p deckmaste_english_v3 --offline`:
69 passed, zero failed or ignored. `cargo test -p deckmaste_construction_core
-p deckmaste_semantics_v2 -p xtask --offline --no-fail-fast`: 855 passed,
31 existing ignored, one failed. The failing
`builtin_v2_noncreature_subtypes_match_each_supported_catalog_and_category`
expects 22 artifact types; catalogs regenerated from the current local CR
contain 23. Its source is byte-identical to the parent revision. No test or
catalog assertion was weakened.

The metadata-derived full gate is `cargo test -p deckmaste
-p deckmaste_construction_core -p deckmaste_english
-p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_spelling
-p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`. It cannot run
completely here: uncached dependencies (including autocfg/rand_chacha) cannot
be downloaded because static.crates.io is unreachable.

Changed Rust files pass nightly rustfmt; the updated shell guard passes syntax
and JSON-output checks. Strict clippy reaches an unchanged warning in
`macro_ron/src/set.rs:733` (`needless_borrows_for_generic_args`); that source
also matches the parent byte-for-byte. Citation checking finds zero
noncompliant sites and 14 preexisting stale citations in unchanged files.
These checks are disclosed as limitations, not claimed green.
