---
needs: []
---
# Demote the six legacy pins to provenance

**R2 — Group R, off-grammar (xtask only; may run beside any grammar ticket).**
Authority: rewrite ADR "Amendment: what a landing proves, discloses, and reports
(2026-09-04)".

Defect. Six numbers designed for the retired corpus-driven plan are enforced as
gates, so the cheapest way past each is to make a grammar addition smaller. They
guard nothing the real invariants do not already guard:

| pin | site | note |
| --- | --- | --- |
| `LICENSED_VOCAB_LEXICON_HOMOGRAPHS == 2` | `crates/xtask/src/english_v2/coverage.rs` const + `--check` | an `!=` equality, so it also fires on a legitimate retirement; the ADR already names the pending third homograph (the Target Verb, `english-v2-target-verb-subject-selection`) |
| `FORM_LITERAL_VOCAB_OVERLAPS_CEILING <= 5` | same file | re-pinned onto the post-landing value (25 → 5), zero headroom |
| builtin noun-morphology census `491 / 165 / 26 / 300` | `crates/xtask/src/english_v2/report.rs`, two `assert_eq!` | adding one general noun the corpus does not attest breaks two tests |
| `roots == 8` in the escape-hatch vector | `report.rs` | root count is grammar shape, not an escape hatch |
| card-name catalog `assert_eq!(names.len(), 32_548)` | `crates/xtask/src/english_v2.rs` | hand-edited on every snapshot refresh |
| parenthetical census `17` / `136` | `crates/xtask/src/english_v2/corpus.rs` | attestation counts for a normalization decision |

Pinned shape. Each becomes a **reported** figure: emitted by `coverage`/`report`
with its named inventory where it has one (the licensed homograph owners, the
surviving overlap surfaces), and never a failure. Keep as gates the real
invariants beside them: the `environment.rs` load errors for an unlicensed
literal/vocabulary collision and for a licensed form literal governing nothing;
the card-name onset-override two-way closure `ensure!`; the noun-morphology
partition identity (`total == derived + explicit + unavailable`) and its
"no unclassifiable surface" error; the seven escape-hatch zeros in the
`report.rs` vector.

Also in scope, because landings treat them as binding and no code computes them:
give the **construction count** one canonical command so records stop disagreeing
(391 / 394 / 397 / 398 have all been quoted), and split the historical
"literal/lexicon collisions: N" figure permanently into its two named fields so
cross-landing comparisons stop being incommensurable.

Fences. Replacing a pin with a looser pin. Deleting an invariant along with its
pin. Adding a new count-shaped gate of any kind.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply; `cargo test --workspace`.

## Landing record (STOP, 2026-09-04)

Measured on change `kpnnsrty` with 17,052 covered lock identities after
`kata refresh`. `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage
--check --workers 8` exited 0. It printed no lock-delta lines: 0 identities
lost and 0 gained. Consequently there are no newly covered identities or
selected analyses to disclose.

PROVE: coverage reported zero selected-uncovered units, unresolved ties,
internal failures, roundtrip mismatches, ownership failures, traversal
failures, leaf-traversal failures, forbidden licensing checkers, and forbidden
provenance-plan mismatches. The retained structural checks include the
environment's lexical-ownership load validation, the card-name onset-override
two-way closure, the noun-morphology partition, and the seven escape-hatch
zeros.

REPORT: the canonical command is `cargo xtask english_v2 report`; it reported
388 construction declarations, 32,548 card-name catalog rows, noun morphology
`491 / 165 / 26 / 300`, and eight roots. Coverage reported 2 licensed
vocabulary/lexicon homograph owners and 5 form-literal/vocabulary overlap
surfaces (all named in its output). Parenthetical provenance reported five
rules-bearing surfaces with 17 total occurrences and nine reminder surfaces
followed by text with 136 total occurrences. Coverage performance was 117.704
s at 8 workers, 140,493 ns/B thread CPU, host load 13.08 / 11.39 / 9.26;
concurrent-process count is unavailable in the sandbox.

DISCLOSE: selection census was unchanged by this xtask-only change and was not
re-run after the workspace-wide test failure below. Permitted licensing
checkers: 20. Deviations and additions: none beyond the ticket; one fixture
test establishes the canonical construction-count command and the demoted-pin
tests re-spell the existing assurances.

STOP: `cargo test --workspace` is red outside this ticket's scope:
`deckmaste_construction_core/tests/builtin_v2_keyword_actions.rs`,
`exchange_has_every_attested_representable_tail_shape`, expects a literal
`with` frame atom but received the declared Preposition vocabulary member.
No `crates/xtask` change can repair that grammar assertion without violating
the ticket's scope, so this work is committed for review but not integrated.

Assurance counts: 0 restored, 4 re-spelled, 0 ignored with blockers, 1 added,
0 removed. Glossary gaps: none.
