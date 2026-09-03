---
needs: [english-v2-preposition-class-data]
---
Replace the name-keyed homograph exemption with a per-member declared
feature (gate-metric-landing-review H1). `reviewed_vocab_lexicon_homograph`
(environment.rs ~:1113) exempts every member of `AttributiveAdjective`
from the vocab/lexicon collision check by vocabulary NAME; adding
`Card = "card"` to that vocab loads clean and only a hardcoded count pin
notices. Per the 2026-09-02 vocab-surface ruling: declare the exemption
on the two members that need it (`Target = "target" { feature
HomographLicense = …; }`, `Untap` likewise), emit it on the VocabSurface
row, consume it in the check, delete the name-keyed function and the
count pin's reliance on it. Probe: `ScratchCard = "card"` inside
AttributiveAdjective must now fail to load. Also (M2): VocabSurface rows
become collision OWNERS so form-literal-vs-vocab collisions are visible.
Note for the chain: adjectives need a lexeme tier (adjective inventory,
core seed + declarations) — a separate ticket after the feature chain;
record it in the landing ledger. Standard constraints apply.

## Landing record (2026-09-02)

The vocabulary-name exemption is gone. `HomographLicense` is a sealed,
vocabulary-member feature emitted on every `VocabSurface`; absent declarations
lower to `Unlicensed`. `AttributiveAdjective` declares that default explicitly,
and only `Target` and `Untap` override it to `Licensed`. The environment now
rejects an unlicensed vocabulary/lexicon homograph by the exact row owner, so a
synthetic `AttributiveAdjective::ScratchCard = "card"` fails against
`CommonNoun::Card` even though it shares the vocabulary that owns the two
licensed members.

`VocabSurface` rows also participate as owners when form literals are checked.
Those closed-class overlaps are census-visible but are not load errors: real
grammar forms intentionally repeat vocabulary surfaces. The coverage metric
therefore moves from 2 to 72: the two licensed vocabulary/lexicon homographs
(`target` and corpus declaration `untap`) plus 70 existing
form-literal/vocabulary overlaps. The public metric name remains unchanged for
schema compatibility, but its documentation now states the expanded scope.

### Numbers and gates

- Coverage: 16,174 -> 16,174 selected and covered units;
  `selected_uncovered_units 0`, `unresolved_ties 0`, `internal_failures 0`,
  `roundtrip_mismatch_units 0`, `ownership_failure_units 0`, `gap_spans 0`,
  `overlap_spans 0`, `synthetic_claims 0`, and
  `provenance_plan_mismatches 0`.
- Constructions: 378 -> 378. No construction or corpus declaration was added,
  deleted, or behaviorally changed.
- Coverage lock: byte-unchanged by this feature at 48,824 lines after the final
  refresh, SHA-256
  `cdd3c0ae4096c6bd538f7453ad46ef3679a38ee7a36d1ffe74f7fe0a634ffe04`.
- `cargo xtask english_v2 coverage --check`: clean; the live summary reports
  32,641 total units, 16,174 selected/covered units, 16,467 ordinary parse
  failures, and 72 fixed-surface collisions.
- `cargo test -p deckmaste_construction_core`: 414 tests green across unit,
  integration, and doc-test targets.
- `cargo test -p deckmaste_english_v2`: 406 tests green across unit,
  integration, and doc-test targets.
- `cargo test -p xtask english_v2::coverage::tests`: 19 tests green.
- `cargo clippy -p deckmaste_construction_core -p deckmaste_english_v2 -p
  xtask --all-targets -- -D warnings`: clean.
- `cargo fmt --all --check`: clean after formatting; only the repository's
  expected stable-toolchain warnings for nightly rustfmt options were emitted.
- No CR citations were added or changed.

### Deviations and additions

1. Added one compiler-emission test proving licensed, explicitly unlicensed,
   and default-unlicensed `VocabSurface` rows. Its default case uses
   `Preposition::During`, the requested probe from the largest closed-class
   vocabulary.
2. Added one environment test. It proves the exact `ScratchCard` rejection and
   makes a form literal `"during"` collide with `Preposition::During`, producing
   one visible census collision without converting the intentional
   closed-class overlap into a load error.
3. Re-spelled two test-only declaration fixtures exposed by the stricter
   row-local check: the homonym pipeline's synthetic verb surface `same` became
   `homonym`, and predicate grammar's synthetic `Declare` verb surface became
   `proclaim`. Their test subjects and asserted outcomes are unchanged.
4. Assurance accounting: restored 0; re-spelled 2; ignored with blockers 0;
   added 2; removed 0.
5. No STOP was taken: the implementation did not contradict a recorded ruling.
6. Scope fence held: no preposition class default, complement licence, or
   existential slot was changed. The only preposition delta is the two test
   occurrences of the `During` surface described above.

### Future work ledger

- Adjectives still need a lexeme tier: an adjective inventory with a core seed
  plus declaration contributions. That work remains a separate follow-up after
  this feature chain.
