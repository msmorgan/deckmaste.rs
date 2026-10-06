---
needs: []
---
# Make the complete corpus run affordable without capping enumeration

The complete corpus run is the progress measure for this grammar, and it has
become too slow to use as one. Recorded figures, each for its own tree:

- 207,807 ms wall for all 32,828 supported faces, 1,031,953 ns/B checked-text
  thread CPU, 24 workers, host load 3.31 / 5.25 / 3.98 with concurrent test
  activity (tree `vwtnryos`, covered 13,226);
- 235,015 ms, 960,452 ns/B, 24 workers, load 3.59 / 3.83 / 3.64 (tree
  `ovrpzmym`, covered 11,249);
- on that second tree Avacyn, Guardian Angel alone took 150,638 ms to validate
  its 33,856 Readings after a 0.099 s chart.

The advisory ceiling is 16.26 s on a quiet host. The records conclude that
structural enumeration and validation, not chart admission, dominate. Cost
grows with coverage and with every legitimately ambiguous face, so later
grammar tickets make it worse.

Profile before designing: confirm where the time goes on the current tree and
how it is distributed across faces. Then reduce the cost of the complete run
while keeping it complete. The obvious candidate is to validate at the level of
the packed forest, so a shared substructure is checked once rather than once
per Reading that contains it; choose on the profile, not on this sentence.

Constraints. Every admitted Reading is still validated: declaration admission,
lexical ownership, byte-exact realization, construction and word traversal.
Enumeration is not capped, sampled or truncated to meet a time. If any per-face
budget is introduced, a face that exceeds it is reported as undetermined, never
as covered, and the budget and the list of such faces are disclosed. Admission
and Reading identity do not change. If a faster scheme would change what
"validated" means for a Reading, STOP and report before building it.

`english-v3-packed-preferred-readings` owns returning a preferred Reading
without enumerating; this ticket owns the complete census. They may share forest
machinery; neither substitutes for the other.

Acceptance: the complete run on one tree before and after reports identical
identities, Reading counts and validation results; wall time, per-byte thread
CPU, memory and the slowest faces are reported with host load and worker count;
and the ticket states the corpus-runtime and forest-growth ceiling that
`english-v3-production-cutover` adopts, with the measurement that justifies it.
Standard constraints apply.
