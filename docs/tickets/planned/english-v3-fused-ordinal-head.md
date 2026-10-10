---
needs: []
---
# Read an ordinal as a fused Modifier-Head: This spell costs {1}{U} more to cast for each target beyond the first.

Split from the wave-8 item "fused cardinal and ordinal heads"; the cardinal
is `english-v3-fused-cardinal-head`.

## Why

*the first* with no noun after it reads nowhere (*the first target* reads).
Scoped out of `docs/tickets/done/english-v3-fused-head-noun-phrases.md`.
Blocker report at trunk `uoqyuxtnowpo`: key `at:first .` 24 sole-blocker
faces. `cargo xtask english-v3 grep '\bbeyond the first\b'` finds **24**
faces, all unread; 24 failing lines match, **23** of them the only failing
line of their face. 22 are the Strive line; Jungle Wurm has *for each
creature blocking it beyond the first*. *beyond* occurs nowhere else (24
faces for `\bbeyond\b`) but is declared and reads ("Draw a card for each
creature beyond this creature." 3): the ordinal is the gap.

## Goal

A definite NP whose head is an ordinal Adjective with no overt noun: *the
first*, as Complement of a Preposition (*beyond the first*), with the
*beyond* PP as postmodifier of the *for each* nominal.

## Analysis

CGEL Ch. 5 §9.3(c), p. 416, [24i]: "*The first student wanted to take
linguistics, but* [*the second*] *did not*", under the heading "Ordinal
adjectives" in the section on fused Modifier-Heads; "there appears to be no
constraint against the NP being indefinitely determined". Ch. 6 §2.1, p. 529,
[3ii] lists "fused modifier-head" among the further adjectival functions
(*the rich*, *the bigger of the two*). Those back an ordinal Adjective as
fused Modifier-Head under a Determiner. The route and the restriction to
attested ordinals are project rulings.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`OrdinalPremodifier` is where *first* reads today (before a noun);
`DeterminedNounPhrase` (`[determiner: lexical(Determinative), head:
Nominal]`) reads *the first target*; `FusedDeterminerHeadNounPhrase` is the
Determiner-Head fusion and is **not** this (here the Determiner *the* is
overt and the Modifier fuses with the Head). The landed *the other* is a
noun under a Determiner (ruling FH1 of the fused-head ticket), not a fusion.

Orchestrator rulings 2026-10-10:

- FO1. One small route: a Nominal consisting of an ordinal Adjective alone,
  taking the place of the head under `DeterminedNounPhrase`. Refine the
  existing ordinal/Nominal Constructions if one can carry it; otherwise one
  new Nominal Construction is authorised. No route through
  `FusedDeterminerHeadNounPhrase`, and no noun homograph of *first*.
- FO2. The licence is a declared feature on the ordinal lexeme (its
  fused-head use), read by that Construction. No test on *first*, *beyond*,
  the Determiner or a card.
- FO3. Declare the use only on ordinals attested bare. Grep before declaring:
  `\bthe (first|second|third|last)( (is|was|to|into|and|or)\b|[.,])` finds 26
  faces (25 unread), nearly all *beyond the first*; record what the rest are.
  Do not add a check whose only job is to block an unattested ordinal.
- FO4. Definite Determiner only (*a second* is unattested: pruning rule).
  Express it through the existing determiner-selection feature the fused-head
  landing added for *other* (ruling FH7 there), not a guard.
- FO5. One analysis per string. The twins to avoid: (a) *the first target*,
  *the first time*, *the first spell you cast each turn* keep exactly their
  Readings: the fused Nominal takes no following noun and is not a
  premodifier host; (b) *for the first time each turn* gains no Reading with
  *the first* as an NP and *time* outside it; (c) *first strike* is
  untouched; (d) *whichever comes first* / adverbial *first* is not this
  route.
- FO6. `vocab:Preposition/Beyond` is declared (`PrepositionFunctionLicence =
  Modifier|PredicativeComplement`, NP Complement) and is not changed.
- FO7. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected; any increase is reported with its
  shape before landing.

## Witnesses

Each is the face's only failing line. Probed 2026-10-10 on the `uoqyuxtnowpo`
binary (`--readings 3 --as-card`): 0; control is the same line with the noun
restored.

- Hour of Need: "Strive — This spell costs {1}{U} more to cast for each
  target beyond the first." 0 (*beyond the first target* 3)
- Launch the Fleet: "Strive — This spell costs {1} more to cast for each
  target beyond the first."
- Silence the Believers: "Strive — This spell costs {2}{B} more to cast for
  each target beyond the first."
- Jungle Wurm: "Whenever this creature becomes blocked, it gets -1/-1 until
  end of turn for each creature blocking it beyond the first." 0 (*beyond
  the first creature* 2)

Sentence probes, 0 today: "This spell costs {1} more to cast for each target
beyond the first."; "Draw a card for each creature beyond the first."; "Put
the first into your hand." Controls that must not change: "This spell costs
{1} more to cast for each target." 2; "This spell costs {1} more to cast for
each target beyond the first target." 3; "It gets -1/-1 until end of turn for
each creature blocking it." 1; "Whenever this creature becomes tapped for the
first time each turn, draw a card." 3.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\b(first|second|third|last)\b|\bbeyond\b` (1,410 faces)
plus 300 random
covered controls, `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; FO7 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_english_v3` and `deckmaste_lexical_source`, once, at
the end: a rebuild after a grammar edit takes about 3 minutes and the test
binary about 7, so batch edits. Tests assert the role (the ordinal is the
fused Modifier-Head under the Determiner; the PP is the nominal's
postmodifier; the FO5 negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.
Glossary: add **Fused Modifier-Head** beside Fused Determiner-Head as part of
the landing.

## Out of scope

- The bare cardinal (`english-v3-fused-cardinal-head`).
- *whichever comes first* (*comes*, *whichever* are unknown words).
- Superlative fused heads (*the greatest*, *the most*).
- *a second* / *another* as heads.

## Landing record

The six items in `docs/tickets/README.md` ("Landing record").
