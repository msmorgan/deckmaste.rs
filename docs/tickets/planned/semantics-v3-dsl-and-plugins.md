---
needs: [lean-v3-proofs-and-cards, lean-v3-authoring-surface, lean-v3-checker-and-drt-fixes, lean-v3-rules-and-card-fidelity, lean-v3-keyword-bodies-and-catalog-sync, lean-v3-proofs-and-docs-alignment]
---
# Create deckmaste_semantics_v3 and plugins_v3 mirroring the proven Lean v3 specification

**Build `crates/deckmaste_semantics_v3` and `plugins_v3/` from the proven
`lean/Semantics/` v3 specification, with explicit `Definition(...)` and
`Template(...)` file wrappers and the Unified Action-Clause Spine.** Fifth
implementation step of [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)
(§0, §§1–4). Supersedes the `v2`-specific cleanup tickets `lean-macros-cutover`,
`plugins-v2-bodies-through-helpers`, and `semantics-v2-embed-candidates` (with
the Lean-side `Templates.lean` / `Definitions.lean` consolidation completed in
[`lean-v3-authoring-impl-2-templates-and-definitions`](lean-v3-authoring-impl-2-templates-and-definitions.md)).
Standard constraints apply.

## Decisions already made (`docs/decisions/semantics-v3.md` §0, §§1–3)

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
- **`Definition(...)` vs. `Template(...)` DSL files in `plugins_v3/`:**
  - Every file in `plugins_v3/` declares its role via its top-level wrapper
    (`Definition(...)` vs `Template(...)`), never by directory path:
    - `Definition(...)` wraps all CR §701 keyword actions (atomic and
      composite), CR §702 keyword abilities, and registry definitions
      (`CounterKind`, `Subtype`, `Designation`, `Relation`, `AbilityWord`,
      `FlavorWord`, `TurnPart`, `Type`), expanding with `ownScope` + `inCaller` +
      parameter identity (`firstMention`/`laterMention`).
    - `Template(...)` wraps all reusable Oracle phrasing helpers and constructor
      aliases, expanding transparently in the caller's DRS scope while emitting
      `firstMention`/`laterMention` whenever a `NounPhrase` parameter is
      mentioned $>1$ time before a zone change.
  - Evaluate whether a small purpose-built `v3` DSL parser (`deckmaste_dsl_v3`)
    replaces `macro_ron` so `Definition(...)`, `Template(...)`, polymorphic
    `any`, `which`, bare numerals, and positional/named arguments parse and
    format directly without RON's one-embed-per-enum and struct/vec injection
    restrictions.
- **`xtask lean` gates on `v3` and `v2` retirement:**
  - Point `cargo xtask lean check`, `cargo xtask lean definitions`,
    `cargo xtask lean macros`, and `cargo xtask facts check` at
    `crates/deckmaste_semantics_v3` and `plugins_v3/` with `--deny-drift`
    enforced.
  - Delete `crates/deckmaste_semantics_v2` and `plugins_v2/canon/` as part of
    this ticket (`deckmaste_lexical_source` is deleted in
    `english-v4-cutover-v3`, and the MTG lexical catalogs move into
    `plugins_v3/` metadata in `english-v4-final-suffix-cutover`).
  - Delete `plugins_v2/builtin/` once `plugins_v3/` replaces its readers:
    `deckmaste_macro_def` (`src/lib.rs` and the `builtin_v2_*` tests), xtask
    `expansions.rs`, and `tests/suite/plugins_v2_declarations.rs`.

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
- `plugins_v2/builtin/` is deleted, with `deckmaste_macro_def`, xtask
  `expansions.rs`, and `plugins_v2_declarations.rs` reading `plugins_v3/` instead.
- `cargo test -p deckmaste_semantics_v3 -p xtask` and
  `cargo clippy -p deckmaste_semantics_v3 -p xtask --tests -- -D warnings` pass.
- `cargo xtask lean check`, `cargo xtask lean definitions`, and
  `cargo xtask lean macros align --deny-drift` pass 100% on `plugins_v3/builtin`
  and `plugins_v3/canon`.
