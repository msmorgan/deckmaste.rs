---
needs: []
---
**A handoff that distributes an instruction over the members of a bound
group, each member's own performer acting.** Routed 2026-10-08 from
`plugins-v2-airbend-reads-its-own-stamp` (its STOP). Standard constraints
apply.

[CR#701.65a]: "For each card exiled this way, for as long as it remains
exiled, its owner may cast it by paying {2} rather than paying its mana cost."
Each exiled card's own owner gets a permission for that card.

## What the model has today

- `each` (`plugins_v2/builtin/macros/determiners/each.ron`) takes a
  Predicate, so it cannot range over a group a body already bound.
- `eachOf` (`nouns/eachOf.ron`, Lean `NounPhrase.eachOf`) refuses a group
  whose number is one (`refuse (g.plur == .many) .plural`), so a keyword
  body whose parameter may be one or several cannot use it.
- `ownerOf(<group>)` is one `PossessorOf` performer for the whole group:
  `act(ownerOf(Param(0)), …)` hands the body to "their owner" once.

## Candidate shapes (not chosen)

- A `forEach(members, act(ownerOf(it), …))`-style instruction: iterate the
  bound group, with a member pronoun inside. The name `forEach` is already an
  amount macro (`amounts/forEach.ron`), so it needs its own name.
- `each` (or `eachOf`) accepting a bound group of either number, so
  `act(ownerOf(each(<group>)), …)` distributes.
- Nearest existing device: the Lean group-handoff `.many` profile from the
  actor-handoff landing (`semantics-v2-actor-handoff`), where a distributive
  performer ("each opponent") has each member perform the whole body.

## Pins

- Two exiled cards with different owners: each owner gets the permission for
  their own card only.
- One exiled card: the permission is unchanged from today's.

## Until it lands

Airbend's body (`keyword_actions/airbend.ron`) is exact for one exiled
object, and for several with one owner, and approximate for several exiled
objects with different owners.
