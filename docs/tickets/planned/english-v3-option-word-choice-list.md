---
needs: []
---
# Read a list of bare option words after a choice verb: Choose odd or even.

## Why

A choice among named options is written as a bare coordination of the option
words: *Choose odd or even.*, *Choose artifact, creature, or land.*, *choose
friend or foe*, *Choose left or right.*, *choose first strike, vigilance, or
lifelink*, *votes for death or taxes*. It reads only when every member happens
to be a bare-capable noun ("Choose heads or tails." 2; "Each player votes for
time or money." 4). Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.json`): hidden under `at:even .`,
`at:foe .`, `at:land .`, `at:sorcery .`, `at:lifelink .`, `at:random .` and
about thirty one-face `word:` keys. `cargo xtask english-v3 grep -i
'\b(chooses?|votes? for) [a-z-]+( [a-z]+)?(, [a-z -]+)*,? or [a-z-]+( [a-z]+)?[.:]'`
finds **116** faces, **76** unread; 72 failing lines match, **57** the only
failing line of their face (the 40 covered are mostly *choose target X or Y*
and *choose among one or two targets*, not this). A hand-cut of the sole lines
gives **67**: **39** whose option words are all declared Lexemes (*odd or
even* 8, card types 10, *friend or foe* 5, *left or right* 6, keyword lists
5, numerals 4, colours 2, *land or nonland* 3) and **28** with an undeclared
label word (*grace or condemnation*, *war or peace*; 22 are *votes for*).
`english-v3-hygiene-3` listed "each player votes for X or Y 27" as unrouted.

## Goal

A coordination of bare option words reads as the Complement of a choice verb
(*choose*, *vote for*), with byte-exact realisation, for every option word
that is a declared Lexeme. The 28 undeclared-label faces are counted and
routed, not read here (OW5).

## Analysis

No CGEL passage is relied on: the options are mentioned, not used (*odd* is
not predicated of anything; *creature* has no Determiner although it is a
singular count noun). The analysis is a project ruling.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`): *choose*
(`core-verb:Choose`) takes an NP Object through `SelectedPredicate`; the
coordination schemas build `Coordination` over NPs; `BareNounPhrase` admits
plural and mass nouns, which is the only reason *heads or tails* and *time or
money* read. The covered Siege faces read by accident: "As this enchantment
enters, choose Khans or Dragons." reads 1 as `ProperNameNounPhrase`
(`lexeme:Name/Khan`) *or* `BareNounPhrase` (*Dragons*), and "Choose Island or
Swamp." reads through `catalog:card-names.txt/Island`. Nearest declared
neighbour for an open label is the lexical Category `FlavorWord`
(`FlavorWordHead`).

Orchestrator rulings 2026-10-11:

- OW1. One Option List route: a coordination (pair or comma series) whose
  coordinates are bare option words, selected as Complement by a frame the
  choice verb declares. Refine the existing coordination schemas with one
  member projection; no parallel coordination family. If a new Construction is
  unavoidable it is ONE member projection (R-budget): say so before building,
  with the faces it buys.
- OW2 (R-class). The licence is class-level: the verb's declared option frame
  (every existing verb of the choice class gets it: *choose*, *vote*; not a
  row per attested option list), and the member is any declared Lexeme of the
  mentionable lexical Categories (Noun, Adjective, Keyword, Numeral, colour
  word) in its citation form. No list of option words, no test on *odd*,
  *friend*, *left*, and no card test. Mint no Lexeme for an unattested word.
- OW3 (R-one, R-wrong). The twin to avoid is the bare-NP coordination that
  reads today by accident. Before designing, record the trees of "Choose heads
  or tails." (2), "Each player votes for time or money." (4), "choose Khans or
  Dragons" (Monastery Siege) and "Choose Island or Swamp." (Roots of Life). If
  the Option List would read beside them, STOP with both trees: which analysis
  those faces keep is a ruling (R-prune permits retiring the accidental one;
  the card-name and `Name/Khan` Readings are wrong analyses, not Readings to
  protect).
- OW4. *at random* after the list (*Choose 1, 2, or 3 at random.*, 3 faces)
  is the existing Adjunct; in scope only if it falls out. A trailing colon
  (Phyrexian Splicer's cost) is the existing cost position.
- OW5. Undeclared label words (28 sole faces: council votes, *war or peace*,
  *Believe or Doubt*) are card-local labels. They are NOT read by declaring
  each word as a Noun. Out of scope here; report the count and the candidate
  mechanism (an open label slot like `FlavorWord`, or per-face declarations)
  for an owner decision. Later mentions (*for each death vote*, *each player
  who chose war*) go with that decision.
- OW6. Newly covered faces are gains. Reading-count changes are expected only
  on covered faces matching the selector, and only as OW3's shape; under OW3
  that is a STOP, not a result to report afterwards. Any other change is a
  defect.

## Witnesses

Each is a sentence of the face's only failing line. Probed 2026-10-11 with the
trunk binary (`--category sentence --readings 8`): all 0.

- Extinction Event: "Choose odd or even."
- Turnabout: "Choose artifact, creature, or land."
- Pir's Whim: "For each player, choose friend or foe."
- Order of Succession: "Choose left or right."
- Angelic Skirmisher: "At the beginning of each combat, choose first strike,
  vigilance, or lifelink."

Also 0: "Each player votes for death or taxes." (Capital Punishment; all words
declared); "Choose 1, 2, or 3 at random." Controls that must not change
except under OW3: "Choose heads or tails." 2; "Choose one or both." 1; "Choose
a color." 1; "Choose a creature type." 1; "Each player votes for a creature."
2; "Each player votes." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(choose|chooses|chose|chosen|vote|votes|voted)\b` plus
300 random covered controls (`--random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against the
claim parent. Zero lost; OW6 for changes. Full `cargo test -p
deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests -- -D
warnings` (and `-p deckmaste_lexical_source` if the lexicon changes) once, at
the end: a rebuild after a grammar edit takes 2–3 minutes and the six test
binaries about 75 seconds; lexicon data loads at runtime. Tests assert the
role (each option a coordinate of the list, the list the verb's Complement,
the OW3 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- OW5's undeclared labels (28 sole faces). *votes for planeswalk or chaos*
  (5 faces, Planechase wording): listed, never ticketed.
- *Choose a creature type other than Wall.* / *a card type other than
  creature* (6 sole faces): a bare type word after *other than*; related
  mention use, separate shape.
- *Choose a number between 0 and 10.* (4 faces); *your choice of 5/1 or 1/5*.
- Quoted options (Raging River, *choose "left" or "right."*).

## Landing record
