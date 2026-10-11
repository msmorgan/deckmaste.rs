---
needs: []
---
# Read Class level lines and *becomes level N*: {2}{U}: Level 2 / When this Class becomes level 2, draw two cards.

## Why

Every Class card has two level lines of the form *cost*`: Level N`, and many
have a trigger on *becomes level N*. Neither reads: Document "{2}{U}: Level
2" 0, "When this Class becomes level 2, draw two cards." 0, NP "level 2" 0,
against "{2}{U}: Draw a card." 1, "When this Class enters, draw two cards." 1
and NP "mana value 2" 1. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.json`): keys `at:: level` 34 and
`at:becomes level` 11 touched faces, never sole (two level lines per card), so
absent from the sole-blocker tables. `cargo xtask english-v3 grep`:
`(?m): Level \d+$` **34** faces (68 lines), all unread; `becomes level \d` 11
faces, all among the 34. On **28** of the 34 every failing line is one of
these two shapes.

## Goal

*Level N* reads as the body of an activated ability line, and *level N* as
the predicative Complement of *become*, the level term coming from its plugin
definition. Byte-exact realisation.

## Analysis

No CGEL basis for the line form: *cost*`: Level N` is Oracle Notation
(glossary **Notation**, **Document**). *level 2* as a noun with a postposed
numeral value follows the existing valued-property analysis (*mana value 2*);
calling the level a valued property is a project ruling.

Existing declarations: `plugins_v2/builtin/macros/designations/level.ron`
(`Designation`, `grammar: FixedTerm(surface: "level")`, `scope:
HeldBy(Object)`); `counter_kinds/levelCounter.ron` (NP "a level counter" 1);
`keyword_abilities/levelUp.ron`. Grammar
(`crates/deckmaste_english_v3/src/declarations.rs`): `ActivatedAbility`
(`[cost: Cost, ": ", body: Paragraph]` and the unterminated-Sentence form);
`MeasuredAttribute` (property noun + value, reads *mana value 2*); the
*become* frames in `deckmaste_lexical_source/lexicon/verbs.ron` ("This
creature becomes monstrous." 1).

Orchestrator rulings 2026-10-11:

- CL1. The level term is plugin-owned (R-plugin). Change the `level`
  Designation's `grammar:` row so that it exports a noun the valued-property
  route reads (the existing `Noun` row, with whatever declared feature marks a
  noun as taking a postposed numeral value); keep one lexical owner for the
  surface *level* in this sense, distinct from the counter-kind term in *level
  counter*. If a `Designation` cannot export that feature, extend the
  Designation family's export recipe (every Designation whose value is a
  number gets it), never a row naming *level* (R-class).
- CL2. *level N*: refine `MeasuredAttribute` or its licence so that it reads
  the exported noun with an Arabic numeral value. No new Construction for the
  NP.
- CL3. The line `{2}{U}: Level 2`: a form on `ActivatedAbility` whose body is
  the valued-property NP without terminal punctuation, licensed by a declared
  feature of the noun (the same one as CL1), not by its spelling. This is a
  form on an existing Construction (R-budget); if it cannot be expressed as
  one, say so before adding a Construction, with the 34 faces it buys.
- CL4. *becomes level 2*: the valued-property NP as predicative Complement of
  *become*, through the existing predicative frame; no frame naming the noun.
  Note "Its mana value becomes 2." reads 0 today: that is a different gap (a
  bare numeral Complement) and is not part of this ticket.
- CL5. One analysis per string (R-one). `{2}{U}: Level 2` has one Reading:
  the body is the level label, not an imperative with a verb *level*, and not
  a Flavor Word line. *becomes level 2* has the predicative Reading only;
  *level* must not read there as the counter-kind term.
- CL6. Capitalisation: the line prints *Level*, the trigger *level*; both are
  the same lexeme under the existing capitalisation handling. No second
  surface.
- CL7. Newly covered faces are gains. No covered face matches `\blevel \d`
  (59 faces, all unread); the three covered faces containing *level* (*level
  counter*, *level up* mentions) must keep their Readings. Any increase is a
  defect.

## Witnesses

Whole faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Their only failing lines are level lines. Re-probe on the
claim parent first.

- Wizard Class: "{2}{U}: Level 2" / "When this Class becomes level 2, draw
  two cards." / "{4}{U}: Level 3" (each line 0).
- Bard Class: "{R}{G}: Level 2" / "{3}{R}{G}: Level 3".
- Stormchaser's Talent: "{3}{U}: Level 2" / "When this Class becomes level 2,
  return target instant or sorcery card from your graveyard to your hand." /
  "{5}{U}: Level 3".
- Druid Class: "{2}{G}: Level 2" / "{4}{G}: Level 3" / "When this Class
  becomes level 3, target land you control becomes a creature with haste and
  …" (the last line may have its own blocker; report it).

Controls that must not change: "{2}{U}: Draw a card." 1; "When this Class
enters, draw two cards." 1; "This creature becomes monstrous." 1; NP "mana
value 2" 1; NP "a level counter" 1; Document "Level up {1}{W}" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\blevel\b` case-insensitively (62) plus every face whose
baseline Readings use `MeasuredAttribute` if CL2 changes its form or
requirements (R-reach; select by `\b(mana value|power|toughness) \d`), plus
300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261008`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost; CL7
for increases; digest-only changes by class. Because `plugins_v2/` changes:
the FULL `cargo test -p xtask`, `-p deckmaste_semantics_v2`, `-p
deckmaste_construction_core`, `-p deckmaste_lexical_source`, `-p
deckmaste_english_v3`, clippy `--tests -- -D warnings` for each, and `cargo
xtask lean check`. Plugin `grammar:` data loads at runtime; a grammar edit
costs a 2–3 minute rebuild and the six test binaries about 75 seconds. Tests
assert the role (the level NP as the body of the activated line, as
predicative Complement of *become*, the CL5 negatives), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only.

## Out of scope

- Leveler bands (*LEVEL 2-6*, a bare *3/3* line):
  `english-v3-leveler-level-bands`.
- Class lines that fail for another reason (6 of the 34 faces).
- A bare numeral Complement of *become* (*Its mana value becomes 2.*).
- Semantics of gaining a level.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
