---
needs: []
---
**The registry declarations are typed by the Lean facts, not by v1.**
Ruling (user, 2026-09-07). `cargo xtask facts generate` writes
`lean/Semantics/Check/Facts.lean`, the tables Lean's laws read, from the
`plugins_v2/builtin` declarations plus hand overlays. It deserialises the
designation, counter, type, and subtype declaration `body` fields through
v1 `deckmaste_semantics` structs (`DesignationDecl`, `DesignationDef`,
`DesignationScope`, `DesignationShape`, `CounterScope`, `Bearing`,
`ron::options`), because those bodies were written in the english_v2 era
before any v2 semantics type existed. v1 is deletion-bound and v2 must not
read through it (`CLAUDE.md`, Crate fates).

The shape, since Lean is the spec: `lean/Semantics/Check/FactTypes.lean`
already declares the columns (`KeywordFacts`, `DesignationFacts`, the
counter and subtype rows). Mirror it into `deckmaste_semantics_v2` as a
`facts` module (name-for-name, under the drift test's discipline; it is
`Check/`-side, so extend the drift test's file list). Make the registry
declaration bodies deserialise into that mirror — a declaration carries
its own columns — and have the generator emit `Facts.lean` from the mirror
through the v2 reader. Overlays survive only for columns no declaration can
carry (state that list); the rest are deleted with their code. Delete the
`deckmaste_semantics` import from `crates/xtask/src/facts/`. `facts check`
shows both generated tables unchanged (the Idris reference is byte-stable).
The declaration bodies are rewritten by load-and-reserialise, not by hand,
and `plugins-v2-dialect` converts them again afterwards, so land this
first. Standard constraints apply.

## Landing record

Measured on `lnkllwtl d9004f55` (code) and `pwtsvlxv 0cda974d` (docs only —
markdown, so every Lean/Rust figure below is unchanged by it). Host: 24
cores, load average 2.76 / 6.59 / 5.67.

### PROVE

**No silent loss.** One loss, named and routed. Retyping the designation
bodies onto `DesignationFacts` drops three v1 `Stored` columns — `shape`,
`uniqueness`, `persistence` — which the Lean fact columns do not carry.
Seventeen of the nineteen rows existed only in `plugins_v2/builtin`
(`plugins/builtin` declares `Commander` and `Monarch` alone), so this is a
unique-copy loss: **routed to the new
`docs/tickets/planned/semantics-v2-designation-storage-columns.md`**, which
records all nineteen rows' dropped values verbatim and states the decision
(Lean column vs. `rules/` table) still to make. `scope` survives as
`DesignationFacts.scope`; an `Enum` shape survives as one row per member
(`DayNight` → "day"/"night", `Sector` → "alpha sector"/"beta sector"/"gamma
sector"). No counter conferral payload was touched: the fifteen counters Lean
names with a `CounterKind` constructor of its own [CR#122.1a,122.1b] keep
their `Counter(confers: …)` bodies, so `plugins-v2-dialect`'s
load-and-reserialise still has them to convert. No `Subtype` body was touched
for the same reason (Saga's conferral payload).

**Structural laws.**
- `cargo xtask facts check`: both generated modules byte-identical —
  `lean/Semantics/Check/Facts.lean` and `idris/src/Experimental/FactsGen.idr`
  "up to date", first run and after a `facts generate` round trip. The
  designation, counter, act, keyword and subtype tables are unchanged to the
  byte.
- `lean/scripts/build`: 80 jobs, "Build completed successfully", no warnings.
- `cargo xtask lean-check`: `plugins_v2/canon` 118/118 and
  `plugins_v2/testing` 2/2 prove `Card.check = []`; no baseline consulted.
- `cargo test -p deckmaste_semantics_v2 --test lean_drift`: 4/4. The new
  `Check/FactTypes` pair is live, not vacuous — renaming one `ActFacts` field
  in the mirror made it report `Check/FactTypes: ActFacts::(fields) fields
  differ`, and the field was restored.
- `cargo xtask gate --changed` derived
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p
  deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask`; run with
  `--no-fail-fast`, zero failures.
- `cargo fmt --all` clean; `cargo clippy` on the three changed crates emits
  nothing on any touched file.
- `cargo xtask cite check --list-noncompliant` 0; `cargo xtask cite check`
  0 stale of 15,700. `jj diff --git | cargo xtask cite audit --diff` audited
  19 sites and each rule reads for its claim. `cite bless` registers nothing
  new (it would only have *removed* two entries — [CR#717.2a] and the [CR#721]
  section — that went stale when unrelated tickets moved to `done/`, so the
  lockfile edit was reverted as out of scope). `crates/xtask/` is excluded from the citation
  sources by `cite-config.json`, so the three new cites in
  `crates/xtask/src/facts/lean.rs` were read against the CR by hand:
  [CR#122.1a] is the +X/+Y counter, [CR#122.1b] the keyword counter.

**No word-naming.** The generator now knows no designation name and no
counter name: `DESIGNATION_OVERLAY` (19 hand rows) is deleted, and the counter
classifier no longer inspects a conferral term. The three subtype frame names
in `subtype_rows` are the pre-existing overlay, untouched.

### DISCLOSE

**Decisions taken.**
1. `deckmaste_semantics_v2::facts` mirrors `Check/FactTypes.lean` whole — all
   18 declarations, including the six (`ActFacts`, `KeywordFacts`,
   `ReferentSort`, `DeedRole`, `KeywordParamSpec`, `SubtypeFacts`) whose
   columns no declaration body carries — because the drift test compares a
   Lean file against a Rust module file-for-file. Lean's `def` helpers
   (`noRole`, `playerAgent`, `allTypes`, …) are not declarations and are not
   mirrored; `noRole` and `onPermanentCard := true` reappear as serde default
   functions.
2. The declaration body is the fact ROWS the declaration contributes: a
   designation body is a `[DesignationFacts, …]` list (one row per label, so
   `DayNight` and `Sector` carry two and three), a marker counter body a
   single `CounterFacts`.
3. A counter Lean names with its own `CounterKind` constructor contributes no
   row and keeps its conferral payload. The generator reads the body through a
   two-variant `CounterBody` enum, so an unrecognised body name is a refusal
   rather than a silent skip.
4. "Through the v2 reader" is read as *through the v2 RON dialect*: bodies
   deserialise with `deckmaste_semantics_v2::ron::raw_options()`. Declaration
   discovery stays on `deckmaste_construction_core::macro_def::read_builtin_v2`
   (not a deletion-bound crate), because that is what supplies the spelling and
   parameter columns the keyword and subtype tables read, and swapping it for
   `reader::Plugin` is a different landing.
5. The conversion ran as a temporary `crates/xtask/src/bin/convert_facts_bodies.rs`
   (load through v1, reserialise through the mirror, splice the `body:` value
   in place so every comment survived) and was deleted; no body was hand-edited.

**Overlays that survive, and why** (the ticket asks for this list):
- `ACTION_OVERLAY` (65 keyword-action rows, `ActFacts`): a keyword action's
  body is its own `Enact(…)` instruction, so there is no slot for the checker
  columns — and `facts labels` already records that these role columns are a
  deontic gate authored per bench sentence, not a transcription of the
  [CR#701] entry.
- `overlay()` (the `KeywordFacts` gate columns): same reason, plus
  `KeywordAbility.ron`'s `metadata` block is `deny_unknown_fields` in
  `deckmaste_construction_core::macro_def::Metadata`, and the same table feeds
  the frozen Idris generator.
- `subtype_rows`' three frame rows (`SubtypeFacts`): Saga's body is its
  conferral payload; retyping it would delete that payload, which
  `plugins-v2-dialect` is scheduled to convert.
- Nothing else. `DESIGNATION_OVERLAY` is deleted with its code, and the
  counter `Bearing`/`CounterModification`/`CounterAbility` classifier with it.

**Ticket premise corrected (no STOP, a narrowing).** The ticket says the
generator deserialises "the designation, counter, type, and subtype
declaration `body` fields" through v1. It reads only the designation and
counter bodies; `subtype_rows` reads identity and spelling, and no `type`
declaration body is read at all. The operative requirement — no
`deckmaste_semantics` import under `crates/xtask/src/facts/` — is met in full
(`grep` reports none).

**Deviations and additions.**
- Added `docs/tickets/planned/semantics-v2-designation-storage-columns.md`
  (the routed loss above).
- Amended `docs/decisions/semantics-v2.md` §10 (the mirror now covers
  `Rules.lean`, already true before this landing, and `Check/FactTypes.lean`)
  and §11 (a registry declaration carries the checker's columns as its body).
- Refreshed three stale file-count phrases in `lean_drift.rs` and
  `lean_emit.rs` ("the six/seven syntax files") that the eighth pair falsified.

**STOPs.** None. Neither STOP condition fired: every `FactTypes.lean` column
the five tables read was derivable from a body or a surviving overlay, and
`facts check` was byte-identical on the first run.

**Glossary.** No gap. Every name is Lean's own, carried across by the
documented mirror mapping; no new Game Model or Oracle English term was
needed.

### REPORT

**Assurance counts.** Restored 0. Re-spelled 3
(`builtin_v2_designations_preserve_identity_surfaces_and_definitions`,
`builtin_v2_counter_kinds_preserve_open_phrases_scopes_and_conferrals`'s
`Energy` scope assertion, `facts::lean::tests::counter_scope_is_read_from_the_declaration`).
Ignored 0. Added 3 tests
(`a_counter_carrying_a_conferral_payload_declares_no_named_row`,
`designation_columns_are_read_from_the_declaration`,
`a_designation_declaring_no_row_is_refused`) plus one drift pair
(`Check/FactTypes`). Removed 0 tests. Eight assertions *inside* the re-spelled
designation test were removed rather than re-spelled, because their subject
column has no successor: `uniqueness` (Monarch `PerGame`, CitysBlessing
`PerPlayer`, DayNight `PerGame`), `persistence` (CitysBlessing `Permanently`,
Goaded `EffectSupplied`, Sector `EffectSupplied`), and `shape` (Goaded
`Relation`, and `Enum` on DayNight, which is now asserted as its two label
rows). All are recorded in the routed ticket.

**Provenance.** 82 files changed. Bodies rewritten: 19 designation
declarations, 56 marker counter declarations; 15 counter declarations
untouched (the `boost` and `keyword` counters). Generated tables: `actFacts`
65 rows, `keywordFacts` 197 rows, `counterFacts` 56, `designationTable` 22,
`subtypeFacts` 3 — every one byte-identical to trunk. `FactsGen.idr` 197 rows,
byte-identical.

**Performance advisory.** `lean/scripts/build` (cold `Semantics` rebuild after
the mirror landed) 70.4s wall, 6m12s user, 24 workers, load average 2.76 at
the start of the run. `cargo xtask lean-check` 33.7s for 120 cards across two
plugins. The gate closure's slowest binary was 78.7s
(`deckmaste_english_v2`'s corpus test). No coverage-command figure applies:
this landing touches no English corpus path.
