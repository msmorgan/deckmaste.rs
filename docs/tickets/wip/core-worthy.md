---
needs: []
---
Worthy [CR#700.16] — a rules term introduced in the June 19 2026 CR: "A creature
is worthy if it's legendary, isn't a Villain, and is red and/or white." ~3 cards
(MSH) refer to "worthy" creatures.

This is a NAMED COMPOUND PREDICATE, not a new engine primitive — every piece it
needs already exists on `Predicate` (`crates/deckmaste_core/src/filter.rs`):

    AllOf([
      Supertype(Legendary),
      Not(Subtype("Villain")),
      OneOf([ColorIs(Red), ColorIs(White)]),
    ])

Work:
- A grammar/macro atom that recognizes the adjective "worthy" (applied to a
  creature) and expands to the conjunction above. Model it on the other named
  characteristic-adjective macros under `plugins/builtin/macros/` — a `Worthy`
  macro whose body is the `AllOf` predicate, so it round-trips parse⇄render like
  any other predicate atom.
- Parser: wire the bare adjective "worthy" (and "worthy creature") in the
  object-description production (`crates/deckmaste_migrations/src/parsers/filter.rs`)
  to emit the macro invocation.

## Done
- "worthy" parses to the compound predicate and renders back to "worthy"; the ~3
  flagged cards graduate. No engine change; `idris-check` count unchanged;
  `cargo test --workspace` green.
