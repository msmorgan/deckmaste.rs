---
needs: [english-structural-recovery-zero]
---
**Carve `crates/deckmaste_english/src/grammar/mod.rs` (~6.0k lines) and
`grammar/clause.rs` (~5.5k lines) into a submodule structure**, following the
`resolve/` split template in [[split-trigger-rs]] (mv-first, `pub(super)`
seams, `pub(crate) use` re-exports, comment-multiset audit).

The `needs:` gate is load-bearing, not politeness: the structural-recovery
campaign appends rules and reduction callbacks to both files every round, and
a concurrent split buys conflicts for nothing. Claim only once that campaign
is done.

## Hard constraint: RuleId registration order

RuleId ordering is a global parse tiebreak — reordering rule registration
changes which parse wins for ambiguous spans corpus-wide. The split must keep
the `add_*_rules` call sequence byte-identical; moving a rule's *definition*
into a submodule is free, moving its *registration position* is not. Gate:
`cargo xtask english recovery` census byte-identical in every cell, and
`cargo xtask english roundtrip --require-clean` stays 0-dirty, before vs
after. Any census cell movement means registration order changed — stop and
fix, don't rationalize.

## Boundaries

Derive submodule seams from the actual reduction-callback groupings (read the
`RuleTag` dispatch clusters), not from an a-priori taxonomy. A starting
hypothesis to verify against the code, not assume: imperative / copular /
conditional / coordination for `clause.rs`; lexical-scan vs rule-registration
vs lowering for `mod.rs`.

## Stranded test suite: `grammar/nominal.rs`

`grammar/nominal.rs` (~1.4k lines) is 100% `#[cfg(test)]` from line 1 — a
test suite for the *parent* module (`use super::super::*`) left behind when
the nominal rules were consolidated into `mod.rs`, still declared as a plain
`mod nominal;`. This split owns fixing it: relocate those tests beside
whichever submodule inherits the nominal rules (or an honestly-named tests
module), so the filename stops promising grammar it doesn't contain.

## Gates

Standard constraints apply, plus: exact `#[test]`-count parity before vs
after; census identity and roundtrip as above; `cargo xtask cite check` (CR
citations move files); the comment-multiset audit from the
[[split-trigger-rs]] template. Its "Gotchas" section (interleaved test-mod
imports, orphaned section banners, `cargo fix --allow-no-vcs`) applies
verbatim.
