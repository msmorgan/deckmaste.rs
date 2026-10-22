---
needs: []
---
# Rank the frequency reading of *no more than three times each turn* ahead of the Direct Object

**Re-scoped by the owner's round-8 treatment plan** from an admission
exclusion to a ranking ticket. The intended Reading is not in the cheapest
tier, so the ticket is not retired.

## Evidence

Measured on `qynmxmro` with `cargo xtask english-v4 parse`.
*Activate no more than three times each turn.* has 5 Derivations and a
cheapest tier of 1:

- **Reading 1, `0/1/16` (cheapest): wrong.** *no more than three times each
  turn* is the Object of *activate* (`NonNumericComplement` over
  `ComparativeQuantityCountDetermined`). *each turn* postmodifies *times*
  (`TemporalPostmodifier`), and that short host earns it Right Association 1.
- **Reading 2, `0/6/14`: endorsed.** A `FrequencyInterval` Adjunct, with *three
  times* the Complement of *than* (`SerialTemporal`) and *each turn* inside the
  interval.
- **Reading 3, `0/6/16`: wrong.** Object *no more than three times*, with *each
  turn* on the Verb Phrase.
- **Reading 4, `0/7/15`: endorsed.** A `ComparativeAdverb` Adjunct, with *each
  turn* on the Verb Phrase.

`no_more_than.txt:13` asserts *three times* as the Complement of *than*, so its
cheapest tier is not all-endorsed. *Activate no more than twice each turn.*
has no Object Reading (4 Derivations, cheapest tier 1 at `0/1/12`).

## Rulings

- Round 4: comparative and modified frequency Noun Phrases (*no more than three
  times*, *at least twice*) are temporal or frequency Adjuncts, never Direct
  Objects in `NonNumericComplement`.
- Round 7, item 1: approved Option A, an admission exclusion
  (`ExcludeTogether([("head.NounKind","Temporal"),("head.NounDistribution","Serial")])`
  on `ComparativeQuantityCountDetermined` and `ComparativeQuantityMassDetermined`).
  It also approved an equative-multiplier ticket.
- Round 8 (`docs/decisions/english-v4.md` §4.4): admission stops only Readings
  that no English grammar admits, and exclusions that exist to suppress a
  dispreferred Reading are frozen. `english-v4-single-holder-feature-values`
  may fold `Serial`, which Option A names.

## Change

1. Find why the Object Reading is cheapest, and look for a purely structural
   Ranking Key fix (§4.3) that ranks an endorsed Reading first. Do not use a
   word or a lexical preference.
2. If no structural fix exists, STOP and return to the owner with both
   rulings: round 4 says this Object is ungrammatical in the Oracle register,
   so Option A would be admission; round 8 allows admission only for what no
   English grammar admits. Do not choose between them yourself.

## Done when

- Every Reading in the cheapest tier of *Activate no more than three times each
  turn.* is endorsed, so `no_more_than.txt:13` has an all-endorsed cheapest
  tier.
- No face is lost.
- Every judgment holds.
- `cargo xtask english-v4 strata --check` passes.
- The landing reports the cheapest tier: ambiguous units with a tier of one,
  the tiers' sum and the largest tier, before and after. No judgment-set entry
  goes from an all-endorsed cheapest tier to one that is not.
- No new `Summary`, `State` or `ProductionDecl` field, and no new `closure.rs`
  pass.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
