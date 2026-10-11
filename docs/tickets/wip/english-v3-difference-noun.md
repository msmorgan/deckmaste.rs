---
needs: []
---
# Declare the scalar noun *difference*: draw cards equal to the difference / where X is the difference between its power and toughness

Expected to be a data ticket: a noun declaration and one nominal frame.

## Why

*difference* is not a word of the lexicon. The positions it fills read with
*number*: "You draw cards equal to the number." 1, "X is the number." 1,
against "You draw cards equal to the difference." 0 and NP "the difference
between its power and its toughness" 0. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): `word:difference` 28 sole, 32
touched. `cargo xtask english-v3 grep -i '\bdifference\b'` finds **32**
faces, all unread; **29** have that line as their only failing line. By
shape: anaphoric *the difference* closing its phrase,
`\bthe difference[.,"]`, **26** faces (23 sole): *equal to the difference*
24 (one of them *less than or equal to the difference*), *where X is the
difference* 2;
*the difference between* NP *and* NP, 6 faces (6 sole).

## Goal

*the difference* reads as a scalar-denoting NP where *the number* does
(Complement of *equal to*, predicative Complement of *X is*), and takes a
*between* Complement (*the difference between that creature's power and its
toughness*), with byte-exact realisation.

## Analysis

No CGEL claim is relied on: the noun's scalar denotation and its selected
*between* Complement are lexical declarations (project rulings). The
anaphoric reading (the difference between the two quantities just compared)
is interpretation, not syntax.

Nearest existing declarations (`crates/deckmaste_lexical_source/lexicon/
core.ron`): `lexeme:CommonNoun/Number` (`NumberTransparency: Yes`,
`ScalarDenotation: Yes`), the noun that reads in these positions;
`vocab:Preposition/Between` (`PrepositionFunctionLicence:
NounComplement|PredicativeComplement`, `PrepositionComplement: NounPhrase`),
which is all but unread today (14 faces contain *between*, 13 unread; NP "a
number between one and five" 0). Grammar
(`crates/deckmaste_english_v3/src/declarations.rs`):
`PrepositionComplementNominal` (`[head: lexical(Noun), " ", marker:
lexical(Preposition), " ", complement: AccusativePhrase]`, `head.frame =
NominalPreposition`), the route for a noun's selected PP Complement.

Orchestrator rulings 2026-10-10:

- DN1. Declare `lexeme:CommonNoun/Difference` in `lexicon/core.ron`: count
  noun, singular attested only (*differences* 0 faces: declare the plural
  unavailable or leave the default per the pruning rule for hand-declared
  forms, and say which), `ScalarDenotation: Yes`. Do not copy
  `NumberTransparency`: *the difference* never takes an *of* NP whose number
  shows through (0 faces for *difference of*).
- DN2. The *between* Complement is a declared nominal frame on the noun
  (`NominalPreposition` with marker *between*), read by
  `PrepositionComplementNominal`. No Construction is added and *between* is
  not made a free NP postmodifier.
- DN3. If the frame alone does not read the six *between* faces (the
  Complement is a coordination of two NPs, one of them genitive-headed:
  *that spell's mana value and that nonland card's mana value*), land DN1 and
  return those faces with the probe evidence; do not STOP the ticket and do
  not add a coordination form here.
- DN4. One analysis per string: *equal to the difference* has the structure
  of *equal to the number*; *where X is the difference between A and B* has
  *between A and B* inside the NP, not as a clause Adjunct (the declared
  licence of *between* has no Adjunct function).
- DN5. Newly covered faces are gains; no Reading-count change on a covered
  face is expected (neither word occurs on one). Report any.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Balance of Power: "If target opponent has more cards in hand than you, draw
  cards equal to the difference."
- Kozilek, the Great Distortion: "When you cast this spell, if you have fewer
  than seven cards in hand, draw cards equal to the difference."
- Shadow of Mortality: "If your life total is less than your starting life
  total, this spell costs {X} less to cast, where X is the difference."
- Ancient Cellarspawn: "Whenever you cast a spell, if the amount of mana
  spent to cast it was less than its mana value, target opponent loses life
  equal to the difference."
- Jaws of Defeat (DN2): "Whenever a creature you control enters, target
  opponent loses life equal to the difference between that creature's power
  and its toughness."
- Doran, Besieged by Time (DN2): "Whenever a creature you control attacks or
  blocks, it gets +X/+X until end of turn, where X is the difference between
  its power and toughness."

Controls that must not change: "You draw cards equal to the number." 1; "X is
the number." 1; Smellerbee, Rebel Fighter "Whenever Smellerbee attacks, you
may discard your hand. If you do, draw cards equal to the number of attacking
creatures." 2 (census); Gregor, Shrewd Magistrate "Whenever Gregor, Shrewd
Magistrate deals combat damage to a player, draw cards equal to its power." 3
(census); Lydia Frye "At the beginning of your end step, surveil X, where X
is the number of tapped Assassins you control." 3 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\bdifference\b|\bbetween\b|\bequal to (the|that) (number|amount|total)[.,]

(41 faces) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost, zero changed among covered faces. `cargo test
-p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end;
lexicon data loads at runtime, so DN1–DN2 need no grammar rebuild. Tests
assert the role (the noun as scalar Complement of *equal to*; *between* PP
as the noun's Complement), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *the total* in the same positions ("You draw cards equal to the total." 0
  today although *total* is declared): a separate defect; report it if the
  bounded census shows the cause.
- Lines with a second blocker: Spry and Mighty (*exactly*), Scholarship
  Sponsor and Tales of the Ancestors (`at:the most`), Conformer Shuriken
  (*greater power than*), Hit the Mother Lode, Lolth, Transmute Artifact.
- Other uses of *between* (*a number between* …).

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
