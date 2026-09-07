---
needs: []
---
**Two crate gaps in `deckmaste_semantics_v2` found by
`semantics-v2-macro-bodies-keyword-actions`.** Terra tier; standard
constraints apply.

- `ron::param_types()` is a closed list of eight; a body needing a
  `Subtype`, `ZoneExpr`, `Quantity`, `TokenSpec`, or `Instruction` argument
  cannot declare it (Amass, Create, Meld, Vote, Search, Face a Villainous
  Choice). Per `semantics-v2.md` §12 every `SupportsMacros` kind is a
  parameter type; make the list total the way `every_supports_macros_type_is_a_kind`
  keeps the kind set total.
- A declaration named like a native constructor at its own position is
  silently shadowed (`Shuffle` registers but can never be invoked;
  `semantics-spelling-lowering.md` §6's collision diagnostic is the v1
  precedent). The reader refuses the collision at load with both names.

Then give Amass, Create, Meld, Vote, Search, and Face a Villainous Choice
their bodies from the Lean macros (their STOP text names each shape), with a
canon card each where the corpus has one, proving through `lean-check`.

## Landing record

Measured on `zllupvruyotk` (2026-09-07), the tip of this claim.

### PROVE

- **No silent loss**: no identity stops being covered by this landing — zero
  bodies were given (see DISCLOSE), so nothing that previously loaded or
  expanded stops doing so. `both_readers_accept_every_builtin_declaration`
  (the shared-contract drift test) still reports the two readers agreeing on
  every one of `plugins_v2/builtin`'s declarations.
- **Structural laws**: not applicable — no card, no Lean emission, no corpus
  command runs in this round (crate-gap work only).
- **No word-naming**: no guard added anywhere names a lexeme, construction,
  verb, noun, preposition, or card identity. The one new reader check
  (`NativeCollision`) reads a declaration's *own name* against its *own
  kind's* native dispatch set — a structural property of the declaration,
  not a hand-picked identity — exactly as `every_declaration_kind_is_
  registered` and the sibling kind-totality tests already do.

Gate (`cargo xtask gate --changed --run`):
`cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask`.
Totals: **1521 passed, 0 failed, 1 ignored, 46 suites**. `cargo fmt --all --
--check` clean (nightly-only options warn, no diff).

### DISCLOSE

**Bodies given: 0 of 6** (Amass, Create, Meld, Vote, Search, Face A
Villainous Choice all remain bodyless). Both crate gaps are real and are
each partly fixed, but neither is fixable to completion within this
ticket's authorized file scope
(`crates/deckmaste_semantics_v2/src/{ron.rs,reader.rs}` and their tests,
plus `crates/xtask/tests/plugins_v2_declarations.rs`) — see STOPs. Giving
any of the six a body that names a new param type, once written, fails to
load on the `deckmaste_construction_core` side even though it now loads on
the `deckmaste_semantics_v2` side, so I did not commit any body that cannot
actually pass the required gate.

**Gap 1 — `ron::param_types()` totality.** Delivered on the
`deckmaste_semantics_v2` side: `param_types()` now registers every
`SupportsMacros`-derived type under its own name (`GameEvent`, `Quantity`,
`ZoneExpr`, `Duration`, `Timing`, `UsageLimit`, `Instruction`, `StaticSpec`,
`TokenSpec`, `Window`, `TurnPart`, plus bare `Predicate`/`NounPhrase`
alongside the existing `Quality`/`Subject` aliases), and `Subtype`
additionally (not itself a `SupportsMacros` kind, but the type Amass's
STOP names). A new test, `every_supports_macros_type_is_a_param_type`,
holds this total the same way `every_supports_macros_type_is_a_kind` holds
`kinds()` total. **This alone does not unblock any of the six**: see STOP
(construction_core mirror), a gap this ticket's diagnosis did not
anticipate.

