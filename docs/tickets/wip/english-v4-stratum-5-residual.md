---
needs: [english-v4-stratum-5-comparatives-measures-and-partitives]
---
# Stratum 5 residual: thirty-three deferred units, ambiguity cleanup, and parser performance

Close all thirty-three Stratum 5 deferred units in
`crates/deckmaste_english_v4/deferred.txt`, resolve seven Stratum 5 structural
ambiguities before Stratum 6 (coordination and right-node raising) multiplies
them, and eliminate three parser/realizer hot-path bottlenecks in
`deckmaste_syntax` and `deckmaste_english_v4`.

## Settled rulings for the thirty-three deferred units

1. **Predicative-gap bare relatives under affirmative *be*** (Mana Cannons;
   Crystal, Inhuman Princess; Tablet of the Guilds; Seal of the Guildpact;
   Bound; Jared Carthalion: *the number of colors that spell is*, *for each of
   the chosen colors it is*).
   - **Why `Exclude("clause.Polarity", "Affirmative")` was previously added:**
     It blocked two ungrammatical analyses:
     1. Non-finite *be* extracting a predicative gap inside a relative clause
        followed by a postnominal passive participle (`*Blue [creatures you
        control can't be __] blocked`); and
     2. Contracted clitic `'s` (`verb:'re`) or full `is` extracting a
        predicative gap inside `[it's __]` / `[it is __]` while 1-word
        `attacking` attached outside the relative clause via
        `GerundParticipialPostmodifier` (`*the player [it's __] attacking`),
        because `GerundParticipialPostmodifier` lacked `heavy: ["head"]`.
   - **Engine (`crates/deckmaste_syntax/src/compile.rs`):**
     - In `valence_family`, when extracting `Gap::Pred` (`SlotFill::Pred`),
       require the verb itself to be finite (`Path::Child(0, form) == Finite` on
       the `VerbPhrase` draft) so non-finite *be* (*can't be \_\_*) never
       extracts a predicative gap; and
     - Require a free verb token (`Test::FreeFrame(frame)`) whenever `gap` is
       `Some(_)` on the verb's valence draft (CGEL Ch. 18 §6.2.1, p. 1615:
       cliticized reduced auxiliaries `'s`/`'re`/`'ve`/`'m` cannot stand
       immediately before a gap, ruling out `*[it's __]`).
   - **Grammar (`crates/deckmaste_english_v4/grammar/english.ron`):**
     - Add `heavy: ["head"]` to `GerundParticipialPostmodifier` (symmetric with
       `ParticipialPostmodifier`), and move 1-word postnominal participles
       (`SingleParticipialPostmodifier` and a symmetric
       `SingleGerundParticipialPostmodifier` with `light: ["head"], cost: 3` for
       *Creatures entering*, *a creature dying*, *a creature attacking*) onto
       `BareNominal` so a single-word participle attaches inside `BareNominal`
       and never outside a `RelativeClause`.
     - For gerund-participial clauses with an overt subject (*If a creature you
       control attacking causes a triggered ability …*, Wulfgar of Icewind
       Dale; CGEL Ch. 14 §2.1, p. 1188), add `SubjectGerund` (`lhs: "Gerund",
       rhs: [Cat("subject", "NounPhrase"), Cat("head", "VerbPhrase")],
       equations: [Require("head.VerbForm", "GerundParticiple"),
       Exclude("subject.Case", "Nominative"), Exclude("subject.Case",
       "Genitive")], cost: 2`).
     - Remove `Exclude("clause.Polarity", "Affirmative")` from
       `BarePredicativeRelative` (and remove the now-unused `Polarity` feature
       from `VerbPhrase`, `Clause`, and `Coordination` in `english.ron`).

