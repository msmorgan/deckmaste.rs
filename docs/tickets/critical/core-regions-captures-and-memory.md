
## Landing record

### Decision A — the snapshot/chased-id split for a capture: settled on [CR#603.7c]

**A capture chases ONCE, at the boundary, and is never chased again.** The
cost half's split (a moved product chased to its new incarnation for actions
[CR#400.7j]; information reads falling back to the snapshot [CR#608.2h]) is
what the capture is taken THROUGH, at creation [CR#603.7a] — so what crosses
is the object as the creating effect left it: for an exile-then-return card,
the card in exile, not the permanent that left the battlefield. Past the
boundary the split does not repeat.

The delayed-trigger case has its own rule, and it is decisive. **[CR#603.7c]**:
"A delayed triggered ability that refers to a particular object still affects
it even if the object changes characteristics. However, if that object is no
longer in the zone it's expected to be in at the time the delayed triggered
ability resolves, the ability won't affect it. (Note that if that object left
that zone and then returned, it's a new object and thus won't be affected …)"
— the parenthesis pointing at [CR#400.7]. A chase across the boundary would do
exactly what it forbids. So across a boundary a capture behaves as a SNAPSHOT for
identity — actions find the captured object or nothing — while information
reads still fall back to last-known information [CR#608.2h], refreshed at the
moment the object departs rather than frozen at capture.

[CR#400.7j] is what makes the capture land on the right object in the first
place, not a licence to keep chasing: it scopes the finding to "other parts of
that effect", and the effect that exiled the card is over by the time the end
step arrives. Confirmed against the archetype's own card — Flickerwisp's
official rulings say the exiled card returns "even if Flickerwisp is no longer
on the battlefield", and that "once the exiled permanent returns, it's
considered a new object with no relation to the object that it was".

Written into ADR law 7, with the guarantee it discharges.

### Decision B — the ability-subterm comparison: permanently superseded

Not recoverable, and the ADR now says so rather than leaving the test ignored.
Law 1 makes a carried region's value a function of its ENCLOSING register
file: `in_carried_region` appends a capture parameter per enclosing register,
so an ability lowered alone and the same ability lowered in a card are
different values by construction. Neither proposed rescue works — a
capture-erasing normalization erases exactly the declaration law 7 introduced,
and an ordinal-shifting one needs the isolated side to know the enclosing
region's definition count, which is precisely the context it does not have.

The test was NOT deleted. It is re-spelled as
`every_semantic_ability_subterm_appears_at_its_own_depth_in_its_lowered_card`,
which keeps the every-depth walk and asserts the two claims that DO hold in
context: (1) the semantic subterm tree and the lowered nested-ability tree are
the same shape at every depth — a nested ability dropped, duplicated or
re-parented by lowering fails; (2) a carried region's parameters are its
intrinsic prefix, byte-identical to the isolated lowering's, followed by
NOTHING but `Provenance::Capture` — law 7's "nothing else crosses a region
boundary", checked over the corpus instead of asserted. A semantic `Expanded`
node descends without consuming a core level (macro provenance is erased at
`lower`). The test carries its own anti-vacuity assertion: it fails if no
carried region in the corpus declares a capture.

### Per-item status

- **The live defect — FIXED.** A created body's declared captures are
  snapshotted at creation (`GameState::capture_snapshot`, read through the
  chasing register accessors and then frozen) and travel on
  `TriggerBindings::captures` through firing, the intervening-if gate,
  placement and resolution; `enter_created_region` supplies them at region
  entry. Supply is total over `Region::captures()` by construction, so a
  DECLARED capture can no longer resolve to an unavailable value at firing.
  `created_trigger_context` no longer hand-carries anything: its
  defending-player copy is deleted (the firing event's roles overwrote it in
  every path anyway — `Frame::defending_player` and
  `activation_defending_player` went with it), and `this` stays as what it
  always was, the created region's own `Source` parameter [CR#603.7d,603.7e],
  not a crossing.
- **Captures declared, not merely reserved — DONE.** `Region::captures()` and
  `Region::linked_cells()` read the declaration back off the params, so
  lowering and the engine agree on one list. `Provenance::Capture`'s and
  `Provenance::Linked`'s "reserved by a later stage" doc comments are replaced
  by the rules that govern them.
- **Linked memory [CR#607] — BUILT.** `OneShotEffect::Remember { cell, kind,
  value }` is the writer; a reading region declares `Provenance::Linked(cell)`
  and is supplied at entry from `GameImage.memory`, keyed by the object both
  abilities are printed on. Lowering reaches it: `Noting` emits a `Remember`
  when another ability on the card reads the cell, `ChooseValue` does the same
  for "the chosen [value]" [CR#607.2d], and
  `Reference::Linked`/`Count::Noted`/`Selection::AmongNoted` declare the cell
  as a parameter instead of panicking. `lower_card` compiles a card twice when
  — and only when — its first pass sees a cell read at all.
- **`Count::TimesPaid` / `Condition::PaidCost` — RE-POINTED.** Both read
  `activation_times_paid`, the announced record on the register file
  announcement and resolution share, written at promote. The stack scan by
  source id is gone. [CR#400.7d] carries the record onto the permanent at the
  stack → battlefield remint (`paid_costs_by_object`), which is what lets a
  kicked permanent's ETB recheck answer after the spell has left the stack —
  [CR#702.33e]'s linked read.
- **A noted product group has a core spelling — DONE.** It is a linked memory
  cell: the producing instruction writes its dest, `Remember` publishes it,
  the acting ability declares it. Both ignored tests are re-spelled and
  un-ignored.
- **The subterm comparison — SETTLED, above.**

### Numbers

| | count |
|---|---|
| restored | 0 |
| re-spelled | 3 |
| ignored with blockers | 0 added, **3 removed** |
| added | 15 |
| removed | 0 |

Re-spelled: `a_noted_product_group_can_be_acted_on` (the deleted
`Selection::PilesOf` stand-in becomes a real produced group published as a
cell, plus a NEW assertion that a graveyard bystander the clause did not put
there is excluded — the old spelling could not distinguish them);
`among_noted_constrained_quantity_surfaces_and_binds_chooser` (body unchanged,
which is the point — it already stood in the successor's shape and passes as
written; only its blocker prose was stale);
`every_semantic_ability_subterm_appears_in_its_lowered_card` →
`every_semantic_ability_subterm_appears_at_its_own_depth_in_its_lowered_card`
(false premise replaced by the two claims that hold; see decision B).

**Ignores closed (3), all named by this ticket:** the two "a noted product
group has no core spelling" blockers
(`resolve::action::tests::a_noted_product_group_can_be_acted_on`,
`resolve::effect::tests::among_noted_constrained_quantity_surfaces_and_binds_chooser`)
and the "lowering is no longer context-free at Ability granularity" blocker
(`corpus_identity::every_semantic_ability_subterm_appears_in_its_lowered_card`).
No `#[ignore]` was added. The 14 that remain in the tree
(`deckmaste_engine` lib 3, `payment` 8, `full_game` 1, `deckmaste_noncanon` 1,
`xtask` 1) carry blockers routed elsewhere and are untouched.

Added (15): engine `resolve::effect` — the exile-then-return delayed capture,
the [CR#603.7c] no-chase control, the per-iteration carried-body capture list,
the linked pair through a declared cell, and the per-object cell control (5);
engine `tests/stack.rs` — the kicked permanent's ETB recheck (1); core
`region` — capture out of range, capture of a declared register, capture at an
ability root, `Remember`'s register read, `Remember`'s cell-kind refusal,
`captures`/`linked_cells` (6); lowering `tests/linked_memory.rs` — the linked
pair's `Remember` + `Linked` parameter, an unread cell publishing nothing, a
dangling link as a card-named lowering error, and a granted `Composite` body's
capture list (4). Minus one: the re-spelled subterm test replaced its
predecessor rather than adding to it, so the file count is 15 new test
functions.

Nothing was deleted to reach green.

### Gate artifacts

- `cargo test --workspace`: 122 suites, **0 failed**. Per-suite ignored counts
  after: `deckmaste_engine` lib 733 passed / **3 ignored** (was 5),
  `tests/payment` 24 / 8, `tests/full_game` 4 / 1, `deckmaste_plugin`
  `corpus_identity` 7 passed / **0 ignored** (was 1), `deckmaste_noncanon` lib
  4 / 1, `xtask` lib 406 / 1; every other suite 0 ignored.
- `cargo test -p deckmaste_core -p deckmaste_lowering`: 50 / 736 / 3 / 4
  passed, 0 failed, **0 ignored** (core was 44).
- `cargo clippy --workspace --all-targets`: clean.
- `cargo xtask cite check`: 17857 citations, **0 stale**.
  `cargo xtask cite check --list-noncompliant`: empty.
- `cargo xtask idris-check plugins/canon`: baseline OK, 68 passes, 12 gaps.
- `cargo xtask idris-check plugins/canon --differential`: 68 agreed sound,
  0 agreed unsound, 12 skipped, **0 disagreements**.
- Wizards regenerated from scratch (`rm -rf plugins/wizards && cargo xtask
  generate plugins/wizards`, 30688 cards) and the four `build.rs` touched.
- `jj st` clean of generated artifacts.

### Deviations and additions

- **Law 8's "a card declares its memory cells" is realized as the `Remember`
  instruction itself**, not a new field on the card shape, and the ADR is
  amended to say so with its reason. **FLAGGED FOR REVIEW as a possible
  ticket-vs-ruling contradiction rather than a plain deviation.** The reading
  taken is that the card DOES declare its cells — in its own text, through
  the instruction that carries the cell name and kind — so the law is
  satisfied rather than overridden; a reviewer who reads law 8 as mandating a
  dedicated field on the card shape should treat this as a contradiction I
  resolved, and the ADR amendment as the thing to revisit. [CR#607.1] scopes linkage to abilities
  "printed on" ONE object, so the card's own text is the whole vocabulary and
  a reader has to be checked card-wide either way; a separate declaration list
  would be a second copy of the same fact, free to drift from the `Remember`s
  the engine actually executes. The cost of the literal reading was also real
  and bought nothing: `CardFace` is constructed by ~300 explicit struct
  literals across the engine's fixtures.
- **The load-time-or-lowering-error guarantee is split by what can fail
  where.** A capture is a LOAD-time refusal (`deckmaste_core::validate`
  already refuses one naming no definition of the creating region; three tests
  pin it). A linked read is a LOWERING refusal naming the card, because a cell
  read is only well-formed relative to the whole card's text, which is exactly
  what the two-pass compile sees and what `validate` (per region) cannot. Both
  are the ticket's "load-time or lowering error, never an unavailable value at
  firing".
- **`GameImage` gains `memory` and `paid_costs_by_object`** (two engine state
  fields beyond the ticket's letter). The first is law 8's store. The second
  is the [CR#400.7d] channel: the ticket requires the ETB recheck to work
  after the spell has left the stack, and the record it reads rides the stack
  entry, so something has to carry it across the remint.
- **`TriggerBindings` gains `captures`** — the channel already threaded from
  creation through firing, placement and resolution, so the capture list
  rides it rather than duplicating that plumbing on `CreatedTrigger`,
  `NotedTrigger`, `PendingTrigger` and `StackObject::Triggered`.
- **`activation_departed` now reaches past the activation table** into the
  delayed registry's captures and the memory store. Both hold frozen ids that
  outlive a resolution, so without it a captured object that departs later
  would carry last-known information from the wrong moment ([CR#608.2h]).
- **New testing fixture card `Kicked Beast`** — no canon card carries kicker
  on a permanent, so the ETB recheck has no witness without one. Added to the
  `kicker_game` deck beside the existing charm and chant.
- **`drain_passing_priority` (test fixture)** answers `OrderTriggers`
  ([CR#603.3b]) as well as priority: two delayed triggers of one player firing
  on the same end step must be ordered, and no existing helper could get past
  that stop.
- **CR citation corrected by `cite audit --diff`.** One site I authored cited
  **[CR#603.2e]** for "the firing event's roles are what the created body
  reads"; that rule is about "becomes"-worded trigger events and says nothing
  about roles. Re-cited to **[CR#608.2k]**, which is the rule that an ability's
  effect refers to what its trigger condition referred to.
- **`cite bless` was NOT run.** Every rule this round cites is already
  registered (checked by name against `cr-citations.lock`) and `cite check`
  reports 0 stale, so the lock is untouched — running `bless` would only prune
  entries cited from the regenerated, gitignored wizards corpus, as the
  previous landing recorded.
- **ADR laws 7 and 8 amended** with decisions A and B, the three-leg
  guarantee, the per-object cell rule, and the paid-cost channel; the Staging
  section names the follow-up.

### Residue routed

- **`engine-granted-body-captures`** (minted, `planned/`): a carried body
  reached by a GRANT (`GainAbility`, a keyword expansion) declares the uniform
  capture ABI but is entered later by the granted-to object with no creating
  register file, so its captures would be unavailable. Measured as unbuilt
  rather than broken: no corpus card reads such a capture — the fourteen
  nested subterms differ from their isolated lowering in `params` only, never
  in a body read. Recorded in ADR law 7 as residue with the ticket named.
- The pre-existing **[CR#607.2a]** cites on the "…this way" family that the
  discourse closeout left alone (`resolve/action.rs`, `resolve/effect.rs`) are
  now defensible: those sites sit on the two tests this round re-spelled onto
  a real linked reader, which is the reader [CR#607.2a] describes.

No STOP was taken.
