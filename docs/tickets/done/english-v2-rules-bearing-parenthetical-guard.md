Guard the reminder strip against rules-bearing parentheticals
(reminder-normalization review F1-F3). 20 corpus units carry non-italic,
rules-bearing parentheticals the strip currently deletes: the
"(as long as this creature is on the battlefield)" clause family,
"(if it's still on the battlefield)", "(even if this card isn't on the
battlefield)", Eluge's "{U} (or {1})", and 11 DFC/Saga "(front face up)"
markers. All 20 are parse failures today, so the lock is uncorrupted —
but when the grammar admits their residue they will select byte-exactly
while silently missing a rules clause, and no existing gate can see it.

Add a mid-line-parenthetical guard to normalization: classify each
stripped group (position, italic status from printed text if available,
or an authored exception list — counted, review-surfaced) so
rules-bearing groups are preserved for the grammar instead of deleted.
Fold in: delete the v1 fossil contract that "(Reminder) Foo" leaves a
leading space (zero corpus instances), and assert on nested parens
(currently silently mangled; zero today). The guard must not change any
currently-selected unit's normalized text — identity stability probe
required.
Standard constraints apply.

Completion (2026-09-02):
- Replaced the regex-only strip with a structural single-line scanner. Five
  authored rules-bearing parentheticals survive byte-exactly, nested groups
  assert, and a leading reminder no longer leaves a fossil space.
- The authoritative Vintage-filtered snapshot contains 17 guarded units, not
  the ticket's original 20-unit review figure. A production-data test pins the
  five per-spelling counts at 2, 1, 11, 2, and 1.
- The stability probe caught two identities that had begun selecting only
  after their `(front face up)` clause was deleted: The Great Synthesis and
  Galian Beast. Both corrected texts are parse failures, so the authenticated
  coverage migration retired exactly those two invalid identities; every
  other selected identity is unchanged.
- Final coverage is 32,641 total / 15,932 selected and covered, with zero
  selected-uncovered units, ties, internal failures, exceptions, round-trip
  mismatches, ownership failures, gaps, overlaps, synthetic claims, or
  provenance mismatches. Unfiltered xtask tests and strict all-target xtask
  Clippy pass.
