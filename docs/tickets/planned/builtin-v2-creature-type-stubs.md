---
needs: [builtin-v2-keyword-action-stubs, builtin-v2-keyword-ability-stubs]
---
Author and commit the creature-type records under
`plugins/builtin_v2/macros/stubs/creature_types`, one for each canonical
`data/gen/catalogs/creature-types.txt` entry, following the ratified
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

These files are source code and graduate in place; do not generate them. Verify
number morphology exhaustively against authoritative grammar evidence—catalog
membership is not evidence that `+s` is valid (`Merfolk` is plural
`Merfolk`, not `Merfolks`). Do not guess unattested forms.

After all three committed inventories exist, add a read-only completeness gate
that checks the catalog-line ↔ filename bijection. It must never write, replace,
or byte-compare authored stub contents. Standard constraints apply.
