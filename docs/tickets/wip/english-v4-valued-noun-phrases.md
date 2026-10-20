---
needs: []
---
# Structural licensing for bare `Valued` noun phrases (*power*, *toughness*)

`nouns.ron` marks *power* and *toughness* with `distribution: Valued` so they
never stand alone as an unmodified bare `NounPhrase`. However, two rules in
`crates/deckmaste_english_v4/grammar/english.ron` still leak unvalued *power*
and *toughness* as bare `NounPhrase`s:

1. `ValuedBareMass` (`english.ron:919–921`) uses `heavy: ["head"]` (`leaves >=
   2`) as a proxy for having a value (*power 3 or less*) or a postmodifier
   (*power equal to …*). Any premodifier (`base power` via
   `NominalPremodifier`, `total power` or `greatest power` via `Premodifier`)
   or any disjunctive `Nominal` coordination (`power or toughness` via
   `Coordination(Nominal, Disjunctive)`) also has `leaves >= 2`, so
   `ValuedBareMass` admits unvalued `[NP base power]`, `[NP total power]`, and
   `[NP power or toughness]` as complete bare `NounPhrase`s.
2. `BarePlural` (`english.ron:955–956`) omits
   `Exclude("head.NounDistribution", "Valued")`. When *power and toughness* (or
   *base power and toughness*) coordinates at `Nominal` (`english.ron:1231`),
   it has `Number = Plural` and `NounDistribution = Valued` (`proximate`), and
   `BarePlural` admits unvalued `[NP power and toughness]` and `[NP powers]`.

As a result, *Sword of the Squeak* (*with base power or toughness 1*) gets a
spurious second derivation (`[NP base power] or [NP toughness 1]`), *Cut Down*
and *Meek Attack* (*total power and toughness 5 or less*) got the wrong reading
`[NP total power] and [NP toughness 5 or less]`, and ungrammatical Clauses like
`*Target creature has base power.` and `*Target creature has power and
toughness.` read.

## Change

- Add `Exclude("head.NounDistribution", "Valued")` to `BarePlural`
  (`english.ron:955`), matching `BareMass` (`english.ron:915`).
- Replace `heavy: ["head"]` on `ValuedBareMass` (`english.ron:919–921`) with
  structural licensing: a bare `Valued` nominal projects to a `NounPhrase` only
  when it has its value (`Amount` Complement or shared RNR value) or a
  postmodifier (*power equal to …*), never from premodification or nominal
  coordination alone.

## Done when

- `*Clause Target creature has base power.`,
  `*Clause Target creature has power and toughness.`, and
  `*Clause Destroy target creature with power or toughness.` hold in judgments.
- *Sword of the Squeak* has no unvalued `[NP base power]` conjunct reading.
- Every unit that reads before the change still reads, with no face lost.

Standard constraints apply.

## Record

- The syntax engine gains a `postdependent` child weight: the child ends in a
  dependent of its head (a Complement, a postmodifier, or a value shared by
  Right Node Raising) rather than in the head, through the last conjunct of a
  coordination. `ValuedBare` (formerly `ValuedBareMass`) requires it in place
  of `heavy`, for either Number, so a valued coordination is licensed by its
  value (*base power and toughness 4/4*).
- Real cards need two more licenses that the change above would have lost.
  `GradedValuedBare` licenses a bare Valued nominal under a graded adjective
  that implies its value (*creatures with greater power*, *tied for greatest
  power*, *equal toughness*); *equal* is now `grade: Equative`, like *same*.
  A counted plural stays an ordinary bare plural (*with different powers*).
  So `BarePlural` excludes only a coordination of Valued nominals, not
  `NounDistribution = Valued` wholesale.
- The Done-when's `*` Judgments are on strings no card contains. Their intent
  is covered by `+` Judgments on real cards in `valued_noun_phrases.txt`
  (Square Up, Stern Scolding, Squall, Gunblade Duelist) and by
  `!NounPhrase "base power"` on Sword of the Squeak.
- Porcelain Gallery and Trench Gorger: *have base power and toughness each equal
  to X* no longer has a bare unvalued Object. Their Judgments now assert the Noun
  Phrase with its postmodifier.
- Cut Down and Meek Attack lose the spurious `[NP total power] and [NP
  toughness 5 or less]` reading, but stay deferred to
  `english-v4-stratum-7-discourse-and-long-tail`: *total* still scopes only
  over *power*, not the coordination.

1. **Census delta** (measured on `owynyzkn`): faces read 28,709 → 28,709;
   total Readings 382,447 → 382,154; lost faces: none.
2. **Judgment set**: 2,898 enforced at `COMPLETED_STRATUM` 6, all hold; 189
   pending.
