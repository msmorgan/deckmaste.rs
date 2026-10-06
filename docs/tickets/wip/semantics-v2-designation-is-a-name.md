---
needs: [semantics-v2-counter-kind-is-a-name]
---
**A designation reads bare by its declaration name (`monstrous`, `goaded`),
not as a string.** Split from `semantics-v2-counter-kind-is-a-name` on
2026-10-05 because it is a second landing over a different type; it follows
that ticket so the typed-name mechanism is built once. Lean first; standard
constraints apply.

## What is decided

- Owner, 2026-10-05, on reading designations bare: "I'd prefer to avoid
  stringly typed so yes."
- Orchestrator's call (the owner did not object): `DesignationLabel` becomes a
  small proper type rather than a `String` alias (Lean `abbrev
  DesignationLabel := String`, `lean/Semantics/Words.lean` ~L169; mirror
  `pub type DesignationLabel = String`,
  `crates/deckmaste_semantics_v2/src/words.rs` ~L265), so the `denoted_by`
  marker attaches to it the way it does to `CounterKind` after
  `semantics-v2-counter-kind-is-a-name`.
- Owner, 2026-10-05 ("yes."): each label is its own bare name (`day`,
  `night`, `monarch`). A declaration with one label names it by the
  declaration name, verbatim, as for counter kinds; a declaration with
  several labels lists its members, each its own name (`dayNight` declares
  `day` and `night`). A phrase label such as "the monarch" stays as the
  declaration's `spelling`, not its name.

## Inventory the work covers (read 2026-10-05)

The 18 declarations under `plugins_v2/builtin/macros/designations/` carry 19
labels. One declaration has more than one label:

| Declaration | Labels | Spelling |
|---|---|---|
| `dayNight` | "day", "night" | "day or night" |

Seven have a label that is a phrase, not the declaration name; each label
becomes the name and the phrase stays in `spelling` (already equal to the
label in all seven):

| Declaration | Label today | Spelling |
|---|---|---|
| `citysBlessing` | "the city's blessing" | "the city's blessing" |
| `enduringStory` | "an enduring story" | "an enduring story" |
| `initiative` | "the initiative" | "the initiative" |
| `leftHalfUnlocked` | "left half unlocked" | "left half unlocked" |
| `monarch` | "the monarch" | "the monarch" |
| `rightHalfUnlocked` | "right half unlocked" | "right half unlocked" |
| `ringBearer` | "Ring-bearer" | "Ring-bearer" |

The other ten already label themselves with their own name and spelling:
`commander`, `goaded`, `harnessed`, `level`, `monstrous`, `prepared`,
`renowned`, `saddled`, `solved`, `suspected`.

## The work

1. **Lean.** Retype `DesignationLabel`; the designation facts lookup and
   `RoomHalf.designation` (`Check/Words.lean`) follow. Pins keep their asserted
   outcomes.
2. **Mirror.** Follow in `crates/deckmaste_semantics_v2`, with `denoted_by` on
   the new type; `lean_drift` holds.
3. **Declarations.** The designation declarations under
   `plugins_v2/builtin/macros/designations/` write their own label as a
   string today (`Designation(label: "monstrous", …)`); they take the name.
4. **Bodies.** Re-spell the string labels in the 11 files that write one
   (`gainDesignation`, `setGameDesignation`, `makeMonstrous`, `designated`,
   `hasDesignation`, `yourCommander`, `goad`, `harness`, `monstrosity`,
   `suspect`, `renown`): `hasDesignation("renowned")` becomes
   `hasDesignation(renowned)`.

## What the counter landing gives this one

`semantics-v2-counter-kind-is-a-name` built the pattern; it is three pieces,
each copyable:

- **The type.** Lean `inductive CounterKind | named (name : String)` with a
  `CounterKind.name` accessor; mirror `enum CounterKind { Named { name: String
  } }` with `name()`, carrying `#[macro_ron(denoted_by(Counter, term =
  crate::rules::Definition::counter_term))]` unchanged. A one-constructor
  inductive because `denoted_by` attaches only to an enum. For designations:
  `DesignationLabel.named (name : String)`, mirror `Named { name }`, and a
  `#[macro_ron(denoted_by(Designation, term =
  crate::rules::Definition::designation_term))]` with a new
  `designation_term` projection (Lean `Definition.designationTerm`; the doc
  comment on `Definition.subtypeTerm` already anticipates it).
