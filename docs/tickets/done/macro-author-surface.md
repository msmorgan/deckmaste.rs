---
needs: [plugin-repoint]
---
**Make the macro layer the only author-facing vocabulary in card/token
containers: the parse-position ban, identity macros, and straggler
registration — Stage 2 of the semantics program.** Design (the settled
record; supersedes this ticket's pre-program draft):
`docs/decisions/semantics-spelling-lowering.md`
(§4-§5). Lands in `macro_ron` (the ban mechanism: both native-candidacy
consults and the restriction bit live in `expand.rs`) and
`deckmaste_semantics` (identity macros, the compiled registry, straggler
registration).

## Scope

- **The ban**: restricted reads at the card/token (and graduation)
  entries, by native-variant-candidacy SUPPRESSION at the single ident
  dispatch point — not match-then-reject — so mirrored spellings route to
  identity macros, bare-nullary sugar keeps parsing, and no value tree is
  ever built for a banned spelling. The restriction bit follows textual
  provenance (recorded at argument capture, restored on re-read); body
  re-reads are exempt however reached. Restriction is per container:
  cards/tokens restricted; `rules/` tables and macro bodies free.
- **Identity macros**: one def per semantics-reachable variant under the
  variant's own name (mirror-by-default — canon re-parses byte-
  identically). Reachable = variants of registry kinds reachable from the
  restricted-container root types via fields/flatten/embed, excluding
  name-erasing loader-tag kinds (spec §5). Scaffolded once by the
  generator, then HAND-OWNED; a coverage gate (variant ↔ def, arities
  verified) replaces wipe-first regeneration. Registration kinds computed
  from the flatten graph; scaffolds mirror arities AND existing
  constructor defaults (byte-identical canon re-parse is the invariant —
  default REMOVAL stays `core-remove-default-args`). Coverage is keyed
  per `(defining type, variant, dispatch kind, signature)` row. At
  `remembers_expansion` kinds the wrappers add invocation provenance to
  previously-bare spellings — "zero behavior change" is measured at
  stored-byte round-trip and lowered core (spec §5).
- **Closed reachability inventory** (spec §4): every reachable
  `(kind, variant)` pair classified identity-covered / native-calculus /
  native-atom (plain enums outside the registry — `Cmp`, phase/step
  enums, `FaceLayout`; canon spells `Eq` and `Beginning(Upkeep)` today);
  suppression applies at registered kinds only, and nothing may be
  unclassified. The identity exemption's trust channel is the COMPILED
  registry (spec §6) — generated Rust table, marker never serialized.
- Suppression must cover BOTH native-candidacy consults: the
  `EnumIntercept` branch AND the untagged-embed pre-scan's own
  `variants.contains` (spec §4).
- **Stragglers**: register `Color` and `Supertype` (and the `Card` root
  scaffold) so the ban is uniform; the closed `Type` enum and counter
  names already conform.
- Negative fixtures: a bare-primitive card fails with a useful error; the
  per-container restriction matrix gets a fixture per container kind.

## Prerequisite found while landing the ban

`check_cycles` rejected identity macros outright. It resolves a body's leading
invoked name and flags `name == def.name` as a self-cycle, but `Kind` carried
no variant list, so it could not tell a real self-invocation from an identity
macro whose body spells its own native variant (`Any` at kind `Filter`, body
`Any`) — the whole of §5's shape. `Kind` now carries the dispatch set, supplied
by the derive from `SupportsMacros::ALL_VARIANTS`, and a body ident naming a
native variant at one of the def's kinds is no longer an edge. §6's collision
diagnostic wants that same data.

## Second prerequisite: variant signatures are not exposed

The reachability inventory is computed from `Kind::variants()`
(`SupportsMacros::ALL_VARIANTS`), which yields 446 rows — 19 synthesized
`Expanded` provenance and 4 structural-calculus rows come off, leaving **423
identity macros to scaffold**.

