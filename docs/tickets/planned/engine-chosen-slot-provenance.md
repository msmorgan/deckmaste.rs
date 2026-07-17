---
needs: []
---
**One unkeyed `frame.anaphora.chosen: Option<Vec<ObjectId>>` slot is shared by
three different concepts, disambiguated only by control-flow position.** Whose
picks live in the slot is decided by a presence-flag, not by identity — the tell
of a concept lacking its own channel.

Readers (`crates/deckmaste_engine/src/`):

- chooser binders (`resolve/effect.rs:134-138`)
- `Selection::Random` (`resolve/query.rs:194-198`)
- constrained noted-subset `AmongNoted` (`resolve/query.rs:345-352`)

A chooser surfaces a decision by testing whether the shared slot is populated
(`effect.rs:199` `if frame.anaphora.chosen.is_some() { return None; }`), and
correctness depends on every enclosing construct clearing it before descending
(`effect.rs:629,730,767,811`; write at `decide.rs:1198`). One missed clear in a
future combinator and a chooser silently adopts an ancestor's picks; the
`AmongNoted` read already only clamps foreign picks to live noted members rather
than proving provenance.

## Fix

Key the channel by binder identity, or — better — don't route re-run picks through
the frame at all: the `BindChoice` continuation (`decide.rs:1198`) knows exactly
which node it re-runs, so hand the picks to that node directly. Then no reader
tests a shared presence-flag. See [[atom-independence-anaphora-only]].

Verify: engine tests; a chooser nested under another chooser (or under `Random` /
`AmongNoted`) resolves its own picks; deliberately omit a clear and confirm the new
design does not leak an ancestor's picks.
