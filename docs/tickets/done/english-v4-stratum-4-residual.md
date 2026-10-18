---
needs: [english-v4-stratum-4-subordinate-clauses-and-triggers]
---
# Stratum 4 residual: twenty deferred units and pre-Stratum 5 cleanup

Close all twenty Stratum 4 deferred units in
`crates/deckmaste_english_v4/deferred.txt` and resolve two Stratum 4
regressions/ambiguities before Stratum 5 begins.

## Settled rulings for the twenty deferred units

1. **Predicative-gap bare relatives** (Spider Climb and nine siblings: *the
   controller of the permanent it becomes*; Dead Ringers: *a color the other
   isn't*).
   - **Engine (`crates/deckmaste_syntax/src/compile.rs`):** CGEL limits
     relativised predicatives to the subjective (`Copular`) type (Ch. 12
     §3.3(c), p. 1045; §3.5.4(g), p. 1054). In `compile.rs`, extract
     `Gap::Pred` only from frames with `FrameFamily::Copular` (never from
     `FrameFamily::ComplexTransitive` objective predicatives such as *turn X
     Pred*, which caused the spurious *on any player's [turn any time …]*).
   - **Grammar (`crates/deckmaste_english_v4/grammar/english.ron`):** Add a
     two-value `"Polarity": ["Affirmative", "Negative"]` feature on
     `VerbPhrase` and `Clause` (or propagate `VerbKind` and `Polarity` from
     the head `VerbPhrase` to `Clause`, with `AnalyticNot`, `NeverNegation`,
     and `CliticNegation` setting `Negative` and lexical/auxiliary projections
     defaulting to `Affirmative`). In `BarePredicativeRelative` (`cost: 3`),
     require `Require("$.NounKind", "Ordinary")` on the host nominal (blocking
     temporal heads like *the last time [it became \_\_] blocked*) and require
     the relative clause's finite predicate to be either a lexical copula
     (`become`: `VerbKind: Lexical`) or a negated auxiliary copula (`isn't`:
     `Polarity: Negative`), excluding affirmative auxiliary *be* (`is` / `'s`)
     so progressive object-gap relatives (*the player it's attacking*, *the
     player it is attacking*) keep a single Reading.

2. ***returned this way* without a goal** (Arid Archway, Foul Renewal).
   - **Lexicon (`prepositions.ron`, `nouns.ron`, `verbs.ron`):** `to` with an
     `NP` or `Amount` complement is declared with `hosts: Nominal` in
     `prepositions.ron` solely for *the player to your right*. Change `to`'s
     `Amount` and `NP` frames in `prepositions.ron` to `hosts: ComplementOnly`,
     and add `Complement(PP(["to"]))` to `player`'s `frames` in `nouns.ron`
     (which `BareNominal` valence already projects). This also eliminates
     unselected `to`-PP nominal postmodification across the grammar.
   - Add `Monotransitive` to `return` in `verbs.ron` (in addition to its
     existing goal-PP frame). Because `to` + NP no longer modifies arbitrary
     nominals, *Return X to Y* retains a single Reading while passive and
     participial *returned this way* parse cleanly.

3. **Postposed bare relative past *at random*** (Infectious Rage: *choose a
   creature at random this Aura can enchant*; Volatile Chimera: *Choose a card
   at random you exiled with …*).
   - `PostposedRelative` requires `ClauseMarking: Relative` when crossing a
     general `PrepositionPhrase` because a PP with an `NP` complement (*for
     each creature*) ends in a noun that a bare relative modifies in situ.
   - `at random` (`PrepositionFrame::AdjP`) has an adjectival complement, not a
     nominal one, so no in-situ attachment competes. Track whether a
     `PrepositionPhrase` has a non-nominal complement (e.g. via a feature or
     production on `PrepositionFrame::AdjP` / `Intransitive`) and admit a
     postposed bare object-gap relative (`PostposedBareRelative`, `cost: 2`,
     `heavy: ["modifier"]`) only across a `Predicate`-hosted
     `PrepositionPhrase` that lacks an `NP` complement.

4. **Singletons**:
   - **Infinite Reflection** (*When this Aura enters attached to a creature, …*):
     Delete `passive: true` from `attached` in `adjectives.ron` (leaving
     `position: Some(Predicative)` alongside `tapped` and `transformed`) and
     remove the now-unused `pub passive: bool` field from `AdjectiveEntry` in
     `crates/deckmaste_lexical_source_v4/src/schema.rs` and `build.rs`.
     Consequently, `attach`'s `MonotransitivePP(["to"])` frame derives its
     passive `VerbPhrase` *attached to NP* normally for both postnominal
     modification (*each Aura attached to it*, via `ParticipialPostmodifier` at
     `cost: 1`) and depictive adjunct use (*enters attached to a creature*),
     while `attached`'s own `Complement(PP(["to"]))` frame in `adjectives.ron`
     continues to supply the adjectival passive after copular *be* / *become*
     (*is attached to*, *becomes attached to*). Raise `DepictiveAdjective` in
     `english.ron` from `cost: 1` to `cost: 2` so nominal postmodification
     (`cost: 1`) outranks clause-level depictive attachment (`cost: 2`) on
     *Equipped creature gets +1/+1 for each Aura attached to it*, and update
     `trigger_clauses.txt` line 12 to assert the top-ranked nominal modifier
     Reading without requiring `readings 1` (since clause-level depictive vs
     NP-internal postmodifier across two distinct heads is a genuine structural
     attachment ambiguity).
   - **Celeborn the Wise** (*… for each card looked at while scrying this way*):
     In `crates/deckmaste_syntax/src/compile.rs`, extend passive derivation
     (`passive_draft` / `passive_by_draft`) to `FrameFamily::Prepositional`
     (`slots == [SlotFill::PP(_)]`, excluding `VerbKind::Auxiliary` such as
     *be*): derive passive and passive-`by` `VerbPhrase` productions whose
     complement child is that `SlotFill::PP` with `GapRule::Require(Gap::NP)`
     (CGEL Ch. 4 §6.1.2, pp. 276–277; Ch. 16 §10.1.2, pp. 1432–1433).
   - **North Star** (*For one spell this turn, you may spend mana as though it
     were mana of any color to cast that spell.*):
     Admit a preposed `PrepositionPhrase` + `TemporalNounPhrase` pair sharing a
     single comma on `Clause` (`PreposedPrepositionAndTemporal`, `cost: 2`), or
     equivalent composition on preposed temporal/prepositional adjuncts.
   - **Critical Hit** (*When you roll a natural 20, …*):
     Add `"natural"` (`position: Some(Attributive)`) to `adjectives.ron`, and
     admit an Arabic numeral as a singular count `BareNominal` head when
     determined (`IndefiniteArticle` / determined numeral nominal, *a 20*, *a
     natural 20*, *a 4*; CGEL Ch. 5 §7.1, p. 386) without allowing bare
     numerals to re-parse as ordinary singular count nouns in undetermined
     contexts.
   - **Overpowering Attack** (*… there is an additional combat phase after this
     phase, followed by an additional main phase.*):
     Add a comma-set heavy passive past-participial supplement on `Clause`
     (`ParticipialSupplement`, `Require("supplement.VerbForm",
     "PastParticiple")`, `Require("supplement.Voice", "Passive")`,
     `heavy: ["supplement"]`, `cost: 3`; CGEL Ch. 14 §9.1, p. 1265; Ch. 15
     §5.1, p. 1350).

## Pre-Stratum 5 regression & ambiguity cleanup

5. **Restore `*Nominal amount {G}` in `judgments/grammar.txt`:**
   - Stratum 4 (`zowmvpzy`) changed `SymbolNominal` from `lhs: "Nominal"` to
     `lhs: "BareNominal"` (`Determination: Overt`), which allowed
     `NominalPremodifier` to attach `AttributiveNoun` (*amount*) to
     `SymbolNominal` (*{G}*), and removed `*Nominal amount {G}` from
     `crates/deckmaste_english_v4/judgments/grammar.txt`.
   - Restore `*Nominal amount {G}` to `judgments/grammar.txt`. Add
     `Exclude("head.Determination", "Overt")` to `NominalPremodifier` in
     `english.ron` (in English, noun premodifiers sit inside determinative and
     adjectival premodifiers—never outside `adj:target` or on a `SymbolNominal`
     head), and set `distribution: HeadOnly` on `amount` in `nouns.ron`.

6. **Restrict `which` and `whichever` determinative distribution (`closed_class.ron` and `tests/seed/mod.rs`):**
   - `which` and `whichever` currently default to
     `DeterminativeDistribution::Dependent`, which lets `Valence` project them
     as ordinary `DeterminerPhrase`s in `Determined` (*Destroy which creature*,
     and the spurious second Reading of *which number you chose* where *which*
     is a plain determiner and *you chose* is a bare relative clause).
   - Set `distribution: Independent` on `which` and `whichever` in both
     `crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron` and
     `crates/deckmaste_english_v4/tests/seed/mod.rs` (their dedicated
     productions `InterrogativeDeterminer`, `FusedSubjectRelative`, and
     `FusedRelative` match `Word("...", "Which")` / `Word("...", "Whichever")`
     directly, and neither is in `FusedQuantifier`). Add `ContentClause` to
     `reselect` in `verbs.ron` (*reselect which player or planeswalker Return
     to Action is enchanting*). Assert `readings 1` on `+Clause You choose a
     creature with mana value equal to the last chosen number.` / `+ContentClause
     which number you chose` in `subordinate_clauses.txt`.

## Gate

- `crates/deckmaste_english_v4/deferred.txt` is empty (0 deferred units across
  Strata 0–4).
- 100% of corpus units in Strata 0–4 parse as **read** (`cargo xtask english-v4
  strata --check`), with 0 `known-wrong-only`, 0 `unread`, and 0
  `law-failures`.
- All judgment suites pass with `*Nominal amount {G}` restored.

Standard constraints apply.


Census delta (`qsruvtzr` → `wukuuylm`): faces read 18,780 → 18,805; total Readings 159,769 → 146,555; lost faces: Betrayal at the Vault, Jagged Lightning, Ojer Axonil, Deepest Might, Swelter, Twinstrike (each read only through an unselected `to` nominal postmodifier, e.g. *deals [2 damage to each of two target] [creatures]*, which ruling 2 removes). Strata 0–4: 71,890 units read, 0 deferred, 0 known-wrong-only, 0 unread, 0 law failures.
Judgment set: seed and cumulative Strata 0–4 pass (1,967 enforced, 1,106 pending); tests and clippy pass for deckmaste_syntax, deckmaste_lexical_source_v4 and deckmaste_english_v4; citation checks pass.
