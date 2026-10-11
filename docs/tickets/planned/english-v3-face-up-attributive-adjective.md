---
needs: []
---
# Declare the attributive adjective *face-up*: separate them into a face-down pile and a face-up pile.

## Why

Data ticket. *face-down* is declared as an attributive adjective and reads
(NP "a face-down pile" 1, NP "a face-down creature" 1); its sibling
*face-up* is not (NP "a face-up pile" 0, NP "a face-up creature" 0). The
intransitive prepositions *face up* / *face down* are both declared ("It
enters face up." 1, "Turn target face-down creature face up." 1). Blocker
report at trunk `qzprwuxz` (`target/english-v3/blockers-wave9.txt`): failing
token `face-up` 15 sole, 16 touched; key `at:a face-up` 9, 10. `cargo xtask
english-v3 grep 'face-up'` finds **17** faces, all unread; **15** have that
line as their only failing line (*a face-down pile and a face-up pile* / *in
a face-up pile* 7, *face-up exiled card(s)* 4, *a face-up … card you own in
exile* 2, others 4).

## Goal

*face-up* reads as an attributive adjective wherever *face-down* does, with
byte-exact realisation.

## Analysis

No CGEL claim is needed: a second member of an existing declared class.
Existing declaration:
`crates/deckmaste_lexical_source/lexicon/vocabulary.rs`, `vocab
AttributiveAdjective { feature HomographLicense = Unlicensed; feature
ModifierLicense = Unrestricted; … FaceDown = "face-down", … }`; the
prepositions `vocab:Preposition/FaceDown` and `vocab:Preposition/FaceUp` in
`lexicon/core.ron`.

Orchestrator rulings 2026-10-11:

- FU1. Declare `FaceUp = "face-up"` beside `FaceDown` with the same
  features. Nothing else: the licence is the class (R-class). No Construction,
  frame or check.
- FU2. One lexical owner each: the hyphenated adjective and the spaced
  preposition stay distinct lexemes, as they are for *face-down* / *face
  down*.
- FU3. One analysis per string (R-one): *a face-up pile* reads exactly as
  many ways as *a face-down pile* (1).
- FU4. Newly covered faces are gains. No covered face contains *face-up*, so
  no Reading-count change on a covered face is expected; the 41 covered
  *face-down* faces must keep their counts and digests.

## Witnesses

Each is the only failing line of its face in `blockers-wave9.json`. Whole
faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- Riddles in the Dark: "Look at the top four cards of your library and
  separate them into a face-down pile and a face-up pile. …"
- Fortune's Favor: "Target opponent looks at the top four cards of your
  library and separates them into a face-down pile and a face-up pile. …"
- Karn, the Great Creator: "−2: You may reveal an artifact card you own from
  outside the game or choose a face-up artifact card you own in exile. …"
- Binding Negotiation: "… Otherwise, you may put a face-up exiled card they
  own into their graveyard."
- Abstract Performance: "Exile the top four cards of your library in a
  face-down pile, then exile the top four cards of your library in a face-up
  pile. …"

Sentence probes: "Exile the rest in a face-up pile." 0; "Separate them into a
face-down pile and a face-up pile." 0. Controls that must not change: "Exile
any number of them in a face-down pile." 2; NP "a face-down creature" 1;
"Turn target face-down creature face up." 1; "It enters face up." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `face-(up|down)` (63) plus 300 random covered controls (`cargo
xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; FU4. Full `cargo test -p
deckmaste_lexical_source` and `-p deckmaste_english_v3` with clippy `--tests
-- -D warnings`. `vocabulary.rs` is compiled, so this data edit needs the 2–3
minute rebuild; the six test binaries take about 75 seconds. Tests assert the
role (*face-up* as attributive Modifier of the head noun, the coordinated
*a face-down pile and a face-up pile*), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Lines with a second blocker (2 of the 17 faces: The Celestial Toymaker,
  Expose the Culprit).
- *up to seven face-up exiled cards you own* (Mirror of Fate) and *from among
  face-up exiled cards* if the remainder fails: report, do not chase.
- Moving the face-status words into a plugin definition
  (`english-v3-plugin-term-definitions` family).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
