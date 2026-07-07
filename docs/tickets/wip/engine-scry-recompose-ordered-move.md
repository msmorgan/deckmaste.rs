---
needs: []
---
## Recompose Scry/Surveil/Fateseal onto Each/Modal/Move; port Arrangement + MoveArranged; delete the Distribute chain

Scry/Surveil/Fateseal currently ride a one-off `look-then-partition` primitive
(`PlayerAction::Distribute` + `Bin` + `PendingDecision::Distribute`). The Idris
north-star already commits the compositional encoding (`Macros.idr` `scry`), so
the bespoke chain can go.

### Target encoding — the committed north-star shape (`Macros.idr`)

Per-card top/bottom pick as a 1-of-2 `Modal`, distributed by a simultaneous `Each`
over the peeked top-N. The within-pile "any order" is the [CR#401.4] default
arrangement of the simultaneous `Each` batch — NOT a bespoke op.

- **Scry N** [CR#701.22a] — `Each(Existing(TopOfLibrary(n)), Modal{1-of-2}[ Move(It, Library(FromTop(0))), Move(It, Library(FromBottom(0))) ])`
- **Surveil N** [CR#701.25a] — same, spill zone is the graveyard: `[ Move(It, Library(FromTop(0))), Move(It, Graveyard) ]`
- **Fateseal N** [CR#701.29a] — Scry over an opponent's library (`TopOfLibrary(of: Opponent)` + the `FromTop/FromBottom` anchors on that library)

`Modal { choose: 1, modes: [...] }` is [CR#700.2]. `Each` (sole distributor),
`Modal`, `Move`, `Library(FromTop/FromBottom)`, `TopOfLibrary` all already exist
in Rust. Mill is the same family with no arrangement (graveyard is unordered):
`Each(Existing(TopOfLibrary(n)), Move(It, Graveyard))`.

### Port to Rust: Arrangement + MoveArranged (Idris-only today)

`idris/src/Core.idr` has `Arrangement = ChosenOrder | RandomOrder | SameOrder`
and `Action::MoveArranged : Selection -> Arrangement -> Destination`. Rust has
neither. Add both to match:

- `Arrangement` enum — `ChosenOrder` (owner arranges, the [CR#401.4] "any order"
  default), `RandomOrder` (shuffled into place, [MTR 3.10] — a randomized pile is
  the same kind of object as a shuffled library; NO player learns the order),
  `SameOrder` (preserve source order). Suffix keeps it distinct from
  `Selection::Random`.
- `Action::MoveArranged(Selection, Arrangement, Destination)` — put a GROUP at an
  ordered position with an explicit arrangement. A single-object `Move` carries
  none.

### Engine: arrange a simultaneous batch at an ordered position

When a simultaneous batch (one `Each` pass, or a `MoveArranged` group) lands at an
ordered-zone anchor (`Library(FromTop/FromBottom)`), arrange it per its
`Arrangement`:

- **`ChosenOrder`** (the default for a bare simultaneous `Move` batch, e.g. scry's
  two piles): surface an arrange `PendingDecision` so the controller orders that
  pile. The engine surfaces every choice.
- **`RandomOrder`** (explicit, via `MoveArranged`): randomize the pile's order;
  reveal nothing ([MTR 3.10]). NO arrange decision.
- **`SameOrder`**: keep source order; no decision.

**Scry never emits `MoveArranged` and never uses `RandomOrder`** — it relies on
the `ChosenOrder` default of its simultaneous `Each`. "Bottom in a random order"
(impulse / reveal-until, dozens of cards) is a *separate* future macro over
`MoveArranged(rest, RandomOrder, FromBottom(0))`. The two paths are distinct by
construction, so scry can't disturb random-order and vice versa. (The impulse
macro itself is out of scope here — this ticket only makes `RandomOrder`
representable and honored so scry's default is unambiguous.)

Fateseal's arrange decision is made by the fatesealing player over the opponent's
library.

### Delete (bespoke scry-partition machinery)

- `PlayerAction::Distribute` + `Bin`
- `PendingDecision::Distribute`, `WorkItem::OpenDistribute`,
  `ChoiceContinuation::Distribute`, `Step::DistributeOpened`
- `GameEvent::Distributed { name }` (+ its apply arm)

**Keep:** `Selection::TopOfLibrary`, `Destination::Library(Anchor)`, the
`Effect::With` group binder / `Those`/`That` anaphor, `Reference::Opponent`. **Do
NOT touch** the unrelated allotment anaphor `Distribute : Count b` (divide
damage/counters as you choose, core `effect.rs`) — different concept, same word.

### Rewrite

`plugins/builtin/macros/action/{Scry,Surveil,Fateseal}.ron` from the
`Distribute(bins:)` body to the `Each`/`Modal`/`Move` body above. Verify the RON
round-trips to the same Idris `scry`/`surveil` the north-star commits
(idris-check). Supersedes the encoding shipped in
`done/engine-scry-surveil-explore.md` (Explore stays split under `engine-explore`).

### Implementation plan (engine map done — file:line refs from an Explore pass)

Seven parts. Ordering decision resolved with the user: **implicit engine arrange**
(keep the `Each(Modal)` macro; engine arranges each ordered-zone pile after the
per-card picks).

1. **`Action::Composite { name, body }`** (new; Rust has only a doc stub,
   action.rs:345). Resolving it runs `body` and emits a named keyword-action
   event carrying `name` ("Scry"/"Surveil"/"Fateseal") — the trigger hook for
   "whenever you scry/surveil", replacing `GameEvent::Distributed`'s role
   (event.rs:366). Mirrors Idris `Composite`.

2. **Same-zone library reposition = NOT a zone change.** Scry never removes cards
   from the library ([CR#701.22a]), so `Move(It, Library(_))` when the object is
   already in that library must REPOSITION (no remint, no `ZoneChanged`, no
   zone-change triggers) — the old `apply_distribution` did direct `VecDeque`
   surgery for exactly this (decide.rs:1586). Today `Action::Move` (resolve.rs:1208)
   unconditionally emits `ZoneWillChange` + remints (step.rs:854). Add a
   same-zone Library→Library reposition path (correctness fix, per
   `fix-convenient-not-quite-right`). Surveil's spill IS a real zone change
   (Library→Graveyard) and keeps reminting.

3. **Post-pick arrange decision (option B).** `Each(Modal[Move…])` resolves
   sequentially (resolve.rs:700 — choice-bearing bodies aren't batched), so the
   "any order freebie" must be delivered: accumulate each per-card ordered-zone
   landing (owner-arranges / `AnyOrder` default) into a resolution-scoped buffer;
   a finalizer `WorkItem` after the `Each` elements surfaces ONE arrange
   `PendingDecision` per (owner, anchor-end) pile of >1, then reorders those
   objects in the library. New: `PendingDecision::ArrangePile` +
   `Decision::Arranged(Vec<ObjectId>)` + `ChoiceContinuation` to walk multiple
   piles + the finalizer WorkItem + the buffer on `GameState`. This same arrange
   surface fills the `MoveGroup` seam (resolve.rs:1313); `RandomOrder` (explicit
   `MoveGroup`, impulse family, out of scope here) randomizes instead of asking.

4. **Look-visibility grant.** `open_distribute` granted `look_grants`
   (step.rs:1497) for the peeked top-N; that grant must move to the
   `Existing(TopOfLibrary(n))` binder resolution so the controller sees the cards
   they're arranging.

5. **Delete the Distribute chain** (~20 sites): `Bin` + `PlayerAction::Distribute`
   (action.rs:14,342), `WorkItem::OpenDistribute` (agenda.rs:127),
   `Progress::DistributeOpened` (step.rs:113), `open_distribute` (step.rs:1486),
   `GameEvent::Distributed` (event.rs:365) + its apply/eval/render arms,
   `ChoiceContinuation::Distribute` (state.rs:192), `PendingDecision::Distribute`
   + `Decision::Distribution` + `submit_distribution` + `apply_distribution`
   (decide.rs:289,341,1538,1586), and the `strategy.rs:380` fallback entry. Keep
   the unrelated `Distribute : Count` allotment anaphor.

6. **Rewrite the 3 macros** to the `Each`/`Composite`/`Modal`/`Move` body; fateseal
   over `TopOfLibrary(of: Opponent)` with the arrange decision made by the
   fatesealing player over the opponent's library.

7. **Tests + idris-check.** Port the four Distribute tests (resolve.rs:7782) to the
   new shape; add arrange-decision + same-zone-reposition + scry-trigger tests;
   confirm the RON round-trips to the committed Idris `scry`/`surveil`.

Library rep: `Zones.libraries: Vec<VecDeque<ObjectId>>`, FRONT = top (zone.rs:9,
state.rs:390). `library_index`: `FromTop(c)=c`, `FromBottom(c)=len-c` (resolve.rs:1399).
