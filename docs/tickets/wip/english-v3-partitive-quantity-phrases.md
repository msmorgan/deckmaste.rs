---
needs: []
---
# Read a partitive headed by a quantity phrase: if two or more of those cards have the same name / prevent 1 of that damage / twice that many of those tokens

## Why

The Cardinal-headed partitive takes a bare cardinal word only: NP "two of
those cards" 1, "one of that damage" 1, but "2 of those cards" 0, "1 of that
damage" 0, "one or more of those cards" 0, "two or more of those creatures"
0, "up to two of them" 0, "that many of those tokens" 0, "twice that many of
those tokens" 0. Each of those quantities reads as a Determiner before a noun
("one or more creatures", "up to two cards", "twice that many tokens" 1,
"prevent 1 damage" 2). Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): keys `at:many of` 11, `at:more of`
10, `at:1 of` 10, part of failing token `of` 77.
`cargo xtask english-v3 grep -i` over the union of the four shapes finds
**59** faces, all unread; **50** have that line as their only failing line:

- Arabic numeral + non-count oblique, `\b\d+ of th`: 21 faces, 20 sole
  (*prevent 1 / 2 of that damage* 17 faces, 16 sole; the other 4 are *all
  but 1 of that damage*, which also need `english-v3-all-but-exception`).
- numeral + *or more*, `\b(one|two|…|\d+|X) or (more|fewer|less) of\b`: 15
  faces, 11 sole.
- *that many of*, `\bthat (many|much) of\b`: 14 faces, 11 sole (all under a
  multiplier: *twice / three times / half that many of*).
- *up to* N *of*, `\bup to (one|two|…|X|\d+) of\b`: 9 faces, 8 sole.

## Goal

The quantity position of the Cardinal-headed partitive admits the quantity
phrases that already function as a Quantitative Determiner, and an Arabic
numeral before a non-count oblique: *two or more of those creatures*, *up to
two of them*, *twice that many of those tokens*, *1 of that damage*, with
byte-exact realisation.

## Analysis

CGEL Ch. 5 §9.2, p. 411: "Here the head is followed by a complement
consisting of *of* + a partitive oblique, an NP which can be plural,
non-count, or, under restricted conditions, count singular"; [5ii] "[*Few* of
her friends] *knew she was ill*" (explicitly partitive). Ch. 5 §11(d)–(e),
p. 432: "[*More* than one] *glass was broken*" and "[*At the very most
twenty*] *people will agree to help*" show the modified quantifier as one
determinative phrase. Those back a fused head realised by a determinative
phrase and the plural / non-count oblique. Admitting each Oracle quantity
phrase as that head is a project ruling. Glossary: **Partitive Noun Phrase**,
**Quantitative Determiner**.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`PartitiveNounPhrase` (`[quantity: Cardinal, " ", complement:
PrepositionPhrase, modifiers: repeat(PartitiveModifier, "")]`,
`partitive_oblique(complement.ObliqueNumber)`, `export number =
quantity.number`). The quantity phrases are `QuantitativeDeterminer`s fed by
`CardinalDeterminer`, `ComparativeQuantityDeterminer` (over
`NumeralComparativeCoordination`, *one or more*), `PrepositionDeterminer`
(over `QuantitativePrepositionPhrase`, *up to two*) and
`DegreeDeterminativePhrase` (*that many*). Arabic numerals are
`NumericQuantityConjunct` (`numeral_kind = Arabic`), the quantity of
`CardinalMeasuredNominal` (*1 damage*).

Orchestrator rulings 2026-10-10:

- PQ1. Refine `PartitiveNounPhrase`: its `quantity` slot takes the category
  that already unifies these phrases (`QuantitativeDeterminer`), or added
  forms on the same Construction for `ComparativeQuantity` and
  `QuantitativePrepositionPhrase`. No new Construction. The exported number
  comes from the quantity's declared agreement (`DeterminerAgreementNumber` /
  number), so *one or more of those cards* stays as flexible as *one or more
  cards* is today.
