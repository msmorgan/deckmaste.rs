---
needs: []
design: true
---
**Sweep composite keyword actions baked as `Action` primitives out of the
mirrored taxonomies.** Set-mechanic verbs sitting as enum variants are
antithetical to the thesis: parsing recovers meaning into *general* primitives,
and mechanics compose from them. The taxonomy already owns the correct
pattern — `Composite{name, body}` carries keyword-action verbs, and builtin
macros decompose them (`Amass.ron`: conditional `Create` + `ChooseOne` +
`PutCounters` + subtype `Modify`, all general primitives).

Confirmed antithetical — both classified **composite** with
`core_referenced: false` in `keywords-classified.json`, yet baked as variants
in both grammars:

- `Action::TheRingTempts(Reference)` [CR#701.54a,701.54c] — blessed decomposition:
  choose a creature → Ring-bearer via the general designation/emblem
  primitives (`GetDesignation`, `GetEmblem`); temptation-tier machinery stays
  engine-side.
- `Action::VentureIntoDungeon(Reference)` [CR#701.49a] — macro over the
  engine's dungeon subsystem; venture-marker machinery stays engine-side
  behind a `Composite` verb arm.

Design-phase candidates, same smell, need a ruling before removal:

- `Action::BecomeDay` / `Action::BecomeNight` [CR#731.1] — day and night are
  designations the game itself has; decide whether a game-scope designation
  primitive generalizes them (the object/player scope already has
  `GetDesignation`) or they stay primitive.
- `Action::RollPlanarDie(Reference)` — Planechase-variant machinery sitting
  beside the general `RollDice`.

Scope per removed variant — the grammars are mirrors, so every change lands
in ALL mirror surfaces:

- Remove the variant from `deckmaste_authoring` and `deckmaste_core`
  `Action` enums and the Idris mirror.
- Re-express as a builtin macro over general primitives; where engine
  machinery is unavoidable, route through `Composite{name, body}` plus a
  `composite_items` per-verb arm (the established closed-dispatch pattern).
- Update every consumer: engine dispatch/step arms, spelling frames for the
  verb's oracle phrasing, legacy render match sites, round-trip tests,
  wizards regen. Inventory by `rg <VariantName>` from the repository root
  per variant — the sweep is the inventory.
- The design pass should also sweep the sibling taxonomy enums
  (`cargo xtask map enums`) for the same smell; `Action` is the confirmed
  locus, not necessarily the only one.

Sequencing: sweep-shaped over the same files as `semantics-crate-rename` —
never run the two concurrently.

Standard constraints apply.
