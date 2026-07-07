---
needs: []
---
**Idris grammar: open subtype *names* so the closed `Subtype` enum and its `idris_emit`
mirror stop needing parity with the macro layer.** A subtype name is currently spelled in
three places that must be kept in lockstep; only the macro layer is open, so every new
subtype forces two hand-edits or the card fails the `idris-check` gate.

**Current shape (the triplication).**

1. `idris/src/Core.idr` (~124-181) models subtypes as five *closed* per-category enums
   (`CreatureSubtype = Bear | Rat | … | Dwarf`, `EnchantmentSubtype = Aura | Saga`,
   `ArtifactSubtype`, `LandSubtype`, `BattleSubtype`) wrapped in
   `data Subtype = CreatureSub … | EnchantmentSub … | …`, plus five `Promote` instances
   and a total `subtypeCategory : Subtype -> Type_`.
2. `crates/deckmaste_cards/src/idris_emit.rs` (~233-289) holds `fn subtype_idris` — a
   hand-mirrored `match` from each name to its Idris constructor application
   (`"Bear" => "(CreatureSub Bear)"`, …). It is a verbatim duplicate of the Core.idr enum;
   an unmapped name returns `gap("unmapped subtype: …")` so the card silently fails to
   emit and the `cargo xtask idris-check` gate rejects it. Same match feeds the
   `CharacteristicPredicate::Subtype` (~505) and `Modification::Subtypes` (~1552) emit paths.
3. `plugins/*/macros/types/**/*.ron` — one open RON file per subtype, auto-generatable from
   Scryfall catalogs (`crates/deckmaste_migrations/src/stubs/subtypes.rs`). The Rust
   `deckmaste_core::Subtype` is an *open* struct (`name: Ident`, `types: Vec<Type>`,
   `confers: Vec<Property>`). This layer already scales to the full MTG subtype set with
   zero manual parity.

So adding one creature subtype means editing (1) the closed Idris enum **and** (2) the Rust
match — steps that duplicate what the open macro/`Subtype`-struct layer already carries. The
Idris grammar does **no** per-subtype pattern-matching (subtypes are identity tags; conferral
lives on the Rust data), so the closed enum buys nothing but the parity burden.

**Fix — open the names, keep the category closed.** Replace the five per-category enums and
the `Subtype` sum with:

```idris
data Category = Creature | Enchantment | Artifact | Land | Battle | Planeswalker | Spell
data Subtype  = MkSubtype Category String
```

`Category` is the genuinely-closed, small set of card-type categories that own subtypes;
subtype *names* become open strings. Consequences:

- `subtypeCategory : Subtype -> Type_` stays total — map the stored `Category` to its
  `Type_`. The `Promote` instances (per-category → `Subtype`) collapse into per-category
  constructor helpers (`creature`, `enchantment`, `artifact`, `land`, `battle`,
  `planeswalker`, `spell` : `String -> Subtype`) so hand-written `idris/src/Cards.idr` reads
  `subtypes := [creature "Bear"]` / `[creature "Merfolk", creature "Wizard"]` / `[aura]` in
  place of today's `^Bear` promote sigil. Update every subtype site in `Cards.idr`.
- `idris_emit`: **delete `subtype_idris` entirely.** Derive the `Category` from the open Rust
  `Subtype.types` (`Type::Creature`/`Kindred → Creature`, `Land → Land`, etc.) and emit
  `(MkSubtype Creature "Griffin")`. Any subtype the macro layer can name now typechecks;
  there is no closed list to fall off. A `Subtype` whose `types` map to no `Category` is the
  only remaining `gap` (a real modeling error, not a coverage gap).
- Fold `subtypeCategory` ordering / the `Promote`-order residual noted in
  `idris-naming-residuals` item 6 into this rewrite (it's obsoleted by the collapse).

**Verify.** `cargo xtask idris-check` (the re-emit + `idris2 --check` gate) must stay green,
and a subtype present in the macro corpus but absent from the *old* closed enum (pick one,
e.g. a creature type with a macro file but no Core.idr constructor) must now emit and
typecheck end-to-end — demonstrating parity is gone. `idris/` must build; workspace green
(fmt + clippy + tests); if any `[CR#…]` citations move, run the cite audit.

*Serializes with the other `idris-*` grammar tickets — they all rewrite `idris/src/Core.idr`,
so only one can be in flight at a time. `needs:` is empty because the blocking is file-level,
not logical precedence.*
