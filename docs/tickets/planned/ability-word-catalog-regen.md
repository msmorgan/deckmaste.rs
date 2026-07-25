---
needs: []
---
**Regenerate the ability-word catalog the parser loads.** Diagnosed
2026-07-24: `Corrupted` is present in the raw Scryfall
`data/catalogs/ability-words.json` but missing from the generated
`data/gen/catalogs/ability-words.txt`, so the parser never sees it.
Regenerate, spot-check the diff for other stragglers, and re-run the english
census to attribute any movement. Scope note: Forecast/Exhaust/Boast are
keyword abilities, not ability words — their header cost:effect frame is an
unmodeled AST shape (currently peeled ungated by `peel_cost_flavor_header`,
by tested design) and is NOT this ticket.
