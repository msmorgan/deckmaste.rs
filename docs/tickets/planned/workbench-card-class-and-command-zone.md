---
needs: []
---
# Give `CardClass` its command-zone arm, and settle the type line's leftovers

Three card-frame residues left by
`docs/tickets/done/workbench-closure-flip-risks.md`, which widened the type
catalog with six command-zone types and then explicitly deferred the frame work
they imply. `workbench-multiface-cards` was the card-frame round that could have
taken them and did not; nothing owns them now.

## 1. `CardClass` has no command-zone arm

Quoted from the closure round's ledger:

> `cardClassOf` is binary, so a conspiracy, dungeon, phenomenon, plane, scheme
> or vanguard card classifies as `PermanentCard` and is read by
> `cardAbilityOk PermanentCard`. That is wrong for a card [CR#110.4] keeps off
> the battlefield, and it is why `selenia` typechecks. A third class belongs to
> a card-frame round, not to a catalog widening.

The ask is the third class and the `cardAbilityOk` rows that follow from it —
what abilities a card that never becomes a permanent may carry.

## 2. `typesCombinable` does not refuse a command-zone type beside another

`[Conspiracy, Creature]` passes, because neither `permanentType` nor `spellType`
is True for it. The closure round recorded this as tolerated overgeneration
rather than a defect, on the doctrine that a refusal needs a rule: "No rule
refuses the combination outright … a card-frame round can tighten it if a rule
turns up." So the deliverable is the rule search, and a pin only if it lands
one.

## 3. `typePrintOrder`'s six provisional values

The closure round gave the six new rows positions 9–14 and said so:

> It is spelling-only and no gate consumes it, so the values are provisional and
> belong with `workbench-type-line-order-is-spelling`.

That ticket is closed. Either confirm the six positions against printed cards
and drop the "provisional" marking, or move the table where the spelling
boundary keeps it — the type line's order is spelling content, per that ticket's
own verdict.

## Consumption boundary

`idris/src/Experimental/Words.idr` (`CardClass`, `cardClassOf`, `typesCombinable`,
`typePrintOrder`), `idris/src/Experimental.idr` (`cardAbilityOk` and the card
frame's readers), the pin modules `idris/src/Experimental/Proofs*.idr`, and the
evidence bench `idris/src/Experimental/Cards.idr`. If §3 moves the print order
out of the workbench it lands beside the declarations in
`crates/deckmaste_english_v2`, as `union-spellings.md` did.

## Acceptance

- A command-zone card no longer reads through `cardAbilityOk PermanentCard`, and
  the term that motivated the finding (`selenia`) is re-checked against the new
  class.
- §2 ends with either a cited rule and a pin, or a written statement that no
  rule refuses the combination.
- `typePrintOrder`'s six values are no longer marked provisional.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
