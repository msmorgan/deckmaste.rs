---
needs: []
---
# Read the determiner-less temporal NP *last turn*: if no spells were cast last turn, …

## Why

*this turn* reads as a temporal Adjunct; *last turn* does not. On census
`qsupksky` (32,828 supported faces, 25,474 covered, 7,354 unread; wave-5
recon of 2026-10-08), the recon bucket "last turn" touches **72** unread
faces and is the sole cause on **64** (touched / sole as in the wave-5 recon;
surface counts, not gain forecasts); most are the day/night werewolf
transform trigger. `cargo xtask english-v3 grep -i '\blast turn\b'` finds 80
faces: bare *last turn* after *spells* / *cast* / *life* on all but 10, which
have *your* / *their* / *its controller's last turn*. Bare *next turn* occurs
once (Peace Talks, *This turn and next turn, …*).

## Goal

*last turn* reads as a temporal NP Adjunct in the post-verbal position where
*this turn* reads today.

## Analysis

CGEL Ch. 8 §6.3, p. 698: temporal-location nouns such as *week* "require
dependents, such as the demonstratives *this* and *that*. *Next* and *last*
occur widely, but with restrictions. *Last* is always deictic". That backs
*last* as the dependent that licenses a determiner-less temporal NP only.

Claim-parent route (`szkxvvylquyv`; original ticket line numbers are historical):
`NominalAdjunctPhrase`
(`crates/deckmaste_english_v3/src/declarations.rs:3707`,
`[determiner: lexical(Determinative), " ", head: Nominal]`), licensed by
`nominal_adjunct_license` (`:3694`) over the declared
`NominalAdjunctDeterminer` (*this*, *that*, *every*, *each*;
`lexicon/core.ron:118–121`, `:192`) and `NominalAdjunctClass` (*turn*
`Temporal`, `core.ron:135`), attached through the `NominalAdjunctPredicate`
schema (`declarations.rs:1579`). *last* lexes only as an ordinal Numeral
(value −1; probe lexical alternatives) and premodifies through
`OrdinalPremodifier` (`declarations.rs:4164`). "Draw a card during your next
turn." reads 1; "Draw a card next turn." 0.

Orchestrator rulings 2026-10-09:

- R14a. Refine `NominalAdjunctPhrase` (a second form or a widened dependent
  slot) so that a deictic dependent licensed by a declared feature can stand
  in place of the Determiner; no new parallel Construction for temporal NPs.
- R14b. The licence is a declared feature on the dependent (the ordinal
  reading of *last*), never a test of the word *last* or *next*. If ordinal
  Numeral readings cannot carry a declared feature today, STOP with the
  measured options rather than adding a word guard.
- R14c. Pruning rule: bare *next turn* has one attested face; license *next*
  only if that face (Peace Talks) needs it and reads correctly, else leave
  it out.
- R14d. *last turn* must not also read as *last* + a Nominal *turn* in some
  other function (e.g. as an Object NP of *cast*) where the claim parent
  does not already admit such a reading.

## R14e resolution (2026-10-09)

Orchestrator ruling R14e supersedes R14b's representation premise. Rehome
*last* as a Word Lexeme mirroring *next*, retire the generated *last* ordinal
alternative, and read its declared dependent licence in NominalAdjunctPhrase.
Do not add a numeral-property channel. Include every supported *last* face
in the bounded manifest. Zero lost faces; changed covered *last* faces must
have equal or lower counts and correct replacement analyses. STOP if a lost
face needs an ordinal-only position that a declaration cannot restore.
Retire unreachable code and re-spell its tests. *Next* receives no new licence:
Peace Talks already reads on the claim parent (R14c).

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0, control (*last
turn* → *this turn*) 1.

- First Response: "At the beginning of each upkeep, if you lost life last
  turn, create a 1/1 white Soldier creature token."
- Lambholt Elder: "At the beginning of each upkeep, if no spells were cast
  last turn, transform this creature."
- Silverpelt Werewolf: "At the beginning of each upkeep, if a player cast two
  or more spells last turn, transform this creature."
- Call of the Full Moon: "At the beginning of each upkeep, if a player cast
  two or more spells last turn, sacrifice this Aura."
- Feast on the Fallen: "At the beginning of each upkeep, if an opponent lost
  life last turn, put a +1/+1 counter on target creature you control."

## Method

Standard constraints apply (CLAUDE.md assurance and scope;
`docs/tickets/README.md` landing record).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text matches case-insensitive `\b(last|next) turn\b`, plus every supported face matching `\blast\b`, 300 stock-sampled
   `\b(this|that|each|every) turn\b` controls and 300 random covered controls
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
   Reading. *last turn* read as an Object or Subject NP rather than a temporal Adjunct, or a determiner-less *turn* licensed without the declared dependent, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *since your last turn ended* (Premature Burial; *ended* is a vocabulary
  gap) and other *your / their last turn* clauses.
- Preposed temporal NPs (`english-v3-next-time-relative-adjunct`).

## Landing record

