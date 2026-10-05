---
needs: []
---
**A Counter's kind is a name; what it does is its Registry Definition, stated
for one counter.** Ruling (2026-10-04): `CounterKind`'s three shapes —
`Boost(power, toughness)`, `Keyword(keyword)`, `Named(label)` — collapse to
one, and every counter is name, holder and Conferrals. Its open questions were
decided with the owner on 2026-10-05 (below). Design-bearing (Lean first);
standard constraints apply. Designations get the same treatment in a separate
landing, `semantics-v2-designation-is-a-name`, which follows this one.

## Why

The shapes let the checker validate a kind without a declaration: any +X/+Y is
well-formed by shape [CR#122.1a], a keyword counter is checked against the
keyword-counter list [CR#122.1b], and only `Named` consults the facts table.
Since `semantics-v2-definition-bodies` every counter declaration carries a
`Counter(kind, holder, confers)` definition, so the shape repeats what
`confers` already says: `p1p1Counter` writes `Boost(Up 1, Up 1)` and then the
power and toughness `Modification`s; `flyingCounter` writes `Keyword("Flying")`
and then the `AbilityGrant`. The `kind` field is doing the work of a name.

## Decisions already made

- A counter declaration's name reads bare at a counter position and denotes
  its kind (`denoted_by` on `CounterKind`, `semantics-v2.md` §12), and
  `CounterKindSource::Printed` is an injection (§11.1). Both rulings
  2026-10-03. Card and macro bodies therefore need not change when the kind's
  representation does.
- The Lean workbench is a work in progress, not a finished spec (2026-10-03):
  `CounterKind` changes in Lean first and the Rust mirror follows.
- The +1/+1 and -1/-1 annihilation rule [CR#704.5q] is not derived from the
  kind's shape and stays with `semantics-v2-counter-annihilation-sba`.

### Decided 2026-10-05 (the former "Open" questions)

- **What the name is: the declaration name, verbatim** (`p1p1Counter`,
  `chargeCounter`). Not the `spelling`, and not the capitalised third form
  `Named(label: "Charge")` writes today.
- **`spelling` stays the compound's STEM** ("+1/+1", "charge"). Every counter
  declaration already yields one compound-noun lexeme "<stem> counter"
  (`macros/meta/CounterKind.ron` `compound_noun`;
  `crates/deckmaste_lexical_source/src/compound.rs`), and
  `crates/deckmaste_construction_core/tests/builtin_v2_counter_kinds.rs`
  forbids a spelling ending in " counter". The owner first proposed
  "+1/+1 counter" as the spelling and withdrew it on this evidence.
- **Self-reference: a definition states what ONE counter confers, and the model
  applies it once per counter held.** Nothing reads its own count and nothing
  refers to itself: `counter(Object, [boost(up(1), up(1))])`. Today
  `p1p1Counter`'s conferral reads its count through `StatOf(axis:
  Counter(kind: Boost(…)), subject: This)`; that goes. Considered and
  rejected: a "this kind of counter" form, and a "this macro" expander hole
  (the owner floated it; it would be a new expander feature,
  `docs/decisions/macros-are-declarative.md`). The name of the helper that
  writes a counter definition is to be **proposed to the owner** before the
  landing; `counter(…)` above is a placeholder.
- **The name's type** (orchestrator's call; the owner did not object): counter
  kinds become a small proper type rather than a `String` alias, so the
  existing `denoted_by` marker (enum-only today) attaches to it. Designations
  get the same answer in `semantics-v2-designation-is-a-name`; the owner wants
  them read bare too ("I'd prefer to avoid stringly typed so yes").

## The change

1. **Lean.** `CounterKind` (`lean/Semantics/Words.lean` ~L523) becomes a name.
   `CounterKind.check` (`Check/PhraseRules.lean` ~L31) becomes one registry
   lookup; today it delegates to `CounterKind.known` (`Check/Keywords.lean`
   ~L84). The holder read takes every holder from `CounterFacts`
   (`Check/Words.lean`, `counterFactsFor`) instead of assuming an Object for
   boost and keyword kinds. A Counter definition's conferrals are per counter,
   and the model multiplies them by the count held. Re-spell the pins in
   `Proofs/Counters.lean` and `Proofs/Rules.lean` (and the other proofs that
   name a kind) against the new shape — same subjects, same outcomes. The
   names `plusOnePlusOne` / `minusOneMinusOne` remain in 15 Lean files;
   re-spell them to the declared names.
2. **Facts.** `cargo xtask facts generate` emits a `CounterFacts` row for every
   counter declaration: `counter_rows` in `crates/xtask/src/facts/lean.rs`
   emits rows for `Named` kinds only today.
3. **Mirror.** `deckmaste_semantics_v2::words::CounterKind` and `reads.rs`
   follow Lean; `lean_drift` holds.
4. **Declarations.** `plugins_v2/builtin/macros/counter_kinds/` has 73 files:
   2 `Boost` (`p1p1Counter`, `m1m1Counter`), 15 `Keyword`, 56 `Named`. Each
   takes its name and states its per-counter conferral. A +X/+Y kind with no
   declaration gets one when a card first needs it; only ±1/±1 are written in
   `plugins_v2/` today. If `semantics-v2-gets-both-deltas` has landed, the
   boost conferral uses its one-modification form.
5. **Bodies.** One file outside the declarations writes `Boost(` (canon
   `Luminarch Aspirant.ron`); re-spell it to `p1p1Counter`. 26 files outside
   write `p1p1Counter`/`m1m1Counter` already and should not change. Then add
   `Printed(kind:` to `no_source_file_writes_out_an_elided_constructor`
   (`crates/deckmaste_semantics_v2/tests/corpus.rs`). (The RON
   `plusOnePlusOne`/`minusOneMinusOne` helpers no longer exist.)

## Proof

- `cargo xtask lean-check` passes with every pin at its old outcome.
- `cargo xtask expansions` before/after: every counter declaration's term
  changes (listed); every card and keyword body is identical except Luminarch
  Aspirant, whose kind now prints as the declared name.
- The English side still reads every counter compound (the stems are
  untouched).

## Out of scope

- Designations (`semantics-v2-designation-is-a-name`).
- Counter annihilation (`semantics-v2-counter-annihilation-sba`).
- Shared-head coordination of counter kinds in English
  (`english-v3-counter-kind-shared-head`).

## Landing record

The series, oldest first: `qrqrtswz` (Lean `CounterKind`, checker, macros,
pins, facts generation, Rust mirror, the 73 declarations' kinds and
conferrals written raw, Luminarch Aspirant and the two testing rules tables),
`pxvxllnp` (the `CounterKind` meta builds the definition; the `boost` and
`grants` conferral helpers; the 73 declarations over them), `mrqlssky` (the
corpus test), `uouzupmw` (ADR, CONTRACTS, glossary), and this record. Each
stage was gated on its own tree. No English grammar, lexicon or corpus input
changed. The cut differs from the ticket's five steps: the mirror cannot read
a `Boost`, `Keyword` or `Named(label)` kind, so the declarations' kinds and
Luminarch Aspirant had to move with the model in the first stage; the second
stage then rewrote the declarations over the meta and helpers without
changing a single expanded term.

**The type.** Lean `inductive CounterKind | named (name : String)`
(`lean/Semantics/Words.lean`), with `CounterKind.name`
(`Check/Words.lean`); mirror `enum CounterKind { Named { name: String } }`
with `CounterKind::name()`, keeping `#[macro_ron(denoted_by(Counter, term =
Definition::counter_term))]`. A one-constructor inductive rather than a
structure because the mirror must be an enum for `denoted_by`, and `lean_drift`
pairs a Lean inductive with a Rust enum. `Definition.counter` keeps its `kind`
field: the `denoted_by` projection sees only the expanded body, not the macro
name, so the body must carry the name.

**Where the name comes from.** The declaration does not write it. The
`CounterKind` meta (`macros/meta/CounterKind.ron`) takes `holder` (default
`Object`) and `confers` (default `[]`) in place of `body`, and builds
`Counter(kind: Named(name: Param(name)), holder: …, confers: …)`, as the
subtype metas build theirs. `deckmaste_construction_core`'s declaration
schema follows (`DiagnosticCounterKind`, a derived body, like
`DiagnosticSubtype`). The owner's sketch `counter(Object, [...])` as a BODY
helper cannot be built: a helper's body cannot see the declaration's name
without an expander feature (the rejected "this macro" hole).

**The per-counter reading** lives in the doc comment of Lean
`Definition.counter` (`lean/Semantics/Rules.lean`), mirrored on Rust
`Definition` (`rules.rs`), in `lean/CONTRACTS.md` (Rules tables), the §7
ruling and the glossary's Registry Definition. Nothing in the checker
depends on it: `Definition.check` checks each conferral once, in the empty
binding context, and computes no multiplicity
(`Proofs/Rules.lean` `p1p1CounterCountMultipliedTwin`: the retired
count-multiplied conferral is refused exactly as the per-counter one, plus the
second stat change's zone check). The reading does NOT fit a shield, stun or
finality counter, whose rules make one or more of them create a single
effect [CR#122.1c,122.1d,122.1h]; all three confer nothing today, which the
doc comment says.

**Checker.** `CounterKind.known` is `knownCounter c.name`;
`CounterKind.scope` reads the holder from the facts row for every kind (the
`.object` fallback stays for an undeclared name, which is refused
`.knownCounter` anyway); `CounterKind.declaresName` is `!c.name.isEmpty`.
Deleted as dead: `counterShift` (`Check/Words.lean`) and `keywordCounterOk`
(`Check/Keywords.lean`). `KeywordFacts.counterEligible` stays: the generator
now reads it off the counter whose conferral grants the keyword
(`Property(AbilityGrant(This, Keyword(K, …)))`), and the Idris `FactsGen`
still consumes it. Nothing derived [CR#704.5q] from `Boost`; no STOP.

**Facts.** `counter_rows` emits one row per counter declaration, labelled
with the kind's name, and refuses a kind other than the declaration's own
name; bodies are read through the builtin plugin's macros so the helpers
expand. Rows: 56 → 73. Added 17: `p1p1Counter`, `m1m1Counter`,
`deathtouchCounter`, `decayedCounter`, `doubleStrikeCounter`,
`exaltedCounter`, `firstStrikeCounter`, `flyingCounter`, `hasteCounter`,
`hexproofCounter`, `indestructibleCounter`, `lifelinkCounter`,
`menaceCounter`, `reachCounter`, `shadowCounter`, `trampleCounter`,
`vigilanceCounter`. Relabelled 56 (`"Charge"` → `"chargeCounter"`,
`"Poison"` → `"poison"`, …). Keyword facts unchanged (15 counter-eligible).

**Pins.** Every pin keeps its asserted value; none went from refused to
accepted or back.
- Re-spelled from a retired kind shape: `Counters.okBoostCounterKind`
  (`.boost (.up 1) (.up 1)` → `p1p1Counter`, `[]`),
  `Counters.badSetBoostCounter` (`.boost (.set 1) (.up 1)` → `.named
  "1/1Counter"`, `[.knownCounter]`), `Counters.okFirstStrikeKeywordCounter`
  (`.keyword "FirstStrike"` → `.named "firstStrikeCounter"`, `[]`),
  `Counters.badCumulativeUpkeepCounter` (`.keyword "CumulativeUpkeep"` →
  `.named "cumulativeUpkeepCounter"`, `[.knownCounter]`),
  `Counters.okCounterMenu`, `Counters.okSameScopeCounterMenu` (menu entry
  `.keyword "FirstStrike"` → `.named "firstStrikeCounter"`, `[]`),
  `Rules.okFlyingCounter` (definition kind `.keyword "Flying"` → `.named
  "flyingCounter"`, `[]`).
- Mechanical: the Lean macros `plusOnePlusOne` / `minusOneMinusOne` are
  `p1p1Counter` / `m1m1Counter` (143 and 9 sites), and `flyingCounter`
  keeps its name; 112 `.named "<Label>"` sites became the declaration name
  (22 Lean files in all); ten bench `.keyword "<K>"` counter kinds became
  `.named "<k>Counter"`.
  Bench cards keep their text otherwise.
- Kept as written, outcome unchanged: `badUnknownCounterLabel` ("Zorp"),
  `badKeywordCounterNamedPlainly` ("Flying" names no counter; doc updated),
  `badDamageResultUndeclaredCounter` ("loyalty"), `badNamelessCounter` ("").
- New (6): `okKeywordCounterByName`, `p1p1CounterDefinitionZone`,
  `m1m1CounterDefinitionZone`, `p1p1CounterCountMultipliedTwin`,
  `grantsIsTheKeywordConferral`, `boostIsOneStatChange`.

**Proof.**
- `lean/scripts/build` on `qrqrtswz`, `pxvxllnp` and the final tree: Build
  completed successfully (82 jobs).
- `cargo xtask lean-check` on `qrqrtswz`: canon 126/126, testing 5/5 (99.5s);
  on `pxvxllnp` and the final tree the same, 126/126 and 5/5 (3.0s and 2.3s:
  the emitted Lean was byte-identical to the stage before).
- `cargo xtask facts check`: up to date on every stage.
- Gate: `cargo xtask gate --changed` derived `cargo test -p
  deckmaste_construction_core -p deckmaste_lexical_source -p
  deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3
  -p xtask` on every stage; run with `--no-fail-fast`: `qrqrtswz` 87
  binaries, 1170 passed, 0 failed, 1 ignored; `pxvxllnp` 1171/0/1;
  `mrqlssky` 1171/0/1 (`lean_drift` 4/4). `uouzupmw` changed docs only.
- `cargo xtask cite check`: 0 stale; `--list-noncompliant`: 0. Blessed:
  [CR#702.9c]. `bless` also pruned three rules nothing cites any more
  (in rules 701, 702 and 717; the lock diff in `qrqrtswz` names them),
  dropped by earlier landings.
- `cargo xtask expansions` before (`snnsrxyr`) and after: declarations 1505
  → 1507 (printed 1457 → 1459), 0 failed. Added `boost`, `grants`. Changed
  120: the 73 counter declarations, and 47 helper and keyword declarations
  whose printed term or sample names a counter kind. Per-card dumps (Rust
  `Debug` and emitted Lean): 26 canon cards and 2 testing cards changed,
  each by a counter kind printing as its name; Luminarch Aspirant also by
  its source. A normaliser mapping each name back to its retired kind term
  (read off the before tree), each per-counter `boost` conferral back to the
  retired count-multiplied pair, and the `Named(name: "Sample")` expansion
  samples back to `label:`, makes every declaration and every card in both
  dumps byte-identical to the before tree: 154 expansion and 82 card nodes
  mapped, 0 unexplained. The second stage changed no expanded term.
- Lexicon: `cargo xtask lexical --card-name "Black Lotus" --export` is
  byte-identical before and after: 35,395 lexemes, 84 with a lemma ending
  " counter" (73 declared + 11 numeric compounds) both times.
- rustfmt clean on the changed Rust; no new clippy finding in changed lines.

**Tests.** Restored 0. Re-spelled: 7 Lean pins (above), and Rust
`the_three_rules_tables_load`, `a_counter_macro_denotes_…_reads_bare_as_printed`,
the restricted-read test of `Named(…)`, the amass fixture (`reader.rs`), two
corpus samples (`corpus.rs`), three expansion samples and a fixture
(`expansions.rs`), `counter_scope_is_read_from_the_declaration`,
`a_positional_body_reads_to_the_same_row` (its counter half is a body no
declaration writes now; re-spelled over `grants`, a positional helper call),
`builtin_v2_counter_kinds_preserve_open_phrases_scopes_and_conferrals` (body
text, and its synthetic `body:` is now `holder:`), and
`a_counter_carrying_a_conferral_payload_declares_no_named_row`, whose subject
(a counter with no row) is retired: re-spelled and renamed
`every_counter_declares_a_row_labelled_with_its_name`. Ignored 0. Added 6
Lean theorems and 2 Rust tests (`a_keyword_counter_is_read_off_its_conferral`,
`a_counter_conferral_helper_writes_what_one_counter_confers`). Removed 0.

**Deviations and additions.**
1. The helper is the meta's `holder`/`confers` parameters plus two conferral
   helpers, not a body helper `counter(…)` (above).
2. New macro kind `Conferral` (hand-built, `ron.rs`) and family
   `macros/conferrals/`, with an `expansions` renderer; Lean macros `boost`
   and `grants` of the same names.
3. `deckmaste_construction_core` gained `DiagnosticCounterKind`; its
   lexical reading is unchanged (export identical).
4. The two testing rules tables wrote `Named("Loyalty")` positionally (the
   ticket's count missed them); they write `loyaltyCounter`.
5. `holder` defaults to `Object`, so the lexical fixtures in
   `construction_core` that declare a counter without a body read unchanged;
   every builtin declaration still writes its holder.

**STOPs.** None. Recorded instead: the Lean checker refuses the +1/+1 and
-1/-1 definitions `[.zoneIs .battlefield]` (a stat change's subject must be a
permanent, where [CR#122.1a] also counts cards in other zones); it refused
the retired spelling too (twice), and no gate runs `Definition.check` over
the declarations. Pinned, not fixed.

**Glossary.** Counter (its kind is its name) and Registry Definition (the
Conferrals are one Counter's, applied per Counter) amended in
`docs/contexts/game-model/CONTEXT.md`.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance advisory:
no English grammar, lexicon or corpus input changed.

**Routed.**
- A +X/+Y kind with no declaration (the lexicon's eleven numeric compounds,
  such as "+1/+0") needs, when a card first writes one, a declaration under
  `counter_kinds/` named for it (say `p1p0Counter`) with `spelling`,
  `compound_stem: Measure`, `compound_onset`, `holder: Object` and `confers:
  [boost(up(1), up(0))]`; nothing else.
- Shield, stun and finality counters need a "one or more of them" reading
  before they confer anything [CR#122.1c,122.1d,122.1h]: for the owner.
- `Definition.check` over the builtin declarations, and the `.zoneIs`
  refusal of the +1/+1 definition: for the owner.
- Helper names `boost` and `grants`, and the meta's `holder`/`confers`: for
  the owner.
