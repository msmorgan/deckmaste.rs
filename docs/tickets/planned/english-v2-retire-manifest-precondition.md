---
needs: [english-v2-plan-09-effect-and-predicate-grammar]
---
Enforce the coverage-lock retire manifest's "untracked" precondition in code.

The Plan 09 retire gate documents its manifest as an untracked, nonempty,
strictly sorted list of lowercase 64-hex identities. Seven of those properties
are enforced in `crates/xtask/src/english_v2/coverage_lock.rs` — `--retire`
requires `--bless` at both the clap and runtime level, an existing schema-2
lock is required, the manifest must be nonempty, strictly sorted and
well-formed, it must equal the complete lost set exactly, it is refused when
nothing is lost, and every bail precedes the write so a mismatch leaves the
lock byte-for-byte intact.

"Untracked" is not enforced anywhere: nothing in `coverage_lock.rs` or
`coverage.rs` consults tracking state. The only thing stopping a plan-scoped
identity list from being committed and then blessed from a tracked path is
convention — against the ruling that process artifacts never enter the source
tree, whose whole point is that convention already failed once.

The check is cheap: refuse a `--retire` path that `jj file list` reports as
tracked. Acceptance: blessing a retirement from a tracked manifest path is
refused with a clear message, the lock is untouched by that refusal, and a
regression test covers it.
