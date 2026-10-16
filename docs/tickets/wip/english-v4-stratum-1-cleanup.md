---
needs: [english-v4-stratum-1-core-clauses-and-nominals]
---
# Stratum 1 cleanup: spurious ambiguities and templating-policing productions

Clean up spurious derivation multipliers and over-policing productions left by
the initial Stratum 1 landing (`oxukttzv`) in `crates/deckmaste_english_v4` and
`crates/deckmaste_lexical_source_v4`:

1. **Duplicate `"target"` lexical entry:** Remove the leftover `"target"` entry
   from `adjectives.ron`. `target` is declared as a `Determinative` in
   `closed_class.ron` (`target creature`), a `Noun` in `nouns.ron` (`any
   target`, `two target creatures`), and a `Verb` in `verbs.ron` (`targets`).
   Keeping `adj:target` duplicates every nominal where `target` follows a
   determiner (`two target lands`, `another target creature`, `a second target
   creature`).
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
     exclude attributive-only `Determination = Overt` adjectives (`only`,
     `other`, `own`, `additional`, `extra`) so `Activate only` does not parse
     `adj:only` as a post-verbal depictive adjunct.
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
