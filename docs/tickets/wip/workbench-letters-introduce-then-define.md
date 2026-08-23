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
