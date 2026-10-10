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

## Findings 2026-10-10 (shelved)

**Orchestrator ruling FC-R1 (2026-10-10): SHELVE.** The 29 gains do not
justify 67 Reading-count increases on covered faces and 456 digest-only
changes. FC5 and the retained-Reading contract cannot be reconciled without a
real licence for where a bare cardinal may head an NP. No grammar or lexicon
change from this attempt is to land. This is a project ruling, not a CGEL
citation. The implementation and all test changes were restored; only this
ticket note is retained. The earlier implementation STOP is superseded by this
ruling. Return this ticket to `planned/` for a later distribution decision.

### Measurement and witnesses

The claim parent was `rmlspxqu`; the attempted change was `wmylrzyv`.
All five named witnesses re-probed at zero before implementation; their
*a card* controls admitted at least three Readings (probe cap three). The six
sentence controls retained their specified counts. After the attempted change,
the five witnesses read, with census counts: *Final Parting* 3, *Codecracker
Hound* 4, *Eye of Yawgmoth* 84, *Underrealm Lich* 13, *Cultivate* 6. These are
per-face sums of line Readings, not Document combinations.

The bounded selector was the Method's case-insensitive pattern
`\b(one|two|three|four|five)( at random)?( (into|onto|on|from|to|at|under|face|and|without|of)\b|[.,]| —)`:
1,095 matching faces plus 300 random covered faces, seed 20261008, with seven
overlapping identities: **1,388 unique faces**. Both runs used the identity
manifest, six workers, zero samples per face, and `/usr/bin/time -v` (before
40.42 s; after 85.90 s). The official `cargo xtask english-v3 diff --lines`
reported **0 lost, 29 gained, 0 decreases, 67 increases, 456 digest-only,
836 unchanged**, no identity/source changes and no internal issues. Covered
faces were 1,075 → 1,104; Reading totals were 50,781 → 52,879.

Evidence was written under
`target/english-v3/english-v3-fused-cardinal-head/`: `before.json`, `after.json`,
`diff.json`, `manifest.json`, `surface.json`, `random.json`, timing reports and
before/after/shape probes. No full corpus census was run. Tests and clippy were
not completed: the test invocation was interrupted at the build lock before
any test binary ran. Ten added tests and both existing test re-spellings were
restored with the implementation; no test removal lands.

### The 29 gained faces

*Claim Territory*; *Clone Shell*; *Codecracker Hound*; *Cultivate*; *Cursed Rack*; *Dimir Charm*; *Eye of Yawgmoth*; *Faerie Snoop*; *Final Parting*; *Flare of Cultivation*; *Fork in the Road*; *Gather the Pack*; *Jarad's Orders*; *Jinxed Choker*; *Kodama's Reach*; *Memories Returning*; *Moment of Truth*; *Navigation Orb*; *Necrodominance*; *Nissa's Pilgrimage*; *Null Profusion*; *Peregrination*; *Reclaim the Wastes*; *Recycle*; *See the Unwritten*; *Sigurd, Jarl of Ravensthorpe*; *Talent of the Telepath*; *Troop of Ponies*; *Underrealm Lich*.

These are census gains, not an assertion that every gained analysis is intended.
In particular, *Cursed Rack*, *Null Profusion*, *Recycle* and *Necrodominance*
admitted their numeric predicates through an NP `NominalComplement` (fused
*four*, *two*, *two*, *five*), rather than a scalar analysis. Those four gains
are separately flagged as unintended-analysis candidates. *Moment of Truth*
read through the already available selected-complement cluster coordination;
this attempt did not add a gapped-coordination construction.

### The 67 increased faces, grouped by shape

Counts below count faces, not new individual Readings. The groups account for
all 67 entries in the official diff. The coordination and partitive groups
collect a family of attachment variants; the examples identify actual probed
shapes, not a claim that every new Reading on a face has the same tree.

