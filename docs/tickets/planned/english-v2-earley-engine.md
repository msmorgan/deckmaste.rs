---
needs: [english-v2-vertical-slice, english-v2-slice-hardening]
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
- The tables are declarative const DATA in their own module — one rule per
  construction form, uniform, written exactly as the stage-4 generator
  would emit them (the same golden discipline as the AST). Each rule pairs
  with one uniform build arm lowering its children to the construction
  struct through the existing public checked constructors; no per-rule
  cleverness a compiler could not plausibly generate.
- ONE public entry point: `parse(text, ctx)` at the `Ability` category,
  returning the checked AST or the structured error — bare sentences parse
  as the ability-level sentence wrapper the slice AST already has. Forest
  and selection types stay crate-internal; this ticket ships no other
  public parse surface (the xtask `inspect`/`probe` surfaces arrive with
  later stages).
- Expectations in the error are STRUCTURED values derived from the live
  chart items (nonterminal category, terminal class, or literal), never
  hand-written prose lists — the stage-2 stopgap's prose expectation
  strings are the named anti-pattern. `Display` may prettify.
- Whitespace and punctuation attachment are VERIFIED at parse exactly as
  render derives them, never normalized: a doubled space or a missing space
  is a parse failure. Accepting-then-normalizing is a canonicalizing
  parser, which this layer bans.
- The deliberately ambiguous input for the tie test uses a TEST-ONLY table
  set; the shipped slice tables must select uniquely — zero ties — on every
  accepted sentence.
- Positional case is checked at parse exactly as render derives it: words
  match their lowercase forms plus a case check against the positional
  derivation (ability-initial and after-terminal-period ⇒ capitalized;
  elsewhere ⇒ lowercase; identities keep inherent case). A non-initial
  "you" parses; "you Gain" does not.
- Both laws are context-threaded (ADR §Bidirectionality): `parse(s, ctx)`
  shares the render context — the card's own self-name — and the scan hook
  matches the self-reference against ctx, never against stored name text.
- Acceptance tracks grammatical Oracle English, not rules legality — the
  slice's deliberately rules-invalid sentence must parse.

Acceptance: both round-trip laws (`render(parse(s)) == s` and
`parse(render(v)) == v`) byte-exact on the slice ticket's four sentences
plus "Whenever a player connives, you gain X life."; a loud-failure test
asserting the chart-derived error shape (span + structured expectation set);
wrong-case AND wrong-whitespace variants asserted as parse failures; one
deliberately ambiguous toy input (test-only tables) asserting the hard-error
tie; the slice's render and constructor tests still green; dependencies
unchanged (catalog layer only). Standard constraints apply.
