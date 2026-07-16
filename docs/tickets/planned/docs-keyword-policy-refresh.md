---
needs: []
---
`docs/keyword-policy.md` has drifted from the code it describes:

- §6 documents a plugin-load enforcement step (`keyword_shape` matching each
  `KeywordAbility` macro's param-type multiset against seven signatures, load
  error on mismatch, a `keywords: HashMap` registry on the plugin) that no
  longer exists in `crates/deckmaste_core/src/plugin.rs`. The `ParamShape` /
  `KeywordDecl` types survive as orphans — re-exported from `lib.rs`,
  exercised by one round-trip test, read by nothing else — and §6's "only
  reads are one test and a clone-forward" line is stale too (no clone-forward
  remains). Decide: reinstate the load-time shape check, or delete the orphan
  types and rewrite §6 to describe current reality.
- §2's three-class table sums to 244 under a "keyword abilities" heading; it
  only reconciles with rules-taxonomy's 192 abilities + 68 actions = 260 if
  the "Composite: 215" row silently spans abilities *and* actions. Label the
  rows so the counts read consistently.