**Gap 2 — the native-collision refusal.** Fully delivered and wired into
`Plugin::load` (not a dead/disabled check): a declaration whose name equals
a native constructor at one of its registered kinds is refused at load,
naming both the declaration's path/name and the colliding kind
(`LoadError::NativeCollision`). Two exemptions, both necessary to avoid
breaking the *existing*, intentional builtin tree:
- **Identity** — the declaration's own body's outermost identifier equals
  its own name (`body_head(macros) == Some(name)`): reading it natively or
  through the macro produces the same value, so nothing is shadowed. This
  is the existing, tested `Subtype`/`CounterKind`/`TurnPart` self-naming
  design (`a_turn_part_declaration_may_name_its_own_constructor`), and it is
  also `Shuffle`'s own shape (`body: Shuffle(agent: Param(0))`) — so
  `Shuffle` needed **no** rename.
- **Bodyless** — a meta-macro's omitted `body` argument defaults to `()`
  (`Default(Any, ())`), which is not identifier-led and was never invocable
  for real value regardless of collision. `Exchange`, `Search`, and `Vote`
  are all bodyless today and all needed **no** rename either.
  Verified: `both_readers_accept_every_builtin_declaration` and the whole
  `plugins_v2/builtin` tree load unchanged, with zero declarations renamed.
- A genuine (non-identity, non-bodyless) collision **is** refused — proven
  by a dedicated unit test using a synthetic declaration, since no such
  declaration currently exists in `plugins_v2/builtin`.

**Additions**: 4 new unit tests (`ron::tests::
every_supports_macros_type_is_a_param_type`;
`reader::tests::a_non_identity_native_collision_is_refused`,
`an_identity_native_collision_is_exempt`,
`a_bodyless_native_collision_is_exempt`). No helper macros. **No fixture
cards added**: the brief asks for at least three exercising "a
representative sample of your family's macros," but none of the six became
invocable (STOPs below), so there is nothing new to exercise — adding a
card that only re-exercises an *already*-bodied, unrelated keyword action
would not be a representative sample of this ticket's own work, and I did
not fabricate a proof to manufacture one. Disclosed as a deviation from the
brief's letter, not silently dropped.

**Glossary gaps**: none.

### STOPs

**STOP — `deckmaste_construction_core::macro_def::ParameterType` mirrors
`deckmaste_semantics_v2::ron::param_types()`'s exact closed eight-name list
independently, and is not in this ticket's authorized scope.**
`docs/decisions/semantics-v2.md` §11: "The file is the shared contract;
neither crate depends on the other for it, and an xtask drift test loads
every declaration both ways" — which means a param type is usable only once
*both* readers accept it. `deckmaste_construction_core`'s own
`ParameterType` enum (`crates/deckmaste_construction_core/src/macro_def.rs`)
is the english_v2/typed side's independent copy of the same eight names
(`Ability, Amount, Condition, Cost, Power, Quality, Subject, Toughness`),
constructed by `ParameterType::new`, with no `Subtype`/`TokenSpec`/
`Quantity`/`Instruction`/`String` variant. Any declaration naming one of
those in `params:` fails `both_readers_accept_every_builtin_declaration`
with `UnknownParameterType`, from the construction_core side alone, even
though `deckmaste_semantics_v2::ron::param_types()` now accepts it. This
crate is outside the file scope this ticket's dispatch explicitly lifted
the "do not edit `crates/`" rule for
(`crates/deckmaste_semantics_v2/src/{ron.rs,reader.rs}` and their tests,
plus the one xtask test file) — I did not touch it. Left bodyless, each
with an updated STOP naming exactly this:
- **Amass** [CR#701.47a] — needs `Subtype` (the amassed subtype is spliced
  into a `Vec<Subtype>` position twice; a raw `String` cannot fill it).
