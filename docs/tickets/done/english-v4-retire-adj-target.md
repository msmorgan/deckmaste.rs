---
needs: []
---
# One Lexeme for the Targeting Marker (`retire adj:target`)

The [`Targeting Marker`](../../contexts/oracle-english/CONTEXT.md) (*target* in
*target creature*, *two target creatures*, *up to one target artifact or
creature*) is currently split across two parts of speech depending on whether an
outer determiner precedes it:

- `det:target` in `crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron`
  when no outer determiner is present (*target creature*, *target artifact or
  creature*, *target creatures*).
- `adj:target` in `crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron`
  (`position: Some(Attributive), determination: Some(Overt)`) when an outer
  determiner or *other* precedes it (*two target creatures*, *another target
  creature*, *other target artifact or creature*).

That split has three defects:

1. One closed-class Oracle marker has two lexical identities (`det:target` and
   `adj:target`), and *target* in *two target creatures* is analyzed as an
   `AdjectivePhrase`.
2. `TargetingPremodifier` and `TargetingCoordination`
   (`crates/deckmaste_english_v4/grammar/english.ron:983–984`) match any
   `AdjectivePhrase` with `Determination = Overt`, so `adj:only`
   (`adjectives.ron:59`, the only other adjective with `determination:
   Some(Overt)`) is also analyzed as a `TargetedNominal` with
   `Targeting = Marked` (*the only creature you control*), while bare-target
   phrases built through `det:target` (*target creature*, *target players*) do
   not go through `TargetedNominal`.
3. Treating `det:target` as a general `Dependent` `DeterminerPhrase` admits it
   in ordinary determiner slots where the Targeting Marker never occurs
   (*target two creatures* via `DeterminedQuantity`, *target first creature* via
   `DeterminedModifiedQuantity`, *the or target creature* via
   `DeterminerPhrase` coordination).

## Change

- Delete `adj:target` from
  `crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron` so
  `det:target` in `closed_class.ron` is the sole Lexeme of the Targeting Marker
  (distinct from `noun:target` and `verb:target`).
- Build every `TargetedNominal` (`TargetingPremodifier`,
  `TargetingCoordination`, `TargetedPremodifier`) from the single Targeting
  Marker Lexeme `det:target` rather than `AdjectivePhrase` with
  `Determination = Overt`.
- Let a `TargetedNominal` both take an outer determiner (*two target creatures*,
  *another target creature*, *a second target creature*, *up to one target
  artifact or creature*) and stand as a determined `NounPhrase` or `Possessor`
  when no outer determiner is present (*target creature*, *target creatures*,
  *target artifact or creature*, *target player's graveyard*), instead of
  routing `det:target` through general `Dependent` `DeterminerPhrase` rules.
- Keep `adj:only` (`adjectives.ron:59`) as an ordinary attributive adjective
  that requires an overt determiner without marking `TargetedNominal` or
  `Targeting = Marked`.
- Rewrite `crates/deckmaste_english_v4/judgments/core_clause_ambiguity.txt:2–5`
  to assert `TargetedNominal` / `det:target` (and `!Nominal "target" head
  noun:target`) in place of `AdjectivePhrase "target" head adj:target`, and
  update `crates/deckmaste_lexical_source_v4/README.md`,
  `crates/deckmaste_english_v4/README.md`, and
  `crates/deckmaste_english_v4/judgments/README.md`.

## Done when

- `adj:target` no longer exists in `adjectives.ron`.
- `*NounPhrase target two creatures` and `*NounPhrase the or target creature`
  hold in judgments, and *the only creature* has no `TargetedNominal`.
- Every unit that reads before the change still reads, with no face lost.

Standard constraints apply.

1. **Census delta** (measured on `zmluqqrq`): faces read 28,705 → 28,705;
   total Readings 389,484 → 390,052; lost faces: none.
2. **Judgment set**: 3,000 enforced at `COMPLETED_STRATUM` 6, all hold; 189
   pending.
