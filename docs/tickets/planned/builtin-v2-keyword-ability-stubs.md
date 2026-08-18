---
needs: [builtin-v2-spelling-stub-design]
---
Author and commit the keyword-ability records under
`plugins/builtin_v2/macros/stubs/keyword_abilities`, one for each canonical
`data/gen/catalogs/keyword-abilities.txt` entry, following the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

These files are source code and graduate in place; do not generate them. Check
running-text capitalization, punctuation, and any parameter-bearing surface
against the authoritative Oracle/CR grammar sources rather than mechanically
lowercasing catalog headings. Standard constraints apply.
