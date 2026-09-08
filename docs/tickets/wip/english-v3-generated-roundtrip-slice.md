---
needs: [english-v3-construction-compiler]
---
# Prove the generated v3 path on an interacting slice

Use the fresh compiler to replace the handwritten prototype rules for one
bounded but interacting grammar slice. This is a proof of the generated path,
not the unit of subsequent grammar migration. Include a finite clause with
agreement, an imperative, a nominal/header use, one selected Complement, one
modifier or attachment ambiguity, and one document boundary.

Parse precomputed lexical alternatives through generated Productions and
admission summaries, retain all grammatical Readings, and materialize them only
on request. The declaration-generated renderer must satisfy byte-exact
parse/render for every admitted witness and independently-constructed-value
render/parse for every finite valid value in the slice. Preserve lexical
identity, feature correlations, spelling variants, capitalization and complete
traversal.

Acceptance includes `cast` as a noun and verb, finite present/past ambiguity
where both are licensed, wrong-agreement rejection, `one` as noun and
determiner/numeral, a multiword overlap, a jointly invalid feature combination,
and two derivations that denote only one Reading. Expose a v3 inspection path
that reports lexical alternatives, packed structure and Readings without
requiring exhaustive enumeration. Measure compiler/build time and chart work.
Once this slice is green, the next implementation ticket turns on the complete
interconnected grammar rather than repeating slices family by family. Standard
constraints apply.

## Landing record

Measured on change `yrsnnrqqtlxt`, with the unchanged production lock's
`covered` count 20,254. The generated slice is now the executable v3 inspection
path; whole-grammar activation remains `english-v3-whole-grammar-activation`.

### PROVE

The 15 Construction declarations in `deckmaste_english_v3/src/slice.rs` generate
all Productions, lexical constraints, admission summaries, checked values,
lazy materializers, realization and traversals. The runtime now consumes the
fresh proc macro directly. No v2 dependency, compatibility adapter, handwritten
Construction renderer or grammar scanner was added. Recognition consumes
precomputed lexical alternatives: a test removes present alternatives and
obtains exactly the past Reading, then removes past alternatives and obtains
no admitted root. Recognition and packed inspection construct zero ASTs.

The shared nominal/Complement grammar interacts with finite person/number
agreement and retained tense, imperatives, optional nominal/predicate attachment,
nominal headers and newline Document boundaries. Exact independently built
Reading sets prove the named cases: noun and verb `cast`; present/past
syncretism where licensed and past alone with singular third-person `it cast`;
noun and Determinative `one` while the independent lexical layer also retains
its Numeral analysis; lexical multiword `first strike` and the compositional
adjective/noun Reading; wrong agreement and selected-frame rejection; and two
normalized noun Productions denoting one Reading. `it draw` rejects even though
singular and third-person features each occur among separate `draw` alternatives.

Both roundtrip laws are exercised by independent complete-value families:
72 nominal values, 180 Basic Noun Phrases, 360 prepositional Complements, 720
imperative compositions spanning both attachment sites, 112 finite clauses
covering every compatible declared subject/verb bundle, spelling variant and
capitalization, and 62 headed/unheaded Documents through four sentences. These
compare complete expected Reading sets and byte-exact realization of every
returned Reading. A separate assertion compares the entire Construction
preorder and lexical traversal, including full lexical values and recovered
declaration provenance. The README states the compositional roundtrip argument
for all finite values and distinguishes it from finite test enumeration.

No existing corpus consumer, coverage lock, lexical source adapter, construction
declaration or runtime witness was removed or redirected. There are no lost or
newly covered production identities to route. All 18 existing packed-runtime
tests and the downstream compiled-consumer tests continue to pass. Every
exhaustive slice census reports zero internal failures and cyclic derivations.
Multiple grammatical Readings remain successful, with no preference mechanism.

No lexical identity, spelling, card name or Construction-identity admission
guard was introduced. The generated admission uses declared feature equations
and full selected-frame signatures exclusively; literal/vocabulary overlap is
empty because grammar literals contain only separators and punctuation.
Vocabulary irregular replacements are lexical declarations, not licensing
checks. Production licensing-checker and `environment.rs` loader totals are
unchanged and were not remeasured; their code and input data did not change.

### DISCLOSE

The production Reading/selection census is unchanged. In the slice,
`you cast one spell with one counter` has exactly eight Readings: two correlated
Number choices, two tenses and two attachment structures. The imperative with
the same Complement/Adjunct has two attachment Readings. `it cast one spell`
has one past Reading. `first strike` has two nominal Readings; noun `cast` and
noun `one` each have one, despite two noun Derivations. All ten listed
wrong-agreement/frame/shape witnesses and all five malformed Document-boundary
witnesses have no Readings. Every newly admitted fixture is backed by complete
AST assertions or an independently enumerated value family, not a coverage-only
success assertion.

