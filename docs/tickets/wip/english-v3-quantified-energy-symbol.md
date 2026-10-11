---
needs: []
---
# Read the energy symbol as a measured nominal head: Pay eight {E}: … / Whenever you get one or more {E}, …

## Why

{E} reads only as cost-symbol Notation: "You get {E}{E}." 1, "Pay {E}{E}:
Draw a card." 1, "You may pay any amount of {E}." 1 (through
`SymbolComplementNominal`). Under a quantifier it reads nowhere: "You get six
{E}." 0, "You get X {E}." 0, "You may pay eight {E}." 0, "Pay eight {E}: Draw
a card." 0, "You may pay one or more {E}." 0, "You get that many {E}." 0, NP
"each {E} paid this way" 0. The spelled-out counterpart reads: "You get six
energy counters." 1, "You may pay one or more energy counters." 1. Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): failing
token `{e}` 30, key `at:more {e}` 9.
`cargo xtask english-v3 grep '\b(one|two|three|four|five|six|seven|eight|nine|ten|fifty|X|more|many|each) \{E\}'`
finds **36** faces, all unread; **29** have that line as their only failing
line. By quantifier: word cardinal (*six*, *eight*, *fifty*) 16 faces (13
sole); *one or more* / *N or more* 12 (9); *that many* 6 (1); *X* 5 (4);
*each* 3 (2). 99 of the 142 {E} faces are covered and must not change.

## Goal

The energy symbol heads a measured Nominal under a Quantitative Determiner,
in Object position, in a Cost component and after *for each*: *pay eight
{E}*, *you get X {E}*, *one or more {E}*, *that many {E}*, *each {E} paid
this way*, with byte-exact realisation.

## Analysis

No CGEL claim: a symbol standing where a noun stands is Notation (glossary
**Notation**, **Cost Component Form**, **Mana Phrase**); everything here is a
project ruling. The agreement evidence is internal: *If seven or more {E} was
paid this way* (Wheel of Potential) patterns with "If seven or more life was
paid this way, draw a card." 2, a singular measured mass, not with a plural
count noun.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`SymbolComplementNominal` (`[head: lexical(Noun), " ", marker:
lexical(Preposition), " ", complement: CostSymbols]`, `head.frame =
NominalSymbols`, *amount of {E}*); the measured-mass routes
`CardinalMeasuredNounPhrase` (`[quantity: Cardinal, " ", head: Nominal]`,
`head.MeasurePosition = Before`, `head.CardinalMeasurementUse = Yes`,
`head.countability = Mass`, *three mana*), `CardinalMeasuredNominal` and
`QuantifiedNounPhrase` (*one or more life*). The symbol is
`vocab:FixedCostSymbol/E` (category Symbol, `lexicon/core.ron`);
`lexeme:CommonNoun/Mana` declares `MeasurePosition: Before`,
`CardinalMeasurementUse: Yes`.

Orchestrator rulings 2026-10-10:

- QE1. One added Nominal form whose head is a lexical Symbol, in the
  bare-nominal family beside `BareFramedNoun` / `SymbolComplementNominal`,
  exporting the measured-mass features the quantifying Constructions already
  read. Those Constructions are not edited: once the Nominal exists,
  `CardinalMeasuredNounPhrase` reads *eight {E}* and `QuantifiedNounPhrase`
  reads *one or more {E}* / *that many {E}*.
- QE2. The licence is declared on the symbol lexeme (the features *mana*
  declares: `MeasurePosition`, `CardinalMeasurementUse`, countability), never
  a test on the symbol's spelling. Only {E} declares them: mana symbols are
  `english-v3-mana-symbol-nominal`'s decision.
- QE3. One analysis per string. Bare {E} / {E}{E} keeps exactly its Readings
  as cost-symbol Notation in *you get {E}{E}*, *Pay {E}:* and *amount of
  {E}*: the new Nominal must not give those a second Reading as a
  determinerless NP. If the form cannot exclude that by a declared feature
  the existing Constructions already check (e.g. `DeterminerRequirement`),
  STOP with the diff.
- QE4. *each {E}* needs a count-singular use; declare it only if the same
  declaration reads *for each {E} paid this way* without adding a Reading to
  QE3's strings; otherwise leave the 3 faces and record them.
- QE5. Newly covered faces are gains. No Reading-count increase on the 99
  covered {E} faces is expected; report any with its shape. Under QE3 it is
  a defect.
