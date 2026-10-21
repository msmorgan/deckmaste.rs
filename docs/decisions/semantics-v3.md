# ADR: Semantics v3 — Clean-Sheet DRT Checker, Definition/Template Split, and Unified Action-Clause Spine

- **Status:** Promoted (accepted by owner 2026-10-13 / 2026-10-14)
- **Supersedes (archived in `docs/archive/decisions/`):**
  - `docs/archive/decisions/semantics-v2.md`
  - `docs/archive/decisions/oracle-text-is-forward-anaphoric.md`
  - `docs/archive/decisions/idris-is-a-soundness-gate.md`

---

## 0. Layer Contract, Charter, and Execution Strategy

### 0.1 Standing Layer Contract and Forward-Anaphoric Reading Order

1. **In-situ surface constructions (no prenex hoisting):** Semantics v3 encodes the Magic card grammar as a reduced, surface-faithful form of English translated from the `deckmaste_english_v4` AST ([`semantics-spelling-lowering.md`](./semantics-spelling-lowering.md) §17; [`english-v4.md`](./english-v4.md)). Constructions occupy their surface positions: no prenex target slot lists, no hoisted binders, and no positional slot reads. All rearrangement, hoisting, and register indexing belong to lowering (`deckmaste_lowering_v3` → `deckmaste_core`, [`core-explicit-regions.md`](./core-explicit-regions.md)).
2. **Strictly forward-anaphoric reading order:** Every constructor argument position is checked in the discourse context of everything textually earlier; constructor argument order is textual order, and `DRS` state threads left to right. There is no cataphora, apart from the clause-internal Contrast Anchor of step 2 below (owner ruling 2026-10-21), and no out-of-band target/announce channel:
   - **A bare `"another"` / `"other"` contrasts with a Contrast Anchor, not necessarily with an earlier target (owner ruling 2026-10-21):** `"another target creature you control"` in reconfigure (`[CR#702.151a]`) and Olivia Voldaren's `"deals 1 damage to another target creature"` (the example under `[CR#707.11]`) have no earlier target; they exclude the source. `Predicate.other` (`isOther`) is the only encoding of a bare printed `"another"` / `"other"` (`"any other target"` is `isAnyOtherTarget`, `and [isAnyTarget, isOther]`); `Predicate.otherThan <NP>` is reserved for a printed `"other than <NP>"`. The checker resolves the anchor in this order (owner rulings 2026-10-21). Every anchor must be compatible with the head in kind and card type, so a creature-headed `"another"` cannot anchor on a source whose card types do not include creature:
     1. the nearest earlier mention of a compatible kind in scope, target or not (`"Choose a player. Each other player…"`); recency is the DRS's, so on Arc Trail (`"deals 2 damage to any target and 1 damage to any other target"`) the first target beats the self-mention. The thing `"another"` describes, including the subject of the `is` condition it is predicated of, is never its own anchor, so backup's `"If that's another creature"` (`[CR#702.165a]`) contrasts with `"this creature"`, not with `"that creature"`;
     2. failing that, a compatible participant in the same clause or event, read forward: when `"another <X>"` is the agent of a `would`, damage-rule, or event clause, that clause's compatible recipient or object is the anchor. Inquisitor's Flail's `"If another creature would deal combat damage to equipped creature, it deals double that damage to equipped creature instead"` contrasts with the equipped creature, so a redirection effect (`[CR#614.9]`) that makes the equipped creature deal combat damage to itself is doubled once by the Flail's first ability, not twice (`[CR#614.1a,616.1f]`);
     3. otherwise the source, for an object head, or `you`, for a player head (`[CR#109.5]`).
   - A delayed trigger or continuous effect reads previously announced targets as settled particulars (`[CR#603.7c]`). They are not its own targets, which are chosen per ability (`[CR#603.3d,601.2c]`), but they are earlier mentions in scope, so under step 1 they can be the Contrast Anchor of an `"other"` in it.
   - Across an activated ability's colon (`cost : effect`), the effect reads cost referents filtered to public current zones (`[CR#400.2,400.7j]`).
   - **A moved referent is read back by anaphora (`[CR#400.7,400.7j]`):** When a `Definition` or `Template` body moves a parameter's referent to another zone, the object in the new zone is a new object; subsequent clauses read it back by anaphora (`it`, `that card`, `"each card exiled this way"`), never by `laterMention` of the pre-move parameter.
3. **Checker Charter (faithful English reading, not runtime engine simulation):** The Lean checker (`lean/Semantics/Check/`) verifies that a written semantic term is a faithful reading of the card's English: its discourse referents, pronouns, demonstratives, outcomes, choices, stamps, reference scopes, plurality, and `instruct` performers resolve, its terms are well-kinded, and its card frame (`cardLine` through `prototypeFrame`) is well-formed. It is not a runtime rules engine (`[CR#101.3,609.3]`).

### 0.2 Motivation for Semantics v3

During the `semantics-v2` work and the anaphora measurement in `semantics-v2-anaphor-resolution-heuristics`, three structural invariants converged:

1. **Piecemeal DRT reinvention in the checker:** `lean/Semantics/Check/` inherited its positional `Bindings = List Binding` stack (`address : Option (List Nat)`, `shiftAddress`, `operandAddress`, `closeOperandAddress`, `Payload.parameterFrame`, `Payload.masked`, `Payload.hidden`), `Window` (`whole`, `parameter`, `top`, `below`, `introduced`, `outsideIntroduced`), `withBindings` / `CaptureInput`, and two-pass `MoveProfile` / `StepProfile` machinery from the Idris prototype. Meanwhile, the measured anaphor rules accepted by the owner (`D′` for bare pronouns `it`/`them`/`they`, and `AC>R1` for demonstratives `that <noun>`/`those <nouns>`) require explicit Discourse Representation Theory (DRT) constructs: stable discourse-referent identities, clause boundaries, grammatical roles (`.subj | .obj | .other`), pronoun-resolution tracking within the current clause, and nested DRS accessibility boxes.
2. **Conflation of rules-defined `Definition`s and syntactic `Template`s:** `plugins_v2/` and `lean/Semantics/Macros.lean` treated keyword actions/abilities (`[CR#701,702]`) and syntactic Oracle phrasing helpers as the same mechanism, inferring `ownScope` / `inCaller` from directory paths or omitting them on hand-written Lean macros (`31` of the `48` alignment drifts in `lean-macros-cutover`).
3. **3–4× verb duplication across clause moods:** Every game action was encoded separately as an imperative `Instruction`, a triggered/replacement `GameEvent` (`21` of `29` constructors), a deontic `StaticSpec.deonticRule` (`Deeds` + `Role` + `DeonticPatient`), and a historical `LookbackClause` (`Condition.happened`, `Predicate.happenedTo`, `Amount.eventTally`), each with its own subject/patient plumbing and checker pass.

### 0.3 Honest `v3` Naming, Lean-First Execution, and Crate Fates

Rather than retrofitting these changes incrementally onto `semantics_v2` and `plugins_v2/` in lockstep with `macro_ron`, **Semantics v3** (`semantics-v3`) is executed **Lean-first**:

