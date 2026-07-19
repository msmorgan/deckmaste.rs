---
needs: []
---
**Seed the deontic "as though" permission-override, driven by Glaring
Spotlight.** The corpus has no way to express an effect that lets a player act
"as though" a restriction/characteristic were absent — a distinct deontic from
`May`/`Cant` (it does not grant an action; it removes an obstacle to an
otherwise-legal action).

## The seed card

Glaring Spotlight ({1} Artifact): *"Creatures your opponents control with
hexproof can be the targets of spells and abilities you control **as though they
didn't have hexproof**. {3}, Sacrifice this artifact: Creatures you control gain
hexproof until end of turn and can't be blocked this turn."*

The load-bearing clause is the AsThough targeting permission: it does not remove
hexproof (a `Cant`/lose-ability layer effect would), it makes YOUR spells/
abilities able to target those creatures *as though the hexproof weren't there*
([CR#702.11d] is the obstacle being seen-through). Other members of the family:
"attacks/blocks as though it didn't have defender", "you may play lands as
though they had flash", "as though it weren't tapped".

## Shape

A continuous effect that, for a scoped set of actions + a scoped set of objects,
suppresses ONE named obstacle when checking that action's legality — leaving the
obstacle intact for everyone/everything else. Model it in the deontic layer
alongside `May`/`Cant` (an `AsThough { action, obstacle, scope }` permission
that the legality check consults), mirroring the Idris deontic vocabulary; do
NOT model it as a characteristic/layer change (it is a per-checker override, not
a real loss of the ability).

## Done

- An AsThough deontic exists in core + is honored by the relevant legality check
  (targeting, for the seed).
- Glaring Spotlight encoded as a canon card: opponents' hexproof creatures are
  legal targets for the controller's spells/abilities, still hexproof to
  everyone else; the sac ability's hexproof grant + can't-be-blocked also encode.
- A semantic engine test: a hexproof creature is an illegal target normally, a
  legal target for the AsThough controller's spell, and still illegal for a
  third player.
