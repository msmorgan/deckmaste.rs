---
needs: []
---
**A `plugins_v2` declaration must be invocable at the semantic position its
body occupies.** Two family landings (2026-09-06) found it is not:

- `Adamant(<ability>)` at an `Ability` slot does not resolve. A declaration
  carries exactly one kind, its family (`AbilityWord`, `KeywordAction`,
  `KeywordAbility`), `deckmaste_construction_core::macro_def::normalized_kind`
  refuses a second, and `deckmaste_semantics_v2`'s reader registers the macro
  under that family kind. Only families whose name coincides with a syntax
  type (`TurnPart`, `CounterKind`, `Subtype`) are invocable today. The
  ability-word canon cards spell `ItalicHead(...)` directly as a result.
- A `TurnPart` declaration whose name equals its body's constructor
  (`Upkeep` → `Upkeep`) is refused at register time as a self-invocation:
  `macro_ron::set::check_cycles` exempts an identity macro only when the
  kind's variant list contains the name, and `semantics_v2::ron::kinds()`
  never registers `TurnPart::kind()`. Eight turn-part declarations are
  blocked on it (their intended bodies sit in each file's STOP comment).

Fix, keeping the file as the shared contract (`semantics-v2.md` §11) and
kinds as one-per-`SupportsMacros`-enum (§12):

- Each meta-declaration emits `kinds: [<Family>, <SemanticKind>]`
  (`KeywordAction` → `Instruction`, `KeywordAbility` → `Ability`,
  `AbilityWord` → `Ability`; the coinciding families stay single). Update
  `plugins_v2/builtin/macros/meta/*.ron`; the stub files need no change.
- `construction_core` reads the family kind it knows and tolerates further
  kinds; `semantics_v2`'s reader registers each declaration under every
  kind that names a `SupportsMacros` enum and ignores family-only kinds.
- `semantics_v2::ron::kinds()` registers every `SupportsMacros` enum's kind,
  `TurnPart` included, so the identity-macro exemption sees its variants.
- Re-point the ability-word canon cards to invoke their macros
  (`Landfall(...)`, `Raid(...)`, `Threshold(...)`), and add a
  `plugins_v2_declarations` test that invokes one declaration of each
  family at its semantic position and expands it. `lean-check
  plugins_v2/canon` proves every card.

Standard constraints apply. Gate closure: construction_core, english_v2,
semantics_v2, xtask.

## Landing record

### PROVE

**No silent loss.** Nothing stopped being covered. No declaration, card, or
test lost a subject: `both_readers_accept_every_builtin_declaration` still
names the same declaration set on both sides, `lean-check plugins_v2/canon`
still proves 3/3 and `plugins_v2/testing` 2/2, and the two tests whose
subject changed shape were re-spelled, not deleted (below). This ticket
touches no `english_v2` corpus, coverage lock, or licensing checker, so the
add-only census, the lock `covered` count, and the homograph/overlap
inventories are not in scope and are unchanged.

**Structural laws.** The three re-pointed canon cards prove `Card.check = []`
through the Lean kernel after expansion, which is the byte-for-byte identity
between the invocation and the `ItalicHead` shape they used to spell by hand:
the emitted term is post-expansion, so a difference would be a proof failure,
not a silent pass. `an_ability_word_invocation_expands_at_an_ability_position`
asserts that identity directly (invocation `==` hand-written expansion).

**No word-naming.** No guard added anywhere names a lexeme, construction,
verb, noun, preposition, or card identity. The one new name-shaped constant is
`TurnPart`'s own type name in `kinds()`, which is a Rust type, not a lexeme.
`cargo xtask cite check --list-noncompliant` reports 0; `cite check` reports
15091 citations, 0 stale; `cite audit --diff` selected 0 citation sites (no
citation line changed).

### DISCLOSE

**What the fix is.** Each of the three non-coinciding declaration metas now
emits `kinds: [<Family>, <SemanticKind>]` — `AbilityWord → Ability`,
`KeywordAbility → Ability`, `KeywordAction → Instruction`. The family kind
leads and stays the declaration's identity;
`deckmaste_construction_core::macro_def::normalized_kind` reads the first kind
and ignores the tail. `deckmaste_semantics_v2` registers the declaration under
every kind it names, which is what makes `Threshold(<ability>)` resolve where a
card writes an ability.

**No `macro_ron` change was needed**, contrary to the possibility the brief
left open. Neither reader had to learn to ignore an unregistered kind:

- `deckmaste_construction_core` never *registers* the declarations it reads.
  `declaration_reader()` builds a `KindSet` holding only `Macro` and inserts
  only the nine meta-macros; a produced declaration is read as data
  (`MacroDef<Metadata>`) and never passed to `insert`/`replace`, so
  `MacroSet::check_kinds` never sees `Instruction` or `Ability`.
- `deckmaste_semantics_v2` already registers both new semantic kinds
  (`Ability`, `Instruction` are `SupportsMacros` types), so `check_kinds`
  passes on the widened list as written.

**The turn-part blocker.** `macro_ron::set::check_cycles` exempts an identity
macro whose body names a native variant of one of the macro's own kinds, and
it reads that variant list off the registered `Kind`. `TurnPart` was
registered by `DECLARATION_KINDS` as a hand-built `Kind::new("TurnPart")`,
whose dispatch set is empty — so `Upkeep` → `Upkeep` resolved to the macro
itself and was rejected as a self-cycle at register time. Fixed by making
`words::TurnPart` derive `SupportsMacros` and registering `TurnPart::kind()`,
which carries the real variant list; the `DECLARATION_KINDS` loop now skips a
family name already registered from a type's own derive, so ordering cannot
silently replace a derived kind with a variant-less one.

**Mechanism that keeps the kind mapping total.**
`deckmaste_semantics_v2::ron::tests::every_supports_macros_type_is_a_kind`
reads the crate's own `src/*.rs`, collects every type under a
`#[derive(… SupportsMacros …)]`, and asserts each is a registered kind. The
derive attribute is the fact, so the test reads the fact rather than a
hand-maintained list that would only restate it; a floor of 17 derives guards
against the scan going vacuous if the sources move.

**Canon cards re-pointed.** `Eumidian Terrabotanist` (`Landfall`),
`Storm Fleet Spy` (`Raid`), and `Mystic Visionary` (`Threshold`) now invoke
their declarations with the same inner ability each previously wrapped by hand;
each card's header comment, which asserted the invocation was unreachable, was
rewritten. `cargo xtask lean-check plugins_v2/canon` proves 3/3.

**Deviations and additions.**

1. `words::TurnPart` swapped from `Deserialize, Expand, Serialize` to
   `SupportsMacros` (which generates all three plus the trait). The ticket
   pins "`kinds()` registers every `SupportsMacros` enum's kind, `TurnPart`
   included", and `TurnPart` was not one; a hand-built kind cannot carry the
   dispatch set the exemption reads, and a hand-copied variant list beside it
   would drift. `TurnPart` is all unit variants, so the generated `Serialize`
   is identical to serde's (the `lean_emit` helper-struct wrinkle applies only
   to struct variants) — `lean_drift`'s four tests stay green. `Subtype` and
   `CounterKind`, the other two coinciding families, were deliberately NOT
   converted: they carry struct variants, their declarations have no identity
   bodies, and nothing in this ticket needs their dispatch sets.
