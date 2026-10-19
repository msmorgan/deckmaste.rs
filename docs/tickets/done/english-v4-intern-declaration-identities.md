---
needs: [english-v4-stratum-5-residual]
---
# Intern declaration identities with `dpsi::Ident` across `deckmaste_syntax`, `deckmaste_lexical`, `deckmaste_lexical_source_v4`, and `deckmaste_english_v4`

`deckmaste_lexical_model` establishes the rule for the v4 lexical/syntax stack
(`crates/deckmaste_lexical_model/src/lib.rs:6-9`):

> Intern only declaration identities; arbitrary input and spellings stay owned.

Currently only `LexemeId` (`pub type LexemeId = dpsi::Ident`) follows that rule,
while every other grammar and lexical declaration identity is heap-allocated and
cloned as `String` — including on the hot path when constructing `ReadingNode`
and `Constituent` trees from the packed parse forest.

## Actionable changes

1. **Add `dpsi` dependency** to `deckmaste_lexical`, `deckmaste_syntax`,
   `deckmaste_lexical_source_v4`, and `deckmaste_english_v4`.
2. **`deckmaste_syntax` (`reading.rs`, `grammar.rs`, `compile.rs`, `step.rs`,
   `laws.rs`):**
   - Replace `String` with `dpsi::Ident` for all grammar declaration identities:
     - `ReadingNode`: `production: Ident`, `category: Ident`, `features:
       BTreeMap<Ident, Ident>`;
     - `Constituent`: `category: Ident`, `function: Ident` (or `Option<Ident>`
       at the root, avoiding empty-string sentinels);
     - `Grammar`, `CategoryDecl`, `ProductionDecl`, `SymbolDecl`, `SchemaDecl`,
       `Equation`: category names, production names, child role/function names,
       feature names, feature values, closed-word grammar names, lexical-class
       names, typed-class names, and core lexeme references (`LexemeId`);
     - `CompiledGrammar` and `Production` (`step.rs`): `categories: Vec<Ident>`,
       `category_ids: BTreeMap<Ident, usize>`, `function_categories:
       BTreeSet<Ident>`, `features: Vec<Ident>`, `values: Vec<Vec<Ident>>`,
       `by_id: BTreeMap<Ident, usize>`, `Production::id: Ident`, and
       `Production::names: Vec<Ident>`.
   - Because `dpsi::Ident` is `Copy` (`u64`) and implements `Serialize` /
     `Deserialize` transparently as a string, `Reading` and `Grammar` RON
     serialization formats remain unchanged while eliminating per-node string
     allocations and string-keyed lookups in `node_trees`, `summarize`,
     `locate`, `child_names`, and `atoms_from`.
3. **`deckmaste_lexical_model` and `deckmaste_lexical` (`lib.rs`, `model.rs`,
   `registry.rs`):**
   - Replace `String` with `dpsi::Ident` on declaration identities (leaving
     arbitrary input text and surface spellings as `String`):
     - `LexicalProperties`: `classes: BTreeSet<Ident>`, `features:
       BTreeMap<Ident, Ident>`;
     - `PropertyDomain`: `name: Ident`, `values: Vec<Ident>`;
     - `LexicalClass`: `name: Ident`;
     - `ClosedWord`: `name: Ident`;
     - `Frame`, `FrameSlot`, `FrameItem::Marker`, `FrameItem::Marked`:
       `kind: Ident`, `category: Ident`, `vocabulary: Ident`, `member: Ident`.
4. **`deckmaste_lexical_source_v4` and `deckmaste_english_v4` (`judgment.rs`,
   `profile.rs`, `census.rs`, `build.rs`):**
   - Update `Assertion` (`category: Ident`, etc.) and `Judgment` (`category:
     Option<Ident>`) in `judgment.rs` and all callers across
     `deckmaste_lexical_source_v4` and `deckmaste_english_v4` to use `Ident`
     directly instead of allocating and comparing `String`s.

## Gate

- Zero behavioral or coverage changes across all 92,934 corpus units (`cargo
  xtask english-v4 strata --check`) and all judgment suites (`cargo test -p
  deckmaste_english_v4`).
- `cargo test` and `cargo clippy --tests -- -D warnings` pass across
  `deckmaste_lexical_model`, `deckmaste_lexical`, `deckmaste_syntax`,
  `deckmaste_lexical_source_v4`, and `deckmaste_english_v4`.

Standard constraints apply.
