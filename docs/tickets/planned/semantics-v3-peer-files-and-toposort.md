---
needs: [lean-v3-proofs-and-cards, lean-v3-authoring-surface, lean-v3-checker-and-drt-fixes, lean-v3-rules-and-card-fidelity, lean-v3-keyword-bodies-and-catalog-sync, lean-v3-proofs-and-docs-alignment, lean-v3-template-twins, lean-v3-exchange-through-definitions, lean-v3-authoring-default-leaks, lean-v3-registry-definition-rename, lean-v3-definition-parameter-identities, lean-v3-deontic-head-gate, lean-v3-control-grant-caused-event, lean-v3-text-word-sort, lean-v3-departure-observation-battlefield-only, lean-v3-partitive-rest-reads-its-kind, lean-v3-bare-pronoun-consuming-verb, lean-v3-shuffle-keeps-last-known, lean-v3-deed-duplicates, lean-v3-checker-fuel-and-flags, lean-v3-checker-runtime-state-audit, lean-v3-doif-otherwise-reads-condition-mentions, lean-v3-compare-over-bound-reads-member]
---
# Peer loose files for Templates and Definitions with topological sort into Lean

**Author every `Template` and `Definition` as a loose peer file in `plugins_v3/`
and topologically sort their declaration dependency graph to emit them in
proving order for Lean (`cargo xtask lean check`).** Standard constraints apply.

## Problem

In the hand-layered Lean workbench (`lean/Semantics/`), authored declarations
were partitioned across a fixed module import chain:

`Templates.lean` → `Definitions/Phrasings.lean` → `Definitions/Actions.lean` → `Definitions/Keywords.lean` → `Definitions/CardTypeRules.lean`

Because Lean requires every referenced symbol to be declared earlier in the
import graph (or earlier in the same file), this fixed module boundary got in
the way of ergonomic authoring:

