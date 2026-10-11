---
needs: []
---
# Export three plugin terms: if tribute wasn't paid / an enduring story / the exiled cards used to craft it

## Why

Three game terms are declared in `plugins_v2/` but export nothing the grammar
can use in their attested frames. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.txt`), counts by `cargo xtask english-v3
grep` and `blockers-wave9.json`:

- A. *tribute* as a noun: key `at:wasn't paid` 11 sole, 11 touched.
  `tribute wasn.t paid` **11** faces, all unread, all sole. "If tribute
  wasn't paid, you gain 4 life." 0 against "If that cost wasn't paid, you
  draw X cards." 1. Recorded as unowned in
  `english-v3-wave9-declaration-batch-b` (Out of scope).
- B. *an enduring story*: key `at:enduring story` 9 sole, 9 touched.
  `enduring story` **9** faces, all unread, all sole. "As long as you have an
  enduring story, creatures you control get +1/+1." 0 against the same
  sentence with *the city's blessing* 1 and *the initiative* 1.
- C. *craft* as a verb, with the core verb *use*: `word:used` 7 sole, 9
  touched. `used to craft` **9** faces, all unread, 7 sole. NP "the exiled
  cards used to craft it" 0; NP "the cards exiled to craft it" 0 (no verb
  *craft*); NP "the cards exiled with it" 2.

Together 29 faces, 27 with exactly one failing line.

## Goal

*tribute* reads as a noun (Subject of passive *pay*), *enduring story* as a
count noun after *an*, and *craft* as a transitive verb in a purpose
infinitival after the passive participle *used*; each export comes from the
term's own plugin definition. Byte-exact realisation.

## Analysis

No CGEL claim is needed: declarations read by existing routes. Glossary:
**Lexeme**, **Compound Noun**, **Verb Frame**.

Existing declarations (`plugins_v2/builtin/macros/`):
`keyword_abilities/tribute.ron` (`grammar: FixedKeyword(surface:
"tribute")`, no noun); `designations/enduringStory.ron` (`grammar:
FixedTerm(surface: "an enduring story")`), beside
`designations/citysBlessing.ron` (`grammar: Noun(singular: "city's
blessing", plural: Unavailable)`) and `designations/initiative.ron` (`Noun(
singular: "initiative")`), both of which read after *have*;
`keyword_abilities/craft.ron` (`FixedKeyword(surface: "craft")`, no `verb:`),
beside `keyword_abilities/saddle.ron` (`verb: (bare: "saddle", frames:
[Predicate([ObjectNounPhrase])])`). Core: `deckmaste_lexical_source/lexicon/
verbs.ron` has no `Use`. The purpose infinitival and the passive participial
postmodifier are existing grammar (`english-v3-purpose-infinitival`,
*the cards exiled with it*).

Orchestrator rulings 2026-10-11:

- PX1 (B, data only). Change the `enduringStory` Designation's row to the
  `Noun` row on the `citysBlessing` model: `Noun(singular: "enduring story")`
  with the article supplied by the grammar, so *an enduring story* reads as
  determiner + noun. Declare the plural as the paradigm gives it or
  `Unavailable` as the definition's author judges the designation (one per
  player); do not keep the article inside a fixed surface. `spelling:` stays.
