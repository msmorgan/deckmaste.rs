---
needs: []
---
**Retire the checker's timing and window refusals (10 constructors).**
Per the checker's charter (`docs/decisions/semantics-v2.md` §7, ruling
2026-10-06; the `lean/CONTRACTS.md` header). The checker admits a written term as a faithful reading of the
card's English: references resolve and terms are well-kinded. It is not a
rules engine, and an instruction the game cannot carry out is still a
faithful reading [CR#101.3,609.3]. Standard constraints apply.

## The bucket

| Constructor | `Refusal.lean` line | Pin lines | Candidate emission sites (`lean/Semantics/Check/`) |
|---|---|---|---|
| `windowOk` | 138 | 11 (4 files) | AbilityRules.lean:410,411; PhraseRules.lean:515,668; Triggers.lean:25,27 |
| `pointWindowOk` | 139 | 1 (1 file) | Triggers.lean:28 |
| `eventUnderway` | 178 | 2 (2 files) | Triggers.lean:36 |
| `durationOk` | 181 | 1 (1 file) | AbilityRules.lean:286,388 |
| `triggerCountOk` | 190 | 1 (1 file) | AbilityRules.lean:717 |
| `untriggeredLimit` | 203 | 1 (1 file) | AbilityRules.lean:701,775 |
| `interceptable` | 204 | 2 (2 files) | AbilityRules.lean:702; Events.lean:54 |
| `heldClause` | 217 | 0 (0 files) | AbilityRules.lean:394 |
| `notInstead` | 220 | 2 (2 files) | AbilityRules.lean:391 |
| `chapterDefaults` | 246 | 4 (2 files) | AbilityRules.lean:785 |

Pin lines in the bucket: 25.

The emission-site column is a grep for `.<name>` under `lean/Semantics/Check/`
(Refusal.lean excluded), 2026-10-06. Some matches are a same-named predicate
or another type's constructor (`NounPhrase.discardOk` is a predicate, not an
emission); confirm each before removing it. Pin lines count matching lines
under `lean/Semantics/Proofs/`.

## The work

- Remove each constructor's emission sites and the constructor itself, or
  keep a constructor with a one-line reason where this ticket allows it.
- Re-spell or retire each pin that asserted a retired refusal. A pin whose
  subject still exists is re-spelled against what the checker now says (the
  same term, its new outcome); a pin is removed only when the refusal was its
  whole subject, and the landing record names it.
- Update the facts table and `lean/CONTRACTS.md`: each section this sweep
  touches says what the checker no longer checks.

## Proof

- No reference-resolution or term-shape pin changes outcome.
- `cargo xtask lean check` over `plugins_v2/canon` and `plugins_v2/testing`
  stays all-pass, and `cargo xtask lean definitions` stays all-pass (or
  their regrouped names, `cargo xtask lean check` and `cargo xtask lean
  definitions`, once `semantics-v2-chooser-and-taker-fields` lands).
- `lean/scripts/build` passes.

## Did any retired pin encode a real card defect?

Mandatory. The landing fills this in, one line per retired or re-spelled pin:
**yes**, naming the card and the defect the refusal caught, or **no**. A yes
is reported to the owner with the landing record; it may argue for keeping
the refusal, which needs a dated ruling.

Every one of the 24 pins below is a hand-written sentence; none is a printed card, and no
canon or testing card failed a retired refusal before this landing (the gate's baseline was
canon 138/138, testing 8/8). The bucket table's census (2026-10-06) counts 25 pin lines,
`windowOk` at 11 in 4 files; at the claim the same grep finds 24, `windowOk` at 10 in 3 files
(`Trigger`, `Mana`, `Turn`): the fourth file's pin (an `insertPart .turn` refused
`windowOk`) was deleted with `insertPart` by `semantics-v2-chooser-and-taker-fields`
(`ouysnvuw`). No pin encoded
a real card defect: **0 yes**.

