---
needs: []
---
# Collapse the possessive and genitive `nominal_form` splits

**R10 — Group R.** Authority: rewrite ADR "Plan 08 unified-membership
clarification (2026-09-04)" (the re-spell is the unified shape's replacement for
a deleted Category) read together with "Amendment: attachment class is a declared
linguistic property (2026-09-04)" (the value list still has to answer to
English).

Defect. One linguistic frame — possessive determiner plus nominal — is seven
constructions. `possessed_singular_reference`, `possessed_plural_reference` and
`possessed_mass_reference`
(`crates/deckmaste_english_v2/src/constructions.rs:3186, 3201, 3216`) have
**identical forms** (`lex(possessor) nominal`) and identical derive lists,
differing only in a `require nominal.nominal_form in [...]`. The four
`genitive_determiner_*` constructions (`:3230, 3245, 3260, 3274`) are the same
frame split four ways.

`NominalForm` has seven values
(`crates/deckmaste_construction_core/src/feature.rs`: `BareSingularNoun,
ModifiedSingularNoun, SingularCoordination, BarePluralNoun, ModifiedPluralNoun,
PluralCoordination, MassNoun`). Eleven `nominal_form in [...]` requires each
select exactly two of the seven, and the partition is not exhaustive:

- the `possessed_*` family has **no coordination arm at all** — `your creature
  and artifact` has no derivation;
- `genitive_determiner_coordination_reference` requires
  `coordination.number is Singular`, so `target player's creatures and
  artifacts` has none either.

The recorded justification for the excluded values is provenance about a deleted
implementation artifact ("`SingularNominal` produced `BareSingularNoun` and
`ModifiedSingularNoun`… all thirteen former narrow-Category roles carry exactly
those lists"), not an English fact. The one English-facing datum is negative and
site-specific: widening to admit coordination broke `Aquatic Alchemist // Bubble
Up` (`your first [instant or sorcery] spell` became `your [first instant] or
[sorcery spell]`) in the `english-v2-number-feature-unification` landing —
evidence about one role, generalized to thirteen.

Pinned shape. One construction per frame, with the determinative's declared
`nominal_license` doing the licensing through the existing
`determiner_licenses_nominal` checker (`:3310`), which already performs exactly
this job elsewhere in the same file. Where a site genuinely must exclude a value,
state the English fact for that site. Re-examine `demonstrative_possessive_
reference`'s `require possessor.number is Singular` in the same pass: it is a
single construction, so the requirement excludes plural possessors rather than
partitioning a domain — the `english-v2-genitive-determiner-collapse` landing
diagnosed it and minted no ticket; this is that ticket.

Landing this unblocks `english-v2-genitive-determiner-collapse`'s own pinned
single construction, which that landing could not deliver ("Those requirements
are load-bearing, not inert, so the three branches stay distinct").

Fences. Re-spelling a category as a value list without an English reason per
site. Widening every list at once without reading the `Aquatic Alchemist` shape
— report that unit's selected analysis explicitly. A `checked by` naming a
construction.

Glossary: Nominal, Nominal Form, Determinative, Determiner, Genitive,
Possessive, Coordination. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` (388 constructions, 17,052 / 32,641
covered; 11 `nominal_form in [...]` sites) — re-measure at claim. Standard
constraints apply.
