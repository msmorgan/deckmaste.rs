---
needs: [plugin-repoint, identity-scaffolds-field-splice]
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

## Open, to settle before restriction is switched on

- **Argument validation bypasses restriction.** `validate_arg` reads captured
  args through `MacroSet` with no `Ctx`, so a banned spelling passes validation
  and fails later at the `param` re-read. The ban still fires; the error is
  just worse-placed.
- **Embed pre-scan route.** §4 says suppression must cover the untagged-embed
  consult. Implemented as written, which makes a banned host variant fall
  through to the embedded type — that finds an identity macro registered at the
  embedded kind, but yields an embed-side error when no macro exists anywhere.
  Leaving `variants.contains` unsuppressed would instead keep it at the host,
  where the "not author vocabulary" error is produced. Unobservable until the
  entry flips; no fixture pins it yet, deliberately.
- **16 identity scaffolds can't read their own canon spelling** —
  [[identity-scaffolds-field-splice]]. A newtype-tuple variant whose one
  field is itself a named struct (`Ability::Activated`/`Triggered`/`Spell`,
  13 `OneShotEffect` variants) is spelled field-spliced by canon, but the
  generic scaffold shape reads a single positional slot as one raw value,
  not a field map — confirmed broken under restriction, with real corpus
  exposure (`OneShotEffect::May` alone: 273 spellings). The coverage gate
  (Task 6) tracks these explicitly (`FIELD_SPLICE_HAZARD`, a temporary
  `except` clause on `every_reachable_row_is_covered`) rather than
  certifying them; this ticket is what removes the exception. Blocking: the
  flip is not safe for these rows until it lands.

## Gates

Standard constraints apply. Canon + workspace suites green; idris-check no
regressions; zero behavior change to expansion of existing canon; coverage
gate green; negative fixtures in place.
