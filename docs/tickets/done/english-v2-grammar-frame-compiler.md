---
needs: [english-v2-grammar-migration-design]
---
# Compile declared frame schemas with the existing Earley engine

Implement the runtime-frame representation in [the migration design](../../english-grammar-design.md#runtime-frame-representation).
Keep `constructions!` as the owner of typed ASTs, checked builders, rules,
renderers and total visitors. Use the existing Earley engine; ordered-choice,
PEG, recursive-descent and a fallback parser are not substitutes.

Unify core/plugin frame atoms at `construction_core::macro_def`, with grammatical
relation, category, fixed lexical marker, marked Complement and optional item.
Generate a checked heterogeneous frame sequence and a schema registry. Prepare
finite `(schema, position)` rules in an environment-owned rule table; adapt the
engine's static RHS borrow, scanner dispatch, root IDs and build dispatch.
Retain source-bearing lexical leaves and declaration identity. Dynamic grammar
IDs are internal and cannot add Construction identities or preference weights.
Reject unknown roles/categories/lexical references at environment construction.

This ticket introduces the compiler capability through a compiled consumer,
without registering a second production grammar. The existing tail-codec path
is deleted by `english-v2-lexeme-owned-verb-frames`, its immediate consumer.
No speculative general-purpose runtime extraction is required.

Acceptance through `deckmaste_construction/tests/compiled_consumer.rs`: two
lexemes use one schema; a new ordered NP/marked-PP/measure schema works by data
alone; required/optional items render with exact ownership; wrong child category,
missing required child and a schema from the wrong head reject; a nonempty
frame-part sequence coordinates for all three existing Coordinators. Include
the inherited `FrameComplementPair` fixture gap. The feature-independent engine
suite still exercises left recursion, forest completeness on its fixtures and
structured failures. Report prepared-rule count and preparation/parse cost on
these fixtures. This is the first check of the runtime-rule engineering choice;
report a demonstrated obstacle instead of adding another per-tail codec.

Standard constraints apply. Production correspondence and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.


## Landing record

Measured on change `stvrspvomrwutryolkusosuuzlwtsouw`, after `kata refresh`;
lock `covered` = **20,254**. The final refresh adds only Lean and bookkeeping
changes relative to the measured Rust/data tree; the dependency gate was rerun
and the corpus evidence remains applicable.

**PROVE:** `frame_family` generates an environment-owned registry, typed child
sum, checked member sequence, source-bearing heads and markers, prepared rules,
scanner/build dispatch, renderer and visitor. Prepared rules compose the existing
static grammar with schema suffixes, nonempty coordinated intervals and a root
adapter. The real Earley engine borrows the owned RHS slices; it remains the
only parser. Schema IDs introduce no Construction identity or selection weight.
Core and plugin atoms share `macro_def::FrameItem`; the tail-codec input spelling
remains an alias for the immediate consumer migration.

The compiled consumer demonstrates shared schemas across two lexical identities,
a new ordered NP/optional marked-PP/measure schema supplied only as data, required
and optional fixed/marked members, exact lexical ownership, typed child values,
all three Coordinators, nonempty conjuncts, wrong-category/missing-member/wrong-head
rejection, and unknown role/category/lexical-reference rejection during preparation.
It checks that an embedded child with a standalone render root contributes no
root punctuation. The inherited `FrameComplementPair` fixture now compiles the
positional sequence and selected-role checker, exercises its checked build,
rejects a mismatched marker and empty sequence, and verifies rendering and
traversal. The feature-independent engine suite retains left-recursion, forest
completeness and structured-failure cases, plus a new owned-rule ambiguity case.

Full-corpus coverage and ambiguity JSON rows are exactly equal to the prior
recorded results of `lean-facts-tables-from-registries` (change `uomrqrvv`): all
32,641 rows, including selected analyses and diagnostics. There are no newly
covered identities and no new losses. Both reports retain 20,252 selected units;
coverage has zero ownership, construction-traversal, leaf-traversal, roundtrip,
unresolved-tie or internal failures. All 904,092 construction visits and 315,652
leaf visits match their expected counts. Roundtrip separately reports 20,252
clean units, zero mismatches, zero ambiguous units and zero internal failures.

The two lock-relative losses are inherited scope retirements already classified
in that landing, not regressions or open re-coverage obligations:

- `1d12969b573ddeab74744733349723633eaded98bf64b4cf0694a3a6b152af56`
  — Start the TARDIS; the out-of-scope planeswalk analysis was retired.
- `8285bf929bff5f9be66997c1eae3405964ca1e901f8467b557b0d51e0e21d49a`
  — TARDIS; the same retired analysis.

The source lock remains unchanged. The production environment loads successfully;
its licensing census reports **0 forbidden** and **24 permitted** checkers.

**DISCLOSE:** Selection is unchanged from the prior recorded census: **17,197
unique / 3,055 specificity-resolved**, with zero exceptions. No construction pair
increases its specificity share. Production still uses the existing tail-codec
path; `english-v2-lexeme-owned-verb-frames` registers the prepared family in its
category graph and deletes that path. No second production grammar is registered.
The prepared-family fixture establishes the engineering seam; its small inputs
are not a claim about full production preparation or parsing cost.

Test accounting: **11 added, 0 restored, 0 re-spelled, 0 removed, 0 newly ignored**.
The gate retains one existing ignored test,
`macros::templates::tests::macro_schema_census_count_matches_21`, whose recorded
blocker is the on-demand live-corpus census cross-check.

Deviations and additions: no production Construction was added or removed.
Fixture-only additions are `noun_phrase`, `preposition_phrase`, `measure_phrase`,
`frame_complement_pair` and `paired_predicate`, plus the generated
`LexicalVerbPhrase` family. The extra standalone-child-root, optional-empty-conjunct
and lexical traversal checks protect the new integration boundaries. `xtask` learns
the new generated-module and frame-family item labels. English and Semantics
remain independent, and no Lean source or Lake project changes. The formal
`FrameItem` interface was inspected through the Lean LSP; no Rust LSP was exposed
by this session. No unresolved ruling contradiction, production regression or
glossary gap was found.

**REPORT:** `cargo xtask gate --changed --clippy --run` derived and passed:

```text
cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask
cargo clippy -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings
```

The dependency gate ran **1,461 passing tests**, with the one existing ignore.
The compiled consumer also runs with `parser-metrics` enabled. Formatting and
strict clippy pass; citation checks report 0 noncompliant and 0 stale. The full corpus commands used four workers each, concurrently:
`english_v2 coverage --check --json` with `DECKMASTE_COVERAGE_LOCK=report`,
`english_v2 ambiguity --require-resolved --json`, and
`english_v2 roundtrip --require-clean --json`.

Fixture timing on the refreshed tree (one test thread; nanoseconds):

| Schema fixture | Total prepared rules | Preparation | Parse/build/render checks |
|---|---:|---:|---:|
| One NP schema shared by two lexemes | 19 | 114,374 | 510,640 |
| NP / optional marked PP / measure | 75 | 22,573 | 1,477,213 |

The first workload checks two accepted strings. The second checks two accepted
strings and two rejected strings. These are sequential cold/warm measurements,
not a scaling comparison. The rule totals include ordinary static productions
and the root adapter. Interval coordination makes preparation quadratic in each
schema's length; no corpus-derived grammar table is built.

Performance advisory: coverage took **205 seconds**, above the **16,260 ms**
quiet-host ceiling, at **162,344 ns/B** accepted thread CPU. It used four workers,
with host load `1m=8.87 / 5m=10.69 / 15m=7.91`; the other two corpus checks and
other work were active, so this is not a quiet-host performance claim. Ambiguity
and roundtrip took 190 and 202 seconds, at 161,734 and 155,867 ns/B respectively.
Their host loads were `10.62 / 11.10 / 7.99` and `9.38 / 10.82 / 7.93`.

The production inventory remains **397 Constructions**. Homographs:
`AttributiveAdjective::Untap` / keyword action `Untap`, and
`TargetingMarker::Target` / `CommonNoun::Target`.
The nine form-literal/vocabulary overlaps remain:
`additional_cost` atom 2 (`additional`), `up_to_quantifying_determiner` atom 1
(`to`), `definite_next_mass_quantity_reference` atoms 0 and 1 (`the`, `next`),
`scalar_less_than_or_equal_to` atom 4 (`to`), `number_of_scalar_value` atom 0
(`the`), `greatest_scalar_value` atom 0 (`the`),
`other_than_qualified_reference` atom 1 (`other`), and `positional_partitive`
atom 0 (`the`). All counts and timings above use the stamped change and lock count.
