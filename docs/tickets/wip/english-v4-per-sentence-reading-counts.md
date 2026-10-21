---
needs: [english-v4-keyword-line-sentence-boundaries, english-v4-mode-bullet-misanalyses]
---
# Census: count Readings per top-level segment and report the surplus

The census gives each Corpus Unit the number of Derivations in its Packed
Forest (`UnitResult.derivations`, totalled in `Report.total_derivations`;
328,186 on `loxuknun`). Many units hold several sentences, or put a label,
cost, or keyword before them, and their count is the product of the parts'
counts. Whiskervale Forerunner's 2,520 measures multiplication rather than
ambiguity: its six segments have counts 1, 35, 9, 2, 1 and 4. Report each top-level
segment's count, and summarize a unit by its surplus.

## Owner's scope ruling

Only a unit's top-level segments are split. These are its top-level sentence
sequence, plus outer structure whose boundaries are just as unambiguous:
- an activation cost before its colon;
- an ability word, flavor word, anchor word, or keyword label before its dash;
- a Keyword Line's items and parameters;
- a modal header, and a bullet's label;
- a chapter, level, or table-row header.

A sentence sequence nested inside anything else is not split. The plainest
case is a quoted ability inside a granted ability: its Readings still multiply
into the enclosing sentence's count.

## Segments

Each unit's Packed Forest is walked from its root. The walk descends through
the outer categories:
- `Document`, `Paragraphs`, `Ability`, `Sentences`, `Effect`, `Mode`, `Modes`,
  `ModalAbility`, `ModeEffect`;
- the Keyword Line categories `KeywordList`, `KeywordLineItem`,
  `KeywordPhrase`, `KeywordParameters`.

The first constituent on each path that is not an outer category is a
segment. So is a lexical leaf of an outer Production, such as an ability-word,
flavor-word, or keyword label. A `Sentence` is always a segment, which is what
keeps nested sequences whole.

A segment is identified by its extent, ignoring structural punctuation and
spaces at its edges (`.`, `•`, `—`, `:`, `|`, `,`). Its Category is not part
of its identity. Two analyses of the same stretch of text in different
Categories are Ambiguity inside that segment, not a different segmentation.
Examples:
- *• Double the number of …* read as a Sentence or as a Noun Phrase mode;
- a bullet's text before the dash read as a Flavor Word label.

A segment's count is the number of Derivations of everything the walk reached
at that extent. For a node that is its `derivation_count` share; for a leaf it
is its number of lexical alternatives.

Evidence, measured on `loxuknun` with a scratch probe that was not committed:
- Over all 88,678 units with a Reading, every outer Production's boundaries
  were stable except in the units of the two needed tickets. That includes
  `ActivatedAbility`, `LabelledAbility`, `FlavorLabelledAbility`,
  `AnchoredAbility`, `ChapterAbility`, `TableRow`, `CommaKeywords`, every
  keyword `Parameters` Production apart from DashAbility items in those
  units, `TextModalHeader`, `ActivatedModalHeader`, `PlusCostedMode`,
  `LabelledCostedMode`, `PawprintMode`, `ModeBullet`, and `SentenceSequence`.
- Two kinds of exception remain:
  - 11 units whose extents differ, all from
    `english-v4-keyword-line-sentence-boundaries`.
  - 42 units with one segmentation but more than one outer structure, so
    their count is not a product. These are its 39 other DashAbility lines and
    the 3 costed bullets of `english-v4-mode-bullet-misanalyses`.
- Everywhere else the unit's count equals the product of its segments' counts,
  exactly.

## Measure: surplus Readings, with the segment counts alongside

A unit's **surplus** is Σ(nᵢ − 1) over its segments' counts nᵢ. A unit is
unambiguous when every segment's count is 1. Equivalently, its surplus is 0,
or `derivations` is 1. The per-segment counts are reported with the surplus.

Alternatives considered:
- **Sum** Σnᵢ: a fully unambiguous two-sentence unit would report 2, so the
  value grows with the number of segments rather than with ambiguity.
- **Max**: hides every ambiguous segment but the worst.
- **One census row per segment**: the Corpus Unit is the census's unit of
  account. Verdicts, faces read, judgments, deferrals, and strata all attach
  to the unit, and a segment cannot be read, known-wrong-only, or deferred on
  its own.

Surplus is the only one of these that is 0 for every unambiguous unit,
whatever its shape. The kept counts still show where the ambiguity is.

`derivations` stays the product. It is the size of the forest, and it is what
`Parse::readings` enumerates, so these keep using it:
- `judgment::READING_LIMIT` (4096) in `Judge::evaluate`;
- the census's `NoReading` verdict (`derivations == 0`).

Measured on `loxuknun`:
- **Surplus:** 175,611, against the current total of 328,186.
- **Ambiguous units:** 17,919, the same as units with a product above 1, as
  expected.
- **Segments:** 19,842 of 117,998 are ambiguous.
- **Fallback:** the 53 units of the needed tickets are counted as
  derivations − 1. Those tickets remove them.