- **The name comes from the meta, not the body.** The `denoted_by`
  projection reads the EXPANDED body, which does not know the macro's name,
  so the definition node must carry it, and a declaration must not write it
  (that would be self-reference). The counter meta takes the definition's
  fields as parameters and writes `Counter(kind: Named(name: Param(name)), …)`
  itself; `meta/Designation.ron` would take `scope`, `effectful`, `zone`,
  `type`, `half` and write `Designation(label: Named(name: Param(name)), …)`.
  A declaration with several members (`dayNight`) cannot take its labels from
  its own name; how its meta lists the members is part of this work.
  `deckmaste_construction_core`'s declaration schema then needs a
  `DiagnosticDesignation` with a derived body (see `DiagnosticCounterKind`
  and `DiagnosticSubtype` in `macro_def.rs`).
- **Facts.** The generator reads each body through the builtin plugin's
  macros (`counter_definitions` in `crates/xtask/src/facts/lean.rs`) and
  refuses a term whose name is not the declaration's own; the facts row's
  label is the name. Every Lean pin, bench card and RON body that writes a
  label string then re-spells to the name (`.named "monstrous"`, bare
  `monstrous` in RON); the counter landing's normaliser for the
  expansion/card-dump comparison is at the orchestrator's scratchpad
  (`counters/normalise.py`).

One difference: some designation labels are today the spelled phrase
("the monarch", "left half unlocked"), not the declaration name, so those
labels change text (the inventory above), and any check that reads the label as English (Room
halves, `RoomHalf.designation`) must read the name instead.

## Proof

`cargo xtask lean-check` passes; `cargo xtask expansions` before/after is
identical except for the label's printed form, every change listed.

## Related

`semantics-v2-designation-storage-columns` (planned) owns the storage columns
the designation declarations lost; it does not block this.
`plugins-v2-out-of-scope-keywords` removes `sector`; whichever lands second
has one designation fewer to re-spell.

## Landing record

The series, oldest first: `txpxmvsk` (Lean `DesignationLabel`, checker,
facts generation, Rust mirror, pins, the 18 declarations' 19 labels and the
11 call sites written raw as `Named(name: …)`), `plruvvku` (the
`Designation` meta builds the definition from the name and columns;
`dayNight` lists its members; loader, facts generator and
`deckmaste_construction_core` follow), `wpmszouy` (call sites write the bare
name; the corpus guard), `vsorslsp` (ADR, glossary, CONTRACTS), and this
record. Each stage was gated on its own tree. No English grammar, lexicon or
corpus input changed. As in the counter landing, the mirror cannot read a
string label, so the declarations' labels and the call sites moved with the
model in the first stage (as the raw constructor), and the later stages
changed no expanded term.

**The type.** Lean `inductive DesignationLabel | named (name : String)`
(`lean/Semantics/Words.lean`), with `DesignationLabel.name`
(`Check/Words.lean`); mirror `enum DesignationLabel { Named { name: String }
}` with `name()`, carrying `#[macro_ron(denoted_by(Designation, term =
crate::rules::Definition::designation_term))]`; new projections
`Definition.designationTerm` / `Definition::designation_term`. The kind
`DesignationLabel` is registered from the derive (`ron.rs`), and the
designation meta names it second (`kinds: [Designation, DesignationLabel]`),
as `KeywordAction` names `Instruction`, so a declaration's name reads at a
designation position. No expander feature: `denoted_by` attached unchanged.

