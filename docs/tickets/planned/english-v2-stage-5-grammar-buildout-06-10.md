---
needs: [english-v2-plan-05-structural-declarations]
---
Continue the English v2 Stage 5 grammar buildout from the integrated Plan 05
structural declaration substrate through Plans 06–10. Authority remains
`docs/decisions/english-v2-rewrite.md`; preserve the generated declaration
authority, open declaration families, immutable parser environment,
`OracleText` document root, structural ownership, and separator/terminator
algebra established by Plans 03–05.

The remaining roadmap is:

- Plan 06: forms and selection;
- Plan 07: editorial and nominal grammar;
- Plan 08: ability and logic grammar;
- Plan 09: effect grammar;
- Plan 10: keyword and frame completion plus corpus closure.

Plans 07–10 consume the open grammar contributions of the keyword and subtype
registries; the stub-authoring tickets (`builtin-v2-keyword-action-stubs`,
`builtin-v2-keyword-ability-stubs`, `builtin-v2-creature-type-stubs`) must land
before the plans that read them — Plan 10 at the latest.

The locked Plan 05 corpus baseline is 32,285 normalized units: 56 are selected,
covered, byte-exact, and totally owned, while 32,229 are ordinary parse
failures. The elapsed-time ceiling remains 16.26 seconds for each of the seven
corpus-scale gates. The Plan 05 measured samples, used for the next plan's named
1.5x WARNING thresholds, are: expand 2.75s, report 0.15s, parse 7.50s,
roundtrip 7.12s, ambiguity 7.87s, coverage 7.96s, and require-complete 7.16s.
Those figures are the starting point, not Stage 5 completion evidence.

Before implementing each remaining plan, author and obtain approval of its
detailed written specification and SDD plan against current parser and corpus
evidence. Hold briefs immutable once execution starts; amendments are separate
recorded files. Execute plans serially with TDD, independent task reviews,
completion audits, coverage-lock migrations, and elapsed/baseline performance
gates whose results are visible in command output.

At each plan boundary, mint a scoped completion ticket for the finished plan
and integrate it, re-minting this ticket's remainder when the lifecycle
requires. Never hold more than one plan's work unintegrated.

Acceptance is the Plan 10 coverage ratchet: every normalized Vintage corpus
unit is either covered (selected, byte-exact, totally owned) or on an
enumerated, reviewed quarantine list of corpus irregularities, which may be
empty and whose every entry is data rather than a grammar special case; no
unresolved ambiguities; no internal, round-trip, or ownership failures among
selected units; every counted-list and performance gate green; and a final
independent review before the last integration. Standard constraints apply.
