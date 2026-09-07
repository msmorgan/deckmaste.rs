---
needs: [semantics-v2-adr-promotion]
---
**Create `crates/deckmaste_semantics_v2`: the Lean-mirrored type universe,
the `macro_ron` kinds, and the reader for `plugins_v2`.** Design per the
promoted `semantics-v2.md`; standard constraints apply. Design-bearing:
sol or Opus tier.

- Types mirror `lean/Semantics/{Words,Events,Phrase,Triggers,Abilities,Card}.lean`
  constructor-for-constructor and field-for-field from the current model.
  `lean-constructor-collapse` is follow-up work after parity, not a prerequisite.
  Settle §3's joined-kind payload here. A drift
  test compares Rust constructor and field names against the Lean
  declarations and fails on any difference; the emitter in
  `lean-card-soundness-gate` later builds on the same mapping.
- `macro_ron` dialect, not plain serde: `SupportsMacros` on every enum a
  macro may occupy, one kind per such enum, `MacroDef<()>`-style opaque
  metadata so the crate never depends on `deckmaste_construction_core` or
  `deckmaste_spelling`. The crate depends on `macro_ron` and nothing
  deletion-bound.
- Reader for `plugins_v2/<plugin>/{macros,cards,tokens,rules}`: walks the
  tree, expands macros (macros invoke other macros), yields cards, tokens,
  and the three rules tables. Rules tables are semantics-language RON, same
  as today's `plugins/builtin/rules`.
- Move `plugins/builtin_v2` to `plugins_v2/builtin`, leave a symlink at the
  old path, re-point `deckmaste_construction_core::macro_def::read_builtin_v2`
  and `crates/xtask/src/facts.rs`. An xtask drift test loads every
  declaration both ways (english_v2 metadata, semantics_v2 body).
- Read `Check/` for the reads lowering will need (binding resolution, kind
  projection) and expose them as plain functions; no refusals, no
  validation.

## Landing record

Change id `quqxmoskkops` (tip of the `semantics-v2-crate` feature line).
The English-v2 coverage lock is untouched — nothing in this landing reads or
writes the corpus, so its PROVE/DISCLOSE corpus tiers are not applicable and
the tiers below are the adapted form the brief asks for.

### Gate

```
cargo xtask gate --changed --from semantics-v2-crate --run
cargo test -p macro_ron -p deckmaste_construction_core -p deckmaste_construction \
  -p deckmaste_english_v2 -p deckmaste_semantics -p deckmaste_lowering \
  -p deckmaste_plugin -p deckmaste_engine -p deckmaste_legacy_render \
  -p deckmaste_migrations -p deckmaste_noncanon -p deckmaste_semantics_v2 \
  -p deckmaste_spelling -p deckmaste_tui -p deckmaste -p xtask
```

Green: 103 suites, 4733 passed, 0 failed, 6 pre-existing ignored. The closure
is that wide because `macro_ron` changed (see STOP 1); the path move alone
would have derived `construction_core`, `english_v2`, `xtask`.

Also green: `cargo fmt --all`; `cargo clippy -p deckmaste_semantics_v2 -p xtask
-p macro_ron --all-targets -- -D warnings`; `cargo xtask facts check` (both
generated files up to date); `cargo xtask cite check --list-noncompliant` (0)
and `cargo xtask cite check` (15084 citations, 0 stale). Every citation this
landing adds was read against its rule text through
`jj diff --git | cargo xtask cite audit --diff` — 72 sites audited, no
right-number-wrong-topic. `lean/scripts/build` was not run: no Lean changed.

### PROVE

**Structural laws.** The mirror is total and exact in both directions.
`tests/lean_drift.rs` reads the six Lean files with a line-oriented scanner and
the six Rust modules with `syn`, then fails on any declaration, constructor,
field, or field ORDER either side has and the other lacks. Field order is
checked because constructor argument order is textual order (ADR §2), so it is
meaning. A second test refuses a vacuous comparison (the scan must find ≥180
declarations and ≥800 members), and a third pins the name mapping itself.
Inventory: 204 declarations, 877 variants, 779 fields.

**No validation, no refusals.** `reads` ports only pure reads from
`lean/Semantics/Check/`; no `Refusal`, no rule function, and nothing that
returns a diagnostic was ported. The reader reports a file that will not read
and takes a file that reads as written.

**No dependency on a deletion-bound crate.** `deckmaste_semantics_v2` depends
on `macro_ron`, `ron`, `serde`, `thiserror` and nothing else (`syn` is a dev
dependency of the drift test only — see Deviations). It never mentions
`deckmaste_construction_core`, `deckmaste_spelling`, `deckmaste_semantics`,
`deckmaste_plugin`, `deckmaste_core`, `deckmaste_features` or
`deckmaste_lowering`.

**No shape invented.** Every type in `words`/`events`/`phrase`/`triggers`/
`abilities`/`card` is a Lean declaration; no variant, no convenience
constructor, no default argument, and no `Expanded` provenance variant was
added. Where a Lean doc comment carries a CR citation, the Rust doc comment
carries the same one; no rule number was written from memory.

