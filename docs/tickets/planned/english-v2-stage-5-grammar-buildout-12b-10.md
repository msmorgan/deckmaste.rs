---
needs: [english-v2-stage-5-grammar-buildout-12-10]
---
Inline and whole-text reminder text (split B). Scope: inline sentence
reminders and whole-text reminders (basic lands' "({T}: Add {G}.)"), with
parentheses owned exactly. Reminder content parses with the same grammar
where it is rules-shaped; where it is not (informal phrasing), bring the
handling back as a STOP question with examples, not an improvised
carve-out.

Acceptance: gates green with the ratchet strictly up; zero ties and
exceptions; byte-exact both directions with total ownership; no process
artifacts in tracked source; genuine tie or contradiction = STOP-and-report.
Standard constraints apply.

Sizing note (composition review): the parenthesis frame already works —
this chunk's unlock is bounded by reminder-BODY grammar. The commonest
missing body construction is the "with <keyword>" postmodifier
("creatures with flying"); 343 of 9,838 paren-bearing units are selected
today. Build the postmodifier as a general declared-keyword consumer,
never per-keyword.