| Pin | File | Retired refusal asserted | Fate | Real card defect? |
|---|---|---|---|---|
| `badWhileDoingMoment` | Trigger.lean | `eventUnderway` | deleted | no: a hand-written sentence, a twin of the rule |
| `badHeaderBareTurnWindow` | Trigger.lean | `windowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badChapterLimit` | Trigger.lean | `chapterDefaults` | deleted | no: a hand-written sentence, a twin of the rule |
| `badChapterIntervening` | Trigger.lean | `chapterDefaults` | deleted | no: a hand-written sentence, a twin of the rule |
| `badMultipliedNonTrigger` | Trigger.lean | `triggerCountOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badChoiceRiderNotUnderway` | Choice.lean | `eventUnderway` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTriggeringReplaced` | Counters.lean | `interceptable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badChapterReplacement` | Static.lean | `interceptable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badActionLimitOnActivated` | Keyword.lean | `untriggeredLimit` | deleted | no: a hand-written sentence, a twin of the rule |
| `badChapterWhile` | Keyword.lean | `chapterDefaults` | deleted | no: a hand-written sentence, a twin of the rule |
| `badChapterJoin` | Keyword.lean | `chapterDefaults` | deleted | no: a hand-written sentence, a twin of the rule |
| `badBeforeTheTurn` | Mana.lean | `windowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPluralAttackWindow` | Mana.lean | `pointWindowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCastBeforeTheTurn` | Mana.lean | `windowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTriggerAtYourTurn` | Turn.lean | `windowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badUntilBeginningOfUpkeep` | Turn.lean | `durationOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPluralPartWindow` | Turn.lean | `windowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAdditionalTurn` | Turn.lean | `windowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPluralPartPossessor` | Turn.lean | `windowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badBeginningOfCombatPlural` | Turn.lean | `windowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badEndingPhaseWindow` | Turn.lean | `windowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAdditionalEndingPhaseAfterTurn` | Turn.lean | `windowOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `nestedReplacementCannotHideInScope` | MacroParameters.lean | `notInstead` | deleted | no: a hand-written sentence, a twin of the rule |
| `badNestedInstead` | Zone.lean | `notInstead` | deleted | no: a hand-written sentence, a twin of the rule |

## Borderline, decided here

`notInstead` and `heldClause` were classified as timing against other game
rules; the sweeping agent decides each with a one-line reason. `heldClause`
has no pins.

## Landing record

Measured on the tip `kvumltlu` (the costs docs commit; its `lean/` tree is the pins commit
`pytrupos`'s) after `kata refresh` onto trunk `zkmuyqxw`, 2026-10-08. Both sweeps landed in one
claim (`checker-sweeps-timing-costs`) and were proved together. No coverage lock, grammar
construction or Rust crate is in play: the change touches `lean/` and `docs/` only.

Commits: `mwwwzqww` the emission sites, the dead predicates and the 10 `Refusal`
constructors; `ltvmkonk` the pins; `lmlzpsyq` CONTRACTS and this record.

### PROVE

- **No silent loss.** Every pin that asserted a retired refusal is named in the table above
  with its fate: 24 deleted (the refusal was its whole claim, retired by the §7 charter ruling
  of 2026-10-06, consequence (b)), 0 re-spelled. `cargo xtask lean check`:
  canon 138/138, testing 8/8; no card newly fails and none newly passes, since every card passed
  before. `cargo xtask lean definitions`: 553/553 (73 counters, 461 subtypes, 19 designations).
  `cargo xtask facts check`: up to date.
- **Structural laws.** `lean/scripts/build` (`lake build --wfail`, 88 jobs): clean at
  the tip; `lake build Semantics SemanticsCards --wfail` passed at the constructor commit, where
  the pin suites cannot yet pass, and the full build from the pins commit on. No reference-resolution or term-shape pin changed outcome:
  no pin asserted a timing refusal beside another refusal, and every other pin passes
  unchanged. Every reference check at a retired site stays: the possessor of a window, a
  header or a "during" condition is still checked as a player in context
  (`OptNoun.check`/`HeaderPossessor.checkIn`), and the "while" event, the duration's event, the
  replaced event and the held clause are still checked as before.
- **No word-naming.** No guard names a lexeme, construction or card.
- **Cites.** `cite check --list-noncompliant` empty; `cite check` 0 stale (16,352 citations);
  `cite audit --diff` from the claim read 5 sites (the five this landing adds in the two
  records), each against its claim. Citations that left with deleted docstrings and pins are not
  stale; the lock is unchanged.
- **Board.** `kata kanban check`: OK.
- **Expansions and lexicon export.** No file under `plugins_v2/`, `crates/` or the lexicon
  changed between the claim and the tip, so both are byte-identical by construction; neither
  was re-run.
- **Rust.** No crate's code or consumed data changed (there is no Rust mirror of `Refusal`;
  the facts generator names none of the 10 and emits none of the deleted `EventFacts` or
  `ActFacts` fields), so no `cargo test -p` or clippy run is owed.

