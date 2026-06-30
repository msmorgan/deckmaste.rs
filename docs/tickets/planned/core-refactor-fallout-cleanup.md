---
needs: []
---
Low-priority fallout from the core-verb-patient-cardinality refactor — fragility
smells and stale docs surfaced by a post-landing review. None is a correctness
bug (the latent `Each`-batch bug and the cheap exhaustiveness/doc wins were fixed
under `core-each-batch-workitems`; the `frame.those` field split and renderer
`Each` generalization live in `core-many-binder-group-move`). Group these into
one cleanup pass.

## Code smells
- **`GetTargets` hard `assert_eq!`** — `crates/deckmaste_engine/src/resolve.rs`
  `eval_selection_set` panics the engine if a card authors `GetTargets(n>0)`
  (`assert_eq!(*spec, 0, …)`). Plugin-authored data should not be able to abort
  the engine. Replace with a structured "multi target-spec announce not yet
  wired" error. Currently unreachable (only Arc Lightning, `GetTargets(0)`).
- **Orphaned `selection_object`** — `crates/deckmaste_cards/src/render/fragment.rs`
  is kept behind a refactor-added `#[allow(dead_code)]` "per the migration plan."
  It's the only 0-caller fn in the touched crates. Delete it unless a concrete
  follow-up stage needs it (and if so, ticket that stage).
- **Duplicated `peel` helpers** — `resolve.rs` (the `Effect::Each` and
  `Effect::With` arms) and `activate.rs:411` each define a local `fn peel`
  (Binder/Effect `Expanded`-unwrapping), the last triggering clippy
  `items_after_statements`. Extract one shared peel helper.
- **`binder_phrase` hardcodes the article "a"** — `render/effect.rs` (~175); now
  the single chokepoint for all `With` one-binder phrasing, so it can't say
  "an"/"the". Add article agreement.
- **`eval_reference_set` one-element wrapper** — `resolve.rs` returns
  `vec![self.eval_reference(…)]` so ~12 verb arms keep group-shaped
  `.into_iter().map(…)` bodies though each acts on exactly one object. Harmless
  (self-documented as transitional), but a later simplification to single-
  `ObjectId` arms would drop the vestigial group-shaping. No correctness issue.

## Stale comments / docs (refactor invalidated these)
- `crates/deckmaste_engine/src/cast.rs:388` — doc-comment describes a
  `Selection::Choose`-inside-a-verb path that no longer exists; rewrite to the
  `With(ChooseOne/Choose)` binder path.
- `crates/deckmaste_engine/tests/replace_registry.rs:221` — comment says "Exile
  is a `PlayerAction`," but the code uses `Move(This, Destination::Zone(Exile))`;
  `PlayerAction::Exile` was dropped. Reword.
- `docs/conformance.md:50` — maps universal "each" to `Selection::Each(Filter)`
  (gone; now `Selection::Filter` + `Effect::Each`). Update the type column.
- `crates/deckmaste_engine/src/resolve.rs` `eval_reference` doc — parenthetical
  "the bound-object resolver `Selection::Ref` funnels through" references a
  removed variant; drop it.
- `crates/macro_ron/src/support.rs:35` and `crates/macro_ron/src/tests.rs:1573` —
  live doc examples cite `Selection::Ref`; swap for a still-existing example.
- **Re-ground stale planned tickets** before they're claimed:
  `engine-choose-foreign-chooser.md` (talks `Selection::Choose` /
  `unresolved_choice` / `PendingChoice`), `core-with-rebindable-that.md:50`
  (migration already done), `idris-naming-reconciliation.md:13` (its `Each`↔
  `ForEach` note now runs opposite the Rust rename).
