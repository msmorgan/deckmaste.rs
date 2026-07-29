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

## Lint-allow narrowing

`grammar/mod.rs` opens with a module-wide `#![allow(dead_code, …)]`
("staged for later milestones") — at 6k lines it also silences genuinely
dead code (an orphaned scanner or Key variant would never warn). The split
is the natural moment to narrow it: keep the allow only on the staged
submodule(s), or per-item. (`catalog.rs` carries the same pattern
crate-wide; out of this ticket's file set — note only.)

## Gates

Standard constraints apply, plus: exact `#[test]`-count parity before vs
after; census identity and roundtrip as above; `cargo xtask cite check` (CR
citations move files); the comment-multiset audit from the
[[split-trigger-rs]] template. Its "Gotchas" section (interleaved test-mod
imports, orphaned section banners, `cargo fix --allow-no-vcs`) applies
verbatim.

## Completion (2026-07-28)

- Inspection found that the stable seams are the chart lifecycle and dispatch
  phases: rule registration, reduction, lowering, scanning, and parse support.
  `clause.rs` now mirrors those registration/reduction/lowering phases, with
  its tests isolated from production code. The constructor's registration
  call sequence remains unchanged.
- Relocated the parent-level nominal suite to `grammar/tests/nominal.rs` and
  the literal-audit wrapper to `grammar/tests/litaudit.rs`. The grammar test
  count remains exactly 429, and the normalized 1,847-line comment multiset is
  identical before and after the move.
- Removed the parent-wide `dead_code` allowance. The few staged variants and
  helpers that still require it now carry narrow, reasoned allowances.
- The before/after recovery reports are byte-identical (SHA-256
  `f14d8a37088b788fe5b2e6e0fd06700c79e2716f63e167dbbe1f74dbc99a6ce0`),
  and all 31,685 supported faces round-trip cleanly with zero mismatches or
  render errors.
- `cargo test -p deckmaste_english` passes (576 unit tests and 115 public API
  tests); formatting and all-target checking are clean. Strict Clippy reports
  only the same pre-existing grammar/test/renderer warnings as the parent
  line; with that baseline allowed, every split module is warning-clean.
- `cargo xtask cite check` reaches the same two malformed placeholder
  citations in `docs/tickets/planned/comment-discipline-sweep.md`; no moved CR
  citation is stale or malformed.
