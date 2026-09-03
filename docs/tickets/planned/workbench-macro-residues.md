---
needs: [workbench-macro-discipline-2]
---
**Add the three macros the bench-fidelity sweep found missing.** Residues of
`workbench-bench-fidelity` (2026-09-03), each a raw-core site that no macro
can spell today:

- `Macros.returnTo` hard-codes an empty rider list, so Open the Vaults
  (`Cards.openTheVaults`, raw `Enact "Return"` with `[Under …]`) cannot read
  through it; give `returnTo` its rider list.
- No `Macros.libraryZ`, so `ZoneAt Library Bare` is spelled raw; add it beside
  `handZ`/`graveyardZ`.
- No set-life macro, so four `ChangeLife w (Set a)` sites are raw; add one
  macro per lemma ("your life total becomes N").

Size: S. Done when: the named sites read through macros, no macro duplicates
another, build 23/23. Standard constraints apply.
