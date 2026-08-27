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
