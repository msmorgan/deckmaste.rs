---
needs: []
---
**Give the Attack deed its player agent.** Residue of
`workbench-deed-guard-residues` (2026-09-04): `Words.actFacts "Attack"`
keeps `agentRole` at `[Object] [Creature]` although the corpus survey
(`docs/memory/scratch/2026-09-04-deed-subjects.md`) finds 195 supported
cards printing "Whenever you attack" and [CR#508.1] makes the active player
the one who declares attackers. The role is load-bearing for
`featureAltOk Attacking` and `Phrase.attackableKind`, so widening it
touches the combat-relation gates. Fix: the agent role admits a player
subject as well as a creature; the creature-attacks reading keeps its
kind gate through the `Attacking` feature; bench one "whenever you attack"
card in full; re-probe the attack pins.

Size: S. Done when: "you attack" spells; every attack pin still refutes;
build at its module count. Standard constraints apply, including the
RON-shaped constraint.
