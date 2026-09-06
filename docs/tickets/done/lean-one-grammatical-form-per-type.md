---
needs: []
---
**One grammatical form per type; spelling and ordering only, no semantic
change.** The 2026-09-05 review found the Lean constructors and macros mixing
imperative, third-person and noun forms with no rule other than the card
sentence each was first written for (`exile x` beside `exiles agent x`,
`sacrifice agent x` beside `mills agent n whose`, `StaticSpec.gains` beside
`StaticSpec.modify`). Fix, in one landing:

- `Instruction` constructors and the instruction macros in
  `lean/Semantics/Macros.lean` use the dictionary form; the agent is an optional
  trailing named argument, so `exile x` and `exile x (agent := you)` replace the
  `exile`/`exiles` pair, `mills` becomes `mill`, `chooses`/`secretlyChooses`
  fold into `choose`, `losesLife`/`gainsLife` become `loseLife`/`gainLife`.
- `GameEvent` constructors stay third-person: an event is a predication.
- `StaticSpec` constructors become nouns naming the kind of continuous effect
  (`modify` → `modification`, `gains` → `abilityGrant`, `costs` → `costShift`,
  `skips` → `partSkip`, `becomes` → `qualityChange`, `intercepts` →
  `replacement`, and so on); the claimant lists the full map in the landing
  record.
