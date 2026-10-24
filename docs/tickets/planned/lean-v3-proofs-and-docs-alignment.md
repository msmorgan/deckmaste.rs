---
needs: [lean-v3-keyword-bodies-and-catalog-sync]
---
# Align Lean v3 proof suites and documentation with production definitions

**Point Lean proof suites at production `Definitions` rather than drifted
hand-copied replicas, add the `[CR#704.5g]` lethal-damage row to
`Proofs/Rules.lean`, strengthen negative/refusal pins, and update
`lean/CONTRACTS.md`, `lean/README.md`, and `docs/decisions/semantics-v3.md`
([`docs/evidence/lean-v3-astra-findings.md`](../../evidence/lean-v3-astra-findings.md)
§§5–6; [`docs/evidence/lean-v3-opus-findings.md`](../../evidence/lean-v3-opus-findings.md)
§§6.1–6.8, §§7.1–7.6).** Standard constraints apply.

## Scope

### 1. Tie Proof Suites Directly to Production `Definitions` (Astra §5, Opus §§6.1–6.2, 6.6)

- Replace the hand-copied/drifted `*Definition` and `*Def` replicas in
  `lean/Semantics/Proofs/KeywordAbilityCensus.lean:17-92` (`saddleDefinition` :19,
  `teamworkDefinition` :43, `awakenCost`/`awakenDefinition` :56-73, `indestructibleDefinition` :84),
  `lean/Semantics/Proofs/BindingIdentity.lean:180-235` (`scryBy` :180, the inline surveil and
  fateseal bodies in `okSurveilTwo` :200 / `okFatesealTwo` :206, `conniveDiscards`/`conniveBy`
  :218-227),
  `lean/Semantics/Proofs/GiftVariants.lean:22-50` (removing the local `gift`
  shadow and its local `giftACard`/`giftATappedFish`), `lean/Semantics/Proofs/KeywordByCardClass.lean:20-40`, and
  `lean/Semantics/Check.lean:44-104` (`exploreDef` :44, `conniveDef` :61, `airbendDef` :85)
  with direct references to the production declarations in
  `lean/Semantics/Definitions/` (rehoming `Check.lean`'s `CorePins`, :20-287, into
  `lean/Semantics/Proofs/`).
  **Reconciled 2026-10-24: live** (partly: `KeywordByCardClass.lean` is done, no single landing
  ticket: its pins now use production `gift`/`haunt`/`ascend` directly, `Definitions/Keywords.lean:806`,
  and its local `halves`/`card` :21,28 are helpers, not replicas; `KeywordAbilityCensus`'s haste pin
  :103 already reads production `haste`, `Definitions/Keywords.lean:867`. Live: the awaken replica
  vs production `awaken` `Definitions/Keywords.lean:266`; scry/surveil/fateseal/connive replicas vs
  `Definitions/Actions.lean:244,257,251,439`; explore/connive/airbend replicas vs
  `Definitions/Actions.lean:424,439,453`; the `GiftVariants` shadow vs `Definitions/Keywords.lean:806-822`.
  Saddle, teamwork and indestructible have no production keyword declaration in
  `Definitions/Keywords.lean` at all, so their replicas have nothing to point at yet.)
- Reconcile `gift`'s class-guard helpers (`isPermanentType` `[CR#702.174b]` vs
  `isPermanentCard`) across `CONTRACTS.md`, `KeywordByCardClass.lean`, and
  `LinkedChoiceReadback.lean` (§6.6).
  **Reconciled 2026-10-24: superseded** by [lean-v3-by-card-class](../done/lean-v3-by-card-class.md):
  gift has no class-guard helper any more; its halves are `byCardClass` (`Definitions/Keywords.lean:806-818`,
  `lean/CONTRACTS.md:989-1040`), and `isPermanentType`/`isPermanentCard`
  (`Definitions/Phrasings.lean:45,51`) appear in neither `KeywordByCardClass.lean` nor
  `LinkedChoiceReadback.lean`.

