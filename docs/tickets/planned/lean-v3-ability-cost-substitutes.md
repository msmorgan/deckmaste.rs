---
needs: [lean-v3-alternative-cost-unification]
---
# Paying something "rather than" an ability's cost is not a spell's alternative cost

Standard constraints apply. Found in passing by `lean-v3-alternative-cost-unification` (B3),
whose ruling covers only spells' alternative costs [CR#118.9].

Two card fragments in `lean/Semantics/Cards/Keyword.lean` write `StaticSpec.altCost`, a spell's
alternative cost, over an ability's cost:

- *New Perspectives* (`newPerspectivesCyclingAltCost`): "As long as you have seven or more cards
  in hand, you may pay {0} rather than pay cycling costs." The encoding drops the condition. The
  cost replaced is cycling's activation cost [CR#702.29a].
- *Thick-Skinned Goblin* (`thickSkinnedGoblinEchoAltCost`): "You may pay {0} rather than pay the
  echo cost for permanents you control." Echo is a triggered ability whose cost is paid while it
  resolves, in "sacrifice it unless you pay [cost]" [CR#702.30a]; it is neither a spell's mana
  cost nor an activation cost.

Change: decide the Core form for paying something instead of an ability's cost (an activation cost
and a cost paid during resolution may want different forms), encode both cards with it, add New
Perspectives' condition, and make `StaticSpec.altCost` (documented as a spell's alternative cost)
refuse a subject that names abilities, or say why it should not.
