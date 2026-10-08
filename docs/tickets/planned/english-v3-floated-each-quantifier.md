---
needs: [english-v3-coordinated-subject-agreement]
---
# Read a floated *each* quantifying the Subject: Two target players each draw a card.

## Why

No clause with a floated *each* reads. On census `xnkuznpo` (docs-only
descendant of `vvxtllnq`; 32,828 supported faces, 23,459 covered, 9,369
unread; wave-4 recon of 2026-10-08), the recon bucket "floated quantifier
*each* (*Ns each get* / *are each*)" touches **262** unread faces and is the
sole cause on **71**. With the bucket "coordinated property subject plural
concord", it touches **278** and is the sole cause on **197**. That union
includes 162 faces with *power and toughness are each …* (Lord of
Extinction, Gaea's Liege, Angry Mob). Those faces also need plural agreement
for the coordinated Nominal Subject, so this ticket `needs`
`english-v3-coordinated-subject-agreement`. The union also includes 10
verbless *with power and toughness each equal to …* faces whose controls
without *each* read 0 (out of scope). Counts are unread faces *touched* /
*sole* as defined in the recon; surface counts, not gain forecasts.

Probes (admitted roots, `--category Sentence --readings 1`), each against
the same string without *each*: "Two target players each draw a card." 0 / 1;
"Up to two target creatures each get +2/+2 and gain trample until end of
turn." 0 / 1; "Those players each discard two cards at random." 0 / 1; "They
each get +2/+2 until end of turn." 0 / 1; "Target creature you control and
target creature an opponent controls each gain indestructible until end of
turn." 0 / 1. "Lord of Extinction's power and toughness are each equal to the
number of cards in all graveyards." 0. Its *are* version without *each* also
reads 0, which is the agreement defect.

## Goal

*each* reads as a quantificational Adjunct quantifying a plural Subject.
It stands in pre-verbal position before a lexical verb (*players each
draw*, *creatures each get … and gain …*, scoping over a coordinated
Predicate). After a finite auxiliary, including copular *be*, it stands in
post-auxiliary position (*are each equal to …*, *are each 2*). Realisation
is byte-exact. A singular Subject does not license it. *each* never
follows a lexical verb. The existing `vocab:FloatedQuantifier/Each`
(`vocabulary.rs:130`, `core.ron:173`) is the lexeme; its floated function
must be a declared feature the grammar reads.

## Analysis

CGEL, Ch. 5 §9.2, p. 413, [13]: fused-head partitives function as
quantificational adjuncts in clause structure and quantify the subject
(*Her parents both felt she had been exploited*). Universal *all*, *both*
and *each* occur there as implicit partitives. CGEL, Ch. 3 §2.1.5, p. 102,
[36]: such determinatives, semantically associated with the subject, appear
in pre-verbal position (*The players all took a card*), may follow an
auxiliary (preferred: *The players had all taken a card*), and cannot follow
a lexical verb (\**The players took all a card*). CGEL, Ch. 2 §3, p. 51:
*be* is an auxiliary verb in its copular use as well, so *are each equal*
is the post-auxiliary position.

Existing route: `FloatedQuantifier` lexemes (`all`, `both`, `each`) are read
today only as Determiners and partitive heads (`each of`, landed). No
clause-level route admits them between Subject and Predicate. This ticket
adds that position for *each*. Give *all* or *both* the same licence only
if the corpus attests them there on a face this ticket covers (pruning
rule).

## Witnesses

Each witness's failing unit is quoted (`xnkuznpo`, 0 Readings each). The
first needs `english-v3-coordinated-subject-agreement`. The other four match
only the floated-*each* bucket.

- Lord of Extinction: "Lord of Extinction's power and toughness are each
  equal to the number of cards in all graveyards." (control "Lord of
  Extinction's power is equal to the number of cards in all graveyards." 1)
- Fated Clash: "Target creature you control and target creature an opponent
  controls each gain indestructible until end of turn." (control without
  *each* 1)
- Press the Advantage: "Up to two target creatures each get +2/+2 and gain
  trample until end of turn." (control without *each* 1)
- Skull Rend: "Those players each discard two cards at random." (control
  without *each* 1)
- Join Forces: "They each get +2/+2 until end of turn." (control without
  *each* 1)

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for every
   crate whose code or consumed data changed (`deckmaste_english_v3` always;
   `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed).
   No xtask tests, no Lean checks, no post-refresh re-census unless refresh
   touched grammar, lexicon or core. New tests go under tests/suite/
   registered in tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A floated *each* read as the Determiner or fused head of a
   following NP (*each get* as an NP), a floated *each* on a singular
   Subject, or one *each* string read both as a Subject-internal and as a
   clause Adjunct, is a defect. A wrong analysis that parses is a defect, not
   a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- Plural agreement for coordinated Nominals:
  `english-v3-coordinated-subject-agreement`.
- Verbless *with power and toughness each equal to …* and *have base power and
  toughness each equal to …* (Karn's Touch, Porcelain Gallery): their controls
  without *each* also read 0.
- Distributive postposed *each* after an Object or measure (*two dollars
  each*), if attested: note it.

## Landing record

The six items in CLAUDE.md. Also give the number of newly covered faces whose
Subject is a coordinated Nominal (*power and toughness*).
