---
needs: [lean-v3-authoring-impl-7-checker-and-regime-retirement]
---
# Split `Definitions.lean` into leaf/composite submodules and unify noun-head and relative-pronoun representations

**Break `lean/Semantics/Definitions.lean` into leaf catalog modules imported by
`Templates.lean` and composite definition modules importing `Templates.lean`,
collapse `NounPhrase.asType` and `NounPhrase.asMarker` (retiring `MarkerWord`
and the unread `Option Subtype` field), and seed `Predicate.did` (`which`) from
its enclosing predicate so `(asMarker .permanent (relative .object))` disappears
from `Cards/`.** Standard constraints apply.

## Scope

### 1. Split `Definitions.lean` into Leaf Catalog and Composite Submodules

- Split the 3,129-line `lean/Semantics/Definitions.lean` into focused submodules
  under `lean/Semantics/Definitions/` (all within `namespace Semantics.Definitions`):
  - **Layer 1 — Leaf catalog definitions (import `Semantics.Primitives`):**
    - `lean/Semantics/Definitions/Counters.lean` (`p1p1Counter`, `m1m1Counter`,
      `loyaltyCounter`, etc., `[CR#122.1]`)
    - `lean/Semantics/Definitions/Designations.lean` (`monarch`, `initiative`,
      `citysBlessing`, `ringBearer`, `goaded`, `suspected`, `saddled`,
      `renowned`, `monstrous`, `solved`, `day`, `night`, `leftHalfUnlocked`,
      `rightHalfUnlocked`, `commander`, etc., `[CR#701.15b,725.1,731.1]`)
    - `lean/Semantics/Definitions/Relations.lean` (`encodedOn`, `haunting`,
      `paired`, `[CR#702.55b,702.95b,702.99b]`)
    - `lean/Semantics/Definitions/Subtypes.lean` (`aura`, `equipment`, `room`,
      `saga`, `vehicle`, `spacecraft`, `fortification`, `case`, `class_`,
      `siege`, `army`, creature/land/planeswalker/spell subtypes, `[CR#205.3]`)
  - **Layer 2 — `lean/Semantics/Templates.lean`:**
    - Import `Semantics.Definitions.Counters`, `Semantics.Definitions.Designations`,
      `Semantics.Definitions.Relations`, and `Semantics.Definitions.Subtypes`
      and `open Semantics.Definitions`.
    - Replace all raw `.named "..."` and `enchantmentType "..."` / `artifactType "..."`
      / `creatureType "..."` strings inside `Templates.lean` (`isHistoric`,
      `yourCommander`, `becomeTheMonarch`, `hasCitysBlessing`, `becomesDay`,
      `becomesNight`, `isDay`, `isNight`, `amassArmy`, `plusOneCountersOn`,
      `minusOneCountersOn`, `loyaltyCountersOn`, `PutLoyaltyCounters`,
      `RemoveLoyaltyCounters`, `thisAura`..`thisClass`, etc.) with the typed
      catalog definitions from Layer 1.
    - Move pure phrasing templates from `/-! ## Templates over Definitions -/` in
      `Definitions.lean` that only need Layer 1 leaf definitions into
      `Templates.lean`.
  - **Layer 3 — Composite definitions (import `Semantics.Templates`):**
    - `lean/Semantics/Definitions/Actions.lean` (turn parts, attach/create/counter,
      keyword actions `[CR#701.1]`, tokens, and templates over composite actions)
    - `lean/Semantics/Definitions/Keywords.lean` (abilities, ability words,
      keyword abilities `[CR#702.1]`)
    - `lean/Semantics/Definitions.lean` imports and re-exports all six submodules
      so downstream `import Semantics.Definitions` and `open Semantics.Definitions`
      continue to work unchanged.

### 2. Unify `NounPhrase.asType` and `NounPhrase.asMarker` and Retire `MarkerWord`

- In `lean/Semantics/Words.lean`, `lean/Semantics/Abilities.lean`, and
  `lean/Semantics/Check/Phrase.lean`:
  - `NounPhrase.asType`'s third field `(subtype : Option Subtype)` is completely
    ignored by `Check/Phrase.lean` (`| .asType t subj _ =>`).
  - `MarkerWord` (`card | token | emblem | spell | permanent | ability`,
    `Words.lean:477-479`) duplicates `NounWord`.
  - Collapse `NounPhrase.asType` and `NounPhrase.asMarker` into a single
    head-noun refinement constructor (or `this` head refinement over `NounWord` /
    `MarkerWord` + `CardType`), retiring the separate `MarkerWord` enum and the
    dead `Option Subtype` parameter on `.asType` while keeping all surface
    self-reference templates (`thisCreature`, `thisArtifact`, `thisLand`,
    `thisPermanent`, `thisSpell`, `thisCard`, `thisAbility`, `thisToken`,
    `thisEmblem`, `thisAura`, `thisEquipment`, `thisVehicle`, `thisSaga`,
    `thisRoom`, `thisCase`, `thisClass`, `thisSiege`, `thisSpacecraft`,
    `thisFortification`) intact.

### 3. Seed `Predicate.did` (`which`) from the Enclosing Predicate and Clean Up `relative` / `gap`

- In `lean/Semantics/Check/Phrase.lean` (`lines 1561-1569` and `1682-1695`):
  - Currently `Predicate.and` only passes `relativeKind : Option Kind` (`some .object`)
    into `Predicate.did`, so `.which` is bound to a bare `kindBinding k` with
    `types = []` and `zone = none` even inside `and [isCreature, happenedTo ...]`,
    which forced cards to wrap `relative .object` in `asMarker .permanent`.
  - Seed `.which` in `Predicate.did` with the enclosing predicate's `bindFor`
    payload (inheriting `seedTy` and permanent battlefield zone evidence from
    sibling conjuncts in `Predicate.and`).
- In `lean/Semantics/Templates.lean`, `lean/Semantics/Definitions/`,
  `lean/Semantics/Cards/*.lean`, and `lean/Semantics/Proofs/*.lean`:
  - Replace `relative (_kind : Kind) : NounPhrase := .which` and
    `gap (kind : Kind) : NounPhrase := .which` (both of which ignored their
    `Kind` parameter) with a single parameterless `semantic_template which : NounPhrase := .which`.
  - Remove all `(asMarker .permanent (relative .object))` and `relative <kind>`
    wrappers across `Cards/*.lean` (`Cards/Cost.lean`, `Cards/Counters.lean`,
    `Cards/Damage.lean`, `Cards/Faces.lean`, `Cards/Trigger.lean`,
    `Cards/Turn.lean`, `Cards/Anaphora.lean`, `Cards/Description.lean`,
    `Cards/Static.lean`) and `Proofs/*.lean`.

## Verification

- `rg 'enchantmentType "|artifactType "|creatureType "' lean/Semantics/Templates.lean`
  returns 0 matches (outside the `enchantmentType` / `artifactType` / `creatureType`
  template declarations themselves).
- `rg 'asMarker \.permanent \(relative' lean/Semantics/Cards/` returns 0 matches.
- `./lean/scripts/build` (`lake build --wfail`) and `cargo xtask lean check` pass
  with zero warnings and zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` empty and `cargo xtask cite check`
  0 stale.
