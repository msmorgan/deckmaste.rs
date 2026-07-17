---
needs: []
---
**Parsed mana-cost symbol lists render with no space after the comma
(`Mana([Red,Red])`), inconsistent with every other emitted list.** Cosmetic,
but the emitted `.todo`/RON should read uniformly: `Mana([Red, Red])`.

Every hand-built list fragment in the parsers joins with `", "` (targets,
cost components — `keyword_ability.rs:231` `components.join(", ")`, effect
bodies, `activated_ability.rs:139`), so a keyword line comes out mixed:
`Keyword(Kicker([Mana([White,White])]))` — the outer cost list `, `-spaced, the
inner mana-symbol list crammed. The single source is
`crates/deckmaste_migrations/src/parsers/cost.rs:158`:

```rust
Ok(Some(format!("Mana({})", ron_options().to_string(&mana)?)))
```

`ron_options()` is the **compact** RON writer (no space after separators), and
`ManaCost(Vec<ManaSymbol>)` serializes straight through it → `[Red,Red]`. This
is the only place a symbol run becomes a string, so the crammed form appears
everywhere a mana cost is embedded in a body (keyword args, activated/mana-
ability costs, `Static(CostModifier(... Reduce([Mana([...])])))`).

**Fix (pick one):**
- **Localized** — at `cost.rs:158`, join the symbols by hand:
  `format!("Mana([{}])", mana.iter().map(serialize_symbol).collect::<Vec<_>>().join(", "))`,
  matching the surrounding `.join(", ")` convention and leaving the shared
  `ron_options()` writer untouched (lowest blast radius).
- **Global** — give `ron_options()` a separator with a trailing space so *every*
  compact fragment gains `, `. Uniform, but re-spaces all serialized fragments
  and churns far more expectation strings — only if we want one comma style
  everywhere.

Either way the emitted body must stay valid RON (it round-trips today; the
space must not break the load).

Verify: update the parser-test expectation strings that pin the crammed form —
`Mana([X,Y])` → `Mana([X, Y])` in `keyword_ability.rs` (e.g. `:351`, `:432`),
`resolve.rs:353`, `static_ability.rs`, and any reject/round-trip fixtures;
`cargo xtask generate plugins/wizards` and confirm multi-symbol keyword costs
(e.g. `From Father to Son`, `Desolation Giant`, `Riptide Survivor`) now emit
`Mana([…, …])`; `cargo test --workspace` green; the card load path still parses
the re-spaced output.
