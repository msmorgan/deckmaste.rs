---
needs: [english-v4-stratum-5-comparatives-measures-and-partitives, english-v4-stratum-5-residual, english-v4-intern-declaration-identities, english-v4-zero-alloc-chart-state, english-v3-disable-frozen-tests]
---
# Stratum 6: Coordination and Right-Node Raising

Implement **Stratum 6** in `crates/deckmaste_english_v4` per
[`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§3, §4.1, §6.1):

- **Binary and Oxford-Comma Serial Coordination:** `A and B`, `A or B`, `A
  and/or B`, and 3+-member Oxford-comma chains (`A, B, and C`; `A, B, or C`;
  `A, B, and/or C`) across `Clause`, `Predicate`, `NounPhrase`, `Nominal`,
  `PrepositionPhrase`, `AdjectivePhrase`, `AdverbPhrase`, `DeterminerPhrase`,
  and `Cost`, plus comma/semicolon-coordinated sequences where licensed by
  Oracle punctuation rules.
- **Correlative Coordination:** `both A and B`, `either A or B`, `neither A nor
  B`, and `not only A, but also B`.
- **Compiler-Derived Right-Node Raising (Shared Complements):** Shared right-edge
  complement satisfaction across coordinated verbs, prepositions, adjectives,
  and nouns (`enters or leaves [the battlefield]`, `put into your hand or exile
  [target card]`, `greater than or equal to [X]`).

## Gate

- 100% of corpus units in Strata 0–6 parse as **read** (0 `known-wrong-only`)
  and satisfy both round-trip laws (`v is an admitted Reading of s => render(v) = s`
  and `v is independently constructed and grammatical => v is a Reading of parse(render(v))`).
- Stratum 0–6 judgment suite passes with zero regressions.

Standard constraints apply.

## Owner ruling: accepted wrong Readings

Against `docs/decisions/english-lexical-analysis.md` (line 97: a newly
admitted wrong Reading is a defect, never a gain), the owner explicitly
accepted the attachment ambiguities that serial unlike Object lists add. In
each, a first Noun Phrase conjunct takes a keyword through its own *with* or
*from among* phrase, which syntax cannot settle. Derivations before → after:
Riftmarked Knight 36 → 72; Liberated Livestock 12 → 36; Odric, Blood-Cursed
36 → 52; Mascot Exhibition 8 → 24; Aragorn, Company Leader 9 → 18; Grimdancer
5 → 9; Triplicate Titan 1 → 4; Trostani's Summoner 1 → 4; Hungry for More
2 → 4; Greymond, Avacyn's Stalwart 2 → 3. The owner likewise accepted twelve
units that keep a narrow Targeting Marker reading beside the wide one, owned
by `english-v4-stratum-7-discourse-and-long-tail`.

The owner also kept *with [keyword and Noun Phrase]* as it reads: a keyword
and a Noun Phrase coordinate as the Complement of *with*. Polukranos, Engine
of Ruin went from 32 to 128 derivations, accepted at 128; it has 64 at landing,
after the later Noun Phrase coordination splits.

## Judgment retirements

Lines are numbered as they stood when the ticket was claimed (`nzwzwknt`).

- `correlative_adjectives.txt:5` (`+Nominal creature both red and green`):
  retired by owner ruling as unattested. Oracle text uses the prenominal *both
  red and green creature* or the predicative *is both red and green*; a bare
  postpositive Adjective Phrase has no corpus witness. It has no Reading.
- `functions_coordination.txt:6–8` (`+Predicative red and a creature`, *red,
  white, and a creature*, *a creature and red*, each asserting an Adjective
  Phrase beside a Noun Phrase): retired by owner ruling as unattested. No
  corpus unit coordinates an Adjective Phrase with a Noun Phrase
  predicatively, and building it, even for determined Noun Phrases, added
  wrong Readings on about 25 units (*if [this creature attacked] or [blocked]
  this combat*). They are grammatical English, but the asserted analysis is not
  built: lines 6 and 7 still have one Reading, as a Noun Phrase coordination of
  the colour noun *red* (and *white*) with *a creature*, which the judgments
  did not assert; line 8 has none.
- `notation_coordination.txt:13, 18` (`+NounPhrase one and one creatures`,
  `*NounPhrase one and one creature`): retired as semantically anomalous and
  unattested; neither now reads. Lines 12 and 17 (*two or one creature(s)*)
  were rewritten to proximity agreement instead (CGEL, Ch. 5 §18.4,
  pp. 508–509).

## Single-value features

`docs/decisions/english-lexical-analysis.md` §3 reports a class with one
member, or a feature with one holder, as a finding. Two features this ticket
adds have one value each, and so act as privative marks whose absence is the
unmarked case:

- `AmountKind: ["ComparativeBounded"]` marks a Comparative-bounded Amount
  (*3 or less*), which only a noun taking an Amount may use. Every other
  Amount site excludes it; a two-valued feature would make every numeral carry
  the unmarked value and change no analysis.
- `Targeting: ["Marked"]` marks a Noun Phrase or Nominal under the Targeting
  Marker. It is held by Noun Phrase, Nominal and Targeted Nominal, and is read
  only by coordination schemas that keep the Marker's scope wide.

The owner may still direct a restructure; neither feature changes a Reading
by its spelling.

1. **Census delta** (measured on `qopyqvns`): faces read 20,391 → 28,707;
   total Readings 173,844 → 393,896; lost faces: Bladehold War-Whip
   (deferred to Stratum 7 by the owner).
2. **Judgment set**: 2,988 enforced at `COMPLETED_STRATUM` 6, all hold; 187
   pending.
