---
needs: [builtin-v2-grammar-consumer]
---
Author and commit the keyword-ability records under
`plugins/builtin_v2/macros/stubs/keyword_abilities`, one for each canonical
`data/gen/catalogs/keyword-abilities.txt` entry, following the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

Each nursery record has the catalog identity as `name`, the literal keyword
head as `spelling`, and only the fixed keyword-line or running-text facts the
English parser needs as `grammar`. The stub does not guess a parameter list or
semantic expansion. When the macro graduates, its positional parameter types
and `<Param(n)>` holes extend this same record; no parallel completed macro is
created.

Audit line-initial versus running-text capitalization, punctuation-bearing
names such as `For Mirrodin!`, multiword names, and parameter-bearing families
against the CR and supported Oracle corpus, following style guide §14,
"Keyword abilities." Preserve approved whole surfaces instead of deriving
them by lowercasing the catalog heading. An unattested form is omitted, not
guessed. Ability words are outside this inventory: their labels do not select
semantic keyword macros [CR#207.2c].

The files are committed source code and graduate in place. Do not generate
them, add a generator, copy v1 `template:`/`kinds:` shapes, or add one core
English construction per keyword. Acceptance includes representative audited
nullary, parameter-bearing, multiword, punctuation-bearing, and running-text
records; every record loads through the v2 declaration reader, and filename
stems and `name` values obey the ADR's mapping. Standard constraints apply.
