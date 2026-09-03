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


Expanded scope (lock-gate landing review M1/M2, 2026-09-02):
- Authority, not disclosure: `--bless --retire` must fail unless the
  manifest is authenticated (the documented "untracked manifest"
  precondition enforced in code) AND the claimed ticket's landing record
  contains a retirement/re-coverage obligation line naming each retired
  identity. An unattended retirement is the exact wrong-authority path the
  parenthetical-guard landing took.
- Fingerprint drift (coordinator default, veto-able): a lock whose corpus
  fingerprint no longer matches the snapshot fails `--check` like
  newly-covered drift does — a data refresh changes identities and must
  be blessed honestly, never left advisory.
- Loss + gain in one run reports both, not only the loss.

## Landing record

- Coverage remained 15,966 selected and locked identities out of 32,641 units
  before and after this gate-only change. The schema-2 lock remains byte
  unchanged at SHA-256
  `b6a4a064882ecde7dca01b41345c805e6bfba7206fb6b58a63af807bcc91406e`
  with source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`.
- A retirement manifest is now authenticated against `jj file list`; a tracked
  path is refused before any lock write. The gate also requires the current
  feature workspace's tracked WIP ticket and one landing-record obligation
  line naming every retired identity.
- Fingerprint-only drift now fails `--check` and names `--bless`. Drift
  diagnostics are emitted before loss handling, so a mixed loss/gain run
  reports every newly covered identity as well as every lost identity.
- Retirement/re-coverage obligation: none; no identity was retired and the
  coverage lock did not change.
- Positive gate artifacts: `cargo test -p xtask` passed 409 library tests (1
  documented ignore), 11 CLI tests, and 1 determinism test; strict all-target
  xtask Clippy and xtask formatting passed; production `cargo xtask english_v2
  coverage --check` passed with 0 selected-uncovered units, ties, internal
  failures, exceptions, round-trip mismatches, ownership failures, gaps,
  overlaps, synthetic claims, or provenance mismatches.
- Assurance counts: restored 0, re-spelled 0, ignored with blockers 0, added 2,
  removed 0. The two added regressions cover tracked-manifest byte preservation
  and exact landing-record obligation authentication; fingerprint-only drift
  and mixed loss/gain reporting extend the existing schema-2 gate test.
- Deviations and additions: none. No constructions were added, changed, or
  deleted; all code and test changes implement the ticket's gate scope.
- STOPs: none.
