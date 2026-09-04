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

Measured on change `qtprrqyl` with 16,771 covered lock identities. The
schema-4 lock remains 49,421 lines with SHA-256
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
- Performance advisory: `cargo xtask english_v2 coverage --check` used 24
  workers, took 21.247511613 seconds at 105,362 ns/B, and reported host load
  15.81/30.41/26.31. `pgrep -c -x codex` reported 1 concurrent Codex process
  at measurement. The elapsed time exceeded the 16.26-second ceiling under
  concurrent Cargo activity; this is advisory, not a STOP.
- Positive artifacts before the landing commit: `cargo fmt --all -- --check`;
  strict all-target Clippy for `deckmaste_construction_core`,
  `deckmaste_english_v2`, and `xtask`; focused tests for those packages;
  `cargo test --workspace`; `cargo xtask english_v2 ambiguity
  --require-resolved`; and `cargo xtask english_v2 coverage --check` all
  exited zero. Representative test artifacts include `test result: ok. 397
  passed; 0 failed`, `test result: ok. 143 passed; 0 failed`, and `test result:
  ok. 423 passed; 0 failed; 1 ignored`. A strict workspace-wide Clippy attempt
  reached ten pre-existing `unneeded_wildcard_pattern` findings in
  `deckmaste_lowering/src/card.rs`; that file is not changed here.
- Assurance census: restored 0; re-spelled 11 existing test functions;
  ignored with blockers 0; added 2 test functions; removed 0. The additions
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
  `you` row. Starting from 59, removing the 17 Class C duplicate literals and
  excluding the 15 licensed rows yields the measured ceiling of 25.
- The coverage report schema advances from 6 to 7 to expose the two collision
  censuses separately. The coverage lock bytes and schema do not change.
- The DSL accepts `licensed("literal")` only as an unnested form atom and
  preserves the licence in generated form-surface metadata while lowering
  the Realization like an ordinary literal.
- No scratch copy or probe tree was created.

glossary gap: licensed form atom — the ticket's explicit row-local licence on
one fixed-surface atom in a Production.

glossary gap: homograph licence — the ticket's declaration that distinct
Lexemes or Word Forms may share a Realization without creating an ungoverned
collision.

glossary gap: LabelTerm — the ticket-named shared Category for the
AbilityWordTerm and FlavorWordTerm constituents used by label Constructions.
