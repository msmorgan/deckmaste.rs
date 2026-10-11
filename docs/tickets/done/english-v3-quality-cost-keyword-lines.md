---
needs: []
---
# Read unmarked Quality-then-Cost keyword lines: Equip legendary creature {1}

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

An unmarked keyword phrase `<keyword> <Quality> <Cost>` reads for a keyword
whose declaration selects a Quality and a Cost, byte-exact. Existing marked
QualityCost routes and bare-Cost *Equip {1}* keep their Readings. QC9 defers
Craft to `english-v3-craft-with-quality-cost`.

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

- QC9 (orchestrator, 2026-10-11). Land the unmarked Quality+Cost form and
  separate grammar-only EquipQuality definition with its facts exemption.
  Preserve every existing keyword signature. Remove the With marker rows,
  which only serve Craft here. Defer Craft to
  `english-v3-craft-with-quality-cost`; its 24 faces stay unread. The owner
  and semantics-v2 line must decide whether a grammar ticket may alter Craft
  and regenerate the v1 facts row. QC9 supersedes QC2's Craft instruction
  and QC6, and resolves the implementation STOP by splitting the ticket.

## Witnesses

Each is the whole failing line of a face whose only failing line it is.
Probed 2026-10-11 with the trunk binary (`--category document --readings 8`):
all 0.

- Ace's Baseball Bat: "Equip legendary creature {1}"
- Steelclaw Lance: "Equip Knight {1}"
- Robe of the Archmagi: "Equip Shaman, Warlock, or Wizard {1}"
Deferred witnesses are recorded in `english-v3-craft-with-quality-cost`: Braided
Net and The Enigma Jewel remain unread under QC9.

Controls that must not change: "Equip {1}" 1; "Equip—Pay 3 life." 1; "Splice
onto Arcane {1}{R}" 1; "Affinity for artifacts" 2; "Protection from
legendary creatures" 2. Under QC9, "Craft {1}{U}" retains 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?im)^(equip|craft|splice|affinity|protection|champion)\b|\b(equip|craft)\b`
plus 300 random covered controls (`--random 300 --seed 20261008`). Craft
remains in this selector to verify all 24 deferred faces stay unread. Use
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

- Craft, including QC7's two fused-material faces; tracked in
  `english-v3-craft-with-quality-cost` under QC9.
- *Commander ninjutsu {U}{B}* (1), *Modular—Sunburst* (1), *Kicker {1}{W}
  and/or {1}{U}* (5 sole), *Cumulative upkeep {W} or {U}* (4), *Suspend
  X—{X}{G}{G}.* (5): other keyword-line shapes, counted in the wave-10 triage.
- Engine bodies for craft and equip-with-Quality.

## Landing record

1. **Census delta — `vwntqpupwqyw`, claim-parent runtime `wlslkrom`.**
   Bounded manifest: 1,399 faces. Method grep selects 915; declaration reach
   adds 187 outside that pattern (419 claim-parent users); the unmarked
   AfterHead class adds Crabomination; 300 random covered controls use seed
   20261008. BEFORE is the original claim-parent census expanded with exact
   parent runtime bytes; official diff leaves all 1,398 shared faces unchanged.
   Covered 1,271 → 1,292; Readings 15,804 → 15,988; gained 21; lost none;
   decreases 0; increases 0; digest-only 0 (all classes, 0 covered lines);
   issues 0. All 24 Craft faces remain unread under QC9. Evidence:
   `target/english-v3/english-v3-quality-cost-keyword-lines/{before,after,diff}.json`.
   Post-refresh comparison with that original claim parent: covered 1,271 →
   1,298; Readings 15,804 → 16,005; gained 27; lost/decreases/increases/
   digest-only/source changes/issues all 0. The official `refresh-diff.json`
   comparing the saved QC9 AFTER with refreshed AFTER attributes six gains
   (17 Readings) to trunk's `zquvqlrmolsk` additional-adjustment landing:
   Bride's Gown, Groom's Finery, Heavy Mattock, Silver-Inlaid Dagger,
   Tenza, Godo's Maul and True-Faith Censer. Ticket gains remain 21.
   Both runs use the same 1,399-face manifest; digest-only classes each have
   0 changed covered lines. Integration-turn refresh (`opkxokpkrpqs`, Partner
   with Name): original 1,399-face selector retained; official comparison
   against reviewed AFTER reports all 1,399 faces unchanged, covered 1,298,
   Readings 16,005; 27 total gains (21 ticket, 6 prior trunk), all regression
   and digest-only classes 0.
2. **Witnesses — `vwntqpupwqyw`.** Ace's Baseball Bat, Steelclaw Lance and
   Robe of the Archmagi: each 0 → 1. Deferred Braided Net and The Enigma
   Jewel: each 0 → 0. Controls before/after: bare Equip 1/1, dashed Equip 1/1,
   Splice 1/1, Affinity 2/2, Protection 2/2; bare Craft retains 1/1 under QC9.
