---
needs: [english-v2-vertical-slice]
---
**Build `english_v2`'s parser — the ONLY parser it will ever have: an
Earley-family chart engine over hand-written grammar tables for the slice
corpus.** Stage 3 of the rewrite's implementation sequence, deliberately
ahead of the declaration compiler. Ordered choice, PEG, recursive descent,
precedence climbing, and every other single-commit strategy are banned by
`docs/decisions/english-v2-rewrite.md` §Parsing — if the chart approach hits
a wall, STOP and report; never substitute an easier parser. A previous run
of this effort failed exactly that way.

Pinned by the ADR; none of this is the implementer's to re-decide:

- Chart with two-kind rule positions (nonterminal | lexical); scan is a
  span-returning injection hook where the terminal tiers (vocab, lexemes,
  codecs, identities/catalogs) plug in. The ADR's salvage-ledger appendix
  entries A1–A3 record what the old chart core got right — ideas only,
  never code.
- Packed forest preserving all surviving readings; selection is a separate
  post-parse pass (computed structural specificity plus a countable
  exception table, empty for now); a tie neither can break is a hard error
  naming both constructions.
- Failure surface: a structured error carrying the furthest chart column's
  span plus its live expectation set — derived from the chart, never
  hand-written prose lists.
- Grammar tables for the slice corpus are HAND-WRITTEN here (the same
  hand-written-golden pattern as the slice); the declaration compiler
  generates them diff-identical in stage 4. No compiler work of any kind in
  this ticket.
- Positional case is checked at parse exactly as render derives it: words
  match their lowercase forms plus a case check against the positional
  derivation (ability-initial and after-terminal-period ⇒ capitalized;
  elsewhere ⇒ lowercase; identities keep inherent case). A non-initial
  "you" parses; "you Gain" does not.
- Acceptance tracks grammatical Oracle English, not rules legality — the
  slice's deliberately rules-invalid sentence must parse.

Acceptance: both round-trip laws (`render(parse(s)) == s` and
`parse(render(v)) == v`) byte-exact on the slice ticket's four sentences
plus "Whenever a player connives, you gain X life."; a loud-failure test
asserting the chart-derived error shape (span + expectation set); one
deliberately ambiguous toy input asserting the hard-error tie; the slice's
render and constructor tests still green; `deckmaste_english_v2` stays a
leaf. Standard constraints apply.