### DISCLOSE

**Settled: §3's joined-kind payload — pair-carrying.** A binding at a joined
kind holds both halves' records side by side, so its kind is the join of the
halves' kinds and it does carry its antecedent's kind pair; the half-reading
anaphor projects one side rather than collapsing the pair. Evidence: the
workbench's own execution shape (`Payload.join` in
`lean/Semantics/Check/Words.lean`, whose `kind` is `Kind.join l.kind r.kind`
and whose zone/type/size/provenance projections read the halves in turn), plus
the census already recorded in `kind-index-joins-union-marking-is-spelling.md`
— the demonstrative echo copies its antecedent's kind pair 33 of 33 times, so a
flat join would reconstruct on every echo what the pair-carrying record keeps.
`docs/decisions/semantics-v2.md` §3 and its header are amended accordingly;
nothing else about the union family moves.

**The kind set is Lean's own answer.** `ron::kinds()` registers exactly the
enums Lean marks `attribute [semantic_expression]` — `GameEvent`, `NounPhrase`,
`Predicate`, `Amount`, `Quantity`, `ZoneExpr`, `Condition`, `Duration`,
`Timing`, `UsageLimit`, `Instruction`, `StaticSpec`, `Cost`, `Ability`,
`TokenSpec`, `Window`, `Delta` — plus `Macro` and the nine declaration kinds
the `macros/meta/` meta-macros produce. Four of those nine (`Subtype`,
`CounterKind`, `TurnPart`, `Type`) name a position a card also writes; the
first three coincide with a v2 syntax type, so a declaration registered there
is invocable at that position. `Type` does not: v2's syntax type is
`CardType`, so a `Type`-kind declaration is a loader tag only until a ticket
reconciles the two names. Booked below.

**Name mapping.** A Lean constructor becomes a Rust variant of the same word
in `UpperCamelCase`; a Lean field a Rust field of the same word in
`snake_case`. Lean's trailing-underscore keyword escape (`from_`, `while_`,
`end_`) is dropped and Rust's own escape applied instead — a raw identifier
(`r#type`, `r#while`, `r#move`, `r#use`, `r#ref`, `r#as`, `r#from`, `r#if`),
which serde reads and writes under the bare word, so the RON surface keeps the
Lean spelling. `Self` is the one word no raw identifier can escape, so
`Determiner::Self_` carries `#[serde(rename = "Self")]`. The drift test
implements this mapping and pins it.

**Every STOP, with its resolution.**

1. **`macro_ron` cannot register a declaration whose metadata is not `()`.**
   `MacroSet::insert`/`replace` take `&MacroDef<()>`, `MacroDef::body` is
   crate-private, and `()` will not deserialize from the
   `metadata: (spelling: …, grammar: …)` every v2 stub carries — so a consumer
   that reads the shared declaration file under its own (or an ignoring)
   metadata type had no way to register what it read. Verified empirically
   before deciding: the `MacroDef<()>` read of
   `plugins_v2/builtin/macros/stubs/types/Artifact.ron` fails with
   `Expected unit`. Every alternative was worse — registering only the
   metadata-free declarations is a silent capability gap keyed on an unrelated
   fact; stripping `metadata:` from the source text cannot work because the
   field is produced by meta-macro expansion, not written in the file.
   **Resolution:** one additive method, `MacroDef::erase_metadata(&self) ->
   MacroDef`, in `crates/macro_ron/src/set.rs`. No behaviour changes, no
   signature changes, no existing call site touched. ADR §12's "`macro_ron` is
   unchanged" describes its design — a dumb expander with consumer-typed opaque
   metadata — and this addition is what makes that design usable by a second
   consumer. It is the reason the gate closure is workspace-wide; the closure
   is green.

2. **`read_builtin_v2` refused its own tree after the move.** The function
   required the root directory to be named `builtin_v2`; its home is now
   `plugins_v2/builtin`. **Resolution:** it accepts `builtin` (the new name) or
   `builtin_v2` (the symlink still standing at the old path); the error message
   and its doc comment say so, and the existing "wrong root" test still
   refuses `something_else`.

3. **`plugins_v2/builtin`'s declarations name eight param types v2 had not
   registered** (`Cost`, `Amount`, `Ability`, `Quality`, `Subject`,
   `Condition`, `Power`, `Toughness`), so every one of the 1541 declarations
   was unregistrable with `UnknownParamType`. **Resolution:**
   `ron::param_types()` registers them, each validated by reading the argument
   as the v2 type it stands for — `Quality` a `Predicate`, `Subject` a
   `NounPhrase`, `Power`/`Toughness` an `Amount` under the stat name the
   declaration fills. This is the argument vocabulary the declaration files
   already write, not Lean's `MacroParameters` (which §12 calls a proof
   device). `crates/xtask/tests/plugins_v2_declarations.rs` proves all 1541
   now register.

