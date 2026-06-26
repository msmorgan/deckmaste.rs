---
needs: []
---
**Idris grammar: close three mana expressivity gaps and merge the two
mana-adding actions.** From the idris↔rust structure audit (2026-06-26); the Rust
model (`crates/deckmaste_core/src/mana.rs`) already has the shapes to adopt.

In `idris/src/Core.idr`:

1. **Hybrid Phyrexian.** `Phyrexian Color` is single-color, so `{G/U/P}` (Tamiyo,
   Compleated Sage) is unrepresentable. → `Phyrexian Color (Maybe Color)`
   ([CR#107.4f]: a hybrid Phyrexian symbol is both component colors). The `None`
   case is plain `{C/P}`-style single-color.

2. **Fixed-set produced mana.** `ProducedMana = OfColor (Maybe Color) | AnyColor`,
   and `AddMana` takes a *list* meaning "add all of these," so "add one mana of
   {W} or {U}" (a choice) has no encoding. → add `OneOf : List (Maybe Color) ->
   ProducedMana`.

3. **Per-unit mana riders.** Restrictions hang on the whole `AddMana` action
   (`onlyToCast`/`confers`), uniform across the list, and there is no
   trigger-on-spend or snow-provenance rider. [CR#106.6a]: under a mana doubler
   each produced mana gets its *own* restriction/trigger. → carry produced mana as
   `(ProducedMana, List ManaRider)` units with a real `ManaRider` sum
   (spend-only / grant-on-spend / trigger-on-spend / snow / persistent).

4. **Merge `AddMana` + `AddManaFor`.** They are two encodings of one verb:
   `AddMana` is a fixed list with riders; `AddManaFor` is a dynamic `Count` × one
   `ProducedMana` but **drops the riders** — so "X mana, only to cast creatures"
   can be neither dynamic-and-restricted. → one
   `AddMana : Count -> <ProducedMana-with-riders> -> Action b` (Rust's shape:
   `PlayerAction::AddMana(Count, ManaProduction)` with `ManaProduction::WithRiders`).

This is data-model only; the Idris probe runs no games. Keep the closed mana enums
otherwise.
