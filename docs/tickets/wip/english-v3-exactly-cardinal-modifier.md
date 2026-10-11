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

1. **Census delta** — `oyqrxmwz`: BEFORE is the empty feature tree at claim
   parent `stymvrko`; AFTER contains this change. Bounded manifest: all 147
   supported faces matching `(?i)\bexactly\b|\bat (least|most)\b`, plus 300
   random covered controls (seed 20261008), 446 distinct faces. Covered
   375 → 403; total Readings 7,030 → 7,627; 28 gains; lost faces none;
   decreases / increases / digest-only / source changes all zero; zero issues.
   `cargo xtask english-v3 diff` owns the comparison; evidence is under
   `target/english-v3/english-v3-exactly-cardinal-modifier/`.
   Refresh recheck at `ummqklns`: all 446 faces retain identical counts and
   digests; covered 403 and Readings 7,627. No trunk Reading delta to attribute.
2. **Witnesses** — `oyqrxmwz`, exact ticket fragments, fully enumerated by
   `cargo xtask english-v3 probe`: all five BEFORE counts were zero, so none
   was dropped. AFTER / unmodified control: Library of Alexandria 2 / 2;
   Homicidal Seclusion 1 / 2; Near-Death Experience 1 / 1; Alluring Suitor
   1 / 1; Amalia Benavides Aguirre 1 / 1. Seclusion's control has lexical
   Determinative *one* and numeral *one* analyses; EX4's new route modifies
   only the numeral. The control retains both. Fixed controls before/after:
   Cauldron's Gift 4 / 4, Felidar Sovereign 1 / 1, Twice Upon a Time 1 / 1,
   *As long as you control up to one creature, draw a card.* 1 / 1.
   EX3's *exactly 13 life* NP also reads once.
3. **Spot checks** — `oyqrxmwz`, ten gained faces, each retaining the lexical
   Adverb as the numeral's Cardinal Modifier: Alluring Suitor (*two*,
   Determiner); Amalia Benavides Aguirre (*20*, Scalar Value); Ayara's
   Oathsworn (*four*, counter Determiner); Deadly Wanderings (*one*,
   creature Determiner); Demonic Rising (*one*, creature Determiner);
   Ghyrson Starn, Kelermorph (*1*, measured damage); Guildpact Paragon
   (*two*, colors Determiner inside the relative predication); Hidetsugu's
   Second Rite (*10*, measured life); Homarid (*one* / *three*, tide-counter
   Determiners); Homicidal Seclusion (*one*, creature Determiner).
   All 28 gained faces' precision-bearing Ability lines were inspected;
   no unintended new analysis was observed.
4. **Deviations and STOPs** — One Construction, `AdverbCardinalModifier`,
   and its declared `CardinalModifierUse` feature supply EX1. No Category,
   frame, numeral Construction or old route was added/deleted. EX3 needs no
   `NumericQuantityConjunct` edit: Arabic amounts and scalar values already
   use the existing modifier slot. No other precision/approximation Adverb
   exists in the declared inventory, so the class ruling adds no lexemes.
   The glossary's PP-only Cardinal Modifier definition was broadened, and
   Cardinal Modifier Use was added with EX1–EX2 as its project authority.
   Eight integration tests are registered once in `suite_3`. No STOPs.
5. **Tests** — `oyqrxmwz`: `cargo test -p deckmaste_english_v3` passed
   (1,084 tests), and `cargo test -p deckmaste_lexical_source` passed
   (86 tests). `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`
   passed. Initial English clippy found two test-helper borrowing issues;
   fixed by borrowing the expected Reading and retaining the full-value
   comparison. Restored 0 / re-spelled 0 / ignored 0 / added 8 / removed 0;
   removals: none. Citation checks: no noncompliant citations, zero stale;
   no new CR citations to bless. At `ummqklns` after Kata refresh,
   `cargo build -p xtask`, both full test suites and both clippy commands above
   passed again. The routine stale workspace was realigned; no conflicts.
6. **Timing** — `oyqrxmwz`: bounded AFTER census wall 16.03 seconds,
   six workers, `--samples-per-face 0`, measured under `/usr/bin/time -v`.
   The verification budget replaces the catalog's full census requirement.
