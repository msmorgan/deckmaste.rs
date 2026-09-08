---
needs: []
---
**A declaration's body is the semantic meaning of the macro; facts derive
from it; Rust owns no mapping into the RON files.** Rulings (user,
2026-09-07), superseding `facts-generator-sheds-v1`'s "a declaration
carries its own facts columns as its body" and the xtask overlays that
`facts-generator-sheds-v1` left standing.

1. **Definition nodes in Lean.** `lean/Semantics/Words.lean` (beside
   `CounterKind`) gains definition constructors, the idiom of
   `Ability.keyword (keyword) (params) (body : List Ability)`:
   `Counter (label) (holder : Kind) (confers : List Conferral)`;
   `Subtype (category) (label) (rules : List <rule record>)` carrying the
   four rules-defined conferrals (equipment, fortification, aura, saga —
   `docs/tickets/done/builtin-v2-noncreature-subtype-stubs.md`) and empty
   otherwise; `Designation (label) (scope) (effectful) (zone) (type)
   (half)` — the current `DesignationFacts` columns as a semantic node,
   with `semantics-v2-designation-storage-columns`' three dropped columns
   reconsidered as fields of it. Each with CR citation and pins, mirrored
   under the drift test. The 20 counter bodies already written as
   `Counter(name:, confers:)` are the shape; the 51 `CounterFacts` rows,
   19 `DesignationFacts` rows and 458 empty subtype bodies are rewritten
   to definitions mechanically (scoped script, comments untouched).
2. **The facts generator derives.** `cargo xtask facts generate` computes
   `CounterFacts`, `DesignationFacts`, `SubtypeFacts`, `KeywordFacts` and
   `ActFacts` rows from the declarations alone. Every xtask overlay
   retires: `ACTION_OVERLAY` (65 rows), `overlay()` (keyword gate
   columns), the subtype frame rows, `DESIGNATION_MAP`, and the
   `label_of`/`name_of` name mapping. For each column the landing record
   states one of: *derived* (from the body, with the derivation named) or
   *declared* (a field of the definition node the macro itself carries).
   A column that is neither is a STOP. `Facts.lean` and `FactsGen.idr`
   are expected byte-identical; a difference is a defect in the
   derivation unless the record shows the overlay value was wrong
   against the CR, in which case the derived value wins and the row is
   listed.
3. Then `plugins-v2-subtypes-macro-only` and a counters twin can run: a
   definition body is an expansion the macro can carry, so `Subtype` and
   `CounterKind` may join the restricted kinds.

Design-bearing; Lean first; `lean-check` 118/118 and 2/2 and
`lean/Generated` byte-identical throughout. Standard constraints apply.

## Landing record

Measured on `ypplkuru ff32f3f5` (the last code-and-docs commit; this record is
markdown, so no figure below moves with it). Host: 24 cores, load average as
stated per measurement.

### PROVE

**No silent loss.** One identity stopped being covered and one gained a
different value; both are named, and neither is a regression.

