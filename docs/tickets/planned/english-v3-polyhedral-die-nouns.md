---
needs: []
---
# Declare the die nouns *d4*, *d6*, *d8*, *d10*, *d12*: Roll a d6.

## Why

Data ticket. *d20* is declared and reads ("Roll a d20." 1); its siblings are
unknown words. Listed out of scope in `english-v3-plugin-term-definitions`
("core data by that precedent"). Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): `word:d6`, `word:d8`, `word:d4`,
`word:d10`, `word:d12`. `cargo xtask english-v3 grep '\bd(4|6|8|10|12)\b'`
finds **15** faces, all unread; on all **15** that line is the only failing
line, keyed `word:d6` 4, `word:d4` 3, `word:d8` 3, `word:d10` 3 (one with a
second unknown word, *just*), `word:d12` 2.

## Goal

The five die nouns are `CommonNoun` lexemes with the features of *d20*, and
read wherever *d20* reads.

## Analysis

No CGEL claim is relied on. Existing declaration:
`crates/deckmaste_lexical_source/lexicon/vocabulary.rs`, `D20 = "d20" {
feature LocativeTemporalLicense = Unlicensed; feature Relationality =
NonRelational; }`, beside `Die = "die" { Plural = "dice", … }`.

Orchestrator rulings 2026-10-10:

- PD1. Declare `D4`, `D6`, `D8`, `D10`, `D12` beside `D20` with the same two
  features. No grammar change, no Construction, no numeral-suffix morphology
  (a productive *d* + numeral rule is not warranted by six lexemes).
- PD2. Plural: `\bd(4|6|8|10|12|20)s\b` finds 0 faces; `(two|three|four|X)
  d(4|6|8|10|12|20)\b` finds 7 (*roll two d6*). The plural is invariant:
  declare it as each lexeme's plural form (as *dice* is for *die*), not as a
  grammar exception.
- PD3. One analysis per string: the die noun is a common noun; it is not also
  a Notation token or a numeral.
- PD4. Newly covered faces are gains. No Reading-count increase on a covered
  face is possible: the words are unknown today, so no covered face contains
  them. Faces that stay unread after the nouns land (a second unknown word,
  *roll … and …* coordination) are listed by name with their next blocker.

## Witnesses

Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0 (vocabulary gap).

- "Roll a d6." (0); "Roll a d8." (0); "Roll a d4." (0); "Roll a d12." (0)
- "Roll two d6 and choose one result." (0; needs PD2)

Controls that must not change: "Roll a d20." 1; "Roll a six-sided die." 1;
"Whenever you roll one or more dice, draw a card." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bd(4|6|8|10|12|20)\b|\bdic?e\b` case-insensitively
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
PD4 for increases. Full `cargo test -p deckmaste_lexical_source` and `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. Lexicon and plugin `grammar:` data load at runtime, so a data-only change
needs no rebuild; a grammar edit costs 2–3 minutes and the six test binaries
about 75 seconds.
Tests assert the role (each die noun a `CommonNoun` head of the Object of *roll*), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Moving die nouns (with *d20*) into a plugin definition:
  `english-v3-plugin-term-definitions`.
- *the result minus 1* (Velukan Dragon), result tables (*1—9 |*), and other
  roll-result constructions.

## Landing record
