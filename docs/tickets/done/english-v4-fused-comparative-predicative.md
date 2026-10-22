---
needs: []
---
# *be more than NP*: the comparative determinative as a fused head

Nameless Race:

> *The amount you pay can't be more than the total number of white nontoken
> permanents your opponents control plus the total number of white cards in
> their graveyards.*

The grammar has no analysis of this sentence. *More than X* in predicative
function does not parse even in its simplest form: *The amount you pay can't
be more than the number of white nontoken permanents your opponents control*
has no Reading.

Nameless Race is counted as read, but only through 25 known-wrong Readings.
All of them are the split *[more than [the total]]DeterminerPhrase [number of
…]*, with the noun *total* stranded before the Nominal that its adjective twin
premodifies. Its judgment (`comparative_quantity_than.txt:16`) holds only
through those Readings. The face keeps reading that way until this ticket
lands; it is not deferred (owner ruling, on withdrawing
`english-v4-excess-damage-relative-split`).

## Authority

- CGEL, Ch. 13 §4.1.1, p. 1126 ("Adjectival *less*"): adjectival *less*
  "contrasts with adjectival *greater* rather than with determinative *more*".
  - The lexicon's predicative adjective *less* (*mana value is less than the
    number of Mountains you control*) therefore has no adjectival *more*
    counterpart.
  - *More* here is the comparative determinative (the comparative of *much*
    or *many*, as `closed_class.ron` declares it) in fused determiner-head
    function, with its *than* Complement: a Noun Phrase as a Predicative
    Complement.
- So the fix is not an adjective *more* in `adjectives.ron`. That was tried
  and measured: it reads Nameless Race correctly, but CGEL rules it out.

## Shape

A comparative determinative (`ComparativeQuantifier`) as a fused head, taking
its selected *than* Complement, forms a Noun Phrase: *[more than [the total
number of …]]*. The production must come from existing features, frame
properties or engine closures, with no new feature in `english.ron`.

## Risk

A fused *more than NP* competes with the determiner *more than two* that
already exists (`+NounPhrase more than two lands => DeterminerPhrase "more than
two" as Determiner of land`, `comparative_quantity_than.txt`).

- *more than two lands* could gain a second analysis, *[more than [two
  lands]]*, with *two lands* a Noun Phrase.
- That reading is wrong wherever the phrase is a quantified Noun Phrase, and
  it must not be admitted.
- Measure every unit with *more/fewer/less than* before and after.

## Done when

- Nameless Race reads as *[can't be [more than [the total number of … plus
  the total number of …]]]*. Its judgment is rewritten to assert that
  analysis, and holds.
- *more than two lands* keeps exactly its determiner analysis.
- No unit rises unless the rise is the new correct Reading. No face is lost.
  `strata --check` passes.

Standard constraints apply.

Owner ruling (round 6, on withdrawing `english-v4-excess-damage-relative-split`):
the ticket stays planned and Nameless Race is not deferred. The 25 wrong
Readings may be removed by structure, never by an engine closure that names
features.

Landed as three grammar changes, each from existing features:

- `ComparativeDeterminedStandard`: *more*/*fewer* fused head with a *than*
  standard that a dependent determinative determines.
- `ComparativeQuantityCountDetermined` and `ComparativeQuantityMassDetermined`
  now require a bare standard (CGEL, Ch. 5 §11(d), p. 432). This removes the
  25 split Readings, and the same split in Nicol Bolas, the Arisen and Jace,
  Reality Sculptor (*[all but the bottom] card*).
- `ComparativePartitive`: *[more than one] of the same mana symbol*. Jegantha,
  the Wellspring had read only through the split *[more than one of the same
  mana] symbol*.

Census delta (measured on `lqmuqwyn` → `nyvrznxw`): faces read 28,799 → 28,799; Readings 214,300 → 214,294; Surplus 88,560 → 88,555; lost faces: none. Nameless Race 25 → 21, Jegantha 2 → 3, Nicol Bolas 3 → 2, Jace 6 → 4.
Judgment set: all hold (`cargo test -p deckmaste_english_v4`; `strata --check` passes).
