---
needs: []
---
# Read a quantified mana or energy symbol beyond cardinal + {E}: Add six {G}; add that much {G}; pay one or more {E}.

## Why

`english-v3-quantified-energy-symbol` (landed) reads a cardinal before the
energy symbol ("You get eight {E}." 1; "You get X {E}." 1). The same position
fails for every mana symbol and for every other quantifier before {E}: "Add
six {G}." 0, "Add that much {G}." 0, "Add X {G}." 0, "You get that many {E}."
0, "You may pay one or more {E}." 0 (NounPhrase "six {G}", "that much {G}",
"that many {E}", "one or more {E}" all 0, while "that much mana" 1 and "one or
more life" 1). Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.json`): `at:{r} .` (7 of 12), `at:{g} .`
(5 of 10), `at:{e} .` (7 of 7), `at:{c} .`. `cargo xtask english-v3 grep -i
'\b(that much|two|three|four|five|six|seven|eight|nine|ten|X) \{[WUBRGC]\}'`
finds **18** faces, all unread, **18** sole (*that much* 9, spelled cardinal
8, *X* 1). `grep -i '\b(that many|one or more) \{E\}'` finds **15** faces, all
unread; 17 failing lines, **11** sole (*one or more* 8, *that many* 3).
Together **29** sole-blocker faces.

## Goal

A symbol the lexicon declares as a nominal symbol reads under the quantifying
Determiners the grammar already has (cardinal, *X*, degree *that much* / *that
many*, bounded *one or more*), as Object of the verbs that already take the
quantified noun, byte-exact.

## Analysis

No CGEL passage is relied on: symbol notation is Oracle layout. Project
rulings throughout; MS2–MS4 of `english-v3-mana-symbol-nominal` stand (no bare
symbol becomes a Noun Phrase; a symbol class is licensed through a declared
feature).

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`): feature
`SymbolNominalUse { No, Count, Mass }` declared per symbol in
`lexicon/core.ron`; `MeasuredSymbolNominal`; tables
`symbol_nominal_countability`, `symbol_measure_position` (`Mass => Before`)
and `symbol_cardinal_measurement` (`Count => No`, `Mass => Yes`); the
quantifying Constructions `CardinalMeasuredNounPhrase` (*three mana*, *eight
{E}*) and `QuantifiedNounPhrase` (*one or more life*, *that much mana*),
which the energy ticket deliberately did not edit. Mana symbols carry
`SymbolNominalUse: Count`, so the cardinal-measurement table refuses them;
{E} is `Mass`, so count-plural *that many* and *one or more* refuse it.

Orchestrator rulings 2026-10-11:

- QS1. First record, for each of the five zero probes above, which
  requirement refuses it (the table row or the Determiner's agreement). The
  fix is a refinement of those declared tables and of the symbol Nominal's
  exports; `CardinalMeasuredNounPhrase` and `QuantifiedNounPhrase` are not
  given symbol-specific forms. No new Construction (R-budget).
- QS2 (R-class). The licence reads `SymbolNominalUse` and the features the
  quantifying Constructions already check. Every symbol of a declared class
  gets the quantified use ({W} {U} {B} {R} {G} {C} alike; hybrid and {S} as
  their class declares), not the attested colours; every Determiner of the
  class gets it (all cardinals, not *six*). No test on a symbol's spelling,
  on *much* or *many*.
- QS3. Oracle writes *that much {G}* (mass) and *six {G}* (cardinal) for
  mana, and both *eight {E}* (mass measurement) and *that many {E}* / *one or
  more {E}* (count agreement) for energy. If one symbol needs two
  `SymbolNominalUse` values, STOP with the attested pairs before adding a
  value: whether the feature gains a member or the quantified symbol exports
  transparent number is a ruling.
- QS4 (R-one). One analysis per string. The twins to avoid: (a) *six {G}* as
  a bare numeral followed by a Mana Phrase Complement; (b) *that much* as a
  fused Object with the symbol a second Complement; (c) "{G}{G}{G}" gaining a
  quantified Reading beside its Mana Phrase one.
- QS5. MS protected negatives stay: bare "{X}" and "{2}" as written in
  `mana::independent_mana_values_preserve_symbol_sequence`; generic and
  numeric symbols are not quantified here (*six {2}* is nothing).
- QS6. Newly covered faces are gains. R-reach: the tables are read by every
  face with a quantified or nominal symbol, so the selector covers every
  `{E}` face and every determiner + symbol face; digest-only changes are
  reported by class. No Reading-count increase is expected on a covered face;
  report any with its shape.

## Witnesses

Each is the face's only failing line, or a sentence of it. Probed 2026-10-11
with the trunk binary (`--readings 8`): all 0.

- Castle Garenbrig (Document): "{2}{G}{G}, {T}: Add six {G}."
- Irencrag Feat: "Add seven {R}."
- Sakiko, Mother of Summer: "Whenever a creature you control deals combat
  damage to a player, add that much {G}."
- Sphinx of the Revelation: "Whenever you gain life, you get that many {E}."
- Pia Nalaar, Chief Mechanic: "At the beginning of your end step, you may
  pay one or more {E}."

Controls that must not change: "You get eight {E}." 1; "You get X {E}." 1;
"Add {G}{G}{G}." 1; "Add that much mana." 1; "Add six mana." 1; "Whenever you
gain life, you get that many cards." 1; "Add an amount of {G} equal to its
power." 1; NounPhrase "one or more life" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\{E\}|\b(that much|that many|one or more|any number
of|two|three|four|five|six|seven|eight|nine|ten|X) \{` plus 300 random covered
controls (`--random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent. Zero
lost; QS6 for changes. Full `cargo test -p deckmaste_english_v3`, `-p
deckmaste_lexical_source` and clippy `--tests -- -D warnings` for both, once,
at the end (a grammar rebuild takes 2–3 minutes, the six test binaries about
75 seconds; lexicon data loads at runtime). Tests assert the role (the symbol
is the quantified Nominal's head, the Determiner its quantifier; QS4 and QS5
negatives), never an incidental count or a Debug-printed tree. Attestation by
`cargo xtask english-v3 grep` only.

## Out of scope

- *add twice that much {G}* (Fangorn, 1 face): multiplier over the degree
  quantity; falls out or stays.
- *Add X mana in any combination of {R} and/or {G}.* (12 sole faces): a
  symbol coordination as Complement of *of*; counted in the wave-10 triage.
- *for each {E} paid this way* / *each {E} you've paid or lost* (7 sole): the
  symbol under *each*.
- `english-v3-mana-symbol-verbless-subject`,
  `english-v3-mana-symbol-passive-subject`.

## Landing record
