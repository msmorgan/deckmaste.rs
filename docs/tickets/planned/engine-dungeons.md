---
needs: []
---
Dungeon objects outside the game, venture progression through rooms, and the
completion trigger (76 cards). Dungeons are not on the battlefield; room
transitions fire triggered abilities.

## Current state (2026-07-18 deep-dive)

The mechanic-specific action variant has been retired. A future venture
builtin macro will expand to `Action::Composite` over the dungeon subsystem;
there is no parse/render arm or Idris spec yet, and zero live corpus cards
(candidates sit in `wizards.bak`). `Type::Dungeon` + the `Dungeon.ron` TypeDef
macro exist.

## Shape

The verb body is expressible as a `Composite` over existing primitives once
the state lands: "if you own no dungeon card in the command zone, choose one
you own from outside the game, put it there, marker on topmost room, trigger
it [CR#701.49a]; else move the marker along an arrow to an adjacent room and
trigger it [CR#701.49b]; leaving the bottommost room removes the dungeon from
the game, completing it [CR#701.49c,309.7]." Rooms are per-card data: a
dungeon card is an ordered room graph, each room an intrinsic trigger "when
you move your venture marker into this room" [CR#309.4c].

## Missing engine primitives (all four required)

1. Per-player venture-marker position state — nothing exists.
2. A dungeon-card pool: "outside the game" source + command-zone mint
   (emblems live there, but no dungeon lifecycle; [CR#309.2d] even forbids
   other effects putting dungeon cards there).
3. Room-ability trigger sourcing wired to marker movement.
4. Completed-dungeon history for "if you've completed a dungeon" conditionals
   (Acererak-family).

Add an idris constructor alongside, or record the asymmetry in
`idris-mirror-enum-gaps` — the `Type::Dungeon` entry there resolves with this
ticket either way.

Note (2026-09-07): the "future venture builtin macro" is a v2 declaration under `plugins_v2/builtin/macros/stubs/keyword_actions/`, routed by `semantics-v2-keyword-action-residues`; the engine primitives above are unchanged.
