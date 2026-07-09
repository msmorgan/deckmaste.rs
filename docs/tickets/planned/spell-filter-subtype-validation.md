---
needs: []
---
**The cast/spell filter parser silently mints a bogus `Subtype(...)` atom for a
single word that is NOT a real subtype, instead of declining.** A latent soundness
gap surfaced during the cast-spell-trigger work in [[macro-second-wave]] (pre-existing;
not introduced there).

## The bug

`parse_cast_event` (and the shared single-word spell-filter fallback it uses in
`crates/deckmaste_migrations/src/parsers/`) resolves "you cast a <word> spell" by
trying `filter::type_filter(word)` and, on `None`, falling back to
`Subtype(to_rust_ident(word))`. That is correct for a real subtype ("Spirit",
"Arcane") but WRONG for a word that is a color, an ability word, or another
non-subtype qualifier:

- "you cast a **red** spell" → `Cast(what: And([Kind(Spell), Subtype("Red")]))` — "Red"
  is a color, not a subtype; the card parses to a filter that matches nothing.
- Same for blue/black/white/green/colorless/multicolored/historic (~54 corpus cards
  across the color + "historic" + "multicolored" forms).

Nothing catches it: `validate.rs`'s `lint_card_subtypes` only validates a card's own
printed type-line subtypes, not `Subtype(...)` atoms embedded inside ability/event
predicates. So these cards graduate with a semantically wrong filter rather than
declining (which would correctly leave them for a future color/qualifier production).

This is the "parse-to-wrong-meaning is worse than declining" case the project fixes
at source rather than grandfathering.

## Scope

- Validate a minted `Subtype(...)` atom against the real subtype catalog at parse
  time (the catalog `validate.rs` already loads): if `word` is not a known subtype,
  DECLINE (return `None`) instead of minting the atom, so the clause falls through
  to the unparsed path (or a future color/qualifier production).
- Alternatively/additionally, add the actual productions for the deferred forms:
  color-filtered spells (`a <color> spell` → a color predicate — verify a color
  filter exists in the grammar), "multicolored", "historic" (historic = an
  artifact, legendary, or Saga card). Landing these graduates the ~54 cards correctly instead
  of just declining them.
- Extend an embedded-`Subtype` catalog check to the validation layer so a bogus
  embedded subtype is caught by a test/gate, not just at authoring time.

## Verification
- `cargo test --workspace` green; `cargo clippy --all-targets -- -D warnings` clean.
- A test proving "you cast a red spell" DECLINES (or parses to a correct color filter)
  rather than emitting `Subtype("Red")`.
- Wipe-first wizards regen; no card graduates with an invalid embedded `Subtype`.
