---
needs: []
---
**Rename remaining Rust types/variants to mirror the Idris model's naming.** The
concept-correcting renames (anaphors, binders, event roles) landed in
`idris-naming-reconciliation` and `idris-naming-residuals`. Several *type-level*
and *variant-level* naming gaps remain — these are the same-concept divergences
the naming policy forbids ("never preserve a weaker Rust concept behind a
matching name").

## Type renames

### 1. `Filter` → `Predicate`
The largest rename. Rust `Filter` is Idris `Predicate` — same concept (a
boolean test over objects or players), same variants, same combinator structure.
The Rust name masks the connection: `Filter` reads as an action (imperative),
while `Predicate` reads as a proposition (declarative), matching the Idris
model's dependent-type discipline.

**Scope:** `pub enum Filter` in `crates/deckmaste_core/src/filter.rs` and every
reference across all crates, plugins (`*.ron` files), the Idris serialization
layer (`idris/src/Ron.idr`), docs, and tickets.

### 2. `DivideAmong` → `Distribute`
Rust `Effect::DivideAmong` / `struct DivideAmong` is Idris `Distribute` — same
`Bindable b Many k` divided-distribution concept. The Rust name was chosen to
avoid colliding with `std::iter::Iterator::distribute` (which doesn't exist),
but it masks the correspondence.

## Variant renames (inside `Filter`/`Predicate`)

### 3. `AllOf` → `And`, `OneOf` → `Or`
Idris `Predicate` uses `And`/`Or`/`Not`; Rust `Filter` uses `AllOf`/`OneOf`/
`Not`. The `Not` already matches; `AllOf`→`And` and `OneOf`→`Or` close the gap.
Applies to both `Filter` (→`Predicate`) and `Condition`.

### 4. `Selection::Filter` → `Selection::SelectAll`
Idris `Selection::SelectAll(Predicate)` is "every match as a group"; Rust spells
it `Selection::Filter(Filter)`. Once `Filter` becomes `Predicate`, this variant
should become `SelectAll(Predicate)` to mirror the Idris constructor.

### 5. `Condition::Is` → `Condition::Matches`
Idris `Condition::Matches(ref, pred)` — "does ref satisfy pred"; Rust spells it
`Condition::Is(Reference, Filter)`. Rename to `Matches(Reference, Predicate)`.

### 6. `Condition::Exists` → `Condition::Exists` (signature only)
Already named `Exists` in both — but Rust's takes `Filter` while Idris's uses
an existential over `Predicate`. Signature changes with the `Filter`→`Predicate`
rename; no variant rename needed.

## Plugin data migration

Every `.ron` card/macro/token file uses the serialized names. The renames above
affect serialization: `Filter(…)` → `Predicate(…)`, `AllOf(…)` → `And(…)`,
`OneOf(…)` → `Or(…)`, `DivideAmong(…)` → `Distribute(…)`. A migration script
(or `cargo xtask migrate`) should batch-rename across `plugins/`.

## Verification

1. `idris2 --build mtg.ipkg` in `idris/` — typecheck passes.
2. `cargo test --workspace` — all tests pass.
3. `cargo xtask cite check` — 0 stale citations.
4. Grep for stale names in docs, tickets, and comments.
