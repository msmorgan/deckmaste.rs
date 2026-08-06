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
   variant is marked — the 16 hazard rows plus `OneShotEffect::Delayed`,
   `Reflexive` and `Targeted`, which the hand-listed hazard table had
   MISSED (`Delayed(event: …)` is spelled 3 times in the committed corpus
   and was broken identically), and `Card::Normal`.
4. **Scaffolds regenerated** (`plugins/builtin/macros/identity/`): 20 files.
   `May` split into `May.ron` (`OneShotEffect`, now named) and
   `May~StaticEffect.ron` (`Deontic::May`, still positional); `With`'s two
   clusters MERGED, since `CostComponent::With` and the spliced
   `OneShotEffect::With` now agree. `Normal.ron` is generated output rather
   than hand-authored — its field list comes from `CardFace` itself, and
   the generator reproduced the hand-written one exactly.
5. **The exception is gone**: `FIELD_SPLICE_HAZARD`, its rot guard,
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
