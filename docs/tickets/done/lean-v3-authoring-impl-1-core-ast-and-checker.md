---
needs: []
---
# Refactor the Lean v3 Core AST, Location algebra, and checker for the authoring surface

**Land all Core AST, `Location` algebra, deontic/verb-spine primitive, relational
`isOpponentOf`, and `CallerScope`/`Authoring` changes across `lean/Semantics/` in
a single pass so `Abilities.lean` and `Check/Phrase.lean` are refactored once.**
Implementation ticket 1 of
[`lean-v3-authoring-surface`](lean-v3-authoring-surface.md). Standard
constraints apply.

## Scope

1. **Core AST Rename (`Instruction` → `Action`, `lean-v3-authoring-surface.md` §1):**
   - Rename `Instruction` to `Action` across `lean/Semantics/Abilities.lean`,
     `Check/*.lean`, `Macros/`, `Proofs/`, and `Cards/`.
2. **Unified `Location` Algebra (`lean-v3-authoring-surface.md` §3):**
   - Delete `ZoneExpr`, `Pile`, `EventSource`, `SearchScope`, and `ZoneScope`.
   - Introduce `Location` (`inCaller`, `zone`, `fromEnd`, `exiledWith`, `pile`,
     `anyLocation`, `not`, `or`) with `AnyValue Location := ⟨.anyLocation⟩`.
   - Collapse `Predicate.inZone`, `Predicate.inPile`, and `Predicate.exiledWith`
     into `Predicate.in_ (loc : Location)`.
   - Delete `PilePlacement.nthFromEnd` and `PilePlacement.orOtherEnd`, and
     flatten `EntryRider` and the remaining `PilePlacement` constructors
     (`inOrder`, `faceDown`, `withCounters`) into `MoveRider`.
   - Drive LKI (`historicalMoveFrom := some .battlefield`) and `putInto`
     departure-vs-arrival classification (`[CR#603.6c,603.10a]`) from
     `Location.positiveZones?`.
3. **Deontic & Quantifier Primitives (`lean-v3-authoring-surface.md` §2):**
   - Rename `StaticSpec.may` → `StaticSpec.can` (`[CR#101.2]`), reserving `may`
     for optional one-shot actions (`Action.withContinuation .optional`,
     `[CR#603.5]`).
   - Delete `DeonticRider` (`stateBased | noRider | play`) and
     `DeonticMods.bound`; `DeonticMods` carries `(timing : Option Timing := none)`,
     `(asThough : Option AsThough := none)`, and
     `(except : Option DeonticExcept := none)`.
   - Add `Quantity.moreThanOf (amount : Amount)` and
     `Quantity.additional (quantity : Quantity)`.
   - Replace `AsThough.of (predicate : Predicate)` with
     `AsThough.condition (cond : Condition)`.
4. **Relational `isOpponentOf` and `isTeammateOf` (`lean-v3-authoring-surface.md` §5.2):**
   - Replace nullary `Predicate.opponent` with relational
     `Predicate.isOpponentOf (who : NounPhrase)` (`[CR#102.2..102.3]`) and
     `Predicate.isTeammateOf (who : NounPhrase)` (`[CR#102.3]`).
   - Retire `PlayerGroupWord` (`allPlayers | yourOpponents | yourTeam`) and
     `NounPhrase.playerGroup` in favor of `allOf isPlayer`, `allOf isOpponent`,
     and `or [you, a isTeammate]`.
5. **Retire `v1`/`v2` Holdover Names & Amend Glossary (`lean-v3-authoring-surface.md` §6.3):**
   - Rename `lean/Semantics/MacroParameters.lean` (`MacroExpansion`,
     `MacroSplice`) and `lean/Semantics/Proofs/MacroParameters.lean` to
     `lean/Semantics/CallerScope.lean` (`CallerScope`, `CallerSplice`) and
     `lean/Semantics/Proofs/CallerScope.lean`.
   - In `lean/Semantics/Authoring.lean`, retire raw `semantic_macro` and
     `macroAttribute` so only `semantic_definition` and `semantic_template`
     exist; rename `MacroContext` → `AuthoringContext`,
     `contextualMacroAttribute` → `contextualAuthoringAttribute`,
     `macro_splice_type%` → `caller_splice_type%`, and
     `Form.onlyMacros` / `Forms.onlyMacros` → `Form.onlyAuthored` /
     `Forms.onlyAuthored`.
   - Delete empty shims `lean/Semantics/Phrase.lean` and
     `lean/Semantics/Triggers.lean`; fold `Check/Triggers.lean`,
     `Check/EventContext.lean`, and `Check/PhraseRules.lean` into
     `Check/Phrase.lean`.
   - Delete `abbrev Binding := Referent` and `abbrev Bindings := DRS` in
     `lean/Semantics/Check/Words.lean`; rename `Payload.gap`, `Referent.isGap`,
     `gapB`, `countGaps`, and `Refusal.gapInScope` → `Payload.difference`,
     `Referent.isDifference`, `differenceB`, `countDifferences`, and
     `Refusal.differenceInScope`.
   - Amend `docs/contexts/game-model/CONTEXT.md` to replace **Semantic Macro**
     with **Definition** and **Template** (`[CR#701.1,702.1]`), listing
     `_Avoid_: Semantic Macro, macro`.
6. **Downstream Build Continuity:**
   - Update `Macros/`, `Proofs/`, and `Cards/` minimally so the workbench builds
     cleanly at the end of this ticket.

## Proof

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `grep -rn "ZoneExpr\|EventSource\|SearchScope\|ZoneScope\|DeonticRider\|PlayerGroupWord\|semantic_macro\|MacroContext\|MacroExpansion\|MacroSplice\|abbrev Binding" lean/Semantics/`
  returns zero matches.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
