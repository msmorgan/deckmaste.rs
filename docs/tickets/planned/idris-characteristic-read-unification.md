---
needs: []
---
**Idris grammar: unify the characteristic/stat READ side against the `Characteristic`
enum, mirroring the already-unified write side.** `[design]` — net-new read predicates;
confirm the shape before coding. From the 2026-06-26 grammar census. Affects
`idris/src/Core.idr`.

The WRITE side is one dependent constructor — `Set`/`Add`/`Alter : (c : Characteristic)
-> CharValue b c -> …`. The READ side is still a flat pile of bespoke predicates
(`HasType`/`HasSupertype`/`HasSubtype`/`HasColor`/`HasName`, plus `SameName`/
`SharesSubtype`), so the two halves of the same concept don't share a shape.

1. **`HasChar` / `SharesChar` parameterized on `Characteristic`.** Add
   `HasChar : (c : Characteristic) -> CharValue b c -> Predicate b AnObject` and
   `SharesChar : (c : Characteristic) -> Reference b AnObject -> Predicate b AnObject`,
   collapsing the flat read atoms the way `Set`/`Alter` collapsed the writes.
   `SharesColor`/`SharesType` (Intimidate, Radiance) then **fall out for free** — no new
   constructors. Keep the boundary from `idris-grammar-collapses` item 4: this is
   *parameterizing against `Characteristic`*, NOT adding a `Where`/`Subject` bridge
   (predicates stay candidate-implicit).

2. **`PlayerStatOf` + a `PlayerAttr` enum.** `LifeTotal`/`HandSize` are bespoke `Count`
   constructors while object stats go through `StatOf` + the stat enum. Fold them into
   `PlayerStatOf : Reference b APlayer -> PlayerAttr -> Count b` mirroring `StatOf`, with
   `PlayerAttr = Life | HandSize | …`. Pairs with the player-side `PlayerStatCmp`
   comparator in `idris-effects-costs-and-choices`.

*Serializes with the other `idris-*` grammar tickets — they all rewrite
`idris/src/Core.idr`, so only one can be in flight at a time. `needs:` is empty because
the blocking is file-level, not logical precedence.*
