---
needs: []
---
# Read leveler band lines: LEVEL 2-6 / 3/3 / First strike

## Why

A leveler card prints, after *Level up {cost}*, two bands of three lines: a
band label (*LEVEL 2-6*, *LEVEL 7+*), a bare power/toughness line (*3/3*) and
the band's abilities. The label and the P/T line do not read: Document "LEVEL
1-2" 0, "4/4" 0, and the whole of Student of Warfare 0, against "Level up
{W}" + "First strike" + "Double strike" 1. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.json`): keys `at:<start> level` 25 touched
faces and `at:N <end>` (`at:4 <end>` 13, `at:3 <end>` 8, …), never sole (four
such lines per card), so absent from the sole-blocker tables. `cargo xtask
english-v3 grep`: `(?m)^LEVEL \d+(-\d+|\+)$` **25** faces (50 lines), all
unread; `(?m)^\d+/\d+$` the same 25 faces (50 lines). On **24** of the 25
every failing line is one of the two shapes.

## Goal

A band label line and a bare power/toughness line each read as a line of the
Document, the label word coming from the level-up keyword's declared block
label. Byte-exact realisation.

## Analysis

No CGEL basis: both lines are Oracle Notation (glossary **Notation**,
**Document**, **Keyword Line**). Project ruling throughout.

Existing declarations: `plugins_v2/builtin/macros/keyword_abilities/
levelUp.ron`, `grammar: FixedKeyword(surface: "level up", block_label:
(surface: "LEVEL"))`; `deckmaste_construction_core/src/macro_def.rs` carries
`block_label` (`BlockLabelGrammar`, `SurfaceFeature::BlockLabel`), but no
Construction in `crates/deckmaste_english_v3/src/declarations.rs` consumes it.
Nearest grammar: `KeywordLine` (`boundary Initial`, `[first: KeywordPhrase,
rest: …]`), `SlashPair` (`[left: ScalarComponent, "/", right:
ScalarComponent]`), `AbilityWordHead` as the label precedent.

Orchestrator rulings 2026-10-11:

- LB1. New Construction, stated up front (R-budget): one Ability-level
  Construction with two forms, the band label line `[label, " ", range]` and
  the bare `SlashPair` line, buying 24 faces (25 touched). If the range label
  of `english-v3-numeric-range-rows` exists at claim time, reuse it and add
  the hyphen form (`2-6`) there; the open form (`7+`) is shared.
- LB2. The label word is read from the keyword definition's declared
  `block_label` surface through the lexical interface (R-plugin, R-class:
  any keyword that declares a block label gets the line; no spelling in the
  grammar). If `block_label` does not reach the lexical reader today, wiring
  it is in scope and is reported as a deviation.
- LB3. The P/T line is unsigned: both components are unsigned numerals.
  Read the declared sign feature (`ScalarSign`) rather than adding a second
  pair category.
- LB4. One analysis per string (R-one). *3/3* on its own line has one
  Reading (the band's P/T); it must not also read as a cost, a keyword
  parameter or a Flavor Word. The band's ability lines (*First strike*) keep
  exactly the Readings they have as ordinary lines: the grammar does not
  group a band's three lines into one constituent (per-line census; the
  grouping is semantics).
- LB5. No check that a band label follows *Level up* or precedes a P/T line
  (R-prune: such a check would only block unattested layouts).
- LB6. Newly covered faces are gains. No covered face has a line matching
  either shape, so no Reading-count change on a covered face is expected; any
  change is a defect.

## Witnesses

Whole faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Their only failing lines are the band lines. Re-probe on
the claim parent first.

- Student of Warfare: "Level up {W}" / "LEVEL 2-6" / "3/3" / "First strike" /
  "LEVEL 7+" / "4/4" / "Double strike".
- Zulaport Enforcer: "Level up {4}" / "LEVEL 1-2" / "3/3" / "LEVEL 3+" /
  "5/5" / "This creature can't be blocked except by black creatures."
- Coralhelm Commander: "LEVEL 2-3" / "3/3" / "Flying" / "LEVEL 4+" / "4/4" /
  "Flying" / "Other Merfolk creatures you control get +1/+1."
- Hexdrinker: "LEVEL 3-7" / "4/4" / "Protection from instants" / "LEVEL 8+" /
  "6/6" / "Protection from everything".

Line probes: "LEVEL 1-2" 0; "4/4" 0. Controls that must not change: Document
"Level up {1}{W}" 1; "Level up {W}" + "First strike" + "Double strike" 1;
"Flying" 1; "Create a 3/3 green Beast creature token." (the `SlashPair`
premodifier) with its present count.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\blevel\b` (62) plus 300 random covered controls (`cargo
xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. If LB3 changes `SlashPair` or
`ScalarComponent` themselves rather than reading them, R-reach applies: add
every face matching `\d+/\d+` and say so. Zero lost; LB6 for changes. Full
`cargo test -p deckmaste_english_v3` and clippy `--tests -- -D warnings`; if
`plugins_v2/` or the lexical reader changes, the FULL `cargo test -p xtask`,
`-p deckmaste_semantics_v2`, `-p deckmaste_construction_core`, `-p
deckmaste_lexical_source` and `cargo xtask lean check` as well. A rebuild
after a grammar edit takes about 2–3 minutes, the six test binaries about 75
seconds. Tests assert the role (the block label, the range, the P/T pair as a
line, the LB4 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Class levels (*{2}{U}: Level 2*): `english-v3-class-level-lines`.
- d20 and station rows: `english-v3-numeric-range-rows`.
- A band ability that fails for its own reason (1 of the 25 faces).
- Grouping bands for semantics.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
