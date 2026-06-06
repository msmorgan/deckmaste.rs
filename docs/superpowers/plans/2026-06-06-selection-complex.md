# Selection Complex Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Land Filter/Reference/Selection (plus a minimal Zone) in `deckmaste_core`, replacing the toy `Target`/`Selector`, and re-encode Lightning Bolt against them — Plan 1 of the spec at `docs/superpowers/specs/2026-06-06-core-building-blocks-design.md` (§1, part of §10).

**Architecture:** Filter is a compartmentalized Rust enum (`Characteristic`/`State`/`Relation` sub-enums) that reads and writes *flat* RON (`Type(Creature)`, never `Characteristic(Type(Creature))`) via a small manual serde impl — `#[serde(untagged)]` variants would route through `deserialize_any` and bypass the macro layer's `EnumIntercept`, so manual dispatch over one combined variant list is load-bearing, not style. Selection and Reference are plain derives. `MacroKind` drops `Target` and gains `Filter`/`Reference`/`Selection`.

**Tech Stack:** Rust (workspace crates `deckmaste_core`, `deckmaste_cards`), serde + ron 0.12 (options in `deckmaste_core::ron`), jj for VCS.

**VCS precondition:** This is a jj repo and `jj commit` snapshots the whole working copy. Before Task 1, start a fresh change on top of the in-flight work (`jj new`) and confirm `jj st` shows no unexpected edits, so each plan commit contains only plan changes.

**Deferred (YAGNI, additive later):** Quantity-bearing Selection variants (`Targets(n, …)`, `UpToTargets`, `Random`, `Superlative`), `These([Reference])`, Reference's `That`/`Bound`/`Linked` (arrive with Events), prelude macros beyond AnyTarget (`YouControl`, `IsOpponent`, `Historic` arrive with their first consumer), `ZoneRef` positions/facing. Adding enum variants later is non-breaking.

---

## File structure

| File | Responsibility |
|---|---|
| Create `crates/deckmaste_core/src/zone.rs` | `Zone` — the seven zones, plain derive |
| Create `crates/deckmaste_core/src/reference.rs` | `Reference` — bound variables |
| Create `crates/deckmaste_core/src/filter.rs` | `ObjectKind`, `CharacteristicFilter`, `StateFilter`, `RelationFilter`, `Filter` (manual serde) |
| Create `crates/deckmaste_core/src/selection.rs` | `Selection` — Filter lifted into a choice context |
| Modify `crates/deckmaste_core/src/lib.rs` | module decls + re-exports; drop `Target`, `Selector` |
| Modify `crates/deckmaste_core/src/ability.rs` | delete `Target`/`Selector`; `SpellAbility.targets: Vec<Selection>`; `Effect::DealDamage(Selection, Uint)` |
| Modify `crates/deckmaste_cards/src/macros.rs` | `MacroKind`: −Target, +Filter/Reference/Selection; tests |
| Move `plugins/builtin/macros/target/AnyTarget.ron` → `plugins/builtin/macros/filter/AnyTarget.ron` | kind `[Filter]`, context-free body |
| Modify `plugins/builtin/macros/example/Self.ron` | kinds `[Subtype, Target]` → `[Subtype, Filter]` |
| Modify `plugins/builtin/cards/Lightning Bolt.ron` | new encoding |
| Modify `crates/deckmaste_cards/tests/builtin.rs` | new Lightning Bolt expectation |

Naming note: `CharacteristicFilter::Subtype` takes an `Ident` (the subtype *name*), not the `Subtype` struct — filtering needs the name only, and a struct payload would sit in the `SkipStructs` intercept gap where bare declared names (`Forest`) can't expand. RON: `Subtype("Forest")`.

---

### Task 1: Zone

**Files:**
- Create: `crates/deckmaste_core/src/zone.rs`
- Modify: `crates/deckmaste_core/src/lib.rs`

- [ ] **Step 1: Write the failing test**

Create `crates/deckmaste_core/src/zone.rs`:

