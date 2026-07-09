---
needs: []
design: true
---
Per-player redaction stays a runner-layer projection by design; core is
full-information. This item fixes the projection's shape and substrate so the
build is turn-key when face-down forces it — it ships the decision, not the code.

## Decision

The core/runner boundary holds. Redaction is a **runner-layer, read-time
projection**; core stays full-information (consistent with the seeded-RNG and
"concession rides every legal list; filtering is the runner's problem"
rulings — hidden-info is the same shape). No projection code ships in this
item; only the design is fixed.

## Projection shape — a borrowed accessor view

The eventual `PlayerView<'a> { state: &GameState, viewer: PlayerId }` is a
zero-copy read facade over the authoritative state. It holds no state of its
own — a pure function of `(state, viewer)` — so it cannot drift and needs no
lifecycle. Its home is the engine crate (beside the layer view), because the
visibility primitives live there and it is a pure read facade, not consumer
policy. Consumers read hidden zones through it instead of indexing the zone
store directly; redacting reads collapse hidden-from-viewer content to
cardinality (opponent hand → a count, library order → unknown).

## Visibility predicate — positional + look_grants

A viewer can see an object's identity iff it is in a public zone
([CR#400.2]: library and hand are the hidden zones), OR the viewer owns/controls
the hidden object, OR the pair `(viewer, object)` is in the engine's
`look_grants` ledger. `look_grants` is the existing persistent per-`(player,
object)` continuing-look permission ([CR#406.3]) that scry/surveil/fateseal
peeks and reveals write, and that shuffle/remint clears — so continuing
knowledge is modeled exactly to the extent the engine writes grants
([CR#701.20]: a reveal shows a card to players; reordering a library makes
revealed cards new objects, destroying order knowledge). Library order is never
exposed beyond the specific top-N objects a grant names. No separate memory
ledger: durable player recall beyond `look_grants` is the player's concern.

## Face-down is the forcing function — and the boundary survives it

Face-down objects (morph/disguise/manifest/cloak; see the face-down item) sit
in a public zone but carry hidden characteristics — a second redaction axis
orthogonal to zone visibility. The view absorbs this with a characteristic
chokepoint (`face(id)` → known-face or hidden) distinct from the zone
predicate. This forces **no core change**: core still knows which card a
face-down object is; only the view withholds the face. So face-down validates
the boundary rather than breaking it — the answer to "revisit the boundary when
face-down lands" is: the boundary holds, face-down is a second view axis, not a
core concern.

## Build trigger

Implement `PlayerView` when either the face-down item is claimed (characteristic
redaction becomes real) or a consumer needs genuinely non-cheating play (an
adversarial sim or a networked/server runner). Until then, cooperative
consumers (the TUI, demo bots) reading the full state is acceptable and
by-design. The migration is mechanical when the trigger fires: route the demo
`Strategy` and the TUI's hidden-zone/printed-face reads through the view, and
add the `can_see`/`face` tests.
