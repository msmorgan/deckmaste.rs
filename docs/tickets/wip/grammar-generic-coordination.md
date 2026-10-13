---
needs: [grammar-dead-declarations-and-requirements]
---
# Generic coordination with per-feature combiners

Coordination is enumerated per category today: 51 `*Series` and `Correlative*` categories hold 295 of 714 interface entries (41%); 257 of 415 schema instances (62%) are coordination rows. Replace this with one declaration and per-feature combiners, generating the same construction and category names so readings do not change.

## Change

- `coordinable NounPhrase, Nominal, ...;` The compiler generates the binary, serial and correlative forms and the series helper categories. The exact surface syntax is settled in the landing; the requirements below are not.
- This is elaboration to today's declarations. The generated constructors must reproduce today's constructor shape, field names and order, form indices and optional `category` field exactly, because the census hashes the complete Reading `Debug` output (`crates/xtask/src/english_v3/validation.rs:31`) and the generated values carry `form`, `category` and ordered fields (`crates/deckmaste_construction_v3_core/src/emit.rs:692–711`). Admission must be unchanged as well; matching names alone does not preserve digests.
- Each feature declares how it combines across conjuncts: `all`, `any`, `same`, or a named table. Combiners allow per-category and per-context overrides, because behaviour is category-sensitive today: `number` combines through `cardinal_coordinate_number` for Cardinals (`CardinalAgreement`, `crates/deckmaste_english_v3/src/declarations.rs:1657`) but through `determined_np_coordinate_number` for determined noun phrases (`declarations.rs:1890`), and finite predicates require agreement (`FiniteConcord`, `:1690`). They allow a table to take arguments from several features, and they allow asymmetric restrictions such as `require Right.CliticHost = Free` (`:1698`). The coordination instance rows carry different distribution and boundary policies (`:3778`); each category keeps the forms (binary, serial, correlative) it has today and no others.
- Export-computing AND and OR tables left by `grammar-compiler-expressions-and-tables` are replaced here by `all` and `any`. Settle first, with a script over the tables used by series instances, how many concord tables are uniform; real logic such as `determined_np_coordinate_number` stays a named table.
- Delete the per-category series and correlative declarations, schemas and instance rows the generator replaces.
- The specification includes an inventory of the existing tests that construct coordination readings by hand (for example `crates/deckmaste_english_v3/tests/suite/correlatives.rs`, `correlative_adjectives.rs`, `correlative_finite_complements.rs`) and keeps each, re-spelled if its constructor must change, for the same witnesses; tests are never deleted to pass, and the landing reports restored, re-spelled, added and removed counts.

## Gate

Every `readings_sha256` identical on every face, checked with the census check mode as an identical-digest landing. The landing updates `crates/deckmaste_construction_v3_core/README.md`. Note: census delta (none); the judgment-set line does not apply yet.