- **Create** [CR#701.7a] — needs `TokenSpec` (chosen over
  `CharacteristicBundle`: `CharacteristicBundle` derives no
  `SupportsMacros`, so it is not itself a macro-position kind, while
  `TokenSpec` is — the totality fix's natural target).
- **Meld** [CR#701.42a] — needs `String` (the melded permanent's name).
  `String` is already registered on the `deckmaste_semantics_v2` side (it
  ships from `macro_ron` itself, not from this ticket's fix), but
  construction_core's closed eight has no `String` variant either.
- **Search** [CR#701.23a] — needs `Quantity` (Lean `searchLibraryFor`'s
  amount of cards to find). Additionally, `Search`'s declared name already
  collides with the native `Instruction::Search` variant, harmlessly today
  only because it is bodyless (gap 2's bodyless exemption); the day it gets
  a real, non-identity body it will need a rename, which conflicts with
  `crates/xtask/src/facts/lean.rs`'s `ACTION_OVERLAY` (also out of this
  ticket's scope — see the note below).
- **Face A Villainous Choice** [CR#701.55a] — needs `Instruction` (the two
  options). No dedicated Lean `semantic_macro` exists for this shape either
  (only the general `chooseModes` it would specialize), a second,
  independent reason it stays bodyless; naming it so a future claimant does
  not have to rediscover it.
- **Vote** [CR#701.38a] — needs `Ballot` and `Disclosure`, neither a
  `SupportsMacros` kind, so gap 1's totality fix does not reach them even in
  principle; registering them is a separate ticket's, on either crate.
  `Vote` has the same latent Search-style rename-vs-`facts/lean.rs`
  conflict once it is bodied.

**Note (not a STOP, a discovered hazard for the routed follow-up ticket):**
`crates/xtask/src/facts/lean.rs`'s `action_rows` matches each `KeywordAction`
declaration's *own name* against `ACTION_OVERLAY` by `normalize(label) ==
normalize(name)` — i.e. it assumes a keyword action's declared `name:` field
doubles as its CR/Lean deed label everywhere. I verified this by
experimentally renaming `Exchange`/`Search`/`Vote` to
`ExchangeAction`/`SearchAction`/`VoteAction` (to resolve gap 2's collision
the "obvious" way, before finding the bodyless exemption): `cargo test -p
xtask --lib facts::` went from 25 passed to 9 failed, all `"<name>Action:
keyword action has no checker-column overlay"`. I reverted the renames
rather than also editing `crates/xtask/src/facts.rs`/`facts/lean.rs`
(out of scope). A future ticket giving `Search`/`Vote`/`Exchange` a real
body will hit this the moment it needs to rename them past gap 2's
exemptions, and should route the `ACTION_OVERLAY` fix (or a per-declaration
`kinds:` override in `plugins_v2/builtin/macros/meta/KeywordAction.ron` so a
colliding keyword action need not register at the shadowed `Instruction`
position at all) alongside it.

### REPORT

- **Assurance counts**: restored 0; re-spelled 0; ignored-with-blocker 0;
  **added 4** (the unit tests above); removed 0.
- **Counts**: 6 declarations in this ticket's family; **0 bodied**, 6
  bodyless with an updated STOP (5 rewritten — Amass, Create, Meld, Face A
  Villainous Choice, Vote — plus Shuffle's exemption note, which changed no
  behavior); 0 fixture cards; 0 helper macros.
- **Citations**: 2 newly registered by `cargo xtask cite bless`
  [CR#701.42a,701.55a], both eyeballed against their citing claim (Meld,
  Face A Villainous Choice) via `jj diff --git | cargo xtask cite audit
  --diff`, which audited **6 citation sites** total (the other 4 —
  `ron.rs`'s `[CR#701.47a]` and `Create.ron`'s `[CR#701.7a,111.3,111.4]` —
  were already locked). `cargo xtask cite check --list-noncompliant` empty;
  `cargo xtask cite check` reports 15203 citations, **0 stale**.
- **Performance advisory**: not applicable — no corpus or `lean-check`
  command runs in this round (no card fixtures).

### Ledger

Routed to a follow-up ticket (not yet minted — the reviewer's call at
integrate, per this ticket's own precedent of leaving STOP routing to
review): extend `deckmaste_construction_core::macro_def::ParameterType` to
mirror `deckmaste_semantics_v2::ron::param_types()`'s totality (gap 1's
other half), and resolve the `Search`/`Vote`/`Exchange` rename-vs-
`ACTION_OVERLAY` conflict (gap 2's residue) — then Amass, Create, Meld,
Search, Face A Villainous Choice can actually be bodied, and Vote once
`Ballot`/`Disclosure` are separately registered on both sides.
