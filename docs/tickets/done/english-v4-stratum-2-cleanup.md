---
needs: [english-v4-stratum-2-valency-passives-and-nonfinites]
---
# Stratum 2 cleanup: spurious ambiguities and weakened negative judgment

Clean up systematic duplicate-derivation multipliers and the weakened negative
judgment left after the Stratum 2 landing (`lrpolpnz`) in
`crates/deckmaste_english_v4` and `crates/deckmaste_lexical_source_v4`:

1. **`verb:get` frame overlap (`verbs.ron`):** `verb:get` is declared with
   `frames: [Monotransitive, Copular]`, while `NominalPredicative` (`Predicative ->
   NounPhrase`) wraps every `NounPhrase` as a `Predicative` and `MeasureNominal`
   (`BareNominal -> Measure`, with `NounDistribution` left unspecified) matches
   `BareSingularTerm` (`NounPhrase -> Nominal`). Consequently, every `gets
   +2/+2`, `You get an emblem`, `You get {E}{E}`, and `gets two poison counters`
   clause across Stratum 2 (1,366 / 3,148 units) parses twice (`Copular` via
   `NominalPredicative` vs `Monotransitive`). In Oracle text, `get` never takes
   an `AdjectivePhrase` or `PrepositionPhrase` predicative, and all judgments
   (`additional_slash_adjustment.txt`, `quantified_energy_symbol.txt`,
   `quantified_mana_symbol.txt`) assert `Object of get`. Restrict `verb:get` to
   `frames: [Monotransitive]` (and ensure bare `MeasureNominal` does not leak
   into arbitrary `NounPhrase` argument positions via `BareSingularTerm` when
   `Measure` is already licensed directly).
2. **Irrealis `were`/`weren't` in independent clauses and weakened negative
   judgment (`english.ron` / `mana_symbol_passive_subject.txt`):** `lrpolpnz`
   weakened `* {U}{B} were spent.` to `* {U}{B} are spent.` in
   `judgments/mana_symbol_passive_subject.txt:13` because `verb:be` and
   `verb:aren't` declare 3sg `irrealis: "were"` / `"weren't"`
   (`WordForm::Irrealis`, `Finite`), and `SubjectPredicate` only checks
   `Require("head.VerbForm", "Finite")`. Propagate `Inflection` (or mood) so
   independent main clauses (`Declarative`, `DeclarativeChoiceHeader`,
   `DeclarativeCost`) exclude `Irrealis` while preserving irrealis `were` for
   subordinate clauses (`as though it were ...`, `if ... were ...` in Stratum 4,
   CGEL Ch. 3 §1.5, pp. 85–88), and restore `* {U}{B} were spent.` in
   `judgments/mana_symbol_passive_subject.txt:13`.
3. **Missing `Number = Singular` on `ProperName` and `NumericNounPhrase`
   (`english.ron`):** Neither `ProperName` (`NounPhrase -> Name`) nor
   `NumericNounPhrase` (`NounPhrase -> Amount`) sets `Require("$.Number",
   "Singular")`. Whenever a card name (`Teferi, Temporal Archmage can be your
   commander.`, `Ozox can't block.`) or a scalar variable/number (`X can't be
   0.`) is the subject of a modal (`can`, `can't`, `may`, `must`, `will`) or a
   preterite verb (`entered`, `dealt`), it matches both `(Third, Singular)` and
   `(Third, Plural)` verb slots, producing 2 duplicate derivations. Card names
   can be plural (`The Fantastic Four deal`, `Aang and Katara enter`), so
   `ProperName` stays unmarked for number; instead a finite verb form spelled
   alike in both numbers is one slot unmarked for `Number`
   (`Lexeme::merge_number_neutral_forms`, by owner decision). Require
   `$.Number = Singular` on `NumericNounPhrase`, with `Amount` carrying its
   numeral-noun agreement as `QuantityNumber` so that a plural numeral (`0`,
   `2`) is still a singular Noun Phrase.
4. **Duplicate `Singular` and `Plural` slots on `pronoun:they` (`them`,
   `their`, `theirs`) and relative `whom`/`whose` (`closed_class.ron`):**
   `closed_class.ron` restricts nominative `"they"` to `Plural` (CGEL Ch. 5
   §17.2.2, p. 494), but retains duplicate `Singular` and `Plural`
   `FormDeclaration`s for `"them"`, `"their"`, and `"theirs"`, and likewise for
   relative `"whom"` and `"whose"`. Because nothing in the grammar agrees with
   `Number` of `Accusative` `"them"`, `PossessiveDeterminer` `"their"`,
   `IndependentGenitive` `"theirs"`, object `"whom"`, or genitive `"whose"`,
   every occurrence multiplies derivations by $2\times$ ($2^k$ for $k$
   occurrences in a sentence). Keep `they` (`they`, `them`, `their`, `theirs`)
   uniformly `Plural` and remove the duplicate number split on `whom`/`whose`.
