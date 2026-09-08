---
needs: []
---
**`ActFacts` and its column types are still mirrored as generated registry
facts, and no generated table uses them any more.** Raised by
`facts-act-columns-are-lean-data`, which moved the `actFacts` table out of the
generated module: `Check/FactTypes.lean` still declares `ActFacts`, `DeedRole`,
`ReferentSort`, `EntityDomain`, `ObjectClass`, `PremiseSort` and `DeedFeature`,
and `crates/deckmaste_semantics_v2/src/facts.rs` still mirrors all seven,
though no Rust code reads any of them and every table that carries them
(`coreDeedFacts`, `abilityDeedFacts`, `actFacts`) is now hand-written in
`Check/Words.lean`.

`tests/lean_drift.rs` pins `Check/FactTypes.lean` against `facts.rs` in both
directions, so the Rust mirror cannot be dropped alone: the seven declarations
would have to move to a Lean module the drift law does not cover (the checker's
own `Check/Words.lean` is the obvious home). That changes which Lean modules
the mirror contract covers — `docs/decisions/semantics-v2.md` §10 — so decide
it there rather than in a generator ticket. Standard constraints apply.

## Landing record

Stamped on `tvqlspyo` (`tvqlspyokrvtlpusznvwymuopkkusmkq`), on top of
`nvlpwwyy` (this workspace's `facts-participle-column-reconciled` landing, so
the moved `ActFacts` is its final shape); `plugins_v2/canon` 118 cards,
`plugins_v2/testing` 2 cards, 65 keyword-action declarations, 65 `actFacts`
rows -- every one of those counts unchanged by this landing. The english_v2
coverage lock is not in this lane (no `deckmaste_english_v2`, `core_verbs.ron`
or corpus change), so no lock `covered` count or construction count applies.

### PROVE

- **No silent loss.** Nothing stopped being covered. The seven declarations
  moved verbatim: `EntityDomain`, `ObjectClass`, `ReferentSort`, `DeedRole`,
  `PremiseSort`, `DeedFeature`, `ActFacts`, together with the attendant
  definitions that are meaningless without them and would otherwise dangle
  (`EntityDomain.admits`, `EntityDomain.kinds`, `noRole`, `DeedRole.types`,
  `playerAgent`, `fieldObject`, `permanentTypes`, `spellTypes`, `allTypes`) --
  one contiguous 88-line block cut from `Check/FactTypes.lean` and pasted into
  `Check/Words.lean`, byte-identical, doc comments and `[CR#102.1,109.1]`,
  `[CR#109.1]`, `[CR#701.1]`, `[CR#701.29a]` cites included. No column, row,
  constructor, default or citation was added, dropped, reordered or re-valued.
  `cargo xtask lean-check plugins_v2/canon` proves 118/118 and
  `plugins_v2/testing` 2/2, unchanged; `lake build` completes 80 jobs.
- **Structural laws.** `cargo xtask facts check` reports both generated modules
  (`lean/Semantics/Check/Facts.lean`, `idris/src/Experimental/FactsGen.idr`) up
  to date, so the generator's output is untouched -- as it must be, since none
  of the seven appears anywhere in the generated `Facts.lean` (checked by
  grep across all seven names plus `noRole`/`playerAgent`/`fieldObject`: zero
  hits). `corpus` (byte-exact declaration and card read/write round-trip)
  green.
- **The drift law is not weakened, and is not vacuous.** `tests/lean_drift.rs`
  still pins `Check/FactTypes.lean` against `facts.rs` in both directions, with
  the same `PAIRS` entry, the same comparison and the same non-vacuity floors
  (`declarations >= 180 && members >= 800` in
  `the_drift_scan_finds_the_whole_syntax`, `names >= 800` in
  `every_lean_name_round_trips_through_the_emitters_mapping`); no assertion,
  floor, allowlist or `PAIRS` entry was changed, and no test enumerated the
  seven by name, so nothing needed re-spelling. Probed on the landed tree that
  the pin still refuses, in both directions: adding a `probe : Bool` field to
  `SubtypeFacts` in `FactTypes.lean` fails
  `rust_mirrors_the_lean_syntax_declaration_for_declaration` with
  "`SubtypeFacts::(fields)` fields differ / Lean: [probe, subtype, frame] /
  Rust: [subtype, frame]", and adding the same field to the Rust
  `SubtypeFacts` fails it with the two lists the other way round. Both probes
  reverted; `jj diff --stat` is identical before and after.
