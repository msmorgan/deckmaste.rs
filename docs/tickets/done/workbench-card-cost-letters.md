---
needs: []
---
# Thread the card's own mana cost letters into its text

Routed from `workbench-anaphora-mentions-and-creation`'s split into five
sub-tickets (2026-08-27). The split report's dispositions table found this
item belongs to neither the parent umbrella nor any of its five sub-rounds: it
is a `Card.idr` face-law/telescope change, not a mention change, so no
anaphora sub-round touches it.

## The gap

`Spell` effects are typed at `[]`, so a card's own mana cost is not yet
threaded into the telescope its text elaborates against. Prosperity's `{X}`
and its text's "X" are one variable only in prose today — the grammar has no
way to make the card's printed cost and the card's printed reference to that
cost's variable the same binding. The fix, as already recorded in
`docs/decisions/oracle-text-is-forward-anaphoric.md` under its obligation
clause ("A card's own mana cost is not yet threaded … [CR#107.3k] is the rule
to read first"):

```
Card.text : AbilitySeq (costLetters cost)
```

`Card.text`'s telescope becomes indexed by the letters extracted from the
card's own printed mana cost (`costLetters cost`), rather than by `[]`, so an
`AbilitySeq` built from it starts with those letters already in scope for a
later `Amount`/`Quality` read to pick up.

[CR#107.3k] is the rule to read first — verify its exact wording via the
`mtg-rules` skill's scripts, never from memory, before touching `Card.idr`'s
signature.

## Consumption boundary

`idris/src/Experimental/Card.idr` (`Card.text`, `costLetters`, the face-law
telescope), the pin modules `idris/src/Experimental/Proofs*.idr` for whatever
proof obligation `Card.text`'s new index incurs, evidence bench
`idris/src/Experimental/Cards.idr` (Prosperity). No Rust crate.

## Acceptance

- `Card.text`'s telescope is indexed by `costLetters cost` rather than `[]`,
  and every existing card benched against the old signature still
  typechecks under the new one.
- Prosperity's `{X}` and its text's "X" read as the same binding.
- [CR#107.3k] is cited for the exact mechanism, verified against the CR text
  in-round, not from memory.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As landed

`CardFace.text : AbilitySeq (costLetters cost)`, exactly as the ticket
specified, and every card already benched typechecks under it unchanged.

- `Words.idr`: `costLetters : Maybe ManaCost -> Bindings` — `[letterB X]`
  where the printed cost writes the variable symbol, `[]` otherwise. ONE
  letter, not a set: [CR#107.3p] gives Y the same rules as X, but
  `ManaSymbol`'s variable arm is unlettered and no printed mana cost writes
  a second variable.
- `Card.idr`: the `CardFace.text` field is now indexed by its own cost's
  letters; `CardText`, `CardChapters` and `CardBox` are generalized from
  `AbilitySeq []` to `AbilitySeq bs`. `AltFace.text` stays at `[]`: a
  nonmodal back face prints no cost [CR#202.3a] and a flip half's
  alternative characteristics apply only on the battlefield [CR#710.2],
  where [CR#107.3g] treats `{X}` as 0.
- `Macros.idr`: `cardOf` and `card` take `AbilitySeq (costLetters cost)`.
- Pins (`Cards.idr`): `prosperityCostLetters`,
  `prosperityTextReadsCostLetter` (the text's "X" now mints nothing — it
  reads the cost's), `textAloneOnceMintedItsOwnLetter` (the same "X" against
  the old empty telescope minted a second binding — the gap, as a term),
  `noVariableSymbolNoLetter` and `noCostNoLetter` (why nothing else moved).
- `docs/decisions/oracle-text-is-forward-anaphoric.md`'s obligation clause is
  rewritten from "not yet threaded" to what landed, since the tracked
  decision would otherwise be stale the moment this commits.

**DEVIATION — [CR#107.3k] is not the rule this mechanism rests on.** The
ticket and the ADR clause both said "[CR#107.3k] is the rule to read first".
Read in full it says the opposite of what a card-level scope wants: *"If an
object's activated ability has an {X}, [-X], or X in its activation cost, the
value of X for that ability is INDEPENDENT of any other values of X chosen
for that object …"* — and it closes by declaring itself an exception to
[CR#107.3i]. It is the exception that BOUNDS the threading, not its licence. The rules that license it are
[CR#107.3a] (the caster announces X as the spell is cast, and any X in the
spell's mana cost equals that value while it is on the stack) and
[CR#107.3i] ("Normally, all instances of X on an object have the same value
at any given time"). Both are cited at the field and in `costLetters`, with
[CR#107.3k] and [CR#107.3j] recorded there as the exceptions.

**Measured, tolerated overgeneration.** The face's telescope reaches an
activated ability written inside the text, so a `{X}` activation cost on an
`{X}`-cost card would read the face's letter where [CR#107.3k] wants an
independent one. Reachable but small: 3 supported cards (Chamber Sentry,
Defenders of Humanity, Wren's Run Hydra). Fixing it belongs to the ability's
own telescope — where an activation cost's `{X}` already mints into
`costIntro` — not to the face law, so it is recorded rather than taken.
**Remainder for the coordinator.**
