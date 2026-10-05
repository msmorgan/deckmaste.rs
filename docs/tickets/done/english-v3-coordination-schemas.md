---
needs: []
---
# Share coordination constructions through schemas

Replace category-specific coordination constructor duplication with genuine
shared bidirectional schemas and category instances. Shared syntax, AST identity,
realization and traversal must be declared once; instances retain category-specific
agreement, feature propagation and selected-frame constraints. Compiled chart
specializations must not become hundreds of hidden named AST constructors.

Migrate ordinary, Oxford serial and correlative coordination and their list
helpers; examine repeated selected-head/shared-complement shapes using the same
schema facility. Preserve nested grouping, source spelling, lexical ownership,
all grammatical readings and both roundtrip laws. Re-spell affected independent
AST fixtures with their original witnesses and assertions; remove no tests.
Standard constraints apply. Full corpus verification uses 24 workers against
`/tmp/voice-fixes-after.json` (6,186 covered faces, 28,799 Readings). Record actual
schema/constructor/instance/rule counts separately. No standalone audit doc.

User refinement: 250 Constructions and 2,800 well-formatted declaration lines are
ultimate ceilings for full card breadth, not today's allowance. Leave substantial
headroom now; fixes/refinements to existing Constructions are the normal path,
with a new Construction requiring a clear grammatical distinction. Share repeated
constraint policies as well as syntax. The existing Construction glossary entry
already defines a grammatical schema; no per-schema glossary entries are needed.


## Landing record

### PROVE

Genuine shared schemas own one named Reading variant, form, cost, realization
and traversal implementation. Explicit category tags retain grammatical identity.
Typed family rows bind concrete child categories and named equation policies;
Self aliases and trailing defaults substitute only explicitly authored contracts.
Every expanded instance is checked independently. No category-specific named
variants are hidden behind authoring macros.

328 former names map to 48 schemas; 134 ordinary constructions remain. The
independent contract comparison finds 294 exact typed form/equation matches and
34 intentional equivalent translations: 32 selected lexical heads use the
16-row partial frame-signature table, and additive/alternative noun phrases use
existing coordinator-sensitive agreement tables. All 134 ordinary constructions
retain their original contracts. No forms or constraints were dropped. Selected
frames, lexical owners, Oxford serial cardinality, grouping, extraction,
placement and participial selection remain explicit.

The final complete corpus reconciles all 32,828 source identities with the
voice-fix baseline. Every individual face retains its Reading count; zero
coverage losses or gains. Covered 6,186; unique 4,246; multiple 1,940. All 28,799
Readings are validated with byte-exact roundtrip, checked AST admission,
construction traversal and lexical traversal/ownership. Zero issues, failures,
limited enumerations or internal errors. Source and lexical inventory hashes are
unchanged. No word-, card-, construction- or lexical-owner-naming guard is added.

Scratch evidence: `/tmp/coordination-schemas-refreshed.json`,
`/tmp/coordination-schemas-refreshed-reconciliation.json`,
`/tmp/coordination-final-mapping.json` and `/tmp/schema-contract-review.json`.
The mapping is a bijective re-spelling of grammatical AST identity, not a
selection or Reading-pruning policy. Sample ordering/fingerprints can change
with variant names and category tags; complete enumeration is preserved.

### DISCLOSE

Tests: restored 0, re-spelled 44 English integration function bodies in 22 files,
ignored 0, added 10 compiler tests, removed 0. All 126 English integration
functions remain; migrated helpers also retain their consumers. Re-spelling
covers 184 direct AST values/patterns and 93 helper invocations. All positive
Oracle witnesses and expected realizations remain. Compiler fixtures independently
check schema identity, category correlation, agreement, cost, traversal,
roundtrip and malformed bindings/rows/defaults. No new positive nonsense passage
is added to English fixtures.

One stale negative expectation was found by the first gate: the retired
SecondarySelectedObjectHead encoded an Object frame in its variant name, whereas
a generic SelectedVerbHead legitimately admits the same predicative frame.
The original lexical word and frame-1 witness now undergo rejection in a
TransitivePredicate object-selection context. The standalone selected head
remains valid. No assertion subject is discarded; the consuming host tests the
actual signature restriction. The final gate checks this correction.

