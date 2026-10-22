---
needs: []
---
# Retired (owner, round 8 / ADR §4.4): *no more than three times each turn* as a Direct Object

*Activate no more than three times each turn.* has a Reading in which *no more
than three times each turn* is the Object of *activate*. That Reading ranks
ahead of the endorsed frequency Adjunct. The ticket is retired with no grammar
change.

## Ruling

Owner, round 8, under `docs/decisions/english-v4.md` §4.4:

- The monotransitive Reading is syntactically well-formed. Rejecting it is a
  matter for semantics, not for admission.
- An admission exclusion would also rest on the single-holder value
  `NounDistribution` `Serial`, which is under audit
  (`english-v4-single-holder-feature-values`).
- The round-4 ruling (frequency Noun Phrases are never Direct Objects) and the
  round-7 approval of an admission exclusion (Option A) are superseded for
  this case.
- *no more than [twice each turn]*, with *twice each turn* as the *than*
  standard, is accepted beside *[twice]* alone (`no_more_than.txt:12`).
- The equative multiplier, *N times / twice as much (of) X*, stays as coverage
  in `english-v4-equative-multiplier`.

## Why no structural Ranking Key fix exists

*Activate no more than three times each turn.* has 5 Derivations.

- **The Object Reading, key `0/1/16`.** *activate* takes the whole phrase as
  its Object, a Complement, so the Verb Phrase has no Adjunct to charge. Inside
  the Object, *each turn* postmodifies the one-word Nominal *times*
  (`TemporalPostmodifier`), which costs 1 word of Right Association.
- **The endorsed Reading, key `0/6/14`.** The frequency phrase is an Adjunct,
  so `EndAdverbial` charges its host *Activate* (1). `FrequencyInterval` then
  attaches *each turn* to the five-word *no more than three times* (5).

The keys differ in Right Association before Production Cost is compared.
Right Association behaves as §4.3 intends: a Complement over an Adjunct, and
the lowest host. Nothing structural separates this Object from *Destroy three
creatures*. Measuring from the host's lexical head, or not charging the Verb
Phrase Adjunct's host, still leaves the Object Reading cheaper.

The one candidate measured was to compare Production Cost before Right
Association, which reorders the §4.3 components. It was run over every
grammatical single-group judgment entry with two or more Readings (722):

- S/A/P/F went from 491/141/10/80 to 482/142/10/88;
- `no_more_than.txt:13` went from F to S, and five other entries improved;
- **14 entries fell from S/A to P/F**, among them `coordination.txt:21`,
  `except_matrix_complements.txt:2–4`, `last_turn_temporal_adjunct.txt:3`,
  `period_noun_phrases.txt:4–5`, `property_equal_to.txt:11`,
  `purpose_infinitival.txt:11` and `partitive_determinative_heads.txt:49`.

`no_more_than.txt:13` still holds: the intended Reading, with *three times*
the Complement of *than*, is in the forest.

Standard constraints apply.

Retired: no grammar change.
