---
needs: []
---
**Finish the discourse stage's architectural half: no nearest-antecedent
search at runtime, no shared choice slot, real lowering diagnostics.** The
stage-2 landing (`core: complete discourse regions`, 2026-09-02) delivered
the instruction-with-dest vocabulary, region-shaped loops and predicates,
per-region-kind params, and zero literal register reads in engine
production code. Four contract items did not land. Standard constraints
apply.

## Scope

1. **Runtime R1 survives.** `engine/src/activation.rs` grew
   `activation_latest_object` and `activation_latest_number`, which scan
   the register file backwards for the newest value of a compatible kind.
   That is nearest-antecedent resolution, the thing
   [Core is explicit regions](../../decisions/core-explicit-regions.md)
   moves to lowering, reintroduced positionally. Three production callers:
   the shield subject and the produced-`That` snapshot in
   `engine/src/resolve/effect.rs`, and ward-{X} pricing in the same file.
   Lowering assigns each a param or def; the scan and both helpers delete.
   This is also why the zero-literal-`RefId` gate passes while these paths
   still read by position.
2. **The shared `chosen` slot survives** with its clearing discipline, in
   `engine/src/activation.rs` and written from `engine/src/payment/
   fulfill.rs`, read behind a presence check in `engine/src/resolve/
   effect.rs` and `engine/src/resolve/query.rs`. Absorbed ticket
   `engine-chosen-slot-provenance` exists to delete exactly this: the
   deciding instruction writes its own dest, and no reader tests a
   presence flag. Its fixture (a chooser nested under another chooser,
   `Random`, or `AmongNoted`) is owned by
   [[core-regions-test-restoration]].
3. **The `noted`/`noting` collection** is still written by `BeginNote` and
   `EndNote` with no remaining reader. Delete it or give it its reader.
4. **Lowering diagnostics are a bare assertion.** R1/R2 refusal panics
   through `assert!`/`expect` with no card name. Stage 2's ticket requires
   a per-card diagnostic carrying card context, and the ADR makes
   ambiguity a compilation error with provenance, not an eval-time
   refusal.
5. **The resolver/certifier differential gate does not exist.** The ADR's
   law 12 and `semantics-spelling-lowering.md` §17 pair lowering's
   computed resolution against the Idris mirror's certification over the
   canon slice. No xtask target compares them. Build it, or record in the
   ADR why the pairing is dropped.

6. **The magnitude anaphor never gets nearest-wins.** A card fixing two
   magnitudes and then reading a bare "that much" is refused by the
   uniqueness gate at lowering rather than resolving to the nearer
   antecedent, because the amount channel is the only anaphor channel
   with no site preference. The absorbed `engine-that-much-frame-scoped`
   ticket's wording, that such a card reads the semantic antecedent,
   implies nearest-wins was intended. The restoration landing pinned the
   refusal as the actual behaviour rather than changing the resolver, so
   this is a live contradiction between a pinned test and an absorbed
   ticket's intent. Settle it: either the amount channel gains a site
   preference and the pin flips, or the refusal is correct and the
   absorbed wording was wrong, recorded here.

## Gates

Standard constraints apply. A grep proves no backwards register scan
survives in `deckmaste_engine`, and no reader tests a `chosen` presence
flag. A deliberately ambiguous fixture card produces a lowering
diagnostic naming the card, not a panic. The differential gate runs over
canon and reports zero disagreements.

## Landing record

### Item 6, the live contradiction: the refusal is correct; the pin stays

