---
needs: [plugins-v3-canon]
---
**Create `crates/deckmaste_lowering_v3`: `semantics_v3` to `core` (updated
2026-10-14 per [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)).**
Sibling of `deckmaste_lowering`; v1 lowering deletes at cutover. Same contract
as `semantics-spelling-lowering.md` §9 and §17 and `core-explicit-regions.md`:
one-way compile, the crate is the divergence ledger, and rearrangement,
hoisting, and register indexing happen here. May fail on a card that breaks a
Lean law; does not validate. Standard constraints apply.

## Lowering Seams (`deckmaste_semantics_v3` → `deckmaste_core`)

1. **`DRS` / `ReferentId` to `core::Region` (`DefId` / `RefId` / `Param`) without a duplicate antecedent stack:**
   - Whereas `v1` `deckmaste_lowering` (`region.rs`) had to maintain its own
     heuristic `antecedents: Vec<Antecedent>` stack to guess which earlier
     `DefId` an `it` or `that <noun>` referred to, `deckmaste_lowering_v3`
     consumes the `DRS` resolution from `deckmaste_semantics_v3::reads`
     (`ReferentId`, `firstMention` / `laterMention` `BindingId`, `D′`, `AC>R1`,
     and `.actedOn` participle stamps) directly.
   - Each introduced `ReferentId` maps deterministically to a `core::DefId` (or
     region `Param` across delayed-trigger / continuous-effect / replacement
     region boundaries `[CR#603.7c]`), and every anaphoric read emits a
     `core::RefId` targeting that resolved `DefId`.
2. **CR §701 `Definition` (`Instruction.ownScope (some (.action name)) body`) to `core::Action::Composite` and `core::EventFilter::Act`:**
   - In imperative position (`scry(2)`, `forage`, `explore`, `connive`, `mill(3)`),
     an `Instruction.ownScope (some (.action name)) body` whose body is a
     composite expansion lowers to `core::Action::Composite { name: VerbName, body }`
     so the engine executes `body` and commits the `VerbName` event fact
     `[CR#701.22b]`, while an atomic `Definition` whose body is a single
     intrinsic `.enact` lowers directly to the corresponding primitive
     `core::Action` (`Destroy`, `Exile`, `Sacrifice`, `Tap`, etc.).
   - Inside `does`, `would`, `did`, or `cant` (`"Whenever you scry"`, `"Whenever
     you forage"`), `ownScope (some (.action name)) body` lowers via its header
     `Deed.action name` and `inCaller` arguments directly to
     `core::EventFilter::Act { verb: VerbName, who, on, cause }` (or the
     corresponding primitive `core::EventFilter` / `core::DeonticAction`).
3. **Unified Action-Clause Spine (`does`, `would`, `did`, `cant`/`must`/`may`/`gatedBy`) to `core::EventFilter`, `core::Replacement`, `core::Lookback`, and `core::Deontic`:**
   - Compiles `GameEvent.does(subj, action)` (over the closed set of
     event-capable `Instruction` heads enforced by `Check/Events.lean`,
     `semantics-v3.md` §3.6) to `core::EventFilter`, routing `Subject.actor np`,
     `Subject.any`, and `Subject.by np` into the event filter's actor/source/cause
     fields and the `Instruction`'s patient/destination fields into the event
     filter's subject/zone fields (`ObservationPoint.arrival` vs `.departure`
     `[CR#700.4,603.6a,603.6c,603.10a]`).
   - Compiles `StaticSpec.replace(would(subj, action), instead(repl), ...)` to
     `core::Replacement` (`[CR#614.1,614.6]`), `DidClause` (`Condition.did`,
     `Predicate.did` with `NounPhrase.which`, `Amount.tally`) to `core::Lookback`,
     and `StaticSpec.cant` / `must` / `may` / `gatedBy` (over the closed set of
     deontic-capable `Instruction` heads enforced by `Check/Abilities.lean`,
     `semantics-v3.md` §3.7) to `core::Deontic` (`core::DeonticAction`).
4. **Action Costs (`Cost.perform(action)`) across `cost : effect`:**
   - Compiles `Cost.perform(action)` to `core::CostComponent` /
     `RunnableCostAction` (`[CR#602.2b,118.1]`), threading cost-bound `DefId`s
     filtered to public zones (`[CR#400.2,400.7j]`) into the activated ability's
     effect region (`"Sacrifice a creature: Draw cards equal to its power"`).
