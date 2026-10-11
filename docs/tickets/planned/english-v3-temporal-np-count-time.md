---
needs: []
---
# Retire the mass-*time* twin in temporal NPs: The next time you cast a spell this turn, …

## Why

Minted from `english-v3-hygiene-3` item A2. Every "the next time" / "the first
time" face carries each of its Readings twice: the two trees are identical
except for the Countability of *time* under *the* (inherited from
`NounPhrase`; `english-v3-next-time-relative-adjunct`, `szkxvvylquyv`). The
review correction `stymwqqo` removed the mass analysis of *each time* only.
`cargo xtask english-v3 grep -i '\bthe (next|first|second|third|last) time\b'`:
**164** faces, 178 matches; **151** covered and 13 unread in
`target/english-v3/baseline-wave9.json`. No coverage gain is expected: this
lowers Reading counts.

Owner decision, 2026-10-10, on whether a temporal-location NP should require
Count: "seems unhygenic yeah". The mass twin is removed.

## Goal

A temporal-location NP headed by *time* (*the next time …*, *the first time
…*) takes the count reading only. Every affected Reading count falls by
exactly the retired twin; nothing else changes.

## Analysis

In *the next time you cast a spell* the head denotes one occasion: the count
sense. Glossary: **Countability**, **Nominal Adjunct Class**. That the
temporal Adjunct use selects it is a project ruling under the owner decision.

Rulings (orchestrator 2026-10-11):

- TC1. The restriction reads a declared feature: the head's Countability in
  the temporal-adjunct NP. The NP that functions as a temporal Adjunct (the
  route that reads the head's Nominal Adjunct Class) requires the count value
  of the Countability its head already exports. No check names *time*,
  *next*, *first* or any Construction; a guard that would have to name one is
  a STOP.
- TC2. Only the temporal-adjunct use is restricted. *time* keeps both
  Countability values as a Lexeme, and the ordinary NP keeps both: NP "the
  time" 2 and "You have no time." 2 are controls.
- TC3. Every decrease is exactly the retired mass twin, listed by count class
  (2 → 1, 4 → 2, 8 → 4, 16 → 8, 20 → 10; a face where only one of several
  lines halves names the line). Any other decrease, any lost face and any
  increase are defects.
- TC4. *for the first time* (a PP, "Draw a card for the first time." 8) and
  predicative *This is the second time.* (4) are not temporal-adjunct NPs.
  If TC1's feature leaves them unchanged, record their counts; extending the
  restriction to them is a separate ruling.

## Witnesses

Probed 2026-10-11 (`--category sentence --readings 64`, enumeration complete)
with the prebuilt binary of the in-flight `english-v3-slash-pair-predicative`
workspace; re-probe at the claim parent before editing.

| Witness | Now | Expected |
|---|---:|---:|
| "The next time a source of your choice would deal damage to you this turn, prevent that damage." | 20 | 10 |
| "The next time you cast a spell this turn, copy that spell." | 8 | 4 |
| "The first time you would draw a card each turn, draw two cards instead." | 16 | 8 |

NP "the next time" 2 (Countability false / true, otherwise one tree); NP "the
first time" 4. Controls that must not change: "Each time you cast a spell,
draw a card." 2; NP "each time" 1; NP "the time" 2; "You have no time." 2.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\btime\b` (case-insensitive: 428 faces, 70 unread in the
wave-9 baseline) plus 300 random covered controls (`--random 300 --seed
20261008`), `--workers 6 --samples-per-face 0`, compared with `cargo xtask
english-v3 diff --lines` against the claim parent. Zero lost, zero increases;
TC3 for decreases. Full `cargo test -p deckmaste_english_v3` and `cargo clippy
-p deckmaste_english_v3 --tests -- -D warnings` (and `-p
deckmaste_lexical_source` if a declared feature value changes), once, at the
end. Tests assert the role (the temporal Adjunct's head is count; the TC2
controls keep both values), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- `english-v3-hygiene-3` item A7 (*first* as two lexemes): the other factor
  of two on *the first time*.
- The 13 unread faces' other blockers.

## Landing record
