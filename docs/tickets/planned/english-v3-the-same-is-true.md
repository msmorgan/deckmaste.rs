---
needs: []
---
# Read anaphoric *the same* with *true for*: Creatures you control are the chosen type. The same is true for creature spells you control and creature cards you own that aren't on the battlefield.

## Why

Oracle text extends a statement to more objects or more abilities with one
fixed sentence, *The same is true for* X. Nothing in it reads: NP "the same"
0, "The same is true." 0, "The same is true for flying." 0, "This creature is
true." 0 (*true* is not a word of the lexicon). Blocker report at trunk
`ymqwzour` (`target/english-v3/blockers-wave8.txt`): `word:true` 23 sole, 25
touched. `cargo xtask english-v3 grep -i '\bthe same is true\b'` finds **25**
faces, all unread; **23** have that line as their only failing line. Every
one is sentence-initial *The same is true for* + Complement. By Complement: a
list of keyword names (*for first strike, double strike, deathtouch, …, and
vigilance*) 15 faces (13 sole); an NP or NP coordination (*for creature
spells you control and creature cards you own that aren't on the
battlefield*, *for artifacts and enchantments*) 10 (10 sole).

## Goal

*the same* reads as an NP in Subject function, *true* as a predicative
Adjective taking a *for* Complement, and that Complement as an NP or a
keyword series: "The same is true for creature spells you control and
creature cards you own that aren't on the battlefield.", "The same is true
for flying, deathtouch, double strike, …, and vigilance.", with byte-exact
realisation.

## Analysis

CGEL Ch. 13 §5.1, p. 1140, "Anaphoric use of *the same* in term comparison":
"The comparative complement, and hence the secondary term in the comparison,
is commonly left understood", [10ii] "*Kim certainly tried, and the same can
be said for Jill*". Ch. 17 §8, p. 1545: *same* "is also commonly found as
fused modifier-head", [5i] "… *and the same applies to public-service
managers*"; "In the fused-head construction … *same* has a special
interpretation, essentially 'the same thing', in an abstract sense." Those
back *the same* as an NP whose head fuses Modifier and Head, in Subject
function. *true for* + NP / keyword list is a lexical frame: project ruling.
Glossary: **Fused Head Use** (a Determinative's; the Modifier-Head fusion of
*same* is the sibling of `english-v3-fused-ordinal-head`'s ordinal: say so
in the landing if the glossary needs the term).

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`DiscontinuousComparativeNominal` with policy
`DiscontinuousComparativeModifier` (`governor.frame = DiscontinuousEquality`,
*the same name as* …), which is how *same* reads today (209 faces contain
*the same*, 123 covered, always before a noun); `FusedDeterminerHeadNounPhrase`
(a Determinative alone as NP); `AdjectivalComplement` and
`ComplementedAdjective` (an Adjective with a marked Complement, from
`docs/tickets/done/english-v3-become-adjectival-passive.md`);
`KeywordComplementPreposition` (a keyword phrase as Complement of a
Preposition, *with flying*).

Orchestrator rulings 2026-10-10:

- ST1. *true* is declared in `lexicon/core.ron` as an Adjective with one
  marked frame, *for* + Complement, read by the existing
  complemented-adjective route. It declares no Premodifier use
  (`AdjectivePremodifierUse: No`; *a true creature* is unattested) and no
  bare predicative use beyond what the frame's optionality gives (*is true*
  alone: 0 faces; do not add a check to block it).
- ST2. *the same* as an NP: follow whichever route
  `english-v3-fused-ordinal-head` establishes for a Modifier fused with the
  Head under a Determiner, if it has landed; otherwise one added form on the
  fused-head NP family (*the* + a Modifier that declares fused-head use),
  licensed by a feature *same* declares. Not a Pronoun lexeme, not a noun
  homograph of *same*.
- ST3. The *for* Complement is an NP (existing) or a keyword series. If the
  complemented-adjective route cannot take a `KeywordPhrase` series after
  the marker, reuse the Complement class `KeywordComplementPreposition`
  reads; that is data on the frame, not a new Construction. If it needs more,
  land the 10 NP faces and return the keyword lists with evidence.
- ST4. One analysis per string. *the same name as the exiled card*, *the same
  mana value*, *the same way* keep exactly their Readings: the fused NP must
  not make *the same* + noun read twice (fused NP + stranded noun). The
  sentence has one structure: [*The same*] [*is*] [*true for* X].
- ST5. Scope of the fused use: only Subject position is attested
  (`'\bthe same([.,]| (is|applies|as)\b)'` 30 faces, 25 of them this
  sentence). Object *do the same* is unattested; no check to block it.
- ST6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected; the 123 covered *the same* faces must keep their counts.
  Report any increase with its shape; under ST4 it is a defect.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Conspiracy: "Creatures you control are the chosen type. The same is true
  for creature spells you control and creature cards you own that aren't on
  the battlefield."
- Celestial Dawn: "Nonland permanents you control are white. The same is true
  for spells you control and nonland cards you own that aren't on the
  battlefield."
- Ward of Bones: "Each opponent who controls more creatures than you can't
  cast creature spells. The same is true for artifacts and enchantments."
- Odric, Lunarch Marshal: "At the beginning of each combat, creatures you
  control gain first strike until end of turn if a creature you control has
  first strike. The same is true for flying, deathtouch, double strike,
  haste, hexproof, indestructible, lifelink, menace, reach, skulk, trample,
  and vigilance."
- Maskwood Nexus: "Creatures you control are every creature type. The same is
  true for creature spells you control and creature cards you own that
  aren't on the battlefield."

Controls that must not change: Realmwright "Lands you control are the chosen
type in addition to their other types." 6 (census); Dune Chanter "Lands you
control and land cards you own that aren't on the battlefield are Deserts in
addition to their other types." 27 (census); Ashiok's Erasure "Your opponents
can't cast spells with the same name as the exiled card." 3 (census); Asha's
Favor "Enchanted creature has flying, first strike, and vigilance." 1
(census); NP "a card with the same mana value" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bthe same\b|\bis true\b` (209 faces: the 25, 61 other
unread, 123 covered) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; ST6 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_lexical_source` and
`deckmaste_english_v3`, once, at the end; a rebuild after a grammar edit
takes about 2–3 minutes, the six test binaries about 75 seconds; lexicon data
loads at runtime. Tests assert the role (*the same* as fused-head Subject;
*true* as predicative Adjective; the *for* phrase as its Complement; the ST4
negatives), never an incidental count or a Debug-printed tree. Attestation by
`cargo xtask english-v3 grep` only.

## Out of scope

- The 2 faces with a second failing line (Animus of Predation, Wretched
  Bonemass).
- Keyword lists containing a term that does not read alone (*landwalk*,
  *protection* without its quality in *fear, first strike, double strike,
  landwalk, protection, and trample*: Concerted Effort, Death-Mask
  Duplicant, Cairn Wanderer): they gain only if the bare-keyword route reads
  them; report, do not widen that route here.
- Other fused uses of *same* and the 61 other unread *the same* faces.

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
