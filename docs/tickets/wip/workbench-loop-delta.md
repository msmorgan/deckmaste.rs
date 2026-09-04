---
needs: []
---
**Return the binding delta explicitly from `effProfile` instead of recovering
it by length, so a loop body's in-place stack mutations survive the loop.**
Fresh workbench review 2026-09-03, F2 — a soundness hole, not an ergonomics
one.

`Effect.effDelta e = take (length (effIntro e) minus length bs) (effIntro e)`
assumes intros only prepend. They do not: `Phrase.moveIntro`,
`Phrase.setZoneReach`, `Words.groupSpent`, `Words.afterShuffle`,
`Words.dropLetter` and `Words.publicOnly` mutate or remove existing bindings.
So `effIntro (ForEachOf grp body)` and `effIntro (Repeated n body)` rebuild
from the *original* `bs` and the body's zone changes vanish.

Probes:

- P8 `[tap (target creature), ForEachOf (each Opponent) (exile You (It OneOf)), untap (It OneOf)]`
  — **admitted**. The creature is exiled inside the loop and untapped after it.
- P8b the same sentence without the loop — correctly refused (stale zone).

The same length arithmetic is in `Macros.itPrior`, `Macros.dealsDamageOwnPower`,
`Effect.doesEffIntro` and `Effect.distributedDelta`.

Fix: have `Effect.EffProfile` carry the split explicitly — `delta : List
Binding` and `rest : Bindings` — rather than letting callers recover it. The
`ForEachOf` clause then returns `pluralizeDelta delta ++ dropElem rest`: drop
the `elemIntro` binding, keep the mutated tail. `Repeated` and `Enact` take
the same shape, and the four macros read the profile instead of subtracting
lengths.

Size: M.

Done when: P8's sentence is refused for the same reason P8b's is, pinned in
`ProofsZone` (or `ProofsTurn`) and probed non-vacuous; a loop whose body
genuinely leaves the subject readable afterwards is a positive bench witness;
`effDelta`'s `take`/`minus` is gone from the tree and no caller reconstructs a
delta from two lengths; every existing pin that twins a loop still refutes for
its own stated reason; the build is 44/44 with 0 errors and 0 warnings.
Standard constraints apply, plus the RON-shaped constraint: a core constructor
is admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

- `Effect.KeepsOuter e = instrIntro e = instrDelta e ++ bs` — the assumption
  `instrDelta`'s length arithmetic was silently making, promoted to a checked
  obligation.
- `ForEachOf`, `Repeated` and `ForEachKindOf` each carry
  `{auto 0 ko : KeepsOuter body}`. A body whose intro mutates the enclosing
  stack (a `Move`/`Enact`-move reaching an object bound outside the loop, a
  `groupSpent`/`afterShuffle` deletion) no longer typechecks, so the loop can
  never republish the stale pre-loop stack.
- `Macros.itPrior` reads `Own OneOf (instrDelta prev) bs {sp = ko}` under the
  same obligation; its `drop (length … minus length …)` outer half and the
  `takeDropAppend` lemma that justified it are gone.