```rust
use serde::{Deserialize, Serialize};

/// A game zone (CR 400.1). Vintage-legal scope: no ante.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Zone {
    Battlefield,
    Command,
    Exile,
    Graveyard,
    Hand,
    Library,
    Stack,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zones_round_trip() {
        let options = crate::ron::options();
        for zone in [
            Zone::Battlefield,
            Zone::Command,
            Zone::Exile,
            Zone::Graveyard,
            Zone::Hand,
            Zone::Library,
            Zone::Stack,
        ] {
            let written = options.to_string(&zone).unwrap();
            let parsed: Zone = options.from_str(&written).unwrap();
            assert_eq!(parsed, zone);
        }
    }
}
```

In `crates/deckmaste_core/src/lib.rs`, after the `mod symbol;` block add:

```rust
mod zone;
pub use zone::Zone;
```

- [ ] **Step 2: Run the test**

Run: `cargo test -p deckmaste_core zone`
Expected: PASS (type + test land together; the "failing" state here is the pre-edit compile)

- [ ] **Step 3: Commit**

```bash
jj commit -m "core: Zone enum (CR 400.1, no ante)"
```

---

### Task 2: Reference

**Files:**
- Create: `crates/deckmaste_core/src/reference.rs`
- Modify: `crates/deckmaste_core/src/lib.rs`

- [ ] **Step 1: Write the type and tests**

Create `crates/deckmaste_core/src/reference.rs`:

```rust
use serde::{Deserialize, Serialize};

/// A bound variable: a value fixed earlier (at announce, by the rules of
/// the position, or by a binder) and referenced later. References name
/// *objects*; amounts live in Quantity (future module).
///
/// Players are objects — `You`, `ControllerOf`, `OwnerOf` resolve to
/// player objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Reference {
    /// The object this ability is printed on / the resolving spell.
    This,
    /// The controller of this ability (CR 109.5).
    You,
    /// The nth target this ability announced (CR 115.3, 601.2c).
    Target(usize),
    ControllerOf(Box<Reference>),
    OwnerOf(Box<Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn references_round_trip() {
        let options = crate::ron::options();
        for reference in [
            Reference::This,
            Reference::You,
            Reference::Target(1),
            Reference::ControllerOf(Box::new(Reference::Target(0))),
        ] {
            let written = options.to_string(&reference).unwrap();
            let parsed: Reference = options.from_str(&written).unwrap();
            assert_eq!(parsed, reference);
        }
    }

    #[test]
    fn target_index_reads_bare() {
        let parsed: Reference = crate::ron::options().from_str("Target(0)").unwrap();
        assert_eq!(parsed, Reference::Target(0));
    }
}
```

In `crates/deckmaste_core/src/lib.rs`, after the `mod plugin;`-adjacent exports add (alphabetical with the others):

```rust
mod reference;
pub use reference::Reference;
```

- [ ] **Step 2: Run the tests**

Run: `cargo test -p deckmaste_core reference`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
jj commit -m "core: Reference enum (bound variables)"
```

---

### Task 3: Filter — compartments in Rust, flat in RON

**Files:**
- Create: `crates/deckmaste_core/src/filter.rs`
- Modify: `crates/deckmaste_core/src/lib.rs`

- [ ] **Step 1: Write the failing tests first**

Create `crates/deckmaste_core/src/filter.rs` with ONLY the test module (types come in step 3, so the failure mode is honest):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Supertype, Type, Zone};

    fn read(source: &str) -> Filter {
        crate::ron::options().from_str(source).unwrap()
    }

    #[test]
    fn atoms_read_flat() {
        assert_eq!(
            read("Type(Creature)"),
            Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
        );
        assert_eq!(
            read(r#"Subtype("Forest")"#),
            Filter::Characteristic(CharacteristicFilter::Subtype("Forest".into())),
        );
        assert_eq!(
            read("Supertype(Basic)"),
            Filter::Characteristic(CharacteristicFilter::Supertype(Supertype::Basic)),
        );
        assert_eq!(
            read("InZone(Battlefield)"),
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
        );
        assert_eq!(read("Kind(Player)"), Filter::Kind(ObjectKind::Player));
    }

    #[test]
    fn combinators_nest() {
        assert_eq!(
            read("AllOf([Kind(Permanent), Type(Creature)])"),
            Filter::AllOf(vec![
                Filter::Kind(ObjectKind::Permanent),
                Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            ]),
        );
        assert_eq!(
            read("Not(Kind(Player))"),
            Filter::Not(Box::new(Filter::Kind(ObjectKind::Player))),
        );
    }

    #[test]
    fn relations_take_filters() {
        use crate::Reference;
        assert_eq!(
            read("Controller(Is(You))"),
            Filter::Relation(RelationFilter::Controller(Box::new(Filter::Is(
                Reference::You
            )))),
        );
    }

    /// The compartment wrappers must not appear in the text: Rust nests,
    /// RON stays flat.
    #[test]
    fn serialization_stays_flat() {
        let filter = Filter::Characteristic(CharacteristicFilter::Type(Type::Creature));
        assert_eq!(
            crate::ron::options().to_string(&filter).unwrap(),
            "Type(Creature)"
        );
    }

    #[test]
    fn filters_round_trip() {
        let source = "OneOf([AllOf([Kind(Permanent),Type(Battle)]),Kind(Player)])";
        let parsed = read(source);
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert_eq!(read(&written), parsed);
    }

    #[test]
    fn unknown_names_error() {
        assert!(
            crate::ron::options().from_str::<Filter>("Bogus(1)").is_err()
        );
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p deckmaste_core filter`
Expected: COMPILE FAIL — `Filter`, `CharacteristicFilter`, etc. not defined

