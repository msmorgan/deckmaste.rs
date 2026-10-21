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

## Owner's ruling (round 2)

#6 just deletes the `Adjunct` Schema on `KeywordPhrase`. There are two such
Schemas, the Nominal postmodifier and the comma-set Supplement; delete both.

**No new features in `english.ron`.** The fix also adds no new Schema or
Production for the rehoming below. If the fix cannot be done that way, STOP
and return to the owner.

## What the deleted Schemas carry today, and where it goes

The Schemas also carry real Oracle text. Rehome each use only through catalog
frames or existing productions.

- **A parameter of a bare mention.**
  - *Enchanted creature has landwalk of the chosen type.* (Traveler's Cloak,
    and Judgment `core_composition.txt:6`) reads only through the Nominal
    Schema today.
  - Landwalk's catalog entry ([CR#702.14a,702.14c]) declares only a bound
    stem.
  - Declare the *of* + Noun Phrase quality of the mention as a Keyword
    Parameter frame in `catalogs/keyword_abilities.ron`, which the existing
    keyword-parameter schemas read.
- **A *where* definition of a Keyword Parameter X.**
  - On a Keyword Line:
    - *Mobilize X, where X is …* (Avenger of the Fallen);
    - *Firebending X, where X is …* (Firebending Student, Sun Warriors, Fire
      Lord Zuko, Zuko, Firebending Master);
    - *Annihilator X, where X is …* (That Which Was Compleated).
  - Inside a sentence: *Equipped creature has menace and mobilize X, where X
    is its power* (Infantry Shield), which the existing Clause Supplement
    Schema already hosts.
  - For a Keyword Line, find an existing production that hosts the Supplement
    on the line or its item, or a catalog parameter frame that takes it
    ([CR#702.86a,702.181a,702.189a] define those parameters as N).
  - If no existing production or catalog frame can carry it, STOP. Do not add
    a Schema.

## Change

- Delete both `KeywordPhrase` Adjunct Schemas.
- Rehome the uses above as stated, with no new feature, Schema or Production.

## Owner's ruling (round 3)

No existing production hosts a *where* Supplement on a Keyword Line, and no
catalog parameter kind takes one. The owner approved narrowing the comma-set
Supplement's host from `KeywordPhrase` to `KeywordLineItem`. A Supplement
anchors to a Clause or line (glossary, "Host Category"), so this keeps the
Keyword Line faces without letting a keyword in a sentence take a Supplement.

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
- No new feature in `english.ron`.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

Census delta (measured on `tutlnwqy` → `rmszzxqu`): faces read 28,709 → 28,709, total Readings 318,400 → 316,018, Surplus 170,281 → 168,240; lost faces: none.
Judgment set: `cargo test -p deckmaste_english_v4` passes; Traveler's Cloak rewritten, 3 entries added, none retired.
