Pin the retirement authenticator in production (retire-manifest landing
review M1/M2/L3). The production `authenticate_retirement` path — the
`jj file list` shell-out, the `default`-workspace refusal, and the
workspace-name -> wip ticket derivation — has no test reaching it: every
retirement test injects a closure, and deleting the single wiring line
(`let mut authenticate = authenticate_retirement;`, coverage_lock.rs
~:249) leaves the suite green. Add tests that drive the real
authenticator (scratch jj repo in a temp dir: tracked manifest refused,
default refused, ticket path derived from the workspace name, obligation
line required). Also: `run_jj` must pass `--ignore-working-copy` so the
check never snapshots (it can currently CREATE the tracking it then
refuses, and snapshots on the default refusal); accept the natural label
`re-coverage obligation:` as a fourth spelling and quote the accepted
labels in the refusal message; a mixed loss+gain run prints the gain
count. Zero behaviour change otherwise; standard constraints apply.

## Landing record

- Coverage remained 15,966 selected and locked identities out of 32,641
  corpus units before and after this gate-only change; constructions remained
  378 -> 378. The schema-3 lock remained byte-for-byte unchanged at SHA-256
  `46d7754d9a4f56b68a63dca1bf4b311789eeca0c826a372be787b826514d33fd`,
  with source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and normalization digest
  `dfd487831e5cd6d02b3ec9d8087e649463b1d023b62cf2916084f4099486d749`.
- The production authenticator now invokes both `jj workspace root` and
  `jj file list` with `--ignore-working-copy`. One child-process regression
  drives the real `apply_with_retirement` wiring through four scratch jj
  workspaces: a tracked manifest is refused without changing the lock,
  `default` is refused without snapshotting a pending file, the workspace-root
  name derives the tracked WIP ticket path, and a landing-record obligation is
  required.
- `re-coverage obligation:` is accepted alongside the three existing labels.
  A refusal quotes all four spellings, and a successful mixed loss/gain run
  reports both the gain count and the gained identity.
- Retirement/re-coverage obligation: none; no corpus identity was retired and
  the production coverage lock did not change.
- Positive gate artifacts: `cargo test -p xtask` passed 413 library tests (1
  documented ignore), 11 CLI tests, and 1 determinism test; strict all-target
  xtask Clippy passed; production `cargo xtask english_v2 coverage --check`
  passed with 15,966 selected-covered identities and zero selected-uncovered,
  unresolved, internal, exception, round-trip, ownership, gap, overlap,
  synthetic-claim, or provenance-plan failures. Temporarily replacing the
  production authenticator function pointer with an accepting closure made the
  new regression fail on the tracked-manifest scenario; restoring the pointer
  returned it to green.
- Assurance counts: restored 0, re-spelled 0, ignored with blockers 0, added 1,
  removed 0.
- Deviations and additions: no constructions, corpus fixtures, or tests beyond
  the ticket's requested production authenticator regression were added,
  changed, or deleted.
- STOPs: none.