### DISCLOSE

- **Constructors removed (10):** `windowOk`, `pointWindowOk`, `eventUnderway`, `durationOk`,
  `triggerCountOk`, `untriggeredLimit`, `interceptable`, `heldClause`, `notInstead`,
  `chapterDefaults`. None kept.
- **Emission sites removed (20 `refuse` calls):** `Timing.check` (`duringPart`, `beforePart`: three),
  `Concurrent.check` (`whileDoing`), `Condition.checkIn` (`duringPart`), `GameEvent.checkIn`
  (`beginningOf`), and in `Check/AbilityRules.lean` `establish` and `delay` (`durationOk`),
  `replace` (`notInstead`, both arms), `holdUntil`, `addTurnPart` (three `windowOk`),
  `StaticSpec.replacement` (`untriggeredLimit`, two `interceptable`), `additionalTriggers`,
  `Ability.activated` (`untriggeredLimit`) and `Ability.triggered` (`chapterDefaults`). The
  ticket's grep column is confirmed: every match was an emission or the predicate it read.
- **Predicates deleted** (each fed only a retired refusal): `partPossessorOk`, `windowOk`,
  `pointWindowOk` (`Check/Phrase.lean`); `HeaderPossessor.ok`, `interceptArmsOk`,
  `Duration.ok`, `durationOk`, `untriggeredLimitOk`, `chapterDefaultsOk`
  (`Check/EventContext.lean`); `interceptOk`, `triggerCountOk`, `durationEventOk`,
  `eventUnderwayOk` and the `EventFacts` fields `interceptable`, `countable`,
  `boundsDuration`, `underway` (`Check/Events.lean`; `hasMagnitude` stays, `tallyOk` reads
  it); `TurnPart.proper`, `actStepwiseOf` and the hand-written `ActFacts.stepwise` field with
  its three rows (fateseal, scry, surveil) (`Check/Words.lean`); `Instruction.heldUntilOk`,
  `Instruction.isInstead` (`Check/Abilities.lean`). `Instruction.unscoped` stays (the deed
  profile reads it).
- **Kind checks kept.** None was owed: no retired site fixed a kind. The window and header
  possessors keep their player-kind check through `OptNoun.check (some .player)`, which was
  never part of the retired refusal.
- **Borderline decisions, one line each.**
  - `notInstead`: retired. That a replacement does not replace a replacement is a rule about
    what replacement effects watch [CR#614.1,614.5]; a nested `replace` is a well-kinded term
    and refers to nothing it cannot resolve.
  - `heldClause`: retired. Which clauses can be undone when the "until" event happens is a
    rule [CR#610.3]; the held clause and its event are still checked for references.
  - `windowOk`/`pointWindowOk`'s plurality half (`partPossessorOk`: a plural possessor
    distributes over players): retired with the window. It says a turn part has one active
    player [CR#102.1], a game rule; it is not a reference (the possessor still resolves through
    `OptNoun.check`), and `durationPossessor`, outside this bucket, is untouched.
- **Deviations and additions.** `GetsBothDeltas.badUntilBeginningOfUpkeepTwin` (the twin of a
  deleted pin) is kept unchanged in its claim, both spellings now `[]`; only its docstring is
  reworded to stop citing the deleted pin. No construction or test was added.
- **STOPs.** None.
- **Glossary.** No new term needed.

### REPORT

- Counts (Assurance): restored 0, re-spelled 0, ignored 0, added 0, removed 24 (each named
  above with the constructor it asserted; all retired by the §7 charter ruling, 2026-10-06,
  consequence (b)).
- Builds: the first build of the claim was warm (`lake build --wfail`, 88 jobs, under a
  second); `lake build Semantics SemanticsCards --wfail` took 154 to 169 seconds at the
  constructor commits, the full build 30 to 37 seconds at the pins commits. `cargo xtask lean
  check` took 99.1 seconds (its own timer; 377 seconds with a cold `cargo-xtask` compile), host
  load average 5.3 on 24 threads.
