---
needs: [english-v4-stratum-3-relatives-and-gaps]
---
# Stratum 3 residual: ten deferred units and postmodifier/adjunct ranking

Close the ten Stratum 3 corpus units deferred from `english-v4-stratum-3-relatives-and-gaps` and repair two adjunct/postmodifier cost inversions introduced or exposed in Stratum 3. The rulings below are settled.

## 1. Ten deferred Stratum 3 units

The Stratum 3 landing read every Stratum 0–3 corpus unit except these ten. Until
all ten read, the census lists them as regressions at the completed Stratum.

1. **Deictic temporal *this game*** (Commander's Insignia, Vanguard of the
   Restless, Jeska, Thrice Reborn, The Swarmlord: *for each time you've cast your
   commander from the command zone this game*):
   - **Ruling:** In CGEL (Ch. 8 §6.1, p. 695), period nouns (*game*, *turn*,
     *combat*, *step*, *phase*) form bare Noun Phrase Adjuncts of temporal
     location with deictic or distributive determinatives (*this*, *that*,
     *last*, *next*, *each*, *every*), never with the plain articles *the* or
     *a* (*\*you win the game* or *\*end the turn* with intransitive *win*/*end*)
     or genitive determiners (*\*your turn* without a Preposition). By contrast,
     serial/frequency *time* also forms temporal Noun Phrase Adjuncts with *an
     additional time*, numerical/quantified determination (*two times*, *X
     times*), and *the* when modified by an ordinal/bounding adjective and
     Relative Clause (*the first/next/last time …*).
   - Declare `game` as `kind: Temporal` in `nouns.ron`, and restrict `NounPhrase`
     temporal-adjunct licensing (`NounKind: Temporal` on `NounPhrase`, or the
     determinative/nominal combination that projects it) so that `this game`,
     `this turn`, `each turn`, `last turn`, `next turn`, `each combat`, `an
     additional time`, and `the next/first/last time <RelativeClause>` are
     admitted as temporal Noun Phrase Adjuncts, while unmodified article/genitive
     Noun Phrases (`the game`, `a game`, `the turn`, `your turn`) are `Ordinary`
     Noun Phrases (giving `You win the game.` and `End the turn.` 1 Reading each).
2. **Relative-clause extraposition** past a predicate Adjunct (Skyship
   Weatherlight: *Choose a card at random that was exiled with Skyship
   Weatherlight*; Metzali, Tower of Triumph: *Choose a creature at random that
   attacked this turn*):
   - **Ruling:** Do not make *at random* a nominal Modifier (which would double
     every *discard a card at random*), and do not attach an unconditional
     `RelativeClause` directly to `VerbPhrase` without an intervening Adjunct
     (which would double every clause ending in an Object + Relative Clause).
     Instead, admit an extraposed marked Relative Clause (`ClauseMarking:
     Relative`) on `VerbPhrase` after a predicate `PrepositionPhrase` Adjunct
     (higher cost than in-situ `NominalPostmodifier`), and teach `has_antecedent`
     in `crates/deckmaste_english_v4/src/judgment.rs` to resolve the Antecedent
     of a `VerbPhrase`-attached Relative Clause to the `object` of the host
     `VerbPhrase`.
3. **Four one-offs:**
   - **Ral Zarek** (*for each coin that comes up heads*): Add `heads` and
     `tails` as nouns in `nouns.ron` and give `come` the phrasal-copular idiom
     frame `Idiom([Prep("up"), Pred])` in `verbs.ron` (CGEL, Ch. 14 §6.1).
   - **Clay Champion** (*for each {G}{G} spent to cast it*): Allow a `Symbols`
     head (or `QuantifiedSymbols` / `Nominal` built from `Symbols`) to take a
     `Postmodifier` so `each {G}{G} spent to cast it` parses compositionally.
   - **Deadly Complication** (*You may have it become no longer suspected*):
     Allow *no longer* (`adv:no longer`) to modify a predicative
     `AdjectivePhrase` (relaxing `AdverbModifiedAdjectivePhrase` in `english.ron`
     from `Require("head.AdjectivePosition", "Predicative")` to
     `Exclude("head.AdjectivePosition", "Attributive")` while keeping
     `Require("$.AdjectivePosition", "Predicative")` on the mother, or via a
     pre-predicative `Central` adverb rule on copular verbs) without adding a
     second derivation to `It's still a land.` or `Target snow land is no longer
     snow.`.
   - **Carth the Lion** (*Planeswalkers' loyalty abilities you activate cost an
     additional [+1] to activate*): Factor bracketed loyalty symbols (`[0]`,
     `[+N]`, `[−N]`) into a shared category that projects both to
     `ActivationCost` and to a singular count `BareNominal` (parallel to
     `MeasureNominal`), so `an additional [+1]` parses as an ordinary determined
     `NounPhrase`.

## 2. Adjunct and postmodifier cost/attachment cleanup

4. **Predicate Noun Phrase Adjunct cost inversion (`TemporalAdjunct` / `MannerAdjunct`):**
   - **Ruling:** Set `cost: 2` on `TemporalAdjunct` and `MannerAdjunct` in
     `crates/deckmaste_english_v4/grammar/english.ron` to match every other
     predicate Adjunct (`PreposedTemporalAdjunct`, `TemporalPostmodifier`,
     `Interleaved`, and `Adjunct(VerbPhrase, PrepositionPhrase)`). Also prevent a
     temporal Noun Phrase functioning as a predicate Adjunct (*this turn*) from
     absorbing following `PrepositionPhrase`s (*to another target creature*) as
     nominal Modifiers of *turn* (for example by requiring `Determination` or
     excluding post-nominal PP Adjuncts on bare deictic temporal Adjuncts, or
     by pricing). In *Prevent all damage that would be dealt this turn to
     another target creature with a +1/+1 counter on it* (Kurbis, Harvest
     Celebrant), the cheapest Reading must attach *this turn* inside the
     relative clause and *to another target creature …* as the selected
     Complement of *dealt*, not as a Modifier of *turn*.
5. **Participial Postmodifier cost and bare-participle attachment (`ParticipialPostmodifier` / `GerundParticipialPostmodifier`):**
   - **Ruling:** Give `ParticipialPostmodifier` and
     `GerundParticipialPostmodifier` `cost: 1` (matching `Nominal`
     `PrepositionPhrase` Postmodifiers and ranking behind `DepictiveAdjective`
     at `cost: 0`), and exclude `AdjectivalPassive: Lexical` on bare one-word
     passive `ParticipialPostmodifier`s (or require a passive participle whose
     Adjectival Passive is a declared status Adjective like *tapped* or *untapped*
     to take a Complement/Adjunct when used as a post-nominal verbal clause, so
     bare *tapped* in *onto the battlefield tapped* is only the depictive
     `AdjectivePhrase`, while *each creature tapped this way* remains a
     `ParticipialPostmodifier`). Ensure that in *Put target creature card with
     mana value X exiled with Shelob onto the battlefield tapped under your
     control*, the cheapest Reading forms the constituent `VerbPhrase "exiled
     with Shelob"` as Modifier of `card` and attaches `AdjectivePhrase "tapped"`
     as Adjunct of `put` (not Modifier of `battlefield`).

## Gate

- `cargo xtask english-v4 strata --check` exits 0 (all 62,561 Strata 0–3 corpus
  units read, 0 `known-wrong-only`, 0 `unread`, 0 law failures).
- Seed and cumulative Strata 0–3 judgments pass, including new judgments pinning:
  - `End the turn.` and `You win the game.` (`readings 1` each, no temporal-adjunct reading for `the turn` or `the game`),
  - `dealt this turn to …` (selected `to` Complement of `deal`, not a Modifier of `turn`),
  - `exiled with …` and `onto the battlefield tapped` (`VerbPhrase "exiled with …"` as Modifier of `card`; `AdjectivePhrase "tapped"` as Adjunct of `put`, `!VerbPhrase "tapped" as Modifier of battlefield`).

Standard constraints apply.