**Deviations and additions** (beyond the ticket's letter, each justified):

- **`syn` as a dev dependency of `deckmaste_semantics_v2`.** The brief limits
  dependencies to `macro_ron`, `serde`, `ron` and std-level crates. The drift
  test must parse this crate's own Rust source; a hand-rolled line scanner over
  Rust would be brittle under `cargo fmt` reflow, which would make the mirror's
  only guard the flakiest thing in the crate. `syn` is dev-only (it does not
  enter the crate's public build graph), is already a workspace dependency, and
  is not deletion-bound. Flagged for review rather than assumed.
- **`Delta` is not a `SupportsMacros` kind.** Lean marks it
  `semantic_expression`, but it is generic (`Delta (α : Type)`) and
  `#[derive(SupportsMacros)]`/`#[derive(Expand)]` reject generics by design. It
  is registered as a hand-built `Kind::new("Delta")` — so a macro of that kind
  is definable and invocable — and carries a hand-written
  `impl<T: Expand> Expand for Delta<T>`. What is lost is the kind's dispatch
  set, which only costs the cycle check and the restricted-read ban their
  precision at `Delta` positions. Named in the code.
- **No `#[macro_ron(literal)]` on `Amount::Lit`.** Lean marks it
  `semantic_literal`, but the marker requires a newtype variant and the mirror
  spells `Lit { value: i32 }` because Lean names that field `value`. Mirroring
  wins; the cost is that a bare numeral does not read at an `Amount` position.
  Reader sugar only — no term is unwritable.
- **`#[serde(default)]` on structure fields with a Lean default, not on
  constructor arguments.** A Lean `structure` field written `:= none` / `:= []`
  means "absent", and `serde(default)` is its encoding. Lean's constructor
  defaults are all the `agent := .you` / `agent := none` slot, and ADR §7 says
  agents are explicit with no default arguments anywhere — so those fields are
  mirrored as ordinary required fields. Called out because it is a real
  divergence from Lean's authoring surface, taken on the ADR's authority.
- **`Copy` on the fieldless enums, and `Box` inside each recursive family.**
  Rust ergonomics and Rust's sizedness, neither visible to the mirror: the
  drift test compares names and order, not representation. Boxing is uniform
  over each reference cycle rather than minimal, so it cannot be wrong.
- **`Nat` → `u32`, `Int` → `i32`.** Lean's numbers are unbounded; a printed
  card's are not.
- **`crates/xtask/src/gate.rs` learns the new path.** `--changed` classified
  `plugins/builtin_v2` paths as declaration data and routed them to every
  reader; it now recognises `plugins_v2/builtin` too, and finds a reader by
  either spelling of the path in its sources. Without this the gate would have
  under-derived its own closure for this landing.
- **`crates/xtask/src/facts/lean.rs` temp fixtures re-pointed.** Eight tests
  built their fixture tree at `<temp>/plugins/builtin_v2` and were red after
  the move. Repaired, not deleted or weakened.
- **`plugins_v2/testing` holds two cards, not one.** "Lightning Bolt" exercises
  nested macro expansion; "Grizzly Bears" exercises the absent-characteristic
  defaults, which nothing else would have covered. Both texts are the corpus's
  (`data/mtgjson/AtomicCards.json`).

**Glossary gaps.** None: every term this landing needed
(`docs/contexts/oracle-english/CONTEXT.md`, `docs/contexts/game-model/`) is
already defined, because every public name is Lean's.

### REPORT

**Assurance counts.** Added 26 tests (12 unit: 5 `reads`, 5 `ron`, plus the
mapping and scan pins in the drift suite; 3 `lean_drift`; 6 `reader`; 2 xtask
`plugins_v2_declarations`; and the fixture-carried cases inside them).
Restored 0. Re-spelled 0. Ignored with blockers 0. **Removed 0** — nothing this
landing touched retired a test's subject; the eight `facts::lean` tests that
went red at the move were repaired in place.

**Provenance.** 204 mirrored declarations / 877 variants / 779 fields against
`lean/Semantics/*.lean`'s six syntax files (1646 lines). 1541 declaration files
moved from `plugins/builtin_v2` to `plugins_v2/builtin`, all 1541 accepted by
both readers. No performance advisory: this landing runs no corpus command, and
the coverage-command wall time is unchanged because nothing it reads changed.

### Ledger — routed at integrate

- **`Type` vs `CardType`.** The `Type` declaration kind does not coincide with
  a v2 syntax position, so builtin type declarations are loader tags rather
  than invocable macros. Reconciling the two names belongs with the
  `semantics-v2-macro-bodies-*` families, which are what give those
  declarations v2 bodies in the first place.
- **The builtin declarations' bodies are still v1 grammar.** They register and
  expand as raw text, but a card invoking one would expand into v1 constructors.
  That is exactly what `semantics-v2-macro-bodies-*` exists to fix; no new
  ticket is owed.
- **`lean-rules-tables`.** `rules.rs`'s three tables have no Lean declaration
  and say so in their own doc comment; when the Lean lands they join the mirror
  and the drift test.
