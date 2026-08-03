---
needs: []
---
**Consolidate duplicated engine integration-test setup without erasing its
semantics.** `fn force_onto_battlefield` is copied across nine integration-test
files, with similar duplication around `card()`, `two_player_with()`, and
`plugin()`. The copies have already drifted: some search the library after the
hand, while others intentionally require a hand card or accept an existing
object id.

Add shared integration-test support under `crates/deckmaste_engine/tests/` and
factor only behaviorally identical setup. Expose distinct, intention-revealing
helpers for materially different operations (for example hand-only,
hand-or-library, and existing-object movement); do not create one permissive
"union" helper whose fallback behavior can make a test pass with the wrong
fixture.

Migrate the duplicated callers in coherent batches. Acceptance requires exact
integration-test count parity, unchanged fixture search semantics at every
call site, and a final inventory explaining any deliberately local helper.
