---
needs: []
---
# Read *exactly* as a modifier of a numeral: As long as you control exactly one creature, … / Activate only if you have exactly seven cards in hand.

## Why

*exactly* is not a word of the lexicon. Its sister bound reads: "As long as
you control at least one creature, draw a card." 1 and NP "at least two
creatures" 1, "at least 13 life" 1, against "As long as you control exactly
one creature, draw a card." 0 and NP "exactly two creatures" 0. Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`):
`word:exactly` 33 sole, 45 touched. `cargo xtask english-v3 grep -i
'\bexactly\b'` finds **45** faces, all unread; **37** have that line as their
only failing line. By what follows: a numeral, `\bexactly (one|two|…|
thirteen|zero|\d+)\b`, **41** faces (33 sole); a determined NP (*exactly the
chosen number of cards*, *exactly those colors*, *exactly that many colors*)
4 (4 sole). 5 of the numeral faces are the colour predicate *that's exactly
two colors* (3 sole); `english-v3-systemic-residuals` already lists the
vocabulary gap *exactly one*.

## Goal

*exactly* + numeral reads wherever the numeral reads alone: as Determiner
(*exactly one creature*, *exactly 13 life*, *exactly seven cards in hand*),
as a scalar value (*its power is exactly 20*) and as the head of a partitive
(*exactly one of those attacking creatures*), with byte-exact realisation.

## Analysis

CGEL Ch. 5 §11(b), p. 431, "Approximation and precision": "Such adverbs as
*approximately*, *roughly*, *about*, *almost*, *nearly*, *practically*,
*exactly*, *just*, *precisely* … occur with numerals"; [2i] "[*approximately
twenty*] *soldiers*". That backs an Adverb modifying the numeral inside the
determinative phrase. The same page's opening examples contrast "[*At least ten*]
*people*" with "[*At least an hour*]", where the modifier takes the whole NP;
the determined-NP shape (*exactly the chosen number*) is that second
structure and is out of scope.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`): category
`CardinalModifier`, the optional `modifier` of `Cardinal`, `LargeCount` and
the other numeral-headed forms (`[modifier: optional(CardinalModifier), head:
lexical(Numeral)]`). Its only source today is the
`QuantitativePrepositionPhrase` instance row `(CardinalModifier,
QuantitativeAdjectiveComplement, QuantitativeAdjectiveComplementLicence)`,
i.e. the PP *at least* (`vocab:Preposition/At` + `vocab:Adjective/Least`,
`QuantitativeAdjectiveComplementUse: Yes`). No Adverb feeds it.

Orchestrator rulings 2026-10-10:

- EX1. One added source of `CardinalModifier`: a lexical Adverb that declares
  the numeral-modifier use, realised with the same trailing separator the PP
  source has. Every numeral form that takes `optional(CardinalModifier)` then
  reads *exactly* N with no further edit. No new category and no edit to the
  numeral Constructions.
- EX2. `vocab:Adverb/Exactly` is declared in `lexicon/core.ron` with that
  feature and no other licence: `VPFinalAdjunct`, `ClauseInitialAdjunct` and
  central position stay unlicensed (0 attested: every one of the 45 faces has
  *exactly* immediately before a numeral or a determined NP).
- EX3. Arabic numerals: *exactly 13 life*, *exactly 1 damage*, *is exactly
  20* must read where "at least 13 life" 1 and "If its power is at least 20,
  draw a card." 1 read. If `NumericQuantityConjunct` (which has no modifier
  slot) is on that path, give it the same optional slot rather than a second
  route; say which in Deviations.
- EX4. One analysis per string: *exactly one creature* is [[*exactly one*]
  *creature*], never *exactly* modifying the NP or the clause. *at least* N
  keeps exactly its Readings (66 covered lines contain *at least* + numeral).
- EX5. Other precision / approximation adverbs (*precisely*, *approximately*,
  *about*, *almost*, *nearly*) are unattested before a numeral in the
  supported corpus; do not declare them (pruning rule) and do not add a check
  whose only job is to block them.
- EX6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected (the word occurs on none); report any with its shape.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Library of Alexandria: "{T}: Draw a card. Activate only if you have exactly
  seven cards in hand." (control "Activate only if you have at least seven
  cards in hand." 2; without the modifier 2)
- Homicidal Seclusion: "As long as you control exactly one creature, that
  creature gets +3/+1 and has lifelink."
- Near-Death Experience: "At the beginning of your upkeep, if you have
  exactly 1 life, you win the game." (control "If you have 13 life, you win
  the game." 1)
- Alluring Suitor: "When you attack with exactly two creatures, transform
  this creature."
- Amalia Benavides Aguirre (EX3, scalar value; check it is the line's only
  blocker): "… Then destroy all other creatures if its power is exactly 20."
  (control "If its power is 20, draw a card." 1)

Controls that must not change: Cauldron's Gift "Adamant — If at least three
black mana was spent to cast this spell, mill four cards." 4 (census);
Felidar Sovereign "At the beginning of your upkeep, if you have 40 or more
life, you win the game." 1 (census); Twice Upon a Time "Cast this spell only
if you control two or more Doctors." 1 (census); "As long as you control up
to one creature, draw a card." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bexactly\b|\bat (least|most)\b` (147 faces: the 45 plus
the *at least / at most* control class) plus 300 random covered controls,
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; EX6 for increases. Full `cargo
test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end; a
rebuild after a grammar edit takes about 2–3 minutes, the six test binaries
about 75 seconds; lexicon data loads at runtime. Tests assert the role (the
Adverb as `CardinalModifier` of the numeral; the EX4 negative), never an
incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- *exactly* + determined NP (4 faces): *exactly the chosen number of cards*
  (Mindblaze, Scrying Glass), *exactly those colors* (Niv-Mizzet Reborn),
  *exactly that many colors plus one* (Evolving Door).
- Faces whose line has another blocker: *crewed by exactly two creatures*
  with a granted quote (Mighty Servant of Leuk-o), *exactly three or exactly
  four cards* (Skullcage), *exactly zero or seven* (The Biblioplex), *rolled
  6 exactly seven times* (Luck Bobblehead), *with different names* tails
  (Turtles Forever, Burning-Rune Demon).
- The colour predicate if *two colors* fails there for an independent
  reason (NP "a spell that's two colors" 1 today).

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