3. **Spot checks — `vwntqpupwqyw`.** Ten gained faces, each one byte-exact
   `QualityCostKeyword` form 1, EquipQuality Head and separately selected
   Quality/Cost: Ace's Baseball Bat and Blackblade Reforged (legendary
   creature); Steelclaw Lance (Knight); Robe of the Archmagi (coordinated
   Shaman, Warlock, or Wizard); Commander's Plate (commander); Dúnedain Blade
   (Human); Bilbo's Ring (Halfling); Lothlórien Blade (Elf); Veteran's
   Powerblade (Soldier); Team Pennant (creature token). No gain relies on
   an unintended route. Crabomination's unmarked Emerge-from line also reads
   once, but its whole face remains unread through separate text, so is no gain.
4. **Deviations and STOPs — `vwntqpupwqyw`.** No Construction or frame added
   or deleted. A marker-concord table preserves marked form 0 while the new
   unmarked form shares its named fields. Initial duplicate Equip noun
   premodifiers are fixed by class-derived lexical distribution for unmarked
   AfterHead QualityCost recipes; no plugin data override exists for that
   licence. The STOP on Craft's semantic signature/generated Idris facts is
   resolved by QC9's split: Craft and its derived macro are restored, both
   With rows omitted, all existing facts rows unchanged. The new planned
   `english-v3-craft-with-quality-cost` preserves the 24 identities, probes,
   exact Lean/Idris diffs and OWNER DECISION. Semantic artifacts add the new EquipQuality macro
   export and facts exemption. Hygiene observations are in hygiene-3.
   Two inherited citation-looking timing decimals are preserved in seconds
   and milliseconds. No glossary gap. Refresh brought grammar and lexicon
   changes and one hygiene-note conflict; both independent notes were
   retained. Kata refresh then exited 0, with no remaining conflicts.
   Integration-turn conflicts preserve both EquipQuality and PartnerWith
   inventory entries, the structural template-total assertion, and both
   hygiene notes; the combined parameter inventory is 112.
5. **Tests — `vwntqpupwqyw`, QC9 rerun.** All full suites and clippy passed:
   `cargo test -p xtask`;
   `cargo clippy -p xtask --tests -- -D warnings`;
   `cargo test -p deckmaste_semantics_v2`;
   `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings`;
   `cargo test -p deckmaste_construction_core`;
   `cargo clippy -p deckmaste_construction_core --tests -- -D warnings`;
   `cargo test -p deckmaste_lexical_source`;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`;
   `cargo test -p deckmaste_english_v3` (1,138 passed, 1 inherited ignored);
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`.
   `cargo xtask lean check` passed. Cite checks: 0 noncompliant, 0 stale;
   `jj --no-pager diff --git | cargo xtask cite audit --diff`: 0 sites.
   Kanban graph check passed. `cargo xtask facts check`
   passed with both generated files unchanged. After refresh, `cargo build -p
   xtask` passed; all ten test/clippy commands above passed again in full.
   Refreshed English: 1,157 passed, 1 inherited ignored; all seven
   `selected_complement_clusters` tests passed with unchanged budget ceilings.
   `cargo xtask lean check` passed again (245 cards); `cargo xtask facts check`
   passed again without changing either generated facts file. Evidence:
   `target/english-v3/english-v3-quality-cost-keyword-lines/post-refresh-checks.json`.
   Integration-turn scoped build, all five full suites, all five clippy
   commands, facts and Lean checks passed again. English: 1,161 passed,
   1 inherited ignored; the all-nine selected-complement chart-budget test
   passed with unchanged ceilings. Evidence: `integration-checks.json` in
   the same target directory. Restored 1:
   `keyword_mention_without_parameter::keyword_premodifiers_keep_keyword_identity_in_costs_and_abilities`
   keeps its original expected value, restored by preventing the new recipe's
   bare noun-premodifier twin. Re-spelled 4:
   `builtin_v2_keyword_ability_nursery_preserves_declared_spelling_and_grammar`
   adds EquipQuality's Quality+Cost signature; Craft's original Cost signature
   and grammar remain pinned; combined inventory is 112 after PartnerWith;
   `every_keyword_declaration_pins_its_template` adds EquipQuality's bare
   two-parameter template while preserving Craft's original template;
   `every_keyword_template_is_consistent_with_its_params_and_grammar` derives
   the complete expected inventory size from both pinned template arrays;
   `keyword_declarations_mint_premodifier_permissions_without_changing_category`
   keeps Keyword category for every keyword and expects NounPremodifier None
   for unmarked AfterHead QualityCost recipes, Keyword otherwise.
   Added 5 English tests; newly ignored 0; removed 0. Unlanded Craft-positive
   cases are withdrawn under QC9 and retained as deferred unread controls;
   no existing test was removed or weakened.
6. **Timing — `vwntqpupwqyw`.** QC9 bounded census: 13.25 seconds wall under
   `/usr/bin/time -v`, 6 workers, 0 samples per face. Refreshed bounded census:
   40.91 seconds wall; xtask rebuild: 6 minutes 37 seconds.
   Integration-turn bounded census: 0:22.25 wall; rebuild: 5 minutes 23 seconds.
   No full-corpus census run.