- `ProofsZone`: `okLoopMovesItsOwn` (positive twin — a loop body that moves
  only what it introduced, read back after the loop as a plural),
  `badLoopedZoneMoveRead` (P8, refused at `KeepsOuter`),
  `badUnloopedZoneMoveRead` (P8b, refused at `untap`'s battlefield gate),
  `okKindLoopKeepsOuter` + `badKindLoopZoneMoveRead` (the `ForEachKindOf` twin
  and pin). Citations `[CR#400.7,701.26b]`.

## Landing record

Numbers before/after: 44/44 modules both; `Effect` check `1m42.8s` real /
`48.9s` user before, `1m48.3s` real / `55.9s` user after. Clean whole-model
build after: `5m23.1s` real / `3m52.2s` user, exit 0 (four sibling rounds were
building concurrently; the pre-change clean build measured `2m50.7s` /
`2m20.8s` on a quieter machine).

Gates (all foreground):

- `cd idris && ./scripts/build` → exit 0, `44/44: Building Cards
  (src/Cards.idr)`, 0 lines matching `Error|Warning`.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 18170 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite bless` → `blessed 1583 rules`; `cr-citations.lock`
  unchanged (both cited rules were already registered).
- `jj --no-pager diff --git > /tmp/ld.diff && cargo xtask cite audit --diff <
  /tmp/ld.diff` → `audited 3 citation site(s)`; each read against its claim.
  [CR#400.7] (new object on a zone change) and [CR#701.26b] ("Only tapped
  permanents can be untapped") both support the refusal they are cited for.

Probes (scratch module, deleted after each run):

- before: P8 typechecked, P8b failed on `So (zoneIsB (zoneOfReach Bare OneOf
  (instrIntro (exile You (It OneOf)))) Battlefield)`.
- after: P8 fails at `ForEachOf`, the message showing `setZoneReach Bare OneOf
  (Just "Exile") (Just Exile) …` on the left of the obligation against the
  un-mutated base on the right; the `ForEachKindOf` twin of P8 fails the same
  way; `okLoopMovesItsOwn` typechecks.

Assurance counts: restored 0, re-spelled 0, ignored 0, added 5
(`okLoopMovesItsOwn`, `badLoopedZoneMoveRead`, `badUnloopedZoneMoveRead`,
`okKindLoopKeepsOuter`, `badKindLoopZoneMoveRead`), removed 1 —
`Macros.takeDropAppend`, a proof helper (not a witness or pin) whose only
consumer, `itPrior`, no longer reconstructs the split from two lengths.

### STOP — the prescribed fix is not compositional

The ticket asks for `delta : List Binding` / `rest : Bindings` on
`InstrProfile`, with `instrIntro e = delta ++ rest`. That split cannot be
defined for every constructor at this ticket's size, and the obstruction is
structural, not incidental:

- `instrIntro (Sequentially es) = instrsIntro es`, where the tail is typed over
  `instrIntro e` — the earlier step's delta and rest already concatenated. A
  later step's in-place mutator (`setZoneReach`, `groupSpent`, `afterShuffle`,
  `dropLetter`, `defineLetter`) applied to `delta ++ rest` cannot be re-split
  without a split-aware mutator, and a split-aware mutator needs
  `(X ++ d) ++ r` to be definitionally `X ++ (d ++ r)` with `d`, `r` abstract,
  which `List.(++)` (recursion on its first argument) does not give.
- The only definitional splits available for `Sequentially` are `delta = []`
  (loops then stop pluralizing their bodies' introductions — permissively
  unsound, and it would strand `Repeated`'s bench cards, e.g. Careful Study's
  `Repeated (Lit 2) (Sequentially [choose …, discard …])`) or `rest = []`
  (loops pluralize the enclosing stack). Both are worse than the hole.
- Defining `instrDelta` per clause is likewise blocked: an accumulating
  `seqDelta` yields the UNMUTATED earlier binding (hand, not graveyard) where
  today's length prefix reads the real final stack, so `instrIntro = delta ++
  bs` would then fail for Careful Study.

The honest fix is to make the split primary — index the `Instructions`
telescope by a segment stack (`List Bindings`) rather than by `instrIntro`, and
give every mutator a segment-aware form, so a loop pushes a segment and pops it
with the body's own introductions separated from the mutated enclosing stack.
That is a grammar-wide change with its own design round, not an M ticket.

Resolution taken: rather than land nothing, this round converts the unchecked
assumption into a checked one. The length prefix is still the most accurate
delta available (it reads the real final stack); what was missing was any check
that it was legitimate. `KeepsOuter` is that check, it refuses exactly the
sentences the loop clause would have mis-published, and it refuses none of the
44-module corpus. **The conductor should decide whether the segment-indexed
telescope is worth a follow-up chain before this ticket is called done.**

### Deviations and additions

- Deviation (see STOP): `instrDelta`'s `take`/`minus` and `distributedDelta`'s
  survive; the ticket's "gone from the tree" clause is not met. `Macros.itPrior`
  and `Macros.dealsDamageOwnPower` no longer subtract lengths.
- Addition: `ForEachKindOf` was guarded too, beyond the ticket's named pair. It
  has a strictly worse form of the same hole — `instrIntro (ForEachKindOf …) =
  bs` discards the body entirely — and was probed admitting the `ForEachKindOf`
  twin of P8 before the guard.
- Addition: `badUnloopedZoneMoveRead` pins P8b, which was refused but unpinned,
  so the pair reads as a pair.
- Attempted and reverted: the same obligation on `Enact` (via
  `EnactKeepsOuter`/`KeepsOuterEach`, dispatching on `nounPlur s`). It closes
  the distributive hole — `Macros.exile (Macros.each Opponent) (Macros.It
  OneOf)` followed by `untap (It OneOf)` is admitted today — but refuses the
  plurality-polymorphic macros at their definitions (`exile`, `sacrifice`,
  `discard`, `puts`, `mills`, `scry`, `surveil`, `shuffleInto`), because
  `nounPlur agent` is abstract there. Closing it means threading the obligation
  through that macro family; **open ledger item for a live ticket**:
  `doesInstrIntro ManyOf` / `distributedDelta` still republish `nomIntro s`
  over a mutated stack.
- Deviation: no new `Cards/` entry. The positive witness lives beside its pin
  in `ProofsZone` (the house rule requires the twin in the pin's module) and
  names the printed pattern it spells (Breach the Multiverse's middle pair). A
  corpus sweep of `select(.supported)` found the shape in Breach the
  Multiverse, Afterlife from the Loam and the Encore reminder text; none spells
  in one clause pair without unrelated riders (delve, "under your control", "in
  addition to its other types").
