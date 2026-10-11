---
needs: []
---
# Read Quality-then-Cost keyword lines: Equip legendary creature {1}; Craft with artifact {1}{U}

## Why

Two keywords take a Quality and then a Cost. *Splice onto Arcane {1}{R}*
reads (1); *Equip* + Quality + Cost and *Craft with* + materials + Cost do
not. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.json`): `at:{1} <end>` (12 of 12),
`at:{u} <end>` (6 of 8), and the other `at:{…} <end>` keys. `cargo xtask
english-v3 grep '(?m)^Craft with'` finds **24** faces, all unread, **24**
sole. `grep '(?m)^Equip [^{\n—"]*[a-z] \{'` finds **31** faces, **22** unread;
21 failing lines match, **21** sole (the 9 covered are sentences that begin
*Equip abilities …* / *Equip costs …*). Together **45** sole-blocker faces.
Equip's Qualities: *legendary creature* 5, single subtypes 9, *commander* 2,
*planeswalker*, *creature token*, *Shaman, Warlock, or Wizard*. Craft's
materials: *artifact* 10, *creature* 2, *Island*, *Cave*, quantified NPs 7
(*one or more Dinosaurs*, *four or more nonlands with activated abilities*),
*two that share a card type* 1, *one or more* 1.

## Goal

A keyword phrase `<keyword> [marker] <Quality> <Cost>` reads for a keyword
whose declaration selects a Quality and a Cost, with or without a selected
Preposition, byte-exact. Bare-Cost *Equip {1}* keeps its Reading.

## Analysis

No CGEL citation applies: keyword lines are Oracle layout. Project rulings
throughout.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`QualityCostKeyword` (`[head: lexical(Keyword), " ", quality:
KeywordQualityPreposition, keyword_parameter_separator(…), cost:
CostSymbols]`, `KeywordParameterClass = QualityCost`), which reads splice;
`KeywordQualityPreposition` with table `keyword_quality_marker` (`For`,
`From`, `Onto`; `KeywordMarker` already has a `With` value);
`KeywordQuality` built by `AdjectivalKeywordQuality`,
`NominalKeywordQuality`, `NounPhraseKeywordQuality`; `CostKeyword` for bare
*Equip {1}*. Plugin side: `equip.ron` (`params: [Cost]`, `FixedKeyword(surface:
"equip", participial_adjective: …)`), `craft.ron` (`params: [Cost]`, with a
STOP comment: "waits on: a second parameter for craft's materials"),
`splice.ron` (`params: [Quality, Cost]`, `parameter: Quality(preposition:
("Preposition", "Onto"))`).

Orchestrator rulings 2026-10-11:

- QC1. Refine `QualityCostKeyword`; no new Construction (R-budget). The
  marked case is data plus one table row: `With` joins
  `keyword_quality_marker`. The unmarked case (Equip) is a second form of the
  same Construction with the Quality directly after the head, selected by the
  head's declared `KeywordMarker = None`. If the compiler cannot give the two
  forms the same named fields, STOP with the shape it needs.
- QC2 (R-plugin). Craft: `craft.ron` becomes `params: [Quality, Cost]` with
  `parameter: Quality(preposition: ("Preposition", "With"))`; its STOP comment
  is resolved for the grammar and stays for the body. Equip-with-Quality is a
  separate definition beside `equip.ron` (the typecycling and dashed-partner
  precedent: one definition per parameter recipe), grammar-only if it has no
  body yet, with its facts registration or exemption. Do not give one
  Lexeme two parameter classes.
- QC3. The licence reads `KeywordParameterClass`, `KeywordMarker` and the
  existing `KeywordQuality` exports. No test on *equip*, *craft*, *with*, a
  subtype or a card.
- QC4 (R-class). Every Quality the three `KeywordQuality` Constructions
  build is admissible; there is no list of attested Qualities. Coordinated
  Qualities (*Shaman, Warlock, or Wizard*) use the existing quality
  coordination. Craft's quantified materials (*one or more Dinosaurs*) are
  `NounPhraseKeywordQuality`; if `nominal_number` or `Targeting` blocks one,
  report it, do not loosen by word.
- QC5 (R-one). One analysis per string. The twins to avoid: (a) *Equip* as
  the imperative verb with a bare NP Object and a stray Cost; (b) the Quality
  swallowing the Cost symbols; (c) bare *Equip {1}* gaining a second Reading
  through the new form with an empty Quality.
- QC6. "Craft {1}{U}" reads 1 today and is unattested (`grep '(?m)^Craft \{'`:
  check, expected 0). After QC2 it stops reading: a string no card prints,
  not a loss; add no route to keep it.
- QC7. Fused materials (*Craft with two that share a card type*, *Craft with
  one or more*, 2 faces) need `english-v3-fused-cardinal-head` (held): out of
  scope.
- QC8. Newly covered faces are gains. No Reading-count change is expected on
  covered faces; R-reach applies because `QualityCostKeyword` and
  `keyword_quality_marker` change: the selector includes every face whose
  baseline uses them (splice, and every `For`/`From`/`Onto` keyword), and
  digest-only changes are reported by class.

## Witnesses

Each is the whole failing line of a face whose only failing line it is.
Probed 2026-10-11 with the trunk binary (`--category document --readings 8`):
all 0.

- Ace's Baseball Bat: "Equip legendary creature {1}"
- Steelclaw Lance: "Equip Knight {1}"
- Robe of the Archmagi: "Equip Shaman, Warlock, or Wizard {1}"
- Braided Net: "Craft with artifact {1}{U}"
- The Enigma Jewel: "Craft with four or more nonlands with activated
  abilities {8}{U}"

Controls that must not change: "Equip {1}" 1; "Equip—Pay 3 life." 1; "Splice
onto Arcane {1}{R}" 1; "Affinity for artifacts" 2; "Protection from
legendary creatures" 2. Changes under QC6: "Craft {1}{U}" 1 → 0.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?im)^(equip|craft|splice|affinity|protection|champion)\b|\b(equip|craft)\b`
plus 300 random covered controls (`--random 300 --seed 20261008`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; QC8 for changes. Because
`plugins_v2/` changes: FULL `cargo test -p xtask`, `-p
deckmaste_semantics_v2`, `-p deckmaste_construction_core`, `-p
deckmaste_english_v3`, clippy `--tests -- -D warnings` for each, and `cargo
xtask lean check`, once, at the end (a grammar rebuild takes 2–3 minutes, the
six test binaries about 75 seconds; plugin `grammar:` data loads at runtime).
Tests assert the role (Quality and Cost are the keyword's two parameters; the
QC5 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- QC7's two fused-material faces.
- *Commander ninjutsu {U}{B}* (1), *Modular—Sunburst* (1), *Kicker {1}{W}
  and/or {1}{U}* (5 sole), *Cumulative upkeep {W} or {U}* (4), *Suspend
  X—{X}{G}{G}.* (5): other keyword-line shapes, counted in the wave-10 triage.
- Engine bodies for craft and equip-with-Quality.

## Landing record
