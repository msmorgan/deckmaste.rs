---
needs: []
---
# Letters are introduced by use and discharged by a definition

`WhereLetter`/`WhereLetterStatic` lift "does something(X), where X is def"
into a scoping constructor with the definition first — a lifting device of
the kind the workbench exists to remove, and the grammar's only departure
from semantics-v2 §2's "argument order IS textual order". The `LetterWord`
parameter was minted for a dimension the rules close at two letters.

## Ruling (2026-08-22)

- **Reference runs forward.** In "Draw X cards, where X is N" the where-
  clause's X is the anaphor; the body's X is an *introducing* mention (a
  variable brought in by use), and the definition is a later predication on
  it — the same shape as a target followed by "that creature". No cataphora;
  the forward-anaphora ADR gains an **obligation** clause: a mention may carry
  an obligation discharged by a later step, checked at the ability boundary.
- **`Letter = X | Y`**, closed by [CR#107.3p] ("Y follows the same rules as
  X"); no other letter exists. Corpus: 0 `where L is` with L ≠ X standalone;
  4 lines define X and Y together (Aspect of Wolf, Phyrexian Ingester,
  Bioplasm, Souvenir T-Shirt).
- **`LetterVal l : Amount bs`** replaces `XVal` and `DefinedLetter`: its delta
  mints `letterB l` when no `l` is bound in `bs`, else reads it. Cost X and
  text X are one variable [CR#107.3c,107.3i].
- **`Define l amt : Effect bs`**, a telescope step in `Sequentially` and in
  `StaticParts`, gated on an `l` already in `bs` — "where X is …" with no X
  to define is the pin, on [CR#107.3c]'s presupposition. A second `Define l`
  in one ability is refused by the same count.
- **Discharge gate at `AbilityAt`**: every `letterB l` leaving the ability is
  matched by a `Define l` or by an `{X}`-class cost. OPEN: whether an
  uncosted, undefined X is CR-meaningless (pin) or controller-chosen /
  zero ([CR#107.3] parent text, [CR#107.3j]) — the consultant decides from
  the rule text; a pin only if a rule closes it.
- **Postposed "where" is spelling**: `Define` renders as ", where X is …"
  attached to its predecessor, never with "then".
- Deleted: `WhereLetter`, `WhereLetterStatic`, `LetterWord`, `letterB`'s
  word parameter, `countLetter` (→ per-`Letter` count), `Macros.whereLetter`,
  `Macros.whereLetterStatic`, the `DefinedLetter` rows of `ProofsAnaphora`
  (replaced by `LetterVal` introduces / `Define` resolves).

## Why it pays

149 of 1107 supported "where X is" lines define X from their own clause's
outcome ("the number of creatures destroyed this way", "that creature's
power"); only a telescope step reads that natively. Term order becomes text
order, and a realised card macro tree translates positionally.

## Consumption boundary

`idris/src/Experimental.idr`, `Words.idr`, `Events.idr`, `Macros.idr`,
`Proofs*.idr` (incl. `ProofsAnaphora`), `Cards.idr` (17 `WhereLetter` + 8
`WhereLetterStatic` sites), `docs/decisions/oracle-text-is-forward-anaphoric.md`
(the obligation clause). No Rust crate.

## Acceptance

- Every former `WhereLetter*` site benches in text order through `Define`;
  one self-referential witness (e.g. Phyrexian Rebirth) and one X/Y witness
  bench; `LetterVal` is the only letter reader.
- The discharge question is answered from the CR at the site.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

`Letter = X | Y` (Words.idr), closed by [CR#107.3p]; the `Kind` constructor is
`LetterK`. `Amount.LetterVal l` is the only letter reader and replaces both
`DefinedLetter` and `XVal`: its delta is `letterDelta l bs`, which mints
`letterB l` when the prefix counts no such letter and reads one otherwise.
`Effect.Define l amt` and `StaticEffect.DefinesLetter l amt` are telescope
steps gated on `So (anyOpenLetter l bs)`; both re-mark every open instance to
`TheD` in place (`defineLetter`). `StaticKind` gains `LetterDefinition`.

Deleted: `WhereLetter`, `WhereLetterStatic`, `LetterWord`, `DefinedLetter`,
`XVal`, `Macros.whereLetter`, `Macros.whereLetterStatic`, `notLetterRider` /
`NotLetterRider` and the `Continuously` gate it served.

Amounts now leave a trace where they did not: `Gets`'s two shifts
(`shiftDelta`), `DefinesPt`, `HasBasePt`, `CostsToCast`, `EntersWithCounters`
in `staticIntro`; a written token's P/T (`ptDelta`/`specDelta`) and the four
turn-structure one-shots in `effIntro`/`preIntro`/`annIntro`. `Macros.gets`
types its duration at the static's own output.

All 17 former letter sites (16 constructor + Krenko through the macro) re-bench
in text order. Two witnesses added: `phyrexianIngesterPump` (X/Y in one
statement) and `soulsMight` (a definition reading the mention its own clause
introduced).

### Ledger

**OPEN-1 — no discharge gate (ratified, landed).** [CR#107.3] gives an
undefined uncosted X its controller's choice and [CR#107.3j] gives a gained
ability's undefined X the value 0, so an open letter leaving an ability is
recorded, not refused. The reasoning sits at `LetterVal`'s docstring and in the
forward-anaphora ADR's new obligation clause; the obligation is soft.

**Deviation — cost X is introduced OPEN, and `badDefineAfterCostX` is not
landed.** The design had a cost's `{X}`/`-X` mint a definite `definedLetterB X`
and pinned "where X is" under such a cost. The citation audit showed the rule
cited for that pin argues the other way: [CR#107.3c] reads "an {X}, [-X], or X
in its cost **and/or** its text, and the value of X is defined by the text of
that spell or ability", which names the refused shape as rules-meaningful. 0
printed lines put an X-bearing activation or mana cost together with a "where X
is" that defines it, but a count is not a refusal. So `costIntro` mints
`letterB X` for both `Mana` (via `manaHasX`) and `LoyaltySymbol LoyaltyDownX`,
`definedLetterB` was dropped as unused, and the pin became the positive
`costXStaysOpen` in `ProofsAnaphora`. Cost X and text X remain one variable
[CR#107.3i].

**Deviation — no `Macros.whereXIs`/`whereYIs`.** The design added four
positional aliases over `Define`/`DefinesLetter`. With the constructors already
in printed order and binding no implicit a card writes, the aliases reorder
nothing; the bench uses the constructors directly and the ADR paragraph says so.

**Deviation — `nounDelta (LibrarySlice …)` does not thread its amount.** The
design's `amtDelta amt ++ nounDelta whose` typechecks in `Experimental.idr` but
breaks the elaboration of `ProofsC.badAfterReflexiveReadsTrigger` ("Can't bind
implicit {amt} of type `Amount ?bs`" while inferring the `Unspellable`
obligation type). No letter site in this round reads a library slice's amount,
so the threading was dropped rather than repaired by annotating an unrelated
pin. Consequence: "reveal the top X cards of your library, where X is …" (Hew
the Entwood) still cannot find its X.

**Deviation — `defineClosesLetter` needs `proof eq` + `rewrite`.** The design's
plain `with (openLetter l b)` does not abstract the scrutinee in the `False`
arm's goal.

**Self-referential witness — Phyrexian Rebirth not benched; Soul's Might
benched instead.** "Destroy all creatures, then create an X/X … token, where X
is the number of creatures destroyed this way" needs the count of a plural
anaphor. `CountOf`/`Aggregate` take a `Predicate`, not a `Noun`, so
`ThoseVerbed Destroy …` cannot be counted; and `Lookback = ThisTurn |
ThisCombat | LastTurn | ThisGame` has no "this way" value, so `HappenedTo`
cannot spell it either. All 67 supported `where X is .*this way` lines are
blocked the same way except Astarion's Thirst, whose second clause needs "a
commander creature you control" — `designationScope CommanderD = HeldByCard`
and `designationChecked CommanderD = False`, so `HasDesignation CommanderD` is
not a predicate. Soul's Might ("Put X +1/+1 counters on target creature, where
X is that creature's power") is in the same class — the definition reads a
mention its own clause introduced, which `WhereLetter`'s outer-typed `def`
could not see — and composes today.

**Phyrexian Ingester's "its" (design OPEN §4).** `That CardW` typechecks; the
witness stands as designed.

**`writtenBound (LetterVal _) = True` (design OPEN §5).** `writtenBound` has no
consumer anywhere in the repo (`comparableBound` and `letterDefines`, its
callers in `docs/idris-workbench-closure-tables.md`, no longer exist). The
merge of `XVal = True` with `DefinedLetter = False` is therefore inert; `True`
was kept because the letter is a token the text writes.

**`costActionOk (Define _ _) = True` (design OPEN §7).** Unmeasured, admitted on
over-generation: a definition instructs nothing at payment.

**A twin-Amount slot mints the letter twice (found in review).** `Gets`'s `pow`
and `tou` are both typed at `nomIntro n`, and a written token's P/T pair and
`HasBasePt`'s pair are both typed at their own `bs` — none of the three is a
telescope, but `staticIntro`/`ptDelta` now compose their deltas as if it were
(`shiftDelta tou ++ shiftDelta pow ++ …`). So "+X/+X" runs `letterDelta X` twice
against the same prefix and mints two `letterB X`: measured,
`countLetter X (staticIntro (Gets thisCreature (PtDown (LetterVal X)) (PtDown
(LetterVal X)))) = 2`. Exercised by Death's Shadow, Nightmarish End, Aettir and
Priwen and Dokai. Nothing refuses or admits wrongly because of it — the
`Define`/`DefinesLetter` gate is existence, not counted uniqueness, and
`defineLetter` re-marks every open instance — but the context records two
bindings for what [CR#107.3i] makes one variable, and the forward-anaphora
ADR's contract clause 2 ("every constructor telescope types each argument in
the previous arguments' output") does not hold of these three slots now that
their arguments carry deltas. Closing it means typing `tou` at `pow`'s output;
a follow-up round, not a mechanical fix.

**Card mana cost still unthreaded (design OPEN §2).** `Spell` effects are typed
at `[]`, so Prosperity's `{X}` and its text X are one variable only in prose.
Making them one binding means `Card.text : AbilitySeq (costLetters cost)` — a
follow-up round. [CR#107.3k] is the rule to read first.

**Pins.** Added: `badDefineWithoutUse` [CR#107.3c] (a definition with no letter
in scope); `badSecondDefine` [CR#107.3i] (the first settled every instance);
`badDoubleXRider`, `badDoubleStaticRider` [CR#107.3i] and `badUnlicensedY`
[CR#107.3p] in `ProofsD`, restated over the new shape. Retired:
`ProofsD.badUnlicensedX` — "Draw X cards" with no definition is now spellable
by OPEN-1's ruling ([CR#107.3], [CR#107.3j]), and the positive fact is carried
by `ProofsAnaphora.letterValIntroducesAtEmptyPrefix`.
`ProofsD.badRiderInsideDuration` — the shape it refused (a duration adverbial
inside a letter rider) no longer exists, the rider having become a sibling
`AndAlso` member. `badDefineAfterCostX` — never landed, see above.

**Gates.** `idris/scripts/build` 19/19 from clean; `grep` for `WhereLetter |
LetterWord | DefinedLetter | XVal` over `idris/src` empty; `Cards.idr` implicit
binds 0; `{default` in `Experimental*` 0; `cargo xtask cite check
--list-noncompliant` empty; `cite check` 0 stale (no new rules to bless — all
five are already in the lock); `cite audit --diff` 45 sites, each read.
Refusal probe: `Define X (Lit 1)` at `[]`, a second `Define X`, and `Define X`
after a `LetterVal Y` introduction are each rejected by the gate; the matching
good term compiles.

**Reviewer finding — twin `Amount` slots double-minted (CLOSED).** Threading the
deltas of two `Amount`/`PtShift` slots typed at the SAME context concatenated
two introductions of one letter: `Gets this (PtDown (LetterVal X)) (PtDown
(LetterVal X))` gave `countLetter X (staticIntro ...) = 2`, against the
forward-anaphora ADR's clause 2 (each argument typed in the previous arguments'
output) and against [CR#107.3i]. Every twin-`Amount` slot is now a telescope:
`Gets`'s `tou : PtShift (shiftIntro pow)` (new `shiftIntro`, beside
`shiftDelta`), `HasBasePt`'s and `ExceptPt`'s `tou : Amount (amtIntro pow)`,
`CompareAmt`'s `bound : Amount (amtIntro subj)`, and `TokenChars.pt`, whose
plain pair became the dependent pair `Maybe (p : Amount bs ** Amount (amtIntro
p))`. `staticIntro`/`ptDelta`/`condDelta` already concatenated in the composing
order and were left as written. `Macros.gets` and `Macros.creatureTokOf` follow
the new telescopes; the raw `MkToken (Just (Lit a, Lit b))` sites became `(Lit a
** Lit b)`. `DefinesPt` and `Times` have one `Amount` each and needed nothing.
The `ProofsD` letter pins needed no change (their second slot is `Lit 0`, which
is context-polymorphic). Witness: `ProofsAnaphora.twinShiftMintsOneLetter`
proves `countLetter X (staticIntro (Gets thisCreature (PtDown (LetterVal X))
(PtDown (LetterVal X)))) = 1`.
