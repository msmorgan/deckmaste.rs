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
  `lean/Semantics/Proofs/KeywordAbilityCensus.lean:18-99`,
  `lean/Semantics/Proofs/BindingIdentity.lean:180-225`,
  `lean/Semantics/Proofs/GiftVariants.lean:21-50` (removing the local `gift`
  shadow), `lean/Semantics/Proofs/KeywordByCardClass.lean:20-40`, and
  `lean/Semantics/Check.lean:16-101` (`exploreDef`, `conniveDef`, `airbendDef`)
  with direct references to the production declarations in
  `lean/Semantics/Definitions/` (rehoming `Check.lean`'s `CorePins` into
  `lean/Semantics/Proofs/`).
- Reconcile `gift`'s class-guard helpers (`isPermanentType` `[CR#702.174b]` vs
  `isPermanentCard`) across `CONTRACTS.md`, `KeywordByCardClass.lean`, and
  `LinkedChoiceReadback.lean` (§6.6).

### 2. Complete the `[CR#704.5g]` Rules Row & Tighten Proof Quality (Astra §6, Opus §§6.3–6.5, 6.8, 7.1)

- **Add the `[CR#704.5g]` lethal-damage row to `builtinTables`
  (`lean/Semantics/Proofs/Rules.lean:395-401`, Astra §6, Opus §7.1):** Now that
  `Stat.defense`, `ProjAxis.markedDamage`, and `GameEvent.stateBasedActionsChecked`
  exist, encode the `[CR#704.5g]` state-based action row and update the docstring
  and `lean/CONTRACTS.md:1036-1041` so only `[CR#704.5h]` deathtouch lookback
  remains noted if still unprojected.
- **Retire vacuous `rfl` restatements (§6.3):** Audit and retire or rewrite the
  ~20 `rfl` expansion-restatement theorems (`Proofs/ActionForms.lean`,
  `controllerSacrificesExpansionForEverySubject`, `lookAndSortSlice_is_the_hand_expansion`,
  `getsPtIsOneCharacteristicChange`, `combatantIsTheRole`, `drawGameConcludes`,
  `grantsIsTheKeywordConferral`, `boostIsOneStatChange`,
  `targetsOnlyIsTargetsAtOnly`, `destroyCantRegenerateIsDestroyThenProhibition`,
  `printedDataNeedsNoExpressionTemplate`) so each kept theorem proves a semantic
  or checker property rather than restating its own definition.
- **Decouple brittle DRS id-arithmetic pins (§6.4):** Rewrite `nextId` / literal
  `ReferentId` arithmetic assertions (especially in `Proofs/ControllerSacrifice.lean`)
  to assert structural referent relationships (distinctness, resolution target,
  role, stamp) rather than raw integer offsets.
- **Pin all live `Refusal` constructors & tighten weak negatives (§6.5):**
  - Replace `.isEmpty = false` / `≠ []` in `badDestroyedPlayer`,
    `badSpeedOfAnObject`, and `amountTemplatesRejectMissingSubjects` with the
    exact expected `[Refusal]` list, and retire `p1p1CounterCountMultipliedTwin`.
  - Add positive/negative pins covering the 12 unpinned `Refusal` constructors
    (`handoffPerformer`, `facesFit`, `counterHolderKind`, `qualityRead`,
    `concreteCost`, `concreteBallot`, `phrasal`, `countedMention`, `choosable`,
    `groupSizeInScope`, `bindingless`, `flatDisjuncts`).
- **Deduplicate proof scaffolding & normalize proof module names (§6.8):** Share
  `olderClauseMaximum`/`olderClauseFilter` and `actionResult`/`nounResult`/`nounZone`
  helpers, remove local shadows (`soFarThisTurn`/`thisGame` in `Lookback.lean`,
  `flyingCounter` in `Proofs/Rules.lean`), and rename ticket-slug proof modules
  (`KeywordAbilityCensus`, `KeywordActionResidues`, `GetsBothDeltas`,
  `CorpusRecency`) to domain names in alphabetical import order in `Proofs.lean`.

### 3. Synchronize `CONTRACTS.md`, `lean/README.md`, `semantics-v3.md` & Glossary (Opus §§7.1–7.6)

- **`lean/CONTRACTS.md` (§7.1, §7.4):** Update the Rules tables section,
  `DesignationFacts` column list, and replace all 17 dead links to
  `docs/decisions/semantics-v2.md` with `docs/archive/decisions/semantics-v2.md`
  or `docs/decisions/semantics-v3.md`.
- **`lean/README.md` (§7.2):** Remove retired rules-row obligations (`nontarget`,
  `grantable`, `counterKindNamed`), nonexistent `DamageResultRule` /
  `Check/Triggers`, old `StaticSpec` constructor names, and `act`; update the pin
  example to `DRS.ofList []`; document the `spelled <|` / `Authoring.Form` /
  `onlyAuthored` authoring boundary and fuel-based termination; fix the dangling
  cross-reference in `Proofs/Numbers.lean:14`.
- **`docs/decisions/semantics-v3.md` (§7.3):** Update §2 and §3 code listings and
  CR numbers (`Action`, `can`, `DeonticMods`, `combatOnly`, cast `[CR#701.5]`,
  create `[CR#701.7]`) to match the current workbench.
- **Glossary (`docs/contexts/game-model/CONTEXT.md`, §7.5):** Reconcile Lean's
  `Definition` (`Rules.lean`) vs the glossary's **Registry Definition** and
  remove the retired **Binding Possessor** entry.
- **Clean up stale untracked generated files (§7.6):** Remove any lingering
  `lean/Generated.lean`, `lean/Generated/`, and `lean/GeneratedDefinitions*`
  leftovers if present.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
