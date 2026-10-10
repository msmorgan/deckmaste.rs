---
needs: []
---
# Read a supertype Premodifier on a bound-stem keyword: Basic landcycling {1}{G}

## Why

*Landcycling {2}* and *Swampwalk* read; the same keywords with a supertype
before the stem do not. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:basic landcycling` 35
sole-blocker faces. `cargo xtask english-v3 grep -i
'\b(snow|legendary|nonbasic|basic) [a-z]*(walk|cycling)\b'` finds **44**
faces, all unread; all 44 failing lines match and **43** are the only failing
line of their face. By shape: *Basic landcycling <cost>* 36 (35 sole);
*Snow forestwalk* / *Snow swampwalk* / *Snow landwalk* 4; *Legendary
landwalk* 2; *Nonbasic landwalk* 2 (one as a granted keyword mention,
Trailblazer's Boots).

`english-v3-typecycling-bound-suffix` left this open (its R8b: *basic* is a
supertype premodifying the *land* stem, not a two-word stem; "record the
count and leave it for a follow-up"), and `english-v3-data-batch-2` deferred
it as needing more than data. This is that follow-up; the landwalk residuals
are the same gap.

## Goal

A bound-stem keyword whose stem is premodified by a supertype reads as a
Keyword Line and as a keyword mention, with byte-exact realisation:
*Basic landcycling {1}{G}*, *Snow forestwalk*, *Legendary landwalk*,
*has nonbasic landwalk*.

## Analysis

No CGEL citation applies (Oracle keyword formation). The glossary's **Bound
Keyword Surface** entry already names *nonbasic landwalk* and *Snow
swampwalk* as cased-together forms.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`BoundQualityKeyword` (`[quality: lexical(Noun), head: lexical(Keyword)]`,
`require quality.frame = BoundKeywordStem`) and `BoundQualityCostKeyword`
(the same plus `" ", cost: CostSymbols`, `head.KeywordParameterClass =
QualityCost`). Declarations: `plugins_v2/builtin/macros/keyword_abilities/
{landwalk,typecycling}.ron` (`bound_suffix: (surface, stem_class)`). The
supertypes are `vocab:Supertype/Basic`, `vocab:Supertype/Snow`,
`vocab:PredicativeAdjective/Legendary` (`lexicon/core.ron`), and *nonbasic*
is the class-declared negative formation.

Orchestrator rulings 2026-10-10:

- SB1. R8b stands: the supertype is a Premodifier of the stem, never part of
  a two-word stem Lexeme and never a new stem-class member. Refine the two
  existing Constructions with one optional leading slot
  (`[modifier: lexical(Adjective), " ", quality, head, …]`); no new
  Construction. If a form cannot be optional there, one added form per
  Construction is the fallback; a third Construction is a STOP.
- SB2. The licence reads declared features: the modifier's declared
  supertype class (the feature the Type Line and noun premodification already
  read) and the stem's `BoundKeywordStem` frame. No test on *basic*, *snow*,
  *land* or the keyword. If no declared feature separates supertype
  Adjectives from other Adjectives, STOP and report rather than listing
  words.
- SB3. Do not restrict which supertype meets which stem: *Snow forestwalk*
  and *Legendary landwalk* are attested, and no check is added whose only job
  is to block an unattested pairing.
- SB4. One analysis per string. The twins to avoid: (a) a compound stem
  Lexeme *basic land* beside the Premodifier analysis; (b) *Basic* read as a
  separate `BareKeyword` or Type Line fragment followed by *landcycling*;
  (c) in mentions, the Adjective attaching to a noun outside the keyword
  (*has nonbasic landwalk* has one Object, the keyword).
- SB5. Capitalization follows the glossary entry: the Premodifier takes the
  line-initial capital and the stem stays lowercase (*Basic landcycling*,
  *Snow swampwalk*); mid-line *legendary landwalk* (Livonya Silone) is all
  lowercase.
- SB6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face matches the attestation pattern. Any
  increase is reported with its shape before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Gleam of Resistance: "Basic landcycling {1}{W}"
- Ash Barrens: "Basic landcycling {1}"
- Rime Dryad: "Snow forestwalk"
- Ayumi, the Last Visitor: "Legendary landwalk"
- Dryad Sophisticate: "Nonbasic landwalk"

Also 0 today: "First strike; legendary landwalk" (Livonya Silone);
"Equipped creature has nonbasic landwalk." (Trailblazer's Boots, sentence).
Controls that must not change: "Landcycling {2}" 1; "Plainscycling {2}" 1;
"Swampcycling {2}" 1; "Forestwalk" 1; "Landwalk" 1; "First strike;
swampwalk" 1; "Equipped creature has swampwalk." 1; "Search your library for
a basic land card." (sentence) 3+.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)[a-z]*(walk|cycling)\b` (616 faces) plus 300 random
covered controls (`cargo xtask english-v3 diff --manifest … --random 300
--seed 20261010`), `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; SB6 for
increases. Full `cargo test -p deckmaste_english_v3` and `cargo clippy -p
deckmaste_english_v3 --tests -- -D warnings` (and the same for
`deckmaste_lexical_source` / `deckmaste_construction_core` if a declaration
or the keyword recipe changes), once, at the end: a rebuild after a grammar
edit takes about 2–3 minutes and the six test binaries about 75 seconds, so
batch edits; plugin `grammar:` data loads at runtime. Tests assert the role
(the supertype as Premodifier of the Bound Keyword Stem, the cost as the
keyword's parameter, the SB4 negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.
Glossary: extend **Bound Keyword Stem** with the Premodifier in the landing.

## Out of scope

- *gains snow landwalk of the chosen type* (Barbarian Guides, 1 face): the
  *of the chosen type* dependent of a keyword mention.
- Typecycling and landwalk engine bodies.
- Cycling-trigger text (*When you cycle this card*).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
