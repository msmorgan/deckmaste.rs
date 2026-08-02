---
needs: [filter-permanent-of-type-subtype]
---
**Memoize macro invocation/expansion during plugin load so the `Arc<Subtype>` /
`Arc<TypeDef>` filter payloads (from `filter-permanent-of-type-subtype`) are
actually SHARED across cards, not re-allocated per reference site.** Once the
filter predicate carries `Arc<Subtype>` (the resolved def), a naive expansion
allocates a fresh Arc + fresh `{name, types, confers}` struct at every
`Subtype(Vampire)` site — so the `Arc` is pointer indirection with no dedup
payoff. At corpus scale (~7k generated `wizards` cards, each naming several
subtypes) that is thousands of byte-identical heap structs.

## Fix

Intern macro expansions: expand each distinct macro invocation (keyed by
name + arguments) ONCE during load and hand the SAME `Arc` to every reference.
The expansion cache lives in the plugin-load / macro-expansion pipeline
(`crates/deckmaste_plugin/src/plugin.rs` load, `crates/macro_ron/src/expand.rs`).
Semantics are unchanged — identical values, shared storage — so this is a pure
memory/alloc optimization, verifiable by pointer-identity across two cards that
name the same subtype plus an unchanged behavior/render suite.

## Scope

Perf/infra — lowest priority tier (perf last). Not limited to subtypes: any
repeated `Arc`-carrying macro invocation benefits, but the concrete payoff that
motivates it is the `Arc<Subtype>`/`Arc<TypeDef>` filter payload, hence the
`needs`. Standard constraints apply. Related: `filter-permanent-of-type-subtype`
(introduces the Arc payload this optimizes).
