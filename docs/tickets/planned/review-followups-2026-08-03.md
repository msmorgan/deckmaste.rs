---
needs: [engine-interpreter-arm-extraction, engine-test-support-consolidation, engine-verb-ident-table, english-large-function-extraction, todo-harness-ci]
---
**Completion record for the low-severity follow-ups from the 2026-08-03
external review.** The original batch mixed correctness hardening, test
infrastructure, two unrelated refactors, and CI plumbing. Each is now an
independently scoped dependency; this ticket is only their final audit.

After every dependency is done:

- rerun the verb-literal and duplicated-helper inventories and explain any
  intentional survivors;
- verify the extracted engine and English functions retained test counts,
  citations, and behavior;
- run `fish tests/todo_test.fish` through the same path CI uses; and
- record any genuinely ad-hoc cleanup from the review, including the
  `replace.rs` test-import placement nit, rather than minting lifecycle work
  for it.

This completion record does not block `portfolio-polish`; only the
correctness- and cold-clone-facing child tickets do.
