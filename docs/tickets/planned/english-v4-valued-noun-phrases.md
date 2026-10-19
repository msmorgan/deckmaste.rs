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
