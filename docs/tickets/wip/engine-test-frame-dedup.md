---
needs: []
---
Dedup the test-fixture boilerplate in `crates/deckmaste_engine/src/resolve.rs`
tests: the repeated `Frame { bindings: None, chosen: None, x: None }` construction
(~56 occurrences) and the 3-line "place on the battlefield" stanza repeated across
several tests (around lines 2010 / 2088 / 2185 / 2452). A `frame_for()` helper
already exists in the `condition.rs` test module but isn't reused here — reuse it
(hoist to a shared engine test-support module if that's cleaner) and add a small
place-on-battlefield helper. Pure test refactor: behavior and assertions
unchanged, `cargo test -p deckmaste_engine` stays green. Invisible to library
consumers; surfaced by the pre-publish code-smell pass.
