---
needs: [english-v4-comma-adjunct-island]
---
# A multiword term whose parts are ordinary words may split at a constituent boundary

Owner ruling (round 3, item 3.3), from `english-v4-comma-adjunct-island`.

## The defect

`crates/deckmaste_syntax/src/chart.rs` `atomic_spans` treats every multiword
Noun lexeme as atomic. `run` drops every leaf that starts or ends strictly
inside that span.

Witness, Stalwart Successor:

> Whenever one or more counters are put on a creature you control, if it's the
> first time counters have been put on that creature this turn, put a +1/+1
> counter on that creature.

*Time counters* is the plural of the catalog counter `time` (`counters.ron`;
[CR#702.62a]). *Time* and *counters* therefore never reach the chart as
separate words.

The intended analysis cannot be built:
- the *if* phrase is a preposed Adjunct of *put* [CR#603.4];
- inside it, the relative-time nominal is [the first time] [counters have been
  put on that creature this turn].

On `pzvrlpou` the unit has 4 Readings, all of them wrong. Under the
`english-v4-comma-adjunct-island` candidate it has 0, so the face goes unread
when that ticket lands (round 3, item 3.2). If `deferred.txt` names the unit,
this landing removes that entry.

## Ruling

- Allow *time counters* to split into *time* and *counters* where the two
  words straddle a constituent boundary.
- Amend the atomicity bullet in `docs/decisions/english-lexical-analysis.md`
  §2, "A multiword game term that a plugin or catalog declares as one term is
  atomic". The new text says that a multiword catalog term does not suppress
  the single-word tokens when its parts are ordinary lexicon words that can
  straddle a constituent boundary.
- The implementer writes that amendment in this landing.

## Change

- Keep the inner leaves of a multiword term whose parts each have their own
  lexical analysis, but only where the parts straddle a constituent boundary.
- The parts must never rebuild the term's own analysis, such as a compound
  Nominal [time] [counters] beside the atomic *time counters*. That would be a
  second tree for the same analysis.
- Whether the rule also covers Core terms (*mana value*, *combat damage*) is
  the implementer's call. Either way, the census decides it: no unit gains a
  Reading.

## Reach

Measured on `pzvrlpou` with a scratch scan of the `atomic_spans` condition
over all 92,934 units:
- 9,539 units contain a multiword Noun term that `atomic_spans` protects,
  with 11,180 occurrences.
- In 5,013 units, every part of at least one such term is also an ordinary
  lexicon word.
  - **Catalog terms:** 2,341 units, 62 terms, 2,475 occurrences. The largest
    are *end step* 996, *untap step* 394, *beginning of combat* 309, *end of
    combat* 115, *time counter* 110, *draw step* 72, *combat phase* 69 and
    *loyalty counter* 64.
  - **Core terms:** 2,801 units, 6 terms, 3,083 occurrences. The largest are
    *mana value* 1,229, *combat damage* 1,139, *mana cost* 552 and *life
    total* 146.
- *Time counter(s)* occurs 110 times in 96 units.

## Acceptance set

- Stalwart Successor reads, with a `+` Judgment:
  - *if it's the first time counters have been put on that creature this
    turn* is an Adjunct of *put*;
  - `Clause "counters have been put on that creature this turn"` is in that
    *if* phrase;
  - no Reading has *time counters* as one Nominal.
- No other unit of the 5,013 gains a Reading.
- In particular, the 96 units with a real *time counter(s)* keep their counts.
  A `+` Judgment on one of them (*Put three time counters on this Aura.*,
  Tourach's Gate) asserts *time counters* as one Nominal and `readings 1`.

## Done when

- `english-lexical-analysis.md` §2 is amended as ruled.
- Stalwart Successor reads.
- No new feature in `english.ron`. If the fix needs one, STOP and return to
  the owner.
- No face is lost, and no unit other than the witness gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