Deviations and additions:

- Added 15 slice Constructions and a bounded vocabulary in the v3 runtime crate,
  plus its proc-macro dependency and a Clap dependency for the inspection example.
  Existing handwritten chart tests remain authentic runtime evidence; the new
  executable path uses generated rules throughout.
- Added the generic `Forest::write_packed` diagnostic view so inspection reports
  shared completed/intermediate nodes, families and lexical leaves without
  expanding paths. `--readings 0` requests no ASTs; a positive limit reports
  that remaining Readings were not counted rather than claiming exhaustion.
- Added `learned`/`learnt` spelling evidence, an intransitive frame negative,
  exhaustive Complement/attachment families, precomputed-input restriction and
  4/16/64-sentence packing evidence. They exercise the ticket's stated laws and
  public input/inspection boundaries without widening the scheduled migration.
- Empty Documents and a nominal header with an empty sentence sequence are
  explicit slice values. There is one Premodifier and at most one prepositional
  Adjunct at each nominal/predicate site; the preposition selects a Basic Noun
  Phrase. The README discloses this bounded grammar and its vocabulary.
- Tests: restored 0, re-spelled 0, ignored 0, added 9, removed 0. No existing
  assertion was weakened. The first strict lint run required Errors sections
  on two new public APIs; both were added and strict linting now passes.

No regression, recorded-ruling contradiction, unresolved STOP or glossary gap
was found. No CR citation changed. No full corpus pass was run: the new
standalone slice does not alter the production grammar or its consumed data.

### REPORT

All figures are stamped with `yrsnnrqqtlxt`, lock covered 20,254. Added v3 slice
Construction count: 15. Existing synthetic compiler Construction inventory and
v2 production inventory are unchanged. Cross-category lexical homographs in
the bounded inputs are `cast`/`casts` (noun/verb), `one` (noun/Determinative/
Numeral), `two` (Determinative/Numeral), and `first` (Adjective/Numeral).
The multiword overlap is `first strike` versus `first` + `strike`. Inflectional
syncretism, separately, includes plain/present `cast`, `draw`, `learn`, `walk`;
present/past/participle `cast`; and past/participle `learned`, `learnt`, `walked`.
Initial-capital alternatives preserve those distinctions. The grammar's named
form-literal/vocabulary overlap inventory is empty.

| Sentences | Lexical alternatives/projections | Items/intermediates | Completed nodes | Families | Completion work | Requested/returned Readings | Derivations/duplicates | Builds |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 4 | 64/64 | 317 | 90 | 415 | 169 | 2/2 | 3/1 | 65 |
| 16 | 256/256 | 1,241 | 354 | 1,627 | 673 | 2/2 | 3/1 | 185 |
| 64 | 1,024/1,024 | 4,937 | 1,410 | 6,475 | 2,689 | 2/2 | 3/1 | 665 |

The 64-sentence forest represents at least `2^64` Readings. Families count both
intermediate edges and completed families. The measurement requests two
Readings only; it does not enumerate that space or promise bounded delay between
distinct Readings in general.

After refreshing onto the coordinator line, `cargo xtask gate --changed
--clippy --run` derived and passed:

```sh
cargo test -p deckmaste_construction_v3 -p deckmaste_english_v3
cargo clippy -p deckmaste_construction_v3 -p deckmaste_english_v3 --all-targets -- -D warnings
```

All 42 tests passed with zero ignores. Nightly formatting checks passed. Direct
inspection smoke checks verified packed-node output with zero requests/builds,
and two exact realized Readings with an explicit non-exhaustion message.

Performance advisory: direct compiler IR parse/validation/emission of the actual
slice tokens took 5,101 microseconds median (5,062 minimum, 12,031 maximum over
25 single-worker samples), emitting 62,135 bytes of token text. Host load was
1.17 / 3.84 / 5.23. Recompiling the generated v3 library with warm dependencies
and one Cargo worker took 289 ms wall time (Cargo reported 250 ms); the
single-test-worker slice census command took 841 ms including compilation.
Host load for these commands was 3.38 / 4.16 / 5.26. Cargo's `--timings` report
is in the ignored target directory. The 16.26 s quiet-host coverage ceiling and
accepted-byte thread-CPU telemetry in ns/B are not applicable measurements for
this standalone slice and were not remeasured as production coverage figures.
Evidence logs are `/tmp/v3-slice-gate-final.log`,
`/tmp/v3-slice-compiler.log` and `/tmp/v3-slice-metrics-measured.log`; no scratch
compiler harness, timing artifact or generated Rust dump enters repository history.
