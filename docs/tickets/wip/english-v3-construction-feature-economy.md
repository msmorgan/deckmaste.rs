---
needs: [english-v3-coordination-schemas]
---
# Reduce construction feature propagation and declaration size

The schema migration leaves 182 named constructions and 2,400 well-formatted
declaration lines. The 250-construction and 2,800-line ceilings apply to the
finished grammar; 400 remaining lines is insufficient headroom at current card
breadth. Reduce the declaration size through fixes and refinements to existing
constructions before spending that headroom on coverage additions.

Audit category features from their producers to their consumers. Remove
provably unused or constant propagation, and share remaining agreement and
selection contracts without erasing category/feature correlation. The migration
already shares typed schema families, explicit policy parameters and defaults;
do not recreate category-specific named AST variants or compress formatting.

Check whether Both/Either/Neither coordination can share one schema with explicit
lexical pair licensing and category-specific permissions. Matching field shapes
alone is insufficient. Preserve distinct relative-clause structures and selected
passive/progressive complements. Record the grammatical justification for each
merge; new constructions require a clear grammatical distinction.

Standard constraints apply. Preserve authentic fixtures, all grammatical
readings and both roundtrip laws. Reconcile complete corpus identities and
reading counts, and report ordinary constructions, schemas, typed instances,
chart productions and physical declaration lines separately.

## Verified implementation (2026-10-05)

Field-parameterized policies share binary right-child and serial rest-child
contracts, including explicit two-field coordinator/source bindings. Policy
arguments must name actual caller fields, and arity is checked even for unused
parameters and default profiles. No category-specific AST variants are added.

Producer-to-consumer proofs remove 25 category feature ports: nine OvertHead
ports (always Yes), thirteen correlative-series CoordinationKind ports with
no surviving consumer, and three clause finiteness ports (always Finite).
Lexical finite-form guards and coordination-kind consumers in nominal/cardinal
agreement remain. The 431 typed construction contracts reconcile exactly after
excluding these proven dead exports/guards; forms, costs, typed children and
remaining equations are unchanged. Both/Either/Neither remain separate where
pair licensing or category permissions differ; no shape-only merge was made.

Declarations fall from 2,400 to 2,232 physical lines, maximum width 100:
134 ordinary constructions, 48 schemas, 297 typed schema instances, 39 feature
types, 33 policies (18 field-parameterized) and 18 tables. No English fixtures
are added, removed, ignored or re-spelled. Three compiler tests are added:
inline/parameterized contract equivalence, invalid argument/arity validation,
and independent recursive-series roundtrip/traversal/cost preservation.

## Landing record

Measured tree pwrpmvlrxnluutzkxxvzpwopnnkxvnsn, covered count 6,186.
Baseline krzqrwxllknpmmzrwrvztnkllvnonmpw uses the identical supported input
and lexical inventory. Complete 24-worker enumeration preserves every face
identity, every face reading count, every retained sample and construction
inventory, and every chart/reading metric. No newly covered or lost identities.
All 28,799 Readings pass declaration admission, lexical ownership, byte-exact
realization and construction/leaf traversal; zero duplicates, internal failures,
limited enumerations or validation issues. The independent generated-value
roundtrip tests supply the second structural law. Census remains 26,642 no,
4,246 one, 1,940 multiple across 32,828 supported faces. Selection costs,
lexical homographs and literal/vocabulary overlaps are unchanged; no new
licensing checker or glossary term is introduced.

Corpus wall time is 28.580 s, 836,334 ns/B checked-text thread CPU, 24 workers,
host load 5.97/7.09/6.63; this exceeds the 16.26 s advisory. The baseline was
37.138 s at load 7.17/9.60/9.29; concurrent tests and differing load prevent
attributing the difference to this refactor. Retained samples are one per face,
with complete enumeration and no reading limit. Scratch reconciliation reports
remain under /tmp, not new repository audit documents.

This implementation advances the attached systemic-residuals workspace while
that ticket's broader cause audit remains unfinished. Keep that ownership and
its unresolved obligations open; do not integrate both tickets as complete.

Verification: `cargo xtask gate --changed --run` exits zero: 1119 passed
tests across 75 suites, 1 existing on-demand live-corpus test ignored.
Lean integration and documentation tests pass. `cargo fmt --all --check`
and `cargo xtask cite check` pass (15,871 citations, zero stale).


## Compiler support for the coverage batch

Typed table requirements express correlated partial selection constraints
without dummy output ports. Explicit custom-feature defaults distinguish an
absent lexical property from an invalid authored value; no-default and built-in
feature checks retain their strict behavior. Reading admission and realization
share an immutable grammar, while lexical inventories remain caller supplied.
Structured frame declarations replace 28 escaped RON strings; compiler tests
prove identical typed frames and emitted tokens across both syntaxes, including
marked and optional slots. The compiler/runtime suite passes 56 tests.

The expanded grammar currently has 161 ordinary constructions and 52 schemas
(213 named), 2,484 physical lines, maximum width 100. This remains too near the
ultimate line ceiling; further safe schema reuse is under audit. No count-based
Reading pruning or compacted formatting is introduced.


Explicit typed instance-header bindings replace 68 repeated body bindings,
including grouped field targets. The generated Reading fields and complete
emitted tokens reconcile exactly against the old syntax. Unknown, duplicate,
lexical/fixed-category targets and policy-column misuse reject. The unused
additive-person table is removed after a zero-consumer check. The compiler
suite now passes 58 tests; focused independent English host tests pass 16.