**NP Coordination: 36 faces.** A cardinal that formerly participated in a
Cardinal coordination can now form an independent NP conjunct. Examples:
*Counterintelligence* (1 → 2), `Return [one] or [two target creatures]`, beside
the original quantified NP; *Get Out* (8 → 18), the corresponding NP
Coordination in `Return one or two target creatures and/or enchantments` with
further attachment variants. *Arc Lightning* (1 → 5) additionally exposes the
adjacent-Object frame: a fused-cardinal coordination ends one Object before
`targets` becomes another Object, as well as variants inside the selected
`among` Complement. The increases therefore cannot all be attributed to a
modal *Choose* header.

*Aberrant Return* (18 → 36); *Abzan Charm* (4 → 14); *Aerial Volley* (4 → 17); *Ajani, Mentor of Heroes* (108 → 115); *Arc Lightning* (1 → 5); *Archenemy's Charm* (2 → 4); *Armament Dragon* (6 → 13); *Awaken the Maelstrom* (31 → 71); *Biogenic Upgrade* (16 → 40); *Bounty of the Hunt* (7 → 13); *Captain America, First Avenger* (8 → 36); *Counterintelligence* (1 → 2); *Defend the Celestus* (5 → 12); *Deft Dismissal* (1 → 5); *Fire at Will* (1 → 5); *Flames of the Firebrand* (1 → 5); *Flameshot* (2 → 6); *Forked Lightning* (1 → 5); *Gandalf, Spark Starter* (2 → 6); *Gang of Devils* (1 → 5); *Get Out* (8 → 18); *Glint Weaver* (37 → 91); *Ignite Disorder* (1 → 5); *Inferno Titan* (2 → 6); *Magic Missile* (2 → 6); *Mogg Mob* (1 → 5); *Opera Love Song* (6 → 12); *Prismari Charm* (2 → 4); *Ral, Caller of Storms* (3 → 7); *Revival of the Ancestors* (7 → 14); *Shambling Swarm* (4 → 10); *Skirk Volcanist* (2 → 6); *Sundering Stroke* (6 → 30); *Trystan's Command* (10 → 20); *Untimely Malfunction* (21 → 42); *Wurmskin Forger* (2 → 5).

**Partitive splitting / separate noun Postmodifiers: 21 faces.** The existing
partitive NP is retained, but a bare `one` can end an Object or PP Complement,
leaving `of …` to attach independently to a surrounding nominal or predicate.
Examples: *Edric, Spymaster of Trest* (3 → 6), separate `to one` and
`of your opponents` Postmodifiers of `combat damage`; *Brilliant Ultimatum*
(156 → 204), separate `from one` and `of those piles` Postmodifiers of `spells`
in a Clause Coordination analysis. *Legerdemain* also demonstrates a relative
clause whose selected Object ends at `one`, with `of those types` attached to
the surrounding nominal and `with it` outside it. These are not simply a
fused-head-plus-`of` twin inside the partitive NP; banning that local twin did
not exclude these external attachments.

*Backdraft* (110 → 202); *Benthic Anomaly* (33 → 81); *Brilliant Ultimatum* (156 → 204); *Edric, Spymaster of Trest* (3 → 6); *Gauntlets of Chaos* (130 → 222); *Gix, Yawgmoth Praetor* (297 → 300); *Guff Rewrites History* (36 → 72); *Jeska, Thrice Reborn* (40 → 46); *Legerdemain* (39 → 80); *Martial Impetus* (4 → 7); *Oviya, Automech Artisan* (11 → 12); *Puppet Master, String Puller* (6 → 9); *Search the City* (15 → 33); *Seifer, Balamb Rival* (10 → 12); *Shared Fate* (82 → 85); *Spellweaver Helix* (26 → 46); *The Master, Mesmerist* (15 → 24); *The Mimeoplasm* (1096 → 1728); *Triton Tactics* (336 → 456); *Turf War* (46 → 76); *Twilight Diviner* (13 → 31).

