---
needs: []
---
**Quantifier-float residue — the two stages round `qfloat` did not land, and
the measured defect in one of them.**
Campaign-internal residue of `english-structural-recovery-zero`, split out so
the diagnosis survives in the tree; fold into the live campaign workspace with
`workflow claim english-quantifier-float-residue --into
english-structural-recovery-zero`.

Round `qfloat` (2026-07-27) landed **Stage 2 only** — the finite verbal
quantifier float, 123 spans / 106 faces. The full design is on disk at
`recovery-harness/out/qfloat-plan.md` and remains valid apart from the Stage 1
defect recorded below. Counts are unresolved rows measured against the
post-`qfloat` census (clause 3380 / 63774, structural total 3514 spans / 3143
faces, noun opacity 974).

## 1. Stage 1 — the specified fix is UNSOUND as written. Do not re-apply it.

The plan's §3 makes `Numeral::Cardinal` parsing ASCII-case-insensitive so that
sentence-initial `Two` and `One` stop being mis-analysed. The defect it targets
is real and is diagnosed to root cause:

- `numeral.rs:167-168` compares against an all-lowercase `SMALL_CARDINALS`
  table with `word == input`, an exact comparison, while every other
  closed-class lexeme matches through `Parser::one_token_match`
  (`grammar/mod.rs:1538-1542`) using `eq_ignore_ascii_case`.
- There is a **second** case-sensitive check: `Numeral::parse`
  (`numeral.rs:91-99`) delegates to `canonical()` (`numeral.rs:102-110`),
  which filters on `numeral.format(value) == input`. Patching only the table
  lookup produces a candidate that then fails canonical validation. Any fix
  must normalize before **both** checks.

**This was implemented, measured, and reverted.** It breaks a pre-existing
anti-misparse gate:

- `one_is_a_dispreferenced_fused_head_noun` (`tests/public_api.rs:882`),
  witness `"One or more target creatures become black until end of turn."`

Making `One` parse as a cardinal lets it compete inside
`scan_at_least_quantity`, and the parser resolves the resulting ambiguity to a
materially wrong tree: `target` becomes the finite verb and `become` is
demoted to a past-participle adjective inside the object nominal. It renders
byte-identical, so roundtrip cannot catch it.

That gate is a previous round's deliberate protection, and its sibling
`number_literal_one_still_wins_over_the_noun_reading` pins the other side of
the same tiebreak. **A redesign must keep both green.** Measured cost of not
having Stage 1: **zero recovery** — Stage 1 clears no structural row by
itself; its only effect is host-tree quality on 16 rows, and Stage 2 cleared
all 16 without it.

Stage 1's inventory is therefore **not a recovery inventory** but an accuracy
one, in two parts:

- **8 opaque `Two` leaves**, currently live in the census as a direct result
  of Stage 2 landing without it: Combo Attack, Huddle Up, Invigorated Rampage,
  Ruthless Disposal, Sick and Tired, Symbiosis, Twigwalker, Windborne Charge.
  Plus the 2 pre-existing ones, Profane Transfusion and Soul Conduit
  (`Two target players exchange life totals.`). Each is a genuine one-word
  lexical gap under an otherwise correct clause — a precision gain over the
  whole-sentence failures they replaced, not a regression.
- **A capitalization asymmetry on `one or two`, 8 rows.** Lowercase
  object-position `one or two target creatures` builds the single quantified
  nominal `Determiner::Target(Some(Quantity::Or(1, 2)))`. Sentence-initial
  `One or two target creatures` instead builds
  `Coordinated(Nominal { determiner: None, head: Singular(Word(One)) }, Or,
  Nominal { determiner: Target(Some(Exact(2))), head: Plural(Creature) })`.
  Both render identically. The coordinated reading is the same analysis the
  grammar deliberately test-locks for `One or more target creatures`, so it is
  consistent rather than simply wrong — but the two spellings of one
  construction should not receive different structures. Verified by tree read
  on `Appeal to Eirdu`.

## 2. Stage 3 — designed, unattempted, no defect known

Notional plural concord for `any number of <plural NP>`. **23 spans in the
float family, plus 6 off-regex `any number of ... each mill` rows.**

Diagnosed to root cause: `any number of target creatures` builds an ordinary
nominal whose head is the **singular** noun `Number` with the referent in an
`of`-complement, so it agrees singular and every plural finite verb fails.
`Any number of target creatures gets +2/+2 until end of turn.` parses clean;
`... get ...` does not. Not a subject-position gap, not capitalization, not
specific to `target`, and not specific to verbal vs copular predicates —
verified with a seven-cell probe grid and a tree read.

The plan (§5) specifies one narrow appended production
`NounPhrase -> AnyDeterminer NumberNoun Of NounPhrase`, deliberately keyed to
the closed-class `Any` determiner plus the `Vocab::Number` count noun rather
than a generic `Determiner Noun` prefix. **That narrowness is load-bearing**:
a generic prefix would be exactly the reduce-dead registration hazard the
invariant audit warns about. It must not become a `PartitiveHead` variant —
the syntactic head really is the measure noun; only the concord is wrong.

This stage **registers a production**, so §A of the invariant audit applies in
full: the categorical gate belongs at dot 1, not at reduce.

Stage 3 was never implemented — the round's mechanic was cut off by an
environmental connection failure after Stage 2's code was complete. No defect
is known or suspected.

## 3. Residue Stage 2 left behind

Named and measured, all still unresolved. These are the six rows the `qfloat`
ledger predicted would remain, each with an independent blocker:

- `Agadeem's Awakening` — `any number of` is an **object** here, and the
  stripped `Return from ... to ... <object>` plus relative host still fails.
  A separate imperative/attachment host gap; concord will not reach it.
- `Smoldering Stagecoach` — coordinated subject plus a `have cascade` host.
- `Eidolon of Countless Battles` — a double-pump `for each` predicate.
- `Miasma Demon` — `up to that many ... get ...` quantified host.
- `Tiamat` — search/list plus relative-clause attachment.
- `Phantasmal Form` — a `have ..., gain ..., and become ...` predicate run;
  belongs with `english-predicate-coordination-residue`.

Also recorded, out of scope and unfixed: inside the `any number of` nominal,
`target` lowers as an ordinary positive `Adjective` modifier rather than the
`Determiner::Target` that the `up to two target creatures` path builds.

## 4. One thing Stage 2 proved that the row inventory missed

The round was scoped by a regex over a fixed verb list, but the productions
key on the **finite verb phrase generally**. Five faces outside the scoped
inventory cleared as the same construction — `Null Chamber` (`each choose`),
`Mana Clash` (`each flip`), `Parker Luck` and `Keen Duelist` (`each reveal`),
and `Kozilek, the Broken Reality` (`each manifest`) — and two `embedded rules`
spans cleared alongside the clause spans.

**Scope future work in this area by the finite-predicate shape, not by a verb
regex.**
