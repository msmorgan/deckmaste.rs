---
needs: []
---

# English v4 Parser & Realization Hot-Path Optimization

## Goal

After `english-v4-intern-declaration-identities` (`tzmzuyuo`) and `english-v4-zero-alloc-chart-state` (`kxvzopnk`), `cargo xtask english-v4 strata` across all 92,934 Corpus Units takes ~129s wall / ~2,163s CPU in a debug build (~48s CPU for the Judgment suite). Remaining CPU time is concentrated in four hot-path bottlenecks across `deckmaste_syntax` and `deckmaste_lexical`:

1. **Unfiltered non-terminal Earley prediction and per-production terminal re-scanning (`crates/deckmaste_syntax/src/chart.rs`, `compile.rs`)**
2. **Oversized `State`/`Summary` keys and `SipHash` table lookups in the Chart (`crates/deckmaste_syntax/src/step.rs`, `chart.rs`)**
3. **Per-node heap `BTreeSet` and `String` allocations in `verify_realization` (`crates/deckmaste_syntax/src/laws.rs`)**
4. **Pointer-chasing `BTreeMap` lookups in lexical analysis (`crates/deckmaste_lexical/src/analysis.rs`)**

This ticket eliminates those bottlenecks while preserving identical parse forests, Admissions, Readings, and round-trip verification results.

---

## Deliverable

### 1. First-Set / Left-Corner Prediction Filtering and Flat Chart Indexing (`crates/deckmaste_syntax/src/compile.rs`, `chart.rs`)

- **Static left-corner terminal mask at bind time (`compile.rs`):**
  - In `chart.rs:326-344`, `CompiledGrammar::viable` currently returns `true` for every Production whose first right-hand-side symbol is `Symbol::Cat { .. }`, causing Earley prediction to expand full non-terminal chains (`Document -> Ability -> Sentences -> Sentence -> Clause -> SubjectPredicate -> NounPhrase -> ...`) at every token offset even when no leaf at `skip(position)` can start the category's left corner.
  - Even for terminal-initial Productions (`Symbol::Leaf` / `Symbol::Punct`), `viable` re-scans `by_start[skip(position)]` and re-evaluates `leaf_matches` once per candidate Production.
  - At `CompiledGrammar::bind` time, compute the transitive left-corner terminal set for every Category and Production (e.g., bitmask of starting `LexicalCategory`, `Numeral` notation, punctuation char, or closed-word/class eligibility).
  - In `Chart::run` (`chart.rs:198-262`), compute the active terminal signature at `skip(position)` once per token position and filter both category prediction (`by_lhs[category]`) and individual Productions by bitwise intersection before calling `Chart::add`.
- **Replace `predicted` and `waiting` hash tables (`chart.rs:195-196, 232, 274-277, 290-293`):**
  - Replace `predicted: HashSet<(usize, usize)>` (which hashes `(position, production)` on every prediction check even though `position` increases monotonically `0..=n`) with a flat `predicted_at: Vec<u32>` stamp array of length `self.productions.len()` storing `position + 1` (or a per-category `predicted_cat_at: Vec<u32>` when filtering at the category level).
  - Replace `waiting: HashMap<(usize, usize), Vec<usize>>` with a flat table indexed by `(position, category)` (`position * self.categories.len() + category`), and iterate by index during completion (`chart.rs:290-295`) instead of `.get(&(origin, node.category)).cloned().unwrap_or_default()`, eliminating a `Vec<usize>` heap clone on every completed Node.

### 2. Compact `State` / `Summary` Representation and Position-Scoped Item Deduplication (`crates/deckmaste_syntax/src/step.rs`, `chart.rs`)

- **Shrink `State` and `Summary` (`step.rs:38-127`):**
  - `State::vars` (`[Option<u16>; 16]`, 64 bytes) and `Summary::values` (`[Option<u16>; 12]`, 48 bytes) use 4 bytes per slot even though every feature domain has `< 32` values (`compile.rs:230-237`).
  - Represent feature values as `u8` with `u8::MAX` as the unset sentinel (or pack `Summary::values` into a `u64` / `u128`), shrinking `State::vars` from 64B to 16B and `Summary::values` from 48B to 12B (or 8B) and reducing `ItemKey` / `NodeKey` size by >50%.
- **Position-local item deduplication and fast hashing (`chart.rs:150-169, 193-194, 303-321`):**
  - `Chart::add` only ever deduplicates items sharing the current `end` position (`agenda` at `position` in prediction/completion, or `position..end` in scanning), or partition `item_index` by `end` position and use a fast fixed-size integer hash (fxhash / ahash style mixer) instead of `SipHash-1-3` over 100+ byte keys.

### 3. Bitmask `EdgeMemo` and Pre-Verified Leaves in `verify_realization` (`crates/deckmaste_syntax/src/laws.rs`)

- **Replace `BTreeSet<Edges>` with a 4-bit mask (`laws.rs:27-39, 98-174`):**
  - `type Edges = (bool, bool)` has only 4 possible values (`(false, false)`, `(false, true)`, `(true, false)`, `(true, true)`).
  - Replace `BTreeSet<Edges>` in `EdgeMemo`, `node_edges`, and `item_edges` with a `Copy` `u8` 4-bit bitmask (`EdgeMask(u8)`), eliminating all heap allocations and `BTreeSet` clones during packed-forest realization verification.
- **Verify leaves once per forest (`laws.rs:176-187`):**
  - `PackedForest::leaf_binding` currently allocates a `String` (`self.tokens[leaf.start..leaf.end].iter().collect()`) and calls `lexicon.realize` every time a leaf is reached during `item_edges`.
  - Precompute a `Vec<Option<Binding>>` of length `self.forest.leaves.len()` once at the start of `verify_realization` (and avoid allocating a `String` when a single-token slice already borrows `&str`).

### 4. Contiguous Transition Lookup in Lexical Analysis (`crates/deckmaste_lexical/src/analysis.rs`)

- In `crates/deckmaste_lexical/src/analysis.rs:21-26`, `TrieNode::children` is a `BTreeMap<char, usize>` probed on every character at every token start offset. Replace `BTreeMap<char, usize>` with a flat sorted `SmallVec<[(char, u32); 4]>` / `Box<[(char, u32)]>` (or ASCII lookup table for the root node) to keep character transitions contiguous in cache.

---

## Verification

Standard constraints apply.

1. `cargo test -p deckmaste_lexical -p deckmaste_syntax -p deckmaste_english_v4`
2. `cargo clippy -p deckmaste_lexical -p deckmaste_syntax -p deckmaste_english_v4 --tests -- -D warnings`
3. Run `cargo xtask english-v4 strata --check` and `cargo test -p deckmaste_english_v4 --test judgments` before and after, reporting wall time and CPU time deltas and confirming zero Census delta and zero Judgment regressions.
