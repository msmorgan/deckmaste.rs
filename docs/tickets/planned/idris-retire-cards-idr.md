---
needs: []
---
**Retire `idris/src/Cards.idr` by migrating its coverage into a RON acceptance
corpus.** Card correctness is now gated by re-emitting each Rust/RON card to
`Core.idr` and typechecking (`cargo xtask idris-check`); the hand-authored
Idris card list is a typecheck-only leaf superseded by that path. 2026-07-19
deep-dive.

## Why it's safe, and the one catch

`Cards.idr` is a **leaf**: nothing imports it, no Rust/test/xtask reads it, the
re-emit gate never pulls it in (`idris_check` probes import `Core` only), and CI
never compiles Idris. Retiring is mechanically just: delete the file + drop
`Cards` from the `idris/mtg.ipkg` modules list (+ two stale `--` comments at
`Spec.idr:684`, `Core.idr:2315`, and `VERIFY.md` prose).

The catch: those 71 hand-authored cards are the **only** thing exercising
certain `Core.idr` constructor arms — because the arm's real card isn't in
canon yet. `Cards.idr` reached those arms by writing Idris *directly*, bypassing
the Rust→Idris bridge. So a straight delete drops model-arm coverage that the
re-emit path cannot replace until each card is authored on the Rust side. The
fix is to migrate, not just delete.

## Card inventory (71 modeled, 11 already canon)

Of the 60 not-yet-canon Idris-modeled cards:

- **38 CANONIZABLE-NOW** — grammar arm exists AND emit is clean. Authoring them
  Rust-side recovers their Core.idr-arm coverage through the enforced bridge.
- **22 BLOCKED** — the Rust side cannot reach the arm today (so their coverage
  is *already* unreachable via re-emit regardless of this ticket):
  - **13 lack a Rust card-grammar arm** — Time Walk (extra-turn verb), Mindslaver
    (control-a-*player*), Vodalian Illusionist (phase-out verb), Pine Walker
    (morph/turn-face-up), Iona / Steely Resolve / Cavern of Souls / Meddling Mage
    / Citadel Siege / Outpost Siege (as-enters chosen-value / mode), Mind Bend
    (word-class text-change), Furnace of Rath + Doubling Season (`ReplaceAmount` /
    event-amount read). Each is its mechanic's open ticket
    (`engine-turn-modification`, `engine-control-another-player`,
    `engine-phasing`, `engine-face-down`, `core-as-enters-choices`,
    `engine-restricted-mana`).
  - **9 have grammar but `idris_emit` GAPS** — Wear//Tear, Brazen Borrower,
    Delver (`Card::TwoFaced` unmapped, `idris_emit.rs:3608` — one arm unblocks
    all three, the cheapest win), Banishing Light + Drudge Skeletons/regen
    (`Action::CreateReplacement`, `:1759`), Approach of the Second Sun
    (`Count::EventCount`, `:1125`), Notion Thief (`EventFilter::Nth`, `:3039`),
    Oblivion Ring (`Reference::Linked`, `:572`), History of Benalia
    (subtype-confer `Property::TurnBased`, `:335`).

## Do (this ticket)

1. **Stand up a non-strict acceptance corpus.** House migrated cards in a
   fidelity-`strict=false` plugin (`plugins/testing/cards/`, or a dedicated
   `plugins/acceptance/`). Non-strict = no oracle entry required, render diffs
   waived — so the artifact needs only to **parse + lint + emit-typecheck**, the
   exact role `Cards.idr` filled, now routed through the real Rust→Idris bridge.
   Wire `cargo xtask idris-check <plugin>` over it (local gate; CI has no idris2).
2. **Migrate the 38 CANONIZABLE cards** to that corpus as RON. Each must parse,
   lint clean, and pass `idris-check`. The 4 complex-path cards (Liliana
   loyalty-cost emit, Midnight Haunting / Smuggler's Copter player-action actor
   slot, Snapcaster cost-of-reference) may trip an emit gap on authoring — if so,
   demote them to BLOCKED and file the gap.
