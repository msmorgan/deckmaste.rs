---
needs: [workbench-union-family-macros]
---
# Make the attack defender a joined-kind noun, gated by [CR#506.3]

`workbench-union-family-macros` gave the semantics a joined kind, but the
attack-defender slot did not move with it: `Attacks`'s `whom` is still
`Maybe (Noun bs Player)`, so no card can name a defender that may be a player
or a planeswalker. **Tahngarth, First Mate** is the per-site residue that
ticket recorded and left unlanded. Its oracle line:

> Whenever an opponent attacks with one or more creatures, if Tahngarth is
> tapped, you may have that opponent gain control of Tahngarth until end of
> combat. If you do, choose a player or planeswalker that opponent is
> attacking. Tahngarth is attacking that player or planeswalker.

## Scope

- `Attacks`'s `whom` becomes a joined-kind noun — `Maybe (Noun (nomIntro n) kd)`
  — so "that player or planeswalker" is an ordinary `Macros.kindJoin` term.
- The same change for `DefendingPlayer`.
- The increment the join does **not** supply: an `Attackable` gate carrying
  [CR#506.3]'s set — "Only a player, a planeswalker, or a battle can be
  attacked." Kind `Object` covers creatures, and a creature is never attacked,
  so the joined kind alone would overgenerate against a rule that names the
  set. This is a rules refusal, not a corpus one, and its docstring cites
  [CR#506.3].
- Bench Tahngarth, First Mate as the positive witness. A pin for the refused
  half (attacking a creature) names [CR#506.3].

The attackable gate is why this is its own ticket: it is a second
construction, not a consequence of the join.

Standard constraints apply.
