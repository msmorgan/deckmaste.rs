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

## Gates

The two ignored tests un-ignore and pass. A deliberately unpayable
multi-step cost leaves the game state untouched. Standard suites green.
