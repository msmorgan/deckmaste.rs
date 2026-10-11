---
needs: []
---
# Declare the adjective *tied* with its *for* Complement, and the noun *vote*: If two or more creatures are tied for least power, you choose one of them.

## Why

*tied* is an unknown word, and the noun *vote* does not exist (only the verb:
"Each player votes for a player." 2, "Those votes are revealed." 0). Blocker
report at trunk `qzprwuxz` (`target/english-v3/blockers-wave9.txt`):
`word:tied` 24 sole-blocker faces, 51 touched. `cargo xtask english-v3 grep
-i '\btied\b'` finds **51** faces, all unread; **47** have that line as their
only failing line. `\bvotes?\b` finds 39 faces, all unread (every council
card); the union of *tied* and the noun uses of *vote* is 58 faces. Shapes of
the 51:

- predicative, *(is|are) tied for …*: 20 faces, 18 sole (*If two or more
  creatures are tied for least power*, *or is tied for the most*);
- postmodifier coordinated with a *with* PP, *… with the most life or tied
  for most life*: 16 faces, 14 sole;
- bare predicative, *the vote is tied*: 13 faces, 12 sole. All 13 are
  council cards whose line also contains undeclared option labels (*If
  knowledge gets more votes or the vote is tied*; `word:knowledge` beside
  `word:tied`), held under OW5 of `english-v3-option-word-choice-list`; they
  are not gained here. The reachable set is the 38 *tied for* faces (35 with
  one failing line).

The *for* Complement is a superlative NP without *the* on 30 faces (*tied for
most life / least power / greatest / lowest life total*); NP "most life" 1,
"the greatest power" 1, "the lowest life total" 1, but "least power" 0,
"greatest power" 0, "lowest life total" 0, "most votes" 0.

## Goal

*tied* reads as an Adjective, alone or with a *for* Preposition Phrase
Complement, as predicative Complement and as postpositive Modifier, including
coordinated with a *with* PP; *vote* reads as a count noun; the determinerless
superlative NP reads as the Complement of *for*. Byte-exact realisation.

## Analysis

CGEL Ch. 6 §3.1, p. 543: "We review here a range of constructions with PP
complements. For each of the prepositions concerned we give … a sample of
adjectives that license complements headed by this preposition"; p. 544
"Adjective + *for*", [11] "*responsible for the poor performance*". That backs
an adjective licensing a *for* PP Complement; *tied* is not in CGEL's sample,
so its membership is a project ruling. Ch. 15 §3.2, p. 1326: "a difference of
category is generally tolerated where there is likeness of function"; [13i]
"*It was* [*extremely expensive and in bad taste*]" (AdjP + PP as predicative
complement). That backs AdjP + PP coordination under one function; applying it
to two postmodifiers of a noun (*player with the most life or tied for most
life*) is a project ruling.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`ComplementedAdjective` (`[head: lexical(Adjective), " ", marker:
lexical(Preposition), " ", complement: AccusativePhrase]`, `require head.frame
= AdjectivalPrepositionComplement`, `marker.InternalisedComplementMarker =
Yes`) and `IntransitiveAdjective`; `SuperlativePremodifiedNominal` (reads
*most life*; requires `SuperlativeQuantifierUse = Yes`); the postpositive
route `PostpositiveNominal`; the coordination schemas. Lexicon:
`deckmaste_lexical_source/lexicon` (`core.ron`, `vocabulary.rs`); the verb
`Vote` in `verbs.ron`.

Orchestrator rulings 2026-10-11:

- TF1. Data first. Declare the Adjective `Tied` with two frames, the
  Intransitive one and `AdjectivalPrepositionComplement` marked by *for*, on
  the model of the existing complemented adjectives; declare the count noun
  `Vote` (plural *votes*). Both are core English words here, not plugin
  terms. *tied* is declared as an Adjective, not derived as the participle of
  a verb *tie* (no verb *tie* is attested: `\b(ties?|tying)\b` finds two
  faces, the Flavor Word *Tie Up* and the noun in *until the tie is broken*,
  Timesifter; mint no lexeme for it, R-class).
- TF2. The licence is the frame and the marker's declared
  `InternalisedComplementMarker`. No construction or check names *tied*,
  *vote* or *for*.
