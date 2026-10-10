---
needs: []
---
# Read a verbless *while* Adjunct: Whenever this creature attacks while saddled, create a Treasure token.

## Why

*while* + clause reads ("Whenever this creature attacks while it's saddled,
it gets +2/+2 until end of turn." 2); *while* + a bare state Adjective does
not. Recorded as residue in `english-v3-keyword-state-adjectives` and
`english-v3-systemic-residuals` ("Verbless *while saddled* remains outside
this repair … its *while tapped* control also fails"). Blocker report at trunk
`ymqwzour` (`target/english-v3/blockers-wave8.json`): the failures hide under
`at:, create`, `at:, it`, `at:, put`. `cargo xtask english-v3 grep '\bwhile
(saddled|tapped|untapped|attacking|blocking|enchanted|equipped|suspected|goaded|face down)'`
finds **26** faces, all unread; 26 failing lines match, all **26** the only
failing line of their face. All 26 are *attacks while saddled*.

## Goal

A *while* PP whose Complement is a subjectless Verbless Clause with a state
Adjective Predicate reads as an Adjunct of the verb, the understood Subject
being the matrix Subject.

## Analysis

CGEL Ch. 14 §10(b), p. 1267: a number of prepositions "allow the complement
to be reduced – either to a non-finite clause … or else a verbless one", with
[2iv] "*He can be very dangerous when drunk*" and [2v] "*While in Paris, I
visited Uncle Leonard*"; "*when* and *whenever*, *while* and *whilst* …
enter into the present construction". That backs *while* + verbless
Complement with an adjectival Predicate. Which Adjectives may be that
Predicate in Oracle English is a project ruling. Glossary: **Verbless
Clause**, **Verbless Predicate Use**, **Verbless Clause Complement**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`VerblessClause` (`form [predicate: lexical(Adjective)]; require
predicate.VerblessPredicateUse = Yes; export VerblessClauseComplement = Yes`)
and `VerblessClauseComplementPreposition` (`require
head.VerblessClauseComplement = Yes; require
complement.VerblessClauseComplement = Yes; require
licence_adjunct(head.PrepositionFunctionLicence) = Yes`), landed for *if able*
(`english-v3-verbless-if-complement`). In `lexicon/core.ron`,
`vocab:Preposition/While` declares no features; only `vocab:Preposition/If`
declares `VerblessClauseComplement: Yes` and only `vocab:Adjective/Able`
`VerblessPredicateUse: Yes`. *saddled* is the state Adjective
exported by `plugins_v2/builtin/macros/keyword_abilities/saddle.ron` ("This
creature is saddled." 2: adjectival and verbal passive, ruling KS4).

Orchestrator rulings 2026-10-10:

- VW1. Data first: declare `VerblessClauseComplement: Yes` on *while*, and
  `VerblessPredicateUse: Yes` on the state Adjective. No new Construction if
  the two existing ones then read the witnesses.
- VW2. Which Adjectives: the licence is declared per class, not per word. The
  keyword state Adjectives derive from class membership
  (`english-v3-keyword-state-adjectives`, KS1–KS6); give that derivation the
  `VerblessPredicateUse` export, so *saddled* gets it as a member, and *tapped*
  / *untapped* by their own declaration only if a witness needs them (none
  does: do not add them speculatively). If the feature cannot be exported by
  the class derivation without naming *saddled*, STOP.
- VW3. The glossary entry **Verbless Predicate Use** is worded for *able*
  ("with its selected infinitival Complement unexpressed"). Amend the entry
  (domain-modeling) so the permission is the Adjective's, with *able* as one
  case; do not bend the feature to the old wording.
- VW4. One analysis per string. *attacks while saddled* has the *while* PP as
  Adjunct of *attacks*. The Predicate is the Adjective: no verbal-passive
  twin (a subjectless past-participial clause after *while*) is added, though
  "is saddled" keeps its two Readings. The PP must not also attach to the main
  clause across the comma.
- VW5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face has *while* + Adjective. An increase on a
  covered *while* + clause face, or on *if able*, is a defect.

- VW6 (orchestrator, 2026-10-10). Extend the generic Adjective class mechanism
  to export declared features onto existing members, preserving every owner
  and introducing no duplicate Lexeme. The class row declares
  `VerblessPredicateUse: Yes`; membership remains declarative and no loader
  condition names a word. A conflicting feature already declared by a member
  is a load error, never a silent override. Disclose the loader capability as
  a deviation and test it with a synthetic class. This resolves VW2's STOP:
  the claim parent exports *saddled* as `lexeme:designation/saddled`, not from
  the participial class, whose recipe would create a second owner.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Gilded Ghoda: "Whenever this creature attacks while saddled, create a
  Treasure token." (0)
- Alacrian Jaguar: "Whenever this creature attacks while saddled, it gets
  +2/+2 until end of turn." (0)
- District Mascot: "Whenever this creature attacks while saddled, put a +1/+1
  counter on it." (not probed after the first two; same opening)
- Dracosaur Auxiliary: "Whenever this creature attacks while saddled, it
  deals 2 damage to any target." (as above)

Also 0 today: "This creature attacks while saddled."; "It has flying while
tapped." (unattested; localisation only). Controls that must not change:
"Whenever this creature attacks while it's saddled, it gets +2/+2 until end
of turn." 2; "Whenever this creature attacks, draw a card." 1; "This creature
is saddled." 2; "You may look at it while it's tapped." 3; and every covered
*if able* face.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bwhile\b` or `\bif able\b` case-insensitively (both licences'
consumers)
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261008`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
VW5 for increases. Full `cargo test -p deckmaste_lexical_source` and `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. Lexicon and plugin `grammar:` data load at runtime, so a data-only change
needs no rebuild; a grammar edit costs 2–3 minutes and the six test binaries
about 75 seconds.
Tests assert the role (the *while* PP as Adjunct of the verb, the Adjective as Predicate of
its Verbless Clause Complement, the VW4 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only. If
a plugin definition changes, run `cargo xtask lean check`.

## Out of scope

- *while* + PP or participle (*while attacking*, *while on the
  battlefield*): unattested as sole blockers.
- *Calamity … attacks while saddled, choose a nonlegendary creature that
  saddled it this turn and create a tapped and attacking token …*: second
  cause (`english-v3-coordinated-participial-premodifier`).
- Semantics of *saddled*.

## Landing record

1. **Census delta** (claim-parent licences and implementation at `wxwozrtz`,
   claim `ktlqxpoo`): bounded 643-face manifest, every supported face matching
   `\bwhile\b|\bif able\b` case-insensitively (347 faces), plus 300 covered
   controls drawn by the official diff from `baseline-wave8.json`, seed
   20261008 (four overlaps). Covered 566 → 591; Readings 10,640 → 10,814.
   Official `diff --lines`: 25 gains, 0 lost, 0 decreases, 0 increases,
   0 digest-only; 0 source changes and 0 issues. Lost faces: none. No Reading
   increase on a previously covered face. All 25 gains use the intended
   construction; wrong-analysis-only gains: 0. Evidence under
   `target/english-v3/english-v3-verbless-while-adjunct/`: `manifest.json`,
   `before.json`, `after.json`, `diff.json`. Calamity remains unread for the
   ticket's named second blocker.
2. **Witnesses** (`wxwozrtz`, complete sentence probes, limit 64): Gilded
   Ghoda, Alacrian Jaguar, District Mascot and Dracosaur Auxiliary each 0 → 1;
   each control with *while saddled* removed reads 1 before and after (W9-A).
   *This creature attacks while saddled* 0 → 1. The unattested *while tapped*
   localization remains 0, as VW2 requires. Finite *while it's saddled*
   control stays 2; the ordinary attack-trigger control stays 1; *This creature
   is saddled* stays 2; *You may look at it while it's tapped* stays 4
   (the draft's 3 was enumeration-capped). Every covered *if able* face keeps
   its count and digest. Probe evidence: `before-probe-*.json` and
   `after-probe-*.json` in the evidence directory.
3. **Spot checks** (`wxwozrtz`): Alacrian Jaguar, Autarch Mammoth, Bounding
   Felidar, Bridled Bighorn, Brightfield Glider, Brightfield Mustang, Bulwark
   Ox, Congregation Gryff, District Mascot and Dracosaur Auxiliary. Each
   inspected Reading places the *while* PP as Adjunct of the attack Predicate;
   its Complement is a Verbless Clause with the existing
   `lexeme:designation/saddled` Adjective as Predicate. Complete Document
   probe counts, witness/control without the new material, respectively:
   1/1, 1/1, 2/2, 1/1, 2/2, 3/3, 5/5, 2/2, 2/2, 1/1. W9-A's independent
   ambiguity is preserved; every spot probe has 0 issues. Evidence:
   `spot-0.json` through `spot-9.json` and matching `spot-control-*.json`.
4. **Deviations and STOPs**: VW2's required STOP established that the claim
   parent owns *saddled* as a designation Adjective, not a participial-class
   derivation. VW6 resolves it: extended the general Adjective class loader
   with declared feature exports and optional negative formation, retaining
   each existing owner and rejecting conflicting member features without an
   override. Synthetic-class loader tests cover export, repeated matching
   declarations, owner preservation and conflict rejection. The existing
   negative class retains its meaning with `Some(Joined)`. Added 0 / deleted
   0 Constructions or Frames; no superseded route to retire. Broadened the
   existing **Verbless Predicate Use** glossary entry under VW3, retaining
   *able* as one case and citing the state permission as a project ruling.
   No unresolved STOP remains.
5. **Tests** (`wxwozrtz`): `cargo test -p deckmaste_english_v3` passed all six
   binaries (899 tests); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed. `cargo test -p deckmaste_lexical_source` passed (42 unit + 44
   integration); `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`
   passed. Initial English clippy found a needless Box replacement in the new
   fixture; assigned into the existing Box, preserving the exact asserted
   values, and reran the full English suite and clippy successfully. Restored
   0 / re-spelled 0 / ignored 0 / added 5 (2 loader, 3 grammar) / removed 0.
   The new module is registered exactly once in `suite_4`. `cargo build -p xtask`
   passed. Citation checks: 0 noncompliant, 0 stale; piped diff audit selected
   0 sites, so no new rules needed blessing. No Lean or full-corpus check ran.
   Logs: `tests-english.log`, `clippy-english.log`, `tests-lexical-source.log`,
   `clippy-lexical-source.log`, `cite-*.log` in the evidence directory.
6. **Timing**: complete bounded census wall time **25.06 seconds**, six workers,
   at `wxwozrtz`, measured by `/usr/bin/time -v` (`after.time`).