- `keywordFacts` for **Aftermath** now reads `definition := [.static, .static]`
  where the overlay wrote `[.static]`. The overlay value was wrong against the
  CR: [CR#702.127a] says aftermath "represents three static abilities", the
  declaration writes two (its first folds the CR's first two clauses into one
  exclusive play permission, as its own comment records), and the derived value
  is the declaration's own. `keywordBodyPartFits` reads the column with `elem`,
  so no card's admissibility changes. **The derived value wins and the row is
  listed** — the ticket's own rule for this case.
- Nothing else changed value. `lean/Semantics/Check/Facts.lean` is otherwise
  byte-identical to trunk across all five tables, and
  `idris/src/Experimental/FactsGen.idr` is byte-identical entire.
- Three keywords keep an overlay `definition` because their declaration writes
  no body — Echo [CR#702.30a], Cascade [CR#702.85a], Backup [CR#702.165a], each
  a STOP already recorded in its own declaration file. Deriving `[]` for them
  would have weakened `keywordBodyFits`; they are declared, not derived, and
  the overlay column shrank from 99 entries to those 3.

**Structural laws.**
- `lean/scripts/build`: "Build completed successfully (80 jobs)", no warnings.
- `cargo xtask lean-check`: `plugins_v2/canon` 118/118 and `plugins_v2/testing`
  2/2 prove `Card.check = []`.
- `cargo xtask facts check`: both generated modules up to date, on a first run
  and after a `facts generate` round trip.
- `cargo test -p deckmaste_semantics_v2 --test lean_drift`: 4/4. The new
  `Rules.lean` declarations (`Conferral`, `Definition`) and the moved
  `Words.lean` one (`DesignationScope`) are live in the scan.
- `cargo xtask facts labels`: all four tables 0 stubs without a row and 0 rows
  without a stub (designations was 3/3 mid-landing, until the four Idris name
  aliases were recorded).
- `cargo xtask gate --changed` derived `cargo test -p deckmaste -p
  deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2
  -p deckmaste_lexical_model -p deckmaste_lexical -p deckmaste_semantics_v2 -p
  xtask`; run with `--no-fail-fast`, zero failures.
- `cargo fmt --all` clean. `cargo clippy` on the four changed crates emits
  nothing on any file this landing touched (two pre-existing `map_or` lints
  stand in `builtin_v2_keyword_abilities.rs`, which it does not).
- `cargo xtask cite check --list-noncompliant` 0; `cargo xtask cite check`
  0 stale of 15,848. `jj diff --git | cargo xtask cite audit --diff` read for
  every new site. `cite bless` registers nothing new. `crates/xtask/` is
  excluded from the citation sources by `cite-config.json`, so its three new
  cites were read by hand — and one was wrong: `[CR#702.166a]` is bargain, not
  backup, corrected to `[CR#702.165a]`.

**No word-naming.** No guard added here names a lexeme, construction, verb,
noun, preposition or card. `subtype_definitions_read` names four DECLARATION
FILES, not words, and each names a STOP that retires it. The generator now
knows no keyword name, no designation name and no subtype name: the keyword
table's `word` is the `Ability.keyword` term's own keyword, `counterEligible`
is read off the counter family, and the subtype identity is the body's own.

### DISCLOSE

**Every identity newly covered.**
- `Semantics.Conferral` (4 constructors) and `Semantics.Definition` (3), in
  `lean/Semantics/Rules.lean`, with `Conferral.check`, `Definition.check`,
  `DesignationScope.narrowable` and `CounterKind.declaresName` in
  `Check/Rules.lean`, and 3 new `Refusal` constructors.
- 11 pins in `Proofs/Rules.lean`: `okChargeCounter`, `okFlyingCounter`,
  `okPoisonCounter`, `badCounterHolder`, `badNamelessCounter`, `okGargoyle`,
  `badNamelessSubtype`, `badSubtypeConfersASpellAbility`, `okMonarch`,
  `okLeftHalfUnlocked`, `badNarrowedGameDesignation`, `badNamelessDesignation`
  — 6 positive and 6 negative, every constructor of both new inductives
  reached, each `by decide`.
- Two registry declarations that close a [CR#122.1b] gap:
  `counter_kinds/decayedCounter.ron` and `exaltedCounter.ron`. The rule names
  fifteen keywords a keyword counter can be; the registry declared thirteen,
  and the two missing ones were carried by the overlay's `counterEligible`
  column instead. With them the derivation is exact and the column retires.

**Decisions taken.**
1. **The definition nodes live in `Rules.lean`, not `Words.lean`.** The ticket
   says "beside `CounterKind`", but a counter definition carries `Conferral`s,
   a conferral carries an `Ability`, and `Words.lean` sits four files below
   `Abilities.lean`. `Rules.lean` is where rules-as-data already lives
   (`SbaRule`, `ConferralRule`, `DamageResultRule`) and is the only file that
   can see everything a definition needs. `DesignationScope` moved from
   `Check/FactTypes.lean` to `Words.lean` so the syntax layer can reach it
   without a syntax file importing the checker.
2. **`Definition.counter` takes a `CounterKind`, not a label** (ticket sketch:
   `Counter (label) (holder) (confers)`). `counterFacts` is the table of
   `.named` counters — a +X/+Y counter [CR#122.1a] and a keyword counter
   [CR#122.1b] are looked up by their own constructor, not by label — and a
   bare `label : String` cannot separate the two without naming words. The
   constructor is the declared feature the derivation reads. Same reasoning for
   `Definition.subtype`, which takes a `words::Subtype` rather than the
   sketch's `(category) (label)`: `SubtypeFacts.subtype` is that type, and a
   separate `SubtypeCategory` enum would be a six-member twin of `CardType`.
   Both are deviations from the ticket's shape, recorded here for veto.
3. **`Conferral` has four flavors, not one.** The four rules-defined subtype
   records and the counter records between them write an intrinsic ability, an
   ability-free continuous property, a state-based action and a turn-based
   action, and the CR keeps those apart ([CR#305.6], [CR#113.12], [CR#704.1],
   [CR#703.1]). A single `confer : Ability` would have forced three of them
   into the ability hierarchy the aura and saga comments explicitly refuse.
4. **The three dropped designation columns stay dropped.**
   `semantics-v2-designation-storage-columns` asks whether `shape`,
   `uniqueness` and `persistence` become fields of the definition node. They do
   not: no Lean law reads them, and that ticket's own rule is "a column no law
   reads should not be a Lean column". The node carries exactly the six
   `DesignationFacts` columns. That ticket stands unchanged.
5. **`DESIGNATION_MAP` retires; four Idris name aliases replace it.** The map
   was stub-name → Idris constructor. The declared labels now reach the Idris
   constructors through `normalize` alone, bar four: `Words.Designation` spells
   the commander designation `CommanderD` (`Commander` is taken there) and
   drops the leading article from three labels while keeping it on
   `TheInitiative`. `DESIGNATION_IDRIS_ALIASES` is a mapping into IDRIS, not
   into the RON files, and retires with the reference generator.
6. **The keyword body is read shallowly.** A keyword declaration's body writes
   `Param(n)`, which only a macro expansion resolves, so it cannot be read as
   an `abilities::Ability`. `KeywordBody`/`DefinitionAbility` declare the
   fields and ignore their values, so every `Param` goes past as an ignored
   value; the read still goes through the v2 dialect, so a positionally applied
   body (buyback, kicker) reads too.

### Per-column disposition (ticket part 2)

*derived* = computed from a body, with the derivation named. *declared* = a
field of the definition node the macro carries. *STOP* = neither.

**`CounterFacts` (2 columns) — fully derived.**

| column | disposition |
| --- | --- |
| `label` | derived: `Definition.counter`'s `kind`, where it is `.named label` |
| `holder` | declared: `Definition.counter (holder)` |

The table's membership is derived too: a counter whose kind is `.boost` or
`.keyword` contributes no row, because nothing looks it up by label.

**`DesignationFacts` (6 columns) — fully declared**, each a field of
`Definition.designation`: `label`, `scope`, `effectful`, `zone`, `type`,
`half`. The number of rows a declaration contributes is declared too (day/night
two, the three sectors three).

**`SubtypeFacts` (2 columns) — one derived, one STOP.**

| column | disposition |
| --- | --- |
| `subtype` | derived: `Definition.subtype`'s own `subtype` field |
| `frame` | **STOP** — 3 rows (`Saga` `.chapters`, `Adventure` `.adventureInset`, `Room` `.doors`). The column says what the CARD FRAME a subtype sits on looks like [CR#714.1,715.1,709.5j]; no conferral expresses a frame, and the definition node has no field for one. Overlay kept, now keyed by declaration name rather than by a capitalised label. |

**`KeywordFacts` (10 columns) — three derived, one hybrid, six STOP.**

| column | disposition |
| --- | --- |
| `word` | derived: the body's `Ability.keyword` keyword. Verified equal to the capitalised declaration name for all 195 before the change, which is what retired `label_of` |
| `definition` | derived: the categories of the body's abilities, in order. 3 rows declared by overlay instead — Echo, Cascade, Backup, whose bodies are empty by a per-file STOP and whose [CR#702] entry still names the category |
| `counterEligible` | derived: the keyword has a `Definition.counter` with `kind: Keyword(...)` in the counter family. 15 rows, exactly [CR#122.1b]'s list, after the two declarations this landing added |
| `argumentSchemas` | derived from the declaration's `params`, PLUS an overlay `extra` (6 rows) for the CR-defined argument VARIANTS a keyword admits beside its declared signature — hexproof from [quality] [CR#702.11d], [type]cycling [cost] [CR#702.29e]. A declaration has one signature; the variant list has no home on it. **Partial STOP** on the variants |
| `regime` | **STOP** — 53 rows |
| `functionsOnStack` | **STOP** — 47 rows |
| `onInstantOrSorceryCard` | **STOP** — 66 rows |
| `paidCost` | **STOP** — 64 rows |
| `onPermanentCard` | **STOP** — 13 rows (false) |
| `wantsModes` | **STOP** — 4 rows |

The six are the gate columns `facts labels` already records as hand-kept: they
are what the CHECKER knows about a keyword ([CR#113.6] zones, the stack regime
its triggered body keys on, whether its cost is a paid cost), not what the
declaration says about itself, and `KeywordAbility.ron`'s `metadata` block is
`deny_unknown_fields` over spelling/grammar/noun_class/category in
`construction_core`, so a home there moves that crate's English seam. They are
ALSO read by the frozen Idris reference generator, which is the reason they
cannot simply move into Lean. Routed to
`docs/tickets/planned/facts-act-columns-are-lean-data.md`, which decides them
beside the `ActFacts` question.

**`ActFacts` (13 columns) — all STOP.** Overlay `ACTION_OVERLAY`, 65 rows;
column occupancy: `agentRole` 45, `patientRole` 15, `dest` 13, `participle` 8,
`bounded` 8, `rides` 4, `stepwise` 3, `feature` 3, `intransitive` 2, `loci` 2,
`counterfactual` 2, `plays` 2, `opponentsLibrary` 1.

Twenty-four of the 65 keyword-action declarations write no body at all, and the
41 that do write an `Enact(verb, instruction, agent)` — an instruction, with no
slot for a checker column. `participle` is not the declaration's English
participle, and the two disagree in both directions: `cast` and `activate`
declare `grammar: Verb(participle: …)` and carry no `ActFacts` participle,
while `destroy`, `discard`, `mill`, `tap` and `untap` carry one and declare
none. `dest` is partly visible in a body's `Move(to: Zone(…))` but not for the
24 bodyless actions, and `facts labels` already calls it a CR fact authored
from the [CR#701] entry.

The finding worth acting on: these columns are hand-written LEAN data living in
Rust string literals. `Check/Words.lean` already hand-writes the checker's
other two deed-fact sources — `coreDeedFacts` (15 rows) and `abilityDeedFacts`
(3 rows), the latter the same `List (Label × ActFacts)` shape — so the 65 rows'
home under "Lean is the spec" is that file, not a declaration body. Routed to
`docs/tickets/planned/facts-act-columns-are-lean-data.md`.

### STOPs, with their resolutions

1. **The four rules-defined subtype conferrals** (`equipment`, `fortification`,
   `aura`, `saga`). The ticket's own third STOP condition fired: each needs a
   constructor the syntax lacks. `aura` needs a predicate for [CR#704.5m]'s
   "attached to an ILLEGAL object or player" (`Predicate.attachment` spells
   only "not attached", so writing it would silently narrow the rule); `saga`
   needs an aggregate over a card's own chapter marks for [CR#714.2d]'s final
   chapter number; `equipment`/`fortification` need the [CR#301.5,301.6] host
   rule as a deontic over the `Attach` deed [CR#701.3a], whose `actFacts` row
   declares no patient role. **Resolution:** the four keep the v1 core record
   verbatim, each with a STOP at the head of its file, and
   `facts::lean::subtype_definitions_read` refuses any OTHER subtype
   declaration whose body is not a `Definition::Subtype`, so the exception list
   can only shrink. Routed to `plugins-v2-subtypes-macro-only`, which now
   carries the three syntax gaps as its step 1.
2. **The -1/-1 annihilation state-based action** [CR#704.5q]. `Amount` has no
   minimum of two amounts, and the rule is a `SbaRule` over a permanent with
   BOTH counter kinds rather than a conferral of one counter. **Resolution:**
   the boost half of `m1M1Counter`'s body converted, the annihilation half kept
   verbatim in the file's STOP comment, routed to the new
   `semantics-v2-counter-annihilation-sba`.
3. **The `ActFacts` and `KeywordFacts` gate columns**, above. **Resolution:**
   overlays kept, routed to the new `facts-act-columns-are-lean-data`.

### Deviations and additions

- Deviations 2 and 3 under "Decisions taken" are shape deviations from the
  ticket's sketch.
- The subtype declarations gained `params: []` beside their new body: the
  declaration reader refuses a semantic body without an explicit positional
  signature.
- Added `decayedCounter.ron` and `exaltedCounter.ron` (above).
- Added `subtype_definitions_read`, a live guard over all 462 subtype bodies.
- Added `DESIGNATION_IDRIS_ALIASES` (4 rows), replacing `DESIGNATION_MAP` (4
  rows over 7 constructors).
- Deleted `label_of`, `name_of`, `stub_shape`, `DESIGNATION_MAP`, the
  `counter_eligible` and `definition` columns of `Row` (the latter kept for 3
  rows), and the `DESIGNATION_STUBS` path constant.
- Amended `docs/decisions/semantics-v2.md` §11 and the Game Model glossary
  (`Conferral` gains its four flavors; `Registry Definition` is new, the term
  this landing needed and the glossary did not define).
- No test was deleted. One was deleted BY ACCIDENT mid-landing — a
  `rindex('}\n')` in an edit script ate
  `noncreature_subtype_nursery_rejects_a_redundant_default_plural_override` —
  caught by an unused-import warning and restored verbatim.

### Assurance counts

Restored 1 (the accidental deletion above). Re-spelled 7:
`rules_defined_conferrals_stay_on_their_subtype_declarations` (its
"must not infer a semantic conferral" half, now asserting the exact identity
body), `builtin_v2_creature_type_nursery_matches_catalog_and_attested_morphology`
(same), `builtin_v2_counter_kinds_preserve_open_phrases_scopes_and_conferrals`
(three body assertions), `facts::lean::tests::a_counter_carrying_a_conferral_payload_declares_no_named_row`,
`facts::lean::tests::a_positional_body_reads_to_the_same_row`,
`facts::lean::tests::new_keyword_requires_a_gate_overlay` (its fixture now
carries a body, as every keyword declaration does),
`facts::tests::recorded_reasons_name_labels_that_are_really_there` and
`facts::tests::the_designation_map_reaches_the_idris_constructors` (both now
read declared labels). Ignored 0. Added 12 Lean pins and 1 Rust guard. Removed
0 tests. Three count assertions were updated for the two new counter
declarations (`builtin_v2_counter_kinds`' name list,
`builtin_v2_noun_classes`' 71 → 73, `declaration_terms`' 30 → 32); each is the
same assertion over a registry that grew by two.

### REPORT

**Provenance.** 566 files changed. Bodies rewritten: 71 counter declarations
(56 marker, 13 keyword, 2 boost), 19 designation declarations, 458 subtype
declarations; 4 subtype declarations deliberately not rewritten. Generated
tables, all byte-identical to trunk but for the one Aftermath row: `actFacts`
65, `keywordFacts` 197, `counterFacts` 56, `designationTable` 22,
`subtypeFacts` 3; `Facts.lean` 359 lines. `FactsGen.idr` 197 rows,
byte-identical. Overlay rows retired: 99 `definition` entries (3 kept), 15
`counter_eligible` entries (all), 4 `DESIGNATION_MAP` entries (replaced by 4
Idris aliases). Overlay rows standing: `ACTION_OVERLAY` 65, `overlay()` 197
rows over 6 gate columns plus 6 `extra` variants, `subtype_rows` 3 frame rows.

**Performance advisory.** `lean/scripts/build` after `lake clean`: 2m17s wall,
7m49s user, 80 jobs, 24 workers, load average 11.28 at the start of the run
(a sibling workspace was building). Trunk's own cold build, measured the same
way on the same tree before any change, was 2m14s wall / 7m41s user at load
1.29, so the two new inductives and their pins cost nothing measurable. `cargo
xtask lean-check` 33.6s for 120 cards across two plugins at load 7.11. The
derived gate closure ran in 1m56s wall / 19m22s user at load ~4.5. No English
coverage figure applies: this landing adds two counter-kind declarations but
runs no corpus command, and `english_v2`'s own suite reports its inspect gate
at 0.0002s against the 16.260s ceiling.