- [ ] **Step 3: Write the implementation**

Prepend to `crates/deckmaste_core/src/filter.rs` (above the test module):

```rust
use std::fmt;

use serde::de::{self, EnumAccess, VariantAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ident::IdentSeed;
use crate::{Ident, Reference, Supertype, Type, Zone};

/// What kind of object something is (CR 109.1). Players are objects here
/// too — the engine gives players ObjectIds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ObjectKind {
    Card,
    Emblem,
    Permanent,
    Player,
    Spell,
    Token,
}

/// Characteristic atoms (CR 109.3): facts printed on or defined for the
/// object. `Subtype` filters by *name* — validating that the name is a
/// declared subtype is a lint, not a parse concern.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CharacteristicFilter {
    Type(Type),
    Subtype(Ident),
    Supertype(Supertype),
}

/// State atoms: where the object is and what's on it — not
/// characteristics (CR 110.5a, 122.1).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum StateFilter {
    InZone(Zone),
}

/// Structural relations the engine owns. Relations are
/// implicitly existential: `Controller(IsOpponent-shaped)` means "whose
/// controller matches".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum RelationFilter {
    Controller(Box<Filter>),
    Owner(Box<Filter>),
    OpponentOf(Box<Filter>),
}

/// A predicate over game objects, players included. Compartmentalized in
/// Rust; flat in RON (`Type(Creature)`, never
/// `Characteristic(Type(Creature))`) via the manual serde impls below.
///
/// Conjunction is explicit (`AllOf`) — an enum position never carries a
/// bare list. Canonical filters are context-free-correct: state the whole
/// predicate even where engine context would make parts redundant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Filter {
    Kind(ObjectKind),
    Characteristic(CharacteristicFilter),
    State(StateFilter),
    Relation(RelationFilter),
    Is(Reference),
    AllOf(Vec<Filter>),
    OneOf(Vec<Filter>),
    Not(Box<Filter>),
}

/// Every name a Filter position accepts, compartments flattened: the
/// variant list the macro layer's enum interception checks before trying
/// Filter macros. Names must stay globally unique across compartments.
const VARIANTS: &[&str] = &[
    "Kind",
    "Type",
    "Subtype",
    "Supertype",
    "InZone",
    "Controller",
    "Owner",
    "OpponentOf",
    "Is",
    "AllOf",
    "OneOf",
    "Not",
];

// Manual serde, not `#[serde(untagged)]` wrappers: untagged variants
// deserialize through `deserialize_any`, which never reaches the macro
// layer's `deserialize_enum` interception — `AnyTarget` would stop
// expanding at Filter positions. Dispatching by name over one combined
// variant list keeps the RON flat *and* the positions macro-aware.
impl<'de> Deserialize<'de> for Filter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FilterVisitor;

        impl<'de> Visitor<'de> for FilterVisitor {
            type Value = Filter;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a filter")
            }

            fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Filter, A::Error> {
                use CharacteristicFilter as C;
                use RelationFilter as R;
                use StateFilter as S;

                let (ident, v) = data.variant_seed(IdentSeed)?;
                Ok(match ident.as_str() {
                    "Kind" => Filter::Kind(v.newtype_variant()?),
                    "Type" => Filter::Characteristic(C::Type(v.newtype_variant()?)),
                    "Subtype" => Filter::Characteristic(C::Subtype(v.newtype_variant()?)),
                    "Supertype" => Filter::Characteristic(C::Supertype(v.newtype_variant()?)),
                    "InZone" => Filter::State(S::InZone(v.newtype_variant()?)),
                    "Controller" => Filter::Relation(R::Controller(v.newtype_variant()?)),
                    "Owner" => Filter::Relation(R::Owner(v.newtype_variant()?)),
                    "OpponentOf" => Filter::Relation(R::OpponentOf(v.newtype_variant()?)),
                    "Is" => Filter::Is(v.newtype_variant()?),
                    "AllOf" => Filter::AllOf(v.newtype_variant()?),
                    "OneOf" => Filter::OneOf(v.newtype_variant()?),
                    "Not" => Filter::Not(v.newtype_variant()?),
                    _ => return Err(de::Error::unknown_variant(&ident, VARIANTS)),
                })
            }
        }

        deserializer.deserialize_enum("Filter", VARIANTS, FilterVisitor)
    }
}

