---
needs: []
---
**Rename the three "last chosen" reads to carry the printed word, and re-gate
`ChosenNumber` and `ExposedChoice` to `= 1`.** Cleanroom review 2026-09-03,
R1, ruled.

`Phrase.LastChosenPlayer` (`Phrase.idr:232`), `OfLastChosen`
(`Phrase.idr:242`), `LastChosenColor` (`Phrase.idr:1807`), `ChosenNumber`
(`Phrase.idr:1839`) and `ExposedChoice` (`Phrase.idr:2715`) all gate on
`ChoiceStands (countChoice q bs)` — "at least one stands" (`Words.idr:1298`) —
and read the most recent. That is the one place the workbench resolves by
recency.

## The ruling

"The last chosen X" is a **printed lemma whose meaning is the word "last"**.
It is not anaphora by preference, and it is not an exemption to "recency is
permanently off": the recency is in the card's own words, so a constructor may
carry it as long as the name says so. The supported corpus prints "the last
chosen …" on 12 cards (Chromatic Armor, Sanctuary Blade and Psychic Paper
bench it through `OfLastChosen`, `Cards.idr:5538/5605/5617`).

So: keep the three printed-lemma reads and rename them to carry the printed
word — `TheLastChosenPlayer`, `OfTheLastChosen`, `TheLastChosenColor` (or the
nearest names that read as the lemma) — with a declaration note saying the
recency is lexical.

`ChosenNumber` and `ExposedChoice` print no "last". They are ordinary chosen
reads and must be gated `= 1` like `OfChosen` / `ChosenPlayer`; the `≥ 1`
gate is a recency read wearing a plain name.

Size: S.

Done when: the build is 23/23 with 0 errors and 0 warnings; the three
printed-lemma reads carry the printed word in their names and a declaration
note stating the recency is lexical; `ChosenNumber` and `ExposedChoice` gate
on `= 1`; every bench site of all five is re-spelled and typechecks, and any
site the tightened gate now refuses is either re-spelled or recorded as
unspellable with its printed sentence; a pin refutes `ChosenNumber` where two
choices stand, and it is non-vacuous. Standard constraints apply.
