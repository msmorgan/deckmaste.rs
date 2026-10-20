---
needs: []
---
# One Noun Phrase coordination schema per coordinator

Stratum 6 splits each Noun Phrase coordination schema by its conjuncts'
Determination and Countability, so an overtly determined first conjunct never
coordinates with a bare mass last one (*[target spell or permanent that's red]
or [green]*; Lush Growth's *[a Mountain], [Forest], and [Plains]*). The owner
accepted that as an exception: it takes the schemas from 9 to 17 in
`crates/deckmaste_english_v4/grammar/english.ron`.

## Change

- Express the restriction once, as a general mechanism, so each coordinator
  needs one schema for untargeted conjuncts and one for a targeted first
  conjunct, as before the split.
- A lexicon fix cannot do it: colour nouns stand bare legitimately (*choose
  red*, *protection from red*).
- Do not leave `Determination` and `Countability` unset on a coordinated
  `NounPhrase` (or require `Carry` rather than `Require` / `Exclude` in
  `PredeterminerCount`, `PredeterminerMass`, `MultipliedNounPhrase`,
  `MultiplierPredetermination`, and `MeasuredComparativeHead`). Today both
  features sit in `unshared` on every `NounPhrase` coordination schema, so
  `Require("head.Determination", "Overt")` succeeds on bare coordinated Noun
  Phrases—giving every *all X and Y* (*Destroy all artifacts and enchantments.*)
  and *both X and Y* in the corpus a spurious third derivation through
  `PredeterminerCount` (`all [[NP artifacts] and [NP enchantments]]`) and
  admitting `*double creatures and planeswalkers` and `*creatures and
  planeswalkers more`.

## Done when

- The Noun Phrase coordination schemas are back to 9, or fewer.
- `*NounPhrase double creatures and planeswalkers` and
  `*NounPhrase creatures and planeswalkers more` hold in judgments, and
  *all artifacts and enchantments* has no `PredeterminerCount` derivation over
  bare coordinated Noun Phrases.
- No unit gains a derivation and no face is lost against the census before
  the change; Lush Growth stays unread (deferred) until a correct reading
  exists.

Standard constraints apply.

Census delta (`kqzxkkzp`): faces read 28,705 → 28,705; total Readings 384,270 → 382,567 (86 units fewer, none more: predeterminers over coordinated Noun Phrases not all determined, and duplicate arithmetic operands over an unvalued coordinated Determination); lost faces: none. Lush Growth stays deferred; strata 0–6 unchanged, 0 known-wrong-only, 0 law failures.
Judgment set (`kqzxkkzp`): 3,016 enforced, 189 pending, all hold; tests and clippy pass for deckmaste_syntax and deckmaste_english_v4.
Done-when restated (coordinator ruling): `*NounPhrase double creatures and planeswalkers` cannot hold, since attributive *double* (*double damage*) is ordinary English and keeps its adjective + noun Readings; the judgment is `+NounPhrase double creatures and planeswalkers => !NounPhrase "creatures and planeswalkers"`, excluding only the multiplier over a bare coordination.
