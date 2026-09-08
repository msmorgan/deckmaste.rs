---
needs: []
---
**The facts generator reads bodies through the dialect.** `cargo xtask facts`
deserialises the keyword-ability, keyword-action, counter-kind and
designation bodies into typed rows through a plainer `MacroSet` than the
semantics_v2 reader, so a body written positionally (`HeldBy(Player)`)
broke `facts check` during `plugins-v2-cosmetic-conversion`; those four
families were excluded from the positional rule and keep their binders
(963 rewrites not made). Point the facts reader at the same `MacroSet`
configuration the v2 reader uses (`denying_unknown_fields`,
`reading_positional_arguments`), then apply the positional rewrite to the
four families with `Facts.lean` and `FactsGen.idr` byte-identical as the
oracle. Standard constraints apply.

## Landing record (2026-09-07)

The facts generator reads declaration bodies through
`deckmaste_semantics_v2::ron::macro_set()` — the v2 reader's own
configuration, not a copy of its flags — and the four families excluded by
`plugins-v2-cosmetic-conversion` now apply one-field constructors
positionally: **823 rewrites across 190 files**, with `lean/Generated`,
`Facts.lean` and `FactsGen.idr` byte-identical through every one.

Measured on `styqykmv` (`corpus: the four stub families apply one-field
constructors positionally`), `nproc` 24, load average 1.8–7.7 across the run.
No coverage lock is in play for this lane.

### PROVE

**No silent loss.** Nothing stopped being covered; coverage rose (below). No
VALUE changed, on four oracles run against the converted tree:

- `cargo xtask lean-check plugins_v2/canon plugins_v2/testing`: 118/118 and
  2/2, and **`lean/Generated/{Canon,Testing}.lean` byte-identical to the
  pre-rewrite baseline** — the same 267,615 B, `Canon.lean`
  `b06c72d68e41b6acb2daf235cec54a1d61192d1765887e7c939293246e4dbe32`,
  `Testing.lean`
  `ac54b387d2bd136584e8620a613ebde60d8999eee22b07f88fa04e9789b87f36`, which are
  the same two hashes `plugins-v2-cosmetic-conversion` recorded. `diff -r`
  clean.
- `cargo xtask facts check`: both generated files up to date, neither
  regenerated — before the rewrite (proving step 1 alone moved no byte) and
  after it.
- `cargo test -p deckmaste_semantics_v2 --test corpus`: green, and its
  nullary-expansion scan now reads 215 declarations where it read 124,
  because the 91 nullary keyword abilities this landing rewrote are newly in
  it.
- The whole `gate --changed` closure green (below), including
  `deckmaste_construction_core`'s builtin reading and `xtask`'s facts
  generation.

**The rewrite is idempotent**: a second run over the converted tree makes 0
rewrites in 0 files.

**Every comment and every layout choice survives.** The script deletes only
the `binder:` text inside an application's parentheses, so the diff is
line-for-line: 190 files, 699 insertions, 699 deletions, and **0 changed lines
carry a comment**. Sites inside a comment or a string literal are masked out
before any match is considered, and an argument list holding a comment is
skipped (0 such sites arose).

**Structural laws.** `cargo test -p deckmaste_semantics_v2 --test lean_drift`
green — the landing edits no Rust mirror declaration. Every emitted card still
proves. Reading a body positionally is the same row as reading it by binder,
pinned by
`xtask::facts::lean::tests::a_positional_body_reads_to_the_same_row`, which
renders the whole registry from two fixture trees — one with
`scope: HeldBy(holder: Player)` and `CounterFacts(label: "Shield", holder:
Object)`, one with `HeldBy(Player)` and `CounterFacts("Shield", Object)` — and
asserts the two renders are equal. Its negative control: with the previous
reader (`raw_options().from_str`) it fails at `1:14: Expected identifier`.