### 2. Complete the `[CR#704.5g]` Rules Row & Tighten Proof Quality (Astra §6, Opus §§6.3–6.5, 6.8, 7.1)

**Narrowed (2026-10-23):** the `[CR#704.5g]` lethal-damage row landed in
`lean-v3-card-type-conferral-rows` as `lethalDamage` on the creature `Definition.cardType` row
(`Definitions/CardTypeRules.lean`), not in `builtinTables`; the first bullet's lethal-damage half is done.

- **Add the `[CR#704.5g]` lethal-damage row to `builtinTables` and complete
  truncated SBA / spell-copy guards (`lean/Semantics/Proofs/Rules.lean:240-251,395-401`,
  Astra §6, Opus §7.1):**
  **Reconciled 2026-10-24: live** (partly: the lethal-damage half and the Saga/Siege guards are
  done, see the sub-items; the storm/replicate targets guard moved to `lean-v3-rules-and-card-fidelity` §2).
  - Now that `Stat.defense`, `ProjAxis.markedDamage`, and
    `GameEvent.stateBasedActionsChecked` exist, encode the `[CR#704.5g]`
    state-based action row and update the docstring and `lean/CONTRACTS.md:1036-1041`.
    **Reconciled 2026-10-24: done** by [lean-v3-card-type-conferral-rows](../done/lean-v3-card-type-conferral-rows.md):
    `lethalDamage` at `Definitions/CardTypeRules.lean:49-53` on the creature row (:85), pinned at
    `Proofs/Rules.lean:252-255`; `lean/CONTRACTS.md:1362-1363` documents it.
  - The Saga and Siege "triggered but not yet left the stack" guards
    (`[CR#704.5s,714.4]`, `[CR#310.7,704.5v]`) moved to
    [lean-v3-saga-rules-fidelity](lean-v3-saga-rules-fidelity.md) §1: `in_ stack` alone misses
    abilities that have triggered and are waiting to be put on the stack. Add the `"If the spell has any targets"`
    guard (`targeting any .someTarget`) to `storm` (`[CR#702.40a]`) and
    `replicate` (`[CR#702.56a]`).
    **Reconciled 2026-10-24: live** (partly: the Saga/Siege half is done by
    [lean-v3-saga-rules-fidelity](../done/lean-v3-saga-rules-fidelity.md), Siege row at
    `Definitions/SubtypeRules.lean:35-47`). The storm/replicate targets guard moved
    2026-10-24 to [lean-v3-rules-and-card-fidelity](lean-v3-rules-and-card-fidelity.md) §2
    ("Copy keywords drop 'If the spell has any targets'"), merged with the same defect on
    gravestorm, casualty, conspire and epic; it is owned there, not here.
- **Retire vacuous `rfl` restatements (§6.3):** Audit and retire or rewrite the
  ~20 `rfl` expansion-restatement theorems (`Proofs/ActionForms.lean`,
  `controllerSacrificesExpansionForEverySubject`, `lookAndSortSlice_is_the_hand_expansion`,
  `getsPtIsOneCharacteristicChange`, `combatantIsTheRole`, `drawGameConcludes`,
  `grantsIsTheKeywordConferral`, `boostIsOneStatChange`,
  `targetsOnlyIsTargetsAtOnly`, `printedDataNeedsNoExpressionTemplate`) so each kept
  theorem proves a semantic or checker property rather than restating its own definition.
  `destroyCantRegenerateIsDestroyThenProhibition` is not retired here: it pins a wrong order, and
  [lean-v3-resolution-riders-and-guards](../done/lean-v3-resolution-riders-and-guards.md) §1 rewrites it
  to pin the corrected rider.
  **Reconciled 2026-10-24: live.** All named restatements still stand: `Proofs/ActionForms.lean:7-41`
  (six `by rfl` pins), `controllerSacrificesExpansionForEverySubject` `Proofs/ControllerSacrifice.lean:132`,
  `lookAndSortSlice_is_the_hand_expansion` `Proofs/BindingIdentity.lean:170`,
  `getsPtIsOneCharacteristicChange` `Proofs/GetsBothDeltas.lean:21`, `combatantIsTheRole`
  `Proofs/Rules.lean:437`, `drawGameConcludes` `Proofs/Trigger.lean:230`,
  `grantsIsTheKeywordConferral`/`boostIsOneStatChange` `Proofs/Rules.lean:128,131`,
  `targetsOnlyIsTargetsAtOnly` `Proofs/SpellCopies.lean:33`, `printedDataNeedsNoExpressionTemplate`
  `Proofs/Authoring.lean:26` (now closed by `plain.onlyAuthored`, not `rfl`, but still a
  restatement). `destroyCantRegenerateIsDestroyThenProhibition` no longer exists (the carve-out's
  ticket is done).
