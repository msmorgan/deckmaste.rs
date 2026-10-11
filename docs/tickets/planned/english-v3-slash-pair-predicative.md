---
needs: []
---
# Read a bare power/toughness pair as predicative Complement: create a token that's a copy of that creature, except it's 1/1.

## Why

A power/toughness pair reads before a noun ("It's a 1/1 creature." 1; "It
becomes a 1/1 creature." 1) and after *base power and toughness* as a
property value ("That creature has base power and toughness 1/1." 2). Alone
after *be* or *become* it reads nowhere: "It's 1/1." 0, "It is 1/1." 0, "The
token is 1/1." 0, "Its base power and toughness become 1/1." 0. Blocker
report at trunk `qzprwuxz` (`target/english-v3/blockers-wave9.json`):
`at:1 .` (3), `at:2 .`, `at:2 until`, `at:4 until`. `cargo xtask english-v3
grep -i "\bis \d+/\d+|\bare \d+/\d+|(it|they).(s|re) \d+/\d+|becomes?
\d+/\d+"` finds **54** faces, **24** unread; 23 failing lines match, **22**
the only failing line of their face. The 30 covered faces all have a noun
after the pair (*become 2/2 creatures*, *They're 2/2 Horror creatures*). Sole
lines by shape: *except it's / the token is N/N* 13; *base power and
toughness become N/N* 6; *if that creature is 1/1* 1; the other 2 have a noun
after the pair and fail elsewhere. So **20** sole faces are this construction.
`english-v3-hygiene-3` listed "except it's N/N 13" as unrouted.

## Goal

A bare pair reads as predicative Complement of *be* and *become* for (a) an
object Subject (*it's 1/1*, *the token is 1/1*) and (b) the Subject *base
power and toughness* (*… become 4/2 until end of turn*), byte-exact.

## Analysis

CGEL Ch. 8 §5.4, p. 693: "Simple location on a scale is commonly expressed by
*be* with an NP as predicative complement", with the dimension example [12iii]
*My other table is six foot by four*. That backs a bare measure expression as
predicative Complement of *be*. Treating Oracle's `N/N` pair as such a measure
expression, and extending it to *become*, is a project ruling.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`SlashPair` (`UnsignedScalar` members); `SlashModifiedNominal` (`[modifier:
SlashPair, " ", head: Nominal]`); the property-value route behind *base
power and toughness 1/1* (`PropertyValuePosition` on *power* / *toughness*);
`ScalarLocation` / `ScalarLocationPredicate` (`selected_frame(head,
ScalarLocation)`, Subject `ScalarDenotation = Yes`), which reads *its power
is 0*; *become*'s `Predicate([PredicativeComplement])` frame.

Orchestrator rulings 2026-10-11:

- SP1. Two Subjects, one Complement. (b) *base power and toughness
  become/are N/N* is scalar location with a paired scale: refine the
  scalar-location frame so a coordinated pair of `ScalarDenotation` property
  nouns takes a `SlashPair` value. (a) *it's 1/1* ascribes the pair to an
  object: the pair is admitted as predicative Complement where the Subject is
  not scalar. Do (b) through `ScalarLocationPredicate`; do (a) by admitting
  `SlashPair` as a predicative Complement Category of the copular frames. No
  new Construction is expected (R-budget); if (a) needs one, it is ONE
  projection (`SlashPair` → predicative Complement), declared up front: it
  buys 14 sole faces.
- SP2. The licence reads the Complement's Category (`SlashPair`, unsigned)
  and, for (b), the Subject's declared `PropertyValuePosition` /
  `ScalarDenotation`. No test on *it*, *token*, *copy* or the digits. A
  signed pair (*+1/+1*) is an adjustment, not a size: it must not become
  predicative ("It is +1/+1." stays 0).
- SP3 (R-class). Every copular verb that declares the predicative frame gets
  the Complement (*be* in every form, including clitic *it's* / *they're*,
  and *become*), not the attested forms.
- SP4 (R-one). One analysis per string. The twins to avoid: (a) *it's 1/1*
  as *it's a 1/1 creature* with an elided head (no fused-head route); (b)
  *become 2/2 creatures* (covered, 30 faces) gaining a Reading where the bare
  pair is the Complement and *creatures* dangles; (c) the pair read as two
  numerals around a slash coordination.
- SP5. Depends on nothing, but overlaps
  `english-v3-scalar-location-plain-be-and-become` (*become* selecting the
  scalar frame). Whichever lands second reuses the first's *become* frame and
  does not add another.
- SP6. Newly covered faces are gains. R-reach: if the copular frames or
  `ScalarLocationPredicate` change, the selector includes every face with a
  `N/N` pair. No Reading-count increase is expected on a covered face; SP4(b)
  names the shape to look for, and it is a defect.

## Witnesses

Each is a sentence of the face's only failing line. Probed 2026-10-11 with the
trunk binary (`--category sentence --readings 8`): all 0.

- Echoing Assault: "Create a token that's a copy of that creature, except
  it's 1/1."
- Endless Evil: "At the beginning of your upkeep, create a token that's a
  copy of enchanted creature, except the token is 1/1."
- Offspring's Revenge: "Create a token that's a copy of that card, except
  it's 1/1."
- Brine Hag: "When this creature dies, the base power and toughness of each
  creature that dealt damage to it this turn become 0/2."
- Mirkwood Meditator: "Whenever a land you control enters, you may have this
  creature's base power and toughness become 4/2 until end of turn." (the
  causative control "… you may have this creature become a 4/2 creature until
  end of turn." reads 3)

Also 0, and must stay 0 (SP2): "It is +1/+1." Controls that must not
change: "It's a 1/1 creature." 1; "It becomes a 1/1
creature." 1; "That creature has base power and toughness 1/1." 2; "Create a
token that's a copy of that creature, except it's an artifact." 2; "Create a
token that's a copy of that creature, except it isn't legendary." 2; "It
gets +1/+0." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b\d+/\d+\b|\bX/X\b` (the surface the change can touch)
plus 300 random covered controls (`--random 300 --seed 20261008`), `--workers
6 --samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; SP6 for changes. Full `cargo test -p
deckmaste_english_v3` and clippy `--tests -- -D warnings` (plus `-p
deckmaste_lexical_source` if frames are declared), once, at the end (a
grammar rebuild takes 2–3 minutes, the six test binaries about 75 seconds;
lexicon data loads at runtime). Tests assert the role (the pair is the
predicative Complement; SP2 and SP4 negatives), never an incidental count or
a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *it becomes your choice of 5/1 or 1/5* (Aquamorph Entity); *become 4/1 or
  1/4* (Master of Winds) unless pair coordination falls out.
- *If X is 6 or more, those permanents are 4/4 creatures in addition to
  their other types* (Dance of the Manse): fails elsewhere.
- Plural-name Subjects (*Moon Girl and Devil Dinosaur's base power and
  toughness become 6/6 and they gain trample*).

## Landing record
