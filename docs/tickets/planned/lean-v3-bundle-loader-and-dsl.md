---
needs: [lean-v3-proofs-and-cards, lean-v3-authoring-surface, lean-v3-checker-and-drt-fixes, lean-v3-rules-and-card-fidelity, lean-v3-keyword-bodies-and-catalog-sync, lean-v3-proofs-and-docs-alignment, lean-v3-template-twins, lean-v3-exchange-through-definitions, lean-v3-authoring-default-leaks, lean-v3-registry-definition-rename, lean-v3-definition-parameter-identities, lean-v3-deontic-head-gate, lean-v3-control-grant-caused-event, lean-v3-text-word-sort, lean-v3-departure-observation-battlefield-only, lean-v3-partitive-rest-reads-its-kind, lean-v3-bare-pronoun-consuming-verb, lean-v3-shuffle-keeps-last-known, lean-v3-deed-duplicates, lean-v3-checker-fuel-and-flags, lean-v3-checker-runtime-state-audit, lean-v3-doif-otherwise-reads-condition-mentions, lean-v3-compare-over-bound-reads-member]
---
# Land `load_bundle` DAG loader, `{ field := val, ... }` command macros, and authoring coercions in Lean

**Add the loose-bundle DAG loader (`load_bundle`), the Genuine Lean 4 `{ field := val, ... }`
declaration command macros, auto-wrapping, and the four authoring coercions / top-down
elaborators to `lean/Semantics/Authoring.lean`.** First of four atomic tickets migrating
`lean/Semantics/Templates.lean` and `lean/Semantics/Definitions/*.lean` into `plugins_v3/builtin/`
([`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §§4.4–4.5). Reference spike
preserved on bookmark `archive-semantics-v3-peer-files-and-toposort`. Standard constraints apply.

## Scope

1. **Command macros (`{ field := val, ... }` brace-and-comma blocks):**
   - `primitive`, `template`, `action`, `keyword`, `ability_word`, `counter`, `designation`,
     `subtype`, `card_type`, `relation`, `token`, `sba`, `conferral`, `card`.
   - Each declaration unifies Layer 2 lexical catalog fields (`lemma`, `plural`, `verbs`,
     `nouns`, `adjectives`, `determinatives`, `bound` matching
     `crates/deckmaste_lexical_source/src/schema.rs`), Layer 3 `spelling := "..."` string
     templates, and `semantics_v3` meaning (`body`, `confers`, `rules`) in one block.
   - No `rule:` data field — CR citations live in `--` comments (e.g. `-- [CR#701.47a]`) checked by
     `cargo xtask cite`.
   - No separate `parameters:` field on `keyword` — derived from the declaration's typed parameter
     signature and `spelling`.
   - `counter`, `subtype`, `designation`, `card_type`, `relation`, `sba`, and `conferral`
     automatically elaborate both their `semantic_definition` constant and their
     `RegistryDefinition` / rule row plus a kernel-checked `decide` proof (`ok_<name>`).
   - `card` automatically elaborates `spelled <| ...` and a kernel-checked `Card.check` proof
     (`ok_<name>`).

2. **Auto-wrapping:**
   - **Auto-`inCaller`:** Every parameter whose type implements `CallerScope` (`Amount`, `Cost`,
     `Predicate`, `Location`, `Quantity`, `Condition`, `Action`, `TokenSpec`, `MoveRider`, `Ballot`,
     plus `Option α` and `List α`) is automatically wrapped in `CallerScope.caller`.
   - **Auto-`firstMention` / `laterMention`:**
     1. `NounPhrase` parameters in `action` / `keyword` (wrapping `inCaller p`, with explicit
        `inCaller p` as opt-out),
     2. Multi-mentioned `NounPhrase` parameters ($\ge 2$ occurrences) in `template` (without
        `inCaller`, eliminating manual sites in `duringNextTurnOf`, `lookAndSortSlice`,
        `controllerSacrifices`), and
     3. Multi-mentioned `theActor` ($\ge 2$ occurrences) in `action` (eliminating manual
        `.firstMention` sites in `fight`, `explore`, `connive`, `endure`).
   - **Auto-`.ownScope` + `.deed` on `action`** and **auto-`Ability.keyword` + pre-registered
     `<name>.label` on `keyword`** (so self-references like `hasKeyword flying` inside `flying`
     `[CR#702.9b]` and `hasKeyword suspend` inside `suspend` `[CR#702.62a]` resolve directly
     without `.named` string literals).

3. **Four authoring coercions / top-down elaborators:**
   1. **Domain list literals (`[...]`):** `@[scoped term_elab Lean.Parser.Term.app] def elabDomainList`
      intercepting `[...]` after macro expansion (`List.cons`) while `expectedType?` is `Action`
      (`sequentially`), `Predicate` (`Predicate.and`), `Condition` (`Condition.and`), `StaticSpec`
      (`conjunction`), `Cost` (`mana` or `compound`), or `Option (List α)` (`some [...]`).
   2. **Colors & generic mana:** Bare color constants (`white`, `blue`, `black`, `red`, `green`),
      `Coe Color ManaSymbol := ⟨pip⟩`, `OfNat ManaSymbol n := ⟨generic n⟩`,
      `Coe Color ColorOrColorless := ⟨.of⟩`.
   3. **Bare nouns:** `creature`, `artifact`, `enchantment`, `land`, `planeswalker`, `battle`,
      `instant`, `sorcery`, `kindred`, `card`, `permanent`, `player`, `token`, `spell` dispatched
      top-down by expected type (`Predicate` vs `NounWord` vs `CardType`), plus `this` → `this_`,
      `Coe Characteristics CharacteristicBundle`, `Coe StaticSpec Ability := ⟨static⟩`.
   4. **Keyword names:** Direct references in `KeywordTerm` and `KeywordLabel` slots for both
      parameterless and parameterized keywords and self-references.

4. **Shared bare names across return types (`(name, return_type)` dispatch and DAG keying):**
   - Multiple declarations may share the same bare name when they build different return types
     (e.g. `and`, `or`, `andOr`, `not` across `Predicate`, `Condition`, `NounPhrase`, `Location`,
     `Timing`, `Cost`; `target`, `attachment`, `copy` across `NounPhrase`/`Predicate` and `Action`;
     `did`, `was` across `Predicate` and `Condition`; `doIf`, `replace` across `Action`,
     `Replacement`, and `StaticSpec`; `crew`, `saddle`, `devour` across `Action` and `Ability`;
     bare nouns `creature`, `artifact`, etc. across `Predicate`, `NounWord`, and `CardType`).
   - When a declaration's bare name `N` is shared across return types `T` (or for any sort-dispatched
     declaration), the command macro emits the underlying Lean constant under `<T>.<N>`
     (`Predicate.and`, `Condition.and`, `Action.devour`, `Ability.devour`) and automatically
     registers `(N, T) ↦ <T>.<N>` in a persistent environment extension consulted by
     `dispatchOnExpectedType` (for both applications `N x ...` and 0-arg identifiers `N`), retiring
     the hand-maintained `expectedTypeDispatch` and `elabKeywordDispatch` tables in `Templates.lean`
     and `Keywords.lean`.
   - **DAG keying in `load_bundle`:** Nodes in the dependency graph are keyed by
     `(bare_name, return_type)` (with deterministic tie-breaking by `(bare_name, return_type)`)
     rather than bare `name` alone. When a declaration `(B, T_B)` references bare identifier `X`:
     - If `X != B`, `(B, T_B)` depends on all declarations with bare name `X`.
     - If `X == B` (e.g. `keyword devour` (`Ability`) calling `action devour` (`Action`), vs. a
       true self-reference like `hasKeyword flying` inside `keyword flying`), an `Ability`
       declaration referencing `B` in an `Action` slot depends on `(B, Action)` (while
       self-references to `<B>.label` resolve to the pre-registered `KeywordLabel` without adding a
       self-loop).

5. **Loose-bundle DAG loader (`load_bundle "<relpath>"`):**
   - Walks all `.lean` files under `<relpath>`, parses commands without requiring `import`,
     `namespace`, or `open` headers, extracts `((decl_name, return_type), referenced_idents)`, and
     topologically sorts individual declarations via Kahn's algorithm with a lexicographically
     ordered ready queue (`BTreeSet` / sorted list by `(decl_name, return_type)`).
   - Reports a clear error naming the cycle declarations and file paths if a dependency cycle exists.

## Proof

- Unit and `decide` tests in `lean/Semantics/Proofs/Authoring.lean` cover:
  - each command macro (`primitive`, `template`, `action`, `keyword`, `counter`, `subtype`, `card`),
  - shared bare names across return types (e.g. `and` on `Predicate` and `Condition`, `devour` on
    `Action` and `Ability`, bare `creature` on `Predicate`, `NounWord`, and `CardType`),
  - auto-`inCaller`, auto-`firstMention`/`laterMention` on multi-mentioned template `NounPhrase`
    parameters and `theActor`, and keyword self-reference,
  - the 4 authoring coercions, and
  - `topoSortBundle` cycle detection and deterministic ordering.
- `cargo xtask lean check` exits 0 with zero errors and zero warnings.
