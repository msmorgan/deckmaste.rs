---
needs: []
---
# Report single-holder feature values, and fold or justify each one

## Owner's ruling (round 8, treatment item 3)

Enforce english-lexical-analysis §3 ("A class with one member, or a feature
with one holder, is reported as a finding") on feature values too. Extend
`compile.rs`'s fewer-than-two-members check from lexical classes to feature
values. Then fold the single-word values (NounDistribution `Serial`, `Valued`,
`BareSingular`; the `LocativeQuantifier` class) and the one-value features
(`AmountKind`, `Targeting`, `Conversion`). `docs/decisions/english-v4.md` §4.4,
item 5, applies the finding to new values too.

## Evidence

Re-grepped on `qynmxmro`. Line numbers drift.

**The check today.** `CompiledGrammar::bind` (`crates/deckmaste_syntax/src/compile.rs:551–590`)
warns into `LoadReport::warnings` when a declared or typed lexical class has
fewer than two members. Nothing checks feature values.

**The values.** `english.ron` line numbers; holders from
`crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron`.

| Value | Holders | Consumers | What depends on it |
|---|---|---|---|
| NounDistribution `Serial` (`:44`; `schema.rs:758–761`) | *time* (`nouns.ron:143`) | `TemporalLocation` excludes it (`:750`); `SerialTemporal` requires it (`:752`); `MultipliedDeterminer` and `MultipliedNounPhrase` require it (`:917`, `:920`) | Frequency Noun Phrase Adjuncts (*any time*, *three times*), multipliers (*three times X*). The landed `english-v4-temporal-np-as-direct-object` fix (`Determined`'s two `ExcludeTogether`s, `:1090–1091`) keys on `NounDistribution` `Ordinary`, so *time* escapes it only because it is `Serial`. The round-7 Option A for `english-v4-frequency-np-direct-object` named `Serial` too; that ticket is now a ranking ticket. |
| NounDistribution `Valued` (`schema.rs:753–757`) | *power*, *toughness* (`nouns.ron:119, 149`) | `AttributiveNoun` excludes it (`:1034`); the bare property Noun Phrases require it (`:1068`, `:1077`, `:1083`); `BarePlural` (`:1140`) | *power 3 or less*, *base power and toughness 4/4*, *its power*; no attributive *power* |
| NounDistribution `BareSingular` (`schema.rs:749`) | *attacking player*, *defending player* (`nouns.ron:176, 178`) | `BareSingularTerm` (`:1027`) and `BareSingularPossessor` (`:1198`) require it; `SingularNominalQuality` (`:513`) and `BarePrepositionalNominal` (`:1065`) exclude it | *defending player controls*, *defending player's* |
| Lexical class `LocativeQuantifier` (`:342`) | `det:anywhere` | `FusedLocative` (`:1033`) | *from anywhere*. The existing class check already reports it. |
| `AmountKind` = [`ComparativeBounded`] (`:46`) | set only by `ComparativeBoundedAmount` (`:876`) | `NumericNounPhrase` excludes it (`:434`) | *power 3 or less* is a noun's value, never a numeric Noun Phrase |
| `Targeting` = [`Marked`] (`:49`) | set by `MarkedDeterminer` and `MarkedMaximum` (`:1176`, `:1178`) | `TargetedNounPhrase` (`:1134`), the Noun Phrase Coordinations (`:1491–1502`) and the Determiner Phrase Coordination (`:1534`) | the Targeting Marker after an outer determiner (*two target*, *up to one target*) |
| `Conversion` = [`Converted`] (`:62`) | derived by `deckmaste_lexical_source_v4/src/build.rs:450–467` for every free noun/adjective lemma pair (colour words, *top*, *total*) | `AdjectivalQuality` excludes it (`:518`); `NominalPredicative` excludes it with a bare determination (`:1371`) | colour words as Quality and as predicative |

## Change

1. **The finding.** In `bind`, add two `LoadReport` warnings: a declared
   feature with fewer than two values, and a feature value with fewer than two
   holders. A holder is a lexeme whose feature bundle carries the value, or,
   for a value no lexeme carries, a Production that sets it on its mother. The
   warning names the feature, the value and its holders. Add a test in
   `deckmaste_syntax` for each warning.
2. **Fold or justify each value in the table.**
   - *Fold*: express the distinction through an existing feature, value,
     class or Frame, so the value goes. Examples: a declared class or frame on
     the lexemes instead of a distribution value; a one-value feature replaced
     by an existing feature's value.
   - *Justify*: keep it, with a comment in `english.ron` or the lexicon citing
     CGEL or `docs/oracle-style-guide.md` for the grammatical distinction it
     carries. Also say why one holder is the whole class in Oracle English.
     A value whose only job is to exclude a dispreferred Reading cannot be
     justified (`english-v4.md` §4.4, item 2). Its exclusion goes, and the
     Ranking Key orders what it kept out.
3. Record each decision in the landing, with what moved for its dependents
   above.

## Done when

- `bind` reports single-holder feature values and one-value features, with
  tests.
- Each value in the table is folded or carries its justification. The load
  report lists only justified ones.
- No face is lost.
- Every judgment holds.
- `cargo xtask english-v4 strata --check` passes.
- The landing reports the cheapest tier: ambiguous units with a tier of one,
  the tiers' sum and the largest tier, before and after. No judgment-set entry
  goes from an all-endorsed cheapest tier to one that is not.
- No new `Summary`, `State` or `ProductionDecl` field, and no new `closure.rs`
  pass.

Standard constraints apply.

## Landing (`vnmvouxm`)

**The finding.** `CompiledGrammar::bind` reports a feature with fewer than two
values and a value with fewer than two holders, naming the holders
(`crates/deckmaste_syntax/tests/load_report.rs`). Holders are the lexemes that
carry the value and the Productions that set it on their mother. Three
refinements keep the report to real findings: a value with neither is a
default (*VerbKind* `Lexical`), held through the Productions that require it of
a child; a value projected from inflection is held by every word form inflected
for it; a feature equated with another (`ObliqueNumber` with `Number`) shares
its holders. Counting lexemes only, where any lexeme holds a value, would
report `Determination` `Overt` (one lexeme, many Productions).

**Decisions.**

| Value | Decision | What moved |
|---|---|---|
| NounDistribution `Serial` | Justified: *times* heads every bounding frequency Noun Phrase and cardinal multiplier (CGEL, Ch. 8 §9, p. 715; Ch. 5 §12, p. 434), so *time* is the whole class, in English and in Oracle. Its consumers build constructions (`SerialTemporal`, the multipliers). | Nothing. Comment in `english.ron` and `schema.rs`. It is the one table value still in the report. |
| NounDistribution `BareSingular` | Justified: a singular count noun heading a bare Noun Phrase, which a singular count noun otherwise cannot (CGEL, Ch. 5 §8.5, p. 409); in Oracle, the combat roles [CR#506.2] (OSG §6, "Controller and owner"). Two holders, so not reported. | Nothing. Comment in `english.ron` and `schema.rs`. |
| NounDistribution `Valued` | Folded: *power* and *toughness* are Ordinary mass nouns, as *mana value* already was. | `ValuedBare` and `GradedValuedBare` retired (`BareMass`, `BarePlural` and `AttributedNominal` build the same Noun Phrases); the `Valued` exclusions in `AttributiveNoun`, `BareMass` and `BarePlural` retired. This exposed `MeasuredComparativeStandard` admitting any Noun Phrase as its measure phrase (*[target creature with power] less than ...*, already true of *mana value*). Its measure is now quantified by a numeral (`Carry` `Numerical`; CGEL, Ch. 6 §3.2, pp. 549–550), which `MeasuredMass` now passes up. |
| Class `LocativeQuantifier` | Folded: one closed-class lexeme fills the slot, so a direct reference (english-lexical-analysis §3). | Closed word `Anywhere`; `FusedLocative` names it. |
| `AmountKind` | Retired: exclusion only. | `NumericNounPhrase` admits *3 or less* as a Noun Phrase. Nothing ranks wrongly. |
| `Targeting` | Retired: exclusion only. | The four `Carry(first.Targeting)` Noun Phrase Coordinations retired (the others now take a marked first conjunct); exclusions in `TargetedNounPhrase` and the Determiner Phrase Coordination retired. |
| `Conversion` | Retired: exclusion only. | `mark_conversions` (lexicon build) and both exclusions retired. `AdjectivalQuality` costs 4: a fused head is the marked construction, so a colour word's noun ranks first in a Quality. |

**Judgments rewritten** (the retired exclusions' witnesses; each entry's
cheapest Reading is the endorsed one): `color_word_categories.txt` 2–5,
`coordination.txt` 27, 59, 71, `keyword_quality_phrases.txt` 4,
`property_pair_each_equal_to.txt` 3, `targeting_projection.txt` 27,
`valued_noun_phrases.txt` 4. Each lost only its `!` assertion or `readings 1`.

**Findings outside the table**, still in the load report, for the owner to
ticket or rule on: classes `FusedHeadAdjective`, `RangeMarker`, `Pawprint`,
`InterrogativeAdverb`; `AdjectivalPassive` `Derived`, `ComplementKind` and
`ObliqueKind` `InterrogativeClause`, and `VerbKind` `Lexical` (no holder);
`AdjunctPosition` `Front`, `Central`, `Phrasal`, `Bounding`; `ClauseMarking`
`To`; `ComplementKind`/`ObliqueKind` `MeasureNP`, `VerblessClause`,
`QuotedAbility`; `DeterminativeDistribution` `IndefiniteArticle`,
`SingularOrQuantified`, `FreeChoice`, `Targeting`; `FinalParameter`
`SetOffCondition`, `SetOffMeasure`, `SetOffKeyword`; `NounDistribution`
`Quantity`; `NounKind`/`ObliqueNounKind` `Manner`, `Symbol`;
`Parameterization` `Supplied`, `Omitted`; `PrepositionMarking` `Oblique`,
`Comparative`; `PronounKind` `Existential`. `ProductionDecl::postdependent`
lost its only grammar use with `ValuedBare`.

**Cheapest tier.** Ambiguous units with a tier of one 14,798 of 16,558 →
15,029 of 16,837; tiers' sum 18,913 → 19,254; largest 8 → 8. No judgment-set
entry left an all-endorsed cheapest tier (single- and multi-group entries with
2–4,000 Derivations: 676 S / 207 A / 11 P / 81 F → 693 / 216 / 10 / 81).

1. **Census delta** (`upkvrokm` → `vnmvouxm`): faces read 28,800 → 28,800;
   Readings 213,916 → 215,764; Surplus 88,285 → 90,034; lost faces: none.
   `strata --check` passes.
2. **Judgment set**: all hold (3,037 enforced, 208 pending), after the
   rewrites above.
