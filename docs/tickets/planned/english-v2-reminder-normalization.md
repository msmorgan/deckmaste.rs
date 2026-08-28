---
needs: [english-v2-stage-5-grammar-buildout-12-10]
---
Strip reminder text in v2 corpus normalization (user ruling 2026-08-28;
supersedes the deleted 12b-10 reminder-grammar chunk). The v1-era
precedent is deckmaste_migrations extract.rs strip_reminder_text —
single-line parentheticals removed keeping at most one surrounding
space, wholly-reminder lines deleted; port the BEHAVIOR into the v2
corpus normalization step (do not depend on deckmaste_migrations; audit
the regex against attested shapes rather than copying blind). The v2
corpus.rs test asserting reminder text is preserved is superseded. A
whole-text reminder normalizes to the empty document, which is valid and
selectable per the empty-document ruling (basic lands become vanillas).

Consequences owned by this ticket:
- One-time blessed lock migration: paren-bearing units re-key (identity
  hashes the normalized text). Enumerate old->new identity pairs, report
  the count, use the authenticated bless mechanism (the 49-identity
  precedent); zero coverage may be LOST net of re-keying — every
  previously-selected unit's stripped text must still select under its
  new identity.
- Delete the now-dead trailing-reminder circumfix constructions from the
  12-10 landing, with an erratum note on that done ticket.
- Probes: a basic land selects as an empty document; a keyword+reminder
  french vanilla selects; "Don't strip" fixture updated to the new
  contract.

Expect a substantial coverage jump (the paren wall was ~9.7k units).
Genuine tie or contradiction = STOP-and-report. Standard constraints
apply.
