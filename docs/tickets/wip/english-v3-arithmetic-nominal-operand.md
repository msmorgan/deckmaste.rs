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
   whose analysed text matches case-insensitive `\b(plus|minus)\b`, plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261009`),
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

Six items, each figure stamped with the change id it was measured on (ruling
2026-10-08):

1. **Census delta**: covered faces and total Readings before/after on the
   bounded manifest; lost faces by name with classification, or "none".
2. **Witnesses**: each witness above with its Reading count after.
3. **Spot checks**: ten newly covered faces by name with the selected analysis.
4. **Deviations and STOPs**: constructions, frames or tests added or deleted
   beyond the ticket's letter, with justification; every STOP and its
   resolution; any glossary gap in `docs/contexts/oracle-english/CONTEXT.md`.
5. **Tests**: the `cargo test -p` / clippy commands and outcomes; counts
   restored / re-spelled / ignored-with-blocker / added / removed.
6. **Timing**: wall time of the bounded census as one figure with worker count
   (6), plus peak RSS from `/usr/bin/time -v`.
