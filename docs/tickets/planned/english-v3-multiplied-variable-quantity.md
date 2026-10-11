---
needs: []
---
# Read a multiplier or fraction over the variable X as a quantity: Put twice X stun counters on it; you gain half X life.

## Why

Residue MP4 of `docs/tickets/done/english-v3-multiplier-predeterminer.md`
("*twice X stun counters*, *twice X life* … have no definite host … otherwise
list them in the landing record"), re-measured as `english-v3-hygiene-3`
asked ("twice X / N times X + noun 13"). The predeterminer route reads a
definite host ("You gain twice that much life." 1; "Put twice that many stun
counters on it." 1; "X is twice its power." 1) and the bare variable reads as
a quantity ("You gain X life." 1; "Put X stun counters on it." 1), but a
multiplier or fraction over the variable reads nowhere: "You gain twice X
life." 0, "Each opponent loses two times X life." 0, "You gain half X life."
0, NounPhrase "twice X" 0. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.json`): `at:life .` (4 of 13),
`at:creature .`, `at:cards .`, `at:it .`, and the mid-sentence `at:rounded
down` / `at:rounded up`. `cargo xtask english-v3 grep -i '\b(twice|half|(two|
three|four|five|ten) times) X\b'` finds **27** faces, all unread; 27 failing
lines match, **19** the only failing line of their face (*twice X* 9, *half
X* 7, *N times X* 4; one line has two). Every one is followed by a noun
(*life*, *damage*, *cards*, counters, tokens).

## Goal

*twice X*, *N times X* and *half X* read as the quantity of a measured or
counted noun wherever bare *X* reads as that quantity, byte-exact, with the
trailing *rounded up/down* supplement the fraction already takes elsewhere.

## Analysis

CGEL Ch. 5 §12, p. 434: "Multipliers occur only with definite NPs", and
"Forms consisting of cardinal numeral + *times* … are clearly NPs. *Twice* and
the archaic *thrice* … are adverbs". That is why the predeterminer route
(host `Definiteness = Definite` in `predeterminer_host`) is right to refuse
*twice X life*: *X life* is not a definite NP. The analysis here is therefore
not a predeterminer: the multiplier scales the variable, and the scaled
variable is the noun's quantity ([[twice X] life], like [[X] life]). That
bracketing is a project ruling; CGEL has no passage on variables.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`ScalarVariable` as the `quantity` of `MeasuredNounPhrase` (*X life*) and of
the counted-noun route (*X stun counters*); `PredeterminerNounPhrase` with
feature `PredeterminerUse { Multiplier, Fraction, … }` and table
`predeterminer_host`; the arithmetic quantity work of
`english-v3-degree-quantity-arithmetic` (*that many plus one*, landed) and
`english-v3-half-rounded` (landed). Multiplier and fraction Lexemes already
declare `PredeterminerUse`.

Orchestrator rulings 2026-10-11:

- MV1. Refine the quantity position, not the predeterminer: the variable
  quantity gains an optional leading multiplier field (the compiler's
  optional lexical field with a trailing separator, R-budget), so *twice X*
  is one quantity. For the phrasal multiplier (*two times*, *three times*)
  reuse whatever constituent `PredeterminerNounPhrase` already takes as its
  modifier. No new Construction; `predeterminer_host` is not widened to
  indefinite hosts (that would contradict p. 434 and read *twice a card*).
- MV2. The licence reads the modifier's declared `PredeterminerUse`
  (`Multiplier` or `Fraction`) and the quantity's declared `ScalarVariable`.
  No test on *twice*, *half*, *X* or the noun.
- MV3 (R-class). Every Lexeme declaring `PredeterminerUse = Multiplier` or
  `Fraction` gets the position (*double X* is unattested; no check to block
  it, no Lexeme minted), and every declared scalar variable (*X*, *Y*).
  A numeral is not a variable: *twice 3 life* is not built.
- MV4 (R-one). One analysis per string. The twins to avoid: (a)
  `PredeterminerNounPhrase` over *X life* (the route MV1 refuses); (b)
  *twice* as a frequency Adjunct of the verb with *X life* its Object ("You
  gain twice X life." must not read as *gain X life, twice*; today "Investigate
  twice." 1 shows the Adjunct exists): if the frequency reading appears,
  STOP with both trees.
- MV5. *half X damage, rounded down, to any target* (Banshee, Eternal
  Flame): the supplement is the landed `english-v3-half-rounded` route; in
  scope if it attaches to the new quantity without a new form, otherwise list
  the faces (6 sole lines carry *rounded*).
- MV6. Bare value use (*equal to twice X*; "You gain life equal to twice X."
  0) is unattested (`grep 'equal to (twice|half) X'`: 0 faces): no work.
- MV7. Newly covered faces are gains. Absent optional fields are omitted from
  Debug, so covered digests must not move; if the quantity Construction's
  form changes, R-reach: every face with a variable quantity is in the
  selector and digest-only changes are reported by class. No Reading-count
  increase is expected on a covered face.

## Witnesses

Each is a sentence of the face's only failing line. Probed 2026-10-11 with the
trunk binary (`--category sentence --readings 8`; Torch the Witness with
`--as-card`): all 0.

- Procrastinate: "Put twice X stun counters on it."
- Sanguine Sacrament: "You gain twice X life."
- Debt to the Deathless: "Each opponent loses two times X life."
- Torch the Witness: "Torch the Witness deals twice X damage to target
  creature."
- Hydroid Krasis: "When you cast this spell, you gain half X life and draw
  half X cards."

Controls that must not change: "Put twice that many stun counters on it." 1;
"You gain twice that much life." 1; "You gain half that much life." 1; "X is
twice its power." 1; "You gain X life." 1; "Each opponent loses X life." 1;
"Put X stun counters on it." 1; "Investigate twice." 1; (as Torch the
Witness) "Torch the Witness deals X damage to target creature." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(twice|half|double|triple|times)\b` (the multiplier and
fraction surface; at least 303 faces) plus, if MV7's form change applies,
`\bX [a-z]`; plus 300 random covered controls (`--random 300 --seed
20261008`), `--workers 6 --samples-per-face 0`, compared with `cargo xtask
english-v3 diff` against the claim parent. Zero lost; MV7 for changes. Full
`cargo test -p deckmaste_english_v3` and clippy `--tests -- -D warnings`,
once, at the end (a grammar rebuild takes 2–3 minutes, the six test binaries
about 75 seconds). Tests assert the role (the multiplier is a dependent of
the variable inside the noun's quantity; MV4 negatives), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only.

## Out of scope

- *twice -X/-X* (1 face): a multiplier over an adjustment pair.
- *add twice that much {G}* (`english-v3-quantified-mana-symbol`).
- *it produces twice as much of that mana* / *three times as much* (Mana
  Reflection, Nyxbloom Ancient, Virtue of Strength; 3 faces): multiplier over
  an equality comparative.
- *three times that many of those tokens are created* (Ojer Taq).

## Landing record
