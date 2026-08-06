---
needs: []
---
**Engine: there is no runtime library-search primitive.**

`Binder::Search` / `Binder::SearchOne` carry the Idris constructors' resolver
inputs, but nothing executes them — `resolve/effect.rs` and `activate.rs` both
surface a labeled seam rather than resolve one.

`parse-tutor-search` (done) built the parser half: it recognizes the
search-reveal-move-shuffle sentence chain and lowers it onto these binders. It
assumed the engine-side primitive existed. It does not, so a tutor parses
correctly and then panics on resolution.

Build the search step per [CR#701.23]:

- Look at all cards in the searched zone, even a hidden one ([CR#701.23a]).
- Fail-to-find: a stated-quality search never compels finding a match that is
  present ([CR#701.23b]); an undefined quality lets the player search but find
  nothing ([CR#701.23c]); a bare quantity must find that many, or as many as
  exist ([CR#701.23d]).
- Reveal only when the effect says to ([CR#701.23e]).
- Shuffle afterwards ([CR#701.24a]), including when nothing was found — the
  shuffle is printed text on the card, an unconditional instruction in the
  effect's sequence ([CR#608.2c]), not a rules-level guarantee. (This bullet
  originally cited [CR#701.24d]; that rule governs shuffling a set of objects
  *into* a library when the set is empty, which is not what a tutor's bare
  "then shuffle" does.)
- Bind the result as `That` — singular for `SearchOne`, a group for `Search`,
  matching the existing binder-cardinality split. The `if_none` arm runs on a
  failed find.

Resolving these through the plain `ChooseObjects` chooser is the tempting
shortcut and is wrong: it would silently skip the reveal and the shuffle, so a
tutor would leave the library in a known order.

Out of scope unless a corpus card drives it: simultaneous multi-player search
ordering ([CR#701.23i]) and searching outside the game ([CR#701.23j]).

Effort: **M**.

## Done

`resolve/effect.rs`'s `binder_choice`/`resolve_binder` now surface `Search`/
`SearchOne` as a `ChooseObjects` decision over `whose`'s `from` zone(s)
(zone+owner filtered directly, not `candidates_with`'s whole-game scan). The
fail-to-find floor: `Predicate::Kind(Card)`/`Any` (no stated quality) forces
`min = choice_bounds(...).lo` ([CR#701.23d]); any other filter floors `min` at
0 ([CR#701.23b]) — [CR#701.23c]'s "can't find any" falls out of the same
mechanism when 0 candidates match, so it needed no separate arm. `OneShotEffect::With`
runs `if_none` instead of the body (frame unchanged, no `That` bound) when the
resolved group is empty; reveal/shuffle stay body-level actions (already built
by `parse-tutor-search`), not new binder behavior — confirmed via
`parse_search_library` that the corpus only ever emits self-search
`SearchOne`/`Library`/no-`if_none`, so [CR#701.23i,701.23j] stay out of scope
as the ticket allowed. `activate.rs`'s cost-side seam is also resolved: a
search-binder cost is unconditionally payable ([CR#701.23b..701.23d] — fail-to-find
is never a partial-payment problem, unlike `Choose`), no corpus card exercises
it yet.

Files: `crates/deckmaste_engine/src/resolve/effect.rs`,
`crates/deckmaste_engine/src/activate.rs`,
`crates/deckmaste_core/src/binder.rs`, `crates/deckmaste_semantics/src/binder.rs`
(doc updates only).

Tests (`deckmaste_engine`): `search_one_semantic_land_tutor_finds_moves_and_shuffles`,
`search_one_reveal_step_in_body_reveals_the_found_card`,
`search_one_bare_quantity_compels_a_find_when_present`,
`search_one_bare_quantity_finds_none_from_an_empty_library`,
`search_one_stated_quality_may_decline_a_present_match`,
`search_if_none_runs_instead_of_body_on_a_failed_find`,
`search_many_binds_the_found_group_as_they` (all in `resolve::effect::tests`),
`search_cost_binder_is_always_payable_even_over_an_empty_library`
(`activate::tests`). `cargo test --workspace`: 0 failures.

Left out: `if_none`'s own shuffle guarantee is the effect author's
responsibility (spell it inside `if_none` if wanted) — the engine doesn't
auto-shuffle on that path since no corpus card populates `if_none` yet;
flagged, not built, since inventing the shape would be speculative.
