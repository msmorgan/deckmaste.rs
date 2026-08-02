---
needs: []
design: true
---
Build the P0.W5 noted-slot store and close the two P0.W4 seams that read it:
`PlayerAction::ChooseAndNote(..)` (`resolve.rs` `todo!("P0.W4: choose-and-note
(slot store is P0.W5)")`) and `Count::Noted(key)` (`resolve.rs` `todo!("P0.W4:
noted read …")`).

The noted surface is wider than the two seams: `ChooseAndNote(Ident,
NotedKind)` (action.rs), `Count::Noted(Ident)` (count.rs),
`Selection::AmongNoted(Ident, Quantity)` (selection.rs), and the
Whims-of-the-Fates pile source `ChoosePile(from: Noted(note, of))`
(effect.rs). The store must serve whichever readers this ticket wires; the
rest stay loud per-site.

Design boundary to settle before building: the relation to
`engine-linked-abilities` (`Reference::Linked(Ident)`, the [CR#607]
linked-ability information store). Notes are choice anaphora ("the chosen
color"); linked info is what an earlier linked ability did or affected
(exile-then-return). One store or two, and which ticket owns the shared
machinery, is the design dialogue.

Scope of note lifetime matters: per-resolution ("choose a color" read later
in the same effect), per-object ("as ~ enters, choose a color" read by its
other abilities while it remains), and game-scope designations are already
separate machinery (`state.designations`).

---

## Spec (design settled 2026-07-12)

Rulings: note lifetime = the resolution ([CR#608.2c] — a mid-resolution
choice is announced while applying the effect [CR#608.2d] and exists only within that instruction sequence; values that outlive
resolution are linked abilities [CR#607] or as-enters choices, owned by
engine-linked-abilities / core-as-enters-choices); AmongNoted
constrained-quantity chooser included; missing/mistyped note reads fizzle
locally per the engine-stat-none-fizzle ruling (never panic, never silent 0).
Kind wiring is READER-GATED: only kinds with existing read grammar get built;
write-only kinds stay loud.

### 1. Resolution note store

A literal `Frame.anaphora` map does NOT work: `Sequentially` clones the frame
into every child work item up front (resolve.rs:524-533), so a note written
during child 1 cannot reach child 2's already-cloned frame. Instead:
`GameState.resolution_notes: HashMap<Ident, NotedValue>` — a
resolution-scoped store (stack discipline = one resolution active at a time),
CLEARED when the resolving entry completes (the AbilityResolved /
spell-resolved emission point; comment the [CR#608.2c] scope rationale).
`NotedValue` is an engine enum; this ticket mints only `Number(Uint)`.
Readers are `&self` — plain map reads.

### 2. ChooseAndNote — kind-gated

`player_action_items` arm (resolve.rs:2056), shape per the ChooseNewTargets
precedent (verb → WorkItem → PendingDecision → submit writes):

- **`NotedKind::Number`**: new WorkItem + PendingDecision (a resolution-time
  number choice — reuse the XValue answer shape if it fits, else a new
  Decision variant; comment the pick). Submit writes
  `resolution_notes[key] = Number(n)`. Unbounded like XValue.
- **`NotedKind::Objects`**: surface via the existing `ChooseObjects`
  decision (decide.rs:307, `BindChoice` submit precedent at
  decide.rs:1120-1137) but the submit writes the EXISTING
  `state.noted[key]` group (`NotedMember` snapshots, state.rs:449) instead
  of `frame.anaphora.chosen` — read back by `AmongNoted`/future `Linked`.
- **`Color` / `CardName` / `Piles`**: loud per-kind `todo!` naming the kind —
  no reader grammar exists yet (no OfChosen-equivalent predicate); write-only
  wiring would be dead machinery.

New decision kind(s) get `decider_player`/`lock`/`visibility` arms, the
mechanical-strategy arm in sim.rs (Number → 0, the X=0 precedent) and
`pending_player` — closing that slice of the P0.W3 seam; runner/TUI exposure
per the boundary convention (engine enumerates/validates, client presents).

### 3. Count::Noted (resolve.rs:3270)

Read `resolution_notes[key]`: `Number(n)` → `n`. Missing key or non-Number
value = authoring mistake → fizzle the consuming read per the
engine-stat-none-fizzle ruling — follow the `unbound_ref` no-op pattern
(resolve.rs:2858-2865) for whatever fizzle channel `eval_count` can express;
if `eval_count` cannot fizzle without invasive plumbing, the arm returns 0
WITH a loud debug_assert + comment linking the stat-none-fizzle ticket as the
principled fix — never a silent bare 0.

### 4. AmongNoted constrained quantity (resolve.rs:2677)

Wire the chooser: candidates = the noted group's live members (the
`m.now` filter already at resolve.rs:2685-2689), surface `ChooseObjects`
with the quantity's bounds, `BindChoice` re-run binds the picks. The
unconstrained full-group read stays as-is.

### 5. Boundaries (unchanged tickets)

`Reference::Linked` stays an unbound-ref fizzle (engine-linked-abilities —
needs per-(ObjectId, Ident) association, a different store).
`Selection::PilesOf` / `SeparatePiles.note` stay unbuilt (pile store).
No Idris changes: Idris already proves note scope structurally
(chosenKind/WithChosenValue, Core.idr:2426); the Rust runtime store follows
its lexical-lifetime model.

### 6. Tests

- Round-trip: ChooseAndNote(Number) surfaces decision, submit stores, later
  `Count::Noted` in the SAME resolution reads it (Sequentially shape — the
  cloned-frame trap is exactly what this asserts against).
- Scope: a note is gone after the resolution completes (next resolution's
  read fizzles/0-asserts, not stale).
- Objects: ChooseAndNote(Objects) writes state.noted; AmongNoted reads it;
  constrained AmongNoted surfaces ChooseObjects, honors quantity bounds,
  binds picks (pattern: destroyed_this_way_* fixtures resolve.rs:6176-6270).
- Missing-key Count::Noted: fizzle path, never panic.
- Loud kinds: Color/CardName/Piles still todo!-panic (pattern-match the
  message).
- Strategy: mechanical Number choice = 0; new pending kind has
  pending_player coverage (sim.rs).

### 7. Gates

nightly fmt; clippy zero warnings; `cargo test -p deckmaste_engine`;
`cargo test -p deckmaste_plugin`; cite check + list-noncompliant + bless/audit
(verify [CR#608.2c] text against the scope claim before citing). No wizards
regen (engine-internal types only) unless a core type changes.
