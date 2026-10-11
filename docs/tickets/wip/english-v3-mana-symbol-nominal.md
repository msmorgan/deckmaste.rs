---
needs: []
---
# Read mana symbols as a nominal head: Whenever you tap a Swamp for mana, add an additional {B}. / Whenever you cast a spell with {X} in its mana cost, …

## Why

Mana symbols read only as a selected Mana Phrase or as Cost Notation ("Add
{G}." 1, "Its controller adds {G}{G}." 1, "You may pay {X}." 1). Where the
symbols stand as the head of an NP they do not: NP "an additional {G}" 0,
"Add an additional {G}." 0, "Its controller adds an additional {G}{G}." 0,
"Add an additional {G} for each Elf on the battlefield." 0 (without *an
additional* 2); NP "a spell with {X} in its mana cost" 0, NP "costs that
contain {X}" 0, NP "{X}" 0. The spelled-out counterparts read: "Add an
additional three mana." 1, "Add one additional mana of any color." 2, NP "an
additional card" 1, NP "a card with a white mana symbol in its mana cost" 3.
Blocker report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`):
keys `at:additional {g}` 9, `at:with {x}` 13; failing tokens `{g}` 24, `{r}`
24, `{b}` 17, `{u}` 15, `{x}` 14 (mixed). By shape, `cargo xtask english-v3
grep`:

- (a) Determiner + *additional* + symbols, `\badditional \{`: **28** faces,
  all unread, **25** sole. *add(s) an additional {C}* 23 (22 sole); *pays {2}
  plus an additional {1} for each …* 3; *for each additional {1}{G} you paid*
  2 (both with a second failing line).
- (b) bare {X} as the Subject of a *with* small clause (or Object of
  *contain*, out of scope),
  `\b(with|contains?) \{X\}`: **20** faces, all unread, **15** sole; *with
  {X} in its / their mana cost(s)* 16 (13 sole).

## Goal

A mana-symbol sequence heads a Nominal under a Determiner and a Premodifier
(*an additional {G}{G}*), and {X} stands as the NP of *a spell with {X} in
its mana cost*, with byte-exact realisation.

## Analysis

No CGEL claim: symbols standing where a noun stands are Notation (glossary
**Notation**, **Mana Phrase**: "licensed separately from symbols that appear
only in Costs"). Everything here is a project ruling. Internal evidence for
the structure: *an additional {G}* alternates with *an additional one mana
of any color* (Verdant Haven, covered) and *an additional three mana*, i.e.
Determiner + *additional* + measured head; *with {X} in its mana cost*
alternates with *with a white mana symbol in its mana cost*.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`): the
symbol categories of the `SymbolSequence` schema (`NamedManaSymbol`,
`NamedCostSymbol`, `NumericCostSymbol`; `CostSymbols`);
`SymbolComplementNominal` (a noun + *of* + `CostSymbols`); `PremodifiedNominal`
(`[modifier: AdjectivePhrase, " ", head: Nominal]`); the verb-side
`SharedManaComplement` schema and *add*'s Mana Phrase frame. The sibling
ticket `english-v3-quantified-energy-symbol` (QE1) adds one Nominal form
headed by a lexical Symbol for {E}.

Orchestrator rulings 2026-10-10:

- MS1. One symbol-headed Nominal form serves both tickets. Whichever lands
  first adds it; the second widens it by declaration. Here its head is a
  mana-symbol sequence and its features are those of a singular count head,
  so *an additional* selects it as it selects *card*. No second Construction.
- MS2. The licence is declared on the symbol class (the mana and numeric
  cost symbols declare the nominal use, as {E} does), never read from the
  spelling of a symbol.
- MS3. One analysis per string. *Add {G}{G}.*, *pay {X}*, *costs {2} less*
  and every Cost keep exactly their Readings: the Nominal exports a
  Determiner requirement, so a bare symbol sequence never also reads as an
  NP Object. *Add an additional {G}* has one structure: *add* + NP
  [*an additional {G}*], not *add* + NP [*an additional*] + Mana Phrase.
- MS4. Shape (b) is not a determinerless NP (that would break MS3). It is
  admitted where the *with* + NP + locative PP small clause takes its NP,
  by a form that accepts a Mana Phrase in that
  position, licensed by a declared feature of the governing frame. If that
  needs more than one added form, land (a) and return (b) with the probe
  evidence; do not STOP the ticket.
- MS5. *an additional {G}{G}* (Overgrowth) is singular *an* over a two-symbol
  sequence: the sequence is one head. *each additional {1}{G} you paid* is
  the same head under *each* with a relative clause; it gains only if the
  relative-clause route takes the new Nominal (report, do not chase).
- MS6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected. Mana symbols occur on thousands of covered faces: any
  increase there is an MS3 defect and is reported with its shape before
  landing.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Crypt Ghast (a): "Whenever you tap a Swamp for mana, add an additional
  {B}."
- Wild Growth (a): "Whenever enchanted land is tapped for mana, its
  controller adds an additional {G}."
- Overgrowth (a): "Whenever enchanted land is tapped for mana, its controller
  adds an additional {G}{G}."
- Elvish Guidance (a): "Whenever enchanted land is tapped for mana, its
  controller adds an additional {G} for each Elf on the battlefield."
- Matterbending Mage (b): "Whenever you cast a spell with {X} in its mana
  cost, this creature can't be blocked this turn." (control "Whenever you
  cast a spell with a counter on it, draw a card." 10)
- Gaddock Teeg (b): "Noncreature spells with {X} in their mana costs can't be
  cast." (control "Noncreature spells with counters on them can't be cast."
  5)

