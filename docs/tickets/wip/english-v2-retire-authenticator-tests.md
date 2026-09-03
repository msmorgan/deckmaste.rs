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
