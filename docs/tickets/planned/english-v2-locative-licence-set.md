---
needs: [english-v2-attachment-class-declared]
---
# `LocativeTemporalLicense` becomes a licence set, and `PrepositionComplementKind` stops naming words

**R11 — Group R.** The mechanism finding of the 2026-09-04 fallout audit; R9 and
much of the licence matrix dissolve into it.

Defect, two halves of one table.

1. **The noun side is an enumerated powerset of attested combinations.**
   `LocativeTemporalLicense`
   (`crates/deckmaste_construction_core/src/feature.rs:184`) has ten values:
   `Unlicensed, OfLicensed, OfAndOnLicensed, OfInAndOnLicensed, InLicensed,
   OnLicensed, InOrOnEdgeLicensed, ObjectAttachmentLicensed, TemporalLicensed,
   OfAndTemporalLicensed`. These are not classes; they are the combinations some
   noun turned out to need, fused into one enum. There is no `OfAndInLicensed`,
   no `InAndTemporalLicensed`, no `OnAndTemporalLicensed` — a noun licensing `of`
   and `in` but not `on` **has no value it can declare**, and must be
   mis-declared as `OfInAndOnLicensed`, silently gaining `on`. In the other
   direction the fusion over-generates: `nominal_preposition_is_licensed`
   (`crates/deckmaste_english_v2/src/constructions.rs:4991`) computes
   `accepts_surface` true for `TemporalLicensed | OfAndTemporalLicensed`, so
   `Beginning`, `End` and `Phase` license `on` and *"on the beginning of your
   upkeep"* is admitted. The default is refuse-until-attested: 43 declared rows
   carry `Unlicensed`.
2. **The preposition side names words.** `PrepositionComplementKind`
   (`feature.rs:175`) has `InComplement` and `OnComplement` — the identity of
   `in` and `on` wearing a feature's clothes — beside genuine classes
   (`UnrestrictedComplement, RelationalComplement, SelectionComplement,
   SourceComplement, TemporalComplement`). Every checker that reads it
   (`:4903, 4991, 5133, 5159`) is therefore a `match` over individual
   prepositions crossed with the attested noun combinations. The 2026-09-02
   amendment's criterion "No preposition or noun is named in any construction"
   holds by the letter and not in substance, and the forbidden-checker gate
   cannot see it because the checker does read a declared feature.

Also here: `nominal_preposition_is_licensed` returns `false` for
`UnrestrictedComplement` outright, so `to`, `for`, `into`, `onto` and `under` can
never postmodify any noun — `It becomes a Bear in addition to its other types.`
fails on that line. Whether each of those five may postmodify is a per-word
English question, answered on the preposition's row after R1.

Pinned shape. One declared value per licence dimension, not per observed
combination: a noun declares the set of preposition classes it licenses, the
preposition declares its class, and admissibility is set membership. Fold
`InComplement` and `OnComplement` into the class axis they belong to (interior /
surface), so no feature value is a word's name. The two sides are one change —
do not land half.

Fences. A wider fused enum. A checker that special-cases a value. Declaring a
noun's licence from its corpus witnesses rather than from what the noun means —
the census is provenance in the record. Adding a value only when a card needs it.

Glossary: Licence, Preposition, Complement, Postmodifier, Locative, Temporal,
Relational, Feature. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` (388 constructions, 17,052 / 32,641
covered; 10 licence values, 43 `Unlicensed` rows) — re-measure at claim. Report
every changed selected analysis; this landing moves the attachment census.
Standard constraints apply.
