---
needs: []
---
# Read *all* as a Predeterminer of a definite Noun Phrase: Discard all the cards in your hand, then draw that many cards.

## Why

*all cards in your hand* and *the cards in your hand* each read; *all the
cards in your hand* does not. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:all the` 17 sole-blocker
faces. `cargo xtask english-v3 grep -i '\ball the\b'` finds **24** faces,
all unread; **21** have that line as their only failing line. By shape:
*discard(s) all the cards in your/their hand* 19 (two inside modal bullets);
*exile / put all the cards from your hand / library / graveyard* 4; *win all
the flips* 1 (Squee's Revenge). 4 of the 21 have a second cause (*that many
cards plus / minus one*: `english-v3-degree-quantity-arithmetic`). *all* +
demonstrative or genitive is nearly absent (`\ball (those|these|your|their|
its|that)\b`: 1 face) and *both the* has 0.

## Goal

*all* before a definite Noun Phrase (*all the cards in your hand*) reads as
a Predeterminer, an external Modifier of that Noun Phrase, with byte-exact
realisation and plural agreement from the host.

## Analysis

CGEL Ch. 5 §12, pp. 433–434: predeterminers "enter into construction with an
NP, not a nominal"; [1i] "*She had lost* [*all her money*]" labelled
[universal quantifier]; [2i] "*all the books/sugar/morning*, *all these
books*, *all that work*, *all Kim's friends*"; "*All* and *both* occur before
the definite determiners: the definite article, the demonstratives, and
genitives." And p. 434: "in each pair *all* is determiner in the version
without *the* and predeterminer in the one with *the*." Those back *all* as
Predeterminer before a definite NP and as Determiner otherwise.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`PredeterminerNounPhrase` (`[modifier: lexical(Adverb), " ", head:
NounPhrase]` and `[modifier: lexical(Noun), " ", head: NounPhrase]`,
`require predeterminer_host(modifier.PredeterminerUse, head.Definiteness) =
Yes`), feature `PredeterminerUse { No, Multiplier, Fraction }`, table
`predeterminer_host` (`(Multiplier, Definite)`, `(Fraction, Definite)`).
*all* is `vocab:FloatedQuantifier/All` (`lexicon/core.ron`, `DeterminerUse:
PluralOrMass`), a Determinative. Glossary: **Predeterminer**,
**Predeterminer Use**.

Orchestrator rulings 2026-10-10:

- PA1. Refine `PredeterminerNounPhrase`: one more form with a Determinative
  modifier, and one more `PredeterminerUse` value for the universal
  quantifier with a `predeterminer_host` row for a Definite host. *all*
  declares that value (data). No new Construction.
- PA2. The licence is the declared `PredeterminerUse` and the host's
  `Definiteness`; no test on *all* or *the*. Demonstrative and genitive
  hosts follow from `Definiteness` if they export Definite; do not add a
  check to block them, and do not declare *both* (0 faces).
- PA3. Number: the host NP's number is exported, as for the other
  Predeterminers, and the host must be compatible with *all*'s declared
  `DeterminerUse` (plural or mass), read from the features both already
  carry. *all the card* need not read and is not blocked by a named guard.
- PA4. One analysis per string. The twins to avoid: (a) *all cards in your
  hand* keeps exactly its Determiner Reading (CGEL's "determiner in the
  version without *the*"); it gains no Predeterminer Reading over a bare
  plural; (b) *all the cards* does not also read with *all* as Determiner of
  a nominal *the cards*, or as a floated quantifier; (c) *all other
  creatures*, *all three* keep their Readings (CGEL p. 434 takes *all* as
  Determiner before a cardinal); (d) Multiplier and Fraction Predeterminers
  (*twice that many*, *half the …*) do not stack with *all*.
- PA5. *all of the cards* (partitive) is a different Construction and stays
  out.
- PA6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face contains *all the*, and PA4(a) forbids a
  second Reading of *all* + bare plural (1,767 faces contain *all*). Report
  any increase with its shape before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Tolarian Winds: "Discard all the cards in your hand, then draw that many
  cards." (control "Discard all cards in your hand, then draw that many
  cards." 3+; "Discard the cards in your hand, …" 3+)
- Awaken the Erstwhile: "Each player discards all the cards in their hand,
  then creates that many 2/2 black Zombie creature tokens."
- Robber Fly: "Whenever this creature becomes blocked, defending player
  discards all the cards in their hand, then draws that many cards."
- Endurance: "When this creature enters, up to one target player puts all
  the cards from their graveyard on the bottom of their library in a random
  order." (may have a second cause; report)
- Forgotten Creation, first sentence: "At the beginning of your upkeep, you
  may discard all the cards in your hand." ("You may discard all the cards
  in your hand." 0)

Also 0 today (`noun-phrase`): "all the cards in your hand"; "all those
cards"; "both the cards". Controls that must not change (`noun-phrase`):
"all cards in your hand" 1; "the cards in your hand" 1; "all other
creatures" 1; sentence "Each player discards the cards in their hand, then
draws that many cards." 2. The without-*the* controls for Awaken the
Erstwhile, Robber Fly and Endurance were not probed (the trunk moved under
the prebuilt binary); probe them on the claim parent.

## Method

Standard constraints apply. Bounded census: every supported face matching
`(?i)\ball\b` (1,767 faces; *all* gains a declared use, so every face with
the word is in the selector) plus 300 random covered controls (`--random 300
--seed 20261010`), `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; PA6 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_english_v3` and `deckmaste_lexical_source`, once,
at the end: a rebuild after a grammar edit takes about 2–3 minutes and the
six test binaries about 75 seconds, so batch edits; lexicon data loads at
runtime. Tests assert the role (*all* as Predeterminer of the definite NP,
the host's number exported, the PA4 negatives), never an incidental count or
a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.
Glossary: extend **Predeterminer** and **Predeterminer Use** with the
universal quantifier (CGEL Ch. 5 §12(a), pp. 433–434) in the landing.

## Out of scope

- *that many cards plus one* / *minus one* after the discard
  (`english-v3-degree-quantity-arithmetic`).
- *all of the …* partitives; *both*.
- *win all the flips* beyond what PA1 gives (Squee's Revenge has other
  unread material).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
