---
needs: [lean-v3-keyword-bodies-and-catalog-sync]
---
# Align Lean v3 proof suites and documentation with production definitions

Point the Lean proof suites at the production declarations in `lean/Semantics/Definitions/`
instead of hand-copied replicas. Tighten the weak negative pins, pin every reachable `Refusal`
constructor, retire the `rfl` restatements and remove raw referent-id arithmetic from pins. Share
the duplicated proof scaffolding, rename the slug-named proof modules, and bring
`lean/CONTRACTS.md`, `lean/README.md`, `docs/decisions/semantics-v3.md` and the Game Model
glossary in line with the current tree. The sources are
[`docs/evidence/lean-v3-astra-findings.md`](../../evidence/lean-v3-astra-findings.md) §§5–6 and
[`docs/evidence/lean-v3-opus-findings.md`](../../evidence/lean-v3-opus-findings.md) §§6.1–6.8 and
§§7.1–7.6. Standard constraints apply.

This is an implementation specification, rewritten 2026-10-25 against the tree at the commit that
records it. Do exactly what it says. Where something is not stated here, it is out of scope.

## How to read this spec

- **Line numbers** are those in the tree at this spec's commit, the parent of commit C1 below.
  Every earlier commit in the plan moves lines. Locate each site by the quoted text or the
  declaration name; the number only tells you where to start looking. If the quoted text is not
  found, stop and report: do not guess.
- All Lean paths are under `lean/Semantics/`, and `Proofs/X.lean` means
  `lean/Semantics/Proofs/X.lean`, unless a path starts with `lean/`, `docs/` or `crates/`.
- "Retire" means delete the declaration together with its docstring and the blank line after it.
  Every retirement below gives its reason. Repeat those reasons in the commit description.
- Code blocks are exact replacement text. Keep the surrounding file's indentation.
- Every replacement theorem in this spec was compiled and its `decide`/`rfl` checked against the
  built tree on 2026-10-25. Two have their own notes: the `anchorsOf` conjunct of
  `okOtherCardOnACreature` (C5) and `grantsIsTheKeywordConferral` (C1), whose renamed form was
  checked in isolation. If a pin fails in place, stop and report. Do not change its asserted
  result.

## Decisions taken in this spec (no owner question is open for the commit plan)

1. **Checker fixtures stay.** `revealThenCountDef` and `drawDiscardCountDef` (now in `Check.lean`)
   are deliberate fixtures, not replicas: each is a Definition that hands off to its own
   parameter, which no production declaration does. They move with `CorePins` and get docstrings
   that say they are fixtures.
2. **`CorePins` moves to `Proofs/CheckerCore.lean`** and keeps its snake_case theorem names. Its
   pins then run only in the full gate (`./lean/scripts/build`), not in `lake build Semantics`.
3. **Duplicates of production pins are retired, not rewritten.** Once the replica is gone, a pin
   that only restates an existing production pin is retired: BindingIdentity's connive section,
   `Check.lean`'s `airbend_def_sound` and `sliceCFactOrFictionPin`, and Actor's
   `badExploreHandedToOpponent`.
4. **A card-fidelity anchor is not an `rfl` restatement.** A conjunct that ties a checked `let` to
   a corpus card's own `def` stays: `CardFidelity.lean:37,58,83,96,211`, `Damage.lean:860,1248`.
   The §6.3 sweep retires only `Template args = <its own body>`.
5. **Referent-id arithmetic** is removed where a public pin asserts an offset from `nextId` or
   `fresh`. Private computation lemmas that must name `nextId + i` because `DRS.prepend`
   allocates that way stay. The 239 other literal-id assertions are not audited here; C9 mints a
   follow-up ticket for them.
6. **`Refusal.phrasal`** is reachable only through `Predicate.checkAt` at a pile or outcome head.
   That is a public checker entry point, so it is pinned there. No card text reaches it, and C9's
   follow-up ticket records that.
7. **ADR edits are in place.** `docs/decisions/semantics-v3.md` is edited in place, as the recent
   doIf ruling was (change `suvlwolo`). A rename to match the code needs no ruling note.
8. **The generated leftovers are an owner action** (see "Owner actions"). No commit touches them or
   `.gitignore`.
9. **No new `.gitignore`, `lakefile.toml` or xtask change.** The new modules are built through the
   `Semantics.Proofs` root.

## Already done (delete from scope; evidence)

- **§1 `KeywordByCardClass.lean`**: its pins already read production `gift (draw 1)`, `haunt` and
  `ascend` (`Proofs/KeywordByCardClass.lean:38-56`). `halves` and `card` are helpers, and `halves`
  moves in C1.
- **§1 haste pin**: `okHasteDefinition` (`Proofs/KeywordAbilityCensus.lean:106-108`) reads
  production `haste` (`Definitions/Keywords.lean:953`).
- **§1 gift class-guard helpers**: superseded by `lean-v3-by-card-class`. Gift's halves are a
  `byCardClass` pair (`Definitions/Keywords.lean:890-906`), and neither `isPermanentType` nor
  `isPermanentCard` appears in `KeywordByCardClass.lean` or `LinkedChoiceReadback.lean`.
- **§1 alias helpers**: `regenerationApplicationBy := regeneration`, `regenerateBy := regenerate` and
  `airbendByCard := airbend` (`Proofs/BindingIdentity.lean:251,252,284`) are aliases of production,
  not replicas.
- **§2 `[CR#704.5g]` lethal damage**: done by `lean-v3-card-type-conferral-rows`.
  `lethalDamage` is on the creature `Definition.cardType` row (`Definitions/CardTypeRules.lean`)
  and pinned in `Proofs/Rules.lean`. The Saga and Siege guards were done by
  `lean-v3-saga-rules-fidelity`. The storm and replicate targets guard is owned by
  `lean-v3-rules-and-card-fidelity` §2.
- **§2 `destroyCantRegenerateIsDestroyThenProhibition`**: no longer exists.
  `destroyCantRegenerateRidesTheDestroy` (`Proofs/Deontic.lean:372`) is its successor and is
  untouched.
- **§2 `concreteCost`, `groupSizeInScope`, `recursionLimit` and `chosenQualityRead` pins**: they
  exist (`Proofs/SelectionEvidence.lean:202,214,255`, `Proofs/DefinitionsAndTemplates.lean:106,110`,
  `Proofs/Choice.lean:34`). `concreteBallot` has a private pin at `Check/Queries.lean:221-225`; C5
  adds a public one.
- **§3 CONTRACTS Rules tables section**: done by `lean-v3-card-type-conferral-rows` and
  `lean-v3-damage-composition`.
- **§3 CONTRACTS `DesignationFacts` column list**: superseded by
  `lean-v3-catalog-items-self-contained`, which deleted `DesignationFacts`.
- **§3 CONTRACTS `GivingWarrant` row** (`lean/CONTRACTS.md:940`): it already says the facts table
  is deleted and that `Definition.designation` has no `effectful` field.
- **§3 CONTRACTS doIf row** (`lean/CONTRACTS.md:695`): rewritten by `lean-v3-checker-and-drt-fixes`
  under owner ruling 2026-10-24, and it matches `Check/Phrase.lean:4271-4320`.
- **§3 CONTRACTS `ActFacts.permanentActor` paragraph** (`lean/CONTRACTS.md:509-519`): rewritten by
  `lean-v3-checker-and-drt-fixes`. It names `DeedPermanent.performs` / `DeedPermanent.isAbout`.
- **§3 CONTRACTS "`KeywordShapes` and `FactsGen` have no indexed syntax"** (`lean/CONTRACTS.md:788-789`):
  obsolete as a defect. The paragraph inventories `idris/src/Experimental/*.idr`, where both
  files exist, so the sentence is true. Do not edit it.
- **§3 README `nontarget`, `grantable`, `counterKindNamed`, `DamageResultRule`**: gone from
  `lean/README.md`.
- **§3 ADR cast and create numbers**: `cast` `[CR#701.5]` and `create` `[CR#701.7]` are right at
  `docs/decisions/semantics-v3.md:298` (`data/rules/cr.txt:3305,3317`).

## Blocked on the rules-rows question (excluded from the commit plan)

The owner has an open question: do rules rows stay as DSL items? Until it is answered, nothing
here adds, removes or reshapes a rules row or the registry `Definition` type.

- **B1. Reconcile Lean's `inductive Definition` with the glossary.** `inductive Definition`
  (`Rules.lean:87`) has the docstring "What one registry declaration MEANS: the rules content the
  declared name stands for". That is the glossary's **Registry Definition**
  (`docs/contexts/game-model/CONTEXT.md:949`). Its constructors are `counter`, `subtype`,
  `designation`, `cardType` and `relation`. The glossary's **Definition** (`:650`) is the trusted
  expansion (`semantic_definition`), and its `_Avoid_` line reserves "Registry Definition" for
  exactly this record. So the identifier takes the name of the wrong entry.
  - The rule is "rename identifiers; never reword entries", so the fix is to rename the Lean type
    to `RegistryDefinition`. The glossary does not change.
  - Blast radius: 32 type references (Rules.lean 2, Check/Rules.lean 2, Definitions/CardTypeRules.lean
    12, SubtypeRules 3, DesignationRules 3, Proofs/Rules.lean 10). About 52 qualified
    `Definition.<x>` uses (Proofs/Rules 35, Check/Rules 3, Rules 3, DesignationRules 2,
    LinkedReadback 2, Words 2, and CardTypeRules, CardTypes, SubtypeRules, Proofs/Counters and
    Templates 1 each). Docs: `lean/CONTRACTS.md` 9 mentions, `lean/README.md:88` 1, the ADR 1
    (including the Law Gate cell C8 writes). Leading-dot constructor uses need no change. The
    deletion-bound Rust `deckmaste_semantics_v2::rules::Definition` is left alone.
  - Why it is blocked: if rules rows stop being DSL items, `inductive Definition` and
    `Check/Rules.lean` are reshaped or deleted and the rename is moot. If they stay, do the rename
    as one mechanical commit, `lean: the registry record is a RegistryDefinition`, after C8.
- **B2.** Every other rules-row item this ticket once carried is done or owned elsewhere (see
  "Already done"). The commit plan changes no row in `builtinTables`,
  `Definitions/CardTypeRules.lean`, `SubtypeRules.lean` or `DesignationRules.lean`.

## What must not change

- No production declaration in `Definitions/`, `Templates.lean`, `Abilities.lean`, `Rules.lean`,
  `Check/` or `Cards/` changes, except these docstring fixes in C8:
  `Cards/Damage.lean` (`purgingScythe`) and `Check/Events.lean` (module docstring).
- No rules row changes (see B2).
- No asserted refusal list is weakened. Every rewrite below asserts at least as much as the pin
  it replaces, except where the spec retires a pin and gives the reason.
- `docs/tickets/done/*` and `docs/evidence/*` are historical records. Do not rewrite references
  in them, including the old module names.
- `lean/CONTRACTS.md:695` (doIf row), `:509-519` (permanentActor), `:788-789` (Idris inventory)
  and `:940` (GivingWarrant) are already correct. Leave them alone.
- `Proofs/Authoring.lean`'s 16 `Primitives.` occurrences at :134, 136, 138, 140, 154, 156, 219,
  221, 223, 225, 227, 229, 262, 264, 266 and 268 stay. They pin that no `Primitives` wrapper
  exists for a catalog item, so the proof is about the raw namespace itself.
- `lakefile.toml`, `.gitignore` and `crates/` are not edited.

## Commit plan

Do the commits in this order, each with `jj --no-pager commit -m "<message>"`. Write each body
exactly as given, then append the list of retired pins with their reasons. Run the per-commit gate
before each commit:

```sh
./lean/scripts/build            # lake build --wfail: zero errors, zero warnings, no sorry
cargo xtask lean check
cargo xtask cite check --list-noncompliant   # must print nothing
cargo xtask cite check                       # 0 stale
jj --no-pager diff --git | cargo xtask cite audit --diff   # read each rule against its claim
```

Run `cargo xtask cite bless` only where a commit says it adds a citation.

| # | Message | Items |
|---|---|---|
| C1 | `lean: shared proof helpers; drop proof-local shadows` | §C1 |
| C2 | `lean: proof suites read production Definitions; CorePins becomes Proofs/CheckerCore` | §C2 |
| C3 | `lean: retire rfl restatements; rewrite the rest to the property they claim` | §C3 |
| C4 | `lean: structural referent pins in place of id arithmetic` | §C4 |
| C5 | `lean: exact refusal lists; pin every reachable Refusal constructor` | §C5 |
| C6 | `lean: proof suites write Templates, not Primitives wrappers` | §C6 |
| C7 | `lean: domain names for the slug-named proof modules` | §C7 |
| C8 | `docs: align CONTRACTS, README, semantics-v3 and the glossary with the workbench` | §C8 |
| C9 | `tickets: follow-ups from proofs-and-docs-alignment` | §C9 |

---

## C1. Shared proof helpers and proof-local shadows

### C1.1 New module `Proofs/Helpers.lean`

Create `lean/Semantics/Proofs/Helpers.lean` with exactly this content:

