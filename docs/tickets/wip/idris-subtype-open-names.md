---
needs: []
---
**Idris grammar: open subtype *names* and source subtype conferrals from the RON, so the
closed `Subtype` enum and its `idris_emit` mirror stop needing parity with the macro layer.**
A subtype name is currently spelled in three places that must be kept in lockstep, and its
conferred abilities are hand-mirrored in Idris — both duplicate what the open macro/`Subtype`
layer already carries. Every new subtype forces hand-edits or the card fails the
`idris-check` gate.

**Current shape (the triplication + the conferral mirror).**

1. `idris/src/Core.idr` (~124-181) models subtypes as five *closed* per-category enums
   (`CreatureSubtype = Bear | … | Dwarf`, `EnchantmentSubtype = Aura | Saga`, `ArtifactSubtype`,
   `LandSubtype`, `BattleSubtype`) wrapped in `data Subtype = CreatureSub … | EnchantmentSub … | …`,
   plus five `Promote` instances and a total `subtypeCategory : Subtype -> Type_`.
2. `crates/deckmaste_cards/src/idris_emit.rs` (~233-289) holds `fn subtype_idris` — a
   hand-mirrored `match` from each name to its Idris constructor (`"Bear" => "(CreatureSub Bear)"`,
   …), a verbatim duplicate of the Core.idr enum. An unmapped name returns
   `gap("unmapped subtype: …")`, so the card silently fails to emit and the
   `cargo xtask idris-check` gate rejects it. Same match feeds the
   `CharacteristicPredicate::Subtype` (~505) and `Modification::Subtypes` (~1552) emit paths.
3. `crates/deckmaste_core/src/type.rs` — the Rust `Subtype` is an *open* struct
   (`name: Ident`, `types: Vec<Type>`, `confers: Vec<Property>`); one macro RON file per subtype
   under `plugins/*/macros/types/**/*.ron`, auto-generatable from Scryfall catalogs. This layer
   scales to the full MTG subtype set with zero manual parity. The Rust **engine already sources
   conferrals from this data** (`deckmaste_engine/src/derive.rs`: a face's abilities = printed
   `++ subtypes[].confers`).
4. Idris `subtypeConfers : Subtype -> List (Ability b)` (~2776) **hand-mirrors** the RON confers
   for the two conferring subtypes (`EnchantmentSub Aura → Static (Sba …)` falls-off SBA,
   `EnchantmentSub Saga → TurnBased …` lore increment). It is called from nothing but a spec
   witness (`Spec.idr:401`) — never wired into card evaluation — and its arms are exactly what
   the RON `Property` flavors already produce. A second parity mirror.

So adding one subtype means editing the closed Idris enum **and** the Rust match; and any
conferring subtype must be hand-mirrored in `subtypeConfers`. The closed enum buys nothing but
this burden.

**Fix — open the names, put conferrals on the value, source both from the RON.**

Idris `Subtype` becomes an open, self-describing value (matching the `type.rs` doctrine that
"a macro-expanded card describes the entirety of its behavior"). Idris has no `Property` type —
"a conferral IS an ability" — so confers are a plain ability list on the value:

```idris
namespace Category
  data Category = Creature | Enchantment | Artifact | Land | Battle | Planeswalker | Spell
data Subtype = MkSubtype Category String (List (Ability Base))
```

The first field is a **`Category`, not a `Type_`**: a *creature* subtype is valid on both
`Creature` and `Kindred` cards and a *spell* subtype on both `Instant` and `Sorcery`, so the
category groups the card types a subtype may appear on rather than naming one. Names are open
strings; conferred abilities ride the value.

Consequences:

- `categoryTypes : Category -> List Type_` — `Creature → [Creature, Kindred]`,
  `Spell → [Instant, Sorcery]`, the rest singleton. `subtypeCategory (MkSubtype cat _ _) = cat`.
- `SubtypesOk` (the card well-formedness proof, ~2738) changes from `Elem (subtypeCategory s)
  (types c)` to **each subtype's category *intersects* the card's `types`** (the card has ≥1 of
  `categoryTypes (subtypeCategory s)`). Must stay auto-solvable so emitted cards need no explicit
  proof.
- `subtypeConfers` collapses to the field projection `subtypeConfers (MkSubtype _ _ cs) = cs`.
  The old hardcoded Aura/Saga arms are **not deleted** — their conferral data is preserved,
  relocated into the hand-authoring helpers `aura`/`saga` (curated Cards.idr sugar), so nothing
  learned is lost while the live/emit source of truth moves to the RON.
- The `Promote` per-category instances collapse into category helper constructors
  `creature`/`enchantment`/`artifact`/`land`/`battle`/`planeswalker`/`spell : String -> Subtype`
  (empty confers) plus `aura`/`saga : Subtype` (confers baked). Hand-written `idris/src/Cards.idr`
  (and `Spec.idr`/`Macros.idr` sites) move from `^Bear`/`^Saga` to `creature "Bear"`/`saga`.
- **Structural:** because `Subtype` now references `Ability Base`, it joins the
  `Subtype ↔ Ability ↔ Characteristics` cycle; the compiler forces the grammar core (the three
  `mutual` blocks + interstitials, ~lines 1071–2636) to **fuse into one `mutual` block**. Proven
  to typecheck with identical guarantees; accepted.
- `idris_emit`: **delete `subtype_idris` entirely.** Emit `MkSubtype <Category> "<name>"
  [<confers>]`, deriving `Category` from the open Rust `Subtype.types` (Creature/Kindred →
  `Creature`, Instant/Sorcery → `Spell`, Land → `Land`, …) and emitting the confers from the Rust
  `Subtype.confers` (`Property::StateBased → Static (Sba …)`, `TurnBased → TurnBased …`,
  `Continuous → Static (Modify …)`, `Ability → Innate …`). Any subtype the macro layer names now
  emits and typechecks; the only remaining `gap` is a `Subtype` whose `types` map to no category
  (a real modeling error). Fold in the `idris-naming-residuals` item 6 `Promote`-order residual
  (obsoleted by the collapse).

**Verify.** `cargo xtask idris-check` (re-emit + `idris2 --check`) green; `idris/` builds
(`mtg.ipkg`). Prove parity is gone: a subtype present in the macro corpus but absent from the
*old* closed enum now emits and typechecks end-to-end, and a card with a conferring subtype
(Aura/Saga/basic land) emits its conferred abilities from the RON. Workspace green (fmt +
clippy + tests); run the cite audit if any CR citation moves.

*Serializes with the other `idris-*` grammar tickets — they all rewrite `idris/src/Core.idr`,
so only one can be in flight at a time. `needs:` is empty because the blocking is file-level,
not logical precedence.*