- `CounterBatch.last` is renamed for what it is, the removal that leaves none
  of that kind (`emptying`), and its docstring records that the emptiness is
  part of the trigger event, read at trigger time, and NOT an intervening if:
  [CR#603.4] rechecks an intervening if on resolution, and the CR itself
  writes the phrase as a trigger event ([CR#310.12b], [CR#702.62a], where a
  separate "if it's exiled" follows it). Do not re-express it as a condition.
- `TurnPart` variants are reordered to the [CR#500.1] sequence (turn, beginning
  phase, untap, upkeep, draw, main, first main, combat, declare attackers,
  declare blockers, first-strike damage, combat damage, end of combat,
  postcombat main, end step, cleanup). Only `DecidableEq` is derived, so the
  order carries no meaning today; it is a reading aid.

Every pin keeps its name and expected list; the verdict-diff script with
spelling normalisation must report 0 problems. Update
`docs/contexts/game-model/CONTEXT.md` entries **Instruction** and **Static
Spec** if their wording assumes the old forms.


## Landing record

Measured tree: `zzttnuzlokqrznkmrzskvzkuurxxsymu`, English coverage-lock
`covered` count **20,254**. This is a semantics-only source migration; the
English Lake project, Rust sources, registry inputs, and coverage lock have
no edits. Default advanced before integration; the final gate was rerun
after refreshing onto that updated tree.

### PROVE

- **No silent loss:** all **2,106** existing named pins remain, with **0**
  verdict problems after spelling normalization. All refusal lists are
  unchanged. The one non-list expected expression that changes is
  `stormCountsEarlierThisTurn`, whose expected syntax uses the renamed
  constructors and reordered agent arguments. All **815** `Spelled` card
  definitions retain their names and remain part of the build.
- The comparison uses the existing verdict-diff approach with a parser fix:
  named-argument `:=` is not an equality, and parameterized theorems and
  term proofs are included. Normalization applies the explicit symbol map
  and argument permutations obtained from Lean LSP references to the
  pre-edit source. Comparison scripts and snapshots remain session scratch.
- The five changed checker modules match only that symbol/argument migration
  after whitespace normalization. Field-context computations, refusal
  conditions, their concatenation order, and every expected refusal are
  preserved. No test subject is retired.
- **Structural laws and no word-naming:** this landing changes no English
  parsing, rendering, traversal, licensing checker, or environment loading.
  Their corpus gates are not rerun for an independent Lean source migration.
  No coverage identities are newly lost or newly covered by this change.
- `cargo xtask gate --changed`: **no workspace crates affected**. No Rust
  tests are selected. `lean/scripts/build` passes **59 jobs**, including
  all existing pins/cards and the six added pins. Lean LSP diagnostics are
  clean for the declarations, checker, macros, and new pin module.
- Citation checks: **0 noncompliant, 0 stale**; the diff audit covers **3**
  citation sites, each checked against the actual rule text.

### DISCLOSE

Instructions and instruction macros now use dictionary forms; static-spec
constructors use nouns. GameEvent constructors retain their names. The full
StaticSpec map, including unchanged noun forms, is:

| Previous constructor | Constructor after this landing |
| --- | --- |
| `modify` | `modification` |
| `definesPt` | `ptDefinition` |
| `switchesPt` | `ptSwitch` |
| `costs` | `costShift` |
| `altCost` | `altCost` |
| `addedCost` | `addedCost` |
| `definesLetter` | `letterDefinition` |
| `gains` | `abilityGrant` |
| `gainsAbilitiesOf` | `abilityGrantFrom` |
| `deontic` | `deonticRule` |
| `keepsUnspentMana` | `manaRetention` |
| `skips` | `partSkip` |
| `becomes` | `qualityChange` |
| `alsoOffBattlefield` | `offBattlefieldScope` |
| `doesntRemove` | `retention` |
| `becomesCopy` | `copyChange` |
| `losesAllAbilities` | `allAbilityLoss` |
| `losesAbilities` | `abilityLoss` |
| `gainsControl` | `controlGrant` |
| `intercepts` | `replacement` |
| `damageRule` | `damageRule` |
| `cantPrevent` | `preventionBan` |
| `conditionally` | `conditional` |
| `onlyDuring` | `partScope` |
| `visibility` | `visibility` |
| `triggersAdditionally` | `additionalTriggers` |
| `entersRider` | `entryRider` |
| `entersChoice` | `entryChoice` |
| `attachChoice` | `attachmentChoice` |
| `andAlso` | `conjunction` |

Instruction agents move to optional trailing `agent` arguments. Existing
explicit actors are preserved at call sites; existing optional slots on
`enact`, `choose`, and turn-part additions retain `none`. Required player
slots default to `.you`. Named macro agents use the same policy, with
`exile` absorbing `exiles`, and `choose` absorbing `chooses` and
`secretlyChooses` through optional `disclosure`. No absent agent is silently
replaced by `.you` in existing syntax.

`CounterBatch.emptying` remains an event qualifier, with the trigger-time
meaning and intervening-if distinction documented beside the constructor.
TurnPart is reordered as requested. The ticket's statement that only
DecidableEq is derived was slightly stale: Repr is also derived, and both
derivations are retained.

**Deviations and additions:**

- Six API pins in `Proofs/InstructionForms.lean` verify omitted versus
  explicit exile/choice agents, secret choice, and explicit mill agency.
  They test the public authoring interface, including coercion into optional
  agent slots. Lean LSP verification reports no axioms or warnings for the
  secret-choice pin.
- Test accounting: **0 restored, 596 respelled, 0 newly ignored, 6 added,
  0 removed**. The remaining existing pins require no textual migration.
- README syntax guidance and its `badChooseYou` example now use the new API;
  CONTRACTS links to constructor names are updated. The Game Model's
  Instruction and Static Spec entries already define the concepts without
  assuming a grammatical spelling, so they need no change. No glossary gap.
- No English constructions or tests added or removed; new-analysis list and
  coverage-loss list are empty for this landing. The English selection
  census and licensing-checker total are not remeasured for this scope.
- **STOPs:** none. Mechanical migration/layout errors were corrected before
  landing. A build raced a later source formatting edit and missed a rebuilt
  dependency artifact; the final gate is run after source edits finish.

### REPORT

All counts above refer to change `zzttnuzlokqrznkmrzskvzkuurxxsymu` and the
unchanged lock count **20,254**. English construction count, homograph and
form-literal/vocabulary inventories, coverage wall time, and per-byte thread
CPU telemetry are not measured by this semantics-only landing. No performance
claim is made against the English quiet-host ceiling; no corpus workers ran.
No branch was pushed and no hosted CI result is claimed.
