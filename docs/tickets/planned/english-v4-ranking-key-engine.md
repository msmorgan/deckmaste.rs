---
needs: [english-v4-comma-adjunct-island, english-v4-relational-noun-of-complement, english-v4-fronted-temporal-pp-clause, english-v4-bare-relative-fronted-adjunct, english-v4-cost-not-passivizable, english-v4-keyword-phrase-adjunct-island, english-v4-auxiliary-adjunct-canonicalisation]
---
# Rank Readings by a lexicographic Ranking Key

`docs/decisions/english-v4.md` §4.3 replaces the scalar sum of
Production Cost with the **Ranking Key**, compared lexicographically, lower
first:

AttachmentClass → CatalogPreference → RightAssociationWords → ProductionCost → FinalTiebreak

This ticket builds the tuple and its extraction. Its components are filled in
by `english-v4-right-association-key`, `english-v4-attachment-classes`
and `english-v4-catalog-attachment-preferences`.

## Why now, and why after the defects

Owner's ruling: sequence the redesign after the spurious-reading defects, so
the ranking needs no workarounds for:
- island violations;
- a passive *cost*;
- bare relatives with fronted adjuncts.

Today's sum leaves most alternatives tied. The cost study measured:
- 6,039 of 17,919 ambiguous units (33.7%) with a single cheapest Reading;
- Mockingbird with 6,354 tied at the minimum;
- 94 of the 310 single-group judgment entries (2–4,000 Readings, some
  unendorsed) with an all-endorsed top tier.

## Change

- In `deckmaste_syntax`, a Reading's rank is a tuple, not a `u64`.
  - `best_readings(k)` keeps each forest node's *k* best by that tuple.
  - Every component is a sum over Productions or attachments, so dynamic
    programming over the Packed Forest stays compositional; no trees are
    enumerated.
  - Expose a way to compute a given Reading's key, for the judgment tooling.
- **ProductionCost**: today's per-Production `cost`, unchanged.
- **FinalTiebreak**: a deterministic total order on Readings, so that the
  cheapest is a single Reading. The existing `Reading` order will do.
- The three middle components exist and are zero until their tickets land.
- The census reports, per unit:
  - the size of the cheapest tier before the FinalTiebreak;
  - the number of units whose cheapest tier is a singleton.
- `parse` prints each Reading's key.
- Admission and Reading counts do not change: the key is a Preference.
- Update `crates/deckmaste_syntax/README.md` and
  `crates/deckmaste_english_v4/README.md`.

## Done when

- With the middle components zero, the ranking equals today's order, broken
  by the FinalTiebreak.
- Total Readings and Surplus are unchanged, and no face is lost.
- A seed-lexicon test shows that a higher component outranks any amount of a
  lower one.

Standard constraints apply.

1. **Census delta**: measured at landing (Readings and Surplus unchanged).
2. **Judgment set**: measured at landing.