```lean
import Semantics.Check.Card

/-!
# Semantics.Proofs.Helpers

Scaffolding the pin suites under `Semantics.Proofs` share: projections of a checker step's
context (at the fuel the `.check` entry points use, 64), the clause-ranking lemmas behind the
universally quantified resolution pins, and small card and context fixtures.
-/

namespace Semantics.Proofs

/-- The context after checking `action` as a resolving instruction, the step `Action.check`
takes. -/
def actionResult (drs : DRS) (action : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs action).1

/-- The context after checking `noun`, by default as an `.other` mention outside an argument slot,
the step `NounPhrase.check` takes; pass `.obj true` for a direct object in an argument slot. -/
def nounResult (drs : DRS) (noun : NounPhrase) (role : GramRole := .other)
    (inArgSlot : Bool := false) : DRS :=
  (stepNounPhrase 64 drs role inArgSlot noun).1

/-- The zone of the first referent `noun` selects, if it has one. -/
def nounZone (drs : DRS) (noun : NounPhrase) : Option Zone :=
  let result := stepNounPhrase 64 drs .other false noun
  result.2.1.head? >>= fun id => (result.1.findById? id).bind (·.zone)

/-- Folding `Nat.max` over referents all from clauses older than `clause` leaves `clause`. -/
theorem olderClauseMaximum (rs : List Referent) (clause : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    rs.foldl (fun acc r => Nat.max acc r.clause) clause = clause := by
  induction rs with
  | nil => rfl
  | cons r rs ih =>
    have hr := older r (by simp)
    have ht : ∀ r ∈ rs, r.clause < clause := by
      intro r h
      exact older r (by simp [h])
    simp only [List.foldl_cons, Nat.max_eq_left (Nat.le_of_lt hr)]
    exact ih ht

/-- No referent from a clause older than `clause` is in `clause`. -/
theorem olderClauseFilter (rs : List Referent) (clause : Nat)
    (older : ∀ r ∈ rs, r.clause < clause) :
    rs.filter (fun r => r.clause == clause) = [] := by
  induction rs with
  | nil => rfl
  | cons r rs ih =>
    have hr := older r (by simp)
    have ht : ∀ r ∈ rs, r.clause < clause := by
      intro r h
      exact older r (by simp [h])
    simp [Nat.ne_of_lt hr, ih ht]

/-- With no direct-argument mention buffered in the current clause, `D′` ranks every referent by
the clause and role it was introduced or last foregrounded in. -/
theorem rankingWithoutPending (drs : DRS) (h : drs.pendingRoles = []) :
    drs.rankingClause = (·.clause) ∧ drs.rankingRole = (·.role.rank) := by
  constructor <;> funext ref <;> simp [DRS.rankingClause, DRS.rankingRole, DRS.isClauseMate, h]

/-- The context no text has been checked in. -/
def emptyContext : DRS := DRS.ofList []

/-- Kind, number and determiner of each binding, newest first. -/
def shape (bs : DRS) : List (Kind × Plurality × Determiner) :=
  bs.referents.map fun b => (b.kind, b.plur, b.det)

/-- A card's printed abilities (a single-faced card). -/
def printedText (card : Spelled) : List Ability :=
  match card.card with
  | .singleFaced face => face.characteristics.text
  | _ => []

/-- An enchantment with the given text. -/
def enchantmentWith (text : List Ability) : Card :=
  .singleFaced { characteristics := { name := "", types := [.enchantment], text } }

/-- The halves of a keyword declaration's body, a `byCardClass` pair as both of its halves. -/
def halves : Ability → List Ability
  | .keyword _ _ body => body.flatMap fun
    | .byCardClass onPermanent onInstantOrSorcery => [onPermanent, onInstantOrSorcery]
    | ab => [ab]
  | _ => []

end Semantics.Proofs
```

In `lean/Semantics/Proofs.lean`, insert `import Semantics.Proofs.Helpers` between
`import Semantics.Proofs.GiftVariants` and `import Semantics.Proofs.HistoricalMove`.

### C1.2 Delete the local copies

In each file below, add `import Semantics.Proofs.Helpers` as the last `import` line and delete the
listed line ranges. Each range includes the docstring and the trailing blank line. Faces gets no
import: it only loses `activatedAbility`.

| File | Delete | What goes |
|---|---|---|
| `Proofs/Anaphora.lean` | 17-20 | `actionResult`, `nounResult` |
| `Proofs/EventPatterns.lean` | 8-15 | `nounZone`, `nounResult` |
| `Proofs/Actor.lean` | 16-20, 36-39 | `actionResult`, `nounResult` (unused), `shape` |
| `Proofs/Description.lean` | 20-22 | `actionResult` |
| `Proofs/CallerScope.lean` | 11-14 | `actionResult`, `nounResult` (keep `selected` at 15-17) |
| `Proofs/ActionFamilies.lean` | 8-17 | `nounZone`, `actionResult`, `nounResult` |
| `Proofs/Composition.lean` | 8-10 | `nounResult` |
| `Proofs/Choice.lean` | 15-17 | `actionResult` |
| `Proofs/Deontic.lean` | 19-25, 493-498 | `actionResult`, `activatedAbility`, `printedText` |
| `Proofs/Mana.lean` | 16-22, 130-133 | `actionResult`, `activatedAbility`, `enchantmentWith` |
| `Proofs/BindingIdentity.lean` | 19-27 | `actionResult` (flipped arguments), `nounResult`, `shape` |
| `Proofs/Attachments.lean` | 8-11 | `nounZone` |
| `Proofs/GetsBothDeltas.lean` | 16-18 | `actionResult` |
| `Proofs/Keyword.lean` | 24-30, 138-141 | `nounResult`, `activatedAbility`, `enchantmentWith` |
| `Proofs/Turn.lean` | 16-25 | `actionResult`, `nounResult`, `activatedAbility` |
| `Proofs/Faces.lean` | 19-22 | `activatedAbility` (no import added) |
| `Proofs/DuringResolution.lean` | 24-30 | `halves` |
| `Proofs/KeywordByCardClass.lean` | 20-26 | `halves` |
| `Proofs/RefusalReachability.lean` | 8-9 | `emptyContext` |
| `Proofs/Retention.lean` | 22-23 | `emptyContext` |
| `Proofs/Counters.lean` | 686-690 | `printedText` (keep the `/-!` section comment at 680-684) |
| `Proofs/ComparisonBox.lean` | 35-65 | `olderClauseMaximum`, `olderClauseFilter`, `rankingWithoutPending` |
| `Proofs/ReferenceScopes.lean` | 24-54 | the same three |

The three clause lemmas were `private`. They become public in `Helpers` and their use sites need
no edit.

### C1.3 Call sites that change

- `Proofs/BindingIdentity.lean`: its `actionResult` took `(i : Action) (drs : DRS)`. Swap the
  arguments at its six uses:
  - :51 `targets ((tapThenUntap (target isCreature)) |> fun i => actionResult i (DRS.ofList []))` →
    `targets (actionResult (DRS.ofList []) (tapThenUntap (target isCreature)))`
  - :57 the same with `tapThenUntapCopied`.
  - :110 the same with `(fightByIdentity (target isCreature) (target isCreature))`.
  - :152 `actionResult (instruct (each (.isOpponentOf .you)) (discardThenExile (a .isCard))) (DRS.ofList [])` →
    `actionResult (DRS.ofList []) (instruct (each (.isOpponentOf .you)) (discardThenExile (a .isCard)))`
  - :265 `actionResult body context` → `actionResult context body`
  - :269 `actionResult (regenerateBy (target isCreature)) (DRS.ofList [])` →
    `actionResult (DRS.ofList []) (regenerateBy (target isCreature))`
- `nounResult` call sites that used the `.obj true` variant get explicit arguments:
  - `Proofs/Anaphora.lean:615` `(nounResult (DRS.ofList []) (youAnd thisCreature))` →
    `(nounResult (DRS.ofList []) (youAnd thisCreature) .obj true)`
  - `Proofs/BindingIdentity.lean:62` `nounResult (DRS.ofList []) (.firstMention "m.0" (target isCreature))` →
    `nounResult (DRS.ofList []) (.firstMention "m.0" (target isCreature)) .obj true`
  - `Proofs/BindingIdentity.lean:69` →
    `nounResult (nounResult (DRS.ofList []) (.firstMention "m.0" (target isArtifact)) .obj true) (target isCreature) .obj true`
- `activatedAbility` is replaced by the Templates `activated` (`Templates.lean:1850`) and
  `activatedOnlyDuring` (`Templates.lean:1853`):
  - Every `activatedAbility X Y` becomes `activated X Y`: Deontic 100, 132, 139, 146, 154; Faces 235;
    Keyword 33, 125, 299; Mana 30, 37, 45, 51, 55, 136, 183; Turn 54, 62, 87, 96, 117, 139, 147,
    154, 163.
  - The four timed calls become `activatedOnlyDuring X Y T`, dropping the `some`:
    - Mana:79-80 `(activatedAbility (.mana [generic 2]) (draw 1) (some (beforePart .combatDamage none)))` →
      `(activatedOnlyDuring (.mana [generic 2]) (draw 1) (beforePart .combatDamage none))`
    - Mana:87-88 `(activatedAbility … (draw 1) (some (beforePart .declareAttackers (some (each .isPlayer)))))` →
      `(activatedOnlyDuring … (draw 1) (beforePart .declareAttackers (some (each .isPlayer))))`
    - Turn:268-269, the same pattern with `duringPart .endStep (some (each .isPlayer))`.
    - Turn:359-360, the same pattern with `duringPart .endingPhase (some (each .isPlayer))`.
- `docs/tickets/planned/lean-v3-regenerate-target-readback.md:60` quotes
  `targets (actionResult (regenerateBy (target isCreature)) (DRS.ofList [])) = 1`. Change it to
  `targets (actionResult (DRS.ofList []) (regenerateBy (target isCreature))) = 1`.

Do not merge these same-named but different helpers: `faceText` (CardFidelity:17,
GetsBothDeltas:410), `triggerEvents` (DuringResolution:32, CardFidelity:190), `selected`
(CallerScope:15, CorpusRecency:38), `artifactCreature` (TypeEvidence:49, Faces:729), `cost`
(SpellCopies:42, KeywordCastPermissions:24, LinkedReadback:317), `chosenDraws`
(LinkedChoiceReadback:27, KeywordByCardClass:59) and `argumentResult` (ControllerSacrifice:414,
MovementScopes:36).

### C1.4 Local shadows

- `Proofs/Lookback.lean:8-11`: delete the local `soFarThisTurn` and `thisGame`. They are identical
  to `Templates.soFarThisTurn` (`Templates.lean:838-839`) and `Templates.thisGame` (`:850`),
  which the file already opens at :6. Their uses at :16, 83, 159, 171, 174, 178 and 182 then
  resolve to the Templates.
- `Proofs/Rules.lean:160-182`. The proof-local `Definition.counter` fixtures shadow the
  production `CounterKind`s `chargeCounter` (`Definitions/Counters.lean:45`) and `flyingCounter`
  (`Definitions/Counters.lean:20`). These fixtures are not rules rows. Rename them to the file's
  own `…CounterDefinition` convention and fold the conferral-helper pins into one. Make three
  edits; the docstrings at 160-162 and 166-169 stay as they are.
  1. Replace lines 163-164 with:

```lean
def chargeCounterDefinition : Definition := .counter chargeCounter []
theorem okChargeCounter : Definition.check chargeCounterDefinition = [] := by decide
```

  2. Replace lines 170-182, from `def flyingCounter : Definition :=` through the end of
     `boostIsOneStatChange`, with:

```lean
def flyingCounterDefinition : Definition :=
  .counter flyingCounter [.property (Templates.abilityGrant .this (keyword (.named "flying")))]
theorem okFlyingCounter : Definition.check flyingCounterDefinition = [] := by decide

/-- A keyword counter's conferral written with `grants` is the one `flyingCounterDefinition`
writes, the keyword gained by its bearer [CR#122.1b,613.1f]. -/
theorem grantsIsTheKeywordConferral :
    flyingCounterDefinition = .counter flyingCounter [grants (.named "flying")] := rfl
```

  3. At :223, `.counter (.named "chargeCounter" (.quality .color)) []` is an inline malformed
     fixture, not the renamed def. Leave it.

  Retired here: `boostIsOneStatChange`, because it restates `boost`'s body. `boost`'s uses are
  checked by `p1p1CounterDefinitionZone` and `m1m1CounterDefinitionZone`.
  `grantsIsTheKeywordConferral` is rewritten from a body restatement into a convergence: two
  spellings, one conferral. It was checked against the renamed fixtures in isolation.

---

## C2. Proof suites read production Definitions

### C2.1 `Proofs/KeywordAbilityCensus.lean`

The production declarations exist: `saddle` (`Definitions/Keywords.lean:1535`), `teamwork`
(`:1749`), `awaken` (`:284`) and `indestructible` (`:1055`). Each replica differs from production
only in production's `.inCaller` wrapping of its arguments.

Lines 1-3 (the three `import` lines) and the blank line 4 stay. Replace lines 5-95 (the module
docstring through `okIndestructibleDefinition`) with the text below. Lines 96 onward stay.

```lean
/-!
# Semantics.Proofs.KeywordAbilityCensus

The keyword abilities `semantics-v2-keyword-ability-census` bodied from vocabulary that already
existed: saddle and teamwork over a group bounded by its total power (`SliceCount.totalling`, as
crew's cost is written), awaken as an alternative cost and a spell ability, and indestructible as
a prohibition over the destroy deed. Each pin reads the declaration in `Semantics.Definitions`.
`lean-v3-keyword-bodies-and-catalog-sync` bodied frenzy, freerunning, encore, paradigm, partner with
and vigilance the same way; their pins follow the census's.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.KeywordAbilityCensus

/-- "Saddle 2" checks [CR#702.171a]. -/
theorem okSaddleDefinition : Ability.check (DRS.ofList []) (saddle 2) = [] := by
  decide

/-- Saddling taps other creatures: a total over "creatures you control" with no "other" may
include the saddled permanent itself, and is a different cost. The bound is still an amount: an
unresolved "its power" is refused in saddle's cost. -/
theorem badSaddleUnresolvedBound :
    (match saddle (powerOf (that (.type .creature))) with
      | .keyword _ _ [.activated cost _ _ _ _ _] => Cost.check (DRS.ofList []) cost
      | _ => []) =
      [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "Teamwork 2" checks [CR#702.194a]. -/
theorem okTeamworkDefinition : Ability.check (DRS.ofList []) (teamwork 2) = [] := by
  decide

/-- "Awaken 4—{5}{U}" checks [CR#702.113a]. -/
theorem okAwakenDefinition :
    Ability.check (DRS.ofList []) (awaken 4 (.mana [generic 5, pip .blue])) = [] := by
  decide

/-- Indestructible checks [CR#702.12b]. -/
theorem okIndestructibleDefinition : Ability.check (DRS.ofList []) indestructible = [] := by
  decide
```

Retired here: `saddleDefinition`, `teamworkDefinition`, `awakenCost`, `awakenDefinition` and
`indestructibleDefinition`, all replicas of production declarations.

### C2.2 `Proofs/BindingIdentity.lean`

Replace lines 180-242, from `private def scryBy` through the blank line before
`/-! ## Regenerate` at :243, with:

```lean
/-- The plurality of the slice's later mention in a look-and-sort declaration: what "put any
number of them" reads back. -/
private def sliceReadBack : Action → Option Plurality
  | .ownScope _ (.sequentially [_, .move (.someOf _ _ readBack@(.laterMention _ _)) _ _ _ _, _]) =>
      some readBack.plur
  | _ => none

/-- "Scry 1." -/
theorem okScryOne : Action.check (DRS.ofList []) (scry 1) = [] := by decide

/-- "Scry 2." -/
theorem okScryTwo : Action.check (DRS.ofList []) (scry 2) = [] := by decide

/-- Scry 1's read back is singular: "that card". -/
theorem scryOneReadsThatCard : sliceReadBack (scry 1) = some .one := by decide

/-- Scry 2's read back is plural: "those cards". -/
theorem scryTwoReadsThoseCards : sliceReadBack (scry 2) = some .many := by decide

/-- "Surveil 2." [CR#701.25a] -/
theorem okSurveilTwo : Action.check (DRS.ofList []) (surveil 2) = [] := by decide

/-- "Fateseal 2." [CR#701.29a]: an opponent's library. -/
theorem okFatesealTwo : Action.check (DRS.ofList []) (fateseal 2) = [] := by decide

```