- PQ2. The Arabic numeral is admitted through `QuantityConjunct`, and only
  with a Singular (non-count) oblique, read from `ObliqueNumber`: *1 of that
  damage*. *2 of those cards* is unattested (0 faces for `\b\d+ of (those|
  them)`): do not add a check whose only job is to block it, and do not build
  it either.
- PQ3. The multiplier stays where `english-v3-multiplier-predeterminer` put
  it: *twice* + [*that many of those tokens*]. If the predeterminer's host
  test rejects a partitive host, extend its declared host table; do not mint
  a second multiplier route.
- PQ4. One analysis per string. *two of those cards* keeps exactly one
  Reading: the bare cardinal must not read once as `Cardinal` and once as a
  `CardinalDeterminer`-fed quantity. *one or more of your opponents* has one
  structure ([*one or more*] + *of* PP), not *one* coordinated with [*more of
  your opponents*].
- PQ5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: 190 covered lines contain *one of (those|them)* and must
  keep their counts. Report any increase with its shape; under PQ4 it is a
  defect.
- PQ6. If one of the four shapes needs more than the slot refinement, land
  the others and return that shape with its evidence; do not STOP the whole
  ticket.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`, enumeration complete); trunk then moved to
`pxmykwnl` and the prebuilt probe no longer loads plugin data, so (census)
controls give the covered line's `reading_count`. Re-probe on the claim
parent first.

- Urza's Armor: "If a source would deal damage to you, prevent 1 of that
  damage." (probed 0; control "… prevent 1 damage." 2)
- Scalpelexis: "Whenever this creature deals combat damage to a player, that
  player exiles the top four cards of their library. If two or more of those
  cards have the same name, repeat this process."
- Mangara, the Diplomat: "Whenever an opponent attacks with creatures, if two
  or more of those creatures are attacking you and/or planeswalkers you
  control, draw a card."
- The Mana Rig: "{X}{X}{X}, {T}: Look at the top X cards of your library. Put
  up to two of them into your hand and the rest on the bottom of your library
  in a random order."
- Parallel Lives: "If an effect would create one or more tokens under your
  control, it creates twice that many of those tokens instead."

Controls that must not change: NP "two of those cards" 1; NP "one of your
opponents" 1; NP "twice that many tokens" 1; NP "that many tokens" 1;
"Destroy one of them." 1; Bruvac the Grandiloquent "If an opponent would mill
one or more cards, they mill twice that many cards instead." 1 (census);
Commune with Evil "Look at the top four cards of your library. Put one of
them into your hand and the rest into your graveyard. You gain 3 life." 3
(census); Sanguine Indulgence "Return up to two target creature cards from
your graveyard to your hand." 1 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(one|two|three|four|five|six|seven|eight|nine|ten|\d+|X|many|much|more|fewer|less) of\b

case-insensitively (410 faces: the 59 plus the bare-cardinal partitive
control class) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; PQ5 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_english_v3` (and
`deckmaste_lexical_source` if data changes), once, at the end; a rebuild
after a grammar edit takes about 2–3 minutes, the six test binaries about 75
seconds. Tests assert the role (the quantity phrase as fused head, the *of*
PP as partitive Complement, the PQ4 negatives), never an incidental count or
a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Determinative heads (*any of*, *all of*, *either of*):
  `english-v3-partitive-determinative-heads`.
- *all but 1 of that damage*, *all but one of them*:
  `english-v3-all-but-exception`.
- *twice / three times as much of that mana* (3 faces, the *as much* equative).
- *a third of*, *a tenth of* (3 faces), *in excess of* (2), *to a maximum of*
  (1), *one of the same mana symbol* (Jegantha).
- Lines with a second blocker: *put up to one of them on top … and the rest
  on the bottom* (2 faces), Celes, Party Dude, Regicide, Innkeeper's Talent
  (*twice that many of each of those kinds of counters*).

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
