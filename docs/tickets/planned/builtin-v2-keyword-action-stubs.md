---
needs: [builtin-v2-grammar-consumer]
---
Author and commit the keyword-action records under
`plugins/builtin_v2/macros/stubs/keyword_actions`, one for each canonical
`data/gen/catalogs/keyword-actions.txt` entry, following the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

Each record has the catalog identity as `name`, the uninflected literal action
head as `spelling`, and only the lexical or fixed-surface facts needed to admit
that action as `grammar`. Do not encode valency or argument legality there:
parameter types and `<Param(n)>` holes arrive when the record graduates into a
semantic macro. Compound actions store whole attested inflected surfaces; they
never store a head offset. Clause-shaped actions such as `TheRingTemptsYou`
use the fixed clause recipe rather than being forced into a verb record.

Check capitalization, punctuation, and every supplied inflection against the
CR and supported Oracle corpus, following style guide §14, "Keyword actions in
sentences." Catalog title case is identity evidence only. If a form has no
authoritative attestation, omit it and leave a targeted note in the record;
never guess. Unsupported semantic actions still receive nursery records: this
ticket inventories spellings and grammar facts, not completed macro bodies.

The records are committed source code and graduate in place. Do not generate
them, add a generator, copy v1 `template:`/`kinds:` shapes, or add one core
English construction per action. Acceptance includes representative audited
records for a simple verb, a compound verb, a punctuation/casing exception,
and the fixed-clause family; every record loads through the v2 declaration
reader, and filename stems and `name` values obey the ADR's mapping. Standard
constraints apply.
