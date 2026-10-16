---
needs: [english-v4-stratum-1-core-clauses-and-nominals]
---
# Stratum 1 cleanup: spurious ambiguities and templating-policing productions

Clean up spurious derivation multipliers and over-policing productions left by
the initial Stratum 1 landing (`oxukttzv`) in `crates/deckmaste_english_v4` and
`crates/deckmaste_lexical_source_v4`:

1. **Targeting Marker vs Target Noun (owner ruling):** Preserve the glossary
   distinction. Keep `adj:target` with `determination: Some(Overt)` for
   `two target creatures` and `another target creature`; prevent `noun:target`
   from duplicating the marker as a premodifier. Keep the count noun in
   `any target` and the determinative in `target creature`.
2. **Preposition `Nominal` complement scope (`prepositions.ron` / `english.ron`):**
   Prepositions licensing a bare singular count-noun complement (`until end of
   turn`, `on top of`, `at end of combat`, `in hand`, `in addition to`, `as
   part of`) must not admit arbitrary compound nominals (`on [Nominal target
   creature]`) or bare plurals/mass nouns that already parse as `NP`. Restrict
   preposition `Nominal` complements so `on target creature` parses only as
   `on [NP target creature]`.
3. **Signed slash-measure duplication (`SignedAmount`):** In `-1/-1`, `-1`
   tokenizes both as a signed `Numeral { value: -1 }` (matching
   `UnsignedMeasureAmount`) and as `Mark('-', Prefix)` + `Numeral { value: 1 }`
   (matching `NegativeMeasureAmount`), producing $2 \times 2 = 4$ duplicate
   `SlashMeasure` trees on every `-1/-1 counter`. Restrict `UnsignedMeasureAmount`
   (or `SignedAmount`) so negative slash-measure numerals parse once.
4. **Focused PP / NP and depictive adjective ambiguity (`Activate only as a sorcery.`):**
   - `DepictiveAdjective` (`VerbPhrase -> VerbPhrase + AdjectivePhrase`) must
     exclude attributive-only adjectives (`only`, `other`, `own`, `additional`,
     `extra`) so `Activate only` does not parse `adj:only` as a post-verbal
     depictive adjunct. Attributive position is independent of overt
     determination: bare plural `other types` must remain grammatical.
   - `AdverbModifiedDeterminerPhrase` (`AdverbPhrase + DeterminerPhrase`) must
     apply only to `Numerical` determiners (`exactly seven`, `exactly 20`)
     rather than duplicating `AdverbModifiedNounPhrase` on every `Determined`
     `NounPhrase`.
   - `adv:as` (the degree modifier of scalar equality in `twice as much`) must
     not combine with arbitrary `NounPhrase`s via `AdverbModifiedNounPhrase` to
     turn `as a sorcery` into a `NounPhrase` direct object of `activate`.
5. **Bare numeral `NumericNounPhrase` vs `Amount` and `Ditransitive` frames:**
   `NumericNounPhrase` (`NounPhrase -> Amount`, used for bare numeral/variable
   NPs like `on X` or `Choose 2`) causes `Scry 1.` to parse twice (`Amount`
   frame vs `Monotransitive` frame) and causes `This creature deals 3 damage to
   any target.` to parse `3` as the `Ditransitive` recipient NP (`deals [3]
   [damage to any target]`). Restrict bare numeral/variable NPs or frame
   selection so `Scry 1.` and `deals 3 damage to any target` do not admit those
   spurious readings, restore `; readings 1` on `+VerbPhrase deals 3 damage` in
   `judgments/measured_damage_nominal.txt` (which `oxukttzv` weakened), and
   remove the corresponding rationalization in `judgments/README.md` (`"The
   intended deals 3 damage Object remains required without an exact count..."`).
6. **Mode-bullet `NominalMode` vs imperative `ModeBullet`:** `NominalMode` (`•
   Nominal .`, for `• 1/3 Turtle with hexproof.`) currently parses imperative
   mode bullets like `• Exile target creature.` and `• Return target creature
   card from your graveyard to your hand.` as multi-noun compounds (`[noun:exile
   [noun:target creature]]`, `[name:Exile [noun:target creature]]`). Restrict
   `NominalMode` / nominal premodification so imperative mode bullets do not
   parse as bare noun-compound fragments.
7. **Remove `AdditiveQuantity` / `PeriodicInterval` policing block:** Replace
   the 5 single-purpose categories (`AdditiveModifier`, `IndefiniteArticle`,
   `PeriodicDeterminer`, `AdditiveDeterminer`, `GroupedMeasure`), 3 closed
   words (`Article`, `Every`, `Another`), `additive: bool` on `AdjectiveEntry`,
   and bespoke productions in `english.ron` with general `QuantifiedNominal` /
   `DeterminedQuantity` handling, drop the over-policing negative judgments
   (`* every two creatures`, `* each two turns`) from `genitive_agreement.txt`,
   and remove the corresponding sentence in `judgments/README.md` (`"Additive
   sequences such as an additional two creatures and singular temporal intervals
   such as every two turns are separate constructions..."`).

Quantity determination preserves internal numeral–noun agreement separately
from external agreement. Distributive groups select singular agreement;
`another two creatures` retains plural agreement. The indefinite article
selects modified quantities, while ordinary determiners keep Number and
Countability selection. These are general grammatical distributions, without
an additive adjective class or temporal-only grouping.

## Gate

- 100% of corpus units in Strata 0–1 parse as **read** (0 `known-wrong-only`,
  0 `unread`, 0 law failures).
- Cumulative Strata 0–1 and seed judgments pass with zero regressions.
- `Scry 1.`, `Destroy two target lands.`, `Activate only as a sorcery.`, and
  `• Exile target creature.` each have 1 derivation; `Put a -1/-1 counter on
  target creature.` has no duplicate `SlashMeasure` or `on [Nominal target
  creature]` derivations; `This creature deals 3 damage to any target.` has no
  `Ditransitive` recipient-`3` derivation.

Standard constraints apply.

Census delta (`qsropvym` → `tzouxypq`): faces read 8,233 → 8,219; total Readings 102,767 → 81,320; lost faces (later-stratum wrong analyses removed): Conflagrate; Explosive Crystal; Fiery Justice; Hail of Arrows; Kjeldoran Pride; Lita, Mechanical Engineer; Master of the Wild Hunt; Pyrotechnics; Rolling Thunder; Sidisi, Regent of the Mire; Spreading Flames; That Which Was Taken; Violent Eruption; Volley of Boulders. Strata 0–1: 56,490/56,490 units read, 0 known-wrong-only, 0 unread, 0 law failures.
Judgment set (`tzouxypq`): seed and cumulative Strata 0–1 pass with zero regressions; focused cleanup, numeric-subject/predicative, and quantity-agreement judgments pass.
