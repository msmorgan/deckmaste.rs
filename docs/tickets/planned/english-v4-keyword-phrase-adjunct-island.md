---
needs: []
---
# A Keyword Phrase takes only its catalog-declared parameters

Owner ruling (cost-audit defect #6): a Keyword Phrase is a closed domain
constituent (`docs/decisions/english-v4.md` §4.2). It takes only the Keyword
Parameters its catalog entry declares. It never takes a free Preposition
Phrase or a *where* Supplement.

## The defect

Two Adjunct Schemas in `english.ron` open the Keyword Phrase to any
Preposition Phrase:

- `Adjunct(host: "KeywordPhrase", …, host_kind: Nominal, …)`, a postmodifier.
  It excludes only `KeywordForm: Parametric`.
- `Adjunct(host: "KeywordPhrase", …, host_kind: Supplement, delimiter: ',')`,
  a comma-set Supplement.

These schemas produce spurious Readings. In each example below, the bracketed
phrase is read as attached to the keyword:

- *create an X/X blue Spirit creature token with flying, [where X is the number
  of unlocked doors among Rooms you control]*: the *where* phrase is a
  Supplement of *flying*.
- the same text with *[of unlocked doors]* as a Modifier of *flying*.
- *It's a 1/1 Spirit creature with flying [in addition to its other types]*:
  *flying in addition to its other* is read as one phrase.
- *create X 1/1 white Vampire creature tokens with lifelink, [where X is
  Elenda's power]*.
- *… with flying and haste, [where X is the greatest mana value …]*.

Authority:

- `docs/decisions/english-v4.md` §4.2: *with* + Keyword Phrase is a Nominal
  postmodifier. A keyword ability is never a host for open-domain attachment.
- [CR#702.1d]: *with [keyword ability]* means "with a [keyword ability]
  ability". The Keyword Phrase names an ability. It is not a description that
  takes restrictive modifiers.

## What the open schemas carry today, and where it must go

The schemas also carry real Oracle text. The fix moves each of those uses onto
the structure that the ruling allows.

- **A parameter of a bare mention.**
  - *Enchanted creature has landwalk of the chosen type.* (Traveler's Cloak,
    and Judgment `core_composition.txt:6`.) It reads only through the open
    Nominal Schema today.
  - Landwalk's catalog entry ([CR#702.14a,702.14c]) declares only a bound
    stem.
  - Declare the *of* + Noun Phrase quality of the mention as a Keyword
    Parameter frame in `catalogs/keyword_abilities.ron`.
- **A *where* definition of a Keyword Parameter X on a Keyword Line.**
  - Examples:
    - *Mobilize X, where X is …* (Avenger of the Fallen; Infantry Shield,
      inside a sentence);
    - *Firebending X, where X is …* (Firebending Student, Sun Warriors, Fire
      Lord Zuko, Zuko, Firebending Master);
    - *Annihilator X, where X is …* (That Which Was Compleated).
  - The Supplement belongs to the Keyword Line item (`KeywordLineItem`). It
    does not belong to the Keyword Phrase.
  - Inside a sentence it belongs to the Clause (the existing Clause Supplement
    Schema), as in *Equipped creature has menace and mobilize X, where X is
    its power*.

## Change

- Remove both `KeywordPhrase` Adjunct Schemas.
- Add the landwalk quality frame to the catalog.
- Add a *where* Supplement on `KeywordLineItem`. It is limited to an item
  whose final parameter is a variable amount, so that it defines X
  ([CR#702.86a,702.181a,702.189a] define those parameters as N).

## Acceptance set

All figures are from the cost-audit evaluation data, measured on `loxuknun`.

- **Corpus sample:**
  - 3 Segments lose 841 Readings, out of a combined Surplus of 1,076.
  - The Segments are:
    - Rooms door token (66 Readings);
    - *It's a 1/1 Spirit creature with flying in addition to its other types*;
    - the *X/X blue and red Elemental … with flying and haste, where X …* trigger.
- **Judgment set:**
  - 3 entries lose 63 Readings, out of a Surplus of 92.
  - The only entry that today reads only through these schemas is
    `core_composition.txt:6` (landwalk). It must keep its Reading through the
    new catalog parameter.
- **Corpus reach:** 85 ambiguous units have a keyword followed by *, where X*
  or *of*.

## Done when

- None of the examples above attaches anything but a declared parameter to the
  keyword.
- Every one of the following still reads:
  - Traveler's Cloak and the Judgment `core_composition.txt:6`;
  - Avenger of the Fallen;
  - Firebending Student;
  - Sun Warriors;
  - Fire Lord Zuko;
  - Zuko, Firebending Master;
  - That Which Was Compleated;
  - Infantry Shield.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
