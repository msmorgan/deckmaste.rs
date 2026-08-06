---
needs: []
---
**Give `macro_ron` a param form for "forward this argument if the call gives
it, omit it entirely from the body if not."** Nothing today expresses that:
`Default(Type, expr)` FILLS an omitted argument with a known expression — a
different thing — and is unavailable in two of the three places this gap
actually shows up: it's rejected on a positional param list outright
(`crates/macro_ron/src/set.rs:313`, "defaults are named-only"), and a named
param's default is, in practice, always `ParamDefault::Implicit` — no
captured expression at all, since `#[macro_ron(default = ...)]` is rejected
on struct-variant fields at compile time
(`crates/macro_ron_derive/src/input.rs:294-304`). Even where a positional
default DOES carry captured text (`ParamDefault::Expr`), that text is Rust
source for the derive's own codegen, not RON — splicing it into a def file
would be wrong on its own terms.

## Why this blocks `macro-author-surface`

The identity-macro scaffold generator (`cargo xtask scaffold-identity`,
`crates/xtask/src/authoring.rs`) mirrors each reachable variant's real
signature so canon re-parses byte-identically once restricted reading is
switched on (`docs/tickets/critical/macro-author-surface.md`). A variant
whose signature has a defaulted param that can't safely be dropped —
`blocked_on_elision` in `authoring.rs` — gets NO scaffold at all rather than
one that drops the param, because dropping it breaks every LONG-FORM
spelling of that variant (the ones that explicitly supply the value the
default would otherwise fill): the scaffold's own declared arity would no
longer match, so `read_args` rejects the call
(`crates/macro_ron/src/expand.rs:1461`, arity/key checks) the moment
restriction routes it through the macro instead of native reading. Today,
before restriction flips, native reads still shadow the macro
(`native_variant_ok`), so nothing catches this — that's exactly why the
first attempt at this generator shipped scaffolds with the bug before it was
caught in review.

**`macro-author-surface`'s restriction flip cannot cover the affected rows
until this lands.** 40 of 418 `NeedsIdentityMacro` rows are currently
excluded rather than scaffolded (36 named, 4 positional — `Action`/
`OneShotEffect` `Create`/`MoveGroup`/`Reveal`/`Cast`/`Move`, most of
`EventFilter`'s event-shaped variants, `Selection`'s library-position
variants, `StaticEffect::TriggerMultiplier`, among others — see
`crates/xtask/src/authoring.rs`'s `blocked_on_elision`, which excludes
structurally rather than by name, so this list moves as the grammar does).

**Measured exposure** (2026-08-06 corpus): 1,916 of 30,506
`plugins/wizards/cards` files and 10 of 79 `plugins/canon/cards` files spell
one of these names with no macro at any kind — `Cast(` 351, `Move(` 932,
`Create(` 747, `Reveal(` 91, plus `ZoneChange`/`Drawn`/`Act`/`LifeGained`/
`LifeLost`/`CounterPlaced`/`CounterRemoved`/`MoveGroup`/`TriggerMultiplier`.
Every one of those files fails to load the moment restriction reaches them
without this fix (or without hand-authoring a non-scaffold workaround per
row, which the generator deliberately does not attempt — see `authoring.rs`
module doc, "BLOCKED: optional-param elision").

## Proposed form, sized

1. **Grammar** (`crates/macro_ron/src/param.rs`): a third `ParamType` shape
   alongside `Default(Type, expr)` — no fill expression, because there isn't
   one to give. Must be legal in a POSITIONAL param list too, which is new:
   today `Default(...)` is rejected there outright at
   `crates/macro_ron/src/set.rs:313`'s `Params` parsing.
2. **Call-site reading** (`crates/macro_ron/src/expand.rs::read_args`,
   `:1461`): `Params::Positional`'s arity check (`args.len() != types.len()`)
   must accept any count in `[required_count, types.len()]`, with the
   elidable params forming a TRAILING run — mirroring the derive's own
   contiguous-suffix rule for the native type's own trailing defaults — so a
   partial call fills the first N slots and leaves the rest genuinely
   absent, not filled with anything. `Params::Named`'s `missing` handling
   already tolerates an omitted key when `default.is_some()`; extend the
   tolerance to the new elidable marker, and `fill_defaults`
   (`crates/macro_ron/src/expand.rs:1394`) must NOT synthesize a value for
   an elided-and-omitted key — it needs a third outcome ("absent"), not just
   "filled with something."
3. **Body-side key omission — the part not yet fully traced, and the real
   size of this ticket.** Today a body's struct-shaped RON (`Cast(who:
   Param(who), what: Param(what))`) hands EVERY key literally present in the
   body text to the destination type's `MapAccess`, unconditionally. Making
   an elided `Param(who)` cause the ENTIRE `who: ...` entry to vanish from
   what the destination sees requires whatever visitor streams a body's map
   to the destination's `Deserialize` (downstream of `EnumIntercept`'s
   struct-variant handling, not `read_args`, which only handles the CALL
   side — the exact site wants locating by whoever picks this up) to peek
   each value before yielding its key, and skip the pair on an elided value.
   This is a streaming-omission capability the reader doesn't have anywhere
   today. Every consumer that currently assumes a body's map entries
   correspond 1:1 to source text — `validate_arg`, the cycle checker — needs
   auditing against "fewer entries than the source text" becoming possible.

**Blast radius**: `crates/macro_ron/src/param.rs`, `set.rs` (`Params`
parsing, `all_defaulted`), `expand.rs` (`read_args`, `fill_defaults`, plus
the body-map streaming path in (3), not yet located). Not a leaf change —
(3) is a structural addition to the expansion engine's map-streaming
machinery, not a tweak to an existing branch. Estimate a multi-day
design-and-implementation effort, not a same-session fix.

## Also fix while here

`crates/macro_ron/src/support.rs`'s doc for `ParamDefault::Implicit` claimed
"a scaffold omits the parameter and lets serde fill it" as if that were
always safe — it silently breaks the long-form spelling for exactly the
reason above. Already corrected (2026-08-06) to point here instead of
re-asserting the false claim; keep that pointer accurate if this ticket's
slug ever changes.

## Acceptance

A named OR positional param can be declared elidable; a call may omit it
(short form, native default fills the field) or supply it (long form,
forwarded); `blocked_on_elision` in `crates/xtask/src/authoring.rs` goes to
zero real rows against the then-current registry, and the 40 currently
excluded scaffolds get generated and pass the same restricted-read round-trip
tests `authoring.rs`'s `restricted_read` module already exercises for the
unblocked shapes, in both short and long spelling. Standard constraints
apply (fmt, clippy, `cargo test --workspace`, CR-citation gates untouched by
this ticket).

Related: [[macro-author-surface]] (the consumer this unblocks — its
restriction flip cannot cover the affected rows without this),
[[core-remove-default-args]] (an alternative, more invasive strategy for a
subset of the same underlying problem: removing the defaults from the
grammar entirely rather than teaching the macro layer to forward them —
worth comparing before implementing this, in case it changes the actual
row count needing elision support).