**The meta.** `macros/meta/Designation.ron` takes `name`, `params`,
`spelling`, `grammar`, `scope`, `effectful`, `zone` (default `None`), `type`
(default `None`), `half` (default `None`) and `members` (default `[]`), and
no `body`; it builds `Designation(label: Named(name: Param(name)), scope: …,
effectful: …, zone: …, type: …, half: …)` and hands it over beside the
members as a record `(members, definition)`.
`deckmaste_semantics_v2::designations` turns the record into the registered
body (the one definition; with members, the list of one definition per
member, the declaration's label replaced by the member's name), exactly as
`keywords` builds a keyword declaration's wrapper. `deckmaste_construction_core`
gained `DiagnosticDesignation` (derived body, unknown `body:` refused).

**`dayNight`'s members.** Chosen: the meta's `members:` list, expanded by the
loader and the facts generator, rather than a second declaration file per
member. Splitting `dayNight` into `day.ron` and `night.ron` would change the
declarations the lexicon reads ("day or night" is one FixedTerm), and a file
declares exactly one macro, so the declaration keeps its spelling and lists
`members: [day, night]`; the loader registers `day` and `night` as macros of
kind `DesignationLabel` only (names, not declarations: they are not in
`Plugin::declarations`), each denoting its own definition, and drops
`DesignationLabel` from `dayNight`'s own kinds, so `dayNight` names no
designation and does not read at a designation position. The mechanism is the
members list and its expansion; nothing beyond it (no STOP).

Before → after, three declarations:

- `goaded`: `body: [Designation(label: "goaded", scope: HeldBy(Object),
  effectful: true, zone: Battlefield, type: None, half: None)]` →
  `scope: HeldBy(Object), effectful: true, zone: Battlefield`.
- `monarch`: `body: [Designation(label: "the monarch", scope:
  HeldBy(Player), effectful: true, zone: None, type: None, half: None)]` →
  `scope: HeldBy(Player), effectful: true`; spelling "the monarch" kept.
- `dayNight`: `body: [Designation(label: "day", scope: HeldByGame, …),
  Designation(label: "night", scope: HeldByGame, …)]` → `scope: HeldByGame,
  effectful: true, members: [day, night]`; spelling "day or night" kept.

**Checker.** `DesignationFacts.label` is the name (`String`, as
`CounterFacts.label`); `findDesignation` matches by name and
`DesignationLabel.facts` looks up `label.name`, so `checked`, `scope`,
`holder`, `seedZone`, `seedType` and `half` are one registry lookup by name.
`RoomHalf.designation` wraps the table's name in `.named` (no English read
remains). `Definition.check` refuses a nameless designation by
`label.name.isEmpty`.

**Facts.** `designationTable` rows: 19 → 19, same order and columns; seven
relabelled: "the city's blessing" → `citysBlessing`, "an enduring story" →
`enduringStory`, "the initiative" → `initiative`, "left half unlocked" →
`leftHalfUnlocked`, "the monarch" → `monarch`, "right half unlocked" →
`rightHalfUnlocked`, "Ring-bearer" → `ringBearer`; the other twelve
(`commander`, `day`, `night`, `goaded`, …) already were the name. The
generator reads each declaration through `designations` and refuses a
definition whose label is not the name it is declared under. Idris aliases
(`DESIGNATION_IDRIS_ALIASES`) are now `commander` → `CommanderD` and
`initiative` → `TheInitiative`; `FactsGen.idr` unchanged.

**Pins.** Every pin keeps its asserted value; the build proves each by
`decide` as before, none went from refused to accepted or back. Re-spelled
(string label → `.named "<name>"`): `Tables`
`everyDayNightLabelIsAnEffectfulDesignation` (`["day", "night"]` →
`[.named "day", .named "night"]`), `bothDoorLabelsAreDeclaredHalves` (RHS
`["left half unlocked", "right half unlocked"]` →
`["leftHalfUnlocked", "rightHalfUnlocked"]`),
`aRuleOnlyDesignationIsNotEffectful` (`"commander"` → `.named "commander"`),
`anUndeclaredDayNightLabelIsNotADesignation` (`"dusk"` → `.named "dusk"`);
`Rules` `okMonarch`/`okLeftHalfUnlocked` (their defs `monarch`,
`leftHalfUnlocked`), `badNarrowedGameDesignation`, `badNamelessDesignation`
(`""` → `.named ""`); `Static` `okBecomesMonarch`, `badGoadedPlayer`,
`okBecomesMonstrous`, `badGainsCommander`, `okGetsAnEnduringStory`;
`Description` `okObjectDesignation`, `badObjectMonarch`; `Deontic`
`okRingBearerHolder`, `badMonarchHolder`, `okRingBearerOnBattlefield`,
`badRingBearerInGraveyard`; `Faces` `okPossessedCommander`,
`badPossessedMonarch`; `Lookback` `designatedSpellCanBeMatched`,
`designatedSpellRetainsItsReference`; `Zone` `okGoadOnBattlefield`,
`badGoadInGraveyard`; `Anaphora` `okNoHolderOnPlayer`,
`badNoHolderOnObject`. The refusals that carry a label now print it as
`.named "<name>"` (`.designationHolder (.named "monarch") .object`, …).

