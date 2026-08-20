---
needs: [english-v2-open-grammar-environment, english-v2-lexeme-morphology]
---
Continue the English v2 Stage 5 grammar buildout from the integrated Plan
03 terminal substrate through Plans 04–10. Authority
`docs/decisions/english-v2-rewrite.md`; the open declaration families and
immutable parser-environment authority established by Plan 03 are
preserved throughout.

The remaining roadmap is:

- Plan 04: generated invariants;
- Plan 05: structural declarations and the `OracleText` root;
- Plan 06: forms and selection;
- Plan 07: editorial and nominal grammar;
- Plan 08: ability and logic grammar;
- Plan 09: effect grammar;
- Plan 10: keyword and frame completion plus corpus closure.

Plans 07–10 consume the open grammar contributions of the keyword and
subtype registries; the stub-authoring tickets
(`builtin-v2-keyword-action-stubs`, `builtin-v2-keyword-ability-stubs`,
`builtin-v2-creature-type-stubs`) must land before the plans that read
them — Plan 10 at the latest.

The current locked baseline is 32,285 normalized corpus units: 48 are
selected, covered, and byte-exact, while 32,237 are ordinary parse
failures. Those figures are the starting point inherited from Plan 03, not
Stage 5 completion evidence.

Before implementing each remaining plan, author its detailed SDD plan
against the current parser and corpus evidence, and hold briefs immutable
once their task's execution starts — amendments are separate recorded
files, never post-hoc edits to a brief. Execute the plans serially, with
their required TDD, independent task reviews, completion audits, and
coverage-lock migrations. Every corpus-scale gate reports elapsed wall
time and compares it against a recorded baseline, failing loudly on
regression: a sudden slowdown is signal, and it must be observable to
agents in command output, not inferred from a hung terminal.

At each plan boundary, mint a scoped completion ticket for the finished
plan and integrate it — the Plan 03 recovery is the precedent — re-minting
this ticket's remainder when the lifecycle requires. Never hold more than
one plan's work un-integrated.

Acceptance: the coverage ratchet at its Plan 10 target — every normalized
Vintage corpus unit either covered (selected, byte-exact, totally owned)
or on the enumerated, reviewed quarantine list of corpus irregularities,
which may be empty and whose every entry is data, never a grammar special
case; no unresolved ambiguities; no internal, round-trip, or ownership
failures among selected units; all counted-list and performance gates
green; and a final independent review of the completed buildout before its
last integration. Standard constraints apply.