- **Decouple brittle DRS id-arithmetic pins (§6.4):** Rewrite `nextId` / literal
  `ReferentId` arithmetic assertions (especially in `Proofs/ControllerSacrifice.lean`)
  to assert structural referent relationships (distinctness, resolution target,
  role, stamp) rather than raw integer offsets.
  **Reconciled 2026-10-24: live.** Raw offsets remain, e.g. `Proofs/ControllerSacrifice.lean:309,374,490,640,969-981`
  (`drs.nextId + 2`, `fresh + 5`, `fresh + 2`, `drs.nextId + 3`), and `nextId - 1`/`- 2` selections in
  `Proofs/Actor.lean:830`, `Proofs/Anaphora.lean:1056-1069`, `Proofs/CorpusRecency.lean:177,225`,
  `Proofs/EventPatterns.lean:238,269`.
- **Pin all live `Refusal` constructors & tighten weak negatives (§6.5):**
  - Replace `.isEmpty = false` / `≠ []` in `badDestroyedPlayer`,
    `badSpeedOfAnObject`, and `amountTemplatesRejectMissingSubjects` with the
    exact expected `[Refusal]` list, and retire `p1p1CounterCountMultipliedTwin`.
    **Reconciled 2026-10-24: live.** `badDestroyedPlayer` `Proofs/KeywordAbilityCensus.lean:96-97`
    and `badSpeedOfAnObject` `Proofs/AbilityQualifiers.lean:66-67` still assert `.isEmpty = false`;
    `amountTemplatesRejectMissingSubjects` `Proofs/CallerScope.lean:160` still asserts `≠ []`;
    `p1p1CounterCountMultipliedTwin` is at `Proofs/Rules.lean:164` (and is cited from
    `lean/CONTRACTS.md:1334`).
  - Add positive/negative pins covering the 12 unpinned `Refusal` constructors
    (`handoffPerformer`, `facesFit`, `counterHolderKind`, `qualityRead`,
    `concreteCost`, `concreteBallot`, `phrasal`, `countedMention`, `choosable`,
    `groupSizeInScope`, `bindingless`, `flatDisjuncts`).
    **Reconciled 2026-10-24: live** (partly: `concreteCost` and `groupSizeInScope` are now pinned,
    `Proofs/SelectionEvidence.lean:200,212,256`, no single landing ticket). The other ten
    (`Check/Refusal.lean:147,127,165,166,20,30,50,55,98,107`) have no `.name` pin in `Proofs/`,
    `Cards/` or `Check.lean`.
