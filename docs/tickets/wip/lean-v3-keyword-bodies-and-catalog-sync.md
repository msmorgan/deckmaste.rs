---
needs: [lean-v3-rules-and-card-fidelity]
---
# Complete bodiless keyword definitions, retire unread registry columns, and cross-pin catalog facts

**Supply expansion bodies for the remaining bodiless keyword definitions in
`lean/Semantics/Definitions/Keywords.lean`, retire unread registry columns in
`lean/Semantics/Rules.lean`, and cross-pin the Lean catalog declarations against
`lean/Semantics/Check/Facts.lean` ([`docs/evidence/lean-v3-astra-findings.md`](../../evidence/lean-v3-astra-findings.md)
§§3–4; [`docs/evidence/lean-v3-opus-findings.md`](../../evidence/lean-v3-opus-findings.md)
§§5.2–5.3, 6.9).** Standard constraints apply.

## Scope

### 1. Expand Bodiless Keyword & Action Definitions (Astra §3, Opus §5.2)

- Audit the 35 bodiless keyword definitions (count as of 2026-10-24: the `keyword`,
  `keywordCosting`, `keywordNumber`, `keywordNumberCosting` and `keywordQuality` helper uses, plus
  `companion`'s empty body at `Keywords.lean:17-18`) in
  `lean/Semantics/Definitions/Keywords.lean` and supply rules-faithful expansion
  bodies for all keywords whose CR meaning is expressible in the `semantics-v3`
  AST (including `echo` `[CR#702.30a]`, `typecycling` `[CR#702.29e]`,
  `equipQuality` `[CR#702.6c]`, `frenzy` `[CR#702.68a]`, `suspend` `[CR#702.62a]`,
  `multikicker` `[CR#702.33c]`, `miracle` `[CR#702.94a]`, `prototype`
  `[CR#702.160a]` — removing `set_option linter.unusedVariables false` at
  `Keywords.lean:1291` — plus `learn` `[CR#701.48]` and `ventureIntoTheDungeon`
  `[CR#701.49]` in `Definitions/Actions.lean`; once venture exists, also author the initiative's
  two "ventures into Undercity" triggers on `initiativeDefinition` in
  `Definitions/DesignationRules.lean` [CR#726.2], left out by
  `lean-v3-designation-inherent-triggers` for want of venture). Also declare `empowerJace`
  [CR#701.71a] in `Definitions/Actions.lean` (added 2026-10-24: no declaration exists; Vintage-legal
  cards such as *Jace's Machinations* (FRA) use it). `multikicker`'s body and its kicker label (a
  multikicker cost is a kicker cost `[CR#702.33c]`) are written by
  [lean-v3-keyword-cast-permissions-and-costs](lean-v3-keyword-cast-permissions-and-costs.md)
  §3; do not give it a separate label here.
  **Reconciled 2026-10-24: live** (partly: `typecycling` (`Keywords.lean:1661-1666`) and
  `equipQuality` (`Keywords.lean:645-649`) have bodies, no single landing ticket; `multikicker`
  has its body and the kicker label (`Keywords.lean:1138-1140`), done by
  [lean-v3-keyword-cast-permissions-and-costs](../done/lean-v3-keyword-cast-permissions-and-costs.md).
  Still bodiless: `echo` (`Keywords.lean:549`), `frenzy` (`:803-804`), `suspend` (`:1589-1590`),
  `miracle` (`:1120-1121`), `prototype` (`:1291-1293`, linter option still at `:1291`); `learn`
  and `ventureIntoTheDungeon` are not declared in `Definitions/Actions.lean` at all; the
  initiative's venture triggers are still absent from `initiativeDefinition`
  (`Definitions/DesignationRules.lean:58-59`, gap noted at `:42-45`). The rest of the bodiless
  set — reach, assist, banding, bandsWithOther, chooseABackground, compleated, craft,
  deathtouch, disturb, doctorsCompanion, doubleStrike, encore, enlist, firstStrike,
  freerunning, fuse, moreThanMeetsTheEye, paradigm, the five partner variants and partnerWith,
  phasing, startYourEngines, trample, vigilance, companion — still needs the audit; `toxic`
  (`:1602-1603`) is bodiless by the Q89 ruling.)
  **Done (partly), 2026-10-25:** bodies for `frenzy`, `freerunning`, `encore`, `partnerWith`
  (its entering trigger; the deck-construction ability is Commander's [CR#702.124a]) and
  `vigilance` (a cause-qualified prohibition of the tap attacking causes [CR#702.20b,508.1f]) in
  `nopwpnpu`, pinned `okFrenzyDefinition`, `okFreerunningDefinition`, `okEncoreDefinition`,
  `okPartnerWithDefinition`, `okVigilanceDefinition` (`Proofs/KeywordAbilityCensus.lean`) and the
  cards *Frenzy Sliver*, *Eagle Vision*, *Briarblade Adept*, *Blaring Captain*
  (`Cards/Keyword.lean`). `paradigm` in `spqvkpmt`, pinned `okParadigmDefinition` and
  *Germination Practicum*. `learn` and `empowerJace` declared in `Definitions/Actions.lean` in
  `kqmwntuv` (learn's Lesson is `outsideTheGame`, the complement of the seven zones
  [CR#400.11]), pinned `okLearn`, `okEmpowerJace` (`Proofs/KeywordActionResidues.lean`) and
  *Eyetwitch*, *Countersculpt*. `prototype` records its printed parameters and the linter option
  is gone (`rokqwlum`). Audit result: `reach`, `assist`, `banding`, `bandsWithOther`, `phasing`,
  `startYourEngines`, `toxic`, the partner variants, `chooseABackground` and `doctorsCompanion`
  are intentionally bodiless (next item). Still bodiless, each a STOP awaiting a decision:
  `echo`, `suspend`, `miracle`, `prototype`, `disturb`, `moreThanMeetsTheEye`, `fuse`,
  `compleated`, `craft`, `enlist`, `companion`, `ventureIntoTheDungeon` and the initiative's two
  venture triggers.
- Note: `lifelink`, `wither`, `infect`, and `toxic` were landed by
  [`lean-v3-damage-composition`](../done/lean-v3-damage-composition.md) (Q89 ruled
  2026-10-22: toxic is one rules-table result reading the total toxic value); do not
  duplicate them here.
  **Reconciled 2026-10-24: done** by [lean-v3-damage-composition](../done/lean-v3-damage-composition.md): `lifelink` (`Keywords.lean:1043-1045`), `wither` (`:1747-1750`) and `infect` (`:982-985`) have bodies; `toxic` (`:1602-1603`) is bodiless with its result in the rules table, as ruled.
- Document in `lean/CONTRACTS.md` the exact set of combat/engine-intrinsic
  keywords that remain intentionally bodiless (`firstStrike`, `doubleStrike`,
  `trample`, `vigilance`, `deathtouch`, etc.).
  **Reconciled 2026-10-24: live**: `lean/CONTRACTS.md` names no such set (no match for
  bodiless/intrinsic/firstStrike/trample); the bodiless definitions are `firstStrike`
  (`Keywords.lean:726`), `doubleStrike` (`:540`), `trample` (`:1618`), `vigilance` (`:1709`),
  `deathtouch` (`:471`), `reach` (`:85`) and the rest listed above.
  **Done, 2026-10-25** (`rokqwlum`): `lean/CONTRACTS.md` "Keyword abilities with no body" lists
  the set and where each keyword's rule is, pinned `intentionallyBodiless`
  (`Proofs/KeywordAbilityCensus.lean`).

### 2. Retire Unread Registry Columns & Cross-Pin Catalog Authorities (Astra §4, Opus §§5.3, 6.9)

- **Retire unread columns in `lean/Semantics/Rules.lean` (§5.3):**
  - Remove `effectful` on `Definition.designation` (`Rules.lean:112-124`, ignored
    by every reader (formerly also by `crates/xtask/src/facts/lean.rs:809`, now deleted) and retired per `lean/CONTRACTS.md`
    "GivingWarrant"), `half` / `RoomHalf.designation` / `DesignationLabel.half`
    (since Room doors use `leftHalfUnlocked` / `rightHalfUnlocked` designations),
    and `permanentType` on `Definition.cardType` (`Rules.lean:81`).
    **Reconciled 2026-10-24: live** (partly: `half` / `RoomHalf.designation` /
    `DesignationLabel.half` are done by
    [lean-v3-catalog-items-self-contained](../done/lean-v3-catalog-items-self-contained.md):
    `DesignationLabel` is `named (name) (scope) (sole)` at `Words.lean:269-271` and `RoomHalf` no
    longer exists under `lean/`; `permanentType` is superseded by
    [lean-v3-card-type-composite-definition](../done/lean-v3-card-type-composite-definition.md):
    `Definition.cardType` is `cardType (cardType) (confers)` at `Rules.lean:131`, permanence is
    the read field `CardType.permanent` at `Words.lean:54`. Live: `effectful` still sits on
    `Definition.designation` (`Rules.lean:124`, doc at `:115`), written `true` at
    `Definitions/DesignationRules.lean:38,59` and `Proofs/Rules.lean:491,505,520,526`, and
    matched only as `_`/`..` (`Check/Rules.lean:59`, `Rules.lean:143`, `Proofs/Rules.lean:202`);
    `crates/xtask/src/facts/lean.rs` is gone with the facts generator, and the retirement note is
    `lean/CONTRACTS.md:885` "GivingWarrant".)
    **Done, 2026-10-25** (`zkvuwlvn`): `Definition.designation` is `designation (label)
    (confers)`; the monarch and initiative rows, the `Proofs/Rules.lean` designation pins and
    the "GivingWarrant" note follow.
- **Cross-pin Lean catalogs against `Check/Facts.lean` (§6.9):**
  - Add missing `Empower Jace` (`[CR#701.71]`) to `keywordActionLabels` and a
    participle row for `"blighted"` (`[CR#701.68c]`).
    **Reconciled 2026-10-24: superseded** by
    [lean-v3-action-facts-deleted](../done/lean-v3-action-facts-deleted.md): `keywordActionLabels`
    and the `participle` column are deleted (keyword actions are typed declarations in
    `Definitions/Actions.lean`; `blight` is at `Actions.lean:633-637`). What remains of the
    Empower Jace half is an `empowerJace` Action declaration, which does not exist; it is in
    scope (Vintage-legal cards use it, e.g. *Jace's Machinations*, FRA) and is now owned by §1
    below `learn`/`ventureIntoTheDungeon` (moved 2026-10-24).
  - Extend `lean/Semantics/Proofs/Tables.lean` so every declared `CounterKind`,
    `DesignationLabel`, `RelationLabel`, `KeywordActionLabel`, and `KeywordLabel`
    in `lean/Semantics/Definitions/` is cross-pinned against `Check/Facts.lean`
    until `semantics-v3-dsl-and-plugins` switches the facts generator from
    `plugins_v2/builtin` to `plugins_v3/`.
    **Reconciled 2026-10-24: superseded** by
    [lean-v3-card-type-composite-definition](../done/lean-v3-card-type-composite-definition.md)
    (with [lean-v3-catalog-items-self-contained](../done/lean-v3-catalog-items-self-contained.md),
    [lean-v3-keyword-facts-deleted](../done/lean-v3-keyword-facts-deleted.md) and
    [lean-v3-action-facts-deleted](../done/lean-v3-action-facts-deleted.md)): `Check/Facts.lean`
    is deleted and each catalog item is its own declaration in `Semantics.Definitions`, where a
    duplicate is a Lean error; `Proofs/Tables.lean:1-15` now reads declarations back, with no
    table to pin against.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
