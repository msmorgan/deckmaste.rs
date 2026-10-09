---
needs: []
---
# Read arithmetic with a nominal operand: X is 2 plus the number of cards in your hand.

## Why

*plus* / *minus* compose numerals only. On census `qsupksky` (32,828
supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08),
the recon bucket "arithmetic over a nominal operand" touches **91** unread
faces and is the sole cause on **72** (touched / sole as in the wave-5 recon;
surface counts, not gain forecasts). Attestation
(`cargo xtask english-v3 grep -i`): *N plus the …* 48 faces, *… minus N* 15
faces; the commonest shapes are *N plus the number of …*, *N minus the number
of …*, *the number of … plus the number of …*, *N plus the sacrificed
creature's mana value*, *that much damage plus 1*.

Probes (`--readings 1`): "Its power is equal to 2 plus 3." 1; "X is 2 minus
the number of cards in your hand." 0; "X is the number of creatures you
control plus the number of lands you control." 0; "This creature deals X plus
1 damage to any target." 1.

## Goal

An arithmetic expression whose operand is a nominal (*the number of …*, *the
sacrificed creature's mana value*, *that much damage*) reads wherever the
numeral-only arithmetic reads today, on either side of *plus* / *minus*.

## Analysis

CGEL Ch. 7 §5.1(a), p. 635, [1i] lists *plus* and *minus* among prepositions
taking an obligatory Complement whose prototype is an Object NP. That backs
*plus* / *minus* + NP as a PP only.

Existing route: `ArithmeticMeasure`
(`crates/deckmaste_english_v3/src/declarations.rs:4246`,
`[left: MeasurePhrase, operator: lexical(Preposition), right: MeasurePhrase]`,
both operands `MeasureKind = Scalar`); `vocab:Preposition/Plus` and
`vocab:Preposition/Minus` declare `MeasureOperator: Yes` and
`PrepositionFunctionLicence: None` (`lexicon/core.ron:168–169`). Nominal
scalar NPs already carry the declared `ScalarDenotation` feature (consumed at
`declarations.rs:2335`; exported by `MeasuredAttribute`, `:4342`).

Orchestrator rulings 2026-10-09:

- R10a. One route. Either widen `ArithmeticMeasure`'s operands to admit a
  nominal operand that declares `ScalarDenotation = Yes`, or recast the
  numeral arithmetic as a PP-postmodified measure (CGEL p. 635) and retire
  `ArithmeticMeasure`; implementer's choice, but the numeral-only and
  nominal-operand cases must go through the same Construction, and the
  superseded route is retired on both sides (Method 9).
- R10b. The operand licence reads the declared `ScalarDenotation` feature,
  never a noun (*number*, *value*) or a Construction name.
- R10c. Chained arithmetic (*N plus X plus Y*) keeps whatever bracketings the
  grammar admits as Readings; it is not pruned to one.
- R10d. *plus* stays `PrepositionFunctionLicence: None` outside this
  operator role: *plus an additional {1}*, *mana cost plus exile three other
  cards* and *that many plus one* (Determinative operand) are not arithmetic
  over a nominal and must not start reading through this route.

- R10e (orchestrator ruling 2026-10-09, resolving the scalar-variable STOP).
  A scalar variable Subject may take a scalar predicative Complement generally:
  `X is 2` is grammatical. Read Scalar Denotation from the variable's lexical
  declaration; no arithmetic-only guard. Re-spell the two `X is 2` exclusions
  as positive exact-count scalar-copular assertions, preserving every other
  negative. Widen the bounded manifest with every supported face matching
  `\bX is\b`; re-run claim parent and candidate. Every face outside the
  case-insensitive arithmetic surface must have zero count and digest change;
  existing where-variable definitions must not overlap scalar copulas. Name
  separately gains whose gained line has no *plus* or *minus*.

- R10f (orchestrator ruling 2026-10-09). Retain the 18 R10e-only gains.
  R10e condition 3 concerns previously covered faces: their counts and digests
  must stay unchanged outside arithmetic, preventing duplicate where-variable
  Readings. New R10e coverage is permitted by condition 4. Banefire's
  `X is 5 or more` is the correct scalar-copular analysis. Spot-check five
  R10e-only gains as scalar-copular clauses with Scalar Value Complements.

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0, control 1. (D) =
`--category Document`.

- Kindle: "Kindle deals X damage to any target, where X is 2 plus the number
  of cards named Kindle in all graveyards." (control without "2 plus")
- Ivory Tower: "At the beginning of your upkeep, you gain X life, where X is
  the number of cards in your hand minus 4." (control without "minus 4")
- Allosaurus Rider: "Allosaurus Rider's power and toughness are each equal to
  1 plus the number of lands you control." (control without "1 plus")
- Haunting Apparition: "Haunting Apparition's power is equal to 1 plus the
  number of green creature cards in the chosen player's graveyard." (control
  without "1 plus")
