---
needs: []
---
# Read a bare cardinal as a fused Determiner-Head: Put one into your hand and the other into your graveyard.

Split from the wave-8 item "fused cardinal and ordinal heads": the cardinal
fuses Determiner and Head, the ordinal fuses Modifier and Head under a
Determiner (`english-v3-fused-ordinal-head`). Two functions, two routes.

## Why

A cardinal with no noun after it is an NP only in the partitive (*one of
them* reads). Bare, it reads nowhere except as a dedicated Complement of
*choose* (*Choose one.* 1; *Put one into your hand.* 0; *Sacrifice one.* 0).
Scoped out of `docs/tickets/done/english-v3-fused-head-noun-phrases.md`, which
has since landed the *the other* / *the rest* half. `cargo xtask english-v3
grep -i '\b(one|two|three|four) (into|onto|on|from|to|at|under|face) '` finds
**33** faces (29 unread); 28 failing lines match, **24** of them the only
failing line of their face (2 of the 24 are *choose one at random —* headers,
not this). 17 of the 24 continue with *the other* / *the rest*, which now
read, so the cardinal is the last blocker there.

## Goal

A cardinal Determinative heads an NP with no overt noun, as Object or
Complement of a Preposition: *put one into your hand*, *put one onto the
battlefield tapped*, *remove one from it*, *one into your graveyard, and one
on the bottom of your library*. Number follows the cardinal (*one* singular,
*two* plural).

## Analysis

CGEL Ch. 5 §4, p. 356, [vi]: "*He gave ten copies to me and* [*six*] *to the
others*", labelled "simple" fused head. Ch. 5 §9.3(a), p. 415: "Those
determinatives which can function as internal modifier as well as determiner
can in general occur as fused modifier-head as well as determiner-head".
Those back a cardinal as fused head of an NP. Which route the grammar uses,
and the fate of the *choose* frame, are project rulings. Glossary: **Fused
Determiner-Head**.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`FusedDeterminerHeadNounPhrase` (`[head: lexical(Determinative)]`, `require
head.FusedHeadUse = Yes`, `require head.DeterminerUse = Singular`, `export
number = Singular`), declared today by `vocab:SingularDemonstrative/This` and
`vocab:CompoundDeterminative/Anywhere` (`lexicon/core.ron`). The cardinal is
`vocab:CardinalDeterminative/One` etc. The partitive *one of them* has its
own Construction (`head.DeterminerUse = SingularCount`, plural oblique).
*Choose one.* reads through a `Cardinal` frame of `core-verb:Choose`, not
through an NP; 454 covered faces match `[Cc]hoose (one|two|three)( —|\.)`.

Orchestrator rulings 2026-10-10:

- FC1. Refine `FusedDeterminerHeadNounPhrase`: admit the cardinal
  Determinatives and export the number the head declares, instead of
  requiring and exporting Singular. No new Construction.
- FC2. The licence is declared: the cardinal declares `FusedHeadUse` (data,
  `lexicon/core.ron`) and its `DeterminerUse` / number feature drives
  agreement. No list of numerals in the grammar and no test on the verb.
- FC3. Declare only the cardinals attested bare (grep per word; *one* is
  nearly all of it, *two* / *three* a handful). Do not add a check whose only
  job is to block the rest.
- FC4. One analysis per string. The twins to avoid: (a) **Choose one.** /
  *choose two —* keep exactly their Readings on all covered modal faces. If
  the fused NP also satisfies an Object frame of *choose*, the string gains a
  second Reading: that is a defect. Resolve it by having one route only:
  either the `Cardinal` frame of *choose* is retired and its tests re-spelled
  against the fused NP (same cards, same asserted outcome; digest-only
  changes on covered faces are then expected and reported), or the fused NP
  is not an Object of *choose*. Retiring is preferred if every string the
  frame reads still reads with the same count; STOP with the diff if not.
  (b) *one of them* keeps exactly 1 Reading: no fused head + *of* PP
  postmodifier twin. (c) *one or more creatures*, *one creature*, *one other
  creature* keep their Readings: the fused head takes no following nominal.
  (d) *one* the common noun (`lexeme:CommonNoun/One`, *the chosen one*) gains
  no twin.
- FC5. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected; the only disclosed change is the
  digest-only one of FC4(a). Any increase is reported with its shape before
  landing.

## Witnesses

Each is the face's only failing line. Probed 2026-10-10 on the `uoqyuxtnowpo`
binary (`--readings 3 --as-card`): 0; control is the same Document with *a
card* for the cardinal.

- Final Parting: "Search your library for two cards. Put one into your hand
  and the other into your graveyard. Then shuffle." 0 (*a card* 3)
- Codecracker Hound: "When this creature enters, look at the top two cards of
  your library. Put one into your hand and the other into your graveyard."
  0 (3)
- Eye of Yawgmoth: "{3}, {T}, Sacrifice a creature: Reveal a number of cards
  from the top of your library equal to the sacrificed creature's power. Put
  one into your hand and exile the rest." 0 (3)
- Underrealm Lich: "If you would draw a card, instead look at the top three
  cards of your library, then put one into your hand and the rest into your
  graveyard." 0 (3)
- Cultivate: "Search your library for up to two basic land cards, reveal
  those cards, put one onto the battlefield tapped and the other into your
  hand, then shuffle." 0 (3)

Sentence probes, 0 today: "Put one into your hand."; "Put two into your
hand."; "Sacrifice one."; "Exile one at random." Controls that must not
change: "Choose one." 1; "Choose two." 1; "Put one of them into your hand."
1; "Put one of them into your hand and the other into your graveyard." 1;
"Put the other into your graveyard." 1; "Put a card into your hand and the
rest into your graveyard." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(one|two|three|four|five)( at random)?( (into|onto|on|from|to|at|under|face|and|without|of)\b|[.,]| —)

(1,095 faces: the bare and partitive cardinals and every *choose one*
header) plus 300
random covered controls, `--workers 6 --samples-per-face 0`, compared with
`cargo xtask english-v3 diff` against the claim parent. Zero lost; FC4 and
FC5 for everything else. Full `cargo test -p` and `cargo clippy -p … --tests
-- -D warnings` for `deckmaste_english_v3` and `deckmaste_lexical_source`,
once, at the end: a rebuild after a grammar edit takes about 3 minutes and
the test binary about 7, so batch edits. Tests assert the role (the cardinal
is the fused Determiner-Head of an NP Object; number agreement; the FC4
negatives), never an incidental count or a Debug-printed tree. Attestation by
`cargo xtask english-v3 grep` only. Amend the glossary entry **Fused
Determiner-Head** if its wording is singular-only.

## Out of scope

- The ordinal: *for each target beyond the first*
  (`english-v3-fused-ordinal-head`).
- *Choose one or both —*, *one or more —*, *up to one —* headers (about 79
  unread faces, listed in `english-v3-named-modes`). If they start reading as
  a consequence, report them as gains with the selected analysis; do not
  extend this ticket to make them read.
- *Put a charge counter on this artifact or remove one from it.* (4 sole
  lines) is in scope only if the fused head is all it needs.
- *one into your graveyard, and one on the bottom of your library* (Moment of
  Truth): gapped coordination beyond what has landed.
- Bare *some* / *each* / *any* as heads (*Put each into your hand.* 0).

## Landing record

The six items in `docs/tickets/README.md` ("Landing record").
