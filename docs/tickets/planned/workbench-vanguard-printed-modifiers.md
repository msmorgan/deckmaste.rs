---
needs: []
---
# Give vanguard cards their two printed numbers

`docs/tickets/done/workbench-card-class-and-command-zone.md` already gave
vanguard cards a `CommandZoneCard` `CardClass`, a type-catalog row, and
per-class ability legality — `selenia` benches today
(`idris/src/Experimental/Cards.idr:1322-1328`) as a vanguard card whose whole
printed text is "Creatures you control have vigilance." But a vanguard card
prints two further characteristics that nothing in the workbench represents:

- [CR#313.6]: "Each vanguard card has a hand modifier printed in its lower
  left corner... applied to the starting hand size and maximum hand size of
  the vanguard card's owner."
- [CR#313.7]: "Each vanguard card has a life modifier printed in its lower
  right corner... applied as the starting life total of the vanguard card's
  owner."

`selenia`'s own two numbers are simply absent from her definition — she
benches for her ability text only.

## Related coverage

The workbench already covers the other command-zone/layout gaps in this
neighborhood: Room (routed to
`workbench-transform-verb-and-face-residues.md`), Meld (same ticket), and
Vanguard/Scheme/Plane/Conspiracy's ability legality
(`workbench-card-class-and-command-zone.md`, done). This ticket is the one
concrete remainder: the two printed numbers a vanguard card carries that its
ability text does not.

## The shape question

This is the same "printed box" pattern `workbench-multiface-cards.md`
(done) already solved for starting loyalty [CR#209.1] and defense [CR#210.1]
— `PrintedBox` in `idris/src/Experimental/Card.idr`. But a vanguard card is
not a `data Card` face-layout value in the six-constructor sense that
`PrintedBox` was built for; it is a `CommandZoneCard`-classed card built
however `workbench-card-class-and-command-zone.md` shaped that class. Decide
whether the hand/life modifiers extend `PrintedBox` (if a `CommandZoneCard`
already routes through the same face/box machinery) or are a field scoped to
the `CommandZoneCard` class directly — against what that class's actual
current shape is, not by assumption.

## Consumption boundary

`idris/src/Experimental/Card.idr` (wherever the `CommandZoneCard` class's
fields and laws live after `workbench-card-class-and-command-zone.md`),
`idris/src/Experimental/Words.idr` if a signed-modifier value type is needed,
evidence bench `idris/src/Experimental/Cards.idr` (`selenia`'s two numbers),
pin modules `idris/src/Experimental/Proofs*.idr` if a law needs one. No Rust
crate.

## Acceptance

- `selenia` (or another vanguard witness, named before building) benches with
  both its printed hand modifier and life modifier represented, not only its
  ability text.
- The shape decision (extend `PrintedBox` vs. a `CommandZoneCard`-scoped
  field) is recorded at the definition site, naming [CR#313.6] and
  [CR#313.7].
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
