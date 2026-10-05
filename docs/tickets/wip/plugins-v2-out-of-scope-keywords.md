---
needs: []
---
**Remove the declarations for mechanics outside the project's scope, and say
in CLAUDE.md that Unfinity is out for now.** Split from
`plugins-v2-keyword-body-defects` on 2026-10-05. Standard constraints apply.

## What is decided

- **Removed:**
  - `hiddenAgenda` — Conspiracy [CR#702.106a]; 15 cards, none legal.
  - `visit` — Attractions [CR#702.159a,717.1]; 50 entries, none
    Vintage-legal.
  - `spaceSculptor` and the `sector` designation [CR#702.158a] — Space
    Beleren is Unfinity. Owner: "Space Beleren is in a funny set. we'll add
    support for the unfinity cards at a later time, but they're a pain in the
    ass we can justifiably avoid dealing with at this time".
- **Kept:** `assemble`, as a bare verb with no body. Owner: "assemble being
  blank seems in line with CR". Steamflogger Boss is a Future Sight card, and
  the rule itself says "Cards and mechanics from the Unstable set aren't
  included in these rules" [CR#701.45a].
- **CLAUDE.md.** Add one clarifying clause to the Scope rule: Unfinity's
  eternal-legal cards are excluded for now with the rest of Unfinity. Leave
  the word "permanently" alone.

## The work

1. Delete `plugins_v2/builtin/macros/keyword_abilities/hiddenAgenda.ron`,
   `visit.ron`, `spaceSculptor.ron` and `designations/sector.ron`.
2. Remove the matching rows wherever a table lists them: the Lean facts table
   (`lean/Semantics/Check/Facts.lean`, the `HiddenAgenda`, `SpaceSculptor` and
   `Visit` rows), `crates/xtask/src/facts.rs` (the `HiddenAgenda`,
   `SpaceSculptor`, `Visit` rows and the three `…Sector` designations), and
   anything `cargo xtask facts generate` regenerates.
   *Correction (2026-10-05):* the three `…Sector` names in `facts.rs` are
   `Words.Designation` constructors of the Idris reference, which is
   deletion-bound and keeps them; a test there reads them back from
   `Words.idr`. They stay, and `facts labels` records them as Idris rows with
   no declaration (`DESIGNATION_ROWS_EXEMPT`).
3. Tests that list the removed names lose those entries:
   `crates/deckmaste_construction_core/tests/builtin_v2_designations.rs`
   (`"sector"`), `crates/deckmaste_construction_core/tests/builtin_v2_keyword_abilities.rs`
   (`"visit"`, twice). The Lean pins in `lean/Semantics/Proofs/Tables.lean`
   whose subject is the sector designations
   (`everySectorLabelIsAnEffectfulDesignation`,
   `anUndeclaredSectorIsNotADesignation`) assert properties of designation
   tables: re-spell each against another declared designation if the property
   still has a subject, and remove it only if not. Search for further sites
   before deleting (`rg -i 'hidden.?agenda|space.?sculptor|sector|\bvisit\b'`).
4. Add the CLAUDE.md clause.

## Proof (Assurance accounting)

The landing record names every test entry and pin removed or re-spelled, each
with this ticket's reason (out of scope: Conspiracy, Attractions, Unfinity),
and gives the counts: restored, re-spelled, ignored with blockers, added,
removed. Docs that describe these mechanics (`docs/designations.md`,
`docs/rules-taxonomy.md`) are listed in the record; edit them only to drop the
removed rows.

## Out of scope

`assemble`'s body (it stays blank).

## Landing record

The series, oldest first, on the claim `xprnzqrurlxs`:

- S1 `uywvursyznqt`: the four declarations retired, with their facts rows,
  test entries and Lean pins.
- S2 `nyvwyrtzwkuy`: the CLAUDE.md Scope clause.
- S3 (this record): `docs/tickets/` only.

S1 and S2 are separate commits; S2 is one clause and changes no build input.

**Proof.**

- Per-stage gate:

  | check | S1 | S2 |
  |---|---|---|
  | `cargo xtask lean-check`, `plugins_v2/canon` | 122/122 cards prove `Card.check = []` | no build input changed |
  | `cargo xtask lean-check`, `plugins_v2/testing` | 4/4 | no build input changed |
  | `cd lean && ./scripts/build` | 81 jobs, success | — |
  | `cargo xtask facts check` | both modules up to date | up to date |
  | `cargo xtask gate --changed`, run with `--no-fail-fast` | 87 binaries, 1166 passed, 0 failed, 1 ignored | the same, rerun on the S2 tree |
  | `cargo xtask cite check --list-noncompliant` / `cite check` | 0 / 0 stale (15930 citations) | 0 / 0 stale |

  The derived command: `cargo test -p deckmaste_construction_core -p
  deckmaste_lexical_source -p deckmaste_semantics_v2 -p
  deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`. The ignored
  test was ignored before this landing; no test function was added or
  removed, so the counts are the claim's counts.
- `jj diff --git | cargo xtask cite audit --diff` audited 0 sites ("nothing
  selected"); the one citation the series adds, [CR#702.158b] in
  `facts.rs`, was read against `data/rules/cr.txt` by hand (it defines the
  three sector designations). No `cite bless` was needed: the rule was
  already registered.
- `cargo xtask expansions`, claim against S1: the only differences are the
  four removed files (`designations/sector`, `keyword_abilities/hiddenAgenda`,
  `spaceSculptor`, `visit`) and the index: 1501 declarations (1453 printed,
  48 skipped) → 1497 (1449 printed, 48 skipped), 0 failed both times.
- Facts: `lean/Semantics/Check/Facts.lean` loses the `HiddenAgenda`,
  `SpaceSculptor` and `Visit` keyword rows and the `alpha sector`, `beta
  sector`, `gamma sector` designation rows; `idris/src/Experimental/FactsGen.idr`
  (also written by `facts generate`) loses the three keyword rows, 197 → 194.
  Both regenerated with `cargo xtask facts generate`, not hand-edited.
- Lexicon (`cargo xtask lexical --card-name "Black Lotus" --export`, whole
  declared inventory): 35399 → 35395 lexemes; plugin lexemes 1492 → 1488. The
  four gone are exactly `lexeme:designation/sector`,
  `lexeme:keyword_ability/hiddenAgenda`, `…/spaceSculptor`, `…/visit`. The
  CR catalog's own `Hidden Agenda`, `Space Sculptor` and `Visit` lexemes
  (`catalog:keyword-abilities.txt`) are independent of the declarations and
  stay. No English test count moved.
- Legality (`data/scryfall/oracle-cards.jsonl`): hidden agenda 15 cards,
  visit 50, none Vintage-legal; the only Vintage-legal cards whose text says
  "sector" are Space Beleren (Unfinity, the subject) and Avalanche of Sector 7,
  where it is part of the name only. No reference was load-bearing for
  another card.

**Tests and pins.** Restored: 0. Re-spelled: 2, the Lean pins in
`lean/Semantics/Proofs/Tables.lean`, each keeping its `decide` value
comparison and moved to the other enum-valued designation, day/night
[CR#731.1]:
`everySectorLabelIsAnEffectfulDesignation` →
`everyDayNightLabelIsAnEffectfulDesignation` (`["day", "night"].all
DesignationLabel.checked = true`); `anUndeclaredSectorIsNotADesignation` →
`anUndeclaredDayNightLabelIsNotADesignation` (`DesignationLabel.checked
"dusk" = false`). Ignored: 0. Added: 1 assertion loop in
`xtask` `facts::tests::recorded_reasons_name_labels_that_are_really_there`
(each new designation-row exemption names a real Idris constructor).
Removed: 0 test functions; entries removed, each for this ticket's reason
(out of scope: Conspiracy, Attractions, Unfinity):
- `builtin_v2_designations.rs`
  `builtin_v2_designations_preserve_identity_surfaces_and_definitions`: the
  `"sector"` name in the declaration list (Unfinity), and the block asserting
  `sector`'s three labels (Unfinity). The block's property, an enum-shaped
  designation declares one row per member, is still asserted by the same
  test on `dayNight`.
- `builtin_v2_keyword_abilities.rs`: `"visit"` (Attractions) from the
  `Ability`-parameter list of the parameter table (its `len()` assertion
  106 → 105, the table's own length) and from
  `unsupported_keyword_parameter_families_are_explicitly_deferred`.

**Deviations and additions.**
1. `facts labels` binds both directions for designations, and the Idris
   reference `Words.idr` still has `AlphaSector`, `BetaSector`,
   `GammaSector`. Rather than edit the deletion-bound Idris type (its
   constructor indices would move), `facts.rs` gains
   `DESIGNATION_ROWS_EXEMPT` with a recorded reason for each, the scope
   mechanism the file documents, and the check above that each names a real
   constructor.
2. `facts::tests::the_designation_map_reaches_the_idris_constructors` keeps
   `AlphaSector`, `BetaSector`, `GammaSector`: its subject is the Idris table
   reader, and the constructors are still in `Words.idr` (the ticket's
   correction above).
3. `facts.rs`'s `GATE_COLUMNS` text: "all 195 stubs" → "all 192 stubs" (the
   keyword-ability stub count).
4. `powerUp.ron`'s comment drops Visit from the list of keywords sharing its
   cause.
5. `docs/decisions/builtin-v2-macro-spelling-and-grammar.md`: a dated
   amendment after "Each catalog line maps to exactly one `.ron` file", naming
   the three `keyword-abilities.txt` lines that now have none.
6. `docs/decisions/semantics-v2.md` needed no change: its §12.1 count (314
   declarations) counts `semantic_macro` lines of `Macros.lean` with a
   same-named declaration, and `Macros.lean` has none of the four (searched).
7. `docs/designations.md` and `docs/rules-taxonomy.md` were not edited. Both
   are censuses of the CR, not of the declarations; dropping the sector rows
   would make "a complete census of the formal designations the Comprehensive
   Rules define" false. Their mentions: `designations.md` the summary row, the
   "Sector designations" section and the state-based-action sentence;
   `rules-taxonomy.md` the designation example "in the same sector", "roll to
   visit", the sector row of the designation table and two prose mentions;
   also `docs/keyword-policy.md` ("roll to visit" in the variant-format list).

**STOPs.** None.

**Left alone.** Deletion-bound v1 code: the hidden agenda comments in
`crates/deckmaste_core/src/decision.rs` and
`crates/deckmaste_semantics/src/decision.rs`;
`crates/deckmaste_english/src/catalog.rs` ("Roll to Visit Your Attractions")
and `regular-vocabulary.tsv` (`sector`, `visit`); `idris/src/Experimental/Words.idr`
(the sector constructors, "Roll To Visit Your Attractions"). Every other
`visit` match in the tree is the traversal verb.
`docs/tickets/maybe/keyword-stub-gate-columns.md` still says "195 stubs";
it is another ticket's text.

**Glossary.** No new term.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance advisory:
the English grammar and lexicon sources were not edited and `coverage` was
not run; the lexicon change is the four plugin lexemes above.

**Routed.** Nothing. Space sculptor and the sector designation return with
Unfinity support, which the owner has deferred; no ticket was opened.

