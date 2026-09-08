---
needs: [english-v3-packed-chart]
---
# Build the fresh bidirectional construction compiler

Create `deckmaste_construction_v3_core` as a normal, directly testable compiler
crate and `deckmaste_construction_v3` as its thin proc-macro shell. They do not
depend on the v2 compiler or parser. Treat the old compiler as a quarry: port
declaration syntax, diagnostics, normalization and generation patterns only
after each piece fits the v3 intermediate representation.

One construction declaration must generate the Category/AST types, chart
Productions and lexical terminal constraints, grammatical-summary/admission
logic, lazy Reading materializers, Realization, and structural plus lexical
traversal. All generated projections come from one validated compiler IR. The
compiler may depend on `deckmaste_lexical_model`; it must not load lexical
catalogs, generate morphology, scan input, construct full AST products during
recognition, store redundant token ownership, or select one Reading from a tie.

Reject declarations whose parser and renderer surfaces diverge, whose required
feature dependency cannot reach admission, whose materializer loses a lexical
identity or spelling variant, or whose recursive/nullable shape cannot be
packed safely. Diagnostics identify the declaration and generated obligation,
without exposing proc-macro panics.

Acceptance compiles synthetic constructions through the public proc macro and
checks every emitted projection. For every finite synthetic Reading, check
byte-exact parse/realize roundtrip and independently constructed-value
realize/parse roundtrip before the generated slice relies on the compiler.
Include alternative surface forms, optional and repeated constituents,
nullable structure, feature agreement, lexical frames, distinct same-surface
Readings, and duplicate-derivation packing. No v2 compatibility aliases or
generated scanner callback are introduced. Standard constraints apply.

## Landing record

Measured on change `syvlsqqmusvq`, with the unchanged production lock's
`covered` count 20,254.

### PROVE

Added the independent `deckmaste_construction_v3_core` compiler and the thin
`deckmaste_construction_v3` proc macro. The compiler's only project dependency
is the data-only lexical model. The proc macro's only production dependency is
the compiler core; its runtime/lexical dependencies are compiled-consumer test
fixtures. There are no v2 dependency edges, aliases or generated scanner
callbacks. No production grammar, lexical declaration, parser, catalog,
coverage lock or production consumer changed. Consequently there are no newly
covered or lost production identities to route, and no production analysis was
replaced by a synthetic witness.

The validated IR generates Category and AST enums, lexical/chart Productions,
finite feature-register admission, lazy materializers, direct checked-value
admission, realization and both traversals. Every form retains its fields
exactly once. Category interfaces delimit all parent-visible features;
equations retain correlation and release checking-only registers after their
last use. Missing features reject, and materialization never decides validity.
Lexical values retain identity, form, feature bundle, spelling variant,
capitalization and selected lexical uses. Their declaration provenance remains
recoverable from the same immutable Lexicon. ASTs store no token positions,
source slices or copied literal spelling. Nullable analysis rejects all
zero-consumption cycles, including those introduced by optional/repeated
normalization. The compiler README states the finite-state and packing argument.

The public macro's compiled consumers independently construct and compare
complete values, rather than recovering their expected ASTs from the parser.
They cover both roundtrip laws for alternative forms and orders, optional and
repeated constituents, nullable alternatives, consuming recursion, correlated
agreement, frames, distribution-feature forwarding, lexical spellings and
uses, and same-surface structural alternatives. Negative witnesses exclude
crossed agreement, missing features, wrong frames, wrong child Categories,
invalid surface indexes and unlicensed lexical boundaries. Independent finite
families include binary sequences through length four, all nine nullable
optional pairs, nullable sequences through length four, and every binary
attachment through five leaves (14 structures at five leaves). Every enumerated
Reading realizes byte-exactly; whole-value equality and explicit traversal
assertions preserve structure and lexical order. No internal materialization
failure or cyclic derivation occurred. Multiple correct Readings remain
successful; no preference or tie-breaking mechanism exists.

Compile-time negative tests reject field loss, duplicate ownership, changed
cardinality, missing surfaces, inaccessible/missing feature exports, crossed
feature domains, contradictory requirements, malformed syntax/signatures and
unsafe recursion. Two additional public-macro compile-fail doctests exercise
lossy forms and nullable cycles. No proc-macro panic is used as a diagnostic.
Admission tests and generated code use declared features only; no lexeme,
construction-identity or input-word licensing guard was added.

### DISCLOSE

