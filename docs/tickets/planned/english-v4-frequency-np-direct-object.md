---
needs: []
---
# A modified frequency Noun Phrase (*no more than three times*) is never a Direct Object

*Activate no more than three times each turn.* has a Reading in which *no
more than three times each turn* is the Direct Object of *activate*, with
*each turn* postmodifying *times*. Under Right Association that Reading is
the cheapest, so `no_more_than.txt:13` lost its all-endorsed top tier. The
judgment asserts *three times* as the Complement of *than*.

## Owner's ruling (round 4, verbatim)

> `no more than three times each turn` read as a direct object:
> comparative/modified frequency NPs (`no more than three times`, `at least
> twice`) are temporal/frequency adjuncts (NounKind: Temporal), never direct
> objects in NonNumericComplement.

## How this escaped `english-v4-temporal-np-as-direct-object`

That ticket (done) closed off temporal Direct Objects in `Determined`
(`english.ron`) with two exclusions. Each requires all of:
- `head.NounKind` `Temporal`;
- `head.NounDistribution` `Ordinary`;
- `determiner.DeterminativeDistribution` `Deictic` or `Distributive`.

That covers *this turn* and *each combat*. *Times* is declared
`kind: Temporal, distribution: Serial` in `nouns.ron`, and *no more than
three* is a Numerical, comparative-bounded determiner. So neither exclusion
applies, and `NonNumericComplement`, which excludes only `Numeric` and
`Manner` NounKinds, admits the Noun Phrase as a Direct Object.

## Change

- A Temporal Noun Phrase headed by a serial frequency noun under a cardinal,
  comparative or bounded determiner (*three times*, *no more than three
  times*, *at least twice* where it is a Noun Phrase) is not admitted as a
  `NonNumericComplement` Direct Object.
- Do it through existing features (`NounKind`, `NounDistribution`,
  `DeterminativeDistribution`), never by naming a verb or a word.
- Keep the genuine temporal complements and subjects the done ticket kept
  (*take an extra turn*, *skip your next turn*).
- **No new features in `english.ron`.** If the fix cannot be done without
  one, STOP and return to the owner.

## Done when

- *Activate no more than three times each turn.* and *Activate no more than
  twice each turn.* have no Reading with the frequency phrase as a Direct
  Object.
- `no_more_than.txt:13` ranks its endorsed Reading in an all-endorsed top
  tier.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