**Call sites.** RON (`plugins_v2/builtin/macros`): 11 sites in 7 files
write the bare name — `hasDesignation(<name>)` 4 (`monstrosity`, `renown`,
`harness`, and `HasDesignation` in `makeMonstrous`), `gainDesignation(…,
<name>)` 6 (`monstrosity`, `renown`, `harness`, `goad`, `suspect`, and
`GainDesignation` in `makeMonstrous`), `Designated(commander, You)` 1
(`yourCommander`). No card writes a designation; `gameIs`, `noHolder`,
`thereIsNo`, `gameBecomes`, `setGameDesignation` and `designated` are
aliases with a parameter and changed nothing. Lean: 66 string labels became
`.named "<name>"` across 20 files (bench cards 23, `Macros.lean` 5, pins 38);
Lean has no denotation of a declaration's name, so it writes the
constructor, as the counter landing's bench does. The corpus guard
`no_source_file_writes_out_an_elided_constructor` now also refuses
`Named(name:` outside `macros/meta/`, the one place a name is built.

**Proof.**
- `lean/scripts/build` on `txpxmvsk` and the final tree: Build completed
  successfully (82 jobs).
- `cargo xtask lean-check` on every stage: canon 127/127, testing 5/5
  (93.2s on `txpxmvsk`, then 2.7s and 2.4s: the emitted Lean was
  byte-identical to the stage before).
- `cargo xtask facts check`: up to date on every stage.
- Gate: `cargo xtask gate --changed` derived `cargo test -p
  deckmaste_construction_core -p deckmaste_lexical_source -p
  deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3
  -p xtask` on every stage; run with `--no-fail-fast`: `txpxmvsk` 92
  binaries, 1199 passed, 0 failed, 2 ignored (both ignored with named
  blockers, pre-existing); `plruvvku` 1202/0/2 (two facts tests re-spelled
  after the first run, then `-p xtask` rerun 374/0/1); `wpmszouy` 1202/0/2
  plus the reader test that the first run compiled as a nested item, rerun
  with `-p deckmaste_semantics_v2` 86/0/0, so 1203/0/2. `vsorslsp` changed
  docs only.
- `cargo xtask cite check`: 0 stale; `--list-noncompliant`: 0; `cite audit
  --diff` read for every stage. No new rule registered.
