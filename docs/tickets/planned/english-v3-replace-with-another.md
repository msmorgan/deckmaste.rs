---
needs: []
---
# Read *replace* X *with another*: Change the text of target spell or permanent by replacing all instances of one color word with another.

## Why

The text-changing cards share one sentence. Its frame reads with a declared
verb and no *with* phrase ("Change the text of target permanent by exiling
all instances of one color word." 2; NP "all instances of one color word" 2),
but *replace* is not a word of the lexicon, and the fused *another* that
closes the sentence reads nowhere (NP "another" 0; "Exile one color word with
another." 0). Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): `word:replacing` 11 sole, 12
touched. `cargo xtask english-v3 grep -i '\breplac(e|es|ed|ing)\b'` finds
**12** faces, all unread, **11** sole; every one is *by replacing all
instances of one* N *with* …: *with another.* 7 faces, *with another or one
basic land type with another* 4, *with Vampire* 1 (New Blood). Fused
*another* beyond these: `'\banother([.,]| (or|and)\b)'` adds 2 faces (*with
the same name as one another*, a different construction).

## Goal

*replace* reads with an Object and a *with* Complement, inside the *by* +
gerund-participial means phrase, and *another* reads as a fused
Determiner-Head in that Complement: "… by replacing all instances of one
color word with another.", with byte-exact realisation.

## Analysis

CGEL Ch. 5 §7.9, p. 391: "*Another* readily functions as fused
determiner-head", [59ii] "*Harriet supports* [*one team*], *and I support*
[*another*]"; "The contrast in [59ii] … is with singulative *one* and this
highlights the alternative sense." That backs fused *another* and its pairing
with a preceding *one* N. The verb frame is a lexical declaration (project
ruling). Glossary: **Fused Determiner-Head**, **Fused Head Use**.

Existing routes: `GerundComplementPreposition`
(`crates/deckmaste_english_v3/src/declarations.rs`; `[head:
lexical(Preposition), " ", complement: SecondaryVerbPhrase]`, *by paying*
…), which already reads the witness with another verb;
`FusedDeterminerHeadNounPhrase` (`[head: lexical(Determinative)]`, `require
head.FusedHeadUse = Yes`, `require head.DeterminerUse = Singular`, `export
number = Singular`), declared by `vocab:SingularDemonstrative/This` and
`vocab:CompoundDeterminative/Anywhere`; `vocab:Determinative/Another`
(`DeterminerUse: SingularCount`, `DeterminerAgreementNumber: Singular`; no
`FusedHeadUse`).

The shelved `english-v3-fused-cardinal-head` refined the same Construction
for bare cardinals and was stopped by ruling FC-R1: 29 gains against 67
Reading-count increases on covered faces, "without a real licence for where
a bare cardinal may head an NP". This ticket must not repeat that.

Orchestrator rulings 2026-10-10:

- RA1. `core-verb:Replace` is declared in `lexicon/verbs.ron` with one frame:
  Object + *with* + Object. Only the gerund-participle is attested (12
  faces); other forms follow the pruning rule for hand-declared forms.
- RA2. Fused *another* refines `FusedDeterminerHeadNounPhrase`: the
  `DeterminerUse` requirement becomes a table that also admits the value
  *another* declares, and *another* declares `FusedHeadUse: Yes`. Exported
  number stays Singular. No cardinal is admitted by this change (FC-R1
  stands): the table row is for the declared value of *another*, and if
  that value is shared with the cardinals (`SingularCount`), the licence is
  `FusedHeadUse`, which no cardinal declares.
- RA3. One analysis per string, measured, not argued: *another* stands
  before a nominal on 1,595 faces (*another target creature*, *another
  creature you control*). None may gain a Reading in which *another* is a
  fused NP and the following nominal attaches elsewhere. Any Reading-count
  increase on a covered face is a defect under this ruling; if the count is
  not zero, restore the change and return with the diff grouped by shape, as
  FC-R1's findings were.
- RA4. The partitive *another of those counters* (2 faces) belongs to
  `english-v3-partitive-determinative-heads`; this ticket does not declare
  the partitive marker.
- RA5. *with another or one basic land type with another* (4 faces) is a
  coordination of two Object + *with* clusters under one verb. It gains only
  if the landed selected-complement cluster coordination reads it; report,
  do not extend.
- RA6. Newly covered faces are gains (7 expected from the simple shape).

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Alter Reality: "Change the text of target spell or permanent by replacing
  all instances of one color word with another."
- Sleight of Mind: the same sentence.
- Glamerdye: the same sentence.
- Magical Hack: "Change the text of target spell or permanent by replacing
  all instances of one basic land type with another."
- Artificial Evolution: "Change the text of target spell or permanent by
  replacing all instances of one creature type with another. The new
  creature type can't be Wall."

Controls that must not change: "Change the text of target permanent." 1;
"Change the text of target permanent by exiling all instances of one color
word." 2; "Exile all instances of one creature type." 2; Alien Symbiosis "You
may cast this card from your graveyard by discarding a card in addition to
paying its other costs." 22 (census); Rock Soldiers "When this creature
enters, destroy up to one target noncreature artifact." 1 (census); any ten
covered *another target* … lines, counts recorded before and after.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\banother\b|\breplac(e|es|ed|ing)\b|\bby [a-z]+ing\b`
(*another* alone is 1,595 faces; the selector takes all of them because RA2
changes what the word can be) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff --lines`
against the claim parent. Zero lost and, under RA3, zero increases. Full
`cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end; a
rebuild after a grammar edit takes about 2–3 minutes, the six test binaries
about 75 seconds; lexicon data loads at runtime. Tests assert the role (the
*with* Complement of *replace*; *another* as fused Determiner-Head; the RA3
negative on *another* + nominal), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- New Blood (*with Vampire*: a bare subtype name as Complement), Balduvian
  Shaman and Spectral Shift (other blockers on the face).
- *one another* (Mechanized Production, Chrome Replicator).
- Bare cardinals as fused heads (`english-v3-fused-cardinal-head`, shelved).

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
