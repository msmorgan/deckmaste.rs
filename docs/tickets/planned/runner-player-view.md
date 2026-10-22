---
needs: [engine-face-down, engine-look-grants-lifecycle]
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

## Correction (2026-10-21): visibility is keyed per zone

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #22.
The predicate above is wrong in two places, and the build follows this section,
not the paragraphs above.

- "OR the viewer owns/controls the hidden object" matches every card in the
  viewer's own library, because off the battlefield the controller is the owner
  ([CR#108.4a]; `crates/deckmaste_engine/src/step/mod.rs:590-594`). [CR#401.2]:
  "Players can't look at or change the order of cards in a library."
- `face(id)` hiding a face-down object "the viewer neither controls nor has a grant
  for" would show an owner a card an opponent exiled face down (Gonti ruling: "No
  player may look at it"). [CR#406.3]: face-down exiled cards "can't be examined by
  any player except when instructions allow it"; [CR#708.5]: "You can't look at
  face-down cards in any other zone or face-down spells or permanents controlled by
  another player."
- "shuffle/remint clears" grants is false at head: a shuffle does not remint
  (`engine-look-grants-lifecycle` fixes the engine side).

Key visibility per zone instead:

- **Hand:** visible to its player ([CR#402.3]) and to teammates where the variant
  lets them review hands ([CR#808.5,809.7,810.5]; [CR#811.5] only for teammates
  sitting next to each other), minus a card drawn face down mid-cast ([CR#121.8]).
- **Library:** visible to no one, except through a live effect or an open look
  window ([CR#401.5,701.20e,701.23a]).
- **Face axis:** consult the controller only for stack and battlefield objects
  ([CR#708.5]); a face-down exiled card is visible only under the [CR#406.3] ledger.

Add tests: an owner can't see their own library; an owner can't see a card an
opponent exiled face down; the owner of a face-down permanent an opponent controls
can't see its face. The "granted scry top then cleared by shuffle" test stays and
needs `engine-look-grants-lifecycle`.
