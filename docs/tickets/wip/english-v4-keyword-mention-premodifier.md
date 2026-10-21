---
needs: []
---
# A keyword premodifier is a Keyword Mention, not a keyword with its cost

Witness, Unblinking Observer:

> {T}: Add {U}. Spend this mana only to pay a disturb cost or cast an instant
> or sorcery spell.

## The defect

On `pzvrlpou`, the *Spend …* sentence has 5 Readings. One of them is wrong:
*a [disturb [cost or cast an instant or sorcery]] spell*.
- *Disturb* takes its Cost Keyword Parameter ([CR#702.146a]; `parameters:
  [[Cost]]` in `keyword_abilities.ron`).
- That Cost is a `VerbalCost`: the Right Node Raising coordination *cost or
  cast* with the Shared Complement *an instant or sorcery*.
- The whole Keyword Phrase is a `KeywordPremodifier` of *spell*.

The intended analysis is *a [disturb] cost or …*: *disturb* is a Keyword
Mention that premodifies *cost*, as the glossary's *flashback cost* does
(`docs/contexts/oracle-english/CONTEXT.md`, Keyword Mention).

## Cause

`KeywordPremodifier` (`english.ron`) accepts any Keyword Phrase whose final
Keyword Parameter is not `SetOffAbility`. That includes a complete phrase
(`Parameterization: Supplied`) that ends in its Cost.

## Candidates

The implementer measures both and keeps the one with no lost face:
- require a parameterized keyword in premodifier position to be a Keyword
  Mention (`Parameterization: Omitted`);
- exclude `FinalParameter: Cost` from the premodifier.

Both use existing features and values. First measure which units read a
premodifier only through a keyword with supplied parameters.

## Done when

- A `+` Judgment on Unblinking Observer's sentence asserts
  `Nominal "disturb cost"` and
  `!KeywordPhrase "disturb cost or cast an instant or sorcery"`.
- No new feature in `english.ron`. If the fix needs one, STOP and return to
  the owner.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
