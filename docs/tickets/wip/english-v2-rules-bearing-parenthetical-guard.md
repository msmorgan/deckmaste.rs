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
