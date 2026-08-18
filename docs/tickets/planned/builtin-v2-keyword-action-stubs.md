---
needs: [builtin-v2-spelling-stub-design]
---
Author and commit the keyword-action records under
`plugins/builtin_v2/macros/stubs/keyword_actions`, one for each canonical
`data/gen/catalogs/keyword-actions.txt` entry, following the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

These files are source code and graduate in place; do not generate them. Check
every lexical, casing, clause, and inflection fact against the authoritative
Oracle/CR grammar sources rather than inferring it from catalog title case.
Standard constraints apply.