2. ***any number of other target X*** (Appa, Steadfast Guardian; Broodlord;
   Guardian of Faith; Lagrella, the Magpie; Lathiel, the Bounteous Dawn; Silver
   Surfer, Cosmic Voyager).
   - `adj:target` sets `Determination: Overt` on `BareNominal` so bare *target
     creatures* does not parse via `BarePlural` (`Require("$.Determination",
     "Bare")`) alongside determinative `det:target`. When `adj:other`
     (`FusedHeadAdjective`) precedes `target creatures`, `det:target` cannot
     appear after `other`, leaving determinerless `other target creatures`
     without a `NounPhrase` projection in the oblique of *any number of*.
   - Add `OtherTargetedPlural` in `english.ron`:
     ```ron
     (name: "OtherTargetedPlural", lhs: "NounPhrase", rhs: [Class("modifier", "FusedHeadAdjective"), Cat("head", "Nominal")],
         equations: [Require("head.Determination", "Overt"), Require("head.Number", "Plural"), Require("$.Determination", "Overt"), Require("$.Case", "Plain"), Require("$.Person", "Third")]),
     ```
     This is disjoint from `BarePlural` (`head.Determination: Bare`) and
     `Determined` (which has an overt `DeterminerPhrase` before `other`), keeps
     `+Nominal other target creatures` and `+NounPhrase two other target
     creatures` at a single Reading, and enables `phase_in_out.txt:19`
     (Guardian of Faith).

3. **Count-noun pro-form *one*** (Ezuri's Predation: *a different one of those
   creatures*; Coveted Falcon: *each one they gained control of*; Xyris, the
   Writhing Storm and Leela, Sevateem Warrior: *the first one they draw this
   turn*).
   - Replace `FusedCardinalHead` in `english.ron` with `CardinalNominal` on
     `BareNominal` (CGEL Ch. 5 §7.1, pp. 386–388; Ch. 17 §5.1, p. 1511):
     ```ron
     (name: "CardinalNominal", lhs: "BareNominal", rhs: [Numeral("head", Cardinal)],
         equations: [Require("$.Determination", "Overt"), Require("$.Number", "Singular"), Require("$.Countability", "Count"), Require("$.NounKind", "Ordinary"), Require("$.NounDistribution", "Ordinary")]),
     ```
     Because `CardinalNominal` sits at `BareNominal` with `Determination: Overt`
     and `Number: Singular`, pro-form *one* accepts premodifiers (*different
     one*, *first one*), relative clauses (*each one they gained control of this
     way*, *the first one they draw this turn*), and partitive `of`-adjuncts (*a
     different one of those creatures*), while *this one*, *that one*, and
     *another one* continue to parse with 1 Reading via `Determined ->
     Unmodified -> CardinalNominal`.
   - For *each one they gained control of this way* (Coveted Falcon; CGEL Ch. 12
     §7.3, p. 1091), allow preposition stranding out of a bare mass relational
     noun's selected `PP` complement (*gained [NP control [PP of \_\_]]*): mark
     `BareNominal`, `Nominal`, `NounPhrase`, and `ComplementNounPhrase` with
     `gap: true` in `english.ron` and add `transparent: ["head"]` to
     `Unmodified`, `BareMass`, and `NonNumericComplement` (keeping `Determined`,
     `BarePlural`, and nominal adjuncts closed to gaps so only selected
     complements of bare mass nouns extract).

4. ***divided evenly, rounded down, among*** (Fireball, Dwarven Catapult) and
   ***dealt to you so far this turn by artifacts*** (Reverse Polarity).
   - `divide` selects `PP(["among", "into"])`, and `Valence` admits an
     `Interleaved` constituent between earlier complements/head and a later
     selected `PP`. Extend `Interleaved` in `english.ron` with:
     - `InterleavedAdverb`: `lhs: "Interleaved", rhs: [Cat("head",
       "AdverbPhrase")], equations: [Require("head.AdjunctPosition", "End")],
       cost: 0` (covering both *divided evenly among* and *dealt to you so far
       this turn by artifacts*); and
     - `InterleavedSupplementedAdverb` (and bare `InterleavedSupplement`):
       comma-bracketed passive past-participial supplements (`Punct(','),
       Cat("supplement", "VerbPhrase"), Punct(',')` with
       `Require("supplement.VerbForm", "PastParticiple")`,
       `Require("supplement.Voice", "Passive")`, `heavy: ["supplement"]`,
       `cost: 3`) following an optional end `AdverbPhrase` (*divided evenly,
       rounded down, among …* and *deals half X damage, rounded down, to any
       target*).

