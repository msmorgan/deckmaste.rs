---
needs: []
---
Retire the `Sideboard`-as-a-zone model and give an object's **position** a type
that can name the out-of-game store, since a zone can't. Introduce a `Location`
that wraps a `Zone` or is `OutsideTheGame`:

```rust
enum Location {
    #[macro_ron(flatten)]
    Zone(Zone),
    OutsideTheGame,
}
```

`Zone` is the **in-game subset** of `Location`. Cards are objects ([CR#109.1]),
and an object can sit outside the game with no zone at all ([CR#400.11]) — so a
position that may be out-of-game is a `Location`; a position the context
*guarantees* is in-game stays a `Zone`. `#[macro_ron(flatten)]` lifts the zone
variant names into `Location`'s dispatch exactly as `Destination::Zone` does
today (`crates/deckmaste_core/src/action.rs`), so a bare `Hand` still
reads/renders as `Hand`, never `Zone(Hand)`.

## Why

The CR is explicit that outside-the-game is **not a zone**, so a `Sideboard`
zone constructor is a rules error:

- [CR#400.1] "There are normally seven zones: library, hand, battlefield,
  graveyard, stack, exile, and command." No sideboard among them.
- [CR#400.11] "An object is outside the game if it isn't in any of the game's
  zones. **Outside the game is not a zone.**"
- [CR#400.11a] "Cards in a player's sideboard are outside the game." — the
  sideboard is deck-construction membership ([CR#100.4]), not a place objects
  sit *during* a game.
- [CR#400.11c] cards outside the game can't be affected by spells/abilities
  except their own CDAs and effects that bring them into the game — a
  restriction that is a property of `OutsideTheGame`, distinct from any zone's
  visibility default.

The concrete driver is the family of effects that reach out-of-game cards:
Wishes (Burning/Living Wish), Companion, Learn/Lesson, and Venture/Dungeon
([CR#309.2] dungeons begin outside the game). They fetch/reveal a **card**
that has no `Zone` — it lives in a `Location::OutsideTheGame`. Modeling that
today would force a fake `Zone::Sideboard`, which is exactly what this ticket
removes.

## Scope

**Retire the Sideboard zone (idris side — Rust `core::Zone` already omits it):**
- `idris/src/Semantics.idr` — drop the `Sideboard` `Zone` constructor and its
  comment (currently the only place asserting sideboard-is-a-zone).
- `idris/src/Semantics.idr` `zoneSort` — drop the `Sideboard` arm.
- `idris/src/Spec.idr` `tZoneSorts` — drop `Sideboard` from the zone list.
- `idris/src/EmitTables.idr` `zoneName` — drop the `Sideboard` arm.
- Mirror the new `Location` sum in idris (nullary `OutsideTheGame`, transparent
  `Zone` arm) so the soundness gate covers it.

**Add `Location` (Rust `core`):**
- New type beside `Zone`, flatten-lifting the zone names as above.
- Verify parse⇄render round-trips (bare zone name ↔ `Zone` arm; `OutsideTheGame`
  as its own token), matching the `Destination` flatten tests in `action.rs`.

**Substitution scoping — the rule for the sweep (cards *are* objects
[CR#109.1], so the axis is in-game vs out-of-game, not card vs object):**
- An object's **position field** that must be able to name the out-of-game
  store → `Location`. This is the widening of the object model: a card in a
  sideboard, or a dungeon before it enters ([CR#309.2]), is an object with no
  zone. Whether `Object`'s stored position (`Option<Zone>` today) becomes
  `Location` is the central call this ticket makes — decide it explicitly
  rather than by mechanical find-replace. (`None` today means "no zone yet";
  `Location::OutsideTheGame` gives that state an actual name.)
- A position the context **guarantees is in-game** stays `Zone`. A `Move`
  destination is the clearest case: you cannot zone-change an object to
  `OutsideTheGame` — leaving the game is a distinct event ([CR#400.11b]), not a
  `Move`. `Destination` (which already excludes non-move zones) stays
  `Zone`-based; battlefield/graveyard/hand/priority/SBA APIs likewise.
- Net: `Location` appears at the *boundary* where out-of-game cards enter the
  model (fetch sources, sideboard membership, reveal-from-outside); the in-game
  hot path keeps `Zone` and is unchanged.

## Out of scope (future, do not gate on this)

Deck construction proper. When it lands:
- `Deck::Mainboard` / `Deck::Sideboard` model the 60+15 membership
  ([CR#100.2a,100.4a]) — the sideboard is a *deck partition*, not a `Location`
  and not a `Zone`.
- The card-scoped **Commander** designation is handled by commander-first
  decklist ordering plus an authored `Commander.ron` rules-grant (or similar),
  not a new zone. (Note the command zone *is* a real zone, [CR#408]; this is
  about the deck-side designation, not where the commander sits in play.)

These are named here only so the `Location` refactor stays independent of them
and claimable on its own.

## Done when

- No `Sideboard` `Zone` constructor anywhere (Rust or idris); idris gate green.
- `Location` exists in `core` with round-trip tests; the flatten spelling keeps
  bare zone names unchanged.
- The in-game-vs-out-of-game scoping rule above is applied: object positions
  that can be out-of-game use `Location`; guaranteed-in-game APIs keep `Zone`.
- Standard constraints apply (fmt, clippy, CR citations blessed, wizards regen
  if touched).
