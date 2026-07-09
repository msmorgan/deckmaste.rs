---
needs: [engine-face-down]
---
Build the per-player redacted view whose shape was fixed by the
`runner-hidden-info` decision. A borrowed accessor `PlayerView<'a> { state:
&GameState, viewer: PlayerId }` — a zero-copy, stateless read facade over the
full-information engine, living in the engine crate beside the layer view.

Visibility predicate (positional + look_grants): a viewer sees an object's
identity iff it is in a public zone ([CR#400.2]), OR the viewer owns/controls
the hidden object, OR `(viewer, object)` is in the engine's `look_grants`
ledger ([CR#406.3]) — the continuing-look permission that peeks/reveals write
and shuffle/remint clears ([CR#701.20]). Redacting reads collapse
hidden-from-viewer content to cardinality (opponent hand → count, library
order → unknown beyond granted top-N).

Characteristic axis (face-down): a `face(id)` chokepoint distinct from the zone
predicate returns the known face or `hidden` for a face-down object the viewer
neither controls nor has a grant for. This is why the build depends on
`engine-face-down` — the face-down object representation is what makes
characteristic redaction real and testable.

Migration when built: route the demo `Strategy` and the TUI's hidden-zone /
printed-face reads through the view; add `can_see` / `face` tests (own vs
opponent hand, public zones, granted scry top then cleared by shuffle,
reveal-to-all, face-down controller vs opponent vs grantee). Core stays
full-information throughout — no change to RNG, zone storage, or the
full-info model.

Early-build override: if a consumer needs genuinely non-cheating play (an
adversarial self-play sim or a networked/server runner) before face-down lands,
the zone/`can_see` half can be built ahead of the `face()` half.