- **Deduplicate proof scaffolding & normalize proof module names (§6.8):** Share
  `olderClauseMaximum`/`olderClauseFilter` and `actionResult`/`nounResult`/`nounZone`
  helpers, remove local shadows (`soFarThisTurn`/`thisGame` in `Lookback.lean`,
  `flyingCounter` in `Proofs/Rules.lean`), and rename ticket-slug proof modules
  (`KeywordAbilityCensus`, `KeywordActionResidues`, `GetsBothDeltas`,
  `CorpusRecency`) to domain names in alphabetical import order in `Proofs.lean`.
  **Reconciled 2026-10-24: live.** `olderClauseMaximum`/`olderClauseFilter` are duplicated at
  `Proofs/ComparisonBox.lean:35,48` and `Proofs/ReferenceScopes.lean:24,37`; `actionResult`/`nounResult`/`nounZone`
  helpers are redefined in 15 proof files (23 definitions); `Proofs/Lookback.lean:9,11` still shadows
  `Templates.soFarThisTurn`/`thisGame` (`Templates.lean:838,850`); `Proofs/Rules.lean:122` defines a
  `flyingCounter : Definition` beside `Definitions/Counters.lean:20`'s `flyingCounter : CounterKind`;
  all four slug modules still exist, and `Proofs.lean:50-51` imports `Retention` before
  `RefusalReachability`.
- **Drop `Primitives.*` qualifications from `Proofs/` (added 2026-10-24):**
  `lean-v3-authoring-surface` §5.4 removed them from `Cards/` and `Definitions/` only; 61 remain in
  `lean/Semantics/Proofs/` (26 in `Authoring.lean`, 10 in `Turn.lean`, 6 in `Actor.lean`, 4 each in
  `Static.lean` and `CallerScope.lean`, 3 each in `Mana.lean` and `Deontic.lean`, the rest in
  `TextChanges.lean`, `ControllerSacrifice.lean` and `Piles.lean`). Write each through its
  Template, or keep the qualification only where a proof is about the raw constructor itself and
  say so. **Reconciled 2026-10-24: live**

### 3. Synchronize `CONTRACTS.md`, `lean/README.md`, `semantics-v3.md` & Glossary (Opus §§7.1–7.6)

- **`lean/CONTRACTS.md` (§7.1, §7.4):** Update the Rules tables section,
  `DesignationFacts` column list, and replace all 17 dead links to
  `docs/decisions/semantics-v2.md` with `docs/archive/decisions/semantics-v2.md`
  or `docs/decisions/semantics-v3.md`.
  **Reconciled 2026-10-24: live** (partly: the Rules tables section is done by
  [lean-v3-card-type-conferral-rows](../done/lean-v3-card-type-conferral-rows.md) and
  [lean-v3-damage-composition](../done/lean-v3-damage-composition.md), `lean/CONTRACTS.md:1302-1370`;
  the `DesignationFacts` column list is superseded by
  [lean-v3-catalog-items-self-contained](../done/lean-v3-catalog-items-self-contained.md), which
  deleted `DesignationFacts`, and `CONTRACTS.md` no longer names it). Live: the dead links are down
  to three: `lean/CONTRACTS.md:20` (`docs/decisions/semantics-v2.md`, a path that no longer exists)
  and the bare `semantics-v2.md` at `lean/CONTRACTS.md:1060,1107`.
- **`lean/CONTRACTS.md` references the Facts deletion removed (added 2026-10-24):** rewrite each
  against the current declarations:
  - `:83-84` checks `verbBecomesOk` "with the deed table's `intransitive` column" (the deed table
    went with [lean-v3-action-facts-deleted](../done/lean-v3-action-facts-deleted.md));
  - `:84-85` describes `keywordBodyFits` ("card-class guard against the keyword's row"); no such
    check exists in `lean/Semantics` and the keyword rows went with
    [lean-v3-keyword-facts-deleted](../done/lean-v3-keyword-facts-deleted.md);
  - `:733-734` says "`KeywordShapes` and `FactsGen` have no indexed syntax"; neither exists;
  - `:885` (`GivingWarrant` row) says "the facts table no longer carries the `effectful` column";
    the facts table is gone entirely and `effectful` survives on `Definition.designation`
    (`Rules.lean:124`; its retirement is owned by `lean-v3-keyword-bodies-and-catalog-sync` §2).
  `:485-496` (`ActFacts.permanentActor`) is owned by `lean-v3-checker-and-drt-fixes` §3
  (`permanentActor`), which corrects that paragraph; do not edit it here.
  **Reconciled 2026-10-24: live**
