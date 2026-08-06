---
needs: []
---
**A newtype-tuple variant whose one field is itself a named struct now
scaffolds field-spliced**, so the 16 identity macros that could not read
their variant's canonical spelling (`Activated(cost: …)`, `May(who: …)`)
read it, and the coverage gate asserts unconditional coverage again.

## What shipped

1. **`macro_ron::MacroFields`** (`support.rs`, derived by
   `#[derive(MacroFields)]`): a named struct's own field list — RON names
   (serde `rename`, raw-ident `r#as` → `as`) paired with a `ParamDefault`.
   A field is `Implicit` when it forwards a `#[serde(default …)]` OR its
   type is `Option<…>` — serde fills a missing `Option` field with `None`
   without any attribute, and a scaffold that made such a field required
   would break the short spelling canon uses (`CardFace::power` and kin).
   The same rule now applies to struct-variant fields.
2. **`#[macro_ron(spliced)]`** (`macro_ron_derive`): on a newtype variant,
   reports `VariantSignature::Named(<Payload as MacroFields>::FIELDS)`
   instead of one opaque positional slot. `Box`/`Arc`/`Rc` are seen
   through. Read and write are untouched — the marker changes only the
   SHAPE the signature reports, which is what the scaffold generator
   mirrors, so no separate `Kind` channel was needed.
3. **The grammar** (`deckmaste_semantics`): every newtype-over-named-struct
   variant is marked — the 16 hazard rows, `Card::Normal`, and five the
   hand-listed hazard table had MISSED: `OneShotEffect::Delayed`,
   `Reflexive`, `Targeted`, `StaticEffect::CostOption` and
   `EnterRider::AsCopy`. Two of those were broken with real corpus exposure
   (`Delayed(event: …)` 3 spellings, `CostOption(components: …)` 4); the
   other three are dispatch-free or native-calculus rows whose signature is
   now simply honest.
4. **The marker set is mechanically enforced**, not remembered
   (`xtask::authoring::field_splice::every_newtype_over_a_crate_struct_is_marked_spliced`):
   it parses the semantics sources the way `cargo xtask map enums` does and
   asserts that every newtype variant of a `SupportsMacros` enum whose
   payload — peeled of `Arc`/`Box`/`Rc` — names a named struct declared in
   the crate carries the marker. The derive cannot answer this (a proc macro
   sees only its own item's tokens), and a hand-maintained marker set would
   re-create the very failure that produced this ticket: the guard found
   `CostOption` and `AsCopy`, which two independent hand surveys had missed.
   `#[derive(MacroFields)]` additionally hard-errors on the serde settings a
   derived field list cannot model (`rename_all`, container `default`,
   `skip`, `flatten`, per-direction `rename(...)`), so a future silent
   mismatch is a compile error instead.
5. **Scaffolds regenerated** (`plugins/builtin/macros/identity/`): 21 files.
   `May` split into `May.ron` (`OneShotEffect`, now named) and
   `May~StaticEffect.ron` (`Deontic::May`, still positional); `With`'s two
   clusters MERGED, since `CostComponent::With` and the spliced
   `OneShotEffect::With` now agree. `Normal.ron` is generated output rather
   than hand-authored — its field list comes from `CardFace` itself, and
   the generator reproduced the hand-written one exactly.
6. **The exception is gone**: `FIELD_SPLICE_HAZARD`, its rot guard,
   `is_single_positional`, and the `except` clause on
   `every_reachable_row_is_covered` are deleted, not emptied.
   `xtask::authoring`'s `unscaffoldable_kinds` went with them — its only
   remaining entry was `Card`, whose defs the legacy oracle already
   tolerates (`UnknownKind` on insert), and it was what kept `Normal.ron`
   outside the generator's reach.

## Gate

`xtask::authoring::restricted_read::every_field_spliced_row_round_trips_under_restriction`
reads every one of these rows at its field-spliced spelling through
`MacroSet::read_str_restricted` and asserts the value equals a native read
of the same text — per row, sources lifted from the committed corpus where
the row occurs there. Row existence is not the gate; the behavioural read is.

Related: [[macro-author-surface]] (the restriction flip this unblocks),
[[macro-ron-optional-param-elision]] (the `Elidable` form these scaffolds
use for a droppable payload field).