No production Reading census or selection census changed. The new fixtures
retain homographs and distinct structures, while equal duplicate derivations
produce one Reading. The 64-element ambiguity witness requests just two
Readings: recognition and iterator creation construct zero ASTs. Its small
forest represents at least `2^64` distinct lexical sequences.

Deviations and additions:

- Added workspace/lock entries, compiler documentation and 24 compiled synthetic
  Construction declarations, together with diagnostic-only invalid declarations.
  No authored production Construction was added or removed. The syntax is fresh
  and compiles to the v3 interfaces directly; no v2 front-end adapter was added.
- Added a final independent lexical-context validation to constructed-value
  admission and realization. Individual free words can otherwise realize at
  unlicensed joined boundaries. The lexical engine checks emitted occurrences
  against the completed surface; it does not parse the grammar. Temporary
  realization offsets are discarded and never enter the AST or chart summaries.
  This costs one independent lexical analysis of the output and is documented.
- Added contextual-boundary, count/mass, numeral, nullable-helper and recursive
  attachment witnesses to strengthen the ticket's roundtrip and packing evidence.
  Added invalid-value and public compile-fail tests to cover the API boundaries.
  A full-frame signature witness additionally exercises every shared frame-item
  shape, item order and canonical duplicate-frame indexing.
- Tests: restored 0, re-spelled 0, ignored 0, added 19, removed 0. These are
  13 compiled-consumer tests, four compiler diagnostic tests and two doctests.
  No existing test or assertion was weakened. A discovered gap in the new
  renderer's lexical-context check was repaired before landing; no existing
  production regression or recorded-ruling contradiction was found.

There is no unresolved STOP or glossary gap. No CR citation changed. Production
licensing-checker totals and environment-loader errors are unchanged and were
not remeasured: their code and consumed declarations are outside this change's
reverse-dependency closure.

### REPORT

All counts here are stamped with `syvlsqqmusvq`, lock covered 20,254. The v3
production Construction count remains zero; the test consumers compile 24
synthetic Constructions. Production homograph and form-literal/vocabulary
inventories were not remeasured. Synthetic homographs are `light` and `x`
(two adjective identities each), `a`/`b` (correlated pronoun feature readings),
`pre` (free/prefix identities), and noun count/mass readings where licensed.
Retained alternatives include `café`/`cafés`/`caféses` and initial capitals,
five numeral notations, empty structures and recursive attachment. Fixture
literals contain only separators, punctuation or empty strings, so their
form-literal/vocabulary overlap inventory is empty.

| Generated repetition length | Lexical alternatives / projections | Items / intermediate nodes | Completed nodes | Families | Completion work | Requested / returned Readings | Builds |
|---|---:|---:|---:|---:|---:|---:|---:|
| 4 | 8 / 8 | 41 | 18 | 71 | 17 | 2 / 2 | 14 |
| 16 | 32 / 32 | 149 | 66 | 263 | 65 | 2 / 2 | 38 |
| 64 | 128 / 128 | 581 | 258 | 1,031 | 257 | 2 / 2 | 134 |

Families count both intermediate edges and completed families. The four-element
exhaustive census separately traverses 256 derivations and retains exactly 16
Readings, reporting 240 duplicates. This explicitly exposes enumeration costs;
the compiler does not claim polynomial delay between distinct Readings.

After a no-op Kata refresh, `cargo xtask gate --changed --clippy --run` derived
and passed:

```sh
cargo test -p deckmaste -p deckmaste_construction_v3_core -p deckmaste_construction_v3
cargo clippy -p deckmaste -p deckmaste_construction_v3_core -p deckmaste_construction_v3 --all-targets -- -D warnings
```

All 19 tests passed with zero ignores. The root package has no tests; the gate
includes it because its workspace manifest changed. Nightly formatting checks
passed for both new crates and the emitted runtime template. `kata kanban check`
reported no duplicate items, cycles or dangling dependencies.

Performance advisory: the single-worker repetition census took 10 ms according
to the test harness; the command including incremental compilation took
1,575 ms, of which Cargo reported 1,550 ms for the test build. The sampled host
load averages were 17.10 / 9.63 / 6.28. This is synthetic compiler/chart evidence,
not a production coverage pass. The 16.26 s quiet-host coverage ceiling and
accepted-byte thread-CPU telemetry in ns/B were not remeasured because this
compiler has no production corpus consumer. Gate and metrics evidence were
captured in `/tmp/construction-v3-gate.log` and
`/tmp/construction-v3-metrics.log`; the compiler README gives reproducible test
commands without relying on those files.
