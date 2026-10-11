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
with `cargo xtask english-v3 diff` against the claim parent. Zero lost; QE5
for increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
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

To be written by the claimant (six items, `docs/tickets/README.md`).
