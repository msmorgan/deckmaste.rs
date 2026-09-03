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
tidier one** (verified against the CR 2026-09-02, while this ticket was
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
