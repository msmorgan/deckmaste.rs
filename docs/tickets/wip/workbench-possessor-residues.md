---
needs: []
---
**Give "each of your postcombat main phases" and "the chosen player's … that player" their own spellings.** Two residues of `workbench-possessor-noun` (2026-09-03).

- `Owner.EachYours` had no replacement: "each of your postcombat main phases"
  (Sphinx of the Second Sun, Brazen Cannonade) collapsed to `Macros.yours`,
  because the `each` quantifies the turn PART, not the possessor. The header
  needs a part quantifier (a `TurnPart` under `EachDet`-style distribution),
  not a possessor value.
- A choice binding and a definite description over it are two bindings for
  one referent: after "the chosen player's upkeep", Black Vise's "that
  player" is refused (`countReach (Word PlayerW) OneOf = 2`, since `choiceB
  PlayerC` and the possessor's own binding both reach `PlayerW`); the bench
  repeats `Macros.the ChosenPlayer`. This is `Described TheDet` over a choice
  generally: the definite description should re-read the choice binding, not
  mint a second one.

Size: S each. Done when: both cards read as printed (Sphinx of the Second
Sun's second main phase; Black Vise's "that player"), each with a witness,
and a pin refuses the double binding. Standard constraints apply.