**Agent / Adjunct analyses: 8 faces.** A scalar `by` Complement additionally
admits an ordinary PP with a fused-cardinal NP Complement. Examples: *Gnat
Miser* (1 → 4), `by one` as an internalised Agent and as an Adjunct at two
predicate attachment levels, alongside the original Scalar Extent Complement;
*Locust Miser* (1 → 4), the same alternatives for `by two`. The corresponding
`increased by` and other `reduced by` faces follow the same distribution issue.

*Altar of the Pantheon* (3 → 9); *Gnat Miser* (1 → 4); *Locust Miser* (1 → 4); *Minamo Scrollkeeper* (2 → 5); *Thought Devourer* (2 → 5); *Thought Eater* (2 → 5); *Thought Nibbler* (2 → 5); *Trusted Advisor* (3 → 6).

**Other PP Complement: 1 face.** *Keldon Firebombers* (1 → 2) additionally
admits a fused `three` inside `except for three` through
`LicensedMatrixComplementPredicate` / `MatrixComplementPreposition` / an
ordinary PP Complement. This singleton is not a noun Postmodifier or Agent.

**Selected-complement cluster tail: 1 face.** *Telling Time* (36 → 39) already
read before this attempt; the added bare-head licence increases its readings
in the sequence `Put one of those cards into your hand, one on top of your
library, and one on the bottom of your library`. The two bare tails (`one on
top …`, `one on the bottom …`) are the additional candidate sites. This is
listed separately from the split-partitive group rather than treated as a new
covered face.

### Why 456 digests changed without a count change

All 456 digest-only faces contain *choose*. The implementation retired the
lexicon's dedicated *Choose Cardinal* frame and the grammar's
`CardinalComplement` / shared-Cardinal route, replacing them with the existing
selected Object route and `CasePhrase(FusedDeterminerHeadNounPhrase)`.
The generated Reading value therefore contains a Determinative NP Object
instead of the former Numeral/Cardinal complement and uses the surviving
Object frame. Reading fingerprints hash those values and traces, so equal
counts do not imply equal digests. This was the route replacement explicitly
anticipated by FC4(a); it is still part of the rejected trade under FC-R1.
No source-text change caused these digest differences.

### Licence options for a future attempt

Standard constraints apply. These are options requiring a declared-feature
licence and a distribution ruling; none is implemented or proven by this run.
Do not enforce them with a lexeme, verb or card-name guard.

1. **A directly selected, uncoordinated Object licence.** Admit the fused
   cardinal only at a verb frame's selected Object/Complement site, rather than
   projecting it as an unrestricted `NounPhrase`. This fits `put one`,
   `remove one`, `sacrifice one` and `exile one at random`, and can keep generic
   Subject, Agent, Adjunct and noun Postmodifier sites closed. Object-only is
   insufficient by itself: *Counterintelligence*'s extra NP Coordination and
   *Legerdemain*'s split partitive are also selected Objects. The licence must
   specify whether it is direct (not propagated through NP Coordination), and
   how it interacts with the retained integrated partitive construction.
2. **A selected Object plus a frame-selected PP licence.** Require the bare
   cardinal Object to belong to the same selected cluster as a PP licensed by
   that frame (`put one into/on …`, `remove one from …`). This is the narrower
   starting recommendation for the five witnesses: their put/reveal sequence
   supplies the destination PP. It also addresses the *Arc Lightning*
   adjacent-Object defect and excludes ordinary `by one` Agents/Adjuncts and
   split nominal Postmodifiers. It does not cover `Sacrifice one.` or
   `Exile one at random.` without a separate licence, and selected PP frames
   still need inspection for unwanted coordination/partitive decompositions.

Prefer option 2 for a first bounded experiment, with option 1 only where a
separate distribution decision licenses bare selected Objects. Preserve the
existing integrated `one of …` and Cardinal coordination Readings. Decide
explicitly how *Choose* is licensed before retiring its existing route again;
keeping that route during a narrower experiment would avoid conflating the
456 expected replacement digests with the head-distribution question. Neither
option establishes that the increases are ungrammatical: FC-R1 requires a real
licence consistent with the retained-Reading contract, not pruning analyses
merely because they produce an inconvenient count.
