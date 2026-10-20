---
needs: []
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

## Record

The owner ordered this ticket to run before its listed `needs`, which follow it.

- Section 1: `yours` and the `IndependentGenitive` Production stay, because
  *yours* is attested (*During turns other than yours*: Mesa Lynx and nine
  more units). `hers`, `theirs` and independent `his` are deleted.
  `WordForm::IndependentGenitive` stays, since `deckmaste_lexical` uses it.
- Section 2 ran as one cumulative pass against `nvstumpz`. 174 of the 227
  listed equations are deleted. 53 stay, because removing them raised a
  Corpus Unit, changed a `+` Judgment or flipped an inflectional `*`
  Judgment. `MarkedMaximum`'s
  `Carry("head.DeterminativeDistribution", "Targeting")` stays: without it,
  Oblivious Bookworm rises from 91 to 121 Readings.
- Section 3: 124 synthetic `*` Judgments on unattested strings are retired
  (owner ruling: no policing of strings that no card contains). Among them is
  `*NounPhrase double creatures and planeswalkers` in `phrase_coordination.txt`,
  restored by [english-v4-targeting-marker-excludes](english-v4-targeting-marker-excludes.md);
  this ticket supersedes that restoration. The 156 `*` Judgments that test
  inflectional-form selection stay, and so do the three Shortened Name scope
  checks in `name_lexeme_scope.txt` (*Chandra*, *Jace*, *Ajani* off their own
  cards [CR#201.5c]): their text is real Oracle text, and they test scope, not
  a word combination. Four `+` Judgments on real cards now cover
  the `[CORPUS-ONLY]` constraints: Warbringer, Super-Soldier Serum, Mnemonic
  Deluge and Nameless Race. The retired `*` Judgments, by their location
  before retirement:
  - `articles.txt:8` a
  - `articles.txt:9` an
  - `as_long_as.txt:11` as long as that you control an artifact
  - `as_long_as.txt:12` as long as control an artifact
  - `as_long_as.txt:14` for as long to you control this creature
  - `as_long_as.txt:15` for as long as that you control this creature
  - `as_though_content_clauses.txt:9` Target creature as though it weren't blocked
  - `base_phrases.txt:12` Choose from attack.
  - `become_adjectival_passive.txt:12` becomes blocked a creature
  - `clitic_contractions.txt:18` it's
  - `clitic_contractions.txt:19` you've
  - `clitic_contractions.txt:20` that's
  - `clitic_contractions.txt:21` 're
  - `clitic_contractions.txt:22` 've
  - `clitic_contractions.txt:23` you've a creature
  - `comparative_complements.txt:17` other to this Equipment
  - `comparative_complements.txt:18` equal than its power
  - `comparative_complements.txt:19` less to its power
  - `comparative_quantities.txt:28` up than two creatures
  - `comparative_quantities.txt:29` more to one creature
  - `comparative_quantity_than.txt:12` a more life than you
  - `comparative_quantity_than.txt:13` more lands as you
  - `core_clause_ambiguity.txt:29` activate additional
  - `core_clause_ambiguity.txt:30` activate extra
  - `core_clause_ambiguity.txt:31` activate other
  - `core_clause_ambiguity.txt:32` activate only
  - `core_clause_ambiguity.txt:33` activate own
  - `correlative_finite_complements.txt:9` if either draw cards or they discard cards
  - `devotion_to_colour.txt:12` your devotion to legendary
  - `devotion_to_colour.txt:13` your devotion to black and legendary
  - `do_so_and_able_to.txt:10` creatures able to
  - `do_so_and_able_to.txt:11` creatures able block this creature
  - `do_so_and_able_to.txt:14` You can't so.
  - `do_so_and_able_to.txt:15` You can't all so.
  - `do_so_and_able_to.txt:16` You do and can't so.
  - `each_of_partitive.txt:16` every of them
  - `each_of_partitive.txt:17` the of them
  - `exactly_cardinal_modifier.txt:13` Its power is exactly.
  - `existential_there.txt:15` there attacks
  - `existential_there.txt:16` there is tapped
  - `focusing_only.txt:12` Activate only before attackers are declared by.
  - `focusing_only.txt:15` only during your turn
  - `functions_coordination.txt:5` if draw cards and they discard cards
  - `generic_predicates.txt:3` amass 1 Orcs
  - `genitive_agreement.txt:6` none creatures' power
  - `genitive_agreement.txt:7` none two creatures
  - `genitive_agreement.txt:15` none one
  - `grammar.txt:29` Draw.
  - `grammar.txt:31` .
  - `grammar.txt:33` Draw cards and .
  - `grammar.txt:55` amount of
  - `grammar.txt:56` amount {G}
  - `grammar.txt:74` You gain life equal than 3.
  - `grammar.txt:75` You gain life greater to 3.
  - `grammar.txt:95` equal than 2
  - `grammar.txt:112` Flying 2
  - `grammar.txt:113` Flying {2}
  - `grammar.txt:114` Landfall
  - `grammar.txt:115` : Draw cards.
  - `grammar.txt:128` Legendary
  - `grammar.txt:129` Legendary — Elf
  - `grammar.txt:130` Creature Legendary — Elf
  - `grammar.txt:131` Elf — Creature
  - `grammar.txt:132` Creature —
  - `grammar.txt:133` Creature — Legendary
  - `grammar.txt:138` a creature white
  - `grammar.txt:151` with you draw cards
  - `grammar.txt:152` that you draw cards
  - `grammar.txt:153` whether you draw cards
  - `grammar.txt:168` Draw cards, discard cards, and.
  - `keyword_dash_cost_line.txt:12` Haste—{3}
  - `keyword_payloads.txt:13` Protection for red
  - `keyword_payloads.txt:14` Devour for Food 3
  - `licence_hygiene.txt:7` Exile a card of without paying its mana cost.
  - `licence_hygiene.txt:8` of of paying its mana cost
  - `once_frequency.txt:13` every
  - `once_frequency.txt:14` a
  - `once_frequency.txt:15` only
  - `partitive_determinative_heads.txt:47` every of them
  - `partitive_determinative_heads.txt:48` the of them
  - `partitive_determinative_heads.txt:49` no of them
  - `partitive_determinative_heads.txt:50` none creatures
  - `partner_with_name_keyword.txt:10` Partner with
  - `phase_in_out.txt:26` This creature phases out out.
  - `phase_in_out.txt:27` phases out
  - `phrase_coordination.txt:22` creatures and planeswalkers more
  - `phrase_coordination.txt:24` double creatures and planeswalkers
  - `play_with_absolute.txt:16` play with able
  - `possessive_phrases.txt:8` your the hand
  - `quality_cost_keyword_lines.txt:8` Craft {1}{U}
  - `quantity_modifiers.txt:16` an equal to its power creature
  - `rather_than_complements.txt:7` its toughness rather than pay its mana cost
  - `scalar_cardinals.txt:16` to two target creatures
  - `scalar_cardinals.txt:17` up two target creatures
  - `seed.txt:28` Put the card.
  - `seed.txt:29` Destroy creature target.
  - `seed.txt:30` Search your library for.
  - `seed.txt:31` The card you put the card into your hand.
  - `slash_measure_selection.txt:2` Draw a card 3/3.
  - `slash_measure_selection.txt:3` Draw a card [+1].
  - `slash_measure_selection.txt:4` Draw 3/3.
  - `starting_with_adjunct.txt:8` starting
  - `starting_with_adjunct.txt:9` starting you
  - `stratum_zero.txt:4` Creature Legendary — Elf
  - `stratum_zero.txt:5` Legendary
  - `stratum_zero.txt:6` Creature —
  - `stratum_zero.txt:7` Creature — Elf — Druid
  - `stratum_zero.txt:10` Flying,
  - `stratum_zero.txt:11` Flying;; vigilance
  - `stratum_zero.txt:12` Flying {2}
  - `stratum_zero.txt:13` Ward
  - `stratum_zero.txt:15` Equip—{3}
  - `stratum_zero.txt:22` Goblinwalk
  - `stratum_zero.txt:23` Nonflying
  - `stratum_zero.txt:31` [2+]: Destroy a creature.
  - `stratum_zero.txt:46` {W}{U}: Level
  - `stratum_zero.txt:52` {1}: {2}: Destroy a creature.
  - `stratum_zero.txt:53` {T}: Flying, vigilance
  - `subordinate_clauses.txt:32` Destroy which creature
  - `typecycling_bound_suffix.txt:16` Wizardwalk
  - `typecycling_bound_suffix.txt:17` Sliverwalk
  - `typecycling_bound_suffix.txt:20` Swampscycling {2}
  - `up_to_that_many.txt:11` to that many target creatures
  - `waterbend_cost_verb.txt:9` waterbend

1. **Census delta** (measured on `urxrrvtz`): faces read 28,705 → 28,709;
   total Readings 382,529 → 382,447; lost faces: none.
2. **Judgment set**: 2,892 enforced at `COMPLETED_STRATUM` 6, all hold; 189
   pending.