- PX2 (A). *tribute* needs a Noun export from a `FixedKeyword`. That
  mechanism is the owner decision recorded as item 4 of
  `english-v3-plugin-term-schema-questions` ("allow `FixedKeyword` to export
  a Noun alongside its existing verb export"). If that field exists at claim
  time, declare the noun on `tribute.ron` through it (mass noun, no article:
  *tribute wasn't paid*). If it does not, do NOT build it here: add *tribute*
  as a second witness to that ticket's item 4 and land B and C alone. Never a
  core noun for a plugin term (R-plugin).
- PX3 (C). Add `verb: (bare: "craft", frames: [Predicate([ObjectNounPhrase
  ])])` to `craft.ron` on the `saddle` model, and declare the core verb `Use`
  (*uses*, *used*) with `Predicate([ObjectNounPhrase])`. *used to craft it*
  is then the passive participle of *use* as postmodifier with a purpose
  infinitival: existing routes, no Construction. If the participial
  postmodifier refuses a purpose infinitival, that licence is read from a
  declared feature (the existing `PurposeAdjunctLicence`), set as data for
  `Use`.
- PX4. The C half does not land on *use* alone (R-wrong): without the
  *craft* verb the phrase cannot read correctly, and a Reading that takes
  *craft* as the keyword noun after *to* is not a gain.
- PX5. One analysis per string (R-one). *used to craft it* is participle +
  purpose clause; it must not also read as the aspectual *used to* (no such
  frame is declared; declare none). *an enduring story* has one Reading; the
  keyword *storied* is untouched. *tribute* the noun must not make the
  keyword line *Tribute 3* read a second way.
- PX6. Declaring `Use` makes *using* and *use* known words on the 32 faces
  of the shelved `english-v3-using-means-adjunct`; they stay unread there
  (no means-adjunct route). If any of them becomes covered, inspect the
  Reading: a *using* gerund attached as an ordinary participial Adjunct is the
  analysis that ticket shelved, so report it rather than count it (R-wrong).
- PX7. Newly covered faces are gains. No Reading-count increase is expected
  on covered faces: the 46 covered *tribute* faces (keyword lines) and the
  covered *craft* face keep their counts.
- PX8. A bodiless or grammar-only change needs no new definition here; if
  one is added anyway, register its facts row or exemption (R-plugin).

## Witnesses

Each is the only failing line of its face in `blockers-wave9.json`. Whole
faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- A. Ornitharch: "When this creature enters, if tribute wasn't paid, create
  two 1/1 white Bird creature tokens with flying."; Snake of the Golden
  Grove: "When this creature enters, if tribute wasn't paid, you gain 4
  life."
- B. Ori, Keeper of Songs: "As long as you have an enduring story, Ori gets
  +1/+0 and has vigilance."; Bombur, Gentle Dreamer: "Bombur doesn't untap
  during your untap step unless you have an enduring story."; Fíli the
  Pathfinder: "As long as you have an enduring story, creatures you control
  get +1/+1."
- C. Jadeheart Attendant: "When this creature enters, you gain life equal to
  the mana value of the exiled card used to craft it."; Cosmium Catalyst:
  "{1}{R}, {T}: Choose an exiled card used to craft this artifact at random.
  …"

Controls that must not change: "If that cost wasn't paid, you draw X cards."
1; "As long as you have the city's blessing, creatures you control get
+1/+1." 1; "As long as you have the initiative, creatures you control get
+1/+1." 1; NP "the cards exiled with it" 2; Document "Tribute 3" with its
present count.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\btribute\b|enduring story|\bstoried\b|\bcraft(s|ed|ing)?\b|\bus(e|es|ed|ing)\b|city's blessing|\binitiative\b

plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261008`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost; PX6
and PX7 for changes. Because `plugins_v2/` changes: the FULL `cargo test -p
xtask`, `-p deckmaste_semantics_v2`, `-p deckmaste_construction_core`, `-p
deckmaste_lexical_source`, `-p deckmaste_english_v3`, clippy `--tests -- -D
warnings` for each, and `cargo xtask lean check`. Plugin `grammar:` rows and
the lexicon load at runtime: no rebuild for data; the six English test
binaries take about 75 seconds. Tests assert the role (*tribute* as Subject
of the passive, *enduring story* as head noun of the Object of *have*,
*craft* as the verb of the purpose clause inside the *used* postmodifier),
never an incidental count or a Debug-printed tree. Attestation by `cargo
xtask english-v3 grep` only.

## Out of scope

- The keyword line *Craft with artifact {3}{W}{W}* (24 faces, each its face's
  only failing line): `english-v3-quality-cost-keyword-lines` (in flight). If
  that landing adds the `verb:` export to `craft.ron`, PX3 shrinks to the
  verb `Use`.
- *using* as a means Adjunct: `english-v3-using-means-adjunct` (shelved).
- *a card type shared among two exiled cards used to craft it* (Apex
  Observatory) if *shared among* fails.
- Party, Roles, day/night, gift, bands, empower: held in
  `english-v3-plugin-term-schema-questions`.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