impl Serialize for Filter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Filter::Kind(kind) => {
                serializer.serialize_newtype_variant("Filter", 0, "Kind", kind)
            }
            // The compartments serialize transparently: RON writes only the
            // inner variant, so the text stays flat.
            Filter::Characteristic(c) => c.serialize(serializer),
            Filter::State(s) => s.serialize(serializer),
            Filter::Relation(r) => r.serialize(serializer),
            Filter::Is(r) => serializer.serialize_newtype_variant("Filter", 8, "Is", r),
            Filter::AllOf(fs) => {
                serializer.serialize_newtype_variant("Filter", 9, "AllOf", fs)
            }
            Filter::OneOf(fs) => {
                serializer.serialize_newtype_variant("Filter", 10, "OneOf", fs)
            }
            Filter::Not(f) => serializer.serialize_newtype_variant("Filter", 11, "Not", f),
        }
    }
}
```

In `crates/deckmaste_core/src/lib.rs` add (alphabetical):

```rust
mod filter;
pub use filter::{CharacteristicFilter, Filter, ObjectKind, RelationFilter, StateFilter};
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p deckmaste_core filter`
Expected: PASS (all 7 tests)

- [ ] **Step 5: Commit**

```bash
jj commit -m "core: Filter — compartments in rust, flat in ron"
```

---

### Task 4: Selection

**Files:**
- Create: `crates/deckmaste_core/src/selection.rs`
- Modify: `crates/deckmaste_core/src/lib.rs`

- [ ] **Step 1: Write the type and tests**

Create `crates/deckmaste_core/src/selection.rs`:

```rust
use serde::{Deserialize, Serialize};

use crate::{Filter, Reference};

/// A Filter lifted into a choice context: who picks, when, how many —
/// and, for the target quantifiers, what gets *bound* for later
/// `Reference::Target(n)` use (CR 115, 601.2c).
///
/// Not a Filter variant on purpose: filters compose under
/// AllOf/OneOf/Not, quantifiers don't, and bare-Filter positions
/// (protection qualities, event participants) must not admit "target".
/// Quantity-bearing quantifiers (`Targets(n, …)`, `UpToTargets`,
/// `Random`) arrive with the Quantity module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Selection {
    /// One target: announced, rechecked at resolution.
    Target(Filter),
    /// Every matching object, evaluated when the instruction applies.
    Each(Filter),
    /// All matching objects, as a single set.
    All(Filter),
    /// One untargeted choice.
    Choose(Filter),
    /// An already-bound object: references lift into Selection here.
    That(Reference),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CharacteristicFilter, ObjectKind, Type};

    fn read(source: &str) -> Selection {
        crate::ron::options().from_str(source).unwrap()
    }

    #[test]
    fn quantifiers_wrap_filters() {
        assert_eq!(
            read("Target(Type(Creature))"),
            Selection::Target(Filter::Characteristic(CharacteristicFilter::Type(
                Type::Creature
            ))),
        );
        assert_eq!(
            read("Each(Kind(Player))"),
            Selection::Each(Filter::Kind(ObjectKind::Player)),
        );
    }

    /// `Target` in Selection takes a Filter; `Target` in Reference takes
    /// an index. Distinct types, distinct positions — never ambiguous.
    #[test]
    fn references_lift_via_that() {
        assert_eq!(read("That(Target(0))"), Selection::That(Reference::Target(0)));
        assert_eq!(read("That(This)"), Selection::That(Reference::This));
    }

    #[test]
    fn selections_round_trip() {
        let source = "Target(AllOf([Kind(Permanent),Type(Creature)]))";
        let parsed = read(source);
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert_eq!(read(&written), parsed);
    }
}
```

In `crates/deckmaste_core/src/lib.rs` add (alphabetical):

```rust
mod selection;
pub use selection::Selection;
```

- [ ] **Step 2: Run the tests**

Run: `cargo test -p deckmaste_core selection`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
jj commit -m "core: Selection — filters lifted into choice contexts"
```