Retired here:

- `scryBy`, and the inline surveil and fateseal bodies: replicas of `scry`/`surveil`/`fateseal`
  (`Definitions/Actions.lean:237,248,243`). The fateseal replica read the controller's opponent,
  where production reads the performer's [CR#701.29a].
- The whole `## Connive` section (`conniveDiscards`, `conniveBy`, `okConniveOne`,
  `okConniveTwoHandedToTarget`, `conniveReadsTheDiscardsNumber`):
  - `conniveDiscards` and `conniveBy` are replicas of `connive` (`Definitions/Actions.lean:418`).
  - Pointed at production, the two `ok` pins are exactly
    `Proofs/DefinitionsAndTemplates.lean:27-31` (`okConniveOne`, `okConniveTarget`).
  - `conniveReadsTheDiscardsNumber` asserts the plurality of a toy `laterMention "d"`, which
    production connive does not use: it loops over "each nonland card discarded this way".

Keep the private `lookAndSortSlice` alias at :163-167; C3 retires the theorem after it.

### C2.3 `Proofs/GiftVariants.lean`

Delete lines 22-51: `giftCost`, `handed`, `costPart`, the local `gift` shadow, `giftACard`,
`giftATappedFish`, and the blank line after them. The local `gift` is production `gift`
(`Definitions/Keywords.lean:890`) without the `.inCaller` wrapping. At :92, change
`triggerCondition? (Semantics.Definitions.gift (draw 1))` to `triggerCondition? (gift (draw 1))`.
`okVariantsCheck`, `okGiftACardOnBothClasses` and `okGiftATappedFishOnBothClasses` are otherwise
unchanged, and they now read production `giftACard` and `giftATappedFish`.

### C2.4 `Proofs/LinkedChoiceReadback.lean`

This file has the same defect, though the old ticket did not name it.

- Rename the local `def gift : LinkSource` (:21) to `giftCost`. Its uses at :33 and :39 become
  `costWasPaid giftCost none .this`. Its docstring "its gift cost" stays.
- Delete `giftBody` (:79-82: docstring, def and blank line). It is a replica of gift's
  declaration.
- In `okPermanentClassReadsChosenPlayer` (:84-90) and `okSpellClassReadsChosenPlayer` (:93-99),
  replace `text := [giftBody]` with `text := [giftACard]`. Change their docstrings to:
  - :83 `/-- Gift's declaration (`giftACard`), its cost and both halves, is carried on a permanent card. -/`
  - :92 `/-- Gift's declaration (`giftACard`), its cost and both halves, is carried on an instant card. -/`
- `costPart`, `permanentHalf`, `spellHalf` and `chosenDraws` stay. They are fixtures for the
  ill-formed texts in `badTwoChosenPlayers` and `keywordBodyThreadsItsCostChoice`.

### C2.5 `Proofs/Actor.lean`, the explore and endure replicas

`exploreBody`, `literalExplore` and `literalEndure` (:668-691) read the permanent as a "that
permanent" anaphor. Production `explore` (`Definitions/Actions.lean:404`) and `endure` (`:661`)
bind the performer at its first mention and read it back by identity.

- Replace the section docstring at :659-666 with:

```lean
/-! ## A permanent can be handed an action

The rules instruct a permanent itself to explore or endure [CR#701.44a,701.63a]; the handoff
takes a permanent as well as a player. Inside it `theActor` is the permanent. The `explore` and
`endure` Definitions bind it at their first mention of the performer, hand the steps the rule
gives "that permanent's controller" on to `controllerOf` it, and read the permanent back by that
identity, not by a "that permanent" anaphor. An object-performed deed written with no handoff to
a permanent is performed by the source permanent. -/
```

- Delete :668-691 (`exploreBody`, `literalExplore`, `literalEndure` with their docstrings).
- `okExploreHandedToTargetCreature`: `literalExplore` → `explore`.
- `okExploreHandedToThisCreature`: `literalExplore` → `explore`. Its docstring becomes
  `/-- Deadeye Tracker's "This creature explores.": `this` introduces nothing; the handoff names
  the permanent the body binds. -/`
- Retire `okExploreReadsThatCreature` (:704-709). Production explore takes no spelling of the
  permanent, so there is no "that creature" variant to read.
- `okEndureHandedToThisCreature`: `(literalEndure 1)` → `(endure 1)`.
- `okExploreHandedToEachCreature`: `literalExplore` → `explore`.
- Retire `badExploreHandedToOpponent` (:730-735). Its extra `.anaphor (.word .permanent) .one 0`
  belonged to the replica. Production behaviour is pinned by `badAuthoredExploreHandedToOpponent`
  (:784-787).
- `okBareExploreAndEndureDefaultToThis` (:749): the statement becomes
  `Action.check (DRS.ofList []) explore = [] ∧ Action.check (DRS.ofList []) (endure 1) = []`.
  Its docstring becomes `/-- A bare "explore" and "endure 1" default to the source permanent the
  same way: `theActor` in the body is "this permanent", which the body binds and reads back. -/`

### C2.6 `Check.lean` `CorePins` → `Proofs/CheckerCore.lean`

- In `lean/Semantics/Check.lean`:
  - delete line 11 (`import Semantics.Definitions.Subtypes`). Only `CorePins` used it.
  - delete lines 19-289: the blank line, `namespace Semantics.Check.CorePins` through
    `end Semantics.Check.CorePins`.
  - replace the module docstring with:

```lean
/-!
# Semantics.Check

Root entry point for the Clean-Sheet DRT Checker (`docs/decisions/semantics-v3.md` §1, §3). Its
core pins are `Semantics.Proofs.CheckerCore`.
-/
```

- Create `lean/Semantics/Proofs/CheckerCore.lean` with exactly the content below.
- In `lean/Semantics/Proofs.lean`, insert `import Semantics.Proofs.CheckerCore` between
  `CharacteristicEdits` and `Choice`.
- This commit adds the citation `[CR#701.10e]` (slice B), so run `cargo xtask cite bless`.

```lean
import Semantics.Check
import Semantics.Definitions

/-!
# Semantics.Proofs.CheckerCore

The DRT checker's core pins, rehomed from `Semantics.Check`: a Definition's parameter handed off
and read back by identity, identity cleared by a zone change, reflexive and clause-mate pronoun
resolution, pre-event (`would`) stepping, the activated-ability colon, event-head gating, and one
composite pin per checker slice. Each pin is closed by `decide`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.CheckerCore

private def targetCreature : NounPhrase := target isCreature

private def anotherTargetCreature : NounPhrase :=
  .described (.target (.range (some 1) (some 1))) (.and [.other, .hasType .creature])

private def aCreature : NounPhrase := a isCreature

private def handZone : Location :=
  .zone .hand (some .you)

private def graveyardZone : Location :=
  .zone .graveyard (some .you)

/-- Checker fixture, not a keyword action and not `explore`: a Definition whose body hands off to
its own parameter, bound at its first mention and read back by identity across the handoff and a
`doIf` (`ownScope` + `inCaller` + `firstMention` / `laterMention` + `doIf` + `putCounters`).
`explore` hands off to its performer's controller instead [CR#701.44a]. -/
def revealThenCountDef (subject : NounPhrase) : Action :=
  .ownScope (some (.action "revealThenCount")) <|
    .instruct (.firstMention "subject" (.inCaller subject)) <|
      .sequentially [
        .expose .reveal (.cards (.slice .top (.lit 1) (.zone .library (some .you)))),
        .doIf
          (.matches (.pro .bare .one) (.hasType .land))
          (.move (.pro .bare .one) .anyLocation (some handZone) .departure [])
          (some (.putCounters (.lit 1) (.printed (.named "p1p1Counter"))
            (.laterMention "subject" (.inCaller subject))))
      ]

theorem reveal_then_count_def_sound :
    (revealThenCountDef targetCreature).check (DRS.ofList []) = [] := by decide

/-- Checker fixture, not a keyword action and not `connive`: a Definition whose body hands off to
its own parameter and on to that object's controller, and reads the parameter back by identity
after a draw, a move and a `did … thisWay` condition (`ownScope` + `inCaller` + `firstMention` /
`laterMention` + `draw` + `move` + `doIf`). `connive` hands off to its performer's controller
instead [CR#701.50a]. -/
def drawDiscardCountDef (subject : NounPhrase) : Action :=
  .ownScope (some (.action "drawDiscardCount")) <|
    .instruct (.firstMention "subject" (.inCaller subject)) <|
      .instruct (.possessorOf .controller .actor) <|
      .sequentially [
        .draw (.exactlyOf (.lit 1)),
        .move (.described (.a .unmarked) (.and [.isCard, .in_ handZone]))
          (handZone) (some graveyardZone) .departure [],
        .doIf
          (.did { subj := .actor .you
                  action := .move (.described (.a .unmarked) (.not (.hasType .land)))
                    (handZone) (some graveyardZone) .departure []
                  within := some .thisWay })
          (.putCounters (.lit 1) (.printed (.named "p1p1Counter"))
            (.laterMention "subject" (.inCaller subject)))
          none
      ]

theorem draw_discard_count_def_sound :
    (drawDiscardCountDef targetCreature).check (DRS.ofList []) = [] := by decide

/-- Zone-change identity clearing [CR#400.7]: a later mention of a parameter first mentioned
before the move that made it a new object is refused with `Refusal.bindingIdentity`. A checker
fixture: its header names no keyword action. -/
theorem post_move_later_mention_refused :
    (Action.ownScope (some (.action "exileThenTap")) <|
      .sequentially
        [ exile (.firstMention "subject" (.inCaller targetCreature)),
          tap (.laterMention "subject" (.inCaller targetCreature)) ]).check (DRS.ofList []) =
      [.bindingIdentity "subject"] := by decide

/-- Principle A (`NounPhrase.refl`) vs. `J2` (clause-mate exclusion):
`"Target creature deals 2 damage to itself"` succeeds, whereas `"Target creature deals 2 damage to it"`
is refused because the only referent in scope is a clause-mate. -/
theorem reflexive_subject_succeeds :
    (Action.instruct targetCreature
      (.dealDamage (.lit 2) (.refl .one))).check (DRS.ofList []) = [] := by decide

theorem clause_mate_pronoun_refused :
    (Action.instruct targetCreature
      (.dealDamage (.lit 2) (.pro .bare .one))).check (DRS.ofList []) =
      [.anaphor .bare .one 0] := by decide

/-- Re-mentioned pronoun subject in clause 2 (`"Tap target creature. It deals 2 damage to itself"`):
records `(r.id, .subj)` in `drs.pendingRoles`, licensing `"itself"` and excluding `"it"` in the
object slot. -/
theorem rementioned_subject_licenses_reflexive :
    (Action.sequentially [
      .setStatus .tapped targetCreature,
      .instruct (.pro .bare .one) (.dealDamage (.lit 2) (.refl .one))
    ]).check (DRS.ofList []) = [] := by decide

theorem rementioned_subject_excludes_clause_mate_pronoun :
    (Action.sequentially [
      .setStatus .tapped targetCreature,
      .instruct (.pro .bare .one) (.dealDamage (.lit 2) (.pro .bare .one))
    ]).check (DRS.ofList []) = [.anaphor .bare .one 0] := by decide

/-- Rule `D′` + Rule `AC>R1` complementary split in the same clause:
Clause 0 introduces subject (`targetCreature`) and object (`anotherTargetCreature`).
Clause 1 resolves `"it"` (`.pro .bare .one`) to the subject via `D′` (setting `pronounResolved := true`),
and resolves `"that creature"` (`.pro (.word (.type .creature)) .one`) to the remaining creature via `AC>R1`. -/
theorem pronoun_and_demonstrative_split :
    (Action.sequentially [
      .instruct targetCreature (.dealDamage (.lit 1) anotherTargetCreature),
      .instruct (.pro .bare .one)
        (.dealDamage (.lit 2) (.pro (.word (.type .creature)) .one))
    ]).check (DRS.ofList []) = [] := by decide

/-- Pre-event (`would`) stepping (*Kalitas* replacement):
`"If a nontoken creature an opponent controls would die, instead exile that creature and create a 2/2 black Zombie creature token."` -/
def kalitasReplacement : Ability :=
  static
    (replace
      (would (a (and [non isToken, isCreature, anOpponentControls])) die)
      (instead (sequentially
        [ exile (that creature),
          create 1 (creatureToken 2 2 [.black] [zombie]) ])))

theorem kalitas_replacement_sound :
    kalitasReplacement.check = [] := by decide

/-- Activated `cost : effect` box across the colon:
`"Sacrifice a creature: Draw cards equal to its power."` -/
def sacrificeDrawActivated : Ability :=
  activated (.perform (sacrifice aCreature)) (draw (powerOf it))

theorem sacrifice_draw_activated_sound :
    sacrificeDrawActivated.check = [] := by decide

/-- Closed event and deontic head-gating (`semantics-v3.md` §3.6–3.7):
compound actions (`sequentially`) are refused inside `does`, `would`, `did`, and `cant` with
`Refusal.eventHead`, while a Definition's `ownScope` header is admitted. -/
theorem compound_event_head_refused :
    (GameEvent.does (.actor .you) (.sequentially [.draw (.exactlyOf (.lit 1)), .draw (.exactlyOf (.lit 1))])).check
      (DRS.ofList []) = [.eventHead] := by decide

theorem own_scope_action_event_head_admitted :
    (GameEvent.does (.actor aCreature) (revealThenCountDef aCreature)).check
      (DRS.ofList []) = [] := by decide

/-- Slice A pin (*Weeping Angel*-style combat trigger + attachment/status):
`"Whenever a creature attacks you, tap it and attach target Equipment to that creature."` -/
def weepingAngelCombatPin : Ability :=
  whenever (does aCreature (attack (some you)))
    (sequentially
      [ tap it,
        attachTo (target (hasSubtype equipment)) (that creature) ])

theorem weeping_angel_combat_pin_sound :
    weepingAngelCombatPin.check = [] := by decide

/-- Slice B pin (counters + characteristic modification + pronoun/demonstrative):
`"Double the number of counters on target creature, then it becomes a Dragon in addition to its other types until end of turn."`
[CR#701.10e] -/
def sliceBCountersAndCharPin : Ability :=
  spell <|
    sequentially
      [ doubleCounters targetCreature,
        establish
          (.characteristicChange it [.typeLine .adds { subtypes := some [dragon] }])
          untilEndOfTurn ]

theorem slice_b_counters_and_char_pin_sound :
    sliceBCountersAndCharPin.check = [] := by decide

/-- Slice D pin (vote outcome + quality choice + `ofChosen` resolution):
`"Choose a color. Starting with you, each player votes for grace or condemnation. Draw a card for each grace vote. Return all creatures of the chosen color to their owners' hands."` -/
def sliceDVoteAndChoicePin : Ability :=
  spell <|
    sequentially
      [ choose (a (qualityNoun .color)),
        instruct eachPlayer (voteStartingWith you .openly (byLabel ["grace", "condemnation"])),
        draw (votesFor "grace"),
        returnToHand (allOf (and [isCreature, ofChosen .color])) ]

theorem slice_d_vote_and_choice_pin_sound :
    sliceDVoteAndChoicePin.check = [] := by decide

/-- Slice E pin (copying + `chooseNewTargets` + damage prevention + `theOutcome .damagePrevented`):
`"Copy target instant or sorcery spell. You may choose new targets for the copy. Prevent the next 3 damage that would be dealt to any target this turn, then draw cards equal to the damage prevented this way."` -/
def sliceECopyAndPreventPin : Ability :=
  spell <|
    sequentially
      [ copyOnStack (target (or [isInstant, isSorcery])),
        may (chooseNewTargets (that .copy)),
        establish
          (.replace (.would .any (.dealDamage .anyAmount .any none))
            (.instead (.preventDamage (.shield 3) none (some (draw preventedThisWay))))
            none .repeatedly none)
          untilEndOfTurn ]

theorem slice_e_copy_and_prevent_pin_sound :
    sliceECopyAndPreventPin.check = [] := by decide

end Semantics.Proofs.CheckerCore
```

