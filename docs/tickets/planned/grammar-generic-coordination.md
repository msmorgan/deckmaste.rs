---
needs: [grammar-compiler-expressions-and-tables]
---
# Generic coordination with per-feature combiners

Coordination is enumerated per category today: 51 `*Series` and `Correlative*` categories hold 295 of 714 interface entries (41%); 257 of 415 schema instances (62%) are coordination rows. Replace this with one declaration and per-feature combiners, generating the same construction and category names so readings do not change.

## Change

- `coordinable NounPhrase, Nominal, ...;` (exact spelling to be fixed in the landing). The compiler generates the binary, serial and correlative forms and the series helper categories.
- Each feature declares how it combines across conjuncts: `all`, `any`, `same`, or a named table. Real logic such as `determined_np_coordinate_number` stays a named table; the plain AND and OR tables are covered by `all` and `any`. Settle first, with a script over the tables used by series instances, how many concord tables are uniform.
- Delete the per-category series and correlative declarations, schemas and instance rows the generator replaces.

## Gate

Every `readings_sha256` identical on every face. Three-line note: census delta (none); size delta per declaration kind (ratchet lowered); judgment-set result.