---

### Task 5: Replace Target/Selector in ability.rs

**Files:**
- Modify: `crates/deckmaste_core/src/ability.rs`
- Modify: `crates/deckmaste_core/src/lib.rs:3-4`

- [ ] **Step 1: Rewrite ability.rs**

Replace the entire contents of `crates/deckmaste_core/src/ability.rs` with:

```rust
use serde::{Deserialize, Serialize};

use crate::{Ident, Selection};

// Temporary types.
type ParamValue = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Effect {
    DealDamage(Selection, crate::Uint),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Expanded<T> {
    pub params: Vec<ParamValue>,
    pub value: Box<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KeywordAbility {
    pub keyword: Ident,
    pub expanded: Expanded<Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SpellAbility {
    pub targets: Vec<Selection>,
    pub effect: Effect,
}

// The struct-carrying variants read flat in RON — `Spell(targets: ..., ...)`,
// not `Spell((targets: ...))` — via the unwrap_variant_newtypes extension.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Ability {
    Static,
    Activated,
    Triggered,
    Spell(SpellAbility),
    Keyword(KeywordAbility),
}
```

(Deleted: `Target`, `Selector`. `Effect::DealDamage` now takes a `Selection` — "deals 3 damage to each creature" needs a quantifier, not just an index.)

In `crates/deckmaste_core/src/lib.rs` change line 4 to:

```rust
pub use ability::{Ability, Effect, KeywordAbility, SpellAbility};
```

- [ ] **Step 2: Run core tests**

