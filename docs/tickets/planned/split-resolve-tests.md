---
needs: []
---
**Hoist the embedded test suites out of `crates/deckmaste_engine/src/resolve/
action.rs` and `resolve/effect.rs` into sibling test files.** The `resolve/`
*code* split already happened; these two files have since grown into 5.5k/5.1k
line files — but the growth is test volume, not logic. Measured 2026-07-24:
`action.rs` logic ends at line 965 (~4.6k test lines, 82%); `effect.rs` logic
ends at 1,851 (~3.3k test lines, 64%). The logic itself is reasonably sized
and does NOT need decomposing; framing this as a logic-behemoth split (as an
external review did) misreads it.

## Shape

Per file: move the embedded `mod tests` body to a sibling (`action/tests.rs`
or `#[path]`-free equivalent by converting the file to a directory module),
keeping the module path, test names, and CR-citation texts identical. Shared
construction helpers go to the existing `#[cfg(test)] mod test_support`
(`src/test_support.rs`) — extend it rather than minting a second fixtures
home. While in there, `player_action.rs` (tests from 660/2,962) and
`count.rs` (652/2,325) are optional same-pattern extractions if cheap;
don't force them.

## Gates

Standard constraints apply, plus: exact `#[test]`-count parity before vs
after; `cargo xtask cite check` 0 stale (the test bodies carry CR citations,
which move files); the comment-multiset audit and the tests-mod-import
gotchas from the [[split-trigger-rs]] template apply verbatim.