2. The module doc of `ron.rs` said kinds are exactly the types Lean's
   `attribute [semantic_expression]` lines name. That was already only
   approximately true (`DECLARATION_KINDS` registered nine more), and
   `TurnPart` is now a macroable kind outside that list; the doc and
   `DECLARATION_KINDS`' own doc were corrected rather than left to mislead.
3. `KeywordAbility`'s meta was widened to `[KeywordAbility, Ability]` even
   though no builtin keyword ability has a body to expand yet — the ticket
   names all three families, and the kind is what a body will need when that
   family's ticket fills them in.
4. `derived_kinds_carry_their_dispatch_set` gained `TurnPart`: it is the exact
   property whose absence was the blocker.
5. The three new `xtask` tests were run against a probe tree with the three
   enabling edits reverted (the `TurnPart::kind()` registration, and the second
   kind on the `AbilityWord` and `KeywordAction` metas). All three failed with
   the pre-fix errors (`` `Threshold` is neither a variant of `Ability` nor a
   known `Ability` macro``, the same for `DrawFor` at `Instruction`, and the
   turn-part cycle), so none is vacuous. Probe reverted.
6. The turn-part and keyword-action builtin stubs were NOT edited (their bodies
   belong to tickets claimed elsewhere), so the two tests that need a body use
   tempdir declarations built from the same shared metas.

