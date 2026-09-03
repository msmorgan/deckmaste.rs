Make coverage-lock drift a check failure. `coverage --check` fails only
on lost identities; unblessed `newly covered` identities are reported and
return Ok (coverage_lock.rs ~:361-378), which is why three of the last
six landings left the lock behind (328, 1,097, and 2 identities). Add-only
bless doctrine means every landing must bless; enforce it: `--check`
exits non-zero on any newly covered identity, with the message naming
`--bless`. Keep the retire path exactly as authenticated. The retire path stays executor-runnable but prints a loud notice that a
retirement is a coordinator ruling (a ticket-vs-purpose contradiction is a
STOP, per CLAUDE.md) and requires a re-coverage or retirement obligation
line in the landing record. Standard constraints apply.

## Landing record

- Coverage remained 15,932 selected and locked identities out of 32,641
  units before and after this gate-only change. The schema-2 lock remains at
  15,932 identities with source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and SHA-256
  `a466fcf1289b7607095d050bab1bf69388e47dbe65a371409cda6881aa8fbe6a`.
- `coverage --check` now lists every newly covered identity and then fails
  with an error naming `--bless`; the regression test also proves that this
  refusal leaves the lock byte-for-byte unchanged. Loss still fails through
  the existing path.
- The authenticated exact-manifest retirement path remains executor-runnable.
  A successful retirement now also prints the required coordinator-ruling,
  STOP, and landing-record warning. Re-coverage/retirement obligation: none;
  no identity was retired and the coverage lock did not change.
- Positive gate artifacts: production `cargo xtask english_v2 coverage
  --check` passed with zero selected-uncovered units, ties, internal failures,
  exceptions, round-trip mismatches, ownership failures, gaps, overlaps,
  synthetic claims, provenance mismatches, or literal/lexicon collisions;
  unfiltered xtask tests passed (399 library + 11 CLI + 1 determinism, 1
  ignored); strict all-target xtask Clippy and xtask formatting passed.
- Deviations and additions: none. No constructions were added, changed, or
  deleted; test changes are exactly the ticket's gate and retirement-warning
  regressions.
- STOPs: none.
