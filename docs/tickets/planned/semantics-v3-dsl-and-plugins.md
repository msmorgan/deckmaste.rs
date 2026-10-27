---
needs: [semantics-v3-peer-files-and-toposort]
---
# Create deckmaste_semantics_v3 and plugins_v3 mirroring the proven Lean v3 specification

**Build `crates/deckmaste_semantics_v3` and `plugins_v3/` from the proven
`lean/Semantics/` v3 specification, with Genuine Lean 4 `{ field := val, ... }`
brace-and-comma declaration blocks and the Unified Action-Clause Spine.** Sixth
implementation step of [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)
(§0, §§1–4). Follows [`semantics-v3-peer-files-and-toposort`](semantics-v3-peer-files-and-toposort.md),
which establishes loose-bundle DAG loading and topological sorting into Lean.
Supersedes the `v2`-specific cleanup tickets `lean-macros-cutover`,
`plugins-v2-bodies-through-helpers`, and `semantics-v2-embed-candidates`.
Standard constraints apply.

## Decisions already made (`docs/decisions/semantics-v3.md` §0, §§1–4)

- **Lean-first authority:** `crates/deckmaste_semantics_v3` mirrors the proven
  `lean/Semantics/` v3 types (`Subject`, `AnyValue` / `any`, `NounPhrase.which`,
  `instruct`, `does`, `would` / `instead`, `cant`/`must`/`may`/`gatedBy` +
  `DeonticMods`, `did` / `tally`, and the 10-constructor `GameEvent`).
- **Port the Clean-Sheet DRT Checker (`reads.rs`):**
  - `crates/deckmaste_semantics_v3/src/reads.rs` mirrors `lean/Semantics/Check/`'s
    `DRS`, `ReferentId`, `Referent`, `GramRole`, nested DRS boxes,
    `enterOwnScope` / `inCaller` / `leaveOwnScope`, `firstMention` /
    `laterMention` (`List ReferentId`), `StepPhase.preEvent` vs `.postEvent`,
    and the `J1`, `J2`, `D′`, and `AC>R1` resolution gates.
- **One language: Genuine Lean 4 `{ field := val, ... }` brace-and-comma declarations:**
  - Every declaration lives as a loose file in `plugins_v3/` (`plugins_v3/builtin/`
    for definitions, rules, registries, and templates; `plugins_v3/canon/` for printed cards).
  - Command macros: `action`, `keyword`, `ability_word`, `counter`, `designation`,
    `subtype`, `card_type`, `relation`, `token`, `sba`, `conferral`, `template`, `card`.
  - Every declaration uses `{ field := val, ... }` brace-and-comma blocks — 100% valid
    Lean 4 checked by `lake build`, and zero-significant-whitespace so the Rust parser
    in `deckmaste_semantics_v3` is a ~250-line balanced-delimiter reader.
  - Self-contained declarations unify Layer 2 lexical catalog fields (`lemma`, `plural`,
    `verbs`, `nouns`, `adjectives`, `determinatives`, `bound` matching
    `crates/deckmaste_lexical_source/src/schema.rs`), Layer 3 `spelling := "..."` string
    templates (the foundation for the CxG `deckmaste_spelling` crate), and `semantics_v3`
    meaning (`body`, `confers`, `rules`) in a single declaration block (retiring separate
    `Counters.lean` + `Proofs/Rules.lean`, `Subtypes.lean` + `SubtypeRules.lean`, etc.).
  - No `rule:` data field — CR citations live in `--` comments (e.g. `-- [CR#701.47a]`) checked by
    `cargo xtask cite`.
  - No separate `parameters:` field on `keyword` — derived from the declaration's typed
    parameter signature and `spelling`.
- **Loose-bundle DAG loading and topological sorting:**
  - Handled by Kahn's algorithm with a lexicographically ordered ready queue (`BTreeSet`
    by declaration name), proven in `semantics-v3-peer-files-and-toposort`.
- **`xtask lean` gates on `v3` and `v2` retirement:**
  - Delete `crates/deckmaste_semantics_v2` and `plugins_v2/canon/` as part of
    this cutover (the pre-v4 `deckmaste_lexical_source` was deleted in
    `english-v4-cutover-v3`, and the MTG lexical catalogs move into
    `plugins_v3/` metadata in `english-v4-final-suffix-cutover`).
  - Delete `plugins_v2/builtin/` and its reader `crates/deckmaste_macro_def`
    once `plugins_v3/` replaces their xtask uses (`expansions.rs` and
    `tests/suite/plugins_v2_declarations.rs`).

## Folded Residues from Retired `v2` Surface Tickets

1. **From `lean-macros-cutover`:**
   - The 31 `"body, loader device"` drifts (`ownScope` / `inCaller` /
     `firstMention` / `laterMention`) and the 9 pronoun-window macros are
     resolved structurally by the `Definition` vs. `Template` split above.
   - Align the 15 signature drifts (`amass`, `modular`, `bushido`, `renown`,
     `fateseal` taking `Amount`/`Subtype`; `move`, `create`, `createTappedToken`,
     `creatureToken[Of]`, `returnToBattlefield`, `establishFor`, `forEach`,
     `companion`, `Actor.mayCastFrom` binder order) and 2 body drifts
     (`gainControl`, `levelUp`).
   - Port or re-spell the 8 `"It computes"` helpers (`chooseSpree`,
     `dealDamageOwnPower`, `joinedHead`, `joinedHeadWhile`, `loseCounters`,
     `partyOf`, `rollRow`, `scaledMana`) and the 6 generator-underivable helpers
     (`activated`, `rollsDice`, `copy`, `applyResultsTable`, `chooseModes`,
     `triggeredJoined`); retire `lookAndSort`, `lookAndSortInto`, and
     `lookedCards` in favor of `lookAndSortSlice`.
2. **From `plugins-v2-bodies-through-helpers`:**
   - Write `Definition` bodies in `plugins_v3/canon/definitions/` through
     `Template` helpers from `plugins_v3/builtin/templates/` wherever one
     expands to the exact sub-term written (leaving raw constructors only where
     no helper exists).
   - Elide `Printed(Param(kind))` in `entersWithCounters`,
     `entersWithAdditionalCounters`, `entersWithFewerCounters`, and
     `exileWithCounters`.
3. **From `semantics-v2-embed-candidates`:**
   - In the 120-card corpus census (1,200 one-field constructor applications),
     84% (1,003) are already covered by the five marked injections (`ManaSymbol::Simple`,
     `Amount::Lit`, `ColorOrColorless::Of`, `SimpleManaSymbol::Specific`,
     `SimpleManaSymbol::Generic`), `Card::SingleFaced` is 119, `NounPhrase::Target`
     is 23, `Subtype::Type` is 18, and `ColorTerm::Lit` has 0 canon reaches.
     Settle these directly in the `v3` DSL / injection rules.

## Proof

- `crates/deckmaste_semantics_v2` and `plugins_v2/canon/` are deleted from the
  workspace.
- `plugins_v2/builtin/` and `crates/deckmaste_macro_def` are deleted, with xtask
  `expansions.rs` and `plugins_v2_declarations.rs` reading `plugins_v3/` instead.
- `cargo test -p deckmaste_semantics_v3 -p xtask` and
  `cargo clippy -p deckmaste_semantics_v3 -p xtask --tests -- -D warnings` pass.
- `cargo xtask lean check`, `cargo xtask lean definitions`, and
  `cargo xtask lean macros align --deny-drift` pass 100% on `plugins_v3/builtin`
  and `plugins_v3/canon`.
