---
needs: []
---
**`DesignationScope` has no `Card` variant, so Commander is declared
object-scope — which the CR explicitly denies.** [CR#903.3]: the commander
designation "is not a characteristic of the object represented by the card;
rather, it is an attribute of the card itself. The card retains this
designation even when it changes zones."

`plugins/builtin/macros/designations/Commander.ron` declares `scope: Object`
and quotes that same clause in its comment as the justification. The
`persistence: Permanently` on the row is the compensating patch: object-scope
state would otherwise expire on a zone change, so the declaration buys
zone-independence from the persistence axis instead of the scope axis.

Three further confirmations that this is card state, not object state:

- [CR#400.7]'s exception list (a–m) has no commander entry. Object state that
  survives a zone change needs one; commander does not, because it is never
  object state.
- [CR#903.3e] lets effects read a commander's characteristics "in all zones,
  including that player's library and hand", so the carrier must stay readable
  in hidden zones rather than only on battlefield objects.
- [CR#903.3] fixes the designation at deck construction ("each deck has a
  legendary card designated as its commander"); [CR#903.3a] and [CR#113.6n]
  put the abilities that modify it before the game begins.

**Design point to settle before implementing.** [CR#903.3b] and [CR#903.3c]
make the melded/merged permanent the commander, and [CR#903.3d] phrases the
common queries per-zone ("control a commander", "cast a commander"). So card
scope needs an object-facing projection. Decide whether `Card` is a third
stored scope with the query layer projecting onto objects, or whether the
carrier is modelled outside the designation registry as deck-construction
data.

Uniqueness stays `None` — a deck may designate two carriers
([CR#702.124b] partner commanders).

Mirror surfaces (39 sites, 9 files — `DesignationScope` is mirrored, so a new
variant lands in all of them):

- `crates/deckmaste_core/src/designation.rs` — the enum
- `crates/deckmaste_semantics/src/designation.rs` — semantics mirror
- `crates/deckmaste_lowering/src/designation.rs` — the `Lower` impl
- `crates/deckmaste_plugin/src/idris_emit.rs` — `designation_scope_idris`,
  which maps to an Idris `Scope` token carrying only `Object`/`Player`
  (`Game` returns `None`)
- `idris/src/Core.idr` — note the scope pair is mid-refactor from closed
  `counterScope`/`designationScope` to dependent indexing; sequence against
  that rather than extending the closed form
- `plugins/builtin/macros/designations/Commander.ron` — the row, and its
  self-contradicting comment
- `crates/deckmaste_plugin/tests/builtin.rs` — pins the wrong value today
  (`assert_eq!(scope_of("Commander"), DesignationScope::Object)`)

Standard constraints apply.
