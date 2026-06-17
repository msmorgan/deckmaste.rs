---
needs: []
---
# `Normalize` trait + faithful cost read

**Goal:** Introduce one principled, grammar-wide operation for collapsing
redundant *structural* nesting in grammar values — a `Normalize` trait, sibling
to `Expand` — and make RON read **faithful** so a macro-spliced cost survives
verbatim (lumpy) rather than being silently flattened at read time.
Normalization becomes an explicit post-read step the consumers invoke at the
boundary.

Two kinds of value "lumpiness" exist: macro provenance (`Expanded` nodes,
already shed by `Expand::expand_all`) and redundant structural nesting (a
`CostComponent::Cost([…])` wrapper spliced into a cost list, nested
`Filter::AllOf([AllOf([…])])`, …). `Normalize` is the intrinsic operation for
the second kind. A fully-canonical value is `x.expand_all().normalize()`.

This is a "fix convenient-but-wrong" change: the old read-time `Cost` flatten
was convenient but made read unfaithful, so it is removed rather than
grandfathered.

## The trait (`crates/macro_ron/src/traverse.rs`, next to `Expand`)

```rust
pub trait Normalize: Sized {
    #[must_use]
    fn normalize(self) -> Self;
}
```

- Consuming `self -> Self` (mirrors `Expand`): rebuilds the tree, no clone.
- **Hand-written per type, NEVER derived** — equivalence needs human judgment.
- Blanket container/leaf impls mirror the `Expand` ones: leaves (`u32`,
  `String`, `Ident`, `bool`, …) = identity; `Box`/`Option`/`Vec`/`HashMap`/
  `BTreeMap`/2- and 3-tuples = recurse. These are helpers a hand impl calls to
  recurse into children, not a derivation.
- Re-exported `macro_ron::Normalize` → `deckmaste_core::Normalize`.

## Hand impls (bottom-up: normalize children, then collapse locally)

1. **`Cost`** (`crates/deckmaste_core/src/cost.rs`): splice every
   `CostComponent::Cost(inner)` one level into the surrounding list
   (associativity of cost concatenation, [CR#601.2b]). `CostComponent` has its
   own impl that recurses into a nested `Cost`, so one `Cost::normalize` pass
   flattens arbitrary depth and is idempotent. Generalizes the removed
   read-time flatten (cycling, [CR#702.29a]).
2. **`Filter`** (`crates/deckmaste_core/src/filter.rs`): recurse into child
   filters (the `Box<Filter>` carried by `Not`, the `RelationFilter`
   compartments, and `StateFilter::RelatedBy`/`Targets`), then for
   `AllOf`/`OneOf`: flatten a nested same-combinator child (associativity) and
   collapse a singleton `AllOf([x])`/`OneOf([x]) → x`. `Where`'s inner
   `Condition` is left as authored (no `Condition` normalization in this pass).
   Out of scope: `Not(Not x)`, `Any`-absorption, dedup/sort, cross-combinator
   merges.

## Faithful read — remove the `Cost` deserialize flatten

- `Cost`'s custom flattening `Deserialize` is replaced with
  `#[derive(Deserialize)] #[serde(transparent)]` (mirrors its transparent
  `Serialize`). A nested `CostComponent::Cost` now SURVIVES read verbatim.
- The invariant "a nested `Cost` never survives into stored data" is
  intentionally **retired**; the `Cost`/`CostComponent::Cost` docs say so.

## Consumer wiring (the blast radius)

Removing read-flatten means macro-spliced (e.g. cycling) costs reach consumers
lumpy. Each cost consumer now operates on a normalized cost at its boundary:

- **`crates/deckmaste_engine/src/resolve.rs`** (`Effect::Unless` arm): the
  authored `unless: Vec<CostComponent>` is normalized
  (`Cost(u.unless).normalize().0`) before it enters the `Unless` continuation,
  so the payment walk (`decide.rs::unless_cost_action`) never meets a nested
  `Cost` — its `unreachable!("nested Cost …")` arm is now guarded by normalize,
  not by deserialize.
- **`crates/deckmaste_engine/src/activate.rs`** (`cost_summary`): already
  recursed through `CostComponent::Cost`; that recursion is the pay path's
  normalization inlined into the summarizing walk (no clone). Comments updated
  to say lumpy costs now legitimately arrive and the recursion is load-bearing.
- **`crates/deckmaste_cards/src/validate.rs`** (cost-eligibility lint): the
  activated cost is `normalize`d before the lint loop, so a verb spliced in via
  a nested `Cost` is still validated (previously the lint silently skipped
  `CostComponent::Cost`).

(The cast path summarizes through `cost_summary`; `concretize`/
`phyrexian_life_verbs`/`render_cost` operate on engine-generated or
macro-argument values that have no nested `Cost`, so they are unaffected.)

## Tests (TDD — `Normalize` has real behavior, so it earns tests)

- `cost.rs`: lumpy nested `Cost` survives read, then `.normalize()` collapses
  it; deep nesting flattens one-pass + idempotent.
- `filter.rs`: `AllOf`/`OneOf` associativity, singleton collapse, nested
  singletons, combinator under a compartment, no cross-combinator merge,
  idempotent.
- `activate.rs`: a cycling-shaped lumpy cost is summarized correctly by the pay
  path ({2} mana + discard-self verb), identically to its normalized form —
  cycling pays at the level the engine supports (from-hand activation of the
  conferred ability is a separate, unbuilt seam, so the end-to-end assertion
  lands at the cost-payment boundary `cost_summary`).
- `keywords.rs`: the cycling expansion's cost reads lumpy and normalizes flat.

## Out of scope (deferred)

- The raw + normalized cache on cards/abilities (faithful raw for rendering,
  derived normalized for the engine) — the north star, not now.
- `Normalize` impls beyond `Cost` + `Filter`; Effect/Ability normalization.
- `Filter` identities beyond AllOf/OneOf associativity + singleton-collapse.

## Verification

`cargo test --workspace`; `cargo clippy --workspace --all-targets` clean;
`cargo +nightly fmt` (touched files); canon 0-mismatch via
`cargo xtask validate` and `cargo xtask validate plugins/wizards`. No RON
read/write semantics changed for canon-authored cards (the change affects only
*macro-spliced* costs, of which canon has none today), so generated wizards are
unaffected; regenerate + re-check canon to confirm.
