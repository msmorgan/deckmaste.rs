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