1. **Templates split across four modules by what they call:** A `Template` that
   composes over a CR §701 `Definition` ([CR#701.1]) or CR §702 `Definition`
   ([CR#702.1]) could not live in `Templates.lean`. That forced a separate
   `Definitions/Phrasings.lean` module plus `/-! ## Templates over Definitions -/`
   sections inside `Definitions/Actions.lean` (`destroyCantRegenerate`,
   `exileUntilLeaves`, `mayCastExiled`, `controllerSacrifices`, `doesntUntap`,
   `mayDeclineUntap`, `putIntoLibraryNthFromTop`, `takeExtraTurnAfterThisOne`)
   and `Definitions/Keywords.lean` (`castWithSplice`, `hasProtectionFromEach`,
   `typedCycling`).
2. **Action Definitions unable to reference Keyword Definitions:** Because
   `Definitions/Actions.lean` was imported by `Definitions/Keywords.lean`, a
   keyword action whose CR rule references a keyword ability could not refer to
   that keyword's `Definition` and had to drop down to a raw `.named` string:
   - `earthbend` (`Definitions/Actions.lean:649`, [CR#701.66a], *"Target land
     you control becomes a 0/0 creature with haste…"*) wrote
     `.addedAbilities [keyword (.named "haste")]` instead of referencing `haste`
     directly.
   - `timeTravel` (`Definitions/Actions.lean:750`, [CR#701.56a], *"each
     suspended card you own…"*) wrote `hasKeyword (.named "suspend")` instead
     of referencing `suspend` directly.
3. **Manual intra-file declaration ordering:** Authors had to manually order
   helper templates and definitions within each file before any caller that used
   them.

## Design

1. **Loose peer files in `plugins_v3/` (`plugins_v3/builtin/` and `plugins_v3/canon/`):**
   - Each declaration and card lives in its own individual `.lean` file, organized for human navigation by role and grammatical/domain category (replacing `plugins_v2/builtin/macros/`):
     - `plugins_v3/builtin/definitions/{keyword_action,keyword_ability,counter,designation,relation,token,card_type,subtype/{creature,artifact,enchantment,land,planeswalker,battle,spell}}/<name>.lean`
     - `plugins_v3/builtin/templates/{ability,ability_word,action,amount,condition,cost,determiner,duration,event,mana,noun,pile,predicate,pronoun,quantity,timing,zone}/<name>.lean`
     - `plugins_v3/builtin/rules/{sba,conferral,...}/<name>.lean`
     - `plugins_v3/canon/cards/<name>.lean` (one `.lean` file per card)
   - **Folder and file names are for human organization only, never semantically significant.**
     Loose bundle files have no `import`, `namespace`, or `open` headers; the loader never infers `Definition` vs. `Template` or grammatical category from the file path.
   - All files across the bundle directories are peers in one flat declaration
     namespace:
     - Any `template` may reference any `action`, `keyword`, `counter`, `subtype`, `designation`, `token`, or other `template`.
     - Any `action` or `keyword` may reference any other `keyword` (e.g. `earthbend` referencing
       `haste` directly, `timeTravel` referencing `suspend` directly), `action`,
       `counter`, `subtype`, `designation`, `token`, or `template`.
2. **Declaration-level dependency graph and deterministic topological sort:**
   - Both the Lean bundle loader (`load_bundle`) and the Rust loader (`deckmaste_semantics_v3`)
     walk all `.lean` files in a bundle directory, extract `(decl_name, referenced_idents)`, and
     topologically sort **individual declarations** via Kahn's algorithm with a lexicographically
     ordered ready queue (`BTreeSet` by declaration name).
   - An edge `A → B` exists whenever declaration `B` references declaration `A` (or a companion
     symbol registered by `A`, such as a keyword ability's `<name>.label` constant).
   - **Cycle refusal:** If the dependency graph contains a cycle, the loader refuses with a diagnostic
     naming the exact cycle of declarations and their file paths.
   - **Deterministic topological order:** The emitted Lean declaration order is completely deterministic
     and independent of filesystem traversal order.
3. **One language: Genuine Lean 4 with `{ field := val, ... }` brace-and-comma blocks:**
   - Command macros: `action`, `keyword`, `ability_word`, `counter`, `designation`, `subtype`,
     `card_type`, `relation`, `token`, `sba`, `conferral`, `template`, `card`.
   - Every declaration uses `{ field := val, ... }` brace-and-comma blocks — 100% valid Lean 4
     checked by `lake build`, with zero-significant-whitespace so the Rust parser in
     `deckmaste_semantics_v3` is a ~250-line balanced-delimiter reader.
   - Self-contained declarations unify Layer 2 lexical catalog fields (`lemma`, `plural`, `verbs`,
     `nouns`, `adjectives`, `determinatives`, `bound` matching
     `crates/deckmaste_lexical_source/src/schema.rs`), Layer 3 `spelling := "..."` string templates
     (foundation for `deckmaste_spelling`), and `semantics_v3` meaning (`body`, `confers`, `rules`)
     in a single declaration block (retiring separate `Counters.lean` + `Proofs/Rules.lean`,
     `Subtypes.lean` + `SubtypeRules.lean`, etc.).
   - No `rule:` data field — CR citations live in `--` comments (e.g. `-- [CR#701.47a]`) checked by `cargo xtask cite`.
   - No separate `parameters:` field on `keyword` — derived from the declaration's typed parameter
     signature and `spelling`.
4. **Auto-wrapping & four authoring coercions / top-down elaborators:**
   - **Auto-`inCaller`:** Every parameter whose type implements `CallerScope` (`Amount`, `Cost`,
     `Predicate`, `Location`, `Quantity`, `Condition`, `Action`, `TokenSpec`, `MoveRider`, `Ballot`,
     plus `Option α` and `List α`) is automatically wrapped in `CallerScope.caller`.
   - **Auto-`firstMention` / `laterMention`:** Generated automatically for:
     1. `NounPhrase` parameters in `action` / `keyword` (wrapping `inCaller p`, with explicit
        `inCaller p` as opt-out),
     2. Multi-mentioned `NounPhrase` parameters ($\ge 2$ occurrences) in `template` (without
        `inCaller`, eliminating manual sites in `duringNextTurnOf`, `lookAndSortSlice`,
        `controllerSacrifices`), and
     3. Multi-mentioned `theActor` ($\ge 2$ occurrences) in `action` (eliminating manual `.firstMention`
        sites in `fight`, `explore`, `connive`, `endure`).
   - **Auto-`.ownScope` + `.deed` on `action`** and **auto-`Ability.keyword` + pre-registered `<name>.label`
     on `keyword`** (so self-references like `hasKeyword flying` inside `flying` and `hasKeyword suspend`
     inside `suspend` resolve without `.named` strings).
   - **Four authoring coercions / top-down elaborators:**
     1. Domain list literals (`[...]`): top-down elaborator on expected type (`Action → sequentially`,
        `Predicate → Predicate.and`, `Condition → Condition.and`, `StaticSpec → conjunction`,
        `Cost → mana` or `compound`, `Option (List α) → some [...]`).
     2. Colors & generic mana: bare color templates (`white`, `blue`, `black`, `red`, `green`),
        `Coe Color ManaSymbol := ⟨pip⟩`, `OfNat ManaSymbol n := ⟨generic n⟩`,
        `Coe Color ColorOrColorless := ⟨.of⟩`.
     3. Bare nouns (`creature`, `artifact`, `enchantment`, `land`, `planeswalker`, `battle`, `instant`,
        `sorcery`, `kindred`, `card`, `permanent`, `player`, `token`, `spell`) dispatched top-down by
        expected type (`Predicate` vs `NounWord` vs `CardType`), plus `this` → `this_`,
        `Coe Characteristics CharacteristicBundle`, `Coe StaticSpec Ability := ⟨static⟩`.
     4. Keyword names in `KeywordTerm` and `KeywordLabel` slots (both parameterless and parameterized
        keywords and self-references).

## Proof

- Reference spike preserved on bookmark `archive-semantics-v3-peer-files-and-toposort` (`lean/Semantics/BundleSpike.lean` + sample loose files in `plugins_v3/builtin/` and `plugins_v3/canon/`).
- Every `Template` and `Definition` currently in `lean/Semantics/Templates.lean` and `lean/Semantics/Definitions/*.lean` lives as a loose file in `plugins_v3/builtin/` and loads via `load_bundle "../plugins_v3/builtin"`, proving cross-folder mutual dependencies (`earthbend` referencing `haste`, `timeTravel` referencing `suspend`, `flyingCounter` referencing `flying`), intra-file forward references, and Kahn's topological sort.
- Cycle detection is verified by unit test (`topoSortBundle` detects dependency cycles and returns an error naming the cycle declarations and file paths).
- `cargo xtask lean check` exits 0 with zero errors and zero warnings.
