---
needs: [english-v2-locative-licence-data]
---
Single ownership for closed-class words (homograph review Class C, 17
rows): `less`, `equal`, `greater`, `more` (three owners), `twice`,
demonstrative `that`, `fewer`, `both`, `any`, `instead` are spelled both
as form literals and as vocab members. One owner each: consume the vocab
member through a DSL `require role is Member` on the declared vocab member
(the `require relation is Among` idiom) and delete the duplicate literal.
Never a Rust `checked by` naming the word: `singular_demonstrative_is_this()`
/ `noun_is_way()` are the shape `english-v2-this-way-lexeme-guard` retires. Class A (16 rows: infinitival `to`,
complementizer `that`, `for each`, `at random`, "as you choose") are
genuinely different words — declare their licence on the form atom so
the census separates governed from ungoverned. Split the metric:
`licensed_vocab_lexicon_homographs` pinned EXACTLY at 2 and
`form_literal_vocab_overlaps` as a decreasing ceiling (70 today; this
ticket lowers it by 17 + the 16 licensed), replacing the bumpable exact pin (re-measure its live value at claim — it has moved 72 -> 60 -> 59 across landings; the ceiling belongs in `coverage --check` beside the lock, not in a `#[cfg(test)]` unit test) and the loosened `assert!(… > 1)` environment pin.
Also fold the flavor-word landing's 2x2 label constructions ({AbilityWord, FlavorWord} x {bare, em-dash-prefixed}) through a shared `LabelTerm` sum (flavor review M1; -2 constructions). Zero coverage change; standard constraints apply.

## Landing record

Measured on change `uqkrqvln` (the reviewed tree, refreshed onto the default
line that carries the licensing-checker census and the one-candidate-domain
core fix) with 16,771 covered lock identities. The schema-4 lock remains
49,421 lines with SHA-256
`2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`.

| metric | before | after |
| --- | ---: | ---: |
| selected and covered units | 16,771 | 16,771 |
| unique selections | 11,515 | 11,515 |
| specificity-resolved selections | 5,256 | 5,256 |
| Construction declarations | 397 | 395 |
| combined literal/Lexeme collisions | 59 | replaced |
| licensed vocabulary/Lexeme homographs | unseparated | 2 |
| unlicensed form-literal/vocabulary overlaps | unseparated | 25 |

- Ownership: all 17 live Class C duplicate form literals now consume the
  vocabulary member's Lexeme and Word Form through a DSL `require role is
  Member`. No Rust callback guard was added. The 15 live Class A form rows
  carry a row-local homograph licence while retaining their distinct
  Realization ownership. The coverage gate pins licensed vocabulary/Lexeme
  homographs exactly at 2 and rejects an unlicensed form-literal/vocabulary
  overlap census above 25.
- Coverage and selection: the lock has +0/-0 identities. No identity became
  newly covered, so there is no newly selected analysis or negative oracle to
  enumerate. Selected-uncovered units, unresolved ties, internal failures,
  exception resolutions and uses, round-trip mismatches, ownership failures,
  gaps, overlaps, synthetic claims, and provenance-plan mismatches are all
  zero. The source fingerprint remains
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and the normalization digest remains
  `f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`.
- Shared label grammar: the former four bare and em-dash-prefixed label
  Constructions now pass through one `LabelTerm` Category and the chapter
  label remains a separate Construction, removing two Construction
  declarations without changing selection.
- Performance advisory: on the reviewed tree `cargo xtask english_v2 coverage
  --check` used 24 workers, took 40.313459653 seconds at 130,091 ns/B, and
  reported host load 33.18/29.12/28.22; `ambiguity --require-resolved` took
  22.425729111 seconds at 112,873 ns/B. The implementer's own run was
  21.494721400 seconds at 112,535 ns/B under host load 28.93/20.21/20.98. The
  sandboxed `pgrep -c -x codex` reading of 1 concurrent Codex process is not
  the host's: 3-6 concurrent Codex executors were running on the host across
  every one of these measurements. Both runs exceeded the 16.26-second
  quiet-host ceiling under that contention; this is advisory, not a STOP.
- The required post-commit Kata refresh rewrote the feature stack twice: once
  by the implementer, and once at review onto the default line carrying the
  licensing-checker census (report schema 7) and the one-candidate-domain core
  fix. The second refresh conflicted only in `coverage.rs`, where both field
  sets were merged under schema 8. Every remeasurement retained the census
  values in the table, the exact 2/ceiling 25 collision split, the
  zero-failure coverage summary, and zero unresolved ties.
- Positive artifacts on the reviewed tree: `cargo fmt --all -- --check`;
  strict all-target Clippy for `deckmaste_construction_core`,
  `deckmaste_english_v2`, and `xtask`; `cargo test --workspace` (127 suites,
  0 failed, including `test result: ok. 397 passed; 0 failed`, `test result:
  ok. 143 passed; 0 failed`, and `test result: ok. 426 passed; 0 failed`);
  `cargo xtask english_v2 ambiguity --require-resolved`; and `cargo xtask
  english_v2 coverage --check` all exited zero. A strict workspace-wide Clippy
  attempt reaches ten pre-existing `unneeded_wildcard_pattern` findings in
  `deckmaste_lowering/src/card.rs`; that file is not changed here.
- Assurance census: restored 0; re-spelled 13 existing test functions and 5
  test fixtures; ignored with blockers 0; added 2 test functions; removed 0. The additions
  authenticate the fixed-surface licence syntax and the exact/ceiling
  collision gate. Existing compiler, environment, runtime, AST, provenance,
  specificity, rendering, and traversal checks were updated to authenticate
  the single-owner Production shapes.
- STOPs: none. There was no selection tie, ruling contradiction, coverage
  drop, newly covered identity, negative oracle, or new Rust callback guard
  naming a lexical or grammar identity.

### Deviations and additions

- The ticket's Class A inventory counted 16 rows including `for each`; the
  prerequisite adjunct work had already removed that duplicate form literal.
  The live inventory at claim contained 15 rows: five infinitival `to` rows,
  five complementizer `that` rows, three `for` rows, one `at` row, and one
  `you` row. The combined 59 was 2 licensed vocabulary/Lexeme homographs plus
  57 form-literal/vocabulary overlaps; removing the 17 Class C duplicate
  literals and licensing the 15 Class A rows leaves the measured ceiling of
  25 (57 - 17 - 15), with the 2 licensed homographs now counted separately.
- The coverage report schema advances to 8 to expose the two collision
  censuses separately. The sibling licensing-checker census landing took 7 on
  the default line; this landing's refresh merged both field sets and moved to
  the next monotonic number. The coverage lock bytes and schema do not change.
- The DSL accepts `licensed("literal")` only as an unnested form atom and
  preserves the licence in generated form-surface metadata while lowering
  the Realization like an ordinary literal.
- No scratch copy or probe tree was created.

### Review corrections

- MEDIUM, ticket-banned duplicate authority: the environment unit test
  re-pinned the ceiling with `assert_eq!(baseline.form_literal_vocab_overlaps(),
  25)`, the `#[cfg(test)]` pin the ticket moved to `coverage --check`. Removed;
  the test keeps its synthetic-surface mechanism assertions and the licensed
  count, and the ceiling now has one owner.
