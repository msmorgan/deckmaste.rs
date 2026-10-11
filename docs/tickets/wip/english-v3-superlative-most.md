---
needs: []
---
# Read superlative *the most* + noun: each player who controls the most creatures

## Why

*the fewest creatures* reads and *most creatures* reads; *the most
creatures* does not. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:the most` 10 sole-blocker
faces. `cargo xtask english-v3 grep -i '\bthe most\b'` finds **33** faces,
32 unread. Cut by what follows:

| Shape | Failing lines | Faces | Sole failing line |
|---|---|---|---|
| *the most* + noun (*life*, *cards in hand*, *creatures*, *lands*, *votes*, *Wizards*) | 24 | 23 | 21 |
| … of which without *or tied for most N* | 13 | 13 | 12 |
| *the most common color* (Adverb *most* + Adjective) | 10 | 9 | 8 |

The 11 lines with *or tied for most life / votes / creatures* have a second
cause (*tied for* reads nowhere). The clean set is the **12**.

## Goal

*most* after the definite article reads as the superlative quantifier of a
plural or mass noun (*the most creatures*, *the most life*, *the most cards
in hand*), as Object of *control* / *have* and as Complement of *with*, with
byte-exact realisation.

## Analysis

CGEL Ch. 13 §6.3.2, p. 1166, [15]: "iii *Kim had interviewed most
candidates.* [superlative or proportional]", "iv *Kim had interviewed the
most candidates.* [superlative only]"; "The two readings are grammatically
distinguished in that only the superlative one allows the insertion of
*the*"; "*most* can be replaced by *least* or *fewest* only in the
superlative reading". Those back *the most* + noun as the superlative of the
degree determinative, distinct from proportional *most*. The internal
structure (what *the* and *most* are to each other) is not settled by the
passage: project ruling.

Existing declarations: `vocab:Determinative/Most` (`lexicon/core.ron`,
`DeterminerUse: PluralOrMass`, `DeterminerKind: Ordinary`), which reads bare
(*most creatures* 1) as Determiner; `vocab:AttributiveAdjective/Fewest`
(category Adjective, `QuantitativeAdjectiveComplementUse: Yes`), which is
how *the fewest creatures* reads today (as an attributive Adjective under
the article; confirm the Construction with a probe on the claim parent); `vocab:Adjective/Greatest`, `vocab:Adjective/Least`. The
definite article is `vocab:DefiniteMarker/The`.

Orchestrator rulings 2026-10-10:

- SM1. One route for the superlative quantifiers: *the most N* reads the way
  *the fewest N* reads, with *the* as Determiner and the superlative as an
  internal pre-head Modifier of the Nominal. *most* declares that use (data)
  on its existing Determinative Lexeme if a Determinative can fill the
  Modifier slot through a declared feature (the cardinal does, see
  `CardinalPremodifiedNominal`); otherwise by a refinement of that
  Construction's licence. Do not mint an Adjective Lexeme *most* beside the
  Determinative, and do not re-categorise *fewest* in this ticket.
- SM2. The licence reads declared features: the Modifier use on *most* and
  an outer definite Determiner. The head must be plural or mass by *most*'s
  declared `DeterminerUse`. No test on *most* or *the*.
- SM3. One analysis per string. The twins to avoid: (a) bare *most
  creatures* keeps exactly its Determiner Reading (proportional or
  superlative is not a syntactic difference there, per [15iii]); it gains no
  Modifier Reading; (b) *the most creatures* does not read with *the*
  determining a Noun Phrase *most creatures* (Determiner over a determined
  NP); (c) *the most common color* is not this: *most* there modifies the
  Adjective and stays unread until its own ticket; this change must not read
  it as *the* + *most* + [*common color*].
- SM4. Positions come from the NP: Object of *controls* / *has*, Complement
  of *with* (*the player with the most life*), with *among your opponents*
  or *in hand* attaching as they do after *the fewest cards in hand* (3+
  today).
- SM5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: the one covered *the most* face and the covered *most* /
  *fewest* faces (`(?i)\b(most|fewest|least)\b`: 147 faces) must keep their
  counts. An increase on a bare *most* face is an SM3(a) defect; report any
  increase with its shape before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--readings 3`): witness 0. Re-probe on the claim parent first.

- No Witnesses: "Each player who controls the most creatures investigates.
  Then destroy all creatures." (controls "Each player who controls three
  creatures investigates." 1; "Each player who controls the fewest creatures
  investigates." 1)
- Wild Mammoth: "At the beginning of your upkeep, if a player controls more
  creatures than each other player, the player who controls the most
  creatures gains control of this creature." (control with "that player" as
  Subject 1)
- Tectonic Hellion: "Whenever this creature attacks, each player who
  controls the most lands sacrifices two lands of their choice." (control
  "Each player who controls three lands sacrifices two lands of their
  choice." 1)
- Ghazbán Ogre: "At the beginning of your upkeep, if a player has more life
  than each other player, the player with the most life gains control of
  this creature."
- Sokenzan Renegade, the Subject "the player who has the most cards in
  hand"