Scaffolding them needs each variant's arity and shape, and §5 keys coverage per
`(defining type, variant, dispatch kind, SIGNATURE)`. Nothing exposes
signatures today. The derive has the data (`Shape::{Unit, Newtype, Tuple,
Struct}` over `Field { ident, ty, default }` in `macro_ron_derive/src/input.rs`)
but emits none of it.

The complication is that a dispatch set is TRANSITIVE: many of the 423 rows are
names inherited from a flattened or embedded type, whose signature lives on that
type rather than the host. So signature emission needs the same compile-time
concatenation `ALL_VARIANTS` gets through `concat_variants` — which today is a
`const fn` over `&'static [&'static str]` and would need a const-constructible
`VariantSignature` to generalize.

Ordering consequence: signature emission is prerequisite to BOTH the scaffold
generator and the compiled registry, so it lands before either.

## Settled at the flip

- **Argument validation bypasses restriction** — FIXED. `Validator` now takes
  the argument text's own restriction and `validate_arg` passes it, so a
  captured argument is validated exactly as the later `param` re-read will read
  it. Observed in practice: the misplaced error blamed the macro body for a
  spelling the card wrote, which is the wrong thing to hand an author. Fixtures:
  `a_banned_argument_is_blamed_on_the_argument` and
  `an_argument_written_in_a_body_validates_free`.
- **Embed pre-scan route** — SUPPRESSION KEPT, because §4's NOTE is normative:
  suppression must cover BOTH native-candidacy consults. That, and not a
  mechanism, is the reason.

  The mechanism claim that looks obvious here is FALSE, and it is worth writing
  down because it is the natural thing to assume. Suppressing the pre-scan is
  *not* what routes an inherited name (a bare `Green` at a `ManaSymbol`
  position) to the identity macro at its defining kind. The `variants` slice a
  type hands `deserialize_enum` is its own names plus flattened compartments'
  — never the embed payload's (`macro_ron_derive`'s `concat_lists`), precisely
  so `embeds_untagged` can reach those identifiers. So for an inherited name
  `variants.contains` is already false and the `&&` short-circuits: that route
  works identically with the consult unsuppressed.

  What suppression actually redirects is only the host's OWN (and
  flatten-inherited) names, and there it is the *worse* option: it sends a
  variant with no macro anywhere to the embedded type, so the error is reported
  embed-side instead of as the host's "not author vocabulary". Not a hole —
  `EnumIntercept` is installed unconditionally, so the unsuppressed path would
  still refuse the spelling, just with the better message. Accepted for
  uniformity: one rule at one place beats two consults with two policies. All
  three routes are pinned in `macro_ron`'s embed fixtures, the cost included
  (`restricted_embed_host_variant_with_no_macro_anywhere_errors_embed_side`).

## Found by the flip

Three defects that only the first restricted read of the corpus could show.

- **Suppression was per-KIND, the inventory is per-ROW.** `NATIVE_CALCULUS` and
  `NATIVE_ATOMS` classify rows that stay natively spellable at kinds that are
  otherwise macroable, but `native_variant_ok` only asked whether the KIND was
  registered — so `OneShotEffect::Targeted` and the five `KeywordAbility`
  intrinsics were banned with no macro allowed to cover them (a
  `KeywordAbility`-kind def named after an intrinsic is rejected by the
  classification suite). 156 corpus test failures. `Kind::natively_spellable`
  carries the carve-out into the ban, populated from the two whitelists so the
  inventory stays the single source of truth.
- **A `Param` hole at an `Option` position.** `deserialize_option` forwarded
  instead of capturing, and under `implicit_some` ron commits to `Some` for any
  input that is not literally `None` — which `Param(i)` is not. 17 wizards cards
  (`Phyrexian(White, None)`) failed once the identity macro stood where the
  native variant had. Latent in `macro_ron` since holes existed.
