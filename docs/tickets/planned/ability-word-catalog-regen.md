---
needs: []
design: true
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

## 2026-07-28 re-audit

`cargo xtask catalogs` is currently a byte-identical regeneration and still
does not add `Corrupted`. The generator's authority is CR 207.2c, and the local
CR snapshot lists 61 ability words without `Corrupted`; the Scryfall catalog
contains 69 entries and is intentionally not consulted for this generated
catalog (`crates/xtask/src/english/data.rs` loads only flavor words directly
from Scryfall). The raw/generated difference also includes several names beyond
`Corrupted`, so hand-adding the original witness would not repair the source
boundary.

This is therefore no longer a design-free regeneration ticket. It needs an
authority decision—update the CR snapshot when an authoritative version lists
the missing word, or deliberately union the broader Scryfall ability-word
catalog—before implementation and census attribution.