What changed against the old `CorePins`, for the commit description:

- `sacrificeDrawActivated` and `sliceBCountersAndCharPin` now read production `sacrifice` and
  `doubleCounters` instead of replicas.
- Kalitas, slices A, D and E are written through Definitions and Templates (`die`, `exile`,
  `create`, `tap`, `attachTo`, `voteStartingWith`, `returnToHand`, `copyOnStack`, `may`) instead
  of raw constructors. Slice D's docstring gains its omitted last sentence. Slice E's prevention
  shield stays raw because `preventNext` takes no rider.
- The private `exileZone` is dropped as unused.
- `postMoveLaterMentionRefused` keeps its refusal, but it is now written through `exile`/`tap`
  under a fixture deed. Its old header falsely said "airbend".
- Retired:
  - `airbendDef` and `airbend_def_sound`: a replica of a superseded airbend body. Production
    `airbend` is pinned at `Proofs/DefinitionsAndTemplates.lean:21-25`, `Proofs/DrtCards.lean:44-49`
    and `Proofs/BindingIdentity.lean:286-292`.
  - `sliceCFactOrFictionPin`: a duplicate of the bench card `factOrFiction` (`Cards/Anaphora.lean:699`),
    which `spelled` checks.

---

## C3. Retire `rfl` restatements

A theorem is retired when its statement is `Template args = <that Template's body>`. "Rewrite"
means replace the theorem, keeping its name unless a new one is given.

### C3.1 Retire (each restates its own definition)

- `Proofs/ActionForms.lean`: `exileOmittedAgent` (7-11), `exileNamedAgent` (13-18),
  `chooseOmittedAgent` (20-21), `chooseNamedAgent` (23-25) and `chooseSecretlyNamedAgent` (27-29).
  `millNamedAgent` is rewritten in C3.2.
- `Proofs/ControllerSacrifice.lean`:
  - `controllerSacrificesExpansionForEverySubject` (132-136): verbatim `Definitions/Actions.lean:542-545`.
  - `expansionIntroducesTheTargetOnce` (17-21, including the `--` comment on 17): a duplicate of
    the same restatement.
- `Proofs/BindingIdentity.lean`: `lookAndSortSlice_is_the_hand_expansion` (169-178, docstring
  included). It is verbatim `Templates.lean:1396-1403`. Keep the private alias at 163-167.
- `Proofs/GetsBothDeltas.lean`: `getsPtIsOneCharacteristicChange` (19-26). `getsPtSlots` (:61) and
  `battleCryGetsBothPrintedDeltas` (:41) carry the semantics.
- `Proofs/Rules.lean`: the block at 484-488, the shared docstring plus `combatantIsTheRole` and
  `playableFromIsThePlayPermission`. Both helpers are used by rows that already pass
  `Definition.check`.
- `Proofs/Trigger.lean`: `drawGameConcludes` (234-235).
- `Proofs/SpellCopies.lean`: `targetsOnlyIsTargetsAtOnly` (31-34). `okSpellTargetsOnly` and
  `badPlayerTargetsOnly` (:20, :26) carry the semantics.
- `Proofs/Authoring.lean`: `authoredCardStillPassesSemanticCheck` (:34) projects `Spelled.ok`,
  which every `Spelled` has. Its three siblings are rewritten in C3.2.
- Whole-body restatements the old ticket did not name:
  - `Proofs/ActionFamilies.lean:219-223` `counterIsOneScopedMove`
  - `Proofs/Anaphora.lean:975-979` `dealDamageOwnPowerIsThePronounForm` (`Damage.lean:826,845` cover the resolution)
  - `Proofs/Attachments.lean:94-95` `isTransformedTemplateReadsWholeBackFace`
  - `Proofs/ContrastAnchor.lean:77-79` `anyOtherTargetIsOneEncoding`
  - `Proofs/Counters.lean:677-679` `okGreaterOf`
  - `Proofs/DefinitionsAndTemplates.lean:172-175` `searchLibraryOrGraveyardIsItsLocationSearch` and `:177-178` `unqualifiedSearchIsForAny`
  - `Proofs/Deontic.lean:85-90` `payLifeIsAPayment` (`:108` covers the coercions)
  - `Proofs/Description.lean:455-457` `coloredManaSpentIsPaidColorsSpent`
  - `Proofs/EventPatterns.lean:277-281` `gainControlImperativeIsTheControlGrant` and `:376-383` `effectBlockedAndTriggerEventShareHead`
  - `Proofs/FaceDown.lean:53-56` `foretoldCastOnALaterTurn`
  - `Proofs/KeywordByCardClass.lean:89-98` `ascendTree` (`:41,48,55` check production `ascend`)
  - `Proofs/Static.lean:198-206` `entersTappedUnlessConditionsTheReplacement`, `:228-232` `entersTappedRidesTheEntering` and `:302-307` `unlessIsIfNot`
  - `Proofs/TextChanges.lean:36-38` `bracketedCoreIsContents` (`:72` `bracketedCardCoreIsUnbracketed` is the real pin)
  - `Proofs/Trigger.lean:194-198` `triggersIsTheAbilityTriggering` and `:206-211` `dayAndNightAreTheGamesDesignations`
  - `Proofs/UntapStep.lean:57-60` `untapDuringIsMust`

### C3.2 Rewrite

- `Proofs/ActionForms.lean` `millNamedAgent` (docstring 31-32, theorem 33-38). The file keeps its
  import in `Proofs.lean`. Replace it with:

```lean
/-- A player mills from their own library [CR#701.17a]: the library is the performer's, so a
handoff to a player checks and a handoff to an object, which has no library, is refused. -/
theorem millReadsThePerformersLibrary :
    Action.check (DRS.ofList []) (instruct .you (mill 3)) = [] ∧
      Action.check (DRS.ofList []) (instruct (target isCreature) (mill 3)) =
        [.kindMismatch .player .object] := by
  decide
```

- `Proofs/Authoring.lean`: delete lines 26-34. That is `printedDataNeedsNoExpressionTemplate`,
  `keywordRecordsDefinitionAuthorship`, `templateDefaultAgentRetainsAuthorship` and the retired
  `authoredCardStillPassesSemanticCheck`. Insert the following right after the `formCalls` mutual
  block (it ends at :312; `formCalls` is private and defined there):

```lean
/-- Printed data alone is authored with no expression Template: its Form names only the mana
cost, the card type and the printed power and toughness. -/
theorem printedDataNeedsNoExpressionTemplate :
    formCalls plain.authoring =
      [``Templates.generic, ``CardType.creature, ``Templates.stat, ``Templates.stat] := by
  decide

/-- A keyword written by its Definition is recorded as that Definition's call. -/
theorem keywordRecordsDefinitionAuthorship :
    (formCalls withKeyword.authoring).contains ``Definitions.reach = true := by decide

/-- A Template whose agent is left to its default is still recorded as the Template's call. -/
theorem templateDefaultAgentRetainsAuthorship :
    (formCalls withDraw.authoring).contains ``Templates.draw = true := by decide
```

- `Proofs/Attachments.lean:123-128` becomes:

```lean
theorem passiveAgentAndAttachmentHostAreDistinct :
    (.by (allOf isCreature) : Subject).patientRole = .subj := rfl
```

  Keep its docstring. `okEquipmentMayBeAttachedToACreature` (:116) checks the host half.
- `Proofs/Counters.lean:754-762` and `:768-776`: drop the first (`rfl`) conjunct of each theorem,
  keeping the remaining conjuncts and the docstrings exactly as written. Each proof becomes
  `by decide`.
- `Proofs/Deontic.lean:379-382` `printedThisTurnProhibitionLastsTheTurn` (cited by
  `lean/CONTRACTS.md:1323`; the name stays):

```lean
theorem printedThisTurnProhibitionLastsTheTurn :
    let duration : Action → Option (Option Duration) := fun
      | .sequentially [_, .establish _ d] => some d
      | _ => none
    duration (doAndForbid (instruct .this (.dealDamage 3 (target isCreature))) (regeneration it)) =
        some (some untilEndOfTurn) ∧
      duration (doAndForbid (instruct .this (.dealDamage 3 (target isCreature))) (regeneration it)) ≠
        duration (destroyCantRegenerate (target isCreature)) :=
  ⟨rfl, by intro h; cases h⟩
```

- `Proofs/Faces.lean:770-780` `stormCountsEarlierThisTurn`:

```lean
theorem stormCountsEarlierThisTurn :
    (match storm with
      | .keyword _ _ [.triggered _ _ _ _ _ _ _ (.sequentially (.copy _ _ count _ :: _))] => some count
      | _ => none) =
      some (eventCount (cast (a (.and [isSpell, isOther]))) (.actor (a .isPlayer)) earlierThisTurn) :=
  rfl
```

- `Proofs/KeywordHolders.lean:106` `convertIsTransform` (cited by `lean/CONTRACTS.md:1347`; the
  name stays):

```lean
theorem convertIsTransform (subject : NounPhrase) :
    (convert subject).namedDeed = (transform subject).namedDeed := rfl
```

- `Proofs/Rules.lean:95-102` `toxicIsOneResultReadingTheTotal` (keep the docstring at 93-94):

```lean
theorem toxicIsOneResultReadingTheTotal :
    (match toxic 2 with
      | .keyword (.named "toxic") [.number _] body => body.isEmpty
      | _ => false) = true := rfl
```

- `Proofs/Rules.lean:460-464` `sagaLoreIncrementIsTurnBased`, dropping the second conjunct:

```lean
theorem sagaLoreIncrementIsTurnBased :
    ∃ parts, sagaDefinition = .subtype Semantics.Definitions.saga parts ∧
      parts[1]? = some (.turnBased .firstMain (doIf sagaHasChapterAbilities sagaLoreIncrement)) :=
  ⟨_, rfl, rfl⟩
```

- `Proofs/Rules.lean:479-482` `okSiegeGuard`. Delete "and is the guard the Siege state-based
  action reads" from its docstring (:476-478). The theorem becomes:

```lean
theorem okSiegeGuard : Condition.check (DRS.ofList []) siegeHasNoWaitingTrigger = [] := by decide
```

- `Proofs/Trigger.lean:271-274` `okAfterRollShift` (cited by `lean/CONTRACTS.md:1501`; the name
  and docstring stay):

```lean
theorem okAfterRollShift :
    (match after youRollADie rollShiftBody with
      | .static (.rollModifier ..) => true
      | _ => false) = true ∧
    Ability.check (DRS.ofList []) (after youRollADie rollShiftBody) = [] :=
  ⟨rfl, by decide⟩
```

- `Proofs/Turn.lean:594-596` `sorcerySpeedRequiresPriority`:

```lean
theorem sorcerySpeedRequiresPriority :
    ∃ rest, asSorcery = .and (.onlyIf (.hasPriority .you) :: rest) := ⟨_, rfl⟩
```

- `Proofs/Turn.lean:697-700` `flashSacrificeRidesThePermission`:

```lean
theorem flashSacrificeRidesThePermission :
    ∃ subject action mods, asThoughFlashSacrificedAtCleanup = .static (.can subject action mods) ∧
      mods.rider = some sacrificedAtCleanup := ⟨_, _, _, rfl, rfl⟩
```

- `Proofs/UntapStep.lean`:
  - :34-37 becomes `theorem doesntUntapForbidsTheUntapDeed (subject : NounPhrase) : (untap subject).namedDeed = some (.action "untap") := rfl`
  - :50-55 `doesntUntapIsNoStunReplacement` (cited by `docs/engine-adrs.md:95`; the name stays):
    drop the first conjunct; the proof becomes `⟨rfl, by decide⟩`.
  - :73-79 `mayDeclineUntapIsChosenProhibition`: keep only the `≠` conjunct; the proof becomes
    `by intro h; injection h with h; cases h`.
- `Proofs/KeywordAbilityCensus.lean:220-228` `okEchoDefinition`: drop the third conjunct
  (`thisPermanent = .headed .permanent .this`); the proof becomes `⟨rfl, rfl⟩`.
- `Proofs/Actor.lean:309-322` `killingWaveHandoffTwin`: drop the second conjunct, which is the
  vacuous `X = X`. If that leaves a single conjunct, prove it with `by decide`.

### C3.3 Keep (state in the commit description that these were audited and kept)

- Cross-spelling convergence: DefinitionsAndTemplates.lean:117, 126, 135, 143; Deontic.lean:108;
  GetsBothDeltas.lean:181; Damage.lean:145.
- Computations over terms: Numbers.lean, ReplacementArms.lean, CallerScope.lean:208, 431,
  Attachments.lean:24, 58, 110, NounWords.lean, Lookback.lean:136, 141, 161, EventPatterns.lean:39,
  50, 63, Cards/Cost.lean:735, Cards/Piles.lean:41, Rules.lean:474, and the private step lemmas in
  ComparisonBox and ControllerSacrifice.