- TF3. Determinerless superlative Complement (*for most life*, *for least
  power*, *for greatest*, *for lowest life total*): refine
  `SuperlativePremodifiedNominal` / the declared superlative class so that
  every lexeme of that class (*most*, *least*, *greatest*, *lowest*,
  *highest*, …) heads a bare NP wherever *most life* already does (R-class:
  the class, not the four attested members). If *greatest* alone after *for*
  (*tied for greatest*, 2 faces) needs the fused head, leave those two to the
  fused-head family.
- TF4. Coordination *with the most life or tied for most life*: one
  coordination of two postmodifiers of the noun, PP + AdjP, through the
  existing coordination schema with an unlike-category instance licensed by
  their shared Modifier function. No new Construction (R-budget); if the
  schema cannot take an unlike pair without one, STOP with the count (16
  faces) rather than adding a member-specific route.
- TF5. One analysis per string (R-one). *are tied for least power* is copula
  + AdjP; the *for* PP must not also attach as a clause Adjunct or as a
  benefactive of an intransitive *tied*. *the vote is tied* has the bare
  adjective only. In *with the most life or tied for most life* the
  coordination is of the two modifiers; *or* must not coordinate *life* with
  a nominal reading of *tied …*, nor the whole clause.
- TF6. Newly covered faces are gains. Covered faces containing *votes* are
  none (39 of 39 unread) and no covered face contains *tied*; the superlative
  refinement (TF3) can add Readings on covered faces containing a bare
  superlative before a noun: none is expected, since the bare NP is new only
  where it failed; report any increase with its shape. An increase in which
  *votes* reads as a noun where the verb stands (*each player votes for*) is
  a defect.
- TF7. If TF3 or TF4 needs more than a refinement, land TF1 with the shapes
  it reads (*the vote is tied*; *are tied for the most*) and return the rest
  with evidence; do not STOP the whole ticket.

## Witnesses

Each is the only failing line of its face in `blockers-wave9.json`. Whole
faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- Drop of Honey: "… If two or more creatures are tied for least power, you
  choose one of them." (sentence 0; control "If two or more creatures are
  tapped, you choose one of them." 2)
- Padeem, Consul of Innovation: "At the beginning of your upkeep, if you
  control the artifact with the greatest mana value or tied for the greatest
  mana value, draw a card."
- Hooded Horror: "This creature can't be blocked as long as defending player
  controls the most creatures or is tied for the most." (control without the
  *or* coordinate 3+)
- Council's Judgment: "… Exile each permanent with the most votes or tied for
  most votes."
- Wretched Banquet: "Destroy target creature if it has the least power or is
  tied for least power among creatures on the battlefield."

Controls that must not change: "Choose a player with the most life." 2;
"Destroy the creature with the least power." 2; "Draw two cards if you
control the creature with the greatest power." 2; "Each player votes for a
player." 2; NP "most life" 1; NP "the lowest life total" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \btied\b|\bvot(e|es|ed|ing)\b|\b(most|least|greatest|fewest|highest|lowest)\b

case-insensitively (R-reach: TF3 alters the superlative route, so every face
using a superlative is selected) plus 300 random covered controls (`cargo
xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; TF6 for increases; report
digest-only changes by class and name every re-spelled test. Full `cargo test
-p deckmaste_lexical_source` and `-p deckmaste_english_v3` with clippy
`--tests -- -D warnings` for both, once, at the end. Lexicon data loads at
runtime; a grammar edit costs a 2–3 minute rebuild and the six test binaries
about 75 seconds. Tests assert the role (*tied* as Adjective head, the *for*
PP as its Complement, the coordinated postmodifier, the TF5 negatives), never
an incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- Council option labels (*grace or condemnation*, *time or knowledge*): OW5 of
  `english-v3-option-word-choice-list`, an owner decision; the 13 *the vote is
  tied* faces wait on it. Planechase *Will of the Planeswalkers* faces (5:
  *planeswalk*, *chaos ensues*) are permanently out of scope.
- *Whenever players finish voting*, *While voting* (`english-v3-small-frames-
  batch-3` and its notes); *You choose how each player votes* (interrogative
  clause, 11 faces touched).
- *each player secretly votes for …*: `english-v3-small-frames-batch-3`.
- *tied for greatest* with a fused head (2 faces, TF3).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
