---
needs: []
---
**`macro_ron` has a param form for "forward this argument if the call gives
it, omit it entirely from the body if not."** `Elidable(Type)`, legal on
named and positional signatures alike, alongside the existing
`Default(Type, expr)` (which FILLS an omitted argument with a known
expression — a different thing, and unavailable for the shapes this gap
actually covered: rejected on a positional list, and a named param's default
is in practice always `ParamDefault::Implicit`, no captured expression at
all).

## What shipped

1. **Grammar** (`crates/macro_ron/src/param.rs`): `ParamType.elidable`, read
   from the `Elidable(Type)` spelling. `Elidable` joins `Default` as a
   reserved param-type name. It does not nest and does not combine with
   `Default`; it composes with a binder contract.
2. **Signature checks** (`crates/macro_ron/src/set.rs`): elidable positional
   params must be a contiguous TRAILING run (`InsertError::ElidableNotTrailing`),
   mirroring the derive's own trailing-default rule for the native tuple
   variants these signatures shadow — a call supplies a prefix. A default
   expression may not reference an elidable param, which might not be there.
   `all_defaulted` deliberately does NOT count elidable params: the native
   struct-variant grammar has no bare spelling either (`Drawn` is a parse
   error where `Drawn()` reads), so granting one would invent an author form
   nothing round-trips back into.
3. **Call side** (`expand.rs::read_args`): a positional call's arity may be
   anything in `[required, types.len()]`; a named call may omit an elidable
   key without `fill_defaults` synthesizing anything for it. The omitted
   params are recorded on the expansion `Frame` as `elided` — they have no
   argument text at all, which is the third outcome the old
   supplied/filled split had no room for.
4. **Body side** — the piece this ticket could not originally place. It is
   NOT in the map-streaming visitor: a macro body is spliced and re-read as
   SOURCE, so "the destination never sees this key" can only mean "the key
   isn't in the source the destination reads". `expand.rs::elide_body` runs
   at the two body re-read sites (`via_capture`'s macro branch and
   `EnumIntercept::visit_enum`, before the `synthesize_expanded` wrapper is
   built), walking the body with ron's own value spans and cutting every
   entry whose value is an omitted param's hole — keyed entries in any
   position, ordered ones only as a trailing run. Because it is span-driven,
   a `Param` inside a string literal is never mistaken for one; because it
   returns the body unchanged when nothing was omitted, no definition
   predating the form expands differently by a byte.

`crates/xtask/src/authoring.rs`'s `blocked_on_elision` is retired: the 40
rows it withheld (36 named, 4 positional) now scaffold, as 35 files.

## Acceptance, met

Both spellings of the same def read, and each produces what a native read of
the same text produces — `restricted_read`'s
`an_elidable_params_spellings_all_round_trip_under_restriction` (named,
`EventFilter::Cast`) and `..._positional_params_arities_...`
(`Action::Cast`'s trailing alternative-cost slot), against the real registry
with every committed scaffold loaded. `elision_gap` now asserts the inverse
of what it used to: every droppable-default row IS scaffolded, with an
`Elidable(Any)` param.

Related: [[macro-author-surface]] (the consumer this unblocked),
[[core-remove-default-args]] (the alternative strategy — removing the
defaults from the grammar entirely — which this makes unnecessary for the
macro layer's purposes, though it may still be wanted on its own merits).