- `cargo xtask expansions` before (`oxyxrpkn`) and after: 1511 declarations,
  1463 printed, 0 failed both times. Changed 33 files: the 18 designation
  declarations, and 15 helper and keyword declarations whose printed term or
  sample names a designation (`gainDesignation`, `hasDesignation`,
  `designated`, `gameIs`, `noHolder`, `thereIsNo`, `gameBecomes`,
  `setGameDesignation`, `makeMonstrous`, `yourCommander`, `renown`, `goad`,
  `harness`, `monstrosity`, `suspect`). Per-card dumps (Rust `Debug` and
  emitted Lean): 3 canon cards changed (Disrupt Decorum, Gluttonous Cyclops,
  Knight of the Pilgrim's Road), each by a designation printing as its name.
  A normaliser mapping each name back to its retired label string, each
  single declaration's `Definition` body back to the one-element list (and
  its header's kinds and read-as line), and the `Named(name: "Sample")`
  designation sample back to `"Sample A"` makes every declaration and every
  card byte-identical to the before tree: expansions 38 Debug labels, 17
  bodies and headers, 8 samples mapped; cards 5 Debug and 5 Lean labels; 0
  unexplained. Stages 2 and 3 changed no expanded term and no card dump.
- Lexicon: `cargo xtask lexical --card-name "Black Lotus" --export` is
  byte-identical before and after (and after stage 2).
- rustfmt clean on the changed Rust.

**Tests.** Restored 0. Re-spelled: 27 Lean theorems (above); Rust
`a_designation_takes_its_holder_kind_from_the_facts` (`reads.rs`, label
`"Monarch"` → `Named { name: "monarch" }`),
`builtin_v2_designations_preserve_identity_surfaces_and_definitions` (body
text: the record's `members: [day, night]` and columns),
`a_positional_body_reads_to_the_same_row` (row label),
`designation_columns_are_read_from_the_declaration` (goaded no longer writes
`half: None`), the two `Designation(name:"Monarch",…)` reader fixtures in
`macro_def/tests.rs` (now write `scope`/`effectful`, which the meta
requires); `same_plugin_game_designation_can_retain_a_conferred_body`, whose
subject (an authored designation body) is retired: re-spelled and renamed
`same_plugin_game_designation_declares_its_columns` (same declaration, its
columns in place of the body; the `confers:` assertion has no counterpart,
since a designation confers nothing); and
`a_designation_declaring_no_row_is_refused`, whose subject (a body listing no
definitions) is retired: re-spelled and renamed
`a_designation_member_that_is_no_name_is_refused` (same declaration, still
refused). Ignored 0. Added: Rust
`a_designation_macro_denotes_the_designation_its_definition_names` and three
`designations` unit tests; the `expansions` renderer for `DesignationLabel`.
Removed 0.

**Deviations and additions.**
1. New module `deckmaste_semantics_v2::designations` and the loader hook
   that registers members; `keywords::Record` became `pub(crate)` to share
   the record reader.
2. `scope` and `effectful` are required meta parameters (no semantic
   default), so the two construction-core fixtures that declared a
   designation with neither now write both.
3. The facts generator's unreachable "designation declares no fact row"
   guard is kept; with the meta, a declaration always defines at least one.
4. `writes_names` exemption (`macros/meta/`) in the corpus guard.

**STOPs.** None. No pin changed outcome; `denoted_by` needed no expander
feature; the members list needed no mechanism beyond its expansion.

**Glossary.** Designation (identified by its declared name, not the printed
phrase; one declaration may declare several) and Registry Definition (a
Designation's name is the Designation) amended in
`docs/contexts/game-model/CONTEXT.md`.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance advisory:
no English grammar, lexicon or corpus input changed.

**Routed.**
- The member mechanism and the names (`members`, `designations` module,
  `ringBearer`, `citysBlessing`, `enduringStory`, `initiative`): for the
  owner.
- `semantics-v2-designation-storage-columns` still owns the storage columns.

**After `kata refresh`.** The refresh rebased the series onto
`semantics-v2-deed-performer-roles` (the deed table's performers; explore and
endure handed to the permanent; a new testing card, Explore Handoff Probe),
`english-v3-targeting-projection-duplication`,
`english-v3-relative-clause-adjuncts` and the `english-v3-by-complement-functions`
claim, with no conflict. None of them writes a designation (no string label
in `plugins_v2` or `lean/Semantics` after the refresh). On the refreshed
tree: `lean/scripts/build` completed (82 jobs); `cargo xtask facts check` up
to date; `cargo xtask lean-check` canon 127/127, testing 6/6 (the new probe)
in 96.5s; the derived gate command, unchanged, with `--no-fail-fast`: 94
binaries, 1213 passed, 0 failed, 1 ignored (the census cross-check, ignored
on demand; the relative-clause pin it used to ignore now runs); the lexicon
export still byte-identical to the before tree; `cargo xtask cite check` 0
stale.
