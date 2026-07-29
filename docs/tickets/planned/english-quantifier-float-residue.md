---
needs: []
---
**Quantifier-float residue — what remains after rounds `qfloat` and `anof`.**
Campaign-internal residue of `english-structural-recovery-zero`, split out so
the diagnosis survives in the tree; fold into the live campaign workspace with
`workflow claim english-quantifier-float-residue --into
english-structural-recovery-zero`.

Round `qfloat` (2026-07-27) landed its Stage 2 — the finite verbal quantifier
float, 123 spans / 106 faces. Round `anof` (2026-07-28) landed **both** stages
this ticket previously carried as outstanding:

- **Stage 3 (`any number of` notional plural concord) — LANDED**, 37 clause
  spans / 596 source tokens, plus one newly exposed embedded-rules span.
- **Stage 1 (sentence-initial cardinal case) — LANDED as a redesign.** The
  fix specified in `qfloat-plan.md` §3 was *not* used; see §2 below.

Counts below are unresolved rows against the post-`anof` census: clause
3342 / 63162, structural total 3477 spans / **3117 faces (9.8375%)**, noun
opacity 964. The designs are on disk at `recovery-harness/out/qfloat-plan.md`
(§3, §5) and `recovery-harness/out/anof-brief.md`.

## 1. Stage 3 landed — scope it by predicate shape, not by a verb regex

`any number of <plural NP>` built an ordinary nominal headed by the **singular**
noun `Number` with the referent in an `of`-complement, so it agreed singular and
every plural finite verb failed. Fixed with one appended `NounPhrase`
production, `AnyDeterminer NumberNoun Of NounPhrase`, gated categorically by
dedicated lexical slots (so the gate sits at scan, not reduce) and registered
with `precedence: 1` so the formal-singular reading still wins wherever it
completes. It lowers to the byte-identical ordinary nominal shape; only the
parse features differ.

**The plan predicted 29 rows and 37 cleared.** Every extra was the same
construction on a predicate no verb regex covered — `phase out` (Clever
Concealment, Guardian of Faith, No More), `have base power and toughness` (The
Bears of Littjara), `become a copy` (Polymorphous Rush), `become 3/3 artifact
creatures` (Depthshaker Titan), `gain double strike` (Phalanx Formation). This
is the second consecutive round in this family where a verb-regex inventory
under-counted; `qfloat`'s Stage 2 did the same. **Scope work in this area by
the finite-predicate shape.**

## 2. Stage 1 landed, but NOT as `qfloat-plan.md` §3 specified

**Do not re-apply plan §3, and do not make `Numeral::parse` case-insensitive.**
`numeral.rs` is a strict canonical notation codec — `canonical()` accepts only
input its own `format` reproduces — and that contract is deliberate and
property-tested. It was left untouched.

The real defect was at the grammar layer: every other closed-class lexeme
matches through `Parser::one_token_match` with `eq_ignore_ascii_case`, while the
numeral scanners fed raw surface text straight into the canonical codec. The
landed fix is a `parse_notation` helper in `grammar/mod.rs` routed through all
five scanner sites (the `Number` lexical slot, at-least, bounded, frequency, and
or-quantity), which retries a failed cardinal parse against the lowercased
surface under **two** gates:

- **`one` is excluded.** It is also a fused-head count noun. Letting `One`
  compete inside the quantity scanners resolves the ambiguity to a materially
  wrong tree (`target` becomes the finite verb, `become` a past-participle
  adjective) that renders byte-identical. `one_is_a_dispreferenced_fused_head_noun`
  and `number_literal_one_still_wins_over_the_noun_reading` both stay green.
- **The retry is sentence-initial only.** Without this gate the round-trip gate
  caught two real regressions: `Prisoner Zero` rendered as `Prisoner zero` and
  `Three Dog` as `three Dog`, because a capitalized cardinal-shaped word inside
  a proper name carried mid-sentence was matched as a numeral and re-rendered
  lowercase. `word_matches` and `catalog_matches` already refuse a capitalized
  non-sentence-initial token as a common-word reading for exactly this reason;
  the numeral path now uses the same idiom.

Result: the 10 opaque `Two` leaves are gone (noun opacity 974 → 964) and
`Secret Tunnel` cleared a whole clause span as well.

**The lesson worth keeping: a corpus blast-radius measurement scoped to
sentence-initial positions does not license an ungated fix.** The measurement
was correct about where the *gain* lives; it said nothing about where the
*retry* could fire. Gate the mechanism to the measured scope.

## 3. Remaining residue

- **The `one or two` capitalization asymmetry, 8 rows — deliberately not
  fixed.** Lowercase object-position `one or two target creatures` builds the
  single quantified nominal `Determiner::Target(Some(Quantity::Or(1, 2)))`;
  sentence-initial `One or two target creatures` instead builds a coordinated
  nominal with `One` as a fused head. Both render identically. The coordinated
  reading is the same analysis the grammar deliberately test-locks for `One or
  more target creatures`, so it is **consistent rather than wrong** — but the
  two spellings of one construction should not receive different structures.
  Fixing it requires resolving the fused-head tiebreak itself, not the case
  fold; that is the real open problem here.
- **Six `any number of` rows remain, each with an independent blocker**, none
  reachable by concord: `Agadeem's Awakening` (object-position `any number of`
  plus an imperative/attachment host gap), `Smoldering Stagecoach` (coordinated
  subject + `have cascade`), `Eidolon of Countless Battles` (double-pump `for
  each`), `Miasma Demon` (`up to that many ... get ...`), `Tiamat` (search/list
  + relative-clause attachment), `Phantasmal Form` (a `have ..., gain ..., and
  become ...` predicate run — belongs with
  `english-predicate-coordination-redesign`).
- **Inside the `any number of` nominal, `target` lowers as an ordinary positive
  `Adjective` modifier** rather than the `Determiner::Target` that the `up to
  two target creatures` path builds. Unchanged by `anof`; a nominal-internal
  category question, not a concord one.