Run: `cargo test -p deckmaste_core`
Expected: PASS (the cards crate is red until Tasks 6–7 — that's expected; core must be green)

- [ ] **Step 3: Commit**

```bash
jj commit -m "core: targets are Selections, Selector dies"
```

---

### Task 6: MacroKind — drop Target, add Filter/Reference/Selection

**Files:**
- Modify: `crates/deckmaste_cards/src/macros.rs:46-64` (enum + from_position) and its tests

- [ ] **Step 1: Update the enum and from_position**

In `crates/deckmaste_cards/src/macros.rs`, replace the `MacroKind` enum and `from_position`:

```rust
/// The kinds of value a macro can expand to: the types whose parse positions
/// consult the macro namespace. Variant names must match the Rust types'
/// serde names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum MacroKind {
    Ability,
    CardFace,
    Filter,
    Reference,
    Selection,
    Subtype,
}

impl MacroKind {
    /// The kind a position with this (serde) type name checks, if any.
    #[must_use]
    pub fn from_position(name: &str) -> Option<Self> {
        Some(match name {
            "Ability" => MacroKind::Ability,
            "CardFace" => MacroKind::CardFace,
            "Filter" => MacroKind::Filter,
            "Reference" => MacroKind::Reference,
            "Selection" => MacroKind::Selection,
            "Subtype" => MacroKind::Subtype,
            _ => return None,
        })
    }
}
```

- [ ] **Step 2: Update the macros.rs tests**

Still in `crates/deckmaste_cards/src/macros.rs`:

(a) Change the test-module import (currently `use deckmaste_core::{Ability, Subtype, Target, Type};`) to:

```rust
use deckmaste_core::{Ability, Filter, ObjectKind, Subtype, Type};
```

(b) In `position_names_track_the_core_types`, replace the assertions with:

```rust
        assert_eq!(position::<Ability>(), Some(MacroKind::Ability));
        assert_eq!(
            position::<deckmaste_core::CardFace>(),
            Some(MacroKind::CardFace)
        );
        assert_eq!(position::<Filter>(), Some(MacroKind::Filter));
        assert_eq!(
            position::<deckmaste_core::Reference>(),
            Some(MacroKind::Reference)
        );
        assert_eq!(
            position::<deckmaste_core::Selection>(),
            Some(MacroKind::Selection)
        );
        assert_eq!(position::<Subtype>(), Some(MacroKind::Subtype));
```

(c) Rewrite `macros_are_namespaced_by_kind` against Filter instead of Target:

```rust
    #[test]
    fn macros_are_namespaced_by_kind() {
        // One macro can serve several kinds, and is only visible at
        // positions of those kinds.
        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "Self".into(),
                kinds: vec![MacroKind::Subtype, MacroKind::Filter],
                params: Params::Positional(vec![ParamType::String]),
                body: "Param(0)".into(),
            })
            .unwrap();

        let filter: Filter = macros.read_str("Self(Kind(Player))").unwrap();
        assert_eq!(filter, Filter::Kind(ObjectKind::Player));

        // The macro is invisible at an Ability position.
        let error = macros.read_str::<Ability>("Self(Static)").unwrap_err();
        assert!(
            error
                .to_string()
                .contains("neither a variant of `Ability` nor a known `Ability` macro"),
            "unexpected error: {error}"
        );
    }
```

(d) Add a test pinning the manual-serde/macro interaction — the reason Filter's serde is hand-written:

```rust
    /// Filter's manual Deserialize must go through `deserialize_enum` with
    /// the full flattened variant list: that is what lets unknown names at
    /// Filter positions fall through to the macro namespace.
    #[test]
    fn filter_positions_expand_macros() {
        let mut macros = MacroSet::default();
        macros
            .insert(&MacroDef {
                name: "AnyTargetish".into(),
                kinds: vec![MacroKind::Filter],
                params: Params::default(),
                body: "OneOf([Kind(Player), AllOf([Kind(Permanent), Type(Creature)])])".into(),
            })
            .unwrap();
        let filter: Filter = macros.read_str("AnyTargetish").unwrap();
        let Filter::OneOf(arms) = filter else {
            panic!("expected OneOf, got {filter:?}");
        };
        assert_eq!(arms[0], Filter::Kind(ObjectKind::Player));
        assert_eq!(arms.len(), 2);
    }
```

- [ ] **Step 3: Flip the two macro definition files in the same task**

The plugin-loading unit tests (`plugin::tests`) parse the real
`plugins/builtin/macros/**` — a `kinds: [Target]` file is now a
`MacroDef` parse error, so these files change together with the enum.

Write `plugins/builtin/macros/filter/AnyTarget.ron` (and delete
`plugins/builtin/macros/target/AnyTarget.ron` — directory paths are
organizational only):

```ron
// CR 115.4
(
    name: "AnyTarget",
    template: "any target",
    kinds: [Filter],
    body: OneOf([
        AllOf([Kind(Permanent), Type(Battle)]),
        AllOf([Kind(Permanent), Type(Creature)]),
        AllOf([Kind(Permanent), Type(Planeswalker)]),
        Kind(Player),
    ]),
)
```

Write `plugins/builtin/macros/example/Self.ron`:

```ron
// The identity macro: expands to its argument, untouched.
(
    name: "Self",
    kinds: [Subtype, Filter],
    params: [String],
    body: Param(0),
)
```

- [ ] **Step 4: Run the cards unit tests**

Run: `cargo test -p deckmaste_cards --lib`
Expected: PASS — plugin-loading tests parse the updated macro files; the
`tests/` integration suite is not built by `--lib` and goes green in Task 7

- [ ] **Step 5: Commit**

```bash
jj commit -m "cards: MacroKind gains Filter/Reference/Selection; AnyTarget is a Filter macro"
```

---

### Task 7: Re-encode Lightning Bolt

**Files:**
- Modify: `plugins/builtin/cards/Lightning Bolt.ron`
- Modify: `crates/deckmaste_cards/tests/builtin.rs`

- [ ] **Step 1: Update the failing integration test first**

In `crates/deckmaste_cards/tests/builtin.rs`, change the core imports to:

```rust
use deckmaste_core::{
    Ability, Card, CardFace, CharacteristicFilter, Effect, Filter, ManaCost, ObjectKind,
    Reference, Selection, SpellAbility, StatValue, Subtype, Supertype, Type,
};
```

and replace `lightning_bolt_expands_target_macros` with:

```rust
/// Filter-position interception through real data: `Target(AnyTarget)`
/// expands `AnyTarget` at the Selection's Filter payload.
#[test]
fn lightning_bolt_expands_filter_macros() {
    let card = builtin().card("Lightning Bolt").unwrap();
    let Card::Normal(face) = card else {
        panic!("Lightning Bolt should be single-faced");
    };
    let permanent_of = |t: Type| {
        Filter::AllOf(vec![
            Filter::Kind(ObjectKind::Permanent),
            Filter::Characteristic(CharacteristicFilter::Type(t)),
        ])
    };
    let any_target = Filter::OneOf(vec![
        permanent_of(Type::Battle),
        permanent_of(Type::Creature),
        permanent_of(Type::Planeswalker),
        Filter::Kind(ObjectKind::Player),
    ]);
    assert_eq!(
        face.abilities,
        vec![Ability::Spell(SpellAbility {
            targets: vec![Selection::Target(any_target)],
            effect: Effect::DealDamage(Selection::That(Reference::Target(0)), 3),
        })]
    );
}
```

- [ ] **Step 2: Run to verify it fails**

Run: `cargo test -p deckmaste_cards --test builtin`
Expected: FAIL — the card file still says `targets: [AnyTarget]`, and at a
`Selection` position `AnyTarget` is "neither a variant of `Selection` nor a
known `Selection` macro" (it is a *Filter* macro now; the Selection wrapper
is exactly what the new encoding adds)

