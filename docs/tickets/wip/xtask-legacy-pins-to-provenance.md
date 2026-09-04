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
