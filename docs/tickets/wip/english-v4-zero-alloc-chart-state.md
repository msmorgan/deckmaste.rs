---
needs: [english-v4-intern-declaration-identities]
---
# Make Earley chart `State` and `Summary` `Copy` with zero heap allocations in `deckmaste_syntax`

Inside the Earley chart (`crates/deckmaste_syntax/src/chart.rs` and
`crates/deckmaste_syntax/src/step.rs`), every prediction, scan, and completion
currently heap-allocates and clones multiple vectors and boxed slices inside
`State` and `Summary`:

- `State::vars: Box<[Option<u16>]>` and `Summary::values: Box<[Option<u16>]>`
  are heap-allocated on every `Production::start()`, `Production::accept()`, and
  `Production::finish()`.
- `State::extensions: Vec<Extension>`, `State::host_extensions: Vec<Extension>`,
  and `Summary::extensions: Vec<Extension>` clone a `Vec<Extension>` on every
  chart step, even though an extension list is an immutable table entry from
  `CompiledGrammar::extensions` that is never mutated or combined after lookup
  (only copied through `head` or reset to empty on coordination).
- `State::conjunct_coordinators: Vec<LexemeId>` heap-allocates a `Vec` even
  though `distinct_coordinator` inspects at most two conjunct children per
  production.

## Actionable changes

1. **Intern extension lists as a `u16` index in `CompiledGrammar`
   (`compile.rs`, `step.rs`):**
   - Store deduplicated extension slices in `CompiledGrammar::extension_sets:
     Vec<Vec<Extension>>` (with index `0` reserved for the empty slice) and map
     `extensions: BTreeMap<(LexemeId, Option<usize>), u16>`.
   - Replace `Summary::extensions`, `State::extensions`, and
     `State::host_extensions` with `u16` indices (`0` when empty).
2. **Replace `State::conjunct_coordinators: Vec<LexemeId>` with a fixed inline
    representation (`step.rs`):**
   - `distinct_coordinator` marks conjunct children of a coordination production
     (at most 2 children). Store `conjunct_coordinators: [Option<LexemeId>; 2]`
     (or check coordinator distinctness directly against `state.coordinator` /
     child summaries during `accept`) with a bind-time assertion in `resolve`
     that `distinct_coordinator.len() <= 2`.
3. **Replace `Summary::values` and `State::vars` boxed slices with fixed-size
   inline arrays (`step.rs`, `compile.rs`):**
   - Define `MAX_CATEGORY_FEATURES` (e.g. `12`) and `MAX_PRODUCTION_VARS` (e.g.
     `16`) in `step.rs`, validated at grammar bind time in `compile.rs` /
     `resolve`.
   - Store `Summary::values: [Option<u16>; MAX_CATEGORY_FEATURES]` and
     `State::vars: [Option<u16>; MAX_PRODUCTION_VARS]` (and precompute
     `Production::start_state: State` in `resolve` so `Production::start(&self)`
     is a trivial copy).
   - Derive `Copy` on both `Summary` and `State`.
4. **Pre-index leaf facts in `CompiledGrammar::leaf_facts` (`step.rs`,
   `chart.rs`):**
   - In `chart.rs`, `Leaf` stores `LeafFacts` whose `features: Vec<(usize,
     u16)>` is linearly scanned in `accept`. Replace `LeafFacts::features` with
     `[Option<u16>; MAX_FEATURES]` or a fixed inline array indexed by feature ID
     (or compact `(u8, u16)` inline storage) so leaf feature lookup in `accept`
     is $O(1)$ without heap allocation.

## Gate

- `State` and `Summary` implement `Copy` (zero heap allocations during
  `Production::start`, `Production::accept`, and `Production::finish`).
- Zero behavioral or coverage changes across all 92,934 corpus units (`cargo
  xtask english-v4 strata --check`) and all judgment suites (`cargo test -p
  deckmaste_english_v4`).
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for
  `deckmaste_syntax` and `deckmaste_english_v4`.

Standard constraints apply.
