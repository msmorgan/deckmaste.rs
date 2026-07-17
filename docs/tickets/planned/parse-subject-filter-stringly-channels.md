---
needs: []
---
**Three parser sites flatten a known subject/filter distinction into a
rendered-RON `String`, then a consumer recovers it by sniffing that string.** Same
disease as [[parse-damage-target-stringly-channel]], in the filter/subject layer:
the parser discards structure it already had, then re-parses its own output —
distinct from legitimate English-phrase reshaping (this reads RON, not English).
All under `crates/deckmaste_migrations/src/parsers/`.

- **`modify.rs:75-81` `filter_to_target`** — `subject_to_filter` (`:27-46`) knows
  at parse time whether the subject was self (`~`/"this …"), attach-host
  ("enchanted/equipped …"), or a class phrase, then flattens all three to a
  `Predicate`-RON string (`Ref(This)`, `Ref(AttachHostOf(This))`, `And([…])`);
  `filter_to_target` recovers it with `strip_prefix("Ref(")`, defended only by a
  **cross-module** invariant ("`parse_phrase` always leads with a head-noun atom").
  Any future `filter::parse_phrase` production that legitimately emits a top-level
  `Ref(…)` (an anaphor head) silently mis-routes `Each(SelectAll(…))` → a
  single-object `Modify`. Consumers: `static_ability.rs:101,118`, `effect.rs:717`.
- **`replacement.rs:311-324` `controller_suffix`** — `strip_controller` (`:236-244`)
  parses the English controller phrase to its RON atom (`ControlledBy(Ref(You))`),
  then this `match`es on *that emitted RON* to regenerate the English suffix ("you
  control") to re-feed `filter::parse_phrase`. A RON→English→RON bounce; a third
  controller phrase must then be added in two string tables that agree only by
  convention.
- **`filter.rs:469-481` `type_filter`** (minor) — `TYPE_NOUN_ATOMS` (`:444-454`)
  stores rendered-RON strings mixing macro names (`"Creature"`) and `Type("…")`
  wrappers; `type_filter` un-renders with `strip_prefix("Type(\"")`. Const/local,
  low risk, same discipline breach.

## Fix

A small enum at the point each distinction is born, and each consumer renders its
own view:

- `subject_to_filter` → `Subject { SelfRef, AttachHost, Class(String) }` (or a
  `Target` directly); `parse_restriction`/`parse_grant`/`pump_scope` render their
  own slot shape.
- `strip_controller` → `(Controller, &str)` with `Controller { You, Opponents }`;
  derive both the atom and the suffix from it.
- `TypeAtom { Macro(&'static str), CardType(&'static str) }`.

The healthy model already in-tree: `count.rs`'s `Binder`/`CountClause` enums carry
exactly this kind of distinction structurally. See
[[atom-independence-anaphora-only]], [[authored-surface-ergonomics-rulings]].

Verify: the existing modify/replacement/filter unit tests pin the same emitted RON;
a `filter::parse_phrase` production emitting a top-level `Ref(…)` no longer
mis-routes `filter_to_target`; `cargo xtask generate plugins/wizards` emits affected
cards unchanged; `cargo test --workspace` green.
