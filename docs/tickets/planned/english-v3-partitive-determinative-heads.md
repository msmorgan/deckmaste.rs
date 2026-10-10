---
needs: []
---
# Read a partitive headed by *any*, *all*, *either* or *another*: If any of that mana is spent on a creature spell, … / remove all of them

## Why

The Determinative-headed partitive reads for *each* only (NP "each of those
cards" 1, "one of those cards" 1 through the Cardinal route). NP "any of
those cards" 0, "any of that mana" 0, "all of them" 0, "all of those cards"
0, "either of those creatures" 0, "another of those counters" 0. Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): key
`at:any of` 15, part of failing token `of` 77.
`cargo xtask english-v3 grep -i '\b(any|either|another|all) of\b'` finds
**35** faces, all unread; **34** have that line as their only failing line:
*any of* 19 (19 sole), *all of* 8 (8), *either of* 6 (5), *another of* 2 (2).
`english-v3-systemic-residuals` lists "Explicit partitives *any of / all of /
none of* (0 covered)" as owned there; this ticket takes *any* and *all* over.

## Goal

A Determinative that declares the partitive use heads an NP with an *of*
Complement: *any of those cards*, *any of that mana*, *any of the exiled
card's colors*, *all of them*, *either of those creatures*, *another of those
counters*, with the number each head declares and byte-exact realisation.

## Analysis

CGEL Ch. 5 §9.2, p. 411: "The partitive may be explicit, with an *of* phrase
in post-head modifier position"; "Here the head is followed by a complement
consisting of *of* + a partitive oblique, an NP which can be plural,
non-count, or, under restricted conditions, count singular", with [6] "*some
of the books* … *all of them*" and "*some of the meat* … *all of it*". Those
back a Determinative fused head with a plural or non-count oblique. Which
Determinatives Oracle text licenses, and the number each exports, are project
rulings. Glossary: **Partitive Noun Phrase**, **Fused Determiner-Head**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`DeterminerHeadPartitiveNounPhrase` (`[head: lexical(Determinative), " ",
complement: PrepositionPhrase, modifiers: repeat(PartitiveModifier, "")]`,
`require head.NominalComplementMarker = Of`, `require
complement.ObliqueNumber = Plural`, `require head.DeterminerUse =
SingularCount`, `export number = Singular`). Only
`vocab:FloatedQuantifier/Each` declares `NominalComplementMarker: Of`
(`lexicon/core.ron`). `Determinative/Either` and `Determinative/Another` are
`SingularCount` and lack only the marker; `Determinative/Any` is
`Unrestricted` and `FloatedQuantifier/All` is `PluralOrMass`.

Orchestrator rulings 2026-10-10:

- PD1. Refine `DeterminerHeadPartitiveNounPhrase`; no new Construction.
  *either* and *another* are data: declare `NominalComplementMarker: Of`.
- PD2. For *any* and *all* the two `require`s become tables over declared
  features: the head's `DeterminerUse` against the oblique's `ObliqueNumber`
  (as `partitive_oblique` does for the Cardinal route), and the exported
  number. *all of* + plural is Plural; *any of* + plural admits both numbers
  (attested: *if any of those cards remain exiled* and *if any of those cards
  shares a card type*: two Readings of the NP are not expected, the verb
  selects); *any of that mana* / *any of that damage* is Singular.
- PD3. The licence is the Determinative's declared marker and
  `DeterminerUse`; no Determinative is named in the grammar.
- PD4. One analysis per string. *each of those creatures* keeps exactly its
  Readings (133 covered lines contain *each of*). The twin to avoid: *all* as
  a floated quantifier or predeterminer followed by an *of* PP attached
  elsewhere (*put all of them …* must not also read as *put all* + PP).
- PD5. Declare only attested heads. *none of* (3 faces; *none* is an unknown
  word), *both of* (1), *some of* / *neither of* / *most of* (0) are not
  declared here; do not add a check whose only job is to block them.
- PD6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected (0 of the 35 faces are covered; *any of* / *all of* occur
  on no covered face). Report any increase with its shape before landing.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`, enumeration complete). Trunk then moved to
`pxmykwnl` and the prebuilt probe no longer loads plugin data, so controls
marked (census) give the covered line's `reading_count` in that file.
Re-probe every witness and control on the claim parent first.

- Chrome Mox: "{T}: Add one mana of any of the exiled card's colors."
- Arena of Glory: "{R}, {T}, Exert this land: Add {R}{R}. If any of that mana
  is spent on a creature spell, it gains haste until end of turn."
- Mindblade Render: "Whenever your opponents are dealt combat damage, if any
  of that damage was dealt by a Warrior, you draw a card and you lose 1 life."
- Ludevic's Test Subject: "{1}{U}: Put a hatchling counter on this creature.
  Then if there are five or more hatchling counters on it, remove all of them
  and transform it."
- Tamiyo, Field Researcher: "+1: Choose up to two target creatures. Until
  your next turn, whenever either of those creatures deals combat damage, you
  draw a card."

Probed 0: "If any of those cards remain exiled, draw a card."; "Remove all of
them."; "Destroy any of them that are Walls." Controls that must not change:
NP "each of those cards" 1; NP "one of those cards" 1; "Destroy one of them."
1; "If one of those cards remains exiled, draw a card." 1; Stall for Time
"Tap up to two target creatures. If this spell was kicked, put a stun counter
on each of those creatures." 1 (census); Strategic Planning "Look at the top
three cards of your library. Put one of them into your hand and the rest into
your graveyard." 3 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b([Aa]ny|[Aa]ll|[Ee]ither|[Aa]nother|[Ee]ach|[Bb]oth|[Nn]one|[Nn]either|[Ss]ome) of\b

(395 faces: the 35 plus the *each of* control class) plus 300 random covered
controls, `--workers 6 --samples-per-face 0`, compared with `cargo xtask
english-v3 diff` against the claim parent. Zero lost; PD6 for increases. Full
`cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end.
PD1 is lexicon data and loads at runtime; PD2 is a grammar edit (rebuild
about 2–3 minutes, the six test binaries about 75 seconds). Tests assert the
role (the Determinative as fused head, the *of* PP as its partitive
Complement, the exported number; the PD4 negative), never an incidental count
or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *none of* (3 faces), *both of* (1): vocabulary or unattested class.
- Quantity-headed partitives (*one or more of*, *up to two of*, *that many
  of*, *1 of that damage*): `english-v3-partitive-quantity-phrases`.
- *all but one of them*: `english-v3-all-but-exception`.
- Unifying the two partitive Constructions' oblique checks (routed by
  `english-v3-systemic-residuals` to `english-v3-licence-plumbing-economy`).
- Faces whose line has a second blocker (e.g. Three Wishes, *any of those
  cards you didn't play*; Squall, Gunblade Duelist).

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
