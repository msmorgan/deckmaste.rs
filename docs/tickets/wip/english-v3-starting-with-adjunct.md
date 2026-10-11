---
needs: []
---
# Read the order Adjunct *starting with NP*: Starting with you, each player chooses a creature.

## Why

*Starting with you, …* reads nowhere: *starting* is known only as the
attributive Adjective of *starting life total*. Blocker report at trunk
`ymqwzour` (`target/english-v3/blockers-wave8.txt`): key `at:starting with`
20 sole-blocker faces. `cargo xtask english-v3 grep -i '\bstarting with\b'`
finds **46** faces, all unread; **43** have that line as their only failing
line. 27 of the 46 lines continue *each player votes for X or Y*, which is
unread on its own ("Each player votes for death or taxes." 0), so the clean
set is the **18** sole-line faces without a vote. Positions: line- or
sentence-initial (32); after a preposed *when* / *if* clause (*When this
creature enters, starting with you, each player may …*, 12); after *Then*
(1); after the Subject (*each player starting with you may pay …*,
Mana-Charged Dragon, 1). The Complement of *with* is *you* (43) or *the next
opponent in turn order* (3).

## Goal

*starting with NP* reads as a preposed Adjunct of the Clause, headed by a
deverbal Preposition *starting* whose Complement is a *with* Preposition
Phrase, with byte-exact realisation.

## Analysis

CGEL Ch. 7 §2.3, p. 610: "a number of prepositions … have arisen through the
conversion of secondary, non-tensed, forms of verbs", [21i] "[*Barring
accidents,*] *they should be back today.*", [21iii] "[*Pertaining to the
contract negotiations,*] *there is nothing to report.*"; "The basis for
analysing the underlined words here as prepositions is that there is no
understood subject." P. 611, [24] and the text after it: such items "take
an obligatory complement – an NP, except for those marked T or F, which take
a *to* or *for* phrase". Those back a participle-form Preposition taking a
PP Complement and heading a fronted Adjunct. *starting* is not in CGEL's
list [24] ("the main prepositions"); treating it as one, on the
no-understood-subject criterion (no participant "starts": the order of the
process does), is a project ruling.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`PrepositionComplementPreposition` (a Preposition taking a Preposition
Phrase Complement, licensed by `head.PrepositionComplement` containing
`PrepositionPhrase` and the table
`licensed_preposition_phrase_complement(PrepositionPhraseComplementLicence,
…)`, values `Unrestricted`, `Source`; *from* declares it for *from among
them*); the preposed Adjunct route read from `PrepositionFunctionLicence`
containing `PreposedAdjunct` (*During your turn, each player may pay any
amount of mana.* 1). Lexicon: `vocab:AttributiveAdjective/Starting`; no
Verb *start* is declared and none is needed.

Orchestrator rulings 2026-10-10:

- SW1. Data first: declare a Preposition Lexeme *starting* (`lexicon/
  core.ron`) with `PrepositionComplement: PrepositionPhrase`, a
  `PrepositionPhraseComplementLicence` value that selects a *with* phrase,
  and `PrepositionFunctionLicence: Adjunct|PreposedAdjunct`. It is read by
  `PrepositionComplementPreposition`; no new Construction. Not a Verb
  *start* with a gerund-participial Adjunct.
- SW2. The licence reads declared features: the outer Preposition's
  Complement licence and the inner Preposition's declared marker (the
  feature `licensed_preposition_phrase_complement` already compares). If
  that table cannot express "a *with* phrase" without naming *with* in the
  grammar, extend the licence feature with a value the inner Preposition
  declares in data. No word test.
- SW3. Positions: the preposed position (clause-initial, and after a
  preposed subordinate clause or *Then*) is in scope and should come from
  `PreposedAdjunct` with no further declaration. The post-Subject position
  (1 face) and the clause-final position (0 faces) are in scope only if the
  same licence reads them; otherwise list the count and leave it. Do not add
  a check whose only job is to block a position.
- SW4. One analysis per string. The twins to avoid: (a) *starting* read as
  the attributive Adjective with *with you* as a separate Adjunct;
  (b) *with you* attaching to the main verb (*chooses … with you*) while
  *starting* is skipped or absorbed; (c) *starting life total* gaining a
  Preposition Reading.
- SW5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: the 18 covered *starting* faces (`(?i)\bstarting\b`: 84
  faces, 66 unread) are *starting life total* (13) and *starting deck* (4)
  shapes and must keep their counts (SW4(c)). Report any increase with its shape
  before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0; control = the sentence
without the Adjunct. Re-probe on the claim parent first.

- The Horus Heresy: "III — Starting with you, each player chooses a
  creature. Destroy each creature chosen this way." (control "Each player
  chooses a creature." 1)
- Eureka: "Starting with you, each player may put a permanent card from
  their hand onto the battlefield. Repeat this process until no one puts a
  card onto the battlefield." (control, first sentence 2)
- Whims of the Fates: "Starting with you, each player separates all
  permanents they control into three piles. Then each player chooses one of
  their piles at random and sacrifices those permanents." (control, first
  sentence 1)
- Druid of Purification: "When this creature enters, starting with you, each
  player may choose an artifact or enchantment you don't control. Destroy
  each permanent chosen this way." (control, first sentence 2)
- Plague of Vermin, first sentence: "Starting with you, each player may pay
  any amount of life." (control 1)

