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

Until `english-v4-excess-damage-relative-split`, Nameless Race read through 25
Readings. All of them were known-wrong splits:
*[more than [the total]]DeterminerPhrase [number of …]*, with the noun *total*
stranded before the Nominal that its adjective twin premodifies. That
landing's `converted_head` closure removed them. The unit is now deferred under
this ticket in `crates/deckmaste_english_v4/deferred.txt`, and its judgment
(`comparative_quantity_than.txt:16`) is pending while it stays deferred.

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
  the total number of …]]]*. Its deferral is removed, and its judgment holds,
  rewritten to assert that analysis.
- *more than two lands* keeps exactly its determiner analysis.
- No unit rises unless the rise is the new correct Reading. No face is lost.
  `strata --check` passes.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
