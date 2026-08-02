---
needs: []
design: true
---
**Give filters a parameterized `PermanentOfSubtype(S)` / `PermanentOfType(T)`
wrapper so authored cards stop naming types/subtypes with stringly
`Subtype("…")` / `Type("…")` atoms.** [CR#109.2]: a bare type or subtype
reference means *a permanent of that type/subtype on the battlefield*. Today
that battlefield-conjunct wrapper exists only for the ~10 card types
(hand-authored `Predicate` twins) and **not at all for subtypes**, so subtype
filters fall back to stringly channels.

## Current state (why the strings are there)

Macro resolution is a strict two-level `(kind, name)` lookup — no coercion. A
`Subtype`/`TypeDef`-kind macro is invisible in a `Predicate` slot
(`crates/macro_ron/src/expand.rs:341` `Probe::visit_enum`,
`crates/macro_ron/src/set.rs:396` `MacroSet::get`); an unresolved ident falls
through to the position's native parser. So a type/subtype only works *in a
filter* if a **`Predicate`-kind macro of that name** exists.

- **Card types have terse `Predicate` twins** (name-overload-by-slot-kind, the
  live mechanism): `plugins/builtin/macros/filter/Creature.ron` =
  `(template: "creature", kinds: [Predicate], body: And([Permanent,
  Type("Creature")]))`, distinct from the `TypeDef` macro
  `plugins/builtin/macros/cardtype/Creature.ron`. The battlefield conjunct is
  hand-copied into each type twin, and its inner arg is the stringly
  `Type("Creature")`.
- **Subtypes have no such twin.** The ~994 generated subtype instances
  (`crates/deckmaste_migrations/src/stubs/subtypes.rs`, meta-macros
  `plugins/builtin/macros/macro/subtype/*`) are `Subtype`-kind only. So a
  subtype in a filter is authored as the raw `And([Permanent, Subtype("X")])`
  or the bare stringly `Subtype("X")` — e.g. `plugins/canon/cards/Standard
  Bearer.ron:23` hand-writes `And([Permanent, Subtype("Flagbearer")])`;
  `plugins/canon/cards/Falkenrath Gorger.ron:39` uses `Subtype("Vampire")`.

A twin **per subtype** (~1000 more generated `Predicate` files) is rejected —
too much generation, two-macro sync. Auto-coercion (a `Subtype` arg
auto-wrapping into a predicate) would be a resolver-semantics change to
`macro_ron` with wide blast radius — also rejected. The parameterized wrapper
below gets the same result with **one macro** and no engine change.

## Design (settled)

1. **Two parameterized `Predicate` macros**, each citing [CR#109.2], template a
   bare passthrough (`"${0}"` — supported by the template engine
   (`crates/deckmaste_plugin/src/render/template.rs:114`), no precedent card yet):
   - `PermanentOfSubtype(S)` → `body: And([Permanent, Subtype(${0})])`, arg a
     `Subtype`-kind slot. `PermanentOfSubtype(Vampire)` renders "Vampire"
     (subtype = proper noun, capitalized as-written, so `"${0}"` is correct).
     **This is the authored spelling for subtype filters** — the whole point,
     since subtypes have no terse twin.
   - `PermanentOfType(T)` → `body: And([Permanent, Type(${0})])`, arg a
     `TypeDef`-kind slot. Cite [CR#109.2].
2. **Refactor the card-type filter twins onto it** — keep them (no churn to the
   ~160 bare-`Creature` filter sites), but change each twin whose body is
   `And([Permanent, Type("X")])` (`filter/Creature.ron` and its type siblings —
   *not* the action/other predicates in that dir: Destroy/Draw/Player/…) to
   `body: PermanentOfType(X)` with the unquoted `TypeDef` arg. The twin keeps its
   own `template` (e.g. `"creature"`), so terse `Creature`-in-filter stays and
   renders lowercase — the twin owns type casing, `PermanentOfType` stays purely
   structural and its own `"${0}"` template is never exercised for types.
   Battlefield conjunct now lives in ONE place.
3. **Migrate authored stringly subtype/type *battlefield* filters** to the
   wrappers: `And([Permanent, Subtype("Flagbearer")])` → `PermanentOfSubtype(
   Flagbearer)`; a battlefield `Type("X")` → the terse type twin (or
   `PermanentOfType(X)`).

## The one open impl question (design pass)

The inner `Type(_)` / `Subtype(_)` `Predicate` variants are **stringly**
(`String` payload, `crates/deckmaste_core/src/filter.rs`). Threading an
unquoted `TypeDef`/`Subtype` macro arg through `${0}` into those slots needs
either (a) **Param forwarding** that adapts the macro arg into the `String`
slot at expansion (macros support Param forwarding — confirm it bridges the
kind), or (b) **typing the inner payloads** (`Type`/`Subtype` take a typed ref,
not `String`). Note the *surface* goal — authored cards free of stringly
type/subtype atoms — is met even if (a) leaves the string inside macro
expansion (same as subtype-def bodies already carrying `CreatureType(name:
"Vampire")`). Deep de-stringing (b) is cleaner but larger; decide scope.

## Do NOT convert the deliberate zone-agnostic uses

`Type("Creature")` matching cards in *non-battlefield* zones is [CR#109.2a]
(card, named zone), semantically distinct from the [CR#109.2] battlefield default —
e.g. `plugins/canon/cards/Falkenrath Gorger.ron:35-38` (inline comment) and
`docs/tickets/done/engine-act-facet-contract.md:79` document `Creature`
(macro, `Permanent ∧ Type Creature`) vs `Type("Creature")` (zone-agnostic).
Migration to `PermanentOf…` is **case-by-case**: only battlefield-scoped uses.
The zone-agnostic form wants a *card*/any-zone atom, not the battlefield
wrapper — its de-stringing (unquoted arg to `Type`) rides on the open question
above, not on the wrapper.

## Scope / linkage

Grammar/authoring-surface hygiene, `planned/` not `critical/` (not an engine
happy-path seam). Standard constraints apply (idris soundness — `PermanentOf…`
are `Predicate` composites over existing `And`/`Permanent`/`Type`/`Subtype`, no
new intrinsic; render parity — twin refactor and the migration must leave
oracle text byte-identical; CR citations; wizards regen; fmt/clippy). Related:
`parse-subject-filter-stringly-channels` (planned/ — the parser/renderer
`TYPE_NOUN_ATOMS` side of the same disease; structural `TypeAtom` proposal),
`spell-filter-subtype-validation` (planned/), `filter-head-subtype-validation`
(done/ — origin of the lone-word→`Subtype("…")` fallback).
