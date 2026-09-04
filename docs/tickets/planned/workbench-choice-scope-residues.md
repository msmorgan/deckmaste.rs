---
needs: []
---
**Close the two choice-scope gaps agent-scoped-choice left.** Residue of
`workbench-agent-scoped-choice` (2026-09-04), which typed `Choose`'s noun at
its chooser so "each player chooses … they control" reads [CR#700.8d].

- **Distributive spend of a distributive choice.** `Macros.sacrifice (each
  AnyPlayer) (theRest Object)` is refused by `EnactKeepsOuter` because
  `TheRest` spends the shared group, yet when the spent parts are the ones
  the same distributive chooser introduced, each player spends only their
  own partition. Let that case stand (a per-agent rest, or a `KeepsOuterEach`
  case that recognises the chooser's own delta), keep the guard for a shared
  group, pin both sides. Unblocks Stick Together's printed shape (its last
  blocker) and Disciple of Caelus Nin.
- **"Not chosen this way" after several standing choices.** `NotChosen`'s
  `ChoiceInScope` is counted uniqueness over one choice; cards write several
  choices under one "this way" (Sculpted Sunburst, two choices; Raiding
  Party, "by any player"). Decide the read (all standing choices of the same
  chooser, or a choice-group binding) and bench one of the two. Celestial
  Judgment additionally needs `ForEachKindOf` to publish its in-loop choice
  outward (`instrIntro (ForEachKindOf …) = bs` today); record whether that
  is a loop-delta gap or a separate shape.

Size: M. Done when: Stick Together is benched in its printed shape; one of
Sculpted Sunburst or Raiding Party is benched; pins probed non-vacuous; build
at its module count. Standard constraints apply, including the RON-shaped
constraint.