- MEDIUM, the ratchet could be lowered without fixing anything: a
  `licensed("...")` atom whose surface no vocabulary member owns was accepted
  silently, so any literal could be excluded from the overlap census by
  declaring a licence that governs nothing. The environment now rejects an
  ungoverned licence (`ParserEnvironmentError::UngovernedHomographLicense`),
  authenticated by a synthetic-surface case. All 15 shipped licences govern a
  real vocabulary surface.
- MEDIUM, opaque exact pin: the licensed-homograph gate failed with counts
  only. The environment now records each admitted homograph's vocabulary
  member and lexical owner from data, and the gate names the rows it saw when
  the census moves. No lexeme, vocabulary, or card name is written in the gate.
- MEDIUM, record arithmetic: the deviation derived the ceiling as 59 - 17 - 15
  = 27 against a measured 25. Corrected above: the subtraction base is the 57
  form-literal/vocabulary overlaps inside the combined 59.
- LOW, record counts: re-spelled test functions were 13, not 11 (5 test
  fixtures were also updated). Corrected above.
- LOW, contention stamp: the sandboxed `pgrep` count is replaced with the
  host's 3-6 concurrent Codex executors.
- Required by this landing, ADR silent: `docs/decisions/english-v2-rewrite.md`
  gains a dated amendment recording the licensed form atom, the single-owner
  rule for closed-class words, the ungoverned-licence rejection, and the
  two-number census. One factual comment documents `FormAtom::LicensedLiteral`
  at its declaration site.
- Refresh reconciliation: the review refresh landed on a default line whose
  sibling had already taken report schema 7 for the licensing-checker census.
  Both field sets are merged and the schema moved to 8; every version pin,
  summary field list, JSON expectation, and constructor was updated, and the
  full gate set was re-run and re-measured on the merged tree. Trunk carries
  report schema 8 after this landing.
- Routed, not fixed here: single ownership pointed general constructions at
  vocabularies named for one earlier consumer (`ObjectOrder::Any` for the
  `any number of` determiner, `CostComparisonDirection` for four scalar
  comparisons, `DistributionReplacement` for the general replacement
  predicate), so the declared names now misdescribe the words they spell.
  Ticket: `english-v2-vocab-names-outgrow-birth-construction`.

glossary gap: licensed form atom — the ticket's explicit row-local licence on
one fixed-surface atom in a Production.

glossary gap: homograph licence — the ticket's declaration that distinct
Lexemes or Word Forms may share a Realization without creating an ungoverned
collision.

glossary gap: LabelTerm — the ticket-named shared Category for the
AbilityWordTerm and FlavorWordTerm constituents used by label Constructions.