- Sub-term extraction pins: KeywordCastPermissions.lean, OffStack.lean, Keyword.lean:217, 464,
  505-541, Damage.lean:52, 68, 1066-1167, Retention.lean, Mana.lean:385, GiftVariants.lean:91,
  KeywordHolders.lean:32-89, DuringResolution.lean, Rules.lean:106, Turn.lean:645, 656, 704,
  Trigger.lean:331, EventPatterns.lean:322, 365, 371.
- Elaborator and coercion pins: Authoring.lean:41, 285, 350-395, 454, 492, 496;
  Counters.lean:674; CallerScope.lean:186, 190; TextChanges.lean:72.
- Argument-order pins: Trigger.lean:309, Counters.lean:183.
- Card-fidelity anchors (Decision 4): CardFidelity.lean:37, 58, 83, 96, 211; Damage.lean:575, 712,
  826, 845, 860, 1248; Deontic.lean:289.

---

## C4. Structural referent pins

These rewrites use the existing API and add no helper: `headerFreshReferents before after`
(`Check/HeaderContext.lean:12`, the referents a step introduced), `DRS.findById?`, `firstReach`
and `resolveCandidates`.

### C4.1 `Proofs/ControllerSacrifice.lean`

- :51-55 `nestedSubjectPublishesExactlyItsFourParticipants`: drop the id from the projected
  tuple. The projection becomes `(fun ref => (ref.kind, ref.zone, ref.ty))` and the expected list
  `[(.player, none, []), (.object, some .battlefield, [.creature]), (.player, none, []), (.object, some .battlefield, [.artifact])]`.
- :111-118 `existingReferenceIsNotIntroducedAgain`: replace the last two conjuncts
  (`… .map (·.id) = some 2 ∧ (after.findById? 2).bind (·.zone) = some .graveyard`) with
  `(firstReach (.word .card) .one (nextClause after)).map (fun ref => (ref.movedFrom, ref.zone)) = some ([0], some .graveyard)`.
  The `0` is the fixture's own input id.
- :123-130 `riderReadsTheStampedPatient`: `ids = [2]` →
  `ids = (afterSacrifice.referents.filter (·.det == .target)).map (·.id)`.
- :138-144 `repeatedControllerSacrificesKeepDistinctPatients`, statement body:

```lean
    let (after, errors) := stepAction 64 .postEvent .obj (DRS.ofList [])
      (.sequentially [action, action])
    let patients := after.referents.filter (fun ref => ref.kind == .object)
    errors = [] ∧
      patients.map (fun ref => (ref.zone, ref.idents)) =
        [(some .graveyard, []), (some .graveyard, [])] ∧
      (patients.map (·.id)).Nodup := by decide
```

- :151-154 `laterPlayerIsTheController`: the projection becomes
  `(fun ref => (ref.det, ref.plur, ref.payload, ref.role))` and the expected value
  `some (.the, .one, .player false, .subj)`.
- :156-162 `laterCardIsTheSacrificedObject`: drop `ref.id` from the projection and `2` from the
  expected tuple; the rest is unchanged.
- :164-170 `outerArtifactDoesNotStealThePatient`: `… .map (·.id) = some 3` →
  `… .map (·.id) = ((headerFreshReferents outer after).filter (·.kind == .object)).head?.map (·.id)`.
- :945-957 `nestedDescriptionMovesOnlyItsOwnReferent`. Rename the current theorem to
  `private theorem nestedDescriptionReferents`, with its signature and proof unchanged. Then add
  after it:

```lean
/-- The complete action moves only its selected new creature and preserves every outer
referent, unrelated named referents included: of its four participants only the creature target
moves, to the graveyard, and the creature it was before is gone; the artifact target stays on
the battlefield. -/
theorem nestedDescriptionMovesOnlyItsOwnReferent (rs : List Referent) (fresh : ReferentId)
    (older : ∀ ref ∈ rs, ref.id < fresh)
    (nameFresh : ∀ ref ∈ rs, ref.idents.elem "controllerSacrifices.subject" = false) :
    let after := (stepAction 64 .postEvent .obj { referents := rs, nextId := fresh }
      nestedAction).1
    let published := after.referents.take 4
    after.referents.drop 4 = rs ∧
      published.map (fun ref => (ref.kind, ref.det, ref.zone, ref.ty)) =
        [(.player, .the, none, []), (.object, .target, some .graveyard, [.creature]),
          (.player, .the, none, []), (.object, .target, some .battlefield, [.artifact])] ∧
      (published.filter (·.zone == some .graveyard)).all (fun moved =>
        moved.movedFrom.all (fun id => (after.referents.find? (·.id == id)).isNone)) := by
  intro after published
  have h := nestedDescriptionReferents rs fresh older nameFresh
  simp only [after, published, h, afterPrefix]
  refine ⟨rfl, rfl, ?_⟩
  have absent : ∀ ref ∈ rs, ref.id ≠ fresh + 2 := fun ref mem => by
    exact Nat.ne_of_lt (Nat.lt_of_lt_of_le (older ref mem) (Nat.le_add_right _ _))
  simp
  exact Or.inr absent
```

### C4.2 Other files

- `Proofs/Actor.lean:850` `amassBareItSelectsChosenArmy`: `selected.map (·.id) = [after.nextId - 1]` →
  `selected.map (·.id) = (after.referents.filter (·.chosenObject)).map (·.id)`.
- `Proofs/Anaphora.lean:1056` and `:1063` (`singularSpellReadHasCopyIdentity`,
  `singularAbilityReadHasCopyIdentity`): `[after.nextId - 1]` →
  `(after.referents.filter (·.payload.orig == some .copy)).map (·.id)`.
- `Proofs/Anaphora.lean:1065-1069` `chosenPlayerDemonstrativeHasNewMentionIdentity`, body:

```lean
    let selected := resolveCandidates (.word .player) .one (nextClause after)
    selected.length = 1 ∧
      selected.map (·.id) =
        ((headerFreshReferents before after).filter (·.kind == .player)).map (·.id) := by decide
```

- `Proofs/CorpusRecency.lean:193`: `[scytheAfterReturn.nextId - 1]` →
  `((headerFreshReferents (nextClause scytheAfterDeath) scytheAfterReturn).filter (·.kind == .object)).map (·.id)`.
- `Proofs/CorpusRecency.lean:241`: `[cycloneAfter.nextId - 1]` →
  `((headerFreshReferents (nextClause cycledContext) cycloneAfter).filter (·.zone == some .exile)).map (·.id)`.
- `Proofs/ReferenceScopes.lean:265`: `[after.nextId - 1]` → `(headerFreshReferents before after).map (·.id)`.
- `Proofs/Turn.lean:174-177` `bareCardReadSelectsExiledTarget`:

```lean
theorem bareCardReadSelectsExiledTarget :
    let afterCost := (stepCost 64 (DRS.ofList []) (.perform (sacrifice (a isCreature)))).1
    (firstReach (.word .card) .one bareCardReadContext).map (·.id) =
      ((headerFreshReferents (nextClause afterCost) bareCardReadContext).filter
        (·.zone == some .exile)).head?.map (·.id) ∧
      (firstReach (.word .card) .one bareCardReadContext).bind (·.zone) = some .exile := by decide
```

- `Proofs/ActionFamilies.lean:66-67`: `(after.findById? before.nextId).map (·.payload) = some (.object …)` →
  `((headerFreshReferents before after).filter (·.kind == .object)).map (·.payload) = [.object [.artifact] (some .battlefield) none (some .token) none]`.
- `Proofs/Description.lean:293-298` `okSuperlativeDescribesAfresh`, body:

```lean
    let (out, ids, errors) := stepNounPhrase 64 chosenFightPair .other false
        (the (and [isCreature, superlative .max (.stat .power) isCreature]))
    errors = [] ∧ ids.length = 1 ∧ ids = (headerFreshReferents chosenFightPair out).map (·.id) := by
  decide
```

- `Proofs/EventPatterns.lean:238` `common.nextId = 2` → `common.nextId = alternative.nextId`, and
  `:269` `common.nextId = 3` → `common.nextId = alternative.nextId`. The claim is that the common
  context reserves the branch's ids.
- `Proofs/HistoricalMove.lean:34` `out.nextId = 8` → `out.nextId = context.nextId`: nothing is
  minted. The `findById? 7` there reads a fixture id and stays.

### C4.3 Not changed (state this in the commit description)

- `ControllerSacrifice.lean`: the 62 lines of `fresh + k` / `drs.nextId + k` inside the private
  lemmas and defs at 199-943 (namespaces `ControllerSuffixLocality`, `MoveActors`). They compute
  the ∀-fresh proof, and `DRS.prepend` allocates `nextId + i`. The `.drop 4` at :46 is the claim.
- Pins whose claim is id allocation itself: MovementScopes.lean:15, 93, 98, 104; Lookback.lean:728;
  ReferenceScopes.lean:256.
- Comparisons to `nextId` with no offset: CallerScope.lean:171, 428; Counters.lean:54, 68;
  Description.lean:258; MovementScopes.lean:108; Cards/Cost.lean:740.
- `CallerScope.lean:51` `⟨999⟩` and `:281` `⟨1⟩` are `AuthoringContext` values, not ids.
- The remaining literal-id assertions (`ids = [n]`, `findById? n`) belong to the C9 follow-up.

---

## C5. Exact refusal lists and constructor coverage

Every list below is what the checker returns today, and each refusal in it is endorsed.

### C5.1 Weak negatives become exact lists

- `Proofs/AbilityQualifiers.lean:66-68`:

```lean
theorem badSpeedOfAnObject :
    Condition.check (DRS.ofList []) (.compareAmt (speedOf thisCreature) .eq 4) =
      [.kindMismatch .player .object] := by
  decide
```

