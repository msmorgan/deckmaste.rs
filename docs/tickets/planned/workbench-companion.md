---
needs: []
---
**Add a small `DeckCondition` sort so Companion's deck restriction has a
spelling [CR#702.139a].** Fresh workbench review 2026-09-03, R3, resolved by
ruling.

**Ruling (settled 2026-09-03): Companion is a deck condition, a sort of its
own.** The condition is a property of the starting deck, evaluated outside the
game, so it is not a `Condition bs` over game state and not a `Predicate` over
an object in a zone. It is a small sort — quantified restrictions over the
cards in the starting deck ("every card has X", "no card has X", a bound on a
characteristic) — read by the Companion keyword row and nothing else.

10 supported cards print Companion.

The sort stays small on purpose: it admits the shapes the ten printed
conditions need and no general query language. A condition the CR does not
make meaningful for a starting deck is refused, and the pin names the reason.

The Companion `keywordFacts` row gains the condition as its parameter; the
shape list from `workbench-facts-from-ron` is the natural home for it if that
ticket has landed, and a one-off row otherwise — this ticket does not depend
on it.

Size: M.

Done when: all ten printed companions are typechecking bench witnesses, or the
landing record names each one left out and why; the `DeckCondition` sort
admits nothing beyond the shapes those cards need; a pin refuses a deck
condition over game state (a zone read, a battlefield object) as
CR-meaningless for a starting deck, probed non-vacuous; the build is 44/44
with 0 errors and 0 warnings. Standard constraints apply, plus the RON-shaped
constraint: a core constructor is admissible only if the RON re-emitter can
produce it from a RON node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
