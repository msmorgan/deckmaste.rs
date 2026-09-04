---
needs: []
---
**Engine: two side environments outlived the activation record.** Law 11 of
[Core is explicit regions](../../decisions/core-explicit-regions.md) makes the
activation record the environment, and the ADR's Rejected list refuses both a
second environment with its own read rule ("a frame-local tier") and a register
file inside `Frame`. Two survive anyway.

**1. Scopes with no region.** `ActivationId::Bare { source, controller }`
(`crates/deckmaste_engine/src/activation.rs:22-25`, constructed at `:35`)
synthesizes a context on demand (`activation.rs:520-534`) for every rule and
cost position that was never given a region, and `Frame::bare` reaches it from
call sites across the engine. Its read path needs a matching escape:
`crates/deckmaste_engine/src/resolve/query.rs:379-391` falls back from the
indexed register read to a provenance lookup when the register is the
`Reg(0)`/`Reg(1)` source/controller parameter, documented there as holding
"until those scopes gain regions of their own". That is the ticket: give them
regions.

**2. The string-keyed note map.** `GameState.resolution_notes`
(`crates/deckmaste_engine/src/state.rs:520-521`) is an `Ident`-keyed map that
`ChooseNoteNumber`/`ChooseNoteCardName` write ONLY as a fallback — both already
write the activation register when a `ChoiceContinuation::BindSymbol { dest }`
is armed, and fall into the map when one is not
(`crates/deckmaste_engine/src/decide/pending/choice.rs:275-281` and
`:305-313`). Two readers keep the fallback alive:

- `Count::Noted(Ident)` (`crates/deckmaste_core/src/count.rs:248`), read at
  `crates/deckmaste_engine/src/resolve/count.rs:462`, which degrades a missing
  note to 0 behind an `eprintln!`. Lowering emits it at
  `crates/deckmaste_lowering/src/count.rs:774-783` only when neither a
  region-local binding nor a cell read matches the name.
- The chosen-name override inside the printed-name filter,
  `crates/deckmaste_engine/src/target.rs:314-318`: a
  `CharacteristicPredicate::Named(name)` substitutes
  `resolution_notes[name]` for the literal when a `CardName` note is present.

**`CharacteristicPredicate::Named` itself stays.** It is the printed-name
filter `named ~` ([CR#201]) at `crates/deckmaste_core/src/filter.rs:63`, and
deleting it would delete name-matching. What retires is the note override at
`target.rs:315-317`: a chosen card name becomes a symbol register the filter
reads by index.

**3. `Count::X` without a declared slot.** `Count::X`
(`crates/deckmaste_core/src/count.rs:162`) reads `activation_x`
(`crates/deckmaste_engine/src/resolve/count.rs:397-399`, an `expect`), and
lowering emits it at `crates/deckmaste_lowering/src/count.rs:740` only when the
region declared no X. `Provenance::AnnouncedX` already exists
(`crates/deckmaste_core/src/region.rs:65`), so this is a declaration gap, not a
missing mechanism.

Scope: rule scopes and cost scopes get regions of their own, so `Bare`, the
`bare` context synthesis, `Frame::bare`, and the `Reg(0)`/`Reg(1)` provenance
fallback all delete; every `Noted`/`Named`-note read becomes a `Let`/`Remember`
def or a declared param and `resolution_notes` deletes with its two readers;
`Count::X` always reads a declared `AnnouncedX` param.

**Not superseded by, and not blocked on,
`docs/tickets/maybe/engine-crossed-channel-scope.md`.** That ticket's stated
mechanism is stale — `Condition::Crossed` no longer reads `frame.anaphora`; it
reads `activation_crossed` (`crates/deckmaste_engine/src/condition.rs:172`),
and a bare context supplies `crossed: None`, so the reachable failure is the
`todo!` at `condition.rs:173-177` that names it as owner. Giving rule and cost
scopes real regions removes the *bare-frame* positions but not the seam: a real
region entered from an activated ability's "Activate only if" gate still
carries no counter-placement fact. So leave that `todo!` standing and do not
repoint it; the exclusivity decision it asks for is still open.

Out of scope: the `Count::Noted` zero-degradation behavior as such — the
fizzling read is [[engine-stat-none-fizzle]]; this ticket removes the channel
that needs it.

Acceptance:

- `ActivationId::Bare`, `Frame::bare`, `resolution_notes`, `Count::Noted`, and
  `Count::X` are absent from the workspace, as is the `Reg(0)`/`Reg(1)`
  fallback arm in `resolve/query.rs`.
- `CharacteristicPredicate::Named` still matches a printed name, with a test.
- `cargo test --workspace` green (engine change, so the workspace gate, not an
  enumerated `-p` list).
- The witness fixtures that exercise noted numbers, chosen card names, and X
  produce the same outcomes as before — re-spelled against the register shape,
  not deleted.

Standard constraints apply. Effort: **L**.

## Landing record

Implemented by a Codex delegate; diff reviewed and every gate re-run by the
orchestrator before commit.

**Shape.** Rule frames now create stored activation records with explicit source
and controller parameters. Spell and activated regions declare source,
controller, announced targets and announced X; triggered regions declare their
event roles, targets and X. Directly constructed legacy abilities with empty
regions normalize to the same explicit shape at the execution boundary. A noted
number lowers to a region definition or a declared linked-cell parameter, and X
lowers to the region's declared announced-X register.

**The printed-name filter.** The literal `Named` match survives untouched, with
`named_matches_card_name` discriminating a hit, a miss, and a non-card. A chosen
card name became a SEPARATE variant beside it, `NamedReg(RefId)`: the value
instruction stores the symbol in the activation register and the filter reads it
by index. The note override inside the literal variant is gone, which is exactly
the split the ticket asked for.

**Absent from the workspace**, by grep: `ActivationId::Bare`, `Frame::bare`,
`resolution_notes`, `Count::Noted`, `Count::X`, and the source/controller
provenance fallback in the indexed read.

**The protected seam is intact.** The unimplemented marker in the condition
module still stands. The only change in that file is inside its own test module,
swapping the bare-frame constructor this task deletes for the ordinary one.

**Tests.** added 0 · re-spelled 8 · removed 0 · weakened 0 · ignored 0 added.
This is a removal task whose acceptance is that outcomes are preserved, so the
eight re-spellings are the substance: the noted-number, chosen-card-name and X
fixtures all assert what they did before against the register shape.

**Gates**, re-run by the orchestrator on the final tree:

- `cargo test --workspace` — 127 result lines, no failures. The
  `deckmaste_construction` compile-fail fixture that was failing earlier in this
  queue has since been fixed by another session, so the workspace gate this
  ticket's acceptance names is genuinely green rather than green-except-one.
- `cargo xtask idris-check plugins/canon --differential` — `differential OK: 0
  disagreements`.
- `cargo xtask cite check` — 0 stale; `--list-noncompliant` — empty.
- `cargo clippy --workspace --all-targets` — 11 warnings in
  `deckmaste_lowering`, all PRE-EXISTING and none introduced here: 10
  redundant-pattern lints in `card.rs:166-184` and one items-after-test-module in
  `ability.rs`. Verified by reproducing the identical breakdown on the default
  line. They want their own sweep and are not this ticket's business.

**Deviations and additions.** None. Scope held to the three parts the ticket
names; 65 files touched, all mechanical consequences of deleting the five
constructs.