**No word-naming.** No guard added here names a lexeme, construction, verb,
noun, preposition or card identity. The rewrite resolves every head against
the mirror's own declared variants (`cargo xtask map enums
crates/deckmaste_semantics_v2/src`); the pin's two strings are a `(constructor,
binder)` spelling of a TYPE's variant, not a word. No forbidden licensing
checker is involved; the lane has none.

### DISCLOSE

**Step 1 — the reader.** `crates/xtask/src/facts/lean.rs` read a counter and a
designation body with `deckmaste_semantics_v2::ron::raw_options().from_str`,
which is the RON dialect without either reader switch. Both sites now read
through `deckmaste_semantics_v2::ron::macro_set()` — the constructor the v2
reader itself calls (`reader::Plugin::load` → `ron::macro_set()`), so
`denying_unknown_fields` and `reading_positional_arguments` are inherited
rather than restated. `facts check` was byte-identical immediately, before any
corpus change: the switches are permissive, so every body that read before
still reads.

Only those two sites read a body at all. The keyword-ability and
keyword-action rows come from the checker-column overlays and the declared
`params:` signature, never from the body — which is why the cosmetic landing's
exclusion of all four families was broader than the failure that prompted it.

**Step 2 — the rewrite.** A scratchpad Python script over the `body:` value
span of each `.ron` in the four family directories; nothing of it entered
version control and it is deleted. It is NOT the type-directed schema walk the
cosmetic converter was, because the positional rule does not need one: the
constructor stays and only its binder goes, so the question is not "which type
declares this head" but "does every reading of this written form take the
argument to the same field". The script answers that from three facts:

1. **The case rule** (`plugins-v2-dialect`, fourth landing): macros are
   camelCase, constructors PascalCase. A lowercase head is skipped — a macro
   with a `Params::Named` signature cannot be applied positionally at all
   (that is the cosmetic landing's STOP-note 1, still open and still out of
   scope).
2. **The written form**: exactly one argument, opening with `binder:`, with no
   comment inside the parentheses.
3. **The mirror's inventory**: of every variant named `Head`, EVERY one that
   declares a field named `binder` declares it alone. Those are exactly the
   readings the written form admits — a variant of `Head` without that field
   would not read today — so whichever type the position expects, the argument
   lands in the same field.

Fact 3 is what makes an ambiguous head safe without type direction, and it
matters: 375 of the 823 sites are at a head several types declare. `Zones` is
the clearest — `EventSource::Zones{zones}` and `Exchanged::Zones{left, right}`
— and only the former carries `zones`, so `Zones(zones: [...])` →
`Zones([...])` is unambiguous while a name-keyed rule could not tell. A
head-and-binder pair that failed any of the three tests was reported, not
guessed; after fact 3 was applied the skip list was **empty**.

**Rewrites by family**: `keyword_abilities` 593, `keyword_actions` 211,
`designations` 19, `counter_kinds` 0 — a counter body is
`CounterFacts(label:, holder:)` or a `Counter(...)` conferral payload, both
multi-field, so the family the cosmetic landing's failure actually came from
has nothing to convert.

**Rewrites by constructor**, 73 distinct heads:

- 104 `Action` · 66 `Up` · 58 `Cost` · 54 `HasType` · 39 `A` · 36 `Printed` ·
  34 `Number` · 34 `Word` · 24 `InZone` · 24 `Core` · 20 `Named` · 20 `Stat` ·
  19 `Optional` · 19 `HeldBy` · 18 `And` · 16 `Not` · 15 `Type` · 14 `Zones` ·
  14 `PossessedBy` · 12 `Sequentially` · 11 `Counterpart` · 11 `Until` ·
  10 `Down` · 10 `Target` · 9 `PayingInstead` · 8 `Gap` · 7 `AbilityHead` ·
  7 `The` · 7 `HasCounters` · 6 `ByKeyword` · 6 `HasKeyword` · 5 `Quality` ·
  5 `CombatPlayer` · 5 `Cards` · 5 `HasSubtype`
- 4 each: `Some`, `ByPlayer`, `Top`, `EntersAs`, `Written`
- 3 each: `Mana`, `PlayerGroup`, `Shuffle`, `Stamped`
- 2 each: `Static`, `Counter`, `ManaCost`, `OtherThan`, `EntersAttacking`,
  `Compound`, `Letter`, `HasStatus`, `TurnOver`, `Or`
- 1 each: `CastFrom`, `ExactlyOf`, `PlayerStat`, `Draws`, `ColorIs`, `Runs`,
  `Subject`, `MoreThan`, `ForAsLongAs`, `Exists`, `OneEnd`, `AbilityOf`,
  `AddedAbilities`, `Under`, `ClearDamage`, `Perform`, `EntersMelded`,
  `AnyCounter`, `EachOf`

**Newly covered, with its oracle named.** `every_nullary_helper_expands` now
reads the `Ability` kind, which is where a keyword ability's body sits: 215
nullary declarations expand where 124 did, +91 — the nullary keyword
abilities, 42 of which are files this landing rewrote. Its negative control: `InCombat(relation: AttackerOf,
counterpart: None)` broken to `InCombat(AttackerOf)` in `battleCry.ron` fails
the test. The scan's floor moved 80 → 200 to match, so losing the new kind is
a failure rather than a quiet drop.

**Where the rewrite is proved, site by site**, since not every declaration has
a reader that reaches it:

| files | how the rewrite is read back |
| --- | --- |
| 17 designations | `facts check` renders every row, byte-identical |
| 45 nullary keyword abilities/actions | the corpus scan expands each body at its kind |
| 68 parameterized ones a canon or testing card invokes | `lean-check`, `lean/Generated` byte-identical |
| 60 parameterized ones no card invokes | inventory argument only — 132 rewrites |

For the last row the residual risk was measured rather than asserted: of the
75 distinct `(constructor, binder)` pairs rewritten anywhere, **only 8 occur
exclusively in a file no oracle reads** — 9 of the 823 sites. Each was read
off the mirror by hand: `KeywordParam::Subject{predicate}`,
`Duration::ForAsLongAs{condition}`, `Reach::Stamped{verb}`,
`LibraryPlace::OneEnd{end}`, `Instruction::TurnOver{subject}`,
`CharacteristicEdit::AddedAbilities{abilities}`, `TokenRider::Under{controller}`,
`Instruction::ClearDamage{subject}` — every one a lone field, and every other
declaration of those heads (`KeywordParamShape::Subject{}`,
`CaptureInput::Subject{expression}`) carries a different field name entirely.

**Deviations and additions.**

- The nullary scan gained the `Ability` kind and a raised floor (above). It is
  beyond the ticket's letter and it is why the keyword-ability half of the
  rewrite has a direct oracle at all.
- The rewrite count is **823, not the 963 the cosmetic landing forecast**. The
  gap is not sites refused: the skip list is empty. Two candidate explanations
  were not run down, because doing so means rebuilding the deleted converter:
  its count may have included the `metadata` half (`grammar: FixedTerm(surface:
  …)` and its 739 siblings, which stay named in every family and which this
  landing deliberately leaves alone), or it may have counted a form its
  type-directed walk classified differently. Recorded as a discrepancy against
  a number in a landed record, not resolved.
- `cargo xtask cite bless` was not run: the audit selects 0 citation sites.
- Two pre-existing clippy warnings in
  `crates/deckmaste_construction_core/tests/builtin_v2_keyword_abilities.rs`
  (`redundant closure`, `map_or` simplification) — that file is untouched by
  this landing.

**STOPs.** None.

**Glossary.** No term this landing needed is missing from
`docs/contexts/game-model/CONTEXT.md` or
`docs/contexts/oracle-english/CONTEXT.md`.

**Assurance counts.** Restored 0, **re-spelled 1**
(`builtin_v2_designations_preserve_identity_surfaces_and_definitions`: three
raw-body assertions, `scope: HeldBy(holder: Player)` ×2 and `scope:
HeldBy(holder: Object)` ×1, re-spelled to the positional form — same
designations, same asserted scopes), retired 0, ignored-with-blocker 0,
**added 1** (`a_positional_body_reads_to_the_same_row`) plus one existing test
extended, removed 0.

### REPORT

- `cargo xtask lean-check plugins_v2/canon plugins_v2/testing`: 118/118 and
  2/2. 2 m 15 s cold for the baseline, 0.4–0.6 s warm after. `lean/Generated`
  byte-identical, 267,615 B.
- `cargo xtask facts check`: both files up to date, neither regenerated.
- `cargo test -p deckmaste_semantics_v2 --test corpus`: `plugins_v2/builtin`
  2,414 declarations across 31 kinds; **215 nullary declarations expand, 206 at
  untested kinds** (was 124 / 297); `plugins_v2/canon` 118 cards;
  `plugins_v2/testing` 2 cards, 1 token, 1 sba / 1 conferral / 1 damage row;
  120 cards read, wrote and read back to the same value; 2,215 source files
  hold no elided constructor.
- `cargo xtask gate --changed` closure, run green in **2 m 42 s**, 51
  `test result: ok` lines, **0 failed**, 1 pre-existing ignored:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p
  deckmaste_english_v2 -p deckmaste_semantics_v2 -p xtask`.
- `cargo xtask cite check --list-noncompliant`: 0 non-compliant.
  `cargo xtask cite check`: 15,741 citations, 0 stale.
  `jj diff --git --from <claim> --to @ | cargo xtask cite audit --diff`: 0
  sites, so `cite bless` was not run.
- `cargo fmt --all --check` clean; clippy clean on `xtask` and
  `deckmaste_semantics_v2` (`--all-targets`), two pre-existing warnings in
  `deckmaste_construction_core`'s untouched keyword-abilities test.
- Diff: 190 corpus files, 699 insertions, 699 deletions; 2 test files.
- Performance advisory: no coverage command runs in this lane, so the 16.26 s
  quiet-host ceiling and the per-byte telemetry line do not apply. The gate
  closure is the lane's cost: 2 m 42 s wall on 24 cores at load average 7.7,
  23 m 12 s user.