- **`lean/CONTRACTS.md:640` changes only with the checker fix (added 2026-10-24):** the `doIf` row
  says a `doIf` with no else-branch has "its output continues", which is the behaviour
  `lean-v3-checker-and-drt-fixes` §1 (disjunction, conditional and negation scoping) changes. That
  ticket owns the row and must rewrite it in the same landing; this ticket must not "sync" it to
  the current code beforehand. **Reconciled 2026-10-24: live**
- **`lean/README.md` (§7.2):** Remove retired rules-row obligations (`nontarget`,
  `grantable`, `counterKindNamed`), nonexistent `DamageResultRule` /
  `Check/Triggers`, old `StaticSpec` constructor names, and `act`; update the pin
  example to `DRS.ofList []`; document the `spelled <|` / `Authoring.Form` /
  `onlyAuthored` authoring boundary and fuel-based termination; fix the dangling
  cross-reference in `Proofs/Numbers.lean:14`.
  **Reconciled 2026-10-24: live** (partly: the `nontarget`, `grantable` and `counterKindNamed`
  obligations and `DamageResultRule` are gone from `lean/README.md`, no single landing ticket). Live:
  `Check/Triggers` at `lean/README.md:137` (no such module; `Check/` has `Events.lean` and
  `AbilityRules.lean`); `act` at `lean/README.md:65-69` (the template is `instruct`,
  `Templates.lean:1612`); "static-spec constructors use nouns (`modification`, `abilityGrant`)" at
  `lean/README.md:60`, which are now Templates (`Templates.lean:2032,1900`); the pin example
  `Action.check [] …` at `lean/README.md:161`; the authoring boundary is documented only in
  `lean/CONTRACTS.md:114-116`, not in the README, and the README's recursion section
  (`lean/README.md:182-196`) says nothing of fuel-based termination; `Proofs/Numbers.lean:14-15`
  still says "see the README" for `Amount`'s nested mutual block, which the README does not describe.
- **`docs/decisions/semantics-v3.md` (§7.3):** Update §2 and §3 code listings and
  CR numbers (`Action`, `can`, `DeonticMods`, cast `[CR#701.5]`,
  create `[CR#701.7]`) to match the current workbench.
  **Reconciled 2026-10-24: live** (partly: cast `[CR#701.5]` and create `[CR#701.7]` are already right
  at `docs/decisions/semantics-v3.md:298`). Live: §3 still writes `Instruction` for `Action`
  (§3.1, `docs/decisions/semantics-v3.md:314-328`) and `may(...)` for the permission constructor `can`
  (:327, :498; `lean/Semantics/Abilities.lean:938`); the §3.7 `DeonticMods` listing (:472-478: `bound`, `rider : DeonticRider`,
  `except : Option Cause`) does not match `lean/Semantics/Abilities.lean:887-899` (`timing`,
  `asThough`, `except : Option DeonticExcept`, `cause`, `rider`).
- **Glossary (`docs/contexts/game-model/CONTEXT.md`, §7.5):** Reconcile Lean's
  `Definition` (`Rules.lean`) vs the glossary's **Registry Definition** and
  remove the retired **Binding Possessor** entry.
  **Reconciled 2026-10-24: live.** **Binding Possessor** is still at
  `docs/contexts/game-model/CONTEXT.md:734`; the glossary's **Definition** (:640, the trusted
  expansion) and **Registry Definition** (:934) are separate entries, but Lean's
  `inductive Definition` (`lean/Semantics/Rules.lean:87`) is the Registry Definition record, so the
  identifier still takes the other entry's name.
- **Clean up stale untracked generated files (§7.6):** Remove any lingering
  `lean/Generated.lean`, `lean/Generated/`, and `lean/GeneratedDefinitions*`
  leftovers if present.
  **Reconciled 2026-10-24: live.** None in this workspace's `lean/`, but the default workspace still
  has untracked `lean/Generated.lean`, `lean/Generated/`, `lean/GeneratedDefinitions.lean` and
  `lean/GeneratedDefinitions/`.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
