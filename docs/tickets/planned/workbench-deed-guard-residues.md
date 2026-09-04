---
needs: []
---
**Close the four small residues of dedup-tables and gate-fixes.** Residue of
`workbench-dedup-tables` and `workbench-gate-fixes` (2026-09-04):

- `Effect.instrProfile (ControllerSacrifices …)` still names the lexeme
  `"Sacrifice"` in its stamp; read the deed's declared feature instead (the
  `DeedFeature` column dedup added).
- `Triggers.eventName (UnlocksDoor …)` constructs a deed label by string;
  route it through the deed table.
- `Macros.fateseal`'s library-possessor slot is unconstrained (`fateseal You
  You` is admitted); gate it so the possessor is an opponent of the agent
  [CR#701.29a], pin the self case.
- `Words.actFacts "Fateseal"` keeps `actStepwise = False` although its
  expansion is scry-shaped; set the column from the expansion and check the
  other look actions agree.

Size: S. Done when: no deed lexeme literal remains in a core gate or stamp
(`grep '"Sacrifice"\|"Search"\|"Attack"\|"Block"\|"Trigger"\|"Activate"'`
over `Effect.idr`/`Triggers.idr`/`Words.idr` finds only the deed table);
the fateseal pin refutes; build at its module count. Standard constraints
apply, including the RON-shaped constraint.
