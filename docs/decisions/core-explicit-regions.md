# Core is explicit regions

> Workbench succession (2026-09-05): current workbench evidence lives in
> `lean/`; Idris paths below are historical. See
> [Lean is the workbench](lean-is-the-workbench.md).

**Every binding in `deckmaste_core` is a declared parameter or a numbered
definition, and every reference is a register read. Core never resolves
anything; lowering resolves once, and the engine reads by index.**

Decided 2026-09-02. Amends [Semantics, spelling, lowering](semantics-spelling-lowering.md)
§17: the per-scope slot environment it previews becomes closed regions with
declared parameters, and the frame-local candidate tier it kept is gone.
Supersedes the `core-reference-slots` ticket (deleted the same day) and the
engine tickets its stages absorb (listed in each stage ticket).

## The contract

1. **Regions.** A region is a closed function: `Region { params, body }`.
   Regions are an ability, each modal mode, each delayed or reflexive
   trigger body, each carried body (`Composite`, a floating replacement),
   each `Each`/`Distribute` body, and every per-candidate predicate
   (`SelectAll`, a `TargetSpec` filter, `Where`, `Pick`, `Aggregate`, a
   continuous per-subject read). A region reads nothing it did not declare.
2. **Params.** `Param { def: DefId, kind, provenance }`. Provenance is typed
   data, never a string: source, controller, event object/patient/actor,
   defending player, announced target `k`, announced X, capture of the
   parent's register `r`, linked memory cell `m`, loop element, allotment,
   candidate. The engine dispatches denotation on it: an announced slot
   keeps the partial-fizzle read [CR#608.2b], a trigger source reads LKI, a
   move product is a new object [CR#400.7]. Param order per region kind is
   fixed by lowering: source, controller, event roles, targets, X; a delayed
   body's captures follow its own event roles; a loop body's element (and
   share) and a predicate's candidate stand alone.
3. **Definitions.** `DefId`s are sequential in textual order per region,
   params first. Only an instruction with a program point defines: a
   decision (`Choose`, `ChooseValue`, `Search`, `SeparatePiles`,
   `ChoosePile`), an action's product (`Act { dest: Option<DefId>, action }`),
   a dig (`RevealUntil`), a cost's paid product, and `Let { dest, expr }`
   when English fixes an evaluation moment, as "that many" does after a
   count [CR#608.2h].
4. **References.** `Reference::Reg(RefId)` is the only binding read. `This`,
   `You`, `Opponent`, `DefendingPlayer`, `EventObject`, `EventPatient`,
   `EventActor`, `It`, `That(Sort)`, `Target(n)`, `Bound(Ident)`,
   `Linked(Ident)`, `Source`, `Selection::They`/`Them`/`Targets(n)`/
   `PilesOf`/`AmongNoted`, and `Count::ThatMany`/`ThatMuch`/`Allotment`
   leave core; semantics keeps its anaphors. Pure derivations
   (`ControllerOf`, `OwnerOf`, `AttachHostOf`, `Coalesce`, `Single`,
   `OpponentOf`, counts over selections) stay nestable expressions
   evaluated at the read site: they introduce no binding and get no def
   unless a `Let` pins them.
5. **Instruction vs expression.** An instruction has a program point: an
   effect, a decision, or a `Let`. An expression is pure and decision-free.
   The validator rejects a decision-bearing form (`Choose`, `Random`, a
   decision-bearing `Pick`) inside an expression. Every pause in a
   resolution therefore lands on an instruction boundary, and resume is
   "write the register, advance".
6. **Structured, not flat.** Instructions carry nested blocks (`If`, `May`,
   `Modal`, `Repeat`, `Batch`, `Simultaneously`, `Each`, `Distribute`,
   `Search.if_none`). A def is visible to later instructions in its block
   and in nested blocks, never after the block closes. There is no phi: a
   value needed after a branch is defined before it.
7. **Captures.** `Delayed`, `Reflexive`, and carried bodies declare
   `captures: [RefId]`; the created region takes them as params with
   capture provenance, snapshotted at creation [CR#603.7,603.12]. Nothing
   else crosses a region boundary. The declaration IS that parameter list —
   `Region::captures()` reads it back as `(here, there)` pairs — not a
   separate field beside it; a second copy of the same fact could only drift
   from the params the engine actually indexes.

   **The snapshot chases once, at the boundary, and never again.** Inside one
   region a moved product is chased to its new incarnation for actions
   [CR#400.7j] while information reads fall back to the snapshot [CR#608.2h]
   (the cost stage's split). A capture is taken THROUGH that chasing read, at
   creation [CR#603.7a], and then frozen: what crosses is the object as the
   creating effect left it — the card in exile, not the permanent that left
   the battlefield. At the firing site it is not chased again. [CR#603.7c]
   settles this and is the sharp case's own rule: a delayed ability "won't
   affect" an object no longer in the zone it was expected to be in, and
   "if that object left that zone and then returned, it's a new object and
   thus won't be affected." So a capture behaves like a snapshot for
   IDENTITY across the boundary — actions find the captured object or
   nothing — while information reads still fall back to last-known
   information [CR#608.2h]; the LKI is refreshed as the object departs, so
   it is the state at departure, not at capture.

   **A declared capture is never unavailable at firing.** Three legs, no
   runtime hole: lowering emits a capture only for a register the enclosing
   region declares; `deckmaste_core::validate` refuses at LOAD any capture
   that names no definition of the creating region; and the engine snapshots
   every capture the created region declares at creation and supplies the
   list at region entry, so supply is total over `Region::captures()`.
   **A grant closes over the same capture ABI.** `GainAbility` is an
   ability-adding characteristic modification applied in layer 6
   [CR#613.1f], but the continuous effect that carries it is created by the
   resolving spell or ability [CR#611.2]. At that creation boundary the engine
   snapshots every executable region inside the granted ability, including
   each member of a composite keyword. The floating effect owns those closure
   values; layer derivation carries them beside the granted ability, and a
   later activation or trigger entry supplies them through
   `enter_created_region`. The granted-to object's source/controller and event
   roles remain the new region's intrinsic parameters; only its declared
   `Provenance::Capture` suffix comes from the granting resolution. Thus a
   controller change, later re-derivation, or composite expansion cannot turn
   a declared grant capture into `Value::Unavailable` or silently substitute
   the later bare frame.

   **Lowering is not context-free at ability granularity, permanently.** A
   carried region appends a capture parameter per enclosing register, so an
   ability lowered in isolation and the same ability lowered inside a card are
   different values, by design. The isolated-subterm equality check is
   therefore SUPERSEDED by the depth-zero check, not recoverable: a
   capture-erasing normalization would erase exactly the declaration this law
   introduces, and an ordinal-shifting one would need the isolated side to
   know the enclosing region's definition count — precisely the context it
   does not have. The every-depth coverage is kept as the two claims that DO
   hold in context (`every_semantic_ability_subterm_appears_at_its_own_depth_in_its_lowered_card`):
   the semantic subterm tree and the lowered nested-ability tree are the same
   shape at every depth, and a carried region's parameters are its intrinsic
   prefix — identical to the isolated lowering's — followed by nothing but
   captures.
8. **Linked memory** [CR#607]. A card declares its memory cells;
   `Remember { cell, kind, value: RefId }` writes one; a reading ability takes
   the cell as a param with linked provenance, supplied at region entry from
   the card's memory keyed by the object both abilities are printed on.

   **The `Remember` instruction IS the declaration.** A card's memory cells
   are exactly the cells its `Remember`s write, with the `kind` each declares;
   there is no separate cell list on the card shape. [CR#607.1] scopes linkage
   to two abilities "printed on" ONE object, so the card's own text is the
   whole vocabulary, and checking a read against it is a card-wide question
   either way. A read whose cell no ability on the card writes is a LOWERING
   error naming the card — not [CR#607.5a]'s runtime-undefined, which covers a
   GAINED reader whose linked writer was not copied, never two abilities
   printed together. Lowering compiles a card twice when (and only when) its
   first pass sees a cell read: pass one collects the reads and writes, pass
   two declares the survivors as parameters.

   A cell is per-OBJECT, so a permanent that leaves and returns reads nothing:
   it is a new object [CR#400.7] and its abilities are not linked to the
   departed one's writes. The published value is snapshotted like a capture —
   chased once through the writing effect's own move [CR#400.7j], never again
   — so [CR#607.2a]'s "cards in the exile zone that were put there as a result
   of" stops naming a card that has since left.

   **The paid-cost record is the one linked read that crosses a zone change.**
   `Condition::PaidCost` and `Count::TimesPaid` read the ACTIVATION's announced
   optional-cost record, never a stack scan by source id. [CR#702.33e] makes
   "if it was kicked" a linked ability read, and a kicked permanent's
   enters-the-battlefield recheck happens after the spell has left the stack —
   so [CR#400.7d] ("an ability of a permanent can reference information about
   the spell that became that permanent as it resolved, including what costs
   were paid to cast that spell") is what lets it answer, and the record
   crosses onto the permanent at the stack → battlefield remint. It rides the
   object rather than a memory cell because the two abilities are on two
   objects [CR#607.2i], which no per-object cell can span.
9. **Announcement is declared on the ability.** Targets are fields of
   `SpellAbility`, `ActivatedAbility`, `TriggeredAbility`, and `Mode`, never
   effect nodes [CR#601.2b,601.2c]. A cost is a field of the kinds that have
   one: spells and activated abilities announce and pay theirs, a mode carries
   its optional rider, and a triggered ability has none, since nothing about
   it is announced or paid. Amended 2026-09-02 to record that deviation, which
   the cost landing made and argued only in a code comment. The cost is a block run at
   announcement in the ability's own activation, so a paid product is a def
   the body reads. `OneShotEffect::Targeted` and the root `AdditionalCost`
   are deleted.
10. **Validate at load, fizzle at run.** `deckmaste_core::validate` checks
    def sequence, dominance, kind, region closure, and the expression
    carve-out. After it, a runtime read can only find a bound-but-departed
    value, which stays under [Invalid semantic input fizzles](invalid-semantic-input-fizzles.md).
    "Never bound" is unrepresentable.
11. **The engine environment is an activation record.** One activation per
    region entry: a `Vec<Value>` sized at load, kept in a per-resolution
    table on `GameState`; a work item carries an activation id, never a
    copy, because `Sequentially` clones frames into child work items up
    front. `Frame` collapses to that id plus payment state. `eval_reference`
    is an indexed read plus provenance dispatch returning one product (live
    id, optional LKI). A predicate over N candidates writes its candidate
    param N times, sequentially.
12. **Resolution happens once, in lowering.** Lowering is the compiler:
    scope elaboration, param declaration, lambda lifting for predicates,
    closure conversion for captures, R1/R2 as per-card diagnostics
    (`deckmaste_lowering::lower_card` returns the refusal as a `Diagnostic`
    naming the card and the antecedents that collided). The Idris mirror
    stays the certifier of semantic input, and the two derivations are
    gate-compared (§17's certifier/resolver split, unchanged) by
    `cargo xtask idris-check <plugin> --differential`, which pairs the two
    verdicts card for card and fails on any disagreement. Emitter gaps carry
    no certifier verdict and are skipped.

    R1/R2 refuse WITHIN a discourse tier, and region-local antecedents form a
    tier above ones captured from an enclosing region: a loop body's
    per-element magnitude is what "that much" means inside it, while two
    magnitudes pinned by the SAME region stay a compile-time refusal. The
    rules supply no proximity tiebreak to break that tie with — [CR#608.2c]
    rejects a step-by-step positional reading outright ("read the whole text
    and apply the rules of English"), and the CR's own mechanism for pinning
    which earlier value a later clause reads is linkage [CR#607.1], which is
    identity and never position. Naming the magnitude is therefore the
    card-side escape.
13. **Numeric ids, typed riders, a printer.** §14's rejection of stored
    binder names stands. Core is generated, so verbosity is free; a
    pretty-printer that shows riders is the debugging affordance.

## Rejected

- **A flat CFG with phi nodes (LLVM IR).** Built for optimization passes the
  engine never runs; this interpreter pauses on decisions and re-enters
  nested bodies.
- **A def per derivation.** Doubles the instruction count without adding a
  binding; `Let` covers the cases where the moment matters, and memoizing
  pure expressions stays an engine option.
- **A frame-local tier for loop variables and candidates** (the
  `core-reference-slots` default). A second environment with its own read
  rule is the hiding this decision forbids.
- **A register file inside `Frame`.** See law 11.
- **A `_v2` core beside the old.** The stage chain evolves one core and
  deletes each channel's old variants as its explicit form lands.

## Staging

Four stages, each behind `core-demacro`: after it, engine fixtures spell
semantics and `lower`, so the plugin corpus and the fixtures are untouched by
core-shape changes and each stage is lowering plus engine plus core types.

`core-regions-substrate` → `core-regions-discourse` →
`core-regions-costs-and-captures`, with `core-regions-piles` off the
discourse stage. Each landed stage carried residue into a follow-up rather
than reopening its own ticket, so the live node set is larger than the four:
`core-regions-substrate-closeout` (the compat register spellings and the
frame collapse), `core-regions-discourse-closeout` (runtime nearest-antecedent
search, the shared choice slot, per-card lowering diagnostics, the
resolver/certifier differential gate), `core-regions-test-restoration` (the
assurance the discourse stage deleted, plus that stage's seven named
fixtures), `core-regions-captures-and-memory` (laws 7 and 8, split off when
the cost half alone ran to ~4k lines), and `core-regions-witness-fixtures`
(hand-spelled semantic fixtures for the nineteen inherited witness cards,
which the wizards corpus cannot exercise). A stage is closed by its own
ticket plus its follow-ups, not by its ticket alone. Stage 3's grant-boundary
follow-up was `engine-granted-body-captures`; it closed law 7's final runtime
supply residue by carrying grant-time closure values through layer-derived
abilities.

Core's shape is independent of which semantics lowers into it. The chain
builds the resolver on today's `deckmaste_semantics` → core path as the
proof of concept; the `semantics_v2` being designed in the workbench gets its
own front half later and reuses the region-building and R1/R2 back half.
Nothing in this contract waits on `semantics_v2`.
