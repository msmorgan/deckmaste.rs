---
needs: [lean-v3-action-clause-spine]
---
# Replace the checker's Bindings stack with the Clean-Sheet DRT Checker

**Replace `Bindings`, positional stack addresses, `MoveProfile`/`StepProfile`,
and `countReach == 1` in `lean/Semantics/Check/` with the Discourse
Representation Structure (`DRS`) and the measured anaphor resolution rules
`J1`, `J2`, `D′`, and `AC>R1`.** Second implementation step of
[`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) (§1).
Subsumes the retired `lean-checker-binding-ids` hypothesis. Standard constraints
apply.

## Decisions already made (`docs/decisions/semantics-v3.md` §1)

- **Clean-sheet `DRS` replaces `Bindings = List Binding` (`semantics-v3.md` §1.2):**
  - Each discourse referent (`Referent`) carries a stable `id : ReferentId`
    (`Nat`), `idents : List BindingId`, `det : Determiner` (including `.part` for
    partitive sub-selections), `plur : Plurality`, `clause : Nat`, `role : GramRole`
    (`.subj | .obj | .other`), `pronounResolved : Bool`, and `payload : Payload`
    (retaining all 7 non-retired `Payload` variants: `.object`, `.player`,
    `.outcome` across all 15 `OutcomeSort`s, `.pile`, `.quality`, `.letter`, and
    `.turnRef`).
  - Delete `address : Option (List Nat)`, `shiftAddress`, `operandAddress`,
    `closeOperandAddress`, `bindingAt`, `setBindingAt`, `Payload.parameterFrame`,
    `Payload.masked`, `Payload.hidden`, `Refusal.unusedCapture`, and the
    `MoveProfile` / `StepProfile` pre-passes from `lean/Semantics/Check/`.
  - Coordinated `.and` / `.or` noun phrases introduce one `Referent` per
    conjunct and return `List ReferentId`; `firstMention id` attaches `id` to
    every referent in that list so `laterMention id` and post-action mutations
    apply to all coordinated referents `[CR#608.2c]`.
  - **Foregrounding on re-mention (`pendingRoles`) and `nextClause`:** Re-mentioning
    an existing referent in a direct verb argument slot (`.subj`, `.obj`, `.other`)
    buffers `(r.id, slotRole)` in `drs.pendingRoles` and applies `(r.clause :=
    drs.currentClause, r.role := slotRole)` at `nextClause` (never mid-clause, and
    never on possessor or nested modifier mentions).
  - **Zone-change identity clearing (`moveIntro`, `[CR#400.7,400.7j]`):** When a
    referent changes zone, clear `r.idents := []` on both the departed and
    post-move referents so any post-move `laterMention` of a pre-move parameter
    `BindingId` is deterministically refused.
- **Voice and `GramRole` assignment (`semantics-v3.md` §3.2):**
  - Active (`Subject.actor np`): `np` is evaluated first with `role := .subj`
    (and set as `drs.performer` for `action`); primary patient inside `action`
    gets `role := .obj`; oblique arguments get `role := .other`.
  - Unspecified-agent passive (`Subject.any`): introduces no agent referent;
    promotes the primary patient inside `action` to `role := .subj`.
  - Explicit-agent passive (`Subject.by np`): promotes the primary patient
    inside `action` to `role := .subj` and evaluates it **first**, then
    evaluates `np` second with `role := .other`.
- **Pre-action (`would`) vs. Post-action (`does` / `did` / imperative) stepping (`semantics-v3.md` §3.5):**
  - `StepPhase.preEvent` (`would` `[CR#614.1,614.6]`): introduces participants
    in their **pre-action** zone/status and prospective magnitudes (`thatMuch`,
    `thatMany`), without introducing post-completion product referents (such as
    drawn cards or created tokens).
  - `StepPhase.postEvent` (`does`, `did`, imperative): transitions patient
    referents to their **post-action** zone/status stamped with `.actedOn deed`
    (creating post-zone-change referents where `[CR#400.7,400.7j]` applies) and
    introduces post-completion product referents.
- **Explicit nested DRS boxes (`semantics-v3.md` §1.3–1.4):**
  1. Top-level imperative box: `a(...)` persists across sequential instructions
     in the paragraph `[CR#608.2c]`.
  2. Duplex / implicational box (`when`/`whenever`, `instead`/`would`, positive
     `if`/`did`): referents introduced in the antecedent box are accessible in
     the consequent box `[CR#603.2,614.1]`.
  3. Negated / modal prohibition box (`cant`, `not`, `unless`, `else` branch with
     respect to a negated condition): referents introduced inside are closed on
     exit and cannot be read by subsequent sentences `[CR#101.2]`.
  4. Distributive loop box (`doForEach`): introduces a singular loop referent
     (`plur = .one`, `role = .subj`) at quantifier header clause `C`, then
     increments `drs.currentClause` to `C + 1` before stepping `body`, closing
     the loop referent on exit while preserving mutations to outer referents.
  5. Relative-clause / comparison box (`Predicate.did` with `NounPhrase.which`,
     `Predicate.compareOver`): opens a subordinate clause box (`currentClause + 1`),
     seeding `which` from the outer `TypeEvidence` (`c.selfType` / conjunction
     evidence) and resolving `ownMember` inside `compareOver` to the outer
     candidate without `Window.top 1`.
  6. Definition Reference Scope (`enterOwnScope` / `inCaller` / `leaveOwnScope`,
     `semantics-v3.md` §1.4): `enterOwnScope` masks caller referents via
     `callerMask := some (drs.referents.map (·.id))`; `.inCaller` temporarily
     unmasks `callerMask` to evaluate `firstMention id arg` in the caller's DRS
     and removes the bound `List ReferentId` from `callerMask`; `leaveOwnScope`
     closes `r.det == .part` and definition-local `.player` scratch referents,
     unmasks caller referents with argument mutations preserved, and exports
     outcome referents (`.outcome`, `.pile`, `.quality`, `.letter`, `.turnRef`,
     and primary `.object` outcomes) with `clause := callerClause` and `role := .obj`.
- **Anaphor Resolution Rules (`semantics-v3.md` §1.5):**
  - **Accessibility (`J1`):** Filters unmasked referents by number agreement,
    sort, carrier-word zone `[CR#109.2,109.2a..109.2c,110.1]`, `<noun>` /
    `<verbed>` match, and consuming-verb domain/zone applicability
    `[CR#109.2,701.21a]`.
  - **Clause-mates (`J2`):** Co-arguments of the same verb (`r.clause ==
    drs.currentClause` at direct verb argument slots). Possessors (`controllerOf
    it`, `its power`) and small-clause objects (`counters on it`) are not
    clause-mates.
  - **Reflexives (`NounPhrase.refl (pl : Plurality)` — Principle A):** Exact
    complement of `J2`; requires an accessible clause-mate `.subj`
    (`r.clause == drs.currentClause && r.role == .subj`).
  - **Bare pronouns (`it`, `them`, `they` — Rule `D′`):** Excludes clause-mates
    (`J2`); picks the nearest clause (`max r.clause`), breaking ties by
    `GramRole` (`.subj < .obj < .other`); refuses with
    `Refusal.anaphor reach plur winners.length` on `0` or `≥ 2` winners; sets
    `r.pronounResolved := true` in `drs.currentClause`.
  - **Demonstratives (`that <noun>`, `those <nouns>` — Rule `AC>R1`):** Filters
    accessible referents by `<noun>`, excludes clause-mates (`A`) and any
    referent with `r.pronounResolved == true` in `drs.currentClause` (`C`), and
    picks the most recent remaining referent (`R1`, `max r.id`).
  - **Stamped participle phrases (`the exiled card`, `the sacrificed creature`):**
    Requires a unique accessible referent matching `<noun>`, plurality, and
    `.actedOn deed`.
- **Nothing `decide` cannot reduce:** All `DRS` operations stay on `List` with
  `DecidableEq`.

## Proof

- Targeted checker build gate passes cleanly: `lake build Semantics.Check`
  (covering `Check/FactTypes.lean`, `Check/Facts.lean`, `Check/Words.lean`,
  `Check/Refusal.lean`, `Check/PhraseRules.lean`, `Check/Phrase.lean`,
  `Check/EventContext.lean`, `Check/Events.lean`, `Check/Triggers.lean`,
  `Check/AbilityRules.lean`, `Check/Abilities.lean`, `Check/Card.lean`, and
  `Check/Rules.lean`).
- `shiftAddress`, `operandAddress`, `closeOperandAddress`, `bindingAt`,
  `setBindingAt`, `Payload.parameterFrame`, `Payload.masked`, `Payload.hidden`,
  `Refusal.unusedCapture`, `MoveProfile`, and `StepProfile` are gone from
  `lean/Semantics/Check/`.
