---
needs: [lean-v3-checker-rules-scope-refusals]
---
# The checker's fact tables: hand-maintained Lean now, registered by Lean declarations later

**Make `lean/Semantics/Check/Facts.lean` hand-maintained Lean and delete the RON-to-Lean
generator (Step 1), then let the Lean declarations register their own facts so a command
materialises the tables (Step 2).** Standard constraints apply. Owner ruling, 2026-10-22: "new
facts is not going to be ron, and can be hooked up through a lean syntax overlay".

`needs` `lean-v3-checker-rules-scope-refusals`, whose §5 edits the four card-class
rows of `Check/Facts.lean` and of the generator's overlay (`crates/xtask/src/facts.rs`); this
ticket deletes both, so it lands after. If that ticket is changed to delete the gates rather than
flip them, the card-class bullet below is already done. Step 2 is Lean-only: it does not need
`semantics-v3-dsl-and-plugins`, which only has to mirror whatever Step 2 settles (see "Other
tickets").

## What generates what today

`cargo xtask facts generate` writes `lean/Semantics/Check/Facts.lean` and
`idris/src/Experimental/FactsGen.idr`. The Lean side reads the `plugins_v2/builtin` RON
declarations (counter kinds, designations, relations, subtypes, keyword actions, keyword
abilities) plus overlays in `crates/xtask/src/facts.rs` (the keyword rows, `overlay()` at
`:214`, the Idris-only `Shape`/`Regime`/`Category` columns) and `crates/xtask/src/facts/lean.rs`
(`render` at `:926`, the subtype frame overlay in `subtype_rows` at `:895`). `facts check` rejects drift in
both outputs; `facts labels` audits the Idris `Words.idr` label sets against the same stubs.

## Verified consumers of the tables (all inside `lean/Semantics`)

Every reader goes through an ordinary Lean `def` (`Card.check` runs under `decide`, so the tables
must stay ordinary definitions: an environment extension cannot feed `decide`).

| Table (row type in `Check/FactTypes.lean`) | Readers | Pins |
|---|---|---|
| `keywordFacts` (`Facts.lean:73`) | `keywordFactsFor`/`knownKeyword` (`Check/Keywords.lean:20-21`); argument schemas via `keywordSchemas` (`Keywords.lean:91`), `KeywordFamily.ok` (`:101`), `KeywordTerm.known` (`:111`), `keywordSchemaFor`/`checkKeywordParamsWith` (`Check/Phrase.lean:1346-1375`, used at `:3833`), `CardFacts.lean:199`; `paidCost` readback in `LinkSource.named` (`Keywords.lean:123-124`); `wantsModes` (`Check/Card.lean:220`); card-class gates `onPermanentCard`/`onInstantOrSorceryCard` in `keywordCardOk` (`Card.lean:29-31`), reached from `classAbilityOk` (`:56`) and `cardTextOk` (`:66-67`, `Card.check` at `:278`) | `Proofs/Tables.lean:39` (distinct), `Proofs/KeywordArguments.lean`, `Proofs/Faces.lean:776-786`, `Proofs/Description.lean:353` |
| `subtypeFacts` (`:516`) | `Subtype.frame`/`framesWith` (`Check/Words.lean:1558-1561`), used at `Card.lean:75` (chapter lines) and `:311` (Adventure inset) | `Tables.lean:48` |
| `designationTable` (`:488`) | `DesignationLabel.facts`/`known`/`scope`/`holder`/`possessorOk`/`heldByItsCard`/`gameWide`, `designationHolderOk` (`Words.lean:1602-1632`), used at `Phrase.lean:81, 2273-2276, 2626, 2793, 3078, 3084` | `Tables.lean:41, 44-46, 51-53` |
| `counterFacts` (`:268`) | `counterFactsFor`/`knownCounter`/`CounterKind.scope` (`Words.lean:1666-1672`), used at `Keywords.lean:144`, `LocalShape.lean:90,93`, `Phrase.lean:1448, 2199`, `ProjAxis.scope` (`Words.lean:1678`) | `Tables.lean:37`, `Proofs/Counters.lean:315,389,419,428` |
| `relationTable` (`:511`) | `RelationLabel.facts`/`known` (`Words.lean:1639-1642`), used at `Phrase.lean:3148` | none |
| `keywordActionLabels` (`:6`) | `Proofs/Tables.lean:28, 33` only; the checker reads `actFacts` in `Check/Words.lean:142`, which is already hand-written | `Tables.lean:27-34` |

Findings that matter to the work:

- **Unread columns.** `DesignationFacts.zone` is read only by `DesignationLabel.seedZone`
  (`Words.lean:1609`), which nothing calls (the `seedZone` at `Phrase.lean:455` is
  `Predicate.seedZone`). `DesignationFacts.type` has no reader. `DesignationFacts.half` is read
  only by the pin `Tables.lean:51-53`. `RelationLabel.left`/`.right` (`Words.lean:1644-1646`)
  have no caller, so relation sides are never checked, although `lean/CONTRACTS.md:970-976` says
  `Action.relate`, `Predicate.related` and `NounPhrase.counterpart` "check each side at the kind
  the declaration gives it". That sentence is wrong today and is fixed in Step 1 (state what
  `Phrase.lean:3145-3148` actually does: it requires the label to be declared and checks both
  operands as objects). Whether to drop the dead columns is the Step 1 call below.