- **No word-naming.** No guard was added or changed. The moved block's guards
  read declared features exactly as before (`DeedFeature`, `PremiseSort`,
  `EntityDomain`); the landing introduces no new one.

### DISCLOSE

- **Where the seven now live.** `lean/Semantics/Check/Words.lean`, immediately
  under the existing `/-! ## The act facts table -/` header and above
  `coreDeedFacts`, `abilityDeedFacts` and `actFacts` -- the three hand-written
  tables that are their only users.
- **What imported them, and why no import line changed.** Exactly one module
  imports `Semantics.Check.FactTypes`: the generated
  `lean/Semantics/Check/Facts.lean` (`import Semantics.Check.FactTypes`, the
  generator's fixed header in `crates/xtask/src/facts/lean.rs`). Every consumer
  of the seven reached them transitively through it and now reaches them
  through `Check.Words` instead: `Check/Words.lean` (`import
  Semantics.Check.Facts`) declares them itself; `Check/Events.lean` (`import
  Semantics.Check.Words`) uses `DeedRole`, `noRole` and `PremiseSort`;
  `Check/EventContext.lean` reaches `Check.Words` through
  `Check.Phrase -> Check.Events`; `Check/Abilities.lean` uses `PremiseSort`
  through `Check.Triggers -> Check.PhraseRules -> Check.EventContext -> ...`;
  `Proofs/Anaphora.lean` and `Proofs/Tables.lean` likewise. Because
  `Check.Words` already sits below `Check.Facts` in the import graph, and the
  block moved downward along that same edge, no import line in the tree needed
  adding, removing or reordering. `FactTypes.lean` keeps `import
  Semantics.Words` and `import Semantics.Events` unchanged.
- **What was deleted from Rust.** All seven mirrors in
  `crates/deckmaste_semantics_v2/src/facts.rs` -- `EntityDomain`,
  `ObjectClass`, `ReferentSort`, `DeedRole`, `PremiseSort`, `DeedFeature`,
  `ActFacts` -- plus the `no_role()` serde default that existed only to type
  `ActFacts`' two role columns (105 lines). No Rust code read any of them
  (grepped for each name across `crates/`: the only hits were
  `deckmaste_core::ObjectClass`, an unrelated type). Every `use` line in
  `facts.rs` is still needed by a surviving declaration, so none was removed;
  `SubtypeFacts`, `DesignationFacts`, `CounterFacts` and `KeywordFacts` keep
  every column they had.
- **The contract amendment.** `docs/decisions/semantics-v2.md` §10 gains one
  paragraph: `Check/FactTypes.lean` holds the row types of the tables
  `cargo xtask facts generate` writes and only those, the mirror contract
  covers exactly them, a checker-owned fact type is declared beside its
  hand-written table in `Check/Words.lean` outside the mirror with no Rust
  counterpart, and moving a fact type between the two homes is a change to what
  the drift test governs and belongs in the ADR.
- **Deviations and additions.** Three, all documentation, none structural.
  (1) `FactTypes.lean`'s module doc, one line, replaced by four saying it now
  holds only the generated tables' row types -- the old line ("the data columns
  shared by generated registry facts and their consumers") became false the
  moment the checker's own columns left. (2) `Check/Words.lean`'s module doc
  gains two lines for the same reason, replacing the bare "Registry data lives
  in `Check.Facts`; this module interprets its declared columns", which no
  longer accounted for the types this module now declares. (3) `facts.rs`'s
  module doc gains a four-line paragraph saying only the generated tables' row
  types are mirrored and `ActFacts` has no Rust mirror at all. The attendant
  definitions listed under "No silent loss" moved with the seven; they are not
  an addition, but the ticket names seven declarations and nine more
  definitions travelled with them, so it is disclosed rather than assumed.
  Nothing else was added or deleted; no test, table, row or proof was touched.
- **STOPs.** None. In particular no ticket-vs-ruling contradiction arose: the
  ticket routes the mirror-coverage question to §10 and §10 is where it was
  decided.
- **Glossary.** No gap. The landing introduces no term; `Deed`, `Keyword
  Action` and `Referent` are already defined in
  `docs/contexts/game-model/CONTEXT.md`, and nothing was renamed.
- **Assurance counts.** Restored 0; re-spelled 0 (no test named any of the
  seven -- `lean_drift.rs` compares whatever the two files declare, so it
  followed the move without an edit); ignored 0; added 0; removed 0.

### REPORT

- Line counts: `lean/Semantics/Check/FactTypes.lean` 176 -> 90 (89 removed --
  the 88-line block and its blank separator -- 3 added by the module doc);
  `lean/Semantics/Check/Words.lean` 1,632 -> 1,723 (+91: the 88-line block, a
  blank, and 2 module-doc lines);
  `crates/deckmaste_semantics_v2/src/facts.rs` 257 -> 157 (105 removed, 5
  added); `docs/decisions/semantics-v2.md` +11. Whole diff: 112 insertions,
  196 deletions across 4 files.
- Gate: `cargo xtask gate --changed` derives
  `cargo test -p deckmaste_semantics_v2 -p xtask`; run with `--run`, green --
  32, 6, 4, 29, 487 (1 ignored, pre-existing), 12, 1, 4 and 5 passed across the
  unit and integration targets, 0 failed. Also run as this ticket's Lean
  oracles, outside the derived closure: `lake build` (80 jobs, "Build completed
  successfully"), `cargo xtask lean-check plugins_v2/canon` (118/118) and
  `plugins_v2/testing` (2/2), `cargo xtask facts check` (both generated modules
  up to date). `cargo fmt --all` clean;
  `cargo clippy -p deckmaste_semantics_v2 -p xtask --all-targets` clean, no
  warnings. `cargo xtask cite check --list-noncompliant` 0 non-compliant;
  `cite check` 15,848 citations, 0 stale, against 15,853 before -- both of the
  code tree, measured before the two landing records were appended; the five
  rule numbers lost are the four deleted Rust mirror sites, which carried
  `[CR#102.1,109.1]`, `[CR#109.1]`, `[CR#701.1]` and `[CR#701.29a]`, and the
  same four sites survive on the Lean side, moved verbatim. With both records
  written the tree total is 15,863, 0 stale. `jj diff --git | cargo xtask cite
  audit --diff` audited 4 sites on the code diff and 17 with the records
  included; every one of the 17 is one of those same four rules, and each rule
  text was read against its claim -- [CR#102.1] and [CR#109.1] against
  `EntityDomain`'s player/object/either, [CR#109.1] against `ObjectClass`'s
  seven object kinds, [CR#701.1] against `DeedFeature` as keyword actions, and
  [CR#701.29a] against `opponentsLibrary` as fateseal's look at an opponent's
  library. No `bless` needed: no rule number is new.
- Performance advisory: the english coverage command is not in this lane, so
  its 16.26 s quiet-host ceiling and per-byte thread-CPU telemetry do not
  apply. Measured instead, on a host also running sibling workspaces:
  `lake build` 80 jobs from a warm cache; `lean-check plugins_v2/canon` 33.1 s
  for 118 cards, `plugins_v2/testing` 0.6 s for 2; the derived gate's `xtask`
  unit target 36.9 s, `lean_drift` 0.01 s, `corpus` 0.32 s.
