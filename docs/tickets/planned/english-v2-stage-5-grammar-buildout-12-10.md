---
needs: [english-v2-stage-5-grammar-buildout-11-10]
---
Keyword-line block composition and trailing reminder text (split A of the
former reminder-text chunk; inline reminders are 12b).

FIRST, the carried HIGH finding (keyword-landing-review.md): a keyword line
currently terminates only at end of input — "Flying\nDraw a card." fails
while the reverse order parses; 6,855 units (27.5% of remaining failures)
die at first-newline+1. A keyword line must compose in any block position.
This shipped without a STOP; for the record, a construction that cannot
compose with sibling blocks is a contradiction-class STOP, not a detail.

Then trailing reminder text: parenthetical reminders after keyword lines,
parentheses and italics-boundary bytes owned exactly (2,317 further
single-line failures are keyword-plus-reminder).

Opening fixes carried from the review: un-narrow the keyword parameter
categories (subject takes the general nominal category — "Enchant creature
you control"; quality is open, not a 3-member sum — "Protection from
everything"); delete the hardcoded vocab CounterfactualAbility and vocab
Status keyword surfaces and route them through declarations; fill the
Designation inventory toward the CR designation set before relying on it
(counter kinds did this correctly: inventory first, then vocab deletion).

Acceptance: gates green with the ratchet strictly up; zero ties and
exceptions; byte-exact both directions with total ownership; no process
artifacts in tracked source; genuine tie or contradiction = STOP-and-report.
Standard constraints apply.