- **Identity provenance is noise to a structural consumer.** Identity macros
  wrap previously-bare spellings in `Expanded(…)` at remembering kinds (§5,
  expected). `deckmaste_legacy_render` matches semantic terms structurally at
  dozens of NESTED positions while also reading a real macro's remembered
  `template:`, so it needs the wrappers that carry a template and not the ones
  that carry only the value's own variant name. `Plugin::rendering_card`(`_from_str`)
  gives it that view — the restricted read runs first and gates it, then a free
  read drops the identity wrappers. It retires with the legacy renderer.

  Confined by `read_api_gate.rs::rendering_view_calls_are_confined`, a separate
  textual scan (such a call names no `read_str` and no `Card`, so the syn
  matcher cannot see it). The confinement is load-bearing, not tidiness: the two
  reads agree only while every variant-named macro is a faithful identity
  mirror, so a collision — `macro-collision-diagnostic`'s open scope — would
  make this value differ from the one the engine loads, and a consumer grading
  against it would report green on a card that does not exist.

## Corrections landed at the final review

- **The bare-numeral literal splice now reads free.** `power: 1` splices a
  `Number(1)` wrapper the READER invented; §4 scopes restriction to AUTHOR
  text, so the spliced re-read runs with `restricted: false` (frame preserved).
  An author who spells `Literal(3)` out is not numeral-led, keeps the
  restricted context, and still routes through the identity macro.

  The earlier entry here deferred this and mis-stated why it was safe to defer:
  it claimed the corpus does not spell bare numerals at `Count`. It does —
  `Draw(3)`, `Mills(Target(0), 5)`, `Up(3)`, `Generic(1)` are all canon. What
  actually kept the churn at zero is that every one of them sits inside an
  enclosing remembering-kind expansion whose `args` are echoed back verbatim,
  so the inner `Expanded` is never serialized. That is a structural accident of
  where those numerals stand, not a property, which is why this is fixed rather
  than left deferred.

  Also: deleting `plugins/builtin/macros/identity/Number.ron` would break
  `power: 1` on every card, with an error naming a token no author wrote.

- **Restriction now rides on the ARGUMENT, not the frame.** It was maintained
  past the first hop only by the invoked macro's declared-param-type check —
  and every identity macro declares `Any`, whose validator accepts anything.
  A body forwarding a card-written argument into a nested macro laundered it.
  `FrameArgs` carries a per-argument provenance bit, `forward_arg` computes it
  from what the splice pulled in, and `Frame` no longer carries one at all.
  Same for a default expression that splices a supplied argument into itself.

- **The hand-owned identity defs have a shape gate**
  (`xtask::authoring::identity_shape`): body head equals the def's own name and
  its holes match its declared params exactly, in both directions. That is what
  keeps the enforcement hole above closed — a nested-invocation identity body
  would reopen it.

## Latent, left open deliberately

One site still drops an argument's restriction:
`deserialize_newtype_struct`'s `RAW_VALUE_TOKEN` capture in
`crates/macro_ron/src/expand.rs` discards the bit `substitute_into` computes,
and its comment's rationale — that the body is read as author vocabulary or not
when it is used — does not hold for a produced definition's body, which is read
free. Unreachable today: no type in the `Card`/`Token`/`Predicate` graph that
`read_str_restricted` reads carries a `RawValue`; the only `RawValue`-bearing
types are `MacroDef`, frames, params, `deckmaste_spelling::lexicon` and
`deckmaste_migrations::todo_card`. It becomes live if a definition-producing
macro ever becomes invocable from card text — mint a ticket then, and fix the
comment before anyone trusts it.

Two further accepted limits, both documented at their sites: restriction cannot
apply inside untagged content (`deserialize_any` hands the fragment to ron
natively; not reachable from the semantics graph, which uses
`#[macro_ron(embed)]` rather than `#[serde(untagged)]`), and provenance is
tracked per argument rather than per byte (conservative — the substitution ORs
restriction, never ANDs it).

## Gates

Standard constraints apply. Canon + workspace suites green; idris-check no
regressions; zero behavior change to expansion of existing canon; coverage
gate green; negative fixtures in place.
