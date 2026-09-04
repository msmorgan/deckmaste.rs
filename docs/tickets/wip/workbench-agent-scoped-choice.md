---
needs: []
---
**Let a chosen noun refer to its chooser.** Residue of
`workbench-party-and-outlaw` and `workbench-party-residues` (2026-09-04).
`Effect.Choose` takes both its chooser and its noun in `bs`, so beside a
distributive chooser (`Macros.each AnyPlayer`) the chosen noun has no
antecedent for "they": "each player chooses up to one creature they control"
[CR#700.8d] fails with `countReach (Word PlayerW) OneOf [] = 1`. Stick
Together is benched as a `ForEachOf` loop for that reason even though
`Predicate.NotChosen` now exists, and the same gap blocks four of the seven
vintage-legal "not chosen this way" cards (Raiding Party, Sculpted Sunburst,
Consuming Tide, Celestial Judgment).

Fix: type `Choose`'s noun at `agentIntro chooser` (the shape `Enact` already
uses for its deed), so a distributive chooser publishes its per-player
binding to the chosen noun; keep the `Choose (UpTo 1)`-per-role structure
and `EnactKeepsOuter` untouched. Re-spell Stick Together back to its printed
distributive shape with `NotChosen`, bench one of the four blocked cards, and
re-probe the choice pins.

Size: M. Done when: Stick Together reads its printed shape; the four cards
are spellable (one benched); every `Choose` bench site still typechecks;
build at its module count. Standard constraints apply, including the
RON-shaped constraint.
