---
needs: [semantics-v2-parity]
---
# Address checker bindings by stable id instead of stack position

**Parked: a hypothesis, promoted only by the spike below.** Standard
constraints apply.

The checker context (`Bindings = List Binding`,
`lean/Semantics/Check/Words.lean`) addresses a binding by its position. A
lexical frame (`Payload.parameterFrame`) stores, per slot, an address "relative
to the tail below the frame", so every push, frame removal and forgetting has
to re-derive those addresses: `shiftAddress`, `operandAddress`,
`closeOperandAddress`, and the two-component case of `bindingAt` and
`setBindingAt`. The hypothesis is that an id fixed when a binding is
introduced, held by frame slots in place of a position, removes that
arithmetic.

## Decisions already made

- **Resolution does not change.** Counted uniqueness over a window stays the
  only anaphora gate (`docs/decisions/workbench-ron-shaped-and-label-rulings.md`,
  rulings 2026-09-04; `docs/decisions/semantics-v2.md` §4; the owner decision
  of 2026-10-05 in `semantics-v2-anaphor-resolution-heuristics`). Every pin
  keeps its outcome.
- **No syntax constructor changes.** `Window`, `Reach`, `NounPhrase.pro`,
  `withBindings` and `inCaller` stay as they are. `Window.parameter (scope
  index)` already names a slot by label, so the positions are private to the
  checker. The Rust syntax enums, the RON registry, the xtask expansions and
  `plugins_v2` therefore do not move. Needing a syntax change is a STOP.
- **Fold-state stays.** A binding keeps `det`, `plur`, and its payload's zone,
  `Stamp`, origin and size (`semantics-v2.md` §3). Carrier-scoping is how
  `cloudshift` checks.
- **The context stays an ordered list threaded as `delta ++ bs`**
  (`docs/decisions/oracle-text-is-forward-anaphoric.md`, clause 2). An id is
  one more datum on a binding, not a replacement for order. A design that
  needs another shape is an amendment to that ADR and a STOP.
- **Nothing `decide` cannot reduce.** `decide` fails on a one-entry
  `Std.HashMap` lookup in this toolchain (tested 2026-10-05).
- **Visibility is not addressing.** `enterCaller` masks macro-local mentions
  while a body is checked against the caller's view, and `leaveCaller` restores
  them with their updates kept. `Payload.hidden` keeps a forgotten mention
  alive for an active capture. Ids do not remove either need. Whether `masked`
  and `hidden` can become structural once slots hold ids is a question for the
  spike, not an acceptance criterion.

## Open question

Where an id comes from. A fresh counter threaded through every `checkIn`
changes every checker signature (424 `Bindings` uses in 26 files, counted
2026-10-05). An id the context can derive for itself may not. The spike picks
one and states its cost.

## Spike: the promotion condition

Prototype on `Check/Words.lean` and the frame paths of `Check/Phrase.lean`
only. Report the helpers deleted and added, the net line count, the number of
signatures changed, whether hand-built `Bindings` literals change spelling
(63 pins build one today, for example in `Proofs/ReferenceScopes.lean`), and
whether `masked` and `hidden` survive. Promote to `planned/` only on a net
deletion with no pin outcome changed. Otherwise record the result here and
leave the ticket parked.

## The work, if promoted

- The `lean/Semantics/Check/` change the spike showed.
- `crates/deckmaste_semantics_v2/src/reads.rs` ports these reads
  (`shift_address`, `operand_address`, `window_addresses`, `enter_caller`,
  `leave_caller`, `restore_masks`) and moves in the same landing
  (`semantics-v2.md` §10). That mirror is still settling until parity, which
  is why this ticket waits on it.
- Pins whose `Bindings` literal changes spelling are re-spelled with the same
  asserted outcome. The pins that expect an `.anaphor` refusal (102 in 22
  files, counted 2026-10-05) keep the same refusal and the same found count.

## Proof

- `shiftAddress`, `closeOperandAddress` and the two-component address case are
  gone from `lean/Semantics/Check/` and from `reads.rs`.
- Every existing pin proves the statement it proves today, by `decide`.
  Re-spelled pins are counted; removed is zero.
- `lean/scripts/build` and `cargo xtask lean-check` pass.

## Not a dependency

`semantics-v2-anaphor-resolution-heuristics` measures the resolution rule,
which this ticket leaves alone. If that work lands a rule that needs clause
structure in the context, re-read this ticket before promoting it.

## Provenance

Began on 2026-10-05 as a proposal to replace the context with an unwindowed
discourse-referent map and to strip zone and provenance from bindings. That
was rejected the same day against the rulings above. Only the addressing
observation survives.