Also 0 today (`noun-phrase`): "the most creatures"; "the most life"; "the
player with the most life"; "the player with the most cards in hand".
Controls that must not change (`noun-phrase`): "the fewest creatures" 1;
"most creatures" 1; "the greatest power" 1; "the player with the fewest
cards in hand" 3+; "the player who controls the fewest creatures" 1; "the
player with 3 life" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(most|fewest|least)\b` (147 faces) plus 300 random
covered controls (`--random 300 --seed 20261010`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; SM5 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_lexical_source`,
and for `deckmaste_english_v3` if a licence changes (a rebuild after a
grammar edit takes about 2–3 minutes and the six test binaries about 75
seconds; lexicon data loads at runtime). Tests assert the role (*the* as
Determiner, *most* as the superlative Modifier of the head noun, the SM3
negatives), never an incidental count or a Debug-printed tree. Attestation
by `cargo xtask english-v3 grep` only. Glossary: add the superlative
quantifier use (CGEL p. 1166 [15iv]) in the landing.

## Out of scope

- *or tied for most life / votes / creatures / common* (18 faces, 16 sole):
  the Adjective *tied* with a *for* Complement and bare superlative *most*.
- *the most common color among all permanents* (9 faces, 8 sole): Adverb
  *most* forming an analytic superlative Adjective Phrase.
- *the spell most recently cast this turn* (1 face).
- Re-categorising *fewest* as a Determinative.

## Landing record

1. **Census delta** (`nklknwxy`, before/after; claim parent `ytpquprw`,
   grammar base `qklnposw`): 446 faces, comprising all 147 supported faces
   matching `(?i)\b(most|fewest|least)\b` anywhere plus 300 covered random
   controls, seed 20261008 (one overlap). Covered 378 → 389; Readings
   7,055 → 7,253. Native `english-v3 diff --lines`: lost none, gained 11,
   decreases 0, increases 0, digest-only 2. Digest classes: *fewest*, one
   line on Gor Muldrak, Amphinologist (1 → 1); *least*, one line on
   Witch-king, Bringer of Ruin (14 → 14). Both replace the adjective-phrase
   wrapper with the shared internal Modifier route, preserving lexical
   identity and attachment. Evidence: `target/english-v3/english-v3-superlative-most/`
   (`manifest.json`, `before.json`, `after.json`, `diff.json`).
2. **Witnesses** (`nklknwxy`): all five ticket witnesses and four failing NP
   fragments re-probed at the claim parent with 0 Readings; none dropped.
   After: No Witnesses 1, Wild Mammoth 1, Tectonic Hellion's trigger 1,
   Ghazbán Ogre 1, Sokenzan Renegade's Subject 4 (its *fewest* control 4).
   *the most creatures*, *the most life*, *the player with the most life*
   each have 1; *the player with the most cards in hand* has 3 (control 3).
   Cardinal/that-player sentence controls each retain 1; *the fewest
   creatures*, bare *most creatures*, *the greatest power*, *the player
   who controls the fewest creatures*, *the player with 3 life*, *the least
   life*, *the fewest lands*, bare *fewest creatures* and bare *least life*
   retain 1. *the most common color* retains 0. Both roundtrip laws are
   exercised by independently built article/Modifier/head values.
3. **Spot checks** (`nklknwxy`): ten newly covered faces reviewed with *the*
   as Determiner and *most* as internal Modifier of the Nominal. Object of
   *controls*: No Witnesses (*creatures*), Wild Mammoth (*creatures*),
   Tectonic Hellion (*lands*), Thoughtbound Primoc (*Wizards*), Scholarship
   Sponsor (*lands*). Object of *has*: Sokenzan Renegade (*cards in hand*).
   Complement of *with*: Adamaro, First to Desire (*cards in hand*),
   Galactus, Devourer of Worlds (*life among your opponents*), Ghazbán Ogre
   (*life*), Tales of the Ancestors (*cards in hand*). All 11 gains were
   probed, including Wild Dogs; enumeration complete, byte-exact, no issues.
   Wrong-analysis-only gains: 0. Independent ambiguity stays, including
   the 4-versus-4 Subject and 3-versus-3 *with* controls of W9-A.
4. **Deviations and STOPs**: owner ruling implemented by moving all three
   existing Lexemes onto one `SuperlativePremodifiedNominal` Construction
   with category-preserving lexical forms and `SuperlativeQuantifierUse`.
   This distinguishes the quantifier's head licence and determiner
   requirement from ordinary adjectival Premodification. The existing
   `AdjectivePremodifierUse` grammar guard, declared No on *fewest* and
   *least*, retires their superseded nominal route; their bare uses and
   quantitative-preposition uses remain. No Lexemes or frames minted.
   *Most* retains PluralOrMass and requires a definite outer Determiner;
   *fewest* declares PluralCount; *least* retains its unrestricted head
   distribution. The initial bounded census found Witch-king's 15-Reading
   face lost when *least* was restricted to PluralOrMass: restored its
   singular count *power* use and added the same card as a regression test.
   Glossary added Superlative Quantifier and Superlative Quantifier Use;
   CGEL p. 1166 backs the meaning, SM1–SM3/owner rulings the internal
   structure. Binding seed 20261008 used. STOPs: none.
5. **Tests** (`nklknwxy`): `cargo test -p deckmaste_english_v3` passed
   (1,093 passed, one inherited ignore); repeated in full after the census
   regression fix. `cargo test -p deckmaste_lexical_source` passed (44).
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Restored 0 / re-spelled 1 / newly ignored 0 / added 5 / removed 0.
   Re-spelled `vocabulary_only_witnesses_preserve_counts_and_roles`:
   both Gor Muldrak cases keep their cards and Reading counts, with the
   expected constituent changed from AdjectivePhrase *fewest* to Nominal
   *fewest creatures*, reflecting the retired wrapper. Added tests are
   `superlative_quantifiers` in `suite_6`, registered once. The inherited
   ignore is `mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`;
   its existing blocker is `english-v3-mana-symbol-verbless-subject` MS4's
   governed Mana Phrase Subject route. No test removals to justify.
6. **Timing** (`nklknwxy`): the bounded 446-face landing census took
   22.61 s wall time under `/usr/bin/time -v`, with 6 workers and
   `--samples-per-face 0`.
