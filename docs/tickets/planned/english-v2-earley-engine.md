---
needs: [english-v2-vertical-slice]
---
**Build `english_v2`'s real parser — the Earley-family chart engine — driven
by hand-written grammar tables for the slice corpus, and DELETE the
recursive-descent stopgap in the same change.** Urgent-ordered ahead of the
declaration compiler: the stopgap is the architecture the ADR bans
(ordered-choice single-commit parsing), and this project's history is that
wrong-architecture code recruits compensations if allowed to sit.

Authority: `docs/decisions/english-v2-rewrite.md` §Parsing, selection, and
failure. Decisions already made:

- Earley-family chart: two-kind rule positions (nonterminal | lexical), scan
  as span-returning injection (the terminal tiers plug in there), packed
  forest preserving all surviving readings. The salvage ledger's Part A
  entries A1-A3 record what the old chart core got right; ideas, not code.
- Selection is a separate post-parse pass — computed structural specificity
  plus a countable exception table; ties are hard errors naming both
  constructions. On the slice corpus this pass is near-trivial; build the
  seam, not sophistication.
- Failure surface: furthest chart column + its live expectation set replaces
  the stopgap's handwritten prose expectations.
- Grammar tables for the slice corpus are HAND-WRITTEN in this ticket (the
  same hand-written-golden pattern as the slice itself); the declaration
  compiler later generates them diff-identical. Do not build any part of the
  compiler here.
- No dual-parser period: the engine lands and `parser.rs`'s recursive-descent
  code is deleted in the same change. The existing test suite — round-trip
  both laws on all slice sentences (including the slice-hardening additions),
  constructed-value render, loud failure — is the acceptance corpus and must
  pass unchanged except the loud-failure test, which updates to assert the
  chart-derived error shape.

Acceptance: all slice tests green on the engine; `parser.rs` recursive
descent gone; forest + selection seam present and exercised by at least one
deliberately ambiguous toy input (hard-error tie asserted); no dependency
changes (english_v2 stays a leaf). Standard constraints apply.
