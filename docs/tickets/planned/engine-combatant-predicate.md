---
needs: []
---
**Read the Combatant role in predicates.** Residue of `engine-combatant-role`
(2026-09-04): the role is conferred and drives the four derived rules, but
the fight both-or-neither guard [CR#701.14b] and the creature state-based
actions [CR#704.5f,704.5g] still test `Type(Creature)`, because a role atom
in the PREDICATE vocabulary can only be authored in the v1 grammar
(`deckmaste_semantics`, deletion-bound). At cutover, when predicates are
authored from the v2 grammar, add `HasRole(Combatant)` to the predicate
sort, point those three sites at it, and delete the `Type(Creature)` proxies.

Size: S at cutover. Done when: no engine rule that the CR states over
creatures as combatants tests the card type instead of the role; workspace
tests green. Standard constraints apply.