Deviations and additions: shared SelectedVerbHead and 16 shared-complement schemas
replace finite/secondary duplication; 18 ordinary finite/secondary predicate
pairs also share schemas. OvertComplement combines the three overt selected-form
wrappers, and CasePhrase combines nominative/accusative projections with explicit
case policies. These extend the ticket's primary coordination migration without
adding grammatical breadth. Six new compiler validation tests and four independent
schema integration tests exercise the new facility. Existing compiler README and
lexical-analysis ADR are amended; no standalone audit/design document or
per-construction glossary entry is added. Construction already denotes a schema
in the owning glossary; no glossary gap was needed.

The ultimate finished-grammar limits are 250 constructions and 2,800 declaration
lines. This landing removes AST proliferation, but 2,400 lines still leaves only
400 lines of headroom. Further declaration economy is owed to
`english-v3-construction-feature-economy`; it must refine feature propagation and
justify further grammatical sharing rather than compress formatting or merge
unrelated functions. The user-requested eventual caps are not treated as today's
budget. Existing card-coverage residuals are unchanged.

### REPORT

Measured refreshed change `krzqrwxllknpmmzrwrvztnkllvnonmpw`, covered 6,186, 24 workers,
unlimited Readings, one sample per face. Source SHA-256
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`; lexical SHA-256
`3d56de49f2e3203130fc710231516aa3dfa67247a944ef330c1f79395c28cae1`.
Baseline `/tmp/voice-fixes-after.json`, measured change
`oooowkkqlpstpvzksvttnulmxylzyltz`, covered 6,186. No legacy coverage-lock or
arbitrated-selection claim applies to this complete v3 census.

Named constructions 462 → 182: 134 ordinary constructions + 48 schemas.
48 typed family declarations expand to 297 category instances; 55 feature
policies are all used. The final grammar emits 483 chart Productions, counted
through the public Grammar trait from the latest compiled artifact. The corpus
exercises 123 construction kinds. Whole declaration file 3,724 → 2,400 physical
lines, maximum width 100; no overlong lines or stacked separate statements.
Lexical homograph and form-literal/vocabulary overlap inventories are unchanged
from the stamped voice-fix/type-macro baselines; no morphology or lexical source
change is made.

Performance advisory: refreshed corpus wall 37.138 seconds against the 16.26-second
quiet-host reference; checked-text thread CPU 837,965 ns/B, host load
[7.166015625,9.5966796875,9.28662109375], 24 workers. Package tests overlapped the
census. This is runtime provenance, not a fitted performance gate.

Refreshed changed-path gate derives
`cargo test -p deckmaste_construction_v3_core -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
The initial pre-refresh scope also included construction_core, lexical_source
and semantics_v2 because the default line advanced with another feature.
Final formatting check exits 0 (stable rustfmt warns about nightly-only options).
Citation check: 0 noncompliant, 15,870 checked, 0 stale; no CR citations changed.
Independent final compiler/contract review found no blockers.


Additional re-spelling: one xtask test retains Jadelight Spelunker's exact input,
two-Reading census, one retained sample and cost 20, replacing the retired
FiniteFrequency string check with FrequencyPredicate plus its finite Category
identity. The isolated corrected test passes. This brings re-spelled function
bodies to 45 (44 English, one xtask); no test is removed or ignored.

Refresh incorporates the completed keyword-body/semantics feature from default;
English grammar/compiler and lexical-source files are unchanged by that refresh.
The refreshed complete census independently preserves every identity and Reading
count. The earlier, already-running broad gate had compiled old semantics tests
before refresh and then read new plugin data; its stale corpus failures are
checked again against freshly compiled semantics tests, without changing either
feature to accommodate those failures.


Fresh `cargo test -p deckmaste_semantics_v2 --test corpus` exits 0: all six
corpus tests pass, resolving the stale pre-refresh binary/data mismatch. No
semantics or plugin source is changed by this feature. The refreshed citation
check covers 15,870 citations; the increased total comes from the incorporated
feature, with zero stale or noncompliant sites.


Final refreshed gate exits 0: 539 tests pass across 47 suites,
zero failures, 1 existing ignored test. Lean workbench integration and doc tests
pass. The existing ignored xtask test is
`macros::templates::tests::macro_schema_census_count_matches_21`, whose documented
blocker is the on-demand live-corpus census cross-check. This feature changes no
ignore attributes. Refreshed formatting and citation checks exit 0.
