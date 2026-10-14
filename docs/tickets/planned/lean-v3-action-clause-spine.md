---
needs: [lean-drt-anaphora-refactor]
---
# Refactor Lean syntax onto the Semantics v3 Unified Action-Clause Spine

**Define every game action once as an `Instruction` and unify imperatives,
triggers, replacements, deontic rules, and lookback clauses around
`(subj : Subject, action : Instruction)`.** First implementation step of
[`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) (§3),
executed Lean-first in `lean/Semantics/`. Standard constraints apply.

## Decisions already made (`docs/decisions/semantics-v3.md` §3)

- **One verb representation (`Instruction`):** Every game action or verb phrase
  is represented once as an `Instruction` (bare infinitive verb phrase:
  `sacrifice(a(creature))`, `destroy(this)`, `attack(any)`, `block(this)`,
  `draw(2)`, `dealDamage(3, anyTarget)`, `putCounters(1, p1p1, this)`).
- **Six clause moods without double-wrapping:**
  1. **2p Imperative (`Instruction`):** bare `action` (implicit Actor is `"you"`
     or the enclosing `instruct` performer `[CR#109.5]`).
  2. **3p Imperative (`Instruction`):** `instruct (subj : NounPhrase) (action : Instruction)`
     (replaces `Instruction.act`; e.g., `instruct(target(opponent), sacrifice(a(creature)))`,
     `instruct(that(creature), explore)` `[CR#701.44a]`).
  3. **Trigger / Event (`GameEvent`):** `does (subj : Subject) (action : Instruction)`
     (collapses 20 of the 29 `GameEvent` constructors into `does`).
  4. **Replacement (`StaticSpec`):** `instead (event : WouldClause) (replacement : Instruction) ...`
     where `WouldClause` is `would (subj : Subject) (action : Instruction)` (or
     `wouldAny : List WouldClause` for disjunctive replacement events `[CR#614.1]`).
     Never double-wrapped in `does`.
  5. **Deontic (`StaticSpec`):** `cant`, `must`, `may`, and `gatedBy (cost : Cost)`,
     each taking `(subj : Subject) (action : Instruction) (mods : DeonticMods := {})`
     (where `DeonticMods` holds `bound : Option CountBound`, `asThough : Option AsThough`,
     `rider : DeonticRider`, `except : Option Cause` `[CR#702.13b,702.145b]`).
     Deletes `StaticSpec.deonticRule`, `Role`, `DeonticPatient`, `DeedComplement`,
     and `CoreDeed.haveCountersPutOn`.
  6. **Lookback (`Condition` / `Predicate` / `Amount`):** `DidClause` =
     `did (subj : Subject) (action : Instruction) (within : LookbackSpan)` where
     `LookbackSpan` is `.duration (d : Duration) (timing : Option Timing := none) | .thisWay | .triggering`.
     - `Condition.did (clause : DidClause)` replaces `Condition.happened` (dropping
       the unused `gap : Option Duration` field).
     - `Predicate.did (clause : DidClause)` replaces `Predicate.happenedTo`, using
       `NounPhrase.which` (replaces `NounPhrase.gap`) in the relativized argument
       slot (`did(which, attack(any), thisTurn)`, `did(any, exile(which), thisWay)`,
       `did(you, discard(which), thisTurn)`, `did(by(this), dealDamage(any, which), thisTurn)`).
     - `Amount.tally (clause : DidClause) (mode : TallyMode)` replaces
       `Amount.eventTally`, where `TallyMode` is `.events` (`"the number of times..."`)
       or `.quantity` (`"the number of cards..."` / `"the amount of life..."`).
- **Voice and `Subject` (`Subject.actor | .any | .by`):**
  - `Subject.actor (np : NounPhrase)` (coerced/elided from `NounPhrase`): active
    voice (`"Whenever you sacrifice a creature"` → `does(you, sacrifice(a(creature)))`).
  - `Subject.any` (`any`): unspecified-agent passive voice (`"Whenever this creature
    becomes blocked"` → `does(any, block(this))`, `"This permanent can't be
    sacrificed"` → `cant(any, sacrifice(this))`, `"If this creature would be
    destroyed"` → `would(any, destroy(this))`).
  - `Subject.by (np : NounPhrase)` (`by(np)`): explicit-agent passive voice
    (`"Whenever this creature becomes blocked by a creature"` →
    `does(by(a(creature)), block(this))`, `"This creature can't be blocked by
    Goblins"` → `cant(by(a(goblin)), block(this))`, `"creatures dealt damage by
    this creature this turn"` → `did(by(this), dealDamage(any, which), thisTurn)`).
- **Polymorphic `any : α` (`AnyValue` typeclass):**
  - Define `class AnyValue (α : Type) where any : α` in `lean/Semantics/Words.lean`
    with instances for `Subject`, `NounPhrase`, `Amount`, `CounterKindSource`,
    `DieSides`, `RollWatch` (`.anyResult`), `ZoneExpr`, and `Timing` (so e.g.
    `"can't have counters put on it"` is `cant(any, putCounters(any, any, this))`).
- **Retire `Window` and `withBindings` from syntax (`Words.lean`, `Phrase.lean`, `Events.lean`, `Abilities.lean`, `MacroParameters.lean`):**
  - Remove `Window` (`whole`, `parameter`, `top`, `below`, `introduced`,
    `outsideIntroduced`), `Instruction.withBindings`, `StaticSpec.withBindings`,
    `CaptureInput`, and `Amount.parameter` (`MacroParameters.lean:29`), including
    the `Window` field on `NounPhrase.pro` and `ChoiceMode.theirChoice`
    (`Events.lean:61`). Parameter identity uses `firstMention` / `laterMention`
    `BindingId`.

## Exact `GameEvent` Mapping (All 27 Domain Constructors Accounted For)

Retain the non-action `GameEvent` constructors specified in
[`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md) §3.6
(`does`, `beginningOf`, `endOf`, `now`, `stateHolds` `[CR#603.8]`,
`statBecomes`, `gameBecomes` `[CR#731.1]`, `chapterMark` `[CR#714.2]`,
`triggers`, `nthOccurrence`, `causes`, and `stateBasedActionsChecked`
`[CR#704.3,704.5h]`), collapsing all 17 action `GameEvent` constructors into
`does(subj, action)`:
- `verbedEvent` (including `Deed.ofAbility`: `crew`, `saddle`, `mutate`, `devour`,
  and optional `becomes : Option Predicate`, `locus : Option ZoneExpr`),
- `casts` (with `from_ : Option EventSource`), `activates` (with `of_`),
  `tappedForMana`, `commitsCrime` `[CR#700.13]`, `losesGame`,
  `paysCost` (with `outcome : PaymentOutcome := .paid`) + `paysLife`,
- `attacksWith` (`attackWith(attackers, defender)`), `combat` (`attack`, `block`,
  `attackUnblocked` `[CR#509.1h]`),
- `becomesTarget` (adding `Instruction.target (subject : NounPhrase)` `[CR#115.1]`,
  shared with deontic shroud/hexproof `cant(by(src), target(this))`),
- `zoneChange` (`MoveMode` carries `from_ : EventSource` and `at : ObservationPoint`
  `.arrival | .departure` `[CR#603.6a,603.6c,603.10a]`),
- `damage` (`dealDamage` with `kind : DamageKind`), `lifeChange` (`changeLife`),
  `draws` (`draw`), `searchesLibrary` (`search`), `createsTokens` (`create`),
- `counterEvent` (`putCounters` / `removeCounters` carrying `CounterBatch`,
  including `.emptying` `[CR#310.12b,702.62a]`, keeping `Instruction.moveCounters`
  distinct for moving counters between two objects `[CR#122.5]`),
- `statusEvent` (`setStatus`), `attaches` (`attach`), `gainsControl`
  (`changeControl`), `rollsDice` (`rollDice` with `DiceBatch` and `RollWatch`),
  and `flipsCoin` (`flipCoin` with `Option FlipCall`).

## Folded Residues

- **From `plugins-v2-menace-bound-helper` (`[CR#702.111b]`):** Menace is
  `"can't be blocked except by two or more creatures"`, which uses `DeonticMods.bound`
  / `except`. Today `CountBound.moreThan(1)` carries `1` where the text prints
  `"two or more"`; consider adding `CountBound.atLeast` in `Words.lean` so
  menace writes its printed bound directly.
- **From `lean-constructor-collapse`:**
  - Fold `RollWatch.anyResult` into `AnyValue.any`.
  -Represent `"was cast"` (`Predicate.wasCast`) through `Predicate.did (did(any, cast(which, any), ...))` (preserving optional caster and source distinctions).
  - Remove `StatusCat` and `Status.category` if still unused in `Check/`; move
    `ManaUnit` to the template/macro layer if `scaledMana` eliminates it.
- **Related (`lean-core-as-event-instructions`):** Keep the local fold of
  `entryChoice` and `attachmentChoice` self-contained without blocking on a
  general as-event instruction.

## Proof

- Targeted syntax build gate passes cleanly:
  `lake build Semantics.Words Semantics.Phrase Semantics.Events Semantics.Triggers Semantics.Abilities Semantics.Rules Semantics.MacroParameters`
  (downstream `Check/`, `Macros/`, `Proofs/`, and `Cards/` modules are updated
  in `lean-v3-drt-checker`, `lean-v3-definitions-and-templates`, and
  `lean-v3-proofs-and-cards`).
- `GameEvent`, `StaticSpec`, `DidClause` (`did`), `Subject`, `AnyValue`, and
  `NounPhrase.which` match `semantics-v3.md` §3, and `Window` and `withBindings`
  are gone from the syntax modules.