- Food Chain (D): "Exile a creature you control: Add X mana of any one color,
  where X is 1 plus the exiled creature's mana value." (control without "1
  plus")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text matches case-insensitive `\b(plus|minus)\b` or
   case-sensitive `\bX is\b` (R10e), plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, `--workers 6` and `--samples-per-face 0`, run
   under `/usr/bin/time -v`; compare it with `cargo xtask english-v3 diff`
   against the claim parent's census of the same manifest. Never run `--all`.
   Tests: `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
   every crate whose code or consumed data changed (`deckmaste_english_v3`
   always; `deckmaste_lexical_source` if `core.ron`, `verbs.ron` or
   `vocabulary.rs` changed). No xtask tests, no Lean checks, no post-refresh
   re-census unless refresh touched grammar, lexicon or core. New tests go
   under `tests/suite/` registered in `tests/main.rs`. Evidence lives under
   the workspace's ignored `target/english-v3/`, never `/tmp`.
2. Probe first: probe each witness and control before implementation and
   record the counts; write each witness as a test. Run the tests and clippy
   once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A nominal operand admitted without `ScalarDenotation`, or *plus an additional …* / *that many plus one* reading as arithmetic, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit lexicon declarations in `crates/deckmaste_lexical_source/lexicon/`
   and `grammar:` frame fields in `plugins_v2` keyword actions; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. Refine an existing Construction before adding one: the Construction count
   is above the ADR's 250 ceiling. A new frame or Construction must not overlap
   an existing one on the same string (two labels for one constituency is a
   spurious duplicate, not an ambiguity); when it supersedes one, retire the
   old route and re-spell its tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- *that many plus one* (Determinative operand), *plus an additional …*,
  *twice the number of …*.
- Faces whose arithmetic sits inside *with mana value equal to 1 plus …*
  (Birthing Pod, Neoform and others) also need
  `english-v3-property-equal-to`; record how many of them read after this
  landing without working on the equality.

## Landing record

1. **Census delta** (candidate `lrolnmwo`, claim parent `lvnyoktx`). The
   widened identity manifest contains 1,614 faces: 235 supported faces matching
   case-insensitive `\b(plus|minus)\b`, 1,157 matching `\bX is\b` (67
   overlap), plus 300 covered random faces selected by the built-in diff with
   seed 20261008 (11 overlap the combined surface). A fresh claim-parent build
   was measured on its empty child `ynwyqszn`: 1,056 covered, 53,660 Readings;
   candidate `lrolnmwo`: 1,142 covered, 58,786 Readings. Official
   `widened-diff.json`: 86 gained, zero lost, zero decreases, zero increases,
   seven digest-only, 1,521 unchanged; no source changes or unmatched faces.
   Lost faces: none. Numeric arithmetic's operand wrappers account for the
   seven digest-only faces (counts preserved): An-Havva Inn, Crater's Claws,
   Hellfire, Invasion of Tarkir, Let the Galaxy Burn, Sidisi, Regent of the
   Mire, Vitalizing Cascade. All 535 previously measured arithmetic/control
   faces remain count- and digest-identical to the earlier candidate, checked
   by the built-in diff (`arithmetic-candidate-equivalence-diff.json`).
   Evidence is under `target/english-v3/english-v3-arithmetic-nominal-operand/`.

   R10e-only gained faces (their gained conditional lines contain no *plus*
   or *minus*), measured on `lrolnmwo`: **18**. Their face Reading counts
   are Banefire 3; Black Sun's Twilight 8; Blue Sun's Twilight 8; Finale of
   Devastation 407; Finale of Eternity 4; Finale of Glory 56; Finale of
   Revelation 34; Kinetic Ooze 53; Martial Coup 1; Red Sun's Twilight 4;
   Roku's Mastery 1; Shantotto, Tactician Magician 12; Silver Scrutiny 10;
   Subterranean Tremors 3; Vision Quest 2,484; West Coast Expansion 18;
   White Sun's Twilight 7; Zero Point Ballad 30. All 18 are outside the
   arithmetic surface. Of the 1,379 selected faces outside that surface,
   1,361 have zero count and digest change. Previously covered outside-surface
   faces all preserve both counts and digests, satisfying R10e condition 3
   as clarified by R10f; the 18 newly covered faces are retained. There are no
   outside-surface losses, decreases, increases or digest-only changes.

2. **Witnesses** (`lrolnmwo`). Each quoted witness failed before implementation
   on the claim parent (zero Readings); its control yielded a Reading.
   Kindle's quoted damage sentence: 3; Ivory Tower's quoted upkeep sentence:
   2; Allosaurus Rider's quoted CDA: 2; Haunting Apparition's quoted CDA: 6;
   Food Chain's quoted activated ability: 1. These are witness-unit counts,
   distinct from per-face sums across lines. The widened run preserves all
   five earlier candidate counts and selected constituent analyses.

3. **Spot checks** (`lrolnmwo`; one selected Reading per face, with the
   unchanged original candidate confirmed by the official diff): Kindle — X
   is 2 plus the count of cards named Kindle in all graveyards; Ivory Tower —
   X is the hand-card count minus 4; Allosaurus Rider — each statistic equals
   1 plus the controlled-land count; Haunting Apparition — power equals 1
   plus the chosen player's green creature-card graveyard count; Food Chain —
   X is 1 plus the exiled creature's mana value; Black Vise — X is the chosen
   player's hand-card count minus 4; Flame Burst — X is 2 plus the count of
   cards named Flame Burst in all graveyards; War Report — life equals the
   battlefield-creature count plus battlefield-artifact count; Dauntless
   Dourbark — each statistic equals the controlled-Forest count plus
   controlled-Treefolk count; Soulless One — each statistic equals the
   battlefield-Zombie count plus the Zombie-card graveyard count. Each
   nominal remains a Noun Phrase operand under the shared ArithmeticMeasure.

   Five R10e-only gains checked on `lrolnmwo` under R10f: Banefire —
   `X is 5 or more`; Finale of Glory — `X is 10 or more`; Roku's Mastery —
   `X is 4 or more`; Silver Scrutiny — `X is 3 or less`; Vision Quest —
   `X is 4 or greater`. Each selected analysis is a scalar-copular finite
   clause: scalar variable X as Subject, ScalarLocationPredicate headed by
   *is*, and the coordinated Scalar Value (`5 or more`, `10 or more`,
   `4 or more`, `3 or less`, `4 or greater`) as Complement. Each clause has
   exactly one Reading. Full-face probes select an issue-free Reading;
   the integration test asserts all three constituents through the existing
   helper, with no Debug-tree substring assertions.

4. **Deviations and STOPs**. Two Arithmetic Operand projections are added:
   scalar Measure Phrase and scalar Noun Phrase. The macro rejects differently
   typed fields in forms of one Construction and duplicate Result categories
   in schema instances; the projections permit one ArithmeticMeasure route
   without a construction guard or compiler change. Its superseded direct
   numeral operands are retired; the existing operator lexical declarations
   remain consumed by the replacement and free *plus* remains unlicensed.
   Quantified noun phrases propagate scalar denotation from declared measure
   position so *that much damage* is admitted. Arithmetic Operand is added
   to the glossary. No verb frames are added. The ten equality-governor
   faces found by corpus grep remain unread on `lrolnmwo`; Torbran's arithmetic
   constituent reads, but its full replacement clause still needs its existing
   amount position extended by another ticket.

   The earlier STOP over the two `X is 2` exclusions is resolved by
   **orchestrator ruling R10e (2026-10-09)**: scalar variables may generally
   take scalar predicative Complements, and the Subject licence reads Scalar
   Denotation on their lexical declarations. X and Y now declare it; the
   Subject exports it. Both exclusions are re-spelled as positive exact-count
   selected scalar-copular assertions; every other negative remains. The
   glossary includes scalar variables under that ruling, not under CGEL.
   Before/after probes preserve `where X is its power` at 1 Reading and
   `where X is the number of cards in your hand` at 3 on `lrolnmwo`.
   A lexical-only diagnostic with the claim-parent binary and new data changes
   no count or digest on any of the 1,614 selected faces.

   The second STOP (2026-10-09), over the 18 non-arithmetic gains, is resolved
   by **orchestrator ruling R10f (2026-10-09)**: retain the gains; R10e
   condition 3 applies to previously covered faces, and newly covered R10e
   faces are permitted by condition 4. All previously covered faces outside
   arithmetic preserve count and digest. The `X is 5 or greater` probe gains
   its correct unique scalar-copular Reading (0 → 1). No pruning, construction
   guard or test weakening is needed. Five of the 18 gains are checked in
   item 3 and the full-suite test.

5. **Tests** (`lrolnmwo`, after R10e/R10f). Full suites and clippy passed:
   `cargo test -p deckmaste_english_v3` — 668 passed, zero failed/ignored;
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` — passed;
   `cargo test -p deckmaste_lexical_source` — 63 passed (28 unit + 35 integration),
   zero failed/ignored;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` — passed.
   Tests restored 0, re-spelled 3, ignored 0, added 11, removed 0.
   Re-spelled: `grammar::independent_measure_values_preserve_operator_structure_and_notation`,
   `copular_scalar_location::scalar_values_and_other_copular_routes_do_not_overlap`,
   `where_variable_clause::supplementary_licence_excludes_integrated_and_locative_routes`.
   No xtask tests or Lean checks. Logs: `tests-r10f.log`, `clippy-r10f.log`,
   `lexical-tests-r10f.log`, `lexical-clippy-r10f.log` in the evidence directory.

6. **Timing** (`lrolnmwo`). The widened bounded candidate census took
   **15.53 s wall**, 6 workers, zero samples per face, peak RSS
   **3,118,080 KiB**, measured by `/usr/bin/time -v` with a prebuilt
   `target/debug/cargo-xtask`; compilation is excluded. The matching
   claim-parent run (`ynwyqszn`, empty child of `lvnyoktx`) took 15.58 s,
   peak RSS 3,062,196 KiB. Neither run used `--all`.