- QE6 (orchestrator, 2026-10-10). No bare-NP licence. Export the existing
  `DeterminerRequirement = Yes` on the symbol-headed Nominal; quantifying
  Noun Phrase Constructions may satisfy it, while bare routes must reject it.
  This authorizes refining QE1's existing quantifying requirements. Target
  zero increases on all 99 covered {E} faces; explain digest-only changes.
  Leave the two quantity witnesses as named residuals unless the same
  refinement fixes them. Any remaining covered-face increase requires
  restoring the implementation and returning the ticket to planned.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Aethertide Whale: "When this creature enters, you get six {E}."
- Aethersquall Ancient: "Pay eight {E}: Return all other creatures to their
  owners' hands. Activate only as a sorcery."
- Roil Cartographer: "{T}, Pay six {E}: Draw three cards."
- Fabrication Module: "Whenever you get one or more {E}, put a +1/+1 counter
  on target creature you control."
- Empyreal Voyager: "Whenever this creature deals combat damage to a player,
  you get that many {E}."
- Overclocked Electromancer: "Whenever this creature deals combat damage to
  a creature, if that creature was dealt excess damage this turn, you get X
  {E}, where X is that excess damage."

Controls that must not change: Nissa, Worldsoul Speaker "Landfall — Whenever
a land you control enters, you get {E}{E}." 2 (census); Solstice Zealot
"{T}, Pay {E}: Tap target creature." 1 (census); Inventor's Axe "Equip—Pay
{E}{E}." 1 (census); Peema Aether-Seer "When this creature enters, you get an
amount of {E} equal to the greatest power among creatures you control." 9
(census); "You may pay any amount of {E}." 1; "You get six energy counters."
1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\{E\}` (142 faces: the 36, 7 other unread, 99 covered) plus
300 random covered controls, `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. QE6 adds all
faces matching `\b(defend|defends|defended|defending)\b` case-insensitively
for the bare participial requirement refinement. Zero lost and zero increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`, once, at
the end; a rebuild after a grammar edit takes about 2–3 minutes, the six test
binaries about 75 seconds; lexicon data loads at runtime. Tests assert the
role (the symbol as head of the measured Nominal under its quantifier; the
QE3 negatives on bare {E}{E}), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Mana symbols as nominal heads (*an additional {G}*, *with {X} in its mana
  cost*): `english-v3-mana-symbol-nominal`.
- *that many plus one {E}* (Izzet Generatorium; `at:many plus` is its own
  blocker), *Pay {2} or {E}{E}{E}* (Razorfield Ripper), *you've paid or lost
  four or more {E}* (shared-object coordination).
- The 7 unread {E} faces not matching the pattern.

## Landing record

1. **Census delta** — At `tznkrzpw`, original claim parent `zmpwzzsl` (code parent
   `oupzlsuq`), bounded BEFORE → QE6 AFTER: covered 609 → 625; Readings
   9,066 → 9,199. Lost none; gained 16; decreases 0; increases 0;
   digest-only 99; unchanged 563; source changes 0; issues 0. Selector:
   every supported face matching `\{E\}` (142), plus 300 random covered
   controls selected by the official diff's `--manifest --random 300
   --seed 20261008`; plus all 245 faces containing any form of *defend*
   for the refined bare participial route, giving 678 distinct faces after
   overlaps. All 99 existing covered {E} faces preserve their Reading counts.
   Their digests change
   because the invariant Symbol Word now records declared mass countability
   (`Some(false)`) instead of undeclared countability; their Notation routes
   are preserved. Official `diff --lines` evidence: this workspace's
   `target/english-v3/english-v3-quantified-energy-symbol/qe6-expanded-diff.json`
   and `qe6-expanded-diff.txt`, alongside `qe6-expanded-before.json`,
   `qe6-expanded-after.json` and `qe6-manifest.json`.
   After refresh, at `mnrokvtn`, the same 678 faces have exactly the same
   coverage, counts and digests: `refresh-only-diff.json` reports all 678
   unchanged, zero losses/decreases/increases. Incoming trunk changes
   `pusrqwml` (partitive quantities), `ulrnwvuw` (waterbend), and `mlnortmq`
   (declaration batch) introduce no census delta on this selector. The final
   first refreshed claim-parent comparison is `refresh-diff.json` / `refresh-diff.txt`.
   Second refresh at `vtylwlmy` brings `qnvlmzuu` (partitive determinative
   heads): `refresh2-only-diff.json` again reports all 678 unchanged, with
   zero losses/decreases/increases. Final comparison: `refresh2-diff.json`
   / `refresh2-diff.txt`; counts and the digest-only class above are unchanged.
2. **Witnesses** — At `tznkrzpw`, claim-parent probes → QE6 probes:
   Aethertide Whale 0 → 1; Aethersquall Ancient 0 → 1; Roil Cartographer
   0 → 1; Fabrication Module 0 → 0; Empyreal Voyager 0 → 0;
   Overclocked Electromancer 0 → 6. Overclocked's counterpart replacing
   *X {E}* with *X energy counters* also has 6 Readings: independent
   ambiguity is preserved under W9-A. Controls: Nissa's Landfall line
   2 → 2; Solstice Zealot 1 → 1; Inventor's Axe 1 → 1; Peema Aether-Seer
   9 → 9; *any amount of {E}* 1 → 1; *six energy counters* 1 → 1.
   Additional controls: *Pay {E}: Draw a card.* 1 → 1; *You get {E}{E}.*
   1 → 1; *each {E} paid this way* 0 → 0. All probes enumerated completely
   with zero validation issues. Fabrication's *one or more* and Empyreal's
   *that many* select plural count heads on the existing quantitative route;
   the declared energy head is singular mass. Fixing that selection or adding
   a scalar measurement route is a separate refinement: both are residuals
   under QE6. The 3 *each* faces remain residuals under QE4.
3. **Spot checks** — At `tznkrzpw`, ten gained faces were fully probed and
   every Reading inspected. Each uses `CardinalMeasuredNounPhrase` with
   `MeasuredSymbolNominal` as head: Aethertide Whale (*six*, Get Object);
   Aethersquall Ancient (*eight*, Pay Cost); Roil Cartographer (*six*, Pay
   Cost); Overclocked Electromancer (*X*, Get Object); Gonti's Aether Heart
   (*eight*, Pay Cost); Era of Innovation (*six*, Pay Cost); Aethertorch
   Renegade (*eight*, Pay Cost); Chthonian Nightmare (*X*, Pay Cost);
   HELIOS One (*X*, Pay Cost); Stone Idol Generator (*six*, Pay Cost).
   No unintended gained analysis found. Evidence: `qe6-spot-0.json` through
   `qe6-spot-9.json` beside the census.
4. **Deviations and STOPs** — QE3's trial STOP is resolved by QE6: the trial's
   47 increases were bare-NP twins; the final symbol Nominal exports the
   measured-nominal landing `xtuursvxqwso`'s existing Determiner Requirement.
   One added `MeasuredSymbolNominal`; {E} declares mass countability,
   Measure Position and Cardinal Measurement Use. QE6 authorizes refining
   `CardinalMeasuredNounPhrase` and `QuantifiedNounPhrase` to satisfy the
   requirement for an unmeasured mass head with Measure Position Before.
   Already measured Nominals remain excluded.
   `BareParticipialStatusNounPhrase` lacked the check and now requires
   `head.DeterminerRequirement = No`, as QE6 directs; other bare routes
   already check the feature. Its only declared verb licence is Defend, so
   the selector also includes all 245 faces matching any form of *defend*.
   No bare-NP licence or twin route is added.
   No frame or route is retired. Count-singular use remains undeclared
   under QE4. The existing independently constructed Attune with Aether
   energy-object test is re-spelled with the declared mass Word metadata,
   preserving its exact value, roundtrip and traversal assertions. Seven
   tests are added in one module registered once in `suite_2.rs`. No glossary
   gap found. The Symbol Word remains invariant; singular Number is exported
   on the Nominal.
5. **Tests** — At `tznkrzpw`, `cargo test -p deckmaste_english_v3`
   passed all 994 tests; `cargo test -p deckmaste_lexical_source` passed all
   86 tests. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`
   passed. After refresh, at `mnrokvtn`, the same four commands passed in
   full: 1,013 English tests and 87 lexical-source tests. `cargo build -p xtask`
   passed before the refreshed bounded census. After the second refresh,
   at `vtylwlmy`, xtask build and the same four full test/clippy commands
   passed again: 1,022 English tests and 87 lexical-source tests. The first full English run found the independently constructed
   energy-object fixture's old countability metadata; it was re-spelled,
   then the full suite passed. English tests and clippy passed again after
   adding the bare participial check. Restored 0 / re-spelled 1 / ignored 0 /
   added 7 / removed 0. Citation checks: zero stale, zero noncompliant, diff
   audit selected zero new citation sites. No Lean checks.
6. **Timing** — At `tznkrzpw`, bounded expanded QE6 AFTER census:
   **12.38 s**, 6 workers, samples per face 0, under `/usr/bin/time -v`. Expanded BEFORE: 20.36 s.
   Refreshed bounded AFTER at `mnrokvtn`: 25.58 s, 6 workers;
   after the second refresh at `vtylwlmy`: 36.49 s, 6 workers. The user's
   bounded-run instruction supersedes the full-corpus timing requirement;
   no full-corpus census was run.
