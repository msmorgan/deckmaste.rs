---
needs: [english-v2-plan-07-editorial-and-nominal-grammar]
---
Continue the English v2 Stage 5 grammar buildout from the integrated Plan 07
editorial-and-nominal substrate through Plans 08–10. Authority remains
`docs/decisions/english-v2-rewrite.md`; preserve generated declaration
authority, the immutable parser environment, opaque full card-name identity,
legendary-only shortened self-reference, the `OracleText` document root,
structural ownership, and selection without silent dominance.

The remaining roadmap is:

- Plan 08: ability and logic grammar;
- Plan 09: effect grammar;
- Plan 10: keyword and frame completion plus corpus closure.

The locked Plan 07 corpus baseline is 32,641 normalized units: 600 are
selected, covered, byte-exact, and totally owned, while 32,041 are ordinary
parse failures. The elapsed-time hard ceiling remains 16.26 seconds for each
of the seven corpus-scale gates. Derived at exactly 1.5× the fresh Plan 07
samples, Plan 08 WARNING thresholds are:

- expand: 1.108501506s;
- report: 0.7404841905s;
- parse: 18.588990480s;
- roundtrip: 17.047222473s;
- ambiguity: 18.7905469695s;
- coverage: 19.1157010635s;
- require-complete: 16.703354229s.

The warning thresholds do not relax the 16.26-second hard ceiling. Before
implementing each remaining plan, author and obtain approval of its detailed
written specification and SDD plan against current parser and corpus evidence.
Execute the plans serially with literal RED/GREEN TDD, independent task
reviews, completion audits, add-only coverage-lock migrations, and elapsed
performance gates whose samples and named warnings remain visible.

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