**STOPs.** None. The one judgement call — converting `TurnPart` to a
`SupportsMacros` type, which is a kind outside Lean's `semantic_expression`
list — was checked against the recorded rulings before proceeding:
`docs/decisions/semantics-v2.md` §12 says kinds are one per `SupportsMacros`
enum, which the change satisfies rather than contradicts, and the
`semantic_expression` claim lived in a code doc comment (already inaccurate),
not in a ruling. Disclosed as deviation 1 and the comment corrected.

**Glossary gap.** None: "kind", "declaration", "family", and "macro" are all
used here as `docs/decisions/semantics-v2.md` §§11–12 define them.

### REPORT

**Assurance counts.** restored 0 · re-spelled 2 · ignored-with-blockers 0 ·
added 4 · removed 0.

- Re-spelled (same subject, same asserted outcome, new spelling — the
  meta-macros' kind list grew a second entry):
  `deckmaste_construction_core::macro_def::tests::graduated_declaration_expands_and_normalizes`
  (was `kinds == [KeywordAction]`, now the exact two-element list with the
  family leading) and
  `…::nursery_and_graduated_sources_use_the_ordinary_macro_reader`
  (was `kinds.len() == 1`, now the leading kind equals the meta the source
  invokes — a stronger check than the count it replaces).
- Added: `deckmaste_semantics_v2::ron::tests::every_supports_macros_type_is_a_kind`;
  `xtask::plugins_v2_declarations::{an_ability_word_invocation_expands_at_an_ability_position,
  a_keyword_action_invocation_expands_at_an_instruction_position,
  a_turn_part_declaration_may_name_its_own_constructor}`.

**Gate.** `cargo xtask gate --changed` derived and ran

```
cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask
```

45 suites, 1517 passed, 0 failed, 1 ignored (pre-existing, in `english_v2`).
Also green: `cargo test -p xtask --test plugins_v2_declarations` (4 passed),
`cargo xtask lean-check plugins_v2/canon` (3/3) and `plugins_v2/testing`
(2/2), `cargo xtask facts check` (both generated files up to date),
`cargo fmt --all`, and
`cargo clippy -p deckmaste_semantics_v2 -p deckmaste_construction_core -p xtask --all-targets -- -D warnings`.

**Performance advisory.** The `english_v2` coverage command and its per-byte
telemetry are not in scope: no `english_v2` code, corpus, or declaration data
this ticket touched feeds it, and the coverage lock is unchanged — so there is
no `covered` count and no ns/B line to report. The gate's own wall time was
73.4 s real / 7 m 42 s user (24 cores, load average 4.25 at start, cargo's
default worker count, warm `target/`); `lean-check plugins_v2/canon` took 0.7 s
warm (74.9 s cold, dominated by `lake` building the generated library) and
`plugins_v2/testing` 1.3 s.

**Stamp.** Every number above was measured on change `wvkvurtz` (this commit),
the parked tip of `plugins-v2-invocation-kinds`, whose ancestors are
`unvsyoxq` (the fix) and `uuxszxpx` (the tests). No coverage lock is involved,
so there is no lock `covered` count to stamp alongside it.