1. **Census delta** (`xmyznkll`, grammar of claim parent `lolqwrss` before):
   bounded manifest of 1,007 faces; covered 832 → 895, Readings 52,567 →
   52,813. Lost none; gained 63; decreases 0; increases 0; digest-only 19;
   unchanged 925; issues 0. Selector: case-insensitive `\b(last|next) turn\b`
   and every `\blast\b` face, plus 300 stock-sampled covered
   `\b(this|that|each|every) turn\b` controls and 300 random covered controls
   (seed 20261008). All 117 *last* faces are included. Equal-count replacements
   through `PremodifiedNominal` → `Adjective`, with *last*'s sole Word owner:
   Arboria 37→37; Avenge 12→12; Beckoning Will-o'-Wisp 3→3; Chromatic Armor
   27→27; Divine Intervention 7→7; Duplicant 76→76; Giant Turtle 9→9; Goblin
   Rock Sled 36→36; Halls of Mist 2→2; Jandor's Ring 4→4; Koh, the Face
   Stealer 14→14; O-Kagachi, Vengeful Kami 7→7; Rally the Horde 896→896;
   Shape of the Wiitigo 14→14; Suffocation 234→234; Tangle Kelp 34→34;
   Triarch Stalker 2→2; Weathered Sentinels 15→15; Wiitigo 38→38. All selected
   replacement analyses retain ordinary premodification; no ordinal-only
   position is missing. Stock `diff --lines` and identity manifest:
   `target/english-v3/english-v3-last-turn-temporal-adjunct/{diff,manifest}.json`.
   After grammar-bearing refresh (`wyrzqktw`, inherited `umzuyqxmtqpx`), the
   same manifest still covers 895 faces / 52,813 Readings. Final stock
   `diff-final.json` against the claim-parent baseline: lost 0, gained 63,
   decreases 0, increases 0, digest-only 193, unchanged 751, issues 0.
   Stock `diff-refresh.json` against the pre-refresh result: no coverage or
   count changes, 182 equal-count digest changes inherited from
   `umzuyqxmtqpx` (division/passive and scalar-NP representation); no
   decreases require attribution or restoration. All *last* faces retain
   their coverage and counts.
2. **Witnesses** (`xmyznkll`; unchanged at `wyrzqktw`): pre-implementation
   probes on the claim parent's grammar all had 0 Readings. After: First
   Response 1; Lambholt Elder 2;
   Silverpelt Werewolf 1; Call of the Full Moon 1; Feast on the Fallen 1.
   These are witness-line counts; census face totals sum all lines. Already
   reading before implementation: *the next turn* 1; *the last time* 2;
   Peace Talks 20 (complete). They were dropped from the red list; *next*
   receives no new licence (orchestrator ruling R14c).
3. **Spot checks** (`xmyznkll`): First Response and Feast on the Fallen attach
   *last turn* to the *lost life* predicate; Lambholt Elder, Afflicted
   Deserter, Mayor of Avabruck, Reckless Waif and Sage of Ancient Lore attach
   it to the passive *no spells were cast* predicate; Silverpelt Werewolf
   and Call of the Full Moon attach it to *cast two or more spells*; Paladin
   of Atonement attaches it to *lost life*. Each uses `NominalAdjunctPhrase`
   form 1 with the declared deictic Adjective dependent and singular count
   *turn*, inside the conditional clause, rather than an extra Object or
   Subject. Probes are complete, with no issues; both laws are checked by
   the census and the typed witness tests.
4. **Deviations and STOPs**: the R14b STOP found that generated Numerals have
   no declared-property channel. [Orchestrator ruling R14e (2026-10-09)](#r14e-resolution-2026-10-09)
   resolved it by authorizing the sole Word owner for *last*, mirroring
   *next*, and retiring the exceptional ordinal value −1 on parsing and
   generation; the numeral-property alternative was not authorized. The
   remaining *to last* scanner component supports other sequence-relative
   ordinals. No new Construction or frame; R14a adds a form to the existing
   `NominalAdjunctPhrase`. The construction core now permits forms with
   different lexical Categories stored in the same Word field and selects
   each form's declared Category for parsing, matching admission. One core
   test covers that generic obligation. The glossary term and feature are
   renamed `NominalAdjunctDependent` because the dependent can be an
   Adjective as well as a Determinative. Ordinary premodification is
   re-spelled through the existing adjective route; no superseded *last*
   ordinal route remains. Refresh's append conflicts in the core tests and
   English test registry were resolved with Harmony by retaining both
   trunk's additions and this ticket's additions; the refresh retry exited
   0, `conflicts() & ancestors(@)` is empty, and no test was dropped. No unresolved STOP.
5. **Tests** (`xmyznkll`): all final commands passed:
   `cargo test -p deckmaste_english_v3`;
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`;
   `cargo test -p deckmaste_lexical`;
   `cargo clippy -p deckmaste_lexical --tests -- -D warnings`;
   `cargo test -p deckmaste_lexical_source`;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`;
   `cargo test -p deckmaste_construction_v3_core`;
   `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings`.
   Initial English clippy found `validate` at 152/150 lines; extracting the
   unchanged storage-compatibility predicate resolved it. English's full
   suite and clippy were repeated after that fix; the other suites and
   clippy ran once. Added 8 (7 English, 1 core); re-spelled 6 (5 numeral
   tests, 1 lexical-source feature test); restored 0; ignored 0; removed 0.
   Citation checks: no noncompliant sites, 0 stale citations; no new CR
   citations require blessing. Grammar-bearing refresh verification
   (`wyrzqktw`): the same eight full-suite/clippy commands were repeated
   and all passed, with no ignored or filtered-out tests.
6. **Timing** (`xmyznkll`): bounded census wall 36.08 s under `/usr/bin/time -v`,
   6 workers, prebuilt `cargo-xtask`, `--samples-per-face 0`; peak RSS
   3,024,828 KiB. Required refresh subset check (`wyrzqktw`): 22.65 s wall,
   6 workers, same prebuilt-binary options; peak RSS 2,974,044 KiB.
   No full census was run.