- **No Lean pin asserts the keyword card-class gate negatively.** Every `cardTextOk` pin over a
  keyword is positive (`KeywordByCardClass.lean:39,45`, `GiftVariants.lean:64,70`,
  `LinkedChoiceReadback.lean:88,94`); the `DrtCards.lean:52-58` and `Faces.lean:52-57` pins are
  about spell and static abilities, which are separate rules and stay. An unknown keyword is
  still refused after the gate goes: `keywordParamFits` (`Phrase.lean:3835`) fails when
  `keywordSchemaFor` finds no schema.
- **Nothing else reads the generated output.** No Rust crate, no `deckmaste_semantics_v2` test and
  no CI step consumes `Facts.lean`. `crates/deckmaste_semantics_v2/src/facts.rs` (the Rust mirror
  of `FactTypes.lean`) is read only by the xtask generator. `.github/workflows/ci.yml` runs no
  `xtask facts` command; the gates are the xtask unit tests `the_committed_module_matches_the_stubs`
  and `every_stub_has_a_row_or_a_recorded_reason` (`facts.rs:2073,2078`), run by `cargo test`.
- **The Idris output has live consumers.** `idris/src/Experimental/Words.idr:10` imports
  `FactsGen` publicly; `Card.idr:44-45,200` read its `onPermanentCard`/`onSpellCard`/`wantsModes`
  columns, and `idris/mtg.ipkg:10`, `mtg-dev.ipkg:25` list it, so the CI `idris` job
  (`ci.yml:168-227`) builds against it.

## Step 1: now, and small

1. **`Check/Facts.lean` is the source of truth.** Replace the header comment (`Facts.lean:1`),
   the `FactTypes.lean` header (`:4-7`, which still names the generator and the semantics-v2
   mirror contract) and `Proofs/Tables.lean`'s header, which says the generator made the
   action-label check "of its own overlay".
2. **Delete the generation.** `crates/xtask/src/facts.rs` and `facts/lean.rs` (about 3,500
   lines, including the Idris-only overlay and the `labels` audit, which has no table left once
   the overlay goes and audits a frozen Idris file), the `Facts` variant and its dispatch
   (`crates/xtask/src/bin/cargo-xtask.rs:18, 96, 134`), `pub mod facts` (`crates/xtask/src/lib.rs:23`)
   and the `expansions.rs:671` comment. Then delete what that leaves dead (`clippy -D warnings`
   finds it): `crates/deckmaste_semantics_v2/src/facts.rs` with `lib.rs:30` and its doc line
   (`lib.rs:5`) and the `rules.rs:111-115` doc links, and any `expansions`/`macro_def` helper only
   the generator used. `lean_emit.rs` is not a facts consumer; leave it.
3. **Freeze, do not delete, `FactsGen.idr`.** Stop generating it; change its header (`:1`) to say
   it is frozen and deleted with `idris/` by `idris-retirement`. Do not edit Idris otherwise.
4. **Remove the gates and mentions**: `lean/README.md:98-101`, `README.md:301-303`,
   `docs/keyword-policy.md:316-318`, `docs/decisions/lean-is-the-workbench.md:37`,
   and `lean/CONTRACTS.md:973` (`cargo xtask facts` writes `relationTable`). Archived ADRs and
   `done/` tickets that name the command are history; leave them. Run
   `rg 'xtask facts|facts (generate|check|labels)|FactsGen'` after and account for each hit.
5. **Drop `onPermanentCard` and `onInstantOrSorceryCard`** from `KeywordFacts`
   (`FactTypes.lean:70-71`), from every `keywordFacts` row, and delete `keywordCardOk`
   (`Card.lean:29-31`); `classAbilityOk`'s keyword arm (`:56`) becomes `true`. They record corpus
   observation, not a CR rule (ADR `semantics-v3.md` §0.1 item 5; the CR imposes no class on
   replicate, split second, wither or web-slinging, and wither works "no matter what zone"
   [CR#702.80c]). The other class rules (`.spell`, `.activated`, `.static`, `.mayBeginOnBattlefield`
   in `classAbilityOk`) are not this ticket; `lean-v3-checker-runtime-state-audit` classes them.
   `lean/CONTRACTS.md`'s section "A keyword definition split by card class" (`:899-923`) changes
   with it: delete the `keywordCardOk` bullet and retitle for what remains, the guarded halves.
   (`:80` and `:721` describe retired checks and the non-keyword class rule, and stay true.)
6. **Unread columns: decide, then say.** Either delete `DesignationFacts.zone/type/half` and
   `RelationFacts.left/right` (and the pin `Tables.lean:51-53`), or keep them and record in
   `FactTypes.lean` that nothing reads them yet. Do not delete them silently:
   `lean-v3-keyword-bodies-and-catalog-sync` §2 retires `half`, and
   `semantics-v3-designation-storage-columns` expects the designation columns back in
   `plugins_v3/`.
7. **Replace the property tests the generator carried.** For each retired xtask test, name the
   Lean pin that carries what it asserted or port it; do not just delete. Candidates:
   `hexproof_and_cycling_admit_their_cr_variants` (covered by `Proofs/Keyword.lean:352` and
   `KeywordArguments.lean:9-33`; confirm), `counter_scope_is_read_from_the_declaration` and
   `designation_columns_are_read_from_the_declaration` (the holder and scope of a handful of
   labels as `decide` pins over `CounterKind.scope` and `DesignationLabel.scope`/`holder`, which
   `Tables.lean` lacks), `a_keyword_counter_is_read_off_its_conferral` (check against
   `Rules.lean`), `every_counter_declares_a_row_labelled_with_its_name` (the `distinct*` pins plus
   Step 2). The generator's own plumbing tests (stub signatures, overlay labels, stale-module,
   Idris constructor maps) are retired with it: they test code that is gone.