5. **Duplicate `Determined` vs `DeterminedMass` and `PredeterminerCount` vs
   `PredeterminerMass` on dual-countability `[Count, Mass]` nouns
   (`english.ron` / `closed_class.ron`):** `Determined` and `DeterminedMass`
   share the identical RHS `[Cat("determiner", "DeterminerPhrase"), Cat("head",
   "Nominal")]`. Because any noun with `countability: [Count, Mass]` (`combat`,
   `time`, `power`, `toughness`, `ability`, `color`, `cost`, `life`, `mana`,
   `type`, `value`, ...) has `Countability = None` on its singular leaf summary,
   combining it with a countability-unspecified determiner (`any`, `the`,
   `this`, `that`, `no`, `some`, `your`, `its`, `their`) matches both
   `Determined` and `DeterminedMass` (`any time`, `this combat`, `the power`,
   `your life`). Give `det:much` `number: Singular` in `closed_class.ron` and
   unify `Determined` + `DeterminedMass` into a single `Determined` production.
   Keep `PredeterminerCount` and `PredeterminerMass` separate but disjoint by
   number (plural head vs singular mass head), so that `*all the card` stays
   excluded.
6. **Homographic `Adjective` vs `AttributiveNoun` premodifiers (`nouns.ron`):**
   Color words (`white`, `blue`, `black`, `red`, `green`) and positional words
   (`top`, `bottom`) are declared in both `adjectives.ron` and `nouns.ron` with
   `distribution: Ordinary`. Consequently, prenominal uses (`a 3/3 green
   Elephant creature token`, `the top card of your library`, `the top three
   cards of your library`) parse both via `Premodifier` (`adj:green`, `adj:top`)
   and via `NominalPremodifier -> AttributiveNoun` (`noun:green`, `noun:top`).
   Mark `noun:white`, `noun:blue`, `noun:black`, `noun:red`, `noun:green`,
   `noun:top`, and `noun:bottom` as `distribution: HeadOnly` in `nouns.ron` so
   their nominal head uses (`devotion to blue`, `on top of your library`) are
   preserved while prenominal modification comes solely from `adjectives.ron`.
7. **False `AdjectivalPassive: Lexical` on `verb:leave` from attributive-only
   homograph `adj:left` (`deckmaste_lexical_source_v4/src/lib.rs`):**
   `mark_lexical_adjectival_passives` marks any verb whose past participle
   matches the lemma of any `Category::Adjective`. Because directional
   `adj:left` (`position: Some(Attributive)`) is spelled `"left"`, `verb:leave`
   is marked `AdjectivalPassive: Lexical`, suppressing its predicative
   adjectival passive even though `adj:left` cannot appear in predicative
   position. Filter `mark_lexical_adjectival_passives` to adjectives that are
   not restricted to `Attributive` position.

## Gate

- 100% of corpus units in Strata 0–2 parse as **read** (0 `known-wrong-only`,
  0 `unread`, 0 law failures).
- Cumulative Strata 0–2 and seed judgments pass with zero regressions, with
  `* {U}{B} were spent.` restored in `judgments/mana_symbol_passive_subject.txt`.
- `Target creature gets +2/+2 until end of turn.`, `You get {E}{E}.`,
  `X can't be 0.`, `You may look at the top card of your library any time.`,
  `Create a 3/3 green Elephant creature token.`, and `Shuffle your hand and
  graveyard into your library.` (or any single-clause sentence with `their` /
  `them`) have no duplicate derivations from the sources above.

Standard constraints apply.

Census delta (`lrpolpnz` → this landing): faces read 9,501 → 9,499; total Readings 108,004 → 86,439; lost faces: Gaze of the Gorgon, Venomous Breath (Stratum 6; read only through an irrealis *were* conjunct under a demonstrative *that* subject). Strata 0–2: 59,621/59,621 units read, 0 known-wrong-only, 0 unread, 0 law failures.
Judgment set: seed and cumulative Strata 0–2 pass (1,388 enforced, 1,595 pending), with `* {U}{B} were spent.` restored; tests and clippy pass for deckmaste_lexical, deckmaste_lexical_source_v4, deckmaste_syntax and deckmaste_english_v4; citation checks pass.
