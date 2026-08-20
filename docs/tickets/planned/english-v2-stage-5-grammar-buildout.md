---
needs: [english-v2-open-grammar-environment]
---
Continue the English v2 Stage 5 grammar buildout from the reviewed and
integrated Plan 03 terminal substrate through Plans 04–10. When work begins,
keep it in the single `stage-5-grammar-buildout` Kata workspace for the
lifetime of this ticket.

The remaining roadmap is:

- Plan 04: generated invariants;
- Plan 05: structural declarations and the `OracleText` root;
- Plan 06: forms and selection;
- Plan 07: editorial and nominal grammar;
- Plan 08: ability and logic grammar;
- Plan 09: effect grammar;
- Plan 10: keyword and frame completion plus corpus closure.

The current locked baseline is 32,285 normalized corpus units: 48 are
selected, covered, and byte-exact, while 32,237 are ordinary parse failures.
Those figures are the starting point inherited from Plan 03, not Stage 5
completion evidence.

Before implementing each remaining plan, author its detailed Superpowers SDD
plan against the current parser and corpus evidence. Execute the plans
serially, with their required TDD, independent task reviews, completion audits,
coverage-lock migrations, and performance gates. Preserve the open declaration
families and immutable parser-environment authority established by Plan 03.

Do not mark this ticket done, integrate it, or retire its workspace at an
intermediate plan boundary. A plan may establish a reviewed checkpoint without
closing the encompassing Stage 5 buildout.

Acceptance requires 100% exact parsing and rendering of the normalized Vintage
corpus, complete lexical ownership for every selected derivation, no unresolved
ambiguities, and no internal, round-trip, or ownership failures. The final
coverage report and lock must contain no uncovered corpus units, all promised
count and performance gates must pass, and the completed buildout must receive
its final independent review before Kata integration and workspace retirement.
