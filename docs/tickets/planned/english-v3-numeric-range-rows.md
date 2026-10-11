---
needs: []
---
# Read a numeric-range row: 1—9 | Draw a card. / 20 | … / 8+ | Flying

## Why

No row of the form *range* `|` *text* reads: Document "1—9 | Draw a card." 0,
"20 | Draw a card." 0, "8+ | Flying" 0, although "Draw a card." and "Flying"
read. Two card families use it: d20 result tables (after *Roll a d20.*) and
Spacecraft / Planet station thresholds (after the keyword line *Station*).
Listed as unowned in `english-v3-polyhedral-die-nouns` (Out of scope: "result
tables (*1—9 |*)"). Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.json`): keys `at:N +` (failing token `+`:
24 sole, 39 touched), `at:1 —` 39, `at:10 —` 33, `at:20 |` 25 touched (a
table has two or more rows, so table faces are never sole-blocker faces).
`cargo xtask english-v3 grep '(?m)^(\d+(—\d+)?\+?) \|'` finds **73** faces,
147 rows, all unread; on **68** of them every failing line is such a row, and
23 have exactly one failing line:

- result rows `N—M |` and `N |`: 42 faces (101 rows), 31 with no other
  failing line;
- threshold rows `N+ |`: 38 faces (46 rows), 31 with no other failing line,
  23 single-line. Row bodies: keyword lists (*Flying, lifelink*) 25, activated
  abilities 7, other sentences and triggered abilities 15. Seven faces
  (Diviner's Portent, Revivify, Wyll's Reversal, …) use `15+ |` inside a d20
  table.

## Goal

A row consisting of a numeric range label, the bar separator and an Ability
reads as one line of the Document, with byte-exact realisation; the label is
a single numeral, a closed range or an open-ended threshold.

## Analysis

No CGEL basis: the row is Oracle document Notation (glossary **Notation**,
**Document**, **Keyword Line**). Project ruling throughout.

Nearest existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`AbilityWordHead` (`[head: lexical(Keyword), " — ", body: Ability]`, `boundary
Initial`, `require body.LabelKind = None`) is the model of a label in front of
an Ability; `KeywordLine`, `ActivatedAbility` and `OrdinaryAbility` supply the
row bodies; `NumericCostSymbol` and `NumericQuantityConjunct` read Arabic
numerals (`numeral_kind = Arabic`). The feature `LabelKind { None,
AbilityWord, FlavorWord }` is what keeps labels from stacking.

Orchestrator rulings 2026-10-11:

- NR1. New Construction, stated up front (R-budget): one Construction for
  the labelled row, `[label: range, " | ", body: Ability]`, buying 68 faces
  (73 touched). The range label is one more small category or three forms on
  that Construction (`N`, `N—M`, `N+`); prefer forms. Nothing existing has a
  bar-separated label, so a refinement is not available; if the implementer
  finds one, use it and say so.
- NR2. The licence reads declared features: `numeral_kind = Arabic` on the
  numerals, `LabelKind = None` on the body, and a new `LabelKind` value (or an
  equivalent declared feature) on the row so a row cannot be the body of an
  Ability Word or Flavor Word label nor of another row. No test on *d20*,
  *Station* or a card. The grammar does not check that a table follows a roll
  or that thresholds follow *Station* (R-class: the row is licensed wherever
  an Ability line is; no restriction whose only job is to block an unattested
  placement).
- NR3. The dash in a closed range is the em dash the cards print (`1—9`).
  The hyphen form (`LEVEL 2-6`) belongs to `english-v3-leveler-level-bands`;
  if that ticket lands first, share its range label rather than declaring a
  second one.
- NR4. One analysis per string (R-one). The row body reads exactly as the
  same text reads as a stand-alone line: "8+ | Flying" has as many Readings
  as "Flying" (1), "1—9 | Draw a card." as many as "Draw a card.". The label
  must not also read as a Flavor Word (`FlavorWordHead`) or as a cost.
- NR5. Row bodies that are several sentences are one Paragraph, as in any
  other Ability; no row-specific sentence handling.
- NR6. Newly covered faces are gains. No covered face contains ` | ` at the
  start of a line (grep: 0 covered), so no Reading-count change on a covered
  face is expected; any change is a defect.

## Witnesses

Whole faces, probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- Wurmwall Sweeper: "When this Spacecraft enters, surveil 2." / "Station" /
  "4+ | Flying" (0; its only failing line is the row).
- The Seriema: … / "Station" / "7+ | Flying" / … (0; sole line).
- The Eternity Elevator: "20+ | {T}: Add X mana of any one color, where X is
  the number of charge counters on The Eternity Elevator." (sole line).
- Arcane Investigator: "Search the Room — {5}{U}: Roll a d20." / "1—9 | Draw
  a card." / "10—20 | Look at the top three cards of your library. Put one of
  them into your hand and the rest on the bottom of your library in any
  order." (0; the two rows are its only failing lines).
- Earth-Cult Elemental: "1—9 | …" / "10—19 | …" / "20 | Each opponent
  sacrifices two permanents of their choice." (single-numeral label).

Row probes: "1—9 | Draw a card." 0; "20 | Draw a card." 0; "8+ | Flying" 0;
"15+ | Scry X, then draw X cards." 0. Controls that must not change:
"Flying" 1; "Station" 1; "{2}{U}: Draw a card." 1; "Roll a d20." 1; and every
covered `AbilityWordHead` / `FlavorWordHead` line in the selector.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?m)^(\d+(—\d+)?\+?) \|` (73) plus every supported face
matching `\bd20\b|\b[Ss]tation\b` (the hosts) plus 300 random covered controls
(`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; NR6 for changes. Full `cargo test
-p deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests --
-D warnings`, once, at the end; a rebuild after a grammar edit takes about 2–3
minutes, the six test binaries about 75 seconds. Tests assert the role (the
range as label, the body as the Ability it is elsewhere, the NR4 negatives),
never an incidental count or a Debug-printed tree. Attestation by `cargo
xtask english-v3 grep` only.

## Out of scope

- Row bodies that fail for their own reason (5 of 73 faces have another
  failing line; some of the 68 may have a body that fails once the label
  reads: report them, do not chase them).
- *LEVEL N-M* bands and bare P/T lines: `english-v3-leveler-level-bands`.
- *{cost}: Level N*: `english-v3-class-level-lines`.
- *natural 20*, *the result minus 1* and other dice vocabulary.
- Saga chapter labels (*I —*, *II, III —*): recorded in
  `english-v3-systemic-residuals` ("Faces covered by a known wrong analysis").

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
