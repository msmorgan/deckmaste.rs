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

Scope fence — the other three v1 normalization helpers are deliberately
NOT ported: expand_keyword_lines (v2 parses keyword lines as document
structure), self_ref_to_tilde (v2 resolves self-reference via the
environment's context_name), and expand_repeated_from_lines
(coordination grammar's job). v2's existing typography normalization
(quote straightening, roll-row dashes) is untouched. Reminder stripping
is the single restored piece.

Expect a substantial coverage jump (the paren wall was ~9.7k units).
Genuine tie or contradiction = STOP-and-report. Standard constraints
apply.

Completion (2026-08-28):
- The Vintage snapshot audit found 10,333 parenthesis-bearing units and
  11,136 affected lines. Every affected line had balanced, nonempty,
  single-line groups; none had nested, empty, or cross-line parentheses,
  and one line had two groups. Corpus normalization now ports the narrow
  v1 behavior after the existing quote/dash normalization.
- The one-time migration enumerated 561 old->new covered-identity pairs in
  an untracked audit manifest. The old side exactly equalled the complete
  lock loss set, every distinct new side selected with total ownership,
  and the existing authenticated `coverage --bless --retire` gate accepted
  the sorted old side. The ordinary post-migration `coverage --check`
  passes without retirement input.
- Coverage rose from 10,540 to 14,259 selected and covered units, a net
  gain of 3,719. The schema-2 lock retains source fingerprint
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and has SHA-256
  `4b51716ae555489ec6411ad6d60666e996387212c3bd6581105e7330e20730cf`.
  Selected-uncovered units, ties, internal failures, exceptions,
  round-trip mismatches, ownership failures, gaps, overlaps, synthetic
  claims, and provenance mismatches are all zero.
- End-to-end probes pin a reminder-only Plains as a selected, totally
  owned empty document and A.I.M. Bot's keyword-plus-reminder text as
  selected, totally owned `Flying`. The old "Don't strip" fixture and the
  inspect/probe normalization fixtures now assert the stripping contract.
- The dead `KeywordReminderText` construction, AST/traversal exports,
  diagnostic category, and exact-reminder grammar test are deleted; the
  12-10 ticket records the superseding erratum.
- Unfiltered `deckmaste_english_v2` and `xtask` tests pass. Strict
  all-target Clippy passes for both crates.