- [ ] **Step 3: Update the card file**

Write `plugins/builtin/cards/Lightning Bolt.ron`:

```ron
Normal(
    name: "Lightning Bolt",
    mana_cost: [Red],
    types: [Instant],
    abilities: [
        Spell(
            targets: [Target(AnyTarget)],
            effect: DealDamage(That(Target(0)), 3),
        ),
    ],
)
```

- [ ] **Step 4: Run the full workspace tests**

Run: `cargo test --workspace`
Expected: PASS — including `tests/validate_builtin.rs` (builtin cards must parse through the real reader) and the unchanged basic-land/Grizzly Bears tests

- [ ] **Step 5: Commit**

```bash
jj commit -m "builtin: AnyTarget is a Filter macro; bolt re-encoded"
```

---

### Task 8: Format, lint, wrap up

**Files:** none new

- [ ] **Step 1: Format and lint**

Run (rustfmt options here are nightly-only; stable silently skips them):

```bash
cargo +nightly fmt
cargo clippy --workspace --all-targets
```

Expected: no warnings. If clippy flags the `Filter::serialize` match (e.g. `match_same_arms`), restructure per its suggestion rather than allow-listing.

- [ ] **Step 2: Full test sweep**

Run: `cargo test --workspace`
Expected: PASS

- [ ] **Step 3: Commit any formatting fallout**

```bash
jj commit -m "fmt + clippy"
```

(If fmt/clippy changed nothing, skip the commit.)

---

## Follow-on plans (not this plan)

Per spec §10, each its own plan + green state:

2. **Cost + Action intrinsics + keyword-action declarations** — `Cost`/`CostComponent`, `Action`, `definitions/keyword_actions/`, unshelve the tokens migration (jj bookmark `tokens-shelved`).
3. **Expanded everywhere** — `Expansion<T>`, serialization-as-invocation, `Ability::Keyword` dissolves, MacroKind-from-Expanded.
4. **Event + Condition** — pattern forms, bindings, `Happened`, ability-word Condition macros.
5. **Effect AST** — `Sequence`, `May`/`If`/`Unless`/`ForEach`, delayed/reflexive (validate.rs's planned lint for degenerate `Sequence` lands here).
6. **StaticEffect/ContinuousEffect + Duration**, then Replacement/Prevention/Mode and remaining leaves.
