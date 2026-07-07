---
needs: []
---
## Recompose Scry/Surveil/Fateseal onto With/Each/Modal; delete the bespoke Distribute chain

Scry/Surveil/Fateseal currently ride a one-off `look-then-partition` primitive
(`PlayerAction::Distribute` + `PendingDecision::Distribute`). The partition is
expressible with grammar we already have — a per-card modal choice over the
looked-at group — so the bespoke chain can go, and the pile-ordering freedom
moves onto the `Move` primitive where it generalizes.

### Target encoding (per keyword-action macro)

```
With(look at top N, Each(That, Modal { choose: 1, modes: [ <dest-a>, <dest-b> ] }))
```

- **Scry N** [CR#701.22a] — modes `[Move(It, TopOfLibrary), Move(It, BottomOfLibrary)]`
- **Surveil N** [CR#701.25a] — modes `[Move(It, TopOfLibrary), Move(It, Graveyard)]`
- **Fateseal N** [CR#701.29a] — Scry over an opponent's library (the `With` look +
  the `Move` destinations target the opponent's library instead of yours)

`Decide([a, b])` in discussion == `Effect::Modal { choose: 1, modes: [a, b] }`
([CR#700.2]). `With` (group binder), `Each` (sole distributor), `That`, and
`Move` all already exist; no new grammar. The looked-at top-N binds as the `With`
group; `Each(That, …)` distributes the per-card top/other decision.

### Engine change: ordered-zone Move surfaces an insertion-order choice

[CR#701.22a] / [CR#701.25a] / [CR#701.29a] all grant "in any order" to **both**
resulting piles. The per-card modal captures the *partition* but not the
*intra-pile order* (e.g. Scry 2 keeping both on top — which you draw first is your
choice). Put the ordering on the destination, not on scry:

- When a **batch of >1 object** lands in an **ordered zone position**
  (`TopOfLibrary` / `BottomOfLibrary`) within a single resolution, `Move`
  surfaces an insertion-order `PendingDecision` so the controller sorts that pile.
- This generalizes for free to any multi-card library manipulation (Brainstorm's
  put-back, Fact-or-Fiction's top pile, etc.) — ordering is a property of moving
  into an ordered zone, not a scry-specific mechanism.
- Single-object moves surface no choice (no ordering freedom to exercise).

Engine surfaces every choice, so the ordering decision is a real
`PendingDecision`, not auto-resolved.

### Delete (bespoke scry-partition machinery)

- `PlayerAction::Distribute` + `Bin`
- `PendingDecision::Distribute`, `WorkItem::OpenDistribute`,
  `ChoiceContinuation::Distribute`, `Step::DistributeOpened`
- `GameEvent::Distributed { name }` (and its apply arm)

**Keep:** `Selection::TopOfLibrary`, the `Effect::With` group binder / `Those`/`That`
anaphor, `Reference::Opponent`. **Do NOT touch** the *unrelated* allotment anaphor
`Distribute : Count b` (the divide-damage/counters-as-you-choose distribution,
core `effect.rs`) — different concept, same word.

### Rewrite

The three `plugins/builtin/macros/action/{Scry,Surveil,Fateseal}.ron` macros to the
`With`/`Each`/`Modal` body above. Supersedes the encoding shipped in
`done/engine-scry-surveil-explore.md` (Explore stays split out under
`engine-explore` — reveal-then-branch, different shape).