Controls that must not change: "During your turn, each player may pay any
amount of mana." 1; "As this creature enters, each player may pay any amount
of mana." 1; "with you" (`preposition-phrase`) 1; the *starting life total*
faces in the census.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bstarting\b` (84 faces) plus 300 random covered controls
(`--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`,
compared with `cargo xtask english-v3 diff` against the claim parent. Zero
lost; SW5 for increases. `cargo test -p` and `cargo clippy -p … --tests --
-D warnings` for `deckmaste_lexical_source`, and for `deckmaste_english_v3`
if the licence feature changes (a rebuild after a grammar edit takes about
2–3 minutes and the six test binaries about 75 seconds; lexicon data loads
at runtime). Tests assert the role (the Preposition *starting* heading a
preposed Adjunct, the *with* phrase as its Complement, the SW4 negatives),
never an incidental count or a Debug-printed tree. Attestation by `cargo
xtask english-v3 grep` only. Glossary: no new term expected; cite the SW1
ruling under **Preposition Function Licence** if a licence value is added.

## Out of scope

- *each player votes for X or Y* (27 of the 46 lines): the vote frame and
  its named options.
- *the next opponent in turn order* (3 faces; `noun-phrase` 0 today).
- *Starting with you and proceeding in the chosen direction* (Order of
  Succession, 1 face).
- Council's dilemma / Join forces label handling beyond what reads today.

## Landing record

1. **Census delta** (`wxxvsntt`, BEFORE before editing, AFTER with this change;
   claim `nltwmkvp`): bounded selector `(?i)\bstarting\b`, 84 attested faces
   plus 300 covered controls, seed 20261008, one overlap: 383 faces.
   Covered 318 → 331; Readings 6,590 → 6,821; gained 13, lost none.
   This ticket gains 12; difference-noun refresh change `ropwqyov` gains
   Shadow of Mortality (0 → 1), with *the difference* read as its declared
   scalar noun phrase (byte-exact, zero issues).
   decreases 0, increases 0, digest-only 1, issues 0. The sole digest change
   is Janjeet Sentry (2 → 2): refresh brought declared mass countability on
   its invariant {E} Words from quantified-energy change `tznkrzpw`.
   The official AFTER-to-refreshed-AFTER diff attributes that change to trunk;
   this ticket’s pre-refresh diff had digest-only 0. Existing covered
   *starting* faces retain their complete Reading sets. Wrong-analysis gains: 0.
2. **Witnesses** (`wxxvsntt`): The Horus Heresy 0 → 2 (control 2), Eureka
   0 → 2 (control 2), Whims of the Fates 0 → 1 (control 1), Druid of
   Purification 0 → 4 (control 4), Plague of Vermin's first sentence
   0 → 1 (control 1). Counts are full-passage probes, not per-line face sums.
   None already read on the claim parent. During, As, *with you*,
   *starting life total*, and *starting deck* controls remain 1.
   All 12 ticket-gained-face full-document probes match their Adjunct-free controls
   (W9-A); in particular Collective Voyage 120/120 and The Horus Heresy 48/48.
3. **Spot checks** (`wxxvsntt`): Alliance of Arms, Collective Voyage,
   Disciple of Caelus Nin, Druid of Purification, Eureka, Hypergenesis,
   Minds Aglow, Shared Trauma, The Horus Heresy, The Legend of Yangchen.
   Each selected Reading has InitialPreposition with a deverbal Preposition
   *starting* whose Complement is the complete *with you* PP; all are
   byte-exact.
4. **Deviations and STOPs**: no new or retired Construction or Frame, no
   loader changes, no design STOP. Both refresh append conflicts in
   `suite_2.rs` were resolved by retaining the imported modules and
   starting-with module once each. Standing pre-integration review is pending. SW2 requires the With Complement licence and
   declared Preposition Phrase Complement Marker, propagated through PP heads
   and Coordination; the glossary defines the marker and cites SW1 under
   Preposition Function Licence. The existing attributive Adjective remains.
   SW3: post-Subject Mana-Charged Dragon (1 face) remains unread; the sole
   Then face, Rejoin the Fight, retains its out-of-scope *next opponent in
   turn order* gap; clause-final attestation 0. Plague of Vermin's later text
   still prevents full-face coverage. Seed 20261008 follows the binding budget.
5. **Tests** (`wxxvsntt`): `cargo test -p deckmaste_english_v3` passed
   (1,027 before refresh; 1,034 after energy; 1,037 after difference);
   `cargo test -p deckmaste_lexical_source` passed (88 tests, all runs).
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed
   before and after both refreshes.
   An initial test compilation needed an explicit expected Lexeme ID type;
   clippy required Copy use and a value comparison for the empty vector.
   For this change: restored 0 / re-spelled 0 / ignored 0 / added 9 / removed 0.
   `cargo build -p xtask` passed; cite checks report 0 noncompliant, 0 stale;
   `jj --no-pager diff --git | cargo xtask cite audit --diff` found no new
   citation sites. Evidence is under `target/english-v3/english-v3-starting-with-adjunct/`.
6. **Timing** (`wxxvsntt`): final refreshed bounded AFTER census 21.58 s wall under
   `/usr/bin/time -v`, 6 workers, samples-per-face 0. No full corpus run,
   per the binding verification budget.
