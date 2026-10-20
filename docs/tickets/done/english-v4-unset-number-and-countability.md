---
needs: []
---
# Bind `Number` and `Countability` on `ProperName`, `[Count, Mass]` nouns, `det:everything`, and `the most`

In `deckmaste_syntax` (`crates/deckmaste_syntax/src/step.rs:721–740`),
`Require` and `Agree` succeed whenever a child's feature is `UNSET` (`None`).
Four places in `deckmaste_english_v4`, `deckmaste_lexical_source_v4`, and
`deckmaste_syntax` leave `Number` or `Countability` unset where it should be
bound, causing duplicate derivations and agreement leaks:

1. **`ProperName` (`crates/deckmaste_english_v4/grammar/english.ron:924`):**
   `Name` has no features and `ProperName` sets only `NounKind`,
   `Determination`, `Case`, and `Person`, leaving `Number` and `Countability`
   unset on card names, `CARDNAME`, shortened names, and `The Ring`. Every
   `ProperName` as the final conjunct of an `and/or` coordination matches both
   the singular and plural `AndOr` schemas (`english.ron:1219–1226`, doubling
   derivations), bypasses subject–verb agreement (`*CARDNAME attack.`,
   `*CARDNAME are tapped.`), and matches `MeasuredComparativeHead` (`*CARDNAME
   more`).
2. **Dual-countability `[Count, Mass]` nouns (`crates/deckmaste_syntax/src/compile.rs:375–378` and `nouns.ron`):**
   `compile.rs` populates `facts.countability` only when
   `lexeme.properties.countability` has length 1. For `[Count, Mass]` nouns
   (*time*, *combat*, *mana value*, *order*, *power*, *toughness*, *death*,
   *fortune*, *profit*, *war*), `Countability` is unset on both singular and
   plural forms, and `Determined` (`english.ron:926`) inherits `head.Countability`
   (`UNSET`) rather than `determiner.Countability`. Consequently:
   - *more than one time* and *more than one combat* match both
     `ComparativeQuantityCountDetermined` (`english.ron:938`) and
     `ComparativeQuantityMassDetermined` (`940`) (2 derivations).
   - *all the time* and *all that mana value* match both `PredeterminerCount`
     (`english.ron:634`) and `PredeterminerMass` (`635`) (2 derivations).
   - Singular count uses like *one time* or *that order* have unset
     `Countability` on the mother `NounPhrase`, matching
     `MeasuredComparativeHead` (`*one time less`, `*that order more`).
3. **`det:everything` (`crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:21`):**
   `everything` omits `number: Singular, countability: Count`, leaving `Number`
   and `Countability` unset on `FusedQuantifierNominalPhrase` (`*Everything are
   exiled.`, plus 2x derivations in `and/or`).
4. **Fused *the most* (`english.ron:757–760` and `closed_class.ron:35`):**
   Because `det:most` has `number: None, countability: None`, *the most*
   matches both `SuperlativeCountDeterminer` and `SuperlativeMassDeterminer`,
   producing two derivations whenever *the most* fuses via
   `FusedDegreeDeterminer` (*tied for the most*).

## Change

- Add `Require("$.Number", "Singular"), Require("$.Countability", "Count")` to
  `ProperName` (`english.ron:924`).
- For `[Count, Mass]` nouns: set `Countability = Some(Count)` on plural forms
  in `compile.rs`, propagate `determiner.Countability` to `$.Countability` in
  `Determined`, and prevent singular `[Count, Mass]` nouns from matching both
  `ComparativeQuantityCountDetermined` and `ComparativeQuantityMassDetermined`
  (or both `PredeterminerCount` and `PredeterminerMass`) on the same determiner.
- Set `number: Singular, countability: Count` on `det:everything` in
  `closed_class.ron:21`.
- Ensure fused *the most* (`FusedDegreeDeterminer`) produces a single
  derivation rather than firing both `SuperlativeCountDeterminer` and
  `SuperlativeMassDeterminer`.

## Done when

- `*Clause Everything are exiled.` and `*NounPhrase one time less` hold in
  judgments, and *more than one time* and *tied for the most* each have a
  single derivation.
- Every unit that reads before the change still reads, with no face lost.

Standard constraints apply.

## Record

- `ProperName` requires `Countability` `Count`. It does not bind `Number`:
  plural card names (*The Fantastic Four deal 3 damage*, *Whenever Krang &
  Shredder enter or attack*) agree in the plural, and requiring `Singular`
  lost 20 faces. `*CARDNAME attack.` and `*CARDNAME more` already had no
  Reading.
- A `[Count, Mass]` noun is `Count` in its plural form (`countable_plural` on
  the lexeme facts in `deckmaste_syntax`); its singular stays unvalued.
  `Determined` already unifies `$`, `determiner` and `head` `Countability`
  through the head convention, so it needed no new equation.
- `ComparativeQuantityMassDetermined` carries `head.Countability` `Mass`, so a
  singular dual noun reads only as a count head: *more than one time* has one
  derivation. `PredeterminerCount` requires a plural head, so *all the time*
  already had one.
- `det:everything` is `Singular` and `Count`.
- `FusedDegreeDeterminer` is a count quantity, as `FusableCardinal` is: fused
  *the most* has one analysis (Hooded Horror 78 → 39), and the spurious fused
  Indirect Object *that much* in *deals [that much] [damage plus 1]* is gone
  (19 units).
- `*NounPhrase one time less` already had no Reading. *that order more* still
  reads through `MeasuredComparativeHead`; it is a word combination no card
  contains, not an inflectional choice, so it gets no `*` entry.
- Judgments in `judgments/number_and_countability.txt`.

Census delta (measured on wyqmuoyv): faces read 28,709 → 28,709; total Readings 381,876 → 381,785; lost faces: none.
Judgment set: 2,905 enforced at `COMPLETED_STRATUM` 6, all hold; 191 pending.
