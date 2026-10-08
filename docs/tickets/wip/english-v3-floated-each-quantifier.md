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

1. **Census delta**: Claim-parent evidence (`mksnrsvvvyoy`) reconstructed from the inherited full report plus landed `lqzlkrrplkuk` and `szlklvnskssz` subset reports, in that order. The first inherited comparison omitted the former subset; correcting it retired three inherited digest changes and sixteen incorrectly attributed gains. The corrected bounded selector is 5,515 supported raw-text faces containing literal ` each ` plus 300 covered controls selected by the existing diff command, seed 20261008; 30 overlap, 5,785 distinct faces (`prxpttomxtxv`). Before refresh: covered 3,608 → 3,821; Readings 376,419 → 470,186 (`mksnrsvvvyoy` → `prxpttomxtxv`). Feature gains 213. After refresh: covered 3,608 → 3,860; Readings 376,419 → 510,350 (`mksnrsvvvyoy` original claim evidence → `prxpttomxtxv` refreshed). Total gains 252, including 39 inherited from `noppnrqzoquy` (clause-final predicative-as; `diff-refresh.json` reports 39 gains and zero other changes); lost 0 (none), decreases 0, increases 0, digest-only 0; 112 newly covered faces have coordinated *power and toughness* Nominal Subjects (`prxpttomxtxv`). All 5,785 enumerations complete, 0 issues/internal failures/duplicate Readings (`prxpttomxtxv`). No full corpus run. Evidence: `target/english-v3/english-v3-floated-each-quantifier/{before,manifest,after,diff}.json`.
2. **Witnesses**: All five units probed before implementation: 0 Readings (`mksnrsvvvyoy`). After (`prxpttomxtxv`): Lord of Extinction 4; Fated Clash 2; Press the Advantage 6; Skull Rend 2; Join Forces 2. Modal Turtle Tracks unit 4 and scalar Angry Mob unit 1 (`prxpttomxtxv`). These are quoted-unit counts, not whole-face counts; self-names probed with `--as-card`.
3. **Spot checks**: Ten newly covered whole faces (`prxpttomxtxv`), one selected byte-exact Reading inspected each: Lord of Extinction (4), Ancient Ooze (7) — coordinated *power and toughness* Subject, post-auxiliary *each*, selected equality Complement; Fated Clash (42) — coordinated target-creature Subject, pre-verbal Adjunct; Press the Advantage (6), Abandon Reason (6) — plural target-creature Subject, Adjunct scopes over coordinated Predicate; Skull Rend (2) — *Those players* Subject, Adjunct before *discard*; Join Forces (2) — plural *They* Subject, Adjunct before *get*; Band Together (6) — plural target-creature Subject, Adjunct before *deal*; Alms Collector (1) — coordinated *you and that player* Subject, Adjunct before *draw*; Ecological Appreciation (362) — *that each have* Subject-relative Clause, plural agreement with antecedent. Saved probes: `spot-0.json` through `spot-9.json`; all ten rechecked after refresh in `spot-refresh-0.json` through `spot-refresh-9.json`, counts and analyses retained (`prxpttomxtxv`).
4. **Deviations and STOPs**: User ruling 2026-10-08 replaces Method's full census with bounded verification and test-first witness builds with pre-implementation probes and tests at the end. Added eight constructions: `QuantifiedClause`, `QuantifiedAuxiliaryHead`, `PostAuxiliaryQuantification`, `QuantifiedScalarLocation`, `QuantifiedBareAuxiliaryPredicate`, `QuantifiedParticipialAuxiliaryPredicate`, `QuantifiedPerfectAuxiliaryPredicate`, `QuantifiedSubjectRelative`. The auxiliary head preserves selected frame ownership; it is a declaration adapter, not a CGEL assertion of constituency. Separate auxiliary-complement routes retain existing frame licences. Relative extension covers the same Subject-quantifying function attested on Ecological Appreciation, Agadeem's Awakening, Slimefoot's Survey, Catapult Fodder, Uncage the Menagerie and Tiamat. CGEL backs Subject function and positions; plural restriction follows the ticket. Refresh conflict at the adjacent glossary insertion resolved by retaining both sets of entries; no code conflict. No new frames, lexemes, word-naming guards, retired routes or unresolved STOPs. Existing determiner/partitive routes retained; no route superseded; *all/both* licence unchanged. Added glossary entries Quantificational Adjunct, Quantificational Adjunct Use and Auxiliary Use; no remaining glossary gap. No distributive postmeasure *each* found; final *one of each* is the existing partitive.
5. **Tests** (`prxpttomxtxv`): `cargo test -p deckmaste_english_v3`: 544 passed, one new relative test failed; fixed its code to use the existing relative Subordinator category. `cargo test -p deckmaste_english_v3 --test main floated_each_quantifier::`: all 7 passed after the fix (538 filtered). `cargo test -p deckmaste_lexical_source`: 59 passed. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`: passed. After the incoming `noppnrqzoquy` grammar/lexicon change, reran only these affected crates: English suite 552 passed, lexical-source suite 59 passed, both clippy commands passed (`prxpttomxtxv`). Formatting check passed. Added 7, restored 1 (the new relative test, by code fix), re-spelled 0, ignored 0, removed 0. Corpus admission, ownership, byte-exact roundtrip and node/word traversal validated for every counted Reading; independently constructed auxiliary-head roundtrip covered by the new test. No xtask tests or Lean checks.
6. **Timing**: Final refreshed bounded census: 73.016 seconds including setup, 12 workers (`prxpttomxtxv`). Corrected pre-refresh bounded census took 89.679 seconds with 12 workers (`prxpttomxtxv`). Initial bounded run used the incomplete inherited baseline's random sample and was superseded; it took 116.880 seconds including setup (`prxpttomxtxv`).