Controls that must not change: Verdant Haven "Whenever enchanted land is
tapped for mana, its controller adds an additional one mana of any color." 9
(census); Valleymaker "{T}, Sacrifice a Forest: Choose a player. That player
adds {G}{G}{G}." 1 (census); Cabal Stronghold "{3}, {T}: Add {B} for each
basic Swamp you control." 2 (census); Light from Within "Chroma — Each
creature you control gets +1/+1 for each white mana symbol in its mana
cost." 6 (census); Exploration "You may play an additional land on each of
your turns." 4 (census); "You may pay {X}." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \badditional\b|\b(with|contains?) \{X\}|\b[Aa]dds? |\bsymbols?\b

plus 300 random covered controls, `--workers 6 --samples-per-face 0`,
compared with `cargo xtask english-v3 diff` against the claim parent (size
the selector with `cargo xtask english-v3 grep --count` and record it: the
*add* class is the MS3 control population and is deliberately whole). Zero
lost; MS6 for increases. Full `cargo test -p` and `cargo clippy -p … --tests
-- -D warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`,
once, at the end; a rebuild after a grammar edit takes about 2–3 minutes, the
six test binaries about 75 seconds; lexicon data loads at runtime. Tests
assert the role (the symbol sequence as head of the Nominal under *an
additional*; the Mana Phrase as Subject of the *with* small clause; the MS3
negatives on bare symbols), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- {E} under a quantifier: `english-v3-quantified-energy-symbol`.
- *pays {2} plus an additional {1} for each* … (3 faces; *plus* arithmetic
  over costs, key `at:many plus` family).
- *Whenever you tap a permanent for {C}* / *a land for {C}* (Forsaken
  Monument, Ultima): the *for* + symbol Complement of *tap* is a separate
  frame question.
- *activate an ability with {X} in its activation cost* (Magus Lucea Kane) if
  *activation cost* does not read; *Spend this mana only on costs that
  contain {X}* (Rosheen, Elementalist's Palette: *contain(s)* is not a word
  of the lexicon).

## Landing record

1. **Census delta** (`qtyxwuyo`, pristine claim parent `sprutzvq` → implementation): bounded 1,832-face manifest; 1,784 → 1,805 covered faces, 25,738 → 26,095 Readings. Built-in `english-v3 diff --lines`: 21 gains; lost faces none; decreases, increases and digest-only changes all zero; unintended gains none. Selector: all 48 faces matching `an additional \{|each additional \{|\b(with|contains?) \{X\}`, plus 1,500 random covered mana-symbol faces and 300 random covered controls, seed 20261008, deduplicated. Evidence: `target/english-v3/english-v3-mana-symbol-nominal/{manifest,before,after,diff}.json`.
2. **Witnesses** (`qtyxwuyo`): every witness was probed before implementation, all zero. After: Crypt Ghast 2, Wild Growth 3, Overgrowth 3, Elvish Guidance 27; corresponding ordinary-noun controls 2, 3, 3, 27 preserve W9-A Ambiguity. Matterbending Mage 0 and Gaddock Teeg 0 returned under MS4 to `english-v3-mana-symbol-verbless-subject`; their original controls remain 10 and 5. Other controls before/after unchanged: Verdant Haven 9, Valleymaker 1, Cabal Stronghold 2, Light from Within 6, Exploration 4, payment 1. MS5's `each additional {1}{G} you paid` Noun Phrase has 2 Readings; its whole faces still have other gaps.
3. **Spot checks** (`qtyxwuyo`): Crypt Ghast `{B}`, Wild Growth `{G}`, Overgrowth `{G}{G}`, Elvish Guidance `{G}`, High Tide `{U}`, Vernal Bloom `{G}`, Black Mana Battery `{B}`, Nirkana Revenant `{B}`, Bubbling Muck `{B}`, Gauntlet of Might `{R}`: each selected *add* Object is the determined Noun Phrase, with *additional* modifying its symbol-sequence Nominal head. Overgrowth's sequence is one singular head; existing PP attachment alternatives survive. Ten probes completed without roundtrip issues.
4. **Deviations and STOPs** (`qtyxwuyo`): MS4 authorizes returning shape (b), requiring a governed Subject projection plus its preposition form to preserve bare Mana Phrase negatives. The first attempt wrongly admitted `{X}` and numerals globally; the existing mana value test found it, and the Mana Phrase production and licences were restored. One existing `MeasuredSymbolNominal` form serves count and mass uses; added `NominalSymbol` and count-only continuation `CountNominalSymbol` projections preserve the energy singleton and numeric symbol identity. Retired the former energy measurement-feature route in both grammar and lexicon. No verb frames added. Glossary gaps filled: Symbol, Nominal Symbol, Count Nominal Symbol, Symbol Nominal Use, citing project rulings. Four new tests include the explicitly blocked (b) test; seven energy tests were re-spelled for the shared head representation. No unresolved STOP.
5. **Tests** (`qtyxwuyo`): `cargo test -p deckmaste_english_v3` passed (1,028 active tests, 1 named blocker); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed; `cargo test -p deckmaste_lexical_source` passed (88 tests); `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed. Citation checks: no noncompliant strings, zero stale citations, diff audit selected no citation sites. Counts: restored 0; re-spelled 7; ignored-with-blocker 1 (`mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`, blocker `english-v3-mana-symbol-verbless-subject`); added 4 (3 active, 1 blocked); removed 0. Earlier runs found a fixture compile error, an overly narrow attachment assertion, and the genuine bare-Mana-Phrase regression described above; all were corrected, with failure evidence retained under `target/`.
6. **Timing** (`qtyxwuyo`): 3m47s760ms measured with `/usr/bin/time -v`, including Cargo lock wait, 6 workers, 0 samples per face; no full census, per the binding verification budget.