3. **Retire `Cards.idr`** once the 38 (+ the 11 already canon) are migrated:
   delete `idris/src/Cards.idr`, drop `Cards` from `idris/mtg.ipkg`, fix the two
   stale comments and `VERIFY.md`. The 22 BLOCKED defs either stay as a
   `Cards.idr` remnant until unblocked, or drop with their intent preserved in
   the mechanic tickets above (decide at retire time — prefer dropping, since the
   arms are unreachable Rust-side anyway and the tickets carry the intent).

## Follow-on (separate tickets, not this one)

- **Emit-bridge arms** for the 9 gap cards — mint per arm; `Card::TwoFaced` first
  (recovers 3 cards). These belong with `idris-mirror-enum-gaps` (Rust→Idris
  emit asymmetries) or their own bridge tickets.
- **Renderer arms** for real-canon promotion. Only ~20 of the 38 render back to
  oracle cleanly today; ~14 need a render arm or waiver (vs the historical ~9%
  waiver rate over 78 canon cards). The gaps are concentrated and real:
  `StaticEffect::CostModifier` has **no render arm at all** (Goblin
  Electromancer / Thalia / Frogmite), modal renders only "Choose one" (no
  "Choose two" — Cryptic Command), plus kicker, leveler bands, and the
  produced-object / loyalty / CDA / control-shift / complex-anthem seams that
  already account for the existing 7 waivers. File these as renderer tickets
  (`CostModifier` first) and promote acceptance-corpus cards into fidelity-gated
  canon as their render arm lands. **Real-canon promotion is not required for the
  coverage goal** — the acceptance corpus reaches every emit arm without it.

## Donate — the sole true orphan

**Donate** (`{2}{U}` Sorcery — "Target player gains control of target permanent
you control.") is the one not-yet-canon card whose design intent had **no ticket
anywhere**; it lives only at `Cards.idr:389`. Migrate it in step 2 (it is
CANONIZABLE). Two intents it is the worked example for:

- **Mixed-kind multi-target** — slot 0 a player (`APlayer`), slot 1 an object
  ("permanent you control"); the two sorted anaphors disambiguate by noun, no
  labels.
- **Rest-of-game continuous control shift** — continuous `Modify(Target 1,
  GainControl(Target 0))` with no stated duration lasts until end of game
  [CR#611.2a]; control-changing applies in layer 2 [CR#613.1b]; the affected
  object is fixed at resolution [CR#611.2c]. Routes through
  `Modification::SetController` (`idris_emit.rs:2149`), which already
  round-trips. (The *one-shot* `Action::GainControl` at `action.rs:236` is a
  separate `idris_emit` gap at `:1738` — a decision for `idris-mirror-enum-gaps`,
  not needed by Donate.)

## Converge Draw as part of the retire

Retiring `Cards.idr` dissolves the standing justification for the **Draw
divergence** (idris `Action.Draw` was kept only because retiring it "forces
migrating ~20 Cards.idr sites"). Per the mirror policy (user ruling 2026-07-19:
prefer mirror over sanctioned divergence — no more Draw-style asymmetries "if we
can help it"), with those sites gone the plan is to **converge**: retire idris
`Action.Draw` to Rust's Composite-only form as part of this cleanup, not to
re-sanction it. The `{default …}` grammar-arg sites are the same shape — fold
their removal in or spin a sibling cleanup. Do NOT retire idris `Action.Draw`
before the `Cards.idr` sites are actually gone (it still has ~20 callers).

Standard constraints apply. Deltas: `idris-check` is a local-only gate (no idris2
in CI); the acceptance plugin must be registered in the fidelity `COVERED` list
as non-strict; retire step must update `idris/mtg.ipkg` or the Idris build breaks.