1. **Phase 1 — Lean-Only Specification and Proofs (`lean/Semantics/`):**
   - To avoid migrating downstream Lean files three times across intermediate tickets, Ticket 1 (`lean-v3-action-clause-spine`) relocates the `v2` Lean modules into an inert reference directory `lean/archive/SemanticsV2/` (outside `lean/lakefile.toml`'s `roots` and `defaultTargets`) and activates `lakefile.toml` targets progressively layer by layer:
     1. `lean-v3-action-clause-spine`: syntax layer (`roots = ["Semantics"]`, `defaultTargets = ["Semantics"]`).
     2. `lean-v3-drt-checker`: adds `"Semantics.Check"` to `roots`.
     3. `lean-v3-definitions-and-templates`: adds `"Semantics.Macros"` and `"Semantics.Macros.Declared"` to `roots`.
     4. `lean-v3-proofs-and-cards`: ports `SemanticsCards` and `SemanticsProofs` once from `lean/archive/SemanticsV2/`, restores `defaultTargets = ["Semantics", "SemanticsCards", "SemanticsProofs"]`, and **deletes `lean/archive/SemanticsV2/`**.
2. **Phase 2 — `deckmaste_semantics_v3` and `plugins_v3/` Cutover:**
   - Once the Lean specification and proof suite are settled and green, `semantics-v3-dsl-and-plugins` creates `crates/deckmaste_semantics_v3` and `plugins_v3/` mirroring the proven `lean/Semantics/` AST and `Definition` / `Template` declarations, re-points `xtask` (`lean`, `facts`) at `v3`, and **immediately deletes `crates/deckmaste_semantics_v2` and `plugins_v2/canon/`** (retiring `plugins_v2/builtin/` once `deckmaste_lexical_source` / `deckmaste_macro_def` is repointed at `plugins_v3/builtin/`).
   - Evaluate replacing `macro_ron` with a purpose-built `v3` DSL parser (`deckmaste_dsl_v3`) if cleaner for `Definition(...)` / `Template(...)` headers, polymorphic `any`, bare literals, and positional/named arguments without RON's structural workarounds.
   - Build `crates/deckmaste_lowering_v3` (`semantics-v3` → `deckmaste_core`), pass the `semantics-v3-parity` ratchet and slow-test suite, and execute `semantics-v1-cutover`.
   - **Deletion-bound:** `lean/archive/SemanticsV2/` is deleted at `lean-v3-proofs-and-cards`; `deckmaste_semantics_v2` and `plugins_v2/canon/` are deleted at `semantics-v3-dsl-and-plugins`; `plugins_v2/builtin/` is deleted once `deckmaste_lexical_source` is repointed at `v3`; `v1` (`deckmaste_semantics`, `deckmaste_lowering`, `deckmaste_legacy_render`, `deckmaste_plugin` with `deck.rs` and `provenance.rs` rehomed, `idris/`, `plugins/builtin`, `plugins/canon`) is deleted at `semantics-v1-cutover`. Never copy `v2`/`v3` work back into `v1`, and add no new `v1` registries.

---

## 1. Pillar 1: Clean-Sheet DRT Checker (`lean/Semantics/Check/`)

### 1.1 Retired `Bindings` and `Window` Machinery

The following Idris-ported structures and passes are **deleted** from `lean/Semantics/Words.lean`, `lean/Semantics/Phrase.lean`, `lean/Semantics/Events.lean`, `lean/Semantics/MacroParameters.lean`, and `lean/Semantics/Check/`:

- `Bindings = List Binding` with positional stack indices (`address : Option (List Nat)`), `shiftAddress`, `operandAddress`, `closeOperandAddress`, `bindingAt`, and `setBindingAt`.
- `Payload.parameterFrame`, `Payload.masked`, and `Payload.hidden`.
- `Instruction.withBindings`, `StaticSpec.withBindings`, `Cost.withBindings`, `CaptureInput`, `Amount.parameter` (`MacroParameters.lean:29`), and `Window` (`whole`, `parameter`, `top`, `below`, `introduced`, `outsideIntroduced`), including `Window` on `NounPhrase.pro` and `ChoiceMode.theirChoice` (`Events.lean:61`).
- `Refusal.unusedCapture`: a `Definition` parameter wrapped in `firstMention` that is mentioned only once (or moved in Step 1 and read back by anaphora in Step 2, like `airbend` `[CR#701.65a]`) is legal and never triggers an unused-capture refusal.
- The nine hand-coded window macros in `lean/Semantics/Macros.lean` (`itPrior`, `itCondSubject`, `attachToIt`, `requireBlockIt`, `agentRef`, `itsOther`, `ownSubject`, `controllerSacrifices`, `lookedCards`).
- The `MoveProfile` and `StepProfile` pre-passes that pre-computed positional stack shifts before stepping instructions.

### 1.2 Discourse Referents (`ReferentId`, `Referent`, `OwnScopeFrame`, `DRS`)

Every mention that introduces an entity, quantity, choice, or outcome into discourse creates a `Referent` with a stable `ReferentId` in the current Discourse Representation Structure (`DRS`):

```lean
abbrev ReferentId := Nat

inductive GramRole where
  | subj   -- Clause subject (active actor, or promoted patient in a passive clause)
  | obj    -- Direct object / primary patient of an active clause
  | other  -- Oblique argument (destination, source, instrument, passive `by`-agent, etc.)
  deriving DecidableEq, Repr

structure Referent where
  id              : ReferentId
  idents          : List BindingId   -- Parameter identities bound via firstMention
  det             : Determiner       -- Includes `.part` for partitive sub-selections
  plur            : Plurality
  clause          : Nat              -- Clause index where introduced or last foregrounded
  role            : GramRole         -- Grammatical role in that clause
  pronounResolved : Bool             -- True if a bare pronoun (`it`/`them`/`they`) resolved to this referent in `currentClause`
  payload         : Payload          -- One of the 7 non-retired Payload variants (.object, .player, .outcome, .pile, .quality, .letter, .turnRef)
  deriving DecidableEq, Repr

structure OwnScopeFrame where
  preEntryIds  : List ReferentId     -- Immutable snapshot of all ReferentIds present before entering this ownScope
  maskedIds    : List ReferentId     -- Currently masked ReferentIds in this ownScope (preEntryIds minus performer and unmasked parameter IDs)
  callerReferentIds : List ReferentId := [] -- Arguments evaluated in caller scope and their successors
  callerClause : Nat                 -- Caller's currentClause saved on entry
  deriving DecidableEq, Repr

structure DRS where
  referents     : List Referent
  nextId        : ReferentId := 0
  currentClause : Nat := 0
  performer     : Option ReferentId := none
  scopeStack    : List OwnScopeFrame := []           -- Stack of active Definition reference scopes (head = innermost ownScope)
  pendingRoles  : List (ReferentId × GramRole) := [] -- Direct-argument mentions in currentClause (consulted by J2 and reflexives)
  deriving DecidableEq, Repr
```

- **All 7 Domain `Payload` Variants in `DRS.referents`:** `Payload` retains `.object` (domain/kind, zone, `Stamp`, origin, size, possessor), `.player`, `.outcome (sort : OutcomeSort)` (all 15 `OutcomeSort`s for `thatMuch`, `thatMany`, `manaProduced`, `rollOutcome`, and `.damageMarked` `[CR#120.3d]`), `.pile`, `.quality` (`chosen` color/type/creature type/name), `.letter` (defined variable `X`), and `.turnRef` (`that turn`), while deleting `.parameterFrame`, `.masked`, and `.hidden`. Partitive sub-selections are identified by `r.det == .part`.
- **Coordinated Noun Phrases (`.and` / `.or`):** Evaluating a conjunctive noun phrase (`.and [target artifact, target enchantment]`) threads introductions in order, introducing one `Referent` per conjunct into the active DRS and returning `List ReferentId`. Evaluating a disjunctive noun phrase (`.or`, e.g. `"you or a white creature you control"` on *Mangara's Equity*) steps each branch independently from the incoming DRS and exports only referents common to every branch (`commonHeaderContext`) plus a joined head referent when applicable, so branch-local indefinites do not leak into the outer DRS as unconditional referents. When bound to a parameter via `firstMention id`, `id` is attached to every exported member of `List ReferentId` so `laterMention id` and post-action zone/status mutations update all coordinated referents uniformly (`[CR#608.2c]`).
- **Direct-Argument Tracking (`pendingRoles`), Clause-Mates, and Clause Advancement (`nextClause`):**
  - When a referent `r` is introduced or re-mentioned in a **direct verb argument slot** (`.subj`, `.obj`, or `.other`), the checker records `(r.id, slotRole)` in `drs.pendingRoles`.
  - Possessor mentions (`its controller`, `ownerOf(it)`, `its power`, `"Target creature's"`, `"that creature's owner"`) and nested modifier mentions do **not** record in `drs.pendingRoles` (so `J2` does not exclude them as clause-mates of the verb), while a possessor introduction or re-mention (`inArgSlot = false`) sets `r.clause := drs.currentClause` and `r.role := .other` so subsequent bare pronouns in the same clause see the foregrounded possessor (`D′`).
  - **Both `J2` (clause-mate exclusion) and Principle A reflexives (`itself`, `themselves`, `yourself`) consult `drs.pendingRoles` as well as `r.clause == drs.currentClause`:**
    - If a previously introduced creature `r` is re-mentioned as the `.subj` of a new clause (`"It deals 2 damage to itself"` / `"It deals 2 damage to it"`), evaluating `"It"` in the `.subj` slot records `(r.id, .subj)` in `drs.pendingRoles`. When the `.obj` slot is evaluated in the same clause, `r` is recognized via `drs.pendingRoles` as the current clause's `.subj`—so `"itself"` succeeds (`Principle A`) and `"it"` excludes `r` as a clause-mate (`J2`).
    - Conversely, in `"Target creature's controller sacrifices it"`, `"Target creature"` is introduced inside a possessor NP (not a direct verb argument slot, so `(r_creature.id, _)` is not in `drs.pendingRoles`, whereas `(r_controller.id, .subj)` is), so `"it"` in the `.obj` slot does not treat `r_creature` as a clause-mate and resolves to `r_creature`.
  - At the end of the clause (`nextClause`), buffered `pendingRoles` updates are committed to `drs.referents` (`r.clause := drs.currentClause`, `r.role := slotRole`), `drs.pendingRoles` is cleared, `drs.currentClause` is incremented, and `pronounResolved := false` is reset on all referents.
  - Until then, `D′` already ranks a buffered referent at `drs.currentClause`, by the best of its buffered slot roles (`DRS.rankingClause`, `DRS.rankingRole`; owner ruling 2026-10-21). A pronoun in a direct argument slot never sees the difference, because `J2` excludes that referent as a clause-mate first. A possessor or amount pronoun does: in *Karplusan Yeti*'s "That creature deals damage equal to its power to this creature", "its" reads the re-mentioned subject "that creature", not the earlier clause's subject.
- **Zone-Change Identity Clearing (`moveIntro`, `[CR#400.7,400.7j]`):** When an object referent changes zone (`moveIntro`), the post-move referent receives a fresh `ReferentId` with `r.idents := []` (and `r.idents` is cleared on the departed referent), so any illegal post-move `laterMention` of the pre-move parameter `BindingId` is deterministically refused by the checker.
- **Decidability contract and `by decide` reduction budget:** `DRS` uses `List` and decidable equality throughout (`no` `Std.HashMap`), preserving `by decide` across all Lean proofs and `cargo xtask lean check`. Before re-spelling the full proof and card bench in `lean-v3-proofs-and-cards`, `lean-v3-drt-checker` measures kernel `by decide` reduction on a representative multi-clause card and multi-step definition (`airbend`, `connive`, `explore`) to verify fast per-declaration reduction with zero `maxRecDepth` or `maxHeartbeats` increases.

### 1.3 Nested DRS Boxes and Quantifier Polarity

In DRT (Kamp & Reyle 1993), the logical force and forward accessibility of an indefinite (`a(creature)`, `bare(creature)`) depend on the **DRS box** in which it is introduced, not on separate determiner constructors:

| DRS Box | Introducing Constructions | Indefinite (`a(...)` / `bare(...)`) Semantics | Accessibility Out of Box |
|---|---|---|---|
| **1. Top-Level Imperative & Activated Cost→Effect Box** | Spell abilities, activated/triggered effect instructions (`sacrifice(a(creature))`), and `Cost.perform(action)` across `cost : effect` (`Ability.activated`, `StaticSpec.specialAction`) | Action-time existential choice (`[CR#608.2c,602.2b]`) | Accessible to all subsequent clauses in the same paragraph (`drs` threads forward). Across an activated ability's colon (`cost : effect`), `Cost.perform(action)` steps `action` in `StepPhase.postEvent` (with `drs.performer` = activator), filters cost-introduced referents to public current zones (`publicOnly`, `[CR#400.2,400.7j]`), and runs `nextClause` at `:` so `"Sacrifice a creature: Draw cards equal to its power"` resolves `"its"`, `"that creature"`, and `"the sacrificed creature"` via `D′`, `AC>R1`, and `.actedOn`. |
| **2. Duplex / Implicational Box (`K₁ ⇒ K₂`)** | Trigger event → effect (`when`/`whenever`), replacement event → replacement (`replace(would(...), instead(...))`), positive conditional → consequent (`if(did(...), thenBranch)`) | Universal quantification (`∀`) over matching events and participants (`[CR#603.2,614.1]`) | Referents introduced in the antecedent box `K₁` are **accessible** inside the consequent box `K₂` (`"Whenever a creature enters, it gets..."`, `"If you sacrificed a creature this turn, return it..."`), and closed after `K₂`. In an `if/then(/else)` instruction (`doIf` / `doOnlyIf`), announced targets (chosen at stack announcement `[CR#601.2c,115.1]`) and mutations to pre-existing outer referents survive in the enclosing DRS, while branch-local indefinite introductions are closed on exit. |
| **3. Negated / Modal Prohibition Box (`¬K` / `¬◇K`)** | Deontic prohibition (`cant(subj, action)`), negated condition (`not(...)`, which "unless" writes), `else` branch of `if(cond, thenBranch, elseBranch)` with respect to `cond` | Universal prohibition ($\neg\Diamond\exists x = \Box\forall x\,\neg$, `[CR#101.2]`) | Referents introduced inside a negated or `cant` box are **closed** on exit and inaccessible to following sentences (nor accessible in `elseBranch` from a negated `cond`). A definite source reference in a negated property test (`"if this enchantment is not a creature"`) remains available; the negated predicate introduces no accessible participants or positive type refinements. In a prohibition, announced targets and definite source or attachment-host references are established in the enclosing ability scope and remain accessible by their original IDs; hypothetical indefinite or universal participants introduced only by the prohibited action remain closed. An explicit `by` phrase still denotes an agent, never an attachment host. |
| **4. Distributive Loop Box (`∀x ∈ S : K`)** | `doForEach(collection, body)` | Universal distribution over the bound collection; introduces a singular loop referent (`plur = .one`, `role = .subj`) at quantifier header clause `C`, then advances `drs.currentClause` to `C + 1` before stepping `body` | Because the loop header is at clause `C` and `body` starts at clause `C + 1`, `it` in a direct verb argument slot inside `body` (e.g., `cast(it)` in `airbend`) is not excluded as a clause-mate (`J2`) and uniquely selects the loop referent (`clause = C`) over any outer antecedent (`clause < C`). Loop-local referents are closed on exit while mutations to outer referents survive, and plural abstraction ($\Sigma X$) exports plural aggregates for loop-internal target announcements and token creations (see below). |
| **5. Relative-Clause / Comparison Box** | `Predicate.did` (`did(..., which, ...)`), `Predicate.compareOver` (`comparesOwnStat`) | Binds `which` (or the comparison candidate) in a subordinate clause box (`currentClause + 1`) | Local to the relative/comparison clause; `ownMember` inside `compareOver` resolves to the outer candidate without needing `Window.top 1`; closed on exit, while definite presuppositional references (`attachHost`: `"enchanted"`, `"equipped"`, `"fortified"`, `[CR#301.5,301.6,303.4b]`) project to the enclosing ability's DRS box. |
| **6. Definition Reference Scope (`ownScope` / `inCaller`)** | Every `Definition` invocation (`[CR#701,702]`) | Evaluates arguments in caller's box (`inCaller`), masks caller referents during body execution, exports mutations and primary outcome on exit | See §1.4 below. |

**Plural Abstraction ($\Sigma X$) on Distributive Loop Exit (`doForEach`):**
- In DRT (Kamp & Reyle 1993), exiting a distributive quantifier box (`doForEach`) closes the singular loop variable and fresh local indefinites, and abstracts plural discourse referents (`plur = .many`, with no local `BindingId` or exact member count) over:
  1. **Explicit target selections (`r.det == .target` inside `Instruction.choose`):** Allocates a plural target referent per target announcement from the announcement's post-iteration zone, type, deed stamps, and origin (so *Vaevictis Asmadi, the Dire* reads `"those permanents"` after choosing a target permanent for each player).
  2. **Post-event object creations (`Origin.token` with `.actedOn (.action "create")` in `StepPhase.postEvent`):** Allocates a plural token referent retaining the created objects' post-iteration zone, types, and stamps (so *Hate Mirage* reads `"those tokens"` after creating a copy for each target, while mere `isToken` predicate filters or `StepPhase.preEvent` descriptions do not).
  Both are derived directly from `Referent`'s existing `det`, `payload.origin`, and `.actedOn` stamps without one-purpose boolean flags on `Referent`.
- **Partitive Sub-Selection and Ceiling Shortfall:** Partitive choices (`r.det == .part` from `choose` / `someOf`, consulted by `Predicate.notChosen` for `"not chosen this way"`) and `.outcome .ceilingShortfall` (from `"draw up to N cards"`) are tracked as standard `DRS` referents and `.outcome` payloads.

### 1.4 Definition Reference Scope (`enterOwnScope`, `inCaller`, `leaveOwnScope`)

In the AST, a `Definition` body is wrapped in `Instruction.ownScope (deed : Option Deed := none) (body : Instruction)` (with `firstMention id (NounPhrase.inCaller arg)` around each first-mentioned `NounPhrase` parameter and `laterMention id (NounPhrase.inCaller arg)` around each later mention: the Binding Identity is the `Definition`'s own and outermost, and only the argument is read in the caller). Carrying `deed : Option Deed` (e.g., `some (.action "scry")`, `some (.action "forage")`, `some (.action "destroy")`) on `Instruction.ownScope` preserves the keyword action's `Deed` identity alongside its expanded body for both `does`/`would`/`did`/`cant` and `deckmaste_lowering_v3` (`core::Action::Composite` `[CR#701.22b]` and `core::EventFilter::Act`). Nested `Definition` calls (e.g., `connive` calling `draw` and `discard`, or `forage` calling `exile` and `sacrifice`) push and pop `OwnScopeFrame` records on `drs.scopeStack`:

1. **Scope Entry (`enterOwnScope` on entering `Instruction.ownScope`):**
   - Captures `preEntryIds := drs.referents.map (·.id)` (an immutable snapshot of all referents existing prior to this `Definition` call) and `maskedIds := preEntryIds.filter (some · != drs.performer)`.
   - Pushes `{ preEntryIds, maskedIds, callerClause := drs.currentClause }` onto `drs.scopeStack` and advances `drs.currentClause` (`drs.currentClause + 1`).
2. **Caller-Scoped Argument Evaluation (`NounPhrase.inCaller`):**
   - When `.inCaller arg` is encountered while `drs.scopeStack = topFrame :: outerFrames`, the checker temporarily pops **only `topFrame`** (`drs.scopeStack := outerFrames`) to evaluate `arg` in the immediate caller's visibility (so any referents masked by an enclosing `outerFrames` definition remain masked!).
   - The enclosing `firstMention id (inCaller arg)` then attaches `id : BindingId`, in the body's own scope, to all `ReferentId`s (`List ReferentId`) `arg` introduced or resolved; `laterMention id (inCaller arg)` reads them back.
   - When `topFrame` is restored onto `drs.scopeStack`, those bound `ReferentId`s are removed from `topFrame.maskedIds` (while `topFrame.preEntryIds` remains untouched) so the parameter referents are visible inside the `Definition` body while all other caller referents remain masked. The frame records these caller-owned arguments in `callerReferentIds`, including arguments first introduced during this evaluation; zone changes carry this ownership to successor referents.
3. **Scope Exit (`leaveOwnScope` on exiting `Instruction.ownScope`):**
   - Pops `topFrame` from `drs.scopeStack` (restoring `outerFrames` directly if nested inside an outer `Definition`).
   - **Restores the caller's roles:** a mention in the body never changes a role the caller can see. The caller's pending roles become `topFrame.entryPending`, the ones it had when the body was entered, and each argument read through `inCaller` (`topFrame.callerReferentIds`) is pending as the call's object, whatever role its mentions in the body took ("Target creature fights target creature. Tap it." reads the fight's source).
   - **Closes definition-internal scratch referents:** Any definition-internal referent (`r.id ∉ topFrame.preEntryIds` and `r.id ∉ topFrame.callerReferentIds`) that is a partitive sub-selection (`r.det == .part` from `someOf` / `choose`) or a definition-local player binding (`Payload.player`) is dropped on exit—using the immutable `topFrame.preEntryIds` rather than `maskedIds` so a player passed as a parameter (`r.id ∈ topFrame.preEntryIds`) is never misclassified as a disposable local player binding.
   - **Preserves argument mutations:** Any zone changes, status changes, or `.actedOn deed` stamps applied to the passed argument referents (or their post-zone-change successor objects `[CR#400.7,400.7j]`) remain in `drs.referents`.
   - **Exports outcome referents:** All non-scratch referents produced during `topFrame` (`r.id ∉ topFrame.preEntryIds`)—including `.outcome` (all 15 `OutcomeSort`s), `.pile`, `.quality`, `.letter`, `.turnRef`, and primary `.object` outcomes (tokens from `create`/`investigate`/`amass`/`incubate`, cards moved/revealed/searched/milled/drawn/scried)—remain in `drs.referents` with `clause := topFrame.callerClause` and `role := .obj`, ready for downstream anaphora (`"Investigate. Sacrifice that token..."`, `"Destroy target creature. It can't be regenerated."`).

`TokenSpec.inCaller` applies the same caller-scope boundary to a token specification passed
into a definition. `create` wraps its token argument so chosen characteristics and copy-source
references resolve in the caller. The checker steps a copy source once and carries that selected
source’s types into the created token. Embedded ability text inherits announced choices only;
ordinary participants and the enclosing performer remain outside that text’s discourse context.

Type views (`headed`, a Head Noun over its subject, and `becomes`, the permanent a spell becomes) refine their selected referent in
place. They do not create a second discourse object merely by describing the same object’s type
or rules role.

### 1.5 Anaphor Resolution Rules (`J1`, `J2`, `D′`, `AC>R1`, and Reflexives)

Every anaphoric `NounPhrase` (`it`, `them`, `they`, `itself`, `themselves`, `yourself`, `that <noun>`, `those <nouns>`) resolves against the visible `referents` in the current `DRS` in two stages. A Participial Description (`the <verbed> <noun>`, §3.8) instead requires the unique referent its deed stamped, and is the only reference that reaches an object a linked ability published (`Referent.linked`):

#### Stage 1: Accessibility Filter (`J1`)

A referent `r ∈ drs.referents` is **accessible** to an anaphor in a consuming verb slot iff:
1. `r.id` is not masked by any active `OwnScopeFrame` on `drs.scopeStack` (`∀ f ∈ drs.scopeStack, r.id ∉ f.maskedIds`);
2. `r.plur` agrees with the anaphor's grammatical number (`.one` for `it` / `itself` / `that <noun>`, `.many` for `them` / `they` / `themselves` / `those <nouns>`, or unfixed when reading a number-neutral description);
3. `r.payload` matches the anaphor's sort, carrier-word zone (`[CR#109.2,109.2a..109.2c,110.1]`: e.g., a bare permanent type requires `.battlefield`, `spell` requires `.stack`, `card` requires a non-battlefield/non-stack zone or zone-transition readback), and `<noun>` filter; and
4. **Consuming-verb applicability (`[CR#109.2,701.21a]`):** The consuming verb or prepositional slot can apply to `r` in `r.payload.zone` and `r.payload.kind` (e.g., a creature that died is in `.graveyard` and cannot have counters put on it as a battlefield permanent or be sacrificed; this settles *Vampiric Embrace* by `J1` before ranking runs).

**Characteristic reads and Last Known Information ([CR#608.2h]):** The carrier-word zone
restriction in item 3 describes action slots. A characteristic read such as “its power” or
“that creature’s toughness” may also select a visible departed permanent using its last
battlefield type information. This does not change the referent’s current zone or admit it
as the patient of a battlefield action. Ordinary `D′` and `AC>R1` ranking still applies:
in *Kalitas, Bloodchief of Ghet* and *Broken Visage*, “its” selects the token and
“that creature’s” selects the destroyed creature. *Vampiric Embrace* still excludes the dead
creature from its counter-placement action. Clarified with the owner during the proof/card
port: this is ordinary LKI phrasing, including “Destroy target creature. You gain life equal
to its power.”

The same information-read boundary covers possessors: after destroying a permanent,
“that permanent’s controller” may read its last known controller. The enclosing damage
instruction acts on that player; the lookup does not make the destroyed permanent available
to a later battlefield action. *Destructive Revelry* exercises this distinction.

A historical movement pattern such as “that creature dies this way” reads a previously
introduced participant against the movement's written origin zone when selecting the movement
patient in a `DidClause` (without a dedicated one-purpose flag on `DRS`). The selected referent
retains its current zone, identity, and stamp. `HistoricalMove` proves the read and subsequent
ordinary-action refusals.

Counter removal cannot read counters across a zone change: the new object's identity may be
available, but its old counters ceased to exist ([CR#400.7,122.2]). Removal or transfer from
that moved referent is refused until an instruction or entry/placement rider establishes new
counters on it. This does not impose a battlefield-only restriction on counters.

#### Stage 2: Form-Specific Resolution Gate (`J2`, `D′`, `AC>R1`, and Principle A Reflexives)

- **Clause-mates (`J2`, `docs/contexts/oracle-english/CONTEXT.md`):** Referent `r` is a **clause-mate** of an anaphor iff the anaphor occupies a direct argument slot (subject, direct object, or oblique complement) of the current verb and `(r.id, _) ∈ drs.pendingRoles` (i.e., `r` was introduced or re-mentioned as a direct co-argument of the same verb in `drs.currentClause`). Possessors (`controllerOf it`, `ownerOf it`, `its power`, `"Target creature's controller"`) and objects of small clauses (`counters on it`) are **not** direct co-arguments of the matrix verb (`DrtCards.countersOnItReadsTheStatSubjectInsideTheSmallClause` and
  `DrtCards.weepingAngelNestedPossessorDoesNotMakeItsCreatureAClauseMate`).
  Re-mentioning a referent in a possessor slot (`"that creature's owner"`, `inArgSlot = false`) updates its `r.clause := drs.currentClause` and `r.role := .other` (matching a newly introduced possessor NP) without recording in `drs.pendingRoles`, so *Weeping Angel*'s subsequent `"it"` in `"that creature's owner shuffles it into their library"` resolves directly to the damaged creature under `D′` without any explicit binding identity.
- **Reflexive Pronouns (`NounPhrase.refl (pl : Plurality)` — `itself`, `themselves`, `yourself`, Principle A):**
  - Exact complement of `J2`: requires an accessible clause-mate subject (`(r.id, .subj) ∈ drs.pendingRoles`) agreeing with `pl` (covering both newly introduced subjects like `"Target creature deals 2 damage to itself"` and re-mentioned subjects like `"It deals 2 damage to itself"`). Refuses with `.anaphor` if no clause-mate `.subj` matches.
- **Rule `D′` — Bare Non-Reflexive Pronouns (`it`, `them`, `they`):**
  1. Exclude clause-mates (`J2`, i.e., any `r` with `(r.id, _) ∈ drs.pendingRoles` when the pronoun is in a direct verb argument slot).
  2. Among remaining accessible referents, select those in the **nearest clause** (maximizing `r.clause`, except that a referent re-mentioned in a direct argument slot of the current clause, `(r.id, slotRole) ∈ drs.pendingRoles`, counts as in `drs.currentClause`; owner ruling 2026-10-21).
  3. Within that clause, rank by `GramRole`: `.subj` beats `.obj`, which beats `.other`. A re-mentioned referent ranks by its best buffered slot role. Steps 2 and 3 change nothing for a pronoun in a direct argument slot, since step 1 has already excluded every buffered referent. They foreground a re-mentioned subject for possessor and amount pronouns: "That creature deals damage equal to its power …" reads "its" as that creature (`Proofs/Damage.lean`, `karplusanYetiItsReadsThatCreature`).
  4. If a single referent strictly wins (`winners.length == 1`), resolve to `r`, mark `r.pronounResolved := true` in `drs.currentClause`, and buffer `(r.id, slotRole)` in `drs.pendingRoles` if in a direct argument slot.
  5. Otherwise (`winners.length == 0` when no accessible candidate exists, or `winners.length ≥ 2` on a true tie under `D′`), **refuse** with `Refusal.anaphor reach plur winners.length`.
- **Rule `AC>R1` — Demonstratives (`that <noun>`, `those <nouns>`):**
  1. Filter accessible referents by `<noun>`.
  2. **Clause-mate exclusion (`A`):** Exclude clause-mates (`J2`).
  3. **Anti-coreference with same-clause pronoun (`C`):** Exclude any referent with `r.pronounResolved == true` in `drs.currentClause` (so in *Kalitas, Bloodchief of Ghet*, `"Its power is equal to that creature's power"`, `"Its"` resolves to the newly created token and `"that creature"` takes the remaining older creature).
  4. **Recency (`R1`):** Among remaining candidates, pick the most recently introduced referent (maximizing `r.id`); refuse with `Refusal.anaphor reach plur 0` only if zero candidates remain.
- **Stamped Action-Participle Referents (`the exiled card`, `the sacrificed creature`):**
  - Filter accessible referents by `<noun>`, plurality, and `.actedOn deed` stamp; require a unique match (`count == 1`), refusing with `Refusal.anaphor reach plur count` otherwise.
- **Required Pins for the 11 Previously Measured `AC>R1` Sites:**
  - All 11 corpus sites that failed flat surface-token `AC>R1` in the preliminary measurement **must** resolve to their intended referent from raw wording in `lean/Semantics/Proofs/` with zero explicit-selector workarounds:
    - Cards 1–9 resolve directly via stable introduction `ReferentId`s on re-mention (`DRS.recordMention`), compositional bottom-up NP evaluation (`stepOps.np`), post-move `ReferentId` allocation (`[CR#400.7]`), and closed `Predicate.or` branches.
    - Card 10 (*Mangara's Equity*) resolves directly once `NounPhrase.or` (`stepNounAlternativesWith`) closes branch-local indefinites so `"a white creature you control"` inside `"you or a white creature you control"` does not leak as an unconditional top-level creature referent.
    - Card 11 (*Noetic Scales*) resolves directly when stepped as its actual `Action.move` / `returnToHand` AST (which evaluates the moved subject NP `"each creature that player controls..."` before the destination `"to its owner's hand"`, rather than the surface Heavy-NP-Shift token order).

    1. *Tahngarth, First Mate*
    2. *Unpredictable Cyclone*
    3. *Eriette, the Beguiler*
    4. *Runesword*
    5. *Scythe of the Wretched*
    6. *Bronze Bombshell*
    7. *Gisela, Blade of Goldnight*
    8. *Solphim, Mayhem Dominus*
    9. *Ram Through*
    10. *Mangara's Equity*
    11. *Noetic Scales*

---

## 2. Pillar 2: `Definition` vs. `Template` Split

File role is declared explicitly by the top-level wrapper in each file, never inferred from directory location.

| Property | `Definition(...)` | `Template(...)` |
|---|---|---|
| **Domain** | Rules-defined vocabulary: CR §701 keyword actions, CR §702 keyword abilities, plus registry definitions (`CounterKind`, `Subtype`, `Designation`, `Relation`, `FlavorWord`, `Type`). An ability word has no rules meaning (`[CR#207.2c]`), so it is a Template over `abilityWord`, its label checked against the CR's list; a turn part is a closed `TurnPart` constructor, declared by no one. | Reusable Oracle phrasing patterns, multi-clause shorthands, and AST constructor aliases. |
| **Top-level file wrapper** | `Definition(...)` (or registry-specific `CounterDefinition(...)`, etc.) | `Template(...)` |
| **Reference Scope** | **Own Reference Scope (`ownScope (deed) (body)`):** Masks caller referents via `enterOwnScope` / `leaveOwnScope`, preserves the keyword action's `Deed` identity (`some (.action name)`), and evaluates arguments in caller's DRS (`inCaller`). | **Transparent caller scope:** Expands inline in the caller's active DRS without masking (`no` `ownScope`, `no` `inCaller`). |
| **Parameter Identity (`firstMention` / `laterMention`)** | Always enabled for `NounPhrase` parameters (supporting `List ReferentId` for coordinated NPs). | Enabled whenever a `NounPhrase` parameter is mentioned $>1$ time in the template body (`firstMention` on first occurrence, `laterMention` on subsequent occurrences before any zone change). |
| **Law Gate** | Checked as an independent unit by `cargo xtask lean definitions`. | Checked at every card or `Definition` call site that expands it. |

### 2.1 Complete Classification of In-Scope Keyword Actions as `Definition`s

Every Vintage-playable CR §701 keyword action—whether its body is a single atomic `.enact` or a multi-step expansion—is a **`Definition`** (Planechase/Archenemy actions remain permanently out of scope and Un-set Contraption/Attraction actions remain deferred per `CLAUDE.md` "Scope"):

- **Atomic CR §701 keyword actions (promoted from `Template`/macro to `Definition`):** `activate` `[CR#701.2]`, `attach` `[CR#701.3]`, `cast` `[CR#701.4]`, `counter` `[CR#701.5]`, `create` `[CR#701.6]`, `destroy` `[CR#701.8]`, `discard` `[CR#701.9]`, `exchange` `[CR#701.12]`, `exile` `[CR#701.13]`, `fight` `[CR#701.14]`, `goad` `[CR#701.15]`, `regenerate` `[CR#701.19]`, `reveal` `[CR#701.20]`, `sacrifice` `[CR#701.21]`, `search` `[CR#701.23]`, `shuffle` `[CR#701.24]`, `tap` `[CR#701.26a]`, `untap` `[CR#701.26b]`, `transform` `[CR#701.27]`, `convert` `[CR#701.28]`, `fateseal` `[CR#701.29]`, `clash` `[CR#701.30]`, `proliferate` `[CR#701.34]`, `detain` `[CR#701.35]`, `populate` `[CR#701.36]`, `monstrosity` `[CR#701.37]`, `vote` `[CR#701.38]`, `meld` `[CR#701.42]`, `exert` `[CR#701.43]`, `cloak` `[CR#701.58]`, `suspect` `[CR#701.60]`, `plot` `[CR#701.62]`.
- **Regeneration (`Definition`):** a regeneration shield's application, removing damage, tapping and removing from combat, is the one `regeneration` Definition (`[CR#701.19a]`); the deed "can't be regenerated" forbids it, and the shield `regenerate` installs applies it (`[CR#701.19c]`). It emits the keyword action's deed, so it is no Template.
- **Composite / multi-step CR §701 keyword actions (`Definition`):** `doublePower` / `doubleToughness` / `doublePT` / `doubleLifeTotal` / `doubleCountersOn` `[CR#701.10]`, `triplePower` / `tripleToughness` / `triplePT` `[CR#701.11]`, `investigate` `[CR#701.16]`, `mill` `[CR#701.17]`, `play` `[CR#701.18]`, `scry` `[CR#701.22]`, `surveil` `[CR#701.25]`, `bolster` `[CR#701.39]`, `manifest` `[CR#701.40]`, `support` `[CR#701.41]`, `explore` `[CR#701.44]`, `amass` `[CR#701.47]`, `learn` `[CR#701.48]`, `connive` `[CR#701.50]`, `incubate` `[CR#701.53]`, `theRingTemptsYou` `[CR#701.54]`, `faceVillainousChoice` `[CR#701.55]`, `timeTravel` `[CR#701.56]`, `discover` `[CR#701.57]`, `collectEvidence` `[CR#701.59]`, `forage` `[CR#701.61]`, `manifestDread` `[CR#701.63]`, `endure` `[CR#701.64]`, `airbend` `[CR#701.65]`, `earthbend` `[CR#701.66]`, `waterbend` `[CR#701.67]`, `firebend` `[CR#701.68]`.
- **Preserved `Deed` Identity on `Instruction.ownScope` (`[CR#701.22b]`):** Every CR §701 keyword-action `Definition` wraps its body in `Instruction.ownScope (some (.action name)) body`.
  - When checking the definition law of an atomic `Definition` whose body is a direct `.enact (.action name ...)` call, the checker strips the outer `.ownScope` wrapper of that self-definition so its direct effect on the passed arguments is checked and exported without double-wrapping.
  - When a composite `Definition` (`scry`, `surveil`, `mill`, `forage`, `explore`, `connive`, `investigate`, etc.) appears in imperative position (`scry(2)`), the checker steps `ownScope (some (.action name)) body` and `deckmaste_lowering_v3` compiles it to `core::Action::Composite { name: VerbName, body }` (`[CR#701.22b]`). When it appears inside `does`, `would`, `cant`, or `did` (`does(you, scry(any))`, `does(you, forage)`), both `Check/Events.lean` and `deckmaste_lowering_v3` read the `Deed.action name` and `inCaller` arguments directly from the `ownScope` header (lowering to `core::EventFilter::Act { verb: VerbName, who, on, cause }`).

### 2.2 Multi-Mention `Template`s

A `Template` is a transparent phrasing helper in the caller's DRS scope, but any `Template` that mentions a `NounPhrase` parameter more than once before a zone change **must** bind `firstMention` on its first syntactic occurrence and `laterMention` on subsequent occurrences so targets and indefinites are not duplicated (`[CR#115.1,608.2c]`):
- Examples: `duringNextTurnOf`, `discardThenForEachNonland` (or re-spelled via `"discarded this way"` per `[CR#701.50a]`), `lookAndSortSlice`, `controllerSacrifices`, `comparesOwnStat`.

---

## 3. Pillar 3: Unified Action-Clause Spine

### 3.1 Core Principle: Every Verb Defined Once as an `Instruction`

Every game action or verb phrase is defined **once** as an `Instruction` (representing the bare infinitive verb phrase: `sacrifice(a(creature))`, `destroy(this)`, `attack()`, `block(this)`, `draw(2)`, `dealDamage(3, anyTarget)`, `putCounters(1, p1p1, this)`, `move(this, battlefield, none)`).

All seven action-clause contexts embed `action : Instruction` directly—with **no** double-wrapping (`replace(would(subj, action), ...)` rather than `replace(does(would(...)))`):

| Clause Mood / Context | AST / DSL Form | Active Voice (`subj = np`) | Unspecified-Agent Passive (`subj = any`) | Explicit-Agent Passive (`subj = by(np)`) |
|---|---|---|---|---|
| **1. 2p Imperative (`Instruction`)** | `action` | `"Sacrifice a creature."` → `sacrifice(a(creature))` | — | — |
| **2. 3p Imperative (`Instruction`)** | `instruct(subj, action)` | `"Target opponent sacrifices a creature."` → `instruct(target(opponent), sacrifice(a(creature)))`<br>`"That creature explores."` → `instruct(that(creature), explore)` | — | — |
| **3. Action Cost (`Cost`)** | `Cost.perform(action)` | `"Sacrifice a creature: Draw a card."` → `activated(perform(sacrifice(a(creature))), draw(a(card)))` | — | — |
| **4. Trigger / Event (`GameEvent`)** | `does(subj, action)` | `"Whenever you sacrifice a creature"` → `does(you, sacrifice(a(creature)))` | `"Whenever this creature becomes blocked"` → `does(any, block(this))` | `"Whenever this creature becomes blocked by a creature"` → `does(by(a(creature)), block(this))` |
| **5. Replacement (`StaticSpec`)** | `replace(would(subj, action), repl, ...)`, where `repl` is `instead(a)` (`a` in the event's place), `also(a)` (`a` done as part of the event, which still happens), or `withRiders(riders)` (the replaced entering happens with Entry Riders `riders`), and `doIf(cond, repl)` builds `onlyIf(cond, repl)`, which applies only while `cond` holds | `"If you would draw a card"` → `replace(would(you, draw(a(card))), ...)` | `"If this creature would be destroyed"` → `replace(would(any, destroy(this)), ...)` | `"If this creature would become blocked by a creature"` → `replace(would(by(a(creature)), block(this)), ...)` |
| **6. Deontic (`StaticSpec`)** | `cant(subj, action)`<br>`must(subj, action)`<br>`may(subj, action)`<br>`gatedBy(cost, subj, action)` | `"This creature can't block."` → `cant(this, block(any))` | `"This creature can't be blocked."` → `cant(any, block(this))` | `"This creature can't be blocked by Goblins."` → `cant(by(a(goblin)), block(this))` |
| **7. Lookback (`Condition` / `Predicate` / `Amount`)** | `did(subj, action, within)` | `"if you sacrificed a creature this turn"` → `did(you, sacrifice(a(creature)), thisTurn)` | `"if this creature was blocked this turn"` → `did(any, block(this), thisTurn)` | `"if this creature was blocked by a creature this turn"` → `did(by(a(creature)), block(this), thisTurn)` |

### 3.2 Voice, `Subject`, and Grammatical-Role Promotion in DRT

The `Subject` of `does`, `would`, `cant`, `must`, `may`, `gatedBy`, and `did` has three constructors:

```lean
inductive Subject where
  | actor (np : NounPhrase)  -- Active voice: `np` is the agent/performer
  | any                      -- Passive voice without `by`-phrase: unspecified agent
  | by    (np : NounPhrase)  -- Passive voice with `by`-phrase: `np` is the oblique agent
  deriving DecidableEq, Repr
```

How the DRT checker evaluates `(subj : Subject, action : Instruction)`:

1. **Active voice (`Subject.actor np`, written as bare `np` in the DSL):**
   - `np` is evaluated first in the clause and stamped with `role := .subj` (and set as `drs.performer` while stepping `action`).
   - The primary patient inside `action` is stamped with `role := .obj`.
   - Any remaining noun phrases inside `action` (destinations, instruments, secondary targets) are stamped with `role := .other`.
2. **Unspecified-agent passive (`Subject.any`, written as `any`):**
   - Introduces **no** agent referent into `drs.referents`.
   - Promotes the primary patient inside `action` to `role := .subj` (matching English surface syntax where the patient is the grammatical subject: `"Whenever a creature is destroyed, it..."`).
3. **Explicit-agent passive (`Subject.by np`, written as `by(np)`):**
   - Promotes the primary patient inside `action` to `role := .subj` and evaluates it **first** (matching English word order: `"Whenever this creature becomes blocked by a creature..."`).
   - Evaluates `np` second and stamps it with `role := .other` (oblique `by`-agent).
   - Consequently, a subsequent bare pronoun `it` in the same or next clause resolves by `D′` (`.subj < .obj < .other`) to the passive subject (`this creature`) when both match in kind and zone, while `that creature` resolves by `AC>R1` (recency) to the `by`-agent (`a creature`)!

### 3.3 Polymorphic `any : α` (`AnyValue` Typeclass)

Instead of separate per-type wildcard constructors (`Subject.any`, `NounPhrase.any`, `Amount.anyAmount`, `CounterKindSource.anyKind`, `DieSides.anyDie`), Lean defines a polymorphic typeclass:

```lean
class AnyValue (α : Type) where
  any : α
```

with instances for:
- `Subject` (`.any` — passive voice / unspecified actor)
- `NounPhrase` (`.any` — unspecified target/object/defender, e.g., `cant(this, block(any))` for *"This creature can't block"*)
- `Amount` (`.anyAmount` — e.g., `cant(any, putCounters(any, any, this))` for *"can't have counters put on it"*)
- `CounterKindSource` (`.anyKind`)
- `DieSides` (`.anyDie`) and `RollWatch` (`.anyResult`)
- `ZoneExpr` (`.anyZone`)
- `Timing` (`.anyTiming`)

### 3.4 Transitive vs. Intransitive `Instruction` Conventions and Relative Clauses (`which`)

- **Transitive vs. Intransitive Argument Convention:**
  - **Intransitive / actor-only actions** (`attack`, `loseTheGame`, `commitCrime`) take no patient `NounPhrase` (or an optional defender/companion); the actor is supplied by the enclosing clause's `subj : Subject` (or is implicit `"you"` / `drs.performer` in a 2p/3p imperative).
  - **Transitive actions** (`exile(patient)`, `destroy(patient)`, `sacrifice(patient)`, `tap(patient)`, `untap(patient)`, `discard(patient)`, `block(attacker)`, `target(objectOrPlayer)`, `move(patient, from, to, obs, cause)`, `dealDamage(amount, recipient, kind)`, `putCounters(count, kind, target)`, `removeCounters(batch, kind, target)`) always keep their primary `patient` inside the `Instruction`, while `subj : Subject` on `does`/`would`/`cant`/`did`/`instruct` supplies the performer (`Subject.actor np`), unspecified passive agent (`Subject.any`), or oblique passive `by`-agent (`Subject.by np`).
- **Relative Clauses with `which` (`NounPhrase.which`, replacing `NounPhrase.gap`):**
  - Relative clauses that filter a noun by an event (`Predicate.did`) use the relative pronoun **`which`** (`NounPhrase.which`, replacing `NounPhrase.gap`) directly inside `did(subj, action, within)` in whatever argument slot is relativized:

| English Noun Phrase | `semantics-v3` Encoding |
|---|---|
| `"all creatures that attacked this turn"` | `allOf(and([creature, did(which, attack, thisTurn)]))` |
| `"each card exiled this way"` | `allOf(and([card, did(any, exile(which), thisWay)]))` |
| `"a card you discarded this turn"` | `a(and([card, did(you, discard(which), thisTurn)]))` |
| `"a creature dealt damage by this creature this turn"` | `a(and([creature, did(by(this), dealDamage(any, which), thisTurn)]))` |

When the checker evaluates `Predicate.did (did(subj, action, within))`:
- It opens a local relative-clause DRS box (`§1.3 Box 5`) where `which` is bound to a temporary referent seeded from the outer `TypeEvidence` (`c.selfType` / conjunction evidence: `.player` when filtering a player description, `.object` with the accumulated `CardType`s and expected zone when filtering an object description).
- It steps `did(subj, action, within)` in that local box (verifying that `which` appears at least once and satisfies `J1` domain compatibility for its verb slot) and discards local indefinite participants and the temporary `which` binder on exit. Definite presuppositional references (`attachHost`: `"enchanted"`, `"equipped"`, or `"fortified"`, `[CR#301.5,301.6,303.4b]`) project to the enclosing ability's DRS box with their identity and zone preserved, without a per-referent `attachmentHost` flag.

### 3.5 Pre-Action (`would`) vs. Post-Action (`does` / `did` / Imperative) DRT Stepping

Because `does(subj, action)`, `would(subj, action)`, `did(subj, action, within)`, and imperative `action` all wrap the same `action : Instruction`, the DRT checker steps `action` under a `StepPhase`:

```lean
inductive StepPhase where
  | preEvent   -- Used by `would(subj, action)` in replacement effects [CR#614.1]
  | postEvent  -- Used by imperative `action`, `instruct`, `does`, and `did`
  deriving DecidableEq, Repr
```

- **`StepPhase.preEvent` (`would`):**
  - Introduces the subject and object referents of `action` in their **pre-action** zone and status (`"If a creature would die, exile it instead"` leaves the dying creature on `.battlefield` so `exile(it)` can legally exile it from the battlefield `[CR#614.6]`).
  - Introduces prospective magnitudes (`thatMuch` damage, `thatMany` counters/tokens/cards) so replacement instructions can read them (`"prevent that much damage"`, `"put twice that many counters on it instead"`).
  - Does **not** introduce post-completion product referents (e.g., `would(you, draw(a(card)))` does not introduce a drawn card in hand).
- **`StepPhase.postEvent` (`does` / `did` / imperative):**
  - Transitions patient referents to their **post-action** zone and status, stamps them with `.actedOn deed`, and creates new post-zone-change referents where `[CR#400.7]` applies.
  - Introduces post-completion product referents (`drawn` cards, `created` tokens, `manaProduced`, `rollOutcome`).

### 3.6 Complete `GameEvent` Constructor Mapping (All 27 Domain Constructors Accounted For)

Of the 27 domain `GameEvent` constructors in `semantics-v2` (`lean/Semantics/Phrase.lean:392-441`, excluding the 2 parameter escapes `parameter` and `macroParam`), **17 action events collapse into `does(subj, action)`** and **10 non-action/structural events remain** alongside `does` (plus `stateBasedActionsChecked` `[CR#704.3]` from the 2026-10-13 ruling Q90):

#### A. Action `GameEvent` Constructors Replaced by `does(subj, action)`

| Old `GameEvent` Constructor | `semantics-v3` `does(subj, action)` Encoding | Exact Field & `Instruction` Mapping |
|---|---|---|
| `verbedEvent (deed, subject, by, becomes, locus)` | `does(subj, action)` | Covers `CoreDeed` (`tap`, `untap`, `destroy`, `sacrifice`, `exile`, `discard`, `mill`, `scry`, `surveil`, `explore`, `connive`, `forage`, `bolster`, `adapt`, `amass`, `populate`, `proliferate`, `regenerate`, `reveal`, `search`, `shuffle`, `vote`, etc.) and `Deed.ofAbility` (`crew`, `saddle`, `mutate`, `devour`), carrying optional `becomes : Option Predicate` and `locus : Option ZoneExpr` on `.enact`. |
| `casts (caster, spell, from_)` | `does(caster, cast(spell, from_))` | `Instruction.cast` carries `from_ : Option EventSource`. |
| `activates (actor, ability, of_)` | `does(actor, activate(ability, of_))` | Uses `Instruction.activate`. |
| `tappedForMana (permanent, by)` | `does(by, tapForMana(permanent))` | Uses `CoreDeed.tapForMana` (`subj = any` when `by = none`, `subj = by(p)` when `by = some p`). |
| `commitsCrime (actor)` | `does(actor, commitCrime)` | `[CR#700.13]` crime action primitive. |
| `losesGame (player)` | `does(player, loseTheGame)` | Uses `Instruction.loseTheGame`. |
| `paysCost (payer, cost, outcome)` | `does(payer, pay(cost, outcome))` | `Instruction.pay` carries `outcome : PaymentOutcome := .paid` (`.paid | .unpaid`). |
| `paysLife (player, amount)` | `does(player, pay(.life amount, .paid))` | Collapses into `Instruction.pay (.life amount)`. |
| `attacksWith (player, attackers, defender)` | `does(player, attackWith(attackers, defender))` | Player-subject combat action (`"Whenever you attack with one or more creatures"`). |
| `combat (subject, role)` | `does(attacker, attack(defender))`<br>`does(blocker, block(attacker))`<br>`does(attacker, attackUnblocked(defender))` | Active (`does(b, block(a))`), passive (`does(any, block(a))` = *"becomes blocked"*), or `by`-passive (`does(by(b), block(a))` = *"becomes blocked by B"*); `attackUnblocked` `[CR#509.1h]`. |
| `becomesTarget (subject, by)` | `does(by, target(subject))` | Adds `Instruction.target (subject : NounPhrase)` (`[CR#115.1]`), used by both `"Whenever ~ becomes the target of..."` (`does(by(src), target(this))`) and deontic shroud/hexproof (`cant(by(src), target(this))`). |
| `zoneChange (subject, from_, to, at, cause)` | `does(any, move(subject, from_, to, at, cause))` | `MoveMode` carries `from_ : EventSource` (`.anywhere`, `.zones`, `.anywhereBut`), `at : ObservationPoint` (`.arrival` for `enters` vs `.departure` for `dies` / `leaves`, `[CR#700.4,603.6a,603.6c,603.10a]`), and `cause : Option Causing`. `die(subject)` (`[CR#700.4]`), `enter(subject)`, and `leave(subject)` are `Template`s expanding to `move(subject, .zones [.battlefield], .graveyard, .departure, none)` etc., so both triggers (`does(any, die(a(creature)))`) and lookbacks (`did(any, die(a(creature)), thisTurn)`) share the single `Instruction.move` AST primitive. |
| `damage (kind, source, recipient, amount)` | `does(source, dealDamage(amount, recipient, kind))` | `dealDamage` carries `kind : Option DamageKind := none`, the explicit qualifier (`some .combat` for *"combat damage"* `[CR#120.2a]`, `some .noncombat` for *"noncombat damage"* `[CR#120.2b]`); `none` is unqualified *"damage"*, which in an instruction is the effect's own (noncombat) damage `[CR#120.2b]` and in an event, replacement, deontic, lookback or static damage rule matches all damage. `StaticSpec.damageRule`, `StaticSpec.preventionBan` and the prevention templates carry the same slot after their required arguments, and `DamageResult` carries it as a defaulted field. Active (`does(src, ...)`), passive (`does(any, ...)` = *"is dealt damage"*), or `by`-passive (`does(by(src), ...)`). |
| `lifeChange (direction, player, amount)` | `does(player, changeLife(direction, amount))` | `gainLife` / `loseLife` templates expand to `changeLife`. |
| `draws (player, cards)` | `does(player, draw(cards))` | Uses `Instruction.draw`. |
| `searchesLibrary (searcher, libraryOwner)` | `does(searcher, search(libraryOf(libraryOwner), any))` | Uses `search` definition. |
| `createsTokens (creator, tokens)` | `does(creator, create(tokens))` | Uses `create` definition. |
| `counterEvent (dir, batch, target, by)` | `countersPut(kind, batch, subject)` / `countersRemoved(kind, batch, subject)`, templates over `does(any, putCounters(…))` / `does(any, removeCounters(…))` | A `Delta`-free pair: the direction is which action the event names. `CounterBatch` is `one`, `many` or `emptying` (`[CR#310.12b,702.62a]`); `Instruction.moveCounters` stays distinct for moving counters between two objects (`[CR#122.5]`). |
| `statusEvent (edge, subject, status, by)` | `does(by, setStatus(edge, subject, status))` | Covers `"becomes tapped"`, `"is turned face up"`, `"phases out"`. |
| `attaches (attachment, object, dir)` | `does(any, attach(attachment, object, dir))` | Uses `attach` / `unattach`. |
| `gainsControl (player, object)` | `does(player, gainControl(object))` | Uses `Instruction.changeControl`. |
| `rollsDice (roller, dice, watch)` | `does(roller, rollDice(count, sides, batch, watch))` | Carries `count : Amount`, `sides : DieSides`, `batch : Plurality` and `watch : RollWatch`. |
| `flipsCoin (flipper, call, outcome)` | `does(flipper, flipCoin(call, outcome))` | Carries `call : Option FlipCall` and `outcome : Option CoinOutcome`. |
| Triggered ability triggering | `does(ability, trigger)` | The automatic triggering occurrence [CR#603.2]. |
| A creature crews or saddles another permanent | `does(creature, crew(vehicle))` / `does(creature, saddle(permanent))` | Cost participation [CR#702.122b,702.122d,702.171c]; the ability's activation and resolution retain their own events. |
| A Room becomes fully unlocked | `does(player, fullyUnlock(room))` | The designation transition specified by [CR#709.5i]. |


- **Closed Event Head-Gating on `does`, `would`, and `did` (`Check/Events.lean`):** Because `deckmaste_core` compiles events, replacements, and lookbacks to the closed `core::EventFilter` enum, `Check/Events.lean` explicitly gates `action` inside `does(subj, action)`, `would(subj, action)`, and `did(subj, action, within)` to the event-capable `Instruction` heads in Table 3.6A above plus `Instruction.ownScope (some (.action name)) body` (CR §701 keyword actions, lowered to `core::EventFilter::Act`), deterministically refusing control-flow and compound imperative instructions (`sequentially`, `doIf`, `doForEach`, `chooseModes`, `establish`, etc.).

#### B. The Remaining Non-Action `GameEvent` Constructors in `semantics-v3`

```lean
inductive GameEvent where
  | does                     (subj : Subject) (action : Instruction)
  | beginningOf              (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef)
  | endOf                    (part : TurnPart) (whose : Option NounPhrase)
  | now                                                                 -- Immediate event for self-replacement ("As this enters...")
  | stateHolds               (condition : Condition)                    -- State triggers ("When this has no counters on it...", "when its power becomes 20" as a comparison) [CR#603.8]
  | gameBecomes              (designation : DesignationName)            -- "Whenever it becomes day / night" [CR#731.1]
  | chapterMark              (chapters : List ChapterNumber)            -- Saga chapter triggers [CR#714.2]
  | triggers                 (ability : NounPhrase) (of_ : NounPhrase) (by : Option NounPhrase) -- "Whenever a triggered ability of ... triggers"
  | nthOccurrence            (ordinal : Ordinal) (per : Option TurnPart) (event : GameEvent)    -- "Whenever you cast your second spell each turn"
  | causes                   (cause : Causing) (event : GameEvent)      -- "Whenever an ability you control causes you to..."
  | stateBasedActionsChecked                                            -- State-based check boundary for lookback [CR#704.3,704.5h] (Ruling 2026-10-13, Q90)
```

### 3.7 Complete `StaticSpec` Deontic, Replacement, Projection, and Damage-Result Unification

`StaticSpec.deonticRule`, `Role` (`agent` vs `patient`), `DeonticPatient` (`noPatient`, `defendingPlayer`, `counterpart`, `targetedBy`, `counterpartsAt`, `host`), `DeedComplement`, and `CoreDeed.haveCountersPutOn` are **deleted** and replaced by unified deontic constructors taking `(subj : Subject, action : Instruction, mods : DeonticMods := {})`:

```lean
structure DeonticMods where
  bound    : Option CountBound := none     -- e.g., "alone", "more than one creature" [CR#702.111b]
  asThough : Option AsThough   := none     -- e.g., "as though it had flying / flash"
  rider    : DeonticRider      := .none    -- play-from-zone / pay-instead riders
  except   : Option Cause      := none     -- Deontic Exception [CR#702.13b,702.145b]
  deriving DecidableEq, Repr
```

- **Compound deontics (`"can't attack or block"`):** Coordinated prohibitions/requirements use `StaticSpec.conjunction [cant(this, attack(any)), cant(this, block(any))]` (with a `cantBoth` template helper where desired), eliminating the single-purpose `Deeds = List Deed` wrapper.
- **Token specification arguments:** `TokenSpec.inCaller` evaluates a token specification in the immediate caller's Reference Scope and restores the callee's scope afterward. `EntryRider.inCaller` applies the same boundary to a whole entry-rider argument, including an attacking defender. The `create` Definition wraps its token-specification parameter and each entry rider, so caller arguments remain caller reads rather than definition-internal reads. Embedded or granted ability text receives only visible chosen-quality and chosen-player referents; ordinary surrounding discourse does not become an antecedent inside that text.
- **Closed Deontic Head-Gating on `cant`, `must`, `may`, and `gatedBy` (`Check/Events.lean`, `Check/Phrase.lean`):** Deontic clauses use a separate closed head gate. It admits `attack`, `block`, `target`, `attachment`, `procedure` (cast/play/activate), `copy`, `draw`, `changeLife`, `dealDamage`, `putCounters`, `removeCounters`, `search`, `turnOver`, `conclude`, `changeControl`, `tapForMana`, `unlock`, `trigger`, `crew`, `saddle`, `fullyUnlock`, `vote`, and `pay`; `setStatus` is limited to tapped, untapped, phased in, or phased out; a raw `move` must name the battlefield as its destination. Named keyword-action definitions use `ownScope (some (.action name))`, with transparent wrapper traversal. Copy and vote permissions are required by Display of Power and additional-vote clauses; payment permissions use `pay .anyCost` with the spend-as-though modifier. `Cost.anyCost` is a wildcard restricted to event, lookback, prospective, and deontic action patterns; performed costs require a specified cost. Likewise, `Ballot.anyBallot` is an action-pattern wildcard for additional-vote permissions; a performed vote requires actual labels or a candidate, and an empty label list remains invalid. Pattern entry restores the caller’s mode before checking subsequent effects. Procedural combinations and other event heads, such as `shuffle`, remain refused. The 2026-10-14 bench extension preserves the distinctions between triggering [CR#603.2], crewing and saddling [CR#702.122b,702.171c], phase-in status [CR#702.26c], and fully unlocking [CR#709.5i].

- **Causal Replacement Patterns (2026-10-14 bench extension):** `WouldClause.causedBy (cause : Causing) (clause : WouldClause)` wraps a prospective action clause with the source, event, or effect that causes it [CR#700.1,614.1a]. The cause is read before the nested clause. Rain of Gore retains both its spell-or-ability source and the controller of that source as the prospective life-gain subject. Its official ruling excluding life gain from combat damage with lifelink provides a distinguishing example; a bare life-gain replacement would lose that restriction.


| Old `StaticSpec` Form | `semantics-v3` Form | Example |
|---|---|---|
| `deonticRule(s, .prohibit, deeds, .agent, ...)` | `cant(s, action, mods)` | `cant(this, block(any))` (*"This creature can't block."*) |
| `deonticRule(s, .prohibit, deeds, .patient, .noPatient, ...)` | `cant(any, action, mods)` | `cant(any, sacrifice(this))` (*"This permanent can't be sacrificed."*), `cant(any, untap(this))` (*"doesn't untap"* `[CR#614.17,502.3]`) |
| `deonticRule(s, .prohibit, deeds, .patient, .counterpart b, ...)` | `cant(by(b), action, mods)` | `cant(by(a(goblin)), block(this))` (*"This creature can't be blocked by Goblins."*) |
| `deonticRule(s, .prohibit, [.core .haveCountersPutOn], .agent, ...)` | `cant(any, putCounters(any, any, s), mods)` | *"can't have counters put on it"* |
| `deonticRule(s, .prohibit, [.target], .patient, .targetedBy src, ...)` | `cant(by(src), target(s), mods)` | *"can't be the target of spells or abilities your opponents control"* (shroud/hexproof/protection) |
| `deonticRule(s, .require, deeds, .agent, ...)` | `must(s, action, mods)` | `must(this, attack(any))` (*"attacks each combat if able"*) |
| `deonticRule(s, .require, deeds, .patient, .counterpart b, ...)` | `must(by(b), action, mods)` | `must(by(allOf(creature)), block(this))` (*"All creatures able to block this creature do so."*) |
| `deonticRule(s, .permit, deeds, .agent, ...)` | `may(s, action, mods)` | `may(you, play(it), { rider := ... })` |
| `deonticRule(s, .gatedBy cost, deeds, ...)` | `gatedBy(cost, subj, action, mods)` | `gatedBy(mana([2]), any, attack(you))` (*Ghostly Prison*) |
| `replacement(event, alternatives, timing, repl, use, limit)` | `replace(would(subj, action), repl, ...)` with `repl` one of `instead(a)`, `also(a)`, `withRiders(riders)`, `onlyIf(cond, repl)` | `replace(would(any, destroy(this)), instead(regeneration(this)))` (with `wouldAny([would(...), ...])` for disjunctive replacement events) |

- **2026-10-13 Owner Rulings Preserved on the `semantics-v3` Spine (Q81–Q90):**
  - **`Stat.defense` and battle entry conferral (Q81, Q82):** `Stat` gains `defense` (`[CR#310.4,310.4a]`, mapped by `Stat.comparedType` to `battle`); `Stat` and `CharacteristicStat` remain distinct enums (`[CR#109.3]`). `[CR#310.4b]` is written as a conferral on the `battle` card-type definition (`enters with defense counters equal to statOf(stat(defense), this)`).
  - **`ProjAxis.markedDamage` and `GameEvent.stateBasedActionsChecked` (Q83, Q90):** `ProjAxis.markedDamage` reads damage marked on an object (`[CR#120.3e,120.6,704.5g]`). `GameEvent.stateBasedActionsChecked` (`[CR#704.3]`) bounds the deathtouch SBA lookback (`[CR#704.5h]`): `did(by(a(withKeyword(deathtouch))), dealDamage(any, this), .duration (.between .stateBasedActionsChecked .now))`.
  - **`DamageResult` and `StaticSpec.damageResult` (Q86, Q87, Q88):** `DamageResult (kind : Option DamageKind := none) (source : Subject) (recipient : Predicate) (mode : ResultMode) (does : Instruction)` (`[CR#120.3,120.4c,702.15b,702.80a]`) uses `Subject` for `source` and `Instruction` for `does`, with `ResultMode = intrinsic | instead (of : OutcomeSort) | inAddition` (`OutcomeSort.damageMarked` `[CR#120.3d]`).

### 3.8 Lookback (`did` and `which`)

`LookbackClause` is replaced by `DidClause`:

```lean
inductive LookbackSpan where
  | duration (d : Duration) (timing : Option Timing := none)
  | thisWay
  | triggering
  deriving DecidableEq, Repr

structure DidClause where
  subj   : Subject
  action : Instruction
  within : Option LookbackSpan
  deriving DecidableEq, Repr
```

The templates mirror `does` and `would`, subject first: `did(subj, action, within)` and its
agentless passive `was(action, within)` (`Subject.any`), each dispatched on the expected type to
`Predicate` or `Condition`. A bare `Duration` coerces to its span. `Condition.did`, `Condition.was`
and the tally templates take a non-optional `LookbackSpan`; only `Predicate.did`/`Predicate.was`
may leave it unwritten (`within = none`), and the checker refuses that
(`Refusal.unanchoredLookback`) anywhere but a Participial Description (below). `happened`,
`happenedThisWay`, `happenedTo` and `wasCast` are retired.

Used uniformly across all three lookback sites:
1. **`Condition.did (clause : DidClause)`** (replaces `Condition.happened`, dropping the unused `gap : Option Duration` parameter):
   - `"if you sacrificed a creature this turn"` → `did(you, sacrifice(a(creature)), thisTurn)`
   - `"if a creature died this turn"` → `did(any, die(a(creature)), thisTurn)` (where the `die(subject)` template `[CR#700.4]` expands to `Instruction.move(subject, .zones [.battlefield], .graveyard, .departure, none)`, matching `does(any, die(a(creature)))` in §3.6)
2. **`Predicate.did (clause : DidClause)`** (replaces `Predicate.happenedTo`, using `NounPhrase.which` in the relativized argument slot):
   - `"creatures that attacked this turn"` → `and([creature, did(which, attack(any), thisTurn)])`
   - `"each card exiled this way"` → `and([card, was(exile(which), thisWay)])`
3. **`Amount.tally (clause : DidClause) (mode : TallyMode)`** (replaces `Amount.eventTally`):
   - `TallyMode.events` (`"the number of times..."`): `tally(did(you, cast(yourCommander), thisGame), .events)`
   - `TallyMode.quantity` (`"the number of cards..."` / `"the amount of life..."`): `tally(did(you, draw(a(card)), thisTurn), .quantity)`, `tally(did(you, gainLife(any), thisTurn), .quantity)`

**Participial Descriptions** (`lean-v3-typed-verbed-deeds`). A `NounPhrase.described` with
determiner `the` (`DetPhrase.the (plur := .one)`, `.many` for "the exiled cards"), `all`, `each`
or the bare plural, whose predicate has a conjunct `Predicate.did` with no span written or `thisWay`,
`which` relativizing what the deed acted on, introduces no referent: it resolves to the one
referent in scope that deed stamped, filtered by the other conjuncts' Head Noun read as the object
was when acted on (`card`, a battlefield location as "permanent", Card Types `[CR#608.2h]`), and
refuses with `Refusal.participialDescription deed n` when `n ≠ 1`. `thisWay` reads only referents
the current ability stamped `[CR#607.1c]`; no span also reads those a linked ability published
`[CR#607.2a]`, where a singular `the` reads a published group `[CR#607.3]`. `Reach.verbed`,
`VerbedMarking` and `theVerbed` are retired; "it"/"them"/"that card" are ordinary pronouns.
Linked and self-linked abilities share one authored AST (`deckmaste_spelling` needs one AST per
printed wording); lowering finds the link from the same resolution, and game-time zone filtering
(a card leaving exile before champion's return resolves) belongs to the lowering/core rewrite
`[CR#607.2a]`.

---

## 4. Roadmap and Ticket Decomposition

The transition to `semantics-v3` is decomposed into the following ordered tickets in `docs/tickets/planned/`:

1. **`lean-v3-action-clause-spine`**: Relocate the `v2` Lean tree to an inert reference directory `lean/archive/SemanticsV2/` (outside `lakefile.toml`), set `lakefile.toml` to build only the syntax root (`roots = ["Semantics"]`, `defaultTargets = ["Semantics"]`), and refactor `lean/Semantics/` AST (`Words.lean`, `Phrase.lean`, `Events.lean`, `Triggers.lean`, `Abilities.lean`, `Rules.lean`, `MacroParameters.lean`) to introduce `Subject` (`actor`, `any`, `by`), `AnyValue` (`any : α`), `NounPhrase.which`, `instruct(subj, action)`, `does(subj, action)` (collapsing 17 action `GameEvent` constructors), `would(subj, action)` + `instead`, `cant`/`must`/`may`/`gatedBy`, and `did(subj, action, within)` + `tally`. Absorbs `lean-constructor-collapse` and `plugins-v2-menace-bound-helper`.
2. **`lean-v3-drt-checker`**: Port `lean/Semantics/Check/` from `lean/archive/SemanticsV2/Check/` onto the `v3` AST, replacing `Bindings`, positional stack addresses, `Window`, `withBindings`, `CaptureInput`, `Refusal.unusedCapture`, and `MoveProfile`/`StepProfile` with the clean-sheet `DRS` (`ReferentId`, `Referent`, `GramRole`, `pendingRoles`, `OwnScopeFrame`, nested DRS boxes, `StepPhase.preEvent` vs `.postEvent`, `J1` accessibility, `J2` clause-mates, Principle A reflexives `NounPhrase.refl`, `D′` bare-pronoun resolution, and `AC>R1` demonstrative resolution), and add `"Semantics.Check"` to `roots` in `lakefile.toml`. Absorbs `lean-checker-binding-ids`.
3. **`lean-v3-definitions-and-templates`**: Port `lean/Semantics/Macros.lean`, `Macros/Primitives.lean`, `Macros/Declared.lean`, and `Authoring.lean` from `lean/archive/SemanticsV2/` onto the `v3` AST and `DRS` checker, enforcing the explicit `Definition` vs. `Template` split, promoting atomic CR §701 keyword actions to `Definition`s (`ownScope` + `inCaller`), stripping outer `ownScope` on intrinsic `.enact`, retiring the 9 hand-coded window macros, resolving `airbend` (`[CR#701.65a]`, absorbing `semantics-v2-per-member-handoff`) and `ripple` (`[CR#702.60a]`), and adding `"Semantics.Macros"` and `"Semantics.Macros.Declared"` back to `roots` in `lakefile.toml`.
4. **`lean-v3-proofs-and-cards`**: Port `lean/Semantics/Proofs/` and `lean/Semantics/Cards/` once from `lean/archive/SemanticsV2/` onto the finished `semantics-v3` spine, checker, and macros, restore `defaultTargets = ["Semantics", "SemanticsCards", "SemanticsProofs"]` in `lakefile.toml`, prove the `D′` and `AC>R1` witness lemmas, flip the 6 resolved ambiguity pins, prove the 11 `AC>R1` corpus-recency witness pins, and **delete `lean/archive/SemanticsV2/`**.
5. **`semantics-v3-dsl-and-plugins`**: Create `crates/deckmaste_semantics_v3` and `plugins_v3/` (including the `v3` DSL parser/emitter for `Definition(...)` and `Template(...)` files, `any`, and the unified spine), re-point `xtask` (`lean`, `facts`) at `v3`, and **delete `crates/deckmaste_semantics_v2` and `plugins_v2/canon/`** (`plugins_v2/builtin/` deletes once `deckmaste_lexical_source` / `deckmaste_macro_def` is repointed at `v3`). Absorbs `lean-macros-cutover`, `plugins-v2-bodies-through-helpers`, and `semantics-v2-embed-candidates`.
6. **`plugins-v3-canon`** (targeting `plugins_v3/`): Author the full canon card and rules-table corpus in `plugins_v3/`, gate-checked by Lean, and retire the hand-written `lean/Semantics/Cards/` bench (`lean-hand-bench-retirement`).
7. **`lowering-v3`** (building `crates/deckmaste_lowering_v3`) and **`semantics-v3-parity`** (comparing `v1` vs. `v3`), culminating in **`semantics-v1-cutover`** (retiring `v1` and renaming `_v3` to the canonical names).

---

## 5. Standing Domain Mechanisms Carried Forward from `semantics-v2`

Because `docs/archive/` is excluded from agent search, the domain mechanisms and rulings settled during `semantics-v2` that remain in force unchanged (modulo the `semantics-v3` spine and DRT checker above) are recorded here so this ADR is self-contained:

- **Layer Contract & Left-to-Right Discourse Threading (`semantics-v2` §§1–3, §7):**
  - Constructions occupy their surface English positions with no prenex target hoisting or positional slot indices; all hoisting and slot indexing belong to lowering.
  - Constructor arguments are evaluated in textual left-to-right order. Delayed triggers read prior referents as settled particulars (`[CR#603.7c]`) while targeting in their own event (`[CR#603.3d,601.2c]`). Activated-ability effects read cost mentions filtered to public current zones (`[CR#400.2,400.7j]`).
  - **Joined-kind bindings (`Payload.join`, `semantics-v2` §3):** A binding at a joined kind holds both halves' records side by side so its kind is the join of its halves' kinds and demonstrative echoes copy the pair (`kind-index-joins-union-marking-is-spelling.md`).
  - **Checker Charter (`[CR#101.3,609.3]`):** The Lean checker verifies that references resolve, terms are well-kinded, and card frames are well-formed (the 14 card-frame rules `cardLine` through `prototypeFrame`), and that a Characteristic-Defining Ability meets the criteria of `[CR#604.3a]` (`cdaEdit`, `cdaSubject`, `cdaGranted`, `cdaConditional`); it is not a runtime rules engine for impossible instructions.
  - **Stat Changes (`StaticSpec.characteristicChange`, `[CR#613.4b,613.4c]`):** Two-stat P/T changes (`"<subject> gets +P/+T"`, `"has base power and toughness P/T"`) use one `characteristicChange subject [stat …, stat …]` node (templates `getsPt`, `getsBase`) so the subject is written and evaluated once. `getsPt` always carries both printed deltas: `"+X/+0"` is `getsPt subject (up X) (up 0)`, because the printed `"+0"` is part of the encoding and spelling must render it back (owner ruling 2026-10-21). The single-stat `modification` template is only for a change whose text names one stat. A Characteristic-Defining Ability is `StaticSpec.characteristicDefining subject edits` (`ptDefinition`, `qualityDefinition`), applied in layer 7a for power and toughness and first in its layer otherwise (`[CR#604.3a,613.3,613.4a]`).
  - **Counter Kinds, Designations, and Deeds as Declared Names (`[CR#122.1,122.1a,122.1b,731.1]`):** `CounterKind.named name`, `DesignationLabel.named name`, `Deed.action name`, and `Deed.ofAbility name` carry their declaration's name verbatim. A counter declaration states what **one** counter confers (`boost(up(1), up(1))`, `grants(flying)`). Each declaration registers its name as a term-level macro while the registry holds its `Definition` record apart for facts generation and `cargo xtask lean definitions` (Ruling 2026-10-10, Q80).
  - **No Chooser on `choose` and No Taker on `addTurnPart` (`[CR#500.7..500.9]`):** Both `choose` and `addTurnPart` (`extraTurn`, `additionalPhase`, `additionalStep`) are performed by the active performer (`drs.performer` / `"you"`).
- **Cost Substitutes (`ManaSubstitute`, `[CR#702.51a,702.66a,702.126a,701.67a]`):**
  - `ManaSubstitute (action : Instruction) (pays : List PipPaid)` (`PipPaid = generic | ofColor (color)`) models `"For each [generic | colored] mana in ..., you may <act> rather than pay that mana"` (convoke, delve, improvise, waterbend), carried by `StaticSpec.paySubstitute` on a spell or `Cost.withSubstitute` on a named cost.
- **Piles (`Pile`, `PileFace`, `[CR#700.3b,700.3c,401.2,404.2,406.4]`):**
  - A `Pile` is a library (face down), a graveyard (face up), `"that pile"`, or a face-down exile pile, supporting `slice (end) (amount) (pile)`, `run (end) (stop) (pile)`, `place (end) (pile)`, and `shuffle (pile)` (with bare `"shuffle"` = `shuffle(library(actor))` `[CR#701.24a]`).
- **Temporal Suite (`Timing`, `Duration`, `UsageLimit`, `TurnRef`, `[CR#307.5,117.1d,611.2b,602.5b]`):**
  - `TurnRef = current | previous | next (player) | each (player) | later | that`.
  - `Timing` combines `inPart (part) (whose) (which)`, `relativeToEvent (before | after) (event)`, `whileDoing (event)`, and `onlyIf (condition)` under `and`/`or`/`not` (with `asSorcery` over main phase + your turn + empty stack `[CR#307.5]`, and instant speed `onlyIf(hasPriority(you))` `[CR#117.1d]`).
  - `Duration` is `between (start : Option GameEvent) (end : Option GameEvent)` (using `beginningOf`, `endOf`, `now`, `stateBasedActionsChecked`) or `forAsLongAs (condition)` `[CR#611.2b]`.
  - `UsageLimit` is `(times) (per : Period)` where `Period = turn | yourTurn | game | part (TurnPart) | thisObject` `[CR#602.5b,702.177a]`.
- **Face-Down Specs, Special Actions, and Move Riders (`[CR#406.3a,708.2,702.37a,702.168a,116.1,614.1c]`):**
  - `FaceDown (characteristics : Option Characteristics)` is `none` for face-down exile and `some` for face-down spells/permanents (morph, disguise, manifest, cloak), used by `Instruction.turnFaceDown`, `EntryRider.faceDown`, `PilePlacement.faceDown`, and `PlayPayment.faceDown`.
  - `StaticSpec.specialAction (when : Option Timing) (cost : Cost) (does : Instruction)` models special actions `[CR#116.1]`.
  - `MoveRider = entry (EntryRider) | placement (PilePlacement)` separates battlefield entry riders from non-battlefield pile placements (`[CR#614.1c,401.4,406.3]`).
- **Modal Choices (`StaticSpec.modeChoice`, `[CR#700.2d,700.2h,700.2i,702.42a,702.120a,702.172a,702.183a]`):**
  - `chooseModes` carries mode quantity, per-mode costs (spree, tiered), and optional pawprint weights (`Quantity.worth`). `StaticSpec.modeChoice (change : ModeChange)` modifies a spell's `chooseModes` via `allInstead` (entwine), `extraModesCost` (escalate), `setQuantity` (spree, tiered), `repeatable`, and `notChosenBefore`.
- **Off-Stack Statics (`StaticSpec.asEvent`, `StaticSpec.whileHolds`, `[CR#604.1,603.7a,603.7g]`):**
  - `StaticSpec.asEvent (event) (does)` models `"as [event], do"` (dash, blitz, warp, Aura mandatory entry attachment `[CR#303.4f]`); "As [this] enters" text (entry choices, devour, riot, tribute, unleash, amplify) instead replaces the entering with `also` (`replace(would(this, enter), also(...))`), and "enters tapped / with / as / transformed" with `withRiders(...)` (`[CR#603.6d,614.1c,614.12]`), and `StaticSpec.whileHolds (condition) (does)` models state statics (ascend, storied, daybound/nightbound `[CR#702.131b,702.145b,702.195a]`).
- **Characteristic Sharing & Group Agreement (`Predicate.shares`, `NounPhrase.agree`, `[CR#201.2a,201.2c,105.2,205.3m]`):**
  - `Predicate.shares (axis) (agreement : share | differ) (with)` checks whether an object shares at least one value (`share`) or none (`differ`) on `axis` with `with`. `NounPhrase.agree (axis) (agreement) (group)` constrains a group selection to mutually share or differ on `axis`.
- **Linked-Ability Readback & Declared Relations (`LinkedFacet`, `LinkSource`, `Ability.textContext`, `[CR#607.2a,607.2d,607.2i,702.33e,702.55b,702.95b,702.99b]`):**
  - `LinkSource` names a linked cost (`byKeyword`, `byNthKeyword`, `theAlternative`, `theAdditional`), and `LinkedFacet` reads back its payment: `colorsSpent`, `manaValueSpent`, `timesPaid`, `readback`, and `taken`. Named object-to-object links (haunt, soulbond, cipher) use `Definition.relation` with `relate`, `related`, and `counterpart`.
  - The objects a linked ability acted on and the player its cost chose are read as the ability would read its own: a Participial Description (§3.8) and `the isChosenPlayer` (exactly one player choice in scope). `Ability.textContext` publishes, to the abilities after it (a card's text, and a keyword's `body`): choices, additional- and alternative-cost results (`.can _ (.procedure .cast _ (.payingInstead cost) ..) _`, emerge and harmonize `[CR#702.119a,702.180a]`), the target `addsTarget` adds, stamped by targeting (mutate `[CR#702.140a]`), what a replacement does `also` (devour, whose "it devoured" is a Deed done by sacrificing `[CR#702.82b]`), and the objects a printed triggered or activated ability itself introduces in a public zone, along each arm it may take (champion `[CR#702.72a,607.2k]`), never what its trigger event names `[CR#607.1]`. Published objects are `linked`: no pronoun or demonstrative reaches them. `LinkSource.byDeed`, `LinkedFacet.objects`/`chosenPlayer`, `NounPhrase.paid`, `linkedObject(s)` and `chosenAsPaid` are retired (`lean-v3-typed-verbed-deeds`).
- **Spell Copies, Text-Changing Effects, and Ability Qualifiers (`[CR#707.10c,702.40a,702.56a,612.1,613.1c,113.10a,702.165c,702.169b]`):**
  - Spell-copy keywords write `may(chooseNewTargets(...))` (`[CR#707.10c]`), guarded by `matches_ thisSpell (targeting any .someTarget)` where the keyword's CR definition specifies `"If the spell has any targets"` (`storm` `[CR#702.40a]`, `replicate` `[CR#702.56a]`).
  - Text-changing effects (cleave, overload, splice, word replacements `[CR#612.1,613.1c,702.47a,702.96a,702.148a]`) use `StaticSpec.textChange (subject) (change)` (`removeBracketed`, `targetToEach`, `appendTextOf`, `replaceWord`) over `Predicate.bracketed` and `Instruction.bracketed`.
  - `KeywordParam.ability` and `Ability.qualified (ability) (timing) (limit) (guard) (from)` model abilities printed with and restricted by a keyword (backup `printedAfter`, saddle, exhaust `[CR#113.10a,702.165c,702.169b..702.169d,702.177a]`).
- **Subtype Rules, Conferrals, and Counter Annihilation (`[CR#301.5,301.6,303.4a,303.4c,310.7,704.5m,704.5n,704.5q,704.5s,704.5v,714.2d,714.3a,714.3c,714.4]`):**
  - `Enchant` owns its spell target restriction, legal-host permission, and entry attachment `[CR#702.5a,303.4a,303.4c,303.4f]`.
  - Subtype and counter state-based actions live as `Conferral.stateBased (when) (then_)` on their definitions (Aura `[CR#704.5m]`, Equipment/Fortification `[CR#704.5n]`, Saga `[CR#704.5s,714.4]` using `ProjAxis.chapter` `[CR#714.2d]`, `chapterAbilityOf` + `in_ stack`, and `Conferral.turnBased` `[CR#714.3c]`, Siege `[CR#310.7,704.5v]` using `abilityOf` + `in_ stack`, and `+1/+1`/`-1/-1` counter annihilation on `m1m1Counter` `[CR#704.5q,122.3]` using `Amount.aggregateOf .min` / `smallerOf`).