Settled on the rules, not on convenience. **[CR#608.2c]** is the only rule
governing how a later clause reads an earlier one, and it rejects a positional
scan outright: "Don't just apply effects step by step without thinking in these
cases—read the whole text and apply the rules of English to the text." A CR
search for a proximity rule finds nothing — "nearest", "most recent",
"immediately preceding" and "ambiguous" return no rule about text
interpretation. What the CR *does* supply is linkage, **[CR#607.1]**: the second
ability "refers only to actions that were taken or objects or players that were
affected by the first, and not by any other ability" — identity, never position.
So nearest-wins has no rules basis, the pinned refusal
(`a_bare_that_much_after_two_magnitudes_is_refused`) is correct, and it was NOT
flipped. The card-side escape is to name the magnitude, already covered by
`a_named_magnitude_reads_the_clause_that_bound_it`.

The absorbed `engine-that-much-frame-scoped` wording was not wrong, but this
ticket's summary of it was. Its two clauses are separate claims:

1. "a card fixing two magnitudes reads the **semantic** antecedent" — semantic,
   not nearest. Where two magnitudes sit in the same region with no structural
   discriminator there IS no semantic antecedent, so refusing is the faithful
   reading, not a contradiction of it.
2. "an `Each` over players followed by 'that much' reads the **per-element**
   amount" — this is a scope rule, and it was **genuinely broken**. Probed and
   confirmed: "~ deals 2 damage to target opponent. Each player loses 3 life.
   You gain that much life." panicked as R2, because the loop body's own pinned
   amount competed with the enclosing region's.

Fixed by giving the resolver's general search two discourse tiers:
region-local antecedents outrank ones captured across a region boundary (ADR
law 7 — nothing crosses a region boundary except declared captures).
Grounded in **[CR#608.2h]** ("the answer is determined only once, when the
effect is applied" — a region is applied once per entry, so a loop body's
magnitude is a different determination) and **[CR#608.2f]** (each affected
player is processed individually). R2 still refuses *within* a tier, which is
why the pin survives unchanged: both fixture magnitudes are region-local. New
fixture: `that_much_in_a_loop_body_reads_the_bodys_own_magnitude`.

### Per-item status

1. **Runtime R1 survives — DONE.** `activation_latest_object` and
   `activation_latest_number` are deleted, with all three production callers
   converted to declared reads:
   - shield subject: `Action::CreateReplacement` gains `subject: Reference`;
     lowering resolves the anaphor and declares the register, the engine
     `eval_reference`s it.
   - produced-`That` snapshot: `created_trigger_context` now chases the frame's
     own SOURCE register through the same-resolution move record
     (`moved_source_snapshot`) instead of grabbing the newest object of any
     kind, which could just as happily have returned an unrelated token.
   - ward-{X} pricing: `TriggeredAbility` gains `where_x: Option<Count>`;
     it is evaluated once at resolution into the region's declared announced-X
     parameter (`Provenance::AnnouncedX`), and `price_variable_cost` reads
     `activation_x`. Lowering no longer mints a `Let` + `set_x` for it, so the
     value cannot be stolen by any magnitude the body pinned more recently —
     which is exactly the bug the old scan had.
2. **The shared `chosen` slot — ALREADY GONE; residue cleaned.** No slot, no
   clearing discipline, no presence-flag reader survives: the deciding
   instruction writes its own dest (`activation_write_objects(.., dest, ..)`).
   What remained was stale prose referring to `frame.anaphora.chosen` in five
   places; retired so the gate grep reads clean and no reader is misled.
3. **The `noted`/`noting` collection — DELETED.** It had no production writer
   or reader once the discourse stage removed `Noting`/`Selection::AmongNoted`.
   `GameState.noted`/`noting`, `NotedMember`, `WorkItem::BeginNote`/`EndNote`,
   `Progress::NoteScoped` and the `note_enacted` hook are gone. The store was a
   *cache* of exactly the enacted past-form `ZoneChange` facts `GameState.history`
   already keeps, so its tests were re-spelled onto those facts (`enacted_moves`)
   rather than deleted — see counts below.
4. **Lowering diagnostics — DONE.** `deckmaste_lowering::lower_card` installs
   the card as the resolver's diagnostic context and returns an R1/R2 refusal as
   a `Diagnostic { card, message }` naming the card and the antecedents that
   collided, instead of an anonymous panic from inside the tree walk.
   `Plugin::card_from_str` routes through it rather than re-deriving the name.
5. **The differential gate — BUILT.**
   `cargo xtask idris-check <plugin> --differential` pairs lowering's computed
   resolution against the Idris mirror's certification card for card. Emitter
   gaps carry no certifier verdict and are skipped; a card the mirror neither
   certifies nor refutes and that is not a listed gap FAILS the gate rather
   than counting as agreement. The verdict table is a pure function
   (`pair_verdicts`) with its own tests, so "0 disagreements" is
   distinguishable from a gate that cannot fail. ADR law 12 now names the
   command.
6. **The magnitude anaphor — SETTLED, above.**

### Numbers

| | count |
|---|---|
| restored | 0 |
| re-spelled | 6 |
| ignored with blockers | 0 added, 0 removed (2 blockers reworded) |
| added | 7 |
| removed | 0 |

Re-spelled: `cards_milled_this_way_reads_the_enacted_product_group` and
`destroyed_this_way_product_group_excludes_indestructible_survivor` (noted store
→ history facts; the Blood-Money test also GAINED an explicit assertion that the
indestructible survivor is absent, which the old spelling only implied);
`a_noted_product_group_can_be_acted_on` and
`among_noted_constrained_quantity_surfaces_and_binds_chooser` (both already
`#[ignore]`d — bodies re-spelled off the deleted store, blockers reworded to name
what is actually missing, ignore status unchanged);
`regenerate_macro_expands_with_typed_reference_param` (now asserts the shield's
subject IS the register the preceding `Let` defined);
`create_shield_rejects_non_sweepable_duration` (passes the declared subject).

Added: `that_much_in_a_loop_body_reads_the_bodys_own_magnitude`;
`crates/deckmaste_lowering/tests/diagnostics.rs` (3 — the ambiguous fixture card
names itself, a refusal does not poison the next card, a resolvable card lowers
through the same entry); `xtask::idris_check::differential_tests` (3 — the
verdict table agrees, disagrees in both directions, and never calls a gap a
disagreement).

Nothing was deleted to reach green. No `#[ignore]` was added, and none removed:
this round closes no other ticket's blocker.

### Gate artifacts

- `cargo test -p deckmaste_core -p deckmaste_lowering`: 44 / 736 / 3 passed,
  0 failed, **0 ignored**.
- `cargo test -p deckmaste_engine`: all suites 0 failed; ignored counts 5
  (lib), 1 (`payment`), 8 (`skeleton`) — unchanged from the claim.
- `cargo test -p deckmaste_plugin`: all suites 0 failed; ignored count 1
  (`canon`) — unchanged.
- `cargo clippy --workspace --all-targets`: clean.
- `cargo xtask cite check`: 17738 citations, 0 stale.
  `cargo xtask cite check --list-noncompliant`: empty.
- `cargo xtask idris-check plugins/canon`: baseline OK, 68 passes, 12 gaps.
- `cargo xtask idris-check plugins/canon --differential`: 68 agreed sound,
  0 agreed unsound, 12 skipped, **0 disagreements**.
- Grep gates: no backwards register scan survives in `deckmaste_engine`
  (both helpers deleted; `activation.rs` has no `.rev()` over `values`), and
  no reader tests a `chosen` presence flag.
- Wizards regenerated from scratch (`rm -rf plugins/wizards && cargo xtask
  generate plugins/wizards`, 30688 cards) and the four `build.rs` touched.

### Deviations and additions

- **`Action::CreateReplacement` gains `subject: Reference`** (core type change
  beyond the ticket's letter). Required: item 1 says "lowering assigns each a
  param or def", and there was no field to assign to. Semantics is unchanged —
  English leaves the protected permanent to the discourse, so lowering resolves
  it.
- **`TriggeredAbility` gains `where_x: Option<Count>`** (core type change).
  Same reason for the ward toll. This also moves the "where X is …" value out
  of a body `Let` and into the region's declared announced-X parameter, which
  is what made the declared read possible; `region::set_x` is deleted as dead.
- **`Plugin::card_resolution_from_str` + `CardResolution`** — a new entry
  inside the restricted read API (spec §4), not beside it. The first attempt
  used `rendering_card_from_str` from xtask and `read_api_gate.rs` correctly
  failed it; the entry now returns the card's printed name with the verdict so
  no second read is needed.
- **CR citation corrections found by `cite audit --diff`.** Three shield sites
  cited **[CR#614.3]** for a claim about the shield's SUBJECT, but that rule is
  about casting restrictions and duration. Re-cited to **[CR#614.1]**, which is the
  rule that says a replacement effect acts "like a shield around whatever
  [it's] affecting". Separately, four "…this way" sites I authored cited
  **[CR#607.2a]**, which is exile-specific linked abilities and does not cover
  a back-reference inside one ability's text; re-cited to **[CR#608.2c]**.
- **`cite bless` was run and REVERTED.** It pruned 23 lock entries that are
  cited only inside the regenerated, gitignored `plugins/wizards` corpus — a
  local artifact, not a real citation change. `cite check` was already 0 stale
  and every rule this round cites was already registered, so the lock is
  untouched.
- ADR law 12 amended to name the differential gate command and to record the
  discourse-tier rule with its two CR grounds.

### Residue routed

- Pre-existing **[CR#607.2a]** cites on the "noted product group" family that
  this round did not author (`resolve/action.rs:1755`, `:6005`;
  `resolve/effect.rs:2623`, `:6453`) are left alone. Those sit on the two
  ignored tests routed to the linked-memory stage, where a linked reader makes
  **[CR#607.2a]** defensible; a sweep of the wider "this way" family belongs
  with **core-regions-captures-and-memory**, which owns that reader.
- `Selection::PilesOf` remains an engine seam
  (`resolve/query.rs`) — **core-regions-piles**.
- A core term that NAMES an enacted product group (so
  `a_noted_product_group_can_be_acted_on` can be un-ignored) —
  **core-regions-captures-and-memory** (ADR law 8: `Remember` writes a cell, a
  reading region takes it as a `Linked` param). The group itself survives as a
  history query; only the term is missing.

No STOP was taken.