| Card | Surplus | Current product |
|---|---|---|
| Mockingbird | 36708 | 36711 |
| Maralen, Fae Ascendant | 10664 | 10668 |
| Spark Double | 5690 | 5692 |
| Sphere of Annihilation | 3191 | 3194 |
| Activated Sleeper | 2142 | 2145 |
| Magus Lucea Kane | 2026 | 4054 |
| Ziatora's Envoy | 2000 | 2004 |
| Tetsuo, Imperial Champion | 1930 | 1934 |
| Moonhold | 1410 | 1412 |
| Draugr Necromancer | 1249 | 1252 |
| Rootha, Mastering the Moment | 1007 | 1009 |
| Spectral Arcanist | 868 | 871 |
| Sentinel Tower | 839 | 841 |
| Wishing Well | 768 | 770 |
| Grolnok, the Omnivore | 679 | 683 |
| Twinning Glass | 648 | 650 |
| Nissa of Shadowed Boughs | 591 | 595 |
| Rionya, Fire Dancer | 579 | 581 |
| Agility Bobblehead | 578 | 581 |
| The Fourteenth Doctor | 551 | 561 |

Ten cards in the current product top 20 drop out of it under surplus:

| Card | Product | Surplus |
|---|---|---|
| Ryan Sinclair | 3,170 | 82 |
| Whiskervale Forerunner | 2,521 | 46 |
| Betor, Ancestor's Voice | 2,446 | 118 |
| Archaic's Agony | 2,113 | 139 |
| Codie, Vociferous Codex | 2,018 | 133 |
| Kiora, Sovereign of the Deep | 1,738 | 46 |
| Shaman's Trance | 1,458 | 76 |
| Radagast the Brown | 1,449 | 184 |
| Olag, Ludevic's Hubris | 1,311 | 139 |
| Batwing Brume | 1,297 | 118 |

Most of the total is ambiguity inside one segment, not multiplication.

## Change

- `deckmaste_syntax`: add a memoized walk over the Packed Forest beside
  `Parse::derivation_count` in `src/reading.rs`. It uses the same
  `walk::ForestMemo`, so a unary cycle contributes no Derivation.
  - It takes the outer Categories, resolved by name at bind time.
  - For each node it computes the set of segment sequences, each segment
    being an extent. It also computes the count of the outer structure, with
    each segment counted as 1.
  - It returns the unit's segments and their counts only when there is exactly
    one segmentation and the counts' product equals `derivation_count`.
    Otherwise it returns a typed "not segmentable" result, carrying the
    reason: several segmentations, or a non-product outer structure.
  - The set is bounded; past the bound the result is "not segmentable".
- `deckmaste_english_v4`:
  - The list of outer Categories is grammar knowledge. It lives with the
    grammar binding, not in `deckmaste_syntax`.
  - `UnitResult` gains `segments` (each segment's extent and count, or the
    not-segmentable reason) and `surplus`, alongside the unchanged
    `derivations`. A not-segmentable unit's surplus is `derivations − 1`.
  - `Report` gains `total_surplus`, the number of ambiguous units, and the
    number of not-segmentable units. Once the needed tickets land, the
    not-segmentable count is 0; `strata --check` fails on any.
  - `Report::summary` prints the surplus total, the ambiguous-unit count, and
    the product total.
- Consumers to update: `tests/census.rs` (the face-sum, total, and
  sum-versus-product assertions keep their meaning for `derivations`; add the
  surplus equivalents), the `xtask english-v4 parse` output (print the segment
  counts and surplus next to "Packed Forest derivations"; its test counts that
  string), and `strata.json` consumers.
- Unaffected: verdicts (Read, known-wrong-only, unread), faces read (28,709),
  Stratum assignment (`profile.rs` never uses a count), Admission, law 1 and
  law 2 (`best_readings`), and the judgment harness (`derivation_count`). The
  `deckmaste_syntax` contract and engine tests assert `derivation_count` and
  are untouched.
- Census semantics change, so update `crates/deckmaste_english_v4/README.md`
  in the same landing: its census section and the paragraph on the total
  derivation count. Also update `crates/deckmaste_syntax/README.md` for the
  new walk.
- Amend `docs/contexts/oracle-english/CONTEXT.md` (Corpus, Census, and
  Strata), using the `domain-modeling` skill:
  - **Segment**: a top-level stretch of a Corpus Unit delimited by its outer
    structure.
  - **Surplus**: a unit's Σ(nᵢ − 1) over its Segments.

  Do not reuse "Reading" for a segment's count. A segment has Derivations; the
  unit has Readings.
- Commit no probe code: the walk lands as product code with its own tests. The
  tests need a seed-lexicon unit of two sentences each with 2 Readings
  (surplus 2, derivations 4), and a quoted ability whose inner sentences
  multiply into the enclosing sentence.

Standard constraints apply.

Census delta: measured at landing.
Judgment set: run at landing.