5. ***Choose up to five {P} worth of modes*** (Season of the Bold, Season of the
   Burrow, Season of Gathering, Season of Loss, Season of Weaving; `[CR#700.2h]`).
   - In `crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron`, give
     `worth` `distribution: DirectMeasure`:
     `(lemma: "worth", distribution: DirectMeasure, countability: [Mass],
     frames: [Complement(PP(["of"]))])`.
   - In `english.ron`, add `PawprintPremodifier` (`lhs: "BareNominal", rhs:
     [Cat("modifier", "Pawprints"), Cat("head", "BareNominal")]`) so `{P}`
     premodifies `worth`, and add `MaximumMeasuredMass` (`lhs: "NounPhrase",
     rhs: [Selected("determiner", "PrepositionPhrase", ["Up"]), Cat("head",
     "Nominal")], equations: [Require("determiner.ComplementKind", "PP"),
     Require("determiner.Determination", "Bare"),
     Require("head.NounDistribution", "DirectMeasure"),
     Require("head.Countability", "Mass"), Require("head.Number", "Singular"),
     Require("$.Determination", "Overt"), Require("$.Case", "Plain"),
     Require("$.Person", "Third")]`).

6. **Single units:**
   - **Taste of Paradise** (*pay {1}{G} any number of times*): Propagate
     `NounKind` through `PrepositionPhrase` (`("NounKind", "NounKind")` on
     `Valence(head: Preposition)`) and `ObliqueNounKind` onto `BareNominal` /
     `Nominal` (`("ObliqueNounKind", "NounKind")` on `Valence(head: Noun)`), and
     change `TransparentQuantity` from `Require("$.NounKind", "Ordinary")` to
     `Agree("$.NounKind", "quantity.ObliqueNounKind")` so *any number of times*
     inherits `NounKind: Temporal` from *times* and projects as a
     `TemporalNounPhrase`.
   - **Stabwhisker the Odious** (*for each card fewer than three in their
     hand*): Add `PostpositiveComparative` (`lhs: "Nominal", rhs: [Cat("head",
     "Nominal"), Cat("modifier", "DeterminerPhrase")], equations:
     [Require("head.Number", "Singular"), Require("modifier.ComplementKind",
     "PP"), Require("modifier.PrepositionMarking", "Comparative"),
     Require("modifier.Number", "Plural"), Exclude("modifier.ObliqueKind",
     "AdvP")], heavy: ["modifier"], cost: 2`).
   - **Birgi, God of Storytelling** (*can boast twice*): Add `verbs: [(lemma:
     "boast", frames: [Intransitive])]` to `Boast` (`[CR#702.142]`) in
     `crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron`.
   - **Diligent Zookeeper** (*…, to a maximum of 10.*): Add `(lemma: "maximum",
     frames: [Complement(PP(["of"]))])` to `nouns.ron`, and change `to`'s `NP`
     frame in `prepositions.ron` from `hosts: ComplementOnly` to `hosts:
     Supplement` so unselected *to NP* attaches only as a comma-delimited
     clause/keyword supplement (`Adjunct(..., host_kind: Supplement, delimiter:
     Some(','))`).
   - **Change of Plans** (*Each of X target creatures you control connive.*):
     Add plural-synesis distributive partitive `SynesisDistributivePartitive`
     (`lhs: "PartitiveNounPhrase", rhs: [Word("head", "Each"),
     Selected("complement", "PrepositionPhrase", ["Of"])], equations:
     [Require("complement.ComplementKind", "NP"), Require("complement.Number",
     "Plural"), Require("complement.Case", "Plain"), Require("$.Number",
     "Plural"), Require("$.NounKind", "Ordinary"), Require("$.Determination",
     "Overt"), Require("$.Case", "Nominative"), Require("$.Person", "Third")],
     cost: 3`; CGEL Ch. 5 §18.2, p. 504). Because `Case: Nominative` is
     excluded from `NonNumericComplement` and `complement.Case: Plain` excludes
     pronominal `them` (`Accusative`), `+NounPhrase each of them => readings 1`
     and `*Clause Each of them get +1/+1` remain untouched.
   - **Defensive Formation** (*Rather than the attacking player, you assign …*):
     Change `rather than`'s `NP` frame in `prepositions.ron` from `hosts:
     Nominal` to `hosts: NominalAndPredicate` so preposed *Rather than NP,* can
     modify a `Clause`.
   - **Luck Bobblehead** (*If you rolled 6 exactly seven times, …*): Add
     `Amount` to `roll`'s `frames` in `verbs.ron` (`frames: [Intransitive,
     Monotransitive, Amount]`).
   - **Erebos's Intervention** (*up to twice X target cards*): Add
     `AdverbModifiedAmount` (`lhs: "Amount", rhs: [Cat("modifier",
     "AdverbPhrase"), Cat("head", "Amount")], equations:
     [Require("modifier.AdjunctPosition", "Phrasal")]`) so *twice X* is an
     `Amount` that feeds `NumericAmountDeterminer` and `up to`'s `Amount`
     frame.
   - **Keeper of the Beasts** (*… more creatures than you do as you activate
     this ability.*): Add `ComparativeClauseComplement` (`lhs:
     "PrepositionPhrase", rhs: [Word("head", "Than"), Cat("complement",
     "FiniteClause")], equations: [Require("$.ComplementKind", "FiniteClause"),
     Require("$.PrepositionMarking", "Comparative")]`; CGEL Ch. 13 §2.1,
     p. 1108), and exclude `ObliqueKind: FiniteClause` / `ComplementKind:
     FiniteClause` from `ComparativeFusedHead`,
     `ComparativeQuantityCountDetermined`, `ComparativeQuantityMassDetermined`,
     and `adj:less` so *than \<FiniteClause\>* attaches only to
     `ComparativeCountDetermined` and `ComparativeMassDetermined` (*more
     creatures than you do …*). Combined with item 11 below, restore
     `readings 1` on `less than this creature's power` in
     `crates/deckmaste_english_v4/judgments/5/comparative_complements.txt`.

