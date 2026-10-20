---
needs: []
---
# Eliminate spurious ambiguity multipliers (`<kind> counter`, existential `there`, `noun:excess`, and `Quality`)

A packed-forest audit of the highest-Reading-count cards in the Stratum 6
Census found four grammatical/lexical overlaps where the same span receives
duplicate analyses that multiply across the sentence:

1. **Compound `counter:<kind>` vs. premodified `noun:counter` (`2x` per counter
   mention; `4x` on cards mentioning counters twice):**
   `crates/deckmaste_lexical_source_v4/catalogs/counters.ron` (`[CR#122.1a..122.1j]`)
   compiles every counter kind into a compound `Category::Noun` Lexeme
   `counter:<kind>` with lemma `"<kind> counter"` (`counter:+1/+1`,
   `counter:loyalty`, `counter:time`, `counter:charge`, `counter:coin`,
   `counter:blood`, `counter:void`, etc.). At the same time, bare
   `noun:counter` (`crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:41`)
   projects to `BareNominal` and accepts `MeasurePremodifier`
   (`english.ron:569`, parsing `+1/+1 counters` a second time as
   `Measure("+1/+1") + noun:counter`) as well as `NominalPremodifier`
   (`english.ron:916`, parsing `loyalty counter`, `time counter`, `coin
   counter`, `book counter`, `charge counter`, `level counter`, `rad counter`,
   etc. a second time as `AttributiveNoun + noun:counter`). Every such counter
   mention in the corpus receives two `BareNominal` Derivations, multiplying
   *Omnivorous Flytrap* (`5,760`), *Vastwood Hydra* (`2,164`), and *Spark
   Double* (`11,382`) by `4x`, and *Betor, Ancestor's Voice* (`4,888`) and
   *Olag, Ludevic's Hubris* (`2,620`) by `2x`.
2. **`LocatedExistentialBe` vs. `ExistentialBe` + `PrepositionPhrase` Adjunct
   (`2x`–`3x` per existential clause):**
   `LocatedExistentialBe` (`english.ron:501`,
   `[Class("head", "BeVerb"), Cat("displaced_subject", "NounPhrase"), Cat("complement", "PrepositionPhrase")]`)
   and `ExistentialBe` (`english.ron:499`,
   `[Class("head", "BeVerb"), Cat("displaced_subject", "NounPhrase")]`) combined
   with `Adjunct(host: "ExistentialPredicate", adjunct: "PrepositionPhrase")`
   (`english.ron:1170`) produce duplicate `ExistentialPredicate` analyses for
   every `there is/are <NP> <PP>` clause (*if there are four or more card types
   among cards in your graveyard*, multiplying *Omnivorous Flytrap* across its
   two existential clauses).
3. **`noun:excess` missing `distribution: HeadOnly` (`2x` per *excess damage*):**
   Unlike `bottom` (`nouns.ron:25`) and `maximum` (`nouns.ron:90`), which set
   `distribution: HeadOnly` because prenominal *bottom* and *maximum* belong to
   `adjectives.ron`, `noun:excess` (`nouns.ron:63`, added for *in excess of*)
   omits `distribution: HeadOnly`. As a result, every *excess damage* in the
   corpus parses twice: once with `adj:excess` (`adjectives.ron:22`) via
   `Premodifier`, and once with `noun:excess` via `AttributiveNoun` /
   `NominalPremodifier` (doubling *Archaic's Agony* from `1,408` to `2,816`).
4. **`SingularNominalQuality` vs. `NominalPhraseQuality` (`english.ron:434–435`):**
   Bare singular mass and bare-singular-term nominals (`red`, `mana`, `colored
   mana`) project to `Quality` both directly through `SingularNominalQuality`
   (`Quality -> Nominal`) and indirectly through `BareMass` / `BareSingularTerm`
   followed by `NominalPhraseQuality` (`Quality -> NounPhrase -> Nominal`).

## Change

- Prevent `noun:counter` from duplicating declared `counter:<kind>` compound
  nouns via `MeasurePremodifier` (`+1/+1` + `noun:counter`) or
  `NominalPremodifier` (`loyalty` / `time` / `coin` / ... + `noun:counter`),
  while keeping bare `noun:counter` in its genuine non-compound uses (*a
  counter on it*, *an additional counter*, *another counter*, *each other
  counter*, *a counter of each kind*).
- Eliminate the duplicate `ExistentialPredicate` derivation between
  `LocatedExistentialBe` (`english.ron:501`) and `ExistentialBe`
  (`english.ron:499`) + `Adjunct(host: "ExistentialPredicate", adjunct:
  "PrepositionPhrase")` (`english.ron:1170`) so a trailing locative/predicate
  Preposition Phrase on `there is/are <NP> <PP>` has one attachment to
  `ExistentialPredicate`.
- Add `distribution: HeadOnly` to `noun:excess` in
  `crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:63`.
- Make `SingularNominalQuality` and `NominalPhraseQuality`
  (`english.ron:434–435`) disjoint so bare singular nominals do not project to
  `Quality` through both paths.

## Done when

- *two +1/+1 counters*, *a loyalty counter*, and *excess damage* each have one
  `Nominal` / `NounPhrase` Derivation (covered by `+` Judgments on real cards).
- *there are four or more card types among cards in your graveyard* has no
  duplicate `ExistentialPredicate` Derivation from `LocatedExistentialBe` vs.
  `ExistentialBe` + `ExistentialPredicate` PP Adjunct.
- `SingularNominalQuality` and `NominalPhraseQuality` do not double-derive the
  same `Quality`.
- Every unit that reads before the change still reads (no face lost), and the
  Census records the Reading reduction across the corpus.

Standard constraints apply.