- `Proofs/KeywordAbilityCensus.lean:99-101`. Production `destroy` is a battlefield-to-graveyard move
  [CR#701.8a], and its patient is bounded to an object or a pile:

```lean
theorem badDestroyedPlayer :
    StaticSpec.check (DRS.ofList []) (cant any (destroy .you)) =
      [.kindLte .player (.join .object .pile)] := by
  decide
```

- `Proofs/CallerScope.lean:160`:

```lean
theorem amountTemplatesRejectMissingSubjects :
    (subjectPower it).check (DRS.ofList []) = [.anaphor .bare .one 0] := by decide
```

- `Proofs/EventPatterns.lean:111-112` `entryAndAttachmentChoicesPublishTheirSort`: replace its two
  `≠ []` conjuncts with

```lean
      Ability.checkText (DRS.ofList []) [colorReader] =
        [.choiceRef .theChoice (.quality .color) 0] ∧
      Ability.checkText (DRS.ofList []) [playerReader] = [.choiceRef .theChoice .player 0] := by
```

- `Proofs/KeywordAbilityCensus.lean:310` `optionalTextChoiceIsPublished`: its `≠ []` conjunct
  becomes

```lean
      Ability.checkText (DRS.ofList []) [substitute] = [.choiceRef .theChoice .player 0] := by decide
```

- `Proofs/TextChanges.lean:41-43`. Its docstring says "refused as it is without them", so it
  asserts both forms:

```lean
theorem bracketedKindMismatchIsRefused :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (target (bracketed isCreature)) =
        [.kindMismatch .player .object] ∧
      NounPhrase.checkAt (some .player) (DRS.ofList []) (target isCreature) =
        [.kindMismatch .player .object] := by
  decide
```

- `Proofs/Sharing.lean:71-74`:

```lean
theorem badSharesWithAPlayer :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (allOf (.and [isCreature, .shares .color .share (target .isPlayer)])) =
        [.kindMismatch .object .player] := by
  decide
```

- `Proofs/Static.lean:214-219`:

```lean
theorem conditionOverTooDeepReplacementIsRefused :
    StaticSpec.check (DRS.ofList [])
      (replace (would you (draw 1))
        (doIf (is you (hasDesignation citysBlessing))
          (nestCallers 9 (instead (draw 2))))) = [.recursionLimit] := by
  decide
```

- `Proofs/Retention.lean:196-197`, the first conjunct of `eachKindLaws`. The checker reports
  `.eachKindPlacement` once per `.inCaller` read in production `protection`. That count is
  incidental, so the pin asserts the set:

```lean
    (Ability.check emptyContext (protection (and [eachKind .color, isCreature]))).eraseDups =
      [.eachKindPlacement] ∧
```

- `Proofs/Actor.lean:184-190` and `:195-201`:

```lean
theorem scopedBodyCannotReadTheCaller :
    let caller : DRS :=
      (DRS.ofList [{ det := .target, plur := .one, payload := .object [.creature] (some .battlefield) none none none }])
    Action.check caller (.ownScope none (destroy it)) = [.anaphor .bare .one 0] ∧
      Action.check caller (destroy it) = [] := by
  decide
```

```lean
theorem keywordAbilityBodyCannotReadItsCard :
    let card : DRS := (DRS.ofList [{ det := .the, plur := .one, payload := .object [.creature] (some .battlefield) none none none }])
    Ability.check card (.keyword (.named "flying") [] [Primitives.Ability.spell none (destroy it)]) =
        [.anaphor .bare .one 0] ∧
      Ability.check card (Primitives.Ability.spell none (destroy it)) = [] := by
  decide
```

  C6 rewrites the two `Primitives.Ability.spell none` in this block. Keep each docstring.

These are not refusal checks and stay: `Cards/Counters.lean:372`, `Proofs/Damage.lean:86`,
`Proofs/Trigger.lean:342`, `Proofs/Faces.lean:642`, and the `DrtCards` `.isEmpty = true` checks.

### C5.2 Retire `p1p1CounterCountMultipliedTwin`

- Delete `Proofs/Rules.lean:203-214`: the docstring, `ownCount`, `p1p1CounterCountMultiplied`,
  the theorem and the blank line after it.
- Reason: it pins that a retired, double-counting spelling checks clean (Opus §6.5). It is the
  wrong-spelling twin of `p1p1CounterDefinitionZone`. `ownCount` has no other use.
- In `lean/CONTRACTS.md:1391-1393`, replace
  ```
  model's: `Definition.check` checks each conferral once in the empty
  context and computes no multiplicity, so nothing in the checker depends on it
  (`Proofs/Rules.lean`, `p1p1CounterCountMultipliedTwin`, which checks clean).
  ```
  with
  ```
  model's: `Definition.check` checks each conferral once in the empty
  context and computes no multiplicity, so nothing in the checker depends on it
  (`p1p1CounterDefinitionZone`, `Proofs/Rules.lean`, states one counter's conferral).
  ```

### C5.3 Pin the unpinned constructors

These `Refusal` constructors (`Check/Refusal.lean`) have no public pin today: `concreteBallot`
(:20), `phrasal` (:30), `countedMention` (:50), `choosable` (:55), `contrastAnchor` (:105),
`bindingless` (:110), `flatDisjuncts` (:119), `facesFit` (:145), `linkedFacetNamed` (:155),
`handoffPerformer` (:168), `counterHolderKind` (:186), `qualityRead` (:187) and `cardName` (:204).
Add each pin below at the stated place.

**`Proofs/Piles.lean`, after `badSingletonBallot` (:36)**: `choosable`, `concreteBallot`.

```lean
/-- Council's Judgment: "each player votes for a nonland permanent you don't control": a
candidate ballot names one object each voter chooses [CR#701.38a,701.38b]. -/
theorem okCandidateBallot :
    Ballot.check (DRS.ofList []) (byCandidate (a (and [isPermanent, non isLand, youDontControl])))
      = [] := by decide

/-- "each player votes for all nonland permanents you don't control": a vote is for one choice
[CR#701.38a], so a candidate ballot's phrase is an indefinite, a target or a counted mention, never
a set-naming determiner. -/
theorem badSetNamingCandidateBallot :
    Ballot.check (DRS.ofList []) (byCandidate (allOf (and [isPermanent, non isLand, youDontControl])))
      = [.choosable] := by decide

/-- "Whenever a player votes": a vote watched as an event pattern needs no ballot written. -/
theorem okWildcardBallotInAPattern :
    GameEvent.check (DRS.ofList []) (does (a isPlayer) vote) = [] := by decide

/-- "Each player votes.": a performed vote names the choices it is for [CR#701.38a]; only a
pattern may leave its ballot open. -/
theorem badWildcardBallotPerformed :
    Action.check (DRS.ofList []) (instruct eachPlayer vote) = [.concreteBallot] := by decide
```

**`Proofs/Piles.lean`, after `okShuffleTheChosenPile` (:163-169)**: `facesFit`, `handoffPerformer`.

```lean
/-- "An opponent separates all creatures into two face-down piles." -/
theorem okOneFacePerPile :
    Action.check (DRS.ofList [])
      (instruct anOpponent (.separateIntoPiles (allOf isCreature) 2 [.faceDown, .faceDown])) = [] := by
  decide

/-- "An opponent separates all creatures into two piles" with one face written: a written face
list gives each pile its face, so it names as many faces as piles. -/
theorem badFaceCountMismatch :
    Action.check (DRS.ofList [])
      (instruct anOpponent (.separateIntoPiles (allOf isCreature) 2 [.faceDown])) = [.facesFit] := by
  decide

/-- "An opponent separates all creatures into two piles. That player deals 1 damage to target
creature.": a handoff names a player. -/
theorem okHandoffAfterPartition :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct anOpponent (separateIntoPiles (allOf isCreature) 2),
          instruct (that .player) (dealDamage 1 (target isCreature)) ]) = [] := by
  decide

/-- "An opponent separates all creatures into two piles. Those piles deal 1 damage to target
creature.": a handoff's performer is a player or an object, and a pile is not an object
[CR#700.3b]. -/
theorem badPileAsHandoffPerformer :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct anOpponent (separateIntoPiles (allOf isCreature) 2),
          instruct (those .pile) (dealDamage 1 (target isCreature)) ]) = [.handoffPerformer] := by
  decide
```

**`Proofs/Counters.lean`, after `badChosenCounterKindRead` (:35)**: `counterHolderKind`.

```lean
/-- "An opponent separates all creatures into two piles. Put a +1/+1 counter on each creature
in the pile of your choice." -/
theorem okCountersOnCreaturesInAPile :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct anOpponent (separateIntoPiles (allOf isCreature) 2),
          putCounters 1 p1p1Counter (each (and [isCreature, .in_ (.pile (pileOfChoice .you))])) ])
      = [] := by
  decide

/-- "An opponent separates all creatures into two piles. Put a +1/+1 counter on the pile of your
choice.": a counter is placed on an object or a player [CR#122.1], and a pile is not an object
[CR#700.3b]. -/
theorem badCountersOnAPile :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct anOpponent (separateIntoPiles (allOf isCreature) 2),
          putCounters 1 p1p1Counter (pileOfChoice .you) ]) = [.counterHolderKind] := by
  decide
```

**`Proofs/Zone.lean`, after `badSingularNameAgreement` (:476-479)**: `countedMention`. Its positive
twin is the existing `okPluralNameAgreement` (:470).

```lean
/-- "… all nonland cards in your hand with the same name": an Agreement's group is a counted or
targeted mention ("two cards with the same name", `okPluralNameAgreement`), not every member of
a set. -/
theorem badUncountedNameAgreement :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.agree .cardName .share (allOf (.and [.not isLand, .in_ hand]))) = [.countedMention] := by
  decide
```

**`Proofs/Description.lean`, after `badNestedConjunction` (:407-412)**: `flatDisjuncts`.

```lean
/-- "If you control a creature, a land, or an artifact, …" -/
theorem okFlatConditionDisjunction :
    Condition.check (DRS.ofList [])
      (.or [ exists_ (.and [isCreature, .hasPossessor .controller .you]),
             exists_ (.and [isLand, .hasPossessor .controller .you]),
             exists_ (.and [isArtifact, .hasPossessor .controller .you]) ]) = [] := by
  decide

/-- "If you control a creature or a land, or you control an artifact, …": a disjunction is
flat, its disjuncts written in one list, as a conjunction is (`badNestedConjunction`). -/
theorem badNestedDisjunction :
    Condition.check (DRS.ofList [])
      (.or [ .or [exists_ (.and [isCreature, .hasPossessor .controller .you]),
                  exists_ (.and [isLand, .hasPossessor .controller .you])],
             exists_ (.and [isArtifact, .hasPossessor .controller .you]) ]) = [.flatDisjuncts] := by
  decide
```

**`Proofs/ContrastAnchor.lean`, before `end` (:388)**: `contrastAnchor`. It uses the file's private
`framed` and `anchorsOf`. The `anchorsOf` conjunct was checked by `#eval` (it returns
`[.source]`) but not yet under `decide`. If it fails under `decide`, stop and report.

```lean
/-- "Return another target creature card from your graveyard to your hand." on a creature: its
Source's zone is unknown, so the Source anchors "another". -/
theorem okOtherCardOnACreature :
    Action.check (framed [.creature])
        (returnToHand (target (and [isCreature, isOther, cardIn yourGraveyard]))) = [] ∧
      anchorsOf (returnToHand (target (and [isCreature, isOther, cardIn yourGraveyard])))
        (framed [.creature]) = [.source] := by
  decide

/-- The same line on an instant: the Source is a spell on the stack, never in a graveyard, and
nothing earlier is a creature card in a graveyard, so "another" has no Contrast Anchor. -/
theorem badOtherCardOnAnInstant :
    Action.check (framed [.instant])
      (returnToHand (target (and [isCreature, isOther, cardIn yourGraveyard]))) =
      [.contrastAnchor .object] := by
  decide
```

**`Proofs/LinkedReadback.lean`, top level, before `end Semantics.Proofs.LinkedReadback` (:596)**:
`linkedFacetNamed`. The term is Stormscape Battlemage's (`Cards/Trigger.lean:509-512`), written out
because `Cards.Trigger` is not imported here.

```lean
/-- Stormscape Battlemage: "When this creature enters, if it was kicked with its first kicker,
you gain 3 life." [CR#702.33f]. -/
theorem okFirstKickerRead :
    Ability.check (DRS.ofList [])
      (when (does thisCreature enter)
        (if_ := costWasPaid (.byNthKeyword (.nth 1) kicker) none thisCreature) (gainLife 3)) = [] := by
  decide

/-- "… if it was kicked with its zeroth kicker …": a Link Source's ordinal counts from the first
of its keyword's costs [CR#702.33f], so the zeroth names none. -/
theorem badZerothKickerRead :
    Ability.check (DRS.ofList [])
      (when (does thisCreature enter)
        (if_ := costWasPaid (.byNthKeyword (.nth 0) kicker) none thisCreature) (gainLife 3)) =
      [.linkedFacetNamed] := by
  decide
```

**`Proofs/Choice.lean`, before `end` (:748)**: `qualityRead`.

```lean
/-- "As this creature enters, choose a color. Creatures are the chosen color in addition to their
other colors." -/
theorem okChosenColorEdit :
    Ability.checkText (DRS.ofList [])
      [ .static (entersChoosing thisCreature .color),
        .static (.characteristicChange .any [.quality .adds (.chosen (.ofChosen .theChoice .color))]) ]
      = [] := by
  decide

/-- "… Creatures are color in addition to their other colors.": a chosen quality is written as
a chosen one ("the chosen color", "the color of your choice", a name, or a shared quality), not as
the bare quality noun. -/
theorem badUnchosenColorEdit :
    Ability.checkText (DRS.ofList [])
      [ .static (entersChoosing thisCreature .color),
        .static (.characteristicChange .any [.quality .adds (.chosen (.qualityNoun .color none))]) ]
      = [.qualityRead] := by
  decide
```

**`Proofs/ComparisonBox.lean`, inside `namespace Semantics.Proofs.ComparisonBox`, before its `end`
(:169)**: `phrasal` (Decision 6). The file uses raw constructors and no opens; so does this.

```lean
private def creatureCountAboveZero (domain : Predicate) : Predicate :=
  .compareOver domain (.countOf (.described .all (.hasType .creature))) .greater (.lit 0)

/-- A comparison whose domain writes no kind ranges over its head's kind: an object. -/
theorem okComparisonOverAnObjectHead :
    Predicate.checkAt .object (DRS.ofList []) (creatureCountAboveZero .isSource) = [] := by decide

/-- A comparison binds one member of its domain, which a phrase denotes (`Kind.phrasal`): an
object, a player or a quality, never a pile [CR#700.3b]. -/
theorem badComparisonOverAPileHead :
    Predicate.checkAt .pile (DRS.ofList []) (creatureCountAboveZero .isSource) =
      [.phrasal .pile] := by decide
```

**`Proofs/Deontic.lean`, after `okIndicativeCrewAsThough` (:585-589)**: `bindingless`.

```lean
/-- Hotshot Mechanic: "… as though its power were 2 greater." -/
theorem okLiteralAsThoughAmount :
    StaticSpec.check (DRS.ofList [])
      (asThough thisCreature (crew (allOf (hasSubtype vehicle))) (greaterBy .power 2)) = [] := by
  decide

/-- "… as though its power were greater by target creature's power.": an as-though amount reads
a value and introduces no referent; a static ability is none of the targeted spells and abilities
[CR#115.1a..115.1d,609.4]. -/
theorem badAnnouncingAsThoughAmount :
    StaticSpec.check (DRS.ofList [])
      (asThough thisCreature (crew (allOf (hasSubtype vehicle)))
        (greaterBy .power (.statOf (.stat .power) (target isCreature)))) = [.bindingless] := by
  decide
```

**`Proofs/Faces.lean`, after `okSpellAbilityOnInstant` (:43-48)**: `cardName`.

```lean
/-- A face is named [CR#201.1]: an instant with no name line is refused. `okSpellAbilityOnInstant`
is its named twin. -/
theorem badUnnamedFace :
    Card.check (one { name := none, cost := some [pip .blue], types := [.instant], text := [spell (draw 1)] })
      = [.cardName] := by
  decide
```

`badAnnouncingAsThoughAmount` cites `[CR#115.1a..115.1d]`. This spec's commit already
registered those rules in `cr-citations.lock`. If `cargo xtask cite check` still reports an
`UNLOCKED` rule, run `cargo xtask cite bless`.

Acceptance for C5: every constructor of `Refusal` in `Check/Refusal.lean` appears as `.name` in an
expected refusal list in `lean/Semantics/Proofs/` or `lean/Semantics/Cards/`.

---

## C6. Templates, not `Primitives` wrappers, in `Proofs/`

The rule is the one `lean-v3-authoring-surface` §5.4 applied to `Cards/` and `Definitions/`:
write each through its Template. There are 81 occurrences on 71 lines; 65 are replaced and the 16
in `Authoring.lean` listed under "What must not change" stay.

| File | Lines | Replace with |
|---|---|---|
| `Actor.lean` | 197, 199 | `Primitives.Ability.spell none (destroy it)` → `spell (destroy it)` (`Templates.lean:2082`) |
| `Actor.lean` | 312, 316, 320, 655 | `Primitives.Action.doForEach` → `doForEach` (`Templates.lean:1528`) |
| `CallerScope.lean` | 89, 95 | `Primitives.Action.setStatus .tapped X` → `setStatus .tapped X` (`Templates.lean:1611`); not the `tap` Definition, which is a different term |
| `CallerScope.lean` | 153, 196 | `Primitives.NounPhrase.you` → `you` (`Templates.lean:84`) |
| `Authoring.lean` | 24, 52, 59, 66, 132, 169, 174, 175, 176 | `Primitives.Ability.spell none X` → `spell X` (at :66, `spell` ends the line) |
| `Authoring.lean` | 74 | `(Primitives.Action.sequentially` → `(sequentially` (`Templates.lean:1607`) |
| `ControllerSacrifice.lean` | 90 | `[Primitives.Ability.spell none` → `[spell` |
| `ControllerSacrifice.lean` | 91 | `(Primitives.Predicate.coord .and [isArtifact, isCreature])` → `(and [isArtifact, isCreature])`. Use the bare `and` Template (`Templates.lean:276,313`), not `.and`: this is inside `spelled <|` |
| `Deontic.lean` | 421, 429, 436 | `Primitives.Cost.scaled` → `scaled` (`Templates.lean:2077`) |
| `Mana.lean` | 205, 210, 213 | the same |
| `KeywordAbilityCensus.lean` | 316, 323, 324, 330, 331, 338, 343, 344, 349, 355 | `Primitives.Ability.spell none` → `spell` |
| `TextChanges.lean` | 62, 76 | the same |
| `Piles.lean` | 268 | `(Primitives.Action.addTurnPart .upkeep (some .mainPhase) 1 none)` → `(addTurnPart .upkeep 1 (some .mainPhase))` (`Templates.lean:1485`: `part count (anchor := none) (followedBy := none)`) |
| `Turn.lean` | 323 | → `(addTurnPart .upkeep 1 (some .combat))` |
| `Turn.lean` | 366 | → `(addTurnPart .endingPhase 1 (some .postcombatMain))` |
| `Turn.lean` | 485, 491, 533, 536 | → `(addTurnPart .upkeep (lifeTotalOf they))` |
| `Turn.lean` | 496 | → `(addTurnPart .upkeep (lifeTotalOf (target (.isOpponentOf .you))))` |
| `Turn.lean` | 501 | → `[addTurnPart .upkeep (lifeTotalOf (target (.isOpponentOf .you))),` |
| `Turn.lean` | 508 | → `(addTurnPart .upkeep (plus (.letter .x) (.letter .x)))` |
| `Turn.lean` | 555-556 | → `(addTurnPart .upkeep` / `(plus (powerOf (target isCreature)) (lifeTotalOf (target (.isOpponentOf .you)))))` |

`Static.lean:125-128` becomes:

```lean
        (and [
              allOf (and [isCreature, youControl]),
              allOf (and [isCreature, isSpell, youControl]),
              allOf (and [isCard, isCreature, youOwn, not (in_ battlefield)]) ])
```

The Turn rows give the whole new call: replace the full `(Primitives.Action.addTurnPart …)`
expression on that line. The `Authoring.lean` `#guard_msgs` error messages must still match.
After C1 and C5 the line numbers in this table have moved, so locate each site by its
`Primitives.` text.

Acceptance: `grep -rn 'Primitives\.' lean/Semantics/Proofs/` prints exactly 16 lines, all in
`Authoring.lean`, each an `example` or a `/-- error: Unknown identifier` docstring.

---

## C7. Domain names for the slug-named proof modules

Move each file with `mv`; jj records the rename. Then update its header, namespace and `end`:

| Old | New | Header / namespace / end lines |
|---|---|---|
| `Proofs/KeywordAbilityCensus.lean` | `Proofs/KeywordBodies.lean` | header :2 after C2 (`# Semantics.Proofs.KeywordAbilityCensus`), `namespace`, `end` |
| `Proofs/KeywordActionResidues.lean` | `Proofs/KeywordActions.lean` | header :7, namespace :15, end :143 |
| `Proofs/GetsBothDeltas.lean` | `Proofs/GetsPt.lean` | namespace :14, end :512 |
| `Proofs/CorpusRecency.lean` | `Proofs/DrtRecency.lean` | namespace :5, end :243 |

Keep the module docstrings' prose as written. Only the `# Semantics.Proofs.X` heading line
changes. Where prose names the ticket slug (e.g. "`semantics-v2-keyword-ability-census`"), it
stays: it names a ticket, not the module.

Replace `lean/Semantics/Proofs.lean`'s import block with:

```lean
import Semantics.Proofs.AbilityQualifiers
import Semantics.Proofs.ActionFamilies
import Semantics.Proofs.ActionForms
import Semantics.Proofs.Actor
import Semantics.Proofs.Anaphora
import Semantics.Proofs.Attachments
import Semantics.Proofs.Authoring
import Semantics.Proofs.BindingIdentity
import Semantics.Proofs.CallerScope
import Semantics.Proofs.CardFidelity
import Semantics.Proofs.CharacteristicEdits
import Semantics.Proofs.CheckerCore
import Semantics.Proofs.Choice
import Semantics.Proofs.ComparisonBox
import Semantics.Proofs.Composition
import Semantics.Proofs.ContrastAnchor
import Semantics.Proofs.ControllerSacrifice
import Semantics.Proofs.Counters
import Semantics.Proofs.Damage
import Semantics.Proofs.DefinitionsAndTemplates
import Semantics.Proofs.Deontic
import Semantics.Proofs.Description
import Semantics.Proofs.DrtCards
import Semantics.Proofs.DrtRecency
import Semantics.Proofs.DrtResolution
import Semantics.Proofs.DrtWitnesses
import Semantics.Proofs.DuringResolution
import Semantics.Proofs.EventPatterns
import Semantics.Proofs.FaceDown
import Semantics.Proofs.Faces
import Semantics.Proofs.GetsPt
import Semantics.Proofs.GiftVariants
import Semantics.Proofs.Helpers
import Semantics.Proofs.HistoricalMove
import Semantics.Proofs.Keyword
import Semantics.Proofs.KeywordActions
import Semantics.Proofs.KeywordArguments
import Semantics.Proofs.KeywordBodies
import Semantics.Proofs.KeywordByCardClass
import Semantics.Proofs.KeywordCastPermissions
import Semantics.Proofs.KeywordHolders
import Semantics.Proofs.LinkedChoiceReadback
import Semantics.Proofs.LinkedReadback
import Semantics.Proofs.Lookback
import Semantics.Proofs.Mana
import Semantics.Proofs.MovementScopes
import Semantics.Proofs.NounWords
import Semantics.Proofs.Numbers
import Semantics.Proofs.OffStack
import Semantics.Proofs.Piles
import Semantics.Proofs.ReferenceScopes
import Semantics.Proofs.RefusalReachability
import Semantics.Proofs.ReplacementArms
import Semantics.Proofs.Retention
import Semantics.Proofs.Rules
import Semantics.Proofs.SelectionEvidence
import Semantics.Proofs.Sharing
import Semantics.Proofs.SpellCopies
import Semantics.Proofs.Static
import Semantics.Proofs.Tables
import Semantics.Proofs.TextChanges
import Semantics.Proofs.Trigger
import Semantics.Proofs.Turn
import Semantics.Proofs.TypeEvidence
import Semantics.Proofs.UntapStep
import Semantics.Proofs.Zone
```

Rewrite these references to the new names. Nothing else references the modules: no
`lakefile.toml`, `lean/README.md`, `crates/xtask` or plugin file does.

- `lean/Semantics/Proofs/Anaphora.lean:944,955`: `GetsBothDeltas.selectedPtSubjectHasStableIdentity`
  → `GetsPt.selectedPtSubjectHasStableIdentity`, and
  `GetsBothDeltas.ambiguousSingularPtSubjectIsRefused` → `GetsPt.ambiguousSingularPtSubjectIsRefused`.
- `lean/CONTRACTS.md:635`: `Proofs/KeywordAbilityCensus.lean` → `Proofs/KeywordBodies.lean`.
- `docs/tickets/planned/lean-v3-alternative-costs-as-printed.md:142`: `Proofs/KeywordAbilityCensus.lean:126`
  → `Proofs/KeywordBodies.lean` (drop the stale line number).
- `docs/tickets/planned/lean-v3-deed-duplicates.md:24`: `Proofs/CorpusRecency.lean:222` →
  `Proofs/DrtRecency.lean` (drop the stale line number).
- `docs/tickets/planned/lean-v3-owner-zone-introduces-owner.md:37`: `Proofs/CorpusRecency.lean` →
  `Proofs/DrtRecency.lean`.

Do not rewrite `docs/tickets/done/*` or `docs/evidence/*`.

Acceptance: `grep -rn 'KeywordAbilityCensus\|KeywordActionResidues\|GetsBothDeltas\|CorpusRecency' lean/ docs/tickets/planned docs/tickets/maybe docs/tickets/critical`
prints nothing except prose that names the ticket slug `semantics-v2-keyword-ability-census` or
`semantics-v2-gets-both-deltas`.

---

## C8. Documentation

### C8.1 `lean/CONTRACTS.md`

1. :20 `(`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06)` →
   `(`docs/archive/decisions/semantics-v2.md` §7, ruling 2026-10-06)`.
2. :1115 `(`semantics-v2.md` §7, ruling 2026-10-09, "A linked ability reads back a` →
   `(`docs/archive/decisions/semantics-v2.md` §7, ruling 2026-10-09, "A linked ability reads back a`.
3. :1162 ``from_` (`semantics-v2.md` §7, ruling 2026-10-09) [CR#113.10a,113.6b]. A`` →
   ``from_` (`docs/archive/decisions/semantics-v2.md` §7, ruling 2026-10-09) [CR#113.10a,113.6b]. A``.
   Rewrap the paragraph if a line exceeds the file's width.
4. :83-88. Replace
   ```
   amount beside "the same kinds of counters") and `verbBecomesOk` (with the deed table's
   `intransitive` column). So are the keyword-definition checks (`keywordBodyFits`: a
   definition part's category, stack regime and card-class guard against the keyword's row)
   and the card face's joint choices (`jointChoices`): a later ability on a face reads a choice
   an earlier one makes through the threaded text context, as any reference does. The deed
   table's role columns are gone, and
   ```
   with
   ```
   amount beside "the same kinds of counters") and `verbBecomesOk` (which read the deed table's
   `intransitive` column). So are the keyword-definition checks (`keywordBodyFits`: a
   definition part's category, stack regime and card-class guard against the keyword's facts row)
   and the card face's joint choices (`jointChoices`): a later ability on a face reads a choice
   an earlier one makes through the threaded text context, as any reference does. The keyword
   and deed facts tables are deleted (`lean-v3-keyword-facts-deleted`,
   `lean-v3-action-facts-deleted`), and
   ```
   Line 89 ("with them `deedFits` and `deedNounOk`: …") stays.
5. :1089-1090. Replace
   ```
     and the facts table no longer carries a definition, a regime, `functionsOnStack`,
     `counterEligible` or a card-class column.
   ```
   with
   ```
     and the keyword facts table that carried a definition, a regime, `functionsOnStack`,
     `counterEligible` and a card-class column is deleted (`lean-v3-keyword-facts-deleted`).
   ```
6. :286-289. In this paragraph, replace `(`anaphor`)` with `(`whichUnbound`)` and
   `` `lookbackSubject` `` with `` `whichUnused` `` (`Check/Refusal.lean:68-73`;
   `Check/Phrase.lean:3085,1547`). Nothing else in the paragraph changes.
7. :398 `(`okJuxtaposeThemReadsTheTiedPermanents`, `purgingScytheThemReadsTheTiedCreatures`). A` →
   `(`okJuxtaposeThemReadsTheTiedPermanents`, `loxodonPeacekeeperThemReadsTheTiedPlayers`). A`
   (`Cards/Choice.lean:1628`).
8. :756 `` `Predicate.seedType` presupposition calculation is not stored type evidence. `` →
   `` `Predicate.seedTy` presupposition calculation is not stored type evidence. ``
   (`Check/PredicateShape.lean:7`).
9. :1194 ``[CR#400.7]. The put and return Templates (`putInto`, `put`, `putWith`,`` →
   ``[CR#400.7]. The put and return Templates (`putInto`, `put`,``. `putWith` was merged into
   `put` (change `uwlyvrlwpovw`). Rewrap the next line if needed.

### C8.2 `lean/README.md`

1. Replace :59-70 (from "Actions and their templates and definitions use dictionary forms" through
   the `choose` disclosure sentence) with:

   ```
   Actions and their templates and definitions use dictionary forms (`exile`, `mill`, `gainLife`);
   static-spec templates use nouns (`modification`, `abilityGrant`), except that the deontic
   templates use their modal (`cant`, `can`, `must`, `gatedBy`) and a replacement effect is
   `replace`, the word it shares with a self-replacement action, its replacement marked `instead`
   where the text says so (`replace (would you (draw 1)) (instead (draw 2))`).
   Event constructors retain their predication forms. Action composition uses `sequentially` and `simultaneously`; fixed repetition
   lives in `Repetition.fixed`. No action writes its performer: it is
   `actor`, the controller unless an enclosing `instruct performer body` hands it to
   another player or a permanent; "target player draws a card" is
   `instruct (target isPlayer) (draw 1)`. A choice's chooser and an added
   turn's taker are the actor too ("target player chooses a creature" is
   `instruct (target isPlayer) (choose (a isCreature))`). `choose` accepts
   `(disclosure := .secretly)`.
   ```

2. In :74-75, replace the sentence "Designations are open labels whose declared scope feeds the
   checker tables." with "A designation is a catalog value whose declared scope the checker reads
   off the value (see "Registry facts")."
3. In :89-92, replace "the row's own obligations are only what the scope adds: the kind it may
   bind, and that a damage result's counter is a declared one." with "the row's own obligation is
   only what the scope adds: the kind it may bind." (`Check/Rules.lean`'s `SbaRule.check` and
   `ConferralRule.check` refuse only `kindLte`.)
4. Replace the "## The checker" section (:134-152) with:

   ```
   ## The checker

   1. **Syntax** (`Semantics/*.lean`) supplies the card-language terms.
   2. **Attributes** (among them `Check/Words`, `Check/PredicateShape`, `Check/Selections` and
      `Check/Queries`) hold the discourse state (`DRS`), the anaphor resolver, and projections such
      as kind, number, zone and type. The stepper (`Check/Phrase`, whose action, cost, static and
      ability entry points `Check/Abilities` re-exports) threads the `DRS` through each term in
      reading order, so field contexts determine which earlier introductions each child can read.
   3. **Rules** (`Check/Phrase`, `Check/Events`, `Check/Keywords`, `Check/AbilityRules`,
      `Check/Card`) check those projections and return named `Refusal` values (`Check/Refusal`);
      `Check/Rules` checks the rules-as-data rows. Laws read structural features and the columns
      a catalog value carries.
   4. **Entry point**: `Card.check : Card → List Refusal`. A `Spelled` value
      combines a card with a proof of `card.check = []`; `spelled` asks `decide`
      to construct that proof (see "The authoring boundary").

   `Predicate.kind?` and `NounPhrase.kind?` infer kinds. A context expecting a
   kind refuses a `kindMismatch` when they disagree. Each disjunct of a joined
   phrase is checked at its own kind. The [checker contracts](CONTRACTS.md)
   record field contexts, enclosed scopes, and the checks or planned redesigns
   that enforce them.
   ```

5. Replace the "## Pins" section (:154-167) with:

   ````
   ## Pins

   A positive pin proves that the exact refusal list is empty; its negative twin
   proves the exact expected list. For example, `Proofs/Anaphora.lean` contains:

   ```lean
   theorem badChooseYou :
       Action.check (DRS.ofList []) (.choose none .you .openly none) = [.choiceClause] := by
     decide
   ```

   `okX` / `badX` names and card sentences are retained when a term is re-spelled.
   Changing the syntax does not authorize weakening the asserted result or
   removing its twin. `Proofs/Tables.lean` reads catalog declarations back through the
   accessors the checker uses (a counter's holder, a designation's scope, the keyword actions a
   permanent performs, a card type's permanence and stats). `decide` evaluates these laws in the
   kernel.
   ````

6. Insert a new section after "## Cards" (after :180), before the recursion section:

   ```
   ## The authoring boundary

   A bench card is `spelled <| card`. The `spelled` elaborator (`Check/Card.lean`) reads the
   elaborated term before reduction erases its Template and Definition calls, and records it as an
   `Authoring.Form` tree (`Semantics/Authoring.lean`): the named calls, data constructors,
   literals and parameters it was written with, and a `raw` leaf for any semantic constructor
   written directly. It refuses the card unless the tree is `onlyAuthored` (it names the raw
   constructors) and `card.check = []`; the resulting `Spelled` carries both proofs, closed by
   `decide`, and its constructor is private, so no record syntax builds one without them. A
   semantic expression in a card is therefore written through a `semantic_template` or a
   `semantic_definition`; `Semantics.Primitives` wraps the syntax constructors as Templates. The
   catalog items (card types, subtypes, counters, designations, relations, keyword labels and
   deeds) are `internal_expansion`, so an undeclared one in a card is refused here, as a raw
   constructor, not by the checker. Authored declarations are trusted: the checker checks a
   Definition's expanded body wherever a card calls it, but nothing proves that the body means
   what the rules say, and an Action Definition is refused at declaration unless its body
   establishes its own Reference Scope (`Action.ownScope`). Card fragments and `Proofs/` terms that
   call the checker directly are fixtures and are not inspected. The full contract is
   [Card authoring and trusted expansion](CONTRACTS.md#card-authoring-and-trusted-expansion); the
   pins are `Proofs/Authoring`.
   ```

7. Replace the "## Structural recursion is load-bearing" section (:182-197) with:

   ```
   ## Fuel is load-bearing

   `decide` asks the kernel to evaluate the checker, and the kernel cannot unfold
   well-founded recursion. The phrase, trigger and ability grammar is one mutual block in
   `Abilities.lean`, elaborated by the scoped `mutual` elaborator `open scoped Semantics.Authoring`
   installs, which omits the eager `brecOn`, `SizeOf` and injectivity theorems the block's nested
   occurrences would make expensive. Without `brecOn` nothing recurses structurally on a spine
   type, so every recursive function over the syntax, in `Check/*` and beside the syntax
   (`Location.positiveZones?`), takes a `fuel : Nat` and carries `termination_by structural fuel`;
   a function that would fall back to well-founded recursion is a build error, not a slow proof.
   The block also derives no instances: `Amount`, `Predicate`, `Action` and the rest have no
   `DecidableEq`, `BEq` or `Repr`, so a pin equating two terms closes by `rfl`, not `decide`
   (`Proofs/Numbers`).

   Fuel bounds nesting depth, not the length of a card's text. The stepper (`stepOps`,
   `Check/Phrase.lean`) spends one unit per level of nested phrase, action, cost or ability, and
   when it runs out refuses `recursionLimit` rather than admitting what it did not read
   (`exhaustedTraversalRefuses`, `conditionOverTooDeepReplacementIsRefused`). The entry points
   pass 64; a pin over a deep fixture may pass more (`stepAction 512 …`). An attribute reader
   (`Predicate.seedTy` and its kin) returns its no-information value when its fuel runs out: no
   card types, no zone, `false`. A term the bench needs read deeper raises the entry point's
   fuel; it never moves a function to well-founded recursion.
   ```

8. The conventions bullet at :213-214. Replace
   ```
   - Types derive `Repr` and `BEq`, and `DecidableEq` where derivation supports
     their recursive shape. Rules-bearing docstrings use bracketed Comprehensive Rules citations.
   ```
   with
   ```
   - Types outside the spine's mutual block derive `Repr` and `BEq`, and `DecidableEq` where
     derivation supports their shape; the block derives none (see "Fuel is load-bearing").
     Rules-bearing docstrings use bracketed Comprehensive Rules citations.
   ```

9. `Check/Triggers` (:137) disappears with item 4. Check afterwards that
   `grep -n 'Check/Triggers\|Action.check \[\]\|\bact (' lean/README.md` prints nothing.

### C8.3 Lean docstrings

- `Proofs/Numbers.lean:14-15`: replace
  `` `Amount` derives no `DecidableEq` (it sits in the `` / `` nested mutual block of `Phrase`, see the README), so an equation over ``
  with `` `Amount` derives no `DecidableEq` (it sits in the `` / `` mutual block of `Abilities.lean`; see the README's "Fuel is load-bearing"), so an equation over ``.
- `Cards/Damage.lean:220-221`, the `purgingScythe` docstring. Replace
  `A counted existential, whose domain "them" reads (`semantics-v3.md` §1.3, box 2).` with
  `The tie-break is written as the choice of one creature with the least toughness, which "that creature" reads.`
  The quoted Oracle sentence stays.
- `Check/Events.lean:8-9`, the module docstring: replace
  `The event facts table, closed event head-gating (`does`, `would`, `did`), and the lookback,`
  with `Closed event head-gating (`does`, `would`, `did`) and the lookback,`. The next line stays.

### C8.4 `docs/decisions/semantics-v3.md` (edit in place, Decision 7)

1. §2 table, Law Gate row (:293). Replace
   ```
   | **Law Gate** | Checked as an independent unit by `cargo xtask lean definitions`. | Checked at every card or `Definition` call site that expands it. |
   ```
   with
   ```
   | **Law Gate** | Trusted, as every authored declaration is (`lean/CONTRACTS.md`, "Card authoring and trusted expansion"); nothing checks a `semantic_definition` on its own, and `cargo xtask lean definitions` is retired. An Action Definition is refused at declaration unless its body establishes its own Reference Scope (`Action.ownScope`, `lean/Semantics/Authoring.lean`); its body is checked at every card or `Definition` call site that expands it, and a Registry Definition's conferrals by `Definition.check`, proved empty by `decide` in `lean/Semantics/Proofs/Rules.lean`. | Checked at every card or `Definition` call site that expands it. |
   ```
   (Evidence: `crates/xtask/src/bin/cargo-xtask.rs:143-145` asserts `lean definitions` fails to
   parse, and `lean/Semantics/Authoring.lean:276-278` refuses a non-`ownScope` Action Definition.)
2. `Instruction` → `Action`.
   - Headings: :315 `### 3.1 Core Principle: Every Verb Defined Once as an `Instruction`` →
     `### 3.1 Core Principle: Every Verb Defined Once as an `Action``, and :377
     `### 3.4 Transitive vs. Intransitive `Instruction` Conventions and Relative Clauses (`which`)` →
     `### 3.4 Transitive vs. Intransitive `Action` Conventions and Relative Clauses (`which`)`.
   - On lines 142, 151, 158, 302, 317, 319, 323, 324, 345, 381, 398, 421, 429, 433, 434, 437, 440,
     443, 451, 457, 472, 524, 560, 625, 634 and 635, replace every `Instruction` with `Action`.
   - :149: replace `Instruction.ownScope (deed : Option Deed := none) (body : Instruction)` with
     `Action.ownScope (deed : Option Deed) (body : Action)`, and every other `Instruction.ownScope`
     on that line with `Action.ownScope`.
   - :424, third cell: `` `Instruction.cast` carries `from_ : Option EventSource`. `` →
     `` `cast` is a Definition over `Action.procedure .cast`, whose `from_ : Option Location` is the written origin. ``
   - :425, third cell: `` Uses `Instruction.activate`. `` → `` Uses the `activate` Definition, over `Action.procedure .activate`. ``
   - :428, the whole row: replace
     `` | `losesGame (player)` | `does(player, loseTheGame)` | Uses `Instruction.loseTheGame`. | `` with
     `` | `losesGame (player)` | `does(player, loseGame)` | Uses the `loseGame` Template, over `Action.conclude .loseGame`. | ``
   - Keep :39 (§0.2, describing v2) and :67 (§1.1's retired `Instruction.withBindings`).
   - Acceptance: `grep -n 'Instruction' docs/decisions/semantics-v3.md` prints only lines 39 and 67.
3. `may` → `can` for the permission:
   - :328: in the deontic cell, `may(subj, action)` → `can(subj, action)`.
   - :335: `` The `Subject` of `does`, `would`, `cant`, `must`, `may`, `gatedBy`, and `did` has three constructors: `` →
     `` The `Subject` of `does`, `would`, `cant`, `must`, `can`, `gatedBy`, and `did` has three constructors: ``
   - :485: in the bold lead, `` `cant`, `must`, `may`, and `gatedBy` `` → `` `cant`, `must`, `can`, and `gatedBy` ``.
   - :501: `` | `deonticRule(s, .permit, deeds, .agent, ...)` | `may(s, action, mods)` | `may(you, play(it), { rider := ... })` | `` →
     `` | `deonticRule(s, .permit, deeds, .agent, ...)` | `can(s, action, mods)` | `can(you, play(it), rider := ...)` | ``
   - Leave :584, :602 and :648, which are the optional `may` Template and §4 history.
4. §3.7 (:472-481). Replace the tail of :472,
   `` … replaced by unified deontic constructors taking `(subj : Subject, action : Instruction, mods : DeonticMods := {})`: ``,
   with
   `` … replaced by unified deontic constructors taking `(subj : Subject, action : Action, mods : DeonticMods)`, whose Templates (`cant`, `can`, `must`, `gatedBy`) take each modifier as an optional argument: ``.
   Replace the `structure DeonticMods` listing (:474-481) with:

   ```lean
   structure DeonticMods where
     timing   : Option Timing        := none  -- when or how often it applies: "each turn", "during each of your turns"
     asThough : Option AsThough      := none  -- e.g., "as though it had flash", "as though it didn't have defender"
     except   : Option DeonticExcept := none  -- Deontic Exception: a keyword or the objects exempted [CR#702.13b,702.111b,702.145b]
     cause    : Option Causing       := none  -- `cant` only: the cause the prohibited event must have [CR#120.3a]
     rider    : Option Action        := none  -- `can` only: a Permission Rider, the rest of the permission's ability [CR#113.2c]
   ```

   (Source: `lean/Semantics/Abilities.lean:902-929`. The structure is in the mutual block and
   derives nothing.)

### C8.5 Glossary (`docs/contexts/game-model/CONTEXT.md`)

Delete the **Binding Possessor** entry (:749-757) and the blank line after it. Lean records no
possessor on a referent any more (`lean/CONTRACTS.md:93,559,859`), and no other entry links to the
term. `PossessorAxis` and `Predicate.hasPossessor` are grammatical possessives, a different
concept. This needs no `domain-modeling` amendment beyond the deletion. The **Definition** /
**Registry Definition** reconciliation is B1 and is blocked.

Acceptance for C8: `grep -rn 'Binding Possessor' docs/contexts` prints nothing;
`grep -n 'semantics-v2.md' lean/CONTRACTS.md` shows only `docs/archive/decisions/semantics-v2.md`.

---

## C9. Follow-up tickets

Create these three files in `docs/tickets/planned/`, each with `needs: []` frontmatter and exactly
this body (heading plus text). Then run `kata kanban check`.

`lean-v3-literal-referent-id-pins.md`:

```
# Pins name referents by structure, not by literal id

`lean-v3-proofs-and-docs-alignment` removed `nextId ± k` arithmetic from public pins. About 239
lines in `lean/Semantics/Proofs/`, `Check.lean` and `Cards/` still assert literal ids
(`ids = [n]`, `findById? n`, `(·.id) = … n`). Per file: GetsPt 32, Damage 20, MovementScopes 16,
Counters 15, CallerScope 15, DrtCards 14, SelectionEvidence 12, Choice 12, Turn 11,
ControllerSacrifice 10, ReferenceScopes 9, EventPatterns 9, others 7 or fewer. An id that reads a
fixture's own `DRS.ofList` / `ofNumbered` input is fine. An id that names a referent a step
minted is as brittle as an offset: rewrite it with `headerFreshReferents`, `firstReach`,
`resolveCandidates` or a filter on the referent's structure, as the alignment ticket's C4 did.
Standard constraints apply.
```

`lean-v3-hand-written-own-scope-bodies.md`:

```
# Audit hand-written `ownScope` bodies in the proof suites

Proof suites still write `.ownScope (some (.action …))` bodies by hand. Some are deliberate
scope or refusal fixtures; others may be replicas of production Definitions, the defect
`lean-v3-proofs-and-docs-alignment` §C2 fixed for the named suites. Classify each site as a
fixture (say so in its docstring) or a replica (point it at the production declaration):
Actor.lean amass and discard bodies, Zone.lean (four sites), KeywordActions.lean (the residue
bodies), ActionFamilies.lean, Counters.lean, CallerScope.lean (a fateseal body inside a
forwardedSubject test), Faces.lean, ActionForms.lean, Anaphora.lean (four sites), Choice.lean,
MovementScopes.lean, HistoricalMove.lean, and `merges` in LinkedReadback.lean and
KeywordCastPermissions.lean (a fragment of `mutate` without production's
`doIf (choiceWasTaken …)`). Authoring.lean and DefinitionsAndTemplates.lean's are intentional.
Standard constraints apply.
```

`lean-v3-caller-argument-refusal-multiplicity.md`:

```
# Decide how often a caller argument's refusal is reported

A Definition that reads a caller argument in several places reports the argument's refusal once
per read: `saddle (powerOf (that (.type .creature)))` reports its anaphor twice (once from the
keyword's parameter list, once from the body), and `protection (and [eachKind .color, isCreature])`
reports `eachKindPlacement` seven times. Decide whether a caller argument is checked once, and
pin the decision. Two related observations from the same audit:

- An as-though amount refuses `bindingless` depending on context: stepped directly, `countOf`
  introduces a referent and `.thatMuch` checks clean; inside a `StaticSpec` the results swap.
- `Refusal.phrasal` is reachable only through `Predicate.checkAt` at a pile or outcome head, and
  no card text reaches it. Decide whether it stays.

Standard constraints apply.
```

---

## Owner actions (not commits)

- The default workspace has untracked, gitignored leftovers from the retired v2 Lean emitters:
  `/home/msmorgan/Projects/deckmaste.rs/lean/Generated.lean`, `lean/Generated/`,
  `lean/GeneratedDefinitions.lean` and `lean/GeneratedDefinitions/`, all dated 2026-10-09 and
  covered by `.gitignore:25-31`. Nothing builds them. Deleting them is a change to the human's
  working copy, not a landing:
  `rm -r lean/Generated lean/Generated.lean lean/GeneratedDefinitions lean/GeneratedDefinitions.lean`
  in `/home/msmorgan/Projects/deckmaste.rs`. Once that is done, a later change may drop
  `.gitignore:25-31`. This ticket does not.

## Final acceptance

- `./lean/scripts/build` passes with zero warnings and no `sorry`.
- `cargo xtask lean check` passes.
- `cargo xtask cite check --list-noncompliant` prints nothing; `cargo xtask cite check` reports 0
  stale; `jj --no-pager diff --git | cargo xtask cite audit --diff` was run before each commit and
  each rule read against its claim.
- `kata kanban check` prints `OK`.
- The greps under C5, C6, C7, C8.2, C8.4 and C8.5 hold, and:
  - `grep -rn 'CorePins' lean/` prints nothing;
  - `grep -rn 'def actionResult\|def nounResult\|def nounZone\|olderClauseMaximum (rs' lean/Semantics/Proofs/`
    prints only `Proofs/Helpers.lean`;
  - `grep -rn 'isEmpty = false\|≠ \[\]' lean/Semantics/Proofs/` prints nothing that is a refusal
    check.
- The landing report lists every test added, rewritten and retired, with the reason for each
  retirement, taken from this spec.
