---
needs: []
---
# Read a *times* NP with a non-cardinal quantifier as a frequency Adjunct: You may repeat this process any number of times.

## Why

A cardinal + *times* reads as a frequency Adjunct ("Repeat this process three
times." 1; "Investigate X times." 1; "Investigate twice." 1). The same NP with
any other quantifier is an NP ("any number of times" 1, "that many times" 1
as NounPhrase) but not an Adjunct: "You may repeat this process any number of
times." 0, "Investigate that many times." 0, "You may pay {2}{R} up to three
times." 0. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.json`): `at:times .` (11 of 11).
`cargo xtask english-v3 grep -i '(any number of|that many|as many|or more)
times'` finds **27** faces, all unread; 27 failing lines match, **22** the
only failing line of their face (per shape, overlapping: *N or more times* 11
sole, *any number of times* 9, *that many times* 5, *as many times as …* 3). `english-v3-hygiene-3`
listed "N or more times 11" as unrouted: it is this construction ("This
ability has been activated three times this turn." 6; "… four or more times
this turn." 0), hidden under `at:step .` and `at:turn ,`.

## Goal

Any NP headed by a frequency-unit noun that the grammar already builds reads
as a bounding frequency Adjunct in the positions the cardinal form has,
byte-exact.

## Analysis

CGEL Ch. 8 §9, p. 715: "The class of bounding frequency expressions contains
the adverbs *once* and *twice* (and archaic *thrice*), NPs with *times* as
head, PPs consisting of *on* + NP with *occasion* as head". The passage says
NPs with *times* as head, not cardinal NPs: the quantifier is whatever the NP
takes. Which Oracle quantifiers occur is corpus fact, not CGEL.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`CountedFrequency: FrequencyPhrase` (`[quantity: Cardinal, " ", head:
lexical(Noun)]`, `require head.FrequencyUnit = Yes`, count noun, number
agreement); schema `FrequencyPredicate` with instances over `FinitePredicate`
and `SecondaryVerbPhrase`; `FrequencyPhraseSeries`; feature `FrequencyUse` on
`AdverbPhrase` (*once*, *twice*). The quantified NPs are built by
`QuantifiedNounPhrase` (*that many*), the partitive quantity route (*any
number of*, `english-v3-partitive-quantity-phrases`, landed) and the *up to*
/ comparative-quantity routes.

Orchestrator rulings 2026-10-11:

- TF1. Refine `CountedFrequency`: a second form whose content is a Noun
  Phrase the grammar already builds, required to export the head's
  `FrequencyUnit` (export it from the NP constructions that can carry it if
  it does not reach the phrase level yet). No new Construction (R-budget), no
  second frequency family, and the cardinal form keeps its Reading identity.
- TF2. The licence reads `FrequencyUnit` on the NP's head and nothing else.
  No test on *times*, *any*, *many* or *number*.
- TF3 (R-class). Every quantified NP over a frequency-unit noun is admitted
  (*any number of times*, *that many times*, *four or more times*, *up to
  three times*, *as many times as you choose* if the comparative NP reads),
  not the attested four. Definite and bare NPs (*the times*, *times*) are
  ungrammatical as bounding frequency Adjuncts: exclude them through the
  NP's declared determination, not by listing.
- TF4 (R-one). One analysis per string. The twins to avoid: (a) the *times*
  NP as a second Object of the verb (*pay {2}{R} [any number of times]*
  as Complement); (b) *three times* gaining a second Reading through the NP
  form (the cardinal form is the one route for cardinal + noun: the NP form
  must not also accept a bare cardinal NP; if the compiler cannot separate
  them, retire the cardinal form and re-spell its tests instead, and say so);
  (c) the multiplier *three times* + NP (predeterminer) when an NP follows.
- TF5. *an additional time* (*Investigate an additional time.* 0; 9 sole
  faces; *that ability triggers an additional time* reads, 33 covered faces):
  first record how the covered faces read. If they use a frequency Adjunct
  that the verbs here refuse, it is this ticket; if they read through
  *trigger*'s own frame, list it and leave it.
- TF6. Newly covered faces are gains. R-reach: `CountedFrequency` changes
  form, so the selector includes every face whose baseline uses it (`\btimes?\b`
  and `\b(once|twice)\b` surfaces). No Reading-count increase is expected on
  a covered face; TF4(b) names the shape to look for, and it is a defect.

## Witnesses

Each is a sentence of the face's only failing line. Probed 2026-10-11 with the
trunk binary (`--category sentence --readings 8`): all 0.

- Ad Nauseam: "You may repeat this process any number of times."
- Bloodthirsty Adversary: "When this creature enters, you may pay {2}{R} any
  number of times."
- Piper Wright, Publick Reporter: "Whenever Piper Wright deals combat damage
  to a player, investigate that many times." (probe with `--as-card`; the
  reduced "Investigate that many times." is 0)
- Trade Secrets: "That opponent may repeat this process as many times as
  they choose."
- Dragon Whelp: "If this ability has been activated four or more times this
  turn, sacrifice this creature at the beginning of the next end step."

Controls that must not change: "Repeat this process three times." 1;
"Investigate twice." 1; "Investigate X times." 1; "You may repeat this
process." 1; "When this creature enters, you may pay {2}{R}." 1; "This
ability has been activated three times this turn." 6; NounPhrase
"any number of times" 1, "that many times" 1, "three times" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\btimes?\b|\b(once|twice)\b` plus 300 random covered
controls (`--random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent. Zero
lost; TF6 for changes. Full `cargo test -p deckmaste_english_v3` and clippy
`--tests -- -D warnings`, once, at the end (a grammar rebuild takes 2–3
minutes, the six test binaries about 75 seconds). Tests assert the role (the
*times* NP is a frequency Adjunct of the predicate; TF4 negatives), never an
incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- *for each time vote* (Expropriate): a council label.
- *each time you surveil* (Enhanced Surveillance): temporal NP relative.
- *the next time* (landed), *for the first time each turn* (landed).
- *three times X life*: `english-v3-multiplied-variable-quantity`.

## Landing record
