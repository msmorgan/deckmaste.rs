---
needs: []
---
Vibranium predefined token [CR#111.10w] — new in the June 19 2026 CR: "A Vibranium
token is a colorless Vibranium artifact token with indestructible and '{T}: Add
{C}. This mana can't be spent to cast a nonartifact spell.'" ~6 cards (MSH) create
Vibranium tokens.

The predefined-token machinery already landed (`engine-tokens`, done), so this is
a token *definition* plus one catalog gap — twin of the existing predefined tokens
(Treasure/Food/Clue), which are colorless artifact tokens with a built-in tap
ability.

Work:
- Add the Vibranium token to the predefined-token table the way Treasure/Food/Clue
  are defined: colorless, artifact, subtype Vibranium, keyword Indestructible, and
  the restricted mana ability "{T}: Add {C}. This mana can't be spent to cast a
  nonartifact spell." (the same restricted-mana shape Treasure/Powerstone use, but
  colorless + artifact-only).

Catalog gap — **Vibranium is a new artifact SUBTYPE** [CR#205.3g] but is NOT in
Scryfall's `artifact-types` catalog yet (the other new subtypes — Plan, and the
Marvel creature types Eternal/Gamma/Inhuman/Kree/Shi'ar/Skrull/Spy — ARE in the
refreshed catalogs, so they flow through `SUBTYPES`
(`crates/deckmaste_migrations/src/parsers/filter.rs`) automatically). Until
Scryfall adds it, the adjective "Vibranium" and the token's type line will decline
as an unknown subtype.
- Supplement `SUBTYPES` (or the artifact-types catalog load) with Vibranium so the
  subtype resolves. Re-check on the next `scripts/fetch_data`: once Scryfall lists
  it, the supplement can be dropped.

## Done
- Cards that create a Vibranium token graduate and emit the predefined token; the
  Vibranium artifact subtype resolves. `idris-check` count unchanged; `cargo test
  --workspace` green.
