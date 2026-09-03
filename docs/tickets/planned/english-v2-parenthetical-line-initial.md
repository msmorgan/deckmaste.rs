Close the line-initial twin of the tripwire escape (tripwire-fixes
review M1). "(a wholly novel gloss) trample." still normalizes to
"trample." with exit 0: the tripwire classifies mid-line groups only, and
a novel group at line START followed by text escapes. Relax the predicate
so any group followed by non-whitespace text on its line is classified
(`!after.trim().is_empty()`); line-FINAL groups (10,414 in the corpus,
the ordinary trailing-reminder shape) stay outside classification — the
ticket's earlier "every group wherever it appears" clause is superseded
by this reading, which is the only one consistent with corpus load and
identity stability. Zero non-classified line-initial-with-text groups
exist in the corpus, so identities cannot move; prove it. Also retire
`previous_strip_reminder_text`: with preservation restored it equals the
shipped scanner, making "the normalizer equals the pre-tripwire
normalizer" a standing law — replace it with the positive probes that
now exist. Standard constraints apply.