## Pre-Stratum 6 ambiguity cleanup

7. **Restrict `ComparativeFusedHead` (`english.ron`) to bare *than* standards:**
   - Currently `more than one creature` parses both as `[DP more than one] [Nom
     creature]` (`ComparativeQuantityCountDetermined`) and as `[NP_fused more
     [PP than [NP one creature]]]` (`ComparativeFusedHead`).
   - Add `("Determination", "Determination")` to `Valence(head: Determinative)`'s
     `agree` list in `english.ron` (propagating `Determination` from `than`'s
     complement to `DeterminerPhrase`), and add `Require("head.Determination",
     "Bare")` to `ComparativeFusedHead` so fused comparative heads (*more than
     {5}*, *more than two*) take only bare numeral/symbol standards, never an
     overtly determined full NP (*one creature*).

8. **Prevent `MeasuredComparativeHead` and `MeasuredComparativeStandard` from
   consuming ordinary count NPs as measure modifiers (`english.ron`):**
   - In *You may choose the same mode more than once*, `[NP the same mode]`
     currently parses as the measure `modifier` of `more than once` via
     `MeasuredComparativeStandard`, yielding a bogus single-NP object of
     `choose`.
   - On `MeasuredComparativeStandard`, add `Exclude("complement.ComplementKind",
     "AdvP")`; on both `MeasuredComparativeHead` and
     `MeasuredComparativeStandard`, add `Exclude("modifier.Countability",
     "Count")` (count-noun measures such as *four more creatures than you* are
     already parsed by `MeasuredComparativeDetermined`, whereas NP measures
     before fused *more*/*less* are either bare symbols/numbers or `DirectMeasure`
     mass nouns like *7 life more than …*).

9. **Exclude `Genitive`, `Nominative`, and `Manner` NPs from
   `NonNumericComplement` and `Manner` NPs from `SubjectPredicate`
   (`english.ron`):**
   - Currently `GenitivePossessor` (`target creature's`, `Case: Genitive`) and
     `NounKind: Manner` (`this way`) are admitted by `NonNumericComplement` as
     the NP complement of any preposition (so *equal to target creature's power*
     also parses as `[AdjP equal to [NP target creature's]]` + `power`, and
     *gained control of this way* takes *this way* as the object of *of*).
   - Add `Exclude("head.Case", "Nominative")`, `Exclude("head.Case",
     "Genitive")`, and `Exclude("head.NounKind", "Manner")` to
     `NonNumericComplement`, and `Exclude("subject.NounKind", "Manner")` to
     `SubjectPredicate`.

10. **Require `ComplementKind: Intransitive` on prenominal and interleaved
    adjectives (`english.ron`):**
    - In English (CGEL Ch. 5 §14.2, p. 445), an `AdjectivePhrase` with a
      post-head complement (*equal to X*, *able to X*) is postpositive
      (`PostpositiveAdjective`), never prenominal.
    - Add `Require("modifier.ComplementKind", "Intransitive")` to `Premodifier`
      and `ModifiedQuantity`, and `Require("head.ComplementKind",
      "Intransitive")` to `InterleavedDepictive`.

11. **Require a nominative pronoun host for bound clitic auxiliaries (`'s`,
    `'re`, `'ve`, `'m`) in `SubjectPredicate` (`english.ron`):**
    - Currently every `<Singular NP>'s <Mass/Plural Noun>` (*this creature's
      power*, *target player's life*, *that spell's mana value*) also parses as a
      finite copular clause (`[NP this creature] + [VP 's [NP power]]`) via
      `SubjectPredicate` and `verb:'re` (`'s`).
    - Per `clitic_contractions.txt:1` ("clitics needing a complement and a
      pronoun host"), require a free verb phrase on `SubjectPredicate`
      (`Free("head", "VerbPhrase")`) and add `CliticSubjectPredicate` with
      `Attached("head", "VerbPhrase", Suffix)` and `Require("subject.Case",
      "Nominative")` so bound auxiliary clitics attach only to nominative
      pronoun subjects (*it's*, *you're*, *they've*, *I'm*). Restore
      `readings 1` on `less than this creature's power` in
      `judgments/5/comparative_complements.txt`.

12. **Eliminate dual `AdverbModifiedNounPhrase` vs
    `AdverbModifiedDeterminerPhrase` attachment (`english.ron`):**
    - Currently `at least one creature`, `exactly one creature`, `twice X life`,
      and `only one target` get two Readings (`[NP [AdvP] [NP [DP] [Nom]]]` and
      `[NP [DP [AdvP] [DP]] [Nom]]`).
    - Exclude `head.NounKind: Numeric` from `AdverbModifiedNounPhrase` (covered
      by `AdverbModifiedAmount` in item 6), and partition phrasal adverbs
      between determiner-phrase modifiers (*at least*, *at most*, *exactly*,
      *how* — `AdverbModifiedDeterminerPhrase` and `AdverbModifiedAmount`) and
      focusing/multiplier NP modifiers (*only*, *twice*, *not*, *just*,
      *immediately* — `AdverbModifiedNounPhrase`), so determined noun phrases
      modified by a phrasal adverb have a single attachment site.

13. **Prevent `AdverbModifiedAdverbPhrase` from applying outside
    `FrequencyInterval` (`english.ron`):**
    - Currently *so far this turn* and *only once each turn* parse both as
      `[AdvP [AdvP so far] [NP this turn]]` and `[AdvP so [AdvP far [NP this
      turn]]]`.
    - Prevent `AdverbModifiedAdverbPhrase` from taking a `FrequencyInterval`
      head (so degree/focusing adverbs modify the adverb head *inside*
      `FrequencyInterval`: `[AdvP [AdvP so far] [NP this turn]]`).

## Parser and realizer performance

14. **Eliminate redundant full Earley re-parses in `law_two` and `admit`
    (`crates/deckmaste_english_v4/src/judgment.rs`,
    `crates/deckmaste_english_v4/src/census.rs`,
    `crates/deckmaste_syntax/src/laws.rs`,
    `crates/deckmaste_syntax/src/reading.rs`):**
    - `judgment::law_two` currently re-runs a full Earley chart parse from
      scratch (`grammar.parse_category(&analyzed, parse.category())`) on every
      checked Reading (up to 8 times per corpus unit in `census` and up to 4,096
      times per judgment). When `text == parse.text()` (which always holds when
      Law 1 holds), `parse` is already the packed parse forest for that exact
      tokenization and scope: check `parse.contains(reading)` directly without
      re-running Earley, and when `text != parse.text()` re-parse at
      `parse.root` (fixing the alias-category Law 2 bug where `compile.rs`
      overwrote an alias category's name in `self.out.categories[id]` with its
      target category's name).
    - Dropped: skipping `lexicon.analyze` in admission unless a spaced noun atom
      exists is unsound (a decomposed multiword noun, *first* + *strike*, has
      no spaced atom, so lexical-atomicity admission would accept it), and the
      measured saving was within run-to-run noise.

15. **Filter terminal-initial productions by lookahead in Earley prediction
    (`crates/deckmaste_syntax/src/chart.rs`):**
    - Roughly 70% of compiled productions begin with `Symbol::Leaf(leaf_test)`
      or `Symbol::Punct(c, _)`. In the prediction loop (`chart.rs`), before
      calling `add(..., p, 0, position, position, ...)` on a production whose
      `rhs[0]` is a terminal:
      - If `Symbol::Leaf(leaf_test)`: require at least one leaf at
        `by_start[skip(position)]` to satisfy `self.leaf_matches(leaf_test,
        &forest.leaves[leaf].facts)`;
      - If `Symbol::Punct(c, _)`: let `at = if c.is_whitespace() { position }
        else { skip(position) }` and require `at < n && tokens[at] == *c`.

16. **Memoize `item_sequences` in `NodeMemo`
    (`crates/deckmaste_syntax/src/reading.rs`):**
    - `node_trees` memoizes by `node` in `NodeMemo`, but `item_sequences` does
      not memoize by `item`, recomputing shared item prefixes across derivations
      during `best_readings` and `readings`. Add an `items: HashMap<usize,
      Vec<(u64, Vec<Reading>)>>` cache (with an `in_progress_items` cycle guard,
      matching `EdgeMemo` in `laws.rs`) to `NodeMemo`.

## Gate

- `crates/deckmaste_english_v4/deferred.txt` lists no
  `english-v4-stratum-5-residual` unit (0 deferred units across Strata 0–5).
- 100% of corpus units in Strata 0–5 parse as **read** (`cargo xtask english-v4
  strata --check`), with 0 `known-wrong-only`, 0 `unread`, and 0
  `law-failures`.
- All judgment suites pass with `guardian of faith` (`phase_in_out.txt:19`) and
  `less than this creature's power => readings 1`
  (`comparative_complements.txt:4`) enforced.

Standard constraints apply.