### Tests, Step 1

- Rewrite `Proofs/Tables.lean`: keep the `distinct*` theorems; `everyDeclaredActionHasFacts` and
  `everyActRowIsDeclared` now compare two hand-written lists, so say so in the docstring and keep
  them (they still catch a row added to one list only; deleting them would weaken the check) until
  Step 2 derives the list.
- Add card-class pins that the gate is gone: a keyword the old rows kept to one class on a card of
  the other (flashback on a creature, flying on an instant) is `cardTextOk = true`; extend
  `KeywordByCardClass.lean` and cite [CR#702.80c] on the wither one.
- `lean/scripts/build` (`lake build --wfail`), `cargo xtask lean check`, `cargo test -p xtask -p
  deckmaste_semantics_v2`, `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D
  warnings`; the cite gates.

## Step 2: later; declarations register their facts

Decided: facts are written in Lean, not RON, as an overlay on the declarations. A declaration
registers its facts through an attribute or a custom command into an environment extension
(`Authoring.lean:154` `spliceSchemas` is the pattern); a command run after the declarations
materialises each table as an ordinary `def` holding a literal list. Only that `def` is
`decide`-visible; the extension is read at elaboration time only, to emit the `def`.

**Layering (verified).** `Check/Words.lean:2` imports `Check.Facts`, and
`Definitions/Actions.lean:2` imports `Check.Phrase`, so the composite Definitions (`Actions`,
`Keywords`) sit above the checker and can never feed it. The leaf catalogs
(`Definitions/Counters`, `Designations`, `Relations`, `Subtypes`) import only `Semantics.Primitives`,
and nothing in `Primitives`/`Templates` imports `Check`, so the checker may import them without a
cycle. So:

- **Leaf-catalog tables** (`counterFacts`, `designationTable`, `relationTable`, `subtypeFacts`):
  attach the columns to the leaf declarations; a module that imports the four catalogs runs the
  command and is imported by `Check/Words.lean`. This moves the checker above the leaf catalogs;
  `lean/CONTRACTS.md:134-146` ("the checker sits beside this stack") is reworded to say the
  leaf catalogs are the one layer the checker reads.
- **Keyword tables** (`keywordFacts`, `keywordActionLabels`): the declarations are composite, so
  the checker cannot read what they register. Two options, **the owner picks before this stage
  starts**: (A) declare keyword and keyword-action heads (name, argument schemas, `paidCost`,
  `wantsModes`) in a leaf module before `Check`, with the composite bodies keyed to them; or (B)
  keep `keywordFacts`/`actFacts` hand-written in `Check` and add a command after the composite
  Definitions that emits a `decide` theorem that the declared keywords and actions and the
  hand tables coincide both ways. Recommended: (B), because (A) splits every keyword in two. (B)
  replaces `everyDeclaredActionHasFacts`/`everyActRowIsDeclared` and adds the same pin for
  `keywordFacts`, which has no cross-check today.

Tests, Step 2: the `Tables.lean` pins become derived; add pins that a declaration with no facts,
and a facts entry with no declaration, are each refused at elaboration (`#guard_msgs`).

## Other tickets this changes (not edited here)

- `semantics-v3-dsl-and-plugins` points `cargo xtask facts check` at `plugins_v3/` with
  `--deny-drift`; that command is gone after Step 1, and so are the "re-points `xtask` (`lean`,
  `facts`)" lines of ADR `semantics-v3.md` §0.3 and the ticket list near `:552`.
- `lean-v3-keyword-bodies-and-catalog-sync` §2 cross-pins catalogs "until `semantics-v3-dsl-and-plugins`
  switches the facts generator"; there is no generator to switch. It also adds rows to
  `keywordActionLabels`: whichever lands second reconciles.
- `idris-retirement` deletes `facts generate/check/labels` "in their Idris form" and says
  `lean-facts-tables-from-registries` re-homes the generator; after Step 1 only `FactsGen.idr`
  remains for it to delete.
- `lean-v3-checker-rules-scope-refusals` §5 flips four class flags; this ticket removes the
  columns instead.
