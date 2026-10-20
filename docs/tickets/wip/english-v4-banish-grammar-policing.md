---
needs: [english-v4-valued-noun-phrases, english-v4-adverb-so-overgeneration, english-v4-mixed-voice-participle-coordination, english-v4-unset-number-and-countability]
---
# Banish combinatorial grammar policing, dead lexical dodges, and synthetic `*` judgments

Under the owner's ruling (recorded in
[`english-v4-targeting-marker-excludes`](../done/english-v4-targeting-marker-excludes.md)),
`deckmaste_english_v4` does not police strings that no card contains. The
grammar's constraints serve exactly two purposes:

1. **Inflectional paradigm selection for bidirectional realization
   (`Semantics ↔ Syntax ↔ Text`):** morphosyntactic features (`Person`,
   `Number`, `ConjunctNumber`, `ObliqueNumber`, `QuantityNumber`, `Case`,
   `Countability`, `VerbForm`, `Inflection`) select the surface word-form within
   a Lexeme's or closed-class item's inflectional paradigm (*draw* vs. *draws*,
   *is* vs. *are* vs. *were* vs. *be* vs. *been*, *creature* vs. *creatures*,
   *they* vs. *them* vs. *their*, *a*/*an* vs. *the*, *many* vs. *much*,
   *this*/*that* vs. *these*/*those*).
2. **Disambiguating real Oracle text:** structural and feature constraints that
   prevent spurious Readings on actual Oracle sentences (i.e. removing the
   constraint increases Readings on $\ge 1$ enforced `+` Judgment or $\ge 1$
   Corpus Unit across the 28,705-face Vintage corpus).

A full ablation audit on `xkvlrypx` across all 641 child-filter equations in
`crates/deckmaste_english_v4/grammar/english.ron`, all 2,755 enforced `+`
Judgments, all 283 enforced `*` Judgments, and all 32,568 unique read rules-text
`Line` Corpus Units (28,705 faces) found that **318 of 641 (49.6%)**
child-filter equations have **zero** `+` Judgment diffs and **zero** Corpus Unit
Reading diffs, alongside dead schema/engine mechanisms and unattested CGEL
closed-class items introduced solely to satisfy synthetic Judgments or bypass
compiler checks.

## Change

### 1. Delete dead engine, schema, and lexical mechanisms

- **Delete `SchemaDecl::Valence::except` and the `TargetingMarker` Production
  (`0` `+` diffs, `0` corpus diffs, `0` `*` flips):**
  Once [`english-v4-targeting-marker-excludes`](../done/english-v4-targeting-marker-excludes.md)
  restored `TargetingMarker` (`english.ron:996`) to
  `lhs: "DeterminerPhrase", rhs: [Word("head", "Target")]` with
  `ComplementKind = Intransitive`, it became identical to the Production that
  `Valence(head: Determinative, phrase: "DeterminerPhrase")` already compiles
  for `det:target`. Remove `except: Vec<String>` from `SchemaDecl::Valence` in
  `crates/deckmaste_syntax/src/grammar.rs` and
  `crates/deckmaste_syntax/src/compile.rs`, remove `except: ["Target"]` from
  `english.ron:1158`, and delete the `TargetingMarker` Production
  (`english.ron:996`).
- **Delete unattested `IndependentGenitive` pronouns (`hers`, `theirs`,
  `yours`, `his`) and the `IndependentGenitive` Production (`0` corpus losses;
  removes 14 spurious Readings across 9 faces):**
  `hers`, `theirs`, and `yours` occur 0 times in Vintage-legal Oracle text, and
  `IndependentGenitive` `his` only creates spurious Readings on un-errata'd
  *his or her* cards. Remove the `IndependentGenitive` pronoun entries (`hers`,
  `theirs`, `yours`, `his`) from
  `crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron`, remove
  `IndependentGenitive` from `WordForm` if unused, delete the
  `IndependentGenitive` Production (`english.ron:965`), and retire the
  synthetic `IndependentGenitive` Judgments in
  `crates/deckmaste_english_v4/judgments/independent_genitive_pronoun.txt` and
  `genitive_agreement.txt`.
- **Delete fake `adv:as` and `DegreeAdverb` (`adverbs.ron:11–12, 31–32`,
  `english.ron:591, 594, 827–830`):**
  `adv:as` (`positions: [Degree]`) was added solely to give `DegreeAdverb` a
  second member alongside `adv:most` so `compile.rs`'s single-member class check
  would not fire, making `*your as recent turn` grammatical. Reference `Most`
  directly via `Word("modifier", "Most")` in `DegreeModifiedAdjective`
  (`english.ron:591`) and `DegreeModifiedAdverb` (`english.ron:594`), and
  delete `adv:as`, `adv:most`, `AdjunctPosition::Degree`, and `DegreeAdverb`.
- **Add `"triple"` to `Multiplier` determinatives (`closed_class.ron:26–27`):**
  [`ymnmuoro`](../done/english-v4-targeting-marker-excludes.md) deleted
  unattested attributive `adj:double` and `adj:triple` from `adjectives.ron`,
  leaving `"triple"` absent from `closed_class.ron`. Add `"triple"` alongside
  `"half"` and `"double"` under `distribution: Multiplier` in
  `closed_class.ron:26–27` (needed by *City on Fire*, *Fiery Emancipation*,
  *Isengard Unleashed*, and *Jeska, Thrice Reborn*: *deals triple that damage*).
- **Delete dead `article_onsets` (`closed_class.ron:9`,
  `crates/deckmaste_lexical_source_v4/src/schema.rs`, `build.rs`):**
  `deckmaste_syntax` never reads `article_onsets`.

### 2. Delete zero-impact combinatorial child-filter equations in `english.ron`

Delete every non-inflectional child-filter equation (`Require`, `Exclude`,
`Carry`, `Agree`, `ExcludeTogether` on child features) whose removal causes
**zero** `+` Judgment regressions/additions and **zero** Reading increases across
all 28,705 corpus faces (re-verifying after
[`english-v4-unset-number-and-countability`](english-v4-unset-number-and-countability.md)
lands):

- **Pure `*`-judgment combinatorial policing equations (`0` `+` diffs, `0`
  corpus diffs):**
  - `Determined`: `Exclude("determiner.DeterminativeDistribution", "Multiplier")`
    (polices `phrase_coordination.txt:24`).
  - `MeasuredComparativeHead`: `Exclude("modifier.Countability", "Count")` and
    `Carry("modifier.Case", "Plain")` (polices `phrase_coordination.txt:22`
    `*NounPhrase creatures and planeswalkers more`).
  - `MaximumMeasuredMass`: `Exclude("determiner.Countability", "Count")`
    (polices `up_to_that_many.txt:10` `*NounPhrase up to that many life`), plus
    vacuous `Require("determiner.ComplementKind", "PP")`,
    `Require("determiner.Determination", "Bare")`,
    `Require("head.NounDistribution", "DirectMeasure")`,
    `Require("head.Countability", "Mass")`, and
    `Require("head.Number", "Singular")`.
  - `MarkedMaximum`: `Require("determiner.ComplementKind", "PP")` and
    `Carry("head.DeterminativeDistribution", "Targeting")` (polices
    `scalar_cardinals.txt:17` `*NounPhrase up two target creatures`).
  - `DeterminedQuantity`:
    `Exclude("determiner.DeterminativeDistribution", "Independent")` (polices
    `genitive_agreement.txt:7` `*NounPhrase none two creatures`).
  - `DeterminedPossessor`:
    `Exclude("determiner.DeterminativeDistribution", "Independent")` (polices
    `genitive_agreement.txt:6` `*NounPhrase none creatures' power`).
- **Vacuous `Coordination` equations (`0` `+` diffs, `0` corpus diffs, `0` `*`
  flips):**
  - On `Coordination(category: "NounPhrase", ...)` (`english.ron:1225–1236`):
    delete the 27 zero-impact `Exclude("first.NounKind", "Numeric")`,
    `Exclude("last.NounKind", "Numeric")`,
    `Exclude("first.NounKind", "Quantity")`, and
    `Exclude("last.NounKind", "Quantity")` equations across `And#34`
    (`first.Numeric`, `last.Numeric`, `last.Quantity`), `And#35` (all 4),
    `AndOr#37..#40` (all 16), `Alternative#41` (`last.Numeric`), and
    `Alternative#42` (all 4) — keeping `Exclude("first.NounKind", "Quantity")`
    on `And#34` (which disambiguates *Rionya, Fire Dancer* and *Serpentine
    Curve*) and the `Disjunctive`/`Nor` equations that disambiguate corpus
    cards. Also delete
    `Exclude("first.DeterminativeDistribution", "Distributive")` on `And#35`,
    `ExcludeTogether([first.Determination=Overt, last.Determination=Bare, last.Countability=Mass])`
    on `AndOr#37`, `Require("last.Determination", "Overt")` and
    `Require("last.Number", "Singular")` on `AndOr#40`, and
    `Require("last.Determination", "Overt")` on `Alternative#42`.
  - On `Coordination(category: "RelativeClause", ...)` (`#47`, `#48`): delete
    all 4 `Require("first.ClauseMarking", "Relative")` and
    `Require("last.ClauseMarking", "Relative")` equations.
- **Vacuous `Exclude(..., "AmountKind", "ComparativeBounded")` equations (21
  equations, all `0` `+` diffs and `0` corpus diffs):**
  - On `AdverbModifiedAmount` (plus `Exclude("modifier.AdjunctPosition", "Front")`),
    `UnsignedMeasureAmount`, `PositiveMeasureAmount` (plus
    `Exclude("head.Sign", "Negative")`), `NegativeMeasureAmount` (plus
    `Exclude("head.Sign", "Negative")`), `NegativeLoyaltySymbolCost`,
    `PositiveLoyaltySymbolCost`, `ZeroLoyaltySymbolCost`, `RangeHeader`
    (`lower`, `upper`), `OpenRangeHeader`, `FusableAmount`,
    `NumericCountPremodifier`, `NumericMassPremodifier`, `AmountArithmetic`,
    `AmountQuantityArithmetic`, `MultipliedAmount`, `NumericArithmetic`,
    `SingleRowHeader`, `OpenRowHeader`, `HyphenRowHeader` (`lower`, `upper`),
    and `UnicodeRangeRowHeader` (`lower`, `upper`).
- **Vacuous `DeterminativeDistribution`, `PrepositionMarking`, `ComplementKind`,
  `NounKind`, `Voice`, `AdjectivePosition`, `Determination`, `Grade`, and
  `ClauseMarking` child-filter equations (`0` `+` diffs, `0` corpus diffs, `0`
  `*` flips):**
  - On `NominalMode`, `ChoiceHeader`, `DeclarativeCost`, `OrdinalFromEnd`,
    `AdverbModifiedNounPhrase`, `AdverbModifiedAdjectivePhrase`,
    `FrequencyInterval`, `ModifiedQuantity`, `InterleavedDepictive`,
    `AdjectivalDepictive` (`Comparative`, `Overt`, `Attributive`),
    `DepictiveAdjunct`, `DepictiveAdjective`, `AdjectivalAttribute` (`Passive`,
    `Predicative`, `Overt`), `AttributedNominal`, `ModifiedTargetingMarker`
    (`Passive`, `Predicative`), `Premodifier`, `AdjectivalConjunct`,
    `TemporalLocation`, `SerialTemporal`, `RelativeTemporalLocation`,
    `DeterminedQuantity` (`SingularOrQuantified`, `Distributive`,
    `IndefiniteArticle`, `NounKind`), `SingularOrQuantifiedDetermination`,
    `DistributiveQuantity`, `DeterminedModifiedQuantity` (`Deictic`,
    `Numerical`, `Independent`, `Count`, `NounKind`),
    `AgreeingModifiedQuantity` (`QuantityNumber`, `SingularOrQuantified`,
    `Independent`, `Count`, `NounKind`), `DistributiveModifiedQuantity`,
    `MaximumDetermination` (`PP`, `Count`), `FusedMaximum`, `Partitive`,
    `DistributivePartitive`, `SynesisDistributivePartitive` (`NP`),
    `AnyPartitive`, `AnyMassPartitive`, `MaximumPartitive`,
    `QuantityPartitive`, `ExceptivePartitive` (`Oblique`, `Dependent`, `NP`),
    `DegreePartitive`, `FractionPartitive`, `ComparativeBoundedAlternative`
    (`Comparative`, `Intransitive`), `ComparativeBoundedAmount`,
    `PostpositiveComparative` (`Comparative`), `PostpositiveLocativeAdjective`,
    `MeasuredComparative` (`Attributive`), `ComparativeCountDetermined`,
    `ComparativeMassDetermined`, `ModifiedComparativeDeterminer`,
    `ComparativeAdverb`, `ComparativeIndirectComplement` (`Intransitive`,
    `Passive`), `EquativeIndirectComplement` (`Intransitive`),
    `DegreeModifiedAdjective`, `DegreeModifiedAdverb`,
    `MeasuredComparativeStandard`, `MeasuredComparativeDetermined`,
    `MeasuredComparativeDeterminer`, `ArithmeticDeterminer`,
    `MultipliedDeterminer`, `MultipliedNounPhrase` (`Plural`),
    `MultipliedAmount` (`Serial`, `Plural`), `DegreeArithmetic` (`Dependent`),
    `MultiplierPredetermination`, `FusedOrdinalHead`, `Determined` (`NounKind`),
    `DistributiveCoordinationDetermined`, `ComparativeQuantityCountDetermined`
    (`FiniteClause`, `NounKind`), `ComparativeQuantityMassDetermined`,
    `FreeChoiceDetermined` (`NounKind`, `Countability`), `FreeChoiceQuantity`,
    `TemporalQuantity` (`Quantity`, `Plural`), `GenitivePossessor`,
    `IndependentGenitivePhrase`, `DeterminedPossessor` (`NounKind`),
    `CoordinatedPossessor`, `MultipleGenitive`, `MaximumPossessor` (`PP`,
    `Number`, `Count`), `BareSingularPossessor`, `QuantifiedPostmodifier`
    (`QuantityNumber`), `ModifiedQuantifiedPostmodifier` (`QuantityNumber`),
    `FusedPostmodifier`, `BarePredicativeRelative`, `MarkedRelative` (`Finite`),
    `FloatedEachSubjectGap`, `GenitiveRelativePhrase` (`Genitive`),
    `GenitiveSubjectRelative`, `GenitiveRelative`,
    `GenitivePredicativeRelative`, `FusedRelative`, `AdjunctGap`,
    `PostposedBareRelative`, `FiniteComplementClause`, `ClosedInterrogative`,
    `SubjectInterrogative`, `GapInterrogative`, and
    `DegreeInterrogativeDeterminer`.

### 3. Replace `[CORPUS-ONLY]` `*` judgments with real-card `+` judgments and retire combinatorial `*` judgments

- **Add `+` Judgments on real Oracle phrases for constraints previously only
  tested by `*` Judgments:**
  1. `CompleteKeywordItem`: `Require("head.Parameterization", "Supplied")` —
     protects 18 corpus cards (*Warbringer*, *Dragonkin Berserker*, *Strong
     Back*); replace `* Ward` (`stratum_zero.txt:13`) with a `+` Judgment that
     asserts the parameterized keyword analysis on a real line.
  2. `TransparentQuantity`: `Require("quantity.ObliqueNumber", "Plural")` —
     protects 6 corpus cards (*Command the Dreadhorde*, *Super-Soldier Serum*,
     *Mantle of the Ancients*); add a `+` Judgment on a real card phrase in
     place of `*NounPhrase any number of target creature`
     (`number_transparency.txt:6`).
  3. `DeterminedQuantity`: `Agree("determiner.Number", "head.QuantityNumber")` —
     protects *Mnemonic Deluge* and *Storm King's Thunder*; add a `+` Judgment
     on the real phrase in place of `*NounPhrase those one creature`
     (`genitive_agreement.txt:28`).
  4. `ComparativeQuantityCountDetermined`:
     `Agree("determiner.ObliqueNumber", "head.Number")` — protects *Nameless
     Race*; add a `+` Judgment on the real phrase.
- **Audit all enforced `*` Judgments in `crates/deckmaste_english_v4/judgments/`:**
  - **Keep** `*` Judgments that test **inflectional paradigm selection**
    (subject–verb `Person`/`Number`/`Inflection` agreement, determiner–noun
    `Number`/`Countability` agreement, pronoun `Case` selection, auxiliary
    `VerbForm` selection) needed for round-trip synthesis.
  - **Retire** synthetic combinatorial `*` Judgments on unattested word
    combinations (*Destroy target creature with flying and with.*,
    *Put a +1/+1 counter on target creature and on.*, *Destroy target creature
    during.*, *Tap target creature with.*, *Look at the top of your library.*,
    *Creatures you control get +2/+2 and have.*, *The mana value is 3 or.*,
    *up two target creatures*, *up to that many life*, *none two creatures*,
    *none creatures' power*, *creatures and planeswalkers more*, etc.), and
    update `crates/deckmaste_english_v4/judgments/README.md`.

## Done when

- `SchemaDecl::Valence::except`, `TargetingMarker`, `IndependentGenitive` (and
  `hers`/`theirs`/`yours`/independent-genitive `his`), `adv:as`, `adv:most`,
  `AdjunctPosition::Degree`, `DegreeAdverb`, and `article_onsets` are gone, and
  `"triple"` is in `closed_class.ron` under `Multiplier`.
- All non-inflectional child-filter equations in `english.ron` with zero `+`
  Judgment diff and zero corpus Reading increase are removed.
- Every `[CORPUS-ONLY]` constraint is covered by a `+` Judgment on a real Oracle
  phrase, and synthetic combinatorial `*` Judgments on unattested strings are
  retired.
- No face is lost (28,705 faces read), no Corpus Unit gains spurious Readings
  from the equation removals (total Readings decrease by the 14 removed
  `IndependentGenitive` *his* derivations), and all tests and clippy pass across
  `deckmaste_syntax`, `deckmaste_lexical_source_v4`, and `deckmaste_english_v4`.

Standard constraints apply.
