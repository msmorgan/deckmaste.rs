---
needs: [gen-dynamic-count]
---
Parser hardening. `head_noun` (in `crates/deckmaste_migrations/src/parsers/filter.rs`)
ends with a fallback that turns ANY lone word into `Subtype("<word>")` without
validating it against the subtype catalog, and its singularization is naive
(`-ies→-y`, then trailing `-s`). This mints wrong atoms two ways:

1. Non-subtypes graduate as bogus subtypes — anaphors (`It`, `You`), object kinds
   (`Token`), and legend names (`Tivadar`, `Veldrane`).
2. Irregular plurals and `-s` singulars mis-derive: `Elves`→`Elve` (should be
   `Elf`), `Zombies`→`Zomby` (the `-ies→-y` rule misfires; should be `Zombie`),
   `Locus`→`Locu` (a singular land type wrongly stripped; should be `Locus`).

These currently graduate *wrong* (verified via a `generate` audit during
gen-dynamic-count: 8 distinct non-catalog `Subtype(...)` atoms in the graduated
corpus).

Fix: validate the lone-word head against the subtype catalogs — reuse the
`SUBTYPES`/`to_rust_ident` machinery already added for subtype ADJECTIVES in
gen-dynamic-count; do NOT duplicate `to_rust_ident`. Make the match plural-aware
(handle `-ves→-f`, distinguish `Zombie`+`s` from `Sorcer`+`ies`, leave singular
`-s` types like `Locus` alone) and emit the canonical catalog subtype. Decline a
lone word that's neither a known subtype nor a known designation (cf. the
commander handling) so a wrong filter never graduates a wrong card.

Verify with a `cargo xtask generate` delta: the de-graduated cards must all be
genuine non-subtypes, and `Elves`/`Zombies`/`Locus` references must now graduate
as the correct subtype. NOTE: distinct from the subtype-ADJECTIVE path ("Elf
creatures"), which is already catalog-gated; this is the lone-word HEAD fallback.
