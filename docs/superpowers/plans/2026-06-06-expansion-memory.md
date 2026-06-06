# Expansion Memory Implementation Plan (Plan 3)

**Goal:** Every enum macro kind remembers its top-level invocation
(`Expanded(Expansion<Self>)`); serialization writes the invocation back;
`Ability::Keyword`/`KeywordAbility`/old `Expanded<T>` dissolve into the
uniform mechanism.

**The mechanism.** The macro layer cannot wrap a deserialized value after
the fact (visitor outputs are opaque), so it wraps *in the stream*: when a
macro `M(args…)` expands at an enum position whose kind carries an
`Expanded` variant, expand.rs synthesizes (into the existing splice arena)

```ron
Expanded(name: "M", args: Positional(["<arg1 source>", …]), value: <body source>)
```

and rereads *that* (frame in scope, so `Param` holes inside the body still
resolve; arg strings are the caller's raw text and hole-free; escaping via
`format!("{arg:?}")` — RON string syntax matches Rust's). The kind's own
(derived or manual) Deserialize handles the `Expanded` variant like any
other. Macro-to-macro chains nest naturally (`Woods` → `Forest` → …).

**Scope cut:** struct kinds (CardFace, Subtype) stay name-erasing — Subtype
already self-names, and nothing engine-meaningful invokes CardFace macros.
Enum kinds get the variant: **Ability, Filter, Selection, Reference,
Effect, CostComponent**. A `MacroKind::remembers_expansion()` table in the
cards crate says which kinds wrap.

## Core (`deckmaste_core`)

New `expansion.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ExpansionArgs {
    Positional(Vec<String>),          // raw RON source per argument
    Named(Vec<(Ident, String)>),
}

/// A remembered macro invocation: the name and raw arguments as written,
/// plus the value the body expanded to. PartialEq is provenance-sensitive
/// by design: `Expanded(Flying, …)` ≠ its raw expansion.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Expansion<T> {
    pub name: Ident,
    #[serde(default = "ExpansionArgs::none")]
    pub args: ExpansionArgs,
    pub value: Box<T>,
}
```

Manual `Serialize for Expansion<T>` writes the INVOCATION, not the struct —
the `&'static str` from `Ident::as_str()` satisfies serde's variant-name
lifetimes, and `ron::value::RawValue` re-emits stored arg source verbatim:

- nullary → `serialize_unit_variant("", 0, name)` → `Flying`
- one positional → `serialize_newtype_variant` with `RawValue::from_ron(arg)`
- n positional → `serialize_tuple_variant`, RawValue elements
- named → `serialize_struct_variant`, field names from interned Idents

Kind changes:
- `Ability`: `Keyword(KeywordAbility)` variant, `KeywordAbility`, and the
  old `Expanded<T>` struct are DELETED; `Expanded(Expansion<Ability>)`
  added. Spell/Activated/Triggered/Static untouched.
- `Filter`, `Effect` (manual serde): "Expanded" joins VARIANTS and the
  visitor match (`Filter::Expanded(v.newtype_variant()?)`); Serialize arm
  delegates to `Expansion::serialize` (flat invocation out).
- `Selection`, `Reference`, `CostComponent` (derived): plain new variant —
  derived Deserialize accepts the synthesized stream; derived Serialize
  would write `Expanded(...)` literally, which is WRONG for these three, so
  they get small manual Serialize impls (match: Expanded delegates to
  Expansion, others mirror the derive via serialize_newtype_variant/unit) —
  or keep derives and accept `#[serde(untagged)]`-free manual impls only
  where tests prove the need; the plan REQUIRES invocation-serialization
  for all six kinds, tested.

Core tests: Expansion serialization for all four arg shapes (exact strings);
provenance-sensitive equality.

## Cards crate

- `MacroKind::remembers_expansion(self) -> bool` (true for the six).
- expand.rs: in `EnumIntercept::visit_enum`'s macro path (and the
  struct-position `via_capture` macro path does NOT change — struct kinds
  don't remember), synthesize the wrapper text instead of rereading the
  body directly, when the position's kind remembers. The synthesized text
  goes through `ReadCtx::splice`. Frame/depth handling identical.
- macros.rs test updates: bodies that wrote
  `Keyword(keyword: "Flying", expanded: (params: [], value: Static))`
  become `Static`, with assertions on the `Expanded` wrapper
  (`name == "Flying"`, `value == Ability::Static`). New tests: chain
  nesting (`Woods` → `Forest`), args memory (a parameterized invocation's
  raw arg text survives), round-trip (read `Flying` with macros in scope →
  serialize → exactly `Flying`).
- Integration expectations grow wrappers: builtin.rs Lightning Bolt
  (`Filter::Expanded { name: "AnyTarget", value: OneOf(…) }`), tokens.rs
  (`CostComponent::Expanded { name: "SacrificeThis", … }`). Update to match
  — this is the feature, not a regression.

## Migrations + data

- `_007_keyword_abilities` template body becomes just `Static` (name/args
  now captured by the mechanism); regenerate wizards (`migrate 7` after
  deleting `plugins/wizards/macros/keyword_abilities/`, then `migrate 8`
  fresh), `cargo xtask validate plugins/wizards` → 0 invalid.
- builtin macros (AnyTarget, SacrificeThis, Self) need no body changes.

## Gates

Workspace tests green at each batch boundary; fmt+clippy; validate both
plugins at the end.
