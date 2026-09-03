---
needs: []
---
**A cost block can begin paying and then fail partway, leaving the
mutation it already made.** Routed from `core-regions-test-restoration`'s
landing, 2026-09-02; two restored tests in
`crates/deckmaste_engine/tests/payment.rs` are `#[ignore]`d on it.
Standard constraints apply.

The whole-body preflight went out with `ChooseAndPay` in the cost-block
landing. A `Choose` step now validates only that its candidates exist, so
a witness that cannot be paid atomically is accepted, and the failure
surfaces after earlier steps of the same block have already applied.
[CR#601.2h] requires a cost to be paid in full or not at all, so a block
that mutates and then fails is a rules violation, not just untidy.

Restore an equivalent check against the instruction-block shape: before
any step applies, the whole block must be payable together. It is not a
per-step candidacy test, which is what exists now.

**A preflight is the only rules-legal mechanism here, not merely the
tidier one** (verified against the Comprehensive Rules on 2026-09-02, while this ticket was
blocked from being claimed). [CR#733.1] supplies the remedy for an action
a player starts but cannot legally complete: the entire action reverses
and payments already made are cancelled. But its tail excludes exactly
the steps that cannot be rewound — those that moved cards to a library,
moved cards from a library anywhere but the stack, shuffled a library, or
revealed cards from one. That exclusion is why [CR#601.2h] orders payment
with the random and library-touching costs LAST. So for the sampling cost
subject that `core-regions-random-cost-subject` added, a rewind is
forbidden outright and the whole-body check is the only conforming
design. Say so in the landing record.

## Gates

The two ignored tests un-ignore and pass. A deliberately unpayable
multi-step cost leaves the game state untouched. Standard suites green.

## Landing record

Payment-time `Choose`, `Sample`, `Search`, and `Let` instructions now bind
their result first in a cloned game state and preflight the consecutive action
IOUs that spend it in declaration order. The real register file, RNG, and game
image are touched only after that projection succeeds. The projected action
set restores the former `ChooseAndPay` whole-body checks for sacrifice, zone
moves and reminting, tap/untap, life, counters, reveal, and discard composites.

Preflight is required rather than merely tidier. [CR#601.2h] forbids partial
payment and places random and library-touching costs in the final payment tier.
Although [CR#733.1] generally reverses an illegal action, it expressly forbids
reversing library moves, shuffles, and library reveals. A sampled cost can
therefore consume non-rewindable entropy or select a non-rewindable library
move; proving its whole action body payable before the real sample is the only
rules-conforming design.

- Construction count: unchanged; this is an engine-only payment check.
- Coverage and lock state: payment integration tests went from 30 passed plus
  2 ignored to 33 passed and 0 ignored. `cr-citations.lock` and all other
  coverage locks are unchanged.
- Assurance: restored 2; re-spelled 1
  (`choose_and_pay_rejects_sacrificing_an_opponents_permanent` now authenticates
  refusal at the choice boundary); ignored with blockers 0; added 1
  (`sampled_cost_preflights_before_consuming_rng_or_mutating_state`); removed 0.
- Positive artifacts: `cargo test -p deckmaste_engine` passed all unit,
  integration, and doc tests (aside from the existing explicitly ignored
  50,000-game Monte Carlo); payment integration suite 33 passed, 0 failed, 0
  ignored; `cargo clippy -p deckmaste_engine --all-targets -- -D warnings`;
  `cargo fmt --all -- --check`; citation scan 0 non-compliant strings and 0
  stale of 17,887; diff audit verified all 4 changed citation sites.
- Workspace gate limitation: `cargo test --workspace` reaches an unrelated
  `deckmaste_construction` trybuild mismatch in 4 of 28 compile-fail fixtures
  because new `parser-metrics` cfg warnings are absent from their checked-in
  stderr. The same focused compile-fail suite fails identically on refreshed
  `default`; no engine test fails.
- Deviations and additions: added the random-sample atomicity regression beyond
  the ticket's two named ignored tests because the ticket's rules analysis
  specifically makes non-rewindable sampling the decisive case. Also corrected
  the ticket's citation-like date prose so the repository citation scanner can
  distinguish it from a bare rule citation.
- STOPs: none.
