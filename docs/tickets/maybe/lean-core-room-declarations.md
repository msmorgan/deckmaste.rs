---
needs: []
---
# Complete the Room door designation fold (`[CR#709.5c..709.5j]`)

Rooms (`[CR#709.5]`) are currently split across two half-connected representations
in `lean/Semantics/`:

1. [`Definitions/Designations.lean:24-26`](../../../lean/Semantics/Definitions/Designations.lean)
   and `Check/Facts.lean:499-500` declare `leftHalfUnlocked` and
   `rightHalfUnlocked` as `DesignationLabel`s (`[CR#709.5c]`), with
   `Proofs/Rules.lean:266-272` checking their `Definition.designation` rows.
2. However, [`Semantics/Abilities.lean:402-405, 1059-1061`](../../../lean/Semantics/Abilities.lean)
   still carries bespoke Room primitives that do not use those designations:
   - `LockState` (`locked | unlocked` in `Semantics/Words.lean`)
   - `Door` (`thisDoor | doorOf (state : Option LockState) (room : NounPhrase)`)
   - `Action.unlock (door : Door)`
   - `Action.fullyUnlock (room : NounPhrase)` (`[CR#709.5i]`)

## Design

- Express Room door state and transitions (`[CR#709.5e..709.5j]`) through the
  existing designation primitives (`gainDesignation`, `loseDesignation`,
  `hasDesignation` with `leftHalfUnlocked` / `rightHalfUnlocked`) or a typed
  half selector over them so `Door`, `LockState`, `Action.unlock`, and
  `Action.fullyUnlock` do not sit alongside unused `leftHalfUnlocked` /
  `rightHalfUnlocked` designation definitions.
- Preserve all Room witnesses in [`Semantics/Cards/Faces.lean:430-475`](../../../lean/Semantics/Cards/Faces.lean)
  (`"When you unlock this door"`, `"Whenever you fully unlock a Room"`, and
  `"unlock a locked door of target Room you control"`).
