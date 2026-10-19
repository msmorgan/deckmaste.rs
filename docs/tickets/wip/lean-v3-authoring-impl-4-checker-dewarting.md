---
needs: [lean-v3-authoring-impl-3-cards-reauthoring]
---
# Retire non-anaphora game-rule checks in the Lean v3 checker

**Retire the remaining game-rule, role-fit, possessor, counter-holder, stat-type,
predicate-contradiction, and zone-presupposition checks in `lean/Semantics/Check/`
so the checker enforces only Discourse Representation Theory (DRT) anaphora
resolution and basic sort/kind agreement.** Standard constraints apply.

## Why

Under `[CR#101.3,609.3]`, impossible actions and vacuously true/false restrictions
are ignored at runtime by the game engine. In `lean/Semantics/`, a clause that says
*"players can't explore"* or *"put a +1/+1 counter on target spell"* must either be:
- **rejected as a side effect of how its `semantic_definition` expands** (for
  example, `explore` expands to `controllerOf .actor` and `that .permanent`, which
  require an `.object` performer through ordinary reference resolution), or
- **silently accepted** if no pronoun/definite description fails to resolve and
  sort/kind agreement holds.

Even after the earlier `lean-checker-sweep-*` passes, `lean/Semantics/Check/` still
carries side-table game-rule checks that do not participate in `reaches` or
`resolveCandidates` (such as `ActFacts` role tables, which caused the `retrace`
false refusal in `impl-3`).

## Scope

### 1. Retire `ActFacts` role-table checks (`Refusal.deedFits` and `Refusal.deedNounOk`)

- In `lean/Semantics/Check/DeonticPatients.lean`, `lean/Semantics/Check/Events.lean`,
  `lean/Semantics/Check/Phrase.lean`, and `lean/Semantics/Check/Words.lean`:
  - Delete `Refusal.deedFits`, `Refusal.deedNounOk`, `deonticDeedFits`,
    `deonticReferentFits`, `deedFits`, `deedNounOk`, `deedAgentDomain`, and the
    `patientErrors` / `actorErrors` checks on `Action.enact` and
    `CombatUpdate.blocking`.
  - Remove `agentRole`, `patientRole`, and `hostRole` (and `RoleFit` if unused)
    from `ActFacts` (`coreDeedFacts`, `abilityDeedFacts`, `actFacts` in
    `Check/Words.lean`), keeping only the fields read by reference resolution and
    salience (`participle`, `dest`, `loci`, and `defaultsToSource` on `enact`).
- In `lean/Semantics/Definitions.lean`:
  - Rewrite `adapt` and `monstrosity` to read `.actor` (which defaults to
    `thisPermanent` when unhanded) instead of hardcoding `thisPermanent`, so
    object-performer requirements on *"players can't adapt"* or *"target player
    monstrosities 2"* arise directly from expanding their `semantic_definition`
    bodies (matching `explore`, `connive`, `endure`, and `fight`).

### 2. Retire `Check/Possessor.lean` (`Refusal.patientPossessor`)

- Delete `lean/Semantics/Check/Possessor.lean` and remove `Check.Possessor` from
  `lean/Semantics.lean` and `Check/Phrase.lean`.
- Delete `Refusal.patientPossessor`, the 6th `possessor : Option Possessor` field
  on `Payload.object`, `ActorShape.opponentOnly`, and `ActFacts.ownPatient` (none
  of which are read by `reaches` or `resolveCandidates`).

### 3. Retire counter-holder & counter-memory game-rule checks

- In `lean/Semantics/Check/LocalShape.lean` and `lean/Semantics/Check/Phrase.lean`:
  - Delete `Refusal.counterMemory` and remove `counterMemoryAvailable : Bool` from
    `Referent` (and its updates across moves and counter placement).
  - Delete `Refusal.counterSourceScope` and `Refusal.counterKindNamed` (and
    `CounterKind.holderScope` / `holderTy`), keeping only `Refusal.counterHolderKind`
    (a counter holder has sort `.object` or `.player`).

### 4. Retire stat card-type presuppositions (`Refusal.statHeadTysOk`)

- In `lean/Semantics/Check/Words.lean` and `lean/Semantics/Check/Phrase.lean`:
  - Delete `Refusal.statHeadTysOk` and `statHeadTysOk` (`[CR#208.3]` card-type
    check on `powerOf` / `toughnessOf` / `loyaltyOf` / `defenseOf`), keeping
    `kindCheck axis.scope actual` (`.object` vs `.player` sort check via
    `Refusal.projScope`).

### 5. Retire predicate-conjunction contradiction checks (`Refusal.contradictionFree`)

- In `lean/Semantics/Check/PredicateShape.lean` and `lean/Semantics/Check/Phrase.lean`:
  - Delete `Refusal.contradictionFree` and `Predicate.contradictionFree` (and its
    helper tables checking `tapped` + `untapped`, `colorless` + colored, `isCard` +
    `isToken`, `isPermanent` + `isInstant`, combat status + `not isCreature`, etc.).
  - If `Check/PredicateShape.lean` becomes empty after removing `contradictionFree`,
    delete the module and its import.

### 6. Retire remaining zone & local-shape game-rule refusals

- **Zone checks in `handoffPerformer` and `designationHolder`:**
  - In `Check/Phrase.lean`, remove the `.battlefield` zone check from
    `handoffPerformer` (keeping only `shape.kind == .player || shape.kind == .object`)
    and remove `zoneFits (source.bind (·.zone)) d.zone` from `designationHolderOk`
    (keeping only the holder `Kind` check).
- **Other local-shape game-rule checks:**
  - Delete `Refusal.settableValue` (`[CR#701.12g]` check on `Exchanged.values` in
    `Check/LocalShape.lean`).
  - Delete `Refusal.permanentSpellType` (card-type check on `.resolvedPermanent` in
    `Check/Phrase.lean`).
  - Delete `Refusal.ascriptionOk` (card-type / subtype table check on `.asType` in
    `Check/Phrase.lean`).
  - Delete `Refusal.riderKind` (`RiderKind` entry-rider vs. pile-placement
    destination check in `Check/Phrase.lean`).

### 7. Update `lean/Semantics/Proofs/` and `lean/CONTRACTS.md`

- Retire negative refusal theorems in `lean/Semantics/Proofs/*.lean` that pinned the
  deleted game-rule refusals (`Proofs/Possessor.lean`, `.deedFits`, `.deedNounOk`,
  `.counterMemory`, `.counterSourceScope`, `.counterKindNamed`, `.statHeadTysOk`,
  `.contradictionFree`, `.settableValue`, `.permanentSpellType`, `.ascriptionOk`,
  `.riderKind`, and the zone-based `.handoffPerformer` / `.designationHolder` pins),
  converting or adding pins where an object-performed keyword definition (`explore`,
  `adapt`, `monstrosity`, `fight`) is refused on a player performer via its
  expanded definition body.
- Update `lean/CONTRACTS.md` so the checker specification and `Refusal` inventory
  match the live code.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) and `cargo xtask lean check` pass
  with zero warnings and zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` empty and `cargo xtask cite check`
  0 stale.
